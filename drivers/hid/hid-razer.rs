//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-razer.c
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
// HID driver for gaming keys on Razer Blackwidow gaming keyboards
// Macro Key Keycodes: M1 = 191, M2 = 192, M3 = 193, M4 = 194, M5 = 195
//
// Copyright (c) 2021 Jelle van der Waa <jvanderwaa@redhat.com>
//

pub const RAZER_BLACKWIDOW_TRANSFER_BUF_SIZE: c_int = 91;
    let mut macro_key_remapping: static bool = 1;
    module_param(macro_key_remapping, bool, 0644);
    MODULE_PARM_DESC(macro_key_remapping, " on (Y) off (N)");
    static unsigned char blackwidow_init[RAZER_BLACKWIDOW_TRANSFER_BUF_SIZE] = {
    0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04,
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x04, 0x00
    };
    static int razer_input_mapping(struct hid_device *hdev,
    struct hid_input *hi, struct hid_field *field,
    struct hid_usage *usage, unsigned long **bit, int *max)
    {
    if (!macro_key_remapping)
    return 0;
    if ((usage.hid & HID_UP_KEYBOARD) != HID_UP_KEYBOARD)
    return 0;
    switch (usage.hid & ~HID_UP_KEYBOARD) {
    case 0x68:
    map_key_clear(KEY_MACRO1);
    return 1;
    case 0x69:
    map_key_clear(KEY_MACRO2);
    return 1;
    case 0x6a:
    map_key_clear(KEY_MACRO3);
    return 1;
    case 0x6b:
    map_key_clear(KEY_MACRO4);
    return 1;
    case 0x6c:
    map_key_clear(KEY_MACRO5);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn razer_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int razer_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    char *buf;
    let mut ret: c_int = 0;
    ret = hid_parse(hdev);
    if (ret)
    return ret;
//
// Only send the enable macro keys command for the third device
// identified as mouse input.
//
    if (hdev.type == HID_TYPE_USBMOUSE) {
    buf = kmemdup(blackwidow_init, RAZER_BLACKWIDOW_TRANSFER_BUF_SIZE, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    ret = hid_hw_raw_request(hdev, 0, buf, RAZER_BLACKWIDOW_TRANSFER_BUF_SIZE,
    HID_FEATURE_REPORT, HID_REQ_SET_REPORT);
    if (ret != RAZER_BLACKWIDOW_TRANSFER_BUF_SIZE)
    hid_err(hdev, "failed to enable macro keys: %d\n", ret);
    kfree(buf);
    }
    return hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    }
    static const struct hid_device_id razer_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_RAZER,
    USB_DEVICE_ID_RAZER_BLACKWIDOW) },
    { HID_USB_DEVICE(USB_VENDOR_ID_RAZER,
    USB_DEVICE_ID_RAZER_BLACKWIDOW_CLASSIC) },
    { HID_USB_DEVICE(USB_VENDOR_ID_RAZER,
    USB_DEVICE_ID_RAZER_BLACKWIDOW_ULTIMATE) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, razer_devices);
    static struct hid_driver razer_driver = {
    .name = "razer",
    .id_table = razer_devices,
    .input_mapping = razer_input_mapping,
    .probe = razer_probe,
    };
    module_hid_driver(razer_driver);
    MODULE_AUTHOR("Jelle van der Waa <jvanderwaa@redhat.com>");
    MODULE_DESCRIPTION("HID driver for gaming keys on Razer Blackwidow gaming keyboards");
    MODULE_LICENSE("GPL");
