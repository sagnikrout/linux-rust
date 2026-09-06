//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ublk_cmd.h
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

// ublk server command definition
//
// Admin commands, issued by ublk server, and handled by ublk driver.
//
// Legacy command definition, don't use in new application, and don't
// add new such definition any more
//
pub const UBLK_CMD_GET_QUEUE_AFFINITY: c_uint = 0x01;
pub const UBLK_CMD_GET_DEV_INFO: c_uint = 0x02;
pub const UBLK_CMD_ADD_DEV: c_uint = 0x04;
pub const UBLK_CMD_DEL_DEV: c_uint = 0x05;
pub const UBLK_CMD_START_DEV: c_uint = 0x06;
pub const UBLK_CMD_STOP_DEV: c_uint = 0x07;
pub const UBLK_CMD_SET_PARAMS: c_uint = 0x08;
pub const UBLK_CMD_GET_PARAMS: c_uint = 0x09;
pub const UBLK_CMD_START_USER_RECOVERY: c_uint = 0x10;
pub const UBLK_CMD_END_USER_RECOVERY: c_uint = 0x11;
pub const UBLK_CMD_GET_DEV_INFO2: c_uint = 0x12;
// Any new ctrl command should encode by __IO*()

//
// Register a shared memory buffer for zero-copy I/O.
// Input:  ctrl_cmd.addr points to struct ublk_shmem_buf_reg (buffer VA + size)
// ctrl_cmd.len  = sizeof(struct ublk_shmem_buf_reg)
// Result: >= 0 is the assigned buffer index, < 0 is error
//
// The kernel pins pages from the calling process's address space
// and inserts PFN ranges into a per-device maple tree. When a block
// request's pages match registered pages, the driver sets
// UBLK_IO_F_SHMEM_ZC and encodes the buffer index + offset in addr,
// allowing the server to access the data via its own mapping of the
// same shared memory — true zero copy.
//
// The memory can be backed by memfd, hugetlbfs, or any GUP-compatible
// shared mapping. Queue freeze is handled internally.
//
// The buffer VA and size are passed via a user buffer (not inline in
// ctrl_cmd) so that unprivileged devices can prepend the device path
// to ctrl_cmd.addr without corrupting the VA.
//

//
// Unregister a shared memory buffer.
// Input:  ctrl_cmd.data[0] = buffer index
//

// Parameter buffer for UBLK_U_CMD_REG_BUF, pointed to by ctrl_cmd.addr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_shmem_buf_reg {
    pub /: *mut *mut __u64 addr; / userspace virtual address of shared memory,
    pub /: *mut *mut __u64 len; / buffer size in bytes, page-aligned, default max 4GB,
    pub flags: __u32,
    pub reserved: __u32,
}

// Pin pages without FOLL_WRITE; usable with write-sealed memfd

//
// 64bits are enough now, and it should be easy to extend in case of
// running out of feature flags
//
pub const UBLK_FEATURES_LEN: c_int = 8;
//
// IO commands, issued by ublk server, and handled by ublk driver.
//
// FETCH_REQ: issued via sqe(URING_CMD) beforehand for fetching IO request
// from ublk driver, should be issued only when starting device. After
// the associated cqe is returned, request's tag can be retrieved via
// cqe->userdata.
//
// COMMIT_AND_FETCH_REQ: issued via sqe(URING_CMD) after ublkserver handled
// this IO request, request's handling result is committed to ublk
// driver, meantime FETCH_REQ is piggyback, and FETCH_REQ has to be
// handled before completing io request.
//
// NEED_GET_DATA: only used for write requests to set io addr and copy data
// When NEED_GET_DATA is set, ublksrv has to issue UBLK_IO_NEED_GET_DATA
// command after ublk driver returns UBLK_IO_RES_NEED_GET_DATA.
//
// It is only used if ublksrv set UBLK_F_NEED_GET_DATA flag
// while starting a ublk device.
//
// Legacy IO command definition, don't use in new application, and don't
// add new such definition any more
//
pub const UBLK_IO_FETCH_REQ: c_uint = 0x20;
pub const UBLK_IO_COMMIT_AND_FETCH_REQ: c_uint = 0x21;
pub const UBLK_IO_NEED_GET_DATA: c_uint = 0x22;
// Any new IO command should encode by __IOWR()

