//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/vlv_dsi_regs.h
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

pub const BXT_MIPI_BASE: c_uint = 0x60000;

// BXT MIPI mode configure
pub const _BXT_MIPIA_TRANS_HACTIVE: c_uint = 0xb0f8;
pub const _BXT_MIPIC_TRANS_HACTIVE: c_uint = 0xb8f8;

pub const _BXT_MIPIA_TRANS_VACTIVE: c_uint = 0xb0fc;
pub const _BXT_MIPIC_TRANS_VACTIVE: c_uint = 0xb8fc;

pub const _BXT_MIPIA_TRANS_VTOTAL: c_uint = 0xb100;
pub const _BXT_MIPIC_TRANS_VTOTAL: c_uint = 0xb900;

pub const _MIPIA_PORT_CTRL: c_uint = 0x61190;
pub const _MIPIC_PORT_CTRL: c_uint = 0x61700;

// BXT port control
pub const _BXT_MIPIA_PORT_CTRL: c_uint = 0xb0c0;
pub const _BXT_MIPIC_PORT_CTRL: c_uint = 0xb8c0;

pub const MIPIA_MIPI4DPHY_DELAY_COUNT_SHIFT: c_int = 27;

pub const DUAL_LINK_MODE_SHIFT: c_int = 26;

pub const MIPIA_FLISDSI_DELAY_COUNT_SHIFT: c_int = 18;

pub const MIPIC_FLISDSI_DELAY_COUNT_HIGH_SHIFT: c_int = 15;

pub const MIPIC_MIPI4DPHY_DELAY_COUNT_SHIFT: c_int = 11;

pub const CSB_SHIFT: c_int = 9;

pub const MIPIC_FLISDSI_DELAY_COUNT_LOW_SHIFT: c_int = 5;

pub const LANE_CONFIGURATION_SHIFT: c_int = 0;

pub const _MIPIA_TEARING_CTRL: c_uint = 0x61194;
pub const _MIPIC_TEARING_CTRL: c_uint = 0x61704;

pub const TEARING_EFFECT_DELAY_SHIFT: c_int = 0;

// MIPI DSI Controller and D-PHY registers
pub const _MIPIA_DEVICE_READY: c_uint = 0xb000;
pub const _MIPIC_DEVICE_READY: c_uint = 0xb800;

pub const _MIPIA_INTR_STAT: c_uint = 0xb004;
pub const _MIPIC_INTR_STAT: c_uint = 0xb804;

pub const _MIPIA_INTR_EN: c_uint = 0xb008;
pub const _MIPIC_INTR_EN: c_uint = 0xb808;

pub const _MIPIA_DSI_FUNC_PRG: c_uint = 0xb00c;
pub const _MIPIC_DSI_FUNC_PRG: c_uint = 0xb80c;

pub const CMD_MODE_CHANNEL_NUMBER_SHIFT: c_int = 5;

pub const VID_MODE_CHANNEL_NUMBER_SHIFT: c_int = 3;

pub const DATA_LANES_PRG_REG_SHIFT: c_int = 0;

pub const _MIPIA_HS_TX_TIMEOUT: c_uint = 0xb010;
pub const _MIPIC_HS_TX_TIMEOUT: c_uint = 0xb810;

pub const HIGH_SPEED_TX_TIMEOUT_COUNTER_MASK: c_uint = 0xffffff;
pub const _MIPIA_LP_RX_TIMEOUT: c_uint = 0xb014;
pub const _MIPIC_LP_RX_TIMEOUT: c_uint = 0xb814;

pub const LOW_POWER_RX_TIMEOUT_COUNTER_MASK: c_uint = 0xffffff;
pub const _MIPIA_TURN_AROUND_TIMEOUT: c_uint = 0xb018;
pub const _MIPIC_TURN_AROUND_TIMEOUT: c_uint = 0xb818;

pub const TURN_AROUND_TIMEOUT_MASK: c_uint = 0x3f;
pub const _MIPIA_DEVICE_RESET_TIMER: c_uint = 0xb01c;
pub const _MIPIC_DEVICE_RESET_TIMER: c_uint = 0xb81c;

pub const DEVICE_RESET_TIMER_MASK: c_uint = 0xffff;
pub const _MIPIA_DPI_RESOLUTION: c_uint = 0xb020;
pub const _MIPIC_DPI_RESOLUTION: c_uint = 0xb820;

