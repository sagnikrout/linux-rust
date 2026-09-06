//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/soundbus/soundbus.h
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
// soundbus generic definitions
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

// When switching from master to slave or the other way around,
// you don't want to have the codec chip acting as clock source
// while the bus still is.
// More importantly, while switch from slave to master, you need
// to turn off the chip's master function first, but then there's
// no clock for a while and other chips might reset, so we notify
// their drivers after having switched.
// The constants here are codec-point of view, so when we switch
// the soundbus to master we tell the codec we're going to switch
// and give it CLOCK_SWITCH_PREPARE_SLAVE!
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_switch {
    CLOCK_SWITCH_PREPARE_SLAVE,
    CLOCK_SWITCH_PREPARE_MASTER,
    CLOCK_SWITCH_SLAVE,
    CLOCK_SWITCH_MASTER,
    CLOCK_SWITCH_NOTIFY,
}

// information on a transfer the codec can take
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transfer_info {
    pub /: *mut *mut *mut u64 formats; / SNDRV_PCM_FMTBIT_,
    pub /: *mut *mut *mut unsigned int rates; / SNDRV_PCM_RATE_,
// flags
// for codecs to distinguish among their TIs
    pub tag: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codec_info_item {
    pub codec: *mut codec_info,
    pub codec_data: *mut c_void,
    pub sdev: *mut soundbus_dev,
// internal, to be used by the soundbus provider
    pub list: list_head,
}

// for prepare, where the codecs need to know
// what we're going to drive the bus with
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_info {
// see below
    pub sysclock_factor: c_int,
    pub bus_factor: c_int,
}

// information on the codec itself, plus function pointers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codec_info {
// the module this lives in
    pub owner: *mut module,
// supported transfer possibilities, array terminated by
// formats or rates being 0.
    pub transfers: *mut transfer_info,
// Master clock speed factor
// to be used (master clock speed = sysclock_factor * sampling freq)
// Unused if the soundbus provider has no such notion.
//
    pub sysclock_factor: c_int,
// Bus factor, bus clock speed = bus_factor * sampling freq)
// Unused if the soundbus provider has no such notion.
//
    pub bus_factor: c_int,
// operations
// clock switching, see above
    pub clock): clock_switch,
// called for each transfer_info when the user
// opens the pcm device to determine what the
// hardware can support at this point in time.
// That can depend on other user-switchable controls.
// Return 1 if usable, 0 if not.
// out points to another instance of a transfer_info
// which is initialised to the values in *ti, and
// it's format and rate values can be modified by
// the callback if it is necessary to further restrict
// the formats that can be used at the moment, for
// example when one codec has multiple logical codec
// info structs for multiple inputs.
//
    pub out): *mut transfer_info,
// called when pcm stream is opened, probably not implemented
// most of the time since it isn't too useful
    pub substream): *mut snd_pcm_substream,
// called when the pcm stream is closed, at this point
// the user choices can all be unlocked (see below)
    pub substream): *mut snd_pcm_substream,
// if the codec must forbid some user choices because
// they are not valid with the substream/transfer info,
// it must do so here. Example: no digital output for
// incompatible framerate, say 8KHz, on Onyx.
// If the selected stuff in the substream is NOT
// compatible, you have to reject this call!
    pub substream): *mut snd_pcm_substream,
// start() is called before data is pushed to the codec.
// Note that start() must be atomic!
    pub substream): *mut snd_pcm_substream,
// stop() is called after data is no longer pushed to the codec.
// Note that stop() must be atomic!
    pub substream): *mut snd_pcm_substream,
    pub state): *mut *mut *mut int (suspend)(struct codec_info_item cii, pm_message_t,
    pub cii): *mut *mut int (resume)(struct codec_info_item,
}

// information on a soundbus device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundbus_dev {
// the bus it belongs to
    pub onbuslist: list_head,
// the of device it represents
    pub ofdev: platform_device,
// what modules go by
    pub modalias: [c_char; 32],
// These fields must be before attach_codec can be called.
// They should be set by the owner of the alsa card object
// that is needed, and whoever sets them must make sure
// that they are unique within that alsa card object.
    pub pcmname: *mut c_char,
    pub pcmid: c_int,
// this is assigned by the soundbus provider in attach_codec
    pub pcm: *mut snd_pcm,
// operations
// attach a codec to this soundbus, give the alsa
// card object the PCMs for this soundbus should be in.
// The 'data' pointer must be unique, it is used as the
// key for detach_codec().
    pub data): *mut *mut codec_info ci, void,
    pub data): *mut *mut *mut void (detach_codec)(struct soundbus_dev dev, void,
// TODO: suspend/resume
// private for the soundbus provider
    pub codec_list: list_head,
    pub have_in:1: u32 have_out:1,,
}

extern "C" {
    pub fn soundbus_add_one(dev: *mut soundbus_dev) -> c_int;
}
extern "C" {
    pub fn soundbus_remove_one(dev: *mut soundbus_dev);
}
extern "C" {
    pub fn soundbus_dev_put(dev: *mut soundbus_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundbus_driver {
    pub name: *mut c_char,
    pub owner: *mut module,
// we don't implement any matching at all
    pub dev): *mut *mut *mut int (probe)(struct soundbus_dev,
    pub dev): *mut *mut void (remove)(struct soundbus_dev,
    pub dev): *mut *mut *mut int (shutdown)(struct soundbus_dev,
    pub driver: device_driver,
}

extern "C" {
    pub fn soundbus_register_driver(drv: *mut soundbus_driver) -> c_int;
}
extern "C" {
    pub fn soundbus_unregister_driver(drv: *mut soundbus_driver);
}
