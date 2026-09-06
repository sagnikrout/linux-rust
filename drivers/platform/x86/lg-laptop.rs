//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/lg-laptop.c
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
// lg-laptop.c - LG Gram ACPI features and hotkeys Driver
//
// Copyright (C) 2018 Matan Ziv-Av <matan@svgalib.org>
//

    .name           = __stringify(_name),   \
    .max_brightness = max,                  \
    .brightness_set = _name##_set,          \
    .brightness_get = _name##_get,          \
    .flags = flag,                          \
    }
    MODULE_AUTHOR("Matan Ziv-Av");
    MODULE_DESCRIPTION("LG WMI Hotkey Driver");
    MODULE_LICENSE("GPL");
    static bool fw_debug;
    module_param(fw_debug, bool, 0);
    MODULE_PARM_DESC(fw_debug, "Enable printing of firmware debug messages");
pub const LG_ADDRESS_SPACE_ID: c_uint = 0x8F;
pub const LG_ADDRESS_SPACE_DEBUG_FLAG_ADR: c_uint = 0x00;
pub const LG_ADDRESS_SPACE_HD_AUDIO_POWER_ADDR: c_uint = 0x01;
pub const LG_ADDRESS_SPACE_FAN_MODE_ADR: c_uint = 0x03;
pub const LG_ADDRESS_SPACE_DTTM_FLAG_ADR: c_uint = 0x20;
pub const LG_ADDRESS_SPACE_CPU_TEMP_ADR: c_uint = 0x21;
pub const LG_ADDRESS_SPACE_CPU_TRIP_LOW_ADR: c_uint = 0x22;
pub const LG_ADDRESS_SPACE_CPU_TRIP_HIGH_ADR: c_uint = 0x23;
pub const LG_ADDRESS_SPACE_MB_TEMP_ADR: c_uint = 0x24;
pub const LG_ADDRESS_SPACE_MB_TRIP_LOW_ADR: c_uint = 0x25;
pub const LG_ADDRESS_SPACE_MB_TRIP_HIGH_ADR: c_uint = 0x26;
pub const LG_ADDRESS_SPACE_DEBUG_MSG_START_ADR: c_uint = 0x3E8;
pub const LG_ADDRESS_SPACE_DEBUG_MSG_END_ADR: c_uint = 0x5E8;
pub const LG_NOTIFY_TABLET_MODE_OFF: c_uint = 0x50;
pub const LG_NOTIFY_TABLET_MODE_ON: c_uint = 0x51;
pub const LG_NOTIFY_HOTKEY: c_uint = 0x80;
pub const LG_NOTIFY_THERMAL: c_uint = 0x81;
pub const LG_NOTIFY_MISC: c_uint = 0x82;
pub const LG_OREP_READ_EC: c_int = 0;
pub const LG_OREP_WRITE_EC: c_int = 1;
pub const LG_OREP_DEBUG: c_int = 2;
pub const LG_OREP_UPDATE_SYSTEM_STATE: c_int = 3;
pub const LG_OREP_INTERCEPT_WMI_EVENTS: c_int = 4;
pub const LG_OREP_WAKE_ON_LAN: c_int = 6;

pub const GOV_TLED: c_uint = 0x2020008;
pub const WM_GET: c_int = 1;
pub const WM_SET: c_int = 2;
pub const WM_KEY_LIGHT: c_uint = 0x400;
pub const WM_TLED: c_uint = 0x404;
pub const WM_FN_LOCK: c_uint = 0x407;
pub const WM_BATT_LIMIT: c_uint = 0x61;
pub const WM_READER_MODE: c_uint = 0xBF;
pub const WM_FAN_MODE: c_uint = 0x33;
pub const WMBB_USB_CHARGE: c_uint = 0x10B;
pub const WMBB_BATT_LIMIT: c_uint = 0x10C;

pub const KBD_LED_BRIGHTNESS_OFF: c_uint = 0x0;
pub const KBD_LED_BRIGHTNESS_HALF: c_uint = 0x2;
pub const KBD_LED_BRIGHTNESS_FULL: c_uint = 0x4;

