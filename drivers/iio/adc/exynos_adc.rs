//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/exynos_adc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// exynos_adc.c - Support for ADC in EXYNOS SoCs
//
// 8 ~ 10 channel, 10/12-bit ADC
//
// Copyright (C) 2013 Naveen Krishna Chatradhi <ch.naveen@samsung.com>
//

// S3C/EXYNOS4412/5250 ADC_V1 registers definitions

// Future ADC_V2 registers definitions

// Bit definitions for ADC_V1

// Bit definitions for S3C2410 / S3C6410 ADC

// ADCTSC Register Bits

    ADC_S3C2410_TSC_YP_SEN | \
    ADC_S3C2410_TSC_XP_SEN | \
    ADC_S3C2410_TSC_XY_PST(3))
// Bit definitions for ADC_V2

pub const ADC_V2_CON2_ACH_MASK: c_uint = 0xF;
pub const MAX_ADC_V2_CHANNELS: c_int = 10;
pub const MAX_ADC_V1_CHANNELS: c_int = 8;
pub const MAX_EXYNOS3250_ADC_CHANNELS: c_int = 2;
pub const MAX_EXYNOS4212_ADC_CHANNELS: c_int = 4;
pub const MAX_S5PV210_ADC_CHANNELS: c_int = 10;
// Bit definitions common for ADC_V1 and ADC_V2

pub const ADC_DATX_MASK: c_uint = 0xFFF;
pub const ADC_DATY_MASK: c_uint = 0xFFF;

