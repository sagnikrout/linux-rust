//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/hda-trace.c
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
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
// Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
// Rander Wang <rander.wang@intel.com>
// Keyon Jie <yang.jie@linux.intel.com>
//
// Hardware interface for generic Intel audio DSP HDA IP
//

#[no_mangle]
unsafe extern "C" fn hda_dsp_trace_prepare(sdev: *mut snd_sof_dev, dmab: *mut snd_dma_buffer) -> c_int {
    static int hda_dsp_trace_prepare(struct snd_sof_dev *sdev, struct snd_dma_buffer *dmab)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    struct hdac_ext_stream *hext_stream = hda.dtrace_stream;
    struct hdac_stream *hstream = &hext_stream.hstream;
    int ret;
    hstream.period_bytes = 0;/* initialize period_bytes */
    hstream.bufsize = dmab.bytes;
    ret = hda_dsp_stream_hw_params(sdev, hext_stream, dmab, core::ptr::null_mut());
    if (ret < 0)
    dev_err(sdev.dev, "error: hdac prepare failed: %d\n", ret);
    return ret;
    }
    int hda_dsp_trace_init(struct snd_sof_dev *sdev, struct snd_dma_buffer *dmab,
    struct sof_ipc_dma_trace_params_ext *dtrace_params)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    int ret;
    hda.dtrace_stream = hda_dsp_stream_get(sdev, SNDRV_PCM_STREAM_CAPTURE,
    SOF_HDA_STREAM_DMI_L1_COMPATIBLE);
    if (!hda.dtrace_stream) {
    dev_err(sdev.dev,
    "error: no available capture stream for DMA trace\n");
    return -ENODEV;
    }
    dtrace_params.stream_tag = hda.dtrace_stream.hstream.stream_tag;
//
// initialize capture stream, set BDL address and return corresponding
// stream tag which will be sent to the firmware by IPC message.
//
    ret = hda_dsp_trace_prepare(sdev, dmab);
    if (ret < 0) {
    dev_err(sdev.dev, "error: hdac trace init failed: %d\n", ret);
    hda_dsp_stream_put(sdev, SNDRV_PCM_STREAM_CAPTURE,
    dtrace_params.stream_tag);
    hda.dtrace_stream = core::ptr::null_mut();
    dtrace_params.stream_tag = 0;
    }
    return ret;
    }
    EXPORT_SYMBOL_NS(hda_dsp_trace_init, "SND_SOC_SOF_INTEL_HDA_COMMON");
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_trace_release(sdev: *mut snd_sof_dev) -> c_int {
    int hda_dsp_trace_release(struct snd_sof_dev *sdev)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    struct hdac_stream *hstream;
    if (hda.dtrace_stream) {
    hstream = &hda.dtrace_stream.hstream;
    hda_dsp_stream_put(sdev,
    SNDRV_PCM_STREAM_CAPTURE,
    hstream.stream_tag);
    hda.dtrace_stream = core::ptr::null_mut();
    return 0;
    }
    dev_dbg(sdev.dev, "DMA trace stream is not opened!\n");
    return -ENODEV;
    }
    EXPORT_SYMBOL_NS(hda_dsp_trace_release, "SND_SOC_SOF_INTEL_HDA_COMMON");
#[no_mangle]
pub unsafe extern "C" fn hda_dsp_trace_trigger(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int {
    int hda_dsp_trace_trigger(struct snd_sof_dev *sdev, int cmd)
    {
    struct sof_intel_hda_dev *hda = sdev.pdata.hw_pdata;
    return hda_dsp_stream_trigger(sdev, hda.dtrace_stream, cmd);
    }
    EXPORT_SYMBOL_NS(hda_dsp_trace_trigger, "SND_SOC_SOF_INTEL_HDA_COMMON");
