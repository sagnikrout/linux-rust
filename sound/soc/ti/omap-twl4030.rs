//! Automatically rewritten from C to Rust
//! Source: sound/soc/ti/omap-twl4030.c
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
// omap-twl4030.c  --  SoC audio for TI SoC based boards with twl4030 codec
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
// All rights reserved.
//
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//
// This driver replaces the following machine drivers:
// omap3beagle (Author: Steve Sakoman <steve@sakoman.com>)
// omap3evm (Author: Anuj Aggarwal <anuj.aggarwal@ti.com>)
// overo (Author: Steve Sakoman <steve@sakoman.com>)
// igep0020 (Author: Enric Balletbo i Serra <eballetbo@iseebcn.com>)
// zoom2 (Author: Misael Lopez Cruz <misael.lopez@ti.com>)
// sdp3430 (Author: Misael Lopez Cruz <misael.lopez@ti.com>)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_twl4030 {
    pub hs_jack_gpio: snd_soc_jack_gpio,
    pub hs_jack: snd_soc_jack,
}

    static int omap_twl4030_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    unsigned int fmt;
    switch (params_channels(params)) {
    case 2: /* Stereo I2S mode */
    fmt =	SND_SOC_DAIFMT_I2S |
    SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBP_CFP;
    break;
    case 4: /* Four channel TDM mode */
    fmt =	SND_SOC_DAIFMT_DSP_A |
    SND_SOC_DAIFMT_IB_NF |
    SND_SOC_DAIFMT_CBP_CFP;
    break;
    default:
    return -EINVAL;
    }
    return snd_soc_runtime_set_dai_fmt(rtd, fmt);
    }
    static const struct snd_soc_ops omap_twl4030_ops = {
    .hw_params = omap_twl4030_hw_params,
    };
    static const struct snd_soc_dapm_widget dapm_widgets[] = {
    SND_SOC_DAPM_SPK("Earpiece Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Handsfree Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_HP("Headset Stereophone", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Ext Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Carkit Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Main Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Sub Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Carkit Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Digital0 Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Digital1 Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line In", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route audio_map[] = {
// Headset Stereophone:  HSOL, HSOR
    {"Headset Stereophone", core::ptr::null_mut(), "HSOL"},
    {"Headset Stereophone", core::ptr::null_mut(), "HSOR"},
// External Speakers: HFL, HFR
    {"Handsfree Spk", core::ptr::null_mut(), "HFL"},
    {"Handsfree Spk", core::ptr::null_mut(), "HFR"},
// External Speakers: PredrivL, PredrivR
    {"Ext Spk", core::ptr::null_mut(), "PREDRIVEL"},
    {"Ext Spk", core::ptr::null_mut(), "PREDRIVER"},
// Carkit speakers:  CARKITL, CARKITR
    {"Carkit Spk", core::ptr::null_mut(), "CARKITL"},
    {"Carkit Spk", core::ptr::null_mut(), "CARKITR"},
// Earpiece
    {"Earpiece Spk", core::ptr::null_mut(), "EARPIECE"},
// External Mics: MAINMIC, SUBMIC with bias
    {"MAINMIC", core::ptr::null_mut(), "Main Mic"},
    {"Main Mic", core::ptr::null_mut(), "Mic Bias 1"},
    {"SUBMIC", core::ptr::null_mut(), "Sub Mic"},
    {"Sub Mic", core::ptr::null_mut(), "Mic Bias 2"},
// Headset Mic: HSMIC with bias
    {"HSMIC", core::ptr::null_mut(), "Headset Mic"},
    {"Headset Mic", core::ptr::null_mut(), "Headset Mic Bias"},
// Digital Mics: DIGIMIC0, DIGIMIC1 with bias
    {"DIGIMIC0", core::ptr::null_mut(), "Digital0 Mic"},
    {"Digital0 Mic", core::ptr::null_mut(), "Mic Bias 1"},
    {"DIGIMIC1", core::ptr::null_mut(), "Digital1 Mic"},
    {"Digital1 Mic", core::ptr::null_mut(), "Mic Bias 2"},
// Carkit In: CARKITMIC
    {"CARKITMIC", core::ptr::null_mut(), "Carkit Mic"},
// Aux In: AUXL, AUXR
    {"AUXL", core::ptr::null_mut(), "Line In"},
    {"AUXR", core::ptr::null_mut(), "Line In"},
    };
// Headset jack detection DAPM pins
    static struct snd_soc_jack_pin hs_jack_pins[] = {
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    {
    .pin = "Headset Stereophone",
    .mask = SND_JACK_HEADPHONE,
    },
    };
#[no_mangle]
unsafe extern "C" fn omap_twl4030_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int omap_twl4030_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_card *card = rtd.card;
    struct omap_twl4030 *priv = snd_soc_card_get_drvdata(card);
    int ret;
//
// This is a bit of a hack, but the GPIO is optional so we
// only want to add the jack detection if the GPIO is there.
//
    if (of_property_present(card.dev.of_node, "ti,jack-det-gpio")) {
    ret = snd_soc_card_jack_new_pins(rtd.card, "Headset Jack",
    SND_JACK_HEADSET,
    &priv.hs_jack, hs_jack_pins,
    ARRAY_SIZE(hs_jack_pins));
    if (ret)
    return ret;
    priv.hs_jack_gpio.name = "ti,jack-det";
    priv.hs_jack_gpio.report = SND_JACK_HEADSET;
    priv.hs_jack_gpio.debounce_time = 200;
    priv.hs_jack_gpio.gpiod_dev = card.dev;
    priv.hs_jack_gpio.idx = 0;
    ret = snd_soc_jack_add_gpios(&priv.hs_jack, 1,
    &priv.hs_jack_gpio);
    if (ret)
    return ret;
    }
    return 0;
    }
// Digital audio interface glue - connects codec <--> CPU
    SND_SOC_DAILINK_DEFS(hifi,
    DAILINK_COMP_ARRAY(COMP_CPU("omap-mcbsp.2")),
    DAILINK_COMP_ARRAY(COMP_CODEC("twl4030-codec", "twl4030-hifi")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("omap-mcbsp.2")));
    SND_SOC_DAILINK_DEFS(voice,
    DAILINK_COMP_ARRAY(COMP_CPU("omap-mcbsp.3")),
    DAILINK_COMP_ARRAY(COMP_CODEC("twl4030-codec", "twl4030-voice")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("omap-mcbsp.3")));
    static struct snd_soc_dai_link omap_twl4030_dai_links[] = {
    {
    .name = "TWL4030 HiFi",
    .stream_name = "TWL4030 HiFi",
    .init = omap_twl4030_init,
    .ops = &omap_twl4030_ops,
    SND_SOC_DAILINK_REG(hifi),
    },
    {
    .name = "TWL4030 Voice",
    .stream_name = "TWL4030 Voice",
    .dai_fmt = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF |
    SND_SOC_DAIFMT_CBP_CFP,
    SND_SOC_DAILINK_REG(voice),
    },
    };
// Audio machine driver
    static struct snd_soc_card omap_twl4030_card = {
    .owner = THIS_MODULE,
    .dai_link = omap_twl4030_dai_links,
    .num_links = ARRAY_SIZE(omap_twl4030_dai_links),
    .dapm_widgets = dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(dapm_widgets),
    .dapm_routes = audio_map,
    .num_dapm_routes = ARRAY_SIZE(audio_map),
    };
#[no_mangle]
unsafe extern "C" fn omap_twl4030_probe(pdev: *mut platform_device) -> c_int {
    static int omap_twl4030_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &omap_twl4030_card;
    struct device_node *node, *dai_node;
    struct omap_twl4030 *priv;
    struct property *prop;
    int ret;
    node = pdev.dev.of_node;
    if (!node)
    return -ENODEV;
    card.dev = &pdev.dev;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct omap_twl4030), GFP_KERNEL);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    ret = snd_soc_of_parse_card_name(card, "ti,model");
    if (ret)
    return ret;
    if (!card.name) {
    dev_err(&pdev.dev, "Card name is not provided\n");
    return -ENODEV;
    }
    dai_node = of_parse_phandle(node, "ti,mcbsp", 0);
    if (!dai_node) {
    dev_err(&pdev.dev, "McBSP node is not provided\n");
    return -EINVAL;
    }
    omap_twl4030_dai_links[0].cpus.dai_name  = core::ptr::null_mut();
    omap_twl4030_dai_links[0].cpus.of_node = dai_node;
    omap_twl4030_dai_links[0].platforms.name  = core::ptr::null_mut();
    omap_twl4030_dai_links[0].platforms.of_node = dai_node;
    dai_node = of_parse_phandle(node, "ti,mcbsp-voice", 0);
    if (!dai_node) {
    card.num_links = 1;
    } else {
    omap_twl4030_dai_links[1].cpus.dai_name  = core::ptr::null_mut();
    omap_twl4030_dai_links[1].cpus.of_node = dai_node;
    omap_twl4030_dai_links[1].platforms.name  = core::ptr::null_mut();
    omap_twl4030_dai_links[1].platforms.of_node = dai_node;
    }
// Optional: audio routing can be provided
    prop = of_find_property(node, "ti,audio-routing", core::ptr::null_mut());
    if (prop) {
    ret = snd_soc_of_parse_audio_routing(card, "ti,audio-routing");
    if (ret)
    return ret;
    card.fully_routed = 1;
    }
    snd_soc_card_set_drvdata(card, priv);
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "devm_snd_soc_register_card() failed\n");
    return 0;
    }
    static const struct of_device_id omap_twl4030_of_match[] = {
    {.compatible = "ti,omap-twl4030", },
    { },
    };
    MODULE_DEVICE_TABLE(of, omap_twl4030_of_match);
    static struct platform_driver omap_twl4030_driver = {
    .driver = {
    .name = "omap-twl4030",
    .pm = &snd_soc_pm_ops,
    .of_match_table = omap_twl4030_of_match,
    },
    .probe = omap_twl4030_probe,
    };
    module_platform_driver(omap_twl4030_driver);
    MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
    MODULE_DESCRIPTION("ALSA SoC for TI SoC based boards with twl4030 codec");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:omap-twl4030");
