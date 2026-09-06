//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netdev_queues.h
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
// struct netdev_config - queue-related configuration for a netdev
// @hds_thresh:		HDS Threshold value.
// @hds_config:		HDS value from userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_config {
    pub hds_thresh: u32,
    pub hds_config: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_queue_config {
    pub rx_page_size: u32,
}

// See the netdev.yaml spec for definition of each statistic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_queue_stats_rx {
    pub bytes: u64,
    pub packets: u64,
    pub alloc_fail: u64,
    pub hw_drops: u64,
    pub hw_drop_overruns: u64,
    pub csum_complete: u64,
    pub csum_unnecessary: u64,
    pub csum_none: u64,
    pub csum_bad: u64,
    pub hw_gro_packets: u64,
    pub hw_gro_bytes: u64,
    pub hw_gro_wire_packets: u64,
    pub hw_gro_wire_bytes: u64,
    pub hw_drop_ratelimits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_queue_stats_tx {
    pub bytes: u64,
    pub packets: u64,
    pub hw_drops: u64,
    pub hw_drop_errors: u64,
    pub csum_none: u64,
    pub needs_csum: u64,
    pub hw_gso_packets: u64,
    pub hw_gso_bytes: u64,
    pub hw_gso_wire_packets: u64,
    pub hw_gso_wire_bytes: u64,
    pub hw_drop_ratelimits: u64,
    pub stop: u64,
    pub wake: u64,
}

//
// struct netdev_stat_ops - netdev ops for fine grained stats
// @get_queue_stats_rx:	get stats for a given Rx queue
// @get_queue_stats_tx:	get stats for a given Tx queue
// @get_base_stats:	get base stats (not belonging to any live instance)
//
// Query stats for a given object. The values of the statistics are undefined
// on entry (specifically they are *not* zero-initialized). Drivers should
// assign values only to the statistics they collect. Statistics which are not
// collected must be left undefined.
//
// Queue objects are not necessarily persistent, and only currently active
// queues are queried by the per-queue callbacks. This means that per-queue
// statistics will not generally add up to the total number of events for
// the device. The @get_base_stats callback allows filling in the delta
// between events for currently live queues and overall device history.
// @get_base_stats can also be used to report any miscellaneous packets
// transferred outside of the main set of queues used by the networking stack.
// When the statistics for the entire device are queried, first @get_base_stats
// is issued to collect the delta, and then a series of per-queue callbacks.
// Only statistics which are set in @get_base_stats will be reported
// at the device level, meaning that unlike in queue callbacks, setting
// a statistic to zero in @get_base_stats is a legitimate thing to do.
// This is because @get_base_stats has a second function of designating which
// statistics are in fact correct for the entire device (e.g. when history
// for some of the events is not maintained, and reliable "total" cannot
// be provided).
//
// Ops are called under the instance lock if netdev_need_ops_lock()
// returns true, otherwise under rtnl_lock.
// Device drivers can assume that when collecting total device stats,
// the @get_base_stats and subsequent per-queue calls are performed
// "atomically" (without releasing the relevant lock).
//
// Device drivers are encouraged to reset the per-queue statistics when
// number of queues change. This is because the primary use case for
// per-queue statistics is currently to detect traffic imbalance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_stat_ops {
    pub stats): *mut netdev_queue_stats_rx,
    pub stats): *mut netdev_queue_stats_tx,
    pub tx): *mut netdev_queue_stats_tx,
}

// The queue checks and honours the page size qcfg parameter
//
// struct netdev_queue_mgmt_ops - netdev ops for queue management
//
// @ndo_queue_mem_size: Size of the struct that describes a queue's memory.
//
// @ndo_queue_mem_alloc: Allocate memory for an RX queue at the specified index.
// The new memory is written at the specified address.
//
// @ndo_queue_mem_free:	Free memory from an RX queue.
//
// @ndo_queue_start:	Start an RX queue with the specified memory and at the
// specified index.
//
// @ndo_queue_stop:	Stop the RX queue at the specified index. The stopped
// queue's memory is written at the specified address.
//
// @ndo_queue_get_dma_dev: Get dma device for zero-copy operations to be used
// for this queue. Return NULL on error.
//
// @ndo_default_qcfg:	(Optional) Populate queue config struct with defaults.
// Queue config structs are passed to this helper before
// the user-requested settings are applied.
//
// @ndo_validate_qcfg: (Optional) Check if queue config is supported.
// Called when configuration affecting a queue may be
// changing, either due to NIC-wide config, or config
// scoped to the queue at a specified index.
// When NIC-wide config is changed the callback will
// be invoked for all queues.
//
// @ndo_queue_create:	Create a new RX queue on a virtual device that will
// be paired with a physical device's queue via leasing.
// Return the new queue id on success, negative error
// on failure.
//
// @supported_params:	Bitmask of supported parameters, see QCFG_*.
//
// Note that @ndo_queue_mem_alloc and @ndo_queue_mem_free may be called while
// the interface is closed. @ndo_queue_start and @ndo_queue_stop will only
// be called for an interface which is open.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_queue_mgmt_ops {
    pub ndo_queue_mem_size: usize,
    pub idx): c_int,
    pub per_queue_mem): *mut c_void,
    pub idx): c_int,
    pub idx): c_int,
    pub qcfg): *mut netdev_queue_config,
    pub extack): *mut netlink_ext_ack,
    pub idx): c_int,
    pub extack): *mut netlink_ext_ack,
    pub supported_params: c_uint,
}

