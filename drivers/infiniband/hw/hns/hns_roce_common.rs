//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_common.h
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


//
// Copyright (c) 2016 Hisilicon Limited.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// ((__le32 *)_ptr + (field_h) / 32) |= cpu_to_le32(             \

// ((__le32 *)_ptr + (field_h) / 32) &=                          \

// ((__le32 *)ptr + (field_h) / 32) |= cpu_to_le32(FIELD_PREP(   \

// ROCEE_REG DEFINITION
pub const ROCEE_VENDOR_ID_REG: c_uint = 0x0;
pub const ROCEE_VENDOR_PART_ID_REG: c_uint = 0x4;
pub const ROCEE_SYS_IMAGE_GUID_L_REG: c_uint = 0xC;
pub const ROCEE_SYS_IMAGE_GUID_H_REG: c_uint = 0x10;
pub const ROCEE_PORT_GID_L_0_REG: c_uint = 0x50;
pub const ROCEE_PORT_GID_ML_0_REG: c_uint = 0x54;
pub const ROCEE_PORT_GID_MH_0_REG: c_uint = 0x58;
pub const ROCEE_PORT_GID_H_0_REG: c_uint = 0x5C;
pub const ROCEE_BT_CMD_H_REG: c_uint = 0x204;
pub const ROCEE_SMAC_L_0_REG: c_uint = 0x240;
pub const ROCEE_SMAC_H_0_REG: c_uint = 0x244;
pub const ROCEE_QP1C_CFG3_0_REG: c_uint = 0x27C;
pub const ROCEE_CAEP_AEQE_CONS_IDX_REG: c_uint = 0x3AC;
pub const ROCEE_CAEP_CEQC_CONS_IDX_0_REG: c_uint = 0x3BC;
pub const ROCEE_ECC_UCERR_ALM1_REG: c_uint = 0xB38;
pub const ROCEE_ECC_UCERR_ALM2_REG: c_uint = 0xB3C;
pub const ROCEE_ECC_CERR_ALM1_REG: c_uint = 0xB44;
pub const ROCEE_ECC_CERR_ALM2_REG: c_uint = 0xB48;
pub const ROCEE_ACK_DELAY_REG: c_uint = 0x14;
pub const ROCEE_GLB_CFG_REG: c_uint = 0x18;
pub const ROCEE_DMAE_USER_CFG1_REG: c_uint = 0x40;
pub const ROCEE_DMAE_USER_CFG2_REG: c_uint = 0x44;
pub const ROCEE_DB_SQ_WL_REG: c_uint = 0x154;
pub const ROCEE_DB_OTHERS_WL_REG: c_uint = 0x158;
pub const ROCEE_RAQ_WL_REG: c_uint = 0x15C;
pub const ROCEE_WRMS_POL_TIME_INTERVAL_REG: c_uint = 0x160;
pub const ROCEE_EXT_DB_SQ_REG: c_uint = 0x164;
pub const ROCEE_EXT_DB_SQ_H_REG: c_uint = 0x168;
pub const ROCEE_EXT_DB_OTH_REG: c_uint = 0x16C;
pub const ROCEE_EXT_DB_OTH_H_REG: c_uint = 0x170;
pub const ROCEE_EXT_DB_SQ_WL_EMPTY_REG: c_uint = 0x174;
pub const ROCEE_EXT_DB_SQ_WL_REG: c_uint = 0x178;
pub const ROCEE_EXT_DB_OTHERS_WL_EMPTY_REG: c_uint = 0x17C;
pub const ROCEE_EXT_DB_OTHERS_WL_REG: c_uint = 0x180;
pub const ROCEE_EXT_RAQ_REG: c_uint = 0x184;
pub const ROCEE_EXT_RAQ_H_REG: c_uint = 0x188;
pub const ROCEE_CAEP_CE_INTERVAL_CFG_REG: c_uint = 0x190;
pub const ROCEE_CAEP_CE_BURST_NUM_CFG_REG: c_uint = 0x194;
pub const ROCEE_BT_CMD_L_REG: c_uint = 0x200;
pub const ROCEE_MB1_REG: c_uint = 0x210;
pub const ROCEE_MB6_REG: c_uint = 0x224;
pub const ROCEE_DB_SQ_L_0_REG: c_uint = 0x230;
pub const ROCEE_DB_OTHERS_L_0_REG: c_uint = 0x238;
pub const ROCEE_QP1C_CFG0_0_REG: c_uint = 0x270;
pub const ROCEE_CAEP_AEQC_AEQE_SHIFT_REG: c_uint = 0x3A0;
pub const ROCEE_CAEP_CEQC_SHIFT_0_REG: c_uint = 0x3B0;
pub const ROCEE_CAEP_CE_IRQ_MASK_0_REG: c_uint = 0x3C0;
pub const ROCEE_CAEP_CEQ_ALM_OVF_0_REG: c_uint = 0x3C4;
pub const ROCEE_CAEP_AE_MASK_REG: c_uint = 0x6C8;
pub const ROCEE_CAEP_AE_ST_REG: c_uint = 0x6CC;
pub const ROCEE_CAEP_CQE_WCMD_EMPTY: c_uint = 0x850;
pub const ROCEE_SCAEP_WR_CQE_CNT: c_uint = 0x8D0;
pub const ROCEE_ECC_UCERR_ALM0_REG: c_uint = 0xB34;
pub const ROCEE_ECC_CERR_ALM0_REG: c_uint = 0xB40;
// V2 ROCEE REG
pub const ROCEE_TX_CMQ_BASEADDR_L_REG: c_uint = 0x07000;
pub const ROCEE_TX_CMQ_BASEADDR_H_REG: c_uint = 0x07004;
pub const ROCEE_TX_CMQ_DEPTH_REG: c_uint = 0x07008;
pub const ROCEE_TX_CMQ_PI_REG: c_uint = 0x07010;
pub const ROCEE_TX_CMQ_CI_REG: c_uint = 0x07014;
pub const ROCEE_RX_CMQ_BASEADDR_L_REG: c_uint = 0x07018;
pub const ROCEE_RX_CMQ_BASEADDR_H_REG: c_uint = 0x0701c;
pub const ROCEE_RX_CMQ_DEPTH_REG: c_uint = 0x07020;
pub const ROCEE_RX_CMQ_TAIL_REG: c_uint = 0x07024;
pub const ROCEE_RX_CMQ_HEAD_REG: c_uint = 0x07028;
pub const ROCEE_VF_EQ_DB_CFG0_REG: c_uint = 0x238;
pub const ROCEE_VF_EQ_DB_CFG1_REG: c_uint = 0x23C;
pub const ROCEE_VF_ABN_INT_CFG_REG: c_uint = 0x13000;
pub const ROCEE_VF_ABN_INT_ST_REG: c_uint = 0x13004;
pub const ROCEE_VF_ABN_INT_EN_REG: c_uint = 0x13008;
pub const ROCEE_VF_EVENT_INT_EN_REG: c_uint = 0x1300c;
