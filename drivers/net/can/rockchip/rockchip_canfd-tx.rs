//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/rockchip/rockchip_canfd-tx.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2023, 2024 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

#[no_mangle]
unsafe extern "C" fn rkcanfd_tx_tail_is_eff(priv: *const rkcanfd_priv) -> bool {
    static bool rkcanfd_tx_tail_is_eff(const struct rkcanfd_priv *priv)
    {
    const struct canfd_frame *cfd;
    const struct sk_buff *skb;
    unsigned int tx_tail;
    if (!rkcanfd_get_tx_pending(priv))
    return false;
    tx_tail = rkcanfd_get_tx_tail(priv);
    skb = priv.can.echo_skb[tx_tail];
    if (!skb) {
    netdev_err(priv.ndev,
    "%s: echo_skb[%u]=core::ptr::null_mut() tx_head=0x%08x tx_tail=0x%08x\n",
    __func__, tx_tail,
    priv.tx_head, priv.tx_tail);
    return false;
    }
    cfd = (struct canfd_frame *)skb.data;
    return cfd.can_id & CAN_EFF_FLAG;
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_get_effective_tx_free(priv: *const rkcanfd_priv) -> c_uint {
    unsigned int rkcanfd_get_effective_tx_free(const struct rkcanfd_priv *priv)
    {
    if (priv.devtype_data.quirks & RKCANFD_QUIRK_RK3568_ERRATUM_6 &&
    rkcanfd_tx_tail_is_eff(priv))
    return 0;
    return rkcanfd_get_tx_free(priv);
    }
    static void rkcanfd_start_xmit_write_cmd(const struct rkcanfd_priv *priv,
    const u32 reg_cmd)
    {
    if (priv.devtype_data.quirks & RKCANFD_QUIRK_RK3568_ERRATUM_12)
    rkcanfd_write(priv, RKCANFD_REG_MODE, priv.reg_mode_default |
    RKCANFD_REG_MODE_SPACE_RX_MODE);
    rkcanfd_write(priv, RKCANFD_REG_CMD, reg_cmd);
    if (priv.devtype_data.quirks & RKCANFD_QUIRK_RK3568_ERRATUM_12)
    rkcanfd_write(priv, RKCANFD_REG_MODE, priv.reg_mode_default);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_xmit_retry(priv: *mut rkcanfd_priv) {
    void rkcanfd_xmit_retry(struct rkcanfd_priv *priv)
    {
    let mut tx_head: c_uint = rkcanfd_get_tx_head(priv);
    let mut reg_cmd: u32 = RKCANFD_REG_CMD_TX_REQ(tx_head);
    rkcanfd_start_xmit_write_cmd(priv, reg_cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn rkcanfd_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    netdev_tx_t rkcanfd_start_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    struct rkcanfd_priv *priv = netdev_priv(ndev);
    u32 reg_frameinfo, reg_id, reg_cmd;
    unsigned int tx_head, frame_len;
    const struct canfd_frame *cfd;
    int err;
    u8 i;
    if (can_dev_dropped_skb(ndev, skb))
    return NETDEV_TX_OK;
    if (!netif_subqueue_maybe_stop(priv.ndev, 0,
    rkcanfd_get_effective_tx_free(priv),
    RKCANFD_TX_STOP_THRESHOLD,
    RKCANFD_TX_START_THRESHOLD)) {
    if (net_ratelimit())
    netdev_info(priv.ndev,
    "Stopping tx-queue (tx_head=0x%08x, tx_tail=0x%08x, tx_pending=%d)\n",
    priv.tx_head, priv.tx_tail,
    rkcanfd_get_tx_pending(priv));
    return NETDEV_TX_BUSY;
    }
    cfd = (struct canfd_frame *)skb.data;
    if (cfd.can_id & CAN_EFF_FLAG) {
    reg_frameinfo = RKCANFD_REG_FD_FRAMEINFO_FRAME_FORMAT;
    reg_id = FIELD_PREP(RKCANFD_REG_FD_ID_EFF, cfd.can_id);
    } else {
    reg_frameinfo = 0;
    reg_id = FIELD_PREP(RKCANFD_REG_FD_ID_SFF, cfd.can_id);
    }
    if (cfd.can_id & CAN_RTR_FLAG)
    reg_frameinfo |= RKCANFD_REG_FD_FRAMEINFO_RTR;
    if (can_is_canfd_skb(skb)) {
    reg_frameinfo |= RKCANFD_REG_FD_FRAMEINFO_FDF;
    if (cfd.flags & CANFD_BRS)
    reg_frameinfo |= RKCANFD_REG_FD_FRAMEINFO_BRS;
    reg_frameinfo |= FIELD_PREP(RKCANFD_REG_FD_FRAMEINFO_DATA_LENGTH,
    can_fd_len2dlc(cfd.len));
    } else {
    reg_frameinfo |= FIELD_PREP(RKCANFD_REG_FD_FRAMEINFO_DATA_LENGTH,
    cfd.len);
    }
    tx_head = rkcanfd_get_tx_head(priv);
    reg_cmd = RKCANFD_REG_CMD_TX_REQ(tx_head);
    rkcanfd_write(priv, RKCANFD_REG_FD_TXFRAMEINFO, reg_frameinfo);
    rkcanfd_write(priv, RKCANFD_REG_FD_TXID, reg_id);
    for (i = 0; i < cfd.len; i += 4)
    rkcanfd_write(priv, RKCANFD_REG_FD_TXDATA0 + i,
// (u32 *)(cfd->data + i));
    frame_len = can_skb_get_frame_len(skb);
    err = can_put_echo_skb(skb, ndev, tx_head, frame_len);
    if (!err)
    netdev_sent_queue(priv.ndev, frame_len);
    WRITE_ONCE(priv.tx_head, priv.tx_head + 1);
    rkcanfd_start_xmit_write_cmd(priv, reg_cmd);
    netif_subqueue_maybe_stop(priv.ndev, 0,
    rkcanfd_get_effective_tx_free(priv),
    RKCANFD_TX_STOP_THRESHOLD,
    RKCANFD_TX_START_THRESHOLD);
    return NETDEV_TX_OK;
    }
    void rkcanfd_handle_tx_done_one(struct rkcanfd_priv *priv, const u32 ts,
    unsigned int *frame_len_p)
    {
    struct net_device_stats *stats = &priv.ndev.stats;
    unsigned int tx_tail;
    struct sk_buff *skb;
    tx_tail = rkcanfd_get_tx_tail(priv);
    skb = priv.can.echo_skb[tx_tail];
// Manual handling of CAN Bus Error counters. See
// rkcanfd_get_corrected_berr_counter() for detailed
// explanation.
//
    if (priv.bec.txerr)
    priv.bec.txerr--;
    if (skb)
    rkcanfd_skb_set_timestamp(priv, skb, ts);
    stats.tx_bytes +=
    can_rx_offload_get_echo_skb_queue_timestamp(&priv.offload,
    tx_tail, ts,
    frame_len_p);
    stats.tx_packets++;
    }
