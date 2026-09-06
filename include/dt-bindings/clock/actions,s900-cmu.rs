//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/actions,s900-cmu.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Device Tree binding constants for Actions Semi S900 Clock Management Unit
//
// Copyright (c) 2014 Actions Semi Inc.
// Copyright (c) 2018 Linaro Ltd.
pub const CLK_NONE: c_int = 0;
// fixed rate clocks
pub const CLK_LOSC: c_int = 1;
pub const CLK_HOSC: c_int = 2;
// pll clocks
pub const CLK_CORE_PLL: c_int = 3;
pub const CLK_DEV_PLL: c_int = 4;
pub const CLK_DDR_PLL: c_int = 5;
pub const CLK_NAND_PLL: c_int = 6;
pub const CLK_DISPLAY_PLL: c_int = 7;
pub const CLK_DSI_PLL: c_int = 8;
pub const CLK_ASSIST_PLL: c_int = 9;
pub const CLK_AUDIO_PLL: c_int = 10;
// system clock
pub const CLK_CPU: c_int = 15;
pub const CLK_DEV: c_int = 16;
pub const CLK_NOC: c_int = 17;
pub const CLK_NOC_MUX: c_int = 18;
pub const CLK_NOC_DIV: c_int = 19;
pub const CLK_AHB: c_int = 20;
pub const CLK_APB: c_int = 21;
pub const CLK_DMAC: c_int = 22;
// peripheral device clock
pub const CLK_GPIO: c_int = 23;
pub const CLK_BISP: c_int = 24;
pub const CLK_CSI0: c_int = 25;
pub const CLK_CSI1: c_int = 26;
pub const CLK_DE0: c_int = 27;
pub const CLK_DE1: c_int = 28;
pub const CLK_DE2: c_int = 29;
pub const CLK_DE3: c_int = 30;
pub const CLK_DSI: c_int = 32;
pub const CLK_GPU: c_int = 33;
pub const CLK_GPU_CORE: c_int = 34;
pub const CLK_GPU_MEM: c_int = 35;
pub const CLK_GPU_SYS: c_int = 36;
pub const CLK_HDE: c_int = 37;
pub const CLK_I2C0: c_int = 38;
pub const CLK_I2C1: c_int = 39;
pub const CLK_I2C2: c_int = 40;
pub const CLK_I2C3: c_int = 41;
pub const CLK_I2C4: c_int = 42;
pub const CLK_I2C5: c_int = 43;
pub const CLK_I2SRX: c_int = 44;
pub const CLK_I2STX: c_int = 45;
pub const CLK_IMX: c_int = 46;
pub const CLK_LCD: c_int = 47;
pub const CLK_NAND0: c_int = 48;
pub const CLK_NAND1: c_int = 49;
pub const CLK_PWM0: c_int = 50;
pub const CLK_PWM1: c_int = 51;
pub const CLK_PWM2: c_int = 52;
pub const CLK_PWM3: c_int = 53;
pub const CLK_PWM4: c_int = 54;
pub const CLK_PWM5: c_int = 55;
pub const CLK_SD0: c_int = 56;
pub const CLK_SD1: c_int = 57;
pub const CLK_SD2: c_int = 58;
pub const CLK_SD3: c_int = 59;
pub const CLK_SENSOR: c_int = 60;
pub const CLK_SPEED_SENSOR: c_int = 61;
pub const CLK_SPI0: c_int = 62;
pub const CLK_SPI1: c_int = 63;
pub const CLK_SPI2: c_int = 64;
pub const CLK_SPI3: c_int = 65;
pub const CLK_THERMAL_SENSOR: c_int = 66;
pub const CLK_UART0: c_int = 67;
pub const CLK_UART1: c_int = 68;
pub const CLK_UART2: c_int = 69;
pub const CLK_UART3: c_int = 70;
pub const CLK_UART4: c_int = 71;
pub const CLK_UART5: c_int = 72;
pub const CLK_UART6: c_int = 73;
pub const CLK_VCE: c_int = 74;
pub const CLK_VDE: c_int = 75;
pub const CLK_USB3_480MPLL0: c_int = 76;
pub const CLK_USB3_480MPHY0: c_int = 77;
pub const CLK_USB3_5GPHY: c_int = 78;
pub const CLK_USB3_CCE: c_int = 79;
pub const CLK_USB3_MAC: c_int = 80;
pub const CLK_TIMER: c_int = 83;
pub const CLK_HDMI_AUDIO: c_int = 84;
pub const CLK_24M: c_int = 85;
pub const CLK_EDP: c_int = 86;
pub const CLK_24M_EDP: c_int = 87;
pub const CLK_EDP_PLL: c_int = 88;
pub const CLK_EDP_LINK: c_int = 89;
pub const CLK_USB2H0_PLLEN: c_int = 90;
pub const CLK_USB2H0_PHY: c_int = 91;
pub const CLK_USB2H0_CCE: c_int = 92;
pub const CLK_USB2H1_PLLEN: c_int = 93;
pub const CLK_USB2H1_PHY: c_int = 94;
pub const CLK_USB2H1_CCE: c_int = 95;
pub const CLK_DDR0: c_int = 96;
pub const CLK_DDR1: c_int = 97;
pub const CLK_DMM: c_int = 98;
pub const CLK_ETH_MAC: c_int = 99;
pub const CLK_RMII_REF: c_int = 100;

