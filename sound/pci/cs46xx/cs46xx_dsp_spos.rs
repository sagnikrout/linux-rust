//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/cs46xx_dsp_spos.h
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
// The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

pub const SYMBOL_CONSTANT: c_uint = 0x0;
pub const SYMBOL_SAMPLE: c_uint = 0x1;
pub const SYMBOL_PARAMETER: c_uint = 0x2;
pub const SYMBOL_CODE: c_uint = 0x3;
pub const SEGTYPE_SP_PROGRAM: c_uint = 0x00000001;
pub const SEGTYPE_SP_PARAMETER: c_uint = 0x00000002;
pub const SEGTYPE_SP_SAMPLE: c_uint = 0x00000003;
pub const SEGTYPE_SP_COEFFICIENT: c_uint = 0x00000004;
pub const DSP_SPOS_UU: c_uint = 0x0deadul     /* unused */;
pub const DSP_SPOS_DC: c_uint = 0x0badul      /* don't care */;
pub const DSP_SPOS_DC_DC: c_uint = 0x0bad0badul  /* don't care */;
pub const DSP_SPOS_UUUU: c_uint = 0xdeadc0edul  /* unused */;
pub const DSP_SPOS_UUHI: c_uint = 0xdeadul;
pub const DSP_SPOS_UULO: c_uint = 0xc0edul;
pub const DSP_SPOS_DCDC: c_uint = 0x0badf1d0ul  /* don't care */;
pub const DSP_SPOS_DCDCHI: c_uint = 0x0badul;
pub const DSP_SPOS_DCDCLO: c_uint = 0xf1d0ul;
pub const DSP_MAX_TASK_NAME: c_int = 60;
pub const DSP_MAX_SYMBOL_NAME: c_int = 100;
pub const DSP_MAX_SCB_NAME: c_int = 60;
pub const DSP_MAX_SCB_DESC: c_int = 200;
pub const DSP_MAX_TASK_DESC: c_int = 50;
pub const DSP_MAX_PCM_CHANNELS: c_int = 32;
pub const DSP_MAX_SRC_NR: c_int = 14;
pub const DSP_PCM_MAIN_CHANNEL: c_int = 1;
pub const DSP_PCM_REAR_CHANNEL: c_int = 2;
pub const DSP_PCM_CENTER_LFE_CHANNEL: c_int = 3;

