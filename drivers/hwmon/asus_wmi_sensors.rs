//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/asus_wmi_sensors.c
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
// HWMON driver for ASUS motherboards that provides sensor readouts via WMI
// interface present in the UEFI of the X370/X470/B450/X399 Ryzen motherboards.
//
// Copyright (C) 2018-2019 Ed Brindley <kernel@maidavale.org>
//
// WMI interface provides:
// - CPU Core Voltage,
// - CPU SOC Voltage,
// - DRAM Voltage,
// - VDDP Voltage,
// - 1.8V PLL Voltage,
// - +12V Voltage,
// - +5V Voltage,
// - 3VSB Voltage,
// - VBAT Voltage,
// - AVCC3 Voltage,
// - SB 1.05V Voltage,
// - CPU Core Voltage,
// - CPU SOC Voltage,
// - DRAM Voltage,
// - CPU Fan RPM,
// - Chassis Fan 1 RPM,
// - Chassis Fan 2 RPM,
// - Chassis Fan 3 RPM,
// - HAMP Fan RPM,
// - Water Pump RPM,
// - CPU OPT RPM,
// - Water Flow RPM,
// - AIO Pump RPM,
// - CPU Temperature,
// - CPU Socket Temperature,
// - Motherboard Temperature,
// - Chipset Temperature,
// - Tsensor 1 Temperature,
// - CPU VRM Temperature,
// - Water In,
// - Water Out,
// - CPU VRM Output Current.
//

