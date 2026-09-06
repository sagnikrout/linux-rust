//! Automatically rewritten from C to Rust
//! Source: net/vmw_vsock/af_vsock_tap.c
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
// Tap functions for AF_VSOCK sockets.
//
// Code based on net/netlink/af_netlink.c tap functions.
//

    static DEFINE_SPINLOCK(vsock_tap_lock);
    static struct list_head vsock_tap_all __read_mostly =
    LIST_HEAD_INIT(vsock_tap_all);
#[no_mangle]
pub unsafe extern "C" fn vsock_add_tap(vt: *mut vsock_tap) -> c_int {
    int vsock_add_tap(struct vsock_tap *vt)
    {
    if (unlikely(vt.dev.type != ARPHRD_VSOCKMON))
    return -EINVAL;
    __module_get(vt.module);
    spin_lock(&vsock_tap_lock);
    list_add_rcu(&vt.list, &vsock_tap_all);
    spin_unlock(&vsock_tap_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(vsock_add_tap);
#[no_mangle]
pub unsafe extern "C" fn vsock_remove_tap(vt: *mut vsock_tap) -> c_int {
    int vsock_remove_tap(struct vsock_tap *vt)
    {
    struct vsock_tap *tmp;
    let mut found: bool = false;
    spin_lock(&vsock_tap_lock);
    list_for_each_entry(tmp, &vsock_tap_all, list) {
    if (vt == tmp) {
    list_del_rcu(&vt.list);
    found = true;
    goto out;
    }
    }
    pr_warn("vsock_remove_tap: %p not found\n", vt);
    out:
    spin_unlock(&vsock_tap_lock);
    synchronize_net();
    if (found)
    module_put(vt.module);
    return found ? 0 : -ENODEV;
    }
    EXPORT_SYMBOL_GPL(vsock_remove_tap);
    static int __vsock_deliver_tap_skb(struct sk_buff *skb,
    struct net_device *dev)
    {
    let mut ret: c_int = 0;
    struct sk_buff *nskb = skb_clone(skb, GFP_ATOMIC);
    if (nskb) {
    dev_hold(dev);
    nskb.dev = dev;
    ret = dev_queue_xmit(nskb);
    if (unlikely(ret > 0))
    ret = net_xmit_errno(ret);
    dev_put(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __vsock_deliver_tap(skb: *mut sk_buff) {
    static void __vsock_deliver_tap(struct sk_buff *skb)
    {
    int ret;
    struct vsock_tap *tmp;
    list_for_each_entry_rcu(tmp, &vsock_tap_all, list) {
    ret = __vsock_deliver_tap_skb(skb, tmp.dev);
    if (unlikely(ret))
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vsock_deliver_tap(opaque): *mut *mut sk_buff build_skb(void, opaque: *mut c_void) {
    void vsock_deliver_tap(struct sk_buff *build_skb(void *opaque), void *opaque)
    {
    struct sk_buff *skb;
    rcu_read_lock();
    if (likely(list_empty(&vsock_tap_all)))
    goto out;
    skb = build_skb(opaque);
    if (skb) {
    __vsock_deliver_tap(skb);
    consume_skb(skb);
    }
    out:
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(vsock_deliver_tap);
