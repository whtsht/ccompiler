# C Compiler in Rust

# EBNF
num
    := 0 | 1 | ... | 255 |

expr
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
    := num | "(" expr ")"
