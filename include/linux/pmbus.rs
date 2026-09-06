//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pmbus.h
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
// Hardware monitoring driver for PMBus devices
//
// Copyright (c) 2010, 2011 Ericsson AB.
//

// flags
//
// PMBUS_SKIP_STATUS_CHECK
//
// During register detection, skip checking the status register for
// communication or command errors.
//
// Some PMBus chips respond with valid data when trying to read an unsupported
// register. For such chips, checking the status register is mandatory when
// trying to determine if a chip register exists or not.
// Other PMBus chips don't support the STATUS_CML register, or report
// communication errors for no explicable reason. For such chips, checking
// the status register must be disabled.
//

//
// PMBUS_WRITE_PROTECTED
// Set if the chip is write protected and write protection is not determined
// by the standard WRITE_PROTECT command.
//

//
// PMBUS_NO_CAPABILITY
//
// Some PMBus chips don't respond with valid data when reading the CAPABILITY
// register. For such chips, this flag should be set so that the PMBus core
// driver doesn't use CAPABILITY to determine it's behavior.
//

//
// PMBUS_READ_STATUS_AFTER_FAILED_CHECK
//
// Some PMBus chips end up in an undefined state when trying to read an
// unsupported register. For such chips, it is necessary to reset the
// chip pmbus controller to a known state after a failed register check.
// This can be done by reading a known register. By setting this flag the
// driver will try to read the STATUS register after each failed
// register check. This read may fail, but it will put the chip in a
// known state.
//

//
// PMBUS_NO_WRITE_PROTECT
//
// Some PMBus chips respond with invalid data when reading the WRITE_PROTECT
// register. For such chips, this flag should be set so that the PMBus core
// driver doesn't use the WRITE_PROTECT command to determine its behavior.
//

//
// PMBUS_USE_COEFFICIENTS_CMD
//
// When this flag is set the PMBus core driver will use the COEFFICIENTS
// register to initialize the coefficients for the direct mode format.
//

//
// PMBUS_OP_PROTECTED
// Set if the chip OPERATION command is protected and protection is not
// determined by the standard WRITE_PROTECT command.
//

//
// PMBUS_VOUT_PROTECTED
// Set if the chip VOUT_COMMAND command is protected and protection is not
// determined by the standard WRITE_PROTECT command.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmbus_platform_data {
    pub /: *mut *mut u32 flags; / Device specific flags,
// regulator support
    pub num_regulators: c_int,
    pub reg_init_data: *mut regulator_init_data,
}
