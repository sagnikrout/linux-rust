//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda_codec.h
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
// Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
//

//
// Structures
//
// codec bus
//
// each controller needs to creata a hda_bus to assign the accessor.
// A hda_bus contains several codecs in the list codec_list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_bus {
    pub core: hdac_bus,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub modelname: *const c_char,
    pub prepare_mutex: mutex,
// assigned PCMs
    pub SNDRV_PCM_DEVICES): DECLARE_BITMAP(pcm_dev_bits,,
// misc op flags
    pub /: *mut *mut unsigned int allow_bus_reset:1; / allow bus reset at fatal error,
// status for codec/controller
    pub /: *mut *mut unsigned int shutdown :1; / being unloaded,
    pub /: *mut *mut unsigned int response_reset:1; / controller was reset,
    pub /: *mut *mut unsigned int in_reset:1; / during reset operation,
    pub /: *mut *mut unsigned int no_response_fallback:1; / don't fallback at RIRB error,
    pub /: *mut *mut unsigned int bus_probing :1; / during probing process,
    pub /: *mut *mut unsigned int keep_power:1; / keep power up for notification,
    pub during: *mut *mut unsigned int jackpoll_in_suspend:1; / keep jack polling,
// runtime suspend
//
    pub /: *mut *mut int primary_dig_out_type; / primary digital out PCM type,
    pub /: *mut *mut unsigned int mixer_assigned; / codec addr for mixer name,
}

// from hdac_bus to hda_bus

//
// codec preset
//
pub const HDA_CODEC_ID_SKIP_PROBE: c_uint = 0x00000001;
pub const HDA_CODEC_ID_GENERIC_HDMI: c_uint = 0x00000101;
pub const HDA_CODEC_ID_GENERIC: c_uint = 0x00000201;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_codec_driver {
    pub core: hdac_driver,
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

extern "C" {
    pub fn hda_codec_driver_unregister(drv: *mut hda_codec_driver);
}

// ops for hda codec driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_codec_ops {
    pub id): *const *const *const int (probe)(struct hda_codec codec, struct hda_device_id,
    pub codec): *mut *mut void (remove)(struct hda_codec,
    pub codec): *mut *mut int (build_controls)(struct hda_codec,
    pub codec): *mut *mut int (build_pcms)(struct hda_codec,
    pub codec): *mut *mut int (init)(struct hda_codec,
    pub res): *mut *mut *mut void (unsol_event)(struct hda_codec codec, unsigned int,
    pub power_state): c_uint,
    pub codec): *mut *mut int (suspend)(struct hda_codec,
    pub codec): *mut *mut int (resume)(struct hda_codec,
    pub nid): *mut *mut *mut int (check_power_status)(struct hda_codec codec, hda_nid_t,
    pub on): *mut *mut *mut void (stream_pm)(struct hda_codec codec, hda_nid_t nid, bool,
}

// PCM callbacks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pcm_ops {
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
}

// PCM information for each substream
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pcm_stream {
    pub exist*/: *mut *mut unsigned int substreams; / number of substreams, 0 = not,
    pub /: *mut *mut unsigned int channels_min; / min. number of channels,
    pub /: *mut *mut unsigned int channels_max; / max. number of channels,
    pub /: *mut *mut hda_nid_t nid; / default NID to query rates/formats/bps, or set up,
    pub /: *mut *mut u32 rates; / supported rates,
    pub /: *mut *mut u64 formats; / supported formats (SNDRV_PCM_FMTBIT_),
    pub /: *mut *mut *mut u32 subformats; / for S32_LE format, SNDRV_PCM_SUBFMTBIT_,
    pub /: *mut *mut unsigned int maxbps; / supported max. bit per sample,
    pub /: *const *const *const snd_pcm_chmap_elem chmap; / chmap to override,
    pub ops: hda_pcm_ops,
}

// PCM types

// for PCM creation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pcm {
    pub name: *mut c_char,
    pub stream: [hda_pcm_stream; 2],
    pub /: *mut *mut unsigned int pcm_type; / HDA_PCM_TYPE_XXX,
    pub /: *mut *mut int device; / device number to assign,
    pub /: *mut *mut *mut snd_pcm pcm; / assigned PCM instance,
    pub /: *mut *mut bool own_chmap; / codec driver provides own channel maps,
// private:
    pub codec: *mut hda_codec,
    pub list: list_head,
    pub disconnected:1: c_uint,
}

// codec information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_codec {
    pub core: hdac_device,
    pub bus: *mut hda_bus,
    pub card: *mut snd_card,
    pub addr*/: *mut *mut unsigned int addr; / codec,
    pub /: *mut *mut u32 probe_id; / overridden id for probing,
