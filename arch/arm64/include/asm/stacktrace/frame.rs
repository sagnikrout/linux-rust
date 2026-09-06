//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/stacktrace/frame.h
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
//
// - FRAME_META_TYPE_NONE
//
// This value is reserved.
//
// - FRAME_META_TYPE_FINAL
//
// The record is the last entry on the stack.
// Unwinding should terminate successfully.
//
// - FRAME_META_TYPE_PT_REGS
//
// The record is embedded within a struct pt_regs, recording the registers at
// an arbitrary point in time.
// Unwinding should consume pt_regs::pc, followed by pt_regs::lr.
//
// Note: all other values are reserved and should result in unwinding
// terminating with an error.
//
pub const FRAME_META_TYPE_NONE: c_int = 0;
pub const FRAME_META_TYPE_FINAL: c_int = 1;
pub const FRAME_META_TYPE_PT_REGS: c_int = 2;
//
// A standard AAPCS64 frame record.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_record {
    pub fp: u64,
    pub lr: u64,
}

//
// A metadata frame record indicating a special unwind.
// The record::{fp,lr} fields must be zero to indicate the presence of
// metadata.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_record_meta {
    pub record: frame_record,
    pub type: u64,
}

