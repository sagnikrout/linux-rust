//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/hwacpi.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: hwacpi - ACPI Hardware Initialization/Mode Interface
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("hwacpi")

//
// FUNCTION:    acpi_hw_set_mode
//
// PARAMETERS:  mode            - SYS_MODE_ACPI or SYS_MODE_LEGACY
//
// RETURN:      Status
//
// DESCRIPTION: Transitions the system into the requested mode.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_hw_set_mode(mode: u32) -> acpi_status {
    acpi_status acpi_hw_set_mode(u32 mode)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(hw_set_mode);
// If the Hardware Reduced flag is set, machine is always in acpi mode
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
//
// ACPI 2.0 clarified that if SMI_CMD in FADT is zero,
// system does not support mode transition.
//
    if (!acpi_gbl_FADT.smi_command) {
    ACPI_ERROR((AE_INFO,
    "No SMI_CMD in FADT, mode transition failed"));
    return_ACPI_STATUS(AE_NO_HARDWARE_RESPONSE);
    }
//
// ACPI 2.0 clarified the meaning of ACPI_ENABLE and ACPI_DISABLE
// in FADT: If it is zero, enabling or disabling is not supported.
// As old systems may have used zero for mode transition,
// we make sure both the numbers are zero to determine these
// transitions are not supported.
//
    if (!acpi_gbl_FADT.acpi_enable && !acpi_gbl_FADT.acpi_disable) {
    ACPI_ERROR((AE_INFO,
    "No ACPI mode transition supported in this system "
    "(enable/disable both zero)"));
    return_ACPI_STATUS(AE_OK);
    }
    switch (mode) {
    case ACPI_SYS_MODE_ACPI:
// BIOS should have disabled ALL fixed and GP events
    status = acpi_hw_write_port(acpi_gbl_FADT.smi_command,
    (u32) acpi_gbl_FADT.acpi_enable, 8);
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "Attempting to enable ACPI mode\n"));
    break;
    case ACPI_SYS_MODE_LEGACY:
//
// BIOS should clear all fixed status bits and restore fixed event
// enable bits to default
//
    status = acpi_hw_write_port(acpi_gbl_FADT.smi_command,
    (u32)acpi_gbl_FADT.acpi_disable, 8);
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "Attempting to enable Legacy (non-ACPI) mode\n"));
    break;
    default:
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Could not write ACPI mode change"));
    return_ACPI_STATUS(status);
    }
    return_ACPI_STATUS(AE_OK);
    }
//
// FUNCTION:    acpi_hw_get_mode
//
// PARAMETERS:  none
//
// RETURN:      SYS_MODE_ACPI or SYS_MODE_LEGACY
//
// DESCRIPTION: Return current operating state of system. Determined by
// querying the SCI_EN bit.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_hw_get_mode() -> u32 {
    u32 acpi_hw_get_mode(void)
    {
    acpi_status status;
    u32 value;
    ACPI_FUNCTION_TRACE(hw_get_mode);
// If the Hardware Reduced flag is set, machine is always in acpi mode
    if (acpi_gbl_reduced_hardware) {
    return_UINT32(ACPI_SYS_MODE_ACPI);
    }
//
// ACPI 2.0 clarified that if SMI_CMD in FADT is zero,
// system does not support mode transition.
//
    if (!acpi_gbl_FADT.smi_command) {
    return_UINT32(ACPI_SYS_MODE_ACPI);
    }
    status = acpi_read_bit_register(ACPI_BITREG_SCI_ENABLE, &value);
    if (ACPI_FAILURE(status)) {
    return_UINT32(ACPI_SYS_MODE_LEGACY);
    }
    if (value) {
    return_UINT32(ACPI_SYS_MODE_ACPI);
    } else {
    return_UINT32(ACPI_SYS_MODE_LEGACY);
    }
    }
