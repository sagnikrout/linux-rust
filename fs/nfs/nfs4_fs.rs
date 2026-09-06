//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/nfs4_fs.h
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
// linux/fs/nfs/nfs4_fs.h
//
// Copyright (C) 2005 Trond Myklebust
//
// NFSv4-specific filesystem definitions and declarations
//

pub const NFS4_MIN_MINOR_VERSION: c_int = 0;

pub const NFS4_MIN_MINOR_VERSION: c_int = 1;

pub const NFS4_MAX_MINOR_VERSION: c_int = 2;

pub const NFS4_MAX_MINOR_VERSION: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_client_state {
    NFS4CLNT_MANAGER_RUNNING  = 0,
    NFS4CLNT_CHECK_LEASE,
    NFS4CLNT_LEASE_EXPIRED,
    NFS4CLNT_RECLAIM_REBOOT,
    NFS4CLNT_RECLAIM_NOGRACE,
    NFS4CLNT_DELEGRETURN,
    NFS4CLNT_SESSION_RESET,
    NFS4CLNT_LEASE_CONFIRM,
    NFS4CLNT_SERVER_SCOPE_MISMATCH,
    NFS4CLNT_PURGE_STATE,
    NFS4CLNT_BIND_CONN_TO_SESSION,
    NFS4CLNT_MOVED,
    NFS4CLNT_LEASE_MOVED,
    NFS4CLNT_DELEGATION_EXPIRED,
    NFS4CLNT_RUN_MANAGER,
    NFS4CLNT_MANAGER_AVAILABLE,
    NFS4CLNT_RECALL_RUNNING,
    NFS4CLNT_RECALL_ANY_LAYOUT_READ,
    NFS4CLNT_RECALL_ANY_LAYOUT_RW,
    NFS4CLNT_DELEGRETURN_DELAYED,
}

pub const NFS4_RENEW_TIMEOUT: c_uint = 0x01;
pub const NFS4_RENEW_DELEGATION_CB: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_minor_version_ops {
    pub minor_version: u32,
    pub init_caps: unsigned,
    pub ): *mut *mut int (init_client)(struct nfs_client,
    pub ): *mut *mut void (shutdown_client)(struct nfs_client,
    pub ): *const nfs4_stateid,
    pub ): *mut nfs_fattr,
    pub ): *mut nfs4_lock_state,
    pub ): *const *const nfs4_stateid , struct cred,
    pub gfp_t): *mut *mut *mut (alloc_seqid)(struct nfs_seqid_counter ,,
    pub data): *mut *mut rpc_xprt xprt, void,
    pub call_sync_ops: *const rpc_call_ops,
    pub sequence_slot_ops: *const nfs4_sequence_slot_ops,
    pub reboot_recovery_ops: *const nfs4_state_recovery_ops,
    pub nograce_recovery_ops: *const nfs4_state_recovery_ops,
    pub state_renewal_ops: *const nfs4_state_maintenance_ops,
    pub mig_recovery_ops: *const nfs4_mig_recovery_ops,
}

pub const NFS_SEQID_CONFIRMED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_seqid_counter {
    pub create_time: ktime_t,
    pub owner_id: u64,
    pub flags: c_int,
    pub counter: u32,
    pub /: *mut *mut spinlock_t lock; / Protects the list,
    pub /: *mut *mut list_head list; / Defines sequence of RPC calls,
    pub /: *mut *mut rpc_wait_queue wait; / RPC call delay queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_seqid {
    pub sequence: *mut nfs_seqid_counter,
    pub list: list_head,
    pub task: *mut rpc_task,
}

//
// NFS4 state_owners and lock_owners are simply labels for ordered
// sequences of RPC calls. Their sole purpose is to provide once-only
// semantics by allowing the server to identify replayed requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_state_owner {
    pub so_server: *mut nfs_server,
    pub so_lru: list_head,
    pub so_expires: c_ulong,
    pub so_server_node: rb_node,
    pub /: *const *const *const cred so_cred; / Associated cred,
    pub so_lock: spinlock_t,
    pub so_count: core::sync::atomic::AtomicI32,
    pub so_flags: c_ulong,
    pub so_states: list_head,
    pub so_seqid: nfs_seqid_counter,
    pub so_delegreturn_mutex: mutex,
}

