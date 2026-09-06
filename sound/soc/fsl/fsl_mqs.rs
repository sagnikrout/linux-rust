//! Automatically rewritten from C to Rust
//! Source: sound/soc/fsl/fsl_mqs.c
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
// ALSA SoC IMX MQS driver
//
// Copyright (C) 2014-2015 Freescale Semiconductor, Inc.
// Copyright 2019 NXP

pub const REG_MQS_CTRL: c_uint = 0x00;

    enum reg_type {
    TYPE_REG_OWN,  /* module own register space */
    TYPE_REG_GPR,  /* register in GPR space */
    TYPE_REG_SM,   /* System Manager controls the register */
    };
//
// struct fsl_mqs_soc_data - soc specific data
//
// @type: control register space type
// @sm_index: index from definition in system manager
// @ctrl_off: control register offset
// @en_mask: enable bit mask
// @en_shift: enable bit shift
// @rst_mask: reset bit mask
// @rst_shift: reset bit shift
// @osr_mask: oversample bit mask
// @osr_shift: oversample bit shift
// @div_mask: clock divider mask
// @div_shift: clock divider bit shift
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mqs_soc_data {
    pub type: enum reg_type,
    pub sm_index: c_int,
    pub ctrl_off: c_int,
    pub en_mask: c_int,
    pub en_shift: c_int,
    pub rst_mask: c_int,
    pub rst_shift: c_int,
    pub osr_mask: c_int,
    pub osr_shift: c_int,
    pub div_mask: c_int,
    pub div_shift: c_int,
}

// codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mqs {
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub ipg: *mut clk,
    pub soc: *const fsl_mqs_soc_data,
    pub reg_mqs_ctrl: c_uint,
}

#[no_mangle]
unsafe extern "C" fn fsl_mqs_sm_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int fsl_mqs_sm_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct fsl_mqs *mqs_priv = context;
    let mut num: c_int = 1;
    if (IS_ENABLED(CONFIG_IMX_SCMI_MISC_DRV) &&
    mqs_priv.soc.ctrl_off == reg)
    return scmi_imx_misc_ctrl_get(mqs_priv.soc.sm_index, &num, val);
    return -EINVAL;
    };
#[no_mangle]
unsafe extern "C" fn fsl_mqs_sm_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int fsl_mqs_sm_write(void *context, unsigned int reg, unsigned int val)
    {
    struct fsl_mqs *mqs_priv = context;
    if (IS_ENABLED(CONFIG_IMX_SCMI_MISC_DRV) &&
    mqs_priv.soc.ctrl_off == reg)
    return scmi_imx_misc_ctrl_set(mqs_priv.soc.sm_index, val);
    return -EINVAL;
    };
    static int fsl_mqs_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct fsl_mqs *mqs_priv = snd_soc_component_get_drvdata(component);
    unsigned long mclk_rate;
    int div, res;
    int lrclk;
    mclk_rate = clk_get_rate(mqs_priv.mclk);
    lrclk = params_rate(params);
