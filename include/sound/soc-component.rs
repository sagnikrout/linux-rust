//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-component.h
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
// soc-component.h
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// Component probe and remove ordering levels for components with runtime
// dependencies.
//

pub const SND_SOC_COMP_ORDER_NORMAL: c_int = 0;
pub const SND_SOC_COMP_ORDER_LATE: c_int = 1;
pub const SND_SOC_COMP_ORDER_LAST: c_int = 2;

// component interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compress_ops {
    pub stream): *mut snd_compr_stream,
    pub stream): *mut snd_compr_stream,
    pub params): *mut snd_compr_params,
    pub params): *mut snd_codec,
    pub metadata): *mut snd_compr_metadata,
    pub metadata): *mut snd_compr_metadata,
    pub cmd): *mut *mut snd_compr_stream stream, int,
    pub tstamp): *mut snd_compr_tstamp64,
    pub count): usize,
    pub vma): *mut vm_area_struct,
    pub bytes): *mut *mut snd_compr_stream stream, size_t,
    pub caps): *mut snd_compr_caps,
    pub codec): *mut snd_compr_codec_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
// Default control and setup, added after probe() is run
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub component): *mut *mut int (probe)(struct snd_soc_component,
    pub component): *mut *mut int (fixup_controls)(struct snd_soc_component,
    pub component): *mut *mut void (remove)(struct snd_soc_component,
    pub component): *mut *mut int (suspend)(struct snd_soc_component,
    pub component): *mut *mut int (resume)(struct snd_soc_component,
    pub reg): c_uint,
    pub val): unsigned int reg, unsigned int,
// pcm creation and destruction
    pub rtd): *mut snd_soc_pcm_runtime,
    pub pcm): *mut snd_pcm,
// component wide operations
    pub dir): int clk_id, int source, unsigned int freq, int,
    pub freq_out): int source, unsigned int freq_in, unsigned int,
    pub data): *mut *mut snd_soc_jack jack, void,
    pub component): *mut *mut int (get_jack_type)(struct snd_soc_component,
// DT
    pub dai_name): *const c_char,
    pub endpoint): *mut device_node,
    pub subseq): snd_soc_dapm_type type, int,
    pub event): *mut *mut *mut int (stream_event)(struct snd_soc_component component, int,
    pub level): snd_soc_bias_level,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub arg): *mut unsigned int cmd, void,
    pub params): *mut snd_pcm_hw_params,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub cmd): *mut *mut snd_pcm_substream substream, int,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub audio_tstamp_report): *mut snd_pcm_audio_tstamp_report,
    pub bytes): c_ulong,
    pub offset): c_ulong,
    pub vma): *mut vm_area_struct,
    pub substream): *mut snd_pcm_substream,
    pub substream): *mut snd_pcm_substream,
    pub compress_ops: *const snd_compress_ops,
// probe ordering - for components with runtime dependencies
    pub probe_order: c_int,
    pub remove_order: c_int,
//
// soc_pcm_trigger() start/stop sequence.
// see also
// snd_soc_dai_link
// soc_pcm_trigger()
//
    pub trigger_start: snd_soc_trigger_order,
    pub trigger_stop: snd_soc_trigger_order,
//
// signal if the module handling the component should not be removed
// if a pcm is open. Setting this would prevent the module
// refcount being incremented in probe() but allow it be incremented
// when a pcm is opened and decremented when it is closed.
//
    pub module_get_upon_open:1: c_uint,
// bits
    pub idle_bias_on:1: c_uint,
    pub suspend_bias_off:1: c_uint,
    pub /: *mut *mut unsigned int use_pmdown_time:1; / care pmdown_time at stop,
//
// Indicates that the component does not care about the endianness of
// PCM audio data and the core will ensure that both LE and BE variants
// of each used format are present. Typically this is because the
// component sits behind a bus that abstracts away the endian of the
// original data, ie. one for which the transmission endian is defined
// (I2S/SLIMbus/SoundWire), or the concept of endian doesn't exist (PDM,
// analogue).
//
    pub endianness:1: c_uint,
    pub legacy_dai_naming:1: c_uint,
