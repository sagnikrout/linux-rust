//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/winmate-fm07-keys.c
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
//
// Driver for the Winmate FM07 front-panel keys
//
// Author: Daniel Beer <daniel.beer@tirotech.co.nz>

pub const PORT_CMD: c_uint = 0x6c;
pub const PORT_DATA: c_uint = 0x68;
pub const EC_ADDR_KEYS: c_uint = 0x3b;
pub const EC_CMD_READ: c_uint = 0x80;

pub const NUM_KEYS: c_int = 5;
// Typically we're done in fewer than 10 iterations
pub const LOOP_TIMEOUT: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn fm07keys_poll(input: *mut input_dev) {
    static void fm07keys_poll(struct input_dev *input)
    {
    uint8_t k;
    int i;
// Flush output buffer
    i = 0;
    while (inb(PORT_CMD) & 0x01) {
    if (++i >= LOOP_TIMEOUT)
    goto timeout;
    inb(PORT_DATA);
    }
// Send request and wait for write completion
    outb(EC_CMD_READ, PORT_CMD);
    i = 0;
    while (inb(PORT_CMD) & 0x02)
    if (++i >= LOOP_TIMEOUT)
    goto timeout;
    outb(EC_ADDR_KEYS, PORT_DATA);
    i = 0;
    while (inb(PORT_CMD) & 0x02)
    if (++i >= LOOP_TIMEOUT)
    goto timeout;
// Wait for data ready
    i = 0;
    while (!(inb(PORT_CMD) & 0x01))
    if (++i >= LOOP_TIMEOUT)
    goto timeout;
    k = inb(PORT_DATA);
// Notify of new key states
    for (i = 0; i < NUM_KEYS; i++) {
    input_report_key(input, BASE_KEY + i, (~k) & 1);
    k >>= 1;
    }
    input_sync(input);
    return;
    timeout:
    dev_warn_ratelimited(&input.dev, "timeout polling IO memory\n");
    }
#[no_mangle]
unsafe extern "C" fn fm07keys_probe(pdev: *mut platform_device) -> c_int {
    static int fm07keys_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct input_dev *input;
    int ret;
    int i;
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "no memory for input device\n");
    return -ENOMEM;
    }
    if (!devm_request_region(dev, PORT_CMD, 1, "Winmate FM07 EC"))
    return -EBUSY;
    if (!devm_request_region(dev, PORT_DATA, 1, "Winmate FM07 EC"))
    return -EBUSY;
    input.name = "Winmate FM07 front-panel keys";
    input.phys = DRV_NAME "/input0";
    input.id.bustype = BUS_HOST;
    input.id.vendor = 0x0001;
    input.id.product = 0x0001;
    input.id.version = 0x0100;
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < NUM_KEYS; i++)
    __set_bit(BASE_KEY + i, input.keybit);
    ret = input_setup_polling(input, fm07keys_poll);
    if (ret) {
    dev_err(dev, "unable to set up polling, err=%d\n", ret);
    return ret;
    }
// These are silicone buttons. They can't be pressed in rapid
// succession too quickly, and 50 Hz seems to be an adequate
// sampling rate without missing any events when tested.
//
    input_set_poll_interval(input, 20);
    ret = input_register_device(input);
    if (ret) {
    dev_err(dev, "unable to register polled device, err=%d\n",
    ret);
    return ret;
    }
    input_sync(input);
    return 0;
    }
    static struct platform_driver fm07keys_driver = {
    .probe		= fm07keys_probe,
    .driver		= {
    .name	= DRV_NAME
    },
    };
    static struct platform_device *dev;
    static const struct dmi_system_id fm07keys_dmi_table[] __initconst = {
    {
// FM07 and FM07P
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Winmate Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "IP30"),
    },
    },
    { }
    };
    MODULE_DEVICE_TABLE(dmi, fm07keys_dmi_table);
#[no_mangle]
unsafe extern "C" fn fm07keys_init() -> int __init {
    static int __init fm07keys_init(void)
    {
    int ret;
    if (!dmi_check_system(fm07keys_dmi_table))
    return -ENODEV;
    ret = platform_driver_register(&fm07keys_driver);
    if (ret) {
    pr_err("fm07keys: failed to register driver, err=%d\n", ret);
    return ret;
    }
    dev = platform_device_register_simple(DRV_NAME, PLATFORM_DEVID_NONE, core::ptr::null_mut(), 0);
    if (IS_ERR(dev)) {
    ret = PTR_ERR(dev);
    pr_err("fm07keys: failed to allocate device, err = %d\n", ret);
    goto fail_register;
    }
    return 0;
    fail_register:
    platform_driver_unregister(&fm07keys_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fm07keys_exit() -> void __exit {
    static void __exit fm07keys_exit(void)
    {
    platform_driver_unregister(&fm07keys_driver);
    platform_device_unregister(dev);
    }
    module_init(fm07keys_init);
    module_exit(fm07keys_exit);
    MODULE_AUTHOR("Daniel Beer <daniel.beer@tirotech.co.nz>");
    MODULE_DESCRIPTION("Winmate FM07 front-panel keys driver");
    MODULE_LICENSE("GPL");
