//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/mem.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
pub unsafe extern "C" fn kasan_init() -> void __init {
    void __init kasan_init(void)
    {
//
// kasan_map_memory will map all of the required address space and
// the host machine will allocate physical memory as necessary.
//
    kasan_map_memory((void *)KASAN_SHADOW_START, KASAN_SHADOW_SIZE);
    init_task.kasan_depth = 0;
//
// Since kasan_init() is called before main(),
// KASAN is initialized but the enablement is deferred after
// jump_label_init(). See arch_mm_preinit().
//
    }
#[no_mangle]
pub unsafe extern "C" fn void(_arg: *mut kasan_init_ptr)(void) -> static {
    static void (*kasan_init_ptr)(void)
    __section(".kasan_init") __used
    = kasan_init;

//
// Initialized during boot, and readonly for initializing page tables
// afterwards
//
    pgd_t swapper_pg_dir[PTRS_PER_PGD];
// Initialized at boot time, and readonly after that
    let mut kmalloc_ok: c_int = 0;
// Used during early boot
    static unsigned long brk_end;
#[no_mangle]
pub unsafe extern "C" fn arch_mm_preinit() -> void __init {
    void __init arch_mm_preinit(void)
    {
// Safe to call after jump_label_init(). Enables KASAN.
    kasan_init_generic();
// Map in the area just after the brk now that kmalloc is about
// to be turned on.
//
    brk_end = PAGE_ALIGN((unsigned long) sbrk(0));
    map_memory(brk_end, __pa(brk_end), uml_reserved - brk_end, 1, 1, 0);
    memblock_free((void *)brk_end, uml_reserved - brk_end);
    uml_reserved = brk_end;
    min_low_pfn = PFN_UP(__pa(uml_reserved));
    max_pfn = max_low_pfn;
    }
#[no_mangle]
pub unsafe extern "C" fn mem_init() -> void __init {
    void __init mem_init(void)
    {
    kmalloc_ok = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) -> void __init {
    void __init arch_zone_limits_init(unsigned long *max_zone_pfns)
    {
    max_zone_pfns[ZONE_NORMAL] = high_physmem >> PAGE_SHIFT;
    }
//
// This can't do anything because nothing in the kernel image can be freed
// since it's not in kernel physical memory.
//
#[no_mangle]
pub unsafe extern "C" fn free_initmem() {
    void free_initmem(void)
    {
    }
// Allocate and free page tables.
    pgd_t *pgd_alloc(struct mm_struct *mm)
    {
    pgd_t *pgd = __pgd_alloc(mm, 0);
    if (pgd)
    memcpy(pgd + USER_PTRS_PER_PGD,
    swapper_pg_dir + USER_PTRS_PER_PGD,
    (PTRS_PER_PGD - USER_PTRS_PER_PGD) * sizeof(pgd_t));
    return pgd;
    }
    void *uml_kmalloc(int size, int flags)
    {
    return kmalloc(size, flags);
    }
    static const pgprot_t protection_map[16] = {
    [VM_NONE]					= PAGE_NONE,
    [VM_READ]					= PAGE_READONLY,
    [VM_WRITE]					= PAGE_COPY,
    [VM_WRITE | VM_READ]				= PAGE_COPY,
    [VM_EXEC]					= PAGE_READONLY,
    [VM_EXEC | VM_READ]				= PAGE_READONLY,
    [VM_EXEC | VM_WRITE]				= PAGE_COPY,
    [VM_EXEC | VM_WRITE | VM_READ]			= PAGE_COPY,
    [VM_SHARED]					= PAGE_NONE,
    [VM_SHARED | VM_READ]				= PAGE_READONLY,
    [VM_SHARED | VM_WRITE]				= PAGE_SHARED,
    [VM_SHARED | VM_WRITE | VM_READ]		= PAGE_SHARED,
    [VM_SHARED | VM_EXEC]				= PAGE_READONLY,
    [VM_SHARED | VM_EXEC | VM_READ]			= PAGE_READONLY,
    [VM_SHARED | VM_EXEC | VM_WRITE]		= PAGE_SHARED,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ]	= PAGE_SHARED
    };
    DECLARE_VM_GET_PAGE_PROT
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    let mut rodata_start: c_ulong = PFN_ALIGN(__start_rodata);
    let mut rodata_end: c_ulong = PFN_ALIGN(__end_rodata);
    os_protect_memory((void *)rodata_start, rodata_end - rodata_start, 1, 0, 0);
    }
