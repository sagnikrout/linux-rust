//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r9a06g032-sysctrl.h
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
//
// R9A06G032 sysctrl IDs
//
// Copyright (C) 2018 Renesas Electronics Europe Limited
//
// Michel Pollet <michel.pollet@bp.renesas.com>, <buserror@gmail.com>
//
pub const R9A06G032_CLK_PLL_USB: c_int = 1;

pub const R9A06G032_CLK_25_PG4: c_int = 26;
pub const R9A06G032_CLK_25_PG5: c_int = 27;
pub const R9A06G032_CLK_25_PG6: c_int = 28;
pub const R9A06G032_CLK_25_PG7: c_int = 29;
pub const R9A06G032_CLK_25_PG8: c_int = 30;
pub const R9A06G032_CLK_ADC: c_int = 31;
pub const R9A06G032_CLK_ECAT100: c_int = 32;
pub const R9A06G032_CLK_HSR100: c_int = 33;
pub const R9A06G032_CLK_I2C0: c_int = 34;
pub const R9A06G032_CLK_I2C1: c_int = 35;
pub const R9A06G032_CLK_MII_REF: c_int = 36;
pub const R9A06G032_CLK_NAND: c_int = 37;
pub const R9A06G032_CLK_NOUSBP2_PG6: c_int = 38;
pub const R9A06G032_CLK_P1_PG2: c_int = 39;
pub const R9A06G032_CLK_P1_PG3: c_int = 40;
pub const R9A06G032_CLK_P1_PG4: c_int = 41;
pub const R9A06G032_CLK_P4_PG3: c_int = 42;
pub const R9A06G032_CLK_P4_PG4: c_int = 43;
pub const R9A06G032_CLK_P6_PG1: c_int = 44;
pub const R9A06G032_CLK_P6_PG2: c_int = 45;
pub const R9A06G032_CLK_P6_PG3: c_int = 46;
pub const R9A06G032_CLK_P6_PG4: c_int = 47;
pub const R9A06G032_CLK_PCI_USB: c_int = 48;
pub const R9A06G032_CLK_QSPI0: c_int = 49;
pub const R9A06G032_CLK_QSPI1: c_int = 50;
pub const R9A06G032_CLK_RGMII_REF: c_int = 51;
pub const R9A06G032_CLK_RMII_REF: c_int = 52;
pub const R9A06G032_CLK_SDIO0: c_int = 53;
pub const R9A06G032_CLK_SDIO1: c_int = 54;
pub const R9A06G032_CLK_SERCOS100: c_int = 55;
pub const R9A06G032_CLK_SLCD: c_int = 56;
pub const R9A06G032_CLK_SPI0: c_int = 57;
pub const R9A06G032_CLK_SPI1: c_int = 58;
pub const R9A06G032_CLK_SPI2: c_int = 59;
pub const R9A06G032_CLK_SPI3: c_int = 60;
pub const R9A06G032_CLK_SPI4: c_int = 61;
pub const R9A06G032_CLK_SPI5: c_int = 62;
pub const R9A06G032_CLK_SWITCH: c_int = 63;
pub const R9A06G032_HCLK_ECAT125: c_int = 65;
pub const R9A06G032_HCLK_PINCONFIG: c_int = 66;
pub const R9A06G032_HCLK_SERCOS: c_int = 67;
pub const R9A06G032_HCLK_SGPIO2: c_int = 68;
pub const R9A06G032_HCLK_SGPIO3: c_int = 69;
pub const R9A06G032_HCLK_SGPIO4: c_int = 70;
pub const R9A06G032_HCLK_TIMER0: c_int = 71;
pub const R9A06G032_HCLK_TIMER1: c_int = 72;
pub const R9A06G032_HCLK_USBF: c_int = 73;
pub const R9A06G032_HCLK_USBH: c_int = 74;
pub const R9A06G032_HCLK_USBPM: c_int = 75;
pub const R9A06G032_CLK_48_PG_F: c_int = 76;
pub const R9A06G032_CLK_48_PG4: c_int = 77;

