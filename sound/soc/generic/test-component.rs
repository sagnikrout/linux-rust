//! Automatically rewritten from C to Rust
//! Source: sound/soc/generic/test-component.c
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
//
// test-component.c  --  Test Audio Component driver
//
// Copyright (C) 2020 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

pub const TEST_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_dai_name {
    pub name: [c_char; TEST_NAME_LEN],
    pub name_playback: [c_char; TEST_NAME_LEN],
    pub name_capture: [c_char; TEST_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_priv {
    pub dev: *mut device,
    pub substream: *mut snd_pcm_substream,
    pub dwork: delayed_work,
    pub component_driver: *mut snd_soc_component_driver,
    pub dai_driver: *mut snd_soc_dai_driver,
    pub name: *mut test_dai_name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_adata {
    pub is_cpu:1: u32,
    pub cmp_v:1: u32,
    pub dai_v:1: u32,
}

    static int test_dai_set_sysclk(struct snd_soc_dai *dai,
    int clk_id, unsigned int freq, int dir)
    {
    mile_stone(dai);
    return 0;
    }
    static int test_dai_set_pll(struct snd_soc_dai *dai, int pll_id, int source,
    unsigned int freq_in, unsigned int freq_out)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    static int test_dai_set_clkdiv(struct snd_soc_dai *dai, int div_id, int div)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int test_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    let mut format: c_uint = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    let mut clock: c_uint = fmt & SND_SOC_DAIFMT_CLOCK_MASK;
    let mut inv: c_uint = fmt & SND_SOC_DAIFMT_INV_MASK;
    let mut master: c_uint = fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    char *str;
    dev_info(dai.dev, "name   : %s", dai.name);
    str = "unknown";
    switch (format) {
    case SND_SOC_DAIFMT_I2S:
    str = "i2s";
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    str = "right_j";
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    str = "left_j";
    break;
    case SND_SOC_DAIFMT_DSP_A:
    str = "dsp_a";
    break;
    case SND_SOC_DAIFMT_DSP_B:
    str = "dsp_b";
    break;
    case SND_SOC_DAIFMT_AC97:
    str = "ac97";
    break;
    case SND_SOC_DAIFMT_PDM:
    str = "pdm";
    break;
    }
    dev_info(dai.dev, "format : %s", str);
    if (clock == SND_SOC_DAIFMT_CONT)
    str = "continuous";
    else
    str = "gated";
    dev_info(dai.dev, "clock  : %s", str);
    str = "unknown";
    switch (master) {
    case SND_SOC_DAIFMT_BP_FP:
    str = "clk provider, frame provider";
    break;
    case SND_SOC_DAIFMT_BC_FP:
    str = "clk consumer, frame provider";
    break;
    case SND_SOC_DAIFMT_BP_FC:
    str = "clk provider, frame consumer";
    break;
    case SND_SOC_DAIFMT_BC_FC:
    str = "clk consumer, frame consumer";
    break;
    }
    dev_info(dai.dev, "clock  : codec is %s", str);
    str = "unknown";
    switch (inv) {
    case SND_SOC_DAIFMT_NB_NF:
    str = "normal bit, normal frame";
    break;
    case SND_SOC_DAIFMT_NB_IF:
    str = "normal bit, invert frame";
    break;
    case SND_SOC_DAIFMT_IB_NF:
    str = "invert bit, normal frame";
    break;
    case SND_SOC_DAIFMT_IB_IF:
    str = "invert bit, invert frame";
    break;
    }
    dev_info(dai.dev, "signal : %s", str);
    return 0;
    }
    static int test_dai_set_tdm_slot(struct snd_soc_dai *dai,
    unsigned int tx_mask, unsigned int rx_mask,
    int slots, int slot_width)
    {
    dev_info(dai.dev, "set tdm slot: tx_mask=0x%08X, rx_mask=0x%08X, slots=%d, slot_width=%d\n",
    tx_mask, rx_mask, slots, slot_width);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int test_dai_mute_stream(struct snd_soc_dai *dai, int mute, int stream)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int test_dai_startup(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    static void test_dai_shutdown(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    mile_stone(dai);
    }
    static int test_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static int test_dai_hw_free(struct snd_pcm_substream *substream, struct snd_soc_dai *dai)
    {
    mile_stone(dai);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    static int test_dai_trigger(struct snd_pcm_substream *substream, int cmd, struct snd_soc_dai *dai)
    {
    mile_stone(dai);
    return 0;
    }
    static const u64 test_dai_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_A	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_B	|
    SND_SOC_POSSIBLE_DAIFMT_AC97	|
    SND_SOC_POSSIBLE_DAIFMT_PDM	|
    SND_SOC_POSSIBLE_DAIFMT_NB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_NB_IF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;
    static const struct snd_soc_dai_ops test_ops = {
    .set_fmt		= test_dai_set_fmt,
    .set_tdm_slot		= test_dai_set_tdm_slot,
    .startup		= test_dai_startup,
    .shutdown		= test_dai_shutdown,
    .auto_selectable_formats	= &test_dai_formats,
    .num_auto_selectable_formats	= 1,
    };
    static const struct snd_soc_dai_ops test_verbose_ops = {
    .set_sysclk		= test_dai_set_sysclk,
    .set_pll		= test_dai_set_pll,
    .set_clkdiv		= test_dai_set_clkdiv,
    .set_fmt		= test_dai_set_fmt,
    .set_tdm_slot		= test_dai_set_tdm_slot,
    .mute_stream		= test_dai_mute_stream,
    .startup		= test_dai_startup,
    .shutdown		= test_dai_shutdown,
    .hw_params		= test_dai_hw_params,
    .hw_free		= test_dai_hw_free,
    .trigger		= test_dai_trigger,
    .auto_selectable_formats	= &test_dai_formats,
    .num_auto_selectable_formats	= 1,
    };

    SNDRV_PCM_FMTBIT_U8		| \
    SNDRV_PCM_FMTBIT_S16_LE	| \
    SNDRV_PCM_FMTBIT_U16_LE	| \
    SNDRV_PCM_FMTBIT_S24_LE	| \
    SNDRV_PCM_FMTBIT_S24_3LE	| \
    SNDRV_PCM_FMTBIT_U24_LE	| \
    SNDRV_PCM_FMTBIT_S32_LE	| \
    SNDRV_PCM_FMTBIT_U32_LE)
#[no_mangle]
unsafe extern "C" fn test_component_probe(component: *mut snd_soc_component) -> c_int {
    static int test_component_probe(struct snd_soc_component *component)
    {
    mile_stone(component);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_component_remove(component: *mut snd_soc_component) {
    static void test_component_remove(struct snd_soc_component *component)
    {
    mile_stone(component);
    }
#[no_mangle]
unsafe extern "C" fn test_component_suspend(component: *mut snd_soc_component) -> c_int {
    static int test_component_suspend(struct snd_soc_component *component)
    {
    mile_stone(component);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_component_resume(component: *mut snd_soc_component) -> c_int {
    static int test_component_resume(struct snd_soc_component *component)
    {
    mile_stone(component);
    return 0;
    }

    static int test_component_pcm_new(struct snd_soc_component *component,
    struct snd_soc_pcm_runtime *rtd)
    {
    mile_stone(component);
    snd_pcm_set_managed_buffer_all(
    rtd.pcm,
    SNDRV_DMA_TYPE_DEV,
    rtd.card.snd_card.dev,
    PREALLOC_BUFFER, PREALLOC_BUFFER);
    return 0;
    }
    static void test_component_pcm_free(struct snd_soc_component *component,
    struct snd_pcm *pcm)
    {
    mile_stone(component);
    }
    static int test_component_set_sysclk(struct snd_soc_component *component,
    int clk_id, int source, unsigned int freq, int dir)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_set_pll(struct snd_soc_component *component, int pll_id,
    int source, unsigned int freq_in, unsigned int freq_out)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_set_jack(struct snd_soc_component *component,
    struct snd_soc_jack *jack,  void *data)
    {
    mile_stone(component);
    return 0;
    }
    static void test_component_seq_notifier(struct snd_soc_component *component,
    enum snd_soc_dapm_type type, int subseq)
    {
    mile_stone(component);
    }
#[no_mangle]
unsafe extern "C" fn test_component_stream_event(component: *mut snd_soc_component, event: c_int) -> c_int {
    static int test_component_stream_event(struct snd_soc_component *component, int event)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    mile_stone(component);
    return 0;
    }
    static const struct snd_pcm_hardware test_component_hardware = {
// Random values to keep userspace happy when checking constraints
    .info			= SNDRV_PCM_INFO_INTERLEAVED	|
    SNDRV_PCM_INFO_MMAP		|
    SNDRV_PCM_INFO_MMAP_VALID,
    .buffer_bytes_max	= 32 * 1024,
    .period_bytes_min	= 32,
    .period_bytes_max	= 8192,
    .periods_min		= 1,
    .periods_max		= 128,
    .fifo_size		= 256,
    };
    static int test_component_open(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    mile_stone(component);
// BE's dont need dummy params
    if (!rtd.dai_link.no_pcm)
    snd_soc_set_runtime_hwparams(substream, &test_component_hardware);
    return 0;
    }
    static int test_component_close(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_ioctl(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    unsigned int cmd, void *arg)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_hw_params(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_hw_free(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_prepare(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    mile_stone(component);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_component_timer_stop(priv: *mut test_priv) {
    static void test_component_timer_stop(struct test_priv *priv)
    {
    cancel_delayed_work(&priv.dwork);
    }
#[no_mangle]
unsafe extern "C" fn test_component_timer_start(priv: *mut test_priv) {
    static void test_component_timer_start(struct test_priv *priv)
    {
    schedule_delayed_work(&priv.dwork, msecs_to_jiffies(10));
    }
#[no_mangle]
unsafe extern "C" fn test_component_dwork(work: *mut work_struct) {
    static void test_component_dwork(struct work_struct *work)
    {
    struct test_priv *priv = container_of(work, struct test_priv, dwork.work);
    if (priv.substream)
    snd_pcm_period_elapsed(priv.substream);
    test_component_timer_start(priv);
    }
    static int test_component_trigger(struct snd_soc_component *component,
    struct snd_pcm_substream *substream, int cmd)
    {
    struct test_priv *priv = dev_get_drvdata(component.dev);
    mile_stone(component);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    test_component_timer_start(priv);
    priv.substream = substream; /* set substream later */
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    priv.substream = core::ptr::null_mut();
    test_component_timer_stop(priv);
    }
    return 0;
    }
    static int test_component_sync_stop(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    mile_stone(component);
    return 0;
    }
    static snd_pcm_uframes_t test_component_pointer(struct snd_soc_component *component,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    static int pointer;
    if (!runtime)
    return 0;
    pointer += 10;
    if (pointer > PREALLOC_BUFFER)
    pointer = 0;
// mile_stone(component);
    return bytes_to_frames(runtime, pointer);
    }
    static int test_component_get_time_info(struct snd_soc_component *component,
    struct snd_pcm_substream *substream,
    struct timespec64 *system_ts,
    struct timespec64 *audio_ts,
    struct snd_pcm_audio_tstamp_config *audio_tstamp_config,
    struct snd_pcm_audio_tstamp_report *audio_tstamp_report)
    {
    mile_stone(component);
    return 0;
    }
    static int test_component_be_hw_params_fixup(struct snd_soc_pcm_runtime *rtd,
    struct snd_pcm_hw_params *params)
    {
    mile_stone_x(rtd.dev);
    return 0;
    }
// CPU
    let mut test_cpu: static struct test_adata = { .is_cpu = 1, .cmp_v = 0, .dai_v = 0, };
    let mut test_cpu_vv: static struct test_adata = { .is_cpu = 1, .cmp_v = 1, .dai_v = 1, };
    let mut test_cpu_nv: static struct test_adata = { .is_cpu = 1, .cmp_v = 0, .dai_v = 1, };
    let mut test_cpu_vn: static struct test_adata = { .is_cpu = 1, .cmp_v = 1, .dai_v = 0, };
// Codec
    let mut test_codec: static struct test_adata = { .is_cpu = 0, .cmp_v = 0, .dai_v = 0, };
    let mut test_codec_vv: static struct test_adata = { .is_cpu = 0, .cmp_v = 1, .dai_v = 1, };
    let mut test_codec_nv: static struct test_adata = { .is_cpu = 0, .cmp_v = 0, .dai_v = 1, };
    let mut test_codec_vn: static struct test_adata = { .is_cpu = 0, .cmp_v = 1, .dai_v = 0, };
    static const struct of_device_id test_of_match[] = {
    { .compatible = "test-cpu",			.data = (void *)&test_cpu,    },
    { .compatible = "test-cpu-verbose",		.data = (void *)&test_cpu_vv, },
    { .compatible = "test-cpu-verbose-dai",		.data = (void *)&test_cpu_nv, },
    { .compatible = "test-cpu-verbose-component",	.data = (void *)&test_cpu_vn, },
    { .compatible = "test-codec",			.data = (void *)&test_codec,    },
    { .compatible = "test-codec-verbose",		.data = (void *)&test_codec_vv, },
    { .compatible = "test-codec-verbose-dai",	.data = (void *)&test_codec_nv, },
    { .compatible = "test-codec-verbose-component",	.data = (void *)&test_codec_vn, },
    {},
    };
    MODULE_DEVICE_TABLE(of, test_of_match);
    static const struct snd_soc_dapm_widget widgets[] = {
//
// FIXME
//
// Just IN/OUT is OK for now,
// but need to be updated ?
//
    SND_SOC_DAPM_INPUT("IN"),
    SND_SOC_DAPM_OUTPUT("OUT"),
    };
#[no_mangle]
unsafe extern "C" fn test_driver_probe(pdev: *mut platform_device) -> c_int {
    static int test_driver_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    const struct test_adata *adata = of_device_get_match_data(&pdev.dev);
    struct snd_soc_component_driver *cdriv;
    struct snd_soc_dai_driver *ddriv;
    struct test_dai_name *dname;
    struct test_priv *priv;
    int num, ret, i;
    num = of_graph_get_endpoint_count(node);
    if (!num) {
    dev_err(dev, "no port exits\n");
    return -EINVAL;
    }
    priv	= devm_kzalloc(dev, sizeof(*priv),		GFP_KERNEL);
    cdriv	= devm_kzalloc(dev, sizeof(*cdriv),		GFP_KERNEL);
    ddriv	= devm_kcalloc(dev, num, sizeof(*ddriv), 	GFP_KERNEL);
    dname	= devm_kcalloc(dev, num, sizeof(*dname), 	GFP_KERNEL);
    if (!priv || !cdriv || !ddriv || !dname || !adata)
    return -EINVAL;
    priv.dev		= dev;
    priv.component_driver	= cdriv;
    priv.dai_driver	= ddriv;
    priv.name		= dname;
    INIT_DELAYED_WORK(&priv.dwork, test_component_dwork);
    dev_set_drvdata(dev, priv);
    if (adata.is_cpu) {
    cdriv.name			= "test_cpu";
    cdriv.pcm_new			= test_component_pcm_new;
    cdriv.pointer			= test_component_pointer;
    cdriv.trigger			= test_component_trigger;
    cdriv.legacy_dai_naming	= 1;
    } else {
    cdriv.name			= "test_codec";
    cdriv.idle_bias_on		= 1;
    cdriv.endianness		= 1;
    }
    cdriv.open		= test_component_open;
    cdriv.dapm_widgets	= widgets;
    cdriv.num_dapm_widgets	= ARRAY_SIZE(widgets);
    if (adata.cmp_v) {
    cdriv.probe			= test_component_probe;
    cdriv.remove			= test_component_remove;
    cdriv.suspend			= test_component_suspend;
    cdriv.resume			= test_component_resume;
    cdriv.set_sysclk		= test_component_set_sysclk;
    cdriv.set_pll			= test_component_set_pll;
    cdriv.set_jack			= test_component_set_jack;
    cdriv.seq_notifier		= test_component_seq_notifier;
    cdriv.stream_event		= test_component_stream_event;
    cdriv.set_bias_level		= test_component_set_bias_level;
    cdriv.close			= test_component_close;
    cdriv.ioctl			= test_component_ioctl;
    cdriv.hw_params		= test_component_hw_params;
    cdriv.hw_free			= test_component_hw_free;
    cdriv.prepare			= test_component_prepare;
    cdriv.sync_stop		= test_component_sync_stop;
    cdriv.get_time_info		= test_component_get_time_info;
    cdriv.be_hw_params_fixup	= test_component_be_hw_params_fixup;
    if (adata.is_cpu)
    cdriv.pcm_free	= test_component_pcm_free;
    }
    i = 0;
    for_each_of_graph_port(node, port) {
    snprintf(dname[i].name, TEST_NAME_LEN, "%s.%d", node.name, i);
    ddriv[i].name = dname[i].name;
    snprintf(dname[i].name_playback, TEST_NAME_LEN, "DAI%d Playback", i);
    ddriv[i].playback.stream_name	= dname[i].name_playback;
    ddriv[i].playback.channels_min	= 1;
    ddriv[i].playback.channels_max	= 384;
    ddriv[i].playback.rates		= STUB_RATES;
    ddriv[i].playback.formats	= STUB_FORMATS;
    snprintf(dname[i].name_capture, TEST_NAME_LEN, "DAI%d Capture", i);
    ddriv[i].capture.stream_name	= dname[i].name_capture;
    ddriv[i].capture.channels_min	= 1;
    ddriv[i].capture.channels_max	= 384;
    ddriv[i].capture.rates		= STUB_RATES;
    ddriv[i].capture.formats	= STUB_FORMATS;
    if (adata.dai_v)
    ddriv[i].ops = &test_verbose_ops;
    else
    ddriv[i].ops = &test_ops;
    i++;
    }
    ret = devm_snd_soc_register_component(dev, cdriv, ddriv, num);
    if (ret < 0)
    return ret;
    mile_stone_x(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_driver_remove(pdev: *mut platform_device) {
    static void test_driver_remove(struct platform_device *pdev)
    {
    mile_stone_x(&pdev.dev);
    }
    static struct platform_driver test_driver = {
    .driver = {
    .name = "test-component",
    .of_match_table = test_of_match,
    },
    .probe  = test_driver_probe,
    .remove = test_driver_remove,
    };
    module_platform_driver(test_driver);
    MODULE_ALIAS("platform:asoc-test-component");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
    MODULE_DESCRIPTION("ASoC Test Component");
    MODULE_LICENSE("GPL v2");
