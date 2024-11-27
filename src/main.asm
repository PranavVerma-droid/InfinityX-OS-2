[org 0x7C00]              ; This is the memory location BIOS loads the boot sector to

start:
    jmp main              ; Jump over the data section

msg db 'Hello, Bootloader!', 0 ; Null-terminated string

main:
    mov ah, 0Eh           ; int 10h / ah=0eh - teletype output
    mov bh, 0            ; Page number
    mov bl, 07h          ; Text attribute

    mov si, msg           ; Point SI to our message
print_char:
    lodsb                 ; Load string byte at [SI] into AL and increment SI
    cmp al, 0             ; If AL=0, end of string
    je done
    int 10h               ; Otherwise, print the character
    jmp print_char        ; And repeat

done:
    cli                   ; Disable interrupts
hang:
    hlt                   ; Halt the CPU
    jmp hang              ; If a NMI happens, halt again

times 510-($-$$) db 0    ; Pad the remainder of the boot sector with zeros
dw 0xAA55                ; Boot sector identifier