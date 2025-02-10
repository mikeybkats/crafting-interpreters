#ifndef clox_table_h
#define clox_table_h

#ifdef DEBUG_TEST
#define STATIC
#else
#define STATIC static
#endif

#include "common.h"
#include "value.h"

typedef struct
{
  Value key;
  Value value;
} Entry;

typedef struct
{
  int    count;
  int    capacity;
  Entry* entries;
} Table;

void initTable(Table* table);
void freeTable(Table* table);

#ifdef DEBUG_TEST
STATIC Entry* findEntry(Entry* entries, int capacity, Value* key);
#endif

bool       tableSet(Table* table, Value* key, Value value);
bool       tableDelete(Table* table, Value* key);
bool       tableGet(Table* table, Value* key, Value* value);
void       tableAddAll(Table* from, Table* to);
ObjString* tableFindString(Table* table, const char* chars, int length, uint32_t hash);
uint32_t   getHashValue(Value value);
int        getEntryIndex(Table* table, Value* key);
bool       isTombstone(Entry* entry);

#endif