//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/max98357a.c
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
// Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
//
// max98357a.c -- MAX98357A ALSA SoC Codec driver
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98357a_priv {
    pub sdmode: *mut gpio_desc,
    pub sdmode_delay: c_uint,
    pub sdmode_switch: c_int,
}

    static int max98357a_daiops_trigger(struct snd_pcm_substream *substream,
    int cmd, struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct max98357a_priv *max98357a =
    snd_soc_component_get_drvdata(component);
    if (!max98357a.sdmode)
    return 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    mdelay(max98357a.sdmode_delay);
    if (max98357a.sdmode_switch) {
    gpiod_set_value(max98357a.sdmode, 1);
    dev_dbg(component.dev, "set sdmode to 1");
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    gpiod_set_value(max98357a.sdmode, 0);
    dev_dbg(component.dev, "set sdmode to 0");
    break;
    }
    return 0;
    }
    static int max98357a_sdmode_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component =
    snd_soc_dapm_to_component(w.dapm);
    struct max98357a_priv *max98357a =
    snd_soc_component_get_drvdata(component);
    if (event & SND_SOC_DAPM_POST_PMU)
    max98357a.sdmode_switch = 1;
#[no_mangle]
pub unsafe extern "C" fn if(SND_SOC_DAPM_POST_PMD: event &) -> else {
    else if (event & SND_SOC_DAPM_POST_PMD)
    max98357a.sdmode_switch = 0;
    return 0;
    }
    static const struct snd_soc_dapm_widget max98357a_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("Speaker"),
    SND_SOC_DAPM_OUT_DRV_E("SD_MODE", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0,
    max98357a_sdmode_event,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    };
    static const struct snd_soc_dapm_route max98357a_dapm_routes[] = {
    {"SD_MODE", core::ptr::null_mut(), "HiFi Playback"},
    {"Speaker", core::ptr::null_mut(), "SD_MODE"},
    };
    static const struct snd_soc_component_driver max98357a_component_driver = {
    .dapm_widgets		= max98357a_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(max98357a_dapm_widgets),
    .dapm_routes		= max98357a_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(max98357a_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct snd_soc_dai_ops max98357a_dai_ops = {
    .trigger        = max98357a_daiops_trigger,
    };
    static struct snd_soc_dai_driver max98357a_dai_driver = {
    .name = "HiFi",
    .playback = {
    .stream_name	= "HiFi Playback",
    .formats	= SNDRV_PCM_FMTBIT_S16 |
    SNDRV_PCM_FMTBIT_S24 |
    SNDRV_PCM_FMTBIT_S32,
    .rates		= SNDRV_PCM_RATE_8000 |
    SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000,
    .rate_min	= 8000,
    .rate_max	= 96000,
    .channels_min	= 1,
    .channels_max	= 2,
    },
    .ops    = &max98357a_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn max98357a_platform_probe(pdev: *mut platform_device) -> c_int {
    static int max98357a_platform_probe(struct platform_device *pdev)
    {
    struct max98357a_priv *max98357a;
    int ret;
    max98357a = devm_kzalloc(&pdev.dev, sizeof(*max98357a), GFP_KERNEL);
    if (!max98357a)
    return -ENOMEM;
    max98357a.sdmode = devm_gpiod_get_optional(&pdev.dev,
    "sdmode", GPIOD_OUT_LOW);
    if (IS_ERR(max98357a.sdmode))
    return PTR_ERR(max98357a.sdmode);
    ret = device_property_read_u32(&pdev.dev, "sdmode-delay",
    &max98357a.sdmode_delay);
    if (ret) {
    max98357a.sdmode_delay = 0;
    dev_dbg(&pdev.dev,
    "no optional property 'sdmode-delay' found, "
    "default: no delay\n");
    }
    dev_set_drvdata(&pdev.dev, max98357a);
    return devm_snd_soc_register_component(&pdev.dev,
    &max98357a_component_driver,
    &max98357a_dai_driver, 1);
    }

    static const struct of_device_id max98357a_device_id[] = {
    { .compatible = "maxim,max98357a" },
    { .compatible = "maxim,max98360a" },
    {}
    };
    MODULE_DEVICE_TABLE(of, max98357a_device_id);

    static const struct acpi_device_id max98357a_acpi_match[] = {
    { "MX98357A", 0 },
    { "MX98360A", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, max98357a_acpi_match);

    static struct platform_driver max98357a_platform_driver = {
    .driver = {
    .name = "max98357a",
    .of_match_table = of_match_ptr(max98357a_device_id),
    .acpi_match_table = ACPI_PTR(max98357a_acpi_match),
    },
    .probe	= max98357a_platform_probe,
    };
    module_platform_driver(max98357a_platform_driver);
    MODULE_DESCRIPTION("Maxim MAX98357A Codec Driver");
    MODULE_LICENSE("GPL v2");
