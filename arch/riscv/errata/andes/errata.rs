//! Automatically rewritten from C to Rust
//! Source: arch/riscv/errata/andes/errata.c
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
// Erratas to be applied for Andes CPU cores
//
// Copyright (C) 2023 Renesas Electronics Corporation.
//
// Author: Lad Prabhakar <prabhakar.mahadev-lad.rj@bp.renesas.com>
//

pub const ANDES_AX45MP_MARCHID: c_uint = 0x8000000000008a45UL;
pub const ANDES_AX45MP_MIMPID: c_uint = 0x500UL;
pub const ANDES_SBI_EXT_ANDES: c_uint = 0x0900031E;
pub const ANDES_SBI_EXT_IOCP_SW_WORKAROUND: c_int = 1;
#[no_mangle]
unsafe extern "C" fn ax45mp_iocp_sw_workaround() -> c_long {
    static long ax45mp_iocp_sw_workaround(void)
    {
    struct sbiret ret;
//
// ANDES_SBI_EXT_IOCP_SW_WORKAROUND SBI EXT checks if the IOCP is missing and
// cache is controllable only then CMO will be applied to the platform.
//
    ret = sbi_ecall(ANDES_SBI_EXT_ANDES, ANDES_SBI_EXT_IOCP_SW_WORKAROUND,
    0, 0, 0, 0, 0, 0);
    return ret.error ? 0 : ret.value;
    }
#[no_mangle]
unsafe extern "C" fn errata_probe_iocp(stage: c_uint, arch_id: c_ulong, impid: c_ulong) {
    static void errata_probe_iocp(unsigned int stage, unsigned long arch_id, unsigned long impid)
    {
    static bool done;
    if (!IS_ENABLED(CONFIG_ERRATA_ANDES_CMO))
    return;
    if (done)
    return;
    done = true;
    if (arch_id != ANDES_AX45MP_MARCHID || impid != ANDES_AX45MP_MIMPID)
    return;
    if (!ax45mp_iocp_sw_workaround())
    return;
// Set this just to make core cbo code happy
    riscv_cbom_block_size = 1;
    riscv_noncoherent_supported();
    }
    void __init_or_module andes_errata_patch_func(struct alt_entry *begin, struct alt_entry *end,
    unsigned long archid, unsigned long impid,
    unsigned int stage)
    {
    BUILD_BUG_ON(ERRATA_ANDES_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);
    if (stage == RISCV_ALTERNATIVES_BOOT)
    errata_probe_iocp(stage, archid, impid);
// we have nothing to patch here ATM so just return back
    }
