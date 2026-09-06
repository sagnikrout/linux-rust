//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adau7002.c
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
// ADAU7002 Stereo PDM-to-I2S/TDM converter driver
//
// Copyright 2014-2016 Analog Devices
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adau7002_priv {
    pub wakeup_delay: c_int,
}

    static int adau7002_aif_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component =
    snd_soc_dapm_to_component(w.dapm);
    struct adau7002_priv *adau7002 =
    snd_soc_component_get_drvdata(component);
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    if (adau7002.wakeup_delay)
    msleep(adau7002.wakeup_delay);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adau7002_component_probe(component: *mut snd_soc_component) -> c_int {
    static int adau7002_component_probe(struct snd_soc_component *component)
    {
    struct adau7002_priv *adau7002;
    adau7002 = devm_kzalloc(component.dev, sizeof(*adau7002),
    GFP_KERNEL);
    if (!adau7002)
    return -ENOMEM;
    device_property_read_u32(component.dev, "wakeup-delay-ms",
    &adau7002.wakeup_delay);
    snd_soc_component_set_drvdata(component, adau7002);
    return 0;
    }
    static const struct snd_soc_dapm_widget adau7002_widgets[] = {
    SND_SOC_DAPM_AIF_OUT_E("ADAU AIF", "Capture", 0,
    SND_SOC_NOPM, 0, 0, adau7002_aif_event,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT("PDM_DAT"),
    SND_SOC_DAPM_REGULATOR_SUPPLY("IOVDD", 0, 0),
    };
    static const struct snd_soc_dapm_route adau7002_routes[] = {
    { "ADAU AIF", core::ptr::null_mut(), "PDM_DAT"},
    { "Capture", core::ptr::null_mut(), "PDM_DAT" },
    { "Capture", core::ptr::null_mut(), "IOVDD" },
    };
    static struct snd_soc_dai_driver adau7002_dai = {
    .name = "adau7002-hifi",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_96000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S18_3LE |
    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE,
    .sig_bits = 20,
    },
    };
    static const struct snd_soc_component_driver adau7002_component_driver = {
    .probe			= adau7002_component_probe,
    .dapm_widgets		= adau7002_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(adau7002_widgets),
    .dapm_routes		= adau7002_routes,
    .num_dapm_routes	= ARRAY_SIZE(adau7002_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn adau7002_probe(pdev: *mut platform_device) -> c_int {
    static int adau7002_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &adau7002_component_driver,
    &adau7002_dai, 1);
    }

    static const struct of_device_id adau7002_dt_ids[] = {
    { .compatible = "adi,adau7002", },
    { }
    };
    MODULE_DEVICE_TABLE(of, adau7002_dt_ids);

    static const struct acpi_device_id adau7002_acpi_match[] = {
    { "ADAU7002", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, adau7002_acpi_match);

    static struct platform_driver adau7002_driver = {
    .driver = {
    .name = "adau7002",
    .of_match_table	= of_match_ptr(adau7002_dt_ids),
    .acpi_match_table = ACPI_PTR(adau7002_acpi_match),
    },
    .probe = adau7002_probe,
    };
    module_platform_driver(adau7002_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("ADAU7002 Stereo PDM-to-I2S/TDM Converter driver");
    MODULE_LICENSE("GPL v2");
