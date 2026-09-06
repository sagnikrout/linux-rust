//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psp-platform-access.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_platform_access_msg {
    PSP_CMD_NONE			= 0x0,
    PSP_SFS_GET_FW_VERSIONS,
    PSP_SFS_UPDATE,
    PSP_CMD_HSTI_QUERY		= 0x14,
    PSP_I2C_REQ_BUS_CMD		= 0x64,
    PSP_DYNAMIC_BOOST_GET_NONCE,
    PSP_DYNAMIC_BOOST_SET_UID,
    PSP_DYNAMIC_BOOST_GET_PARAMETER,
    PSP_DYNAMIC_BOOST_SET_PARAMETER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_req_buffer_hdr {
    pub payload_size: u32,
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_request {
    pub header: psp_req_buffer_hdr,
    pub buf: *mut c_void,
    pub __packed: },
//
// psp_send_platform_access_msg() - Send a message to control platform features
//
// This function is intended to be used by drivers outside of ccp to communicate
// with the platform.
//
// Returns:
// 0:           success
// -%EBUSY:     mailbox in recovery or in use
// -%ENODEV:    driver not bound with PSP device
// -%ETIMEDOUT: request timed out
// -%EIO:       unknown error (see kernel log)
//
    pub req): *mut int psp_send_platform_access_msg(enum psp_platform_access_msg, struct psp_request,
//
// psp_ring_platform_doorbell() - Ring platform doorbell
//
// This function is intended to be used by drivers outside of ccp to ring the
// platform doorbell with a message.
//
// Returns:
// 0:           success
// -%EBUSY:     mailbox in recovery or in use
// -%ENODEV:    driver not bound with PSP device
// -%ETIMEDOUT: request timed out
// -%EIO:       error will be stored in result argument
//
    pub result): *mut int psp_ring_platform_doorbell(int msg, u32,
//
// psp_check_platform_access_status() - Checks whether platform features is ready
//
// This function is intended to be used by drivers outside of ccp to determine
// if platform features has initialized.
//
// Returns:
// 0          platform features is ready
// -%ENODEV   platform features is not ready or present
//
    pub psp_check_platform_access_status(void): c_int,
