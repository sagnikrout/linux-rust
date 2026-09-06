//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/rpc_pipe_fs.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe_dir_head {
    pub pdh_entries: list_head,
    pub pdh_dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe_dir_object {
    pub pdo_head: list_head,
    pub pdo_ops: *const rpc_pipe_dir_object_ops,
    pub pdo_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe_dir_object_ops {
    pub pdo): *mut rpc_pipe_dir_object,
    pub pdo): *mut rpc_pipe_dir_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe_msg {
    pub list: list_head,
    pub data: *mut c_void,
    pub len: usize,
    pub copied: usize,
    pub errno: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe_ops {
    pub size_t): *mut *mut *mut *mut *mut ssize_t (upcall)(struct file , struct rpc_pipe_msg , char __user ,,
    pub size_t): *const *const *const *const ssize_t (downcall)(struct file , char __user ,,
    pub ): *mut *mut void (release_pipe)(struct inode,
    pub ): *mut *mut int (open_pipe)(struct inode,
    pub ): *mut *mut void (destroy_msg)(struct rpc_pipe_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_pipe {
    pub pipe: list_head,
    pub in_upcall: list_head,
    pub in_downcall: list_head,
    pub pipelen: c_int,
    pub nreaders: c_int,
    pub nwriters: c_int,
pub const RPC_PIPE_WAIT_FOR_OPEN: c_int = 1;
    pub flags: c_int,
    pub queue_timeout: delayed_work,
    pub ops: *const rpc_pipe_ops,
    pub lock: spinlock_t,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_inode {
    pub vfs_inode: inode,
    pub private: *mut c_void,
    pub pipe: *mut rpc_pipe,
    pub waitq: wait_queue_head_t,
}

extern "C" {
    pub fn container_of(_arg: inode, rpc_inode: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn rpc_pipefs_notifier_register(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn rpc_pipefs_notifier_unregister(: *mut notifier_block);
}
extern "C" {
    pub fn rpc_pipefs_init_net(net: *mut net) -> c_int;
}
extern "C" {
    pub fn rpc_pipefs_exit_net(net: *mut net);
}
extern "C" {
    pub fn rpc_put_sb_net(net: *const net);
}
extern "C" {
    pub fn rpc_queue_upcall(: *mut rpc_pipe, : *mut rpc_pipe_msg) -> c_int;
}
// returns true if the msg is in-flight, i.e., already eaten by the peer
extern "C" {
    pub fn rpc_create_client_dir(: *mut dentry, : *const c_char, : *mut rpc_clnt) -> c_int;
}
extern "C" {
    pub fn rpc_remove_client_dir(: *mut rpc_clnt) -> c_int;
}
extern "C" {
    pub fn rpc_init_pipe_dir_head(pdh: *mut rpc_pipe_dir_head);
}
extern "C" {
    pub fn rpc_remove_cache_dir(: *mut dentry);
}
extern "C" {
    pub fn rpc_destroy_pipe_data(pipe: *mut rpc_pipe);
}
extern "C" {
    pub fn rpc_unlink(: *mut rpc_pipe);
}
extern "C" {
    pub fn register_rpc_pipefs() -> c_int;
}
extern "C" {
    pub fn unregister_rpc_pipefs();
}
extern "C" {
    pub fn gssd_running(net: *mut net) -> bool;
}
