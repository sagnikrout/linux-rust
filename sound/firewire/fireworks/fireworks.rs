//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/fireworks/fireworks.h
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
// fireworks.h - a part of driver for Fireworks based devices
//
// Copyright (c) 2009-2010 Clemens Ladisch
// Copyright (c) 2013-2014 Takashi Sakamoto
//

// Macro flag: #define SOUND_FIREWORKS_H_INCLUDED

pub const SND_EFW_MAX_MIDI_OUT_PORTS: c_int = 2;
pub const SND_EFW_MAX_MIDI_IN_PORTS: c_int = 2;
pub const SND_EFW_MULTIPLIER_MODES: c_int = 3;
pub const HWINFO_NAME_SIZE_BYTES: c_int = 32;
pub const HWINFO_MAX_CAPS_GROUPS: c_int = 8;
//
// This should be greater than maximum bytes for EFW response content.
// Currently response against command for isochronous channel mapping is
// confirmed to be the maximum one. But for flexibility, use maximum data
// payload for asynchronous primary packets at S100 (Cable base rate) in
// IEEE Std 1394-1995.
//
pub const SND_EFW_RESPONSE_MAXIMUM_BYTES: c_uint = 0x200U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_efw_phys_grp {
    pub /: *mut *mut u8 type; / see enum snd_efw_grp_type,
    pub count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_efw {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub card_index: c_int,
    pub mutex: mutex,
    pub lock: spinlock_t,
// for transaction
    pub seqnum: u32,
    pub resp_addr_changable: bool,
// for quirks
    pub is_af9: bool,
    pub is_fireworks3: bool,
    pub firmware_version: u32,
    pub midi_in_ports: c_uint,
    pub midi_out_ports: c_uint,
    pub supported_sampling_rate: c_uint,
    pub pcm_capture_channels: [c_uint; SND_EFW_MULTIPLIER_MODES],
    pub pcm_playback_channels: [c_uint; SND_EFW_MULTIPLIER_MODES],
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub substreams_counter: c_uint,
// hardware metering parameters
    pub phys_out: c_uint,
    pub phys_in: c_uint,
    pub phys_out_grp_count: c_uint,
    pub phys_in_grp_count: c_uint,
    pub phys_out_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub phys_in_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
// for uapi
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
// response queue
    pub resp_buf: *mut u8,
    pub pull_ptr: *mut u8,
    pub push_ptr: *mut u8,
    pub domain: amdtp_domain,
}

extern "C" {
    pub fn snd_efw_transaction_register() -> c_int;
}
extern "C" {
    pub fn snd_efw_transaction_unregister();
}
extern "C" {
    pub fn snd_efw_transaction_bus_reset(unit: *mut fw_unit);
}
extern "C" {
    pub fn snd_efw_transaction_add_instance(efw: *mut snd_efw);
}
extern "C" {
    pub fn snd_efw_transaction_remove_instance(efw: *mut snd_efw);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_efw_hwinfo {
    pub flags: u32,
    pub guid_hi: u32,
    pub guid_lo: u32,
    pub type: u32,
    pub version: u32,
    pub vendor_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub model_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub supported_clocks: u32,
    pub amdtp_rx_pcm_channels: u32,
    pub amdtp_tx_pcm_channels: u32,
    pub phys_out: u32,
    pub phys_in: u32,
    pub phys_out_grp_count: u32,
    pub phys_out_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub phys_in_grp_count: u32,
    pub phys_in_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub midi_out_ports: u32,
    pub midi_in_ports: u32,
    pub max_sample_rate: u32,
    pub min_sample_rate: u32,
    pub dsp_version: u32,
    pub arm_version: u32,
    pub mixer_playback_channels: u32,
    pub mixer_capture_channels: u32,
    pub fpga_version: u32,
    pub amdtp_rx_pcm_channels_2x: u32,
    pub amdtp_tx_pcm_channels_2x: u32,
    pub amdtp_rx_pcm_channels_4x: u32,
    pub amdtp_tx_pcm_channels_4x: u32,
    pub reserved: [u32; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_efw_grp_type {
    SND_EFW_CH_TYPE_ANALOG			= 0,
    SND_EFW_CH_TYPE_SPDIF			= 1,
    SND_EFW_CH_TYPE_ADAT			= 2,
    SND_EFW_CH_TYPE_SPDIF_OR_ADAT		= 3,
    SND_EFW_CH_TYPE_ANALOG_MIRRORING	= 4,
    SND_EFW_CH_TYPE_HEADPHONES		= 5,
    SND_EFW_CH_TYPE_I2S			= 6,
    SND_EFW_CH_TYPE_GUITAR			= 7,
    SND_EFW_CH_TYPE_PIEZO_GUITAR		= 8,
    SND_EFW_CH_TYPE_GUITAR_STRING		= 9,
    SND_EFW_CH_TYPE_DUMMY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_efw_phys_meters {
    pub /: *mut *mut u32 status; / guitar state/midi signal/clock input detect,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
    pub out_meters: u32,
    pub in_meters: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub values: [u32; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_efw_clock_source {
    SND_EFW_CLOCK_SOURCE_INTERNAL	= 0,
// Unused.
    SND_EFW_CLOCK_SOURCE_WORDCLOCK	= 2,
    SND_EFW_CLOCK_SOURCE_SPDIF	= 3,
    SND_EFW_CLOCK_SOURCE_ADAT_1	= 4,
    SND_EFW_CLOCK_SOURCE_ADAT_2	= 5,
    SND_EFW_CLOCK_SOURCE_CONTINUOUS	= 6	/* internal variable clock */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_efw_transport_mode {
    SND_EFW_TRANSPORT_MODE_WINDOWS	= 0,
    SND_EFW_TRANSPORT_MODE_IEC61883	= 1,
}

    pub addr_low): u16 addr_high, u32,
    pub mode): snd_efw_transport_mode,
    pub hwinfo): *mut snd_efw_hwinfo,
    pub len): c_uint,
    pub source): *mut snd_efw_clock_source,
    pub rate): *mut *mut int snd_efw_command_get_sampling_rate(struct snd_efw efw, unsigned int,
    pub rate): *mut *mut int snd_efw_command_set_sampling_rate(struct snd_efw efw, unsigned int,
    pub efw): *mut int snd_efw_stream_init_duplex(struct snd_efw,
    pub frames_per_buffer): c_uint,
    pub efw): *mut int snd_efw_stream_start_duplex(struct snd_efw,
    pub efw): *mut void snd_efw_stream_stop_duplex(struct snd_efw,
    pub efw): *mut void snd_efw_stream_update_duplex(struct snd_efw,
    pub efw): *mut void snd_efw_stream_destroy_duplex(struct snd_efw,
    pub efw): *mut void snd_efw_stream_lock_changed(struct snd_efw,
    pub efw): *mut int snd_efw_stream_lock_try(struct snd_efw,
    pub efw): *mut void snd_efw_stream_lock_release(struct snd_efw,
    pub efw): *mut void snd_efw_proc_init(struct snd_efw,
    pub efw): *mut int snd_efw_create_midi_devices(struct snd_efw,
    pub efw): *mut int snd_efw_create_pcm_devices(struct snd_efw,
    pub mode): *mut int snd_efw_get_multiplier_mode(unsigned int sampling_rate, unsigned int,
    pub efw): *mut int snd_efw_create_hwdep_device(struct snd_efw,
