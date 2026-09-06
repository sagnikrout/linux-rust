//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/chips/chipreg.c
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
// Registration for chip drivers
//

    static DEFINE_SPINLOCK(chip_drvs_lock);
    static LIST_HEAD(chip_drvs_list);
#[no_mangle]
pub unsafe extern "C" fn register_mtd_chip_driver(drv: *mut mtd_chip_driver) {
    void register_mtd_chip_driver(struct mtd_chip_driver *drv)
    {
    spin_lock(&chip_drvs_lock);
    list_add(&drv.list, &chip_drvs_list);
    spin_unlock(&chip_drvs_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_mtd_chip_driver(drv: *mut mtd_chip_driver) {
    void unregister_mtd_chip_driver(struct mtd_chip_driver *drv)
    {
    spin_lock(&chip_drvs_lock);
    list_del(&drv.list);
    spin_unlock(&chip_drvs_lock);
    }
    static struct mtd_chip_driver *get_mtd_chip_driver (const char *name)
    {
    struct mtd_chip_driver *ret = core::ptr::null_mut(), *this;
    spin_lock(&chip_drvs_lock);
    list_for_each_entry(this, &chip_drvs_list, list) {
    if (!strcmp(this.name, name)) {
    ret = this;
    break;
    }
    }
    if (ret && !try_module_get(ret.module))
    ret = core::ptr::null_mut();
    spin_unlock(&chip_drvs_lock);
    return ret;
    }
// Hide all the horrid details, like some silly person taking
    get_module_symbol() away from us, from the caller. */
    struct mtd_info *do_map_probe(const char *name, struct map_info *map)
    {
    struct mtd_chip_driver *drv;
    struct mtd_info *ret;
    drv = get_mtd_chip_driver(name);
    if (!drv && !request_module("%s", name))
    drv = get_mtd_chip_driver(name);
    if (!drv)
    return core::ptr::null_mut();
    ret = drv.probe(map);
// We decrease the use count here. It may have been a
    probe-only module, which is no longer required from this
    point, having given us a handle on (and increased the use
    count of) the actual driver code.
//
    module_put(drv.module);
    return ret;
    }
//
// Destroy an MTD device which was created for a map device.
// Make sure the MTD device is already unregistered before calling this
//
#[no_mangle]
pub unsafe extern "C" fn map_destroy(mtd: *mut mtd_info) {
    void map_destroy(struct mtd_info *mtd)
    {
    struct map_info *map = mtd.priv;
    if (map.fldrv.destroy)
    map.fldrv.destroy(mtd);
    module_put(map.fldrv.module);
    kfree(mtd);
    }
    EXPORT_SYMBOL(register_mtd_chip_driver);
    EXPORT_SYMBOL(unregister_mtd_chip_driver);
    EXPORT_SYMBOL(do_map_probe);
    EXPORT_SYMBOL(map_destroy);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("Core routines for registering and invoking MTD chip drivers");
