//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/asoundef.h
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
// Advanced Linux Sound Architecture - ALSA - Driver
// Copyright (c) 1994-2000 by Jaroslav Kysela <perex@perex.cz>
//
// Digital audio interface
//
// IEC958 subframe format

// AES/IEC958 channel status bits

pub const IEC958_AES1_CON_CATEGORY: c_uint = 0x7f;
pub const IEC958_AES1_CON_GENERAL: c_uint = 0x00;
pub const IEC958_AES1_CON_LASEROPT_MASK: c_uint = 0x07;
pub const IEC958_AES1_CON_LASEROPT_ID: c_uint = 0x01;

pub const IEC958_AES1_CON_DIGDIGCONV_MASK: c_uint = 0x07;
pub const IEC958_AES1_CON_DIGDIGCONV_ID: c_uint = 0x02;

pub const IEC958_AES1_CON_MAGNETIC_MASK: c_uint = 0x07;
pub const IEC958_AES1_CON_MAGNETIC_ID: c_uint = 0x03;

pub const IEC958_AES1_CON_BROADCAST1_MASK: c_uint = 0x07;
pub const IEC958_AES1_CON_BROADCAST1_ID: c_uint = 0x04;

pub const IEC958_AES1_CON_BROADCAST2_MASK: c_uint = 0x0f;
pub const IEC958_AES1_CON_BROADCAST2_ID: c_uint = 0x0e;
pub const IEC958_AES1_CON_MUSICAL_MASK: c_uint = 0x07;
pub const IEC958_AES1_CON_MUSICAL_ID: c_uint = 0x05;

pub const IEC958_AES1_CON_ADC_MASK: c_uint = 0x1f;
pub const IEC958_AES1_CON_ADC_ID: c_uint = 0x06;

pub const IEC958_AES1_CON_ADC_COPYRIGHT_MASK: c_uint = 0x1f;
pub const IEC958_AES1_CON_ADC_COPYRIGHT_ID: c_uint = 0x16;

pub const IEC958_AES1_CON_SOLIDMEM_MASK: c_uint = 0x0f;
pub const IEC958_AES1_CON_SOLIDMEM_ID: c_uint = 0x08;

pub const IEC958_AES1_CON_EXPERIMENTAL: c_uint = 0x40;

//
// CEA-861 Audio InfoFrame. Used in HDMI and DisplayPort
//

//
// MIDI v1.0 interface
//
pub const MIDI_CHANNELS: c_int = 16;

