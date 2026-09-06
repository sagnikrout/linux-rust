//! Automatically rewritten from C to Rust
//! Source: sound/soc/spear/spdif_in.c
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
// ALSA SoC SPDIF In Audio Layer for spear processors
//
// Copyright (C) 2012 ST Microelectronics
// Vipin Kumar <vipin.kumar@st.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spdif_in_params {
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spdif_in_dev {
    pub clk: *mut clk,
    pub dma_params: spear_dma_data,
    pub saved_params: spdif_in_params,
    pub io_base: *mut c_void,
    pub dev: *mut device,
    pub (*reset_perip)(void): *mut c_void,
    pub irq: c_int,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub config: snd_dmaengine_pcm_config,
}

#[no_mangle]
unsafe extern "C" fn spdif_in_configure(host: *mut spdif_in_dev) {
    static void spdif_in_configure(struct spdif_in_dev *host)
    {
    u32 ctrl = SPDIF_IN_PRTYEN | SPDIF_IN_STATEN | SPDIF_IN_USREN |
    SPDIF_IN_VALEN | SPDIF_IN_BLKEN;
    ctrl |= SPDIF_MODE_16BIT | SPDIF_FIFO_THRES_16;
    writel(ctrl, host.io_base + SPDIF_IN_CTRL);
    writel(0xF, host.io_base + SPDIF_IN_IRQ_MASK);
    }
#[no_mangle]
unsafe extern "C" fn spdif_in_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int spdif_in_dai_probe(struct snd_soc_dai *dai)
    {
    struct spdif_in_dev *host = snd_soc_dai_get_drvdata(dai);
    host.dma_params_rx.filter_data = &host.dma_params;
    snd_soc_dai_dma_data_set_capture(dai, &host.dma_params_rx);
    return 0;
    }
    static void spdif_in_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct spdif_in_dev *host = snd_soc_dai_get_drvdata(dai);
    if (substream.stream != SNDRV_PCM_STREAM_CAPTURE)
    return;
    writel(0x0, host.io_base + SPDIF_IN_IRQ_MASK);
    }
#[no_mangle]
unsafe extern "C" fn spdif_in_format(host: *mut spdif_in_dev, format: u32) {
    static void spdif_in_format(struct spdif_in_dev *host, u32 format)
    {
    let mut ctrl: u32 = readl(host.io_base + SPDIF_IN_CTRL);
    switch (format) {
    case SNDRV_PCM_FORMAT_S16_LE:
    ctrl |= SPDIF_XTRACT_16BIT;
    break;
    case SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE:
    ctrl &= ~SPDIF_XTRACT_16BIT;
    break;
    }
    writel(ctrl, host.io_base + SPDIF_IN_CTRL);
    }
    static int spdif_in_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct spdif_in_dev *host = snd_soc_dai_get_drvdata(dai);
    u32 format;
    if (substream.stream != SNDRV_PCM_STREAM_CAPTURE)
    return -EINVAL;
    format = params_format(params);
    host.saved_params.format = format;
    return 0;
    }
    static int spdif_in_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct spdif_in_dev *host = snd_soc_dai_get_drvdata(dai);
    u32 ctrl;
    let mut ret: c_int = 0;
    if (substream.stream != SNDRV_PCM_STREAM_CAPTURE)
    return -EINVAL;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    clk_enable(host.clk);
    spdif_in_configure(host);
    spdif_in_format(host, host.saved_params.format);
    ctrl = readl(host.io_base + SPDIF_IN_CTRL);
    ctrl |= SPDIF_IN_SAMPLE | SPDIF_IN_ENB;
    writel(ctrl, host.io_base + SPDIF_IN_CTRL);
    writel(0xF, host.io_base + SPDIF_IN_IRQ_MASK);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    ctrl = readl(host.io_base + SPDIF_IN_CTRL);
    ctrl &= ~(SPDIF_IN_SAMPLE | SPDIF_IN_ENB);
    writel(ctrl, host.io_base + SPDIF_IN_CTRL);
    writel(0x0, host.io_base + SPDIF_IN_IRQ_MASK);
    if (host.reset_perip)
    host.reset_perip();
    clk_disable(host.clk);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct snd_soc_dai_ops spdif_in_dai_ops = {
    .shutdown	= spdif_in_shutdown,
    .probe = spdif_in_dai_probe,
    .trigger	= spdif_in_trigger,
    .hw_params	= spdif_in_hw_params,
    };
    static struct snd_soc_dai_driver spdif_in_dai = {
    .capture = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = (SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | \
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | \
    SNDRV_PCM_RATE_192000),
    .formats = SNDRV_PCM_FMTBIT_S16_LE | \
    SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
    },
    .ops = &spdif_in_dai_ops,
    };
    static const struct snd_soc_component_driver spdif_in_component = {
    .name			= "spdif-in",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn spdif_in_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t spdif_in_irq(int irq, void *arg)
    {
    struct spdif_in_dev *host = (struct spdif_in_dev *)arg;
    let mut irq_status: u32 = readl(host.io_base + SPDIF_IN_IRQ);
    if (!irq_status)
    return IRQ_NONE;
    if (irq_status & SPDIF_IRQ_FIFOWRITE)
    dev_err(host.dev, "spdif in: fifo write error");
    if (irq_status & SPDIF_IRQ_EMPTYFIFOREAD)
    dev_err(host.dev, "spdif in: empty fifo read error");
    if (irq_status & SPDIF_IRQ_FIFOFULL)
    dev_err(host.dev, "spdif in: fifo full error");
    if (irq_status & SPDIF_IRQ_OUTOFRANGE)
    dev_err(host.dev, "spdif in: out of range error");
    writel(0, host.io_base + SPDIF_IN_IRQ);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spdif_in_probe(pdev: *mut platform_device) -> c_int {
    static int spdif_in_probe(struct platform_device *pdev)
    {
    struct spdif_in_dev *host;
    struct spear_spdif_platform_data *pdata;
    struct resource *res_fifo;
    void __iomem *io_base;
    int ret;
    io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(io_base))
    return PTR_ERR(io_base);
    res_fifo = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res_fifo)
    return -EINVAL;
    host = devm_kzalloc(&pdev.dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    host.io_base = io_base;
    host.irq = platform_get_irq(pdev, 0);
    if (host.irq < 0)
    return host.irq;
    host.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return PTR_ERR(host.clk);
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata)
    return -EINVAL;
    host.dma_params.data = pdata.dma_params;
    host.dma_params.addr = res_fifo.start;
    host.dma_params.max_burst = 16;
    host.dma_params.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    host.reset_perip = pdata.reset_perip;
    host.dev = &pdev.dev;
    dev_set_drvdata(&pdev.dev, host);
    ret = devm_request_irq(&pdev.dev, host.irq, spdif_in_irq, 0,
    "spdif-in", host);
    if (ret)
    return ret;
    ret = devm_snd_soc_register_component(&pdev.dev, &spdif_in_component,
    &spdif_in_dai, 1);
    if (ret)
    return ret;
    return devm_spear_pcm_platform_register(&pdev.dev, &host.config,
    pdata.filter);
    }
    static struct platform_driver spdif_in_driver = {
    .probe		= spdif_in_probe,
    .driver		= {
    .name	= "spdif-in",
    },
    };
    module_platform_driver(spdif_in_driver);
    MODULE_AUTHOR("Vipin Kumar <vipin.kumar@st.com>");
    MODULE_DESCRIPTION("SPEAr SPDIF IN SoC Interface");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:spdif_in");
