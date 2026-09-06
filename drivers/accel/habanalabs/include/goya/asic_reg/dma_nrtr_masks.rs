//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_nrtr_masks.h
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
// DMA_NRTR (Prototype: IF_NRTR)
//
// DMA_NRTR_HBW_MAX_CRED
pub const DMA_NRTR_HBW_MAX_CRED_WR_RQ_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_MAX_CRED_WR_RQ_MASK: c_uint = 0x3F;
pub const DMA_NRTR_HBW_MAX_CRED_WR_RS_SHIFT: c_int = 8;
pub const DMA_NRTR_HBW_MAX_CRED_WR_RS_MASK: c_uint = 0x3F00;
pub const DMA_NRTR_HBW_MAX_CRED_RD_RQ_SHIFT: c_int = 16;
pub const DMA_NRTR_HBW_MAX_CRED_RD_RQ_MASK: c_uint = 0x3F0000;
pub const DMA_NRTR_HBW_MAX_CRED_RD_RS_SHIFT: c_int = 24;
pub const DMA_NRTR_HBW_MAX_CRED_RD_RS_MASK: c_uint = 0x3F000000;
// DMA_NRTR_LBW_MAX_CRED
pub const DMA_NRTR_LBW_MAX_CRED_WR_RQ_SHIFT: c_int = 0;
pub const DMA_NRTR_LBW_MAX_CRED_WR_RQ_MASK: c_uint = 0x3F;
pub const DMA_NRTR_LBW_MAX_CRED_WR_RS_SHIFT: c_int = 8;
pub const DMA_NRTR_LBW_MAX_CRED_WR_RS_MASK: c_uint = 0x3F00;
pub const DMA_NRTR_LBW_MAX_CRED_RD_RQ_SHIFT: c_int = 16;
pub const DMA_NRTR_LBW_MAX_CRED_RD_RQ_MASK: c_uint = 0x3F0000;
pub const DMA_NRTR_LBW_MAX_CRED_RD_RS_SHIFT: c_int = 24;
pub const DMA_NRTR_LBW_MAX_CRED_RD_RS_MASK: c_uint = 0x3F000000;
// DMA_NRTR_DBG_E_ARB
pub const DMA_NRTR_DBG_E_ARB_W_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_E_ARB_W_MASK: c_uint = 0x7;
pub const DMA_NRTR_DBG_E_ARB_S_SHIFT: c_int = 8;
pub const DMA_NRTR_DBG_E_ARB_S_MASK: c_uint = 0x700;
pub const DMA_NRTR_DBG_E_ARB_N_SHIFT: c_int = 16;
pub const DMA_NRTR_DBG_E_ARB_N_MASK: c_uint = 0x70000;
pub const DMA_NRTR_DBG_E_ARB_L_SHIFT: c_int = 24;
pub const DMA_NRTR_DBG_E_ARB_L_MASK: c_uint = 0x7000000;
// DMA_NRTR_DBG_W_ARB
pub const DMA_NRTR_DBG_W_ARB_E_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_W_ARB_E_MASK: c_uint = 0x7;
pub const DMA_NRTR_DBG_W_ARB_S_SHIFT: c_int = 8;
pub const DMA_NRTR_DBG_W_ARB_S_MASK: c_uint = 0x700;
pub const DMA_NRTR_DBG_W_ARB_N_SHIFT: c_int = 16;
pub const DMA_NRTR_DBG_W_ARB_N_MASK: c_uint = 0x70000;
pub const DMA_NRTR_DBG_W_ARB_L_SHIFT: c_int = 24;
pub const DMA_NRTR_DBG_W_ARB_L_MASK: c_uint = 0x7000000;
// DMA_NRTR_DBG_N_ARB
pub const DMA_NRTR_DBG_N_ARB_W_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_N_ARB_W_MASK: c_uint = 0x7;
pub const DMA_NRTR_DBG_N_ARB_E_SHIFT: c_int = 8;
pub const DMA_NRTR_DBG_N_ARB_E_MASK: c_uint = 0x700;
pub const DMA_NRTR_DBG_N_ARB_S_SHIFT: c_int = 16;
pub const DMA_NRTR_DBG_N_ARB_S_MASK: c_uint = 0x70000;
pub const DMA_NRTR_DBG_N_ARB_L_SHIFT: c_int = 24;
pub const DMA_NRTR_DBG_N_ARB_L_MASK: c_uint = 0x7000000;
// DMA_NRTR_DBG_S_ARB
pub const DMA_NRTR_DBG_S_ARB_W_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_S_ARB_W_MASK: c_uint = 0x7;
pub const DMA_NRTR_DBG_S_ARB_E_SHIFT: c_int = 8;
pub const DMA_NRTR_DBG_S_ARB_E_MASK: c_uint = 0x700;
pub const DMA_NRTR_DBG_S_ARB_N_SHIFT: c_int = 16;
pub const DMA_NRTR_DBG_S_ARB_N_MASK: c_uint = 0x70000;
pub const DMA_NRTR_DBG_S_ARB_L_SHIFT: c_int = 24;
pub const DMA_NRTR_DBG_S_ARB_L_MASK: c_uint = 0x7000000;
// DMA_NRTR_DBG_L_ARB
pub const DMA_NRTR_DBG_L_ARB_W_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_L_ARB_W_MASK: c_uint = 0x7;
pub const DMA_NRTR_DBG_L_ARB_E_SHIFT: c_int = 8;
pub const DMA_NRTR_DBG_L_ARB_E_MASK: c_uint = 0x700;
pub const DMA_NRTR_DBG_L_ARB_S_SHIFT: c_int = 16;
pub const DMA_NRTR_DBG_L_ARB_S_MASK: c_uint = 0x70000;
pub const DMA_NRTR_DBG_L_ARB_N_SHIFT: c_int = 24;
pub const DMA_NRTR_DBG_L_ARB_N_MASK: c_uint = 0x7000000;
// DMA_NRTR_DBG_E_ARB_MAX
pub const DMA_NRTR_DBG_E_ARB_MAX_CREDIT_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_E_ARB_MAX_CREDIT_MASK: c_uint = 0x3F;
// DMA_NRTR_DBG_W_ARB_MAX
pub const DMA_NRTR_DBG_W_ARB_MAX_CREDIT_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_W_ARB_MAX_CREDIT_MASK: c_uint = 0x3F;
// DMA_NRTR_DBG_N_ARB_MAX
pub const DMA_NRTR_DBG_N_ARB_MAX_CREDIT_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_N_ARB_MAX_CREDIT_MASK: c_uint = 0x3F;
// DMA_NRTR_DBG_S_ARB_MAX
pub const DMA_NRTR_DBG_S_ARB_MAX_CREDIT_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_S_ARB_MAX_CREDIT_MASK: c_uint = 0x3F;
// DMA_NRTR_DBG_L_ARB_MAX
pub const DMA_NRTR_DBG_L_ARB_MAX_CREDIT_SHIFT: c_int = 0;
pub const DMA_NRTR_DBG_L_ARB_MAX_CREDIT_MASK: c_uint = 0x3F;
// DMA_NRTR_SPLIT_COEF
pub const DMA_NRTR_SPLIT_COEF_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_COEF_VAL_MASK: c_uint = 0xFFFF;
// DMA_NRTR_SPLIT_CFG
pub const DMA_NRTR_SPLIT_CFG_FORCE_WAK_ORDER_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_CFG_FORCE_WAK_ORDER_MASK: c_uint = 0x1;
pub const DMA_NRTR_SPLIT_CFG_FORCE_STRONG_ORDER_SHIFT: c_int = 1;
pub const DMA_NRTR_SPLIT_CFG_FORCE_STRONG_ORDER_MASK: c_uint = 0x2;
pub const DMA_NRTR_SPLIT_CFG_DEFAULT_MESH_SHIFT: c_int = 2;
pub const DMA_NRTR_SPLIT_CFG_DEFAULT_MESH_MASK: c_uint = 0xC;
pub const DMA_NRTR_SPLIT_CFG_RD_RATE_LIM_EN_SHIFT: c_int = 4;
pub const DMA_NRTR_SPLIT_CFG_RD_RATE_LIM_EN_MASK: c_uint = 0x10;
pub const DMA_NRTR_SPLIT_CFG_WR_RATE_LIM_EN_SHIFT: c_int = 5;
pub const DMA_NRTR_SPLIT_CFG_WR_RATE_LIM_EN_MASK: c_uint = 0x20;
pub const DMA_NRTR_SPLIT_CFG_B2B_OPT_SHIFT: c_int = 6;
pub const DMA_NRTR_SPLIT_CFG_B2B_OPT_MASK: c_uint = 0x1C0;
// DMA_NRTR_SPLIT_RD_SAT
pub const DMA_NRTR_SPLIT_RD_SAT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_RD_SAT_VAL_MASK: c_uint = 0xFFFF;
// DMA_NRTR_SPLIT_RD_RST_TOKEN
pub const DMA_NRTR_SPLIT_RD_RST_TOKEN_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_RD_RST_TOKEN_VAL_MASK: c_uint = 0xFFFF;
// DMA_NRTR_SPLIT_RD_TIMEOUT
pub const DMA_NRTR_SPLIT_RD_TIMEOUT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_RD_TIMEOUT_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA_NRTR_SPLIT_WR_SAT
pub const DMA_NRTR_SPLIT_WR_SAT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_WR_SAT_VAL_MASK: c_uint = 0xFFFF;
// DMA_NRTR_WPLIT_WR_TST_TOLEN
pub const DMA_NRTR_WPLIT_WR_TST_TOLEN_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_WPLIT_WR_TST_TOLEN_VAL_MASK: c_uint = 0xFFFF;
// DMA_NRTR_SPLIT_WR_TIMEOUT
pub const DMA_NRTR_SPLIT_WR_TIMEOUT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SPLIT_WR_TIMEOUT_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA_NRTR_HBW_RANGE_HIT
pub const DMA_NRTR_HBW_RANGE_HIT_IND_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_RANGE_HIT_IND_MASK: c_uint = 0xFF;
// DMA_NRTR_HBW_RANGE_MASK_L
pub const DMA_NRTR_HBW_RANGE_MASK_L_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_RANGE_MASK_L_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA_NRTR_HBW_RANGE_MASK_H
pub const DMA_NRTR_HBW_RANGE_MASK_H_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_RANGE_MASK_H_VAL_MASK: c_uint = 0x3FFFF;
// DMA_NRTR_HBW_RANGE_BASE_L
pub const DMA_NRTR_HBW_RANGE_BASE_L_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_RANGE_BASE_L_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA_NRTR_HBW_RANGE_BASE_H
pub const DMA_NRTR_HBW_RANGE_BASE_H_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_HBW_RANGE_BASE_H_VAL_MASK: c_uint = 0x3FFFF;
// DMA_NRTR_LBW_RANGE_HIT
pub const DMA_NRTR_LBW_RANGE_HIT_IND_SHIFT: c_int = 0;
pub const DMA_NRTR_LBW_RANGE_HIT_IND_MASK: c_uint = 0xFFFF;
// DMA_NRTR_LBW_RANGE_MASK
pub const DMA_NRTR_LBW_RANGE_MASK_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_LBW_RANGE_MASK_VAL_MASK: c_uint = 0x3FFFFFF;
// DMA_NRTR_LBW_RANGE_BASE
pub const DMA_NRTR_LBW_RANGE_BASE_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_LBW_RANGE_BASE_VAL_MASK: c_uint = 0x3FFFFFF;
// DMA_NRTR_RGLTR
pub const DMA_NRTR_RGLTR_WR_EN_SHIFT: c_int = 0;
pub const DMA_NRTR_RGLTR_WR_EN_MASK: c_uint = 0x1;
pub const DMA_NRTR_RGLTR_RD_EN_SHIFT: c_int = 4;
pub const DMA_NRTR_RGLTR_RD_EN_MASK: c_uint = 0x10;
// DMA_NRTR_RGLTR_WR_RESULT
pub const DMA_NRTR_RGLTR_WR_RESULT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_RGLTR_WR_RESULT_VAL_MASK: c_uint = 0xFF;
// DMA_NRTR_RGLTR_RD_RESULT
pub const DMA_NRTR_RGLTR_RD_RESULT_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_RGLTR_RD_RESULT_VAL_MASK: c_uint = 0xFF;
// DMA_NRTR_SCRAMB_EN
pub const DMA_NRTR_SCRAMB_EN_VAL_SHIFT: c_int = 0;
pub const DMA_NRTR_SCRAMB_EN_VAL_MASK: c_uint = 0x1;
// DMA_NRTR_NON_LIN_SCRAMB
pub const DMA_NRTR_NON_LIN_SCRAMB_EN_SHIFT: c_int = 0;
pub const DMA_NRTR_NON_LIN_SCRAMB_EN_MASK: c_uint = 0x1;
