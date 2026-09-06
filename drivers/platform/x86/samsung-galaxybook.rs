//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/samsung-galaxybook.c
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
// Samsung Galaxy Book driver
//
// Copyright (c) 2025 Joshua Grisham <josh@joshuagrisham.com>
//
// With contributions to the SCAI ACPI device interface:
// Copyright (c) 2024 Giulio Girardi <giulio.girardi@protechgroup.it>
//
// Implementation inspired by existing x86 platform drivers.
// Thank you to the authors!
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_galaxybook {
    pub platform: *mut platform_device,
    pub acpi: *mut acpi_device,
    pub fw_attrs_dev: *mut device,
    pub fw_attrs_kset: *mut kset,
// block in case firmware attributes are updated in multiple threads
    pub fw_attr_lock: mutex,
    pub has_kbd_backlight: bool,
    pub has_block_recording: bool,
    pub has_performance_mode: bool,
    pub kbd_backlight: led_classdev,
    pub kbd_backlight_hotkey_work: work_struct,
// block in case brightness updated using hotkey and another thread
    pub kbd_backlight_lock: mutex,
    pub i8042_filter_ptr: *mut c_void,
    pub block_recording_hotkey_work: work_struct,
    pub input: *mut input_dev,
    pub battery_hook: acpi_battery_hook,
    pub profile_performance_modes: [u8; PLATFORM_PROFILE_LAST],
}

    enum galaxybook_fw_attr_id {
    GB_ATTR_POWER_ON_LID_OPEN,
    GB_ATTR_USB_CHARGING,
    GB_ATTR_BLOCK_RECORDING,
    };
    static const char * const galaxybook_fw_attr_name[] = {
    [GB_ATTR_POWER_ON_LID_OPEN] = "power_on_lid_open",
    [GB_ATTR_USB_CHARGING]      = "usb_charging",
    [GB_ATTR_BLOCK_RECORDING]   = "block_recording",
    };
    static const char * const galaxybook_fw_attr_desc[] = {
    [GB_ATTR_POWER_ON_LID_OPEN] = "Power On Lid Open",
    [GB_ATTR_USB_CHARGING]      = "USB Charging",
    [GB_ATTR_BLOCK_RECORDING]   = "Block Recording",
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct galaxybook_fw_attr {
    pub galaxybook: *mut samsung_galaxybook,
    pub fw_attr_id: enum galaxybook_fw_attr_id,
    pub attr_group: attribute_group,
    pub display_name: kobj_attribute,
    pub current_value: kobj_attribute,
    pub value): *mut *mut *mut int (get_value)(struct samsung_galaxybook galaxybook, bool,
    pub value): *const *const *const int (set_value)(struct samsung_galaxybook galaxybook, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sawb {
    pub safn: u16,
    pub sasb: u16,
    pub rflg: u8,
    union {
    struct {
    pub gunm: u8,
    pub guds: [u8; 250],
    pub __packed: },
    struct {
    pub caid: [u8; 16],
    pub fncn: u8,
    pub subn: u8,
    pub iob0: u8,
    pub iob1: u8,
    pub iob2: u8,
    pub iob3: u8,
    pub iob4: u8,
    pub iob5: u8,
    pub iob6: u8,
    pub iob7: u8,
    pub iob8: u8,
    pub iob9: u8,
    pub __packed: },
    struct {
    pub iob_prefix: [u8; 18],
    pub iobs: [u8; 10],
    pub __packed: },
    pub __packed: },
    pub __packed: },
pub const GB_SAWB_LEN_SETTINGS: c_uint = 0x15;
pub const GB_SAWB_LEN_PERFORMANCE_MODE: c_uint = 0x100;
pub const GB_SAFN: c_uint = 0x5843;
pub const GB_SASB_KBD_BACKLIGHT: c_uint = 0x78;
pub const GB_SASB_POWER_MANAGEMENT: c_uint = 0x7a;
pub const GB_SASB_USB_CHARGING_GET: c_uint = 0x67;
pub const GB_SASB_USB_CHARGING_SET: c_uint = 0x68;
pub const GB_SASB_NOTIFICATIONS: c_uint = 0x86;
pub const GB_SASB_BLOCK_RECORDING: c_uint = 0x8a;
pub const GB_SASB_PERFORMANCE_MODE: c_uint = 0x91;
pub const GB_SAWB_RFLG_POS: c_int = 4;
pub const GB_SAWB_GB_GUNM_POS: c_int = 5;
pub const GB_RFLG_SUCCESS: c_uint = 0xaa;
pub const GB_GUNM_FAIL: c_uint = 0xff;
pub const GB_GUNM_FEATURE_ENABLE: c_uint = 0xbb;
pub const GB_GUNM_FEATURE_ENABLE_SUCCESS: c_uint = 0xdd;
pub const GB_GUDS_FEATURE_ENABLE: c_uint = 0xaa;
pub const GB_GUDS_FEATURE_ENABLE_SUCCESS: c_uint = 0xcc;
pub const GB_GUNM_GET: c_uint = 0x81;
pub const GB_GUNM_SET: c_uint = 0x82;
pub const GB_GUNM_POWER_MANAGEMENT: c_uint = 0x82;
pub const GB_GUNM_USB_CHARGING_GET: c_uint = 0x80;
pub const GB_GUNM_USB_CHARGING_ON: c_uint = 0x81;
pub const GB_GUNM_USB_CHARGING_OFF: c_uint = 0x80;
pub const GB_GUDS_POWER_ON_LID_OPEN: c_uint = 0xa3;
pub const GB_GUDS_POWER_ON_LID_OPEN_GET: c_uint = 0x81;
pub const GB_GUDS_POWER_ON_LID_OPEN_SET: c_uint = 0x80;
pub const GB_GUDS_BATTERY_CHARGE_CONTROL: c_uint = 0xe9;
pub const GB_GUDS_BATTERY_CHARGE_CONTROL_GET: c_uint = 0x91;
pub const GB_GUDS_BATTERY_CHARGE_CONTROL_SET: c_uint = 0x90;
pub const GB_GUNM_ACPI_NOTIFY_ENABLE: c_uint = 0x80;
pub const GB_GUDS_ACPI_NOTIFY_ENABLE: c_uint = 0x02;
pub const GB_BLOCK_RECORDING_ON: c_uint = 0x0;
pub const GB_BLOCK_RECORDING_OFF: c_uint = 0x1;
pub const GB_FNCN_PERFORMANCE_MODE: c_uint = 0x51;
pub const GB_SUBN_PERFORMANCE_MODE_LIST: c_uint = 0x01;
pub const GB_SUBN_PERFORMANCE_MODE_GET: c_uint = 0x02;
pub const GB_SUBN_PERFORMANCE_MODE_SET: c_uint = 0x03;
// guid 8246028d-8bca-4a55-ba0f-6f1e6b921b8f
    static const guid_t performance_mode_guid =
    pub 0x8f): GUID_INIT(0x8246028d, 0x8bca, 0x4a55, 0xba, 0x0f, 0x6f, 0x1e, 0x6b, 0x92, 0x1b,,

pub const GB_PERFORMANCE_MODE_FANOFF: c_uint = 0xb;
pub const GB_PERFORMANCE_MODE_LOWNOISE: c_uint = 0xa;
pub const GB_PERFORMANCE_MODE_OPTIMIZED: c_uint = 0x0;
pub const GB_PERFORMANCE_MODE_OPTIMIZED_V2: c_uint = 0x2;
pub const GB_PERFORMANCE_MODE_PERFORMANCE: c_uint = 0x1;
pub const GB_PERFORMANCE_MODE_PERFORMANCE_V2: c_uint = 0x15;
pub const GB_PERFORMANCE_MODE_ULTRA: c_uint = 0x16;
pub const GB_PERFORMANCE_MODE_IGNORE1: c_uint = 0x14;
pub const GB_PERFORMANCE_MODE_IGNORE2: c_uint = 0xc;

pub const GB_ACPI_METHOD_ENABLE_ON: c_int = 1;
pub const GB_ACPI_METHOD_ENABLE_OFF: c_int = 0;

pub const GB_KBD_BACKLIGHT_MAX_BRIGHTNESS: c_int = 3;
pub const GB_ACPI_NOTIFY_BATTERY_STATE_CHANGED: c_uint = 0x61;
pub const GB_ACPI_NOTIFY_DEVICE_ON_TABLE: c_uint = 0x6c;
pub const GB_ACPI_NOTIFY_DEVICE_OFF_TABLE: c_uint = 0x6d;
pub const GB_ACPI_NOTIFY_HOTKEY_PERFORMANCE_MODE: c_uint = 0x70;
pub const GB_ACPI_NOTIFY_HOTKEY_KBD_BACKLIGHT: c_uint = 0x7d;
pub const GB_ACPI_NOTIFY_HOTKEY_MICMUTE: c_uint = 0x6e;
pub const GB_ACPI_NOTIFY_HOTKEY_CAMERA: c_uint = 0x6f;
pub const GB_KEY_KBD_BACKLIGHT_KEYDOWN: c_uint = 0x2c;
pub const GB_KEY_KBD_BACKLIGHT_KEYUP: c_uint = 0xac;
pub const GB_KEY_BLOCK_RECORDING_KEYDOWN: c_uint = 0x1f;
pub const GB_KEY_BLOCK_RECORDING_KEYUP: c_uint = 0x9f;
pub const GB_KEY_BATTERY_NOTIFY_KEYUP: c_uint = 0xf;
pub const GB_KEY_BATTERY_NOTIFY_KEYDOWN: c_uint = 0x8f;
//
// Optional features which have been determined as not supported on a particular
// device will return GB_NOT_SUPPORTED from their init function. Positive
// EOPNOTSUPP is used as the underlying value instead of negative to
// differentiate this return code from valid upstream failures.
//

//
// ACPI method handling
//
    static int galaxybook_acpi_method(struct samsung_galaxybook *galaxybook, acpi_string method,
    struct sawb *buf, size_t len)
    {
    pub NULL}: acpi_buffer output = {ACPI_ALLOCATE_BUFFER,,
    pub out_obj: *mut union acpi_object in_obj,,
    pub input: acpi_object_list,
    pub status: acpi_status,
    pub err: c_int,
    pub ACPI_TYPE_BUFFER: in_obj.type =,
    pub len: in_obj.buffer.length =,
    pub )buf: *mut in_obj.buffer.pointer = (u8,
    pub 1: input.count =,
    pub &in_obj: input.pointer =,
    status = acpi_evaluate_object_typed(galaxybook.acpi.handle, method, &input, &output,
    if (ACPI_FAILURE(status)) {
    pub %s\n",: dev_err(&galaxybook->acpi->dev, "failed to execute method %s; got,
    pub acpi_format_exception(status)): method,,
    pub -EIO: return,
    }
    pub output.pointer: out_obj =,
    if (out_obj.buffer.length != len || out_obj.buffer.length < GB_SAWB_GB_GUNM_POS + 1) {
    dev_err(&galaxybook.acpi.dev,
    pub mismatch\n",: "failed to execute %s; response length,
    pub -EPROTO: err =,
    pub out_free: goto,
    }
    if (out_obj.buffer.pointer[GB_SAWB_RFLG_POS] != GB_RFLG_SUCCESS) {
    dev_err(&galaxybook.acpi.dev,
    pub 0x%x\n",: "failed to execute %s; device did not respond with success code,
    pub GB_RFLG_SUCCESS): method,,
    pub -ENXIO: err =,
    pub out_free: goto,
    }
    if (out_obj.buffer.pointer[GB_SAWB_GB_GUNM_POS] == GB_GUNM_FAIL) {
    dev_err(&galaxybook.acpi.dev,
    pub 0x%x\n",: "failed to execute %s; device responded with failure code,
    pub GB_GUNM_FAIL): method,,
    pub -ENXIO: err =,
    pub out_free: goto,
    }
    pub len): memcpy(buf, out_obj->buffer.pointer,,
    pub 0: err =,
    out_free:
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_enable_acpi_feature(galaxybook: *mut samsung_galaxybook, sasb: u16) -> c_int {
    static int galaxybook_enable_acpi_feature(struct samsung_galaxybook *galaxybook, const u16 sasb)
    {
    pub {}: sawb buf =,
    pub err: c_int,
    pub GB_SAFN: buf.safn =,
    pub sasb: buf.sasb =,
    pub GB_GUNM_FEATURE_ENABLE: buf.gunm =,
    pub GB_GUDS_FEATURE_ENABLE: buf.guds[0] =,
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    pub GB_SAWB_LEN_SETTINGS): &buf,,
    if (err)
    pub err: return,
    if (buf.gunm != GB_GUNM_FEATURE_ENABLE_SUCCESS &&
    buf.guds[0] != GB_GUDS_FEATURE_ENABLE_SUCCESS)
    pub -ENODEV: return,
    pub 0: return,
    }
//
// Keyboard Backlight
//
    static int kbd_backlight_acpi_get(struct samsung_galaxybook *galaxybook,
    enum led_brightness *brightness)
    {
    pub {}: sawb buf =,
    pub err: c_int,
    pub GB_SAFN: buf.safn =,
    pub GB_SASB_KBD_BACKLIGHT: buf.sasb =,
    pub GB_GUNM_GET: buf.gunm =,
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    pub GB_SAWB_LEN_SETTINGS): &buf,,
    if (err)
    pub err: return,
// brightness = buf.gunm;
    pub 0: return,
    }
    static int kbd_backlight_acpi_set(struct samsung_galaxybook *galaxybook,
    const enum led_brightness brightness)
    {
    pub {}: sawb buf =,
    pub GB_SAFN: buf.safn =,
    pub GB_SASB_KBD_BACKLIGHT: buf.sasb =,
    pub GB_GUNM_SET: buf.gunm =,
    pub brightness: buf.guds[0] =,
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    pub GB_SAWB_LEN_SETTINGS): &buf,,
    }
#[no_mangle]
unsafe extern "C" fn kbd_backlight_show(led: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness kbd_backlight_show(struct led_classdev *led)
    {
    struct samsung_galaxybook *galaxybook =
    pub kbd_backlight): container_of(led, struct samsung_galaxybook,,
    pub brightness: enum led_brightness,
    pub err: c_int,
    pub &brightness): err = kbd_backlight_acpi_get(galaxybook,,
    if (err)
    pub err: return,
    pub brightness: return,
    }
    static int kbd_backlight_store(struct led_classdev *led,
    const enum led_brightness brightness)
    {
    struct samsung_galaxybook *galaxybook =
    pub kbd_backlight): container_of_const(led, struct samsung_galaxybook,,
    pub brightness): return kbd_backlight_acpi_set(galaxybook,,
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_kbd_backlight_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_kbd_backlight_init(struct samsung_galaxybook *galaxybook)
    {
    pub {}: led_init_data init_data =,
    pub brightness: enum led_brightness,
    pub err: c_int,
    pub &galaxybook->kbd_backlight_lock): err = devm_mutex_init(&galaxybook->platform->dev,,
    if (err)
    pub err: return,
    pub GB_SASB_KBD_BACKLIGHT): err = galaxybook_enable_acpi_feature(galaxybook,,
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    pub err): "failed to enable kbd_backlight feature, error %d\n",,
    pub GB_NOT_SUPPORTED: return,
    }
    pub &brightness): err = kbd_backlight_acpi_get(galaxybook,,
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    pub err): "failed to get initial kbd_backlight brightness, error %d\n",,
    pub GB_NOT_SUPPORTED: return,
    }
    pub DRIVER_NAME: init_data.devicename =,
    pub LED_FUNCTION_KBD_BACKLIGHT: init_data.default_label = ":",
    pub true: init_data.devname_mandatory =,
    pub kbd_backlight_show: galaxybook->kbd_backlight.brightness_get =,
    pub kbd_backlight_store: galaxybook->kbd_backlight.brightness_set_blocking =,
    pub LED_BRIGHT_HW_CHANGED: galaxybook->kbd_backlight.flags =,
    pub GB_KBD_BACKLIGHT_MAX_BRIGHTNESS: galaxybook->kbd_backlight.max_brightness =,
    return devm_led_classdev_register_ext(&galaxybook.platform.dev,
    pub &init_data): &galaxybook->kbd_backlight,,
    }