pub const KBD_LED_MODE_OFF: c_uint = 0x0;
pub const KBD_LED_MODE_ON: c_uint = 0x1;

// Exact purpose is unknown, maybe some sort of brightness limit?
pub const KBD_LED_MAGIC: c_uint = 0x05;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lg_wmab_buffer_result {
    pub value: __le32,
    pub status: __le32,
    pub __packed: },
    pub pf_device: *mut static struct platform_device,
    pub battery_limit_use_wmbb: static int,
    pub kbd_backlight_available: static bool,
    pub kbd_backlight: static struct led_classdev,
    pub dev): *mut static enum led_brightness get_kbd_backlight_level(struct device,
    static const struct key_entry wmi_keymap[] = {
// Placeholder value send by multiple hotkeys handled by ACPI
    { KE_IGNORE,	0x0,		{ KEY_UNKNOWN }},
// LG control panel
    { KE_KEY,	0x70,		{ KEY_F15 }},
// Touchpad toggle
    { KE_KEY,	0x74,		{ KEY_F21 }},
// Mute Audio, already handled by ACPI
    { KE_IGNORE,	0x78,		{ KEY_MUTE }},
// Read mode
    { KE_KEY,	0xf020000,	{ KEY_F14 }},
// Open settings
    { KE_KEY,	0xf070002,	{ KEY_CONFIG }},
// Keyboard backlight - pressing this key both sends an event and changes backlight level
    { KE_KEY,	0x10000000,	{ KEY_F16 }},
// Hotkey combination pressed
    { KE_IGNORE,	0x30010000,	{ KEY_UNKNOWN }},
// Hotkey combination released
    { KE_IGNORE,	0x30010001,	{ KEY_UNKNOWN }},
// Change fan mode
    { KE_KEY,	0x30010051,	{ KEY_PERFORMANCE }},
// Disable camera
    { KE_KEY,	0x40020000,	{ KEY_CAMERA_ACCESS_TOGGLE }},
// Mute microphone
    { KE_KEY,	0x40020001,	{ KEY_MICMUTE }},
// Fn-Lock
    { KE_KEY,	0x40030001,	{ KEY_FN_ESC }},
    {KE_END, 0}
}

    static int lg_laptop_execute_orep(acpi_handle handle, u64 command, u64 value,
    unsigned long long *result)
    {
    union acpi_object objs[] = {
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = command,
    },
    },
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = value,
    },
    }
    };
    struct acpi_object_list args = {
    .count = ARRAY_SIZE(objs),
    .pointer = objs,
    };
    acpi_status status;
    status = acpi_evaluate_integer(handle, "OREP", &args, result);
    if (ACPI_FAILURE(status))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ggov(arg0: u32) -> c_int {
    static int ggov(u32 arg0)
    {
    union acpi_object args[1];
    union acpi_object *r;
    acpi_status status;
    acpi_handle handle;
    struct acpi_object_list arg;
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    int res;
    args[0].type = ACPI_TYPE_INTEGER;
    args[0].integer.value = arg0;
    status = acpi_get_handle(core::ptr::null_mut(), (acpi_string) SB_GGOV_METHOD, &handle);
    if (ACPI_FAILURE(status)) {
    pr_err("Cannot get handle");
    return -ENODEV;
    }
    arg.count = 1;
    arg.pointer = args;
    status = acpi_evaluate_object(handle, core::ptr::null_mut(), &arg, &buffer);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "GGOV: call failed.\n");
    return -EINVAL;
    }
    r = buffer.pointer;
    if (r.type != ACPI_TYPE_INTEGER) {
    kfree(r);
    return -EINVAL;
    }
    res = r.integer.value;
    kfree(r);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn lg_wmab_get(dev: *mut device, method: u32, result: *mut u32) -> c_int {
    static int lg_wmab_get(struct device *dev, u32 method, u32 *result)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    union acpi_object in[] = {
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = method,
    },
    },
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = WM_GET,
    },
    },
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = 0,
    },
    },
    };
    struct lg_wmab_buffer_result *buffer_result;
    struct acpi_object_list input = {
    .count = ARRAY_SIZE(in),
    .pointer = in,
    };
    acpi_status status;
    status = acpi_evaluate_object(ACPI_HANDLE(dev), "WMAB", &input, &buffer);
    if (ACPI_FAILURE(status))
    return -EIO;
    union acpi_object *obj __free(kfree) = buffer.pointer;
    if (!obj)
    return -ENODATA;
    switch (obj.type) {
    case ACPI_TYPE_INTEGER:
// result = obj->integer.value;
    return 0;
    case ACPI_TYPE_BUFFER:
    if (obj.buffer.length != sizeof(*buffer_result))
    return -EPROTO;
    buffer_result = (struct lg_wmab_buffer_result *)obj.buffer.pointer;
    if (get_unaligned_le32(&buffer_result.status))
    return -EIO;
// result = get_unaligned_le32(&buffer_result->value);
    return 0;
    default:
    return -EPROTO;
    }
    }
