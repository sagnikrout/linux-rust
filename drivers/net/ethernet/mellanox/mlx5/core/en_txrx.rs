//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_txrx.c
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
// Copyright (c) 2015, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[no_mangle]
pub unsafe extern "C" fn mlx5e_channel_no_affinity_change(c: *mut mlx5e_channel) -> bool {
    static inline bool mlx5e_channel_no_affinity_change(struct mlx5e_channel *c)
    {
    let mut current_cpu: c_int = smp_processor_id();
    return cpumask_test_cpu(current_cpu, c.aff_mask);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_handle_tx_dim(sq: *mut mlx5e_txqsq) {
    static void mlx5e_handle_tx_dim(struct mlx5e_txqsq *sq)
    {
    struct mlx5e_sq_stats *stats = sq.stats;
    let mut dim_sample: dim_sample = {};
    if (unlikely(!test_bit(MLX5E_SQ_STATE_DIM, &sq.state)))
    return;
    dim_update_sample(sq.cq.event_ctr, stats.packets, stats.bytes, &dim_sample);
    net_dim(sq.dim, &dim_sample);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_handle_rx_dim(rq: *mut mlx5e_rq) {
    static void mlx5e_handle_rx_dim(struct mlx5e_rq *rq)
    {
    struct mlx5e_rq_stats *stats = rq.stats;
    let mut dim_sample: dim_sample = {};
    if (unlikely(!test_bit(MLX5E_RQ_STATE_DIM, &rq.state)))
    return;
    dim_update_sample(rq.cq.event_ctr, stats.packets, stats.bytes, &dim_sample);
    net_dim(rq.dim, &dim_sample);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_trigger_irq(sq: *mut mlx5e_icosq) {
    void mlx5e_trigger_irq(struct mlx5e_icosq *sq)
    {
    struct mlx5_wq_cyc *wq = &sq.wq;
    struct mlx5e_tx_wqe *nopwqe;
    let mut pi: u16 = mlx5_wq_cyc_ctr2ix(wq, sq.pc);
    sq.db.wqe_info[pi] = (struct mlx5e_icosq_wqe_info) {
    .wqe_type   = MLX5E_ICOSQ_WQE_NOP,
    .num_wqebbs = 1,
    };
    nopwqe = mlx5e_post_nop(wq, sq.sqn, &sq.pc);
    mlx5e_notify_hw(wq, sq.pc, sq.uar_map, &nopwqe.ctrl);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_napi_xsk_post(xsksq: *mut mlx5e_xdpsq, xskrq: *mut mlx5e_rq) -> bool {
    static bool mlx5e_napi_xsk_post(struct mlx5e_xdpsq *xsksq, struct mlx5e_rq *xskrq)
    {
    let mut need_wakeup: bool = xsk_uses_need_wakeup(xskrq.xsk_pool);
    let mut busy_xsk: bool = false, xsk_rx_alloc_err;
// If SQ is empty, there are no TX completions to trigger NAPI, so set
// need_wakeup. Do it before queuing packets for TX to avoid race
// condition with userspace.
//
    if (need_wakeup && xsksq.pc == xsksq.cc)
    xsk_set_tx_need_wakeup(xsksq.xsk_pool);
    busy_xsk |= mlx5e_xsk_tx(xsksq, MLX5E_TX_XSK_POLL_BUDGET);
// If we queued some packets for TX, no need for wakeup anymore.
    if (need_wakeup && xsksq.pc != xsksq.cc)
    xsk_clear_tx_need_wakeup(xsksq.xsk_pool);
// If WQ is empty, RX won't trigger NAPI, so set need_wakeup. Do it
// before refilling to avoid race condition with userspace.
//
    if (need_wakeup && !mlx5e_rqwq_get_cur_sz(xskrq))
    xsk_set_rx_need_wakeup(xskrq.xsk_pool);
    xsk_rx_alloc_err = INDIRECT_CALL_2(xskrq.post_wqes,
    mlx5e_post_rx_mpwqes,
    mlx5e_post_rx_wqes,
    xskrq);
// Ask for wakeup if WQ is not full after refill.
    if (!need_wakeup)
    busy_xsk |= xsk_rx_alloc_err;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: xsk_rx_alloc_err) -> else {
    else if (xsk_rx_alloc_err)
    xsk_set_rx_need_wakeup(xskrq.xsk_pool);
    else
    xsk_clear_rx_need_wakeup(xskrq.xsk_pool);
    return busy_xsk;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    int mlx5e_napi_poll(struct napi_struct *napi, int budget)
    {
    struct mlx5e_channel *c = container_of(napi, struct mlx5e_channel,
    napi);
    struct mlx5e_icosq *aicosq = c.async_icosq;
    struct mlx5e_ch_stats *ch_stats = c.stats;
    struct mlx5e_xdpsq *xsksq = &c.xsksq;
    struct mlx5e_txqsq __rcu **qos_sqs;
    struct mlx5e_rq *xskrq = &c.xskrq;
    struct mlx5e_rq *rq = &c.rq;
    let mut aff_change: bool = false;
    let mut busy_xsk: bool = false;
    let mut busy: bool = false;
    let mut work_done: c_int = 0;
    u16 qos_sqs_size;
    bool xsk_open;
    int i;
    rcu_read_lock();
    qos_sqs = rcu_dereference(c.qos_sqs);
    xsk_open = test_bit(MLX5E_CHANNEL_STATE_XSK, c.state);
    ch_stats.poll++;
    for (i = 0; i < c.num_tc; i++)
    busy |= mlx5e_poll_tx_cq(&c.sq[i].cq, budget);
    if (unlikely(qos_sqs)) {
    smp_rmb(); /* Pairs with mlx5e_qos_alloc_queues. */
    qos_sqs_size = READ_ONCE(c.qos_sqs_size);
    for (i = 0; i < qos_sqs_size; i++) {
    struct mlx5e_txqsq *sq = rcu_dereference(qos_sqs[i]);
    if (sq)
    busy |= mlx5e_poll_tx_cq(&sq.cq, budget);
    }
    }
// budget=0 means we may be in IRQ context, do as little as possible
    if (unlikely(!budget))
    goto out;
    if (c.xdp) {
    if (c.xdpsq)
    busy |= mlx5e_poll_xdpsq_cq(&c.xdpsq.cq);
    busy |= mlx5e_poll_xdpsq_cq(&c.rq_xdpsq.cq);
    if (xsk_open)
    work_done += mlx5e_poll_rx_cq(&xskrq.cq, budget);
    }
    if (likely(budget - work_done))
    work_done += mlx5e_poll_rx_cq(&rq.cq, budget - work_done);
    busy |= work_done == budget;
    mlx5e_poll_ico_cq(&c.icosq.cq);
    if (aicosq) {
    if (mlx5e_poll_ico_cq(&aicosq.cq))
// Don't clear the flag if nothing was polled to prevent
// queueing more WQEs and overflowing the async ICOSQ.
//
    clear_bit(MLX5E_SQ_STATE_PENDING_XSK_TX,
    &aicosq.state);
// Keep after async ICOSQ CQ poll
    if (unlikely(mlx5e_ktls_rx_pending_resync_list(aicosq, budget)))
    busy |= mlx5e_ktls_rx_handle_resync_list(aicosq,
    budget);
    if (xsk_open) {
    busy |= mlx5e_poll_xdpsq_cq(&xsksq.cq);
    busy_xsk |= mlx5e_napi_xsk_post(xsksq, xskrq);
    busy |= busy_xsk;
    }
    }
    busy |= INDIRECT_CALL_2(rq.post_wqes,
    mlx5e_post_rx_mpwqes,
    mlx5e_post_rx_wqes,
    rq);
    if (busy) {
    if (likely(mlx5e_channel_no_affinity_change(c))) {
    work_done = budget;
    goto out;
    }
    ch_stats.aff_change++;
    aff_change = true;
    if (work_done == budget)
    work_done--;
    }
    if (unlikely(!napi_complete_done(napi, work_done)))
    goto out;
    ch_stats.arm++;
    for (i = 0; i < c.num_tc; i++) {
    mlx5e_handle_tx_dim(&c.sq[i]);
    mlx5e_cq_arm(&c.sq[i].cq);
    }
    if (unlikely(qos_sqs)) {
    for (i = 0; i < qos_sqs_size; i++) {
    struct mlx5e_txqsq *sq = rcu_dereference(qos_sqs[i]);
    if (sq) {
    mlx5e_handle_tx_dim(sq);
    mlx5e_cq_arm(&sq.cq);
    }
    }
    }
    mlx5e_handle_rx_dim(rq);
    mlx5e_cq_arm(&rq.cq);
    mlx5e_cq_arm(&c.icosq.cq);
    if (aicosq) {
    mlx5e_cq_arm(&aicosq.cq);
    if (xsk_open) {
    mlx5e_handle_rx_dim(xskrq);
    mlx5e_cq_arm(&xsksq.cq);
    mlx5e_cq_arm(&xskrq.cq);
    }
    if (c.xdpsq)
    mlx5e_cq_arm(&c.xdpsq.cq);
    }
    if (unlikely(aff_change && busy_xsk)) {
    mlx5e_trigger_napi_async_icosq(c);
    ch_stats.force_irq++;
    }
    out:
    rcu_read_unlock();
    return work_done;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_completion_event(mcq: *mut mlx5_core_cq, eqe: *mut mlx5_eqe) {
    void mlx5e_completion_event(struct mlx5_core_cq *mcq, struct mlx5_eqe *eqe)
    {
    struct mlx5e_cq *cq = container_of(mcq, struct mlx5e_cq, mcq);
    napi_schedule(cq.napi);
    cq.event_ctr++;
    cq.ch_stats.events++;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_cq_error_event(mcq: *mut mlx5_core_cq, event: enum mlx5_event) {
    void mlx5e_cq_error_event(struct mlx5_core_cq *mcq, enum mlx5_event event)
    {
    struct mlx5e_cq *cq = container_of(mcq, struct mlx5e_cq, mcq);
    struct net_device *netdev = cq.netdev;
    netdev_err(netdev, "%s: cqn=0x%.6x event=0x%.2x\n",
    __func__, mcq.cqn, event);
    }
