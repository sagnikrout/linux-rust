//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/cs4245.h
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
pub const CS4245_CHIP_ID: c_uint = 0x01;
pub const CS4245_POWER_CTRL: c_uint = 0x02;
pub const CS4245_DAC_CTRL_1: c_uint = 0x03;
pub const CS4245_ADC_CTRL: c_uint = 0x04;
pub const CS4245_MCLK_FREQ: c_uint = 0x05;
pub const CS4245_SIGNAL_SEL: c_uint = 0x06;
pub const CS4245_PGA_B_CTRL: c_uint = 0x07;
pub const CS4245_PGA_A_CTRL: c_uint = 0x08;
pub const CS4245_ANALOG_IN: c_uint = 0x09;
pub const CS4245_DAC_A_CTRL: c_uint = 0x0a;
pub const CS4245_DAC_B_CTRL: c_uint = 0x0b;
pub const CS4245_DAC_CTRL_2: c_uint = 0x0c;
pub const CS4245_INT_STATUS: c_uint = 0x0d;
pub const CS4245_INT_MASK: c_uint = 0x0e;
pub const CS4245_INT_MODE_MSB: c_uint = 0x0f;
pub const CS4245_INT_MODE_LSB: c_uint = 0x10;
// Chip ID
pub const CS4245_CHIP_PART_MASK: c_uint = 0xf0;
pub const CS4245_CHIP_REV_MASK: c_uint = 0x0f;
// Power Control
pub const CS4245_FREEZE: c_uint = 0x80;
pub const CS4245_PDN_MIC: c_uint = 0x08;
pub const CS4245_PDN_ADC: c_uint = 0x04;
pub const CS4245_PDN_DAC: c_uint = 0x02;
pub const CS4245_PDN: c_uint = 0x01;
// DAC Control
pub const CS4245_DAC_FM_MASK: c_uint = 0xc0;
pub const CS4245_DAC_FM_SINGLE: c_uint = 0x00;
pub const CS4245_DAC_FM_DOUBLE: c_uint = 0x40;
pub const CS4245_DAC_FM_QUAD: c_uint = 0x80;
pub const CS4245_DAC_DIF_MASK: c_uint = 0x30;
pub const CS4245_DAC_DIF_LJUST: c_uint = 0x00;
pub const CS4245_DAC_DIF_I2S: c_uint = 0x10;
pub const CS4245_DAC_DIF_RJUST_16: c_uint = 0x20;
pub const CS4245_DAC_DIF_RJUST_24: c_uint = 0x30;
pub const CS4245_RESERVED_1: c_uint = 0x08;
pub const CS4245_MUTE_DAC: c_uint = 0x04;
pub const CS4245_DEEMPH: c_uint = 0x02;
pub const CS4245_DAC_MASTER: c_uint = 0x01;
// ADC Control
pub const CS4245_ADC_FM_MASK: c_uint = 0xc0;
pub const CS4245_ADC_FM_SINGLE: c_uint = 0x00;
pub const CS4245_ADC_FM_DOUBLE: c_uint = 0x40;
pub const CS4245_ADC_FM_QUAD: c_uint = 0x80;
pub const CS4245_ADC_DIF_MASK: c_uint = 0x10;
pub const CS4245_ADC_DIF_LJUST: c_uint = 0x00;
pub const CS4245_ADC_DIF_I2S: c_uint = 0x10;
pub const CS4245_MUTE_ADC: c_uint = 0x04;
pub const CS4245_HPF_FREEZE: c_uint = 0x02;
pub const CS4245_ADC_MASTER: c_uint = 0x01;
// MCLK Frequency
pub const CS4245_MCLK1_MASK: c_uint = 0x70;
pub const CS4245_MCLK1_SHIFT: c_int = 4;
pub const CS4245_MCLK2_MASK: c_uint = 0x07;
pub const CS4245_MCLK2_SHIFT: c_int = 0;
pub const CS4245_MCLK_1: c_int = 0;
pub const CS4245_MCLK_1_5: c_int = 1;
pub const CS4245_MCLK_2: c_int = 2;
pub const CS4245_MCLK_3: c_int = 3;
pub const CS4245_MCLK_4: c_int = 4;
// Signal Selection
pub const CS4245_A_OUT_SEL_MASK: c_uint = 0x60;
pub const CS4245_A_OUT_SEL_HIZ: c_uint = 0x00;
pub const CS4245_A_OUT_SEL_DAC: c_uint = 0x20;
pub const CS4245_A_OUT_SEL_PGA: c_uint = 0x40;
pub const CS4245_LOOP: c_uint = 0x02;
pub const CS4245_ASYNCH: c_uint = 0x01;
// Channel B/A PGA Control
pub const CS4245_PGA_GAIN_MASK: c_uint = 0x3f;
// ADC Input Control
pub const CS4245_PGA_SOFT: c_uint = 0x10;
pub const CS4245_PGA_ZERO: c_uint = 0x08;
pub const CS4245_SEL_MASK: c_uint = 0x07;
pub const CS4245_SEL_MIC: c_uint = 0x00;
pub const CS4245_SEL_INPUT_1: c_uint = 0x01;
pub const CS4245_SEL_INPUT_2: c_uint = 0x02;
pub const CS4245_SEL_INPUT_3: c_uint = 0x03;
pub const CS4245_SEL_INPUT_4: c_uint = 0x04;
pub const CS4245_SEL_INPUT_5: c_uint = 0x05;
pub const CS4245_SEL_INPUT_6: c_uint = 0x06;
// DAC Channel A/B Volume Control
pub const CS4245_VOL_MASK: c_uint = 0xff;
// DAC Control 2
pub const CS4245_DAC_SOFT: c_uint = 0x80;
pub const CS4245_DAC_ZERO: c_uint = 0x40;
pub const CS4245_INVERT_DAC: c_uint = 0x20;
pub const CS4245_INT_ACTIVE_HIGH: c_uint = 0x01;
// Interrupt Status/Mask/Mode
pub const CS4245_ADC_CLK_ERR: c_uint = 0x08;
pub const CS4245_DAC_CLK_ERR: c_uint = 0x04;
pub const CS4245_ADC_OVFL: c_uint = 0x02;
pub const CS4245_ADC_UNDRFL: c_uint = 0x01;

pub const CS4245_SPI_ADDRESS: c_uint = 0x9e;
pub const CS4245_SPI_WRITE: c_int = 0;
pub const CS4245_SPI_READ: c_int = 1;
