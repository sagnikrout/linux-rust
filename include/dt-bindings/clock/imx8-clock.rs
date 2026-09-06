//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx8-clock.h
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
// LPCG clocks
// LSIO SS LPCG
pub const IMX_LSIO_LPCG_PWM0_IPG_CLK: c_int = 0;
pub const IMX_LSIO_LPCG_PWM0_IPG_S_CLK: c_int = 1;
pub const IMX_LSIO_LPCG_PWM0_IPG_HF_CLK: c_int = 2;
pub const IMX_LSIO_LPCG_PWM0_IPG_SLV_CLK: c_int = 3;
pub const IMX_LSIO_LPCG_PWM0_IPG_MSTR_CLK: c_int = 4;
pub const IMX_LSIO_LPCG_PWM1_IPG_CLK: c_int = 5;
pub const IMX_LSIO_LPCG_PWM1_IPG_S_CLK: c_int = 6;
pub const IMX_LSIO_LPCG_PWM1_IPG_HF_CLK: c_int = 7;
pub const IMX_LSIO_LPCG_PWM1_IPG_SLV_CLK: c_int = 8;
pub const IMX_LSIO_LPCG_PWM1_IPG_MSTR_CLK: c_int = 9;
pub const IMX_LSIO_LPCG_PWM2_IPG_CLK: c_int = 10;
pub const IMX_LSIO_LPCG_PWM2_IPG_S_CLK: c_int = 11;
pub const IMX_LSIO_LPCG_PWM2_IPG_HF_CLK: c_int = 12;
pub const IMX_LSIO_LPCG_PWM2_IPG_SLV_CLK: c_int = 13;
pub const IMX_LSIO_LPCG_PWM2_IPG_MSTR_CLK: c_int = 14;
pub const IMX_LSIO_LPCG_PWM3_IPG_CLK: c_int = 15;
pub const IMX_LSIO_LPCG_PWM3_IPG_S_CLK: c_int = 16;
pub const IMX_LSIO_LPCG_PWM3_IPG_HF_CLK: c_int = 17;
pub const IMX_LSIO_LPCG_PWM3_IPG_SLV_CLK: c_int = 18;
pub const IMX_LSIO_LPCG_PWM3_IPG_MSTR_CLK: c_int = 19;
pub const IMX_LSIO_LPCG_PWM4_IPG_CLK: c_int = 20;
pub const IMX_LSIO_LPCG_PWM4_IPG_S_CLK: c_int = 21;
pub const IMX_LSIO_LPCG_PWM4_IPG_HF_CLK: c_int = 22;
pub const IMX_LSIO_LPCG_PWM4_IPG_SLV_CLK: c_int = 23;
pub const IMX_LSIO_LPCG_PWM4_IPG_MSTR_CLK: c_int = 24;
pub const IMX_LSIO_LPCG_PWM5_IPG_CLK: c_int = 25;
pub const IMX_LSIO_LPCG_PWM5_IPG_S_CLK: c_int = 26;
pub const IMX_LSIO_LPCG_PWM5_IPG_HF_CLK: c_int = 27;
pub const IMX_LSIO_LPCG_PWM5_IPG_SLV_CLK: c_int = 28;
pub const IMX_LSIO_LPCG_PWM5_IPG_MSTR_CLK: c_int = 29;
pub const IMX_LSIO_LPCG_PWM6_IPG_CLK: c_int = 30;
pub const IMX_LSIO_LPCG_PWM6_IPG_S_CLK: c_int = 31;
pub const IMX_LSIO_LPCG_PWM6_IPG_HF_CLK: c_int = 32;
pub const IMX_LSIO_LPCG_PWM6_IPG_SLV_CLK: c_int = 33;
pub const IMX_LSIO_LPCG_PWM6_IPG_MSTR_CLK: c_int = 34;
pub const IMX_LSIO_LPCG_PWM7_IPG_CLK: c_int = 35;
pub const IMX_LSIO_LPCG_PWM7_IPG_S_CLK: c_int = 36;
pub const IMX_LSIO_LPCG_PWM7_IPG_HF_CLK: c_int = 37;
pub const IMX_LSIO_LPCG_PWM7_IPG_SLV_CLK: c_int = 38;
pub const IMX_LSIO_LPCG_PWM7_IPG_MSTR_CLK: c_int = 39;
pub const IMX_LSIO_LPCG_GPT0_IPG_CLK: c_int = 40;
pub const IMX_LSIO_LPCG_GPT0_IPG_S_CLK: c_int = 41;
pub const IMX_LSIO_LPCG_GPT0_IPG_HF_CLK: c_int = 42;
pub const IMX_LSIO_LPCG_GPT0_IPG_SLV_CLK: c_int = 43;
pub const IMX_LSIO_LPCG_GPT0_IPG_MSTR_CLK: c_int = 44;
pub const IMX_LSIO_LPCG_GPT1_IPG_CLK: c_int = 45;
pub const IMX_LSIO_LPCG_GPT1_IPG_S_CLK: c_int = 46;
pub const IMX_LSIO_LPCG_GPT1_IPG_HF_CLK: c_int = 47;
pub const IMX_LSIO_LPCG_GPT1_IPG_SLV_CLK: c_int = 48;
pub const IMX_LSIO_LPCG_GPT1_IPG_MSTR_CLK: c_int = 49;
pub const IMX_LSIO_LPCG_GPT2_IPG_CLK: c_int = 50;
pub const IMX_LSIO_LPCG_GPT2_IPG_S_CLK: c_int = 51;
pub const IMX_LSIO_LPCG_GPT2_IPG_HF_CLK: c_int = 52;
pub const IMX_LSIO_LPCG_GPT2_IPG_SLV_CLK: c_int = 53;
pub const IMX_LSIO_LPCG_GPT2_IPG_MSTR_CLK: c_int = 54;
pub const IMX_LSIO_LPCG_GPT3_IPG_CLK: c_int = 55;
pub const IMX_LSIO_LPCG_GPT3_IPG_S_CLK: c_int = 56;
pub const IMX_LSIO_LPCG_GPT3_IPG_HF_CLK: c_int = 57;
pub const IMX_LSIO_LPCG_GPT3_IPG_SLV_CLK: c_int = 58;
pub const IMX_LSIO_LPCG_GPT3_IPG_MSTR_CLK: c_int = 59;
pub const IMX_LSIO_LPCG_GPT4_IPG_CLK: c_int = 60;
pub const IMX_LSIO_LPCG_GPT4_IPG_S_CLK: c_int = 61;
pub const IMX_LSIO_LPCG_GPT4_IPG_HF_CLK: c_int = 62;
pub const IMX_LSIO_LPCG_GPT4_IPG_SLV_CLK: c_int = 63;
pub const IMX_LSIO_LPCG_GPT4_IPG_MSTR_CLK: c_int = 64;
pub const IMX_LSIO_LPCG_FSPI0_HCLK: c_int = 65;
pub const IMX_LSIO_LPCG_FSPI0_IPG_CLK: c_int = 66;
pub const IMX_LSIO_LPCG_FSPI0_IPG_S_CLK: c_int = 67;
pub const IMX_LSIO_LPCG_FSPI0_IPG_SFCK: c_int = 68;
pub const IMX_LSIO_LPCG_FSPI1_HCLK: c_int = 69;
pub const IMX_LSIO_LPCG_FSPI1_IPG_CLK: c_int = 70;
pub const IMX_LSIO_LPCG_FSPI1_IPG_S_CLK: c_int = 71;
pub const IMX_LSIO_LPCG_FSPI1_IPG_SFCK: c_int = 72;
pub const IMX_LSIO_LPCG_CLK_END: c_int = 73;
// Connectivity SS LPCG
pub const IMX_CONN_LPCG_SDHC0_IPG_CLK: c_int = 0;
pub const IMX_CONN_LPCG_SDHC0_PER_CLK: c_int = 1;
pub const IMX_CONN_LPCG_SDHC0_HCLK: c_int = 2;
pub const IMX_CONN_LPCG_SDHC1_IPG_CLK: c_int = 3;
pub const IMX_CONN_LPCG_SDHC1_PER_CLK: c_int = 4;
pub const IMX_CONN_LPCG_SDHC1_HCLK: c_int = 5;
pub const IMX_CONN_LPCG_SDHC2_IPG_CLK: c_int = 6;
pub const IMX_CONN_LPCG_SDHC2_PER_CLK: c_int = 7;
pub const IMX_CONN_LPCG_SDHC2_HCLK: c_int = 8;
pub const IMX_CONN_LPCG_GPMI_APB_CLK: c_int = 9;
pub const IMX_CONN_LPCG_GPMI_BCH_APB_CLK: c_int = 10;
pub const IMX_CONN_LPCG_GPMI_BCH_IO_CLK: c_int = 11;
pub const IMX_CONN_LPCG_GPMI_BCH_CLK: c_int = 12;
pub const IMX_CONN_LPCG_APBHDMA_CLK: c_int = 13;
pub const IMX_CONN_LPCG_ENET0_ROOT_CLK: c_int = 14;
pub const IMX_CONN_LPCG_ENET0_TX_CLK: c_int = 15;
pub const IMX_CONN_LPCG_ENET0_AHB_CLK: c_int = 16;
pub const IMX_CONN_LPCG_ENET0_IPG_S_CLK: c_int = 17;
pub const IMX_CONN_LPCG_ENET0_IPG_CLK: c_int = 18;
pub const IMX_CONN_LPCG_ENET1_ROOT_CLK: c_int = 19;
pub const IMX_CONN_LPCG_ENET1_TX_CLK: c_int = 20;
pub const IMX_CONN_LPCG_ENET1_AHB_CLK: c_int = 21;
pub const IMX_CONN_LPCG_ENET1_IPG_S_CLK: c_int = 22;
pub const IMX_CONN_LPCG_ENET1_IPG_CLK: c_int = 23;
pub const IMX_CONN_LPCG_CLK_END: c_int = 24;
// ADMA SS LPCG
pub const IMX_ADMA_LPCG_UART0_IPG_CLK: c_int = 0;
pub const IMX_ADMA_LPCG_UART0_BAUD_CLK: c_int = 1;
pub const IMX_ADMA_LPCG_UART1_IPG_CLK: c_int = 2;
pub const IMX_ADMA_LPCG_UART1_BAUD_CLK: c_int = 3;
pub const IMX_ADMA_LPCG_UART2_IPG_CLK: c_int = 4;
pub const IMX_ADMA_LPCG_UART2_BAUD_CLK: c_int = 5;
pub const IMX_ADMA_LPCG_UART3_IPG_CLK: c_int = 6;
pub const IMX_ADMA_LPCG_UART3_BAUD_CLK: c_int = 7;
pub const IMX_ADMA_LPCG_SPI0_IPG_CLK: c_int = 8;
pub const IMX_ADMA_LPCG_SPI1_IPG_CLK: c_int = 9;
pub const IMX_ADMA_LPCG_SPI2_IPG_CLK: c_int = 10;
pub const IMX_ADMA_LPCG_SPI3_IPG_CLK: c_int = 11;
pub const IMX_ADMA_LPCG_SPI0_CLK: c_int = 12;
pub const IMX_ADMA_LPCG_SPI1_CLK: c_int = 13;
pub const IMX_ADMA_LPCG_SPI2_CLK: c_int = 14;
pub const IMX_ADMA_LPCG_SPI3_CLK: c_int = 15;
pub const IMX_ADMA_LPCG_CAN0_IPG_CLK: c_int = 16;
pub const IMX_ADMA_LPCG_CAN0_IPG_PE_CLK: c_int = 17;
pub const IMX_ADMA_LPCG_CAN0_IPG_CHI_CLK: c_int = 18;
pub const IMX_ADMA_LPCG_CAN1_IPG_CLK: c_int = 19;
pub const IMX_ADMA_LPCG_CAN1_IPG_PE_CLK: c_int = 20;
pub const IMX_ADMA_LPCG_CAN1_IPG_CHI_CLK: c_int = 21;
pub const IMX_ADMA_LPCG_CAN2_IPG_CLK: c_int = 22;
pub const IMX_ADMA_LPCG_CAN2_IPG_PE_CLK: c_int = 23;
pub const IMX_ADMA_LPCG_CAN2_IPG_CHI_CLK: c_int = 24;
pub const IMX_ADMA_LPCG_I2C0_CLK: c_int = 25;
pub const IMX_ADMA_LPCG_I2C1_CLK: c_int = 26;
pub const IMX_ADMA_LPCG_I2C2_CLK: c_int = 27;
pub const IMX_ADMA_LPCG_I2C3_CLK: c_int = 28;
pub const IMX_ADMA_LPCG_I2C0_IPG_CLK: c_int = 29;
pub const IMX_ADMA_LPCG_I2C1_IPG_CLK: c_int = 30;
pub const IMX_ADMA_LPCG_I2C2_IPG_CLK: c_int = 31;
pub const IMX_ADMA_LPCG_I2C3_IPG_CLK: c_int = 32;
pub const IMX_ADMA_LPCG_FTM0_CLK: c_int = 33;
pub const IMX_ADMA_LPCG_FTM1_CLK: c_int = 34;
pub const IMX_ADMA_LPCG_FTM0_IPG_CLK: c_int = 35;
pub const IMX_ADMA_LPCG_FTM1_IPG_CLK: c_int = 36;
pub const IMX_ADMA_LPCG_PWM_HI_CLK: c_int = 37;
pub const IMX_ADMA_LPCG_PWM_IPG_CLK: c_int = 38;
pub const IMX_ADMA_LPCG_LCD_PIX_CLK: c_int = 39;
pub const IMX_ADMA_LPCG_LCD_APB_CLK: c_int = 40;
pub const IMX_ADMA_LPCG_DSP_ADB_CLK: c_int = 41;
pub const IMX_ADMA_LPCG_DSP_IPG_CLK: c_int = 42;
pub const IMX_ADMA_LPCG_DSP_CORE_CLK: c_int = 43;
pub const IMX_ADMA_LPCG_OCRAM_IPG_CLK: c_int = 44;
pub const IMX_ADMA_LPCG_CLK_END: c_int = 45;
pub const IMX_ADMA_ACM_AUD_CLK0_SEL: c_int = 0;
pub const IMX_ADMA_ACM_AUD_CLK1_SEL: c_int = 1;
pub const IMX_ADMA_ACM_MCLKOUT0_SEL: c_int = 2;
pub const IMX_ADMA_ACM_MCLKOUT1_SEL: c_int = 3;
pub const IMX_ADMA_ACM_ESAI0_MCLK_SEL: c_int = 4;
pub const IMX_ADMA_ACM_ESAI1_MCLK_SEL: c_int = 5;
pub const IMX_ADMA_ACM_GPT0_MUX_CLK_SEL: c_int = 6;
pub const IMX_ADMA_ACM_GPT1_MUX_CLK_SEL: c_int = 7;
pub const IMX_ADMA_ACM_GPT2_MUX_CLK_SEL: c_int = 8;
pub const IMX_ADMA_ACM_GPT3_MUX_CLK_SEL: c_int = 9;
pub const IMX_ADMA_ACM_GPT4_MUX_CLK_SEL: c_int = 10;
pub const IMX_ADMA_ACM_GPT5_MUX_CLK_SEL: c_int = 11;
pub const IMX_ADMA_ACM_SAI0_MCLK_SEL: c_int = 12;
pub const IMX_ADMA_ACM_SAI1_MCLK_SEL: c_int = 13;
pub const IMX_ADMA_ACM_SAI2_MCLK_SEL: c_int = 14;
pub const IMX_ADMA_ACM_SAI3_MCLK_SEL: c_int = 15;
pub const IMX_ADMA_ACM_SAI4_MCLK_SEL: c_int = 16;
pub const IMX_ADMA_ACM_SAI5_MCLK_SEL: c_int = 17;
pub const IMX_ADMA_ACM_SAI6_MCLK_SEL: c_int = 18;
pub const IMX_ADMA_ACM_SAI7_MCLK_SEL: c_int = 19;
pub const IMX_ADMA_ACM_SPDIF0_TX_CLK_SEL: c_int = 20;
pub const IMX_ADMA_ACM_SPDIF1_TX_CLK_SEL: c_int = 21;
pub const IMX_ADMA_ACM_MQS_TX_CLK_SEL: c_int = 22;
pub const IMX_ADMA_ACM_ASRC0_MUX_CLK_SEL: c_int = 23;
pub const IMX_ADMA_ACM_ASRC1_MUX_CLK_SEL: c_int = 24;
pub const IMX_ADMA_ACM_CLK_END: c_int = 25;
