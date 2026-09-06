//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs_xdr.h
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
// To change the maximum rsize and wsize supported by the NFS client, adjust
// NFS_MAX_FILE_IO_SIZE.  64KB is a typical maximum, but some servers can
// support a megabyte or more.  The default is left at 4096 bytes, which is
// reasonable for NFS over UDP.
//

pub const NFS_BITMASK_SZ: c_int = 3;
// aux_flags in nfs_fattr

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_string {
    pub len: c_uint,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fsid {
    pub major: u64,
    pub minor: u64,
}

//
// Helper for checking equality between 2 fsids.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_threshold {
    pub bm: __u32,
    pub l_type: __u32,
    pub rd_sz: __u64,
    pub wr_sz: __u64,
    pub rd_io_sz: __u64,
    pub wr_io_sz: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fattr {
    pub /: *mut *mut __u64 valid; / which fields are valid,
    pub mode: umode_t,
    pub nlink: __u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rdev: dev_t,
    pub size: __u64,
    pub blocksize: __u32,
    pub blocks: __u32,
    pub nfs2: },
    pub used: __u64,
    pub nfs3: },
    pub du: },
    pub fsid: nfs_fsid,
    pub fileid: __u64,
    pub mounted_on_fileid: __u64,
    pub atime: timespec64,
    pub mtime: timespec64,
    pub ctime: timespec64,
    pub btime: timespec64,
    pub /: *mut *mut __u32 aux_flags; / NFSv4 auxiliary flags bitfield,
    pub /: *mut *mut __u64 change_attr; / NFSv4 change attribute,
    pub /: *mut *mut __u64 pre_change_attr;/ pre-op NFSv4 change attribute,
    pub /: *mut *mut __u64 pre_size; / pre_op_attr.size,
    pub /: *mut *mut timespec64 pre_mtime; / pre_op_attr.mtime,
    pub /: *mut *mut timespec64 pre_ctime; / pre_op_attr.ctime,
    pub time_start: c_ulong,
    pub gencount: c_ulong,
    pub owner_name: *mut nfs4_string,
    pub group_name: *mut nfs4_string,
    pub /: *mut *mut *mut nfs4_threshold mdsthreshold; / pNFS threshold hints,
    pub label: *mut nfs4_label,
}

