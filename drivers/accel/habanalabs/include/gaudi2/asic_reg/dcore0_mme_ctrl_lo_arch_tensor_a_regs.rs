//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_ctrl_lo_arch_tensor_a_regs.h
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
// DCORE0_MME_CTRL_LO_ARCH_TENSOR_A
// (Prototype: MME_TENSOR)
//
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_VALID_ELEMENTS_0: c_uint = 0x40CB040;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_VALID_ELEMENTS_1: c_uint = 0x40CB044;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_VALID_ELEMENTS_2: c_uint = 0x40CB048;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_VALID_ELEMENTS_3: c_uint = 0x40CB04C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_VALID_ELEMENTS_4: c_uint = 0x40CB050;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_LOOP_STRIDE_0: c_uint = 0x40CB054;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_LOOP_STRIDE_1: c_uint = 0x40CB058;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_LOOP_STRIDE_2: c_uint = 0x40CB05C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_LOOP_STRIDE_3: c_uint = 0x40CB060;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_LOOP_STRIDE_4: c_uint = 0x40CB064;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_ROI_SIZE_0: c_uint = 0x40CB068;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_ROI_SIZE_1: c_uint = 0x40CB06C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_ROI_SIZE_2: c_uint = 0x40CB070;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_ROI_SIZE_3: c_uint = 0x40CB074;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_SPATIAL_STRIDES_0: c_uint = 0x40CB078;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_SPATIAL_STRIDES_1: c_uint = 0x40CB07C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_SPATIAL_STRIDES_2: c_uint = 0x40CB080;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_SPATIAL_STRIDES_3: c_uint = 0x40CB084;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_START_OFFSET_0: c_uint = 0x40CB088;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_START_OFFSET_1: c_uint = 0x40CB08C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_START_OFFSET_2: c_uint = 0x40CB090;
pub const mmDCORE0_MME_CTRL_LO_ARCH_TENSOR_A_START_OFFSET_3: c_uint = 0x40CB094;
