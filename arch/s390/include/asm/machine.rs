//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/machine.h
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
// Copyright IBM Corp. 2024
//

pub const MFEATURE_LOWCORE: c_int = 0;
pub const MFEATURE_PCI_MIO: c_int = 1;
pub const MFEATURE_SCC: c_int = 2;
pub const MFEATURE_TLB_GUEST: c_int = 3;
pub const MFEATURE_TX: c_int = 4;
pub const MFEATURE_ESOP: c_int = 5;
pub const MFEATURE_DIAG9C: c_int = 6;
pub const MFEATURE_VM: c_int = 7;
pub const MFEATURE_KVM: c_int = 8;
pub const MFEATURE_LPAR: c_int = 9;
pub const MFEATURE_DIAG288: c_int = 10;

extern "C" {
    pub fn test_bit(_arg: nr, _arg: mfeatures) -> return;
}
extern "C" {
    pub fn __test_machine_feature(_arg: nr, _arg: machine_features) -> return;
}

