//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/reg.h
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
// This file is part of wl12xx
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

pub const REGISTERS_BASE: c_uint = 0x00300000;
pub const DRPW_BASE: c_uint = 0x00310000;
pub const REGISTERS_DOWN_SIZE: c_uint = 0x00008800;
pub const REGISTERS_WORK_SIZE: c_uint = 0x0000b000;

// ===============================================

// =============================================
// ==============================================

// =============================================

// =============================================

// =============================================

// =============================================

// =============================================

// Device Configuration registers

// Embedded ARM CPU Control
// ===============================================

// ===============================================
// ================================================

// Power Management registers

// Scratch Pad registers

// Spare registers

pub const WL12XX_CMD_MBOX_ADDRESS: c_uint = 0x407B4;

// Command/Information Mailbox Pointers
// ===============================================

// ===============================================

// ===============================================

pub const EE_WRITE: c_uint = 0x00000001ul;
pub const EE_READ: c_uint = 0x00000002ul;
// ===============================================

// ===============================================

// ===============================================

// ===============================================

pub const ACX_MAX_GPIO_LINES: c_int = 15;
// ===============================================

pub const ACX_CONT_WIND_MIN_MASK: c_uint = 0x0000007f;
pub const ACX_CONT_WIND_MAX: c_uint = 0x03ff0000;
pub const REF_FREQ_19_2: c_int = 0;
pub const REF_FREQ_26_0: c_int = 1;
pub const REF_FREQ_38_4: c_int = 2;
pub const REF_FREQ_40_0: c_int = 3;
pub const REF_FREQ_33_6: c_int = 4;
pub const REF_FREQ_NUM: c_int = 5;
pub const LUT_PARAM_INTEGER_DIVIDER: c_int = 0;
pub const LUT_PARAM_FRACTIONAL_DIVIDER: c_int = 1;
pub const LUT_PARAM_ATTN_BB: c_int = 2;
pub const LUT_PARAM_ALPHA_BB: c_int = 3;
pub const LUT_PARAM_STOP_TIME_BB: c_int = 4;
pub const LUT_PARAM_BB_PLL_LOOP_FILTER: c_int = 5;
pub const LUT_PARAM_NUM: c_int = 6;

pub const USE_EEPROM: c_int = 0;
pub const NVS_DATA_BUNDARY_ALIGNMENT: c_int = 4;
// Firmware image header size
pub const FW_HDR_SIZE: c_int = 8;
//

//
pub const OCP_CMD_LOOP: c_int = 32;
pub const OCP_CMD_WRITE: c_uint = 0x1;
pub const OCP_CMD_READ: c_uint = 0x2;

pub const OCP_STATUS_NO_RESP: c_uint = 0x00000;
pub const OCP_STATUS_OK: c_uint = 0x10000;
pub const OCP_STATUS_REQ_FAILED: c_uint = 0x20000;
pub const OCP_STATUS_RESP_ERROR: c_uint = 0x30000;
pub const OCP_REG_POLARITY: c_uint = 0x0064;
pub const OCP_REG_CLK_TYPE: c_uint = 0x0448;
pub const OCP_REG_CLK_POLARITY: c_uint = 0x0cb2;
pub const OCP_REG_CLK_PULL: c_uint = 0x0cb4;

pub const FREF_CLK_TYPE_BITS: c_uint = 0xfffffe7f;
pub const CLK_REQ_PRCM: c_uint = 0x100;
pub const FREF_CLK_POLARITY_BITS: c_uint = 0xfffff8ff;
pub const CLK_REQ_OUTN_SEL: c_uint = 0x700;
pub const WU_COUNTER_PAUSE_VAL: c_uint = 0x3FF;
// PLL configuration algorithm for wl128x
pub const SYS_CLK_CFG_REG: c_uint = 0x2200;
// Bit[0]   -  0-TCXO,  1-FREF

