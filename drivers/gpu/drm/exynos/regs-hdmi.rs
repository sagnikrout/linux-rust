//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-hdmi.h
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
// Cloned from drivers/media/video/s5p-tv/regs-hdmi.h
//
// Copyright (c) 2010-2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// HDMI register header file for Samsung TVOUT driver
//
// Register part
//
// HDMI Version 1.3 & Common

// Control registers

// Core registers

// Timing generator registers

//
// Bit definition part
//
// HDMI_INTC_CON

// HDMI_INTC_FLAG

// HDMI_PHY_RSTOUT

// HDMI_CORE_RSTOUT

// HDMI_CON_0

// HDMI_CON_2

// HDMI_PHY_STATUS

// HDMI_MODE_SEL

// HDMI_TG_CMD

// HDMI Version 1.4
// Control registers
// #define HDMI_INTC_CON		HDMI_CTRL_BASE(0x0000)
// #define HDMI_INTC_FLAG		HDMI_CTRL_BASE(0x0004)

// #define HDMI_HPD_STATUS		HDMI_CTRL_BASE(0x000C)

// PHY Control bit definition
// HDMI_PHY_CON_0

// Video related registers

// Audio related registers

// Packet related registers

// AVI bit definition

// AUI bit definition

// VSI bit definition

// HDCP related registers

// HDMI I2S register

// n must be within range 0...(HDMI_I2S_CH_ST_MAXNUM - 1)
pub const HDMI_I2S_CH_ST_MAXNUM: c_int = 5;

// I2S bit definition
// I2S_CLK_CON

// I2S_CON_1

// I2S_CON_2

// I2S_PIN_SEL_0

// I2S_PIN_SEL_1

// I2S_PIN_SEL_2

// I2S_PIN_SEL_3

// I2S_DSD_CON

// I2S_MUX_CON

// I2S_CH_ST_CON

// I2S_CH_ST_0 / I2S_CH_ST_SH_0

// I2S_CH_ST_1 / I2S_CH_ST_SH_1

// I2S_CH_ST_2 / I2S_CH_ST_SH_2

// I2S_CH_ST_3 / I2S_CH_ST_SH_3

// I2S_CH_ST_4 / I2S_CH_ST_SH_4

// I2S_MUX_CH

// I2S_MUX_CUV

// I2S_CUV_L_R

// Timing generator registers
// TG configure/status registers

// HDMI PHY Registers Offsets
pub const HDMIPHY_POWER: c_uint = 0x74;
pub const HDMIPHY_MODE_SET_DONE: c_uint = 0x7c;
pub const HDMIPHY5433_MODE_SET_DONE: c_uint = 0x84;
// HDMI PHY Values
pub const HDMI_PHY_POWER_ON: c_uint = 0x80;
pub const HDMI_PHY_POWER_OFF: c_uint = 0xff;
// HDMI PHY Values
pub const HDMI_PHY_DISABLE_MODE_SET: c_uint = 0x80;
pub const HDMI_PHY_ENABLE_MODE_SET: c_uint = 0x00;
// PMU Registers for PHY
pub const PMU_HDMI_PHY_CONTROL: c_uint = 0x700;

pub const EXYNOS5433_SYSREG_DISP_HDMI_PHY: c_uint = 0x1008;
pub const SYSREG_HDMI_REFCLK_INT_CLK: c_int = 1;
