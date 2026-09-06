//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/system76_acpi.c
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
// System76 ACPI Driver
//
// Copyright (C) 2023 System76
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

    enum kbled_type {
    KBLED_NONE,
    KBLED_WHITE,
    KBLED_RGB,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct system76_data {
    pub acpi_dev: *mut acpi_device,
    pub ap_led: led_classdev,
    pub kb_led: led_classdev,
    pub kb_brightness: enum led_brightness,
    pub kb_toggle_brightness: enum led_brightness,
    pub kb_color: c_int,
    pub therm: *mut device,
    pub nfan: *mut union acpi_object,
    pub ntmp: *mut union acpi_object,
    pub input: *mut input_dev,
    pub has_open_ec: bool,
    pub kbled_type: enum kbled_type,
}

    static const struct acpi_device_id device_ids[] = {
    {"17761776", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, device_ids);
// Array of keyboard LED brightness levels
    static const enum led_brightness kb_levels[] = {
    48,
    72,
    96,
    144,
    192,
    255
    };
// Array of keyboard LED colors in 24-bit RGB format
    static const int kb_colors[] = {
    0xFFFFFF,
    0x0000FF,
    0xFF0000,
    0xFF00FF,
    0x00FF00,
    0x00FFFF,
    0xFFFF00
    };
// Get a System76 ACPI device value by name
#[no_mangle]
unsafe extern "C" fn system76_get(data: *mut system76_data, method: *mut c_char) -> c_int {
    static int system76_get(struct system76_data *data, char *method)
    {
    acpi_handle handle;
    acpi_status status;
    let mut ret: c_ulonglong = 0;
    handle = acpi_device_handle(data.acpi_dev);
    status = acpi_evaluate_integer(handle, method, core::ptr::null_mut(), &ret);
    if (ACPI_SUCCESS(status))
    return ret;
    return -ENODEV;
    }
// Get a System76 ACPI device value by name with index
#[no_mangle]
unsafe extern "C" fn system76_get_index(data: *mut system76_data, method: *mut c_char, index: c_int) -> c_int {
    static int system76_get_index(struct system76_data *data, char *method, int index)
    {
    union acpi_object obj;
    struct acpi_object_list obj_list;
    acpi_handle handle;
    acpi_status status;
    let mut ret: c_ulonglong = 0;
    obj.type = ACPI_TYPE_INTEGER;
    obj.integer.value = index;
    obj_list.count = 1;
    obj_list.pointer = &obj;
    handle = acpi_device_handle(data.acpi_dev);
    status = acpi_evaluate_integer(handle, method, &obj_list, &ret);
    if (ACPI_SUCCESS(status))
    return ret;
    return -ENODEV;
    }
// Get a System76 ACPI device object by name
#[no_mangle]
unsafe extern "C" fn system76_get_object(data: *mut system76_data, method: *mut c_char, obj: *mut union acpi_object) -> c_int {
    static int system76_get_object(struct system76_data *data, char *method, union acpi_object **obj)
    {
    acpi_handle handle;
    acpi_status status;
    let mut buf: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    handle = acpi_device_handle(data.acpi_dev);
    status = acpi_evaluate_object(handle, method, core::ptr::null_mut(), &buf);
    if (ACPI_SUCCESS(status)) {
// obj = buf.pointer;
    return 0;
    }
    return -ENODEV;
    }
// Get a name from a System76 ACPI device object
    static char *system76_name(union acpi_object *obj, int index)
    {
    if (obj && obj.type == ACPI_TYPE_PACKAGE && index <= obj.package.count) {
    if (obj.package.elements[index].type == ACPI_TYPE_STRING)
    return obj.package.elements[index].string.pointer;
    }
    return core::ptr::null_mut();
    }
// Set a System76 ACPI device value by name
#[no_mangle]
unsafe extern "C" fn system76_set(data: *mut system76_data, method: *mut c_char, value: c_int) -> c_int {
    static int system76_set(struct system76_data *data, char *method, int value)
    {
    union acpi_object obj;
    struct acpi_object_list obj_list;
    acpi_handle handle;
    acpi_status status;
    obj.type = ACPI_TYPE_INTEGER;
    obj.integer.value = value;
    obj_list.count = 1;
    obj_list.pointer = &obj;
    handle = acpi_device_handle(data.acpi_dev);
    status = acpi_evaluate_object(handle, method, &obj_list, core::ptr::null_mut());
    if (ACPI_SUCCESS(status))
    return 0;
    else
    return -1;
    }
pub const BATTERY_THRESHOLD_INVALID: c_uint = 0xFF;
    enum {
    THRESHOLD_START,
    THRESHOLD_END,
    };
#[no_mangle]
unsafe extern "C" fn battery_get_threshold(which: c_int, buf: *mut c_char) -> isize {
    static ssize_t battery_get_threshold(int which, char *buf)
    {
    struct acpi_object_list input;
    union acpi_object param;
    acpi_handle handle;
    acpi_status status;
    let mut ret: c_ulonglong = BATTERY_THRESHOLD_INVALID;
    handle = ec_get_handle();
    if (!handle)
    return -ENODEV;
    input.count = 1;
    input.pointer = &param;
// Start/stop selection
    param.type = ACPI_TYPE_INTEGER;
    param.integer.value = which;
    status = acpi_evaluate_integer(handle, "GBCT", &input, &ret);
    if (ACPI_FAILURE(status))
    return -EIO;
    if (ret == BATTERY_THRESHOLD_INVALID)
    return -EINVAL;
    return sysfs_emit(buf, "%d\n", (int)ret);
    }
#[no_mangle]
unsafe extern "C" fn battery_set_threshold(which: c_int, buf: *const c_char, count: usize) -> isize {
    static ssize_t battery_set_threshold(int which, const char *buf, size_t count)
    {
    struct acpi_object_list input;
    union acpi_object params[2];
    acpi_handle handle;
    acpi_status status;
    unsigned int value;
    int ret;
    handle = ec_get_handle();
    if (!handle)
    return -ENODEV;
    ret = kstrtouint(buf, 10, &value);
    if (ret)
    return ret;
    if (value > 100)
    return -EINVAL;
    input.count = 2;
    input.pointer = params;
// Start/stop selection
    params[0].type = ACPI_TYPE_INTEGER;
    params[0].integer.value = which;
// Threshold value
    params[1].type = ACPI_TYPE_INTEGER;
    params[1].integer.value = value;
    status = acpi_evaluate_object(handle, "SBCT", &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EIO;
    return count;
    }
    static ssize_t charge_control_start_threshold_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return battery_get_threshold(THRESHOLD_START, buf);
    }
    static ssize_t charge_control_start_threshold_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return battery_set_threshold(THRESHOLD_START, buf, count);
    }
    static DEVICE_ATTR_RW(charge_control_start_threshold);
    static ssize_t charge_control_end_threshold_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return battery_get_threshold(THRESHOLD_END, buf);
    }
    static ssize_t charge_control_end_threshold_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return battery_set_threshold(THRESHOLD_END, buf, count);
    }
    static DEVICE_ATTR_RW(charge_control_end_threshold);
    static struct attribute *system76_battery_attrs[] = {
    &dev_attr_charge_control_start_threshold.attr,
    &dev_attr_charge_control_end_threshold.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(system76_battery);
#[no_mangle]
unsafe extern "C" fn system76_battery_add(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int system76_battery_add(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
// System76 EC only supports 1 battery
    if (strcmp(battery.desc.name, "BAT0") != 0)
    return -ENODEV;
    if (device_add_groups(&battery.dev, system76_battery_groups))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn system76_battery_remove(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int system76_battery_remove(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
    device_remove_groups(&battery.dev, system76_battery_groups);
    return 0;
    }
    static struct acpi_battery_hook system76_battery_hook = {
    .add_battery = system76_battery_add,
    .remove_battery = system76_battery_remove,
    .name = "System76 Battery Extension",
    };
#[no_mangle]
unsafe extern "C" fn system76_battery_init() {
    static void system76_battery_init(void)
    {
    battery_hook_register(&system76_battery_hook);
    }
#[no_mangle]
unsafe extern "C" fn system76_battery_exit() {
    static void system76_battery_exit(void)
    {
    battery_hook_unregister(&system76_battery_hook);
    }
// Get the airplane mode LED brightness
#[no_mangle]
unsafe extern "C" fn ap_led_get(led: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness ap_led_get(struct led_classdev *led)
    {
    struct system76_data *data;
    int value;
    data = container_of(led, struct system76_data, ap_led);
    value = system76_get(data, "GAPL");
    if (value > 0)
    return (enum led_brightness)value;
    else
    return LED_OFF;
    }
// Set the airplane mode LED brightness
#[no_mangle]
unsafe extern "C" fn ap_led_set(led: *mut led_classdev, value: enum led_brightness) -> c_int {
    static int ap_led_set(struct led_classdev *led, enum led_brightness value)
    {
    struct system76_data *data;
    data = container_of(led, struct system76_data, ap_led);
    return system76_set(data, "SAPL", value == LED_OFF ? 0 : 1);
    }
// Get the last set keyboard LED brightness
#[no_mangle]
unsafe extern "C" fn kb_led_get(led: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness kb_led_get(struct led_classdev *led)
    {
    struct system76_data *data;
    data = container_of(led, struct system76_data, kb_led);
    return data.kb_brightness;
    }
// Set the keyboard LED brightness
#[no_mangle]
unsafe extern "C" fn kb_led_set(led: *mut led_classdev, value: enum led_brightness) -> c_int {
    static int kb_led_set(struct led_classdev *led, enum led_brightness value)
    {
    struct system76_data *data;
    data = container_of(led, struct system76_data, kb_led);
    data.kb_brightness = value;
    if (acpi_has_method(acpi_device_handle(data.acpi_dev), "GKBK")) {
    return system76_set(data, "SKBB", (int)data.kb_brightness);
    } else {
    return system76_set(data, "SKBL", (int)data.kb_brightness);
    }
    }
// Get the last set keyboard LED color
    static ssize_t kb_led_color_show(
    struct device *dev,
    struct device_attribute *dev_attr,
    char *buf)
    {
    struct led_classdev *led;
    struct system76_data *data;
    led = dev_get_drvdata(dev);
    data = container_of(led, struct system76_data, kb_led);
    return sysfs_emit(buf, "%06X\n", data.kb_color);
    }
// Set the keyboard LED color
    static ssize_t kb_led_color_store(
    struct device *dev,
    struct device_attribute *dev_attr,
    const char *buf,
    size_t size)
    {
    struct led_classdev *led;
    struct system76_data *data;
    unsigned int val;
    int ret;
    led = dev_get_drvdata(dev);
    data = container_of(led, struct system76_data, kb_led);
    ret = kstrtouint(buf, 16, &val);
    if (ret)
    return ret;
    if (val > 0xFFFFFF)
    return -EINVAL;
    data.kb_color = (int)val;
    system76_set(data, "SKBC", data.kb_color);
    return size;
    }
    static struct device_attribute dev_attr_kb_led_color = {
    .attr = {
    .name = "color",
    .mode = 0644,
    },
    .show = kb_led_color_show,
    .store = kb_led_color_store,
    };
    static struct attribute *system76_kb_led_color_attrs[] = {
    &dev_attr_kb_led_color.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(system76_kb_led_color);
// Notify that the keyboard LED was changed by hardware
#[no_mangle]
unsafe extern "C" fn kb_led_notify(data: *mut system76_data) {
    static void kb_led_notify(struct system76_data *data)
    {
    led_classdev_notify_brightness_hw_changed(
    &data.kb_led,
    data.kb_brightness
    );
    }
// Read keyboard LED brightness as set by hardware
#[no_mangle]
unsafe extern "C" fn kb_led_hotkey_hardware(data: *mut system76_data) {
    static void kb_led_hotkey_hardware(struct system76_data *data)
    {
    int value;
    if (acpi_has_method(acpi_device_handle(data.acpi_dev), "GKBK")) {
    value = system76_get(data, "GKBB");
    } else {
    value = system76_get(data, "GKBL");
    }
    if (value < 0)
    return;
    data.kb_brightness = value;
    kb_led_notify(data);
    }
// Toggle the keyboard LED
#[no_mangle]
unsafe extern "C" fn kb_led_hotkey_toggle(data: *mut system76_data) {
    static void kb_led_hotkey_toggle(struct system76_data *data)
    {
    if (data.kb_brightness > 0) {
    data.kb_toggle_brightness = data.kb_brightness;
    kb_led_set(&data.kb_led, 0);
    } else {
    kb_led_set(&data.kb_led, data.kb_toggle_brightness);
    }
    kb_led_notify(data);
    }
// Decrease the keyboard LED brightness
#[no_mangle]
unsafe extern "C" fn kb_led_hotkey_down(data: *mut system76_data) {
    static void kb_led_hotkey_down(struct system76_data *data)
    {
    int i;
    if (data.kb_brightness > 0) {
    for (i = ARRAY_SIZE(kb_levels); i > 0; i--) {
    if (kb_levels[i - 1] < data.kb_brightness) {
    kb_led_set(&data.kb_led, kb_levels[i - 1]);
    break;
    }
    }
    } else {
    kb_led_set(&data.kb_led, data.kb_toggle_brightness);
    }
    kb_led_notify(data);
    }
// Increase the keyboard LED brightness
#[no_mangle]
unsafe extern "C" fn kb_led_hotkey_up(data: *mut system76_data) {
    static void kb_led_hotkey_up(struct system76_data *data)
    {
    int i;
    if (data.kb_brightness > 0) {
    for (i = 0; i < ARRAY_SIZE(kb_levels); i++) {
    if (kb_levels[i] > data.kb_brightness) {
    kb_led_set(&data.kb_led, kb_levels[i]);
    break;
    }
    }
    } else {
    kb_led_set(&data.kb_led, data.kb_toggle_brightness);
    }
    kb_led_notify(data);
    }
// Cycle the keyboard LED color
#[no_mangle]
unsafe extern "C" fn kb_led_hotkey_color(data: *mut system76_data) {
    static void kb_led_hotkey_color(struct system76_data *data)
    {
    int i;
    if (data.kbled_type != KBLED_RGB)
    return;
    if (data.kb_brightness > 0) {
    for (i = 0; i < ARRAY_SIZE(kb_colors); i++) {
    if (kb_colors[i] == data.kb_color)
    break;
    }
    i += 1;
    if (i >= ARRAY_SIZE(kb_colors))
    i = 0;
    data.kb_color = kb_colors[i];
    system76_set(data, "SKBC", data.kb_color);
    } else {
    kb_led_set(&data.kb_led, data.kb_toggle_brightness);
    }
    kb_led_notify(data);
    }
    static umode_t thermal_is_visible(const void *drvdata, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct system76_data *data = drvdata;
    switch (type) {
    case hwmon_fan:
    case hwmon_pwm:
    if (system76_name(data.nfan, channel))
    return 0444;
    break;
    case hwmon_temp:
    if (system76_name(data.ntmp, channel))
    return 0444;
    break;
    default:
    return 0;
    }
    return 0;
    }
    static int thermal_read(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    struct system76_data *data = dev_get_drvdata(dev);
    int raw;
    switch (type) {
    case hwmon_fan:
    if (attr == hwmon_fan_input) {
    raw = system76_get_index(data, "GFAN", channel);
    if (raw < 0)
    return raw;
// val = (raw >> 8) & 0xFFFF;
    return 0;
    }
    break;
    case hwmon_pwm:
    if (attr == hwmon_pwm_input) {
    raw = system76_get_index(data, "GFAN", channel);
    if (raw < 0)
    return raw;
// val = raw & 0xFF;
    return 0;
    }
    break;
    case hwmon_temp:
    if (attr == hwmon_temp_input) {
    raw = system76_get_index(data, "GTMP", channel);
    if (raw < 0)
    return raw;
// val = raw * 1000;
    return 0;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
    static int thermal_read_string(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    struct system76_data *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_fan:
    if (attr == hwmon_fan_label) {
// str = system76_name(data->nfan, channel);
    if (*str)
    return 0;
    }
    break;
    case hwmon_temp:
    if (attr == hwmon_temp_label) {
// str = system76_name(data->ntmp, channel);
    if (*str)
    return 0;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
    static const struct hwmon_ops thermal_ops = {
    .is_visible = thermal_is_visible,
    .read = thermal_read,
    .read_string = thermal_read_string,
    };
// Allocate up to 8 fans and temperatures
    static const struct hwmon_channel_info * const thermal_channel_info[] = {
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL,
    HWMON_F_INPUT | HWMON_F_LABEL),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info thermal_chip_info = {
    .ops = &thermal_ops,
    .info = thermal_channel_info,
    };
#[no_mangle]
unsafe extern "C" fn input_key(data: *mut system76_data, code: c_uint) {
    static void input_key(struct system76_data *data, unsigned int code)
    {
    input_report_key(data.input, code, 1);
    input_sync(data.input);
    input_report_key(data.input, code, 0);
    input_sync(data.input);
    }
// Handle ACPI notification
#[no_mangle]
unsafe extern "C" fn system76_notify(handle: acpi_handle, event: u32, context: *mut c_void) {
    static void system76_notify(acpi_handle handle, u32 event, void *context)
    {
    struct system76_data *data = context;
    switch (event) {
    case 0x80:
    kb_led_hotkey_hardware(data);
    break;
    case 0x81:
    kb_led_hotkey_toggle(data);
    break;
    case 0x82:
    kb_led_hotkey_down(data);
    break;
    case 0x83:
    kb_led_hotkey_up(data);
    break;
    case 0x84:
    kb_led_hotkey_color(data);
    break;
    case 0x85:
    input_key(data, KEY_SCREENLOCK);
    break;
    }
    }
// Probe a System76 platform device
#[no_mangle]
unsafe extern "C" fn system76_probe(pdev: *mut platform_device) -> c_int {
    static int system76_probe(struct platform_device *pdev)
    {
    struct acpi_device *acpi_dev;
    struct system76_data *data;
    int err;
    acpi_dev = ACPI_COMPANION(&pdev.dev);
    if (!acpi_dev)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.acpi_dev = acpi_dev;
// Some models do not run open EC firmware. Check for an ACPI method
// that only exists on open EC to guard functionality specific to it.
    data.has_open_ec = acpi_has_method(acpi_device_handle(data.acpi_dev), "NFAN");
    err = system76_get(data, "INIT");
    if (err)
    return err;
    data.ap_led.name = "system76_acpi::airplane";
    data.ap_led.flags = LED_CORE_SUSPENDRESUME;
    data.ap_led.brightness_get = ap_led_get;
    data.ap_led.brightness_set_blocking = ap_led_set;
    data.ap_led.max_brightness = 1;
    data.ap_led.default_trigger = "rfkill-none";
    err = devm_led_classdev_register(&pdev.dev, &data.ap_led);
    if (err)
    return err;
    data.kb_led.name = "system76_acpi::kbd_backlight";
    data.kb_led.flags = LED_BRIGHT_HW_CHANGED | LED_CORE_SUSPENDRESUME;
    data.kb_led.brightness_get = kb_led_get;
    data.kb_led.brightness_set_blocking = kb_led_set;
    if (acpi_has_method(acpi_device_handle(data.acpi_dev), "GKBK")) {
// Use the new ACPI methods
    data.kbled_type = system76_get(data, "GKBK");
    switch (data.kbled_type) {
    case KBLED_NONE:
// Nothing to do: Device will not be registered.
    break;
    case KBLED_WHITE:
    data.kb_led.max_brightness = 255;
    data.kb_toggle_brightness = 72;
    break;
    case KBLED_RGB:
    data.kb_led.max_brightness = 255;
    data.kb_led.groups = system76_kb_led_color_groups;
    data.kb_toggle_brightness = 72;
    data.kb_color = 0xffffff;
    system76_set(data, "SKBC", data.kb_color);
    break;
    }
    } else {
// Use the old ACPI methods
    if (acpi_has_method(acpi_device_handle(data.acpi_dev), "SKBC")) {
    data.kbled_type = KBLED_RGB;
    data.kb_led.max_brightness = 255;
    data.kb_led.groups = system76_kb_led_color_groups;
    data.kb_toggle_brightness = 72;
    data.kb_color = 0xffffff;
    system76_set(data, "SKBC", data.kb_color);
    } else {
    data.kbled_type = KBLED_WHITE;
    data.kb_led.max_brightness = 5;
    }
    }
    if (data.kbled_type != KBLED_NONE) {
    err = devm_led_classdev_register(&pdev.dev, &data.kb_led);
    if (err)
    return err;
    }
    data.input = devm_input_allocate_device(&pdev.dev);
    if (!data.input)
    return -ENOMEM;
    data.input.name = "System76 ACPI Hotkeys";
    data.input.phys = "system76_acpi/input0";
    data.input.id.bustype = BUS_HOST;
    data.input.dev.parent = &pdev.dev;
    input_set_capability(data.input, EV_KEY, KEY_SCREENLOCK);
    err = input_register_device(data.input);
    if (err)
    return err;
    err = acpi_dev_install_notify_handler(acpi_dev, ACPI_DEVICE_NOTIFY,
    system76_notify, data);
    if (err)
    return err;
    if (data.has_open_ec) {
    err = system76_get_object(data, "NFAN", &data.nfan);
    if (err)
    goto error;
    err = system76_get_object(data, "NTMP", &data.ntmp);
    if (err)
    goto error;
    data.therm = devm_hwmon_device_register_with_info(&pdev.dev,
    "system76_acpi", data, &thermal_chip_info, core::ptr::null_mut());
    err = PTR_ERR_OR_ZERO(data.therm);
    if (err)
    goto error;
    system76_battery_init();
    }
    return 0;
    error:
    if (data.has_open_ec) {
    kfree(data.ntmp);
    kfree(data.nfan);
    }
    acpi_dev_remove_notify_handler(acpi_dev, ACPI_DEVICE_NOTIFY, system76_notify);
    return err;
    }
// Remove a System76 platform device
#[no_mangle]
unsafe extern "C" fn system76_remove(pdev: *mut platform_device) {
    static void system76_remove(struct platform_device *pdev)
    {
    struct system76_data *data = platform_get_drvdata(pdev);
    if (data.has_open_ec) {
    system76_battery_exit();
    kfree(data.nfan);
    kfree(data.ntmp);
    }
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev),
    ACPI_DEVICE_NOTIFY, system76_notify);
    system76_get(data, "FINI");
    }
    static struct platform_driver system76_driver = {
    .probe = system76_probe,
    .remove = system76_remove,
    .driver = {
    .name = "System76 ACPI Driver",
    .acpi_match_table = device_ids,
    },
    };
    module_platform_driver(system76_driver);
    MODULE_DESCRIPTION("System76 ACPI Driver");
    MODULE_AUTHOR("Jeremy Soller <jeremy@system76.com>");
    MODULE_LICENSE("GPL");
