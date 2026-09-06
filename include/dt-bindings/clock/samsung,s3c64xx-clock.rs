//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/samsung,s3c64xx-clock.h
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
// Copyright (c) 2013 Tomasz Figa <tomasz.figa at gmail.com>
//
// Device Tree binding constants for Samsung S3C64xx clock controller.
//
// Let each exported clock get a unique index, which is used on DT-enabled
// platforms to lookup the clock from a clock specifier. These indices are
// therefore considered an ABI and so must not be changed. This implies
// that new clocks should be added either in free spaces between clock groups
// or at the end.
//
// Core clocks.
pub const CLK27M: c_int = 1;
pub const CLK48M: c_int = 2;
pub const FOUT_APLL: c_int = 3;
pub const FOUT_MPLL: c_int = 4;
pub const FOUT_EPLL: c_int = 5;
pub const ARMCLK: c_int = 6;
pub const HCLKX2: c_int = 7;
pub const HCLK: c_int = 8;
pub const PCLK: c_int = 9;
// HCLK bus clocks.
pub const HCLK_3DSE: c_int = 16;
pub const HCLK_UHOST: c_int = 17;
pub const HCLK_SECUR: c_int = 18;
pub const HCLK_SDMA1: c_int = 19;
pub const HCLK_SDMA0: c_int = 20;
pub const HCLK_IROM: c_int = 21;
pub const HCLK_DDR1: c_int = 22;
pub const HCLK_MEM1: c_int = 23;
pub const HCLK_MEM0: c_int = 24;
pub const HCLK_USB: c_int = 25;
pub const HCLK_HSMMC2: c_int = 26;
pub const HCLK_HSMMC1: c_int = 27;
pub const HCLK_HSMMC0: c_int = 28;
pub const HCLK_MDP: c_int = 29;
pub const HCLK_DHOST: c_int = 30;
pub const HCLK_IHOST: c_int = 31;
pub const HCLK_DMA1: c_int = 32;
pub const HCLK_DMA0: c_int = 33;
pub const HCLK_JPEG: c_int = 34;
pub const HCLK_CAMIF: c_int = 35;
pub const HCLK_SCALER: c_int = 36;
pub const HCLK_2D: c_int = 37;
pub const HCLK_TV: c_int = 38;
pub const HCLK_POST0: c_int = 39;
pub const HCLK_ROT: c_int = 40;
pub const HCLK_LCD: c_int = 41;
pub const HCLK_TZIC: c_int = 42;
pub const HCLK_INTC: c_int = 43;
pub const HCLK_MFC: c_int = 44;
pub const HCLK_DDR0: c_int = 45;
// PCLK bus clocks.
pub const PCLK_IIC1: c_int = 48;
pub const PCLK_IIS2: c_int = 49;
pub const PCLK_SKEY: c_int = 50;
pub const PCLK_CHIPID: c_int = 51;
pub const PCLK_SPI1: c_int = 52;
pub const PCLK_SPI0: c_int = 53;
pub const PCLK_HSIRX: c_int = 54;
pub const PCLK_HSITX: c_int = 55;
pub const PCLK_GPIO: c_int = 56;
pub const PCLK_IIC0: c_int = 57;
pub const PCLK_IIS1: c_int = 58;
pub const PCLK_IIS0: c_int = 59;
pub const PCLK_AC97: c_int = 60;
pub const PCLK_TZPC: c_int = 61;
pub const PCLK_TSADC: c_int = 62;
pub const PCLK_KEYPAD: c_int = 63;
pub const PCLK_IRDA: c_int = 64;
pub const PCLK_PCM1: c_int = 65;
pub const PCLK_PCM0: c_int = 66;
pub const PCLK_PWM: c_int = 67;
pub const PCLK_RTC: c_int = 68;
pub const PCLK_WDT: c_int = 69;
pub const PCLK_UART3: c_int = 70;
pub const PCLK_UART2: c_int = 71;
pub const PCLK_UART1: c_int = 72;
pub const PCLK_UART0: c_int = 73;
pub const PCLK_MFC: c_int = 74;
// Special clocks.
pub const SCLK_UHOST: c_int = 80;
pub const SCLK_MMC2_48: c_int = 81;
pub const SCLK_MMC1_48: c_int = 82;
pub const SCLK_MMC0_48: c_int = 83;
pub const SCLK_MMC2: c_int = 84;
pub const SCLK_MMC1: c_int = 85;
pub const SCLK_MMC0: c_int = 86;
pub const SCLK_SPI1_48: c_int = 87;
pub const SCLK_SPI0_48: c_int = 88;
pub const SCLK_SPI1: c_int = 89;
pub const SCLK_SPI0: c_int = 90;
pub const SCLK_DAC27: c_int = 91;
pub const SCLK_TV27: c_int = 92;
pub const SCLK_SCALER27: c_int = 93;
pub const SCLK_SCALER: c_int = 94;
pub const SCLK_LCD27: c_int = 95;
pub const SCLK_LCD: c_int = 96;
pub const SCLK_FIMC: c_int = 97;
pub const SCLK_POST0_27: c_int = 98;
pub const SCLK_AUDIO2: c_int = 99;
pub const SCLK_POST0: c_int = 100;
pub const SCLK_AUDIO1: c_int = 101;
pub const SCLK_AUDIO0: c_int = 102;
pub const SCLK_SECUR: c_int = 103;
pub const SCLK_IRDA: c_int = 104;
pub const SCLK_UART: c_int = 105;
pub const SCLK_MFC: c_int = 106;
pub const SCLK_CAM: c_int = 107;
pub const SCLK_JPEG: c_int = 108;
pub const SCLK_ONENAND: c_int = 109;
// MEM0 bus clocks - S3C6410-specific.
pub const MEM0_CFCON: c_int = 112;
pub const MEM0_ONENAND1: c_int = 113;
pub const MEM0_ONENAND0: c_int = 114;
pub const MEM0_NFCON: c_int = 115;
pub const MEM0_SROM: c_int = 116;
// Muxes.
pub const MOUT_APLL: c_int = 128;
pub const MOUT_MPLL: c_int = 129;
pub const MOUT_EPLL: c_int = 130;
pub const MOUT_MFC: c_int = 131;
pub const MOUT_AUDIO0: c_int = 132;
pub const MOUT_AUDIO1: c_int = 133;
pub const MOUT_UART: c_int = 134;
pub const MOUT_SPI0: c_int = 135;
pub const MOUT_SPI1: c_int = 136;
pub const MOUT_MMC0: c_int = 137;
pub const MOUT_MMC1: c_int = 138;
pub const MOUT_MMC2: c_int = 139;
pub const MOUT_UHOST: c_int = 140;
pub const MOUT_IRDA: c_int = 141;
pub const MOUT_LCD: c_int = 142;
pub const MOUT_SCALER: c_int = 143;
pub const MOUT_DAC27: c_int = 144;
pub const MOUT_TV27: c_int = 145;
pub const MOUT_AUDIO2: c_int = 146;
// Dividers.
pub const DOUT_MPLL: c_int = 160;
pub const DOUT_SECUR: c_int = 161;
pub const DOUT_CAM: c_int = 162;
pub const DOUT_JPEG: c_int = 163;
pub const DOUT_MFC: c_int = 164;
pub const DOUT_MMC0: c_int = 165;
pub const DOUT_MMC1: c_int = 166;
pub const DOUT_MMC2: c_int = 167;
pub const DOUT_LCD: c_int = 168;
pub const DOUT_SCALER: c_int = 169;
pub const DOUT_UHOST: c_int = 170;
pub const DOUT_SPI0: c_int = 171;
pub const DOUT_SPI1: c_int = 172;
pub const DOUT_AUDIO0: c_int = 173;
pub const DOUT_AUDIO1: c_int = 174;
pub const DOUT_UART: c_int = 175;
pub const DOUT_IRDA: c_int = 176;
pub const DOUT_FIMC: c_int = 177;
pub const DOUT_AUDIO2: c_int = 178;
// Total number of clocks.

