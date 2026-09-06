//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/vscsiif.h
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
// vscsiif.h
//
// Based on the blkif.h code.
//
// Copyright(c) FUJITSU Limited 2008.
//

//
// Feature and Parameter Negotiation
// =================================
// The two halves of a Xen pvSCSI driver utilize nodes within the XenStore to
// communicate capabilities and to negotiate operating parameters.  This
// section enumerates these nodes which reside in the respective front and
// backend portions of the XenStore, following the XenBus convention.
//
// Any specified default value is in effect if the corresponding XenBus node
// is not present in the XenStore.
//
// XenStore nodes in sections marked "PRIVATE" are solely for use by the
// driver side whose XenBus tree contains them.
//
// Backend XenBus Nodes
//
// ------------------ Backend Device Identification (PRIVATE) ------------------
//
// p-devname
// Values:         string
//
// A free string used to identify the physical device (e.g. a disk name).
//
// p-dev
// Values:         string
//
// A string specifying the backend device: either a 4-tuple "h:c:t:l"
// (host, controller, target, lun, all integers), or a WWN (e.g.
// "naa.60014054ac780582:0").
//
// v-dev
// Values:         string
//
// A string specifying the frontend device in form of a 4-tuple "h:c:t:l"
// (host, controller, target, lun, all integers).
//
// --------------------------------- Features ---------------------------------
//
// feature-sg-grant
// Values:         unsigned [VSCSIIF_SG_TABLESIZE...65535]
// Default Value:  0
//
// Specifies the maximum number of scatter/gather elements in grant pages
// supported. If not set, the backend supports up to VSCSIIF_SG_TABLESIZE
// SG elements specified directly in the request.
//
// Frontend XenBus Nodes
//
// ----------------------- Request Transport Parameters -----------------------
//
// event-channel
// Values:         unsigned
//
// The identifier of the Xen event channel used to signal activity
// in the ring buffer.
//
// ring-ref
// Values:         unsigned
//
// The Xen grant reference granting permission for the backend to map
// the sole page in a single page sized ring buffer.
//
// protocol
// Values:         string (XEN_IO_PROTO_ABI_*)
// Default Value:  XEN_IO_PROTO_ABI_NATIVE
//
// The machine ABI rules governing the format of all ring request and
// response structures.
//
// Xenstore format in practice
// ===========================
//
// The backend driver uses a single_host:many_devices notation to manage domU
// devices. Everything is stored in /local/domain/<backend_domid>/backend/vscsi/.
// The xenstore layout looks like this (dom0 is assumed to be the backend_domid):
//
// <domid>/<vhost>/feature-host = "0"
// <domid>/<vhost>/frontend = "/local/domain/<domid>/device/vscsi/0"
// <domid>/<vhost>/frontend-id = "<domid>"
// <domid>/<vhost>/online = "1"
// <domid>/<vhost>/state = "4"
// <domid>/<vhost>/vscsi-devs/dev-0/p-dev = "8:0:2:1" or "naa.wwn:lun"
// <domid>/<vhost>/vscsi-devs/dev-0/state = "4"
// <domid>/<vhost>/vscsi-devs/dev-0/v-dev = "0:0:0:0"
// <domid>/<vhost>/vscsi-devs/dev-1/p-dev = "8:0:2:2"
// <domid>/<vhost>/vscsi-devs/dev-1/state = "4"
// <domid>/<vhost>/vscsi-devs/dev-1/v-dev = "0:0:1:0"
//
// The frontend driver maintains its state in
// /local/domain/<domid>/device/vscsi/.
//
// <vhost>/backend = "/local/domain/0/backend/vscsi/<domid>/<vhost>"
// <vhost>/backend-id = "0"
// <vhost>/event-channel = "20"
// <vhost>/ring-ref = "43"
// <vhost>/state = "4"
// <vhost>/vscsi-devs/dev-0/state = "4"
// <vhost>/vscsi-devs/dev-1/state = "4"
//
// In addition to the entries for backend and frontend these flags are stored
// for the toolstack:
//
// <domid>/<vhost>/vscsi-devs/dev-1/p-devname = "/dev/$device"
// <domid>/<vhost>/libxl_ctrl_index = "0"
//
// Backend/frontend protocol
// =========================
//
// To create a vhost along with a device:
// <domid>/<vhost>/feature-host = "0"
// <domid>/<vhost>/frontend = "/local/domain/<domid>/device/vscsi/0"
// <domid>/<vhost>/frontend-id = "<domid>"
// <domid>/<vhost>/online = "1"
// <domid>/<vhost>/state = "1"
// <domid>/<vhost>/vscsi-devs/dev-0/p-dev = "8:0:2:1"
// <domid>/<vhost>/vscsi-devs/dev-0/state = "1"
// <domid>/<vhost>/vscsi-devs/dev-0/v-dev = "0:0:0:0"
// Wait for <domid>/<vhost>/state + <domid>/<vhost>/vscsi-devs/dev-0/state become 4
//
// To add another device to a vhost:
// <domid>/<vhost>/state = "7"
// <domid>/<vhost>/vscsi-devs/dev-1/p-dev = "8:0:2:2"
// <domid>/<vhost>/vscsi-devs/dev-1/state = "1"
// <domid>/<vhost>/vscsi-devs/dev-1/v-dev = "0:0:1:0"
// Wait for <domid>/<vhost>/state + <domid>/<vhost>/vscsi-devs/dev-1/state become 4
//
// To remove a device from a vhost:
// <domid>/<vhost>/state = "7"
// <domid>/<vhost>/vscsi-devs/dev-1/state = "5"
// Wait for <domid>/<vhost>/state to become 4
// Wait for <domid>/<vhost>/vscsi-devs/dev-1/state become 6
// Remove <domid>/<vhost>/vscsi-devs/dev-1/{state,p-dev,v-dev,p-devname}
// Remove <domid>/<vhost>/vscsi-devs/dev-1
//
// Requests from the frontend to the backend
//
// Request a SCSI operation specified via a CDB in vscsiif_request.cmnd.
// The target is specified via channel, id and lun.
//
// The operation to be performed is specified via a CDB in cmnd[], the length
// of the CDB is in cmd_len. sc_data_direction specifies the direction of data
// (to the device, from the device, or none at all).
//
// If data is to be transferred to or from the device the buffer(s) in the
// guest memory is/are specified via one or multiple scsiif_request_segment
// descriptors each specifying a memory page via a grant_ref_t, a offset into
// the page and the length of the area in that page. All scsiif_request_segment
// areas concatenated form the resulting data buffer used by the operation.
// If the number of scsiif_request_segment areas is not too large (less than
// or equal VSCSIIF_SG_TABLESIZE) the areas can be specified directly in the
// seg[] array and the number of valid scsiif_request_segment elements is to be
// set in nr_segments.
//
// If "feature-sg-grant" in the Xenstore is set it is possible to specify more
// than VSCSIIF_SG_TABLESIZE scsiif_request_segment elements via indirection.
// The maximum number of allowed scsiif_request_segment elements is the value
// of the "feature-sg-grant" entry from Xenstore. When using indirection the
// seg[] array doesn't contain specifications of the data buffers, but
// references to scsiif_request_segment arrays, which in turn reference the
// data buffers. While nr_segments holds the number of populated seg[] entries
// (plus the set VSCSIIF_SG_GRANT bit), the number of scsiif_request_segment
// elements referencing the target data buffers is calculated from the lengths
// of the seg[] elements (the sum of all valid seg[].length divided by the
// size of one scsiif_request_segment structure). The frontend may use a mix of
// direct and indirect requests.
//
pub const VSCSIIF_ACT_SCSI_CDB: c_int = 1;
//
// Request abort of a running operation for the specified target given by
// channel, id, lun and the operation's rqid in ref_rqid.
//
pub const VSCSIIF_ACT_SCSI_ABORT: c_int = 2;
//
// Request a device reset of the specified target (channel and id).
//
pub const VSCSIIF_ACT_SCSI_RESET: c_int = 3;
//
// Preset scatter/gather elements for a following request. Deprecated.
// Keeping the define only to avoid usage of the value "4" for other actions.
//
pub const VSCSIIF_ACT_SCSI_SG_PRESET: c_int = 4;
//
// Maximum scatter/gather segments per request.
//
// Considering balance between allocating at least 16 "vscsiif_request"
// structures on one page (4096 bytes) and the number of scatter/gather
// elements needed, we decided to use 26 as a magic number.
//
// If "feature-sg-grant" is set, more scatter/gather elements can be specified
// by placing them in one or more (up to VSCSIIF_SG_TABLESIZE) granted pages.
// In this case the vscsiif_request seg elements don't contain references to
// the user data, but to the SG elements referencing the user data.
//
pub const VSCSIIF_SG_TABLESIZE: c_int = 26;
//
// based on Linux kernel 2.6.18, still valid
//
// Changing these values requires support of multiple protocols via the rings
// as "old clients" will blindly use these values and the resulting structure
// sizes.
//
pub const VSCSIIF_MAX_COMMAND_SIZE: c_int = 16;
pub const VSCSIIF_SENSE_BUFFERSIZE: c_int = 96;
pub const VSCSIIF_PAGE_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsiif_request_segment {
    pub gref: grant_ref_t,
    pub offset: u16,
    pub length: u16,
}

