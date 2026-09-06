//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/bcm-vk/bcm_vk_msg.h
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
//
// Copyright 2018-2020 Broadcom.
//

// Single message queue control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_msgq {
    pub /: *mut *mut u16 type; / queue type,
    pub /: *mut *mut u16 num; / queue number,
    pub /: *mut *mut u32 start; / offset in BAR1 where the queue memory starts,
    pub /: *mut *mut u32 rd_idx; / read idx,
    pub /: *mut *mut u32 wr_idx; / write idx,
    pub /*: *mut u32 size;,
// size, which is in number of 16byte blocks,
// to align with the message data structure.
//
    pub /*: *mut u32 nxt;,
// nxt offset to the next msg queue struct.
// This is to provide flexibity for alignment purposes.
//
// Least significant 16 bits in below field hold doorbell register offset
pub const DB_SHIFT: c_int = 16;
    pub /: *mut *mut u32 db_offset; / queue doorbell register offset in BAR0,
    pub rsvd: u32,
}

//
// Structure to record static info from the msgq sync.  We keep local copy
// for some of these variables for both performance + checking purpose.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_sync_qinfo {
    pub q_start: *mut void __iomem,
    pub q_size: u32,
    pub q_mask: u32,
    pub q_low: u32,
    pub q_db_offset: u32,
}

//
// message block - basic unit in the message where a message's size is always
// N x sizeof(basic_block)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vk_msg_blk {
    pub function_id: u8,
pub const VK_FID_TRANS_BUF: c_int = 5;
pub const VK_FID_SHUTDOWN: c_int = 8;
pub const VK_FID_INIT: c_int = 9;
    pub /: *mut *mut u8 size; / size of the message in number of vk_msg_blk's,
    pub /: *mut *mut u16 trans_id; / transport id, queue & msg_id,
    pub context_id: u32,
pub const VK_NEW_CTX: c_int = 0;
    pub cmd: u32,
pub const VK_CMD_PLANES_MASK: c_uint = 0x000f /* number of planes to up/download */;
pub const VK_CMD_UPLOAD: c_uint = 0x0400 /* memory transfer to vk */;
pub const VK_CMD_DOWNLOAD: c_uint = 0x0500 /* memory transfer from vk */;
pub const VK_CMD_MASK: c_uint = 0x0f00 /* command mask */;
    pub arg: u32,
}

// vk_msg_blk is 16 bytes fixed

// shift for fast division of basic msg blk size
pub const VK_MSGQ_BLK_SZ_SHIFT: c_int = 4;
// use msg_id 0 for any simplex host2vk communication
pub const VK_SIMPLEX_MSG_ID: c_int = 0;
// context per session opening of sysfs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_ctx {
    pub /: *mut *mut list_head node; / use for linkage in Hash Table,
    pub idx: c_uint,
    pub in_use: bool,
    pub pid: pid_t,
    pub hash_idx: u32,
    pub /: *mut *mut u32 q_num; / queue number used by the stream,
    pub miscdev: *mut miscdevice,
    pub /: *mut *mut atomic_t pend_cnt; / number of items pending to be read from host,
    pub /: *mut *mut atomic_t dma_cnt; / any dma transaction outstanding,
    pub rd_wq: wait_queue_head_t,
}

// pid hash table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_ht_entry {
    pub head: list_head,
}

// structure for house keeping a single work entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_wkent {
    pub /: *mut *mut list_head node; / for linking purpose,
    pub ctx: *mut bcm_vk_ctx,
// Store up to 4 dma pointers
    pub dma: [bcm_vk_dma; VK_DMA_MAX_ADDRS],
    pub /: *mut *mut u32 to_h_blks; / response,
    pub to_h_msg: *mut vk_msg_blk,
//
// put the to_v_msg at the end so that we could simply append to_v msg
// to the end of the allocated block
//
    pub usr_msg_id: u32,
    pub to_v_blks: u32,
    pub seq_num: u32,
    pub __counted_by(to_v_blks): vk_msg_blk to_v_msg[],
}

// queue stats counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_qs_cnts {
    pub /: *mut *mut u32 cnt; / general counter, used to limit output,
    pub acc_sum: u32,
    pub /: *mut *mut u32 max_occ; / max during a sampling period,
    pub /: *mut *mut u32 max_abs; / the abs max since reset,
}

// control channel structure for either to_v or to_h communication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_msg_chan {
    pub q_nr: u32,
// Mutex to access msgq
    pub msgq_mutex: mutex,
// pointing to BAR locations
    pub msgq: [*mut bcm_vk_msgq __iomem; VK_MSGQ_MAX_NR],
// Spinlock to access pending queue
    pub pendq_lock: spinlock_t,
// for temporary storing pending items, one for each queue
    pub pendq: [list_head; VK_MSGQ_MAX_NR],
// static queue info from the sync
    pub sync_qinfo: [bcm_vk_sync_qinfo; VK_MSGQ_MAX_NR],
}

// totol number of message q allowed by the driver
pub const VK_MSGQ_PER_CHAN_MAX: c_int = 3;

// total number of supported ctx, 32 ctx each for 5 components

// hash table defines to store the opened FDs

// The following are offsets of DDR info provided by the vk card

// shutdown types supported
pub const VK_SHUTDOWN_PID: c_int = 1;
pub const VK_SHUTDOWN_GRACEFUL: c_int = 2;
