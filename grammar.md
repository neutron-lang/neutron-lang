# The sol language grammar definition

### Rules

- Any instruction line will be finished with a semi-colon: `;`.
- Variables and functions names can't be defined with a exactly language keyword.
- Variables and functions names can't be started using a number.
- Variables and functions names can't contain special symbols or spaces.
- All non compiler instructions goes inside a function code block.
- All programs start in the main function, this function can be changed later.

## Special symbols

- `{` + `}`
- `[` + `]`
- `(` + `)`
- `<` + `>`
- `.`
- `,`
- `?`
- `:`
- `;`
- `-# + #-`

## Operators

### Simple operators

- `=` - Atribute value to a variable.
- `+` - Sum value `a` with `b`.
- `-` - Subrtract value `b` from `a`.
- `/` - Divide value `a` by `b`.
- `*` - Multiply value `a` by `b`.
- `%` - Modulus of value `a` by `b`.
- `<` - If value `a` is less than `b` returns true, else returns false.
- `>` - If value `a` is greater than `b` returns true, else returns false.

### Mixed operators

- `+=` - Sum the variable value with the value on the left of the operator and then atribute to the variable.
- `-=` - Subrtract the variable value with the value on the left of the operator and then atribute to the variable.
- `/=` - Divide the variable value by the value on the left of the operator and then atribute to the variable.
- `*=` - Multiply the variable value by the value on the left of the operator and then atribute to the variable.
- `%=` - Modulus of the variable value by the value on the left of the operator and then atribute to the variable.
- `<=` - If the value `a` is less or equal to value `b` returns true, else returns false.
- `>=` - If the value `a` is greater or equal to value `b` returns true, else returns false.
- `->` - Set the return type of a function or acess a internal member of a struct.

## Variable types and return types

- `int` - Any number negative or positive without dot: `2024`.
- `float` - Any number negative or positive with or without dot: `1.0`.
- `bool` - True or false.
- `char` - Any single character: `'A'`.
- `str` - An group of characters: `"Sol"`.
- `void` - Nothing.
- `array` - A list of values of a single type separated by commas: `[1, 2, 3]`.
- `tuple` - A list of values of any type separated by comas: `("Sol", 2024, 1.0, 'a', true)`.
- `dict` - A list of keys with any value: `{"name": "Sol", "edition": 2024, "launched": false}`.
- `any` - A dynamic type, can receive any value, but this type can be slow.
- `null` - Absolute nothing, and it do nothing.

## Syntax definition

### Comments

```
-# The comment goes here #-

```

- `-#` to `#-` - Tells to the compiler to ignore from the first symbol to the second symbol.
- `The comment goes here` - This is the comment content, it will be ignored by the compiler.

### Imports

```
import module_name.specific_sumodule_or_function;

```

- `import` - Tells to the compiler you're importing a module or file to your code.
- `module_name` - The indentifier of the module or file for import.
- `.specific_sumodule_or_function` - If you don't want all the content of the module or file, you can specifie what you want, like a specific class or function.

### Variables

```
var variable_name: type = initial_value;

```

- `var` - Tells to the compiler you're defining a variable.
- `variable_name` - It's the identification of the variable.
- `: type` - It's optional, this define the variable type, if you don't define, it will be `any`.
- `=` - Initialize the variable, this is optional, if you don't initialize, it will be the default value of the type or `null`.

### Functions

```
func function_name(arguments) {
    -# Your code goes here #-
}
```

- `func` - Tells to the compiler you're defining a function.
- `function_name` - The identification of the function.
- `(arguments)` - Any argument for the function.
- `{}` - The function code block.

### Structs

```
struct struct_name {
    -# Struct variables or functions goes here #-
}
```

- `struct` - Tells to the compiler you're defining a struct.
- `struct_name` - The identification of the struct.
- `{}` - The struct code block.

### Classes

```
class class_name {
    -# Class code goes here #-
}
```

- `class` - Tells to the compiler you're defining a class.
- `class_name` - The class identification.
- `{}` - The class code block.
