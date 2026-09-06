//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/micrel/ks8851_spi.c
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
// drivers/net/ethernet/micrel/ks8851.c
//
// Copyright 2009 Simtec Electronics
// http://www.simtec.co.uk
// Ben Dooks <ben@simtec.co.uk>
//

    static int msg_enable;
//
// struct ks8851_net_spi - KS8851 SPI driver private data
// @lock: Lock to ensure that the device is not accessed when busy.
// @tx_work: Work queue for tx packets
// @ks8851: KS8851 driver common private data
// @spidev: The spi device we're bound to.
// @spi_msg1: pre-setup SPI transfer with one message, @spi_xfer1.
// @spi_msg2: pre-setup SPI transfer with two messages, @spi_xfer2.
// @spi_xfer1: @spi_msg1 SPI transfer structure
// @spi_xfer2: @spi_msg2 SPI transfer structure
//
// The @lock ensures that the chip is protected when certain operations are
// in progress. When the read or write packet transfer is in progress, most
// of the chip registers are not accessible until the transfer is finished and
// the DMA has been de-asserted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks8851_net_spi {
    pub ks8851: ks8851_net,
    pub lock: mutex,
    pub tx_work: work_struct,
    pub spidev: *mut spi_device,
    pub spi_msg1: spi_message,
    pub spi_msg2: spi_message,
    pub spi_xfer1: spi_transfer,
    pub spi_xfer2: [spi_transfer; 2],
}

// SPI frame opcodes
pub const KS_SPIOP_RD: c_uint = 0x00;
pub const KS_SPIOP_WR: c_uint = 0x40;
pub const KS_SPIOP_RXFIFO: c_uint = 0x80;
pub const KS_SPIOP_TXFIFO: c_uint = 0xC0;
// shift for byte-enable data

// turn register number and byte-enable mask into data for start of packet

    (BYTE_EN(_byteen) | (_reg) << (8 + 2) | (_reg) >> 6)
//
// ks8851_lock_spi - register access lock
// @ks: The chip state
//
// Claim chip register access lock
//
#[no_mangle]
unsafe extern "C" fn ks8851_lock_spi(ks: *mut ks8851_net) {
    static void ks8851_lock_spi(struct ks8851_net *ks)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    mutex_lock(&kss.lock);
    }
//
// ks8851_unlock_spi - register access unlock
// @ks: The chip state
//
// Release chip register access lock
//
#[no_mangle]
unsafe extern "C" fn ks8851_unlock_spi(ks: *mut ks8851_net) {
    static void ks8851_unlock_spi(struct ks8851_net *ks)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    mutex_unlock(&kss.lock);
    }
// SPI register read/write calls.
//
// All these calls issue SPI transactions to access the chip's registers. They
// all require that the necessary lock is held to prevent accesses when the
// chip is busy transferring packet data (RX/TX FIFO accesses).
//
// ks8851_wrreg16_spi - write 16bit register value to chip via SPI
// @ks: The chip state
// @reg: The register address
// @val: The value to write
//
// Issue a write to put the value @val into the register specified in @reg.
//
    static void ks8851_wrreg16_spi(struct ks8851_net *ks, unsigned int reg,
    unsigned int val)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    struct spi_transfer *xfer = &kss.spi_xfer1;
    struct spi_message *msg = &kss.spi_msg1;
    __le16 txb[2];
    int ret;
    txb[0] = cpu_to_le16(MK_OP(reg & 2 ? 0xC : 0x03, reg) | KS_SPIOP_WR);
    txb[1] = cpu_to_le16(val);
    xfer.tx_buf = txb;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = 4;
    ret = spi_sync(kss.spidev, msg);
    if (ret < 0)
    netdev_err(ks.netdev, "spi_sync() failed\n");
    }
