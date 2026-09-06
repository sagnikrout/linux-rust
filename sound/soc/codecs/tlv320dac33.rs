//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320dac33.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC Texas Instruments TLV320DAC33 codec driver
//
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//
// Copyright:   (C) 2009 Nokia Corporation
//
pub const DAC33_PAGE_SELECT: c_uint = 0x00;
pub const DAC33_PWR_CTRL: c_uint = 0x01;
pub const DAC33_PLL_CTRL_A: c_uint = 0x02;
pub const DAC33_PLL_CTRL_B: c_uint = 0x03;
pub const DAC33_PLL_CTRL_C: c_uint = 0x04;
pub const DAC33_PLL_CTRL_D: c_uint = 0x05;
pub const DAC33_PLL_CTRL_E: c_uint = 0x06;
pub const DAC33_INT_OSC_CTRL: c_uint = 0x07;
pub const DAC33_INT_OSC_FREQ_RAT_A: c_uint = 0x08;
pub const DAC33_INT_OSC_FREQ_RAT_B: c_uint = 0x09;
pub const DAC33_INT_OSC_DAC_RATIO_SET: c_uint = 0x0A;
pub const DAC33_CALIB_TIME: c_uint = 0x0B;
pub const DAC33_INT_OSC_CTRL_B: c_uint = 0x0C;
pub const DAC33_INT_OSC_CTRL_C: c_uint = 0x0D;
pub const DAC33_INT_OSC_STATUS: c_uint = 0x0E;
pub const DAC33_INT_OSC_DAC_RATIO_READ: c_uint = 0x0F;
pub const DAC33_INT_OSC_FREQ_RAT_READ_A: c_uint = 0x10;
pub const DAC33_INT_OSC_FREQ_RAT_READ_B: c_uint = 0x11;
pub const DAC33_SER_AUDIOIF_CTRL_A: c_uint = 0x12;
pub const DAC33_SER_AUDIOIF_CTRL_B: c_uint = 0x13;
pub const DAC33_SER_AUDIOIF_CTRL_C: c_uint = 0x14;
pub const DAC33_FIFO_CTRL_A: c_uint = 0x15;
pub const DAC33_UTHR_MSB: c_uint = 0x16;
pub const DAC33_UTHR_LSB: c_uint = 0x17;
pub const DAC33_ATHR_MSB: c_uint = 0x18;
pub const DAC33_ATHR_LSB: c_uint = 0x19;
pub const DAC33_LTHR_MSB: c_uint = 0x1A;
pub const DAC33_LTHR_LSB: c_uint = 0x1B;
pub const DAC33_PREFILL_MSB: c_uint = 0x1C;
pub const DAC33_PREFILL_LSB: c_uint = 0x1D;
pub const DAC33_NSAMPLE_MSB: c_uint = 0x1E;
pub const DAC33_NSAMPLE_LSB: c_uint = 0x1F;
pub const DAC33_FIFO_WPTR_MSB: c_uint = 0x20;
pub const DAC33_FIFO_WPTR_LSB: c_uint = 0x21;
pub const DAC33_FIFO_RPTR_MSB: c_uint = 0x22;
pub const DAC33_FIFO_RPTR_LSB: c_uint = 0x23;
pub const DAC33_FIFO_DEPTH_MSB: c_uint = 0x24;
pub const DAC33_FIFO_DEPTH_LSB: c_uint = 0x25;
pub const DAC33_SAMPLES_REMAINING_MSB: c_uint = 0x26;
pub const DAC33_SAMPLES_REMAINING_LSB: c_uint = 0x27;
pub const DAC33_FIFO_IRQ_FLAG: c_uint = 0x28;
pub const DAC33_FIFO_IRQ_MASK: c_uint = 0x29;
pub const DAC33_FIFO_IRQ_MODE_A: c_uint = 0x2A;
pub const DAC33_FIFO_IRQ_MODE_B: c_uint = 0x2B;
pub const DAC33_DAC_CTRL_A: c_uint = 0x2C;
pub const DAC33_DAC_CTRL_B: c_uint = 0x2D;
pub const DAC33_DAC_CTRL_C: c_uint = 0x2E;
pub const DAC33_LDAC_DIG_VOL_CTRL: c_uint = 0x2F;
pub const DAC33_RDAC_DIG_VOL_CTRL: c_uint = 0x30;
pub const DAC33_DAC_STATUS_FLAGS: c_uint = 0x31;
pub const DAC33_ASRC_CTRL_A: c_uint = 0x32;
pub const DAC33_ASRC_CTRL_B: c_uint = 0x33;
pub const DAC33_SRC_REF_CLK_RATIO_A: c_uint = 0x34;
pub const DAC33_SRC_REF_CLK_RATIO_B: c_uint = 0x35;
pub const DAC33_SRC_EST_REF_CLK_RATIO_A: c_uint = 0x36;
pub const DAC33_SRC_EST_REF_CLK_RATIO_B: c_uint = 0x37;
pub const DAC33_INTP_CTRL_A: c_uint = 0x38;
pub const DAC33_INTP_CTRL_B: c_uint = 0x39;
// Registers 0x3A - 0x3F Reserved
pub const DAC33_LDAC_PWR_CTRL: c_uint = 0x40;
pub const DAC33_RDAC_PWR_CTRL: c_uint = 0x41;
pub const DAC33_OUT_AMP_CM_CTRL: c_uint = 0x42;
pub const DAC33_OUT_AMP_PWR_CTRL: c_uint = 0x43;
pub const DAC33_OUT_AMP_CTRL: c_uint = 0x44;
pub const DAC33_LINEL_TO_LLO_VOL: c_uint = 0x45;
// Registers 0x45 - 0x47 Reserved
pub const DAC33_LINER_TO_RLO_VOL: c_uint = 0x48;
pub const DAC33_ANA_VOL_SOFT_STEP_CTRL: c_uint = 0x49;
pub const DAC33_OSC_TRIM: c_uint = 0x4A;
// Registers 0x4B - 0x7C Reserved
pub const DAC33_DEVICE_ID_MSB: c_uint = 0x7D;
pub const DAC33_DEVICE_ID_LSB: c_uint = 0x7E;
pub const DAC33_DEVICE_REV_ID: c_uint = 0x7F;
pub const DAC33_CACHEREGNUM: c_int = 128;
// Bit definitions
// DAC33_PWR_CTRL (0x01)

