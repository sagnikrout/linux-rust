//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/sun4i-ts.c
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
// Allwinner sunxi resistive touchscreen controller driver
//
// Copyright (C) 2013 - 2014 Hans de Goede <hdegoede@redhat.com>
//
// The hwmon parts are based on work by Corentin LABBE which is:
// Copyright (C) 2013 Corentin LABBE <clabbe.montjoie@gmail.com>
//
// The sun4i-ts controller is capable of detecting a second touch, but when a
// second touch is present then the accuracy becomes so bad the reported touch
// location is not useable.
//
// The original android driver contains some complicated heuristics using the
// aprox. distance between the 2 touches to see if the user is making a pinch
// open / close movement, and then reports emulated multi-touch events around
// the last touch coordinate (as the dual-touch coordinates are worthless).
//
// These kinds of heuristics are just asking for trouble (and don't belong
// in the kernel). So this driver offers straight forward, reliable single
// touch functionality only.
//
// s.a. A20 User Manual "1.15 TP" (Documentation/arch/arm/sunxi.rst)
// (looks like the description in the A20 User Manual v1.3 is better
// than the one in the A10 User Manual v.1.5)
//

pub const TP_CTRL0: c_uint = 0x00;
pub const TP_CTRL1: c_uint = 0x04;
pub const TP_CTRL2: c_uint = 0x08;
pub const TP_CTRL3: c_uint = 0x0c;
pub const TP_INT_FIFOC: c_uint = 0x10;
pub const TP_INT_FIFOS: c_uint = 0x14;
pub const TP_TPR: c_uint = 0x18;
pub const TP_CDAT: c_uint = 0x1c;
pub const TEMP_DATA: c_uint = 0x20;
pub const TP_DATA: c_uint = 0x24;
// TP_CTRL0 bits

// TP_CTRL1 bits

// on sun6i, bits 3~6 are left shifted by 1 to 4~7

// TP_CTRL2 bits

// TP_CTRL3 bits

// TP_INT_FIFOC irq and fifo mask / control bits

// TP_INT_FIFOS irq and fifo status bits

// TP_TPR bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_ts_data {
    pub dev: *mut device,
    pub input: *mut input_dev,
    pub base: *mut void __iomem,
    pub irq: c_uint,
    pub ignore_fifo_data: bool,
    pub temp_data: c_int,
    pub temp_offset: c_int,
    pub temp_step: c_int,
}

