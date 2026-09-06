//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-plantronics.c
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
// Plantronics USB HID Driver
//
// Copyright (c) 2014 JD Cole <jd.cole@plantronics.com>
// Copyright (c) 2015-2018 Terry Junge <terry.junge@plantronics.com>
//

pub const PLT_HID_1_0_PAGE: c_uint = 0xffa00000;
pub const PLT_HID_2_0_PAGE: c_uint = 0xffa20000;
pub const PLT_BASIC_TELEPHONY: c_uint = 0x0003;
pub const PLT_BASIC_EXCEPTION: c_uint = 0x0005;
pub const PLT_VOL_UP: c_uint = 0x00b1;
pub const PLT_VOL_DOWN: c_uint = 0x00b2;
pub const PLT_MIC_MUTE: c_uint = 0x00b5;

pub const PLT_DA60: c_uint = 0xda60;
pub const PLT_BT300_MIN: c_uint = 0x0413;
pub const PLT_BT300_MAX: c_uint = 0x0418;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plt_drv_data {
    pub device_type: c_ulong,
    pub last_key_ts: c_ulong,
    pub double_key_to: c_ulong,
    pub last_key: __u16,
}

    static int plantronics_input_mapping(struct hid_device *hdev,
    struct hid_input *hi,
    struct hid_field *field,
    struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    unsigned short mapped_key;
    struct plt_drv_data *drv_data = hid_get_drvdata(hdev);
    let mut plt_type: c_ulong = drv_data.device_type;
    let mut allow_mute: c_int = usage.hid == HID_TELEPHONY_MUTE;
    int allow_consumer = field.application == HID_CP_CONSUMERCONTROL &&
    (usage.hid & HID_USAGE_PAGE) == HID_UP_CONSUMER &&
    usage.hid != HID_CONSUMER_MUTE;
// special case for PTT products
    if (field.application == HID_GD_JOYSTICK)
    goto defaulted;
// non-standard types or multi-HID interfaces - plt_type is PID
    if (!(plt_type & HID_USAGE_PAGE)) {
    switch (plt_type) {
    case PLT_DA60:
    if (allow_consumer)
    goto defaulted;
    if (usage.hid == HID_CONSUMER_MUTE) {
    mapped_key = KEY_MICMUTE;
    goto mapped;
    }
    break;
    default:
    if (allow_consumer || allow_mute)
    goto defaulted;
    }
    goto ignored;
    }
// handle standard consumer control mapping
// and standard telephony mic mute mapping
    if (allow_consumer || allow_mute)
    goto defaulted;
// handle vendor unique types - plt_type is 0xffa0uuuu or 0xffa2uuuu
// if not 'basic telephony compliant' - map vendor unique controls
    if (!((plt_type & HID_USAGE) >= PLT_BASIC_TELEPHONY &&
    (plt_type & HID_USAGE) != PLT_BASIC_EXCEPTION) &&
    !((field.application ^ plt_type) & HID_USAGE_PAGE))
    switch (usage.hid) {
    case PLT1_VOL_UP:
    case PLT2_VOL_UP:
    mapped_key = KEY_VOLUMEUP;
    goto mapped;
    case PLT1_VOL_DOWN:
    case PLT2_VOL_DOWN:
    mapped_key = KEY_VOLUMEDOWN;
    goto mapped;
    case PLT1_MIC_MUTE:
    case PLT2_MIC_MUTE:
    mapped_key = KEY_MICMUTE;
    goto mapped;
    }
//
// Future mapping of call control or other usages,
// if and when keys are defined would go here
// otherwise, ignore everything else that was not mapped
//
    ignored:
    hid_dbg(hdev, "usage: %08x (appl: %08x) - ignored\n",
    usage.hid, field.application);
    return -1;
    defaulted:
    hid_dbg(hdev, "usage: %08x (appl: %08x) - defaulted\n",
    usage.hid, field.application);
    return 0;
    mapped:
    hid_map_usage_clear(hi, usage, bit, max, EV_KEY, mapped_key);
    hid_dbg(hdev, "usage: %08x (appl: %08x) - mapped to key %d\n",
    usage.hid, field.application, mapped_key);
    return 1;
    }
    static int plantronics_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
    struct plt_drv_data *drv_data = hid_get_drvdata(hdev);
    unsigned long prev_tsto, cur_ts;
    __u16 prev_key, cur_key;
