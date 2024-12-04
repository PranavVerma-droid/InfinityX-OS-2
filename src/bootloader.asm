[org 0x7c00]            ; BIOS loads from this address
[bits 16]               ; 16-bit real mode (Fuck this shit)


start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00              ; Stack Setup


    mov si, msg
    call print_string


    ; Infinity Loop
    cli
    hlt
    jmp $

print_string:
    pusha
    mov ah, 0x0e

.loop:
    lodsb
    or al, al
    jz .done
    int 0x10
    jmp .loop

.done:
    popa
    ret

msg: db 'InfinityX OS V2 - Made by Pranav Verma.', 13, 10, 0

times 510-($-$$) db 0           ; Zeroes Padding
dw 0xaa55                       ; Boot Sig.