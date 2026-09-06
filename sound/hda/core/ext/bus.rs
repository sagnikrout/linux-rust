//! Automatically rewritten from C to Rust
//! Source: sound/hda/core/ext/bus.c
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
// hdac-ext-bus.c - HD-audio extended core bus functions.
//
// Copyright (C) 2014-2015 Intel Corp
// Author: Jeeja KP <jeeja.kp@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

    MODULE_DESCRIPTION("HDA extended core");
    MODULE_LICENSE("GPL v2");
//
// snd_hdac_ext_bus_init - initialize a HD-audio extended bus
// @bus: the pointer to HDAC bus object
// @dev: device pointer
// @ops: bus verb operators
// @ext_ops: operators used for ASoC HDA codec drivers
//
// Returns 0 if successful, or a negative error code.
//
    int snd_hdac_ext_bus_init(struct hdac_bus *bus, struct device *dev,
    const struct hdac_bus_ops *ops,
    const struct hdac_ext_bus_ops *ext_ops)
    {
    int ret;
    ret = snd_hdac_bus_init(bus, dev, ops);
    if (ret < 0)
    return ret;
    bus.ext_ops = ext_ops;
// FIXME:
// Currently only one bus is supported, if there is device with more
// buses, bus->idx should be greater than 0, but there needs to be a
// reliable way to always assign same number.
//
    bus.idx = 0;
    bus.cmd_dma_state = true;
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_init);
//
// snd_hdac_ext_bus_exit - clean up a HD-audio extended bus
// @bus: the pointer to HDAC bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_exit(bus: *mut hdac_bus) {
    void snd_hdac_ext_bus_exit(struct hdac_bus *bus)
    {
    snd_hdac_bus_exit(bus);
    WARN_ON(!list_empty(&bus.hlink_list));
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_exit);
//
// snd_hdac_ext_bus_device_remove - remove HD-audio extended codec base devices
//
// @bus: the pointer to HDAC bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_device_remove(bus: *mut hdac_bus) {
    void snd_hdac_ext_bus_device_remove(struct hdac_bus *bus)
    {
    struct hdac_device *codec, *__codec;
//
// we need to remove all the codec devices objects created in the
// snd_hdac_ext_bus_device_init
//
    list_for_each_entry_safe(codec, __codec, &bus.codec_list, list) {
    snd_hdac_device_unregister(codec);
    put_device(&codec.dev);
    }
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_device_remove);

    struct hdac_device, dev))
    static inline struct hdac_driver *get_hdrv(struct device *dev)
    {
    struct hdac_driver *hdrv = drv_to_hdac_driver(dev.driver);
    return hdrv;
    }
    static inline struct hdac_device *get_hdev(struct device *dev)
    {
    struct hdac_device *hdev = dev_to_hdac_dev(dev);
    return hdev;
    }
#[no_mangle]
unsafe extern "C" fn hda_ext_drv_probe(dev: *mut device) -> c_int {
    static int hda_ext_drv_probe(struct device *dev)
    {
    return (get_hdrv(dev)).probe(get_hdev(dev));
    }
#[no_mangle]
unsafe extern "C" fn hdac_ext_drv_remove(dev: *mut device) -> c_int {
    static int hdac_ext_drv_remove(struct device *dev)
    {
    return (get_hdrv(dev)).remove(get_hdev(dev));
    }
#[no_mangle]
unsafe extern "C" fn hdac_ext_drv_shutdown(dev: *mut device) {
    static void hdac_ext_drv_shutdown(struct device *dev)
    {
    return (get_hdrv(dev)).shutdown(get_hdev(dev));
    }
//
// snd_hda_ext_driver_register - register a driver for ext hda devices
//
// @drv: ext hda driver structure
//
#[no_mangle]
pub unsafe extern "C" fn snd_hda_ext_driver_register(drv: *mut hdac_driver) -> c_int {
    int snd_hda_ext_driver_register(struct hdac_driver *drv)
    {
    drv.type = HDA_DEV_ASOC;
    drv.driver.bus = &snd_hda_bus_type;
// we use default match
    if (drv.probe)
    drv.driver.probe = hda_ext_drv_probe;
    if (drv.remove)
    drv.driver.remove = hdac_ext_drv_remove;
    if (drv.shutdown)
    drv.driver.shutdown = hdac_ext_drv_shutdown;
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(snd_hda_ext_driver_register);
//
// snd_hda_ext_driver_unregister - unregister a driver for ext hda devices
//
// @drv: ext hda driver structure
//
#[no_mangle]
pub unsafe extern "C" fn snd_hda_ext_driver_unregister(drv: *mut hdac_driver) {
    void snd_hda_ext_driver_unregister(struct hdac_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(snd_hda_ext_driver_unregister);
