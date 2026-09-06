//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lockd/bind.h
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
// linux/include/linux/lockd/bind.h
//
// This is the part of lockd visible to nfsd and the nfs client.
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//
// struct nlmsvc_binding - lockd -> nfsd callback table
// @owner:  module that provides this binding.
// @fopen:  open a file by NFS file handle on behalf of an NLM request.
// @fclose: close a file that was previously opened via @fopen.
// Implementations MUST be semantically equivalent to fput().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsvc_binding {
    pub owner: *mut module,
    pub flags): *mut *mut *mut file filp, int,
    pub filp): *mut *mut void (fclose)(struct file,
}

//
// Similar to nfs_client_initdata, but without the NFS-specific
// rpc_ops field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmclnt_initdata {
    pub hostname: *const c_char,
    pub address: *const sockaddr,
    pub addrlen: usize,
    pub protocol: c_ushort,
    pub nfs_version: u32,
    pub noresvport: c_int,
    pub net: *mut net,
    pub nlmclnt_ops: *const nlmclnt_operations,
    pub cred: *const cred,
}

//
// Functions exported by the lockd module
//
extern "C" {
    pub fn nlmclnt_done(host: *mut nlm_host);
}
extern "C" {
    pub fn nlmclnt_shutdown_rpc_clnt(host: *mut nlm_host);
}
//
// NLM client operations provide a means to modify RPC processing of NLM
// requests.  Callbacks receive a pointer to data passed into the call to
// nlmclnt_proc().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmclnt_operations {
// Called on successful allocation of nlm_rqst, use for allocation or
// reference counting.
    pub ): *mut *mut void (nlmclnt_alloc_call)(void,
// Called in rpc_task_prepare for unlock.  A return value of true
// indicates the callback has put the task to sleep on a waitqueue
// and NLM should not call rpc_call_start().
    pub ): *mut *mut *mut bool (nlmclnt_unlock_prepare)(struct rpc_task, void,
// Called when the nlm_rqst is freed, callbacks should clean up here
    pub ): *mut *mut void (nlmclnt_release_call)(void,
}

extern "C" {
    pub fn nlmclnt_proc(host: *mut nlm_host, cmd: c_int, fl: *mut file_lock, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn lockd_up(net: *mut net, cred: *const cred) -> c_int;
}
extern "C" {
    pub fn lockd_down(net: *mut net);
}
//
// Cluster failover support
//
extern "C" {
    pub fn nlmsvc_unlock_all_by_sb(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nlmsvc_unlock_all_by_ip(server_addr: *mut sockaddr) -> c_int;
}
