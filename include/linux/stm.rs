//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stm.h
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
// System Trace Module (STM) infrastructure apis
// Copyright (C) 2014 Intel Corporation.
//

//
// enum stp_packet_type - STP packets that an STM driver sends
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stp_packet_type {
    STP_PACKET_DATA = 0,
    STP_PACKET_FLAG,
    STP_PACKET_USER,
    STP_PACKET_MERR,
    STP_PACKET_GERR,
    STP_PACKET_TRIG,
    STP_PACKET_XSYNC,
}

//
// enum stp_packet_flags - STP packet modifiers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stp_packet_flags {
    STP_PACKET_MARKED	= 0x1,
    STP_PACKET_TIMESTAMPED	= 0x2,
}

//
// enum stm_source_type - STM source driver
// @STM_USER: any STM trace source
// @STM_FTRACE: ftrace STM source
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm_source_type {
    STM_USER,
    STM_FTRACE,
}

//
// struct stm_data - STM device description and callbacks
// @name:		device name
// @stm:		internal structure, only used by stm class code
// @sw_start:		first STP master available to software
// @sw_end:		last STP master available to software
// @sw_nchannels:	number of STP channels per master
// @sw_mmiosz:		size of one channel's IO space, for mmap, optional
// @hw_override:	masters in the STP stream will not match the ones
// assigned by software, but are up to the STM hardware
// @packet:		callback that sends an STP packet
// @mmio_addr:		mmap callback, optional
// @link:		called when a new stm_source gets linked to us, optional
// @unlink:		likewise for unlinking, again optional
// @set_options:	set device-specific options on a channel
//
// Fill out this structure before calling stm_register_device() to create
// an STM device and stm_unregister_device() to destroy it. It will also be
// passed back to @packet(), @mmio_addr(), @link(), @unlink() and @set_options()
// callbacks.
//
// Normally, an STM device will have a range of masters available to software
// and the rest being statically assigned to various hardware trace sources.
// The former is defined by the range [@sw_start..@sw_end] of the device
// description. That is, the lowest master that can be allocated to software
// writers is @sw_start and data from this writer will appear is @sw_start
// master in the STP stream.
//
// The @packet callback should adhere to the following rules:
// 1) it must return the number of bytes it consumed from the payload;
// 2) therefore, if it sent a packet that does not have payload (like FLAG),
// it must return zero;
// 3) if it does not support the requested packet type/flag combination,
// it must return -ENOTSUPP.
//
// The @unlink callback is called when there are no more active writers so
// that the master/channel can be quiesced.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_data {
    pub name: *const c_char,
    pub stm: *mut stm_device,
    pub sw_start: c_uint,
    pub sw_end: c_uint,
    pub sw_nchannels: c_uint,
    pub sw_mmiosz: c_uint,
    pub hw_override: c_uint,
    pub ): *const c_uchar,
    pub int): unsigned int, unsigned,
    pub int): unsigned,
    pub int): unsigned,
    pub long): unsigned,
}

extern "C" {
    pub fn stm_unregister_device(stm_data: *mut stm_data);
}
//
// struct stm_source_data - STM source device description and callbacks
// @name:	device name, will be used for policy lookup
// @src:	internal structure, only used by stm class code
// @nr_chans:	number of channels to allocate
// @type:	type of STM source driver represented by stm_source_type
// @link:	called when this source gets linked to an STM device
// @unlink:	called when this source is about to get unlinked from its STM
//
// Fill in this structure before calling stm_source_register_device() to
// register a source device. Also pass it to unregister and write calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_source_data {
    pub name: *const c_char,
    pub src: *mut stm_source_device,
    pub percpu: c_uint,
    pub nr_chans: c_uint,
    pub type: c_uint,
    pub data): *mut *mut int (link)(struct stm_source_data,
    pub data): *mut *mut void (unlink)(struct stm_source_data,
}

extern "C" {
    pub fn stm_source_unregister_device(data: *mut stm_source_data);
}
