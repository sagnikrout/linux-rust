//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amilo-rfkill.c
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
// Support for rfkill on some Fujitsu-Siemens Amilo laptops.
// Copyright 2011 Ben Hutchings.
//
// Based in part on the fsam7440 driver, which is:
// Copyright 2005 Alejandro Vidal Mata & Javier Vidal Mata.
// and on the fsaa1655g driver, which is:
// Copyright 2006 Martin Večeřa.
//

//
// These values were obtained from disassembling and debugging the
// PM.exe program installed in the Fujitsu-Siemens AMILO A1655G
//
pub const A1655_WIFI_COMMAND: c_uint = 0x10C5;
pub const A1655_WIFI_ON: c_uint = 0x25;
pub const A1655_WIFI_OFF: c_uint = 0x45;
#[no_mangle]
unsafe extern "C" fn amilo_a1655_rfkill_set_block(data: *mut c_void, blocked: bool) -> c_int {
    static int amilo_a1655_rfkill_set_block(void *data, bool blocked)
    {
    let mut param: u8 = blocked ? A1655_WIFI_OFF : A1655_WIFI_ON;
    int rc;
    i8042_lock_chip();
    rc = i8042_command(&param, A1655_WIFI_COMMAND);
    i8042_unlock_chip();
    return rc;
    }
    static const struct rfkill_ops amilo_a1655_rfkill_ops = {
    .set_block = amilo_a1655_rfkill_set_block
    };
//
// These values were obtained from disassembling the PM.exe program
// installed in the Fujitsu-Siemens AMILO M 7440
//
pub const M7440_PORT1: c_uint = 0x118f;
pub const M7440_PORT2: c_uint = 0x118e;
pub const M7440_RADIO_ON1: c_uint = 0x12;
pub const M7440_RADIO_ON2: c_uint = 0x80;
pub const M7440_RADIO_OFF1: c_uint = 0x10;
pub const M7440_RADIO_OFF2: c_uint = 0x00;
#[no_mangle]
unsafe extern "C" fn amilo_m7440_rfkill_set_block(data: *mut c_void, blocked: bool) -> c_int {
    static int amilo_m7440_rfkill_set_block(void *data, bool blocked)
    {
    let mut val1: u8 = blocked ? M7440_RADIO_OFF1 : M7440_RADIO_ON1;
    let mut val2: u8 = blocked ? M7440_RADIO_OFF2 : M7440_RADIO_ON2;
    outb(val1, M7440_PORT1);
    outb(val2, M7440_PORT2);
// Check whether the state has changed correctly
    if (inb(M7440_PORT1) != val1 || inb(M7440_PORT2) != val2)
    return -EIO;
    return 0;
    }
    static const struct rfkill_ops amilo_m7440_rfkill_ops = {
    .set_block = amilo_m7440_rfkill_set_block
    };
    static const struct dmi_system_id amilo_rfkill_id_table[] = {
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_BOARD_NAME, "AMILO A1655"),
    },
    .driver_data = (void *)&amilo_a1655_rfkill_ops
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_BOARD_NAME, "AMILO L1310"),
    },
    .driver_data = (void *)&amilo_a1655_rfkill_ops
    },
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_BOARD_NAME, "AMILO M7440"),
    },
    .driver_data = (void *)&amilo_m7440_rfkill_ops
    },
    {}
    };
    static struct platform_device *amilo_rfkill_pdev;
    static struct rfkill *amilo_rfkill_dev;
#[no_mangle]
unsafe extern "C" fn amilo_rfkill_probe(device: *mut platform_device) -> c_int {
    static int amilo_rfkill_probe(struct platform_device *device)
    {
    int rc;
    const struct dmi_system_id *system_id =
    dmi_first_match(amilo_rfkill_id_table);
    if (!system_id)
    return -ENXIO;
    amilo_rfkill_dev = rfkill_alloc(KBUILD_MODNAME, &device.dev,
    RFKILL_TYPE_WLAN,
    system_id.driver_data, core::ptr::null_mut());
    if (!amilo_rfkill_dev)
    return -ENOMEM;
    rc = rfkill_register(amilo_rfkill_dev);
    if (rc)
    goto fail;
    return 0;
    fail:
    rfkill_destroy(amilo_rfkill_dev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn amilo_rfkill_remove(device: *mut platform_device) {
    static void amilo_rfkill_remove(struct platform_device *device)
    {
    rfkill_unregister(amilo_rfkill_dev);
    rfkill_destroy(amilo_rfkill_dev);
    }
    static struct platform_driver amilo_rfkill_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    },
    .probe = amilo_rfkill_probe,
    .remove = amilo_rfkill_remove,
    };
#[no_mangle]
unsafe extern "C" fn amilo_rfkill_init() -> int __init {
    static int __init amilo_rfkill_init(void)
    {
    int rc;
    if (dmi_first_match(amilo_rfkill_id_table) == core::ptr::null_mut())
    return -ENODEV;
    rc = platform_driver_register(&amilo_rfkill_driver);
    if (rc)
    return rc;
    amilo_rfkill_pdev = platform_device_register_simple(KBUILD_MODNAME,
    PLATFORM_DEVID_NONE,
    core::ptr::null_mut(), 0);
    if (IS_ERR(amilo_rfkill_pdev)) {
    rc = PTR_ERR(amilo_rfkill_pdev);
    goto fail;
    }
    return 0;
    fail:
    platform_driver_unregister(&amilo_rfkill_driver);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn amilo_rfkill_exit() -> void __exit {
    static void __exit amilo_rfkill_exit(void)
    {
    platform_device_unregister(amilo_rfkill_pdev);
    platform_driver_unregister(&amilo_rfkill_driver);
    }
    MODULE_AUTHOR("Ben Hutchings <ben@decadent.org.uk>");
    MODULE_DESCRIPTION("Fujitsu-Siemens Amilo rfkill support");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(dmi, amilo_rfkill_id_table);
    module_init(amilo_rfkill_init);
    module_exit(amilo_rfkill_exit);
