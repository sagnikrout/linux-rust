//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/misc/amd-apml.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2021-2024 Advanced Micro Devices, Inc.
//

// Mailbox data size for data_in and data_out
pub const AMD_SBI_MB_DATA_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apml_mbox_msg {
//
// Mailbox Message ID
//
    pub cmd: __u32,
//
// [0]...[3] mailbox 32bit input/output data
//
    pub mb_in_out: __u32,
//
// Error code is returned in case of soft mailbox error
//
    pub fw_ret_code: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apml_cpuid_msg {
//
// CPUID input
// [0]...[3] cpuid func,
// [4][5] cpuid: thread
// [6] cpuid: ext function & read eax/ebx or ecx/edx
// [7:0] -> bits [7:4] -> ext function &
// bit [0] read eax/ebx or ecx/edx
// CPUID output
//
    pub cpu_in_out: __u64,
//
// Status code for CPUID read
//
    pub fw_ret_code: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apml_mcamsr_msg {
//
// MCAMSR input
// [0]...[3] mca msr func,
// [4][5] thread
// MCAMSR output
//
    pub mcamsr_in_out: __u64,
//
// Status code for MCA/MSR access
//
    pub fw_ret_code: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apml_reg_xfer_msg {
//
// RMI register address offset
//
    pub reg_addr: __u16,
//
// Register data for read/write
//
    pub data_in_out: __u8,
//
// Register read or write
//
    pub rflag: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apml_tsi_xfer_msg {
    pub /: *mut *mut __u8 reg_addr; / TSI register address offset,
    pub /: *mut *mut __u8 data_in_out; / Register data for read/write,
    pub /: *mut *mut __u8 rflag; / Register read or write,
    pub /: *mut *mut __u8 pad; / Explicit padding,
}

//
// AMD sideband interface base IOCTL
//
pub const SB_BASE_IOCTL_NR: c_uint = 0xF9;
//
// DOC: SBRMI_IOCTL_MBOX_CMD
//
// @Parameters
//
// @struct apml_mbox_msg
// Pointer to the &struct apml_mbox_msg that will contain the protocol
// information
//
// @Description
// IOCTL command for APML messages using generic _IOWR
// The IOCTL provides userspace access to AMD sideband mailbox protocol
// - Mailbox message read/write(0x0~0xFF)
// - returning "-EFAULT" if none of the above
// "-EPROTOTYPE" error is returned to provide additional error details
//

//
// DOC: SBRMI_IOCTL_CPUID_CMD
//
// @Parameters
//
// @struct apml_cpuid_msg
// Pointer to the &struct apml_cpuid_msg that will contain the protocol
// information
//
// @Description
// IOCTL command for APML messages using generic _IOWR
// The IOCTL provides userspace access to AMD sideband cpuid protocol
// - CPUID protocol to get CPU details for Function/Ext Function
// at thread level
// - returning "-EFAULT" if none of the above
// "-EPROTOTYPE" error is returned to provide additional error details
//

//
// DOC: SBRMI_IOCTL_MCAMSR_CMD
//
// @Parameters
//
// @struct apml_mcamsr_msg
// Pointer to the &struct apml_mcamsr_msg that will contain the protocol
// information
//
// @Description
// IOCTL command for APML messages using generic _IOWR
// The IOCTL provides userspace access to AMD sideband MCAMSR protocol
// - MCAMSR protocol to get MCA bank details for Function at thread level
// - returning "-EFAULT" if none of the above
// "-EPROTOTYPE" error is returned to provide additional error details
//

//
// DOC: SBRMI_IOCTL_REG_XFER_CMD
//
// @Parameters
//
// @struct apml_reg_xfer_msg
// Pointer to the &struct apml_reg_xfer_msg that will contain the protocol
// information
//
// @Description
// IOCTL command for APML messages using generic _IOWR
// The IOCTL provides userspace access to AMD sideband register xfer protocol
// - Register xfer protocol to get/set hardware register for given offset
//

//
// DOC: SBTSI_IOCTL_REG_XFER_CMD
//
// @Parameters
//
// @struct apml_tsi_xfer_msg
// Pointer to the &struct apml_tsi_xfer_msg that will contain the protocol
// information
//
// @Description
// IOCTL command for APML TSI messages using generic _IOWR
// The IOCTL provides userspace access to AMD sideband TSI register xfer protocol
// - TSI protocol to read/write temperature sensor registers
//

