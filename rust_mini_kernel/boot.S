.section .multiboot_header, "a"
.align 4
.long 0x1BADB002
.long 0x00000003
.long -(0x1BADB002 + 0x00000003)

.section .bss, "aw", @nobits
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 16384
stack_top:

.section .rodata, "a"
.align 16
gdt64:
    .quad 0
.set GDT_CODE, . - gdt64
    .word 0xffff, 0x0000
    .byte 0x00, 0x9a, 0xaf, 0x00 /* 64-bit code segment */
.set GDT_DATA, . - gdt64
    .word 0xffff, 0x0000
    .byte 0x00, 0x92, 0xcf, 0x00 /* 64-bit data segment */
gdt64_end:

pointer:
    .word gdt64_end - gdt64 - 1
    .long gdt64
    .long 0

.section .text
.code32
.global start
start:
    mov $stack_top, %esp

    /* Zero out page tables */
    mov $p4_table, %edi
    mov $0, %eax
    mov $3072, %ecx /* 3 * 1024 dwords = 12KB */
    rep stosl

    /* Point first entry of p4_table to p3_table */
    mov $p3_table, %eax
    or $0b11, %eax
    mov %eax, p4_table

    /* Point first entry of p3_table to p2_table */
    mov $p2_table, %eax
    or $0b11, %eax
    mov %eax, p3_table

    /* Map 512 * 2MiB = 1GiB */
    mov $0, %ecx
map_p2:
    mov $0x200000, %eax
    mul %ecx
    or $0b10000011, %eax
    mov %eax, p2_table(,%ecx,8)
    inc %ecx
    cmp $512, %ecx
    jne map_p2

    /* Load CR3 */
    mov $p4_table, %eax
    mov %eax, %cr3

    /* Enable PAE (bit 5), OSFXSR (bit 9), OSXMMEXCPT (bit 10) in CR4 */
    mov %cr4, %eax
    or $(1 << 5) | (1 << 9) | (1 << 10), %eax
    mov %eax, %cr4

    /* Set Long Mode bit in EFER MSR */
    mov $0xC0000080, %ecx
    rdmsr
    or $(1 << 8), %eax
    wrmsr

    /* Enable Paging (bit 31), Protection (bit 0), MP (bit 1), clear EM (bit 2) in CR0 */
    mov %cr0, %eax
    and $~(1 << 2), %eax
    or $(1 << 31) | (1 << 0) | (1 << 1), %eax
    mov %eax, %cr0

    /* Load 64-bit GDT */
    lgdt (pointer)

    /* Far jump to 64-bit code segment */
    ljmp $GDT_CODE, $long_mode_start

.code64
long_mode_start:
    mov $GDT_DATA, %ax
    mov %ax, %ss
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    /* Call rust_main */
    call rust_main

    cli
1:  hlt
    jmp 1b

.global default_exception_handler
default_exception_handler:
    cli
2:  hlt
    jmp 2b