//
// return 0 if the command is run successfully, otherwise failure code
// is returned
//

//
// If failure code is returned, nothing in the command buffer is handled.
// Otherwise, the returned value means how many bytes in command buffer
// are handled actually, then number of handled IOs can be calculated with
// `elem_bytes` for each IO. IOs in the remained bytes are not committed,
// userspace has to check return value for dealing with partial committing
// correctly.
//

//
// Fetch io commands to provided buffer in multishot style,
// `IORING_URING_CMD_MULTISHOT` is required for this command.
//

// only ABORT means that no re-fetch
pub const UBLK_IO_RES_OK: c_int = 0;
pub const UBLK_IO_RES_NEED_GET_DATA: c_int = 1;

pub const UBLKSRV_CMD_BUF_OFFSET: c_int = 0;
pub const UBLKSRV_IO_BUF_OFFSET: c_uint = 0x80000000;
// tag bit is 16bit, so far limit at most 4096 IOs for each queue
pub const UBLK_MAX_QUEUE_DEPTH: c_int = 4096;
// single IO buffer max size is 32MB
pub const UBLK_IO_BUF_OFF: c_int = 0;
pub const UBLK_IO_BUF_BITS: c_int = 25;

// so at most 64K IOs for each queue

pub const UBLK_TAG_BITS: c_int = 16;

// max 4096 queues

pub const UBLK_QID_BITS: c_int = 12;

// Copy to/from request integrity buffer instead of data buffer
pub const UBLK_INTEGRITY_FLAG_OFF: c_int = 62;

//
// ublk server can register data buffers for incoming I/O requests with a sparse
// io_uring buffer table. The request buffer can then be used as the data buffer
// for io_uring operations via the fixed buffer index.
// Note that the ublk server can never directly access the request data memory.
//
// To use this feature, the ublk server must first register a sparse buffer
// table on an io_uring instance.
// When an incoming ublk request is received, the ublk server submits a
// UBLK_U_IO_REGISTER_IO_BUF command to that io_uring instance. The
// ublksrv_io_cmd's q_id and tag specify the request whose buffer to register
// and addr is the index in the io_uring's buffer table to install the buffer.
// SQEs can now be submitted to the io_uring to read/write the request's buffer
// by enabling fixed buffers (e.g. using IORING_OP_{READ,WRITE}_FIXED or
// IORING_URING_CMD_FIXED) and passing the registered buffer index in buf_index.
// Once the last io_uring operation using the request's buffer has completed,
// the ublk server submits a UBLK_U_IO_UNREGISTER_IO_BUF command with q_id, tag,
// and addr again specifying the request buffer to unregister.
// The ublk request is completed when its buffer is unregistered from all
// io_uring instances and the ublk server issues UBLK_U_IO_COMMIT_AND_FETCH_REQ.
//
// Not available for UBLK_F_UNPRIVILEGED_DEV, as a ublk server can leak
// uninitialized kernel memory by not reading into the full request buffer.
//

//
// Force to complete io cmd via io_uring_cmd_complete_in_task so that
// performance comparison is done easily with using task_work_add
//

//
// User should issue io cmd again for write requests to
// set io buffer address and copy data from bio vectors
// to the userspace io buffer.
//
// In this mode, task_work is not used.
//

//
// - Block devices are recoverable if ublk server exits and restarts
// - Outstanding I/O when ublk server exits is met with errors
// - I/O issued while there is no ublk server queues
//

//
// - Block devices are recoverable if ublk server exits and restarts
// - Outstanding I/O when ublk server exits is reissued
// - I/O issued while there is no ublk server queues
//

//
// Unprivileged user can create /dev/ublkcN and /dev/ublkbN.
//
// /dev/ublk-control needs to be available for unprivileged user, and it
// can be done via udev rule to make all control commands available to
// unprivileged user. Except for the command of UBLK_CMD_ADD_DEV, all
// other commands are only allowed for the owner of the specified device.
//
// When userspace sends UBLK_CMD_ADD_DEV, the device pair's owner_uid and
// owner_gid are stored to ublksrv_ctrl_dev_info by kernel, so far only
// the current user's uid/gid is stored, that said owner of the created
// device is always the current user.
//
// We still need udev rule to apply OWNER/GROUP with the stored owner_uid
// and owner_gid.
//
// Then ublk server can be run as unprivileged user, and /dev/ublkbN can
// be accessed and managed by its owner represented by owner_uid/owner_gid.
//

