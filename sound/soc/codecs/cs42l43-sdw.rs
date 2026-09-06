//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs42l43-sdw.c
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
// CS42L43 CODEC driver SoundWire handling
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

    int cs42l43_sdw_add_peripheral(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct cs42l43_codec *priv = snd_soc_component_get_drvdata(dai.component);
    struct sdw_stream_runtime *sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    struct sdw_slave *sdw = dev_to_sdw_dev(priv.dev.parent);
    let mut sconfig: sdw_stream_config = {0};
    let mut pconfig: sdw_port_config = {0};
    int ret;
    if (!sdw_stream)
    return -EINVAL;
    snd_sdw_params_to_config(substream, params, &sconfig, &pconfig);
    pconfig.num = dai.id;
    ret = sdw_stream_add_slave(sdw, &sconfig, &pconfig, 1, sdw_stream);
    if (ret) {
    dev_err(priv.dev, "Failed to add sdw stream: %d\n", ret);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_add_peripheral, "SND_SOC_CS42L43");
    int cs42l43_sdw_remove_peripheral(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct cs42l43_codec *priv = snd_soc_component_get_drvdata(dai.component);
    struct sdw_stream_runtime *sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    struct sdw_slave *sdw = dev_to_sdw_dev(priv.dev.parent);
    if (!sdw_stream)
    return -EINVAL;
    return sdw_stream_remove_slave(sdw, sdw_stream);
    }
    EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_remove_peripheral, "SND_SOC_CS42L43");
#[no_mangle]
pub unsafe extern "C" fn cs42l43_sdw_set_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    int cs42l43_sdw_set_stream(struct snd_soc_dai *dai, void *sdw_stream, int direction)
    {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_set_stream, "SND_SOC_CS42L43");
    MODULE_DESCRIPTION("CS42L43 CODEC SoundWire Driver");
    MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
