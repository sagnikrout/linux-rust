//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/raspberrypi-hwmon.c
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
// Raspberry Pi voltage sensor driver
//
// Based on firmware/raspberrypi.c by Noralf Trønnes
//
// Copyright (C) 2018 Stefan Wahren <stefan.wahren@i2se.com>
// Copyright (C) 2026 Shubham Chakraborty <chakrabortyshubham66@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_hwmon_data {
    pub hwmon_dev: *mut device,
    pub fw: *mut rpi_firmware,
    pub valid_inputs: u32,
    pub last_throttled: u32,
    pub get_values_poll_work: delayed_work,
}

    static const char * const rpi_hwmon_labels[] = {
    "core",
    "sdram_c",
    "sdram_i",
    "sdram_p",
    };
#[no_mangle]
unsafe extern "C" fn rpi_firmware_get_throttled(data: *mut rpi_hwmon_data) {
    static void rpi_firmware_get_throttled(struct rpi_hwmon_data *data)
    {
    u32 new_uv, old_uv, value;
    int ret;
// Request firmware to clear sticky bits
    value = 0xffff;
    ret = rpi_firmware_property(data.fw, RPI_FIRMWARE_GET_THROTTLED,
    &value, sizeof(value));
    if (ret) {
    dev_err_once(data.hwmon_dev, "Failed to get throttled (%d)\n",
    ret);
    return;
    }
    new_uv = value & UNDERVOLTAGE_STICKY_BIT;
    old_uv = data.last_throttled & UNDERVOLTAGE_STICKY_BIT;
    data.last_throttled = value;
    if (new_uv == old_uv)
    return;
    if (new_uv)
    dev_crit(data.hwmon_dev, "Undervoltage detected!\n");
    else
    dev_info(data.hwmon_dev, "Voltage normalised\n");
    hwmon_notify_event(data.hwmon_dev, hwmon_in, hwmon_in_lcrit_alarm, 0);
    }
    static int rpi_firmware_get_voltage(struct rpi_hwmon_data *data, u32 id,
    long *val)
    {
    struct rpi_firmware_get_voltage_request packet =
    RPI_FIRMWARE_GET_VOLTAGE_REQUEST(id);
    int ret;
    ret = rpi_firmware_property(data.fw, RPI_FIRMWARE_GET_VOLTAGE,
    &packet, sizeof(packet));
    if (ret)
    return ret;
// val = le32_to_cpu(packet.value) / 1000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_values_poll(work: *mut work_struct) {
    static void get_values_poll(struct work_struct *work)
    {
    struct rpi_hwmon_data *data;
    data = container_of(work, struct rpi_hwmon_data,
    get_values_poll_work.work);
    rpi_firmware_get_throttled(data);
//
// We can't run faster than the sticky shift (100ms) since we get
// flipping in the sticky bits that are cleared.
//
    schedule_delayed_work(&data.get_values_poll_work, 2 * HZ);
    }
#[no_mangle]
unsafe extern "C" fn rpi_hwmon_cancel_poll_work(res: *mut c_void) {
    static void rpi_hwmon_cancel_poll_work(void *res)
    {
    struct rpi_hwmon_data *data = res;
    disable_delayed_work_sync(&data.get_values_poll_work);
    }
    static int rpi_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct rpi_hwmon_data *data = dev_get_drvdata(dev);
    if (type == hwmon_in) {
    switch (attr) {
    case hwmon_in_input:
    switch (channel) {
    case 0:
    return rpi_firmware_get_voltage(data,
    RPI_FIRMWARE_VOLT_ID_CORE,
    val);
    case 1:
    return rpi_firmware_get_voltage(data,
    RPI_FIRMWARE_VOLT_ID_SDRAM_C,
    val);
    case 2:
    return rpi_firmware_get_voltage(data,
    RPI_FIRMWARE_VOLT_ID_SDRAM_I,
    val);
    case 3:
    return rpi_firmware_get_voltage(data,
    RPI_FIRMWARE_VOLT_ID_SDRAM_P,
    val);
    default:
    return -EOPNOTSUPP;
    }
    case hwmon_in_lcrit_alarm:
    if (channel == 0) {
// val = !!(data->last_throttled & UNDERVOLTAGE_STICKY_BIT);
    return 0;
    }
    return -EOPNOTSUPP;
    default:
    return -EOPNOTSUPP;
    }
    }
    return -EOPNOTSUPP;
    }
    static int rpi_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    if (type == hwmon_in && attr == hwmon_in_label) {
    if (channel >= ARRAY_SIZE(rpi_hwmon_labels))
    return -EOPNOTSUPP;
// str = rpi_hwmon_labels[channel];
    return 0;
    }
    return -EOPNOTSUPP;
    }
    static umode_t rpi_is_visible(const void *_data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct rpi_hwmon_data *data = _data;
    if (type == hwmon_in) {
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_label:
    if (!(data.valid_inputs & BIT(channel)))
    return 0;
    return 0444;
    case hwmon_in_lcrit_alarm:
    if (channel == 0)
    return 0444;
    return 0;
    default:
    return 0;
    }
    }
    return 0;
    }
    static const struct hwmon_channel_info * const rpi_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL | HWMON_I_LCRIT_ALARM,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops rpi_hwmon_ops = {
    .is_visible = rpi_is_visible,
    .read = rpi_read,
    .read_string = rpi_read_string,
    };
    static const struct hwmon_chip_info rpi_chip_info = {
    .ops = &rpi_hwmon_ops,
    .info = rpi_info,
    };
