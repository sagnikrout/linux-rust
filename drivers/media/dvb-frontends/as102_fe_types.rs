//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/as102_fe_types.h
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
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2008 Pierrick Hascoet <pierrick.hascoet@abilis.com>
//
// MACRO DEFINITIONS
//
// bandwidth constant values
pub const BW_5_MHZ: c_uint = 0x00;
pub const BW_6_MHZ: c_uint = 0x01;
pub const BW_7_MHZ: c_uint = 0x02;
pub const BW_8_MHZ: c_uint = 0x03;
// hierarchy priority selection values
pub const HIER_NO_PRIORITY: c_uint = 0x00;
pub const HIER_LOW_PRIORITY: c_uint = 0x01;
pub const HIER_HIGH_PRIORITY: c_uint = 0x02;
// constellation available values
pub const CONST_QPSK: c_uint = 0x00;
pub const CONST_QAM16: c_uint = 0x01;
pub const CONST_QAM64: c_uint = 0x02;
pub const CONST_UNKNOWN: c_uint = 0xFF;
// hierarchy available values
pub const HIER_NONE: c_uint = 0x00;
pub const HIER_ALPHA_1: c_uint = 0x01;
pub const HIER_ALPHA_2: c_uint = 0x02;
pub const HIER_ALPHA_4: c_uint = 0x03;
pub const HIER_UNKNOWN: c_uint = 0xFF;
// interleaving available values
pub const INTLV_NATIVE: c_uint = 0x00;
pub const INTLV_IN_DEPTH: c_uint = 0x01;
pub const INTLV_UNKNOWN: c_uint = 0xFF;
// code rate available values
pub const CODE_RATE_1_2: c_uint = 0x00;
pub const CODE_RATE_2_3: c_uint = 0x01;
pub const CODE_RATE_3_4: c_uint = 0x02;
pub const CODE_RATE_5_6: c_uint = 0x03;
pub const CODE_RATE_7_8: c_uint = 0x04;
pub const CODE_RATE_UNKNOWN: c_uint = 0xFF;
// guard interval available values
pub const GUARD_INT_1_32: c_uint = 0x00;
pub const GUARD_INT_1_16: c_uint = 0x01;
pub const GUARD_INT_1_8: c_uint = 0x02;
pub const GUARD_INT_1_4: c_uint = 0x03;
pub const GUARD_UNKNOWN: c_uint = 0xFF;
// transmission mode available values
pub const TRANS_MODE_2K: c_uint = 0x00;
pub const TRANS_MODE_8K: c_uint = 0x01;
pub const TRANS_MODE_4K: c_uint = 0x02;
pub const TRANS_MODE_UNKNOWN: c_uint = 0xFF;
// DVBH signalling available values
pub const TIMESLICING_PRESENT: c_uint = 0x01;
pub const MPE_FEC_PRESENT: c_uint = 0x02;
// tune state available
pub const TUNE_STATUS_NOT_TUNED: c_uint = 0x00;
pub const TUNE_STATUS_IDLE: c_uint = 0x01;
pub const TUNE_STATUS_LOCKING: c_uint = 0x02;
pub const TUNE_STATUS_SIGNAL_DVB_OK: c_uint = 0x03;
pub const TUNE_STATUS_STREAM_DETECTED: c_uint = 0x04;
pub const TUNE_STATUS_STREAM_TUNED: c_uint = 0x05;
pub const TUNE_STATUS_ERROR: c_uint = 0xFF;
// available TS FID filter types
pub const TS_PID_TYPE_TS: c_int = 0;
pub const TS_PID_TYPE_PSI_SI: c_int = 1;
pub const TS_PID_TYPE_MPE: c_int = 2;
// number of echos available
pub const MAX_ECHOS: c_int = 15;
// Context types
pub const CONTEXT_LNA: c_int = 1010;
pub const CONTEXT_ELNA_HYSTERESIS: c_int = 4003;
pub const CONTEXT_ELNA_GAIN: c_int = 4004;
pub const CONTEXT_MER_THRESHOLD: c_int = 5005;
pub const CONTEXT_MER_OFFSET: c_int = 5006;
pub const CONTEXT_IR_STATE: c_int = 7000;
pub const CONTEXT_TSOUT_MSB_FIRST: c_int = 7004;
pub const CONTEXT_TSOUT_FALLING_EDGE: c_int = 7005;
// Configuration modes
pub const CFG_MODE_ON: c_int = 0;
pub const CFG_MODE_OFF: c_int = 1;
pub const CFG_MODE_AUTO: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_tps {
    pub modulation: u8,
    pub hierarchy: u8,
    pub interleaving_mode: u8,
    pub code_rate_HP: u8,
    pub code_rate_LP: u8,
    pub guard_interval: u8,
    pub transmission_mode: u8,
    pub DVBH_mask_HP: u8,
    pub DVBH_mask_LP: u8,
    pub cell_ID: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_tune_args {
// frequency
    pub freq: u32,
// bandwidth
    pub bandwidth: u8,
// hierarchy selection
    pub hier_select: u8,
// constellation
    pub modulation: u8,
// hierarchy
    pub hierarchy: u8,
// interleaving mode
    pub interleaving_mode: u8,
// code rate
    pub code_rate: u8,
// guard interval
    pub guard_interval: u8,
// transmission mode
    pub transmission_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_tune_status {
// tune status
    pub tune_state: u8,
// signal strength
    pub signal_strength: i16,
// packet error rate 10^-4
    pub PER: u16,
// bit error rate 10^-4
    pub BER: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_demod_stats {
// frame counter
    pub frame_count: u32,
// Bad frame counter
    pub bad_frame_count: u32,
// Number of wrong bytes fixed by Reed-Solomon
    pub bytes_fixed_by_rs: u32,
// Averaged MER
    pub mer: u16,
// statistics calculation state indicator (started or not)
    pub has_started: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_ts_filter {
    pub /: *mut *mut uint16_t pid; / valid PID value 0x00 : 0x2000,
    pub /: *mut *mut uint8_t type; / Red TS_PID_TYPE_<N> values,
    pub /: *mut *mut uint8_t idx; / index in filtering table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_register_value {
    pub mode: u8,
    pub /: *mut *mut uint8_t value8; / 8 bit value,
    pub /: *mut *mut uint16_t value16; / 16 bit value,
    pub /: *mut *mut uint32_t value32; / 32 bit value,
    pub u: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_register_addr {
// register addr
    pub addr: u32,
// register mode access
    pub mode: u8,
    pub __packed: },