pub const R9A06G032_HCLK_CAN0: c_int = 85;
pub const R9A06G032_HCLK_CAN1: c_int = 86;
pub const R9A06G032_HCLK_DELTASIGMA: c_int = 87;
pub const R9A06G032_HCLK_PWMPTO: c_int = 88;
pub const R9A06G032_HCLK_RSV: c_int = 89;
pub const R9A06G032_HCLK_SGPIO0: c_int = 90;
pub const R9A06G032_HCLK_SGPIO1: c_int = 91;
pub const R9A06G032_RTOS_MDC: c_int = 92;
pub const R9A06G032_CLK_CM3: c_int = 93;
pub const R9A06G032_CLK_DDRC: c_int = 94;
pub const R9A06G032_CLK_ECAT25: c_int = 95;
pub const R9A06G032_CLK_HSR50: c_int = 96;
pub const R9A06G032_CLK_HW_RTOS: c_int = 97;
pub const R9A06G032_CLK_SERCOS50: c_int = 98;
pub const R9A06G032_HCLK_ADC: c_int = 99;
pub const R9A06G032_HCLK_CM3: c_int = 100;
pub const R9A06G032_HCLK_CRYPTO_EIP150: c_int = 101;
pub const R9A06G032_HCLK_CRYPTO_EIP93: c_int = 102;
pub const R9A06G032_HCLK_DDRC: c_int = 103;
pub const R9A06G032_HCLK_DMA0: c_int = 104;
pub const R9A06G032_HCLK_DMA1: c_int = 105;
pub const R9A06G032_HCLK_GMAC0: c_int = 106;
pub const R9A06G032_HCLK_GMAC1: c_int = 107;
pub const R9A06G032_HCLK_GPIO0: c_int = 108;
pub const R9A06G032_HCLK_GPIO1: c_int = 109;
pub const R9A06G032_HCLK_GPIO2: c_int = 110;
pub const R9A06G032_HCLK_HSR: c_int = 111;
pub const R9A06G032_HCLK_I2C0: c_int = 112;
pub const R9A06G032_HCLK_I2C1: c_int = 113;
pub const R9A06G032_HCLK_LCD: c_int = 114;
pub const R9A06G032_HCLK_MSEBI_M: c_int = 115;
pub const R9A06G032_HCLK_MSEBI_S: c_int = 116;
pub const R9A06G032_HCLK_NAND: c_int = 117;
pub const R9A06G032_HCLK_PG_I: c_int = 118;
pub const R9A06G032_HCLK_PG19: c_int = 119;
pub const R9A06G032_HCLK_PG20: c_int = 120;
pub const R9A06G032_HCLK_PG3: c_int = 121;
pub const R9A06G032_HCLK_PG4: c_int = 122;
pub const R9A06G032_HCLK_QSPI0: c_int = 123;
pub const R9A06G032_HCLK_QSPI1: c_int = 124;
pub const R9A06G032_HCLK_ROM: c_int = 125;
pub const R9A06G032_HCLK_RTC: c_int = 126;
pub const R9A06G032_HCLK_SDIO0: c_int = 127;
pub const R9A06G032_HCLK_SDIO1: c_int = 128;
pub const R9A06G032_HCLK_SEMAP: c_int = 129;
pub const R9A06G032_HCLK_SPI0: c_int = 130;
pub const R9A06G032_HCLK_SPI1: c_int = 131;
pub const R9A06G032_HCLK_SPI2: c_int = 132;
pub const R9A06G032_HCLK_SPI3: c_int = 133;
pub const R9A06G032_HCLK_SPI4: c_int = 134;
pub const R9A06G032_HCLK_SPI5: c_int = 135;
pub const R9A06G032_HCLK_SWITCH: c_int = 136;
pub const R9A06G032_HCLK_SWITCH_RG: c_int = 137;
pub const R9A06G032_HCLK_UART0: c_int = 138;
pub const R9A06G032_HCLK_UART1: c_int = 139;
pub const R9A06G032_HCLK_UART2: c_int = 140;
pub const R9A06G032_HCLK_UART3: c_int = 141;
pub const R9A06G032_HCLK_UART4: c_int = 142;
pub const R9A06G032_HCLK_UART5: c_int = 143;
pub const R9A06G032_HCLK_UART6: c_int = 144;
pub const R9A06G032_HCLK_UART7: c_int = 145;
pub const R9A06G032_CLK_UART0: c_int = 146;
pub const R9A06G032_CLK_UART1: c_int = 147;
pub const R9A06G032_CLK_UART2: c_int = 148;
pub const R9A06G032_CLK_UART3: c_int = 149;
pub const R9A06G032_CLK_UART4: c_int = 150;
pub const R9A06G032_CLK_UART5: c_int = 151;
pub const R9A06G032_CLK_UART6: c_int = 152;
pub const R9A06G032_CLK_UART7: c_int = 153;
