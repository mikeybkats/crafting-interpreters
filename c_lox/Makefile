CXX=clang
CXXFLAGS=-std=c11 -Wall
TARGET=build/outputExecutable
SRC=$(wildcard *.c)
OBJ=$(SRC:.c=.o)

# Ensure the output directory exists
$(shell mkdir -p build)

$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^

%.o: %.c
	$(CXX) $(CXXFLAGS) -c $<

clean:
	rm -f $(OBJ) $(TARGET)
