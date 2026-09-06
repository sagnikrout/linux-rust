//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/atlas_btns.c
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
// atlas_btns.c - Atlas Wallmount Touchscreen ACPI Extras
//
// Copyright (C) 2006 Jaya Kumar
// Based on Toshiba ACPI by John Belmonte and ASUS ACPI
// This work was sponsored by CIS(M) Sdn Bhd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atlas_btns {
    pub input_dev: *mut input_dev,
    pub keymap: [c_ushort; 16],
}

// button handling code
    static acpi_status acpi_atlas_button_setup(acpi_handle region_handle,
    u32 function, void *handler_context,
    void **return_context)
    {
// return_context =
    (function != ACPI_REGION_DEACTIVATE) ? handler_context : core::ptr::null_mut();
    return AE_OK;
    }
    static acpi_status acpi_atlas_button_handler(u32 function,
    acpi_physical_address address,
    u32 bit_width, u64 *value,
    void *handler_context,
    void *region_context)
    {
    struct atlas_btns *atlas = region_context;
    if (function == ACPI_WRITE) {
    let mut code: c_int = address & 0x0f;
    let mut key_down: c_int = !(address & 0x10);
    input_event(atlas.input_dev, EV_MSC, MSC_SCAN, code);
    input_report_key(atlas.input_dev, atlas.keymap[code],
    key_down);
    input_sync(atlas.input_dev);
    return AE_OK;
    }
    dev_warn(atlas.input_dev.dev.parent,
    "unexpected function: function=%x,address=%lx,value=%x\n",
    function, (unsigned long)address, (u32)*value);
    return AE_BAD_PARAMETER;
    }
#[no_mangle]
unsafe extern "C" fn atlas_acpi_button_probe(pdev: *mut platform_device) -> c_int {
    static int atlas_acpi_button_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    struct atlas_btns *atlas;
    struct input_dev *input_dev;
    acpi_status status;
    int i;
    int err;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    atlas = devm_kzalloc(&pdev.dev, sizeof(*atlas), GFP_KERNEL);
    if (!atlas)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    atlas.input_dev = input_dev;
    input_dev.name = "Atlas ACPI button driver";
    input_dev.phys = "ASIM0000/atlas/input0";
    input_dev.id.bustype = BUS_HOST;
    input_dev.keycode = atlas.keymap;
    input_dev.keycodesize = sizeof(atlas.keymap[0]);
    input_dev.keycodemax = ARRAY_SIZE(atlas.keymap);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    for (i = 0; i < ARRAY_SIZE(atlas.keymap); i++) {
    if (i < 9) {
    atlas.keymap[i] = KEY_F1 + i;
    input_set_capability(input_dev, EV_KEY, KEY_F1 + i);
    } else {
    atlas.keymap[i] = KEY_RESERVED;
    }
    }
    err = input_register_device(input_dev);
    if (err)
    return dev_err_probe(&pdev.dev, err,
    "couldn't register input device\n");
// hookup button handler
    status = acpi_install_address_space_handler(device.handle,
    0x81,
    &acpi_atlas_button_handler,
    &acpi_atlas_button_setup,
    atlas);
    if (ACPI_FAILURE(status))
    return dev_err_probe(&pdev.dev, -EINVAL,
    "error installing addr spc handler\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atlas_acpi_button_remove(pdev: *mut platform_device) {
    static void atlas_acpi_button_remove(struct platform_device *pdev)
    {
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
    acpi_remove_address_space_handler(device.handle, 0x81,
    &acpi_atlas_button_handler);
    }
    static const struct acpi_device_id atlas_device_ids[] = {
    { "ASIM0000" },
    { "" }
    };
    MODULE_DEVICE_TABLE(acpi, atlas_device_ids);
    static struct platform_driver atlas_acpi_driver = {
    .probe = atlas_acpi_button_probe,
    .remove = atlas_acpi_button_remove,
    .driver = {
    .name = ACPI_ATLAS_NAME,
    .acpi_match_table = atlas_device_ids,
    },
    };
    module_platform_driver(atlas_acpi_driver);
    MODULE_AUTHOR("Jaya Kumar");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Atlas button driver");
