//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/xen-blkback/common.h
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
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// This is the maximum number of segments that would be allowed in indirect
// requests. This value will also be passed to the frontend.
//
pub const MAX_INDIRECT_SEGMENTS: c_int = 256;
//
// Xen use 4K pages. The guest may use different page size (4K or 64K)
// Number of Xen pages per segment
//

// Not a real protocol.  Used to generate ring structs which contain
// the elements common to all protocols only.  This way we get a
// compiler-checkable way to use common struct elements, so we can
// avoid using switch(protocol) in a number of places.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_common_request {
    pub dummy: c_char,
}

// i386 protocol version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_32_request_rw {
    pub /: *mut *mut uint8_t nr_segments; / number of segments,
    pub /: *mut *mut blkif_vdev_t handle; / only for read/write requests,
    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub /: *mut *mut blkif_sector_t sector_number;/ start sector idx on disk (r/w only),
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_32_request_discard {
    pub /: *mut *mut uint8_t flag; / BLKIF_DISCARD_SECURE or zero,
    pub /: *mut *mut blkif_vdev_t _pad1; / was "handle" for read/write requests,
    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub /: *mut *mut blkif_sector_t sector_number;/ start sector idx on disk (r/w only),
    pub nr_sectors: u64,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_32_request_other {
    pub _pad1: u8,
    pub _pad2: blkif_vdev_t,
    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_32_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad1: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
//
// The maximum number of indirect segments (and pages) that will
// be used is determined by MAX_INDIRECT_SEGMENTS, this value
// is also exported to the guest (via xenstore
// feature-max-indirect-segments entry), so the frontend knows how
// many indirect segments the backend supports.
//
    pub /: *mut *mut uint64_t _pad2; / make it 64 byte aligned,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_32_request {
    pub /: *mut *mut uint8_t operation; / BLKIF_OP_???,
    pub rw: blkif_x86_32_request_rw,
    pub discard: blkif_x86_32_request_discard,
    pub other: blkif_x86_32_request_other,
    pub indirect: blkif_x86_32_request_indirect,
    pub u: },
    pub __attribute__((__packed__)): },
// x86_64 protocol version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_64_request_rw {
    pub /: *mut *mut uint8_t nr_segments; / number of segments,
    pub /: *mut *mut blkif_vdev_t handle; / only for read/write requests,
    pub /: *mut *mut uint32_t _pad1; / offsetof(blkif_request..,u.rw.id)==8,
    pub id: u64,
    pub /: *mut *mut blkif_sector_t sector_number;/ start sector idx on disk (r/w only),
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_64_request_discard {
    pub /: *mut *mut uint8_t flag; / BLKIF_DISCARD_SECURE or zero,
    pub /: *mut *mut blkif_vdev_t _pad1; / was "handle" for read/write requests,
    pub /: *mut *mut uint32_t _pad2; / offsetof(blkif_..,u.discard.id)==8,
    pub id: u64,
    pub /: *mut *mut blkif_sector_t sector_number;/ start sector idx on disk (r/w only),
    pub nr_sectors: u64,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_64_request_other {
    pub _pad1: u8,
    pub _pad2: blkif_vdev_t,
    pub /: *mut *mut uint32_t _pad3; / offsetof(blkif_..,u.discard.id)==8,
    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_64_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,
    pub /: *mut *mut uint32_t _pad1; / offsetof(blkif_..,u.indirect.id)==8,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad2: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
//
// The maximum number of indirect segments (and pages) that will
// be used is determined by MAX_INDIRECT_SEGMENTS, this value
// is also exported to the guest (via xenstore
// feature-max-indirect-segments entry), so the frontend knows how
// many indirect segments the backend supports.
//
    pub /: *mut *mut uint32_t _pad3; / make it 64 byte aligned,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_x86_64_request {
    pub /: *mut *mut uint8_t operation; / BLKIF_OP_???,
    pub rw: blkif_x86_64_request_rw,
    pub discard: blkif_x86_64_request_discard,
    pub other: blkif_x86_64_request_other,
    pub indirect: blkif_x86_64_request_indirect,
    pub u: },
    pub __attribute__((__packed__)): },
    pub blkif_response): struct,
    pub __packed): blkif_response,
    pub blkif_response): struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub union blkif_back_rings {
    pub native: blkif_back_ring,
    pub common: blkif_common_back_ring,
    pub x86_32: blkif_x86_32_back_ring,
    pub x86_64: blkif_x86_64_back_ring,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkif_protocol {
    BLKIF_PROTOCOL_NATIVE = 1,
    BLKIF_PROTOCOL_X86_32 = 2,
    BLKIF_PROTOCOL_X86_64 = 3,
}

//
// Default protocol if the frontend doesn't specify one.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_vbd {
// What the domain refers to this vbd as.
    pub handle: blkif_vdev_t,
// Non-zero -> read-only
    pub readonly: c_uchar,
// VDISK_xxx
    pub type: c_uchar,
// phys device that this vbd maps to.
    pub pdevice: u32,
    pub bdev_file: *mut file,
// Cached size parameter.
    pub size: sector_t,
    pub flush_support:1: c_uint,
    pub discard_secure:1: c_uint,
// Connect-time cached feature_persistent parameter value
    pub feature_gnt_persistent_parm:1: c_uint,
// Persistent grants feature negotiation result
    pub feature_gnt_persistent:1: c_uint,
    pub overflow_max_grants:1: c_uint,
}

