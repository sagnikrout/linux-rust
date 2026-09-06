//! Automatically rewritten from C to Rust
//! Source: sound/soc/meson/aiu-formatter-i2s.c
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
// Copyright (c) 2026 BayLibre, SAS.
// Author: Valerio Setti <vsetti@baylibre.com>

    static struct snd_soc_dai *
    aiu_formatter_i2s_get_be(struct snd_soc_dapm_widget *w)
    {
    struct snd_soc_dapm_path *p;
    struct snd_soc_dai *be;
    snd_soc_dapm_widget_for_each_sink_path(w, p) {
    if (!p.connect)
    continue;
    if (p.sink.id == snd_soc_dapm_dai_in)
    return (struct snd_soc_dai *)p.sink.priv;
    be = aiu_formatter_i2s_get_be(p.sink);
    if (be)
    return be;
    }
    return core::ptr::null_mut();
    }
    static struct gx_stream *
    aiu_formatter_i2s_get_stream(struct snd_soc_dapm_widget *w)
    {
    struct snd_soc_dai *be = aiu_formatter_i2s_get_be(w);
    if (!be)
    return core::ptr::null_mut();
    return snd_soc_dai_dma_data_get_playback(be);
    }
    static int aiu_formatter_i2s_prepare(struct regmap *map,
    const struct gx_formatter_hw *quirks,
    struct gx_stream *ts)
    {
// Always operate in split (classic interleaved) mode
    let mut desc: c_uint = 0;
//
// Pipeline reset is already implemented in aiu_fifo_i2s_trigger() at
// trigger time.
//
    switch (ts.physical_width) {
    case 16: /* Nothing to do */
    break;
    case 32:
    desc |= (AIU_I2S_SOURCE_DESC_MODE_24BIT |
    AIU_I2S_SOURCE_DESC_MODE_32BIT);
    break;
    default:
    return -EINVAL;
    }
    switch (ts.channels) {
    case 2: /* Nothing to do */
    break;
    case 8:
    desc |= AIU_I2S_SOURCE_DESC_MODE_8CH;
    break;
    default:
    return -EINVAL;
    }
    regmap_update_bits(map, AIU_I2S_SOURCE_DESC,
    AIU_I2S_SOURCE_DESC_MODE_8CH |
    AIU_I2S_SOURCE_DESC_MODE_24BIT |
    AIU_I2S_SOURCE_DESC_MODE_32BIT,
    desc);
// Send data MSB first
    regmap_update_bits(map, AIU_I2S_DAC_CFG,
    AIU_I2S_DAC_CFG_MSB_FIRST,
    AIU_I2S_DAC_CFG_MSB_FIRST);
    return 0;
    }
    const struct gx_formatter_ops aiu_formatter_i2s_ops = {
    .get_stream	= aiu_formatter_i2s_get_stream,
    .prepare	= aiu_formatter_i2s_prepare,
    };
