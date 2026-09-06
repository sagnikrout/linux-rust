//! Automatically rewritten from C to Rust
//! Source: sound/soc/samsung/dmaengine.c
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
// dmaengine.c - Samsung dmaengine wrapper
//
// Author: Mark Brown <broonie@linaro.org>
// Copyright 2013 Linaro

    int samsung_asoc_dma_platform_register(struct device *dev, dma_filter_fn filter,
    const char *tx, const char *rx,
    struct device *dma_dev)
    {
    struct snd_dmaengine_pcm_config *pcm_conf;
    pcm_conf = devm_kzalloc(dev, sizeof(*pcm_conf), GFP_KERNEL);
    if (!pcm_conf)
    return -ENOMEM;
    pcm_conf.prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config;
    pcm_conf.compat_filter_fn = filter;
    pcm_conf.dma_dev = dma_dev;
    pcm_conf.chan_names[SNDRV_PCM_STREAM_PLAYBACK] = tx;
    pcm_conf.chan_names[SNDRV_PCM_STREAM_CAPTURE] = rx;
    return devm_snd_dmaengine_pcm_register(dev, pcm_conf,
    SND_DMAENGINE_PCM_FLAG_COMPAT);
    }
    EXPORT_SYMBOL_GPL(samsung_asoc_dma_platform_register);
    MODULE_AUTHOR("Mark Brown <broonie@linaro.org>");
    MODULE_DESCRIPTION("Samsung dmaengine ASoC driver");
    MODULE_LICENSE("GPL");
