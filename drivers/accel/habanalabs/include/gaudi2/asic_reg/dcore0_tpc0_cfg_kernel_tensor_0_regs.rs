//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_cfg_kernel_tensor_0_regs.h
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
// DCORE0_TPC0_CFG_KERNEL_TENSOR_0
// (Prototype: TPC_TENSOR)
//
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_BASE_ADDR_LOW: c_uint = 0x400B000;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_BASE_ADDR_HIGH: c_uint = 0x400B004;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_PADDING_VALUE: c_uint = 0x400B008;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_TENSOR_CONFIG: c_uint = 0x400B00C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_0_SIZE: c_uint = 0x400B010;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_0_STRIDE: c_uint = 0x400B014;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_1_SIZE: c_uint = 0x400B018;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_1_STRIDE: c_uint = 0x400B01C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_2_SIZE: c_uint = 0x400B020;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_2_STRIDE: c_uint = 0x400B024;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_3_SIZE: c_uint = 0x400B028;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_3_STRIDE: c_uint = 0x400B02C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_4_SIZE: c_uint = 0x400B030;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_4_STRIDE: c_uint = 0x400B034;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_PREF_STRIDE: c_uint = 0x400B038;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_0_SIZE_STRIDE_HIGH: c_uint = 0x400B03C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_1_SIZE_STRIDE_HIGH: c_uint = 0x400B040;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_2_SIZE_STRIDE_HIGH: c_uint = 0x400B044;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_3_SIZE_STRIDE_HIGH: c_uint = 0x400B048;
pub const mmDCORE0_TPC0_CFG_KERNEL_TENSOR_0_DIM_4_SIZE_STRIDE_HIGH: c_uint = 0x400B04C;
