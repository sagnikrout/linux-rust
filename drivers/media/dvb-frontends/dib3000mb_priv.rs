//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib3000mb_priv.h
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
// dib3000mb_priv.h
//
// Copyright (C) 2004 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// for more information see dib3000mb.c .
//
// handy shortcuts

// debug

// mask for enabling a specific pid for the pid_filter

// common values for tuning

// frontend state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib3000_state {
    pub i2c: *mut *mut i2c_adapter,
// configuration settings
    pub config: dib3000_config,
    pub frontend: dvb_frontend,
    pub timing_offset: c_int,
    pub timing_offset_comp_done: c_int,
    pub last_tuned_bw: u32,
    pub last_tuned_freq: u32,
}

// register addresses and some of their default values
// restart subsystems

// FFT size

// Guard time

// QAM

// Alpha coefficient high priority Viterbi algorithm

// spectrum inversion

// DDS frequency value (IF position) ad ? values don't match reg_3000mb.txt

// timing frequency (carrier spacing)
// impulse noise parameter
// 36 ???
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dib3000mb_impulse_noise_type {
    DIB3000MB_IMPNOISE_OFF,
    DIB3000MB_IMPNOISE_MOBILE,
    DIB3000MB_IMPNOISE_FIXED,
    DIB3000MB_IMPNOISE_DEFAULT
}

//
// Dual Automatic-Gain-Control
// - gains RF in tuner (AGC1)
// - gains IF after filtering (AGC2)
//
// also from 16 to 18
// phase noise
// 36 is set when setting the impulse noise
// lock duration
// AGC loop bandwidth
//
// lock0 definition (coff_lock)
//

//
// lock1 definition (cpil_lock)
// for auto search
// which values hide behind the lock masks
//

//
// lock2 definition (fec_lock)

//
// SEQ ? what was that again ... :)
// changes when, inversion, guard time and fft is
// either automatically detected or not
//

// bandwidth

// isi

// sync impovement

// phase noise compensation inhibition

// mobile mode ???

// fft

// QAM for mobile mode

//
// data diversity when having more than one chip on-board
// see also DIB3000MB_OUTPUT_MODE_DATA_DIVERSITY
//

// vit hrch

// vit code rate

// vit select hp

// time frame for Bit-Error-Rate calculation

// 142 - 152 FIFO parameters
// which is what ?
//

// MPEG2 TS output mode

//
// pidfilter
// it is not a hardware pidfilter but a filter which drops all pids
// except the ones set. Necessary because of the limited USB1.1 bandwidth.
// regs 153-168
//

//
// output mode
// USB devices have to use 'slave'-mode
// see also DIB3000MB_REG_ELECT_OUT_MODE
//

// irq event mask

// filter coefficients
//
// mobile algorithm (when you are moving with your device)
// but not faster than 90 km/h
//

// multiple demodulators algorithm

// terminator, no more demods

// bring the device into a known

// hardware clock configuration

// power down config

// electrical output mode

// set the tuner i2c address

// monitoring registers (read only)
// agc loop locked (size: 1)

// agc power (size: 16)

// agc1 value (16)

// agc2 value (16)

// total RF power (16), can be used for signal strength

// dds_frequency with offset (24)

// timing offset signed (24)

// fft start position (13)

// carriers locked (1)

// noise power (24)

//
// signal power (16), this and the above can be
// used to calculate the signal/noise - ratio
//

// mer (24)

//
// Transmission Parameter Signalling (TPS)
// the following registers can be used to get TPS-information.
// The values are according to the DVB-T standard.
//
// TPS locked (1)

// QAM from TPS (2) (values according to DIB3000MB_REG_QAM)

// hierarchy from TPS (1)

// alpha from TPS (3) (values according to DIB3000MB_REG_VIT_ALPHA)

// code rate high priority from TPS (3) (values according to DIB3000MB_FEC_*)

// code rate low priority from TPS (3) if DIB3000MB_REG_TPS_VIT_ALPHA

// guard time from TPS (2) (values according to DIB3000MB_REG_GUARD_TIME

// fft size from TPS (2) (values according to DIB3000MB_REG_FFT)

// cell id from TPS (16)

// TPS (68)

// bit error rate (before RS correction) (21)

// packet error rate (uncorrected TS packets) (16)

// uncorrected packet count (16)

// viterbi locked (1)

// viterbi inidcator (16)

// transport stream sync lock (1)

// transport stream RS lock (1)

// lock mask 0 value (1)

// lock mask 1 value (1)

// lock mask 2 value (1)

// interrupt pending for auto search

