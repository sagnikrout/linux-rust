//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/imx6ul_tsc.c
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
// Freescale i.MX6UL touchscreen controller driver
//
// Copyright (C) 2015 Freescale Semiconductor, Inc.

// ADC configuration registers field define

pub const ADC_CONV_DISABLE: c_uint = 0x1F;

pub const ADC_12BIT_MODE: c_uint = 0x2;
pub const ADC_IPG_CLK: c_uint = 0x00;

pub const ADC_CLK_DIV_8: c_uint = 0x03;

pub const SELECT_CHANNEL_4: c_uint = 0x04;
pub const SELECT_CHANNEL_1: c_uint = 0x01;
// ADC registers
pub const REG_ADC_HC0: c_uint = 0x00;
pub const REG_ADC_HC1: c_uint = 0x04;
pub const REG_ADC_HC2: c_uint = 0x08;
pub const REG_ADC_HC3: c_uint = 0x0C;
pub const REG_ADC_HC4: c_uint = 0x10;
pub const REG_ADC_HS: c_uint = 0x14;
pub const REG_ADC_R0: c_uint = 0x18;
pub const REG_ADC_CFG: c_uint = 0x2C;
pub const REG_ADC_GC: c_uint = 0x30;
pub const REG_ADC_GS: c_uint = 0x34;

// TSC registers
pub const REG_TSC_BASIC_SETTING: c_uint = 0x00;
pub const REG_TSC_PRE_CHARGE_TIME: c_uint = 0x10;
pub const REG_TSC_FLOW_CONTROL: c_uint = 0x20;
pub const REG_TSC_MEASURE_VALUE: c_uint = 0x30;
pub const REG_TSC_INT_EN: c_uint = 0x40;
pub const REG_TSC_INT_SIG_EN: c_uint = 0x50;
pub const REG_TSC_INT_STATUS: c_uint = 0x60;
pub const REG_TSC_DEBUG_MODE: c_uint = 0x70;
pub const REG_TSC_DEBUG_MODE2: c_uint = 0x80;
// TSC_MEASURE_VALUE register field define

// TSC configuration registers field define

pub const DE_GLITCH_DEF: c_uint = 0x02;

pub const DETECT_MODE: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx6ul_tsc {
    pub dev: *mut device,
    pub input: *mut input_dev,
    pub tsc_regs: *mut void __iomem,
    pub adc_regs: *mut void __iomem,
    pub tsc_clk: *mut clk,
    pub adc_clk: *mut clk,
    pub xnur_gpio: *mut gpio_desc,
    pub measure_delay_time: u32,
    pub pre_charge_time: u32,
    pub average_enable: bool,
    pub average_select: u32,
    pub de_glitch: u32,
    pub completion: completion,
}

