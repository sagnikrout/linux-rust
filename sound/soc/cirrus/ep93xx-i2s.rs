//! Automatically rewritten from C to Rust
//! Source: sound/soc/cirrus/ep93xx-i2s.c
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
// linux/sound/soc/ep93xx-i2s.c
// EP93xx I2S driver
//
// Copyright (C) 2010 Ryan Mallon
//
// Based on the original driver by:
// Copyright (C) 2007 Chase Douglas <chasedouglas@gmail>
// Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
//

pub const EP93XX_I2S_TXCLKCFG: c_uint = 0x00;
pub const EP93XX_I2S_RXCLKCFG: c_uint = 0x04;
pub const EP93XX_I2S_GLSTS: c_uint = 0x08;
pub const EP93XX_I2S_GLCTRL: c_uint = 0x0C;
pub const EP93XX_I2S_I2STX0LFT: c_uint = 0x10;
pub const EP93XX_I2S_I2STX0RT: c_uint = 0x14;
pub const EP93XX_I2S_TXLINCTRLDATA: c_uint = 0x28;
pub const EP93XX_I2S_TXCTRL: c_uint = 0x2C;
pub const EP93XX_I2S_TXWRDLEN: c_uint = 0x30;
pub const EP93XX_I2S_TX0EN: c_uint = 0x34;
pub const EP93XX_I2S_RXLINCTRLDATA: c_uint = 0x58;
pub const EP93XX_I2S_RXCTRL: c_uint = 0x5C;
pub const EP93XX_I2S_RXWRDLEN: c_uint = 0x60;
pub const EP93XX_I2S_RX0EN: c_uint = 0x64;