// use ioctl encoding for uring command

//
// Copy between request and user buffer by pread()/pwrite()
//
// Not available for UBLK_F_UNPRIVILEGED_DEV, otherwise userspace may
// deceive us by not filling request buffer, then kernel uninitialized
// data may be leaked.
//

//
// User space sets this flag when setting up the device to request zoned storage support. Kernel may
// deny the request by returning an error.
//

//
// - Block devices are recoverable if ublk server exits and restarts
// - Outstanding I/O when ublk server exits is met with errors
// - I/O issued while there is no ublk server is met with errors
//

//
// Resizing a block device is possible with UBLK_U_CMD_UPDATE_SIZE
// New size is passed in cmd->data[0] and is in units of sectors
//

//
// request buffer is registered automatically to uring_cmd's io_uring
// context before delivering this io command to ublk server, meantime
// it is un-registered automatically when completing this io command.
//
// For using this feature:
//
// - ublk server has to create sparse buffer table on the same `io_ring_ctx`
// for issuing `UBLK_IO_FETCH_REQ` and `UBLK_IO_COMMIT_AND_FETCH_REQ`.
// If uring_cmd isn't issued on same `io_ring_ctx`, it is ublk server's
// responsibility to unregister the buffer by issuing `IO_UNREGISTER_IO_BUF`
// manually, otherwise this ublk request won't complete.
//
// - ublk server passes auto buf register data via uring_cmd's sqe->addr,
// `struct ublk_auto_buf_reg` is populated from sqe->addr, please see
// the definition of ublk_sqe_addr_to_auto_buf_reg()
//
// - pass buffer index from `ublk_auto_buf_reg.index`
//
// - all reserved fields in `ublk_auto_buf_reg` need to be zeroed
//
// - pass flags from `ublk_auto_buf_reg.flags` if needed
//
// This way avoids extra cost from two uring_cmd, but also simplifies backend
// implementation, such as, the dependency on IO_REGISTER_IO_BUF and
// IO_UNREGISTER_IO_BUF becomes not necessary.
//
// If wrong data or flags are provided, both IO_FETCH_REQ and
// IO_COMMIT_AND_FETCH_REQ are failed, for the latter, the ublk IO request
// won't be completed until new IO_COMMIT_AND_FETCH_REQ command is issued
// successfully
//

//
// Control command `UBLK_U_CMD_QUIESCE_DEV` is added for quiescing device,
// which state can be transitioned to `UBLK_S_DEV_QUIESCED` or
// `UBLK_S_DEV_FAIL_IO` finally, and it needs ublk server cooperation for
// handling `UBLK_IO_RES_ABORT` correctly.
//
// Typical use case is for supporting to upgrade ublk server application,
// meantime keep ublk block device persistent during the period.
//
// This feature is only available when UBLK_F_USER_RECOVERY is enabled.
//
// Note, this command returns -EBUSY in case that all IO commands are being
// handled by ublk server and not completed in specified time period which
// is passed from the control command parameter.
//

//
// If this feature is set, ublk_drv supports each (qid,tag) pair having
// its own independent daemon task that is responsible for handling it.
// If it is not set, daemons are per-queue instead, so for two pairs
// (qid1,tag1) and (qid2,tag2), if qid1 == qid2, then the same task must
// be responsible for handling (qid1,tag1) and (qid2,tag2).
//

//
// If this feature is set, UBLK_U_IO_REGISTER_IO_BUF/UBLK_U_IO_UNREGISTER_IO_BUF
// can be issued for an I/O on any task. q_id and tag are also ignored in
// UBLK_U_IO_UNREGISTER_IO_BUF's ublksrv_io_cmd.
// If it is unset, zero-copy buffers can only be registered and unregistered by
// the I/O's daemon task. The q_id and tag of the registered buffer are required
// in UBLK_U_IO_UNREGISTER_IO_BUF's ublksrv_io_cmd.
//

