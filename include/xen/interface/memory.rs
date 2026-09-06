//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/memory.h
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


// SPDX-License-Identifier: MIT
//
// memory.h
//
// Memory reservation and information.
//
// Copyright (c) 2005, Keir Fraser <keir@xensource.com>
//

//
// Increase or decrease the specified domain's memory reservation. Returns a
// -ve errcode on failure, or the # extents successfully allocated or freed.
// arg == addr of struct xen_memory_reservation.
//
pub const XENMEM_increase_reservation: c_int = 0;
pub const XENMEM_decrease_reservation: c_int = 1;
pub const XENMEM_populate_physmap: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_memory_reservation {
//
// XENMEM_increase_reservation:
// OUT: MFN (*not* GMFN) bases of extents that were allocated
// XENMEM_decrease_reservation:
// IN:  GMFN bases of extents to free
// XENMEM_populate_physmap:
// IN:  GPFN bases of extents to populate with memory
// OUT: GMFN bases of extents that were allocated
// (NB. This command also updates the mach_to_phys translation table)
//
    pub extent_start: GUEST_HANDLE(xen_pfn_t),
// Number of extents, and size/alignment of each (2^extent_order pages).
    pub nr_extents: xen_ulong_t,
    pub extent_order: c_uint,
//
// Maximum # bits addressable by the user of the allocated region (e.g.,
// I/O devices often have a 32-bit limitation even in 64-bit systems). If
// zero then the user has no addressing restriction.
// This field is not used by XENMEM_decrease_reservation.
//
    pub address_bits: c_uint,
//
// Domain whose reservation is being changed.
// Unprivileged domains can specify only DOMID_SELF.
//
    pub domid: domid_t,
}

//
// An atomic exchange of memory pages. If return code is zero then
// @out.extent_list provides GMFNs of the newly-allocated memory.
// Returns zero on complete success, otherwise a negative error code.
// On complete success then always @nr_exchanged == @in.nr_extents.
// On partial success @nr_exchanged indicates how much work was done.
//
pub const XENMEM_exchange: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_memory_exchange {
//
// [IN] Details of memory extents to be exchanged (GMFN bases).
// Note that @in.address_bits is ignored and unused.
//
    pub in: xen_memory_reservation,
//
// [IN/OUT] Details of new memory extents.
// We require that:
// 1. @in.domid == @out.domid
// 2. @in.nr_extents  << @in.extent_order ==
// @out.nr_extents << @out.extent_order
// 3. @in.extent_start and @out.extent_start lists must not overlap
// 4. @out.extent_start lists GPFN bases to be populated
// 5. @out.extent_start is overwritten with allocated GMFN bases
//
    pub out: xen_memory_reservation,
//
// [OUT] Number of input extents that were successfully exchanged:
// 1. The first @nr_exchanged input extents were successfully
// deallocated.
// 2. The corresponding first entries in the output extent list correctly
// indicate the GMFNs that were successfully exchanged.
// 3. All other input and output extents are untouched.
// 4. If not all input exents are exchanged then the return code of this
// command will be non-zero.
// 5. THIS FIELD MUST BE INITIALISED TO ZERO BY THE CALLER!
//
    pub nr_exchanged: xen_ulong_t,
}

//
// Returns the maximum machine frame number of mapped RAM in this system.
// This command always succeeds (it never returns an error code).
// arg == NULL.
//
pub const XENMEM_maximum_ram_page: c_int = 2;
//
// Returns the current or maximum memory reservation, in pages, of the
// specified domain (may be DOMID_SELF). Returns -ve errcode on failure.
// arg == addr of domid_t.
//
pub const XENMEM_current_reservation: c_int = 3;
pub const XENMEM_maximum_reservation: c_int = 4;
//
// Returns a list of MFN bases of 2MB extents comprising the machine_to_phys
// mapping table. Architectures which do not have a m2p table do not implement
// this command.
// arg == addr of xen_machphys_mfn_list_t.
//
pub const XENMEM_machphys_mfn_list: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_machphys_mfn_list {
//
// Size of the 'extent_start' array. Fewer entries will be filled if the
// machphys table is smaller than max_extents * 2MB.
//
    pub max_extents: c_uint,
//
// Pointer to buffer to fill with list of extent starts. If there are
// any large discontiguities in the machine address space, 2MB gaps in
// the machphys table will be represented by an MFN base of zero.
//
    pub extent_start: GUEST_HANDLE(xen_pfn_t),
//
// Number of extents written to the above array. This will be smaller
// than 'max_extents' if the machphys table is smaller than max_e * 2MB.
//
    pub nr_extents: c_uint,
}

