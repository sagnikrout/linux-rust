//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-dpcm.h
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
// linux/sound/soc-dpcm.h -- ALSA SoC Dynamic PCM Support
//
// Author:		Liam Girdwood <lrg@ti.com>
//

//
// Types of runtime_update to perform. e.g. originated from FE PCM ops
// or audio route changes triggered by muxes/mixers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_update {
    SND_SOC_DPCM_UPDATE_NO	= 0,
    SND_SOC_DPCM_UPDATE_BE,
    SND_SOC_DPCM_UPDATE_FE,
}

//
// Dynamic PCM Frontend -> Backend link management states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_link_state {
    SND_SOC_DPCM_LINK_STATE_NEW	= 0,	/* newly created link */
    SND_SOC_DPCM_LINK_STATE_FREE,		/* link to be dismantled */
}

//
// Dynamic PCM Frontend -> Backend link PCM states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_state {
    SND_SOC_DPCM_STATE_NEW	= 0,
    SND_SOC_DPCM_STATE_OPEN,
    SND_SOC_DPCM_STATE_HW_PARAMS,
    SND_SOC_DPCM_STATE_PREPARE,
    SND_SOC_DPCM_STATE_START,
    SND_SOC_DPCM_STATE_STOP,
    SND_SOC_DPCM_STATE_PAUSED,
    SND_SOC_DPCM_STATE_SUSPEND,
    SND_SOC_DPCM_STATE_HW_FREE,
    SND_SOC_DPCM_STATE_CLOSE,
}

//
// Dynamic PCM trigger ordering. Triggering flexibility is required as some
// DSPs require triggering before/after their CPU platform and DAIs.
//
// i.e. some clients may want to manually order this call in their PCM
// trigger() whilst others will just use the regular core ordering.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_trigger {
    SND_SOC_DPCM_TRIGGER_PRE		= 0,
    SND_SOC_DPCM_TRIGGER_POST,
}

//
// Dynamic PCM link
// This links together a FE and BE DAI at runtime and stores the link
// state information and the hw_params configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dpcm {
// FE and BE DAIs
    pub be: *mut snd_soc_pcm_runtime,
    pub fe: *mut snd_soc_pcm_runtime,
// link state
    pub state: snd_soc_dpcm_link_state,
// list of BE and FE for this DPCM link
    pub list_be: list_head,
    pub list_fe: list_head,

    pub debugfs_state: *mut dentry,

}

//
// Dynamic PCM runtime data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dpcm_runtime {
    pub be_clients: list_head,
    pub fe_clients: list_head,
    pub users: c_int,
    pub hw_params: snd_pcm_hw_params,
// state and update
    pub runtime_update: snd_soc_dpcm_update,
    pub state: snd_soc_dpcm_state,
    pub /: *mut *mut int trigger_pending; / trigger cmd + 1 if pending, 0 if not,
    pub /: *mut *mut int be_start; / refcount protected by BE stream pcm lock,
    pub /: *mut *mut int be_pause; / refcount protected by BE stream pcm lock,
    pub /: *mut *mut bool fe_pause; / used to track STOP after PAUSE,
}

// get the substream for this BE
// update audio routing between PCMs and any DAI links
extern "C" {
    pub fn snd_soc_dpcm_runtime_update(card: *mut snd_soc_card) -> c_int;
}

extern "C" {
    pub fn soc_dpcm_debugfs_add(rtd: *mut snd_soc_pcm_runtime);
}

extern "C" {
    pub fn dpcm_path_put(list: *mut snd_soc_dapm_widget_list);
}
extern "C" {
    pub fn dpcm_be_dai_startup(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
}
extern "C" {
    pub fn dpcm_be_disconnect(fe: *mut snd_soc_pcm_runtime, stream: c_int);
}
extern "C" {
    pub fn dpcm_clear_pending_state(fe: *mut snd_soc_pcm_runtime, stream: c_int);
}
extern "C" {
    pub fn dpcm_be_dai_hw_free(fe: *mut snd_soc_pcm_runtime, stream: c_int);
}
extern "C" {
    pub fn dpcm_be_dai_hw_params(fe: *mut snd_soc_pcm_runtime, tream: c_int) -> c_int;
}
extern "C" {
    pub fn dpcm_be_dai_trigger(fe: *mut snd_soc_pcm_runtime, stream: c_int, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn dpcm_be_dai_prepare(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
}
extern "C" {
    pub fn dpcm_dapm_stream_event(fe: *mut snd_soc_pcm_runtime, dir: c_int, event: c_int);
}
extern "C" {
    pub fn dpcm_end_walk_at_be(widget: *mut snd_soc_dapm_widget, dir: snd_soc_dapm_direction) -> bool;
}