// Bit[3:2] - 01-TCXO, 10-FREF

// Bit[4]   -  0-TCXO,  1-FREF

pub const TCXO_ILOAD_INT_REG: c_uint = 0x2264;
pub const TCXO_CLK_DETECT_REG: c_uint = 0x2266;

pub const FREF_ILOAD_INT_REG: c_uint = 0x2084;
pub const FREF_CLK_DETECT_REG: c_uint = 0x2086;

// Use this reg for masking during driver access
pub const WL_SPARE_REG: c_uint = 0x2320;

// Bit[6:5:3] -  mask wl write SYS_CLK_CFG[8:5:2:4]

pub const PLL_LOCK_COUNTERS_REG: c_uint = 0xD8C;
pub const PLL_LOCK_COUNTERS_COEX: c_uint = 0x0F;
pub const PLL_LOCK_COUNTERS_MCS: c_uint = 0xF0;
pub const MCS_PLL_OVERRIDE_REG: c_uint = 0xD90;
pub const MCS_PLL_CONFIG_REG: c_uint = 0xD92;
pub const MCS_SEL_IN_FREQ_MASK: c_uint = 0x0070;
pub const MCS_SEL_IN_FREQ_SHIFT: c_int = 4;
pub const MCS_PLL_CONFIG_REG_VAL: c_uint = 0x73;

pub const MCS_PLL_M_REG: c_uint = 0xD94;
pub const MCS_PLL_N_REG: c_uint = 0xD96;
pub const MCS_PLL_M_REG_VAL: c_uint = 0xC8;
pub const MCS_PLL_N_REG_VAL: c_uint = 0x07;
pub const SDIO_IO_DS: c_uint = 0xd14;
// SDIO/wSPI DS configuration values
// end PLL configuration algorithm for wl128x
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

// ===============================================
pub const HI_CFG_UART_ENABLE: c_uint = 0x00000004;
pub const HI_CFG_RST232_ENABLE: c_uint = 0x00000008;
pub const HI_CFG_CLOCK_REQ_SELECT: c_uint = 0x00000010;
pub const HI_CFG_HOST_INT_ENABLE: c_uint = 0x00000020;
pub const HI_CFG_VLYNQ_OUTPUT_ENABLE: c_uint = 0x00000040;
pub const HI_CFG_HOST_INT_ACTIVE_LOW: c_uint = 0x00000080;
pub const HI_CFG_UART_TX_OUT_GPIO_15: c_uint = 0x00000100;
pub const HI_CFG_UART_TX_OUT_GPIO_14: c_uint = 0x00000200;
pub const HI_CFG_UART_TX_OUT_GPIO_7: c_uint = 0x00000400;

pub const WL127X_REG_FUSE_DATA_2_1: c_uint = 0x050a;
pub const WL128X_REG_FUSE_DATA_2_1: c_uint = 0x2152;
pub const PG_VER_MASK: c_uint = 0x3c;
pub const PG_VER_OFFSET: c_int = 2;
pub const WL127X_PG_MAJOR_VER_MASK: c_uint = 0x3;
pub const WL127X_PG_MAJOR_VER_OFFSET: c_uint = 0x0;
pub const WL127X_PG_MINOR_VER_MASK: c_uint = 0xc;
pub const WL127X_PG_MINOR_VER_OFFSET: c_uint = 0x2;
pub const WL128X_PG_MAJOR_VER_MASK: c_uint = 0xc;
pub const WL128X_PG_MAJOR_VER_OFFSET: c_uint = 0x2;
pub const WL128X_PG_MINOR_VER_MASK: c_uint = 0x3;
pub const WL128X_PG_MINOR_VER_OFFSET: c_uint = 0x0;

pub const WL12XX_REG_FUSE_BD_ADDR_1: c_uint = 0x00310eb4;
pub const WL12XX_REG_FUSE_BD_ADDR_2: c_uint = 0x00310eb8;
