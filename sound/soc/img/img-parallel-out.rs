//! Automatically rewritten from C to Rust
//! Source: sound/soc/img/img-parallel-out.c
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
// IMG parallel output controller driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

pub const IMG_PRL_OUT_TX_FIFO: c_int = 0;
pub const IMG_PRL_OUT_CTL: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_prl_out {
    pub base: *mut void __iomem,
    pub clk_sys: *mut clk,
    pub clk_ref: *mut clk,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub dev: *mut device,
    pub rst: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn img_prl_out_suspend(dev: *mut device) -> c_int {
    static int img_prl_out_suspend(struct device *dev)
    {
    struct img_prl_out *prl = dev_get_drvdata(dev);
    clk_disable_unprepare(prl.clk_ref);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_prl_out_resume(dev: *mut device) -> c_int {
    static int img_prl_out_resume(struct device *dev)
    {
    struct img_prl_out *prl = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(prl.clk_ref);
    if (ret) {
    dev_err(dev, "clk_enable failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static inline void img_prl_out_writel(struct img_prl_out *prl,
    u32 val, u32 reg)
    {
    writel(val, prl.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn img_prl_out_readl(prl: *mut img_prl_out, reg: u32) -> u32 {
    static inline u32 img_prl_out_readl(struct img_prl_out *prl, u32 reg)
    {
    return readl(prl.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn img_prl_out_reset(prl: *mut img_prl_out) {
    static void img_prl_out_reset(struct img_prl_out *prl)
    {
    u32 ctl;
    ctl = img_prl_out_readl(prl, IMG_PRL_OUT_CTL) &
    ~IMG_PRL_OUT_CTL_ME_MASK;
    reset_control_assert(prl.rst);
    reset_control_deassert(prl.rst);
    img_prl_out_writel(prl, ctl, IMG_PRL_OUT_CTL);
    }
    static int img_prl_out_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct img_prl_out *prl = snd_soc_dai_get_drvdata(dai);
    u32 reg;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
    reg |= IMG_PRL_OUT_CTL_ME_MASK;
    img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    img_prl_out_reset(prl);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int img_prl_out_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
    {
    struct img_prl_out *prl = snd_soc_dai_get_drvdata(dai);
    unsigned int rate, channels;
    u32 reg, control_set = 0;
    rate = params_rate(params);
    channels = params_channels(params);
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S32_LE:
    control_set |= IMG_PRL_OUT_CTL_PACKH_MASK;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    break;
    default:
    return -EINVAL;
    }
    if (channels != 2)
    return -EINVAL;
    clk_set_rate(prl.clk_ref, rate * 256);
    reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
    reg = (reg & ~IMG_PRL_OUT_CTL_PACKH_MASK) | control_set;
    img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_prl_out_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int img_prl_out_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct img_prl_out *prl = snd_soc_dai_get_drvdata(dai);
    u32 reg, control_set = 0;
    int ret;
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    break;
    case SND_SOC_DAIFMT_NB_IF:
    control_set |= IMG_PRL_OUT_CTL_EDGE_MASK;
    break;
    default:
    return -EINVAL;
    }
    ret = pm_runtime_resume_and_get(prl.dev);
    if (ret < 0)
    return ret;
    reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
    reg = (reg & ~IMG_PRL_OUT_CTL_EDGE_MASK) | control_set;
    img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);
    pm_runtime_put(prl.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_prl_out_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int img_prl_out_dai_probe(struct snd_soc_dai *dai)
    {
    struct img_prl_out *prl = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &prl.dma_data, core::ptr::null_mut());
    return 0;
    }
    static const struct snd_soc_dai_ops img_prl_out_dai_ops = {
    .probe		= img_prl_out_dai_probe,
    .trigger	= img_prl_out_trigger,
    .hw_params	= img_prl_out_hw_params,
    .set_fmt	= img_prl_out_set_fmt
    };
    static struct snd_soc_dai_driver img_prl_out_dai = {
    .playback = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE
    },
    .ops = &img_prl_out_dai_ops
    };
    static const struct snd_soc_component_driver img_prl_out_component = {
    .name = "img-prl-out",
    .legacy_dai_naming = 1,
    };
#[no_mangle]
unsafe extern "C" fn img_prl_out_probe(pdev: *mut platform_device) -> c_int {
    static int img_prl_out_probe(struct platform_device *pdev)
    {
    struct img_prl_out *prl;
    struct resource *res;
    void __iomem *base;
    int ret;
    struct device *dev = &pdev.dev;
    prl = devm_kzalloc(&pdev.dev, sizeof(*prl), GFP_KERNEL);
    if (!prl)
    return -ENOMEM;
    platform_set_drvdata(pdev, prl);
    prl.dev = &pdev.dev;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    prl.base = base;
    prl.rst = devm_reset_control_get_exclusive(&pdev.dev, "rst");
    if (IS_ERR(prl.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(prl.rst),
    "No top level reset found\n");
    prl.clk_sys = devm_clk_get(&pdev.dev, "sys");
    if (IS_ERR(prl.clk_sys))
    return dev_err_probe(dev, PTR_ERR(prl.clk_sys),
    "Failed to acquire clock 'sys'\n");
    prl.clk_ref = devm_clk_get(&pdev.dev, "ref");
    if (IS_ERR(prl.clk_ref))
    return dev_err_probe(dev, PTR_ERR(prl.clk_ref),
    "Failed to acquire clock 'ref'\n");
    ret = clk_prepare_enable(prl.clk_sys);
    if (ret)
    return ret;
    img_prl_out_writel(prl, IMG_PRL_OUT_CTL_EDGE_MASK, IMG_PRL_OUT_CTL);
    img_prl_out_reset(prl);
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = img_prl_out_resume(&pdev.dev);
    if (ret)
    goto err_pm_disable;
    }
    prl.dma_data.addr = res.start + IMG_PRL_OUT_TX_FIFO;
    prl.dma_data.addr_width = 4;
    prl.dma_data.maxburst = 4;
    ret = devm_snd_soc_register_component(&pdev.dev,
    &img_prl_out_component,
    &img_prl_out_dai, 1);
    if (ret)
    goto err_suspend;
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, core::ptr::null_mut(), 0);
    if (ret)
    goto err_suspend;
    return 0;
    err_suspend:
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_prl_out_suspend(&pdev.dev);
    err_pm_disable:
    pm_runtime_disable(&pdev.dev);
    clk_disable_unprepare(prl.clk_sys);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_prl_out_dev_remove(pdev: *mut platform_device) {
    static void img_prl_out_dev_remove(struct platform_device *pdev)
    {
    struct img_prl_out *prl = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_prl_out_suspend(&pdev.dev);
    clk_disable_unprepare(prl.clk_sys);
    }
    static const struct of_device_id img_prl_out_of_match[] = {
    { .compatible = "img,parallel-out" },
    {}
    };
    MODULE_DEVICE_TABLE(of, img_prl_out_of_match);
    static const struct dev_pm_ops img_prl_out_pm_ops = {
    RUNTIME_PM_OPS(img_prl_out_suspend, img_prl_out_resume, core::ptr::null_mut())
    };
    static struct platform_driver img_prl_out_driver = {
    .driver = {
    .name = "img-parallel-out",
    .of_match_table = img_prl_out_of_match,
    .pm = pm_ptr(&img_prl_out_pm_ops)
    },
    .probe = img_prl_out_probe,
    .remove = img_prl_out_dev_remove
    };
    module_platform_driver(img_prl_out_driver);
    MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
    MODULE_DESCRIPTION("IMG Parallel Output Driver");
    MODULE_LICENSE("GPL v2");
