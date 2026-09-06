//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_ctrl_lo_arch_non_tensor_end_regs.h
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
// DCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END
// (Prototype: MME_NON_TENSOR_DESCRIPTOR)
//
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_CONV_KERNEL_SIZE_MINUS_1: c_uint = 0x40CB280;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_CONV_LOW: c_uint = 0x40CB284;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_CONV_HIGH: c_uint = 0x40CB288;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_OUTER_LOOP: c_uint = 0x40CB28C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_NUM_ITERATIONS_MINUS_1: c_uint = 0x40CB290;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SB_REPEAT: c_uint = 0x40CB294;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_FP8_BIAS: c_uint = 0x40CB298;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_RATE_LIMITER: c_uint = 0x40CB29C;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_USER_DATA: c_uint = 0x40CB2A0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_PERF_EVT_IN: c_uint = 0x40CB2A4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_PERF_EVT_OUT: c_uint = 0x40CB2A8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_PCU: c_uint = 0x40CB2AC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SLAVE_SYNC_OBJ0_ADDR: c_uint = 0x40CB2B0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SLAVE_SYNC_OBJ1_ADDR: c_uint = 0x40CB2B4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_POWER_LOOP: c_uint = 0x40CB2B8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE0_MASTER: c_uint = 0x40CB2BC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE1_MASTER: c_uint = 0x40CB2C0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE2_MASTER: c_uint = 0x40CB2C4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE3_MASTER: c_uint = 0x40CB2C8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE0_SLAVE: c_uint = 0x40CB2CC;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE1_SLAVE: c_uint = 0x40CB2D0;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE2_SLAVE: c_uint = 0x40CB2D4;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_SPARE3_SLAVE: c_uint = 0x40CB2D8;
pub const mmDCORE0_MME_CTRL_LO_ARCH_NON_TENSOR_END_WKL_ID: c_uint = 0x40CB2DC;
