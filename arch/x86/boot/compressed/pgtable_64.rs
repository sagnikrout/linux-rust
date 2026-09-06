//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/compressed/pgtable_64.c
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

pub const BIOS_START_MIN: c_uint = 0x20000U	/* 128K, less than this is insane */;
pub const BIOS_START_MAX: c_uint = 0x9f000U	/* 640K, absolute maximum */;
// __pgtable_l5_enabled needs to be in .data to avoid being cleared along with .bss
    unsigned int __section(".data") __pgtable_l5_enabled;
    unsigned int __section(".data") pgdir_shift = 39;
    unsigned int __section(".data") ptrs_per_p4d = 1;
// Buffer to preserve trampoline memory
    static char trampoline_save[TRAMPOLINE_32BIT_SIZE];
//
// Trampoline address will be printed by extract_kernel() for debugging
// purposes.
//
// Avoid putting the pointer into .bss as it will be cleared between
// configure_5level_paging() and extract_kernel().
//
    unsigned long *trampoline_32bit __section(".data");
    int cmdline_find_option_bool(const char *option);
#[no_mangle]
unsafe extern "C" fn find_trampoline_placement() -> c_ulong {
    static unsigned long find_trampoline_placement(void)
    {
    let mut bios_start: c_ulong = 0, ebda_start = 0;
    struct boot_e820_entry *entry;
    char *signature;
    int i;
//
// Find a suitable spot for the trampoline.
// This code is based on reserve_bios_regions().
//
// EFI systems may not provide legacy ROM. The memory may not be mapped
// at all.
//
// Only look for values in the legacy ROM for non-EFI system.
//
    signature = (char *)&boot_params_ptr.efi_info.efi_loader_signature;
    if (strncmp(signature, EFI32_LOADER_SIGNATURE, 4) &&
    strncmp(signature, EFI64_LOADER_SIGNATURE, 4)) {
    ebda_start = *(unsigned short *)0x40e << 4;
    bios_start = *(unsigned short *)0x413 << 10;
    }
    if (bios_start < BIOS_START_MIN || bios_start > BIOS_START_MAX)
    bios_start = BIOS_START_MAX;
    if (ebda_start > BIOS_START_MIN && ebda_start < bios_start)
    bios_start = ebda_start;
    bios_start = round_down(bios_start, PAGE_SIZE);
// Find the first usable memory region under bios_start.
    for (i = boot_params_ptr.e820_entries - 1; i >= 0; i--) {
    let mut new: c_ulong = bios_start;
    entry = &boot_params_ptr.e820_table[i];
// Skip all entries above bios_start.
    if (bios_start <= entry.addr)
    continue;
// Skip non-RAM entries.
    if (entry.type != E820_TYPE_RAM)
    continue;
// Adjust bios_start to the end of the entry if needed.
    if (bios_start > entry.addr + entry.size)
    new = entry.addr + entry.size;
// Keep bios_start page-aligned.
    new = round_down(new, PAGE_SIZE);
// Skip the entry if it's too small.
    if (new - TRAMPOLINE_32BIT_SIZE < entry.addr)
    continue;
// Protect against underflow.
    if (new - TRAMPOLINE_32BIT_SIZE > bios_start)
    break;
    bios_start = new;
    break;
    }
// Place the trampoline just below the end of low memory
    return bios_start - TRAMPOLINE_32BIT_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn configure_5level_paging(bp: *mut boot_params, pgtable: *mut c_void) -> asmlinkage void {
    asmlinkage void configure_5level_paging(struct boot_params *bp, void *pgtable)
    {
    void (*toggle_la57)(void *cr3);
    let mut l5_required: bool = false;
// Initialize boot_params. Required for cmdline_find_option_bool().
    sanitize_boot_params(bp);
    boot_params_ptr = bp;
//
// Check if LA57 is desired and supported.
//
// There are several parts to the check:
// - if user asked to disable 5-level paging: no5lvl in cmdline
// - if the machine supports 5-level paging:
// + CPUID leaf 7 is supported
// + the leaf has the feature bit set
//
    if (!cmdline_find_option_bool("no5lvl") &&
    native_cpuid_eax(0) >= 7 && (native_cpuid_ecx(7) & BIT(16))) {
    l5_required = true;
// Initialize variables for 5-level paging
    __pgtable_l5_enabled = 1;
    pgdir_shift = 48;
    ptrs_per_p4d = 512;
    }
//
// The trampoline will not be used if the paging mode is already set to
// the desired one.
//
    if (l5_required == !!(native_read_cr4() & X86_CR4_LA57))
    return;
    trampoline_32bit = (unsigned long *)find_trampoline_placement();
// Preserve trampoline memory
    memcpy(trampoline_save, trampoline_32bit, TRAMPOLINE_32BIT_SIZE);
// Clear trampoline memory first
    memset(trampoline_32bit, 0, TRAMPOLINE_32BIT_SIZE);
// Copy trampoline code in place
    toggle_la57 = memcpy(trampoline_32bit +
    TRAMPOLINE_32BIT_CODE_OFFSET / sizeof(unsigned long),
    &trampoline_32bit_src, TRAMPOLINE_32BIT_CODE_SIZE);
//
// Avoid the need for a stack in the 32-bit trampoline code, by using
// LJMP rather than LRET to return back to long mode. LJMP takes an
// immediate absolute address, which needs to be adjusted based on the
// placement of the trampoline.
//
// (u32 *)((u8 *)toggle_la57 + trampoline_ljmp_imm_offset) +=
    (unsigned long)toggle_la57;
//
// The code below prepares page table in trampoline memory.
//
// The new page table will be used by trampoline code for switching
// from 4- to 5-level paging or vice versa.
//
    if (l5_required) {
//
// For 4- to 5-level paging transition, set up current CR3 as
// the first and the only entry in a new top-level page table.
//
// trampoline_32bit = native_read_cr3_pa() | _PAGE_TABLE_NOENC;
    } else {
    u64 *new_cr3;
    pgd_t *pgdp;
//
// For 5- to 4-level paging transition, copy page table pointed
// by first entry in the current top-level page table as our
// new top-level page table.
//
// We cannot just point to the page table from trampoline as it
// may be above 4G.
//
    pgdp = (pgd_t *)native_read_cr3_pa();
    new_cr3 = (u64 *)(native_pgd_val(pgdp[0]) & PTE_PFN_MASK);
    memcpy(trampoline_32bit, new_cr3, PAGE_SIZE);
    }
    toggle_la57(trampoline_32bit);
//
// Move the top level page table out of trampoline memory.
//
    memcpy(pgtable, trampoline_32bit, PAGE_SIZE);
    native_write_cr3((unsigned long)pgtable);
// Restore trampoline memory
    memcpy(trampoline_32bit, trampoline_save, TRAMPOLINE_32BIT_SIZE);
    }
