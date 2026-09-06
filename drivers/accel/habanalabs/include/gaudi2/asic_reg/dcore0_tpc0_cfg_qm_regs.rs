//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_cfg_qm_regs.h
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
// DCORE0_TPC0_CFG_QM
// (Prototype: TPC_NON_TENSOR_DESCRIPTOR)
//
pub const mmDCORE0_TPC0_CFG_QM_KERNEL_BASE_ADDRESS_LOW: c_uint = 0x400BAE4;
pub const mmDCORE0_TPC0_CFG_QM_KERNEL_BASE_ADDRESS_HIGH: c_uint = 0x400BAE8;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_DIM_0: c_uint = 0x400BAEC;
pub const mmDCORE0_TPC0_CFG_QM_TID_SIZE_DIM_0: c_uint = 0x400BAF0;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_DIM_1: c_uint = 0x400BAF4;
pub const mmDCORE0_TPC0_CFG_QM_TID_SIZE_DIM_1: c_uint = 0x400BAF8;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_DIM_2: c_uint = 0x400BAFC;
pub const mmDCORE0_TPC0_CFG_QM_TID_SIZE_DIM_2: c_uint = 0x400BB00;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_DIM_3: c_uint = 0x400BB04;
pub const mmDCORE0_TPC0_CFG_QM_TID_SIZE_DIM_3: c_uint = 0x400BB08;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_DIM_4: c_uint = 0x400BB0C;
pub const mmDCORE0_TPC0_CFG_QM_TID_SIZE_DIM_4: c_uint = 0x400BB10;
pub const mmDCORE0_TPC0_CFG_QM_KERNEL_CONFIG: c_uint = 0x400BB14;
pub const mmDCORE0_TPC0_CFG_QM_KERNEL_ID: c_uint = 0x400BB18;
pub const mmDCORE0_TPC0_CFG_QM_POWER_LOOP: c_uint = 0x400BB1C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_0: c_uint = 0x400BB20;
pub const mmDCORE0_TPC0_CFG_QM_SRF_1: c_uint = 0x400BB24;
pub const mmDCORE0_TPC0_CFG_QM_SRF_2: c_uint = 0x400BB28;
pub const mmDCORE0_TPC0_CFG_QM_SRF_3: c_uint = 0x400BB2C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_4: c_uint = 0x400BB30;
pub const mmDCORE0_TPC0_CFG_QM_SRF_5: c_uint = 0x400BB34;
pub const mmDCORE0_TPC0_CFG_QM_SRF_6: c_uint = 0x400BB38;
pub const mmDCORE0_TPC0_CFG_QM_SRF_7: c_uint = 0x400BB3C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_8: c_uint = 0x400BB40;
pub const mmDCORE0_TPC0_CFG_QM_SRF_9: c_uint = 0x400BB44;
pub const mmDCORE0_TPC0_CFG_QM_SRF_10: c_uint = 0x400BB48;
pub const mmDCORE0_TPC0_CFG_QM_SRF_11: c_uint = 0x400BB4C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_12: c_uint = 0x400BB50;
pub const mmDCORE0_TPC0_CFG_QM_SRF_13: c_uint = 0x400BB54;
pub const mmDCORE0_TPC0_CFG_QM_SRF_14: c_uint = 0x400BB58;
pub const mmDCORE0_TPC0_CFG_QM_SRF_15: c_uint = 0x400BB5C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_16: c_uint = 0x400BB60;
pub const mmDCORE0_TPC0_CFG_QM_SRF_17: c_uint = 0x400BB64;
pub const mmDCORE0_TPC0_CFG_QM_SRF_18: c_uint = 0x400BB68;
pub const mmDCORE0_TPC0_CFG_QM_SRF_19: c_uint = 0x400BB6C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_20: c_uint = 0x400BB70;
pub const mmDCORE0_TPC0_CFG_QM_SRF_21: c_uint = 0x400BB74;
pub const mmDCORE0_TPC0_CFG_QM_SRF_22: c_uint = 0x400BB78;
pub const mmDCORE0_TPC0_CFG_QM_SRF_23: c_uint = 0x400BB7C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_24: c_uint = 0x400BB80;
pub const mmDCORE0_TPC0_CFG_QM_SRF_25: c_uint = 0x400BB84;
pub const mmDCORE0_TPC0_CFG_QM_SRF_26: c_uint = 0x400BB88;
pub const mmDCORE0_TPC0_CFG_QM_SRF_27: c_uint = 0x400BB8C;
pub const mmDCORE0_TPC0_CFG_QM_SRF_28: c_uint = 0x400BB90;
pub const mmDCORE0_TPC0_CFG_QM_SRF_29: c_uint = 0x400BB94;
pub const mmDCORE0_TPC0_CFG_QM_SRF_30: c_uint = 0x400BB98;
pub const mmDCORE0_TPC0_CFG_QM_SRF_31: c_uint = 0x400BB9C;
pub const mmDCORE0_TPC0_CFG_QM_KERNEL_ID_INC: c_uint = 0x400BBA0;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_SIZE_HIGH_DIM_0: c_uint = 0x400BBA4;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_SIZE_HIGH_DIM_1: c_uint = 0x400BBA8;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_SIZE_HIGH_DIM_2: c_uint = 0x400BBAC;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_SIZE_HIGH_DIM_3: c_uint = 0x400BBB0;
pub const mmDCORE0_TPC0_CFG_QM_TID_BASE_SIZE_HIGH_DIM_4: c_uint = 0x400BBB4;