pub const NFS_LOCK_NEW: c_int = 0;
pub const NFS_LOCK_RECLAIM: c_int = 1;
pub const NFS_LOCK_EXPIRED: c_int = 2;
//
// struct nfs4_state maintains the client-side state for a given
// (state_owner,inode) tuple (OPEN) or state_owner (LOCK).
//
// OPEN:
// In order to know when to OPEN_DOWNGRADE or CLOSE the state on the server,
// we need to know how many files are open for reading or writing on a
// given inode. This information too is stored here.
//
// LOCK: one nfs4_state (LOCK) to hold the lock stateid nfs4_state(OPEN)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lock_state {
    pub /: *mut *mut list_head ls_locks; / Other lock stateids,
    pub /: *mut *mut *mut nfs4_state  ls_state; / Pointer to open state,
pub const NFS_LOCK_INITIALIZED: c_int = 0;
pub const NFS_LOCK_LOST: c_int = 1;
pub const NFS_LOCK_UNLOCKING: c_int = 2;
    pub ls_flags: c_ulong,
    pub ls_seqid: nfs_seqid_counter,
    pub ls_stateid: nfs4_stateid,
    pub ls_count: refcount_t,
    pub ls_owner: fl_owner_t,
}

// bits for nfs4_state->flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_state {
    pub /: *mut *mut list_head open_states; / List of states for the same state_owner,
    pub /: *mut *mut list_head inode_states; / List of states for the same inode,
    pub /: *mut *mut list_head lock_states; / List of subservient lock stateids,
    pub /: *mut *mut *mut nfs4_state_owner owner; / Pointer to the open owner,
    pub /: *mut *mut *mut inode inode; / Pointer to the inode,
    pub /: *mut *mut unsigned long flags; / Do we hold any locks?,
    pub /: *mut *mut spinlock_t state_lock; / Protects the lock_states list,
    pub /: *mut *mut seqlock_t seqlock; / Protects the stateid/open_stateid,
    pub /: *mut *mut nfs4_stateid stateid; / Current stateid: may be delegation,
    pub /: *mut *mut nfs4_stateid open_stateid; / OPEN stateid,
