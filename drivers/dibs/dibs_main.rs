//! Automatically rewritten from C to Rust
//! Source: drivers/dibs/dibs_main.c
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
// DIBS - Direct Internal Buffer Sharing
//
// Implementation of the DIBS class module
//
// Copyright IBM Corp. 2025
//

    MODULE_DESCRIPTION("Direct Internal Buffer Sharing class");
    MODULE_LICENSE("GPL");
    static const struct class dibs_class = {
    .name		= "dibs",
    };
// use an array rather a list for fast mapping:
    static struct dibs_client *clients[MAX_DIBS_CLIENTS];
    static u8 max_client;
    static DEFINE_MUTEX(clients_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibs_dev_list {
    pub list: list_head,
    pub /: *mut *mut mutex mutex; / protects dibs device list,
}

    static struct dibs_dev_list dibs_dev_list = {
    .list = LIST_HEAD_INIT(dibs_dev_list.list),
    .mutex = __MUTEX_INITIALIZER(dibs_dev_list.mutex),
    };
    static void dibs_setup_forwarding(struct dibs_client *client,
    struct dibs_dev *dibs)
    {
    unsigned long flags;
    spin_lock_irqsave(&dibs.lock, flags);
    dibs.subs[client.id] = client;
    spin_unlock_irqrestore(&dibs.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn dibs_register_client(client: *mut dibs_client) -> c_int {
    int dibs_register_client(struct dibs_client *client)
    {
    struct dibs_dev *dibs;
    int i, rc = -ENOSPC;
    mutex_lock(&dibs_dev_list.mutex);
    mutex_lock(&clients_lock);
    for (i = 0; i < MAX_DIBS_CLIENTS; ++i) {
    if (!clients[i]) {
    clients[i] = client;
    client.id = i;
    if (i == max_client)
    max_client++;
    rc = 0;
    break;
    }
    }
    mutex_unlock(&clients_lock);
    if (i < MAX_DIBS_CLIENTS) {
// initialize with all devices that we got so far
    list_for_each_entry(dibs, &dibs_dev_list.list, list) {
    dibs.priv[i] = core::ptr::null_mut();
    client.ops.add_dev(dibs);
    dibs_setup_forwarding(client, dibs);
    }
    }
    mutex_unlock(&dibs_dev_list.mutex);
    return rc;
    }
    EXPORT_SYMBOL_GPL(dibs_register_client);
#[no_mangle]
pub unsafe extern "C" fn dibs_unregister_client(client: *mut dibs_client) -> c_int {
    int dibs_unregister_client(struct dibs_client *client)
    {
    struct dibs_dev *dibs;
    unsigned long flags;
    int max_dmbs;
    let mut rc: c_int = 0;
    mutex_lock(&dibs_dev_list.mutex);
    list_for_each_entry(dibs, &dibs_dev_list.list, list) {
    spin_lock_irqsave(&dibs.lock, flags);
    max_dmbs = dibs.ops.max_dmbs();
    for (int i = 0; i < max_dmbs; ++i) {
    if (dibs.dmb_clientid_arr[i] == client.id) {
    WARN(1, "%s: attempt to unregister '%s' with registered dmb(s)\n",
    __func__, client.name);
    rc = -EBUSY;
    goto err_reg_dmb;
    }
    }
// Stop forwarding IRQs and events
    dibs.subs[client.id] = core::ptr::null_mut();
    spin_unlock_irqrestore(&dibs.lock, flags);
    clients[client.id].ops.del_dev(dibs);
    dibs.priv[client.id] = core::ptr::null_mut();
    }
    mutex_lock(&clients_lock);
    clients[client.id] = core::ptr::null_mut();
    if (client.id + 1 == max_client)
    max_client--;
    mutex_unlock(&clients_lock);
    mutex_unlock(&dibs_dev_list.mutex);
    return rc;
    err_reg_dmb:
    spin_unlock_irqrestore(&dibs.lock, flags);
    mutex_unlock(&dibs_dev_list.mutex);
    return rc;
    }
    EXPORT_SYMBOL_GPL(dibs_unregister_client);
#[no_mangle]
unsafe extern "C" fn dibs_dev_release(dev: *mut device) {
    static void dibs_dev_release(struct device *dev)
    {
    struct dibs_dev *dibs;
    dibs = container_of(dev, struct dibs_dev, dev);
    kfree(dibs.dmb_clientid_arr);
    kfree(dibs);
    }
    struct dibs_dev *dibs_dev_alloc(void)
    {
    struct dibs_dev *dibs;
    dibs = kzalloc_obj(*dibs);
    if (!dibs)
    return dibs;
    spin_lock_init(&dibs.lock);
    dibs.dev.release = dibs_dev_release;
    dibs.dev.class = &dibs_class;
    device_initialize(&dibs.dev);
    return dibs;
    }
    EXPORT_SYMBOL_GPL(dibs_dev_alloc);
    static ssize_t gid_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct dibs_dev *dibs;
    dibs = container_of(dev, struct dibs_dev, dev);
    return sysfs_emit(buf, "%pUb\n", &dibs.gid);
    }
    static DEVICE_ATTR_RO(gid);
    static ssize_t fabric_id_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct dibs_dev *dibs;
    u16 fabric_id;
    dibs = container_of(dev, struct dibs_dev, dev);
    fabric_id = dibs.ops.get_fabric_id(dibs);
    return sysfs_emit(buf, "0x%04x\n", fabric_id);
    }
    static DEVICE_ATTR_RO(fabric_id);
    static struct attribute *dibs_dev_attrs[] = {
    &dev_attr_gid.attr,
    &dev_attr_fabric_id.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group dibs_dev_attr_group = {
    .attrs = dibs_dev_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn dibs_dev_add(dibs: *mut dibs_dev) -> c_int {
    int dibs_dev_add(struct dibs_dev *dibs)
    {
    int max_dmbs;
    int i, ret;
    max_dmbs = dibs.ops.max_dmbs();
    dibs.dmb_clientid_arr = kzalloc(max_dmbs, GFP_KERNEL);
    if (!dibs.dmb_clientid_arr)
    return -ENOMEM;
    memset(dibs.dmb_clientid_arr, NO_DIBS_CLIENT, max_dmbs);
    ret = device_add(&dibs.dev);
    if (ret)
    return ret;
    ret = sysfs_create_group(&dibs.dev.kobj, &dibs_dev_attr_group);
    if (ret) {
    dev_err(&dibs.dev, "sysfs_create_group failed for dibs_dev\n");
    device_del(&dibs.dev);
    return ret;
    }
    mutex_lock(&dibs_dev_list.mutex);
    mutex_lock(&clients_lock);
    for (i = 0; i < max_client; ++i) {
    if (clients[i]) {
    clients[i].ops.add_dev(dibs);
    dibs_setup_forwarding(clients[i], dibs);
    }
    }
    mutex_unlock(&clients_lock);
    list_add(&dibs.list, &dibs_dev_list.list);
    mutex_unlock(&dibs_dev_list.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(dibs_dev_add);
#[no_mangle]
pub unsafe extern "C" fn dibs_dev_del(dibs: *mut dibs_dev) {
    void dibs_dev_del(struct dibs_dev *dibs)
    {
    unsigned long flags;
    int i;
    sysfs_remove_group(&dibs.dev.kobj, &dibs_dev_attr_group);
    spin_lock_irqsave(&dibs.lock, flags);
    for (i = 0; i < MAX_DIBS_CLIENTS; ++i)
    dibs.subs[i] = core::ptr::null_mut();
    spin_unlock_irqrestore(&dibs.lock, flags);
    mutex_lock(&dibs_dev_list.mutex);
    mutex_lock(&clients_lock);
    for (i = 0; i < max_client; ++i) {
    if (clients[i])
    clients[i].ops.del_dev(dibs);
    }
    mutex_unlock(&clients_lock);
    list_del_init(&dibs.list);
    mutex_unlock(&dibs_dev_list.mutex);
    device_del(&dibs.dev);
    }
    EXPORT_SYMBOL_GPL(dibs_dev_del);
#[no_mangle]
unsafe extern "C" fn dibs_init() -> int __init {
    static int __init dibs_init(void)
    {
    int rc;
    rc = class_register(&dibs_class);
    if (rc)
    return rc;
    rc = dibs_loopback_init();
    if (rc)
    pr_err("%s fails with %d\n", __func__, rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn dibs_exit() -> void __exit {
    static void __exit dibs_exit(void)
    {
    dibs_loopback_exit();
    class_unregister(&dibs_class);
    }
    subsys_initcall(dibs_init);
    module_exit(dibs_exit);
