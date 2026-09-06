//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/ssh_msgb.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// SSH message builder functions.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// struct msgbuf - Buffer struct to construct SSH messages.
// @begin: Pointer to the beginning of the allocated buffer space.
// @end:   Pointer to the end (one past last element) of the allocated buffer
// space.
// @ptr:   Pointer to the first free element in the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgbuf {
    pub begin: *mut u8,
    pub end: *mut u8,
    pub ptr: *mut u8,
}

//
// msgb_init() - Initialize the given message buffer struct.
// @msgb: The buffer struct to initialize
// @ptr:  Pointer to the underlying memory by which the buffer will be backed.
// @cap:  Size of the underlying memory.
//
// Initialize the given message buffer struct using the provided memory as
// backing.
//
// msgb_bytes_used() - Return the current number of bytes used in the buffer.
// @msgb: The message buffer.
//
// msgb->ptr = value;
//
// msgb_push_u16() - Push a u16 value to the buffer.
// @msgb:  The message buffer.
// @value: The value to push to the buffer.
//
// msgb_push_syn() - Push SSH SYN bytes to the buffer.
// @msgb: The message buffer.
//
// msgb_push_buf() - Push raw data to the buffer.
// @msgb: The message buffer.
// @buf:  The data to push to the buffer.
// @len:  The length of the data to push to the buffer.
//
// msgb_push_crc() - Compute CRC and push it to the buffer.
// @msgb: The message buffer.
// @buf:  The data for which the CRC should be computed.
// @len:  The length of the data for which the CRC should be computed.
//
// msgb_push_frame() - Push a SSH message frame header to the buffer.
// @msgb: The message buffer
// @ty:   The type of the frame.
// @len:  The length of the payload of the frame.
// @seq:  The sequence ID of the frame/packet.
//
// msgb_push_ack() - Push a SSH ACK frame to the buffer.
// @msgb: The message buffer
// @seq:  The sequence ID of the frame/packet to be ACKed.
//
// SYN.
// ACK-type frame + CRC.
// Payload CRC (ACK-type frames do not have a payload).
//
// msgb_push_nak() - Push a SSH NAK frame to the buffer.
// @msgb: The message buffer
//
// SYN.
// NAK-type frame + CRC.
// Payload CRC (ACK-type frames do not have a payload).
//
// msgb_push_cmd() - Push a SSH command frame with payload to the buffer.
// @msgb: The message buffer.
// @seq:  The sequence ID (SEQ) of the frame/packet.
// @rqid: The request ID (RQID) of the request contained in the frame.
// @rqst: The request to wrap in the frame.
//
// SYN.
// Command frame + CRC.
// Frame payload: Command struct + payload.
// Command payload.
// CRC for command struct + payload.