//
// Battery Extension (adds charge_control_end_threshold to the battery device)
//
#[no_mangle]
unsafe extern "C" fn charge_control_end_threshold_acpi_get(galaxybook: *mut samsung_galaxybook, value: *mut u8) -> c_int {
    static int charge_control_end_threshold_acpi_get(struct samsung_galaxybook *galaxybook, u8 *value)
    {
    pub {}: sawb buf =,
    pub err: c_int,
    pub GB_SAFN: buf.safn =,
    pub GB_SASB_POWER_MANAGEMENT: buf.sasb =,
    pub GB_GUNM_POWER_MANAGEMENT: buf.gunm =,
    pub GB_GUDS_BATTERY_CHARGE_CONTROL: buf.guds[0] =,
    pub GB_GUDS_BATTERY_CHARGE_CONTROL_GET: buf.guds[1] =,
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    pub GB_SAWB_LEN_SETTINGS): &buf,,
    if (err)
    pub err: return,
// value = buf.guds[1];
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn charge_control_end_threshold_acpi_set(galaxybook: *mut samsung_galaxybook, value: u8) -> c_int {
    static int charge_control_end_threshold_acpi_set(struct samsung_galaxybook *galaxybook, u8 value)
    {
    pub {}: sawb buf =,
    pub GB_SAFN: buf.safn =,
    pub GB_SASB_POWER_MANAGEMENT: buf.sasb =,
    pub GB_GUNM_POWER_MANAGEMENT: buf.gunm =,
    pub GB_GUDS_BATTERY_CHARGE_CONTROL: buf.guds[0] =,
    pub GB_GUDS_BATTERY_CHARGE_CONTROL_SET: buf.guds[1] =,
    pub value: buf.guds[2] =,
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    pub GB_SAWB_LEN_SETTINGS): &buf,,
    }
    static int galaxybook_battery_ext_property_get(struct power_supply *psy,
    const struct power_supply_ext *ext,
    void *ext_data,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    pub ext_data: *mut *mut samsung_galaxybook galaxybook =,
    pub value: u8,
    pub err: c_int,
    if (psp != POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD)
    pub -EINVAL: return,
    pub &value): err = charge_control_end_threshold_acpi_get(galaxybook,,
    if (err)
    pub err: return,
//
// device stores "no end threshold" as 0 instead of 100;
// if device has 0, report 100
//
    if (value == 0)
    pub 100: value =,
    pub value: val->intval =,
    pub 0: return,
    }
    static int galaxybook_battery_ext_property_set(struct power_supply *psy,
    const struct power_supply_ext *ext,
    void *ext_data,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    pub ext_data: *mut *mut samsung_galaxybook galaxybook =,
    pub value: u8,
    if (psp != POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD)
    pub -EINVAL: return,
    pub val->intval: value =,
    if (value < 1 || value > 100)
    pub -EINVAL: return,
//
// device stores "no end threshold" as 0 instead of 100;
// if setting to 100, send 0
//
    if (value == 100)
    pub 0: value =,
    pub value): return charge_control_end_threshold_acpi_set(galaxybook,,
    }
    static int galaxybook_battery_ext_property_is_writeable(struct power_supply *psy,
    const struct power_supply_ext *ext,
    void *ext_data,
    enum power_supply_property psp)
    {
    if (psp == POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD)
    pub true: return,
    pub false: return,
    }
    static const enum power_supply_property galaxybook_battery_properties[] = {
    POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD,
}

    static const struct power_supply_ext galaxybook_battery_ext = {
    .name			= DRIVER_NAME,
    .properties		= galaxybook_battery_properties,
    .num_properties		= ARRAY_SIZE(galaxybook_battery_properties),
    .get_property		= galaxybook_battery_ext_property_get,
    .set_property		= galaxybook_battery_ext_property_set,
    .property_is_writeable	= galaxybook_battery_ext_property_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn galaxybook_battery_add(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int galaxybook_battery_add(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
    struct samsung_galaxybook *galaxybook =
    container_of(hook, struct samsung_galaxybook, battery_hook);
    return power_supply_register_extension(battery, &galaxybook_battery_ext,
    &battery.dev, galaxybook);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_battery_remove(battery: *mut power_supply, hook: *mut acpi_battery_hook) -> c_int {
    static int galaxybook_battery_remove(struct power_supply *battery, struct acpi_battery_hook *hook)
    {
    power_supply_unregister_extension(battery, &galaxybook_battery_ext);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_battery_threshold_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_battery_threshold_init(struct samsung_galaxybook *galaxybook)
    {
    u8 value;
    int err;
    err = charge_control_end_threshold_acpi_get(galaxybook, &value);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "failed to get initial battery charge end threshold, error %d\n", err);
    return 0;
    }
    galaxybook.battery_hook.add_battery = galaxybook_battery_add;
    galaxybook.battery_hook.remove_battery = galaxybook_battery_remove;
    galaxybook.battery_hook.name = "Samsung Galaxy Book Battery Extension";
    return devm_battery_hook_register(&galaxybook.platform.dev, &galaxybook.battery_hook);
    }
//
// Platform Profile / Performance mode
//
#[no_mangle]
unsafe extern "C" fn performance_mode_acpi_get(galaxybook: *mut samsung_galaxybook, performance_mode: *mut u8) -> c_int {
    static int performance_mode_acpi_get(struct samsung_galaxybook *galaxybook, u8 *performance_mode)
    {
    let mut buf: sawb = {};
    int err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_PERFORMANCE_MODE;
    export_guid(buf.caid, &GB_PERFORMANCE_MODE_GUID);
    buf.fncn = GB_FNCN_PERFORMANCE_MODE;
    buf.subn = GB_SUBN_PERFORMANCE_MODE_GET;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_PERFORMANCE_MODE,
    &buf, GB_SAWB_LEN_PERFORMANCE_MODE);
    if (err)
    return err;
// performance_mode = buf.iob0;
    return 0;
    }
    static int performance_mode_acpi_set(struct samsung_galaxybook *galaxybook,
    const u8 performance_mode)
    {
    let mut buf: sawb = {};
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_PERFORMANCE_MODE;
    export_guid(buf.caid, &GB_PERFORMANCE_MODE_GUID);
    buf.fncn = GB_FNCN_PERFORMANCE_MODE;
    buf.subn = GB_SUBN_PERFORMANCE_MODE_SET;
    buf.iob0 = performance_mode;
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_PERFORMANCE_MODE,
    &buf, GB_SAWB_LEN_PERFORMANCE_MODE);
    }
    static int get_performance_mode_profile(struct samsung_galaxybook *galaxybook,
    const u8 performance_mode,
    enum platform_profile_option *profile)
    {
    switch (performance_mode) {
    case GB_PERFORMANCE_MODE_FANOFF:
// profile = PLATFORM_PROFILE_LOW_POWER;
    break;
    case GB_PERFORMANCE_MODE_LOWNOISE:
// profile = PLATFORM_PROFILE_QUIET;
    break;
    case GB_PERFORMANCE_MODE_OPTIMIZED:
    case GB_PERFORMANCE_MODE_OPTIMIZED_V2:
// profile = PLATFORM_PROFILE_BALANCED;
    break;
    case GB_PERFORMANCE_MODE_PERFORMANCE:
    case GB_PERFORMANCE_MODE_PERFORMANCE_V2:
    case GB_PERFORMANCE_MODE_ULTRA:
// profile = PLATFORM_PROFILE_PERFORMANCE;
    break;
    case GB_PERFORMANCE_MODE_IGNORE1:
    case GB_PERFORMANCE_MODE_IGNORE2:
    return -EOPNOTSUPP;
    default:
    dev_warn(&galaxybook.platform.dev,
    "unrecognized performance mode 0x%x\n", performance_mode);
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int galaxybook_platform_profile_get(struct device *dev,
    enum platform_profile_option *profile)
    {
    struct samsung_galaxybook *galaxybook = dev_get_drvdata(dev);
    u8 performance_mode;
    int err;
    err = performance_mode_acpi_get(galaxybook, &performance_mode);
    if (err)
    return err;
    return get_performance_mode_profile(galaxybook, performance_mode, profile);
    }
    static int galaxybook_platform_profile_set(struct device *dev,
    enum platform_profile_option profile)
    {
    struct samsung_galaxybook *galaxybook = dev_get_drvdata(dev);
    return performance_mode_acpi_set(galaxybook,
    galaxybook.profile_performance_modes[profile]);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_platform_profile_probe(drvdata: *mut c_void, choices: *mut c_ulong) -> c_int {
    static int galaxybook_platform_profile_probe(void *drvdata, unsigned long *choices)
    {
    struct samsung_galaxybook *galaxybook = drvdata;
    u8 *perfmodes = galaxybook.profile_performance_modes;
    enum platform_profile_option profile;
    let mut buf: sawb = {};
    unsigned int i;
    int err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_PERFORMANCE_MODE;
    export_guid(buf.caid, &GB_PERFORMANCE_MODE_GUID);
    buf.fncn = GB_FNCN_PERFORMANCE_MODE;
    buf.subn = GB_SUBN_PERFORMANCE_MODE_LIST;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_PERFORMANCE_MODE,
    &buf, GB_SAWB_LEN_PERFORMANCE_MODE);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "failed to get supported performance modes, error %d\n", err);
    return err;
    }
// set initial default profile performance mode values
    perfmodes[PLATFORM_PROFILE_LOW_POWER] = GB_PERFORMANCE_MODE_FANOFF;
    perfmodes[PLATFORM_PROFILE_QUIET] = GB_PERFORMANCE_MODE_LOWNOISE;
    perfmodes[PLATFORM_PROFILE_BALANCED] = GB_PERFORMANCE_MODE_OPTIMIZED;
    perfmodes[PLATFORM_PROFILE_PERFORMANCE] = GB_PERFORMANCE_MODE_PERFORMANCE;
//
// Value returned in iob0 will have the number of supported performance
// modes per device. The performance mode values will then be given as a
// list after this (iob1-iobX). Loop through the supported values and
// enable their mapped platform_profile choice, overriding "legacy"
// values along the way if a non-legacy value exists.
//
    for (i = 1; i <= buf.iob0; i++) {
    err = get_performance_mode_profile(galaxybook, buf.iobs[i], &profile);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "ignoring unmapped performance mode 0x%x\n", buf.iobs[i]);
    continue;
    }
    switch (buf.iobs[i]) {
    case GB_PERFORMANCE_MODE_OPTIMIZED_V2:
    perfmodes[profile] = GB_PERFORMANCE_MODE_OPTIMIZED_V2;
    break;
    case GB_PERFORMANCE_MODE_PERFORMANCE_V2:
// only update if not already overwritten by Ultra
    if (perfmodes[profile] != GB_PERFORMANCE_MODE_ULTRA)
    perfmodes[profile] = GB_PERFORMANCE_MODE_PERFORMANCE_V2;
    break;
    case GB_PERFORMANCE_MODE_ULTRA:
    perfmodes[profile] = GB_PERFORMANCE_MODE_ULTRA;
    break;
    default:
    break;
    }
    set_bit(profile, choices);
    dev_dbg(&galaxybook.platform.dev,
    "setting platform profile %d to use performance mode 0x%x\n",
    profile, perfmodes[profile]);
    }
// initialize performance_mode using balanced's mapped value
    if (test_bit(PLATFORM_PROFILE_BALANCED, choices))
    return performance_mode_acpi_set(galaxybook, perfmodes[PLATFORM_PROFILE_BALANCED]);
    return 0;
    }
    static const struct platform_profile_ops galaxybook_platform_profile_ops = {
    .probe = galaxybook_platform_profile_probe,
    .profile_get = galaxybook_platform_profile_get,
    .profile_set = galaxybook_platform_profile_set,
    };
#[no_mangle]
unsafe extern "C" fn galaxybook_platform_profile_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_platform_profile_init(struct samsung_galaxybook *galaxybook)
    {
    struct device *platform_profile_dev;
    u8 performance_mode;
    int err;
    err = performance_mode_acpi_get(galaxybook, &performance_mode);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "failed to get initial performance mode, error %d\n", err);
    return GB_NOT_SUPPORTED;
    }
    platform_profile_dev = devm_platform_profile_register(&galaxybook.platform.dev,
    DRIVER_NAME, galaxybook,
    &galaxybook_platform_profile_ops);
    return PTR_ERR_OR_ZERO(platform_profile_dev);
    }
