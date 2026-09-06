//! Automatically rewritten from C Header to Rust Module
//! Source: fs/lockd/lockd.h
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
// Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
//

//
// Enable lockd debugging.
// Requires CONFIG_SUNRPC_DEBUG.
//

pub const NLMDBG_SVC: c_uint = 0x0001;
pub const NLMDBG_CLIENT: c_uint = 0x0002;
pub const NLMDBG_CLNTLOCK: c_uint = 0x0004;
pub const NLMDBG_SVCLOCK: c_uint = 0x0008;
pub const NLMDBG_MONITOR: c_uint = 0x0010;
pub const NLMDBG_CLNTSUBS: c_uint = 0x0020;
pub const NLMDBG_SVCSUBS: c_uint = 0x0040;
pub const NLMDBG_HOSTCACHE: c_uint = 0x0080;
pub const NLMDBG_XDR: c_uint = 0x0100;
pub const NLMDBG_ALL: c_uint = 0x7fff;
//
// Version string
//

//
// Default timeout for RPC calls (seconds)
//
pub const LOCKD_DFLT_TIMEO: c_int = 10;
//
// Number of leading bytes of nfs_fh.data that file_hash()
// digests when bucketing nlm_files[]. Sized for historical
// NFSv2 handles; nfs_fh.data must be initialized at least
// this far before lookup, regardless of fh.size.
//
pub const LOCKD_FH_HASH_SIZE: c_int = 32;
// error codes new to NLMv4

//
// Internal-use status codes, not to be placed on the wire.
// Version handlers translate these to appropriate wire values.
//

//
// Lockd host handle (used both by the client and server personality).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_host {
    pub /: *mut *mut hlist_node h_hash; / doubly linked list,
    pub /: *mut *mut sockaddr_storage h_addr; / peer address,
    pub h_addrlen: usize,
    pub /: *mut *mut sockaddr_storage h_srcaddr; / our address (optional),
    pub h_srcaddrlen: usize,
    pub /: *mut *mut *mut rpc_clnt h_rpcclnt; / RPC client to talk to peer,
    pub /: *mut *mut *mut char h_name; / remote hostname,
    pub /: *mut *mut u32 h_version; / interface version,
    pub /: *mut *mut unsigned short h_proto; / transport proto,
    pub 1: h_inuse :,
    pub /: *mut *mut wait_queue_head_t h_gracewait; / wait while reclaiming,
    pub /: *mut *mut rw_semaphore h_rwsem; / Reboot recovery lock,
    pub /: *mut *mut u32 h_state; / pseudo-state counter,
    pub /: *mut *mut u32 h_nsmstate; / true remote NSM state,
    pub /: *mut *mut u32 h_pidcount; / Pseudopids,
    pub /: *mut *mut refcount_t h_count; / reference count,
    pub /: *mut *mut mutex h_mutex; / mutex for pmap binding,
    pub /: *mut *mut unsigned long h_nextrebind; / next portmap call,
    pub /: *mut *mut unsigned long h_expires; / eligible for GC,
    pub /: *mut *mut list_head h_lockowners; / Lockowners for the client,
    pub h_lock: spinlock_t,
    pub /: *mut *mut list_head h_granted; / Locks in GRANTED state,
    pub /: *mut *mut list_head h_reclaim; / Locks in RECLAIM state,
    pub /: *mut *mut *mut nsm_handle h_nsmhandle; / NSM status handle,
    pub /: *mut *mut *mut char h_addrbuf; / address eyecatcher,
    pub /: *mut *mut *mut net net; / host net,
    pub h_cred: *const cred,
    pub 1]: char nodename[UNX_MAXNODENAME +,
    pub /: *const *const *const nlmclnt_operations h_nlmclnt_ops; / Callback ops for NLM users,
}

//
// The largest string sm_addrbuf should hold is a full-size IPv6 address
// (no "::" anywhere) with a scope ID.  The buffer size is computed to
// hold eight groups of colon-separated four-hex-digit numbers, a
// percent sign, a scope id (at most 32 bits, in decimal), and NUL.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsm_handle {
    pub sm_link: list_head,
    pub sm_count: refcount_t,
    pub sm_mon_name: *mut c_char,
    pub sm_name: *mut c_char,
    pub sm_addr: sockaddr_storage,
    pub sm_addrlen: usize,
    pub /: *mut *mut sm_sticky : 1; / don't unmonitor,
    pub sm_priv: nsm_private,
    pub sm_addrbuf: [c_char; NSM_ADDRBUF],
}

