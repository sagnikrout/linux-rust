//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/nfs4idmap.h
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


//
// fs/nfs/nfs4idmap.h
//
// UID and GID to name mapping for clients.
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Marius Aamodt Eriksen <marius@umich.edu>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of the University nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// Forward declaration to make this header independent of others
extern "C" {
    pub fn nfs_idmap_init() -> c_int;
}
extern "C" {
    pub fn nfs_idmap_quit();
}
extern "C" {
    pub fn nfs_idmap_new(: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs_idmap_delete(: *mut nfs_client);
}
extern "C" {
    pub fn nfs_fattr_free_names(: *mut nfs_fattr);
}
extern "C" {
    pub fn nfs_fattr_map_and_free_names(: *mut nfs_server, : *mut nfs_fattr);
}
extern "C" {
    pub fn nfs_map_name_to_uid(: *const nfs_server, : *const c_char, _arg: usize, : *mut kuid_t) -> c_int;
}
extern "C" {
    pub fn nfs_map_group_to_gid(: *const nfs_server, : *const c_char, _arg: usize, : *mut kgid_t) -> c_int;
}
extern "C" {
    pub fn nfs_map_uid_to_name(: *const nfs_server, _arg: kuid_t, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn nfs_map_gid_to_group(: *const nfs_server, _arg: kgid_t, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn nfs_map_string_to_numeric(name: *const c_char, namelen: usize, res: *mut __u32) -> c_int;
}
