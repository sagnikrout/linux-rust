//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-zydacron.c
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
// HID driver for zydacron remote control
//
// Copyright (c) 2010 Don Prince <dhprince.devel@yahoo.co.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zc_device {
    pub input_ep81: *mut input_dev,
    pub last_key: [c_ushort; 4],
}

//
// Zydacron remote control has an invalid HID report descriptor,
// that needs fixing before we can parse it.
//
    static const __u8 *zc_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 253 &&
    rdesc[0x96] == 0xbc && rdesc[0x97] == 0xff &&
    rdesc[0xca] == 0xbc && rdesc[0xcb] == 0xff &&
    rdesc[0xe1] == 0xbc && rdesc[0xe2] == 0xff) {
    hid_info(hdev,
    "fixing up zydacron remote control report descriptor\n");
    rdesc[0x96] = rdesc[0xca] = rdesc[0xe1] = 0x0c;
    rdesc[0x97] = rdesc[0xcb] = rdesc[0xe2] = 0x00;
    }
    return rdesc;
    }

    hid_map_usage_clear(hi, usage, bit, max, EV_KEY, (c))
    static int zc_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    int i;
    struct zc_device *zc = hid_get_drvdata(hdev);
    zc.input_ep81 = hi.input;
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_CONSUMER)
    return 0;
    dbg_hid("zynacron input mapping event [0x%x]\n",
    usage.hid & HID_USAGE);
    switch (usage.hid & HID_USAGE) {
// report 2
    case 0x10:
    zc_map_key_clear(KEY_MODE);
    break;
    case 0x30:
    zc_map_key_clear(KEY_SCREEN);
    break;
    case 0x70:
    zc_map_key_clear(KEY_INFO);
    break;
// report 3
    case 0x04:
    zc_map_key_clear(KEY_RADIO);
    break;
// report 4
    case 0x0d:
    zc_map_key_clear(KEY_PVR);
    break;
    case 0x25:
    zc_map_key_clear(KEY_TV);
    break;
    case 0x47:
    zc_map_key_clear(KEY_AUDIO);
    break;
    case 0x49:
    zc_map_key_clear(KEY_AUX);
    break;
    case 0x4a:
    zc_map_key_clear(KEY_VIDEO);
    break;
    case 0x48:
    zc_map_key_clear(KEY_DVD);
    break;
    case 0x24:
    zc_map_key_clear(KEY_MENU);
    break;
    case 0x32:
    zc_map_key_clear(KEY_TEXT);
    break;
    default:
    return 0;
    }
    for (i = 0; i < 4; i++)
    zc.last_key[i] = 0;
    return 1;
    }
    static int zc_raw_event(struct hid_device *hdev, struct hid_report *report,
    u8 *data, int size)
    {
    struct zc_device *zc = hid_get_drvdata(hdev);
    let mut ret: c_int = 0;
    unsigned key;
    unsigned short index;
    if (report.id == data[0] && (hdev.claimed & HID_CLAIMED_INPUT)) {
// break keys
    for (index = 0; index < 4; index++) {
    key = zc.last_key[index];
    if (key) {
    input_event(zc.input_ep81, EV_KEY, key, 0);
    zc.last_key[index] = 0;
    }
    }
    key = 0;
    switch (report.id) {
    case 0x02:
    case 0x03:
    switch (data[1]) {
    case 0x10:
    key = KEY_MODE;
    index = 0;
    break;
    case 0x30:
    key = KEY_SCREEN;
    index = 1;
    break;
    case 0x70:
    key = KEY_INFO;
    index = 2;
    break;
    case 0x04:
    key = KEY_RADIO;
    index = 3;
    break;
    }
    if (key) {
    input_event(zc.input_ep81, EV_KEY, key, 1);
    zc.last_key[index] = key;
    }
    ret = 1;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zc_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int zc_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    int ret;
    struct zc_device *zc;
    zc = devm_kzalloc(&hdev.dev, sizeof(*zc), GFP_KERNEL);
    if (zc == core::ptr::null_mut()) {
    hid_err(hdev, "can't alloc descriptor\n");
    return -ENOMEM;
    }
    hid_set_drvdata(hdev, zc);
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    return ret;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    return ret;
    }
    return 0;
    }
    static const struct hid_device_id zc_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_ZYDACRON, USB_DEVICE_ID_ZYDACRON_REMOTE_CONTROL) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, zc_devices);
    static struct hid_driver zc_driver = {
    .name = "zydacron",
    .id_table = zc_devices,
    .report_fixup = zc_report_fixup,
    .input_mapping = zc_input_mapping,
    .raw_event = zc_raw_event,
    .probe = zc_probe,
    };
    module_hid_driver(zc_driver);
    MODULE_DESCRIPTION("HID driver for zydacron remote control");
    MODULE_LICENSE("GPL");
