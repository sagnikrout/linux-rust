//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mcde/mcde_dsi_regs.h
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
pub const DSI_MCTL_INTEGRATION_MODE: c_uint = 0x00000000;
pub const DSI_MCTL_MAIN_DATA_CTL: c_uint = 0x00000004;

pub const DSI_MCTL_MAIN_PHY_CTL: c_uint = 0x00000008;

pub const DSI_MCTL_MAIN_PHY_CTL_WAIT_BURST_TIME_SHIFT: c_int = 6;
pub const DSI_MCTL_MAIN_PHY_CTL_WAIT_BURST_TIME_MASK: c_uint = 0x000003C0;

pub const DSI_MCTL_PLL_CTL: c_uint = 0x0000000C;
pub const DSI_MCTL_LANE_STS: c_uint = 0x00000010;
pub const DSI_MCTL_DPHY_TIMEOUT: c_uint = 0x00000014;
pub const DSI_MCTL_DPHY_TIMEOUT_CLK_DIV_SHIFT: c_int = 0;
pub const DSI_MCTL_DPHY_TIMEOUT_CLK_DIV_MASK: c_uint = 0x0000000F;
pub const DSI_MCTL_DPHY_TIMEOUT_HSTX_TO_VAL_SHIFT: c_int = 4;
pub const DSI_MCTL_DPHY_TIMEOUT_HSTX_TO_VAL_MASK: c_uint = 0x0003FFF0;
pub const DSI_MCTL_DPHY_TIMEOUT_LPRX_TO_VAL_SHIFT: c_int = 18;
pub const DSI_MCTL_DPHY_TIMEOUT_LPRX_TO_VAL_MASK: c_uint = 0xFFFC0000;
pub const DSI_MCTL_ULPOUT_TIME: c_uint = 0x00000018;
pub const DSI_MCTL_ULPOUT_TIME_CKLANE_ULPOUT_TIME_SHIFT: c_int = 0;
pub const DSI_MCTL_ULPOUT_TIME_CKLANE_ULPOUT_TIME_MASK: c_uint = 0x000001FF;
pub const DSI_MCTL_ULPOUT_TIME_DATA_ULPOUT_TIME_SHIFT: c_int = 9;
pub const DSI_MCTL_ULPOUT_TIME_DATA_ULPOUT_TIME_MASK: c_uint = 0x0003FE00;
pub const DSI_MCTL_DPHY_STATIC: c_uint = 0x0000001C;

pub const DSI_MCTL_DPHY_STATIC_UI_X4_SHIFT: c_int = 6;
pub const DSI_MCTL_DPHY_STATIC_UI_X4_MASK: c_uint = 0x00000FC0;
pub const DSI_MCTL_MAIN_EN: c_uint = 0x00000020;

pub const DSI_MCTL_MAIN_STS: c_uint = 0x00000024;

pub const DSI_MCTL_DPHY_ERR: c_uint = 0x00000028;
pub const DSI_INT_VID_RDDATA: c_uint = 0x00000030;
pub const DSI_INT_VID_GNT: c_uint = 0x00000034;
pub const DSI_INT_CMD_RDDATA: c_uint = 0x00000038;
pub const DSI_INT_CMD_GNT: c_uint = 0x0000003C;
pub const DSI_INT_INTERRUPT_CTL: c_uint = 0x00000040;
pub const DSI_CMD_MODE_CTL: c_uint = 0x00000050;
pub const DSI_CMD_MODE_CTL_IF1_ID_SHIFT: c_int = 0;
pub const DSI_CMD_MODE_CTL_IF1_ID_MASK: c_uint = 0x00000003;
pub const DSI_CMD_MODE_CTL_IF2_ID_SHIFT: c_int = 2;
pub const DSI_CMD_MODE_CTL_IF2_ID_MASK: c_uint = 0x0000000C;

pub const DSI_CMD_MODE_CTL_FIL_VALUE_SHIFT: c_int = 8;
pub const DSI_CMD_MODE_CTL_FIL_VALUE_MASK: c_uint = 0x0000FF00;
pub const DSI_CMD_MODE_CTL_TE_TIMEOUT_SHIFT: c_int = 16;
pub const DSI_CMD_MODE_CTL_TE_TIMEOUT_MASK: c_uint = 0x03FF0000;
pub const DSI_CMD_MODE_STS: c_uint = 0x00000054;

