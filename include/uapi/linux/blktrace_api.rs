//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/blktrace_api.h
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

//
// Trace categories
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blktrace_cat {
    BLK_TC_READ	= 1 << 0,	/* reads */
    BLK_TC_WRITE	= 1 << 1,	/* writes */
    BLK_TC_FLUSH	= 1 << 2,	/* flush */
    BLK_TC_SYNC	= 1 << 3,	/* sync IO */
    BLK_TC_SYNCIO	= BLK_TC_SYNC,
    BLK_TC_QUEUE	= 1 << 4,	/* queueing/merging */
    BLK_TC_REQUEUE	= 1 << 5,	/* requeueing */
    BLK_TC_ISSUE	= 1 << 6,	/* issue */
    BLK_TC_COMPLETE	= 1 << 7,	/* completions */
    BLK_TC_FS	= 1 << 8,	/* fs requests */
    BLK_TC_PC	= 1 << 9,	/* pc requests */
    BLK_TC_NOTIFY	= 1 << 10,	/* special message */
    BLK_TC_AHEAD	= 1 << 11,	/* readahead */
    BLK_TC_META	= 1 << 12,	/* metadata */
    BLK_TC_DISCARD	= 1 << 13,	/* discard requests */
    BLK_TC_DRV_DATA	= 1 << 14,	/* binary per-driver data */
    BLK_TC_FUA	= 1 << 15,	/* fua requests */

    BLK_TC_END_V1	= 1 << 15,	/* we've run out of bits! */

    BLK_TC_ZONE_APPEND	= 1ull << 16,  	/* zone append */
    BLK_TC_ZONE_RESET	= 1ull << 17,	/* zone reset */
    BLK_TC_ZONE_RESET_ALL	= 1ull << 18,	/* zone reset all */
    BLK_TC_ZONE_FINISH	= 1ull << 19,	/* zone finish */
    BLK_TC_ZONE_OPEN	= 1ull << 20,	/* zone open */
    BLK_TC_ZONE_CLOSE	= 1ull << 21,	/* zone close */

    BLK_TC_WRITE_ZEROES	= 1ull << 22,	/* write-zeroes */

    BLK_TC_END_V2		= 1ull << 22,
}

//
// Basic trace actions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blktrace_act {
    __BLK_TA_QUEUE = 1,		/* queued */
    __BLK_TA_BACKMERGE,		/* back merged to existing rq */
    __BLK_TA_FRONTMERGE,		/* front merge to existing rq */
    __BLK_TA_GETRQ,			/* allocated new request */
    __BLK_TA_SLEEPRQ,		/* sleeping on rq allocation */
    __BLK_TA_REQUEUE,		/* request requeued */
    __BLK_TA_ISSUE,			/* sent to driver */
    __BLK_TA_COMPLETE,		/* completed by driver */
    __BLK_TA_PLUG,			/* queue was plugged */
    __BLK_TA_UNPLUG_IO,		/* queue was unplugged by io */
    __BLK_TA_UNPLUG_TIMER,		/* queue was unplugged by timer */
    __BLK_TA_INSERT,		/* insert request */
    __BLK_TA_SPLIT,			/* bio was split */
    __BLK_TA_BOUNCE,		/* unused, was: bio was bounced */
    __BLK_TA_REMAP,			/* bio was remapped */
    __BLK_TA_ABORT,			/* request aborted */
    __BLK_TA_DRV_DATA,		/* driver-specific binary data */
    __BLK_TA_ZONE_PLUG,		/* zone write plug was plugged */
    __BLK_TA_ZONE_UNPLUG,		/* zone write plug was unplugged */
    __BLK_TA_CGROUP = 1 << 8,	/* from a cgroup*/
}

//
// Notify events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blktrace_notify {
    __BLK_TN_PROCESS = 0,		/* establish pid/name mapping */
    __BLK_TN_TIMESTAMP,		/* include system clock */
    __BLK_TN_MESSAGE,		/* Character string message */
    __BLK_TN_CGROUP = __BLK_TA_CGROUP, /* from a cgroup */
}

//
// Trace actions in full. Additionally, read or write is masked
//

pub const BLK_IO_TRACE_MAGIC: c_uint = 0x65617400;
pub const BLK_IO_TRACE_VERSION: c_uint = 0x07;
pub const BLK_IO_TRACE2_VERSION: c_uint = 0x08;
//
// The trace itself
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_io_trace {
    pub /: *mut *mut __u32 magic; / MAGIC << 8 | version,
    pub /: *mut *mut __u32 sequence; / event number,
    pub /: *mut *mut __u64 time; / in nanoseconds,
    pub /: *mut *mut __u64 sector; / disk offset,
    pub /: *mut *mut __u32 bytes; / transfer length,
    pub /: *mut *mut __u32 action; / what happened,
    pub /: *mut *mut __u32 pid; / who did it,
    pub /: *mut *mut __u32 device; / device number,
    pub /: *mut *mut __u32 cpu; / on what cpu did it happen,
    pub /: *mut *mut __u16 error; / completion error,
    pub /: *mut *mut __u16 pdu_len; / length of data after this trace,
// cgroup id will be stored here if exists
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_io_trace2 {
    pub /: *mut *mut __u32 magic; / MAGIC << 8 | BLK_IO_TRACE2_VERSION,
    pub /: *mut *mut __u32 sequence; / event number,
    pub /: *mut *mut __u64 time; / in nanoseconds,
    pub /: *mut *mut __u64 sector; / disk offset,
    pub /: *mut *mut __u32 bytes; / transfer length,
    pub /: *mut *mut __u32 pid; / who did it,
    pub /: *mut *mut __u64 action; / what happened,
    pub /: *mut *mut __u32 device; / device number,
    pub /: *mut *mut __u32 cpu; / on what cpu did it happen,
    pub /: *mut *mut __u16 error; / completion error,
    pub /: *mut *mut __u16 pdu_len; / length of data after this trace,
    pub pad: [__u8; 12],
// cgroup id will be stored here if it exists
}

//
// The remap event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_io_trace_remap {
    pub device_from: __be32,
    pub device_to: __be32,
    pub sector_from: __be64,
}

pub const BLKTRACE_BDEV_SIZE: c_int = 32;
pub const BLKTRACE_BDEV_SIZE2: c_int = 64;
//
// User setup structure passed with BLKTRACESETUP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_user_trace_setup {
    pub /: *mut *mut char name[BLKTRACE_BDEV_SIZE]; / output,
    pub /: *mut *mut __u16 act_mask; / input,
    pub /: *mut *mut __u32 buf_size; / input,
    pub /: *mut *mut __u32 buf_nr; / input,
    pub start_lba: __u64,
    pub end_lba: __u64,
    pub pid: __u32,
}

//
// User setup structure passed with BLKTRACESETUP2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_user_trace_setup2 {
    pub /: *mut *mut char name[BLKTRACE_BDEV_SIZE2]; / output,
    pub /: *mut *mut __u64 act_mask; / input,
    pub /: *mut *mut __u32 buf_size; / input,
    pub /: *mut *mut __u32 buf_nr; / input,
    pub start_lba: __u64,
    pub end_lba: __u64,
    pub pid: __u32,
    pub /: *mut *mut __u32 flags; / currently unused,
    pub reserved: [__u64; 11],
}
