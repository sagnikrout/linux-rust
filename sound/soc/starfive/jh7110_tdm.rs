//! Automatically rewritten from C to Rust
//! Source: sound/soc/starfive/jh7110_tdm.c
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
// jh7110_tdm.c -- StarFive JH7110 TDM driver
//
// Copyright (C) 2023 StarFive Technology Co., Ltd.
//
// Author: Walker Chen <walker.chen@starfivetech.com>
//

pub const TDM_PCMGBCR: c_uint = 0x00;

pub const CLKPOL_BIT: c_int = 5;
pub const ELM_BIT: c_int = 3;
pub const SYNCM_BIT: c_int = 2;
pub const MS_BIT: c_int = 1;
pub const TDM_PCMTXCR: c_uint = 0x04;

pub const IFL_BIT: c_int = 11;
pub const WL_BIT: c_int = 8;
pub const SSCALE_BIT: c_int = 4;
pub const SL_BIT: c_int = 2;
pub const LRJ_BIT: c_int = 1;
pub const TDM_PCMRXCR: c_uint = 0x08;

pub const TDM_PCMDIV: c_uint = 0x0c;
pub const JH7110_TDM_FIFO: c_uint = 0x170c0000;
pub const JH7110_TDM_FIFO_DEPTH: c_int = 32;
    enum TDM_MASTER_SLAVE_MODE {
    TDM_AS_MASTER = 0,
    TDM_AS_SLAVE,
    };
    enum TDM_CLKPOL {
// tx raising and rx falling
    TDM_TX_RASING_RX_FALLING = 0,
// tx falling and rx raising
    TDM_TX_FALLING_RX_RASING,
    };
    enum TDM_ELM {
// only work while SYNCM=0
    TDM_ELM_LATE = 0,
    TDM_ELM_EARLY,
    };
    enum TDM_SYNCM {
// short frame sync
    TDM_SYNCM_SHORT = 0,
// long frame sync
    TDM_SYNCM_LONG,
    };
    enum TDM_IFL {
// FIFO to send or received : half-1/2, Quarter-1/4
    TDM_FIFO_HALF = 0,
    TDM_FIFO_QUARTER,
    };
    enum TDM_WL {
// send or received word length
    TDM_8BIT_WORD_LEN = 0,
    TDM_16BIT_WORD_LEN,
    TDM_20BIT_WORD_LEN,
    TDM_24BIT_WORD_LEN,
    TDM_32BIT_WORD_LEN,
    };
    enum TDM_SL {
// send or received slot length
    TDM_8BIT_SLOT_LEN = 0,
    TDM_16BIT_SLOT_LEN,
    TDM_32BIT_SLOT_LEN,
    };
    enum TDM_LRJ {
// left-justify or right-justify
    TDM_RIGHT_JUSTIFY = 0,
    TDM_LEFT_JUSTIFT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdm_chan_cfg {
    pub ifl: enum TDM_IFL,
    pub wl: enum TDM_WL,
    pub sscale: c_uchar,
    pub sl: enum TDM_SL,
    pub lrj: enum TDM_LRJ,
    pub enable: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_tdm_dev {
    pub tdm_base: *mut void __iomem,
    pub dev: *mut device,
    pub clks: [clk_bulk_data; 6],
    pub resets: *mut reset_control,
    pub clkpolity: enum TDM_CLKPOL,
    pub elm: enum TDM_ELM,
    pub syncm: enum TDM_SYNCM,
    pub ms_mode: enum TDM_MASTER_SLAVE_MODE,
    pub tx: tdm_chan_cfg,
    pub rx: tdm_chan_cfg,
    pub syncdiv: u16,
    pub samplerate: u32,
    pub pcmclk: u32,
// data related to DMA transfers b/w tdm and DMAC
    pub play_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub saved_pcmgbcr: u32,
    pub saved_pcmtxcr: u32,
    pub saved_pcmrxcr: u32,
    pub saved_pcmdiv: u32,
}

#[no_mangle]
pub unsafe extern "C" fn jh7110_tdm_readl(tdm: *mut jh7110_tdm_dev, reg: u16) -> u32 {
    static inline u32 jh7110_tdm_readl(struct jh7110_tdm_dev *tdm, u16 reg)
    {
    return readl_relaxed(tdm.tdm_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn jh7110_tdm_writel(tdm: *mut jh7110_tdm_dev, reg: u16, val: u32) {
    static inline void jh7110_tdm_writel(struct jh7110_tdm_dev *tdm, u16 reg, u32 val)
    {
    writel_relaxed(val, tdm.tdm_base + reg);
    }
    static void jh7110_tdm_save_context(struct jh7110_tdm_dev *tdm,
    struct snd_pcm_substream *substream)
    {
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    tdm.saved_pcmtxcr = jh7110_tdm_readl(tdm, TDM_PCMTXCR);
    else
    tdm.saved_pcmrxcr = jh7110_tdm_readl(tdm, TDM_PCMRXCR);
    }
    static void jh7110_tdm_start(struct jh7110_tdm_dev *tdm,
    struct snd_pcm_substream *substream)
    {
    u32 data;
    data = jh7110_tdm_readl(tdm, TDM_PCMGBCR);
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, data | PCMGBCR_ENABLE);
// restore context
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    jh7110_tdm_writel(tdm, TDM_PCMTXCR, tdm.saved_pcmtxcr | PCMTXCR_TXEN);
    else
    jh7110_tdm_writel(tdm, TDM_PCMRXCR, tdm.saved_pcmrxcr | PCMRXCR_RXEN);
    }
    static void jh7110_tdm_stop(struct jh7110_tdm_dev *tdm,
    struct snd_pcm_substream *substream)
    {
    unsigned int val;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    val = jh7110_tdm_readl(tdm, TDM_PCMTXCR);
    val &= ~PCMTXCR_TXEN;
    jh7110_tdm_writel(tdm, TDM_PCMTXCR, val);
    } else {
    val = jh7110_tdm_readl(tdm, TDM_PCMRXCR);
    val &= ~PCMRXCR_RXEN;
    jh7110_tdm_writel(tdm, TDM_PCMRXCR, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_syncdiv(tdm: *mut jh7110_tdm_dev) -> c_int {
    static int jh7110_tdm_syncdiv(struct jh7110_tdm_dev *tdm)
    {
    u32 sl, sscale, syncdiv;
    sl = max(tdm.rx.sl, tdm.tx.sl);
    sscale = max(tdm.rx.sscale, tdm.tx.sscale);
    syncdiv = tdm.pcmclk / tdm.samplerate - 1;
    if ((syncdiv + 1) < (sl * sscale)) {
    dev_err(tdm.dev, "Failed to set syncdiv!\n");
    return -EINVAL;
    }
    if (tdm.syncm == TDM_SYNCM_LONG &&
    (tdm.rx.sscale <= 1 || tdm.tx.sscale <= 1) &&
    ((syncdiv + 1) <= sl)) {
    dev_err(tdm.dev, "Wrong syncdiv! It must be (syncdiv+1) > max[tx.sl, rx.sl]\n");
    return -EINVAL;
    }
    jh7110_tdm_writel(tdm, TDM_PCMDIV, syncdiv);
    return 0;
    }
    static int jh7110_tdm_config(struct jh7110_tdm_dev *tdm,
    struct snd_pcm_substream *substream)
    {
    u32 datarx, datatx;
    int ret;
    ret = jh7110_tdm_syncdiv(tdm);
    if (ret)
    return ret;
    datarx = (tdm.rx.ifl << IFL_BIT) |
    (tdm.rx.wl << WL_BIT) |
    (tdm.rx.sscale << SSCALE_BIT) |
    (tdm.rx.sl << SL_BIT) |
    (tdm.rx.lrj << LRJ_BIT);
    datatx = (tdm.tx.ifl << IFL_BIT) |
    (tdm.tx.wl << WL_BIT) |
    (tdm.tx.sscale << SSCALE_BIT) |
    (tdm.tx.sl << SL_BIT) |
    (tdm.tx.lrj << LRJ_BIT);
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    jh7110_tdm_writel(tdm, TDM_PCMTXCR, datatx);
    else
    jh7110_tdm_writel(tdm, TDM_PCMRXCR, datarx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_clk_disable(tdm: *mut jh7110_tdm_dev) {
    static void jh7110_tdm_clk_disable(struct jh7110_tdm_dev *tdm)
    {
    clk_bulk_disable_unprepare(ARRAY_SIZE(tdm.clks), tdm.clks);
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_clk_enable(tdm: *mut jh7110_tdm_dev) -> c_int {
    static int jh7110_tdm_clk_enable(struct jh7110_tdm_dev *tdm)
    {
    int ret;
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(tdm.clks), tdm.clks);
    if (ret) {
    dev_err(tdm.dev, "Failed to enable tdm clocks\n");
    return ret;
    }
    ret = reset_control_deassert(tdm.resets);
    if (ret) {
    dev_err(tdm.dev, "Failed to deassert tdm resets\n");
    goto dis_tdm_clk;
    }
// select tdm_ext clock as the clock source for tdm
    ret = clk_set_parent(tdm.clks[5].clk, tdm.clks[4].clk);
    if (ret) {
    dev_err(tdm.dev, "Can't set extern clock source for clk_tdm\n");
    goto dis_tdm_clk;
    }
    return 0;
    dis_tdm_clk:
    clk_bulk_disable_unprepare(ARRAY_SIZE(tdm.clks), tdm.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_runtime_suspend(dev: *mut device) -> c_int {
    static int jh7110_tdm_runtime_suspend(struct device *dev)
    {
    struct jh7110_tdm_dev *tdm = dev_get_drvdata(dev);
    jh7110_tdm_clk_disable(tdm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_runtime_resume(dev: *mut device) -> c_int {
    static int jh7110_tdm_runtime_resume(struct device *dev)
    {
    struct jh7110_tdm_dev *tdm = dev_get_drvdata(dev);
    return jh7110_tdm_clk_enable(tdm);
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_system_suspend(dev: *mut device) -> c_int {
    static int jh7110_tdm_system_suspend(struct device *dev)
    {
    struct jh7110_tdm_dev *tdm = dev_get_drvdata(dev);
// save context
    tdm.saved_pcmgbcr = jh7110_tdm_readl(tdm, TDM_PCMGBCR);
    tdm.saved_pcmdiv = jh7110_tdm_readl(tdm, TDM_PCMDIV);
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_system_resume(dev: *mut device) -> c_int {
    static int jh7110_tdm_system_resume(struct device *dev)
    {
    struct jh7110_tdm_dev *tdm = dev_get_drvdata(dev);
// restore context
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, tdm.saved_pcmgbcr);
    jh7110_tdm_writel(tdm, TDM_PCMDIV, tdm.saved_pcmdiv);
    return pm_runtime_force_resume(dev);
    }
    static const struct snd_soc_component_driver jh7110_tdm_component = {
    .name = "jh7110-tdm",
    };
    static int jh7110_tdm_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *cpu_dai)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai_link *dai_link = rtd.dai_link;
    dai_link.trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;
    return 0;
    }
    static int jh7110_tdm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct jh7110_tdm_dev *tdm = snd_soc_dai_get_drvdata(dai);
    int chan_wl, chan_sl, chan_nr;
    unsigned int data_width;
    unsigned int dma_bus_width;
    struct snd_dmaengine_dai_dma_data *dma_data = core::ptr::null_mut();
    int ret;
    data_width = params_width(params);
    tdm.samplerate = params_rate(params);
    tdm.pcmclk = params_channels(params) * tdm.samplerate * data_width;
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    chan_wl = TDM_16BIT_WORD_LEN;
    chan_sl = TDM_16BIT_SLOT_LEN;
    dma_bus_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    chan_wl = TDM_32BIT_WORD_LEN;
    chan_sl = TDM_32BIT_SLOT_LEN;
    dma_bus_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    default:
    dev_err(tdm.dev, "tdm: unsupported PCM fmt");
    return -EINVAL;
    }
    chan_nr = params_channels(params);
    switch (chan_nr) {
    case 1:
    case 2:
    case 4:
    case 6:
    case 8:
    break;
    default:
    dev_err(tdm.dev, "channel not supported\n");
    return -EINVAL;
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    tdm.tx.wl = chan_wl;
    tdm.tx.sl = chan_sl;
    tdm.tx.sscale = chan_nr;
    tdm.play_dma_data.addr_width = dma_bus_width;
    dma_data = &tdm.play_dma_data;
    } else {
    tdm.rx.wl = chan_wl;
    tdm.rx.sl = chan_sl;
    tdm.rx.sscale = chan_nr;
    tdm.capture_dma_data.addr_width = dma_bus_width;
    dma_data = &tdm.capture_dma_data;
    }
    snd_soc_dai_set_dma_data(dai, substream, dma_data);
    ret = jh7110_tdm_config(tdm, substream);
    if (ret)
    return ret;
    jh7110_tdm_save_context(tdm, substream);
    return 0;
    }
    static int jh7110_tdm_trigger(struct snd_pcm_substream *substream,
    int cmd, struct snd_soc_dai *dai)
    {
    struct jh7110_tdm_dev *tdm = snd_soc_dai_get_drvdata(dai);
    let mut ret: c_int = 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    jh7110_tdm_start(tdm, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    jh7110_tdm_stop(tdm, substream);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int jh7110_tdm_set_dai_fmt(struct snd_soc_dai *cpu_dai,
    unsigned int fmt)
    {
    struct jh7110_tdm_dev *tdm = snd_soc_dai_get_drvdata(cpu_dai);
    unsigned int gbcr;
// set master/slave audio interface
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BP_FP:
// cpu is master
    tdm.ms_mode = TDM_AS_MASTER;
    break;
    case SND_SOC_DAIFMT_BC_FC:
// codec is master
    tdm.ms_mode = TDM_AS_SLAVE;
    break;
    case SND_SOC_DAIFMT_BC_FP:
    case SND_SOC_DAIFMT_BP_FC:
    return -EINVAL;
    default:
    dev_dbg(tdm.dev, "dwc : Invalid clock provider format\n");
    return -EINVAL;
    }
    gbcr = (tdm.clkpolity << CLKPOL_BIT) |
    (tdm.elm << ELM_BIT) |
    (tdm.syncm << SYNCM_BIT) |
    (tdm.ms_mode << MS_BIT);
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, gbcr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int jh7110_tdm_dai_probe(struct snd_soc_dai *dai)
    {
    struct jh7110_tdm_dev *tdm = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &tdm.play_dma_data, &tdm.capture_dma_data);
    snd_soc_dai_set_drvdata(dai, tdm);
    return 0;
    }
    static const struct snd_soc_dai_ops jh7110_tdm_dai_ops = {
    .probe		= jh7110_tdm_dai_probe,
    .startup	= jh7110_tdm_startup,
    .hw_params	= jh7110_tdm_hw_params,
    .trigger	= jh7110_tdm_trigger,
    .set_fmt	= jh7110_tdm_set_dai_fmt,
    };

    SNDRV_PCM_FMTBIT_S32_LE)
    static struct snd_soc_dai_driver jh7110_tdm_dai = {
    .name = "sf_tdm",
    .id = 0,
    .playback = {
    .stream_name    = "Playback",
    .channels_min   = 1,
    .channels_max   = 8,
    .rates          = JH7110_TDM_RATES,
    .formats        = JH7110_TDM_FORMATS,
    },
    .capture = {
    .stream_name    = "Capture",
    .channels_min   = 1,
    .channels_max   = 8,
    .rates          = JH7110_TDM_RATES,
    .formats        = JH7110_TDM_FORMATS,
    },
    .ops = &jh7110_tdm_dai_ops,
    .symmetric_rate = 1,
    };
    static const struct snd_pcm_hardware jh7110_pcm_hardware = {
    .info			= (SNDRV_PCM_INFO_MMAP		|
    SNDRV_PCM_INFO_MMAP_VALID	|
    SNDRV_PCM_INFO_PAUSE		|
    SNDRV_PCM_INFO_RESUME	|
    SNDRV_PCM_INFO_INTERLEAVED	|
    SNDRV_PCM_INFO_BLOCK_TRANSFER),
    .buffer_bytes_max	= 192512,
    .period_bytes_min	= 4096,
    .period_bytes_max	= 32768,
    .periods_min		= 1,
    .periods_max		= 48,
    .fifo_size		= 16,
    };
    static const struct snd_dmaengine_pcm_config jh7110_dmaengine_pcm_config = {
    .pcm_hardware = &jh7110_pcm_hardware,
    .prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config,
    .prealloc_buffer_size = 192512,
    };
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_init_params(tdm: *mut jh7110_tdm_dev) {
    static void jh7110_tdm_init_params(struct jh7110_tdm_dev *tdm)
    {
    tdm.clkpolity = TDM_TX_RASING_RX_FALLING;
    tdm.elm = TDM_ELM_LATE;
    tdm.syncm = TDM_SYNCM_SHORT;
    tdm.rx.ifl = TDM_FIFO_HALF;
    tdm.tx.ifl = TDM_FIFO_HALF;
    tdm.rx.wl = TDM_16BIT_WORD_LEN;
    tdm.tx.wl = TDM_16BIT_WORD_LEN;
    tdm.rx.sscale = 2;
    tdm.tx.sscale = 2;
    tdm.rx.lrj = TDM_LEFT_JUSTIFT;
    tdm.tx.lrj = TDM_LEFT_JUSTIFT;
    tdm.play_dma_data.addr = JH7110_TDM_FIFO;
    tdm.play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    tdm.play_dma_data.fifo_size = JH7110_TDM_FIFO_DEPTH / 2;
    tdm.play_dma_data.maxburst = 16;
    tdm.capture_dma_data.addr = JH7110_TDM_FIFO;
    tdm.capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    tdm.capture_dma_data.fifo_size = JH7110_TDM_FIFO_DEPTH / 2;
    tdm.capture_dma_data.maxburst = 8;
    }
    static int jh7110_tdm_clk_reset_get(struct platform_device *pdev,
    struct jh7110_tdm_dev *tdm)
    {
    int ret;
    tdm.clks[0].id = "mclk_inner";
    tdm.clks[1].id = "tdm_ahb";
    tdm.clks[2].id = "tdm_apb";
    tdm.clks[3].id = "tdm_internal";
    tdm.clks[4].id = "tdm_ext";
    tdm.clks[5].id = "tdm";
    ret = devm_clk_bulk_get(&pdev.dev, ARRAY_SIZE(tdm.clks), tdm.clks);
    if (ret)
    return ret;
    tdm.resets = devm_reset_control_array_get_exclusive(&pdev.dev);
    if (IS_ERR(tdm.resets)) {
    dev_err(&pdev.dev, "Failed to get tdm resets\n");
    return PTR_ERR(tdm.resets);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_probe(pdev: *mut platform_device) -> c_int {
    static int jh7110_tdm_probe(struct platform_device *pdev)
    {
    struct jh7110_tdm_dev *tdm;
    int ret;
    tdm = devm_kzalloc(&pdev.dev, sizeof(*tdm), GFP_KERNEL);
    if (!tdm)
    return -ENOMEM;
    tdm.tdm_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tdm.tdm_base))
    return PTR_ERR(tdm.tdm_base);
    tdm.dev = &pdev.dev;
    ret = jh7110_tdm_clk_reset_get(pdev, tdm);
    if (ret)
    return ret;
    jh7110_tdm_init_params(tdm);
    dev_set_drvdata(&pdev.dev, tdm);
    ret = devm_snd_soc_register_component(&pdev.dev, &jh7110_tdm_component,
    &jh7110_tdm_dai, 1);
    if (ret)
    return ret;
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev,
    &jh7110_dmaengine_pcm_config,
    SND_DMAENGINE_PCM_FLAG_COMPAT);
    if (ret)
    return ret;
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = jh7110_tdm_runtime_resume(&pdev.dev);
    if (ret) {
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_tdm_dev_remove(pdev: *mut platform_device) {
    static void jh7110_tdm_dev_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id jh7110_tdm_of_match[] = {
    { .compatible = "starfive,jh7110-tdm", },
    {}
    };
    MODULE_DEVICE_TABLE(of, jh7110_tdm_of_match);
    static const struct dev_pm_ops jh7110_tdm_pm_ops = {
    RUNTIME_PM_OPS(jh7110_tdm_runtime_suspend,
    jh7110_tdm_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(jh7110_tdm_system_suspend,
    jh7110_tdm_system_resume)
    };
    static struct platform_driver jh7110_tdm_driver = {
    .driver = {
    .name = "jh7110-tdm",
    .of_match_table = jh7110_tdm_of_match,
    .pm = pm_ptr(&jh7110_tdm_pm_ops),
    },
    .probe = jh7110_tdm_probe,
    .remove = jh7110_tdm_dev_remove,
    };
    module_platform_driver(jh7110_tdm_driver);
    MODULE_DESCRIPTION("StarFive JH7110 TDM ASoC Driver");
    MODULE_AUTHOR("Walker Chen <walker.chen@starfivetech.com>");
    MODULE_LICENSE("GPL");