//
// TSC module need ADC to get the measure value. So
// before config TSC, we should initialize ADC module.
//
#[no_mangle]
unsafe extern "C" fn imx6ul_adc_init(tsc: *mut imx6ul_tsc) -> c_int {
    static int imx6ul_adc_init(struct imx6ul_tsc *tsc)
    {
    let mut adc_hc: u32 = 0;
    u32 adc_gc;
    u32 adc_gs;
    u32 adc_cfg;
    unsigned long timeout;
    reinit_completion(&tsc.completion);
    adc_cfg = readl(tsc.adc_regs + REG_ADC_CFG);
    adc_cfg &= ~(ADC_CONV_MODE_MASK | ADC_INPUT_CLK_MASK);
    adc_cfg |= FIELD_PREP(ADC_CONV_MODE_MASK, ADC_12BIT_MODE) |
    FIELD_PREP(ADC_INPUT_CLK_MASK, ADC_IPG_CLK);
    adc_cfg &= ~(ADC_CLK_DIV_MASK | ADC_SAMPLE_MODE);
    adc_cfg |= FIELD_PREP(ADC_CLK_DIV_MASK, ADC_CLK_DIV_8);
    if (tsc.average_enable) {
    adc_cfg &= ~ADC_AVGS_MASK;
    adc_cfg |= FIELD_PREP(ADC_AVGS_MASK, tsc.average_select);
    }
    adc_cfg &= ~ADC_HARDWARE_TRIGGER;
    writel(adc_cfg, tsc.adc_regs + REG_ADC_CFG);
// enable calibration interrupt
    adc_hc |= ADC_AIEN;
    adc_hc |= FIELD_PREP(ADC_ADCH_MASK, ADC_CONV_DISABLE);
    writel(adc_hc, tsc.adc_regs + REG_ADC_HC0);
// start ADC calibration
    adc_gc = readl(tsc.adc_regs + REG_ADC_GC);
    adc_gc |= ADC_CAL;
    if (tsc.average_enable)
    adc_gc |= ADC_AVGE;
    writel(adc_gc, tsc.adc_regs + REG_ADC_GC);
    timeout = wait_for_completion_timeout
    (&tsc.completion, ADC_TIMEOUT);
    if (timeout == 0) {
    dev_err(tsc.dev, "Timeout for adc calibration\n");
    return -ETIMEDOUT;
    }
    adc_gs = readl(tsc.adc_regs + REG_ADC_GS);
    if (adc_gs & ADC_CALF) {
    dev_err(tsc.dev, "ADC calibration failed\n");
    return -EINVAL;
    }
// TSC need the ADC work in hardware trigger
    adc_cfg = readl(tsc.adc_regs + REG_ADC_CFG);
    adc_cfg |= ADC_HARDWARE_TRIGGER;
    writel(adc_cfg, tsc.adc_regs + REG_ADC_CFG);
    return 0;
    }
//
// This is a TSC workaround. Currently TSC misconnect two
// ADC channels, this function remap channel configure for
// hardware trigger.
//
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_channel_config(tsc: *mut imx6ul_tsc) {
    static void imx6ul_tsc_channel_config(struct imx6ul_tsc *tsc)
    {
    u32 adc_hc0, adc_hc1, adc_hc2, adc_hc3, adc_hc4;
    adc_hc0 = FIELD_PREP(ADC_AIEN, 0);
    writel(adc_hc0, tsc.adc_regs + REG_ADC_HC0);
    adc_hc1 = FIELD_PREP(ADC_AIEN, 0) |
    FIELD_PREP(ADC_ADCH_MASK, SELECT_CHANNEL_4);
    writel(adc_hc1, tsc.adc_regs + REG_ADC_HC1);
    adc_hc2 = FIELD_PREP(ADC_AIEN, 0);
    writel(adc_hc2, tsc.adc_regs + REG_ADC_HC2);
    adc_hc3 = FIELD_PREP(ADC_AIEN, 0) |
    FIELD_PREP(ADC_ADCH_MASK, SELECT_CHANNEL_1);
    writel(adc_hc3, tsc.adc_regs + REG_ADC_HC3);
    adc_hc4 = FIELD_PREP(ADC_AIEN, 0);
    writel(adc_hc4, tsc.adc_regs + REG_ADC_HC4);
    }
//
// TSC setting, confige the pre-charge time and measure delay time.
// different touch screen may need different pre-charge time and
// measure delay time.
//
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_set(tsc: *mut imx6ul_tsc) {
    static void imx6ul_tsc_set(struct imx6ul_tsc *tsc)
    {
    let mut basic_setting: u32 = 0;
    u32 debug_mode2;
    u32 start;
    basic_setting |= FIELD_PREP(MEASURE_DELAY_TIME_MASK,
    tsc.measure_delay_time);
    basic_setting |= AUTO_MEASURE;
    writel(basic_setting, tsc.tsc_regs + REG_TSC_BASIC_SETTING);
    debug_mode2 = FIELD_PREP(DE_GLITCH_MASK, tsc.de_glitch);
    writel(debug_mode2, tsc.tsc_regs + REG_TSC_DEBUG_MODE2);
    writel(tsc.pre_charge_time, tsc.tsc_regs + REG_TSC_PRE_CHARGE_TIME);
    writel(MEASURE_INT_EN, tsc.tsc_regs + REG_TSC_INT_EN);
    writel(MEASURE_SIG_EN | VALID_SIG_EN,
    tsc.tsc_regs + REG_TSC_INT_SIG_EN);
// start sense detection
    start = readl(tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
    start |= START_SENSE;
    start &= ~TSC_DISABLE;
    writel(start, tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_init(tsc: *mut imx6ul_tsc) -> c_int {
    static int imx6ul_tsc_init(struct imx6ul_tsc *tsc)
    {
    int err;
    err = imx6ul_adc_init(tsc);
    if (err)
    return err;
    imx6ul_tsc_channel_config(tsc);
    imx6ul_tsc_set(tsc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_disable(tsc: *mut imx6ul_tsc) {
    static void imx6ul_tsc_disable(struct imx6ul_tsc *tsc)
    {
    u32 tsc_flow;
    u32 adc_cfg;
// TSC controller enters to idle status
    tsc_flow = readl(tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
    tsc_flow |= TSC_DISABLE;
    writel(tsc_flow, tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
// ADC controller enters to stop mode
    adc_cfg = readl(tsc.adc_regs + REG_ADC_HC0);
    adc_cfg |= ADC_CONV_DISABLE;
    writel(adc_cfg, tsc.adc_regs + REG_ADC_HC0);
    }
// Delay some time (max 2ms), wait the pre-charge done.
#[no_mangle]
unsafe extern "C" fn tsc_wait_detect_mode(tsc: *mut imx6ul_tsc) -> bool {
    static bool tsc_wait_detect_mode(struct imx6ul_tsc *tsc)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(2);
    u32 state_machine;
    u32 debug_mode2;
    do {
    if (time_after(jiffies, timeout))
    return false;
    usleep_range(200, 400);
    debug_mode2 = readl(tsc.tsc_regs + REG_TSC_DEBUG_MODE2);
    state_machine = FIELD_GET(STATE_MACHINE_MASK, debug_mode2);
    } while (state_machine != DETECT_MODE);
    usleep_range(200, 400);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tsc_irq_fn(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tsc_irq_fn(int irq, void *dev_id)
    {
    struct imx6ul_tsc *tsc = dev_id;
    u32 status;
    u32 value;
    u32 x, y;
    u32 start;
    status = readl(tsc.tsc_regs + REG_TSC_INT_STATUS);
// write 1 to clear the bit measure-signal
    writel(MEASURE_SIGNAL | DETECT_SIGNAL,
    tsc.tsc_regs + REG_TSC_INT_STATUS);
// It's a HW self-clean bit. Set this bit and start sense detection
    start = readl(tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
    start |= START_SENSE;
    writel(start, tsc.tsc_regs + REG_TSC_FLOW_CONTROL);
    if (status & MEASURE_SIGNAL) {
    value = readl(tsc.tsc_regs + REG_TSC_MEASURE_VALUE);
    x = FIELD_GET(X_VALUE_MASK, value);
    y = FIELD_GET(Y_VALUE_MASK, value);
//
// In detect mode, we can get the xnur gpio value,
// otherwise assume contact is stiull active.
//
    if (!tsc_wait_detect_mode(tsc) ||
    gpiod_get_value_cansleep(tsc.xnur_gpio)) {
    input_report_key(tsc.input, BTN_TOUCH, 1);
    input_report_abs(tsc.input, ABS_X, x);
    input_report_abs(tsc.input, ABS_Y, y);
    } else {
    input_report_key(tsc.input, BTN_TOUCH, 0);
    }
    input_sync(tsc.input);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adc_irq_fn(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t adc_irq_fn(int irq, void *dev_id)
    {
    struct imx6ul_tsc *tsc = dev_id;
    u32 coco;
    coco = readl(tsc.adc_regs + REG_ADC_HS);
    if (coco & 0x01) {
    readl(tsc.adc_regs + REG_ADC_R0);
    complete(&tsc.completion);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_start(tsc: *mut imx6ul_tsc) -> c_int {
    static int imx6ul_tsc_start(struct imx6ul_tsc *tsc)
    {
    int err;
    err = clk_prepare_enable(tsc.adc_clk);
    if (err) {
    dev_err(tsc.dev,
    "Could not prepare or enable the adc clock: %d\n",
    err);
    return err;
    }
    err = clk_prepare_enable(tsc.tsc_clk);
    if (err) {
    dev_err(tsc.dev,
    "Could not prepare or enable the tsc clock: %d\n",
    err);
    goto disable_adc_clk;
    }
    err = imx6ul_tsc_init(tsc);
    if (err)
    goto disable_tsc_clk;
    return 0;
    disable_tsc_clk:
    clk_disable_unprepare(tsc.tsc_clk);
    disable_adc_clk:
    clk_disable_unprepare(tsc.adc_clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_stop(tsc: *mut imx6ul_tsc) {
    static void imx6ul_tsc_stop(struct imx6ul_tsc *tsc)
    {
    imx6ul_tsc_disable(tsc);
    clk_disable_unprepare(tsc.tsc_clk);
    clk_disable_unprepare(tsc.adc_clk);
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_open(input_dev: *mut input_dev) -> c_int {
    static int imx6ul_tsc_open(struct input_dev *input_dev)
    {
    struct imx6ul_tsc *tsc = input_get_drvdata(input_dev);
    return imx6ul_tsc_start(tsc);
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_close(input_dev: *mut input_dev) {
    static void imx6ul_tsc_close(struct input_dev *input_dev)
    {
    struct imx6ul_tsc *tsc = input_get_drvdata(input_dev);
    imx6ul_tsc_stop(tsc);
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_probe(pdev: *mut platform_device) -> c_int {
    static int imx6ul_tsc_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct imx6ul_tsc *tsc;
    struct input_dev *input_dev;
    int err;
    int tsc_irq;
    int adc_irq;
    u32 average_samples;
    u32 de_glitch;
    tsc = devm_kzalloc(&pdev.dev, sizeof(*tsc), GFP_KERNEL);
    if (!tsc)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = "iMX6UL Touchscreen Controller";
    input_dev.id.bustype = BUS_HOST;
    input_dev.open = imx6ul_tsc_open;
    input_dev.close = imx6ul_tsc_close;
    input_set_capability(input_dev, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input_dev, ABS_X, 0, 0xFFF, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 0xFFF, 0, 0);
    input_set_drvdata(input_dev, tsc);
    tsc.dev = &pdev.dev;
    tsc.input = input_dev;
    init_completion(&tsc.completion);
    tsc.xnur_gpio = devm_gpiod_get(&pdev.dev, "xnur", GPIOD_IN);
    if (IS_ERR(tsc.xnur_gpio)) {
    err = PTR_ERR(tsc.xnur_gpio);
    dev_err(&pdev.dev,
    "failed to request GPIO tsc_X- (xnur): %d\n", err);
    return err;
    }
    tsc.tsc_regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tsc.tsc_regs)) {
    err = PTR_ERR(tsc.tsc_regs);
    dev_err(&pdev.dev, "failed to remap tsc memory: %d\n", err);
    return err;
    }
    tsc.adc_regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(tsc.adc_regs)) {
    err = PTR_ERR(tsc.adc_regs);
    dev_err(&pdev.dev, "failed to remap adc memory: %d\n", err);
    return err;
    }
    tsc.tsc_clk = devm_clk_get(&pdev.dev, "tsc");
    if (IS_ERR(tsc.tsc_clk)) {
    err = PTR_ERR(tsc.tsc_clk);
    dev_err(&pdev.dev, "failed getting tsc clock: %d\n", err);
    return err;
    }
    tsc.adc_clk = devm_clk_get(&pdev.dev, "adc");
    if (IS_ERR(tsc.adc_clk)) {
    err = PTR_ERR(tsc.adc_clk);
    dev_err(&pdev.dev, "failed getting adc clock: %d\n", err);
    return err;
    }
    tsc_irq = platform_get_irq(pdev, 0);
    if (tsc_irq < 0)
    return tsc_irq;
    adc_irq = platform_get_irq(pdev, 1);
    if (adc_irq < 0)
    return adc_irq;
    err = devm_request_threaded_irq(tsc.dev, tsc_irq,
    core::ptr::null_mut(), tsc_irq_fn, IRQF_ONESHOT,
    dev_name(&pdev.dev), tsc);
    if (err) {
    dev_err(&pdev.dev,
    "failed requesting tsc irq %d: %d\n",
    tsc_irq, err);
    return err;
    }
    err = devm_request_irq(tsc.dev, adc_irq, adc_irq_fn, 0,
    dev_name(&pdev.dev), tsc);
    if (err) {
    dev_err(&pdev.dev,
    "failed requesting adc irq %d: %d\n",
    adc_irq, err);
    return err;
    }
    err = of_property_read_u32(np, "measure-delay-time",
    &tsc.measure_delay_time);
    if (err)
    tsc.measure_delay_time = 0xffff;
    err = of_property_read_u32(np, "pre-charge-time",
    &tsc.pre_charge_time);
    if (err)
    tsc.pre_charge_time = 0xfff;
    err = of_property_read_u32(np, "touchscreen-average-samples",
    &average_samples);
    if (err)
    average_samples = 1;
    switch (average_samples) {
    case 1:
    tsc.average_enable = false;
    tsc.average_select = 0; /* value unused; initialize anyway */
    break;
    case 4:
    case 8:
    case 16:
    case 32:
    tsc.average_enable = true;
    tsc.average_select = ilog2(average_samples) - 2;
    break;
    default:
    dev_err(&pdev.dev,
    "touchscreen-average-samples (%u) must be 1, 4, 8, 16 or 32\n",
    average_samples);
    return -EINVAL;
    }
    err = of_property_read_u32(np, "debounce-delay-us", &de_glitch);
    if (err) {
    tsc.de_glitch = DE_GLITCH_DEF;
    } else {
    u64 cycles;
    let mut rate: c_ulong = clk_get_rate(tsc.tsc_clk);
    cycles = DIV64_U64_ROUND_UP((u64)de_glitch * rate, USEC_PER_SEC);
    if (cycles <= 0x3ff)
    tsc.de_glitch = 3;
#[no_mangle]
pub unsafe extern "C" fn if(0x7ff: cycles <=) -> else {
    else if (cycles <= 0x7ff)
    tsc.de_glitch = 2;
#[no_mangle]
pub unsafe extern "C" fn if(0xfff: cycles <=) -> else {
    else if (cycles <= 0xfff)
    tsc.de_glitch = 1;
    else
    tsc.de_glitch = 0;
    }
    err = input_register_device(tsc.input);
    if (err) {
    dev_err(&pdev.dev,
    "failed to register input device: %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, tsc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_suspend(dev: *mut device) -> c_int {
    static int imx6ul_tsc_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct imx6ul_tsc *tsc = platform_get_drvdata(pdev);
    struct input_dev *input_dev = tsc.input;
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev))
    imx6ul_tsc_stop(tsc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx6ul_tsc_resume(dev: *mut device) -> c_int {
    static int imx6ul_tsc_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct imx6ul_tsc *tsc = platform_get_drvdata(pdev);
    struct input_dev *input_dev = tsc.input;
    int error;
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev)) {
    error = imx6ul_tsc_start(tsc);
    if (error)
    return error;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(imx6ul_tsc_pm_ops,
    imx6ul_tsc_suspend, imx6ul_tsc_resume);
    static const struct of_device_id imx6ul_tsc_match[] = {
    { .compatible = "fsl,imx6ul-tsc", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx6ul_tsc_match);
    static struct platform_driver imx6ul_tsc_driver = {
    .driver		= {
    .name	= "imx6ul-tsc",
    .of_match_table	= imx6ul_tsc_match,
    .pm	= pm_sleep_ptr(&imx6ul_tsc_pm_ops),
    },
    .probe		= imx6ul_tsc_probe,
    };
    module_platform_driver(imx6ul_tsc_driver);
    MODULE_AUTHOR("Haibo Chen <haibo.chen@freescale.com>");
    MODULE_DESCRIPTION("Freescale i.MX6UL Touchscreen controller driver");
    MODULE_LICENSE("GPL v2");
