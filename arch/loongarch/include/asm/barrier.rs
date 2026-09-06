//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/barrier.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Hint encoding:
//
// Bit4: ordering or completion (0: completion, 1: ordering)
// Bit3: barrier for previous read (0: true, 1: false)
// Bit2: barrier for previous write (0: true, 1: false)
// Bit1: barrier for succeeding read (0: true, 1: false)
// Bit0: barrier for succeeding write (0: true, 1: false)
//
// Hint 0x700: barrier for "read after read" from the same address
//

//
// array_index_mask_nospec() - generate a ~0 mask when index < size, 0 otherwise
// @index: array element index
// @size: number of elements in array
//
// Returns:
// 0 - (@index < @size)
//

// (volatile __u8 *)&p = *(__u8 *)__u.__c;			\
// (volatile __u16 *)&p = *(__u16 *)__u.__c;			\

