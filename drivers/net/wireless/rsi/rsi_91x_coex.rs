//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/rsi/rsi_91x_coex.c
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


//
// Copyright (c) 2018 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

    static enum rsi_coex_queues rsi_coex_determine_coex_q
    (struct rsi_coex_ctrl_block *coex_cb)
    {
    let mut q_num: enum rsi_coex_queues = RSI_COEX_Q_INVALID;
    if (skb_queue_len(&coex_cb.coex_tx_qs[RSI_COEX_Q_COMMON]) > 0)
    q_num = RSI_COEX_Q_COMMON;
    if (skb_queue_len(&coex_cb.coex_tx_qs[RSI_COEX_Q_BT]) > 0)
    q_num = RSI_COEX_Q_BT;
    if (skb_queue_len(&coex_cb.coex_tx_qs[RSI_COEX_Q_WLAN]) > 0)
    q_num = RSI_COEX_Q_WLAN;
    return q_num;
    }
#[no_mangle]
unsafe extern "C" fn rsi_coex_sched_tx_pkts(coex_cb: *mut rsi_coex_ctrl_block) {
    static void rsi_coex_sched_tx_pkts(struct rsi_coex_ctrl_block *coex_cb)
    {
    let mut coex_q: enum rsi_coex_queues = RSI_COEX_Q_INVALID;
    struct sk_buff *skb;
    do {
    coex_q = rsi_coex_determine_coex_q(coex_cb);
    rsi_dbg(INFO_ZONE, "queue = %d\n", coex_q);
    if (coex_q == RSI_COEX_Q_BT) {
    skb = skb_dequeue(&coex_cb.coex_tx_qs[RSI_COEX_Q_BT]);
    rsi_send_bt_pkt(coex_cb.priv, skb);
    }
    } while (coex_q != RSI_COEX_Q_INVALID);
    }
#[no_mangle]
unsafe extern "C" fn rsi_coex_scheduler_thread(data: *mut c_void) -> c_int {
    static int rsi_coex_scheduler_thread(void *data)
    {
    struct rsi_common *common = data;
    struct rsi_coex_ctrl_block *coex_cb = common.coex_cb;
    let mut timeout: u32 = EVENT_WAIT_FOREVER;
    do {
    rsi_wait_event(&coex_cb.coex_tx_thread.event, timeout);
    rsi_reset_event(&coex_cb.coex_tx_thread.event);
    rsi_coex_sched_tx_pkts(coex_cb);
    } while (atomic_read(&coex_cb.coex_tx_thread.thread_done) == 0);
    kthread_complete_and_exit(&coex_cb.coex_tx_thread.completion, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn rsi_coex_recv_pkt(common: *mut rsi_common, msg: *mut u8) -> c_int {
    int rsi_coex_recv_pkt(struct rsi_common *common, u8 *msg)
    {
    let mut msg_type: u8 = msg[RSI_RX_DESC_MSG_TYPE_OFFSET];
    switch (msg_type) {
    case COMMON_CARD_READY_IND:
    rsi_dbg(INFO_ZONE, "common card ready received\n");
    common.hibernate_resume = false;
    rsi_handle_card_ready(common, msg);
    break;
    case SLEEP_NOTIFY_IND:
    rsi_dbg(INFO_ZONE, "sleep notify received\n");
    rsi_mgmt_pkt_recv(common, msg);
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rsi_map_coex_q(hal_queue: u8) -> c_int {
    static inline int rsi_map_coex_q(u8 hal_queue)
    {
    switch (hal_queue) {
    case RSI_COEX_Q:
    return RSI_COEX_Q_COMMON;
    case RSI_WLAN_Q:
    return RSI_COEX_Q_WLAN;
    case RSI_BT_Q:
    return RSI_COEX_Q_BT;
    }
    return RSI_COEX_Q_INVALID;
    }
#[no_mangle]
pub unsafe extern "C" fn rsi_coex_send_pkt(priv: *mut c_void, skb: *mut sk_buff, hal_queue: u8) -> c_int {
    int rsi_coex_send_pkt(void *priv, struct sk_buff *skb, u8 hal_queue)
    {
    struct rsi_common *common = priv;
    struct rsi_coex_ctrl_block *coex_cb = common.coex_cb;
    struct skb_info *tx_params = core::ptr::null_mut();
    enum rsi_coex_queues coex_q;
    int status;
    coex_q = rsi_map_coex_q(hal_queue);
    if (coex_q == RSI_COEX_Q_INVALID) {
    rsi_dbg(ERR_ZONE, "Invalid coex queue\n");
    return -EINVAL;
    }
    if (coex_q != RSI_COEX_Q_COMMON &&
    coex_q != RSI_COEX_Q_WLAN) {
    skb_queue_tail(&coex_cb.coex_tx_qs[coex_q], skb);
    rsi_set_event(&coex_cb.coex_tx_thread.event);
    return 0;
    }
    if (common.iface_down) {
    tx_params =
    (struct skb_info *)&IEEE80211_SKB_CB(skb).driver_data;
    if (!(tx_params.flags & INTERNAL_MGMT_PKT)) {
    rsi_indicate_tx_status(common.priv, skb, -EINVAL);
    return 0;
    }
    }
// Send packet to hal
    if (skb.priority == MGMT_SOFT_Q)
    status = rsi_send_mgmt_pkt(common, skb);
    else
    status = rsi_send_data_pkt(common, skb);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn rsi_coex_attach(common: *mut rsi_common) -> c_int {
    int rsi_coex_attach(struct rsi_common *common)
    {
    struct rsi_coex_ctrl_block *coex_cb;
    int cnt;
    coex_cb = kzalloc_obj(*coex_cb);
    if (!coex_cb)
    return -ENOMEM;
    common.coex_cb = (void *)coex_cb;
    coex_cb.priv = common;
// Initialize co-ex queues
    for (cnt = 0; cnt < NUM_COEX_TX_QUEUES; cnt++)
    skb_queue_head_init(&coex_cb.coex_tx_qs[cnt]);
    rsi_init_event(&coex_cb.coex_tx_thread.event);
// Initialize co-ex thread
    if (rsi_create_kthread(common,
    &coex_cb.coex_tx_thread,
    rsi_coex_scheduler_thread,
    "Coex-Tx-Thread")) {
    rsi_dbg(ERR_ZONE, "%s: Unable to init tx thrd\n", __func__);
    kfree(coex_cb);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rsi_coex_detach(common: *mut rsi_common) {
    void rsi_coex_detach(struct rsi_common *common)
    {
    struct rsi_coex_ctrl_block *coex_cb = common.coex_cb;
    int cnt;
    rsi_kill_thread(&coex_cb.coex_tx_thread);
    for (cnt = 0; cnt < NUM_COEX_TX_QUEUES; cnt++)
    skb_queue_purge(&coex_cb.coex_tx_qs[cnt]);
    kfree(coex_cb);
    }
