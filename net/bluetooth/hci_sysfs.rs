//! Automatically rewritten from C to Rust
//! Source: net/bluetooth/hci_sysfs.c
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
// Bluetooth HCI driver model support.

    static const struct class bt_class = {
    .name = "bluetooth",
    };
#[no_mangle]
unsafe extern "C" fn bt_link_release(dev: *mut device) {
    static void bt_link_release(struct device *dev)
    {
    struct hci_conn *conn = to_hci_conn(dev);
    kfree(conn);
    }
    static const struct device_type bt_link = {
    .name    = "link",
    .release = bt_link_release,
    };
#[no_mangle]
pub unsafe extern "C" fn hci_conn_init_sysfs(conn: *mut hci_conn) {
    void hci_conn_init_sysfs(struct hci_conn *conn)
    {
    struct hci_dev *hdev = conn.hdev;
    bt_dev_dbg(hdev, "conn %p", conn);
    conn.dev.type = &bt_link;
    conn.dev.class = &bt_class;
    conn.dev.parent = &hdev.dev;
    device_initialize(&conn.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn hci_conn_add_sysfs(conn: *mut hci_conn) {
    void hci_conn_add_sysfs(struct hci_conn *conn)
    {
    struct hci_dev *hdev = conn.hdev;
    bt_dev_dbg(hdev, "conn %p", conn);
    if (device_is_registered(&conn.dev))
    return;
    dev_set_name(&conn.dev, "%s:%d", hdev.name, conn.handle);
    if (device_add(&conn.dev) < 0)
    bt_dev_err(hdev, "failed to register connection device");
    }
#[no_mangle]
pub unsafe extern "C" fn hci_conn_del_sysfs(conn: *mut hci_conn) {
    void hci_conn_del_sysfs(struct hci_conn *conn)
    {
    struct hci_dev *hdev = conn.hdev;
    bt_dev_dbg(hdev, "conn %p", conn);
    if (!device_is_registered(&conn.dev)) {
// If device_add() has *not* succeeded, use *only* put_device()
// to drop the reference count.
//
    put_device(&conn.dev);
    return;
    }
// If there are devices using the connection as parent reset it to NULL
// before unregistering the device.
//
    while (1) {
    struct device *dev;
    dev = device_find_any_child(&conn.dev);
    if (!dev)
    break;
    device_move(dev, core::ptr::null_mut(), DPM_ORDER_DEV_LAST);
    put_device(dev);
    }
    device_unregister(&conn.dev);
    }
#[no_mangle]
unsafe extern "C" fn bt_host_release(dev: *mut device) {
    static void bt_host_release(struct device *dev)
    {
    struct hci_dev *hdev = to_hci_dev(dev);
    if (hci_dev_test_flag(hdev, HCI_UNREGISTER)) {
    hci_release_dev(hdev);
    } else {
    cleanup_srcu_struct(&hdev.srcu);
    kfree(hdev);
    }
    module_put(THIS_MODULE);
    }
    static ssize_t reset_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct hci_dev *hdev = to_hci_dev(dev);
    if (hdev.reset)
    hdev.reset(hdev);
    return count;
    }
    static DEVICE_ATTR_WO(reset);
    static struct attribute *bt_host_attrs[] = {
    &dev_attr_reset.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(bt_host);
    static const struct device_type bt_host = {
    .name    = "host",
    .release = bt_host_release,
    .groups = bt_host_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn hci_init_sysfs(hdev: *mut hci_dev) {
    void hci_init_sysfs(struct hci_dev *hdev)
    {
    struct device *dev = &hdev.dev;
    dev.type = &bt_host;
    dev.class = &bt_class;
    __module_get(THIS_MODULE);
    device_initialize(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn bt_sysfs_init() -> int __init {
    int __init bt_sysfs_init(void)
    {
    return class_register(&bt_class);
    }
#[no_mangle]
pub unsafe extern "C" fn bt_sysfs_cleanup() {
    void bt_sysfs_cleanup(void)
    {
    class_unregister(&bt_class);
    }
