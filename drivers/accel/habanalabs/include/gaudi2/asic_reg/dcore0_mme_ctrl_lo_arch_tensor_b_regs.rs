//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_ctrl_lo_arch_tensor_b_regs.h
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
// DCORE0_MME_CTRL_LO_ARCH_TENSOR_B
// (Prototype: MME_TENSOR)
//
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_VALID_ELEMENTS_0: c_uint = 0x40CB098;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_VALID_ELEMENTS_1: c_uint = 0x40CB09C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_VALID_ELEMENTS_2: c_uint = 0x40CB0A0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_VALID_ELEMENTS_3: c_uint = 0x40CB0A4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_VALID_ELEMENTS_4: c_uint = 0x40CB0A8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_LOOP_STRIDE_0: c_uint = 0x40CB0AC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_LOOP_STRIDE_1: c_uint = 0x40CB0B0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_LOOP_STRIDE_2: c_uint = 0x40CB0B4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_LOOP_STRIDE_3: c_uint = 0x40CB0B8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_LOOP_STRIDE_4: c_uint = 0x40CB0BC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_ROI_SIZE_0: c_uint = 0x40CB0C0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_ROI_SIZE_1: c_uint = 0x40CB0C4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_ROI_SIZE_2: c_uint = 0x40CB0C8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_ROI_SIZE_3: c_uint = 0x40CB0CC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_SPATIAL_STRIDES_0: c_uint = 0x40CB0D0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_SPATIAL_STRIDES_1: c_uint = 0x40CB0D4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_SPATIAL_STRIDES_2: c_uint = 0x40CB0D8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_SPATIAL_STRIDES_3: c_uint = 0x40CB0DC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_START_OFFSET_0: c_uint = 0x40CB0E0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_START_OFFSET_1: c_uint = 0x40CB0E4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_START_OFFSET_2: c_uint = 0x40CB0E8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_B_START_OFFSET_3: c_uint = 0x40CB0EC;
