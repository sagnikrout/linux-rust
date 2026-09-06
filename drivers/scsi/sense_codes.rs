//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sense_codes.h
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
// The canonical list of T10 Additional Sense Codes is available at:
// http://www.t10.org/lists/asc-num.txt [most recent: 20200817]
//
// SENSE_CODE(0x40NN, "Ram failure")
// SENSE_CODE(0x40NN, "Diagnostic failure on component nn")
// SENSE_CODE(0x41NN, "Data path failure")
// SENSE_CODE(0x42NN, "Power-on or self-test failure")
//
// SENSE_CODE(0x4DNN, "Tagged overlapped commands (nn = queue tag)")
//
// SENSE_CODE(0x70NN, "Decompression exception short algorithm id of nn")
//
