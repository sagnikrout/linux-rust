//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/buffer-dma.h
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
// Copyright 2013-2015 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

//
// enum iio_block_state - State of a struct iio_dma_buffer_block
// @IIO_BLOCK_STATE_QUEUED: Block is on the incoming queue
// @IIO_BLOCK_STATE_ACTIVE: Block is currently being processed by the DMA
// @IIO_BLOCK_STATE_DONE: Block is on the outgoing queue
// @IIO_BLOCK_STATE_DEAD: Block has been marked as to be freed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_block_state {
    IIO_BLOCK_STATE_QUEUED,
    IIO_BLOCK_STATE_ACTIVE,
    IIO_BLOCK_STATE_DONE,
    IIO_BLOCK_STATE_DEAD,
}

//
// struct iio_dma_buffer_block - IIO buffer block
// @head: List head
// @size: Total size of the block in bytes
// @bytes_used: Number of bytes that contain valid data
// @vaddr: Virutal address of the blocks memory
// @phys_addr: Physical address of the blocks memory
// @queue: Parent DMA buffer queue
// @kref: kref used to manage the lifetime of block
// @state: Current state of the block
// @cyclic: True if this is a cyclic buffer
// @fileio: True if this buffer is used for fileio mode
// @sg_table: DMA table for the transfer when transferring a DMABUF
// @fence: DMA fence to be signaled when a DMABUF transfer is complete
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dma_buffer_block {
// May only be accessed by the owner of the block
    pub head: list_head,
    pub bytes_used: usize,
//
// Set during allocation, constant thereafter. May be accessed read-only
// by anybody holding a reference to the block.
//
    pub vaddr: *mut c_void,
    pub phys_addr: dma_addr_t,
    pub size: usize,
    pub queue: *mut iio_dma_buffer_queue,
// Must not be accessed outside the core.
    pub kref: kref,
//
// Must not be accessed outside the core. Access needs to hold
// queue->list_lock if the block is not owned by the core.
//
    pub state: iio_block_state,
    pub cyclic: bool,
    pub fileio: bool,
    pub sg_table: *mut sg_table,
    pub fence: *mut dma_fence,
}

//
// struct iio_dma_buffer_queue_fileio - FileIO state for the DMA buffer
// @blocks: Buffer blocks used for fileio
// @active_block: Block being used in read()
// @pos: Read offset in the active block
// @block_size: Size of each block
// @next_dequeue: index of next block that will be dequeued
// @enabled: Whether the buffer is operating in fileio mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dma_buffer_queue_fileio {
    pub blocks: [*mut iio_dma_buffer_block; 2],
    pub active_block: *mut iio_dma_buffer_block,
    pub pos: usize,
    pub block_size: usize,
    pub next_dequeue: c_uint,
    pub enabled: bool,
}

//
// struct iio_dma_buffer_queue - DMA buffer base structure
// @buffer: IIO buffer base structure
// @dev: Parent device
// @ops: DMA buffer callbacks
// @lock: Protects the incoming list, active and the fields in the fileio
// substruct
// @list_lock: Protects lists that contain blocks which can be modified in
// atomic context as well as blocks on those lists. This is the outgoing queue
// list and typically also a list of active blocks in the part that handles
// the DMA controller
// @incoming: List of buffers on the incoming queue
// @active: Whether the buffer is currently active
// @num_dmabufs: Total number of DMABUFs attached to this queue
// @fileio: FileIO state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dma_buffer_queue {
    pub buffer: iio_buffer,
    pub dev: *mut device,
    pub ops: *const iio_dma_buffer_ops,
//
// A mutex to protect accessing, configuring (eg: enqueuing DMA blocks)
// and do file IO on struct iio_dma_buffer_queue objects.
//
    pub lock: mutex,
// A spin lock to protect adding/removing blocks to the queue list
    pub list_lock: spinlock_t,
    pub incoming: list_head,
    pub active: bool,
    pub num_dmabufs: core::sync::atomic::AtomicI32,
    pub fileio: iio_dma_buffer_queue_fileio,
}

//
// struct iio_dma_buffer_ops - DMA buffer callback operations
// @submit: Called when a block is submitted to the DMA controller
// @abort: Should abort all pending transfers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dma_buffer_ops {
    pub block): *mut iio_dma_buffer_block,
    pub queue): *mut *mut void (abort)(struct iio_dma_buffer_queue,
}

extern "C" {
    pub fn iio_dma_buffer_block_done(block: *mut iio_dma_buffer_block);
}
extern "C" {
    pub fn iio_dma_buffer_enable(buffer: *mut iio_buffer, indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_dma_buffer_usage(buffer: *mut iio_buffer) -> usize;
}
extern "C" {
    pub fn iio_dma_buffer_set_bytes_per_datum(buffer: *mut iio_buffer, bpd: usize) -> c_int;
}
extern "C" {
    pub fn iio_dma_buffer_set_length(buffer: *mut iio_buffer, length: c_uint) -> c_int;
}
extern "C" {
    pub fn iio_dma_buffer_request_update(buffer: *mut iio_buffer) -> c_int;
}
extern "C" {
    pub fn iio_dma_buffer_exit(queue: *mut iio_dma_buffer_queue);
}
extern "C" {
    pub fn iio_dma_buffer_release(queue: *mut iio_dma_buffer_queue);
}
extern "C" {
    pub fn iio_dma_buffer_lock_queue(buffer: *mut iio_buffer);
}
extern "C" {
    pub fn iio_dma_buffer_unlock_queue(buffer: *mut iio_buffer);
}
