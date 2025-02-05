#include "table.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "value.h"

#define TABLE_MAX_LOAD 0.75  // the table should grow when capacity reaches 75%

void initTable(Table* table) {
  table->count    = 0;
  table->capacity = 0;
  table->entries  = NULL;
}

void freeTable(Table* table) {
  FREE_ARRAY(Entry, table->entries, table->capacity);
  initTable(table);
}

uint32_t getHashValue(Value value) {
  switch (value.type) {
    case VAL_NIL:
      return 0;
    case VAL_BOOL:
      return AS_BOOL(value) ? 1 : 2;
    case VAL_NUMBER: {
      double number = AS_NUMBER(value);

      uint32_t hash  = 0;
      uint8_t* bytes = (uint8_t*)&number;
      for (int i = 0; i < sizeof(double); i++) {
        hash = 31 * hash + bytes[i];
      }
      return hash;
    }
    case VAL_OBJ:
      if (IS_STRING(value)) {
        return AS_STRING(value)->hash;
      }
      // return AS_OBJ(value);
  }
  return 0;
}

bool isTombstone(Entry* entry) {
  return IS_NIL(entry->key) && AS_BOOL(entry->value) == true;
}

int getEntryIndex(Table* table, Value* key) {
  return getHashValue(*key) % table->capacity;
}

/**
 * ## findEntry
 *
 * @brief find an entry in the table
 *
 * @param entries
 * @param capacity
 * @param key
 * @return
 */
STATIC Entry* findEntry(Entry* entries, int capacity, Value* key) {
  // map key's hash code to an index within the array bounds
  // uint32_t index     = key->hash % capacity;
  uint32_t index     = getHashValue(*key) % capacity;
  Entry*   tombstone = NULL;

  for (;;) {
    // get the entry by index
    Entry* entry = &entries[index];

    if (IS_NIL(entry->key)) {
      if (IS_NIL(entry->value)) {
        // empty entry found
        return tombstone != NULL ? tombstone : entry;
      } else {
        if (tombstone == NULL) tombstone = entry;
      }
    } else if (valuesEqual(entry->key, *key)) {
      // key found
      return entry;
    }

    // if the previous conditions don't pass then there is a collision in the table
    // index to the next entry
    index = (index + 1) % capacity;
  }
}

/**
 * ## adjustCapacity
 */
static void adjustCapacity(Table* table, int capacity) {
  Entry* entries = ALLOCATE(Entry, capacity);
  table->count   = 0;
  for (int i = 0; i < capacity; i++) {
    entries[i].key   = NIL_VAL;  // previously NULL
    entries[i].value = NIL_VAL;
  }

  // take the old table entries and copy them into the newly sized entries
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (IS_NIL(entry->key)) continue;

    Entry* dest = findEntry(entries, capacity, &entry->key);
    dest->key   = entry->key;
    dest->value = entry->value;
    table->count++;
  }

  FREE_ARRAY(Entry, table->entries, table->capacity);
  table->entries  = entries;
  table->capacity = capacity;
}

bool tableSet(Table* table, Value* key, Value* value) {
  // check to make sure the entry can fit
  if (table->count + 1 > table->capacity * TABLE_MAX_LOAD) {
    int capacity = GROW_CAPACITY(table->capacity);
    adjustCapacity(table, capacity);
  }

  // find a bucket for the entry
  Entry* entry = findEntry(table->entries, table->capacity, key);

  // update the size of the table
  bool isNewKey = IS_NIL(entry->key);
  // increment the count only if the new entry is not a tombstone (key is null and value is Nil)
  if (isNewKey && IS_NIL(entry->value)) table->count++;

  // copy the entry into the table
  entry->key   = *key;
  entry->value = *value;

  return isNewKey;
}

bool tableDelete(Table* table, Value* key) {
  if (table->count == 0) return false;

  // find the entry
  Entry* entry = findEntry(table->entries, table->capacity, key);

  // nothing to delete if not found
  if (IS_NIL(entry->key)) return false;

  // place a tombstone in the entry
  // a tombstone is an entry with no key and a value of true
  entry->key   = NIL_VAL;  // previously NULL
  entry->value = BOOL_VAL(true);
  return true;
}

bool tableGet(Table* table, Value* key, Value* value) {
  if (table->count == 0) return false;

  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (IS_NIL(entry->key)) return false;

  *value = entry->value;
  return true;
}

void tableAddAll(Table* from, Table* to) {
  for (int i = 0; i < from->capacity; i++) {
    Entry* entry = &from->entries[i];
    if (IS_NIL(entry->key)) {
      tableSet(to, &entry->key, &entry->value);
    }
  }
}

ObjString* tableFindString(Table* table, const char* chars, int length, uint32_t hash) {
  if (table->count == 0) return NULL;

  uint32_t index = hash % table->capacity;
  for (;;) {
    Entry* entry = &table->entries[index];

    if (IS_NIL(entry->key)) {
      // stop if an empty non-tombstone entry is found
      if (IS_NIL(entry->value)) return NULL;
    }
    // if there is a hash collision, do a character by character string comparison
    else if (IS_STRING(entry->key)) {
      ObjString* string = AS_STRING(entry->key);
      if (string->length == length && string->hash == hash && memcmp(string->chars, chars, length) == 0) {
        // entry matches -- found!
        return string;
      }

      index = (index + 1) % table->capacity;
    }
  }

  return NULL;
}