//
// Firmware Attributes
//
// Power on lid open (device should power on when lid is opened)
#[no_mangle]
unsafe extern "C" fn power_on_lid_open_acpi_get(galaxybook: *mut samsung_galaxybook, value: *mut bool) -> c_int {
    static int power_on_lid_open_acpi_get(struct samsung_galaxybook *galaxybook, bool *value)
    {
    let mut buf: sawb = {};
    int err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_POWER_MANAGEMENT;
    buf.gunm = GB_GUNM_POWER_MANAGEMENT;
    buf.guds[0] = GB_GUDS_POWER_ON_LID_OPEN;
    buf.guds[1] = GB_GUDS_POWER_ON_LID_OPEN_GET;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    if (err)
    return err;
// value = buf.guds[1];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn power_on_lid_open_acpi_set(galaxybook: *mut samsung_galaxybook, value: bool) -> c_int {
    static int power_on_lid_open_acpi_set(struct samsung_galaxybook *galaxybook, const bool value)
    {
    let mut buf: sawb = {};
    lockdep_assert_held(&galaxybook.fw_attr_lock);
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_POWER_MANAGEMENT;
    buf.gunm = GB_GUNM_POWER_MANAGEMENT;
    buf.guds[0] = GB_GUDS_POWER_ON_LID_OPEN;
    buf.guds[1] = GB_GUDS_POWER_ON_LID_OPEN_SET;
    buf.guds[2] = value ? 1 : 0;
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    }
// USB Charging (USB ports can provide power when device is powered off)
#[no_mangle]
unsafe extern "C" fn usb_charging_acpi_get(galaxybook: *mut samsung_galaxybook, value: *mut bool) -> c_int {
    static int usb_charging_acpi_get(struct samsung_galaxybook *galaxybook, bool *value)
    {
    let mut buf: sawb = {};
    int err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_USB_CHARGING_GET;
    buf.gunm = GB_GUNM_USB_CHARGING_GET;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    if (err)
    return err;
// value = buf.gunm == 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_charging_acpi_set(galaxybook: *mut samsung_galaxybook, value: bool) -> c_int {
    static int usb_charging_acpi_set(struct samsung_galaxybook *galaxybook, const bool value)
    {
    let mut buf: sawb = {};
    lockdep_assert_held(&galaxybook.fw_attr_lock);
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_USB_CHARGING_SET;
    buf.gunm = value ? GB_GUNM_USB_CHARGING_ON : GB_GUNM_USB_CHARGING_OFF;
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    }
// Block recording (blocks access to camera and microphone)
#[no_mangle]
unsafe extern "C" fn block_recording_acpi_get(galaxybook: *mut samsung_galaxybook, value: *mut bool) -> c_int {
    static int block_recording_acpi_get(struct samsung_galaxybook *galaxybook, bool *value)
    {
    let mut buf: sawb = {};
    int err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_BLOCK_RECORDING;
    buf.gunm = GB_GUNM_GET;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    if (err)
    return err;
// value = buf.gunm == GB_BLOCK_RECORDING_ON;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn block_recording_acpi_set(galaxybook: *mut samsung_galaxybook, value: bool) -> c_int {
    static int block_recording_acpi_set(struct samsung_galaxybook *galaxybook, const bool value)
    {
    let mut buf: sawb = {};
    int err;
    lockdep_assert_held(&galaxybook.fw_attr_lock);
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_BLOCK_RECORDING;
    buf.gunm = GB_GUNM_SET;
    buf.guds[0] = value ? GB_BLOCK_RECORDING_ON : GB_BLOCK_RECORDING_OFF;
    err = galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    if (err)
    return err;
    input_report_switch(galaxybook.input,
    SW_CAMERA_LENS_COVER, value ? 1 : 0);
    input_sync(galaxybook.input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_input_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_input_init(struct samsung_galaxybook *galaxybook)
    {
    galaxybook.input = devm_input_allocate_device(&galaxybook.platform.dev);
    if (!galaxybook.input)
    return -ENOMEM;
    galaxybook.input.name = "Samsung Galaxy Book Camera Lens Cover";
    galaxybook.input.phys = DRIVER_NAME "/input0";
    galaxybook.input.id.bustype = BUS_HOST;
    input_set_capability(galaxybook.input, EV_KEY, KEY_MICMUTE);
    input_set_capability(galaxybook.input, EV_SW, SW_CAMERA_LENS_COVER);
    return input_register_device(galaxybook.input);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_block_recording_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_block_recording_init(struct samsung_galaxybook *galaxybook)
    {
    bool value;
    int err;
    err = galaxybook_enable_acpi_feature(galaxybook, GB_SASB_BLOCK_RECORDING);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "failed to initialize block_recording, error %d\n", err);
    return GB_NOT_SUPPORTED;
    }
    guard(mutex)(&galaxybook.fw_attr_lock);
    err = block_recording_acpi_get(galaxybook, &value);
    if (err) {
    dev_dbg(&galaxybook.platform.dev,
    "failed to get initial block_recording state, error %d\n", err);
    return GB_NOT_SUPPORTED;
    }
    input_report_switch(galaxybook.input, SW_CAMERA_LENS_COVER, value ? 1 : 0);
    input_sync(galaxybook.input);
    return 0;
    }
// Firmware Attributes setup
#[no_mangle]
unsafe extern "C" fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t type_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "enumeration\n");
    }
    let mut fw_attr_type: static struct kobj_attribute = __ATTR_RO(type);
#[no_mangle]
unsafe extern "C" fn default_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t default_value_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "0\n");
    }
    let mut fw_attr_default_value: static struct kobj_attribute = __ATTR_RO(default_value);
#[no_mangle]
unsafe extern "C" fn possible_values_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t possible_values_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "0;1\n");
    }
    let mut fw_attr_possible_values: static struct kobj_attribute = __ATTR_RO(possible_values);
    static ssize_t display_name_language_code_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n", GB_ATTR_LANGUAGE_CODE);
    }
    static struct kobj_attribute fw_attr_display_name_language_code =
    __ATTR_RO(display_name_language_code);
