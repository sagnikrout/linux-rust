//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/clc97b.h
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

// Macro flag: #define _clc97b_h_
// dma opcode instructions
// Macro flag: #define NVC97B_DMA

pub const NVC97B_DMA_OPCODE_METHOD: c_uint = 0x00000000;
pub const NVC97B_DMA_OPCODE_JUMP: c_uint = 0x00000001;
pub const NVC97B_DMA_OPCODE_NONINC_METHOD: c_uint = 0x00000002;
pub const NVC97B_DMA_OPCODE_SET_SUBDEVICE_MASK: c_uint = 0x00000003;

pub const NVC97B_DMA_DATA_NOP: c_uint = 0x00000000;