// detected preset
    pub preset: *const hda_device_id,
    pub /: *const *const *const char modelname; / model name for preset,
// PCM to create, set by hda_codec_ops.build_pcms callback
    pub pcm_list_head: list_head,
    pub pcm_ref: snd_refcount,
// codec specific info
    pub spec: *mut c_void,
// beep device
    pub beep: *mut hda_beep,
    pub beep_mode: c_uint,
    pub beep_just_power_on: bool,
// widget capabilities cache
    pub wcaps: *mut u32,
    pub /: *mut *mut snd_array mixers; / list of assigned mixer elements,
    pub /: *mut *mut snd_array nids; / list of mapped mixer elements,
    pub /: *mut *mut list_head conn_list; / linked-list of connection-list,
    pub spdif_mutex: mutex,
    pub control_mutex: mutex,
    pub spdif_out: snd_array,
    pub /: *mut *mut unsigned int spdif_in_enable; / SPDIF input enable?,
    pub /: *const *const *const hda_nid_t follower_dig_outs; / optional digital out follower widgets,
    pub /: *mut *mut snd_array init_pins; / initial (BIOS) pin configurations,
    pub /: *mut *mut snd_array driver_pins; / pin configs set by codec parser,
    pub /: *mut *mut snd_array cvt_setups; / audio convert setups,
    pub user_mutex: mutex,

    pub /: *mut *mut snd_array init_verbs; / additional init verbs,
    pub /: *mut *mut snd_array hints; / additional hints,
    pub /: *mut *mut snd_array user_pins; / default pin configs to override,

    pub /: *mut *mut *mut snd_hwdep hwdep; / assigned hwdep device,

// misc flags
    pub /: *mut *mut unsigned int configured:1; / codec was configured,
    pub /: *mut *mut unsigned int in_freeing:1; / being released,
    pub /: *mut *mut unsigned int display_power_control:1; / needs display power,
    pub each: *mut *mut unsigned int spdif_status_reset :1; / needs to toggle SPDIF for,
// status change
// (e.g. Realtek codecs)
//
    pub index: *mut *mut unsigned int pin_amp_workaround:1; / pin out-amp takes,
// (e.g. Conexant codecs)
//
    pub index: *mut *mut unsigned int single_adc_amp:1; / adc in-amp takes no,
// (e.g. CX20549 codec)
//
    pub /: *mut *mut unsigned int no_sticky_stream:1; / no sticky-PCM stream assignment,
    pub /: *mut *mut unsigned int pins_shutup:1; / pins are shut up,
    pub /: *mut *mut unsigned int no_trigger_sense:1; / don't trigger at pin-sensing,
    pub /: *mut *mut unsigned int no_jack_detect:1; / Machine has no jack-detection,
    pub /: *mut *mut unsigned int inv_eapd:1; / broken h/w: inverted EAPD control,
    pub /: *mut *mut unsigned int inv_jack_detect:1; / broken h/w: inverted detection bit,
    pub /: *mut *mut unsigned int pcm_format_first:1; / PCM format must be set first,
    pub /: *mut *mut unsigned int cached_write:1; / write only to caches,
    pub /: *mut *mut unsigned int dp_mst:1; / support DP1.2 Multi-stream transport,
    pub /: *mut *mut unsigned int dump_coef:1; / dump processing coefs in codec proc file,
    pub /: *mut *mut unsigned int power_save_node:1; / advanced PM for each widget,
    pub /: *mut *mut unsigned int auto_runtime_pm:1; / enable automatic codec runtime pm,
    pub /: *mut *mut unsigned int force_pin_prefix:1; / Add location prefix,
    pub /: *mut *mut unsigned int link_down_at_suspend:1; / link down at runtime suspend,
    pub /: *mut *mut unsigned int relaxed_resume:1; / don't resume forcibly for jack,
    pub /: *mut *mut unsigned int forced_resume:1; / forced resume for jack,
    pub /: *mut *mut unsigned int acomp_requested_resume:1; / resume requested by acomp,
    pub /: *mut *mut unsigned int no_stream_clean_at_suspend:1; / do not clean streams at suspend,
    pub /: *mut *mut unsigned int ctl_dev_id:1; / old control element id build behaviour,
    pub /: *mut *mut unsigned int eld_jack_detect:1; / Machine jack-detection by ELD,
    pub power_on_acct: c_ulong,
    pub power_off_acct: c_ulong,
    pub power_jiffies: c_ulong,
// filter the requested power state per nid
    pub power_state): c_uint,
// codec-specific additional proc output
    pub nid): *mut *mut hda_codec codec, hda_nid_t,
// jack detection
    pub jacktbl: snd_array,
    pub /: *mut *mut unsigned long jackpoll_interval; / In jiffies. Zero means no poll, rely on unsol events,
    pub jackpoll_work: delayed_work,
    pub /: *mut *mut int depop_delay; / depop delay in ms, -1 for default delay time,
