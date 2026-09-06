//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-aio.c
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
// WMI hotkeys support for Dell All-In-One series
//

    MODULE_DESCRIPTION("WMI hotkeys driver for Dell All-In-One series");
    MODULE_LICENSE("GPL");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dell_wmi_event {
    pub length: u16,
// 0x000: A hot key pressed or an event occurred
// 0x00F: A sequence of hot keys are pressed
    pub type: u16,
    pub event: [u16; ],
}

    static const char *dell_wmi_aio_guids[] = {
    EVENT_GUID1,
    EVENT_GUID2,
    core::ptr::null_mut()
    };
    MODULE_ALIAS("wmi:"EVENT_GUID1);
    MODULE_ALIAS("wmi:"EVENT_GUID2);
    static const struct key_entry dell_wmi_aio_keymap[] = {
    { KE_KEY, 0xc0, { KEY_VOLUMEUP } },
    { KE_KEY, 0xc1, { KEY_VOLUMEDOWN } },
    { KE_KEY, 0xe030, { KEY_VOLUMEUP } },
    { KE_KEY, 0xe02e, { KEY_VOLUMEDOWN } },
    { KE_KEY, 0xe020, { KEY_MUTE } },
    { KE_KEY, 0xe027, { KEY_DISPLAYTOGGLE } },
    { KE_KEY, 0xe006, { KEY_BRIGHTNESSUP } },
    { KE_KEY, 0xe005, { KEY_BRIGHTNESSDOWN } },
    { KE_KEY, 0xe00b, { KEY_SWITCHVIDEOMODE } },
    { KE_END, 0 }
    };
    static struct input_dev *dell_wmi_aio_input_dev;
//
// The new WMI event data format will follow the dell_wmi_event structure
// So, we will check if the buffer matches the format
//
#[no_mangle]
unsafe extern "C" fn dell_wmi_aio_event_check(buffer: *mut u8, length: c_int) -> bool {
    static bool dell_wmi_aio_event_check(u8 *buffer, int length)
    {
    struct dell_wmi_event *event = (struct dell_wmi_event *)buffer;
    if (event == core::ptr::null_mut() || length < 6)
    return false;
    if ((event.type == 0 || event.type == 0xf) &&
    event.length >= 2)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn dell_wmi_aio_notify(obj: *mut union acpi_object, context: *mut c_void) {
    static void dell_wmi_aio_notify(union acpi_object *obj, void *context)
    {
    struct dell_wmi_event *event;
    if (obj) {
    let mut scancode: c_uint = 0;
    switch (obj.type) {
    case ACPI_TYPE_INTEGER:
// Most All-In-One correctly return integer scancode
    scancode = obj.integer.value;
    sparse_keymap_report_event(dell_wmi_aio_input_dev,
    scancode, 1, true);
    break;
    case ACPI_TYPE_BUFFER:
    if (dell_wmi_aio_event_check(obj.buffer.pointer,
    obj.buffer.length)) {
    event = (struct dell_wmi_event *)
    obj.buffer.pointer;
    scancode = event.event[0];
    } else {
// Broken machines return the scancode in a
    buffer */
    if (obj.buffer.pointer &&
    obj.buffer.length > 0)
    scancode = obj.buffer.pointer[0];
    }
    if (scancode)
    sparse_keymap_report_event(
    dell_wmi_aio_input_dev,
    scancode, 1, true);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn dell_wmi_aio_input_setup() -> int __init {
    static int __init dell_wmi_aio_input_setup(void)
    {
    int err;
    dell_wmi_aio_input_dev = input_allocate_device();
    if (!dell_wmi_aio_input_dev)
    return -ENOMEM;
    dell_wmi_aio_input_dev.name = "Dell AIO WMI hotkeys";
    dell_wmi_aio_input_dev.phys = "wmi/input0";
    dell_wmi_aio_input_dev.id.bustype = BUS_HOST;
    err = sparse_keymap_setup(dell_wmi_aio_input_dev,
    dell_wmi_aio_keymap, core::ptr::null_mut());
    if (err) {
    pr_err("Unable to setup input device keymap\n");
    goto err_free_dev;
    }
    err = input_register_device(dell_wmi_aio_input_dev);
    if (err) {
    pr_info("Unable to register input device\n");
    goto err_free_dev;
    }
    return 0;
    err_free_dev:
    input_free_device(dell_wmi_aio_input_dev);
    return err;
    }
    static const char *dell_wmi_aio_find(void)
    {
    int i;
    for (i = 0; dell_wmi_aio_guids[i] != core::ptr::null_mut(); i++)
    if (wmi_has_guid(dell_wmi_aio_guids[i]))
    return dell_wmi_aio_guids[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dell_wmi_aio_init() -> int __init {
    static int __init dell_wmi_aio_init(void)
    {
    int err;
    const char *guid;
    guid = dell_wmi_aio_find();
    if (!guid) {
    pr_warn("No known WMI GUID found\n");
    return -ENXIO;
    }
    err = dell_wmi_aio_input_setup();
    if (err)
    return err;
    err = wmi_install_notify_handler(guid, dell_wmi_aio_notify, core::ptr::null_mut());
    if (err) {
    pr_err("Unable to register notify handler - %d\n", err);
    input_unregister_device(dell_wmi_aio_input_dev);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dell_wmi_aio_exit() -> void __exit {
    static void __exit dell_wmi_aio_exit(void)
    {
    const char *guid;
    guid = dell_wmi_aio_find();
    wmi_remove_notify_handler(guid);
    input_unregister_device(dell_wmi_aio_input_dev);
    }
    module_init(dell_wmi_aio_init);
    module_exit(dell_wmi_aio_exit);
