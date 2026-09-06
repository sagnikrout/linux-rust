//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_ffh.c
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
// Author: Sudeep Holla <sudeep.holla@arm.com>
// Copyright 2022 Arm Limited
//

    static struct acpi_ffh_info ffh_ctx;
    int __weak acpi_ffh_address_space_arch_setup(void *handler_ctxt,
    void **region_ctxt)
    {
    return -EOPNOTSUPP;
    }
    int __weak acpi_ffh_address_space_arch_handler(acpi_integer *value,
    void *region_context)
    {
    return -EOPNOTSUPP;
    }
    static acpi_status
    acpi_ffh_address_space_setup(acpi_handle region_handle, u32 function,
    void *handler_context,  void **region_context)
    {
    return acpi_ffh_address_space_arch_setup(handler_context,
    region_context);
    }
    static acpi_status
    acpi_ffh_address_space_handler(u32 function, acpi_physical_address addr,
    u32 bits, acpi_integer *value,
    void *handler_context, void *region_context)
    {
    return acpi_ffh_address_space_arch_handler(value, region_context);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_init_ffh() -> void __init {
    void __init acpi_init_ffh(void)
    {
    acpi_status status;
    status = acpi_install_address_space_handler(ACPI_ROOT_OBJECT,
    ACPI_ADR_SPACE_FIXED_HARDWARE,
    &acpi_ffh_address_space_handler,
    &acpi_ffh_address_space_setup,
    &ffh_ctx);
    if (ACPI_FAILURE(status))
    pr_alert("OperationRegion handler could not be installed\n");
    }
