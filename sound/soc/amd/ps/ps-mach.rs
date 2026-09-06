//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/ps/ps-mach.c
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
// Machine driver for AMD Pink Sardine platform using DMIC
//
// Copyright 2022 Advanced Micro Devices, Inc.
//

    SND_SOC_DAILINK_DEF(acp63_pdm,
    DAILINK_COMP_ARRAY(COMP_CPU("acp_ps_pdm_dma.0")));
    SND_SOC_DAILINK_DEF(dmic_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec.0",
    "dmic-hifi")));
    SND_SOC_DAILINK_DEF(pdm_platform,
    DAILINK_COMP_ARRAY(COMP_PLATFORM("acp_ps_pdm_dma.0")));
    static struct snd_soc_dai_link acp63_dai_pdm[] = {
    {
    .name = "acp63-dmic-capture",
    .stream_name = "DMIC capture",
    .capture_only = 1,
    SND_SOC_DAILINK_REG(acp63_pdm, dmic_codec, pdm_platform),
    },
    };
    static struct snd_soc_card acp63_card = {
    .name = "acp63",
    .owner = THIS_MODULE,
    .dai_link = acp63_dai_pdm,
    .num_links = 1,
    };
#[no_mangle]
unsafe extern "C" fn acp63_probe(pdev: *mut platform_device) -> c_int {
    static int acp63_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card;
    int ret;
    platform_set_drvdata(pdev, &acp63_card);
    card = platform_get_drvdata(pdev);
    acp63_card.dev = &pdev.dev;
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret) {
    return dev_err_probe(&pdev.dev, ret,
    "snd_soc_register_card(%s) failed\n",
    card.name);
    }
    return 0;
    }
    static struct platform_driver acp63_mach_driver = {
    .driver = {
    .name = "acp_ps_mach",
    .pm = &snd_soc_pm_ops,
    },
    .probe = acp63_probe,
    };
    module_platform_driver(acp63_mach_driver);
    MODULE_AUTHOR("Syed.SabaKareem@amd.com");
    MODULE_DESCRIPTION("AMD Pink Sardine support for DMIC");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRV_NAME);
