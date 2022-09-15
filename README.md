# C Compiler in Rust

# EBNF
num
    := 0 | 1 | ... | 255 |

expr
    := mul ("+" mul | "-" mul)*

mul
    := primary ("*" primary | "-" primary)*

primary
    := num | "(" expr ")"
