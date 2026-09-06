//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/aspeed,ast2700-scu.h
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
// Device Tree binding constants for AST2700 reset controller.
//
// Copyright (c) 2024 Aspeed Technology Inc.
//
// SOC0
pub const SCU0_RESET_SDRAM: c_int = 0;
pub const SCU0_RESET_DDRPHY: c_int = 1;
pub const SCU0_RESET_RSA: c_int = 2;
pub const SCU0_RESET_SHA3: c_int = 3;
pub const SCU0_RESET_HACE: c_int = 4;
pub const SCU0_RESET_SOC: c_int = 5;
pub const SCU0_RESET_VIDEO: c_int = 6;
pub const SCU0_RESET_2D: c_int = 7;
pub const SCU0_RESET_PCIS: c_int = 8;
pub const SCU0_RESET_RVAS0: c_int = 9;
pub const SCU0_RESET_RVAS1: c_int = 10;
pub const SCU0_RESET_SM3: c_int = 11;
pub const SCU0_RESET_SM4: c_int = 12;
pub const SCU0_RESET_CRT0: c_int = 13;
pub const SCU0_RESET_ECC: c_int = 14;
pub const SCU0_RESET_DP_PCI: c_int = 15;
pub const SCU0_RESET_UFS: c_int = 16;
pub const SCU0_RESET_EMMC: c_int = 17;
pub const SCU0_RESET_PCIE1RST: c_int = 18;
pub const SCU0_RESET_PCIE1RSTOE: c_int = 19;
pub const SCU0_RESET_PCIE0RST: c_int = 20;
pub const SCU0_RESET_PCIE0RSTOE: c_int = 21;
pub const SCU0_RESET_JTAG: c_int = 22;
pub const SCU0_RESET_MCTP0: c_int = 23;
pub const SCU0_RESET_MCTP1: c_int = 24;
pub const SCU0_RESET_XDMA0: c_int = 25;
pub const SCU0_RESET_XDMA1: c_int = 26;
pub const SCU0_RESET_H2X1: c_int = 27;
pub const SCU0_RESET_DP: c_int = 28;
pub const SCU0_RESET_DP_MCU: c_int = 29;
pub const SCU0_RESET_SSP: c_int = 30;
pub const SCU0_RESET_H2X0: c_int = 31;
pub const SCU0_RESET_PORTA_VHUB: c_int = 32;
pub const SCU0_RESET_PORTA_PHY3: c_int = 33;
pub const SCU0_RESET_PORTA_XHCI: c_int = 34;
pub const SCU0_RESET_PORTB_VHUB: c_int = 35;
pub const SCU0_RESET_PORTB_PHY3: c_int = 36;
pub const SCU0_RESET_PORTB_XHCI: c_int = 37;
pub const SCU0_RESET_PORTA_VHUB_EHCI: c_int = 38;
pub const SCU0_RESET_PORTB_VHUB_EHCI: c_int = 39;
pub const SCU0_RESET_UHCI: c_int = 40;
pub const SCU0_RESET_TSP: c_int = 41;
pub const SCU0_RESET_E2M0: c_int = 42;
pub const SCU0_RESET_E2M1: c_int = 43;
pub const SCU0_RESET_VLINK: c_int = 44;
// SOC1
pub const SCU1_RESET_LPC0: c_int = 0;
pub const SCU1_RESET_LPC1: c_int = 1;
pub const SCU1_RESET_MII: c_int = 2;
pub const SCU1_RESET_PECI: c_int = 3;
pub const SCU1_RESET_PWM: c_int = 4;
pub const SCU1_RESET_MAC0: c_int = 5;
pub const SCU1_RESET_MAC1: c_int = 6;
pub const SCU1_RESET_MAC2: c_int = 7;
pub const SCU1_RESET_ADC: c_int = 8;
pub const SCU1_RESET_SD: c_int = 9;
pub const SCU1_RESET_ESPI0: c_int = 10;
pub const SCU1_RESET_ESPI1: c_int = 11;
pub const SCU1_RESET_JTAG1: c_int = 12;
pub const SCU1_RESET_SPI0: c_int = 13;
pub const SCU1_RESET_SPI1: c_int = 14;
pub const SCU1_RESET_SPI2: c_int = 15;
pub const SCU1_RESET_I3C0: c_int = 16;
pub const SCU1_RESET_I3C1: c_int = 17;
pub const SCU1_RESET_I3C2: c_int = 18;
pub const SCU1_RESET_I3C3: c_int = 19;
pub const SCU1_RESET_I3C4: c_int = 20;
pub const SCU1_RESET_I3C5: c_int = 21;
pub const SCU1_RESET_I3C6: c_int = 22;
pub const SCU1_RESET_I3C7: c_int = 23;
pub const SCU1_RESET_I3C8: c_int = 24;
pub const SCU1_RESET_I3C9: c_int = 25;
pub const SCU1_RESET_I3C10: c_int = 26;
pub const SCU1_RESET_I3C11: c_int = 27;
pub const SCU1_RESET_I3C12: c_int = 28;
pub const SCU1_RESET_I3C13: c_int = 29;
pub const SCU1_RESET_I3C14: c_int = 30;
pub const SCU1_RESET_I3C15: c_int = 31;
pub const SCU1_RESET_MCU0: c_int = 32;
pub const SCU1_RESET_MCU1: c_int = 33;
pub const SCU1_RESET_H2A_SPI1: c_int = 34;
pub const SCU1_RESET_H2A_SPI2: c_int = 35;
pub const SCU1_RESET_UART0: c_int = 36;
pub const SCU1_RESET_UART1: c_int = 37;
pub const SCU1_RESET_UART2: c_int = 38;
pub const SCU1_RESET_UART3: c_int = 39;
pub const SCU1_RESET_I2C_FILTER: c_int = 40;
pub const SCU1_RESET_CALIPTRA: c_int = 41;
pub const SCU1_RESET_XDMA: c_int = 42;
pub const SCU1_RESET_FSI: c_int = 43;
pub const SCU1_RESET_CAN: c_int = 44;
pub const SCU1_RESET_MCTP: c_int = 45;
pub const SCU1_RESET_I2C: c_int = 46;
pub const SCU1_RESET_UART6: c_int = 47;
pub const SCU1_RESET_UART7: c_int = 48;
pub const SCU1_RESET_UART8: c_int = 49;
pub const SCU1_RESET_UART9: c_int = 50;
pub const SCU1_RESET_LTPI0: c_int = 51;
pub const SCU1_RESET_VGAL: c_int = 52;
pub const SCU1_RESET_LTPI1: c_int = 53;
pub const SCU1_RESET_ACE: c_int = 54;
pub const SCU1_RESET_E2M: c_int = 55;
pub const SCU1_RESET_UHCI: c_int = 56;
pub const SCU1_RESET_PORTC_USB2UART: c_int = 57;
pub const SCU1_RESET_PORTC_VHUB_EHCI: c_int = 58;
pub const SCU1_RESET_PORTD_USB2UART: c_int = 59;
pub const SCU1_RESET_PORTD_VHUB_EHCI: c_int = 60;
pub const SCU1_RESET_H2X: c_int = 61;
pub const SCU1_RESET_I3CDMA: c_int = 62;
pub const SCU1_RESET_PCIE2RST: c_int = 63;