//
// Returns the location in virtual address space of the machine_to_phys
// mapping table. Architectures which do not have a m2p table, or which do not
// map it by default into guest address space, do not implement this command.
// arg == addr of xen_machphys_mapping_t.
//
pub const XENMEM_machphys_mapping: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_machphys_mapping {
    pub /: *mut *mut xen_ulong_t v_start, v_end; / Start and end virtual addresses.,
    pub /: *mut *mut xen_ulong_t max_mfn; / Maximum MFN that can be looked up.,
}

// XENMEM_add_to_physmap_range only.
//

//
// Sets the GPFN at which a particular page appears in the specified guest's
// pseudophysical address space.
// arg == addr of xen_add_to_physmap_t.
//
pub const XENMEM_add_to_physmap: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_add_to_physmap {
// Which domain to change the mapping for.
    pub domid: domid_t,
// Number of pages to go through for gmfn_range
    pub size: u16,
// Source mapping space.
    pub space: c_uint,
// Index into source mapping space.
    pub idx: xen_ulong_t,
// GPFN where the source mapping page should appear.
    pub gpfn: xen_pfn_t,
}

// REMOVED
// #define XENMEM_translate_gpfn_list  8
pub const XENMEM_add_to_physmap_range: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_add_to_physmap_range {
// IN
// Which domain to change the mapping for.
    pub domid: domid_t,
    pub /: *mut *mut uint16_t space; / => enum phys_map_space,
// Number of pages to go through
    pub size: u16,
    pub /: *mut *mut domid_t foreign_domid; / IFF gmfn_foreign,
// Indexes into space being mapped.
    pub idxs: GUEST_HANDLE(xen_ulong_t),
// GPFN in domid where the source mapping page should appear.
    pub gpfns: GUEST_HANDLE(xen_pfn_t),
// OUT
// Per index error code.
    pub errs: GUEST_HANDLE(int),
}

//
// Returns the pseudo-physical memory map as it was when the domain
// was started (specified by XENMEM_set_memory_map).
// arg == addr of struct xen_memory_map.
//
pub const XENMEM_memory_map: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_memory_map {
//
// On call the number of entries which can be stored in buffer. On
// return the number of entries which have been stored in
// buffer.
//
    pub nr_entries: c_uint,
//
// Entries in the buffer are in the same format as returned by the
// BIOS INT 0x15 EAX=0xE820 call.
//
    pub buffer: GUEST_HANDLE(void),
}

//
// Returns the real physical memory map. Passes the same structure as
// XENMEM_memory_map.
// arg == addr of struct xen_memory_map.
//
pub const XENMEM_machine_memory_map: c_int = 10;
//
// Unmaps the page appearing at a particular GPFN from the specified guest's
// pseudophysical address space.
// arg == addr of xen_remove_from_physmap_t.
//
pub const XENMEM_remove_from_physmap: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_remove_from_physmap {
// Which domain to change the mapping for.
    pub domid: domid_t,
// GPFN of the current mapping of the page.
    pub gpfn: xen_pfn_t,
}

//
// Get the pages for a particular guest resource, so that they can be
// mapped directly by a tools domain.
//
pub const XENMEM_acquire_resource: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_mem_acquire_resource {
// IN - The domain whose resource is to be mapped
    pub domid: domid_t,
// IN - the type of resource
    pub type: u16,
pub const XENMEM_resource_ioreq_server: c_int = 0;
pub const XENMEM_resource_grant_table: c_int = 1;
//
// IN - a type-specific resource identifier, which must be zero
// unless stated otherwise.
//
// type == XENMEM_resource_ioreq_server -> id == ioreq server id
// type == XENMEM_resource_grant_table -> id defined below
//
    pub id: u32,
pub const XENMEM_resource_grant_table_id_shared: c_int = 0;
pub const XENMEM_resource_grant_table_id_status: c_int = 1;
// IN/OUT - As an IN parameter number of frames of the resource
// to be mapped. However, if the specified value is 0 and
// frame_list is NULL then this field will be set to the
// maximum value supported by the implementation on return.
//
    pub nr_frames: u32,
//
// OUT - Must be zero on entry. On return this may contain a bitwise
// OR of the following values.
//
    pub flags: u32,
// The resource pages have been assigned to the calling domain
pub const _XENMEM_rsrc_acq_caller_owned: c_int = 0;

//
// IN - the index of the initial frame to be mapped. This parameter
// is ignored if nr_frames is 0.
//
    pub frame: u64,
pub const XENMEM_resource_ioreq_server_frame_bufioreq: c_int = 0;

//
// IN/OUT - If the tools domain is PV then, upon return, frame_list
// will be populated with the MFNs of the resource.
// If the tools domain is HVM then it is expected that, on
// entry, frame_list will be populated with a list of GFNs
// that will be mapped to the MFNs of the resource.
// If -EIO is returned then the frame_list has only been
// partially mapped and it is up to the caller to unmap all
// the GFNs.
// This parameter may be NULL if nr_frames is 0.
//
    pub frame_list: GUEST_HANDLE(xen_pfn_t),
}
