//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/qdsp6/q6apm-lpass-dais.c
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
// Copyright (c) 2021, Linaro Limited

pub const AUDIOREACH_BE_PCM_BASE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm_dai_priv_data {
    pub mclk: *mut clk,
    pub bclk: *mut clk,
    pub bclk_enabled: bool mclk_enabled,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6apm_lpass_dai_data {
    pub graph: [*mut q6apm_graph; APM_PORT_MAX],
    pub is_port_started: [bool; APM_PORT_MAX],
    pub module_config: [audioreach_module_config; APM_PORT_MAX],
    pub priv: [q6apm_dai_priv_data; APM_PORT_MAX],
}

#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_disable_clocks(dai_data: *mut q6apm_lpass_dai_data, id: c_int) {
    static void q6apm_lpass_dai_disable_clocks(struct q6apm_lpass_dai_data *dai_data, int id)
    {
    if (dai_data.priv[id].mclk_enabled) {
    clk_disable_unprepare(dai_data.priv[id].mclk);
    dai_data.priv[id].mclk_enabled = false;
    }
    if (dai_data.priv[id].bclk_enabled) {
    clk_disable_unprepare(dai_data.priv[id].bclk);
    dai_data.priv[id].bclk_enabled = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_put_clocks(dai_data: *mut q6apm_lpass_dai_data) {
    static void q6apm_lpass_dai_put_clocks(struct q6apm_lpass_dai_data *dai_data)
    {
    int i;
    for (i = 0; i < APM_PORT_MAX; i++) {
    q6apm_lpass_dai_disable_clocks(dai_data, i);
    if (dai_data.priv[i].mclk) {
    clk_put(dai_data.priv[i].mclk);
    dai_data.priv[i].mclk = core::ptr::null_mut();
    }
    if (dai_data.priv[i].bclk) {
    clk_put(dai_data.priv[i].bclk);
    dai_data.priv[i].bclk = core::ptr::null_mut();
    }
    }
    }
    static int q6dma_set_channel_map(struct snd_soc_dai *dai,
    unsigned int tx_num,
    const unsigned int *tx_ch_mask,
    unsigned int rx_num,
    const unsigned int *rx_ch_mask)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    int i;
    switch (dai.id) {
    case WSA_CODEC_DMA_TX_0:
    case WSA_CODEC_DMA_TX_1:
    case WSA_CODEC_DMA_TX_2:
    case VA_CODEC_DMA_TX_0:
    case VA_CODEC_DMA_TX_1:
    case VA_CODEC_DMA_TX_2:
    case TX_CODEC_DMA_TX_0:
    case TX_CODEC_DMA_TX_1:
    case TX_CODEC_DMA_TX_2:
    case TX_CODEC_DMA_TX_3:
    case TX_CODEC_DMA_TX_4:
    case TX_CODEC_DMA_TX_5:
    if (!tx_ch_mask) {
    dev_err(dai.dev, "tx slot not found\n");
    return -EINVAL;
    }
    if (tx_num > AR_PCM_MAX_NUM_CHANNEL) {
    dev_err(dai.dev, "invalid tx num %d\n",
    tx_num);
    return -EINVAL;
    }
    for (i = 0; i < tx_num; i++)
    cfg.channel_map[i] = tx_ch_mask[i];
    break;
    case WSA_CODEC_DMA_RX_0:
    case WSA_CODEC_DMA_RX_1:
    case RX_CODEC_DMA_RX_0:
    case RX_CODEC_DMA_RX_1:
    case RX_CODEC_DMA_RX_2:
    case RX_CODEC_DMA_RX_3:
    case RX_CODEC_DMA_RX_4:
    case RX_CODEC_DMA_RX_5:
    case RX_CODEC_DMA_RX_6:
    case RX_CODEC_DMA_RX_7:
// rx
    if (!rx_ch_mask) {
    dev_err(dai.dev, "rx slot not found\n");
    return -EINVAL;
    }
    if (rx_num > APM_PORT_MAX_AUDIO_CHAN_CNT) {
    dev_err(dai.dev, "invalid rx num %d\n",
    rx_num);
    return -EINVAL;
    }
    for (i = 0; i < rx_num; i++)
    cfg.channel_map[i] = rx_ch_mask[i];
    break;
    default:
    dev_err(dai.dev, "%s: invalid dai id 0x%x\n",
    __func__, dai.id);
    return -EINVAL;
    }
    return 0;
    }
    static int q6hdmi_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    let mut channels: c_int = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS).max;
    int ret;
    cfg.bit_width = params_width(params);
    cfg.sample_rate = params_rate(params);
    cfg.num_channels = channels;
    audioreach_set_default_channel_mapping(cfg.channel_map, channels);
    switch (dai.id) {
    case DISPLAY_PORT_RX_0:
    cfg.dp_idx = 0;
    break;
    case DISPLAY_PORT_RX_1 ... DISPLAY_PORT_RX_7:
    cfg.dp_idx = dai.id - DISPLAY_PORT_RX_1 + 1;
    break;
    }
    ret = q6dsp_get_channel_allocation(channels);
    if (ret < 0)
    return ret;
    cfg.channel_allocation = ret;
    return 0;
    }
    static int q6dma_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    let mut channels: c_int = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS).max;
    cfg.bit_width = params_width(params);
    cfg.sample_rate = params_rate(params);
    cfg.num_channels = channels;
    audioreach_set_default_channel_mapping(cfg.channel_map, channels);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    static void q6apm_lpass_dai_shutdown(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    int rc;
    if (dai_data.is_port_started[dai.id]) {
    rc = q6apm_graph_stop(dai_data.graph[dai.id]);
    dai_data.is_port_started[dai.id] = false;
    if (rc < 0)
    dev_err(dai.dev, "failed to stop APM port (%d)\n", rc);
    }
    if (dai_data.graph[dai.id]) {
    q6apm_graph_close(dai_data.graph[dai.id]);
    dai_data.graph[dai.id] = core::ptr::null_mut();
    }
    }
    static int q6apm_lpass_dai_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    if (!dai_data.is_port_started[dai.id]) {
    ret = q6apm_graph_start(dai_data.graph[dai.id]);
    if (ret < 0)
    dev_err(dai.dev, "Failed to start APM port %d\n", dai.id);
    else
    dai_data.is_port_started[dai.id] = true;
    }
    break;
    default:
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int q6apm_lpass_dai_prepare(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    struct q6apm_graph *graph;
    let mut graph_id: c_int = dai.id;
    int rc;
    if (dai_data.is_port_started[dai.id]) {
    q6apm_graph_stop(dai_data.graph[dai.id]);
    dai_data.is_port_started[dai.id] = false;
    }
//
// It is recommend to load DSP with source graph first and then sink
// graph, so sequence for playback and capture will be different
//
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK && dai_data.graph[dai.id] == core::ptr::null_mut()) {
    graph = q6apm_graph_open(dai.dev, core::ptr::null_mut(), dai.dev, graph_id, substream.stream);
    if (IS_ERR(graph)) {
    dev_err(dai.dev, "Failed to open graph (%d)\n", graph_id);
    rc = PTR_ERR(graph);
    return rc;
    }
    dai_data.graph[graph_id] = graph;
    }
    cfg.direction = substream.stream;
    rc = q6apm_graph_media_format_pcm(dai_data.graph[dai.id], cfg);
    if (rc) {
    dev_err(dai.dev, "Failed to set media format %d\n", rc);
    goto err;
    }
    rc = q6apm_graph_prepare(dai_data.graph[dai.id]);
    if (rc) {
    dev_err(dai.dev, "Failed to prepare Graph %d\n", rc);
    goto err;
    }
    return 0;
    err:
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    q6apm_graph_close(dai_data.graph[dai.id]);
    dai_data.graph[dai.id] = core::ptr::null_mut();
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int q6apm_lpass_dai_startup(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct q6apm_graph *graph;
    let mut graph_id: c_int = dai.id;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    graph = q6apm_graph_open(dai.dev, core::ptr::null_mut(), dai.dev, graph_id, substream.stream);
    if (IS_ERR(graph)) {
    dev_err(dai.dev, "Failed to open graph (%d)\n", graph_id);
    return PTR_ERR(graph);
    }
    dai_data.graph[graph_id] = graph;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6i2s_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int q6i2s_dai_startup(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    return q6apm_lpass_dai_startup(substream, dai);
    }
#[no_mangle]
unsafe extern "C" fn q6i2s_lpass_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    static void q6i2s_lpass_dai_shutdown(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    q6apm_lpass_dai_shutdown(substream, dai);
    q6apm_lpass_dai_disable_clocks(dai_data, dai.id);
    }
#[no_mangle]
unsafe extern "C" fn q6i2s_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    static int q6i2s_set_sysclk(struct snd_soc_dai *dai, int clk_id, unsigned int freq, int dir)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct clk *sysclk = core::ptr::null_mut();
    bool *enabled = core::ptr::null_mut();
    let mut ret: c_int = 0;
    switch (clk_id) {
    case LPAIF_MI2S_MCLK:
    sysclk = dai_data.priv[dai.id].mclk;
    enabled = &dai_data.priv[dai.id].mclk_enabled;
    break;
    case LPAIF_MI2S_BCLK:
    sysclk = dai_data.priv[dai.id].bclk;
    enabled = &dai_data.priv[dai.id].bclk_enabled;
    break;
    default:
    return -EINVAL;
    }
    if (sysclk) {
    ret = clk_set_rate(sysclk, freq);
    if (ret) {
    dev_err(dai.dev, "Error, Unable to set rate (%d) for sysclk %d\n",
    freq, clk_id);
    return ret;
    }
    if (*enabled)
    return 0;
    ret = clk_prepare_enable(sysclk);
    if (ret) {
    dev_err(dai.dev, "Error, Unable to prepare (%d) sysclk\n", clk_id);
    return ret;
    }
// enabled = true;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int q6i2s_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    cfg.fmt = fmt;
    return 0;
    }
    static int q6tdm_set_tdm_slot(struct snd_soc_dai *dai,
    unsigned int tx_mask,
    unsigned int rx_mask,
    int slots, int slot_width)
    {
    struct q6apm_lpass_dai_data *dai_data = dev_get_drvdata(dai.dev);
    struct audioreach_module_config *cfg = &dai_data.module_config[dai.id];
    unsigned int cap_mask, slot_mask;
    if (slot_width != 16 && slot_width != 32) {
    dev_err(dai.dev, "%s: invalid slot_width %d\n", __func__, slot_width);
    return -EINVAL;
    }
    switch (slots) {
    case 2:
    case 4:
    case 8:
    case 16:
    cap_mask = GENMASK(slots - 1, 0);
    break;
    default:
    dev_err(dai.dev, "%s: invalid slots %d\n", __func__, slots);
    return -EINVAL;
    }
    switch (dai.id) {
    case PRIMARY_TDM_RX_0 ... QUINARY_TDM_TX_7:
    slot_mask = (dai.id & 0x1) ? tx_mask : rx_mask;
    if (slot_mask & ~cap_mask) {
    dev_err(dai.dev, "%s: invalid slot mask 0x%x for %d slots\n",
    __func__, slot_mask, slots);
    return -EINVAL;
    }
    cfg.nslots_per_frame = slots;
    cfg.slot_width = slot_width;
    cfg.slot_mask = slot_mask;
    break;
    default:
    dev_err(dai.dev, "%s: invalid dai id 0x%x\n", __func__, dai.id);
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops q6dma_ops = {
    .prepare	= q6apm_lpass_dai_prepare,
    .startup	= q6apm_lpass_dai_startup,
    .shutdown	= q6apm_lpass_dai_shutdown,
    .set_channel_map  = q6dma_set_channel_map,
    .hw_params        = q6dma_hw_params,
    .trigger	= q6apm_lpass_dai_trigger,
    };
    static const struct snd_soc_dai_ops q6i2s_ops = {
    .prepare	= q6apm_lpass_dai_prepare,
    .startup	= q6i2s_dai_startup,
    .shutdown	= q6i2s_lpass_dai_shutdown,
    .set_channel_map  = q6dma_set_channel_map,
    .hw_params        = q6dma_hw_params,
    .set_fmt	= q6i2s_set_fmt,
    .set_sysclk	= q6i2s_set_sysclk,
    .trigger	= q6apm_lpass_dai_trigger,
    };
    static const struct snd_soc_dai_ops q6hdmi_ops = {
    .prepare	= q6apm_lpass_dai_prepare,
    .startup	= q6apm_lpass_dai_startup,
    .shutdown	= q6apm_lpass_dai_shutdown,
    .hw_params	= q6hdmi_hw_params,
    .set_fmt	= q6i2s_set_fmt,
    .trigger	= q6apm_lpass_dai_trigger,
    };
    static const struct snd_soc_dai_ops q6tdm_ops = {
    .prepare	= q6apm_lpass_dai_prepare,
    .startup	= q6apm_lpass_dai_startup,
    .shutdown	= q6i2s_lpass_dai_shutdown,
    .set_tdm_slot	= q6tdm_set_tdm_slot,
    .hw_params	= q6dma_hw_params,
    .set_fmt	= q6i2s_set_fmt,
    .set_sysclk	= q6i2s_set_sysclk,
    .trigger	= q6apm_lpass_dai_trigger,
    };
    static const struct snd_soc_component_driver q6apm_lpass_dai_component = {
    .name = "q6apm-be-dai-component",
    .of_xlate_dai_name = q6dsp_audio_ports_of_xlate_dai_name,
    .be_pcm_base = AUDIOREACH_BE_PCM_BASE,
    .use_dai_pcm_id = true,
    .remove_order   = SND_SOC_COMP_ORDER_FIRST,
    };
    static int of_q6apm_parse_dai_data(struct device *dev,
    struct q6apm_lpass_dai_data *data)
    {
    int ret;
    for_each_child_of_node_scoped(dev.of_node, node) {
    struct q6apm_dai_priv_data *priv;
    int id;
    ret = of_property_read_u32(node, "reg", &id);
    if (ret || id < 0 || id >= APM_PORT_MAX) {
    dev_err(dev, "valid dai id not found:%d\n", ret);
    continue;
    }
    switch (id) {
// MI2S specific properties
    case PRIMARY_MI2S_RX ... QUATERNARY_MI2S_TX:
    case QUINARY_MI2S_RX ... QUINARY_MI2S_TX:
    case SENARY_MI2S_RX ... SENARY_MI2S_TX:
    case PRIMARY_TDM_RX_0 ... QUINARY_TDM_TX_7:
    priv = &data.priv[id];
    priv.mclk = of_clk_get_by_name(node, "mclk");
    if (IS_ERR(priv.mclk)) {
    let mut err: c_int = PTR_ERR(priv.mclk);
    priv.mclk = core::ptr::null_mut();
    if (err == -EPROBE_DEFER) {
    q6apm_lpass_dai_put_clocks(data);
    return dev_err_probe(dev, err,
    "unable to get mi2s mclk\n");
    }
    }
    priv.bclk = of_clk_get_by_name(node, "bclk");
    if (IS_ERR(priv.bclk)) {
    let mut err: c_int = PTR_ERR(priv.bclk);
    priv.bclk = core::ptr::null_mut();
    if (err == -EPROBE_DEFER) {
    q6apm_lpass_dai_put_clocks(data);
    return dev_err_probe(dev, err,
    "unable to get mi2s bclk\n");
    }
    }
    break;
    default:
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_clocks_action(data: *mut c_void) {
    static void q6apm_lpass_dai_clocks_action(void *data)
    {
    q6apm_lpass_dai_put_clocks(data);
    }
#[no_mangle]
unsafe extern "C" fn q6apm_lpass_dai_dev_probe(pdev: *mut platform_device) -> c_int {
    static int q6apm_lpass_dai_dev_probe(struct platform_device *pdev)
    {
    struct q6dsp_audio_port_dai_driver_config cfg;
    struct q6apm_lpass_dai_data *dai_data;
    struct snd_soc_dai_driver *dais;
    struct device *dev = &pdev.dev;
    int num_dais;
    int ret;
    dai_data = devm_kzalloc(dev, sizeof(*dai_data), GFP_KERNEL);
    if (!dai_data)
    return -ENOMEM;
    dev_set_drvdata(dev, dai_data);
    ret = of_q6apm_parse_dai_data(dev, dai_data);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, q6apm_lpass_dai_clocks_action, dai_data);
    if (ret)
    return ret;
    memset(&cfg, 0, sizeof(cfg));
    cfg.q6i2s_ops = &q6i2s_ops;
    cfg.q6dma_ops = &q6dma_ops;
    cfg.q6hdmi_ops = &q6hdmi_ops;
    cfg.q6tdm_ops = &q6tdm_ops;
    dais = q6dsp_audio_ports_set_config(dev, &cfg, &num_dais);
    return devm_snd_soc_register_component(dev, &q6apm_lpass_dai_component, dais, num_dais);
    }

    static const struct of_device_id q6apm_lpass_dai_device_id[] = {
    { .compatible = "qcom,q6apm-lpass-dais" },
    {},
    };
    MODULE_DEVICE_TABLE(of, q6apm_lpass_dai_device_id);

    static struct platform_driver q6apm_lpass_dai_platform_driver = {
    .driver = {
    .name = "q6apm-lpass-dais",
    .of_match_table = of_match_ptr(q6apm_lpass_dai_device_id),
    },
    .probe = q6apm_lpass_dai_dev_probe,
    };
    module_platform_driver(q6apm_lpass_dai_platform_driver);
    MODULE_DESCRIPTION("AUDIOREACH APM LPASS dai driver");
    MODULE_LICENSE("GPL");
