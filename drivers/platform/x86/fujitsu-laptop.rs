//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/fujitsu-laptop.c
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
// -*-linux-c-*-
//
    Copyright (C) 2007,2008 Jonathan Woithe <jwoithe@just42.net>
    Copyright (C) 2008 Peter Gruber <nokos@gmx.net>
    Copyright (C) 2008 Tony Vroon <tony@linx.net>
    Based on earlier work:
    Copyright (C) 2003 Shane Spencer <shane@bogomip.com>
    Adrian Yee <brewt-fujitsu@brewt.org>
    Templated from msi-laptop.c and thinkpad_acpi.c which is copyright
    by its respective authors.
//
// fujitsu-laptop.c - Fujitsu laptop support, providing access to additional
// features made available on a range of Fujitsu laptops including the
// P2xxx/P5xxx/S2xxx/S6xxx/S7xxx series.
//
// This driver implements a vendor-specific backlight control interface for
// Fujitsu laptops and provides support for hotkeys present on certain Fujitsu
// laptops.
//
// This driver has been tested on a Fujitsu Lifebook S2110, S6410, S7020 and
// P8010.  It should work on most P-series and S-series Lifebooks, but
// YMMV.
//
// The module parameter use_alt_lcd_levels switches between different ACPI
// brightness controls which are used by different Fujitsu laptops.  In most
// cases the correct method is automatically detected. "use_alt_lcd_levels=1"
// is applicable for a Fujitsu Lifebook S6410 if autodetection fails.
//

pub const FUJITSU_LCD_N_LEVELS: c_int = 8;

pub const ACPI_FUJITSU_NOTIFY_CODE: c_uint = 0x80;
// FUNC interface - command values

// FUNC interface - responses
pub const UNSUPPORTED_CMD: c_uint = 0x80000000;
// FUNC interface - status flags

// FUNC interface - LED control

// FUNC interface - backlight power control

pub const BACKLIGHT_ON: c_int = 0;
// FUNC interface - battery control interface
pub const FUNC_S006_METHOD: c_uint = 0x1006;
pub const CHARGE_CONTROL_RW: c_uint = 0x21;
// Scancodes read from the GIRB register
pub const KEY1_CODE: c_uint = 0x410;
pub const KEY2_CODE: c_uint = 0x411;
pub const KEY3_CODE: c_uint = 0x412;
pub const KEY4_CODE: c_uint = 0x413;
pub const KEY5_CODE: c_uint = 0x414;
pub const KEY6_CODE: c_uint = 0x415;
pub const KEY7_CODE: c_uint = 0x416;
pub const KEY8_CODE: c_uint = 0x417;
pub const KEY9_CODE: c_uint = 0x420;
// Hotkey ringbuffer limits
pub const MAX_HOTKEY_RINGBUFFER_SIZE: c_int = 100;
pub const RINGBUFFERSIZE: c_int = 40;
// Module parameters
    let mut use_alt_lcd_levels: static int = -1;
    static bool disable_brightness_adjust;
// Device controlling the backlight and associated keys
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fujitsu_bl {
    pub input: *mut input_dev,
    pub phys: [c_char; 32],
    pub bl_device: *mut backlight_device,
    pub max_brightness: c_uint,
    pub brightness_level: c_uint,
}

    static struct fujitsu_bl *fujitsu_bl;
// Device used to access hotkeys and other features on the laptop
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fujitsu_laptop {
    pub input: *mut input_dev,
    pub phys: [c_char; 32],
    pub pf_device: *mut platform_device,
    pub fifo: kfifo,
    pub fifo_lock: spinlock_t,
    pub flags_supported: c_int,
    pub flags_state: c_int,
    pub charge_control_supported: bool,
}

    static struct device *fext;
// Fujitsu ACPI interface function
    static int call_fext_func(struct device *dev,
    int func, int op, int feature, int state)
    {
    union acpi_object params[4] = {
    { .integer.type = ACPI_TYPE_INTEGER, .integer.value = func },
    { .integer.type = ACPI_TYPE_INTEGER, .integer.value = op },
    { .integer.type = ACPI_TYPE_INTEGER, .integer.value = feature },
    { .integer.type = ACPI_TYPE_INTEGER, .integer.value = state }
    };
    let mut arg_list: acpi_object_list = { 4, params };
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    unsigned long long value;
    acpi_status status;
    status = acpi_evaluate_integer(handle, "FUNC", &arg_list, &value);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "Failed to evaluate FUNC\n");
    return -ENODEV;
    }
    acpi_handle_debug(handle, "FUNC 0x%x (args 0x%x, 0x%x, 0x%x) returned 0x%x\n",
    func, op, feature, state, (int)value);
    return value;
    }
