//! Automatically rewritten from C to Rust
//! Source: sound/soc/ti/davinci-evm.c
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
// ASoC driver for TI DAVINCI EVM platform
//
// Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
// Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_card_drvdata_davinci {
    pub mclk: *mut clk,
    pub sysclk: unsigned,
}

#[no_mangle]
unsafe extern "C" fn evm_startup(substream: *mut snd_pcm_substream) -> c_int {
    static int evm_startup(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_card *soc_card = rtd.card;
    struct snd_soc_card_drvdata_davinci *drvdata =
    snd_soc_card_get_drvdata(soc_card);
    if (drvdata.mclk)
    return clk_prepare_enable(drvdata.mclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn evm_shutdown(substream: *mut snd_pcm_substream) {
    static void evm_shutdown(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_card *soc_card = rtd.card;
    struct snd_soc_card_drvdata_davinci *drvdata =
    snd_soc_card_get_drvdata(soc_card);
    clk_disable_unprepare(drvdata.mclk);
    }
    static int evm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_card *soc_card = rtd.card;
    let mut ret: c_int = 0;
    unsigned sysclk = ((struct snd_soc_card_drvdata_davinci *)
    snd_soc_card_get_drvdata(soc_card)).sysclk;
// set the codec system clock
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, sysclk, SND_SOC_CLOCK_OUT);
    if (ret < 0)
    return ret;
// set the CPU system clock
    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, sysclk, SND_SOC_CLOCK_OUT);
    if (ret < 0 && ret != -ENOTSUPP)
    return ret;
    return 0;
    }
    static const struct snd_soc_ops evm_ops = {
    .startup = evm_startup,
    .shutdown = evm_shutdown,
    .hw_params = evm_hw_params,
    };
// davinci-evm machine dapm widgets
    static const struct snd_soc_dapm_widget aic3x_dapm_widgets[] = {
    SND_SOC_DAPM_HP("Headphone Jack", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line Out", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Mic Jack", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line In", core::ptr::null_mut()),
    };
// davinci-evm machine audio_mapnections to the codec pins
    static const struct snd_soc_dapm_route audio_map[] = {
// Headphone connected to HPLOUT, HPROUT
    {"Headphone Jack", core::ptr::null_mut(), "HPLOUT"},
    {"Headphone Jack", core::ptr::null_mut(), "HPROUT"},
// Line Out connected to LLOUT, RLOUT
    {"Line Out", core::ptr::null_mut(), "LLOUT"},
    {"Line Out", core::ptr::null_mut(), "RLOUT"},
// Mic connected to (MIC3L | MIC3R)
    {"MIC3L", core::ptr::null_mut(), "Mic Bias"},
    {"MIC3R", core::ptr::null_mut(), "Mic Bias"},
    {"Mic Bias", core::ptr::null_mut(), "Mic Jack"},
// Line In connected to (LINE1L | LINE2L), (LINE1R | LINE2R)
    {"LINE1L", core::ptr::null_mut(), "Line In"},
    {"LINE2L", core::ptr::null_mut(), "Line In"},
    {"LINE1R", core::ptr::null_mut(), "Line In"},
    {"LINE2R", core::ptr::null_mut(), "Line In"},
    };
// Logic for a aic3x as connected on a davinci-evm
#[no_mangle]
unsafe extern "C" fn evm_aic3x_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int evm_aic3x_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(rtd.card);
    struct device_node *np = card.dev.of_node;
    int ret;
// Add davinci-evm specific widgets
    snd_soc_dapm_new_controls(dapm, aic3x_dapm_widgets,
    ARRAY_SIZE(aic3x_dapm_widgets));
    if (np) {
    ret = snd_soc_of_parse_audio_routing(card, "ti,audio-routing");
    if (ret)
    return ret;
    } else {
// Set up davinci-evm specific audio path audio_map
    snd_soc_dapm_add_routes(dapm, audio_map,
    ARRAY_SIZE(audio_map));
    }
// not connected
    snd_soc_dapm_disable_pin(dapm, "MONO_LOUT");
    snd_soc_dapm_disable_pin(dapm, "HPLCOM");
    snd_soc_dapm_disable_pin(dapm, "HPRCOM");
    return 0;
    }
//
// The struct is used as place holder. It will be completely
// filled with data from dt node.
//
    SND_SOC_DAILINK_DEFS(evm,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC(core::ptr::null_mut(), "tlv320aic3x-hifi")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    static struct snd_soc_dai_link evm_dai_tlv320aic3x = {
    .name		= "TLV320AIC3X",
    .stream_name	= "AIC3X",
    .ops            = &evm_ops,
    .init           = evm_aic3x_init,
    .dai_fmt = SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_CBP_CFP |
    SND_SOC_DAIFMT_IB_NF,
    SND_SOC_DAILINK_REG(evm),
    };
    static const struct of_device_id davinci_evm_dt_ids[] = {
    {
    .compatible = "ti,da830-evm-audio",
    .data = (void *) &evm_dai_tlv320aic3x,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, davinci_evm_dt_ids);
// davinci evm audio machine driver
    static struct snd_soc_card evm_soc_card = {
    .owner = THIS_MODULE,
    .num_links = 1,
    };
#[no_mangle]
unsafe extern "C" fn davinci_evm_probe(pdev: *mut platform_device) -> c_int {
    static int davinci_evm_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct snd_soc_dai_link *dai;
    struct snd_soc_card_drvdata_davinci *drvdata = core::ptr::null_mut();
    struct clk *mclk;
    let mut ret: c_int = 0;
    dai = (struct snd_soc_dai_link *) device_get_match_data(&pdev.dev);
    if (!dai) {
    dev_err(&pdev.dev, "Error: No device match found\n");
    return -ENODEV;
    }
    evm_soc_card.dai_link = dai;
    dai.codecs.of_node = of_parse_phandle(np, "ti,audio-codec", 0);
    if (!dai.codecs.of_node)
    return -EINVAL;
    dai.cpus.of_node = of_parse_phandle(np, "ti,mcasp-controller", 0);
    if (!dai.cpus.of_node) {
    ret = -EINVAL;
    goto err_put;
    }
    dai.platforms.of_node = dai.cpus.of_node;
    evm_soc_card.dev = &pdev.dev;
    ret = snd_soc_of_parse_card_name(&evm_soc_card, "ti,model");
    if (ret)
    goto err_put;
    mclk = devm_clk_get(&pdev.dev, "mclk");
    if (PTR_ERR(mclk) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto err_put;
    } else if (IS_ERR(mclk)) {
    dev_dbg(&pdev.dev, "mclk not found.\n");
    mclk = core::ptr::null_mut();
    }
    drvdata = devm_kzalloc(&pdev.dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata) {
    ret = -ENOMEM;
    goto err_put;
    }
    drvdata.mclk = mclk;
    ret = of_property_read_u32(np, "ti,codec-clock-rate", &drvdata.sysclk);
    if (ret < 0) {
    if (!drvdata.mclk) {
    dev_err(&pdev.dev,
    "No clock or clock rate defined.\n");
    ret = -EINVAL;
    goto err_put;
    }
    drvdata.sysclk = clk_get_rate(drvdata.mclk);
    } else if (drvdata.mclk) {
    let mut requestd_rate: c_uint = drvdata.sysclk;
    clk_set_rate(drvdata.mclk, drvdata.sysclk);
    drvdata.sysclk = clk_get_rate(drvdata.mclk);
    if (drvdata.sysclk != requestd_rate)
    dev_warn(&pdev.dev,
    "Could not get requested rate %u using %u.\n",
    requestd_rate, drvdata.sysclk);
    }
    snd_soc_card_set_drvdata(&evm_soc_card, drvdata);
    ret = devm_snd_soc_register_card(&pdev.dev, &evm_soc_card);
    if (ret) {
    dev_err_probe(&pdev.dev, ret, "snd_soc_register_card() failed\n");
    goto err_put;
    }
    return ret;
    err_put:
    dai.platforms.of_node = core::ptr::null_mut();
    if (dai.cpus.of_node) {
    of_node_put(dai.cpus.of_node);
    dai.cpus.of_node = core::ptr::null_mut();
    }
    if (dai.codecs.of_node) {
    of_node_put(dai.codecs.of_node);
    dai.codecs.of_node = core::ptr::null_mut();
    }
    return ret;
    }
    static struct platform_driver davinci_evm_driver = {
    .probe		= davinci_evm_probe,
    .driver		= {
    .name	= "davinci_evm",
    .pm	= &snd_soc_pm_ops,
    .of_match_table = davinci_evm_dt_ids,
    },
    };
    module_platform_driver(davinci_evm_driver);
    MODULE_AUTHOR("Vladimir Barinov");
    MODULE_DESCRIPTION("TI DAVINCI EVM ASoC driver");
    MODULE_LICENSE("GPL");
