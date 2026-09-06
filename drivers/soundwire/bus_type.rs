//! Automatically rewritten from C to Rust
//! Source: drivers/soundwire/bus_type.c
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
// Copyright(c) 2015-17 Intel Corporation.

//
// sdw_get_device_id - find the matching SoundWire device id
// @slave: SoundWire Slave Device
// @drv: SoundWire Slave Driver
//
// The match is done by comparing the mfg_id and part_id from the
// struct sdw_device_id.
//
    static const struct sdw_device_id *
    sdw_get_device_id(struct sdw_slave *slave, const struct sdw_driver *drv)
    {
    const struct sdw_device_id *id;
    for (id = drv.id_table; id && id.mfg_id; id++)
    if (slave.id.mfg_id == id.mfg_id &&
    slave.id.part_id == id.part_id  &&
    (!id.sdw_version ||
    slave.id.sdw_version == id.sdw_version) &&
    (!id.class_id ||
    slave.id.class_id == id.class_id))
    return id;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sdw_bus_match(dev: *mut device, ddrv: *const device_driver) -> c_int {
    static int sdw_bus_match(struct device *dev, const struct device_driver *ddrv)
    {
    struct sdw_slave *slave;
    const struct sdw_driver *drv;
    let mut ret: c_int = 0;
    if (is_sdw_slave(dev)) {
    slave = dev_to_sdw_dev(dev);
    drv = drv_to_sdw_driver(ddrv);
    ret = !!sdw_get_device_id(slave, drv);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sdw_slave_modalias(slave: *const sdw_slave, buf: *mut c_char, size: usize) -> c_int {
    int sdw_slave_modalias(const struct sdw_slave *slave, char *buf, size_t size)
    {
// modalias is sdw:m<mfg_id>p<part_id>v<version>c<class_id>
    return snprintf(buf, size, "sdw:m%04Xp%04Xv%02Xc%02X\n",
    slave.id.mfg_id, slave.id.part_id,
    slave.id.sdw_version, slave.id.class_id);
    }
#[no_mangle]
pub unsafe extern "C" fn sdw_slave_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    int sdw_slave_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct sdw_slave *slave = dev_to_sdw_dev(dev);
    char modalias[32];
    sdw_slave_modalias(slave, modalias, sizeof(modalias));
    if (add_uevent_var(env, "MODALIAS=%s", modalias))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdw_bus_probe(dev: *mut device) -> c_int {
    static int sdw_bus_probe(struct device *dev)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    struct sdw_driver *drv = drv_to_sdw_driver(dev.driver);
    const struct sdw_device_id *id;
    int ret;
//
// fw description is mandatory to bind
//
    if (!dev.fwnode)
    return -ENODEV;
    if (!IS_ENABLED(CONFIG_ACPI) && !dev.of_node)
    return -ENODEV;
    id = sdw_get_device_id(slave, drv);
    if (!id)
    return -ENODEV;
//
// attach to power domain but don't turn on (last arg)
//
    ret = dev_pm_domain_attach(dev, 0);
    if (ret)
    return ret;
    ret = ida_alloc_max(&slave.bus.slave_ida, SDW_FW_MAX_DEVICES - 1, GFP_KERNEL);
    if (ret < 0) {
    dev_err(dev, "Failed to allocated ID: %d\n", ret);
    return ret;
    }
    slave.index = ret;
// Create IRQ mapping now so the driver can get it in probe()
    sdw_irq_create_mapping(slave);
    ret = drv.probe(slave, id);
    if (ret) {
    ida_free(&slave.bus.slave_ida, slave.index);
    return ret;
    }
    mutex_lock(&slave.sdw_dev_lock);
// device is probed so let's read the properties now
    if (drv.ops && drv.ops.read_prop)
    drv.ops.read_prop(slave);
// init the dynamic sysfs attributes we need
    ret = sdw_slave_sysfs_dpn_init(slave);
    if (ret < 0)
    dev_warn(dev, "failed to initialise sysfs: %d\n", ret);
//
// Check for valid clk_stop_timeout, use DisCo worst case value of
// 300ms
//
// TODO: check the timeouts and driver removal case
//
    if (slave.prop.clk_stop_timeout == 0)
    slave.prop.clk_stop_timeout = 300;
    slave.bus.clk_stop_timeout = max_t(u32, slave.bus.clk_stop_timeout,
    slave.prop.clk_stop_timeout);
    slave.probed = true;
//
// if the probe happened after the bus was started, notify the codec driver
// of the current hardware status to e.g. start the initialization.
// Errors are only logged as warnings to avoid failing the probe.
//
    if (drv.ops && drv.ops.update_status) {
    ret = drv.ops.update_status(slave, slave.status);
    if (ret < 0)
    dev_warn(dev, "failed to update status at probe: %d\n", ret);
    }
    mutex_unlock(&slave.sdw_dev_lock);
    dev_dbg(dev, "probe complete\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdw_bus_remove(dev: *mut device) {
    static void sdw_bus_remove(struct device *dev)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    struct sdw_driver *drv = drv_to_sdw_driver(dev.driver);
    mutex_lock(&slave.sdw_dev_lock);
    slave.probed = false;
    mutex_unlock(&slave.sdw_dev_lock);
    if (drv.remove)
    drv.remove(slave);
    ida_free(&slave.bus.slave_ida, slave.index);
    }
#[no_mangle]
unsafe extern "C" fn sdw_bus_shutdown(dev: *mut device) {
    static void sdw_bus_shutdown(struct device *dev)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    struct sdw_driver *drv = drv_to_sdw_driver(dev.driver);
    if (dev.driver && drv.shutdown)
    drv.shutdown(slave);
    }
    const struct bus_type sdw_bus_type = {
    .name = "soundwire",
    .match = sdw_bus_match,
    .probe = sdw_bus_probe,
    .remove = sdw_bus_remove,
    .shutdown = sdw_bus_shutdown,
    };
    EXPORT_SYMBOL_GPL(sdw_bus_type);
//
// __sdw_register_driver() - register a SoundWire Slave driver
// @drv: driver to register
// @owner: owning module/driver
//
// Return: zero on success, else a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn __sdw_register_driver(drv: *mut sdw_driver, owner: *mut module) -> c_int {
    int __sdw_register_driver(struct sdw_driver *drv, struct module *owner)
    {
    drv.driver.bus = &sdw_bus_type;
    if (!drv.probe) {
    pr_err("driver %s didn't provide SDW probe routine\n",
    drv.driver.name);
    return -EINVAL;
    }
    drv.driver.owner = owner;
    drv.driver.dev_groups = sdw_attr_groups;
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(__sdw_register_driver);
//
// sdw_unregister_driver() - unregisters the SoundWire Slave driver
// @drv: driver to unregister
//
#[no_mangle]
pub unsafe extern "C" fn sdw_unregister_driver(drv: *mut sdw_driver) {
    void sdw_unregister_driver(struct sdw_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL_GPL(sdw_unregister_driver);
#[no_mangle]
unsafe extern "C" fn sdw_bus_init() -> int __init {
    static int __init sdw_bus_init(void)
    {
    sdw_debugfs_init();
    return bus_register(&sdw_bus_type);
    }
#[no_mangle]
unsafe extern "C" fn sdw_bus_exit() -> void __exit {
    static void __exit sdw_bus_exit(void)
    {
    sdw_debugfs_exit();
    bus_unregister(&sdw_bus_type);
    }
    postcore_initcall(sdw_bus_init);
    module_exit(sdw_bus_exit);
    MODULE_DESCRIPTION("SoundWire bus");
    MODULE_LICENSE("GPL v2");
