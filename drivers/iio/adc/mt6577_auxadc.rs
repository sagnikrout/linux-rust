//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/mt6577_auxadc.c
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Zhiyong Tao <zhiyong.tao@mediatek.com>
//

// Register definitions
pub const MT6577_AUXADC_CON0: c_uint = 0x00;
pub const MT6577_AUXADC_CON1: c_uint = 0x04;
pub const MT6577_AUXADC_CON2: c_uint = 0x10;

pub const MT6577_AUXADC_DAT0: c_uint = 0x14;

pub const MT6577_AUXADC_MISC: c_uint = 0x94;

pub const MT6577_AUXADC_DAT_MASK: c_uint = 0xfff;
pub const MT6577_AUXADC_SLEEP_US: c_int = 1000;
pub const MT6577_AUXADC_TIMEOUT_US: c_int = 10000;
pub const MT6577_AUXADC_POWER_READY_MS: c_int = 1;
pub const MT6577_AUXADC_SAMPLE_READY_US: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_auxadc_compatible {
    pub sample_data_cali: bool,
    pub check_global_idle: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6577_auxadc_device {
    pub reg_base: *mut void __iomem,
    pub adc_clk: *mut clk,
    pub lock: mutex,
    pub dev_comp: *const mtk_auxadc_compatible,
}

    static const struct mtk_auxadc_compatible mt8186_compat = {
    .sample_data_cali = false,
    .check_global_idle = false,
    };
    static const struct mtk_auxadc_compatible mt8173_compat = {
    .sample_data_cali = false,
    .check_global_idle = true,
    };
    static const struct mtk_auxadc_compatible mt6765_compat = {
    .sample_data_cali = true,
    .check_global_idle = false,
    };

    .type = IIO_VOLTAGE,				    \
    .indexed = 1,					    \
    .channel = (idx),				    \
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED), \
    }
    static const struct iio_chan_spec mt6577_auxadc_iio_channels[] = {
    MT6577_AUXADC_CHANNEL(0),
    MT6577_AUXADC_CHANNEL(1),
    MT6577_AUXADC_CHANNEL(2),
    MT6577_AUXADC_CHANNEL(3),
    MT6577_AUXADC_CHANNEL(4),
    MT6577_AUXADC_CHANNEL(5),
    MT6577_AUXADC_CHANNEL(6),
    MT6577_AUXADC_CHANNEL(7),
    MT6577_AUXADC_CHANNEL(8),
    MT6577_AUXADC_CHANNEL(9),
    MT6577_AUXADC_CHANNEL(10),
    MT6577_AUXADC_CHANNEL(11),
    MT6577_AUXADC_CHANNEL(12),
    MT6577_AUXADC_CHANNEL(13),
    MT6577_AUXADC_CHANNEL(14),
    MT6577_AUXADC_CHANNEL(15),
    };
// For Voltage calculation

#[no_mangle]
unsafe extern "C" fn mt_auxadc_get_cali_data(rawdata: c_int, enable_cali: bool) -> c_int {
    static int mt_auxadc_get_cali_data(int rawdata, bool enable_cali)
    {
    return rawdata;
    }
    static inline void mt6577_auxadc_mod_reg(void __iomem *reg,
    u32 or_mask, u32 and_mask)
    {
    u32 val;
    val = readl(reg);
    val |= or_mask;
    val &= ~and_mask;
    writel(val, reg);
    }
    static int mt6577_auxadc_read(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan)
    {
    u32 val;
    void __iomem *reg_channel;
    int ret;
    struct mt6577_auxadc_device *adc_dev = iio_priv(indio_dev);
    reg_channel = adc_dev.reg_base + MT6577_AUXADC_DAT0 +
    chan.channel * 0x04;
    mutex_lock(&adc_dev.lock);
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_CON1,
    0, 1 << chan.channel);
// read channel and make sure old ready bit == 0
    ret = readl_poll_timeout(reg_channel, val,
    ((val & MT6577_AUXADC_RDY0) == 0),
    MT6577_AUXADC_SLEEP_US,
    MT6577_AUXADC_TIMEOUT_US);
    if (ret < 0) {
    dev_err(indio_dev.dev.parent,
    "wait for channel[%d] ready bit clear time out\n",
    chan.channel);
    goto err_timeout;
    }
// set bit to trigger sample
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_CON1,
    1 << chan.channel, 0);
// we must delay here for hardware sample channel data
    udelay(MT6577_AUXADC_SAMPLE_READY_US);
    if (adc_dev.dev_comp.check_global_idle) {
// check MTK_AUXADC_CON2 if auxadc is idle
    ret = readl_poll_timeout(adc_dev.reg_base + MT6577_AUXADC_CON2,
    val, ((val & MT6577_AUXADC_STA) == 0),
    MT6577_AUXADC_SLEEP_US,
    MT6577_AUXADC_TIMEOUT_US);
    if (ret < 0) {
    dev_err(indio_dev.dev.parent,
    "wait for auxadc idle time out\n");
    goto err_timeout;
    }
    }
// read channel and make sure ready bit == 1
    ret = readl_poll_timeout(reg_channel, val,
    ((val & MT6577_AUXADC_RDY0) != 0),
    MT6577_AUXADC_SLEEP_US,
    MT6577_AUXADC_TIMEOUT_US);
    if (ret < 0) {
    dev_err(indio_dev.dev.parent,
    "wait for channel[%d] data ready time out\n",
    chan.channel);
    goto err_timeout;
    }
