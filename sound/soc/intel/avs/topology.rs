//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/topology.h
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
// Copyright(c) 2021 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub version: u32,
    pub comp: *mut snd_soc_component,
    pub libs: *mut avs_tplg_library,
    pub num_libs: u32,
    pub fmts: *mut avs_audio_format,
    pub num_fmts: u32,
    pub modcfgs_base: *mut avs_tplg_modcfg_base,
    pub num_modcfgs_base: u32,
    pub modcfgs_ext: *mut avs_tplg_modcfg_ext,
    pub num_modcfgs_ext: u32,
    pub pplcfgs: *mut avs_tplg_pplcfg,
    pub num_pplcfgs: u32,
    pub bindings: *mut avs_tplg_binding,
    pub num_bindings: u32,
    pub condpath_tmpls: *mut avs_tplg_path_template,
    pub num_condpath_tmpls: u32,
    pub init_configs: *mut avs_tplg_init_config,
    pub num_init_configs: u32,
    pub nhlt_configs: *mut avs_tplg_nhlt_config,
    pub num_nhlt_configs: u32,
    pub path_tmpl_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_library {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

// Matches header of struct avs_mod_cfg_base.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_modcfg_base {
    pub cpc: u32,
    pub ibs: u32,
    pub obs: u32,
    pub is_pages: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_pin_format {
    pub pin_index: u32,
    pub iobs: u32,
    pub fmt: *mut avs_audio_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_modcfg_ext {
    pub type: guid_t,
    pub num_input_pins: u16,
    pub num_output_pins: u16,
    pub pin_fmts: *mut avs_tplg_pin_format,
    pub generic: },
    pub out_fmt: *mut avs_audio_format,
    pub /: *mut *mut *mut avs_audio_format blob_fmt; / optional override,
    pub feature_mask: u32,
    pub vindex: avs_virtual_index,
    pub dma_type: u32,
    pub dma_buffer_size: u32,
    pub copier: },
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
    pub wake_tick_period: u32,
    pub vindex: avs_virtual_index,
    pub dma_type: u32,
    pub dma_buffer_size: u32,
    pub /: *mut *mut *mut avs_audio_format blob_fmt; / optional override,
    pub whm: },
    pub out_channel_config: u32,
    pub coefficients_select: u32,
    pub coefficients: [i32; AVS_COEFF_CHANNELS_MAX],
    pub channel_map: u32,
    pub updown_mix: },
    pub out_freq: u32,
    pub src: },
    pub out_freq: u32,
    pub mode: u8,
    pub disable_jitter_buffer: u8,
    pub asrc: },
    pub cpc_lp_mode: u32,
    pub wov: },
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
    pub cpc_lp_mode: u32,
    pub aec: },
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
    pub mux: },
    pub out_fmt: *mut avs_audio_format,
    pub micsel: },
    pub target_volume: u32,
    pub curve_type: u32,
    pub curve_duration: u32,
    pub peakvol: },
}

// Specifies path behaviour during PCM ->trigger(START) command.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_tplg_trigger {
    AVS_TPLG_TRIGGER_AUTO = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_pplcfg {
    pub req_size: u16,
    pub priority: u8,
    pub lp: bool,
    pub attributes: u16,
    pub trigger: avs_tplg_trigger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_binding {
    pub target_tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub target_path_tmpl_id: u32,
    pub target_ppl_id: u32,
    pub target_mod_id: u32,
    pub target_mod_pin: u8,
    pub mod_id: u32,
    pub mod_pin: u8,
    pub is_sink: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_path_template_id {
    pub id: u32,
    pub tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_path_template {
    pub id: u32,
    pub w: *mut snd_soc_dapm_widget,
// Conditional path.
    pub source: avs_tplg_path_template_id,
    pub sink: avs_tplg_path_template_id,
    pub path_list: list_head,
    pub owner: *mut avs_tplg,
// Driver path templates management.
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_init_config {
    pub id: u32,
    pub param: u8,
    pub length: usize,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_nhlt_config {
    pub id: u32,
    pub blob: *mut acpi_nhlt_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_path {
    pub id: u32,
// Path format requirements.
    pub fe_fmt: *mut avs_audio_format,
    pub be_fmt: *mut avs_audio_format,
// Condpath path-variant requirements.
    pub source_path_id: u32,
    pub sink_path_id: u32,
    pub ppl_list: list_head,
    pub owner: *mut avs_tplg_path_template,
// Path template path-variants management.
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_pipeline {
    pub id: u32,
    pub cfg: *mut avs_tplg_pplcfg,
    pub bindings: *mut avs_tplg_binding,
    pub num_bindings: u32,
    pub mod_list: list_head,
    pub owner: *mut avs_tplg_path,
// Path pipelines management.
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tplg_module {
    pub id: u32,
    pub cfg_base: *mut avs_tplg_modcfg_base,
    pub in_fmt: *mut avs_audio_format,
    pub core_id: u8,
    pub domain: u8,
    pub cfg_ext: *mut avs_tplg_modcfg_ext,
    pub ctl_id: u32,
    pub num_config_ids: u32,
    pub config_ids: *mut u32,
    pub nhlt_config: *mut avs_tplg_nhlt_config,
    pub owner: *mut avs_tplg_pipeline,
// Pipeline modules management.
    pub node: list_head,
}

extern "C" {
    pub fn avs_load_topology(comp: *mut snd_soc_component, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn avs_remove_topology(comp: *mut snd_soc_component) -> c_int;
}
