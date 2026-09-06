//! Automatically rewritten from C to Rust
//! Source: sound/hda/core/hda_bus_type.c
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
// HD-audio bus
//

    MODULE_DESCRIPTION("HD-audio bus");
    MODULE_LICENSE("GPL");
//
// hdac_get_device_id - gets the hdac device id entry
// @hdev: HD-audio core device
// @drv: HD-audio codec driver
//
// Compares the hdac device vendor_id and revision_id to the hdac_device
// driver id_table and returns the matching device id entry.
//
    const struct hda_device_id *
    hdac_get_device_id(struct hdac_device *hdev, const struct hdac_driver *drv)
    {
    if (drv.id_table) {
    const struct hda_device_id *id  = drv.id_table;
    while (id.vendor_id) {
    if (hdev.vendor_id == id.vendor_id &&
    (!id.rev_id || id.rev_id == hdev.revision_id))
    return id;
    id++;
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(hdac_get_device_id);
#[no_mangle]
unsafe extern "C" fn hdac_codec_match(dev: *mut hdac_device, drv: *const hdac_driver) -> c_int {
    static int hdac_codec_match(struct hdac_device *dev, const struct hdac_driver *drv)
    {
    return !!hdac_get_device_id(dev, drv);
    }
#[no_mangle]
unsafe extern "C" fn hda_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int hda_bus_match(struct device *dev, const struct device_driver *drv)
    {
    struct hdac_device *hdev = dev_to_hdac_dev(dev);
    const struct hdac_driver *hdrv = drv_to_hdac_driver(drv);
    if (hdev.type != hdrv.type)
    return 0;
//
// if driver provided a match function use that otherwise we will
// use hdac_codec_match function
//
    if (hdrv.match)
    return hdrv.match(hdev, hdrv);
    return hdac_codec_match(hdev, hdrv);
    }
#[no_mangle]
unsafe extern "C" fn hda_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int hda_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    char modalias[32];
    snd_hdac_codec_modalias(dev_to_hdac_dev(dev), modalias,
    sizeof(modalias));
    if (add_uevent_var(env, "MODALIAS=%s", modalias))
    return -ENOMEM;
    return 0;
    }
    const struct bus_type snd_hda_bus_type = {
    .name = "hdaudio",
    .match = hda_bus_match,
    .uevent = hda_uevent,
    };
    EXPORT_SYMBOL_GPL(snd_hda_bus_type);
#[no_mangle]
unsafe extern "C" fn hda_bus_init() -> int __init {
    static int __init hda_bus_init(void)
    {
    return bus_register(&snd_hda_bus_type);
    }
#[no_mangle]
unsafe extern "C" fn hda_bus_exit() -> void __exit {
    static void __exit hda_bus_exit(void)
    {
    bus_unregister(&snd_hda_bus_type);
    }
    subsys_initcall(hda_bus_init);
    module_exit(hda_bus_exit);
