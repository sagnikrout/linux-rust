//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/scm.c
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
// Recognize and maintain s390 storage class memory.
//
// Copyright IBM Corp. 2012
// Author(s): Sebastian Ott <sebott@linux.vnet.ibm.com>
//

    static struct device *scm_root;

#[no_mangle]
unsafe extern "C" fn scmdev_probe(dev: *mut device) -> c_int {
    static int scmdev_probe(struct device *dev)
    {
    struct scm_device *scmdev = to_scm_dev(dev);
    struct scm_driver *scmdrv = to_scm_drv(dev.driver);
    return scmdrv.probe ? scmdrv.probe(scmdev) : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn scmdev_remove(dev: *mut device) {
    static void scmdev_remove(struct device *dev)
    {
    struct scm_device *scmdev = to_scm_dev(dev);
    struct scm_driver *scmdrv = to_scm_drv(dev.driver);
    if (scmdrv.remove)
    scmdrv.remove(scmdev);
    }
#[no_mangle]
unsafe extern "C" fn scmdev_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int scmdev_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    return add_uevent_var(env, "MODALIAS=scm:scmdev");
    }
    static const struct bus_type scm_bus_type = {
    .name  = "scm",
    .probe = scmdev_probe,
    .remove = scmdev_remove,
    .uevent = scmdev_uevent,
    };
//
// scm_driver_register() - register a scm driver
// @scmdrv: driver to be registered
//
#[no_mangle]
pub unsafe extern "C" fn scm_driver_register(scmdrv: *mut scm_driver) -> c_int {
    int scm_driver_register(struct scm_driver *scmdrv)
    {
    struct device_driver *drv = &scmdrv.drv;
    drv.bus = &scm_bus_type;
    return driver_register(drv);
    }
    EXPORT_SYMBOL_GPL(scm_driver_register);
//
// scm_driver_unregister() - deregister a scm driver
// @scmdrv: driver to be deregistered
//
#[no_mangle]
pub unsafe extern "C" fn scm_driver_unregister(scmdrv: *mut scm_driver) {
    void scm_driver_unregister(struct scm_driver *scmdrv)
    {
    driver_unregister(&scmdrv.drv);
    }
    EXPORT_SYMBOL_GPL(scm_driver_unregister);
#[no_mangle]
pub unsafe extern "C" fn scm_irq_handler(aob: *mut aob, error: blk_status_t) {
    void scm_irq_handler(struct aob *aob, blk_status_t error)
    {
    struct aob_rq_header *aobrq = (void *) aob.request.data;
    struct scm_device *scmdev = aobrq.scmdev;
    struct scm_driver *scmdrv = to_scm_drv(scmdev.dev.driver);
    scmdrv.handler(scmdev, aobrq.data, error);
    }
    EXPORT_SYMBOL_GPL(scm_irq_handler);

    static ssize_t show_##name(struct device *dev,				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct scm_device *scmdev = to_scm_dev(dev);			\
    int ret;							\
    \
    device_lock(dev);						\
    ret = sysfs_emit(buf, "%u\n", scmdev.attrs.name);		\
    device_unlock(dev);						\
    \
    return ret;							\
    }									\
    static DEVICE_ATTR(name, S_IRUGO, show_##name, core::ptr::null_mut());
    scm_attr(persistence);
    scm_attr(oper_state);
    scm_attr(data_state);
    scm_attr(rank);
    scm_attr(release);
    scm_attr(res_id);
    static struct attribute *scmdev_attrs[] = {
    &dev_attr_persistence.attr,
    &dev_attr_oper_state.attr,
    &dev_attr_data_state.attr,
    &dev_attr_rank.attr,
    &dev_attr_release.attr,
    &dev_attr_res_id.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group scmdev_attr_group = {
    .attrs = scmdev_attrs,
    };
    static const struct attribute_group *scmdev_attr_groups[] = {
    &scmdev_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn scmdev_release(dev: *mut device) {
    static void scmdev_release(struct device *dev)
    {
    struct scm_device *scmdev = to_scm_dev(dev);
    kfree(scmdev);
    }
    static void scmdev_setup(struct scm_device *scmdev, struct sale *sale,
    unsigned int size, unsigned int max_blk_count)
    {
    dev_set_name(&scmdev.dev, "%016llx", (unsigned long long) sale.sa);
    scmdev.nr_max_block = max_blk_count;
    scmdev.address = sale.sa;
    scmdev.size = 1UL << size;
    scmdev.attrs.rank = sale.rank;
    scmdev.attrs.persistence = sale.p;
    scmdev.attrs.oper_state = sale.op_state;
    scmdev.attrs.data_state = sale.data_state;
    scmdev.attrs.rank = sale.rank;
    scmdev.attrs.release = sale.r;
    scmdev.attrs.res_id = sale.rid;
    scmdev.dev.parent = scm_root;
    scmdev.dev.bus = &scm_bus_type;
    scmdev.dev.release = scmdev_release;
    scmdev.dev.groups = scmdev_attr_groups;
    }
//
// Check for state-changes, notify the driver and userspace.
//
#[no_mangle]
unsafe extern "C" fn scmdev_update(scmdev: *mut scm_device, sale: *mut sale) {
    static void scmdev_update(struct scm_device *scmdev, struct sale *sale)
    {
    struct scm_driver *scmdrv;
    bool changed;
    device_lock(&scmdev.dev);
    changed = scmdev.attrs.rank != sale.rank ||
    scmdev.attrs.oper_state != sale.op_state;
    scmdev.attrs.rank = sale.rank;
    scmdev.attrs.oper_state = sale.op_state;
    if (!scmdev.dev.driver)
    goto out;
    scmdrv = to_scm_drv(scmdev.dev.driver);
    if (changed && scmdrv.notify)
    scmdrv.notify(scmdev, SCM_CHANGE);
    out:
    device_unlock(&scmdev.dev);
    if (changed)
    kobject_uevent(&scmdev.dev.kobj, KOBJ_CHANGE);
    }
#[no_mangle]
unsafe extern "C" fn check_address(dev: *mut device, data: *const c_void) -> c_int {
    static int check_address(struct device *dev, const void *data)
    {
    struct scm_device *scmdev = to_scm_dev(dev);
    const struct sale *sale = data;
    return scmdev.address == sale.sa;
    }
    static struct scm_device *scmdev_find(struct sale *sale)
    {
    struct device *dev;
    dev = bus_find_device(&scm_bus_type, core::ptr::null_mut(), sale, check_address);
    return dev ? to_scm_dev(dev) : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn scm_add(scm_info: *mut chsc_scm_info, num: usize) -> c_int {
    static int scm_add(struct chsc_scm_info *scm_info, size_t num)
    {
    struct sale *sale, *scmal = scm_info.scmal;
    struct scm_device *scmdev;
    int ret;
    for (sale = scmal; sale < scmal + num; sale++) {
    scmdev = scmdev_find(sale);
    if (scmdev) {
    scmdev_update(scmdev, sale);
// Release reference from scm_find().
    put_device(&scmdev.dev);
    continue;
    }
    scmdev = kzalloc_obj(*scmdev);
    if (!scmdev)
    return -ENODEV;
    scmdev_setup(scmdev, sale, scm_info.is, scm_info.mbc);
    ret = device_register(&scmdev.dev);
    if (ret) {
// Release reference from device_initialize().
    put_device(&scmdev.dev);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scm_update_information() -> c_int {
    int scm_update_information(void)
    {
    struct chsc_scm_info *scm_info;
    let mut token: u64 = 0;
    size_t num;
    int ret;
    scm_info = (void *)__get_free_page(GFP_KERNEL | GFP_DMA);
    if (!scm_info)
    return -ENOMEM;
    do {
    ret = chsc_scm_info(scm_info, token);
    if (ret)
    break;
    num = (scm_info.response.length -
    (offsetof(struct chsc_scm_info, scmal) -
    offsetof(struct chsc_scm_info, response))
    ) / sizeof(struct sale);
    ret = scm_add(scm_info, num);
    if (ret)
    break;
    token = scm_info.restok;
    } while (token);
    free_page((unsigned long)scm_info);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scm_dev_avail(dev: *mut device, unused: *mut c_void) -> c_int {
    static int scm_dev_avail(struct device *dev, void *unused)
    {
    struct scm_driver *scmdrv = to_scm_drv(dev.driver);
    struct scm_device *scmdev = to_scm_dev(dev);
    if (dev.driver && scmdrv.notify)
    scmdrv.notify(scmdev, SCM_AVAIL);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scm_process_availability_information() -> c_int {
    int scm_process_availability_information(void)
    {
    return bus_for_each_dev(&scm_bus_type, core::ptr::null_mut(), core::ptr::null_mut(), scm_dev_avail);
    }
#[no_mangle]
unsafe extern "C" fn scm_init() -> int __init {
    static int __init scm_init(void)
    {
    int ret;
    ret = bus_register(&scm_bus_type);
    if (ret)
    return ret;
    scm_root = root_device_register("scm");
    if (IS_ERR(scm_root)) {
    bus_unregister(&scm_bus_type);
    return PTR_ERR(scm_root);
    }
    scm_update_information();
    return 0;
    }
    subsys_initcall_sync(scm_init);
