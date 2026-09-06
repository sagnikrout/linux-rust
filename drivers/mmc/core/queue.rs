//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/queue.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmc_issued {
    MMC_REQ_STARTED,
    MMC_REQ_BUSY,
    MMC_REQ_FAILED_TO_START,
    MMC_REQ_FINISHED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmc_issue_type {
    MMC_ISSUE_SYNC,
    MMC_ISSUE_DCMD,
    MMC_ISSUE_ASYNC,
    MMC_ISSUE_MAX,
}

extern "C" {
    pub fn blk_mq_rq_to_pdu(_arg: rq) -> return;
}
extern "C" {
    pub fn blk_mq_rq_from_pdu(_arg: mqr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_blk_request {
    pub mrq: mmc_request,
    pub sbc: mmc_command,
    pub cmd: mmc_command,
    pub stop: mmc_command,
    pub data: mmc_data,
}

//
// enum mmc_drv_op - enumerates the operations in the mmc_queue_req
// @MMC_DRV_OP_IOCTL: ioctl operation
// @MMC_DRV_OP_IOCTL_RPMB: RPMB-oriented ioctl operation
// @MMC_DRV_OP_BOOT_WP: write protect boot partitions
// @MMC_DRV_OP_GET_CARD_STATUS: get card status
// @MMC_DRV_OP_GET_EXT_CSD: get the EXT CSD from an eMMC card
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmc_drv_op {
    MMC_DRV_OP_IOCTL,
    MMC_DRV_OP_IOCTL_RPMB,
    MMC_DRV_OP_BOOT_WP,
    MMC_DRV_OP_GET_CARD_STATUS,
    MMC_DRV_OP_GET_EXT_CSD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_queue_req {
    pub brq: mmc_blk_request,
    pub sg: *mut scatterlist,
    pub drv_op: mmc_drv_op,
    pub drv_op_result: c_int,
    pub drv_op_data: *mut c_void,
    pub ioc_count: u8,
    pub retries: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_queue {
    pub card: *mut mmc_card,
    pub ctx: mmc_ctx,
    pub tag_set: blk_mq_tag_set,
    pub blkdata: *mut mmc_blk_data,
    pub queue: *mut request_queue,
    pub lock: spinlock_t,
    pub in_flight: [c_int; MMC_ISSUE_MAX],
    pub cqe_busy: c_uint,

    pub busy: bool,
    pub recovery_needed: bool,
    pub in_recovery: bool,
    pub rw_wait: bool,
    pub waiting: bool,
    pub recovery_work: work_struct,
    pub wait: wait_queue_head_t,
    pub recovery_req: *mut request,
    pub complete_req: *mut request,
    pub complete_lock: mutex,
    pub complete_work: work_struct,
}

extern "C" {
    pub fn mmc_cleanup_queue(: *mut mmc_queue);
}
extern "C" {
    pub fn mmc_queue_suspend(: *mut mmc_queue);
}
extern "C" {
    pub fn mmc_queue_resume(: *mut mmc_queue);
}
extern "C" {
    pub fn mmc_cqe_check_busy(mq: *mut mmc_queue);
}
extern "C" {
    pub fn mmc_cqe_recovery_notifier(mrq: *mut mmc_request);
}
extern "C" {
    pub fn mmc_issue_type(mq: *mut mmc_queue, req: *mut request) -> mmc_issue_type;
}