// read data
    val = readl(reg_channel) & MT6577_AUXADC_DAT_MASK;
    mutex_unlock(&adc_dev.lock);
    return val;
    err_timeout:
    mutex_unlock(&adc_dev.lock);
    return -ETIMEDOUT;
    }
    static int mt6577_auxadc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long info)
    {
    struct mt6577_auxadc_device *adc_dev = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_PROCESSED:
// val = mt6577_auxadc_read(indio_dev, chan);
    if (*val < 0) {
    dev_err(indio_dev.dev.parent,
    "failed to sample data on channel[%d]\n",
    chan.channel);
    return *val;
    }
    if (adc_dev.dev_comp.sample_data_cali)
// val = mt_auxadc_get_cali_data(*val, true);
// Convert adc raw data to voltage: 0 - 1500 mV
// val = *val * VOLTAGE_FULL_RANGE / AUXADC_PRECISE;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info mt6577_auxadc_info = {
    .read_raw = &mt6577_auxadc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn mt6577_auxadc_resume(dev: *mut device) -> c_int {
    static int mt6577_auxadc_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct mt6577_auxadc_device *adc_dev = iio_priv(indio_dev);
    int ret;
    ret = clk_prepare_enable(adc_dev.adc_clk);
    if (ret) {
    pr_err("failed to enable auxadc clock\n");
    return ret;
    }
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_MISC,
    MT6577_AUXADC_PDN_EN, 0);
    mdelay(MT6577_AUXADC_POWER_READY_MS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6577_auxadc_suspend(dev: *mut device) -> c_int {
    static int mt6577_auxadc_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct mt6577_auxadc_device *adc_dev = iio_priv(indio_dev);
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_MISC,
    0, MT6577_AUXADC_PDN_EN);
    clk_disable_unprepare(adc_dev.adc_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6577_power_off(data: *mut c_void) {
    static void mt6577_power_off(void *data)
    {
    struct mt6577_auxadc_device *adc_dev = data;
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_MISC,
    0, MT6577_AUXADC_PDN_EN);
    }
#[no_mangle]
unsafe extern "C" fn mt6577_auxadc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6577_auxadc_probe(struct platform_device *pdev)
    {
    struct mt6577_auxadc_device *adc_dev;
    unsigned long adc_clk_rate;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*adc_dev));
    if (!indio_dev)
    return -ENOMEM;
    adc_dev = iio_priv(indio_dev);
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.info = &mt6577_auxadc_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = mt6577_auxadc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(mt6577_auxadc_iio_channels);
    adc_dev.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(adc_dev.reg_base))
    return dev_err_probe(&pdev.dev, PTR_ERR(adc_dev.reg_base),
    "failed to get auxadc base address\n");
    adc_dev.adc_clk = devm_clk_get_enabled(&pdev.dev, "main");
    if (IS_ERR(adc_dev.adc_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(adc_dev.adc_clk),
    "failed to enable auxadc clock\n");
    adc_clk_rate = clk_get_rate(adc_dev.adc_clk);
    if (!adc_clk_rate)
    return dev_err_probe(&pdev.dev, -EINVAL, "null clock rate\n");
    adc_dev.dev_comp = device_get_match_data(&pdev.dev);
    mutex_init(&adc_dev.lock);
    mt6577_auxadc_mod_reg(adc_dev.reg_base + MT6577_AUXADC_MISC,
    MT6577_AUXADC_PDN_EN, 0);
    mdelay(MT6577_AUXADC_POWER_READY_MS);
    platform_set_drvdata(pdev, indio_dev);
    ret = devm_add_action_or_reset(&pdev.dev, mt6577_power_off, adc_dev);
    if (ret)
    return ret;
    ret = devm_iio_device_register(&pdev.dev, indio_dev);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "failed to register iio device\n");
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mt6577_auxadc_pm_ops,
    mt6577_auxadc_suspend,
    mt6577_auxadc_resume);
    static const struct of_device_id mt6577_auxadc_of_match[] = {
    { .compatible = "mediatek,mt2701-auxadc", .data = &mt8173_compat },
    { .compatible = "mediatek,mt2712-auxadc", .data = &mt8173_compat },
    { .compatible = "mediatek,mt7622-auxadc", .data = &mt8173_compat },
    { .compatible = "mediatek,mt8173-auxadc", .data = &mt8173_compat },
    { .compatible = "mediatek,mt8186-auxadc", .data = &mt8186_compat },
    { .compatible = "mediatek,mt6765-auxadc", .data = &mt6765_compat },
    { }
    };
    MODULE_DEVICE_TABLE(of, mt6577_auxadc_of_match);
    static struct platform_driver mt6577_auxadc_driver = {
    .driver = {
    .name   = "mt6577-auxadc",
    .of_match_table = mt6577_auxadc_of_match,
    .pm = pm_sleep_ptr(&mt6577_auxadc_pm_ops),
    },
    .probe	= mt6577_auxadc_probe,
    };
    module_platform_driver(mt6577_auxadc_driver);
    MODULE_AUTHOR("Zhiyong Tao <zhiyong.tao@mediatek.com>");
    MODULE_DESCRIPTION("MTK AUXADC Device Driver");
    MODULE_LICENSE("GPL v2");
