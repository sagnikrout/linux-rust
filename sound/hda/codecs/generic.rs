//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/generic.h
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
// Generic BIOS auto-parser helper functions for HD-audio
//
// Copyright (c) 2012 Takashi Iwai <tiwai@suse.de>
//

// table entry for multi-io paths
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_multi_io {
    pub /: *mut *mut hda_nid_t pin; / multi-io widget pin NID,
    pub /: *mut *mut hda_nid_t dac; / DAC to be connected,
    pub /: *mut *mut unsigned int ctl_in; / cached input-pin control value,
}

// Widget connection path
//
// For output, stored in the order of DAC -> ... -> pin,
// for input, pin -> ... -> ADC.
//
// idx[i] contains the source index number to select on of the widget path[i];
// e.g. idx[1] is the index of the DAC (path[0]) selected by path[1] widget
// multi[] indicates whether it's a selector widget with multi-connectors
// (i.e. the connection selection is mandatory)
// vol_ctl and mute_ctl contains the NIDs for the assigned mixers
//
pub const MAX_NID_PATH_DEPTH: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nid_path {
    pub depth: c_int,
    pub path: [hda_nid_t; MAX_NID_PATH_DEPTH],
    pub idx: [c_uchar; MAX_NID_PATH_DEPTH],
    pub multi: [c_uchar; MAX_NID_PATH_DEPTH],
    pub /: *mut *mut unsigned int ctls[NID_PATH_NUM_CTLS]; / NID_PATH_XXX_CTL,
    pub /: *mut *mut bool active:1; / activated by driver,
    pub /: *mut *mut bool pin_enabled:1; / pins are enabled,
    pub /: *mut *mut bool pin_fixed:1; / path with fixed pin,
    pub /: *mut *mut bool stream_enabled:1; / stream is active,
}

// mic/line-in auto switching entry
pub const MAX_AUTO_MIC_PINS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct automic_entry {
    pub /: *mut *mut hda_nid_t pin; / pin,
    pub /: *mut *mut int idx; / imux index, -1 = invalid,
    pub /: *mut *mut *mut unsigned int attr; / pin attribute (INPUT_PIN_ATTR_),
}

