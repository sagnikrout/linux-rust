//! Automatically rewritten from C to Rust
//! Source: sound/core/oss/copy.c
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


// SPDX-License-Identifier: LGPL-2.0+
//
// Linear conversion Plug-In
// Copyright (c) 2000 by Abramo Bagnara <abramo@alsa-project.org>
//

    static snd_pcm_sframes_t copy_transfer(struct snd_pcm_plugin *plugin,
    const struct snd_pcm_plugin_channel *src_channels,
    struct snd_pcm_plugin_channel *dst_channels,
    snd_pcm_uframes_t frames)
    {
    unsigned int channel;
    unsigned int nchannels;
    if (snd_BUG_ON(!plugin || !src_channels || !dst_channels))
    return -ENXIO;
    if (frames == 0)
    return 0;
    nchannels = plugin.src_format.channels;
    for (channel = 0; channel < nchannels; channel++) {
    if (snd_BUG_ON(src_channels.area.first % 8 ||
    src_channels.area.step % 8))
    return -ENXIO;
    if (snd_BUG_ON(dst_channels.area.first % 8 ||
    dst_channels.area.step % 8))
    return -ENXIO;
    if (!src_channels.enabled) {
    if (dst_channels.wanted)
    snd_pcm_area_silence(&dst_channels.area, 0, frames, plugin.dst_format.format);
    dst_channels.enabled = 0;
    continue;
    }
    dst_channels.enabled = 1;
    snd_pcm_area_copy(&src_channels.area, 0, &dst_channels.area, 0, frames, plugin.src_format.format);
    src_channels++;
    dst_channels++;
    }
    return frames;
    }
    int snd_pcm_plugin_build_copy(struct snd_pcm_substream *plug,
    struct snd_pcm_plugin_format *src_format,
    struct snd_pcm_plugin_format *dst_format,
    struct snd_pcm_plugin **r_plugin)
    {
    int err;
    struct snd_pcm_plugin *plugin;
    int width;
    if (snd_BUG_ON(!r_plugin))
    return -ENXIO;
// r_plugin = NULL;
    if (snd_BUG_ON(src_format.format != dst_format.format))
    return -ENXIO;
    if (snd_BUG_ON(src_format.rate != dst_format.rate))
    return -ENXIO;
    if (snd_BUG_ON(src_format.channels != dst_format.channels))
    return -ENXIO;
    width = snd_pcm_format_physical_width(src_format.format);
    if (snd_BUG_ON(width <= 0))
    return -ENXIO;
    err = snd_pcm_plugin_build(plug, "copy", src_format, dst_format,
    0, &plugin);
    if (err < 0)
    return err;
    plugin.transfer = copy_transfer;
// r_plugin = plugin;
    return 0;
    }
