#include "chunk.h"

typedef struct {
  int length;
  int capacity;
  char* encodedData;
} RleData;

RleData* rleInit(int capacity);
RleData* rleEncodeLine(int* data, int length);
char* rleEncodeLines(Chunk* data, int line);
int* rleDecode(RleData* data);
