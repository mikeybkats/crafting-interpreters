CXX=clang
CXXFLAGS=-std=c11 -Wall
TARGET=build/src/outputExecutable
SRC_DIR=src
BUILD_DIR=build/src

SRC_FILES=$(wildcard $(SRC_DIR)/*.c)
OBJ_FILES=$(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(SRC_FILES))

# Ensure the output directory exists
# $(shell mkdir -p build)
$(shell mkdir -p build $(BUILD_DIR))


$(TARGET): $(OBJ_FILES)
	$(CXX) $(CXXFLAGS) -o $@ $^

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	$(CXX) $(CXXFLAGS) -c $< -o $@	

clean:
	rm -f $(OBJ_FILES) $(TARGET)
