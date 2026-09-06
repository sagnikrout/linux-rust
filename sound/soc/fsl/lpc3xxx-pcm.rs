//! Automatically rewritten from C to Rust
//! Source: sound/soc/fsl/lpc3xxx-pcm.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2008 NXP Semiconductors
// Copyright 2023 Timesys Corporation <piotr.wojtaszczyk@timesys.com>

    SNDRV_PCM_FMTBIT_U8 | \
    SNDRV_PCM_FMTBIT_S16_LE | \
    SNDRV_PCM_FMTBIT_U16_LE | \
    SNDRV_PCM_FMTBIT_S24_LE | \
    SNDRV_PCM_FMTBIT_U24_LE | \
    SNDRV_PCM_FMTBIT_S32_LE | \
    SNDRV_PCM_FMTBIT_U32_LE | \
    SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE)
    static const struct snd_pcm_hardware lpc3xxx_pcm_hardware = {
    .info = (SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME),
    .formats = STUB_FORMATS,
    .period_bytes_min = 128,
    .period_bytes_max = 2048,
    .periods_min = 2,
    .periods_max = 1024,
    .buffer_bytes_max = 128 * 1024
    };
    static const struct snd_dmaengine_pcm_config lpc3xxx_dmaengine_pcm_config = {
    .pcm_hardware = &lpc3xxx_pcm_hardware,
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    .compat_filter_fn = pl08x_filter_id,
    .prealloc_buffer_size = 128 * 1024,
    };
    static const struct snd_soc_component_driver lpc3xxx_soc_platform_driver = {
    .name = "lpc32xx-pcm",
    };
#[no_mangle]
pub unsafe extern "C" fn lpc3xxx_pcm_register(pdev: *mut platform_device) -> c_int {
    int lpc3xxx_pcm_register(struct platform_device *pdev)
    {
    int ret;
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, &lpc3xxx_dmaengine_pcm_config, 0);
    if (ret) {
    dev_err(&pdev.dev, "failed to register dmaengine: %d\n", ret);
    return ret;
    }
    return devm_snd_soc_register_component(&pdev.dev, &lpc3xxx_soc_platform_driver,
    core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL(lpc3xxx_pcm_register);
