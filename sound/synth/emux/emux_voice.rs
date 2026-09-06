//! Automatically rewritten from C Header to Rust Module
//! Source: sound/synth/emux/emux_voice.h
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
// A structure to keep track of each hardware voice
//
// Copyright (C) 1999 Steve Ratcliffe
// Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
//

// Prototypes for emux_seq.c
extern "C" {
    pub fn snd_emux_init_seq(emu: *mut snd_emux, card: *mut snd_card, index: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emux_detach_seq(emu: *mut snd_emux);
}
extern "C" {
    pub fn snd_emux_reset_port(port: *mut snd_emux_port);
}
extern "C" {
    pub fn snd_emux_inc_count(emu: *mut snd_emux) -> c_int;
}
extern "C" {
    pub fn snd_emux_dec_count(emu: *mut snd_emux);
}
extern "C" {
    pub fn snd_emux_init_virmidi(emu: *mut snd_emux, card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_emux_delete_virmidi(emu: *mut snd_emux) -> c_int;
}
// Prototypes for emux_synth.c
extern "C" {
    pub fn snd_emux_init_voices(emu: *mut snd_emux);
}
extern "C" {
    pub fn snd_emux_note_on(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_emux_note_off(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_emux_key_press(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_emux_terminate_note(p: *mut c_void, note: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_emux_control(p: *mut c_void, type: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_emux_sounds_off_all(port: *mut snd_emux_port);
}
extern "C" {
    pub fn snd_emux_update_port(port: *mut snd_emux_port, update: c_int);
}
extern "C" {
    pub fn snd_emux_timer_callback(t: *mut timer_list);
}
// emux_effect.c

extern "C" {
    pub fn snd_emux_create_effect(p: *mut snd_emux_port);
}
extern "C" {
    pub fn snd_emux_delete_effect(p: *mut snd_emux_port);
}
extern "C" {
    pub fn snd_emux_clear_effect(p: *mut snd_emux_port);
}
extern "C" {
    pub fn snd_emux_setup_effect(vp: *mut snd_emux_voice);
}

// emux_nrpn.c
// emux_oss.c
extern "C" {
    pub fn snd_emux_init_seq_oss(emu: *mut snd_emux);
}
extern "C" {
    pub fn snd_emux_detach_seq_oss(emu: *mut snd_emux);
}
// emux_proc.c

extern "C" {
    pub fn snd_emux_proc_init(emu: *mut snd_emux, card: *mut snd_card, device: c_int);
}
extern "C" {
    pub fn snd_emux_proc_free(emu: *mut snd_emux);
}

// emux_hwdep.c
extern "C" {
    pub fn snd_emux_init_hwdep(emu: *mut snd_emux) -> c_int;
}
extern "C" {
    pub fn snd_emux_delete_hwdep(emu: *mut snd_emux);
}