//
// Support the following commands for delivering & committing io command
// in batch.
//
// - UBLK_U_IO_PREP_IO_CMDS
// - UBLK_U_IO_COMMIT_IO_CMDS
// - UBLK_U_IO_FETCH_IO_CMDS
// - UBLK_U_IO_REGISTER_IO_BUF
// - UBLK_U_IO_UNREGISTER_IO_BUF
//
// The existing UBLK_U_IO_FETCH_REQ, UBLK_U_IO_COMMIT_AND_FETCH_REQ and
// UBLK_U_IO_NEED_GET_DATA uring_cmd are not supported for this feature.
//

//
// ublk device supports requests with integrity/metadata buffer.
// Requires UBLK_F_USER_COPY.
//

//
// The device supports the UBLK_CMD_TRY_STOP_DEV command, which
// allows stopping the device only if there are no openers.
//

// Disable automatic partition scanning when device is started

//
// Enable shared memory zero copy. When enabled, the server can register
// shared memory buffers via UBLK_U_CMD_REG_BUF. If a block request's
// pages match a registered buffer, UBLK_IO_F_SHMEM_ZC is set and addr
// encodes the buffer index + offset instead of a userspace buffer address.
//

// ublksrv_io_desc size is specified by ublksrv_ctrl_dev_info's io_desc_size

// device state
pub const UBLK_S_DEV_DEAD: c_int = 0;
pub const UBLK_S_DEV_LIVE: c_int = 1;
pub const UBLK_S_DEV_QUIESCED: c_int = 2;
pub const UBLK_S_DEV_FAIL_IO: c_int = 3;
// shipped via sqe->cmd of io_uring command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublksrv_ctrl_cmd {
// sent to which device, must be valid
    pub dev_id: __u32,
// sent to which queue, must be -1 if the cmd isn't for queue
    pub queue_id: __u16,
//
// cmd specific buffer, can be IN or OUT.
//
    pub len: __u16,
    pub addr: __u64,
// inline data
    pub data: [__u64; 1],
//
// Used for UBLK_F_UNPRIVILEGED_DEV and UBLK_CMD_GET_DEV_INFO2
// only, include null char
//
    pub dev_path_len: __u16,
    pub pad: __u16,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublksrv_ctrl_dev_info {
    pub nr_hw_queues: __u16,
    pub queue_depth: __u16,
    pub state: __u16,
    pub io_desc_size: __u16,
    pub max_io_buf_bytes: __u32,
    pub dev_id: __u32,
    pub ublksrv_pid: __s32,
    pub pad1: __u32,
    pub flags: __u64,
// For ublksrv internal use, invisible to ublk driver
    pub ublksrv_flags: __u64,
    pub /: *mut *mut __u32 owner_uid; / store by kernel,
    pub /: *mut *mut __u32 owner_gid; / store by kernel,
    pub reserved1: __u64,
    pub reserved2: __u64,
}

pub const UBLK_IO_OP_READ: c_int = 0;
pub const UBLK_IO_OP_WRITE: c_int = 1;
pub const UBLK_IO_OP_FLUSH: c_int = 2;
pub const UBLK_IO_OP_DISCARD: c_int = 3;
pub const UBLK_IO_OP_WRITE_SAME: c_int = 4;
pub const UBLK_IO_OP_WRITE_ZEROES: c_int = 5;
pub const UBLK_IO_OP_ZONE_OPEN: c_int = 10;
pub const UBLK_IO_OP_ZONE_CLOSE: c_int = 11;
pub const UBLK_IO_OP_ZONE_FINISH: c_int = 12;
pub const UBLK_IO_OP_ZONE_APPEND: c_int = 13;
pub const UBLK_IO_OP_ZONE_RESET_ALL: c_int = 14;
pub const UBLK_IO_OP_ZONE_RESET: c_int = 15;
//
// Construct a zone report. The report request is carried in `struct
// ublksrv_io_desc`. The `start_sector` field must be the first sector of a zone
// and shall indicate the first zone of the report. The `nr_zones` shall
// indicate how many zones should be reported at most. The report shall be
// delivered as a `struct blk_zone` array. To report fewer zones than requested,
// zero the last entry of the returned array.
//
// Related definitions(blk_zone, blk_zone_cond, blk_zone_type, ...) in
// include/uapi/linux/blkzoned.h are part of ublk UAPI.
//
pub const UBLK_IO_OP_REPORT_ZONES: c_int = 18;

