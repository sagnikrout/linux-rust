//! Automatically rewritten from C to Rust
//! Source: net/bridge/br_vlan_tunnel.c
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
// Bridge per vlan tunnel port dst_metadata handling code
//
// Authors:
// Roopa Prabhu		<roopa@cumulusnetworks.com>
//

    static inline int br_vlan_tunid_cmp(struct rhashtable_compare_arg *arg,
    const void *ptr)
    {
    const struct net_bridge_vlan *vle = ptr;
    let mut tunid: __be64 = *(__be64 *)arg.key;
    return vle.tinfo.tunnel_id != tunid;
    }
    static const struct rhashtable_params br_vlan_tunnel_rht_params = {
    .head_offset = offsetof(struct net_bridge_vlan, tnode),
    .key_offset = offsetof(struct net_bridge_vlan, tinfo.tunnel_id),
    .key_len = sizeof(__be64),
    .nelem_hint = 3,
    .obj_cmpfn = br_vlan_tunid_cmp,
    .automatic_shrinking = true,
    };
    static struct net_bridge_vlan *br_vlan_tunnel_lookup(struct rhashtable *tbl,
    __be64 tunnel_id)
    {
    return rhashtable_lookup_fast(tbl, &tunnel_id,
    br_vlan_tunnel_rht_params);
    }
