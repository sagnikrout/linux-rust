//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,c3-reset.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (c) 2023 Amlogic, Inc. All rights reserved.
//
// RESET0
// 0-3
pub const RESET_USBCTRL: c_int = 4;
// 5-7
pub const RESET_USBPHY20: c_int = 8;
// 9
pub const RESET_USB2DRD: c_int = 10;
pub const RESET_MIPI_DSI_HOST: c_int = 11;
pub const RESET_MIPI_DSI_PHY: c_int = 12;
// 13-20
pub const RESET_GE2D: c_int = 21;
pub const RESET_DWAP: c_int = 22;
// 23-31
// RESET1
pub const RESET_AUDIO: c_int = 32;
// 33-34
pub const RESET_DDRAPB: c_int = 35;
pub const RESET_DDR: c_int = 36;
pub const RESET_DOS_CAPB3: c_int = 37;
pub const RESET_DOS: c_int = 38;
// 39-46
pub const RESET_NNA: c_int = 47;
pub const RESET_ETHERNET: c_int = 48;
pub const RESET_ISP: c_int = 49;
pub const RESET_VC9000E_APB: c_int = 50;
pub const RESET_VC9000E_A: c_int = 51;
// 52
pub const RESET_VC9000E_CORE: c_int = 53;
// 54-63
// RESET2
pub const RESET_ABUS_ARB: c_int = 64;
pub const RESET_IRCTRL: c_int = 65;
// 66
pub const RESET_TEMP_PII: c_int = 67;
// 68-72
pub const RESET_SPICC_0: c_int = 73;
pub const RESET_SPICC_1: c_int = 74;
pub const RESET_RSA: c_int = 75;
// 76-79
pub const RESET_MSR_CLK: c_int = 80;
pub const RESET_SPIFC: c_int = 81;
pub const RESET_SAR_ADC: c_int = 82;
// 83-87
pub const RESET_ACODEC: c_int = 88;
// 89-90
pub const RESET_WATCHDOG: c_int = 91;
// 92-95
// RESET3
pub const RESET_ISP_NIC_GPV: c_int = 96;
pub const RESET_ISP_NIC_MAIN: c_int = 97;
pub const RESET_ISP_NIC_VCLK: c_int = 98;
pub const RESET_ISP_NIC_VOUT: c_int = 99;
pub const RESET_ISP_NIC_ALL: c_int = 100;
pub const RESET_VOUT: c_int = 101;
pub const RESET_VOUT_VENC: c_int = 102;
// 103
pub const RESET_CVE_NIC_GPV: c_int = 104;
pub const RESET_CVE_NIC_MAIN: c_int = 105;
pub const RESET_CVE_NIC_GE2D: c_int = 106;
pub const RESET_CVE_NIC_DW: c_int = 106;
pub const RESET_CVE_NIC_CVE: c_int = 108;
pub const RESET_CVE_NIC_ALL: c_int = 109;
pub const RESET_CVE: c_int = 110;
// 112-127
// RESET4
pub const RESET_RTC: c_int = 128;
pub const RESET_PWM_AB: c_int = 129;
pub const RESET_PWM_CD: c_int = 130;
pub const RESET_PWM_EF: c_int = 131;
pub const RESET_PWM_GH: c_int = 132;
pub const RESET_PWM_IJ: c_int = 133;
pub const RESET_PWM_KL: c_int = 134;
pub const RESET_PWM_MN: c_int = 135;
// 136-137
pub const RESET_UART_A: c_int = 138;
pub const RESET_UART_B: c_int = 139;
pub const RESET_UART_C: c_int = 140;
pub const RESET_UART_D: c_int = 141;
pub const RESET_UART_E: c_int = 142;
pub const RESET_UART_F: c_int = 143;
pub const RESET_I2C_S_A: c_int = 144;
pub const RESET_I2C_M_A: c_int = 145;
pub const RESET_I2C_M_B: c_int = 146;
pub const RESET_I2C_M_C: c_int = 147;
pub const RESET_I2C_M_D: c_int = 148;
// 149-151
pub const RESET_SD_EMMC_A: c_int = 152;
pub const RESET_SD_EMMC_B: c_int = 153;
pub const RESET_SD_EMMC_C: c_int = 154;
// RESET5
// 160-172
pub const RESET_BRG_NIC_NNA: c_int = 173;
pub const RESET_BRG_MUX_NIC_MAIN: c_int = 174;
pub const RESET_BRG_AO_NIC_ALL: c_int = 175;
// 176-183
pub const RESET_BRG_NIC_VAPB: c_int = 184;
pub const RESET_BRG_NIC_SDIO_B: c_int = 185;
pub const RESET_BRG_NIC_SDIO_A: c_int = 186;
pub const RESET_BRG_NIC_EMMC: c_int = 187;
pub const RESET_BRG_NIC_DSU: c_int = 188;
pub const RESET_BRG_NIC_SYSCLK: c_int = 189;
pub const RESET_BRG_NIC_MAIN: c_int = 190;
pub const RESET_BRG_NIC_ALL: c_int = 191;
