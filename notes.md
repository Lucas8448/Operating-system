Notes
-----

Assemble bootloader:

"C:\Program Files\NASM\nasm.exe" -f bin -o bootloader.bin bootloader.asm

Run OS:

"C:\Program Files\qemu\qemu-system-i386.exe" -fda bootloader.bin

Compile kernel:

Open WSL, cd to folder and paste: i686-elf-gcc -c kernel.c -o kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

