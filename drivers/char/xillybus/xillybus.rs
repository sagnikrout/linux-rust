//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/xillybus/xillybus.h
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
// linux/drivers/misc/xillybus.h
//
// Copyright 2011 Xillybus Ltd, http://xillybus.com
//
// Header file for the Xillybus FPGA/host framework.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilly_buffer {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut int end_offset; / Counting elements, not bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilly_idt_handle {
    pub chandesc: *mut c_uchar,
    pub names: *mut c_uchar,
    pub names_len: c_int,
    pub entries: c_int,
}

//
// Read-write confusion: wr_* and rd_* notation sticks to FPGA view, so
// wr_* buffers are those consumed by read(), since the FPGA writes to them
// and vice versa.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilly_channel {
    pub endpoint: *mut xilly_endpoint,
    pub chan_num: c_int,
    pub log2_element_size: c_int,
    pub seekable: c_int,
    pub /: *mut *mut *mut *mut xilly_buffer wr_buffers; / FPGA writes, driver reads!,
    pub num_wr_buffers: c_int,
    pub /: *mut *mut unsigned int wr_buf_size; / In bytes,
    pub wr_fpga_buf_idx: c_int,
    pub wr_host_buf_idx: c_int,
    pub wr_host_buf_pos: c_int,
    pub wr_empty: c_int,
    pub /: *mut *mut int wr_ready; / Significant only when wr_empty == 1,
    pub wr_sleepy: c_int,
    pub wr_eof: c_int,
    pub wr_hangup: c_int,
    pub wr_spinlock: spinlock_t,
    pub wr_mutex: mutex,
    pub wr_wait: wait_queue_head_t,
    pub wr_ready_wait: wait_queue_head_t,
    pub wr_ref_count: c_int,
    pub wr_synchronous: c_int,
    pub wr_allow_partial: c_int,
    pub wr_exclusive_open: c_int,
    pub wr_supports_nonempty: c_int,
    pub /: *mut *mut *mut *mut xilly_buffer rd_buffers; / FPGA reads, driver writes!,
    pub num_rd_buffers: c_int,
    pub /: *mut *mut unsigned int rd_buf_size; / In bytes,
    pub rd_fpga_buf_idx: c_int,
    pub rd_host_buf_pos: c_int,
    pub rd_host_buf_idx: c_int,
    pub rd_full: c_int,
    pub rd_spinlock: spinlock_t,
    pub rd_mutex: mutex,
    pub rd_wait: wait_queue_head_t,
    pub rd_ref_count: c_int,
    pub rd_allow_partial: c_int,
    pub rd_synchronous: c_int,
    pub rd_exclusive_open: c_int,
    pub rd_workitem: delayed_work,
    pub rd_leftovers: [c_uchar; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilly_endpoint {
    pub dev: *mut device,
    pub owner: *mut module,
    pub /: *mut *mut int dma_using_dac; / =1 if 64-bit DMA is used, =0 otherwise.,
    pub registers: *mut __iomem void,
    pub fatal_error: c_int,
    pub register_mutex: mutex,
    pub ep_wait: wait_queue_head_t,
    pub /: *mut *mut int num_channels; / EXCLUDING message buffer,
    pub channels: *mut xilly_channel,
    pub msg_counter: c_int,
    pub failed_messages: c_int,
    pub idtlen: c_int,
    pub msgbuf_addr: *mut u32,
    pub msgbuf_dma_addr: dma_addr_t,
    pub msg_buf_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilly_mapping {
    pub device: *mut device,
    pub dma_addr: dma_addr_t,
    pub size: usize,
    pub direction: c_int,
}

extern "C" {
    pub fn xillybus_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn xillybus_endpoint_discovery(endpoint: *mut xilly_endpoint) -> c_int;
}
extern "C" {
    pub fn xillybus_endpoint_remove(endpoint: *mut xilly_endpoint);
}
