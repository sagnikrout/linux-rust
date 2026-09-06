//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/adp5520-keys.c
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
// Keypad driver for Analog Devices ADP5520 MFD PMICs
//
// Copyright 2009 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_keys {
    pub input: *mut input_dev,
    pub notifier: notifier_block,
    pub master: *mut device,
    pub keycode: [c_ushort; ADP5520_KEYMAPSIZE],
}

    static void adp5520_keys_report_event(struct adp5520_keys *dev,
    unsigned short keymask, int value)
    {
    int i;
    for (i = 0; i < ADP5520_MAXKEYS; i++)
    if (keymask & (1 << i))
    input_report_key(dev.input, dev.keycode[i], value);
    input_sync(dev.input);
    }
    static int adp5520_keys_notifier(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct adp5520_keys *dev;
    uint8_t reg_val_lo, reg_val_hi;
    unsigned short keymask;
    dev = container_of(nb, struct adp5520_keys, notifier);
    if (event & ADP5520_KP_INT) {
    adp5520_read(dev.master, ADP5520_KP_INT_STAT_1, &reg_val_lo);
    adp5520_read(dev.master, ADP5520_KP_INT_STAT_2, &reg_val_hi);
    keymask = (reg_val_hi << 8) | reg_val_lo;
// Read twice to clear
    adp5520_read(dev.master, ADP5520_KP_INT_STAT_1, &reg_val_lo);
    adp5520_read(dev.master, ADP5520_KP_INT_STAT_2, &reg_val_hi);
    keymask |= (reg_val_hi << 8) | reg_val_lo;
    adp5520_keys_report_event(dev, keymask, 1);
    }
    if (event & ADP5520_KR_INT) {
    adp5520_read(dev.master, ADP5520_KR_INT_STAT_1, &reg_val_lo);
    adp5520_read(dev.master, ADP5520_KR_INT_STAT_2, &reg_val_hi);
    keymask = (reg_val_hi << 8) | reg_val_lo;
// Read twice to clear
    adp5520_read(dev.master, ADP5520_KR_INT_STAT_1, &reg_val_lo);
    adp5520_read(dev.master, ADP5520_KR_INT_STAT_2, &reg_val_hi);
    keymask |= (reg_val_hi << 8) | reg_val_lo;
    adp5520_keys_report_event(dev, keymask, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_keys_probe(pdev: *mut platform_device) -> c_int {
    static int adp5520_keys_probe(struct platform_device *pdev)
    {
    struct adp5520_keys_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct input_dev *input;
    struct adp5520_keys *dev;
    int ret, i;
    unsigned char en_mask, ctl_mask = 0;
    if (pdev.id != ID_ADP5520) {
    dev_err(&pdev.dev, "only ADP5520 supports Keypad\n");
    return -EINVAL;
    }
    if (!pdata) {
    dev_err(&pdev.dev, "missing platform data\n");
    return -EINVAL;
    }
    if (!(pdata.rows_en_mask && pdata.cols_en_mask))
    return -EINVAL;
    dev = devm_kzalloc(&pdev.dev, sizeof(*dev), GFP_KERNEL);
    if (!dev) {
    dev_err(&pdev.dev, "failed to alloc memory\n");
    return -ENOMEM;
    }
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    dev.master = pdev.dev.parent;
    dev.input = input;
    input.name = pdev.name;
    input.phys = "adp5520-keys/input0";
    input.dev.parent = &pdev.dev;
    input.id.bustype = BUS_I2C;
    input.id.vendor = 0x0001;
    input.id.product = 0x5520;
    input.id.version = 0x0001;
    input.keycodesize = sizeof(dev.keycode[0]);
    input.keycodemax = pdata.keymapsize;
    input.keycode = dev.keycode;
    memcpy(dev.keycode, pdata.keymap,
    pdata.keymapsize * input.keycodesize);
// setup input device
    __set_bit(EV_KEY, input.evbit);
    if (pdata.repeat)
    __set_bit(EV_REP, input.evbit);
    for (i = 0; i < input.keycodemax; i++)
    __set_bit(dev.keycode[i], input.keybit);
    __clear_bit(KEY_RESERVED, input.keybit);
    ret = input_register_device(input);
    if (ret) {
    dev_err(&pdev.dev, "unable to register input device\n");
    return ret;
    }
    en_mask = pdata.rows_en_mask | pdata.cols_en_mask;
    ret = adp5520_set_bits(dev.master, ADP5520_GPIO_CFG_1, en_mask);
    if (en_mask & ADP5520_COL_C3)
    ctl_mask |= ADP5520_C3_MODE;
    if (en_mask & ADP5520_ROW_R3)
    ctl_mask |= ADP5520_R3_MODE;
    if (ctl_mask)
    ret |= adp5520_set_bits(dev.master, ADP5520_LED_CONTROL,
    ctl_mask);
    ret |= adp5520_set_bits(dev.master, ADP5520_GPIO_PULLUP,
    pdata.rows_en_mask);
    if (ret) {
    dev_err(&pdev.dev, "failed to write\n");
    return -EIO;
    }
    dev.notifier.notifier_call = adp5520_keys_notifier;
    ret = adp5520_register_notifier(dev.master, &dev.notifier,
    ADP5520_KP_IEN | ADP5520_KR_IEN);
    if (ret) {
    dev_err(&pdev.dev, "failed to register notifier\n");
    return ret;
    }
    platform_set_drvdata(pdev, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_keys_remove(pdev: *mut platform_device) {
    static void adp5520_keys_remove(struct platform_device *pdev)
    {
    struct adp5520_keys *dev = platform_get_drvdata(pdev);
    adp5520_unregister_notifier(dev.master, &dev.notifier,
    ADP5520_KP_IEN | ADP5520_KR_IEN);
    }
    static struct platform_driver adp5520_keys_driver = {
    .driver	= {
    .name	= "adp5520-keys",
    },
    .probe		= adp5520_keys_probe,
    .remove		= adp5520_keys_remove,
    };
    module_platform_driver(adp5520_keys_driver);
    MODULE_AUTHOR("Michael Hennerich <hennerich@blackfin.uclinux.org>");
    MODULE_DESCRIPTION("Keys ADP5520 Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:adp5520-keys");
