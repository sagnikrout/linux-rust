//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/cdma.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Tegra host1x Command DMA
//
// Copyright (c) 2010-2013, NVIDIA Corporation.
//

//
// cdma
//
// This is in charge of a host command DMA channel.
// Sends ops to a push buffer, and takes responsibility for unpinning
// (& possibly freeing) of memory after those ops have completed.
// Producer:
// begin
// push - send ops to the push buffer
// end - start command DMA and enqueue handles to be unpinned
// Consumer:
// update - call to update sync queue and push buffer, unpin memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct push_buffer {
    pub /: *mut *mut *mut void mapped; / mapped pushbuffer memory,
    pub /: *mut *mut dma_addr_t dma; / device address of pushbuffer,
    pub /: *mut *mut dma_addr_t phys; / physical address of pushbuffer,
    pub /: *mut *mut u32 fence; / index we've written,
    pub /: *mut *mut u32 pos; / index to write to,
    pub size: u32,
    pub alloc_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_timeout {
    pub /: *mut *mut delayed_work wq; / work queue,
    pub /: *mut *mut bool initialized; / timer one-time setup flag,
    pub /: *mut *mut *mut host1x_syncpt syncpt; / buffer completion syncpt,
    pub /: *mut *mut u32 syncpt_val; / syncpt value when completed,
    pub /: *mut *mut ktime_t start_ktime; / starting time,
// context timeout information
    pub client: *mut host1x_client,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdma_event {
    CDMA_EVENT_NONE,		/* not waiting for any event */
    CDMA_EVENT_SYNC_QUEUE_EMPTY,	/* wait for empty sync queue */
    CDMA_EVENT_PUSH_BUFFER_SPACE	/* wait for space in push buffer */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_cdma {
    pub /: *mut *mut mutex lock; / controls access to shared state,
    pub /: *mut *mut completion complete; / signalled when event occurs,
    pub /: *mut *mut cdma_event event; / event that complete is waiting for,
    pub /: *mut *mut unsigned int slots_used; / pb slots used in current submit,
    pub /: *mut *mut unsigned int slots_free; / pb slots free in current submit,
    pub /: *mut *mut unsigned int first_get; / DMAGET value, where submit begins,
    pub /: *mut *mut unsigned int last_pos; / last value written to DMAPUT,
    pub /: *mut *mut push_buffer push_buffer; / channel's push buffer,
    pub /: *mut *mut list_head sync_queue; / job queue,
    pub /: *mut *mut buffer_timeout timeout; / channel's timeout state/wq,
    pub running: bool,
    pub torndown: bool,
    pub update_work: work_struct,
}

extern "C" {
    pub fn host1x_cdma_init(cdma: *mut host1x_cdma) -> c_int;
}
extern "C" {
    pub fn host1x_cdma_deinit(cdma: *mut host1x_cdma) -> c_int;
}
extern "C" {
    pub fn host1x_cdma_begin(cdma: *mut host1x_cdma, job: *mut host1x_job) -> c_int;
}
extern "C" {
    pub fn host1x_cdma_push(cdma: *mut host1x_cdma, op1: u32, op2: u32);
}
extern "C" {
    pub fn host1x_cdma_end(cdma: *mut host1x_cdma, job: *mut host1x_job);
}
extern "C" {
    pub fn host1x_cdma_update(cdma: *mut host1x_cdma);
}
