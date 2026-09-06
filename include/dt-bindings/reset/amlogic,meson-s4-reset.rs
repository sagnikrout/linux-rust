//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson-s4-reset.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (c) 2021 Amlogic, Inc. All rights reserved.
// Author: Zelong Dong <zelong.dong@amlogic.com>
//
// RESET0
pub const RESET_USB_DDR0: c_int = 0;
pub const RESET_USB_DDR1: c_int = 1;
pub const RESET_USB_DDR2: c_int = 2;
pub const RESET_USB_DDR3: c_int = 3;
pub const RESET_USBCTRL: c_int = 4;
// 5-7
pub const RESET_USBPHY20: c_int = 8;
pub const RESET_USBPHY21: c_int = 9;
// 10-15
pub const RESET_HDMITX_APB: c_int = 16;
pub const RESET_BRG_VCBUS_DEC: c_int = 17;
pub const RESET_VCBUS: c_int = 18;
pub const RESET_VID_PLL_DIV: c_int = 19;
pub const RESET_VDI6: c_int = 20;
pub const RESET_GE2D: c_int = 21;
pub const RESET_HDMITXPHY: c_int = 22;
pub const RESET_VID_LOCK: c_int = 23;
pub const RESET_VENCL: c_int = 24;
pub const RESET_VDAC: c_int = 25;
pub const RESET_VENCP: c_int = 26;
pub const RESET_VENCI: c_int = 27;
pub const RESET_RDMA: c_int = 28;
pub const RESET_HDMI_TX: c_int = 29;
pub const RESET_VIU: c_int = 30;
pub const RESET_VENC: c_int = 31;
// RESET1
pub const RESET_AUDIO: c_int = 32;
pub const RESET_MALI_APB: c_int = 33;
pub const RESET_MALI: c_int = 34;
pub const RESET_DDR_APB: c_int = 35;
pub const RESET_DDR: c_int = 36;
pub const RESET_DOS_APB: c_int = 37;
pub const RESET_DOS: c_int = 38;
// 39-47
pub const RESET_ETH: c_int = 48;
// 49-51
pub const RESET_DEMOD: c_int = 52;
// 53-63
// RESET2
pub const RESET_ABUS_ARB: c_int = 64;
pub const RESET_IR_CTRL: c_int = 65;
pub const RESET_TEMPSENSOR_DDR: c_int = 66;
pub const RESET_TEMPSENSOR_PLL: c_int = 67;
// 68-71
pub const RESET_SMART_CARD: c_int = 72;
pub const RESET_SPICC0: c_int = 73;
// 74
pub const RESET_RSA: c_int = 75;
// 76-79
pub const RESET_MSR_CLK: c_int = 80;
pub const RESET_SPIFC: c_int = 81;
pub const RESET_SARADC: c_int = 82;
// 83-87
pub const RESET_ACODEC: c_int = 88;
pub const RESET_CEC: c_int = 89;
pub const RESET_AFIFO: c_int = 90;
pub const RESET_WATCHDOG: c_int = 91;
// 92-95
// RESET3
// 96-127
// RESET4
// 128-131
pub const RESET_PWM_AB: c_int = 132;
pub const RESET_PWM_CD: c_int = 133;
pub const RESET_PWM_EF: c_int = 134;
pub const RESET_PWM_GH: c_int = 135;
pub const RESET_PWM_IJ: c_int = 136;
// 137
pub const RESET_UART_A: c_int = 138;
pub const RESET_UART_B: c_int = 139;
pub const RESET_UART_C: c_int = 140;
pub const RESET_UART_D: c_int = 141;
pub const RESET_UART_E: c_int = 142;
// 143
pub const RESET_I2C_S_A: c_int = 144;
pub const RESET_I2C_M_A: c_int = 145;
pub const RESET_I2C_M_B: c_int = 146;
pub const RESET_I2C_M_C: c_int = 147;
pub const RESET_I2C_M_D: c_int = 148;
pub const RESET_I2C_M_E: c_int = 149;
// 150-151
pub const RESET_SD_EMMC_A: c_int = 152;
pub const RESET_SD_EMMC_B: c_int = 153;
pub const RESET_NAND_EMMC: c_int = 154;
// 155-159
// RESET5
pub const RESET_BRG_VDEC_PIPL0: c_int = 160;
pub const RESET_BRG_HEVCF_PIPL0: c_int = 161;
// 162
pub const RESET_BRG_HCODEC_PIPL0: c_int = 163;
pub const RESET_BRG_GE2D_PIPL0: c_int = 164;
pub const RESET_BRG_VPU_PIPL0: c_int = 165;
pub const RESET_BRG_CPU_PIPL0: c_int = 166;
pub const RESET_BRG_MALI_PIPL0: c_int = 167;
// 168
pub const RESET_BRG_MALI_PIPL1: c_int = 169;
// 170-171
pub const RESET_BRG_HEVCF_PIPL1: c_int = 172;
pub const RESET_BRG_HEVCB_PIPL1: c_int = 173;
// 174-183
pub const RESET_RAMA: c_int = 184;
// 185-186
pub const RESET_BRG_NIC_VAPB: c_int = 187;
pub const RESET_BRG_NIC_DSU: c_int = 188;
pub const RESET_BRG_NIC_SYSCLK: c_int = 189;
pub const RESET_BRG_NIC_MAIN: c_int = 190;
pub const RESET_BRG_NIC_ALL: c_int = 191;
