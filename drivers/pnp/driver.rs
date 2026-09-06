//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/driver.c
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
// driver.c - device id matching, driver model, etc.
//
// Copyright 2002 Adam Belay <ambx1@neo.rr.com>
//

#[no_mangle]
unsafe extern "C" fn compare_func(ida: *const c_char, idb: *const c_char) -> c_int {
    static int compare_func(const char *ida, const char *idb)
    {
    int i;
// we only need to compare the last 4 chars
    for (i = 3; i < 7; i++) {
    if (ida[i] != 'X' &&
    idb[i] != 'X' && toupper(ida[i]) != toupper(idb[i]))
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn compare_pnp_id(pos: *mut pnp_id, id: *const c_char) -> c_int {
    int compare_pnp_id(struct pnp_id *pos, const char *id)
    {
    if (!pos || !id || (strlen(id) != 7))
    return 0;
    if (memcmp(id, "ANYDEVS", 7) == 0)
    return 1;
    while (pos) {
    if (memcmp(pos.id, id, 3) == 0)
    if (compare_func(pos.id, id) == 1)
    return 1;
    pos = pos.next;
    }
    return 0;
    }
    static const struct pnp_device_id *match_device(const struct pnp_driver *drv,
    struct pnp_dev *dev)
    {
    const struct pnp_device_id *drv_id = drv.id_table;
    if (!drv_id)
    return core::ptr::null_mut();
    while (*drv_id.id) {
    if (compare_pnp_id(dev.id, drv_id.id))
    return drv_id;
    drv_id++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_device_attach(pnp_dev: *mut pnp_dev) -> c_int {
    int pnp_device_attach(struct pnp_dev *pnp_dev)
    {
    mutex_lock(&pnp_lock);
    if (pnp_dev.status != PNP_READY) {
    mutex_unlock(&pnp_lock);
    return -EBUSY;
    }
    pnp_dev.status = PNP_ATTACHED;
    mutex_unlock(&pnp_lock);
    return 0;
    }
    EXPORT_SYMBOL(pnp_device_attach);
#[no_mangle]
pub unsafe extern "C" fn pnp_device_detach(pnp_dev: *mut pnp_dev) {
    void pnp_device_detach(struct pnp_dev *pnp_dev)
    {
    mutex_lock(&pnp_lock);
    if (pnp_dev.status == PNP_ATTACHED)
    pnp_dev.status = PNP_READY;
    mutex_unlock(&pnp_lock);
    }
    EXPORT_SYMBOL(pnp_device_detach);
#[no_mangle]
unsafe extern "C" fn pnp_device_probe(dev: *mut device) -> c_int {
    static int pnp_device_probe(struct device *dev)
    {
    int error;
    struct pnp_driver *pnp_drv;
    struct pnp_dev *pnp_dev;
    const struct pnp_device_id *dev_id = core::ptr::null_mut();
    pnp_dev = to_pnp_dev(dev);
    pnp_drv = to_pnp_driver(dev.driver);
    error = pnp_device_attach(pnp_dev);
    if (error < 0)
    return error;
    if (pnp_dev.active == 0) {
    if (!(pnp_drv.flags & PNP_DRIVER_RES_DO_NOT_CHANGE)) {
    error = pnp_activate_dev(pnp_dev);
    if (error < 0)
    return error;
    }
    } else if ((pnp_drv.flags & PNP_DRIVER_RES_DISABLE)
    == PNP_DRIVER_RES_DISABLE) {
    error = pnp_disable_dev(pnp_dev);
    if (error < 0)
    return error;
    }
    error = 0;
    if (pnp_drv.probe) {
    dev_id = match_device(pnp_drv, pnp_dev);
    if (dev_id != core::ptr::null_mut())
    error = pnp_drv.probe(pnp_dev, dev_id);
    }
    if (error >= 0) {
    pnp_dev.driver = pnp_drv;
    error = 0;
    } else
    goto fail;
    return error;
    fail:
    pnp_device_detach(pnp_dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pnp_device_remove(dev: *mut device) {
    static void pnp_device_remove(struct device *dev)
    {
    struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    struct pnp_driver *drv = pnp_dev.driver;
    if (drv) {
    if (drv.remove)
    drv.remove(pnp_dev);
    pnp_dev.driver = core::ptr::null_mut();
    }
    if (pnp_dev.active &&
    (!drv || !(drv.flags & PNP_DRIVER_RES_DO_NOT_CHANGE)))
    pnp_disable_dev(pnp_dev);
    pnp_device_detach(pnp_dev);
    }
#[no_mangle]
unsafe extern "C" fn pnp_device_shutdown(dev: *mut device) {
    static void pnp_device_shutdown(struct device *dev)
    {
    struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    struct pnp_driver *drv = pnp_dev.driver;
    if (drv && drv.shutdown)
    drv.shutdown(pnp_dev);
    }
#[no_mangle]
unsafe extern "C" fn pnp_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int pnp_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    struct pnp_id *pos;
    const struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    if (!dev)
    return -ENODEV;
    pos = pnp_dev.id;
    while (pos) {
    if (add_uevent_var(env, "MODALIAS=pnp:d%s", pos.id))
    return -ENOMEM;
    pos = pos.next;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pnp_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int pnp_bus_match(struct device *dev, const struct device_driver *drv)
    {
    struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    const struct pnp_driver *pnp_drv = to_pnp_driver(drv);
    if (match_device(pnp_drv, pnp_dev) == core::ptr::null_mut())
    return 0;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn __pnp_bus_suspend(dev: *mut device, state: pm_message_t) -> c_int {
    static int __pnp_bus_suspend(struct device *dev, pm_message_t state)
    {
    struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    struct pnp_driver *pnp_drv = pnp_dev.driver;
    int error;
    if (!pnp_drv)
    return 0;
    if (pnp_drv.driver.pm && pnp_drv.driver.pm.suspend) {
    error = pnp_drv.driver.pm.suspend(dev);
    suspend_report_result(dev, pnp_drv.driver.pm.suspend, error);
    if (error)
    return error;
    }
    if (pnp_drv.suspend) {
    error = pnp_drv.suspend(pnp_dev, state);
    if (error)
    return error;
    }
// can_write is necessary to be able to re-start the device on resume
    if (pnp_can_disable(pnp_dev) && pnp_can_write(pnp_dev)) {
    error = pnp_stop_dev(pnp_dev);
    if (error)
    return error;
    }
    if (pnp_can_suspend(pnp_dev))
    pnp_dev.protocol.suspend(pnp_dev, state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pnp_bus_suspend(dev: *mut device) -> c_int {
    static int pnp_bus_suspend(struct device *dev)
    {
    return __pnp_bus_suspend(dev, PMSG_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn pnp_bus_freeze(dev: *mut device) -> c_int {
    static int pnp_bus_freeze(struct device *dev)
    {
    return __pnp_bus_suspend(dev, PMSG_FREEZE);
    }
#[no_mangle]
unsafe extern "C" fn pnp_bus_poweroff(dev: *mut device) -> c_int {
    static int pnp_bus_poweroff(struct device *dev)
    {
    return __pnp_bus_suspend(dev, PMSG_HIBERNATE);
    }
#[no_mangle]
unsafe extern "C" fn pnp_bus_resume(dev: *mut device) -> c_int {
    static int pnp_bus_resume(struct device *dev)
    {
    struct pnp_dev *pnp_dev = to_pnp_dev(dev);
    struct pnp_driver *pnp_drv = pnp_dev.driver;
    int error;
    if (!pnp_drv)
    return 0;
    if (pnp_dev.protocol.resume) {
    error = pnp_dev.protocol.resume(pnp_dev);
    if (error)
    return error;
    }
    if (pnp_can_write(pnp_dev)) {
    error = pnp_start_dev(pnp_dev);
    if (error)
    return error;
    }
    if (pnp_drv.driver.pm && pnp_drv.driver.pm.resume) {
    error = pnp_drv.driver.pm.resume(dev);
    if (error)
    return error;
    }
    if (pnp_drv.resume) {
    error = pnp_drv.resume(pnp_dev);
    if (error)
    return error;
    }
    return 0;
    }
    static const struct dev_pm_ops pnp_bus_dev_pm_ops = {
// Suspend callbacks
    .suspend = pnp_bus_suspend,
    .resume = pnp_bus_resume,
// Hibernate callbacks
    .freeze = pnp_bus_freeze,
    .thaw = pnp_bus_resume,
    .poweroff = pnp_bus_poweroff,
    .restore = pnp_bus_resume,
    };
    const struct bus_type pnp_bus_type = {
    .name    = "pnp",
    .match   = pnp_bus_match,
    .uevent  = pnp_uevent,
    .probe   = pnp_device_probe,
    .remove  = pnp_device_remove,
    .shutdown = pnp_device_shutdown,
    .pm	 = &pnp_bus_dev_pm_ops,
    .dev_groups = pnp_dev_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn dev_is_pnp(dev: *const device) -> bool {
    bool dev_is_pnp(const struct device *dev)
    {
    return dev.bus == &pnp_bus_type;
    }
    EXPORT_SYMBOL_GPL(dev_is_pnp);
#[no_mangle]
pub unsafe extern "C" fn pnp_register_driver(drv: *mut pnp_driver) -> c_int {
    int pnp_register_driver(struct pnp_driver *drv)
    {
    drv.driver.name = drv.name;
    drv.driver.bus = &pnp_bus_type;
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL(pnp_register_driver);
#[no_mangle]
pub unsafe extern "C" fn pnp_unregister_driver(drv: *mut pnp_driver) {
    void pnp_unregister_driver(struct pnp_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL(pnp_unregister_driver);
//
// pnp_add_id - adds an EISA id to the specified device
// @dev: pointer to the desired device
// @id: pointer to an EISA id string
//
    struct pnp_id *pnp_add_id(struct pnp_dev *dev, const char *id)
    {
    struct pnp_id *dev_id, *ptr;
    dev_id = kzalloc_obj(struct pnp_id);
    if (!dev_id)
    return core::ptr::null_mut();
    dev_id.id[0] = id[0];
    dev_id.id[1] = id[1];
    dev_id.id[2] = id[2];
    dev_id.id[3] = tolower(id[3]);
    dev_id.id[4] = tolower(id[4]);
    dev_id.id[5] = tolower(id[5]);
    dev_id.id[6] = tolower(id[6]);
    dev_id.id[7] = '\0';
    dev_id.next = core::ptr::null_mut();
    ptr = dev.id;
    while (ptr && ptr.next)
    ptr = ptr.next;
    if (ptr)
    ptr.next = dev_id;
    else
    dev.id = dev_id;
    return dev_id;
    }
