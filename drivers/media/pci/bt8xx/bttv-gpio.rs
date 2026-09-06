//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/bt8xx/bttv-gpio.c
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
    bttv-gpio.c  --  gpio sub drivers
    sysfs-based sub driver interface for bttv
    mainly intended for gpio access
    Copyright (C) 1996,97,98 Ralph  Metzler (rjkm@thp.uni-koeln.de)
    & Marcus Metzler (mocm@thp.uni-koeln.de)
    (c) 1999-2003 Gerd Knorr <kraxel@bytesex.org>
//

// -----------------------------------------------------------------------
// internal: the bttv "bus"
#[no_mangle]
unsafe extern "C" fn bttv_sub_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int bttv_sub_bus_match(struct device *dev, const struct device_driver *drv)
    {
    const struct bttv_sub_driver *sub = to_bttv_sub_drv(drv);
    let mut len: c_int = strlen(sub.wanted);
    if (0 == strncmp(dev_name(dev), sub.wanted, len))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bttv_sub_probe(dev: *mut device) -> c_int {
    static int bttv_sub_probe(struct device *dev)
    {
    struct bttv_sub_device *sdev = to_bttv_sub_dev(dev);
    struct bttv_sub_driver *sub = to_bttv_sub_drv(dev.driver);
    return sub.probe ? sub.probe(sdev) : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn bttv_sub_remove(dev: *mut device) {
    static void bttv_sub_remove(struct device *dev)
    {
    struct bttv_sub_device *sdev = to_bttv_sub_dev(dev);
    struct bttv_sub_driver *sub = to_bttv_sub_drv(dev.driver);
    if (sub.remove)
    sub.remove(sdev);
    }
    const struct bus_type bttv_sub_bus_type = {
    .name   = "bttv-sub",
    .match  = &bttv_sub_bus_match,
    .probe  = bttv_sub_probe,
    .remove = bttv_sub_remove,
    };
#[no_mangle]
unsafe extern "C" fn release_sub_device(dev: *mut device) {
    static void release_sub_device(struct device *dev)
    {
    struct bttv_sub_device *sub = to_bttv_sub_dev(dev);
    kfree(sub);
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_sub_add_device(core: *mut bttv_core, name: *mut c_char) -> c_int {
    int bttv_sub_add_device(struct bttv_core *core, char *name)
    {
    struct bttv_sub_device *sub;
    int err;
    sub = kzalloc_obj(*sub);
    if (core::ptr::null_mut() == sub)
    return -ENOMEM;
    sub.core        = core;
    sub.dev.parent  = &core.pci.dev;
    sub.dev.bus     = &bttv_sub_bus_type;
    sub.dev.release = release_sub_device;
    dev_set_name(&sub.dev, "%s%d", name, core.nr);
    err = device_register(&sub.dev);
    if (0 != err) {
    put_device(&sub.dev);
    return err;
    }
    pr_info("%d: add subdevice \"%s\"\n", core.nr, dev_name(&sub.dev));
    list_add_tail(&sub.list,&core.subs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_sub_del_devices(core: *mut bttv_core) -> c_int {
    int bttv_sub_del_devices(struct bttv_core *core)
    {
    struct bttv_sub_device *sub, *save;
    list_for_each_entry_safe(sub, save, &core.subs, list) {
    list_del(&sub.list);
    device_unregister(&sub.dev);
    }
    return 0;
    }
// -----------------------------------------------------------------------
// external: sub-driver register/unregister
#[no_mangle]
pub unsafe extern "C" fn bttv_sub_register(sub: *mut bttv_sub_driver, wanted: *mut c_char) -> c_int {
    int bttv_sub_register(struct bttv_sub_driver *sub, char *wanted)
    {
    sub.drv.bus = &bttv_sub_bus_type;
    snprintf(sub.wanted,sizeof(sub.wanted),"%s",wanted);
    return driver_register(&sub.drv);
    }
    EXPORT_SYMBOL(bttv_sub_register);
#[no_mangle]
pub unsafe extern "C" fn bttv_sub_unregister(sub: *mut bttv_sub_driver) -> c_int {
    int bttv_sub_unregister(struct bttv_sub_driver *sub)
    {
    driver_unregister(&sub.drv);
    return 0;
    }
    EXPORT_SYMBOL(bttv_sub_unregister);
// -----------------------------------------------------------------------
// external: gpio access functions
#[no_mangle]
pub unsafe extern "C" fn bttv_gpio_inout(core: *mut bttv_core, mask: u32, outbits: u32) {
    void bttv_gpio_inout(struct bttv_core *core, u32 mask, u32 outbits)
    {
    struct bttv *btv = container_of(core, struct bttv, c);
    unsigned long flags;
    u32 data;
    spin_lock_irqsave(&btv.gpio_lock,flags);
    data = btread(BT848_GPIO_OUT_EN);
    data = data & ~mask;
    data = data | (mask & outbits);
    btwrite(data,BT848_GPIO_OUT_EN);
    spin_unlock_irqrestore(&btv.gpio_lock,flags);
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_gpio_read(core: *mut bttv_core) -> u32 {
    u32 bttv_gpio_read(struct bttv_core *core)
    {
    struct bttv *btv = container_of(core, struct bttv, c);
    u32 value;
    value = btread(BT848_GPIO_DATA);
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_gpio_write(core: *mut bttv_core, value: u32) {
    void bttv_gpio_write(struct bttv_core *core, u32 value)
    {
    struct bttv *btv = container_of(core, struct bttv, c);
    btwrite(value,BT848_GPIO_DATA);
    }
#[no_mangle]
pub unsafe extern "C" fn bttv_gpio_bits(core: *mut bttv_core, mask: u32, bits: u32) {
    void bttv_gpio_bits(struct bttv_core *core, u32 mask, u32 bits)
    {
    struct bttv *btv = container_of(core, struct bttv, c);
    unsigned long flags;
    u32 data;
    spin_lock_irqsave(&btv.gpio_lock,flags);
    data = btread(BT848_GPIO_DATA);
    data = data & ~mask;
    data = data | (mask & bits);
    btwrite(data,BT848_GPIO_DATA);
    spin_unlock_irqrestore(&btv.gpio_lock,flags);
    }
