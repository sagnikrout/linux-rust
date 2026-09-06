//! Automatically rewritten from C to Rust
//! Source: net/core/gro_cells.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gro_cell {
    pub napi_skbs: sk_buff_head,
    pub napi: napi_struct,
    pub bh_lock: local_lock_t,
}

#[no_mangle]
pub unsafe extern "C" fn gro_cells_receive(gcells: *mut gro_cells, skb: *mut sk_buff) -> c_int {
    int gro_cells_receive(struct gro_cells *gcells, struct sk_buff *skb)
    {
    struct net_device *dev = skb.dev;
    let mut have_bh_lock: bool = false;
    struct gro_cell *cell;
    int res;
    rcu_read_lock();
    if (unlikely(!(dev.flags & IFF_UP)))
    goto drop;
    skb_unset_transport_header(skb);
    if (!gcells.cells || skb_cloned(skb) || netif_elide_gro(dev)) {
    res = netif_rx(skb);
    goto unlock;
    }
    local_lock_nested_bh(&gcells.cells.bh_lock);
    have_bh_lock = true;
    cell = this_cpu_ptr(gcells.cells);
    if (skb_queue_len(&cell.napi_skbs) > READ_ONCE(net_hotdata.max_backlog)) {
    drop:
    dev_core_stats_rx_dropped_inc(dev);
    kfree_skb(skb);
    res = NET_RX_DROP;
    goto unlock;
    }
    __skb_queue_tail(&cell.napi_skbs, skb);
    if (skb_queue_len(&cell.napi_skbs) == 1)
    napi_schedule(&cell.napi);
    res = NET_RX_SUCCESS;
    unlock:
    if (have_bh_lock)
    local_unlock_nested_bh(&gcells.cells.bh_lock);
    rcu_read_unlock();
    return res;
    }
    EXPORT_SYMBOL(gro_cells_receive);
// called under BH context
#[no_mangle]
unsafe extern "C" fn gro_cell_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int gro_cell_poll(struct napi_struct *napi, int budget)
    {
    struct gro_cell *cell = container_of(napi, struct gro_cell, napi);
    struct sk_buff *skb;
    let mut work_done: c_int = 0;
    while (work_done < budget) {
    __local_lock_nested_bh(&cell.bh_lock);
    skb = __skb_dequeue(&cell.napi_skbs);
    __local_unlock_nested_bh(&cell.bh_lock);
    if (!skb)
    break;
    napi_gro_receive(napi, skb);
    work_done++;
    }
    if (work_done < budget)
    napi_complete_done(napi, work_done);
    return work_done;
    }
#[no_mangle]
pub unsafe extern "C" fn gro_cells_init(gcells: *mut gro_cells, dev: *mut net_device) -> c_int {
    int gro_cells_init(struct gro_cells *gcells, struct net_device *dev)
    {
    int i;
    gcells.cells = alloc_percpu(struct gro_cell);
    if (!gcells.cells)
    return -ENOMEM;
    for_each_possible_cpu(i) {
    struct gro_cell *cell = per_cpu_ptr(gcells.cells, i);
    __skb_queue_head_init(&cell.napi_skbs);
    local_lock_init(&cell.bh_lock);
    set_bit(NAPI_STATE_NO_BUSY_POLL, &cell.napi.state);
    netif_napi_add(dev, &cell.napi, gro_cell_poll);
    napi_enable(&cell.napi);
    }
    return 0;
    }
    EXPORT_SYMBOL(gro_cells_init);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_free_defer {
    pub rcu: rcu_head,
    pub ptr: *mut void __percpu,
}

#[no_mangle]
unsafe extern "C" fn percpu_free_defer_callback(head: *mut rcu_head) {
    static void percpu_free_defer_callback(struct rcu_head *head)
    {
    struct percpu_free_defer *defer;
    defer = container_of(head, struct percpu_free_defer, rcu);
    free_percpu(defer.ptr);
    kfree(defer);
    }
#[no_mangle]
pub unsafe extern "C" fn gro_cells_destroy(gcells: *mut gro_cells) {
    void gro_cells_destroy(struct gro_cells *gcells)
    {
    struct percpu_free_defer *defer;
    int i;
    if (!gcells.cells)
    return;
    for_each_possible_cpu(i) {
    struct gro_cell *cell = per_cpu_ptr(gcells.cells, i);
    napi_disable(&cell.napi);
    __netif_napi_del(&cell.napi);
    __skb_queue_purge(&cell.napi_skbs);
    }
// We need to observe an rcu grace period before freeing ->cells,
// because netpoll could access dev->napi_list under rcu protection.
// Try hard using call_rcu() instead of synchronize_rcu(),
// because we might be called from cleanup_net(), and we
// definitely do not want to block this critical task.
//
    defer = kmalloc_obj(*defer, GFP_KERNEL | __GFP_NOWARN);
    if (likely(defer)) {
    defer.ptr = gcells.cells;
    call_rcu(&defer.rcu, percpu_free_defer_callback);
    } else {
// We do not hold RTNL at this point, synchronize_net()
// would not be able to expedite this sync.
//
    synchronize_rcu_expedited();
    free_percpu(gcells.cells);
    }
    gcells.cells = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(gro_cells_destroy);