// fix-up list
    pub fixup_id: c_int,
    pub fixup_list: *const hda_fixup,
    pub fixup_name: *const c_char,
// additional init verbs
    pub verbs: snd_array,
}

// snd_hda_codec_read/write optional flags

//
// constructors
//
extern "C" {
    pub fn snd_hda_codec_configure(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_codec_update_widgets(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_codec_register(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_codec_unregister(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_codec_cleanup_for_unbind(codec: *mut hda_codec);
}
//
// low level functions
//
extern "C" {
    pub fn snd_hdac_codec_read(_arg: &codec->core, _arg: nid, _arg: flags, _arg: verb, _arg: parm) -> return;
}
extern "C" {
    pub fn snd_hdac_codec_write(_arg: &codec->core, _arg: nid, _arg: flags, _arg: verb, _arg: parm) -> return;
}
// sync after write
// use snd_hda_codec_read() for writing;
// the returned value is usually discarded
//
extern "C" {
    pub fn snd_hdac_codec_read(_arg: &codec->core, _arg: nid, _arg: flags, _arg: verb, _arg: parm) -> return;
}

extern "C" {
    pub fn snd_hda_get_connections(_arg: codec, _arg: nid, _arg: NULL, _arg: 0) -> return;
}

extern "C" {
    pub fn snd_hda_get_num_devices(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
}
extern "C" {
    pub fn snd_hda_get_dev_select(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}
extern "C" {
    pub fn snd_hda_set_dev_select(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: u32,
    pub param: u32,
}

// cached write
extern "C" {
    pub fn snd_hdac_regmap_write(_arg: &codec->core, _arg: nid, _arg: verb, _arg: parm) -> return;
}
// the struct for codec->pin_configs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pincfg {
    pub nid: hda_nid_t,
    pub /: *mut *mut unsigned char ctrl; / original pin control value,
    pub /: *mut *mut unsigned char target; / target pin control value,
    pub /: *mut *mut unsigned int cfg; / default configuration,
}

extern "C" {
    pub fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
}
extern "C" {
    pub fn snd_hda_shutup_pins(codec: *mut hda_codec);
}
// SPDIF controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_spdif_out {
    pub /: *mut *mut hda_nid_t nid; / Converter nid values relate to,
    pub /: *mut *mut unsigned int status; / IEC958 status bits,
    pub /: *mut *mut unsigned short ctls; / SPDIF control bits,
}

extern "C" {
    pub fn snd_hda_spdif_ctls_unassign(codec: *mut hda_codec, idx: c_int);
}
extern "C" {
    pub fn snd_hda_spdif_ctls_assign(codec: *mut hda_codec, idx: c_int, nid: hda_nid_t);
}
//
// Mixer
//
extern "C" {
    pub fn snd_hda_codec_build_controls(codec: *mut hda_codec) -> c_int;
}
//
// PCM
//
extern "C" {
    pub fn snd_hda_codec_parse_pcms(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_codec_build_pcms(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_codec_cleanup_for_unbind(codec: *mut hda_codec);
}

//
// Misc
//
extern "C" {
    pub fn snd_hda_get_codec_name(codec: *mut hda_codec, name: *mut c_char, namelen: c_int);
}
extern "C" {
    pub fn snd_hda_lock_devices(bus: *mut hda_bus) -> c_int;
}
extern "C" {
    pub fn snd_hda_unlock_devices(bus: *mut hda_bus);
}
extern "C" {
    pub fn snd_hda_bus_reset(bus: *mut hda_bus);
}
extern "C" {
    pub fn snd_hda_bus_reset_codecs(bus: *mut hda_bus);
}
extern "C" {
    pub fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const c_char) -> c_int;
}
//
// power management
//
// power saving
//

extern "C" {
    pub fn snd_hda_codec_set_power_save(codec: *mut hda_codec, delay: c_int);
}
extern "C" {
    pub fn snd_hda_set_power_save(bus: *mut hda_bus, delay: c_int);
}
extern "C" {
    pub fn snd_hda_update_power_acct(codec: *mut hda_codec);
}
//
// PM with auto-cleanup: call like CLASS(snd_hda_power, pm)(codec)
// If the error handling is needed, refer pm.err.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __hda_power_obj {
    pub codec: *mut hda_codec,
    pub err: c_int,
}

//
// patch firmware
//
extern "C" {
    pub fn snd_hda_load_patch(bus: *mut hda_bus, size: usize, buf: *const c_void) -> c_int;
}

extern "C" {
    pub fn snd_hda_codec_load_dsp_trigger(codec: *mut hda_codec, start: bool);
}

