//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/ideapad_slidebar.c
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
// Input driver for slidebars on some Lenovo IdeaPad laptops
//
// Copyright (C) 2013 Andrey Moiseev <o2g.org.ru@gmail.com>
//
// Reverse-engineered from Lenovo SlideNav software (SBarHook.dll).
//
// Trademarks are the property of their respective owners.
//
// Currently tested and works on:
// Lenovo IdeaPad Y550
// Lenovo IdeaPad Y550P
//
// Other models can be added easily. To test,
// load with 'force' parameter set 'true'.
//
// LEDs blinking and input mode are managed via sysfs,
// (hex, unsigned byte value):
// /sys/devices/platform/ideapad_slidebar/slidebar_mode
//
// The value is in byte range, however, I only figured out
// how bits 0b10011001 work. Some other bits, probably,
// are meaningful too.
//
// Possible states:
//
// STD_INT, ONMOV_INT, OFF_INT, LAST_POLL, OFF_POLL
//
// Meaning:
// released      touched
// STD       'heartbeat'   lights follow the finger
// ONMOV     no lights     lights follow the finger
// LAST      at last pos   lights follow the finger
// OFF       no lights     no lights
//
// INT       all input events are generated, interrupts are used
// POLL      no input events by default, to get them,
// send 0b10000000 (read below)
//
// Commands: write
//
// All      |  0b01001 -> STD_INT
// possible |  0b10001 -> ONMOV_INT
// states   |  0b01000 -> OFF_INT
//
// |  0b0 -> LAST_POLL
// STD_INT or ONMOV_INT |
// |  0b1 -> STD_INT
//
// |  0b0 -> OFF_POLL
// OFF_INT or OFF_POLL  |
// |  0b1 -> OFF_INT
//
// Any state |   0b10000000 ->  if the slidebar has updated data,
// produce one input event (last position),
// switch to respective POLL mode
// (like 0x0), if not in POLL mode yet.
//
// Get current state: read
//
// masked by 0x11 read value means:
//
// 0x00   LAST
// 0x01   STD
// 0x10   OFF
// 0x11   ONMOV
//

pub const IDEAPAD_BASE: c_uint = 0xff29;
    static bool force;
    module_param(force, bool, 0);
    MODULE_PARM_DESC(force, "Force driver load, ignore DMI data");
    static DEFINE_SPINLOCK(io_lock);
    static struct input_dev *slidebar_input_dev;
    static struct platform_device *slidebar_platform_dev;
