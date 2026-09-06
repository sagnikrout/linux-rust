//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-topology.h
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
// linux/sound/soc-topology.h -- ALSA SoC Firmware Controls and DAPM
//
// Copyright (C) 2012 Texas Instruments Inc.
// Copyright (C) 2015 Intel Corporation.
//
// Simple file API to load FW that includes mixers, coefficients, DAPM graphs,
// algorithms, equalisers, DAIs, widgets, FE caps, BE caps, codec link caps etc.
//

// dynamic object type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dobj_type {
    SND_SOC_DOBJ_NONE		= 0,	/* object is not dynamic */
    SND_SOC_DOBJ_MIXER,
    SND_SOC_DOBJ_BYTES,
    SND_SOC_DOBJ_ENUM,
    SND_SOC_DOBJ_GRAPH,
    SND_SOC_DOBJ_WIDGET,
    SND_SOC_DOBJ_DAI_LINK,
    SND_SOC_DOBJ_PCM,
    SND_SOC_DOBJ_CODEC_LINK,
    SND_SOC_DOBJ_BACKEND_LINK,
}

// dynamic control object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dobj_control {
    pub kcontrol: *mut snd_kcontrol,
    pub dtexts: *mut c_char,
    pub dvalues: *mut c_ulong,
}

// dynamic widget object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dobj_widget {
    pub /: *mut *mut *mut unsigned int kcontrol_type; / kcontrol type: mixer, enum, bytes,
}

// generic dynamic object - all dynamic objects belong to this struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dobj {
    pub type: snd_soc_dobj_type,
    pub /: *mut *mut unsigned int index; / objects can belong in different groups,
    pub list: list_head,
    pub dobj): *mut *mut *mut int (unload)(struct snd_soc_component comp, struct snd_soc_dobj,
    pub control: snd_soc_dobj_control,
    pub widget: snd_soc_dobj_widget,
}

//
// Kcontrol operations - used to map handlers onto firmware based controls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_kcontrol_ops {
    pub id: u32,
    pub ucontrol): *mut snd_ctl_elem_value,
    pub ucontrol): *mut snd_ctl_elem_value,
    pub uinfo): *mut snd_ctl_elem_info,
}

// Bytes ext operations, for TLV byte controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_bytes_ext_ops {
    pub id: u32,
    pub size): c_uint,
    pub size): *const *const unsigned int __user bytes, unsigned int,
}

//
// DAPM widget event handlers - used to map handlers onto widgets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_widget_events {
    pub type: u16,
    pub event): *mut *mut snd_kcontrol k, int,
}

//
// Public API - Used by component drivers to load and unload dynamic objects
// and their resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_ops {
// external kcontrol init - used for any driver specific init
    pub ): *mut *mut snd_kcontrol_new , snd_soc_tplg_ctl_hdr,
    pub ): *mut snd_soc_dobj,
// DAPM graph route element loading and unloading
    pub route): *mut snd_soc_dapm_route,
    pub ): *mut snd_soc_dobj,
// external widget init - used for any driver specific init
    pub ): *mut snd_soc_tplg_dapm_widget,
    pub ): *mut snd_soc_tplg_dapm_widget,
    pub ): *mut snd_soc_dobj,
// FE DAI - used for any driver specific init
    pub dai): *mut *mut snd_soc_tplg_pcm pcm, snd_soc_dai,
    pub ): *mut snd_soc_dobj,
// DAI link - used for any driver specific init
    pub cfg): *mut snd_soc_tplg_link_config,
    pub ): *mut snd_soc_dobj,
// callback to handle vendor bespoke data
    pub ): *mut snd_soc_tplg_hdr,
    pub ): *mut snd_soc_tplg_hdr,
// completion - called at completion of firmware loading
    pub comp): *mut *mut int (complete)(struct snd_soc_component,
// manifest - optional to inform component of manifest
    pub ): *mut snd_soc_tplg_manifest,
// vendor specific kcontrol handlers available for binding
    pub io_ops: *const snd_soc_tplg_kcontrol_ops,
    pub io_ops_count: c_int,
// vendor specific bytes ext handlers available for binding
    pub bytes_ext_ops: *const snd_soc_tplg_bytes_ext_ops,
    pub bytes_ext_ops_count: c_int,
}

// gets a pointer to data from the firmware block header
// Dynamic Object loading and removal for component drivers
extern "C" {
    pub fn snd_soc_tplg_component_remove(comp: *mut snd_soc_component) -> c_int;
}
// Binds event handlers to dynamic widgets

