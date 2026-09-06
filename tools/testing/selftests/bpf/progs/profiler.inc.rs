//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/profiler.inc.h
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
// Copyright (c) 2020 Facebook

pub const NULL: c_int = 0;

pub const O_WRONLY: c_int = 00000001;
pub const O_RDWR: c_int = 00000002;
pub const O_DIRECTORY: c_int = 00200000;
pub const __O_TMPFILE: c_int = 020000000;

pub const S_IFMT: c_int = 00170000;
pub const S_IFSOCK: c_int = 0140000;
pub const S_IFLNK: c_int = 0120000;
pub const S_IFREG: c_int = 0100000;
pub const S_IFBLK: c_int = 0060000;
pub const S_IFDIR: c_int = 0040000;
pub const S_IFCHR: c_int = 0020000;
pub const S_IFIFO: c_int = 0010000;
pub const S_ISUID: c_int = 0004000;
pub const S_ISGID: c_int = 0002000;
pub const S_ISVTX: c_int = 0001000;

pub const KILL_DATA_ARRAY_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_kill_data_arr_t {
    pub array: [var_kill_data_t; KILL_DATA_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union any_profiler_data_t {
    pub var_exec: var_exec_data_t,
    pub var_kill: var_kill_data_t,
    pub var_sysctl: var_sysctl_data_t,
    pub var_filemod: var_filemod_data_t,
    pub var_fork: var_fork_data_t,
    pub var_kill_data_arr: var_kill_data_arr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_iattrs___52 {
    pub ia_iattr: iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernfs_node___52 {
    pub ino: u32,
    pub generation: u32,
}

extern "C" {
    pub fn IS_ERR_VALUE(long)ptr: (unsigned) -> return;
}

// root_pos = payload - payload_start;
extern "C" {
    pub fn BPF_CORE_READ(_arg: node52, _arg: id.ino) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_subsys_id___local {
    pids_cgrp_id___local = 123, /* value doesn't matter */
}

// device_id = dev_id;
// file_ino = ino;