pub const VERTICAL_ADDRESS_SHIFT: c_int = 16;

pub const HORIZONTAL_ADDRESS_SHIFT: c_int = 0;
pub const HORIZONTAL_ADDRESS_MASK: c_uint = 0xffff;
pub const _MIPIA_DBI_FIFO_THROTTLE: c_uint = 0xb024;
pub const _MIPIC_DBI_FIFO_THROTTLE: c_uint = 0xb824;

// regs below are bits 15:0
pub const _MIPIA_HSYNC_PADDING_COUNT: c_uint = 0xb028;
pub const _MIPIC_HSYNC_PADDING_COUNT: c_uint = 0xb828;

pub const _MIPIA_HBP_COUNT: c_uint = 0xb02c;
pub const _MIPIC_HBP_COUNT: c_uint = 0xb82c;

pub const _MIPIA_HFP_COUNT: c_uint = 0xb030;
pub const _MIPIC_HFP_COUNT: c_uint = 0xb830;

pub const _MIPIA_HACTIVE_AREA_COUNT: c_uint = 0xb034;
pub const _MIPIC_HACTIVE_AREA_COUNT: c_uint = 0xb834;

pub const _MIPIA_VSYNC_PADDING_COUNT: c_uint = 0xb038;
pub const _MIPIC_VSYNC_PADDING_COUNT: c_uint = 0xb838;

pub const _MIPIA_VBP_COUNT: c_uint = 0xb03c;
pub const _MIPIC_VBP_COUNT: c_uint = 0xb83c;

pub const _MIPIA_VFP_COUNT: c_uint = 0xb040;
pub const _MIPIC_VFP_COUNT: c_uint = 0xb840;

pub const _MIPIA_HIGH_LOW_SWITCH_COUNT: c_uint = 0xb044;
pub const _MIPIC_HIGH_LOW_SWITCH_COUNT: c_uint = 0xb844;

pub const _MIPIA_DPI_CONTROL: c_uint = 0xb048;
pub const _MIPIC_DPI_CONTROL: c_uint = 0xb848;

pub const _MIPIA_DPI_DATA: c_uint = 0xb04c;
pub const _MIPIC_DPI_DATA: c_uint = 0xb84c;

pub const COMMAND_BYTE_SHIFT: c_int = 0;

pub const _MIPIA_INIT_COUNT: c_uint = 0xb050;
pub const _MIPIC_INIT_COUNT: c_uint = 0xb850;

pub const MASTER_INIT_TIMER_SHIFT: c_int = 0;

pub const _MIPIA_MAX_RETURN_PKT_SIZE: c_uint = 0xb054;
pub const _MIPIC_MAX_RETURN_PKT_SIZE: c_uint = 0xb854;

pub const MAX_RETURN_PKT_SIZE_SHIFT: c_int = 0;

pub const _MIPIA_VIDEO_MODE_FORMAT: c_uint = 0xb058;
pub const _MIPIC_VIDEO_MODE_FORMAT: c_uint = 0xb858;

pub const _MIPIA_EOT_DISABLE: c_uint = 0xb05c;
pub const _MIPIC_EOT_DISABLE: c_uint = 0xb85c;

pub const _MIPIA_LP_BYTECLK: c_uint = 0xb060;
pub const _MIPIC_LP_BYTECLK: c_uint = 0xb860;

pub const LP_BYTECLK_SHIFT: c_int = 0;

pub const _MIPIA_TLPX_TIME_COUNT: c_uint = 0xb0a4;
pub const _MIPIC_TLPX_TIME_COUNT: c_uint = 0xb8a4;

pub const _MIPIA_CLK_LANE_TIMING: c_uint = 0xb098;
pub const _MIPIC_CLK_LANE_TIMING: c_uint = 0xb898;

// bits 31:0
pub const _MIPIA_LP_GEN_DATA: c_uint = 0xb064;
pub const _MIPIC_LP_GEN_DATA: c_uint = 0xb864;

// bits 31:0
pub const _MIPIA_HS_GEN_DATA: c_uint = 0xb068;
pub const _MIPIC_HS_GEN_DATA: c_uint = 0xb868;

pub const _MIPIA_LP_GEN_CTRL: c_uint = 0xb06c;
pub const _MIPIC_LP_GEN_CTRL: c_uint = 0xb86c;

pub const _MIPIA_HS_GEN_CTRL: c_uint = 0xb070;
pub const _MIPIC_HS_GEN_CTRL: c_uint = 0xb870;

