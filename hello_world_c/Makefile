CC = clang
CFLAGS = -Wall -Wextra

# Directory containing your C source files
SRC_DIR = src

# Get a list of all C source files in the SRC_DIR
SRC_FILES = $(wildcard $(SRC_DIR)/*.c)

# Generate a list of corresponding executable names
EXE_NAMES = $(patsubst $(SRC_DIR)/%.c,%,$(SRC_FILES))

# Build all executables
all: $(EXE_NAMES)

# Rule to build each executable
$(EXE_NAMES): % : $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(EXE_NAMES)
