//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memremap.h
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
// struct vmem_altmap - pre-allocated storage for vmemmap_populate
// @base_pfn: base of the entire dev_pagemap mapping
// @reserve: pages mapped, but reserved for driver use (relative to @base)
// @free: free pages set aside in the mapping for memmap storage
// @align: pages reserved to meet allocation alignments
// @alloc: track pages consumed, private to vmemmap_populate()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmem_altmap {
    pub base_pfn: c_ulong,
    pub end_pfn: c_ulong,
    pub reserve: c_ulong,
    pub free: c_ulong,
    pub align: c_ulong,
    pub alloc: c_ulong,
}

//
// Specialize ZONE_DEVICE memory into multiple types each has a different
// usage.
//
// MEMORY_DEVICE_PRIVATE:
// Device memory that is not directly addressable by the CPU: CPU can neither
// read nor write private memory. In this case, we do still have struct pages
// backing the device memory. Doing so simplifies the implementation, but it is
// important to remember that there are certain points at which the struct page
// must be treated as an opaque object, rather than a "normal" struct page.
//
// A more complete discussion of unaddressable memory may be found in
// include/linux/hmm.h and Documentation/mm/hmm.rst.
//
// MEMORY_DEVICE_COHERENT:
// Device memory that is cache coherent from device and CPU point of view. This
// is used on platforms that have an advanced system bus (like CAPI or CXL). A
// driver can hotplug the device memory using ZONE_DEVICE and with that memory
// type. Any page of a process can be migrated to such memory. However no one
// should be allowed to pin such memory so that it can always be evicted.
//
// MEMORY_DEVICE_FS_DAX:
// Host memory that has similar access semantics as System RAM i.e. DMA
// coherent and supports page pinning. In support of coordinating page
// pinning vs other operations MEMORY_DEVICE_FS_DAX arranges for a
// wakeup event whenever a page is unpinned and becomes idle. This
// wakeup is used to coordinate physical address space management (ex:
// fs truncate/hole punch) vs pinned pages (ex: device dma).
//
// MEMORY_DEVICE_GENERIC:
// Host memory that has similar access semantics as System RAM i.e. DMA
// coherent and supports page pinning. This is for example used by DAX devices
// that expose memory using a character device.
//
// MEMORY_DEVICE_PCI_P2PDMA:
// Device memory residing in a PCI BAR intended for use with Peer-to-Peer
// transactions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memory_type {
// 0 is reserved to catch uninitialized type fields
    MEMORY_DEVICE_PRIVATE = 1,
    MEMORY_DEVICE_COHERENT,
    MEMORY_DEVICE_FS_DAX,
    MEMORY_DEVICE_GENERIC,
    MEMORY_DEVICE_PCI_P2PDMA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pagemap_ops {
//
// Called once the folio refcount reaches 0.  The reference count will be
// reset to one by the core code after the method is called to prepare
// for handing out the folio again.
//
    pub folio): *mut *mut void (folio_free)(struct folio,
//
// Used for private (un-addressable) device memory only.  Must migrate
// the page back to a CPU accessible page.
//
    pub vmf): *mut *mut vm_fault_t (migrate_to_ram)(struct vm_fault,
//
// Handle the memory failure happens on a range of pfns.  Notify the
// processes who are using these pfns, and try to recover the data on
// them if necessary.  The mf_flags is finally passed to the recover
// function through the whole notify routine.
//
// When this is not implemented, or it returns -EOPNOTSUPP, the caller
// will fall back to a common handler called mf_generic_kill_procs().
//
    pub mf_flags): unsigned long nr_pages, int,
//
// Used for private (un-addressable) device memory only.
// This callback is used when a folio is split into
// a smaller folio
//
    pub tail): *mut *mut *mut void (folio_split)(struct folio head, struct folio,
}

//
// struct dev_pagemap - metadata for ZONE_DEVICE mappings
// @altmap: pre-allocated/reserved memory for vmemmap allocations
// @ref: reference count that pins the devm_memremap_pages() mapping
// @done: completion for @ref
// @type: memory type: see MEMORY_* above in memremap.h
// @flags: PGMAP_* flags to specify defailed behavior
// @vmemmap_shift: structural definition of how the vmemmap page metadata
// is populated, specifically the metadata page order.
// A zero value (default) uses base pages as the vmemmap metadata
// representation. A bigger value will set up compound struct pages
// of the requested order value.
// @ops: method table
// @owner: an opaque pointer identifying the entity that manages this
// instance.  Used by various helpers to make sure that no
// foreign ZONE_DEVICE memory is accessed.
// @nr_range: number of ranges to be mapped
// @range: range to be mapped when nr_range == 1
// @ranges: array of ranges to be mapped when nr_range > 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pagemap {
    pub altmap: vmem_altmap,
    pub ref: percpu_ref,
    pub done: completion,
    pub type: memory_type,
    pub flags: c_uint,
    pub vmemmap_shift: c_ulong,
    pub ops: *const dev_pagemap_ops,
    pub owner: *mut c_void,
    pub nr_range: c_int,
    pub range: range,
    pub ranges): DECLARE_FLEX_ARRAY(struct range,,
}

extern "C" {
    pub fn folio_is_device_coherent(_arg: page_folio(page)) -> return;
}
extern "C" {
    pub fn folio_is_fsdax(_arg: page_folio(page)) -> return;
}

extern "C" {
    pub fn memunmap_pages(pgmap: *mut dev_pagemap);
}
extern "C" {
    pub fn devm_memunmap_pages(dev: *mut device, pgmap: *mut dev_pagemap);
}
extern "C" {
    pub fn pgmap_pfn_valid(pgmap: *mut dev_pagemap, pfn: c_ulong) -> bool;
}
extern "C" {
    pub fn memremap_compat_align() -> c_ulong;
}

//
// Fail attempts to call devm_memremap_pages() without
// ZONE_DEVICE support enabled, this requires callers to fall
// back to plain devm_memremap() based on config
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}
// when memremap_pages() is disabled all archs can remap a single page

