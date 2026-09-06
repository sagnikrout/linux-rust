//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/marvell,mmp2.h
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
// fixed clocks and plls
pub const MMP2_CLK_CLK32: c_int = 1;
pub const MMP2_CLK_VCTCXO: c_int = 2;
pub const MMP2_CLK_PLL1: c_int = 3;
pub const MMP2_CLK_PLL1_2: c_int = 8;
pub const MMP2_CLK_PLL1_4: c_int = 9;
pub const MMP2_CLK_PLL1_8: c_int = 10;
pub const MMP2_CLK_PLL1_16: c_int = 11;
pub const MMP2_CLK_PLL1_3: c_int = 12;
pub const MMP2_CLK_PLL1_6: c_int = 13;
pub const MMP2_CLK_PLL1_12: c_int = 14;
pub const MMP2_CLK_PLL1_20: c_int = 15;
pub const MMP2_CLK_PLL2: c_int = 16;
pub const MMP2_CLK_PLL2_2: c_int = 17;
pub const MMP2_CLK_PLL2_4: c_int = 18;
pub const MMP2_CLK_PLL2_8: c_int = 19;
pub const MMP2_CLK_PLL2_16: c_int = 20;
pub const MMP2_CLK_PLL2_3: c_int = 21;
pub const MMP2_CLK_PLL2_6: c_int = 22;
pub const MMP2_CLK_PLL2_12: c_int = 23;
pub const MMP2_CLK_VCTCXO_2: c_int = 24;
pub const MMP2_CLK_VCTCXO_4: c_int = 25;
pub const MMP2_CLK_UART_PLL: c_int = 26;
pub const MMP2_CLK_USB_PLL: c_int = 27;
pub const MMP3_CLK_PLL1_P: c_int = 28;
pub const MMP3_CLK_PLL2_P: c_int = 29;
pub const MMP3_CLK_PLL3: c_int = 30;
pub const MMP2_CLK_I2S0: c_int = 31;
pub const MMP2_CLK_I2S1: c_int = 32;
// apb peripherals
pub const MMP2_CLK_TWSI0: c_int = 60;
pub const MMP2_CLK_TWSI1: c_int = 61;
pub const MMP2_CLK_TWSI2: c_int = 62;
pub const MMP2_CLK_TWSI3: c_int = 63;
pub const MMP2_CLK_TWSI4: c_int = 64;
pub const MMP2_CLK_TWSI5: c_int = 65;
pub const MMP2_CLK_GPIO: c_int = 66;
pub const MMP2_CLK_KPC: c_int = 67;
pub const MMP2_CLK_RTC: c_int = 68;
pub const MMP2_CLK_PWM0: c_int = 69;
pub const MMP2_CLK_PWM1: c_int = 70;
pub const MMP2_CLK_PWM2: c_int = 71;
pub const MMP2_CLK_PWM3: c_int = 72;
pub const MMP2_CLK_UART0: c_int = 73;
pub const MMP2_CLK_UART1: c_int = 74;
pub const MMP2_CLK_UART2: c_int = 75;
pub const MMP2_CLK_UART3: c_int = 76;
pub const MMP2_CLK_SSP0: c_int = 77;
pub const MMP2_CLK_SSP1: c_int = 78;
pub const MMP2_CLK_SSP2: c_int = 79;
pub const MMP2_CLK_SSP3: c_int = 80;
pub const MMP2_CLK_TIMER: c_int = 81;
pub const MMP2_CLK_THERMAL0: c_int = 82;
pub const MMP3_CLK_THERMAL1: c_int = 83;
pub const MMP3_CLK_THERMAL2: c_int = 84;
pub const MMP3_CLK_THERMAL3: c_int = 85;
// axi peripherals
pub const MMP2_CLK_SDH0: c_int = 101;
pub const MMP2_CLK_SDH1: c_int = 102;
pub const MMP2_CLK_SDH2: c_int = 103;
pub const MMP2_CLK_SDH3: c_int = 104;
pub const MMP2_CLK_USB: c_int = 105;
pub const MMP2_CLK_DISP0: c_int = 106;
pub const MMP2_CLK_DISP0_MUX: c_int = 107;
pub const MMP2_CLK_DISP0_SPHY: c_int = 108;
pub const MMP2_CLK_DISP1: c_int = 109;
pub const MMP2_CLK_DISP1_MUX: c_int = 110;
pub const MMP2_CLK_CCIC_ARBITER: c_int = 111;
pub const MMP2_CLK_CCIC0: c_int = 112;
pub const MMP2_CLK_CCIC0_MIX: c_int = 113;
pub const MMP2_CLK_CCIC0_PHY: c_int = 114;
pub const MMP2_CLK_CCIC0_SPHY: c_int = 115;
pub const MMP2_CLK_CCIC1: c_int = 116;
pub const MMP2_CLK_CCIC1_MIX: c_int = 117;
pub const MMP2_CLK_CCIC1_PHY: c_int = 118;
pub const MMP2_CLK_CCIC1_SPHY: c_int = 119;
pub const MMP2_CLK_DISP0_LCDC: c_int = 120;
pub const MMP2_CLK_USBHSIC0: c_int = 121;
pub const MMP2_CLK_USBHSIC1: c_int = 122;
pub const MMP2_CLK_GPU_BUS: c_int = 123;

pub const MMP2_CLK_GPU_3D: c_int = 124;

pub const MMP3_CLK_GPU_2D: c_int = 125;
pub const MMP3_CLK_SDH4: c_int = 126;
pub const MMP2_CLK_AUDIO: c_int = 127;
