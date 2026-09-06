//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/mxm-wmi.c
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
// MXM WMI driver
//
// Copyright(C) 2010 Red Hat.
//

    MODULE_AUTHOR("Dave Airlie");
    MODULE_DESCRIPTION("MXM WMI Driver");
    MODULE_LICENSE("GPL");

    MODULE_ALIAS("wmi:"MXM_WMMX_GUID);
pub const MXM_WMMX_FUNC_MXDS: c_uint = 0x5344584D /* "MXDS" */;
pub const MXM_WMMX_FUNC_MXMX: c_uint = 0x53445344 /* "MXMX" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxds_args {
    pub func: u32,
    pub args: u32,
    pub xarg: u32,
}

#[no_mangle]
pub unsafe extern "C" fn mxm_wmi_call_mxds(adapter: c_int) -> c_int {
    int mxm_wmi_call_mxds(int adapter)
    {
    struct mxds_args args = {
    .func = MXM_WMMX_FUNC_MXDS,
    .args = 0,
    .xarg = 1,
    };
    let mut input: acpi_buffer = { (acpi_size)sizeof(args), &args };
    acpi_status status;
    printk("calling mux switch %d\n", adapter);
    status = wmi_evaluate_method(MXM_WMMX_GUID, 0x0, adapter, &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return status;
    printk("mux switched %d\n", status);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mxm_wmi_call_mxds);
#[no_mangle]
pub unsafe extern "C" fn mxm_wmi_call_mxmx(adapter: c_int) -> c_int {
    int mxm_wmi_call_mxmx(int adapter)
    {
    struct mxds_args args = {
    .func = MXM_WMMX_FUNC_MXMX,
    .args = 0,
    .xarg = 1,
    };
    let mut input: acpi_buffer = { (acpi_size)sizeof(args), &args };
    acpi_status status;
    printk("calling mux switch %d\n", adapter);
    status = wmi_evaluate_method(MXM_WMMX_GUID, 0x0, adapter, &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return status;
    printk("mux mutex set switched %d\n", status);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mxm_wmi_call_mxmx);
#[no_mangle]
pub unsafe extern "C" fn mxm_wmi_supported() -> bool {
    bool mxm_wmi_supported(void)
    {
    bool guid_valid;
    guid_valid = wmi_has_guid(MXM_WMMX_GUID);
    return guid_valid;
    }
    EXPORT_SYMBOL_GPL(mxm_wmi_supported);
