//! Automatically rewritten from C to Rust
//! Source: drivers/mcb/mcb-core.c
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
// MEN Chameleon Bus.
//
// Copyright (C) 2013 MEN Mikroelektronik GmbH (www.men.de)
// Author: Johannes Thumshirn <johannes.thumshirn@men.de>
//

    static DEFINE_IDA(mcb_ida);
    static const struct mcb_device_id *mcb_match_id(const struct mcb_device_id *ids,
    struct mcb_device *dev)
    {
    if (ids) {
    while (ids.device) {
    if (ids.device == dev.id)
    return ids;
    ids++;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mcb_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int mcb_match(struct device *dev, const struct device_driver *drv)
    {
    const struct mcb_driver *mdrv = to_mcb_driver(drv);
    struct mcb_device *mdev = to_mcb_device(dev);
    const struct mcb_device_id *found_id;
    found_id = mcb_match_id(mdrv.id_table, mdev);
    if (found_id)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcb_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int mcb_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct mcb_device *mdev = to_mcb_device(dev);
    int ret;
    ret = add_uevent_var(env, "MODALIAS=mcb:16z%03d", mdev.id);
    if (ret)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcb_probe(dev: *mut device) -> c_int {
    static int mcb_probe(struct device *dev)
    {
    struct mcb_driver *mdrv = to_mcb_driver(dev.driver);
    struct mcb_device *mdev = to_mcb_device(dev);
    const struct mcb_device_id *found_id;
    struct module *carrier_mod;
    int ret;
    found_id = mcb_match_id(mdrv.id_table, mdev);
    if (!found_id)
    return -ENODEV;
    carrier_mod = mdev.dev.parent.driver.owner;
    if (!try_module_get(carrier_mod))
    return -EINVAL;
    get_device(dev);
    ret = mdrv.probe(mdev, found_id);
    if (ret) {
    module_put(carrier_mod);
    put_device(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcb_remove(dev: *mut device) {
    static void mcb_remove(struct device *dev)
    {
    struct mcb_driver *mdrv = to_mcb_driver(dev.driver);
    struct mcb_device *mdev = to_mcb_device(dev);
    struct module *carrier_mod;
    if (mdrv.remove)
    mdrv.remove(mdev);
    carrier_mod = mdev.dev.parent.driver.owner;
    module_put(carrier_mod);
    put_device(&mdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn mcb_shutdown(dev: *mut device) {
    static void mcb_shutdown(struct device *dev)
    {
    struct mcb_driver *mdrv = to_mcb_driver(dev.driver);
    struct mcb_device *mdev = to_mcb_device(dev);
    if (mdrv && mdrv.shutdown)
    mdrv.shutdown(mdev);
    }
    static ssize_t revision_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mcb_bus *bus = to_mcb_bus(dev);
    return sysfs_emit(buf, "%d\n", bus.revision);
    }
    static DEVICE_ATTR_RO(revision);
    static ssize_t model_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mcb_bus *bus = to_mcb_bus(dev);
    return sysfs_emit(buf, "%c\n", bus.model);
    }
    static DEVICE_ATTR_RO(model);
    static ssize_t minor_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mcb_bus *bus = to_mcb_bus(dev);
    return sysfs_emit(buf, "%d\n", bus.minor);
    }
    static DEVICE_ATTR_RO(minor);
    static ssize_t name_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mcb_bus *bus = to_mcb_bus(dev);
    return sysfs_emit(buf, "%s\n", bus.name);
    }
    static DEVICE_ATTR_RO(name);
    static struct attribute *mcb_bus_attrs[] = {
    &dev_attr_revision.attr,
    &dev_attr_model.attr,
    &dev_attr_minor.attr,
    &dev_attr_name.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group mcb_carrier_group = {
    .attrs = mcb_bus_attrs,
    };
    static const struct attribute_group *mcb_carrier_groups[] = {
    &mcb_carrier_group,
    core::ptr::null_mut(),
    };
    static const struct bus_type mcb_bus_type = {
    .name = "mcb",
    .match = mcb_match,
    .uevent = mcb_uevent,
    .probe = mcb_probe,
    .remove = mcb_remove,
    .shutdown = mcb_shutdown,
    };
    static const struct device_type mcb_carrier_device_type = {
    .name = "mcb-carrier",
    .groups = mcb_carrier_groups,
    };
//
// __mcb_register_driver() - Register a @mcb_driver at the system
// @drv: The @mcb_driver
// @owner: The @mcb_driver's module
// @mod_name: The name of the @mcb_driver's module
//
// Register a @mcb_driver at the system. Perform a sanity check, if
// .probe method is provided by the driver.
//
    int __mcb_register_driver(struct mcb_driver *drv, struct module *owner,
    const char *mod_name)
    {
    if (!drv.probe)
    return -EINVAL;
    drv.driver.owner = owner;
    drv.driver.bus = &mcb_bus_type;
    drv.driver.mod_name = mod_name;
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL_NS_GPL(__mcb_register_driver, "MCB");
//
// mcb_unregister_driver() - Unregister a @mcb_driver from the system
// @drv: The @mcb_driver
//
// Unregister a @mcb_driver from the system.
//
#[no_mangle]
pub unsafe extern "C" fn mcb_unregister_driver(drv: *mut mcb_driver) {
    void mcb_unregister_driver(struct mcb_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_unregister_driver, "MCB");
#[no_mangle]
unsafe extern "C" fn mcb_release_dev(dev: *mut device) {
    static void mcb_release_dev(struct device *dev)
    {
    struct mcb_device *mdev = to_mcb_device(dev);
    mcb_bus_put(mdev.bus);
    kfree(mdev);
    }
//
// mcb_device_register() - Register a mcb_device
// @bus: The @mcb_bus of the device
// @dev: The @mcb_device
//
// Register a specific @mcb_device at a @mcb_bus and the system itself.
//
#[no_mangle]
pub unsafe extern "C" fn mcb_device_register(bus: *mut mcb_bus, dev: *mut mcb_device) -> c_int {
    int mcb_device_register(struct mcb_bus *bus, struct mcb_device *dev)
    {
    int ret;
    int device_id;
    device_initialize(&dev.dev);
    mcb_bus_get(bus);
    dev.dev.bus = &mcb_bus_type;
    dev.dev.parent = bus.dev.parent;
    dev.dev.release = mcb_release_dev;
    dev.dma_dev = bus.carrier;
    device_id = dev.id;
    dev_set_name(&dev.dev, "mcb%d-16z%03d-%d:%d:%d",
    bus.bus_nr, device_id, dev.inst, dev.group, dev.var);
    ret = device_add(&dev.dev);
    if (ret < 0) {
    pr_err("Failed registering device 16z%03d on bus mcb%d (%d)\n",
    device_id, bus.bus_nr, ret);
    goto out;
    }
    return 0;
    out:
    put_device(&dev.dev);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(mcb_device_register, "MCB");
#[no_mangle]
unsafe extern "C" fn mcb_free_bus(dev: *mut device) {
    static void mcb_free_bus(struct device *dev)
    {
    struct mcb_bus *bus = to_mcb_bus(dev);
    put_device(bus.carrier);
    ida_free(&mcb_ida, bus.bus_nr);
    kfree(bus);
    }
//
// mcb_alloc_bus() - Allocate a new @mcb_bus
// @carrier: generic &struct device for the carrier device
//
// Allocate a new @mcb_bus.
//
    struct mcb_bus *mcb_alloc_bus(struct device *carrier)
    {
    struct mcb_bus *bus;
    int bus_nr;
    int rc;
    bus = kzalloc_obj(struct mcb_bus);
    if (!bus)
    return ERR_PTR(-ENOMEM);
    bus_nr = ida_alloc(&mcb_ida, GFP_KERNEL);
    if (bus_nr < 0) {
    kfree(bus);
    return ERR_PTR(bus_nr);
    }
    bus.bus_nr = bus_nr;
    bus.carrier = get_device(carrier);
    device_initialize(&bus.dev);
    bus.dev.parent = carrier;
    bus.dev.bus = &mcb_bus_type;
    bus.dev.type = &mcb_carrier_device_type;
    bus.dev.release = mcb_free_bus;
    dev_set_name(&bus.dev, "mcb:%d", bus_nr);
    rc = device_add(&bus.dev);
    if (rc)
    goto err_put;
    return bus;
    err_put:
    put_device(&bus.dev);
    return ERR_PTR(rc);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_alloc_bus, "MCB");
#[no_mangle]
unsafe extern "C" fn __mcb_devices_unregister(dev: *mut device, data: *mut c_void) -> c_int {
    static int __mcb_devices_unregister(struct device *dev, void *data)
    {
    device_unregister(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mcb_devices_unregister(bus: *mut mcb_bus) {
    static void mcb_devices_unregister(struct mcb_bus *bus)
    {
    bus_for_each_dev(bus.dev.bus, core::ptr::null_mut(), core::ptr::null_mut(), __mcb_devices_unregister);
    }
//
// mcb_release_bus() - Free a @mcb_bus
// @bus: The @mcb_bus to release
//
// Release an allocated @mcb_bus from the system.
//
#[no_mangle]
pub unsafe extern "C" fn mcb_release_bus(bus: *mut mcb_bus) {
    void mcb_release_bus(struct mcb_bus *bus)
    {
    mcb_devices_unregister(bus);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_release_bus, "MCB");
//
// mcb_bus_get() - Increment refcnt
// @bus: The @mcb_bus
//
// Get a @mcb_bus' ref
//
    struct mcb_bus *mcb_bus_get(struct mcb_bus *bus)
    {
    if (bus)
    get_device(&bus.dev);
    return bus;
    }
    EXPORT_SYMBOL_NS_GPL(mcb_bus_get, "MCB");
//
// mcb_bus_put() - Decrement refcnt
// @bus: The @mcb_bus
//
// Release a @mcb_bus' ref
//
#[no_mangle]
pub unsafe extern "C" fn mcb_bus_put(bus: *mut mcb_bus) {
    void mcb_bus_put(struct mcb_bus *bus)
    {
    if (bus)
    put_device(&bus.dev);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_bus_put, "MCB");
//
// mcb_alloc_dev() - Allocate a device
// @bus: The @mcb_bus the device is part of
//
// Allocate a @mcb_device and add bus.
//
    struct mcb_device *mcb_alloc_dev(struct mcb_bus *bus)
    {
    struct mcb_device *dev;
    dev = kzalloc_obj(struct mcb_device);
    if (!dev)
    return core::ptr::null_mut();
    dev.bus = bus;
    return dev;
    }
    EXPORT_SYMBOL_NS_GPL(mcb_alloc_dev, "MCB");
//
// mcb_free_dev() - Free @mcb_device
// @dev: The device to free
//
// Free a @mcb_device
//
#[no_mangle]
pub unsafe extern "C" fn mcb_free_dev(dev: *mut mcb_device) {
    void mcb_free_dev(struct mcb_device *dev)
    {
    kfree(dev);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_free_dev, "MCB");
#[no_mangle]
unsafe extern "C" fn __mcb_bus_add_devices(dev: *mut device, data: *mut c_void) -> c_int {
    static int __mcb_bus_add_devices(struct device *dev, void *data)
    {
    int retval;
    retval = device_attach(dev);
    if (retval < 0) {
    dev_err(dev, "Error adding device (%d)\n", retval);
    return retval;
    }
    return 0;
    }
//
// mcb_bus_add_devices() - Add devices in the bus' internal device list
// @bus: The @mcb_bus we add the devices
//
// Add devices in the bus' internal device list to the system.
//
#[no_mangle]
pub unsafe extern "C" fn mcb_bus_add_devices(bus: *const mcb_bus) {
    void mcb_bus_add_devices(const struct mcb_bus *bus)
    {
    bus_for_each_dev(bus.dev.bus, core::ptr::null_mut(), core::ptr::null_mut(), __mcb_bus_add_devices);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_bus_add_devices, "MCB");
//
// mcb_get_resource() - get a resource for a mcb device
// @dev: the mcb device
// @type: the type of resource
//
    struct resource *mcb_get_resource(struct mcb_device *dev, unsigned int type)
    {
    if (type == IORESOURCE_MEM)
    return &dev.mem;
#[no_mangle]
pub unsafe extern "C" fn if(IORESOURCE_IRQ: type ==) -> else {
    else if (type == IORESOURCE_IRQ)
    return &dev.irq;
    else
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_NS_GPL(mcb_get_resource, "MCB");
//
// mcb_request_mem() - Request memory
// @dev: The @mcb_device the memory is for
// @name: The name for the memory reference.
//
// Request memory for a @mcb_device. If @name is NULL the driver name will
// be used.
//
    struct resource *mcb_request_mem(struct mcb_device *dev, const char *name)
    {
    struct resource *mem;
    u32 size;
    if (!name)
    name = dev.dev.driver.name;
    size = resource_size(&dev.mem);
    mem = request_mem_region(dev.mem.start, size, name);
    if (!mem)
    return ERR_PTR(-EBUSY);
    return mem;
    }
    EXPORT_SYMBOL_NS_GPL(mcb_request_mem, "MCB");
//
// mcb_release_mem() - Release memory requested by device
// @mem: The memory resource to be released
//
// Release memory that was prior requested via @mcb_request_mem().
//
#[no_mangle]
pub unsafe extern "C" fn mcb_release_mem(mem: *mut resource) {
    void mcb_release_mem(struct resource *mem)
    {
    u32 size;
    size = resource_size(mem);
    release_mem_region(mem.start, size);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_release_mem, "MCB");
#[no_mangle]
unsafe extern "C" fn __mcb_get_irq(dev: *mut mcb_device) -> c_int {
    static int __mcb_get_irq(struct mcb_device *dev)
    {
    struct resource *irq;
    irq = mcb_get_resource(dev, IORESOURCE_IRQ);
    return irq.start;
    }
//
// mcb_get_irq() - Get device's IRQ number
// @dev: The @mcb_device the IRQ is for
//
// Get the IRQ number of a given @mcb_device.
//
#[no_mangle]
pub unsafe extern "C" fn mcb_get_irq(dev: *mut mcb_device) -> c_int {
    int mcb_get_irq(struct mcb_device *dev)
    {
    struct mcb_bus *bus = dev.bus;
    if (bus.get_irq)
    return bus.get_irq(dev);
    return __mcb_get_irq(dev);
    }
    EXPORT_SYMBOL_NS_GPL(mcb_get_irq, "MCB");
#[no_mangle]
unsafe extern "C" fn mcb_init() -> c_int {
    static int mcb_init(void)
    {
    return bus_register(&mcb_bus_type);
    }
#[no_mangle]
unsafe extern "C" fn mcb_exit() {
    static void mcb_exit(void)
    {
    ida_destroy(&mcb_ida);
    bus_unregister(&mcb_bus_type);
    }
// mcb must be initialized after PCI but before the chameleon drivers.
// That means we must use some initcall between subsys_initcall and
// device_initcall.
//
    fs_initcall(mcb_init);
    module_exit(mcb_exit);
    MODULE_DESCRIPTION("MEN Chameleon Bus Driver");
    MODULE_AUTHOR("Johannes Thumshirn <johannes.thumshirn@men.de>");
    MODULE_LICENSE("GPL v2");