#[no_mangle]
unsafe extern "C" fn vlan_tunnel_info_release(vlan: *mut net_bridge_vlan) {
    static void vlan_tunnel_info_release(struct net_bridge_vlan *vlan)
    {
    struct metadata_dst *tdst = rtnl_dereference(vlan.tinfo.tunnel_dst);
    WRITE_ONCE(vlan.tinfo.tunnel_id, 0);
    RCU_INIT_POINTER(vlan.tinfo.tunnel_dst, core::ptr::null_mut());
    dst_release(&tdst.dst);
    }
    void vlan_tunnel_info_del(struct net_bridge_vlan_group *vg,
    struct net_bridge_vlan *vlan)
    {
    if (!rcu_access_pointer(vlan.tinfo.tunnel_dst))
    return;
    rhashtable_remove_fast(&vg.tunnel_hash, &vlan.tnode,
    br_vlan_tunnel_rht_params);
    vlan_tunnel_info_release(vlan);
    }
    static int __vlan_tunnel_info_add(struct net_bridge_vlan_group *vg,
    struct net_bridge_vlan *vlan, u32 tun_id)
    {
    struct metadata_dst *metadata = rtnl_dereference(vlan.tinfo.tunnel_dst);
    let mut key: __be64 = key32_to_tunnel_id(cpu_to_be32(tun_id));
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    int err;
    if (metadata)
    return -EEXIST;
    __set_bit(IP_TUNNEL_KEY_BIT, flags);
    metadata = __ip_tun_set_dst(0, 0, 0, 0, 0, flags, key, 0);
    if (!metadata)
    return -EINVAL;
    metadata.u.tun_info.mode |= IP_TUNNEL_INFO_TX | IP_TUNNEL_INFO_BRIDGE;
    rcu_assign_pointer(vlan.tinfo.tunnel_dst, metadata);
    WRITE_ONCE(vlan.tinfo.tunnel_id, key);
    err = rhashtable_lookup_insert_fast(&vg.tunnel_hash, &vlan.tnode,
    br_vlan_tunnel_rht_params);
    if (err)
    goto out;
    return 0;
    out:
    vlan_tunnel_info_release(vlan);
    return err;
    }
// Must be protected by RTNL.
// Must be called with vid in range from 1 to 4094 inclusive.
//
    int nbp_vlan_tunnel_info_add(const struct net_bridge_port *port, u16 vid,
    u32 tun_id)
    {
    struct net_bridge_vlan_group *vg;
    struct net_bridge_vlan *vlan;
    ASSERT_RTNL();
    vg = nbp_vlan_group(port);
    vlan = br_vlan_find(vg, vid);
    if (!vlan)
    return -EINVAL;
    return __vlan_tunnel_info_add(vg, vlan, tun_id);
    }
// Must be protected by RTNL.
// Must be called with vid in range from 1 to 4094 inclusive.
//
#[no_mangle]
pub unsafe extern "C" fn nbp_vlan_tunnel_info_delete(port: *const net_bridge_port, vid: u16) -> c_int {
    int nbp_vlan_tunnel_info_delete(const struct net_bridge_port *port, u16 vid)
    {
    struct net_bridge_vlan_group *vg;
    struct net_bridge_vlan *v;
    ASSERT_RTNL();
    vg = nbp_vlan_group(port);
    v = br_vlan_find(vg, vid);
    if (!v)
    return -ENOENT;
    vlan_tunnel_info_del(vg, v);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __vlan_tunnel_info_flush(vg: *mut net_bridge_vlan_group) {
    static void __vlan_tunnel_info_flush(struct net_bridge_vlan_group *vg)
    {
    struct net_bridge_vlan *vlan, *tmp;
    list_for_each_entry_safe(vlan, tmp, &vg.vlan_list, vlist)
    vlan_tunnel_info_del(vg, vlan);
    }
#[no_mangle]
pub unsafe extern "C" fn nbp_vlan_tunnel_info_flush(port: *mut net_bridge_port) {
    void nbp_vlan_tunnel_info_flush(struct net_bridge_port *port)
    {
    struct net_bridge_vlan_group *vg;
    ASSERT_RTNL();
    vg = nbp_vlan_group(port);
    __vlan_tunnel_info_flush(vg);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_tunnel_init(vg: *mut net_bridge_vlan_group) -> c_int {
    int vlan_tunnel_init(struct net_bridge_vlan_group *vg)
    {
    return rhashtable_init(&vg.tunnel_hash, &br_vlan_tunnel_rht_params);
    }
#[no_mangle]
pub unsafe extern "C" fn vlan_tunnel_deinit(vg: *mut net_bridge_vlan_group) {
    void vlan_tunnel_deinit(struct net_bridge_vlan_group *vg)
    {
    rhashtable_destroy(&vg.tunnel_hash);
    }
    void br_handle_ingress_vlan_tunnel(struct sk_buff *skb,
    struct net_bridge_port *p,
    struct net_bridge_vlan_group *vg)
    {
    struct ip_tunnel_info *tinfo = skb_tunnel_info(skb);
    struct net_bridge_vlan *vlan;
    if (!vg || !tinfo)
    return;
// if already tagged, ignore
    if (skb_vlan_tagged(skb))
    return;
// lookup vid, given tunnel id
    vlan = br_vlan_tunnel_lookup(&vg.tunnel_hash, tinfo.key.tun_id);
    if (!vlan)
    return;
    skb_dst_drop(skb);
    __vlan_hwaccel_put_tag(skb, p.br.vlan_proto, vlan.vid);
    }
    int br_handle_egress_vlan_tunnel(struct sk_buff *skb,
    struct net_bridge_vlan *vlan)
    {
    IP_TUNNEL_DECLARE_FLAGS(flags) = { };
    struct metadata_dst *tunnel_dst;
    __be64 tunnel_id;
    if (!vlan)
    return 0;
    tunnel_id = READ_ONCE(vlan.tinfo.tunnel_id);
    if (!tunnel_id || unlikely(!skb_vlan_tag_present(skb)))
    return 0;
    skb_dst_drop(skb);
// For 802.1ad (QinQ), skb_vlan_pop() incorrectly moves the C-VLAN
// from payload to hwaccel after clearing S-VLAN. We only need to
// clear the hwaccel S-VLAN; the C-VLAN must stay in payload for
// correct VXLAN encapsulation. This is also correct for 802.1Q
// where no C-VLAN exists in payload.
//
    __vlan_hwaccel_clear_tag(skb);
    if (BR_INPUT_SKB_CB(skb).backup_nhid) {
    __set_bit(IP_TUNNEL_KEY_BIT, flags);
    tunnel_dst = __ip_tun_set_dst(0, 0, 0, 0, 0, flags,
    tunnel_id, 0);
    if (!tunnel_dst)
    return -ENOMEM;
    tunnel_dst.u.tun_info.mode |= IP_TUNNEL_INFO_TX |
    IP_TUNNEL_INFO_BRIDGE;
    tunnel_dst.u.tun_info.key.nhid =
    BR_INPUT_SKB_CB(skb).backup_nhid;
    skb_dst_set(skb, &tunnel_dst.dst);
    return 0;
    }
    tunnel_dst = rcu_dereference(vlan.tinfo.tunnel_dst);
    if (tunnel_dst && dst_hold_safe(&tunnel_dst.dst))
    skb_dst_set(skb, &tunnel_dst.dst);
    return 0;
    }
