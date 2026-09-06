//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/aspeed,ast2700-scu.h
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
// Device Tree binding constants for AST2700 clock controller.
//
// Copyright (c) 2024 Aspeed Technology Inc.
//
// SOC0 clk
pub const SCU0_CLKIN: c_int = 0;
pub const SCU0_CLK_24M: c_int = 1;
pub const SCU0_CLK_192M: c_int = 2;
pub const SCU0_CLK_UART: c_int = 3;
pub const SCU0_CLK_UART_DIV13: c_int = 3;
pub const SCU0_CLK_PSP: c_int = 4;
pub const SCU0_CLK_HPLL: c_int = 5;
pub const SCU0_CLK_HPLL_DIV2: c_int = 6;
pub const SCU0_CLK_HPLL_DIV4: c_int = 7;
pub const SCU0_CLK_HPLL_DIV_AHB: c_int = 8;
pub const SCU0_CLK_DPLL: c_int = 9;
pub const SCU0_CLK_MPLL: c_int = 10;
pub const SCU0_CLK_MPLL_DIV2: c_int = 11;
pub const SCU0_CLK_MPLL_DIV4: c_int = 12;
pub const SCU0_CLK_MPLL_DIV8: c_int = 13;
pub const SCU0_CLK_MPLL_DIV_AHB: c_int = 14;
pub const SCU0_CLK_D0: c_int = 15;
pub const SCU0_CLK_D1: c_int = 16;
pub const SCU0_CLK_CRT0: c_int = 17;
pub const SCU0_CLK_CRT1: c_int = 18;
pub const SCU0_CLK_MPHY: c_int = 19;
pub const SCU0_CLK_AXI0: c_int = 20;
pub const SCU0_CLK_AXI1: c_int = 21;
pub const SCU0_CLK_AHB: c_int = 22;
pub const SCU0_CLK_APB: c_int = 23;
pub const SCU0_CLK_UART4: c_int = 24;
pub const SCU0_CLK_EMMCMUX: c_int = 25;
pub const SCU0_CLK_EMMC: c_int = 26;
pub const SCU0_CLK_U2PHY_CLK12M: c_int = 27;
pub const SCU0_CLK_U2PHY_REFCLK: c_int = 28;
// SOC0 clk-gate
pub const SCU0_CLK_GATE_MCLK: c_int = 29;
pub const SCU0_CLK_GATE_ECLK: c_int = 30;
pub const SCU0_CLK_GATE_2DCLK: c_int = 31;
pub const SCU0_CLK_GATE_VCLK: c_int = 32;
pub const SCU0_CLK_GATE_BCLK: c_int = 33;
pub const SCU0_CLK_GATE_VGA0CLK: c_int = 34;
pub const SCU0_CLK_GATE_REFCLK: c_int = 35;
pub const SCU0_CLK_GATE_PORTBUSB2CLK: c_int = 36;
pub const SCU0_CLK_GATE_UHCICLK: c_int = 37;
pub const SCU0_CLK_GATE_VGA1CLK: c_int = 38;
pub const SCU0_CLK_GATE_DDRPHYCLK: c_int = 39;
pub const SCU0_CLK_GATE_E2M0CLK: c_int = 40;
pub const SCU0_CLK_GATE_HACCLK: c_int = 41;
pub const SCU0_CLK_GATE_PORTAUSB2CLK: c_int = 42;
pub const SCU0_CLK_GATE_UART4CLK: c_int = 43;
pub const SCU0_CLK_GATE_SLICLK: c_int = 44;
pub const SCU0_CLK_GATE_DACCLK: c_int = 45;
pub const SCU0_CLK_GATE_DP: c_int = 46;
pub const SCU0_CLK_GATE_E2M1CLK: c_int = 47;
pub const SCU0_CLK_GATE_CRT0CLK: c_int = 48;
pub const SCU0_CLK_GATE_CRT1CLK: c_int = 49;
pub const SCU0_CLK_GATE_ECDSACLK: c_int = 50;
pub const SCU0_CLK_GATE_RSACLK: c_int = 51;
pub const SCU0_CLK_GATE_RVAS0CLK: c_int = 52;
pub const SCU0_CLK_GATE_UFSCLK: c_int = 53;
pub const SCU0_CLK_GATE_EMMCCLK: c_int = 54;
pub const SCU0_CLK_GATE_RVAS1CLK: c_int = 55;
pub const SCU0_CLK_U2PHY_REFCLKSRC: c_int = 56;
pub const SCU0_CLK_AHBMUX: c_int = 57;
pub const SCU0_CLK_MPHYSRC: c_int = 58;
// SOC1 clk
pub const SCU1_CLKIN: c_int = 0;
pub const SCU1_CLK_HPLL: c_int = 1;
pub const SCU1_CLK_APLL: c_int = 2;
pub const SCU1_CLK_APLL_DIV2: c_int = 3;
pub const SCU1_CLK_APLL_DIV4: c_int = 4;
pub const SCU1_CLK_DPLL: c_int = 5;
pub const SCU1_CLK_UXCLK: c_int = 6;
pub const SCU1_CLK_HUXCLK: c_int = 7;
pub const SCU1_CLK_UARTX: c_int = 8;
pub const SCU1_CLK_HUARTX: c_int = 9;
pub const SCU1_CLK_AHB: c_int = 10;
pub const SCU1_CLK_APB: c_int = 11;
pub const SCU1_CLK_UART0: c_int = 12;
pub const SCU1_CLK_UART1: c_int = 13;
pub const SCU1_CLK_UART2: c_int = 14;
pub const SCU1_CLK_UART3: c_int = 15;
pub const SCU1_CLK_UART5: c_int = 16;
pub const SCU1_CLK_UART6: c_int = 17;
pub const SCU1_CLK_UART7: c_int = 18;
pub const SCU1_CLK_UART8: c_int = 19;
pub const SCU1_CLK_UART9: c_int = 20;
pub const SCU1_CLK_UART10: c_int = 21;
pub const SCU1_CLK_UART11: c_int = 22;
pub const SCU1_CLK_UART12: c_int = 23;
pub const SCU1_CLK_UART13: c_int = 24;
pub const SCU1_CLK_UART14: c_int = 25;
pub const SCU1_CLK_APLL_DIVN: c_int = 26;
pub const SCU1_CLK_SDMUX: c_int = 27;
pub const SCU1_CLK_SDCLK: c_int = 28;
pub const SCU1_CLK_RMII: c_int = 29;
pub const SCU1_CLK_RGMII: c_int = 30;
pub const SCU1_CLK_MACHCLK: c_int = 31;
pub const SCU1_CLK_MAC0RCLK: c_int = 32;
pub const SCU1_CLK_MAC1RCLK: c_int = 33;
pub const SCU1_CLK_CAN: c_int = 34;
// SOC1 clk gate
pub const SCU1_CLK_GATE_LCLK0: c_int = 35;
pub const SCU1_CLK_GATE_LCLK1: c_int = 36;
pub const SCU1_CLK_GATE_ESPI0CLK: c_int = 37;
pub const SCU1_CLK_GATE_ESPI1CLK: c_int = 38;
pub const SCU1_CLK_GATE_SDCLK: c_int = 39;
pub const SCU1_CLK_GATE_IPEREFCLK: c_int = 40;
pub const SCU1_CLK_GATE_REFCLK: c_int = 41;
pub const SCU1_CLK_GATE_LPCHCLK: c_int = 42;
pub const SCU1_CLK_GATE_MAC0CLK: c_int = 43;
pub const SCU1_CLK_GATE_MAC1CLK: c_int = 44;
pub const SCU1_CLK_GATE_MAC2CLK: c_int = 45;
pub const SCU1_CLK_GATE_UART0CLK: c_int = 46;
pub const SCU1_CLK_GATE_UART1CLK: c_int = 47;
pub const SCU1_CLK_GATE_UART2CLK: c_int = 48;
pub const SCU1_CLK_GATE_UART3CLK: c_int = 49;
pub const SCU1_CLK_GATE_I2CCLK: c_int = 50;
pub const SCU1_CLK_GATE_I3C0CLK: c_int = 51;
pub const SCU1_CLK_GATE_I3C1CLK: c_int = 52;
pub const SCU1_CLK_GATE_I3C2CLK: c_int = 53;
pub const SCU1_CLK_GATE_I3C3CLK: c_int = 54;
pub const SCU1_CLK_GATE_I3C4CLK: c_int = 55;
pub const SCU1_CLK_GATE_I3C5CLK: c_int = 56;
pub const SCU1_CLK_GATE_I3C6CLK: c_int = 57;
pub const SCU1_CLK_GATE_I3C7CLK: c_int = 58;
pub const SCU1_CLK_GATE_I3C8CLK: c_int = 59;
pub const SCU1_CLK_GATE_I3C9CLK: c_int = 60;
pub const SCU1_CLK_GATE_I3C10CLK: c_int = 61;
pub const SCU1_CLK_GATE_I3C11CLK: c_int = 62;
pub const SCU1_CLK_GATE_I3C12CLK: c_int = 63;
pub const SCU1_CLK_GATE_I3C13CLK: c_int = 64;
pub const SCU1_CLK_GATE_I3C14CLK: c_int = 65;
pub const SCU1_CLK_GATE_I3C15CLK: c_int = 66;
pub const SCU1_CLK_GATE_UART5CLK: c_int = 67;
pub const SCU1_CLK_GATE_UART6CLK: c_int = 68;
pub const SCU1_CLK_GATE_UART7CLK: c_int = 69;
pub const SCU1_CLK_GATE_UART8CLK: c_int = 70;
pub const SCU1_CLK_GATE_UART9CLK: c_int = 71;
pub const SCU1_CLK_GATE_UART10CLK: c_int = 72;
pub const SCU1_CLK_GATE_UART11CLK: c_int = 73;
pub const SCU1_CLK_GATE_UART12CLK: c_int = 74;
pub const SCU1_CLK_GATE_FSICLK: c_int = 75;
pub const SCU1_CLK_GATE_LTPIPHYCLK: c_int = 76;
pub const SCU1_CLK_GATE_LTPICLK: c_int = 77;
pub const SCU1_CLK_GATE_VGALCLK: c_int = 78;
pub const SCU1_CLK_GATE_UHCICLK: c_int = 79;
pub const SCU1_CLK_GATE_CANCLK: c_int = 80;
pub const SCU1_CLK_GATE_PCICLK: c_int = 81;
pub const SCU1_CLK_GATE_SLICLK: c_int = 82;
pub const SCU1_CLK_GATE_E2MCLK: c_int = 83;
pub const SCU1_CLK_GATE_PORTCUSB2CLK: c_int = 84;
pub const SCU1_CLK_GATE_PORTDUSB2CLK: c_int = 85;
pub const SCU1_CLK_GATE_LTPI1TXCLK: c_int = 86;
pub const SCU1_CLK_I3C: c_int = 87;
pub const SCU1_CLK_HPLL_DIV4: c_int = 88;
pub const SCU1_CLK_PECI: c_int = 89;