pub const DSP_IEC958_CHANNEL: c_int = 5;
pub const DSP_SPDIF_STATUS_OUTPUT_ENABLED: c_int = 1;
pub const DSP_SPDIF_STATUS_PLAYBACK_OPEN: c_int = 2;
pub const DSP_SPDIF_STATUS_HW_ENABLED: c_int = 4;
pub const DSP_SPDIF_STATUS_INPUT_CTRL_ENABLED: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_symbol_entry {
    pub address: u32,
    pub symbol_name: [c_char; DSP_MAX_SYMBOL_NAME],
    pub symbol_type: c_int,
// initialized by driver
    pub module: *mut *mut dsp_module_desc,
    pub deleted: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_symbol_desc {
    pub nsymbols: c_int,
    pub symbols: *mut dsp_symbol_entry,
// initialized by driver
    pub highest_frag_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_segment_desc {
    pub segment_type: c_int,
    pub offset: u32,
    pub size: u32,
    pub data: *mut *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_module_desc {
    pub module_name: *mut *mut c_char,
    pub symbol_table: dsp_symbol_desc,
    pub nsegments: c_int,
    pub segments: *mut *mut dsp_segment_desc,
// initialized by driver
    pub overlay_begin_address: u32,
    pub load_address: u32,
    pub nfixups: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_scb_descriptor {
    pub scb_name: [c_char; DSP_MAX_SCB_NAME],
    pub address: u32,
    pub index: c_int,
    pub data: *mut u32,
    pub sub_list_ptr: *mut *mut dsp_scb_descriptor,
    pub next_scb_ptr: *mut *mut dsp_scb_descriptor,
    pub parent_scb_ptr: *mut *mut dsp_scb_descriptor,
    pub task_entry: *mut *mut dsp_symbol_entry,
    pub scb_symbol: *mut *mut dsp_symbol_entry,
    pub proc_info: *mut snd_info_entry,
    pub ref_count: c_int,
    pub volume: [u16; 2],
    pub :1: unsigned int deleted,
    pub :1: unsigned int updated,
    pub :1: unsigned int volume_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_task_descriptor {
    pub task_name: [c_char; DSP_MAX_TASK_NAME],
    pub size: c_int,
    pub address: u32,
    pub index: c_int,
    pub data: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_pcm_channel_descriptor {
    pub active: c_int,
    pub src_slot: c_int,
    pub pcm_slot: c_int,
    pub sample_rate: u32,
    pub unlinked: u32,
    pub pcm_reader_scb: *mut *mut dsp_scb_descriptor,
    pub src_scb: *mut *mut dsp_scb_descriptor,
    pub mixer_scb: *mut *mut dsp_scb_descriptor,
    pub private_data: *mut *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_spos_instance {
    pub /: *mut *mut dsp_symbol_desc symbol_table; / currently available loaded symbols in SP,
    pub nmodules: c_int,
    pub /: *mut *mut *mut dsp_module_desc  modules; / modules loaded into SP,
    pub code: dsp_segment_desc,
// Main PCM playback mixer
    pub master_mix_scb: *mut *mut dsp_scb_descriptor,
    pub dac_volume_right: u16,
    pub dac_volume_left: u16,
// Rear/surround PCM playback mixer
    pub rear_mix_scb: *mut *mut dsp_scb_descriptor,
// Center/LFE mixer
    pub center_lfe_mix_scb: *mut *mut dsp_scb_descriptor,
    pub npcm_channels: c_int,
    pub nsrc_scb: c_int,
    pub pcm_channels: [dsp_pcm_channel_descriptor; DSP_MAX_PCM_CHANNELS],
    pub src_scb_slots: [c_int; DSP_MAX_SRC_NR],
// cache this symbols
    pub /: *mut *mut *mut dsp_symbol_entry  null_algorithm; / used by PCMreaderSCB's,
    pub /: *mut *mut *mut dsp_symbol_entry  s16_up; / used by SRCtaskSCB's,
// proc fs
    pub snd_card: *mut snd_card,
    pub proc_dsp_dir: *mut *mut snd_info_entry,
// SCB's descriptors
    pub nscb: c_int,
    pub scb_highest_frag_index: c_int,
    pub scbs: [dsp_scb_descriptor; DSP_MAX_SCB_DESC],
    pub the_null_scb: *mut *mut dsp_scb_descriptor,
// Task's descriptors
    pub ntask: c_int,
    pub tasks: [dsp_task_descriptor; DSP_MAX_TASK_DESC],
// SPDIF status
    pub spdif_status_out: c_int,
    pub spdif_status_in: c_int,
    pub spdif_input_volume_right: u16,
    pub spdif_input_volume_left: u16,
// spdif channel status,
    pub spdif_csuv_default: c_uint,
    pub spdif_csuv_stream: c_uint,
// SPDIF input sample rate converter
    pub spdif_in_src: *mut *mut dsp_scb_descriptor,
// SPDIF input asynch. receiver
    pub asynch_rx_scb: *mut *mut dsp_scb_descriptor,
// Capture record mixer SCB
    pub record_mixer_scb: *mut *mut dsp_scb_descriptor,
// CODEC input SCB
    pub codec_in_scb: *mut *mut dsp_scb_descriptor,
// reference snooper
    pub ref_snoop_scb: *mut *mut dsp_scb_descriptor,
// SPDIF output  PCM reference
    pub spdif_pcm_input_scb: *mut *mut dsp_scb_descriptor,
// asynch TX task
    pub asynch_tx_scb: *mut *mut dsp_scb_descriptor,
// record sources
    pub pcm_input: *mut *mut dsp_scb_descriptor,
    pub adc_input: *mut *mut dsp_scb_descriptor,
    pub spdif_in_sample_rate: c_int,
}