// Battery charge control code
    static ssize_t charge_control_end_threshold_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int cc_end_value, s006_cc_return;
    unsigned int value;
    int ret;
    ret = kstrtouint(buf, 10, &value);
    if (ret)
    return ret;
    if (value > 100)
    return -EINVAL;
    if (value < 50)
    value = 50;
    cc_end_value = value * 0x100 + 0x20;
    s006_cc_return = call_fext_func(fext, FUNC_S006_METHOD,
    CHARGE_CONTROL_RW, cc_end_value, 0x0);
    if (s006_cc_return < 0)
    return s006_cc_return;
//
// The S006 0x21 method returns 0x00 in case the provided value
// is invalid.
//
    if (s006_cc_return == 0x00)
    return -EINVAL;
    return count;
    }
    static ssize_t charge_control_end_threshold_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    int status;
    status = call_fext_func(fext, FUNC_S006_METHOD,
    CHARGE_CONTROL_RW, 0x21, 0x0);
    if (status < 0)
    return status;
    return sysfs_emit(buf, "%d\n", status);
    }
    static DEVICE_ATTR_RW(charge_control_end_threshold);
// ACPI battery hook
    static int fujitsu_battery_add_hook(struct power_supply *battery,
    struct acpi_battery_hook *hook)
    {
    return device_create_file(&battery.dev,
    &dev_attr_charge_control_end_threshold);
    }
    static int fujitsu_battery_remove_hook(struct power_supply *battery,
    struct acpi_battery_hook *hook)
    {
    device_remove_file(&battery.dev,
    &dev_attr_charge_control_end_threshold);
    return 0;
    }
    static struct acpi_battery_hook battery_hook = {
    .add_battery = fujitsu_battery_add_hook,
    .remove_battery = fujitsu_battery_remove_hook,
    .name = "Fujitsu Battery Extension",
    };
