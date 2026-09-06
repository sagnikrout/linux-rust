//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/irq.h
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
// include/linux/mfd/wm831x/irq.h -- Interrupt controller for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Interrupt number assignments within Linux
pub const WM831X_IRQ_TEMP_THW: c_int = 0;
pub const WM831X_IRQ_GPIO_1: c_int = 1;
pub const WM831X_IRQ_GPIO_2: c_int = 2;
pub const WM831X_IRQ_GPIO_3: c_int = 3;
pub const WM831X_IRQ_GPIO_4: c_int = 4;
pub const WM831X_IRQ_GPIO_5: c_int = 5;
pub const WM831X_IRQ_GPIO_6: c_int = 6;
pub const WM831X_IRQ_GPIO_7: c_int = 7;
pub const WM831X_IRQ_GPIO_8: c_int = 8;
pub const WM831X_IRQ_GPIO_9: c_int = 9;
pub const WM831X_IRQ_GPIO_10: c_int = 10;
pub const WM831X_IRQ_GPIO_11: c_int = 11;
pub const WM831X_IRQ_GPIO_12: c_int = 12;
pub const WM831X_IRQ_GPIO_13: c_int = 13;
pub const WM831X_IRQ_GPIO_14: c_int = 14;
pub const WM831X_IRQ_GPIO_15: c_int = 15;
pub const WM831X_IRQ_GPIO_16: c_int = 16;
pub const WM831X_IRQ_ON: c_int = 17;
pub const WM831X_IRQ_PPM_SYSLO: c_int = 18;
pub const WM831X_IRQ_PPM_PWR_SRC: c_int = 19;
pub const WM831X_IRQ_PPM_USB_CURR: c_int = 20;
pub const WM831X_IRQ_WDOG_TO: c_int = 21;
pub const WM831X_IRQ_RTC_PER: c_int = 22;
pub const WM831X_IRQ_RTC_ALM: c_int = 23;
pub const WM831X_IRQ_CHG_BATT_HOT: c_int = 24;
pub const WM831X_IRQ_CHG_BATT_COLD: c_int = 25;
pub const WM831X_IRQ_CHG_BATT_FAIL: c_int = 26;
pub const WM831X_IRQ_CHG_OV: c_int = 27;
pub const WM831X_IRQ_CHG_END: c_int = 29;
pub const WM831X_IRQ_CHG_TO: c_int = 30;
pub const WM831X_IRQ_CHG_MODE: c_int = 31;
pub const WM831X_IRQ_CHG_START: c_int = 32;
pub const WM831X_IRQ_TCHDATA: c_int = 33;
pub const WM831X_IRQ_TCHPD: c_int = 34;
pub const WM831X_IRQ_AUXADC_DATA: c_int = 35;
pub const WM831X_IRQ_AUXADC_DCOMP1: c_int = 36;
pub const WM831X_IRQ_AUXADC_DCOMP2: c_int = 37;
pub const WM831X_IRQ_AUXADC_DCOMP3: c_int = 38;
pub const WM831X_IRQ_AUXADC_DCOMP4: c_int = 39;
pub const WM831X_IRQ_CS1: c_int = 40;
pub const WM831X_IRQ_CS2: c_int = 41;
pub const WM831X_IRQ_HC_DC1: c_int = 42;
pub const WM831X_IRQ_HC_DC2: c_int = 43;
pub const WM831X_IRQ_UV_LDO1: c_int = 44;
pub const WM831X_IRQ_UV_LDO2: c_int = 45;
pub const WM831X_IRQ_UV_LDO3: c_int = 46;
pub const WM831X_IRQ_UV_LDO4: c_int = 47;
pub const WM831X_IRQ_UV_LDO5: c_int = 48;
pub const WM831X_IRQ_UV_LDO6: c_int = 49;
pub const WM831X_IRQ_UV_LDO7: c_int = 50;
pub const WM831X_IRQ_UV_LDO8: c_int = 51;
pub const WM831X_IRQ_UV_LDO9: c_int = 52;
pub const WM831X_IRQ_UV_LDO10: c_int = 53;
pub const WM831X_IRQ_UV_DC1: c_int = 54;
pub const WM831X_IRQ_UV_DC2: c_int = 55;
pub const WM831X_IRQ_UV_DC3: c_int = 56;
pub const WM831X_IRQ_UV_DC4: c_int = 57;
pub const WM831X_NUM_IRQS: c_int = 58;
//
// R16400 (0x4010) - System Interrupts
//
pub const WM831X_PS_INT: c_uint = 0x8000  /* PS_INT */;
pub const WM831X_PS_INT_MASK: c_uint = 0x8000  /* PS_INT */;

pub const WM831X_TEMP_INT: c_uint = 0x4000  /* TEMP_INT */;
pub const WM831X_TEMP_INT_MASK: c_uint = 0x4000  /* TEMP_INT */;

pub const WM831X_GP_INT: c_uint = 0x2000  /* GP_INT */;
pub const WM831X_GP_INT_MASK: c_uint = 0x2000  /* GP_INT */;

pub const WM831X_ON_PIN_INT: c_uint = 0x1000  /* ON_PIN_INT */;
pub const WM831X_ON_PIN_INT_MASK: c_uint = 0x1000  /* ON_PIN_INT */;

pub const WM831X_WDOG_INT: c_uint = 0x0800  /* WDOG_INT */;
pub const WM831X_WDOG_INT_MASK: c_uint = 0x0800  /* WDOG_INT */;

