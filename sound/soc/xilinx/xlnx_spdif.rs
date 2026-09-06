//! Automatically rewritten from C to Rust
//! Source: sound/soc/xilinx/xlnx_spdif.c
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
// Xilinx ASoC SPDIF audio support
//
// Copyright (C) 2018 Xilinx, Inc.
//
// Author: Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>
//

    (SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | \
    SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | \
    SNDRV_PCM_RATE_192000)

pub const XSPDIF_IRQ_STS_REG: c_uint = 0x20;
pub const XSPDIF_IRQ_ENABLE_REG: c_uint = 0x28;
pub const XSPDIF_SOFT_RESET_REG: c_uint = 0x40;
pub const XSPDIF_CONTROL_REG: c_uint = 0x44;
pub const XSPDIF_CHAN_0_STS_REG: c_uint = 0x4C;
pub const XSPDIF_GLOBAL_IRQ_ENABLE_REG: c_uint = 0x1C;
pub const XSPDIF_CH_A_USER_DATA_REG_0: c_uint = 0x64;

pub const XSPDIF_CLOCK_CONFIG_BITS_SHIFT: c_int = 2;
pub const XSPDIF_SOFT_RESET_VALUE: c_uint = 0xA;
pub const MAX_CHANNELS: c_int = 2;
pub const AES_SAMPLE_WIDTH: c_int = 32;
pub const CH_STATUS_UPDATE_TIMEOUT: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spdif_dev_data {
    pub mode: u32,
    pub aclk: u32,
    pub rx_chsts_updated: bool,
    pub base: *mut void __iomem,
    pub axi_clk: *mut clk,
    pub chsts_q: wait_queue_head_t,
}

