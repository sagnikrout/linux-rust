//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs_fs_sb.h
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
// The nfs_client identifies our client state to the server.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_client {
    pub cl_count: refcount_t,
    pub cl_mds_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut int cl_cons_state; / current construction state (-ve: init error),

    pub /: *mut *mut unsigned long cl_res_state; / NFS resources state,

    pub /: *mut *mut unsigned long cl_flags; / behavior switches,

    pub /: *mut *mut sockaddr_storage cl_addr; / server identifier,
    pub cl_addrlen: usize,
    pub /: *mut *mut *mut char  cl_hostname; / hostname of server,
    pub /: *mut *mut *mut char  cl_acceptor; / GSSAPI acceptor name,
    pub /: *mut *mut list_head cl_share_link; / link in global client list,
    pub /: *mut *mut list_head cl_superblocks; / List of nfs_server structs,
    pub cl_rpcclient: *mut *mut rpc_clnt,
    pub /: *const *const *const nfs_rpc_ops rpc_ops; / NFS protocol vector,
    pub /: *mut *mut int cl_proto; / Network transport protocol,
    pub /: *mut *mut *mut nfs_subversion  cl_nfs_mod; / pointer to nfs version module,
    pub /: *mut *mut u32 cl_minorversion;/ NFSv4 minorversion,
    pub /: *mut *mut unsigned int cl_nconnect; / Number of connections,
    pub /: *mut *mut unsigned int cl_max_connect; / max number of xprts allowed,
    pub /: *const *const *const char  cl_principal; / used for machine cred,
    pub /: *mut *mut xprtsec_parms cl_xprtsec; / xprt security policy,

    pub /: *mut *mut list_head cl_ds_clients; / auth flavor data servers,
    pub /: *mut *mut u64 cl_clientid; / constant,
    pub /: *mut *mut nfs4_verifier cl_confirm; / Clientid verifier,
    pub cl_state: c_ulong,
    pub cl_lock: spinlock_t,
    pub cl_lease_time: c_ulong,
    pub cl_last_renewal: c_ulong,
    pub cl_renewd: delayed_work,
    pub cl_rpcwaitq: rpc_wait_queue,
// idmapper
    pub cl_idmap: *mut *mut idmap,
// Client owner identifier
    pub cl_owner_id: *const *const c_char,
    pub /: *mut *mut u32 cl_cb_ident; / v4.0 callback identifier,
    pub cl_mvops: *const nfs4_minor_version_ops,
    pub cl_mig_gen: c_ulong,
// NFSv4.0 transport blocking
    pub cl_slot_tbl: *mut nfs4_slot_table,
// The sequence id to use for the next CREATE_SESSION
    pub cl_seqid: u32,
// The flags used for obtaining the clientid during EXCHANGE_ID
    pub cl_exchange_flags: u32,
    pub /: *mut *mut *mut nfs4_session cl_session; / shared session,
    pub cl_preserve_clid: bool,
    pub cl_serverowner: *mut nfs41_server_owner,
    pub cl_serverscope: *mut nfs41_server_scope,
    pub cl_implid: *mut nfs41_impl_id,
// nfs 4.1+ state protection modes:
    pub cl_sp4_flags: c_ulong,

// must use machine cred

    pub cl_lock_waitq: wait_queue_head_t,

// Our own IP address, as a null-terminated string.
// This is used to generate the mv0 callback address.
//
    pub cl_ipaddr: [c_char; 48],
    pub cl_net: *mut net,
    pub cl_ns_tracker: netns_tracker,
    pub pending_cb_stateids: list_head,
    pub rcu: rcu_head,

    pub cl_nfssvc_boot: timespec64,
    pub cl_boot_lock: seqlock_t,
    pub cl_uuid: nfs_uuid_t,
    pub cl_local_probe_work: work_struct,

}

//
// NFS client parameters stored in the superblock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_server {
    pub /: *mut *mut *mut nfs_client  nfs_client; / shared client and NFS4 state,
    pub structs: *mut *mut list_head client_link; / List of other nfs_server,
// that share the same client
//
    pub /: *mut *mut list_head master_link; / link in master servers list,
    pub /: *mut *mut *mut rpc_clnt  client; / RPC client handle,
    pub /: *mut *mut *mut rpc_clnt  client_acl; / ACL RPC client handle,
    pub /: *mut *mut *mut nlm_host nlm_host; / NLM client handle,
    pub /: *mut *mut *mut nfs_iostats __percpu io_stats; / I/O statistics,
    pub /: *mut *mut wait_queue_head_t write_congestion_wait; / wait until write congestion eases,
    pub /: *mut *mut atomic_long_t writeback; / number of writeback pages,
    pub /: *mut *mut unsigned int write_congested;/ flag set when writeback gets too high,
    pub /: *mut *mut unsigned int flags; / various flags,
// The following are for internal use only. Also see uapi/linux/nfs_mount.h
pub const NFS_MOUNT_LOOKUP_CACHE_NONEG: c_uint = 0x10000;
pub const NFS_MOUNT_LOOKUP_CACHE_NONE: c_uint = 0x20000;
pub const NFS_MOUNT_NORESVPORT: c_uint = 0x40000;
pub const NFS_MOUNT_LEGACY_INTERFACE: c_uint = 0x80000;
pub const NFS_MOUNT_LOCAL_FLOCK: c_uint = 0x100000;
pub const NFS_MOUNT_LOCAL_FCNTL: c_uint = 0x200000;
pub const NFS_MOUNT_SOFTERR: c_uint = 0x400000;
pub const NFS_MOUNT_SOFTREVAL: c_uint = 0x800000;
pub const NFS_MOUNT_WRITE_EAGER: c_uint = 0x01000000;
pub const NFS_MOUNT_WRITE_WAIT: c_uint = 0x02000000;
pub const NFS_MOUNT_TRUNK_DISCOVERY: c_uint = 0x04000000;
pub const NFS_MOUNT_SHUTDOWN: c_uint = 0x08000000;
pub const NFS_MOUNT_NO_ALIGNWRITE: c_uint = 0x10000000;
pub const NFS_MOUNT_FORCE_RDIRPLUS: c_uint = 0x20000000;
pub const NFS_MOUNT_NETUNREACH_FATAL: c_uint = 0x40000000;
    pub /: *mut *mut unsigned int automount_inherit; / Properties inherited by automount,
