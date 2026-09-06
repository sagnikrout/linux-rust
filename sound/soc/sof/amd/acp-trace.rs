//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/amd/acp-trace.c
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
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vishnuvardhanrao Ravuapati <vishnuvardhanrao.ravulapati@amd.com>
// V Sujith Kumar Reddy <Vsujithkumar.Reddy@amd.com>
// This file support Host TRACE Logger driver callback for SOF FW

pub const ACP_LOGGER_STREAM: c_int = 8;
pub const NUM_PAGES: c_int = 16;
#[no_mangle]
pub unsafe extern "C" fn acp_sof_trace_release(sdev: *mut snd_sof_dev) -> c_int {
    int acp_sof_trace_release(struct snd_sof_dev *sdev)
    {
    struct acp_dsp_stream *stream;
    struct acp_dev_data *adata;
    int ret;
    adata = sdev.pdata.hw_pdata;
    stream = adata.dtrace_stream;
    ret = acp_dsp_stream_put(sdev, stream);
    if (ret < 0) {
    dev_err(sdev.dev, "Failed to release trace stream\n");
    return ret;
    }
    adata.dtrace_stream = core::ptr::null_mut();
    return 0;
    }
    EXPORT_SYMBOL_NS(acp_sof_trace_release, "SND_SOC_SOF_AMD_COMMON");
    int acp_sof_trace_init(struct snd_sof_dev *sdev, struct snd_dma_buffer *dmab,
    struct sof_ipc_dma_trace_params_ext *dtrace_params)
    {
    struct acp_dsp_stream *stream;
    struct acp_dev_data *adata;
    int ret;
    adata = sdev.pdata.hw_pdata;
    stream = acp_dsp_stream_get(sdev, ACP_LOGGER_STREAM);
    if (!stream)
    return -ENODEV;
    stream.dmab = dmab;
    stream.num_pages = NUM_PAGES;
    ret = acp_dsp_stream_config(sdev, stream);
    if (ret < 0) {
    acp_dsp_stream_put(sdev, stream);
    return ret;
    }
    adata.dtrace_stream = stream;
    dtrace_params.stream_tag = stream.stream_tag;
    dtrace_params.buffer.phy_addr = stream.reg_offset;
    return 0;
    }
    EXPORT_SYMBOL_NS(acp_sof_trace_init, "SND_SOC_SOF_AMD_COMMON");
