//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_lli_defs.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).

// Max DLLI size
// AKA CC_DSCRPTR_QUEUE_WORD1_DIN_SIZE_BIT_SIZE
//
pub const DLLI_SIZE_BIT_SIZE: c_uint = 0x18;
pub const CC_MAX_MLLI_ENTRY_SIZE: c_uint = 0xFFFF;
pub const LLI_MAX_NUM_OF_DATA_ENTRIES: c_int = 128;
pub const LLI_MAX_NUM_OF_ASSOC_DATA_ENTRIES: c_int = 8;

pub const MAX_NUM_OF_BUFFERS_IN_MLLI: c_int = 4;

// Size of entry
pub const LLI_ENTRY_WORD_SIZE: c_int = 2;

// Word0[31:0] = ADDR[31:0]
pub const LLI_WORD0_OFFSET: c_int = 0;
pub const LLI_LADDR_BIT_OFFSET: c_int = 0;
pub const LLI_LADDR_BIT_SIZE: c_int = 32;
// Word1[31:16] = ADDR[47:32]; Word1[15:0] = SIZE
pub const LLI_WORD1_OFFSET: c_int = 1;
pub const LLI_SIZE_BIT_OFFSET: c_int = 0;
pub const LLI_SIZE_BIT_SIZE: c_int = 16;
pub const LLI_HADDR_BIT_OFFSET: c_int = 16;
pub const LLI_HADDR_BIT_SIZE: c_int = 16;

