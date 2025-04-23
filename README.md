# calc-rs


## Example
For expr like `(+ 34 35)` asm would be:

```asm
format ELF64
section ".text" executable
public main
extrn printf
msg db "RESULT: %d",10,0
main:
    mov rax, 34
    push rax
    mov rax, 35
    pop rbx
    add rax, rbx

    mov rdi, msg
    mov rsi, rax
    xor rax, rax
    call printf
    mov rax, 0
    ret
```

## TODO
- [ ] Make executable static (for now its linked with libc for printf)
- [ ] Good README
- [ ] Usage reference
