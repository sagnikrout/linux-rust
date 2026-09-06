//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/grant_table.h
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
// grant_table.h
//
// Interface for granting foreign access to page frames, and receiving
// page-ownership transfers.
//
// Copyright (c) 2004, K A Fraser
//

//
// GRANT TABLE REPRESENTATION
//
// Some rough guidelines on accessing and updating grant-table entries
// in a concurrency-safe manner. For more information, Linux contains a
// reference implementation for guest OSes (drivers/xen/grant_table.c, see
// http://git.kernel.org/?p=linux/kernel/git/torvalds/linux.git;a=blob;f=drivers/xen/grant-table.c;hb=HEAD
//
// NB. WMB is a no-op on current-generation x86 processors. However, a
// compiler barrier will still be required.
//
// Introducing a valid entry into the grant table:
// 1. Write ent->domid.
// 2. Write ent->frame:
// GTF_permit_access:   Frame to which access is permitted.
// GTF_accept_transfer: Pseudo-phys frame slot being filled by new
// frame, or zero if none.
// 3. Write memory barrier (WMB).
// 4. Write ent->flags, inc. valid type.
//
// Invalidating an unused GTF_permit_access entry:
// 1. flags = ent->flags.
// 2. Observe that !(flags & (GTF_reading|GTF_writing)).
// 3. Check result of SMP-safe CMPXCHG(&ent->flags, flags, 0).
// NB. No need for WMB as reuse of entry is control-dependent on success of
// step 3, and all architectures guarantee ordering of ctrl-dep writes.
//
// Invalidating an in-use GTF_permit_access entry:
// This cannot be done directly. Request assistance from the domain controller
// which can set a timeout on the use of a grant entry and take necessary
// action. (NB. This is not yet implemented!).
//
// Invalidating an unused GTF_accept_transfer entry:
// 1. flags = ent->flags.
// 2. Observe that !(flags & GTF_transfer_committed). [*]
// 3. Check result of SMP-safe CMPXCHG(&ent->flags, flags, 0).
// NB. No need for WMB as reuse of entry is control-dependent on success of
// step 3, and all architectures guarantee ordering of ctrl-dep writes.
// [*] If GTF_transfer_committed is set then the grant entry is 'committed'.
// The guest must /not/ modify the grant entry until the address of the
// transferred frame is written. It is safe for the guest to spin waiting
// for this to occur (detect by observing GTF_transfer_completed in
// ent->flags).
//
// Invalidating a committed GTF_accept_transfer entry:
// 1. Wait for (ent->flags & GTF_transfer_completed).
//
// Changing a GTF_permit_access from writable to read-only:
// Use SMP-safe CMPXCHG to set GTF_readonly, while checking !GTF_writing.
//
// Changing a GTF_permit_access from read-only to writable:
// Use SMP-safe bit-setting instruction.
//
// Reference to a grant entry in a specified domain's grant table.
//
pub type grant_ref_t = u32;
//
// A grant table comprises a packed array of grant entries in one or more
// page frames shared between Xen and a guest.
// [XEN]: This field is written by Xen and read by the sharing guest.
// [GST]: This field is written by the guest and read by Xen.
//
// Version 1 of the grant table entry structure is maintained largely for
// backwards compatibility.  New guests are recommended to support using
// version 2 to overcome version 1 limitations, but to default to version 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_v1 {
// GTF_xxx: various type and flag information.  [XEN,GST]
    pub flags: u16,
// The domain being granted foreign privileges. [GST]
    pub domid: domid_t,
//
// GTF_permit_access: GFN that @domid is allowed to map and access. [GST]
// GTF_accept_transfer: GFN that @domid is allowed to transfer into. [GST]
// GTF_transfer_completed: MFN whose ownership transferred by @domid
// (non-translated guests only). [XEN]
//
    pub frame: u32,
}

