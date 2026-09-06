//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usbaudio.h
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
// (Tentative) USB Audio Driver for ALSA
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//

// handling of USB vendor/product ID pairs as 32-bit numbers

//
pub const MAX_CARD_INTERFACES: c_int = 16;
//
// Structure holding assosiation between Audio Control Interface
// and given Streaming or Midi Interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_intf_to_ctrl {
    pub interface: u8,
    pub ctrl_intf: *mut usb_host_interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_audio {
    pub index: c_int,
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub intf: [*mut usb_interface; MAX_CARD_INTERFACES],
    pub usb_id: u32,
    pub quirk_type: u16,
    pub mutex: mutex,
    pub system_suspend: c_uint,
    pub active: core::sync::atomic::AtomicI32,
    pub shutdown: core::sync::atomic::AtomicI32,
    pub usage_count: snd_refcount,
    pub quirk_flags: u64,
    pub /: *mut *mut unsigned int need_delayed_register:1; / warn for delayed registration,
    pub num_interfaces: c_int,
    pub last_iface: c_int,
    pub num_suspended_intf: c_int,
    pub sample_rate_read_error: c_int,
    pub /: *mut *mut int badd_profile; / UAC3 BADD profile,
    pub /: *mut *mut list_head pcm_list; / list of pcm streams,
    pub /: *mut *mut list_head ep_list; / list of audio-related endpoints,
    pub /: *mut *mut list_head iface_ref_list; / list of interface refcounts,
    pub /: *mut *mut list_head clock_ref_list; / list of clock refcounts,
    pub pcm_devs: c_int,
    pub /: *mut *mut unsigned int num_rawmidis; / number of created rawmidi devices,
    pub /: *mut *mut list_head midi_list; / list of midi interfaces,
    pub /: *mut *mut list_head midi_v2_list; / list of MIDI 2 interfaces,
    pub /: *mut *mut list_head mixer_list; / list of mixer interfaces,
    pub /: *mut *mut int setup; / from the 'device_setup' module param,
    pub /: *mut *mut bool generic_implicit_fb; / from the 'implicit_fb' module param,
    pub /: *mut *mut bool autoclock; / from the 'autoclock' module param,
    pub /: *mut *mut bool lowlatency; / from the 'lowlatency' module param,
    pub /: *mut *mut *mut usb_host_interface ctrl_intf; / the audio control interface,
    pub media_dev: *mut media_device,
    pub ctl_intf_media_devnode: *mut media_intf_devnode,
    pub num_intf_to_ctrl: c_uint,
    pub intf_to_ctrl: [snd_intf_to_ctrl; MAX_CARD_INTERFACES],
}

//
// Information about devices with broken descriptors
//
// special values for .ifnum

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quirk_type {
    QUIRK_IGNORE_INTERFACE,
    QUIRK_COMPOSITE,
    QUIRK_AUTODETECT,
    QUIRK_MIDI_STANDARD_INTERFACE,
    QUIRK_MIDI_FIXED_ENDPOINT,
    QUIRK_MIDI_YAMAHA,
    QUIRK_MIDI_ROLAND,
    QUIRK_MIDI_MIDIMAN,
    QUIRK_MIDI_NOVATION,
    QUIRK_MIDI_RAW_BYTES,
    QUIRK_MIDI_EMAGIC,
    QUIRK_MIDI_CME,
    QUIRK_MIDI_AKAI,
    QUIRK_MIDI_US122L,
    QUIRK_MIDI_FTDI,
    QUIRK_MIDI_CH345,
    QUIRK_AUDIO_STANDARD_INTERFACE,
    QUIRK_AUDIO_FIXED_ENDPOINT,
    QUIRK_AUDIO_EDIROL_UAXX,
    QUIRK_AUDIO_STANDARD_MIXER,

    QUIRK_TYPE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_audio_quirk {
    pub vendor_name: *const c_char,
    pub product_name: *const c_char,
    pub ifnum: i16,
    pub type: u16,
    pub data: *const c_void,
}

extern "C" {
    pub fn snd_usb_lock_shutdown(chip: *mut snd_usb_audio) -> c_int;
}
extern "C" {
    pub fn snd_usb_unlock_shutdown(chip: *mut snd_usb_audio);
}
// auto-cleanup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_usb_lock {
    pub chip: *mut snd_usb_audio,
    pub err: c_int,
}

