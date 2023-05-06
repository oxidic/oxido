# Oxido

- [Oxido](#oxido)
  - [Files:](#files)
  - [Installation](#installation)
  - [Uninstallation](#uninstallation)
  - [Usage](#usage)
  - [Syntax](#syntax)
    - [Data types](#data-types)
    - [Variables](#variables)
    - [Reassignments](#reassignments)
    - [If statements](#if-statements)
    - [Loop statements](#loop-statements)
    - [Functions](#functions)
    - [Exiting](#exiting)
  - [Standard Library](#standard-library)
    - [IO](#io)
      - [print()](#print)
      - [println()](#println)
    - [Types](#types)
      - [str()](#str)
      - [int()](#int)
      - [bool()](#bool)


Oxido is a dynamic interpreted programming language basing most of its syntax on Rust.

## Files:

The files may end with the `oxi` extension, however the extension is ignored.

Oxido uses waterfall approach to files, for a directory/file `example`, Oxido will run the first file in order of, `example`, `example/main.oxi`, `example/src/main.oxi`.

## Installation

[Oxate](https://github.com/oxidite/oxate), the official installer can be used to install the latest release from GitHub.

```bash
oxate install
```

## Uninstallation

You can use `oxate` to remove the current installation.

```bash
oxate uninstall
```

## Usage

You can run an Oxido file using the `oxido` command. The files may end with the `oxi` extension, however Oxido ignores the extension, and runs the file as-is.

```bash
oxido <FILE> [OPTIONS]
```

For example:

```bash
oxido main.oxi
```

Conventionally, Oxido files are named `main.oxi`.

## Syntax

### Data types

* String: A string is any value inside `"` (double quotes) passing the regex `\"[A-Za-z0-9 !]+\"`.

* Int: Integers (no fractions), passing the regex `[0-9]+`.

* Bool: `true` or `false`
  
* Vec: A uniform collection of the other data types, denoted by `[T]`.

### Variables

Variables are declared by the `let` keyword, followed by the identifier, which must pass the regex `[A-Za-z]+`, followed by the data type (optional) and an equal sign and the expression.

For example:

```rs
let a: str = "Hi mom!";
let n = 5;
let z = 5 * 5;
let f: int = factorial(5);
```

### Reassignments

Reassignments are the same as assignments with the condition that `let` keyword is not used and the variable must have been declared before. Data types are enfored while reassigning values.

```rs
let a: int = 0;
a = 5;
a = 5 * 5;
a = factorial(5);
a = "Hi mom!"; // error: incorrect data type
```

### If statements

If statements check whether the given condition is true or not using the `==` or `<` or `>` operator. The `==` is applicable on strings and integers both, while `<` or `>` can only be used on integers. The condition must be followed after the code to be executed in the case the condition is true in curly braces `{}`.

```rs
if a == 5 {
    print(a);
}
```

### Loop statements

Loop statements repeat given conditions until `break` is called.The conditions to be executed in the loop must be followed after the `loop` keyword in `{}`.

```rs
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

### Functions

Funcitons store the given conditions until they are called. They are declared with the name of the function, the name must be a valid identifier, followed by args, seperated by commas in `()` and the statement in `{}`.

```rs
let text = "Hi mom!";

fn message(x: str) -> int {
    print(x);
    return 0;
}

message(text);
```

### Exiting

The `exit` keyword can be used to exit the program with the specified exit code

```rs
print("Hi mom!");

exit(0);
```

## Standard Library

Oxido includes a standard library which can be used for basic functions.

### IO

#### print()

Print the given inputs to stdout.

#### println()

Print the given inputs to stdout and leave a newline.

### Types

#### str()

Convert the value to str data type

#### int()

Convert the value to int data type

#### bool()

Convert the value to bool data type