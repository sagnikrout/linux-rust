//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netmem.h
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
// Network memory
//
// Author:	Mina Almasry <almasrymina@google.com>
//

// These fields in struct page are used by the page_pool and net stack:
//
// struct {
// unsigned long pp_magic;
// struct page_pool *pp;
// unsigned long _pp_mapping_pad;
// unsigned long dma_addr;
// atomic_long_t pp_ref_count;
// };
//
// We mirror the page_pool fields here so the page_pool can access these
// fields without worrying whether the underlying fields belong to a
// page or netmem_desc.
//
// CAUTION: Do not update the fields in netmem_desc without also
// updating the anonymous aliasing union in struct net_iov.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netmem_desc {
    pub _flags: c_ulong,
    pub pp_magic: c_ulong,
    pub pp: *mut page_pool,
    pub _pp_mapping_pad: c_ulong,
    pub dma_addr: c_ulong,
    pub pp_ref_count: atomic_long_t,
}

//
// Since struct netmem_desc uses the space in struct page, the size
// should be checked, until struct netmem_desc has its own instance from
// slab, to avoid conflicting with other members within struct page.
//
// net_iov
// We overload the LSB of the struct page pointer to indicate whether it's
// a page or net_iov.
//
pub const NET_IOV: c_uint = 0x01UL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_iov_type {
    NET_IOV_DMABUF,
    NET_IOV_IOURING,
}

// A memory descriptor representing abstract networking I/O vectors,
// generally for non-pages memory that doesn't have its corresponding
// struct page and needs to be explicitly allocated through slab.
//
// net_iovs are allocated and used by networking code, and the size of
// the chunk is PAGE_SIZE.
//
// This memory can be any form of non-struct paged memory.  Examples
// include imported dmabuf memory and imported io_uring memory.  See
// net_iov_type for all the supported types.
//
// @pp_magic:	pp field, similar to the one in struct page/struct
// netmem_desc.
// @pp:		the pp this net_iov belongs to, if any.
// @dma_addr:	the dma addrs of the net_iov. Needed for the network
// card to send/receive this net_iov.
// @pp_ref_count: the pp ref count of this net_iov, exactly the same
// usage as struct page/struct netmem_desc.
// @owner:	the net_iov_area this net_iov belongs to, if any.
// @type:	the type of the memory.  Different types of net_iovs are
// supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_iov {
    pub desc: netmem_desc,
    pub type: net_iov_type,
    pub owner: *mut net_iov_area,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_iov_area {
// Array of net_iovs for this area.
    pub niovs: *mut net_iov,
    pub num_niovs: usize,
// Offset into the dma-buf where this chunk starts.
    pub base_virtual: c_ulong,
}

// Initialize a niov: stamp the owning area, the memory provider type.
//
// netmem
//
// typedef netmem_ref - a nonexistent type marking a reference to generic
// network memory.
//
// A netmem_ref can be a struct page* or a struct net_iov* underneath.
//
// Use the supplied helpers to obtain the underlying memory pointer and fields.
//
pub type netmem_ref = unsigned long ;
//
// __netmem_to_page - unsafely get pointer to the &page backing @netmem
// @netmem: netmem reference to convert
//
// Unsafe version of netmem_to_page(). When @netmem is always page-backed,
// e.g. when it's a header buffer, performs faster and generates smaller
// object code (no check for the LSB, no WARN). When @netmem points to IOV,
// provokes undefined behaviour.
//
// Return: pointer to the &page (garbage if @netmem is not page-backed).
//
extern "C" {
    pub fn __netmem_to_page(_arg: netmem) -> return;
}

//
// virt_to_netmem - convert virtual memory pointer to a netmem reference
// @data: host memory pointer to convert
//
// Return: netmem reference to the &page backing this virtual address.
//
extern "C" {
    pub fn page_to_netmem(_arg: virt_to_page(data)) -> return;
}
// The non-pp refcount of net_iov is always 1. On net_iov, we only
// support pp refcounting which uses the pp_ref_count field.
//
extern "C" {
    pub fn page_ref_count(_arg: netmem_to_page(netmem)) -> return;
}
extern "C" {
    pub fn page_to_pfn(_arg: netmem_to_page(netmem)) -> return;
}
// XXX: How to extract netmem_desc from page must be changed, once
// netmem_desc no longer overlays on page and will be allocated through
// slab.
//

// CAUTION: Check if the page is a pp page before calling this helper or
// know it's a pp page.
//

//
// __netmem_to_nmdesc - unsafely get pointer to the &netmem_desc backing
// @netmem
// @netmem: netmem reference to convert
//
// Unsafe version that can be used only when @netmem is always backed by
// system memory, performs faster and generates smaller object code (no
// check for the LSB, no WARN). When @netmem points to IOV, provokes
// undefined behaviour.
//
// Return: pointer to the &netmem_desc (garbage if @netmem is not backed
// by system memory).
//
// netmem_to_nmdesc - convert netmem_ref to struct netmem_desc * for
// access to common fields.
// @netmem: netmem reference to get netmem_desc.
//
// All the sub types of netmem_ref (netmem_desc, net_iov) have the same
// pp, pp_magic, dma_addr, and pp_ref_count fields via netmem_desc.
//
// Return: the pointer to struct netmem_desc * regardless of its
// underlying type.
//
extern "C" {
    pub fn __pp_page_to_nmdesc()p: *mut (struct page) -> return;
}
//
// __netmem_get_pp - unsafely get pointer to the &page_pool backing @netmem
// @netmem: netmem reference to get the pointer from
//
// Unsafe version of netmem_get_pp(). When @netmem is always page-backed,
// e.g. when it's a header buffer, performs faster and generates smaller
// object code (avoids clearing the LSB). When @netmem points to IOV,
// provokes invalid memory access.
//
// Return: pointer to the &page_pool (garbage if @netmem is not page-backed).
//
// NUMA node preference only makes sense if we're allocating
// system memory. Memory providers (which give us net_iovs)
// choose for us.
//
// niov are never compounded
extern "C" {
    pub fn page_to_netmem(_arg: compound_head(netmem_to_page(netmem))) -> return;
}
//
// __netmem_address - unsafely get pointer to the memory backing @netmem
// @netmem: netmem reference to get the pointer for
//
// Unsafe version of netmem_address(). When @netmem is always page-backed,
// e.g. when it's a header buffer, performs faster and generates smaller
// object code (no check for the LSB). When @netmem points to IOV, provokes
// undefined behaviour.
//
// Return: pointer to the memory (garbage if @netmem is not page-backed).
//
extern "C" {
    pub fn page_address(_arg: __netmem_to_page(netmem)) -> return;
}
extern "C" {
    pub fn __netmem_address(_arg: netmem) -> return;
}
//
// netmem_is_pfmemalloc - check if @netmem was allocated under memory pressure
// @netmem: netmem reference to check
//
// Return: true if @netmem is page-backed and the page was allocated under
// memory pressure, false otherwise.
//
extern "C" {
    pub fn page_is_pfmemalloc(_arg: netmem_to_page(netmem)) -> return;
}

extern "C" {
    pub fn __get_netmem(netmem: netmem_ref);
}
extern "C" {
    pub fn __put_netmem(netmem: netmem_ref);
}

