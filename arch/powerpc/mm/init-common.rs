//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/init-common.c
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
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Dave Engebretsen <engebret@us.ibm.com>
// Rework for PPC64 port.
//

    let mut __ro_after_init: phys_addr_t memstart_addr = (phys_addr_t)~0ull;
    EXPORT_SYMBOL_GPL(memstart_addr);
    phys_addr_t kernstart_addr __ro_after_init;
    EXPORT_SYMBOL_GPL(kernstart_addr);
    let mut __ro_after_init: unsigned long kernstart_virt_addr = KERNELBASE;
    EXPORT_SYMBOL_GPL(kernstart_virt_addr);
    let mut disable_kuep: bool = !IS_ENABLED(CONFIG_PPC_KUEP);
    let mut disable_kuap: bool = !IS_ENABLED(CONFIG_PPC_KUAP);

    bool __ro_after_init kfence_disabled;
    let mut kfence_early_init: bool __ro_after_init = !!CONFIG_KFENCE_SAMPLE_INTERVAL;

#[no_mangle]
unsafe extern "C" fn parse_nosmep(p: *mut c_char) -> int __init {
    static int __init parse_nosmep(char *p)
    {
    if (!IS_ENABLED(CONFIG_PPC_BOOK3S_64))
    return 0;
    disable_kuep = true;
    pr_warn("Disabling Kernel Userspace Execution Prevention\n");
    return 0;
    }
    early_param("nosmep", parse_nosmep);
#[no_mangle]
unsafe extern "C" fn parse_nosmap(p: *mut c_char) -> int __init {
    static int __init parse_nosmap(char *p)
    {
    disable_kuap = true;
    pr_warn("Disabling Kernel Userspace Access Protection\n");
    return 0;
    }
    early_param("nosmap", parse_nosmap);
#[no_mangle]
pub unsafe extern "C" fn setup_kuep(disabled: bool) -> void __weak {
    void __weak setup_kuep(bool disabled)
    {
    if (!IS_ENABLED(CONFIG_PPC_KUEP) || disabled)
    return;
    if (smp_processor_id() != boot_cpuid)
    return;
    pr_info("Activating Kernel Userspace Execution Prevention\n");
    }
#[no_mangle]
pub unsafe extern "C" fn setup_kup() {
    void setup_kup(void)
    {
    setup_kuap(disable_kuap);
    setup_kuep(disable_kuep);
    }

    {							\
    memset(addr, 0, sizeof(pgd_t) << (shift));	\
    }
    CTOR(0); CTOR(1); CTOR(2); CTOR(3); CTOR(4); CTOR(5); CTOR(6); CTOR(7);
    CTOR(8); CTOR(9); CTOR(10); CTOR(11); CTOR(12); CTOR(13); CTOR(14); CTOR(15);
#[no_mangle]
unsafe extern "C" fn void(: *mut *mut ctor(int shift))(void) -> inline {
    static inline void (*ctor(int shift))(void *)
    {
    BUILD_BUG_ON(MAX_PGTABLE_INDEX_SIZE != 15);
    switch (shift) {
    case 0: return ctor_0;
    case 1: return ctor_1;
    case 2: return ctor_2;
    case 3: return ctor_3;
    case 4: return ctor_4;
    case 5: return ctor_5;
    case 6: return ctor_6;
    case 7: return ctor_7;
    case 8: return ctor_8;
    case 9: return ctor_9;
    case 10: return ctor_10;
    case 11: return ctor_11;
    case 12: return ctor_12;
    case 13: return ctor_13;
    case 14: return ctor_14;
    case 15: return ctor_15;
    }
    return core::ptr::null_mut();
    }
    struct kmem_cache *pgtable_cache[MAX_PGTABLE_INDEX_SIZE + 1];
    EXPORT_SYMBOL_GPL(pgtable_cache);	/* used by kvm_hv module */
//
// Create a kmem_cache() for pagetables.  This is not used for PTE
// pages - they're linked to struct page, come from the normal free
// pages pool and have a different entry size (see real_pte_t) to
// everything else.  Caches created by this function are used for all
// the higher level pagetables, and for hugepage pagetables.
//
#[no_mangle]
pub unsafe extern "C" fn pgtable_cache_add(shift: c_uint) {
    void pgtable_cache_add(unsigned int shift)
    {
    char *name;
    let mut table_size: c_ulong = sizeof(pgd_t) << shift;
    let mut align: c_ulong = table_size;
// When batching pgtable pointers for RCU freeing, we store
// the index size in the low bits.  Table alignment must be
// big enough to fit it.
//
    let mut minalign: c_ulong = MAX_PGTABLE_INDEX_SIZE + 1;
    struct kmem_cache *new = core::ptr::null_mut();
// It would be nice if this was a BUILD_BUG_ON(), but at the
// moment, gcc doesn't seem to recognize is_power_of_2 as a
// constant expression, so so much for that.
    BUG_ON(!is_power_of_2(minalign));
    BUG_ON(shift > MAX_PGTABLE_INDEX_SIZE);
    if (PGT_CACHE(shift))
    return; /* Already have a cache of this size */
    align = max_t(unsigned long, align, minalign);
    name = kasprintf(GFP_KERNEL, "pgtable-2^%d", shift);
    if (name)
    new = kmem_cache_create(name, table_size, align, 0, ctor(shift));
    if (!new)
    panic("Could not allocate pgtable cache for order %d", shift);
    kfree(name);
    pgtable_cache[shift] = new;
    pr_debug("Allocated pgtable cache for order %d\n", shift);
    }
    EXPORT_SYMBOL_GPL(pgtable_cache_add);	/* used by kvm_hv module */
#[no_mangle]
pub unsafe extern "C" fn pgtable_cache_init() {
    void pgtable_cache_init(void)
    {
    pgtable_cache_add(PGD_INDEX_SIZE);
    if (PMD_CACHE_INDEX)
    pgtable_cache_add(PMD_CACHE_INDEX);
//
// In all current configs, when the PUD index exists it's the
// same size as either the pgd or pmd index except with THP enabled
// on book3s 64
//
    if (PUD_CACHE_INDEX)
    pgtable_cache_add(PUD_CACHE_INDEX);
    }
