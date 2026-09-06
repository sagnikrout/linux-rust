//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_midi_emul.h
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
// Midi channel definition for optional channel management.
//
// Copyright (C) 1999 Steve Ratcliffe
//

//
// This structure is used to keep track of the current state on each
// channel.  All drivers for hardware that does not understand midi
// directly will probably need to use this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_midi_channel {
    pub /: *mut *mut *mut void private; / A back pointer to driver data,
    pub /: *mut *mut int number; / The channel number,
    pub /: *mut *mut int client; / The client associated with this channel,
    pub /: *mut *mut int port; / The port associated with this channel,
    pub /: *mut *mut unsigned char midi_mode; / GM, GS, XG etc,
    pub /: *mut *mut unsigned char midi_aftertouch; / Aftertouch (key pressure),
    pub /: *mut *mut unsigned char midi_pressure; / Channel pressure,
    pub /: *mut *mut unsigned char midi_program; / Instrument number,
    pub /: *mut *mut short midi_pitchbend; / Pitch bend amount,
    pub /: *mut *mut unsigned char control[128]; / Current value of all controls,
    pub /: *mut *mut unsigned char note[128]; / Current status for all notes,
    pub /: *mut *mut short gm_rpn_pitch_bend_range; / Pitch bend range,
    pub /: *mut *mut short gm_rpn_fine_tuning; / Master fine tuning,
    pub /: *mut *mut short gm_rpn_coarse_tuning; / Master coarse tuning,
}

//
// A structure that represets a set of channels bound to a port.  There
// would usually be 16 channels per port.  But fewer could be used for
// particular cases.
// The channel set consists of information describing the client and
// port for this midi synth and an array of snd_midi_channel structures.
// A driver that had no need for snd_midi_channel could still use the
// channel set type if it wished with the channel array null.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_midi_channel_set {
    pub /: *mut *mut *mut void private_data; / Driver data,
    pub /: *mut *mut int client; / Client for this port,
    pub /: *mut *mut int port; / The port number,
    pub /: *mut *mut unsigned char midi_mode; / MIDI operating mode,
    pub /: *mut *mut unsigned char gs_master_volume; / SYSEX master volume: 0-127,
    pub gs_chorus_mode: c_uchar,
    pub gs_reverb_mode: c_uchar,
    pub /: *mut *mut int max_channels; / Size of the channels array,
    pub __counted_by(max_channels): snd_midi_channel channels[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_midi_op {
    pub chan): *mut *mut *mut void (note_on)(void private_data, int note, int vel, struct snd_midi_channel,
    pub /: *mut *mut *mut *mut *mut void (note_off)(void private_data,int note, int vel, struct snd_midi_channel chan); / release note,
    pub chan): *mut *mut *mut void (key_press)(void private_data, int note, int vel, struct snd_midi_channel,
    pub /: *mut *mut *mut *mut *mut void (note_terminate)(void private_data, int note, struct snd_midi_channel chan); / terminate note immediately,
    pub chan): *mut *mut *mut void (control)(void private_data, int type, struct snd_midi_channel,
    pub chset): *mut snd_midi_channel_set,
    pub chset): *mut snd_midi_channel_set,
}

//
// These defines are used so that pitchbend, aftertouch etc, can be
// distinguished from controller values.
//
// 0-127 controller values
pub const MIDI_CTL_PITCHBEND: c_uint = 0x80;
pub const MIDI_CTL_AFTERTOUCH: c_uint = 0x81;
pub const MIDI_CTL_CHAN_PRESSURE: c_uint = 0x82;
//
// These names exist to allow symbolic access to the controls array.
// The usage is eg: chan->gm_bank_select.  Another implementation would
// be really have these members in the struct, and not the array.
//

//
// These macros give the complete value of the controls that consist
// of coarse and fine pairs.  Of course the fine controls are seldom used
// but there is no harm in being complete.
//

// MIDI mode

pub const SNDRV_MIDI_MODE_GM: c_int = 1;
pub const SNDRV_MIDI_MODE_GS: c_int = 2;
pub const SNDRV_MIDI_MODE_XG: c_int = 3;
pub const SNDRV_MIDI_MODE_MT32: c_int = 4;
// MIDI note state
pub const SNDRV_MIDI_NOTE_OFF: c_uint = 0x00;
pub const SNDRV_MIDI_NOTE_ON: c_uint = 0x01;
pub const SNDRV_MIDI_NOTE_RELEASED: c_uint = 0x02;
pub const SNDRV_MIDI_NOTE_SOSTENUTO: c_uint = 0x04;
pub const SNDRV_MIDI_PARAM_TYPE_REGISTERED: c_int = 0;
pub const SNDRV_MIDI_PARAM_TYPE_NONREGISTERED: c_int = 1;
// SYSEX parse flag
// Prototypes for midi_process.c
extern "C" {
    pub fn snd_midi_channel_set_clear(chset: *mut snd_midi_channel_set);
}
extern "C" {
    pub fn snd_midi_channel_free_set(chset: *mut snd_midi_channel_set);
}
