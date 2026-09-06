//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/powerpc.c
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

// Inserts new slb entries
#[no_mangle]
unsafe extern "C" fn insert_slb_entry(p: c_ulong, ssize: c_int, page_size: c_int) {
    static void insert_slb_entry(unsigned long p, int ssize, int page_size)
    {
    unsigned long flags;
    flags = SLB_VSID_KERNEL | mmu_psize_defs[page_size].sllp;
    preempt_disable();
    asm volatile("slbmte %0,%1" :
    : "r" (mk_vsid_data(p, ssize, flags)),
    "r" (mk_esid_data(p, ssize, SLB_NUM_BOLTED))
    : "memory");
    isync();
    asm volatile("slbmte %0,%1" :
    : "r" (mk_vsid_data(p, ssize, flags)),
    "r" (mk_esid_data(p, ssize, SLB_NUM_BOLTED + 1))
    : "memory");
    isync();
    preempt_enable();
    }
// Inject slb multihit on vmalloc-ed address i.e 0xD00...
#[no_mangle]
unsafe extern "C" fn inject_vmalloc_slb_multihit() -> c_int {
    static int inject_vmalloc_slb_multihit(void)
    {
    char *p;
    p = vmalloc(PAGE_SIZE);
    if (!p)
    return -ENOMEM;
    insert_slb_entry((unsigned long)p, MMU_SEGSIZE_1T, mmu_vmalloc_psize);
//
// This triggers exception, If handled correctly we must recover
// from this error.
//
    p[0] = '!';
    vfree(p);
    return 0;
    }
// Inject slb multihit on kmalloc-ed address i.e 0xC00...
#[no_mangle]
unsafe extern "C" fn inject_kmalloc_slb_multihit() -> c_int {
    static int inject_kmalloc_slb_multihit(void)
    {
    char *p;
    p = kmalloc(2048, GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    insert_slb_entry((unsigned long)p, MMU_SEGSIZE_1T, mmu_linear_psize);
//
// This triggers exception, If handled correctly we must recover
// from this error.
//
    p[0] = '!';
    kfree(p);
    return 0;
    }
//
// Few initial SLB entries are bolted. Add a test to inject
// multihit in bolted entry 0.
//
#[no_mangle]
unsafe extern "C" fn insert_dup_slb_entry_0() {
    static void insert_dup_slb_entry_0(void)
    {
    let mut test_address: c_ulong = PAGE_OFFSET, *test_ptr;
    unsigned long esid, vsid;
    let mut i: c_ulong = 0;
    test_ptr = (unsigned long *)test_address;
    preempt_disable();
    asm volatile("slbmfee  %0,%1" : "=r" (esid) : "r" (i));
    asm volatile("slbmfev  %0,%1" : "=r" (vsid) : "r" (i));
// for i !=0 we would need to mask out the old entry number
    asm volatile("slbmte %0,%1" :
    : "r" (vsid),
    "r" (esid | SLB_NUM_BOLTED)
    : "memory");
    isync();
    asm volatile("slbmfee  %0,%1" : "=r" (esid) : "r" (i));
    asm volatile("slbmfev  %0,%1" : "=r" (vsid) : "r" (i));
// for i !=0 we would need to mask out the old entry number
    asm volatile("slbmte %0,%1" :
    : "r" (vsid),
    "r" (esid | (SLB_NUM_BOLTED + 1))
    : "memory");
    isync();
    pr_info("%s accessing test address 0x%lx: 0x%lx\n",
    __func__, test_address, *test_ptr);
    preempt_enable();
    }

    static __always_inline void tlbiel_va(unsigned long va,
    unsigned long pid,
    unsigned long ap,
    unsigned long ric)
    {
    unsigned long rb, rs, prs, r;
    rb = va & ~(PPC_BITMASK(52, 63));
    rb |= ap << PPC_BITLSHIFT(58);
    rs = pid << PPC_BITLSHIFT(31);
    prs = 1; /* process scoped */
    r = 1;   /* radix format */
//
// Trigger an MCE by issuing radix tlbiel with an invalid operand combination.
// The combination of RIC = 2 with IS = 0 (Invalidation selector specified
// in the RB register) is invalid.
// This invalid combination causes hardware to raise a machine check.
//
#[no_mangle]
pub unsafe extern "C" fn volatile(_arg: PPC_TLBIEL(%0, _arg: %4, _arg: %3, _arg: %2, _arg: %1) -> asm {
    asm volatile(PPC_TLBIEL(%0, %4, %3, %2, %1)
    : : "r"(rb), "i"(r), "i"(prs), "i"(ric), "r"(rs) : "memory");
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PPC_SLB_MULTIHIT() {
    static void lkdtm_PPC_SLB_MULTIHIT(void)
    {

    if (!radix_enabled()) {
    pr_info("Injecting SLB multihit errors\n");
//
// These need not be separate tests, And they do pretty
// much same thing. In any case we must recover from the
// errors introduced by these functions, machine would not
// survive these tests in case of failure to handle.
//
    inject_vmalloc_slb_multihit();
    inject_kmalloc_slb_multihit();
    insert_dup_slb_entry_0();
    pr_info("Recovered from SLB multihit errors\n");
    } else {
    pr_err("XFAIL: This test is for ppc64 and with hash mode MMU only\n");
    }

    pr_err("XFAIL: This test requires CONFIG_PPC_64S_HASH_MMU\n");

    }
#[no_mangle]
unsafe extern "C" fn lkdtm_PPC_RADIX_TLBIEL() {
    static void lkdtm_PPC_RADIX_TLBIEL(void)
    {
    let mut addr: c_ulong = PAGE_OFFSET;
    if (radix_enabled()) {
    pr_info("Injecting Radix TLB invalidation MCE\n");
    tlbiel_va(addr, 0, 0, RIC_FLUSH_ALL);
    pr_info("Recovered from radix tlbiel attempt\n");
    } else {
    pr_err("XFAIL: This test is for ppc64 and with radix mode MMU only\n");
    }
    }
    static struct crashtype crashtypes[] = {
    CRASHTYPE(PPC_SLB_MULTIHIT),
    CRASHTYPE(PPC_RADIX_TLBIEL),
    };
    struct crashtype_category powerpc_crashtypes = {
    .crashtypes = crashtypes,
    .len	    = ARRAY_SIZE(crashtypes),
    };
