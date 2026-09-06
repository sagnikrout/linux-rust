//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/common/hda_local.h
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
// Universal Interface for Intel High Definition Audio Codec
//
// Local helper functions
//
// Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
//

// We abuse kcontrol_new.subdev field to pass the NID corresponding to
// the given new control.  If id.subdev has a bit flag HDA_SUBDEV_NID_FLAG,
// snd_hda_ctl_add() takes the lower-bit subdev value as a valid NID.
//
// Note that the subdevice field is cleared again before the real registration
// in snd_hda_ctl_add(), so that this value won't appear in the outside.
//

//
// for mixer controls
//

// mono volume with index (index=0,1,...) (channel=1,2)

// stereo volume with index

// mono volume

// stereo volume

// stereo volume with min=mute

// mono mute switch with index (index=0,1,...) (channel=1,2)

// stereo mute switch with index

// mono mute switch

// stereo mute switch

// special beep mono mute switch with index (index=0,1,...) (channel=1,2)

// no digital beep - just the standard one

// special beep mono mute switch

// special beep stereo mute switch

// lowlevel accessor with caching; use carefully

extern "C" {
    pub fn snd_hda_codec_reset(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_codec_disconnect_pcms(codec: *mut hda_codec);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_vmaster_mute_hook {
// below two fields must be filled by the caller of
// snd_hda_add_vmaster_hook() beforehand
//
    pub sw_kctl: *mut snd_kcontrol,
    pub int): *mut *mut *mut void (hook)(void ,,
// below are initialized automatically
    pub codec: *mut hda_codec,
}

extern "C" {
    pub fn snd_hda_sync_vmaster_hook(hook: *mut hda_vmaster_mute_hook);
}
// amp value bits
pub const HDA_AMP_MUTE: c_uint = 0x80;
pub const HDA_AMP_UNMUTE: c_uint = 0x00;
pub const HDA_AMP_VOLMASK: c_uint = 0x7f;
//
// SPDIF I/O
//

extern "C" {
    pub fn snd_hda_create_spdif_in_ctls(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}
//
// input MUX helper
//
pub const HDA_MAX_NUM_INPUTS: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_input_mux_item {
    pub label: [c_char; 32],
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_input_mux {
    pub num_items: c_uint,
    pub items: [hda_input_mux_item; HDA_MAX_NUM_INPUTS],
}

//
// Multi-channel / digital-out PCM helper
//
pub const HDA_MAX_OUTS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_multi_out {
    pub /: *mut *mut int num_dacs; / # of DACs, must be more than 1,
    pub /: *const *const *const hda_nid_t dac_nids; / DAC list,
    pub /: *mut *mut hda_nid_t hp_nid; / optional DAC for HP, 0 when not exists,
    pub /: *mut *mut hda_nid_t hp_out_nid[HDA_MAX_OUTS]; / DACs for multiple HPs,
    pub /: *mut *mut hda_nid_t extra_out_nid[HDA_MAX_OUTS]; / other (e.g. speaker) DACs,
    pub /: *mut *mut hda_nid_t dig_out_nid; / digital out audio widget,
    pub follower_dig_outs: *const hda_nid_t,
    pub /: *mut *mut int max_channels; / currently supported analog channels,
    pub /: *mut *mut int dig_out_used; / current usage of digital out (HDA_DIG_XXX),
    pub /: *mut *mut int no_share_stream; / don't share a stream with multiple pins,
    pub /: *mut *mut int share_spdif; / share SPDIF pin,
// PCM information for both analog and SPDIF DACs
    pub analog_rates: c_uint,
    pub analog_maxbps: c_uint,
    pub analog_formats: u64,
    pub spdif_rates: c_uint,
    pub spdif_maxbps: c_uint,
    pub spdif_formats: u64,
    pub /: *mut *mut *mut snd_kcontrol share_spdif_kctl; / cached shared SPDIF switch,
}

//
// generic proc interface
//

extern "C" {
    pub fn snd_hda_codec_proc_new(codec: *mut hda_codec) -> c_int;
}

pub const SND_PRINT_BITS_ADVISED_BUFSIZE: c_int = 16;
extern "C" {
    pub fn snd_print_pcm_bits(pcm: c_int, buf: *mut c_char, buflen: c_int);
}
//
// Misc
//
// Fix-up pin default configurations and add default verbs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_fixup {
    pub type: c_int,
    pub /: *mut *mut bool chained:1; / call the chained fixup(s) after this,
    pub /: *mut *mut bool chained_before:1; / call the chained fixup(s) before this,
    pub chain_id: c_int,
    pub pins: *const hda_pintbl,
    pub verbs: *const hda_verb,
    pub action): c_int,
    pub v: },
}

//
// extended form of snd_pci_quirk:
// for PCI SSID matching, use SND_PCI_QUIRK() like before;
// for codec SSID matching, use the new HDA_CODEC_QUIRK() instead
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_quirk {
    pub /: *mut *mut unsigned short subvendor; / PCI subvendor ID,
    pub /: *mut *mut unsigned short subdevice; / PCI subdevice ID,
    pub /: *mut *mut unsigned short subdevice_mask; / bitmask to match,
    pub /: *mut *mut bool match_codec_ssid; / match only with codec SSID,
    pub /: *mut *mut int value; / value,

    pub /: *const *const *const char name; / name of the device (optional),

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hda_pin_quirk {
    pub /: *mut *mut unsigned int codec; / Codec vendor/device ID,
    pub /: *mut *mut unsigned short subvendor; / PCI subvendor ID,
    pub /: *const *const *const hda_pintbl pins; / list of matching pins,

    pub name: *const c_char,

    pub /: *mut *mut int value; / quirk value,
}

// fixup types
// fixup action definitions
extern "C" {
    pub fn snd_hda_add_verbs(codec: *mut hda_codec, list: *const hda_verb) -> c_int;
}
extern "C" {
    pub fn snd_hda_apply_verbs(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
}
extern "C" {
    pub fn __snd_hda_apply_fixup(codec: *mut hda_codec, id: c_int, action: c_int, depth: c_int);
}
// helper macros to retrieve pin default-config values

// amp values

pub const AMP_OUT_MUTE: c_uint = 0xb080;
pub const AMP_OUT_UNMUTE: c_uint = 0xb000;
pub const AMP_OUT_ZERO: c_uint = 0xb000;
// pinctl values

extern "C" {
    pub fn snd_hda_get_default_vref(codec: *mut hda_codec, pin: hda_nid_t) -> c_uint;
}
//
// snd_hda_set_pin_ctl - Set a pin-control value safely
// @codec: the codec instance
// @pin: the pin NID to set the control
// @val: the pin-control value (AC_PINCTL_* bits)
//
// This function sets the pin-control value to the given pin, but
// filters out the invalid pin-control bits when the pin has no such
// capabilities.  For example, when PIN_HP is passed but the pin has no
// HP-drive capability, the HP bit is omitted.
//
// The function doesn't check the input VREF capability bits, though.
// Use snd_hda_get_default_vref() to guess the right value.
// Also, this function is only for analog pins, not for HDMI pins.
//
extern "C" {
    pub fn _snd_hda_set_pin_ctl(_arg: codec, _arg: pin, _arg: val, _arg: false) -> return;
}
//
// snd_hda_set_pin_ctl_cache - Set a pin-control value safely
// @codec: the codec instance
// @pin: the pin NID to set the control
// @val: the pin-control value (AC_PINCTL_* bits)
//
// Just like snd_hda_set_pin_ctl() but write to cache as well.
//
extern "C" {
    pub fn _snd_hda_set_pin_ctl(_arg: codec, _arg: pin, _arg: val, _arg: true) -> return;
}
extern "C" {
    pub fn snd_hda_codec_get_pin_target(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}

// Set the codec power_state flag to indicate to allow unsol event handling;
// see hda_codec_unsol_event() in hda_bind.c.  Calling this might confuse the
// state tracking, so use with care.
//
// get widget capabilities
//
// get the widget type from widget capability bits
extern "C" {
    pub fn query_amp_caps(codec: *mut hda_codec, nid: hda_nid_t, direction: c_int) -> u32;
}
//
// snd_hda_query_pin_caps - Query PIN capabilities
// @codec: the HD-auio codec
// @nid: the NID to query
//
// Query PIN capabilities for the given widget.
// Returns the obtained capability bits.
//
// When cap bits have been already read, this doesn't read again but
// returns the cached value.
//
extern "C" {
    pub fn snd_hda_param_read(_arg: codec, _arg: nid, _arg: AC_PAR_PIN_CAP) -> return;
}
//
// snd_hda_override_pin_caps - Override the pin capabilities
// @codec: the CODEC
// @nid: the NID to override
// @caps: the capability bits to set
//
// Override the cached PIN capabilitiy bits value by the given one.
//
// Returns zero if successful or a negative error code.
//
extern "C" {
    pub fn snd_hdac_override_parm(_arg: &codec->core, _arg: nid, _arg: AC_PAR_PIN_CAP, _arg: caps) -> return;
}

// flags for hda_nid_item

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_nid_item {
    pub kctl: *mut snd_kcontrol,
    pub index: c_uint,
    pub nid: hda_nid_t,
    pub flags: c_ushort,
}

extern "C" {
    pub fn snd_hda_ctls_clear(codec: *mut hda_codec);
}
//
// hwdep interface
//

extern "C" {
    pub fn snd_hda_create_hwdep(codec: *mut hda_codec) -> c_int;
}

extern "C" {
    pub fn snd_hda_sysfs_init(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_sysfs_clear(codec: *mut hda_codec);
}

extern "C" {
    pub fn snd_hda_get_bool_hint(codec: *mut hda_codec, key: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_hda_get_int_hint(codec: *mut hda_codec, key: *const c_char, valp: *mut c_int) -> c_int;
}

//
// power-management
//
extern "C" {
    pub fn snd_hda_schedule_power_save(codec: *mut hda_codec);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_amp_list {
    pub nid: hda_nid_t,
    pub dir: c_uchar,
    pub idx: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_loopback_check {
    pub amplist: *const hda_amp_list,
    pub power_on: c_int,
}

// check whether the actual power state matches with the target state
extern "C" {
    pub fn snd_hdac_check_power_state(_arg: &codec->core, _arg: nid, _arg: target_state) -> return;
}
extern "C" {
    pub fn snd_hdac_sync_power_state(_arg: &codec->core, _arg: nid, _arg: target_state) -> return;
}
extern "C" {
    pub fn snd_hda_codec_shutdown(codec: *mut hda_codec);
}
//
// AMP control callbacks
//
// retrieve parameters from private_value

//
// enum control helper
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_eld {
    pub monitor_present: bool,
    pub eld_valid: bool,
    pub eld_size: c_int,
    pub eld_buffer: [c_char; ELD_MAX_SIZE],
    pub info: snd_parsed_hdmi_eld,
}

extern "C" {
    pub fn snd_hdmi_get_eld_size(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}

pub const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: c_int = 80;
extern "C" {
    pub fn snd_print_channel_allocation(spk_alloc: c_int, buf: *mut c_char, buflen: c_int);
}
extern "C" {
    pub fn snd_hda_codec_display_power(codec: *mut hda_codec, enable: bool);
}
//

// append a suffix string safely; equivalent with strlcat()
