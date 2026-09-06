//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/db8500-prcmu-regs.h
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
// Copyright (C) STMicroelectronics 2009
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Kumar Sanghvi <kumar.sanghvi@stericsson.com>
// Author: Sundar Iyer <sundar.iyer@stericsson.com>
//
// PRCM Unit registers
//

pub const PRCM_ARM_PLLDIVPS_ARM_BRM_RATE: c_uint = 0x3f;
pub const PRCM_ARM_PLLDIVPS_MAX_MASK: c_uint = 0xf;

pub const PRCM_PLLARM_LOCKP_PRCM_PLLARM_LOCKP3: c_uint = 0x2;

pub const PRCM_PLLARM_ENABLE_PRCM_PLLARM_ENABLE: c_uint = 0x1;
pub const PRCM_PLLARM_ENABLE_PRCM_PLLARM_COUNTON: c_uint = 0x100;

// CPU mailbox registers

pub const PRCM_HOSTACCESS_REQ_HOSTACCESS_REQ: c_uint = 0x1;

pub const ARM_WAKEUP_MODEM: c_uint = 0x1;

// System reset register

// Level shifter and clamp control registers

// PRCMU clock/PLL/reset registers

pub const PRCM_PLL_FREQ_D_SHIFT: c_int = 0;

pub const PRCM_PLL_FREQ_N_SHIFT: c_int = 8;

pub const PRCM_PLL_FREQ_R_SHIFT: c_int = 16;

pub const PRCM_DSI_PLLOUT_SEL_DSI0_PLLOUT_DIVSEL_SHIFT: c_int = 0;

pub const PRCM_DSI_PLLOUT_SEL_DSI1_PLLOUT_DIVSEL_SHIFT: c_int = 8;

pub const PRCM_DSI_PLLOUT_SEL_OFF: c_int = 0;
pub const PRCM_DSI_PLLOUT_SEL_PHI: c_int = 1;
pub const PRCM_DSI_PLLOUT_SEL_PHI_2: c_int = 2;
pub const PRCM_DSI_PLLOUT_SEL_PHI_4: c_int = 3;
pub const PRCM_DSITVCLK_DIV_DSI0_ESC_CLK_DIV_SHIFT: c_int = 0;

pub const PRCM_DSITVCLK_DIV_DSI1_ESC_CLK_DIV_SHIFT: c_int = 8;

pub const PRCM_DSITVCLK_DIV_DSI2_ESC_CLK_DIV_SHIFT: c_int = 16;

// ePOD and memory power signal control registers

// Debug power control unit registers

// Miscellaneous unit registers

pub const PRCM_GPIOCR_DBG_STM_MOD_CMD1: c_uint = 0x800;
pub const PRCM_GPIOCR_DBG_UARTMOD_CMD0: c_uint = 0x1;
// PRCMU HW semaphore

pub const PRCM_CLKOCR_CLKODIV0_SHIFT: c_int = 0;

pub const PRCM_CLKOCR_CLKOSEL0_SHIFT: c_int = 6;

pub const PRCM_CLKOCR_CLKODIV1_SHIFT: c_int = 16;

pub const PRCM_CLKOCR_CLKOSEL1_SHIFT: c_int = 22;

// GPIOCR register

// Miscellaneous unit registers

// System reset register