#[no_mangle]
unsafe extern "C" fn xlnx_spdifrx_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t xlnx_spdifrx_irq_handler(int irq, void *arg)
    {
    u32 val;
    struct spdif_dev_data *ctx = arg;
    val = readl(ctx.base + XSPDIF_IRQ_STS_REG);
    if (val & XSPDIF_CH_STS_MASK) {
    writel(val & XSPDIF_CH_STS_MASK,
    ctx.base + XSPDIF_IRQ_STS_REG);
    val = readl(ctx.base +
    XSPDIF_IRQ_ENABLE_REG);
    writel(val & ~XSPDIF_CH_STS_MASK,
    ctx.base + XSPDIF_IRQ_ENABLE_REG);
    ctx.rx_chsts_updated = true;
    wake_up_interruptible(&ctx.chsts_q);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
    static int xlnx_spdif_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    u32 val;
    struct spdif_dev_data *ctx = dev_get_drvdata(dai.dev);
    val = readl(ctx.base + XSPDIF_CONTROL_REG);
    val |= XSPDIF_FIFO_FLUSH_MASK;
    writel(val, ctx.base + XSPDIF_CONTROL_REG);
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    writel(XSPDIF_CH_STS_MASK,
    ctx.base + XSPDIF_IRQ_ENABLE_REG);
    writel(XSPDIF_GLOBAL_IRQ_ENABLE,
    ctx.base + XSPDIF_GLOBAL_IRQ_ENABLE_REG);
    }
    return 0;
    }
    static void xlnx_spdif_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct spdif_dev_data *ctx = dev_get_drvdata(dai.dev);
    writel(XSPDIF_SOFT_RESET_VALUE, ctx.base + XSPDIF_SOFT_RESET_REG);
    }
    static int xlnx_spdif_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    u32 val, clk_div, clk_cfg;
    struct spdif_dev_data *ctx = dev_get_drvdata(dai.dev);
    clk_div = DIV_ROUND_CLOSEST(ctx.aclk, MAX_CHANNELS * AES_SAMPLE_WIDTH *
    params_rate(params));
    switch (clk_div) {
    case 4:
    clk_cfg = 0;
    break;
    case 8:
    clk_cfg = 1;
    break;
    case 16:
    clk_cfg = 2;
    break;
    case 24:
    clk_cfg = 3;
    break;
    case 32:
    clk_cfg = 4;
    break;
    case 48:
    clk_cfg = 5;
    break;
    case 64:
    clk_cfg = 6;
    break;
    default:
    return -EINVAL;
    }
    val = readl(ctx.base + XSPDIF_CONTROL_REG);
    val &= ~XSPDIF_CLOCK_CONFIG_BITS_MASK;
    val |= clk_cfg << XSPDIF_CLOCK_CONFIG_BITS_SHIFT;
    writel(val, ctx.base + XSPDIF_CONTROL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx_stream_detect(dai: *mut snd_soc_dai) -> c_int {
    static int rx_stream_detect(struct snd_soc_dai *dai)
    {
    int err;
    struct spdif_dev_data *ctx = dev_get_drvdata(dai.dev);
    let mut jiffies: c_ulong = msecs_to_jiffies(CH_STATUS_UPDATE_TIMEOUT);
// start capture only if stream is detected within 40ms timeout
    err = wait_event_interruptible_timeout(ctx.chsts_q,
    ctx.rx_chsts_updated,
    jiffies);
    if (!err) {
    dev_err(dai.dev, "No streaming audio detected!\n");
    return -EINVAL;
    }
    ctx.rx_chsts_updated = false;
    return 0;
    }
    static int xlnx_spdif_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    u32 val;
    let mut ret: c_int = 0;
    struct spdif_dev_data *ctx = dev_get_drvdata(dai.dev);
    val = readl(ctx.base + XSPDIF_CONTROL_REG);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    val |= XSPDIF_CORE_ENABLE_MASK;
    writel(val, ctx.base + XSPDIF_CONTROL_REG);
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    ret = rx_stream_detect(dai);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    val &= ~XSPDIF_CORE_ENABLE_MASK;
    writel(val, ctx.base + XSPDIF_CONTROL_REG);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static const struct snd_soc_dai_ops xlnx_spdif_dai_ops = {
    .startup = xlnx_spdif_startup,
    .shutdown = xlnx_spdif_shutdown,
    .trigger = xlnx_spdif_trigger,
    .hw_params = xlnx_spdif_hw_params,
    };
    static struct snd_soc_dai_driver xlnx_spdif_tx_dai = {
    .name = "xlnx_spdif_tx",
    .playback = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = XLNX_SPDIF_RATES,
    .formats = XLNX_SPDIF_FORMATS,
    },
    .ops = &xlnx_spdif_dai_ops,
    };
    static struct snd_soc_dai_driver xlnx_spdif_rx_dai = {
    .name = "xlnx_spdif_rx",
    .capture = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = XLNX_SPDIF_RATES,
    .formats = XLNX_SPDIF_FORMATS,
    },
    .ops = &xlnx_spdif_dai_ops,
    };
    static const struct snd_soc_component_driver xlnx_spdif_component = {
    .name = "xlnx-spdif",
    .legacy_dai_naming = 1,
    };
    static const struct of_device_id xlnx_spdif_of_match[] = {
    { .compatible = "xlnx,spdif-2.0", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xlnx_spdif_of_match);
#[no_mangle]
unsafe extern "C" fn xlnx_spdif_probe(pdev: *mut platform_device) -> c_int {
    static int xlnx_spdif_probe(struct platform_device *pdev)
    {
    int ret;
    struct snd_soc_dai_driver *dai_drv;
    struct spdif_dev_data *ctx;
    struct device *dev = &pdev.dev;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.axi_clk = devm_clk_get_enabled(dev, "s_axi_aclk");
    if (IS_ERR(ctx.axi_clk))
    return dev_err_probe(dev, PTR_ERR(ctx.axi_clk),
    "failed to get s_axi_aclk\n");
    ctx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.base))
    return PTR_ERR(ctx.base);
    ret = device_property_read_u32(dev, "xlnx,spdif-mode", &ctx.mode);
    if (ret < 0)
    return dev_err_probe(dev, ret, "cannot get SPDIF mode\n");
    if (ctx.mode) {
    dai_drv = &xlnx_spdif_tx_dai;
    } else {
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    ret = devm_request_irq(dev, ret,
    xlnx_spdifrx_irq_handler,
    0, "XLNX_SPDIF_RX", ctx);
    if (ret)
    return ret;
    init_waitqueue_head(&ctx.chsts_q);
    dai_drv = &xlnx_spdif_rx_dai;
    }
    ret = device_property_read_u32(dev, "xlnx,aud_clk_i", &ctx.aclk);
    if (ret < 0)
    return dev_err_probe(dev, ret, "cannot get aud_clk_i value\n");
    dev_set_drvdata(dev, ctx);
    ret = devm_snd_soc_register_component(dev, &xlnx_spdif_component,
    dai_drv, 1);
    if (ret)
    return ret;
    writel(XSPDIF_SOFT_RESET_VALUE, ctx.base + XSPDIF_SOFT_RESET_REG);
    dev_info(dev, "%s DAI registered\n", dai_drv.name);
    return 0;
    }
    static struct platform_driver xlnx_spdif_driver = {
    .driver = {
    .name = "xlnx-spdif",
    .of_match_table = xlnx_spdif_of_match,
    },
    .probe = xlnx_spdif_probe,
    };
    module_platform_driver(xlnx_spdif_driver);
    MODULE_AUTHOR("Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>");
    MODULE_DESCRIPTION("XILINX SPDIF driver");
    MODULE_LICENSE("GPL v2");
