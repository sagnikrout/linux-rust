//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/page_pool/types.h
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

// map/unmap
//

// from page_pool will be
// DMA-synced-for-device according to
// the length provided by the device
// driver.
// Please note DMA-sync-for-CPU is still
// device driver responsibility
//

// Allow unreadable (net_iov backed) netmem in this page_pool. Drivers setting
// this must be able to support unreadable netmem, where netmem_address() would
// return NULL. This flag should not be set for header page_pools.
//
// If the driver sets PP_FLAG_ALLOW_UNREADABLE_NETMEM, it should also set
// page_pool_params.slow.queue_idx.
//

// Index limit to stay within PP_DMA_INDEX_BITS for DMA indices

//
// Fast allocation side cache array/stack
//
// The cache size and refill watermark is related to the network
// use-case.  The NAPI budget is 64 packets.  After a NAPI poll the RX
// ring is usually refilled and the max consumed elements will be 64,
// thus a natural max size of objects needed in the cache.
// The refill watermark is set to 64 for 4KB pages,
// and scales to balance its size in bytes across page sizes.
//
// Keeping room for more objects, is due to XDP_DROP use-case.  As
// XDP_DROP allows the opportunity to recycle objects directly into
// this array, as it shares the same softirq/NAPI protection.  If
// cache is already full (or partly full) then the XDP_DROP recycles
// would have to take a slower code path.
//

pub const PP_ALLOC_CACHE_REFILL: c_int = 4;

pub const PP_ALLOC_CACHE_REFILL: c_int = 16;

pub const PP_ALLOC_CACHE_REFILL: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_alloc_cache {
    pub count: u32,
    pub cache: [netmem_ref; PP_ALLOC_CACHE_SIZE],
}

//
// struct page_pool_params - page pool parameters
// @fast:	params accessed frequently on hotpath
// @order:	2^order pages on allocation
// @pool_size:	size of the ptr_ring
// @nid:	NUMA node id to allocate from pages from
// @dev:	device, for DMA pre-mapping purposes
// @napi:	NAPI which is the sole consumer of pages, otherwise NULL
// @dma_dir:	DMA mapping direction
// @max_len:	max DMA sync memory size for PP_FLAG_DMA_SYNC_DEV
// @offset:	DMA sync address offset for PP_FLAG_DMA_SYNC_DEV
// @slow:	params with slowpath access only (initialization and Netlink)
// @netdev:	netdev this pool will serve (leave as NULL if none or multiple)
// @queue_idx:	queue idx this page_pool is being created for.
// @flags:	PP_FLAG_DMA_MAP, PP_FLAG_DMA_SYNC_DEV, PP_FLAG_SYSTEM_POOL,
// PP_FLAG_ALLOW_UNREADABLE_NETMEM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool_params {
    pub order: c_uint,
    pub pool_size: c_uint,
    pub nid: c_int,
    pub dev: *mut device,
    pub napi: *mut napi_struct,
    pub dma_dir: dma_data_direction,
    pub max_len: c_uint,
    pub offset: c_uint,
    pub netdev: *mut net_device,
    pub queue_idx: c_uint,
    pub flags: c_uint,
// private: used by test code only
    pub arg): *mut *mut void (init_callback)(netmem_ref netmem, void,
    pub init_arg: *mut c_void,
}

//
// struct page_pool_alloc_stats - allocation statistics
// @fast:	successful fast path allocations
// @slow:	slow path order-0 allocations
// @slow_high_order: slow path high order allocations
// @empty:	ptr ring is empty, so a slow path allocation was forced
// @refill:	an allocation which triggered a refill of the cache
// @waive:	pages obtained from the ptr ring that cannot be added to
// the cache due to a NUMA mismatch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool_alloc_stats {
    pub fast: u64,
    pub slow: u64,
    pub slow_high_order: u64,
    pub empty: u64,
    pub refill: u64,
    pub waive: u64,
}

//
// struct page_pool_recycle_stats - recycling (freeing) statistics
// @cached:	recycling placed page in the page pool cache
// @cache_full:	page pool cache was full
// @ring:	page placed into the ptr ring
// @ring_full:	page released from page pool because the ptr ring was full
// @released_refcnt:	page released (and not recycled) because refcnt > 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool_recycle_stats {
    pub cached: u64,
    pub cache_full: u64,
    pub ring: u64,
    pub ring_full: u64,
    pub released_refcnt: u64,
}

