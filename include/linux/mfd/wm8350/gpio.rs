//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/gpio.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// gpio.h  --  GPIO Driver for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//

//
// GPIO Registers.
//
pub const WM8350_GPIO_DEBOUNCE: c_uint = 0x80;
pub const WM8350_GPIO_PIN_PULL_UP_CONTROL: c_uint = 0x81;
pub const WM8350_GPIO_PULL_DOWN_CONTROL: c_uint = 0x82;
pub const WM8350_GPIO_INT_MODE: c_uint = 0x83;
pub const WM8350_GPIO_CONTROL: c_uint = 0x85;
pub const WM8350_GPIO_CONFIGURATION_I_O: c_uint = 0x86;
pub const WM8350_GPIO_PIN_POLARITY_TYPE: c_uint = 0x87;
pub const WM8350_GPIO_FUNCTION_SELECT_1: c_uint = 0x8C;
pub const WM8350_GPIO_FUNCTION_SELECT_2: c_uint = 0x8D;
pub const WM8350_GPIO_FUNCTION_SELECT_3: c_uint = 0x8E;
pub const WM8350_GPIO_FUNCTION_SELECT_4: c_uint = 0x8F;
pub const WM8350_GPIO_LEVEL: c_uint = 0xE6;
//
// GPIO Functions
//
pub const WM8350_GPIO0_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO0_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO0_PWR_ON_IN: c_uint = 0x1;
pub const WM8350_GPIO0_PWR_ON_OUT: c_uint = 0x1;
pub const WM8350_GPIO0_LDO_EN_IN: c_uint = 0x2;
pub const WM8350_GPIO0_VRTC_OUT: c_uint = 0x2;
pub const WM8350_GPIO0_LPWR1_IN: c_uint = 0x3;
pub const WM8350_GPIO0_POR_B_OUT: c_uint = 0x3;
pub const WM8350_GPIO1_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO1_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO1_PWR_ON_IN: c_uint = 0x1;
pub const WM8350_GPIO1_DO_CONF_OUT: c_uint = 0x1;
pub const WM8350_GPIO1_LDO_EN_IN: c_uint = 0x2;
pub const WM8350_GPIO1_RESET_OUT: c_uint = 0x2;
pub const WM8350_GPIO1_LPWR2_IN: c_uint = 0x3;
pub const WM8350_GPIO1_MEMRST_OUT: c_uint = 0x3;
pub const WM8350_GPIO2_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO2_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO2_PWR_ON_IN: c_uint = 0x1;
pub const WM8350_GPIO2_PWR_ON_OUT: c_uint = 0x1;
pub const WM8350_GPIO2_WAKE_UP_IN: c_uint = 0x2;
pub const WM8350_GPIO2_VRTC_OUT: c_uint = 0x2;
pub const WM8350_GPIO2_32KHZ_IN: c_uint = 0x3;
pub const WM8350_GPIO2_32KHZ_OUT: c_uint = 0x3;
pub const WM8350_GPIO3_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO3_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO3_PWR_ON_IN: c_uint = 0x1;
pub const WM8350_GPIO3_P_CLK_OUT: c_uint = 0x1;
pub const WM8350_GPIO3_LDO_EN_IN: c_uint = 0x2;
pub const WM8350_GPIO3_VRTC_OUT: c_uint = 0x2;
pub const WM8350_GPIO3_PWR_OFF_IN: c_uint = 0x3;
pub const WM8350_GPIO3_32KHZ_OUT: c_uint = 0x3;
pub const WM8350_GPIO4_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO4_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO4_MR_IN: c_uint = 0x1;
pub const WM8350_GPIO4_MEM_RST_OUT: c_uint = 0x1;
pub const WM8350_GPIO4_FLASH_IN: c_uint = 0x2;
pub const WM8350_GPIO4_ADA_OUT: c_uint = 0x2;
pub const WM8350_GPIO4_HIBERNATE_IN: c_uint = 0x3;
pub const WM8350_GPIO4_FLASH_OUT: c_uint = 0x3;
pub const WM8350_GPIO4_MICDET_OUT: c_uint = 0x4;
pub const WM8350_GPIO4_MICSHT_OUT: c_uint = 0x5;
pub const WM8350_GPIO5_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO5_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO5_LPWR1_IN: c_uint = 0x1;
pub const WM8350_GPIO5_P_CLK_OUT: c_uint = 0x1;
pub const WM8350_GPIO5_ADCLRCLK_IN: c_uint = 0x2;
pub const WM8350_GPIO5_ADCLRCLK_OUT: c_uint = 0x2;
pub const WM8350_GPIO5_HIBERNATE_IN: c_uint = 0x3;
pub const WM8350_GPIO5_32KHZ_OUT: c_uint = 0x3;
pub const WM8350_GPIO5_MICDET_OUT: c_uint = 0x4;
pub const WM8350_GPIO5_MICSHT_OUT: c_uint = 0x5;
pub const WM8350_GPIO5_ADA_OUT: c_uint = 0x6;
pub const WM8350_GPIO5_OPCLK_OUT: c_uint = 0x7;
pub const WM8350_GPIO6_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO6_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO6_LPWR2_IN: c_uint = 0x1;
pub const WM8350_GPIO6_MEMRST_OUT: c_uint = 0x1;
pub const WM8350_GPIO6_FLASH_IN: c_uint = 0x2;
pub const WM8350_GPIO6_ADA_OUT: c_uint = 0x2;
pub const WM8350_GPIO6_HIBERNATE_IN: c_uint = 0x3;
pub const WM8350_GPIO6_RTC_OUT: c_uint = 0x3;
pub const WM8350_GPIO6_MICDET_OUT: c_uint = 0x4;
pub const WM8350_GPIO6_MICSHT_OUT: c_uint = 0x5;
pub const WM8350_GPIO6_ADCLRCLKB_OUT: c_uint = 0x6;
pub const WM8350_GPIO6_SDOUT_OUT: c_uint = 0x7;
pub const WM8350_GPIO7_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO7_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO7_LPWR3_IN: c_uint = 0x1;
pub const WM8350_GPIO7_P_CLK_OUT: c_uint = 0x1;
pub const WM8350_GPIO7_MASK_IN: c_uint = 0x2;
pub const WM8350_GPIO7_VCC_FAULT_OUT: c_uint = 0x2;
pub const WM8350_GPIO7_HIBERNATE_IN: c_uint = 0x3;
pub const WM8350_GPIO7_BATT_FAULT_OUT: c_uint = 0x3;
pub const WM8350_GPIO7_MICDET_OUT: c_uint = 0x4;
pub const WM8350_GPIO7_MICSHT_OUT: c_uint = 0x5;
pub const WM8350_GPIO7_ADA_OUT: c_uint = 0x6;
pub const WM8350_GPIO7_CSB_IN: c_uint = 0x7;
pub const WM8350_GPIO8_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO8_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO8_MR_IN: c_uint = 0x1;
pub const WM8350_GPIO8_VCC_FAULT_OUT: c_uint = 0x1;
pub const WM8350_GPIO8_ADCBCLK_IN: c_uint = 0x2;
pub const WM8350_GPIO8_ADCBCLK_OUT: c_uint = 0x2;
pub const WM8350_GPIO8_PWR_OFF_IN: c_uint = 0x3;
pub const WM8350_GPIO8_BATT_FAULT_OUT: c_uint = 0x3;
pub const WM8350_GPIO8_ALTSCL_IN: c_uint = 0xf;
pub const WM8350_GPIO9_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO9_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO9_HEARTBEAT_IN: c_uint = 0x1;
pub const WM8350_GPIO9_VCC_FAULT_OUT: c_uint = 0x1;
pub const WM8350_GPIO9_MASK_IN: c_uint = 0x2;
pub const WM8350_GPIO9_LINE_GT_BATT_OUT: c_uint = 0x2;
pub const WM8350_GPIO9_PWR_OFF_IN: c_uint = 0x3;
pub const WM8350_GPIO9_BATT_FAULT_OUT: c_uint = 0x3;
pub const WM8350_GPIO9_ALTSDA_OUT: c_uint = 0xf;
pub const WM8350_GPIO10_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO10_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO10_ISINKC_OUT: c_uint = 0x1;
pub const WM8350_GPIO10_PWR_OFF_IN: c_uint = 0x2;
pub const WM8350_GPIO10_LINE_GT_BATT_OUT: c_uint = 0x2;
pub const WM8350_GPIO10_CHD_IND_IN: c_uint = 0x3;
pub const WM8350_GPIO11_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO11_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO11_ISINKD_OUT: c_uint = 0x1;
pub const WM8350_GPIO11_WAKEUP_IN: c_uint = 0x2;
pub const WM8350_GPIO11_LINE_GT_BATT_OUT: c_uint = 0x2;
pub const WM8350_GPIO11_CHD_IND_IN: c_uint = 0x3;
pub const WM8350_GPIO12_GPIO_IN: c_uint = 0x0;
pub const WM8350_GPIO12_GPIO_OUT: c_uint = 0x0;
pub const WM8350_GPIO12_ISINKE_OUT: c_uint = 0x1;
pub const WM8350_GPIO12_LINE_GT_BATT_OUT: c_uint = 0x2;
pub const WM8350_GPIO12_LINE_EN_OUT: c_uint = 0x3;
pub const WM8350_GPIO12_32KHZ_OUT: c_uint = 0x4;
pub const WM8350_GPIO_DIR_IN: c_int = 0;
pub const WM8350_GPIO_DIR_OUT: c_int = 1;
pub const WM8350_GPIO_ACTIVE_LOW: c_int = 0;
pub const WM8350_GPIO_ACTIVE_HIGH: c_int = 1;
pub const WM8350_GPIO_PULL_NONE: c_int = 0;
pub const WM8350_GPIO_PULL_UP: c_int = 1;
pub const WM8350_GPIO_PULL_DOWN: c_int = 2;
pub const WM8350_GPIO_INVERT_OFF: c_int = 0;
pub const WM8350_GPIO_INVERT_ON: c_int = 1;
pub const WM8350_GPIO_DEBOUNCE_OFF: c_int = 0;
pub const WM8350_GPIO_DEBOUNCE_ON: c_int = 1;
//
// R30 (0x1E) - GPIO Interrupt Status
//
pub const WM8350_GP12_EINT: c_uint = 0x1000;
pub const WM8350_GP11_EINT: c_uint = 0x0800;
pub const WM8350_GP10_EINT: c_uint = 0x0400;
pub const WM8350_GP9_EINT: c_uint = 0x0200;
pub const WM8350_GP8_EINT: c_uint = 0x0100;
pub const WM8350_GP7_EINT: c_uint = 0x0080;
pub const WM8350_GP6_EINT: c_uint = 0x0040;
pub const WM8350_GP5_EINT: c_uint = 0x0020;
pub const WM8350_GP4_EINT: c_uint = 0x0010;
pub const WM8350_GP3_EINT: c_uint = 0x0008;
pub const WM8350_GP2_EINT: c_uint = 0x0004;
pub const WM8350_GP1_EINT: c_uint = 0x0002;
pub const WM8350_GP0_EINT: c_uint = 0x0001;
//
// R128 (0x80) - GPIO Debounce
//
pub const WM8350_GP12_DB: c_uint = 0x1000;
pub const WM8350_GP11_DB: c_uint = 0x0800;
pub const WM8350_GP10_DB: c_uint = 0x0400;
pub const WM8350_GP9_DB: c_uint = 0x0200;
pub const WM8350_GP8_DB: c_uint = 0x0100;
pub const WM8350_GP7_DB: c_uint = 0x0080;
pub const WM8350_GP6_DB: c_uint = 0x0040;
pub const WM8350_GP5_DB: c_uint = 0x0020;
pub const WM8350_GP4_DB: c_uint = 0x0010;
pub const WM8350_GP3_DB: c_uint = 0x0008;
pub const WM8350_GP2_DB: c_uint = 0x0004;
pub const WM8350_GP1_DB: c_uint = 0x0002;
pub const WM8350_GP0_DB: c_uint = 0x0001;
//
// R129 (0x81) - GPIO Pin pull up Control
//
pub const WM8350_GP12_PU: c_uint = 0x1000;
pub const WM8350_GP11_PU: c_uint = 0x0800;
pub const WM8350_GP10_PU: c_uint = 0x0400;
pub const WM8350_GP9_PU: c_uint = 0x0200;
pub const WM8350_GP8_PU: c_uint = 0x0100;
pub const WM8350_GP7_PU: c_uint = 0x0080;
pub const WM8350_GP6_PU: c_uint = 0x0040;
pub const WM8350_GP5_PU: c_uint = 0x0020;
pub const WM8350_GP4_PU: c_uint = 0x0010;
pub const WM8350_GP3_PU: c_uint = 0x0008;
pub const WM8350_GP2_PU: c_uint = 0x0004;
pub const WM8350_GP1_PU: c_uint = 0x0002;
pub const WM8350_GP0_PU: c_uint = 0x0001;
//
// R130 (0x82) - GPIO Pull down Control
//
pub const WM8350_GP12_PD: c_uint = 0x1000;
pub const WM8350_GP11_PD: c_uint = 0x0800;
pub const WM8350_GP10_PD: c_uint = 0x0400;
pub const WM8350_GP9_PD: c_uint = 0x0200;
pub const WM8350_GP8_PD: c_uint = 0x0100;
pub const WM8350_GP7_PD: c_uint = 0x0080;
pub const WM8350_GP6_PD: c_uint = 0x0040;
pub const WM8350_GP5_PD: c_uint = 0x0020;
pub const WM8350_GP4_PD: c_uint = 0x0010;
pub const WM8350_GP3_PD: c_uint = 0x0008;
pub const WM8350_GP2_PD: c_uint = 0x0004;
pub const WM8350_GP1_PD: c_uint = 0x0002;
pub const WM8350_GP0_PD: c_uint = 0x0001;
//
// R131 (0x83) - GPIO Interrupt Mode
//
pub const WM8350_GP12_INTMODE: c_uint = 0x1000;
pub const WM8350_GP11_INTMODE: c_uint = 0x0800;
pub const WM8350_GP10_INTMODE: c_uint = 0x0400;
pub const WM8350_GP9_INTMODE: c_uint = 0x0200;
pub const WM8350_GP8_INTMODE: c_uint = 0x0100;
pub const WM8350_GP7_INTMODE: c_uint = 0x0080;
pub const WM8350_GP6_INTMODE: c_uint = 0x0040;
pub const WM8350_GP5_INTMODE: c_uint = 0x0020;
pub const WM8350_GP4_INTMODE: c_uint = 0x0010;
pub const WM8350_GP3_INTMODE: c_uint = 0x0008;
pub const WM8350_GP2_INTMODE: c_uint = 0x0004;
pub const WM8350_GP1_INTMODE: c_uint = 0x0002;
pub const WM8350_GP0_INTMODE: c_uint = 0x0001;
//
// R133 (0x85) - GPIO Control
//
pub const WM8350_GP_DBTIME_MASK: c_uint = 0x00C0;
//
// R134 (0x86) - GPIO Configuration (i/o)
//
pub const WM8350_GP12_DIR: c_uint = 0x1000;
pub const WM8350_GP11_DIR: c_uint = 0x0800;
pub const WM8350_GP10_DIR: c_uint = 0x0400;
pub const WM8350_GP9_DIR: c_uint = 0x0200;
pub const WM8350_GP8_DIR: c_uint = 0x0100;
pub const WM8350_GP7_DIR: c_uint = 0x0080;
pub const WM8350_GP6_DIR: c_uint = 0x0040;
pub const WM8350_GP5_DIR: c_uint = 0x0020;
pub const WM8350_GP4_DIR: c_uint = 0x0010;
pub const WM8350_GP3_DIR: c_uint = 0x0008;
pub const WM8350_GP2_DIR: c_uint = 0x0004;
pub const WM8350_GP1_DIR: c_uint = 0x0002;
pub const WM8350_GP0_DIR: c_uint = 0x0001;
//
// R135 (0x87) - GPIO Pin Polarity / Type
//
pub const WM8350_GP12_CFG: c_uint = 0x1000;
pub const WM8350_GP11_CFG: c_uint = 0x0800;
pub const WM8350_GP10_CFG: c_uint = 0x0400;
pub const WM8350_GP9_CFG: c_uint = 0x0200;
pub const WM8350_GP8_CFG: c_uint = 0x0100;
pub const WM8350_GP7_CFG: c_uint = 0x0080;
pub const WM8350_GP6_CFG: c_uint = 0x0040;
pub const WM8350_GP5_CFG: c_uint = 0x0020;
pub const WM8350_GP4_CFG: c_uint = 0x0010;
pub const WM8350_GP3_CFG: c_uint = 0x0008;
pub const WM8350_GP2_CFG: c_uint = 0x0004;
pub const WM8350_GP1_CFG: c_uint = 0x0002;
pub const WM8350_GP0_CFG: c_uint = 0x0001;
//
// R140 (0x8C) - GPIO Function Select 1
//
pub const WM8350_GP3_FN_MASK: c_uint = 0xF000;
pub const WM8350_GP2_FN_MASK: c_uint = 0x0F00;
pub const WM8350_GP1_FN_MASK: c_uint = 0x00F0;
pub const WM8350_GP0_FN_MASK: c_uint = 0x000F;
//
// R141 (0x8D) - GPIO Function Select 2
//
pub const WM8350_GP7_FN_MASK: c_uint = 0xF000;
pub const WM8350_GP6_FN_MASK: c_uint = 0x0F00;
pub const WM8350_GP5_FN_MASK: c_uint = 0x00F0;
pub const WM8350_GP4_FN_MASK: c_uint = 0x000F;
//
// R142 (0x8E) - GPIO Function Select 3
//
pub const WM8350_GP11_FN_MASK: c_uint = 0xF000;
pub const WM8350_GP10_FN_MASK: c_uint = 0x0F00;
pub const WM8350_GP9_FN_MASK: c_uint = 0x00F0;
pub const WM8350_GP8_FN_MASK: c_uint = 0x000F;
//
// R143 (0x8F) - GPIO Function Select 4
//
pub const WM8350_GP12_FN_MASK: c_uint = 0x000F;
//
// R230 (0xE6) - GPIO Pin Status
//
pub const WM8350_GP12_LVL: c_uint = 0x1000;
pub const WM8350_GP11_LVL: c_uint = 0x0800;
pub const WM8350_GP10_LVL: c_uint = 0x0400;
pub const WM8350_GP9_LVL: c_uint = 0x0200;
pub const WM8350_GP8_LVL: c_uint = 0x0100;
pub const WM8350_GP7_LVL: c_uint = 0x0080;
pub const WM8350_GP6_LVL: c_uint = 0x0040;
pub const WM8350_GP5_LVL: c_uint = 0x0020;
pub const WM8350_GP4_LVL: c_uint = 0x0010;
pub const WM8350_GP3_LVL: c_uint = 0x0008;
pub const WM8350_GP2_LVL: c_uint = 0x0004;
pub const WM8350_GP1_LVL: c_uint = 0x0002;
pub const WM8350_GP0_LVL: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_gpio {
    pub pdev: *mut platform_device,
}

//
// GPIO Interrupts
//