//
// Transmit empty interrupt level select:
// 0 - Generate interrupt when FIFO is half empty
// 1 - Generate interrupt when FIFO is empty
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_i2s_info {
    pub mclk: *mut clk,
    pub sclk: *mut clk,
    pub lrclk: *mut clk,
    pub regs: *mut void __iomem,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
}

    static inline void ep93xx_i2s_write_reg(struct ep93xx_i2s_info *info,
    unsigned reg, unsigned val)
    {
    __raw_writel(val, info.regs + reg);
    }
    static inline unsigned ep93xx_i2s_read_reg(struct ep93xx_i2s_info *info,
    unsigned reg)
    {
    return __raw_readl(info.regs + reg);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_enable(info: *mut ep93xx_i2s_info, stream: c_int) -> c_int {
    static int ep93xx_i2s_enable(struct ep93xx_i2s_info *info, int stream)
    {
    unsigned base_reg;
    int err;
    if ((ep93xx_i2s_read_reg(info, EP93XX_I2S_TX0EN) & 0x1) == 0 &&
    (ep93xx_i2s_read_reg(info, EP93XX_I2S_RX0EN) & 0x1) == 0) {
// Enable clocks
    err = clk_prepare_enable(info.mclk);
    if (err)
    return err;
    err = clk_prepare_enable(info.sclk);
    if (err) {
    clk_disable_unprepare(info.mclk);
    return err;
    }
    err = clk_prepare_enable(info.lrclk);
    if (err) {
    clk_disable_unprepare(info.sclk);
    clk_disable_unprepare(info.mclk);
    return err;
    }
// Enable i2s
    ep93xx_i2s_write_reg(info, EP93XX_I2S_GLCTRL, 1);
    }
// Enable fifo
    if (stream == SNDRV_PCM_STREAM_PLAYBACK)
    base_reg = EP93XX_I2S_TX0EN;
    else
    base_reg = EP93XX_I2S_RX0EN;
    ep93xx_i2s_write_reg(info, base_reg, 1);
// Enable TX IRQs (FIFO empty or underflow)
    if (IS_ENABLED(CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG) &&
    stream == SNDRV_PCM_STREAM_PLAYBACK)
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXCTRL,
    EP93XX_I2S_TXCTRL_TXEMPTY_LVL |
    EP93XX_I2S_TXCTRL_TXUFIE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_disable(info: *mut ep93xx_i2s_info, stream: c_int) {
    static void ep93xx_i2s_disable(struct ep93xx_i2s_info *info, int stream)
    {
    unsigned base_reg;
// Disable IRQs
    if (IS_ENABLED(CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG) &&
    stream == SNDRV_PCM_STREAM_PLAYBACK)
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXCTRL, 0);
// Disable fifo
    if (stream == SNDRV_PCM_STREAM_PLAYBACK)
    base_reg = EP93XX_I2S_TX0EN;
    else
    base_reg = EP93XX_I2S_RX0EN;
    ep93xx_i2s_write_reg(info, base_reg, 0);
    if ((ep93xx_i2s_read_reg(info, EP93XX_I2S_TX0EN) & 0x1) == 0 &&
    (ep93xx_i2s_read_reg(info, EP93XX_I2S_RX0EN) & 0x1) == 0) {
// Disable i2s
    ep93xx_i2s_write_reg(info, EP93XX_I2S_GLCTRL, 0);
// Disable clocks
    clk_disable_unprepare(info.lrclk);
    clk_disable_unprepare(info.sclk);
    clk_disable_unprepare(info.mclk);
    }
    }
//
// According to documentation I2S controller can handle underflow conditions
// just fine, but in reality the state machine is sometimes confused so that
// the whole stream is shifted by one byte. The watchdog below disables the TX
// FIFO, fills the buffer with zeroes and re-enables the FIFO. State machine
// is being reset and by filling the buffer we get some time before next
// underflow happens.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ep93xx_i2s_interrupt(int irq, void *dev_id)
    {
    struct ep93xx_i2s_info *info = dev_id;
// Disable FIFO
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TX0EN, 0);
//
// Fill TX FIFO with zeroes, this way we can defer next IRQs as much as
// possible and get more time for DMA to catch up. Actually there are
// only 8 samples in this FIFO, so even on 8kHz maximum deferral here is
// 1ms.
//
    while (!(ep93xx_i2s_read_reg(info, EP93XX_I2S_GLSTS) &
    EP93XX_I2S_GLSTS_TX0_FIFO_FULL)) {
    ep93xx_i2s_write_reg(info, EP93XX_I2S_I2STX0LFT, 0);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_I2STX0RT, 0);
    }
// Re-enable FIFO
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TX0EN, 1);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int ep93xx_i2s_dai_probe(struct snd_soc_dai *dai)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai,	&info.dma_params_tx,
    &info.dma_params_rx);
    return 0;
    }
    static int ep93xx_i2s_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(dai);
    return ep93xx_i2s_enable(info, substream.stream);
    }
    static void ep93xx_i2s_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(dai);
    ep93xx_i2s_disable(info, substream.stream);
    }
    static int ep93xx_i2s_set_dai_fmt(struct snd_soc_dai *cpu_dai,
    unsigned int fmt)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(cpu_dai);
    unsigned int clk_cfg;
    let mut txlin_ctrl: c_uint = 0;
    let mut rxlin_ctrl: c_uint = 0;
    clk_cfg  = ep93xx_i2s_read_reg(info, EP93XX_I2S_RXCLKCFG);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    clk_cfg |= EP93XX_I2S_CLKCFG_REL;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    clk_cfg &= ~EP93XX_I2S_CLKCFG_REL;
    break;
    case SND_SOC_DAIFMT_RIGHT_J:
    clk_cfg &= ~EP93XX_I2S_CLKCFG_REL;
    rxlin_ctrl |= EP93XX_I2S_RXLINCTRLDATA_R_JUST;
    txlin_ctrl |= EP93XX_I2S_TXLINCTRLDATA_R_JUST;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BP_FP:
// CPU is provider
    clk_cfg |= EP93XX_I2S_CLKCFG_MASTER;
    break;
    case SND_SOC_DAIFMT_BC_FC:
// Codec is provider
    clk_cfg &= ~EP93XX_I2S_CLKCFG_MASTER;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
// Negative bit clock, lrclk low on left word
    clk_cfg &= ~(EP93XX_I2S_CLKCFG_CKP | EP93XX_I2S_CLKCFG_LRS);
    break;
    case SND_SOC_DAIFMT_NB_IF:
