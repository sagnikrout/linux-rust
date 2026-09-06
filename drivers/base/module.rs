//! Automatically rewritten from C to Rust
//! Source: drivers/base/module.c
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
// module.c - module sysfs fun for drivers
//

    static char *make_driver_name(const struct device_driver *drv)
    {
    char *driver_name;
    driver_name = kasprintf(GFP_KERNEL, "%s:%s", drv.bus.name, drv.name);
    if (!driver_name)
    return core::ptr::null_mut();
    return driver_name;
    }
#[no_mangle]
unsafe extern "C" fn module_create_drivers_dir(mk: *mut module_kobject) {
    static void module_create_drivers_dir(struct module_kobject *mk)
    {
    static DEFINE_MUTEX(drivers_dir_mutex);
    mutex_lock(&drivers_dir_mutex);
    if (mk && !mk.drivers_dir)
    mk.drivers_dir = kobject_create_and_add("drivers", &mk.kobj);
    mutex_unlock(&drivers_dir_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn module_add_driver(mod: *mut module, drv: *const device_driver) -> c_int {
    int module_add_driver(struct module *mod, const struct device_driver *drv)
    {
    char *driver_name;
    struct module_kobject *mk = core::ptr::null_mut();
    int ret;
    if (!drv)
    return 0;
    if (mod)
    mk = &mod.mkobj;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: drv->mod_name) -> else {
// Lookup or create built-in module entry in /sys/module
    mk = lookup_or_create_module_kobject(drv.mod_name);
    if (mk) {
// remember our module structure
    drv.p.mkobj = mk;
// lookup_or_create_module_kobject took a reference
    kobject_put(&mk.kobj);
    }
    }
    if (!mk)
    return 0;
    ret = sysfs_create_link(&drv.p.kobj, &mk.kobj, "module");
    if (ret)
    return ret;
    driver_name = make_driver_name(drv);
    if (!driver_name) {
    ret = -ENOMEM;
    goto out_remove_kobj;
    }
    module_create_drivers_dir(mk);
    if (!mk.drivers_dir) {
    ret = -EINVAL;
    goto out_free_driver_name;
    }
    ret = sysfs_create_link(mk.drivers_dir, &drv.p.kobj, driver_name);
    if (ret)
    goto out_remove_drivers_dir;
    kfree(driver_name);
    return 0;
    out_remove_drivers_dir:
    sysfs_remove_link(mk.drivers_dir, driver_name);
    out_free_driver_name:
    kfree(driver_name);
    out_remove_kobj:
    sysfs_remove_link(&drv.p.kobj, "module");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn module_remove_driver(drv: *const device_driver) {
    void module_remove_driver(const struct device_driver *drv)
    {
    struct module_kobject *mk = core::ptr::null_mut();
    char *driver_name;
    if (!drv)
    return;
    sysfs_remove_link(&drv.p.kobj, "module");
    if (drv.owner)
    mk = &drv.owner.mkobj;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: drv->p->mkobj) -> else {
    else if (drv.p.mkobj)
    mk = drv.p.mkobj;
    if (mk && mk.drivers_dir) {
    driver_name = make_driver_name(drv);
    if (driver_name) {
    sysfs_remove_link(mk.drivers_dir, driver_name);
    kfree(driver_name);
    }
    }
    }
