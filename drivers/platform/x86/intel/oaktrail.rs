//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/oaktrail.c
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
// Intel OakTrail Platform support
//
// Copyright (C) 2010-2011 Intel Corporation
// Author: Yin Kangkai (kangkai.yin@intel.com)
//
// based on Compal driver, Copyright (C) 2008 Cezary Jackiewicz
// <cezary.jackiewicz (at) gmail.com>, based on MSI driver
// Copyright (C) 2006 Lennart Poettering <mzxreary (at) 0pointer (dot) de>
//
// This driver does below things:
// 1. registers itself in the Linux backlight control in
// /sys/class/backlight/intel_oaktrail
//
// 2. registers in the rfkill subsystem here: /sys/class/rfkill/rfkillX
// for these components: wifi, bluetooth, wwan (3g), gps
//
// This driver might work on other products based on Oaktrail. If you
// want to try it you can pass force=1 as argument to the module which
// will force it to load even when the DMI data doesn't identify the
// product as compatible.
//

//
// This is the devices status address in EC space, and the control bits
// definition:
//
// (1 << 0):	Camera enable/disable, RW.
// (1 << 1):	Bluetooth enable/disable, RW.
// (1 << 2):	GPS enable/disable, RW.
// (1 << 3):	WiFi enable/disable, RW.
// (1 << 4):	WWAN (3G) enable/disable, RW.
// (1 << 5):	Touchscreen enable/disable, Read Only.
//
pub const OT_EC_DEVICE_STATE_ADDRESS: c_uint = 0xD6;

//
// This is the address in EC space and commands used to control LCD backlight:
//
// Two steps needed to change the LCD backlight:
// 1. write the backlight percentage into OT_EC_BL_BRIGHTNESS_ADDRESS;
// 2. write OT_EC_BL_CONTROL_ON_DATA into OT_EC_BL_CONTROL_ADDRESS.
//
// To read the LCD back light, just read out the value from
// OT_EC_BL_BRIGHTNESS_ADDRESS.
//
// LCD backlight brightness range: 0 - 100 (OT_EC_BL_BRIGHTNESS_MAX)
//
pub const OT_EC_BL_BRIGHTNESS_ADDRESS: c_uint = 0x44;
pub const OT_EC_BL_BRIGHTNESS_MAX: c_int = 100;
pub const OT_EC_BL_CONTROL_ADDRESS: c_uint = 0x3A;
pub const OT_EC_BL_CONTROL_ON_DATA: c_uint = 0x1A;
    static bool force;
    module_param(force, bool, 0);
    MODULE_PARM_DESC(force, "Force driver load, ignore DMI data");
    static struct platform_device *oaktrail_device;
    static struct backlight_device *oaktrail_bl_device;
    static struct rfkill *bt_rfkill;
    static struct rfkill *gps_rfkill;
    static struct rfkill *wifi_rfkill;
    static struct rfkill *wwan_rfkill;
