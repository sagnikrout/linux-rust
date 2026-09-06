//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctmixer.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctmixer.h
//
// @Brief
// This file contains the definition of the mixer device functions.
//
// @Author	Liu Chun
// @Date 	Mar 28 2008
//

pub const INIT_VOL: c_uint = 0x1c00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MIXER_PORT_T {
    MIX_WAVE_FRONT,
    MIX_WAVE_REAR,
    MIX_WAVE_CENTLFE,
    MIX_WAVE_SURROUND,
    MIX_SPDIF_OUT,
    MIX_PCMO_FRONT,
    MIX_MIC_IN,
    MIX_LINE_IN,
    MIX_SPDIF_IN,
    MIX_PCMI_FRONT,
    MIX_PCMI_REAR,
    MIX_PCMI_CENTLFE,
    MIX_PCMI_SURROUND,

    NUM_MIX_PORTS
}

// alsa mixer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_mixer {
    pub atc: *mut ct_atc,
    pub /: *mut *mut *mut *mut sum sums; / sum resources for signal collection,
    pub /: *mut *mut *mut snd_kcontrol line_mic_kctls[2]; / line/mic capture switch controls,
    pub /: *mut *mut unsigned int switch_state; / A bit-map to indicate state of switches,
    pub rright): *mut *mut *mut rsc rleft, rsc,
    pub rsc): *mut MIXER_PORT_T type, struct rsc,
    pub rsc): *mut MIXER_PORT_T type, struct rsc,

    pub mixer): *mut *mut int (resume)(struct ct_mixer,

    pub /: *mut *mut *mut amixer amixers[]; / amixer resources for volume control,
}

extern "C" {
    pub fn ct_mixer_create(atc: *mut ct_atc, rmixer: *mut ct_mixer) -> c_int;
}
extern "C" {
    pub fn ct_mixer_destroy(mixer: *mut ct_mixer) -> c_int;
}
