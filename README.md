# Elang
This is a simple scripting "language" that you can use to automate tests
requiring ethernet communication. You can use it in either an interactive shell
mode, or a file to be executed.

## Syntax
The goal of the language is to be dead simple for easy automation. As a result,
the language doesn't have a proper lexer or parser, but just looks for reserved
keywords to tell the "interpreter" what to do. All networking overhead is done
by the interpreter for you:

```py
# In general, this "language" has 2 types: strings and integers.
# instructions are of the structure INSTRUCTION: VALUE
# You must begin each script by specifying these 4 parameters, otherwise it is
# invalid syntax and the program will quit
protocol: "udp"
format: "raw"
address: "127.0.0.1"
port: 42

# You will be able to store variables!
store: "udp" as foo

# easily send messages like this
send: "Hello World!"

# you can change parameters on the fly
format: "scpi"
port: 5025

send: "*IDN?"
```