// active stream id
// PCM hook action
// DAC assignment badness table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct badness_table {
    pub /: *mut *mut int no_primary_dac; / no primary DAC,
    pub /: *mut *mut int no_dac; / no secondary DACs,
    pub /: *mut *mut int shared_primary; / primary DAC is shared with main output,
    pub /: *mut *mut int shared_surr; / secondary DAC shared with main or primary,
    pub /: *mut *mut int shared_clfe; / third DAC shared with main or primary,
    pub /: *mut *mut int shared_surr_main; / secondary DAC sahred with main/DAC0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_gen_spec {
    pub /: *mut *mut char stream_name_analog[32]; / analog PCM stream,
    pub stream_analog_playback: *const hda_pcm_stream,
    pub stream_analog_capture: *const hda_pcm_stream,
    pub /: *mut *mut char stream_name_alt_analog[32]; / alternative analog PCM stream,
    pub stream_analog_alt_playback: *const hda_pcm_stream,
    pub stream_analog_alt_capture: *const hda_pcm_stream,
    pub /: *mut *mut char stream_name_digital[32]; / digital PCM stream,
    pub stream_digital_playback: *const hda_pcm_stream,
    pub stream_digital_capture: *const hda_pcm_stream,
// PCM
    pub active_streams: c_uint,
    pub pcm_mutex: mutex,
// playback
    pub set-up: *mut *mut hda_multi_out multiout; / playback,
// max_channels, dacs must be set
// dig_out_nid and hp_nid are optional
//
    pub alt_dac_nid: hda_nid_t,
    pub /: *mut *mut hda_nid_t follower_dig_outs[3]; / optional - for auto-parsing,
    pub dig_out_type: c_int,
// capture
    pub num_adc_nids: c_uint,
    pub adc_nids: [hda_nid_t; AUTO_CFG_MAX_INS],
    pub /: *mut *mut hda_nid_t dig_in_nid; / digital-in NID; optional,
    pub /: *mut *mut hda_nid_t mixer_nid; / analog-mixer NID,
    pub /: *mut *mut hda_nid_t mixer_merge_nid; / aamix merge-point NID (optional),
    pub input_labels: [*const c_char; HDA_MAX_NUM_INPUTS],
    pub input_label_idxs: [c_int; HDA_MAX_NUM_INPUTS],
// capture setup for dynamic dual-adc switch
    pub cur_adc: hda_nid_t,
    pub cur_adc_stream_tag: c_uint,
    pub cur_adc_format: c_uint,
// capture source
    pub input_mux: hda_input_mux,
    pub cur_mux: [c_uint; 3],
// channel model
// min_channel_count contains the minimum channel count for primary
// outputs.  When multi_ios is set, the channels can be configured
// between min_channel_count and (min_channel_count + multi_ios * 2).
//
// ext_channel_count contains the current channel count of the primary
// out.  This varies in the range above.
//
// Meanwhile, const_channel_count is the channel count for all outputs
// including headphone and speakers.  It's a constant value, and the
// PCM is set up as max(ext_channel_count, const_channel_count).
//
    pub /: *mut *mut int min_channel_count; / min. channel count for primary out,
    pub /: *mut *mut int ext_channel_count; / current channel count for primary,
    pub /: *mut *mut int const_channel_count; / channel count for all,
// PCM information
    pub /: *mut *mut *mut hda_pcm pcm_rec[3]; / used in build_pcms(),
// dynamic controls, init_verbs and input_mux
    pub autocfg: auto_pin_cfg,
    pub kctls: snd_array,
    pub private_dac_nids: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub imux_pins: [hda_nid_t; HDA_MAX_NUM_INPUTS],
    pub dyn_adc_idx: [c_uint; HDA_MAX_NUM_INPUTS],
// shared hp/mic
    pub shared_mic_vref_pin: hda_nid_t,
    pub hp_mic_pin: hda_nid_t,
    pub hp_mic_mux_idx: c_int,
// DAC/ADC lists
    pub num_all_dacs: c_int,
    pub all_dacs: [hda_nid_t; 16],
    pub num_all_adcs: c_int,
    pub all_adcs: [hda_nid_t; AUTO_CFG_MAX_INS],
// path list
    pub paths: snd_array,
// path indices
    pub out_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub hp_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub speaker_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub aamix_out_paths: [c_int; 3],
    pub digout_paths: [c_int; AUTO_CFG_MAX_OUTS],
    pub input_paths: [c_int; HDA_MAX_NUM_INPUTS][AUTO_CFG_MAX_INS],
    pub loopback_paths: [c_int; HDA_MAX_NUM_INPUTS],
    pub loopback_merge_path: c_int,
    pub digin_path: c_int,
// auto-mic stuff
    pub am_num_entries: c_int,
    pub am_entry: [automic_entry; MAX_AUTO_MIC_PINS],
// for pin sensing
// current status; set in hda_generic.c
    pub hp_jack_present:1: c_uint,
    pub line_jack_present:1: c_uint,
    pub /: *mut *mut unsigned int speaker_muted:1; / current status of speaker mute,
    pub /: *mut *mut unsigned int line_out_muted:1; / current status of LO mute,
// internal states of automute / autoswitch behavior
    pub auto_mic:1: c_uint,
    pub /: *mut *mut unsigned int automute_speaker:1; / automute speaker outputs,
    pub /: *mut *mut unsigned int automute_lo:1; / automute LO outputs,
// capabilities detected by parser
    pub /: *mut *mut unsigned int detect_hp:1; / Headphone detection enabled,
    pub /: *mut *mut unsigned int detect_lo:1; / Line-out detection enabled,
    pub /: *mut *mut unsigned int automute_speaker_possible:1; / there are speakers and either LO or HP,
    pub /: *mut *mut unsigned int automute_lo_possible:1; / there are line outs and HP,
// additional parameters set by codec drivers
    pub /: *mut *mut unsigned int master_mute:1; / master mute over all,
    pub /: *mut *mut unsigned int keep_vref_in_automute:1; / Don't clear VREF in automute,
    pub /: *mut *mut unsigned int line_in_auto_switch:1; / allow line-in auto switch,
    pub /: *mut *mut unsigned int auto_mute_via_amp:1; / auto-mute via amp instead of pinctl,
// parser behavior flags; set before snd_hda_gen_parse_auto_config()
    pub /: *mut *mut unsigned int suppress_auto_mute:1; / suppress input jack auto mute,
    pub /: *mut *mut unsigned int suppress_auto_mic:1; / suppress input jack auto switch,
// other parse behavior flags
    pub /: *mut *mut unsigned int need_dac_fix:1; / need to limit DACs for multi channels,
    pub /: *mut *mut unsigned int hp_mic:1; / Allow HP as a mic-in,
    pub /: *mut *mut unsigned int suppress_hp_mic_detect:1; / Don't detect HP/mic,
    pub /: *mut *mut unsigned int no_primary_hp:1; / Don't prefer HP pins to speaker pins,
    pub /: *mut *mut unsigned int no_multi_io:1; / Don't try multi I/O config,
    pub /: *mut *mut unsigned int multi_cap_vol:1; / allow multiple capture xxx volumes,
    pub /: *mut *mut unsigned int inv_dmic_split:1; / inverted dmic w/a for conexant,
    pub /: *mut *mut unsigned int own_eapd_ctl:1; / set EAPD by own function,
    pub /: *mut *mut unsigned int keep_eapd_on:1; / don't turn off EAPD automatically,
    pub /: *mut *mut unsigned int vmaster_mute_led:1; / add SPK-LED flag to vmaster mute switch,
    pub /: *mut *mut unsigned int mic_mute_led:1; / add MIC-LED flag to capture mute switch,
    pub /: *mut *mut unsigned int indep_hp:1; / independent HP supported,
    pub /: *mut *mut unsigned int prefer_hp_amp:1; / enable HP amp for speaker if any,
    pub /: *mut *mut unsigned int add_stereo_mix_input:2; / add aamix as a capture src,
    pub /: *mut *mut unsigned int add_jack_modes:1; / add i/o jack mode enum ctls,
    pub /: *mut *mut unsigned int power_down_unused:1; / power down unused widgets,
    pub /: *mut *mut unsigned int dac_min_mute:1; / minimal = mute for DACs,
    pub /: *mut *mut unsigned int suppress_vmaster:1; / don't create vmaster kctls,
// other internal flags
    pub /: *mut *mut unsigned int no_analog:1; / digital I/O only,
    pub /: *mut *mut unsigned int dyn_adc_switch:1; / switch ADCs (for ALC275),
    pub /: *mut *mut unsigned int indep_hp_enabled:1; / independent HP enabled,
    pub have_aamix_ctl:1: c_uint,
    pub hp_mic_jack_modes:1: c_uint,
    pub /: *mut *mut unsigned int skip_verbs:1; / don't apply verbs at snd_hda_gen_init(),
// additional mute flags (only effective with auto_mute_via_amp=1)
    pub mute_bits: u64,
// bitmask for skipping volume controls
    pub out_vol_mask: u64,
// badness tables for output path evaluations
    pub main_out_badness: *const badness_table,
    pub extra_out_badness: *const badness_table,
// preferred pin/DAC pairs; an array of paired NIDs
    pub preferred_dacs: *const hda_nid_t,
// loopback mixing mode
    pub aamix_mode: bool,
// digital beep
    pub beep_nid: hda_nid_t,
// for virtual master
    pub vmaster_nid: hda_nid_t,
    pub vmaster_tlv: [c_uint; 4],
    pub vmaster_mute: hda_vmaster_mute_hook,
    pub loopback: hda_loopback_check,
    pub loopback_list: snd_array,
// multi-io
    pub multi_ios: c_int,
    pub multi_io: [hda_multi_io; 4],
// hooks
    pub codec): *mut *mut void (init_hook)(struct hda_codec,
    pub codec): *mut *mut void (automute_hook)(struct hda_codec,
    pub ucontrol): *mut snd_ctl_elem_value,
// PCM hooks
    pub action): c_int,
    pub action): c_int,
// automute / autoswitch hooks
    pub cb): *mut hda_jack_callback,
    pub cb): *mut hda_jack_callback,
    pub cb): *mut hda_jack_callback,
// leds
    pub led_cdevs: [*mut led_classdev; NUM_AUDIO_LEDS],
}

// values for add_stereo_mix_input flag
extern "C" {
    pub fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_init(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_remove(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_get_path_idx(codec: *mut hda_codec, path: *mut nid_path) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
}
// standard jack event callbacks
extern "C" {
    pub fn snd_hda_gen_update_outputs(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
}
extern "C" {
    pub fn snd_hda_gen_fix_pin_power(codec: *mut hda_codec, pin: hda_nid_t) -> c_int;
}
extern "C" {
    pub fn snd_hda_gen_shutup_speakers(codec: *mut hda_codec) -> bool;
}