// this component uses topology and ignore machine driver FEs
    pub ignore_machine: *const c_char,
    pub topology_name_prefix: *const c_char,
    pub params): *mut snd_pcm_hw_params,
    pub /: *mut *mut bool use_dai_pcm_id; / use DAI link PCM ID as PCM device number,
    pub /: *mut *mut int be_pcm_base; / base device ID for all BE PCMs,
    pub debugfs_prefix: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_component {
    pub name: *const c_char,
    pub name_prefix: *const c_char,
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub active: c_uint,
    pub /: *mut *mut unsigned int suspended:1; / is in suspend PM state,
    pub list: list_head,
    pub /: *mut *mut list_head card_aux_list; / for auxiliary bound components,
    pub card_list: list_head,
    pub card_device_link: *mut device_link,
    pub driver: *const snd_soc_component_driver,
    pub dai_list: list_head,
    pub num_dai: c_int,
    pub regmap: *mut regmap,
    pub io_mutex: mutex,
// attached dynamic objects
    pub dobj_list: list_head,
//
// DO NOT use any of the fields below in drivers, they are temporary and
// are going to be removed again soon. If you use them in driver code
// the driver will be marked as BROKEN when these fields are removed.
//
    pub dapm: *mut snd_soc_dapm_context,
// machine specific init
    pub component): *mut *mut int (init)(struct snd_soc_component,
// function mark
    pub mark_module: *mut c_void,
    pub mark_open: *mut snd_pcm_substream,
    pub mark_hw_params: *mut snd_pcm_substream,
    pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_open: *mut snd_compr_stream,
    pub mark_pm: *mut c_void,
    pub debugfs_root: *mut dentry,
// Component private data
    pub priv: *mut c_void,
}

//
// snd_soc_component_to_dapm() - Returns the DAPM context associated with a
// component
// @component: The component for which to get the DAPM context
//
// snd_soc_component_cache_sync() - Sync the register cache with the hardware
// @component: COMPONENT to sync
//
// Note: This function will call regcache_sync()
//
extern "C" {
    pub fn regcache_sync(_arg: component->regmap) -> return;
}
extern "C" {
    pub fn snd_soc_component_set_name(component: *mut snd_soc_component, name: *const c_char);
}
extern "C" {
    pub fn snd_soc_component_set_priv(component: *mut snd_soc_component, priv: *mut c_void);
}
extern "C" {
    pub fn snd_soc_component_init(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_is_dummy(component: *mut snd_soc_component) -> c_int;
}
// component IO
extern "C" {
    pub fn snd_soc_component_async_complete(component: *mut snd_soc_component);
}
// component wide operations
extern "C" {
    pub fn snd_soc_component_get_jack_type(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_regmap_val_bytes(component: *mut snd_soc_component) -> c_int;
}

extern "C" {
    pub fn snd_soc_component_exit_regmap(component: *mut snd_soc_component);
}

// Macro flag: #define snd_soc_component_module_get_when_probe(component)\

extern "C" {
    pub fn dev_get_drvdata(_arg: c->dev) -> return;
}
// component controls
// component driver ops
extern "C" {
    pub fn snd_soc_component_suspend(component: *mut snd_soc_component);
}
extern "C" {
    pub fn snd_soc_component_resume(component: *mut snd_soc_component);
}
extern "C" {
    pub fn snd_soc_component_is_suspended(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_probe(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_fixup_controls(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_remove(component: *mut snd_soc_component);
}
extern "C" {
    pub fn snd_soc_component_compr_trigger(cstream: *mut snd_compr_stream, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_component_compr_ack(cstream: *mut snd_compr_stream, bytes: usize) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_component_pointer(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_component_sync_stop(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_component_new(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_component_free(rtd: *mut snd_soc_pcm_runtime);
}
extern "C" {
    pub fn snd_soc_pcm_component_prepare(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_component_ack(substream: *mut snd_pcm_substream) -> c_int;
}
