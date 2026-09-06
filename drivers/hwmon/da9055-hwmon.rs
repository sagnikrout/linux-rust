//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/da9055-hwmon.c
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
// HWMON Driver for Dialog DA9055
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

pub const DA9055_ADCIN_DIV: c_int = 102;
pub const DA9055_VSYS_DIV: c_int = 85;
pub const DA9055_ADC_VSYS: c_int = 0;
pub const DA9055_ADC_ADCIN1: c_int = 1;
pub const DA9055_ADC_ADCIN2: c_int = 2;
pub const DA9055_ADC_ADCIN3: c_int = 3;
pub const DA9055_ADC_TJUNC: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9055_hwmon {
    pub da9055: *mut da9055,
    pub hwmon_lock: mutex,
    pub irq_lock: mutex,
    pub done: completion,
}

    static const char * const input_names[] = {
    [DA9055_ADC_VSYS]	= "VSYS",
    [DA9055_ADC_ADCIN1]	= "ADC IN1",
    [DA9055_ADC_ADCIN2]	= "ADC IN2",
    [DA9055_ADC_ADCIN3]	= "ADC IN3",
    [DA9055_ADC_TJUNC]	= "CHIP TEMP",
    };
    static const u8 chan_mux[DA9055_ADC_TJUNC + 1] = {
    [DA9055_ADC_VSYS]	= DA9055_ADC_MUX_VSYS,
    [DA9055_ADC_ADCIN1]	= DA9055_ADC_MUX_ADCIN1,
    [DA9055_ADC_ADCIN2]	= DA9055_ADC_MUX_ADCIN2,
    [DA9055_ADC_ADCIN3]	= DA9055_ADC_MUX_ADCIN3,
    [DA9055_ADC_TJUNC]	= DA9055_ADC_MUX_T_SENSE,
    };
    static int da9055_adc_manual_read(struct da9055_hwmon *hwmon,
    unsigned char channel)
    {
    int ret;
    unsigned short calc_data;
    unsigned short data;
    unsigned char mux_sel;
    struct da9055 *da9055 = hwmon.da9055;
    if (channel > DA9055_ADC_TJUNC)
    return -EINVAL;
    mutex_lock(&hwmon.irq_lock);
// Selects desired MUX for manual conversion
    mux_sel = chan_mux[channel] | DA9055_ADC_MAN_CONV;
    ret = da9055_reg_write(da9055, DA9055_REG_ADC_MAN, mux_sel);
    if (ret < 0)
    goto err;
// Wait for an interrupt
    if (!wait_for_completion_timeout(&hwmon.done,
    msecs_to_jiffies(500))) {
    dev_err(da9055.dev,
    "timeout waiting for ADC conversion interrupt\n");
    ret = -ETIMEDOUT;
    goto err;
    }
    ret = da9055_reg_read(da9055, DA9055_REG_ADC_RES_H);
    if (ret < 0)
    goto err;
    calc_data = (unsigned short)ret;
    data = calc_data << 2;
    ret = da9055_reg_read(da9055, DA9055_REG_ADC_RES_L);
    if (ret < 0)
    goto err;
    calc_data = (unsigned short)(ret & DA9055_ADC_LSB_MASK);
    data |= calc_data;
    ret = data;
    err:
    mutex_unlock(&hwmon.irq_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9055_auxadc_irq(irq: c_int, irq_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9055_auxadc_irq(int irq, void *irq_data)
    {
    struct da9055_hwmon *hwmon = irq_data;
    complete(&hwmon.done);
    return IRQ_HANDLED;
    }
// Conversion function for VSYS and ADCINx
#[no_mangle]
pub unsafe extern "C" fn volt_reg_to_mv(value: c_int, channel: c_int) -> c_int {
    static inline int volt_reg_to_mv(int value, int channel)
    {
    if (channel == DA9055_ADC_VSYS)
    return DIV_ROUND_CLOSEST(value * 1000, DA9055_VSYS_DIV) + 2500;
    else
    return DIV_ROUND_CLOSEST(value * 1000, DA9055_ADCIN_DIV);
    }
#[no_mangle]
unsafe extern "C" fn da9055_enable_auto_mode(da9055: *mut da9055, channel: c_int) -> c_int {
    static int da9055_enable_auto_mode(struct da9055 *da9055, int channel)
    {
    return da9055_reg_update(da9055, DA9055_REG_ADC_CONT, 1 << channel,
    1 << channel);
    }
#[no_mangle]
unsafe extern "C" fn da9055_disable_auto_mode(da9055: *mut da9055, channel: c_int) -> c_int {
    static int da9055_disable_auto_mode(struct da9055 *da9055, int channel)
    {
    return da9055_reg_update(da9055, DA9055_REG_ADC_CONT, 1 << channel, 0);
    }
    static ssize_t da9055_auto_ch_show(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct da9055_hwmon *hwmon = dev_get_drvdata(dev);
    int ret, adc;
    let mut channel: c_int = to_sensor_dev_attr(devattr).index;
    mutex_lock(&hwmon.hwmon_lock);
    ret = da9055_enable_auto_mode(hwmon.da9055, channel);
    if (ret < 0)
    goto hwmon_err;
    usleep_range(10000, 10500);
    adc = da9055_reg_read(hwmon.da9055, DA9055_REG_VSYS_RES + channel);
    if (adc < 0) {
    ret = adc;
    goto hwmon_err_release;
    }
    ret = da9055_disable_auto_mode(hwmon.da9055, channel);
    if (ret < 0)
    goto hwmon_err;
    mutex_unlock(&hwmon.hwmon_lock);
    return sprintf(buf, "%d\n", volt_reg_to_mv(adc, channel));
    hwmon_err_release:
    da9055_disable_auto_mode(hwmon.da9055, channel);
    hwmon_err:
    mutex_unlock(&hwmon.hwmon_lock);
    return ret;
    }
    static ssize_t da9055_tjunc_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct da9055_hwmon *hwmon = dev_get_drvdata(dev);
    int tjunc;
    int toffset;
    tjunc = da9055_adc_manual_read(hwmon, DA9055_ADC_TJUNC);
    if (tjunc < 0)
    return tjunc;
    toffset = da9055_reg_read(hwmon.da9055, DA9055_REG_T_OFFSET);
    if (toffset < 0)
    return toffset;
//
// Degrees celsius = -0.4084 * (ADC_RES - T_OFFSET) + 307.6332
// T_OFFSET is a trim value used to improve accuracy of the result
//
#[no_mangle]
pub unsafe extern "C" fn sprintf(_arg: buf, _arg: "%d\n", toffset: *mut *mut DIV_ROUND_CLOSEST(-4084  (tjunc -) -> return {
    return sprintf(buf, "%d\n", DIV_ROUND_CLOSEST(-4084 * (tjunc - toffset)
    + 3076332, 10000));
    }
    static ssize_t label_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    return sprintf(buf, "%s\n",
    input_names[to_sensor_dev_attr(devattr).index]);
    }
    static SENSOR_DEVICE_ATTR_RO(in0_input, da9055_auto_ch, DA9055_ADC_VSYS);
    static SENSOR_DEVICE_ATTR_RO(in0_label, label, DA9055_ADC_VSYS);
    static SENSOR_DEVICE_ATTR_RO(in1_input, da9055_auto_ch, DA9055_ADC_ADCIN1);
    static SENSOR_DEVICE_ATTR_RO(in1_label, label, DA9055_ADC_ADCIN1);
    static SENSOR_DEVICE_ATTR_RO(in2_input, da9055_auto_ch, DA9055_ADC_ADCIN2);
    static SENSOR_DEVICE_ATTR_RO(in2_label, label, DA9055_ADC_ADCIN2);
    static SENSOR_DEVICE_ATTR_RO(in3_input, da9055_auto_ch, DA9055_ADC_ADCIN3);
    static SENSOR_DEVICE_ATTR_RO(in3_label, label, DA9055_ADC_ADCIN3);
    static SENSOR_DEVICE_ATTR_RO(temp1_input, da9055_tjunc, DA9055_ADC_TJUNC);
    static SENSOR_DEVICE_ATTR_RO(temp1_label, label, DA9055_ADC_TJUNC);
    static struct attribute *da9055_attrs[] = {
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in0_label.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_label.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_label.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in3_label.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(da9055);
#[no_mangle]
unsafe extern "C" fn da9055_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int da9055_hwmon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct da9055_hwmon *hwmon;
    struct device *hwmon_dev;
    int hwmon_irq, ret;
    hwmon = devm_kzalloc(dev, sizeof(struct da9055_hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
    mutex_init(&hwmon.hwmon_lock);
    mutex_init(&hwmon.irq_lock);
    init_completion(&hwmon.done);
    hwmon.da9055 = dev_get_drvdata(pdev.dev.parent);
    hwmon_irq = platform_get_irq_byname(pdev, "HWMON");
    if (hwmon_irq < 0)
    return hwmon_irq;
    ret = devm_request_threaded_irq(&pdev.dev, hwmon_irq,
    core::ptr::null_mut(), da9055_auxadc_irq,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "adc-irq", hwmon);
    if (ret != 0)
    return ret;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, "da9055",
    hwmon,
    da9055_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver da9055_hwmon_driver = {
    .probe = da9055_hwmon_probe,
    .driver = {
    .name = "da9055-hwmon",
    },
    };
    module_platform_driver(da9055_hwmon_driver);
    MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
    MODULE_DESCRIPTION("DA9055 HWMON driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9055-hwmon");
