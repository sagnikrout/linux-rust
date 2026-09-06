//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s64/radix_tlb.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TLB flush routines for radix kernels.
//
// Copyright 2015-2016, Aneesh Kumar K.V, IBM Corporation.
//

//
// tlbiel instruction for radix, set invalidation
// i.e., r=1 and is=01 or is=10 or is=11
//
    static __always_inline void tlbiel_radix_set_isa300(unsigned int set, unsigned int is,
    unsigned int pid,
    unsigned int ric, unsigned int prs)
    {
    unsigned long rb;
    unsigned long rs;
    rb = (set << PPC_BITLSHIFT(51)) | (is << PPC_BITLSHIFT(53));
    rs = ((unsigned long)pid << PPC_BITLSHIFT(31));
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIEL(%0, _arg: %1, _arg: %2, _arg: %3, _arg: 1) -> asm {
    asm volatile(PPC_TLBIEL(%0, %1, %2, %3, 1)
    : : "r"(rb), "r"(rs), "i"(ric), "i"(prs)
    : "memory");
    }
#[no_mangle]
unsafe extern "C" fn tlbiel_all_isa300(num_sets: c_uint, is: c_uint) {
    static void tlbiel_all_isa300(unsigned int num_sets, unsigned int is)
    {
    unsigned int set;
    asm volatile("ptesync": : :"memory");
//
// Flush the first set of the TLB, and the entire Page Walk Cache
// and partition table entries. Then flush the remaining sets of the
// TLB.
//
    if (early_cpu_has_feature(CPU_FTR_HVMODE)) {
// MSR[HV] should flush partition scope translations first.
    tlbiel_radix_set_isa300(0, is, 0, RIC_FLUSH_ALL, 0);
    if (!early_cpu_has_feature(CPU_FTR_ARCH_31)) {
    for (set = 1; set < num_sets; set++)
    tlbiel_radix_set_isa300(set, is, 0,
    RIC_FLUSH_TLB, 0);
    }
    }
// Flush process scoped entries.
    tlbiel_radix_set_isa300(0, is, 0, RIC_FLUSH_ALL, 1);
    if (!early_cpu_has_feature(CPU_FTR_ARCH_31)) {
    for (set = 1; set < num_sets; set++)
    tlbiel_radix_set_isa300(set, is, 0, RIC_FLUSH_TLB, 1);
    }
    ppc_after_tlbiel_barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn radix__tlbiel_all(action: c_uint) {
    void radix__tlbiel_all(unsigned int action)
    {
    unsigned int is;
    switch (action) {
    case TLB_INVAL_SCOPE_GLOBAL:
    is = 3;
    break;
    case TLB_INVAL_SCOPE_LPID:
    is = 2;
    break;
    default:
    BUG();
    }
    if (early_cpu_has_feature(CPU_FTR_ARCH_300))
    tlbiel_all_isa300(POWER9_TLB_SETS_RADIX, is);
    else
    WARN(1, "%s called on pre-POWER9 CPU\n", __func__);
    asm volatile(PPC_ISA_3_0_INVALIDATE_ERAT "; isync" : : :"memory");
    }
    static __always_inline void __tlbiel_pid(unsigned long pid, int set,
    unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = PPC_BIT(53); /* IS = 1 */
    rb |= set << PPC_BITLSHIFT(51);
    rs = ((unsigned long)pid) << PPC_BITLSHIFT(31);
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIEL(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIEL(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 1, rb, rs, ric, prs, r);
    }
#[no_mangle]
unsafe extern "C" fn __tlbie_pid(pid: c_ulong, ric: c_ulong) -> __always_inline void {
    static __always_inline void __tlbie_pid(unsigned long pid, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = PPC_BIT(53); /* IS = 1 */
    rs = pid << PPC_BITLSHIFT(31);
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 0, rb, rs, ric, prs, r);
    }
#[no_mangle]
unsafe extern "C" fn __tlbie_lpid(lpid: c_ulong, ric: c_ulong) -> __always_inline void {
    static __always_inline void __tlbie_lpid(unsigned long lpid, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = PPC_BIT(52); /* IS = 2 */
    rs = lpid;
    prs = 0; /* partition scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(lpid, 0, rb, rs, ric, prs, r);
    }
#[no_mangle]
unsafe extern "C" fn __tlbie_lpid_guest(lpid: c_ulong, ric: c_ulong) -> __always_inline void {
    static __always_inline void __tlbie_lpid_guest(unsigned long lpid, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = PPC_BIT(52); /* IS = 2 */
    rs = lpid;
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(lpid, 0, rb, rs, ric, prs, r);
    }
    static __always_inline void __tlbiel_va(unsigned long va, unsigned long pid,
    unsigned long ap, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = va & ~(PPC_BITMASK(52, 63));
    rb |= ap << PPC_BITLSHIFT(58);
    rs = pid << PPC_BITLSHIFT(31);
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIEL(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIEL(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 1, rb, rs, ric, prs, r);
    }
    static __always_inline void __tlbie_va(unsigned long va, unsigned long pid,
    unsigned long ap, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = va & ~(PPC_BITMASK(52, 63));
    rb |= ap << PPC_BITLSHIFT(58);
    rs = pid << PPC_BITLSHIFT(31);
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 0, rb, rs, ric, prs, r);
    }
    static __always_inline void __tlbie_va_lpid(unsigned long va, unsigned long lpid,
    unsigned long ap, unsigned long ric)
    {
    unsigned long rb,rs,prs,r;
    rb = va & ~(PPC_BITMASK(52, 63));
    rb |= ap << PPC_BITLSHIFT(58);
    rs = lpid;
    prs = 0; /* partition scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(lpid, 0, rb, rs, ric, prs, r);
    }
    static inline void fixup_tlbie_va(unsigned long va, unsigned long pid,
    unsigned long ap)
    {
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va(va, 0, ap, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va(va, pid, ap, RIC_FLUSH_TLB);
    }
    }
    static inline void fixup_tlbie_va_range(unsigned long va, unsigned long pid,
    unsigned long ap)
    {
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_pid(0, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va(va, pid, ap, RIC_FLUSH_TLB);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_tlbie_pid(pid: c_ulong) {
    static inline void fixup_tlbie_pid(unsigned long pid)
    {
//
// We can use any address for the invalidation, pick one which is
// probably unused as an optimisation.
//
    let mut va: c_ulong = ((1UL << 52) - 1);
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_pid(0, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va(va, pid, mmu_get_ap(MMU_PAGE_64K), RIC_FLUSH_TLB);
    }
    }
    static inline void fixup_tlbie_va_lpid(unsigned long va, unsigned long lpid,
    unsigned long ap)
    {
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va_lpid(va, 0, ap, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va_lpid(va, lpid, ap, RIC_FLUSH_TLB);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_tlbie_lpid(lpid: c_ulong) {
    static inline void fixup_tlbie_lpid(unsigned long lpid)
    {
//
// We can use any address for the invalidation, pick one which is
// probably unused as an optimisation.
//
    let mut va: c_ulong = ((1UL << 52) - 1);
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_lpid(0, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync": : :"memory");
    __tlbie_va_lpid(va, lpid, mmu_get_ap(MMU_PAGE_64K), RIC_FLUSH_TLB);
    }
    }
//
// We use 128 set in radix mode and 256 set in hpt mode.
//
#[no_mangle]
pub unsafe extern "C" fn _tlbiel_pid(pid: c_ulong, ric: c_ulong) {
    static inline void _tlbiel_pid(unsigned long pid, unsigned long ric)
    {
    int set;
    asm volatile("ptesync": : :"memory");
    switch (ric) {
    case RIC_FLUSH_PWC:
// For PWC, only one flush is needed
    __tlbiel_pid(pid, 0, RIC_FLUSH_PWC);
    ppc_after_tlbiel_barrier();
    return;
    case RIC_FLUSH_TLB:
    __tlbiel_pid(pid, 0, RIC_FLUSH_TLB);
    break;
    case RIC_FLUSH_ALL:
    default:
//
// Flush the first set of the TLB, and if
// we're doing a RIC_FLUSH_ALL, also flush
// the entire Page Walk Cache.
//
    __tlbiel_pid(pid, 0, RIC_FLUSH_ALL);
    }
    if (!cpu_has_feature(CPU_FTR_ARCH_31)) {
// For the remaining sets, just flush the TLB
    for (set = 1; set < POWER9_TLB_SETS_RADIX ; set++)
    __tlbiel_pid(pid, set, RIC_FLUSH_TLB);
    }
    ppc_after_tlbiel_barrier();
    asm volatile(PPC_RADIX_INVALIDATE_ERAT_USER "; isync" : : :"memory");
    }
#[no_mangle]
pub unsafe extern "C" fn _tlbie_pid(pid: c_ulong, ric: c_ulong) {
    static inline void _tlbie_pid(unsigned long pid, unsigned long ric)
    {
    asm volatile("ptesync": : :"memory");
//
// Workaround the fact that the "ric" argument to __tlbie_pid
// must be a compile-time constraint to match the "i" constraint
// in the asm statement.
//
    switch (ric) {
    case RIC_FLUSH_TLB:
    __tlbie_pid(pid, RIC_FLUSH_TLB);
    fixup_tlbie_pid(pid);
    break;
    case RIC_FLUSH_PWC:
    __tlbie_pid(pid, RIC_FLUSH_PWC);
    break;
    case RIC_FLUSH_ALL:
    default:
    __tlbie_pid(pid, RIC_FLUSH_ALL);
    fixup_tlbie_pid(pid);
    }
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbiel_pid {
    pub pid: c_ulong,
    pub ric: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn do_tlbiel_pid(info: *mut c_void) {
    static void do_tlbiel_pid(void *info)
    {
    struct tlbiel_pid *t = info;
    if (t.ric == RIC_FLUSH_TLB)
    _tlbiel_pid(t.pid, RIC_FLUSH_TLB);
#[no_mangle]
pub unsafe extern "C" fn if(RIC_FLUSH_PWC: t->ric ==) -> else {
    else if (t.ric == RIC_FLUSH_PWC)
    _tlbiel_pid(t.pid, RIC_FLUSH_PWC);
    else
    _tlbiel_pid(t.pid, RIC_FLUSH_ALL);
    }
    static inline void _tlbiel_pid_multicast(struct mm_struct *mm,
    unsigned long pid, unsigned long ric)
    {
    struct cpumask *cpus = mm_cpumask(mm);
    let mut t: tlbiel_pid = { .pid = pid, .ric = ric };
    on_each_cpu_mask(cpus, do_tlbiel_pid, &t, 1);
//
// Always want the CPU translations to be invalidated with tlbiel in
// these paths, so while coprocessors must use tlbie, we can not
// optimise away the tlbiel component.
//
    if (atomic_read(&mm.context.copros) > 0)
    _tlbie_pid(pid, RIC_FLUSH_ALL);
    }
#[no_mangle]
pub unsafe extern "C" fn _tlbie_lpid(lpid: c_ulong, ric: c_ulong) {
    static inline void _tlbie_lpid(unsigned long lpid, unsigned long ric)
    {
    asm volatile("ptesync": : :"memory");
//
// Workaround the fact that the "ric" argument to __tlbie_pid
// must be a compile-time contraint to match the "i" constraint
// in the asm statement.
//
    switch (ric) {
    case RIC_FLUSH_TLB:
    __tlbie_lpid(lpid, RIC_FLUSH_TLB);
    fixup_tlbie_lpid(lpid);
    break;
    case RIC_FLUSH_PWC:
    __tlbie_lpid(lpid, RIC_FLUSH_PWC);
    break;
    case RIC_FLUSH_ALL:
    default:
    __tlbie_lpid(lpid, RIC_FLUSH_ALL);
    fixup_tlbie_lpid(lpid);
    }
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
#[no_mangle]
unsafe extern "C" fn _tlbie_lpid_guest(lpid: c_ulong, ric: c_ulong) -> __always_inline void {
    static __always_inline void _tlbie_lpid_guest(unsigned long lpid, unsigned long ric)
    {
//
// Workaround the fact that the "ric" argument to __tlbie_pid
// must be a compile-time contraint to match the "i" constraint
// in the asm statement.
//
    switch (ric) {
    case RIC_FLUSH_TLB:
    __tlbie_lpid_guest(lpid, RIC_FLUSH_TLB);
    break;
    case RIC_FLUSH_PWC:
    __tlbie_lpid_guest(lpid, RIC_FLUSH_PWC);
    break;
    case RIC_FLUSH_ALL:
    default:
    __tlbie_lpid_guest(lpid, RIC_FLUSH_ALL);
    }
    fixup_tlbie_lpid(lpid);
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
    static inline void __tlbiel_va_range(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long page_size,
    unsigned long psize)
    {
    unsigned long addr;
    let mut ap: c_ulong = mmu_get_ap(psize);
    for (addr = start; addr < end; addr += page_size)
    __tlbiel_va(addr, pid, ap, RIC_FLUSH_TLB);
    }
    static __always_inline void _tlbiel_va(unsigned long va, unsigned long pid,
    unsigned long psize, unsigned long ric)
    {
    let mut ap: c_ulong = mmu_get_ap(psize);
    asm volatile("ptesync": : :"memory");
    __tlbiel_va(va, pid, ap, ric);
    ppc_after_tlbiel_barrier();
    }
    static inline void _tlbiel_va_range(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long page_size,
    unsigned long psize, bool also_pwc)
    {
    asm volatile("ptesync": : :"memory");
    if (also_pwc)
    __tlbiel_pid(pid, 0, RIC_FLUSH_PWC);
    __tlbiel_va_range(start, end, pid, page_size, psize);
    ppc_after_tlbiel_barrier();
    }
    static inline void __tlbie_va_range(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long page_size,
    unsigned long psize)
    {
    unsigned long addr;
    let mut ap: c_ulong = mmu_get_ap(psize);
    for (addr = start; addr < end; addr += page_size)
    __tlbie_va(addr, pid, ap, RIC_FLUSH_TLB);
    fixup_tlbie_va_range(addr - page_size, pid, ap);
    }
    static __always_inline void _tlbie_va(unsigned long va, unsigned long pid,
    unsigned long psize, unsigned long ric)
    {
    let mut ap: c_ulong = mmu_get_ap(psize);
    asm volatile("ptesync": : :"memory");
    __tlbie_va(va, pid, ap, ric);
    fixup_tlbie_va(va, pid, ap);
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbiel_va {
    pub pid: c_ulong,
    pub va: c_ulong,
    pub psize: c_ulong,
    pub ric: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn do_tlbiel_va(info: *mut c_void) {
    static void do_tlbiel_va(void *info)
    {
    struct tlbiel_va *t = info;
    if (t.ric == RIC_FLUSH_TLB)
    _tlbiel_va(t.va, t.pid, t.psize, RIC_FLUSH_TLB);
#[no_mangle]
pub unsafe extern "C" fn if(RIC_FLUSH_PWC: t->ric ==) -> else {
    else if (t.ric == RIC_FLUSH_PWC)
    _tlbiel_va(t.va, t.pid, t.psize, RIC_FLUSH_PWC);
    else
    _tlbiel_va(t.va, t.pid, t.psize, RIC_FLUSH_ALL);
    }
    static inline void _tlbiel_va_multicast(struct mm_struct *mm,
    unsigned long va, unsigned long pid,
    unsigned long psize, unsigned long ric)
    {
    struct cpumask *cpus = mm_cpumask(mm);
    let mut t: tlbiel_va = { .va = va, .pid = pid, .psize = psize, .ric = ric };
    on_each_cpu_mask(cpus, do_tlbiel_va, &t, 1);
    if (atomic_read(&mm.context.copros) > 0)
    _tlbie_va(va, pid, psize, RIC_FLUSH_TLB);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbiel_va_range {
    pub pid: c_ulong,
    pub start: c_ulong,
    pub end: c_ulong,
    pub page_size: c_ulong,
    pub psize: c_ulong,
    pub also_pwc: bool,
}

#[no_mangle]
unsafe extern "C" fn do_tlbiel_va_range(info: *mut c_void) {
    static void do_tlbiel_va_range(void *info)
    {
    struct tlbiel_va_range *t = info;
    _tlbiel_va_range(t.start, t.end, t.pid, t.page_size,
    t.psize, t.also_pwc);
    }
    static __always_inline void _tlbie_va_lpid(unsigned long va, unsigned long lpid,
    unsigned long psize, unsigned long ric)
    {
    let mut ap: c_ulong = mmu_get_ap(psize);
    asm volatile("ptesync": : :"memory");
    __tlbie_va_lpid(va, lpid, ap, ric);
    fixup_tlbie_va_lpid(va, lpid, ap);
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
    static inline void _tlbie_va_range(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long page_size,
    unsigned long psize, bool also_pwc)
    {
    asm volatile("ptesync": : :"memory");
    if (also_pwc)
    __tlbie_pid(pid, RIC_FLUSH_PWC);
    __tlbie_va_range(start, end, pid, page_size, psize);
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }
    static inline void _tlbiel_va_range_multicast(struct mm_struct *mm,
    unsigned long start, unsigned long end,
    unsigned long pid, unsigned long page_size,
    unsigned long psize, bool also_pwc)
    {
    struct cpumask *cpus = mm_cpumask(mm);
    struct tlbiel_va_range t = { .start = start, .end = end,
    .pid = pid, .page_size = page_size,
    .psize = psize, .also_pwc = also_pwc };
    on_each_cpu_mask(cpus, do_tlbiel_va_range, &t, 1);
    if (atomic_read(&mm.context.copros) > 0)
    _tlbie_va_range(start, end, pid, page_size, psize, also_pwc);
    }
//
// Base TLB flushing operations:
//
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes kernel pages
//
// - local_* variants of page and mm only apply to the current
// processor
//
#[no_mangle]
pub unsafe extern "C" fn radix__local_flush_tlb_mm(mm: *mut mm_struct) {
    void radix__local_flush_tlb_mm(struct mm_struct *mm)
    {
    let mut pid: c_ulong = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
    _tlbiel_pid(pid, RIC_FLUSH_TLB);
    preempt_enable();
    }
    EXPORT_SYMBOL(radix__local_flush_tlb_mm);

#[no_mangle]
pub unsafe extern "C" fn radix__local_flush_all_mm(mm: *mut mm_struct) {
    void radix__local_flush_all_mm(struct mm_struct *mm)
    {
    let mut pid: c_ulong = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
    _tlbiel_pid(pid, RIC_FLUSH_ALL);
    preempt_enable();
    }
    EXPORT_SYMBOL(radix__local_flush_all_mm);
#[no_mangle]
unsafe extern "C" fn __flush_all_mm(mm: *mut mm_struct, fullmm: bool) {
    static void __flush_all_mm(struct mm_struct *mm, bool fullmm)
    {
    radix__local_flush_all_mm(mm);
    }

    void radix__local_flush_tlb_page_psize(struct mm_struct *mm, unsigned long vmaddr,
    int psize)
    {
    let mut pid: c_ulong = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
    _tlbiel_va(vmaddr, pid, psize, RIC_FLUSH_TLB);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn radix__local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void radix__local_flush_tlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {

// need the return fix for nohash.c
    if (is_vm_hugetlb_page(vma))
    return radix__local_flush_hugetlb_page(vma, vmaddr);

    radix__local_flush_tlb_page_psize(vma.vm_mm, vmaddr, mmu_virtual_psize);
    }
    EXPORT_SYMBOL(radix__local_flush_tlb_page);
#[no_mangle]
unsafe extern "C" fn mm_needs_flush_escalation(mm: *mut mm_struct) -> bool {
    static bool mm_needs_flush_escalation(struct mm_struct *mm)
    {
//
// The P9 nest MMU has issues with the page walk cache caching PTEs
// and not flushing them when RIC = 0 for a PID/LPID invalidate.
//
// This may have been fixed in shipping firmware (by disabling PWC
// or preventing it from caching PTEs), but until that is confirmed,
// this workaround is required - escalate all RIC=0 IS=1/2/3 flushes
// to RIC=2.
//
// POWER10 (and P9P) does not have this problem.
//
    if (cpu_has_feature(CPU_FTR_ARCH_31))
    return false;
    if (atomic_read(&mm.context.copros) > 0)
    return true;
    return false;
    }
//
// If always_flush is true, then flush even if this CPU can't be removed
// from mm_cpumask.
//
#[no_mangle]
unsafe extern "C" fn exit_lazy_flush_tlb(mm: *mut mm_struct) {
    static void exit_lazy_flush_tlb(struct mm_struct *mm)
    {
    let mut pid: c_ulong = mm.context.id;
    let mut cpu: c_int = smp_processor_id();
//
// A kthread could have done a mmget_not_zero() after the flushing CPU
// checked mm_cpumask, and be in the process of kthread_use_mm when
// interrupted here. In that case, current->mm will be set to mm,
// because kthread_use_mm() setting ->mm and switching to the mm is
// done with interrupts off.
//
    if (current.mm == mm)
    goto out;
    if (current.active_mm == mm) {
    unsigned long flags;
    WARN_ON_ONCE(current.mm != core::ptr::null_mut());
//
// It is a kernel thread and is using mm as the lazy tlb, so
// switch it to init_mm. This is not always called from IPI
// (e.g., flush_type_needed), so must disable irqs.
//
    local_irq_save(flags);
    mmgrab_lazy_tlb(&init_mm);
    current.active_mm = &init_mm;
    switch_mm_irqs_off(mm, &init_mm, current);
    mmdrop_lazy_tlb(mm);
    local_irq_restore(flags);
    }
//
// This IPI may be initiated from any source including those not
// running the mm, so there may be a racing IPI that comes after
// this one which finds the cpumask already clear. Check and avoid
// underflowing the active_cpus count in that case. The race should
// not otherwise be a problem, but the TLB must be flushed because
// that's what the caller expects.
//
    if (cpumask_test_cpu(cpu, mm_cpumask(mm))) {
    dec_mm_active_cpus(mm);
    cpumask_clear_cpu(cpu, mm_cpumask(mm));
    }
    out:
    _tlbiel_pid(pid, RIC_FLUSH_ALL);
    }

#[no_mangle]
unsafe extern "C" fn do_exit_flush_lazy_tlb(arg: *mut c_void) {
    static void do_exit_flush_lazy_tlb(void *arg)
    {
    struct mm_struct *mm = arg;
    exit_lazy_flush_tlb(mm);
    }
#[no_mangle]
unsafe extern "C" fn exit_flush_lazy_tlbs(mm: *mut mm_struct) {
    static void exit_flush_lazy_tlbs(struct mm_struct *mm)
    {
//
// Would be nice if this was async so it could be run in
// parallel with our local flush, but generic code does not
// give a good API for it. Could extend the generic code or
// make a special powerpc IPI for flushing TLBs.
// For now it's not too performance critical.
//
    smp_call_function_many(mm_cpumask(mm), do_exit_flush_lazy_tlb,
    (void *)mm, 1);
    }

    static inline void exit_flush_lazy_tlbs(struct mm_struct *mm) { }

    static DEFINE_PER_CPU(unsigned int, mm_cpumask_trim_clock);
//
// Interval between flushes at which we send out IPIs to check whether the
// mm_cpumask can be trimmed for the case where it's not a single-threaded
// process flushing its own mm. The intent is to reduce the cost of later
// flushes. Don't want this to be so low that it adds noticable cost to TLB
// flushing, or so high that it doesn't help reduce global TLBIEs.
//
    let mut tlb_mm_cpumask_trim_timer: static unsigned long = 1073;
#[no_mangle]
unsafe extern "C" fn tick_and_test_trim_clock() -> bool {
    static bool tick_and_test_trim_clock(void)
    {
    if (__this_cpu_inc_return(mm_cpumask_trim_clock) ==
    tlb_mm_cpumask_trim_timer) {
    __this_cpu_write(mm_cpumask_trim_clock, 0);
    return true;
    }
    return false;
    }
    enum tlb_flush_type {
    FLUSH_TYPE_NONE,
    FLUSH_TYPE_LOCAL,
    FLUSH_TYPE_GLOBAL,
    };
#[no_mangle]
unsafe extern "C" fn flush_type_needed(mm: *mut mm_struct, fullmm: bool) -> enum tlb_flush_type {
    static enum tlb_flush_type flush_type_needed(struct mm_struct *mm, bool fullmm)
    {
    let mut active_cpus: c_int = atomic_read(&mm.context.active_cpus);
    let mut cpu: c_int = smp_processor_id();
    if (active_cpus == 0)
    return FLUSH_TYPE_NONE;
    if (active_cpus == 1 && cpumask_test_cpu(cpu, mm_cpumask(mm))) {
    if (current.mm != mm) {
//
// Asynchronous flush sources may trim down to nothing
// if the process is not running, so occasionally try
// to trim.
//
    if (tick_and_test_trim_clock()) {
    exit_lazy_flush_tlb(mm);
    return FLUSH_TYPE_NONE;
    }
    }
    return FLUSH_TYPE_LOCAL;
    }
// Coprocessors require TLBIE to invalidate nMMU.
    if (atomic_read(&mm.context.copros) > 0)
    return FLUSH_TYPE_GLOBAL;
//
// In the fullmm case there's no point doing the exit_flush_lazy_tlbs
// because the mm is being taken down anyway, and a TLBIE tends to
// be faster than an IPI+TLBIEL.
//
    if (fullmm)
    return FLUSH_TYPE_GLOBAL;
//
// If we are running the only thread of a single-threaded process,
// then we should almost always be able to trim off the rest of the
// CPU mask (except in the case of use_mm() races), so always try
// trimming the mask.
//
    if (atomic_read(&mm.mm_users) <= 1 && current.mm == mm) {
    exit_flush_lazy_tlbs(mm);
//
// use_mm() race could prevent IPIs from being able to clear
// the cpumask here, however those users are established
// after our first check (and so after the PTEs are removed),
// and the TLB still gets flushed by the IPI, so this CPU
// will only require a local flush.
//
    return FLUSH_TYPE_LOCAL;
    }
//
// Occasionally try to trim down the cpumask. It's possible this can
// bring the mask to zero, which results in no flush.
//
    if (tick_and_test_trim_clock()) {
    exit_flush_lazy_tlbs(mm);
    if (current.mm == mm)
    return FLUSH_TYPE_LOCAL;
    if (cpumask_test_cpu(cpu, mm_cpumask(mm)))
    exit_lazy_flush_tlb(mm);
    return FLUSH_TYPE_NONE;
    }
    return FLUSH_TYPE_GLOBAL;
    }

#[no_mangle]
pub unsafe extern "C" fn radix__flush_tlb_mm(mm: *mut mm_struct) {
    void radix__flush_tlb_mm(struct mm_struct *mm)
    {
    unsigned long pid;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
//
// Order loads of mm_cpumask (in flush_type_needed) vs previous
// stores to clear ptes before the invalidate. See barrier in
// switch_mm_irqs_off
//
    smp_mb();
    type = flush_type_needed(mm, false);
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_pid(pid, RIC_FLUSH_TLB);
    } else if (type == FLUSH_TYPE_GLOBAL) {
    if (!mmu_has_feature(MMU_FTR_GTSE)) {
    let mut tgt: c_ulong = H_RPTI_TARGET_CMMU;
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    pseries_rpt_invalidate(pid, tgt, H_RPTI_TYPE_TLB,
    H_RPTI_PAGE_ALL, 0, -1UL);
    } else if (cputlb_use_tlbie()) {
    if (mm_needs_flush_escalation(mm))
    _tlbie_pid(pid, RIC_FLUSH_ALL);
    else
    _tlbie_pid(pid, RIC_FLUSH_TLB);
    } else {
    _tlbiel_pid_multicast(mm, pid, RIC_FLUSH_TLB);
    }
    }
    preempt_enable();
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, 0, -1UL);
    }
    EXPORT_SYMBOL(radix__flush_tlb_mm);
#[no_mangle]
unsafe extern "C" fn __flush_all_mm(mm: *mut mm_struct, fullmm: bool) {
    static void __flush_all_mm(struct mm_struct *mm, bool fullmm)
    {
    unsigned long pid;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    type = flush_type_needed(mm, fullmm);
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_pid(pid, RIC_FLUSH_ALL);
    } else if (type == FLUSH_TYPE_GLOBAL) {
    if (!mmu_has_feature(MMU_FTR_GTSE)) {
    let mut tgt: c_ulong = H_RPTI_TARGET_CMMU;
    let mut type: c_ulong = H_RPTI_TYPE_ALL;
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    pseries_rpt_invalidate(pid, tgt, type,
    H_RPTI_PAGE_ALL, 0, -1UL);
    } else if (cputlb_use_tlbie())
    _tlbie_pid(pid, RIC_FLUSH_ALL);
    else
    _tlbiel_pid_multicast(mm, pid, RIC_FLUSH_ALL);
    }
    preempt_enable();
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, 0, -1UL);
    }
#[no_mangle]
pub unsafe extern "C" fn radix__flush_all_mm(mm: *mut mm_struct) {
    void radix__flush_all_mm(struct mm_struct *mm)
    {
    __flush_all_mm(mm, false);
    }
    EXPORT_SYMBOL(radix__flush_all_mm);
    void radix__flush_tlb_page_psize(struct mm_struct *mm, unsigned long vmaddr,
    int psize)
    {
    unsigned long pid;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    type = flush_type_needed(mm, false);
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_va(vmaddr, pid, psize, RIC_FLUSH_TLB);
    } else if (type == FLUSH_TYPE_GLOBAL) {
    if (!mmu_has_feature(MMU_FTR_GTSE)) {
    unsigned long tgt, pg_sizes, size;
    tgt = H_RPTI_TARGET_CMMU;
    pg_sizes = psize_to_rpti_pgsize(psize);
    size = 1UL << mmu_psize_to_shift(psize);
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    pseries_rpt_invalidate(pid, tgt, H_RPTI_TYPE_TLB,
    pg_sizes, vmaddr,
    vmaddr + size);
    } else if (cputlb_use_tlbie())
    _tlbie_va(vmaddr, pid, psize, RIC_FLUSH_TLB);
    else
    _tlbiel_va_multicast(mm, vmaddr, pid, psize, RIC_FLUSH_TLB);
    }
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn radix__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void radix__flush_tlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {

    if (is_vm_hugetlb_page(vma))
    return radix__flush_hugetlb_page(vma, vmaddr);

    radix__flush_tlb_page_psize(vma.vm_mm, vmaddr, mmu_virtual_psize);
    }
    EXPORT_SYMBOL(radix__flush_tlb_page);

#[no_mangle]
unsafe extern "C" fn do_tlbiel_kernel(info: *mut c_void) {
    static void do_tlbiel_kernel(void *info)
    {
    _tlbiel_pid(0, RIC_FLUSH_ALL);
    }
#[no_mangle]
pub unsafe extern "C" fn _tlbiel_kernel_broadcast() {
    static inline void _tlbiel_kernel_broadcast(void)
    {
    on_each_cpu(do_tlbiel_kernel, core::ptr::null_mut(), 1);
    if (tlbie_capable) {
//
// Coherent accelerators don't refcount kernel memory mappings,
// so have to always issue a tlbie for them. This is quite a
// slow path anyway.
//
    _tlbie_pid(0, RIC_FLUSH_ALL);
    }
    }
//
// If kernel TLBIs ever become local rather than global, then
// drivers/misc/ocxl/link.c:ocxl_link_add_pe will need some work, as it
// assumes kernel TLBIs are global.
//
#[no_mangle]
pub unsafe extern "C" fn radix__flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    void radix__flush_tlb_kernel_range(unsigned long start, unsigned long end)
    {
    if (!mmu_has_feature(MMU_FTR_GTSE)) {
    let mut tgt: c_ulong = H_RPTI_TARGET_CMMU | H_RPTI_TARGET_NMMU;
    let mut type: c_ulong = H_RPTI_TYPE_ALL;
    pseries_rpt_invalidate(0, tgt, type, H_RPTI_PAGE_ALL,
    start, end);
    } else if (cputlb_use_tlbie())
    _tlbie_pid(0, RIC_FLUSH_ALL);
    else
    _tlbiel_kernel_broadcast();
    }
    EXPORT_SYMBOL(radix__flush_tlb_kernel_range);
//
// Doesn't appear to be used anywhere. Remove.
//

//
// Number of pages above which we invalidate the entire PID rather than
// flush individual pages, for local and global flushes respectively.
//
// tlbie goes out to the interconnect and individual ops are more costly.
// It also does not iterate over sets like the local tlbiel variant when
// invalidating a full PID, so it has a far lower threshold to change from
// individual page flushes to full-pid flushes.
//
    let mut __read_mostly: static u32 tlb_single_page_flush_ceiling = 33;
    let mut __read_mostly: static u32 tlb_local_single_page_flush_ceiling = POWER9_TLB_SETS_RADIX * 2;
    static inline void __radix__flush_tlb_range(struct mm_struct *mm,
    unsigned long start, unsigned long end)
    {
    unsigned long pid;
    let mut page_shift: c_uint = mmu_psize_defs[mmu_virtual_psize].shift;
    let mut page_size: c_ulong = 1UL << page_shift;
    let mut nr_pages: c_ulong = (end - start) >> page_shift;
    bool flush_pid, flush_pwc = false;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    WARN_ON_ONCE(end == TLB_FLUSH_ALL);
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    type = flush_type_needed(mm, false);
    if (type == FLUSH_TYPE_NONE)
    goto out;
    if (type == FLUSH_TYPE_GLOBAL)
    flush_pid = nr_pages > tlb_single_page_flush_ceiling;
    else
    flush_pid = nr_pages > tlb_local_single_page_flush_ceiling;
//
// full pid flush already does the PWC flush. if it is not full pid
// flush check the range is more than PMD and force a pwc flush
// mremap() depends on this behaviour.
//
    if (!flush_pid && (end - start) >= PMD_SIZE)
    flush_pwc = true;
    if (!mmu_has_feature(MMU_FTR_GTSE) && type == FLUSH_TYPE_GLOBAL) {
    let mut type: c_ulong = H_RPTI_TYPE_TLB;
    let mut tgt: c_ulong = H_RPTI_TARGET_CMMU;
    let mut pg_sizes: c_ulong = psize_to_rpti_pgsize(mmu_virtual_psize);
    if (IS_ENABLED(CONFIG_TRANSPARENT_HUGEPAGE))
    pg_sizes |= psize_to_rpti_pgsize(MMU_PAGE_2M);
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    if (flush_pwc)
    type |= H_RPTI_TYPE_PWC;
    pseries_rpt_invalidate(pid, tgt, type, pg_sizes, start, end);
    } else if (flush_pid) {
//
// We are now flushing a range larger than PMD size force a RIC_FLUSH_ALL
//
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_pid(pid, RIC_FLUSH_ALL);
    } else {
    if (cputlb_use_tlbie()) {
    _tlbie_pid(pid, RIC_FLUSH_ALL);
    } else {
    _tlbiel_pid_multicast(mm, pid, RIC_FLUSH_ALL);
    }
    }
    } else {
    bool hflush;
    unsigned long hstart, hend;
    hstart = (start + PMD_SIZE - 1) & PMD_MASK;
    hend = end & PMD_MASK;
    hflush = IS_ENABLED(CONFIG_TRANSPARENT_HUGEPAGE) && hstart < hend;
    if (type == FLUSH_TYPE_LOCAL) {
    asm volatile("ptesync": : :"memory");
    if (flush_pwc)
// For PWC, only one flush is needed
    __tlbiel_pid(pid, 0, RIC_FLUSH_PWC);
    __tlbiel_va_range(start, end, pid, page_size, mmu_virtual_psize);
    if (hflush)
    __tlbiel_va_range(hstart, hend, pid,
    PMD_SIZE, MMU_PAGE_2M);
    ppc_after_tlbiel_barrier();
    } else if (cputlb_use_tlbie()) {
    asm volatile("ptesync": : :"memory");
    if (flush_pwc)
    __tlbie_pid(pid, RIC_FLUSH_PWC);
    __tlbie_va_range(start, end, pid, page_size, mmu_virtual_psize);
    if (hflush)
    __tlbie_va_range(hstart, hend, pid,
    PMD_SIZE, MMU_PAGE_2M);
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    } else {
    _tlbiel_va_range_multicast(mm,
    start, end, pid, page_size, mmu_virtual_psize, flush_pwc);
    if (hflush)
    _tlbiel_va_range_multicast(mm,
    hstart, hend, pid, PMD_SIZE, MMU_PAGE_2M, flush_pwc);
    }
    }
    out:
    preempt_enable();
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, end);
    }
    void radix__flush_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {

    if (is_vm_hugetlb_page(vma))
    return radix__flush_hugetlb_tlb_range(vma, start, end);

    __radix__flush_tlb_range(vma.vm_mm, start, end);
    }
    EXPORT_SYMBOL(radix__flush_tlb_range);
#[no_mangle]
unsafe extern "C" fn radix_get_mmu_psize(page_size: c_int) -> c_int {
    static int radix_get_mmu_psize(int page_size)
    {
    int psize;
    if (page_size == (1UL << mmu_psize_defs[mmu_virtual_psize].shift))
    psize = mmu_virtual_psize;
#[no_mangle]
pub unsafe extern "C" fn if(mmu_psize_defs[MMU_PAGE_2M].shift): page_size == (1UL <<) -> else {
    else if (page_size == (1UL << mmu_psize_defs[MMU_PAGE_2M].shift))
    psize = MMU_PAGE_2M;
#[no_mangle]
pub unsafe extern "C" fn if(mmu_psize_defs[MMU_PAGE_1G].shift): page_size == (1UL <<) -> else {
    else if (page_size == (1UL << mmu_psize_defs[MMU_PAGE_1G].shift))
    psize = MMU_PAGE_1G;
    else
    return -1;
    return psize;
    }
//
// Flush partition scoped LPID address translation for all CPUs.
//
    void radix__flush_tlb_lpid_page(unsigned int lpid,
    unsigned long addr,
    unsigned long page_size)
    {
    let mut psize: c_int = radix_get_mmu_psize(page_size);
    _tlbie_va_lpid(addr, lpid, psize, RIC_FLUSH_TLB);
    }
    EXPORT_SYMBOL_GPL(radix__flush_tlb_lpid_page);
//
// Flush partition scoped PWC from LPID for all CPUs.
//
#[no_mangle]
pub unsafe extern "C" fn radix__flush_pwc_lpid(lpid: c_uint) {
    void radix__flush_pwc_lpid(unsigned int lpid)
    {
    _tlbie_lpid(lpid, RIC_FLUSH_PWC);
    }
    EXPORT_SYMBOL_GPL(radix__flush_pwc_lpid);
//
// Flush partition scoped translations from LPID (=LPIDR)
//
#[no_mangle]
pub unsafe extern "C" fn radix__flush_all_lpid(lpid: c_uint) {
    void radix__flush_all_lpid(unsigned int lpid)
    {
    _tlbie_lpid(lpid, RIC_FLUSH_ALL);
    }
    EXPORT_SYMBOL_GPL(radix__flush_all_lpid);
//
// Flush process scoped translations from LPID (=LPIDR)
//
#[no_mangle]
pub unsafe extern "C" fn radix__flush_all_lpid_guest(lpid: c_uint) {
    void radix__flush_all_lpid_guest(unsigned int lpid)
    {
    _tlbie_lpid_guest(lpid, RIC_FLUSH_ALL);
    }
#[no_mangle]
pub unsafe extern "C" fn radix__tlb_flush(tlb: *mut mmu_gather) {
    void radix__tlb_flush(struct mmu_gather *tlb)
    {
    let mut psize: c_int = 0;
    struct mm_struct *mm = tlb.mm;
    let mut page_size: c_int = tlb.page_size;
    let mut start: c_ulong = tlb.start;
    let mut end: c_ulong = tlb.end;
//
// if page size is not something we understand, do a full mm flush
//
// A "fullmm" flush must always do a flush_all_mm (RIC=2) flush
// that flushes the process table entry cache upon process teardown.
// See the comment for radix in arch_exit_mmap().
//
    if (tlb.fullmm) {
    if (IS_ENABLED(CONFIG_MMU_LAZY_TLB_SHOOTDOWN)) {
//
// Shootdown based lazy tlb mm refcounting means we
// have to IPI everyone in the mm_cpumask anyway soon
// when the mm goes away, so might as well do it as
// part of the final flush now.
//
// If lazy shootdown was improved to reduce IPIs (e.g.,
// by batching), then it may end up being better to use
// tlbies here instead.
//
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    exit_flush_lazy_tlbs(mm);
    __flush_all_mm(mm, true);
    preempt_enable();
    } else {
    __flush_all_mm(mm, true);
    }
    } else if ( (psize = radix_get_mmu_psize(page_size)) == -1) {
    if (!tlb.freed_tables)
    radix__flush_tlb_mm(mm);
    else
    radix__flush_all_mm(mm);
    } else {
    if (!tlb.freed_tables)
    radix__flush_tlb_range_psize(mm, start, end, psize);
    else
    radix__flush_tlb_pwc_range_psize(mm, start, end, psize);
    }
    }
    static void __radix__flush_tlb_range_psize(struct mm_struct *mm,
    unsigned long start, unsigned long end,
    int psize, bool also_pwc)
    {
    unsigned long pid;
    let mut page_shift: c_uint = mmu_psize_defs[psize].shift;
    let mut page_size: c_ulong = 1UL << page_shift;
    let mut nr_pages: c_ulong = (end - start) >> page_shift;
    bool flush_pid;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
    WARN_ON_ONCE(end == TLB_FLUSH_ALL);
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    type = flush_type_needed(mm, false);
    if (type == FLUSH_TYPE_NONE)
    goto out;
    if (type == FLUSH_TYPE_GLOBAL)
    flush_pid = nr_pages > tlb_single_page_flush_ceiling;
    else
    flush_pid = nr_pages > tlb_local_single_page_flush_ceiling;
    if (!mmu_has_feature(MMU_FTR_GTSE) && type == FLUSH_TYPE_GLOBAL) {
    let mut tgt: c_ulong = H_RPTI_TARGET_CMMU;
    let mut type: c_ulong = H_RPTI_TYPE_TLB;
    let mut pg_sizes: c_ulong = psize_to_rpti_pgsize(psize);
    if (also_pwc)
    type |= H_RPTI_TYPE_PWC;
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    pseries_rpt_invalidate(pid, tgt, type, pg_sizes, start, end);
    } else if (flush_pid) {
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_pid(pid, also_pwc ? RIC_FLUSH_ALL : RIC_FLUSH_TLB);
    } else {
    if (cputlb_use_tlbie()) {
    if (mm_needs_flush_escalation(mm))
    also_pwc = true;
    _tlbie_pid(pid,
    also_pwc ?  RIC_FLUSH_ALL : RIC_FLUSH_TLB);
    } else {
    _tlbiel_pid_multicast(mm, pid,
    also_pwc ?  RIC_FLUSH_ALL : RIC_FLUSH_TLB);
    }
    }
    } else {
    if (type == FLUSH_TYPE_LOCAL)
    _tlbiel_va_range(start, end, pid, page_size, psize, also_pwc);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cputlb_use_tlbie()) -> else {
    else if (cputlb_use_tlbie())
    _tlbie_va_range(start, end, pid, page_size, psize, also_pwc);
    else
    _tlbiel_va_range_multicast(mm,
    start, end, pid, page_size, psize, also_pwc);
    }
    out:
    preempt_enable();
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, end);
    }
    void radix__flush_tlb_range_psize(struct mm_struct *mm, unsigned long start,
    unsigned long end, int psize)
    {
    return __radix__flush_tlb_range_psize(mm, start, end, psize, false);
    }
    void radix__flush_tlb_pwc_range_psize(struct mm_struct *mm, unsigned long start,
    unsigned long end, int psize)
    {
    __radix__flush_tlb_range_psize(mm, start, end, psize, true);
    }

#[no_mangle]
pub unsafe extern "C" fn radix__flush_tlb_collapsed_pmd(mm: *mut mm_struct, addr: c_ulong) {
    void radix__flush_tlb_collapsed_pmd(struct mm_struct *mm, unsigned long addr)
    {
    unsigned long pid, end;
    enum tlb_flush_type type;
    pid = mm.context.id;
    if (WARN_ON_ONCE(pid == MMU_NO_CONTEXT))
    return;
// 4k page size, just blow the world
    if (PAGE_SIZE == 0x1000) {
    radix__flush_all_mm(mm);
    return;
    }
    end = addr + HPAGE_PMD_SIZE;
// Otherwise first do the PWC, then iterate the pages.
    preempt_disable();
    smp_mb(); /* see radix__flush_tlb_mm */
    type = flush_type_needed(mm, false);
    if (type == FLUSH_TYPE_LOCAL) {
    _tlbiel_va_range(addr, end, pid, PAGE_SIZE, mmu_virtual_psize, true);
    } else if (type == FLUSH_TYPE_GLOBAL) {
    if (!mmu_has_feature(MMU_FTR_GTSE)) {
    unsigned long tgt, type, pg_sizes;
    tgt = H_RPTI_TARGET_CMMU;
    type = H_RPTI_TYPE_ALL;
    pg_sizes = psize_to_rpti_pgsize(mmu_virtual_psize);
    if (atomic_read(&mm.context.copros) > 0)
    tgt |= H_RPTI_TARGET_NMMU;
    pseries_rpt_invalidate(pid, tgt, type, pg_sizes,
    addr, end);
    } else if (cputlb_use_tlbie())
    _tlbie_va_range(addr, end, pid, PAGE_SIZE, mmu_virtual_psize, true);
    else
    _tlbiel_va_range_multicast(mm,
    addr, end, pid, PAGE_SIZE, mmu_virtual_psize, true);
    }
    preempt_enable();
    }

    void radix__flush_pmd_tlb_range(struct vm_area_struct *vma,
    unsigned long start, unsigned long end)
    {
    radix__flush_tlb_range_psize(vma.vm_mm, start, end, MMU_PAGE_2M);
    }
    EXPORT_SYMBOL(radix__flush_pmd_tlb_range);
    void radix__flush_pud_tlb_range(struct vm_area_struct *vma,
    unsigned long start, unsigned long end)
    {
    radix__flush_tlb_range_psize(vma.vm_mm, start, end, MMU_PAGE_1G);
    }
    EXPORT_SYMBOL(radix__flush_pud_tlb_range);
#[no_mangle]
pub unsafe extern "C" fn radix__flush_tlb_all() {
    void radix__flush_tlb_all(void)
    {
    unsigned long rb,prs,r,rs;
    let mut ric: c_ulong = RIC_FLUSH_ALL;
    rb = 0x3 << PPC_BITLSHIFT(53); /* IS = 3 */
    prs = 0; /* partition scoped */
    r = 1;   /* radix format */
    rs = 1 & ((1UL << 32) - 1); /* any LPID value to flush guest mappings */
    asm volatile("ptesync": : :"memory");
//
// now flush guest entries by passing PRS = 1 and LPID != 0
//
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(1), "i"(ric), "r"(rs) : "memory");
//
// now flush host entires by passing PRS = 0 and LPID == 0
//
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(0) : "memory");
    asm volatile("eieio; tlbsync; ptesync": : :"memory");
    }

    static __always_inline void __tlbie_pid_lpid(unsigned long pid,
    unsigned long lpid,
    unsigned long ric)
    {
    unsigned long rb, rs, prs, r;
    rb = PPC_BIT(53); /* IS = 1 */
    rs = (pid << PPC_BITLSHIFT(31)) | (lpid & ~(PPC_BITMASK(0, 31)));
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 0, rb, rs, ric, prs, r);
    }
    static __always_inline void __tlbie_va_pid_lpid(unsigned long va, unsigned long pid,
    unsigned long lpid,
    unsigned long ap, unsigned long ric)
    {
    unsigned long rb, rs, prs, r;
    rb = va & ~(PPC_BITMASK(52, 63));
    rb |= ap << PPC_BITLSHIFT(58);
    rs = (pid << PPC_BITLSHIFT(31)) | (lpid & ~(PPC_BITMASK(0, 31)));
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIE_5(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIE_5(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    trace_tlbie(0, 0, rb, rs, ric, prs, r);
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_tlbie_pid_lpid(pid: c_ulong, lpid: c_ulong) {
    static inline void fixup_tlbie_pid_lpid(unsigned long pid, unsigned long lpid)
    {
//
// We can use any address for the invalidation, pick one which is
// probably unused as an optimisation.
//
    let mut va: c_ulong = ((1UL << 52) - 1);
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync" : : : "memory");
    __tlbie_pid_lpid(0, lpid, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync" : : : "memory");
    __tlbie_va_pid_lpid(va, pid, lpid, mmu_get_ap(MMU_PAGE_64K),
    RIC_FLUSH_TLB);
    }
    }
    static inline void _tlbie_pid_lpid(unsigned long pid, unsigned long lpid,
    unsigned long ric)
    {
    asm volatile("ptesync" : : : "memory");
//
// Workaround the fact that the "ric" argument to __tlbie_pid
// must be a compile-time contraint to match the "i" constraint
// in the asm statement.
//
    switch (ric) {
    case RIC_FLUSH_TLB:
    __tlbie_pid_lpid(pid, lpid, RIC_FLUSH_TLB);
    fixup_tlbie_pid_lpid(pid, lpid);
    break;
    case RIC_FLUSH_PWC:
    __tlbie_pid_lpid(pid, lpid, RIC_FLUSH_PWC);
    break;
    case RIC_FLUSH_ALL:
    default:
    __tlbie_pid_lpid(pid, lpid, RIC_FLUSH_ALL);
    fixup_tlbie_pid_lpid(pid, lpid);
    }
    asm volatile("eieio; tlbsync; ptesync" : : : "memory");
    }
    static inline void fixup_tlbie_va_range_pid_lpid(unsigned long va,
    unsigned long pid,
    unsigned long lpid,
    unsigned long ap)
    {
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG)) {
    asm volatile("ptesync" : : : "memory");
    __tlbie_pid_lpid(0, lpid, RIC_FLUSH_TLB);
    }
    if (cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG)) {
    asm volatile("ptesync" : : : "memory");
    __tlbie_va_pid_lpid(va, pid, lpid, ap, RIC_FLUSH_TLB);
    }
    }
    static inline void __tlbie_va_range_pid_lpid(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long lpid,
    unsigned long page_size,
    unsigned long psize)
    {
    unsigned long addr;
    let mut ap: c_ulong = mmu_get_ap(psize);
    for (addr = start; addr < end; addr += page_size)
    __tlbie_va_pid_lpid(addr, pid, lpid, ap, RIC_FLUSH_TLB);
    fixup_tlbie_va_range_pid_lpid(addr - page_size, pid, lpid, ap);
    }
    static inline void _tlbie_va_range_pid_lpid(unsigned long start, unsigned long end,
    unsigned long pid, unsigned long lpid,
    unsigned long page_size,
    unsigned long psize, bool also_pwc)
    {
    asm volatile("ptesync" : : : "memory");
    if (also_pwc)
    __tlbie_pid_lpid(pid, lpid, RIC_FLUSH_PWC);
    __tlbie_va_range_pid_lpid(start, end, pid, lpid, page_size, psize);
    asm volatile("eieio; tlbsync; ptesync" : : : "memory");
    }
//
// Performs process-scoped invalidations for a given LPID
// as part of H_RPT_INVALIDATE hcall.
//
    void do_h_rpt_invalidate_prt(unsigned long pid, unsigned long lpid,
    unsigned long type, unsigned long pg_sizes,
    unsigned long start, unsigned long end)
    {
    unsigned long psize, nr_pages;
    struct mmu_psize_def *def;
    bool flush_pid;
//
// A H_RPTI_TYPE_ALL request implies RIC=3, hence
// do a single IS=1 based flush.
//
    if ((type & H_RPTI_TYPE_ALL) == H_RPTI_TYPE_ALL) {
    _tlbie_pid_lpid(pid, lpid, RIC_FLUSH_ALL);
    return;
    }
    if (type & H_RPTI_TYPE_PWC)
    _tlbie_pid_lpid(pid, lpid, RIC_FLUSH_PWC);
// Full PID flush
    if (start == 0 && end == -1)
    return _tlbie_pid_lpid(pid, lpid, RIC_FLUSH_TLB);
// Do range invalidation for all the valid page sizes
    for (psize = 0; psize < MMU_PAGE_COUNT; psize++) {
    def = &mmu_psize_defs[psize];
    if (!(pg_sizes & def.h_rpt_pgsize))
    continue;
    nr_pages = (end - start) >> def.shift;
    flush_pid = nr_pages > tlb_single_page_flush_ceiling;
//
// If the number of pages spanning the range is above
// the ceiling, convert the request into a full PID flush.
// And since PID flush takes out all the page sizes, there
// is no need to consider remaining page sizes.
//
    if (flush_pid) {
    _tlbie_pid_lpid(pid, lpid, RIC_FLUSH_TLB);
    return;
    }
    _tlbie_va_range_pid_lpid(start, end, pid, lpid,
    (1UL << def.shift), psize, false);
    }
    }
    EXPORT_SYMBOL_GPL(do_h_rpt_invalidate_prt);

#[no_mangle]
unsafe extern "C" fn create_tlb_single_page_flush_ceiling() -> int __init {
    static int __init create_tlb_single_page_flush_ceiling(void)
    {
    debugfs_create_u32("tlb_single_page_flush_ceiling", 0600,
    arch_debugfs_dir, &tlb_single_page_flush_ceiling);
    debugfs_create_u32("tlb_local_single_page_flush_ceiling", 0600,
    arch_debugfs_dir, &tlb_local_single_page_flush_ceiling);
    return 0;
    }
    late_initcall(create_tlb_single_page_flush_ceiling);
