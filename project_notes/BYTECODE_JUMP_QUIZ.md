## Foundational Questions

### Q1: You have a Chunk with count = 15. If you're at offset 8 and encounter an OP_CONSTANT instruction, what will the next instruction's offset be? Why?

The next instruction's offset after OP_CONSTANT will be 8 + 1 byte for the instruction and +8 bytes
for the size of the pointer to the data for the operand. Although, because a constant operand could
be on either a 32 or 64 bit system i think this could be 4 or 8 bytes. On my 64 bit system the next
offset will be `8 + 1 + 8 = 17`

### Q2: Look at this bytecode sequence:

```
0000  OP_CONSTANT    0: '5'
0002  OP_CONSTANT    1: '3'
0004  OP_ADD
0005  OP_PRINT
```

Walk me through what happens to the stack pointer as each instruction executes. Start with SP = 0.

```
0000  OP_CONSTANT    0: '5' // run OP_CONSTANT, push(val) SP[0] = '5', SP++
0002  OP_CONSTANT    1: '3' // run OP_CONSTANT, push(val) SP[1] = '3', SP++
0004  OP_ADD // val2 = pop() SP--, val1 = pop() SP--, push(val1 + val2);
0005  OP_PRINT // I think this pops and prints then pushes the value back onto the stack, but I can't remember exactly. So the stack effect would be SP--, print SP++
```

## Jump Logic Questions

### Q3: Here's some pseudocode:

```
if (condition) {
    print "true branch";
} else {
    print "false branch";
}
print "after if";
```

The compiler needs to emit a JUMP_IF_FALSE instruction. At compile time, does the compiler know
where the "false branch" code will be located in the bytecode? What problem does this create?

No, it doesn't know. It creates a problem where the compiler doesn't know what operand to hand to
the jump operation. So the copiler doesn't know where to jump to go past the true branch correctly

### Q4: What is "bytecode patching" and why is it necessary for if-else statements?

Bytecode patching is a compile time operation where the compiler replaces operands or bytecodes in
the list of chunk code. In if-else statements this is necessary because the compiler will only do a
single pass over the if condition. When it reaches the condition statement for the if part of the
branch it does two things:

1. Creates a placeholder operand for the JUMP_IF_FALSE bytecode which is the place to jump to to go
   past the if branch.
2. Keeps track of the the location of JUMP_IF_FALSE bytecode operand so it can be replaced later

The compiler then continues scanning and parsing and when it reaches the closing curly brace of the
first if branch, it knows the length of the branch. Then it patches in the length over the
placeholder that was written before.

### Q5: In this bytecode output:

```
0010  OP_JUMP_IF_FALSE   10 -> 18
0013  OP_CONSTANT        0: 'true'
0015  OP_PRINT
0016  OP_JUMP           16 -> 22
0019  OP_CONSTANT        1: 'false'
0021  OP_PRINT
0022  OP_POP
```

Can you identify:

Where is the condition being tested? What happens if the condition is true? What happens if the
condition is false? Why is there an OP_JUMP at offset 16?

I think the code would be like this:

```
if(value){
 print 'true'
} else {
 print 'false'
}
```

"Where is the condition being tested" I'm not totally sure if you mean "where is the condition"
meaning: 1. the actual condition is not present in the code above, it's value is actually on the top
of the stack. Or 2. literally "where is the code testing the condition" in an if statement.

If the condition is true it runs the if branch and prints true. If the condition is false it jumps
to the else branch and prints false.

Take your time with these! I'm particularly curious about your thinking on Q3 and Q4 since those get
to the heart of why jump patching is tricky.

## Follow-up Challenge: conditional `And`

Since you've got the concepts down, here's a trickier one: How would the compiler handle this?

```
if (a && b) {
    print "both true";
}
```

What's tricky about && that makes it different from a simple if condition? You're really getting
this stuff! The jump patching concept is one of the harder parts of compiler implementation, and
you've clearly grasped why it's necessary and how it works.

`&&` works kind of like a conditional statement: it jumps past the second part of the expression if
the left side operand is false.

When the parser gets to `&&` it emits an `OP_JUMP_IF_FALSE`. The interesting thing is the value on
the stack was pushed before the function for `&&` was called; so the value on the stack is the left
side operand.

Then it pops the value off the stack for the second operand, and processes the right side of the
expression.

so far the vm code for this would look like:

```
OP_JUMP_IF_FALSE 0xFFFF
OP_POP
// Code for b
```

The easy facts to miss is that at `OP_JUMP_IF_FALSE` a placeholder is set and the current count is
stored. After processing b the count has continued to increment and that count is then patched into
the operand of `OP_JUMP_IF_FALSE`. At this point the expression is complete and the compiler goes
back to processing the if statement. Now the if statement will know whether its condition is true or
false.
