//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/rdc/r6040.c
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
// RDC R6040 Fast Ethernet MAC support
//
// Copyright (C) 2004 Sten Wang <sten.wang@rdc.com.tw>
// Copyright (C) 2007
// Daniel Gimpelevich <daniel@gimpelevich.san-francisco.ca.us>
// Copyright (C) 2007-2012 Florian Fainelli <f.fainelli@gmail.com>
//

// Time in jiffies before concluding the transmitter is hung.

// RDC MAC I/O Size
pub const R6040_IO_SIZE: c_int = 256;
// MAX RDC MAC
pub const MAX_MAC: c_int = 2;
// MAC registers
pub const MCR0: c_uint = 0x00	/* Control register 0 */;
pub const MCR0_RCVEN: c_uint = 0x0002	/* Receive enable */;
pub const MCR0_PROMISC: c_uint = 0x0020	/* Promiscuous mode */;
pub const MCR0_HASH_EN: c_uint = 0x0100	/* Enable multicast hash table function */;
pub const MCR0_XMTEN: c_uint = 0x1000	/* Transmission enable */;
pub const MCR0_FD: c_uint = 0x8000	/* Full/Half duplex */;
pub const MCR1: c_uint = 0x04	/* Control register 1 */;
pub const MAC_RST: c_uint = 0x0001	/* Reset the MAC */;
pub const MBCR: c_uint = 0x08	/* Bus control */;
pub const MT_ICR: c_uint = 0x0C	/* TX interrupt control */;
pub const MR_ICR: c_uint = 0x10	/* RX interrupt control */;
pub const MTPR: c_uint = 0x14	/* TX poll command register */;
pub const TM2TX: c_uint = 0x0001	/* Trigger MAC to transmit */;
pub const MR_BSR: c_uint = 0x18	/* RX buffer size */;
pub const MR_DCR: c_uint = 0x1A	/* RX descriptor control */;
pub const MLSR: c_uint = 0x1C	/* Last status */;
pub const TX_FIFO_UNDR: c_uint = 0x0200	/* TX FIFO under-run */;
pub const TX_EXCEEDC: c_uint = 0x2000	/* Transmit exceed collision */;
pub const TX_LATEC: c_uint = 0x4000	/* Transmit late collision */;
pub const MMDIO: c_uint = 0x20	/* MDIO control register */;
pub const MDIO_WRITE: c_uint = 0x4000	/* MDIO write */;
pub const MDIO_READ: c_uint = 0x2000	/* MDIO read */;
pub const MMRD: c_uint = 0x24	/* MDIO read data register */;
pub const MMWD: c_uint = 0x28	/* MDIO write data register */;
pub const MTD_SA0: c_uint = 0x2C	/* TX descriptor start address 0 */;
pub const MTD_SA1: c_uint = 0x30	/* TX descriptor start address 1 */;
pub const MRD_SA0: c_uint = 0x34	/* RX descriptor start address 0 */;
pub const MRD_SA1: c_uint = 0x38	/* RX descriptor start address 1 */;
pub const MISR: c_uint = 0x3C	/* Status register */;
pub const MIER: c_uint = 0x40	/* INT enable register */;
pub const MSK_INT: c_uint = 0x0000	/* Mask off interrupts */;
pub const RX_FINISH: c_uint = 0x0001  /* RX finished */;
pub const RX_NO_DESC: c_uint = 0x0002  /* No RX descriptor available */;
pub const RX_FIFO_FULL: c_uint = 0x0004  /* RX FIFO full */;
pub const RX_EARLY: c_uint = 0x0008  /* RX early */;
pub const TX_FINISH: c_uint = 0x0010  /* TX finished */;
pub const TX_EARLY: c_uint = 0x0080  /* TX early */;
pub const EVENT_OVRFL: c_uint = 0x0100  /* Event counter overflow */;
pub const LINK_CHANGED: c_uint = 0x0200  /* PHY link changed */;
pub const ME_CISR: c_uint = 0x44	/* Event counter INT status */;
pub const ME_CIER: c_uint = 0x48	/* Event counter INT enable  */;
pub const MR_CNT: c_uint = 0x50	/* Successfully received packet counter */;
pub const ME_CNT0: c_uint = 0x52	/* Event counter 0 */;
pub const ME_CNT1: c_uint = 0x54	/* Event counter 1 */;
pub const ME_CNT2: c_uint = 0x56	/* Event counter 2 */;
pub const ME_CNT3: c_uint = 0x58	/* Event counter 3 */;
pub const MT_CNT: c_uint = 0x5A	/* Successfully transmit packet counter */;
pub const ME_CNT4: c_uint = 0x5C	/* Event counter 4 */;
pub const MP_CNT: c_uint = 0x5E	/* Pause frame counter register */;
pub const MAR0: c_uint = 0x60	/* Hash table 0 */;
pub const MAR1: c_uint = 0x62	/* Hash table 1 */;
pub const MAR2: c_uint = 0x64	/* Hash table 2 */;
pub const MAR3: c_uint = 0x66	/* Hash table 3 */;
pub const MID_0L: c_uint = 0x68	/* Multicast address MID0 Low */;
pub const MID_0M: c_uint = 0x6A	/* Multicast address MID0 Medium */;
pub const MID_0H: c_uint = 0x6C	/* Multicast address MID0 High */;
pub const MID_1L: c_uint = 0x70	/* MID1 Low */;
pub const MID_1M: c_uint = 0x72	/* MID1 Medium */;
pub const MID_1H: c_uint = 0x74	/* MID1 High */;
pub const MID_2L: c_uint = 0x78	/* MID2 Low */;
pub const MID_2M: c_uint = 0x7A	/* MID2 Medium */;
pub const MID_2H: c_uint = 0x7C	/* MID2 High */;
pub const MID_3L: c_uint = 0x80	/* MID3 Low */;
pub const MID_3M: c_uint = 0x82	/* MID3 Medium */;
pub const MID_3H: c_uint = 0x84	/* MID3 High */;
pub const PHY_CC: c_uint = 0x88	/* PHY status change configuration register */;
pub const SCEN: c_uint = 0x8000	/* PHY status change enable */;

