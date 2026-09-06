//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/msi-wmi.c
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
// MSI WMI hotkeys
//
// Copyright (C) 2009 Novell <trenn@suse.de>
//
// Most stuff taken over from hp-wmi
//

    MODULE_AUTHOR("Thomas Renninger <trenn@suse.de>");
    MODULE_DESCRIPTION("MSI laptop WMI hotkeys driver");
    MODULE_LICENSE("GPL");

    MODULE_ALIAS("wmi:" MSIWMI_BIOS_GUID);
    MODULE_ALIAS("wmi:" MSIWMI_MSI_EVENT_GUID);
    MODULE_ALIAS("wmi:" MSIWMI_WIND_EVENT_GUID);
    enum msi_scancodes {
// Generic MSI keys (not present on MSI Wind)
    MSI_KEY_BRIGHTNESSUP	= 0xD0,
    MSI_KEY_BRIGHTNESSDOWN,
    MSI_KEY_VOLUMEUP,
    MSI_KEY_VOLUMEDOWN,
    MSI_KEY_MUTE,
// MSI Wind keys
    WIND_KEY_TOUCHPAD	= 0x08,	/* Fn+F3 touchpad toggle */
    WIND_KEY_BLUETOOTH	= 0x56,	/* Fn+F11 Bluetooth toggle */
    WIND_KEY_CAMERA,		/* Fn+F6 webcam toggle */
    WIND_KEY_WLAN		= 0x5f,	/* Fn+F11 Wi-Fi toggle */
    WIND_KEY_TURBO,			/* Fn+F10 turbo mode toggle */
    WIND_KEY_ECO		= 0x69,	/* Fn+F10 ECO mode toggle */
// MSI Claw keys
    CLAW_KEY_VOLUMEDOWN	= 0x21,
    CLAW_KEY_CENTER		= 0x29,	/* MSI M-Center main menu */
    CLAW_KEY_QUICK_LONG	= 0x2a,	/* MSI M-Center quick access long hold */
    CLAW_KEY_VOLUMEUP	= 0x32,
    CLAW_KEY_QUICK_SHORT	= 0x58,	/* MSI M-Center quick access short press */
    };
    static struct key_entry msi_wmi_keymap[] = {
    { KE_KEY, MSI_KEY_BRIGHTNESSUP,		{KEY_BRIGHTNESSUP} },
    { KE_KEY, MSI_KEY_BRIGHTNESSDOWN,	{KEY_BRIGHTNESSDOWN} },
    { KE_KEY, MSI_KEY_VOLUMEUP,		{KEY_VOLUMEUP} },
    { KE_KEY, MSI_KEY_VOLUMEDOWN,		{KEY_VOLUMEDOWN} },
    { KE_KEY, MSI_KEY_MUTE,			{KEY_MUTE} },
// These keys work without WMI. Ignore them to avoid double keycodes
    { KE_IGNORE, WIND_KEY_TOUCHPAD,		{KEY_TOUCHPAD_TOGGLE} },
    { KE_IGNORE, WIND_KEY_BLUETOOTH,	{KEY_BLUETOOTH} },
    { KE_IGNORE, WIND_KEY_CAMERA,		{KEY_CAMERA} },
    { KE_IGNORE, WIND_KEY_WLAN,		{KEY_WLAN} },
// These are unknown WMI events found on MSI Wind
    { KE_IGNORE, 0x00 },
    { KE_IGNORE, 0x62 },
    { KE_IGNORE, 0x63 },
// These are MSI Wind keys that should be handled via WMI
    { KE_KEY, WIND_KEY_TURBO,		{KEY_PROG1} },
    { KE_KEY, WIND_KEY_ECO,			{KEY_PROG2} },
// These are MSI Claw keys, used for MSI M-Center in Windows
    { KE_KEY, CLAW_KEY_CENTER,		{KEY_F15} },
// These MSI Claw keys work without WMI. Ignore them to avoid double keycodes
    { KE_IGNORE, CLAW_KEY_QUICK_SHORT },
    { KE_IGNORE, CLAW_KEY_QUICK_LONG },
    { KE_IGNORE, CLAW_KEY_VOLUMEUP },
    { KE_IGNORE, CLAW_KEY_VOLUMEDOWN },
    { KE_END, 0 }
    };
    static ktime_t last_pressed;
    static const struct {
    const char *guid;
    bool quirk_last_pressed;
    } *event_wmi, event_wmis[] = {
    { MSIWMI_MSI_EVENT_GUID, true },
    { MSIWMI_WIND_EVENT_GUID, false },
    };
    static struct backlight_device *backlight;
    static int backlight_map[] = { 0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF };
    static struct input_dev *msi_wmi_input_dev;
