//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/icl_dsi_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

// Gen11 DSI

pub const _ICL_DSI_ESC_CLK_DIV0: c_uint = 0x6b090;
pub const _ICL_DSI_ESC_CLK_DIV1: c_uint = 0x6b890;

pub const _ICL_DPHY_ESC_CLK_DIV0: c_uint = 0x162190;
pub const _ICL_DPHY_ESC_CLK_DIV1: c_uint = 0x6C190;

pub const ICL_BYTE_CLK_PER_ESC_CLK_SHIFT: c_int = 16;
pub const ICL_ESC_CLK_DIV_MASK: c_uint = 0x1ff;
pub const ICL_ESC_CLK_DIV_SHIFT: c_int = 0;

pub const _ADL_MIPIO_REG: c_uint = 0x180;

pub const _DSI_CMD_FRMCTL_0: c_uint = 0x6b034;
pub const _DSI_CMD_FRMCTL_1: c_uint = 0x6b834;

pub const _DSI_INTR_MASK_REG_0: c_uint = 0x6b070;
pub const _DSI_INTR_MASK_REG_1: c_uint = 0x6b870;

pub const _DSI_INTR_IDENT_REG_0: c_uint = 0x6b074;
pub const _DSI_INTR_IDENT_REG_1: c_uint = 0x6b874;

// ICL DSI MODE control
pub const _ICL_DSI_IO_MODECTL_0: c_uint = 0x6B094;
pub const _ICL_DSI_IO_MODECTL_1: c_uint = 0x6B894;

// TGL DSI Chicken register
pub const _TGL_DSI_CHKN_REG_0: c_uint = 0x6B0C0;
pub const _TGL_DSI_CHKN_REG_1: c_uint = 0x6B8C0;

pub const _ICL_DSI_T_INIT_MASTER_0: c_uint = 0x6b088;
pub const _ICL_DSI_T_INIT_MASTER_1: c_uint = 0x6b888;

pub const _DPHY_CLK_TIMING_PARAM_0: c_uint = 0x162180;
pub const _DPHY_CLK_TIMING_PARAM_1: c_uint = 0x6c180;

pub const _DSI_CLK_TIMING_PARAM_0: c_uint = 0x6b080;
pub const _DSI_CLK_TIMING_PARAM_1: c_uint = 0x6b880;

pub const CLK_PREPARE_SHIFT: c_int = 28;

pub const CLK_ZERO_SHIFT: c_int = 20;

pub const CLK_PRE_SHIFT: c_int = 16;

pub const CLK_POST_SHIFT: c_int = 8;

pub const CLK_TRAIL_SHIFT: c_int = 0;
pub const _DPHY_DATA_TIMING_PARAM_0: c_uint = 0x162184;
pub const _DPHY_DATA_TIMING_PARAM_1: c_uint = 0x6c184;

pub const _DSI_DATA_TIMING_PARAM_0: c_uint = 0x6B084;
pub const _DSI_DATA_TIMING_PARAM_1: c_uint = 0x6B884;

pub const HS_PREPARE_SHIFT: c_int = 24;

pub const HS_ZERO_SHIFT: c_int = 16;

pub const HS_TRAIL_SHIFT: c_int = 8;

pub const HS_EXIT_SHIFT: c_int = 0;
pub const _DPHY_TA_TIMING_PARAM_0: c_uint = 0x162188;
pub const _DPHY_TA_TIMING_PARAM_1: c_uint = 0x6c188;

pub const _DSI_TA_TIMING_PARAM_0: c_uint = 0x6b098;
pub const _DSI_TA_TIMING_PARAM_1: c_uint = 0x6b898;

pub const TA_SURE_SHIFT: c_int = 16;

pub const TA_GO_SHIFT: c_int = 8;

pub const TA_GET_SHIFT: c_int = 0;
// DSI transcoder configuration
pub const _DSI_TRANS_FUNC_CONF_0: c_uint = 0x6b030;
pub const _DSI_TRANS_FUNC_CONF_1: c_uint = 0x6b830;

pub const OP_MODE_SHIFT: c_int = 28;

pub const PIX_FMT_SHIFT: c_int = 16;

pub const PIX_VIRT_CHAN_SHIFT: c_int = 12;

pub const PIX_BUF_THRESHOLD_SHIFT: c_int = 10;

pub const CONTINUOUS_CLK_SHIFT: c_int = 8;

pub const LINK_CALIBRATION_SHIFT: c_int = 4;

pub const _DSI_CMD_RXCTL_0: c_uint = 0x6b0d4;
pub const _DSI_CMD_RXCTL_1: c_uint = 0x6b8d4;

pub const NUMBER_RX_PLOAD_DW_SHIFT: c_int = 0;
pub const _DSI_CMD_TXCTL_0: c_uint = 0x6b0d0;
pub const _DSI_CMD_TXCTL_1: c_uint = 0x6b8d0;

pub const FREE_HEADER_CREDIT_SHIFT: c_uint = 0x8;

pub const FREE_PLOAD_CREDIT_SHIFT: c_int = 0;
pub const MAX_HEADER_CREDIT: c_uint = 0x10;
pub const MAX_PLOAD_CREDIT: c_uint = 0x40;
pub const _DSI_CMD_TXHDR_0: c_uint = 0x6b100;
pub const _DSI_CMD_TXHDR_1: c_uint = 0x6b900;

pub const PARAM_WC_LOWER_SHIFT: c_int = 8;
pub const PARAM_WC_UPPER_SHIFT: c_int = 16;

pub const VC_SHIFT: c_int = 6;

pub const DT_SHIFT: c_int = 0;
pub const _DSI_CMD_TXPYLD_0: c_uint = 0x6b104;
pub const _DSI_CMD_TXPYLD_1: c_uint = 0x6b904;

pub const _DSI_LP_MSG_0: c_uint = 0x6b0d8;
pub const _DSI_LP_MSG_1: c_uint = 0x6b8d8;

// DSI timeout registers
pub const _DSI_HSTX_TO_0: c_uint = 0x6b044;
pub const _DSI_HSTX_TO_1: c_uint = 0x6b844;

pub const HSTX_TIMEOUT_VALUE_SHIFT: c_int = 16;

pub const _DSI_LPRX_HOST_TO_0: c_uint = 0x6b048;
pub const _DSI_LPRX_HOST_TO_1: c_uint = 0x6b848;

pub const LPRX_TIMEOUT_VALUE_SHIFT: c_int = 0;

pub const _DSI_PWAIT_TO_0: c_uint = 0x6b040;
pub const _DSI_PWAIT_TO_1: c_uint = 0x6b840;

pub const PRESET_TIMEOUT_VALUE_SHIFT: c_int = 16;

pub const PRESPONSE_TIMEOUT_VALUE_SHIFT: c_int = 0;

pub const _DSI_TA_TO_0: c_uint = 0x6b04c;
pub const _DSI_TA_TO_1: c_uint = 0x6b84c;

pub const TA_TIMEOUT_VALUE_SHIFT: c_int = 0;

