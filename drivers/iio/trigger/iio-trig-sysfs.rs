//! Automatically rewritten from C to Rust
//! Source: drivers/iio/trigger/iio-trig-sysfs.c
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
// Copyright 2011 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_sysfs_trig {
    pub trig: *mut iio_trigger,
    pub work: irq_work,
    pub id: c_int,
    pub l: list_head,
}

    static LIST_HEAD(iio_sysfs_trig_list);
    static DEFINE_MUTEX(iio_sysfs_trig_list_mut);
    static int iio_sysfs_trigger_probe(int id);
    static ssize_t iio_sysfs_trig_add(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t len)
    {
    int ret;
    unsigned long input;
    ret = kstrtoul(buf, 10, &input);
    if (ret)
    return ret;
    ret = iio_sysfs_trigger_probe(input);
    if (ret)
    return ret;
    return len;
    }
    static DEVICE_ATTR(add_trigger, S_IWUSR, core::ptr::null_mut(), &iio_sysfs_trig_add);
    static int iio_sysfs_trigger_remove(int id);
    static ssize_t iio_sysfs_trig_remove(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t len)
    {
    int ret;
    unsigned long input;
    ret = kstrtoul(buf, 10, &input);
    if (ret)
    return ret;
    ret = iio_sysfs_trigger_remove(input);
    if (ret)
    return ret;
    return len;
    }
    static DEVICE_ATTR(remove_trigger, S_IWUSR, core::ptr::null_mut(), &iio_sysfs_trig_remove);
    static struct attribute *iio_sysfs_trig_attrs[] = {
    &dev_attr_add_trigger.attr,
    &dev_attr_remove_trigger.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group iio_sysfs_trig_group = {
    .attrs = iio_sysfs_trig_attrs,
    };
    static const struct attribute_group *iio_sysfs_trig_groups[] = {
    &iio_sysfs_trig_group,
    core::ptr::null_mut()
    };
// Nothing to actually do upon release
#[no_mangle]
unsafe extern "C" fn iio_trigger_sysfs_release(dev: *mut device) {
    static void iio_trigger_sysfs_release(struct device *dev)
    {
    }
    static struct device iio_sysfs_trig_dev = {
    .bus = &iio_bus_type,
    .groups = iio_sysfs_trig_groups,
    .release = &iio_trigger_sysfs_release,
    };
#[no_mangle]
unsafe extern "C" fn iio_sysfs_trigger_work(work: *mut irq_work) {
    static void iio_sysfs_trigger_work(struct irq_work *work)
    {
    struct iio_sysfs_trig *trig = container_of(work, struct iio_sysfs_trig,
    work);
    iio_trigger_poll(trig.trig);
    }
    static ssize_t iio_sysfs_trigger_poll(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct iio_trigger *trig = to_iio_trigger(dev);
    struct iio_sysfs_trig *sysfs_trig = iio_trigger_get_drvdata(trig);
    irq_work_queue(&sysfs_trig.work);
    return count;
    }
    static DEVICE_ATTR(trigger_now, S_IWUSR, core::ptr::null_mut(), iio_sysfs_trigger_poll);
    static struct attribute *iio_sysfs_trigger_attrs[] = {
    &dev_attr_trigger_now.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group iio_sysfs_trigger_attr_group = {
    .attrs = iio_sysfs_trigger_attrs,
    };
    static const struct attribute_group *iio_sysfs_trigger_attr_groups[] = {
    &iio_sysfs_trigger_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn iio_sysfs_trigger_probe(id: c_int) -> c_int {
    static int iio_sysfs_trigger_probe(int id)
    {
    struct iio_sysfs_trig *t;
    int ret;
    let mut foundit: bool = false;
    mutex_lock(&iio_sysfs_trig_list_mut);
    list_for_each_entry(t, &iio_sysfs_trig_list, l)
    if (id == t.id) {
    foundit = true;
    break;
    }
    if (foundit) {
    ret = -EINVAL;
    goto err_unlock;
    }
    t = kmalloc_obj(*t);
    if (t == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err_unlock;
    }
    t.id = id;
    t.trig = iio_trigger_alloc(&iio_sysfs_trig_dev, "sysfstrig%d", id);
    if (!t.trig) {
    ret = -ENOMEM;
    goto err_free_sys_trig;
    }
    t.trig.dev.groups = iio_sysfs_trigger_attr_groups;
    iio_trigger_set_drvdata(t.trig, t);
    t.work = IRQ_WORK_INIT_HARD(iio_sysfs_trigger_work);
    ret = iio_trigger_register(t.trig);
    if (ret)
    goto err_free_trig;
    list_add(&t.l, &iio_sysfs_trig_list);
    __module_get(THIS_MODULE);
    mutex_unlock(&iio_sysfs_trig_list_mut);
    return 0;
    err_free_trig:
    iio_trigger_free(t.trig);
    err_free_sys_trig:
    kfree(t);
    err_unlock:
    mutex_unlock(&iio_sysfs_trig_list_mut);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iio_sysfs_trigger_remove(id: c_int) -> c_int {
    static int iio_sysfs_trigger_remove(int id)
    {
    struct iio_sysfs_trig *t = core::ptr::null_mut(), *iter;
    mutex_lock(&iio_sysfs_trig_list_mut);
    list_for_each_entry(iter, &iio_sysfs_trig_list, l)
    if (id == iter.id) {
    t = iter;
    break;
    }
    if (!t) {
    mutex_unlock(&iio_sysfs_trig_list_mut);
    return -EINVAL;
    }
    iio_trigger_unregister(t.trig);
    irq_work_sync(&t.work);
    iio_trigger_free(t.trig);
    list_del(&t.l);
    kfree(t);
    module_put(THIS_MODULE);
    mutex_unlock(&iio_sysfs_trig_list_mut);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_sysfs_trig_init() -> int __init {
    static int __init iio_sysfs_trig_init(void)
    {
    int ret;
    device_initialize(&iio_sysfs_trig_dev);
    dev_set_name(&iio_sysfs_trig_dev, "iio_sysfs_trigger");
    ret = device_add(&iio_sysfs_trig_dev);
    if (ret)
    put_device(&iio_sysfs_trig_dev);
    return ret;
    }
    module_init(iio_sysfs_trig_init);
#[no_mangle]
unsafe extern "C" fn iio_sysfs_trig_exit() -> void __exit {
    static void __exit iio_sysfs_trig_exit(void)
    {
    device_unregister(&iio_sysfs_trig_dev);
    }
    module_exit(iio_sysfs_trig_exit);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("Sysfs based trigger for the iio subsystem");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:iio-trig-sysfs");
