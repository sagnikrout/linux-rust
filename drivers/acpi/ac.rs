//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/ac.c
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
// acpi_ac.c - ACPI AC Adapter Driver (Revision: 27)
//
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//

pub const ACPI_AC_NOTIFY_STATUS: c_uint = 0x80;
pub const ACPI_AC_STATUS_OFFLINE: c_uint = 0x00;
pub const ACPI_AC_STATUS_ONLINE: c_uint = 0x01;
pub const ACPI_AC_STATUS_UNKNOWN: c_uint = 0xFF;
    MODULE_AUTHOR("Paul Diefenbaugh");
    MODULE_DESCRIPTION("ACPI AC Adapter Driver");
    MODULE_LICENSE("GPL");
    static const struct acpi_device_id ac_device_ids[] = {
    {"ACPI0003", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, ac_device_ids);
    static int ac_sleep_before_get_state_ms;
    static int ac_only;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ac {
    pub charger: *mut power_supply,
    pub charger_desc: power_supply_desc,
    pub device: *mut acpi_device,
    pub state: c_ulonglong,
    pub battery_nb: notifier_block,
}

// AC Adapter Management
#[no_mangle]
unsafe extern "C" fn acpi_ac_get_state(ac: *mut acpi_ac) -> c_int {
    static int acpi_ac_get_state(struct acpi_ac *ac)
    {
    let mut status: acpi_status = AE_OK;
    if (!ac)
    return -EINVAL;
    if (ac_only) {
    ac.state = 1;
    return 0;
    }
    status = acpi_evaluate_integer(ac.device.handle, "_PSR", core::ptr::null_mut(),
    &ac.state);
    if (ACPI_FAILURE(status)) {
    acpi_handle_info(ac.device.handle,
    "Error reading AC Adapter state: %s\n",
    acpi_format_exception(status));
    ac.state = ACPI_AC_STATUS_UNKNOWN;
    return -ENODEV;
    }
    return 0;
    }
// sysfs I/F
    static int get_ac_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct acpi_ac *ac = to_acpi_ac(psy);
    if (!ac)
    return -ENODEV;
    if (acpi_ac_get_state(ac))
    return -ENODEV;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = ac.state;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property ac_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
// Driver Model
#[no_mangle]
unsafe extern "C" fn acpi_ac_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_ac_notify(acpi_handle handle, u32 event, void *data)
    {
    struct acpi_ac *ac = data;
    struct acpi_device *adev = ac.device;
    switch (event) {
    default:
    acpi_handle_debug(adev.handle, "Unsupported event [0x%x]\n",
    event);
    fallthrough;
    case ACPI_AC_NOTIFY_STATUS:
    case ACPI_NOTIFY_BUS_CHECK:
    case ACPI_NOTIFY_DEVICE_CHECK:
//
// A buggy BIOS may notify AC first and then sleep for
// a specific time before doing actual operations in the
// EC event handler (_Qxx). This will cause the AC state
// reported by the ACPI event to be incorrect, so wait for a
// specific time for the EC event handler to make progress.
//
    if (ac_sleep_before_get_state_ms > 0)
    msleep(ac_sleep_before_get_state_ms);
    acpi_ac_get_state(ac);
    acpi_bus_generate_netlink_event(ACPI_AC_CLASS,
    dev_name(&adev.dev), event,
    ac.state);
    acpi_notifier_call_chain(ACPI_AC_CLASS, acpi_device_bid(adev),
    event, ac.state);
    power_supply_changed(ac.charger);
    }
    }
    static int acpi_ac_battery_notify(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct acpi_ac *ac = container_of(nb, struct acpi_ac, battery_nb);
    struct acpi_bus_event *event = (struct acpi_bus_event *)data;
//
// On HP Pavilion dv6-6179er AC status notifications aren't triggered
// when adapter is plugged/unplugged. However, battery status
// notifications are triggered when battery starts charging or
// discharging. Re-reading AC status triggers lost AC notifications,
// if AC status has changed.
//
    if (strcmp(event.device_class, ACPI_BATTERY_CLASS) == 0 &&
    event.type == ACPI_BATTERY_NOTIFY_STATUS)
    acpi_ac_get_state(ac);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn thinkpad_e530_quirk(d: *const dmi_system_id) -> int __init {
    static int __init thinkpad_e530_quirk(const struct dmi_system_id *d)
    {
    ac_sleep_before_get_state_ms = 1000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ac_only_quirk(d: *const dmi_system_id) -> int __init {
    static int __init ac_only_quirk(const struct dmi_system_id *d)
    {
    ac_only = 1;
    return 0;
    }
// Please keep this list alphabetically sorted
    static const struct dmi_system_id ac_dmi_table[]  __initconst = {
    {
// Kodlix GK45 returning incorrect state
    .callback = ac_only_quirk,
    .matches = {
    DMI_MATCH(DMI_PRODUCT_NAME, "GK45"),
    },
    },
    {
// Lenovo Thinkpad e530, see comment in acpi_ac_notify()
    .callback = thinkpad_e530_quirk,
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "32597CG"),
    },
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn acpi_ac_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_ac_probe(struct platform_device *pdev)
    {
    let mut psy_cfg: power_supply_config = {};
    struct device *dev = &pdev.dev;
    struct acpi_device *adev;
    struct acpi_ac *ac;
    int result;
    adev = ACPI_COMPANION(&pdev.dev);
    if (!adev)
    return -ENODEV;
    ac = devm_kzalloc(dev, sizeof(*ac), GFP_KERNEL);
    if (!ac)
    return -ENOMEM;
    ac.device = adev;
    platform_set_drvdata(pdev, ac);
    result = acpi_ac_get_state(ac);
    if (result)
    return result;
    psy_cfg.drv_data = ac;
    ac.charger_desc.name = acpi_device_bid(adev);
    ac.charger_desc.type = POWER_SUPPLY_TYPE_MAINS;
    ac.charger_desc.properties = ac_props;
    ac.charger_desc.num_properties = ARRAY_SIZE(ac_props);
    ac.charger_desc.get_property = get_ac_property;
    ac.charger = devm_power_supply_register(dev, &ac.charger_desc, &psy_cfg);
    if (IS_ERR(ac.charger))
    return PTR_ERR(ac.charger);
    pr_info("AC Adapter [%s] (%s-line)\n", acpi_device_bid(adev),
    str_on_off(ac.state));
    result = devm_acpi_install_notify_handler(dev, ACPI_ALL_NOTIFY,
    acpi_ac_notify, ac);
    if (result)
    return result;
    ac.battery_nb.notifier_call = acpi_ac_battery_notify;
    register_acpi_notifier(&ac.battery_nb);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn acpi_ac_resume(dev: *mut device) -> c_int {
    static int acpi_ac_resume(struct device *dev)
    {
    struct acpi_ac *ac = dev_get_drvdata(dev);
    unsigned int old_state;
    old_state = ac.state;
    if (acpi_ac_get_state(ac))
    return 0;
    if (old_state != ac.state)
    power_supply_changed(ac.charger);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(acpi_ac_pm, core::ptr::null_mut(), acpi_ac_resume);
#[no_mangle]
unsafe extern "C" fn acpi_ac_remove(pdev: *mut platform_device) {
    static void acpi_ac_remove(struct platform_device *pdev)
    {
    struct acpi_ac *ac = platform_get_drvdata(pdev);
    unregister_acpi_notifier(&ac.battery_nb);
    }
    static struct platform_driver acpi_ac_driver = {
    .probe = acpi_ac_probe,
    .remove = acpi_ac_remove,
    .driver = {
    .name = "ac",
    .acpi_match_table = ac_device_ids,
    .pm = &acpi_ac_pm,
    },
    };
#[no_mangle]
unsafe extern "C" fn acpi_ac_init() -> int __init {
    static int __init acpi_ac_init(void)
    {
    int result;
    if (acpi_disabled)
    return -ENODEV;
    if (acpi_quirk_skip_acpi_ac_and_battery())
    return -ENODEV;
    dmi_check_system(ac_dmi_table);
    result = platform_driver_register(&acpi_ac_driver);
    if (result < 0)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_ac_exit() -> void __exit {
    static void __exit acpi_ac_exit(void)
    {
    platform_driver_unregister(&acpi_ac_driver);
    }
    module_init(acpi_ac_init);
    module_exit(acpi_ac_exit);
