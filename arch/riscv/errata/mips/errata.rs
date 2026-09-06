//! Automatically rewritten from C to Rust
//! Source: arch/riscv/errata/mips/errata.c
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
// Copyright (C) 2025 MIPS.
//

#[no_mangle]
pub unsafe extern "C" fn errata_probe_pause() -> bool {
    static inline bool errata_probe_pause(void)
    {
    if (!IS_ENABLED(CONFIG_ERRATA_MIPS_P8700_PAUSE_OPCODE))
    return false;
    if (!riscv_isa_vendor_extension_available(MIPS_VENDOR_ID, XMIPSEXECTL))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mips_errata_probe() -> u32 {
    static u32 mips_errata_probe(void)
    {
    let mut cpu_req_errata: u32 = 0;
    if (errata_probe_pause())
    cpu_req_errata |= BIT(ERRATA_MIPS_P8700_PAUSE_OPCODE);
    return cpu_req_errata;
    }
    void mips_errata_patch_func(struct alt_entry *begin, struct alt_entry *end,
    unsigned long archid, unsigned long impid,
    unsigned int stage)
    {
    struct alt_entry *alt;
    let mut cpu_req_errata: u32 = mips_errata_probe();
    u32 tmp;
    BUILD_BUG_ON(ERRATA_MIPS_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);
    if (stage == RISCV_ALTERNATIVES_EARLY_BOOT)
    return;
    for (alt = begin; alt < end; alt++) {
    if (alt.vendor_id != MIPS_VENDOR_ID)
    continue;
    if (alt.patch_id >= ERRATA_MIPS_NUMBER) {
    WARN(1, "MIPS errata id:%d not in kernel errata list\n",
    alt.patch_id);
    continue;
    }
    tmp = (1U << alt.patch_id);
    if (cpu_req_errata & tmp) {
    mutex_lock(&text_mutex);
    patch_text_nosync(ALT_OLD_PTR(alt), ALT_ALT_PTR(alt),
    alt.alt_len);
    mutex_unlock(&text_mutex);
    }
    }
    }
