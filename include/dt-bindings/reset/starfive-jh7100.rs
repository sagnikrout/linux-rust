//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/starfive-jh7100.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2021 Ahmad Fatoum, Pengutronix
//
pub const JH7100_RSTN_DOM3AHB_BUS: c_int = 0;
pub const JH7100_RSTN_DOM7AHB_BUS: c_int = 1;
pub const JH7100_RST_U74: c_int = 2;
pub const JH7100_RSTN_U74_AXI: c_int = 3;
pub const JH7100_RSTN_SGDMA2P_AHB: c_int = 4;
pub const JH7100_RSTN_SGDMA2P_AXI: c_int = 5;
pub const JH7100_RSTN_DMA2PNOC_AXI: c_int = 6;
pub const JH7100_RSTN_DLA_AXI: c_int = 7;
pub const JH7100_RSTN_DLANOC_AXI: c_int = 8;
pub const JH7100_RSTN_DLA_APB: c_int = 9;
pub const JH7100_RST_VP6_DRESET: c_int = 10;
pub const JH7100_RST_VP6_BRESET: c_int = 11;
pub const JH7100_RSTN_VP6_AXI: c_int = 12;
pub const JH7100_RSTN_VDECBRG_MAIN: c_int = 13;
pub const JH7100_RSTN_VDEC_AXI: c_int = 14;
pub const JH7100_RSTN_VDEC_BCLK: c_int = 15;
pub const JH7100_RSTN_VDEC_CCLK: c_int = 16;
pub const JH7100_RSTN_VDEC_APB: c_int = 17;
pub const JH7100_RSTN_JPEG_AXI: c_int = 18;
pub const JH7100_RSTN_JPEG_CCLK: c_int = 19;
pub const JH7100_RSTN_JPEG_APB: c_int = 20;
pub const JH7100_RSTN_JPCGC300_MAIN: c_int = 21;
pub const JH7100_RSTN_GC300_2X: c_int = 22;
pub const JH7100_RSTN_GC300_AXI: c_int = 23;
pub const JH7100_RSTN_GC300_AHB: c_int = 24;
pub const JH7100_RSTN_VENC_AXI: c_int = 25;
pub const JH7100_RSTN_VENCBRG_MAIN: c_int = 26;
pub const JH7100_RSTN_VENC_BCLK: c_int = 27;
pub const JH7100_RSTN_VENC_CCLK: c_int = 28;
pub const JH7100_RSTN_VENC_APB: c_int = 29;
pub const JH7100_RSTN_DDRPHY_APB: c_int = 30;
pub const JH7100_RSTN_NOC_ROB: c_int = 31;
pub const JH7100_RSTN_NOC_COG: c_int = 32;
pub const JH7100_RSTN_HIFI4_AXI: c_int = 33;
pub const JH7100_RSTN_HIFI4NOC_AXI: c_int = 34;
pub const JH7100_RST_HIFI4_DRESET: c_int = 35;
pub const JH7100_RST_HIFI4_BRESET: c_int = 36;
pub const JH7100_RSTN_USB_AXI: c_int = 37;
pub const JH7100_RSTN_USBNOC_AXI: c_int = 38;
pub const JH7100_RSTN_SGDMA1P_AXI: c_int = 39;
pub const JH7100_RSTN_DMA1P_AXI: c_int = 40;
pub const JH7100_RSTN_X2C_AXI: c_int = 41;
pub const JH7100_RSTN_NNE_AHB: c_int = 42;
pub const JH7100_RSTN_NNE_AXI: c_int = 43;
pub const JH7100_RSTN_NNENOC_AXI: c_int = 44;
pub const JH7100_RSTN_DLASLV_AXI: c_int = 45;
pub const JH7100_RSTN_DSPX2C_AXI: c_int = 46;
pub const JH7100_RSTN_VIN_SRC: c_int = 47;
pub const JH7100_RSTN_ISPSLV_AXI: c_int = 48;
pub const JH7100_RSTN_VIN_AXI: c_int = 49;
pub const JH7100_RSTN_VINNOC_AXI: c_int = 50;
pub const JH7100_RSTN_ISP0_AXI: c_int = 51;
pub const JH7100_RSTN_ISP0NOC_AXI: c_int = 52;
pub const JH7100_RSTN_ISP1_AXI: c_int = 53;
pub const JH7100_RSTN_ISP1NOC_AXI: c_int = 54;
pub const JH7100_RSTN_VOUT_SRC: c_int = 55;
pub const JH7100_RSTN_DISP_AXI: c_int = 56;
pub const JH7100_RSTN_DISPNOC_AXI: c_int = 57;
pub const JH7100_RSTN_SDIO0_AHB: c_int = 58;
pub const JH7100_RSTN_SDIO1_AHB: c_int = 59;
pub const JH7100_RSTN_GMAC_AHB: c_int = 60;
pub const JH7100_RSTN_SPI2AHB_AHB: c_int = 61;
pub const JH7100_RSTN_SPI2AHB_CORE: c_int = 62;
pub const JH7100_RSTN_EZMASTER_AHB: c_int = 63;
pub const JH7100_RST_E24: c_int = 64;
pub const JH7100_RSTN_QSPI_AHB: c_int = 65;
pub const JH7100_RSTN_QSPI_CORE: c_int = 66;
pub const JH7100_RSTN_QSPI_APB: c_int = 67;
pub const JH7100_RSTN_SEC_AHB: c_int = 68;
pub const JH7100_RSTN_AES: c_int = 69;
pub const JH7100_RSTN_PKA: c_int = 70;
pub const JH7100_RSTN_SHA: c_int = 71;
pub const JH7100_RSTN_TRNG_APB: c_int = 72;
pub const JH7100_RSTN_OTP_APB: c_int = 73;
pub const JH7100_RSTN_UART0_APB: c_int = 74;
pub const JH7100_RSTN_UART0_CORE: c_int = 75;
pub const JH7100_RSTN_UART1_APB: c_int = 76;
pub const JH7100_RSTN_UART1_CORE: c_int = 77;
pub const JH7100_RSTN_SPI0_APB: c_int = 78;
pub const JH7100_RSTN_SPI0_CORE: c_int = 79;
pub const JH7100_RSTN_SPI1_APB: c_int = 80;
pub const JH7100_RSTN_SPI1_CORE: c_int = 81;
pub const JH7100_RSTN_I2C0_APB: c_int = 82;
pub const JH7100_RSTN_I2C0_CORE: c_int = 83;
pub const JH7100_RSTN_I2C1_APB: c_int = 84;
pub const JH7100_RSTN_I2C1_CORE: c_int = 85;
pub const JH7100_RSTN_GPIO_APB: c_int = 86;
pub const JH7100_RSTN_UART2_APB: c_int = 87;
pub const JH7100_RSTN_UART2_CORE: c_int = 88;
pub const JH7100_RSTN_UART3_APB: c_int = 89;
pub const JH7100_RSTN_UART3_CORE: c_int = 90;
pub const JH7100_RSTN_SPI2_APB: c_int = 91;
pub const JH7100_RSTN_SPI2_CORE: c_int = 92;
pub const JH7100_RSTN_SPI3_APB: c_int = 93;
pub const JH7100_RSTN_SPI3_CORE: c_int = 94;
pub const JH7100_RSTN_I2C2_APB: c_int = 95;
pub const JH7100_RSTN_I2C2_CORE: c_int = 96;
pub const JH7100_RSTN_I2C3_APB: c_int = 97;
pub const JH7100_RSTN_I2C3_CORE: c_int = 98;
pub const JH7100_RSTN_WDTIMER_APB: c_int = 99;
pub const JH7100_RSTN_WDT: c_int = 100;
pub const JH7100_RSTN_TIMER0: c_int = 101;
pub const JH7100_RSTN_TIMER1: c_int = 102;
pub const JH7100_RSTN_TIMER2: c_int = 103;
pub const JH7100_RSTN_TIMER3: c_int = 104;
pub const JH7100_RSTN_TIMER4: c_int = 105;
pub const JH7100_RSTN_TIMER5: c_int = 106;
pub const JH7100_RSTN_TIMER6: c_int = 107;
pub const JH7100_RSTN_VP6INTC_APB: c_int = 108;
pub const JH7100_RSTN_PWM_APB: c_int = 109;
pub const JH7100_RSTN_MSI_APB: c_int = 110;
pub const JH7100_RSTN_TEMP_APB: c_int = 111;
pub const JH7100_RSTN_TEMP_SENSE: c_int = 112;
pub const JH7100_RSTN_SYSERR_APB: c_int = 113;
pub const JH7100_RSTN_END: c_int = 114;
