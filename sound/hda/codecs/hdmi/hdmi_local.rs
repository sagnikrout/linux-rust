//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/hdmi/hdmi_local.h
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
// HD-audio HDMI codec driver
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_spec_per_cvt {
    pub cvt_nid: hda_nid_t,
    pub /: *mut *mut bool assigned; / the stream has been assigned,
    pub /: *mut *mut bool silent_stream; / silent stream activated,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u32,
    pub formats: u64,
    pub maxbps: c_uint,
}

// max. connections to a widget
pub const HDA_MAX_CONNECTIONS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_spec_per_pin {
    pub pin_nid: hda_nid_t,
    pub dev_id: c_int,
// pin idx, different device entries on the same pin use the same idx
    pub pin_nid_idx: c_int,
    pub num_mux_nids: c_int,
    pub mux_nids: [hda_nid_t; HDA_MAX_CONNECTIONS],
    pub mux_idx: c_int,
    pub cvt_nid: hda_nid_t,
    pub codec: *mut hda_codec,
    pub sink_eld: hdmi_eld,
    pub lock: mutex,
    pub work: delayed_work,
    pub dynamically*/: *mut *mut *mut hdmi_pcm pcm; / pointer to spec->pcm_rec[n],
    pub /: *mut *mut int pcm_idx; / which pcm is attached. -1 means no pcm is attached,
    pub /: *mut *mut int prev_pcm_idx; / previously assigned pcm index,
    pub repoll_count: c_int,
    pub /: *mut *mut bool setup; / the stream has been set up by prepare callback,
    pub silent_stream: bool,
    pub /: *mut *mut int channels; / current number of channels,
    pub non_pcm: bool,
    pub /: *mut *mut bool chmap_set; / channel-map override by ALSA API?,
    pub /: *mut *mut unsigned char chmap[8]; / ALSA API channel-map,

    pub proc_entry: *mut snd_info_entry,

}

// operations used by generic code that can be overridden by codec drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_ops {
    pub eld_size): *mut *mut int dev_id, unsigned char buf, int,
    pub conn_type): int ca, int active_channels, int,
// enable/disable HBR (HD passthrough)
    pub hbr): int dev_id, bool,
    pub format): c_int,
//
// Optional hook invoked at the beginning of the PCM prepare
// sequence, before the audio infoframe and stream format are
// (re)programmed. Used to disable keep-alive / silent stream so
// that the format change is not done while keep-alive is active.
//
    pub per_pin): *mut hdmi_spec_per_pin,
    pub cvt_nid): hda_nid_t,
    pub enable): bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_pcm {
    pub pcm: *mut hda_pcm,
    pub jack: *mut snd_jack,
    pub eld_ctl: *mut snd_kcontrol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_spec {
    pub codec: *mut hda_codec,
    pub num_cvts: c_int,
    pub /: *mut *mut snd_array cvts; / hdmi_spec_per_cvt,
    pub /: *mut *mut hda_nid_t cvt_nids[4]; / only for haswell fix,
//
// num_pins is the number of virtual pins
// for example, there are 3 pins, and each pin
// has 4 device entries, then the num_pins is 12
//
    pub num_pins: c_int,
//
// num_nids is the number of real pins
// In the above example, num_nids is 3
//
    pub num_nids: c_int,
//
// dev_num is the number of device entries
// on each pin.
// In the above example, dev_num is 4
//
    pub dev_num: c_int,
    pub /: *mut *mut snd_array pins; / hdmi_spec_per_pin,
    pub pcm_rec: [hdmi_pcm; 8],
    pub pcm_lock: mutex,
    pub /: *mut *mut mutex bind_lock; / for audio component binding,
// pcm_bitmap means which pcms have been assigned to pins
    pub pcm_bitmap: c_ulong,
    pub /: *mut *mut int pcm_used; / counter of pcm_rec[],
// bitmap shows whether the pcm is opened in user space
// bit 0 means the first playback PCM (PCM3);
// bit 1 means the second playback PCM, and so on.
//
    pub pcm_in_use: c_ulong,
    pub temp_eld: hdmi_eld,
    pub ops: hdmi_ops,
    pub dyn_pin_out: bool,
    pub static_pcm_mapping: bool,
// hdmi interrupt trigger control flag for Nvidia codec
    pub hdmi_intr_trig_ctrl: bool,
    pub /: *mut *mut bool nv_dp_workaround; / workaround DP audio infoframe for Nvidia,
    pub /: *mut *mut bool intel_hsw_fixup; / apply Intel platform-specific fixups,
//
// Non-generic VIA/NVIDIA specific
//
    pub multiout: hda_multi_out,
    pub pcm_playback: hda_pcm_stream,
    pub /: *mut *mut bool use_acomp_notifier; / use eld_notify callback for hotplug,
    pub /: *mut *mut bool acomp_registered; / audio component registered in this driver,
    pub /: *mut *mut bool force_connect; / force connectivity,
    pub drm_audio_ops: drm_audio_component_audio_ops,
    pub /: *mut *mut *mut *mut int (port2pin)(struct hda_codec codec, int port); / reverse port/pin mapping,
    pub chmap: hdac_chmap,
    pub vendor_nid: hda_nid_t,
    pub port_map: *const c_int,
    pub port_num: c_int,
    pub silent_stream_type: c_int,
    pub hw_constraints_channels: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_infoframe {
    pub /: *mut *mut u8 type; / 0x84,
    pub /: *mut *mut u8 ver; / 0x01,
    pub /: *mut *mut u8 len; / 0x0a,
    pub checksum: u8,
    pub /: *mut *mut u8 CC02_CT47; / CC in bits 0:2, CT in 4:7,
    pub SS01_SF24: u8,
    pub CXT04: u8,
    pub CA: u8,
    pub LFEPBL01_LSV36_DM_INH7: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_audio_infoframe {
    pub /: *mut *mut u8 type; / 0x84,
    pub /: *mut *mut u8 len; / 0x1b,
    pub /: *mut *mut u8 ver; / 0x11 << 2,
    pub /: *mut *mut u8 CC02_CT47; / match with HDMI infoframe from this on,
    pub SS01_SF24: u8,
    pub CXT04: u8,
    pub CA: u8,
    pub LFEPBL01_LSV36_DM_INH7: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union audio_infoframe {
    pub hdmi: hdmi_audio_infoframe,
    pub dp: dp_audio_infoframe,
    pub bytes): DECLARE_FLEX_ARRAY(u8,,
}

// support only the safe format and rate

pub const SUPPORTED_MAXBPS: c_int = 16;

// support all rates and formats

pub const SUPPORTED_MAXBPS: c_int = 24;

//
// HDMI routines
//

// obtain hdmi_pcm object assigned to idx

// obtain hda_pcm object assigned to idx

// Generic HDMI codec support
extern "C" {
    pub fn snd_hda_hdmi_generic_alloc(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_parse_codec(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_probe(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: c_uint);
}

extern "C" {
    pub fn snd_hda_hdmi_generic_init_per_pins(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_generic_spec_free(codec: *mut hda_codec);
}
// Audio component support
extern "C" {
    pub fn snd_hda_hdmi_acomp_pin_eld_notify(audio_ptr: *mut c_void, port: c_int, dev_id: c_int);
}
// Simple / legacy HDMI codec support
extern "C" {
    pub fn snd_hda_hdmi_simple_remove(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_hdmi_simple_build_pcms(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_simple_build_controls(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_hdmi_simple_init(codec: *mut hda_codec) -> c_int;
}