pub const ASUSWMI_METHODID_GET_VALUE: c_uint = 0x52574543 /* RWEC */;
pub const ASUSWMI_METHODID_UPDATE_BUFFER: c_uint = 0x51574543 /* QWEC */;
pub const ASUSWMI_METHODID_GET_INFO: c_uint = 0x50574543 /* PWEC */;
pub const ASUSWMI_METHODID_GET_NUMBER: c_uint = 0x50574572 /* PWEr */;
pub const ASUSWMI_METHODID_GET_VERSION: c_uint = 0x50574574 /* PWEt */;
pub const ASUS_WMI_MAX_STR_SIZE: c_int = 32;

    .matches = {								\
    DMI_EXACT_MATCH(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC."),	\
    DMI_EXACT_MATCH(DMI_BOARD_NAME, name),				\
    },									\
    }
    static const struct dmi_system_id asus_wmi_dmi_table[] = {
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME X399-A"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("PRIME X470-PRO"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VI EXTREME"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("CROSSHAIR VI HERO"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VI HERO (WI-FI AC)"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VII HERO"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG CROSSHAIR VII HERO (WI-FI)"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B450-E GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B450-F GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B450-F GAMING II"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX B450-I GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X399-E GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X470-F GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG STRIX X470-I GAMING"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG ZENITH EXTREME"),
    DMI_EXACT_MATCH_ASUS_BOARD_NAME("ROG ZENITH EXTREME ALPHA"),
    {}
    };
    MODULE_DEVICE_TABLE(dmi, asus_wmi_dmi_table);
    enum asus_wmi_sensor_class {
    VOLTAGE		= 0x0,
    TEMPERATURE_C	= 0x1,
    FAN_RPM		= 0x2,
    CURRENT		= 0x3,
    WATER_FLOW	= 0x4,
    };
    enum asus_wmi_location {
    CPU		= 0x0,
    CPU_SOC		= 0x1,
    DRAM		= 0x2,
    MOTHERBOARD	= 0x3,
    CHIPSET		= 0x4,
    AUX		= 0x5,
    VRM		= 0x6,
    COOLER		= 0x7
    };
    enum asus_wmi_type {
    SIGNED_INT	= 0x0,
    UNSIGNED_INT	= 0x1,
    SCALED		= 0x3,
    };
    enum asus_wmi_source {
    SIO		= 0x1,
    EC		= 0x2
    };
    static enum hwmon_sensor_types asus_data_types[] = {
    [VOLTAGE]	= hwmon_in,
    [TEMPERATURE_C]	= hwmon_temp,
    [FAN_RPM]	= hwmon_fan,
    [CURRENT]	= hwmon_curr,
    [WATER_FLOW]	= hwmon_fan,
    };
    static u32 hwmon_attributes[hwmon_max] = {
    [hwmon_chip]	= HWMON_C_REGISTER_TZ,
    [hwmon_temp]	= HWMON_T_INPUT | HWMON_T_LABEL,
    [hwmon_in]	= HWMON_I_INPUT | HWMON_I_LABEL,
    [hwmon_curr]	= HWMON_C_INPUT | HWMON_C_LABEL,
    [hwmon_fan]	= HWMON_F_INPUT | HWMON_F_LABEL,
    };
//
// struct asus_wmi_sensor_info - sensor info.
// @id: sensor id.
// @data_type: sensor class e.g. voltage, temp etc.
// @location: sensor location.
// @name: sensor name.
// @source: sensor source.
// @type: sensor type signed, unsigned etc.
// @cached_value: cached sensor value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_wmi_sensor_info {
    pub id: u32,
    pub data_type: c_int,
    pub location: c_int,
    pub name: [c_char; ASUS_WMI_MAX_STR_SIZE],
    pub source: c_int,
    pub type: c_int,
    pub cached_value: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_wmi_wmi_info {
    pub /: *mut *mut unsigned long source_last_updated[3]; / in jiffies,
    pub sensor_count: c_int,
    pub info: [*const asus_wmi_sensor_info; hwmon_max],
    pub info_by_id: *mut asus_wmi_sensor_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_wmi_sensors {
    pub wmi: asus_wmi_wmi_info,
// lock access to internal cache
    pub lock: mutex,
}

//
// Universal method for calling WMI method
//
#[no_mangle]
unsafe extern "C" fn asus_wmi_call_method(method_id: u32, args: *mut u32, output: *mut acpi_buffer) -> c_int {
    static int asus_wmi_call_method(u32 method_id, u32 *args, struct acpi_buffer *output)
    {
    let mut input: acpi_buffer = {(acpi_size) sizeof(*args), args };
    acpi_status status;
    status = wmi_evaluate_method(ASUSWMI_MONITORING_GUID, 0,
    method_id, &input, output);
    if (ACPI_FAILURE(status))
    return -EIO;
    return 0;
    }
//
// Gets the version of the ASUS sensors interface implemented
//
#[no_mangle]
unsafe extern "C" fn asus_wmi_get_version(version: *mut u32) -> c_int {
    static int asus_wmi_get_version(u32 *version)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u32 args[] = {0, 0, 0};
    union acpi_object *obj;
    int err;
    err = asus_wmi_call_method(ASUSWMI_METHODID_GET_VERSION, args, &output);
    if (err)
    return err;
    obj = output.pointer;
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    err = 0;
// version = obj->integer.value;
    out_free_obj:
    ACPI_FREE(obj);
    return err;
    }
//
// Gets the number of sensor items
//
#[no_mangle]
unsafe extern "C" fn asus_wmi_get_item_count(count: *mut u32) -> c_int {
    static int asus_wmi_get_item_count(u32 *count)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u32 args[] = {0, 0, 0};
    union acpi_object *obj;
    int err;
    err = asus_wmi_call_method(ASUSWMI_METHODID_GET_NUMBER, args, &output);
    if (err)
    return err;
    obj = output.pointer;
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    err = 0;
// count = obj->integer.value;
    out_free_obj:
    ACPI_FREE(obj);
    return err;
    }
    static int asus_wmi_hwmon_add_chan_info(struct hwmon_channel_info *asus_wmi_hwmon_chan,
    struct device *dev, int num,
    enum hwmon_sensor_types type, u32 config)
    {
    u32 *cfg;
    cfg = devm_kcalloc(dev, num + 1, sizeof(*cfg), GFP_KERNEL);
    if (!cfg)
    return -ENOMEM;
    asus_wmi_hwmon_chan.type = type;
    asus_wmi_hwmon_chan.config = cfg;
    memset32(cfg, config, num);
    return 0;
    }
//
// For a given sensor item returns details e.g. type (voltage/temperature/fan speed etc), bank etc
//
#[no_mangle]
unsafe extern "C" fn asus_wmi_sensor_info(index: c_int, s: *mut asus_wmi_sensor_info) -> c_int {
    static int asus_wmi_sensor_info(int index, struct asus_wmi_sensor_info *s)
    {
    union acpi_object name_obj, data_type_obj, location_obj, source_obj, type_obj;
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u32 args[] = {index, 0};
    union acpi_object *obj;
    int err;
    err = asus_wmi_call_method(ASUSWMI_METHODID_GET_INFO, args, &output);
    if (err)
    return err;
    s.id = index;
    obj = output.pointer;
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_PACKAGE) {
    err = -EIO;
    goto out_free_obj;
    }
    if (obj.package.count != 5) {
    err = -EIO;
    goto out_free_obj;
    }
    name_obj = obj.package.elements[0];
    if (name_obj.type != ACPI_TYPE_STRING) {
    err = -EIO;
    goto out_free_obj;
    }
    strscpy(s.name, name_obj.string.pointer, sizeof(s.name));
    data_type_obj = obj.package.elements[1];
    if (data_type_obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    s.data_type = data_type_obj.integer.value;
    location_obj = obj.package.elements[2];
    if (location_obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    s.location = location_obj.integer.value;
    source_obj = obj.package.elements[3];
    if (source_obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    s.source = source_obj.integer.value;
    type_obj = obj.package.elements[4];
    if (type_obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    err = 0;
    s.type = type_obj.integer.value;
    out_free_obj:
    ACPI_FREE(obj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn asus_wmi_update_buffer(source: c_int) -> c_int {
    static int asus_wmi_update_buffer(int source)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u32 args[] = {source, 0};
    return asus_wmi_call_method(ASUSWMI_METHODID_UPDATE_BUFFER, args, &output);
    }
#[no_mangle]
unsafe extern "C" fn asus_wmi_get_sensor_value(index: u8, value: *mut c_long) -> c_int {
    static int asus_wmi_get_sensor_value(u8 index, long *value)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u32 args[] = {index, 0};
    union acpi_object *obj;
    int err;
    err = asus_wmi_call_method(ASUSWMI_METHODID_GET_VALUE, args, &output);
    if (err)
    return err;
    obj = output.pointer;
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_INTEGER) {
    err = -EIO;
    goto out_free_obj;
    }
    err = 0;
// value = obj->integer.value;
    out_free_obj:
    ACPI_FREE(obj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn asus_wmi_update_values_for_source(source: u8, sensor_data: *mut asus_wmi_sensors) -> c_int {
    static int asus_wmi_update_values_for_source(u8 source, struct asus_wmi_sensors *sensor_data)
    {
    struct asus_wmi_sensor_info *sensor;
    let mut value: c_long = 0;
    int ret;
    int i;
    for (i = 0; i < sensor_data.wmi.sensor_count; i++) {
    sensor = sensor_data.wmi.info_by_id[i];
    if (sensor && sensor.source == source) {
    ret = asus_wmi_get_sensor_value(sensor.id, &value);
    if (ret)
    return ret;
    sensor.cached_value = value;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asus_wmi_scale_sensor_value(value: u32, data_type: c_int) -> c_int {
    static int asus_wmi_scale_sensor_value(u32 value, int data_type)
    {
// FAN_RPM and WATER_FLOW don't need scaling
    switch (data_type) {
    case VOLTAGE:
// value in microVolts
    return DIV_ROUND_CLOSEST(value,  KILO);
    case TEMPERATURE_C:
// value in Celsius
    return value * MILLIDEGREE_PER_DEGREE;
    case CURRENT:
// value in Amperes
    return value * MILLI;
    }
    return value;
    }
    static int asus_wmi_get_cached_value_or_update(const struct asus_wmi_sensor_info *sensor,
    struct asus_wmi_sensors *sensor_data,
    u32 *value)
    {
    let mut ret: c_int = 0;
    mutex_lock(&sensor_data.lock);
    if (time_after(jiffies, sensor_data.wmi.source_last_updated[sensor.source] + HZ)) {
    ret = asus_wmi_update_buffer(sensor.source);
    if (ret)
    goto unlock;
    ret = asus_wmi_update_values_for_source(sensor.source, sensor_data);
    if (ret)
    goto unlock;
    sensor_data.wmi.source_last_updated[sensor.source] = jiffies;
    }
// value = sensor->cached_value;
    unlock:
    mutex_unlock(&sensor_data.lock);
    return ret;
    }
// Now follow the functions that implement the hwmon interface
    static int asus_wmi_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    const struct asus_wmi_sensor_info *sensor;
    let mut value: u32 = 0;
    int ret;
    struct asus_wmi_sensors *sensor_data = dev_get_drvdata(dev);
    sensor = *(sensor_data.wmi.info[type] + channel);
    ret = asus_wmi_get_cached_value_or_update(sensor, sensor_data, &value);
    if (ret)
    return ret;
// val = asus_wmi_scale_sensor_value(value, sensor->data_type);
    return ret;
    }
    static int asus_wmi_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    struct asus_wmi_sensors *sensor_data = dev_get_drvdata(dev);
    const struct asus_wmi_sensor_info *sensor;
    sensor = *(sensor_data.wmi.info[type] + channel);
// str = sensor->name;
    return 0;
    }
    static umode_t asus_wmi_hwmon_is_visible(const void *drvdata,
    enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    const struct asus_wmi_sensors *sensor_data = drvdata;
    const struct asus_wmi_sensor_info *sensor;
    sensor = *(sensor_data.wmi.info[type] + channel);
    if (sensor)
    return 0444;
    return 0;
    }
    static const struct hwmon_ops asus_wmi_hwmon_ops = {
    .is_visible = asus_wmi_hwmon_is_visible,
    .read = asus_wmi_hwmon_read,
    .read_string = asus_wmi_hwmon_read_string,
    };
    static struct hwmon_chip_info asus_wmi_chip_info = {
    .ops = &asus_wmi_hwmon_ops,
    .info = core::ptr::null_mut(),
    };
    static int asus_wmi_configure_sensor_setup(struct device *dev,
    struct asus_wmi_sensors *sensor_data)
    {
    const struct hwmon_channel_info **ptr_asus_wmi_ci;
    struct hwmon_channel_info *asus_wmi_hwmon_chan;
    int nr_count[hwmon_max] = {}, nr_types = 0;
    struct asus_wmi_sensor_info *temp_sensor;
    const struct hwmon_chip_info *chip_info;
    enum hwmon_sensor_types type;
    struct device *hwdev;
    int i, idx;
    int err;
    for (i = 0; i < sensor_data.wmi.sensor_count; i++) {
    struct asus_wmi_sensor_info sensor;
    err = asus_wmi_sensor_info(i, &sensor);
    if (err)
    return err;
    switch (sensor.data_type) {
    case TEMPERATURE_C:
    case VOLTAGE:
    case CURRENT:
    case FAN_RPM:
    case WATER_FLOW:
    type = asus_data_types[sensor.data_type];
    if (!nr_count[type])
    nr_types++;
    nr_count[type]++;
    break;
    }
    }
    if (nr_count[hwmon_temp])
    nr_count[hwmon_chip]++, nr_types++;
    asus_wmi_hwmon_chan = devm_kcalloc(dev, nr_types,
    sizeof(*asus_wmi_hwmon_chan),
    GFP_KERNEL);
    if (!asus_wmi_hwmon_chan)
    return -ENOMEM;
    ptr_asus_wmi_ci = devm_kcalloc(dev, nr_types + 1,
    sizeof(*ptr_asus_wmi_ci), GFP_KERNEL);
    if (!ptr_asus_wmi_ci)
    return -ENOMEM;
    asus_wmi_chip_info.info = ptr_asus_wmi_ci;
    chip_info = &asus_wmi_chip_info;
    sensor_data.wmi.info_by_id = devm_kcalloc(dev, sensor_data.wmi.sensor_count,
    sizeof(*sensor_data.wmi.info_by_id),
    GFP_KERNEL);
    if (!sensor_data.wmi.info_by_id)
    return -ENOMEM;
    for (type = 0; type < hwmon_max; type++) {
    if (!nr_count[type])
    continue;
    err = asus_wmi_hwmon_add_chan_info(asus_wmi_hwmon_chan, dev,
    nr_count[type], type,
    hwmon_attributes[type]);
    if (err)
    return err;
// ptr_asus_wmi_ci++ = asus_wmi_hwmon_chan++;
    sensor_data.wmi.info[type] = devm_kcalloc(dev,
    nr_count[type],
    sizeof(*sensor_data.wmi.info),
    GFP_KERNEL);
    if (!sensor_data.wmi.info[type])
    return -ENOMEM;
    }
    for (i = sensor_data.wmi.sensor_count - 1; i >= 0; i--) {
    temp_sensor = devm_kzalloc(dev, sizeof(*temp_sensor), GFP_KERNEL);
    if (!temp_sensor)
    return -ENOMEM;
    err = asus_wmi_sensor_info(i, temp_sensor);
    if (err)
    continue;
    switch (temp_sensor.data_type) {
    case TEMPERATURE_C:
    case VOLTAGE:
    case CURRENT:
    case FAN_RPM:
    case WATER_FLOW:
    type = asus_data_types[temp_sensor.data_type];
    idx = --nr_count[type];
// (sensor_data->wmi.info[type] + idx) = temp_sensor;
    sensor_data.wmi.info_by_id[i] = temp_sensor;
    break;
    }
    }
    dev_dbg(dev, "board has %d sensors",
    sensor_data.wmi.sensor_count);
    hwdev = devm_hwmon_device_register_with_info(dev, "asus_wmi_sensors",
    sensor_data, chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwdev);
    }
#[no_mangle]
unsafe extern "C" fn asus_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int asus_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct asus_wmi_sensors *sensor_data;
    struct device *dev = &wdev.dev;
    let mut version: u32 = 0;
    if (!dmi_check_system(asus_wmi_dmi_table))
    return -ENODEV;
    sensor_data = devm_kzalloc(dev, sizeof(*sensor_data), GFP_KERNEL);
    if (!sensor_data)
    return -ENOMEM;
    if (asus_wmi_get_version(&version))
    return -ENODEV;
    if (asus_wmi_get_item_count(&sensor_data.wmi.sensor_count))
    return -ENODEV;
    if (sensor_data.wmi.sensor_count  <= 0 || version < 2) {
    dev_info(dev, "version: %u with %d sensors is unsupported\n",
    version, sensor_data.wmi.sensor_count);
    return -ENODEV;
    }
    mutex_init(&sensor_data.lock);
    dev_set_drvdata(dev, sensor_data);
    return asus_wmi_configure_sensor_setup(dev, sensor_data);
    }
    static const struct wmi_device_id asus_wmi_id_table[] = {
    { ASUSWMI_MONITORING_GUID, core::ptr::null_mut() },
    { }
    };
    static struct wmi_driver asus_sensors_wmi_driver = {
    .driver = {
    .name = "asus_wmi_sensors",
    },
    .id_table = asus_wmi_id_table,
    .probe = asus_wmi_probe,
    };
    module_wmi_driver(asus_sensors_wmi_driver);
    MODULE_AUTHOR("Ed Brindley <kernel@maidavale.org>");
    MODULE_DESCRIPTION("Asus WMI Sensors Driver");
    MODULE_LICENSE("GPL");
