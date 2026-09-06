//! Automatically rewritten from C to Rust
//! Source: sound/soc/img/pistachio-internal-dac.c
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
// Pistachio internal dac driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

pub const PISTACHIO_INTERNAL_DAC_CTRL: c_uint = 0x40;
pub const PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK: c_uint = 0x2;
pub const PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK: c_uint = 0x1;
pub const PISTACHIO_INTERNAL_DAC_SRST: c_uint = 0x44;
pub const PISTACHIO_INTERNAL_DAC_SRST_MASK: c_uint = 0x1;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL: c_uint = 0x48;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_SHIFT: c_int = 0;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_MASK: c_uint = 0xFFF;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK: c_uint = 0x1000;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_SHIFT: c_int = 13;
pub const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_MASK: c_uint = 0x1FE000;
pub const PISTACHIO_INTERNAL_DAC_PWR: c_uint = 0x1;
pub const PISTACHIO_INTERNAL_DAC_PWR_MASK: c_uint = 0x1;

    SNDRV_PCM_FMTBIT_S32_LE)
// codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_internal_dac {
    pub regmap: *mut regmap,
    pub supply: *mut regulator,
    pub mute: bool,
}

    static const struct snd_kcontrol_new pistachio_internal_dac_snd_controls[] = {
    SOC_SINGLE("Playback Switch", PISTACHIO_INTERNAL_DAC_CTRL, 2, 1, 1)
    };
    static const struct snd_soc_dapm_widget pistachio_internal_dac_widgets[] = {
    SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT("AOUTL"),
    SND_SOC_DAPM_OUTPUT("AOUTR"),
    };
    static const struct snd_soc_dapm_route pistachio_internal_dac_routes[] = {
    { "AOUTL", core::ptr::null_mut(), "DAC" },
    { "AOUTR", core::ptr::null_mut(), "DAC" },
    };
    static void pistachio_internal_dac_reg_writel(struct regmap *top_regs,
    u32 val, u32 reg)
    {
    regmap_update_bits(top_regs, PISTACHIO_INTERNAL_DAC_GTI_CTRL,
    PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_MASK,
    reg << PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_SHIFT);
    regmap_update_bits(top_regs, PISTACHIO_INTERNAL_DAC_GTI_CTRL,
    PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_MASK,
    val << PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_SHIFT);
    regmap_update_bits(top_regs, PISTACHIO_INTERNAL_DAC_GTI_CTRL,
    PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK,
    PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK);
    regmap_update_bits(top_regs, PISTACHIO_INTERNAL_DAC_GTI_CTRL,
    PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK, 0);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_pwr_off(dac: *mut pistachio_internal_dac) {
    static void pistachio_internal_dac_pwr_off(struct pistachio_internal_dac *dac)
    {
    regmap_update_bits(dac.regmap, PISTACHIO_INTERNAL_DAC_CTRL,
    PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK,
    PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK);
    pistachio_internal_dac_reg_writel(dac.regmap, 0,
    PISTACHIO_INTERNAL_DAC_PWR);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_pwr_on(dac: *mut pistachio_internal_dac) {
    static void pistachio_internal_dac_pwr_on(struct pistachio_internal_dac *dac)
    {
    regmap_update_bits(dac.regmap, PISTACHIO_INTERNAL_DAC_SRST,
    PISTACHIO_INTERNAL_DAC_SRST_MASK,
    PISTACHIO_INTERNAL_DAC_SRST_MASK);
    regmap_update_bits(dac.regmap, PISTACHIO_INTERNAL_DAC_SRST,
    PISTACHIO_INTERNAL_DAC_SRST_MASK, 0);
    pistachio_internal_dac_reg_writel(dac.regmap,
    PISTACHIO_INTERNAL_DAC_PWR_MASK,
    PISTACHIO_INTERNAL_DAC_PWR);
    regmap_update_bits(dac.regmap, PISTACHIO_INTERNAL_DAC_CTRL,
    PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK, 0);
    }
    static struct snd_soc_dai_driver pistachio_internal_dac_dais[] = {
    {
    .name = "pistachio_internal_dac",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = PISTACHIO_INTERNAL_DAC_FORMATS,
    }
    },
    };
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_codec_probe(component: *mut snd_soc_component) -> c_int {
    static int pistachio_internal_dac_codec_probe(struct snd_soc_component *component)
    {
    struct pistachio_internal_dac *dac = snd_soc_component_get_drvdata(component);
    snd_soc_component_init_regmap(component, dac.regmap);
    return 0;
    }
    static const struct snd_soc_component_driver pistachio_internal_dac_driver = {
    .probe			= pistachio_internal_dac_codec_probe,
    .controls		= pistachio_internal_dac_snd_controls,
    .num_controls		= ARRAY_SIZE(pistachio_internal_dac_snd_controls),
    .dapm_widgets		= pistachio_internal_dac_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(pistachio_internal_dac_widgets),
    .dapm_routes		= pistachio_internal_dac_routes,
    .num_dapm_routes	= ARRAY_SIZE(pistachio_internal_dac_routes),
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_probe(pdev: *mut platform_device) -> c_int {
    static int pistachio_internal_dac_probe(struct platform_device *pdev)
    {
    struct pistachio_internal_dac *dac;
    int ret, voltage;
    struct device *dev = &pdev.dev;
    u32 reg;
    dac = devm_kzalloc(dev, sizeof(*dac), GFP_KERNEL);
    if (!dac)
    return -ENOMEM;
    platform_set_drvdata(pdev, dac);
    dac.regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "img,cr-top");
    if (IS_ERR(dac.regmap))
    return PTR_ERR(dac.regmap);
    dac.supply = devm_regulator_get(dev, "VDD");
    if (IS_ERR(dac.supply))
    return dev_err_probe(dev, PTR_ERR(dac.supply),
    "failed to acquire supply 'VDD-supply'\n");
    ret = regulator_enable(dac.supply);
    if (ret) {
    dev_err(dev, "failed to enable supply: %d\n", ret);
    return ret;
    }
    voltage = regulator_get_voltage(dac.supply);
    switch (voltage) {
    case 1800000:
    reg = 0;
    break;
    case 3300000:
    reg = PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK;
    break;
    default:
    dev_err(dev, "invalid voltage: %d\n", voltage);
    ret = -EINVAL;
    goto err_regulator;
    }
    regmap_update_bits(dac.regmap, PISTACHIO_INTERNAL_DAC_CTRL,
    PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK, reg);
    pistachio_internal_dac_pwr_off(dac);
    pistachio_internal_dac_pwr_on(dac);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);
    ret = devm_snd_soc_register_component(dev,
    &pistachio_internal_dac_driver,
    pistachio_internal_dac_dais,
    ARRAY_SIZE(pistachio_internal_dac_dais));
    if (ret) {
    dev_err(dev, "failed to register component: %d\n", ret);
    goto err_pwr;
    }
    return 0;
    err_pwr:
    pm_runtime_disable(&pdev.dev);
    pistachio_internal_dac_pwr_off(dac);
    err_regulator:
    regulator_disable(dac.supply);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_remove(pdev: *mut platform_device) {
    static void pistachio_internal_dac_remove(struct platform_device *pdev)
    {
    struct pistachio_internal_dac *dac = dev_get_drvdata(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pistachio_internal_dac_pwr_off(dac);
    regulator_disable(dac.supply);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_rt_resume(dev: *mut device) -> c_int {
    static int pistachio_internal_dac_rt_resume(struct device *dev)
    {
    struct pistachio_internal_dac *dac = dev_get_drvdata(dev);
    int ret;
    ret = regulator_enable(dac.supply);
    if (ret) {
    dev_err(dev, "failed to enable supply: %d\n", ret);
    return ret;
    }
    pistachio_internal_dac_pwr_on(dac);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pistachio_internal_dac_rt_suspend(dev: *mut device) -> c_int {
    static int pistachio_internal_dac_rt_suspend(struct device *dev)
    {
    struct pistachio_internal_dac *dac = dev_get_drvdata(dev);
    pistachio_internal_dac_pwr_off(dac);
    regulator_disable(dac.supply);
    return 0;
    }
    static const struct dev_pm_ops pistachio_internal_dac_pm_ops = {
    RUNTIME_PM_OPS(pistachio_internal_dac_rt_suspend,
    pistachio_internal_dac_rt_resume, core::ptr::null_mut())
    };
    static const struct of_device_id pistachio_internal_dac_of_match[] = {
    { .compatible = "img,pistachio-internal-dac" },
    {}
    };
    MODULE_DEVICE_TABLE(of, pistachio_internal_dac_of_match);
    static struct platform_driver pistachio_internal_dac_plat_driver = {
    .driver = {
    .name = "img-pistachio-internal-dac",
    .of_match_table = pistachio_internal_dac_of_match,
    .pm = pm_ptr(&pistachio_internal_dac_pm_ops)
    },
    .probe = pistachio_internal_dac_probe,
    .remove = pistachio_internal_dac_remove
    };
    module_platform_driver(pistachio_internal_dac_plat_driver);
    MODULE_DESCRIPTION("Pistachio Internal DAC driver");
    MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
    MODULE_LICENSE("GPL v2");
