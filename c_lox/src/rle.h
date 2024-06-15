#include "chunk.h"

typedef struct {
  int length;
  int capacity;
  char* encodedData;
} RleData;

RleData* rleInit(int capacity);
RleData* rleEncodeLines(int* data, int length);
int* rleDecodeLines(RleData* data);
