#include <stdint.h>
// Define the multiboot header structure
typedef struct multiboot_header
{
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
    uint32_t header_addr;
    uint32_t load_addr;
    uint32_t load_end_addr;
    uint32_t bss_end_addr;
    uint32_t entry_addr;
} multiboot_header_t;

// Define the multiboot header
asm(".section .multiboot\n\t"
    ".long 0x1BADB002\n\t"       // magic
    ".long 0x00           \n\t"  // flags
    ".long -0x1BADB002-0x00\n\t" // checksum
    ".long 0x00           \n\t"  // header_addr
    ".long 0x1000         \n\t"  // load_addr
    ".long 0x1000         \n\t"  // load_end_addr
    ".long 0x00           \n\t"  // bss_end_addr
    ".long 0x1000         \n\t"  // entry_addr
    ".previous");

// Define a macro to easily output to the console
#define VGA_ADDRESS 0xB8000
#define VGA_WIDTH 80
void print_string(char *string)
{
    volatile unsigned short *video_memory = (unsigned short *)VGA_ADDRESS;
    for (int i = 0; string[i] != '\0'; i++)
    {
        unsigned char character = string[i];
        video_memory[i] = (video_memory[i] & 0xFF00) | character;
    }
}

void _start()
{
    // Clear the screen
    volatile unsigned short *video_memory = (unsigned short *)VGA_ADDRESS;
    for (int i = 0; i < VGA_WIDTH * 25; i++)
    {
        video_memory[i] = (video_memory[i] & 0xFF00) | ' ';
    }

    // Print a message to the console
    char message[] = "Welcome to My Operating System!";
    print_string(message);

    // Hang in an infinite loop
    while (1)
    {
    }
}