// DAC33_INT_OSC_CTRL (0x07)

// DAC33_INT_OSC_CTRL_B (0x0C)

// DAC33_INT_OSC_CTRL_C (0x0D)

// DAC33_INT_OSC_STATUS (0x0E)

// DAC33_SER_AUDIOIF_CTRL_A (0x12)

// DAC33_SER_AUDIOIF_CTRL_B (0x13)

// DAC33_FIFO_CTRL_A (0x15)

//
// UTHR, ATHR, LTHR, PREFILL, NSAMPLE (0x16 - 0x1F)
// 13-bit values
//

// DAC33_FIFO_IRQ_MASK (0x29)

// DAC33_FIFO_IRQ_MODE_A (0x2A)

// DAC33_FIFO_IRQ_MODE_B (0x2B)

// DAC33_DAC_CTRL_A (0x2C)

// DAC33_DAC_CTRL_B (0x2D)

// DAC33_DAC_CTRL_C (0x2E)

// DAC33_ASRC_CTRL_A (0x32)

// DAC33_ASRC_CTRL_B (0x33)

// DAC33_INTP_CTRL_A (0x38)

// DAC33_LDAC_PWR_CTRL (0x40)
// DAC33_RDAC_PWR_CTRL (0x41)

// DAC33_ANA_VOL_SOFT_STEP_CTRL (0x49)

pub const TLV320DAC33_MCLK: c_int = 0;
pub const TLV320DAC33_SLEEPCLK: c_int = 1;