// The first few grant table entries will be preserved across grant table
// version changes and may be pre-populated at domain creation by tools.
//
pub const GNTTAB_NR_RESERVED_ENTRIES: c_int = 8;
pub const GNTTAB_RESERVED_CONSOLE: c_int = 0;
pub const GNTTAB_RESERVED_XENSTORE: c_int = 1;
//
// Type of grant entry.
// GTF_invalid: This grant entry grants no privileges.
// GTF_permit_access: Allow @domid to map/access @frame.
// GTF_accept_transfer: Allow @domid to transfer ownership of one page frame
// to this guest. Xen writes the page number to @frame.
// GTF_transitive: Allow @domid to transitively access a subrange of
// @trans_grant in @trans_domid.  No mappings are allowed.
//

//
// Subflags for GTF_permit_access and GTF_transitive.
// GTF_readonly: Restrict @domid to read-only mappings and accesses. [GST]
// GTF_reading: Grant entry is currently mapped for reading by @domid. [XEN]
// GTF_writing: Grant entry is currently mapped for writing by @domid. [XEN]
// Further subflags for GTF_permit_access only.
// GTF_PAT, GTF_PWT, GTF_PCD: (x86) cache attribute flags to be used for
// mappings of the grant [GST]
// GTF_sub_page: Grant access to only a subrange of the page.  @domid
// will only be allowed to copy from the grant, and not
// map it. [GST]
//

//
// Subflags for GTF_accept_transfer:
// GTF_transfer_committed: Xen sets this flag to indicate that it is committed
// to transferring ownership of a page frame. When a guest sees this flag
// it must /not/ modify the grant entry until GTF_transfer_completed is
// set by Xen.
// GTF_transfer_completed: It is safe for the guest to spin-wait on this flag
// after reading GTF_transfer_committed. Xen will always write the frame
// address, followed by ORing this flag, in a timely manner.
//

//
// Version 2 grant table entries.  These fulfil the same role as
// version 1 entries, but can represent more complicated operations.
// Any given domain will have either a version 1 or a version 2 table,
// and every entry in the table will be the same version.
//
// The interface by which domains use grant references does not depend
// on the grant table version in use by the other domain.
//
// Version 1 and version 2 grant entries share a common prefix.  The
// fields of the prefix are documented as part of struct
// grant_entry_v1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_entry_header {
    pub flags: u16,
    pub domid: domid_t,
}

//
// Version 2 of the grant entry structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union grant_entry_v2 {
    pub hdr: grant_entry_header,
//
// This member is used for V1-style full page grants, where either:
//
// -- hdr.type is GTF_accept_transfer, or
// -- hdr.type is GTF_permit_access and GTF_sub_page is not set.
//
// In that case, the frame field has the same semantics as the
// field of the same name in the V1 entry structure.
//
    pub hdr: grant_entry_header,
    pub pad0: u32,
    pub frame: u64,
    pub full_page: },
//
// If the grant type is GTF_grant_access and GTF_sub_page is set,
// @domid is allowed to access bytes [@page_off,@page_off+@length)
// in frame @frame.
//
    pub hdr: grant_entry_header,
    pub page_off: u16,
    pub length: u16,
    pub frame: u64,
    pub sub_page: },
//
// If the grant is GTF_transitive, @domid is allowed to use the
// grant @gref in domain @trans_domid, as if it was the local
// domain.  Obviously, the transitive access must be compatible
// with the original grant.
//
// The current version of Xen does not allow transitive grants
// to be mapped.
//
    pub hdr: grant_entry_header,
    pub trans_domid: domid_t,
    pub pad0: u16,
    pub gref: grant_ref_t,
    pub transitive: },
    pub /: *mut *mut uint32_t __spacer[4]; / Pad to a power of two,
}