// Size of one request is 252 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vscsiif_request {
    pub /: *mut *mut uint16_t rqid; / private guest value, echoed in resp,
    pub /: *mut *mut uint8_t act; / command between backend and frontend,
    pub /: *mut *mut uint8_t cmd_len; / valid CDB bytes,
    pub /: *mut *mut uint8_t cmnd[VSCSIIF_MAX_COMMAND_SIZE]; / the CDB,
    pub /: *mut *mut uint16_t timeout_per_command; / deprecated,
    pub /: *mut *mut uint16_t channel, id, lun; / (virtual) device specification,
    pub /: *mut *mut uint16_t ref_rqid; / command abort reference,
    pub DMA_TO_DEVICE(1): *mut *mut uint8_t sc_data_direction; / for,
    pub /: *mut *mut uint8_t nr_segments; / Number of pieces of scatter-gather,
//
// flag in nr_segments: SG elements via grant page
//
// If VSCSIIF_SG_GRANT is set, the low 7 bits of nr_segments specify the number
// of grant pages containing SG elements. Usable if "feature-sg-grant" set.
//
pub const VSCSIIF_SG_GRANT: c_uint = 0x80;
    pub seg: [scsiif_request_segment; VSCSIIF_SG_TABLESIZE],
    pub reserved: [u32; 3],
}

