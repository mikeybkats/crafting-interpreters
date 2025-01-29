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
  uint32_t index = key->hash % capacity;

  for (;;) {
    // get the entry by index
    Entry* entry = &entries[index];

    // if the entry key equals the find key
    // or
    // if the entry key is NULL (the bucket contains no entry)
    if (entry->key == key || entry->key == NULL) {
      // entry found, return the entry
      return entry;
    }

    index = (index + 1) % capacity;
  }
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
  if (isNewKey) table->count++;

  // copy the entry into the table
  entry->key   = key;
  entry->value = value;

  return isNewKey;
}