pub type grant_status_t = u16;
//
// GRANT TABLE QUERIES AND USES
//
pub const GNTTABOP_map_grant_ref: c_int = 0;
pub const GNTTABOP_unmap_grant_ref: c_int = 1;
pub const GNTTABOP_setup_table: c_int = 2;
pub const GNTTABOP_dump_table: c_int = 3;
pub const GNTTABOP_transfer: c_int = 4;
pub const GNTTABOP_copy: c_int = 5;
pub const GNTTABOP_query_size: c_int = 6;
pub const GNTTABOP_unmap_and_replace: c_int = 7;
pub const GNTTABOP_set_version: c_int = 8;
pub const GNTTABOP_get_status_frames: c_int = 9;
pub const GNTTABOP_get_version: c_int = 10;
pub const GNTTABOP_swap_grant_ref: c_int = 11;
pub const GNTTABOP_cache_flush: c_int = 12;
// ` }
//
// Handle to track a mapping created via a grant reference.
//
pub type grant_handle_t = u32;
//
// GNTTABOP_map_grant_ref: Map the grant entry (<dom>,<ref>) for access
// by devices and/or host CPUs. If successful, <handle> is a tracking number
// that must be presented later to destroy the mapping(s). On error, <status>
// is a negative status code.
// NOTES:
// 1. If GNTMAP_device_map is specified then <dev_bus_addr> is the address
// via which I/O devices may access the granted frame.
// 2. If GNTMAP_host_map is specified then a mapping will be added at
// either a host virtual address in the current address space, or at
// a PTE at the specified machine address.  The type of mapping to
// perform is selected through the GNTMAP_contains_pte flag, and the
// address is specified in <host_addr>.
// 3. Mappings should only be destroyed via GNTTABOP_unmap_grant_ref. If a
// host mapping is destroyed by other means then it is *NOT* guaranteed
// to be accounted to the correct grant reference!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_map_grant_ref {
// IN parameters.
    pub host_addr: u64,
    pub /: *mut *mut *mut uint32_t flags; / GNTMAP_,
    pub ref: grant_ref_t,
    pub dom: domid_t,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
    pub handle: grant_handle_t,
    pub dev_bus_addr: u64,
}

//
// GNTTABOP_unmap_grant_ref: Destroy one or more grant-reference mappings
// tracked by <handle>. If <host_addr> or <dev_bus_addr> is zero, that
// field is ignored. If non-zero, they must refer to a device/host mapping
// that is tracked by <handle>
// NOTES:
// 1. The call may fail in an undefined manner if either mapping is not
// tracked by <handle>.
// 3. After executing a batch of unmaps, it is guaranteed that no stale
// mappings will remain in the device or host TLBs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_unmap_grant_ref {
// IN parameters.
    pub host_addr: u64,
    pub dev_bus_addr: u64,
    pub handle: grant_handle_t,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
}

//
// GNTTABOP_setup_table: Set up a grant table for <dom> comprising at least
// <nr_frames> pages. The frame addresses are written to the <frame_list>.
// Only <nr_frames> addresses are written, even if the table is larger.
// NOTES:
// 1. <dom> may be specified as DOMID_SELF.
// 2. Only a sufficiently-privileged domain may specify <dom> != DOMID_SELF.
// 3. Xen may not support more than a single grant-table page per domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_setup_table {
// IN parameters.
    pub dom: domid_t,
    pub nr_frames: u32,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
    pub frame_list: GUEST_HANDLE(xen_pfn_t),
}

//
// GNTTABOP_dump_table: Dump the contents of the grant table to the
// xen console. Debugging use only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_dump_table {
// IN parameters.
    pub dom: domid_t,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
}

//
// GNTTABOP_transfer: Transfer <frame> to a foreign domain. The foreign domain
// has previously registered its interest in the transfer via <domid, ref>.
//
// Note that, even if the transfer fails, the specified page no longer belongs
// to the calling domain *unless* the error is GNTST_bad_page.
//
// Note further that only PV guests can use this operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_transfer {
// IN parameters.
    pub mfn: xen_pfn_t,
    pub domid: domid_t,
    pub ref: grant_ref_t,
// OUT parameters.
    pub status: i16,
}

//
// GNTTABOP_copy: Hypervisor based copy
// source and destinations can be eithers MFNs or, for foreign domains,
// grant references. the foreign domain has to grant read/write access
// in its grant table.
//
// The flags specify what type source and destinations are (either MFN
// or grant reference).
//
// Note that this can also be used to copy data between two domains
// via a third party if the source and destination domains had previously
// grant appropriate access to their pages to the third party.
//
// source_offset specifies an offset in the source frame, dest_offset
// the offset in the target frame and  len specifies the number of
// bytes to be copied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_copy {
// IN parameters.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_copy_ptr {
    pub ref: grant_ref_t,
    pub gmfn: xen_pfn_t,
    pub u: },
    pub domid: domid_t,
    pub offset: u16,
    pub dest: } source,,
    pub len: u16,
    pub /: *mut *mut *mut uint16_t flags; / GNTCOPY_,
