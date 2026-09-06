//! Automatically rewritten from C to Rust
//! Source: sound/soc/au1x/ac97c.c
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
// Au1000/Au1500/Au1100 AC97C controller driver for ASoC
//
// (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
//
// based on the old ALSA driver originally written by
// Charles Eidsness <charles@cooper-street.com>
//

// register offsets and bits
pub const AC97_CONFIG: c_uint = 0x00;
pub const AC97_STATUS: c_uint = 0x04;
pub const AC97_DATA: c_uint = 0x08;
pub const AC97_CMDRESP: c_uint = 0x0c;
pub const AC97_ENABLE: c_uint = 0x10;

// how often to retry failed codec register reads/writes
pub const AC97_RW_RETRIES: c_int = 5;

    SNDRV_PCM_RATE_CONTINUOUS

    (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE)
// instance data. There can be only one, MacLeod!!!!, fortunately there IS only
// once AC97C on early Alchemy chips. The newer ones aren't so lucky.
//
    static struct au1xpsc_audio_data *ac97c_workdata;

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
    static unsigned short au1xac97c_ac97_read(struct snd_ac97 *ac97,
    unsigned short r)
    {
    struct au1xpsc_audio_data *ctx = ac97_to_ctx(ac97);
    unsigned int tmo, retry;
    unsigned long data;
    data = ~0;
    retry = AC97_RW_RETRIES;
    do {
    mutex_lock(&ctx.lock);
    tmo = 6;
    while ((RD(ctx, AC97_STATUS) & STAT_CP) && --tmo)
    udelay(21);	/* wait an ac97 frame time */
    if (!tmo) {
    pr_debug("ac97rd timeout #1\n");
    goto next;
    }
    WR(ctx, AC97_CMDRESP, CMD_IDX(r) | CMD_READ);
// stupid errata: data is only valid for 21us, so
// poll, Forrest, poll...
//
    tmo = 0x10000;
    while ((RD(ctx, AC97_STATUS) & STAT_CP) && --tmo)
    asm volatile ("nop");
    data = RD(ctx, AC97_CMDRESP);
    if (!tmo)
    pr_debug("ac97rd timeout #2\n");
    next:
    mutex_unlock(&ctx.lock);
    } while (--retry && !tmo);
    pr_debug("AC97RD %04x %04lx %d\n", r, data, retry);
    return retry ? data & 0xffff : 0xffff;
    }
    static void au1xac97c_ac97_write(struct snd_ac97 *ac97, unsigned short r,
    unsigned short v)
    {
    struct au1xpsc_audio_data *ctx = ac97_to_ctx(ac97);
    unsigned int tmo, retry;
    retry = AC97_RW_RETRIES;
    do {
    mutex_lock(&ctx.lock);
    for (tmo = 5; (RD(ctx, AC97_STATUS) & STAT_CP) && tmo; tmo--)
    udelay(21);
    if (!tmo) {
    pr_debug("ac97wr timeout #1\n");
    goto next;
    }
    WR(ctx, AC97_CMDRESP, CMD_WRITE | CMD_IDX(r) | CMD_SET_DATA(v));
    for (tmo = 10; (RD(ctx, AC97_STATUS) & STAT_CP) && tmo; tmo--)
    udelay(21);
    if (!tmo)
    pr_debug("ac97wr timeout #2\n");
    next:
    mutex_unlock(&ctx.lock);
    } while (--retry && !tmo);
    pr_debug("AC97WR %04x %04x %d\n", r, v, retry);
    }
