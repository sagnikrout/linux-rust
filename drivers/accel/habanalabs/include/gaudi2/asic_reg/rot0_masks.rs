//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/rot0_masks.h
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
// ROT0
// (Prototype: ROTATOR)
//
// ROT0_KMD_MODE
pub const ROT0_KMD_MODE_EN_SHIFT: c_int = 0;
pub const ROT0_KMD_MODE_EN_MASK: c_uint = 0x1;
// ROT0_CPL_QUEUE_EN
pub const ROT0_CPL_QUEUE_EN_Q_EN_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_EN_Q_EN_MASK: c_uint = 0x1;
// ROT0_CPL_QUEUE_ADDR_L
pub const ROT0_CPL_QUEUE_ADDR_L_VAL_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_ADDR_L_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_CPL_QUEUE_ADDR_H
pub const ROT0_CPL_QUEUE_ADDR_H_VAL_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_ADDR_H_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_CPL_QUEUE_DATA
pub const ROT0_CPL_QUEUE_DATA_VAL_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_DATA_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_CPL_QUEUE_AWUSER
pub const ROT0_CPL_QUEUE_AWUSER_VAL_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_AWUSER_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_CPL_QUEUE_AXI
pub const ROT0_CPL_QUEUE_AXI_CACHE_SHIFT: c_int = 0;
pub const ROT0_CPL_QUEUE_AXI_CACHE_MASK: c_uint = 0xF;
pub const ROT0_CPL_QUEUE_AXI_PROT_SHIFT: c_int = 4;
pub const ROT0_CPL_QUEUE_AXI_PROT_MASK: c_uint = 0x70;
// ROT0_CPL_MSG_THRESHOLD
pub const ROT0_CPL_MSG_THRESHOLD_VAL_SHIFT: c_int = 0;
pub const ROT0_CPL_MSG_THRESHOLD_VAL_MASK: c_uint = 0x3F;
// ROT0_CPL_MSG_AXI
pub const ROT0_CPL_MSG_AXI_CACHE_SHIFT: c_int = 0;
pub const ROT0_CPL_MSG_AXI_CACHE_MASK: c_uint = 0xF;
pub const ROT0_CPL_MSG_AXI_PROT_SHIFT: c_int = 4;
pub const ROT0_CPL_MSG_AXI_PROT_MASK: c_uint = 0x70;
// ROT0_AXI_WB
pub const ROT0_AXI_WB_CACHE_SHIFT: c_int = 0;
pub const ROT0_AXI_WB_CACHE_MASK: c_uint = 0xF;
pub const ROT0_AXI_WB_PROT_SHIFT: c_int = 4;
pub const ROT0_AXI_WB_PROT_MASK: c_uint = 0x70;
// ROT0_ERR_CFG
pub const ROT0_ERR_CFG_STOP_ON_ERR_SHIFT: c_int = 0;
pub const ROT0_ERR_CFG_STOP_ON_ERR_MASK: c_uint = 0x1;
// ROT0_ERR_STATUS
pub const ROT0_ERR_STATUS_ROT_HBW_RD_SHIFT: c_int = 0;
pub const ROT0_ERR_STATUS_ROT_HBW_RD_MASK: c_uint = 0x1;
pub const ROT0_ERR_STATUS_ROT_HBW_WR_SHIFT: c_int = 1;
pub const ROT0_ERR_STATUS_ROT_HBW_WR_MASK: c_uint = 0x2;
pub const ROT0_ERR_STATUS_QMAN_HBW_RD_SHIFT: c_int = 2;
pub const ROT0_ERR_STATUS_QMAN_HBW_RD_MASK: c_uint = 0x4;
pub const ROT0_ERR_STATUS_QMAN_HBW_WR_SHIFT: c_int = 3;
pub const ROT0_ERR_STATUS_QMAN_HBW_WR_MASK: c_uint = 0x8;
pub const ROT0_ERR_STATUS_ROT_LBW_WR_SHIFT: c_int = 4;
pub const ROT0_ERR_STATUS_ROT_LBW_WR_MASK: c_uint = 0x10;
// ROT0_WBC_MAX_OUTSTANDING
pub const ROT0_WBC_MAX_OUTSTANDING_VAL_SHIFT: c_int = 0;
pub const ROT0_WBC_MAX_OUTSTANDING_VAL_MASK: c_uint = 0xFFFF;
// ROT0_WBC_RL
pub const ROT0_WBC_RL_SATURATION_SHIFT: c_int = 0;
pub const ROT0_WBC_RL_SATURATION_MASK: c_uint = 0xFF;
pub const ROT0_WBC_RL_TIMEOUT_SHIFT: c_int = 8;
pub const ROT0_WBC_RL_TIMEOUT_MASK: c_uint = 0xFF00;
pub const ROT0_WBC_RL_RST_TOKEN_SHIFT: c_int = 16;
pub const ROT0_WBC_RL_RST_TOKEN_MASK: c_uint = 0xFF0000;
pub const ROT0_WBC_RL_RATE_LIMITER_EN_SHIFT: c_int = 24;
pub const ROT0_WBC_RL_RATE_LIMITER_EN_MASK: c_uint = 0x1000000;
// ROT0_WBC_INFLIGHTS
pub const ROT0_WBC_INFLIGHTS_VAL_SHIFT: c_int = 0;
pub const ROT0_WBC_INFLIGHTS_VAL_MASK: c_uint = 0xFFFF;
// ROT0_WBC_INFO
pub const ROT0_WBC_INFO_EMPTY_SHIFT: c_int = 0;
pub const ROT0_WBC_INFO_EMPTY_MASK: c_uint = 0x1;
pub const ROT0_WBC_INFO_AXI_IDLE_SHIFT: c_int = 1;
pub const ROT0_WBC_INFO_AXI_IDLE_MASK: c_uint = 0x2;
// ROT0_WBC_MON
pub const ROT0_WBC_MON_CNT_SHIFT: c_int = 0;
pub const ROT0_WBC_MON_CNT_MASK: c_uint = 0x1;
pub const ROT0_WBC_MON_TS_SHIFT: c_int = 8;
pub const ROT0_WBC_MON_TS_MASK: c_uint = 0x300;
pub const ROT0_WBC_MON_CONTEXT_ID_SHIFT: c_int = 16;
pub const ROT0_WBC_MON_CONTEXT_ID_MASK: c_uint = 0xFFFF0000;
// ROT0_RSB_CAM_MAX_SIZE
pub const ROT0_RSB_CAM_MAX_SIZE_DATA_SHIFT: c_int = 0;
pub const ROT0_RSB_CAM_MAX_SIZE_DATA_MASK: c_uint = 0xFFFF;
pub const ROT0_RSB_CAM_MAX_SIZE_MD_SHIFT: c_int = 16;
pub const ROT0_RSB_CAM_MAX_SIZE_MD_MASK: c_uint = 0xFFFF0000;
// ROT0_RSB_CFG
pub const ROT0_RSB_CFG_CACHE_DISABLE_SHIFT: c_int = 0;
pub const ROT0_RSB_CFG_CACHE_DISABLE_MASK: c_uint = 0x1;
pub const ROT0_RSB_CFG_ENABLE_CGATE_SHIFT: c_int = 1;
pub const ROT0_RSB_CFG_ENABLE_CGATE_MASK: c_uint = 0x2;
// ROT0_RSB_MAX_OS
pub const ROT0_RSB_MAX_OS_VAL_SHIFT: c_int = 0;
pub const ROT0_RSB_MAX_OS_VAL_MASK: c_uint = 0xFFFF;
// ROT0_RSB_RL
pub const ROT0_RSB_RL_SATURATION_SHIFT: c_int = 0;
pub const ROT0_RSB_RL_SATURATION_MASK: c_uint = 0xFF;
pub const ROT0_RSB_RL_TIMEOUT_SHIFT: c_int = 8;
pub const ROT0_RSB_RL_TIMEOUT_MASK: c_uint = 0xFF00;
pub const ROT0_RSB_RL_RST_TOKEN_SHIFT: c_int = 16;
pub const ROT0_RSB_RL_RST_TOKEN_MASK: c_uint = 0xFF0000;
pub const ROT0_RSB_RL_RATE_LIMITER_EN_SHIFT: c_int = 24;
pub const ROT0_RSB_RL_RATE_LIMITER_EN_MASK: c_uint = 0x1000000;
// ROT0_RSB_INFLIGHTS
pub const ROT0_RSB_INFLIGHTS_VAL_SHIFT: c_int = 0;
pub const ROT0_RSB_INFLIGHTS_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_RSB_OCCUPANCY
pub const ROT0_RSB_OCCUPANCY_VAL_SHIFT: c_int = 0;
pub const ROT0_RSB_OCCUPANCY_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_RSB_INFO
pub const ROT0_RSB_INFO_EMPTY_SHIFT: c_int = 0;
pub const ROT0_RSB_INFO_EMPTY_MASK: c_uint = 0x1;
pub const ROT0_RSB_INFO_AXI_IDLE_SHIFT: c_int = 1;
pub const ROT0_RSB_INFO_AXI_IDLE_MASK: c_uint = 0x2;
// ROT0_RSB_MON
pub const ROT0_RSB_MON_CNT_SHIFT: c_int = 0;
pub const ROT0_RSB_MON_CNT_MASK: c_uint = 0x1FFF;
pub const ROT0_RSB_MON_TS_SHIFT: c_int = 16;
pub const ROT0_RSB_MON_TS_MASK: c_uint = 0x30000;
// ROT0_RSB_MON_CONTEXT_ID
pub const ROT0_RSB_MON_CONTEXT_ID_VAL_SHIFT: c_int = 0;
pub const ROT0_RSB_MON_CONTEXT_ID_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_MSS_HALT
pub const ROT0_MSS_HALT_VAL_SHIFT: c_int = 0;
pub const ROT0_MSS_HALT_VAL_MASK: c_uint = 0x7;
// ROT0_MSS_SEI_STATUS
pub const ROT0_MSS_SEI_STATUS_I0_SHIFT: c_int = 0;
pub const ROT0_MSS_SEI_STATUS_I0_MASK: c_uint = 0x1;
pub const ROT0_MSS_SEI_STATUS_I1_SHIFT: c_int = 1;
pub const ROT0_MSS_SEI_STATUS_I1_MASK: c_uint = 0x2;
pub const ROT0_MSS_SEI_STATUS_I2_SHIFT: c_int = 2;
pub const ROT0_MSS_SEI_STATUS_I2_MASK: c_uint = 0x4;
pub const ROT0_MSS_SEI_STATUS_I3_SHIFT: c_int = 3;
pub const ROT0_MSS_SEI_STATUS_I3_MASK: c_uint = 0x8;
pub const ROT0_MSS_SEI_STATUS_I4_SHIFT: c_int = 4;
pub const ROT0_MSS_SEI_STATUS_I4_MASK: c_uint = 0x10;
pub const ROT0_MSS_SEI_STATUS_I5_SHIFT: c_int = 5;
pub const ROT0_MSS_SEI_STATUS_I5_MASK: c_uint = 0x20;
pub const ROT0_MSS_SEI_STATUS_I6_SHIFT: c_int = 6;
pub const ROT0_MSS_SEI_STATUS_I6_MASK: c_uint = 0x40;
pub const ROT0_MSS_SEI_STATUS_I7_SHIFT: c_int = 7;
pub const ROT0_MSS_SEI_STATUS_I7_MASK: c_uint = 0x80;
pub const ROT0_MSS_SEI_STATUS_I8_SHIFT: c_int = 8;
pub const ROT0_MSS_SEI_STATUS_I8_MASK: c_uint = 0x100;
pub const ROT0_MSS_SEI_STATUS_I9_SHIFT: c_int = 9;
pub const ROT0_MSS_SEI_STATUS_I9_MASK: c_uint = 0x200;
pub const ROT0_MSS_SEI_STATUS_I10_SHIFT: c_int = 10;
pub const ROT0_MSS_SEI_STATUS_I10_MASK: c_uint = 0x400;
pub const ROT0_MSS_SEI_STATUS_I11_SHIFT: c_int = 11;
pub const ROT0_MSS_SEI_STATUS_I11_MASK: c_uint = 0x800;
pub const ROT0_MSS_SEI_STATUS_I12_SHIFT: c_int = 12;
pub const ROT0_MSS_SEI_STATUS_I12_MASK: c_uint = 0x1000;
pub const ROT0_MSS_SEI_STATUS_I13_SHIFT: c_int = 13;
pub const ROT0_MSS_SEI_STATUS_I13_MASK: c_uint = 0x2000;
pub const ROT0_MSS_SEI_STATUS_I14_SHIFT: c_int = 14;
pub const ROT0_MSS_SEI_STATUS_I14_MASK: c_uint = 0x4000;
pub const ROT0_MSS_SEI_STATUS_I15_SHIFT: c_int = 15;
pub const ROT0_MSS_SEI_STATUS_I15_MASK: c_uint = 0x8000;
pub const ROT0_MSS_SEI_STATUS_I16_SHIFT: c_int = 16;
pub const ROT0_MSS_SEI_STATUS_I16_MASK: c_uint = 0x10000;
pub const ROT0_MSS_SEI_STATUS_I17_SHIFT: c_int = 17;
pub const ROT0_MSS_SEI_STATUS_I17_MASK: c_uint = 0x20000;
pub const ROT0_MSS_SEI_STATUS_I18_SHIFT: c_int = 18;
pub const ROT0_MSS_SEI_STATUS_I18_MASK: c_uint = 0x40000;
pub const ROT0_MSS_SEI_STATUS_I19_SHIFT: c_int = 19;
pub const ROT0_MSS_SEI_STATUS_I19_MASK: c_uint = 0x80000;
pub const ROT0_MSS_SEI_STATUS_I20_SHIFT: c_int = 20;
pub const ROT0_MSS_SEI_STATUS_I20_MASK: c_uint = 0x100000;
pub const ROT0_MSS_SEI_STATUS_I21_SHIFT: c_int = 21;
pub const ROT0_MSS_SEI_STATUS_I21_MASK: c_uint = 0x200000;
// ROT0_MSS_SEI_MASK
pub const ROT0_MSS_SEI_MASK_VAL_SHIFT: c_int = 0;
pub const ROT0_MSS_SEI_MASK_VAL_MASK: c_uint = 0x3FFFFF;
// ROT0_MSS_SPI_STATUS
pub const ROT0_MSS_SPI_STATUS_I0_SHIFT: c_int = 0;
pub const ROT0_MSS_SPI_STATUS_I0_MASK: c_uint = 0x1;
pub const ROT0_MSS_SPI_STATUS_I1_SHIFT: c_int = 1;
pub const ROT0_MSS_SPI_STATUS_I1_MASK: c_uint = 0x2;
pub const ROT0_MSS_SPI_STATUS_I2_SHIFT: c_int = 2;
pub const ROT0_MSS_SPI_STATUS_I2_MASK: c_uint = 0x4;
pub const ROT0_MSS_SPI_STATUS_I3_SHIFT: c_int = 3;
pub const ROT0_MSS_SPI_STATUS_I3_MASK: c_uint = 0x8;
pub const ROT0_MSS_SPI_STATUS_I4_SHIFT: c_int = 4;
pub const ROT0_MSS_SPI_STATUS_I4_MASK: c_uint = 0x10;
pub const ROT0_MSS_SPI_STATUS_I5_SHIFT: c_int = 5;
pub const ROT0_MSS_SPI_STATUS_I5_MASK: c_uint = 0x20;
pub const ROT0_MSS_SPI_STATUS_I6_SHIFT: c_int = 6;
pub const ROT0_MSS_SPI_STATUS_I6_MASK: c_uint = 0x40;
pub const ROT0_MSS_SPI_STATUS_I7_SHIFT: c_int = 7;
pub const ROT0_MSS_SPI_STATUS_I7_MASK: c_uint = 0x80;
// ROT0_MSS_SPI_MASK
pub const ROT0_MSS_SPI_MASK_VAL_SHIFT: c_int = 0;
pub const ROT0_MSS_SPI_MASK_VAL_MASK: c_uint = 0xFF;
// ROT0_DISABLE_PAD_CALC
pub const ROT0_DISABLE_PAD_CALC_VAL_SHIFT: c_int = 0;
pub const ROT0_DISABLE_PAD_CALC_VAL_MASK: c_uint = 0x3;
// ROT0_QMAN_CFG
pub const ROT0_QMAN_CFG_FORCE_STOP_SHIFT: c_int = 0;
pub const ROT0_QMAN_CFG_FORCE_STOP_MASK: c_uint = 0x1;
// ROT0_CLK_EN
pub const ROT0_CLK_EN_LBW_CFG_DIS_SHIFT: c_int = 0;
pub const ROT0_CLK_EN_LBW_CFG_DIS_MASK: c_uint = 0x1;
pub const ROT0_CLK_EN_DBG_CFG_DIS_SHIFT: c_int = 4;
pub const ROT0_CLK_EN_DBG_CFG_DIS_MASK: c_uint = 0x10;
pub const ROT0_CLK_EN_SB_EMPTY_MASK_SHIFT: c_int = 5;
pub const ROT0_CLK_EN_SB_EMPTY_MASK_MASK: c_uint = 0x20;
// ROT0_MRSB_CAM_MAX_SIZE
pub const ROT0_MRSB_CAM_MAX_SIZE_DATA_SHIFT: c_int = 0;
pub const ROT0_MRSB_CAM_MAX_SIZE_DATA_MASK: c_uint = 0xFFFF;
pub const ROT0_MRSB_CAM_MAX_SIZE_MD_SHIFT: c_int = 16;
pub const ROT0_MRSB_CAM_MAX_SIZE_MD_MASK: c_uint = 0xFFFF0000;
// ROT0_MRSB_CFG
pub const ROT0_MRSB_CFG_CACHE_DISABLE_SHIFT: c_int = 0;
pub const ROT0_MRSB_CFG_CACHE_DISABLE_MASK: c_uint = 0x1;
pub const ROT0_MRSB_CFG_ENABLE_CGATE_SHIFT: c_int = 1;
pub const ROT0_MRSB_CFG_ENABLE_CGATE_MASK: c_uint = 0x2;
// ROT0_MRSB_MAX_OS
pub const ROT0_MRSB_MAX_OS_VAL_SHIFT: c_int = 0;
pub const ROT0_MRSB_MAX_OS_VAL_MASK: c_uint = 0xFFFF;
// ROT0_MRSB_RL
pub const ROT0_MRSB_RL_SATURATION_SHIFT: c_int = 0;
pub const ROT0_MRSB_RL_SATURATION_MASK: c_uint = 0xFF;
pub const ROT0_MRSB_RL_TIMEOUT_SHIFT: c_int = 8;
pub const ROT0_MRSB_RL_TIMEOUT_MASK: c_uint = 0xFF00;
pub const ROT0_MRSB_RL_RST_TOKEN_SHIFT: c_int = 16;
pub const ROT0_MRSB_RL_RST_TOKEN_MASK: c_uint = 0xFF0000;
pub const ROT0_MRSB_RL_RATE_LIMITER_EN_SHIFT: c_int = 24;
pub const ROT0_MRSB_RL_RATE_LIMITER_EN_MASK: c_uint = 0x1000000;
// ROT0_MRSB_INFLIGHTS
pub const ROT0_MRSB_INFLIGHTS_VAL_SHIFT: c_int = 0;
pub const ROT0_MRSB_INFLIGHTS_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_MRSB_OCCUPANCY
pub const ROT0_MRSB_OCCUPANCY_VAL_SHIFT: c_int = 0;
pub const ROT0_MRSB_OCCUPANCY_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_MRSB_INFO
pub const ROT0_MRSB_INFO_EMPTY_SHIFT: c_int = 0;
pub const ROT0_MRSB_INFO_EMPTY_MASK: c_uint = 0x1;
pub const ROT0_MRSB_INFO_AXI_IDLE_SHIFT: c_int = 1;
pub const ROT0_MRSB_INFO_AXI_IDLE_MASK: c_uint = 0x2;
// ROT0_MRSB_MON
pub const ROT0_MRSB_MON_CNT_SHIFT: c_int = 0;
pub const ROT0_MRSB_MON_CNT_MASK: c_uint = 0x1FFF;
pub const ROT0_MRSB_MON_TS_SHIFT: c_int = 16;
pub const ROT0_MRSB_MON_TS_MASK: c_uint = 0x30000;
// ROT0_MRSB_MON_CONTEXT_ID
pub const ROT0_MRSB_MON_CONTEXT_ID_VAL_SHIFT: c_int = 0;
pub const ROT0_MRSB_MON_CONTEXT_ID_VAL_MASK: c_uint = 0xFFFFFFFF;
// ROT0_MSS_STS
pub const ROT0_MSS_STS_IS_HALT_SHIFT: c_int = 0;
pub const ROT0_MSS_STS_IS_HALT_MASK: c_uint = 0x1;
