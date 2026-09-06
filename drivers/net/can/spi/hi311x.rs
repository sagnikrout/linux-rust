//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/spi/hi311x.c
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
// CAN bus driver for Holt HI3110 CAN Controller with SPI Interface
//
// Copyright(C) Timesys Corporation 2016
//
// Based on Microchip 251x CAN Controller (mcp251x) Linux kernel driver
// Copyright 2009 Christian Pellegrin EVOL S.r.l.
// Copyright 2007 Raymarine UK, Ltd. All Rights Reserved.
// Copyright 2006 Arcom Control Systems Ltd.
//
// Based on CAN bus driver for the CCAN controller written by
// - Sascha Hauer, Marc Kleine-Budde, Pengutronix
// - Simon Kallweit, intefo AG
// Copyright 2007
//

pub const HI3110_MASTER_RESET: c_uint = 0x56;
pub const HI3110_READ_CTRL0: c_uint = 0xD2;
pub const HI3110_READ_CTRL1: c_uint = 0xD4;
pub const HI3110_READ_STATF: c_uint = 0xE2;
pub const HI3110_WRITE_CTRL0: c_uint = 0x14;
pub const HI3110_WRITE_CTRL1: c_uint = 0x16;
pub const HI3110_WRITE_INTE: c_uint = 0x1C;
pub const HI3110_WRITE_BTR0: c_uint = 0x18;
pub const HI3110_WRITE_BTR1: c_uint = 0x1A;
pub const HI3110_READ_BTR0: c_uint = 0xD6;
pub const HI3110_READ_BTR1: c_uint = 0xD8;
pub const HI3110_READ_INTF: c_uint = 0xDE;
pub const HI3110_READ_ERR: c_uint = 0xDC;
pub const HI3110_READ_FIFO_WOTIME: c_uint = 0x48;
pub const HI3110_WRITE_FIFO: c_uint = 0x12;
pub const HI3110_READ_MESSTAT: c_uint = 0xDA;
pub const HI3110_READ_REC: c_uint = 0xEA;
pub const HI3110_READ_TEC: c_uint = 0xEC;

pub const HI3110_BTR0_SJW_SHIFT: c_int = 6;
pub const HI3110_BTR0_BRP_SHIFT: c_int = 0;

pub const HI3110_BTR1_TSEG2_SHIFT: c_int = 4;
pub const HI3110_BTR1_TSEG1_SHIFT: c_int = 0;
pub const HI3110_FIFO_WOTIME_TAG_OFF: c_int = 0;
pub const HI3110_FIFO_WOTIME_ID_OFF: c_int = 1;
pub const HI3110_FIFO_WOTIME_DLC_OFF: c_int = 5;
pub const HI3110_FIFO_WOTIME_DAT_OFF: c_int = 6;

