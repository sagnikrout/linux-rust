//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/amd/acp-pcm.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//
// PCM interface for generic AMD audio ACP DSP block
//

    int acp_pcm_hw_params(struct snd_sof_dev *sdev, struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_sof_platform_stream_params *platform_params)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct acp_dsp_stream *stream = runtime.private_data;
    unsigned int buf_offset, index;
    u32 size;
    int ret;
    size = runtime.dma_bytes;
    stream.num_pages = PFN_UP(runtime.dma_bytes);
    stream.dmab = substream.runtime.dma_buffer_p;
    ret = acp_dsp_stream_config(sdev, stream);
    if (ret < 0) {
    dev_err(sdev.dev, "stream configuration failed\n");
    return ret;
    }
    platform_params.use_phy_address = true;
    platform_params.phy_addr = stream.reg_offset;
    platform_params.stream_tag = stream.stream_tag;
    platform_params.cont_update_posn = 1;
// write buffer size of stream in scratch memory
    buf_offset = sdev.debug_box.offset +
    offsetof(struct scratch_reg_conf, buf_size);
    index = stream.stream_tag - 1;
    buf_offset = buf_offset + index * 4;
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + buf_offset, size);
    return 0;
    }
    EXPORT_SYMBOL_NS(acp_pcm_hw_params, "SND_SOC_SOF_AMD_COMMON");
#[no_mangle]
pub unsafe extern "C" fn acp_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    int acp_pcm_open(struct snd_sof_dev *sdev, struct snd_pcm_substream *substream)
    {
    struct acp_dsp_stream *stream;
    stream = acp_dsp_stream_get(sdev, 0);
    if (!stream)
    return -ENODEV;
    substream.runtime.private_data = stream;
    stream.substream = substream;
    return 0;
    }
    EXPORT_SYMBOL_NS(acp_pcm_open, "SND_SOC_SOF_AMD_COMMON");
#[no_mangle]
pub unsafe extern "C" fn acp_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int {
    int acp_pcm_close(struct snd_sof_dev *sdev, struct snd_pcm_substream *substream)
    {
    struct acp_dsp_stream *stream;
    stream = substream.runtime.private_data;
    if (!stream) {
    dev_err(sdev.dev, "No open stream\n");
    return -EINVAL;
    }
    stream.substream = core::ptr::null_mut();
    substream.runtime.private_data = core::ptr::null_mut();
    return acp_dsp_stream_put(sdev, stream);
    }
    EXPORT_SYMBOL_NS(acp_pcm_close, "SND_SOC_SOF_AMD_COMMON");
    snd_pcm_uframes_t acp_pcm_pointer(struct snd_sof_dev *sdev,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_component *scomp = sdev.component;
    struct snd_sof_pcm_stream *stream;
    struct sof_ipc_stream_posn posn;
    struct snd_sof_pcm *spcm;
    snd_pcm_uframes_t pos;
    int ret;
    spcm = snd_sof_find_spcm_dai(scomp, rtd);
    if (!spcm) {
    dev_warn_ratelimited(sdev.dev, "warn: can't find PCM with DAI ID %d\n",
    rtd.dai_link.id);
    return 0;
    }
    stream = &spcm.stream[substream.stream];
    ret = snd_sof_ipc_msg_data(sdev, stream, &posn, sizeof(posn));
    if (ret < 0) {
    dev_warn(sdev.dev, "failed to read stream position: %d\n", ret);
    return 0;
    }
    memcpy(&stream.posn, &posn, sizeof(posn));
    pos = spcm.stream[substream.stream].posn.host_posn;
    pos = bytes_to_frames(substream.runtime, pos);
    return pos;
    }
    EXPORT_SYMBOL_NS(acp_pcm_pointer, "SND_SOC_SOF_AMD_COMMON");
