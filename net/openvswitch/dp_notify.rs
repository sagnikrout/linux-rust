//! Automatically rewritten from C to Rust
//! Source: net/openvswitch/dp_notify.c
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
// Copyright (c) 2007-2012 Nicira, Inc.
//

#[no_mangle]
unsafe extern "C" fn dp_detach_port_notify(vport: *mut vport) {
    static void dp_detach_port_notify(struct vport *vport)
    {
    struct sk_buff *notify;
    struct datapath *dp;
    dp = vport.dp;
    notify = ovs_vport_cmd_build_info(vport, ovs_dp_get_net(dp),
    0, 0, OVS_VPORT_CMD_DEL);
    ovs_dp_detach_port(vport);
    if (IS_ERR(notify)) {
    genl_set_err(&dp_vport_genl_family, ovs_dp_get_net(dp), 0,
    0, PTR_ERR(notify));
    return;
    }
    genlmsg_multicast_netns(&dp_vport_genl_family,
    ovs_dp_get_net(dp), notify, 0,
    0, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn ovs_dp_notify_wq(work: *mut work_struct) {
    void ovs_dp_notify_wq(struct work_struct *work)
    {
    struct ovs_net *ovs_net = container_of(work, struct ovs_net, dp_notify_work);
    struct datapath *dp;
    ovs_lock();
    list_for_each_entry(dp, &ovs_net.dps, list_node) {
    int i;
    for (i = 0; i < DP_VPORT_HASH_BUCKETS; i++) {
    struct vport *vport;
    struct hlist_node *n;
    hlist_for_each_entry_safe(vport, n, &dp.ports[i], dp_hash_node) {
    if (vport.ops.type == OVS_VPORT_TYPE_INTERNAL)
    continue;
    if (!(netif_is_ovs_port(vport.dev)))
    dp_detach_port_notify(vport);
    }
    }
    }
    ovs_unlock();
    }
    static int dp_device_event(struct notifier_block *unused, unsigned long event,
    void *ptr)
    {
    struct ovs_net *ovs_net;
    struct net_device *dev = netdev_notifier_info_to_dev(ptr);
    struct vport *vport = core::ptr::null_mut();
    if (!ovs_is_internal_dev(dev))
    vport = ovs_netdev_get_vport(dev);
    if (!vport)
    return NOTIFY_DONE;
    if (event == NETDEV_UNREGISTER) {
// upper_dev_unlink and decrement promisc immediately
    ovs_netdev_detach_dev(vport);
// schedule vport destroy, dev_put and genl notification
    ovs_net = net_generic(dev_net(dev), ovs_net_id);
    queue_work(system_percpu_wq, &ovs_net.dp_notify_work);
    }
    return NOTIFY_DONE;
    }
    struct notifier_block ovs_dp_device_notifier = {
    .notifier_call = dp_device_event
    };
