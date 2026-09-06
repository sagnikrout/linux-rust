//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/emux_legacy.h
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
// Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
//
// Definitions of OSS compatible headers for Emu8000 device informations
//

//
// awe hardware controls
//
pub const _EMUX_OSS_DEBUG_MODE: c_uint = 0x00;
pub const _EMUX_OSS_REVERB_MODE: c_uint = 0x01;
pub const _EMUX_OSS_CHORUS_MODE: c_uint = 0x02;
pub const _EMUX_OSS_REMOVE_LAST_SAMPLES: c_uint = 0x03;
pub const _EMUX_OSS_INITIALIZE_CHIP: c_uint = 0x04;
pub const _EMUX_OSS_SEND_EFFECT: c_uint = 0x05;
pub const _EMUX_OSS_TERMINATE_CHANNEL: c_uint = 0x06;
pub const _EMUX_OSS_TERMINATE_ALL: c_uint = 0x07;
pub const _EMUX_OSS_INITIAL_VOLUME: c_uint = 0x08;

pub const _EMUX_OSS_RESET_CHANNEL: c_uint = 0x09;
pub const _EMUX_OSS_CHANNEL_MODE: c_uint = 0x0a;
pub const _EMUX_OSS_DRUM_CHANNELS: c_uint = 0x0b;
pub const _EMUX_OSS_MISC_MODE: c_uint = 0x0c;
pub const _EMUX_OSS_RELEASE_ALL: c_uint = 0x0d;
pub const _EMUX_OSS_NOTEOFF_ALL: c_uint = 0x0e;
pub const _EMUX_OSS_CHN_PRESSURE: c_uint = 0x0f;
pub const _EMUX_OSS_EQUALIZER: c_uint = 0x11;
pub const _EMUX_OSS_MODE_FLAG: c_uint = 0x80;
pub const _EMUX_OSS_COOKED_FLAG: c_uint = 0x40	/* not supported */;
pub const _EMUX_OSS_MODE_VALUE_MASK: c_uint = 0x3F;
//
// mode type definitions
//
// 0*/	EMUX_MD_EXCLUSIVE_OFF,	/* obsolete
// 1*/	EMUX_MD_EXCLUSIVE_ON,	/* obsolete
// 2*/	EMUX_MD_VERSION,		/* read only
// 3*/	EMUX_MD_EXCLUSIVE_SOUND,	/* 0/1: exclusive note on (default=1)
// 4*/	EMUX_MD_REALTIME_PAN,	/* 0/1: do realtime pan change (default=1)
// 5*/	EMUX_MD_GUS_BANK,	/* bank number for GUS patches (default=0)
// 6*/	EMUX_MD_KEEP_EFFECT,	/* 0/1: keep effect values, (default=0)
// 7*/	EMUX_MD_ZERO_ATTEN,	/* attenuation of max volume (default=32)
// 8*/	EMUX_MD_CHN_PRIOR,	/* 0/1: set MIDI channel priority mode (default=1)
// 9*/	EMUX_MD_MOD_SENSE,	/* integer: modwheel sensitivity (def=18)
// 10*/	EMUX_MD_DEF_PRESET,	/* integer: default preset number (def=0)
// 11*/	EMUX_MD_DEF_BANK,	/* integer: default bank number (def=0)
// 12*/	EMUX_MD_DEF_DRUM,	/* integer: default drumset number (def=0)
// 13*/	EMUX_MD_TOGGLE_DRUM_BANK, /* 0/1: toggle drum flag with bank# (def=0)
// 14*/	EMUX_MD_NEW_VOLUME_CALC,	/* 0/1: volume calculation mode (def=1)
// 15*/	EMUX_MD_CHORUS_MODE,	/* integer: chorus mode (def=2)
// 16*/	EMUX_MD_REVERB_MODE,	/* integer: chorus mode (def=4)
// 17*/	EMUX_MD_BASS_LEVEL,	/* integer: bass level (def=5)
// 18*/	EMUX_MD_TREBLE_LEVEL,	/* integer: treble level (def=9)
// 19*/	EMUX_MD_DEBUG_MODE,	/* integer: debug level (def=0)
// 20*/	EMUX_MD_PAN_EXCHANGE,	/* 0/1: exchange panning direction (def=0)
//
// effect parameters
//
// modulation envelope parameters
// 0*/	EMUX_FX_ENV1_DELAY,	/* WORD: ENVVAL
// 1*/	EMUX_FX_ENV1_ATTACK,	/* BYTE: up ATKHLD
// 2*/	EMUX_FX_ENV1_HOLD,	/* BYTE: lw ATKHLD
// 3*/	EMUX_FX_ENV1_DECAY,	/* BYTE: lw DCYSUS
// 4*/	EMUX_FX_ENV1_RELEASE,	/* BYTE: lw DCYSUS
// 5*/	EMUX_FX_ENV1_SUSTAIN,	/* BYTE: up DCYSUS
// 6*/	EMUX_FX_ENV1_PITCH,	/* BYTE: up PEFE
// 7*/	EMUX_FX_ENV1_CUTOFF,	/* BYTE: lw PEFE
// volume envelope parameters
// 8*/	EMUX_FX_ENV2_DELAY,	/* WORD: ENVVOL
// 9*/	EMUX_FX_ENV2_ATTACK,	/* BYTE: up ATKHLDV
// 10*/	EMUX_FX_ENV2_HOLD,	/* BYTE: lw ATKHLDV
// 11*/	EMUX_FX_ENV2_DECAY,	/* BYTE: lw DCYSUSV
// 12*/	EMUX_FX_ENV2_RELEASE,	/* BYTE: lw DCYSUSV
// 13*/	EMUX_FX_ENV2_SUSTAIN,	/* BYTE: up DCYSUSV
// LFO1 (tremolo & vibrato) parameters
// 14*/	EMUX_FX_LFO1_DELAY,	/* WORD: LFO1VAL
// 15*/	EMUX_FX_LFO1_FREQ,	/* BYTE: lo TREMFRQ
// 16*/	EMUX_FX_LFO1_VOLUME,	/* BYTE: up TREMFRQ
// 17*/	EMUX_FX_LFO1_PITCH,	/* BYTE: up FMMOD
// 18*/	EMUX_FX_LFO1_CUTOFF,	/* BYTE: lo FMMOD
// LFO2 (vibrato) parameters
// 19*/	EMUX_FX_LFO2_DELAY,	/* WORD: LFO2VAL
// 20*/	EMUX_FX_LFO2_FREQ,	/* BYTE: lo FM2FRQ2
// 21*/	EMUX_FX_LFO2_PITCH,	/* BYTE: up FM2FRQ2
// Other overall effect parameters
// 22*/	EMUX_FX_INIT_PITCH,	/* SHORT: pitch offset
// 23*/	EMUX_FX_CHORUS,		/* BYTE: chorus effects send (0-255)
// 24*/	EMUX_FX_REVERB,		/* BYTE: reverb effects send (0-255)
// 25*/	EMUX_FX_CUTOFF,		/* BYTE: up IFATN
// 26*/	EMUX_FX_FILTERQ,		/* BYTE: up CCCA
// Sample / loop offset changes
// 27*/	EMUX_FX_SAMPLE_START,	/* SHORT: offset
// 28*/	EMUX_FX_LOOP_START,	/* SHORT: offset
// 29*/	EMUX_FX_LOOP_END,	/* SHORT: offset
// 30*/	EMUX_FX_COARSE_SAMPLE_START,	/* SHORT: upper word offset
// 31*/	EMUX_FX_COARSE_LOOP_START,	/* SHORT: upper word offset
// 32*/	EMUX_FX_COARSE_LOOP_END,		/* SHORT: upper word offset
// 33*/	EMUX_FX_ATTEN,		/* BYTE: lo IFATN
// number of effects

// effect flag values
pub const EMUX_FX_FLAG_OFF: c_int = 0;
pub const EMUX_FX_FLAG_SET: c_int = 1;
pub const EMUX_FX_FLAG_ADD: c_int = 2;