pub const WM831X_TCHDATA_INT: c_uint = 0x0400  /* TCHDATA_INT */;
pub const WM831X_TCHDATA_INT_MASK: c_uint = 0x0400  /* TCHDATA_INT */;

pub const WM831X_TCHPD_INT: c_uint = 0x0200  /* TCHPD_INT */;
pub const WM831X_TCHPD_INT_MASK: c_uint = 0x0200  /* TCHPD_INT */;

pub const WM831X_AUXADC_INT: c_uint = 0x0100  /* AUXADC_INT */;
pub const WM831X_AUXADC_INT_MASK: c_uint = 0x0100  /* AUXADC_INT */;

pub const WM831X_PPM_INT: c_uint = 0x0080  /* PPM_INT */;
pub const WM831X_PPM_INT_MASK: c_uint = 0x0080  /* PPM_INT */;

pub const WM831X_CS_INT: c_uint = 0x0040  /* CS_INT */;
pub const WM831X_CS_INT_MASK: c_uint = 0x0040  /* CS_INT */;

pub const WM831X_RTC_INT: c_uint = 0x0020  /* RTC_INT */;
pub const WM831X_RTC_INT_MASK: c_uint = 0x0020  /* RTC_INT */;

pub const WM831X_OTP_INT: c_uint = 0x0010  /* OTP_INT */;
pub const WM831X_OTP_INT_MASK: c_uint = 0x0010  /* OTP_INT */;

pub const WM831X_CHILD_INT: c_uint = 0x0008  /* CHILD_INT */;
pub const WM831X_CHILD_INT_MASK: c_uint = 0x0008  /* CHILD_INT */;

pub const WM831X_CHG_INT: c_uint = 0x0004  /* CHG_INT */;
pub const WM831X_CHG_INT_MASK: c_uint = 0x0004  /* CHG_INT */;

pub const WM831X_HC_INT: c_uint = 0x0002  /* HC_INT */;
pub const WM831X_HC_INT_MASK: c_uint = 0x0002  /* HC_INT */;

pub const WM831X_UV_INT: c_uint = 0x0001  /* UV_INT */;
pub const WM831X_UV_INT_MASK: c_uint = 0x0001  /* UV_INT */;

//
// R16401 (0x4011) - Interrupt Status 1
//
pub const WM831X_PPM_SYSLO_EINT: c_uint = 0x8000  /* PPM_SYSLO_EINT */;
pub const WM831X_PPM_SYSLO_EINT_MASK: c_uint = 0x8000  /* PPM_SYSLO_EINT */;

pub const WM831X_PPM_PWR_SRC_EINT: c_uint = 0x4000  /* PPM_PWR_SRC_EINT */;
pub const WM831X_PPM_PWR_SRC_EINT_MASK: c_uint = 0x4000  /* PPM_PWR_SRC_EINT */;

pub const WM831X_PPM_USB_CURR_EINT: c_uint = 0x2000  /* PPM_USB_CURR_EINT */;
pub const WM831X_PPM_USB_CURR_EINT_MASK: c_uint = 0x2000  /* PPM_USB_CURR_EINT */;

pub const WM831X_ON_PIN_EINT: c_uint = 0x1000  /* ON_PIN_EINT */;
pub const WM831X_ON_PIN_EINT_MASK: c_uint = 0x1000  /* ON_PIN_EINT */;

pub const WM831X_WDOG_TO_EINT: c_uint = 0x0800  /* WDOG_TO_EINT */;
pub const WM831X_WDOG_TO_EINT_MASK: c_uint = 0x0800  /* WDOG_TO_EINT */;

pub const WM831X_TCHDATA_EINT: c_uint = 0x0400  /* TCHDATA_EINT */;
pub const WM831X_TCHDATA_EINT_MASK: c_uint = 0x0400  /* TCHDATA_EINT */;

pub const WM831X_TCHPD_EINT: c_uint = 0x0200  /* TCHPD_EINT */;
pub const WM831X_TCHPD_EINT_MASK: c_uint = 0x0200  /* TCHPD_EINT */;

pub const WM831X_AUXADC_DATA_EINT: c_uint = 0x0100  /* AUXADC_DATA_EINT */;
pub const WM831X_AUXADC_DATA_EINT_MASK: c_uint = 0x0100  /* AUXADC_DATA_EINT */;

pub const WM831X_AUXADC_DCOMP4_EINT: c_uint = 0x0080  /* AUXADC_DCOMP4_EINT */;
pub const WM831X_AUXADC_DCOMP4_EINT_MASK: c_uint = 0x0080  /* AUXADC_DCOMP4_EINT */;

pub const WM831X_AUXADC_DCOMP3_EINT: c_uint = 0x0040  /* AUXADC_DCOMP3_EINT */;
pub const WM831X_AUXADC_DCOMP3_EINT_MASK: c_uint = 0x0040  /* AUXADC_DCOMP3_EINT */;

pub const WM831X_AUXADC_DCOMP2_EINT: c_uint = 0x0020  /* AUXADC_DCOMP2_EINT */;
pub const WM831X_AUXADC_DCOMP2_EINT_MASK: c_uint = 0x0020  /* AUXADC_DCOMP2_EINT */;

pub const WM831X_AUXADC_DCOMP1_EINT: c_uint = 0x0010  /* AUXADC_DCOMP1_EINT */;
pub const WM831X_AUXADC_DCOMP1_EINT_MASK: c_uint = 0x0010  /* AUXADC_DCOMP1_EINT */;

