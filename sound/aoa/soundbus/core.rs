//! Automatically rewritten from C to Rust
//! Source: sound/aoa/soundbus/core.c
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
// soundbus
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

    MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Apple Soundbus");
    struct soundbus_dev *soundbus_dev_get(struct soundbus_dev *dev)
    {
    struct device *tmp;
    if (!dev)
    return core::ptr::null_mut();
    tmp = get_device(&dev.ofdev.dev);
    if (tmp)
    return to_soundbus_device(tmp);
    else
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(soundbus_dev_get);
#[no_mangle]
pub unsafe extern "C" fn soundbus_dev_put(dev: *mut soundbus_dev) {
    void soundbus_dev_put(struct soundbus_dev *dev)
    {
    if (dev)
    put_device(&dev.ofdev.dev);
    }
    EXPORT_SYMBOL_GPL(soundbus_dev_put);
#[no_mangle]
unsafe extern "C" fn soundbus_probe(dev: *mut device) -> c_int {
    static int soundbus_probe(struct device *dev)
    {
    let mut error: c_int = -ENODEV;
    struct soundbus_driver *drv;
    struct soundbus_dev *soundbus_dev;
    drv = to_soundbus_driver(dev.driver);
    soundbus_dev = to_soundbus_device(dev);
    if (!drv.probe)
    return error;
    soundbus_dev_get(soundbus_dev);
    error = drv.probe(soundbus_dev);
    if (error)
    soundbus_dev_put(soundbus_dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn soundbus_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int soundbus_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct soundbus_dev * soundbus_dev;
    const struct platform_device * of;
    const char *compat;
    let mut retval: c_int = 0;
    int cplen, seen = 0;
    if (!dev)
    return -ENODEV;
    soundbus_dev = to_soundbus_device(dev);
    if (!soundbus_dev)
    return -ENODEV;
    of = &soundbus_dev.ofdev;
// stuff we want to pass to /sbin/hotplug
    retval = add_uevent_var(env, "OF_NAME=%pOFn", of.dev.of_node);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "OF_TYPE=%s", of_node_get_device_type(of.dev.of_node));
    if (retval)
    return retval;
// Since the compatible field can contain pretty much anything
// it's not really legal to split it out with commas. We split it
// up using a number of environment variables instead.
    compat = of_get_property(of.dev.of_node, "compatible", &cplen);
    while (compat && cplen > 0) {
    let mut tmp: c_int = env.buflen;
    retval = add_uevent_var(env, "OF_COMPATIBLE_%d=%s", seen, compat);
    if (retval)
    return retval;
    compat += env.buflen - tmp;
    cplen -= env.buflen - tmp;
    seen += 1;
    }
    retval = add_uevent_var(env, "OF_COMPATIBLE_N=%d", seen);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "MODALIAS=%s", soundbus_dev.modalias);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn soundbus_device_remove(dev: *mut device) {
    static void soundbus_device_remove(struct device *dev)
    {
    let mut soundbus_dev: *mut soundbus_dev = to_soundbus_device(dev);
    let mut drv: *mut soundbus_driver = to_soundbus_driver(dev.driver);
    if (dev.driver && drv.remove)
    drv.remove(soundbus_dev);
    soundbus_dev_put(soundbus_dev);
    }
#[no_mangle]
unsafe extern "C" fn soundbus_device_shutdown(dev: *mut device) {
    static void soundbus_device_shutdown(struct device *dev)
    {
    let mut soundbus_dev: *mut soundbus_dev = to_soundbus_device(dev);
    let mut drv: *mut soundbus_driver = to_soundbus_driver(dev.driver);
    if (dev.driver && drv.shutdown)
    drv.shutdown(soundbus_dev);
    }
// soundbus_dev_attrs is declared in sysfs.c
    ATTRIBUTE_GROUPS(soundbus_dev);
    static const struct bus_type soundbus_bus_type = {
    .name		= "aoa-soundbus",
    .probe		= soundbus_probe,
    .uevent		= soundbus_uevent,
    .remove		= soundbus_device_remove,
    .shutdown	= soundbus_device_shutdown,
    .dev_groups	= soundbus_dev_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn soundbus_add_one(dev: *mut soundbus_dev) -> c_int {
    int soundbus_add_one(struct soundbus_dev *dev)
    {
    static int devcount;
// sanity checks
    if (!dev.attach_codec ||
    !dev.ofdev.dev.of_node ||
    dev.pcmname ||
    dev.pcmid != -1) {
    printk(KERN_ERR "soundbus: adding device failed sanity check!\n");
    return -EINVAL;
    }
    dev_set_name(&dev.ofdev.dev, "soundbus:%x", ++devcount);
    dev.ofdev.dev.bus = &soundbus_bus_type;
    return of_device_register(&dev.ofdev);
    }
    EXPORT_SYMBOL_GPL(soundbus_add_one);
#[no_mangle]
pub unsafe extern "C" fn soundbus_remove_one(dev: *mut soundbus_dev) {
    void soundbus_remove_one(struct soundbus_dev *dev)
    {
    of_device_unregister(&dev.ofdev);
    }
    EXPORT_SYMBOL_GPL(soundbus_remove_one);
#[no_mangle]
pub unsafe extern "C" fn soundbus_register_driver(drv: *mut soundbus_driver) -> c_int {
    int soundbus_register_driver(struct soundbus_driver *drv)
    {
// initialize common driver fields
    drv.driver.name = drv.name;
    drv.driver.bus = &soundbus_bus_type;
// register with core
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(soundbus_register_driver);
#[no_mangle]
pub unsafe extern "C" fn soundbus_unregister_driver(drv: *mut soundbus_driver) {
    void soundbus_unregister_driver(struct soundbus_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(soundbus_unregister_driver);
#[no_mangle]
unsafe extern "C" fn soundbus_init() -> int __init {
    static int __init soundbus_init(void)
    {
    return bus_register(&soundbus_bus_type);
    }
#[no_mangle]
unsafe extern "C" fn soundbus_exit() -> void __exit {
    static void __exit soundbus_exit(void)
    {
    bus_unregister(&soundbus_bus_type);
    }
    subsys_initcall(soundbus_init);
    module_exit(soundbus_exit);
