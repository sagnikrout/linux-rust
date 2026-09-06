//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/imx93_adc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// NXP i.MX93 ADC driver
//
// Copyright 2023 NXP
//

// Register map definition
pub const IMX93_ADC_MCR: c_uint = 0x00;
pub const IMX93_ADC_MSR: c_uint = 0x04;
pub const IMX93_ADC_ISR: c_uint = 0x10;
pub const IMX93_ADC_IMR: c_uint = 0x20;
pub const IMX93_ADC_CIMR0: c_uint = 0x24;
pub const IMX93_ADC_CTR0: c_uint = 0x94;
pub const IMX93_ADC_NCMR0: c_uint = 0xA4;
pub const IMX93_ADC_PCDR0: c_uint = 0x100;
pub const IMX93_ADC_PCDR1: c_uint = 0x104;
pub const IMX93_ADC_PCDR2: c_uint = 0x108;
pub const IMX93_ADC_PCDR3: c_uint = 0x10C;
pub const IMX93_ADC_PCDR4: c_uint = 0x110;
pub const IMX93_ADC_PCDR5: c_uint = 0x114;
pub const IMX93_ADC_PCDR6: c_uint = 0x118;
pub const IMX93_ADC_PCDR7: c_uint = 0x11C;
pub const IMX93_ADC_CALSTAT: c_uint = 0x39C;
pub const IMX93_ADC_CALCFG0: c_uint = 0x3A0;
// ADC bit shift

    IMX93_ADC_ISR_ECH_MASK)

// ADC status
pub const IMX93_ADC_MSR_ADCSTATUS_IDLE: c_int = 0;
pub const IMX93_ADC_MSR_ADCSTATUS_POWER_DOWN: c_int = 1;
pub const IMX93_ADC_MSR_ADCSTATUS_WAIT_STATE: c_int = 2;
pub const IMX93_ADC_MSR_ADCSTATUS_BUSY_IN_CALIBRATION: c_int = 3;
pub const IMX93_ADC_MSR_ADCSTATUS_SAMPLE: c_int = 4;
pub const IMX93_ADC_MSR_ADCSTATUS_CONVERSION: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_adc {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub ipg_clk: *mut clk,
    pub irq: c_int,
    pub vref: *mut regulator,
// lock to protect against multiple access to the device
    pub lock: mutex,
    pub completion: completion,
}

    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = (_idx),					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),	\
    }
    static const struct iio_chan_spec imx93_adc_iio_channels[] = {
    IMX93_ADC_CHAN(0),
    IMX93_ADC_CHAN(1),
    IMX93_ADC_CHAN(2),
    IMX93_ADC_CHAN(3),
    IMX93_ADC_CHAN(4),
    IMX93_ADC_CHAN(5),
    IMX93_ADC_CHAN(6),
    IMX93_ADC_CHAN(7),
    };