pub const DSI_DIRECT_CMD_SEND: c_uint = 0x00000060;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS: c_uint = 0x00000064;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_SHIFT: c_int = 0;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_MASK: c_uint = 0x00000007;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_WRITE: c_int = 0;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_READ: c_int = 1;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_TE_REQ: c_int = 4;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_TRIG_REQ: c_int = 5;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_NAT_BTA_REQ: c_int = 6;

pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_HEAD_SHIFT: c_int = 8;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_HEAD_MASK: c_uint = 0x00003F00;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_ID_SHIFT: c_int = 14;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_CMD_SIZE_SHIFT: c_int = 16;

pub const DSI_DIRECT_CMD_MAIN_SETTINGS_TRIGGER_VAL_SHIFT: c_int = 24;
pub const DSI_DIRECT_CMD_MAIN_SETTINGS_TRIGGER_VAL_MASK: c_uint = 0x0F000000;
pub const DSI_DIRECT_CMD_STS: c_uint = 0x00000068;

pub const DSI_DIRECT_CMD_STS_TRIGGER_VAL_MASK: c_uint = 0x00007800;
pub const DSI_DIRECT_CMD_STS_TRIGGER_VAL_SHIFT: c_int = 11;
pub const DSI_DIRECT_CMD_STS_ACK_VAL_SHIFT: c_int = 16;
pub const DSI_DIRECT_CMD_STS_ACK_VAL_MASK: c_uint = 0xFFFF0000;
pub const DSI_DIRECT_CMD_RD_INIT: c_uint = 0x0000006C;
pub const DSI_DIRECT_CMD_RD_INIT_RESET_SHIFT: c_int = 0;
pub const DSI_DIRECT_CMD_RD_INIT_RESET_MASK: c_uint = 0xFFFFFFFF;
pub const DSI_DIRECT_CMD_WRDAT0: c_uint = 0x00000070;
pub const DSI_DIRECT_CMD_WRDAT1: c_uint = 0x00000074;
pub const DSI_DIRECT_CMD_WRDAT2: c_uint = 0x00000078;
pub const DSI_DIRECT_CMD_WRDAT3: c_uint = 0x0000007C;
pub const DSI_DIRECT_CMD_RDDAT: c_uint = 0x00000080;
pub const DSI_DIRECT_CMD_RD_PROPERTY: c_uint = 0x00000084;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_SIZE_SHIFT: c_int = 0;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_SIZE_MASK: c_uint = 0x0000FFFF;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_ID_SHIFT: c_int = 16;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_ID_MASK: c_uint = 0x00030000;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_DCSNOTGENERIC_SHIFT: c_int = 18;
pub const DSI_DIRECT_CMD_RD_PROPERTY_RD_DCSNOTGENERIC_MASK: c_uint = 0x00040000;
pub const DSI_DIRECT_CMD_RD_STS: c_uint = 0x00000088;
pub const DSI_VID_MAIN_CTL: c_uint = 0x00000090;
pub const DSI_VID_MAIN_CTL_START_MODE_SHIFT: c_int = 0;
pub const DSI_VID_MAIN_CTL_START_MODE_MASK: c_uint = 0x00000003;
pub const DSI_VID_MAIN_CTL_STOP_MODE_SHIFT: c_int = 2;
pub const DSI_VID_MAIN_CTL_STOP_MODE_MASK: c_uint = 0x0000000C;
pub const DSI_VID_MAIN_CTL_VID_ID_SHIFT: c_int = 4;
pub const DSI_VID_MAIN_CTL_VID_ID_MASK: c_uint = 0x00000030;
pub const DSI_VID_MAIN_CTL_HEADER_SHIFT: c_int = 6;
pub const DSI_VID_MAIN_CTL_HEADER_MASK: c_uint = 0x00000FC0;
pub const DSI_VID_MAIN_CTL_VID_PIXEL_MODE_16BITS: c_int = 0;

