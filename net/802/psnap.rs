//! Automatically rewritten from C to Rust
//! Source: net/802/psnap.c
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
// SNAP data link layer. Derived from 802.2
//
// Alan Cox <alan@lxorguk.ukuu.org.uk>,
// from the 802.2 layer by Greg Page.
// Merged in additions from Greg Page's psnap.c.
//

    static LIST_HEAD(snap_list);
    static DEFINE_SPINLOCK(snap_lock);
    static struct llc_sap *snap_sap;
//
// Find a snap client by matching the 5 bytes.
//
    static struct datalink_proto *find_snap_client(const unsigned char *desc)
    {
    struct datalink_proto *proto = core::ptr::null_mut(), *p;
    list_for_each_entry_rcu(p, &snap_list, node, lockdep_is_held(&snap_lock)) {
    if (!memcmp(p.type, desc, 5)) {
    proto = p;
    break;
    }
    }
    return proto;
    }
//
// A SNAP packet has arrived
//
    static int snap_rcv(struct sk_buff *skb, struct net_device *dev,
    struct packet_type *pt, struct net_device *orig_dev)
    {
    let mut rc: c_int = 1;
    struct datalink_proto *proto;
    static struct packet_type snap_packet_type = {
    .type = cpu_to_be16(ETH_P_SNAP),
    };
    if (unlikely(!pskb_may_pull(skb, 5)))
    goto drop;
    rcu_read_lock();
    proto = find_snap_client(skb.data);
    if (proto) {
// Pass the frame on.
    skb_pull_rcsum(skb, 5);
    skb_reset_transport_header(skb);
    rc = proto.rcvfunc(skb, dev, &snap_packet_type, orig_dev);
    }
    rcu_read_unlock();
    if (unlikely(!proto))
    goto drop;
    out:
    return rc;
    drop:
    kfree_skb(skb);
    goto out;
    }
//
// Put a SNAP header on a frame and pass to 802.2
//
    static int snap_request(struct datalink_proto *dl,
    struct sk_buff *skb, const u8 *dest)
    {
    memcpy(skb_push(skb, 5), dl.type, 5);
    llc_build_and_send_ui_pkt(snap_sap, skb, dest, snap_sap.laddr.lsap);
    return 0;
    }
//
// Set up the SNAP layer
//
    EXPORT_SYMBOL(register_snap_client);
    EXPORT_SYMBOL(unregister_snap_client);
    static const char snap_err_msg[] __initconst =
    KERN_CRIT "SNAP - unable to register with 802.2\n";
#[no_mangle]
unsafe extern "C" fn snap_init() -> int __init {
    static int __init snap_init(void)
    {
    snap_sap = llc_sap_open(0xAA, snap_rcv);
    if (!snap_sap) {
    printk(snap_err_msg);
    return -EBUSY;
    }
    return 0;
    }
    module_init(snap_init);
#[no_mangle]
unsafe extern "C" fn snap_exit() -> void __exit {
    static void __exit snap_exit(void)
    {
    llc_sap_put(snap_sap);
    }
    module_exit(snap_exit);
//
// Register SNAP clients. We don't yet use this for IP.
//
    struct datalink_proto *register_snap_client(const unsigned char *desc,
    int (*rcvfunc)(struct sk_buff *,
    struct net_device *,
    struct packet_type *,
    struct net_device *))
    {
    struct datalink_proto *proto = core::ptr::null_mut();
    spin_lock_bh(&snap_lock);
    if (find_snap_client(desc))
    goto out;
    proto = kmalloc_obj(*proto, GFP_ATOMIC);
    if (proto) {
    memcpy(proto.type, desc, 5);
    proto.rcvfunc		= rcvfunc;
    proto.header_length	= 5 + 3; /* snap + 802.2 */
    proto.request		= snap_request;
    list_add_rcu(&proto.node, &snap_list);
    }
    out:
    spin_unlock_bh(&snap_lock);
    return proto;
    }
//
// Unregister SNAP clients. Protocols no longer want to play with us ...
//
#[no_mangle]
pub unsafe extern "C" fn unregister_snap_client(proto: *mut datalink_proto) {
    void unregister_snap_client(struct datalink_proto *proto)
    {
    spin_lock_bh(&snap_lock);
    list_del_rcu(&proto.node);
    spin_unlock_bh(&snap_lock);
    synchronize_net();
    kfree(proto);
    }
    MODULE_DESCRIPTION("SNAP data link layer. Derived from 802.2");
    MODULE_LICENSE("GPL");
