//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stb0899_priv.h
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

pub const FE_ERROR: c_int = 0;
pub const FE_NOTICE: c_int = 1;
pub const FE_INFO: c_int = 2;
pub const FE_DEBUG: c_int = 3;
pub const FE_DEBUGREG: c_int = 4;

pub const BYTE0: c_int = 0;
pub const BYTE1: c_int = 8;
pub const BYTE2: c_int = 16;
pub const BYTE3: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_status {
    NOAGC1	= 0,
    AGC1OK,
    NOTIMING,
    ANALOGCARRIER,
    TIMINGOK,
    NOAGC2,
    AGC2OK,
    NOCARRIER,
    CARRIEROK,
    NODATA,
    FALSELOCK,
    DATAOK,
    OUTOFRANGE,
    RANGEOK,
    DVBS2_DEMOD_LOCK,
    DVBS2_DEMOD_NOLOCK,
    DVBS2_FEC_LOCK,
    DVBS2_FEC_NOLOCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_modcod {
    STB0899_DUMMY_PLF,
    STB0899_QPSK_14,
    STB0899_QPSK_13,
    STB0899_QPSK_25,
    STB0899_QPSK_12,
    STB0899_QPSK_35,
    STB0899_QPSK_23,
    STB0899_QPSK_34,
    STB0899_QPSK_45,
    STB0899_QPSK_56,
    STB0899_QPSK_89,
    STB0899_QPSK_910,
    STB0899_8PSK_35,
    STB0899_8PSK_23,
    STB0899_8PSK_34,
    STB0899_8PSK_56,
    STB0899_8PSK_89,
    STB0899_8PSK_910,
    STB0899_16APSK_23,
    STB0899_16APSK_34,
    STB0899_16APSK_45,
    STB0899_16APSK_56,
    STB0899_16APSK_89,
    STB0899_16APSK_910,
    STB0899_32APSK_34,
    STB0899_32APSK_45,
    STB0899_32APSK_56,
    STB0899_32APSK_89,
    STB0899_32APSK_910
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_frame {
    STB0899_LONG_FRAME,
    STB0899_SHORT_FRAME
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_alpha {
    RRC_20,
    RRC_25,
    RRC_35
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_tab {
    pub real: i32,
    pub read: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stb0899_fec {
    STB0899_FEC_1_2			= 13,
    STB0899_FEC_2_3			= 18,
    STB0899_FEC_3_4			= 21,
    STB0899_FEC_5_6			= 24,
    STB0899_FEC_6_7			= 25,
    STB0899_FEC_7_8			= 26
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_params {
    pub /: *mut *mut u32 freq; / Frequency,
    pub /: *mut *mut u32 srate; / Symbol rate,
    pub fecrate: fe_code_rate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_internal {
    pub master_clk: u32,
    pub /: *mut *mut u32 freq; / Demod internal Frequency,
    pub /: *mut *mut u32 srate; / Demod internal Symbol rate,
    pub /: *mut *mut stb0899_fec fecrate; / Demod internal FEC rate,
    pub /: *mut *mut s32 srch_range; / Demod internal Search Range,
    pub /: *mut *mut s32 sub_range; / Demod current sub range (Hz),
    pub /: *mut *mut s32 tuner_step; / Tuner step (Hz),
    pub /: *mut *mut s32 tuner_offst; / Relative offset to carrier (Hz),
    pub /: *mut *mut u32 tuner_bw; / Current bandwidth of the tuner (Hz),
    pub /: *mut *mut s32 mclk; / Masterclock Divider factor (binary),
    pub /: *mut *mut s32 rolloff; / Current RollOff of the filter (x100),
    pub /: *mut *mut s16 derot_freq; / Current derotator frequency (Hz),
    pub derot_percent: i16,
    pub /: *mut *mut s16 direction; / Current derotator search direction,
    pub /: *mut *mut s16 derot_step; / Derotator step (binary value),
    pub /: *mut *mut s16 t_derot; / Derotator time constant (ms),
    pub /: *mut *mut s16 t_data; / Data recovery time constant (ms),
    pub /: *mut *mut s16 sub_dir; / Direction of the next sub range,
    pub /: *mut *mut s16 t_agc1; / Agc1 time constant (ms),
    pub /: *mut *mut s16 t_agc2; / Agc2 time constant (ms),
    pub /: *mut *mut u32 lock; / Demod internal lock state,
    pub /: *mut *mut stb0899_status status; / Demod internal status,
// DVB-S2
    pub /: *mut *mut s32 agc_gain; / RF AGC Gain,
    pub /: *mut *mut s32 center_freq; / Nominal carrier frequency,
    pub /: *mut *mut s32 av_frame_coarse; / Coarse carrier freq search frames,
    pub /: *mut *mut s32 av_frame_fine; / Fine carrier freq search frames,
    pub /: *mut *mut s16 step_size; / Carrier frequency search step size,
    pub rrc_alpha: stb0899_alpha,
    pub inversion: stb0899_inversion,
    pub modcod: stb0899_modcod,
    pub /: *mut *mut u8 pilots; / Pilots found,
    pub frame_length: stb0899_frame,
    pub /: *mut *mut u8 v_status; / VSTATUS,
    pub /: *mut *mut u8 err_ctrl; / ERRCTRLn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stb0899_state {
    pub i2c: *mut i2c_adapter,
    pub config: *mut stb0899_config,
    pub frontend: dvb_frontend,
    pub /: *mut *mut *mut u32 verbose; / Cached module verbosity level,
    pub /: *mut *mut stb0899_internal internal; / Device internal parameters,
// cached params from API
    pub delsys: fe_delivery_system,
    pub params: stb0899_params,
    pub /: *mut *mut u32 rx_freq; / DiSEqC 2.0 receiver freq,
    pub search_lock: mutex,
}

// stb0899.c
extern "C" {
    pub fn stb0899_i2c_gate_ctrl(fe: *mut dvb_frontend, enable: c_int) -> c_int;
}

// #define STB0899_WRITE_S2REG(DEVICE, REG, DATA)	(_stb0899_write_s2reg(state, DEVICE, STB0899_BASE_##REG, STB0899_OFF0_##REG, DATA))
// stb0899_algo.c
extern "C" {
    pub fn stb0899_dvbs_algo(state: *mut stb0899_state) -> stb0899_status;
}
extern "C" {
    pub fn stb0899_dvbs2_algo(state: *mut stb0899_state) -> stb0899_status;
}
extern "C" {
    pub fn stb0899_carr_width(state: *mut stb0899_state) -> c_long;
}
