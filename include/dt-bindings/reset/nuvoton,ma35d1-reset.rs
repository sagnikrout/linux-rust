//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/nuvoton,ma35d1-reset.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2023 Nuvoton Technologies.
// Author: Chi-Fen Li <cfli0@nuvoton.com>
//
// Device Tree binding constants for MA35D1 reset controller.
//
pub const MA35D1_RESET_CHIP: c_int = 0;
pub const MA35D1_RESET_CA35CR0: c_int = 1;
pub const MA35D1_RESET_CA35CR1: c_int = 2;
pub const MA35D1_RESET_CM4: c_int = 3;
pub const MA35D1_RESET_PDMA0: c_int = 4;
pub const MA35D1_RESET_PDMA1: c_int = 5;
pub const MA35D1_RESET_PDMA2: c_int = 6;
pub const MA35D1_RESET_PDMA3: c_int = 7;
pub const MA35D1_RESET_DISP: c_int = 8;
pub const MA35D1_RESET_VCAP0: c_int = 9;
pub const MA35D1_RESET_VCAP1: c_int = 10;
pub const MA35D1_RESET_GFX: c_int = 11;
pub const MA35D1_RESET_VDEC: c_int = 12;
pub const MA35D1_RESET_WHC0: c_int = 13;
pub const MA35D1_RESET_WHC1: c_int = 14;
pub const MA35D1_RESET_GMAC0: c_int = 15;
pub const MA35D1_RESET_GMAC1: c_int = 16;
pub const MA35D1_RESET_HWSEM: c_int = 17;
pub const MA35D1_RESET_EBI: c_int = 18;
pub const MA35D1_RESET_HSUSBH0: c_int = 19;
pub const MA35D1_RESET_HSUSBH1: c_int = 20;
pub const MA35D1_RESET_HSUSBD: c_int = 21;
pub const MA35D1_RESET_USBHL: c_int = 22;
pub const MA35D1_RESET_SDH0: c_int = 23;
pub const MA35D1_RESET_SDH1: c_int = 24;
pub const MA35D1_RESET_NAND: c_int = 25;
pub const MA35D1_RESET_GPIO: c_int = 26;
pub const MA35D1_RESET_MCTLP: c_int = 27;
pub const MA35D1_RESET_MCTLC: c_int = 28;
pub const MA35D1_RESET_DDRPUB: c_int = 29;
pub const MA35D1_RESET_TMR0: c_int = 30;
pub const MA35D1_RESET_TMR1: c_int = 31;
pub const MA35D1_RESET_TMR2: c_int = 32;
pub const MA35D1_RESET_TMR3: c_int = 33;
pub const MA35D1_RESET_I2C0: c_int = 34;
pub const MA35D1_RESET_I2C1: c_int = 35;
pub const MA35D1_RESET_I2C2: c_int = 36;
pub const MA35D1_RESET_I2C3: c_int = 37;
pub const MA35D1_RESET_QSPI0: c_int = 38;
pub const MA35D1_RESET_SPI0: c_int = 39;
pub const MA35D1_RESET_SPI1: c_int = 40;
pub const MA35D1_RESET_SPI2: c_int = 41;
pub const MA35D1_RESET_UART0: c_int = 42;
pub const MA35D1_RESET_UART1: c_int = 43;
pub const MA35D1_RESET_UART2: c_int = 44;
pub const MA35D1_RESET_UART3: c_int = 45;
pub const MA35D1_RESET_UART4: c_int = 46;
pub const MA35D1_RESET_UART5: c_int = 47;
pub const MA35D1_RESET_UART6: c_int = 48;
pub const MA35D1_RESET_UART7: c_int = 49;
pub const MA35D1_RESET_CANFD0: c_int = 50;
pub const MA35D1_RESET_CANFD1: c_int = 51;
pub const MA35D1_RESET_EADC0: c_int = 52;
pub const MA35D1_RESET_I2S0: c_int = 53;
pub const MA35D1_RESET_SC0: c_int = 54;
pub const MA35D1_RESET_SC1: c_int = 55;
pub const MA35D1_RESET_QSPI1: c_int = 56;
pub const MA35D1_RESET_SPI3: c_int = 57;
pub const MA35D1_RESET_EPWM0: c_int = 58;
pub const MA35D1_RESET_EPWM1: c_int = 59;
pub const MA35D1_RESET_QEI0: c_int = 60;
pub const MA35D1_RESET_QEI1: c_int = 61;
pub const MA35D1_RESET_ECAP0: c_int = 62;
pub const MA35D1_RESET_ECAP1: c_int = 63;
pub const MA35D1_RESET_CANFD2: c_int = 64;
pub const MA35D1_RESET_ADC0: c_int = 65;
pub const MA35D1_RESET_TMR4: c_int = 66;
pub const MA35D1_RESET_TMR5: c_int = 67;
pub const MA35D1_RESET_TMR6: c_int = 68;
pub const MA35D1_RESET_TMR7: c_int = 69;
pub const MA35D1_RESET_TMR8: c_int = 70;
pub const MA35D1_RESET_TMR9: c_int = 71;
pub const MA35D1_RESET_TMR10: c_int = 72;
pub const MA35D1_RESET_TMR11: c_int = 73;
pub const MA35D1_RESET_UART8: c_int = 74;
pub const MA35D1_RESET_UART9: c_int = 75;
pub const MA35D1_RESET_UART10: c_int = 76;
pub const MA35D1_RESET_UART11: c_int = 77;
pub const MA35D1_RESET_UART12: c_int = 78;
pub const MA35D1_RESET_UART13: c_int = 79;
pub const MA35D1_RESET_UART14: c_int = 80;
pub const MA35D1_RESET_UART15: c_int = 81;
pub const MA35D1_RESET_UART16: c_int = 82;
pub const MA35D1_RESET_I2S1: c_int = 83;
pub const MA35D1_RESET_I2C4: c_int = 84;
pub const MA35D1_RESET_I2C5: c_int = 85;
pub const MA35D1_RESET_EPWM2: c_int = 86;
pub const MA35D1_RESET_ECAP2: c_int = 87;
pub const MA35D1_RESET_QEI2: c_int = 88;
pub const MA35D1_RESET_CANFD3: c_int = 89;
pub const MA35D1_RESET_KPI: c_int = 90;
pub const MA35D1_RESET_GIC: c_int = 91;
pub const MA35D1_RESET_SSMCC: c_int = 92;
pub const MA35D1_RESET_SSPCC: c_int = 93;
pub const MA35D1_RESET_COUNT: c_int = 94;