// The following 3 fields are protected by owner->so_lock
    pub /: *mut *mut unsigned int n_rdonly; / Number of read-only references,
    pub /: *mut *mut unsigned int n_wronly; / Number of write-only references,
    pub /: *mut *mut unsigned int n_rdwr; / Number of read/write references,
    pub /: *mut *mut fmode_t state; / State on the server (R,W, or RW),
    pub count: refcount_t,
    pub waitq: wait_queue_head_t,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_exception {
    pub state: *mut nfs4_state,
    pub inode: *mut inode,
    pub stateid: *mut nfs4_stateid,
    pub timeout: c_long,
    pub retrans: c_ushort,
    pub 1: unsigned char task_is_privileged :,
    pub 1: retry :,
    pub interruptible: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_state_recovery_ops {
    pub owner_flag_bit: c_int,
    pub state_flag_bit: c_int,
    pub ): *mut *mut *mut int (recover_open)(struct nfs4_state_owner , struct nfs4_state,
    pub ): *mut *mut *mut int (recover_lock)(struct nfs4_state , struct file_lock,
    pub ): *const *const *const int (establish_clid)(struct nfs_client , struct cred,
    pub ): *const *const *const int (reclaim_complete)(struct nfs_client , struct cred,
    pub ): *const cred,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_opendata {
    pub kref: kref,
    pub o_arg: nfs_openargs,
    pub o_res: nfs_openres,
    pub c_arg: nfs_open_confirmargs,
    pub c_res: nfs_open_confirmres,
    pub owner_name: nfs4_string,
    pub group_name: nfs4_string,
    pub a_label: *mut nfs4_label,
    pub f_attr: nfs_fattr,
    pub dir: *mut dentry,
    pub dentry: *mut dentry,
    pub owner: *mut nfs4_state_owner,
    pub state: *mut nfs4_state,
    pub attrs: iattr,
    pub lgp: *mut nfs4_layoutget,
    pub timestamp: c_ulong,
    pub rpc_done: bool,
    pub file_created: bool,
    pub is_recover: bool,
    pub cancelled: bool,
    pub rpc_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_add_xprt_data {
    pub clp: *mut nfs_client,
    pub cred: *const cred,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_sequence_slot_ops {
    pub ): *mut *mut *mut int (process)(struct rpc_task , struct nfs4_sequence_res,
    pub ): *mut *mut *mut int (done)(struct rpc_task , struct nfs4_sequence_res,
    pub ): *mut *mut void (free_slot)(struct nfs4_sequence_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_state_maintenance_ops {
    pub unsigned): *const *const *const *const int (sched_state_renewal)(struct nfs_client , struct cred ,,
    pub ): *const *const *const cred  (get_state_renewal_cred)(nfs_client,
    pub ): *const *const *const int (renew_lease)(struct nfs_client , struct cred,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_mig_recovery_ops {
    pub ): *const *const *const nfs4_fs_locations , page , cred,
    pub ): *const *const *const int (fsid_present)(struct inode , struct cred,
}

// dir.c
// fs_context.c
// nfs4client.c
// nfs4namespace.c
extern "C" {
    pub fn nfs4_submount(: *mut fs_context, : *mut nfs_server) -> c_int;
}
// nfs4proc.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_call_sync_data {
    pub seq_server: *const nfs_server,
    pub seq_args: *mut nfs4_sequence_args,
    pub seq_res: *mut nfs4_sequence_res,
}

extern "C" {
    pub fn nfs4_handle_exception(: *mut nfs_server, _arg: c_int, : *mut nfs4_exception) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_setclientid(: *mut nfs_client, _arg: u32, short: unsigned, : *const cred, : *mut nfs4_setclientid_res) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_setclientid_confirm(: *mut nfs_client, arg: *mut nfs4_setclientid_res, : *const cred) -> c_int;
}
extern "C" {
    pub fn renew_lease(server: *const nfs_server, timestamp: c_ulong);
}
extern "C" {
    pub fn nfs4_proc_bind_conn_to_session(: *mut nfs_client, cred: *const cred) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_exchange_id(clp: *mut nfs_client, cred: *const cred) -> c_int;
}
extern "C" {
    pub fn nfs4_destroy_clientid(clp: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs4_init_clientid(: *mut nfs_client, : *const cred) -> c_int;
}
extern "C" {
    pub fn nfs41_init_clientid(: *mut nfs_client, : *const cred) -> c_int;
}
extern "C" {
    pub fn nfs4_do_close(state: *mut nfs4_state, gfp_mask: gfp_t, wait: c_int) -> c_int;
}
extern "C" {
    pub fn nfs4_server_capabilities(server: *mut nfs_server, fhandle: *mut nfs_fh) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_fsid_present(: *mut inode, : *const cred) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_secinfo(: *mut inode, : *const qstr, : *mut nfs4_secinfo_flavors) -> c_int;
}
extern "C" {
    pub fn nfs4_open_reclaim(: *mut nfs4_state_owner, : *mut nfs4_state) -> c_int;
}
extern "C" {
    pub fn nfs4_open_expired(: *mut nfs4_state_owner, : *mut nfs4_state) -> c_int;
}
extern "C" {
    pub fn nfs4_lock_reclaim(state: *mut nfs4_state, request: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nfs4_lock_expired(state: *mut nfs4_state, request: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nfs_state_clear_delegation(state: *mut nfs4_state);
}
extern "C" {
    pub fn nfs_state_clear_open_state_flags(state: *mut nfs4_state);
}
extern "C" {
    pub fn do_renew_lease(clp: *mut nfs_client, timestamp: c_ulong);
}
extern "C" {
    pub fn nfs4_match_stateid(s1: *const nfs4_stateid, s2: *const nfs4_stateid) -> bool;
}
extern "C" {
    pub fn nfs41_sequence_done(: *mut rpc_task, : *mut nfs4_sequence_res) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_create_session(: *mut nfs_client, : *const cred) -> c_int;
}
extern "C" {
    pub fn nfs4_proc_destroy_session(: *mut nfs4_session, : *const cred) -> c_int;
}
// Using machine creds for cleanup operations
// is only relevent if the client credentials
// might expire. So don't bother for
// RPC_AUTH_UNIX.  If file was only exported to
// sec=sys, the PUTFH would fail anyway.
//
// clntp = clp->cl_rpcclient;
//
// Function responsible for determining if an rpc_message should use the
// machine cred under SP4_MACH_CRED and if so switching the credential and
// authflavor (using the nfs_client's rpc_clnt which will be krb5i/p).
// Should be called before rpc_call_sync/rpc_call_async.
//
// Special wrapper to nfs4_state_protect for write.
// If WRITE can use machine cred but COMMIT cannot, make sure all writes
// that use machine cred use NFS_FILE_SYNC.
//
extern "C" {
    pub fn nfs41_shutdown_client(: *mut nfs_client);
}
extern "C" {
    pub fn nfs41_init_client(: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs4_free_client(: *mut nfs_client);
}
// nfs4renewd.c
extern "C" {
    pub fn nfs4_schedule_state_renewal(: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_kill_renewd(: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_renew_state(: *mut work_struct);
}
extern "C" {
    pub fn nfs4_set_lease_period(clp: *mut nfs_client, period: u32);
}
// nfs4state.c
extern "C" {
    pub fn nfs4_schedule_session_recovery(: *mut nfs4_session, _arg: c_int);
}
extern "C" {
    pub fn nfs41_notify_server(: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_put_state_owner(: *mut nfs4_state_owner);
}
extern "C" {
    pub fn nfs4_purge_state_owners(: *mut nfs_server, : *mut list_head);
}
extern "C" {
    pub fn nfs4_free_state_owners(head: *mut list_head);
}
extern "C" {
    pub fn nfs4_get_open_state(: *mut inode, : *mut nfs4_state_owner) -> *mut nfs4_state;
}
extern "C" {
    pub fn nfs4_put_open_state(: *mut nfs4_state);
}
extern "C" {
    pub fn nfs4_close_state(: *mut nfs4_state, _arg: fmode_t);
}
extern "C" {
    pub fn nfs4_close_sync(: *mut nfs4_state, _arg: fmode_t);
}
extern "C" {
    pub fn nfs4_state_set_mode_locked(: *mut nfs4_state, _arg: fmode_t);
}
extern "C" {
    pub fn nfs4_state_mark_reclaim_nograce(: *mut nfs_client, : *mut nfs4_state) -> c_int;
}
extern "C" {
    pub fn nfs4_schedule_lease_recovery(: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_wait_clnt_recover(clp: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs4_client_recover_expired_lease(clp: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs4_schedule_state_manager(: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_schedule_path_down_recovery(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs4_schedule_stateid_recovery(: *const nfs_server, : *mut nfs4_state) -> c_int;
}
extern "C" {
    pub fn nfs4_schedule_migration_recovery(: *const nfs_server) -> c_int;
}
extern "C" {
    pub fn nfs4_schedule_lease_moved_recovery(: *mut nfs_client);
}
extern "C" {
    pub fn nfs41_handle_sequence_flag_errors(clp: *mut nfs_client, flags: u32, _arg: bool);
}
extern "C" {
    pub fn nfs4_put_lock_state(lsp: *mut nfs4_lock_state);
}
extern "C" {
    pub fn nfs4_set_lock_state(state: *mut nfs4_state, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn nfs_wait_on_sequence(seqid: *mut nfs_seqid, task: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn nfs_increment_open_seqid(status: c_int, seqid: *mut nfs_seqid);
}
extern "C" {
    pub fn nfs_increment_lock_seqid(status: c_int, seqid: *mut nfs_seqid);
}
extern "C" {
    pub fn nfs_release_seqid(seqid: *mut nfs_seqid);
}
extern "C" {
    pub fn nfs_free_seqid(seqid: *mut nfs_seqid);
}
extern "C" {
    pub fn nfs4_free_lock_state(server: *mut nfs_server, lsp: *mut nfs4_lock_state);
}
extern "C" {
    pub fn nfs4_proc_commit(dst: *mut file, offset: __u64, count: __u32, res: *mut nfs_commitres) -> c_int;
}
// nfs4super.c

extern "C" {
    pub fn nfs4_try_get_tree(: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn nfs4_get_referral_tree(: *mut fs_context) -> c_int;
}
// nfs4sysctl.c

extern "C" {
    pub fn nfs4_register_sysctl() -> c_int;
}
extern "C" {
    pub fn nfs4_unregister_sysctl();
}

// nfs4xdr.c

// callback_xdr.c
// nfs42xattr.c

extern "C" {
    pub fn nfs4_xattr_cache_init() -> int __init;
}
extern "C" {
    pub fn nfs4_xattr_cache_exit();
}
extern "C" {
    pub fn nfs4_xattr_cache_remove(inode: *mut inode, name: *const c_char);
}
extern "C" {
    pub fn nfs4_xattr_cache_zap(inode: *mut inode);
}

