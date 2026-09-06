//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/amlogic,a9-aoclkc.h
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
// Copyright (C) 2026 Amlogic, Inc. All rights reserved.
//
pub const CLKID_AO_XTAL_IN: c_int = 0;
pub const CLKID_AO_XTAL: c_int = 1;
pub const CLKID_AO_SYS: c_int = 2;
pub const CLKID_AO_SYS_I3C: c_int = 3;
pub const CLKID_AO_SYS_RTC_REG: c_int = 4;
pub const CLKID_AO_SYS_CLKTREE: c_int = 5;
pub const CLKID_AO_SYS_RST_CTRL: c_int = 6;
pub const CLKID_AO_SYS_PAD: c_int = 7;
pub const CLKID_AO_SYS_RTC_DIG: c_int = 8;
pub const CLKID_AO_SYS_IRQ: c_int = 9;
pub const CLKID_AO_SYS_PWRCTRL: c_int = 10;
pub const CLKID_AO_SYS_PWM_A: c_int = 11;
pub const CLKID_AO_SYS_PWM_B: c_int = 12;
pub const CLKID_AO_SYS_PWM_C: c_int = 13;
pub const CLKID_AO_SYS_PWM_D: c_int = 14;
pub const CLKID_AO_SYS_PWM_E: c_int = 15;
pub const CLKID_AO_SYS_PWM_F: c_int = 16;
pub const CLKID_AO_SYS_PWM_G: c_int = 17;
pub const CLKID_AO_SYS_I2C_A: c_int = 18;
pub const CLKID_AO_SYS_I2C_B: c_int = 19;
pub const CLKID_AO_SYS_I2C_C: c_int = 20;
pub const CLKID_AO_SYS_I2C_D: c_int = 21;
pub const CLKID_AO_SYS_SED: c_int = 22;
pub const CLKID_AO_SYS_IR_CTRL: c_int = 23;
pub const CLKID_AO_SYS_UART_B: c_int = 24;
pub const CLKID_AO_SYS_UART_C: c_int = 25;
pub const CLKID_AO_SYS_UART_D: c_int = 26;
pub const CLKID_AO_SYS_UART_E: c_int = 27;
pub const CLKID_AO_SYS_SPISG_0: c_int = 28;
pub const CLKID_AO_SYS_RTC_SECURE: c_int = 29;
pub const CLKID_AO_SYS_CEC: c_int = 30;
pub const CLKID_AO_SYS_AOCPU: c_int = 31;
pub const CLKID_AO_SYS_SRAM: c_int = 32;
pub const CLKID_AO_SYS_SPISG_1: c_int = 33;
pub const CLKID_AO_SYS_SPISG_2: c_int = 34;
pub const CLKID_AO_PWM_A_SEL: c_int = 35;
pub const CLKID_AO_PWM_A_DIV: c_int = 36;
pub const CLKID_AO_PWM_A: c_int = 37;
pub const CLKID_AO_PWM_B_SEL: c_int = 38;
pub const CLKID_AO_PWM_B_DIV: c_int = 39;
pub const CLKID_AO_PWM_B: c_int = 40;
pub const CLKID_AO_PWM_C_SEL: c_int = 41;
pub const CLKID_AO_PWM_C_DIV: c_int = 42;
pub const CLKID_AO_PWM_C: c_int = 43;
pub const CLKID_AO_PWM_D_SEL: c_int = 44;
pub const CLKID_AO_PWM_D_DIV: c_int = 45;
pub const CLKID_AO_PWM_D: c_int = 46;
pub const CLKID_AO_PWM_E_SEL: c_int = 47;
pub const CLKID_AO_PWM_E_DIV: c_int = 48;
pub const CLKID_AO_PWM_E: c_int = 49;
pub const CLKID_AO_PWM_F_SEL: c_int = 50;
pub const CLKID_AO_PWM_F_DIV: c_int = 51;
pub const CLKID_AO_PWM_F: c_int = 52;
pub const CLKID_AO_PWM_G_SEL: c_int = 53;
pub const CLKID_AO_PWM_G_DIV: c_int = 54;
pub const CLKID_AO_PWM_G: c_int = 55;
pub const CLKID_AO_RTC_DUALDIV_IN: c_int = 56;
pub const CLKID_AO_RTC_DUALDIV_DIV: c_int = 57;
pub const CLKID_AO_RTC_DUALDIV_SEL: c_int = 58;
pub const CLKID_AO_RTC_DUALDIV: c_int = 59;
pub const CLKID_AO_RTC: c_int = 60;
pub const CLKID_AO_CEC_DUALDIV_IN: c_int = 61;
pub const CLKID_AO_CEC_DUALDIV_DIV: c_int = 62;
pub const CLKID_AO_CEC_DUALDIV_SEL: c_int = 63;
pub const CLKID_AO_CEC_DUALDIV: c_int = 64;
pub const CLKID_AO_CEC: c_int = 65;
