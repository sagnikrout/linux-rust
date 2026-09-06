//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-steelseries-arctis.c
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
// HID driver for Steelseries arctis headsets
//
// Copyright (c) 2023 Bastien Nocera
// Copyright (c) 2026 Sriman Achanta
//

    struct steelseries_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct steelseries_device_info {
    pub capabilities: c_ulong,
    pub sync_interface: u8,
    pub async_interface: u8,
    pub hdev): *mut *mut int (request_status)(struct hid_device,
    pub size): *mut *mut *mut *mut void (parse_status)(struct steelseries_device sd, u8 data, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct steelseries_device {
    pub refcnt: kref,
    pub hdev: *mut hid_device,
    pub info: *const steelseries_device_info,
    pub status_work: delayed_work,
    pub battery_desc: power_supply_desc,
    pub battery: *mut power_supply,
    pub headset_connected: bool,
    pub battery_capacity: u8,
    pub battery_charging: bool,
    pub lock: spinlock_t,
    pub removed: bool,
}

#[no_mangle]
unsafe extern "C" fn steelseries_device_release(ref: *mut kref) {
    static void steelseries_device_release(struct kref *ref)
    {
    struct steelseries_device *sd =
    container_of(ref, struct steelseries_device, refcnt);
    kfree(sd);
    }
//
// Headset report helpers
//
    static int steelseries_send_report(struct hid_device *hdev, const u8 *data,
    int len, enum hid_report_type type)
    {
    u8 *buf;
    int ret;
    buf = kmemdup(data, len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    ret = hid_hw_raw_request(hdev, data[0], buf, len, type,
    HID_REQ_SET_REPORT);
    kfree(buf);
    if (ret < 0)
    return ret;
    if (ret < len)
    return -EIO;
    return 0;
    }
    static inline int steelseries_send_output_report(struct hid_device *hdev,
    const u8 *data, int len)
    {
    return steelseries_send_report(hdev, data, len, HID_OUTPUT_REPORT);
    }
//
// Headset status request functions
//
#[no_mangle]
unsafe extern "C" fn steelseries_arctis_1_request_status(hdev: *mut hid_device) -> c_int {
    static int steelseries_arctis_1_request_status(struct hid_device *hdev)
    {
    const u8 data[] = { 0x06, 0x12 };
    return steelseries_send_output_report(hdev, data, sizeof(data));
    }
#[no_mangle]
unsafe extern "C" fn steelseries_arctis_9_request_status(hdev: *mut hid_device) -> c_int {
    static int steelseries_arctis_9_request_status(struct hid_device *hdev)
    {
    const u8 data[] = { 0x00, 0x20 };
    return steelseries_send_output_report(hdev, data, sizeof(data));
    }
#[no_mangle]
unsafe extern "C" fn steelseries_arctis_nova_request_status(hdev: *mut hid_device) -> c_int {
    static int steelseries_arctis_nova_request_status(struct hid_device *hdev)
    {
    const u8 data[] = { 0x00, 0xb0 };
    return steelseries_send_output_report(hdev, data, sizeof(data));
    }
//
// Headset battery helpers
//
#[no_mangle]
unsafe extern "C" fn battery_capacity_to_level(capacity: c_int) -> c_int {
    static int battery_capacity_to_level(int capacity)
    {
    if (capacity >= 50)
    return POWER_SUPPLY_CAPACITY_LEVEL_NORMAL;
    if (capacity >= 20)
    return POWER_SUPPLY_CAPACITY_LEVEL_LOW;
    return POWER_SUPPLY_CAPACITY_LEVEL_CRITICAL;
    }
#[no_mangle]
unsafe extern "C" fn steelseries_map_capacity(capacity: u8, min_in: u8, max_in: u8) -> u8 {
    static u8 steelseries_map_capacity(u8 capacity, u8 min_in, u8 max_in)
    {
    if (capacity >= max_in)
    return 100;
    if (capacity <= min_in)
    return 0;
    return (capacity - min_in) * 100 / (max_in - min_in);
    }
//
// Headset status parse functions
//
    static void steelseries_arctis_1_parse_status(struct steelseries_device *sd,
    u8 *data, int size)
    {
// Only the battery status report echoes the request header.
    if (size < 8 || data[0] != 0x06 || data[1] != 0x12)
    return;
    sd.headset_connected = (data[2] != 0x01);
    sd.battery_capacity = data[3];
    }
    static void steelseries_arctis_9_parse_status(struct steelseries_device *sd,
    u8 *data, int size)
    {
    if (size < 5)
    return;
    if (data[0] == 0xaa && data[1] == 0x01) {
    sd.headset_connected = true;
    sd.battery_charging = (data[4] == 0x01);
    sd.battery_capacity = steelseries_map_capacity(data[3], 0x64, 0x9a);
    } else {
// Device off: 0x55 (no status) or 0x03 (stale status).
    sd.headset_connected = false;
    sd.battery_charging = false;
    }
    }
    static void steelseries_arctis_nova_parse_status(struct steelseries_device *sd,
    u8 *data, int size)
    {
    if (size < 2)
    return;
    switch (data[0]) {
    case 0xb0:
    if (size < 4)
    return;
    sd.headset_connected = (data[1] == 0x03);
    sd.battery_capacity = data[2];
    sd.battery_charging = (data[3] == 0x01);
    break;
    case 0xb7:
    sd.battery_capacity = data[1];
    break;
    case 0xb9:
    sd.headset_connected = (data[1] == 0x03);
    break;
    case 0xbb:
    sd.battery_charging = (data[1] == 0x01);
    break;
    }
    }
    static void steelseries_arctis_nova_7_parse_status(struct steelseries_device *sd,
    u8 *data, int size)
    {
    if (size < 2)
    return;
    switch (data[0]) {
    case 0xb0:
    if (size < 4)
    return;
    sd.headset_connected = (data[1] == 0x03);
    sd.battery_capacity = steelseries_map_capacity(data[2], 0, 4);
    sd.battery_charging = (data[3] == 0x01);
    break;
    case 0xb7:
    sd.battery_capacity = steelseries_map_capacity(data[1], 0, 4);
    break;
    case 0xb9:
    sd.headset_connected = (data[1] == 0x03);
    break;
    case 0xbb:
    sd.battery_charging = (data[1] == 0x01);
    break;
    }
    }
//
// Device info definitions
//
    static const struct steelseries_device_info arctis_1_info = {
    .sync_interface = 3,
    .capabilities = SS_CAP_BATTERY,
    .request_status = steelseries_arctis_1_request_status,
    .parse_status = steelseries_arctis_1_parse_status,
    };
    static const struct steelseries_device_info arctis_9_info = {
    .sync_interface = 0,
    .capabilities = SS_CAP_BATTERY,
    .request_status = steelseries_arctis_9_request_status,
    .parse_status = steelseries_arctis_9_parse_status,
    };
    static const struct steelseries_device_info arctis_nova_info = {
    .sync_interface = 3,
    .async_interface = 5,
    .capabilities = SS_CAP_BATTERY,
    .request_status = steelseries_arctis_nova_request_status,
    .parse_status = steelseries_arctis_nova_parse_status,
    };
    static const struct steelseries_device_info arctis_nova_7_info = {
    .sync_interface = 3,
    .async_interface = 5,
    .capabilities = SS_CAP_BATTERY,
    .request_status = steelseries_arctis_nova_request_status,
    .parse_status = steelseries_arctis_nova_7_parse_status,
    };
//
// Headset wireless status and battery infrastructure
//
pub const STEELSERIES_HEADSET_STATUS_TIMEOUT_MS: c_int = 3000;
    static void
    steelseries_headset_set_wireless_status(struct hid_device *hdev,
    bool connected)
    {
    struct usb_interface *intf;
    if (!hid_is_usb(hdev))
    return;
    intf = to_usb_interface(hdev.dev.parent);
    usb_set_wireless_status(intf, connected ?
    USB_WIRELESS_STATUS_CONNECTED :
    USB_WIRELESS_STATUS_DISCONNECTED);
    }

    static int steelseries_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct steelseries_device *sd = power_supply_get_drvdata(psy);
    size_t prefix_len;
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = sd.hdev.name;
    while ((prefix_len = str_has_prefix(val.strval, STEELSERIES_PREFIX)))
    val.strval += prefix_len;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = "SteelSeries";
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = 1;
    break;
    case POWER_SUPPLY_PROP_STATUS:
    if (!sd.headset_connected)
    val.intval = POWER_SUPPLY_STATUS_UNKNOWN;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sd->battery_charging) -> else {
    else if (sd.battery_charging)
    val.intval = sd.battery_capacity >= 100 ?
    POWER_SUPPLY_STATUS_FULL :
    POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case POWER_SUPPLY_PROP_SCOPE:
    val.intval = POWER_SUPPLY_SCOPE_DEVICE;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    val.intval = sd.battery_capacity;
    break;
    case POWER_SUPPLY_PROP_CAPACITY_LEVEL:
    val.intval = battery_capacity_to_level(sd.battery_capacity);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static enum power_supply_property steelseries_battery_props[] = {
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_SCOPE,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CAPACITY_LEVEL,
    };
//
// Delayed work handlers for status polling
//
#[no_mangle]
unsafe extern "C" fn steelseries_status_timer_work_handler(work: *mut work_struct) {
    static void steelseries_status_timer_work_handler(struct work_struct *work)
    {
    struct steelseries_device *sd = container_of(
    work, struct steelseries_device, status_work.work);
    unsigned long flags;
    sd.info.request_status(sd.hdev);
    spin_lock_irqsave(&sd.lock, flags);
// Async devices push status events themselves; only poll once.
    if (!sd.removed && !sd.info.async_interface)
    schedule_delayed_work(&sd.status_work,
    msecs_to_jiffies(STEELSERIES_HEADSET_STATUS_TIMEOUT_MS));
    spin_unlock_irqrestore(&sd.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn steelseries_battery_register(sd: *mut steelseries_device) -> c_int {
    static int steelseries_battery_register(struct steelseries_device *sd)
    {
    let mut battery_cfg: power_supply_config = { .drv_data = sd, };
    struct power_supply *battery;
    int ret;
    sd.battery_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    sd.battery_desc.properties = steelseries_battery_props;
    sd.battery_desc.num_properties = ARRAY_SIZE(steelseries_battery_props);
    sd.battery_desc.get_property = steelseries_battery_get_property;
    sd.battery_desc.use_for_apm = 0;
    sd.battery_desc.name = devm_kasprintf(&sd.hdev.dev, GFP_KERNEL,
    "steelseries_headset_battery_%s",
    sd.hdev.uniq[0] ? sd.hdev.uniq :
    dev_name(&sd.hdev.dev));
    if (!sd.battery_desc.name)
    return -ENOMEM;
// avoid the warning of 0% battery while waiting for the first info
    sd.battery_capacity = 100;
    sd.battery_charging = false;
    sd.headset_connected = false;
    steelseries_headset_set_wireless_status(sd.hdev, false);
    battery = power_supply_register(&sd.hdev.dev,
    &sd.battery_desc, &battery_cfg);
    if (IS_ERR(battery)) {
    ret = PTR_ERR(battery);
    hid_err(sd.hdev,
    "%s:power_supply_register failed with error %d\n",
    __func__, ret);
    return ret;
    }
    power_supply_powers(battery, &sd.hdev.dev);
// Assign on success only, so a concurrent raw_event never sees an ERR_PTR.
    sd.battery = battery;
    return 0;
    }
    static struct hid_driver steelseries_arctis_driver;
    static struct steelseries_device *
    steelseries_get_sibling_sd(struct hid_device *hdev, int interface_num)
    {
    struct usb_interface *intf = to_usb_interface(hdev.dev.parent);
    struct usb_device *usb_dev = interface_to_usbdev(intf);
    struct usb_interface *sibling_intf;
    struct hid_device *sibling_hdev;
    struct steelseries_device *sd = core::ptr::null_mut();
    sibling_intf = usb_ifnum_to_if(usb_dev, interface_num);
    if (!sibling_intf)
    return core::ptr::null_mut();
//
// usb_get_intfdata() only yields a hid_device when usbhid is bound;
// gate on the descriptor class so a non-HID sibling (e.g. a crafted
// device exposing storage or audio here) is never treated as one.
//
    if (sibling_intf.cur_altsetting.desc.bInterfaceClass != USB_INTERFACE_CLASS_HID)
    return core::ptr::null_mut();
//
// Take the sibling's device lock across the intfdata read and the
// kref_get so a concurrent unbind cannot free the hid_device underneath
// us; usbhid leaves intfdata dangling on disconnect, so dev.driver is
// the reliable "still bound" test under this lock. Use device_trylock()
// to stay off the lockdep chain of the interface being probed and let
// the caller retry via -EPROBE_DEFER if the sibling is momentarily busy.
//
    if (!device_trylock(&sibling_intf.dev))
    return core::ptr::null_mut();
    if (sibling_intf.dev.driver) {
    sibling_hdev = usb_get_intfdata(sibling_intf);
    if (sibling_hdev &&
    sibling_hdev.driver == &steelseries_arctis_driver) {
    sd = hid_get_drvdata(sibling_hdev);
    if (sd)
    kref_get(&sd.refcnt);
    }
    }
    device_unlock(&sibling_intf.dev);
    return sd;
    }
    static int steelseries_arctis_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    const struct steelseries_device_info *info =
    (const struct steelseries_device_info *)id.driver_data;
    struct steelseries_device *sd;
    struct usb_interface *intf;
    u8 interface_num;
    int ret;
    if (hid_is_usb(hdev)) {
    intf = to_usb_interface(hdev.dev.parent);
    interface_num = intf.cur_altsetting.desc.bInterfaceNumber;
    } else {
    return -ENODEV;
    }
    ret = hid_parse(hdev);
    if (ret)
    return ret;
// Let hid-generic handle non-vendor or unknown interfaces
    if (interface_num != info.sync_interface &&
    (!info.async_interface || interface_num != info.async_interface))
    return hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (interface_num == info.sync_interface) {
    sd = kzalloc_obj(*sd);
    if (!sd)
    return -ENOMEM;
    kref_init(&sd.refcnt);
    sd.hdev = hdev;
    sd.info = info;
    spin_lock_init(&sd.lock);
    INIT_DELAYED_WORK(&sd.status_work, steelseries_status_timer_work_handler);
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret)
    goto err_free;
    ret = hid_hw_open(hdev);
    if (ret)
    goto err_stop;
    if (info.capabilities & SS_CAP_BATTERY) {
    ret = steelseries_battery_register(sd);
    if (ret < 0)
    hid_warn(hdev, "Failed to register battery: %d\n", ret);
    }
//
// Publish drvdata only once fully initialised: the async sibling
// attaches by reading it, so it must never observe a half-built or
// failed instance. A failed probe never gets here, so the error
// path below has nothing to unpublish.
//
    hid_set_drvdata(hdev, sd);
    schedule_delayed_work(&sd.status_work, msecs_to_jiffies(100));
    return 0;
    }
//
// The async interface shares the steelseries_device created by the
// sync interface. Defer until the sync interface has probed and
// published its drvdata.
//
    if (info.async_interface && interface_num == info.async_interface) {
    sd = steelseries_get_sibling_sd(hdev, info.sync_interface);
    if (!sd)
    return -EPROBE_DEFER;
    hid_set_drvdata(hdev, sd);
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    kref_put(&sd.refcnt, steelseries_device_release);
    return ret;
    }
    ret = hid_hw_open(hdev);
    if (ret) {
    hid_hw_stop(hdev);
    kref_put(&sd.refcnt, steelseries_device_release);
    return ret;
    }
    return 0;
    }
    return -ENODEV;
    err_stop:
    hid_hw_stop(hdev);
    err_free:
// drvdata is unpublished until full success, so no sibling can hold sd.
    kref_put(&sd.refcnt, steelseries_device_release);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn steelseries_arctis_remove(hdev: *mut hid_device) {
    static void steelseries_arctis_remove(struct hid_device *hdev)
    {
    struct steelseries_device *sd;
    struct power_supply *battery;
    unsigned long flags;
    struct usb_interface *intf;
    u8 interface_num;
    if (hid_is_usb(hdev)) {
    intf = to_usb_interface(hdev.dev.parent);
    interface_num = intf.cur_altsetting.desc.bInterfaceNumber;
    } else {
    return;
    }
    sd = hid_get_drvdata(hdev);
    if (!sd) {
    hid_hw_stop(hdev);
    return;
    }
    if (interface_num == sd.info.sync_interface) {
    spin_lock_irqsave(&sd.lock, flags);
    sd.removed = true;
    battery = sd.battery;
    sd.battery = core::ptr::null_mut();
    spin_unlock_irqrestore(&sd.lock, flags);
    cancel_delayed_work_sync(&sd.status_work);
    if (battery)
    power_supply_unregister(battery);
    }
    hid_hw_close(hdev);
    hid_hw_stop(hdev);
    kref_put(&sd.refcnt, steelseries_device_release);
    }
    static int steelseries_arctis_raw_event(struct hid_device *hdev,
    struct hid_report *report, u8 *data, int size)
    {
    struct steelseries_device *sd = hid_get_drvdata(hdev);
    u8 old_capacity;
    bool old_connected;
    bool old_charging;
    bool is_async_interface;
    unsigned long flags;
    if (!sd)
    return 0;
    is_async_interface = (hdev != sd.hdev);
    spin_lock_irqsave(&sd.lock, flags);
    if (sd.removed) {
    spin_unlock_irqrestore(&sd.lock, flags);
    return 0;
    }
    old_capacity = sd.battery_capacity;
    old_connected = sd.headset_connected;
    old_charging = sd.battery_charging;
    sd.info.parse_status(sd, data, size);
    if (sd.headset_connected != old_connected) {
    hid_dbg(hdev,
    "Connected status changed from %sconnected to %sconnected\n",
    old_connected ? "" : "not ",
    sd.headset_connected ? "" : "not ");
    if (sd.headset_connected && !old_connected &&
    sd.info.async_interface && is_async_interface)
    schedule_delayed_work(&sd.status_work, 0);
    if (sd.battery) {
    steelseries_headset_set_wireless_status(sd.hdev,
    sd.headset_connected);
    power_supply_changed(sd.battery);
    }
    }
    if (sd.battery_capacity != old_capacity) {
    hid_dbg(hdev, "Battery capacity changed from %d%% to %d%%\n",
    old_capacity, sd.battery_capacity);
    if (sd.battery)
    power_supply_changed(sd.battery);
    }
    if (sd.battery_charging != old_charging) {
    hid_dbg(hdev,
    "Battery charging status changed from %scharging to %scharging\n",
    old_charging ? "" : "not ",
    sd.battery_charging ? "" : "not ");
    if (sd.battery)
    power_supply_changed(sd.battery);
    }
    spin_unlock_irqrestore(&sd.lock, flags);
    return 0;
    }
    static const struct hid_device_id steelseries_arctis_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_1_X),
    .driver_data = (unsigned long)&arctis_1_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_9),
    .driver_data = (unsigned long)&arctis_9_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_5_X),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7),
    .driver_data = (unsigned long)&arctis_nova_7_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X),
    .driver_data = (unsigned long)&arctis_nova_7_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X_2),
    .driver_data = (unsigned long)&arctis_nova_7_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_DIABLO),
    .driver_data = (unsigned long)&arctis_nova_7_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_WOW),
    .driver_data = (unsigned long)&arctis_nova_7_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_2026),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_P_2026),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X_2026),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_DIABLO_2026),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_GEN2),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X_GEN2),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X_GEN2_2),
    .driver_data = (unsigned long)&arctis_nova_info },
    { HID_USB_DEVICE(USB_VENDOR_ID_STEELSERIES,
    USB_DEVICE_ID_STEELSERIES_ARCTIS_NOVA_7_X_GEN2_3),
    .driver_data = (unsigned long)&arctis_nova_info },
    {}
    };
    MODULE_DEVICE_TABLE(hid, steelseries_arctis_devices);
    static struct hid_driver steelseries_arctis_driver = {
    .name = "hid-steelseries-arctis",
    .id_table = steelseries_arctis_devices,
    .probe = steelseries_arctis_probe,
    .remove = steelseries_arctis_remove,
    .raw_event = steelseries_arctis_raw_event,
    };
    module_hid_driver(steelseries_arctis_driver);
    MODULE_DESCRIPTION("HID driver for Steelseries arctis headsets");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Mayer <git@mayer-bgk.de>");
    MODULE_AUTHOR("Bastien Nocera <hadess@hadess.net>");
    MODULE_AUTHOR("Sriman Achanta <srimanachanta@gmail.com>");
