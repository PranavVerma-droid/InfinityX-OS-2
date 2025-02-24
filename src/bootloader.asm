[org 0x7c00]            ; Tell assembler where this code will be loaded

; Set up segments
mov ax, 0               ; Initialize segments to 0
mov ds, ax
mov es, ax
mov ss, ax
mov sp, 0x7c00         ; Stack grows downwards from where we are loaded

; Print welcome message
mov si, welcome_msg     ; Load message address into SI register
call print_string      ; Call our string-printing function

; Infinite loop
jmp $

; Function to print a string
print_string:
    pusha               ; Save all registers
    mov ah, 0x0e        ; BIOS teletype function

.loop:
    lodsb               ; Load byte from SI into AL and increment SI
    cmp al, 0           ; Check if we've hit the null terminator
    je .done            ; If so, we're done (yes)
    int 0x10            ; Otherwise, print the character
    jmp .loop           ; And continue looping

.done:
    popa                ; Restore all registers
    ret

; Data section
welcome_msg: db 'InfinityX OS V2 Bootloader.', 0x0D, 0x0A, 0   ; Message with newline and null terminator

; Boot sector padding
times 510-($-$$) db 0   ; Fill the rest of sector with zeros
dw 0xaa55              ; Boot signature