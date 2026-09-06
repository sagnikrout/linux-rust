//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-dapm.h
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
//
// linux/sound/soc-dapm.h -- ALSA SoC Dynamic Audio Power Management
//
// Author:	Liam Girdwood
// Created:	Aug 11th 2005
// Copyright:	Wolfson Microelectronics. PLC.
//

// widget has no PM register bit

//
// SoC dynamic audio power management
//
// We can have up to 4 power domains
// 1. Codec domain - VREF, VMID
// Usually controlled at codec probe/remove, although can be set
// at stream time if power is not needed for sidetone, etc.
// 2. Platform/Machine domain - physically connected inputs and outputs
// Is platform/machine and user action specific, is set in the machine
// driver and by userspace e.g when HP are inserted
// 3. Path domain - Internal codec path mixers
// Are automatically set when mixer and mux settings are
// changed by the user.
// 4. Stream domain - DAC's and ADC's.
// Enabled when stream playback/capture is started.
//
// codec domain

// platform domain

// path domain

// DEPRECATED: use SND_SOC_DAPM_SUPPLY

// Simplified versions of above macros, assuming wncontrols = ARRAY_SIZE(wcontrols)

// path domain with event - event handler must return 0 for success

// additional sequencing control within an event type

// Simplified versions of above macros, assuming wncontrols = ARRAY_SIZE(wcontrols)

// events that are pre and post DAPM

// stream domain

// generic widgets

// dapm kcontrol types

// dapm stream operations
pub const SND_SOC_DAPM_STREAM_NOP: c_uint = 0x0;
pub const SND_SOC_DAPM_STREAM_START: c_uint = 0x1;
pub const SND_SOC_DAPM_STREAM_STOP: c_uint = 0x2;
pub const SND_SOC_DAPM_STREAM_SUSPEND: c_uint = 0x4;
pub const SND_SOC_DAPM_STREAM_RESUME: c_uint = 0x8;
pub const SND_SOC_DAPM_STREAM_PAUSE_PUSH: c_uint = 0x10;
pub const SND_SOC_DAPM_STREAM_PAUSE_RELEASE: c_uint = 0x20;
// dapm event types
pub const SND_SOC_DAPM_PRE_PMU: c_uint = 0x1	/* before widget power up */;
pub const SND_SOC_DAPM_POST_PMU: c_uint = 0x2	/* after  widget power up */;
pub const SND_SOC_DAPM_PRE_PMD: c_uint = 0x4	/* before widget power down */;
pub const SND_SOC_DAPM_POST_PMD: c_uint = 0x8	/* after  widget power down */;
pub const SND_SOC_DAPM_PRE_REG: c_uint = 0x10	/* before audio path setup */;
pub const SND_SOC_DAPM_POST_REG: c_uint = 0x20	/* after  audio path setup */;
pub const SND_SOC_DAPM_WILL_PMU: c_uint = 0x40	/* called at start of sequence */;
pub const SND_SOC_DAPM_WILL_PMD: c_uint = 0x80	/* called at start of sequence */;

// convenience event type detection

// regulator widget flags
pub const SND_SOC_DAPM_REGULATOR_BYPASS: c_uint = 0x1	/* bypass when disabled */;
//
// Bias levels
//
// @ON:      Bias is fully on for audio playback and capture operations.
// @PREPARE: Prepare for audio operations. Called before DAPM switching for
// stream start and stop operations.
// @STANDBY: Low power standby state when no playback/capture operations are
// in progress. NOTE: The transition time between STANDBY and ON
// should be as fast as possible and no longer than 10ms.
// @OFF:     Power Off. No restrictions on transition times.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

