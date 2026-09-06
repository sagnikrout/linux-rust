//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv090x_priv.h
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

// Macro flag: #define __STV090x_PRIV_H

pub const FE_ERROR: c_int = 0;
pub const FE_NOTICE: c_int = 1;
pub const FE_INFO: c_int = 2;
pub const FE_DEBUG: c_int = 3;
pub const FE_DEBUGREG: c_int = 4;

pub const STV090x_IQPOWER_THRESHOLD: c_int = 30;
pub const STV090x_SEARCH_AGC2_TH_CUT20: c_int = 700;
pub const STV090x_SEARCH_AGC2_TH_CUT30: c_int = 1400;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_signal_state {
    STV090x_NOAGC1,
    STV090x_NOCARRIER,
    STV090x_NODATA,
    STV090x_DATAOK,
    STV090x_RANGEOK,
    STV090x_OUTOFRANGE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_fec {
    STV090x_PR12 = 0,
    STV090x_PR23,
    STV090x_PR34,
    STV090x_PR45,
    STV090x_PR56,
    STV090x_PR67,
    STV090x_PR78,
    STV090x_PR89,
    STV090x_PR910,
    STV090x_PRERR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_modulation {
    STV090x_QPSK,
    STV090x_8PSK,
    STV090x_16APSK,
    STV090x_32APSK,
    STV090x_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_frame {
    STV090x_LONG_FRAME,
    STV090x_SHORT_FRAME
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_pilot {
    STV090x_PILOTS_OFF,
    STV090x_PILOTS_ON
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_rolloff {
    STV090x_RO_35,
    STV090x_RO_25,
    STV090x_RO_20
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_inversion {
    STV090x_IQ_AUTO,
    STV090x_IQ_NORMAL,
    STV090x_IQ_SWAP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_modcod {
    STV090x_DUMMY_PLF = 0,
    STV090x_QPSK_14,
    STV090x_QPSK_13,
    STV090x_QPSK_25,
    STV090x_QPSK_12,
    STV090x_QPSK_35,
    STV090x_QPSK_23,
    STV090x_QPSK_34,
    STV090x_QPSK_45,
    STV090x_QPSK_56,
    STV090x_QPSK_89,
    STV090x_QPSK_910,
    STV090x_8PSK_35,
    STV090x_8PSK_23,
    STV090x_8PSK_34,
    STV090x_8PSK_56,
    STV090x_8PSK_89,
    STV090x_8PSK_910,
    STV090x_16APSK_23,
    STV090x_16APSK_34,
    STV090x_16APSK_45,
    STV090x_16APSK_56,
    STV090x_16APSK_89,
    STV090x_16APSK_910,
    STV090x_32APSK_34,
    STV090x_32APSK_45,
    STV090x_32APSK_56,
    STV090x_32APSK_89,
    STV090x_32APSK_910,
    STV090x_MODCODE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_search {
    STV090x_SEARCH_DSS = 0,
    STV090x_SEARCH_DVBS1,
    STV090x_SEARCH_DVBS2,
    STV090x_SEARCH_AUTO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_algo {
    STV090x_BLIND_SEARCH,
    STV090x_COLD_SEARCH,
    STV090x_WARM_SEARCH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv090x_delsys {
    STV090x_ERROR = 0,
    STV090x_DVBS1 = 1,
    STV090x_DVBS2,
    STV090x_DSS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_long_frame_crloop {
    pub modcod: stv090x_modcod,
    pub crl_pilots_on_2: u8,
    pub crl_pilots_off_2: u8,
    pub crl_pilots_on_5: u8,
    pub crl_pilots_off_5: u8,
    pub crl_pilots_on_10: u8,
    pub crl_pilots_off_10: u8,
    pub crl_pilots_on_20: u8,
    pub crl_pilots_off_20: u8,
    pub crl_pilots_on_30: u8,
    pub crl_pilots_off_30: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_short_frame_crloop {
    pub modulation: stv090x_modulation,
    pub /: *mut *mut u8 crl_2; / SR < 3M,
    pub /: *mut *mut u8 crl_5; / 3 < SR <= 7M,
    pub /: *mut *mut u8 crl_10; / 7 < SR <= 15M,
    pub /: *mut *mut u8 crl_20; / 10 < SR <= 25M,
    pub /: *mut *mut u8 crl_30; / 10 < SR <= 45M,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_reg {
    pub addr: u16,
    pub data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_tab {
    pub real: i32,
    pub read: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_internal {
    pub i2c_adap: *mut i2c_adapter,
    pub i2c_addr: u8,
    pub /: *mut *mut mutex demod_lock; / Lock access to shared register,
    pub /: *mut *mut mutex tuner_lock; / Lock access to tuners,
    pub /: *mut *mut s32 mclk; / Masterclock Divider factor,
    pub dev_ver: u32,
    pub num_used: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv090x_state {
    pub device: stv090x_device,
    pub demod: stv090x_demodulator,
    pub demod_mode: stv090x_mode,
    pub internal: *mut stv090x_internal,
    pub i2c: *mut i2c_adapter,
    pub config: *mut stv090x_config,
    pub frontend: dvb_frontend,
    pub /: *mut *mut *mut u32 verbose; / Cached module verbosity,
    pub delsys: stv090x_delsys,
    pub fec: stv090x_fec,
    pub modulation: stv090x_modulation,
    pub modcod: stv090x_modcod,
    pub search_mode: stv090x_search,
    pub frame_len: stv090x_frame,
    pub pilots: stv090x_pilot,
    pub rolloff: stv090x_rolloff,
    pub inversion: stv090x_inversion,
    pub algo: stv090x_algo,
    pub frequency: u32,
    pub srate: u32,
    pub tuner_bw: i32,
    pub search_range: i32,
    pub DemodTimeout: i32,
    pub FecTimeout: i32,
}