//
// mclk_rate / (oversample(32,64) * FS * 2 * divider ) = repeat_rate;
// if repeat_rate is 8, mqs can achieve better quality.
// oversample rate is fix to 32 currently.
//
    div = mclk_rate / (32 * lrclk * 2 * 8);
    res = mclk_rate % (32 * lrclk * 2 * 8);
    if (res == 0 && div > 0 && div <= 256) {
    regmap_update_bits(mqs_priv.regmap, mqs_priv.soc.ctrl_off,
    mqs_priv.soc.div_mask,
    (div - 1) << mqs_priv.soc.div_shift);
    regmap_update_bits(mqs_priv.regmap, mqs_priv.soc.ctrl_off,
    mqs_priv.soc.osr_mask, 0);
    } else {
    dev_err(component.dev, "can't get proper divider\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_mqs_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int fsl_mqs_set_dai_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
// Only LEFT_J & SLAVE mode is supported.
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_LEFT_J:
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
    case SND_SOC_DAIFMT_NB_NF:
    break;
    default:
    return -EINVAL;
    }
    switch (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_CBC_CFC:
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int fsl_mqs_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct fsl_mqs *mqs_priv = snd_soc_component_get_drvdata(component);
    regmap_update_bits(mqs_priv.regmap, mqs_priv.soc.ctrl_off,
    mqs_priv.soc.en_mask,
    1 << mqs_priv.soc.en_shift);
    return 0;
    }
    static void fsl_mqs_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct fsl_mqs *mqs_priv = snd_soc_component_get_drvdata(component);
    regmap_update_bits(mqs_priv.regmap, mqs_priv.soc.ctrl_off,
    mqs_priv.soc.en_mask, 0);
    }
    static const struct snd_soc_component_driver soc_codec_fsl_mqs = {
    .idle_bias_on = 1,
    };
    static const struct snd_soc_dai_ops fsl_mqs_dai_ops = {
    .startup = fsl_mqs_startup,
    .shutdown = fsl_mqs_shutdown,
    .hw_params = fsl_mqs_hw_params,
    .set_fmt = fsl_mqs_set_dai_fmt,
    };
    static struct snd_soc_dai_driver fsl_mqs_dai = {
    .name		= "fsl-mqs-dai",
    .playback	= {
    .stream_name	= "Playback",
    .channels_min	= 2,
    .channels_max	= 2,
    .rates		= FSL_MQS_RATES,
    .formats	= FSL_MQS_FORMATS,
    },
    .ops = &fsl_mqs_dai_ops,
    };
    static const struct regmap_config fsl_mqs_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = REG_MQS_CTRL,
    .cache_type = REGCACHE_NONE,
    };
    static const struct regmap_config fsl_mqs_sm_regmap = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_read = fsl_mqs_sm_read,
    .reg_write = fsl_mqs_sm_write,
    };
