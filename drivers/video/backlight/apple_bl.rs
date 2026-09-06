//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/apple_bl.c
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
// Backlight Driver for Intel-based Apples
//
// Copyright (c) Red Hat <mjg@redhat.com>
// Based on code from Pommed:
// Copyright (C) 2006 Nicolas Boichat <nicolas @boichat.ch>
// Copyright (C) 2006 Felipe Alfaro Solana <felipe_alfaro @linuxmail.org>
// Copyright (C) 2007 Julien BLACHE <jb@jblache.org>
//
// This driver triggers SMIs which cause the firmware to change the
// backlight brightness. This is icky in many ways, but it's impractical to
// get at the firmware code in order to figure out what it's actually doing.
//

    static struct backlight_device *apple_backlight_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_data {
// I/O resource to allocate.
    pub iostart: c_ulong,
    pub iolen: c_ulong,
// Backlight operations structure.
    pub backlight_ops: backlight_ops,
    pub (*set_brightness)(int): *mut c_void,
}

    static const struct hw_data *hw_data;
// Module parameters.
    static int debug;
    module_param_named(debug, debug, int, 0644);
    MODULE_PARM_DESC(debug, "Set to one to enable debugging messages.");
//
// Implementation for machines with Intel chipset.
//
#[no_mangle]
unsafe extern "C" fn intel_chipset_set_brightness(intensity: c_int) {
    static void intel_chipset_set_brightness(int intensity)
    {
    outb(0x04 | (intensity << 4), 0xb3);
    outb(0xbf, 0xb2);
    }
#[no_mangle]
unsafe extern "C" fn intel_chipset_send_intensity(bd: *mut backlight_device) -> c_int {
    static int intel_chipset_send_intensity(struct backlight_device *bd)
    {
    let mut intensity: c_int = bd.props.brightness;
    if (debug)
    pr_debug("setting brightness to %d\n", intensity);
    intel_chipset_set_brightness(intensity);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_chipset_get_intensity(bd: *mut backlight_device) -> c_int {
    static int intel_chipset_get_intensity(struct backlight_device *bd)
    {
    int intensity;
    outb(0x03, 0xb3);
    outb(0xbf, 0xb2);
    intensity = inb(0xb3) >> 4;
    if (debug)
    pr_debug("read brightness of %d\n", intensity);
    return intensity;
    }
    static const struct hw_data intel_chipset_data = {
    .iostart = 0xb2,
    .iolen = 2,
    .backlight_ops	= {
    .options	= BL_CORE_SUSPENDRESUME,
    .get_brightness	= intel_chipset_get_intensity,
    .update_status	= intel_chipset_send_intensity,
    },
    .set_brightness = intel_chipset_set_brightness,
    };
//
// Implementation for machines with Nvidia chipset.
//
#[no_mangle]
unsafe extern "C" fn nvidia_chipset_set_brightness(intensity: c_int) {
    static void nvidia_chipset_set_brightness(int intensity)
    {
    outb(0x04 | (intensity << 4), 0x52f);
    outb(0xbf, 0x52e);
    }
#[no_mangle]
unsafe extern "C" fn nvidia_chipset_send_intensity(bd: *mut backlight_device) -> c_int {
    static int nvidia_chipset_send_intensity(struct backlight_device *bd)
    {
    let mut intensity: c_int = bd.props.brightness;
    if (debug)
    pr_debug("setting brightness to %d\n", intensity);
    nvidia_chipset_set_brightness(intensity);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvidia_chipset_get_intensity(bd: *mut backlight_device) -> c_int {
    static int nvidia_chipset_get_intensity(struct backlight_device *bd)
    {
    int intensity;
    outb(0x03, 0x52f);
    outb(0xbf, 0x52e);
    intensity = inb(0x52f) >> 4;
    if (debug)
    pr_debug("read brightness of %d\n", intensity);
    return intensity;
    }
    static const struct hw_data nvidia_chipset_data = {
    .iostart = 0x52e,
    .iolen = 2,
    .backlight_ops		= {
    .options	= BL_CORE_SUSPENDRESUME,
    .get_brightness	= nvidia_chipset_get_intensity,
    .update_status	= nvidia_chipset_send_intensity
    },
    .set_brightness = nvidia_chipset_set_brightness,
    };
#[no_mangle]
unsafe extern "C" fn apple_bl_probe(pdev: *mut platform_device) -> c_int {
    static int apple_bl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct pci_dev *host;
    int intensity;
    host = pci_get_domain_bus_and_slot(0, 0, 0);
    if (!host) {
    pr_err("unable to find PCI host\n");
    return -ENODEV;
    }
    if (host.vendor == PCI_VENDOR_ID_INTEL)
    hw_data = &intel_chipset_data;
#[no_mangle]
pub unsafe extern "C" fn if(PCI_VENDOR_ID_NVIDIA: host->vendor ==) -> else {
    else if (host.vendor == PCI_VENDOR_ID_NVIDIA)
    hw_data = &nvidia_chipset_data;
    pci_dev_put(host);
    if (!hw_data) {
    pr_err("unknown hardware\n");
    return -ENODEV;
    }
// Check that the hardware responds - this may not work under EFI
    intensity = hw_data.backlight_ops.get_brightness(core::ptr::null_mut());
    if (!intensity) {
    hw_data.set_brightness(1);
    if (!hw_data.backlight_ops.get_brightness(core::ptr::null_mut()))
    return -ENODEV;
    hw_data.set_brightness(0);
    }
    if (!request_region(hw_data.iostart, hw_data.iolen,
    "Apple backlight"))
    return -ENXIO;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = 15;
    apple_backlight_device = backlight_device_register("apple_backlight",
    core::ptr::null_mut(), core::ptr::null_mut(), &hw_data.backlight_ops, &props);
    if (IS_ERR(apple_backlight_device)) {
    release_region(hw_data.iostart, hw_data.iolen);
    return PTR_ERR(apple_backlight_device);
    }
    apple_backlight_device.props.brightness =
    hw_data.backlight_ops.get_brightness(apple_backlight_device);
    backlight_update_status(apple_backlight_device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_bl_remove(pdev: *mut platform_device) {
    static void apple_bl_remove(struct platform_device *pdev)
    {
    backlight_device_unregister(apple_backlight_device);
    release_region(hw_data.iostart, hw_data.iolen);
    hw_data = core::ptr::null_mut();
    }
    static const struct acpi_device_id apple_bl_ids[] = {
    {"APP0002", 0},
    {"", 0},
    };
    static struct platform_driver apple_bl_driver = {
    .probe = apple_bl_probe,
    .remove = apple_bl_remove,
    .driver = {
    .name = "Apple backlight",
    .acpi_match_table = apple_bl_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn apple_bl_init() -> int __init {
    static int __init apple_bl_init(void)
    {
//
// Use ACPI video detection code to see if this driver should register
// or if another driver, e.g. the apple-gmux driver should be used.
//
    if (acpi_video_get_backlight_type() != acpi_backlight_vendor)
    return -ENODEV;
    return platform_driver_register(&apple_bl_driver);
    }
#[no_mangle]
unsafe extern "C" fn apple_bl_exit() -> void __exit {
    static void __exit apple_bl_exit(void)
    {
    platform_driver_unregister(&apple_bl_driver);
    }
    module_init(apple_bl_init);
    module_exit(apple_bl_exit);
    MODULE_AUTHOR("Matthew Garrett <mjg@redhat.com>");
    MODULE_DESCRIPTION("Apple Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(acpi, apple_bl_ids);
    MODULE_ALIAS("mbp_nvidia_bl");