#[no_mangle]
unsafe extern "C" fn display_name_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t display_name_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    struct galaxybook_fw_attr *fw_attr =
    container_of(attr, struct galaxybook_fw_attr, display_name);
    return sysfs_emit(buf, "%s\n", galaxybook_fw_attr_desc[fw_attr.fw_attr_id]);
    }
#[no_mangle]
unsafe extern "C" fn current_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t current_value_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    struct galaxybook_fw_attr *fw_attr =
    container_of(attr, struct galaxybook_fw_attr, current_value);
    bool value;
    int err;
    err = fw_attr.get_value(fw_attr.galaxybook, &value);
    if (err)
    return err;
    return sysfs_emit(buf, "%u\n", value);
    }
    static ssize_t current_value_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct galaxybook_fw_attr *fw_attr =
    container_of(attr, struct galaxybook_fw_attr, current_value);
    struct samsung_galaxybook *galaxybook = fw_attr.galaxybook;
    bool value;
    int err;
    if (!count)
    return -EINVAL;
    err = kstrtobool(buf, &value);
    if (err)
    return err;
    guard(mutex)(&galaxybook.fw_attr_lock);
    err = fw_attr.set_value(galaxybook, value);
    if (err)
    return err;
    return count;
    }
pub const NUM_FW_ATTR_ENUM_ATTRS: c_int = 6;
    static int galaxybook_fw_attr_init(struct samsung_galaxybook *galaxybook,
    const enum galaxybook_fw_attr_id fw_attr_id,
    int (*get_value)(struct samsung_galaxybook *galaxybook,
    bool *value),
    int (*set_value)(struct samsung_galaxybook *galaxybook,
    const bool value))
    {
    struct galaxybook_fw_attr *fw_attr;
    struct attribute **attrs;
    fw_attr = devm_kzalloc(&galaxybook.platform.dev, sizeof(*fw_attr), GFP_KERNEL);
    if (!fw_attr)
    return -ENOMEM;
    attrs = devm_kcalloc(&galaxybook.platform.dev, NUM_FW_ATTR_ENUM_ATTRS + 1,
    sizeof(*attrs), GFP_KERNEL);
    if (!attrs)
    return -ENOMEM;
    attrs[0] = &fw_attr_type.attr;
    attrs[1] = &fw_attr_default_value.attr;
    attrs[2] = &fw_attr_possible_values.attr;
    attrs[3] = &fw_attr_display_name_language_code.attr;
    sysfs_attr_init(&fw_attr.display_name.attr);
    fw_attr.display_name.attr.name = "display_name";
    fw_attr.display_name.attr.mode = 0444;
    fw_attr.display_name.show = display_name_show;
    attrs[4] = &fw_attr.display_name.attr;
    sysfs_attr_init(&fw_attr.current_value.attr);
    fw_attr.current_value.attr.name = "current_value";
    fw_attr.current_value.attr.mode = 0644;
    fw_attr.current_value.show = current_value_show;
    fw_attr.current_value.store = current_value_store;
    attrs[5] = &fw_attr.current_value.attr;
    attrs[6] = core::ptr::null_mut();
    fw_attr.galaxybook = galaxybook;
    fw_attr.fw_attr_id = fw_attr_id;
    fw_attr.attr_group.name = galaxybook_fw_attr_name[fw_attr_id];
    fw_attr.attr_group.attrs = attrs;
    fw_attr.get_value = get_value;
    fw_attr.set_value = set_value;
    return sysfs_create_group(&galaxybook.fw_attrs_kset.kobj, &fw_attr.attr_group);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_kset_unregister(data: *mut c_void) {
    static void galaxybook_kset_unregister(void *data)
    {
    struct kset *kset = data;
    kset_unregister(kset);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_fw_attrs_dev_unregister(data: *mut c_void) {
    static void galaxybook_fw_attrs_dev_unregister(void *data)
    {
    struct device *fw_attrs_dev = data;
    device_unregister(fw_attrs_dev);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_fw_attrs_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_fw_attrs_init(struct samsung_galaxybook *galaxybook)
    {
    bool value;
    int err;
    err = devm_mutex_init(&galaxybook.platform.dev, &galaxybook.fw_attr_lock);
    if (err)
    return err;
    galaxybook.fw_attrs_dev = device_create(&firmware_attributes_class, core::ptr::null_mut(), MKDEV(0, 0),
    core::ptr::null_mut(), "%s", DRIVER_NAME);
    if (IS_ERR(galaxybook.fw_attrs_dev))
    return PTR_ERR(galaxybook.fw_attrs_dev);
    err = devm_add_action_or_reset(&galaxybook.platform.dev,
    galaxybook_fw_attrs_dev_unregister,
    galaxybook.fw_attrs_dev);
    if (err)
    return err;
    galaxybook.fw_attrs_kset = kset_create_and_add("attributes", core::ptr::null_mut(),
    &galaxybook.fw_attrs_dev.kobj);
    if (!galaxybook.fw_attrs_kset)
    return -ENOMEM;
    err = devm_add_action_or_reset(&galaxybook.platform.dev,
    galaxybook_kset_unregister, galaxybook.fw_attrs_kset);
    if (err)
    return err;
    err = power_on_lid_open_acpi_get(galaxybook, &value);
    if (!err) {
    err = galaxybook_fw_attr_init(galaxybook,
    GB_ATTR_POWER_ON_LID_OPEN,
    &power_on_lid_open_acpi_get,
    &power_on_lid_open_acpi_set);
    if (err)
    return err;
    }
    err = usb_charging_acpi_get(galaxybook, &value);
    if (!err) {
    err = galaxybook_fw_attr_init(galaxybook,
    GB_ATTR_USB_CHARGING,
    &usb_charging_acpi_get,
    &usb_charging_acpi_set);
    if (err)
    return err;
    }
    err = galaxybook_block_recording_init(galaxybook);
    if (err == GB_NOT_SUPPORTED)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    else if (err)
    return err;
    galaxybook.has_block_recording = true;
    return galaxybook_fw_attr_init(galaxybook,
    GB_ATTR_BLOCK_RECORDING,
    &block_recording_acpi_get,
    &block_recording_acpi_set);
    }
//
// Hotkeys and notifications
//
#[no_mangle]
unsafe extern "C" fn galaxybook_kbd_backlight_hotkey_work(work: *mut work_struct) {
    static void galaxybook_kbd_backlight_hotkey_work(struct work_struct *work)
    {
    struct samsung_galaxybook *galaxybook =
    from_work(galaxybook, work, kbd_backlight_hotkey_work);
    int brightness;
    int err;
    guard(mutex)(&galaxybook.kbd_backlight_lock);
    brightness = galaxybook.kbd_backlight.brightness;
    if (brightness < galaxybook.kbd_backlight.max_brightness)
    brightness++;
    else
    brightness = 0;
    err = led_set_brightness_sync(&galaxybook.kbd_backlight, brightness);
    if (err) {
    dev_err(&galaxybook.platform.dev,
    "failed to set kbd_backlight brightness, error %d\n", err);
    return;
    }
    led_classdev_notify_brightness_hw_changed(&galaxybook.kbd_backlight, brightness);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_block_recording_hotkey_work(work: *mut work_struct) {
    static void galaxybook_block_recording_hotkey_work(struct work_struct *work)
    {
    struct samsung_galaxybook *galaxybook =
    from_work(galaxybook, work, block_recording_hotkey_work);
    bool value;
    int err;
    guard(mutex)(&galaxybook.fw_attr_lock);
    err = block_recording_acpi_get(galaxybook, &value);
    if (err) {
    dev_err(&galaxybook.platform.dev,
    "failed to get block_recording, error %d\n", err);
    return;
    }
    err = block_recording_acpi_set(galaxybook, !value);
    if (err)
    dev_err(&galaxybook.platform.dev,
    "failed to set block_recording, error %d\n", err);
    }
    static bool galaxybook_i8042_filter(unsigned char data, unsigned char str, struct serio *port,
    void *context)
    {
    struct samsung_galaxybook *galaxybook = context;
    static bool extended;
    if (str & I8042_STR_AUXDATA)
    return false;
    if (data == 0xe0) {
    extended = true;
    return true;
    } else if (extended) {
    extended = false;
    switch (data) {
    case GB_KEY_KBD_BACKLIGHT_KEYDOWN:
    return true;
    case GB_KEY_KBD_BACKLIGHT_KEYUP:
    if (galaxybook.has_kbd_backlight)
    schedule_work(&galaxybook.kbd_backlight_hotkey_work);
    return true;
    case GB_KEY_BLOCK_RECORDING_KEYDOWN:
    return true;
    case GB_KEY_BLOCK_RECORDING_KEYUP:
    if (galaxybook.has_block_recording)
    schedule_work(&galaxybook.block_recording_hotkey_work);
    return true;
// battery notification already sent to battery + SCAI device
    case GB_KEY_BATTERY_NOTIFY_KEYUP:
    case GB_KEY_BATTERY_NOTIFY_KEYDOWN:
    return true;
    default:
//
// Report the previously filtered e0 before continuing
// with the next non-filtered byte.
//
    serio_interrupt(port, 0xe0, 0);
    return false;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_i8042_filter_remove(data: *mut c_void) {
    static void galaxybook_i8042_filter_remove(void *data)
    {
    struct samsung_galaxybook *galaxybook = data;
    i8042_remove_filter(galaxybook_i8042_filter);
    cancel_work_sync(&galaxybook.kbd_backlight_hotkey_work);
    cancel_work_sync(&galaxybook.block_recording_hotkey_work);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_i8042_filter_install(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_i8042_filter_install(struct samsung_galaxybook *galaxybook)
    {
    int err;
    if (!galaxybook.has_kbd_backlight && !galaxybook.has_block_recording)
    return 0;
    INIT_WORK(&galaxybook.kbd_backlight_hotkey_work,
    galaxybook_kbd_backlight_hotkey_work);
    INIT_WORK(&galaxybook.block_recording_hotkey_work,
    galaxybook_block_recording_hotkey_work);
    err = i8042_install_filter(galaxybook_i8042_filter, galaxybook);
    if (err)
    return err;
    return devm_add_action_or_reset(&galaxybook.platform.dev,
    galaxybook_i8042_filter_remove, galaxybook);
    }
//
// ACPI device setup
//
#[no_mangle]
unsafe extern "C" fn galaxybook_acpi_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void galaxybook_acpi_notify(acpi_handle handle, u32 event, void *data)
    {
    struct samsung_galaxybook *galaxybook = data;
    switch (event) {
    case GB_ACPI_NOTIFY_BATTERY_STATE_CHANGED:
    case GB_ACPI_NOTIFY_DEVICE_ON_TABLE:
    case GB_ACPI_NOTIFY_DEVICE_OFF_TABLE:
    break;
    case GB_ACPI_NOTIFY_HOTKEY_PERFORMANCE_MODE:
    if (galaxybook.has_performance_mode)
    platform_profile_cycle();
    break;
    case GB_ACPI_NOTIFY_HOTKEY_KBD_BACKLIGHT:
    if (galaxybook.has_kbd_backlight)
    schedule_work(&galaxybook.kbd_backlight_hotkey_work);
    break;
    case GB_ACPI_NOTIFY_HOTKEY_MICMUTE:
    input_report_key(galaxybook.input, KEY_MICMUTE, 1);
    input_sync(galaxybook.input);
    input_report_key(galaxybook.input, KEY_MICMUTE, 0);
    input_sync(galaxybook.input);
    break;
    case GB_ACPI_NOTIFY_HOTKEY_CAMERA:
    if (galaxybook.has_block_recording) {
    schedule_work(&galaxybook.block_recording_hotkey_work);
    } else {
    input_report_switch(galaxybook.input, SW_CAMERA_LENS_COVER,
    !test_bit(SW_CAMERA_LENS_COVER, galaxybook.input.sw));
    input_sync(galaxybook.input);
    }
    break;
    default:
    dev_warn(&galaxybook.platform.dev,
    "unknown ACPI notification event: 0x%x\n", event);
    }
    acpi_bus_generate_netlink_event(DRIVER_NAME, dev_name(&galaxybook.platform.dev),
    event, 1);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_enable_acpi_notify(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_enable_acpi_notify(struct samsung_galaxybook *galaxybook)
    {
    let mut buf: sawb = {};
    int err;
    err = galaxybook_enable_acpi_feature(galaxybook, GB_SASB_NOTIFICATIONS);
    if (err)
    return err;
    buf.safn = GB_SAFN;
    buf.sasb = GB_SASB_NOTIFICATIONS;
    buf.gunm = GB_GUNM_ACPI_NOTIFY_ENABLE;
    buf.guds[0] = GB_GUDS_ACPI_NOTIFY_ENABLE;
    return galaxybook_acpi_method(galaxybook, GB_ACPI_METHOD_SETTINGS,
    &buf, GB_SAWB_LEN_SETTINGS);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_acpi_remove_notify_handler(data: *mut c_void) {
    static void galaxybook_acpi_remove_notify_handler(void *data)
    {
    struct samsung_galaxybook *galaxybook = data;
    acpi_remove_notify_handler(galaxybook.acpi.handle, ACPI_ALL_NOTIFY,
    galaxybook_acpi_notify);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_acpi_disable(data: *mut c_void) {
    static void galaxybook_acpi_disable(void *data)
    {
    struct samsung_galaxybook *galaxybook = data;
    acpi_execute_simple_method(galaxybook.acpi.handle,
    GB_ACPI_METHOD_ENABLE, GB_ACPI_METHOD_ENABLE_OFF);
    }
#[no_mangle]
unsafe extern "C" fn galaxybook_acpi_init(galaxybook: *mut samsung_galaxybook) -> c_int {
    static int galaxybook_acpi_init(struct samsung_galaxybook *galaxybook)
    {
    acpi_status status;
    int err;
    status = acpi_execute_simple_method(galaxybook.acpi.handle, GB_ACPI_METHOD_ENABLE,
    GB_ACPI_METHOD_ENABLE_ON);
    if (ACPI_FAILURE(status))
    return -EIO;
    err = devm_add_action_or_reset(&galaxybook.platform.dev,
    galaxybook_acpi_disable, galaxybook);
    if (err)
    return err;
    status = acpi_install_notify_handler(galaxybook.acpi.handle, ACPI_ALL_NOTIFY,
    galaxybook_acpi_notify, galaxybook);
    if (ACPI_FAILURE(status))
    return -EIO;
    err = devm_add_action_or_reset(&galaxybook.platform.dev,
    galaxybook_acpi_remove_notify_handler, galaxybook);
    if (err)
    return err;
    err = galaxybook_enable_acpi_notify(galaxybook);
    if (err)
    dev_dbg(&galaxybook.platform.dev, "failed to enable ACPI notifications; "
    "some hotkeys will not be supported\n");
    err = galaxybook_enable_acpi_feature(galaxybook, GB_SASB_POWER_MANAGEMENT);
    if (err)
    dev_dbg(&galaxybook.platform.dev,
    "failed to initialize ACPI power management features; "
    "many features of this driver will not be available\n");
    return 0;
    }
//
// Platform driver
//
#[no_mangle]
unsafe extern "C" fn galaxybook_probe(pdev: *mut platform_device) -> c_int {
    static int galaxybook_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&pdev.dev);
    struct samsung_galaxybook *galaxybook;
    int err;
    if (!adev)
    return -ENODEV;
    galaxybook = devm_kzalloc(&pdev.dev, sizeof(*galaxybook), GFP_KERNEL);
    if (!galaxybook)
    return -ENOMEM;
    galaxybook.platform = pdev;
    galaxybook.acpi = adev;
//
// Features must be enabled and initialized in the following order to
// avoid failures seen on certain devices:
// - GB_SASB_POWER_MANAGEMENT (including performance mode)
// - GB_SASB_KBD_BACKLIGHT
// - GB_SASB_BLOCK_RECORDING (as part of fw_attrs init)
//
    err = galaxybook_acpi_init(galaxybook);
    if (err)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize ACPI device\n");
    err = galaxybook_platform_profile_init(galaxybook);
    if (!err)
    galaxybook.has_performance_mode = true;
#[no_mangle]
pub unsafe extern "C" fn if(GB_NOT_SUPPORTED: err !=) -> else {
    else if (err != GB_NOT_SUPPORTED)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize platform profile\n");
    err = galaxybook_battery_threshold_init(galaxybook);
    if (err)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize battery threshold\n");
    err = galaxybook_kbd_backlight_init(galaxybook);
    if (!err)
    galaxybook.has_kbd_backlight = true;
#[no_mangle]
pub unsafe extern "C" fn if(GB_NOT_SUPPORTED: err !=) -> else {
    else if (err != GB_NOT_SUPPORTED)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize kbd_backlight\n");
    err = galaxybook_input_init(galaxybook);
    if (err)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize input device\n");
    err = galaxybook_fw_attrs_init(galaxybook);
    if (err)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize firmware-attributes\n");
    err = galaxybook_i8042_filter_install(galaxybook);
    if (err)
    return dev_err_probe(&galaxybook.platform.dev, err,
    "failed to initialize i8042_filter\n");
    return 0;
    }
    static const struct acpi_device_id galaxybook_device_ids[] = {
    { "SAM0426" },
    { "SAM0427" },
    { "SAM0428" },
    { "SAM0429" },
    { "SAM0430" },
    { "SAMB430" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, galaxybook_device_ids);
    static struct platform_driver galaxybook_platform_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .acpi_match_table = galaxybook_device_ids,
    },
    .probe = galaxybook_probe,
    };
    module_platform_driver(galaxybook_platform_driver);
    MODULE_AUTHOR("Joshua Grisham <josh@joshuagrisham.com>");
    MODULE_DESCRIPTION("Samsung Galaxy Book driver");
    MODULE_LICENSE("GPL");
