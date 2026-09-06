//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/spk_priv.h
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


// SPDX-License-Identifier: GPL-2.0+
// spk_priv.h
// review functions for the speakup screen review package.
// originally written by: Kirk Reiser and Andy Berdan.
//
// extensively modified by David Borowski.
//
// Copyright (C) 1998  Kirk Reiser.
// Copyright (C) 2003  David Borowski.
//

pub const SPACE: c_uint = 0x20;

// synth flags, for odd synths

pub const SYNTH_START: c_int = 1;

pub const SYNTH_START: c_int = 0;

pub const KT_SPKUP: c_int = 15;

pub const SYNTH_DEFAULT_SER: c_int = 0;
extern "C" {
    pub fn spk_stop_serial_interrupt();
}
extern "C" {
    pub fn spk_serial_release(synth: *mut spk_synth);
}
extern "C" {
    pub fn spk_ttyio_release(synth: *mut spk_synth);
}
extern "C" {
    pub fn spk_ttyio_register_ldisc();
}
extern "C" {
    pub fn spk_ttyio_unregister_ldisc();
}
extern "C" {
    pub fn synth_buffer_skip_nonlatin1();
}
extern "C" {
    pub fn synth_buffer_getc() -> u16;
}
extern "C" {
    pub fn synth_buffer_peek() -> u16;
}
extern "C" {
    pub fn synth_buffer_empty() -> c_int;
}
extern "C" {
    pub fn spk_serial_synth_probe(synth: *mut spk_synth) -> c_int;
}
extern "C" {
    pub fn spk_ttyio_synth_probe(synth: *mut spk_synth) -> c_int;
}
extern "C" {
    pub fn spk_do_catch_up(synth: *mut spk_synth);
}
extern "C" {
    pub fn spk_do_catch_up_unicode(synth: *mut spk_synth);
}
extern "C" {
    pub fn spk_synth_flush(synth: *mut spk_synth);
}
extern "C" {
    pub fn spk_synth_get_index(synth: *mut spk_synth) -> c_uchar;
}
extern "C" {
    pub fn spk_synth_is_alive_nop(synth: *mut spk_synth) -> c_int;
}
extern "C" {
    pub fn spk_synth_is_alive_restart(synth: *mut spk_synth) -> c_int;
}
extern "C" {
    pub fn synth_printf(buf: *const c_char, ...);
}
extern "C" {
    pub fn synth_putwc(wc: u16);
}
extern "C" {
    pub fn synth_putwc_s(wc: u16);
}
extern "C" {
    pub fn synth_putws(buf: *const u16);
}
extern "C" {
    pub fn synth_putws_s(buf: *const u16);
}
extern "C" {
    pub fn synth_request_region(start: c_ulong, n: c_ulong) -> c_int;
}
extern "C" {
    pub fn synth_release_region(start: c_ulong, n: c_ulong) -> c_int;
}
extern "C" {
    pub fn synth_add(in_synth: *mut spk_synth) -> c_int;
}
extern "C" {
    pub fn synth_remove(in_synth: *mut spk_synth);
}
