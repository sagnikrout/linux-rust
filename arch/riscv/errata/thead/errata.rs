//! Automatically rewritten from C to Rust
//! Source: arch/riscv/errata/thead/errata.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2021 Heiko Stuebner <heiko@sntech.de>
//

pub const CSR_TH_SXSTATUS: c_uint = 0x5c0;

    static bool errata_probe_mae(unsigned int stage,
    unsigned long arch_id, unsigned long impid)
    {
    if (!IS_ENABLED(CONFIG_ERRATA_THEAD_MAE))
    return false;
    if (arch_id != 0 || impid != 0)
    return false;
    if (stage != RISCV_ALTERNATIVES_EARLY_BOOT &&
    stage != RISCV_ALTERNATIVES_MODULE)
    return false;
    if (!(csr_read(CSR_TH_SXSTATUS) & SXSTATUS_MAEE))
    return false;
    return true;
    }
//
// th.dcache.ipa rs1 (invalidate, physical address)
// | 31 - 25 | 24 - 20 | 19 - 15 | 14 - 12 | 11 - 7 | 6 - 0 |
// 0000001    01010      rs1       000      00000  0001011
// th.dcache.iva rs1 (invalidate, virtual address)
// 0000001    00110      rs1       000      00000  0001011
//
// th.dcache.cpa rs1 (clean, physical address)
// | 31 - 25 | 24 - 20 | 19 - 15 | 14 - 12 | 11 - 7 | 6 - 0 |
// 0000001    01001      rs1       000      00000  0001011
// th.dcache.cva rs1 (clean, virtual address)
// 0000001    00101      rs1       000      00000  0001011
//
// th.dcache.cipa rs1 (clean then invalidate, physical address)
// | 31 - 25 | 24 - 20 | 19 - 15 | 14 - 12 | 11 - 7 | 6 - 0 |
// 0000001    01011      rs1       000      00000  0001011
// th.dcache.civa rs1 (clean then invalidate, virtual address)
// 0000001    00111      rs1       000      00000  0001011
//
// th.sync.s (make sure all cache operations finished)
// | 31 - 25 | 24 - 20 | 19 - 15 | 14 - 12 | 11 - 7 | 6 - 0 |
// 0000000    11001     00000      000      00000  0001011
//

    asm volatile("mv a0, %1\n\t"						\
    "j 2f\n\t"							\
    "3:\n\t"							\
    THEAD_##_op##_A0 "\n\t"					\
    "add a0, a0, %0\n\t"					\
    "2:\n\t"							\
    "bltu a0, %2, 3b\n\t"					\
    THEAD_SYNC_S						\
    : : "r"(_cachesize),					\
    "r"((unsigned long)(_start) & ~((_cachesize) - 1UL)),	\
    "r"((unsigned long)(_start) + (_size))			\
    : "a0")
#[no_mangle]
unsafe extern "C" fn thead_errata_cache_inv(paddr: phys_addr_t, size: usize) {
    static void thead_errata_cache_inv(phys_addr_t paddr, size_t size)
    {
    THEAD_CMO_OP(INVAL, paddr, size, riscv_cbom_block_size);
    }
#[no_mangle]
unsafe extern "C" fn thead_errata_cache_wback(paddr: phys_addr_t, size: usize) {
    static void thead_errata_cache_wback(phys_addr_t paddr, size_t size)
    {
    THEAD_CMO_OP(CLEAN, paddr, size, riscv_cbom_block_size);
    }
#[no_mangle]
unsafe extern "C" fn thead_errata_cache_wback_inv(paddr: phys_addr_t, size: usize) {
    static void thead_errata_cache_wback_inv(phys_addr_t paddr, size_t size)
    {
    THEAD_CMO_OP(FLUSH, paddr, size, riscv_cbom_block_size);
    }
    static const struct riscv_nonstd_cache_ops thead_errata_cmo_ops = {
    .wback = &thead_errata_cache_wback,
    .inv = &thead_errata_cache_inv,
    .wback_inv = &thead_errata_cache_wback_inv,
    };
    static bool errata_probe_cmo(unsigned int stage,
    unsigned long arch_id, unsigned long impid)
    {
    if (!IS_ENABLED(CONFIG_ERRATA_THEAD_CMO))
    return false;
    if (arch_id != 0 || impid != 0)
    return false;
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT)
    return false;
    if (stage == RISCV_ALTERNATIVES_BOOT) {
    riscv_cbom_block_size = L1_CACHE_BYTES;
    riscv_noncoherent_supported();
    riscv_noncoherent_register_cache_ops(&thead_errata_cmo_ops);
    }
    return true;
    }
    static bool errata_probe_pmu(unsigned int stage,
    unsigned long arch_id, unsigned long impid)
    {
    if (!IS_ENABLED(CONFIG_ERRATA_THEAD_PMU))
    return false;
// target-c9xx cores report arch_id and impid as 0
    if (arch_id != 0 || impid != 0)
    return false;
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT)
    return false;
    return true;
    }
    static bool errata_probe_ghostwrite(unsigned int stage,
    unsigned long arch_id, unsigned long impid)
    {
    if (!IS_ENABLED(CONFIG_ERRATA_THEAD_GHOSTWRITE))
    return false;
//
// target-c9xx cores report arch_id and impid as 0
//
// While ghostwrite may not affect all c9xx cores that implement
// xtheadvector, there is no further granularity than c9xx. Assume
// vulnerable for this entire class of processors when xtheadvector is
// enabled.
//
    if (arch_id != 0 || impid != 0)
    return false;
    if (stage != RISCV_ALTERNATIVES_EARLY_BOOT)
    return false;
    ghostwrite_set_vulnerable();
    return true;
    }
    static u32 thead_errata_probe(unsigned int stage,
    unsigned long archid, unsigned long impid)
    {
    let mut cpu_req_errata: u32 = 0;
    if (errata_probe_mae(stage, archid, impid))
    cpu_req_errata |= BIT(ERRATA_THEAD_MAE);
    errata_probe_cmo(stage, archid, impid);
    if (errata_probe_pmu(stage, archid, impid))
    cpu_req_errata |= BIT(ERRATA_THEAD_PMU);
    errata_probe_ghostwrite(stage, archid, impid);
    return cpu_req_errata;
    }
    void thead_errata_patch_func(struct alt_entry *begin, struct alt_entry *end,
    unsigned long archid, unsigned long impid,
    unsigned int stage)
    {
    struct alt_entry *alt;
    let mut cpu_req_errata: u32 = thead_errata_probe(stage, archid, impid);
    u32 tmp;
    void *oldptr, *altptr;
    BUILD_BUG_ON(ERRATA_THEAD_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);
    for (alt = begin; alt < end; alt++) {
    if (alt.vendor_id != THEAD_VENDOR_ID)
    continue;
    if (alt.patch_id >= ERRATA_THEAD_NUMBER)
    continue;
    tmp = (1U << alt.patch_id);
    if (cpu_req_errata & tmp) {
    oldptr = ALT_OLD_PTR(alt);
    altptr = ALT_ALT_PTR(alt);
// On vm-alternatives, the mmu isn't running yet
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT) {
    memcpy(oldptr, altptr, alt.alt_len);
    } else {
    mutex_lock(&text_mutex);
    patch_text_nosync(oldptr, altptr, alt.alt_len);
    mutex_unlock(&text_mutex);
    }
    }
    }
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT)
    local_flush_icache_all();
    }
