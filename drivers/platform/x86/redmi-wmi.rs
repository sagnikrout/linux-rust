//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/redmi-wmi.c
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


// SPDX-License-Identifier: GPL-2.0
// WMI driver for Xiaomi Redmibooks

    static const struct key_entry redmi_wmi_keymap[] = {
    {KE_KEY, 0x00000201,	{KEY_SELECTIVE_SCREENSHOT}},
    {KE_KEY, 0x00000301,	{KEY_ALL_APPLICATIONS}},
    {KE_KEY, 0x00001b01,	{KEY_CONFIG}},
    {KE_KEY, 0x00011b01,	{KEY_CONFIG}},
    {KE_KEY, 0x00010101,	{KEY_SWITCHVIDEOMODE}},
    {KE_KEY, 0x00001a01,	{KEY_REFRESH_RATE_TOGGLE}},
// AI button has code for each position
    {KE_KEY, 0x00011801,	{KEY_ASSISTANT}},
    {KE_KEY, 0x00011901,	{KEY_ASSISTANT}},
// Keyboard backlight: Off / Auto / Low / High (new state in byte 2)
    {KE_KEY, 0x00000501,	{KEY_KBDILLUMTOGGLE}},
    {KE_KEY, 0x00800501,	{KEY_KBDILLUMTOGGLE}},
    {KE_KEY, 0x00050501,	{KEY_KBDILLUMTOGGLE}},
    {KE_KEY, 0x000a0501,	{KEY_KBDILLUMTOGGLE}},
// Xiaomi G Command Center
    {KE_KEY, 0x00010a01,	{KEY_VENDOR}},
// OEM preset power mode: 1=Balanced 2=Silent 3=Turbo 4=Full speed
    {KE_KEY, 0x00011601,	{KEY_PERFORMANCE}},
    {KE_KEY, 0x00021601,	{KEY_PERFORMANCE}},
    {KE_KEY, 0x00031601,	{KEY_PERFORMANCE}},
    {KE_KEY, 0x00041601,	{KEY_PERFORMANCE}},
// Fn Lock state: 1=on 0=off
    {KE_KEY, 0x00000701,	{KEY_FN_ESC}},
    {KE_KEY, 0x00010701,	{KEY_FN_ESC}},
// Fn+`/1/2/3/4
    {KE_KEY, 0x00011101, {KEY_F13}},
    {KE_KEY, 0x00011201, {KEY_F14}},
    {KE_KEY, 0x00011301, {KEY_F15}},
    {KE_KEY, 0x00011401, {KEY_F16}},
    {KE_KEY, 0x00011501, {KEY_F17}},
    {KE_END}
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redmi_wmi {
    pub input_dev: *mut input_dev,
// Protects the key event sequence
    pub key_lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn redmi_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int redmi_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct redmi_wmi *data;
    int err;
// Init dev
    data = devm_kzalloc(&wdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    dev_set_drvdata(&wdev.dev, data);
    err = devm_mutex_init(&wdev.dev, &data.key_lock);
    if (err)
    return err;
    data.input_dev = devm_input_allocate_device(&wdev.dev);
    if (!data.input_dev)
    return -ENOMEM;
    data.input_dev.name = "Redmibook WMI keys";
    data.input_dev.phys = "wmi/input0";
    err = sparse_keymap_setup(data.input_dev, redmi_wmi_keymap, core::ptr::null_mut());
    if (err)
    return err;
    return input_register_device(data.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn redmi_wmi_notify(wdev: *mut wmi_device, obj: *mut union acpi_object) {
    static void redmi_wmi_notify(struct wmi_device *wdev, union acpi_object *obj)
    {
    struct key_entry *entry;
    struct redmi_wmi *data = dev_get_drvdata(&wdev.dev);
    let mut autorelease: bool = true;
    u32 payload;
    let mut value: c_int = 1;
    if (obj.type != ACPI_TYPE_BUFFER) {
    dev_err(&wdev.dev, "Bad response type %u\n", obj.type);
    return;
    }
    if (obj.buffer.length < 32) {
    dev_err(&wdev.dev, "Invalid buffer length %u\n", obj.buffer.length);
    return;
    }
    payload = get_unaligned_le32(obj.buffer.pointer);
    entry = sparse_keymap_entry_from_scancode(data.input_dev, payload);
    if (!entry) {
    dev_dbg(&wdev.dev, "Unknown WMI event with payload %u", payload);
    return;
    }
// AI key quirk
    if (entry.keycode == KEY_ASSISTANT) {
    value = !(payload & AI_KEY_VALUE_MASK);
    autorelease = false;
    }
    guard(mutex)(&data.key_lock);
    sparse_keymap_report_entry(data.input_dev, entry, value, autorelease);
    }
    static const struct wmi_device_id redmi_wmi_id_table[] = {
    { WMI_REDMIBOOK_KEYBOARD_EVENT_GUID, core::ptr::null_mut() },
    { }
    };
    static struct wmi_driver redmi_wmi_driver = {
    .driver = {
    .name = "redmi-wmi",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table = redmi_wmi_id_table,
    .min_event_size = 32,
    .probe = redmi_wmi_probe,
    .notify = redmi_wmi_notify,
    .no_singleton = true,
    };
    module_wmi_driver(redmi_wmi_driver);
    MODULE_DEVICE_TABLE(wmi, redmi_wmi_id_table);
    MODULE_AUTHOR("Gladyshev Ilya <foxido@foxido.dev>");
    MODULE_DESCRIPTION("Redmibook WMI driver");
    MODULE_LICENSE("GPL");
