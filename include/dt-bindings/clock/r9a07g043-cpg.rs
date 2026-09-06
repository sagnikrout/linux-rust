//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r9a07g043-cpg.h
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
// Copyright (C) 2022 Renesas Electronics Corp.
//

// R9A07G043 CPG Core Clocks
pub const R9A07G043_CLK_I: c_int = 0;
pub const R9A07G043_CLK_I2: c_int = 1;
pub const R9A07G043_CLK_S0: c_int = 2;
pub const R9A07G043_CLK_SPI0: c_int = 3;
pub const R9A07G043_CLK_SPI1: c_int = 4;
pub const R9A07G043_CLK_SD0: c_int = 5;
pub const R9A07G043_CLK_SD1: c_int = 6;
pub const R9A07G043_CLK_M0: c_int = 7;

pub const R9A07G043_CLK_HP: c_int = 10;
pub const R9A07G043_CLK_TSU: c_int = 11;
pub const R9A07G043_CLK_ZT: c_int = 12;
pub const R9A07G043_CLK_P0: c_int = 13;
pub const R9A07G043_CLK_P1: c_int = 14;
pub const R9A07G043_CLK_P2: c_int = 15;

pub const R9A07G043_OSCCLK: c_int = 17;
pub const R9A07G043_CLK_P0_DIV2: c_int = 18;
// R9A07G043 Module Clocks

pub const R9A07G043_SYC_CNT_CLK: c_int = 11;
pub const R9A07G043_DMAC_ACLK: c_int = 12;
pub const R9A07G043_DMAC_PCLK: c_int = 13;
pub const R9A07G043_OSTM0_PCLK: c_int = 14;
pub const R9A07G043_OSTM1_PCLK: c_int = 15;
pub const R9A07G043_OSTM2_PCLK: c_int = 16;
pub const R9A07G043_MTU_X_MCK_MTU3: c_int = 17;
pub const R9A07G043_POE3_CLKM_POE: c_int = 18;
pub const R9A07G043_WDT0_PCLK: c_int = 19;
pub const R9A07G043_WDT0_CLK: c_int = 20;

pub const R9A07G043_SPI_CLK2: c_int = 23;
pub const R9A07G043_SPI_CLK: c_int = 24;
pub const R9A07G043_SDHI0_IMCLK: c_int = 25;
pub const R9A07G043_SDHI0_IMCLK2: c_int = 26;
pub const R9A07G043_SDHI0_CLK_HS: c_int = 27;
pub const R9A07G043_SDHI0_ACLK: c_int = 28;
pub const R9A07G043_SDHI1_IMCLK: c_int = 29;
pub const R9A07G043_SDHI1_IMCLK2: c_int = 30;
pub const R9A07G043_SDHI1_CLK_HS: c_int = 31;
pub const R9A07G043_SDHI1_ACLK: c_int = 32;

pub const R9A07G043_SSI0_PCLK2: c_int = 42;
pub const R9A07G043_SSI0_PCLK_SFR: c_int = 43;
pub const R9A07G043_SSI1_PCLK2: c_int = 44;
pub const R9A07G043_SSI1_PCLK_SFR: c_int = 45;
pub const R9A07G043_SSI2_PCLK2: c_int = 46;
pub const R9A07G043_SSI2_PCLK_SFR: c_int = 47;
pub const R9A07G043_SSI3_PCLK2: c_int = 48;
pub const R9A07G043_SSI3_PCLK_SFR: c_int = 49;