// Negative bit clock, lrclk low on right word
    clk_cfg &= ~EP93XX_I2S_CLKCFG_CKP;
    clk_cfg |= EP93XX_I2S_CLKCFG_LRS;
    break;
    case SND_SOC_DAIFMT_IB_NF:
// Positive bit clock, lrclk low on left word
    clk_cfg |= EP93XX_I2S_CLKCFG_CKP;
    clk_cfg &= ~EP93XX_I2S_CLKCFG_LRS;
    break;
    case SND_SOC_DAIFMT_IB_IF:
// Positive bit clock, lrclk low on right word
    clk_cfg |= EP93XX_I2S_CLKCFG_CKP | EP93XX_I2S_CLKCFG_LRS;
    break;
    }
// Write new register values
    ep93xx_i2s_write_reg(info, EP93XX_I2S_RXCLKCFG, clk_cfg);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXCLKCFG, clk_cfg);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_RXLINCTRLDATA, rxlin_ctrl);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXLINCTRLDATA, txlin_ctrl);
    return 0;
    }
    static int ep93xx_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(dai);
    unsigned word_len, div, sdiv, lrdiv;
    int err;
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    word_len = EP93XX_I2S_WRDLEN_16;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    word_len = EP93XX_I2S_WRDLEN_24;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    word_len = EP93XX_I2S_WRDLEN_32;
    break;
    default:
    return -EINVAL;
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXWRDLEN, word_len);
    else
    ep93xx_i2s_write_reg(info, EP93XX_I2S_RXWRDLEN, word_len);
//
// EP93xx I2S module can be setup so SCLK / LRCLK value can be
// 32, 64, 128. MCLK / SCLK value can be 2 and 4.
// We set LRCLK equal to `rate' and minimum SCLK / LRCLK
// value is 64, because our sample size is 32 bit * 2 channels.
// I2S standard permits us to transmit more bits than
// the codec uses.
//
    div = clk_get_rate(info.mclk) / params_rate(params);
    sdiv = 4;
    if (div > (256 + 512) / 2) {
    lrdiv = 128;
    } else {
    lrdiv = 64;
    if (div < (128 + 256) / 2)
    sdiv = 2;
    }
    err = clk_set_rate(info.sclk, clk_get_rate(info.mclk) / sdiv);
    if (err)
    return err;
    err = clk_set_rate(info.lrclk, clk_get_rate(info.sclk) / lrdiv);
    if (err)
    return err;
    return 0;
    }
    static int ep93xx_i2s_set_sysclk(struct snd_soc_dai *cpu_dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct ep93xx_i2s_info *info = snd_soc_dai_get_drvdata(cpu_dai);
    if (dir == SND_SOC_CLOCK_IN || clk_id != 0)
    return -EINVAL;
    if (!freq)
    return 0;
    return clk_set_rate(info.mclk, freq);
    }

