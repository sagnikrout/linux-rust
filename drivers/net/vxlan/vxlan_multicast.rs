//! Automatically rewritten from C to Rust
//! Source: drivers/net/vxlan/vxlan_multicast.c
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
// Vxlan multicast group handling
//

// Update multicast group membership when first VNI on
// multicast address is brought up
//
    int vxlan_igmp_join(struct vxlan_dev *vxlan, union vxlan_addr *rip,
    int rifindex)
    {
    union vxlan_addr *ip = (rip ? : &vxlan.default_dst.remote_ip);
    let mut ifindex: c_int = (rifindex ? : vxlan.default_dst.remote_ifindex);
    let mut ret: c_int = -EINVAL;
    struct sock *sk;
    if (ip.sa.sa_family == AF_INET) {
    struct vxlan_sock *sock4 = rtnl_dereference(vxlan.vn4_sock);
    struct ip_mreqn mreq = {
    .imr_multiaddr.s_addr	= ip.sin.sin_addr.s_addr,
    .imr_ifindex		= ifindex,
    };
    sk = sock4.sk;
    lock_sock(sk);
    ret = ip_mc_join_group(sk, &mreq);
    release_sock(sk);

    } else {
    struct vxlan_sock *sock6 = rtnl_dereference(vxlan.vn6_sock);
    sk = sock6.sk;
    lock_sock(sk);
    ret = ipv6_sock_mc_join(sk, ifindex, &ip.sin6.sin6_addr);
    release_sock(sk);

    }
    return ret;
    }
    int vxlan_igmp_leave(struct vxlan_dev *vxlan, union vxlan_addr *rip,
    int rifindex)
    {
    union vxlan_addr *ip = (rip ? : &vxlan.default_dst.remote_ip);
    let mut ifindex: c_int = (rifindex ? : vxlan.default_dst.remote_ifindex);
    let mut ret: c_int = -EINVAL;
    struct sock *sk;
    if (ip.sa.sa_family == AF_INET) {
    struct vxlan_sock *sock4 = rtnl_dereference(vxlan.vn4_sock);
    struct ip_mreqn mreq = {
    .imr_multiaddr.s_addr	= ip.sin.sin_addr.s_addr,
    .imr_ifindex		= ifindex,
    };
    sk = sock4.sk;
    lock_sock(sk);
    ret = ip_mc_leave_group(sk, &mreq);
    release_sock(sk);

    } else {
    struct vxlan_sock *sock6 = rtnl_dereference(vxlan.vn6_sock);
    sk = sock6.sk;
    lock_sock(sk);
    ret = ipv6_sock_mc_drop(sk, ifindex, &ip.sin6.sin6_addr);
    release_sock(sk);

    }
    return ret;
    }
    static bool vxlan_group_used_match(union vxlan_addr *ip, int ifindex,
    union vxlan_addr *rip, int rifindex)
    {
    if (!vxlan_addr_multicast(rip))
    return false;
    if (!vxlan_addr_equal(rip, ip))
    return false;
    if (rifindex != ifindex)
    return false;
    return true;
    }
    static bool vxlan_group_used_by_vnifilter(struct vxlan_dev *vxlan,
    union vxlan_addr *ip, int ifindex)
    {
    struct vxlan_vni_group *vg = rtnl_dereference(vxlan.vnigrp);
    struct vxlan_vni_node *v, *tmp;
    if (vxlan_group_used_match(ip, ifindex,
    &vxlan.default_dst.remote_ip,
    vxlan.default_dst.remote_ifindex))
    return true;
    list_for_each_entry_safe(v, tmp, &vg.vni_list, vlist) {
    if (!vxlan_addr_multicast(&v.remote_ip))
    continue;
    if (vxlan_group_used_match(ip, ifindex,
    &v.remote_ip,
    vxlan.default_dst.remote_ifindex))
    return true;
    }
    return false;
    }
