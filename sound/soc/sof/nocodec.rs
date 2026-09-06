//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/nocodec.c
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
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

    static struct snd_soc_card sof_nocodec_card = {
    .name = "nocodec", /* the sof- prefix is added by the core */
    .owner = THIS_MODULE
    };
    static int sof_nocodec_bes_setup(struct device *dev,
    struct snd_soc_dai_driver *drv,
    struct snd_soc_dai_link *links,
    int link_num)
    {
    struct snd_soc_card *card = &sof_nocodec_card;
    struct snd_soc_dai_link_component *dlc;
    int i;
    if (!drv || !links)
    return -EINVAL;
// set up BE dai_links
    for (i = 0; i < link_num; i++) {
    dlc = devm_kcalloc(dev, 2, sizeof(*dlc), GFP_KERNEL);
    if (!dlc)
    return -ENOMEM;
    links[i].name = devm_kasprintf(dev, GFP_KERNEL,
    "NoCodec-%d", i);
    if (!links[i].name)
    return -ENOMEM;
    links[i].stream_name = links[i].name;
    links[i].cpus = &dlc[0];
    links[i].codecs = &snd_soc_dummy_dlc;
    links[i].platforms = &dlc[1];
    links[i].num_cpus = 1;
    links[i].num_codecs = 1;
    links[i].num_platforms = 1;
    links[i].id = i;
    links[i].no_pcm = 1;
    links[i].cpus.dai_name = drv[i].name;
    links[i].platforms.name = dev_name(dev.parent);
    links[i].playback_only =  drv[i].playback.channels_min && !drv[i].capture.channels_min;
    links[i].capture_only  = !drv[i].playback.channels_min &&  drv[i].capture.channels_min;
    links[i].be_hw_params_fixup = sof_pcm_dai_link_fixup;
    }
    card.dai_link = links;
    card.num_links = link_num;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sof_nocodec_setup(dev: *mut device, mach: *mut snd_soc_acpi_mach) -> c_int {
    static int sof_nocodec_setup(struct device *dev, struct snd_soc_acpi_mach *mach)
    {
    let mut num_dai_drivers: u32 = mach.mach_params.num_dai_drivers;
    struct snd_soc_dai_driver *dai_drivers = mach.mach_params.dai_drivers;
    struct snd_soc_dai_link *links;
// create dummy BE dai_links
    links = devm_kcalloc(dev, num_dai_drivers, sizeof(struct snd_soc_dai_link), GFP_KERNEL);
    if (!links)
    return -ENOMEM;
    return sof_nocodec_bes_setup(dev, dai_drivers, links, num_dai_drivers);
    }
#[no_mangle]
unsafe extern "C" fn sof_nocodec_probe(pdev: *mut platform_device) -> c_int {
    static int sof_nocodec_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &sof_nocodec_card;
    struct snd_soc_acpi_mach *mach;
    int ret;
    card.dev = &pdev.dev;
    mach = pdev.dev.platform_data;
    snd_soc_card_set_topology_name(card, "sof");
    ret = sof_nocodec_setup(card.dev, mach);
    if (ret < 0)
    return ret;
    return devm_snd_soc_register_card(&pdev.dev, card);
    }
    static struct platform_driver sof_nocodec_audio = {
    .probe = sof_nocodec_probe,
    .driver = {
    .name = "sof-nocodec",
    .pm = &snd_soc_pm_ops,
    },
    };
    module_platform_driver(sof_nocodec_audio)
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("ASoC sof nocodec");
    MODULE_AUTHOR("Liam Girdwood");
    MODULE_ALIAS("platform:sof-nocodec");