pub const WM831X_RTC_PER_EINT: c_uint = 0x0008  /* RTC_PER_EINT */;
pub const WM831X_RTC_PER_EINT_MASK: c_uint = 0x0008  /* RTC_PER_EINT */;

pub const WM831X_RTC_ALM_EINT: c_uint = 0x0004  /* RTC_ALM_EINT */;
pub const WM831X_RTC_ALM_EINT_MASK: c_uint = 0x0004  /* RTC_ALM_EINT */;

pub const WM831X_TEMP_THW_EINT: c_uint = 0x0002  /* TEMP_THW_EINT */;
pub const WM831X_TEMP_THW_EINT_MASK: c_uint = 0x0002  /* TEMP_THW_EINT */;

//
// R16402 (0x4012) - Interrupt Status 2
//
pub const WM831X_CHG_BATT_HOT_EINT: c_uint = 0x8000  /* CHG_BATT_HOT_EINT */;
pub const WM831X_CHG_BATT_HOT_EINT_MASK: c_uint = 0x8000  /* CHG_BATT_HOT_EINT */;

pub const WM831X_CHG_BATT_COLD_EINT: c_uint = 0x4000  /* CHG_BATT_COLD_EINT */;
pub const WM831X_CHG_BATT_COLD_EINT_MASK: c_uint = 0x4000  /* CHG_BATT_COLD_EINT */;

pub const WM831X_CHG_BATT_FAIL_EINT: c_uint = 0x2000  /* CHG_BATT_FAIL_EINT */;
pub const WM831X_CHG_BATT_FAIL_EINT_MASK: c_uint = 0x2000  /* CHG_BATT_FAIL_EINT */;

pub const WM831X_CHG_OV_EINT: c_uint = 0x1000  /* CHG_OV_EINT */;
pub const WM831X_CHG_OV_EINT_MASK: c_uint = 0x1000  /* CHG_OV_EINT */;

pub const WM831X_CHG_END_EINT: c_uint = 0x0800  /* CHG_END_EINT */;
pub const WM831X_CHG_END_EINT_MASK: c_uint = 0x0800  /* CHG_END_EINT */;

pub const WM831X_CHG_TO_EINT: c_uint = 0x0400  /* CHG_TO_EINT */;
pub const WM831X_CHG_TO_EINT_MASK: c_uint = 0x0400  /* CHG_TO_EINT */;

pub const WM831X_CHG_MODE_EINT: c_uint = 0x0200  /* CHG_MODE_EINT */;
pub const WM831X_CHG_MODE_EINT_MASK: c_uint = 0x0200  /* CHG_MODE_EINT */;

pub const WM831X_CHG_START_EINT: c_uint = 0x0100  /* CHG_START_EINT */;
pub const WM831X_CHG_START_EINT_MASK: c_uint = 0x0100  /* CHG_START_EINT */;

pub const WM831X_CS2_EINT: c_uint = 0x0080  /* CS2_EINT */;
pub const WM831X_CS2_EINT_MASK: c_uint = 0x0080  /* CS2_EINT */;

pub const WM831X_CS1_EINT: c_uint = 0x0040  /* CS1_EINT */;
pub const WM831X_CS1_EINT_MASK: c_uint = 0x0040  /* CS1_EINT */;

pub const WM831X_OTP_CMD_END_EINT: c_uint = 0x0020  /* OTP_CMD_END_EINT */;
pub const WM831X_OTP_CMD_END_EINT_MASK: c_uint = 0x0020  /* OTP_CMD_END_EINT */;

pub const WM831X_OTP_ERR_EINT: c_uint = 0x0010  /* OTP_ERR_EINT */;
pub const WM831X_OTP_ERR_EINT_MASK: c_uint = 0x0010  /* OTP_ERR_EINT */;

pub const WM831X_PS_POR_EINT: c_uint = 0x0004  /* PS_POR_EINT */;
pub const WM831X_PS_POR_EINT_MASK: c_uint = 0x0004  /* PS_POR_EINT */;

pub const WM831X_PS_SLEEP_OFF_EINT: c_uint = 0x0002  /* PS_SLEEP_OFF_EINT */;
pub const WM831X_PS_SLEEP_OFF_EINT_MASK: c_uint = 0x0002  /* PS_SLEEP_OFF_EINT */;

pub const WM831X_PS_ON_WAKE_EINT: c_uint = 0x0001  /* PS_ON_WAKE_EINT */;
pub const WM831X_PS_ON_WAKE_EINT_MASK: c_uint = 0x0001  /* PS_ON_WAKE_EINT */;

//
// R16403 (0x4013) - Interrupt Status 3
//
pub const WM831X_UV_LDO10_EINT: c_uint = 0x0200  /* UV_LDO10_EINT */;
pub const WM831X_UV_LDO10_EINT_MASK: c_uint = 0x0200  /* UV_LDO10_EINT */;

pub const WM831X_UV_LDO9_EINT: c_uint = 0x0100  /* UV_LDO9_EINT */;
pub const WM831X_UV_LDO9_EINT_MASK: c_uint = 0x0100  /* UV_LDO9_EINT */;

pub const WM831X_UV_LDO8_EINT: c_uint = 0x0080  /* UV_LDO8_EINT */;
pub const WM831X_UV_LDO8_EINT_MASK: c_uint = 0x0080  /* UV_LDO8_EINT */;

pub const WM831X_UV_LDO7_EINT: c_uint = 0x0040  /* UV_LDO7_EINT */;
pub const WM831X_UV_LDO7_EINT_MASK: c_uint = 0x0040  /* UV_LDO7_EINT */;

