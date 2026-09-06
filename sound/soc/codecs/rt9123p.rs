//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/rt9123p.c
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
// rt9123p.c -- RT9123 (HW Mode) ALSA SoC Codec driver
//
// Author: ChiYuan Huang <cy_huang@richtek.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt9123p_priv {
    pub enable: *mut gpio_desc,
    pub enable_delay: c_uint,
    pub enable_switch: c_int,
}

    static int rt9123p_daiops_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *comp = dai.component;
    struct rt9123p_priv *rt9123p = snd_soc_component_get_drvdata(comp);
    if (!rt9123p.enable)
    return 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    mdelay(rt9123p.enable_delay);
    if (rt9123p.enable_switch) {
    gpiod_set_value(rt9123p.enable, 1);
    dev_dbg(comp.dev, "set enable to 1");
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    gpiod_set_value(rt9123p.enable, 0);
    dev_dbg(comp.dev, "set enable to 0");
    break;
    default:
    break;
    }
    return 0;
    }
    static int rt9123p_enable_event(struct snd_soc_dapm_widget *w, struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *comp = snd_soc_dapm_to_component(w.dapm);
    struct rt9123p_priv *rt9123p = snd_soc_component_get_drvdata(comp);
    if (event & SND_SOC_DAPM_POST_PMU)
    rt9123p.enable_switch = 1;
#[no_mangle]
pub unsafe extern "C" fn if(SND_SOC_DAPM_POST_PMD: event &) -> else {
    else if (event & SND_SOC_DAPM_POST_PMD)
    rt9123p.enable_switch = 0;
    return 0;
    }
    static const struct snd_soc_dapm_widget rt9123p_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("SPK"),
    SND_SOC_DAPM_OUT_DRV_E("Amp Drv", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0, rt9123p_enable_event,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    };
    static const struct snd_soc_dapm_route rt9123p_dapm_routes[] = {
    {"Amp Drv", core::ptr::null_mut(), "HiFi Playback"},
    {"SPK", core::ptr::null_mut(), "Amp Drv"},
    };
    static const struct snd_soc_component_driver rt9123p_comp_driver = {
    .dapm_widgets		= rt9123p_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(rt9123p_dapm_widgets),
    .dapm_routes		= rt9123p_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(rt9123p_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct snd_soc_dai_ops rt9123p_dai_ops = {
    .trigger        = rt9123p_daiops_trigger,
    };
    static struct snd_soc_dai_driver rt9123p_dai_driver = {
    .name = "HiFi",
    .playback = {
    .stream_name	= "HiFi Playback",
    .formats	= SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 |
    SNDRV_PCM_FMTBIT_S32,
    .rates		= SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_24000 |
    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000,
    .rate_min	= 8000,
    .rate_max	= 96000,
    .channels_min	= 1,
    .channels_max	= 2,
    },
    .ops    = &rt9123p_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn rt9123p_platform_probe(pdev: *mut platform_device) -> c_int {
    static int rt9123p_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rt9123p_priv *rt9123p;
    int ret;
    rt9123p = devm_kzalloc(dev, sizeof(*rt9123p), GFP_KERNEL);
    if (!rt9123p)
    return -ENOMEM;
    rt9123p.enable = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(rt9123p.enable))
    return PTR_ERR(rt9123p.enable);
    ret = device_property_read_u32(dev, "enable-delay-ms", &rt9123p.enable_delay);
    if (ret) {
    rt9123p.enable_delay = 0;
    dev_dbg(dev, "no optional property 'enable-delay-ms' found, default: no delay\n");
    }
    platform_set_drvdata(pdev, rt9123p);
    return devm_snd_soc_register_component(dev, &rt9123p_comp_driver, &rt9123p_dai_driver, 1);
    }

    static const struct of_device_id rt9123p_device_id[] = {
    { .compatible = "richtek,rt9123p" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt9123p_device_id);

    static const struct acpi_device_id rt9123p_acpi_match[] = {
    { "RT9123P", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, rt9123p_acpi_match);

    static struct platform_driver rt9123p_platform_driver = {
    .driver = {
    .name = "rt9123p",
    .of_match_table = of_match_ptr(rt9123p_device_id),
    .acpi_match_table = ACPI_PTR(rt9123p_acpi_match),
    },
    .probe	= rt9123p_platform_probe,
    };
    module_platform_driver(rt9123p_platform_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("ASoC rt9123p Driver");
    MODULE_LICENSE("GPL");
