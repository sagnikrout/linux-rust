//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/xe_sysctrl_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2026 Intel Corporation
//

//
// DOC: System Controller ABI
//
// This header defines the Application Binary Interface (ABI) used by
// drm/xe to communicate with System Controller firmware on Intel Xe3p
// discrete GPU platforms.
//
// System Controller (sysctrl) is a firmware-managed entity on Intel
// dGPUs responsible for certain low-level platform management
// functions.
//
// Communication protocol:
//
// Communication uses a mailbox interface with messages composed of:
//
// - Application message header (struct xe_sysctrl_app_msg_hdr)
// containing group_id, command, and version
// - Variable-length, command-specific payload
//
// Message header format:
//
// The 32-bit application message header is packed as:
//
// - Bits [7:0]   : Group ID identifying command group
// - Bits [15:8]  : Command identifier within group
// - Bits [23:16] : Command version for interface compatibility
// - Bits [31:24] : Reserved, must be zero
//
// This header defines firmware ABI message formats and constants shared
// between driver and System Controller firmware.
//
// struct xe_sysctrl_app_msg_hdr - Application layer message header
// @data: 32-bit header data
//
// Header structure for application-level messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sysctrl_app_msg_hdr {
    pub data: u32,
    pub __packed: },

pub const SYSCTRL_HDR_COMMAND_MAX: c_uint = 0x7f;