#[no_mangle]
unsafe extern "C" fn sun4i_ts_irq_handle_input(ts: *mut sun4i_ts_data, reg_val: u32) {
    static void sun4i_ts_irq_handle_input(struct sun4i_ts_data *ts, u32 reg_val)
    {
    u32 x, y;
    if (reg_val & FIFO_DATA_PENDING) {
    x = readl(ts.base + TP_DATA);
    y = readl(ts.base + TP_DATA);
// The 1st location reported after an up event is unreliable
    if (!ts.ignore_fifo_data) {
    input_report_abs(ts.input, ABS_X, x);
    input_report_abs(ts.input, ABS_Y, y);
//
// The hardware has a separate down status bit, but
// that gets set before we get the first location,
// resulting in reporting a click on the old location.
//
    input_report_key(ts.input, BTN_TOUCH, 1);
    input_sync(ts.input);
    } else {
    ts.ignore_fifo_data = false;
    }
    }
    if (reg_val & TP_UP_PENDING) {
    ts.ignore_fifo_data = true;
    input_report_key(ts.input, BTN_TOUCH, 0);
    input_sync(ts.input);
    }
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ts_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun4i_ts_irq(int irq, void *dev_id)
    {
    struct sun4i_ts_data *ts = dev_id;
    u32 reg_val;
    reg_val  = readl(ts.base + TP_INT_FIFOS);
    if (reg_val & TEMP_DATA_PENDING)
    ts.temp_data = readl(ts.base + TEMP_DATA);
    if (ts.input)
    sun4i_ts_irq_handle_input(ts, reg_val);
    writel(reg_val, ts.base + TP_INT_FIFOS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ts_open(dev: *mut input_dev) -> c_int {
    static int sun4i_ts_open(struct input_dev *dev)
    {
    struct sun4i_ts_data *ts = input_get_drvdata(dev);
// Flush, set trig level to 1, enable temp, data and up irqs
    writel(TEMP_IRQ_EN(1) | DATA_IRQ_EN(1) | FIFO_TRIG(1) | FIFO_FLUSH(1) |
    TP_UP_IRQ_EN(1), ts.base + TP_INT_FIFOC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ts_close(dev: *mut input_dev) {
    static void sun4i_ts_close(struct input_dev *dev)
    {
    struct sun4i_ts_data *ts = input_get_drvdata(dev);
// Deactivate all input IRQs
    writel(TEMP_IRQ_EN(1), ts.base + TP_INT_FIFOC);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_get_temp(ts: *const sun4i_ts_data, temp: *mut c_int) -> c_int {
    static int sun4i_get_temp(const struct sun4i_ts_data *ts, int *temp)
    {
// No temp_data until the first irq
    if (ts.temp_data == -1)
    return -EAGAIN;
// temp = ts->temp_data * ts->temp_step - ts->temp_offset;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_get_tz_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int sun4i_get_tz_temp(struct thermal_zone_device *tz, int *temp)
    {
    return sun4i_get_temp(thermal_zone_device_priv(tz), temp);
    }
    static const struct thermal_zone_device_ops sun4i_ts_tz_ops = {
    .get_temp = sun4i_get_tz_temp,
    };
    static ssize_t show_temp(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct sun4i_ts_data *ts = dev_get_drvdata(dev);
    int temp;
    int error;
    error = sun4i_get_temp(ts, &temp);
    if (error)
    return error;
    return sprintf(buf, "%d\n", temp);
    }
    static ssize_t show_temp_label(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    return sprintf(buf, "SoC temperature\n");
    }
    static DEVICE_ATTR(temp1_input, S_IRUGO, show_temp, core::ptr::null_mut());
    static DEVICE_ATTR(temp1_label, S_IRUGO, show_temp_label, core::ptr::null_mut());
    static struct attribute *sun4i_ts_attrs[] = {
    &dev_attr_temp1_input.attr,
    &dev_attr_temp1_label.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(sun4i_ts);
#[no_mangle]
unsafe extern "C" fn sun4i_ts_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_ts_probe(struct platform_device *pdev)
    {
    struct sun4i_ts_data *ts;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device *hwmon;
    struct thermal_zone_device *thermal;
    int error;
    u32 reg;
    bool ts_attached;
    let mut tp_sensitive_adjust: u32 = 15;
    let mut filter_type: u32 = 1;
    ts = devm_kzalloc(dev, sizeof(struct sun4i_ts_data), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.dev = dev;
    ts.ignore_fifo_data = true;
    ts.temp_data = -1;
    if (of_device_is_compatible(np, "allwinner,sun6i-a31-ts")) {
// Allwinner SDK has temperature (C) = (value / 6) - 271
    ts.temp_offset = 271000;
    ts.temp_step = 167;
    } else if (of_device_is_compatible(np, "allwinner,sun4i-a10-ts")) {
//
// The A10 temperature sensor has quite a wide spread, these
// parameters are based on the averaging of the calibration
// results of 4 completely different boards, with a spread of
// temp_step from 0.096 - 0.170 and temp_offset from 176 - 331.
//
    ts.temp_offset = 257000;
    ts.temp_step = 133;
    } else {
//
// The user manuals do not contain the formula for calculating
// the temperature. The formula used here is from the AXP209,
// which is designed by X-Powers, an affiliate of Allwinner:
//
// temperature (C) = (value * 0.1) - 144.7
//
// Allwinner does not have any documentation whatsoever for
// this hardware. Moreover, it is claimed that the sensor
// is inaccurate and cannot work properly.
//
    ts.temp_offset = 144700;
    ts.temp_step = 100;
    }
    ts_attached = of_property_read_bool(np, "allwinner,ts-attached");
    if (ts_attached) {
    ts.input = devm_input_allocate_device(dev);
    if (!ts.input)
    return -ENOMEM;
    ts.input.name = pdev.name;
    ts.input.phys = "sun4i_ts/input0";
    ts.input.open = sun4i_ts_open;
    ts.input.close = sun4i_ts_close;
    ts.input.id.bustype = BUS_HOST;
    ts.input.id.vendor = 0x0001;
    ts.input.id.product = 0x0001;
    ts.input.id.version = 0x0100;
    ts.input.evbit[0] =  BIT(EV_SYN) | BIT(EV_KEY) | BIT(EV_ABS);
    __set_bit(BTN_TOUCH, ts.input.keybit);
    input_set_abs_params(ts.input, ABS_X, 0, 4095, 0, 0);
    input_set_abs_params(ts.input, ABS_Y, 0, 4095, 0, 0);
    input_set_drvdata(ts.input, ts);
    }
    ts.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ts.base))
    return PTR_ERR(ts.base);
    ts.irq = platform_get_irq(pdev, 0);
    error = devm_request_irq(dev, ts.irq, sun4i_ts_irq, 0, "sun4i-ts", ts);
    if (error)
    return error;
//
// Select HOSC clk, clkin = clk / 6, adc samplefreq = clkin / 8192,
// t_acq = clkin / (16 * 64)
//
    writel(ADC_CLK_SEL(0) | ADC_CLK_DIV(2) | FS_DIV(7) | T_ACQ(63),
    ts.base + TP_CTRL0);
//
// tp_sensitive_adjust is an optional property
// tp_mode = 0 : only x and y coordinates, as we don't use dual touch
//
    of_property_read_u32(np, "allwinner,tp-sensitive-adjust",
    &tp_sensitive_adjust);
    writel(TP_SENSITIVE_ADJUST(tp_sensitive_adjust) | TP_MODE_SELECT(0),
    ts.base + TP_CTRL2);
//
// Enable median and averaging filter, optional property for
// filter type.
//
    of_property_read_u32(np, "allwinner,filter-type", &filter_type);
    writel(FILTER_EN(1) | FILTER_TYPE(filter_type), ts.base + TP_CTRL3);
// Enable temperature measurement, period 1953 (2 seconds)
    writel(TEMP_ENABLE(1) | TEMP_PERIOD(1953), ts.base + TP_TPR);
//
// Set stylus up debounce to aprox 10 ms, enable debounce, and
// finally enable tp mode.
//
    reg = STYLUS_UP_DEBOUN(5) | STYLUS_UP_DEBOUN_EN(1);
    if (of_device_is_compatible(np, "allwinner,sun6i-a31-ts"))
    reg |= SUN6I_TP_MODE_EN(1);
    else
    reg |= TP_MODE_EN(1);
    writel(reg, ts.base + TP_CTRL1);
//
// The thermal core does not register hwmon devices for DT-based
// thermal zone sensors, such as this one.
//
    hwmon = devm_hwmon_device_register_with_groups(ts.dev, "sun4i_ts",
    ts, sun4i_ts_groups);
    if (IS_ERR(hwmon))
    return PTR_ERR(hwmon);
    thermal = devm_thermal_of_zone_register(ts.dev, 0, ts,
    &sun4i_ts_tz_ops);
    if (IS_ERR(thermal))
    return PTR_ERR(thermal);
    writel(TEMP_IRQ_EN(1), ts.base + TP_INT_FIFOC);
    if (ts_attached) {
    error = input_register_device(ts.input);
    if (error) {
    writel(0, ts.base + TP_INT_FIFOC);
    return error;
    }
    }
    platform_set_drvdata(pdev, ts);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ts_remove(pdev: *mut platform_device) {
    static void sun4i_ts_remove(struct platform_device *pdev)
    {
    struct sun4i_ts_data *ts = platform_get_drvdata(pdev);
// Explicit unregister to avoid open/close changing the imask later
    if (ts.input)
    input_unregister_device(ts.input);
// Deactivate all IRQs
    writel(0, ts.base + TP_INT_FIFOC);
    }
    static const struct of_device_id sun4i_ts_of_match[] = {
    { .compatible = "allwinner,sun4i-a10-ts", },
    { .compatible = "allwinner,sun5i-a13-ts", },
    { .compatible = "allwinner,sun6i-a31-ts", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sun4i_ts_of_match);
    static struct platform_driver sun4i_ts_driver = {
    .driver	= {
    .name	= "sun4i-ts",
    .of_match_table = sun4i_ts_of_match,
    },
    .probe	= sun4i_ts_probe,
    .remove	= sun4i_ts_remove,
    };
    module_platform_driver(sun4i_ts_driver);
    MODULE_DESCRIPTION("Allwinner sun4i resistive touchscreen controller driver");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_LICENSE("GPL");
