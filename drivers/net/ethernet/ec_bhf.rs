//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/ec_bhf.c
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
// drivers/net/ethernet/ec_bhf.c
//
// Copyright (C) 2014 Darek Marcinkiewicz <reksio@newterm.pl>
//
// This is a driver for EtherCAT master module present on CCAT FPGA.
// Those can be found on Bechhoff CX50xx industrial PCs.
//

pub const TIMER_INTERVAL_NSEC: c_int = 20000;
pub const INFO_BLOCK_SIZE: c_uint = 0x10;
pub const INFO_BLOCK_TYPE: c_uint = 0x0;
pub const INFO_BLOCK_REV: c_uint = 0x2;
pub const INFO_BLOCK_BLK_CNT: c_uint = 0x4;
pub const INFO_BLOCK_TX_CHAN: c_uint = 0x4;
pub const INFO_BLOCK_RX_CHAN: c_uint = 0x5;
pub const INFO_BLOCK_OFFSET: c_uint = 0x8;
pub const EC_MII_OFFSET: c_uint = 0x4;
pub const EC_FIFO_OFFSET: c_uint = 0x8;
pub const EC_MAC_OFFSET: c_uint = 0xc;
pub const MAC_FRAME_ERR_CNT: c_uint = 0x0;
pub const MAC_RX_ERR_CNT: c_uint = 0x1;
pub const MAC_CRC_ERR_CNT: c_uint = 0x2;
pub const MAC_LNK_LST_ERR_CNT: c_uint = 0x3;
pub const MAC_TX_FRAME_CNT: c_uint = 0x10;
pub const MAC_RX_FRAME_CNT: c_uint = 0x14;
pub const MAC_TX_FIFO_LVL: c_uint = 0x20;
pub const MAC_DROPPED_FRMS: c_uint = 0x28;
pub const MAC_CONNECTED_CCAT_FLAG: c_uint = 0x78;
pub const MII_MAC_ADDR: c_uint = 0x8;
pub const MII_MAC_FILT_FLAG: c_uint = 0xe;
pub const MII_LINK_STATUS: c_uint = 0xf;
pub const FIFO_TX_REG: c_uint = 0x0;
pub const FIFO_TX_RESET: c_uint = 0x8;
pub const FIFO_RX_REG: c_uint = 0x10;

pub const FIFO_RX_RESET: c_uint = 0x18;
pub const DMA_CHAN_OFFSET: c_uint = 0x1000;
pub const DMA_CHAN_SIZE: c_uint = 0x8;
pub const DMA_WINDOW_SIZE_MASK: c_uint = 0xfffffffc;
pub const ETHERCAT_MASTER_ID: c_uint = 0x14;
    static const struct pci_device_id ids[] = {
    { PCI_DEVICE(0x15ec, 0x5000), },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, ids);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_header {
pub const RXHDR_NEXT_ADDR_MASK: c_uint = 0xffffffu;

    pub next: __le32,
pub const RXHDR_NEXT_RECV_FLAG: c_uint = 0x1;
    pub recv: __le32,
pub const RXHDR_LEN_MASK: c_uint = 0xfffu;
    pub len: __le16,
    pub port: __le16,
    pub reserved: __le32,
    pub timestamp: [u8; 8],
    pub __packed: },