pub const DSI_VID_MAIN_CTL_REG_BLKLINE_MODE_NULL: c_int = 0;

pub const DSI_VID_MAIN_CTL_REG_BLKEOL_MODE_NULL: c_int = 0;

pub const DSI_VID_MAIN_CTL_RECOVERY_MODE_SHIFT: c_int = 21;
pub const DSI_VID_MAIN_CTL_RECOVERY_MODE_MASK: c_uint = 0x00600000;
pub const DSI_VID_VSIZE: c_uint = 0x00000094;
pub const DSI_VID_VSIZE_VSA_LENGTH_SHIFT: c_int = 0;
pub const DSI_VID_VSIZE_VSA_LENGTH_MASK: c_uint = 0x0000003F;
pub const DSI_VID_VSIZE_VBP_LENGTH_SHIFT: c_int = 6;
pub const DSI_VID_VSIZE_VBP_LENGTH_MASK: c_uint = 0x00000FC0;
pub const DSI_VID_VSIZE_VFP_LENGTH_SHIFT: c_int = 12;
pub const DSI_VID_VSIZE_VFP_LENGTH_MASK: c_uint = 0x000FF000;
pub const DSI_VID_VSIZE_VACT_LENGTH_SHIFT: c_int = 20;
pub const DSI_VID_VSIZE_VACT_LENGTH_MASK: c_uint = 0x7FF00000;
pub const DSI_VID_HSIZE1: c_uint = 0x00000098;
pub const DSI_VID_HSIZE1_HSA_LENGTH_SHIFT: c_int = 0;
pub const DSI_VID_HSIZE1_HSA_LENGTH_MASK: c_uint = 0x000003FF;
pub const DSI_VID_HSIZE1_HBP_LENGTH_SHIFT: c_int = 10;
pub const DSI_VID_HSIZE1_HBP_LENGTH_MASK: c_uint = 0x000FFC00;
pub const DSI_VID_HSIZE1_HFP_LENGTH_SHIFT: c_int = 20;
pub const DSI_VID_HSIZE1_HFP_LENGTH_MASK: c_uint = 0x7FF00000;
pub const DSI_VID_HSIZE2: c_uint = 0x0000009C;
pub const DSI_VID_HSIZE2_RGB_SIZE_SHIFT: c_int = 0;
pub const DSI_VID_HSIZE2_RGB_SIZE_MASK: c_uint = 0x00001FFF;
pub const DSI_VID_BLKSIZE1: c_uint = 0x000000A0;
pub const DSI_VID_BLKSIZE1_BLKLINE_EVENT_PCK_SHIFT: c_int = 0;
pub const DSI_VID_BLKSIZE1_BLKLINE_EVENT_PCK_MASK: c_uint = 0x00001FFF;
pub const DSI_VID_BLKSIZE1_BLKEOL_PCK_SHIFT: c_int = 13;
pub const DSI_VID_BLKSIZE1_BLKEOL_PCK_MASK: c_uint = 0x03FFE000;
pub const DSI_VID_BLKSIZE2: c_uint = 0x000000A4;
pub const DSI_VID_BLKSIZE2_BLKLINE_PULSE_PCK_SHIFT: c_int = 0;
pub const DSI_VID_BLKSIZE2_BLKLINE_PULSE_PCK_MASK: c_uint = 0x00001FFF;
pub const DSI_VID_PCK_TIME: c_uint = 0x000000A8;
pub const DSI_VID_PCK_TIME_BLKEOL_DURATION_SHIFT: c_int = 0;
pub const DSI_VID_PCK_TIME_BLKEOL_DURATION_MASK: c_uint = 0x00000FFF;
pub const DSI_VID_DPHY_TIME: c_uint = 0x000000AC;
pub const DSI_VID_DPHY_TIME_REG_LINE_DURATION_SHIFT: c_int = 0;
pub const DSI_VID_DPHY_TIME_REG_LINE_DURATION_MASK: c_uint = 0x00001FFF;
pub const DSI_VID_DPHY_TIME_REG_WAKEUP_TIME_SHIFT: c_int = 13;
pub const DSI_VID_DPHY_TIME_REG_WAKEUP_TIME_MASK: c_uint = 0x00FFE000;
pub const DSI_VID_MODE_STS: c_uint = 0x000000BC;