//
// For UBLK_F_AUTO_BUF_REG & UBLK_AUTO_BUF_REG_FALLBACK only.
//
// This flag is set if auto buffer register is failed & ublk server passes
// UBLK_AUTO_BUF_REG_FALLBACK, and ublk server need to register buffer
// manually for handling the delivered IO command if this flag is observed
//
// ublk server has to check this flag if UBLK_AUTO_BUF_REG_FALLBACK is
// passed in.
//

// Request has an integrity data buffer

//
// I/O buffer is in a registered shared memory buffer. When set, the addr
// field in ublksrv_io_desc encodes buffer index and byte offset instead
// of a userspace virtual address.
//

//
// io cmd is described by this structure, and stored in share memory, indexed
// by request tag.
//
// The data is stored by ublk driver, and read by ublksrv after one fetch command
// returns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublksrv_io_desc {
// op: bit 0-7, flags: bit 8-31
    pub op_flags: __u32,
    pub nr_sectors: __u32,
    pub /: *mut *mut __u32 nr_zones; / for UBLK_IO_OP_REPORT_ZONES,
}

// start sector for this io
// buffer address in ublksrv daemon vm space, from ublk driver
//
// If this flag is set, fallback by completing the uring_cmd and setting
// `UBLK_IO_F_NEED_REG_BUF` in case of auto-buf-register failure;
// otherwise the client ublk request is failed silently
//
// If ublk server passes this flag, it has to check if UBLK_IO_F_NEED_REG_BUF
// is set in `ublksrv_io_desc.op_flags`. If UBLK_IO_F_NEED_REG_BUF is set,
// ublk server needs to register io buffer manually for handling IO command.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_auto_buf_reg {
// index for registering the delivered request buffer
    pub index: __u16,
    pub flags: __u8,
    pub reserved0: __u8,
//
// io_ring FD can be passed via the reserve field in future for
// supporting to register io buffer to external io_uring
//
    pub reserved1: __u32,
}

//
// For UBLK_F_AUTO_BUF_REG, auto buffer register data is carried via
// uring_cmd's sqe->addr:
//
// - bit0 ~ bit15: buffer index
// - bit16 ~ bit23: flags
// - bit24 ~ bit31: reserved0
// - bit32 ~ bit63: reserved1
//
// issued to ublk driver via /dev/ublkcN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublksrv_io_cmd {
    pub q_id: __u16,
// for fetch/commit which result
    pub tag: __u16,
// io result, it is valid for COMMIT* command only
    pub result: __s32,
//
// userspace buffer address in ublksrv daemon process, valid for
// FETCH* command only
//
// `addr` should not be used when UBLK_F_USER_COPY is enabled,
// because userspace handles data copy by pread()/pwrite() over
// /dev/ublkcN. But in case of UBLK_F_ZONED, this union is
// re-used to pass back the allocated LBA for
// UBLK_IO_OP_ZONE_APPEND which actually depends on
// UBLK_F_USER_COPY
//
    pub addr: __u64,
    pub zone_append_lba: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_elem_header {
    pub /: *mut *mut __u16 tag; / IO tag,
//
// Buffer index for incoming io command, only valid iff
// UBLK_F_AUTO_BUF_REG is set
//
    pub buf_index: __u16,
    pub /: *mut *mut __s32 result; / I/O completion result (commit only),
}

