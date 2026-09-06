//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/boards/rt5682.c
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

// Default: MCLK on, MCLK 19.2M, SSP0
    let mut avs_rt5682_quirk: static unsigned long = AVS_RT5682_MCLK_EN | AVS_RT5682_SSP_CODEC(0);
#[no_mangle]
unsafe extern "C" fn avs_rt5682_quirk_cb(id: *const dmi_system_id) -> c_int {
    static int avs_rt5682_quirk_cb(const struct dmi_system_id *id)
    {
    avs_rt5682_quirk = (unsigned long)id.driver_data;
    return 1;
    }
    static const struct dmi_system_id avs_rt5682_quirk_table[] = {
    {
    .callback = avs_rt5682_quirk_cb,
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "WhiskeyLake Client"),
    },
    .driver_data = (void *)(AVS_RT5682_MCLK_EN |
    AVS_RT5682_MCLK_24MHZ |
    AVS_RT5682_SSP_CODEC(1)),
    },
    {
    .callback = avs_rt5682_quirk_cb,
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Ice Lake Client"),
    },
    .driver_data = (void *)(AVS_RT5682_MCLK_EN |
    AVS_RT5682_SSP_CODEC(0)),
    },
    {}
    };
    static const struct snd_kcontrol_new card_controls[] = {
    SOC_DAPM_PIN_SWITCH("Headphone Jack"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    };
    static const struct snd_soc_dapm_widget card_widgets[] = {
    SND_SOC_DAPM_HP("Headphone Jack", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route card_base_routes[] = {
// HP jack connectors - unknown if we have jack detect
    { "Headphone Jack", core::ptr::null_mut(), "HPOL" },
    { "Headphone Jack", core::ptr::null_mut(), "HPOR" },
// other jacks
    { "IN1P", core::ptr::null_mut(), "Headset Mic" },
    };
    static const struct snd_soc_jack_pin card_jack_pins[] = {
    {
    .pin = "Headphone Jack",
    .mask = SND_JACK_HEADPHONE,
    },
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    };
#[no_mangle]
unsafe extern "C" fn avs_rt5682_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    static int avs_rt5682_codec_init(struct snd_soc_pcm_runtime *runtime)
    {
    struct snd_soc_component *component = snd_soc_rtd_to_codec(runtime, 0).component;
    struct snd_soc_card *card = runtime.card;
    struct snd_soc_jack_pin *pins;
    struct snd_soc_jack *jack;
    int num_pins, ret;
    jack = snd_soc_card_get_drvdata(card);
    num_pins = ARRAY_SIZE(card_jack_pins);
    pins = devm_kmemdup_array(card.dev, card_jack_pins, num_pins,
    sizeof(card_jack_pins[0]), GFP_KERNEL);
    if (!pins)
    return -ENOMEM;
// Need to enable ASRC function for 24MHz mclk rate
    if ((avs_rt5682_quirk & AVS_RT5682_MCLK_EN) &&
    (avs_rt5682_quirk & AVS_RT5682_MCLK_24MHZ)) {
    rt5682_sel_asrc_clk_src(component, RT5682_DA_STEREO1_FILTER |
    RT5682_AD_STEREO1_FILTER, RT5682_CLK_SEL_I2S1_ASRC);
    }
    ret = snd_soc_card_jack_new_pins(card, "Headset Jack", SND_JACK_HEADSET | SND_JACK_BTN_0 |
    SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3, jack,
    pins, num_pins);
    if (ret) {
    dev_err(card.dev, "Headset Jack creation failed: %d\n", ret);
    return ret;
    }
    snd_jack_set_key(jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if (ret) {
    dev_err(card.dev, "Headset Jack call-back failed: %d\n", ret);
    return ret;
    }
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn avs_rt5682_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    static void avs_rt5682_codec_exit(struct snd_soc_pcm_runtime *rtd)
    {
    snd_soc_component_set_jack(snd_soc_rtd_to_codec(rtd, 0).component, core::ptr::null_mut(), core::ptr::null_mut());
    }
    static int
    avs_rt5682_hw_params(struct snd_pcm_substream *substream, struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *runtime = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    int pll_source, freq_in, freq_out;
    int ret;
    if (avs_rt5682_quirk & AVS_RT5682_MCLK_EN) {
    pll_source = RT5682_PLL1_S_MCLK;
    if (avs_rt5682_quirk & AVS_RT5682_MCLK_24MHZ)
    freq_in = 24000000;
    else
    freq_in = 19200000;
    } else {
    pll_source = RT5682_PLL1_S_BCLK1;
    freq_in = params_rate(params) * 50;
    }
    freq_out = params_rate(params) * 512;
    ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL1, pll_source, freq_in, freq_out);
    if (ret < 0)
    dev_err(runtime.dev, "Set PLL failed: %d\n", ret);
    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL1, freq_out, SND_SOC_CLOCK_IN);
    if (ret < 0)
    dev_err(runtime.dev, "Set sysclk failed: %d\n", ret);
// slot_width should be equal or larger than data length.
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x0, 0x0, 2, params_width(params));
    if (ret < 0)
    dev_err(runtime.dev, "Set TDM slot failed: %d\n", ret);
    return ret;
    }
    static const struct snd_soc_ops avs_rt5682_ops = {
    .hw_params = avs_rt5682_hw_params,
    };
    static int
    avs_rt5682_be_fixup(struct snd_soc_pcm_runtime *runtime, struct snd_pcm_hw_params *params)
    {
    struct snd_interval *rate, *channels;
    struct snd_mask *fmt;
    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
// The ADSP will convert the FE rate to 48k, stereo
    rate.min = rate.max = 48000;
    channels.min = channels.max = 2;
// set SSPN to 24 bit
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
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
    dl.codecs.name = devm_kasprintf(dev, GFP_KERNEL, "i2c-10EC5682:00");
    dl.codecs.dai_name = devm_kasprintf(dev, GFP_KERNEL, AVS_RT5682_CODEC_DAI_NAME);
    if (!dl.cpus.dai_name || !dl.codecs.name || !dl.codecs.dai_name)
    return -ENOMEM;
    platform.name = dev_name(dev);
    dl.num_cpus = 1;
    dl.num_codecs = 1;
    dl.platforms = platform;
    dl.num_platforms = 1;
    dl.id = 0;
    dl.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    dl.init = avs_rt5682_codec_init;
    dl.exit = avs_rt5682_codec_exit;
    dl.be_hw_params_fixup = avs_rt5682_be_fixup;
    dl.ops = &avs_rt5682_ops;
    dl.nonatomic = 1;
    dl.no_pcm = 1;
// dai_link = dl;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    static int avs_card_suspend_pre(struct snd_soc_card *card)
    {
    struct snd_soc_dai *codec_dai = snd_soc_card_get_codec_dai(card, AVS_RT5682_CODEC_DAI_NAME);
    return snd_soc_component_set_jack(codec_dai.component, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    static int avs_card_resume_post(struct snd_soc_card *card)
    {
    struct snd_soc_dai *codec_dai = snd_soc_card_get_codec_dai(card, AVS_RT5682_CODEC_DAI_NAME);
    struct snd_soc_jack *jack = snd_soc_card_get_drvdata(card);
    return snd_soc_component_set_jack(codec_dai.component, jack, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn avs_rt5682_probe(pdev: *mut platform_device) -> c_int {
    static int avs_rt5682_probe(struct platform_device *pdev)
    {
    struct snd_soc_dai_link *dai_link;
    struct snd_soc_acpi_mach *mach;
    struct avs_mach_pdata *pdata;
    struct snd_soc_card *card;
    struct snd_soc_jack *jack;
    struct device *dev = &pdev.dev;
    int ssp_port, tdm_slot, ret;
    if (pdev.id_entry && pdev.id_entry.driver_data)
    avs_rt5682_quirk = (unsigned long)pdev.id_entry.driver_data;
    dmi_check_system(avs_rt5682_quirk_table);
    dev_dbg(dev, "avs_rt5682_quirk = %lx\n", avs_rt5682_quirk);
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
    jack = devm_kzalloc(dev, sizeof(*jack), GFP_KERNEL);
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!jack || !card)
    return -ENOMEM;
    if (pdata.obsolete_card_names) {
    card.name = "avs_rt5682";
    } else {
    card.driver_name = "avs_rt5682";
    card.long_name = card.name = "AVS I2S ALC5682";
    }
    card.dev = dev;
    card.owner = THIS_MODULE;
    card.suspend_pre = avs_card_suspend_pre;
    card.resume_post = avs_card_resume_post;
    card.dai_link = dai_link;
    card.num_links = 1;
    card.controls = card_controls;
    card.num_controls = ARRAY_SIZE(card_controls);
    card.dapm_widgets = card_widgets;
    card.num_dapm_widgets = ARRAY_SIZE(card_widgets);
    card.dapm_routes = card_base_routes;
    card.num_dapm_routes = ARRAY_SIZE(card_base_routes);
    card.fully_routed = true;
    snd_soc_card_set_drvdata(card, jack);
    return devm_snd_soc_register_deferrable_card(dev, card);
    }
    static const struct platform_device_id avs_rt5682_driver_ids[] = {
    {
    .name = "avs_rt5682",
    },
    { }
    };
    MODULE_DEVICE_TABLE(platform, avs_rt5682_driver_ids);
    static struct platform_driver avs_rt5682_driver = {
    .probe = avs_rt5682_probe,
    .driver = {
    .name = "avs_rt5682",
    .pm = &snd_soc_pm_ops,
    },
    .id_table = avs_rt5682_driver_ids,
    };
    module_platform_driver(avs_rt5682_driver)
    MODULE_DESCRIPTION("Intel rt5682 machine driver");
    MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
    MODULE_LICENSE("GPL");