#[no_mangle]
unsafe extern "C" fn au1xac97c_ac97_warm_reset(ac97: *mut snd_ac97) {
    static void au1xac97c_ac97_warm_reset(struct snd_ac97 *ac97)
    {
    struct au1xpsc_audio_data *ctx = ac97_to_ctx(ac97);
    WR(ctx, AC97_CONFIG, ctx.cfg | CFG_SG | CFG_SN);
    msleep(20);
    WR(ctx, AC97_CONFIG, ctx.cfg | CFG_SG);
    WR(ctx, AC97_CONFIG, ctx.cfg);
    }
#[no_mangle]
unsafe extern "C" fn au1xac97c_ac97_cold_reset(ac97: *mut snd_ac97) {
    static void au1xac97c_ac97_cold_reset(struct snd_ac97 *ac97)
    {
    struct au1xpsc_audio_data *ctx = ac97_to_ctx(ac97);
    int i;
    WR(ctx, AC97_CONFIG, ctx.cfg | CFG_RS);
    msleep(500);
    WR(ctx, AC97_CONFIG, ctx.cfg);
// wait for codec ready
    i = 50;
    while (((RD(ctx, AC97_STATUS) & STAT_RD) == 0) && --i)
    msleep(20);
    if (!i)
    printk(KERN_ERR "ac97c: codec not ready after cold reset\n");
    }
// AC97 controller operations
    static struct snd_ac97_bus_ops ac97c_bus_ops = {
    .read		= au1xac97c_ac97_read,
    .write		= au1xac97c_ac97_write,
    .reset		= au1xac97c_ac97_cold_reset,
    .warm_reset	= au1xac97c_ac97_warm_reset,
    };
    static int alchemy_ac97c_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct au1xpsc_audio_data *ctx = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_set_dma_data(dai, substream, &ctx.dmaids[0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1xac97c_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int au1xac97c_dai_probe(struct snd_soc_dai *dai)
    {
    return ac97c_workdata ? 0 : -ENODEV;
    }
    static const struct snd_soc_dai_ops alchemy_ac97c_ops = {
    .probe			= au1xac97c_dai_probe,
    .startup		= alchemy_ac97c_startup,
    };
    static struct snd_soc_dai_driver au1xac97c_dai_driver = {
    .name			= "alchemy-ac97c",
    .playback = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .capture = {
    .rates		= AC97_RATES,
    .formats	= AC97_FMTS,
    .channels_min	= 2,
    .channels_max	= 2,
    },
    .ops			= &alchemy_ac97c_ops,
    };
    static const struct snd_soc_component_driver au1xac97c_component = {
    .name			= "au1xac97c",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn au1xac97c_drvprobe(pdev: *mut platform_device) -> c_int {
    static int au1xac97c_drvprobe(struct platform_device *pdev)
    {
    int ret;
    struct resource *iores, *dmares;
    struct au1xpsc_audio_data *ctx;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    mutex_init(&ctx.lock);
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
// switch it on
    WR(ctx, AC97_ENABLE, EN_D | EN_CE);
    WR(ctx, AC97_ENABLE, EN_CE);
    ctx.cfg = CFG_RC(3) | CFG_XS(3);
    WR(ctx, AC97_CONFIG, ctx.cfg);
    platform_set_drvdata(pdev, ctx);
    ret = snd_soc_set_ac97_ops(&ac97c_bus_ops);
    if (ret)
    return ret;
    ret = snd_soc_register_component(&pdev.dev, &au1xac97c_component,
    &au1xac97c_dai_driver, 1);
    if (ret)
    return ret;
    ac97c_workdata = ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1xac97c_drvremove(pdev: *mut platform_device) {
    static void au1xac97c_drvremove(struct platform_device *pdev)
    {
    struct au1xpsc_audio_data *ctx = platform_get_drvdata(pdev);
    snd_soc_unregister_component(&pdev.dev);
    WR(ctx, AC97_ENABLE, EN_D);	/* clock off, disable */
    ac97c_workdata = core::ptr::null_mut();	/* MDEV */
    }

#[no_mangle]
unsafe extern "C" fn au1xac97c_drvsuspend(dev: *mut device) -> c_int {
    static int au1xac97c_drvsuspend(struct device *dev)
    {
    struct au1xpsc_audio_data *ctx = dev_get_drvdata(dev);
    WR(ctx, AC97_ENABLE, EN_D);	/* clock off, disable */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn au1xac97c_drvresume(dev: *mut device) -> c_int {
    static int au1xac97c_drvresume(struct device *dev)
    {
    struct au1xpsc_audio_data *ctx = dev_get_drvdata(dev);
    WR(ctx, AC97_ENABLE, EN_D | EN_CE);
    WR(ctx, AC97_ENABLE, EN_CE);
    WR(ctx, AC97_CONFIG, ctx.cfg);
    return 0;
    }
    static const struct dev_pm_ops au1xpscac97_pmops = {
    .suspend	= au1xac97c_drvsuspend,
    .resume		= au1xac97c_drvresume,
    };

    static struct platform_driver au1xac97c_driver = {
    .driver	= {
    .name	= "alchemy-ac97c",
    .pm	= AU1XPSCAC97_PMOPS,
    },
    .probe		= au1xac97c_drvprobe,
    .remove		= au1xac97c_drvremove,
    };
    module_platform_driver(au1xac97c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Au1000/1500/1100 AC97C ASoC driver");
    MODULE_AUTHOR("Manuel Lauss");
