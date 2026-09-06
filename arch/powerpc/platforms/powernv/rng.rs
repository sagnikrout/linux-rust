//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/rng.c
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
// Copyright 2013, Michael Ellerman, IBM Corporation.
//

pub const DARN_ERR: c_uint = 0xFFFFFFFFFFFFFFFFul;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_rng {
    pub regs: *mut void __iomem,
    pub regs_real: *mut void __iomem,
    pub mask: c_ulong,
}

    static DEFINE_PER_CPU(struct pnv_rng *, pnv_rng);
#[no_mangle]
unsafe extern "C" fn rng_whiten(rng: *mut pnv_rng, val: c_ulong) -> c_ulong {
    static unsigned long rng_whiten(struct pnv_rng *rng, unsigned long val)
    {
    unsigned long parity;
// Calculate the parity of the value
    asm (".machine push;   \
    .machine power7; \
    popcntd %0,%1;   \
    .machine pop;"
    : "=r" (parity) : "r" (val));
// xor our value with the previous mask
    val ^= rng.mask;
// update the mask based on the parity of this value
    rng.mask = (rng.mask << 1) | (parity & 1);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pnv_get_random_darn(v: *mut c_ulong) -> c_int {
    static int pnv_get_random_darn(unsigned long *v)
    {
    unsigned long val;
// Using DARN with L=1 - 64-bit conditioned random number
    asm volatile(PPC_DARN(%0, 1) : "=r"(val));
    if (val == DARN_ERR)
    return 0;
// v = val;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn initialise_darn() -> int __init {
    static int __init initialise_darn(void)
    {
    unsigned long val;
    int i;
    if (!cpu_has_feature(CPU_FTR_ARCH_300))
    return -ENODEV;
    for (i = 0; i < 10; i++) {
    if (pnv_get_random_darn(&val)) {
    ppc_md.get_random_seed = pnv_get_random_darn;
    return 0;
    }
    }
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn pnv_get_random_long(v: *mut c_ulong) -> c_int {
    int pnv_get_random_long(unsigned long *v)
    {
    struct pnv_rng *rng;
    if (mfmsr() & MSR_DR) {
    rng = get_cpu_var(pnv_rng);
// v = rng_whiten(rng, in_be64(rng->regs));
    put_cpu_var(rng);
    } else {
    rng = raw_cpu_read(pnv_rng);
// v = rng_whiten(rng, __raw_rm_readq(rng->regs_real));
    }
    return 1;
    }
    EXPORT_SYMBOL_GPL(pnv_get_random_long);
    static __init void rng_init_per_cpu(struct pnv_rng *rng,
    struct device_node *dn)
    {
    int chip_id, cpu;
    chip_id = of_get_ibm_chip_id(dn);
    if (chip_id == -1)
    pr_warn("No ibm,chip-id found for %pOF.\n", dn);
    for_each_possible_cpu(cpu) {
    if (per_cpu(pnv_rng, cpu) == core::ptr::null_mut() ||
    cpu_to_chip_id(cpu) == chip_id) {
    per_cpu(pnv_rng, cpu) = rng;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn rng_create(dn: *mut device_node) -> __init int {
    static __init int rng_create(struct device_node *dn)
    {
    struct pnv_rng *rng;
    struct resource res;
    unsigned long val;
    rng = kzalloc_obj(*rng);
    if (!rng)
    return -ENOMEM;
    if (of_address_to_resource(dn, 0, &res)) {
    kfree(rng);
    return -ENXIO;
    }
    rng.regs_real = (void __iomem *)res.start;
    rng.regs = of_iomap(dn, 0);
    if (!rng.regs) {
    kfree(rng);
    return -ENXIO;
    }
    val = in_be64(rng.regs);
    rng.mask = val;
    rng_init_per_cpu(rng, dn);
    ppc_md.get_random_seed = pnv_get_random_long;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pnv_get_random_long_early(v: *mut c_ulong) -> int __init {
    static int __init pnv_get_random_long_early(unsigned long *v)
    {
    struct device_node *dn;
    if (!slab_is_available())
    return 0;
    if (cmpxchg(&ppc_md.get_random_seed, pnv_get_random_long_early,
    core::ptr::null_mut()) != pnv_get_random_long_early)
    return 0;
    for_each_compatible_node(dn, core::ptr::null_mut(), "ibm,power-rng")
    rng_create(dn);
    if (!ppc_md.get_random_seed)
    return 0;
    return ppc_md.get_random_seed(v);
    }
#[no_mangle]
pub unsafe extern "C" fn pnv_rng_init() -> void __init {
    void __init pnv_rng_init(void)
    {
    struct device_node *dn;
// Prefer darn over the rest.
    if (!initialise_darn())
    return;
    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,power-rng");
    if (dn)
    ppc_md.get_random_seed = pnv_get_random_long_early;
    of_node_put(dn);
    }
#[no_mangle]
unsafe extern "C" fn pnv_rng_late_init() -> int __init {
    static int __init pnv_rng_late_init(void)
    {
    struct device_node *dn;
    unsigned long v;
// In case it wasn't called during init for some other reason.
    if (ppc_md.get_random_seed == pnv_get_random_long_early)
    pnv_get_random_long_early(&v);
    if (ppc_md.get_random_seed == pnv_get_random_long) {
    for_each_compatible_node(dn, core::ptr::null_mut(), "ibm,power-rng")
    of_platform_device_create(dn, core::ptr::null_mut(), core::ptr::null_mut());
    }
    return 0;
    }
    machine_subsys_initcall(powernv, pnv_rng_late_init);