pub const WM831X_UV_LDO6_EINT: c_uint = 0x0020  /* UV_LDO6_EINT */;
pub const WM831X_UV_LDO6_EINT_MASK: c_uint = 0x0020  /* UV_LDO6_EINT */;

pub const WM831X_UV_LDO5_EINT: c_uint = 0x0010  /* UV_LDO5_EINT */;
pub const WM831X_UV_LDO5_EINT_MASK: c_uint = 0x0010  /* UV_LDO5_EINT */;

pub const WM831X_UV_LDO4_EINT: c_uint = 0x0008  /* UV_LDO4_EINT */;
pub const WM831X_UV_LDO4_EINT_MASK: c_uint = 0x0008  /* UV_LDO4_EINT */;

pub const WM831X_UV_LDO3_EINT: c_uint = 0x0004  /* UV_LDO3_EINT */;
pub const WM831X_UV_LDO3_EINT_MASK: c_uint = 0x0004  /* UV_LDO3_EINT */;

pub const WM831X_UV_LDO2_EINT: c_uint = 0x0002  /* UV_LDO2_EINT */;
pub const WM831X_UV_LDO2_EINT_MASK: c_uint = 0x0002  /* UV_LDO2_EINT */;

pub const WM831X_UV_LDO1_EINT: c_uint = 0x0001  /* UV_LDO1_EINT */;
pub const WM831X_UV_LDO1_EINT_MASK: c_uint = 0x0001  /* UV_LDO1_EINT */;

//
// R16404 (0x4014) - Interrupt Status 4
//
pub const WM831X_HC_DC2_EINT: c_uint = 0x0200  /* HC_DC2_EINT */;
pub const WM831X_HC_DC2_EINT_MASK: c_uint = 0x0200  /* HC_DC2_EINT */;

pub const WM831X_HC_DC1_EINT: c_uint = 0x0100  /* HC_DC1_EINT */;
pub const WM831X_HC_DC1_EINT_MASK: c_uint = 0x0100  /* HC_DC1_EINT */;

pub const WM831X_UV_DC4_EINT: c_uint = 0x0008  /* UV_DC4_EINT */;
pub const WM831X_UV_DC4_EINT_MASK: c_uint = 0x0008  /* UV_DC4_EINT */;

pub const WM831X_UV_DC3_EINT: c_uint = 0x0004  /* UV_DC3_EINT */;
pub const WM831X_UV_DC3_EINT_MASK: c_uint = 0x0004  /* UV_DC3_EINT */;

pub const WM831X_UV_DC2_EINT: c_uint = 0x0002  /* UV_DC2_EINT */;
pub const WM831X_UV_DC2_EINT_MASK: c_uint = 0x0002  /* UV_DC2_EINT */;

pub const WM831X_UV_DC1_EINT: c_uint = 0x0001  /* UV_DC1_EINT */;
pub const WM831X_UV_DC1_EINT_MASK: c_uint = 0x0001  /* UV_DC1_EINT */;

//
// R16405 (0x4015) - Interrupt Status 5
//
pub const WM831X_GP16_EINT: c_uint = 0x8000  /* GP16_EINT */;
pub const WM831X_GP16_EINT_MASK: c_uint = 0x8000  /* GP16_EINT */;

pub const WM831X_GP15_EINT: c_uint = 0x4000  /* GP15_EINT */;
pub const WM831X_GP15_EINT_MASK: c_uint = 0x4000  /* GP15_EINT */;

pub const WM831X_GP14_EINT: c_uint = 0x2000  /* GP14_EINT */;
pub const WM831X_GP14_EINT_MASK: c_uint = 0x2000  /* GP14_EINT */;

pub const WM831X_GP13_EINT: c_uint = 0x1000  /* GP13_EINT */;
pub const WM831X_GP13_EINT_MASK: c_uint = 0x1000  /* GP13_EINT */;

pub const WM831X_GP12_EINT: c_uint = 0x0800  /* GP12_EINT */;
pub const WM831X_GP12_EINT_MASK: c_uint = 0x0800  /* GP12_EINT */;

pub const WM831X_GP11_EINT: c_uint = 0x0400  /* GP11_EINT */;
pub const WM831X_GP11_EINT_MASK: c_uint = 0x0400  /* GP11_EINT */;

pub const WM831X_GP10_EINT: c_uint = 0x0200  /* GP10_EINT */;
pub const WM831X_GP10_EINT_MASK: c_uint = 0x0200  /* GP10_EINT */;

pub const WM831X_GP9_EINT: c_uint = 0x0100  /* GP9_EINT */;
pub const WM831X_GP9_EINT_MASK: c_uint = 0x0100  /* GP9_EINT */;

pub const WM831X_GP8_EINT: c_uint = 0x0080  /* GP8_EINT */;
pub const WM831X_GP8_EINT_MASK: c_uint = 0x0080  /* GP8_EINT */;

pub const WM831X_GP7_EINT: c_uint = 0x0040  /* GP7_EINT */;
pub const WM831X_GP7_EINT_MASK: c_uint = 0x0040  /* GP7_EINT */;

pub const WM831X_GP6_EINT: c_uint = 0x0020  /* GP6_EINT */;
pub const WM831X_GP6_EINT_MASK: c_uint = 0x0020  /* GP6_EINT */;

pub const WM831X_GP5_EINT: c_uint = 0x0010  /* GP5_EINT */;
pub const WM831X_GP5_EINT_MASK: c_uint = 0x0010  /* GP5_EINT */;

