//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/epapr_paravirt.c
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
// ePAPR para-virtualization support.
//
// Copyright (C) 2012 Freescale Semiconductor, Inc.
//

    extern void epapr_ev_idle(void);
    extern u32 epapr_ev_idle_start[];

    bool epapr_paravirt_enabled;
    static bool __maybe_unused epapr_has_idle;
    static int __init early_init_dt_scan_epapr(unsigned long node,
    const char *uname,
    int depth, void *data)
    {
    const u32 *insts;
    int len;
    int i;
    insts = of_get_flat_dt_prop(node, "hcall-instructions", &len);
    if (!insts)
    return 0;
    if (len % 4 || len > (4 * 4))
    return -1;
    for (i = 0; i < (len / 4); i++) {
    let mut inst: ppc_inst_t = ppc_inst(be32_to_cpu(insts[i]));
    patch_instruction(epapr_hypercall_start + i, inst);

    patch_instruction(epapr_ev_idle_start + i, inst);

    }

    if (of_get_flat_dt_prop(node, "has-idle", core::ptr::null_mut()))
    epapr_has_idle = true;

    epapr_paravirt_enabled = true;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn epapr_paravirt_early_init() -> int __init {
    int __init epapr_paravirt_early_init(void)
    {
    of_scan_flat_dt(early_init_dt_scan_epapr, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn epapr_idle_init() -> int __init {
    static int __init epapr_idle_init(void)
    {

    if (epapr_has_idle)
    ppc_md.power_save = epapr_ev_idle;

    return 0;
    }
    postcore_initcall(epapr_idle_init);
