//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/imx/clk-imx8qxp-lpcg.h
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
// Copyright 2018 NXP
// Dong Aisheng <aisheng.dong@nxp.com>
//
// LSIO SS
pub const LSIO_PWM_0_LPCG: c_uint = 0x00000;
pub const LSIO_PWM_1_LPCG: c_uint = 0x10000;
pub const LSIO_PWM_2_LPCG: c_uint = 0x20000;
pub const LSIO_PWM_3_LPCG: c_uint = 0x30000;
pub const LSIO_PWM_4_LPCG: c_uint = 0x40000;
pub const LSIO_PWM_5_LPCG: c_uint = 0x50000;
pub const LSIO_PWM_6_LPCG: c_uint = 0x60000;
pub const LSIO_PWM_7_LPCG: c_uint = 0x70000;
pub const LSIO_GPIO_0_LPCG: c_uint = 0x80000;
pub const LSIO_GPIO_1_LPCG: c_uint = 0x90000;
pub const LSIO_GPIO_2_LPCG: c_uint = 0xa0000;
pub const LSIO_GPIO_3_LPCG: c_uint = 0xb0000;
pub const LSIO_GPIO_4_LPCG: c_uint = 0xc0000;
pub const LSIO_GPIO_5_LPCG: c_uint = 0xd0000;
pub const LSIO_GPIO_6_LPCG: c_uint = 0xe0000;
pub const LSIO_GPIO_7_LPCG: c_uint = 0xf0000;
pub const LSIO_FSPI_0_LPCG: c_uint = 0x120000;
pub const LSIO_FSPI_1_LPCG: c_uint = 0x130000;
pub const LSIO_GPT_0_LPCG: c_uint = 0x140000;
pub const LSIO_GPT_1_LPCG: c_uint = 0x150000;
pub const LSIO_GPT_2_LPCG: c_uint = 0x160000;
pub const LSIO_GPT_3_LPCG: c_uint = 0x170000;
pub const LSIO_GPT_4_LPCG: c_uint = 0x180000;
pub const LSIO_OCRAM_LPCG: c_uint = 0x190000;
pub const LSIO_KPP_LPCG: c_uint = 0x1a0000;
pub const LSIO_ROMCP_LPCG: c_uint = 0x100000;
// Connectivity SS
pub const CONN_USDHC_0_LPCG: c_uint = 0x00000;
pub const CONN_USDHC_1_LPCG: c_uint = 0x10000;
pub const CONN_USDHC_2_LPCG: c_uint = 0x20000;
pub const CONN_ENET_0_LPCG: c_uint = 0x30000;
pub const CONN_ENET_1_LPCG: c_uint = 0x40000;
pub const CONN_DTCP_LPCG: c_uint = 0x50000;
pub const CONN_USB_2_LPCG: c_uint = 0x70000;
pub const CONN_USB_3_LPCG: c_uint = 0x80000;
pub const CONN_NAND_LPCG: c_uint = 0x90000;
pub const CONN_EDMA_LPCG: c_uint = 0xa0000;
// ADMA SS
pub const ADMA_ASRC_0_LPCG: c_uint = 0x400000;
pub const ADMA_ESAI_0_LPCG: c_uint = 0x410000;
pub const ADMA_SPDIF_0_LPCG: c_uint = 0x420000;
pub const ADMA_SAI_0_LPCG: c_uint = 0x440000;
pub const ADMA_SAI_1_LPCG: c_uint = 0x450000;
pub const ADMA_SAI_2_LPCG: c_uint = 0x460000;
pub const ADMA_SAI_3_LPCG: c_uint = 0x470000;
pub const ADMA_GPT_5_LPCG: c_uint = 0x4b0000;
pub const ADMA_GPT_6_LPCG: c_uint = 0x4c0000;
pub const ADMA_GPT_7_LPCG: c_uint = 0x4d0000;
pub const ADMA_GPT_8_LPCG: c_uint = 0x4e0000;
pub const ADMA_GPT_9_LPCG: c_uint = 0x4f0000;
pub const ADMA_GPT_10_LPCG: c_uint = 0x500000;
pub const ADMA_HIFI_LPCG: c_uint = 0x580000;
pub const ADMA_OCRAM_LPCG: c_uint = 0x590000;
pub const ADMA_EDMA_0_LPCG: c_uint = 0x5f0000;
pub const ADMA_ASRC_1_LPCG: c_uint = 0xc00000;
pub const ADMA_SAI_4_LPCG: c_uint = 0xc20000;
pub const ADMA_SAI_5_LPCG: c_uint = 0xc30000;
pub const ADMA_AMIX_LPCG: c_uint = 0xc40000;
pub const ADMA_MQS_LPCG: c_uint = 0xc50000;
pub const ADMA_ACM_LPCG: c_uint = 0xc60000;
pub const ADMA_REC_CLK0_LPCG: c_uint = 0xd00000;
pub const ADMA_REC_CLK1_LPCG: c_uint = 0xd10000;
pub const ADMA_PLL_CLK0_LPCG: c_uint = 0xd20000;
pub const ADMA_PLL_CLK1_LPCG: c_uint = 0xd30000;
pub const ADMA_MCLKOUT0_LPCG: c_uint = 0xd50000;
pub const ADMA_MCLKOUT1_LPCG: c_uint = 0xd60000;
pub const ADMA_EDMA_1_LPCG: c_uint = 0xdf0000;
pub const ADMA_LPSPI_0_LPCG: c_uint = 0x1400000;
pub const ADMA_LPSPI_1_LPCG: c_uint = 0x1410000;
pub const ADMA_LPSPI_2_LPCG: c_uint = 0x1420000;
pub const ADMA_LPSPI_3_LPCG: c_uint = 0x1430000;
pub const ADMA_LPUART_0_LPCG: c_uint = 0x1460000;
pub const ADMA_LPUART_1_LPCG: c_uint = 0x1470000;
pub const ADMA_LPUART_2_LPCG: c_uint = 0x1480000;
pub const ADMA_LPUART_3_LPCG: c_uint = 0x1490000;
pub const ADMA_LCD_LPCG: c_uint = 0x1580000;
pub const ADMA_PWM_LPCG: c_uint = 0x1590000;
pub const ADMA_LPI2C_0_LPCG: c_uint = 0x1c00000;
pub const ADMA_LPI2C_1_LPCG: c_uint = 0x1c10000;
pub const ADMA_LPI2C_2_LPCG: c_uint = 0x1c20000;
pub const ADMA_LPI2C_3_LPCG: c_uint = 0x1c30000;
pub const ADMA_ADC_0_LPCG: c_uint = 0x1c80000;
pub const ADMA_FTM_0_LPCG: c_uint = 0x1ca0000;
pub const ADMA_FTM_1_LPCG: c_uint = 0x1cb0000;
pub const ADMA_FLEXCAN_0_LPCG: c_uint = 0x1cd0000;
pub const ADMA_FLEXCAN_1_LPCG: c_uint = 0x1ce0000;
pub const ADMA_FLEXCAN_2_LPCG: c_uint = 0x1cf0000;