// Size of one response is 252 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vscsiif_response {
    pub /: *mut *mut uint16_t rqid; / identifies request,
    pub padding: u8,
    pub sense_len: u8,
    pub sense_buffer: [u8; VSCSIIF_SENSE_BUFFERSIZE],
    pub rslt: i32,
    pub -: *mut *mut uint32_t residual_len; / request bufflen,
    pub reserved: [u32; 36],
}

// SCSI I/O status from vscsiif_response->rslt

// Host I/O status from vscsiif_response->rslt

pub const XEN_VSCSIIF_RSLT_HOST_OK: c_int = 0;
// Couldn't connect before timeout
pub const XEN_VSCSIIF_RSLT_HOST_NO_CONNECT: c_int = 1;
// Bus busy through timeout
pub const XEN_VSCSIIF_RSLT_HOST_BUS_BUSY: c_int = 2;
// Timed out for other reason
pub const XEN_VSCSIIF_RSLT_HOST_TIME_OUT: c_int = 3;
// Bad target
pub const XEN_VSCSIIF_RSLT_HOST_BAD_TARGET: c_int = 4;
// Abort for some other reason
pub const XEN_VSCSIIF_RSLT_HOST_ABORT: c_int = 5;
// Parity error
pub const XEN_VSCSIIF_RSLT_HOST_PARITY: c_int = 6;
// Internal error
pub const XEN_VSCSIIF_RSLT_HOST_ERROR: c_int = 7;
// Reset by somebody
pub const XEN_VSCSIIF_RSLT_HOST_RESET: c_int = 8;
// Unexpected interrupt
pub const XEN_VSCSIIF_RSLT_HOST_BAD_INTR: c_int = 9;
// Force command past mid-layer
pub const XEN_VSCSIIF_RSLT_HOST_PASSTHROUGH: c_int = 10;
// Retry requested
pub const XEN_VSCSIIF_RSLT_HOST_SOFT_ERROR: c_int = 11;
// Hidden retry requested
pub const XEN_VSCSIIF_RSLT_HOST_IMM_RETRY: c_int = 12;
// Requeue command requested
pub const XEN_VSCSIIF_RSLT_HOST_REQUEUE: c_int = 13;
// Transport error disrupted I/O
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_DISRUPTED: c_int = 14;
// Transport class fastfailed
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_FAILFAST: c_int = 15;
// Permanent target failure
pub const XEN_VSCSIIF_RSLT_HOST_TARGET_FAILURE: c_int = 16;
// Permanent nexus failure on path
pub const XEN_VSCSIIF_RSLT_HOST_NEXUS_FAILURE: c_int = 17;
// Space allocation on device failed
pub const XEN_VSCSIIF_RSLT_HOST_ALLOC_FAILURE: c_int = 18;
// Medium error
pub const XEN_VSCSIIF_RSLT_HOST_MEDIUM_ERROR: c_int = 19;
// Transport marginal errors
pub const XEN_VSCSIIF_RSLT_HOST_TRANSPORT_MARGINAL: c_int = 20;
// Result values of reset operations
pub const XEN_VSCSIIF_RSLT_RESET_SUCCESS: c_uint = 0x2002;
pub const XEN_VSCSIIF_RSLT_RESET_FAILED: c_uint = 0x2003;