#[no_mangle]
unsafe extern "C" fn imx93_adc_power_down(adc: *mut imx93_adc) {
    static void imx93_adc_power_down(struct imx93_adc *adc)
    {
    u32 mcr, msr;
    int ret;
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr |= FIELD_PREP(IMX93_ADC_MCR_PWDN_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
    ret = readl_poll_timeout(adc.regs + IMX93_ADC_MSR, msr,
    ((msr & IMX93_ADC_MSR_ADCSTATUS_MASK) ==
    IMX93_ADC_MSR_ADCSTATUS_POWER_DOWN),
    1, 50);
    if (ret == -ETIMEDOUT)
    dev_warn(adc.dev,
    "ADC do not in power down mode, current MSR is %x\n",
    msr);
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_power_up(adc: *mut imx93_adc) {
    static void imx93_adc_power_up(struct imx93_adc *adc)
    {
    u32 mcr;
// bring ADC out of power down state, in idle state
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr &= ~FIELD_PREP(IMX93_ADC_MCR_PWDN_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_config_ad_clk(adc: *mut imx93_adc) {
    static void imx93_adc_config_ad_clk(struct imx93_adc *adc)
    {
    u32 mcr;
// put adc in power down mode
    imx93_adc_power_down(adc);
// config the AD_CLK equal to bus clock
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr |= FIELD_PREP(IMX93_ADC_MCR_ADCLKSE_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
    imx93_adc_power_up(adc);
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_calibration(adc: *mut imx93_adc) -> c_int {
    static int imx93_adc_calibration(struct imx93_adc *adc)
    {
    u32 mcr, msr, calcfg;
    int ret;
// make sure ADC in power down mode
    imx93_adc_power_down(adc);
// config SAR controller operating clock
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr &= ~FIELD_PREP(IMX93_ADC_MCR_ADCLKSE_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
    imx93_adc_power_up(adc);
// Enable loading of calibrated values even in fail condition
    calcfg = readl(adc.regs + IMX93_ADC_CALCFG0);
    calcfg |= IMX93_ADC_CALCFG0_LDFAIL_MASK;
    writel(calcfg, adc.regs + IMX93_ADC_CALCFG0);
//
// TODO: we use the default TSAMP/NRSMPL/AVGEN in MCR,
// can add the setting of these bit if need in future.
//
// run calibration
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr |= FIELD_PREP(IMX93_ADC_MCR_CALSTART_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
// wait calibration to be finished
    ret = readl_poll_timeout(adc.regs + IMX93_ADC_MSR, msr,
    !(msr & IMX93_ADC_MSR_CALBUSY_MASK), 1000, 2000000);
    if (ret == -ETIMEDOUT) {
    dev_warn(adc.dev, "ADC do not finish calibration in 2 min!\n");
    imx93_adc_power_down(adc);
    return ret;
    }
// check whether calbration is success or not
    msr = readl(adc.regs + IMX93_ADC_MSR);
    if (msr & IMX93_ADC_MSR_CALFAIL_MASK) {
//
// Only give warning here, this means the noise of the
// reference voltage do not meet the requirement:
// ADC reference voltage Noise < 1.8V * 1/2^ENOB
// And the resault of ADC is not that accurate.
//
    dev_warn(adc.dev, "ADC calibration failed!\n");
    }
    return 0;
    }
    static int imx93_adc_read_channel_conversion(struct imx93_adc *adc,
    int channel_number,
    int *result)
    {
    u32 channel;
    u32 imr, mcr, pcda;
    long ret;
    reinit_completion(&adc.completion);
// config channel mask register
    channel = 1 << channel_number;
    writel(channel, adc.regs + IMX93_ADC_NCMR0);
// TODO: can config desired sample time in CTRn if need
// config interrupt mask
    imr = FIELD_PREP(IMX93_ADC_IMR_EOC_MASK, 1);
    writel(imr, adc.regs + IMX93_ADC_IMR);
    writel(channel, adc.regs + IMX93_ADC_CIMR0);
// config one-shot mode
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr &= ~FIELD_PREP(IMX93_ADC_MCR_MODE_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
// start normal conversion
    mcr = readl(adc.regs + IMX93_ADC_MCR);
    mcr |= FIELD_PREP(IMX93_ADC_MCR_NSTART_MASK, 1);
    writel(mcr, adc.regs + IMX93_ADC_MCR);
    ret = wait_for_completion_interruptible_timeout(&adc.completion,
    IMX93_ADC_TIMEOUT);
    if (ret == 0)
    return -ETIMEDOUT;
    if (ret < 0)
    return ret;
    pcda = readl(adc.regs + IMX93_ADC_PCDR0 + channel_number * 4);
// result = FIELD_GET(IMX93_ADC_PCDR_CDATA_MASK, pcda);
    return ret;
    }
    static int imx93_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct imx93_adc *adc = iio_priv(indio_dev);
    struct device *dev = adc.dev;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    pm_runtime_get_sync(dev);
    mutex_lock(&adc.lock);
    ret = imx93_adc_read_channel_conversion(adc, chan.channel, val);
    mutex_unlock(&adc.lock);
    pm_runtime_put_sync_autosuspend(dev);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = regulator_get_voltage(adc.vref);
    if (ret < 0)
    return ret;
// val = ret / 1000;
// val2 = 12;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = clk_get_rate(adc->ipg_clk);
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx93_adc_isr(int irq, void *dev_id)
    {
    struct imx93_adc *adc = dev_id;
    u32 isr, eoc, unexpected;
    isr = readl(adc.regs + IMX93_ADC_ISR);
    if (FIELD_GET(IMX93_ADC_ISR_EOC_ECH_MASK, isr)) {
    eoc = isr & IMX93_ADC_ISR_EOC_ECH_MASK;
    writel(eoc, adc.regs + IMX93_ADC_ISR);
    complete(&adc.completion);
    }
    unexpected = isr & ~IMX93_ADC_ISR_EOC_ECH_MASK;
    if (unexpected) {
    writel(unexpected, adc.regs + IMX93_ADC_ISR);
    dev_err(adc.dev, "Unexpected interrupt 0x%08x.\n", unexpected);
    return IRQ_NONE;
    }
    return IRQ_HANDLED;
    }
    static const struct iio_info imx93_adc_iio_info = {
    .read_raw = &imx93_adc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn imx93_adc_probe(pdev: *mut platform_device) -> c_int {
    static int imx93_adc_probe(struct platform_device *pdev)
    {
    struct imx93_adc *adc;
    struct iio_dev *indio_dev;
    struct device *dev = &pdev.dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*adc));
    if (!indio_dev)
    return -ENOMEM;
    adc = iio_priv(indio_dev);
    adc.dev = dev;
    mutex_init(&adc.lock);
    adc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(adc.regs))
    return dev_err_probe(dev, PTR_ERR(adc.regs),
    "Failed getting ioremap resource\n");
// The third irq is for ADC conversion usage
    adc.irq = platform_get_irq(pdev, 2);
    if (adc.irq < 0)
    return adc.irq;
    adc.ipg_clk = devm_clk_get(dev, "ipg");
    if (IS_ERR(adc.ipg_clk))
    return dev_err_probe(dev, PTR_ERR(adc.ipg_clk),
    "Failed getting clock.\n");
    adc.vref = devm_regulator_get(dev, "vref");
    if (IS_ERR(adc.vref))
    return dev_err_probe(dev, PTR_ERR(adc.vref),
    "Failed getting reference voltage.\n");
    ret = regulator_enable(adc.vref);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to enable reference voltage.\n");
    platform_set_drvdata(pdev, indio_dev);
    init_completion(&adc.completion);
    indio_dev.name = "imx93-adc";
    indio_dev.info = &imx93_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = imx93_adc_iio_channels;
    indio_dev.num_channels = ARRAY_SIZE(imx93_adc_iio_channels);
    ret = clk_prepare_enable(adc.ipg_clk);
    if (ret) {
    dev_err_probe(dev, ret,
    "Failed to enable ipg clock.\n");
    goto error_regulator_disable;
    }
    ret = request_irq(adc.irq, imx93_adc_isr, 0, IMX93_ADC_DRIVER_NAME, adc);
    if (ret < 0) {
    dev_err_probe(dev, ret,
    "Failed requesting irq, irq = %d\n", adc.irq);
    goto error_ipg_clk_disable;
    }
    ret = imx93_adc_calibration(adc);
    if (ret < 0)
    goto error_free_adc_irq;
    imx93_adc_config_ad_clk(adc);
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err_probe(dev, ret,
    "Failed to register this iio device.\n");
    goto error_adc_power_down;
    }
    pm_runtime_set_active(dev);
    pm_runtime_set_autosuspend_delay(dev, 50);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_enable(dev);
    return 0;
    error_adc_power_down:
    imx93_adc_power_down(adc);
    error_free_adc_irq:
    free_irq(adc.irq, adc);
    error_ipg_clk_disable:
    clk_disable_unprepare(adc.ipg_clk);
    error_regulator_disable:
    regulator_disable(adc.vref);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_remove(pdev: *mut platform_device) {
    static void imx93_adc_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct imx93_adc *adc = iio_priv(indio_dev);
    struct device *dev = adc.dev;
// adc power down need clock on
    pm_runtime_get_sync(dev);
    pm_runtime_disable(dev);
    pm_runtime_dont_use_autosuspend(dev);
    pm_runtime_put_noidle(dev);
    iio_device_unregister(indio_dev);
    imx93_adc_power_down(adc);
    free_irq(adc.irq, adc);
    clk_disable_unprepare(adc.ipg_clk);
    regulator_disable(adc.vref);
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_runtime_suspend(dev: *mut device) -> c_int {
    static int imx93_adc_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct imx93_adc *adc = iio_priv(indio_dev);
    imx93_adc_power_down(adc);
    clk_disable_unprepare(adc.ipg_clk);
    regulator_disable(adc.vref);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_adc_runtime_resume(dev: *mut device) -> c_int {
    static int imx93_adc_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct imx93_adc *adc = iio_priv(indio_dev);
    int ret;
    ret = regulator_enable(adc.vref);
    if (ret) {
    dev_err(dev,
    "Can't enable adc reference top voltage, err = %d\n",
    ret);
    return ret;
    }
    ret = clk_prepare_enable(adc.ipg_clk);
    if (ret) {
    dev_err(dev, "Could not prepare or enable clock.\n");
    goto err_disable_reg;
    }
    imx93_adc_power_up(adc);
    return 0;
    err_disable_reg:
    regulator_disable(adc.vref);
    return ret;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(imx93_adc_pm_ops,
    imx93_adc_runtime_suspend,
    imx93_adc_runtime_resume, core::ptr::null_mut());
    static const struct of_device_id imx93_adc_match[] = {
    { .compatible = "nxp,imx93-adc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx93_adc_match);
    static struct platform_driver imx93_adc_driver = {
    .probe		= imx93_adc_probe,
    .remove		= imx93_adc_remove,
    .driver		= {
    .name	= IMX93_ADC_DRIVER_NAME,
    .of_match_table = imx93_adc_match,
    .pm	= pm_ptr(&imx93_adc_pm_ops),
    },
    };
    module_platform_driver(imx93_adc_driver);
    MODULE_DESCRIPTION("NXP i.MX93 ADC driver");
    MODULE_AUTHOR("Haibo Chen <haibo.chen@nxp.com>");
    MODULE_LICENSE("GPL");
