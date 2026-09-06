//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/vdpa_sim/vdpa_sim_net.c
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
// VDPA simulator for networking device.
//
// Copyright (c) 2020, Red Hat Inc. All rights reserved.
// Author: Jason Wang <jasowang@redhat.com>
//

    (1ULL << VIRTIO_NET_F_MAC) | \
    (1ULL << VIRTIO_NET_F_STATUS) | \
    (1ULL << VIRTIO_NET_F_MTU) | \
    (1ULL << VIRTIO_NET_F_CTRL_VQ) | \
    (1ULL << VIRTIO_NET_F_CTRL_MAC_ADDR))
// 3 virtqueues, 2 address spaces, 2 virtqueue groups
pub const VDPASIM_NET_VQ_NUM: c_int = 3;
pub const VDPASIM_NET_AS_NUM: c_int = 2;
pub const VDPASIM_NET_GROUP_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_dataq_stats {
    pub syncp: u64_stats_sync,
    pub pkts: u64,
    pub bytes: u64,
    pub drops: u64,
    pub errors: u64,
    pub overruns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_cq_stats {
    pub syncp: u64_stats_sync,
    pub requests: u64,
    pub successes: u64,
    pub errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_net {
    pub vdpasim: vdpasim,
    pub tx_stats: vdpasim_dataq_stats,
    pub rx_stats: vdpasim_dataq_stats,
    pub cq_stats: vdpasim_cq_stats,
    pub buffer: *mut c_void,
}

    static struct vdpasim_net *sim_to_net(struct vdpasim *vdpasim)
    {
    return container_of(vdpasim, struct vdpasim_net, vdpasim);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_net_complete(vq: *mut vdpasim_virtqueue, len: usize) {
    static void vdpasim_net_complete(struct vdpasim_virtqueue *vq, size_t len)
    {
// Make sure data is wrote before advancing index
    smp_wmb();
    vringh_complete_iotlb(&vq.vring, vq.head, len);
// Make sure used is visible before rasing the interrupt.
    smp_wmb();
    local_bh_disable();
    if (vringh_need_notify_iotlb(&vq.vring) > 0)
    vringh_notify(&vq.vring);
    local_bh_enable();
    }
#[no_mangle]
unsafe extern "C" fn receive_filter(vdpasim: *mut vdpasim, len: usize) -> bool {
    static bool receive_filter(struct vdpasim *vdpasim, size_t len)
    {
    let mut modern: bool = vdpasim.features & (1ULL << VIRTIO_F_VERSION_1);
    size_t hdr_len = modern ? sizeof(struct virtio_net_hdr_v1) :
    sizeof(struct virtio_net_hdr);
    struct virtio_net_config *vio_config = vdpasim.config;
    struct vdpasim_net *net = sim_to_net(vdpasim);
    if (len < ETH_ALEN + hdr_len)
    return false;
    if (is_broadcast_ether_addr(net.buffer + hdr_len) ||
    is_multicast_ether_addr(net.buffer + hdr_len))
    return true;
    if (!strncmp(net.buffer + hdr_len, vio_config.mac, ETH_ALEN))
    return true;
    return false;
    }
    static virtio_net_ctrl_ack vdpasim_handle_ctrl_mac(struct vdpasim *vdpasim,
    u8 cmd)
    {
    struct virtio_net_config *vio_config = vdpasim.config;
    struct vdpasim_virtqueue *cvq = &vdpasim.vqs[2];
    let mut status: virtio_net_ctrl_ack = VIRTIO_NET_ERR;
    size_t read;
    switch (cmd) {
    case VIRTIO_NET_CTRL_MAC_ADDR_SET:
    read = vringh_iov_pull_iotlb(&cvq.vring, &cvq.in_iov,
    vio_config.mac, ETH_ALEN);
    if (read == ETH_ALEN)
    status = VIRTIO_NET_OK;
    break;
    default:
    break;
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_handle_cvq(vdpasim: *mut vdpasim) {
    static void vdpasim_handle_cvq(struct vdpasim *vdpasim)
    {
    struct vdpasim_virtqueue *cvq = &vdpasim.vqs[2];
    struct vdpasim_net *net = sim_to_net(vdpasim);
    let mut status: virtio_net_ctrl_ack = VIRTIO_NET_ERR;
    struct virtio_net_ctrl_hdr ctrl;
    size_t read, write;
    let mut requests: u64 = 0, errors = 0, successes = 0;
    int err;
    if (!(vdpasim.features & (1ULL << VIRTIO_NET_F_CTRL_VQ)))
    return;
    if (!cvq.ready)
    return;
    while (true) {
    err = vringh_getdesc_iotlb(&cvq.vring, &cvq.in_iov,
    &cvq.out_iov,
    &cvq.head, GFP_ATOMIC);
    if (err <= 0)
    break;
    ++requests;
    read = vringh_iov_pull_iotlb(&cvq.vring, &cvq.in_iov, &ctrl,
    sizeof(ctrl));
    if (read != sizeof(ctrl)) {
    ++errors;
    break;
    }
    switch (ctrl.class) {
    case VIRTIO_NET_CTRL_MAC:
    status = vdpasim_handle_ctrl_mac(vdpasim, ctrl.cmd);
    break;
    default:
    break;
    }
    if (status == VIRTIO_NET_OK)
    ++successes;
    else
    ++errors;
// Make sure data is wrote before advancing index
    smp_wmb();
    write = vringh_iov_push_iotlb(&cvq.vring, &cvq.out_iov,
    &status, sizeof(status));
    vringh_complete_iotlb(&cvq.vring, cvq.head, write);
    vringh_kiov_cleanup(&cvq.in_iov);
    vringh_kiov_cleanup(&cvq.out_iov);
// Make sure used is visible before rasing the interrupt.
    smp_wmb();
    local_bh_disable();
    if (cvq.cb)
    cvq.cb(cvq.private);
    local_bh_enable();
    }
    u64_stats_update_begin(&net.cq_stats.syncp);
    net.cq_stats.requests += requests;
    net.cq_stats.errors += errors;
    net.cq_stats.successes += successes;
    u64_stats_update_end(&net.cq_stats.syncp);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_net_work(vdpasim: *mut vdpasim) {
    static void vdpasim_net_work(struct vdpasim *vdpasim)
    {
    struct vdpasim_virtqueue *txq = &vdpasim.vqs[1];
    struct vdpasim_virtqueue *rxq = &vdpasim.vqs[0];
    struct vdpasim_net *net = sim_to_net(vdpasim);
    ssize_t read, write;
    let mut tx_pkts: u64 = 0, rx_pkts = 0, tx_bytes = 0, rx_bytes = 0;
    let mut rx_drops: u64 = 0, rx_overruns = 0, rx_errors = 0, tx_errors = 0;
    int err;
    mutex_lock(&vdpasim.mutex);
    if (!vdpasim.running)
    goto out;
    if (!(vdpasim.status & VIRTIO_CONFIG_S_DRIVER_OK))
    goto out;
    vdpasim_handle_cvq(vdpasim);
    if (!txq.ready || !rxq.ready)
    goto out;
    while (true) {
    err = vringh_getdesc_iotlb(&txq.vring, &txq.out_iov, core::ptr::null_mut(),
    &txq.head, GFP_ATOMIC);
    if (err <= 0) {
    if (err)
    ++tx_errors;
    break;
    }
    ++tx_pkts;
    read = vringh_iov_pull_iotlb(&txq.vring, &txq.out_iov,
    net.buffer, PAGE_SIZE);
    tx_bytes += read;
    if (!receive_filter(vdpasim, read)) {
    ++rx_drops;
    vdpasim_net_complete(txq, 0);
    continue;
    }
    err = vringh_getdesc_iotlb(&rxq.vring, core::ptr::null_mut(), &rxq.in_iov,
    &rxq.head, GFP_ATOMIC);
    if (err <= 0) {
    ++rx_overruns;
    vdpasim_net_complete(txq, 0);
    break;
    }
    write = vringh_iov_push_iotlb(&rxq.vring, &rxq.in_iov,
    net.buffer, read);
    if (write <= 0) {
    ++rx_errors;
    break;
    }
    ++rx_pkts;
    rx_bytes += write;
    vdpasim_net_complete(txq, 0);
    vdpasim_net_complete(rxq, write);
    if (tx_pkts > 4) {
    vdpasim_schedule_work(vdpasim);
    goto out;
    }
    }
    out:
    mutex_unlock(&vdpasim.mutex);
    u64_stats_update_begin(&net.tx_stats.syncp);
    net.tx_stats.pkts += tx_pkts;
    net.tx_stats.bytes += tx_bytes;
    net.tx_stats.errors += tx_errors;
    u64_stats_update_end(&net.tx_stats.syncp);
    u64_stats_update_begin(&net.rx_stats.syncp);
    net.rx_stats.pkts += rx_pkts;
    net.rx_stats.bytes += rx_bytes;
    net.rx_stats.drops += rx_drops;
    net.rx_stats.errors += rx_errors;
    net.rx_stats.overruns += rx_overruns;
    u64_stats_update_end(&net.rx_stats.syncp);
    }
    static int vdpasim_net_get_stats(struct vdpasim *vdpasim, u16 idx,
    struct sk_buff *msg,
    struct netlink_ext_ack *extack)
    {
    struct vdpasim_net *net = sim_to_net(vdpasim);
    u64 rx_pkts, rx_bytes, rx_errors, rx_overruns, rx_drops;
    u64 tx_pkts, tx_bytes, tx_errors, tx_drops;
    u64 cq_requests, cq_successes, cq_errors;
    unsigned int start;
    let mut err: c_int = -EMSGSIZE;
    switch(idx) {
    case 0:
    do {
    start = u64_stats_fetch_begin(&net.rx_stats.syncp);
    rx_pkts = net.rx_stats.pkts;
    rx_bytes = net.rx_stats.bytes;
    rx_errors = net.rx_stats.errors;
    rx_overruns = net.rx_stats.overruns;
    rx_drops = net.rx_stats.drops;
    } while (u64_stats_fetch_retry(&net.rx_stats.syncp, start));
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "rx packets"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    rx_pkts, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "rx bytes"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    rx_bytes, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "rx errors"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    rx_errors, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "rx overruns"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    rx_overruns, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "rx drops"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    rx_drops, VDPA_ATTR_PAD))
    break;
    err = 0;
    break;
    case 1:
    do {
    start = u64_stats_fetch_begin(&net.tx_stats.syncp);
    tx_pkts = net.tx_stats.pkts;
    tx_bytes = net.tx_stats.bytes;
    tx_errors = net.tx_stats.errors;
    tx_drops = net.tx_stats.drops;
    } while (u64_stats_fetch_retry(&net.tx_stats.syncp, start));
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "tx packets"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    tx_pkts, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "tx bytes"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    tx_bytes, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "tx errors"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    tx_errors, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "tx drops"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    tx_drops, VDPA_ATTR_PAD))
    break;
    err = 0;
    break;
    case 2:
    do {
    start = u64_stats_fetch_begin(&net.cq_stats.syncp);
    cq_requests = net.cq_stats.requests;
    cq_successes = net.cq_stats.successes;
    cq_errors = net.cq_stats.errors;
    } while (u64_stats_fetch_retry(&net.cq_stats.syncp, start));
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "cvq requests"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    cq_requests, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "cvq successes"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    cq_successes, VDPA_ATTR_PAD))
    break;
    if (nla_put_string(msg, VDPA_ATTR_DEV_VENDOR_ATTR_NAME,
    "cvq errors"))
    break;
    if (nla_put_u64_64bit(msg, VDPA_ATTR_DEV_VENDOR_ATTR_VALUE,
    cq_errors, VDPA_ATTR_PAD))
    break;
    err = 0;
    break;
    default:
    err = -EINVAL;
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_net_get_config(vdpasim: *mut vdpasim, config: *mut c_void) {
    static void vdpasim_net_get_config(struct vdpasim *vdpasim, void *config)
    {
    struct virtio_net_config *net_config = config;
    net_config.status = cpu_to_vdpasim16(vdpasim, VIRTIO_NET_S_LINK_UP);
    }
    static int vdpasim_net_set_attr(struct vdpa_mgmt_dev *mdev, struct vdpa_device *dev,
    const struct vdpa_dev_set_config *config)
    {
    struct vdpasim *vdpasim = container_of(dev, struct vdpasim, vdpa);
    struct virtio_net_config *vio_config = vdpasim.config;
    mutex_lock(&vdpasim.mutex);
    if (config.mask & (1 << VDPA_ATTR_DEV_NET_CFG_MACADDR)) {
    ether_addr_copy(vio_config.mac, config.net.mac);
    mutex_unlock(&vdpasim.mutex);
    return 0;
    }
    mutex_unlock(&vdpasim.mutex);
    return -EOPNOTSUPP;
    }
    static void vdpasim_net_setup_config(struct vdpasim *vdpasim,
    const struct vdpa_dev_set_config *config)
    {
    struct virtio_net_config *vio_config = vdpasim.config;
    if (config.mask & (1 << VDPA_ATTR_DEV_NET_CFG_MACADDR))
    memcpy(vio_config.mac, config.net.mac, ETH_ALEN);
    if (config.mask & (1 << VDPA_ATTR_DEV_NET_CFG_MTU))
    vio_config.mtu = cpu_to_vdpasim16(vdpasim, config.net.mtu);
    else
// Setup default MTU to be 1500
    vio_config.mtu = cpu_to_vdpasim16(vdpasim, 1500);
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_net_free(vdpasim: *mut vdpasim) {
    static void vdpasim_net_free(struct vdpasim *vdpasim)
    {
    struct vdpasim_net *net = sim_to_net(vdpasim);
    kvfree(net.buffer);
    }
    static struct device *vdpasim_net_mgmtdev;
    static int vdpasim_net_dev_add(struct vdpa_mgmt_dev *mdev, const char *name,
    const struct vdpa_dev_set_config *config)
    {
    let mut dev_attr: vdpasim_dev_attr = {};
    struct vdpasim_net *net;
    struct vdpasim *simdev;
    int ret;
    dev_attr.mgmt_dev = mdev;
    dev_attr.name = name;
    dev_attr.id = VIRTIO_ID_NET;
    dev_attr.supported_features = VDPASIM_NET_FEATURES;
    dev_attr.nvqs = VDPASIM_NET_VQ_NUM;
    dev_attr.ngroups = VDPASIM_NET_GROUP_NUM;
    dev_attr.nas = VDPASIM_NET_AS_NUM;
    dev_attr.alloc_size = sizeof(struct vdpasim_net);
    dev_attr.config_size = sizeof(struct virtio_net_config);
    dev_attr.get_config = vdpasim_net_get_config;
    dev_attr.work_fn = vdpasim_net_work;
    dev_attr.get_stats = vdpasim_net_get_stats;
    dev_attr.free = vdpasim_net_free;
    simdev = vdpasim_create(&dev_attr, config);
    if (IS_ERR(simdev))
    return PTR_ERR(simdev);
    vdpasim_net_setup_config(simdev, config);
    net = sim_to_net(simdev);
    u64_stats_init(&net.tx_stats.syncp);
    u64_stats_init(&net.rx_stats.syncp);
    u64_stats_init(&net.cq_stats.syncp);
    net.buffer = kvmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!net.buffer) {
    ret = -ENOMEM;
    goto reg_err;
    }
//
// Initialization must be completed before this call, since it can
// connect the device to the vDPA bus, so requests can arrive after
// this call.
//
    ret = _vdpa_register_device(&simdev.vdpa, VDPASIM_NET_VQ_NUM);
    if (ret)
    goto reg_err;
    return 0;
    reg_err:
    put_device(&simdev.vdpa.dev);
    return ret;
    }
    static void vdpasim_net_dev_del(struct vdpa_mgmt_dev *mdev,
    struct vdpa_device *dev)
    {
    struct vdpasim *simdev = container_of(dev, struct vdpasim, vdpa);
    _vdpa_unregister_device(&simdev.vdpa);
    }
    static const struct vdpa_mgmtdev_ops vdpasim_net_mgmtdev_ops = {
    .dev_add = vdpasim_net_dev_add,
    .dev_del = vdpasim_net_dev_del,
    .dev_set_attr = vdpasim_net_set_attr
    };
    static struct virtio_device_id id_table[] = {
    { VIRTIO_ID_NET, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    static struct vdpa_mgmt_dev mgmt_dev = {
    .id_table = id_table,
    .ops = &vdpasim_net_mgmtdev_ops,
    .config_attr_mask = (1 << VDPA_ATTR_DEV_NET_CFG_MACADDR |
    1 << VDPA_ATTR_DEV_NET_CFG_MTU |
    1 << VDPA_ATTR_DEV_FEATURES),
    .max_supported_vqs = VDPASIM_NET_VQ_NUM,
    .supported_features = VDPASIM_NET_FEATURES,
    };
#[no_mangle]
unsafe extern "C" fn vdpasim_net_init() -> int __init {
    static int __init vdpasim_net_init(void)
    {
    int ret;
    vdpasim_net_mgmtdev = root_device_register("vdpasim_net");
    if (IS_ERR(vdpasim_net_mgmtdev))
    return PTR_ERR(vdpasim_net_mgmtdev);
    mgmt_dev.device = vdpasim_net_mgmtdev;
    ret = vdpa_mgmtdev_register(&mgmt_dev);
    if (ret)
    goto parent_err;
    return 0;
    parent_err:
    root_device_unregister(vdpasim_net_mgmtdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vdpasim_net_exit() -> void __exit {
    static void __exit vdpasim_net_exit(void)
    {
    vdpa_mgmtdev_unregister(&mgmt_dev);
    root_device_unregister(vdpasim_net_mgmtdev);
    }
    module_init(vdpasim_net_init);
    module_exit(vdpasim_net_exit);
    MODULE_VERSION(DRV_VERSION);
    MODULE_LICENSE(DRV_LICENSE);
    MODULE_AUTHOR(DRV_AUTHOR);
    MODULE_DESCRIPTION(DRV_DESC);
