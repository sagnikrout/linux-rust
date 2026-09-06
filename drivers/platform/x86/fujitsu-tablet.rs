//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/fujitsu-tablet.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2006-2012 Robert Gerlach <khnz@gmx.de>
// Copyright (C) 2005-2006 Jan Rychter <jan@rychter.com>
//

pub const INVERT_TABLET_MODE_BIT: c_uint = 0x01;
pub const INVERT_DOCK_STATE_BIT: c_uint = 0x02;
pub const FORCE_TABLET_MODE_IF_UNDOCK: c_uint = 0x04;
pub const KEYMAP_LEN: c_int = 16;
    static const struct acpi_device_id fujitsu_ids[] = {
    { .id = "FUJ02BD" },
    { .id = "FUJ02BF" },
    { .id = "" }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fujitsu_config {
    pub keymap: [c_ushort; KEYMAP_LEN],
    pub quirks: c_uint,
}

    static unsigned short keymap_Lifebook_Tseries[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_SCROLLDOWN,
    KEY_SCROLLUP,
    KEY_ROTATE_DISPLAY,
    KEY_LEFTCTRL,
    KEY_BRIGHTNESSUP,
    KEY_BRIGHTNESSDOWN,
    KEY_BRIGHTNESS_ZERO,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_LEFTALT
    };
    static unsigned short keymap_Lifebook_T901[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_SCROLLDOWN,
    KEY_SCROLLUP,
    KEY_CYCLEWINDOWS,
    KEY_LEFTCTRL,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_LEFTMETA
    };
    static unsigned short keymap_Lifebook_T902[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_VOLUMEDOWN,
    KEY_VOLUMEUP,
    KEY_CYCLEWINDOWS,
    KEY_PROG1,
    KEY_PROG2,
    KEY_LEFTMETA,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    };
    static unsigned short keymap_Lifebook_U810[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_PROG1,
    KEY_PROG2,
    KEY_ROTATE_DISPLAY,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_UP,
    KEY_DOWN,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_LEFTCTRL,
    KEY_LEFTALT
    };
    static unsigned short keymap_Stylistic_Tseries[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_PRINT,
    KEY_BACKSPACE,
    KEY_SPACE,
    KEY_ENTER,
    KEY_BRIGHTNESSUP,
    KEY_BRIGHTNESSDOWN,
    KEY_DOWN,
    KEY_UP,
    KEY_SCROLLUP,
    KEY_SCROLLDOWN,
    KEY_LEFTCTRL,
    KEY_LEFTALT
    };
    static unsigned short keymap_Stylistic_ST5xxx[KEYMAP_LEN] __initdata = {
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_RESERVED,
    KEY_MAIL,
    KEY_ROTATE_DISPLAY,
    KEY_ESC,
    KEY_ENTER,
    KEY_BRIGHTNESSUP,
    KEY_BRIGHTNESSDOWN,
    KEY_DOWN,
    KEY_UP,
    KEY_SCROLLUP,
    KEY_SCROLLDOWN,
    KEY_LEFTCTRL,
    KEY_LEFTALT
    };
    static struct {
    struct input_dev *idev;
    struct fujitsu_config config;
    unsigned long prev_keymask;
    char name[17];
    char phys[21];
    int irq;
    int io_base;
    int io_length;
    } fujitsu;
#[no_mangle]
unsafe extern "C" fn fujitsu_ack() -> u8 {
    static u8 fujitsu_ack(void)
    {
    return inb(fujitsu.io_base + 2);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_status() -> u8 {
    static u8 fujitsu_status(void)
    {
    return inb(fujitsu.io_base + 6);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_read_register(addr: u8) -> u8 {
    static u8 fujitsu_read_register(const u8 addr)
    {
    outb(addr, fujitsu.io_base);
    return inb(fujitsu.io_base + 4);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_send_state() {
    static void fujitsu_send_state(void)
    {
    int state;
    int dock, tablet_mode;
    state = fujitsu_read_register(0xdd);
    dock = state & 0x02;
    if (fujitsu.config.quirks & INVERT_DOCK_STATE_BIT)
    dock = !dock;
    if ((fujitsu.config.quirks & FORCE_TABLET_MODE_IF_UNDOCK) && (!dock)) {
    tablet_mode = 1;
    } else{
    tablet_mode = state & 0x01;
    if (fujitsu.config.quirks & INVERT_TABLET_MODE_BIT)
    tablet_mode = !tablet_mode;
    }
    input_report_switch(fujitsu.idev, SW_DOCK, dock);
    input_report_switch(fujitsu.idev, SW_TABLET_MODE, tablet_mode);
    input_sync(fujitsu.idev);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_reset() {
    static void fujitsu_reset(void)
    {
    let mut timeout: c_int = 50;
    fujitsu_ack();
    while ((fujitsu_status() & 0x02) && (--timeout))
    msleep(20);
    fujitsu_send_state();
    }
    static int input_fujitsu_setup(struct device *parent, const char *name,
    const char *phys)
    {
    struct input_dev *idev;
    int error;
    int i;
    idev = input_allocate_device();
    if (!idev)
    return -ENOMEM;
    idev.dev.parent = parent;
    idev.phys = phys;
    idev.name = name;
    idev.id.bustype = BUS_HOST;
    idev.id.vendor  = 0x1734;	/* Fujitsu Siemens Computer GmbH */
    idev.id.product = 0x0001;
    idev.id.version = 0x0101;
    idev.keycode = fujitsu.config.keymap;
    idev.keycodesize = sizeof(fujitsu.config.keymap[0]);
    idev.keycodemax = ARRAY_SIZE(fujitsu.config.keymap);
    __set_bit(EV_REP, idev.evbit);
    for (i = 0; i < ARRAY_SIZE(fujitsu.config.keymap); i++)
    if (fujitsu.config.keymap[i])
    input_set_capability(idev, EV_KEY, fujitsu.config.keymap[i]);
    input_set_capability(idev, EV_MSC, MSC_SCAN);
    input_set_capability(idev, EV_SW, SW_DOCK);
    input_set_capability(idev, EV_SW, SW_TABLET_MODE);
    error = input_register_device(idev);
    if (error) {
    input_free_device(idev);
    return error;
    }
    fujitsu.idev = idev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn input_fujitsu_remove() {
    static void input_fujitsu_remove(void)
    {
    input_unregister_device(fujitsu.idev);
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t fujitsu_interrupt(int irq, void *dev_id)
    {
    unsigned long keymask, changed;
    unsigned int keycode;
    int pressed;
    int i;
    if (unlikely(!(fujitsu_status() & 0x01)))
    return IRQ_NONE;
    fujitsu_send_state();
    keymask  = fujitsu_read_register(0xde);
    keymask |= fujitsu_read_register(0xdf) << 8;
    keymask ^= 0xffff;
    changed = keymask ^ fujitsu.prev_keymask;
    if (changed) {
    fujitsu.prev_keymask = keymask;
    for_each_set_bit(i, &changed, KEYMAP_LEN) {
    keycode = fujitsu.config.keymap[i];
    pressed = keymask & changed & BIT(i);
    if (pressed)
    input_event(fujitsu.idev, EV_MSC, MSC_SCAN, i);
    input_report_key(fujitsu.idev, keycode, pressed);
    input_sync(fujitsu.idev);
    }
    }
    fujitsu_ack();
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_dmi_common(dmi: *const dmi_system_id) -> void __init {
    static void __init fujitsu_dmi_common(const struct dmi_system_id *dmi)
    {
    pr_info("%s\n", dmi.ident);
    memcpy(fujitsu.config.keymap, dmi.driver_data,
    sizeof(fujitsu.config.keymap));
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_dmi_lifebook(dmi: *const dmi_system_id) -> int __init {
    static int __init fujitsu_dmi_lifebook(const struct dmi_system_id *dmi)
    {
    fujitsu_dmi_common(dmi);
    fujitsu.config.quirks |= INVERT_TABLET_MODE_BIT;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_dmi_stylistic(dmi: *const dmi_system_id) -> int __init {
    static int __init fujitsu_dmi_stylistic(const struct dmi_system_id *dmi)
    {
    fujitsu_dmi_common(dmi);
    fujitsu.config.quirks |= FORCE_TABLET_MODE_IF_UNDOCK;
    fujitsu.config.quirks |= INVERT_DOCK_STATE_BIT;
    return 1;
    }
    static const struct dmi_system_id dmi_ids[] __initconst = {
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu Lifebook T901",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LifeBook T901")
    },
    .driver_data = keymap_Lifebook_T901
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu Lifebook T901",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK T901")
    },
    .driver_data = keymap_Lifebook_T901
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu Lifebook T902",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK T902")
    },
    .driver_data = keymap_Lifebook_T902
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu Siemens P/T Series",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LIFEBOOK")
    },
    .driver_data = keymap_Lifebook_Tseries
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu Lifebook T Series",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LifeBook T")
    },
    .driver_data = keymap_Lifebook_Tseries
    },
    {
    .callback = fujitsu_dmi_stylistic,
    .ident = "Fujitsu Siemens Stylistic T Series",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Stylistic T")
    },
    .driver_data = keymap_Stylistic_Tseries
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Fujitsu LifeBook U810",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "LifeBook U810")
    },
    .driver_data = keymap_Lifebook_U810
    },
    {
    .callback = fujitsu_dmi_stylistic,
    .ident = "Fujitsu Siemens Stylistic ST5xxx Series",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "STYLISTIC ST5")
    },
    .driver_data = keymap_Stylistic_ST5xxx
    },
    {
    .callback = fujitsu_dmi_stylistic,
    .ident = "Fujitsu Siemens Stylistic ST5xxx Series",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Stylistic ST5")
    },
    .driver_data = keymap_Stylistic_ST5xxx
    },
    {
    .callback = fujitsu_dmi_lifebook,
    .ident = "Unknown (using defaults)",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, ""),
    DMI_MATCH(DMI_PRODUCT_NAME, "")
    },
    .driver_data = keymap_Lifebook_Tseries
    },
    { core::ptr::null_mut() }
    };
#[no_mangle]
unsafe extern "C" fn fujitsu_walk_resources(res: *mut acpi_resource, data: *mut c_void) -> acpi_status {
    static acpi_status fujitsu_walk_resources(struct acpi_resource *res, void *data)
    {
    switch (res.type) {
    case ACPI_RESOURCE_TYPE_IRQ:
    fujitsu.irq = res.data.irq.interrupts[0];
    return AE_OK;
    case ACPI_RESOURCE_TYPE_IO:
    fujitsu.io_base = res.data.io.minimum;
    fujitsu.io_length = res.data.io.address_length;
    return AE_OK;
    case ACPI_RESOURCE_TYPE_END_TAG:
    if (fujitsu.irq && fujitsu.io_base)
    return AE_OK;
    else
    return AE_NOT_FOUND;
    default:
    return AE_ERROR;
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_fujitsu_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev;
    acpi_status status;
    int error;
    adev = ACPI_COMPANION(&pdev.dev);
    if (!adev)
    return -ENODEV;
    status = acpi_walk_resources(adev.handle, METHOD_NAME__CRS,
    fujitsu_walk_resources, core::ptr::null_mut());
    if (ACPI_FAILURE(status) || !fujitsu.irq || !fujitsu.io_base)
    return -ENODEV;
    scnprintf(fujitsu.name, sizeof(fujitsu.name), "Fujitsu %s", acpi_device_hid(adev));
    scnprintf(fujitsu.phys, sizeof(fujitsu.phys), "%s/input0", acpi_device_hid(adev));
    error = input_fujitsu_setup(&pdev.dev, fujitsu.name, fujitsu.phys);
    if (error)
    return error;
    if (!request_region(fujitsu.io_base, fujitsu.io_length, MODULENAME)) {
    input_fujitsu_remove();
    return -EBUSY;
    }
    fujitsu_reset();
    error = request_irq(fujitsu.irq, fujitsu_interrupt,
    IRQF_SHARED, MODULENAME, fujitsu_interrupt);
    if (error) {
    release_region(fujitsu.io_base, fujitsu.io_length);
    input_fujitsu_remove();
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_remove(pdev: *mut platform_device) {
    static void acpi_fujitsu_remove(struct platform_device *pdev)
    {
    free_irq(fujitsu.irq, fujitsu_interrupt);
    release_region(fujitsu.io_base, fujitsu.io_length);
    input_fujitsu_remove();
    }

#[no_mangle]
unsafe extern "C" fn acpi_fujitsu_resume(dev: *mut device) -> c_int {
    static int acpi_fujitsu_resume(struct device *dev)
    {
    fujitsu_reset();
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(acpi_fujitsu_pm, core::ptr::null_mut(), acpi_fujitsu_resume);
    static struct platform_driver acpi_fujitsu_driver = {
    .probe = acpi_fujitsu_probe,
    .remove = acpi_fujitsu_remove,
    .driver = {
    .name = MODULENAME,
    .acpi_match_table = fujitsu_ids,
    .pm = &acpi_fujitsu_pm,
    },
    };
#[no_mangle]
unsafe extern "C" fn fujitsu_module_init() -> int __init {
    static int __init fujitsu_module_init(void)
    {
    int error;
    dmi_check_system(dmi_ids);
    error = platform_driver_register(&acpi_fujitsu_driver);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fujitsu_module_exit() -> void __exit {
    static void __exit fujitsu_module_exit(void)
    {
    platform_driver_unregister(&acpi_fujitsu_driver);
    }
    module_init(fujitsu_module_init);
    module_exit(fujitsu_module_exit);
    MODULE_AUTHOR("Robert Gerlach <khnz@gmx.de>");
    MODULE_DESCRIPTION("Fujitsu tablet pc extras driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("2.5");
    MODULE_DEVICE_TABLE(acpi, fujitsu_ids);
