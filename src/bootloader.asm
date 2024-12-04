[org 0x7c00]            ; BIOS load address
[bits 16]               ; 16-bit real mode (Fuck this shit)

KERNEL_OFFSET equ 0x1000     ; kernel load address
MEMORY_MAP_ADDR equ 0x5000   ; mem map storage address

section .text
    global start

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    sti                     ; interupts

    mov [boot_drive], dl

    ; display message 1
    mov si, msg1
    call print_string

    ; display message 2
    mov si, msg2
    call print_string

    
    call enable_a20             ; en A20
    call get_memory_map         ; get mem map
    
    
    mov si, msg_load_kernel     ; kernel loading [TEMP]
    call print_string
    call load_kernel
    lgdt [gdt_descriptor]       ; GDT loading


    cli
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    jmp CODE_SEG:init_pm

[bits 32]
init_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    mov ebp, 0x90000
    mov esp, ebp

    jmp begin_pm

begin_pm:
    cli
    hlt
    jmp $

[bits 16]
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

enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

get_memory_map:
    mov di, MEMORY_MAP_ADDR
    xor ebx, ebx
    mov edx, 0x534D4150
    mov eax, 0xE820
    mov [es:di + 20], dword 1
    mov ecx, 24
    int 0x15
    jc .failed
    ret
.failed:
    mov si, msg_mm_failed
    call print_string
    ret

load_kernel:
    mov ah, 0x02                ; reat sec.
    mov al, 32                  ; no. of sectors
    mov ch, 0                   ; cyl. no.
    mov dh, 0                   ; head n.
    mov cl, 2                   ; start -> sec 2
    mov dl, [boot_drive]
    mov bx, KERNEL_OFFSET
    int 0x13
    jc .disk_error
    ret
.disk_error:
    mov si, msg_disk_error
    call print_string
    jmp $

gdt_start:
    dq 0               ; GDT

gdt_code:
    dw 0xFFFF          ; Limit
    dw 0x0000          ; Base
    db 0x00            ; Base
    db 10011010b       ; Flags
    db 11001111b       ; Flags + Upper Limit
    db 0x00            ; Base

gdt_data:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b
    db 11001111b
    db 0x00

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

boot_drive: db 0
msg1: db 'InfinityX OS V2 - Booting in Protected Mode...', 13, 10, 0
msg2: db 'Made by Pranav Verma.', 13, 10, 0


msg_load_kernel: db 'Loading kernel...', 13, 10, 0
msg_disk_error: db 'Disk read error!', 13, 10, 0
msg_mm_failed: db 'Memory map failed!', 13, 10, 0

times 510-($-$$) db 0
dw 0xaa55

section .stage2
    times 512 db 0