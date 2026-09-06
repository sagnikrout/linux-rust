//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_ctrl_lo_arch_base_addr_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DCORE0_MME_CTRL_LO_ARCH_BASE_ADDR
// (Prototype: MME_ADDRESS_DESCRIPTOR)
//
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_COUT1_LOW: c_uint = 0x40CB008;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_COUT1_HIGH: c_uint = 0x40CB00C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_COUT0_LOW: c_uint = 0x40CB010;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_COUT0_HIGH: c_uint = 0x40CB014;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_A_LOW: c_uint = 0x40CB018;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_A_HIGH: c_uint = 0x40CB01C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_B_LOW: c_uint = 0x40CB020;
pub const mmDCORE0_MME_CTRL_LO_ARCH_BASE_ADDR_B_HIGH: c_uint = 0x40CB024;