pub const R9A07G043_USB_U2H0_HCLK: c_int = 51;
pub const R9A07G043_USB_U2H1_HCLK: c_int = 52;
pub const R9A07G043_USB_U2P_EXR_CPUCLK: c_int = 53;
pub const R9A07G043_USB_PCLK: c_int = 54;
pub const R9A07G043_ETH0_CLK_AXI: c_int = 55;
pub const R9A07G043_ETH0_CLK_CHI: c_int = 56;
pub const R9A07G043_ETH1_CLK_AXI: c_int = 57;
pub const R9A07G043_ETH1_CLK_CHI: c_int = 58;
pub const R9A07G043_I2C0_PCLK: c_int = 59;
pub const R9A07G043_I2C1_PCLK: c_int = 60;
pub const R9A07G043_I2C2_PCLK: c_int = 61;
pub const R9A07G043_I2C3_PCLK: c_int = 62;
pub const R9A07G043_SCIF0_CLK_PCK: c_int = 63;
pub const R9A07G043_SCIF1_CLK_PCK: c_int = 64;
pub const R9A07G043_SCIF2_CLK_PCK: c_int = 65;
pub const R9A07G043_SCIF3_CLK_PCK: c_int = 66;
pub const R9A07G043_SCIF4_CLK_PCK: c_int = 67;
pub const R9A07G043_SCI0_CLKP: c_int = 68;
pub const R9A07G043_SCI1_CLKP: c_int = 69;
pub const R9A07G043_IRDA_CLKP: c_int = 70;
pub const R9A07G043_RSPI0_CLKB: c_int = 71;
pub const R9A07G043_RSPI1_CLKB: c_int = 72;
pub const R9A07G043_RSPI2_CLKB: c_int = 73;
pub const R9A07G043_CANFD_PCLK: c_int = 74;
pub const R9A07G043_GPIO_HCLK: c_int = 75;
pub const R9A07G043_ADC_ADCLK: c_int = 76;
pub const R9A07G043_ADC_PCLK: c_int = 77;
pub const R9A07G043_TSU_PCLK: c_int = 78;

// R9A07G043 Resets

pub const R9A07G043_DMAC_ARESETN: c_int = 17;
pub const R9A07G043_DMAC_RST_ASYNC: c_int = 18;
pub const R9A07G043_SYC_RESETN: c_int = 19;
pub const R9A07G043_OSTM0_PRESETZ: c_int = 20;
pub const R9A07G043_OSTM1_PRESETZ: c_int = 21;
pub const R9A07G043_OSTM2_PRESETZ: c_int = 22;
pub const R9A07G043_MTU_X_PRESET_MTU3: c_int = 23;
pub const R9A07G043_POE3_RST_M_REG: c_int = 24;
pub const R9A07G043_WDT0_PRESETN: c_int = 25;

pub const R9A07G043_SPI_RST: c_int = 27;
pub const R9A07G043_SDHI0_IXRST: c_int = 28;
pub const R9A07G043_SDHI1_IXRST: c_int = 29;

pub const R9A07G043_SSI0_RST_M2_REG: c_int = 36;
pub const R9A07G043_SSI1_RST_M2_REG: c_int = 37;
pub const R9A07G043_SSI2_RST_M2_REG: c_int = 38;
pub const R9A07G043_SSI3_RST_M2_REG: c_int = 39;

pub const R9A07G043_USB_U2H0_HRESETN: c_int = 41;
pub const R9A07G043_USB_U2H1_HRESETN: c_int = 42;
pub const R9A07G043_USB_U2P_EXL_SYSRST: c_int = 43;
pub const R9A07G043_USB_PRESETN: c_int = 44;
pub const R9A07G043_ETH0_RST_HW_N: c_int = 45;
pub const R9A07G043_ETH1_RST_HW_N: c_int = 46;
pub const R9A07G043_I2C0_MRST: c_int = 47;
pub const R9A07G043_I2C1_MRST: c_int = 48;
pub const R9A07G043_I2C2_MRST: c_int = 49;
pub const R9A07G043_I2C3_MRST: c_int = 50;
pub const R9A07G043_SCIF0_RST_SYSTEM_N: c_int = 51;
pub const R9A07G043_SCIF1_RST_SYSTEM_N: c_int = 52;
pub const R9A07G043_SCIF2_RST_SYSTEM_N: c_int = 53;
pub const R9A07G043_SCIF3_RST_SYSTEM_N: c_int = 54;
pub const R9A07G043_SCIF4_RST_SYSTEM_N: c_int = 55;
pub const R9A07G043_SCI0_RST: c_int = 56;
pub const R9A07G043_SCI1_RST: c_int = 57;
pub const R9A07G043_IRDA_RST: c_int = 58;
pub const R9A07G043_RSPI0_RST: c_int = 59;
pub const R9A07G043_RSPI1_RST: c_int = 60;
pub const R9A07G043_RSPI2_RST: c_int = 61;
pub const R9A07G043_CANFD_RSTP_N: c_int = 62;
pub const R9A07G043_CANFD_RSTC_N: c_int = 63;
pub const R9A07G043_GPIO_RSTN: c_int = 64;
pub const R9A07G043_GPIO_PORT_RESETN: c_int = 65;
pub const R9A07G043_GPIO_SPARE_RESETN: c_int = 66;
pub const R9A07G043_ADC_PRESETN: c_int = 67;
pub const R9A07G043_ADC_ADRST_N: c_int = 68;
pub const R9A07G043_TSU_PRESETN: c_int = 69;

