//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_packet.c
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2021 Microchip Technology Inc. and its subsidiaries.
//

pub const INJ_TIMEOUT_NS: c_int = 50000;
#[no_mangle]
pub unsafe extern "C" fn sparx5_xtr_flush(sparx5: *mut sparx5, grp: u8) {
    void sparx5_xtr_flush(struct sparx5 *sparx5, u8 grp)
    {
// Start flush
    spx5_wr(QS_XTR_FLUSH_FLUSH_SET(BIT(grp)), sparx5, QS_XTR_FLUSH);
// Allow to drain
    mdelay(1);
// All Queues normal
    spx5_wr(0, sparx5, QS_XTR_FLUSH);
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_ifh_parse(sparx5: *mut sparx5, ifh: *mut u32, info: *mut frame_info) {
    void sparx5_ifh_parse(struct sparx5 *sparx5, u32 *ifh, struct frame_info *info)
    {
    u8 *xtr_hdr = (u8 *)ifh;
// FWD is bit 45-72 (28 bits), but we only read the 27 LSB for now
    u32 fwd =
    ((u32)xtr_hdr[27] << 24) |
    ((u32)xtr_hdr[28] << 16) |
    ((u32)xtr_hdr[29] <<  8) |
    ((u32)xtr_hdr[30] <<  0);
    fwd = (fwd >> 5);
    info.src_port = spx5_field_get(GENMASK(is_sparx5(sparx5) ? 7 : 6, 1),
    fwd);
//
// Bit 270-271 are occasionally unexpectedly set by the hardware,
// clear bits before extracting timestamp
//
    info.timestamp =
    ((u64)(xtr_hdr[2] & GENMASK(5, 0)) << 24) |
    ((u64)xtr_hdr[3] << 16) |
    ((u64)xtr_hdr[4] <<  8) |
    ((u64)xtr_hdr[5] <<  0);
    }
#[no_mangle]
unsafe extern "C" fn sparx5_xtr_grp(sparx5: *mut sparx5, grp: u8, byte_swap: bool) {
    static void sparx5_xtr_grp(struct sparx5 *sparx5, u8 grp, bool byte_swap)
    {
    let mut eof_flag: bool = false, pruned_flag = false, abort_flag = false;
    struct net_device *netdev;
    struct sparx5_port *port;
    struct frame_info fi;
    int i, byte_cnt = 0;
    struct sk_buff *skb;
    u32 ifh[IFH_LEN];
    u32 *rxbuf;
// Get IFH
    for (i = 0; i < IFH_LEN; i++)
    ifh[i] = spx5_rd(sparx5, QS_XTR_RD(grp));
// Decode IFH (what's needed)
    sparx5_ifh_parse(sparx5, ifh, &fi);
// Map to port netdev
    port = fi.src_port < sparx5.data.consts.n_ports ?
    sparx5.ports[fi.src_port] : core::ptr::null_mut();
    if (!port || !port.ndev) {
    dev_err(sparx5.dev, "Data on inactive port %d\n", fi.src_port);
    sparx5_xtr_flush(sparx5, grp);
    return;
    }
// Have netdev, get skb
    netdev = port.ndev;
    skb = netdev_alloc_skb(netdev, netdev.mtu + ETH_HLEN);
    if (!skb) {
    sparx5_xtr_flush(sparx5, grp);
    dev_err(sparx5.dev, "No skb allocated\n");
    netdev.stats.rx_dropped++;
    return;
    }
    rxbuf = (u32 *)skb.data;
// Now, pull frame data
    while (!eof_flag) {
    let mut val: u32 = spx5_rd(sparx5, QS_XTR_RD(grp));
    let mut cmp: u32 = val;
    if (byte_swap)
    cmp = ntohl(( __be32)val);
    switch (cmp) {
    case XTR_NOT_READY:
    break;
    case XTR_ABORT:
// No accompanying data
    abort_flag = true;
    eof_flag = true;
    break;
    case XTR_EOF_0:
    case XTR_EOF_1:
    case XTR_EOF_2:
    case XTR_EOF_3:
// This assumes STATUS_WORD_POS == 1, Status
// just after last data
//
    if (!byte_swap)
    val = ntohl(( __be32)val);
    byte_cnt -= (4 - XTR_VALID_BYTES(val));
    eof_flag = true;
    break;
    case XTR_PRUNED:
// But get the last 4 bytes as well
    eof_flag = true;
    pruned_flag = true;
    fallthrough;
    case XTR_ESCAPE:
// rxbuf = spx5_rd(sparx5, QS_XTR_RD(grp));
    byte_cnt += 4;
    rxbuf++;
    break;
    default:
// rxbuf = val;
    byte_cnt += 4;
    rxbuf++;
    }
    }
    if (abort_flag || pruned_flag || !eof_flag) {
    netdev_err(netdev, "Discarded frame: abort:%d pruned:%d eof:%d\n",
    abort_flag, pruned_flag, eof_flag);
    kfree_skb(skb);
    netdev.stats.rx_dropped++;
    return;
    }
// Everything we see on an interface that is in the HW bridge
// has already been forwarded
//
    if (test_bit(port.portno, sparx5.bridge_mask))
    skb.offload_fwd_mark = 1;
// Finish up skb
    skb_put(skb, byte_cnt - ETH_FCS_LEN);
    eth_skb_pad(skb);
    sparx5_ptp_rxtstamp(sparx5, skb, fi.timestamp);
    skb.protocol = eth_type_trans(skb, netdev);
    netdev.stats.rx_bytes += skb.len;
    netdev.stats.rx_packets++;
    netif_rx(skb);
    }
    static int sparx5_inject(struct sparx5 *sparx5,
    u32 *ifh,
    struct sk_buff *skb,
    struct net_device *ndev)
    {
    let mut grp: c_int = INJ_QUEUE;
    u32 val, w, count;
    u8 *buf;
    val = spx5_rd(sparx5, QS_INJ_STATUS);
    if (!(QS_INJ_STATUS_FIFO_RDY_GET(val) & BIT(grp))) {
    pr_err_ratelimited("Injection: Queue not ready: 0x%lx\n",
    QS_INJ_STATUS_FIFO_RDY_GET(val));
    return -EBUSY;
    }
// Indicate SOF
    spx5_wr(QS_INJ_CTRL_SOF_SET(1) |
    QS_INJ_CTRL_GAP_SIZE_SET(1),
    sparx5, QS_INJ_CTRL(grp));
// Write the IFH to the chip.
    for (w = 0; w < IFH_LEN; w++)
    spx5_wr(ifh[w], sparx5, QS_INJ_WR(grp));
// Write words, round up
    count = DIV_ROUND_UP(skb.len, 4);
    buf = skb.data;
    for (w = 0; w < count; w++, buf += 4) {
    val = get_unaligned((const u32 *)buf);
    spx5_wr(val, sparx5, QS_INJ_WR(grp));
    }
// Add padding
    while (w < (60 / 4)) {
    spx5_wr(0, sparx5, QS_INJ_WR(grp));
    w++;
    }
// Indicate EOF and valid bytes in last word
    spx5_wr(QS_INJ_CTRL_GAP_SIZE_SET(1) |
    QS_INJ_CTRL_VLD_BYTES_SET(skb.len < 60 ? 0 : skb.len % 4) |
    QS_INJ_CTRL_EOF_SET(1),
    sparx5, QS_INJ_CTRL(grp));
// Add dummy CRC
    spx5_wr(0, sparx5, QS_INJ_WR(grp));
    w++;
    val = spx5_rd(sparx5, QS_INJ_STATUS);
    if (QS_INJ_STATUS_WMARK_REACHED_GET(val) & BIT(grp)) {
    struct sparx5_port *port = netdev_priv(ndev);
    pr_err_ratelimited("Injection: Watermark reached: 0x%lx\n",
    QS_INJ_STATUS_WMARK_REACHED_GET(val));
    netif_stop_queue(ndev);
    hrtimer_start(&port.inj_timer, INJ_TIMEOUT_NS,
    HRTIMER_MODE_REL);
    }
    return NETDEV_TX_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_port_xmit_impl(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    netdev_tx_t sparx5_port_xmit_impl(struct sk_buff *skb, struct net_device *dev)
    {
    struct net_device_stats *stats = &dev.stats;
    struct sparx5_port *port = netdev_priv(dev);
    struct sparx5 *sparx5 = port.sparx5;
    const struct sparx5_ops *ops;
    u32 ifh[IFH_LEN];
    netdev_tx_t ret;
    ops = sparx5.data.ops;
    memset(ifh, 0, IFH_LEN * 4);
    sparx5_set_port_ifh(sparx5, ifh, port.portno);
    if (sparx5.ptp && skb_shinfo(skb).tx_flags & SKBTX_HW_TSTAMP) {
    if (sparx5_ptp_txtstamp_request(port, skb) < 0)
    return NETDEV_TX_BUSY;
    sparx5_set_port_ifh_rew_op(ifh, SPARX5_SKB_CB(skb).rew_op);
    sparx5_set_port_ifh_pdu_type(sparx5, ifh,
    SPARX5_SKB_CB(skb).pdu_type);
    sparx5_set_port_ifh_pdu_w16_offset(sparx5, ifh,
    SPARX5_SKB_CB(skb).pdu_w16_offset);
    sparx5_set_port_ifh_timestamp(sparx5, ifh,
    SPARX5_SKB_CB(skb).ts_id);
    }
    skb_tx_timestamp(skb);
    spin_lock(&sparx5.tx_lock);
    if (sparx5.fdma_irq > 0)
    ret = ops.fdma_xmit(sparx5, ifh, skb, dev);
    else
    ret = sparx5_inject(sparx5, ifh, skb, dev);
    spin_unlock(&sparx5.tx_lock);
    if (ret == -EBUSY)
    goto busy;
    if (ret < 0)
    goto drop;
    if (!is_sparx5(sparx5))
// When lan969x and TX_OK, stats and SKB consumption is handled
// in the TX completion loop, so dont go any further.
//
    return NETDEV_TX_OK;
    stats.tx_bytes += skb.len;
    stats.tx_packets++;
    sparx5.tx.packets++;
    if (skb_shinfo(skb).tx_flags & SKBTX_HW_TSTAMP &&
    SPARX5_SKB_CB(skb).rew_op == IFH_REW_OP_TWO_STEP_PTP)
    return NETDEV_TX_OK;
    dev_consume_skb_any(skb);
    return NETDEV_TX_OK;
    drop:
    stats.tx_dropped++;
    sparx5.tx.dropped++;
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    busy:
    if (skb_shinfo(skb).tx_flags & SKBTX_HW_TSTAMP &&
    SPARX5_SKB_CB(skb).rew_op == IFH_REW_OP_TWO_STEP_PTP)
    sparx5_ptp_txtstamp_release(port, skb);
    return NETDEV_TX_BUSY;
    }
#[no_mangle]
unsafe extern "C" fn sparx5_injection_timeout(tmr: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart sparx5_injection_timeout(struct hrtimer *tmr)
    {
    struct sparx5_port *port = container_of(tmr, struct sparx5_port,
    inj_timer);
    let mut grp: c_int = INJ_QUEUE;
    u32 val;
    val = spx5_rd(port.sparx5, QS_INJ_STATUS);
    if (QS_INJ_STATUS_WMARK_REACHED_GET(val) & BIT(grp)) {
    pr_err_ratelimited("Injection: Reset watermark count\n");
// Reset Watermark count to restart
    spx5_rmw(DSM_DEV_TX_STOP_WM_CFG_DEV_TX_CNT_CLR_SET(1),
    DSM_DEV_TX_STOP_WM_CFG_DEV_TX_CNT_CLR,
    port.sparx5,
    DSM_DEV_TX_STOP_WM_CFG(port.portno));
    }
    netif_wake_queue(port.ndev);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_manual_injection_mode(sparx5: *mut sparx5) -> c_int {
    int sparx5_manual_injection_mode(struct sparx5 *sparx5)
    {
    let mut byte_swap: c_int = 1;
    int portno;
// Change mode to manual extraction and injection
    spx5_wr(QS_XTR_GRP_CFG_MODE_SET(1) |
    QS_XTR_GRP_CFG_STATUS_WORD_POS_SET(1) |
    QS_XTR_GRP_CFG_BYTE_SWAP_SET(byte_swap),
    sparx5, QS_XTR_GRP_CFG(XTR_QUEUE));
    spx5_wr(QS_INJ_GRP_CFG_MODE_SET(1) |
    QS_INJ_GRP_CFG_BYTE_SWAP_SET(byte_swap),
    sparx5, QS_INJ_GRP_CFG(INJ_QUEUE));
// CPU ports capture setup
    for (portno = sparx5_get_internal_port(sparx5, SPX5_PORT_CPU_0);
    portno <= sparx5_get_internal_port(sparx5, SPX5_PORT_CPU_1);
    portno++) {
// ASM CPU port: No preamble, IFH, enable padding
    spx5_wr(ASM_PORT_CFG_PAD_ENA_SET(1) |
    ASM_PORT_CFG_NO_PREAMBLE_ENA_SET(1) |
    ASM_PORT_CFG_INJ_FORMAT_CFG_SET(1), /* 1 = IFH */
    sparx5, ASM_PORT_CFG(portno));
// Reset WM cnt to unclog queued frames
    spx5_rmw(DSM_DEV_TX_STOP_WM_CFG_DEV_TX_CNT_CLR_SET(1),
    DSM_DEV_TX_STOP_WM_CFG_DEV_TX_CNT_CLR,
    sparx5,
    DSM_DEV_TX_STOP_WM_CFG(portno));
// Set Disassembler Stop Watermark level
    spx5_rmw(DSM_DEV_TX_STOP_WM_CFG_DEV_TX_STOP_WM_SET(0),
    DSM_DEV_TX_STOP_WM_CFG_DEV_TX_STOP_WM,
    sparx5,
    DSM_DEV_TX_STOP_WM_CFG(portno));
// Enable Disassembler buffer underrun watchdog
//
    spx5_rmw(DSM_BUF_CFG_UNDERFLOW_WATCHDOG_DIS_SET(0),
    DSM_BUF_CFG_UNDERFLOW_WATCHDOG_DIS,
    sparx5,
    DSM_BUF_CFG(portno));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_xtr_handler(irq: c_int, _sparx5: *mut c_void) -> irqreturn_t {
    irqreturn_t sparx5_xtr_handler(int irq, void *_sparx5)
    {
    struct sparx5 *s5 = _sparx5;
    let mut poll: c_int = 64;
// Check data in queue
    while (spx5_rd(s5, QS_XTR_DATA_PRESENT) & BIT(XTR_QUEUE) && poll-- > 0)
    sparx5_xtr_grp(s5, XTR_QUEUE, false);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_port_inj_timer_setup(port: *mut sparx5_port) {
    void sparx5_port_inj_timer_setup(struct sparx5_port *port)
    {
    hrtimer_setup(&port.inj_timer, sparx5_injection_timeout, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL);
    }
