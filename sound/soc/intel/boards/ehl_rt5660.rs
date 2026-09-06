//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/boards/ehl_rt5660.c
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
// Copyright (c) 2020 Intel Corporation
//
// ehl_rt5660 - ASOC Machine driver for Elkhart Lake platforms
// with rt5660 codec
//

pub const HDMI_LINK_START: c_int = 3;
pub const HDMI_LINE_END: c_int = 6;
pub const IDISP_CODEC_MASK: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_card_private {
    pub hdmi_pcm_list: list_head,
    pub idisp_codec: bool,
}

    static const struct snd_kcontrol_new rt5660_controls[] = {
    SOC_DAPM_PIN_SWITCH("Speaker"),
// There are two MICBIAS in rt5660, each for one MIC
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("Headset Mic2"),
    SOC_DAPM_PIN_SWITCH("Line Out"),
    };
    static const struct snd_soc_dapm_widget rt5660_widgets[] = {
    SND_SOC_DAPM_SPK("Speaker", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic2", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("SoC DMIC", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line Out", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route rt5660_map[] = {
    {"Speaker", core::ptr::null_mut(), "SPO"},
    {"Headset Mic", core::ptr::null_mut(), "MICBIAS1"},
    {"Headset Mic2", core::ptr::null_mut(), "MICBIAS2"},
    {"IN1P", core::ptr::null_mut(), "Headset Mic"},
    {"IN2P", core::ptr::null_mut(), "Headset Mic2"},
    {"Line Out", core::ptr::null_mut(), "LOUTL"},
    {"Line Out", core::ptr::null_mut(), "LOUTR"},
    {"DMic", core::ptr::null_mut(), "SoC DMIC"},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_hdmi_pcm {
    pub head: list_head,
    pub codec_dai: *mut snd_soc_dai,
    pub device: c_int,
}

#[no_mangle]
unsafe extern "C" fn hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int hdmi_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct sof_card_private *ctx = snd_soc_card_get_drvdata(rtd.card);
    struct snd_soc_dai *dai = snd_soc_rtd_to_codec(rtd, 0);
    struct sof_hdmi_pcm *pcm;
    pcm = devm_kzalloc(rtd.card.dev, sizeof(*pcm), GFP_KERNEL);
    if (!pcm)
    return -ENOMEM;
// dai_link id is 1:1 mapped to the PCM device
    pcm.device = rtd.dai_link.id;
    pcm.codec_dai = dai;
    list_add_tail(&pcm.head, &ctx.hdmi_pcm_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn card_late_probe(card: *mut snd_soc_card) -> c_int {
    static int card_late_probe(struct snd_soc_card *card)
    {
    struct sof_card_private *ctx = snd_soc_card_get_drvdata(card);
    struct sof_hdmi_pcm *pcm;
    if (list_empty(&ctx.hdmi_pcm_list))
    return -ENOENT;
    if (!ctx.idisp_codec)
    return 0;
    pcm = list_first_entry(&ctx.hdmi_pcm_list, struct sof_hdmi_pcm, head);
    return hda_dsp_hdmi_build_controls(card, pcm.codec_dai.component);
    }
    static int rt5660_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    int ret;
    ret = snd_soc_dai_set_sysclk(codec_dai,
    RT5660_SCLK_S_PLL1,
    params_rate(params) * 512,
    SND_SOC_CLOCK_IN);
    if (ret < 0) {
    dev_err(rtd.dev, "snd_soc_dai_set_sysclk err = %d\n", ret);
    return ret;
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0,
    RT5660_PLL1_S_BCLK,
    params_rate(params) * 50,
    params_rate(params) * 512);
    if (ret < 0)
    dev_err(rtd.dev, "can't set codec pll: %d\n", ret);
    return ret;
    }
    static const struct snd_soc_ops rt5660_ops = {
    .hw_params = rt5660_hw_params,
    };
    SND_SOC_DAILINK_DEF(ssp0_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("SSP0 Pin")));
    SND_SOC_DAILINK_DEF(rt5660_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5660:00", "rt5660-aif1")));
    SND_SOC_DAILINK_DEF(platform,
    DAILINK_COMP_ARRAY(COMP_PLATFORM("0000:00:1f.3")));
    SND_SOC_DAILINK_DEF(dmic_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("DMIC01 Pin")));
    SND_SOC_DAILINK_DEF(dmic_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec", "dmic-hifi")));
    SND_SOC_DAILINK_DEF(dmic16k,
    DAILINK_COMP_ARRAY(COMP_CPU("DMIC16k Pin")));
    SND_SOC_DAILINK_DEF(idisp1_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("iDisp1 Pin")));
    SND_SOC_DAILINK_DEF(idisp1_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi1")));
    SND_SOC_DAILINK_DEF(idisp2_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("iDisp2 Pin")));
    SND_SOC_DAILINK_DEF(idisp2_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi2")));
    SND_SOC_DAILINK_DEF(idisp3_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("iDisp3 Pin")));
    SND_SOC_DAILINK_DEF(idisp3_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi3")));
    SND_SOC_DAILINK_DEF(idisp4_pin,
    DAILINK_COMP_ARRAY(COMP_CPU("iDisp4 Pin")));
    SND_SOC_DAILINK_DEF(idisp4_codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi4")));
    static struct snd_soc_dai_link ehl_rt5660_dailink[] = {
// back ends
    {
    .name = "SSP0-Codec",
    .id = 0,
    .no_pcm = 1,
    .ops = &rt5660_ops,
    SND_SOC_DAILINK_REG(ssp0_pin, rt5660_codec, platform),
    },
    {
    .name = "dmic48k",
    .id = 1,
    .ignore_suspend = 1,
    .capture_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(dmic_pin, dmic_codec, platform),
    },
    {
    .name = "dmic16k",
    .id = 2,
    .ignore_suspend = 1,
    .capture_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(dmic16k, dmic_codec, platform),
    },
    {
    .name = "iDisp1",
    .id = 5,
    .init = hdmi_init,
    .playback_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(idisp1_pin, idisp1_codec, platform),
    },
    {
    .name = "iDisp2",
    .id = 6,
    .init = hdmi_init,
    .playback_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(idisp2_pin, idisp2_codec, platform),
    },
    {
    .name = "iDisp3",
    .id = 7,
    .init = hdmi_init,
    .playback_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(idisp3_pin, idisp3_codec, platform),
    },
    {
    .name = "iDisp4",
    .id = 8,
    .init = hdmi_init,
    .playback_only = 1,
    .no_pcm = 1,
    SND_SOC_DAILINK_REG(idisp4_pin, idisp4_codec, platform),
    },
    };
// SoC card
    static struct snd_soc_card snd_soc_card_ehl_rt5660 = {
    .name = "ehl-rt5660",
    .owner = THIS_MODULE,
    .dai_link = ehl_rt5660_dailink,
    .num_links = ARRAY_SIZE(ehl_rt5660_dailink),
    .dapm_widgets = rt5660_widgets,
    .num_dapm_widgets = ARRAY_SIZE(rt5660_widgets),
    .dapm_routes = rt5660_map,
    .num_dapm_routes = ARRAY_SIZE(rt5660_map),
    .controls = rt5660_controls,
    .num_controls = ARRAY_SIZE(rt5660_controls),
    .fully_routed = true,
    .late_probe = card_late_probe,
    };
// If hdmi codec is not supported, switch to use dummy codec
    static void hdmi_link_init(struct snd_soc_card *card,
    struct sof_card_private *ctx,
    struct snd_soc_acpi_mach *mach)
    {
    int i;
    if (mach.mach_params.codec_mask & IDISP_CODEC_MASK) {
    ctx.idisp_codec = true;
    return;
    }
//
// if HDMI is not enabled in kernel config, or
// hdmi codec is not supported
//
    for (i = HDMI_LINK_START; i <= HDMI_LINE_END; i++)
    card.dai_link[i].codecs[0] = snd_soc_dummy_dlc;
    }
#[no_mangle]
unsafe extern "C" fn snd_ehl_rt5660_probe(pdev: *mut platform_device) -> c_int {
    static int snd_ehl_rt5660_probe(struct platform_device *pdev)
    {
    struct snd_soc_acpi_mach *mach;
    struct snd_soc_card *card = &snd_soc_card_ehl_rt5660;
    struct sof_card_private *ctx;
    int ret;
    card.dev = &pdev.dev;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    INIT_LIST_HEAD(&ctx.hdmi_pcm_list);
    snd_soc_card_set_drvdata(card, ctx);
    mach = pdev.dev.platform_data;
    ret = snd_soc_fixup_dai_links_platform_name(card,
    mach.mach_params.platform);
    if (ret)
    return ret;
    hdmi_link_init(card, ctx, mach);
    return devm_snd_soc_register_card(&pdev.dev, card);
    }
    static const struct platform_device_id ehl_board_ids[] = {
    { .name = "ehl_rt5660" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, ehl_board_ids);
    static struct platform_driver snd_ehl_rt5660_driver = {
    .driver = {
    .name = "ehl_rt5660",
    .pm = &snd_soc_pm_ops,
    },
    .probe = snd_ehl_rt5660_probe,
    .id_table = ehl_board_ids,
    };
    module_platform_driver(snd_ehl_rt5660_driver);
    MODULE_DESCRIPTION("ASoC Intel(R) Elkhartlake + rt5660 Machine driver");
    MODULE_AUTHOR("libin.yang@intel.com");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON");