//
// Rigorous type checking on sockaddr type conversions
//
// Map an fl_owner_t into a unique 32-bit "pid"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_lockowner {
    pub list: list_head,
    pub count: refcount_t,
    pub host: *mut nlm_host,
    pub owner: fl_owner_t,
    pub pid: u32,
}

//
// This is the representation of a blocked client lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_wait {
    pub /: *mut *mut list_head b_list; / linked list,
    pub /: *mut *mut wait_queue_head_t b_wait; / where to wait on,
    pub b_host: *mut nlm_host,
    pub /: *mut *mut *mut file_lock b_lock; / local file lock,
    pub /: *mut *mut __be32 b_status; / grant callback status,
}

//
// Memory chunk for NLM client RPC request.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_rqst {
    pub a_count: refcount_t,
    pub /: *mut *mut unsigned int a_flags; / initial RPC task flags,
    pub /: *mut *mut *mut nlm_host  a_host; / host handle,
    pub /: *mut *mut lockd_args a_args; / arguments,
    pub /: *mut *mut lockd_res a_res; / result,
    pub a_block: *mut *mut nlm_block,
    pub /: *mut *mut unsigned int a_retries; / Retry count,
    pub a_owner: [u8; NLMCLNT_OHSIZE],
    pub /: *mut *mut *mut void  a_callback_data; / sent to nlmclnt_operations callbacks,
}

//
// This struct describes a file held open by lockd on behalf of
// an NFS client.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_file {
    pub /: *mut *mut hlist_node f_list; / linked list,
    pub /: *mut *mut nfs_fh f_handle; / NFS file handle,
    pub pointers,: *mut *mut *mut file  f_file[2]; / VFS file,
    pub /: *mut *mut *mut lockd_share  f_shares; / DOS shares,
    pub /: *mut *mut list_head f_blocks; / blocked locks,
    pub /: *mut *mut unsigned int f_locks; / guesstimate # of locks,
    pub /: *mut *mut unsigned int f_count; / reference count,
    pub /: *mut *mut mutex f_mutex; / avoid concurrent access,
}

//
// This is a server block (i.e. a lock requested by some client which
// couldn't be granted because of a conflicting lock).
//

// timeout on non-blocking call:

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlm_block {
    pub /: *mut *mut kref b_count; / Reference count,
    pub /: *mut *mut list_head b_list; / linked list of all blocks,
    pub /: *mut *mut list_head b_flist; / linked list (per file),
    pub /: *mut *mut *mut nlm_rqst  b_call; / RPC args & callback info,
    pub /: *mut *mut *mut svc_serv  b_daemon; / NLM service,
    pub /: *mut *mut *mut nlm_host  b_host; / host handle for RPC clnt,
    pub /: *mut *mut unsigned long b_when; / next re-xmit,
    pub /: *mut *mut unsigned int b_id; / block id,
    pub /: *mut *mut unsigned char b_granted; / VFS granted lock,
    pub /: *mut *mut *mut nlm_file  b_file; / file in question,
    pub /: *mut *mut *mut cache_req  b_cache_req; / deferred request handling,
    pub b_deferred_req: *mut *mut cache_deferred_req,
    pub /: *mut *mut unsigned int b_flags; / block flags,

}

//
// Global variables
//

