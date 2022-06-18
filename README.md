# Oxido

Oxido is a dynamic interpreted programming language basing most of its syntax on Rust. Conventionally, the files may end with the `o` extension, however Oxido ignores the extension.

## Syntax

### Data types

* String: A string is any value inside `"` (double quotes) passing the regex `\"[A-Za-z0-9 !]+\"`.

* Integer: Integers (no fractions), passing the regex `[0-9]+`.

### Variables

Variables are declared by the `let` keyword, followed by a space and the identifier, which must pass the regex `[A-Za-z]+`, followed by an equals sign and the value which must be a single string or an integer or an expression of integers, or a variable itself.

For example:

```ox
let a = "Hi mom!";
let a = 5;
let a = 5 * 5;
```

### Reassignments

Reassignments are done stating the identifier, which must pass the regex `[A-Za-z]+`, followed by an equals sign and the value which must be a single string or an integer or an expression of integers, or a variable itself.

```ox
let a = 0;
a = "Hi mom!";
a = 5;
a = 5 * 5;
```

### Printing

The `print` keyword can be used to print variables, expressions and strings to stdout, the value must be inside two parentheses.

```ox
print(a);
print(5);
print(5 * 5);
print("Hello world");
```

### If statements

If statements check whether the given condition is true or not using the `==` or `<` or `>` operator. The `==` is applicable on strings and integers both, while `<` or `>` can only be used on integers. The condition must be followed after the code to be executed in the case the condition is true in curly braces `{}`.

```ox
if a == 5 {
    print(a);
}
```

### Loop statements

Loop statements repeat given conditions until `break` is called.The conditions to be executed in the loop must be followed after the `loop` keyword in `{}`.

```ox
let b = 0;

loop {
    b = b + 1;

    if b == 5 {
        print("Hi mom!");
        break;
    }

    print(b);
}
```

