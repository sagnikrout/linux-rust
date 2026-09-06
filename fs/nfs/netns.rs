//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/netns.h
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
// NFS-private data for each "struct net".  Accessed with net_generic().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_dev_msg {
    pub status: i32,
    pub minor: uint32_t major,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_net {
    pub nfs_dns_resolve: *mut cache_detail,
    pub bl_device_pipe: *mut rpc_pipe,
    pub bl_mount_reply: bl_dev_msg,
    pub bl_wq: wait_queue_head_t,
    pub bl_mutex: mutex,
    pub nfs_client_list: list_head,
    pub nfs_volume_list: list_head,

    pub /: *mut *mut idr cb_ident_idr; / Protected by nfs_client_lock,
    pub nfs_callback_tcpport: c_ushort,
    pub nfs_callback_tcpport6: c_ushort,
    pub 1]: int cb_users[NFS4_MAX_MINOR_VERSION +,
    pub nfs4_data_server_cache: list_head,
    pub nfs4_data_server_lock: spinlock_t,

    pub nfs_client: *mut nfs_netns_client,
    pub nfs_client_lock: spinlock_t,
    pub boot_time: ktime_t,
    pub rpcstats: rpc_stat,

    pub proc_nfsfs: *mut proc_dir_entry,

}
