//! Automatically rewritten from C Header to Rust Module
//! Source: fs/adfs/dir_f.h
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
// linux/fs/adfs/dir_f.h
//
// Copyright (C) 1999 Russell King
//
// Structures of directories on the F format disk
//
// Directory header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_dirheader {
    pub startmasseq: __u8,
    pub startname: [__u8; 4],
    pub __attribute__((packed)): },
pub const ADFS_NEWDIR_SIZE: c_int = 2048;
pub const ADFS_NUM_DIR_ENTRIES: c_int = 77;
//
// Directory entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_direntry {
pub const ADFS_F_NAME_LEN: c_int = 10;
    pub dirobname: [c_char; ADFS_F_NAME_LEN],
    pub dirload: [__u8; 4],
    pub direxec: [__u8; 4],
    pub dirlen: [__u8; 4],
    pub dirinddiscadd: [__u8; 3],
    pub newdiratts: __u8,
    pub __attribute__((packed)): },
//
// Directory tail
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_olddirtail {
    pub dirlastmask: __u8,
    pub dirname: [c_char; 10],
    pub dirparent: [__u8; 3],
    pub dirtitle: [c_char; 19],
    pub reserved: [__u8; 14],
    pub endmasseq: __u8,
    pub endname: [__u8; 4],
    pub dircheckbyte: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_newdirtail {
    pub dirlastmask: __u8,
    pub reserved: [__u8; 2],
    pub dirparent: [__u8; 3],
    pub dirtitle: [c_char; 19],
    pub dirname: [c_char; 10],
    pub endmasseq: __u8,
    pub endname: [__u8; 4],
    pub dircheckbyte: __u8,
    pub __attribute__((packed)): },