#[no_mangle]
unsafe extern "C" fn rpi_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int rpi_hwmon_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rpi_hwmon_data *data;
    long voltage;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
// Parent driver assure that firmware is correct
    data.fw = dev_get_drvdata(dev.parent);
    ret = rpi_firmware_get_voltage(data, RPI_FIRMWARE_VOLT_ID_CORE,
    &voltage);
    if (!ret)
    data.valid_inputs |= BIT(0);
    ret = rpi_firmware_get_voltage(data, RPI_FIRMWARE_VOLT_ID_SDRAM_C,
    &voltage);
    if (!ret)
    data.valid_inputs |= BIT(1);
    ret = rpi_firmware_get_voltage(data, RPI_FIRMWARE_VOLT_ID_SDRAM_I,
    &voltage);
    if (!ret)
    data.valid_inputs |= BIT(2);
    ret = rpi_firmware_get_voltage(data, RPI_FIRMWARE_VOLT_ID_SDRAM_P,
    &voltage);
    if (!ret)
    data.valid_inputs |= BIT(3);
    data.hwmon_dev = devm_hwmon_device_register_with_info(dev, "rpi_volt",
    data,
    &rpi_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(data.hwmon_dev))
    return PTR_ERR(data.hwmon_dev);
    INIT_DELAYED_WORK(&data.get_values_poll_work, get_values_poll);
    ret = devm_add_action_or_reset(dev, rpi_hwmon_cancel_poll_work, data);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, data);
    schedule_delayed_work(&data.get_values_poll_work, 2 * HZ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_hwmon_suspend(dev: *mut device) -> c_int {
    static int rpi_hwmon_suspend(struct device *dev)
    {
    struct rpi_hwmon_data *data = dev_get_drvdata(dev);
    cancel_delayed_work_sync(&data.get_values_poll_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_hwmon_resume(dev: *mut device) -> c_int {
    static int rpi_hwmon_resume(struct device *dev)
    {
    struct rpi_hwmon_data *data = dev_get_drvdata(dev);
    get_values_poll(&data.get_values_poll_work.work);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rpi_hwmon_pm_ops, rpi_hwmon_suspend,
    rpi_hwmon_resume);
    static struct platform_driver rpi_hwmon_driver = {
    .probe = rpi_hwmon_probe,
    .driver = {
    .name = "raspberrypi-hwmon",
    .pm = pm_ptr(&rpi_hwmon_pm_ops),
    },
    };
    module_platform_driver(rpi_hwmon_driver);
    MODULE_AUTHOR("Stefan Wahren <wahrenst@gmx.net>");
    MODULE_AUTHOR("Shubham Chakraborty <chakrabortyshubham66@gmail.com>");
    MODULE_DESCRIPTION("Raspberry Pi voltage sensor driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:raspberrypi-hwmon");