pub const WM831X_GP4_EINT: c_uint = 0x0008  /* GP4_EINT */;
pub const WM831X_GP4_EINT_MASK: c_uint = 0x0008  /* GP4_EINT */;

pub const WM831X_GP3_EINT: c_uint = 0x0004  /* GP3_EINT */;
pub const WM831X_GP3_EINT_MASK: c_uint = 0x0004  /* GP3_EINT */;

pub const WM831X_GP2_EINT: c_uint = 0x0002  /* GP2_EINT */;
pub const WM831X_GP2_EINT_MASK: c_uint = 0x0002  /* GP2_EINT */;

pub const WM831X_GP1_EINT: c_uint = 0x0001  /* GP1_EINT */;
pub const WM831X_GP1_EINT_MASK: c_uint = 0x0001  /* GP1_EINT */;

//
// R16407 (0x4017) - IRQ Config
//
pub const WM831X_IRQ_OD: c_uint = 0x0002  /* IRQ_OD */;
pub const WM831X_IRQ_OD_MASK: c_uint = 0x0002  /* IRQ_OD */;

pub const WM831X_IM_IRQ: c_uint = 0x0001  /* IM_IRQ */;
pub const WM831X_IM_IRQ_MASK: c_uint = 0x0001  /* IM_IRQ */;

//
// R16408 (0x4018) - System Interrupts Mask
//
pub const WM831X_IM_PS_INT: c_uint = 0x8000  /* IM_PS_INT */;
pub const WM831X_IM_PS_INT_MASK: c_uint = 0x8000  /* IM_PS_INT */;

pub const WM831X_IM_TEMP_INT: c_uint = 0x4000  /* IM_TEMP_INT */;
pub const WM831X_IM_TEMP_INT_MASK: c_uint = 0x4000  /* IM_TEMP_INT */;

pub const WM831X_IM_GP_INT: c_uint = 0x2000  /* IM_GP_INT */;
pub const WM831X_IM_GP_INT_MASK: c_uint = 0x2000  /* IM_GP_INT */;

pub const WM831X_IM_ON_PIN_INT: c_uint = 0x1000  /* IM_ON_PIN_INT */;
pub const WM831X_IM_ON_PIN_INT_MASK: c_uint = 0x1000  /* IM_ON_PIN_INT */;

pub const WM831X_IM_WDOG_INT: c_uint = 0x0800  /* IM_WDOG_INT */;
pub const WM831X_IM_WDOG_INT_MASK: c_uint = 0x0800  /* IM_WDOG_INT */;

pub const WM831X_IM_TCHDATA_INT: c_uint = 0x0400  /* IM_TCHDATA_INT */;
pub const WM831X_IM_TCHDATA_INT_MASK: c_uint = 0x0400  /* IM_TCHDATA_INT */;

pub const WM831X_IM_TCHPD_INT: c_uint = 0x0200  /* IM_TCHPD_INT */;
pub const WM831X_IM_TCHPD_INT_MASK: c_uint = 0x0200  /* IM_TCHPD_INT */;

pub const WM831X_IM_AUXADC_INT: c_uint = 0x0100  /* IM_AUXADC_INT */;
pub const WM831X_IM_AUXADC_INT_MASK: c_uint = 0x0100  /* IM_AUXADC_INT */;

pub const WM831X_IM_PPM_INT: c_uint = 0x0080  /* IM_PPM_INT */;
pub const WM831X_IM_PPM_INT_MASK: c_uint = 0x0080  /* IM_PPM_INT */;

pub const WM831X_IM_CS_INT: c_uint = 0x0040  /* IM_CS_INT */;
pub const WM831X_IM_CS_INT_MASK: c_uint = 0x0040  /* IM_CS_INT */;

pub const WM831X_IM_RTC_INT: c_uint = 0x0020  /* IM_RTC_INT */;
pub const WM831X_IM_RTC_INT_MASK: c_uint = 0x0020  /* IM_RTC_INT */;

pub const WM831X_IM_OTP_INT: c_uint = 0x0010  /* IM_OTP_INT */;
pub const WM831X_IM_OTP_INT_MASK: c_uint = 0x0010  /* IM_OTP_INT */;

pub const WM831X_IM_CHILD_INT: c_uint = 0x0008  /* IM_CHILD_INT */;
pub const WM831X_IM_CHILD_INT_MASK: c_uint = 0x0008  /* IM_CHILD_INT */;

pub const WM831X_IM_CHG_INT: c_uint = 0x0004  /* IM_CHG_INT */;
pub const WM831X_IM_CHG_INT_MASK: c_uint = 0x0004  /* IM_CHG_INT */;

pub const WM831X_IM_HC_INT: c_uint = 0x0002  /* IM_HC_INT */;
pub const WM831X_IM_HC_INT_MASK: c_uint = 0x0002  /* IM_HC_INT */;

pub const WM831X_IM_UV_INT: c_uint = 0x0001  /* IM_UV_INT */;
pub const WM831X_IM_UV_INT_MASK: c_uint = 0x0001  /* IM_UV_INT */;

//
// R16409 (0x4019) - Interrupt Status 1 Mask
//
pub const WM831X_IM_PPM_SYSLO_EINT: c_uint = 0x8000  /* IM_PPM_SYSLO_EINT */;
pub const WM831X_IM_PPM_SYSLO_EINT_MASK: c_uint = 0x8000  /* IM_PPM_SYSLO_EINT */;

pub const WM831X_IM_PPM_PWR_SRC_EINT: c_uint = 0x4000  /* IM_PPM_PWR_SRC_EINT */;
pub const WM831X_IM_PPM_PWR_SRC_EINT_MASK: c_uint = 0x4000  /* IM_PPM_PWR_SRC_EINT */;

