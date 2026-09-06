//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lochnagar1_regs.h
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
// Lochnagar1 register definitions
//
// Copyright (c) 2017-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//
// Register Addresses
pub const LOCHNAGAR1_CDC_AIF1_SEL: c_uint = 0x0008;
pub const LOCHNAGAR1_CDC_AIF2_SEL: c_uint = 0x0009;
pub const LOCHNAGAR1_CDC_AIF3_SEL: c_uint = 0x000A;
pub const LOCHNAGAR1_CDC_MCLK1_SEL: c_uint = 0x000B;
pub const LOCHNAGAR1_CDC_MCLK2_SEL: c_uint = 0x000C;
pub const LOCHNAGAR1_CDC_AIF_CTRL1: c_uint = 0x000D;
pub const LOCHNAGAR1_CDC_AIF_CTRL2: c_uint = 0x000E;
pub const LOCHNAGAR1_EXT_AIF_CTRL: c_uint = 0x000F;
pub const LOCHNAGAR1_DSP_AIF1_SEL: c_uint = 0x0010;
pub const LOCHNAGAR1_DSP_AIF2_SEL: c_uint = 0x0011;
pub const LOCHNAGAR1_DSP_CLKIN_SEL: c_uint = 0x0012;
pub const LOCHNAGAR1_DSP_AIF: c_uint = 0x0013;
pub const LOCHNAGAR1_GF_AIF1: c_uint = 0x0014;
pub const LOCHNAGAR1_GF_AIF2: c_uint = 0x0015;
pub const LOCHNAGAR1_PSIA_AIF: c_uint = 0x0016;
pub const LOCHNAGAR1_PSIA1_SEL: c_uint = 0x0017;
pub const LOCHNAGAR1_PSIA2_SEL: c_uint = 0x0018;
pub const LOCHNAGAR1_SPDIF_AIF_SEL: c_uint = 0x0019;
pub const LOCHNAGAR1_GF_AIF3_SEL: c_uint = 0x001C;
pub const LOCHNAGAR1_GF_AIF4_SEL: c_uint = 0x001D;
pub const LOCHNAGAR1_GF_CLKOUT1_SEL: c_uint = 0x001E;
pub const LOCHNAGAR1_GF_AIF1_SEL: c_uint = 0x001F;
pub const LOCHNAGAR1_GF_AIF2_SEL: c_uint = 0x0020;
pub const LOCHNAGAR1_GF_GPIO2: c_uint = 0x0026;
pub const LOCHNAGAR1_GF_GPIO3: c_uint = 0x0027;
pub const LOCHNAGAR1_GF_GPIO7: c_uint = 0x0028;
pub const LOCHNAGAR1_RST: c_uint = 0x0029;
pub const LOCHNAGAR1_LED1: c_uint = 0x002A;
pub const LOCHNAGAR1_LED2: c_uint = 0x002B;
pub const LOCHNAGAR1_I2C_CTRL: c_uint = 0x0046;
//
// (0x0008 - 0x000C, 0x0010 - 0x0012, 0x0017 - 0x0020)
// CDC_AIF1_SEL - GF_AIF2_SEL
//
pub const LOCHNAGAR1_SRC_MASK: c_uint = 0xFF;
pub const LOCHNAGAR1_SRC_SHIFT: c_int = 0;
// (0x000D)  CDC_AIF_CTRL1
pub const LOCHNAGAR1_CDC_AIF2_LRCLK_DIR_MASK: c_uint = 0x40;
pub const LOCHNAGAR1_CDC_AIF2_LRCLK_DIR_SHIFT: c_int = 6;
pub const LOCHNAGAR1_CDC_AIF2_BCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_CDC_AIF2_BCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_CDC_AIF2_ENA_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_CDC_AIF2_ENA_SHIFT: c_int = 4;
pub const LOCHNAGAR1_CDC_AIF1_LRCLK_DIR_MASK: c_uint = 0x04;
pub const LOCHNAGAR1_CDC_AIF1_LRCLK_DIR_SHIFT: c_int = 2;
pub const LOCHNAGAR1_CDC_AIF1_BCLK_DIR_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_CDC_AIF1_BCLK_DIR_SHIFT: c_int = 1;
pub const LOCHNAGAR1_CDC_AIF1_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_CDC_AIF1_ENA_SHIFT: c_int = 0;
// (0x000E)  CDC_AIF_CTRL2
pub const LOCHNAGAR1_CDC_AIF3_LRCLK_DIR_MASK: c_uint = 0x40;
pub const LOCHNAGAR1_CDC_AIF3_LRCLK_DIR_SHIFT: c_int = 6;
pub const LOCHNAGAR1_CDC_AIF3_BCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_CDC_AIF3_BCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_CDC_AIF3_ENA_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_CDC_AIF3_ENA_SHIFT: c_int = 4;
pub const LOCHNAGAR1_CDC_MCLK1_ENA_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_CDC_MCLK1_ENA_SHIFT: c_int = 1;
pub const LOCHNAGAR1_CDC_MCLK2_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_CDC_MCLK2_ENA_SHIFT: c_int = 0;
// (0x000F)  EXT_AIF_CTRL
pub const LOCHNAGAR1_SPDIF_AIF_LRCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_SPDIF_AIF_LRCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_SPDIF_AIF_BCLK_DIR_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_SPDIF_AIF_BCLK_DIR_SHIFT: c_int = 4;
pub const LOCHNAGAR1_SPDIF_AIF_ENA_MASK: c_uint = 0x08;
pub const LOCHNAGAR1_SPDIF_AIF_ENA_SHIFT: c_int = 3;
// (0x0013)  DSP_AIF
pub const LOCHNAGAR1_DSP_AIF2_LRCLK_DIR_MASK: c_uint = 0x40;
pub const LOCHNAGAR1_DSP_AIF2_LRCLK_DIR_SHIFT: c_int = 6;
pub const LOCHNAGAR1_DSP_AIF2_BCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_DSP_AIF2_BCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_DSP_AIF2_ENA_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_DSP_AIF2_ENA_SHIFT: c_int = 4;
pub const LOCHNAGAR1_DSP_CLKIN_ENA_MASK: c_uint = 0x08;
pub const LOCHNAGAR1_DSP_CLKIN_ENA_SHIFT: c_int = 3;
pub const LOCHNAGAR1_DSP_AIF1_LRCLK_DIR_MASK: c_uint = 0x04;
pub const LOCHNAGAR1_DSP_AIF1_LRCLK_DIR_SHIFT: c_int = 2;
pub const LOCHNAGAR1_DSP_AIF1_BCLK_DIR_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_DSP_AIF1_BCLK_DIR_SHIFT: c_int = 1;
pub const LOCHNAGAR1_DSP_AIF1_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_DSP_AIF1_ENA_SHIFT: c_int = 0;
// (0x0014)  GF_AIF1
pub const LOCHNAGAR1_GF_CLKOUT1_ENA_MASK: c_uint = 0x40;
pub const LOCHNAGAR1_GF_CLKOUT1_ENA_SHIFT: c_int = 6;
pub const LOCHNAGAR1_GF_AIF3_LRCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_GF_AIF3_LRCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_GF_AIF3_BCLK_DIR_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_GF_AIF3_BCLK_DIR_SHIFT: c_int = 4;
pub const LOCHNAGAR1_GF_AIF3_ENA_MASK: c_uint = 0x08;
pub const LOCHNAGAR1_GF_AIF3_ENA_SHIFT: c_int = 3;
pub const LOCHNAGAR1_GF_AIF1_LRCLK_DIR_MASK: c_uint = 0x04;
pub const LOCHNAGAR1_GF_AIF1_LRCLK_DIR_SHIFT: c_int = 2;
pub const LOCHNAGAR1_GF_AIF1_BCLK_DIR_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_GF_AIF1_BCLK_DIR_SHIFT: c_int = 1;
pub const LOCHNAGAR1_GF_AIF1_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_GF_AIF1_ENA_SHIFT: c_int = 0;
// (0x0015)  GF_AIF2
pub const LOCHNAGAR1_GF_AIF4_LRCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_GF_AIF4_LRCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_GF_AIF4_BCLK_DIR_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_GF_AIF4_BCLK_DIR_SHIFT: c_int = 4;
pub const LOCHNAGAR1_GF_AIF4_ENA_MASK: c_uint = 0x08;
pub const LOCHNAGAR1_GF_AIF4_ENA_SHIFT: c_int = 3;
pub const LOCHNAGAR1_GF_AIF2_LRCLK_DIR_MASK: c_uint = 0x04;
pub const LOCHNAGAR1_GF_AIF2_LRCLK_DIR_SHIFT: c_int = 2;
pub const LOCHNAGAR1_GF_AIF2_BCLK_DIR_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_GF_AIF2_BCLK_DIR_SHIFT: c_int = 1;
pub const LOCHNAGAR1_GF_AIF2_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_GF_AIF2_ENA_SHIFT: c_int = 0;
// (0x0016)  PSIA_AIF
pub const LOCHNAGAR1_PSIA2_LRCLK_DIR_MASK: c_uint = 0x40;
pub const LOCHNAGAR1_PSIA2_LRCLK_DIR_SHIFT: c_int = 6;
pub const LOCHNAGAR1_PSIA2_BCLK_DIR_MASK: c_uint = 0x20;
pub const LOCHNAGAR1_PSIA2_BCLK_DIR_SHIFT: c_int = 5;
pub const LOCHNAGAR1_PSIA2_ENA_MASK: c_uint = 0x10;
pub const LOCHNAGAR1_PSIA2_ENA_SHIFT: c_int = 4;
pub const LOCHNAGAR1_PSIA1_LRCLK_DIR_MASK: c_uint = 0x04;
pub const LOCHNAGAR1_PSIA1_LRCLK_DIR_SHIFT: c_int = 2;
pub const LOCHNAGAR1_PSIA1_BCLK_DIR_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_PSIA1_BCLK_DIR_SHIFT: c_int = 1;
pub const LOCHNAGAR1_PSIA1_ENA_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_PSIA1_ENA_SHIFT: c_int = 0;
// (0x0029)  RST
pub const LOCHNAGAR1_DSP_RESET_MASK: c_uint = 0x02;
pub const LOCHNAGAR1_DSP_RESET_SHIFT: c_int = 1;
pub const LOCHNAGAR1_CDC_RESET_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_CDC_RESET_SHIFT: c_int = 0;
// (0x0046)  I2C_CTRL
pub const LOCHNAGAR1_CDC_CIF_MODE_MASK: c_uint = 0x01;
pub const LOCHNAGAR1_CDC_CIF_MODE_SHIFT: c_int = 0;
