//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0900_priv.h
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
// stv0900_priv.h
//
// Driver for ST STV0900 satellite demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//

pub const STV0900_MAXLOOKUPSIZE: c_int = 500;
pub const STV0900_BLIND_SEARCH_AGC2_TH: c_int = 700;
pub const STV0900_BLIND_SEARCH_AGC2_TH_CUT30: c_int = 1400;
pub const IQPOWER_THRESHOLD: c_int = 30;
// One point of the lookup table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv000_lookpoint {
    pub /: *mut *mut s32 realval;/ real value,
    pub /: *mut *mut s32 regval;/ binary value,
}

// Lookup table definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_table {
    pub /: *mut *mut s32 size;/ Size of the lookup table,
    pub /: *mut *mut stv000_lookpoint table[STV0900_MAXLOOKUPSIZE];/ Lookup table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_error {
    STV0900_NO_ERROR = 0,
    STV0900_INVALID_HANDLE,
    STV0900_BAD_PARAMETER,
    STV0900_I2C_ERROR,
    STV0900_SEARCH_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_clock_type {
    STV0900_USE_REGISTERS_DEFAULT,
    STV0900_SERIAL_PUNCT_CLOCK,/*Serial punctured clock */
    STV0900_SERIAL_CONT_CLOCK,/*Serial continues clock */
    STV0900_PARALLEL_PUNCT_CLOCK,/*Parallel punctured clock */
    STV0900_DVBCI_CLOCK/*Parallel continues clock : DVBCI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_search_state {
    STV0900_SEARCH = 0,
    STV0900_PLH_DETECTED,
    STV0900_DVBS2_FOUND,
    STV0900_DVBS_FOUND

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_ldpc_state {
    STV0900_PATH1_OFF_PATH2_OFF = 0,
    STV0900_PATH1_ON_PATH2_OFF = 1,
    STV0900_PATH1_OFF_PATH2_ON = 2,
    STV0900_PATH1_ON_PATH2_ON = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_signal_type {
    STV0900_NOAGC1 = 0,
    STV0900_AGC1OK,
    STV0900_NOTIMING,
    STV0900_ANALOGCARRIER,
    STV0900_TIMINGOK,
    STV0900_NOAGC2,
    STV0900_AGC2OK,
    STV0900_NOCARRIER,
    STV0900_CARRIEROK,
    STV0900_NODATA,
    STV0900_DATAOK,
    STV0900_OUTOFRANGE,
    STV0900_RANGEOK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_demod_num {
    STV0900_DEMOD_1,
    STV0900_DEMOD_2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_tracking_standard {
    STV0900_DVBS1_STANDARD,/* Found Standard*/
    STV0900_DVBS2_STANDARD,
    STV0900_DSS_STANDARD,
    STV0900_TURBOCODE_STANDARD,
    STV0900_UNKNOWN_STANDARD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_search_standard {
    STV0900_AUTO_SEARCH,
    STV0900_SEARCH_DVBS1,/* Search Standard*/
    STV0900_SEARCH_DVBS2,
    STV0900_SEARCH_DSS,
    STV0900_SEARCH_TURBOCODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_search_algo {
    STV0900_BLIND_SEARCH,/* offset freq and SR are Unknown */
    STV0900_COLD_START,/* only the SR is known */
    STV0900_WARM_START/* offset freq and SR are known */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_modulation {
    STV0900_QPSK,
    STV0900_8PSK,
    STV0900_16APSK,
    STV0900_32APSK,
    STV0900_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_modcode {
    STV0900_DUMMY_PLF,
    STV0900_QPSK_14,
    STV0900_QPSK_13,
    STV0900_QPSK_25,
    STV0900_QPSK_12,
    STV0900_QPSK_35,
    STV0900_QPSK_23,
    STV0900_QPSK_34,
    STV0900_QPSK_45,
    STV0900_QPSK_56,
    STV0900_QPSK_89,
    STV0900_QPSK_910,
    STV0900_8PSK_35,
    STV0900_8PSK_23,
    STV0900_8PSK_34,
    STV0900_8PSK_56,
    STV0900_8PSK_89,
    STV0900_8PSK_910,
    STV0900_16APSK_23,
    STV0900_16APSK_34,
    STV0900_16APSK_45,
    STV0900_16APSK_56,
    STV0900_16APSK_89,
    STV0900_16APSK_910,
    STV0900_32APSK_34,
    STV0900_32APSK_45,
    STV0900_32APSK_56,
    STV0900_32APSK_89,
    STV0900_32APSK_910,
    STV0900_MODCODE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_fec {
    STV0900_FEC_1_2 = 0,
    STV0900_FEC_2_3,
    STV0900_FEC_3_4,
    STV0900_FEC_4_5,/*for turbo code only*/
    STV0900_FEC_5_6,
    STV0900_FEC_6_7,/*for DSS only */
    STV0900_FEC_7_8,
    STV0900_FEC_8_9,/*for turbo code only*/
    STV0900_FEC_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_frame_length {
    STV0900_LONG_FRAME,
    STV0900_SHORT_FRAME
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_pilot {
    STV0900_PILOTS_OFF,
    STV0900_PILOTS_ON
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_rolloff {
    STV0900_35,
    STV0900_25,
    STV0900_20
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_search_iq {
    STV0900_IQ_AUTO,
    STV0900_IQ_AUTO_NORMAL_FIRST,
    STV0900_IQ_FORCE_NORMAL,
    STV0900_IQ_FORCE_SWAPPED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0900_iq_inversion {
    STV0900_IQ_NORMAL,
    STV0900_IQ_SWAPPED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_diseqc_mode {
    STV0900_22KHZ_Continues = 0,
    STV0900_DISEQC_2_3_PWM = 2,
    STV0900_DISEQC_3_3_PWM = 3,
    STV0900_DISEQC_2_3_ENVELOP = 4,
    STV0900_DISEQC_3_3_ENVELOP = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0900_demod_mode {
    STV0900_SINGLE = 0,
    STV0900_DUAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_init_params {
    pub /: *mut *mut u32 dmd_ref_clk;/ Reference,Input clock for the demod in Hz,
// Demodulator Type (single demod or dual demod)
    pub demod_mode: fe_stv0900_demod_mode,
    pub rolloff: fe_stv0900_rolloff,
    pub path1_ts_clock: fe_stv0900_clock_type,
    pub tun1_maddress: u8,
    pub tuner1_adc: c_int,
    pub tuner1_type: c_int,
// IQ from the tuner1 to the demod
    pub tun1_iq_inv: stv0900_iq_inversion,
    pub path2_ts_clock: fe_stv0900_clock_type,
    pub tun2_maddress: u8,
    pub tuner2_adc: c_int,
    pub tuner2_type: c_int,
// IQ from the tuner2 to the demod
    pub tun2_iq_inv: stv0900_iq_inversion,
    pub ts_config: *mut stv0900_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_search_params {
    pub /: *mut *mut fe_stv0900_demod_num path;/ Path Used demod1 or 2,
    pub /: *mut *mut u32 frequency;/ Transponder frequency (in KHz),
    pub bds)*/: *mut *mut u32 symbol_rate;/ Transponder symbol rate (in,
    pub /: *mut *mut u32 search_range;/ Range of the search (in Hz),
    pub standard: fe_stv0900_search_standard,
    pub modulation: fe_stv0900_modulation,
    pub fec: fe_stv0900_fec,
    pub modcode: fe_stv0900_modcode,
    pub iq_inversion: fe_stv0900_search_iq,
    pub search_algo: fe_stv0900_search_algo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_signal_info {
    pub /: *mut *mut int locked;/ Transponder locked,
    pub /: *mut *mut u32 frequency;/ Transponder frequency (in KHz),
    pub /: *mut *mut u32 symbol_rate;/ Transponder symbol rate (in Mbds),
    pub standard: fe_stv0900_tracking_standard,
    pub fec: fe_stv0900_fec,
    pub modcode: fe_stv0900_modcode,
    pub modulation: fe_stv0900_modulation,
    pub pilot: fe_stv0900_pilot,
    pub frame_len: fe_stv0900_frame_length,
    pub spectrum: stv0900_iq_inversion,
    pub rolloff: fe_stv0900_rolloff,
    pub /: *mut *mut s32 Power;/ Power of the RF signal (dBm),
    pub x10)*/: *mut *mut s32 C_N;/ Carrier to noise ratio (dB,
    pub /: *mut *mut u32 BER;/ Bit error rate (x10^7),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_internal {
    pub quartz: i32,
    pub mclk: i32,
// manual RollOff for DVBS1/DSS only
    pub rolloff: fe_stv0900_rolloff,
// Demodulator use for single demod or for dual demod)
    pub demod_mode: fe_stv0900_demod_mode,
// Demods
    pub freq: [i32; 2],
    pub bw: [i32; 2],
    pub symbol_rate: [i32; 2],
    pub srch_range: [i32; 2],
// for software/auto tuner
    pub tuner_type: [c_int; 2],
// algorithm for search Blind, Cold or Warm
    pub srch_algo: [fe_stv0900_search_algo; 2],
// search standard: Auto, DVBS1/DSS only or DVBS2 only
    pub srch_standard: [fe_stv0900_search_standard; 2],
// inversion search : auto, auto norma first, normal or inverted
    pub srch_iq_inv: [fe_stv0900_search_iq; 2],
    pub modcode: [fe_stv0900_modcode; 2],
    pub modulation: [fe_stv0900_modulation; 2],
    pub fec: [fe_stv0900_fec; 2],
    pub result: [stv0900_signal_info; 2],
    pub err: [fe_stv0900_error; 2],
    pub i2c_adap: *mut i2c_adapter,
    pub i2c_addr: u8,
    pub /: *mut *mut u8 clkmode;/ 0 for CLKI, 2 for XTALI,
    pub chip_id: u8,
    pub ts_config: *mut stv0900_reg,
    pub errs: fe_stv0900_error,
    pub dmds_used: c_int,
}

// state for each demod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_state {
// pointer for internal params, one for each pair of demods
    pub internal: *mut stv0900_internal,
    pub i2c_adap: *mut i2c_adapter,
    pub config: *const stv0900_config,
    pub frontend: dvb_frontend,
    pub demod: c_int,
}

extern "C" {
    pub fn ge2comp(a: i32, width: i32) -> i32;
}
extern "C" {
    pub fn stv0900_algo(fe: *mut dvb_frontend) -> fe_stv0900_signal_type;
}
extern "C" {
    pub fn stv0900_set_bandwidth(fe: *mut dvb_frontend, bandwidth: u32);
}