pub const WM831X_IM_PPM_USB_CURR_EINT: c_uint = 0x2000  /* IM_PPM_USB_CURR_EINT */;
pub const WM831X_IM_PPM_USB_CURR_EINT_MASK: c_uint = 0x2000  /* IM_PPM_USB_CURR_EINT */;

pub const WM831X_IM_ON_PIN_EINT: c_uint = 0x1000  /* IM_ON_PIN_EINT */;
pub const WM831X_IM_ON_PIN_EINT_MASK: c_uint = 0x1000  /* IM_ON_PIN_EINT */;

pub const WM831X_IM_WDOG_TO_EINT: c_uint = 0x0800  /* IM_WDOG_TO_EINT */;
pub const WM831X_IM_WDOG_TO_EINT_MASK: c_uint = 0x0800  /* IM_WDOG_TO_EINT */;

pub const WM831X_IM_TCHDATA_EINT: c_uint = 0x0400  /* IM_TCHDATA_EINT */;
pub const WM831X_IM_TCHDATA_EINT_MASK: c_uint = 0x0400  /* IM_TCHDATA_EINT */;

pub const WM831X_IM_TCHPD_EINT: c_uint = 0x0200  /* IM_TCHPD_EINT */;
pub const WM831X_IM_TCHPD_EINT_MASK: c_uint = 0x0200  /* IM_TCHPD_EINT */;

pub const WM831X_IM_AUXADC_DATA_EINT: c_uint = 0x0100  /* IM_AUXADC_DATA_EINT */;
pub const WM831X_IM_AUXADC_DATA_EINT_MASK: c_uint = 0x0100  /* IM_AUXADC_DATA_EINT */;

pub const WM831X_IM_AUXADC_DCOMP4_EINT: c_uint = 0x0080  /* IM_AUXADC_DCOMP4_EINT */;
pub const WM831X_IM_AUXADC_DCOMP4_EINT_MASK: c_uint = 0x0080  /* IM_AUXADC_DCOMP4_EINT */;

pub const WM831X_IM_AUXADC_DCOMP3_EINT: c_uint = 0x0040  /* IM_AUXADC_DCOMP3_EINT */;
pub const WM831X_IM_AUXADC_DCOMP3_EINT_MASK: c_uint = 0x0040  /* IM_AUXADC_DCOMP3_EINT */;

pub const WM831X_IM_AUXADC_DCOMP2_EINT: c_uint = 0x0020  /* IM_AUXADC_DCOMP2_EINT */;
pub const WM831X_IM_AUXADC_DCOMP2_EINT_MASK: c_uint = 0x0020  /* IM_AUXADC_DCOMP2_EINT */;

pub const WM831X_IM_AUXADC_DCOMP1_EINT: c_uint = 0x0010  /* IM_AUXADC_DCOMP1_EINT */;
pub const WM831X_IM_AUXADC_DCOMP1_EINT_MASK: c_uint = 0x0010  /* IM_AUXADC_DCOMP1_EINT */;

pub const WM831X_IM_RTC_PER_EINT: c_uint = 0x0008  /* IM_RTC_PER_EINT */;
pub const WM831X_IM_RTC_PER_EINT_MASK: c_uint = 0x0008  /* IM_RTC_PER_EINT */;

pub const WM831X_IM_RTC_ALM_EINT: c_uint = 0x0004  /* IM_RTC_ALM_EINT */;
pub const WM831X_IM_RTC_ALM_EINT_MASK: c_uint = 0x0004  /* IM_RTC_ALM_EINT */;

pub const WM831X_IM_TEMP_THW_EINT: c_uint = 0x0002  /* IM_TEMP_THW_EINT */;
pub const WM831X_IM_TEMP_THW_EINT_MASK: c_uint = 0x0002  /* IM_TEMP_THW_EINT */;

//
// R16410 (0x401A) - Interrupt Status 2 Mask
//
pub const WM831X_IM_CHG_BATT_HOT_EINT: c_uint = 0x8000  /* IM_CHG_BATT_HOT_EINT */;
pub const WM831X_IM_CHG_BATT_HOT_EINT_MASK: c_uint = 0x8000  /* IM_CHG_BATT_HOT_EINT */;

pub const WM831X_IM_CHG_BATT_COLD_EINT: c_uint = 0x4000  /* IM_CHG_BATT_COLD_EINT */;
pub const WM831X_IM_CHG_BATT_COLD_EINT_MASK: c_uint = 0x4000  /* IM_CHG_BATT_COLD_EINT */;

pub const WM831X_IM_CHG_BATT_FAIL_EINT: c_uint = 0x2000  /* IM_CHG_BATT_FAIL_EINT */;
pub const WM831X_IM_CHG_BATT_FAIL_EINT_MASK: c_uint = 0x2000  /* IM_CHG_BATT_FAIL_EINT */;

pub const WM831X_IM_CHG_OV_EINT: c_uint = 0x1000  /* IM_CHG_OV_EINT */;
pub const WM831X_IM_CHG_OV_EINT_MASK: c_uint = 0x1000  /* IM_CHG_OV_EINT */;

pub const WM831X_IM_CHG_END_EINT: c_uint = 0x0800  /* IM_CHG_END_EINT */;
pub const WM831X_IM_CHG_END_EINT_MASK: c_uint = 0x0800  /* IM_CHG_END_EINT */;

