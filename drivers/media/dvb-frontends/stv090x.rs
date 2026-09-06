//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv090x.h
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

// Macro flag: #define __STV090x_H
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_demodulator {
    STV090x_DEMODULATOR_0 = 1,
    STV090x_DEMODULATOR_1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_device {
    STV0903	=  0,
    STV0900,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_mode {
    STV090x_DUAL = 0,
    STV090x_SINGLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_tsmode {
    STV090x_TSMODE_SERIAL_PUNCTURED	= 1,
    STV090x_TSMODE_SERIAL_CONTINUOUS,
    STV090x_TSMODE_PARALLEL_PUNCTURED,
    STV090x_TSMODE_DVBCI
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_clkmode {
    STV090x_CLK_INT = 0, /* Clk i/p = CLKI */
    STV090x_CLK_EXT = 2 /* Clk i/p = XTALI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_i2crpt {
    STV090x_RPTLEVEL_256	= 0,
    STV090x_RPTLEVEL_128	= 1,
    STV090x_RPTLEVEL_64	= 2,
    STV090x_RPTLEVEL_32	= 3,
    STV090x_RPTLEVEL_16	= 4,
    STV090x_RPTLEVEL_8	= 5,
    STV090x_RPTLEVEL_4	= 6,
    STV090x_RPTLEVEL_2	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_adc_range {
    STV090x_ADC_2Vpp	= 0,
    STV090x_ADC_1Vpp	= 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_config {
    pub device: stv090x_device,
    pub demod_mode: stv090x_mode,
    pub clk_mode: stv090x_clkmode,
    pub demod: stv090x_demodulator,
    pub /: *mut *mut u32 xtal; / default: 8000000,
    pub /: *mut *mut u8 address; / default: 0x68,
    pub ts1_mode: u8,
    pub ts2_mode: u8,
    pub ts1_clk: u32,
    pub ts2_clk: u32,
    pub 1: u8 ts1_tei :,
    pub 1: u8 ts2_tei :,
    pub repeater_level: stv090x_i2crpt,
    pub /: *mut *mut u8 tuner_bbgain; / default: 10db,
    pub /: *mut *mut stv090x_adc_range adc1_range; / default: 2Vpp,
    pub /: *mut *mut stv090x_adc_range adc2_range; / default: 2Vpp,
    pub diseqc_envelope_mode: bool,
    pub fe): *mut *mut int (tuner_init)(struct dvb_frontend,
    pub fe): *mut *mut int (tuner_sleep)(struct dvb_frontend,
    pub mode): *mut *mut *mut int (tuner_set_mode)(struct dvb_frontend fe, enum tuner_mode,
    pub frequency): *mut *mut *mut int (tuner_set_frequency)(struct dvb_frontend fe, u32,
    pub frequency): *mut *mut *mut int (tuner_get_frequency)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_set_bandwidth)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_get_bandwidth)(struct dvb_frontend fe, u32,
    pub gain): *mut *mut *mut int (tuner_set_bbgain)(struct dvb_frontend fe, u32,
    pub gain): *mut *mut *mut int (tuner_get_bbgain)(struct dvb_frontend fe, u32,
    pub refclk): *mut *mut *mut int (tuner_set_refclk)(struct dvb_frontend fe, u32,
    pub status): *mut *mut *mut int (tuner_get_status)(struct dvb_frontend fe, u32,
    pub lock): *mut *mut *mut void (tuner_i2c_lock)(struct dvb_frontend fe, int,
// dir = 0 -> output, dir = 1 -> input/open-drain
    pub xor_value): u8,
    pub i2c): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
}

