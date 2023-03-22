BITS 16
org 0x7c00

start:
    ; Set up the stack
    xor ax, ax
    mov ss, ax
    mov sp, 0x7c00

    mov si, hello_msg
    call print
    ; Load kernel from disk
    mov bx, 0x1000 ; Destination address for kernel code
    mov dh, 1 ; Head number
    mov dl, 0 ; Drive number
    mov cx, 2 ; Sector count
    mov ah, 0x02 ; BIOS read sector function
    mov al, 0x01 ; Sector count to read
    mov ch, 0x00 ; Cylinder number
    mov cl, 0x02 ; Sector number
    int 0x13 ; BIOS disk interrupt
    
    ; Transfer control to kernel
    jmp 0x1000 ; Jump to the start of kernel code

hang:
    call get_key
    mov ah, 0x0E
    int 0x10
    jmp hang


print:
    lodsb
    or al, al
    jz done
    mov ah, 0x0E
    int 0x10
    jmp print
done:
    ret

get_key:
    xor ah, ah
    int 0x16
    ret

hello_msg db 'Welcome to badOS (TM)', 0

times 510-($-$$) db 0
dw 0xAA55