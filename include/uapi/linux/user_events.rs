//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/user_events.h
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
// Copyright (c) 2021-2022, Microsoft Corporation.
//
// Authors:
// Beau Belgrave <beaub@linux.microsoft.com>
//

// Create dynamic location entry within a 32-bit value

// List of supported registration flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum user_reg_flag {
// Event will not delete upon last reference closing
    USER_EVENT_REG_PERSIST		= 1U << 0,

// Event will be allowed to have multiple formats
    USER_EVENT_REG_MULTI_FORMAT	= 1U << 1,

// This value or above is currently non-ABI
    USER_EVENT_REG_MAX		= 1U << 2,
}

//
// Describes an event registration and stores the results of the registration.
// This structure is passed to the DIAG_IOCSREG ioctl, callers at a minimum
// must set the size and name_args before invocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_reg {
// Input: Size of the user_reg structure being used
    pub size: __u32,
// Input: Bit in enable address to use
    pub enable_bit: __u8,
// Input: Enable size in bytes at address
    pub enable_size: __u8,
// Input: Flags to use, if any
    pub flags: __u16,
// Input: Address to update when enabled
    pub enable_addr: __u64,
// Input: Pointer to string with event name, description and flags
    pub name_args: __u64,
// Output: Index of the event to use when writing data
    pub write_index: __u32,
    pub __attribute__((__packed__)): },
//
// Describes an event unregister, callers must set the size, address and bit.
// This structure is passed to the DIAG_IOCSUNREG ioctl to disable bit updates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_unreg {
// Input: Size of the user_unreg structure being used
    pub size: __u32,
// Input: Bit to unregister
    pub disable_bit: __u8,
// Input: Reserved, set to 0
    pub __reserved: __u8,
// Input: Reserved, set to 0
    pub __reserved2: __u16,
// Input: Address to unregister
    pub disable_addr: __u64,
    pub __attribute__((__packed__)): },

// Request to register a user_event

// Request to delete a user_event

// Requests to unregister a user_event

