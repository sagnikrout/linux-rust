//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma0_core_masks.h
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
// DMA0_CORE (Prototype: DMA_CORE)
//
// DMA0_CORE_CFG_0
pub const DMA0_CORE_CFG_0_EN_SHIFT: c_int = 0;
pub const DMA0_CORE_CFG_0_EN_MASK: c_uint = 0x1;
// DMA0_CORE_CFG_1
pub const DMA0_CORE_CFG_1_HALT_SHIFT: c_int = 0;
pub const DMA0_CORE_CFG_1_HALT_MASK: c_uint = 0x1;
pub const DMA0_CORE_CFG_1_FLUSH_SHIFT: c_int = 1;
pub const DMA0_CORE_CFG_1_FLUSH_MASK: c_uint = 0x2;
pub const DMA0_CORE_CFG_1_SB_FORCE_MISS_SHIFT: c_int = 2;
pub const DMA0_CORE_CFG_1_SB_FORCE_MISS_MASK: c_uint = 0x4;
// DMA0_CORE_LBW_MAX_OUTSTAND
pub const DMA0_CORE_LBW_MAX_OUTSTAND_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_LBW_MAX_OUTSTAND_VAL_MASK: c_uint = 0x1F;
// DMA0_CORE_SRC_BASE_LO
pub const DMA0_CORE_SRC_BASE_LO_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_BASE_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_BASE_HI
pub const DMA0_CORE_SRC_BASE_HI_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_BASE_HI_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_BASE_LO
pub const DMA0_CORE_DST_BASE_LO_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_BASE_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_BASE_HI
pub const DMA0_CORE_DST_BASE_HI_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_BASE_HI_VAL_MASK: c_uint = 0xFFFFFF;
pub const DMA0_CORE_DST_BASE_HI_CTX_ID_HI_SHIFT: c_int = 24;
pub const DMA0_CORE_DST_BASE_HI_CTX_ID_HI_MASK: c_uint = 0xFF000000;
// DMA0_CORE_SRC_TSIZE_1
pub const DMA0_CORE_SRC_TSIZE_1_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_TSIZE_1_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_STRIDE_1
pub const DMA0_CORE_SRC_STRIDE_1_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_STRIDE_1_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_TSIZE_2
pub const DMA0_CORE_SRC_TSIZE_2_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_TSIZE_2_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_STRIDE_2
pub const DMA0_CORE_SRC_STRIDE_2_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_STRIDE_2_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_TSIZE_3
pub const DMA0_CORE_SRC_TSIZE_3_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_TSIZE_3_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_STRIDE_3
pub const DMA0_CORE_SRC_STRIDE_3_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_STRIDE_3_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_TSIZE_4
pub const DMA0_CORE_SRC_TSIZE_4_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_TSIZE_4_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_STRIDE_4
pub const DMA0_CORE_SRC_STRIDE_4_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_STRIDE_4_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_SRC_TSIZE_0
pub const DMA0_CORE_SRC_TSIZE_0_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_SRC_TSIZE_0_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_TSIZE_1
pub const DMA0_CORE_DST_TSIZE_1_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_TSIZE_1_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_STRIDE_1
pub const DMA0_CORE_DST_STRIDE_1_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_STRIDE_1_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_TSIZE_2
pub const DMA0_CORE_DST_TSIZE_2_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_TSIZE_2_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_STRIDE_2
pub const DMA0_CORE_DST_STRIDE_2_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_STRIDE_2_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_TSIZE_3
pub const DMA0_CORE_DST_TSIZE_3_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_TSIZE_3_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_STRIDE_3
pub const DMA0_CORE_DST_STRIDE_3_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_STRIDE_3_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_TSIZE_4
pub const DMA0_CORE_DST_TSIZE_4_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_TSIZE_4_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_STRIDE_4
pub const DMA0_CORE_DST_STRIDE_4_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_STRIDE_4_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DST_TSIZE_0
pub const DMA0_CORE_DST_TSIZE_0_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_DST_TSIZE_0_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_COMMIT
pub const DMA0_CORE_COMMIT_WR_COMP_EN_SHIFT: c_int = 0;
pub const DMA0_CORE_COMMIT_WR_COMP_EN_MASK: c_uint = 0x1;
pub const DMA0_CORE_COMMIT_TRANSPOSE_SHIFT: c_int = 1;
pub const DMA0_CORE_COMMIT_TRANSPOSE_MASK: c_uint = 0x2;
pub const DMA0_CORE_COMMIT_DTYPE_SHIFT: c_int = 2;
pub const DMA0_CORE_COMMIT_DTYPE_MASK: c_uint = 0x4;
pub const DMA0_CORE_COMMIT_LIN_SHIFT: c_int = 3;
pub const DMA0_CORE_COMMIT_LIN_MASK: c_uint = 0x8;
pub const DMA0_CORE_COMMIT_MEM_SET_SHIFT: c_int = 4;
pub const DMA0_CORE_COMMIT_MEM_SET_MASK: c_uint = 0x10;
pub const DMA0_CORE_COMMIT_COMPRESS_SHIFT: c_int = 5;
pub const DMA0_CORE_COMMIT_COMPRESS_MASK: c_uint = 0x20;
pub const DMA0_CORE_COMMIT_DECOMPRESS_SHIFT: c_int = 6;
pub const DMA0_CORE_COMMIT_DECOMPRESS_MASK: c_uint = 0x40;
pub const DMA0_CORE_COMMIT_CTX_ID_SHIFT: c_int = 16;
pub const DMA0_CORE_COMMIT_CTX_ID_MASK: c_uint = 0xFF0000;
// DMA0_CORE_WR_COMP_WDATA
pub const DMA0_CORE_WR_COMP_WDATA_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_COMP_WDATA_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_WR_COMP_ADDR_LO
pub const DMA0_CORE_WR_COMP_ADDR_LO_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_COMP_ADDR_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_WR_COMP_ADDR_HI
pub const DMA0_CORE_WR_COMP_ADDR_HI_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_COMP_ADDR_HI_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_WR_COMP_AWUSER_31_11
pub const DMA0_CORE_WR_COMP_AWUSER_31_11_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_COMP_AWUSER_31_11_VAL_MASK: c_uint = 0x1FFFFF;
// DMA0_CORE_TE_NUMROWS
pub const DMA0_CORE_TE_NUMROWS_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_TE_NUMROWS_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_PROT
pub const DMA0_CORE_PROT_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_PROT_VAL_MASK: c_uint = 0x1;
pub const DMA0_CORE_PROT_ERR_VAL_SHIFT: c_int = 1;
pub const DMA0_CORE_PROT_ERR_VAL_MASK: c_uint = 0x2;
// DMA0_CORE_SECURE_PROPS
pub const DMA0_CORE_SECURE_PROPS_ASID_SHIFT: c_int = 0;
pub const DMA0_CORE_SECURE_PROPS_ASID_MASK: c_uint = 0x3FF;
pub const DMA0_CORE_SECURE_PROPS_MMBP_SHIFT: c_int = 10;
pub const DMA0_CORE_SECURE_PROPS_MMBP_MASK: c_uint = 0x400;
// DMA0_CORE_NON_SECURE_PROPS
pub const DMA0_CORE_NON_SECURE_PROPS_ASID_SHIFT: c_int = 0;
pub const DMA0_CORE_NON_SECURE_PROPS_ASID_MASK: c_uint = 0x3FF;
pub const DMA0_CORE_NON_SECURE_PROPS_MMBP_SHIFT: c_int = 10;
pub const DMA0_CORE_NON_SECURE_PROPS_MMBP_MASK: c_uint = 0x400;
// DMA0_CORE_RD_MAX_OUTSTAND
pub const DMA0_CORE_RD_MAX_OUTSTAND_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_MAX_OUTSTAND_VAL_MASK: c_uint = 0xFFF;
// DMA0_CORE_RD_MAX_SIZE
pub const DMA0_CORE_RD_MAX_SIZE_DATA_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_MAX_SIZE_DATA_MASK: c_uint = 0x7FF;
pub const DMA0_CORE_RD_MAX_SIZE_MD_SHIFT: c_int = 16;
pub const DMA0_CORE_RD_MAX_SIZE_MD_MASK: c_uint = 0x7FF0000;
// DMA0_CORE_RD_ARCACHE
pub const DMA0_CORE_RD_ARCACHE_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_ARCACHE_VAL_MASK: c_uint = 0xF;
// DMA0_CORE_RD_ARUSER_31_11
pub const DMA0_CORE_RD_ARUSER_31_11_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_ARUSER_31_11_VAL_MASK: c_uint = 0x1FFFFF;
// DMA0_CORE_RD_INFLIGHTS
pub const DMA0_CORE_RD_INFLIGHTS_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_INFLIGHTS_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_WR_MAX_OUTSTAND
pub const DMA0_CORE_WR_MAX_OUTSTAND_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_MAX_OUTSTAND_VAL_MASK: c_uint = 0xFFF;
// DMA0_CORE_WR_MAX_AWID
pub const DMA0_CORE_WR_MAX_AWID_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_MAX_AWID_VAL_MASK: c_uint = 0xFFFF;
// DMA0_CORE_WR_AWCACHE
pub const DMA0_CORE_WR_AWCACHE_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_AWCACHE_VAL_MASK: c_uint = 0xF;
// DMA0_CORE_WR_AWUSER_31_11
pub const DMA0_CORE_WR_AWUSER_31_11_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_AWUSER_31_11_VAL_MASK: c_uint = 0x1FFFFF;
// DMA0_CORE_WR_INFLIGHTS
pub const DMA0_CORE_WR_INFLIGHTS_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_INFLIGHTS_VAL_MASK: c_uint = 0xFFFF;
// DMA0_CORE_RD_RATE_LIM_CFG_0
pub const DMA0_CORE_RD_RATE_LIM_CFG_0_RST_TOKEN_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_RATE_LIM_CFG_0_RST_TOKEN_MASK: c_uint = 0xFF;
pub const DMA0_CORE_RD_RATE_LIM_CFG_0_SAT_SHIFT: c_int = 16;
pub const DMA0_CORE_RD_RATE_LIM_CFG_0_SAT_MASK: c_uint = 0xFF0000;
// DMA0_CORE_RD_RATE_LIM_CFG_1
pub const DMA0_CORE_RD_RATE_LIM_CFG_1_TOUT_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_RATE_LIM_CFG_1_TOUT_MASK: c_uint = 0xFF;
pub const DMA0_CORE_RD_RATE_LIM_CFG_1_EN_SHIFT: c_int = 31;
pub const DMA0_CORE_RD_RATE_LIM_CFG_1_EN_MASK: c_uint = 0x80000000;
// DMA0_CORE_WR_RATE_LIM_CFG_0
pub const DMA0_CORE_WR_RATE_LIM_CFG_0_RST_TOKEN_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_RATE_LIM_CFG_0_RST_TOKEN_MASK: c_uint = 0xFF;
pub const DMA0_CORE_WR_RATE_LIM_CFG_0_SAT_SHIFT: c_int = 16;
pub const DMA0_CORE_WR_RATE_LIM_CFG_0_SAT_MASK: c_uint = 0xFF0000;
// DMA0_CORE_WR_RATE_LIM_CFG_1
pub const DMA0_CORE_WR_RATE_LIM_CFG_1_TOUT_SHIFT: c_int = 0;
pub const DMA0_CORE_WR_RATE_LIM_CFG_1_TOUT_MASK: c_uint = 0xFF;
pub const DMA0_CORE_WR_RATE_LIM_CFG_1_EN_SHIFT: c_int = 31;
pub const DMA0_CORE_WR_RATE_LIM_CFG_1_EN_MASK: c_uint = 0x80000000;
// DMA0_CORE_ERR_CFG
pub const DMA0_CORE_ERR_CFG_ERR_MSG_EN_SHIFT: c_int = 0;
pub const DMA0_CORE_ERR_CFG_ERR_MSG_EN_MASK: c_uint = 0x1;
pub const DMA0_CORE_ERR_CFG_STOP_ON_ERR_SHIFT: c_int = 1;
pub const DMA0_CORE_ERR_CFG_STOP_ON_ERR_MASK: c_uint = 0x2;
// DMA0_CORE_ERR_CAUSE
pub const DMA0_CORE_ERR_CAUSE_HBW_RD_ERR_SHIFT: c_int = 0;
pub const DMA0_CORE_ERR_CAUSE_HBW_RD_ERR_MASK: c_uint = 0x1;
pub const DMA0_CORE_ERR_CAUSE_HBW_WR_ERR_SHIFT: c_int = 1;
pub const DMA0_CORE_ERR_CAUSE_HBW_WR_ERR_MASK: c_uint = 0x2;
pub const DMA0_CORE_ERR_CAUSE_LBW_WR_ERR_SHIFT: c_int = 2;
pub const DMA0_CORE_ERR_CAUSE_LBW_WR_ERR_MASK: c_uint = 0x4;
pub const DMA0_CORE_ERR_CAUSE_DESC_OVF_SHIFT: c_int = 3;
pub const DMA0_CORE_ERR_CAUSE_DESC_OVF_MASK: c_uint = 0x8;
// DMA0_CORE_ERRMSG_ADDR_LO
pub const DMA0_CORE_ERRMSG_ADDR_LO_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_ERRMSG_ADDR_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_ERRMSG_ADDR_HI
pub const DMA0_CORE_ERRMSG_ADDR_HI_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_ERRMSG_ADDR_HI_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_ERRMSG_WDATA
pub const DMA0_CORE_ERRMSG_WDATA_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_ERRMSG_WDATA_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_STS0
pub const DMA0_CORE_STS0_RD_REQ_CNT_SHIFT: c_int = 0;
pub const DMA0_CORE_STS0_RD_REQ_CNT_MASK: c_uint = 0x7FFF;
pub const DMA0_CORE_STS0_WR_REQ_CNT_SHIFT: c_int = 16;
pub const DMA0_CORE_STS0_WR_REQ_CNT_MASK: c_uint = 0x7FFF0000;
pub const DMA0_CORE_STS0_BUSY_SHIFT: c_int = 31;
pub const DMA0_CORE_STS0_BUSY_MASK: c_uint = 0x80000000;
// DMA0_CORE_STS1
pub const DMA0_CORE_STS1_IS_HALT_SHIFT: c_int = 0;
pub const DMA0_CORE_STS1_IS_HALT_MASK: c_uint = 0x1;
// DMA0_CORE_RD_DBGMEM_ADD
pub const DMA0_CORE_RD_DBGMEM_ADD_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_DBGMEM_ADD_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_RD_DBGMEM_DATA_WR
pub const DMA0_CORE_RD_DBGMEM_DATA_WR_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_DBGMEM_DATA_WR_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_RD_DBGMEM_DATA_RD
pub const DMA0_CORE_RD_DBGMEM_DATA_RD_VAL_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_DBGMEM_DATA_RD_VAL_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_RD_DBGMEM_CTRL
pub const DMA0_CORE_RD_DBGMEM_CTRL_WR_NRD_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_DBGMEM_CTRL_WR_NRD_MASK: c_uint = 0x1;
// DMA0_CORE_RD_DBGMEM_RC
pub const DMA0_CORE_RD_DBGMEM_RC_VALID_SHIFT: c_int = 0;
pub const DMA0_CORE_RD_DBGMEM_RC_VALID_MASK: c_uint = 0x1;
// DMA0_CORE_DBG_HBW_AXI_AR_CNT
// DMA0_CORE_DBG_HBW_AXI_AW_CNT
// DMA0_CORE_DBG_LBW_AXI_AW_CNT
// DMA0_CORE_DBG_DESC_CNT
pub const DMA0_CORE_DBG_DESC_CNT_RD_STS_CTX_CNT_SHIFT: c_int = 0;
pub const DMA0_CORE_DBG_DESC_CNT_RD_STS_CTX_CNT_MASK: c_uint = 0xFFFFFFFF;
// DMA0_CORE_DBG_STS
pub const DMA0_CORE_DBG_STS_RD_CTX_FULL_SHIFT: c_int = 0;
pub const DMA0_CORE_DBG_STS_RD_CTX_FULL_MASK: c_uint = 0x1;
pub const DMA0_CORE_DBG_STS_WR_CTX_FULL_SHIFT: c_int = 1;
pub const DMA0_CORE_DBG_STS_WR_CTX_FULL_MASK: c_uint = 0x2;
pub const DMA0_CORE_DBG_STS_WR_COMP_FULL_SHIFT: c_int = 2;
pub const DMA0_CORE_DBG_STS_WR_COMP_FULL_MASK: c_uint = 0x4;
pub const DMA0_CORE_DBG_STS_RD_CTX_EMPTY_SHIFT: c_int = 3;
pub const DMA0_CORE_DBG_STS_RD_CTX_EMPTY_MASK: c_uint = 0x8;
pub const DMA0_CORE_DBG_STS_WR_CTX_EMPTY_SHIFT: c_int = 4;
pub const DMA0_CORE_DBG_STS_WR_CTX_EMPTY_MASK: c_uint = 0x10;
pub const DMA0_CORE_DBG_STS_WR_COMP_EMPTY_SHIFT: c_int = 5;
pub const DMA0_CORE_DBG_STS_WR_COMP_EMPTY_MASK: c_uint = 0x20;
pub const DMA0_CORE_DBG_STS_TE_EMPTY_SHIFT: c_int = 6;
pub const DMA0_CORE_DBG_STS_TE_EMPTY_MASK: c_uint = 0x40;
pub const DMA0_CORE_DBG_STS_TE_BUSY_SHIFT: c_int = 7;
pub const DMA0_CORE_DBG_STS_TE_BUSY_MASK: c_uint = 0x80;
pub const DMA0_CORE_DBG_STS_GSKT_EMPTY_SHIFT: c_int = 8;
pub const DMA0_CORE_DBG_STS_GSKT_EMPTY_MASK: c_uint = 0x100;
pub const DMA0_CORE_DBG_STS_GSKT_FULL_SHIFT: c_int = 9;
pub const DMA0_CORE_DBG_STS_GSKT_FULL_MASK: c_uint = 0x200;
pub const DMA0_CORE_DBG_STS_RDBUF_FULLNESS_SHIFT: c_int = 20;
pub const DMA0_CORE_DBG_STS_RDBUF_FULLNESS_MASK: c_uint = 0x7FF00000;
// DMA0_CORE_DBG_RD_DESC_ID
// DMA0_CORE_DBG_WR_DESC_ID
