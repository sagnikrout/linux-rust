//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hfi1/ipoib_main.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation.
//
// This file contains HFI1 support for ipoib functionality
//

#[no_mangle]
unsafe extern "C" fn qpn_from_mac(mac_arr: *const u8) -> u32 {
    static u32 qpn_from_mac(const u8 *mac_arr)
    {
    return (u32)mac_arr[1] << 16 | mac_arr[2] << 8 | mac_arr[3];
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_dev_init(dev: *mut net_device) -> c_int {
    static int hfi1_ipoib_dev_init(struct net_device *dev)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    int ret;
    ret = priv.netdev_ops.ndo_init(dev);
    if (ret)
    return ret;
    ret = hfi1_netdev_add_data(priv.dd,
    qpn_from_mac(priv.netdev.dev_addr),
    dev);
    if (ret < 0) {
    priv.netdev_ops.ndo_uninit(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_dev_uninit(dev: *mut net_device) {
    static void hfi1_ipoib_dev_uninit(struct net_device *dev)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    hfi1_netdev_remove_data(priv.dd, qpn_from_mac(priv.netdev.dev_addr));
    priv.netdev_ops.ndo_uninit(dev);
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_dev_open(dev: *mut net_device) -> c_int {
    static int hfi1_ipoib_dev_open(struct net_device *dev)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    int ret;
    ret = priv.netdev_ops.ndo_open(dev);
    if (!ret) {
    struct hfi1_ibport *ibp = to_iport(priv.device,
    priv.port_num);
    struct rvt_qp *qp;
    let mut qpn: u32 = qpn_from_mac(priv.netdev.dev_addr);
    rcu_read_lock();
    qp = rvt_lookup_qpn(ib_to_rvt(priv.device), &ibp.rvp, qpn);
    if (!qp) {
    rcu_read_unlock();
    priv.netdev_ops.ndo_stop(dev);
    return -EINVAL;
    }
    rvt_get_qp(qp);
    priv.qp = qp;
    rcu_read_unlock();
    hfi1_netdev_enable_queues(priv.dd);
    hfi1_ipoib_napi_tx_enable(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_dev_stop(dev: *mut net_device) -> c_int {
    static int hfi1_ipoib_dev_stop(struct net_device *dev)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    if (!priv.qp)
    return 0;
    hfi1_ipoib_napi_tx_disable(dev);
    hfi1_netdev_disable_queues(priv.dd);
    rvt_put_qp(priv.qp);
    priv.qp = core::ptr::null_mut();
    return priv.netdev_ops.ndo_stop(dev);
    }
    static const struct net_device_ops hfi1_ipoib_netdev_ops = {
    .ndo_init         = hfi1_ipoib_dev_init,
    .ndo_uninit       = hfi1_ipoib_dev_uninit,
    .ndo_open         = hfi1_ipoib_dev_open,
    .ndo_stop         = hfi1_ipoib_dev_stop,
    };
    static int hfi1_ipoib_mcast_attach(struct net_device *dev,
    struct ib_device *device,
    union ib_gid *mgid,
    u16 mlid,
    int set_qkey,
    u32 qkey)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    let mut qpn: u32 = (u32)qpn_from_mac(priv.netdev.dev_addr);
    struct hfi1_ibport *ibp = to_iport(priv.device, priv.port_num);
    struct rvt_qp *qp;
    let mut ret: c_int = -EINVAL;
    rcu_read_lock();
    qp = rvt_lookup_qpn(ib_to_rvt(priv.device), &ibp.rvp, qpn);
    if (qp) {
    rvt_get_qp(qp);
    rcu_read_unlock();
    if (set_qkey)
    priv.qkey = qkey;
// attach QP to multicast group
    ret = ib_attach_mcast(&qp.ibqp, mgid, mlid);
    rvt_put_qp(qp);
    } else {
    rcu_read_unlock();
    }
    return ret;
    }
    static int hfi1_ipoib_mcast_detach(struct net_device *dev,
    struct ib_device *device,
    union ib_gid *mgid,
    u16 mlid)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    let mut qpn: u32 = (u32)qpn_from_mac(priv.netdev.dev_addr);
    struct hfi1_ibport *ibp = to_iport(priv.device, priv.port_num);
    struct rvt_qp *qp;
    let mut ret: c_int = -EINVAL;
    rcu_read_lock();
    qp = rvt_lookup_qpn(ib_to_rvt(priv.device), &ibp.rvp, qpn);
    if (qp) {
    rvt_get_qp(qp);
    rcu_read_unlock();
    ret = ib_detach_mcast(&qp.ibqp, mgid, mlid);
    rvt_put_qp(qp);
    } else {
    rcu_read_unlock();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_netdev_dtor(dev: *mut net_device) {
    static void hfi1_ipoib_netdev_dtor(struct net_device *dev)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    hfi1_ipoib_txreq_deinit(priv);
    hfi1_ipoib_rxq_deinit(priv.netdev);
    }
#[no_mangle]
unsafe extern "C" fn hfi1_ipoib_set_id(dev: *mut net_device, id: c_int) {
    static void hfi1_ipoib_set_id(struct net_device *dev, int id)
    {
    struct hfi1_ipoib_dev_priv *priv = hfi1_ipoib_priv(dev);
    priv.pkey_index = (u16)id;
    ib_query_pkey(priv.device,
    priv.port_num,
    priv.pkey_index,
    &priv.pkey);
    }
    static int hfi1_ipoib_setup_rn(struct ib_device *device,
    u32 port_num,
    struct net_device *netdev,
    void *param)
    {
    struct hfi1_devdata *dd = dd_from_ibdev(device);
    struct rdma_netdev *rn = netdev_priv(netdev);
    struct hfi1_ipoib_dev_priv *priv;
    int rc;
    rn.send = hfi1_ipoib_send;
    rn.tx_timeout = hfi1_ipoib_tx_timeout;
    rn.attach_mcast = hfi1_ipoib_mcast_attach;
    rn.detach_mcast = hfi1_ipoib_mcast_detach;
    rn.set_id = hfi1_ipoib_set_id;
    rn.hca = device;
    rn.port_num = port_num;
    rn.mtu = netdev.mtu;
    priv = hfi1_ipoib_priv(netdev);
    priv.dd = dd;
    priv.netdev = netdev;
    priv.device = device;
    priv.port_num = port_num;
    priv.netdev_ops = netdev.netdev_ops;
    ib_query_pkey(device, port_num, priv.pkey_index, &priv.pkey);
    rc = hfi1_ipoib_txreq_init(priv);
    if (rc) {
    dd_dev_err(dd, "IPoIB netdev TX init - failed(%d)\n", rc);
    return rc;
    }
    rc = hfi1_ipoib_rxq_init(netdev);
    if (rc) {
    dd_dev_err(dd, "IPoIB netdev RX init - failed(%d)\n", rc);
    hfi1_ipoib_txreq_deinit(priv);
    return rc;
    }
    netdev.netdev_ops = &hfi1_ipoib_netdev_ops;
    netdev.priv_destructor = hfi1_ipoib_netdev_dtor;
    netdev.needs_free_netdev = true;
    netdev.pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS;
    return 0;
    }
    int hfi1_ipoib_rn_get_params(struct ib_device *device,
    u32 port_num,
    enum rdma_netdev_t type,
    struct rdma_netdev_alloc_params *params)
    {
    struct hfi1_devdata *dd = dd_from_ibdev(device);
    if (type != RDMA_NETDEV_IPOIB)
    return -EOPNOTSUPP;
    if (!HFI1_CAP_IS_KSET(AIP) || !dd.num_netdev_contexts)
    return -EOPNOTSUPP;
    if (!port_num || port_num > dd.num_pports)
    return -EINVAL;
    params.sizeof_priv = sizeof(struct hfi1_ipoib_rdma_netdev);
    params.txqs = dd.num_sdma;
    params.rxqs = dd.num_netdev_contexts;
    params.param = core::ptr::null_mut();
    params.initialize_rdma_netdev = hfi1_ipoib_setup_rn;
    return 0;
    }
