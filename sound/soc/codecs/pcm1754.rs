//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm1754.c
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
// PCM1754 DAC ASoC codec driver
//
// Copyright (c) 2022 Alvin Šipraga <alsi@bang-olufsen.dk>
// Copyright (c) 2025 Stefan Kerkmann <s.kerkmann@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm1754_priv {
    pub format: c_uint,
    pub gpiod_mute: *mut gpio_desc,
    pub gpiod_format: *mut gpio_desc,
}

    static int pcm1754_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int format)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct pcm1754_priv *priv = snd_soc_component_get_drvdata(component);
    priv.format = format;
    return 0;
    }
    static int pcm1754_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *codec_dai)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct pcm1754_priv *priv = snd_soc_component_get_drvdata(component);
    int format;
    switch (priv.format & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_RIGHT_J:
    switch (params_width(params)) {
    case 16:
    format = 1;
    break;
    default:
    return -EINVAL;
    }
    break;
    case SND_SOC_DAIFMT_I2S:
    switch (params_width(params)) {
    case 16:
    fallthrough;
    case 24:
    format = 0;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    dev_err(component.dev, "Invalid DAI format\n");
    return -EINVAL;
    }
    gpiod_set_value_cansleep(priv.gpiod_format, format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm1754_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int pcm1754_mute_stream(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct pcm1754_priv *priv = snd_soc_component_get_drvdata(dai.component);
    gpiod_set_value_cansleep(priv.gpiod_mute, mute);
    return 0;
    }
    static const u64 pcm1754_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J;
    static const struct snd_soc_dai_ops pcm1754_dai_ops = {
    .set_fmt = pcm1754_set_dai_fmt,
    .hw_params = pcm1754_hw_params,
    .mute_stream = pcm1754_mute_stream,
    .auto_selectable_formats = &pcm1754_selectable_formats,
    .num_auto_selectable_formats = 1,
    };
    static const struct snd_soc_dai_driver pcm1754_dai = {
    .name = "pcm1754",
    .playback = {
    .stream_name	= "Playback",
    .channels_min	= 2,
    .channels_max	= 2,
    .rates		= SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min	= 5000,
    .rate_max	= 200000,
    .formats	= SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE
    },
    .ops = &pcm1754_dai_ops,
    };
    static const struct snd_soc_dapm_widget pcm1754_dapm_widgets[] = {
    SND_SOC_DAPM_REGULATOR_SUPPLY("VCC", 0, 0),
    SND_SOC_DAPM_DAC("DAC1", "Channel 1 Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC("DAC2", "Channel 2 Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT("VOUTL"),
    SND_SOC_DAPM_OUTPUT("VOUTR"),
    };
    static const struct snd_soc_dapm_route pcm1754_dapm_routes[] = {
    { "DAC1", core::ptr::null_mut(), "Playback" },
    { "DAC2", core::ptr::null_mut(), "Playback" },
    { "DAC1", core::ptr::null_mut(), "VCC" },
    { "DAC2", core::ptr::null_mut(), "VCC" },
    { "VOUTL", core::ptr::null_mut(), "DAC1" },
    { "VOUTR", core::ptr::null_mut(), "DAC2" },
    };
    static const struct snd_soc_component_driver soc_component_dev_pcm1754 = {
    .dapm_widgets = pcm1754_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(pcm1754_dapm_widgets),
    .dapm_routes = pcm1754_dapm_routes,
    .num_dapm_routes = ARRAY_SIZE(pcm1754_dapm_routes),
    };
#[no_mangle]
unsafe extern "C" fn pcm1754_probe(pdev: *mut platform_device) -> c_int {
    static int pcm1754_probe(struct platform_device *pdev)
    {
    struct pcm1754_priv *priv;
    struct device *dev = &pdev.dev;
    struct snd_soc_dai_driver *dai_drv;
    int ret;
    dai_drv = devm_kmemdup(dev, &pcm1754_dai, sizeof(*dai_drv), GFP_KERNEL);
    if (!dai_drv)
    return -ENOMEM;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.gpiod_mute = devm_gpiod_get_optional(dev, "mute", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.gpiod_mute))
    return dev_err_probe(dev, PTR_ERR(priv.gpiod_mute),
    "failed to get mute gpio");
    priv.gpiod_format = devm_gpiod_get_optional(dev, "format", GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_format))
    return dev_err_probe(dev, PTR_ERR(priv.gpiod_format),
    "failed to get format gpio");
    dev_set_drvdata(dev, priv);
    ret = devm_snd_soc_register_component(
    &pdev.dev, &soc_component_dev_pcm1754, dai_drv, 1);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register");
    return 0;
    }

    static const struct of_device_id pcm1754_of_match[] = {
    { .compatible = "ti,pcm1754" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm1754_of_match);

    static struct platform_driver pcm1754_codec_driver = {
    .driver = {
    .name = "pcm1754-codec",
    .of_match_table = of_match_ptr(pcm1754_of_match),
    },
    .probe = pcm1754_probe,
    };
    module_platform_driver(pcm1754_codec_driver);
    MODULE_DESCRIPTION("ASoC PCM1754 driver");
    MODULE_AUTHOR("Alvin Šipraga <alsi@bang-olufsen.dk>");
    MODULE_AUTHOR("Stefan Kerkmann <s.kerkmann@pengutronix.de>");
    MODULE_LICENSE("GPL");