//
// struct page_pool_stats - combined page pool use statistics
// @alloc_stats:	see struct page_pool_alloc_stats
// @recycle_stats:	see struct page_pool_recycle_stats
//
// Wrapper struct for combining page pool stats with different storage
// requirements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool_stats {
    pub alloc_stats: page_pool_alloc_stats,
    pub recycle_stats: page_pool_recycle_stats,
}

// The whole frag API block must stay within one cacheline. On 32-bit systems,
// sizeof(long) == sizeof(int), so that the block size is ``3 * sizeof(long)``.
// On 64-bit systems, the actual size is ``2 * sizeof(long) + sizeof(int)``.
// The closest pow-2 to both of them is ``4 * sizeof(long)``, so just use that
// one for simplicity.
// Having it aligned to a cacheline boundary may be excessive and doesn't bring
// any good.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_memory_provider_params {
    pub mp_priv: *mut c_void,
    pub mp_ops: *const memory_provider_ops,
    pub rx_page_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool {
    pub p: page_pool_params_fast,
    pub cpuid: c_int,
    pub pages_state_hold_cnt: u32,
    pub /: *mut *mut bool has_init_callback:1; / slow::init_callback is set,
    pub /: *mut *mut bool dma_map:1; / Perform DMA mapping,
    pub /: *mut *mut bool dma_sync:1; / Perform DMA sync for device,
    pub /: *mut *mut bool dma_sync_for_cpu:1; / Perform DMA sync for cpu,

    pub /: *mut *mut bool system:1; / This is a global percpu pool,

    pub PAGE_POOL_FRAG_GROUP_ALIGN): __cacheline_group_begin_aligned(frag,,
    pub frag_users: c_long,
    pub frag_page: netmem_ref,
    pub frag_offset: c_uint,
    pub PAGE_POOL_FRAG_GROUP_ALIGN): __cacheline_group_end_aligned(frag,,
    pub release_dw: delayed_work,
    pub pool): *mut *mut void (disconnect)(void,
    pub defer_start: c_ulong,
    pub defer_warn: c_ulong,

// these stats are incremented while in softirq context
    pub alloc_stats: page_pool_alloc_stats,

    pub xdp_mem_id: u32,
//
// Data structure for allocation side
//
// Drivers allocation side usually already perform some kind
// of resource protection.  Piggyback on this protection, and
// require driver to protect allocation side.
//
// For NIC drivers this means, allocate a page_pool per
// RX-queue. As the RX-queue is already protected by
// Softirq/BH scheduling and napi_schedule. NAPI schedule
// guarantee that a single napi_struct will only be scheduled
// on a single CPU (see napi_schedule).
//
    pub ____cacheline_aligned_in_smp: pp_alloc_cache alloc,
// Data structure for storing recycled pages.
//
// Returning/freeing pages is more complicated synchronization
// wise, because free's can happen on remote CPUs, with no
// association with allocation resource.
//
// Use ptr_ring, as it separates consumer and producer
// efficiently, it a way that doesn't bounce cache-lines.
//
// TODO: Implement bulk return pages into this structure.
//
    pub ring: ptr_ring,
    pub mp_priv: *mut c_void,
    pub mp_ops: *const memory_provider_ops,
    pub dma_mapped: xarray,

// recycle stats are per-cpu to avoid locking
    pub recycle_stats: *mut page_pool_recycle_stats __percpu,

    pub pages_state_release_cnt: core::sync::atomic::AtomicI32,
// A page_pool is strictly tied to a single RX-queue being
// protected by NAPI, due to above pp_alloc_cache. This
// refcnt serves purpose is to simplify drivers error handling.
//
    pub user_cnt: refcount_t,
    pub destroy_cnt: u64,
// Slow/Control-path information follows
    pub slow: page_pool_params_slow,
// User-facing fields, protected by page_pools_lock
    pub list: hlist_node,
    pub detach_time: ktime_t,
    pub id: u32,
    pub user: },
}

extern "C" {
    pub fn page_pool_alloc_netmems(pool: *mut page_pool, gfp: gfp_t) -> netmem_ref;
}

extern "C" {
    pub fn page_pool_disable_direct_recycling(pool: *mut page_pool);
}
extern "C" {
    pub fn page_pool_destroy(pool: *mut page_pool);
}
extern "C" {
    pub fn page_pool_put_netmem_bulk(data: *mut netmem_ref, count: u32);
}

// Caller must provide appropriate safe context, e.g. NAPI.
extern "C" {
    pub fn page_pool_update_nid(pool: *mut page_pool, new_nid: c_int);
}
