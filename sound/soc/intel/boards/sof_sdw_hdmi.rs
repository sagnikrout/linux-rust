//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/boards/sof_sdw_hdmi.c
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
// Copyright (c) 2020 Intel Corporation
//
// sof_sdw_hdmi - Helpers to handle HDMI from generic machine driver
//

#[no_mangle]
pub unsafe extern "C" fn sof_sdw_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    int sof_sdw_hdmi_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(rtd.card);
    struct intel_mc_ctx *intel_ctx = (struct intel_mc_ctx *)ctx.private;
    struct snd_soc_dai *dai = snd_soc_rtd_to_codec(rtd, 0);
    intel_ctx.hdmi.hdmi_comp = dai.component;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sof_sdw_hdmi_card_late_probe(card: *mut snd_soc_card) -> c_int {
    int sof_sdw_hdmi_card_late_probe(struct snd_soc_card *card)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct intel_mc_ctx *intel_ctx = (struct intel_mc_ctx *)ctx.private;
    if (!intel_ctx.hdmi.idisp_codec)
    return 0;
    if (!intel_ctx.hdmi.hdmi_comp)
    return -EINVAL;
    return hda_dsp_hdmi_build_controls(card, intel_ctx.hdmi.hdmi_comp);
    }
