//! Automatically rewritten from C to Rust
//! Source: drivers/peci/core.c
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
// Copyright (c) 2018-2021 Intel Corporation

    static DEFINE_IDA(peci_controller_ida);
#[no_mangle]
unsafe extern "C" fn peci_controller_dev_release(dev: *mut device) {
    static void peci_controller_dev_release(struct device *dev)
    {
    struct peci_controller *controller = to_peci_controller(dev);
    mutex_destroy(&controller.bus_lock);
    ida_free(&peci_controller_ida, controller.id);
    kfree(controller);
    }
    const struct device_type peci_controller_type = {
    .release	= peci_controller_dev_release,
    };
#[no_mangle]
pub unsafe extern "C" fn peci_controller_scan_devices(controller: *mut peci_controller) -> c_int {
    int peci_controller_scan_devices(struct peci_controller *controller)
    {
    int ret;
    u8 addr;
    for (addr = PECI_BASE_ADDR; addr < PECI_BASE_ADDR + PECI_DEVICE_NUM_MAX; addr++) {
    ret = peci_device_create(controller, addr);
    if (ret)
    return ret;
    }
    return 0;
    }
    static struct peci_controller *peci_controller_alloc(struct device *dev,
    const struct peci_controller_ops *ops)
    {
    struct peci_controller *controller;
    int ret;
    if (!ops.xfer)
    return ERR_PTR(-EINVAL);
    controller = kzalloc_obj(*controller);
    if (!controller)
    return ERR_PTR(-ENOMEM);
    ret = ida_alloc_max(&peci_controller_ida, U8_MAX, GFP_KERNEL);
    if (ret < 0)
    goto err;
    controller.id = ret;
    controller.ops = ops;
    controller.dev.parent = dev;
    controller.dev.bus = &peci_bus_type;
    controller.dev.type = &peci_controller_type;
    device_initialize(&controller.dev);
    mutex_init(&controller.bus_lock);
    return controller;
    err:
    kfree(controller);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn unregister_child(dev: *mut device, dummy: *mut c_void) -> c_int {
    static int unregister_child(struct device *dev, void *dummy)
    {
    peci_device_destroy(to_peci_device(dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_controller(_controller: *mut c_void) {
    static void unregister_controller(void *_controller)
    {
    struct peci_controller *controller = _controller;
//
// Detach any active PECI devices. This can't fail, thus we do not
// check the returned value.
//
    device_for_each_child_reverse(&controller.dev, core::ptr::null_mut(), unregister_child);
    device_unregister(&controller.dev);
    fwnode_handle_put(controller.dev.fwnode);
    pm_runtime_disable(&controller.dev);
    }
//
// devm_peci_controller_add() - add PECI controller
// @dev: device for devm operations
// @ops: pointer to controller specific methods
//
// In final stage of its probe(), peci_controller driver calls
// devm_peci_controller_add() to register itself with the PECI bus.
//
// Return: Pointer to the newly allocated controller or ERR_PTR() in case of failure.
//
    struct peci_controller *devm_peci_controller_add(struct device *dev,
    const struct peci_controller_ops *ops)
    {
    struct peci_controller *controller;
    int ret;
    controller = peci_controller_alloc(dev, ops);
    if (IS_ERR(controller))
    return controller;
    ret = dev_set_name(&controller.dev, "peci-%d", controller.id);
    if (ret)
    goto err_put;
    pm_runtime_no_callbacks(&controller.dev);
    pm_suspend_ignore_children(&controller.dev, true);
    pm_runtime_enable(&controller.dev);
    device_set_node(&controller.dev, fwnode_handle_get(dev_fwnode(dev)));
    ret = device_add(&controller.dev);
    if (ret)
    goto err_fwnode;
    ret = devm_add_action_or_reset(dev, unregister_controller, controller);
    if (ret)
    return ERR_PTR(ret);
//
// Ignoring retval since failures during scan are non-critical for
// controller itself.
//
    peci_controller_scan_devices(controller);
    return controller;
    err_fwnode:
    fwnode_handle_put(controller.dev.fwnode);
    pm_runtime_disable(&controller.dev);
    err_put:
    put_device(&controller.dev);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_NS_GPL(devm_peci_controller_add, "PECI");
    static const struct peci_device_id *
    peci_bus_match_device_id(const struct peci_device_id *id, struct peci_device *device)
    {
    while (id.x86_vfm != 0) {
    if (id.x86_vfm == device.info.x86_vfm)
    return id;
    id++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn peci_bus_device_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int peci_bus_device_match(struct device *dev, const struct device_driver *drv)
    {
    struct peci_device *device = to_peci_device(dev);
    const struct peci_driver *peci_drv = to_peci_driver(drv);
    if (dev.type != &peci_device_type)
    return 0;
    return !!peci_bus_match_device_id(peci_drv.id_table, device);
    }
#[no_mangle]
unsafe extern "C" fn peci_bus_device_probe(dev: *mut device) -> c_int {
    static int peci_bus_device_probe(struct device *dev)
    {
    struct peci_device *device = to_peci_device(dev);
    struct peci_driver *driver = to_peci_driver(dev.driver);
    return driver.probe(device, peci_bus_match_device_id(driver.id_table, device));
    }
#[no_mangle]
unsafe extern "C" fn peci_bus_device_remove(dev: *mut device) {
    static void peci_bus_device_remove(struct device *dev)
    {
    struct peci_device *device = to_peci_device(dev);
    struct peci_driver *driver = to_peci_driver(dev.driver);
    if (driver.remove)
    driver.remove(device);
    }
    const struct bus_type peci_bus_type = {
    .name		= "peci",
    .match		= peci_bus_device_match,
    .probe		= peci_bus_device_probe,
    .remove		= peci_bus_device_remove,
    .bus_groups	= peci_bus_groups,
    };
#[no_mangle]
unsafe extern "C" fn peci_init() -> int __init {
    static int __init peci_init(void)
    {
    int ret;
    ret = bus_register(&peci_bus_type);
    if (ret < 0) {
    pr_err("peci: failed to register PECI bus type!\n");
    return ret;
    }
    return 0;
    }
    module_init(peci_init);
#[no_mangle]
unsafe extern "C" fn peci_exit() -> void __exit {
    static void __exit peci_exit(void)
    {
    bus_unregister(&peci_bus_type);
    }
    module_exit(peci_exit);
    MODULE_AUTHOR("Jason M Bills <jason.m.bills@linux.intel.com>");
    MODULE_AUTHOR("Jae Hyun Yoo <jae.hyun.yoo@linux.intel.com>");
    MODULE_AUTHOR("Iwona Winiarska <iwona.winiarska@intel.com>");
    MODULE_DESCRIPTION("PECI bus core module");
    MODULE_LICENSE("GPL");
