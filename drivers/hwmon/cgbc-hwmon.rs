//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/cgbc-hwmon.c
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
// cgbc-hwmon - Congatec Board Controller hardware monitoring driver
//
// Copyright (C) 2024 Thomas Richard <thomas.richard@bootlin.com>
//

pub const CGBC_HWMON_CMD_SENSOR: c_uint = 0x77;
pub const CGBC_HWMON_CMD_SENSOR_DATA_SIZE: c_uint = 0x05;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_hwmon_sensor {
    pub type: enum hwmon_sensor_types,
    pub active: bool,
    pub index: c_uint,
    pub channel: c_uint,
    pub label: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_hwmon_data {
    pub cgbc: *mut cgbc_device_data,
    pub nb_sensors: c_uint,
    pub sensors: *mut cgbc_hwmon_sensor,
}

    enum cgbc_sensor_types {
    CGBC_HWMON_TYPE_TEMP = 1,
    CGBC_HWMON_TYPE_IN,
    CGBC_HWMON_TYPE_FAN
    };
    static const char * const cgbc_hwmon_labels_temp[] = {
    "CPU Temperature",
    "Box Temperature",
    "Ambient Temperature",
    "Board Temperature",
    "Carrier Temperature",
    "Chipset Temperature",
    "Video Temperature",
    "Other Temperature",
    "TOPDIM Temperature",
    "BOTTOMDIM Temperature",
    };
    static const struct {
    enum hwmon_sensor_types type;
    const char *label;
    } cgbc_hwmon_labels_in[] = {
    { hwmon_in, "CPU Voltage" },
    { hwmon_in, "DC Runtime Voltage" },
    { hwmon_in, "DC Standby Voltage" },
    { hwmon_in, "CMOS Battery Voltage" },
    { hwmon_in, "Battery Voltage" },
    { hwmon_in, "AC Voltage" },
    { hwmon_in, "Other Voltage" },
    { hwmon_in, "5V Voltage" },
    { hwmon_in, "5V Standby Voltage" },
    { hwmon_in, "3V3 Voltage" },
    { hwmon_in, "3V3 Standby Voltage" },
    { hwmon_in, "VCore A Voltage" },
    { hwmon_in, "VCore B Voltage" },
    { hwmon_in, "12V Voltage" },
    { hwmon_curr, "DC Current" },
    { hwmon_curr, "5V Current" },
    { hwmon_curr, "12V Current" },
    };
pub const CGBC_HWMON_NB_IN_SENSORS: c_int = 14;
    static const char * const cgbc_hwmon_labels_fan[] = {
    "CPU Fan",
    "Box Fan",
    "Ambient Fan",
    "Chipset Fan",
    "Video Fan",
    "Other Fan",
    };
#[no_mangle]
unsafe extern "C" fn cgbc_hwmon_cmd(cgbc: *mut cgbc_device_data, index: u8, data: *mut u8) -> c_int {
    static int cgbc_hwmon_cmd(struct cgbc_device_data *cgbc, u8 index, u8 *data)
    {
    u8 cmd[2] = {CGBC_HWMON_CMD_SENSOR, index};
    return cgbc_command(cgbc, cmd, sizeof(cmd), data, CGBC_HWMON_CMD_SENSOR_DATA_SIZE, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn cgbc_hwmon_probe_sensors(dev: *mut device, hwmon: *mut cgbc_hwmon_data) -> c_int {
    static int cgbc_hwmon_probe_sensors(struct device *dev, struct cgbc_hwmon_data *hwmon)
    {
    struct cgbc_device_data *cgbc = hwmon.cgbc;
    struct cgbc_hwmon_sensor *sensor = hwmon.sensors;
    u8 data[CGBC_HWMON_CMD_SENSOR_DATA_SIZE], nb_sensors, i;
    int ret;
    ret = cgbc_hwmon_cmd(cgbc, 0, &data[0]);
    if (ret)
    return ret;
    nb_sensors = data[0];
    hwmon.sensors = devm_kzalloc(dev, sizeof(*hwmon.sensors) * nb_sensors, GFP_KERNEL);
    if (!hwmon.sensors)
    return -ENOMEM;
    sensor = hwmon.sensors;
    for (i = 0; i < nb_sensors; i++) {
    enum cgbc_sensor_types type;
    unsigned int channel;
//
// No need to request data for the first sensor.
// We got data for the first sensor when we ask the number of sensors to the Board
// Controller.
//
    if (i) {
    ret = cgbc_hwmon_cmd(cgbc, i, &data[0]);
    if (ret)
    return ret;
    }
    type = FIELD_GET(CGBC_HWMON_TYPE_MASK, data[1]);
    channel = FIELD_GET(CGBC_HWMON_ID_MASK, data[1]) - 1;
    if (type == CGBC_HWMON_TYPE_TEMP && channel < ARRAY_SIZE(cgbc_hwmon_labels_temp)) {
    sensor.type = hwmon_temp;
    sensor.label = cgbc_hwmon_labels_temp[channel];
    } else if (type == CGBC_HWMON_TYPE_IN &&
    channel < ARRAY_SIZE(cgbc_hwmon_labels_in)) {
//
// The Board Controller doesn't differentiate current and voltage sensors.
// Get the sensor type from cgbc_hwmon_labels_in[channel].type instead.
//
    sensor.type = cgbc_hwmon_labels_in[channel].type;
    sensor.label = cgbc_hwmon_labels_in[channel].label;
    } else if (type == CGBC_HWMON_TYPE_FAN &&
    channel < ARRAY_SIZE(cgbc_hwmon_labels_fan)) {
    sensor.type = hwmon_fan;
    sensor.label = cgbc_hwmon_labels_fan[channel];
    } else {
    dev_warn(dev, "Board Controller returned an unknown sensor (type=%d, channel=%d), ignore it",
    type, channel);
    continue;
    }
    sensor.active = FIELD_GET(CGBC_HWMON_ACTIVE_BIT, data[1]);
    sensor.channel = channel;
    sensor.index = i;
    sensor++;
    hwmon.nb_sensors++;
    }
    return 0;
    }
    static struct cgbc_hwmon_sensor *cgbc_hwmon_find_sensor(struct cgbc_hwmon_data *hwmon,
    enum hwmon_sensor_types type, int channel)
    {
    struct cgbc_hwmon_sensor *sensor = core::ptr::null_mut();
    int i;
//
// The Board Controller doesn't differentiate current and voltage sensors.
// The channel value (from the Board Controller point of view) shall be computed for current
// sensors.
//
    if (type == hwmon_curr)
    channel += CGBC_HWMON_NB_IN_SENSORS;
    for (i = 0; i < hwmon.nb_sensors; i++) {
    if (hwmon.sensors[i].type == type && hwmon.sensors[i].channel == channel) {
    sensor = &hwmon.sensors[i];
    break;
    }
    }
    return sensor;
    }
    static int cgbc_hwmon_read(struct device *dev, enum hwmon_sensor_types type, u32 attr, int channel,
    long *val)
    {
    struct cgbc_hwmon_data *hwmon = dev_get_drvdata(dev);
    struct cgbc_hwmon_sensor *sensor = cgbc_hwmon_find_sensor(hwmon, type, channel);
    struct cgbc_device_data *cgbc = hwmon.cgbc;
    u8 data[CGBC_HWMON_CMD_SENSOR_DATA_SIZE];
    int ret;
    ret = cgbc_hwmon_cmd(cgbc, sensor.index, &data[0]);
    if (ret)
    return ret;
// val = (data[3] << 8) | data[2];
//
// For the Board Controller 1lsb = 0.1 degree centigrade.
// Other units are as expected.
//
    if (sensor.type == hwmon_temp)
// val *= 100;
    return 0;
    }
    static umode_t cgbc_hwmon_is_visible(const void *_data, enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    struct cgbc_hwmon_data *data = (struct cgbc_hwmon_data *)_data;
    struct cgbc_hwmon_sensor *sensor;
    sensor = cgbc_hwmon_find_sensor(data, type, channel);
    if (!sensor)
    return 0;
    return sensor.active ? 0444 : 0;
    }
    static int cgbc_hwmon_read_string(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    struct cgbc_hwmon_data *hwmon = dev_get_drvdata(dev);
    struct cgbc_hwmon_sensor *sensor = cgbc_hwmon_find_sensor(hwmon, type, channel);
// str = sensor->label;
    return 0;
    }
    static const struct hwmon_channel_info * const cgbc_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL, HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL, HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL, HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL, HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL, HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL, HWMON_I_INPUT | HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT | HWMON_C_LABEL, HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL),
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_LABEL, HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL, HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL, HWMON_F_INPUT | HWMON_F_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops cgbc_hwmon_ops = {
    .is_visible = cgbc_hwmon_is_visible,
    .read = cgbc_hwmon_read,
    .read_string = cgbc_hwmon_read_string,
    };
    static const struct hwmon_chip_info cgbc_chip_info = {
    .ops = &cgbc_hwmon_ops,
    .info = cgbc_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn cgbc_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int cgbc_hwmon_probe(struct platform_device *pdev)
    {
    struct cgbc_device_data *cgbc = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct cgbc_hwmon_data *data;
    struct device *hwmon_dev;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.cgbc = cgbc;
    ret = cgbc_hwmon_probe_sensors(dev, data);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to probe sensors");
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "cgbc_hwmon", data, &cgbc_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver cgbc_hwmon_driver = {
    .driver = {
    .name = "cgbc-hwmon",
    },
    .probe = cgbc_hwmon_probe,
    };
    module_platform_driver(cgbc_hwmon_driver);
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
    MODULE_DESCRIPTION("Congatec Board Controller Hardware Monitoring Driver");
    MODULE_LICENSE("GPL");
