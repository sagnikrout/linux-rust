//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_cfg_kernel_regs.h
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
// DCORE0_TPC0_CFG_KERNEL
// (Prototype: TPC_NON_TENSOR_DESCRIPTOR)
//
pub const mmDCORE0_TPC0_CFG_KERNEL_KERNEL_BASE_ADDRESS_LOW: c_uint = 0x400B508;
pub const mmDCORE0_TPC0_CFG_KERNEL_KERNEL_BASE_ADDRESS_HIGH: c_uint = 0x400B50C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_DIM_0: c_uint = 0x400B510;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_SIZE_DIM_0: c_uint = 0x400B514;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_DIM_1: c_uint = 0x400B518;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_SIZE_DIM_1: c_uint = 0x400B51C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_DIM_2: c_uint = 0x400B520;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_SIZE_DIM_2: c_uint = 0x400B524;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_DIM_3: c_uint = 0x400B528;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_SIZE_DIM_3: c_uint = 0x400B52C;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_DIM_4: c_uint = 0x400B530;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_SIZE_DIM_4: c_uint = 0x400B534;
pub const mmDCORE0_TPC0_CFG_KERNEL_KERNEL_CONFIG: c_uint = 0x400B538;
pub const mmDCORE0_TPC0_CFG_KERNEL_KERNEL_ID: c_uint = 0x400B53C;
pub const mmDCORE0_TPC0_CFG_KERNEL_POWER_LOOP: c_uint = 0x400B540;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_0: c_uint = 0x400B544;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_1: c_uint = 0x400B548;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_2: c_uint = 0x400B54C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_3: c_uint = 0x400B550;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_4: c_uint = 0x400B554;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_5: c_uint = 0x400B558;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_6: c_uint = 0x400B55C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_7: c_uint = 0x400B560;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_8: c_uint = 0x400B564;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_9: c_uint = 0x400B568;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_10: c_uint = 0x400B56C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_11: c_uint = 0x400B570;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_12: c_uint = 0x400B574;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_13: c_uint = 0x400B578;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_14: c_uint = 0x400B57C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_15: c_uint = 0x400B580;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_16: c_uint = 0x400B584;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_17: c_uint = 0x400B588;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_18: c_uint = 0x400B58C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_19: c_uint = 0x400B590;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_20: c_uint = 0x400B594;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_21: c_uint = 0x400B598;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_22: c_uint = 0x400B59C;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_23: c_uint = 0x400B5A0;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_24: c_uint = 0x400B5A4;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_25: c_uint = 0x400B5A8;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_26: c_uint = 0x400B5AC;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_27: c_uint = 0x400B5B0;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_28: c_uint = 0x400B5B4;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_29: c_uint = 0x400B5B8;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_30: c_uint = 0x400B5BC;
pub const mmDCORE0_TPC0_CFG_KERNEL_SRF_31: c_uint = 0x400B5C0;
pub const mmDCORE0_TPC0_CFG_KERNEL_KERNEL_ID_INC: c_uint = 0x400B5C4;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_SIZE_HIGH_DIM_0: c_uint = 0x400B5C8;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_SIZE_HIGH_DIM_1: c_uint = 0x400B5CC;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_SIZE_HIGH_DIM_2: c_uint = 0x400B5D0;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_SIZE_HIGH_DIM_3: c_uint = 0x400B5D4;
pub const mmDCORE0_TPC0_CFG_KERNEL_TID_BASE_SIZE_HIGH_DIM_4: c_uint = 0x400B5D8;
