//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/soundbus/i2sbus/interface.h
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
// i2sbus driver -- interface register definitions
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//
// i2s bus control registers, at least what we know about them

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_interface_regs {
    pub /: *mut *mut __le32 intr_ctl; / 0x00,
    pub /: *mut *mut __le32 serial_format; / 0x10,
    pub /: *mut *mut __le32 codec_msg_out; / 0x20,
    pub /: *mut *mut __le32 codec_msg_in; / 0x30,
    pub /: *mut *mut __le32 frame_count; / 0x40,
    pub /: *mut *mut __le32 frame_match; / 0x50,
    pub /: *mut *mut __le32 data_word_sizes; / 0x60,
    pub /: *mut *mut __le32 peak_level_sel; / 0x70,
    pub /: *mut *mut __le32 peak_level_in0; / 0x80,
    pub /: *mut *mut __le32 peak_level_in1; / 0x90,
// total size: 0x100 bytes
    pub __packed: },
// interrupt register is just a bitfield with
// interrupt enable and pending bits
pub const I2S_REG_INTR_CTL: c_uint = 0x00;

// serial format register is more interesting :)
// It contains:
// - clock source
// - MClk divisor
// - SClk divisor
// - SClk master flag
// - serial format (sony, i2s 64x, i2s 32x, dav, silabs)
// - external sample frequency interrupt (don't understand)
// - external sample frequency
//
pub const I2S_REG_SERIAL_FORMAT: c_uint = 0x10;
// clock source. You get either 18.432, 45.1584 or 49.1520 MHz

// also, let's define the exact clock speeds here, in Hz
pub const I2S_CLOCK_SPEED_18MHz: c_int = 18432000;
pub const I2S_CLOCK_SPEED_45MHz: c_int = 45158400;
pub const I2S_CLOCK_SPEED_49MHz: c_int = 49152000;
// MClk is the clock that drives the codec, usually called its 'system clock'.
// It is derived by taking only every 'divisor' tick of the clock.
//

    pub d: c_int,
    pub 0: *mut *mut case 1: out |= I2S_SF_MCLKDIV_1; return,
    pub 0: *mut *mut case 3: out |= I2S_SF_MCLKDIV_3; return,
    pub 0: *mut *mut case 5: out |= I2S_SF_MCLKDIV_5; return,
    pub 0: *mut *mut case 14: out |= I2S_SF_MCLKDIV_14; return,
    pub -1: if (div%2) return,
    pub div/2-1: d =,
    pub -1: return,
// out |= I2S_SF_MCLKDIV_OTHER(div);
    pub 0: return,
// SClk is the clock that drives the i2s wire bus. Note that it is
// derived from the MClk above by taking only every 'divisor' tick
// of MClk.
//

    pub d: c_int,
    pub 0: *mut *mut case 1: out |= I2S_SF_SCLKDIV_1; return,
    pub 0: *mut *mut case 3: out |= I2S_SF_SCLKDIV_3; return,
    pub -1: if (div%2) return,
    pub div/2-1: d =,
    pub -1: if (d == 8 || d == 9) return,
// out |= I2S_SF_SCLKDIV_OTHER(div);
    pub 0: return,

// serial format is the way the data is put to the i2s wire bus

// unknown

// probably gives external frequency?

// used to send codec messages, but how isn't clear
pub const I2S_REG_CODEC_MSG_OUT: c_uint = 0x20;
// used to receive codec messages, but how isn't clear
pub const I2S_REG_CODEC_MSG_IN: c_uint = 0x30;
// frame count reg isn't clear to me yet, but probably useful
pub const I2S_REG_FRAME_COUNT: c_uint = 0x40;
// program to some value, and get interrupt if frame count reaches it
pub const I2S_REG_FRAME_MATCH: c_uint = 0x50;
// this register describes how the bus transfers data
pub const I2S_REG_DATA_WORD_SIZES: c_uint = 0x60;
// number of interleaved input channels

// word size of input data

// number of interleaved output channels

// word size of output data

// unknown
pub const I2S_REG_PEAK_LEVEL_SEL: c_uint = 0x70;
// unknown
pub const I2S_REG_PEAK_LEVEL_IN0: c_uint = 0x80;
// unknown
pub const I2S_REG_PEAK_LEVEL_IN1: c_uint = 0x90;
