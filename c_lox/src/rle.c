#include "rle.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const int REALLOCATION_BUFFER = 2;

RleData* rleInit(int capacity) {
  RleData* data     = (RleData*)malloc(sizeof(RleData));
  data->encodedData = (char*)malloc(capacity * sizeof(char*));
  data->capacity    = capacity;
  data->length      = 0;
  return data;
}

void rleGrowIfNeeded(RleData* rleData, int additionalLength) {
  while (rleData->length + additionalLength >= rleData->capacity) {
    rleData->capacity    = GROW_CAPACITY(rleData->capacity);
    rleData->encodedData = (char*)realloc(rleData->encodedData, rleData->capacity * sizeof(char));
  }
}

void writeEncodedData(RleData* rleData, const char* encodedData) {
  // get the length
  int entryLength = strlen(encodedData);

  // grow the array if needed
  rleGrowIfNeeded(rleData, entryLength);

  // copy the data into the memory location
  memcpy(rleData->encodedData + rleData->length, encodedData, entryLength);

  // update the length in the rleData
  rleData->length += entryLength;

  // ensure string null termination
  rleData->encodedData[rleData->length] = '\0';
}

RleData* rleEncodeLines(int* data, int length) {
  // Initialize an empty result string.
  RleData* encodedLines = rleInit(14);

  // Traverse the input data.
  for (int i = 0; i < length; i++) {
    // For each entry in the data, count its consecutive occurrences.
    int entry           = data[i];
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

    char lengthCode[20];  // Use a fixed-size buffer
    snprintf(lengthCode, sizeof(lengthCode), "%d", occurencesCount);
    char encodedData[50];  // Use a fixed-size buffer
    snprintf(encodedData, sizeof(encodedData), "%d x %s", entry, lengthCode);

    writeEncodedData(encodedLines, encodedData);
    if (i < length - 1) {
      writeEncodedData(encodedLines, ", ");
    }
  }

  return encodedLines;
}

int* rleDecodeLines(RleData* data, int* decodedLength) {
  int* decodedData  = malloc(sizeof(int) * data->capacity);
  int  decodedCount = 0;

  char lineNumber[20];
  char multiplier[20];

  int x = 0;  // x is the cursor for selection
  while (x < data->length - 1) {
    int i = 0;  // i is the target for writing data

    // get the line number
    while (data->encodedData[x] != 'x') {
      // loop through the line number with i
      if (data->encodedData[x] != ' ') {
        lineNumber[i] = data->encodedData[x];
        i++;
      }
      x++;
    }

    lineNumber[i] = '\0';

    i = 0;  // reset target for writing data
    x = x + 2;

    // get the multiplier (number of lines)
    while (data->encodedData[x] != ',' && data->encodedData[x] != '\0') {
      if (data->encodedData[x] != ' ') {
        multiplier[i] = data->encodedData[x];
        i++;
      }
      x++;
    }
    multiplier[i] = '\0';

    int lineNumberInt = atoi(lineNumber);
    int multiplierInt = atoi(multiplier);

    // Ensure there's enough space in decodedData
    // if the total number of entries is greater than the capacity
    if (multiplierInt + decodedCount >= data->capacity * REALLOCATION_BUFFER) {
      GROW_CAPACITY(data->capacity);
      decodedData = realloc(decodedData, sizeof(int) * data->capacity);
    }

    for (int j = 0; j < multiplierInt; j++) {
      decodedData[decodedCount] = lineNumberInt;
      decodedCount++;
    }

    x++;
  }

  // why is decoded length needed?
  *decodedLength = decodedCount;
  return decodedData;
}

int getLine(RleData* data, int offset) {
  int  length      = 0;
  int* lineNumbers = rleDecodeLines(data, &length);

  return lineNumbers[offset];
}