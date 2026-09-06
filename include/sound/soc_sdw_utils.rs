//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc_sdw_utils.h
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
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright(c) 2024 Advanced Micro Devices, Inc.
//

pub const SOC_SDW_MAX_DAI_NUM: c_int = 8;
pub const SOC_SDW_MAX_AUX_NUM: c_int = 2;
pub const SOC_SDW_MAX_NO_PROPS: c_int = 2;

// If a CODEC has an optional speaker output, this quirk will enable it

//
// If the CODEC has additional devices attached directly to it.
//
// For the cs42l43:
// - 0 - No speaker output
// - SOC_SDW_CODEC_SPKR - CODEC internal speaker
// - SOC_SDW_SIDECAR_AMPS - 2x Sidecar amplifiers + CODEC internal speaker
// - SOC_SDW_CODEC_SPKR | SOF_SIDECAR_AMPS - Not currently supported
//

pub const SOC_SDW_JACK_OUT_DAI_ID: c_int = 0;
pub const SOC_SDW_JACK_IN_DAI_ID: c_int = 1;
pub const SOC_SDW_AMP_OUT_DAI_ID: c_int = 2;
pub const SOC_SDW_AMP_IN_DAI_ID: c_int = 3;
pub const SOC_SDW_DMIC_DAI_ID: c_int = 4;
pub const SOC_SDW_DAI_TYPE_JACK: c_int = 0;
pub const SOC_SDW_DAI_TYPE_AMP: c_int = 1;
pub const SOC_SDW_DAI_TYPE_MIC: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_mc_private {
    pub card: snd_soc_card,
    pub sdw_headset: snd_soc_jack,
    pub /: *mut *mut *mut device headset_codec_dev; / only one headset per card,
    pub amp_dev2: *mut *mut device amp_dev1,,
    pub append_dai_type: bool,
    pub ignore_internal_dmic: bool,
    pub private: *mut c_void,
    pub mc_quirk: c_ulong,
    pub codec_info_list_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_dai_info {
    pub /: *const *const bool direction[2]; / playback & capture support,
    pub codec_name: *const c_char,
    pub dai_name: *const c_char,
    pub component_name: *const c_char,
    pub dai_type: c_int,
    pub /: *const *const int dailink[2]; / dailink id for each direction,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub widgets: *const snd_soc_dapm_widget,
    pub num_widgets: c_int,
    pub playback): bool,
    pub dai_link): *mut *mut *mut int (exit)(struct snd_soc_card card, struct snd_soc_dai_link,
    pub dai): *mut *mut *mut int (rtd_init)(struct snd_soc_pcm_runtime rtd, struct snd_soc_dai,
    pub /: *mut *mut bool rtd_init_done; / Indicate that the rtd_init callback is done,
    pub quirk: c_ulong,
    pub quirk_exclude: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_aux_info {
    pub codec_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_codec_info {
    pub vendor_id: c_int,
    pub part_id: c_int,
    pub version_id: c_int,
    pub name_prefix: *const c_char,
    pub amp_num: c_int,
    pub acpi_id: [u8; ACPI_ID_LEN],
    pub ignore_internal_dmic: bool,
    pub ops: *const snd_soc_ops,
    pub dais: [asoc_sdw_dai_info; SOC_SDW_MAX_DAI_NUM],
    pub dai_num: c_int,
    pub auxs: [asoc_sdw_aux_info; SOC_SDW_MAX_AUX_NUM],
    pub aux_num: c_int,
// Force AMP-style name_prefix handling (append AMP index) even if MIC/Jack DAIs exist
    pub is_amp: bool,
    pub card): *mut *mut int (codec_card_late_probe)(struct snd_soc_card,
    pub num_devs): *mut *mut int num_dais, int,
    pub codec_conf): *mut snd_soc_codec_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_endpoint {
    pub list: list_head,
    pub link_mask: u32,
    pub codec_name: *const c_char,
    pub name_prefix: *const c_char,
    pub include_sidecar: bool,
    pub codec_info: *mut asoc_sdw_codec_info,
    pub dai_info: *const asoc_sdw_dai_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asoc_sdw_dailink {
    pub initialised: bool,
    pub group_id: u8,
    pub 1]: u32 link_mask[SNDRV_PCM_STREAM_LAST +,
    pub 1]: int num_devs[SNDRV_PCM_STREAM_LAST +,
    pub endpoints: list_head,
}

extern "C" {
    pub fn asoc_sdw_get_codec_info_list_count() -> c_int;
}
extern "C" {
    pub fn asoc_sdw_startup(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_prepare(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_prepare(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_hw_free(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_shutdown(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn asoc_sdw_mc_dailink_exit_loop(card: *mut snd_soc_card);
}
extern "C" {
    pub fn asoc_sdw_card_late_probe(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_get_dai_type(type: u32) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rtd_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
// DMIC support
extern "C" {
    pub fn asoc_sdw_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
// RT711 support
extern "C" {
    pub fn asoc_sdw_rt711_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int;
}
// RT711-SDCA support
extern "C" {
    pub fn asoc_sdw_rt_sdca_jack_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int;
}
// RT1308 I2S support
// generic amp support
extern "C" {
    pub fn asoc_sdw_rt_amp_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int;
}
// CS42L43 support
// es9356 codec support
extern "C" {
    pub fn asoc_sdw_es9356_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int;
}
// CS AMP support
extern "C" {
    pub fn asoc_sdw_cs35l56_volume_limit(card: *mut snd_soc_card, name_prefix: *const c_char) -> c_int;
}
// MAXIM codec support
// dai_link init callbacks
extern "C" {
    pub fn asoc_sdw_rt_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt_amp_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt700_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt711_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt_mf_sdca_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_rt5682_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l42_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l43_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l43_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l43_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l45_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs42l45_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs47l47_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs47l47_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_cs_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_maxim_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
// TI
extern "C" {
    pub fn asoc_sdw_ti_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_ti_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_ti_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_es9356_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_es9356_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn asoc_sdw_es9356_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
}
