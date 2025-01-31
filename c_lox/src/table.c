#include "table.h"

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

static Entry* findEntry(Entry* entries, int capacity, ObjString* key) {
  // map key's hash code to an index within the array bounds
  uint32_t index     = key->hash % capacity;
  Entry*   tombstone = NULL;

  for (;;) {
    // get the entry by index
    Entry* entry = &entries[index];

    // check for tombstones
    if (entry->key == NULL) {
      // if truely empty, return the entry
      // a truely empty entry is one with NULL key and Nil value
      if (IS_NIL(entry->value)) {
        return tombstone != NULL ? tombstone : entry;
      } else {
        // a tombstone has a value of true and an empty NULL key
        // tombstone found
        if (tombstone == NULL) tombstone = entry;
      }
    } else if (entry->key == key) {
      // key found
      return entry;
    }

    // if the entry key equals the find key
    // or
    // if the entry key is NULL (the bucket contains no entry)
    // if (entry->key == key || entry->key == NULL) {
    //   // entry found, return the entry
    //   return entry;
    // }

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
    entries[i].key   = NULL;
    entries[i].value = NIL_VAL;
  }

  // take the old table entries and copy them into the newly sized entries
  for (int i = 0; i < table->capacity; i++) {
    Entry* entry = &table->entries[i];
    if (entry->key == NULL) continue;

    Entry* dest = findEntry(entries, capacity, entry->key);
    dest->key   = entry->key;
    dest->value = entry->value;
    table->count++;
  }

  FREE_ARRAY(Entry, table->entries, table->capacity);
  table->entries  = entries;
  table->capacity = capacity;
}

bool tableSet(Table* table, ObjString* key, Value value) {
  // check to make sure the entry can fit
  if (table->count + 1 > table->capacity * TABLE_MAX_LOAD) {
    int capacity = GROW_CAPACITY(table->capacity);
    adjustCapacity(table, capacity);
  }

  // find a bucket for the entry
  Entry* entry = findEntry(table->entries, table->capacity, key);

  // update the size of the table
  bool isNewKey = entry->key == NULL;
  // increment the count only if the new entry is not a tombstone (key is null and value is Nil)
  if (isNewKey && IS_NIL(entry->value)) table->count++;

  // copy the entry into the table
  entry->key   = key;
  entry->value = value;

  return isNewKey;
}

bool tableDelete(Table* table, ObjString* key) {
  if (table->count == 0) return false;

  // find the entry
  Entry* entry = findEntry(table->entries, table->capacity, key);

  // nothing to delete if not found
  if (entry->key == NULL) return false;

  // place a tombstone in the entry
  // a tombstone is an entry with no key and a value of true
  entry->key   = NULL;
  entry->value = BOOL_VAL(true);
  return true;
}

bool tableGet(Table* table, ObjString* key, Value* value) {
  if (table->count == 0) return false;

  Entry* entry = findEntry(table->entries, table->capacity, key);
  if (entry->key == NULL) return false;

  *value = entry->value;
  return true;
}

void tableAddAll(Table* from, Table* to) {
  for (int i = 0; i < from->capacity; i++) {
    Entry* entry = &from->entries[i];
    if (entry->key != NULL) {
      tableSet(to, entry->key, entry->value);
    }
  }
}

ObjString* tableFindString(Table* table, const char* chars, int length, uint32_t hash) {
  if (table->count == 0) return NULL;

  uint32_t index = hash % table->capacity;
  for (;;) {
    Entry* entry = &table->entries[index];

    if (entry->key == NULL) {
      // stop if an empty non-tombstone entry is found
      if (IS_NIL(entry->value)) return NULL;
    }
    // if there is a hash collision, do a character by character string comparison
    else if (entry->key->length == length && entry->key->hash == hash &&
             memcmp(entry->key->chars, chars, length) == 0) {
      // entry matches -- found!
      return entry->key;
    }

    index = (index + 1) % table->capacity;
  }
}