#[no_mangle]
unsafe extern "C" fn msi_wmi_query_block(instance: c_int, ret: *mut c_int) -> c_int {
    static int msi_wmi_query_block(int instance, int *ret)
    {
    acpi_status status;
    union acpi_object *obj;
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    status = wmi_query_block(MSIWMI_BIOS_GUID, instance, &output);
    if (ACPI_FAILURE(status))
    return -EIO;
    obj = output.pointer;
    if (!obj || obj.type != ACPI_TYPE_INTEGER) {
    if (obj) {
    pr_err("query block returned object "
    "type: %d - buffer length:%d\n", obj.type,
    obj.type == ACPI_TYPE_BUFFER ?
    obj.buffer.length : 0);
    }
    kfree(obj);
    return -EINVAL;
    }
// ret = obj->integer.value;
    kfree(obj);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msi_wmi_set_block(instance: c_int, value: c_int) -> c_int {
    static int msi_wmi_set_block(int instance, int value)
    {
    acpi_status status;
    let mut input: acpi_buffer = { sizeof(int), &value };
    pr_debug("Going to set block of instance: %d - value: %d\n",
    instance, value);
    status = wmi_set_block(MSIWMI_BIOS_GUID, instance, &input);
    return ACPI_SUCCESS(status) ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn bl_get(bd: *mut backlight_device) -> c_int {
    static int bl_get(struct backlight_device *bd)
    {
    int level, err, ret;
// Instance 1 is "get backlight", cmp with DSDT
    err = msi_wmi_query_block(1, &ret);
    if (err) {
    pr_err("Could not query backlight: %d\n", err);
    return -EINVAL;
    }
    pr_debug("Get: Query block returned: %d\n", ret);
    for (level = 0; level < ARRAY_SIZE(backlight_map); level++) {
    if (backlight_map[level] == ret) {
    pr_debug("Current backlight level: 0x%X - index: %d\n",
    backlight_map[level], level);
    break;
    }
    }
    if (level == ARRAY_SIZE(backlight_map)) {
    pr_err("get: Invalid brightness value: 0x%X\n", ret);
    return -EINVAL;
    }
    return level;
    }
#[no_mangle]
unsafe extern "C" fn bl_set_status(bd: *mut backlight_device) -> c_int {
    static int bl_set_status(struct backlight_device *bd)
    {
    let mut bright: c_int = bd.props.brightness;
    if (bright >= ARRAY_SIZE(backlight_map) || bright < 0)
    return -EINVAL;
// Instance 0 is "set backlight"
    return msi_wmi_set_block(0, backlight_map[bright]);
    }
    static const struct backlight_ops msi_backlight_ops = {
    .get_brightness	= bl_get,
    .update_status	= bl_set_status,
    };
#[no_mangle]
unsafe extern "C" fn msi_wmi_notify(obj: *mut union acpi_object, context: *mut c_void) {
    static void msi_wmi_notify(union acpi_object *obj, void *context)
    {
    struct key_entry *key = core::ptr::null_mut();
    let mut eventcode: c_int = 0;
    if (!obj)
    return;
    switch (obj.type) {
    case ACPI_TYPE_INTEGER:
    eventcode = obj.integer.value;
    pr_debug("Eventcode: 0x%x\n", eventcode);
    break;
    case ACPI_TYPE_BUFFER:
// Field returns u8[2] here, but is u32 by spec. Allow "oversized" buffers.
    if (obj.buffer.length < 2)
    return;
// pointer[0] is key ID, pointer[1] is active state. We don't get release
// events, so ignore the active state and treat as autorelease.
//
    eventcode = obj.buffer.pointer[0];
    pr_debug("Eventcode: 0x%x\n", eventcode);
    break;
    default:
    pr_info("Unknown event received\n");
    return;
    }
    key = sparse_keymap_entry_from_scancode(msi_wmi_input_dev, eventcode);
    if (!key) {
    pr_info("Unknown key pressed - 0x%x\n", eventcode);
    return;
    }
    if (event_wmi.quirk_last_pressed) {
    let mut cur: ktime_t = ktime_get_real();
    let mut diff: ktime_t = ktime_sub(cur, last_pressed);
// Ignore event if any event happened in a 50 ms
// timeframe -> Key press may result in 10-20 GPEs
//
    if (ktime_to_us(diff) < 1000 * 50) {
    pr_debug("Suppressed key event 0x%X - Last press was %lld us ago\n",
    key.code, ktime_to_us(diff));
    return;
    }
    last_pressed = cur;
    }
// Brightness is served via acpi video driver
    if (key.type == KE_KEY &&
    (backlight || (key.code == MSI_KEY_BRIGHTNESSUP ||
    key.code == MSI_KEY_BRIGHTNESSDOWN)))
    return;
    pr_debug("Send key: 0x%X - Input layer keycode: %d\n", key.code, key.keycode);
    sparse_keymap_report_entry(msi_wmi_input_dev, key, 1, true);
    }
#[no_mangle]
unsafe extern "C" fn msi_wmi_backlight_setup() -> int __init {
    static int __init msi_wmi_backlight_setup(void)
    {
    int err;
    struct backlight_properties props;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = ARRAY_SIZE(backlight_map) - 1;
    backlight = backlight_device_register(DRV_NAME, core::ptr::null_mut(), core::ptr::null_mut(),
    &msi_backlight_ops,
    &props);
    if (IS_ERR(backlight))
    return PTR_ERR(backlight);
    err = bl_get(core::ptr::null_mut());
    if (err < 0) {
    backlight_device_unregister(backlight);
    return err;
    }
    backlight.props.brightness = err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msi_wmi_input_setup() -> int __init {
    static int __init msi_wmi_input_setup(void)
    {
    int err;
    msi_wmi_input_dev = input_allocate_device();
    if (!msi_wmi_input_dev)
    return -ENOMEM;
    msi_wmi_input_dev.name = "MSI WMI hotkeys";
    msi_wmi_input_dev.phys = "wmi/input0";
    msi_wmi_input_dev.id.bustype = BUS_HOST;
    err = sparse_keymap_setup(msi_wmi_input_dev, msi_wmi_keymap, core::ptr::null_mut());
    if (err)
    goto err_free_dev;
    err = input_register_device(msi_wmi_input_dev);
    if (err)
    goto err_free_dev;
    last_pressed = 0;
    return 0;
    err_free_dev:
    input_free_device(msi_wmi_input_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn msi_wmi_init() -> int __init {
    static int __init msi_wmi_init(void)
    {
    int err;
    int i;
    for (i = 0; i < ARRAY_SIZE(event_wmis); i++) {
    if (!wmi_has_guid(event_wmis[i].guid))
    continue;
    err = msi_wmi_input_setup();
    if (err) {
    pr_err("Unable to setup input device\n");
    return err;
    }
    err = wmi_install_notify_handler(event_wmis[i].guid,
    msi_wmi_notify, core::ptr::null_mut());
    if (ACPI_FAILURE(err)) {
    pr_err("Unable to setup WMI notify handler\n");
    goto err_free_input;
    }
    pr_debug("Event handler installed\n");
    event_wmi = &event_wmis[i];
    break;
    }
    if (wmi_has_guid(MSIWMI_BIOS_GUID) &&
    acpi_video_get_backlight_type() == acpi_backlight_vendor) {
    err = msi_wmi_backlight_setup();
    if (err) {
    pr_err("Unable to setup backlight device\n");
    goto err_uninstall_handler;
    }
    pr_debug("Backlight device created\n");
    }
    if (!event_wmi && !backlight) {
    pr_err("This machine doesn't have neither MSI-hotkeys nor backlight through WMI\n");
    return -ENODEV;
    }
    return 0;
    err_uninstall_handler:
    if (event_wmi)
    wmi_remove_notify_handler(event_wmi.guid);
    err_free_input:
    if (event_wmi)
    input_unregister_device(msi_wmi_input_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn msi_wmi_exit() -> void __exit {
    static void __exit msi_wmi_exit(void)
    {
    if (event_wmi) {
    wmi_remove_notify_handler(event_wmi.guid);
    input_unregister_device(msi_wmi_input_dev);
    }
    backlight_device_unregister(backlight);
    }
    module_init(msi_wmi_init);
    module_exit(msi_wmi_exit);
