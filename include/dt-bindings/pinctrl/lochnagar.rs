//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/lochnagar.h
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
// Device Tree defines for Lochnagar pinctrl
//
// Copyright (c) 2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//
pub const LOCHNAGAR1_PIN_CDC_RESET: c_int = 0;
pub const LOCHNAGAR1_PIN_DSP_RESET: c_int = 1;
pub const LOCHNAGAR1_PIN_CDC_CIF1MODE: c_int = 2;
pub const LOCHNAGAR1_PIN_NUM_GPIOS: c_int = 3;
pub const LOCHNAGAR2_PIN_CDC_RESET: c_int = 0;
pub const LOCHNAGAR2_PIN_DSP_RESET: c_int = 1;
pub const LOCHNAGAR2_PIN_CDC_CIF1MODE: c_int = 2;
pub const LOCHNAGAR2_PIN_CDC_LDOENA: c_int = 3;
pub const LOCHNAGAR2_PIN_SPDIF_HWMODE: c_int = 4;
pub const LOCHNAGAR2_PIN_SPDIF_RESET: c_int = 5;
pub const LOCHNAGAR2_PIN_FPGA_GPIO1: c_int = 6;
pub const LOCHNAGAR2_PIN_FPGA_GPIO2: c_int = 7;
pub const LOCHNAGAR2_PIN_FPGA_GPIO3: c_int = 8;
pub const LOCHNAGAR2_PIN_FPGA_GPIO4: c_int = 9;
pub const LOCHNAGAR2_PIN_FPGA_GPIO5: c_int = 10;
pub const LOCHNAGAR2_PIN_FPGA_GPIO6: c_int = 11;
pub const LOCHNAGAR2_PIN_CDC_GPIO1: c_int = 12;
pub const LOCHNAGAR2_PIN_CDC_GPIO2: c_int = 13;
pub const LOCHNAGAR2_PIN_CDC_GPIO3: c_int = 14;
pub const LOCHNAGAR2_PIN_CDC_GPIO4: c_int = 15;
pub const LOCHNAGAR2_PIN_CDC_GPIO5: c_int = 16;
pub const LOCHNAGAR2_PIN_CDC_GPIO6: c_int = 17;
pub const LOCHNAGAR2_PIN_CDC_GPIO7: c_int = 18;
pub const LOCHNAGAR2_PIN_CDC_GPIO8: c_int = 19;
pub const LOCHNAGAR2_PIN_DSP_GPIO1: c_int = 20;
pub const LOCHNAGAR2_PIN_DSP_GPIO2: c_int = 21;
pub const LOCHNAGAR2_PIN_DSP_GPIO3: c_int = 22;
pub const LOCHNAGAR2_PIN_DSP_GPIO4: c_int = 23;
pub const LOCHNAGAR2_PIN_DSP_GPIO5: c_int = 24;
pub const LOCHNAGAR2_PIN_DSP_GPIO6: c_int = 25;
pub const LOCHNAGAR2_PIN_GF_GPIO2: c_int = 26;
pub const LOCHNAGAR2_PIN_GF_GPIO3: c_int = 27;
pub const LOCHNAGAR2_PIN_GF_GPIO7: c_int = 28;
pub const LOCHNAGAR2_PIN_CDC_AIF1_BCLK: c_int = 29;
pub const LOCHNAGAR2_PIN_CDC_AIF1_RXDAT: c_int = 30;
pub const LOCHNAGAR2_PIN_CDC_AIF1_LRCLK: c_int = 31;
pub const LOCHNAGAR2_PIN_CDC_AIF1_TXDAT: c_int = 32;
pub const LOCHNAGAR2_PIN_CDC_AIF2_BCLK: c_int = 33;
pub const LOCHNAGAR2_PIN_CDC_AIF2_RXDAT: c_int = 34;
pub const LOCHNAGAR2_PIN_CDC_AIF2_LRCLK: c_int = 35;
pub const LOCHNAGAR2_PIN_CDC_AIF2_TXDAT: c_int = 36;
pub const LOCHNAGAR2_PIN_CDC_AIF3_BCLK: c_int = 37;
pub const LOCHNAGAR2_PIN_CDC_AIF3_RXDAT: c_int = 38;
pub const LOCHNAGAR2_PIN_CDC_AIF3_LRCLK: c_int = 39;
pub const LOCHNAGAR2_PIN_CDC_AIF3_TXDAT: c_int = 40;
pub const LOCHNAGAR2_PIN_DSP_AIF1_BCLK: c_int = 41;
pub const LOCHNAGAR2_PIN_DSP_AIF1_RXDAT: c_int = 42;
pub const LOCHNAGAR2_PIN_DSP_AIF1_LRCLK: c_int = 43;
pub const LOCHNAGAR2_PIN_DSP_AIF1_TXDAT: c_int = 44;
pub const LOCHNAGAR2_PIN_DSP_AIF2_BCLK: c_int = 45;
pub const LOCHNAGAR2_PIN_DSP_AIF2_RXDAT: c_int = 46;
pub const LOCHNAGAR2_PIN_DSP_AIF2_LRCLK: c_int = 47;
pub const LOCHNAGAR2_PIN_DSP_AIF2_TXDAT: c_int = 48;
pub const LOCHNAGAR2_PIN_PSIA1_BCLK: c_int = 49;
pub const LOCHNAGAR2_PIN_PSIA1_RXDAT: c_int = 50;
pub const LOCHNAGAR2_PIN_PSIA1_LRCLK: c_int = 51;
pub const LOCHNAGAR2_PIN_PSIA1_TXDAT: c_int = 52;
pub const LOCHNAGAR2_PIN_PSIA2_BCLK: c_int = 53;
pub const LOCHNAGAR2_PIN_PSIA2_RXDAT: c_int = 54;
pub const LOCHNAGAR2_PIN_PSIA2_LRCLK: c_int = 55;
pub const LOCHNAGAR2_PIN_PSIA2_TXDAT: c_int = 56;
pub const LOCHNAGAR2_PIN_GF_AIF3_BCLK: c_int = 57;
pub const LOCHNAGAR2_PIN_GF_AIF3_RXDAT: c_int = 58;
pub const LOCHNAGAR2_PIN_GF_AIF3_LRCLK: c_int = 59;
pub const LOCHNAGAR2_PIN_GF_AIF3_TXDAT: c_int = 60;
pub const LOCHNAGAR2_PIN_GF_AIF4_BCLK: c_int = 61;
pub const LOCHNAGAR2_PIN_GF_AIF4_RXDAT: c_int = 62;
pub const LOCHNAGAR2_PIN_GF_AIF4_LRCLK: c_int = 63;
pub const LOCHNAGAR2_PIN_GF_AIF4_TXDAT: c_int = 64;
pub const LOCHNAGAR2_PIN_GF_AIF1_BCLK: c_int = 65;
pub const LOCHNAGAR2_PIN_GF_AIF1_RXDAT: c_int = 66;
pub const LOCHNAGAR2_PIN_GF_AIF1_LRCLK: c_int = 67;
pub const LOCHNAGAR2_PIN_GF_AIF1_TXDAT: c_int = 68;
pub const LOCHNAGAR2_PIN_GF_AIF2_BCLK: c_int = 69;
pub const LOCHNAGAR2_PIN_GF_AIF2_RXDAT: c_int = 70;
pub const LOCHNAGAR2_PIN_GF_AIF2_LRCLK: c_int = 71;
pub const LOCHNAGAR2_PIN_GF_AIF2_TXDAT: c_int = 72;
pub const LOCHNAGAR2_PIN_DSP_UART1_RX: c_int = 73;
pub const LOCHNAGAR2_PIN_DSP_UART1_TX: c_int = 74;
pub const LOCHNAGAR2_PIN_DSP_UART2_RX: c_int = 75;
pub const LOCHNAGAR2_PIN_DSP_UART2_TX: c_int = 76;
pub const LOCHNAGAR2_PIN_GF_UART2_RX: c_int = 77;
pub const LOCHNAGAR2_PIN_GF_UART2_TX: c_int = 78;
pub const LOCHNAGAR2_PIN_USB_UART_RX: c_int = 79;
pub const LOCHNAGAR2_PIN_CDC_PDMCLK1: c_int = 80;
pub const LOCHNAGAR2_PIN_CDC_PDMDAT1: c_int = 81;
pub const LOCHNAGAR2_PIN_CDC_PDMCLK2: c_int = 82;
pub const LOCHNAGAR2_PIN_CDC_PDMDAT2: c_int = 83;
pub const LOCHNAGAR2_PIN_CDC_DMICCLK1: c_int = 84;
pub const LOCHNAGAR2_PIN_CDC_DMICDAT1: c_int = 85;
pub const LOCHNAGAR2_PIN_CDC_DMICCLK2: c_int = 86;
pub const LOCHNAGAR2_PIN_CDC_DMICDAT2: c_int = 87;
pub const LOCHNAGAR2_PIN_CDC_DMICCLK3: c_int = 88;
pub const LOCHNAGAR2_PIN_CDC_DMICDAT3: c_int = 89;
pub const LOCHNAGAR2_PIN_CDC_DMICCLK4: c_int = 90;
pub const LOCHNAGAR2_PIN_CDC_DMICDAT4: c_int = 91;
pub const LOCHNAGAR2_PIN_DSP_DMICCLK1: c_int = 92;
pub const LOCHNAGAR2_PIN_DSP_DMICDAT1: c_int = 93;
pub const LOCHNAGAR2_PIN_DSP_DMICCLK2: c_int = 94;
pub const LOCHNAGAR2_PIN_DSP_DMICDAT2: c_int = 95;
pub const LOCHNAGAR2_PIN_I2C2_SCL: c_int = 96;
pub const LOCHNAGAR2_PIN_I2C2_SDA: c_int = 97;
pub const LOCHNAGAR2_PIN_I2C3_SCL: c_int = 98;
pub const LOCHNAGAR2_PIN_I2C3_SDA: c_int = 99;
pub const LOCHNAGAR2_PIN_I2C4_SCL: c_int = 100;
pub const LOCHNAGAR2_PIN_I2C4_SDA: c_int = 101;
pub const LOCHNAGAR2_PIN_DSP_STANDBY: c_int = 102;
pub const LOCHNAGAR2_PIN_CDC_MCLK1: c_int = 103;
pub const LOCHNAGAR2_PIN_CDC_MCLK2: c_int = 104;
pub const LOCHNAGAR2_PIN_DSP_CLKIN: c_int = 105;
pub const LOCHNAGAR2_PIN_PSIA1_MCLK: c_int = 106;
pub const LOCHNAGAR2_PIN_PSIA2_MCLK: c_int = 107;
pub const LOCHNAGAR2_PIN_GF_GPIO1: c_int = 108;
pub const LOCHNAGAR2_PIN_GF_GPIO5: c_int = 109;
pub const LOCHNAGAR2_PIN_DSP_GPIO20: c_int = 110;
pub const LOCHNAGAR2_PIN_NUM_GPIOS: c_int = 111;
