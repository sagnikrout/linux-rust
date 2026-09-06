//! Automatically rewritten from C to Rust
//! Source: drivers/base/isa.c
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
// ISA bus.
//

    static struct device *isa_bus;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isa_dev {
    pub dev: device,
    pub next: *mut device,
    pub id: c_uint,
}

#[no_mangle]
unsafe extern "C" fn isa_bus_match(dev: *mut device, driver: *const device_driver) -> c_int {
    static int isa_bus_match(struct device *dev, const struct device_driver *driver)
    {
    struct isa_driver *isa_driver = to_isa_driver(driver);
    if (dev.platform_data == isa_driver) {
    if (!isa_driver.match ||
    isa_driver.match(dev, to_isa_dev(dev).id))
    return 1;
    dev.platform_data = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isa_bus_probe(dev: *mut device) -> c_int {
    static int isa_bus_probe(struct device *dev)
    {
    struct isa_driver *isa_driver = dev.platform_data;
    if (isa_driver && isa_driver.probe)
    return isa_driver.probe(dev, to_isa_dev(dev).id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isa_bus_remove(dev: *mut device) {
    static void isa_bus_remove(struct device *dev)
    {
    struct isa_driver *isa_driver = dev.platform_data;
    if (isa_driver && isa_driver.remove)
    isa_driver.remove(dev, to_isa_dev(dev).id);
    }
#[no_mangle]
unsafe extern "C" fn isa_bus_shutdown(dev: *mut device) {
    static void isa_bus_shutdown(struct device *dev)
    {
    struct isa_driver *isa_driver = dev.platform_data;
    if (isa_driver && isa_driver.shutdown)
    isa_driver.shutdown(dev, to_isa_dev(dev).id);
    }
#[no_mangle]
unsafe extern "C" fn isa_bus_suspend(dev: *mut device, state: pm_message_t) -> c_int {
    static int isa_bus_suspend(struct device *dev, pm_message_t state)
    {
    struct isa_driver *isa_driver = dev.platform_data;
    if (isa_driver && isa_driver.suspend)
    return isa_driver.suspend(dev, to_isa_dev(dev).id, state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isa_bus_resume(dev: *mut device) -> c_int {
    static int isa_bus_resume(struct device *dev)
    {
    struct isa_driver *isa_driver = dev.platform_data;
    if (isa_driver && isa_driver.resume)
    return isa_driver.resume(dev, to_isa_dev(dev).id);
    return 0;
    }
    static const struct bus_type isa_bus_type = {
    .name		= "isa",
    .match		= isa_bus_match,
    .probe		= isa_bus_probe,
    .remove		= isa_bus_remove,
    .shutdown	= isa_bus_shutdown,
    .suspend	= isa_bus_suspend,
    .resume		= isa_bus_resume
    };
#[no_mangle]
unsafe extern "C" fn isa_dev_release(dev: *mut device) {
    static void isa_dev_release(struct device *dev)
    {
    kfree(to_isa_dev(dev));
    }
#[no_mangle]
pub unsafe extern "C" fn isa_unregister_driver(isa_driver: *mut isa_driver) {
    void isa_unregister_driver(struct isa_driver *isa_driver)
    {
    struct device *dev = isa_driver.devices;
    while (dev) {
    struct device *tmp = to_isa_dev(dev).next;
    device_unregister(dev);
    dev = tmp;
    }
    driver_unregister(&isa_driver.driver);
    }
    EXPORT_SYMBOL_GPL(isa_unregister_driver);
#[no_mangle]
pub unsafe extern "C" fn isa_register_driver(isa_driver: *mut isa_driver, ndev: c_uint) -> c_int {
    int isa_register_driver(struct isa_driver *isa_driver, unsigned int ndev)
    {
    int error;
    unsigned int id;
    isa_driver.driver.bus	= &isa_bus_type;
    isa_driver.devices	= core::ptr::null_mut();
    error = driver_register(&isa_driver.driver);
    if (error)
    return error;
    for (id = 0; id < ndev; id++) {
    struct isa_dev *isa_dev;
    isa_dev = kzalloc_obj(*isa_dev);
    if (!isa_dev) {
    error = -ENOMEM;
    break;
    }
    isa_dev.dev.parent	= isa_bus;
    isa_dev.dev.bus	= &isa_bus_type;
    dev_set_name(&isa_dev.dev, "%s.%u",
    isa_driver.driver.name, id);
    isa_dev.dev.platform_data	= isa_driver;
    isa_dev.dev.release		= isa_dev_release;
    isa_dev.id			= id;
    isa_dev.dev.coherent_dma_mask = DMA_BIT_MASK(24);
    isa_dev.dev.dma_mask = &isa_dev.dev.coherent_dma_mask;
    error = device_register(&isa_dev.dev);
    if (error) {
    put_device(&isa_dev.dev);
    break;
    }
    isa_dev.next = isa_driver.devices;
    isa_driver.devices = &isa_dev.dev;
    }
    if (!error && !isa_driver.devices)
    error = -ENODEV;
    if (error)
    isa_unregister_driver(isa_driver);
    return error;
    }
    EXPORT_SYMBOL_GPL(isa_register_driver);
#[no_mangle]
unsafe extern "C" fn isa_bus_init() -> int __init {
    static int __init isa_bus_init(void)
    {
    int error;
    error = bus_register(&isa_bus_type);
    if (error)
    return error;
    isa_bus = root_device_register("isa");
    if (IS_ERR(isa_bus)) {
    bus_unregister(&isa_bus_type);
    return PTR_ERR(isa_bus);
    }
    return 0;
    }
    postcore_initcall(isa_bus_init);