extern "C" {
    pub fn netif_rxq_has_unreadable_mp(dev: *mut net_device, rxq_idx: c_uint) -> bool;
}
//
// DOC: Lockless queue stopping / waking helpers.
//
// The netif_txq_maybe_stop() and __netif_txq_completed_wake()
// macros are designed to safely implement stopping
// and waking netdev queues without full lock protection.
//
// We assume that there can be no concurrent stop attempts and no concurrent
// wake attempts. The try-stop should happen from the xmit handler,
// while wake up should be triggered from NAPI poll context.
// The two may run concurrently (single producer, single consumer).
//
// The try-stop side is expected to run from the xmit handler and therefore
// it does not reschedule Tx (netif_tx_start_queue() instead of
// netif_tx_wake_queue()). Uses of the ``stop`` macros outside of the xmit
// handler may lead to xmit queue being enabled but not run.
// The waking side does not have similar context restrictions.
//
// The macros guarantee that rings will not remain stopped if there's
// space available, but they do *not* prevent false wake ups when
// the ring is full! Drivers should check for ring full at the start
// for the xmit handler.
//
// All descriptor ring indexes (and other relevant shared state) must
// be updated before invoking the macros.
//

// Producer index and stop bit must be visible		\
// to consumer before we recheck.			\
// Pairs with a barrier in __netif_txq_completed_wake(). \
// \
// We need to check again in a case another		\
// CPU has just made room available.			\
// \
//
// netif_txq_maybe_stop() - locklessly stop a Tx queue, if needed
// @txq:	struct netdev_queue to stop/start
// @get_desc:	get current number of free descriptors (see requirements below!)
// @stop_thrs:	minimal number of available descriptors for queue to be left
// enabled
// @start_thrs:	minimal number of descriptors to re-enable the queue, can be
// equal to @stop_thrs or higher to avoid frequent waking
//
// All arguments may be evaluated multiple times, beware of side effects.
// @get_desc must be a formula or a function call, it must always
// return up-to-date information when evaluated!
// Expected to be used from ndo_start_xmit, see the comment on top of the file.
//
// Returns:
// 0 if the queue was stopped
// 1 if the queue was left enabled
// -1 if the queue was re-enabled (raced with waking)
//

// Variant of netdev_tx_completed_queue() which guarantees smp_mb() if
// @bytes != 0, regardless of kernel config.
//
// __netif_txq_completed_wake() - locklessly wake a Tx queue, if needed
// @txq:	struct netdev_queue to stop/start
// @pkts:	number of packets completed
// @bytes:	number of bytes completed
// @get_desc:	get current number of free descriptors (see requirements below!)
// @start_thrs:	minimal number of descriptors to re-enable the queue
// @down_cond:	down condition, predicate indicating that the queue should
// not be woken up even if descriptors are available
//
// All arguments may be evaluated multiple times.
// @get_desc must be a formula or a function call, it must always
// return up-to-date information when evaluated!
// Reports completed pkts/bytes to BQL.
//
// Returns:
// 0 if the queue was woken up
// 1 if the queue was already enabled (or disabled but @down_cond is true)
// -1 if the queue was left unchanged (@start_thrs not reached)
//

// Report to BQL and piggy back on its barrier.		\
// Barrier makes sure that anybody stopping the queue	\
// after this point sees the new consumer index.	\
// Pairs with barrier in netif_txq_try_stop().		\
// \

// subqueue variants follow

extern "C" {
    pub fn jiffies_to_msecs(trans_start: jiffies -) -> return;
}