pub const HI3110_FIFO_TAG_OFF: c_int = 0;
pub const HI3110_FIFO_ID_OFF: c_int = 1;
pub const HI3110_FIFO_STD_DLC_OFF: c_int = 3;
pub const HI3110_FIFO_STD_DATA_OFF: c_int = 4;
pub const HI3110_FIFO_EXT_DLC_OFF: c_int = 5;
pub const HI3110_FIFO_EXT_DATA_OFF: c_int = 6;
pub const HI3110_CAN_MAX_DATA_LEN: c_int = 8;
pub const HI3110_RX_BUF_LEN: c_int = 15;
pub const HI3110_TX_STD_BUF_LEN: c_int = 12;
pub const HI3110_TX_EXT_BUF_LEN: c_int = 14;
pub const HI3110_CAN_FRAME_MAX_BITS: c_int = 128;
pub const HI3110_EFF_FLAGS: c_uint = 0x18 /* IDE + SRR */;
pub const HI3110_TX_ECHO_SKB_MAX: c_int = 1;

    static const struct can_bittiming_const hi3110_bittiming_const = {
    .name = DEVICE_NAME,
    .tseg1_min = 2,
    .tseg1_max = 16,
    .tseg2_min = 2,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 64,
    .brp_inc = 1,
    };
    enum hi3110_model {
    CAN_HI3110_HI3110 = 0x3110,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3110_priv {
    pub can: can_priv,
    pub net: *mut net_device,
    pub spi: *mut spi_device,
    pub model: enum hi3110_model,
    pub /: *mut *mut mutex hi3110_lock; / SPI device lock,
    pub spi_tx_buf: *mut u8,
    pub spi_rx_buf: *mut u8,
    pub tx_skb: *mut sk_buff,
    pub wq: *mut workqueue_struct,
    pub tx_work: work_struct,
    pub restart_work: work_struct,
    pub force_quit: c_int,
    pub after_suspend: c_int,
pub const HI3110_AFTER_SUSPEND_UP: c_int = 1;
pub const HI3110_AFTER_SUSPEND_DOWN: c_int = 2;
pub const HI3110_AFTER_SUSPEND_POWER: c_int = 4;
pub const HI3110_AFTER_SUSPEND_RESTART: c_int = 8;
    pub restart_tx: c_int,
    pub tx_busy: bool,
    pub power: *mut regulator,
    pub transceiver: *mut regulator,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn hi3110_clean(net: *mut net_device) {
    static void hi3110_clean(struct net_device *net)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    if (priv.tx_skb || priv.tx_busy)
    net.stats.tx_errors++;
    dev_kfree_skb(priv.tx_skb);
    if (priv.tx_busy)
    can_free_echo_skb(priv.net, 0, core::ptr::null_mut());
    priv.tx_skb = core::ptr::null_mut();
    priv.tx_busy = false;
    }
// Note about handling of error return of hi3110_spi_trans: accessing
// registers via SPI is not really different conceptually than using
// normal I/O assembler instructions, although it's much more
// complicated from a practical POV. So it's not advisable to always
// check the return value of this function. Imagine that every
// read{b,l}, write{b,l} and friends would be bracketed in "if ( < 0)
// error();", it would be a great mess (well there are some situation
// when exception handling C++ like could be useful after all). So we
// just check that transfers are OK at the beginning of our
// conversation with the chip and to avoid doing really nasty things
// (like injecting bogus packets in the network stack).
//
#[no_mangle]
unsafe extern "C" fn hi3110_spi_trans(spi: *mut spi_device, len: c_int) -> c_int {
    static int hi3110_spi_trans(struct spi_device *spi, int len)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    struct spi_transfer t = {
    .tx_buf = priv.spi_tx_buf,
    .rx_buf = priv.spi_rx_buf,
    .len = len,
    .cs_change = 0,
    };
    struct spi_message m;
    int ret;
    spi_message_init(&m);
    spi_message_add_tail(&t, &m);
    ret = spi_sync(spi, &m);
    if (ret)
    dev_err(&spi.dev, "spi transfer failed: ret = %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_cmd(spi: *mut spi_device, command: u8) -> c_int {
    static int hi3110_cmd(struct spi_device *spi, u8 command)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = command;
    dev_dbg(&spi.dev, "hi3110_cmd: %02X\n", command);
    return hi3110_spi_trans(spi, 1);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_read(spi: *mut spi_device, command: u8) -> u8 {
    static u8 hi3110_read(struct spi_device *spi, u8 command)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    let mut val: u8 = 0;
    priv.spi_tx_buf[0] = command;
    hi3110_spi_trans(spi, 2);
    val = priv.spi_rx_buf[1];
    return val;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_write(spi: *mut spi_device, reg: u8, val: u8) {
    static void hi3110_write(struct spi_device *spi, u8 reg, u8 val)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = reg;
    priv.spi_tx_buf[1] = val;
    hi3110_spi_trans(spi, 2);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_tx_frame(spi: *mut spi_device, buf: *mut u8, len: c_int) {
    static void hi3110_hw_tx_frame(struct spi_device *spi, u8 *buf, int len)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = HI3110_WRITE_FIFO;
    memcpy(priv.spi_tx_buf + 1, buf, len);
    hi3110_spi_trans(spi, len + 1);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_tx(spi: *mut spi_device, frame: *mut can_frame) {
    static void hi3110_hw_tx(struct spi_device *spi, struct can_frame *frame)
    {
    u8 buf[HI3110_TX_EXT_BUF_LEN];
    buf[HI3110_FIFO_TAG_OFF] = 0;
    if (frame.can_id & CAN_EFF_FLAG) {
// Extended frame
    buf[HI3110_FIFO_ID_OFF] = (frame.can_id & CAN_EFF_MASK) >> 21;
    buf[HI3110_FIFO_ID_OFF + 1] =
    (((frame.can_id & CAN_EFF_MASK) >> 13) & 0xe0) |
    HI3110_EFF_FLAGS |
    (((frame.can_id & CAN_EFF_MASK) >> 15) & 0x07);
    buf[HI3110_FIFO_ID_OFF + 2] =
    (frame.can_id & CAN_EFF_MASK) >> 7;
    buf[HI3110_FIFO_ID_OFF + 3] =
    ((frame.can_id & CAN_EFF_MASK) << 1) |
    ((frame.can_id & CAN_RTR_FLAG) ? 1 : 0);
    buf[HI3110_FIFO_EXT_DLC_OFF] = frame.len;
    memcpy(buf + HI3110_FIFO_EXT_DATA_OFF,
    frame.data, frame.len);
    hi3110_hw_tx_frame(spi, buf, HI3110_TX_EXT_BUF_LEN -
    (HI3110_CAN_MAX_DATA_LEN - frame.len));
    } else {
// Standard frame
    buf[HI3110_FIFO_ID_OFF] =   (frame.can_id & CAN_SFF_MASK) >> 3;
    buf[HI3110_FIFO_ID_OFF + 1] =
    ((frame.can_id & CAN_SFF_MASK) << 5) |
    ((frame.can_id & CAN_RTR_FLAG) ? (1 << 4) : 0);
    buf[HI3110_FIFO_STD_DLC_OFF] = frame.len;
    memcpy(buf + HI3110_FIFO_STD_DATA_OFF,
    frame.data, frame.len);
    hi3110_hw_tx_frame(spi, buf, HI3110_TX_STD_BUF_LEN -
    (HI3110_CAN_MAX_DATA_LEN - frame.len));
    }
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_rx_frame(spi: *mut spi_device, buf: *mut u8) {
    static void hi3110_hw_rx_frame(struct spi_device *spi, u8 *buf)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    priv.spi_tx_buf[0] = HI3110_READ_FIFO_WOTIME;
    hi3110_spi_trans(spi, HI3110_RX_BUF_LEN);
    memcpy(buf, priv.spi_rx_buf + 1, HI3110_RX_BUF_LEN - 1);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_rx(spi: *mut spi_device) {
    static void hi3110_hw_rx(struct spi_device *spi)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    struct sk_buff *skb;
    struct can_frame *frame;
    u8 buf[HI3110_RX_BUF_LEN - 1];
    skb = alloc_can_skb(priv.net, &frame);
    if (!skb) {
    priv.net.stats.rx_dropped++;
    return;
    }
    hi3110_hw_rx_frame(spi, buf);
    if (buf[HI3110_FIFO_WOTIME_TAG_OFF] & HI3110_FIFO_WOTIME_TAG_IDE) {
// IDE is recessive (1), indicating extended 29-bit frame
    frame.can_id = CAN_EFF_FLAG;
    frame.can_id |=
    (buf[HI3110_FIFO_WOTIME_ID_OFF] << 21) |
    (((buf[HI3110_FIFO_WOTIME_ID_OFF + 1] & 0xE0) >> 5) << 18) |
    ((buf[HI3110_FIFO_WOTIME_ID_OFF + 1] & 0x07) << 15) |
    (buf[HI3110_FIFO_WOTIME_ID_OFF + 2] << 7) |
    (buf[HI3110_FIFO_WOTIME_ID_OFF + 3] >> 1);
    } else {
// IDE is dominant (0), frame indicating standard 11-bit
    frame.can_id =
    (buf[HI3110_FIFO_WOTIME_ID_OFF] << 3) |
    ((buf[HI3110_FIFO_WOTIME_ID_OFF + 1] & 0xE0) >> 5);
    }
// Data length
    frame.len = can_cc_dlc2len(buf[HI3110_FIFO_WOTIME_DLC_OFF] & 0x0F);
    if (buf[HI3110_FIFO_WOTIME_ID_OFF + 3] & HI3110_FIFO_WOTIME_ID_RTR) {
    frame.can_id |= CAN_RTR_FLAG;
    } else {
    memcpy(frame.data, buf + HI3110_FIFO_WOTIME_DAT_OFF,
    frame.len);
    priv.net.stats.rx_bytes += frame.len;
    }
    priv.net.stats.rx_packets++;
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_sleep(spi: *mut spi_device) {
    static void hi3110_hw_sleep(struct spi_device *spi)
    {
    hi3110_write(spi, HI3110_WRITE_CTRL0, HI3110_CTRL0_SLEEP_MODE);
    }
    static netdev_tx_t hi3110_hard_start_xmit(struct sk_buff *skb,
    struct net_device *net)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    if (priv.tx_skb || priv.tx_busy) {
    dev_err(&spi.dev, "hard_xmit called while tx busy\n");
    return NETDEV_TX_BUSY;
    }
    if (can_dev_dropped_skb(net, skb))
    return NETDEV_TX_OK;
    netif_stop_queue(net);
    priv.tx_skb = skb;
    queue_work(priv.wq, &priv.tx_work);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_do_set_mode(net: *mut net_device, mode: enum can_mode) -> c_int {
    static int hi3110_do_set_mode(struct net_device *net, enum can_mode mode)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    switch (mode) {
    case CAN_MODE_START:
    hi3110_clean(net);
// We have to delay work since SPI I/O may sleep
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    priv.restart_tx = 1;
    if (priv.can.restart_ms == 0)
    priv.after_suspend = HI3110_AFTER_SUSPEND_RESTART;
    queue_work(priv.wq, &priv.restart_work);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int hi3110_get_berr_counter(const struct net_device *net,
    struct can_berr_counter *bec)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    mutex_lock(&priv.hi3110_lock);
    bec.txerr = hi3110_read(spi, HI3110_READ_TEC);
    bec.rxerr = hi3110_read(spi, HI3110_READ_REC);
    mutex_unlock(&priv.hi3110_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_set_normal_mode(spi: *mut spi_device) -> c_int {
    static int hi3110_set_normal_mode(struct spi_device *spi)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    let mut reg: u8 = 0;
    hi3110_write(spi, HI3110_WRITE_INTE, HI3110_INT_BUSERR |
    HI3110_INT_RXFIFO | HI3110_INT_TXCPLT);
// Enable TX
    hi3110_write(spi, HI3110_WRITE_CTRL1, HI3110_CTRL1_TXEN);
    if (priv.can.ctrlmode & CAN_CTRLMODE_LOOPBACK)
    reg = HI3110_CTRL0_LOOPBACK_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(CAN_CTRLMODE_LISTENONLY: priv->can.ctrlmode &) -> else {
    else if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    reg = HI3110_CTRL0_MONITOR_MODE;
    else
    reg = HI3110_CTRL0_NORMAL_MODE;
    hi3110_write(spi, HI3110_WRITE_CTRL0, reg);
// Wait for the device to enter the mode
    mdelay(HI3110_OST_DELAY_MS);
    reg = hi3110_read(spi, HI3110_READ_CTRL0);
    if ((reg & HI3110_CTRL0_MODE_MASK) != reg)
    return -EBUSY;
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_do_set_bittiming(net: *mut net_device) -> c_int {
    static int hi3110_do_set_bittiming(struct net_device *net)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    struct can_bittiming *bt = &priv.can.bittiming;
    struct spi_device *spi = priv.spi;
    hi3110_write(spi, HI3110_WRITE_BTR0,
    ((bt.sjw - 1) << HI3110_BTR0_SJW_SHIFT) |
    ((bt.brp - 1) << HI3110_BTR0_BRP_SHIFT));
    hi3110_write(spi, HI3110_WRITE_BTR1,
    (priv.can.ctrlmode &
    CAN_CTRLMODE_3_SAMPLES ?
    HI3110_BTR1_SAMP_3PERBIT : HI3110_BTR1_SAMP_1PERBIT) |
    ((bt.phase_seg1 + bt.prop_seg - 1)
    << HI3110_BTR1_TSEG1_SHIFT) |
    ((bt.phase_seg2 - 1) << HI3110_BTR1_TSEG2_SHIFT));
    dev_dbg(&spi.dev, "BT: 0x%02x 0x%02x\n",
    hi3110_read(spi, HI3110_READ_BTR0),
    hi3110_read(spi, HI3110_READ_BTR1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_setup(net: *mut net_device) -> c_int {
    static int hi3110_setup(struct net_device *net)
    {
    hi3110_do_set_bittiming(net);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_reset(spi: *mut spi_device) -> c_int {
    static int hi3110_hw_reset(struct spi_device *spi)
    {
    u8 reg;
    int ret;
// Wait for oscillator startup timer after power up
    mdelay(HI3110_OST_DELAY_MS);
    ret = hi3110_cmd(spi, HI3110_MASTER_RESET);
    if (ret)
    return ret;
// Wait for oscillator startup timer after reset
    mdelay(HI3110_OST_DELAY_MS);
    reg = hi3110_read(spi, HI3110_READ_CTRL0);
    if ((reg & HI3110_CTRL0_MODE_MASK) != HI3110_CTRL0_INIT_MODE)
    return -ENODEV;
// As per the datasheet it appears the error flags are
// not cleared on reset. Explicitly clear them by performing a read
//
    hi3110_read(spi, HI3110_READ_ERR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_hw_probe(spi: *mut spi_device) -> c_int {
    static int hi3110_hw_probe(struct spi_device *spi)
    {
    u8 statf;
    hi3110_hw_reset(spi);
// Confirm correct operation by checking against reset values
// in datasheet
//
    statf = hi3110_read(spi, HI3110_READ_STATF);
    dev_dbg(&spi.dev, "statf: %02X\n", statf);
    if (statf != 0x82)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_power_enable(reg: *mut regulator, enable: c_int) -> c_int {
    static int hi3110_power_enable(struct regulator *reg, int enable)
    {
    if (IS_ERR_OR_NULL(reg))
    return 0;
    if (enable)
    return regulator_enable(reg);
    else
    return regulator_disable(reg);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_stop(net: *mut net_device) -> c_int {
    static int hi3110_stop(struct net_device *net)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    close_candev(net);
    priv.force_quit = 1;
    free_irq(spi.irq, priv);
    mutex_lock(&priv.hi3110_lock);
// Disable transmit, interrupts and clear flags
    hi3110_write(spi, HI3110_WRITE_CTRL1, 0x0);
    hi3110_write(spi, HI3110_WRITE_INTE, 0x0);
    hi3110_read(spi, HI3110_READ_INTF);
    hi3110_clean(net);
    hi3110_hw_sleep(spi);
    hi3110_power_enable(priv.transceiver, 0);
    priv.can.state = CAN_STATE_STOPPED;
    mutex_unlock(&priv.hi3110_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_tx_work_handler(ws: *mut work_struct) {
    static void hi3110_tx_work_handler(struct work_struct *ws)
    {
    struct hi3110_priv *priv = container_of(ws, struct hi3110_priv,
    tx_work);
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    struct can_frame *frame;
    mutex_lock(&priv.hi3110_lock);
    if (priv.tx_skb) {
    if (priv.can.state == CAN_STATE_BUS_OFF) {
    hi3110_clean(net);
    } else {
    frame = (struct can_frame *)priv.tx_skb.data;
    hi3110_hw_tx(spi, frame);
    priv.tx_busy = true;
    can_put_echo_skb(priv.tx_skb, net, 0, 0);
    priv.tx_skb = core::ptr::null_mut();
    }
    }
    mutex_unlock(&priv.hi3110_lock);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_restart_work_handler(ws: *mut work_struct) {
    static void hi3110_restart_work_handler(struct work_struct *ws)
    {
    struct hi3110_priv *priv = container_of(ws, struct hi3110_priv,
    restart_work);
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    mutex_lock(&priv.hi3110_lock);
    if (priv.after_suspend) {
    hi3110_hw_reset(spi);
    hi3110_setup(net);
    if (priv.after_suspend & HI3110_AFTER_SUSPEND_RESTART) {
    hi3110_set_normal_mode(spi);
    } else if (priv.after_suspend & HI3110_AFTER_SUSPEND_UP) {
    netif_device_attach(net);
    hi3110_clean(net);
    hi3110_set_normal_mode(spi);
    netif_wake_queue(net);
    } else {
    hi3110_hw_sleep(spi);
    }
    priv.after_suspend = 0;
    priv.force_quit = 0;
    }
    if (priv.restart_tx) {
    priv.restart_tx = 0;
    hi3110_hw_reset(spi);
    hi3110_setup(net);
    hi3110_clean(net);
    hi3110_set_normal_mode(spi);
    netif_wake_queue(net);
    }
    mutex_unlock(&priv.hi3110_lock);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_can_ist(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi3110_can_ist(int irq, void *dev_id)
    {
    struct hi3110_priv *priv = dev_id;
    struct spi_device *spi = priv.spi;
    struct net_device *net = priv.net;
    mutex_lock(&priv.hi3110_lock);
    while (!priv.force_quit) {
    enum can_state new_state;
    u8 intf, eflag, statf;
    while (!(HI3110_STAT_RXFMTY &
    (statf = hi3110_read(spi, HI3110_READ_STATF)))) {
    hi3110_hw_rx(spi);
    }
    intf = hi3110_read(spi, HI3110_READ_INTF);
    eflag = hi3110_read(spi, HI3110_READ_ERR);
// Update can state
    if (eflag & HI3110_ERR_BUSOFF)
    new_state = CAN_STATE_BUS_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(HI3110_ERR_PASSIVE_MASK: eflag &) -> else {
    else if (eflag & HI3110_ERR_PASSIVE_MASK)
    new_state = CAN_STATE_ERROR_PASSIVE;
#[no_mangle]
pub unsafe extern "C" fn if(HI3110_STAT_ERRW: statf &) -> else {
    else if (statf & HI3110_STAT_ERRW)
    new_state = CAN_STATE_ERROR_WARNING;
    else
    new_state = CAN_STATE_ERROR_ACTIVE;
    if (new_state != priv.can.state) {
    struct can_frame *cf;
    struct sk_buff *skb;
    enum can_state rx_state, tx_state;
    u8 rxerr, txerr;
    skb = alloc_can_err_skb(net, &cf);
    txerr = hi3110_read(spi, HI3110_READ_TEC);
    rxerr = hi3110_read(spi, HI3110_READ_REC);
    tx_state = txerr >= rxerr ? new_state : 0;
    rx_state = txerr <= rxerr ? new_state : 0;
    can_change_state(net, cf, tx_state, rx_state);
    if (new_state == CAN_STATE_BUS_OFF) {
    if (skb)
    netif_rx(skb);
    can_bus_off(net);
    if (priv.can.restart_ms == 0) {
    priv.force_quit = 1;
    hi3110_hw_sleep(spi);
    break;
    }
    } else if (skb) {
    cf.can_id |= CAN_ERR_CNT;
    cf.data[6] = txerr;
    cf.data[7] = rxerr;
    netif_rx(skb);
    }
    }
// Update bus errors
    if ((intf & HI3110_INT_BUSERR) &&
    (priv.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING)) {
    struct can_frame *cf;
    struct sk_buff *skb;
// Check for protocol errors
    if (eflag & HI3110_ERR_PROTOCOL_MASK) {
    skb = alloc_can_err_skb(net, &cf);
    if (skb)
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR;
    priv.can.can_stats.bus_error++;
    if (eflag & HI3110_ERR_BITERR) {
    priv.net.stats.tx_errors++;
    if (skb)
    cf.data[2] |= CAN_ERR_PROT_BIT;
    } else if (eflag & HI3110_ERR_FRMERR) {
    priv.net.stats.rx_errors++;
    if (skb)
    cf.data[2] |= CAN_ERR_PROT_FORM;
    } else if (eflag & HI3110_ERR_STUFERR) {
    priv.net.stats.rx_errors++;
    if (skb)
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    } else if (eflag & HI3110_ERR_CRCERR) {
    priv.net.stats.rx_errors++;
    if (skb)
    cf.data[3] |= CAN_ERR_PROT_LOC_CRC_SEQ;
    } else if (eflag & HI3110_ERR_ACKERR) {
    priv.net.stats.tx_errors++;
    if (skb)
    cf.data[3] |= CAN_ERR_PROT_LOC_ACK;
    }
    netdev_dbg(priv.net, "Bus Error\n");
    if (skb) {
    cf.data[6] = hi3110_read(spi, HI3110_READ_TEC);
    cf.data[7] = hi3110_read(spi, HI3110_READ_REC);
    netif_rx(skb);
    }
    }
    }
    if (priv.tx_busy && statf & HI3110_STAT_TXMTY) {
    net.stats.tx_packets++;
    net.stats.tx_bytes += can_get_echo_skb(net, 0, core::ptr::null_mut());
    priv.tx_busy = false;
    netif_wake_queue(net);
    }
    if (intf == 0)
    break;
    }
    mutex_unlock(&priv.hi3110_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_open(net: *mut net_device) -> c_int {
    static int hi3110_open(struct net_device *net)
    {
    struct hi3110_priv *priv = netdev_priv(net);
    struct spi_device *spi = priv.spi;
    let mut flags: c_ulong = IRQF_ONESHOT | IRQF_TRIGGER_HIGH;
    int ret;
    ret = open_candev(net);
    if (ret)
    return ret;
    mutex_lock(&priv.hi3110_lock);
    ret = hi3110_power_enable(priv.transceiver, 1);
    if (ret)
    goto out_close_candev;
    priv.force_quit = 0;
    priv.tx_skb = core::ptr::null_mut();
    priv.tx_busy = false;
    ret = request_threaded_irq(spi.irq, core::ptr::null_mut(), hi3110_can_ist,
    flags, DEVICE_NAME, priv);
    if (ret) {
    dev_err(&spi.dev, "failed to acquire irq %d\n", spi.irq);
    goto out_close;
    }
    ret = hi3110_hw_reset(spi);
    if (ret)
    goto out_free_irq;
    ret = hi3110_setup(net);
    if (ret)
    goto out_free_irq;
    ret = hi3110_set_normal_mode(spi);
    if (ret)
    goto out_free_irq;
    netif_wake_queue(net);
    mutex_unlock(&priv.hi3110_lock);
    return 0;
    out_free_irq:
    free_irq(spi.irq, priv);
    hi3110_hw_sleep(spi);
    out_close:
    hi3110_power_enable(priv.transceiver, 0);
    out_close_candev:
    close_candev(net);
    mutex_unlock(&priv.hi3110_lock);
    return ret;
    }
    static const struct net_device_ops hi3110_netdev_ops = {
    .ndo_open = hi3110_open,
    .ndo_stop = hi3110_stop,
    .ndo_start_xmit = hi3110_hard_start_xmit,
    };
    static const struct ethtool_ops hi3110_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static const struct of_device_id hi3110_of_match[] = {
    {
    .compatible	= "holt,hi3110",
    .data		= (void *)CAN_HI3110_HI3110,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, hi3110_of_match);
    static const struct spi_device_id hi3110_id_table[] = {
    {
    .name		= "hi3110",
    .driver_data	= (kernel_ulong_t)CAN_HI3110_HI3110,
    },
    { }
    };
    MODULE_DEVICE_TABLE(spi, hi3110_id_table);
#[no_mangle]
unsafe extern "C" fn hi3110_can_probe(spi: *mut spi_device) -> c_int {
    static int hi3110_can_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct net_device *net;
    struct hi3110_priv *priv;
    struct clk *clk;
    u32 freq;
    int ret;
    clk = devm_clk_get_optional(&spi.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "no CAN clock source defined\n");
    if (clk) {
    freq = clk_get_rate(clk);
    } else {
    ret = device_property_read_u32(dev, "clock-frequency", &freq);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get clock-frequency!\n");
    }
// Sanity check
    if (freq > 40000000)
    return -ERANGE;
// Allocate can/net device
    net = alloc_candev(sizeof(struct hi3110_priv), HI3110_TX_ECHO_SKB_MAX);
    if (!net)
    return -ENOMEM;
    ret = clk_prepare_enable(clk);
    if (ret)
    goto out_free;
    net.netdev_ops = &hi3110_netdev_ops;
    net.ethtool_ops = &hi3110_ethtool_ops;
    net.flags |= IFF_ECHO;
    priv = netdev_priv(net);
    priv.can.bittiming_const = &hi3110_bittiming_const;
    priv.can.do_set_mode = hi3110_do_set_mode;
    priv.can.do_get_berr_counter = hi3110_get_berr_counter;
    priv.can.clock.freq = freq / 2;
    priv.can.ctrlmode_supported = CAN_CTRLMODE_3_SAMPLES |
    CAN_CTRLMODE_LOOPBACK |
    CAN_CTRLMODE_LISTENONLY |
    CAN_CTRLMODE_BERR_REPORTING;
    priv.model = (enum hi3110_model)(uintptr_t)spi_get_device_match_data(spi);
    priv.net = net;
    priv.clk = clk;
    spi_set_drvdata(spi, priv);
// Configure the SPI bus
    spi.bits_per_word = 8;
    ret = spi_setup(spi);
    if (ret)
    goto out_clk;
    priv.power = devm_regulator_get_optional(&spi.dev, "vdd");
    priv.transceiver = devm_regulator_get_optional(&spi.dev, "xceiver");
    if ((PTR_ERR(priv.power) == -EPROBE_DEFER) ||
    (PTR_ERR(priv.transceiver) == -EPROBE_DEFER)) {
    ret = -EPROBE_DEFER;
    goto out_clk;
    }
    ret = hi3110_power_enable(priv.power, 1);
    if (ret)
    goto out_clk;
    priv.wq = alloc_workqueue("hi3110_wq",
    WQ_FREEZABLE | WQ_MEM_RECLAIM | WQ_PERCPU,
    0);
    if (!priv.wq) {
    ret = -ENOMEM;
    goto out_clk;
    }
    INIT_WORK(&priv.tx_work, hi3110_tx_work_handler);
    INIT_WORK(&priv.restart_work, hi3110_restart_work_handler);
    priv.spi = spi;
    mutex_init(&priv.hi3110_lock);
    priv.spi_tx_buf = devm_kzalloc(&spi.dev, HI3110_RX_BUF_LEN,
    GFP_KERNEL);
    if (!priv.spi_tx_buf) {
    ret = -ENOMEM;
    goto error_probe;
    }
    priv.spi_rx_buf = devm_kzalloc(&spi.dev, HI3110_RX_BUF_LEN,
    GFP_KERNEL);
    if (!priv.spi_rx_buf) {
    ret = -ENOMEM;
    goto error_probe;
    }
    SET_NETDEV_DEV(net, &spi.dev);
    ret = hi3110_hw_probe(spi);
    if (ret) {
    dev_err_probe(dev, ret, "Cannot initialize %x. Wrong wiring?\n", priv.model);
    goto error_probe;
    }
    hi3110_hw_sleep(spi);
    ret = register_candev(net);
    if (ret)
    goto error_probe;
    netdev_info(net, "%x successfully initialized.\n", priv.model);
    return 0;
    error_probe:
    destroy_workqueue(priv.wq);
    priv.wq = core::ptr::null_mut();
    hi3110_power_enable(priv.power, 0);
    out_clk:
    clk_disable_unprepare(clk);
    out_free:
    free_candev(net);
    return dev_err_probe(dev, ret, "Probe failed\n");
    }
#[no_mangle]
unsafe extern "C" fn hi3110_can_remove(spi: *mut spi_device) {
    static void hi3110_can_remove(struct spi_device *spi)
    {
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    struct net_device *net = priv.net;
    unregister_candev(net);
    hi3110_power_enable(priv.power, 0);
    destroy_workqueue(priv.wq);
    priv.wq = core::ptr::null_mut();
    clk_disable_unprepare(priv.clk);
    free_candev(net);
    }
#[no_mangle]
unsafe extern "C" fn hi3110_can_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused hi3110_can_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    struct net_device *net = priv.net;
    priv.force_quit = 1;
    disable_irq(spi.irq);
// Note: at this point neither IST nor workqueues are running.
// open/stop cannot be called anyway so locking is not needed
//
    if (netif_running(net)) {
    netif_device_detach(net);
    hi3110_hw_sleep(spi);
    hi3110_power_enable(priv.transceiver, 0);
    priv.after_suspend = HI3110_AFTER_SUSPEND_UP;
    } else {
    priv.after_suspend = HI3110_AFTER_SUSPEND_DOWN;
    }
    if (!IS_ERR_OR_NULL(priv.power)) {
    regulator_disable(priv.power);
    priv.after_suspend |= HI3110_AFTER_SUSPEND_POWER;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3110_can_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused hi3110_can_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct hi3110_priv *priv = spi_get_drvdata(spi);
    if (priv.after_suspend & HI3110_AFTER_SUSPEND_POWER)
    hi3110_power_enable(priv.power, 1);
    if (priv.after_suspend & HI3110_AFTER_SUSPEND_UP) {
    hi3110_power_enable(priv.transceiver, 1);
    queue_work(priv.wq, &priv.restart_work);
    } else {
    priv.after_suspend = 0;
    }
    priv.force_quit = 0;
    enable_irq(spi.irq);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(hi3110_can_pm_ops, hi3110_can_suspend, hi3110_can_resume);
    static struct spi_driver hi3110_can_driver = {
    .driver = {
    .name = DEVICE_NAME,
    .of_match_table = hi3110_of_match,
    .pm = &hi3110_can_pm_ops,
    },
    .id_table = hi3110_id_table,
    .probe = hi3110_can_probe,
    .remove = hi3110_can_remove,
    };
    module_spi_driver(hi3110_can_driver);
    MODULE_AUTHOR("Akshay Bhat <akshay.bhat@timesys.com>");
    MODULE_AUTHOR("Casey Fitzpatrick <casey.fitzpatrick@timesys.com>");
    MODULE_DESCRIPTION("Holt HI-3110 CAN driver");
    MODULE_LICENSE("GPL v2");