#[no_mangle]
unsafe extern "C" fn slidebar_pos_get() -> u8 {
    static u8 slidebar_pos_get(void)
    {
    guard(spinlock_irqsave)(&io_lock);
    outb(0xf4, 0xff29);
    outb(0xbf, 0xff2a);
    return inb(0xff2b);
    }
#[no_mangle]
unsafe extern "C" fn slidebar_mode_get() -> u8 {
    static u8 slidebar_mode_get(void)
    {
    guard(spinlock_irqsave)(&io_lock);
    outb(0xf7, 0xff29);
    outb(0x8b, 0xff2a);
    return inb(0xff2b);
    }
#[no_mangle]
unsafe extern "C" fn slidebar_mode_set(mode: u8) {
    static void slidebar_mode_set(u8 mode)
    {
    guard(spinlock_irqsave)(&io_lock);
    outb(0xf7, 0xff29);
    outb(0x8b, 0xff2a);
    outb(mode, 0xff2b);
    }
    static bool slidebar_i8042_filter(unsigned char data, unsigned char str,
    struct serio *port, void *context)
    {
    let mut extended: static bool = false;
// We are only interested in data coming form KBC port
    if (str & I8042_STR_AUXDATA)
    return false;
// Scancodes: e03b on move, e0bb on release.
    if (data == 0xe0) {
    extended = true;
    return true;
    }
    if (!extended)
    return false;
    extended = false;
    if (likely((data & 0x7f) != 0x3b)) {
    serio_interrupt(port, 0xe0, 0);
    return false;
    }
    if (data & 0x80) {
    input_report_key(slidebar_input_dev, BTN_TOUCH, 0);
    } else {
    input_report_key(slidebar_input_dev, BTN_TOUCH, 1);
    input_report_abs(slidebar_input_dev, ABS_X, slidebar_pos_get());
    }
    input_sync(slidebar_input_dev);
    return true;
    }
    static ssize_t show_slidebar_mode(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "%x\n", slidebar_mode_get());
    }
    static ssize_t store_slidebar_mode(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u8 mode;
    int error;
    error = kstrtou8(buf, 0, &mode);
    if (error)
    return error;
    slidebar_mode_set(mode);
    return count;
    }
    static DEVICE_ATTR(slidebar_mode, S_IWUSR | S_IRUGO,
    show_slidebar_mode, store_slidebar_mode);
    static struct attribute *ideapad_attrs[] = {
    &dev_attr_slidebar_mode.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group ideapad_attr_group = {
    .attrs = ideapad_attrs
    };
    static const struct attribute_group *ideapad_attr_groups[] = {
    &ideapad_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn ideapad_probe(pdev: *mut *mut platform_device) -> int __init {
    static int __init ideapad_probe(struct platform_device* pdev)
    {
    int err;
    if (!request_region(IDEAPAD_BASE, 3, "ideapad_slidebar")) {
    dev_err(&pdev.dev, "IO ports are busy\n");
    return -EBUSY;
    }
    slidebar_input_dev = input_allocate_device();
    if (!slidebar_input_dev) {
    dev_err(&pdev.dev, "Failed to allocate input device\n");
    err = -ENOMEM;
    goto err_release_ports;
    }
    slidebar_input_dev.name = "IdeaPad Slidebar";
    slidebar_input_dev.id.bustype = BUS_HOST;
    slidebar_input_dev.dev.parent = &pdev.dev;
    input_set_capability(slidebar_input_dev, EV_KEY, BTN_TOUCH);
    input_set_capability(slidebar_input_dev, EV_ABS, ABS_X);
    input_set_abs_params(slidebar_input_dev, ABS_X, 0, 0xff, 0, 0);
    err = i8042_install_filter(slidebar_i8042_filter, core::ptr::null_mut());
    if (err) {
    dev_err(&pdev.dev,
    "Failed to install i8042 filter: %d\n", err);
    goto err_free_dev;
    }
    err = input_register_device(slidebar_input_dev);
    if (err) {
    dev_err(&pdev.dev,
    "Failed to register input device: %d\n", err);
    goto err_remove_filter;
    }
    return 0;
    err_remove_filter:
    i8042_remove_filter(slidebar_i8042_filter);
    err_free_dev:
    input_free_device(slidebar_input_dev);
    err_release_ports:
    release_region(IDEAPAD_BASE, 3);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ideapad_remove(pdev: *mut platform_device) {
    static void ideapad_remove(struct platform_device *pdev)
    {
    i8042_remove_filter(slidebar_i8042_filter);
    input_unregister_device(slidebar_input_dev);
    release_region(IDEAPAD_BASE, 3);
    }
    static struct platform_driver slidebar_drv = {
    .driver = {
    .name = "ideapad_slidebar",
    },
    .remove = ideapad_remove,
    };
#[no_mangle]
unsafe extern "C" fn ideapad_dmi_check(id: *const dmi_system_id) -> int __init {
    static int __init ideapad_dmi_check(const struct dmi_system_id *id)
    {
    pr_info("Laptop model '%s'\n", id.ident);
    return 1;
    }
    static const struct dmi_system_id ideapad_dmi[] __initconst = {
    {
    .ident = "Lenovo IdeaPad Y550",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "20017"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "Lenovo IdeaPad Y550")
    },
    .callback = ideapad_dmi_check
    },
    {
    .ident = "Lenovo IdeaPad Y550P",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "20035"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "Lenovo IdeaPad Y550P")
    },
    .callback = ideapad_dmi_check
    },
    { core::ptr::null_mut(), }
    };
    MODULE_DEVICE_TABLE(dmi, ideapad_dmi);
#[no_mangle]
unsafe extern "C" fn slidebar_init() -> int __init {
    static int __init slidebar_init(void)
    {
    int err;
    if (!force && !dmi_check_system(ideapad_dmi)) {
    pr_err("DMI does not match\n");
    return -ENODEV;
    }
    slidebar_platform_dev = platform_device_alloc("ideapad_slidebar", -1);
    if (!slidebar_platform_dev) {
    pr_err("Not enough memory\n");
    return -ENOMEM;
    }
    slidebar_platform_dev.dev.groups = ideapad_attr_groups;
    err = platform_device_add(slidebar_platform_dev);
    if (err) {
    pr_err("Failed to register platform device\n");
    goto err_free_dev;
    }
    err = platform_driver_probe(&slidebar_drv, ideapad_probe);
    if (err) {
    pr_err("Failed to register platform driver\n");
    goto err_delete_dev;
    }
    return 0;
    err_delete_dev:
    platform_device_del(slidebar_platform_dev);
    err_free_dev:
    platform_device_put(slidebar_platform_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn slidebar_exit() -> void __exit {
    static void __exit slidebar_exit(void)
    {
    platform_device_unregister(slidebar_platform_dev);
    platform_driver_unregister(&slidebar_drv);
    }
    module_init(slidebar_init);
    module_exit(slidebar_exit);
    MODULE_AUTHOR("Andrey Moiseev <o2g.org.ru@gmail.com>");
    MODULE_DESCRIPTION("Slidebar input support for some Lenovo IdeaPad laptops");
    MODULE_LICENSE("GPL");
