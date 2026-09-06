//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/blkif.h
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
// blkif.h
//
// Unified block-device I/O interface for Xen guest OSes.
//
// Copyright (c) 2003-2004, Keir Fraser
//

//
// Front->back notifications: When enqueuing a new request, sending a
// notification can be made conditional on req_event (i.e., the generic
// hold-off mechanism provided by the ring macros). Backends must set
// req_event appropriately (e.g., using RING_FINAL_CHECK_FOR_REQUESTS()).
//
// Back->front notifications: When enqueuing a new response, sending a
// notification can be made conditional on rsp_event (i.e., the generic
// hold-off mechanism provided by the ring macros). Frontends must set
// rsp_event appropriately (e.g., using RING_FINAL_CHECK_FOR_RESPONSES()).
//
pub type blkif_vdev_t = u16;
pub type blkif_sector_t = u64;
//
// Multiple hardware queues/rings:
// If supported, the backend will write the key "multi-queue-max-queues" to
// the directory for that vbd, and set its value to the maximum supported
// number of queues.
// Frontends that are aware of this feature and wish to use it can write the
// key "multi-queue-num-queues" with the number they wish to use, which must be
// greater than zero, and no more than the value reported by the backend in
// "multi-queue-max-queues".
//
// For frontends requesting just one queue, the usual event-channel and
// ring-ref keys are written as before, simplifying the backend processing
// to avoid distinguishing between a frontend that doesn't understand the
// multi-queue feature, and one that does, but requested only one queue.
//
// Frontends requesting two or more queues must not write the toplevel
// event-channel and ring-ref keys, instead writing those keys under sub-keys
// having the name "queue-N" where N is the integer ID of the queue/ring for
// which those keys belong. Queues are indexed from zero.
// For example, a frontend with two queues must write the following set of
// queue-related keys:
//
// /local/domain/1/device/vbd/0/multi-queue-num-queues = "2"
// /local/domain/1/device/vbd/0/queue-0 = ""
// /local/domain/1/device/vbd/0/queue-0/ring-ref = "<ring-ref#0>"
// /local/domain/1/device/vbd/0/queue-0/event-channel = "<evtchn#0>"
// /local/domain/1/device/vbd/0/queue-1 = ""
// /local/domain/1/device/vbd/0/queue-1/ring-ref = "<ring-ref#1>"
// /local/domain/1/device/vbd/0/queue-1/event-channel = "<evtchn#1>"
//
// It is also possible to use multiple queues/rings together with
// feature multi-page ring buffer.
// For example, a frontend requests two queues/rings and the size of each ring
// buffer is two pages must write the following set of related keys:
//
// /local/domain/1/device/vbd/0/multi-queue-num-queues = "2"
// /local/domain/1/device/vbd/0/ring-page-order = "1"
// /local/domain/1/device/vbd/0/queue-0 = ""
// /local/domain/1/device/vbd/0/queue-0/ring-ref0 = "<ring-ref#0>"
// /local/domain/1/device/vbd/0/queue-0/ring-ref1 = "<ring-ref#1>"
// /local/domain/1/device/vbd/0/queue-0/event-channel = "<evtchn#0>"
// /local/domain/1/device/vbd/0/queue-1 = ""
// /local/domain/1/device/vbd/0/queue-1/ring-ref0 = "<ring-ref#2>"
// /local/domain/1/device/vbd/0/queue-1/ring-ref1 = "<ring-ref#3>"
// /local/domain/1/device/vbd/0/queue-1/event-channel = "<evtchn#1>"
//
// REQUEST CODES.
//
pub const BLKIF_OP_READ: c_int = 0;
pub const BLKIF_OP_WRITE: c_int = 1;
//
// Recognised only if "feature-barrier" is present in backend xenbus info.
// The "feature_barrier" node contains a boolean indicating whether barrier
// requests are likely to succeed or fail. Either way, a barrier request
// may fail at any time with BLKIF_RSP_EOPNOTSUPP if it is unsupported by
// the underlying block-device hardware. The boolean simply indicates whether
// or not it is worthwhile for the frontend to attempt barrier requests.
// If a backend does not recognise BLKIF_OP_WRITE_BARRIER, it should *not
// create the "feature-barrier" node!
//
pub const BLKIF_OP_WRITE_BARRIER: c_int = 2;
//
// Recognised if "feature-flush-cache" is present in backend xenbus
// info.  A flush will ask the underlying storage hardware to flush its
// non-volatile caches as appropriate.  The "feature-flush-cache" node
// contains a boolean indicating whether flush requests are likely to
// succeed or fail. Either way, a flush request may fail at any time
// with BLKIF_RSP_EOPNOTSUPP if it is unsupported by the underlying
// block-device hardware. The boolean simply indicates whether or not it
// is worthwhile for the frontend to attempt flushes.  If a backend does
// not recognise BLKIF_OP_WRITE_FLUSH_CACHE, it should *not* create the
// "feature-flush-cache" node!
//
pub const BLKIF_OP_FLUSH_DISKCACHE: c_int = 3;
//
// Recognised only if "feature-discard" is present in backend xenbus info.
// The "feature-discard" node contains a boolean indicating whether trim
// (ATA) or unmap (SCSI) - conviently called discard requests are likely
// to succeed or fail. Either way, a discard request
// may fail at any time with BLKIF_RSP_EOPNOTSUPP if it is unsupported by
// the underlying block-device hardware. The boolean simply indicates whether
// or not it is worthwhile for the frontend to attempt discard requests.
// If a backend does not recognise BLKIF_OP_DISCARD, it should *not
// create the "feature-discard" node!
//
// Discard operation is a request for the underlying block device to mark
// extents to be erased. However, discard does not guarantee that the blocks
// will be erased from the device - it is just a hint to the device
// controller that these blocks are no longer in use. What the device
// controller does with that information is left to the controller.
// Discard operations are passed with sector_number as the
// sector index to begin discard operations at and nr_sectors as the number of
// sectors to be discarded. The specified sectors should be discarded if the
// underlying block device supports trim (ATA) or unmap (SCSI) operations,
// or a BLKIF_RSP_EOPNOTSUPP  should be returned.
// More information about trim/unmap operations at:
// http://t13.org/Documents/UploadedDocuments/docs2008
// e07154r6-Data_Set_Management_Proposal_for_ATA-ACS2.doc
// http://www.seagate.com/staticfiles/support/disc/manuals
// Interface%20manuals/100293068c.pdf
// The backend can optionally provide three extra XenBus attributes to
// further optimize the discard functionality:
// 'discard-alignment' - Devices that support discard functionality may
// internally allocate space in units that are bigger than the exported
// logical block size. The discard-alignment parameter indicates how many bytes
// the beginning of the partition is offset from the internal allocation unit's
// natural alignment.
// 'discard-granularity'  - Devices that support discard functionality may
// internally allocate space using units that are bigger than the logical block
// size. The discard-granularity parameter indicates the size of the internal
// allocation unit in bytes if reported by the device. Otherwise the
// discard-granularity will be set to match the device's physical block size.
// 'discard-secure' - All copies of the discarded sectors (potentially created
// by garbage collection) must also be erased.  To use this feature, the flag
// BLKIF_DISCARD_SECURE must be set in the blkif_request_trim.
//
pub const BLKIF_OP_DISCARD: c_int = 5;
//
// Recognized if "feature-max-indirect-segments" in present in the backend
// xenbus info. The "feature-max-indirect-segments" node contains the maximum
// number of segments allowed by the backend per request. If the node is
// present, the frontend might use blkif_request_indirect structs in order to
// issue requests with more than BLKIF_MAX_SEGMENTS_PER_REQUEST (11). The
// maximum number of indirect segments is fixed by the backend, but the
// frontend can issue requests with any number of indirect segments as long as
// it's less than the number provided by the backend. The indirect_grefs field
// in blkif_request_indirect should be filled by the frontend with the
// grant references of the pages that are holding the indirect segments.
// These pages are filled with an array of blkif_request_segment that hold the
// information about the segments. The number of indirect pages to use is
// determined by the number of segments an indirect request contains. Every
// indirect page can contain a maximum of
// (PAGE_SIZE / sizeof(struct blkif_request_segment)) segments, so to
// calculate the number of indirect pages to use we have to do
// ceil(indirect_segments / (PAGE_SIZE / sizeof(struct blkif_request_segment))).
//
// If a backend does not recognize BLKIF_OP_INDIRECT, it should *not
// create the "feature-max-indirect-segments" node!
//
pub const BLKIF_OP_INDIRECT: c_int = 6;
//
// Maximum scatter/gather segments per request.
// This is carefully chosen so that sizeof(struct blkif_ring) <= PAGE_SIZE.
// NB. This could be 12 if the ring indexes weren't stored in the same page.
//
pub const BLKIF_MAX_SEGMENTS_PER_REQUEST: c_int = 11;
pub const BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_segment {
    pub /: *mut *mut grant_ref_t gref; / reference to I/O buffer frame,
// @first_sect: first sector in frame to transfer (inclusive).
// @last_sect: last sector in frame to transfer (inclusive).
    pub last_sect: uint8_t first_sect,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_rw {
    pub /: *mut *mut uint8_t nr_segments; / number of segments,
    pub /: *mut *mut blkif_vdev_t handle; / only for read/write requests,

    pub /: *mut *mut uint32_t _pad1; / offsetof(blkif_request,u.rw.id) == 8,

    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub /: *mut *mut blkif_sector_t sector_number;/ start sector idx on disk (r/w only),
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_discard {
    pub /: *mut *mut uint8_t flag; / BLKIF_DISCARD_SECURE or zero.,

    pub /: *mut *mut blkif_vdev_t _pad1; / only for read/write requests,

    pub offsetof(blkif_req..,u.discard.id)==8*/: *mut *mut uint32_t _pad2; /,

    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub sector_number: blkif_sector_t,
    pub nr_sectors: u64,
    pub _pad3: u8,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_other {
    pub _pad1: u8,
    pub /: *mut *mut blkif_vdev_t _pad2; / only for read/write requests,

    pub offsetof(blkif_req..,u.other.id)==8*/: *mut *mut uint32_t _pad3; /,

    pub /: *mut *mut uint64_t id; / private guest value, echoed in resp,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,

    pub /: *mut *mut uint32_t _pad1; / offsetof(blkif_...,u.indirect.id) == 8,

    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad2: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
    pub /: *mut *mut uint32_t _pad3; / make it 64 byte aligned,

    pub /: *mut *mut uint64_t _pad3; / make it 64 byte aligned,

    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request {
    pub /: *mut *mut uint8_t operation; / BLKIF_OP_???,
    pub rw: blkif_request_rw,
    pub discard: blkif_request_discard,
    pub other: blkif_request_other,
    pub indirect: blkif_request_indirect,
    pub u: },
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_response {
    pub /: *mut *mut uint64_t id; / copied from request,
    pub /: *mut *mut uint8_t operation; / copied from request,
    pub /: *mut *mut int16_t status; / BLKIF_RSP_???,
}

//
// STATUS RETURN CODES.
//
// Operation not supported (only happens on barrier writes).

// Operation failed for some unspecified reason (-EIO).

// Operation completed successfully.
pub const BLKIF_RSP_OKAY: c_int = 0;
//
// Generate blkif ring structures and types.
//
pub const VDISK_CDROM: c_uint = 0x1;
pub const VDISK_REMOVABLE: c_uint = 0x2;
pub const VDISK_READONLY: c_uint = 0x4;
// Xen-defined major numbers for virtual disks, they look strangely
// familiar
pub const XEN_IDE0_MAJOR: c_int = 3;
pub const XEN_IDE1_MAJOR: c_int = 22;
pub const XEN_SCSI_DISK0_MAJOR: c_int = 8;
pub const XEN_SCSI_DISK1_MAJOR: c_int = 65;
pub const XEN_SCSI_DISK2_MAJOR: c_int = 66;
pub const XEN_SCSI_DISK3_MAJOR: c_int = 67;
pub const XEN_SCSI_DISK4_MAJOR: c_int = 68;
pub const XEN_SCSI_DISK5_MAJOR: c_int = 69;
pub const XEN_SCSI_DISK6_MAJOR: c_int = 70;
pub const XEN_SCSI_DISK7_MAJOR: c_int = 71;
pub const XEN_SCSI_DISK8_MAJOR: c_int = 128;
pub const XEN_SCSI_DISK9_MAJOR: c_int = 129;
pub const XEN_SCSI_DISK10_MAJOR: c_int = 130;
pub const XEN_SCSI_DISK11_MAJOR: c_int = 131;
pub const XEN_SCSI_DISK12_MAJOR: c_int = 132;
pub const XEN_SCSI_DISK13_MAJOR: c_int = 133;
pub const XEN_SCSI_DISK14_MAJOR: c_int = 134;
pub const XEN_SCSI_DISK15_MAJOR: c_int = 135;
