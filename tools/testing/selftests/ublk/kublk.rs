//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/ublk/kublk.h
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

// allow ublk_dep.h to override ublk_cmd.h

pub const MAX_BACK_FILES: c_int = 4;
// part 1: libublk

pub const UBLK_CTRL_RING_DEPTH: c_int = 32;

pub const UBLK_MAX_QUEUES_SHIFT: c_int = 5;

pub const UBLK_MAX_THREADS_SHIFT: c_int = 5;

pub const UBLK_QUEUE_DEPTH: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_ctx {
// stripe
    pub chunk_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fault_inject_ctx {
// fault_inject
    pub delay_us: c_ulong,
    pub die_during_fetch: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct params_ctx {
    pub types: __u32,
    pub logical_bs_shift: __u32,
    pub physical_bs_shift: __u32,
    pub io_min_shift: __u32,
    pub io_opt_shift: __u32,
    pub max_sectors: __u32,
    pub chunk_sectors: __u32,
    pub dev_sectors: __u64,
    pub max_open_zones: __u32,
    pub max_active_zones: __u32,
    pub max_zone_append_sectors: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_ctx {
    pub tgt_type: [c_char; 16],
    pub flags: c_ulong,
    pub nr_hw_queues: unsigned,
    pub nthreads: c_ushort,
    pub queue_depth: unsigned,
    pub dev_id: c_int,
    pub nr_files: c_int,
    pub files: [*mut c_char; MAX_BACK_FILES],
    pub logging:1: c_uint,
    pub all:1: c_uint,
    pub fg:1: c_uint,
    pub recovery:1: c_uint,
    pub auto_zc_fallback:1: c_uint,
    pub per_io_tasks:1: c_uint,
    pub no_ublk_fixed_fd:1: c_uint,
    pub safe_stop:1: c_uint,
    pub no_auto_part_scan:1: c_uint,
    pub rdonly_shmem_buf:1: c_uint,
    pub rotate_auto_buf:1: c_uint,
    pub integrity_flags: __u32,
    pub metadata_size: __u8,
    pub pi_offset: __u8,
    pub csum_type: __u8,
    pub tag_size: __u8,
    pub io_desc_size: __u16,
    pub _evtfd: c_int,
    pub _shmid: c_int,
// built from shmem, only for ublk_dump_dev()
    pub shadow_dev: *mut ublk_dev,
// for 'update_size' command
    pub size: c_ulonglong,
    pub params: params_ctx,
    pub htlb_path: *mut c_char,
    pub stripe: stripe_ctx,
    pub fault_inject: fault_inject_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_ctrl_cmd_data {
    pub cmd_op: __u32,
pub const CTRL_CMD_HAS_DATA: c_int = 1;
pub const CTRL_CMD_HAS_BUF: c_int = 2;
    pub flags: __u32,
    pub data: [__u64; 2],
    pub addr: __u64,
    pub len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_io {
    pub buf_addr: *mut c_char,
    pub integrity_buf: *mut c_void,

    pub flags: c_ushort,
    pub /: *mut *mut unsigned short refs; / used by target code only,
    pub tag: c_int,
    pub result: c_int,
    pub buf_index: c_ushort,
    pub tgt_ios: c_ushort,
    pub auto_buf_phase: c_uchar,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_tgt_ops {
    pub name: *const c_char,
    pub ): *const *const *const int (init_tgt)(struct dev_ctx ctx, struct ublk_dev,
    pub ): *mut *mut void (deinit_tgt)(struct ublk_dev,
    pub batch): int tag, bool,
    pub tag): *mut *mut *mut *mut int (queue_io)(struct ublk_thread , struct ublk_queue , int,
    pub ): *const io_uring_cqe,
//
// Target specific command line handling
//
// each option requires argument for target command line
//
    pub argv[]): *mut *mut *mut void (parse_cmd_line)(struct dev_ctx ctx, int argc, char,
    pub ops): *const *const void (usage)(struct ublk_tgt_ops,
// return buffer index for UBLK_F_AUTO_BUF_REG
    pub tag): *const *const ublk_queue , int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_tgt {
    pub dev_size: c_ulong,
    pub sq_depth: c_uint,
    pub cq_depth: c_uint,
    pub ops: *const ublk_tgt_ops,
    pub params: ublk_params,
    pub nr_backing_files: c_int,
    pub backing_file_size: [c_ulong; MAX_BACK_FILES],
    pub backing_file: [c_char; MAX_BACK_FILES][PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_queue {
    pub q_id: c_int,
    pub q_depth: c_int,
    pub dev: *mut ublk_dev,
    pub tgt_ops: *const ublk_tgt_ops,
    pub io_cmd_buf: *mut ublksrv_io_desc,
// borrow three bit of ublk uapi flags, which may never be used

    pub flags: __u64,
    pub /: *mut *mut int ublk_fd; / cached ublk char device fd,
    pub metadata_size: __u8,
    pub io_desc_size: __u16,
    pub ios: [ublk_io; UBLK_QUEUE_DEPTH],
// used for prep io commands
    pub lock: pthread_spinlock_t,
}

// align with `ublk_elem_header`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_batch_elem {
    pub tag: __u16,
    pub buf_index: __u16,
    pub result: __s32,
    pub buf_addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct batch_commit_buf {
    pub q_id: c_ushort,
    pub buf_idx: c_ushort,
    pub elem: *mut c_void,
    pub done: c_ushort,
    pub count: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct batch_fetch_buf {
    pub br: *mut io_uring_buf_ring,
    pub fetch_buf: *mut c_void,
    pub fetch_buf_size: c_uint,
    pub fetch_buf_off: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_thread {
// Thread-local copy of queue-to-thread mapping for this thread
    pub q_map: [c_uchar; UBLK_MAX_QUEUES],
    pub dev: *mut ublk_dev,
    pub idx: c_ushort,
    pub nr_queues: c_ushort,

    pub state: unsigned,
    pub cmd_inflight: c_uint,
    pub io_inflight: c_uint,
    pub nr_bufs: c_ushort,
    pub auto_buf_stride: c_ushort,
// followings are for BATCH_IO
    pub commit_buf_start: c_ushort,
    pub commit_buf_elem_size: c_uchar,
//
// We just support single device, so pre-calculate commit/prep flags
//
    pub cmd_flags: c_ushort,
    pub nr_commit_buf: c_uint,
    pub commit_buf_size: c_uint,
    pub commit_buf: *mut c_void,

    pub commit_buf_alloc: allocator,
    pub commit: *mut batch_commit_buf,
// FETCH_IO_CMDS buffer
    pub nr_fetch_bufs: c_ushort,
    pub fetch: *mut batch_fetch_buf,
    pub ring: io_uring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_dev {
    pub tgt: ublk_tgt,
    pub dev_info: ublksrv_ctrl_dev_info,
    pub q: [ublk_queue; UBLK_MAX_QUEUES],
    pub nthreads: unsigned,
    pub per_io_tasks: unsigned,
    pub /: *mut *mut int fds[MAX_BACK_FILES + 1]; / fds[0] points to /dev/ublkcN,
    pub nr_fds: c_int,
    pub ctrl_fd: c_int,
    pub ring: io_uring,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn ublk_queue_io_cmd(t: *mut ublk_thread, io: *mut ublk_io) -> c_int;
}
extern "C" {
    pub fn __ublk_use_batch_io(_arg: q->flags) -> return;
}
extern "C" {
    pub fn __ublk_use_batch_io(_arg: dev->dev_info.flags) -> return;
}
// only work for handle single device in this pthread context
// All targets currently use interval_exp = logical_bs_shift = 9
// we only have 7 bits to encode q_id
extern "C" {
    pub fn _IOC_NR(_arg: op) -> return;
}
extern "C" {
    pub fn container_of(_arg: io, ublk_queue: struct, _arg: ios[io->tag]) -> return;
}
// Return the raw ublk FD for index 0
// Adjust index for backing files (index 1 becomes 0, etc.)
extern "C" {
    pub fn ublk_batch_io_buf_idx(_arg: t, _arg: q, _arg: tag) -> return;
}
extern "C" {
    pub fn ublk_queue_use_zc(ublk_queue_use_auto_zc(q: q) ||) -> return;
}
//
// Each IO's buffer index has to be calculated by this helper for
// UBLKS_T_BATCH_IO
//
extern "C" {
    pub fn ublk_batch_io_buf_idx(_arg: t, _arg: q, _arg: tag) -> return;
}
// Queue UBLK_U_IO_PREP_IO_CMDS for a specific queue with batch elements
extern "C" {
    pub fn ublk_batch_queue_prep_io_cmds(t: *mut ublk_thread, q: *mut ublk_queue) -> c_int;
}
// Start fetching I/O commands using multishot UBLK_U_IO_FETCH_IO_CMDS
extern "C" {
    pub fn ublk_batch_start_fetch(t: *mut ublk_thread);
}
// Handle completion of batch I/O commands (prep/commit)
// Initialize batch I/O state and calculate buffer parameters
extern "C" {
    pub fn ublk_batch_prepare(t: *mut ublk_thread);
}
// Allocate and register commit buffers for batch operations
extern "C" {
    pub fn ublk_batch_alloc_buf(t: *mut ublk_thread) -> c_int;
}
// Free commit buffers and cleanup batch allocator
extern "C" {
    pub fn ublk_batch_free_buf(t: *mut ublk_thread);
}
// Prepare a new commit buffer for batching completed I/O operations
extern "C" {
    pub fn ublk_batch_prep_commit(t: *mut ublk_thread);
}
// Submit UBLK_U_IO_COMMIT_IO_CMDS with batched completed I/O operations
extern "C" {
    pub fn ublk_batch_commit_io_cmds(t: *mut ublk_thread);
}
// Add a completed I/O operation to the current batch commit buffer
extern "C" {
    pub fn ublk_queue_io_cmd(_arg: t, _arg: io) -> return;
}
// shared memory zero-copy support
pub const UBLK_BUF_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ublk_shmem_entry {
    pub fd: c_int,
    pub mmap_base: *mut c_void,
    pub size: usize,
}

extern "C" {
    pub fn backing_file_tgt_deinit(dev: *mut ublk_dev);
}
extern "C" {
    pub fn backing_file_tgt_init(dev: *mut ublk_dev, nr_direct: c_uint) -> c_int;
}
