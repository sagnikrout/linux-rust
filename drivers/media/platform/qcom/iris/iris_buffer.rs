//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_buffer.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// enum iris_buffer_type
//
// @BUF_INPUT: input buffer to the iris hardware
// @BUF_OUTPUT: output buffer from the iris hardware
// @BUF_BIN: buffer to store intermediate bin data
// @BUF_ARP: buffer for auto register programming
// @BUF_COMV: buffer to store colocated motion vectors
// @BUF_NON_COMV: buffer to hold config data for HW
// @BUF_LINE: buffer to store decoding/encoding context data for HW
// @BUF_DPB: buffer to store display picture buffers for reference
// @BUF_PERSIST: buffer to store session context data
// @BUF_SCRATCH_1: buffer to store decoding/encoding context data for HW
// @BUF_SCRATCH_2: buffer to store encoding context data for HW
// @BUF_VPSS: buffer to store VPSS context data for HW
// @BUF_PARTIAL: buffer for AV1 IBC data
// @BUF_TYPE_MAX: max buffer types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_buffer_type {
    BUF_INPUT = 1,
    BUF_OUTPUT,
    BUF_BIN,
    BUF_ARP,
    BUF_COMV,
    BUF_NON_COMV,
    BUF_LINE,
    BUF_DPB,
    BUF_PERSIST,
    BUF_SCRATCH_1,
    BUF_SCRATCH_2,
    BUF_VPSS,
    BUF_PARTIAL,
    BUF_TYPE_MAX,
}

//
// enum iris_buffer_attributes
//
// BUF_ATTR_DEFERRED: buffer queued by client but not submitted to firmware.
// BUF_ATTR_PENDING_RELEASE: buffers requested to be released from firmware.
// BUF_ATTR_QUEUED: buffers submitted to firmware.
// BUF_ATTR_DEQUEUED: buffers received from firmware.
// BUF_ATTR_BUFFER_DONE: buffers sent back to vb2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_buffer_attributes {
    BUF_ATTR_DEFERRED		= BIT(0),
    BUF_ATTR_PENDING_RELEASE	= BIT(1),
    BUF_ATTR_QUEUED			= BIT(2),
    BUF_ATTR_DEQUEUED		= BIT(3),
    BUF_ATTR_BUFFER_DONE		= BIT(4),
}

//
// struct iris_buffer
//
// @vb2: v4l2 vb2 buffer
// @list: list head for the iris_buffers structure
// @inst: iris instance structure
// @type: enum for type of iris buffer
// @index: identifier for the iris buffer
// @fd: file descriptor of the buffer
// @buffer_size: accessible buffer size in bytes starting from addr_offset
// @data_offset: accessible buffer offset from base address
// @data_size: data size in bytes
// @device_addr: device address of the buffer
// @kvaddr: kernel virtual address of the buffer
// @dma_attrs: dma attributes
// @flags: buffer flags. It is represented as bit masks.
// @timestamp: timestamp of the buffer in nano seconds (ns)
// @attr: enum for iris buffer attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_buffer {
    pub vb2: vb2_v4l2_buffer,
    pub list: list_head,
    pub inst: *mut iris_inst,
    pub type: iris_buffer_type,
    pub index: u32,
    pub fd: c_int,
    pub buffer_size: usize,
    pub data_offset: u32,
    pub data_size: usize,
    pub device_addr: dma_addr_t,
    pub kvaddr: *mut c_void,
    pub dma_attrs: c_ulong,
    pub /: *mut *mut *mut u32 flags; / V4L2_BUF_FLAG_,
    pub timestamp: u64,
    pub attr: iris_buffer_attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_buffers {
    pub list: list_head,
    pub min_count: u32,
    pub size: u32,
}

extern "C" {
    pub fn iris_get_buffer_size(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> c_int;
}
extern "C" {
    pub fn iris_get_internal_buffers(inst: *mut iris_inst, plane: u32);
}
extern "C" {
    pub fn iris_create_internal_buffers(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_queue_internal_buffers(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_queue_internal_deferred_buffers(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> c_int;
}
extern "C" {
    pub fn iris_destroy_internal_buffer(inst: *mut iris_inst, buffer: *mut iris_buffer);
}
extern "C" {
    pub fn iris_destroy_all_internal_buffers(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_destroy_dequeued_internal_buffers(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_alloc_and_queue_persist_bufs(inst: *mut iris_inst, buf_type: iris_buffer_type) -> c_int;
}
extern "C" {
    pub fn iris_alloc_and_queue_input_int_bufs(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_queue_buffer(inst: *mut iris_inst, buf: *mut iris_buffer) -> c_int;
}
extern "C" {
    pub fn iris_queue_deferred_buffers(inst: *mut iris_inst, buf_type: iris_buffer_type) -> c_int;
}
extern "C" {
    pub fn iris_vb2_buffer_done(inst: *mut iris_inst, buf: *mut iris_buffer) -> c_int;
}
extern "C" {
    pub fn iris_vb2_queue_error(inst: *mut iris_inst);
}
