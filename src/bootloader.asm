[org 0x7c00]            ; Tell assembler where this code will be loaded (lul)

; Set up segments (FUCK)
mov ax, 0               ; Initialize segments to 0
mov ds, ax
mov es, ax
mov ss, ax
mov sp, 0x7c00         ; Stack grows downwards from where we are loaded

; Print a character
mov ah, 0x0e           ; BIOS teletype output
mov al, 'H'            ; Character to print
int 0x10               ; Print it

; Infinite loop
jmp $

; Boot sector padding
times 510-($-$$) db 0  ; Fill the rest of sector with zeros
dw 0xaa55             ; Boot signature at the end of bootloader