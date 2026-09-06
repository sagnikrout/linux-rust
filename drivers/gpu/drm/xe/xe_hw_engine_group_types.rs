//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_engine_group_types.h
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
// Copyright © 2024 Intel Corporation
//

//
// enum xe_hw_engine_group_execution_mode - possible execution modes of a hw
// engine group
//
// @EXEC_MODE_LR: execution in long-running mode
// @EXEC_MODE_DMA_FENCE: execution in dma fence mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_hw_engine_group_execution_mode {
    EXEC_MODE_LR,
    EXEC_MODE_DMA_FENCE,
}

//
// struct xe_hw_engine_group - Hardware engine group
//
// hw engines belong to the same group if they share hardware resources in a way
// that prevents them from making progress when one is stuck on a page fault.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_engine_group {
//
// @exec_queue_list: list of exec queues attached to this
// xe_hw_engine_group
//
    pub exec_queue_list: list_head,
// @resume_work: worker to resume faulting LR exec queues
    pub resume_work: work_struct,
// @resume_wq: workqueue to resume faulting LR exec queues
    pub resume_wq: *mut workqueue_struct,
//
// @mode_sem: used to protect this group's hardware resources and ensure
// mutual exclusion between execution only in faulting LR mode and
// execution only in DMA_FENCE mode
//
    pub mode_sem: rw_semaphore,
// @cur_mode: current execution mode of this hw engine group
    pub cur_mode: xe_hw_engine_group_execution_mode,
}