//
// ks8851_rdreg - issue read register command and return the data
// @ks: The device state
// @op: The register address and byte enables in message format.
// @rxb: The RX buffer to return the result into
// @rxl: The length of data expected.
//
// This is the low level read call that issues the necessary spi message(s)
// to read data from the register specified in @op.
//
    static void ks8851_rdreg(struct ks8851_net *ks, unsigned int op,
    u8 *rxb, unsigned int rxl)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    struct spi_transfer *xfer;
    struct spi_message *msg;
    __le16 *txb = (__le16 *)ks.txd;
    u8 *trx = ks.rxd;
    int ret;
    txb[0] = cpu_to_le16(op | KS_SPIOP_RD);
    if (kss.spidev.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    msg = &kss.spi_msg2;
    xfer = kss.spi_xfer2;
    xfer.tx_buf = txb;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = 2;
    xfer++;
    xfer.tx_buf = core::ptr::null_mut();
    xfer.rx_buf = trx;
    xfer.len = rxl;
    } else {
    msg = &kss.spi_msg1;
    xfer = &kss.spi_xfer1;
    xfer.tx_buf = txb;
    xfer.rx_buf = trx;
    xfer.len = rxl + 2;
    }
    ret = spi_sync(kss.spidev, msg);
    if (ret < 0)
    netdev_err(ks.netdev, "read: spi_sync() failed\n");
#[no_mangle]
pub unsafe extern "C" fn if(SPI_CONTROLLER_HALF_DUPLEX: kss->spidev->controller->flags &) -> else {
    else if (kss.spidev.controller.flags & SPI_CONTROLLER_HALF_DUPLEX)
    memcpy(rxb, trx, rxl);
    else
    memcpy(rxb, trx + 2, rxl);
    }
//
// ks8851_rdreg16_spi - read 16 bit register from device via SPI
// @ks: The chip information
// @reg: The register address
//
// Read a 16bit register from the chip, returning the result
//
#[no_mangle]
unsafe extern "C" fn ks8851_rdreg16_spi(ks: *mut ks8851_net, reg: c_uint) -> c_uint {
    static unsigned int ks8851_rdreg16_spi(struct ks8851_net *ks, unsigned int reg)
    {
    let mut rx: __le16 = 0;
    ks8851_rdreg(ks, MK_OP(reg & 2 ? 0xC : 0x3, reg), (u8 *)&rx, 2);
    return le16_to_cpu(rx);
    }
//
// ks8851_rdfifo_spi - read data from the receive fifo via SPI
// @ks: The device state.
// @buff: The buffer address
// @len: The length of the data to read
//
// Issue an RXQ FIFO read command and read the @len amount of data from
// the FIFO into the buffer specified by @buff.
//
#[no_mangle]
unsafe extern "C" fn ks8851_rdfifo_spi(ks: *mut ks8851_net, buff: *mut u8, len: c_uint) {
    static void ks8851_rdfifo_spi(struct ks8851_net *ks, u8 *buff, unsigned int len)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    struct spi_transfer *xfer = kss.spi_xfer2;
    struct spi_message *msg = &kss.spi_msg2;
    u8 txb[1];
    int ret;
    netif_dbg(ks, rx_status, ks.netdev,
    "%s: %d@%p\n", __func__, len, buff);
// set the operation we're issuing
    txb[0] = KS_SPIOP_RXFIFO;
    xfer.tx_buf = txb;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = 1;
    xfer++;
    xfer.rx_buf = buff;
    xfer.tx_buf = core::ptr::null_mut();
    xfer.len = len;
    ret = spi_sync(kss.spidev, msg);
    if (ret < 0)
    netdev_err(ks.netdev, "%s: spi_sync() failed\n", __func__);
    }
//
// ks8851_wrfifo_spi - write packet to TX FIFO via SPI
// @ks: The device state.
// @txp: The sk_buff to transmit.
// @irq: IRQ on completion of the packet.
//
// Send the @txp to the chip. This means creating the relevant packet header
// specifying the length of the packet and the other information the chip
// needs, such as IRQ on completion. Send the header and the packet data to
// the device.
//
    static void ks8851_wrfifo_spi(struct ks8851_net *ks, struct sk_buff *txp,
    bool irq)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    struct spi_transfer *xfer = kss.spi_xfer2;
    struct spi_message *msg = &kss.spi_msg2;
    let mut fid: c_uint = 0;
    int ret;
    netif_dbg(ks, tx_queued, ks.netdev, "%s: skb %p, %d@%p, irq %d\n",
    __func__, txp, txp.len, txp.data, irq);
    fid = ks.fid++;
    fid &= TXFR_TXFID_MASK;
    if (irq)
    fid |= TXFR_TXIC;	/* irq on completion */
// start header at txb[1] to align txw entries
    ks.txh.txb[1] = KS_SPIOP_TXFIFO;
    ks.txh.txw[1] = cpu_to_le16(fid);
    ks.txh.txw[2] = cpu_to_le16(txp.len);
    xfer.tx_buf = &ks.txh.txb[1];
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = 5;
    xfer++;
    xfer.tx_buf = txp.data;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = ALIGN(txp.len, 4);
    ret = spi_sync(kss.spidev, msg);
    if (ret < 0)
    netdev_err(ks.netdev, "%s: spi_sync() failed\n", __func__);
    }
