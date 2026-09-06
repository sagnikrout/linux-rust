//! Automatically rewritten from C Header to Rust Module
//! Source: fs/adfs/dir_fplus.h
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
// linux/fs/adfs/dir_fplus.h
//
// Copyright (C) 1999 Russell King
//
// Structures of directories on the F+ format disk
//
pub const ADFS_FPLUS_NAME_LEN: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_bigdirheader {
    pub startmasseq: __u8,
    pub bigdirversion: [__u8; 3],
    pub bigdirstartname: __le32,
    pub bigdirnamelen: __le32,
    pub bigdirsize: __le32,
    pub bigdirentries: __le32,
    pub bigdirnamesize: __le32,
    pub bigdirparent: __le32,
    pub bigdirname: [c_char; 1],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_bigdirentry {
    pub bigdirload: __le32,
    pub bigdirexec: __le32,
    pub bigdirlen: __le32,
    pub bigdirindaddr: __le32,
    pub bigdirattr: __le32,
    pub bigdirobnamelen: __le32,
    pub bigdirobnameptr: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_bigdirtail {
    pub bigdirendname: __le32,
    pub bigdirendmasseq: __u8,
    pub reserved: [__u8; 2],
    pub bigdircheckbyte: __u8,
// C attribute field omitted
