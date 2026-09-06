//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/dice/dice.h
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
// dice.h - a part of driver for Dice based devices
//
// Copyright (c) Clemens Ladisch
// Copyright (c) 2014 Takashi Sakamoto
//

// Macro flag: #define SOUND_DICE_H_INCLUDED

//
// This module support maximum 2 pairs of tx/rx isochronous streams for
// our convinience.
//
// In documents for ASICs called with a name of 'DICE':
// - ASIC for DICE II:
// - Maximum 2 tx and 4 rx are supported.
// - A packet supports maximum 16 data channels.
// - TCD2210/2210-E (so-called 'Dice Mini'):
// - Maximum 2 tx and 2 rx are supported.
// - A packet supports maximum 16 data channels.
// - TCD2220/2220-E (so-called 'Dice Jr.')
// - 2 tx and 2 rx are supported.
// - A packet supports maximum 16 data channels.
// - TCD3070-CH (so-called 'Dice III')
// - Maximum 2 tx and 2 rx are supported.
// - A packet supports maximum 32 data channels.
//
// For the above, MIDI conformant data channel is just on the first isochronous
// stream.
//
pub const MAX_STREAMS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dice_rate_mode {
    SND_DICE_RATE_MODE_LOW = 0,
    SND_DICE_RATE_MODE_MIDDLE,
    SND_DICE_RATE_MODE_HIGH,
    SND_DICE_RATE_MODE_COUNT,
}

extern "C" {
    pub fn int(dice: *mut *mut snd_dice_detect_formats_t)(struct snd_dice) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dice {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub lock: spinlock_t,
    pub mutex: mutex,
// Offsets for sub-addresses
    pub global_offset: c_uint,
    pub rx_offset: c_uint,
    pub tx_offset: c_uint,
    pub sync_offset: c_uint,
    pub rsrv_offset: c_uint,
    pub clock_caps: c_uint,
    pub tx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
    pub rx_pcm_chs: [c_uint; MAX_STREAMS][SND_DICE_RATE_MODE_COUNT],
    pub tx_midi_ports: [c_uint; MAX_STREAMS],
    pub rx_midi_ports: [c_uint; MAX_STREAMS],
    pub notification_handler: fw_address_handler,
    pub owner_generation: c_int,
    pub notification_bits: u32,
// For uapi
    pub /: *mut *mut int dev_lock_count; / > 0 driver, < 0 userspace,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
// For streaming
    pub tx_resources: [fw_iso_resources; MAX_STREAMS],
    pub rx_resources: [fw_iso_resources; MAX_STREAMS],
    pub tx_stream: [amdtp_stream; MAX_STREAMS],
    pub rx_stream: [amdtp_stream; MAX_STREAMS],
    pub global_enabled:1: bool,
    pub disable_double_pcm_frames:1: bool,
    pub clock_accepted: completion,
    pub substreams_counter: c_uint,
    pub domain: amdtp_domain,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dice_addr_type {
    SND_DICE_ADDR_TYPE_PRIVATE,
    SND_DICE_ADDR_TYPE_GLOBAL,
    SND_DICE_ADDR_TYPE_TX,
    SND_DICE_ADDR_TYPE_RX,
    SND_DICE_ADDR_TYPE_SYNC,
    SND_DICE_ADDR_TYPE_RSRV,
}

extern "C" {
    pub fn snd_dice_transaction_get_rate(dice: *mut snd_dice, rate: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn snd_dice_transaction_set_enable(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_transaction_clear_enable(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_transaction_init(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_transaction_reinit(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_transaction_destroy(dice: *mut snd_dice);
}
pub const SND_DICE_RATES_COUNT: c_int = 7;
extern "C" {
    pub fn snd_dice_stream_start_duplex(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_stream_stop_duplex(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_stream_init_duplex(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_stream_destroy_duplex(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_stream_update_duplex(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_stream_detect_current_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_stream_lock_try(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_stream_lock_release(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_create_pcm(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_create_hwdep(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_create_proc(dice: *mut snd_dice);
}
extern "C" {
    pub fn snd_dice_create_midi(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_tcelectronic_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_alesis_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_alesis_mastercontrol_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_extension_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_mytek_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_presonus_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_harman_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_focusrite_pro40_tcd3070_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_weiss_formats(dice: *mut snd_dice) -> c_int;
}
extern "C" {
    pub fn snd_dice_detect_teac_formats(dice: *mut snd_dice) -> c_int;
}