//
// These functions are intended to be called from acpi_fujitsu_laptop_add and
// acpi_fujitsu_laptop_remove.
//
#[no_mangle]
unsafe extern "C" fn fujitsu_battery_charge_control_add(dev: *mut device) -> c_int {
    static int fujitsu_battery_charge_control_add(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    int s006_cc_return;
    priv.charge_control_supported = false;
//
// Check if the S006 0x21 method exists by trying to get the current
// battery charge limit.
//
    s006_cc_return = call_fext_func(fext, FUNC_S006_METHOD,
    CHARGE_CONTROL_RW, 0x21, 0x0);
    if (s006_cc_return < 0)
    return s006_cc_return;
    if (s006_cc_return == UNSUPPORTED_CMD)
    return -ENODEV;
    priv.charge_control_supported = true;
    battery_hook_register(&battery_hook);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_battery_charge_control_remove(dev: *mut device) {
    static void fujitsu_battery_charge_control_remove(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    if (priv.charge_control_supported)
    battery_hook_unregister(&battery_hook);
    }
// Hardware access for LCD brightness control
#[no_mangle]
unsafe extern "C" fn set_lcd_level(dev: *mut device, level: c_int) -> c_int {
    static int set_lcd_level(struct device *dev, int level)
    {
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    acpi_status status;
    char *method;
    switch (use_alt_lcd_levels) {
    case -1:
    if (acpi_has_method(handle, "SBL2"))
    method = "SBL2";
    else
    method = "SBLL";
    break;
    case 1:
    method = "SBL2";
    break;
    default:
    method = "SBLL";
    break;
    }
    acpi_handle_debug(handle, "set lcd level via %s [%d]\n", method, level);
    if (level < 0 || level >= priv.max_brightness)
    return -EINVAL;
    status = acpi_execute_simple_method(handle, method, level);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "Failed to evaluate %s\n", method);
    return -ENODEV;
    }
    priv.brightness_level = level;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_lcd_level(dev: *mut device) -> c_int {
    static int get_lcd_level(struct device *dev)
    {
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    let mut state: c_ulonglong = 0;
    let mut status: acpi_status = AE_OK;
    acpi_handle_debug(handle, "get lcd level via GBLL\n");
    status = acpi_evaluate_integer(handle, "GBLL", core::ptr::null_mut(), &state);
    if (ACPI_FAILURE(status))
    return 0;
    priv.brightness_level = state & 0x0fffffff;
    return priv.brightness_level;
    }
#[no_mangle]
unsafe extern "C" fn get_max_brightness(dev: *mut device) -> c_int {
    static int get_max_brightness(struct device *dev)
    {
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    let mut state: c_ulonglong = 0;
    let mut status: acpi_status = AE_OK;
    acpi_handle_debug(handle, "get max lcd level via RBLL\n");
    status = acpi_evaluate_integer(handle, "RBLL", core::ptr::null_mut(), &state);
    if (ACPI_FAILURE(status))
    return -1;
    priv.max_brightness = state;
    return priv.max_brightness;
    }
// Backlight device stuff
#[no_mangle]
unsafe extern "C" fn bl_get_brightness(b: *mut backlight_device) -> c_int {
    static int bl_get_brightness(struct backlight_device *b)
    {
    struct device *dev = bl_get_data(b);
    return b.props.power == BACKLIGHT_POWER_OFF ? 0 : get_lcd_level(dev);
    }
#[no_mangle]
unsafe extern "C" fn bl_update_status(b: *mut backlight_device) -> c_int {
    static int bl_update_status(struct backlight_device *b)
    {
    if (fext) {
    if (b.props.power == BACKLIGHT_POWER_OFF)
    call_fext_func(fext, FUNC_BACKLIGHT, 0x1,
    BACKLIGHT_PARAM_POWER, BACKLIGHT_OFF);
    else
    call_fext_func(fext, FUNC_BACKLIGHT, 0x1,
    BACKLIGHT_PARAM_POWER, BACKLIGHT_ON);
    }
    return set_lcd_level(bl_get_data(b), b.props.brightness);
    }
    static const struct backlight_ops fujitsu_bl_ops = {
    .get_brightness = bl_get_brightness,
    .update_status = bl_update_status,
    };
    static ssize_t lid_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    if (!(priv.flags_supported & FLAG_LID))
    return sysfs_emit(buf, "unknown\n");
    if (priv.flags_state & FLAG_LID)
    return sysfs_emit(buf, "open\n");
    else
    return sysfs_emit(buf, "closed\n");
    }
    static ssize_t dock_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    if (!(priv.flags_supported & FLAG_DOCK))
    return sysfs_emit(buf, "unknown\n");
    if (priv.flags_state & FLAG_DOCK)
    return sysfs_emit(buf, "docked\n");
    else
    return sysfs_emit(buf, "undocked\n");
    }
    static ssize_t radios_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    if (!(priv.flags_supported & FLAG_RFKILL))
    return sysfs_emit(buf, "unknown\n");
    if (priv.flags_state & FLAG_RFKILL)
    return sysfs_emit(buf, "on\n");
    else
    return sysfs_emit(buf, "killed\n");
    }
    static DEVICE_ATTR_RO(lid);
    static DEVICE_ATTR_RO(dock);
    static DEVICE_ATTR_RO(radios);
    static struct attribute *fujitsu_pf_attributes[] = {
    &dev_attr_lid.attr,
    &dev_attr_dock.attr,
    &dev_attr_radios.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group fujitsu_pf_attribute_group = {
    .attrs = fujitsu_pf_attributes
    };
    static struct platform_driver fujitsu_pf_driver = {
    .driver = {
    .name = "fujitsu-laptop",
    }
    };
// ACPI device for LCD brightness control
    static const struct key_entry keymap_backlight[] = {
    { KE_KEY, true, { KEY_BRIGHTNESSUP } },
    { KE_KEY, false, { KEY_BRIGHTNESSDOWN } },
    { KE_END, 0 }
    };
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_bl_input_setup(dev: *mut device) -> c_int {
    static int acpi_fujitsu_bl_input_setup(struct device *dev)
    {
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    struct acpi_device *device = ACPI_COMPANION(dev);
    int ret;
    priv.input = devm_input_allocate_device(dev);
    if (!priv.input)
    return -ENOMEM;
    snprintf(priv.phys, sizeof(priv.phys), "%s/video/input0",
    acpi_device_hid(device));
    priv.input.name = ACPI_FUJITSU_BL_DEVICE_NAME;
    priv.input.phys = priv.phys;
    priv.input.id.bustype = BUS_HOST;
    priv.input.id.product = 0x06;
    ret = sparse_keymap_setup(priv.input, keymap_backlight, core::ptr::null_mut());
    if (ret)
    return ret;
    return input_register_device(priv.input);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_backlight_register(dev: *mut device) -> c_int {
    static int fujitsu_backlight_register(struct device *dev)
    {
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    const struct backlight_properties props = {
    .brightness = priv.brightness_level,
    .max_brightness = priv.max_brightness - 1,
    .type = BACKLIGHT_PLATFORM
    };
    struct backlight_device *bd;
    bd = devm_backlight_device_register(dev, "fujitsu-laptop",
    dev, dev, &fujitsu_bl_ops, &props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
    priv.bl_device = bd;
    return 0;
    }
// Brightness notify
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_bl_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_fujitsu_bl_notify(acpi_handle handle, u32 event, void *data)
    {
    struct device *dev = data;
    struct fujitsu_bl *priv = dev_get_drvdata(dev);
    int oldb, newb;
    if (event != ACPI_FUJITSU_NOTIFY_CODE) {
    acpi_handle_info(handle, "unsupported event [0x%x]\n", event);
    sparse_keymap_report_event(priv.input, -1, 1, true);
    return;
    }
    oldb = priv.brightness_level;
    get_lcd_level(dev);
    newb = priv.brightness_level;
    acpi_handle_debug(handle, "brightness button event [%i . %i]\n",
    oldb, newb);
    if (oldb == newb)
    return;
    if (!disable_brightness_adjust)
    set_lcd_level(dev, newb);
    sparse_keymap_report_event(priv.input, oldb < newb, 1, true);
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_bl_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_fujitsu_bl_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    struct fujitsu_bl *priv;
    int ret;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    if (acpi_video_get_backlight_type() != acpi_backlight_vendor)
    return -ENODEV;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    fujitsu_bl = priv;
    platform_set_drvdata(pdev, priv);
    pr_info("ACPI: %s [%s]\n", ACPI_FUJITSU_BL_DEVICE_NAME,
    acpi_device_bid(device));
    if (get_max_brightness(&pdev.dev) <= 0)
    priv.max_brightness = FUJITSU_LCD_N_LEVELS;
    get_lcd_level(&pdev.dev);
    ret = acpi_fujitsu_bl_input_setup(&pdev.dev);
    if (ret)
    return ret;
    ret = fujitsu_backlight_register(&pdev.dev);
    if (ret)
    return ret;
    return acpi_dev_install_notify_handler(device, ACPI_DEVICE_NOTIFY,
    acpi_fujitsu_bl_notify, &pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_bl_remove(pdev: *mut platform_device) {
    static void acpi_fujitsu_bl_remove(struct platform_device *pdev)
    {
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev),
    ACPI_DEVICE_NOTIFY, acpi_fujitsu_bl_notify);
    }
// ACPI device for hotkey handling
    static const struct key_entry keymap_default[] = {
    { KE_KEY, KEY1_CODE,            { KEY_PROG1 } },
    { KE_KEY, KEY2_CODE,            { KEY_PROG2 } },
    { KE_KEY, KEY3_CODE,            { KEY_PROG3 } },
    { KE_KEY, KEY4_CODE,            { KEY_PROG4 } },
    { KE_KEY, KEY9_CODE,            { KEY_RFKILL } },
// Soft keys read from status flags
    { KE_KEY, FLAG_RFKILL,          { KEY_RFKILL } },
    { KE_KEY, FLAG_TOUCHPAD_TOGGLE, { KEY_TOUCHPAD_TOGGLE } },
    { KE_KEY, FLAG_MICMUTE,         { KEY_MICMUTE } },
    { KE_END, 0 }
    };
    static const struct key_entry keymap_s64x0[] = {
    { KE_KEY, KEY1_CODE, { KEY_SCREENLOCK } },	/* "Lock" */
    { KE_KEY, KEY2_CODE, { KEY_HELP } },		/* "Mobility Center */
    { KE_KEY, KEY3_CODE, { KEY_PROG3 } },
    { KE_KEY, KEY4_CODE, { KEY_PROG4 } },
    { KE_END, 0 }
    };
    static const struct key_entry keymap_p8010[] = {
    { KE_KEY, KEY1_CODE, { KEY_HELP } },		/* "Support" */
    { KE_KEY, KEY2_CODE, { KEY_PROG2 } },
    { KE_KEY, KEY3_CODE, { KEY_SWITCHVIDEOMODE } },	/* "Presentation" */
    { KE_KEY, KEY4_CODE, { KEY_WWW } },		/* "WWW" */
    { KE_END, 0 }
    };
    static const struct key_entry keymap_s2110[] = {
    { KE_KEY, KEY1_CODE, { KEY_PROG1 } }, /* "A" */
    { KE_KEY, KEY2_CODE, { KEY_PROG2 } }, /* "B" */
    { KE_KEY, KEY3_CODE, { KEY_WWW } },   /* "Internet" */
    { KE_KEY, KEY4_CODE, { KEY_EMAIL } }, /* "E-mail" */
    { KE_KEY, KEY5_CODE, { KEY_STOPCD } },
    { KE_KEY, KEY6_CODE, { KEY_PLAYPAUSE } },
    { KE_KEY, KEY7_CODE, { KEY_PREVIOUSSONG } },
    { KE_KEY, KEY8_CODE, { KEY_NEXTSONG } },
    { KE_END, 0 }
    };
    static const struct key_entry *keymap = keymap_default;
#[no_mangle]
unsafe extern "C" fn fujitsu_laptop_dmi_keymap_override(id: *const dmi_system_id) -> c_int {
    static int fujitsu_laptop_dmi_keymap_override(const struct dmi_system_id *id)
    {
    pr_info("Identified laptop model '%s'\n", id.ident);
    keymap = id.driver_data;
    return 1;
    }
    static const struct dmi_system_id fujitsu_laptop_dmi_table[] = {
    {
    .callback = fujitsu_laptop_dmi_keymap_override,
    .ident = "Fujitsu Siemens S6410",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK S6410"),
    },
    .driver_data = (void *)keymap_s64x0
    },
    {
    .callback = fujitsu_laptop_dmi_keymap_override,
    .ident = "Fujitsu Siemens S6420",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK S6420"),
    },
    .driver_data = (void *)keymap_s64x0
    },
    {
    .callback = fujitsu_laptop_dmi_keymap_override,
    .ident = "Fujitsu LifeBook P8010",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LifeBook P8010"),
    },
    .driver_data = (void *)keymap_p8010
    },
    {
    .callback = fujitsu_laptop_dmi_keymap_override,
    .ident = "Fujitsu LifeBook S2110",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK S2110"),
    },
    .driver_data = (void *)keymap_s2110
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_input_setup(dev: *mut device) -> c_int {
    static int acpi_fujitsu_laptop_input_setup(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    struct acpi_device *device = ACPI_COMPANION(dev);
    int ret;
    priv.input = devm_input_allocate_device(dev);
    if (!priv.input)
    return -ENOMEM;
    snprintf(priv.phys, sizeof(priv.phys), "%s/input0",
    acpi_device_hid(device));
    priv.input.name = ACPI_FUJITSU_LAPTOP_DEVICE_NAME;
    priv.input.phys = priv.phys;
    priv.input.id.bustype = BUS_HOST;
    dmi_check_system(fujitsu_laptop_dmi_table);
    ret = sparse_keymap_setup(priv.input, keymap, core::ptr::null_mut());
    if (ret)
    return ret;
    return input_register_device(priv.input);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_laptop_platform_add(dev: *mut device) -> c_int {
    static int fujitsu_laptop_platform_add(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    int ret;
    priv.pf_device = platform_device_alloc("fujitsu-laptop", PLATFORM_DEVID_NONE);
    if (!priv.pf_device)
    return -ENOMEM;
    platform_set_drvdata(priv.pf_device, priv);
    ret = platform_device_add(priv.pf_device);
    if (ret)
    goto err_put_platform_device;
    ret = sysfs_create_group(&priv.pf_device.dev.kobj,
    &fujitsu_pf_attribute_group);
    if (ret)
    goto err_del_platform_device;
    return 0;
    err_del_platform_device:
    platform_device_del(priv.pf_device);
    err_put_platform_device:
    platform_device_put(priv.pf_device);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_laptop_platform_remove(dev: *mut device) {
    static void fujitsu_laptop_platform_remove(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    sysfs_remove_group(&priv.pf_device.dev.kobj,
    &fujitsu_pf_attribute_group);
    platform_device_unregister(priv.pf_device);
    }
    static int logolamp_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct device *parent = cdev.dev.parent;
    let mut poweron: c_int = FUNC_LED_ON, always = FUNC_LED_ON;
    int ret;
    if (brightness < LED_HALF)
    poweron = FUNC_LED_OFF;
    if (brightness < LED_FULL)
    always = FUNC_LED_OFF;
    ret = call_fext_func(parent, FUNC_LEDS, 0x1, LOGOLAMP_POWERON, poweron);
    if (ret < 0)
    return ret;
    return call_fext_func(parent, FUNC_LEDS, 0x1, LOGOLAMP_ALWAYS, always);
    }
#[no_mangle]
unsafe extern "C" fn logolamp_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness logolamp_get(struct led_classdev *cdev)
    {
    struct device *parent = cdev.dev.parent;
    int ret;
    ret = call_fext_func(parent, FUNC_LEDS, 0x2, LOGOLAMP_ALWAYS, 0x0);
    if (ret == FUNC_LED_ON)
    return LED_FULL;
    ret = call_fext_func(parent, FUNC_LEDS, 0x2, LOGOLAMP_POWERON, 0x0);
    if (ret == FUNC_LED_ON)
    return LED_HALF;
    return LED_OFF;
    }
    static int kblamps_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct device *parent = cdev.dev.parent;
    if (brightness >= LED_FULL)
    return call_fext_func(parent, FUNC_LEDS, 0x1, KEYBOARD_LAMPS,
    FUNC_LED_ON);
    else
    return call_fext_func(parent, FUNC_LEDS, 0x1, KEYBOARD_LAMPS,
    FUNC_LED_OFF);
    }
#[no_mangle]
unsafe extern "C" fn kblamps_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness kblamps_get(struct led_classdev *cdev)
    {
    let mut brightness: enum led_brightness = LED_OFF;
    if (call_fext_func(cdev.dev.parent,
    FUNC_LEDS, 0x2, KEYBOARD_LAMPS, 0x0) == FUNC_LED_ON)
    brightness = LED_FULL;
    return brightness;
    }
    static int radio_led_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct device *parent = cdev.dev.parent;
    if (brightness >= LED_FULL)
    return call_fext_func(parent, FUNC_FLAGS, 0x5, RADIO_LED_ON,
    RADIO_LED_ON);
    else
    return call_fext_func(parent, FUNC_FLAGS, 0x5, RADIO_LED_ON,
    0x0);
    }
#[no_mangle]
unsafe extern "C" fn radio_led_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness radio_led_get(struct led_classdev *cdev)
    {
    struct device *parent = cdev.dev.parent;
    let mut brightness: enum led_brightness = LED_OFF;
    if (call_fext_func(parent, FUNC_FLAGS, 0x4, 0x0, 0x0) & RADIO_LED_ON)
    brightness = LED_FULL;
    return brightness;
    }
    static int eco_led_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct device *parent = cdev.dev.parent;
    int curr;
    curr = call_fext_func(parent, FUNC_LEDS, 0x2, ECO_LED, 0x0);
    if (brightness >= LED_FULL)
    return call_fext_func(parent, FUNC_LEDS, 0x1, ECO_LED,
    curr | ECO_LED_ON);
    else
    return call_fext_func(parent, FUNC_LEDS, 0x1, ECO_LED,
    curr & ~ECO_LED_ON);
    }
#[no_mangle]
unsafe extern "C" fn eco_led_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness eco_led_get(struct led_classdev *cdev)
    {
    struct device *parent = cdev.dev.parent;
    let mut brightness: enum led_brightness = LED_OFF;
    if (call_fext_func(parent, FUNC_LEDS, 0x2, ECO_LED, 0x0) & ECO_LED_ON)
    brightness = LED_FULL;
    return brightness;
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_leds_register(dev: *mut device) -> c_int {
    static int acpi_fujitsu_laptop_leds_register(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    struct led_classdev *led;
    int ret;
    if (call_fext_func(dev, FUNC_LEDS, 0x0, 0x0, 0x0) & LOGOLAMP_POWERON) {
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.name = "fujitsu::logolamp";
    led.brightness_set_blocking = logolamp_set;
    led.brightness_get = logolamp_get;
    ret = devm_led_classdev_register(dev, led);
    if (ret)
    return ret;
    }
    if ((call_fext_func(dev, FUNC_LEDS, 0x0, 0x0, 0x0) & KEYBOARD_LAMPS) &&
    (call_fext_func(dev, FUNC_BUTTONS, 0x0, 0x0, 0x0) == 0x0)) {
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.name = "fujitsu::kblamps";
    led.brightness_set_blocking = kblamps_set;
    led.brightness_get = kblamps_get;
    ret = devm_led_classdev_register(dev, led);
    if (ret)
    return ret;
    }
//
// Some Fujitsu laptops have a radio toggle button in place of a slide
// switch and all such machines appear to also have an RF LED.  Based on
// comparing DSDT tables of four Fujitsu Lifebook models (E744, E751,
// S7110, S8420; the first one has a radio toggle button, the other
// three have slide switches), bit 17 of flags_supported (the value
// returned by method S000 of ACPI device FUJ02E3) seems to indicate
// whether given model has a radio toggle button.
//
    if (priv.flags_supported & BIT(17)) {
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.name = "fujitsu::radio_led";
    led.brightness_set_blocking = radio_led_set;
    led.brightness_get = radio_led_get;
    led.default_trigger = "rfkill-any";
    ret = devm_led_classdev_register(dev, led);
    if (ret)
    return ret;
    }
// Support for eco led is not always signaled in bit corresponding
// to the bit used to control the led. According to the DSDT table,
// bit 14 seems to indicate presence of said led as well.
// Confirm by testing the status.
//
    if ((call_fext_func(dev, FUNC_LEDS, 0x0, 0x0, 0x0) & BIT(14)) &&
    (call_fext_func(dev, FUNC_LEDS, 0x2, ECO_LED, 0x0) != UNSUPPORTED_CMD)) {
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.name = "fujitsu::eco_led";
    led.brightness_set_blocking = eco_led_set;
    led.brightness_get = eco_led_get;
    ret = devm_led_classdev_register(dev, led);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_press(dev: *mut device, scancode: c_int) {
    static void acpi_fujitsu_laptop_press(struct device *dev, int scancode)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    int ret;
    ret = kfifo_in_locked(&priv.fifo, (unsigned char *)&scancode,
    sizeof(scancode), &priv.fifo_lock);
    if (ret != sizeof(scancode)) {
    dev_info(&priv.input.dev, "Could not push scancode [0x%x]\n",
    scancode);
    return;
    }
    sparse_keymap_report_event(priv.input, scancode, 1, false);
    dev_dbg(&priv.input.dev, "Push scancode into ringbuffer [0x%x]\n",
    scancode);
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_release(dev: *mut device) {
    static void acpi_fujitsu_laptop_release(struct device *dev)
    {
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    int scancode, ret;
    while (true) {
    ret = kfifo_out_locked(&priv.fifo, (unsigned char *)&scancode,
    sizeof(scancode), &priv.fifo_lock);
    if (ret != sizeof(scancode))
    return;
    sparse_keymap_report_event(priv.input, scancode, 0, false);
    dev_dbg(&priv.input.dev,
    "Pop scancode from ringbuffer [0x%x]\n", scancode);
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_fujitsu_laptop_notify(acpi_handle handle, u32 event, void *data)
    {
    struct device *dev = data;
    struct fujitsu_laptop *priv = dev_get_drvdata(dev);
    unsigned long flags;
    int scancode, i = 0;
    unsigned int irb;
    if (event != ACPI_FUJITSU_NOTIFY_CODE) {
    acpi_handle_info(handle, "Unsupported event [0x%x]\n", event);
    sparse_keymap_report_event(priv.input, -1, 1, true);
    return;
    }
    if (priv.flags_supported)
    priv.flags_state = call_fext_func(dev, FUNC_FLAGS, 0x4, 0x0, 0x0);
    while ((irb = call_fext_func(dev, FUNC_BUTTONS, 0x1, 0x0, 0x0)) != 0 &&
    i++ < MAX_HOTKEY_RINGBUFFER_SIZE) {
    scancode = irb & 0x4ff;
    if (sparse_keymap_entry_from_scancode(priv.input, scancode))
    acpi_fujitsu_laptop_press(dev, scancode);
#[no_mangle]
pub unsafe extern "C" fn if(0: scancode ==) -> else {
    else if (scancode == 0)
    acpi_fujitsu_laptop_release(dev);
    else
    acpi_handle_info(handle, "Unknown GIRB result [%x]\n", irb);
    }
//
// First seen on the Skylake-based Lifebook E736/E746/E756), the
// touchpad toggle hotkey (Fn+F4) is handled in software. Other models
// have since added additional "soft keys". These are reported in the
// status flags queried using FUNC_FLAGS.
//
    if (priv.flags_supported & (FLAG_SOFTKEYS)) {
    flags = call_fext_func(dev, FUNC_FLAGS, 0x1, 0x0, 0x0);
    flags &= (FLAG_SOFTKEYS);
    for_each_set_bit(i, &flags, BITS_PER_LONG)
    sparse_keymap_report_event(priv.input, BIT(i), 1, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_fujitsu_laptop_probe(struct platform_device *pdev)
    {
    struct fujitsu_laptop *priv;
    struct acpi_device *device;
    int ret, i = 0;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    WARN_ONCE(fext, "More than one FUJ02E3 ACPI device was found.  Driver may not work as intended.");
    fext = &pdev.dev;
    platform_set_drvdata(pdev, priv);
// kfifo
    spin_lock_init(&priv.fifo_lock);
    ret = kfifo_alloc(&priv.fifo, RINGBUFFERSIZE * sizeof(int),
    GFP_KERNEL);
    if (ret)
    return ret;
    pr_info("ACPI: %s [%s]\n", ACPI_FUJITSU_LAPTOP_DEVICE_NAME,
    acpi_device_bid(device));
    while (call_fext_func(fext, FUNC_BUTTONS, 0x1, 0x0, 0x0) != 0 &&
    i++ < MAX_HOTKEY_RINGBUFFER_SIZE)
    ; /* No action, result is discarded */
    acpi_handle_debug(device.handle, "Discarded %i ringbuffer entries\n",
    i);
    priv.flags_supported = call_fext_func(fext, FUNC_FLAGS, 0x0, 0x0, 0x0);
// Make sure our bitmask of supported functions is cleared if the
    RFKILL function block is not implemented, like on the S7020. */
    if (priv.flags_supported == UNSUPPORTED_CMD)
    priv.flags_supported = 0;
    if (priv.flags_supported)
    priv.flags_state = call_fext_func(fext, FUNC_FLAGS, 0x4, 0x0,
    0x0);
// Suspect this is a keymap of the application panel, print it
    acpi_handle_info(device.handle, "BTNI: [0x%x]\n",
    call_fext_func(fext, FUNC_BUTTONS, 0x0, 0x0, 0x0));
// Sync backlight power status
    if (fujitsu_bl && fujitsu_bl.bl_device &&
    acpi_video_get_backlight_type() == acpi_backlight_vendor) {
    if (call_fext_func(fext, FUNC_BACKLIGHT, 0x2,
    BACKLIGHT_PARAM_POWER, 0x0) == BACKLIGHT_OFF)
    fujitsu_bl.bl_device.props.power = BACKLIGHT_POWER_OFF;
    else
    fujitsu_bl.bl_device.props.power = BACKLIGHT_POWER_ON;
    }
    ret = acpi_fujitsu_laptop_input_setup(fext);
    if (ret)
    goto err_free_fifo;
    ret = acpi_fujitsu_laptop_leds_register(fext);
    if (ret)
    goto err_free_fifo;
    ret = fujitsu_laptop_platform_add(fext);
    if (ret)
    goto err_free_fifo;
    ret = acpi_dev_install_notify_handler(device, ACPI_DEVICE_NOTIFY,
    acpi_fujitsu_laptop_notify, fext);
    if (ret)
    goto err_platform_remove;
    ret = fujitsu_battery_charge_control_add(fext);
    if (ret < 0)
    pr_warn("Unable to register battery charge control: %d\n", ret);
    return 0;
    err_platform_remove:
    fujitsu_laptop_platform_remove(fext);
    err_free_fifo:
    kfifo_free(&priv.fifo);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_laptop_remove(pdev: *mut platform_device) {
    static void acpi_fujitsu_laptop_remove(struct platform_device *pdev)
    {
    struct fujitsu_laptop *priv = platform_get_drvdata(pdev);
    fujitsu_battery_charge_control_remove(&pdev.dev);
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev), ACPI_DEVICE_NOTIFY,
    acpi_fujitsu_laptop_notify);
    fujitsu_laptop_platform_remove(&pdev.dev);
    kfifo_free(&priv.fifo);
    }
// Initialization
    static const struct acpi_device_id fujitsu_bl_device_ids[] = {
    {ACPI_FUJITSU_BL_HID, 0},
    {"", 0},
    };
    static struct platform_driver acpi_fujitsu_bl_driver = {
    .probe = acpi_fujitsu_bl_probe,
    .remove = acpi_fujitsu_bl_remove,
    .driver = {
    .name = ACPI_FUJITSU_BL_DRIVER_NAME,
    .acpi_match_table = fujitsu_bl_device_ids,
    },
    };
    static const struct acpi_device_id fujitsu_laptop_device_ids[] = {
    {ACPI_FUJITSU_LAPTOP_HID, 0},
    {"", 0},
    };
    static struct platform_driver acpi_fujitsu_laptop_driver = {
    .probe = acpi_fujitsu_laptop_probe,
    .remove = acpi_fujitsu_laptop_remove,
    .driver = {
    .name = ACPI_FUJITSU_LAPTOP_DRIVER_NAME,
    .acpi_match_table = fujitsu_laptop_device_ids,
    },
    };
    static const struct acpi_device_id fujitsu_ids[] __used = {
    {ACPI_FUJITSU_BL_HID, 0},
    {ACPI_FUJITSU_LAPTOP_HID, 0},
    {"", 0}
    };
    MODULE_DEVICE_TABLE(acpi, fujitsu_ids);
#[no_mangle]
unsafe extern "C" fn fujitsu_init() -> int __init {
    static int __init fujitsu_init(void)
    {
    int ret;
    ret = platform_driver_register(&acpi_fujitsu_bl_driver);
    if (ret)
    return ret;
// Register platform stuff
    ret = platform_driver_register(&fujitsu_pf_driver);
    if (ret)
    goto err_unregister_acpi;
// Register laptop driver
    ret = platform_driver_register(&acpi_fujitsu_laptop_driver);
    if (ret)
    goto err_unregister_platform_driver;
    pr_info("driver " FUJITSU_DRIVER_VERSION " successfully loaded\n");
    return 0;
    err_unregister_platform_driver:
    platform_driver_unregister(&fujitsu_pf_driver);
    err_unregister_acpi:
    platform_driver_unregister(&acpi_fujitsu_bl_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_cleanup() -> void __exit {
    static void __exit fujitsu_cleanup(void)
    {
    platform_driver_unregister(&acpi_fujitsu_laptop_driver);
    platform_driver_unregister(&fujitsu_pf_driver);
    platform_driver_unregister(&acpi_fujitsu_bl_driver);
    pr_info("driver unloaded\n");
    }
    module_init(fujitsu_init);
    module_exit(fujitsu_cleanup);
    module_param(use_alt_lcd_levels, int, 0644);
    MODULE_PARM_DESC(use_alt_lcd_levels, "Interface used for setting LCD brightness level (-1 = auto, 0 = force SBLL, 1 = force SBL2)");
    module_param(disable_brightness_adjust, bool, 0644);
    MODULE_PARM_DESC(disable_brightness_adjust, "Disable LCD brightness adjustment");
    MODULE_AUTHOR("Jonathan Woithe, Peter Gruber, Tony Vroon");
    MODULE_DESCRIPTION("Fujitsu laptop extras support");
    MODULE_VERSION(FUJITSU_DRIVER_VERSION);
    MODULE_LICENSE("GPL");
