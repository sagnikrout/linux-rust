//! Automatically rewritten from C to Rust
//! Source: sound/core/oss/io.c
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
// PCM I/O Plug-In Interface
// Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>
//

//
// Basic io plugin
//
    static snd_pcm_sframes_t io_playback_transfer(struct snd_pcm_plugin *plugin,
    const struct snd_pcm_plugin_channel *src_channels,
    struct snd_pcm_plugin_channel *dst_channels,
    snd_pcm_uframes_t frames)
    {
    if (snd_BUG_ON(!plugin))
    return -ENXIO;
    if (snd_BUG_ON(!src_channels))
    return -ENXIO;
    if (plugin.access == SNDRV_PCM_ACCESS_RW_INTERLEAVED) {
    return pcm_write(plugin.plug, src_channels.area.addr, frames);
    } else {
    int channel, channels = plugin.dst_format.channels;
    void **bufs = (void**)plugin.extra_data;
    if (snd_BUG_ON(!bufs))
    return -ENXIO;
    for (channel = 0; channel < channels; channel++) {
    if (src_channels[channel].enabled)
    bufs[channel] = src_channels[channel].area.addr;
    else
    bufs[channel] = core::ptr::null_mut();
    }
    return pcm_writev(plugin.plug, bufs, frames);
    }
    }
    static snd_pcm_sframes_t io_capture_transfer(struct snd_pcm_plugin *plugin,
    const struct snd_pcm_plugin_channel *src_channels,
    struct snd_pcm_plugin_channel *dst_channels,
    snd_pcm_uframes_t frames)
    {
    if (snd_BUG_ON(!plugin))
    return -ENXIO;
    if (snd_BUG_ON(!dst_channels))
    return -ENXIO;
    if (plugin.access == SNDRV_PCM_ACCESS_RW_INTERLEAVED) {
    return pcm_read(plugin.plug, dst_channels.area.addr, frames);
    } else {
    int channel, channels = plugin.dst_format.channels;
    void **bufs = (void**)plugin.extra_data;
    if (snd_BUG_ON(!bufs))
    return -ENXIO;
    for (channel = 0; channel < channels; channel++) {
    if (dst_channels[channel].enabled)
    bufs[channel] = dst_channels[channel].area.addr;
    else
    bufs[channel] = core::ptr::null_mut();
    }
    return pcm_readv(plugin.plug, bufs, frames);
    }
    return 0;
    }
    static snd_pcm_sframes_t io_src_channels(struct snd_pcm_plugin *plugin,
    snd_pcm_uframes_t frames,
    struct snd_pcm_plugin_channel **channels)
    {
    int err;
    unsigned int channel;
    struct snd_pcm_plugin_channel *v;
    err = snd_pcm_plugin_client_channels(plugin, frames, &v);
    if (err < 0)
    return err;
// channels = v;
    if (plugin.access == SNDRV_PCM_ACCESS_RW_INTERLEAVED) {
    for (channel = 0; channel < plugin.src_format.channels; ++channel, ++v)
    v.wanted = 1;
    }
    return frames;
    }
    int snd_pcm_plugin_build_io(struct snd_pcm_substream *plug,
    struct snd_pcm_hw_params *params,
    struct snd_pcm_plugin **r_plugin)
    {
    int err;
    struct snd_pcm_plugin_format format;
    struct snd_pcm_plugin *plugin;
    if (snd_BUG_ON(!r_plugin))
    return -ENXIO;
// r_plugin = NULL;
    if (snd_BUG_ON(!plug || !params))
    return -ENXIO;
    format.format = params_format(params);
    format.rate = params_rate(params);
    format.channels = params_channels(params);
    err = snd_pcm_plugin_build(plug, "I/O io",
    &format, &format,
    sizeof(void *) * format.channels,
    &plugin);
    if (err < 0)
    return err;
    plugin.access = params_access(params);
    if (snd_pcm_plug_stream(plug) == SNDRV_PCM_STREAM_PLAYBACK) {
    plugin.transfer = io_playback_transfer;
    if (plugin.access == SNDRV_PCM_ACCESS_RW_INTERLEAVED)
    plugin.client_channels = io_src_channels;
    } else {
    plugin.transfer = io_capture_transfer;
    }
// r_plugin = plugin;
    return 0;
    }