#[no_mangle]
unsafe extern "C" fn lg_wmab_set(dev: *mut device, method: u32, arg: u32) -> c_int {
    static int lg_wmab_set(struct device *dev, u32 method, u32 arg)
    {
    union acpi_object in[] = {
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = method,
    },
    },
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = WM_SET,
    },
    },
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = arg,
    },
    },
    };
    struct acpi_object_list input = {
    .count = ARRAY_SIZE(in),
    .pointer = in,
    };
    acpi_status status;
    status = acpi_evaluate_object(ACPI_HANDLE(dev), "WMAB", &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EIO;
    return 0;
    }
    static union acpi_object *lg_wmbb(struct device *dev, u32 method_id, u32 arg1, u32 arg2)
    {
    union acpi_object args[3];
    acpi_status status;
    struct acpi_object_list arg;
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    u8 buf[32];
// (u32 *)buf = method_id;
// (u32 *)(buf + 4) = arg1;
// (u32 *)(buf + 16) = arg2;
    args[0].type = ACPI_TYPE_INTEGER;
    args[0].integer.value = 0; /* ignored */
    args[1].type = ACPI_TYPE_INTEGER;
    args[1].integer.value = 1; /* Must be 1 or 2. Does not matter which */
    args[2].type = ACPI_TYPE_BUFFER;
    args[2].buffer.length = 32;
    args[2].buffer.pointer = buf;
    arg.count = 3;
    arg.pointer = args;
    status = acpi_evaluate_object(ACPI_HANDLE(dev), "WMBB", &arg, &buffer);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "WMBB: call failed.\n");
    return core::ptr::null_mut();
    }
    return (union acpi_object *)buffer.pointer;
    }
    static ssize_t fan_mode_store(struct device *dev,
    struct device_attribute *attr,
    const char *buffer, size_t count)
    {
    unsigned long value;
    int ret;
    ret = kstrtoul(buffer, 10, &value);
    if (ret)
    return ret;
    if (value >= 3)
    return -EINVAL;
    ret = lg_wmab_set(dev, WM_FAN_MODE, FIELD_PREP(FAN_MODE_LOWER, value) |
    FIELD_PREP(FAN_MODE_UPPER, value));
    if (ret < 0)
    return ret;
    return count;
    }
    static ssize_t fan_mode_show(struct device *dev,
    struct device_attribute *attr, char *buffer)
    {
    u32 mode;
    int ret;
    ret = lg_wmab_get(dev, WM_FAN_MODE, &mode);
    if (ret < 0)
    return ret;
    return sysfs_emit(buffer, "%lu\n", FIELD_GET(FAN_MODE_LOWER, mode));
    }
    static ssize_t usb_charge_store(struct device *dev,
    struct device_attribute *attr,
    const char *buffer, size_t count)
    {
    bool value;
    union acpi_object *r;
    int ret;
    ret = kstrtobool(buffer, &value);
    if (ret)
    return ret;
    r = lg_wmbb(dev, WMBB_USB_CHARGE, WM_SET, value);
    if (!r)
    return -EIO;
    kfree(r);
    return count;
    }
    static ssize_t usb_charge_show(struct device *dev,
    struct device_attribute *attr, char *buffer)
    {
    unsigned int status;
    union acpi_object *r;
    r = lg_wmbb(dev, WMBB_USB_CHARGE, WM_GET, 0);
    if (!r)
    return -EIO;
    if (r.type != ACPI_TYPE_BUFFER) {
    kfree(r);
    return -EIO;
    }
    status = !!r.buffer.pointer[0x10];
    kfree(r);
    return sysfs_emit(buffer, "%d\n", status);
    }
    static ssize_t reader_mode_store(struct device *dev,
    struct device_attribute *attr,
    const char *buffer, size_t count)
    {
    bool value;
    int ret;
    ret = kstrtobool(buffer, &value);
    if (ret)
    return ret;
    ret = lg_wmab_set(dev, WM_READER_MODE, value);
    if (ret < 0)
    return ret;
    return count;
    }
    static ssize_t reader_mode_show(struct device *dev,
    struct device_attribute *attr, char *buffer)
    {
    u32 status;
    int ret;
    ret = lg_wmab_get(dev, WM_READER_MODE, &status);
    if (ret < 0)
    return ret;
    return sysfs_emit(buffer, "%d\n", !!status);
    }
    static ssize_t fn_lock_store(struct device *dev,
    struct device_attribute *attr,
    const char *buffer, size_t count)
    {
    bool value;
    int ret;
    ret = kstrtobool(buffer, &value);
    if (ret)
    return ret;
    ret = lg_wmab_set(dev, WM_FN_LOCK, value);
    if (ret < 0)
    return ret;
    return count;
    }
    static ssize_t fn_lock_show(struct device *dev,
    struct device_attribute *attr, char *buffer)
    {
    u32 status;
    int ret;
    ret = lg_wmab_get(dev, WM_FN_LOCK, &status);
    if (ret < 0)
    return ret;
    return sysfs_emit(buffer, "%d\n", !!status);
    }
    static ssize_t charge_control_end_threshold_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    unsigned long value;
    int ret;
    ret = kstrtoul(buf, 10, &value);
    if (ret)
    return ret;
    if (value == 100 || value == 80) {
    union acpi_object *r;
    if (battery_limit_use_wmbb) {
    r = lg_wmbb(&pf_device.dev, WMBB_BATT_LIMIT, WM_SET, value);
    if (!r)
    return -EIO;
    kfree(r);
    } else {
    ret = lg_wmab_set(&pf_device.dev, WM_BATT_LIMIT, value);
    if (ret < 0)
    return ret;
    }
    return count;
    }
    return -EINVAL;
    }
    static ssize_t charge_control_end_threshold_show(struct device *device,
    struct device_attribute *attr,
    char *buf)
    {
    union acpi_object *r;
    u32 status;
    int ret;
    if (battery_limit_use_wmbb) {
    r = lg_wmbb(&pf_device.dev, WMBB_BATT_LIMIT, WM_GET, 0);
    if (!r)
    return -EIO;
    if (r.type != ACPI_TYPE_BUFFER) {
    kfree(r);
    return -EIO;
    }
    status = r.buffer.pointer[0x10];
    kfree(r);
    } else {
    ret = lg_wmab_get(&pf_device.dev, WM_BATT_LIMIT, &status);
    if (ret < 0)
    return ret;
    }
    if (status != 80 && status != 100)
    status = 0;
    return sysfs_emit(buf, "%d\n", status);
    }
    static ssize_t battery_care_limit_show(struct device *dev,
    struct device_attribute *attr,
    char *buffer)
    {
    return charge_control_end_threshold_show(dev, attr, buffer);
    }
    static ssize_t battery_care_limit_store(struct device *dev,
    struct device_attribute *attr,
    const char *buffer, size_t count)
    {
    return charge_control_end_threshold_store(dev, attr, buffer, count);
    }
    static DEVICE_ATTR_RW(fan_mode);
    static DEVICE_ATTR_RW(usb_charge);
    static DEVICE_ATTR_RW(reader_mode);
    static DEVICE_ATTR_RW(fn_lock);
    static DEVICE_ATTR_RW(charge_control_end_threshold);
    static DEVICE_ATTR_RW(battery_care_limit);
