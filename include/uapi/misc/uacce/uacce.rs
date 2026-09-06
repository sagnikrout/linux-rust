//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/misc/uacce/uacce.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

//
// UACCE_CMD_START_Q: Start queue
//

//
// UACCE_CMD_PUT_Q:
// User actively stop queue and free queue resource immediately
// Optimization method since close fd may delay
//

//
// UACCE Device flags:
// UACCE_DEV_SVA: Shared Virtual Addresses
// Support PASID
// Support device page faults (PCI PRI or SMMU Stall)
//

//
// enum uacce_qfrt: queue file region type
// @UACCE_QFRT_MMIO: device mmio region
// @UACCE_QFRT_DUS: device user share region
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uacce_qfrt {
    UACCE_QFRT_MMIO = 0,
    UACCE_QFRT_DUS = 1,
}
