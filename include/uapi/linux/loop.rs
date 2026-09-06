//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/loop.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// Copyright 1993 by Theodore Ts'o.
//
pub const LO_NAME_SIZE: c_int = 64;
pub const LO_KEY_SIZE: c_int = 32;
//
// Loop flags
//
// LO_FLAGS that can be set using LOOP_SET_STATUS(64)

// LO_FLAGS that can be cleared using LOOP_SET_STATUS(64)

// LO_FLAGS that can be set using LOOP_CONFIGURE

// Backwards compatibility version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loop_info {
    pub /: *mut *mut int lo_number; / ioctl r/o,
    pub /: *mut *mut __kernel_old_dev_t lo_device; / ioctl r/o,
    pub /: *mut *mut unsigned long lo_inode; / ioctl r/o,
    pub /: *mut *mut __kernel_old_dev_t lo_rdevice; / ioctl r/o,
    pub lo_offset: c_int,
    pub /: *mut *mut int lo_encrypt_type; / obsolete, ignored,
    pub /: *mut *mut int lo_encrypt_key_size; / ioctl w/o,
    pub lo_flags: c_int,
    pub lo_name: [c_char; LO_NAME_SIZE],
    pub /: *mut *mut unsigned char lo_encrypt_key[LO_KEY_SIZE]; / ioctl w/o,
    pub lo_init: [c_ulong; 2],
    pub reserved: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loop_info64 {
    pub /: *mut *mut __u64 lo_device; / ioctl r/o,
    pub /: *mut *mut __u64 lo_inode; / ioctl r/o,
    pub /: *mut *mut __u64 lo_rdevice; / ioctl r/o,
    pub lo_offset: __u64,
    pub /: *mut *mut __u64 lo_sizelimit;/ bytes, 0 == max available,
    pub /: *mut *mut __u32 lo_number; / ioctl r/o,
    pub /: *mut *mut __u32 lo_encrypt_type; / obsolete, ignored,
    pub /: *mut *mut __u32 lo_encrypt_key_size; / ioctl w/o,
    pub lo_flags: __u32,
    pub lo_file_name: [__u8; LO_NAME_SIZE],
    pub lo_crypt_name: [__u8; LO_NAME_SIZE],
    pub /: *mut *mut __u8 lo_encrypt_key[LO_KEY_SIZE]; / ioctl w/o,
    pub lo_init: [__u64; 2],
}

//
// struct loop_config - Complete configuration for a loop device.
// @fd: fd of the file to be used as a backing file for the loop device.
// @block_size: block size to use; ignored if 0.
// @info: struct loop_info64 to configure the loop device with.
//
// This structure is used with the LOOP_CONFIGURE ioctl, and can be used to
// atomically setup and configure all loop device parameters at once.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loop_config {
    pub fd: __u32,
    pub block_size: __u32,
    pub info: loop_info64,
    pub __reserved: [__u64; 8],
}

//
// Loop filter types
//
pub const LO_CRYPT_NONE: c_int = 0;
pub const LO_CRYPT_XOR: c_int = 1;
pub const LO_CRYPT_DES: c_int = 2;

pub const LO_CRYPT_BLOW: c_int = 4;
pub const LO_CRYPT_CAST128: c_int = 5;
pub const LO_CRYPT_IDEA: c_int = 6;
pub const LO_CRYPT_DUMMY: c_int = 9;
pub const LO_CRYPT_SKIPJACK: c_int = 10;
pub const LO_CRYPT_CRYPTOAPI: c_int = 18;
pub const MAX_LO_CRYPT: c_int = 20;
//
// IOCTL commands --- we will commandeer 0x4C ('L')
//
pub const LOOP_SET_FD: c_uint = 0x4C00;
pub const LOOP_CLR_FD: c_uint = 0x4C01;
pub const LOOP_SET_STATUS: c_uint = 0x4C02;
pub const LOOP_GET_STATUS: c_uint = 0x4C03;
pub const LOOP_SET_STATUS64: c_uint = 0x4C04;
pub const LOOP_GET_STATUS64: c_uint = 0x4C05;
pub const LOOP_CHANGE_FD: c_uint = 0x4C06;
pub const LOOP_SET_CAPACITY: c_uint = 0x4C07;
pub const LOOP_SET_DIRECT_IO: c_uint = 0x4C08;
pub const LOOP_SET_BLOCK_SIZE: c_uint = 0x4C09;
pub const LOOP_CONFIGURE: c_uint = 0x4C0A;
// /dev/loop-control interface
pub const LOOP_CTL_ADD: c_uint = 0x4C80;
pub const LOOP_CTL_REMOVE: c_uint = 0x4C81;
pub const LOOP_CTL_GET_FREE: c_uint = 0x4C82;
