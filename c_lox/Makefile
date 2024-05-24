CXX=clang
CXXFLAGS=-std=c11 -Wall
TARGET=build/outputExecutable
SRC=$(wildcard *.c)
# OBJ=$(SRC:.c=.o)
OBJDIR=build
OBJ=$(patsubst %.c,$(OBJDIR)/%.o,$(SRC))

# Ensure the output directory exists
# $(shell mkdir -p build)
$(shell mkdir -p build $(OBJDIR))


$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^

# %.o: %.c
# 	$(CXX) $(CXXFLAGS) -c $<

$(OBJDIR)/%.o: %.c
	$(CXX) $(CXXFLAGS) -c $< -o $@	

clean:
	rm -f $(OBJ) $(TARGET) ${OBJDIR}
