//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/x1e80100.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2023, Linaro Limited

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x1e80100_snd_data {
    pub stream_prepared: [bool; AFE_PORT_MAX],
    pub card: *mut snd_soc_card,
    pub jack: snd_soc_jack,
    pub dp_jack: [snd_soc_jack; 8],
    pub jack_setup: bool,
}

#[no_mangle]
unsafe extern "C" fn x1e80100_snd_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int x1e80100_snd_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct x1e80100_snd_data *data = snd_soc_card_get_drvdata(rtd.card);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_jack *dp_jack = core::ptr::null_mut();
    let mut dp_pcm_id: c_int = 0;
    switch (cpu_dai.id) {
    case WSA_CODEC_DMA_RX_0:
    case WSA_CODEC_DMA_RX_1:
//
// Set limit of -3 dB on Digital Volume and 0 dB on PA Volume
// to reduce the risk of speaker damage until we have active
// speaker protection in place.
//
    snd_soc_limit_volume(card, "WSA WSA_RX0 Digital Volume", 81);
    snd_soc_limit_volume(card, "WSA WSA_RX1 Digital Volume", 81);
    snd_soc_limit_volume(card, "WSA2 WSA_RX0 Digital Volume", 81);
    snd_soc_limit_volume(card, "WSA2 WSA_RX1 Digital Volume", 81);
    snd_soc_limit_volume(card, "SpkrLeft PA Volume", 6);
    snd_soc_limit_volume(card, "SpkrRight PA Volume", 6);
    snd_soc_limit_volume(card, "WooferLeft PA Volume", 6);
    snd_soc_limit_volume(card, "TweeterLeft PA Volume", 6);
    snd_soc_limit_volume(card, "WooferRight PA Volume", 6);
    snd_soc_limit_volume(card, "TweeterRight PA Volume", 6);
    break;
    case DISPLAY_PORT_RX_0:
    dp_pcm_id = 0;
    dp_jack = &data.dp_jack[dp_pcm_id];
    break;
    case DISPLAY_PORT_RX_1 ... DISPLAY_PORT_RX_7:
    dp_pcm_id = cpu_dai.id - DISPLAY_PORT_RX_1 + 1;
    dp_jack = &data.dp_jack[dp_pcm_id];
    break;
    default:
    break;
    }
    if (dp_jack)
    return qcom_snd_dp_jack_setup(rtd, dp_jack, dp_pcm_id);
    return qcom_snd_wcd_jack_setup(rtd, &data.jack, &data.jack_setup);
    }
    static int x1e80100_be_hw_params_fixup(struct snd_soc_pcm_runtime *rtd,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_interval *rate = hw_param_interval(params,
    SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval *channels = hw_param_interval(params,
    SNDRV_PCM_HW_PARAM_CHANNELS);
    rate.min = rate.max = 48000;
    switch (cpu_dai.id) {
    case TX_CODEC_DMA_TX_0:
    case TX_CODEC_DMA_TX_1:
    case TX_CODEC_DMA_TX_2:
    case TX_CODEC_DMA_TX_3:
    channels.min = 1;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1e80100_snd_hw_map_channels(ch_map: *mut c_uint, num: c_int) -> c_int {
    static int x1e80100_snd_hw_map_channels(unsigned int *ch_map, int num)
    {
    switch (num) {
    case 1:
    ch_map[0] = PCM_CHANNEL_FC;
    break;
    case 2:
    ch_map[0] = PCM_CHANNEL_FL;
    ch_map[1] = PCM_CHANNEL_FR;
    break;
    case 3:
    ch_map[0] = PCM_CHANNEL_FL;
    ch_map[1] = PCM_CHANNEL_FR;
    ch_map[2] = PCM_CHANNEL_FC;
    break;
    case 4:
    ch_map[0] = PCM_CHANNEL_FL;
    ch_map[1] = PCM_CHANNEL_LB;
    ch_map[2] = PCM_CHANNEL_FR;
    ch_map[3] = PCM_CHANNEL_RB;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1e80100_snd_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int x1e80100_snd_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct x1e80100_snd_data *data = snd_soc_card_get_drvdata(rtd.card);
    let mut channels: c_uint = substream.runtime.channels;
    unsigned int rx_slot[4];
    int ret;
    switch (cpu_dai.id) {
    case WSA_CODEC_DMA_RX_0:
    case WSA_CODEC_DMA_RX_1:
    ret = x1e80100_snd_hw_map_channels(rx_slot, channels);
    if (ret)
    return ret;
    ret = snd_soc_dai_set_channel_map(cpu_dai, 0, core::ptr::null_mut(),
    channels, rx_slot);
    if (ret)
    return ret;
    break;
    default:
    break;
    }
    return qcom_snd_sdw_prepare(substream, &data.stream_prepared[cpu_dai.id]);
    }
#[no_mangle]
unsafe extern "C" fn x1e80100_snd_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int x1e80100_snd_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct x1e80100_snd_data *data = snd_soc_card_get_drvdata(rtd.card);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    return qcom_snd_sdw_hw_free(substream, &data.stream_prepared[cpu_dai.id]);
    }
    static const struct snd_soc_ops x1e80100_be_ops = {
    .startup = qcom_snd_sdw_startup,
    .shutdown = qcom_snd_sdw_shutdown,
    .hw_free = x1e80100_snd_hw_free,
    .prepare = x1e80100_snd_prepare,
    };
#[no_mangle]
unsafe extern "C" fn x1e80100_add_be_ops(card: *mut snd_soc_card) {
    static void x1e80100_add_be_ops(struct snd_soc_card *card)
    {
    struct snd_soc_dai_link *link;
    int i;
    for_each_card_prelinks(card, i, link) {
    if (link.no_pcm == 1) {
    link.init = x1e80100_snd_init;
    link.be_hw_params_fixup = x1e80100_be_hw_params_fixup;
    link.ops = &x1e80100_be_ops;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn x1e80100_platform_probe(pdev: *mut platform_device) -> c_int {
    static int x1e80100_platform_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card;
    struct x1e80100_snd_data *data;
    struct device *dev = &pdev.dev;
    int ret;
    card = devm_kzalloc(dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
// Allocate the private data
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    card.owner = THIS_MODULE;
    card.dev = dev;
    dev_set_drvdata(dev, card);
    snd_soc_card_set_drvdata(card, data);
    ret = qcom_snd_parse_of(card);
    if (ret)
    return ret;
    card.driver_name = of_device_get_match_data(dev);
    x1e80100_add_be_ops(card);
    return devm_snd_soc_register_card(dev, card);
    }
    static const struct of_device_id snd_x1e80100_dt_match[] = {
    { .compatible = "qcom,x1e80100-sndcard", .data = "x1e80100" },
    { .compatible = "qcom,glymur-sndcard", .data = "glymur" },
    {}
    };
    MODULE_DEVICE_TABLE(of, snd_x1e80100_dt_match);
    static struct platform_driver snd_x1e80100_driver = {
    .probe  = x1e80100_platform_probe,
    .driver = {
    .name = "snd-x1e80100",
    .of_match_table = snd_x1e80100_dt_match,
    },
    };
    module_platform_driver(snd_x1e80100_driver);
    MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org");
    MODULE_AUTHOR("Krzysztof Kozlowski <krzysztof.kozlowski@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm X1E80100 ASoC Machine Driver");
    MODULE_LICENSE("GPL");
