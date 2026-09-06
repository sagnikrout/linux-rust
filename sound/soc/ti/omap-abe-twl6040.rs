//! Automatically rewritten from C to Rust
//! Source: sound/soc/ti/omap-abe-twl6040.c
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
// omap-abe-twl6040.c  --  SoC audio for TI OMAP based boards with ABE and
// twl6040 codec
//
// Author: Misael Lopez Cruz <misael.lopez@ti.com>
//

    SND_SOC_DAILINK_DEFS(link0,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC("twl6040-codec",
    "twl6040-legacy")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(link1,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec",
    "dmic-hifi")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abe_twl6040 {
    pub card: snd_soc_card,
    pub dai_links: [snd_soc_dai_link; 2],
    pub /: *mut *mut int jack_detection; / board can detect jack events,
    pub /: *mut *mut int mclk_freq; / MCLK frequency speed for twl6040,
}

    static struct platform_device *dmic_codec_dev;
    static int omap_abe_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    struct snd_soc_card *card = rtd.card;
    struct abe_twl6040 *priv = snd_soc_card_get_drvdata(card);
    int clk_id, freq;
    int ret;
    clk_id = twl6040_get_clk_id(codec_dai.component);
    if (clk_id == TWL6040_SYSCLK_SEL_HPPLL)
    freq = priv.mclk_freq;
#[no_mangle]
pub unsafe extern "C" fn if(TWL6040_SYSCLK_SEL_LPPLL: clk_id ==) -> else {
    else if (clk_id == TWL6040_SYSCLK_SEL_LPPLL)
    freq = 32768;
    else
    return -EINVAL;
// set the codec mclk
    ret = snd_soc_dai_set_sysclk(codec_dai, clk_id, freq,
    SND_SOC_CLOCK_IN);
    if (ret) {
    printk(KERN_ERR "can't set codec system clock\n");
    return ret;
    }
    return ret;
    }
    static const struct snd_soc_ops omap_abe_ops = {
    .hw_params = omap_abe_hw_params,
    };
    static int omap_abe_dmic_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int = 0;
    ret = snd_soc_dai_set_sysclk(cpu_dai, OMAP_DMIC_SYSCLK_PAD_CLKS,
    19200000, SND_SOC_CLOCK_IN);
    if (ret < 0) {
    printk(KERN_ERR "can't set DMIC cpu system clock\n");
    return ret;
    }
    ret = snd_soc_dai_set_sysclk(cpu_dai, OMAP_DMIC_ABE_DMIC_CLK, 2400000,
    SND_SOC_CLOCK_OUT);
    if (ret < 0) {
    printk(KERN_ERR "can't set DMIC output clock\n");
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_ops omap_abe_dmic_ops = {
    .hw_params = omap_abe_dmic_hw_params,
    };
// Headset jack
    static struct snd_soc_jack hs_jack;
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
// SDP4430 machine DAPM
    static const struct snd_soc_dapm_widget twl6040_dapm_widgets[] = {
// Outputs
    SND_SOC_DAPM_HP("Headset Stereophone", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Earphone Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Ext Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line Out", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Vibrator", core::ptr::null_mut()),
// Inputs
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Main Handset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Sub Handset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line In", core::ptr::null_mut()),
// Digital microphones
    SND_SOC_DAPM_MIC("Digital Mic", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route audio_map[] = {
// Routings for outputs
    {"Headset Stereophone", core::ptr::null_mut(), "HSOL"},
    {"Headset Stereophone", core::ptr::null_mut(), "HSOR"},
    {"Earphone Spk", core::ptr::null_mut(), "EP"},
    {"Ext Spk", core::ptr::null_mut(), "HFL"},
    {"Ext Spk", core::ptr::null_mut(), "HFR"},
    {"Line Out", core::ptr::null_mut(), "AUXL"},
    {"Line Out", core::ptr::null_mut(), "AUXR"},
    {"Vibrator", core::ptr::null_mut(), "VIBRAL"},
    {"Vibrator", core::ptr::null_mut(), "VIBRAR"},
// Routings for inputs
    {"HSMIC", core::ptr::null_mut(), "Headset Mic"},
    {"Headset Mic", core::ptr::null_mut(), "Headset Mic Bias"},
    {"MAINMIC", core::ptr::null_mut(), "Main Handset Mic"},
    {"Main Handset Mic", core::ptr::null_mut(), "Main Mic Bias"},
    {"SUBMIC", core::ptr::null_mut(), "Sub Handset Mic"},
    {"Sub Handset Mic", core::ptr::null_mut(), "Main Mic Bias"},
    {"AFML", core::ptr::null_mut(), "Line In"},
    {"AFMR", core::ptr::null_mut(), "Line In"},
    };
#[no_mangle]
unsafe extern "C" fn omap_abe_twl6040_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int omap_abe_twl6040_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_component *component = snd_soc_rtd_to_codec(rtd, 0).component;
    struct snd_soc_card *card = rtd.card;
    struct abe_twl6040 *priv = snd_soc_card_get_drvdata(card);
    int hs_trim;
    int ret;
//
// Configure McPDM offset cancellation based on the HSOTRIM value from
// twl6040.
//
    hs_trim = twl6040_get_trim_value(component, TWL6040_TRIM_HSOTRIM);
    omap_mcpdm_configure_dn_offsets(rtd, TWL6040_HSF_TRIM_LEFT(hs_trim),
    TWL6040_HSF_TRIM_RIGHT(hs_trim));
// Headset jack detection only if it is supported
    if (priv.jack_detection) {
    ret = snd_soc_card_jack_new_pins(rtd.card, "Headset Jack",
    SND_JACK_HEADSET, &hs_jack,
    hs_jack_pins,
    ARRAY_SIZE(hs_jack_pins));
    if (ret)
    return ret;
    twl6040_hs_jack_detect(component, &hs_jack, SND_JACK_HEADSET);
    }
    return 0;
    }
    static const struct snd_soc_dapm_route dmic_audio_map[] = {
    {"DMic", core::ptr::null_mut(), "Digital Mic"},
    {"Digital Mic", core::ptr::null_mut(), "Digital Mic1 Bias"},
    };
#[no_mangle]
unsafe extern "C" fn omap_abe_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int omap_abe_dmic_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(rtd.card);
    return snd_soc_dapm_add_routes(dapm, dmic_audio_map,
    ARRAY_SIZE(dmic_audio_map));
    }
#[no_mangle]
unsafe extern "C" fn omap_abe_probe(pdev: *mut platform_device) -> c_int {
    static int omap_abe_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct snd_soc_card *card;
    struct device_node *dai_node;
    struct abe_twl6040 *priv;
    let mut num_links: c_int = 0;
    let mut ret: c_int = 0;
    if (!node) {
    dev_err(&pdev.dev, "of node is missing.\n");
    return -ENODEV;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(struct abe_twl6040), GFP_KERNEL);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    card = &priv.card;
    card.dev = &pdev.dev;
    card.owner = THIS_MODULE;
    card.dapm_widgets = twl6040_dapm_widgets;
    card.num_dapm_widgets = ARRAY_SIZE(twl6040_dapm_widgets);
    card.dapm_routes = audio_map;
    card.num_dapm_routes = ARRAY_SIZE(audio_map);
    ret = snd_soc_of_parse_card_name(card, "ti,model");
    if (ret)
    return ret;
    ret = snd_soc_of_parse_audio_routing(card, "ti,audio-routing");
    if (ret)
    return ret;
    dai_node = of_parse_phandle(node, "ti,mcpdm", 0);
    if (!dai_node) {
    dev_err(&pdev.dev, "McPDM node is not provided\n");
    return -EINVAL;
    }
    priv.dai_links[0].name = "DMIC";
    priv.dai_links[0].stream_name = "TWL6040";
    priv.dai_links[0].cpus = link0_cpus;
    priv.dai_links[0].num_cpus = 1;
    priv.dai_links[0].cpus.of_node = dai_node;
    priv.dai_links[0].platforms = link0_platforms;
    priv.dai_links[0].num_platforms = 1;
    priv.dai_links[0].platforms.of_node = dai_node;
    priv.dai_links[0].codecs = link0_codecs;
    priv.dai_links[0].num_codecs = 1;
    priv.dai_links[0].init = omap_abe_twl6040_init;
    priv.dai_links[0].ops = &omap_abe_ops;
    dai_node = of_parse_phandle(node, "ti,dmic", 0);
    if (dai_node) {
    num_links = 2;
    priv.dai_links[1].name = "TWL6040";
    priv.dai_links[1].stream_name = "DMIC Capture";
    priv.dai_links[1].cpus = link1_cpus;
    priv.dai_links[1].num_cpus = 1;
    priv.dai_links[1].cpus.of_node = dai_node;
    priv.dai_links[1].platforms = link1_platforms;
    priv.dai_links[1].num_platforms = 1;
    priv.dai_links[1].platforms.of_node = dai_node;
    priv.dai_links[1].codecs = link1_codecs;
    priv.dai_links[1].num_codecs = 1;
    priv.dai_links[1].init = omap_abe_dmic_init;
    priv.dai_links[1].ops = &omap_abe_dmic_ops;
    } else {
    num_links = 1;
    }
    priv.jack_detection = of_property_read_bool(node, "ti,jack-detection");
    of_property_read_u32(node, "ti,mclk-freq", &priv.mclk_freq);
    if (!priv.mclk_freq) {
    dev_err(&pdev.dev, "MCLK frequency not provided\n");
    return -EINVAL;
    }
    card.fully_routed = 1;
    card.dai_link = priv.dai_links;
    card.num_links = num_links;
    snd_soc_card_set_drvdata(card, priv);
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    dev_err_probe(&pdev.dev, ret,
    "devm_snd_soc_register_card() failed\n");
    return ret;
    }
    static const struct of_device_id omap_abe_of_match[] = {
    {.compatible = "ti,abe-twl6040", },
    { },
    };
    MODULE_DEVICE_TABLE(of, omap_abe_of_match);
    static struct platform_driver omap_abe_driver = {
    .driver = {
    .name = "omap-abe-twl6040",
    .pm = &snd_soc_pm_ops,
    .of_match_table = omap_abe_of_match,
    },
    .probe = omap_abe_probe,
    };
#[no_mangle]
unsafe extern "C" fn omap_abe_init() -> int __init {
    static int __init omap_abe_init(void)
    {
    int ret;
    dmic_codec_dev = platform_device_register_simple("dmic-codec", -1, core::ptr::null_mut(),
    0);
    if (IS_ERR(dmic_codec_dev)) {
    pr_err("%s: dmic-codec device registration failed\n", __func__);
    return PTR_ERR(dmic_codec_dev);
    }
    ret = platform_driver_register(&omap_abe_driver);
    if (ret) {
    pr_err("%s: platform driver registration failed\n", __func__);
    platform_device_unregister(dmic_codec_dev);
    }
    return ret;
    }
    module_init(omap_abe_init);
#[no_mangle]
unsafe extern "C" fn omap_abe_exit() -> void __exit {
    static void __exit omap_abe_exit(void)
    {
    platform_driver_unregister(&omap_abe_driver);
    platform_device_unregister(dmic_codec_dev);
    }
    module_exit(omap_abe_exit);
    MODULE_AUTHOR("Misael Lopez Cruz <misael.lopez@ti.com>");
    MODULE_DESCRIPTION("ALSA SoC for OMAP boards with ABE and twl6040 codec");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:omap-abe-twl6040");
