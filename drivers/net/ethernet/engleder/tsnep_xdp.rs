//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/engleder/tsnep_xdp.c
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
// Copyright (C) 2022 Gerhard Engleder <gerhard@engleder-embedded.com>

    int tsnep_xdp_setup_prog(struct tsnep_adapter *adapter, struct bpf_prog *prog,
    struct netlink_ext_ack *extack)
    {
    struct bpf_prog *old_prog;
    old_prog = xchg(&adapter.xdp_prog, prog);
    if (old_prog)
    bpf_prog_put(old_prog);
    return 0;
    }
    static int tsnep_xdp_enable_pool(struct tsnep_adapter *adapter,
    struct xsk_buff_pool *pool, u16 queue_id)
    {
    struct tsnep_queue *queue;
    int retval;
    if (queue_id >= adapter.num_rx_queues ||
    queue_id >= adapter.num_tx_queues)
    return -EINVAL;
    queue = &adapter.queue[queue_id];
    if (queue.rx.queue_index != queue_id ||
    queue.tx.queue_index != queue_id) {
    netdev_err(adapter.netdev,
    "XSK support only for TX/RX queue pairs\n");
    return -EOPNOTSUPP;
    }
    retval = xsk_pool_dma_map(pool, adapter.dmadev,
    DMA_ATTR_SKIP_CPU_SYNC);
    if (retval) {
    netdev_err(adapter.netdev, "failed to map XSK pool\n");
    return retval;
    }
    retval = tsnep_enable_xsk(queue, pool);
    if (retval) {
    xsk_pool_dma_unmap(pool, DMA_ATTR_SKIP_CPU_SYNC);
    return retval;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsnep_xdp_disable_pool(adapter: *mut tsnep_adapter, queue_id: u16) -> c_int {
    static int tsnep_xdp_disable_pool(struct tsnep_adapter *adapter, u16 queue_id)
    {
    struct xsk_buff_pool *pool;
    struct tsnep_queue *queue;
    if (queue_id >= adapter.num_rx_queues ||
    queue_id >= adapter.num_tx_queues)
    return -EINVAL;
    pool = xsk_get_pool_from_qid(adapter.netdev, queue_id);
    if (!pool)
    return -EINVAL;
    queue = &adapter.queue[queue_id];
    tsnep_disable_xsk(queue);
    xsk_pool_dma_unmap(pool, DMA_ATTR_SKIP_CPU_SYNC);
    return 0;
    }
    int tsnep_xdp_setup_pool(struct tsnep_adapter *adapter,
    struct xsk_buff_pool *pool, u16 queue_id)
    {
    return pool ? tsnep_xdp_enable_pool(adapter, pool, queue_id) :
    tsnep_xdp_disable_pool(adapter, queue_id);
    }
