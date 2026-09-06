//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/simple_card_utils.h
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
// simple_card_utils.h
//
// Copyright (c) 2016 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_util_tdm_width_map {
    pub sample_bits: u8,
    pub slot_count: u8,
    pub slot_width: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_util_dai {
    pub name: *const c_char,
    pub sysclk: c_uint,
    pub clk_direction: c_int,
    pub slots: c_int,
    pub slot_width: c_int,
    pub tx_slot_mask: c_uint,
    pub rx_slot_mask: c_uint,
    pub clk: *mut clk,
    pub clk_fixed: bool,
    pub tdm_width_map: *mut simple_util_tdm_width_map,
    pub n_tdm_widths: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_util_data {
    pub convert_rate: u32,
    pub convert_channels: u32,
    pub convert_sample_format: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_util_jack {
    pub jack: snd_soc_jack,
    pub pin: snd_soc_jack_pin,
    pub gpio: snd_soc_jack_gpio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prop_nums {
    pub cpus: c_int,
    pub codecs: c_int,
    pub platforms: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simple_util_sysclk_order {
    SIMPLE_SYSCLK_ORDER_CODEC_FIRST = 0,
    SIMPLE_SYSCLK_ORDER_CPU_FIRST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_util_priv {
    pub snd_card: snd_soc_card,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_dai_props {
    pub cpu_dai: *mut simple_util_dai,
    pub codec_dai: *mut simple_util_dai,
    pub adata: simple_util_data,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num: prop_nums,
    pub mclk_fs: c_uint,
    pub sysclk_order: simple_util_sysclk_order,
    pub dai_props: *mut },
    pub hp_jack: simple_util_jack,
    pub mic_jack: simple_util_jack,
    pub aux_jacks: *mut snd_soc_jack,
    pub dai_link: *mut snd_soc_dai_link,
    pub dais: *mut simple_util_dai,
    pub dlcs: *mut snd_soc_dai_link_component,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub pa_gpio: *mut gpio_desc,
    pub ops: *const snd_soc_ops,
    pub dpcm_selectable:1: c_uint,
    pub force_dpcm:1: c_uint,
}

// has the same effect as simple_priv_to_props(). Preferred over
// simple_priv_to_props() when dealing with PCM runtime data as
// the ID stored in rtd->id may not be a valid array index.
//

pub const SNDRV_MAX_LINKS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_info {
    pub /: *mut *mut int link; / number of link,
    pub /: *mut *mut int cpu; / turn for CPU / Codec,
    pub num: [prop_nums; SNDRV_MAX_LINKS],
}

extern "C" {
    pub fn simple_util_startup(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn simple_util_shutdown(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn simple_util_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}

extern "C" {
    pub fn simple_util_clean_reference(priv: *mut simple_util_priv);
}
extern "C" {
    pub fn simple_util_is_convert_required(data: *const simple_util_data) -> bool;
}
extern "C" {
    pub fn simple_util_get_sample_fmt(data: *mut simple_util_data) -> c_int;
}
extern "C" {
    pub fn simple_util_init_aux_jacks(card: *mut snd_soc_card, prefix: *mut c_char) -> c_int;
}
extern "C" {
    pub fn simple_util_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn graph_util_card_probe(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn graph_util_is_ports0(port: *mut device_node) -> c_int;
}

// dai might be NULL

// Macro flag: #define  simple_util_debug_info(priv)