pub const PKT_PAYLOAD_SIZE: c_uint = 0x7e8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc {
    pub header: rx_header,
    pub data: [u8; PKT_PAYLOAD_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_header {
    pub len: __le16,
pub const TX_HDR_PORT_0: c_uint = 0x1;
pub const TX_HDR_PORT_1: c_uint = 0x2;
    pub port: u8,
    pub ts_enable: u8,
pub const TX_HDR_SENT: c_uint = 0x1;
    pub sent: __le32,
    pub timestamp: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
    pub header: tx_header,
    pub data: [u8; PKT_PAYLOAD_SIZE],
    pub __packed: },
pub const FIFO_SIZE: c_int = 64;
    pub TIMER_INTERVAL_NSEC: static long polling_frequency =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bhf_dma {
    pub buf: *mut u8,
    pub len: usize,
    pub buf_phys: dma_addr_t,
    pub alloc: *mut u8,
    pub alloc_len: usize,
    pub alloc_phys: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_bhf_priv {
    pub net_dev: *mut net_device,
    pub dev: *mut pci_dev,
    pub io: *mut void __iomem,
    pub dma_io: *mut void __iomem,
    pub hrtimer: hrtimer,
    pub tx_dma_chan: c_int,
    pub rx_dma_chan: c_int,
    pub ec_io: *mut void __iomem,
    pub fifo_io: *mut void __iomem,
    pub mii_io: *mut void __iomem,
    pub mac_io: *mut void __iomem,
    pub rx_buf: bhf_dma,
    pub rx_descs: *mut rx_desc,
    pub rx_dnext: c_int,
    pub rx_dcount: c_int,
    pub tx_buf: bhf_dma,
    pub tx_descs: *mut tx_desc,
    pub tx_dcount: c_int,
    pub tx_dnext: c_int,
    pub stat_rx_bytes: u64,
    pub stat_tx_bytes: u64,
}

#[no_mangle]
unsafe extern "C" fn ec_bhf_reset(priv: *mut ec_bhf_priv) {
    static void ec_bhf_reset(struct ec_bhf_priv *priv)
    {
    iowrite8(0, priv.mac_io + MAC_FRAME_ERR_CNT);
    iowrite8(0, priv.mac_io + MAC_RX_ERR_CNT);
    iowrite8(0, priv.mac_io + MAC_CRC_ERR_CNT);
    iowrite8(0, priv.mac_io + MAC_LNK_LST_ERR_CNT);
    iowrite32(0, priv.mac_io + MAC_TX_FRAME_CNT);
    iowrite32(0, priv.mac_io + MAC_RX_FRAME_CNT);
    iowrite8(0, priv.mac_io + MAC_DROPPED_FRMS);
    iowrite8(0, priv.fifo_io + FIFO_TX_RESET);
    iowrite8(0, priv.fifo_io + FIFO_RX_RESET);
    iowrite8(0, priv.mac_io + MAC_TX_FIFO_LVL);
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_send_packet(priv: *mut ec_bhf_priv, desc: *mut tx_desc) {
    static void ec_bhf_send_packet(struct ec_bhf_priv *priv, struct tx_desc *desc)
    {
    let mut len: u32 = le16_to_cpu(desc.header.len) + sizeof(desc.header);
    let mut addr: u32 = (u8 *)desc - priv.tx_buf.buf;
    iowrite32((ALIGN(len, 8) << 24) | addr, priv.fifo_io + FIFO_TX_REG);
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_desc_sent(desc: *mut tx_desc) -> c_int {
    static int ec_bhf_desc_sent(struct tx_desc *desc)
    {
    return le32_to_cpu(desc.header.sent) & TX_HDR_SENT;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_process_tx(priv: *mut ec_bhf_priv) {
    static void ec_bhf_process_tx(struct ec_bhf_priv *priv)
    {
    if (unlikely(netif_queue_stopped(priv.net_dev))) {
// Make sure that we perceive changes to tx_dnext.
    smp_rmb();
    if (ec_bhf_desc_sent(&priv.tx_descs[priv.tx_dnext]))
    netif_wake_queue(priv.net_dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_pkt_received(desc: *mut rx_desc) -> c_int {
    static int ec_bhf_pkt_received(struct rx_desc *desc)
    {
    return le32_to_cpu(desc.header.recv) & RXHDR_NEXT_RECV_FLAG;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_add_rx_desc(priv: *mut ec_bhf_priv, desc: *mut rx_desc) {
    static void ec_bhf_add_rx_desc(struct ec_bhf_priv *priv, struct rx_desc *desc)
    {
    iowrite32(FIFO_RX_ADDR_VALID | ((u8 *)(desc) - priv.rx_buf.buf),
    priv.fifo_io + FIFO_RX_REG);
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_process_rx(priv: *mut ec_bhf_priv) {
    static void ec_bhf_process_rx(struct ec_bhf_priv *priv)
    {
    struct rx_desc *desc = &priv.rx_descs[priv.rx_dnext];
    while (ec_bhf_pkt_received(desc)) {
    int pkt_size = (le16_to_cpu(desc.header.len) &
    RXHDR_LEN_MASK) - sizeof(struct rx_header) - 4;
    u8 *data = desc.data;
    struct sk_buff *skb;
    skb = netdev_alloc_skb_ip_align(priv.net_dev, pkt_size);
    if (skb) {
    skb_put_data(skb, data, pkt_size);
    skb.protocol = eth_type_trans(skb, priv.net_dev);
    priv.stat_rx_bytes += pkt_size;
    netif_rx(skb);
    } else {
    dev_err_ratelimited(PRIV_TO_DEV(priv),
    "Couldn't allocate a skb_buff for a packet of size %u\n",
    pkt_size);
    }
    desc.header.recv = 0;
    ec_bhf_add_rx_desc(priv, desc);
    priv.rx_dnext = (priv.rx_dnext + 1) % priv.rx_dcount;
    desc = &priv.rx_descs[priv.rx_dnext];
    }
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_timer_fun(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart ec_bhf_timer_fun(struct hrtimer *timer)
    {
    struct ec_bhf_priv *priv = container_of(timer, struct ec_bhf_priv,
    hrtimer);
    ec_bhf_process_rx(priv);
    ec_bhf_process_tx(priv);
    if (!netif_running(priv.net_dev))
    return HRTIMER_NORESTART;
    hrtimer_forward_now(timer, polling_frequency);
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_setup_offsets(priv: *mut ec_bhf_priv) -> c_int {
    static int ec_bhf_setup_offsets(struct ec_bhf_priv *priv)
    {
    struct device *dev = PRIV_TO_DEV(priv);
    unsigned block_count, i;
    void __iomem *ec_info;
    block_count = ioread8(priv.io + INFO_BLOCK_BLK_CNT);
    for (i = 0; i < block_count; i++) {
    u16 type = ioread16(priv.io + i * INFO_BLOCK_SIZE +
    INFO_BLOCK_TYPE);
    if (type == ETHERCAT_MASTER_ID)
    break;
    }
    if (i == block_count) {
    dev_err(dev, "EtherCAT master with DMA block not found\n");
    return -ENODEV;
    }
    ec_info = priv.io + i * INFO_BLOCK_SIZE;
    priv.tx_dma_chan = ioread8(ec_info + INFO_BLOCK_TX_CHAN);
    priv.rx_dma_chan = ioread8(ec_info + INFO_BLOCK_RX_CHAN);
    priv.ec_io = priv.io + ioread32(ec_info + INFO_BLOCK_OFFSET);
    priv.mii_io = priv.ec_io + ioread32(priv.ec_io + EC_MII_OFFSET);
    priv.fifo_io = priv.ec_io + ioread32(priv.ec_io + EC_FIFO_OFFSET);
    priv.mac_io = priv.ec_io + ioread32(priv.ec_io + EC_MAC_OFFSET);
    return 0;
    }
    static netdev_tx_t ec_bhf_start_xmit(struct sk_buff *skb,
    struct net_device *net_dev)
    {
    struct ec_bhf_priv *priv = netdev_priv(net_dev);
    struct tx_desc *desc;
    unsigned len;
    desc = &priv.tx_descs[priv.tx_dnext];
    skb_copy_and_csum_dev(skb, desc.data);
    len = skb.len;
    memset(&desc.header, 0, sizeof(desc.header));
    desc.header.len = cpu_to_le16(len);
    desc.header.port = TX_HDR_PORT_0;
    ec_bhf_send_packet(priv, desc);
    priv.tx_dnext = (priv.tx_dnext + 1) % priv.tx_dcount;
    if (!ec_bhf_desc_sent(&priv.tx_descs[priv.tx_dnext])) {
// Make sure that updates to tx_dnext are perceived
// by timer routine.
//
    smp_wmb();
    netif_stop_queue(net_dev);
    }
    priv.stat_tx_bytes += len;
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    static int ec_bhf_alloc_dma_mem(struct ec_bhf_priv *priv,
    struct bhf_dma *buf,
    int channel,
    int size)
    {
    let mut offset: c_int = channel * DMA_CHAN_SIZE + DMA_CHAN_OFFSET;
    struct device *dev = PRIV_TO_DEV(priv);
    u32 mask;
    iowrite32(0xffffffff, priv.dma_io + offset);
    mask = ioread32(priv.dma_io + offset);
    mask &= DMA_WINDOW_SIZE_MASK;
// We want to allocate a chunk of memory that is:
// - aligned to the mask we just read
// - is of size 2^mask bytes (at most)
// In order to ensure that we will allocate buffer of
// 2 * 2^mask bytes.
//
    buf.len = min_t(int, ~mask + 1, size);
    buf.alloc_len = 2 * buf.len;
    buf.alloc = dma_alloc_coherent(dev, buf.alloc_len, &buf.alloc_phys,
    GFP_KERNEL);
    if (buf.alloc == core::ptr::null_mut()) {
    dev_err(dev, "Failed to allocate buffer\n");
    return -ENOMEM;
    }
    buf.buf_phys = (buf.alloc_phys + buf.len) & mask;
    buf.buf = buf.alloc + (buf.buf_phys - buf.alloc_phys);
    iowrite32(0, priv.dma_io + offset + 4);
    iowrite32(buf.buf_phys, priv.dma_io + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_setup_tx_descs(priv: *mut ec_bhf_priv) {
    static void ec_bhf_setup_tx_descs(struct ec_bhf_priv *priv)
    {
    let mut i: c_int = 0;
    priv.tx_dcount = priv.tx_buf.len / sizeof(struct tx_desc);
    priv.tx_descs = (struct tx_desc *)priv.tx_buf.buf;
    priv.tx_dnext = 0;
    for (i = 0; i < priv.tx_dcount; i++)
    priv.tx_descs[i].header.sent = cpu_to_le32(TX_HDR_SENT);
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_setup_rx_descs(priv: *mut ec_bhf_priv) {
    static void ec_bhf_setup_rx_descs(struct ec_bhf_priv *priv)
    {
    int i;
    priv.rx_dcount = priv.rx_buf.len / sizeof(struct rx_desc);
    priv.rx_descs = (struct rx_desc *)priv.rx_buf.buf;
    priv.rx_dnext = 0;
    for (i = 0; i < priv.rx_dcount; i++) {
    struct rx_desc *desc = &priv.rx_descs[i];
    u32 next;
    if (i != priv.rx_dcount - 1)
    next = (u8 *)(desc + 1) - priv.rx_buf.buf;
    else
    next = 0;
    next |= RXHDR_NEXT_VALID;
    desc.header.next = cpu_to_le32(next);
    desc.header.recv = 0;
    ec_bhf_add_rx_desc(priv, desc);
    }
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_open(net_dev: *mut net_device) -> c_int {
    static int ec_bhf_open(struct net_device *net_dev)
    {
    struct ec_bhf_priv *priv = netdev_priv(net_dev);
    struct device *dev = PRIV_TO_DEV(priv);
    let mut err: c_int = 0;
    ec_bhf_reset(priv);
    err = ec_bhf_alloc_dma_mem(priv, &priv.rx_buf, priv.rx_dma_chan,
    FIFO_SIZE * sizeof(struct rx_desc));
    if (err) {
    dev_err(dev, "Failed to allocate rx buffer\n");
    goto out;
    }
    ec_bhf_setup_rx_descs(priv);
    err = ec_bhf_alloc_dma_mem(priv, &priv.tx_buf, priv.tx_dma_chan,
    FIFO_SIZE * sizeof(struct tx_desc));
    if (err) {
    dev_err(dev, "Failed to allocate tx buffer\n");
    goto error_rx_free;
    }
    iowrite8(0, priv.mii_io + MII_MAC_FILT_FLAG);
    ec_bhf_setup_tx_descs(priv);
    netif_start_queue(net_dev);
    hrtimer_setup(&priv.hrtimer, ec_bhf_timer_fun, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    hrtimer_start(&priv.hrtimer, polling_frequency, HRTIMER_MODE_REL);
    return 0;
    error_rx_free:
    dma_free_coherent(dev, priv.rx_buf.alloc_len, priv.rx_buf.alloc,
    priv.rx_buf.alloc_phys);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_stop(net_dev: *mut net_device) -> c_int {
    static int ec_bhf_stop(struct net_device *net_dev)
    {
    struct ec_bhf_priv *priv = netdev_priv(net_dev);
    struct device *dev = PRIV_TO_DEV(priv);
    hrtimer_cancel(&priv.hrtimer);
    ec_bhf_reset(priv);
    netif_tx_disable(net_dev);
    dma_free_coherent(dev, priv.tx_buf.alloc_len,
    priv.tx_buf.alloc, priv.tx_buf.alloc_phys);
    dma_free_coherent(dev, priv.rx_buf.alloc_len,
    priv.rx_buf.alloc, priv.rx_buf.alloc_phys);
    return 0;
    }
    static void
    ec_bhf_get_stats(struct net_device *net_dev,
    struct rtnl_link_stats64 *stats)
    {
    struct ec_bhf_priv *priv = netdev_priv(net_dev);
    stats.rx_errors = ioread8(priv.mac_io + MAC_RX_ERR_CNT) +
    ioread8(priv.mac_io + MAC_CRC_ERR_CNT) +
    ioread8(priv.mac_io + MAC_FRAME_ERR_CNT);
    stats.rx_packets = ioread32(priv.mac_io + MAC_RX_FRAME_CNT);
    stats.tx_packets = ioread32(priv.mac_io + MAC_TX_FRAME_CNT);
    stats.rx_dropped = ioread8(priv.mac_io + MAC_DROPPED_FRMS);
    stats.tx_bytes = priv.stat_tx_bytes;
    stats.rx_bytes = priv.stat_rx_bytes;
    }
    static const struct net_device_ops ec_bhf_netdev_ops = {
    .ndo_start_xmit		= ec_bhf_start_xmit,
    .ndo_open		= ec_bhf_open,
    .ndo_stop		= ec_bhf_stop,
    .ndo_get_stats64	= ec_bhf_get_stats,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr
    };
#[no_mangle]
unsafe extern "C" fn ec_bhf_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int ec_bhf_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct net_device *net_dev;
    struct ec_bhf_priv *priv;
    void __iomem *dma_io;
    u8 addr[ETH_ALEN];
    void __iomem *io;
    let mut err: c_int = 0;
    err = pci_enable_device(dev);
    if (err)
    return err;
    pci_set_master(dev);
    err = dma_set_mask_and_coherent(&dev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_err(&dev.dev,
    "Required dma mask not supported, failed to initialize device\n");
    goto err_disable_dev;
    }
    err = pci_request_regions(dev, "ec_bhf");
    if (err) {
    dev_err(&dev.dev, "Failed to request pci memory regions\n");
    goto err_disable_dev;
    }
    io = pci_iomap(dev, 0, 0);
    if (!io) {
    dev_err(&dev.dev, "Failed to map pci card memory bar 0");
    err = -EIO;
    goto err_release_regions;
    }
    dma_io = pci_iomap(dev, 2, 0);
    if (!dma_io) {
    dev_err(&dev.dev, "Failed to map pci card memory bar 2");
    err = -EIO;
    goto err_unmap;
    }
    net_dev = alloc_etherdev(sizeof(struct ec_bhf_priv));
    if (net_dev == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto err_unmap_dma_io;
    }
    pci_set_drvdata(dev, net_dev);
    SET_NETDEV_DEV(net_dev, &dev.dev);
    net_dev.features = 0;
    net_dev.flags |= IFF_NOARP;
    net_dev.netdev_ops = &ec_bhf_netdev_ops;
    priv = netdev_priv(net_dev);
    priv.net_dev = net_dev;
    priv.io = io;
    priv.dma_io = dma_io;
    priv.dev = dev;
    err = ec_bhf_setup_offsets(priv);
    if (err < 0)
    goto err_free_net_dev;
    memcpy_fromio(addr, priv.mii_io + MII_MAC_ADDR, ETH_ALEN);
    eth_hw_addr_set(net_dev, addr);
    err = register_netdev(net_dev);
    if (err < 0)
    goto err_free_net_dev;
    return 0;
    err_free_net_dev:
    free_netdev(net_dev);
    err_unmap_dma_io:
    pci_iounmap(dev, dma_io);
    err_unmap:
    pci_iounmap(dev, io);
    err_release_regions:
    pci_release_regions(dev);
    err_disable_dev:
    pci_disable_device(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ec_bhf_remove(dev: *mut pci_dev) {
    static void ec_bhf_remove(struct pci_dev *dev)
    {
    struct net_device *net_dev = pci_get_drvdata(dev);
    struct ec_bhf_priv *priv = netdev_priv(net_dev);
    unregister_netdev(net_dev);
    pci_iounmap(dev, priv.dma_io);
    pci_iounmap(dev, priv.io);
    free_netdev(net_dev);
    pci_release_regions(dev);
    pci_disable_device(dev);
    }
    static struct pci_driver pci_driver = {
    .name		= "ec_bhf",
    .id_table	= ids,
    .probe		= ec_bhf_probe,
    .remove		= ec_bhf_remove,
    };
    module_pci_driver(pci_driver);
    module_param(polling_frequency, long, 0444);
    MODULE_PARM_DESC(polling_frequency, "Polling timer frequency in ns");
    MODULE_DESCRIPTION("Beckhoff CX5020 EtherCAT Ethernet driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Dariusz Marcinkiewicz <reksio@newterm.pl>");
