//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda1004x.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10046_xtal {
    TDA10046_XTAL_4M,
    TDA10046_XTAL_16M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10046_agc {
    TDA10046_AGC_DEFAULT,		/* original configuration */
    TDA10046_AGC_IFO_AUTO_NEG,	/* IF AGC only, automatic, negative */
    TDA10046_AGC_IFO_AUTO_POS,	/* IF AGC only, automatic, positive */
    TDA10046_AGC_TDA827X,		/* IF AGC only, special setup for tda827x */
}

// Many (hybrid) boards use GPIO 1 and 3
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10046_gpio {
    TDA10046_GPTRI  = 0x00,		/* All GPIOs tristate */
    TDA10046_GP00   = 0x40,		/* GPIO3=0, GPIO1=0 */
    TDA10046_GP01   = 0x42,		/* GPIO3=0, GPIO1=1 */
    TDA10046_GP10   = 0x48,		/* GPIO3=1, GPIO1=0 */
    TDA10046_GP11   = 0x4a,		/* GPIO3=1, GPIO1=1 */
    TDA10046_GP00_I = 0x80,		/* GPIO3=0, GPIO1=0, invert in sleep mode*/
    TDA10046_GP01_I = 0x82,		/* GPIO3=0, GPIO1=1, invert in sleep mode */
    TDA10046_GP10_I = 0x88,		/* GPIO3=1, GPIO1=0, invert in sleep mode */
    TDA10046_GP11_I = 0x8a,		/* GPIO3=1, GPIO1=1, invert in sleep mode */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10046_if {
    TDA10046_FREQ_3617,		/* original config, 36,166 MHZ */
    TDA10046_FREQ_3613,		/* 36,13 MHZ */
    TDA10046_FREQ_045,		/* low IF, 4.0, 4.5, or 5.0 MHZ */
    TDA10046_FREQ_052,		/* low IF, 5.1667 MHZ for tda9889 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda10046_tsout {
    TDA10046_TS_PARALLEL  = 0x00,	/* parallel transport stream, default */
    TDA10046_TS_SERIAL    = 0x01,	/* serial transport stream */
}

// the demodulator's i2c address
// does the "inversion" need inverted?
// Does the OCLK signal need inverted?
// parallel or serial transport stream
// Xtal frequency, 4 or 16MHz
// IF frequency
// AGC configuration
// setting of GPIO1 and 3
// slave address and configuration of the tuner
// if the board uses another I2c Bridge (tda8290), its address
// request firmware for device
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda1004x_demod {
    TDA1004X_DEMOD_TDA10045,
    TDA1004X_DEMOD_TDA10046,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda1004x_state {
    pub i2c: *mut *mut i2c_adapter,
    pub config: *const *const tda1004x_config,
    pub frontend: dvb_frontend,
// private demod data
    pub demod_type: tda1004x_demod,
}

