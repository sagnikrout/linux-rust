//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_ffa/smccc.c
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
// Copyright (C) 2021 ARM Ltd.
//

#[no_mangle]
unsafe extern "C" fn __arm_ffa_fn_smc(args: ffa_value_t, res: *mut ffa_value_t) {
    static void __arm_ffa_fn_smc(ffa_value_t args, ffa_value_t *res)
    {
    arm_smccc_1_2_smc(&args, res);
    }
#[no_mangle]
unsafe extern "C" fn __arm_ffa_fn_hvc(args: ffa_value_t, res: *mut ffa_value_t) {
    static void __arm_ffa_fn_hvc(ffa_value_t args, ffa_value_t *res)
    {
    arm_smccc_1_2_hvc(&args, res);
    }
#[no_mangle]
pub unsafe extern "C" fn ffa_transport_init(invoke_ffa_fn: *mut ffa_fn) -> c_int {
    int ffa_transport_init(ffa_fn **invoke_ffa_fn)
    {
    enum arm_smccc_conduit conduit;
    if (arm_smccc_get_version() < ARM_SMCCC_VERSION_1_2)
    return -EOPNOTSUPP;
    conduit = arm_smccc_1_1_get_conduit();
    if (conduit == SMCCC_CONDUIT_NONE) {
    pr_err("%s: invalid SMCCC conduit\n", __func__);
    return -EOPNOTSUPP;
    }
    if (conduit == SMCCC_CONDUIT_SMC)
// invoke_ffa_fn = __arm_ffa_fn_smc;
    else
// invoke_ffa_fn = __arm_ffa_fn_hvc;
    return 0;
    }
