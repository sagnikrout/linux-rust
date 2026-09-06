//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs_fs.h
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
// linux/include/linux/nfs_fs.h
//
// Copyright (C) 1992  Rick Sladkey
//
// OS-specific nfs filesystem definitions and declarations
//

//
// Enable dprintk() debugging support for nfs client.
//

//
// These are the default for number of transports to different server IPs
//
pub const NFS_MAX_TRANSPORTS: c_int = 16;
//
// Size of the NFS directory verifier
//
pub const NFS_DIR_VERIFIER_SIZE: c_int = 2;
//
// NFSv3/v4 Access mode cache entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_access_entry {
    pub rb_node: rb_node,
    pub lru: list_head,
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
    pub group_info: *mut group_info,
    pub timestamp: u64,
    pub mask: __u32,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lock_context {
    pub count: refcount_t,
    pub list: list_head,
    pub open_context: *mut nfs_open_context,
    pub lockowner: fl_owner_t,
    pub io_count: core::sync::atomic::AtomicI32,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_file_localio {
    pub ro_file: *mut nfsd_file __rcu,
    pub rw_file: *mut nfsd_file __rcu,
    pub list: list_head,
    pub /: *mut *mut *mut void __rcu nfs_uuid; / opaque pointer to 'nfs_uuid_t',
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_open_context {
    pub lock_context: nfs_lock_context,
    pub flock_owner: fl_owner_t,
    pub dentry: *mut dentry,
    pub cred: *const cred,
    pub /: *mut *mut *mut rpc_cred __rcu ll_cred; / low-level cred - use to check for expiry,
    pub state: *mut nfs4_state,
    pub mode: fmode_t,
    pub error: c_int,
    pub flags: c_ulong,

    pub mdsthreshold: *mut nfs4_threshold,
    pub list: list_head,
    pub rcu_head: rcu_head,
    pub nfl: nfs_file_localio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_open_dir_context {
    pub list: list_head,
    pub cache_hits: core::sync::atomic::AtomicI32,
    pub cache_misses: core::sync::atomic::AtomicI32,
    pub attr_gencount: c_ulong,
    pub verf: [__be32; NFS_DIR_VERIFIER_SIZE],
    pub dir_cookie: __u64,
    pub last_cookie: __u64,
    pub page_index: pgoff_t,
    pub dtsize: c_uint,
    pub force_clear: bool,
    pub eof: bool,
    pub rcu_head: rcu_head,
}

//
// NFSv4 delegation
//
// nfs fs inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_inode {
//
// NFS file handle
//
    pub fh: nfs_fh,
//
// Various flags
//
    pub /: *mut *mut unsigned long flags; / atomic bit ops,
    pub /: *mut *mut unsigned long cache_validity; / bit mask,
//
// NFS Attributes not included in struct inode
//
    pub btime: timespec64,
    pub 1: bool uncacheable_file_data :,
//
// read_cache_jiffies is when we started read-caching this inode.
// attrtimeo is for how long the cached information is assumed
// to be valid. A successful attribute revalidation doubles
// attrtimeo (up to acregmax/acdirmax), a failure resets it to
// acregmin/acdirmin.
//
// We need to revalidate the cached attrs for this inode if
//
// jiffies - read_cache_jiffies >= attrtimeo
//
// Please note the comparison is greater than or equal
// so that zero timeout values can be specified.
//
    pub read_cache_jiffies: c_ulong,
    pub attrtimeo: c_ulong,
    pub attrtimeo_timestamp: c_ulong,
    pub attr_gencount: c_ulong,
    pub access_cache: rb_root,
    pub access_cache_entry_lru: list_head,
    pub access_cache_inode_lru: list_head,
// Directory
// "Generation counter" for the attribute cache.
// This is bumped whenever we update the metadata
// on the server.
//
    pub cache_change_attribute: c_ulong,
//
// This is the cookie verifier used for NFSv3 readdir
// operations
//
    pub cookieverf: [__be32; NFS_DIR_VERIFIER_SIZE],
// Readers: in-flight sillydelete RPC calls
// Writers: rmdir
    pub rmdir_sem: rw_semaphore,
}

// Regular file
// Open contexts for shared mmap writes
// Keep track of out-of-order replies.
// The ooo array contains start/end pairs of
// numbers from the changeid sequence when
// the inode's iversion has been updated.
// It also contains end/start pair (i.e. reverse order)
// of sections of the changeid sequence that have
// been seen in replies from the server.
// Normally these should match and when both
// A:B and B:A are found in ooo, they are both removed.
// And if a reply with A:B causes an iversion update
// of A:B, then neither are added.
// When a reply has pre_change that doesn't match
// iversion, then the changeid pair and any consequent
// change in iversion ARE added.  Later replies
// might fill in the gaps, or possibly a gap is caused
// by a change from another client.
// When a file or directory is opened, if the ooo table
// is not empty, then we assume the gaps were due to
// another client and we invalidate the cached data.
//
// We can only track a limited number of concurrent gaps.
// Currently that limit is 16.
// We allocate the table on demand.  If there is insufficient
// memory, then we probably cannot cache the file anyway
// so there is no loss.
//

// NFSv4 state
// pNFS layout information

// how many bytes have been written/read and how many bytes queued up

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_copy_state {
    pub copies: list_head,
    pub src_copies: list_head,
    pub stateid: nfs4_stateid,
    pub completion: completion,
    pub count: u64,
    pub verf: nfs_writeverf,
    pub error: c_int,
    pub flags: c_int,
    pub parent_src_state: *mut nfs4_state,
    pub parent_dst_state: *mut nfs4_state,
}

//
// Access bit flags
//
pub const NFS_ACCESS_READ: c_uint = 0x0001;
pub const NFS_ACCESS_LOOKUP: c_uint = 0x0002;
pub const NFS_ACCESS_MODIFY: c_uint = 0x0004;
pub const NFS_ACCESS_EXTEND: c_uint = 0x0008;
pub const NFS_ACCESS_DELETE: c_uint = 0x0010;
pub const NFS_ACCESS_EXECUTE: c_uint = 0x0020;
pub const NFS_ACCESS_XAREAD: c_uint = 0x0040;
pub const NFS_ACCESS_XAWRITE: c_uint = 0x0080;
pub const NFS_ACCESS_XALIST: c_uint = 0x0100;
//
// Cache validity bit flags
//

//
// Bit offsets in flags field
//

extern "C" {
    pub fn container_of(_arg: inode, nfs_inode: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn NFS_SB(_arg: inode->i_sb) -> return;
}
extern "C" {
    pub fn test_bit(_arg: NFS_INO_STALE, _arg: &NFS_I(inode)->flags) -> return;
}
//
// nfs_save_change_attribute - Returns the inode attribute change cookie
// @dir - pointer to parent directory inode
// The "cache change attribute" is updated when we need to revalidate
// our dentry cache after a directory was seen to change on the server.
//
// linux/fs/nfs/inode.c
//
extern "C" {
    pub fn nfs_sync_mapping(mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn nfs_zap_mapping(inode: *mut inode, mapping: *mut address_space);
}
extern "C" {
    pub fn nfs_zap_caches(: *mut inode);
}
extern "C" {
    pub fn nfs_set_inode_stale(inode: *mut inode);
}
extern "C" {
    pub fn nfs_invalidate_atime(: *mut inode);
}
extern "C" {
    pub fn nfs_refresh_inode(: *mut inode, : *mut nfs_fattr) -> c_int;
}
extern "C" {
    pub fn nfs_post_op_update_inode(inode: *mut inode, fattr: *mut nfs_fattr) -> c_int;
}
extern "C" {
    pub fn nfs_post_op_update_inode_force_wcc(inode: *mut inode, fattr: *mut nfs_fattr) -> c_int;
}
extern "C" {
    pub fn nfs_post_op_update_inode_force_wcc_locked(inode: *mut inode, fattr: *mut nfs_fattr) -> c_int;
}
extern "C" {
    pub fn nfs_access_add_cache(: *mut inode, : *mut nfs_access_entry, : *const cred);
}
extern "C" {
    pub fn nfs_access_set_mask(: *mut nfs_access_entry, _arg: u32);
}
extern "C" {
    pub fn nfs_permission(: *mut mnt_idmap, : *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_open(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn nfs_attribute_cache_expired(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_revalidate_inode(inode: *mut inode, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn __nfs_revalidate_inode(: *mut nfs_server, : *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_clear_invalid_mapping(mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn nfs_mapping_need_revalidate_inode(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn nfs_revalidate_mapping(inode: *mut inode, mapping: *mut address_space) -> c_int;
}
extern "C" {
    pub fn nfs_revalidate_mapping_rcu(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn nfs_setattr_update_inode(inode: *mut inode, attr: *mut iattr, : *mut nfs_fattr);
}
extern "C" {
    pub fn nfs_setsecurity(inode: *mut inode, fattr: *mut nfs_fattr);
}
extern "C" {
    pub fn put_nfs_open_context(ctx: *mut nfs_open_context);
}
extern "C" {
    pub fn nfs_inode_attach_open_context(ctx: *mut nfs_open_context);
}
extern "C" {
    pub fn nfs_file_set_open_context(filp: *mut file, ctx: *mut nfs_open_context);
}
extern "C" {
    pub fn nfs_file_clear_open_context(flip: *mut file);
}
extern "C" {
    pub fn nfs_put_lock_context(l_ctx: *mut nfs_lock_context);
}
extern "C" {
    pub fn nfs_fattr_init(fattr: *mut nfs_fattr);
}
extern "C" {
    pub fn nfs_fattr_set_barrier(fattr: *mut nfs_fattr);
}
extern "C" {
    pub fn nfs_inc_attr_generation_counter() -> c_ulong;
}

extern "C" {
    pub fn _nfs_display_fhandle_hash(fh: *const nfs_fh) -> u32;
}
extern "C" {
    pub fn _nfs_display_fhandle_hash(_arg: fh) -> return;
}
extern "C" {
    pub fn _nfs_display_fhandle(fh: *const nfs_fh, caption: *const c_char);
}

//
// linux/fs/nfs/nfsroot.c
//
// linux/net/ipv4/ipconfig.c: trims ip addr off front of name, too.
//
// linux/fs/nfs/file.c
//

//
// linux/fs/nfs/direct.c
//
// linux/fs/nfs/dir.c
//
extern "C" {
    pub fn nfs_force_lookup_revalidate(dir: *mut inode);
}
extern "C" {
    pub fn nfs_set_verifier(dentry: *mut *mut dentry, verf: c_ulong);
}

extern "C" {
    pub fn nfs_clear_verifier_delegated(inode: *mut inode);
}

extern "C" {
    pub fn nfs_may_open(inode: *mut inode, cred: *const cred, openflags: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_access_zap_cache(inode: *mut inode);
}
//
// linux/fs/nfs/symlink.c
//
// linux/fs/nfs/sysctl.c
//

extern "C" {
    pub fn nfs_register_sysctl() -> c_int;
}
extern "C" {
    pub fn nfs_unregister_sysctl();
}

pub const nfs_register_sysctl(): c_int = 0;

//
// linux/fs/nfs/namespace.c
//
extern "C" {
    pub fn nfs_release_automount_timer();
}
//
// linux/fs/nfs/unlink.c
//
extern "C" {
    pub fn nfs_complete_unlink(dentry: *mut dentry, : *mut inode);
}
//
// linux/fs/nfs/write.c
//
extern "C" {
    pub fn nfs_writepages(: *mut address_space, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn nfs_flush_incompatible(file: *mut file, folio: *mut folio) -> c_int;
}
//
// Try to write back everything synchronously (but check the
// return value!)
//
extern "C" {
    pub fn nfs_sync_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_wb_all(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn nfs_wb_folio(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn nfs_wb_folio_reclaim(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn nfs_wb_folio_cancel(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn nfs_commit_inode(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_commit_free(data: *mut nfs_commit_data);
}
extern "C" {
    pub fn nfs_commit_begin(cinfo: *mut nfs_mds_commit_info);
}
extern "C" {
    pub fn nfs_commit_end(cinfo: *mut nfs_mds_commit_info) -> bool;
}
//
// linux/fs/nfs/read.c
//
extern "C" {
    pub fn nfs_read_folio(: *mut file, : *mut folio) -> c_int;
}
extern "C" {
    pub fn nfs_readahead(: *mut readahead_control);
}
//
// inline functions
//
extern "C" {
    pub fn min_t(_arg: u64, _arg: size, _arg: OFFSET_MAX) -> return;
}

// We need to block new opens while a file is being unlinked.
// If it is opened *before* we decide to unlink, we will silly-rename
// instead. If it is opened *after*, then we need to create or will fail.
// If we allow the two to race, we could end up with a file that is open
// but deleted on the server resulting in ESTALE.
// So use ->d_fsdata to record when the unlink is happening
// and block dentry revalidation while it is set.
//

