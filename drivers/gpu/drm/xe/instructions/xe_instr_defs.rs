//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/instructions/xe_instr_defs.h
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
// Copyright © 2023 Intel Corporation
//

//
// The first dword of any GPU instruction is the "instruction header."  Bits
// 31:29 identify the general type of the command and determine how exact
// opcodes and sub-opcodes will be encoded in the remaining bits.
//

//
// Most (but not all) instructions have a "length" field in the instruction
// header.  The value expected is the total number of dwords for the
// instruction, minus two.
//
// Some instructions have length fields longer or shorter than 8 bits, but
// those are rare.  This definition can be used for the common case where
// the length field is from 7:0.
//

