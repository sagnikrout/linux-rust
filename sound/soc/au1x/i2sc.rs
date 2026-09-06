//! Automatically rewritten from C to Rust
//! Source: sound/soc/au1x/i2sc.c
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
// Au1000/Au1500/Au1100 I2S controller driver for ASoC
//
// (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
//
// Note: clock supplied to the I2S controller must be 256x samplerate.
//

pub const I2S_RXTX: c_uint = 0x00;
pub const I2S_CFG: c_uint = 0x04;
pub const I2S_ENABLE: c_uint = 0x08;

// only limited by clock generator and board design

    SNDRV_PCM_RATE_CONTINUOUS

    (SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 |		\
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE |	\
    SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_U16_BE |	\
    SNDRV_PCM_FMTBIT_S18_3LE | SNDRV_PCM_FMTBIT_U18_3LE |	\
    SNDRV_PCM_FMTBIT_S18_3BE | SNDRV_PCM_FMTBIT_U18_3BE |	\
    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_U20_3LE |	\
    SNDRV_PCM_FMTBIT_S20_3BE | SNDRV_PCM_FMTBIT_U20_3BE |	\
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_BE |	\
    SNDRV_PCM_FMTBIT_U24_LE | SNDRV_PCM_FMTBIT_U24_BE |	\
    0)
