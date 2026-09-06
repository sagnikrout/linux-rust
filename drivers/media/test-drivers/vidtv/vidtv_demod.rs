//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_demod.h
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
// The Virtual DTV test driver serves as a reference DVB driver and helps
// validate the existing APIs in the media subsystem. It can also aid
// developers working on userspace applications.
//
// Copyright (C) 2020 Daniel W. S. Almeida
// Based on the example driver written by Emard <emard@softhome.net>
//

//
// struct vidtv_demod_cnr_to_qual_s - Map CNR values to a given combination of
// modulation and fec_inner
// @modulation: see enum fe_modulation
// @fec: see enum fe_fec_rate
// @cnr_ok: S/N threshold to consider the signal as OK. Below that, there's
// a chance of losing sync.
// @cnr_good: S/N threshold to consider the signal strong.
//
// This struct matches values for 'good' and 'ok' CNRs given the combination
// of modulation and fec_inner in use. We might simulate some noise if the
// signal quality is not too good.
//
// The values were taken from libdvbv5.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_demod_cnr_to_qual_s {
    pub modulation: u32,
    pub fec: u32,
    pub cnr_ok: u32,
    pub cnr_good: u32,
}

//
// struct vidtv_demod_config - Configuration used to init the demod
// @drop_tslock_prob_on_low_snr: probability of losing the lock due to low snr
// @recover_tslock_prob_on_good_snr: probability of recovering when the signal
// improves
//
// The configuration used to init the demodulator module, usually filled
// by a bridge driver. For vidtv, this is filled by vidtv_bridge before the
// demodulator module is probed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_demod_config {
    pub drop_tslock_prob_on_low_snr: u8,
    pub recover_tslock_prob_on_good_snr: u8,
}

//
// struct vidtv_demod_state - The demodulator state
// @frontend: The frontend structure allocated by the demod.
// @config: The config used to init the demod.
// @status: the demod status.
// @tuner_cnr: current S/N ratio for the signal carrier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_demod_state {
    pub frontend: dvb_frontend,
    pub config: vidtv_demod_config,
    pub status: fe_status,
    pub tuner_cnr: u16,
}
