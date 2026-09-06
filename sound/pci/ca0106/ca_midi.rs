//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ca0106/ca_midi.h
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
// Copyright 10/16/2005 Tilman Kranz <tilde@tk-sls.de>
// Creative Audio MIDI, for the CA0106 Driver
// Version: 0.0.1
//
// Changelog:
// See ca_midi.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ca_midi {
    pub rmidi: *mut snd_rawmidi,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub dev_id: *mut c_void,
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub channel: c_uint,
    pub midi_mode: c_uint,
    pub port: c_int,
    pub rx_enable: int tx_enable,,
    pub ipr_rx: int ipr_tx,,
    pub output_ready: int input_avail,,
    pub enter_uart: int ack, reset,,
    pub status): *mut *mut *mut void (interrupt)(struct snd_ca_midi midi, unsigned int,
    pub intr): *mut *mut *mut void (interrupt_enable)(struct snd_ca_midi midi, int,
    pub intr): *mut *mut *mut void (interrupt_disable)(struct snd_ca_midi midi, int,
    pub idx): *mut *mut *mut unsigned char (read)(struct snd_ca_midi midi, int,
    pub idx): *mut *mut *mut void (write)(struct snd_ca_midi midi, int data, int,
// get info from dev_id
    pub dev_id): *mut *mut *mut snd_card (get_dev_id_card)(void,
    pub dev_id): *mut *mut int (get_dev_id_port)(void,
}

extern "C" {
    pub fn ca_midi_init(card: *mut c_void, midi: *mut snd_ca_midi, device: c_int, name: *mut c_char) -> c_int;
}
