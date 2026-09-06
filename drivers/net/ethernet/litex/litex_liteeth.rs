//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/litex/litex_liteeth.c
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
// LiteX Liteeth Ethernet
//
// Copyright 2017 Joel Stanley <joel@jms.id.au>
//

pub const LITEETH_WRITER_SLOT: c_uint = 0x00;
pub const LITEETH_WRITER_LENGTH: c_uint = 0x04;
pub const LITEETH_WRITER_ERRORS: c_uint = 0x08;
pub const LITEETH_WRITER_EV_STATUS: c_uint = 0x0C;
pub const LITEETH_WRITER_EV_PENDING: c_uint = 0x10;
pub const LITEETH_WRITER_EV_ENABLE: c_uint = 0x14;
pub const LITEETH_READER_START: c_uint = 0x18;
pub const LITEETH_READER_READY: c_uint = 0x1C;
pub const LITEETH_READER_LEVEL: c_uint = 0x20;
pub const LITEETH_READER_SLOT: c_uint = 0x24;
pub const LITEETH_READER_LENGTH: c_uint = 0x28;
pub const LITEETH_READER_EV_STATUS: c_uint = 0x2C;
pub const LITEETH_READER_EV_PENDING: c_uint = 0x30;
pub const LITEETH_READER_EV_ENABLE: c_uint = 0x34;
pub const LITEETH_PREAMBLE_CRC: c_uint = 0x38;
pub const LITEETH_PREAMBLE_ERRORS: c_uint = 0x3C;
pub const LITEETH_CRC_ERRORS: c_uint = 0x40;
pub const LITEETH_PHY_CRG_RESET: c_uint = 0x00;
pub const LITEETH_MDIO_W: c_uint = 0x04;
pub const LITEETH_MDIO_R: c_uint = 0x0C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct liteeth {
    pub base: *mut void __iomem,
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub slot_size: u32,
// Tx
    pub tx_slot: u32,
    pub num_tx_slots: u32,
    pub tx_base: *mut void __iomem,
// Rx
    pub rx_slot: u32,
    pub num_rx_slots: u32,
    pub rx_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn liteeth_rx(netdev: *mut net_device) -> c_int {
    static int liteeth_rx(struct net_device *netdev)
    {
    struct liteeth *priv = netdev_priv(netdev);
    struct sk_buff *skb;
    unsigned char *data;
    u8 rx_slot;
    int len;
    rx_slot = litex_read8(priv.base + LITEETH_WRITER_SLOT);
    len = litex_read32(priv.base + LITEETH_WRITER_LENGTH);
    if (len == 0 || len > 2048)
    goto rx_drop;
    skb = netdev_alloc_skb_ip_align(netdev, len);
    if (!skb) {
    netdev_err(netdev, "couldn't get memory\n");
    goto rx_drop;
    }
    data = skb_put(skb, len);
    memcpy_fromio(data, priv.rx_base + rx_slot * priv.slot_size, len);
    skb.protocol = eth_type_trans(skb, netdev);
    dev_sw_netstats_rx_add(netdev, len);
    return netif_rx(skb);
    rx_drop:
    netdev.stats.rx_dropped++;
    netdev.stats.rx_errors++;
    return NET_RX_DROP;
    }
#[no_mangle]
unsafe extern "C" fn liteeth_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t liteeth_interrupt(int irq, void *dev_id)
    {
    struct net_device *netdev = dev_id;
    struct liteeth *priv = netdev_priv(netdev);
    u8 reg;
    reg = litex_read8(priv.base + LITEETH_READER_EV_PENDING);
    if (reg) {
    if (netif_queue_stopped(netdev))
    netif_wake_queue(netdev);
    litex_write8(priv.base + LITEETH_READER_EV_PENDING, reg);
    }
    reg = litex_read8(priv.base + LITEETH_WRITER_EV_PENDING);
    if (reg) {
    liteeth_rx(netdev);
    litex_write8(priv.base + LITEETH_WRITER_EV_PENDING, reg);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn liteeth_open(netdev: *mut net_device) -> c_int {
    static int liteeth_open(struct net_device *netdev)
    {
    struct liteeth *priv = netdev_priv(netdev);
    int err;
// Clear pending events
    litex_write8(priv.base + LITEETH_WRITER_EV_PENDING, 1);
    litex_write8(priv.base + LITEETH_READER_EV_PENDING, 1);
    err = request_irq(netdev.irq, liteeth_interrupt, 0, netdev.name, netdev);
    if (err) {
    netdev_err(netdev, "failed to request irq %d\n", netdev.irq);
    return err;
    }
// Enable IRQs
    litex_write8(priv.base + LITEETH_WRITER_EV_ENABLE, 1);
    litex_write8(priv.base + LITEETH_READER_EV_ENABLE, 1);
    netif_carrier_on(netdev);
    netif_start_queue(netdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn liteeth_stop(netdev: *mut net_device) -> c_int {
    static int liteeth_stop(struct net_device *netdev)
    {
    struct liteeth *priv = netdev_priv(netdev);
    netif_stop_queue(netdev);
    netif_carrier_off(netdev);
    litex_write8(priv.base + LITEETH_WRITER_EV_ENABLE, 0);
    litex_write8(priv.base + LITEETH_READER_EV_ENABLE, 0);
    free_irq(netdev.irq, netdev);
    return 0;
    }
    static netdev_tx_t liteeth_start_xmit(struct sk_buff *skb,
    struct net_device *netdev)
    {
    struct liteeth *priv = netdev_priv(netdev);
    void __iomem *txbuffer;
    if (!litex_read8(priv.base + LITEETH_READER_READY)) {
    if (net_ratelimit())
    netdev_err(netdev, "LITEETH_READER_READY not ready\n");
    netif_stop_queue(netdev);
    return NETDEV_TX_BUSY;
    }
// Reject oversize packets
    if (unlikely(skb.len > priv.slot_size)) {
    if (net_ratelimit())
    netdev_err(netdev, "tx packet too big\n");
    dev_kfree_skb_any(skb);
    netdev.stats.tx_dropped++;
    netdev.stats.tx_errors++;
    return NETDEV_TX_OK;
    }
    txbuffer = priv.tx_base + priv.tx_slot * priv.slot_size;
    memcpy_toio(txbuffer, skb.data, skb.len);
    litex_write8(priv.base + LITEETH_READER_SLOT, priv.tx_slot);
    litex_write16(priv.base + LITEETH_READER_LENGTH, skb.len);
    litex_write8(priv.base + LITEETH_READER_START, 1);
    dev_sw_netstats_tx_add(netdev, 1, skb.len);
    priv.tx_slot = (priv.tx_slot + 1) % priv.num_tx_slots;
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    }
    static void
    liteeth_get_stats64(struct net_device *netdev, struct rtnl_link_stats64 *stats)
    {
    netdev_stats_to_stats64(stats, &netdev.stats);
    dev_fetch_sw_netstats(stats, netdev.tstats);
    }
    static const struct net_device_ops liteeth_netdev_ops = {
    .ndo_open		= liteeth_open,
    .ndo_stop		= liteeth_stop,
    .ndo_get_stats64	= liteeth_get_stats64,
    .ndo_start_xmit         = liteeth_start_xmit,
    };
#[no_mangle]
unsafe extern "C" fn liteeth_setup_slots(priv: *mut liteeth) {
    static void liteeth_setup_slots(struct liteeth *priv)
    {
    struct device_node *np = priv.dev.of_node;
    int err;
    err = of_property_read_u32(np, "litex,rx-slots", &priv.num_rx_slots);
    if (err) {
    dev_dbg(priv.dev, "unable to get litex,rx-slots, using 2\n");
    priv.num_rx_slots = 2;
    }
    err = of_property_read_u32(np, "litex,tx-slots", &priv.num_tx_slots);
    if (err) {
    dev_dbg(priv.dev, "unable to get litex,tx-slots, using 2\n");
    priv.num_tx_slots = 2;
    }
    err = of_property_read_u32(np, "litex,slot-size", &priv.slot_size);
    if (err) {
    dev_dbg(priv.dev, "unable to get litex,slot-size, using 0x800\n");
    priv.slot_size = 0x800;
    }
    }
#[no_mangle]
unsafe extern "C" fn liteeth_probe(pdev: *mut platform_device) -> c_int {
    static int liteeth_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct net_device *netdev;
    void __iomem *buf_base;
    struct liteeth *priv;
    int irq, err;
    netdev = devm_alloc_etherdev(dev, sizeof(*priv));
    if (!netdev)
    return -ENOMEM;
    SET_NETDEV_DEV(netdev, &pdev.dev);
    platform_set_drvdata(pdev, netdev);
    priv = netdev_priv(netdev);
    priv.netdev = netdev;
    priv.dev = dev;
    netdev.tstats = devm_netdev_alloc_pcpu_stats(dev,
    struct pcpu_sw_netstats);
    if (!netdev.tstats)
    return -ENOMEM;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    netdev.irq = irq;
    priv.base = devm_platform_ioremap_resource_byname(pdev, "mac");
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    buf_base = devm_platform_ioremap_resource_byname(pdev, "buffer");
    if (IS_ERR(buf_base))
    return PTR_ERR(buf_base);
    liteeth_setup_slots(priv);
// Rx slots
    priv.rx_base = buf_base;
    priv.rx_slot = 0;
// Tx slots come after Rx slots
    priv.tx_base = buf_base + priv.num_rx_slots * priv.slot_size;
    priv.tx_slot = 0;
    err = of_get_ethdev_address(dev.of_node, netdev);
    if (err)
    eth_hw_addr_random(netdev);
    netdev.netdev_ops = &liteeth_netdev_ops;
    err = devm_register_netdev(dev, netdev);
    if (err) {
    dev_err(dev, "Failed to register netdev %d\n", err);
    return err;
    }
    netdev_info(netdev, "irq %d slots: tx %d rx %d size %d\n",
    netdev.irq, priv.num_tx_slots, priv.num_rx_slots, priv.slot_size);
    return 0;
    }
    static const struct of_device_id liteeth_of_match[] = {
    { .compatible = "litex,liteeth" },
    { }
    };
    MODULE_DEVICE_TABLE(of, liteeth_of_match);
    static struct platform_driver liteeth_driver = {
    .probe = liteeth_probe,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = liteeth_of_match,
    },
    };
    module_platform_driver(liteeth_driver);
    MODULE_AUTHOR("Joel Stanley <joel@jms.id.au>");
    MODULE_DESCRIPTION("LiteX Liteeth Ethernet driver");
    MODULE_LICENSE("GPL");
