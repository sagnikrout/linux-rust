//! Automatically rewritten from C to Rust
//! Source: arch/riscv/errata/sifive/errata.c
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
// Copyright (C) 2021 Sifive.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct errata_info_t {
    pub name: [c_char; 32],
    pub impid): *mut *mut bool (check_func)(unsigned long arch_id, unsigned long,
}

#[no_mangle]
unsafe extern "C" fn errata_cip_453_check_func(arch_id: c_ulong, impid: c_ulong) -> bool {
    static bool errata_cip_453_check_func(unsigned long  arch_id, unsigned long impid)
    {
//
// Affected cores:
// Architecture ID: 0x8000000000000007
// Implement ID: 0x20181004 <= impid <= 0x20191105
//
    if (arch_id != 0x8000000000000007 ||
    (impid < 0x20181004 || impid > 0x20191105))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn errata_cip_1200_check_func(arch_id: c_ulong, impid: c_ulong) -> bool {
    static bool errata_cip_1200_check_func(unsigned long  arch_id, unsigned long impid)
    {
//
// Affected cores:
// Architecture ID: 0x8000000000000007 or 0x1
// Implement ID: mimpid[23:0] <= 0x200630 and mimpid != 0x01200626
//
    if (arch_id != 0x8000000000000007 && arch_id != 0x1)
    return false;
    if ((impid & 0xffffff) > 0x200630 || impid == 0x1200626)
    return false;

    tlb_flush_all_threshold = 0;

    return true;
    }
    static struct errata_info_t errata_list[ERRATA_SIFIVE_NUMBER] = {
    {
    .name = "cip-453",
    .check_func = errata_cip_453_check_func
    },
    {
    .name = "cip-1200",
    .check_func = errata_cip_1200_check_func
    },
    };
    static u32 __init_or_module sifive_errata_probe(unsigned long archid,
    unsigned long impid)
    {
    int idx;
    let mut cpu_req_errata: u32 = 0;
    for (idx = 0; idx < ERRATA_SIFIVE_NUMBER; idx++)
    if (errata_list[idx].check_func(archid, impid))
    cpu_req_errata |= (1U << idx);
    return cpu_req_errata;
    }
    void sifive_errata_patch_func(struct alt_entry *begin, struct alt_entry *end,
    unsigned long archid, unsigned long impid,
    unsigned int stage)
    {
    struct alt_entry *alt;
    u32 cpu_req_errata;
    u32 tmp;
    BUILD_BUG_ON(ERRATA_SIFIVE_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT)
    return;
    cpu_req_errata = sifive_errata_probe(archid, impid);
    for (alt = begin; alt < end; alt++) {
    if (alt.vendor_id != SIFIVE_VENDOR_ID)
    continue;
    if (alt.patch_id >= ERRATA_SIFIVE_NUMBER)
    continue;
    tmp = (1U << alt.patch_id);
    if (cpu_req_errata & tmp) {
    mutex_lock(&text_mutex);
    patch_text_nosync(ALT_OLD_PTR(alt), ALT_ALT_PTR(alt),
    alt.alt_len);
    mutex_unlock(&text_mutex);
    }
    }
    }