// dapm widget types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_type {
    snd_soc_dapm_input = 0,		/* input pin */
    snd_soc_dapm_output,		/* output pin */
    snd_soc_dapm_mux,		/* selects 1 analog signal from many inputs */
    snd_soc_dapm_mux_named_ctl,	/* mux with named controls */
    snd_soc_dapm_demux,		/* connects the input to one of multiple outputs */
    snd_soc_dapm_mixer,		/* mixes several analog signals together */
    snd_soc_dapm_mixer_named_ctl,	/* mixer with named controls */
    snd_soc_dapm_pga,		/* programmable gain/attenuation (volume) */
    snd_soc_dapm_out_drv,		/* output driver */
    snd_soc_dapm_adc,		/* analog to digital converter */
    snd_soc_dapm_dac,		/* digital to analog converter */
    snd_soc_dapm_micbias,		/* microphone bias (power) - DEPRECATED: use snd_soc_dapm_supply */
    snd_soc_dapm_mic,		/* microphone */
    snd_soc_dapm_hp,		/* headphones */
    snd_soc_dapm_spk,		/* speaker */
    snd_soc_dapm_line,		/* line input/output */
    snd_soc_dapm_switch,		/* analog switch */
    snd_soc_dapm_vmid,		/* codec bias/vmid - to minimise pops */
    snd_soc_dapm_pre,		/* machine specific pre widget - exec first */
    snd_soc_dapm_post,		/* machine specific post widget - exec last */
    snd_soc_dapm_supply,		/* power/clock supply */
    snd_soc_dapm_pinctrl,		/* pinctrl */
    snd_soc_dapm_regulator_supply,	/* external regulator */
    snd_soc_dapm_clock_supply,	/* external clock */
    snd_soc_dapm_aif_in,		/* audio interface input */
    snd_soc_dapm_aif_out,		/* audio interface output */
    snd_soc_dapm_siggen,		/* signal generator */
    snd_soc_dapm_sink,
    snd_soc_dapm_dai_in,		/* link to DAI structure */
    snd_soc_dapm_dai_out,
    snd_soc_dapm_dai_link,		/* link between two DAI structures */
    snd_soc_dapm_kcontrol,		/* Auto-disabled kcontrol */
    snd_soc_dapm_buffer,		/* DSP/CODEC internal buffer */
    snd_soc_dapm_scheduler,		/* DSP/CODEC internal scheduler */
    snd_soc_dapm_effect,		/* DSP/CODEC effect component */
    snd_soc_dapm_src,		/* DSP/CODEC SRC component */
    snd_soc_dapm_asrc,		/* DSP/CODEC ASRC component */
    snd_soc_dapm_encoder,		/* FW/SW audio encoder component */
    snd_soc_dapm_decoder,		/* FW/SW audio decoder component */

// Don't edit below this line
    SND_SOC_DAPM_TYPE_COUNT
}

//
// DAPM audio route definition.
//
// Defines an audio route originating at source via control and finishing
// at sink.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
// Note: currently only supported for links where source is a supply
    pub sink): *mut snd_soc_dapm_widget,
    pub dobj: snd_soc_dobj,
}

// dapm audio path between two widgets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_path {
    pub name: *const c_char,
//
// source (input) and sink (output) widgets
// The union is for convience, since it is a lot nicer to type
// p->source, rather than p->node[SND_SOC_DAPM_DIR_IN]
//
    pub source: *mut snd_soc_dapm_widget,
    pub sink: *mut snd_soc_dapm_widget,
}

// status
// dapm widget
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub id: snd_soc_dapm_type,
    pub /: *const *const *const char name; / widget name,
    pub /: *const *const *const char sname; / stream name,
    pub list: list_head,
    pub dapm: *mut snd_soc_dapm_context,
    pub /: *mut *mut *mut void priv; / widget specific data,
    pub /: *mut *mut *mut regulator regulator; / attached regulator,
    pub /: *mut *mut *mut pinctrl pinctrl; / attached pinctrl,
