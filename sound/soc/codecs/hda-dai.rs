//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/hda-dai.c
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

#[no_mangle]
unsafe extern "C" fn hda_codec_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int hda_codec_dai_startup(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct hda_pcm_stream *stream_info;
    struct hda_codec *codec;
    struct hda_pcm *pcm;
    int ret;
    codec = dev_to_hda_codec(dai.dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    pcm = container_of(stream_info, struct hda_pcm, stream[substream.stream]);
    dev_dbg(dai.dev, "open stream codec: %08x, info: %p, pcm: %p %s substream: %p\n",
    codec.core.vendor_id, stream_info, pcm, pcm.name, substream);
    snd_hda_codec_pcm_get(pcm);
    ret = stream_info.ops.open(stream_info, codec, substream);
    if (ret < 0) {
    dev_err(dai.dev, "codec open failed: %d\n", ret);
    snd_hda_codec_pcm_put(pcm);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hda_codec_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    static void hda_codec_dai_shutdown(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct hda_pcm_stream *stream_info;
    struct hda_codec *codec;
    struct hda_pcm *pcm;
    int ret;
    codec = dev_to_hda_codec(dai.dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    pcm = container_of(stream_info, struct hda_pcm, stream[substream.stream]);
    dev_dbg(dai.dev, "close stream codec: %08x, info: %p, pcm: %p %s substream: %p\n",
    codec.core.vendor_id, stream_info, pcm, pcm.name, substream);
    ret = stream_info.ops.close(stream_info, codec, substream);
    if (ret < 0)
    dev_err(dai.dev, "codec close failed: %d\n", ret);
    snd_hda_codec_pcm_put(pcm);
    }
#[no_mangle]
unsafe extern "C" fn hda_codec_dai_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int hda_codec_dai_hw_free(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct hda_pcm_stream *stream_info;
    struct hda_codec *codec;
    codec = dev_to_hda_codec(dai.dev);
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    snd_hda_codec_cleanup(codec, stream_info, substream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hda_codec_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int hda_codec_dai_prepare(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct hda_pcm_stream *stream_info;
    struct hdac_stream *stream;
    struct hda_codec *codec;
    unsigned int format;
    unsigned int bits;
    int ret;
    codec = dev_to_hda_codec(dai.dev);
    stream = substream.runtime.private_data;
    stream_info = snd_soc_dai_get_dma_data(dai, substream);
    bits = snd_hdac_stream_format_bits(runtime.format, runtime.subformat,
    stream_info.maxbps);
    format = snd_hdac_stream_format(runtime.channels, bits, runtime.rate);
    ret = snd_hda_codec_prepare(codec, stream_info, stream.stream_tag, format, substream);
    if (ret < 0) {
    dev_err(dai.dev, "codec prepare failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    const struct snd_soc_dai_ops snd_soc_hda_codec_dai_ops = {
    .startup = hda_codec_dai_startup,
    .shutdown = hda_codec_dai_shutdown,
    .hw_free = hda_codec_dai_hw_free,
    .prepare = hda_codec_dai_prepare,
    };
    EXPORT_SYMBOL_GPL(snd_soc_hda_codec_dai_ops);
