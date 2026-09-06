//! Automatically rewritten from C to Rust
//! Source: sound/soc/img/img-spdif-out.c
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
// IMG SPDIF output controller driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

pub const IMG_SPDIF_OUT_TX_FIFO: c_uint = 0x0;
pub const IMG_SPDIF_OUT_CTL: c_uint = 0x4;

pub const IMG_SPDIF_OUT_CSL: c_uint = 0x14;
pub const IMG_SPDIF_OUT_CSH_UV: c_uint = 0x18;
pub const IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT: c_int = 0;
pub const IMG_SPDIF_OUT_CSH_UV_CSH_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_spdif_out {
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
    pub clk_sys: *mut clk,
    pub clk_ref: *mut clk,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub dev: *mut device,
    pub rst: *mut reset_control,
    pub suspend_ctl: u32,
    pub suspend_csl: u32,
    pub suspend_csh: u32,
}

#[no_mangle]
unsafe extern "C" fn img_spdif_out_runtime_suspend(dev: *mut device) -> c_int {
    static int img_spdif_out_runtime_suspend(struct device *dev)
    {
    struct img_spdif_out *spdif = dev_get_drvdata(dev);
    clk_disable_unprepare(spdif.clk_ref);
    clk_disable_unprepare(spdif.clk_sys);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_runtime_resume(dev: *mut device) -> c_int {
    static int img_spdif_out_runtime_resume(struct device *dev)
    {
    struct img_spdif_out *spdif = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(spdif.clk_sys);
    if (ret) {
    dev_err(dev, "clk_enable failed: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(spdif.clk_ref);
    if (ret) {
    dev_err(dev, "clk_enable failed: %d\n", ret);
    clk_disable_unprepare(spdif.clk_sys);
    return ret;
    }
    return 0;
    }
    static inline void img_spdif_out_writel(struct img_spdif_out *spdif, u32 val,
    u32 reg)
    {
    writel(val, spdif.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn img_spdif_out_readl(spdif: *mut img_spdif_out, reg: u32) -> u32 {
    static inline u32 img_spdif_out_readl(struct img_spdif_out *spdif, u32 reg)
    {
    return readl(spdif.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_reset(spdif: *mut img_spdif_out) {
    static void img_spdif_out_reset(struct img_spdif_out *spdif)
    {
    u32 ctl, status_low, status_high;
    ctl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL) &
    ~IMG_SPDIF_OUT_CTL_SRT_MASK;
    status_low = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    status_high = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    reset_control_assert(spdif.rst);
    reset_control_deassert(spdif.rst);
    img_spdif_out_writel(spdif, ctl, IMG_SPDIF_OUT_CTL);
    img_spdif_out_writel(spdif, status_low, IMG_SPDIF_OUT_CSL);
    img_spdif_out_writel(spdif, status_high, IMG_SPDIF_OUT_CSH_UV);
    }
    static int img_spdif_out_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int img_spdif_out_get_status_mask(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    ucontrol.value.iec958.status[0] = 0xff;
    ucontrol.value.iec958.status[1] = 0xff;
    ucontrol.value.iec958.status[2] = 0xff;
    ucontrol.value.iec958.status[3] = 0xff;
    ucontrol.value.iec958.status[4] = 0xff;
    return 0;
    }
    static int img_spdif_out_get_status(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct img_spdif_out *spdif = snd_soc_dai_get_drvdata(cpu_dai);
    u32 reg;
    guard(spinlock_irqsave)(&spdif.lock);
    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    ucontrol.value.iec958.status[0] = reg & 0xff;
    ucontrol.value.iec958.status[1] = (reg >> 8) & 0xff;
    ucontrol.value.iec958.status[2] = (reg >> 16) & 0xff;
    ucontrol.value.iec958.status[3] = (reg >> 24) & 0xff;
    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    ucontrol.value.iec958.status[4] =
    (reg & IMG_SPDIF_OUT_CSH_UV_CSH_MASK) >>
    IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT;
    return 0;
    }
    static int img_spdif_out_set_status(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct img_spdif_out *spdif = snd_soc_dai_get_drvdata(cpu_dai);
    u32 reg;
    reg = ((u32)ucontrol.value.iec958.status[3] << 24);
    reg |= ((u32)ucontrol.value.iec958.status[2] << 16);
    reg |= ((u32)ucontrol.value.iec958.status[1] << 8);
    reg |= (u32)ucontrol.value.iec958.status[0];
    guard(spinlock_irqsave)(&spdif.lock);
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CSL);
    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    reg &= ~IMG_SPDIF_OUT_CSH_UV_CSH_MASK;
    reg |= (u32)ucontrol.value.iec958.status[4] <<
    IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT;
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CSH_UV);
    return 0;
    }
    static struct snd_kcontrol_new img_spdif_out_controls[] = {
    {
    .access = SNDRV_CTL_ELEM_ACCESS_READ,
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = SNDRV_CTL_NAME_IEC958("", PLAYBACK, MASK),
    .info = img_spdif_out_info,
    .get = img_spdif_out_get_status_mask
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = SNDRV_CTL_NAME_IEC958("", PLAYBACK, DEFAULT),
    .info = img_spdif_out_info,
    .get = img_spdif_out_get_status,
    .put = img_spdif_out_set_status
    }
    };
    static int img_spdif_out_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct img_spdif_out *spdif = snd_soc_dai_get_drvdata(dai);
    u32 reg;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
    reg |= IMG_SPDIF_OUT_CTL_SRT_MASK;
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CTL);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    scoped_guard(spinlock_irqsave, &spdif.lock)
    img_spdif_out_reset(spdif);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int img_spdif_out_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct img_spdif_out *spdif = snd_soc_dai_get_drvdata(dai);
    unsigned int channels;
    long pre_div_a, pre_div_b, diff_a, diff_b, rate, clk_rate;
    u32 reg;
    snd_pcm_format_t format;
    rate = params_rate(params);
    format = params_format(params);
    channels = params_channels(params);
    dev_dbg(spdif.dev, "hw_params rate %ld channels %u format %u\n",
    rate, channels, format);
    if (format != SNDRV_PCM_FORMAT_S32_LE)
    return -EINVAL;
    if (channels != 2)
    return -EINVAL;
    pre_div_a = clk_round_rate(spdif.clk_ref, rate * 256);
    if (pre_div_a < 0)
    return pre_div_a;
    pre_div_b = clk_round_rate(spdif.clk_ref, rate * 384);
    if (pre_div_b < 0)
    return pre_div_b;
    diff_a = abs((pre_div_a / 256) - rate);
    diff_b = abs((pre_div_b / 384) - rate);
// If diffs are equal, use lower clock rate
    if (diff_a > diff_b)
    clk_set_rate(spdif.clk_ref, pre_div_b);
    else
    clk_set_rate(spdif.clk_ref, pre_div_a);
//
// Another driver (eg machine driver) may have rejected the above
// change. Get the current rate and set the register bit according to
// the new min diff
//
    clk_rate = clk_get_rate(spdif.clk_ref);
    diff_a = abs((clk_rate / 256) - rate);
    diff_b = abs((clk_rate / 384) - rate);
    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
    if (diff_a <= diff_b)
    reg &= ~IMG_SPDIF_OUT_CTL_CLK_MASK;
    else
    reg |= IMG_SPDIF_OUT_CTL_CLK_MASK;
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int img_spdif_out_dai_probe(struct snd_soc_dai *dai)
    {
    struct img_spdif_out *spdif = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &spdif.dma_data, core::ptr::null_mut());
    snd_soc_add_dai_controls(dai, img_spdif_out_controls,
    ARRAY_SIZE(img_spdif_out_controls));
    return 0;
    }
    static const struct snd_soc_dai_ops img_spdif_out_dai_ops = {
    .probe		= img_spdif_out_dai_probe,
    .trigger	= img_spdif_out_trigger,
    .hw_params	= img_spdif_out_hw_params
    };
    static struct snd_soc_dai_driver img_spdif_out_dai = {
    .playback = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE
    },
    .ops = &img_spdif_out_dai_ops
    };
    static const struct snd_soc_component_driver img_spdif_out_component = {
    .name = "img-spdif-out",
    .legacy_dai_naming = 1,
    };
#[no_mangle]
unsafe extern "C" fn img_spdif_out_probe(pdev: *mut platform_device) -> c_int {
    static int img_spdif_out_probe(struct platform_device *pdev)
    {
    struct img_spdif_out *spdif;
    struct resource *res;
    void __iomem *base;
    int ret;
    struct device *dev = &pdev.dev;
    spdif = devm_kzalloc(&pdev.dev, sizeof(*spdif), GFP_KERNEL);
    if (!spdif)
    return -ENOMEM;
    platform_set_drvdata(pdev, spdif);
    spdif.dev = &pdev.dev;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    spdif.base = base;
    spdif.rst = devm_reset_control_get_exclusive(&pdev.dev, "rst");
    if (IS_ERR(spdif.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(spdif.rst),
    "No top level reset found\n");
    spdif.clk_sys = devm_clk_get(&pdev.dev, "sys");
    if (IS_ERR(spdif.clk_sys))
    return dev_err_probe(dev, PTR_ERR(spdif.clk_sys),
    "Failed to acquire clock 'sys'\n");
    spdif.clk_ref = devm_clk_get(&pdev.dev, "ref");
    if (IS_ERR(spdif.clk_ref))
    return dev_err_probe(dev, PTR_ERR(spdif.clk_ref),
    "Failed to acquire clock 'ref'\n");
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = img_spdif_out_runtime_resume(&pdev.dev);
    if (ret)
    goto err_pm_disable;
    }
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret < 0)
    goto err_suspend;
    img_spdif_out_writel(spdif, IMG_SPDIF_OUT_CTL_FS_MASK,
    IMG_SPDIF_OUT_CTL);
    img_spdif_out_reset(spdif);
    pm_runtime_put(&pdev.dev);
    spin_lock_init(&spdif.lock);
    spdif.dma_data.addr = res.start + IMG_SPDIF_OUT_TX_FIFO;
    spdif.dma_data.addr_width = 4;
    spdif.dma_data.maxburst = 4;
    ret = devm_snd_soc_register_component(&pdev.dev,
    &img_spdif_out_component,
    &img_spdif_out_dai, 1);
    if (ret)
    goto err_suspend;
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, core::ptr::null_mut(), 0);
    if (ret)
    goto err_suspend;
    dev_dbg(&pdev.dev, "Probe successful\n");
    return 0;
    err_suspend:
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_spdif_out_runtime_suspend(&pdev.dev);
    err_pm_disable:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_dev_remove(pdev: *mut platform_device) {
    static void img_spdif_out_dev_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_spdif_out_runtime_suspend(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_suspend(dev: *mut device) -> c_int {
    static int img_spdif_out_suspend(struct device *dev)
    {
    struct img_spdif_out *spdif = dev_get_drvdata(dev);
    int ret;
    if (pm_runtime_status_suspended(dev)) {
    ret = img_spdif_out_runtime_resume(dev);
    if (ret)
    return ret;
    }
    spdif.suspend_ctl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
    spdif.suspend_csl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    spdif.suspend_csh = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    img_spdif_out_runtime_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_spdif_out_resume(dev: *mut device) -> c_int {
    static int img_spdif_out_resume(struct device *dev)
    {
    struct img_spdif_out *spdif = dev_get_drvdata(dev);
    int ret;
    ret = img_spdif_out_runtime_resume(dev);
    if (ret)
    return ret;
    img_spdif_out_writel(spdif, spdif.suspend_ctl, IMG_SPDIF_OUT_CTL);
    img_spdif_out_writel(spdif, spdif.suspend_csl, IMG_SPDIF_OUT_CSL);
    img_spdif_out_writel(spdif, spdif.suspend_csh, IMG_SPDIF_OUT_CSH_UV);
    if (pm_runtime_status_suspended(dev))
    img_spdif_out_runtime_suspend(dev);
    return 0;
    }
    static const struct of_device_id img_spdif_out_of_match[] = {
    { .compatible = "img,spdif-out" },
    {}
    };
    MODULE_DEVICE_TABLE(of, img_spdif_out_of_match);
    static const struct dev_pm_ops img_spdif_out_pm_ops = {
    RUNTIME_PM_OPS(img_spdif_out_runtime_suspend, img_spdif_out_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(img_spdif_out_suspend, img_spdif_out_resume)
    };
    static struct platform_driver img_spdif_out_driver = {
    .driver = {
    .name = "img-spdif-out",
    .of_match_table = img_spdif_out_of_match,
    .pm = pm_ptr(&img_spdif_out_pm_ops)
    },
    .probe = img_spdif_out_probe,
    .remove = img_spdif_out_dev_remove
    };
    module_platform_driver(img_spdif_out_driver);
    MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
    MODULE_DESCRIPTION("IMG SPDIF Output driver");
    MODULE_LICENSE("GPL v2");
