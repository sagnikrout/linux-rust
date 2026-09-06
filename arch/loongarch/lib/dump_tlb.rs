//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/lib/dump_tlb.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 1994, 1995 by Waldorf Electronics, written by Ralf Baechle.
// Copyright (C) 1999 by Silicon Graphics, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn dump_tlb_regs() {
    void dump_tlb_regs(void)
    {
    let mut field: c_int = 2 * sizeof(unsigned long);
    pr_info("Index    : 0x%0x\n", read_csr_tlbidx());
    pr_info("PageSize : 0x%0x\n", read_csr_pagesize());
    pr_info("EntryHi  : 0x%0*lx\n", field, (unsigned long)read_csr_entryhi());
    pr_info("EntryLo0 : 0x%0*lx\n", field, (unsigned long)read_csr_entrylo0());
    pr_info("EntryLo1 : 0x%0*lx\n", field, (unsigned long)read_csr_entrylo1());
    }
#[no_mangle]
unsafe extern "C" fn dump_tlb(first: c_int, last: c_int) {
    static void dump_tlb(int first, int last)
    {
    unsigned long s_entryhi, entryhi, asid;
    unsigned long long entrylo0, entrylo1, pa;
    unsigned int index;
    unsigned int s_index, s_asid;
    unsigned int pagesize, c0, c1, i;
    let mut asidmask: c_ulong = cpu_asid_mask(&current_cpu_data);
    let mut pwidth: c_int = 16;
    let mut vwidth: c_int = 16;
    let mut asidwidth: c_int = DIV_ROUND_UP(ilog2(asidmask) + 1, 4);
    s_entryhi = read_csr_entryhi();
    s_index = read_csr_tlbidx();
    s_asid = read_csr_asid();
    for (i = first; i <= last; i++) {
    write_csr_index(i);
    tlb_read();
    pagesize = read_csr_pagesize();
    entryhi	 = read_csr_entryhi();
    entrylo0 = read_csr_entrylo0();
    entrylo1 = read_csr_entrylo1();
    index = read_csr_tlbidx();
    asid = read_csr_asid();
// EHINV bit marks entire entry as invalid
    if (index & CSR_TLBIDX_EHINV)
    continue;
//
// ASID takes effect in absence of G (global) bit.
//
    if (!((entrylo0 | entrylo1) & ENTRYLO_G) &&
    asid != s_asid)
    continue;
//
// Only print entries in use
//
    pr_info("Index: %4d pgsize=0x%x ", i, (1 << pagesize));
    c0 = (entrylo0 & ENTRYLO_C) >> ENTRYLO_C_SHIFT;
    c1 = (entrylo1 & ENTRYLO_C) >> ENTRYLO_C_SHIFT;
    pr_cont("va=0x%0*lx asid=0x%0*lx",
    vwidth, (entryhi & ~0x1fffUL), asidwidth, asid & asidmask);
// NR/NX are in awkward places, so mask them off separately

    pa = entrylo0 & ~(ENTRYLO_NR | ENTRYLO_NX);

    pa = pa & PAGE_MASK;
    pr_cont("\n\t[");

    pr_cont("nr=%d nx=%d ",
    (entrylo0 & ENTRYLO_NR) ? 1 : 0,
    (entrylo0 & ENTRYLO_NX) ? 1 : 0);

    pr_cont("pa=0x%0*llx c=%d d=%d v=%d g=%d plv=%lld] [",
    pwidth, pa, c0,
    (entrylo0 & ENTRYLO_D) ? 1 : 0,
    (entrylo0 & ENTRYLO_V) ? 1 : 0,
    (entrylo0 & ENTRYLO_G) ? 1 : 0,
    (entrylo0 & ENTRYLO_PLV) >> ENTRYLO_PLV_SHIFT);
// NR/NX are in awkward places, so mask them off separately

    pa = entrylo1 & ~(ENTRYLO_NR | ENTRYLO_NX);

    pa = pa & PAGE_MASK;

    pr_cont("nr=%d nx=%d ",
    (entrylo1 & ENTRYLO_NR) ? 1 : 0,
    (entrylo1 & ENTRYLO_NX) ? 1 : 0);

    pr_cont("pa=0x%0*llx c=%d d=%d v=%d g=%d plv=%lld]\n",
    pwidth, pa, c1,
    (entrylo1 & ENTRYLO_D) ? 1 : 0,
    (entrylo1 & ENTRYLO_V) ? 1 : 0,
    (entrylo1 & ENTRYLO_G) ? 1 : 0,
    (entrylo1 & ENTRYLO_PLV) >> ENTRYLO_PLV_SHIFT);
    }
    pr_info("\n");
    write_csr_entryhi(s_entryhi);
    write_csr_tlbidx(s_index);
    write_csr_asid(s_asid);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_tlb_all() {
    void dump_tlb_all(void)
    {
    dump_tlb(0, current_cpu_data.tlbsize - 1);
    }
