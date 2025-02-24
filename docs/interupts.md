# BIOS Interupts 

## Video Services (INT 0x10)

### Set Video Mode
```
mov ah, 0x00      ; Function: Set video mode
mov al, MODE      ; Mode number (0x03 = 80x25 text mode)
int 0x10
```

### Write Character (TTY Mode)

```
mov ah, 0x0E      ; Function: TTY output
mov al, CHAR      ; Character to print
int 0x10
```

### Write String
```
mov ah, 0x13      ; Function: Write string
mov al, 1         ; Write mode
mov bh, 0         ; Page number
mov bl, COLOR     ; Color attribute
mov cx, LENGTH    ; String length
mov dh, ROW       ; Row
mov dl, COL       ; Column
mov es, STRING_SEGMENT
mov bp, STRING_OFFSET
int 0x10
```

## Disk Services (INT 0x13)

### Reset Disk System
```
mov ah, 0x00      ; Function: Reset disk
mov dl, DRIVE     ; Drive number
int 0x13
```

### Read Sectors
```
mov ah, 0x02      ; Function: Read sectors
mov al, SECTORS   ; Number of sectors
mov ch, CYLINDER  ; Cylinder number
mov cl, SECTOR    ; Sector number
mov dh, HEAD      ; Head number
mov dl, DRIVE     ; Drive number
mov bx, BUFFER    ; Buffer address
int 0x13
```

### Write Sectors
```
mov ah, 0x03      ; Function: Write sectors
mov al, SECTORS   ; Number of sectors
mov ch, CYLINDER  ; Cylinder number
mov cl, SECTOR    ; Sector number
mov dh, HEAD      ; Head number
mov dl, DRIVE     ; Drive number
mov bx, BUFFER    ; Buffer address
int 0x13
```

## Keyboard Services (INT 0x16)

### Read Keyboard Input
```
mov ah, 0x00      ; Function: Read keyboard
int 0x16          ; Returns: AL = ASCII, AH = Scan code
```

### Check for Keystroke
```
mov ah, 0x01      ; Function: Check keyboard status
int 0x16          ; ZF set if no keystroke
```

## Memory Services (INT 0x15)

### Get Extended Memory Size
```
mov ah, 0x88      ; Function: Get extended memory size
int 0x15          ; Returns: AX = KB above 1MB
```

### Get Memory Map
```
mov eax, 0xE820   ; Function: Get memory map
mov edx, 0x534D4150  ; 'SMAP'
mov ecx, 24       ; Size of buffer
mov di, buffer    ; Buffer address
int 0x15
```

## Time Services (INT 0x1A)

### Get System Time
```
mov ah, 0x00      ; Function: Get system time
int 0x1A          ; Returns: CX:DX = clock ticks
```

### Get Real-Time Clock
```
mov ah, 0x02      ; Function: Get RTC time
int 0x1A          ; CH=hours, CL=minutes, DH=seconds
```



