//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/boards/cht_bsw_nau8824.c
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
// cht-bsw-nau8824.c - ASoc Machine driver for Intel Cherryview-based
// platforms Cherrytrail and Braswell, with nau8824 codec.
//
// Copyright (C) 2018 Intel Corp
// Copyright (C) 2018 Nuvoton Technology Corp
//
// Author: Wang, Joseph C <joequant@gmail.com>
// Co-author: John Hsu <KCHSU0@nuvoton.com>
// This file is based on cht_bsw_rt5672.c and cht-bsw-max98090.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cht_mc_private {
    pub jack: snd_soc_jack,
}

    static struct snd_soc_jack_pin cht_bsw_jack_pins[] = {
    {
    .pin = "Headphone",
    .mask = SND_JACK_HEADPHONE,
    },
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    };
    static const struct snd_soc_dapm_widget cht_dapm_widgets[] = {
    SND_SOC_DAPM_HP("Headphone", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Int Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Ext Spk", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route cht_audio_map[] = {
    {"Ext Spk", core::ptr::null_mut(), "SPKOUTL"},
    {"Ext Spk", core::ptr::null_mut(), "SPKOUTR"},
    {"Headphone", core::ptr::null_mut(), "HPOL"},
    {"Headphone", core::ptr::null_mut(), "HPOR"},
    {"MIC1", core::ptr::null_mut(), "Int Mic"},
    {"MIC2", core::ptr::null_mut(), "Int Mic"},
    {"HSMIC1", core::ptr::null_mut(), "Headset Mic"},
    {"HSMIC2", core::ptr::null_mut(), "Headset Mic"},
    {"Playback", core::ptr::null_mut(), "ssp2 Tx"},
    {"ssp2 Tx", core::ptr::null_mut(), "codec_out0"},
    {"ssp2 Tx", core::ptr::null_mut(), "codec_out1"},
    {"codec_in0", core::ptr::null_mut(), "ssp2 Rx" },
    {"codec_in1", core::ptr::null_mut(), "ssp2 Rx" },
    {"ssp2 Rx", core::ptr::null_mut(), "Capture"},
    };
    static const struct snd_kcontrol_new cht_mc_controls[] = {
    SOC_DAPM_PIN_SWITCH("Headphone"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("Int Mic"),
    SOC_DAPM_PIN_SWITCH("Ext Spk"),
    };
    static int cht_aif1_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    int ret;
    ret = snd_soc_dai_set_sysclk(codec_dai, NAU8824_CLK_FLL_FS, 0,
    SND_SOC_CLOCK_IN);
    if (ret < 0) {
    dev_err(codec_dai.dev, "can't set FS clock %d\n", ret);
    return ret;
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0, 0, params_rate(params),
    params_rate(params) * 256);
    if (ret < 0) {
    dev_err(codec_dai.dev, "can't set FLL: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cht_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    static int cht_codec_init(struct snd_soc_pcm_runtime *runtime)
    {
    struct cht_mc_private *ctx = snd_soc_card_get_drvdata(runtime.card);
    struct snd_soc_jack *jack = &ctx.jack;
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    struct snd_soc_component *component = codec_dai.component;
    int ret, jack_type;
// NAU88L24 supports 4 buttons headset detection
// KEY_PLAYPAUSE
// KEY_VOICECOMMAND
// KEY_VOLUMEUP
// KEY_VOLUMEDOWN
//
    jack_type = SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 |
    SND_JACK_BTN_2 | SND_JACK_BTN_3;
    ret = snd_soc_card_jack_new_pins(runtime.card, "Headset", jack_type,
    jack, cht_bsw_jack_pins, ARRAY_SIZE(cht_bsw_jack_pins));
    if (ret) {
    dev_err(runtime.dev,
    "Headset Jack creation failed %d\n", ret);
    return ret;
    }
    snd_jack_set_key(jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    nau8824_enable_jack_detect(component, jack);
    return ret;
    }
    static int cht_codec_fixup(struct snd_soc_pcm_runtime *rtd,
    struct snd_pcm_hw_params *params)
    {
    struct snd_interval *rate = hw_param_interval(params,
    SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval *channels = hw_param_interval(params,
    SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_mask *fmt =
    hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    int ret;
// The DSP will convert the FE rate to 48k, stereo, 24bits
    rate.min = rate.max = 48000;
    channels.min = channels.max = 2;
// set SSP2 to 24-bit
    snd_mask_none(fmt);
    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
// TDM 4 slots 24 bit, set Rx & Tx bitmask to 4 active slots
    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_codec(rtd, 0), 0xf, 0x1, 4, 24);
    if (ret < 0) {
    dev_err(rtd.dev, "can't set codec TDM slot %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cht_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    static int cht_aif1_startup(struct snd_pcm_substream *substream)
    {
    return snd_pcm_hw_constraint_single(substream.runtime,
    SNDRV_PCM_HW_PARAM_RATE, 48000);
    }
    static const struct snd_soc_ops cht_aif1_ops = {
    .startup = cht_aif1_startup,
    };
    static const struct snd_soc_ops cht_be_ssp2_ops = {
    .hw_params = cht_aif1_hw_params,
    };
    SND_SOC_DAILINK_DEF(dummy,
    DAILINK_COMP_ARRAY(COMP_DUMMY()));
    SND_SOC_DAILINK_DEF(media,
    DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai")));
    SND_SOC_DAILINK_DEF(deepbuffer,
    DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai")));
    SND_SOC_DAILINK_DEF(ssp2_port,
    DAILINK_COMP_ARRAY(COMP_CPU("ssp2-port")));
    SND_SOC_DAILINK_DEF(ssp2_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10508824:00",
    NAU8824_CODEC_DAI)));
    SND_SOC_DAILINK_DEF(platform,
    DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform")));
    static struct snd_soc_dai_link cht_dailink[] = {
// Front End DAI links
    [MERR_DPCM_AUDIO] = {
    .name = "Audio Port",
    .stream_name = "Audio",
    .nonatomic = true,
    .dynamic = 1,
    .ops = &cht_aif1_ops,
    SND_SOC_DAILINK_REG(media, dummy, platform),
    },
    [MERR_DPCM_DEEP_BUFFER] = {
    .name = "Deep-Buffer Audio Port",
    .stream_name = "Deep-Buffer Audio",
    .nonatomic = true,
    .dynamic = 1,
    .playback_only = 1,
    .ops = &cht_aif1_ops,
    SND_SOC_DAILINK_REG(deepbuffer, dummy, platform),
    },
// Back End DAI links
    {
// SSP2 - Codec
    .name = "SSP2-Codec",
    .id = 0,
    .no_pcm = 1,
    .dai_fmt = SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF
    | SND_SOC_DAIFMT_CBC_CFC,
    .init = cht_codec_init,
    .be_hw_params_fixup = cht_codec_fixup,
    .ops = &cht_be_ssp2_ops,
    SND_SOC_DAILINK_REG(ssp2_port, ssp2_codec, platform),
    },
    };
// use space before codec name to simplify card ID, and simplify driver name

// SoC card
    static struct snd_soc_card snd_soc_card_cht = {
    .owner = THIS_MODULE,
    .dai_link = cht_dailink,
    .num_links = ARRAY_SIZE(cht_dailink),
    .dapm_widgets = cht_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(cht_dapm_widgets),
    .dapm_routes = cht_audio_map,
    .num_dapm_routes = ARRAY_SIZE(cht_audio_map),
    .controls = cht_mc_controls,
    .num_controls = ARRAY_SIZE(cht_mc_controls),
    };
#[no_mangle]
unsafe extern "C" fn snd_cht_mc_probe(pdev: *mut platform_device) -> c_int {
    static int snd_cht_mc_probe(struct platform_device *pdev)
    {
    struct cht_mc_private *drv;
    struct snd_soc_acpi_mach *mach;
    const char *platform_name;
    bool sof_parent;
    int ret_val;
    drv = devm_kzalloc(&pdev.dev, sizeof(*drv), GFP_KERNEL);
    if (!drv)
    return -ENOMEM;
    snd_soc_card_set_drvdata(&snd_soc_card_cht, drv);
// override platform name, if required
    snd_soc_card_cht.dev = &pdev.dev;
    mach = pdev.dev.platform_data;
    platform_name = mach.mach_params.platform;
    ret_val = snd_soc_fixup_dai_links_platform_name(&snd_soc_card_cht,
    platform_name);
    if (ret_val)
    return ret_val;
    sof_parent = snd_soc_acpi_sof_parent(&pdev.dev);
// set card and driver name
    if (sof_parent) {
    snd_soc_card_cht.name = SOF_CARD_NAME;
    snd_soc_card_cht.driver_name = SOF_DRIVER_NAME;
    } else {
    snd_soc_card_cht.name = CARD_NAME;
    snd_soc_card_cht.driver_name = DRIVER_NAME;
    }
    snd_soc_card_cht.components = nau8824_components();
// set pm ops
    if (sof_parent)
    pdev.dev.driver.pm = &snd_soc_pm_ops;
// register the soc card
    ret_val = devm_snd_soc_register_card(&pdev.dev, &snd_soc_card_cht);
    if (ret_val) {
    dev_err(&pdev.dev,
    "snd_soc_register_card failed %d\n", ret_val);
    return ret_val;
    }
    platform_set_drvdata(pdev, &snd_soc_card_cht);
    return ret_val;
    }
    static struct platform_driver snd_cht_mc_driver = {
    .driver = {
    .name = "cht-bsw-nau8824",
    },
    .probe = snd_cht_mc_probe,
    };
    module_platform_driver(snd_cht_mc_driver);
    MODULE_DESCRIPTION("ASoC Intel(R) Baytrail CR Machine driver");
    MODULE_AUTHOR("Wang, Joseph C <joequant@gmail.com>");
    MODULE_AUTHOR("John Hsu <KCHSU0@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:cht-bsw-nau8824");
