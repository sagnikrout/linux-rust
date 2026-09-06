//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx7ulp-clock.h
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
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017~2018 NXP
//
// SCG1
pub const IMX7ULP_CLK_DUMMY: c_int = 0;
pub const IMX7ULP_CLK_ROSC: c_int = 1;
pub const IMX7ULP_CLK_SOSC: c_int = 2;
pub const IMX7ULP_CLK_FIRC: c_int = 3;
pub const IMX7ULP_CLK_SPLL_PRE_SEL: c_int = 4;
pub const IMX7ULP_CLK_SPLL_PRE_DIV: c_int = 5;
pub const IMX7ULP_CLK_SPLL: c_int = 6;
pub const IMX7ULP_CLK_SPLL_POST_DIV1: c_int = 7;
pub const IMX7ULP_CLK_SPLL_POST_DIV2: c_int = 8;
pub const IMX7ULP_CLK_SPLL_PFD0: c_int = 9;
pub const IMX7ULP_CLK_SPLL_PFD1: c_int = 10;
pub const IMX7ULP_CLK_SPLL_PFD2: c_int = 11;
pub const IMX7ULP_CLK_SPLL_PFD3: c_int = 12;
pub const IMX7ULP_CLK_SPLL_PFD_SEL: c_int = 13;
pub const IMX7ULP_CLK_SPLL_SEL: c_int = 14;
pub const IMX7ULP_CLK_APLL_PRE_SEL: c_int = 15;
pub const IMX7ULP_CLK_APLL_PRE_DIV: c_int = 16;
pub const IMX7ULP_CLK_APLL: c_int = 17;
pub const IMX7ULP_CLK_APLL_POST_DIV1: c_int = 18;
pub const IMX7ULP_CLK_APLL_POST_DIV2: c_int = 19;
pub const IMX7ULP_CLK_APLL_PFD0: c_int = 20;
pub const IMX7ULP_CLK_APLL_PFD1: c_int = 21;
pub const IMX7ULP_CLK_APLL_PFD2: c_int = 22;
pub const IMX7ULP_CLK_APLL_PFD3: c_int = 23;
pub const IMX7ULP_CLK_APLL_PFD_SEL: c_int = 24;
pub const IMX7ULP_CLK_APLL_SEL: c_int = 25;
pub const IMX7ULP_CLK_UPLL: c_int = 26;
pub const IMX7ULP_CLK_SYS_SEL: c_int = 27;
pub const IMX7ULP_CLK_CORE_DIV: c_int = 28;
pub const IMX7ULP_CLK_BUS_DIV: c_int = 29;
pub const IMX7ULP_CLK_PLAT_DIV: c_int = 30;
pub const IMX7ULP_CLK_DDR_SEL: c_int = 31;
pub const IMX7ULP_CLK_DDR_DIV: c_int = 32;
pub const IMX7ULP_CLK_NIC_SEL: c_int = 33;
pub const IMX7ULP_CLK_NIC0_DIV: c_int = 34;
pub const IMX7ULP_CLK_GPU_DIV: c_int = 35;
pub const IMX7ULP_CLK_NIC1_DIV: c_int = 36;
pub const IMX7ULP_CLK_NIC1_BUS_DIV: c_int = 37;
pub const IMX7ULP_CLK_NIC1_EXT_DIV: c_int = 38;
// IMX7ULP_CLK_MIPI_PLL is unsupported and shouldn't be used in DT
pub const IMX7ULP_CLK_MIPI_PLL: c_int = 39;
pub const IMX7ULP_CLK_SIRC: c_int = 40;
pub const IMX7ULP_CLK_SOSC_BUS_CLK: c_int = 41;
pub const IMX7ULP_CLK_FIRC_BUS_CLK: c_int = 42;
pub const IMX7ULP_CLK_SPLL_BUS_CLK: c_int = 43;
pub const IMX7ULP_CLK_HSRUN_SYS_SEL: c_int = 44;
pub const IMX7ULP_CLK_HSRUN_CORE_DIV: c_int = 45;
pub const IMX7ULP_CLK_CORE: c_int = 46;
pub const IMX7ULP_CLK_HSRUN_CORE: c_int = 47;
pub const IMX7ULP_CLK_SCG1_END: c_int = 48;
// PCC2
pub const IMX7ULP_CLK_DMA1: c_int = 0;
pub const IMX7ULP_CLK_RGPIO2P1: c_int = 1;
pub const IMX7ULP_CLK_FLEXBUS: c_int = 2;
pub const IMX7ULP_CLK_SEMA42_1: c_int = 3;
pub const IMX7ULP_CLK_DMA_MUX1: c_int = 4;
pub const IMX7ULP_CLK_CAAM: c_int = 6;
pub const IMX7ULP_CLK_LPTPM4: c_int = 7;
pub const IMX7ULP_CLK_LPTPM5: c_int = 8;
pub const IMX7ULP_CLK_LPIT1: c_int = 9;
pub const IMX7ULP_CLK_LPSPI2: c_int = 10;
pub const IMX7ULP_CLK_LPSPI3: c_int = 11;
pub const IMX7ULP_CLK_LPI2C4: c_int = 12;
pub const IMX7ULP_CLK_LPI2C5: c_int = 13;
pub const IMX7ULP_CLK_LPUART4: c_int = 14;
pub const IMX7ULP_CLK_LPUART5: c_int = 15;
pub const IMX7ULP_CLK_FLEXIO1: c_int = 16;
pub const IMX7ULP_CLK_USB0: c_int = 17;
pub const IMX7ULP_CLK_USB1: c_int = 18;
pub const IMX7ULP_CLK_USB_PHY: c_int = 19;
pub const IMX7ULP_CLK_USB_PL301: c_int = 20;
pub const IMX7ULP_CLK_USDHC0: c_int = 21;
pub const IMX7ULP_CLK_USDHC1: c_int = 22;
pub const IMX7ULP_CLK_WDG1: c_int = 23;
pub const IMX7ULP_CLK_WDG2: c_int = 24;
pub const IMX7ULP_CLK_PCC2_END: c_int = 25;
// PCC3
pub const IMX7ULP_CLK_LPTPM6: c_int = 0;
pub const IMX7ULP_CLK_LPTPM7: c_int = 1;
pub const IMX7ULP_CLK_LPI2C6: c_int = 2;
pub const IMX7ULP_CLK_LPI2C7: c_int = 3;
pub const IMX7ULP_CLK_LPUART6: c_int = 4;
pub const IMX7ULP_CLK_LPUART7: c_int = 5;
pub const IMX7ULP_CLK_VIU: c_int = 6;
pub const IMX7ULP_CLK_DSI: c_int = 7;
pub const IMX7ULP_CLK_LCDIF: c_int = 8;
pub const IMX7ULP_CLK_MMDC: c_int = 9;
pub const IMX7ULP_CLK_PCTLC: c_int = 10;
pub const IMX7ULP_CLK_PCTLD: c_int = 11;
pub const IMX7ULP_CLK_PCTLE: c_int = 12;
pub const IMX7ULP_CLK_PCTLF: c_int = 13;
pub const IMX7ULP_CLK_GPU3D: c_int = 14;
pub const IMX7ULP_CLK_GPU2D: c_int = 15;
pub const IMX7ULP_CLK_PCC3_END: c_int = 16;
// SMC1
pub const IMX7ULP_CLK_ARM: c_int = 0;
pub const IMX7ULP_CLK_SMC1_END: c_int = 1;
