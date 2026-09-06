//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/ffh.c
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
// Implements ARM64 specific callbacks to support ACPI FFH Operation Region as
// specified in https://developer.arm.com/docs/den0048/latest
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ffh_data {
    pub info: acpi_ffh_info,
    void (*invoke_ffh_fn)(unsigned long a0, unsigned long a1,
    unsigned long a2, unsigned long a3,
    unsigned long a4, unsigned long a5,
    unsigned long a6, unsigned long a7,
    struct arm_smccc_res *args,
    pub res): *mut arm_smccc_quirk,
    void (*invoke_ffh64_fn)(const struct arm_smccc_1_2_regs *args,
    pub res): *mut arm_smccc_1_2_regs,
}

#[no_mangle]
pub unsafe extern "C" fn acpi_ffh_address_space_arch_setup(handler_ctxt: *mut c_void, region_ctxt: *mut c_void) -> c_int {
    int acpi_ffh_address_space_arch_setup(void *handler_ctxt, void **region_ctxt)
    {
    enum arm_smccc_conduit conduit;
    struct acpi_ffh_data *ffh_ctxt;
    if (arm_smccc_get_version() < ARM_SMCCC_VERSION_1_2)
    return -EOPNOTSUPP;
    conduit = arm_smccc_1_1_get_conduit();
    if (conduit == SMCCC_CONDUIT_NONE) {
    pr_err("%s: invalid SMCCC conduit\n", __func__);
    return -EOPNOTSUPP;
    }
    ffh_ctxt = kzalloc_obj(*ffh_ctxt);
    if (!ffh_ctxt)
    return -ENOMEM;
    if (conduit == SMCCC_CONDUIT_SMC) {
    ffh_ctxt.invoke_ffh_fn = __arm_smccc_smc;
    ffh_ctxt.invoke_ffh64_fn = arm_smccc_1_2_smc;
    } else {
    ffh_ctxt.invoke_ffh_fn = __arm_smccc_hvc;
    ffh_ctxt.invoke_ffh64_fn = arm_smccc_1_2_hvc;
    }
    memcpy(ffh_ctxt, handler_ctxt, sizeof(ffh_ctxt.info));
// region_ctxt = ffh_ctxt;
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn acpi_ffh_smccc_owner_allowed(fid: u32) -> bool {
    static bool acpi_ffh_smccc_owner_allowed(u32 fid)
    {
    let mut owner: c_int = ARM_SMCCC_OWNER_NUM(fid);
    if (owner == ARM_SMCCC_OWNER_STANDARD ||
    owner == ARM_SMCCC_OWNER_SIP || owner == ARM_SMCCC_OWNER_OEM)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_ffh_address_space_arch_handler(value: *mut acpi_integer, region_context: *mut c_void) -> c_int {
    int acpi_ffh_address_space_arch_handler(acpi_integer *value, void *region_context)
    {
    let mut ret: c_int = 0;
    struct acpi_ffh_data *ffh_ctxt = region_context;
    if (ffh_ctxt.info.offset == 0) {
// SMC/HVC 32bit call
    struct arm_smccc_res res;
    u32 a[8] = { 0 }, *ptr = (u32 *)value;
    if (!ARM_SMCCC_IS_FAST_CALL(*ptr) || ARM_SMCCC_IS_64(*ptr) ||
    !acpi_ffh_smccc_owner_allowed(*ptr) ||
    ffh_ctxt.info.length > 32) {
    ret = AE_ERROR;
    } else {
    int idx, len = ffh_ctxt.info.length >> 2;
    for (idx = 0; idx < len; idx++)
    a[idx] = *(ptr + idx);
    ffh_ctxt.invoke_ffh_fn(a[0], a[1], a[2], a[3], a[4],
    a[5], a[6], a[7], &res, core::ptr::null_mut());
    memcpy(value, &res, sizeof(res));
    }
    } else if (ffh_ctxt.info.offset == 1) {
// SMC/HVC 64bit call
    struct arm_smccc_1_2_regs *r = (struct arm_smccc_1_2_regs *)value;
    if (!ARM_SMCCC_IS_FAST_CALL(r.a0) || !ARM_SMCCC_IS_64(r.a0) ||
    !acpi_ffh_smccc_owner_allowed(r.a0) ||
    ffh_ctxt.info.length > sizeof(*r)) {
    ret = AE_ERROR;
    } else {
    ffh_ctxt.invoke_ffh64_fn(r, r);
    memcpy(value, r, ffh_ctxt.info.length);
    }
    } else {
    ret = AE_ERROR;
    }
    return ret;
    }