//
// uring_cmd buffer structure for batch commands
//
// buffer includes multiple elements, which number is specified by
// `nr_elem`. Each element buffer is organized in the following order:
//
// struct ublk_elem_buffer {
// // Mandatory fields (8 bytes)
// struct ublk_elem_header header;
//
// // Optional fields (8 bytes each, included based on flags)
//
// // Buffer address (if UBLK_BATCH_F_HAS_BUF_ADDR) for copying data
// // between ublk request and ublk server buffer
// __u64 buf_addr;
//
// // returned Zone append LBA (if UBLK_BATCH_F_HAS_ZONE_LBA)
// __u64 zone_lba;
// }
//
// Used for `UBLK_U_IO_PREP_IO_CMDS` and `UBLK_U_IO_COMMIT_IO_CMDS`
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_batch_io {
    pub q_id: __u16,

    pub flags: __u16,
    pub nr_elem: __u16,
    pub elem_bytes: __u8,
    pub reserved: __u8,
    pub reserved2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_basic {

    pub attrs: __u32,
    pub logical_bs_shift: __u8,
    pub physical_bs_shift: __u8,
    pub io_opt_shift: __u8,
    pub io_min_shift: __u8,
    pub max_sectors: __u32,
    pub chunk_sectors: __u32,
    pub dev_sectors: __u64,
    pub virt_boundary_mask: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_discard {
    pub discard_alignment: __u32,
    pub discard_granularity: __u32,
    pub max_discard_sectors: __u32,
    pub max_write_zeroes_sectors: __u32,
    pub max_discard_segments: __u16,
    pub reserved0: __u16,
}

//
// read-only, can't set via UBLK_CMD_SET_PARAMS, disk_devt is available
// after device is started
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_devt {
    pub char_major: __u32,
    pub char_minor: __u32,
    pub disk_major: __u32,
    pub disk_minor: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_zoned {
    pub max_open_zones: __u32,
    pub max_active_zones: __u32,
    pub max_zone_append_sectors: __u32,
    pub reserved: [__u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_dma_align {
    pub alignment: __u32,
    pub pad: [__u8; 4],
}

pub const UBLK_MIN_SEGMENT_SIZE: c_int = 4096;
//
// If any one of the three segment parameter is set as 0, the behavior is
// undefined.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_segment {
//
// seg_boundary_mask + 1 needs to be power_of_2(), and the sum has
// to be >= UBLK_MIN_SEGMENT_SIZE(4096)
//
    pub seg_boundary_mask: __u64,
//
// max_segment_size could be override by virt_boundary_mask, so be
// careful when setting both.
//
// max_segment_size has to be >= UBLK_MIN_SEGMENT_SIZE(4096)
//
    pub max_segment_size: __u32,
    pub max_segments: __u16,
    pub pad: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_param_integrity {
    pub /: *mut *mut *mut __u32 flags; / LBMD_PI_CAP_ from linux/fs.h,
    pub /: *mut *mut __u16 max_integrity_segments; / 0 means no limit,
    pub interval_exp: __u8,
    pub /: *mut *mut __u8 metadata_size; / UBLK_PARAM_TYPE_INTEGRITY requires nonzero,
    pub pi_offset: __u8,
    pub /: *mut *mut *mut __u8 csum_type; / LBMD_PI_CSUM_ from linux/fs.h,
    pub tag_size: __u8,
    pub pad: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_params {
//
// Total length of parameters, userspace has to set 'len' for both
// SET_PARAMS and GET_PARAMS command, and driver may update len
// if two sides use different version of 'ublk_params', same with
// 'types' fields.
//
    pub len: __u32,

    pub /: *mut *mut __u32 types; / types of parameter included,
    pub basic: ublk_param_basic,
    pub discard: ublk_param_discard,
    pub devt: ublk_param_devt,
    pub zoned: ublk_param_zoned,
    pub dma: ublk_param_dma_align,
    pub seg: ublk_param_segment,
    pub integrity: ublk_param_integrity,
}

//
// Shared memory zero-copy addr encoding for UBLK_IO_F_SHMEM_ZC.
//
// When UBLK_IO_F_SHMEM_ZC is set, ublksrv_io_desc.addr is encoded as:
// bits [0:31]  = byte offset within the buffer (up to 4GB)
// bits [32:47] = buffer index (up to 65536)
// bits [48:63] = reserved (must be zero)
//
pub const UBLK_SHMEM_ZC_OFF_MASK: c_uint = 0xffffffffULL;
pub const UBLK_SHMEM_ZC_IDX_OFF: c_int = 32;
pub const UBLK_SHMEM_ZC_IDX_MASK: c_uint = 0xffffULL;
