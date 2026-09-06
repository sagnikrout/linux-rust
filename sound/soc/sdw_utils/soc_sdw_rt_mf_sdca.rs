//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_rt_mf_sdca.c
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
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2024 Intel Corporation.
//
// soc_sdw_rt_mf_sdca
// - Helpers to handle RT Multifunction Codec from generic machine driver
//

pub const CODEC_NAME_SIZE: c_int = 6;
// dapm routes for RT-SPK will be registered dynamically
    static const struct snd_soc_dapm_route rt712_spk_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt712 SPOL" },
    { "Speaker", core::ptr::null_mut(), "rt712 SPOR" },
    };
    static const struct snd_soc_dapm_route rt721_spk_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt721 SPK" },
    };
    static const struct snd_soc_dapm_route rt722_spk_map[] = {
    { "Speaker", core::ptr::null_mut(), "rt722 SPK" },
    };
// Structure to map codec names to respective route arrays and sizes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codec_route_map {
    pub codec_name: *const c_char,
    pub route_map: *const snd_soc_dapm_route,
    pub route_size: usize,
}

// Codec route maps array
    static const struct codec_route_map codec_routes[] = {
    { "rt712", rt712_spk_map, ARRAY_SIZE(rt712_spk_map) },
    { "rt721", rt721_spk_map, ARRAY_SIZE(rt721_spk_map) },
    { "rt722", rt722_spk_map, ARRAY_SIZE(rt722_spk_map) },
    };
    static const struct codec_route_map *get_codec_route_map(const char *codec_name)
    {
    for (size_t i = 0; i < ARRAY_SIZE(codec_routes); i++) {
    if (strcmp(codec_routes[i].codec_name, codec_name) == 0)
    return &codec_routes[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_mf_sdca_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_rt_mf_sdca_spk_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    char codec_name[CODEC_NAME_SIZE];
    int ret;
// acquire codec name
    snprintf(codec_name, CODEC_NAME_SIZE, "%s", dai.name);
// acquire corresponding route map and size
    const struct codec_route_map *route_map = get_codec_route_map(codec_name);
    if (!route_map) {
    dev_err(rtd.dev, "failed to get codec name and route map\n");
    return -EINVAL;
    }
// Add routes
    ret = snd_soc_dapm_add_routes(dapm, route_map.route_map, route_map.route_size);
    if (ret)
    dev_err(rtd.dev, "failed to add rt sdca spk map: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt_mf_sdca_spk_rtd_init, "SND_SOC_SDW_UTILS");
