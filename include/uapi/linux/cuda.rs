//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cuda.h
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
// Definitions for talking to the CUDA.  The CUDA is a microcontroller
// which controls the ADB, system power, RTC, and various other things.
//
// Copyright (C) 1996 Paul Mackerras.
//
// CUDA commands (2nd byte)
pub const CUDA_WARM_START: c_int = 0;
pub const CUDA_AUTOPOLL: c_int = 1;
pub const CUDA_GET_6805_ADDR: c_int = 2;
pub const CUDA_GET_TIME: c_int = 3;
pub const CUDA_GET_PRAM: c_int = 7;
pub const CUDA_SET_6805_ADDR: c_int = 8;
pub const CUDA_SET_TIME: c_int = 9;
pub const CUDA_POWERDOWN: c_uint = 0xa;
pub const CUDA_POWERUP_TIME: c_uint = 0xb;
pub const CUDA_SET_PRAM: c_uint = 0xc;
pub const CUDA_MS_RESET: c_uint = 0xd;
pub const CUDA_SEND_DFAC: c_uint = 0xe;
pub const CUDA_RESET_SYSTEM: c_uint = 0x11;
pub const CUDA_SET_IPL: c_uint = 0x12;
pub const CUDA_SET_AUTO_RATE: c_uint = 0x14;
pub const CUDA_GET_AUTO_RATE: c_uint = 0x16;
pub const CUDA_SET_DEVICE_LIST: c_uint = 0x19;
pub const CUDA_GET_DEVICE_LIST: c_uint = 0x1a;
pub const CUDA_GET_SET_IIC: c_uint = 0x22;
