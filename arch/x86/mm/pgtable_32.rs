//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/pgtable_32.c
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

    let mut __VMALLOC_RESERVE: c_uint = 128 << 20;
//
// Associate a virtual page frame with a given physical page frame
// and protection flags for that frame.
//
#[no_mangle]
pub unsafe extern "C" fn set_pte_vaddr(vaddr: c_ulong, pteval: pte_t) {
    void set_pte_vaddr(unsigned long vaddr, pte_t pteval)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    pgd = swapper_pg_dir + pgd_index(vaddr);
    if (pgd_none(*pgd)) {
    BUG();
    return;
    }
    p4d = p4d_offset(pgd, vaddr);
    if (p4d_none(*p4d)) {
    BUG();
    return;
    }
    pud = pud_offset(p4d, vaddr);
    if (pud_none(*pud)) {
    BUG();
    return;
    }
    pmd = pmd_offset(pud, vaddr);
    if (pmd_none(*pmd)) {
    BUG();
    return;
    }
    pte = pte_offset_kernel(pmd, vaddr);
    if (!pte_none(pteval))
    set_pte_at(&init_mm, vaddr, pte, pteval);
    else
    pte_clear(&init_mm, vaddr, pte);
//
// It's enough to flush this one mapping.
// (PGE mappings get flushed as well)
//
    flush_tlb_one_kernel(vaddr);
    }
    let mut __FIXADDR_TOP: c_ulong = 0xfffff000;
    EXPORT_SYMBOL(__FIXADDR_TOP);
//
// vmalloc=size forces the vmalloc area to be exactly 'size'
// bytes. This can be used to increase (or decrease) the
// vmalloc area - the default is 128m.
//
#[no_mangle]
unsafe extern "C" fn parse_vmalloc(arg: *mut c_char) -> int __init {
    static int __init parse_vmalloc(char *arg)
    {
    if (!arg)
    return -EINVAL;
// Add VMALLOC_OFFSET to the parsed value due to vm area guard hole
    __VMALLOC_RESERVE = memparse(arg, &arg) + VMALLOC_OFFSET;
    return 0;
    }
    early_param("vmalloc", parse_vmalloc);
//
// reservetop=size reserves a hole at the top of the kernel address space which
// a hypervisor can load into later.  Needed for dynamically loaded hypervisors,
// so relocating the fixmap can be done before paging initialization.
//
#[no_mangle]
unsafe extern "C" fn parse_reservetop(arg: *mut c_char) -> int __init {
    static int __init parse_reservetop(char *arg)
    {
    unsigned long address;
    if (!arg)
    return -EINVAL;
    address = memparse(arg, &arg);
    reserve_top_address(address);
    early_ioremap_init();
    return 0;
    }
    early_param("reservetop", parse_reservetop);