//
// MIDI commands
//
pub const MIDI_CMD_NOTE_OFF: c_uint = 0x80;
pub const MIDI_CMD_NOTE_ON: c_uint = 0x90;
pub const MIDI_CMD_NOTE_PRESSURE: c_uint = 0xa0;
pub const MIDI_CMD_CONTROL: c_uint = 0xb0;
pub const MIDI_CMD_PGM_CHANGE: c_uint = 0xc0;
pub const MIDI_CMD_CHANNEL_PRESSURE: c_uint = 0xd0;
pub const MIDI_CMD_BENDER: c_uint = 0xe0;
pub const MIDI_CMD_COMMON_SYSEX: c_uint = 0xf0;
pub const MIDI_CMD_COMMON_MTC_QUARTER: c_uint = 0xf1;
pub const MIDI_CMD_COMMON_SONG_POS: c_uint = 0xf2;
pub const MIDI_CMD_COMMON_SONG_SELECT: c_uint = 0xf3;
pub const MIDI_CMD_COMMON_TUNE_REQUEST: c_uint = 0xf6;
pub const MIDI_CMD_COMMON_SYSEX_END: c_uint = 0xf7;
pub const MIDI_CMD_COMMON_CLOCK: c_uint = 0xf8;
pub const MIDI_CMD_COMMON_START: c_uint = 0xfa;
pub const MIDI_CMD_COMMON_CONTINUE: c_uint = 0xfb;
pub const MIDI_CMD_COMMON_STOP: c_uint = 0xfc;
pub const MIDI_CMD_COMMON_SENSING: c_uint = 0xfe;
pub const MIDI_CMD_COMMON_RESET: c_uint = 0xff;
//
// MIDI controllers
//
pub const MIDI_CTL_MSB_BANK: c_uint = 0x00;
pub const MIDI_CTL_MSB_MODWHEEL: c_uint = 0x01;
pub const MIDI_CTL_MSB_BREATH: c_uint = 0x02;
pub const MIDI_CTL_MSB_FOOT: c_uint = 0x04;
pub const MIDI_CTL_MSB_PORTAMENTO_TIME: c_uint = 0x05;
pub const MIDI_CTL_MSB_DATA_ENTRY: c_uint = 0x06;
pub const MIDI_CTL_MSB_MAIN_VOLUME: c_uint = 0x07;
pub const MIDI_CTL_MSB_BALANCE: c_uint = 0x08;
pub const MIDI_CTL_MSB_PAN: c_uint = 0x0a;
pub const MIDI_CTL_MSB_EXPRESSION: c_uint = 0x0b;
pub const MIDI_CTL_MSB_EFFECT1: c_uint = 0x0c;
pub const MIDI_CTL_MSB_EFFECT2: c_uint = 0x0d;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE1: c_uint = 0x10;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE2: c_uint = 0x11;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE3: c_uint = 0x12;
pub const MIDI_CTL_MSB_GENERAL_PURPOSE4: c_uint = 0x13;
pub const MIDI_CTL_LSB_BANK: c_uint = 0x20;
pub const MIDI_CTL_LSB_MODWHEEL: c_uint = 0x21;
pub const MIDI_CTL_LSB_BREATH: c_uint = 0x22;
pub const MIDI_CTL_LSB_FOOT: c_uint = 0x24;
pub const MIDI_CTL_LSB_PORTAMENTO_TIME: c_uint = 0x25;
pub const MIDI_CTL_LSB_DATA_ENTRY: c_uint = 0x26;
pub const MIDI_CTL_LSB_MAIN_VOLUME: c_uint = 0x27;
pub const MIDI_CTL_LSB_BALANCE: c_uint = 0x28;
pub const MIDI_CTL_LSB_PAN: c_uint = 0x2a;
pub const MIDI_CTL_LSB_EXPRESSION: c_uint = 0x2b;
pub const MIDI_CTL_LSB_EFFECT1: c_uint = 0x2c;
pub const MIDI_CTL_LSB_EFFECT2: c_uint = 0x2d;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE1: c_uint = 0x30;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE2: c_uint = 0x31;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE3: c_uint = 0x32;
pub const MIDI_CTL_LSB_GENERAL_PURPOSE4: c_uint = 0x33;
pub const MIDI_CTL_SUSTAIN: c_uint = 0x40;
pub const MIDI_CTL_PORTAMENTO: c_uint = 0x41;
pub const MIDI_CTL_SOSTENUTO: c_uint = 0x42;
pub const MIDI_CTL_SOFT_PEDAL: c_uint = 0x43;
pub const MIDI_CTL_LEGATO_FOOTSWITCH: c_uint = 0x44;
pub const MIDI_CTL_HOLD2: c_uint = 0x45;
pub const MIDI_CTL_SC1_SOUND_VARIATION: c_uint = 0x46;
pub const MIDI_CTL_SC2_TIMBRE: c_uint = 0x47;
pub const MIDI_CTL_SC3_RELEASE_TIME: c_uint = 0x48;
pub const MIDI_CTL_SC4_ATTACK_TIME: c_uint = 0x49;
pub const MIDI_CTL_SC5_BRIGHTNESS: c_uint = 0x4a;
pub const MIDI_CTL_SC6: c_uint = 0x4b;
pub const MIDI_CTL_SC7: c_uint = 0x4c;
pub const MIDI_CTL_SC8: c_uint = 0x4d;
pub const MIDI_CTL_SC9: c_uint = 0x4e;
pub const MIDI_CTL_SC10: c_uint = 0x4f;
pub const MIDI_CTL_GENERAL_PURPOSE5: c_uint = 0x50;
pub const MIDI_CTL_GENERAL_PURPOSE6: c_uint = 0x51;
pub const MIDI_CTL_GENERAL_PURPOSE7: c_uint = 0x52;
pub const MIDI_CTL_GENERAL_PURPOSE8: c_uint = 0x53;
pub const MIDI_CTL_PORTAMENTO_CONTROL: c_uint = 0x54;
pub const MIDI_CTL_E1_REVERB_DEPTH: c_uint = 0x5b;
pub const MIDI_CTL_E2_TREMOLO_DEPTH: c_uint = 0x5c;
pub const MIDI_CTL_E3_CHORUS_DEPTH: c_uint = 0x5d;
pub const MIDI_CTL_E4_DETUNE_DEPTH: c_uint = 0x5e;
pub const MIDI_CTL_E5_PHASER_DEPTH: c_uint = 0x5f;
pub const MIDI_CTL_DATA_INCREMENT: c_uint = 0x60;
pub const MIDI_CTL_DATA_DECREMENT: c_uint = 0x61;
pub const MIDI_CTL_NONREG_PARM_NUM_LSB: c_uint = 0x62;
pub const MIDI_CTL_NONREG_PARM_NUM_MSB: c_uint = 0x63;
pub const MIDI_CTL_REGIST_PARM_NUM_LSB: c_uint = 0x64;
pub const MIDI_CTL_REGIST_PARM_NUM_MSB: c_uint = 0x65;
pub const MIDI_CTL_ALL_SOUNDS_OFF: c_uint = 0x78;
pub const MIDI_CTL_RESET_CONTROLLERS: c_uint = 0x79;
pub const MIDI_CTL_LOCAL_CONTROL_SWITCH: c_uint = 0x7a;
pub const MIDI_CTL_ALL_NOTES_OFF: c_uint = 0x7b;
pub const MIDI_CTL_OMNI_OFF: c_uint = 0x7c;
pub const MIDI_CTL_OMNI_ON: c_uint = 0x7d;
pub const MIDI_CTL_MONO1: c_uint = 0x7e;
pub const MIDI_CTL_MONO2: c_uint = 0x7f;
