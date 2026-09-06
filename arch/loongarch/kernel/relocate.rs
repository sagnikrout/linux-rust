//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/relocate.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// Support for Kernel relocation at boot time
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

    static unsigned long reloc_offset;
#[no_mangle]
pub unsafe extern "C" fn relocate_relative() -> void __init {
    static inline void __init relocate_relative(void)
    {
    Elf64_Rela *rela, *rela_end;
    rela = (Elf64_Rela *)&__rela_dyn_begin;
    rela_end = (Elf64_Rela *)&__rela_dyn_end;
    for ( ; rela < rela_end; rela++) {
    let mut addr: Elf64_Addr = rela.r_offset;
    let mut relocated_addr: Elf64_Addr = rela.r_addend;
    if (rela.r_info != R_LARCH_RELATIVE)
    continue;
    relocated_addr = (Elf64_Addr)RELOCATED(relocated_addr);
// (Elf64_Addr *)RELOCATED(addr) = relocated_addr;
    }

    u64 *addr = core::ptr::null_mut();
    u64 *relr = (u64 *)&__relr_dyn_begin;
    u64 *relr_end = (u64 *)&__relr_dyn_end;
    for ( ; relr < relr_end; relr++) {
    if ((*relr & 1) == 0) {
    addr = (u64 *)(*relr + reloc_offset);
// addr++ += reloc_offset;
    } else {
    for (u64 *p = addr, r = *relr >> 1; r; p++, r >>= 1)
    if (r & 1)
// p += reloc_offset;
    addr += 63;
    }
    }

    }
#[no_mangle]
pub unsafe extern "C" fn relocate_absolute(random_offset: c_long) -> void __init {
    static inline void __init relocate_absolute(long random_offset)
    {
    void *begin, *end;
    struct rela_la_abs *p;
    begin = RELOCATED_KASLR(&__la_abs_begin);
    end   = RELOCATED_KASLR(&__la_abs_end);
    for (p = begin; (void *)p < end; p++) {
    let mut v: c_long = p.symvalue;
    uint32_t lu12iw, ori;

    uint32_t lu32id, lu52id;

    union loongarch_instruction *insn = (void *)p.pc;
    lu12iw = (v >> 12) & 0xfffff;
    ori    = v & 0xfff;

    lu32id = (v >> 32) & 0xfffff;
    lu52id = v >> 52;

    insn[0].reg1i20_format.immediate = lu12iw;
    insn[1].reg2i12_format.immediate = ori;

    insn[2].reg1i20_format.immediate = lu32id;
    insn[3].reg2i12_format.immediate = lu52id;

    }
    }

    static inline __init unsigned long rotate_xor(unsigned long hash,
    const void *area, size_t size)
    {
    size_t i, diff;
    const typeof(hash) *ptr = PTR_ALIGN(area, sizeof(hash));
    diff = (void *)ptr - area;
    if (size < diff + sizeof(hash))
    return hash;
    size = ALIGN_DOWN(size - diff, sizeof(hash));
    for (i = 0; i < size / sizeof(hash); i++) {
// Rotate by odd number of bits and XOR.
    hash = (hash << ((sizeof(hash) * 8) - 7)) | (hash >> 7);
    hash ^= ptr[i];
    }
    return hash;
    }
#[no_mangle]
pub unsafe extern "C" fn get_random_boot() -> __init unsigned long {
    static inline __init unsigned long get_random_boot(void)
    {
    let mut hash: c_ulong = 0;
    let mut entropy: c_ulong = random_get_entropy();
// Attempt to create a simple but unpredictable starting entropy.
    hash = rotate_xor(hash, linux_banner, strlen(linux_banner));
// Add in any runtime entropy we can get
    hash = rotate_xor(hash, &entropy, sizeof(entropy));
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn nokaslr(p: *mut c_char) -> int __init {
    static int __init nokaslr(char *p)
    {
    return 0; /* Just silence the boot warning */
    }
    early_param("nokaslr", nokaslr);

//
// Note: strictly-defined KASLR means the kernel's final runtime address
// has a random offset from the kernel's load address, which is implemented
// in relocate.c; broadly-defined KALSR means the kernel's final runtime
// address has a random offset from the kernel's link address (a.k.a.
// VMLINUX_LOAD_ADDRESS), which also include the efistlub implementation,
// kexec_file implementation and QEMU direct kernel boot. kaslr_disabled()
// return true only means strictly-defined KASLR is disabled.
//
#[no_mangle]
pub unsafe extern "C" fn kaslr_disabled() -> __init bool {
    static inline __init bool kaslr_disabled(void)
    {
    char *str;
    const char *builtin_cmdline = CONFIG_CMDLINE;
    if (kaslr_offset())
    return true; /* KASLR is performed during early boot. */
    str = strstr(builtin_cmdline, "nokaslr");
    if (str == builtin_cmdline || (str > builtin_cmdline && *(str - 1) == ' ')) {
    pr_info(KASLR_DISABLED_MESSAGE, "\'nokaslr\'", "built-in");
    return true;
    }
    str = strstr(boot_command_line, "nokaslr");
    if (str == boot_command_line || (str > boot_command_line && *(str - 1) == ' ')) {
    pr_info(KASLR_DISABLED_MESSAGE, "\'nokaslr\'", "bootloader");
    return true;
    }

    str = strstr(builtin_cmdline, "nohibernate");
    if (str == builtin_cmdline || (str > builtin_cmdline && *(str - 1) == ' '))
    return false;
    str = strstr(boot_command_line, "nohibernate");
    if (str == boot_command_line || (str > boot_command_line && *(str - 1) == ' '))
    return false;
    str = strstr(builtin_cmdline, "noresume");
    if (str == builtin_cmdline || (str > builtin_cmdline && *(str - 1) == ' '))
    return false;
    str = strstr(boot_command_line, "noresume");
    if (str == boot_command_line || (str > boot_command_line && *(str - 1) == ' '))
    return false;
    str = strstr(builtin_cmdline, "resume=");
    if (str == builtin_cmdline || (str > builtin_cmdline && *(str - 1) == ' ')) {
    pr_info(KASLR_DISABLED_MESSAGE, "\'resume=\'", "built-in");
    return true;
    }
    str = strstr(boot_command_line, "resume=");
    if (str == boot_command_line || (str > boot_command_line && *(str - 1) == ' ')) {
    pr_info(KASLR_DISABLED_MESSAGE, "\'resume=\'", "bootloader");
    return true;
    }

    str = strstr(boot_command_line, "kexec_file");
    if (str == boot_command_line || (str > boot_command_line && *(str - 1) == ' ')) {
    pr_info(KASLR_DISABLED_MESSAGE, "\'kexec_file\'", "bootloader");
    return true;
    }
    return false;
    }
// Choose a new address for the kernel
    static inline void __init *determine_relocation_address(void)
    {
    unsigned long kernel_length;
    unsigned long random_offset;
    void *destination = _text;
    if (kaslr_disabled())
    return destination;
    kernel_length = (unsigned long)_end - (unsigned long)_text;
    random_offset = get_random_boot() << 16;
    random_offset &= (CONFIG_RANDOMIZE_BASE_MAX_OFFSET - 1);
    if (random_offset < kernel_length)
    random_offset += ALIGN(kernel_length, 0xffff);
    return RELOCATED_KASLR(destination);
    }
#[no_mangle]
unsafe extern "C" fn determine_initrd_address(size: *mut c_ulong) -> unsigned long __init {
    static unsigned long __init determine_initrd_address(unsigned long *size)
    {
    let mut start: c_ulong = 0;
    unsigned long key_length;
    char *p, *endp, *key = "initrd=";
    key_length = strlen(key);
    p = strstr(boot_command_line, key);
    if (!p) {
    key = "initrdmem=";
    key_length = strlen(key);
    p = strstr(boot_command_line, key);
    }
    if (p == boot_command_line || (p > boot_command_line && *(p - 1) == ' ')) {
    p += key_length;
    start = memparse(p, &endp);
    if (*endp == ',')
// size = memparse(endp + 1, NULL);
    }
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn relocation_addr_valid(location_new: *mut c_void) -> int __init {
    static inline int __init relocation_addr_valid(void *location_new)
    {
    unsigned long kernel_start, kernel_size;
    unsigned long initrd_start, initrd_size = 0;
    if ((unsigned long)location_new & 0x00000ffff)
    return 0; /* Inappropriately aligned new location */
    if ((unsigned long)location_new < (unsigned long)_end)
    return 0; /* New location overlaps original kernel */
    initrd_start = determine_initrd_address(&initrd_size);
    if (initrd_start && initrd_size) {
    kernel_start = PHYSADDR(location_new);
    kernel_size = (unsigned long)_end - (unsigned long)_text;
    if (kernel_start < (initrd_start + initrd_size) &&
    initrd_start < (kernel_start + kernel_size))
    return 0; /* initrd/initramfs overlaps kernel */
    }
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn update_reloc_offset(addr: *mut c_ulong, random_offset: c_long) -> void __init {
    static inline void __init update_reloc_offset(unsigned long *addr, long random_offset)
    {
    unsigned long *new_addr = (unsigned long *)RELOCATED_KASLR(addr);
// new_addr = (unsigned long)reloc_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn relocate_kernel() -> unsigned long __init {
    unsigned long __init relocate_kernel(void)
    {
    unsigned long kernel_length;
    let mut random_offset: c_ulong = 0;
    void *location_new = _text; /* Default to original kernel start */
    char *cmdline = early_memremap_ro(fw_arg1, COMMAND_LINE_SIZE); /* Boot command line is passed in fw_arg1 */
    strscpy(boot_command_line, cmdline, COMMAND_LINE_SIZE);

    location_new = determine_relocation_address();
// Sanity check relocation address
    if (relocation_addr_valid(location_new))
    random_offset = (unsigned long)location_new - (unsigned long)(_text);

    reloc_offset = (unsigned long)_text - VMLINUX_LOAD_ADDRESS;
    early_memunmap(cmdline, COMMAND_LINE_SIZE);
    if (random_offset) {
    kernel_length = (unsigned long)(_end) - (unsigned long)(_text);
// Copy the kernel to it's new location
    memcpy(location_new, _text, kernel_length);
// Sync the caches ready for execution of new kernel
    __asm__ __volatile__ (
    "ibar 0 \t\n"
    "dbar 0 \t\n"
    ::: "memory");
    reloc_offset += random_offset;
// The current thread is now within the relocated kernel
    current_thread_pointer = RELOCATED_KASLR(current_thread_pointer);
    update_reloc_offset(&reloc_offset, random_offset);
    }
    if (reloc_offset)
    relocate_relative();
    relocate_absolute(random_offset);
    return random_offset;
    }
//
// Show relocation information on panic.
//
#[no_mangle]
unsafe extern "C" fn show_kernel_relocation(level: *const c_char) {
    static void show_kernel_relocation(const char *level)
    {
    if (reloc_offset > 0) {
    printk(level);
    pr_cont("Kernel relocated by 0x%lx\n", reloc_offset);
    pr_cont(" .text @ 0x%px\n", _text);
    pr_cont(" .data @ 0x%px\n", _sdata);
    pr_cont(" .bss  @ 0x%px\n", __bss_start);
    }
    }
    static int kernel_location_notifier_fn(struct notifier_block *self,
    unsigned long v, void *p)
    {
    show_kernel_relocation(KERN_EMERG);
    return NOTIFY_DONE;
    }
    static struct notifier_block kernel_location_notifier = {
    .notifier_call = kernel_location_notifier_fn
    };
#[no_mangle]
unsafe extern "C" fn register_kernel_offset_dumper() -> int __init {
    static int __init register_kernel_offset_dumper(void)
    {
    atomic_notifier_chain_register(&panic_notifier_list,
    &kernel_location_notifier);
    return 0;
    }
    arch_initcall(register_kernel_offset_dumper);
