//! Automatically rewritten from C to Rust
//! Source: drivers/net/rionet.c
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
// rionet - Ethernet driver over RapidIO messaging services
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

    MODULE_AUTHOR(DRV_AUTHOR);
    MODULE_DESCRIPTION(DRV_DESC);
    MODULE_LICENSE("GPL");

    (NETIF_MSG_DRV          | \
    NETIF_MSG_LINK         | \
    NETIF_MSG_RX_ERR       | \
    NETIF_MSG_TX_ERR)
pub const RIONET_DOORBELL_JOIN: c_uint = 0x1000;
pub const RIONET_DOORBELL_LEAVE: c_uint = 0x1001;
pub const RIONET_MAILBOX: c_int = 0;

pub const RIONET_MAX_NETS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rionet_private {
    pub mport: *mut rio_mport,
    pub rx_skb: [*mut sk_buff; RIONET_RX_RING_SIZE],
    pub tx_skb: [*mut sk_buff; RIONET_TX_RING_SIZE],
    pub rx_slot: c_int,
    pub tx_slot: c_int,
    pub tx_cnt: c_int,
    pub ack_slot: c_int,
    pub lock: spinlock_t,
    pub tx_lock: spinlock_t,
    pub msg_enable: u32,
    pub open: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rionet_peer {
    pub node: list_head,
    pub rdev: *mut rio_dev,
    pub res: *mut resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rionet_net {
    pub ndev: *mut net_device,
    pub peers: list_head,
    pub /: *mut *mut spinlock_t lock; / net info access lock,
    pub active: *mut rio_dev,
    pub /: *mut *mut int nact; / number of active peers,
}

    static struct rionet_net nets[RIONET_MAX_NETS];

    ((src_ops & RIO_SRC_OPS_DATA_MSG) &&	\
    (dst_ops & RIO_DST_OPS_DATA_MSG) &&	\
    (src_ops & RIO_SRC_OPS_DOORBELL) &&	\
    (dst_ops & RIO_DST_OPS_DOORBELL))

    is_rionet_capable(dev.src_ops, dev.dst_ops)

#[no_mangle]
unsafe extern "C" fn rionet_rx_clean(ndev: *mut net_device) -> c_int {
    static int rionet_rx_clean(struct net_device *ndev)
    {
    int i;
    let mut error: c_int = 0;
    struct rionet_private *rnet = netdev_priv(ndev);
    void *data;
    i = rnet.rx_slot;
    do {
    if (!rnet.rx_skb[i])
    continue;
    if (!(data = rio_get_inb_message(rnet.mport, RIONET_MAILBOX)))
    break;
    rnet.rx_skb[i].data = data;
    skb_put(rnet.rx_skb[i], RIO_MAX_MSG_SIZE);
    rnet.rx_skb[i].protocol =
    eth_type_trans(rnet.rx_skb[i], ndev);
    error = __netif_rx(rnet.rx_skb[i]);
    if (error == NET_RX_DROP) {
    ndev.stats.rx_dropped++;
    } else {
    ndev.stats.rx_packets++;
    ndev.stats.rx_bytes += RIO_MAX_MSG_SIZE;
    }
    } while ((i = (i + 1) % RIONET_RX_RING_SIZE) != rnet.rx_slot);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn rionet_rx_fill(ndev: *mut net_device, end: c_int) {
    static void rionet_rx_fill(struct net_device *ndev, int end)
    {
    int i;
    struct rionet_private *rnet = netdev_priv(ndev);
    i = rnet.rx_slot;
    do {
    rnet.rx_skb[i] = dev_alloc_skb(RIO_MAX_MSG_SIZE);
    if (!rnet.rx_skb[i])
    break;
    rio_add_inb_buffer(rnet.mport, RIONET_MAILBOX,
    rnet.rx_skb[i].data);
    } while ((i = (i + 1) % RIONET_RX_RING_SIZE) != end);
    rnet.rx_slot = i;
    }
    static int rionet_queue_tx_msg(struct sk_buff *skb, struct net_device *ndev,
    struct rio_dev *rdev)
    {
    struct rionet_private *rnet = netdev_priv(ndev);
    rio_add_outb_message(rnet.mport, rdev, 0, skb.data, skb.len);
    rnet.tx_skb[rnet.tx_slot] = skb;
    ndev.stats.tx_packets++;
    ndev.stats.tx_bytes += skb.len;
    if (++rnet.tx_cnt == RIONET_TX_RING_SIZE)
    netif_stop_queue(ndev);
    ++rnet.tx_slot;
    rnet.tx_slot &= (RIONET_TX_RING_SIZE - 1);
    if (netif_msg_tx_queued(rnet))
    printk(KERN_INFO "%s: queued skb len %8.8x\n", DRV_NAME,
    skb.len);
    return 0;
    }
    static netdev_tx_t rionet_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    int i;
    struct rionet_private *rnet = netdev_priv(ndev);
    struct ethhdr *eth = (struct ethhdr *)skb.data;
    u16 destid;
    unsigned long flags;
    let mut add_num: c_int = 1;
    spin_lock_irqsave(&rnet.tx_lock, flags);
    if (is_multicast_ether_addr(eth.h_dest))
    add_num = nets[rnet.mport.id].nact;
    if ((rnet.tx_cnt + add_num) > RIONET_TX_RING_SIZE) {
    netif_stop_queue(ndev);
    spin_unlock_irqrestore(&rnet.tx_lock, flags);
    printk(KERN_ERR "%s: BUG! Tx Ring full when queue awake!\n",
    ndev.name);
    return NETDEV_TX_BUSY;
    }
    if (is_multicast_ether_addr(eth.h_dest)) {
    let mut count: c_int = 0;
    for (i = 0; i < RIO_MAX_ROUTE_ENTRIES(rnet.mport.sys_size);
    i++)
    if (nets[rnet.mport.id].active[i]) {
    rionet_queue_tx_msg(skb, ndev,
    nets[rnet.mport.id].active[i]);
    if (count)
    refcount_inc(&skb.users);
    count++;
    }
    } else if (RIONET_MAC_MATCH(eth.h_dest)) {
    destid = RIONET_GET_DESTID(eth.h_dest);
    if (nets[rnet.mport.id].active[destid])
    rionet_queue_tx_msg(skb, ndev,
    nets[rnet.mport.id].active[destid]);
    else {
//
// If the target device was removed from the list of
// active peers but we still have TX packets targeting
// it just report sending a packet to the target
// (without actual packet transfer).
//
    ndev.stats.tx_packets++;
    ndev.stats.tx_bytes += skb.len;
    dev_kfree_skb_any(skb);
    }
    }
    spin_unlock_irqrestore(&rnet.tx_lock, flags);
    return NETDEV_TX_OK;
    }
    static void rionet_dbell_event(struct rio_mport *mport, void *dev_id, u16 sid, u16 tid,
    u16 info)
    {
    struct net_device *ndev = dev_id;
    struct rionet_private *rnet = netdev_priv(ndev);
    struct rionet_peer *peer;
    let mut netid: c_uchar = rnet.mport.id;
    if (netif_msg_intr(rnet))
    printk(KERN_INFO "%s: doorbell sid %4.4x tid %4.4x info %4.4x",
    DRV_NAME, sid, tid, info);
    if (info == RIONET_DOORBELL_JOIN) {
    if (!nets[netid].active[sid]) {
    spin_lock(&nets[netid].lock);
    list_for_each_entry(peer, &nets[netid].peers, node) {
    if (peer.rdev.destid == sid) {
    nets[netid].active[sid] = peer.rdev;
    nets[netid].nact++;
    }
    }
    spin_unlock(&nets[netid].lock);
    rio_mport_send_doorbell(mport, sid,
    RIONET_DOORBELL_JOIN);
    }
    } else if (info == RIONET_DOORBELL_LEAVE) {
    spin_lock(&nets[netid].lock);
    if (nets[netid].active[sid]) {
    nets[netid].active[sid] = core::ptr::null_mut();
    nets[netid].nact--;
    }
    spin_unlock(&nets[netid].lock);
    } else {
    if (netif_msg_intr(rnet))
    printk(KERN_WARNING "%s: unhandled doorbell\n",
    DRV_NAME);
    }
    }
#[no_mangle]
unsafe extern "C" fn rionet_inb_msg_event(mport: *mut rio_mport, dev_id: *mut c_void, mbox: c_int, slot: c_int) {
    static void rionet_inb_msg_event(struct rio_mport *mport, void *dev_id, int mbox, int slot)
    {
    int n;
    struct net_device *ndev = dev_id;
    struct rionet_private *rnet = netdev_priv(ndev);
    if (netif_msg_intr(rnet))
    printk(KERN_INFO "%s: inbound message event, mbox %d slot %d\n",
    DRV_NAME, mbox, slot);
    spin_lock(&rnet.lock);
    if ((n = rionet_rx_clean(ndev)) != rnet.rx_slot)
    rionet_rx_fill(ndev, n);
    spin_unlock(&rnet.lock);
    }
#[no_mangle]
unsafe extern "C" fn rionet_outb_msg_event(mport: *mut rio_mport, dev_id: *mut c_void, mbox: c_int, slot: c_int) {
    static void rionet_outb_msg_event(struct rio_mport *mport, void *dev_id, int mbox, int slot)
    {
    struct net_device *ndev = dev_id;
    struct rionet_private *rnet = netdev_priv(ndev);
    spin_lock(&rnet.tx_lock);
    if (netif_msg_intr(rnet))
    printk(KERN_INFO
    "%s: outbound message event, mbox %d slot %d\n",
    DRV_NAME, mbox, slot);
    while (rnet.tx_cnt && (rnet.ack_slot != slot)) {
// dma unmap single
    dev_kfree_skb_irq(rnet.tx_skb[rnet.ack_slot]);
    rnet.tx_skb[rnet.ack_slot] = core::ptr::null_mut();
    ++rnet.ack_slot;
    rnet.ack_slot &= (RIONET_TX_RING_SIZE - 1);
    rnet.tx_cnt--;
    }
    if (rnet.tx_cnt < RIONET_TX_RING_SIZE)
    netif_wake_queue(ndev);
    spin_unlock(&rnet.tx_lock);
    }
#[no_mangle]
unsafe extern "C" fn rionet_open(ndev: *mut net_device) -> c_int {
    static int rionet_open(struct net_device *ndev)
    {
    int i, rc = 0;
    struct rionet_peer *peer;
    struct rionet_private *rnet = netdev_priv(ndev);
    let mut netid: c_uchar = rnet.mport.id;
    unsigned long flags;
    if (netif_msg_ifup(rnet))
    printk(KERN_INFO "%s: open\n", DRV_NAME);
    if ((rc = rio_request_inb_dbell(rnet.mport,
    (void *)ndev,
    RIONET_DOORBELL_JOIN,
    RIONET_DOORBELL_LEAVE,
    rionet_dbell_event)) < 0)
    goto out;
    if ((rc = rio_request_inb_mbox(rnet.mport,
    (void *)ndev,
    RIONET_MAILBOX,
    RIONET_RX_RING_SIZE,
    rionet_inb_msg_event)) < 0)
    goto out;
    if ((rc = rio_request_outb_mbox(rnet.mport,
    (void *)ndev,
    RIONET_MAILBOX,
    RIONET_TX_RING_SIZE,
    rionet_outb_msg_event)) < 0)
    goto out;
// Initialize inbound message ring
    for (i = 0; i < RIONET_RX_RING_SIZE; i++)
    rnet.rx_skb[i] = core::ptr::null_mut();
    rnet.rx_slot = 0;
    rionet_rx_fill(ndev, 0);
    rnet.tx_slot = 0;
    rnet.tx_cnt = 0;
    rnet.ack_slot = 0;
    netif_carrier_on(ndev);
    netif_start_queue(ndev);
    spin_lock_irqsave(&nets[netid].lock, flags);
    list_for_each_entry(peer, &nets[netid].peers, node) {
// Send a join message
    rio_send_doorbell(peer.rdev, RIONET_DOORBELL_JOIN);
    }
    spin_unlock_irqrestore(&nets[netid].lock, flags);
    rnet.open = true;
    out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn rionet_close(ndev: *mut net_device) -> c_int {
    static int rionet_close(struct net_device *ndev)
    {
    struct rionet_private *rnet = netdev_priv(ndev);
    struct rionet_peer *peer;
    let mut netid: c_uchar = rnet.mport.id;
    unsigned long flags;
    int i;
    if (netif_msg_ifup(rnet))
    printk(KERN_INFO "%s: close %s\n", DRV_NAME, ndev.name);
    netif_stop_queue(ndev);
    netif_carrier_off(ndev);
    rnet.open = false;
    for (i = 0; i < RIONET_RX_RING_SIZE; i++)
    kfree_skb(rnet.rx_skb[i]);
    spin_lock_irqsave(&nets[netid].lock, flags);
    list_for_each_entry(peer, &nets[netid].peers, node) {
    if (nets[netid].active[peer.rdev.destid]) {
    rio_send_doorbell(peer.rdev, RIONET_DOORBELL_LEAVE);
    nets[netid].active[peer.rdev.destid] = core::ptr::null_mut();
    }
    if (peer.res)
    rio_release_outb_dbell(peer.rdev, peer.res);
    }
    spin_unlock_irqrestore(&nets[netid].lock, flags);
    rio_release_inb_dbell(rnet.mport, RIONET_DOORBELL_JOIN,
    RIONET_DOORBELL_LEAVE);
    rio_release_inb_mbox(rnet.mport, RIONET_MAILBOX);
    rio_release_outb_mbox(rnet.mport, RIONET_MAILBOX);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rionet_remove_dev(dev: *mut device, sif: *mut subsys_interface) {
    static void rionet_remove_dev(struct device *dev, struct subsys_interface *sif)
    {
    struct rio_dev *rdev = to_rio_dev(dev);
    let mut netid: c_uchar = rdev.net.hport.id;
    struct rionet_peer *peer;
    int state, found = 0;
    unsigned long flags;
    if (!dev_rionet_capable(rdev))
    return;
    spin_lock_irqsave(&nets[netid].lock, flags);
    list_for_each_entry(peer, &nets[netid].peers, node) {
    if (peer.rdev == rdev) {
    list_del(&peer.node);
    if (nets[netid].active[rdev.destid]) {
    state = atomic_read(&rdev.state);
    if (state != RIO_DEVICE_GONE &&
    state != RIO_DEVICE_INITIALIZING) {
    rio_send_doorbell(rdev,
    RIONET_DOORBELL_LEAVE);
    }
    nets[netid].active[rdev.destid] = core::ptr::null_mut();
    nets[netid].nact--;
    }
    found = 1;
    break;
    }
    }
    spin_unlock_irqrestore(&nets[netid].lock, flags);
    if (found) {
    if (peer.res)
    rio_release_outb_dbell(rdev, peer.res);
    kfree(peer);
    }
    }
    static void rionet_get_drvinfo(struct net_device *ndev,
    struct ethtool_drvinfo *info)
    {
    struct rionet_private *rnet = netdev_priv(ndev);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    strscpy(info.version, DRV_VERSION, sizeof(info.version));
    strscpy(info.fw_version, "n/a", sizeof(info.fw_version));
    strscpy(info.bus_info, rnet.mport.name, sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn rionet_get_msglevel(ndev: *mut net_device) -> u32 {
    static u32 rionet_get_msglevel(struct net_device *ndev)
    {
    struct rionet_private *rnet = netdev_priv(ndev);
    return rnet.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn rionet_set_msglevel(ndev: *mut net_device, value: u32) {
    static void rionet_set_msglevel(struct net_device *ndev, u32 value)
    {
    struct rionet_private *rnet = netdev_priv(ndev);
    rnet.msg_enable = value;
    }
    static const struct ethtool_ops rionet_ethtool_ops = {
    .get_drvinfo = rionet_get_drvinfo,
    .get_msglevel = rionet_get_msglevel,
    .set_msglevel = rionet_set_msglevel,
    .get_link = ethtool_op_get_link,
    };
    static const struct net_device_ops rionet_netdev_ops = {
    .ndo_open		= rionet_open,
    .ndo_stop		= rionet_close,
    .ndo_start_xmit		= rionet_start_xmit,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,
    };
#[no_mangle]
unsafe extern "C" fn rionet_setup_netdev(mport: *mut rio_mport, ndev: *mut net_device) -> c_int {
    static int rionet_setup_netdev(struct rio_mport *mport, struct net_device *ndev)
    {
    let mut rc: c_int = 0;
    struct rionet_private *rnet;
    u8 addr[ETH_ALEN];
    u16 device_id;
    const size_t rionet_active_bytes = sizeof(void *) *
    RIO_MAX_ROUTE_ENTRIES(mport.sys_size);
    nets[mport.id].active = (struct rio_dev **)__get_free_pages(GFP_KERNEL,
    get_order(rionet_active_bytes));
    if (!nets[mport.id].active) {
    rc = -ENOMEM;
    goto out;
    }
    memset((void *)nets[mport.id].active, 0, rionet_active_bytes);
// Set up private area
    rnet = netdev_priv(ndev);
    rnet.mport = mport;
    rnet.open = false;
// Set the default MAC address
    device_id = rio_local_get_device_id(mport);
    addr[0] = 0x00;
    addr[1] = 0x01;
    addr[2] = 0x00;
    addr[3] = 0x01;
    addr[4] = device_id >> 8;
    addr[5] = device_id & 0xff;
    eth_hw_addr_set(ndev, addr);
    ndev.netdev_ops = &rionet_netdev_ops;
    ndev.mtu = RIONET_MAX_MTU;
// MTU range: 68 - 4082
    ndev.min_mtu = ETH_MIN_MTU;
    ndev.max_mtu = RIONET_MAX_MTU;
    ndev.lltx = true;
    SET_NETDEV_DEV(ndev, &mport.dev);
    ndev.ethtool_ops = &rionet_ethtool_ops;
    spin_lock_init(&rnet.lock);
    spin_lock_init(&rnet.tx_lock);
    rnet.msg_enable = RIONET_DEFAULT_MSGLEVEL;
    rc = register_netdev(ndev);
    if (rc != 0) {
    free_pages((unsigned long)nets[mport.id].active,
    get_order(rionet_active_bytes));
    goto out;
    }
    printk(KERN_INFO "%s: %s %s Version %s, MAC %pM, %s\n",
    ndev.name,
    DRV_NAME,
    DRV_DESC,
    DRV_VERSION,
    ndev.dev_addr,
    mport.name);
    out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn rionet_add_dev(dev: *mut device, sif: *mut subsys_interface) -> c_int {
    static int rionet_add_dev(struct device *dev, struct subsys_interface *sif)
    {
    let mut rc: c_int = -ENODEV;
    u32 lsrc_ops, ldst_ops;
    struct rionet_peer *peer;
    struct net_device *ndev = core::ptr::null_mut();
    struct rio_dev *rdev = to_rio_dev(dev);
    let mut netid: c_uchar = rdev.net.hport.id;
    if (netid >= RIONET_MAX_NETS)
    return rc;
//
// If first time through this net, make sure local device is rionet
// capable and setup netdev (this step will be skipped in later probes
// on the same net).
//
    if (!nets[netid].ndev) {
    rio_local_read_config_32(rdev.net.hport, RIO_SRC_OPS_CAR,
    &lsrc_ops);
    rio_local_read_config_32(rdev.net.hport, RIO_DST_OPS_CAR,
    &ldst_ops);
    if (!is_rionet_capable(lsrc_ops, ldst_ops)) {
    printk(KERN_ERR
    "%s: local device %s is not network capable\n",
    DRV_NAME, rdev.net.hport.name);
    goto out;
    }
// Allocate our net_device structure
    ndev = alloc_etherdev(sizeof(struct rionet_private));
    if (ndev == core::ptr::null_mut()) {
    rc = -ENOMEM;
    goto out;
    }
    rc = rionet_setup_netdev(rdev.net.hport, ndev);
    if (rc) {
    printk(KERN_ERR "%s: failed to setup netdev (rc=%d)\n",
    DRV_NAME, rc);
    free_netdev(ndev);
    goto out;
    }
    INIT_LIST_HEAD(&nets[netid].peers);
    spin_lock_init(&nets[netid].lock);
    nets[netid].nact = 0;
    nets[netid].ndev = ndev;
    }
//
// If the remote device has mailbox/doorbell capabilities,
// add it to the peer list.
//
    if (dev_rionet_capable(rdev)) {
    struct rionet_private *rnet;
    unsigned long flags;
    rnet = netdev_priv(nets[netid].ndev);
    peer = kzalloc_obj(*peer);
    if (!peer) {
    rc = -ENOMEM;
    goto out;
    }
    peer.rdev = rdev;
    peer.res = rio_request_outb_dbell(peer.rdev,
    RIONET_DOORBELL_JOIN,
    RIONET_DOORBELL_LEAVE);
    if (!peer.res) {
    pr_err("%s: error requesting doorbells\n", DRV_NAME);
    kfree(peer);
    rc = -ENOMEM;
    goto out;
    }
    spin_lock_irqsave(&nets[netid].lock, flags);
    list_add_tail(&peer.node, &nets[netid].peers);
    spin_unlock_irqrestore(&nets[netid].lock, flags);
    pr_debug("%s: %s add peer %s\n",
    DRV_NAME, __func__, rio_name(rdev));
// If netdev is already opened, send join request to new peer
    if (rnet.open)
    rio_send_doorbell(peer.rdev, RIONET_DOORBELL_JOIN);
    }
    return 0;
    out:
    return rc;
    }
    static int rionet_shutdown(struct notifier_block *nb, unsigned long code,
    void *unused)
    {
    struct rionet_peer *peer;
    unsigned long flags;
    int i;
    pr_debug("%s: %s\n", DRV_NAME, __func__);
    for (i = 0; i < RIONET_MAX_NETS; i++) {
    if (!nets[i].ndev)
    continue;
    spin_lock_irqsave(&nets[i].lock, flags);
    list_for_each_entry(peer, &nets[i].peers, node) {
    if (nets[i].active[peer.rdev.destid]) {
    rio_send_doorbell(peer.rdev,
    RIONET_DOORBELL_LEAVE);
    nets[i].active[peer.rdev.destid] = core::ptr::null_mut();
    }
    }
    spin_unlock_irqrestore(&nets[i].lock, flags);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn rionet_remove_mport(dev: *mut device) {
    static void rionet_remove_mport(struct device *dev)
    {
    struct rio_mport *mport = to_rio_mport(dev);
    struct net_device *ndev;
    let mut id: c_int = mport.id;
    pr_debug("%s %s\n", __func__, mport.name);
    WARN(nets[id].nact, "%s called when connected to %d peers\n",
    __func__, nets[id].nact);
    WARN(!nets[id].ndev, "%s called for mport without NDEV\n",
    __func__);
    if (nets[id].ndev) {
    ndev = nets[id].ndev;
    netif_stop_queue(ndev);
    unregister_netdev(ndev);
    free_pages((unsigned long)nets[id].active,
    get_order(sizeof(void *) *
    RIO_MAX_ROUTE_ENTRIES(mport.sys_size)));
    nets[id].active = core::ptr::null_mut();
    free_netdev(ndev);
    nets[id].ndev = core::ptr::null_mut();
    }
    }

    static struct rio_device_id rionet_id_table[] = {
    {RIO_DEVICE(RIO_ANY_ID, RIO_ANY_ID)},
    { 0, }	/* terminate list */
    };
    MODULE_DEVICE_TABLE(rapidio, rionet_id_table);

    static struct subsys_interface rionet_interface = {
    .name		= "rionet",
    .subsys		= &rio_bus_type,
    .add_dev	= rionet_add_dev,
    .remove_dev	= rionet_remove_dev,
    };
    static struct notifier_block rionet_notifier = {
    .notifier_call = rionet_shutdown,
    };
// the rio_mport_interface is used to handle local mport devices
    static struct class_interface rio_mport_interface __refdata = {
    .class = &rio_mport_class,
    .add_dev = core::ptr::null_mut(),
    .remove_dev = rionet_remove_mport,
    };
#[no_mangle]
unsafe extern "C" fn rionet_init() -> int __init {
    static int __init rionet_init(void)
    {
    int ret;
    ret = register_reboot_notifier(&rionet_notifier);
    if (ret) {
    pr_err("%s: failed to register reboot notifier (err=%d)\n",
    DRV_NAME, ret);
    return ret;
    }
    ret = class_interface_register(&rio_mport_interface);
    if (ret) {
    pr_err("%s: class_interface_register error: %d\n",
    DRV_NAME, ret);
    return ret;
    }
    return subsys_interface_register(&rionet_interface);
    }
#[no_mangle]
unsafe extern "C" fn rionet_exit() -> void __exit {
    static void __exit rionet_exit(void)
    {
    unregister_reboot_notifier(&rionet_notifier);
    subsys_interface_unregister(&rionet_interface);
    class_interface_unregister(&rio_mport_interface);
    }
    late_initcall(rionet_init);
    module_exit(rionet_exit);