// See if multicast group is already in use by other ID
    bool vxlan_group_used(struct vxlan_net *vn, struct vxlan_dev *dev,
    __be32 vni, union vxlan_addr *rip, int rifindex)
    {
    union vxlan_addr *ip = (rip ? : &dev.default_dst.remote_ip);
    let mut ifindex: c_int = (rifindex ? : dev.default_dst.remote_ifindex);
    struct vxlan_dev *vxlan;
    struct vxlan_sock *sock4;

    struct vxlan_sock *sock6;

    let mut family: c_ushort = dev.default_dst.remote_ip.sa.sa_family;
    sock4 = rtnl_dereference(dev.vn4_sock);
// The vxlan_sock is only used by dev, leaving group has
// no effect on other vxlan devices.
//
    if (family == AF_INET && sock4 && refcount_read(&sock4.refcnt) == 1)
    return false;

    sock6 = rtnl_dereference(dev.vn6_sock);
    if (family == AF_INET6 && sock6 && refcount_read(&sock6.refcnt) == 1)
    return false;

    list_for_each_entry(vxlan, &vn.vxlan_list, next) {
    if (!netif_running(vxlan.dev) || vxlan == dev)
    continue;
    if (family == AF_INET &&
    rtnl_dereference(vxlan.vn4_sock) != sock4)
    continue;

    if (family == AF_INET6 &&
    rtnl_dereference(vxlan.vn6_sock) != sock6)
    continue;

    if (vxlan.cfg.flags & VXLAN_F_VNIFILTER) {
    if (!vxlan_group_used_by_vnifilter(vxlan, ip, ifindex))
    continue;
    } else {
    if (!vxlan_group_used_match(ip, ifindex,
    &vxlan.default_dst.remote_ip,
    vxlan.default_dst.remote_ifindex))
    continue;
    }
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_multicast_join_vnigrp(vxlan: *mut vxlan_dev) -> c_int {
    static int vxlan_multicast_join_vnigrp(struct vxlan_dev *vxlan)
    {
    struct vxlan_vni_group *vg = rtnl_dereference(vxlan.vnigrp);
    struct vxlan_vni_node *v, *tmp, *vgood = core::ptr::null_mut();
    let mut ret: c_int = 0;
    list_for_each_entry_safe(v, tmp, &vg.vni_list, vlist) {
    if (!vxlan_addr_multicast(&v.remote_ip))
    continue;
// skip if address is same as default address
    if (vxlan_addr_equal(&v.remote_ip,
    &vxlan.default_dst.remote_ip))
    continue;
    ret = vxlan_igmp_join(vxlan, &v.remote_ip, 0);
    if (ret == -EADDRINUSE)
    ret = 0;
    if (ret)
    goto out;
    vgood = v;
    }
    out:
    if (ret) {
    list_for_each_entry_safe(v, tmp, &vg.vni_list, vlist) {
    if (!vxlan_addr_multicast(&v.remote_ip))
    continue;
    if (vxlan_addr_equal(&v.remote_ip,
    &vxlan.default_dst.remote_ip))
    continue;
    vxlan_igmp_leave(vxlan, &v.remote_ip, 0);
    if (v == vgood)
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_multicast_leave_vnigrp(vxlan: *mut vxlan_dev) -> c_int {
    static int vxlan_multicast_leave_vnigrp(struct vxlan_dev *vxlan)
    {
    struct vxlan_net *vn = net_generic(vxlan.net, vxlan_net_id);
    struct vxlan_vni_group *vg = rtnl_dereference(vxlan.vnigrp);
    struct vxlan_vni_node *v, *tmp;
    let mut last_err: c_int = 0, ret;
    list_for_each_entry_safe(v, tmp, &vg.vni_list, vlist) {
    if (vxlan_addr_multicast(&v.remote_ip) &&
    !vxlan_group_used(vn, vxlan, v.vni, &v.remote_ip,
    0)) {
    ret = vxlan_igmp_leave(vxlan, &v.remote_ip, 0);
    if (ret)
    last_err = ret;
    }
    }
    return last_err;
    }
#[no_mangle]
pub unsafe extern "C" fn vxlan_multicast_join(vxlan: *mut vxlan_dev) -> c_int {
    int vxlan_multicast_join(struct vxlan_dev *vxlan)
    {
    let mut ret: c_int = 0;
    if (vxlan_addr_multicast(&vxlan.default_dst.remote_ip)) {
    ret = vxlan_igmp_join(vxlan, &vxlan.default_dst.remote_ip,
    vxlan.default_dst.remote_ifindex);
    if (ret == -EADDRINUSE)
    ret = 0;
    if (ret)
    return ret;
    }
    if (vxlan.cfg.flags & VXLAN_F_VNIFILTER)
    return vxlan_multicast_join_vnigrp(vxlan);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vxlan_multicast_leave(vxlan: *mut vxlan_dev) -> c_int {
    int vxlan_multicast_leave(struct vxlan_dev *vxlan)
    {
    struct vxlan_net *vn = net_generic(vxlan.net, vxlan_net_id);
    let mut ret: c_int = 0;
    if (vxlan_addr_multicast(&vxlan.default_dst.remote_ip) &&
    !vxlan_group_used(vn, vxlan, 0, core::ptr::null_mut(), 0)) {
    ret = vxlan_igmp_leave(vxlan, &vxlan.default_dst.remote_ip,
    vxlan.default_dst.remote_ifindex);
    if (ret)
    return ret;
    }
    if (vxlan.cfg.flags & VXLAN_F_VNIFILTER)
    return vxlan_multicast_leave_vnigrp(vxlan);
    return 0;
    }
