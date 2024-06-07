#include "rle.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

RleData* rleInit(int capacity) {
  RleData* data = (RleData*)malloc(sizeof(RleData));
  data->encodedData = (char*)malloc(capacity * sizeof(char*));
  data->capacity = capacity;
  data->length = 0;
  return data;
}

// char* formatLengthCode(int count) {
//   char lengthPrefix[6] = "x";

//   // TODO: how is this fixed size buffer determined?
//   char* lengthCode = malloc(14 * sizeof(char));
//   sprintf(lengthCode, "%s %d", lengthPrefix, count);

//   return lengthCode;
// }

// char* formatEncodedEntry(char* lengthCode, int entry) {
//   char* encodedEntry = malloc(14 * sizeof(char));
//   sprintf(encodedEntry, "%d %s", entry, lengthCode);

//   return encodedEntry;
// }

void rleGrowIfNeeded(RleData* rleData, int additionalLength) {
  while (rleData->length + additionalLength >= rleData->capacity) {
    rleData->capacity = GROW_CAPACITY(rleData->capacity);
    // rleData->encodedData =
    //     (char*)realloc(rleData->encodedData, rleData->capacity *
    //     sizeof(char));
  }
}

void writeEncodedData(RleData* encodedLines, char* encodedData) {
  int strLength = strlen(encodedData);
  encodedLines->encodedData[encodedLines->length] =
      malloc((strLength + 1) * sizeof(char));

  strcpy(encodedLines->encodedData[encodedLines->length], encodedData);
}

char* rleEncodeLines(Chunk* data, int line) {
  // Initialize an empty result string.
  char* result = malloc(data->capacity * sizeof(char));
  return result;
}

RleData* rleEncodeLine(int* data, int length) {
  // Initialize an empty result string.
  RleData* encodedLines = rleInit(14);

  // Traverse the input data.
  for (int i = 0; i < length; i++) {
    // For each entry in the data, count its consecutive occurrences.
    int entry = data[i];
    int occurencesCount = 0;

    for (int j = i; j < length; j++) {
      if (data[j] == entry) {
        occurencesCount++;
        i++;
      } else {
        break;
      }
    }
    i--;

    // Append the count and the character to the result string.
    // char* lengthCode = formatLengthCode(occurencesCount);
    // char* encodedData = formatEncodedEntry(lengthCode, entry);
    char lengthCode[20];  // Use a fixed-size buffer
    snprintf(lengthCode, sizeof(lengthCode), "%d", occurencesCount);
    char encodedData[50];  // Use a fixed-size buffer
    snprintf(encodedData, sizeof(encodedData), "%d x %s", entry, lengthCode);
    writeEncodedData(encodedLines, encodedData);
    // encodedLines->length++;
  }

  return encodedLines;
}

int* rleDecode(RleData* data) {
  int* decodedData = malloc(sizeof(int));

  return decodedData;
}

int getLine(int index) { return 0; }