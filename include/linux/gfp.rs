//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gfp.h
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

// Helper macro to avoid gfp flags if they are the default one

//
// !__GFP_DIRECT_RECLAIM -> direct claim is not allowed.
// !__GFP_KSWAPD_RECLAIM -> it's not safe to wake up kswapd.
// All GFP_* flags including GFP_NOWAIT use one or both flags.
// alloc_pages_nolock() is the only API that doesn't specify either flag.
//
// This is stronger than GFP_NOWAIT or GFP_ATOMIC because
// those are guaranteed to never block on a sleeping lock.
// Here we are enforcing that the allocation doesn't ever spin
// on any locks (i.e. only trylocks). There is no high level
// GFP_$FOO flag for this use in alloc_pages_nolock() as the
// regular page allocator doesn't fully support this
// allocation mode.
//

//
// GFP_ZONE_TABLE is a word size bitstring that is used for looking up the
// zone to use given the lowest 4 bits of gfp_t. Entries are GFP_ZONES_SHIFT
// bits long and there are 16 of them to cover all possible combinations of
// __GFP_DMA, __GFP_DMA32, __GFP_MOVABLE and __GFP_HIGHMEM.
//
// The zone fallback order is MOVABLE=>HIGHMEM=>NORMAL=>DMA32=>DMA.
// But GFP_MOVABLE is not only a zone specifier but also an allocation
// policy. Therefore __GFP_MOVABLE plus another zone selector is valid.
// Only 1 bit of the lowest 3 bits (DMA,DMA32,HIGHMEM) can be set to "1".
//
// bit       result
// =================
// 0x0    => NORMAL
// 0x1    => DMA or NORMAL
// 0x2    => HIGHMEM or NORMAL
// 0x3    => BAD (DMA+HIGHMEM)
// 0x4    => DMA32 or NORMAL
// 0x5    => BAD (DMA+DMA32)
// 0x6    => BAD (HIGHMEM+DMA32)
// 0x7    => BAD (HIGHMEM+DMA32+DMA)
// 0x8    => NORMAL (MOVABLE+0)
// 0x9    => DMA or NORMAL (MOVABLE+DMA)
// 0xa    => MOVABLE (Movable is valid only if HIGHMEM is set too)
// 0xb    => BAD (MOVABLE+HIGHMEM+DMA)
// 0xc    => DMA32 or NORMAL (MOVABLE+DMA32)
// 0xd    => BAD (MOVABLE+DMA32+DMA)
// 0xe    => BAD (MOVABLE+DMA32+HIGHMEM)
// 0xf    => BAD (MOVABLE+DMA32+HIGHMEM+DMA)
//
// GFP_ZONES_SHIFT must be <= 2 on 32 bit platforms.
//

// ZONE_DEVICE is not a valid GFP zone specifier
pub const GFP_ZONES_SHIFT: c_int = 2;

//
// GFP_ZONE_BAD is a bitmap for all combinations of __GFP_DMA, __GFP_DMA32
// __GFP_HIGHMEM and __GFP_MOVABLE that are not permitted. One flag per
// entry starting with bit 0. Bit is set if the combination is not
// allowed.
//

//
// There is only one page-allocator function, and two main namespaces to
// it. The alloc_page*() variants return 'struct page *' and as such
// can allocate highmem pages, the *get*page*() variants return
// virtual kernel addresses to the allocated page(s).
//

//
// gfp flag masking for nested internal allocations.
//
// For code that needs to do allocations inside the public allocation API (e.g.
// memory allocation tracking code) the allocations need to obey the caller
// allocation context constrains to prevent allocation context mismatches (e.g.
// GFP_KERNEL allocations in GFP_NOFS contexts) from potential deadlock
// situations.
//
// It is also assumed that these nested allocations are for internal kernel
// object storage purposes only and are not going to be used for DMA, etc. Hence
// we strip out all the zone information and leave just the context information
// intact.
//
// Further, internal allocations must fail before the higher level allocation
// can fail, so we must make them fail faster and fail silently. We also don't
// want them to deplete emergency reserves.  Hence nested allocations must be
// prepared for these allocations to fail.
//
// We get the zone list from the current node and the gfp_mask.
// This zone list contains a maximum of MAX_NUMNODES*MAX_NR_ZONES zones.
// There are two zonelists per node, one for all zones with memory and
// one containing just zones from the node the zonelist belongs to.
//
// For the case of non-NUMA systems the NODE_DATA() gets optimized to
// &contig_page_data at compile-time.
//
extern "C" {
    pub fn NODE_DATA(gfp_zonelist(flags: nid)->node_zonelists +) -> return;
}