//
// Maximal number of supported layout drivers.
//
pub const NFS_MAX_LAYOUT_TYPES: c_int = 8;
//
// Info on the file system
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fsinfo {
    pub /: *mut *mut *mut nfs_fattr fattr; / Post-op attributes,
    pub /: *mut *mut __u32 rtmax; / max. read transfer size,
    pub /: *mut *mut __u32 rtpref; / pref. read transfer size,
    pub /: *mut *mut __u32 rtmult; / reads should be multiple of this,
    pub /: *mut *mut __u32 wtmax; / max. write transfer size,
    pub /: *mut *mut __u32 wtpref; / pref. write transfer size,
    pub /: *mut *mut __u32 wtmult; / writes should be multiple of this,
    pub /: *mut *mut __u32 dtpref; / pref. readdir transfer size,
    pub maxfilesize: __u64,
    pub /: *mut *mut timespec64 time_delta; / server time granularity,
    pub /: *mut *mut __u32 lease_time; / in seconds,
    pub /: *mut *mut __u32 nlayouttypes; / number of layouttypes,
    pub /: *mut *mut __u32 layouttype[NFS_MAX_LAYOUT_TYPES]; / supported pnfs layout driver,
    pub /: *mut *mut __u32 blksize; / preferred pnfs io block size,
    pub /: *mut *mut __u32 clone_blksize; / granularity of a CLONE operation,
    pub /: *mut *mut change_attr_type; / Info about change attr,
    pub /: *mut *mut __u32 xattr_support; / User xattrs supported,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fsstat {
    pub /: *mut *mut *mut nfs_fattr fattr; / Post-op attributes,
    pub /: *mut *mut __u64 tbytes; / total size in bytes,
    pub /: *mut *mut __u64 fbytes; / # of free bytes,
    pub /: *mut *mut __u64 abytes; / # of bytes available to user,
    pub /: *mut *mut __u64 tfiles; / # of files,
    pub /: *mut *mut __u64 ffiles; / # of free files,
    pub /: *mut *mut __u64 afiles; / # of files available to user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs2_fsstat {
    pub /: *mut *mut __u32 tsize; / Server transfer size,
    pub /: *mut *mut __u32 bsize; / Filesystem block size,
    pub /: *mut *mut __u32 blocks; / No. of "bsize" blocks on filesystem,
    pub /: *mut *mut __u32 bfree; / No. of free "bsize" blocks,
    pub /: *mut *mut __u32 bavail; / No. of available "bsize" blocks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pathconf {
    pub /: *mut *mut *mut nfs_fattr fattr; / Post-op attributes,
    pub /: *mut *mut __u32 max_link; / max # of hard links,
    pub /: *mut *mut __u32 max_namelen; / max name length,
    pub case_insensitive: bool,
    pub case_preserving: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_change_info {
    pub atomic: u32,
    pub before: u64,
    pub after: u64,
}

// nfs41 sessions channel attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_channel_attrs {
    pub max_rqst_sz: u32,
    pub max_resp_sz: u32,
    pub max_resp_sz_cached: u32,
    pub max_ops: u32,
    pub max_reqs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_sequence_args {
    pub sa_slot: *mut nfs4_slot,
    pub 1: sa_privileged :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_sequence_res {
    pub sr_slot_ops: *const nfs4_sequence_slot_ops,
    pub /: *mut *mut *mut nfs4_slot sr_slot; / slot used to send request,
    pub sr_timestamp: c_ulong,
    pub /: *mut *mut int sr_status; / sequence operation status,
    pub sr_status_flags: u32,
    pub sr_highest_slotid: u32,
    pub sr_target_highest_slotid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_get_lease_time_args {
    pub la_seq_args: nfs4_sequence_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_get_lease_time_res {
    pub lr_seq_res: nfs4_sequence_res,
    pub lr_fsinfo: *mut nfs_fsinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_xdr_opaque_ops {
    pub ): *const nfs4_xdr_opaque_data,
    pub ): *mut *mut void (free)(struct nfs4_xdr_opaque_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_xdr_opaque_data {
    pub ops: *const nfs4_xdr_opaque_ops,
    pub data: *mut c_void,
}

pub const PNFS_LAYOUT_MAXSIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutdriver_data {
    pub pages: *mut page,
    pub pglen: __u32,
    pub len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_layout_range {
    pub iomode: u32,
    pub offset: u64,
    pub length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutget_args {
    pub seq_args: nfs4_sequence_args,
    pub type: __u32,
    pub range: pnfs_layout_range,
    pub minlength: __u64,
    pub maxcount: __u32,
    pub inode: *mut inode,
    pub ctx: *mut nfs_open_context,
    pub stateid: nfs4_stateid,
    pub layout: nfs4_layoutdriver_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutget_res {
    pub seq_res: nfs4_sequence_res,
    pub status: c_int,
    pub return_on_close: __u32,
    pub range: pnfs_layout_range,
    pub type: __u32,
    pub stateid: nfs4_stateid,
    pub layoutp: *mut nfs4_layoutdriver_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutget {
    pub args: nfs4_layoutget_args,
    pub res: nfs4_layoutget_res,
    pub cred: *const cred,
    pub lo: *mut pnfs_layout_hdr,
    pub gfp_flags: gfp_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_getdeviceinfo_args {
    pub seq_args: nfs4_sequence_args,
    pub pdev: *mut pnfs_device,
    pub notify_types: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_getdeviceinfo_res {
    pub seq_res: nfs4_sequence_res,
    pub pdev: *mut pnfs_device,
    pub notification: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutcommit_args {
    pub seq_args: nfs4_sequence_args,
    pub stateid: nfs4_stateid,
    pub lastbytewritten: __u64,
    pub inode: *mut inode,
    pub bitmask: *const u32,
    pub layoutupdate_len: usize,
    pub layoutupdate_page: *mut page,
    pub layoutupdate_pages: *mut page,
    pub start_p: *mut __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutcommit_res {
    pub seq_res: nfs4_sequence_res,
    pub fattr: *mut nfs_fattr,
    pub server: *const nfs_server,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutcommit_data {
    pub task: rpc_task,
    pub fattr: nfs_fattr,
    pub lseg_list: list_head,
    pub cred: *const cred,
    pub inode: *mut inode,
    pub args: nfs4_layoutcommit_args,
    pub res: nfs4_layoutcommit_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutreturn_args {
    pub seq_args: nfs4_sequence_args,
    pub layout: *mut pnfs_layout_hdr,
    pub inode: *mut inode,
    pub range: pnfs_layout_range,
    pub stateid: nfs4_stateid,
    pub layout_type: __u32,
    pub ld_private: *mut nfs4_xdr_opaque_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutreturn_res {
    pub seq_res: nfs4_sequence_res,
    pub lrs_present: u32,
    pub stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layoutreturn {
    pub args: nfs4_layoutreturn_args,
    pub res: nfs4_layoutreturn_res,
    pub cred: *const cred,
    pub clp: *mut nfs_client,
    pub inode: *mut inode,
    pub rpc_status: c_int,
    pub ld_private: nfs4_xdr_opaque_data,
}

pub const PNFS_LAYOUTSTATS_MAXSIZE: c_int = 384;
// Per file per deviceid layoutstats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layoutstat_devinfo {
    pub dev_id: nfs4_deviceid,
    pub offset: __u64,
    pub length: __u64,
    pub read_count: __u64,
    pub read_bytes: __u64,
    pub write_count: __u64,
    pub write_bytes: __u64,
    pub layout_type: __u32,
    pub ld_private: nfs4_xdr_opaque_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layoutstat_args {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub inode: *mut inode,
    pub stateid: nfs4_stateid,
    pub num_dev: c_int,
    pub devinfo: *mut nfs42_layoutstat_devinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layoutstat_res {
    pub seq_res: nfs4_sequence_res,
    pub num_dev: c_int,
    pub rpc_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layoutstat_data {
    pub inode: *mut inode,
    pub args: nfs42_layoutstat_args,
    pub res: nfs42_layoutstat_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_device_error {
    pub dev_id: nfs4_deviceid,
    pub status: c_int,
    pub opnum: nfs_opnum4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layout_error {
    pub offset: __u64,
    pub length: __u64,
    pub stateid: nfs4_stateid,
    pub errors: [nfs42_device_error; 1],
}

pub const NFS42_LAYOUTERROR_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layouterror_args {
    pub seq_args: nfs4_sequence_args,
    pub inode: *mut inode,
    pub num_errors: c_uint,
    pub errors: [nfs42_layout_error; NFS42_LAYOUTERROR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layouterror_res {
    pub seq_res: nfs4_sequence_res,
    pub num_errors: c_uint,
    pub rpc_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_layouterror_data {
    pub args: nfs42_layouterror_args,
    pub res: nfs42_layouterror_res,
    pub inode: *mut inode,
    pub lseg: *mut pnfs_layout_segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_clone_args {
    pub seq_args: nfs4_sequence_args,
    pub src_fh: *mut nfs_fh,
    pub dst_fh: *mut nfs_fh,
    pub src_stateid: nfs4_stateid,
    pub dst_stateid: nfs4_stateid,
    pub src_offset: __u64,
    pub dst_offset: __u64,
    pub count: __u64,
    pub dst_bitmask: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_clone_res {
    pub seq_res: nfs4_sequence_res,
    pub rpc_status: c_uint,
    pub dst_fattr: *mut nfs_fattr,
    pub server: *const nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stateowner_id {
    pub create_time: __u64,
    pub uniquifier: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_open_delegation {
    pub open_delegation_type: __u32,
    pub type: fmode_t,
    pub do_recall: __u32,
    pub stateid: nfs4_stateid,
    pub pagemod_limit: c_ulong,
}

//
// Arguments to the open call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_openargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub seqid: *mut *mut nfs_seqid,
    pub open_flags: c_int,
    pub fmode: fmode_t,
    pub share_access: u32,
    pub access: u32,
    pub clientid: __u64,
    pub id: stateowner_id,
    pub /: *mut *mut *mut iattr  attrs; / UNCHECKED, GUARDED, EXCLUSIVE4_1,
    pub /: *mut *mut nfs4_verifier verifier; / EXCLUSIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_openres {
    pub seq_res: nfs4_sequence_res,
    pub stateid: nfs4_stateid,
    pub fh: nfs_fh,
    pub cinfo: nfs4_change_info,
    pub rflags: __u32,
    pub f_attr: *mut *mut nfs_fattr,
    pub seqid: *mut *mut nfs_seqid,
    pub server: *const nfs_server,
    pub attrset: [__u32; NFS4_BITMAP_SIZE],
    pub owner: *mut nfs4_string,
    pub group_owner: *mut nfs4_string,
    pub delegation: nfs4_open_delegation,
    pub access_request: __u32,
    pub access_supported: __u32,
    pub access_result: __u32,
    pub lg_res: *mut nfs4_layoutget_res,
}

//
// Arguments to the open_confirm call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_open_confirmargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub stateid: *mut *mut nfs4_stateid,
    pub seqid: *mut *mut nfs_seqid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_open_confirmres {
    pub seq_res: nfs4_sequence_res,
    pub stateid: nfs4_stateid,
    pub seqid: *mut *mut nfs_seqid,
}

//
// Arguments to the close call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_closeargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub stateid: nfs4_stateid,
    pub seqid: *mut *mut nfs_seqid,
    pub fmode: fmode_t,
    pub share_access: u32,
    pub bitmask: *const *const u32,
    pub bitmask_store: [u32; NFS_BITMASK_SZ],
    pub lr_args: *mut nfs4_layoutreturn_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_closeres {
    pub seq_res: nfs4_sequence_res,
    pub stateid: nfs4_stateid,
    pub fattr: *mut *mut nfs_fattr,
    pub seqid: *mut *mut nfs_seqid,
    pub server: *const nfs_server,
    pub lr_res: *mut nfs4_layoutreturn_res,
    pub lr_ret: c_int,
}

//
// * Arguments to the lock,lockt, and locku call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lowner {
    pub clientid: __u64,
    pub id: __u64,
    pub s_dev: dev_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lock_args {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub fl: *mut *mut file_lock,
    pub lock_seqid: *mut *mut nfs_seqid,
    pub lock_stateid: nfs4_stateid,
    pub open_seqid: *mut *mut nfs_seqid,
    pub open_stateid: nfs4_stateid,
    pub lock_owner: nfs_lowner,
    pub 1: unsigned char block :,
    pub 1: unsigned char reclaim :,
    pub 1: unsigned char new_lock_owner :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lock_res {
    pub seq_res: nfs4_sequence_res,
    pub stateid: nfs4_stateid,
    pub lock_seqid: *mut *mut nfs_seqid,
    pub open_seqid: *mut *mut nfs_seqid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_locku_args {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub fl: *mut *mut file_lock,
    pub seqid: *mut *mut nfs_seqid,
    pub stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_locku_res {
    pub seq_res: nfs4_sequence_res,
    pub stateid: nfs4_stateid,
    pub seqid: *mut *mut nfs_seqid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lockt_args {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub fl: *mut *mut file_lock,
    pub lock_owner: nfs_lowner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_lockt_res {
    pub seq_res: nfs4_sequence_res,
    pub /: *mut *mut *mut file_lock  denied; / LOCK, LOCKT failed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_release_lockowner_args {
    pub seq_args: nfs4_sequence_args,
    pub lock_owner: nfs_lowner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_release_lockowner_res {
    pub seq_res: nfs4_sequence_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_delegattr {
    pub atime: timespec64,
    pub mtime: timespec64,
    pub atime_set: bool,
    pub mtime_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_delegreturnargs {
    pub seq_args: nfs4_sequence_args,
    pub fhandle: *const nfs_fh,
    pub stateid: *const nfs4_stateid,
    pub bitmask: *const u32,
    pub bitmask_store: [u32; NFS_BITMASK_SZ],
    pub lr_args: *mut nfs4_layoutreturn_args,
    pub sattr_args: *mut nfs4_delegattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_delegreturnres {
    pub seq_res: nfs4_sequence_res,
    pub fattr: *mut *mut nfs_fattr,
    pub server: *mut nfs_server,
    pub lr_res: *mut nfs4_layoutreturn_res,
    pub lr_ret: c_int,
    pub sattr_res: bool,
    pub sattr_ret: c_int,
}

//
// Arguments to the write call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_write_verifier {
    pub data: [c_char; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_writeverf {
    pub verifier: nfs_write_verifier,
    pub committed: nfs3_stable_how,
}

//
// Arguments shared by the read and write call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pgio_args {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub context: *mut nfs_open_context,
    pub lock_context: *mut nfs_lock_context,
    pub stateid: nfs4_stateid,
    pub offset: __u64,
    pub count: __u32,
    pub pgbase: c_uint,
    pub pages: *mut *mut *mut page,
    pub /: *mut *mut unsigned int replen; / used by read,
    pub /: *const *const *const u32  bitmask; / used by write,
    pub /: *mut *mut u32 bitmask_store[NFS_BITMASK_SZ]; / used by write,
    pub /: *mut *mut nfs3_stable_how stable; / used by write,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pgio_res {
    pub seq_res: nfs4_sequence_res,
    pub fattr: *mut *mut nfs_fattr,
    pub count: __u64,
    pub op_status: __u32,
    pub /: *mut *mut unsigned int replen; / used by read,
    pub /: *mut *mut int eof; / used by read,
    pub /: *mut *mut *mut void  scratch; / used by read,
}

//
// Arguments to the commit call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_commitargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub offset: __u64,
    pub count: __u32,
    pub bitmask: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_commitres {
    pub seq_res: nfs4_sequence_res,
    pub op_status: __u32,
    pub fattr: *mut nfs_fattr,
    pub verf: *mut nfs_writeverf,
    pub server: *const nfs_server,
}

//
// Common arguments to the unlink call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_removeargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const nfs_fh,
    pub name: qstr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_removeres {
    pub seq_res: nfs4_sequence_res,
    pub server: *mut nfs_server,
    pub dir_attr: *mut nfs_fattr,
    pub cinfo: nfs4_change_info,
}

//
// Common arguments to the rename call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_renameargs {
    pub seq_args: nfs4_sequence_args,
    pub old_dir: *const nfs_fh,
    pub new_dir: *const nfs_fh,
    pub old_name: *const qstr,
    pub new_name: *const qstr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_renameres {
    pub seq_res: nfs4_sequence_res,
    pub server: *mut nfs_server,
    pub old_cinfo: nfs4_change_info,
    pub old_fattr: *mut nfs_fattr,
    pub new_cinfo: nfs4_change_info,
    pub new_fattr: *mut nfs_fattr,
}

// parsed sec= options

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_auth_info {
    pub flavor_len: c_uint,
    pub flavors: [rpc_authflavor_t; NFS_AUTH_INFO_MAX_FLAVORS],
}

//
// Argument struct for decode_entry function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_entry {
    pub ino: __u64,
    pub cookie: __u64,
    pub name: *const *const c_char,
    pub len: c_uint,
    pub eof: c_int,
    pub fh: *mut *mut nfs_fh,
    pub fattr: *mut *mut nfs_fattr,
    pub d_type: c_uchar,
    pub server: *mut *mut nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_readdir_arg {
    pub dentry: *mut dentry,
    pub cred: *const cred,
    pub verf: *mut __be32,
    pub cookie: u64,
    pub pages: *mut page,
    pub page_len: c_uint,
    pub plus: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_readdir_res {
    pub verf: *mut __be32,
}

//
// The following types are for NFSv2 only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_sattrargs {
    pub fh: *mut *mut nfs_fh,
    pub sattr: *mut *mut iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_diropargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_createargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
    pub sattr: *mut *mut iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_setattrargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub stateid: nfs4_stateid,
    pub iap: *mut *mut iattr,
    pub /: *const *const *const nfs_server  server; / Needed for name mapping,
    pub bitmask: *const *const u32,
    pub label: *const nfs4_label,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_acl_type {
    NFS4ACL_NONE = 0,
    NFS4ACL_ACL,
    NFS4ACL_DACL,
    NFS4ACL_SACL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_setaclargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub acl_type: nfs4_acl_type,
    pub acl_len: usize,
    pub acl_pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_setaclres {
    pub seq_res: nfs4_sequence_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_getaclargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut *mut nfs_fh,
    pub acl_type: nfs4_acl_type,
    pub acl_len: usize,
    pub acl_pages: *mut *mut *mut page,
}

// getxattr ACL interface flags
pub const NFS4_ACL_TRUNC: c_uint = 0x0001	/* ACL was truncated */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_getaclres {
    pub seq_res: nfs4_sequence_res,
    pub acl_type: nfs4_acl_type,
    pub acl_len: usize,
    pub acl_data_offset: usize,
    pub acl_flags: c_int,
    pub acl_scratch: *mut *mut folio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_setattrres {
    pub seq_res: nfs4_sequence_res,
    pub fattr: *mut *mut nfs_fattr,
    pub server: *const *const nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_linkargs {
    pub fromfh: *mut *mut nfs_fh,
    pub tofh: *mut *mut nfs_fh,
    pub toname: *const *const c_char,
    pub tolen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_symlinkargs {
    pub fromfh: *mut *mut nfs_fh,
    pub fromname: *const *const c_char,
    pub fromlen: c_uint,
    pub pages: *mut *mut *mut page,
    pub pathlen: c_uint,
    pub sattr: *mut *mut iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_readdirargs {
    pub fh: *mut *mut nfs_fh,
    pub cookie: __u32,
    pub count: c_uint,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_getaclargs {
    pub fh: *mut *mut nfs_fh,
    pub mask: c_int,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_setaclargs {
    pub inode: *mut *mut inode,
    pub mask: c_int,
    pub acl_access: *mut *mut posix_acl,
    pub acl_default: *mut *mut posix_acl,
    pub len: usize,
    pub npages: c_uint,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_diropok {
    pub fh: *mut *mut nfs_fh,
    pub fattr: *mut *mut nfs_fattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_readlinkargs {
    pub fh: *mut *mut nfs_fh,
    pub pgbase: c_uint,
    pub pglen: c_uint,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_sattrargs {
    pub fh: *mut *mut nfs_fh,
    pub sattr: *mut *mut iattr,
    pub guard: c_uint,
    pub guardtime: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_diropargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_accessargs {
    pub fh: *mut *mut nfs_fh,
    pub access: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_createargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
    pub sattr: *mut *mut iattr,
    pub createmode: nfs3_createmode,
    pub verifier: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_mkdirargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
    pub sattr: *mut *mut iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_symlinkargs {
    pub fromfh: *mut *mut nfs_fh,
    pub fromname: *const *const c_char,
    pub fromlen: c_uint,
    pub pages: *mut *mut *mut page,
    pub pathlen: c_uint,
    pub sattr: *mut *mut iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_mknodargs {
    pub fh: *mut *mut nfs_fh,
    pub name: *const *const c_char,
    pub len: c_uint,
    pub type: nfs3_ftype,
    pub sattr: *mut *mut iattr,
    pub rdev: dev_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_linkargs {
    pub fromfh: *mut *mut nfs_fh,
    pub tofh: *mut *mut nfs_fh,
    pub toname: *const *const c_char,
    pub tolen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_readdirargs {
    pub fh: *mut *mut nfs_fh,
    pub cookie: __u64,
    pub verf: [__be32; 2],
    pub plus: bool,
    pub count: c_uint,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_diropres {
    pub dir_attr: *mut *mut nfs_fattr,
    pub fh: *mut *mut nfs_fh,
    pub fattr: *mut *mut nfs_fattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_accessres {
    pub fattr: *mut *mut nfs_fattr,
    pub access: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_readlinkargs {
    pub fh: *mut *mut nfs_fh,
    pub pgbase: c_uint,
    pub pglen: c_uint,
    pub pages: *mut *mut *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_linkres {
    pub dir_attr: *mut *mut nfs_fattr,
    pub fattr: *mut *mut nfs_fattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_readdirres {
    pub dir_attr: *mut *mut nfs_fattr,
    pub verf: *mut *mut __be32,
    pub plus: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs3_getaclres {
    pub fattr: *mut *mut nfs_fattr,
    pub mask: c_int,
    pub acl_access_count: c_uint,
    pub acl_default_count: c_uint,
    pub acl_access: *mut *mut posix_acl,
    pub acl_default: *mut *mut posix_acl,
}

pub type clientid4 = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_accessargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
    pub access: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_accessres {
    pub seq_res: nfs4_sequence_res,
    pub server: *const *const nfs_server,
    pub fattr: *mut *mut nfs_fattr,
    pub supported: u32,
    pub access: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_create_arg {
    pub seq_args: nfs4_sequence_args,
    pub ftype: u32,
    pub pages: *mut *mut *mut page,
    pub len: c_uint,
    pub /: *mut *mut } symlink; / NF4LNK,
    pub specdata1: u32,
    pub specdata2: u32,
    pub /: *mut *mut } device; / NF4BLK, NF4CHR,
    pub u: },
    pub name: *const *const qstr,
    pub server: *const *const nfs_server,
    pub attrs: *const *const iattr,
    pub dir_fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
    pub label: *const nfs4_label,
    pub umask: umode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_create_res {
    pub seq_res: nfs4_sequence_res,
    pub server: *const *const nfs_server,
    pub fh: *mut *mut nfs_fh,
    pub fattr: *mut *mut nfs_fattr,
    pub dir_cinfo: nfs4_change_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fsinfo_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fsinfo_res {
    pub seq_res: nfs4_sequence_res,
    pub fsinfo: *mut nfs_fsinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_getattr_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
    pub get_dir_deleg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_gdd_res {
    pub status: u32,
    pub deleg: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_getattr_res {
    pub seq_res: nfs4_sequence_res,
    pub server: *const *const nfs_server,
    pub fattr: *mut *mut nfs_fattr,
    pub gdd_res: *mut *mut nfs4_gdd_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_link_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub dir_fh: *const *const nfs_fh,
    pub name: *const *const qstr,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_link_res {
    pub seq_res: nfs4_sequence_res,
    pub server: *const *const nfs_server,
    pub fattr: *mut *mut nfs_fattr,
    pub cinfo: nfs4_change_info,
    pub dir_attr: *mut *mut nfs_fattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lookup_arg {
    pub seq_args: nfs4_sequence_args,
    pub dir_fh: *const *const nfs_fh,
    pub name: *const *const qstr,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lookup_res {
    pub seq_res: nfs4_sequence_res,
    pub server: *const *const nfs_server,
    pub fattr: *mut *mut nfs_fattr,
    pub fh: *mut *mut nfs_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lookupp_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const nfs_fh,
    pub bitmask: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lookupp_res {
    pub seq_res: nfs4_sequence_res,
    pub server: *const nfs_server,
    pub fattr: *mut nfs_fattr,
    pub fh: *mut nfs_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lookup_root_arg {
    pub seq_args: nfs4_sequence_args,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_pathconf_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_pathconf_res {
    pub seq_res: nfs4_sequence_res,
    pub pathconf: *mut nfs_pathconf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_readdir_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub cookie: u64,
    pub verifier: nfs4_verifier,
    pub count: u32,
    pub /: *mut *mut *mut *mut page  pages; / zero-copy data,
    pub /: *mut *mut unsigned int pgbase; / zero-copy data,
    pub bitmask: *const *const u32,
    pub plus: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_readdir_res {
    pub seq_res: nfs4_sequence_res,
    pub verifier: nfs4_verifier,
    pub pgbase: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_readlink {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub pgbase: c_uint,
    pub /: *mut *mut unsigned int pglen; / zero-copy data,
    pub /: *mut *mut *mut *mut page  pages; / zero-copy data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_readlink_res {
    pub seq_res: nfs4_sequence_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_setclientid {
    pub sc_verifier: *const *const nfs4_verifier,
    pub sc_prog: u32,
    pub sc_netid_len: c_uint,
    pub 1]: char sc_netid[RPCBIND_MAXNETIDLEN +,
    pub sc_uaddr_len: c_uint,
    pub 1]: char sc_uaddr[RPCBIND_MAXUADDRLEN +,
    pub sc_clnt: *mut nfs_client,
    pub sc_cred: *mut rpc_cred,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_setclientid_res {
    pub clientid: u64,
    pub confirm: nfs4_verifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_statfs_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const *const nfs_fh,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_statfs_res {
    pub seq_res: nfs4_sequence_res,
    pub fsstat: *mut nfs_fsstat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_open_caps {
    pub oa_share_access: [u32; 1],
    pub oa_share_deny: [u32; 1],
    pub oa_share_access_want: [u32; 1],
    pub oa_open_claim: [u32; 1],
    pub oa_createmode: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_server_caps_arg {
    pub seq_args: nfs4_sequence_args,
    pub fhandle: *mut nfs_fh,
    pub bitmask: *const *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_server_caps_res {
    pub seq_res: nfs4_sequence_res,
    pub attr_bitmask: [u32; 3],
    pub exclcreat_bitmask: [u32; 3],
    pub acl_bitmask: u32,
    pub has_links: u32,
    pub has_symlinks: u32,
    pub fh_expire_type: u32,
    pub case_insensitive: u32,
    pub case_preserving: u32,
    pub open_caps: nfs4_open_caps,
}

pub const NFS4_PATHNAME_MAXCOMPONENTS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_pathname {
    pub ncomponents: c_uint,
    pub components: [nfs4_string; NFS4_PATHNAME_MAXCOMPONENTS],
}

pub const NFS4_FS_LOCATION_MAXSERVERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fs_location {
    pub nservers: c_uint,
    pub servers: [nfs4_string; NFS4_FS_LOCATION_MAXSERVERS],
    pub rootpath: nfs4_pathname,
}

pub const NFS4_FS_LOCATIONS_MAXENTRIES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fs_locations {
    pub fattr: *mut nfs_fattr,
    pub server: *const nfs_server,
    pub fs_path: nfs4_pathname,
    pub nlocations: c_int,
    pub locations: [nfs4_fs_location; NFS4_FS_LOCATIONS_MAXENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fs_locations_arg {
    pub seq_args: nfs4_sequence_args,
    pub dir_fh: *const nfs_fh,
    pub fh: *const nfs_fh,
    pub name: *const qstr,
    pub page: *mut page,
    pub bitmask: *const u32,
    pub clientid: clientid4,
    pub renew:1: unsigned char migration:1,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fs_locations_res {
    pub seq_res: nfs4_sequence_res,
    pub fs_locations: *mut nfs4_fs_locations,
    pub renew:1: unsigned char migration:1,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_secinfo4 {
    pub flavor: u32,
    pub flavor_info: rpcsec_gss_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_secinfo_flavors {
    pub num_flavors: c_uint,
    pub flavors: [nfs4_secinfo4; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_secinfo_arg {
    pub seq_args: nfs4_sequence_args,
    pub dir_fh: *const nfs_fh,
    pub name: *const qstr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_secinfo_res {
    pub seq_res: nfs4_sequence_res,
    pub flavors: *mut nfs4_secinfo_flavors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fsid_present_arg {
    pub seq_args: nfs4_sequence_args,
    pub fh: *const nfs_fh,
    pub clientid: clientid4,
    pub renew:1: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_fsid_present_res {
    pub seq_res: nfs4_sequence_res,
    pub fh: *mut nfs_fh,
    pub renew:1: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_commit_bucket {
    pub written: list_head,
    pub committing: list_head,
    pub lseg: *mut pnfs_layout_segment,
    pub direct_verf: nfs_writeverf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_commit_array {
    pub cinfo_list: list_head,
    pub lseg_list: list_head,
    pub lseg: *mut pnfs_layout_segment,
    pub rcu: rcu_head,
    pub refcount: refcount_t,
    pub nbuckets: c_uint,
    pub __counted_by(nbuckets): pnfs_commit_bucket buckets[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_ds_commit_info {
    pub commits: list_head,
    pub nwritten: c_uint,
    pub ncommitting: c_uint,
    pub ops: *const pnfs_commit_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_state_protection {
    pub how: u32,
    pub enforce: nfs4_op_map,
    pub allow: nfs4_op_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_exchange_id_args {
    pub client: *mut nfs_client,
    pub verifier: nfs4_verifier,
    pub flags: u32,
    pub state_protect: nfs41_state_protection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_server_owner {
    pub minor_id: u64,
    pub major_id_sz: u32,
    pub major_id: [c_char; NFS4_OPAQUE_LIMIT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_server_scope {
    pub server_scope_sz: u32,
    pub server_scope: [c_char; NFS4_OPAQUE_LIMIT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_impl_id {
    pub 1]: char domain[NFS4_OPAQUE_LIMIT +,
    pub 1]: char name[NFS4_OPAQUE_LIMIT +,
    pub date: nfstime4,
}

pub const MAX_BIND_CONN_TO_SESSION_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_bind_conn_to_session_args {
    pub client: *mut nfs_client,
    pub sessionid: nfs4_sessionid,
    pub dir: u32,
    pub use_conn_in_rdma_mode: bool,
    pub retries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_bind_conn_to_session_res {
    pub sessionid: nfs4_sessionid,
    pub dir: u32,
    pub use_conn_in_rdma_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_exchange_id_res {
    pub clientid: u64,
    pub seqid: u32,
    pub flags: u32,
    pub server_owner: *mut nfs41_server_owner,
    pub server_scope: *mut nfs41_server_scope,
    pub impl_id: *mut nfs41_impl_id,
    pub state_protect: nfs41_state_protection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_create_session_args {
    pub client: *mut nfs_client,
    pub clientid: u64,
    pub seqid: u32,
    pub flags: u32,
    pub cb_program: u32,
    pub /: *mut *mut nfs4_channel_attrs fc_attrs; / Fore Channel,
    pub /: *mut *mut nfs4_channel_attrs bc_attrs; / Back Channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_create_session_res {
    pub sessionid: nfs4_sessionid,
    pub seqid: u32,
    pub flags: u32,
    pub /: *mut *mut nfs4_channel_attrs fc_attrs; / Fore Channel,
    pub /: *mut *mut nfs4_channel_attrs bc_attrs; / Back Channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_reclaim_complete_args {
    pub seq_args: nfs4_sequence_args,
// In the future extend to include curr_fh for use with migration
    pub one_fs:1: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_reclaim_complete_res {
    pub seq_res: nfs4_sequence_res,
}

pub const SECINFO_STYLE_CURRENT_FH: c_int = 0;
pub const SECINFO_STYLE_PARENT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_secinfo_no_name_args {
    pub seq_args: nfs4_sequence_args,
    pub style: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_test_stateid_args {
    pub seq_args: nfs4_sequence_args,
    pub stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_test_stateid_res {
    pub seq_res: nfs4_sequence_res,
    pub status: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_free_stateid_args {
    pub seq_args: nfs4_sequence_args,
    pub stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs41_free_stateid_res {
    pub seq_res: nfs4_sequence_res,
    pub status: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_ds_commit_info {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_falloc_args {
    pub seq_args: nfs4_sequence_args,
    pub falloc_fh: *mut nfs_fh,
    pub falloc_stateid: nfs4_stateid,
    pub falloc_offset: u64,
    pub falloc_length: u64,
    pub falloc_bitmask: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_falloc_res {
    pub seq_res: nfs4_sequence_res,
    pub status: c_uint,
    pub falloc_fattr: *mut nfs_fattr,
    pub falloc_server: *const nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_copy_args {
    pub seq_args: nfs4_sequence_args,
    pub src_fh: *mut nfs_fh,
    pub src_stateid: nfs4_stateid,
    pub src_pos: u64,
    pub dst_fh: *mut nfs_fh,
    pub dst_stateid: nfs4_stateid,
    pub dst_pos: u64,
    pub count: u64,
    pub sync: bool,
    pub cp_src: *mut nl4_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_write_res {
    pub stateid: nfs4_stateid,
    pub count: u64,
    pub verifier: nfs_writeverf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_copy_res {
    pub seq_res: nfs4_sequence_res,
    pub write_res: nfs42_write_res,
    pub consecutive: bool,
    pub synchronous: bool,
    pub commit_res: nfs_commitres,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_offload_status_args {
    pub osa_seq_args: nfs4_sequence_args,
    pub osa_src_fh: *mut nfs_fh,
    pub osa_stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_offload_status_res {
    pub osr_seq_res: nfs4_sequence_res,
    pub osr_count: u64,
    pub complete_count: c_int,
    pub osr_complete: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_copy_notify_args {
    pub cna_seq_args: nfs4_sequence_args,
    pub cna_src_fh: *mut nfs_fh,
    pub cna_src_stateid: nfs4_stateid,
    pub cna_dst: nl4_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_copy_notify_res {
    pub cnr_seq_res: nfs4_sequence_res,
    pub cnr_lease_time: nfstime4,
    pub cnr_stateid: nfs4_stateid,
    pub cnr_src: nl4_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_seek_args {
    pub seq_args: nfs4_sequence_args,
    pub sa_fh: *mut nfs_fh,
    pub sa_stateid: nfs4_stateid,
    pub sa_offset: u64,
    pub sa_what: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_seek_res {
    pub seq_res: nfs4_sequence_res,
    pub status: c_uint,
    pub sr_eof: u32,
    pub sr_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_setxattrargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub bitmask: *const u32,
    pub xattr_name: *const c_char,
    pub xattr_flags: u32,
    pub xattr_len: usize,
    pub xattr_pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_setxattrres {
    pub seq_res: nfs4_sequence_res,
    pub cinfo: nfs4_change_info,
    pub fattr: *mut nfs_fattr,
    pub server: *const nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_getxattrargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub xattr_name: *const c_char,
    pub xattr_len: usize,
    pub xattr_pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_getxattrres {
    pub seq_res: nfs4_sequence_res,
    pub xattr_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_listxattrsargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub count: u32,
    pub cookie: u64,
    pub xattr_pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_listxattrsres {
    pub seq_res: nfs4_sequence_res,
    pub scratch: *mut folio,
    pub xattr_buf: *mut c_void,
    pub xattr_len: usize,
    pub cookie: u64,
    pub eof: bool,
    pub copied: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_removexattrargs {
    pub seq_args: nfs4_sequence_args,
    pub fh: *mut nfs_fh,
    pub bitmask: *const u32,
    pub xattr_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_removexattrres {
    pub seq_res: nfs4_sequence_res,
    pub cinfo: nfs4_change_info,
    pub fattr: *mut nfs_fattr,
    pub server: *const nfs_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_page_array {
    pub pagevec: *mut page,
    pub /: *mut *mut unsigned int npages; / Max length of pagevec,
    pub page_array: [*mut page; NFS_PAGEVEC_SIZE],
}

// used as flag bits in nfs_pgio_header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pgio_header {
    pub inode: *mut inode,
    pub cred: *const cred,
    pub pages: list_head,
    pub req: *mut nfs_page,
    pub /: *mut *mut nfs_writeverf verf; / Used for writes,
    pub rw_mode: fmode_t,
    pub lseg: *mut pnfs_layout_segment,
    pub io_start: loff_t,
    pub mds_ops: *const rpc_call_ops,
    pub hdr): *mut *mut void (release) (struct nfs_pgio_header,
    pub completion_ops: *const nfs_pgio_completion_ops,
    pub rw_ops: *const nfs_rw_ops,
    pub io_completion: *mut nfs_io_completion,
    pub dreq: *mut nfs_direct_req,

    pub netfs: *mut c_void,

    pub retrans: c_ushort,
    pub pnfs_error: c_int,
    pub /: *mut *mut int error; / merge with pnfs_error,
    pub /: *mut *mut unsigned int good_bytes; / boundary of good data,
    pub flags: c_ulong,
//
// rpc data
//
    pub task: rpc_task,
    pub fattr: nfs_fattr,
    pub /: *mut *mut nfs_pgio_args args; / argument struct,
    pub /: *mut *mut nfs_pgio_res res; / result struct,
    pub /: *mut *mut unsigned long timestamp; / For lease renewal,
    pub ): *mut *mut *mut int (pgio_done_cb)(struct rpc_task , struct nfs_pgio_header,
    pub /: *mut *mut __u64 mds_offset; / Filelayout dense stripe,
    pub page_array: nfs_page_array,
    pub /: *mut *mut *mut nfs_client ds_clp; / pNFS data server,
    pub /: *mut *mut u32 ds_commit_idx; / ds index if ds_clp is set,
    pub /: *mut *mut u32 pgio_mirror_idx;/ mirror index in pgio layer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_mds_commit_info {
    pub rpcs_out: core::sync::atomic::AtomicI32,
    pub ncommit: atomic_long_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_commit_completion_ops {
    pub data): *mut *mut void (completion) (struct nfs_commit_data,
    pub ): *mut *mut *mut void (resched_write) (struct nfs_commit_info , struct nfs_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_commit_info {
    pub /: *mut *mut *mut inode inode; / Needed for inode->i_lock,
    pub mds: *mut nfs_mds_commit_info,
    pub ds: *mut pnfs_ds_commit_info,
    pub /: *mut *mut *mut nfs_direct_req dreq; / O_DIRECT request,
    pub completion_ops: *const nfs_commit_completion_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_commit_data {
    pub task: rpc_task,
    pub inode: *mut inode,
    pub cred: *const cred,
    pub fattr: nfs_fattr,
    pub verf: nfs_writeverf,
    pub /: *mut *mut list_head pages; / Coalesced requests we wish to flush,
    pub /: *mut *mut list_head list; / lists of nfs_write_data,
    pub /: *mut *mut *mut nfs_direct_req dreq; / O_DIRECT request,
    pub /: *mut *mut nfs_commitargs args; / argument struct,
    pub /: *mut *mut nfs_commitres res; / result struct,
    pub context: *mut nfs_open_context,
    pub lseg: *mut pnfs_layout_segment,
    pub /: *mut *mut *mut nfs_client ds_clp; / pNFS data server,
    pub ds_commit_index: c_int,
    pub lwb: loff_t,
    pub mds_ops: *const rpc_call_ops,
    pub completion_ops: *const nfs_commit_completion_ops,
    pub data): *mut *mut *mut int (commit_done_cb) (struct rpc_task task, struct nfs_commit_data,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pgio_completion_ops {
    pub int): *mut *mut *mut void (error_cleanup)(struct list_head head,,
    pub hdr): *mut *mut void (init_hdr)(struct nfs_pgio_header,
    pub hdr): *mut *mut void (completion)(struct nfs_pgio_header,
    pub hdr): *mut *mut void (reschedule_io)(struct nfs_pgio_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_unlinkdata {
    pub args: nfs_removeargs,
    pub res: nfs_removeres,
    pub dentry: *mut dentry,
    pub cred: *const cred,
    pub dir_attr: nfs_fattr,
    pub timeout: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_renamedata {
    pub args: nfs_renameargs,
    pub res: nfs_renameres,
    pub task: rpc_task,
    pub cred: *const cred,
    pub old_dir: *mut inode,
    pub old_dentry: *mut dentry,
    pub old_fattr: nfs_fattr,
    pub new_dir: *mut inode,
    pub new_dentry: *mut dentry,
    pub new_fattr: nfs_fattr,
    pub ): *mut *mut *mut void (complete)(struct rpc_task , struct nfs_renamedata,
    pub timeout: c_long,
    pub cancelled: bool,
}

//
// RPC procedure vector for NFSv2/NFSv3 demuxing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_rpc_ops {
    pub /: *mut *mut u32 version; / Protocol version,
    pub dentry_ops: *const dentry_operations,
    pub dir_inode_ops: *const inode_operations,
    pub file_inode_ops: *const inode_operations,
    pub file_ops: *const file_operations,
    pub nlmclnt_ops: *const nlmclnt_operations,
    pub ): *mut nfs_fsinfo,
    pub ): *mut *mut *mut int (submount) (struct fs_context , struct nfs_server,
    pub ): *mut *mut int (try_get_tree) (struct fs_context,
    pub ): *mut *mut nfs_fattr , inode,
    pub ): *mut iattr,
    pub ): *mut *mut nfs_fh , nfs_fattr,
    pub ): *mut nfs_fattr,
    pub ): *const *const *const *const int (access) (struct inode , struct nfs_access_entry , struct cred,
    pub int): unsigned,
    pub int): *mut *mut iattr ,,
    pub ): *mut *mut *mut int (remove) (struct inode , struct dentry,
    pub ): *mut *mut *mut *mut void (unlink_setup) (struct rpc_message , struct dentry , struct inode,
    pub ): *mut *mut *mut void (unlink_rpc_prepare) (struct rpc_task , struct nfs_unlinkdata,
    pub ): *mut *mut *mut int (unlink_done) (struct rpc_task , struct inode,
    pub same_parent): *mut inode,
    pub ): *mut *mut *mut void (rename_rpc_prepare)(struct rpc_task task, struct nfs_renamedata,
    pub new_dir): *mut *mut *mut *mut int (rename_done) (struct rpc_task task, struct inode old_dir, struct inode,
    pub ): *const *const *const *const int (link) (struct inode , struct inode , struct qstr,
    pub ): *mut unsigned int, struct iattr,
    pub ): *mut *mut *mut *mut *mut dentry (mkdir) (inode , dentry , iattr,
    pub ): *const *const *const int (rmdir) (struct inode , struct qstr,
    pub ): *mut *mut *mut int (readdir) (struct nfs_readdir_arg , struct nfs_readdir_res,
    pub ): *mut nfs_fsstat,
    pub ): *mut nfs_fsinfo,
    pub ): *mut nfs_pathconf,
    pub ): *mut *mut *mut int (set_capabilities)(struct nfs_server , struct nfs_fh,
    pub bool): *mut *mut *mut *mut int (decode_dirent)(struct xdr_stream , struct nfs_entry ,,
    pub ): *mut nfs_pgio_header,
    pub ): *mut *mut *mut void (read_setup)(struct nfs_pgio_header , struct rpc_message,
    pub ): *mut *mut *mut int (read_done)(struct rpc_task , struct nfs_pgio_header,
    pub ): *mut rpc_clnt,
    pub ): *mut *mut *mut int (write_done)(struct rpc_task , struct nfs_pgio_header,
    pub ): *mut rpc_clnt,
    pub ): *mut *mut *mut void (commit_rpc_prepare)(struct rpc_task , struct nfs_commit_data,
    pub ): *mut *mut *mut int (commit_done) (struct rpc_task , struct nfs_commit_data,
    pub ): *mut *mut *mut int (lock)(struct file , int, struct file_lock,
    pub ): *const *const int (lock_check_bounds)(struct file_lock,
    pub ): *mut *mut void (clear_acl_cache)(struct inode,
    pub int): *mut *mut *mut void (close_context)(struct nfs_open_context ctx,,
    pub ): *mut c_int,
    pub int): *mut *mut *mut int (have_delegation)(struct inode , fmode_t,,
    pub ): *mut *mut void (return_delegation)(struct inode,
    pub ): *const *const *const nfs_client (alloc_client) (nfs_client_initdata,
    pub ): *const nfs_client_initdata,
    pub ): *mut *mut void (free_client) (struct nfs_client,
    pub ): *mut *mut *mut nfs_server (create_server)(fs_context,
    pub rpc_authflavor_t): *mut *mut nfs_fattr ,,
    pub ): *mut *mut *mut int (discover_trunking)(struct nfs_server , struct nfs_fh,
    pub inode): *mut *mut void (enable_swap)(struct inode,
    pub inode): *mut *mut void (disable_swap)(struct inode,
}

//
// Helper functions used by NFS client and/or server
//
// Function vectors etc. for the NFS client
//
