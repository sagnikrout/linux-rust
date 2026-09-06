//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/src4xxx.h
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
// src4xxx.h  --  SRC4XXX ALSA SoC audio driver
//
// Copyright 2021-2022 Deqx Pty Ltd
// Author: Matt R Flax <flatmax@flatmax.com>
pub const SRC4XXX_RES_00: c_uint = 0x00;
pub const SRC4XXX_PWR_RST_01: c_uint = 0x01;
pub const SRC4XXX_RESET: c_uint = 0x80;
pub const SRC4XXX_POWER_DOWN: c_uint = 0x00;
pub const SRC4XXX_POWER_ENABLE: c_uint = 0x20;
pub const SRC4XXX_ENABLE_SRC: c_uint = 0x1;
pub const SRC4XXX_ENABLE_SRC_SHIFT: c_int = 0;
pub const SRC4XXX_ENABLE_DIR: c_uint = 0x2;
pub const SRC4XXX_ENABLE_DIR_SHIFT: c_int = 1;
pub const SRC4XXX_ENABLE_DIT: c_uint = 0x4;
pub const SRC4XXX_ENABLE_DIT_SHIFT: c_int = 2;
pub const SRC4XXX_ENABLE_PORT_B: c_uint = 0x8;
pub const SRC4XXX_ENABLE_PORT_B_SHIFT: c_int = 3;
pub const SRC4XXX_ENABLE_PORT_A: c_uint = 0x10;
pub const SRC4XXX_ENABLE_PORT_A_SHIFT: c_int = 4;
pub const SRC4XXX_PORTA_CTL_03: c_uint = 0x03;
pub const SRC4XXX_BUS_MASTER: c_uint = 0x8;
pub const SRC4XXX_BUS_LEFT_J: c_uint = 0x0;
pub const SRC4XXX_BUS_I2S: c_uint = 0x1;
pub const SRC4XXX_BUS_RIGHT_J_16: c_uint = 0x4;
pub const SRC4XXX_BUS_RIGHT_J_18: c_uint = 0x5;
pub const SRC4XXX_BUS_RIGHT_J_20: c_uint = 0x6;
pub const SRC4XXX_BUS_RIGHT_J_24: c_uint = 0x7;
pub const SRC4XXX_BUS_FMT_MS_MASK: c_uint = 0xf;
pub const SRC4XXX_PORTA_CTL_04: c_uint = 0x04;
pub const SRC4XXX_MCLK_DIV_MASK: c_uint = 0x3;

pub const SRC4XXX_PORTB_CTL_05: c_uint = 0x05;
pub const SRC4XXX_PORTB_CTL_06: c_uint = 0x06;
pub const SRC4XXX_TX_CTL_07: c_uint = 0x07;
pub const SRC4XXX_TX_MCLK_DIV_MASK: c_uint = 0x60;
pub const SRC4XXX_TX_MCLK_DIV_SHIFT: c_int = 5;
pub const SRC4XXX_TX_CTL_08: c_uint = 0x08;
pub const SRC4XXX_TX_CTL_09: c_uint = 0x09;
pub const SRC4XXX_SRC_DIT_IRQ_MSK_0B: c_uint = 0x0B;
pub const SRC4XXX_SRC_BTI_EN: c_uint = 0x01;
pub const SRC4XXX_SRC_TSLIP_EN: c_uint = 0x02;
pub const SRC4XXX_SRC_DIT_IRQ_MODE_0C: c_uint = 0x0C;
pub const SRC4XXX_RCV_CTL_0D: c_uint = 0x0D;
pub const SRC4XXX_RXCLK_RXCKI: c_uint = 0x0;
pub const SRC4XXX_RXCLK_MCLK: c_uint = 0x8;
pub const SRC4XXX_RCV_CTL_0E: c_uint = 0x0E;
pub const SRC4XXX_REC_MCLK_EN: c_uint = 0x1;

pub const SRC4XXX_PLL2_LOL: c_uint = 0x8;
pub const SRC4XXX_RCV_PLL_0F: c_uint = 0x0F;
pub const SRC4XXX_RCV_PLL_10: c_uint = 0x10;
pub const SRC4XXX_RCV_PLL_11: c_uint = 0x11;
pub const SRC4XXX_RVC_IRQ_MSK_16: c_uint = 0x16;
pub const SRC4XXX_RVC_IRQ_MSK_17: c_uint = 0x17;
pub const SRC4XXX_RVC_IRQ_MODE_18: c_uint = 0x18;
pub const SRC4XXX_RVC_IRQ_MODE_19: c_uint = 0x19;
pub const SRC4XXX_RVC_IRQ_MODE_1A: c_uint = 0x1A;
pub const SRC4XXX_GPIO_1_1B: c_uint = 0x1B;
pub const SRC4XXX_GPIO_2_1C: c_uint = 0x1C;
pub const SRC4XXX_GPIO_3_1D: c_uint = 0x1D;
pub const SRC4XXX_GPIO_4_1E: c_uint = 0x1E;
pub const SRC4XXX_SCR_CTL_2D: c_uint = 0x2D;
pub const SRC4XXX_SCR_CTL_2E: c_uint = 0x2E;
pub const SRC4XXX_SCR_CTL_2F: c_uint = 0x2F;
pub const SRC4XXX_SCR_CTL_30: c_uint = 0x30;
pub const SRC4XXX_SCR_CTL_31: c_uint = 0x31;
pub const SRC4XXX_PAGE_SEL_7F: c_uint = 0x7F;
// read only registers
pub const SRC4XXX_GLOBAL_ITR_STS_02: c_uint = 0x02;
pub const SRC4XXX_SRC_DIT_STS_0A: c_uint = 0x0A;
pub const SRC4XXX_NON_AUDIO_D_12: c_uint = 0x12;
pub const SRC4XXX_RVC_STS_13: c_uint = 0x13;
pub const SRC4XXX_RVC_STS_14: c_uint = 0x14;
pub const SRC4XXX_RVC_STS_15: c_uint = 0x15;
pub const SRC4XXX_SUB_CODE_1F: c_uint = 0x1F;
pub const SRC4XXX_SUB_CODE_20: c_uint = 0x20;
pub const SRC4XXX_SUB_CODE_21: c_uint = 0x21;
pub const SRC4XXX_SUB_CODE_22: c_uint = 0x22;
pub const SRC4XXX_SUB_CODE_23: c_uint = 0x23;
pub const SRC4XXX_SUB_CODE_24: c_uint = 0x24;
pub const SRC4XXX_SUB_CODE_25: c_uint = 0x25;
pub const SRC4XXX_SUB_CODE_26: c_uint = 0x26;
pub const SRC4XXX_SUB_CODE_27: c_uint = 0x27;
pub const SRC4XXX_SUB_CODE_28: c_uint = 0x28;
pub const SRC4XXX_PC_PREAMBLE_HI_29: c_uint = 0x29;
pub const SRC4XXX_PC_PREAMBLE_LO_2A: c_uint = 0x2A;
pub const SRC4XXX_PD_PREAMBLE_HI_2B: c_uint = 0x2B;
pub const SRC4XXX_PC_PREAMBLE_LO_2C: c_uint = 0x2C;
pub const SRC4XXX_IO_RATIO_32: c_uint = 0x32;
pub const SRC4XXX_IO_RATIO_33: c_uint = 0x33;