#[no_mangle]
pub unsafe extern "C" fn RD(ctx: *mut au1xpsc_audio_data, reg: c_int) -> c_ulong {
    static inline unsigned long RD(struct au1xpsc_audio_data *ctx, int reg)
    {
    return __raw_readl(ctx.mmio + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn WR(ctx: *mut au1xpsc_audio_data, reg: c_int, v: c_ulong) {
    static inline void WR(struct au1xpsc_audio_data *ctx, int reg, unsigned long v)
    {
    __raw_writel(v, ctx.mmio + reg);
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn au1xi2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int au1xi2s_set_fmt(struct snd_soc_dai *cpu_dai, unsigned int fmt)
    {
    struct au1xpsc_audio_data *ctx = snd_soc_dai_get_drvdata(cpu_dai);
    unsigned long c;
    int ret;
    ret = -EINVAL;
    c = ctx.cfg;
    c &= ~CFG_FM_MASK;
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    c |= CFG_FM_I2S;
    break;
    case SND_SOC_DAIFMT_MSB:
    c |= CFG_FM_RJ;
    break;
    case SND_SOC_DAIFMT_LSB:
    c |= CFG_FM_LJ;
    break;
    default:
    goto out;
    }
    c &= ~(CFG_IC | CFG_ICK);		/* IB-IF */
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    c |= CFG_IC | CFG_ICK;
    break;
    case SND_SOC_DAIFMT_NB_IF:
    c |= CFG_IC;
    break;
    case SND_SOC_DAIFMT_IB_NF:
    c |= CFG_ICK;
    break;
    case SND_SOC_DAIFMT_IB_IF:
    break;
    default:
    goto out;
    }
// I2S controller only supports provider
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BP_FP:	/* CODEC consumer */
    break;
    default:
    goto out;
    }
    ret = 0;
    ctx.cfg = c;
    out:
    return ret;
    }
    static int au1xi2s_trigger(struct snd_pcm_substream *substream,
    int cmd, struct snd_soc_dai *dai)
    {
    struct au1xpsc_audio_data *ctx = snd_soc_dai_get_drvdata(dai);
    let mut stype: c_int = SUBSTREAM_TYPE(substream);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
// power up
    WR(ctx, I2S_ENABLE, EN_D | EN_CE);
    WR(ctx, I2S_ENABLE, EN_CE);
    ctx.cfg |= (stype == PCM_TX) ? CFG_TN : CFG_RN;
    WR(ctx, I2S_CFG, ctx.cfg);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    ctx.cfg &= ~((stype == PCM_TX) ? CFG_TN : CFG_RN);
    WR(ctx, I2S_CFG, ctx.cfg);
    WR(ctx, I2S_ENABLE, EN_D);		/* power off */
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msbits_to_reg(msbits: c_int) -> c_ulong {
    static unsigned long msbits_to_reg(int msbits)
    {
    switch (msbits) {
    case 8:
    return CFG_SZ_8;
    case 16:
    return CFG_SZ_16;
    case 18:
    return CFG_SZ_18;
    case 20:
    return CFG_SZ_20;
    case 24:
    return CFG_SZ_24;
    }
    return 0;
    }
    static int au1xi2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct au1xpsc_audio_data *ctx = snd_soc_dai_get_drvdata(dai);
    unsigned long v;
    v = msbits_to_reg(params.msbits);
    if (!v)
    return -EINVAL;
    ctx.cfg &= ~CFG_SZ_MASK;
    ctx.cfg |= v;
    return 0;
    }
    static int au1xi2s_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct au1xpsc_audio_data *ctx = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_set_dma_data(dai, substream, &ctx.dmaids[0]);
    return 0;
    }
    static const u64 au1xi2s_selectable_formats =
    SND_SOC_POSSIBLE_DAIFMT_I2S	|
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J	|
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
    SND_SOC_POSSIBLE_DAIFMT_NB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_NB_IF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_NF	|
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;
    static const struct snd_soc_dai_ops au1xi2s_dai_ops = {
    .startup	= au1xi2s_startup,
    .trigger	= au1xi2s_trigger,
    .hw_params	= au1xi2s_hw_params,
    .set_fmt	= au1xi2s_set_fmt,
    .auto_selectable_formats	= &au1xi2s_selectable_formats,
    .num_auto_selectable_formats	= 1,
    };
    static struct snd_soc_dai_driver au1xi2s_dai_driver = {
    .symmetric_rate		= 1,
    .playback = {
    .rates		= AU1XI2SC_RATES,
    .formats	= AU1XI2SC_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .capture = {
    .rates		= AU1XI2SC_RATES,
    .formats	= AU1XI2SC_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .ops = &au1xi2s_dai_ops,
    };
    static const struct snd_soc_component_driver au1xi2s_component = {
    .name			= "au1xi2s",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn au1xi2s_drvprobe(pdev: *mut platform_device) -> c_int {
    static int au1xi2s_drvprobe(struct platform_device *pdev)
    {
    struct resource *iores, *dmares;
    struct au1xpsc_audio_data *ctx;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    iores = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!iores)
    return -ENODEV;
    if (!devm_request_mem_region(&pdev.dev, iores.start,
    resource_size(iores),
    pdev.name))
    return -EBUSY;
    ctx.mmio = devm_ioremap(&pdev.dev, iores.start,
    resource_size(iores));
    if (!ctx.mmio)
    return -EBUSY;
    dmares = platform_get_resource(pdev, IORESOURCE_DMA, 0);
    if (!dmares)
    return -EBUSY;
    ctx.dmaids[SNDRV_PCM_STREAM_PLAYBACK] = dmares.start;
    dmares = platform_get_resource(pdev, IORESOURCE_DMA, 1);
    if (!dmares)
    return -EBUSY;
    ctx.dmaids[SNDRV_PCM_STREAM_CAPTURE] = dmares.start;
    platform_set_drvdata(pdev, ctx);
    return snd_soc_register_component(&pdev.dev, &au1xi2s_component,
    &au1xi2s_dai_driver, 1);
    }
#[no_mangle]
unsafe extern "C" fn au1xi2s_drvremove(pdev: *mut platform_device) {
    static void au1xi2s_drvremove(struct platform_device *pdev)
    {
    struct au1xpsc_audio_data *ctx = platform_get_drvdata(pdev);
    snd_soc_unregister_component(&pdev.dev);
    WR(ctx, I2S_ENABLE, EN_D);	/* clock off, disable */
    }
#[no_mangle]
unsafe extern "C" fn au1xi2s_drvsuspend(dev: *mut device) -> c_int {
    static int au1xi2s_drvsuspend(struct device *dev)
    {
    struct au1xpsc_audio_data *ctx = dev_get_drvdata(dev);
    WR(ctx, I2S_ENABLE, EN_D);	/* clock off, disable */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1xi2s_drvresume(dev: *mut device) -> c_int {
    static int au1xi2s_drvresume(struct device *dev)
    {
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(au1xi2sc_pmops, au1xi2s_drvsuspend,
    au1xi2s_drvresume);
    static struct platform_driver au1xi2s_driver = {
    .driver	= {
    .name	= "alchemy-i2sc",
    .pm	= pm_ptr(&au1xi2sc_pmops),
    },
    .probe		= au1xi2s_drvprobe,
    .remove		= au1xi2s_drvremove,
    };
    module_platform_driver(au1xi2s_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Au1000/1500/1100 I2S ASoC driver");
    MODULE_AUTHOR("Manuel Lauss");
