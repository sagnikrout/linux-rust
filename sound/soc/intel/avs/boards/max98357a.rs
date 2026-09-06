//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/boards/max98357a.c
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

    static const struct snd_kcontrol_new card_controls[] = {
    SOC_DAPM_PIN_SWITCH("Spk"),
    };
    static const struct snd_soc_dapm_widget card_widgets[] = {
    SND_SOC_DAPM_SPK("Spk", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route card_base_routes[] = {
    { "Spk", core::ptr::null_mut(), "Speaker" },
    };
    static int
    avs_max98357a_be_fixup(struct snd_soc_pcm_runtime *runrime, struct snd_pcm_hw_params *params)
    {
    struct snd_interval *rate, *channels;
    struct snd_mask *fmt;
    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
// The ADSP will convert the FE rate to 48k, stereo
    rate.min = rate.max = 48000;
    channels.min = channels.max = 2;
// set SSP0 to 16 bit
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);
    return 0;
    }
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
    dl.codecs = devm_kzalloc(dev, sizeof(*dl.codecs), GFP_KERNEL);
    if (!dl.name || !dl.cpus || !dl.codecs)
    return -ENOMEM;
    dl.cpus.dai_name = devm_kasprintf(dev, GFP_KERNEL,
    AVS_STRING_FMT("SSP", " Pin", ssp_port, tdm_slot));
    dl.codecs.name = devm_kasprintf(dev, GFP_KERNEL, "MX98357A:00");
    dl.codecs.dai_name = devm_kasprintf(dev, GFP_KERNEL, "HiFi");
    if (!dl.cpus.dai_name || !dl.codecs.name || !dl.codecs.dai_name)
    return -ENOMEM;
    platform.name = dev_name(dev);
    dl.num_cpus = 1;
    dl.num_codecs = 1;
    dl.platforms = platform;
    dl.num_platforms = 1;
    dl.id = 0;
    dl.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    dl.be_hw_params_fixup = avs_max98357a_be_fixup;
    dl.nonatomic = 1;
    dl.no_pcm = 1;
    dl.playback_only = 1;
// dai_link = dl;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avs_max98357a_probe(pdev: *mut platform_device) -> c_int {
    static int avs_max98357a_probe(struct platform_device *pdev)
    {
    struct snd_soc_dai_link *dai_link;
    struct snd_soc_acpi_mach *mach;
    struct avs_mach_pdata *pdata;
    struct snd_soc_card *card;
    struct device *dev = &pdev.dev;
    int ssp_port, tdm_slot, ret;
    mach = dev_get_platdata(dev);
    pdata = mach.pdata;
    ret = avs_mach_get_ssp_tdm(dev, mach, &ssp_port, &tdm_slot);
    if (ret)
    return ret;
    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &dai_link);
    if (ret) {
    dev_err(dev, "Failed to create dai link: %d", ret);
    return ret;
    }
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    if (pdata.obsolete_card_names) {
    card.name = "avs_max98357a";
    } else {
    card.driver_name = "avs_max98357a";
    card.long_name = card.name = "AVS I2S MAX98357A";
    }
    card.dev = dev;
    card.owner = THIS_MODULE;
    card.dai_link = dai_link;
    card.num_links = 1;
    card.controls = card_controls;
    card.num_controls = ARRAY_SIZE(card_controls);
    card.dapm_widgets = card_widgets;
    card.num_dapm_widgets = ARRAY_SIZE(card_widgets);
    card.dapm_routes = card_base_routes;
    card.num_dapm_routes = ARRAY_SIZE(card_base_routes);
    card.fully_routed = true;
    return devm_snd_soc_register_deferrable_card(dev, card);
    }
    static const struct platform_device_id avs_max98357a_driver_ids[] = {
    { .name = "avs_max98357a" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, avs_max98357a_driver_ids);
    static struct platform_driver avs_max98357a_driver = {
    .probe = avs_max98357a_probe,
    .driver = {
    .name = "avs_max98357a",
    .pm = &snd_soc_pm_ops,
    },
    .id_table = avs_max98357a_driver_ids,
    };
    module_platform_driver(avs_max98357a_driver)
    MODULE_DESCRIPTION("Intel max98357a machine driver");
    MODULE_LICENSE("GPL");