pub const NFS_AUTOMOUNT_INHERIT_BSIZE: c_uint = 0x0001;
pub const NFS_AUTOMOUNT_INHERIT_RSIZE: c_uint = 0x0002;
pub const NFS_AUTOMOUNT_INHERIT_WSIZE: c_uint = 0x0004;
    pub /: *mut *mut unsigned int caps; / server capabilities,
    pub /: *mut *mut __u64 fattr_valid; / Valid attributes,
    pub /: *mut *mut unsigned int rsize; / read size,
    pub /: *mut *mut unsigned int rpages; / read size (in pages),
    pub /: *mut *mut unsigned int wsize; / write size,
    pub /: *mut *mut unsigned int wtmult; / server disk block size,
    pub /: *mut *mut unsigned int dtsize; / readdir size,
    pub /: *mut *mut unsigned short port; / "port=" setting,
    pub /: *mut *mut unsigned int bsize; / server block size,

    pub /: *mut *mut unsigned int gxasize; / getxattr size,
    pub /: *mut *mut unsigned int sxasize; / setxattr size,
    pub /: *mut *mut unsigned int lxasize; / listxattr size,

    pub /: *mut *mut unsigned int acregmin; / attr cache timeouts,
    pub acregmax: c_uint,
    pub acdirmin: c_uint,
    pub acdirmax: c_uint,
    pub namelen: c_uint,
    pub /: *mut *mut unsigned int options; / extra options enabled by mount,
    pub /: *mut *mut unsigned int clone_blksize; / granularity of a CLONE operation,
pub const NFS_OPTION_FSCACHE: c_uint = 0x00000001	/* - local caching enabled */;
pub const NFS_OPTION_MIGRATION: c_uint = 0x00000002	/* - NFSv4 migration enabled */;
    pub /: *mut *mut change_attr_type;/ Description of change attribute,
    pub fsid: nfs_fsid,
    pub /: *mut *mut int s_sysfs_id; / sysfs dentry index,
    pub /: *mut *mut __u64 maxfilesize; / maximum file size,
    pub /: *mut *mut unsigned long mount_time; / when this fs was mounted,
    pub /: *mut *mut *mut super_block super; / VFS super block,
    pub /: *mut *mut dev_t s_dev; / superblock dev numbers,
    pub /: *mut *mut nfs_auth_info auth_info; / parsed auth flavors,

    pub /: *mut *mut *mut fscache_volume fscache; / superblock cookie,
    pub /: *mut *mut *mut char fscache_uniq; / Uniquifier (or NULL),

// The following #defines numerically match the NFSv4 equivalents

    pub file: *mut *mut u32 fh_expire_type; / V4 bitmask representing,
    pub /: *mut *mut u32 pnfs_blksize; / layout_blksize attr,

    pub set: *mut *mut u32 attr_bitmask[3];/ V4 bitmask representing the,
    pub attr_bitmask_nl: [u32; 3],
// V4 bitmask representing the
    pub exclcreat_bitmask: [u32; 3],
// V4 bitmask representing the
    pub cache_consistency_bitmask: [u32; 3],
// V4 bitmask representing the subset
    pub ACEs: *mut *mut u32 acl_bitmask; / V4 bitmask representing the,
    pub /: *mut *mut *mut pnfs_layoutdriver_type pnfs_curr_ld; / Active layout driver,
    pub roc_rpcwaitq: rpc_wait_queue,
// the following fields are protected by nfs_client->cl_lock
    pub state_owners: rb_root,

    pub owner_ctr: core::sync::atomic::AtomicI64,
    pub state_owners_lru: list_head,
    pub layouts: list_head,
    pub delegations: list_head,
    pub delegations_lock: spinlock_t,
    pub delegations_return: list_head,
    pub delegations_lru: list_head,
    pub delegations_delayed: list_head,
    pub nr_active_delegations: atomic_long_t,
    pub delegation_hash_mask: c_uint,
    pub delegation_hash_table: *mut hlist_head,
    pub ss_copies: list_head,
    pub ss_src_copies: list_head,
    pub delegation_flags: c_ulong,

    pub delegation_gen: c_ulong,
    pub mig_gen: c_ulong,
    pub mig_status: c_ulong,

    pub ): *mut *mut void (destroy)(struct nfs_server,
    pub /: *mut *mut atomic_t active; / Keep trace of any activity to this server,
// mountd-related mount options
    pub mountd_address: sockaddr_storage,
    pub mountd_addrlen: usize,
    pub mountd_version: u32,
    pub mountd_port: c_ushort,
    pub mountd_protocol: c_ushort,
    pub uoc_rpcwaitq: rpc_wait_queue,
// XDR related information
    pub read_hdrsize: c_uint,
// User namespace info
    pub cred: *const cred,
    pub has_sec_mnt_opts: bool,
    pub kobj: kobject,
    pub rcu: rcu_head,
}

// Server capabilities

