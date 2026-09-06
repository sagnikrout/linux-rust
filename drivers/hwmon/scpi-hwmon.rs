//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/scpi-hwmon.c
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
// System Control and Power Interface(SCPI) based hwmon sensor driver
//
// Copyright (C) 2015 ARM Ltd.
// Punit Agrawal <punit.agrawal@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_data {
    pub scale: c_uint,
    pub info: scpi_sensor_info,
    pub dev_attr_input: device_attribute,
    pub dev_attr_label: device_attribute,
    pub input: [c_char; 20],
    pub label: [c_char; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_thermal_zone {
    pub sensor_id: c_int,
    pub scpi_sensors: *mut scpi_sensors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_sensors {
    pub scpi_ops: *mut scpi_ops,
    pub data: *mut sensor_data,
    pub thermal_zones: list_head,
    pub attrs: *mut attribute,
    pub group: attribute_group,
    pub groups: [*const attribute_group; 2],
}

    static const u32 gxbb_scpi_scale[] = {
    [TEMPERATURE]	= 1,		/* (celsius)		*/
    [VOLTAGE]	= 1000,		/* (millivolts)		*/
    [CURRENT]	= 1000,		/* (milliamperes)	*/
    [POWER]		= 1000000,	/* (microwatts)		*/
    [ENERGY]	= 1000000,	/* (microjoules)	*/
    };
    static const u32 scpi_scale[] = {
    [TEMPERATURE]	= 1000,		/* (millicelsius)	*/
    [VOLTAGE]	= 1000,		/* (millivolts)		*/
    [CURRENT]	= 1000,		/* (milliamperes)	*/
    [POWER]		= 1000000,	/* (microwatts)		*/
    [ENERGY]	= 1000000,	/* (microjoules)	*/
    };
#[no_mangle]
unsafe extern "C" fn scpi_scale_reading(value: *mut u64, sensor: *mut sensor_data) {
    static void scpi_scale_reading(u64 *value, struct sensor_data *sensor)
    {
    if (scpi_scale[sensor.info.class] != sensor.scale) {
// value *= scpi_scale[sensor->info.class];
    do_div(*value, sensor.scale);
    }
    }
#[no_mangle]
unsafe extern "C" fn scpi_read_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int scpi_read_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct scpi_thermal_zone *zone = thermal_zone_device_priv(tz);
    struct scpi_sensors *scpi_sensors = zone.scpi_sensors;
    struct scpi_ops *scpi_ops = scpi_sensors.scpi_ops;
    struct sensor_data *sensor = &scpi_sensors.data[zone.sensor_id];
    u64 value;
    int ret;
    ret = scpi_ops.sensor_get_value(sensor.info.sensor_id, &value);
    if (ret)
    return ret;
    scpi_scale_reading(&value, sensor);
// temp = value;
    return 0;
    }
// hwmon callback functions
    static ssize_t
    scpi_show_sensor(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct scpi_sensors *scpi_sensors = dev_get_drvdata(dev);
    struct scpi_ops *scpi_ops = scpi_sensors.scpi_ops;
    struct sensor_data *sensor;
    u64 value;
    int ret;
    sensor = container_of(attr, struct sensor_data, dev_attr_input);
    ret = scpi_ops.sensor_get_value(sensor.info.sensor_id, &value);
    if (ret)
    return ret;
    scpi_scale_reading(&value, sensor);
//
// Temperature sensor values are treated as signed values based on
// observation even though that is not explicitly specified, and
// because an unsigned u64 temperature does not really make practical
// sense especially when the temperature is below zero degrees Celsius.
//
    if (sensor.info.class == TEMPERATURE)
    return sprintf(buf, "%lld\n", (s64)value);
    return sprintf(buf, "%llu\n", value);
    }
    static ssize_t
    scpi_show_label(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct sensor_data *sensor;
    sensor = container_of(attr, struct sensor_data, dev_attr_label);
    return sprintf(buf, "%s\n", sensor.info.name);
    }
    static const struct thermal_zone_device_ops scpi_sensor_ops = {
    .get_temp = scpi_read_temp,
    };
    static const struct of_device_id scpi_of_match[] = {
    {.compatible = "arm,scpi-sensors", .data = &scpi_scale},
    {.compatible = "amlogic,meson-gxbb-scpi-sensors", .data = &gxbb_scpi_scale},
    {},
    };
    MODULE_DEVICE_TABLE(of, scpi_of_match);
#[no_mangle]
unsafe extern "C" fn scpi_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int scpi_hwmon_probe(struct platform_device *pdev)
    {
    u16 nr_sensors, i;
    const u32 *scale;
    let mut num_temp: c_int = 0, num_volt = 0, num_current = 0, num_power = 0;
    let mut num_energy: c_int = 0;
    struct scpi_ops *scpi_ops;
    struct device *hwdev, *dev = &pdev.dev;
    struct scpi_sensors *scpi_sensors;
    int idx, ret;
    scpi_ops = get_scpi_ops();
    if (!scpi_ops)
    return -EPROBE_DEFER;
    ret = scpi_ops.sensor_get_capability(&nr_sensors);
    if (ret)
    return ret;
    if (!nr_sensors)
    return -ENODEV;
    scpi_sensors = devm_kzalloc(dev, sizeof(*scpi_sensors), GFP_KERNEL);
    if (!scpi_sensors)
    return -ENOMEM;
    scpi_sensors.data = devm_kcalloc(dev, nr_sensors,
    sizeof(*scpi_sensors.data), GFP_KERNEL);
    if (!scpi_sensors.data)
    return -ENOMEM;
    scpi_sensors.attrs = devm_kcalloc(dev, (nr_sensors * 2) + 1,
    sizeof(*scpi_sensors.attrs), GFP_KERNEL);
    if (!scpi_sensors.attrs)
    return -ENOMEM;
    scpi_sensors.scpi_ops = scpi_ops;
    scale = of_device_get_match_data(&pdev.dev);
    if (!scale) {
    dev_err(&pdev.dev, "Unable to initialize scpi-hwmon data\n");
    return -ENODEV;
    }
    for (i = 0, idx = 0; i < nr_sensors; i++) {
    struct sensor_data *sensor = &scpi_sensors.data[idx];
    ret = scpi_ops.sensor_get_info(i, &sensor.info);
    if (ret)
    return ret;
    switch (sensor.info.class) {
    case TEMPERATURE:
    snprintf(sensor.input, sizeof(sensor.input),
    "temp%d_input", num_temp + 1);
    snprintf(sensor.label, sizeof(sensor.input),
    "temp%d_label", num_temp + 1);
    num_temp++;
    break;
    case VOLTAGE:
    snprintf(sensor.input, sizeof(sensor.input),
    "in%d_input", num_volt);
    snprintf(sensor.label, sizeof(sensor.input),
    "in%d_label", num_volt);
    num_volt++;
    break;
    case CURRENT:
    snprintf(sensor.input, sizeof(sensor.input),
    "curr%d_input", num_current + 1);
    snprintf(sensor.label, sizeof(sensor.input),
    "curr%d_label", num_current + 1);
    num_current++;
    break;
    case POWER:
    snprintf(sensor.input, sizeof(sensor.input),
    "power%d_input", num_power + 1);
    snprintf(sensor.label, sizeof(sensor.input),
    "power%d_label", num_power + 1);
    num_power++;
    break;
    case ENERGY:
    snprintf(sensor.input, sizeof(sensor.input),
    "energy%d_input", num_energy + 1);
    snprintf(sensor.label, sizeof(sensor.input),
    "energy%d_label", num_energy + 1);
    num_energy++;
    break;
    default:
    continue;
    }
    sensor.scale = scale[sensor.info.class];
    sensor.dev_attr_input.attr.mode = 0444;
    sensor.dev_attr_input.show = scpi_show_sensor;
    sensor.dev_attr_input.attr.name = sensor.input;
    sensor.dev_attr_label.attr.mode = 0444;
    sensor.dev_attr_label.show = scpi_show_label;
    sensor.dev_attr_label.attr.name = sensor.label;
    scpi_sensors.attrs[idx << 1] = &sensor.dev_attr_input.attr;
    scpi_sensors.attrs[(idx << 1) + 1] = &sensor.dev_attr_label.attr;
    sysfs_attr_init(scpi_sensors.attrs[idx << 1]);
    sysfs_attr_init(scpi_sensors.attrs[(idx << 1) + 1]);
    idx++;
    }
    scpi_sensors.group.attrs = scpi_sensors.attrs;
    scpi_sensors.groups[0] = &scpi_sensors.group;
    platform_set_drvdata(pdev, scpi_sensors);
    hwdev = devm_hwmon_device_register_with_groups(dev,
    "scpi_sensors", scpi_sensors, scpi_sensors.groups);
    if (IS_ERR(hwdev))
    return PTR_ERR(hwdev);
//
// Register the temperature sensors with the thermal framework
// to allow their usage in setting up the thermal zones from
// device tree.
//
// NOTE: Not all temperature sensors maybe used for thermal
// control
//
    INIT_LIST_HEAD(&scpi_sensors.thermal_zones);
    for (i = 0; i < nr_sensors; i++) {
    struct sensor_data *sensor = &scpi_sensors.data[i];
    struct thermal_zone_device *z;
    struct scpi_thermal_zone *zone;
    if (sensor.info.class != TEMPERATURE)
    continue;
    zone = devm_kzalloc(dev, sizeof(*zone), GFP_KERNEL);
    if (!zone)
    return -ENOMEM;
    zone.sensor_id = i;
    zone.scpi_sensors = scpi_sensors;
    z = devm_thermal_of_zone_register(dev,
    sensor.info.sensor_id,
    zone,
    &scpi_sensor_ops);
//
// The call to thermal_zone_of_sensor_register returns
// an error for sensors that are not associated with
// any thermal zones or if the thermal subsystem is
// not configured.
//
    if (IS_ERR(z))
    devm_kfree(dev, zone);
    }
    return 0;
    }
    static struct platform_driver scpi_hwmon_platdrv = {
    .driver = {
    .name	= "scpi-hwmon",
    .of_match_table = scpi_of_match,
    },
    .probe		= scpi_hwmon_probe,
    };
    module_platform_driver(scpi_hwmon_platdrv);
    MODULE_AUTHOR("Punit Agrawal <punit.agrawal@arm.com>");
    MODULE_DESCRIPTION("ARM SCPI HWMON interface driver");
    MODULE_LICENSE("GPL v2");
