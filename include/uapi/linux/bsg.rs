//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/bsg.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const BSG_PROTOCOL_SCSI: c_int = 0;
pub const BSG_SUB_PROTOCOL_SCSI_CMD: c_int = 0;
pub const BSG_SUB_PROTOCOL_SCSI_TMF: c_int = 1;
pub const BSG_SUB_PROTOCOL_SCSI_TRANSPORT: c_int = 2;
//
// For flag constants below:
// sg.h sg_io_hdr also has bits defined for it's flags member. These
// two flag values (0x10 and 0x20) have the same meaning in sg.h . For
// bsg the BSG_FLAG_Q_AT_HEAD flag is ignored since it is the deafult.
//
pub const BSG_FLAG_Q_AT_TAIL: c_uint = 0x10 /* default is Q_AT_HEAD */;
pub const BSG_FLAG_Q_AT_HEAD: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_io_v4 {
    pub /: *mut *mut __s32 guard; / [i] 'Q' to differentiate from v3,
    pub /: *mut *mut __u32 protocol; / [i] 0 -> SCSI , ....,
    pub task: *mut *mut __u32 subprotocol; / [i] 0 -> SCSI command, 1 -> SCSI,
    pub /: *mut *mut __u32 request_len; / [i] in bytes,
    pub /: *mut *mut *mut __u64 request; / [i], [i] {SCSI: cdb},
    pub /: *mut *mut __u64 request_tag; / [i] {SCSI: task tag (only if flagged)},
    pub /: *mut *mut __u32 request_attr; / [i] {SCSI: task attribute},
    pub /: *mut *mut __u32 request_priority; / [i] {SCSI: task priority},
    pub /: *mut *mut __u32 request_extra; / [i] {spare, for padding},
    pub /: *mut *mut __u32 max_response_len; / [i] in bytes,
    pub /: *mut *mut *mut __u64 response; / [i], [o] {SCSI: (auto)sense data},
// "dout_": data out (to device); "din_": data in (from device)
    pub else: *mut *mut __u32 dout_iovec_count; / [i] 0 -> "flat" dout transfer,
    pub /: *mut *mut __u32 dout_xfer_len; / [i] bytes to be transferred to device,
    pub /: *mut *mut __u32 din_iovec_count; / [i] 0 -> "flat" din transfer,
    pub /: *mut *mut __u32 din_xfer_len; / [i] bytes to be transferred from device,
    pub /: *mut *mut *mut __u64 dout_xferp; / [i], [i],
    pub /: *mut *mut *mut __u64 din_xferp; / [i], [o],
    pub /: *mut *mut __u32 timeout; / [i] units: millisecond,
    pub /: *mut *mut __u32 flags; / [i] bit mask,
    pub /: *mut *mut __u64 usr_ptr; / [i->o] unused internally,
    pub /: *mut *mut __u32 spare_in; / [i],
    pub /: *mut *mut __u32 driver_status; / [o] 0 -> ok,
    pub /: *mut *mut __u32 transport_status; / [o] 0 -> ok,
    pub /: *mut *mut __u32 device_status; / [o] {SCSI: command completion status},
    pub /: *mut *mut __u32 retry_delay; / [o] {SCSI: status auxiliary information},
    pub /: *mut *mut __u32 info; / [o] additional information,
    pub /: *mut *mut __u32 duration; / [o] time to complete, in milliseconds,
    pub /: *mut *mut __u32 response_len; / [o] bytes of response actually written,
    pub /: *mut *mut __s32 din_resid; / [o] din_xfer_len - actual_din_xfer_len,
    pub /: *mut *mut __s32 dout_resid; / [o] dout_xfer_len - actual_dout_xfer_len,
    pub /: *mut *mut __u64 generated_tag; / [o] {SCSI: transport generated task tag},
    pub /: *mut *mut __u32 spare_out; / [o],
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_uring_cmd {
    pub /: *mut *mut *mut __u64 request; / [i], [i] command descriptor address,
    pub /: *mut *mut __u32 request_len; / [i] command descriptor length in bytes,
    pub /: *mut *mut *mut __u32 protocol; / [i] protocol type (BSG_PROTOCOL_),
    pub /: *mut *mut *mut __u32 subprotocol; / [i] subprotocol type (BSG_SUB_PROTOCOL_),
    pub /: *mut *mut __u32 max_response_len; / [i] response buffer size in bytes,
    pub /: *mut *mut *mut __u64 response; / [i], [o] response data address,
    pub /: *mut *mut *mut __u64 dout_xferp; / [i], [i],
    pub /: *mut *mut __u32 dout_xfer_len; / [i] bytes to be transferred to device,
    pub else: *mut *mut __u32 dout_iovec_count; / [i] 0 -> "flat" dout transfer,
// dout_xferp points to array of iovec
//
    pub /: *mut *mut *mut __u64 din_xferp; / [i], [o],
    pub /: *mut *mut __u32 din_xfer_len; / [i] bytes to be transferred from device,
    pub /: *mut *mut __u32 din_iovec_count; / [i] 0 -> "flat" din transfer,
    pub /: *mut *mut __u32 timeout_ms; / [i] timeout in milliseconds,
    pub /: *mut *mut __u8 reserved[12]; / reserved for future extension,
}

// Must match IORING_OP_URING_CMD payload size (e.g. SQE128).

//
// SCSI BSG io_uring completion (res2, 64-bit)
//
// When using BSG_PROTOCOL_SCSI + BSG_SUB_PROTOCOL_SCSI_CMD with
// IORING_OP_URING_CMD, the completion queue entry (CQE) contains:
// - result: errno (0 on success)
// - res2: packed SCSI status
//
// res2 bit layout:
// [0..7]   device_status  (SCSI status byte, e.g. CHECK_CONDITION)
// [8..15]  driver_status  (e.g. DRIVER_SENSE when sense data is valid)
// [16..23] host_status    (e.g. DID_OK, DID_TIME_OUT)
// [24..31] sense_len_wr   (bytes of sense data written to response buffer)
// [32..63] resid_len      (residual transfer length)
//
