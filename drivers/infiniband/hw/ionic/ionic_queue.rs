//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ionic/ionic_queue.h
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
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

pub const IONIC_MAX_DEPTH: c_uint = 0xffff;
pub const IONIC_MAX_CQ_DEPTH: c_uint = 0xffff;

//
// struct ionic_queue - Ring buffer used between device and driver
// @size:	Size of the buffer, in bytes
// @dma:	Dma address of the buffer
// @ptr:	Buffer virtual address
// @prod:	Driver position in the queue
// @cons:	Device position in the queue
// @mask:	Capacity of the queue, subtracting the hole
// This value is equal to ((1 << depth_log2) - 1)
// @depth_log2: Log base two size depth of the queue
// @stride_log2: Log base two size of an element in the queue
// @dbell:	Doorbell identifying bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_queue {
    pub size: usize,
    pub dma: dma_addr_t,
    pub ptr: *mut c_void,
    pub prod: u16,
    pub cons: u16,
    pub mask: u16,
    pub depth_log2: u8,
    pub stride_log2: u8,
    pub dbell: u64,
}

//
// ionic_queue_init() - Initialize user space queue
// @q:		Uninitialized queue structure
// @dma_dev:	DMA device for mapping
// @depth:	Depth of the queue
// @stride:	Size of each element of the queue
//
// Return: status code
//
// ionic_queue_destroy() - Destroy user space queue
// @q:		Queue structure
// @dma_dev:	DMA device for mapping
//
// Return: status code
//
extern "C" {
    pub fn ionic_queue_destroy(q: *mut ionic_queue, dma_dev: *mut device);
}
//
// ionic_queue_empty() - Test if queue is empty
// @q:		Queue structure
//
// This is only valid for to-device queues.
//
// Return: is empty
//
// ionic_queue_length() - Get the current length of the queue
// @q:		Queue structure
//
// This is only valid for to-device queues.
//
// Return: length
//
// ionic_queue_length_remaining() - Get the remaining length of the queue
// @q:		Queue structure
//
// This is only valid for to-device queues.
//
// Return: length remaining
//
// ionic_queue_full() - Test if queue is full
// @q:		Queue structure
//
// This is only valid for to-device queues.
//
// Return: is full
//
// ionic_color_wrap() - Flip the color if prod is wrapped
// @prod:	Queue index just after advancing
// @color:	Queue color just prior to advancing the index
//
// Return: color after advancing the index
//
// logical xor color with (prod == 0)
//
// ionic_queue_at() - Get the element at the given index
// @q:		Queue structure
// @idx:	Index in the queue
//
// The index must be within the bounds of the queue.  It is not checked here.
//
// Return: pointer to element at index
//
// ionic_queue_at_prod() - Get the element at the producer index
// @q:		Queue structure
//
// Return: pointer to element at producer index
//
extern "C" {
    pub fn ionic_queue_at(_arg: q, _arg: q->prod) -> return;
}
//
// ionic_queue_at_cons() - Get the element at the consumer index
// @q:		Queue structure
//
// Return: pointer to element at consumer index
//
extern "C" {
    pub fn ionic_queue_at(_arg: q, _arg: q->cons) -> return;
}
//
// ionic_queue_next() - Compute the next index
// @q:		Queue structure
// @idx:	Index
//
// Return: next index after idx
//
// ionic_queue_produce() - Increase the producer index
// @q:		Queue structure
//
// Caller must ensure that the queue is not full.  It is not checked here.
//
// ionic_queue_consume() - Increase the consumer index
// @q:		Queue structure
//
// Caller must ensure that the queue is not empty.  It is not checked here.
//
// This is only valid for to-device queues.
//
// ionic_queue_consume_entries() - Increase the consumer index by entries
// @q:				Queue structure
// @entries:		Number of entries to increment
//
// Caller must ensure that the queue is not empty.  It is not checked here.
//
// This is only valid for to-device queues.
//
// ionic_queue_dbell_init() - Initialize doorbell bits for queue id
// @q:		Queue structure
// @qid:	Queue identifying number
//
// ionic_queue_dbell_val() - Get current doorbell update value
// @q:		Queue structure
//
// Return: current doorbell update value
//