pub const EXYNOS_ADCV1_PHY_OFFSET: c_uint = 0x0718;
pub const EXYNOS_ADCV2_PHY_OFFSET: c_uint = 0x0720;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_adc {
    pub data: *mut exynos_adc_data,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub pmu_map: *mut regmap,
    pub clk: *mut clk,
    pub sclk: *mut clk,
    pub irq: c_uint,
    pub vdd: *mut regulator,
    pub completion: completion,
    pub value: u32,
    pub version: c_uint,
//
// Lock to protect from potential concurrent access to the
// completion callback during a manual conversion. For this driver
// a wait-callback is used to wait for the conversion result,
// so in the meantime no other read request (or conversion start)
// must be performed, otherwise it would interfere with the
// current conversion result.
//
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_adc_data {
    pub num_channels: c_int,
    pub needs_sclk: bool,
    pub needs_adc_phy: bool,
    pub phy_offset: c_int,
    pub mask: u32,
    pub info): *mut *mut void (init_hw)(struct exynos_adc,
    pub info): *mut *mut void (exit_hw)(struct exynos_adc,
    pub info): *mut *mut void (clear_irq)(struct exynos_adc,
    pub addr): *mut *mut *mut void (start_conv)(struct exynos_adc info, unsigned long,
}

#[no_mangle]
unsafe extern "C" fn exynos_adc_unprepare_clk(info: *mut exynos_adc) {
    static void exynos_adc_unprepare_clk(struct exynos_adc *info)
    {
    if (info.data.needs_sclk)
    clk_unprepare(info.sclk);
    clk_unprepare(info.clk);
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_prepare_clk(info: *mut exynos_adc) -> c_int {
    static int exynos_adc_prepare_clk(struct exynos_adc *info)
    {
    int ret;
    ret = clk_prepare(info.clk);
    if (ret) {
    dev_err(info.dev, "failed preparing adc clock: %d\n", ret);
    return ret;
    }
    if (info.data.needs_sclk) {
    ret = clk_prepare(info.sclk);
    if (ret) {
    clk_unprepare(info.clk);
    dev_err(info.dev,
    "failed preparing sclk_adc clock: %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_disable_clk(info: *mut exynos_adc) {
    static void exynos_adc_disable_clk(struct exynos_adc *info)
    {
    if (info.data.needs_sclk)
    clk_disable(info.sclk);
    clk_disable(info.clk);
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_enable_clk(info: *mut exynos_adc) -> c_int {
    static int exynos_adc_enable_clk(struct exynos_adc *info)
    {
    int ret;
    ret = clk_enable(info.clk);
    if (ret) {
    dev_err(info.dev, "failed enabling adc clock: %d\n", ret);
    return ret;
    }
    if (info.data.needs_sclk) {
    ret = clk_enable(info.sclk);
    if (ret) {
    clk_disable(info.clk);
    dev_err(info.dev,
    "failed enabling sclk_adc clock: %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_v1_init_hw(info: *mut exynos_adc) {
    static void exynos_adc_v1_init_hw(struct exynos_adc *info)
    {
    u32 con1;
    if (info.data.needs_adc_phy)
    regmap_write(info.pmu_map, info.data.phy_offset, 1);
// set default prescaler values and Enable prescaler
    con1 =  ADC_V1_CON_PRSCLV(49) | ADC_V1_CON_PRSCEN;
// Enable 12-bit ADC resolution
    con1 |= ADC_V1_CON_RES;
    writel(con1, ADC_V1_CON(info.regs));
// set touchscreen delay
    writel(10000, ADC_V1_DLY(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_v1_exit_hw(info: *mut exynos_adc) {
    static void exynos_adc_v1_exit_hw(struct exynos_adc *info)
    {
    u32 con;
    if (info.data.needs_adc_phy)
    regmap_write(info.pmu_map, info.data.phy_offset, 0);
    con = readl(ADC_V1_CON(info.regs));
    con |= ADC_V1_CON_STANDBY;
    writel(con, ADC_V1_CON(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_v1_clear_irq(info: *mut exynos_adc) {
    static void exynos_adc_v1_clear_irq(struct exynos_adc *info)
    {
    writel(1, ADC_V1_INTCLR(info.regs));
    }
    static void exynos_adc_v1_start_conv(struct exynos_adc *info,
    unsigned long addr)
    {
    u32 con1;
    writel(addr, ADC_V1_MUX(info.regs));
    con1 = readl(ADC_V1_CON(info.regs));
    writel(con1 | ADC_CON_EN_START, ADC_V1_CON(info.regs));
    }
// Exynos4212 and 4412 is like ADCv1 but with four channels only
    static const struct exynos_adc_data exynos4212_adc_data = {
    .num_channels	= MAX_EXYNOS4212_ADC_CHANNELS,
    .mask		= ADC_DATX_MASK,	/* 12 bit ADC resolution */
    .needs_adc_phy	= true,
    .phy_offset	= EXYNOS_ADCV1_PHY_OFFSET,
    .init_hw	= exynos_adc_v1_init_hw,
    .exit_hw	= exynos_adc_v1_exit_hw,
    .clear_irq	= exynos_adc_v1_clear_irq,
    .start_conv	= exynos_adc_v1_start_conv,
    };
    static const struct exynos_adc_data exynos_adc_v1_data = {
    .num_channels	= MAX_ADC_V1_CHANNELS,
    .mask		= ADC_DATX_MASK,	/* 12 bit ADC resolution */
    .needs_adc_phy	= true,
    .phy_offset	= EXYNOS_ADCV1_PHY_OFFSET,
    .init_hw	= exynos_adc_v1_init_hw,
    .exit_hw	= exynos_adc_v1_exit_hw,
    .clear_irq	= exynos_adc_v1_clear_irq,
    .start_conv	= exynos_adc_v1_start_conv,
    };
    static const struct exynos_adc_data exynos_adc_s5pv210_data = {
    .num_channels	= MAX_S5PV210_ADC_CHANNELS,
    .mask		= ADC_DATX_MASK,	/* 12 bit ADC resolution */
    .init_hw	= exynos_adc_v1_init_hw,
    .exit_hw	= exynos_adc_v1_exit_hw,
    .clear_irq	= exynos_adc_v1_clear_irq,
    .start_conv	= exynos_adc_v1_start_conv,
    };
    static void exynos_adc_s3c64xx_start_conv(struct exynos_adc *info,
    unsigned long addr)
    {
    u32 con1;
    con1 = readl(ADC_V1_CON(info.regs));
    con1 &= ~ADC_S3C2410_CON_SELMUX(0x7);
    con1 |= ADC_S3C2410_CON_SELMUX(addr);
    writel(con1 | ADC_CON_EN_START, ADC_V1_CON(info.regs));
    }
    static struct exynos_adc_data const exynos_adc_s3c64xx_data = {
    .num_channels	= MAX_ADC_V1_CHANNELS,
    .mask		= ADC_DATX_MASK,	/* 12 bit ADC resolution */
    .init_hw	= exynos_adc_v1_init_hw,
    .exit_hw	= exynos_adc_v1_exit_hw,
    .clear_irq	= exynos_adc_v1_clear_irq,
    .start_conv	= exynos_adc_s3c64xx_start_conv,
    };
#[no_mangle]
unsafe extern "C" fn exynos_adc_v2_init_hw(info: *mut exynos_adc) {
    static void exynos_adc_v2_init_hw(struct exynos_adc *info)
    {
    u32 con1, con2;
    if (info.data.needs_adc_phy)
    regmap_write(info.pmu_map, info.data.phy_offset, 1);
    con1 = ADC_V2_CON1_SOFT_RESET;
    writel(con1, ADC_V2_CON1(info.regs));
    con2 = ADC_V2_CON2_OSEL | ADC_V2_CON2_ESEL |
    ADC_V2_CON2_HIGHF | ADC_V2_CON2_C_TIME(0);
    writel(con2, ADC_V2_CON2(info.regs));
// Enable interrupts
    writel(1, ADC_V2_INT_EN(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_v2_exit_hw(info: *mut exynos_adc) {
    static void exynos_adc_v2_exit_hw(struct exynos_adc *info)
    {
    u32 con;
    if (info.data.needs_adc_phy)
    regmap_write(info.pmu_map, info.data.phy_offset, 0);
    con = readl(ADC_V2_CON1(info.regs));
    con &= ~ADC_CON_EN_START;
    writel(con, ADC_V2_CON1(info.regs));
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_v2_clear_irq(info: *mut exynos_adc) {
    static void exynos_adc_v2_clear_irq(struct exynos_adc *info)
    {
    writel(1, ADC_V2_INT_ST(info.regs));
    }
    static void exynos_adc_v2_start_conv(struct exynos_adc *info,
    unsigned long addr)
    {
    u32 con1, con2;
    con2 = readl(ADC_V2_CON2(info.regs));
    con2 &= ~ADC_V2_CON2_ACH_MASK;
    con2 |= ADC_V2_CON2_ACH_SEL(addr);
    writel(con2, ADC_V2_CON2(info.regs));
    con1 = readl(ADC_V2_CON1(info.regs));
    writel(con1 | ADC_CON_EN_START, ADC_V2_CON1(info.regs));
    }
    static const struct exynos_adc_data exynos_adc_v2_data = {
    .num_channels	= MAX_ADC_V2_CHANNELS,
    .mask		= ADC_DATX_MASK, /* 12 bit ADC resolution */
    .needs_adc_phy	= true,
    .phy_offset	= EXYNOS_ADCV2_PHY_OFFSET,
    .init_hw	= exynos_adc_v2_init_hw,
    .exit_hw	= exynos_adc_v2_exit_hw,
    .clear_irq	= exynos_adc_v2_clear_irq,
    .start_conv	= exynos_adc_v2_start_conv,
    };
    static const struct exynos_adc_data exynos3250_adc_data = {
    .num_channels	= MAX_EXYNOS3250_ADC_CHANNELS,
    .mask		= ADC_DATX_MASK, /* 12 bit ADC resolution */
    .needs_sclk	= true,
    .needs_adc_phy	= true,
    .phy_offset	= EXYNOS_ADCV1_PHY_OFFSET,
    .init_hw	= exynos_adc_v2_init_hw,
    .exit_hw	= exynos_adc_v2_exit_hw,
    .clear_irq	= exynos_adc_v2_clear_irq,
    .start_conv	= exynos_adc_v2_start_conv,
    };
#[no_mangle]
unsafe extern "C" fn exynos_adc_exynos7_init_hw(info: *mut exynos_adc) {
    static void exynos_adc_exynos7_init_hw(struct exynos_adc *info)
    {
    u32 con1, con2;
    con1 = ADC_V2_CON1_SOFT_RESET;
    writel(con1, ADC_V2_CON1(info.regs));
    con2 = readl(ADC_V2_CON2(info.regs));
    con2 &= ~ADC_V2_CON2_C_TIME(7);
    con2 |= ADC_V2_CON2_C_TIME(0);
    writel(con2, ADC_V2_CON2(info.regs));
// Enable interrupts
    writel(1, ADC_V2_INT_EN(info.regs));
    }
    static const struct exynos_adc_data exynos7_adc_data = {
    .num_channels	= MAX_ADC_V1_CHANNELS,
    .mask		= ADC_DATX_MASK, /* 12 bit ADC resolution */
    .init_hw	= exynos_adc_exynos7_init_hw,
    .exit_hw	= exynos_adc_v2_exit_hw,
    .clear_irq	= exynos_adc_v2_clear_irq,
    .start_conv	= exynos_adc_v2_start_conv,
    };
    static const struct of_device_id exynos_adc_match[] = {
    {
    .compatible = "samsung,s3c6410-adc",
    .data = &exynos_adc_s3c64xx_data,
    }, {
    .compatible = "samsung,s5pv210-adc",
    .data = &exynos_adc_s5pv210_data,
    }, {
    .compatible = "samsung,exynos4212-adc",
    .data = &exynos4212_adc_data,
    }, {
    .compatible = "samsung,exynos-adc-v1",
    .data = &exynos_adc_v1_data,
    }, {
    .compatible = "samsung,exynos-adc-v2",
    .data = &exynos_adc_v2_data,
    }, {
    .compatible = "samsung,exynos3250-adc",
    .data = &exynos3250_adc_data,
    }, {
    .compatible = "samsung,exynos7-adc",
    .data = &exynos7_adc_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, exynos_adc_match);
    static struct exynos_adc_data *exynos_adc_get_data(struct platform_device *pdev)
    {
    const struct of_device_id *match;
    match = of_match_node(exynos_adc_match, pdev.dev.of_node);
    return (struct exynos_adc_data *)match.data;
    }
    static int exynos_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct exynos_adc *info = iio_priv(indio_dev);
    unsigned long time_left;
    int ret;
    if (mask == IIO_CHAN_INFO_SCALE) {
    ret = regulator_get_voltage(info.vdd);
    if (ret < 0)
    return ret;
// Regulator voltage is in uV, but need mV
// val = ret / 1000;
// val2 = info->data->mask;
    return IIO_VAL_FRACTIONAL;
    } else if (mask != IIO_CHAN_INFO_RAW) {
    return -EINVAL;
    }
    mutex_lock(&info.lock);
    reinit_completion(&info.completion);
// Select the channel to be used and Trigger conversion
    if (info.data.start_conv)
    info.data.start_conv(info, chan.address);
    time_left = wait_for_completion_timeout(&info.completion,
    EXYNOS_ADC_TIMEOUT);
    if (time_left == 0) {
    dev_warn(&indio_dev.dev, "Conversion timed out! Resetting\n");
    if (info.data.init_hw)
    info.data.init_hw(info);
    ret = -ETIMEDOUT;
    } else {
// val = info->value;
// val2 = 0;
    ret = IIO_VAL_INT;
    }
    mutex_unlock(&info.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos_adc_isr(int irq, void *dev_id)
    {
    struct exynos_adc *info = dev_id;
    let mut mask: u32 = info.data.mask;
// Read value
    info.value = readl(ADC_V1_DATX(info.regs)) & mask;
// clear irq
    if (info.data.clear_irq)
    info.data.clear_irq(info);
    complete(&info.completion);
    return IRQ_HANDLED;
    }
    static int exynos_adc_reg_access(struct iio_dev *indio_dev,
    unsigned reg, unsigned writeval,
    unsigned *readval)
    {
    struct exynos_adc *info = iio_priv(indio_dev);
    if (readval == core::ptr::null_mut())
    return -EINVAL;
// readval = readl(info->regs + reg);
    return 0;
    }
    static const struct iio_info exynos_adc_iio_info = {
    .read_raw = &exynos_read_raw,
    .debugfs_reg_access = &exynos_adc_reg_access,
    };

    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = _index,				\
    .address = _index,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SCALE),	\
    .datasheet_name = _id,				\
    }
    static const struct iio_chan_spec exynos_adc_iio_channels[] = {
    ADC_CHANNEL(0, "adc0"),
    ADC_CHANNEL(1, "adc1"),
    ADC_CHANNEL(2, "adc2"),
    ADC_CHANNEL(3, "adc3"),
    ADC_CHANNEL(4, "adc4"),
    ADC_CHANNEL(5, "adc5"),
    ADC_CHANNEL(6, "adc6"),
    ADC_CHANNEL(7, "adc7"),
    ADC_CHANNEL(8, "adc8"),
    ADC_CHANNEL(9, "adc9"),
    };
#[no_mangle]
unsafe extern "C" fn exynos_adc_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_adc_probe(struct platform_device *pdev)
    {
    struct exynos_adc *info = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    struct device_node *np = pdev.dev.of_node;
    struct iio_dev *indio_dev = core::ptr::null_mut();
    int ret;
    int irq;
    indio_dev = devm_iio_device_alloc(dev, sizeof(struct exynos_adc));
    if (!indio_dev)
    return -ENOMEM;
    info = iio_priv(indio_dev);
    info.data = exynos_adc_get_data(pdev);
    if (!info.data)
    return dev_err_probe(dev, -EINVAL, "failed getting exynos_adc_data\n");
    info.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(info.regs))
    return PTR_ERR(info.regs);
    if (info.data.needs_adc_phy) {
    info.pmu_map = syscon_regmap_lookup_by_phandle(np, "samsung,syscon-phandle");
    if (IS_ERR(info.pmu_map))
    return dev_err_probe(dev, PTR_ERR(info.pmu_map),
    "syscon regmap lookup failed.\n");
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    info.irq = irq;
    info.dev = dev;
    init_completion(&info.completion);
    info.clk = devm_clk_get(dev, "adc");
    if (IS_ERR(info.clk))
    return dev_err_probe(dev, PTR_ERR(info.clk), "failed getting clock\n");
    if (info.data.needs_sclk) {
    info.sclk = devm_clk_get(dev, "sclk");
    if (IS_ERR(info.sclk))
    return dev_err_probe(dev, PTR_ERR(info.sclk),
    "failed getting sclk clock\n");
    }
    info.vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(info.vdd))
    return dev_err_probe(dev, PTR_ERR(info.vdd), "failed getting regulator");
    ret = regulator_enable(info.vdd);
    if (ret)
    return ret;
    ret = exynos_adc_prepare_clk(info);
    if (ret)
    goto err_disable_reg;
    ret = exynos_adc_enable_clk(info);
    if (ret)
    goto err_unprepare_clk;
    platform_set_drvdata(pdev, indio_dev);
    indio_dev.name = dev_name(dev);
    indio_dev.info = &exynos_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = exynos_adc_iio_channels;
    indio_dev.num_channels = info.data.num_channels;
    mutex_init(&info.lock);
    ret = request_irq(info.irq, exynos_adc_isr, 0, dev_name(dev), info);
    if (ret < 0) {
    dev_err(dev, "failed requesting irq, irq = %d\n", info.irq);
    goto err_disable_clk;
    }
    ret = iio_device_register(indio_dev);
    if (ret)
    goto err_irq;
    if (info.data.init_hw)
    info.data.init_hw(info);
    ret = of_platform_populate(np, exynos_adc_match, core::ptr::null_mut(), &indio_dev.dev);
    if (ret < 0) {
    dev_err(dev, "failed adding child nodes\n");
    goto err_of_populate;
    }
    return 0;
    err_of_populate:
    of_platform_depopulate(&indio_dev.dev);
    iio_device_unregister(indio_dev);
    err_irq:
    free_irq(info.irq, info);
    err_disable_clk:
    if (info.data.exit_hw)
    info.data.exit_hw(info);
    exynos_adc_disable_clk(info);
    err_unprepare_clk:
    exynos_adc_unprepare_clk(info);
    err_disable_reg:
    regulator_disable(info.vdd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_remove(pdev: *mut platform_device) {
    static void exynos_adc_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct exynos_adc *info = iio_priv(indio_dev);
    of_platform_depopulate(&indio_dev.dev);
    iio_device_unregister(indio_dev);
    free_irq(info.irq, info);
    if (info.data.exit_hw)
    info.data.exit_hw(info);
    exynos_adc_disable_clk(info);
    exynos_adc_unprepare_clk(info);
    regulator_disable(info.vdd);
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_suspend(dev: *mut device) -> c_int {
    static int exynos_adc_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct exynos_adc *info = iio_priv(indio_dev);
    if (info.data.exit_hw)
    info.data.exit_hw(info);
    exynos_adc_disable_clk(info);
    regulator_disable(info.vdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_adc_resume(dev: *mut device) -> c_int {
    static int exynos_adc_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct exynos_adc *info = iio_priv(indio_dev);
    int ret;
    ret = regulator_enable(info.vdd);
    if (ret)
    return ret;
    ret = exynos_adc_enable_clk(info);
    if (ret)
    return ret;
    if (info.data.init_hw)
    info.data.init_hw(info);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(exynos_adc_pm_ops, exynos_adc_suspend,
    exynos_adc_resume);
    static struct platform_driver exynos_adc_driver = {
    .probe		= exynos_adc_probe,
    .remove		= exynos_adc_remove,
    .driver		= {
    .name	= "exynos-adc",
    .of_match_table = exynos_adc_match,
    .pm	= pm_sleep_ptr(&exynos_adc_pm_ops),
    },
    };
    module_platform_driver(exynos_adc_driver);
    MODULE_AUTHOR("Naveen Krishna Chatradhi <ch.naveen@samsung.com>");
    MODULE_DESCRIPTION("Samsung EXYNOS5 ADC driver");
    MODULE_LICENSE("GPL v2");
