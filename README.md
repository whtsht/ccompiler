# C Compiler in Rust

# EBNF

```
program
    := stmt*

stmt
    := expr ";"
    | "return" expr ";"
    | "if" "(" expr ")" stmt ("else" stmt)?

expr
    := assign

assign
    := equality ("=" assign)?

equality
    := relational ("==" relational | "!=" relational)*

relational
    := add ("<" add | "<=" add | ">" add | ">=" add)*

add
    := mul ("+" mul | "-" mul)*

mul
    := unary ("*" unary | "-" unary)*

unary
    := ("+" | "-")? primary

primary
    := num | ident | "(" expr ")"

num
    := 0 | 1 | ... | 255

ident
    := a | b | ... | z
```
