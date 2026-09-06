//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/virtual_ncidev.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Virtual NCI device simulation driver
//
// Copyright (C) 2020 Samsung Electronics
// Bongsu Jeon <bongsu.jeon@samsung.com>
//

pub const IOCTL_GET_NCIDEV_IDX: c_int = 0;

    NFC_PROTO_MIFARE_MASK | \
    NFC_PROTO_FELICA_MASK | \
    NFC_PROTO_ISO14443_MASK | \
    NFC_PROTO_ISO14443_B_MASK | \
    NFC_PROTO_ISO15693_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtual_nci_dev {
    pub ndev: *mut nci_dev,
    pub mtx: mutex,
    pub send_buff: *mut sk_buff,
    pub wq: wait_queue_head,
    pub running: bool,
}

#[no_mangle]
unsafe extern "C" fn virtual_nci_open(ndev: *mut nci_dev) -> c_int {
    static int virtual_nci_open(struct nci_dev *ndev)
    {
    struct virtual_nci_dev *vdev = nci_get_drvdata(ndev);
    vdev.running = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtual_nci_close(ndev: *mut nci_dev) -> c_int {
    static int virtual_nci_close(struct nci_dev *ndev)
    {
    struct virtual_nci_dev *vdev = nci_get_drvdata(ndev);
    mutex_lock(&vdev.mtx);
    kfree_skb(vdev.send_buff);
    vdev.send_buff = core::ptr::null_mut();
    vdev.running = false;
    mutex_unlock(&vdev.mtx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtual_nci_send(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int {
    static int virtual_nci_send(struct nci_dev *ndev, struct sk_buff *skb)
    {
    struct virtual_nci_dev *vdev = nci_get_drvdata(ndev);
    mutex_lock(&vdev.mtx);
    if (vdev.send_buff || !vdev.running) {
    mutex_unlock(&vdev.mtx);
    kfree_skb(skb);
    return -1;
    }
    vdev.send_buff = skb_copy(skb, GFP_KERNEL);
    if (!vdev.send_buff) {
    mutex_unlock(&vdev.mtx);
    kfree_skb(skb);
    return -1;
    }
    mutex_unlock(&vdev.mtx);
    wake_up_interruptible(&vdev.wq);
    consume_skb(skb);
    return 0;
    }
    static const struct nci_ops virtual_nci_ops = {
    .open = virtual_nci_open,
    .close = virtual_nci_close,
    .send = virtual_nci_send
    };
    static ssize_t virtual_ncidev_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct virtual_nci_dev *vdev = file.private_data;
    size_t actual_len;
    mutex_lock(&vdev.mtx);
    while (!vdev.send_buff) {
    mutex_unlock(&vdev.mtx);
    if (wait_event_interruptible(vdev.wq, vdev.send_buff))
    return -EFAULT;
    mutex_lock(&vdev.mtx);
    }
    actual_len = min_t(size_t, count, vdev.send_buff.len);
    if (copy_to_user(buf, vdev.send_buff.data, actual_len)) {
    mutex_unlock(&vdev.mtx);
    return -EFAULT;
    }
    skb_pull(vdev.send_buff, actual_len);
    if (vdev.send_buff.len == 0) {
    consume_skb(vdev.send_buff);
    vdev.send_buff = core::ptr::null_mut();
    }
    mutex_unlock(&vdev.mtx);
    return actual_len;
    }
    static ssize_t virtual_ncidev_write(struct file *file,
    const char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct virtual_nci_dev *vdev = file.private_data;
    struct sk_buff *skb;
    skb = alloc_skb(count, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    if (copy_from_user(skb_put(skb, count), buf, count)) {
    kfree_skb(skb);
    return -EFAULT;
    }
    nci_recv_frame(vdev.ndev, skb);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn virtual_ncidev_open(inode: *mut inode, file: *mut file) -> c_int {
    static int virtual_ncidev_open(struct inode *inode, struct file *file)
    {
    let mut ret: c_int = 0;
    struct virtual_nci_dev *vdev;
    vdev = kzalloc_obj(*vdev);
    if (!vdev)
    return -ENOMEM;
    vdev.ndev = nci_allocate_device(&virtual_nci_ops,
    VIRTUAL_NFC_PROTOCOLS, 0, 0);
    if (!vdev.ndev) {
    kfree(vdev);
    return -ENOMEM;
    }
    mutex_init(&vdev.mtx);
    init_waitqueue_head(&vdev.wq);
    file.private_data = vdev;
    nci_set_drvdata(vdev.ndev, vdev);
    ret = nci_register_device(vdev.ndev);
    if (ret < 0) {
    nci_free_device(vdev.ndev);
    mutex_destroy(&vdev.mtx);
    kfree(vdev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtual_ncidev_close(inode: *mut inode, file: *mut file) -> c_int {
    static int virtual_ncidev_close(struct inode *inode, struct file *file)
    {
    struct virtual_nci_dev *vdev = file.private_data;
    nci_unregister_device(vdev.ndev);
    nci_free_device(vdev.ndev);
    mutex_destroy(&vdev.mtx);
    kfree(vdev);
    return 0;
    }
    static long virtual_ncidev_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    struct virtual_nci_dev *vdev = file.private_data;
    const struct nfc_dev *nfc_dev = vdev.ndev.nfc_dev;
    void __user *p = (void __user *)arg;
    if (cmd != IOCTL_GET_NCIDEV_IDX)
    return -ENOTTY;
    if (copy_to_user(p, &nfc_dev.idx, sizeof(nfc_dev.idx)))
    return -EFAULT;
    return 0;
    }
    static const struct file_operations virtual_ncidev_fops = {
    .owner = THIS_MODULE,
    .read = virtual_ncidev_read,
    .write = virtual_ncidev_write,
    .open = virtual_ncidev_open,
    .release = virtual_ncidev_close,
    .unlocked_ioctl = virtual_ncidev_ioctl
    };
    static struct miscdevice miscdev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "virtual_nci",
    .fops = &virtual_ncidev_fops,
    .mode = 0600,
    };
    module_misc_device(miscdev);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Virtual NCI device simulation driver");
    MODULE_AUTHOR("Bongsu Jeon <bongsu.jeon@samsung.com>");
