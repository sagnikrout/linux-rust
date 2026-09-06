//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/midi.h
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


// SPDX-License-Identifier: GPL-2.0
// maximum number of endpoints per interface
pub const MIDI_MAX_ENDPOINTS: c_int = 2;
// data for QUIRK_MIDI_FIXED_ENDPOINT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_midi_endpoint_info {
    pub /: *mut *mut int8_t out_ep; / ep number, 0 autodetect,
    pub /: *mut *mut uint8_t out_interval; / interval for interrupt endpoints,
    pub in_ep: i8,
    pub in_interval: u8,
    pub /: *mut *mut uint16_t out_cables; / bitmask,
    pub /: *mut *mut uint16_t in_cables; / bitmask,
    pub assoc_in_jacks: [i16; 16],
    pub assoc_out_jacks: [i16; 16],
}

// for QUIRK_MIDI_YAMAHA, data is NULL
// for QUIRK_MIDI_MIDIMAN, data points to a snd_usb_midi_endpoint_info
// structure (out_cables and in_cables only)
// for QUIRK_COMPOSITE, data points to an array of snd_usb_audio_quirk
// structures, terminated with .ifnum = -1
// for QUIRK_AUDIO_FIXED_ENDPOINT, data points to an audioformat structure
// for QUIRK_AUDIO/MIDI_STANDARD_INTERFACE, data is NULL
// for QUIRK_AUDIO_EDIROL_UA700_UA25/UA1000, data is NULL
// for QUIRK_IGNORE_INTERFACE, data is NULL
// for QUIRK_MIDI_NOVATION and _RAW, data is NULL
// for QUIRK_MIDI_EMAGIC, data points to a snd_usb_midi_endpoint_info
// structure (out_cables and in_cables only)
// for QUIRK_MIDI_CME, data is NULL
// for QUIRK_MIDI_AKAI, data is NULL
extern "C" {
    pub fn __snd_usbmidi_create(_arg: card, _arg: iface, _arg: midi_list, _arg: quirk, _arg: 0, _arg: NULL) -> return;
}
extern "C" {
    pub fn snd_usbmidi_input_stop(p: *mut list_head);
}
extern "C" {
    pub fn snd_usbmidi_input_start(p: *mut list_head);
}
extern "C" {
    pub fn snd_usbmidi_disconnect(p: *mut list_head);
}
extern "C" {
    pub fn snd_usbmidi_suspend(p: *mut list_head);
}
extern "C" {
    pub fn snd_usbmidi_resume(p: *mut list_head);
}
