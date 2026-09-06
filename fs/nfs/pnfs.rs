//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/pnfs.h
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
// pNFS client data structures.
//
// Copyright (c) 2002
// The Regents of the University of Michigan
// All Rights Reserved
//
// Dean Hildebrand <dhildebz@umich.edu>
//
// Permission is granted to use, copy, create derivative works, and
// redistribute this software and such derivative works for any purpose,
// so long as the name of the University of Michigan is not used in
// any advertising or publicity pertaining to the use or distribution
// of this software without specific, written prior authorization. If
// the above copyright notice or any other identification of the
// University of Michigan is included in any copy of any portion of
// this software, then the disclaimer below must also be included.
//
// This software is provided as is, without representation or warranty
// of any kind either express or implied, including without limitation
// the implied warranties of merchantability, fitness for a particular
// purpose, or noninfringement.  The Regents of the University of
// Michigan shall not be liable for any damages, including special,
// indirect, incidental, or consequential damages, with respect to any
// claim arising out of or in connection with the use of the software,
// even if it has been or is hereafter advised of the possibility of
// such damages.
//

// Individual ip address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_pnfs_ds_addr {
    pub da_addr: sockaddr_storage,
    pub da_addrlen: usize,
    pub /: *mut *mut list_head da_node; / nfs4_pnfs_dev_hlist dev_dslist,
    pub /: *mut *mut *mut char da_remotestr; / human readable addr+port,
    pub da_netid: *const c_char,
    pub da_transport: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_pnfs_ds {
    pub /: *mut *mut list_head ds_node; / nfs4_pnfs_dev_hlist dev_dslist,
    pub /: *mut *mut *mut char ds_remotestr; / comma sep list of addrs,
    pub ds_addrs: list_head,
    pub ds_net: *const net,
    pub ds_clp: *mut nfs_client,
    pub ds_count: refcount_t,
    pub /: *mut *mut u32 ds_version; / cache key, with ds_addrs,
    pub ds_state: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_layout_segment {
    pub pls_list: list_head,
    pub pls_lc_list: list_head,
    pub pls_commits: list_head,
    pub pls_range: pnfs_layout_range,
    pub pls_refcount: refcount_t,
    pub pls_seq: u32,
    pub pls_flags: c_ulong,
    pub pls_layout: *mut pnfs_layout_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_try_status {
    PNFS_ATTEMPTED     = 0,
    PNFS_NOT_ATTEMPTED = 1,
    PNFS_TRY_AGAIN     = 2,
}

//
// Default data server connection timeout and retrans vaules.
// Set by module parameters dataserver_timeo and dataserver_retrans.
//

pub const NFS4_DEF_DS_RETRANS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum layoutdriver_policy_flags {
// Should the pNFS client commit and return the layout upon truncate to
// a smaller size
    PNFS_LAYOUTRET_ON_SETATTR	= 1 << 0,
    PNFS_LAYOUTRET_ON_ERROR		= 1 << 1,
    PNFS_READ_WHOLE_PAGE		= 1 << 2,
    PNFS_LAYOUTGET_ON_OPEN		= 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_layout_destroy_mode {
    PNFS_LAYOUT_INVALIDATE = 0,
    PNFS_LAYOUT_BULK_RETURN,
    PNFS_LAYOUT_FILE_BULK_RETURN,
}

// Per-layout driver specific registration structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_layoutdriver_type {
    pub pnfs_tblid: list_head,
    pub id: u32,
    pub name: *const c_char,
    pub owner: *mut module,
    pub flags: unsigned,
    pub max_layoutget_response: unsigned,
    pub ): *const *const *const int (set_layoutdriver) (struct nfs_server , struct nfs_fh,
    pub ): *mut *mut int (clear_layoutdriver) (struct nfs_server,
    pub gfp_flags): *mut *mut *mut *mut pnfs_layout_hdr  (alloc_layout_hdr) (inode inode, gfp_t,
    pub ): *mut *mut void (free_layout_hdr) (struct pnfs_layout_hdr,
    pub gfp_flags): *mut *mut *mut *mut *mut pnfs_layout_segment  (alloc_lseg) (pnfs_layout_hdr layoutid, nfs4_layoutget_res lgr, gfp_t,
    pub lseg): *mut *mut void (free_lseg) (struct pnfs_layout_segment,
    pub free_me): *mut list_head,
    pub range): *mut pnfs_layout_range,
// test for nfs page cache coalescing
    pub pg_read_ops: *const nfs_pageio_ops,
    pub pg_write_ops: *const nfs_pageio_ops,
    pub inode): *mut *mut *mut pnfs_ds_commit_info (get_ds_info) (inode,
    pub datasync): *mut *mut *mut int (sync)(struct inode inode, bool,
//
// Return PNFS_ATTEMPTED to indicate the layout code has attempted
// I/O, else return PNFS_NOT_ATTEMPTED to fall back to normal NFS
//
    pub ): *mut *mut pnfs_try_status (read_pagelist)(struct nfs_pgio_header,
    pub int): *mut *mut *mut pnfs_try_status (write_pagelist)(struct nfs_pgio_header ,,
    pub ): *mut *mut void (free_deviceid_node) (struct nfs4_deviceid_node,
    pub gfp_flags): gfp_t,
    pub ): *mut *mut int (prepare_layoutreturn) (struct nfs4_layoutreturn_args,
    pub data): *mut *mut void (cleanup_layoutcommit) (struct nfs4_layoutcommit_data,
    pub args): *mut *mut int (prepare_layoutcommit) (struct nfs4_layoutcommit_args,
    pub args): *mut *mut int (prepare_layoutstats) (struct nfs42_layoutstat_args,
    pub lseg): *mut *mut void (cancel_io)(struct pnfs_layout_segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_commit_ops {
    pub ): *mut pnfs_layout_segment,
    pub inode): *mut inode,
    pub cinfo): *mut nfs_commit_info,
    pub ds_commit_idx): u32,
    pub cinfo): *mut nfs_commit_info,
    pub max): c_int,
    pub cinfo): *mut nfs_commit_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_layout_hdr {
    pub plh_refcount: refcount_t,
    pub /: *mut *mut atomic_t plh_outstanding; / number of RPCs out,
    pub /: *mut *mut list_head plh_layouts; / other client layouts,
    pub plh_bulk_destroy: list_head,
    pub /: *mut *mut list_head plh_segs; / layout segments list,
    pub /: *mut *mut list_head plh_return_segs; / invalid layout segments,
    pub /: *mut *mut unsigned long plh_block_lgets; / block LAYOUTGET if >0,
    pub plh_retry_timestamp: c_ulong,
    pub plh_flags: c_ulong,
    pub plh_stateid: nfs4_stateid,
    pub /: *mut *mut u32 plh_barrier; / ignore lower seqids,
    pub plh_return_seq: u32,
    pub plh_return_iomode: pnfs_iomode,
    pub /: *mut *mut loff_t plh_lwb; / last write byte for layoutcommit,
    pub /: *const *const *const cred plh_lc_cred; / layoutcommit cred,
    pub plh_inode: *mut inode,
    pub plh_rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_device {
    pub dev_id: nfs4_deviceid,
    pub layout_type: c_uint,
    pub mincount: c_uint,
    pub /: *mut *mut unsigned int maxcount; / gdia_maxcount,
    pub pages: *mut page,
    pub pgbase: c_uint,
    pub /: *mut *mut unsigned int pglen; / reply buffer length,
    pub /: *mut *mut unsigned char nocache : 1;/ May not be cached,
}

pub const NFS4_PNFS_GETDEVLIST_MAXNUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_devicelist {
    pub eof: c_uint,
    pub num_devs: c_uint,
    pub dev_id: [nfs4_deviceid; NFS4_PNFS_GETDEVLIST_MAXNUM],
}

extern "C" {
    pub fn pnfs_register_layoutdriver(: *mut pnfs_layoutdriver_type) -> c_int;
}
extern "C" {
    pub fn pnfs_unregister_layoutdriver(: *mut pnfs_layoutdriver_type);
}
extern "C" {
    pub fn pnfs_put_layoutdriver(ld: *const pnfs_layoutdriver_type);
}
// nfs4proc.c

extern "C" {
    pub fn max_response_pages(server: *mut nfs_server) -> usize;
}
// pnfs.c
extern "C" {
    pub fn pnfs_get_layout_hdr(lo: *mut pnfs_layout_hdr);
}
extern "C" {
    pub fn pnfs_put_lseg(lseg: *mut pnfs_layout_segment);
}
extern "C" {
    pub fn set_pnfs_layoutdriver(: *mut nfs_server, : *const nfs_fh, : *mut nfs_fsinfo);
}
extern "C" {
    pub fn unset_pnfs_layoutdriver(: *mut nfs_server);
}
extern "C" {
    pub fn pnfs_generic_pg_check_layout(pgio: *mut nfs_pageio_descriptor, req: *mut nfs_page);
}
extern "C" {
    pub fn pnfs_generic_pg_init_read(: *mut nfs_pageio_descriptor, : *mut nfs_page);
}
extern "C" {
    pub fn pnfs_generic_pg_readpages(desc: *mut nfs_pageio_descriptor) -> c_int;
}
extern "C" {
    pub fn pnfs_generic_pg_cleanup(: *mut nfs_pageio_descriptor);
}
extern "C" {
    pub fn pnfs_generic_pg_writepages(desc: *mut nfs_pageio_descriptor) -> c_int;
}
extern "C" {
    pub fn pnfs_set_lo_fail(lseg: *mut pnfs_layout_segment);
}
extern "C" {
    pub fn pnfs_layoutget_free(lgp: *mut nfs4_layoutget);
}
extern "C" {
    pub fn pnfs_free_lseg_list(tmp_list: *mut list_head);
}
extern "C" {
    pub fn pnfs_destroy_layout(: *mut nfs_inode);
}
extern "C" {
    pub fn pnfs_destroy_layout_final(: *mut nfs_inode);
}
extern "C" {
    pub fn pnfs_destroy_all_layouts(: *mut nfs_client);
}
extern "C" {
    pub fn pnfs_put_layout_hdr(lo: *mut pnfs_layout_hdr);
}
extern "C" {
    pub fn pnfs_wait_on_layoutreturn(ino: *mut inode, task: *mut rpc_task) -> bool;
}
extern "C" {
    pub fn pnfs_set_layoutcommit(: *mut inode, : *mut pnfs_layout_segment, _arg: loff_t);
}
extern "C" {
    pub fn pnfs_cleanup_layoutcommit(data: *mut nfs4_layoutcommit_data);
}
extern "C" {
    pub fn pnfs_layoutcommit_inode(inode: *mut inode, sync: bool) -> c_int;
}
extern "C" {
    pub fn pnfs_generic_sync(inode: *mut inode, datasync: bool) -> c_int;
}
extern "C" {
    pub fn pnfs_nfs_generic_sync(inode: *mut inode, datasync: bool) -> c_int;
}
extern "C" {
    pub fn _pnfs_return_layout(: *mut inode) -> c_int;
}
extern "C" {
    pub fn pnfs_commit_and_return_layout(: *mut inode) -> c_int;
}
extern "C" {
    pub fn pnfs_ld_write_done(: *mut nfs_pgio_header);
}
extern "C" {
    pub fn pnfs_ld_read_done(: *mut nfs_pgio_header);
}
extern "C" {
    pub fn pnfs_read_resend_pnfs(: *mut nfs_pgio_header, mirror_idx: c_uint);
}
extern "C" {
    pub fn nfs4_deviceid_mark_client_invalid(clp: *mut nfs_client);
}
extern "C" {
    pub fn pnfs_read_done_resend_to_mds(: *mut nfs_pgio_header) -> c_int;
}
extern "C" {
    pub fn pnfs_write_done_resend_to_mds(: *mut nfs_pgio_header) -> c_int;
}
extern "C" {
    pub fn pnfs_layout_handle_reboot(clp: *mut nfs_client) -> c_int;
}
// nfs4_deviceid_flags
// pnfs_dev.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_deviceid_node {
    pub node: hlist_node,
    pub tmpnode: hlist_node,
    pub ld: *const pnfs_layoutdriver_type,
    pub nfs_client: *const nfs_client,
    pub flags: c_ulong,
    pub timestamp_unavailable: c_ulong,
    pub deviceid: nfs4_deviceid,
    pub rcu: rcu_head,
    pub ref: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn nfs4_delete_deviceid(: *const pnfs_layoutdriver_type, : *const nfs_client, : *const nfs4_deviceid);
}
extern "C" {
    pub fn nfs4_put_deviceid_node(: *mut nfs4_deviceid_node) -> bool;
}
extern "C" {
    pub fn nfs4_mark_deviceid_available(node: *mut nfs4_deviceid_node);
}
extern "C" {
    pub fn nfs4_mark_deviceid_unavailable(node: *mut nfs4_deviceid_node);
}
extern "C" {
    pub fn nfs4_test_deviceid_unavailable(node: *mut nfs4_deviceid_node) -> bool;
}
extern "C" {
    pub fn nfs4_deviceid_purge_client(: *const nfs_client);
}
// pnfs_nfs.c
extern "C" {
    pub fn pnfs_free_commit_array(p: *mut pnfs_commit_array);
}
extern "C" {
    pub fn pnfs_generic_ds_cinfo_destroy(fl_cinfo: *mut pnfs_ds_commit_info);
}
extern "C" {
    pub fn pnfs_generic_commit_release(calldata: *mut c_void);
}
extern "C" {
    pub fn pnfs_generic_prepare_to_resend_writes(data: *mut nfs_commit_data);
}
extern "C" {
    pub fn pnfs_generic_rw_release(data: *mut c_void);
}
extern "C" {
    pub fn pnfs_generic_scan_commit_lists(cinfo: *mut nfs_commit_info, max: c_int) -> c_int;
}
extern "C" {
    pub fn pnfs_generic_write_commit_done(task: *mut rpc_task, data: *mut c_void);
}
extern "C" {
    pub fn nfs4_pnfs_ds_put(ds: *mut nfs4_pnfs_ds);
}
extern "C" {
    pub fn nfs4_pnfs_v3_ds_connect_unload();
}
extern "C" {
    pub fn nfs4_lgopen_release(lgp: *mut nfs4_layoutget);
}
// Return true if a layout driver is being used for this mountpoint
// Should the pNFS client commit and return the layout upon a setattr
extern "C" {
    pub fn NFS_SERVER(_arg: inode)->pnfs_curr_ld->sync(inode, _arg: datasync) -> return;
}
extern "C" {
    pub fn _pnfs_return_layout(_arg: ino) -> return;
}
//
// Are 2 ranges intersecting?
// start1                             end1
// [----------------------------------)
// start2           end2
// [----------------)
//
extern "C" {
    pub fn pnfs_is_range_intersecting(_arg: l1->offset, _arg: end1, _arg: l2->offset, _arg: end2) -> return;
}

extern "C" {
    pub fn nfs4_print_deviceid(dev_id: *const nfs4_deviceid);
}

extern "C" {
    pub fn pnfs_report_layoutstat(inode: *mut inode, gfp_flags: gfp_t) -> c_int;
}