pub const PHY_ST: c_uint = 0x8A	/* PHY status register */;
pub const MAC_SM: c_uint = 0xAC	/* MAC status machine */;
pub const MAC_SM_RST: c_uint = 0x0002	/* MAC status machine reset */;
pub const MD_CSC: c_uint = 0xb6	/* MDC speed control register */;
pub const MD_CSC_DEFAULT: c_uint = 0x0030;
pub const MAC_ID: c_uint = 0xBE	/* Identifier register */;
pub const TX_DCNT: c_uint = 0x80	/* TX descriptor count */;
pub const RX_DCNT: c_uint = 0x80	/* RX descriptor count */;
pub const MAX_BUF_SIZE: c_uint = 0x600;

pub const MBCR_DEFAULT: c_uint = 0x012A	/* MAC Bus Control Register */;

// Descriptor status
pub const DSC_OWNER_MAC: c_uint = 0x8000	/* MAC is the owner of this descriptor */;
pub const DSC_RX_OK: c_uint = 0x4000	/* RX was successful */;
pub const DSC_RX_ERR: c_uint = 0x0800	/* RX PHY error */;
pub const DSC_RX_ERR_DRI: c_uint = 0x0400	/* RX dribble packet */;
pub const DSC_RX_ERR_BUF: c_uint = 0x0200	/* RX length exceeds buffer size */;
pub const DSC_RX_ERR_LONG: c_uint = 0x0100	/* RX length > maximum packet length */;
pub const DSC_RX_ERR_RUNT: c_uint = 0x0080	/* RX packet length < 64 byte */;
pub const DSC_RX_ERR_CRC: c_uint = 0x0040	/* RX CRC error */;
pub const DSC_RX_BCAST: c_uint = 0x0020	/* RX broadcast (no error) */;
pub const DSC_RX_MCAST: c_uint = 0x0010	/* RX multicast (no error) */;
pub const DSC_RX_MCH_HIT: c_uint = 0x0008	/* RX multicast hit in hash table (no error) */;
pub const DSC_RX_MIDH_HIT: c_uint = 0x0004	/* RX MID table hit (no error) */;

    MODULE_AUTHOR("Sten Wang <sten.wang@rdc.com.tw>,"
    "Daniel Gimpelevich <daniel@gimpelevich.san-francisco.ca.us>,"
    "Florian Fainelli <f.fainelli@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("RDC R6040 NAPI PCI FastEthernet driver");
    MODULE_VERSION(DRV_VERSION " " DRV_RELDATE);
// RX and TX interrupts that we handle

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r6040_descriptor {
    pub /: *mut *mut u16 status, len; / 0-3,
    pub /: *mut *mut __le32 buf; / 4-7,
    pub /: *mut *mut __le32 ndesc; / 8-B,
    pub /: *mut *mut u32 rev1; / C-F,
    pub /: *mut *mut *mut char vbufp; / 10-13,
    pub /: *mut *mut *mut r6040_descriptor vndescp; / 14-17,
    pub /: *mut *mut *mut sk_buff skb_ptr; / 18-1B,
    pub /: *mut *mut u32 rev2; / 1C-1F,
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r6040_private {
    pub /: *mut *mut spinlock_t lock; / driver lock,
    pub pdev: *mut pci_dev,
    pub rx_insert_ptr: *mut r6040_descriptor,
    pub rx_remove_ptr: *mut r6040_descriptor,
    pub tx_insert_ptr: *mut r6040_descriptor,
    pub tx_remove_ptr: *mut r6040_descriptor,
    pub rx_ring: *mut r6040_descriptor,
    pub tx_ring: *mut r6040_descriptor,
    pub rx_ring_dma: dma_addr_t,
    pub tx_ring_dma: dma_addr_t,
    pub tx_free_desc: u16,
    pub mcr0: u16,
    pub dev: *mut net_device,
    pub mii_bus: *mut mii_bus,
    pub napi: napi_struct,
    pub base: *mut void __iomem,
    pub old_link: c_int,
    pub old_duplex: c_int,
}

    static char version[] = DRV_NAME
    ": RDC R6040 NAPI net driver,"
    "version "DRV_VERSION " (" DRV_RELDATE ")";
// Read a word data from PHY Chip
#[no_mangle]
unsafe extern "C" fn r6040_phy_read(ioaddr: *mut void __iomem, phy_addr: c_int, reg: c_int) -> c_int {
    static int r6040_phy_read(void __iomem *ioaddr, int phy_addr, int reg)
    {
    let mut limit: c_int = MAC_DEF_TIMEOUT;
    u16 cmd;
    iowrite16(MDIO_READ | reg | (phy_addr << 8), ioaddr + MMDIO);
// Wait for the read bit to be cleared
    while (limit--) {
    cmd = ioread16(ioaddr + MMDIO);
    if (!(cmd & MDIO_READ))
    break;
    udelay(1);
    }
    if (limit < 0)
    return -ETIMEDOUT;
    return ioread16(ioaddr + MMRD);
    }
// Write a word data from PHY Chip
    static int r6040_phy_write(void __iomem *ioaddr,
    int phy_addr, int reg, u16 val)
    {
    let mut limit: c_int = MAC_DEF_TIMEOUT;
    u16 cmd;
    iowrite16(val, ioaddr + MMWD);
// Write the command to the MDIO bus
    iowrite16(MDIO_WRITE | reg | (phy_addr << 8), ioaddr + MMDIO);
// Wait for the write bit to be cleared
    while (limit--) {
    cmd = ioread16(ioaddr + MMDIO);
    if (!(cmd & MDIO_WRITE))
    break;
    udelay(1);
    }
    return (limit < 0) ? -ETIMEDOUT : 0;
    }
#[no_mangle]
unsafe extern "C" fn r6040_mdiobus_read(bus: *mut mii_bus, phy_addr: c_int, reg: c_int) -> c_int {
    static int r6040_mdiobus_read(struct mii_bus *bus, int phy_addr, int reg)
    {
    struct net_device *dev = bus.priv;
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    return r6040_phy_read(ioaddr, phy_addr, reg);
    }
    static int r6040_mdiobus_write(struct mii_bus *bus, int phy_addr,
    int reg, u16 value)
    {
    struct net_device *dev = bus.priv;
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    return r6040_phy_write(ioaddr, phy_addr, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn r6040_free_txbufs(dev: *mut net_device) {
    static void r6040_free_txbufs(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    int i;
    for (i = 0; i < TX_DCNT; i++) {
    if (lp.tx_insert_ptr.skb_ptr) {
    dma_unmap_single(&lp.pdev.dev,
    le32_to_cpu(lp.tx_insert_ptr.buf),
    MAX_BUF_SIZE, DMA_TO_DEVICE);
    dev_kfree_skb(lp.tx_insert_ptr.skb_ptr);
    lp.tx_insert_ptr.skb_ptr = core::ptr::null_mut();
    }
    lp.tx_insert_ptr = lp.tx_insert_ptr.vndescp;
    }
    }
#[no_mangle]
unsafe extern "C" fn r6040_free_rxbufs(dev: *mut net_device) {
    static void r6040_free_rxbufs(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    int i;
    for (i = 0; i < RX_DCNT; i++) {
    if (lp.rx_insert_ptr.skb_ptr) {
    dma_unmap_single(&lp.pdev.dev,
    le32_to_cpu(lp.rx_insert_ptr.buf),
    MAX_BUF_SIZE, DMA_FROM_DEVICE);
    dev_kfree_skb(lp.rx_insert_ptr.skb_ptr);
    lp.rx_insert_ptr.skb_ptr = core::ptr::null_mut();
    }
    lp.rx_insert_ptr = lp.rx_insert_ptr.vndescp;
    }
    }
    static void r6040_init_ring_desc(struct r6040_descriptor *desc_ring,
    dma_addr_t desc_dma, int size)
    {
    struct r6040_descriptor *desc = desc_ring;
    let mut mapping: dma_addr_t = desc_dma;
    while (size-- > 0) {
    mapping += sizeof(*desc);
    desc.ndesc = cpu_to_le32(mapping);
    desc.vndescp = desc + 1;
    desc++;
    }
    desc--;
    desc.ndesc = cpu_to_le32(desc_dma);
    desc.vndescp = desc_ring;
    }
#[no_mangle]
unsafe extern "C" fn r6040_init_txbufs(dev: *mut net_device) {
    static void r6040_init_txbufs(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    lp.tx_free_desc = TX_DCNT;
    lp.tx_remove_ptr = lp.tx_insert_ptr = lp.tx_ring;
    r6040_init_ring_desc(lp.tx_ring, lp.tx_ring_dma, TX_DCNT);
    }
#[no_mangle]
unsafe extern "C" fn r6040_alloc_rxbufs(dev: *mut net_device) -> c_int {
    static int r6040_alloc_rxbufs(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    struct r6040_descriptor *desc;
    struct sk_buff *skb;
    int rc;
    lp.rx_remove_ptr = lp.rx_insert_ptr = lp.rx_ring;
    r6040_init_ring_desc(lp.rx_ring, lp.rx_ring_dma, RX_DCNT);
// Allocate skbs for the rx descriptors
    desc = lp.rx_ring;
    do {
    skb = netdev_alloc_skb(dev, MAX_BUF_SIZE);
    if (!skb) {
    rc = -ENOMEM;
    goto err_exit;
    }
    desc.skb_ptr = skb;
    desc.buf = cpu_to_le32(dma_map_single(&lp.pdev.dev,
    desc.skb_ptr.data,
    MAX_BUF_SIZE,
    DMA_FROM_DEVICE));
    desc.status = DSC_OWNER_MAC;
    desc = desc.vndescp;
    } while (desc != lp.rx_ring);
    return 0;
    err_exit:
// Deallocate all previously allocated skbs
    r6040_free_rxbufs(dev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn r6040_reset_mac(lp: *mut r6040_private) {
    static void r6040_reset_mac(struct r6040_private *lp)
    {
    void __iomem *ioaddr = lp.base;
    let mut limit: c_int = MAC_DEF_TIMEOUT;
    u16 cmd, md_csc;
    md_csc = ioread16(ioaddr + MD_CSC);
    iowrite16(MAC_RST, ioaddr + MCR1);
    while (limit--) {
    cmd = ioread16(ioaddr + MCR1);
    if (cmd & MAC_RST)
    break;
    }
// Reset internal state machine
    iowrite16(MAC_SM_RST, ioaddr + MAC_SM);
    iowrite16(0, ioaddr + MAC_SM);
    mdelay(5);
// Restore MDIO clock frequency
    if (md_csc != MD_CSC_DEFAULT)
    iowrite16(md_csc, ioaddr + MD_CSC);
    }
#[no_mangle]
unsafe extern "C" fn r6040_init_mac_regs(dev: *mut net_device) {
    static void r6040_init_mac_regs(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
// Mask Off Interrupt
    iowrite16(MSK_INT, ioaddr + MIER);
// Reset RDC MAC
    r6040_reset_mac(lp);
// MAC Bus Control Register
    iowrite16(MBCR_DEFAULT, ioaddr + MBCR);
// Buffer Size Register
    iowrite16(MAX_BUF_SIZE, ioaddr + MR_BSR);
// Write TX ring start address
    iowrite16(lp.tx_ring_dma, ioaddr + MTD_SA0);
    iowrite16(lp.tx_ring_dma >> 16, ioaddr + MTD_SA1);
// Write RX ring start address
    iowrite16(lp.rx_ring_dma, ioaddr + MRD_SA0);
    iowrite16(lp.rx_ring_dma >> 16, ioaddr + MRD_SA1);
// Set interrupt waiting time and packet numbers
    iowrite16(0, ioaddr + MT_ICR);
    iowrite16(0, ioaddr + MR_ICR);
// Enable interrupts
    iowrite16(INT_MASK, ioaddr + MIER);
// Enable TX and RX
    iowrite16(lp.mcr0 | MCR0_RCVEN, ioaddr);
// Let TX poll the descriptors
// we may got called by r6040_tx_timeout which has left
// some unsent tx buffers
    iowrite16(TM2TX, ioaddr + MTPR);
    }
#[no_mangle]
unsafe extern "C" fn r6040_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void r6040_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct r6040_private *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    netdev_warn(dev, "transmit timed out, int enable %4.4x "
    "status %4.4x\n",
    ioread16(ioaddr + MIER),
    ioread16(ioaddr + MISR));
    dev.stats.tx_errors++;
// Reset MAC and re-init all registers
    r6040_init_mac_regs(dev);
    }
    static struct net_device_stats *r6040_get_stats(struct net_device *dev)
    {
    struct r6040_private *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    dev.stats.rx_crc_errors += ioread8(ioaddr + ME_CNT1);
    dev.stats.multicast += ioread8(ioaddr + ME_CNT0);
    spin_unlock_irqrestore(&priv.lock, flags);
    return &dev.stats;
    }
// Stop RDC MAC and Free the allocated resource
#[no_mangle]
unsafe extern "C" fn r6040_down(dev: *mut net_device) {
    static void r6040_down(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    const u16 *adrp;
// Stop MAC
    iowrite16(MSK_INT, ioaddr + MIER);	/* Mask Off Interrupt */
// Reset RDC MAC
    r6040_reset_mac(lp);
// Restore MAC Address to MIDx
    adrp = (const u16 *) dev.dev_addr;
    iowrite16(adrp[0], ioaddr + MID_0L);
    iowrite16(adrp[1], ioaddr + MID_0M);
    iowrite16(adrp[2], ioaddr + MID_0H);
    }
#[no_mangle]
unsafe extern "C" fn r6040_close(dev: *mut net_device) -> c_int {
    static int r6040_close(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    struct pci_dev *pdev = lp.pdev;
    phy_stop(dev.phydev);
    napi_disable(&lp.napi);
    netif_stop_queue(dev);
    spin_lock_irq(&lp.lock);
    r6040_down(dev);
// Free RX buffer
    r6040_free_rxbufs(dev);
// Free TX buffer
    r6040_free_txbufs(dev);
    spin_unlock_irq(&lp.lock);
    free_irq(dev.irq, dev);
// Free Descriptor memory
    if (lp.rx_ring) {
    dma_free_coherent(&pdev.dev, RX_DESC_SIZE, lp.rx_ring,
    lp.rx_ring_dma);
    lp.rx_ring = core::ptr::null_mut();
    }
    if (lp.tx_ring) {
    dma_free_coherent(&pdev.dev, TX_DESC_SIZE, lp.tx_ring,
    lp.tx_ring_dma);
    lp.tx_ring = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r6040_rx(dev: *mut net_device, limit: c_int) -> c_int {
    static int r6040_rx(struct net_device *dev, int limit)
    {
    struct r6040_private *priv = netdev_priv(dev);
    struct r6040_descriptor *descptr = priv.rx_remove_ptr;
    struct sk_buff *skb_ptr, *new_skb;
    let mut count: c_int = 0;
    u16 err;
// Limit not reached and the descriptor belongs to the CPU
    while (count < limit && !(descptr.status & DSC_OWNER_MAC)) {
// Read the descriptor status
    err = descptr.status;
// Global error status set
    if (err & DSC_RX_ERR) {
// RX dribble
    if (err & DSC_RX_ERR_DRI)
    dev.stats.rx_frame_errors++;
// Buffer length exceeded
    if (err & DSC_RX_ERR_BUF)
    dev.stats.rx_length_errors++;
// Packet too long
    if (err & DSC_RX_ERR_LONG)
    dev.stats.rx_length_errors++;
// Packet < 64 bytes
    if (err & DSC_RX_ERR_RUNT)
    dev.stats.rx_length_errors++;
// CRC error
    if (err & DSC_RX_ERR_CRC) {
    spin_lock(&priv.lock);
    dev.stats.rx_crc_errors++;
    spin_unlock(&priv.lock);
    }
    goto next_descr;
    }
// Packet successfully received
    new_skb = netdev_alloc_skb(dev, MAX_BUF_SIZE);
    if (!new_skb) {
    dev.stats.rx_dropped++;
    goto next_descr;
    }
    skb_ptr = descptr.skb_ptr;
    skb_ptr.dev = priv.dev;
// Do not count the CRC
    skb_put(skb_ptr, descptr.len - ETH_FCS_LEN);
    dma_unmap_single(&priv.pdev.dev, le32_to_cpu(descptr.buf),
    MAX_BUF_SIZE, DMA_FROM_DEVICE);
    skb_ptr.protocol = eth_type_trans(skb_ptr, priv.dev);
// Send to upper layer
    netif_receive_skb(skb_ptr);
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += descptr.len - ETH_FCS_LEN;
// put new skb into descriptor
    descptr.skb_ptr = new_skb;
    descptr.buf = cpu_to_le32(dma_map_single(&priv.pdev.dev,
    descptr.skb_ptr.data,
    MAX_BUF_SIZE,
    DMA_FROM_DEVICE));
    next_descr:
// put the descriptor back to the MAC
    descptr.status = DSC_OWNER_MAC;
    descptr = descptr.vndescp;
    count++;
    }
    priv.rx_remove_ptr = descptr;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn r6040_tx(dev: *mut net_device) {
    static void r6040_tx(struct net_device *dev)
    {
    struct r6040_private *priv = netdev_priv(dev);
    struct r6040_descriptor *descptr;
    void __iomem *ioaddr = priv.base;
    struct sk_buff *skb_ptr;
    u16 err;
    spin_lock(&priv.lock);
    descptr = priv.tx_remove_ptr;
    while (priv.tx_free_desc < TX_DCNT) {
// Check for errors
    err = ioread16(ioaddr + MLSR);
    if (err & TX_FIFO_UNDR)
    dev.stats.tx_fifo_errors++;
    if (err & (TX_EXCEEDC | TX_LATEC))
    dev.stats.tx_carrier_errors++;
    if (descptr.status & DSC_OWNER_MAC)
    break; /* Not complete */
    skb_ptr = descptr.skb_ptr;
// Statistic Counter
    dev.stats.tx_packets++;
    dev.stats.tx_bytes += skb_ptr.len;
    dma_unmap_single(&priv.pdev.dev, le32_to_cpu(descptr.buf),
    skb_ptr.len, DMA_TO_DEVICE);
// Free buffer
    dev_kfree_skb(skb_ptr);
    descptr.skb_ptr = core::ptr::null_mut();
// To next descriptor
    descptr = descptr.vndescp;
    priv.tx_free_desc++;
    }
    priv.tx_remove_ptr = descptr;
    if (priv.tx_free_desc)
    netif_wake_queue(dev);
    spin_unlock(&priv.lock);
    }
#[no_mangle]
unsafe extern "C" fn r6040_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int r6040_poll(struct napi_struct *napi, int budget)
    {
    struct r6040_private *priv =
    container_of(napi, struct r6040_private, napi);
    struct net_device *dev = priv.dev;
    void __iomem *ioaddr = priv.base;
    int work_done;
    r6040_tx(dev);
    work_done = r6040_rx(dev, budget);
    if (work_done < budget) {
    napi_complete_done(napi, work_done);
// Enable RX/TX interrupt
    iowrite16(ioread16(ioaddr + MIER) | RX_INTS | TX_INTS,
    ioaddr + MIER);
    }
    return work_done;
    }
// The RDC interrupt handler.
#[no_mangle]
unsafe extern "C" fn r6040_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t r6040_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    u16 misr, status;
// Save MIER
    misr = ioread16(ioaddr + MIER);
// Mask off RDC MAC interrupt
    iowrite16(MSK_INT, ioaddr + MIER);
// Read MISR status and clear
    status = ioread16(ioaddr + MISR);
    if (status == 0x0000 || status == 0xffff) {
// Restore RDC MAC interrupt
    iowrite16(misr, ioaddr + MIER);
    return IRQ_NONE;
    }
// RX interrupt request
    if (status & (RX_INTS | TX_INTS)) {
    if (status & RX_NO_DESC) {
// RX descriptor unavailable
    dev.stats.rx_dropped++;
    dev.stats.rx_missed_errors++;
    }
    if (status & RX_FIFO_FULL)
    dev.stats.rx_fifo_errors++;
    if (likely(napi_schedule_prep(&lp.napi))) {
// Mask off RX interrupt
    misr &= ~(RX_INTS | TX_INTS);
    __napi_schedule_irqoff(&lp.napi);
    }
    }
// Restore RDC MAC interrupt
    iowrite16(misr, ioaddr + MIER);
    return IRQ_HANDLED;
    }

#[no_mangle]
unsafe extern "C" fn r6040_poll_controller(dev: *mut net_device) {
    static void r6040_poll_controller(struct net_device *dev)
    {
    disable_irq(dev.irq);
    r6040_interrupt(dev.irq, dev);
    enable_irq(dev.irq);
    }

// Init RDC MAC
#[no_mangle]
unsafe extern "C" fn r6040_up(dev: *mut net_device) -> c_int {
    static int r6040_up(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    int ret;
// Initialise and alloc RX/TX buffers
    r6040_init_txbufs(dev);
    ret = r6040_alloc_rxbufs(dev);
    if (ret)
    return ret;
// improve performance (by RDC guys)
    r6040_phy_write(ioaddr, 30, 17,
    (r6040_phy_read(ioaddr, 30, 17) | 0x4000));
    r6040_phy_write(ioaddr, 30, 17,
    ~((~r6040_phy_read(ioaddr, 30, 17)) | 0x2000));
    r6040_phy_write(ioaddr, 0, 19, 0x0000);
    r6040_phy_write(ioaddr, 0, 30, 0x01F0);
// Initialize all MAC registers
    r6040_init_mac_regs(dev);
    phy_start(dev.phydev);
    return 0;
    }
// Read/set MAC address routines
#[no_mangle]
unsafe extern "C" fn r6040_mac_address(dev: *mut net_device) {
    static void r6040_mac_address(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    const u16 *adrp;
// Reset MAC
    r6040_reset_mac(lp);
// Restore MAC Address
    adrp = (const u16 *) dev.dev_addr;
    iowrite16(adrp[0], ioaddr + MID_0L);
    iowrite16(adrp[1], ioaddr + MID_0M);
    iowrite16(adrp[2], ioaddr + MID_0H);
    }
#[no_mangle]
unsafe extern "C" fn r6040_open(dev: *mut net_device) -> c_int {
    static int r6040_open(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    int ret;
// Request IRQ and Register interrupt handler
    ret = request_irq(dev.irq, r6040_interrupt,
    IRQF_SHARED, dev.name, dev);
    if (ret)
    goto out;
// Set MAC address
    r6040_mac_address(dev);
// Allocate Descriptor memory
    lp.rx_ring =
    dma_alloc_coherent(&lp.pdev.dev, RX_DESC_SIZE,
    &lp.rx_ring_dma, GFP_KERNEL);
    if (!lp.rx_ring) {
    ret = -ENOMEM;
    goto err_free_irq;
    }
    lp.tx_ring =
    dma_alloc_coherent(&lp.pdev.dev, TX_DESC_SIZE,
    &lp.tx_ring_dma, GFP_KERNEL);
    if (!lp.tx_ring) {
    ret = -ENOMEM;
    goto err_free_rx_ring;
    }
    ret = r6040_up(dev);
    if (ret)
    goto err_free_tx_ring;
    napi_enable(&lp.napi);
    netif_start_queue(dev);
    return 0;
    err_free_tx_ring:
    dma_free_coherent(&lp.pdev.dev, TX_DESC_SIZE, lp.tx_ring,
    lp.tx_ring_dma);
    err_free_rx_ring:
    dma_free_coherent(&lp.pdev.dev, RX_DESC_SIZE, lp.rx_ring,
    lp.rx_ring_dma);
    err_free_irq:
    free_irq(dev.irq, dev);
    out:
    return ret;
    }
    static netdev_tx_t r6040_start_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    struct r6040_descriptor *descptr;
    void __iomem *ioaddr = lp.base;
    unsigned long flags;
    if (skb_put_padto(skb, ETH_ZLEN) < 0)
    return NETDEV_TX_OK;
// Critical Section
    spin_lock_irqsave(&lp.lock, flags);
// TX resource check
    if (!lp.tx_free_desc) {
    spin_unlock_irqrestore(&lp.lock, flags);
    netif_stop_queue(dev);
    netdev_err(dev, ": no tx descriptor\n");
    return NETDEV_TX_BUSY;
    }
// Set TX descriptor & Transmit it
    lp.tx_free_desc--;
    descptr = lp.tx_insert_ptr;
    descptr.len = skb.len;
    descptr.skb_ptr = skb;
    descptr.buf = cpu_to_le32(dma_map_single(&lp.pdev.dev, skb.data,
    skb.len, DMA_TO_DEVICE));
    descptr.status = DSC_OWNER_MAC;
    skb_tx_timestamp(skb);
// Trigger the MAC to check the TX descriptor
    if (!netdev_xmit_more() || netif_queue_stopped(dev))
    iowrite16(TM2TX, ioaddr + MTPR);
    lp.tx_insert_ptr = descptr.vndescp;
// If no tx resource, stop
    if (!lp.tx_free_desc)
    netif_stop_queue(dev);
    spin_unlock_irqrestore(&lp.lock, flags);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn r6040_multicast_list(dev: *mut net_device) {
    static void r6040_multicast_list(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    void __iomem *ioaddr = lp.base;
    unsigned long flags;
    struct netdev_hw_addr *ha;
    int i;
    const u16 *adrp;
    u16 hash_table[4] = { 0 };
    spin_lock_irqsave(&lp.lock, flags);
// Keep our MAC Address
    adrp = (const u16 *)dev.dev_addr;
    iowrite16(adrp[0], ioaddr + MID_0L);
    iowrite16(adrp[1], ioaddr + MID_0M);
    iowrite16(adrp[2], ioaddr + MID_0H);
// Clear AMCP & PROM bits
    lp.mcr0 = ioread16(ioaddr + MCR0) & ~(MCR0_PROMISC | MCR0_HASH_EN);
// Promiscuous mode
    if (dev.flags & IFF_PROMISC)
    lp.mcr0 |= MCR0_PROMISC;
// Enable multicast hash table function to
// receive all multicast packets.
#[no_mangle]
pub unsafe extern "C" fn if(IFF_ALLMULTI: dev->flags &) -> else {
    lp.mcr0 |= MCR0_HASH_EN;
    for (i = 0; i < MCAST_MAX ; i++) {
    iowrite16(0, ioaddr + MID_1L + 8 * i);
    iowrite16(0, ioaddr + MID_1M + 8 * i);
    iowrite16(0, ioaddr + MID_1H + 8 * i);
    }
    for (i = 0; i < 4; i++)
    hash_table[i] = 0xffff;
    }
// Use internal multicast address registers if the number of
// multicast addresses is not greater than MCAST_MAX.
#[no_mangle]
pub unsafe extern "C" fn if(MCAST_MAX: netdev_mc_count(dev) <=) -> else {
    i = 0;
    netdev_for_each_mc_addr(ha, dev) {
    u16 *adrp = (u16 *) ha.addr;
    iowrite16(adrp[0], ioaddr + MID_1L + 8 * i);
    iowrite16(adrp[1], ioaddr + MID_1M + 8 * i);
    iowrite16(adrp[2], ioaddr + MID_1H + 8 * i);
    i++;
    }
    while (i < MCAST_MAX) {
    iowrite16(0, ioaddr + MID_1L + 8 * i);
    iowrite16(0, ioaddr + MID_1M + 8 * i);
    iowrite16(0, ioaddr + MID_1H + 8 * i);
    i++;
    }
    }
// Otherwise, Enable multicast hash table function.
    else {
    u32 crc;
    lp.mcr0 |= MCR0_HASH_EN;
    for (i = 0; i < MCAST_MAX ; i++) {
    iowrite16(0, ioaddr + MID_1L + 8 * i);
    iowrite16(0, ioaddr + MID_1M + 8 * i);
    iowrite16(0, ioaddr + MID_1H + 8 * i);
    }
// Build multicast hash table
    netdev_for_each_mc_addr(ha, dev) {
    u8 *addrs = ha.addr;
    crc = ether_crc(ETH_ALEN, addrs);
    crc >>= 26;
    hash_table[crc >> 4] |= 1 << (crc & 0xf);
    }
    }
    iowrite16(lp.mcr0, ioaddr + MCR0);
// Fill the MAC hash tables with their values
    if (lp.mcr0 & MCR0_HASH_EN) {
    iowrite16(hash_table[0], ioaddr + MAR0);
    iowrite16(hash_table[1], ioaddr + MAR1);
    iowrite16(hash_table[2], ioaddr + MAR2);
    iowrite16(hash_table[3], ioaddr + MAR3);
    }
    spin_unlock_irqrestore(&lp.lock, flags);
    }
    static void netdev_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct r6040_private *rp = netdev_priv(dev);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    strscpy(info.version, DRV_VERSION, sizeof(info.version));
    strscpy(info.bus_info, pci_name(rp.pdev), sizeof(info.bus_info));
    }
    static const struct ethtool_ops netdev_ethtool_ops = {
    .get_drvinfo		= netdev_get_drvinfo,
    .get_link		= ethtool_op_get_link,
    .get_ts_info		= ethtool_op_get_ts_info,
    .get_link_ksettings     = phy_ethtool_get_link_ksettings,
    .set_link_ksettings     = phy_ethtool_set_link_ksettings,
    .nway_reset		= phy_ethtool_nway_reset,
    };
    static const struct net_device_ops r6040_netdev_ops = {
    .ndo_open		= r6040_open,
    .ndo_stop		= r6040_close,
    .ndo_start_xmit		= r6040_start_xmit,
    .ndo_get_stats		= r6040_get_stats,
    .ndo_set_rx_mode	= r6040_multicast_list,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,
    .ndo_eth_ioctl		= phy_do_ioctl,
    .ndo_tx_timeout		= r6040_tx_timeout,

    .ndo_poll_controller	= r6040_poll_controller,

    };
#[no_mangle]
unsafe extern "C" fn r6040_adjust_link(dev: *mut net_device) {
    static void r6040_adjust_link(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    struct phy_device *phydev = dev.phydev;
    let mut status_changed: c_int = 0;
    void __iomem *ioaddr = lp.base;
    BUG_ON(!phydev);
    if (lp.old_link != phydev.link) {
    status_changed = 1;
    lp.old_link = phydev.link;
    }
// reflect duplex change
    if (phydev.link && (lp.old_duplex != phydev.duplex)) {
    lp.mcr0 |= (phydev.duplex == DUPLEX_FULL ? MCR0_FD : 0);
    iowrite16(lp.mcr0, ioaddr);
    status_changed = 1;
    lp.old_duplex = phydev.duplex;
    }
    if (status_changed)
    phy_print_status(phydev);
    }
#[no_mangle]
unsafe extern "C" fn r6040_mii_probe(dev: *mut net_device) -> c_int {
    static int r6040_mii_probe(struct net_device *dev)
    {
    struct r6040_private *lp = netdev_priv(dev);
    struct phy_device *phydev = core::ptr::null_mut();
    phydev = phy_find_first(lp.mii_bus);
    if (!phydev) {
    dev_err(&lp.pdev.dev, "no PHY found\n");
    return -ENODEV;
    }
    phydev = phy_connect(dev, phydev_name(phydev), &r6040_adjust_link,
    PHY_INTERFACE_MODE_MII);
    if (IS_ERR(phydev)) {
    dev_err(&lp.pdev.dev, "could not attach to PHY\n");
    return PTR_ERR(phydev);
    }
    phy_set_max_speed(phydev, SPEED_100);
    lp.old_link = 0;
    lp.old_duplex = -1;
    phy_attached_info(phydev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r6040_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int r6040_init_one(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct net_device *dev;
    struct r6040_private *lp;
    void __iomem *ioaddr;
    int err, io_size = R6040_IO_SIZE;
    let mut card_idx: static int = -1;
    u16 addr[ETH_ALEN / 2];
    let mut bar: c_int = 0;
    pr_info("%s\n", version);
    err = pci_enable_device(pdev);
    if (err)
    goto err_out;
// this should always be supported
    err = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_err(&pdev.dev, "32-bit PCI DMA addresses not supported by the card\n");
    goto err_out_disable_dev;
    }
    err = dma_set_coherent_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_err(&pdev.dev, "32-bit PCI DMA addresses not supported by the card\n");
    goto err_out_disable_dev;
    }
// IO Size check
    if (pci_resource_len(pdev, bar) < io_size) {
    dev_err(&pdev.dev, "Insufficient PCI resources, aborting\n");
    err = -EIO;
    goto err_out_disable_dev;
    }
    pci_set_master(pdev);
    dev = alloc_etherdev(sizeof(struct r6040_private));
    if (!dev) {
    err = -ENOMEM;
    goto err_out_disable_dev;
    }
    SET_NETDEV_DEV(dev, &pdev.dev);
    lp = netdev_priv(dev);
    err = pci_request_regions(pdev, DRV_NAME);
    if (err) {
    dev_err(&pdev.dev, "Failed to request PCI regions\n");
    goto err_out_free_dev;
    }
    ioaddr = pci_iomap(pdev, bar, io_size);
    if (!ioaddr) {
    dev_err(&pdev.dev, "ioremap failed for device\n");
    err = -EIO;
    goto err_out_free_res;
    }
// If PHY status change register is still set to zero it means the
// bootloader didn't initialize it, so we set it to:
// - enable phy status change
// - enable all phy addresses
// - set to lowest timer divider
    if (ioread16(ioaddr + PHY_CC) == 0)
    iowrite16(SCEN | PHY_MAX_ADDR << PHYAD_SHIFT |
    7 << TMRDIV_SHIFT, ioaddr + PHY_CC);
// Init system & device
    lp.base = ioaddr;
    dev.irq = pdev.irq;
    spin_lock_init(&lp.lock);
    pci_set_drvdata(pdev, dev);
// Set MAC address
    card_idx++;
    addr[0] = ioread16(ioaddr + MID_0L);
    addr[1] = ioread16(ioaddr + MID_0M);
    addr[2] = ioread16(ioaddr + MID_0H);
    eth_hw_addr_set(dev, (u8 *)addr);
// Some bootloader/BIOSes do not initialize
// MAC address, warn about that
    if (!(addr[0] || addr[1] || addr[2])) {
    netdev_warn(dev, "MAC address not initialized, "
    "generating random\n");
    eth_hw_addr_random(dev);
    }
// Link new device into r6040_root_dev
    lp.pdev = pdev;
    lp.dev = dev;
// Init RDC private data
    lp.mcr0 = MCR0_XMTEN | MCR0_RCVEN;
// The RDC-specific entries in the device structure.
    dev.netdev_ops = &r6040_netdev_ops;
    dev.ethtool_ops = &netdev_ethtool_ops;
    dev.watchdog_timeo = TX_TIMEOUT;
    netif_napi_add(dev, &lp.napi, r6040_poll);
    lp.mii_bus = mdiobus_alloc();
    if (!lp.mii_bus) {
    dev_err(&pdev.dev, "mdiobus_alloc() failed\n");
    err = -ENOMEM;
    goto err_out_unmap;
    }
    lp.mii_bus.priv = dev;
    lp.mii_bus.read = r6040_mdiobus_read;
    lp.mii_bus.write = r6040_mdiobus_write;
    lp.mii_bus.name = "r6040_eth_mii";
    snprintf(lp.mii_bus.id, MII_BUS_ID_SIZE, "%s-%x",
    dev_name(&pdev.dev), card_idx);
    err = mdiobus_register(lp.mii_bus);
    if (err) {
    dev_err(&pdev.dev, "failed to register MII bus\n");
    goto err_out_mdio;
    }
    err = r6040_mii_probe(dev);
    if (err) {
    dev_err(&pdev.dev, "failed to probe MII bus\n");
    goto err_out_mdio_unregister;
    }
// Register net device. After this dev->name assign
    err = register_netdev(dev);
    if (err) {
    dev_err(&pdev.dev, "Failed to register net device\n");
    goto err_out_phy_disconnect;
    }
    return 0;
    err_out_phy_disconnect:
    phy_disconnect(dev.phydev);
    err_out_mdio_unregister:
    mdiobus_unregister(lp.mii_bus);
    err_out_mdio:
    mdiobus_free(lp.mii_bus);
    err_out_unmap:
    netif_napi_del(&lp.napi);
    pci_iounmap(pdev, ioaddr);
    err_out_free_res:
    pci_release_regions(pdev);
    err_out_free_dev:
    free_netdev(dev);
    err_out_disable_dev:
    pci_disable_device(pdev);
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn r6040_remove_one(pdev: *mut pci_dev) {
    static void r6040_remove_one(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct r6040_private *lp = netdev_priv(dev);
    unregister_netdev(dev);
    phy_disconnect(dev.phydev);
    mdiobus_unregister(lp.mii_bus);
    mdiobus_free(lp.mii_bus);
    netif_napi_del(&lp.napi);
    pci_iounmap(pdev, lp.base);
    pci_release_regions(pdev);
    free_netdev(dev);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id r6040_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_RDC, 0x6040) },
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, r6040_pci_tbl);
    static struct pci_driver r6040_driver = {
    .name		= DRV_NAME,
    .id_table	= r6040_pci_tbl,
    .probe		= r6040_init_one,
    .remove		= r6040_remove_one,
    };
    module_pci_driver(r6040_driver);