extern "C" {
    pub fn free_pages_bulk(page_array: *mut page, nr_pages: c_ulong);
}

// Bulk allocate order-0 pages

extern "C" {
    pub fn alloc_pages_bulk_noprof(_arg: gfp, _arg: nid, _arg: NULL, _arg: nr_pages, _arg: page_array) -> return;
}

extern "C" {
    pub fn __folio_alloc_noprof(_arg: gfp, _arg: order, _arg: nid, _arg: NULL) -> return;
}

//
// Allocate pages, preferring the node given as nid. When nid == NUMA_NO_NODE,
// prefer the current CPU's closest node. Otherwise node must be valid and
// online.
//

extern "C" {
    pub fn alloc_pages_node_noprof(_arg: numa_node_id(), _arg: gfp_mask, _arg: order) -> return;
}
extern "C" {
    pub fn __folio_alloc_node_noprof(_arg: gfp, _arg: order, _arg: numa_node_id()) -> return;
}
extern "C" {
    pub fn folio_alloc_noprof(_arg: gfp, _arg: order) -> return;
}
extern "C" {
    pub fn folio_alloc_noprof(_arg: gfp, _arg: order) -> return;
}

extern "C" {
    pub fn get_free_pages_noprof(gfp_mask: gfp_t, order: c_uint) -> c_ulong;
}

extern "C" {
    pub fn get_zeroed_page_noprof(gfp_mask: gfp_t) -> c_ulong;
}

extern "C" {
    pub fn free_pages_exact(virt: *mut c_void, size: usize);
}

extern "C" {
    pub fn __free_pages(page: *mut page, order: c_uint);
}
extern "C" {
    pub fn free_pages_nolock(page: *mut page, order: c_uint);
}
extern "C" {
    pub fn free_pages(addr: c_ulong, order: c_uint);
}

extern "C" {
    pub fn drain_local_pages(zone: *mut zone);
}
extern "C" {
    pub fn page_alloc_init_late();
}
extern "C" {
    pub fn setup_pcp_cacheinfo(cpu: c_uint);
}
//
// gfp_allowed_mask is set to GFP_BOOT_MASK during early boot to restrict what
// GFP flags are used before interrupts are enabled. Once interrupts are
// enabled, it is set to __GFP_BITS_MASK while the system is running. During
// hibernation, it is used by PM to avoid I/O during memory allocation while
// devices are suspended.
//
// Returns true if the gfp_mask allows use of ALLOC_NO_WATERMARK
extern "C" {
    pub fn gfp_pfmemalloc_allowed(gfp_mask: gfp_t) -> bool;
}
// A helper for checking if gfp includes all the specified flags
extern "C" {
    pub fn gfp_has_flags(_arg: gfp, __GFP_FS: __GFP_IO |) -> return;
}
//
// Check if the gfp flags allow compaction - GFP_NOIO is a really
// tricky context because the migration might require IO.
//
extern "C" {
    pub fn IS_ENABLED(__GFP_IO: CONFIG_COMPACTION) && (gfp_mask &) -> return;
}
extern "C" {
    pub fn vma_thp_gfp_mask(vma: *mut vm_area_struct) -> gfp_t;
}

pub type acr_flags_t = u32;

// The below functions must be run on a range from a single zone.

extern "C" {
    pub fn free_contig_frozen_range(pfn: c_ulong, nr_pages: c_ulong);
}
extern "C" {
    pub fn free_contig_range(pfn: c_ulong, nr_pages: c_ulong);
}

extern "C" {
    pub fn __free_contig_range(pfn: c_ulong, nr_pages: c_ulong);
}