#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_suspend(component: *mut snd_soc_component) -> c_int {
    static int ep93xx_i2s_suspend(struct snd_soc_component *component)
    {
    struct ep93xx_i2s_info *info = snd_soc_component_get_drvdata(component);
    if (!snd_soc_component_active(component))
    return 0;
    ep93xx_i2s_disable(info, SNDRV_PCM_STREAM_PLAYBACK);
    ep93xx_i2s_disable(info, SNDRV_PCM_STREAM_CAPTURE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_resume(component: *mut snd_soc_component) -> c_int {
    static int ep93xx_i2s_resume(struct snd_soc_component *component)
    {
    struct ep93xx_i2s_info *info = snd_soc_component_get_drvdata(component);
    int err;
    if (!snd_soc_component_active(component))
    return 0;
    err = ep93xx_i2s_enable(info, SNDRV_PCM_STREAM_PLAYBACK);
    if (err)
    return err;
    return ep93xx_i2s_enable(info, SNDRV_PCM_STREAM_CAPTURE);
    }

    static const u64 ep93xx_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_NB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_NB_IF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;
    static const struct snd_soc_dai_ops ep93xx_i2s_dai_ops = {
    .probe		= ep93xx_i2s_dai_probe,
    .startup	= ep93xx_i2s_startup,
    .shutdown	= ep93xx_i2s_shutdown,
    .hw_params	= ep93xx_i2s_hw_params,
    .set_sysclk	= ep93xx_i2s_set_sysclk,
    .set_fmt	= ep93xx_i2s_set_dai_fmt,
    .auto_selectable_formats	= &ep93xx_selectable_formats,
    .num_auto_selectable_formats	= 1,
    };

    static struct snd_soc_dai_driver ep93xx_i2s_dai = {
    .symmetric_rate	= 1,
    .playback	= {
    .channels_min	= 2,
    .channels_max	= 2,
    .rates		= SNDRV_PCM_RATE_8000_192000,
    .formats	= EP93XX_I2S_FORMATS,
    },
    .capture	= {
    .channels_min	= 2,
    .channels_max	= 2,
    .rates		= SNDRV_PCM_RATE_8000_192000,
    .formats	= EP93XX_I2S_FORMATS,
    },
    .ops		= &ep93xx_i2s_dai_ops,
    };
    static const struct snd_soc_component_driver ep93xx_i2s_component = {
    .name			= "ep93xx-i2s",
    .suspend		= ep93xx_i2s_suspend,
    .resume			= ep93xx_i2s_resume,
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xx_i2s_probe(struct platform_device *pdev)
    {
    struct ep93xx_i2s_info *info;
    int err;
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(info.regs))
    return PTR_ERR(info.regs);
    if (IS_ENABLED(CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG)) {
    let mut irq: c_int = platform_get_irq(pdev, 0);
    if (irq <= 0)
    return irq < 0 ? irq : -ENODEV;
    err = devm_request_irq(&pdev.dev, irq, ep93xx_i2s_interrupt, 0,
    pdev.name, info);
    if (err)
    return err;
    }
    info.mclk = clk_get(&pdev.dev, "mclk");
    if (IS_ERR(info.mclk)) {
    err = PTR_ERR(info.mclk);
    goto fail;
    }
    info.sclk = clk_get(&pdev.dev, "sclk");
    if (IS_ERR(info.sclk)) {
    err = PTR_ERR(info.sclk);
    goto fail_put_mclk;
    }
    info.lrclk = clk_get(&pdev.dev, "lrclk");
    if (IS_ERR(info.lrclk)) {
    err = PTR_ERR(info.lrclk);
    goto fail_put_sclk;
    }
    dev_set_drvdata(&pdev.dev, info);
    err = devm_snd_soc_register_component(&pdev.dev, &ep93xx_i2s_component,
    &ep93xx_i2s_dai, 1);
    if (err)
    goto fail_put_lrclk;
    err = devm_ep93xx_pcm_platform_register(&pdev.dev);
    if (err)
    goto fail_put_lrclk;
    return 0;
    fail_put_lrclk:
    clk_put(info.lrclk);
    fail_put_sclk:
    clk_put(info.sclk);
    fail_put_mclk:
    clk_put(info.mclk);
    fail:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_i2s_remove(pdev: *mut platform_device) {
    static void ep93xx_i2s_remove(struct platform_device *pdev)
    {
    struct ep93xx_i2s_info *info = dev_get_drvdata(&pdev.dev);
    clk_put(info.lrclk);
    clk_put(info.sclk);
    clk_put(info.mclk);
    }
    static const struct of_device_id ep93xx_i2s_of_ids[] = {
    { .compatible = "cirrus,ep9301-i2s" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ep93xx_i2s_of_ids);
    static struct platform_driver ep93xx_i2s_driver = {
    .probe	= ep93xx_i2s_probe,
    .remove = ep93xx_i2s_remove,
    .driver	= {
    .name	= "ep93xx-i2s",
    .of_match_table = ep93xx_i2s_of_ids,
    },
    };
    module_platform_driver(ep93xx_i2s_driver);
    MODULE_ALIAS("platform:ep93xx-i2s");
    MODULE_AUTHOR("Ryan Mallon");
    MODULE_DESCRIPTION("EP93XX I2S driver");
    MODULE_LICENSE("GPL");