// Number of requests that we can fit in a ring
pub const XEN_BLKIF_REQS_PER_PAGE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct persistent_gnt {
    pub page: *mut page,
    pub gnt: grant_ref_t,
    pub handle: grant_handle_t,
    pub last_used: c_ulong,
    pub active: bool,
    pub node: rb_node,
    pub remove_node: list_head,
}

// Per-ring information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_blkif_ring {
// Physical parameters of the comms window.
    pub irq: c_uint,
    pub blk_rings: blkif_back_rings,
    pub blk_ring: *mut c_void,
// Private fields.
    pub blk_ring_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub inflight: core::sync::atomic::AtomicI32,
    pub active: bool,
// One thread per blkif ring.
    pub xenblkd: *mut task_struct,
    pub waiting_reqs: c_uint,
// List of all 'pending_req' available
    pub pending_free: list_head,
// And its spinlock.
    pub pending_free_lock: spinlock_t,
    pub pending_free_wq: wait_queue_head_t,
// Tree to store persistent grants.
    pub persistent_gnts: rb_root,
    pub persistent_gnt_c: c_uint,
    pub persistent_gnt_in_use: core::sync::atomic::AtomicI32,
    pub next_lru: c_ulong,
// Statistics.
    pub st_print: c_ulong,
    pub st_rd_req: c_ulonglong,
    pub st_wr_req: c_ulonglong,
    pub st_oo_req: c_ulonglong,
    pub st_f_req: c_ulonglong,
    pub st_ds_req: c_ulonglong,
    pub st_rd_sect: c_ulonglong,
    pub st_wr_sect: c_ulonglong,
// Used by the kworker that offload work from the persistent purge.
    pub persistent_purge_list: list_head,
    pub persistent_purge_work: work_struct,
// Buffer of free pages to map grant refs.
    pub free_pages: gnttab_page_cache,
    pub free_work: work_struct,
// Thread shutdown wait queue.
    pub shutdown_wq: wait_queue_head_t,
    pub blkif: *mut xen_blkif,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_blkif {
// Unique identifier for this interface.
    pub domid: domid_t,
    pub handle: c_uint,
// Comms information.
    pub blk_protocol: blkif_protocol,
// The VBD attached to this interface.
    pub vbd: xen_vbd,
// Back pointer to the backend_info.
    pub be: *mut backend_info,
    pub refcnt: core::sync::atomic::AtomicI32,
// for barrier (drain) requests
    pub drain_complete: completion,
    pub drain: core::sync::atomic::AtomicI32,
    pub free_work: work_struct,
    pub nr_ring_pages: c_uint,
    pub multi_ref: bool,
// All rings for this device.
    pub rings: *mut xen_blkif_ring,
    pub nr_rings: c_uint,
    pub buffer_squeeze_end: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg_buf {
    pub offset: c_ulong,
    pub nsec: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_page {
    pub page: *mut page,
    pub persistent_gnt: *mut persistent_gnt,
    pub handle: grant_handle_t,
    pub gref: grant_ref_t,
}

//
// Each outstanding request that we've passed to the lower device layers has a
// 'pending_req' allocated to it. Each buffer_head that completes decrements
// the pendcnt towards zero. When it hits zero, the specified domain has a
// response queued for it, with the saved 'id' passed back.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_req {
    pub ring: *mut xen_blkif_ring,
    pub id: u64,
    pub nr_segs: c_int,
    pub pendcnt: core::sync::atomic::AtomicI32,
    pub operation: c_ushort,
    pub status: c_int,
    pub free_list: list_head,
    pub segments: [*mut grant_page; MAX_INDIRECT_SEGMENTS],
// Indirect descriptors
    pub indirect_pages: [*mut grant_page; MAX_INDIRECT_PAGES],
    pub seg: [seg_buf; MAX_INDIRECT_SEGMENTS],
    pub biolist: [*mut bio; MAX_INDIRECT_SEGMENTS],
    pub unmap: [gnttab_unmap_grant_ref; MAX_INDIRECT_SEGMENTS],
    pub unmap_pages: [*mut page; MAX_INDIRECT_SEGMENTS],
    pub gnttab_unmap_data: gntab_unmap_queue_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_req {
    pub dev: c_ushort,
    pub nr_sects: blkif_sector_t,
    pub bdev: *mut block_device,
    pub sector_number: blkif_sector_t,
}

extern "C" {
    pub fn xen_blkif_interface_init() -> c_int;
}
extern "C" {
    pub fn xen_blkif_interface_fini();
}
extern "C" {
    pub fn xen_blkif_xenbus_init() -> c_int;
}
extern "C" {
    pub fn xen_blkif_xenbus_fini();
}
extern "C" {
    pub fn xen_blkif_be_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn xen_blkif_schedule(arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xen_blkbk_free_caches(ring: *mut xen_blkif_ring);
}
extern "C" {
    pub fn xen_blkbk_unmap_purged_grants(work: *mut work_struct);
}