#[no_mangle]
unsafe extern "C" fn lg_battery_add(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int lg_battery_add(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
    if (device_create_file(&battery.dev,
    &dev_attr_charge_control_end_threshold))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lg_battery_remove(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int lg_battery_remove(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
    device_remove_file(&battery.dev,
    &dev_attr_charge_control_end_threshold);
    return 0;
    }
    static struct acpi_battery_hook battery_hook = {
    .add_battery = lg_battery_add,
    .remove_battery = lg_battery_remove,
    .name = "LG Battery Extension",
    };
    static struct attribute *dev_attributes[] = {
    &dev_attr_fan_mode.attr,
    &dev_attr_usb_charge.attr,
    &dev_attr_reader_mode.attr,
    &dev_attr_fn_lock.attr,
    &dev_attr_battery_care_limit.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group dev_attribute_group = {
    .attrs = dev_attributes,
    };
    static void tpad_led_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    lg_wmab_set(cdev.dev.parent, WM_TLED, brightness > LED_OFF);
    }
#[no_mangle]
unsafe extern "C" fn tpad_led_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness tpad_led_get(struct led_classdev *cdev)
    {
    return ggov(GOV_TLED) > 0 ? LED_ON : LED_OFF;
    }
    static LED_DEVICE(tpad_led, 1, 0);
    static void kbd_backlight_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
// Must always be written
    let mut value: u32 = KBD_LED_STATUS;
    u32 mode, bright;
    if (brightness <= LED_OFF) {
    mode = KBD_LED_MODE_OFF;
    bright = KBD_LED_BRIGHTNESS_OFF;
    } else {
    mode = KBD_LED_MODE_ON;
    if (brightness >= LED_FULL)
    bright = KBD_LED_BRIGHTNESS_FULL;
    else
    bright = KBD_LED_BRIGHTNESS_HALF;
    }
    value |= FIELD_PREP(KBD_LED_BRIGHTNESS_MASK, bright);
    value |= FIELD_PREP(KBD_LED_MODE_MASK, mode);
    lg_wmab_set(cdev.dev.parent, WM_KEY_LIGHT, value);
    }
#[no_mangle]
unsafe extern "C" fn get_kbd_backlight_level(dev: *mut device) -> enum led_brightness {
    static enum led_brightness get_kbd_backlight_level(struct device *dev)
    {
    u32 value;
    int ret;
    ret = lg_wmab_get(dev, WM_KEY_LIGHT, &value);
    if (ret < 0)
    return LED_OFF;
    if (FIELD_GET(KBD_LED_MAGIC_MASK, value) != KBD_LED_MAGIC)
    return LED_OFF;
    if (FIELD_GET(KBD_LED_MODE_MASK, value) == KBD_LED_MODE_OFF)
    return LED_OFF;
    switch (FIELD_GET(KBD_LED_BRIGHTNESS_MASK, value)) {
    case KBD_LED_BRIGHTNESS_FULL:
    return LED_FULL;
    case KBD_LED_BRIGHTNESS_HALF:
    return LED_HALF;
    default:
    return LED_OFF;
    }
    }
#[no_mangle]
unsafe extern "C" fn kbd_backlight_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness kbd_backlight_get(struct led_classdev *cdev)
    {
    return get_kbd_backlight_level(cdev.dev.parent);
    }
    static LED_DEVICE(kbd_backlight, 255, LED_BRIGHT_HW_CHANGED);
    static struct platform_driver pf_driver = {
    .driver = {
    .name = PLATFORM_NAME,
    }
    };
#[no_mangle]
unsafe extern "C" fn lg_laptop_get_event_data(handle: acpi_handle, value: u32, data: *mut u32) -> c_int {
    static int lg_laptop_get_event_data(acpi_handle handle, u32 value, u32 *data)
    {
    union acpi_object objs[] = {
    {
    .integer = {
    .type = ACPI_TYPE_INTEGER,
    .value = value,
    },
    }
    };
    struct acpi_object_list args = {
    .count = ARRAY_SIZE(objs),
    .pointer = objs,
    };
    unsigned long long result;
    acpi_status status;
    status = acpi_evaluate_integer(handle, "_WED", &args, &result);
    if (ACPI_FAILURE(status))
    return -EIO;
    if (result > U32_MAX)
    return -EPROTO;
// data = result;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lg_laptop_handle_input_event(input_dev: *mut input_dev, value: u32, data: u32) {
    static void lg_laptop_handle_input_event(struct input_dev *input_dev, u32 value, u32 data)
    {
    unsigned int kbd_brightness;
    switch (value) {
    case LG_NOTIFY_HOTKEY:
    sparse_keymap_report_event(input_dev, data, 1, true);
    break;
    case LG_NOTIFY_THERMAL:
// Currently not supported
    break;
    case LG_NOTIFY_MISC:
    switch (data) {
    case 0x10000000:
    if (!kbd_backlight_available)
    break;
    kbd_brightness = get_kbd_backlight_level(kbd_backlight.dev.parent);
    led_classdev_notify_brightness_hw_changed(&kbd_backlight, kbd_brightness);
    break;
    default:
    sparse_keymap_report_event(input_dev, data, 1, true);
    }
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn lg_laptop_notify_handler(handle: acpi_handle, value: u32, context: *mut c_void) {
    static void lg_laptop_notify_handler(acpi_handle handle, u32 value, void *context)
    {
    struct input_dev *input_dev = context;
    u32 data;
    int ret;
    switch (value) {
    case LG_NOTIFY_TABLET_MODE_OFF:
    case LG_NOTIFY_TABLET_MODE_ON:
// Already handled by intel-hid
    return;
    case LG_NOTIFY_HOTKEY:
    case LG_NOTIFY_THERMAL:
    case LG_NOTIFY_MISC:
    ret = lg_laptop_get_event_data(handle, value, &data);
    if (ret < 0) {
    dev_notice(input_dev.dev.parent, "Failed to get event data: %d\n", ret);
    return;
    }
    dev_dbg(input_dev.dev.parent, "Received event %u (%u)\n", value, data);
    lg_laptop_handle_input_event(input_dev, value, data);
    return;
    default:
    dev_notice(input_dev.dev.parent, "Received unknown event %u\n", value);
    }
    };
#[no_mangle]
unsafe extern "C" fn lg_laptop_remove_notify_handler(context: *mut c_void) {
    static void lg_laptop_remove_notify_handler(void *context)
    {
    let mut handle: acpi_handle = context;
    acpi_remove_notify_handler(handle, ACPI_ALL_NOTIFY, lg_laptop_notify_handler);
    }
#[no_mangle]
unsafe extern "C" fn lg_laptop_reenable_wmi_events(context: *mut c_void) {
    static void lg_laptop_reenable_wmi_events(void *context)
    {
    let mut handle: acpi_handle = context;
    unsigned long long dummy;
    lg_laptop_execute_orep(handle, LG_OREP_INTERCEPT_WMI_EVENTS, 0, &dummy);
    }
#[no_mangle]
unsafe extern "C" fn lg_laptop_input_init(dev: *mut device, handle: acpi_handle) -> c_int {
    static int lg_laptop_input_init(struct device *dev, acpi_handle handle)
    {
    struct input_dev *input_dev;
    unsigned long long result;
    acpi_status status;
    int ret;
    if (!acpi_has_method(handle, "_WED"))
    return 0;
    input_dev = devm_input_allocate_device(dev);
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = "LG WMI hotkeys";
    input_dev.phys = "wmi/input0";
    input_dev.id.bustype = BUS_HOST;
    ret = sparse_keymap_setup(input_dev, wmi_keymap, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    ret = input_register_device(input_dev);
    if (ret < 0)
    return ret;
    status = acpi_install_notify_handler(handle, ACPI_ALL_NOTIFY, lg_laptop_notify_handler,
    input_dev);
    if (ACPI_FAILURE(status))
    return -EIO;
    ret = devm_add_action_or_reset(dev, lg_laptop_remove_notify_handler, handle);
    if (ret < 0)
    return ret;
    if (acpi_has_method(handle, "OREP")) {
    ret = lg_laptop_execute_orep(handle, LG_OREP_INTERCEPT_WMI_EVENTS, 1, &result);
    if (ret < 0)
    return ret;
    if (result)
    return -EIO;
    ret = devm_add_action_or_reset(dev, lg_laptop_reenable_wmi_events, handle);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static acpi_status lg_laptop_address_space_write(struct device *dev, acpi_physical_address address,
    size_t size, u64 value)
    {
    u8 byte;
// Ignore any debug messages
    if (address >= LG_ADDRESS_SPACE_DEBUG_MSG_START_ADR &&
    address <= LG_ADDRESS_SPACE_DEBUG_MSG_END_ADR)
    return AE_OK;
    if (size != sizeof(byte))
    return AE_BAD_PARAMETER;
    byte = value & 0xFF;
    switch (address) {
    case LG_ADDRESS_SPACE_HD_AUDIO_POWER_ADDR:
//
// The HD audio power field is not affected by the DTTM flag,
// so we have to manually check fw_debug.
//
    if (fw_debug)
    dev_dbg(dev, "HD audio power %s\n", str_enabled_disabled(byte));
    return AE_OK;
    case LG_ADDRESS_SPACE_FAN_MODE_ADR:
//
// The fan mode field is not affected by the DTTM flag, so we
// have to manually check fw_debug.
//
    if (fw_debug)
    dev_dbg(dev, "Fan mode set to mode %u\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_CPU_TEMP_ADR:
    dev_dbg(dev, "CPU temperature is %u °C\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_CPU_TRIP_LOW_ADR:
    dev_dbg(dev, "CPU lower trip point set to %u °C\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_CPU_TRIP_HIGH_ADR:
    dev_dbg(dev, "CPU higher trip point set to %u °C\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_MB_TEMP_ADR:
    dev_dbg(dev, "Motherboard temperature is %u °C\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_MB_TRIP_LOW_ADR:
    dev_dbg(dev, "Motherboard lower trip point set to %u °C\n", byte);
    return AE_OK;
    case LG_ADDRESS_SPACE_MB_TRIP_HIGH_ADR:
    dev_dbg(dev, "Motherboard higher trip point set to %u °C\n", byte);
    return AE_OK;
    default:
    dev_notice_ratelimited(dev, "Ignoring write to unknown opregion address %llu\n",
    address);
    return AE_OK;
    }
    }
    static acpi_status lg_laptop_address_space_read(struct device *dev, acpi_physical_address address,
    size_t size, u64 *value)
    {
    if (size != 1)
    return AE_BAD_PARAMETER;
    switch (address) {
    case LG_ADDRESS_SPACE_DEBUG_FLAG_ADR:
// Debug messages are already printed using the standard ACPI Debug object
// value = 0x00;
    return AE_OK;
    case LG_ADDRESS_SPACE_DTTM_FLAG_ADR:
// value = fw_debug;
    return AE_OK;
    default:
    dev_notice_ratelimited(dev, "Attempt to read unknown opregion address %llu\n",
    address);
    return AE_BAD_PARAMETER;
    }
    }
    static acpi_status lg_laptop_address_space_handler(u32 function, acpi_physical_address address,
    u32 bits, u64 *value, void *handler_context,
    void *region_context)
    {
    struct device *dev = handler_context;
    size_t size;
    if (bits % BITS_PER_BYTE)
    return AE_BAD_PARAMETER;
    size = bits / BITS_PER_BYTE;
    switch (function) {
    case ACPI_READ:
    return lg_laptop_address_space_read(dev, address, size, value);
    case ACPI_WRITE:
    return lg_laptop_address_space_write(dev, address, size, *value);
    default:
    return AE_BAD_PARAMETER;
    }
    }
#[no_mangle]
unsafe extern "C" fn lg_laptop_remove_address_space_handler(data: *mut c_void) {
    static void lg_laptop_remove_address_space_handler(void *data)
    {
    struct acpi_device *device = data;
    acpi_remove_address_space_handler(device.handle, LG_ADDRESS_SPACE_ID,
    &lg_laptop_address_space_handler);
    }
#[no_mangle]
unsafe extern "C" fn acpi_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_probe(struct platform_device *pdev)
    {
    struct platform_device_info pdev_info = {
    .name = PLATFORM_NAME,
    .id = PLATFORM_DEVID_NONE,
    };
    struct acpi_device *device;
    acpi_status status;
    int ret;
    const char *product;
    let mut year: c_int = 2017;
    if (pf_device)
    return 0;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    pdev_info.fwnode = acpi_fwnode_handle(device),
    status = acpi_install_address_space_handler(device.handle, LG_ADDRESS_SPACE_ID,
    &lg_laptop_address_space_handler,
    core::ptr::null_mut(), &pdev.dev);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    ret = devm_add_action_or_reset(&pdev.dev, lg_laptop_remove_address_space_handler,
    device);
    if (ret < 0)
    return ret;
    ret = platform_driver_register(&pf_driver);
    if (ret)
    return ret;
    pf_device = platform_device_register_full(&pdev_info);
    if (IS_ERR(pf_device)) {
    ret = PTR_ERR(pf_device);
    pf_device = core::ptr::null_mut();
    pr_err("unable to register platform device\n");
    goto out_platform_registered;
    }
    product = dmi_get_system_info(DMI_PRODUCT_NAME);
    if (product && strlen(product) > 4)
    switch (product[4]) {
    case '5':
    if (strlen(product) > 5)
    switch (product[5]) {
    case 'N':
    year = 2021;
    break;
    case '0':
    year = 2016;
    break;
    default:
    year = 2022;
    }
    break;
    case '6':
    year = 2016;
    break;
    case '7':
    year = 2017;
    break;
    case '8':
    year = 2018;
    break;
    case '9':
    year = 2019;
    break;
    case '0':
    if (strlen(product) > 5)
    switch (product[5]) {
    case 'N':
    year = 2020;
    break;
    case 'P':
    year = 2021;
    break;
    case 'Q':
    year = 2022;
    break;
    case 'R':
    year = 2023;
    break;
    case 'S':
    year = 2024;
    break;
    default:
    year = 2025;
    }
    break;
    default:
    year = 2019;
    }
    pr_info("product: %s  year: %d\n", product ?: "unknown", year);
    if (year >= 2019)
    battery_limit_use_wmbb = 1;
// LEDs are optional
    ret = devm_led_classdev_register(&pdev.dev, &kbd_backlight);
    if (ret < 0)
    kbd_backlight_available = false;
    else
    kbd_backlight_available = true;
    devm_led_classdev_register(&pdev.dev, &tpad_led);
    ret = lg_laptop_input_init(&pdev.dev, device.handle);
    if (ret < 0)
    goto out_platform_device;
    ret = sysfs_create_group(&pf_device.dev.kobj, &dev_attribute_group);
    if (ret)
    goto out_platform_device;
    battery_hook_register(&battery_hook);
    return 0;
    out_platform_device:
    platform_device_unregister(pf_device);
    out_platform_registered:
    platform_driver_unregister(&pf_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_remove(pdev: *mut platform_device) {
    static void acpi_remove(struct platform_device *pdev)
    {
    sysfs_remove_group(&pf_device.dev.kobj, &dev_attribute_group);
    battery_hook_unregister(&battery_hook);
    platform_device_unregister(pf_device);
    pf_device = core::ptr::null_mut();
    platform_driver_unregister(&pf_driver);
    }
    static const struct acpi_device_id device_ids[] = {
    {"LGEX0820", 0},
    {"", 0}
    };
    MODULE_DEVICE_TABLE(acpi, device_ids);
    static struct platform_driver acpi_driver = {
    .probe = acpi_probe,
    .remove = acpi_remove,
    .driver = {
    .name = "LG Gram Laptop Support",
    .acpi_match_table = device_ids,
    },
    };
    module_platform_driver(acpi_driver);