#[no_mangle]
unsafe extern "C" fn fsl_mqs_probe(pdev: *mut platform_device) -> c_int {
    static int fsl_mqs_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device_node *gpr_np = core::ptr::null_mut();
    struct fsl_mqs *mqs_priv;
    void __iomem *regs;
    int ret;
    mqs_priv = devm_kzalloc(&pdev.dev, sizeof(*mqs_priv), GFP_KERNEL);
    if (!mqs_priv)
    return -ENOMEM;
// On i.MX6sx the MQS control register is in GPR domain
// But in i.MX8QM/i.MX8QXP the control register is moved
// to its own domain.
//
    mqs_priv.soc = of_device_get_match_data(&pdev.dev);
    if (mqs_priv.soc.type == TYPE_REG_GPR) {
    gpr_np = of_parse_phandle(np, "gpr", 0);
    if (!gpr_np) {
    dev_err(&pdev.dev, "failed to get gpr node by phandle\n");
    return -EINVAL;
    }
    mqs_priv.regmap = syscon_node_to_regmap(gpr_np);
    of_node_put(gpr_np);
    if (IS_ERR(mqs_priv.regmap)) {
    dev_err(&pdev.dev, "failed to get gpr regmap\n");
    return PTR_ERR(mqs_priv.regmap);
    }
    } else if (mqs_priv.soc.type == TYPE_REG_SM) {
    mqs_priv.regmap = devm_regmap_init(&pdev.dev,
    core::ptr::null_mut(),
    mqs_priv,
    &fsl_mqs_sm_regmap);
    if (IS_ERR(mqs_priv.regmap)) {
    dev_err(&pdev.dev, "failed to init regmap: %ld\n",
    PTR_ERR(mqs_priv.regmap));
    return PTR_ERR(mqs_priv.regmap);
    }
    } else {
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    mqs_priv.regmap = devm_regmap_init_mmio_clk(&pdev.dev,
    "core",
    regs,
    &fsl_mqs_regmap_config);
    if (IS_ERR(mqs_priv.regmap)) {
    dev_err(&pdev.dev, "failed to init regmap: %ld\n",
    PTR_ERR(mqs_priv.regmap));
    return PTR_ERR(mqs_priv.regmap);
    }
    mqs_priv.ipg = devm_clk_get(&pdev.dev, "core");
    if (IS_ERR(mqs_priv.ipg)) {
    dev_err(&pdev.dev, "failed to get the clock: %ld\n",
    PTR_ERR(mqs_priv.ipg));
    return PTR_ERR(mqs_priv.ipg);
    }
    }
    mqs_priv.mclk = devm_clk_get(&pdev.dev, "mclk");
    if (IS_ERR(mqs_priv.mclk)) {
    dev_err(&pdev.dev, "failed to get the clock: %ld\n",
    PTR_ERR(mqs_priv.mclk));
    return PTR_ERR(mqs_priv.mclk);
    }
    dev_set_drvdata(&pdev.dev, mqs_priv);
    pm_runtime_enable(&pdev.dev);
    ret = devm_snd_soc_register_component(&pdev.dev, &soc_codec_fsl_mqs,
    &fsl_mqs_dai, 1);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_mqs_remove(pdev: *mut platform_device) {
    static void fsl_mqs_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn fsl_mqs_runtime_resume(dev: *mut device) -> c_int {
    static int fsl_mqs_runtime_resume(struct device *dev)
    {
    struct fsl_mqs *mqs_priv = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(mqs_priv.ipg);
    if (ret) {
    dev_err(dev, "failed to enable ipg clock\n");
    return ret;
    }
    ret = clk_prepare_enable(mqs_priv.mclk);
    if (ret) {
    dev_err(dev, "failed to enable mclk clock\n");
    clk_disable_unprepare(mqs_priv.ipg);
    return ret;
    }
    regmap_write(mqs_priv.regmap, mqs_priv.soc.ctrl_off, mqs_priv.reg_mqs_ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_mqs_runtime_suspend(dev: *mut device) -> c_int {
    static int fsl_mqs_runtime_suspend(struct device *dev)
    {
    struct fsl_mqs *mqs_priv = dev_get_drvdata(dev);
    regmap_read(mqs_priv.regmap, mqs_priv.soc.ctrl_off, &mqs_priv.reg_mqs_ctrl);
    clk_disable_unprepare(mqs_priv.mclk);
    clk_disable_unprepare(mqs_priv.ipg);
    return 0;
    }
    static const struct dev_pm_ops fsl_mqs_pm_ops = {
    RUNTIME_PM_OPS(fsl_mqs_runtime_suspend, fsl_mqs_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx8qm_data = {
    .type = TYPE_REG_OWN,
    .ctrl_off = REG_MQS_CTRL,
    .en_mask  = MQS_EN_MASK,
    .en_shift = MQS_EN_SHIFT,
    .rst_mask = MQS_SW_RST_MASK,
    .rst_shift = MQS_SW_RST_SHIFT,
    .osr_mask = MQS_OVERSAMPLE_MASK,
    .osr_shift = MQS_OVERSAMPLE_SHIFT,
    .div_mask = MQS_CLK_DIV_MASK,
    .div_shift = MQS_CLK_DIV_SHIFT,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx6sx_data = {
    .type = TYPE_REG_GPR,
    .ctrl_off = IOMUXC_GPR2,
    .en_mask  = IMX6SX_GPR2_MQS_EN_MASK,
    .en_shift = IMX6SX_GPR2_MQS_EN_SHIFT,
    .rst_mask = IMX6SX_GPR2_MQS_SW_RST_MASK,
    .rst_shift = IMX6SX_GPR2_MQS_SW_RST_SHIFT,
    .osr_mask  = IMX6SX_GPR2_MQS_OVERSAMPLE_MASK,
    .osr_shift = IMX6SX_GPR2_MQS_OVERSAMPLE_SHIFT,
    .div_mask  = IMX6SX_GPR2_MQS_CLK_DIV_MASK,
    .div_shift = IMX6SX_GPR2_MQS_CLK_DIV_SHIFT,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx93_data = {
    .type = TYPE_REG_GPR,
    .ctrl_off = 0x20,
    .en_mask  = BIT(1),
    .en_shift = 1,
    .rst_mask = BIT(2),
    .rst_shift = 2,
    .osr_mask = BIT(3),
    .osr_shift = 3,
    .div_mask = GENMASK(15, 8),
    .div_shift = 8,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx95_aon_data = {
    .type = TYPE_REG_SM,
    .sm_index = SCMI_IMX95_CTRL_MQS1_SETTINGS,
    .ctrl_off = 0x88,
    .en_mask  = BIT(1),
    .en_shift = 1,
    .rst_mask = BIT(2),
    .rst_shift = 2,
    .osr_mask = BIT(3),
    .osr_shift = 3,
    .div_mask = GENMASK(15, 8),
    .div_shift = 8,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx95_netc_data = {
    .type = TYPE_REG_GPR,
    .ctrl_off = 0x0,
    .en_mask  = BIT(2),
    .en_shift = 2,
    .rst_mask = BIT(3),
    .rst_shift = 3,
    .osr_mask = BIT(4),
    .osr_shift = 4,
    .div_mask = GENMASK(16, 9),
    .div_shift = 9,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx943_aon_data = {
    .type = TYPE_REG_SM,
    .sm_index = SCMI_IMX94_CTRL_MQS1_SETTINGS,
    .ctrl_off = 0x88,
    .en_mask  = BIT(1),
    .en_shift = 1,
    .rst_mask = BIT(2),
    .rst_shift = 2,
    .osr_mask = BIT(3),
    .osr_shift = 3,
    .div_mask = GENMASK(15, 8),
    .div_shift = 8,
    };
    static const struct fsl_mqs_soc_data fsl_mqs_imx943_wakeup_data = {
    .type = TYPE_REG_SM,
    .sm_index = SCMI_IMX94_CTRL_MQS2_SETTINGS,
    .ctrl_off = 0x10,
    .en_mask  = BIT(1),
    .en_shift = 1,
    .rst_mask = BIT(2),
    .rst_shift = 2,
    .osr_mask = BIT(3),
    .osr_shift = 3,
    .div_mask = GENMASK(15, 8),
    .div_shift = 8,
    };
    static const struct of_device_id fsl_mqs_dt_ids[] = {
    { .compatible = "fsl,imx8qm-mqs", .data = &fsl_mqs_imx8qm_data },
    { .compatible = "fsl,imx6sx-mqs", .data = &fsl_mqs_imx6sx_data },
    { .compatible = "fsl,imx93-mqs", .data = &fsl_mqs_imx93_data },
    { .compatible = "fsl,imx95-aonmix-mqs", .data = &fsl_mqs_imx95_aon_data },
    { .compatible = "fsl,imx95-netcmix-mqs", .data = &fsl_mqs_imx95_netc_data },
    { .compatible = "fsl,imx943-aonmix-mqs", .data = &fsl_mqs_imx943_aon_data },
    { .compatible = "fsl,imx943-wakeupmix-mqs", .data = &fsl_mqs_imx943_wakeup_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, fsl_mqs_dt_ids);
    static struct platform_driver fsl_mqs_driver = {
    .probe		= fsl_mqs_probe,
    .remove		= fsl_mqs_remove,
    .driver		= {
    .name	= "fsl-mqs",
    .of_match_table = fsl_mqs_dt_ids,
    .pm = pm_ptr(&fsl_mqs_pm_ops),
    },
    };
    module_platform_driver(fsl_mqs_driver);
    MODULE_AUTHOR("Shengjiu Wang <Shengjiu.Wang@nxp.com>");
    MODULE_DESCRIPTION("MQS codec driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:fsl-mqs");
