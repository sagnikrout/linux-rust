//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stb0899_drv.h
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

pub const STB0899_TSMODE_SERIAL: c_int = 1;
pub const STB0899_CLKPOL_FALLING: c_int = 2;
pub const STB0899_CLKNULL_PARITY: c_int = 3;
pub const STB0899_SYNC_FORCED: c_int = 4;
pub const STB0899_FECMODE_DSS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_s1_reg {
    pub address: u16,
    pub data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_s2_reg {
    pub offset: u16,
    pub base_address: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_inversion {
    IQ_SWAP_OFF	= +1, /* inversion affects the sign of e. g. */
    IQ_SWAP_ON	= -1, /* the derotator frequency register    */
}

pub const STB0899_GPIO00: c_uint = 0xf140;
pub const STB0899_GPIO01: c_uint = 0xf141;
pub const STB0899_GPIO02: c_uint = 0xf142;
pub const STB0899_GPIO03: c_uint = 0xf143;
pub const STB0899_GPIO04: c_uint = 0xf144;
pub const STB0899_GPIO05: c_uint = 0xf145;
pub const STB0899_GPIO06: c_uint = 0xf146;
pub const STB0899_GPIO07: c_uint = 0xf147;
pub const STB0899_GPIO08: c_uint = 0xf148;
pub const STB0899_GPIO09: c_uint = 0xf149;
pub const STB0899_GPIO10: c_uint = 0xf14a;
pub const STB0899_GPIO11: c_uint = 0xf14b;
pub const STB0899_GPIO12: c_uint = 0xf14c;
pub const STB0899_GPIO13: c_uint = 0xf14d;
pub const STB0899_GPIO14: c_uint = 0xf14e;
pub const STB0899_GPIO15: c_uint = 0xf14f;
pub const STB0899_GPIO16: c_uint = 0xf150;
pub const STB0899_GPIO17: c_uint = 0xf151;
pub const STB0899_GPIO18: c_uint = 0xf152;
pub const STB0899_GPIO19: c_uint = 0xf153;
pub const STB0899_GPIO20: c_uint = 0xf154;
pub const STB0899_GPIOPULLUP: c_uint = 0x01 /* Output device is connected to Vdd */;
pub const STB0899_GPIOPULLDN: c_uint = 0x00 /* Output device is connected to Vss */;
pub const STB0899_POSTPROC_GPIO_POWER: c_uint = 0x00;
pub const STB0899_POSTPROC_GPIO_LOCK: c_uint = 0x01;
//
// Post process output configuration control
// 1. POWER ON/OFF		(index 0)
// 2. FE_HAS_LOCK/LOCK_LOSS	(index 1)
//
// @gpio	= one of the above listed GPIO's
// @level	= output state: pulled up or low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_postproc {
    pub gpio: u16,
    pub level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_config {
    pub init_dev: *const stb0899_s1_reg,
    pub init_s2_demod: *const stb0899_s2_reg,
    pub init_s1_demod: *const stb0899_s1_reg,
    pub init_s2_fec: *const stb0899_s2_reg,
    pub init_tst: *const stb0899_s1_reg,
    pub postproc: *const stb0899_postproc,
    pub inversion: stb0899_inversion,
    pub xtal_freq: u32,
    pub demod_address: u8,
    pub ts_output_mode: u8,
    pub block_sync_mode: u8,
    pub ts_pfbit_toggle: u8,
    pub clock_polarity: u8,
    pub data_clk_parity: u8,
    pub fec_mode: u8,
    pub data_output_ctl: u8,
    pub data_fifo_mode: u8,
    pub out_rate_comp: u8,
    pub i2c_repeater: u8,
// int	inversion;
    pub lo_clk: c_int,
    pub hi_clk: c_int,
    pub esno_ave: u32,
    pub esno_quant: u32,
    pub avframes_coarse: u32,
    pub avframes_fine: u32,
    pub miss_threshold: u32,
    pub uwp_threshold_acq: u32,
    pub uwp_threshold_track: u32,
    pub uwp_threshold_sof: u32,
    pub sof_search_timeout: u32,
    pub btr_nco_bits: u32,
    pub btr_gain_shift_offset: u32,
    pub crl_nco_bits: u32,
    pub ldpc_max_iter: u32,
    pub frequency): *mut *mut *mut int (tuner_set_frequency)(struct dvb_frontend fe, u32,
    pub frequency): *mut *mut *mut int (tuner_get_frequency)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_set_bandwidth)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_get_bandwidth)(struct dvb_frontend fe, u32,
    pub rf_gain): *mut *mut *mut int (tuner_set_rfsiggain)(struct dvb_frontend fe, u32,
}

