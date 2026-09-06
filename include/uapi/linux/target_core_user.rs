//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/target_core_user.h
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
// This header will be used by application too

//
// DOC: Ring Design
// Ring Design
// -----------
//
// The mmaped area is divided into three parts:
// 1) The mailbox (struct tcmu_mailbox, below);
// 2) The command ring;
// 3) Everything beyond the command ring (data).
//
// The mailbox tells userspace the offset of the command ring from the
// start of the shared memory region, and how big the command ring is.
//
// The kernel passes SCSI commands to userspace by putting a struct
// tcmu_cmd_entry in the ring, updating mailbox->cmd_head, and poking
// userspace via UIO's interrupt mechanism.
//
// tcmu_cmd_entry contains a header. If the header type is PAD,
// userspace should skip hdr->length bytes (mod cmdr_size) to find the
// next cmd_entry.
//
// Otherwise, the entry will contain offsets into the mmaped area that
// contain the cdb and data buffers -- the latter accessible via the
// iov array. iov addresses are also offsets into the shared area.
//
// When userspace is completed handling the command, set
// entry->rsp.scsi_status, fill in rsp.sense_buffer if appropriate,
// and also set mailbox->cmd_tail equal to the old cmd_tail plus
// hdr->length, mod cmdr_size. If cmd_tail doesn't equal cmd_head, it
// should process the next packet the same way, and so on.
//
pub const TCMU_MAILBOX_VERSION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmu_mailbox {
    pub version: __u16,
    pub flags: __u16,
    pub cmdr_off: __u32,
    pub cmdr_size: __u32,
    pub cmd_head: __u32,
// Updated by user. On its own cacheline
    pub __attribute__((__aligned__(ALIGN_SIZE))): __u32 cmd_tail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcmu_opcode {
    TCMU_OP_PAD = 0,
    TCMU_OP_CMD,
    TCMU_OP_TMR,
}

//
// Only a few opcodes, and length is 8-byte aligned, so use low bits for opcode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmu_cmd_entry_hdr {
    pub len_op: __u32,
    pub cmd_id: __u16,
    pub kflags: __u8,
pub const TCMU_UFLAG_UNKNOWN_OP: c_uint = 0x1;
pub const TCMU_UFLAG_READ_LEN: c_uint = 0x2;
pub const TCMU_UFLAG_KEEP_BUF: c_uint = 0x4;
    pub uflags: __u8,
    pub __packed: },
pub const TCMU_OP_MASK: c_uint = 0x7;
    pub TCMU_OP_MASK: return len_op &,
// len_op &= ~TCMU_OP_MASK;
// len_op |= (op & TCMU_OP_MASK);
    pub ~TCMU_OP_MASK: return len_op &,
// len_op &= TCMU_OP_MASK;
// len_op |= len;
// Currently the same as SCSI_SENSE_BUFFERSIZE
pub const TCMU_SENSE_BUFFERSIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmu_cmd_entry {
    pub hdr: tcmu_cmd_entry_hdr,
    pub iov_cnt: __u32,
    pub iov_bidi_cnt: __u32,
    pub iov_dif_cnt: __u32,
    pub cdb_off: __u64,
    pub __pad1: __u64,
    pub __pad2: __u64,
    pub iov): __DECLARE_FLEX_ARRAY(struct iovec,,
    pub req: },
    pub scsi_status: __u8,
    pub __pad1: __u8,
    pub __pad2: __u16,
    pub read_len: __u32,
    pub sense_buffer: [c_char; TCMU_SENSE_BUFFERSIZE],
    pub rsp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcmu_tmr_entry {
    pub hdr: tcmu_cmd_entry_hdr,
pub const TCMU_TMR_UNKNOWN: c_int = 0;
pub const TCMU_TMR_ABORT_TASK: c_int = 1;
pub const TCMU_TMR_ABORT_TASK_SET: c_int = 2;
pub const TCMU_TMR_CLEAR_ACA: c_int = 3;
pub const TCMU_TMR_CLEAR_TASK_SET: c_int = 4;
pub const TCMU_TMR_LUN_RESET: c_int = 5;
pub const TCMU_TMR_TARGET_WARM_RESET: c_int = 6;
pub const TCMU_TMR_TARGET_COLD_RESET: c_int = 7;
// Pseudo reset due to received PR OUT
pub const TCMU_TMR_LUN_RESET_PRO: c_int = 128;
    pub tmr_type: __u8,
    pub __pad1: __u8,
    pub __pad2: __u16,
    pub cmd_cnt: __u32,
    pub __pad3: __u64,
    pub __pad4: __u64,
    pub cmd_ids: [__u16; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcmu_genl_cmd {
    TCMU_CMD_UNSPEC,
    TCMU_CMD_ADDED_DEVICE,
    TCMU_CMD_REMOVED_DEVICE,
    TCMU_CMD_RECONFIG_DEVICE,
    TCMU_CMD_ADDED_DEVICE_DONE,
    TCMU_CMD_REMOVED_DEVICE_DONE,
    TCMU_CMD_RECONFIG_DEVICE_DONE,
    TCMU_CMD_SET_FEATURES,
    __TCMU_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcmu_genl_attr {
    TCMU_ATTR_UNSPEC,
    TCMU_ATTR_DEVICE,
    TCMU_ATTR_MINOR,
    TCMU_ATTR_PAD,
    TCMU_ATTR_DEV_CFG,
    TCMU_ATTR_DEV_SIZE,
    TCMU_ATTR_WRITECACHE,
    TCMU_ATTR_CMD_STATUS,
    TCMU_ATTR_DEVICE_ID,
    TCMU_ATTR_SUPP_KERN_CMD_REPLY,
    __TCMU_ATTR_MAX,
}