pub const WM831X_IM_CHG_TO_EINT: c_uint = 0x0400  /* IM_CHG_TO_EINT */;
pub const WM831X_IM_CHG_TO_EINT_MASK: c_uint = 0x0400  /* IM_CHG_TO_EINT */;

pub const WM831X_IM_CHG_MODE_EINT: c_uint = 0x0200  /* IM_CHG_MODE_EINT */;
pub const WM831X_IM_CHG_MODE_EINT_MASK: c_uint = 0x0200  /* IM_CHG_MODE_EINT */;

pub const WM831X_IM_CHG_START_EINT: c_uint = 0x0100  /* IM_CHG_START_EINT */;
pub const WM831X_IM_CHG_START_EINT_MASK: c_uint = 0x0100  /* IM_CHG_START_EINT */;

pub const WM831X_IM_CS2_EINT: c_uint = 0x0080  /* IM_CS2_EINT */;
pub const WM831X_IM_CS2_EINT_MASK: c_uint = 0x0080  /* IM_CS2_EINT */;

pub const WM831X_IM_CS1_EINT: c_uint = 0x0040  /* IM_CS1_EINT */;
pub const WM831X_IM_CS1_EINT_MASK: c_uint = 0x0040  /* IM_CS1_EINT */;

pub const WM831X_IM_OTP_CMD_END_EINT: c_uint = 0x0020  /* IM_OTP_CMD_END_EINT */;
pub const WM831X_IM_OTP_CMD_END_EINT_MASK: c_uint = 0x0020  /* IM_OTP_CMD_END_EINT */;

pub const WM831X_IM_OTP_ERR_EINT: c_uint = 0x0010  /* IM_OTP_ERR_EINT */;
pub const WM831X_IM_OTP_ERR_EINT_MASK: c_uint = 0x0010  /* IM_OTP_ERR_EINT */;

pub const WM831X_IM_PS_POR_EINT: c_uint = 0x0004  /* IM_PS_POR_EINT */;
pub const WM831X_IM_PS_POR_EINT_MASK: c_uint = 0x0004  /* IM_PS_POR_EINT */;

pub const WM831X_IM_PS_SLEEP_OFF_EINT: c_uint = 0x0002  /* IM_PS_SLEEP_OFF_EINT */;
pub const WM831X_IM_PS_SLEEP_OFF_EINT_MASK: c_uint = 0x0002  /* IM_PS_SLEEP_OFF_EINT */;

pub const WM831X_IM_PS_ON_WAKE_EINT: c_uint = 0x0001  /* IM_PS_ON_WAKE_EINT */;
pub const WM831X_IM_PS_ON_WAKE_EINT_MASK: c_uint = 0x0001  /* IM_PS_ON_WAKE_EINT */;

//
// R16411 (0x401B) - Interrupt Status 3 Mask
//
pub const WM831X_IM_UV_LDO10_EINT: c_uint = 0x0200  /* IM_UV_LDO10_EINT */;
pub const WM831X_IM_UV_LDO10_EINT_MASK: c_uint = 0x0200  /* IM_UV_LDO10_EINT */;

pub const WM831X_IM_UV_LDO9_EINT: c_uint = 0x0100  /* IM_UV_LDO9_EINT */;
pub const WM831X_IM_UV_LDO9_EINT_MASK: c_uint = 0x0100  /* IM_UV_LDO9_EINT */;

pub const WM831X_IM_UV_LDO8_EINT: c_uint = 0x0080  /* IM_UV_LDO8_EINT */;
pub const WM831X_IM_UV_LDO8_EINT_MASK: c_uint = 0x0080  /* IM_UV_LDO8_EINT */;

pub const WM831X_IM_UV_LDO7_EINT: c_uint = 0x0040  /* IM_UV_LDO7_EINT */;
pub const WM831X_IM_UV_LDO7_EINT_MASK: c_uint = 0x0040  /* IM_UV_LDO7_EINT */;

pub const WM831X_IM_UV_LDO6_EINT: c_uint = 0x0020  /* IM_UV_LDO6_EINT */;
pub const WM831X_IM_UV_LDO6_EINT_MASK: c_uint = 0x0020  /* IM_UV_LDO6_EINT */;

pub const WM831X_IM_UV_LDO5_EINT: c_uint = 0x0010  /* IM_UV_LDO5_EINT */;
pub const WM831X_IM_UV_LDO5_EINT_MASK: c_uint = 0x0010  /* IM_UV_LDO5_EINT */;

pub const WM831X_IM_UV_LDO4_EINT: c_uint = 0x0008  /* IM_UV_LDO4_EINT */;
pub const WM831X_IM_UV_LDO4_EINT_MASK: c_uint = 0x0008  /* IM_UV_LDO4_EINT */;

pub const WM831X_IM_UV_LDO3_EINT: c_uint = 0x0004  /* IM_UV_LDO3_EINT */;
pub const WM831X_IM_UV_LDO3_EINT_MASK: c_uint = 0x0004  /* IM_UV_LDO3_EINT */;

pub const WM831X_IM_UV_LDO2_EINT: c_uint = 0x0002  /* IM_UV_LDO2_EINT */;
pub const WM831X_IM_UV_LDO2_EINT_MASK: c_uint = 0x0002  /* IM_UV_LDO2_EINT */;

pub const WM831X_IM_UV_LDO1_EINT: c_uint = 0x0001  /* IM_UV_LDO1_EINT */;
pub const WM831X_IM_UV_LDO1_EINT_MASK: c_uint = 0x0001  /* IM_UV_LDO1_EINT */;