pub const LONG_PACKET_WORD_COUNT_SHIFT: c_int = 8;

pub const SHORT_PACKET_PARAM_SHIFT: c_int = 8;

pub const VIRTUAL_CHANNEL_SHIFT: c_int = 6;

pub const DATA_TYPE_SHIFT: c_int = 0;

// data type values, see include/video/mipi_display.h
pub const _MIPIA_GEN_FIFO_STAT: c_uint = 0xb074;
pub const _MIPIC_GEN_FIFO_STAT: c_uint = 0xb874;

pub const _MIPIA_HS_LS_DBI_ENABLE: c_uint = 0xb078;
pub const _MIPIC_HS_LS_DBI_ENABLE: c_uint = 0xb878;

pub const _MIPIA_DPHY_PARAM: c_uint = 0xb080;
pub const _MIPIC_DPHY_PARAM: c_uint = 0xb880;

pub const EXIT_ZERO_COUNT_SHIFT: c_int = 24;

pub const TRAIL_COUNT_SHIFT: c_int = 16;

pub const CLK_ZERO_COUNT_SHIFT: c_int = 8;

pub const PREPARE_COUNT_SHIFT: c_int = 0;

pub const _MIPIA_DBI_BW_CTRL: c_uint = 0xb084;
pub const _MIPIC_DBI_BW_CTRL: c_uint = 0xb884;

pub const _MIPIA_CLK_LANE_SWITCH_TIME_CNT: c_uint = 0xb088;
pub const _MIPIC_CLK_LANE_SWITCH_TIME_CNT: c_uint = 0xb888;

pub const LP_HS_SSW_CNT_SHIFT: c_int = 16;

pub const HS_LP_PWR_SW_CNT_SHIFT: c_int = 0;

pub const _MIPIA_STOP_STATE_STALL: c_uint = 0xb08c;
pub const _MIPIC_STOP_STATE_STALL: c_uint = 0xb88c;

pub const STOP_STATE_STALL_COUNTER_SHIFT: c_int = 0;

pub const _MIPIA_INTR_STAT_REG_1: c_uint = 0xb090;
pub const _MIPIC_INTR_STAT_REG_1: c_uint = 0xb890;

pub const _MIPIA_INTR_EN_REG_1: c_uint = 0xb094;
pub const _MIPIC_INTR_EN_REG_1: c_uint = 0xb894;

// XXX: only pipe A ?!?

pub const DBI_TYPEC_OPTION_SHIFT: c_int = 28;

pub const DBI_TYPEC_FREQ_SHIFT: c_int = 24;

pub const DBI_TYPEC_OVERRIDE_COUNTER_SHIFT: c_int = 0;

// MIPI adapter registers
pub const _MIPIA_CTRL: c_uint = 0xb104;
pub const _MIPIC_CTRL: c_uint = 0xb904;

pub const READ_REQUEST_PRIORITY_SHIFT: c_int = 3;

pub const BXT_PIPE_SELECT_SHIFT: c_int = 7;

pub const BXT_PIXEL_OVERLAP_CNT_SHIFT: c_int = 10;

pub const _MIPIA_DATA_ADDRESS: c_uint = 0xb108;
pub const _MIPIC_DATA_ADDRESS: c_uint = 0xb908;

pub const DATA_MEM_ADDRESS_SHIFT: c_int = 5;

pub const _MIPIA_DATA_LENGTH: c_uint = 0xb10c;
pub const _MIPIC_DATA_LENGTH: c_uint = 0xb90c;

pub const DATA_LENGTH_SHIFT: c_int = 0;

pub const _MIPIA_COMMAND_ADDRESS: c_uint = 0xb110;
pub const _MIPIC_COMMAND_ADDRESS: c_uint = 0xb910;

pub const COMMAND_MEM_ADDRESS_SHIFT: c_int = 5;

pub const _MIPIA_COMMAND_LENGTH: c_uint = 0xb114;
pub const _MIPIC_COMMAND_LENGTH: c_uint = 0xb914;

pub const _MIPIA_READ_DATA_RETURN0: c_uint = 0xb118;
pub const _MIPIC_READ_DATA_RETURN0: c_uint = 0xb918;

pub const _MIPIA_READ_DATA_VALID: c_uint = 0xb138;
pub const _MIPIC_READ_DATA_VALID: c_uint = 0xb938;