// Usages are filtered in plantronics_usages.
// HZ too low for ms resolution - double key detection disabled
// or it is a key release - handle key presses only.
    if (!drv_data.double_key_to || !value)
    return 0;
    prev_tsto = drv_data.last_key_ts + drv_data.double_key_to;
    cur_ts = drv_data.last_key_ts = jiffies;
    prev_key = drv_data.last_key;
    cur_key = drv_data.last_key = usage.code;
// If the same key occurs in <= double_key_to -- ignore it
    if (prev_key == cur_key && time_before_eq(cur_ts, prev_tsto)) {
    hid_dbg(hdev, "double key %d ignored\n", cur_key);
    return 1; /* Ignore the repeated key. */
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plantronics_device_type(hdev: *mut hid_device) -> c_ulong {
    static unsigned long plantronics_device_type(struct hid_device *hdev)
    {
    unsigned i, col_page;
    let mut plt_type: c_ulong = hdev.product;
// multi-HID interfaces? - plt_type is PID
    if (plt_type >= PLT_BT300_MIN && plt_type <= PLT_BT300_MAX)
    goto exit;
// determine primary vendor page
    for (i = 0; i < hdev.maxcollection; i++) {
    col_page = hdev.collection[i].usage & HID_USAGE_PAGE;
    if (col_page == PLT_HID_2_0_PAGE) {
    plt_type = hdev.collection[i].usage;
    break;
    }
    if (col_page == PLT_HID_1_0_PAGE)
    plt_type = hdev.collection[i].usage;
    }
    exit:
    hid_dbg(hdev, "plt_type decoded as: %08lx\n", plt_type);
    return plt_type;
    }
    static int plantronics_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    struct plt_drv_data *drv_data;
    int ret;
    drv_data = devm_kzalloc(&hdev.dev, sizeof(*drv_data), GFP_KERNEL);
    if (!drv_data)
    return -ENOMEM;
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    return ret;
    }
    drv_data.device_type = plantronics_device_type(hdev);
    drv_data.double_key_to = msecs_to_jiffies(PLT_DOUBLE_KEY_TIMEOUT);
    drv_data.last_key_ts = jiffies - drv_data.double_key_to;
// if HZ does not allow ms resolution - disable double key detection
    if (drv_data.double_key_to < PLT_DOUBLE_KEY_TIMEOUT)
    drv_data.double_key_to = 0;
    hid_set_drvdata(hdev, drv_data);
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT |
    HID_CONNECT_HIDINPUT_FORCE | HID_CONNECT_HIDDEV_FORCE);
    if (ret)
    hid_err(hdev, "hw start failed\n");
    return ret;
    }
    static const struct hid_device_id plantronics_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_PLANTRONICS, HID_ANY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, plantronics_devices);
    static const struct hid_usage_id plantronics_usages[] = {
    { HID_CP_VOLUMEUP, EV_KEY, HID_ANY_ID },
    { HID_CP_VOLUMEDOWN, EV_KEY, HID_ANY_ID },
    { HID_TELEPHONY_MUTE, EV_KEY, HID_ANY_ID },
    { HID_CONSUMER_MUTE, EV_KEY, HID_ANY_ID },
    { PLT2_VOL_UP, EV_KEY, HID_ANY_ID },
    { PLT2_VOL_DOWN, EV_KEY, HID_ANY_ID },
    { PLT2_MIC_MUTE, EV_KEY, HID_ANY_ID },
    { PLT1_VOL_UP, EV_KEY, HID_ANY_ID },
    { PLT1_VOL_DOWN, EV_KEY, HID_ANY_ID },
    { PLT1_MIC_MUTE, EV_KEY, HID_ANY_ID },
    { HID_TERMINATOR, HID_TERMINATOR, HID_TERMINATOR }
    };
    static struct hid_driver plantronics_driver = {
    .name = "plantronics",
    .id_table = plantronics_devices,
    .usage_table = plantronics_usages,
    .input_mapping = plantronics_input_mapping,
    .event = plantronics_event,
    .probe = plantronics_probe,
    };
    module_hid_driver(plantronics_driver);
    MODULE_AUTHOR("JD Cole <jd.cole@plantronics.com>");
    MODULE_AUTHOR("Terry Junge <terry.junge@plantronics.com>");
    MODULE_DESCRIPTION("Plantronics USB HID Driver");
    MODULE_LICENSE("GPL");