//
// R16412 (0x401C) - Interrupt Status 4 Mask
//
pub const WM831X_IM_HC_DC2_EINT: c_uint = 0x0200  /* IM_HC_DC2_EINT */;
pub const WM831X_IM_HC_DC2_EINT_MASK: c_uint = 0x0200  /* IM_HC_DC2_EINT */;

pub const WM831X_IM_HC_DC1_EINT: c_uint = 0x0100  /* IM_HC_DC1_EINT */;
pub const WM831X_IM_HC_DC1_EINT_MASK: c_uint = 0x0100  /* IM_HC_DC1_EINT */;

pub const WM831X_IM_UV_DC4_EINT: c_uint = 0x0008  /* IM_UV_DC4_EINT */;
pub const WM831X_IM_UV_DC4_EINT_MASK: c_uint = 0x0008  /* IM_UV_DC4_EINT */;

pub const WM831X_IM_UV_DC3_EINT: c_uint = 0x0004  /* IM_UV_DC3_EINT */;
pub const WM831X_IM_UV_DC3_EINT_MASK: c_uint = 0x0004  /* IM_UV_DC3_EINT */;

pub const WM831X_IM_UV_DC2_EINT: c_uint = 0x0002  /* IM_UV_DC2_EINT */;
pub const WM831X_IM_UV_DC2_EINT_MASK: c_uint = 0x0002  /* IM_UV_DC2_EINT */;

pub const WM831X_IM_UV_DC1_EINT: c_uint = 0x0001  /* IM_UV_DC1_EINT */;
pub const WM831X_IM_UV_DC1_EINT_MASK: c_uint = 0x0001  /* IM_UV_DC1_EINT */;

//
// R16413 (0x401D) - Interrupt Status 5 Mask
//
pub const WM831X_IM_GP16_EINT: c_uint = 0x8000  /* IM_GP16_EINT */;
pub const WM831X_IM_GP16_EINT_MASK: c_uint = 0x8000  /* IM_GP16_EINT */;

pub const WM831X_IM_GP15_EINT: c_uint = 0x4000  /* IM_GP15_EINT */;
pub const WM831X_IM_GP15_EINT_MASK: c_uint = 0x4000  /* IM_GP15_EINT */;

pub const WM831X_IM_GP14_EINT: c_uint = 0x2000  /* IM_GP14_EINT */;
pub const WM831X_IM_GP14_EINT_MASK: c_uint = 0x2000  /* IM_GP14_EINT */;

pub const WM831X_IM_GP13_EINT: c_uint = 0x1000  /* IM_GP13_EINT */;
pub const WM831X_IM_GP13_EINT_MASK: c_uint = 0x1000  /* IM_GP13_EINT */;

pub const WM831X_IM_GP12_EINT: c_uint = 0x0800  /* IM_GP12_EINT */;
pub const WM831X_IM_GP12_EINT_MASK: c_uint = 0x0800  /* IM_GP12_EINT */;

pub const WM831X_IM_GP11_EINT: c_uint = 0x0400  /* IM_GP11_EINT */;
pub const WM831X_IM_GP11_EINT_MASK: c_uint = 0x0400  /* IM_GP11_EINT */;

pub const WM831X_IM_GP10_EINT: c_uint = 0x0200  /* IM_GP10_EINT */;
pub const WM831X_IM_GP10_EINT_MASK: c_uint = 0x0200  /* IM_GP10_EINT */;

pub const WM831X_IM_GP9_EINT: c_uint = 0x0100  /* IM_GP9_EINT */;
pub const WM831X_IM_GP9_EINT_MASK: c_uint = 0x0100  /* IM_GP9_EINT */;

pub const WM831X_IM_GP8_EINT: c_uint = 0x0080  /* IM_GP8_EINT */;
pub const WM831X_IM_GP8_EINT_MASK: c_uint = 0x0080  /* IM_GP8_EINT */;

pub const WM831X_IM_GP7_EINT: c_uint = 0x0040  /* IM_GP7_EINT */;
pub const WM831X_IM_GP7_EINT_MASK: c_uint = 0x0040  /* IM_GP7_EINT */;

pub const WM831X_IM_GP6_EINT: c_uint = 0x0020  /* IM_GP6_EINT */;
pub const WM831X_IM_GP6_EINT_MASK: c_uint = 0x0020  /* IM_GP6_EINT */;

pub const WM831X_IM_GP5_EINT: c_uint = 0x0010  /* IM_GP5_EINT */;
pub const WM831X_IM_GP5_EINT_MASK: c_uint = 0x0010  /* IM_GP5_EINT */;

pub const WM831X_IM_GP4_EINT: c_uint = 0x0008  /* IM_GP4_EINT */;
pub const WM831X_IM_GP4_EINT_MASK: c_uint = 0x0008  /* IM_GP4_EINT */;

pub const WM831X_IM_GP3_EINT: c_uint = 0x0004  /* IM_GP3_EINT */;
pub const WM831X_IM_GP3_EINT_MASK: c_uint = 0x0004  /* IM_GP3_EINT */;

pub const WM831X_IM_GP2_EINT: c_uint = 0x0002  /* IM_GP2_EINT */;
pub const WM831X_IM_GP2_EINT_MASK: c_uint = 0x0002  /* IM_GP2_EINT */;

pub const WM831X_IM_GP1_EINT: c_uint = 0x0001  /* IM_GP1_EINT */;
pub const WM831X_IM_GP1_EINT_MASK: c_uint = 0x0001  /* IM_GP1_EINT */;