// dapm control
    pub /: *mut *mut int reg; / negative reg = no direct dapm,
    pub /: *mut *mut unsigned char shift; / bits to shift,
    pub /: *mut *mut unsigned int mask; / non-shifted mask,
    pub /: *mut *mut unsigned int on_val; / on state value,
    pub /: *mut *mut unsigned int off_val; / off state value,
    pub /: *mut *mut unsigned char power:1; / block power status,
    pub /: *mut *mut unsigned char active:1; / active stream on DAC, ADC's,
    pub /: *mut *mut unsigned char connected:1; / connected codec pin,
    pub /: *mut *mut unsigned char new:1; / cnew complete,
    pub /: *mut *mut unsigned char force:1; / force state,
    pub /: *mut *mut unsigned char ignore_suspend:1; / kept enabled over suspend,
    pub /: *mut *mut unsigned char new_power:1; / power from this run,
    pub /: *mut *mut unsigned char power_checked:1; / power checked this run,
    pub /: *mut *mut unsigned char is_supply:1; / Widget is a supply type widget,
    pub /: *mut *mut unsigned char is_ep:2; / Widget is a endpoint type widget,
    pub /: *mut *mut unsigned char no_wname_in_kcontrol_name:1; / No widget name prefix in kcontrol name,
    pub /: *mut *mut int subseq; / sort within widget type,
    pub w): *mut *mut int (power_check)(struct snd_soc_dapm_widget,
// external events
    pub /: *mut *mut unsigned short event_flags; / flags to specify event types,
    pub int): *mut *mut *mut *mut int (event)(struct snd_soc_dapm_widget, struct snd_kcontrol ,,
// kcontrols that relate to this widget
    pub num_kcontrols: c_int,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub kcontrols: *mut snd_kcontrol,
    pub dobj: snd_soc_dobj,
// widget input and output edges
    pub edges: [list_head; 2],
// used during DAPM updates
    pub work_list: list_head,
    pub power_list: list_head,
    pub dirty: list_head,
    pub endpoints: [c_int; 2],
    pub clk: *mut clk,
    pub channel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_update {
    pub kcontrol: *mut snd_kcontrol,
    pub reg: c_int,
    pub mask: c_int,
    pub val: c_int,
    pub reg2: c_int,
    pub mask2: c_int,
    pub val2: c_int,
    pub has_second_set: bool,
}

// A list of widgets associated with an object, typically a snd_kcontrol
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget_list {
    pub num_widgets: c_int,
    pub __counted_by(num_widgets): *mut *mut snd_soc_dapm_widget widgets[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_stats {
    pub power_checks: c_int,
    pub path_checks: c_int,
    pub neighbour_checks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_pinctrl_priv {
    pub active_state: *const c_char,
    pub sleep_state: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_direction {
    SND_SOC_DAPM_DIR_IN,
    SND_SOC_DAPM_DIR_OUT
}

extern "C" {
    pub fn snd_soc_dapm_regulator_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_clock_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_pinctrl_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
}
// dapm controls
extern "C" {
    pub fn snd_soc_dapm_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_info_pin_switch(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_pin_switch(kcontrol: *mut snd_kcontrol, uncontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_put_pin_switch(kcontrol: *mut snd_kcontrol, uncontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_free_widget(w: *mut snd_soc_dapm_widget);
}
extern "C" {
    pub fn snd_soc_dapm_link_dai_widgets(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_connect_dai_link_widgets(card: *mut snd_soc_card);
}
extern "C" {
    pub fn snd_soc_dapm_ignore_suspend_widgets(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_widget_name_cmp(widget: *mut snd_soc_dapm_widget, s: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_idle_bias(dapm: *mut snd_soc_dapm_context) -> bool;
}
extern "C" {
    pub fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, on: bool);
}
// dapm path setup
extern "C" {
    pub fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_free(dapm: *mut snd_soc_dapm_context);
}
extern "C" {
    pub fn snd_soc_dapm_free_widget(w: *mut snd_soc_dapm_widget);
}
// dapm events
extern "C" {
    pub fn snd_soc_dapm_stream_event(rtd: *mut snd_soc_pcm_runtime, stream: c_int, event: c_int);
}
extern "C" {
    pub fn snd_soc_dapm_stream_stop(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
}
extern "C" {
    pub fn snd_soc_dapm_shutdown(card: *mut snd_soc_card);
}
// external DAPM widget events
// dapm sys fs - used by the core
extern "C" {
    pub fn snd_soc_dapm_debugfs_init(dapm: *mut snd_soc_dapm_context, parent: *mut dentry);
}
extern "C" {
    pub fn snd_soc_dapm_debugfs_pop_time(parent: *mut dentry);
}
// dapm audio pin control and status
extern "C" {
    pub fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_pin_status(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_ignore_suspend(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_pin_has_prefix(card: *mut snd_soc_card, pin: *const c_char) -> bool;
}
extern "C" {
    pub fn snd_soc_dapm_mark_endpoints_dirty(card: *mut snd_soc_card);
}
// dapm path query
extern "C" {
    pub fn snd_soc_dapm_dai_free_widgets(list: *mut snd_soc_dapm_widget_list);
}
extern "C" {
    pub fn snd_soc_dapm_kcontrol_get_value(kcontrol: *const snd_kcontrol) -> c_uint;
}
extern "C" {
    pub fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
}
extern "C" {
    pub fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
}
extern "C" {
    pub fn snd_soc_dapm_init_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level);
}

//
// snd_soc_dapm_widget_for_each_path - Iterates over all paths in the
// specified direction of a widget
// @w: The widget
// @dir: Whether to iterate over the paths where the specified widget is the
// incoming or outgoing widgets
// @p: The path iterator variable
//

//
// snd_soc_dapm_widget_for_each_path_safe - Iterates over all paths in the
// specified direction of a widget
// @w: The widget
// @dir: Whether to iterate over the paths where the specified widget is the
// incoming or outgoing widgets
// @p: The path iterator variable
// @next_p: Temporary storage for the next path
//
// This function works like snd_soc_dapm_widget_for_each_path, expect that
// it is safe to remove the current path from the list while iterating
//

//
// snd_soc_dapm_widget_for_each_sink_path - Iterates over all paths leaving a
// widget
// @w: The widget
// @p: The path iterator variable
//

//
// snd_soc_dapm_widget_for_each_source_path - Iterates over all paths leading to
// a widget
// @w: The widget
// @p: The path iterator variable
//

