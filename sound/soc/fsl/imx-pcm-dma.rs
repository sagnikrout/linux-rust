//! Automatically rewritten from C to Rust
//! Source: sound/soc/fsl/imx-pcm-dma.c
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
// imx-pcm-dma-mx2.c  --  ALSA Soc Audio Layer
//
// Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
//
// This code is based on code copyrighted by Freescale,
// Liam Girdwood, Javier Martin and probably others.
//

#[no_mangle]
unsafe extern "C" fn filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool filter(struct dma_chan *chan, void *param)
    {
    if (!imx_dma_is_general_purpose(chan))
    return false;
    chan.private = param;
    return true;
    }
    static const struct snd_dmaengine_pcm_config imx_dmaengine_pcm_config = {
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    .compat_filter_fn = filter,
    };
#[no_mangle]
pub unsafe extern "C" fn imx_pcm_dma_init(pdev: *mut platform_device) -> c_int {
    int imx_pcm_dma_init(struct platform_device *pdev)
    {
    struct snd_dmaengine_pcm_config *config;
    config = devm_kzalloc(&pdev.dev,
    sizeof(struct snd_dmaengine_pcm_config), GFP_KERNEL);
    if (!config)
    return -ENOMEM;
// config = imx_dmaengine_pcm_config;
    return devm_snd_dmaengine_pcm_register(&pdev.dev,
    config,
    SND_DMAENGINE_PCM_FLAG_COMPAT);
    }
    EXPORT_SYMBOL_GPL(imx_pcm_dma_init);
    MODULE_DESCRIPTION("Freescale i.MX PCM DMA interface");
    MODULE_LICENSE("GPL");