//
// Lockd client functions
//
extern "C" {
    pub fn nlm_alloc_call(host: *mut nlm_host) -> *mut nlm_rqst;
}
extern "C" {
    pub fn nlm_async_call(: *mut nlm_rqst, _arg: u32, : *const rpc_call_ops) -> c_int;
}
extern "C" {
    pub fn nlm_async_reply(: *mut nlm_rqst, _arg: u32, : *const rpc_call_ops) -> c_int;
}
extern "C" {
    pub fn nlmclnt_release_call(: *mut nlm_rqst);
}
extern "C" {
    pub fn nlmclnt_queue_block(block: *mut nlm_wait);
}
extern "C" {
    pub fn nlmclnt_dequeue_block(block: *mut nlm_wait) -> __be32;
}
extern "C" {
    pub fn nlmclnt_wait(block: *mut nlm_wait, req: *mut nlm_rqst, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn nlmclnt_recovery(: *mut nlm_host);
}
extern "C" {
    pub fn nlmclnt_next_cookie(: *mut lockd_cookie);
}

//
// Host cache
//
extern "C" {
    pub fn nlmclnt_release_host(: *mut nlm_host);
}
extern "C" {
    pub fn nlmsvc_release_host(: *mut nlm_host);
}
extern "C" {
    pub fn nlm_bind_host(: *mut nlm_host) -> *mut rpc_clnt;
}
extern "C" {
    pub fn nlm_rebind_host(: *mut nlm_host);
}
extern "C" {
    pub fn nlm_get_host(: *mut nlm_host) -> *mut nlm_host;
}
extern "C" {
    pub fn nlm_shutdown_hosts();
}
extern "C" {
    pub fn nlm_shutdown_hosts_net(net: *mut net);
}
//
// Host monitoring
//
extern "C" {
    pub fn nsm_monitor(host: *const nlm_host) -> c_int;
}
extern "C" {
    pub fn nsm_unmonitor(host: *const nlm_host);
}
extern "C" {
    pub fn nsm_release(nsm: *mut nsm_handle);
}
//
// This is used in garbage collection and resource reclaim
// A return value != 0 means destroy the lock/block/share
//
extern "C" {
    pub fn int(owner: *mut *mut nlm_host_match_fn_t)(void, ref: *mut nlm_host) -> typedef;
}
//
// Server-side lock handling
//
extern "C" {
    pub fn lock_to_openmode(: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nlmsvc_unlock(net: *mut net, : *mut nlm_file, : *mut lockd_lock) -> __be32;
}
extern "C" {
    pub fn nlmsvc_cancel_blocked(net: *mut net, : *mut nlm_file, : *mut lockd_lock) -> __be32;
}
extern "C" {
    pub fn nlmsvc_retry_blocked(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn nlmsvc_grant_reply(: *mut lockd_cookie, _arg: __be32);
}
extern "C" {
    pub fn nlmsvc_release_call(: *mut nlm_rqst);
}
extern "C" {
    pub fn nlmsvc_locks_init_private(: *mut file_lock, : *mut nlm_host, _arg: pid_t);
}
extern "C" {
    pub fn nlmsvc_dispatch(rqstp: *mut svc_rqst) -> c_int;
}
//
// File handling for the server personality
//
extern "C" {
    pub fn nlm_release_file(: *mut nlm_file);
}
extern "C" {
    pub fn nlmsvc_put_lockowner(: *mut nlm_lockowner);
}
extern "C" {
    pub fn nlmsvc_release_lockowner(: *mut lockd_lock);
}
extern "C" {
    pub fn nlmsvc_mark_resources(: *mut net);
}
extern "C" {
    pub fn nlmsvc_free_host_resources(: *mut nlm_host);
}
extern "C" {
    pub fn nlmsvc_invalidate_all();
}
extern "C" {
    pub fn file_inode(_arg: nlmsvc_file_file(file)) -> return;
}
extern "C" {
    pub fn exportfs_cannot_lock(_arg: nlmsvc_file_file(file)->f_path.dentry->d_sb->s_export_op) -> return;
}
extern "C" {
    pub fn ipv4_is_loopback(_arg: sin->sin_addr.s_addr) -> return;
}

extern "C" {
    pub fn ipv4_is_loopback(_arg: sin6->sin6_addr.s6_addr32[3]) -> return;
}

//
// Ensure incoming requests are from local privileged callers.
//
// Return TRUE if sender is local and is connecting via a privileged port;
// otherwise return FALSE.
//
extern "C" {
    pub fn __nlm_privileged_request4(_arg: sap) -> return;
}
extern "C" {
    pub fn __nlm_privileged_request6(_arg: sap) -> return;
}
//
// Compare two NLM locks.
// When the second lock is of type F_UNLCK, this acts like a wildcard.
//
// lockd_set_file_lock_range3 - set the byte range of a file_lock
// @fl: file_lock whose length fields are to be initialized
// @off: starting offset of the lock, in bytes
// @len: length of the byte range, in bytes, or zero
//
// NLMv3 uses a (start, length) representation for lock byte ranges,
// while the kernel's file_lock uses (start, end). Treat a length of
// zero or arithmetic overflow (end wrapping negative when the sum
// exceeds S32_MAX) as "lock to end of file."
//
// lockd_set_file_lock_range4 - set the byte range of a file_lock
// @fl: file_lock whose length fields are to be initialized
// @off: starting offset of the lock, in bytes
// @len: length of the byte range, in bytes, or zero
//
// The NLMv4 protocol represents lock byte ranges as (start, length),
// where length zero means "lock to end of file." The kernel's file_lock
// structure uses (start, end) representation. Convert from NLMv4 format
// to file_lock format, clamping the starting offset and treating
// arithmetic overflow as "lock to EOF."
//
