//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_timer.h
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
// ALSA sequencer Timer
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_timer_tick {
    pub /: *mut *mut snd_seq_tick_time_t cur_tick; / current tick,
    pub /: *mut *mut unsigned long resolution; / time per tick in nsec,
    pub /: *mut *mut unsigned long fraction; / current time per tick in nsec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_timer {
// ... tempo / offset / running state
    pub /: *mut *mut initialized:1; / timer is initialized,
    pub /: *mut *mut unsigned int tempo; / current tempo, us/tick,
    pub /: *mut *mut int ppq; / time resolution, ticks/quarter,
    pub /: *mut *mut snd_seq_real_time_t cur_time; / current time,
    pub /: *mut *mut snd_seq_timer_tick tick; / current tick,
    pub tick_updated: c_int,
    pub /: *mut *mut int type; / timer type,
    pub /: *mut *mut snd_timer_id alsa_id; / ALSA's timer ID,
    pub /: *mut *mut *mut snd_timer_instance timeri; / timer instance,
    pub ticks: c_uint,
    pub /: *mut *mut unsigned long preferred_resolution; / timer resolution, ticks/sec,
    pub skew: c_uint,
    pub skew_base: c_uint,
    pub tempo_base: c_uint,
    pub /: *mut *mut timespec64 last_update; / time of last clock update, used for interpolation,
    pub lock: spinlock_t,
}

// create new timer (constructor)
// delete timer (destructor)
extern "C" {
    pub fn snd_seq_timer_delete(tmr: *mut snd_seq_timer);
}
//
// compare timestamp between events
// return 1 if a >= b; otherwise return 0
// compare ticks
// compare real time
// roll-over
// increment timestamp
// called by timer isr
extern "C" {
    pub fn snd_seq_timer_open(q: *mut snd_seq_queue) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_close(q: *mut snd_seq_queue) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_defaults(tmr: *mut snd_seq_timer);
}
extern "C" {
    pub fn snd_seq_timer_reset(tmr: *mut snd_seq_timer);
}
extern "C" {
    pub fn snd_seq_timer_stop(tmr: *mut snd_seq_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_start(tmr: *mut snd_seq_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_continue(tmr: *mut snd_seq_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_set_tempo(tmr: *mut snd_seq_timer, tempo: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_set_position_tick(tmr: *mut snd_seq_timer, position: snd_seq_tick_time_t) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_set_position_time(tmr: *mut snd_seq_timer, position: snd_seq_real_time_t) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_set_skew(tmr: *mut snd_seq_timer, skew: c_uint, base: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_seq_timer_get_cur_tick(tmr: *mut snd_seq_timer) -> snd_seq_tick_time_t;
}
