//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mucse/rnpgbe/rnpgbe_hw.h
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
// Copyright(c) 2020 - 2025 Mucse Corporation.
pub const MUCSE_N500_FWPF_CTRL_BASE: c_uint = 0x28b00;
pub const MUCSE_N500_FWPF_SHM_BASE: c_uint = 0x2d000;
pub const MUCSE_GBE_PFFW_MBX_CTRL_OFFSET: c_uint = 0x5500;
pub const MUCSE_GBE_FWPF_MBX_MASK_OFFSET: c_uint = 0x5700;
pub const MUCSE_N210_FWPF_CTRL_BASE: c_uint = 0x29400;
pub const MUCSE_N210_FWPF_SHM_BASE: c_uint = 0x2d900;
pub const RNPGBE_DMA_AXI_EN: c_uint = 0x0010;
pub const RNPGBE_MAX_QUEUES: c_int = 8;