//
// calc_txlen - calculate size of message to send packet
// @len: Length of data
//
// Returns the size of the TXFIFO message needed to send
// this packet.
//
#[no_mangle]
unsafe extern "C" fn calc_txlen(len: c_uint) -> c_uint {
    static unsigned int calc_txlen(unsigned int len)
    {
    return ALIGN(len + 4, 4);
    }
//
// ks8851_tx_work - process tx packet(s)
// @work: The work structure what was scheduled.
//
// This is called when a number of packets have been scheduled for
// transmission and need to be sent to the device.
//
#[no_mangle]
unsafe extern "C" fn ks8851_tx_work(work: *mut work_struct) {
    static void ks8851_tx_work(struct work_struct *work)
    {
    let mut dequeued_len: c_uint = 0;
    struct ks8851_net_spi *kss;
    unsigned short tx_space;
    struct ks8851_net *ks;
    struct sk_buff *txb;
    bool last;
    kss = container_of(work, struct ks8851_net_spi, tx_work);
    ks = &kss.ks8851;
    last = skb_queue_empty(&ks.txq);
    ks8851_lock_spi(ks);
    while (!last) {
    txb = skb_dequeue(&ks.txq);
    last = skb_queue_empty(&ks.txq);
    if (txb) {
    dequeued_len += calc_txlen(txb.len);
    ks8851_wrreg16_spi(ks, KS_RXQCR,
    ks.rc_rxqcr | RXQCR_SDA);
    ks8851_wrfifo_spi(ks, txb, last);
    ks8851_wrreg16_spi(ks, KS_RXQCR, ks.rc_rxqcr);
    ks8851_wrreg16_spi(ks, KS_TXQCR, TXQCR_METFE);
    ks8851_done_tx(ks, txb);
    }
    }
    tx_space = ks8851_rdreg16_spi(ks, KS_TXMIR);
    spin_lock_bh(&ks.statelock);
    ks.queued_len -= dequeued_len;
    ks.tx_space = tx_space;
    spin_unlock_bh(&ks.statelock);
    ks8851_unlock_spi(ks);
    }
//
// ks8851_flush_tx_work_spi - flush outstanding TX work
// @ks: The device state
//
#[no_mangle]
unsafe extern "C" fn ks8851_flush_tx_work_spi(ks: *mut ks8851_net) {
    static void ks8851_flush_tx_work_spi(struct ks8851_net *ks)
    {
    struct ks8851_net_spi *kss = to_ks8851_spi(ks);
    flush_work(&kss.tx_work);
    }
//
// ks8851_start_xmit_spi - transmit packet using SPI
// @skb: The buffer to transmit
// @dev: The device used to transmit the packet.
//
// Called by the network layer to transmit the @skb. Queue the packet for
// the device and schedule the necessary work to transmit the packet when
// it is free.
//
// We do this to firstly avoid sleeping with the network device locked,
// and secondly so we can round up more than one packet to transmit which
// means we can try and avoid generating too many transmit done interrupts.
//
    static netdev_tx_t ks8851_start_xmit_spi(struct sk_buff *skb,
    struct net_device *dev)
    {
    let mut needed: c_uint = calc_txlen(skb.len);
    struct ks8851_net *ks = netdev_priv(dev);
    let mut ret: netdev_tx_t = NETDEV_TX_OK;
    struct ks8851_net_spi *kss;
    kss = to_ks8851_spi(ks);
    netif_dbg(ks, tx_queued, ks.netdev,
    "%s: skb %p, %d@%p\n", __func__, skb, skb.len, skb.data);
    spin_lock(&ks.statelock);
    if (ks.queued_len + needed > ks.tx_space) {
    netif_stop_queue(dev);
    ret = NETDEV_TX_BUSY;
    } else {
    ks.queued_len += needed;
    skb_queue_tail(&ks.txq, skb);
    }
    spin_unlock(&ks.statelock);
    if (ret == NETDEV_TX_OK)
    schedule_work(&kss.tx_work);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ks8851_probe_spi(spi: *mut spi_device) -> c_int {
    static int ks8851_probe_spi(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct ks8851_net_spi *kss;
    struct net_device *netdev;
    struct ks8851_net *ks;
    netdev = devm_alloc_etherdev(dev, sizeof(struct ks8851_net_spi));
    if (!netdev)
    return -ENOMEM;
    spi.bits_per_word = 8;
    kss = netdev_priv(netdev);
    ks = &kss.ks8851;
    ks.lock = ks8851_lock_spi;
    ks.unlock = ks8851_unlock_spi;
    ks.rdreg16 = ks8851_rdreg16_spi;
    ks.wrreg16 = ks8851_wrreg16_spi;
    ks.rdfifo = ks8851_rdfifo_spi;
    ks.wrfifo = ks8851_wrfifo_spi;
    ks.start_xmit = ks8851_start_xmit_spi;
    ks.flush_tx_work = ks8851_flush_tx_work_spi;

    IRQ_TXI |	/* TX done */		\
    IRQ_RXI |	/* RX done */		\
    IRQ_SPIBEI |	/* SPI bus error */	\
    IRQ_TXPSI |	/* TX process stop */	\
    IRQ_RXPSI)	/* RX process stop */
    ks.rc_ier = STD_IRQ;
    kss.spidev = spi;
    mutex_init(&kss.lock);
    INIT_WORK(&kss.tx_work, ks8851_tx_work);
// initialise pre-made spi transfer messages
    spi_message_init(&kss.spi_msg1);
    spi_message_add_tail(&kss.spi_xfer1, &kss.spi_msg1);
    spi_message_init(&kss.spi_msg2);
    spi_message_add_tail(&kss.spi_xfer2[0], &kss.spi_msg2);
    spi_message_add_tail(&kss.spi_xfer2[1], &kss.spi_msg2);
    netdev.irq = spi.irq;
    return ks8851_probe_common(netdev, dev, msg_enable);
    }
#[no_mangle]
unsafe extern "C" fn ks8851_remove_spi(spi: *mut spi_device) {
    static void ks8851_remove_spi(struct spi_device *spi)
    {
    ks8851_remove_common(&spi.dev);
    }
    static const struct of_device_id ks8851_match_table[] = {
    { .compatible = "micrel,ks8851" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ks8851_match_table);
    static struct spi_driver ks8851_driver = {
    .driver = {
    .name = "ks8851",
    .of_match_table = ks8851_match_table,
    .pm = &ks8851_pm_ops,
    },
    .probe = ks8851_probe_spi,
    .remove = ks8851_remove_spi,
    };
    module_spi_driver(ks8851_driver);
    MODULE_DESCRIPTION("KS8851 Network driver");
    MODULE_AUTHOR("Ben Dooks <ben@simtec.co.uk>");
    MODULE_LICENSE("GPL");
    module_param_named(message, msg_enable, int, 0);
    MODULE_PARM_DESC(message, "Message verbosity level (0=none, 31=all)");
    MODULE_ALIAS("spi:ks8851");