pub const DSI_VID_VCA_SETTING1: c_uint = 0x000000C0;
pub const DSI_VID_VCA_SETTING1_MAX_BURST_LIMIT_SHIFT: c_int = 0;
pub const DSI_VID_VCA_SETTING1_MAX_BURST_LIMIT_MASK: c_uint = 0x0000FFFF;

pub const DSI_VID_VCA_SETTING2: c_uint = 0x000000C4;
pub const DSI_VID_VCA_SETTING2_EXACT_BURST_LIMIT_SHIFT: c_int = 0;
pub const DSI_VID_VCA_SETTING2_EXACT_BURST_LIMIT_MASK: c_uint = 0x0000FFFF;
pub const DSI_VID_VCA_SETTING2_MAX_LINE_LIMIT_SHIFT: c_int = 16;
pub const DSI_VID_VCA_SETTING2_MAX_LINE_LIMIT_MASK: c_uint = 0xFFFF0000;
pub const DSI_CMD_MODE_STS_CTL: c_uint = 0x000000F4;

pub const DSI_DIRECT_CMD_STS_CTL: c_uint = 0x000000F8;

pub const DSI_VID_MODE_STS_CTL: c_uint = 0x00000100;

pub const DSI_TG_STS_CTL: c_uint = 0x00000104;
pub const DSI_MCTL_DHPY_ERR_CTL: c_uint = 0x00000108;
pub const DSI_MCTL_MAIN_STS_CLR: c_uint = 0x00000110;
pub const DSI_CMD_MODE_STS_CLR: c_uint = 0x00000114;

pub const DSI_DIRECT_CMD_STS_CLR: c_uint = 0x00000118;

pub const DSI_DIRECT_CMD_RD_STS_CLR: c_uint = 0x0000011C;
pub const DSI_VID_MODE_STS_CLR: c_uint = 0x00000120;
pub const DSI_TG_STS_CLR: c_uint = 0x00000124;
pub const DSI_MCTL_DPHY_ERR_CLR: c_uint = 0x00000128;
pub const DSI_MCTL_MAIN_STS_FLAG: c_uint = 0x00000130;
pub const DSI_CMD_MODE_STS_FLAG: c_uint = 0x00000134;
pub const DSI_DIRECT_CMD_STS_FLAG: c_uint = 0x00000138;
pub const DSI_DIRECT_CMD_RD_STS_FLAG: c_uint = 0x0000013C;
pub const DSI_VID_MODE_STS_FLAG: c_uint = 0x00000140;
pub const DSI_TG_STS_FLAG: c_uint = 0x00000144;
pub const DSI_DPHY_LANES_TRIM: c_uint = 0x00000150;
pub const DSI_DPHY_LANES_TRIM_DPHY_SKEW_DAT1_SHIFT: c_int = 0;
pub const DSI_DPHY_LANES_TRIM_DPHY_SKEW_DAT1_MASK: c_uint = 0x00000003;

pub const DSI_DPHY_LANES_TRIM_DPHY_SKEW_CLK_SHIFT: c_int = 6;
pub const DSI_DPHY_LANES_TRIM_DPHY_SKEW_CLK_MASK: c_uint = 0x000000C0;
pub const DSI_DPHY_LANES_TRIM_DPHY_LP_RX_VIL_CLK_SHIFT: c_int = 8;
pub const DSI_DPHY_LANES_TRIM_DPHY_LP_RX_VIL_CLK_MASK: c_uint = 0x00000300;
pub const DSI_DPHY_LANES_TRIM_DPHY_LP_TX_SLEWRATE_CLK_SHIFT: c_int = 10;
pub const DSI_DPHY_LANES_TRIM_DPHY_LP_TX_SLEWRATE_CLK_MASK: c_uint = 0x00000C00;
pub const DSI_DPHY_LANES_TRIM_DPHY_SPECS_90_81B_0_81: c_int = 0;

pub const DSI_ID_REG: c_uint = 0x00000FF0;