// OUT parameters.
    pub status: i16,
}

//
// GNTTABOP_query_size: Query the current and maximum sizes of the shared
// grant table.
// NOTES:
// 1. <dom> may be specified as DOMID_SELF.
// 2. Only a sufficiently-privileged domain may specify <dom> != DOMID_SELF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_query_size {
// IN parameters.
    pub dom: domid_t,
// OUT parameters.
    pub nr_frames: u32,
    pub max_nr_frames: u32,
    pub /: *mut *mut *mut int16_t status; / GNTST_,
}

//
// GNTTABOP_unmap_and_replace: Destroy one or more grant-reference mappings
// tracked by <handle> but atomically replace the page table entry with one
// pointing to the machine address under <new_addr>.  <new_addr> will be
// redirected to the null entry.
// NOTES:
// 1. The call may fail in an undefined manner if either mapping is not
// tracked by <handle>.
// 2. After executing a batch of unmaps, it is guaranteed that no stale
// mappings will remain in the device or host TLBs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_unmap_and_replace {
// IN parameters.
    pub host_addr: u64,
    pub new_addr: u64,
    pub handle: grant_handle_t,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
}

//
// GNTTABOP_set_version: Request a particular version of the grant
// table shared table structure.  This operation may be used to toggle
// between different versions, but must be performed while no grants
// are active.  The only defined versions are 1 and 2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_set_version {
// IN/OUT parameters
    pub version: u32,
}

//
// GNTTABOP_get_status_frames: Get the list of frames used to store grant
// status for <dom>. In grant format version 2, the status is separated
// from the other shared grant fields to allow more efficient synchronization
// using barriers instead of atomic cmpexch operations.
// <nr_frames> specify the size of vector <frame_list>.
// The frame addresses are returned in the <frame_list>.
// Only <nr_frames> addresses are returned, even if the table is larger.
// NOTES:
// 1. <dom> may be specified as DOMID_SELF.
// 2. Only a sufficiently-privileged domain may specify <dom> != DOMID_SELF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_get_status_frames {
// IN parameters.
    pub nr_frames: u32,
    pub dom: domid_t,
// OUT parameters.
    pub /: *mut *mut *mut int16_t status; / GNTST_,
    pub frame_list: GUEST_HANDLE(uint64_t),
}

//
// GNTTABOP_get_version: Get the grant table version which is in
// effect for domain <dom>.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_get_version {
// IN parameters
    pub dom: domid_t,
    pub pad: u16,
// OUT parameters
    pub version: u32,
}

//
// GNTTABOP_swap_grant_ref: Swap the contents of two grant entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_swap_grant_ref {
// IN parameters
    pub ref_a: grant_ref_t,
    pub ref_b: grant_ref_t,
// OUT parameters
    pub /: *mut *mut *mut int16_t status; / GNTST_,
}

//
// Issue one or more cache maintenance operations on a portion of a
// page granted to the calling domain by a foreign domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnttab_cache_flush {
    pub dev_bus_addr: u64,
    pub ref: grant_ref_t,
    pub a: },
    pub /: *mut *mut uint16_t offset; / offset from start of grant,
    pub /: *mut *mut uint16_t length; / size within the grant,

    pub op: u32,
}

//
// Bitfield values for gnttab_map_grant_ref.flags.
//
// Map the grant entry for access by I/O devices.

// Map the grant entry for access by host CPUs.

// Accesses to the granted frame will be restricted to read-only access.

//
// GNTMAP_host_map subflag:
// 0 => The host mapping is usable only by the guest OS.
// 1 => The host mapping is usable by guest OS + current application.
//

//
// GNTMAP_contains_pte subflag:
// 0 => This map request contains a host virtual address.
// 1 => This map request contains the machine addess of the PTE to update.
//

//
// Bits to be placed in guest kernel available PTE bits (architecture
// dependent; only supported when XENFEAT_gnttab_map_avail_bits is set).
//

//
// Values for error status returns. All errors are -ve.
//

