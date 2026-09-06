//! Automatically rewritten from C Header to Rust Module
//! Source: sound/drivers/opl3/opl3_voice.h
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
// Copyright (c) 2000 Uros Bizjak <uros@kss-loka.si>
//

// Prototypes for opl3_seq.c
extern "C" {
    pub fn snd_opl3_synth_use_inc(opl3: *mut *mut snd_opl3) -> c_int;
}
extern "C" {
    pub fn snd_opl3_synth_use_dec(opl3: *mut *mut snd_opl3);
}
extern "C" {
    pub fn snd_opl3_synth_setup(opl3: *mut *mut snd_opl3) -> c_int;
}
extern "C" {
    pub fn snd_opl3_synth_cleanup(opl3: *mut *mut snd_opl3);
}
// Prototypes for opl3_midi.c
extern "C" {
    pub fn snd_opl3_note_on(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_note_off(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_key_press(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_terminate_note(p: *mut c_void, note: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_control(p: *mut c_void, type: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_nrpn(p: *mut c_void, chan: *mut snd_midi_channel, chset: *mut snd_midi_channel_set);
}
extern "C" {
    pub fn snd_opl3_sysex(p: *mut c_void, buf: *mut c_uchar, len: c_int, parsed: c_int, chset: *mut snd_midi_channel_set);
}
extern "C" {
    pub fn snd_opl3_calc_volume(reg: *mut c_uchar, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl3_timer_func(t: *mut timer_list);
}
// Prototypes for opl3_drums.c
extern "C" {
    pub fn snd_opl3_load_drums(opl3: *mut snd_opl3);
}
extern "C" {
    pub fn snd_opl3_drum_switch(opl3: *mut snd_opl3, note: c_int, vel: c_int, on_off: c_int, chan: *mut snd_midi_channel);
}
// Prototypes for opl3_oss.c

extern "C" {
    pub fn snd_opl3_init_seq_oss(opl3: *mut snd_opl3, name: *mut c_char);
}
extern "C" {
    pub fn snd_opl3_free_seq_oss(opl3: *mut snd_opl3);
}

