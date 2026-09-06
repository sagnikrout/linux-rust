//! Automatically rewritten from C to Rust
//! Source: net/qrtr/tun.c
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
// Copyright (c) 2018, Linaro Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qrtr_tun {
    pub ep: qrtr_endpoint,
    pub queue: sk_buff_head,
    pub readq: wait_queue_head_t,
}

#[no_mangle]
unsafe extern "C" fn qrtr_tun_send(ep: *mut qrtr_endpoint, skb: *mut sk_buff) -> c_int {
    static int qrtr_tun_send(struct qrtr_endpoint *ep, struct sk_buff *skb)
    {
    struct qrtr_tun *tun = container_of(ep, struct qrtr_tun, ep);
    skb_queue_tail(&tun.queue, skb);
// wake up any blocking processes, waiting for new data
    wake_up_interruptible(&tun.readq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int qrtr_tun_open(struct inode *inode, struct file *filp)
    {
    struct qrtr_tun *tun;
    int ret;
    tun = kzalloc_obj(*tun);
    if (!tun)
    return -ENOMEM;
    skb_queue_head_init(&tun.queue);
    init_waitqueue_head(&tun.readq);
    tun.ep.xmit = qrtr_tun_send;
    filp.private_data = tun;
    ret = qrtr_endpoint_register(&tun.ep, QRTR_EP_NID_AUTO);
    if (ret)
    goto out;
    return 0;
    out:
    filp.private_data = core::ptr::null_mut();
    kfree(tun);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    static ssize_t qrtr_tun_read_iter(struct kiocb *iocb, struct iov_iter *to)
    {
    struct file *filp = iocb.ki_filp;
    struct qrtr_tun *tun = filp.private_data;
    struct sk_buff *skb;
    int count;
    while (!(skb = skb_dequeue(&tun.queue))) {
    if (filp.f_flags & O_NONBLOCK)
    return -EAGAIN;
// Wait until we get data or the endpoint goes away
    if (wait_event_interruptible(tun.readq,
    !skb_queue_empty(&tun.queue)))
    return -ERESTARTSYS;
    }
    count = min_t(size_t, iov_iter_count(to), skb.len);
    if (copy_to_iter(skb.data, count, to) != count)
    count = -EFAULT;
    kfree_skb(skb);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    static ssize_t qrtr_tun_write_iter(struct kiocb *iocb, struct iov_iter *from)
    {
    struct file *filp = iocb.ki_filp;
    struct qrtr_tun *tun = filp.private_data;
    let mut len: usize = iov_iter_count(from);
    ssize_t ret;
    void *kbuf;
    if (!len)
    return -EINVAL;
    if (len > KMALLOC_MAX_SIZE)
    return -ENOMEM;
    kbuf = kzalloc(len, GFP_KERNEL);
    if (!kbuf)
    return -ENOMEM;
    if (!copy_from_iter_full(kbuf, len, from)) {
    kfree(kbuf);
    return -EFAULT;
    }
    ret = qrtr_endpoint_post(&tun.ep, kbuf, len);
    kfree(kbuf);
    return ret < 0 ? ret : len;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_poll(filp: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t qrtr_tun_poll(struct file *filp, poll_table *wait)
    {
    struct qrtr_tun *tun = filp.private_data;
    let mut mask: __poll_t = 0;
    poll_wait(filp, &tun.readq, wait);
    if (!skb_queue_empty(&tun.queue))
    mask |= EPOLLIN | EPOLLRDNORM;
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int qrtr_tun_release(struct inode *inode, struct file *filp)
    {
    struct qrtr_tun *tun = filp.private_data;
    qrtr_endpoint_unregister(&tun.ep);
// Discard all SKBs
    skb_queue_purge(&tun.queue);
    kfree(tun);
    return 0;
    }
    static const struct file_operations qrtr_tun_ops = {
    .owner = THIS_MODULE,
    .open = qrtr_tun_open,
    .poll = qrtr_tun_poll,
    .read_iter = qrtr_tun_read_iter,
    .write_iter = qrtr_tun_write_iter,
    .release = qrtr_tun_release,
    };
    static struct miscdevice qrtr_tun_miscdev = {
    MISC_DYNAMIC_MINOR,
    "qrtr-tun",
    &qrtr_tun_ops,
    };
#[no_mangle]
unsafe extern "C" fn qrtr_tun_init() -> int __init {
    static int __init qrtr_tun_init(void)
    {
    int ret;
    ret = misc_register(&qrtr_tun_miscdev);
    if (ret)
    pr_err("failed to register Qualcomm IPC Router tun device\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qrtr_tun_exit() -> void __exit {
    static void __exit qrtr_tun_exit(void)
    {
    misc_deregister(&qrtr_tun_miscdev);
    }
    module_init(qrtr_tun_init);
    module_exit(qrtr_tun_exit);
    MODULE_DESCRIPTION("Qualcomm IPC Router TUN device");
    MODULE_LICENSE("GPL v2");
