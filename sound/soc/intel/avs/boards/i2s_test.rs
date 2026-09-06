//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/boards/i2s_test.c
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

    static int avs_create_dai_link(struct device *dev, int ssp_port, int tdm_slot,
    struct snd_soc_dai_link **dai_link)
    {
    struct snd_soc_dai_link_component *platform;
    struct snd_soc_dai_link *dl;
    dl = devm_kzalloc(dev, sizeof(*dl), GFP_KERNEL);
    platform = devm_kzalloc(dev, sizeof(*platform), GFP_KERNEL);
    if (!dl || !platform)
    return -ENOMEM;
    dl.name = devm_kasprintf(dev, GFP_KERNEL,
    AVS_STRING_FMT("SSP", "-Codec", ssp_port, tdm_slot));
    dl.cpus = devm_kzalloc(dev, sizeof(*dl.cpus), GFP_KERNEL);
    if (!dl.name || !dl.cpus)
    return -ENOMEM;
    dl.cpus.dai_name = devm_kasprintf(dev, GFP_KERNEL,
    AVS_STRING_FMT("SSP", " Pin", ssp_port, tdm_slot));
    dl.codecs = &snd_soc_dummy_dlc;
    if (!dl.cpus.dai_name || !dl.codecs.name || !dl.codecs.dai_name)
    return -ENOMEM;
    platform.name = dev_name(dev);
    dl.num_cpus = 1;
    dl.num_codecs = 1;
    dl.platforms = platform;
    dl.num_platforms = 1;
    dl.id = 0;
    dl.nonatomic = 1;
    dl.no_pcm = 1;
// dai_link = dl;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avs_i2s_test_probe(pdev: *mut platform_device) -> c_int {
    static int avs_i2s_test_probe(struct platform_device *pdev)
    {
    struct snd_soc_dai_link *dai_link;
    struct snd_soc_acpi_mach *mach;
    struct avs_mach_pdata *pdata;
    struct snd_soc_card *card;
    struct device *dev = &pdev.dev;
    int ssp_port, tdm_slot, ret;
    mach = dev_get_platdata(dev);
    pdata = mach.pdata;
    if (!avs_mach_singular_ssp(mach)) {
    dev_err(dev, "Invalid SSP configuration\n");
    return -EINVAL;
    }
    ssp_port = avs_mach_ssp_port(mach);
    if (!avs_mach_singular_tdm(mach, ssp_port)) {
    dev_err(dev, "Invalid TDM configuration\n");
    return -EINVAL;
    }
    tdm_slot = avs_mach_ssp_tdm(mach, ssp_port);
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    if (pdata.obsolete_card_names) {
    card.name = devm_kasprintf(dev, GFP_KERNEL,
    AVS_STRING_FMT("ssp", "-loopback", ssp_port, tdm_slot));
    } else {
    card.driver_name = "avs_i2s_test";
    card.long_name = card.name = devm_kasprintf(dev, GFP_KERNEL,
    AVS_STRING_FMT("AVS I2S TEST-", "",
    ssp_port, tdm_slot));
    }
    if (!card.name)
    return -ENOMEM;
    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &dai_link);
    if (ret) {
    dev_err(dev, "Failed to create dai link: %d\n", ret);
    return ret;
    }
    card.dev = dev;
    card.owner = THIS_MODULE;
    card.dai_link = dai_link;
    card.num_links = 1;
    card.fully_routed = true;
    return devm_snd_soc_register_deferrable_card(dev, card);
    }
    static const struct platform_device_id avs_i2s_test_driver_ids[] = {
    { .name = "avs_i2s_test" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, avs_i2s_test_driver_ids);
    static struct platform_driver avs_i2s_test_driver = {
    .probe = avs_i2s_test_probe,
    .driver = {
    .name = "avs_i2s_test",
    .pm = &snd_soc_pm_ops,
    },
    .id_table = avs_i2s_test_driver_ids,
    };
    module_platform_driver(avs_i2s_test_driver);
    MODULE_DESCRIPTION("Intel i2s test machine driver");
    MODULE_LICENSE("GPL");
