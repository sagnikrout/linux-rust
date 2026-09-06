//! Automatically rewritten from C to Rust
//! Source: ipc/msgutil.c
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
// linux/ipc/msgutil.c
// Copyright (C) 1999, 2004 Manfred Spraul
//

    DEFINE_SPINLOCK(mq_lock);
//
// The next 2 defines are here bc this is the only file
// compiled when either CONFIG_SYSVIPC and CONFIG_POSIX_MQUEUE
// and not CONFIG_IPC_NS.
//
    struct ipc_namespace init_ipc_ns = {
    .ns = NS_COMMON_INIT(init_ipc_ns),
    .user_ns = &init_user_ns,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg {
    pub next: *mut msg_msgseg,
// the next part of the message follows immediately
}

    static kmem_buckets *msg_buckets __ro_after_init;
#[no_mangle]
unsafe extern "C" fn init_msg_buckets() -> int __init {
    static int __init init_msg_buckets(void)
    {
    msg_buckets = kmem_buckets_create("msg_msg", SLAB_ACCOUNT,
    sizeof(struct msg_msg),
    DATALEN_MSG, core::ptr::null_mut());
    return 0;
    }
    subsys_initcall(init_msg_buckets);
    static struct msg_msg *alloc_msg(size_t len)
    {
    struct msg_msg *msg;
    struct msg_msgseg **pseg;
    size_t alen;
    alen = min(len, DATALEN_MSG);
    msg = kmem_buckets_alloc(msg_buckets, sizeof(*msg) + alen, GFP_KERNEL);
    if (msg == core::ptr::null_mut())
    return core::ptr::null_mut();
    msg.next = core::ptr::null_mut();
    msg.security = core::ptr::null_mut();
    len -= alen;
    pseg = &msg.next;
    while (len > 0) {
    struct msg_msgseg *seg;
    cond_resched();
    alen = min(len, DATALEN_SEG);
    seg = kmalloc(sizeof(*seg) + alen, GFP_KERNEL_ACCOUNT);
    if (seg == core::ptr::null_mut())
    goto out_err;
// pseg = seg;
    seg.next = core::ptr::null_mut();
    pseg = &seg.next;
    len -= alen;
    }
    return msg;
    out_err:
    free_msg(msg);
    return core::ptr::null_mut();
    }
    struct msg_msg *load_msg(const void __user *src, size_t len)
    {
    struct msg_msg *msg;
    struct msg_msgseg *seg;
    let mut err: c_int = -EFAULT;
    size_t alen;
    msg = alloc_msg(len);
    if (msg == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    alen = min(len, DATALEN_MSG);
    if (copy_from_user(msg + 1, src, alen))
    goto out_err;
    for (seg = msg.next; seg != core::ptr::null_mut(); seg = seg.next) {
    len -= alen;
    src = (char __user *)src + alen;
    alen = min(len, DATALEN_SEG);
    if (copy_from_user(seg + 1, src, alen))
    goto out_err;
    }
    err = security_msg_msg_alloc(msg);
    if (err)
    goto out_err;
    return msg;
    out_err:
    free_msg(msg);
    return ERR_PTR(err);
    }

    struct msg_msg *copy_msg(struct msg_msg *src, struct msg_msg *dst)
    {
    struct msg_msgseg *dst_pseg, *src_pseg;
    let mut len: usize = src.m_ts;
    size_t alen;
    if (src.m_ts > dst.m_ts)
    return ERR_PTR(-EINVAL);
    alen = min(len, DATALEN_MSG);
    memcpy(dst + 1, src + 1, alen);
    for (dst_pseg = dst.next, src_pseg = src.next;
    src_pseg != core::ptr::null_mut();
    dst_pseg = dst_pseg.next, src_pseg = src_pseg.next) {
    len -= alen;
    alen = min(len, DATALEN_SEG);
    memcpy(dst_pseg + 1, src_pseg + 1, alen);
    }
    dst.m_type = src.m_type;
    dst.m_ts = src.m_ts;
    return dst;
    }

    struct msg_msg *copy_msg(struct msg_msg *src, struct msg_msg *dst)
    {
    return ERR_PTR(-ENOSYS);
    }

#[no_mangle]
pub unsafe extern "C" fn store_msg(dest: *mut void __user, msg: *mut msg_msg, len: usize) -> c_int {
    int store_msg(void __user *dest, struct msg_msg *msg, size_t len)
    {
    size_t alen;
    struct msg_msgseg *seg;
    alen = min(len, DATALEN_MSG);
    if (copy_to_user(dest, msg + 1, alen))
    return -1;
    for (seg = msg.next; seg != core::ptr::null_mut(); seg = seg.next) {
    len -= alen;
    dest = (char __user *)dest + alen;
    alen = min(len, DATALEN_SEG);
    if (copy_to_user(dest, seg + 1, alen))
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_msg(msg: *mut msg_msg) {
    void free_msg(struct msg_msg *msg)
    {
    struct msg_msgseg *seg;
    security_msg_msg_free(msg);
    seg = msg.next;
    kfree(msg);
    while (seg != core::ptr::null_mut()) {
    struct msg_msgseg *tmp = seg.next;
    cond_resched();
    kfree(seg);
    seg = tmp;
    }
    }
