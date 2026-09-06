//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/pci_nrtr_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PCI_NRTR (Prototype: IF_NRTR)
//
pub const mmPCI_NRTR_HBW_MAX_CRED: c_uint = 0x100;
pub const mmPCI_NRTR_LBW_MAX_CRED: c_uint = 0x120;
pub const mmPCI_NRTR_DBG_E_ARB: c_uint = 0x300;
pub const mmPCI_NRTR_DBG_W_ARB: c_uint = 0x304;
pub const mmPCI_NRTR_DBG_N_ARB: c_uint = 0x308;
pub const mmPCI_NRTR_DBG_S_ARB: c_uint = 0x30C;
pub const mmPCI_NRTR_DBG_L_ARB: c_uint = 0x310;
pub const mmPCI_NRTR_DBG_E_ARB_MAX: c_uint = 0x320;
pub const mmPCI_NRTR_DBG_W_ARB_MAX: c_uint = 0x324;
pub const mmPCI_NRTR_DBG_N_ARB_MAX: c_uint = 0x328;
pub const mmPCI_NRTR_DBG_S_ARB_MAX: c_uint = 0x32C;
pub const mmPCI_NRTR_DBG_L_ARB_MAX: c_uint = 0x330;
pub const mmPCI_NRTR_SPLIT_COEF_0: c_uint = 0x400;
pub const mmPCI_NRTR_SPLIT_COEF_1: c_uint = 0x404;
pub const mmPCI_NRTR_SPLIT_COEF_2: c_uint = 0x408;
pub const mmPCI_NRTR_SPLIT_COEF_3: c_uint = 0x40C;
pub const mmPCI_NRTR_SPLIT_COEF_4: c_uint = 0x410;
pub const mmPCI_NRTR_SPLIT_COEF_5: c_uint = 0x414;
pub const mmPCI_NRTR_SPLIT_COEF_6: c_uint = 0x418;
pub const mmPCI_NRTR_SPLIT_COEF_7: c_uint = 0x41C;
pub const mmPCI_NRTR_SPLIT_COEF_8: c_uint = 0x420;
pub const mmPCI_NRTR_SPLIT_COEF_9: c_uint = 0x424;
pub const mmPCI_NRTR_SPLIT_CFG: c_uint = 0x440;
pub const mmPCI_NRTR_SPLIT_RD_SAT: c_uint = 0x444;
pub const mmPCI_NRTR_SPLIT_RD_RST_TOKEN: c_uint = 0x448;
pub const mmPCI_NRTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x44C;
pub const mmPCI_NRTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x450;
pub const mmPCI_NRTR_SPLIT_WR_SAT: c_uint = 0x454;
pub const mmPCI_NRTR_WPLIT_WR_TST_TOLEN: c_uint = 0x458;
pub const mmPCI_NRTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x45C;
pub const mmPCI_NRTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x460;
pub const mmPCI_NRTR_HBW_RANGE_HIT: c_uint = 0x470;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_0: c_uint = 0x480;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_1: c_uint = 0x484;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_2: c_uint = 0x488;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_3: c_uint = 0x48C;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_4: c_uint = 0x490;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_5: c_uint = 0x494;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_6: c_uint = 0x498;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_7: c_uint = 0x49C;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_0: c_uint = 0x4A0;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_1: c_uint = 0x4A4;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_2: c_uint = 0x4A8;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_3: c_uint = 0x4AC;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_4: c_uint = 0x4B0;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_5: c_uint = 0x4B4;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_6: c_uint = 0x4B8;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_7: c_uint = 0x4BC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_0: c_uint = 0x4C0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_1: c_uint = 0x4C4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_2: c_uint = 0x4C8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_3: c_uint = 0x4CC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_4: c_uint = 0x4D0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_5: c_uint = 0x4D4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_6: c_uint = 0x4D8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_7: c_uint = 0x4DC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_0: c_uint = 0x4E0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_1: c_uint = 0x4E4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_2: c_uint = 0x4E8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_3: c_uint = 0x4EC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_4: c_uint = 0x4F0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_5: c_uint = 0x4F4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_6: c_uint = 0x4F8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_7: c_uint = 0x4FC;
pub const mmPCI_NRTR_LBW_RANGE_HIT: c_uint = 0x500;
pub const mmPCI_NRTR_LBW_RANGE_MASK_0: c_uint = 0x510;
pub const mmPCI_NRTR_LBW_RANGE_MASK_1: c_uint = 0x514;
pub const mmPCI_NRTR_LBW_RANGE_MASK_2: c_uint = 0x518;
pub const mmPCI_NRTR_LBW_RANGE_MASK_3: c_uint = 0x51C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_4: c_uint = 0x520;
pub const mmPCI_NRTR_LBW_RANGE_MASK_5: c_uint = 0x524;
pub const mmPCI_NRTR_LBW_RANGE_MASK_6: c_uint = 0x528;
pub const mmPCI_NRTR_LBW_RANGE_MASK_7: c_uint = 0x52C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_8: c_uint = 0x530;
pub const mmPCI_NRTR_LBW_RANGE_MASK_9: c_uint = 0x534;
pub const mmPCI_NRTR_LBW_RANGE_MASK_10: c_uint = 0x538;
pub const mmPCI_NRTR_LBW_RANGE_MASK_11: c_uint = 0x53C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_12: c_uint = 0x540;
pub const mmPCI_NRTR_LBW_RANGE_MASK_13: c_uint = 0x544;
pub const mmPCI_NRTR_LBW_RANGE_MASK_14: c_uint = 0x548;
pub const mmPCI_NRTR_LBW_RANGE_MASK_15: c_uint = 0x54C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_0: c_uint = 0x550;
pub const mmPCI_NRTR_LBW_RANGE_BASE_1: c_uint = 0x554;
pub const mmPCI_NRTR_LBW_RANGE_BASE_2: c_uint = 0x558;
pub const mmPCI_NRTR_LBW_RANGE_BASE_3: c_uint = 0x55C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_4: c_uint = 0x560;
pub const mmPCI_NRTR_LBW_RANGE_BASE_5: c_uint = 0x564;
pub const mmPCI_NRTR_LBW_RANGE_BASE_6: c_uint = 0x568;
pub const mmPCI_NRTR_LBW_RANGE_BASE_7: c_uint = 0x56C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_8: c_uint = 0x570;
pub const mmPCI_NRTR_LBW_RANGE_BASE_9: c_uint = 0x574;
pub const mmPCI_NRTR_LBW_RANGE_BASE_10: c_uint = 0x578;
pub const mmPCI_NRTR_LBW_RANGE_BASE_11: c_uint = 0x57C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_12: c_uint = 0x580;
pub const mmPCI_NRTR_LBW_RANGE_BASE_13: c_uint = 0x584;
pub const mmPCI_NRTR_LBW_RANGE_BASE_14: c_uint = 0x588;
pub const mmPCI_NRTR_LBW_RANGE_BASE_15: c_uint = 0x58C;
pub const mmPCI_NRTR_RGLTR: c_uint = 0x590;
pub const mmPCI_NRTR_RGLTR_WR_RESULT: c_uint = 0x594;
pub const mmPCI_NRTR_RGLTR_RD_RESULT: c_uint = 0x598;
pub const mmPCI_NRTR_SCRAMB_EN: c_uint = 0x600;
pub const mmPCI_NRTR_NON_LIN_SCRAMB: c_uint = 0x604;
