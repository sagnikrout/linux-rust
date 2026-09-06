//! Automatically rewritten from C to Rust
//! Source: drivers/s390/crypto/zcrypt_card.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2001, 2012
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
// Cornelia Huck <cornelia.huck@de.ibm.com>
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
// Ralph Wuerthner <rwuerthn@de.ibm.com>
// MSGTYPE restruct:		  Holger Dengler <hd@linux.vnet.ibm.com>
//

//
// Device attributes common for all crypto card devices.
//
    static ssize_t type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct zcrypt_card *zc = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%s\n", zc.type_string);
    }
    static DEVICE_ATTR_RO(type);
    static ssize_t online_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct zcrypt_card *zc = dev_get_drvdata(dev);
    struct ap_card *ac = to_ap_card(dev);
    let mut online: c_int = ac.config && !ac.chkstop && zc.online ? 1 : 0;
    return sysfs_emit(buf, "%d\n", online);
    }
    static ssize_t online_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zcrypt_card *zc = dev_get_drvdata(dev);
    struct ap_card *ac = to_ap_card(dev);
    struct zcrypt_queue *zq;
    int online, id, i = 0, maxzqs = 0;
    struct zcrypt_queue **zq_uelist = core::ptr::null_mut();
    if (sscanf(buf, "%d\n", &online) != 1 || online < 0 || online > 1)
    return -EINVAL;
    if (online && (!ac.config || ac.chkstop))
    return -ENODEV;
    zc.online = online;
    id = zc.card.id;
    ZCRYPT_DBF_INFO("%s card=%02x online=%d\n", __func__, id, online);
    ap_send_online_uevent(&ac.ap_dev, online);
    spin_lock(&zcrypt_list_lock);
//
// As we are in atomic context here, directly sending uevents
// does not work. So collect the zqueues in a dynamic array
// and process them after zcrypt_list_lock release. As we get/put
// the zqueue objects, we make sure they exist after lock release.
//
    list_for_each_entry(zq, &zc.zqueues, list)
    maxzqs++;
    if (maxzqs > 0)
    zq_uelist = kzalloc_objs(*zq_uelist, maxzqs + 1, GFP_ATOMIC);
    list_for_each_entry(zq, &zc.zqueues, list)
    if (zcrypt_queue_force_online(zq, online))
    if (zq_uelist) {
    zcrypt_queue_get(zq);
    zq_uelist[i++] = zq;
    }
    spin_unlock(&zcrypt_list_lock);
    if (zq_uelist) {
    for (i = 0; zq_uelist[i]; i++) {
    zq = zq_uelist[i];
    ap_send_online_uevent(&zq.queue.ap_dev, online);
    zcrypt_queue_put(zq);
    }
    kfree(zq_uelist);
    }
    return count;
    }
    static DEVICE_ATTR_RW(online);
    static ssize_t load_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct zcrypt_card *zc = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", atomic_read(&zc.load));
    }
    static DEVICE_ATTR_RO(load);
    static struct attribute *zcrypt_card_attrs[] = {
    &dev_attr_type.attr,
    &dev_attr_online.attr,
    &dev_attr_load.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group zcrypt_card_attr_group = {
    .attrs = zcrypt_card_attrs,
    };
    struct zcrypt_card *zcrypt_card_alloc(void)
    {
    struct zcrypt_card *zc;
    zc = kzalloc_obj(*zc);
    if (!zc)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&zc.list);
    INIT_LIST_HEAD(&zc.zqueues);
    kref_init(&zc.refcount);
    return zc;
    }
    EXPORT_SYMBOL(zcrypt_card_alloc);
#[no_mangle]
pub unsafe extern "C" fn zcrypt_card_free(zc: *mut zcrypt_card) {
    void zcrypt_card_free(struct zcrypt_card *zc)
    {
    kfree(zc);
    }
    EXPORT_SYMBOL(zcrypt_card_free);
#[no_mangle]
unsafe extern "C" fn zcrypt_card_release(kref: *mut kref) {
    static void zcrypt_card_release(struct kref *kref)
    {
    struct zcrypt_card *zdev =
    container_of(kref, struct zcrypt_card, refcount);
    zcrypt_card_free(zdev);
    }
#[no_mangle]
pub unsafe extern "C" fn zcrypt_card_get(zc: *mut zcrypt_card) {
    void zcrypt_card_get(struct zcrypt_card *zc)
    {
    kref_get(&zc.refcount);
    }
    EXPORT_SYMBOL(zcrypt_card_get);
#[no_mangle]
pub unsafe extern "C" fn zcrypt_card_put(zc: *mut zcrypt_card) -> c_int {
    int zcrypt_card_put(struct zcrypt_card *zc)
    {
    return kref_put(&zc.refcount, zcrypt_card_release);
    }
    EXPORT_SYMBOL(zcrypt_card_put);
//
// zcrypt_card_register() - Register a crypto card device.
// @zc: Pointer to a crypto card device
//
// Register a crypto card device. Returns 0 if successful.
//
#[no_mangle]
pub unsafe extern "C" fn zcrypt_card_register(zc: *mut zcrypt_card) -> c_int {
    int zcrypt_card_register(struct zcrypt_card *zc)
    {
    int rc;
    spin_lock(&zcrypt_list_lock);
    list_add_tail(&zc.list, &zcrypt_card_list);
    spin_unlock(&zcrypt_list_lock);
    zc.online = 1;
    ZCRYPT_DBF_INFO("%s card=%02x register online=1\n",
    __func__, zc.card.id);
    rc = sysfs_create_group(&zc.card.ap_dev.device.kobj,
    &zcrypt_card_attr_group);
    if (rc) {
    spin_lock(&zcrypt_list_lock);
    list_del_init(&zc.list);
    spin_unlock(&zcrypt_list_lock);
    }
    return rc;
    }
    EXPORT_SYMBOL(zcrypt_card_register);
//
// zcrypt_card_unregister(): Unregister a crypto card device.
// @zc: Pointer to crypto card device
//
// Unregister a crypto card device.
//
#[no_mangle]
pub unsafe extern "C" fn zcrypt_card_unregister(zc: *mut zcrypt_card) {
    void zcrypt_card_unregister(struct zcrypt_card *zc)
    {
    ZCRYPT_DBF_INFO("%s card=%02x unregister\n",
    __func__, zc.card.id);
    spin_lock(&zcrypt_list_lock);
    list_del_init(&zc.list);
    spin_unlock(&zcrypt_list_lock);
    sysfs_remove_group(&zc.card.ap_dev.device.kobj,
    &zcrypt_card_attr_group);
    zcrypt_card_put(zc);
    }
    EXPORT_SYMBOL(zcrypt_card_unregister);
