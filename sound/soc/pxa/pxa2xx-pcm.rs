//! Automatically rewritten from C to Rust
//! Source: sound/soc/pxa/pxa2xx-pcm.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/sound/arm/pxa2xx-pcm.c -- ALSA PCM interface for the Intel PXA2xx chip
//
// Author:	Nicolas Pitre
// Created:	Nov 30, 2004
// Copyright:	(C) 2004 MontaVista Software, Inc.
//

    static const struct snd_soc_component_driver pxa2xx_soc_platform = {
    .pcm_new	= pxa2xx_soc_pcm_new,
    .open		= pxa2xx_soc_pcm_open,
    .close		= pxa2xx_soc_pcm_close,
    .hw_params	= pxa2xx_soc_pcm_hw_params,
    .prepare	= pxa2xx_soc_pcm_prepare,
    .trigger	= pxa2xx_soc_pcm_trigger,
    .pointer	= pxa2xx_soc_pcm_pointer,
    };
#[no_mangle]
unsafe extern "C" fn pxa2xx_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    static int pxa2xx_soc_platform_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev, &pxa2xx_soc_platform,
    core::ptr::null_mut(), 0);
    }
    static struct platform_driver pxa_pcm_driver = {
    .driver = {
    .name = "pxa-pcm-audio",
    },
    .probe = pxa2xx_soc_platform_probe,
    };
    module_platform_driver(pxa_pcm_driver);
    MODULE_AUTHOR("Nicolas Pitre");
    MODULE_DESCRIPTION("Intel PXA2xx PCM DMA module");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:pxa-pcm-audio");
