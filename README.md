# C Compiler in Rust

# EBNF
num
    := 0 | 1 | ... | 255 |

expr
    := mul ("+" mul | "-" mul)*

mul
    := unary ("*" unary | "-" unary)*

unary
    := ("+" | "-")? primary

primary
    := num | "(" expr ")"
