//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/zynqmp-ipi-message.h
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


// SPDX-License-Identifier: GPL-2.0
//
// struct zynqmp_ipi_message - ZynqMP IPI message structure
// @len:  Length of message
// @data: message payload
//
// This is the structure for data used in mbox_send_message
// the maximum length of data buffer is fixed to 32 bytes.
// Client is supposed to be aware of this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_ipi_message {
    pub len: usize,
    pub data: [u8; ],
}
