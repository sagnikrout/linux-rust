//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/rep/bond.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2020 Mellanox Technologies Inc. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_bond {
    pub nb: notifier_block,
    pub nn: netdev_net_notifier,
    pub metadata_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_bond_slave_entry {
    pub list: list_head,
    pub netdev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_bond_metadata {
    pub /: *mut *mut list_head list; / link to global list of rep_bond_metadata,
    pub esw: *mut mlx5_eswitch,
// private of uplink holding rep bond metadata list
    pub lag_dev: *mut net_device,
    pub metadata_reg_c_0: u32,
    pub /: *mut *mut list_head slaves_list; / slaves list,
    pub slaves: c_int,
}

    static struct mlx5e_rep_bond_metadata *
    mlx5e_lookup_rep_bond_metadata(struct mlx5_rep_uplink_priv *uplink_priv,
    const struct net_device *lag_dev)
    {
    struct mlx5e_rep_bond_metadata *found = core::ptr::null_mut();
    struct mlx5e_rep_bond_metadata *cur;
    list_for_each_entry(cur, &uplink_priv.bond.metadata_list, list) {
    if (cur.lag_dev == lag_dev) {
    found = cur;
    break;
    }
    }
    return found;
    }
    static struct mlx5e_rep_bond_slave_entry *
    mlx5e_lookup_rep_bond_slave_entry(struct mlx5e_rep_bond_metadata *mdata,
    const struct net_device *netdev)
    {
    struct mlx5e_rep_bond_slave_entry *found = core::ptr::null_mut();
    struct mlx5e_rep_bond_slave_entry *cur;
    list_for_each_entry(cur, &mdata.slaves_list, list) {
    if (cur.netdev == netdev) {
    found = cur;
    break;
    }
    }
    return found;
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_rep_bond_metadata_release(mdata: *mut mlx5e_rep_bond_metadata) {
    static void mlx5e_rep_bond_metadata_release(struct mlx5e_rep_bond_metadata *mdata)
    {
    netdev_dbg(mdata.lag_dev, "destroy rep_bond_metadata(%d)\n",
    mdata.metadata_reg_c_0);
    list_del(&mdata.list);
    mlx5_esw_match_metadata_free(mdata.esw, mdata.metadata_reg_c_0);
    WARN_ON(!list_empty(&mdata.slaves_list));
    kfree(mdata);
    }
// This must be called under rtnl_lock
    int mlx5e_rep_bond_enslave(struct mlx5_eswitch *esw, struct net_device *netdev,
    struct net_device *lag_dev)
    {
    struct mlx5e_rep_bond_slave_entry *s_entry;
    struct mlx5e_rep_bond_metadata *mdata;
    struct mlx5e_rep_priv *rpriv;
    struct mlx5e_priv *priv;
    int err;
    ASSERT_RTNL();
    rpriv = mlx5_eswitch_get_uplink_priv(esw, REP_ETH);
    mdata = mlx5e_lookup_rep_bond_metadata(&rpriv.uplink_priv, lag_dev);
    if (!mdata) {
// First netdev becomes slave, no metadata presents the lag_dev. Create one
    mdata = kzalloc_obj(*mdata);
    if (!mdata)
    return -ENOMEM;
    mdata.lag_dev = lag_dev;
    mdata.esw = esw;
    INIT_LIST_HEAD(&mdata.slaves_list);
    mdata.metadata_reg_c_0 = mlx5_esw_match_metadata_alloc(esw);
    if (!mdata.metadata_reg_c_0) {
    kfree(mdata);
    return -ENOSPC;
    }
    list_add(&mdata.list, &rpriv.uplink_priv.bond.metadata_list);
    netdev_dbg(lag_dev, "create rep_bond_metadata(%d)\n",
    mdata.metadata_reg_c_0);
    }
    s_entry = kzalloc_obj(*s_entry);
    if (!s_entry) {
    err = -ENOMEM;
    goto entry_alloc_err;
    }
    s_entry.netdev = netdev;
    priv = netdev_priv(netdev);
    rpriv = priv.ppriv;
    err = mlx5_esw_acl_ingress_vport_metadata_update(esw, rpriv.rep.vport,
    mdata.metadata_reg_c_0);
    if (err)
    goto ingress_err;
    mdata.slaves++;
    list_add_tail(&s_entry.list, &mdata.slaves_list);
    netdev_dbg(netdev, "enslave rep vport(%d) lag_dev(%s) metadata(0x%x)\n",
    rpriv.rep.vport, lag_dev.name, mdata.metadata_reg_c_0);
    return 0;
    ingress_err:
    kfree(s_entry);
    entry_alloc_err:
    if (!mdata.slaves)
    mlx5e_rep_bond_metadata_release(mdata);
    return err;
    }
// This must be called under rtnl_lock
    void mlx5e_rep_bond_unslave(struct mlx5_eswitch *esw,
    const struct net_device *netdev,
    const struct net_device *lag_dev)
    {
    struct mlx5e_rep_bond_slave_entry *s_entry;
    struct mlx5e_rep_bond_metadata *mdata;
    struct mlx5e_rep_priv *rpriv;
    struct mlx5e_priv *priv;
    ASSERT_RTNL();
    rpriv = mlx5_eswitch_get_uplink_priv(esw, REP_ETH);
    mdata = mlx5e_lookup_rep_bond_metadata(&rpriv.uplink_priv, lag_dev);
    if (!mdata)
    return;
    s_entry = mlx5e_lookup_rep_bond_slave_entry(mdata, netdev);
    if (!s_entry)
    return;
    priv = netdev_priv(netdev);
    rpriv = priv.ppriv;
// Reset bond_metadata to zero first then reset all ingress/egress
// acls and rx rules of unslave representor's vport
//
    mlx5_esw_acl_ingress_vport_metadata_update(esw, rpriv.rep.vport, 0);
    mlx5_esw_acl_egress_vport_unbond(esw, rpriv.rep.vport);
    mlx5e_rep_bond_update(priv, false);
    list_del(&s_entry.list);
    netdev_dbg(netdev, "unslave rep vport(%d) lag_dev(%s) metadata(0x%x)\n",
    rpriv.rep.vport, lag_dev.name, mdata.metadata_reg_c_0);
    if (--mdata.slaves == 0)
    mlx5e_rep_bond_metadata_release(mdata);
    kfree(s_entry);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_rep_is_lag_netdev(netdev: *mut net_device) -> bool {
    static bool mlx5e_rep_is_lag_netdev(struct net_device *netdev)
    {
    return netif_is_lag_port(netdev) && mlx5e_eswitch_vf_rep(netdev);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_rep_changelowerstate_event(netdev: *mut net_device, ptr: *mut c_void) {
    static void mlx5e_rep_changelowerstate_event(struct net_device *netdev, void *ptr)
    {
    struct netdev_notifier_changelowerstate_info *info;
    struct netdev_lag_lower_state_info *lag_info;
    struct mlx5e_rep_priv *rpriv;
    struct net_device *lag_dev;
    struct mlx5e_priv *priv;
    struct list_head *iter;
    struct net_device *dev;
    u16 acl_vport_num;
    u16 fwd_vport_num;
    int err;
    info = ptr;
    lag_info = info.lower_state_info;
// This is not an event of a representor becoming active slave
    if (!lag_info.tx_enabled)
    return;
    priv = netdev_priv(netdev);
    rpriv = priv.ppriv;
    fwd_vport_num = rpriv.rep.vport;
    lag_dev = netdev_master_upper_dev_get(netdev);
    if (!lag_dev)
    return;
    netdev_dbg(netdev, "lag_dev(%s)'s slave vport(%d) is txable(%d)\n",
    lag_dev.name, fwd_vport_num, net_lag_port_dev_txable(netdev));
// Point everyone's egress acl to the vport of the active representor
    netdev_for_each_lower_dev(lag_dev, dev, iter) {
    priv = netdev_priv(dev);
    rpriv = priv.ppriv;
    acl_vport_num = rpriv.rep.vport;
    if (acl_vport_num != fwd_vport_num) {
// Only single rx_rule for unique bond_metadata should be
// present, delete it if it's saved as passive vport's
// rx_rule with destination as passive vport's root_ft
//
    mlx5e_rep_bond_update(priv, true);
    err = mlx5_esw_acl_egress_vport_bond(priv.mdev.priv.eswitch,
    fwd_vport_num,
    acl_vport_num);
    if (err)
    netdev_warn(dev,
    "configure slave vport(%d) egress fwd, err(%d)",
    acl_vport_num, err);
    }
    }
// Insert new rx_rule for unique bond_metadata, save it as active vport's
// rx_rule with new destination as active vport's root_ft
//
    err = mlx5e_rep_bond_update(netdev_priv(netdev), false);
    if (err)
    netdev_warn(netdev, "configure active slave vport(%d) rx_rule, err(%d)",
    fwd_vport_num, err);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_rep_changeupper_event(netdev: *mut net_device, ptr: *mut c_void) {
    static void mlx5e_rep_changeupper_event(struct net_device *netdev, void *ptr)
    {
    struct netdev_notifier_changeupper_info *info = ptr;
    struct mlx5e_rep_priv *rpriv;
    struct net_device *lag_dev;
    struct mlx5e_priv *priv;
    priv = netdev_priv(netdev);
    rpriv = priv.ppriv;
    lag_dev = info.upper_dev;
    netdev_dbg(netdev, "%sslave vport(%d) lag(%s)\n",
    info.linking ? "en" : "un", rpriv.rep.vport, lag_dev.name);
    if (info.linking)
    mlx5e_rep_bond_enslave(priv.mdev.priv.eswitch, netdev, lag_dev);
    else
    mlx5e_rep_bond_unslave(priv.mdev.priv.eswitch, netdev, lag_dev);
    }
// Bond device of representors and netdev events are used here in specific way
// to support eswitch vports bonding and to perform failover of eswitch vport
// by modifying the vport's egress acl of lower dev representors. Thus this
// also change the traditional behavior of lower dev under bond device.
// All non-representor netdevs or representors of other vendors as lower dev
// of bond device are not supported.
//
    static int mlx5e_rep_esw_bond_netevent(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct net_device *netdev = netdev_notifier_info_to_dev(ptr);
    struct mlx5e_rep_priv *rpriv;
    struct mlx5e_rep_bond *bond;
    struct mlx5e_priv *priv;
    if (!mlx5e_rep_is_lag_netdev(netdev))
    return NOTIFY_DONE;
    bond = container_of(nb, struct mlx5e_rep_bond, nb);
    priv = netdev_priv(netdev);
    rpriv = mlx5_eswitch_get_uplink_priv(priv.mdev.priv.eswitch, REP_ETH);
// Verify VF representor is on the same device of the bond handling the netevent.
    if (rpriv.uplink_priv.bond != bond)
    return NOTIFY_DONE;
    switch (event) {
    case NETDEV_CHANGELOWERSTATE:
    mlx5e_rep_changelowerstate_event(netdev, ptr);
    break;
    case NETDEV_CHANGEUPPER:
    mlx5e_rep_changeupper_event(netdev, ptr);
    break;
    }
    return NOTIFY_DONE;
    }
// If HW support eswitch vports bonding, register a specific notifier to
// handle it when two or more representors are bonded
//
#[no_mangle]
pub unsafe extern "C" fn mlx5e_rep_bond_init(rpriv: *mut mlx5e_rep_priv) -> c_int {
    int mlx5e_rep_bond_init(struct mlx5e_rep_priv *rpriv)
    {
    struct mlx5_rep_uplink_priv *uplink_priv = &rpriv.uplink_priv;
    struct net_device *netdev = rpriv.netdev;
    struct mlx5e_priv *priv;
    let mut ret: c_int = 0;
    priv = netdev_priv(netdev);
    if (!mlx5_esw_acl_egress_fwd2vport_supported(priv.mdev.priv.eswitch))
    goto out;
    uplink_priv.bond = kvzalloc_obj(*uplink_priv.bond);
    if (!uplink_priv.bond) {
    ret = -ENOMEM;
    goto out;
    }
    INIT_LIST_HEAD(&uplink_priv.bond.metadata_list);
    uplink_priv.bond.nb.notifier_call = mlx5e_rep_esw_bond_netevent;
    ret = register_netdevice_notifier_dev_net(netdev,
    &uplink_priv.bond.nb,
    &uplink_priv.bond.nn);
    if (ret) {
    netdev_err(netdev, "register bonding netevent notifier, err(%d)\n", ret);
    kvfree(uplink_priv.bond);
    uplink_priv.bond = core::ptr::null_mut();
    }
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_rep_bond_cleanup(rpriv: *mut mlx5e_rep_priv) {
    void mlx5e_rep_bond_cleanup(struct mlx5e_rep_priv *rpriv)
    {
    struct mlx5e_priv *priv = netdev_priv(rpriv.netdev);
    if (!mlx5_esw_acl_egress_fwd2vport_supported(priv.mdev.priv.eswitch) ||
    !rpriv.uplink_priv.bond)
    return;
    unregister_netdevice_notifier_dev_net(rpriv.netdev,
    &rpriv.uplink_priv.bond.nb,
    &rpriv.uplink_priv.bond.nn);
    kvfree(rpriv.uplink_priv.bond);
    }