// rfkill
#[no_mangle]
unsafe extern "C" fn oaktrail_rfkill_set(data: *mut c_void, blocked: bool) -> c_int {
    static int oaktrail_rfkill_set(void *data, bool blocked)
    {
    u8 value;
    u8 result;
    let mut radio: c_ulong = (unsigned long) data;
    ec_read(OT_EC_DEVICE_STATE_ADDRESS, &result);
    if (!blocked)
    value = (u8) (result | radio);
    else
    value = (u8) (result & ~radio);
    ec_write(OT_EC_DEVICE_STATE_ADDRESS, value);
    return 0;
    }
    static const struct rfkill_ops oaktrail_rfkill_ops = {
    .set_block = oaktrail_rfkill_set,
    };
    static struct rfkill *oaktrail_rfkill_new(char *name, enum rfkill_type type,
    unsigned long mask)
    {
    struct rfkill *rfkill_dev;
    u8 value;
    int err;
    rfkill_dev = rfkill_alloc(name, &oaktrail_device.dev, type,
    &oaktrail_rfkill_ops, (void *)mask);
    if (!rfkill_dev)
    return ERR_PTR(-ENOMEM);
    ec_read(OT_EC_DEVICE_STATE_ADDRESS, &value);
    rfkill_init_sw_state(rfkill_dev, (value & mask) != 1);
    err = rfkill_register(rfkill_dev);
    if (err) {
    rfkill_destroy(rfkill_dev);
    return ERR_PTR(err);
    }
    return rfkill_dev;
    }
#[no_mangle]
pub unsafe extern "C" fn __oaktrail_rfkill_cleanup(rf: *mut rfkill) {
    static inline void __oaktrail_rfkill_cleanup(struct rfkill *rf)
    {
    if (rf) {
    rfkill_unregister(rf);
    rfkill_destroy(rf);
    }
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_rfkill_cleanup() {
    static void oaktrail_rfkill_cleanup(void)
    {
    __oaktrail_rfkill_cleanup(wifi_rfkill);
    __oaktrail_rfkill_cleanup(bt_rfkill);
    __oaktrail_rfkill_cleanup(gps_rfkill);
    __oaktrail_rfkill_cleanup(wwan_rfkill);
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_rfkill_init() -> c_int {
    static int oaktrail_rfkill_init(void)
    {
    int ret;
    wifi_rfkill = oaktrail_rfkill_new("oaktrail-wifi",
    RFKILL_TYPE_WLAN,
    OT_EC_WIFI_MASK);
    if (IS_ERR(wifi_rfkill)) {
    ret = PTR_ERR(wifi_rfkill);
    wifi_rfkill = core::ptr::null_mut();
    goto cleanup;
    }
    bt_rfkill = oaktrail_rfkill_new("oaktrail-bluetooth",
    RFKILL_TYPE_BLUETOOTH,
    OT_EC_BT_MASK);
    if (IS_ERR(bt_rfkill)) {
    ret = PTR_ERR(bt_rfkill);
    bt_rfkill = core::ptr::null_mut();
    goto cleanup;
    }
    gps_rfkill = oaktrail_rfkill_new("oaktrail-gps",
    RFKILL_TYPE_GPS,
    OT_EC_GPS_MASK);
    if (IS_ERR(gps_rfkill)) {
    ret = PTR_ERR(gps_rfkill);
    gps_rfkill = core::ptr::null_mut();
    goto cleanup;
    }
    wwan_rfkill = oaktrail_rfkill_new("oaktrail-wwan",
    RFKILL_TYPE_WWAN,
    OT_EC_WWAN_MASK);
    if (IS_ERR(wwan_rfkill)) {
    ret = PTR_ERR(wwan_rfkill);
    wwan_rfkill = core::ptr::null_mut();
    goto cleanup;
    }
    return 0;
    cleanup:
    oaktrail_rfkill_cleanup();
    return ret;
    }
// backlight
#[no_mangle]
unsafe extern "C" fn get_backlight_brightness(b: *mut backlight_device) -> c_int {
    static int get_backlight_brightness(struct backlight_device *b)
    {
    u8 value;
    ec_read(OT_EC_BL_BRIGHTNESS_ADDRESS, &value);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn set_backlight_brightness(b: *mut backlight_device) -> c_int {
    static int set_backlight_brightness(struct backlight_device *b)
    {
    let mut percent: u8 = (u8) b.props.brightness;
    if (percent < 0 || percent > OT_EC_BL_BRIGHTNESS_MAX)
    return -EINVAL;
    ec_write(OT_EC_BL_BRIGHTNESS_ADDRESS, percent);
    ec_write(OT_EC_BL_CONTROL_ADDRESS, OT_EC_BL_CONTROL_ON_DATA);
    return 0;
    }
    static const struct backlight_ops oaktrail_bl_ops = {
    .get_brightness = get_backlight_brightness,
    .update_status	= set_backlight_brightness,
    };
#[no_mangle]
unsafe extern "C" fn oaktrail_backlight_init() -> c_int {
    static int oaktrail_backlight_init(void)
    {
    struct backlight_device *bd;
    struct backlight_properties props;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = OT_EC_BL_BRIGHTNESS_MAX;
    bd = backlight_device_register(DRIVER_NAME,
    &oaktrail_device.dev, core::ptr::null_mut(),
    &oaktrail_bl_ops,
    &props);
    if (IS_ERR(bd)) {
    oaktrail_bl_device = core::ptr::null_mut();
    pr_warn("Unable to register backlight device\n");
    return PTR_ERR(bd);
    }
    oaktrail_bl_device = bd;
    bd.props.brightness = get_backlight_brightness(bd);
    bd.props.power = BACKLIGHT_POWER_ON;
    backlight_update_status(bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_backlight_exit() {
    static void oaktrail_backlight_exit(void)
    {
    backlight_device_unregister(oaktrail_bl_device);
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_probe(pdev: *mut platform_device) -> c_int {
    static int oaktrail_probe(struct platform_device *pdev)
    {
    return 0;
    }
    static struct platform_driver oaktrail_driver = {
    .driver = {
    .name = DRIVER_NAME,
    },
    .probe	= oaktrail_probe,
    };
#[no_mangle]
unsafe extern "C" fn dmi_check_cb(id: *const dmi_system_id) -> c_int {
    static int dmi_check_cb(const struct dmi_system_id *id)
    {
    pr_info("Identified model '%s'\n", id.ident);
    return 0;
    }
    static const struct dmi_system_id oaktrail_dmi_table[] __initconst = {
    {
    .ident = "OakTrail platform",
    .matches = {
    DMI_MATCH(DMI_PRODUCT_NAME, "OakTrail platform"),
    },
    .callback = dmi_check_cb
    },
    { }
    };
    MODULE_DEVICE_TABLE(dmi, oaktrail_dmi_table);
#[no_mangle]
unsafe extern "C" fn oaktrail_init() -> int __init {
    static int __init oaktrail_init(void)
    {
    int ret;
    if (acpi_disabled) {
    pr_err("ACPI needs to be enabled for this driver to work!\n");
    return -ENODEV;
    }
    if (!force && !dmi_check_system(oaktrail_dmi_table)) {
    pr_err("Platform not recognized (You could try the module's force-parameter)");
    return -ENODEV;
    }
    ret = platform_driver_register(&oaktrail_driver);
    if (ret) {
    pr_warn("Unable to register platform driver\n");
    goto err_driver_reg;
    }
    oaktrail_device = platform_device_alloc(DRIVER_NAME, PLATFORM_DEVID_NONE);
    if (!oaktrail_device) {
    pr_warn("Unable to allocate platform device\n");
    ret = -ENOMEM;
    goto err_device_alloc;
    }
    ret = platform_device_add(oaktrail_device);
    if (ret) {
    pr_warn("Unable to add platform device\n");
    goto err_device_add;
    }
    if (acpi_video_get_backlight_type() == acpi_backlight_vendor) {
    ret = oaktrail_backlight_init();
    if (ret)
    goto err_backlight;
    }
    ret = oaktrail_rfkill_init();
    if (ret) {
    pr_warn("Setup rfkill failed\n");
    goto err_rfkill;
    }
    pr_info("Driver "DRIVER_VERSION" successfully loaded\n");
    return 0;
    err_rfkill:
    oaktrail_backlight_exit();
    err_backlight:
    platform_device_del(oaktrail_device);
    err_device_add:
    platform_device_put(oaktrail_device);
    err_device_alloc:
    platform_driver_unregister(&oaktrail_driver);
    err_driver_reg:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_cleanup() -> void __exit {
    static void __exit oaktrail_cleanup(void)
    {
    oaktrail_backlight_exit();
    oaktrail_rfkill_cleanup();
    platform_device_unregister(oaktrail_device);
    platform_driver_unregister(&oaktrail_driver);
    pr_info("Driver unloaded\n");
    }
    module_init(oaktrail_init);
    module_exit(oaktrail_cleanup);
    MODULE_AUTHOR("Yin Kangkai <kangkai.yin@intel.com>");
    MODULE_DESCRIPTION("Intel Oaktrail Platform ACPI Extras");
    MODULE_VERSION(DRIVER_VERSION);
    MODULE_LICENSE("GPL");
