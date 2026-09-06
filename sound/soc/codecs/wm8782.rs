//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wm8782.c
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
// sound/soc/codecs/wm8782.c
// simple, strap-pin configured 24bit 2ch ADC
//
// Copyright: 2011 Raumfeld GmbH
// Author: Johannes Stezenbach <js@sig21.net>
//
// based on ad73311.c
// Copyright:	Analog Devices Inc.
// Author:	Cliff Cai <cliff.cai@analog.com>
//

// regulator power supply names
    static const char *supply_names[] = {
    "Vdda", /* analog supply, 2.7V - 3.6V */
    "Vdd",  /* digital supply, 2.7V - 5.5V */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8782_priv {
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(supply_names)],
    pub max_rate: c_int,
}

#[no_mangle]
unsafe extern "C" fn wm8782_dai_startup(sub: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int wm8782_dai_startup(struct snd_pcm_substream *sub, struct snd_soc_dai *dai)
    {
    struct snd_pcm_runtime *runtime = sub.runtime;
    struct wm8782_priv *priv =
    snd_soc_component_get_drvdata(dai.component);
    return snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_RATE,
    8000, priv.max_rate);
    }
    static const struct snd_soc_dapm_widget wm8782_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("AINL"),
    SND_SOC_DAPM_INPUT("AINR"),
    };
    static const struct snd_soc_dapm_route wm8782_dapm_routes[] = {
    { "Capture", core::ptr::null_mut(), "AINL" },
    { "Capture", core::ptr::null_mut(), "AINR" },
    };
    static const struct snd_soc_dai_ops wm8782_dai_ops = {
    .startup = &wm8782_dai_startup,
    };
    static struct snd_soc_dai_driver wm8782_dai = {
    .name = "wm8782",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE,
    },
    .ops = &wm8782_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn wm8782_soc_probe(component: *mut snd_soc_component) -> c_int {
    static int wm8782_soc_probe(struct snd_soc_component *component)
    {
    struct wm8782_priv *priv = snd_soc_component_get_drvdata(component);
    return regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    }
#[no_mangle]
unsafe extern "C" fn wm8782_soc_remove(component: *mut snd_soc_component) {
    static void wm8782_soc_remove(struct snd_soc_component *component)
    {
    struct wm8782_priv *priv = snd_soc_component_get_drvdata(component);
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    }

#[no_mangle]
unsafe extern "C" fn wm8782_soc_suspend(component: *mut snd_soc_component) -> c_int {
    static int wm8782_soc_suspend(struct snd_soc_component *component)
    {
    struct wm8782_priv *priv = snd_soc_component_get_drvdata(component);
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm8782_soc_resume(component: *mut snd_soc_component) -> c_int {
    static int wm8782_soc_resume(struct snd_soc_component *component)
    {
    struct wm8782_priv *priv = snd_soc_component_get_drvdata(component);
    return regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    }

    static const struct snd_soc_component_driver soc_component_dev_wm8782 = {
    .probe			= wm8782_soc_probe,
    .remove			= wm8782_soc_remove,
    .suspend		= wm8782_soc_suspend,
    .resume			= wm8782_soc_resume,
    .dapm_widgets		= wm8782_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(wm8782_dapm_widgets),
    .dapm_routes		= wm8782_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(wm8782_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn wm8782_probe(pdev: *mut platform_device) -> c_int {
    static int wm8782_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct wm8782_priv *priv;
    int ret, i, fsampen;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(dev, priv);
    for (i = 0; i < ARRAY_SIZE(supply_names); i++)
    priv.supplies[i].supply = supply_names[i];
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(priv.supplies),
    priv.supplies);
    if (ret < 0)
    return ret;
// Assume lowest value by default to avoid inadvertent overclocking
    fsampen = 0;
    if (np)
    of_property_read_u32(np, "wlf,fsampen", &fsampen);
    switch (fsampen) {
    case 0:
    priv.max_rate = 48000;
    break;
    case 1:
    priv.max_rate = 96000;
    break;
    case 2:
    priv.max_rate = 192000;
    break;
    default:
    dev_err(dev, "Invalid wlf,fsampen value");
    return -EINVAL;
    }
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_wm8782, &wm8782_dai, 1);
    }

    static const struct of_device_id wm8782_of_match[] = {
    { .compatible = "wlf,wm8782", },
    { }
    };
    MODULE_DEVICE_TABLE(of, wm8782_of_match);

    static struct platform_driver wm8782_codec_driver = {
    .driver = {
    .name = "wm8782",
    .of_match_table = of_match_ptr(wm8782_of_match),
    },
    .probe = wm8782_probe,
    };
    module_platform_driver(wm8782_codec_driver);
    MODULE_DESCRIPTION("ASoC WM8782 driver");
    MODULE_AUTHOR("Johannes Stezenbach <js@sig21.net>");
    MODULE_LICENSE("GPL");
