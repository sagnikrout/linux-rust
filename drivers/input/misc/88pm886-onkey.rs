//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/88pm886-onkey.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm886_onkey {
    pub idev: *mut input_dev,
    pub chip: *mut pm886_chip,
}

#[no_mangle]
unsafe extern "C" fn pm886_onkey_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm886_onkey_irq_handler(int irq, void *data)
    {
    struct pm886_onkey *onkey = data;
    struct regmap *regmap = onkey.chip.regmap;
    struct input_dev *idev = onkey.idev;
    struct device *parent = idev.dev.parent;
    unsigned int val;
    int err;
    err = regmap_read(regmap, PM886_REG_STATUS1, &val);
    if (err) {
    dev_err(parent, "Failed to read status: %d\n", err);
    return IRQ_NONE;
    }
    val &= PM886_ONKEY_STS1;
    input_report_key(idev, KEY_POWER, val);
    input_sync(idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm886_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int pm886_onkey_probe(struct platform_device *pdev)
    {
    struct pm886_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct pm886_onkey *onkey;
    struct input_dev *idev;
    int irq, err;
    onkey = devm_kzalloc(dev, sizeof(*onkey), GFP_KERNEL);
    if (!onkey)
    return -ENOMEM;
    onkey.chip = chip;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return dev_err_probe(dev, irq, "Failed to get IRQ\n");
    idev = devm_input_allocate_device(dev);
    if (!idev) {
    dev_err(dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    onkey.idev = idev;
    idev.name = "88pm886-onkey";
    idev.phys = "88pm886-onkey/input0";
    idev.id.bustype = BUS_I2C;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    err = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), pm886_onkey_irq_handler,
    IRQF_ONESHOT | IRQF_NO_SUSPEND, "onkey",
    onkey);
    if (err)
    return dev_err_probe(dev, err, "Failed to request IRQ\n");
    err = input_register_device(idev);
    if (err)
    return dev_err_probe(dev, err, "Failed to register input device\n");
    return 0;
    }
    static const struct platform_device_id pm886_onkey_id_table[] = {
    { "88pm886-onkey", },
    { }
    };
    MODULE_DEVICE_TABLE(platform, pm886_onkey_id_table);
    static struct platform_driver pm886_onkey_driver = {
    .driver = {
    .name = "88pm886-onkey",
    },
    .probe = pm886_onkey_probe,
    .id_table = pm886_onkey_id_table,
    };
    module_platform_driver(pm886_onkey_driver);
    MODULE_DESCRIPTION("Marvell 88PM886 onkey driver");
    MODULE_AUTHOR("Karel Balej <balejk@matfyz.cz>");
    MODULE_LICENSE("GPL");