//
// Driver behavior quirk flags, stored in chip->quirk_flags
//
// QUIRK_FLAG_GET_SAMPLE_RATE:
// Skip reading sample rate for devices, as some devices behave inconsistently
// or return error
// QUIRK_FLAG_SHARE_MEDIA_DEVICE:
// Create Media Controller API entries
// QUIRK_FLAG_ALIGN_TRANSFER:
// Allow alignment on audio sub-slot (channel samples) rather than on audio
// slots (audio frames)
// QUIRK_TX_LENGTH:
// Add length specifier to transfers
// QUIRK_FLAG_PLAYBACK_FIRST:
// Start playback stream at first even in implement feedback mode
// QUIRK_FLAG_SKIP_CLOCK_SELECTOR:
// Skip clock selector setup; the device may reset to invalid state
// QUIRK_FLAG_IGNORE_CLOCK_SOURCE:
// Ignore errors from clock source search; i.e. hardcoded clock
// QUIRK_FLAG_ITF_USB_DSD_DAC:
// Indicates the device is for ITF-USB DSD based DACs that need a vendor cmd
// to switch between PCM and native DSD mode
// QUIRK_FLAG_CTL_MSG_DELAY:
// Add a delay of 20ms at each control message handling
// QUIRK_FLAG_CTL_MSG_DELAY_1M:
// Add a delay of 1-2ms at each control message handling
// QUIRK_FLAG_CTL_MSG_DELAY_5M:
// Add a delay of 5-6ms at each control message handling
// QUIRK_FLAG_IFACE_DELAY:
// Add a delay of 50ms at each interface setup
// QUIRK_FLAG_VALIDATE_RATES:
// Perform sample rate validations at probe
// QUIRK_FLAG_DISABLE_AUTOSUSPEND:
// Disable runtime PM autosuspend
// QUIRK_FLAG_IGNORE_CTL_ERROR:
// Ignore errors for mixer access
// QUIRK_FLAG_DSD_RAW:
// Support generic DSD raw U32_BE format
// QUIRK_FLAG_SET_IFACE_FIRST:
// Set up the interface at first like UAC1
// QUIRK_FLAG_GENERIC_IMPLICIT_FB
// Apply the generic implicit feedback sync mode (same as implicit_fb=1 option)
// QUIRK_FLAG_SKIP_IMPLICIT_FB
// Don't apply implicit feedback sync mode
// QUIRK_FLAG_IFACE_SKIP_CLOSE
// Don't closed interface during setting sample rate
// QUIRK_FLAG_FORCE_IFACE_RESET
// Force an interface reset whenever stopping & restarting a stream
// (e.g. after xrun)
// QUIRK_FLAG_FIXED_RATE
// Do not set PCM rate (frequency) when only one rate is available
// for the given endpoint.
// QUIRK_FLAG_MIC_RES_16 and QUIRK_FLAG_MIC_RES_384
// Set the fixed resolution for Mic Capture Volume (mostly for webcams)
// QUIRK_FLAG_MIXER_PLAYBACK_MIN_MUTE
// Set minimum volume control value as mute for devices where the lowest
// playback value represents muted state instead of minimum audible volume
// QUIRK_FLAG_MIXER_CAPTURE_MIN_MUTE
// Similar to QUIRK_FLAG_MIXER_PLAYBACK_MIN_MUTE, but for capture streams
// QUIRK_FLAG_SKIP_IFACE_SETUP
// Skip the probe-time interface setup (usb_set_interface,
// init_pitch, init_sample_rate); redundant with
// snd_usb_endpoint_prepare() at stream-open time
// QUIRK_FLAG_MIXER_PLAYBACK_LINEAR_VOL
// Set linear volume mapping for devices where the playback volume control
// value is mapped to voltage (instead of dB) level linearly. In short:
// x(raw) = (raw - raw_min) / (raw_max - raw_min); V(x) = k * x;
// dB(x) = 20 * log10(x). Overrides QUIRK_FLAG_MIXER_PLAYBACK_MIN_MUTE
// QUIRK_FLAG_MIXER_CAPTURE_LINEAR_VOL
// Similar to QUIRK_FLAG_MIXER_PLAYBACK_LINEAR_VOL, but for capture streams.
// Overrides QUIRK_FLAG_MIXER_CAPTURE_MIN_MUTE
// QUIRK_FLAG_IFB_SILENCE_ON_EMPTY
// In implicit feedback mode, when an entire capture URB returns with
// all iso_frame_desc[i].status != 0 (bytes==0), do not silently return
// from snd_usb_handle_sync_urb. Instead fall through and enqueue a
// packet_info containing only size-0 packets, so the OUT ring keeps
// moving (emits silence). Needed by Behringer Flow 8 (1397:050c).
// QUIRK_FLAG_MIXER_GET_CUR_OK
// On some devices, whether their GET_CUR being sticky depends on whether
// hotpluggable components are present. When the hotpluggable components are
// missing on probe, their GET_CUR behavior is classified as broken. Set the
// flag to prevent the heuristics from gating GET_CUR.
// QUIRK_FLAG_PLAYBACK_URB_FIXUP
// Set URB_ISO_ASAP flag for isochronous URBs and force nurbs to MAX_URBS.
// This is needed for devices that exhibit boot-time audio stuttering due
// to insufficient buffer depth combined with xHCI scheduling variability.
// The larger buffer (MAX_URBS = 12, ~64ms) absorbs system scheduling
// jitter during boot, while URB_ISO_ASAP ensures consistent xHCI scheduling.
// QUIRK_FLAG_ALWAYS_SET_RATE
// Issue SET_CUR for the sample rate even when the clock already reports the
// requested rate.  A device advertising a single rate is otherwise never sent
// the request at all, and some require it before streaming will start.
//
// Please also edit snd_usb_audio_quirk_flag_names and alsa-configuration.rst

