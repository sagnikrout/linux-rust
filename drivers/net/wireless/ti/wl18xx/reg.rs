//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// This file is part of wlcore
//
// Copyright (C) 2011 Texas Instruments Inc.
//
pub const WL18XX_REGISTERS_BASE: c_uint = 0x00800000;
pub const WL18XX_CODE_BASE: c_uint = 0x00000000;
pub const WL18XX_DATA_BASE: c_uint = 0x00400000;
pub const WL18XX_DOUBLE_BUFFER_BASE: c_uint = 0x00600000;
pub const WL18XX_MCU_KEY_SEARCH_BASE: c_uint = 0x00700000;
pub const WL18XX_PHY_BASE: c_uint = 0x00900000;
pub const WL18XX_TOP_OCP_BASE: c_uint = 0x00A00000;
pub const WL18XX_PACKET_RAM_BASE: c_uint = 0x00B00000;
pub const WL18XX_HOST_BASE: c_uint = 0x00C00000;
pub const WL18XX_REGISTERS_DOWN_SIZE: c_uint = 0x0000B000;
pub const WL18XX_REG_BOOT_PART_START: c_uint = 0x00802000;
pub const WL18XX_REG_BOOT_PART_SIZE: c_uint = 0x00014578;
pub const WL18XX_PHY_INIT_MEM_ADDR: c_uint = 0x80926000;
pub const WL18XX_PHY_END_MEM_ADDR: c_uint = 0x8093CA44;

// Scratch Pad registers

// Spare registers

// PRCM registers
pub const PLATFORM_DETECTION: c_uint = 0xA0E3E0;
pub const OCS_EN: c_uint = 0xA02080;
pub const PRIMARY_CLK_DETECT: c_uint = 0xA020A6;
pub const PLLSH_COEX_PLL_N: c_uint = 0xA02384;
pub const PLLSH_COEX_PLL_M: c_uint = 0xA02382;
pub const PLLSH_COEX_PLL_SWALLOW_EN: c_uint = 0xA0238E;
pub const PLLSH_WL_PLL_SEL: c_uint = 0xA02398;
pub const PLLSH_WCS_PLL_N: c_uint = 0xA02362;
pub const PLLSH_WCS_PLL_M: c_uint = 0xA02360;
pub const PLLSH_WCS_PLL_Q_FACTOR_CFG_1: c_uint = 0xA02364;
pub const PLLSH_WCS_PLL_Q_FACTOR_CFG_2: c_uint = 0xA02366;
pub const PLLSH_WCS_PLL_P_FACTOR_CFG_1: c_uint = 0xA02368;
pub const PLLSH_WCS_PLL_P_FACTOR_CFG_2: c_uint = 0xA0236A;
pub const PLLSH_WCS_PLL_SWALLOW_EN: c_uint = 0xA0236C;
pub const PLLSH_WL_PLL_EN: c_uint = 0xA02392;
pub const PLLSH_WCS_PLL_Q_FACTOR_CFG_1_MASK: c_uint = 0xFFFF;
pub const PLLSH_WCS_PLL_Q_FACTOR_CFG_2_MASK: c_uint = 0x007F;
pub const PLLSH_WCS_PLL_P_FACTOR_CFG_1_MASK: c_uint = 0xFFFF;
pub const PLLSH_WCS_PLL_P_FACTOR_CFG_2_MASK: c_uint = 0x000F;
pub const PLLSH_WL_PLL_EN_VAL1: c_uint = 0x7;
pub const PLLSH_WL_PLL_EN_VAL2: c_uint = 0x2;
pub const PLLSH_COEX_PLL_SWALLOW_EN_VAL1: c_uint = 0x2;
pub const PLLSH_COEX_PLL_SWALLOW_EN_VAL2: c_uint = 0x11;
pub const PLLSH_WCS_PLL_SWALLOW_EN_VAL1: c_uint = 0x1;
pub const PLLSH_WCS_PLL_SWALLOW_EN_VAL2: c_uint = 0x12;
pub const PLLSH_WL_PLL_SEL_WCS_PLL: c_uint = 0x0;
pub const PLLSH_WL_PLL_SEL_COEX_PLL: c_uint = 0x1;
pub const WL18XX_REG_FUSE_DATA_1_3: c_uint = 0xA0260C;
pub const WL18XX_PG_VER_MASK: c_uint = 0x70;
pub const WL18XX_PG_VER_OFFSET: c_int = 4;
pub const WL18XX_ROM_VER_MASK: c_uint = 0x3e00;
pub const WL18XX_ROM_VER_OFFSET: c_int = 9;
pub const WL18XX_METAL_VER_MASK: c_uint = 0xC;
pub const WL18XX_METAL_VER_OFFSET: c_int = 2;
pub const WL18XX_NEW_METAL_VER_MASK: c_uint = 0x180;
pub const WL18XX_NEW_METAL_VER_OFFSET: c_int = 7;
pub const WL18XX_PACKAGE_TYPE_OFFSET: c_int = 13;
pub const WL18XX_PACKAGE_TYPE_WSP: c_int = 0;
pub const WL18XX_REG_FUSE_DATA_2_3: c_uint = 0xA02614;
pub const WL18XX_RDL_VER_MASK: c_uint = 0x1f00;
pub const WL18XX_RDL_VER_OFFSET: c_int = 8;
pub const WL18XX_REG_FUSE_BD_ADDR_1: c_uint = 0xA02602;
pub const WL18XX_REG_FUSE_BD_ADDR_2: c_uint = 0xA02606;
pub const WL18XX_CMD_MBOX_ADDRESS: c_uint = 0xB007B4;
pub const WL18XX_FW_STATUS_ADDR: c_uint = 0x50F8;

//
// Host Command Interrupt. Setting this bit masks
// the interrupt that the host issues to inform
// the FW that it has sent a command
// to the Wlan hardware Command Mailbox.
//

//
// Host Event Acknowlegde Interrupt. The host
// sets this bit to acknowledge that it received
// the unsolicited information from the event
// mailbox.
//

//
// To boot the firmware in PLT mode we need to write this value in
// SCR_PAD8 before starting.
//
pub const WL18XX_SCR_PAD8_PLT: c_uint = 0xBABABEBE;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl18xx_rdl_num {
    RDL_NONE	= 0,
    RDL_1_HP	= 1,
    RDL_2_SP	= 2,
    RDL_3_HP	= 3,
    RDL_4_SP	= 4,
    RDL_5_SP	= 0x11,
    RDL_6_SP	= 0x12,
    RDL_7_SP	= 0x13,
    RDL_8_SP	= 0x14,

    _RDL_LAST,
    RDL_MAX = _RDL_LAST - 1,
}

// FPGA_SPARE_1 register - used to change the PHY ATPG clock at boot time
pub const WL18XX_PHY_FPGA_SPARE_1: c_uint = 0x8093CA40;
// command to disable FDSP clock
pub const MEM_FDSP_CLK_120_DISABLE: c_uint = 0x80000000;
// command to set ATPG clock toward FDSP Code RAM rather than its own clock
pub const MEM_FDSP_CODERAM_FUNC_CLK_SEL: c_uint = 0xC0000000;
// command to re-enable FDSP clock
pub const MEM_FDSP_CLK_120_ENABLE: c_uint = 0x40000000;
