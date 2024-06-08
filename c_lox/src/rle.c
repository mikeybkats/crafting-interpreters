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

void rleGrowIfNeeded(RleData* rleData, int additionalLength) {
  while (rleData->length + additionalLength >= rleData->capacity) {
    rleData->capacity = GROW_CAPACITY(rleData->capacity);
    rleData->encodedData =
        (char*)realloc(rleData->encodedData, rleData->capacity * sizeof(char));
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

int* rleDecodeLines(RleData* data) {
  int* decodedData = malloc(sizeof(int));

  char* lineNumber = malloc(sizeof(char));
  char* multiplier = malloc(sizeof(char));

  for (int x = 0; x < data->length; x++) {
    int i = 0;

    // get the line number
    while (data->encodedData[i] != 'x') {
      if (data->encodedData[i] != ' ') {
        lineNumber[i] = data->encodedData[i];
      }
      i++;
    }

    // get the multiplier (number of lines)
    while (data->encodedData[i] != ',') {
      multiplier[i] = data->encodedData[i];
      i++;
    }

    int lineNumberInt = atoi(lineNumber);
    int multiplierInt = atoi(multiplier);

    printf("lineNumberInt: %d, multiplierInt: %d\n", lineNumberInt,
           multiplierInt);

    for (int j = x; j < multiplierInt; x++) {
      decodedData[j] = lineNumberInt;
    }
  }

  return decodedData;
}

// int getLine(int index) { return 0; }