//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/es7134.c
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
// Copyright (c) 2017 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

//
// The everest 7134 is a very simple DA converter with no register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct es7134_clock_mode {
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub mclk_fs: *mut c_uint,
    pub mclk_fs_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es7134_chip {
    pub dai_drv: *mut snd_soc_dai_driver,
    pub modes: *const es7134_clock_mode,
    pub mode_num: c_uint,
    pub extra_widgets: *const snd_soc_dapm_widget,
    pub extra_widget_num: c_uint,
    pub extra_routes: *const snd_soc_dapm_route,
    pub extra_route_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es7134_data {
    pub mclk: c_uint,
    pub chip: *const es7134_chip,
}

    static int es7134_check_mclk(struct snd_soc_dai *dai,
    struct es7134_data *priv,
    unsigned int rate)
    {
    let mut mfs: c_uint = priv.mclk / rate;
    int i, j;
    for (i = 0; i < priv.chip.mode_num; i++) {
    const struct es7134_clock_mode *mode = &priv.chip.modes[i];
    if (rate < mode.rate_min || rate > mode.rate_max)
    continue;
    for (j = 0; j < mode.mclk_fs_num; j++) {
    if (mode.mclk_fs[j] == mfs)
    return 0;
    }
    dev_err(dai.dev, "unsupported mclk_fs %u for rate %u\n",
    mfs, rate);
    return -EINVAL;
    }
// should not happen
    dev_err(dai.dev, "unsupported rate: %u\n", rate);
    return -EINVAL;
    }
    static int es7134_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct es7134_data *priv = snd_soc_dai_get_drvdata(dai);
// mclk has not been provided, assume it is OK
    if (!priv.mclk)
    return 0;
    return es7134_check_mclk(dai, priv, params_rate(params));
    }
    static int es7134_set_sysclk(struct snd_soc_dai *dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct es7134_data *priv = snd_soc_dai_get_drvdata(dai);
    if (dir == SND_SOC_CLOCK_IN && clk_id == 0) {
    priv.mclk = freq;
    return 0;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn es7134_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int es7134_set_fmt(struct snd_soc_dai *codec_dai, unsigned int fmt)
    {
    fmt &= (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK |
    SND_SOC_DAIFMT_MASTER_MASK);
    if (fmt != (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBC_CFC)) {
    dev_err(codec_dai.dev, "Invalid DAI format\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn es7134_component_probe(c: *mut snd_soc_component) -> c_int {
    static int es7134_component_probe(struct snd_soc_component *c)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(c);
    struct es7134_data *priv = snd_soc_component_get_drvdata(c);
    const struct es7134_chip *chip = priv.chip;
    int ret;
    if (chip.extra_widget_num) {
    ret = snd_soc_dapm_new_controls(dapm, chip.extra_widgets,
    chip.extra_widget_num);
    if (ret) {
    dev_err(c.dev, "failed to add extra widgets\n");
    return ret;
    }
    }
    if (chip.extra_route_num) {
    ret = snd_soc_dapm_add_routes(dapm, chip.extra_routes,
    chip.extra_route_num);
    if (ret) {
    dev_err(c.dev, "failed to add extra routes\n");
    return ret;
    }
    }
    return 0;
    }
    static const struct snd_soc_dai_ops es7134_dai_ops = {
    .set_fmt	= es7134_set_fmt,
    .hw_params	= es7134_hw_params,
    .set_sysclk	= es7134_set_sysclk,
    };
    static struct snd_soc_dai_driver es7134_dai = {
    .name = "es7134-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = (SNDRV_PCM_RATE_8000_48000 |
    SNDRV_PCM_RATE_88200      |
    SNDRV_PCM_RATE_96000      |
    SNDRV_PCM_RATE_176400     |
    SNDRV_PCM_RATE_192000),
    .formats = (SNDRV_PCM_FMTBIT_S16_LE  |
    SNDRV_PCM_FMTBIT_S18_3LE |
    SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_3LE |
    SNDRV_PCM_FMTBIT_S24_LE),
    },
    .ops = &es7134_dai_ops,
    };
    static const struct es7134_clock_mode es7134_modes[] = {
    {
// Single speed mode
    .rate_min = 8000,
    .rate_max = 50000,
    .mclk_fs = (unsigned int[]) { 256, 384, 512, 768, 1024 },
    .mclk_fs_num = 5,
    }, {
// Double speed mode
    .rate_min = 84000,
    .rate_max = 100000,
    .mclk_fs = (unsigned int[]) { 128, 192, 256, 384, 512 },
    .mclk_fs_num = 5,
    }, {
// Quad speed mode
    .rate_min = 167000,
    .rate_max = 192000,
    .mclk_fs = (unsigned int[]) { 128, 192, 256 },
    .mclk_fs_num = 3,
    },
    };
// Digital I/O are also supplied by VDD on the es7134
    static const struct snd_soc_dapm_route es7134_extra_routes[] = {
    { "Playback", core::ptr::null_mut(), "VDD", }
    };
    static const struct es7134_chip es7134_chip __maybe_unused = {
    .dai_drv = &es7134_dai,
    .modes = es7134_modes,
    .mode_num = ARRAY_SIZE(es7134_modes),
    .extra_routes = es7134_extra_routes,
    .extra_route_num = ARRAY_SIZE(es7134_extra_routes),
    };
    static const struct snd_soc_dapm_widget es7134_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("AOUTL"),
    SND_SOC_DAPM_OUTPUT("AOUTR"),
    SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_REGULATOR_SUPPLY("VDD", 0, 0),
    };
    static const struct snd_soc_dapm_route es7134_dapm_routes[] = {
    { "AOUTL", core::ptr::null_mut(), "DAC" },
    { "AOUTR", core::ptr::null_mut(), "DAC" },
    { "DAC", core::ptr::null_mut(), "VDD" },
    };
    static const struct snd_soc_component_driver es7134_component_driver = {
    .probe			= es7134_component_probe,
    .dapm_widgets		= es7134_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(es7134_dapm_widgets),
    .dapm_routes		= es7134_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(es7134_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static struct snd_soc_dai_driver es7154_dai = {
    .name = "es7154-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = (SNDRV_PCM_RATE_8000_48000 |
    SNDRV_PCM_RATE_88200      |
    SNDRV_PCM_RATE_96000),
    .formats = (SNDRV_PCM_FMTBIT_S16_LE  |
    SNDRV_PCM_FMTBIT_S18_3LE |
    SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_3LE |
    SNDRV_PCM_FMTBIT_S24_LE),
    },
    .ops = &es7134_dai_ops,
    };
    static const struct es7134_clock_mode es7154_modes[] = {
    {
// Single speed mode
    .rate_min = 8000,
    .rate_max = 50000,
    .mclk_fs = (unsigned int[]) { 32, 64, 128, 192, 256,
    384, 512, 768, 1024 },
    .mclk_fs_num = 9,
    }, {
// Double speed mode
    .rate_min = 84000,
    .rate_max = 100000,
    .mclk_fs = (unsigned int[]) { 128, 192, 256, 384, 512,
    768, 1024},
    .mclk_fs_num = 7,
    }
    };
// Es7154 has a separate supply for digital I/O
    static const struct snd_soc_dapm_widget es7154_extra_widgets[] = {
    SND_SOC_DAPM_REGULATOR_SUPPLY("PVDD", 0, 0),
    };
    static const struct snd_soc_dapm_route es7154_extra_routes[] = {
    { "Playback", core::ptr::null_mut(), "PVDD", }
    };
    static const struct es7134_chip es7154_chip __maybe_unused = {
    .dai_drv = &es7154_dai,
    .modes = es7154_modes,
    .mode_num = ARRAY_SIZE(es7154_modes),
    .extra_routes = es7154_extra_routes,
    .extra_route_num = ARRAY_SIZE(es7154_extra_routes),
    .extra_widgets = es7154_extra_widgets,
    .extra_widget_num = ARRAY_SIZE(es7154_extra_widgets),
    };
#[no_mangle]
unsafe extern "C" fn es7134_probe(pdev: *mut platform_device) -> c_int {
    static int es7134_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct es7134_data *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.chip = of_device_get_match_data(dev);
    if (!priv.chip) {
    dev_err(dev, "failed to match device\n");
    return -ENODEV;
    }
    return devm_snd_soc_register_component(&pdev.dev,
    &es7134_component_driver,
    priv.chip.dai_drv, 1);
    }

    static const struct of_device_id es7134_ids[] = {
    { .compatible = "everest,es7134", .data = &es7134_chip },
    { .compatible = "everest,es7144", .data = &es7134_chip },
    { .compatible = "everest,es7154", .data = &es7154_chip },
    { }
    };
    MODULE_DEVICE_TABLE(of, es7134_ids);

    static struct platform_driver es7134_driver = {
    .driver = {
    .name = "es7134",
    .of_match_table = of_match_ptr(es7134_ids),
    },
    .probe = es7134_probe,
    };
    module_platform_driver(es7134_driver);
    MODULE_DESCRIPTION("ASoC ES7134 audio codec driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL");
