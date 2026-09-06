//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/xdr4.h
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
// Server-side types for NFSv4.
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Kendrick Smith <kmsmith@umich.edu>
// Andy Adamson   <andros@umich.edu>
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

pub const NFSD4_MAX_TAGLEN: c_int = 128;

//
// nfsd4_encode_bool - Encode an XDR bool type result
// @xdr: target XDR stream
// @val: boolean value to encode
//
// Return values:
// %nfs_ok: @val encoded; @xdr advanced to next position
// %nfserr_resource: stream buffer space exhausted
//
// p = val ? xdr_one : xdr_zero;
//
// nfsd4_encode_uint32_t - Encode an XDR uint32_t type result
// @xdr: target XDR stream
// @val: integer value to encode
//
// Return values:
// %nfs_ok: @val encoded; @xdr advanced to next position
// %nfserr_resource: stream buffer space exhausted
//
// p = cpu_to_be32(val);

//
// nfsd4_encode_uint64_t - Encode an XDR uint64_t type result
// @xdr: target XDR stream
// @val: integer value to encode
//
// Return values:
// %nfs_ok: @val encoded; @xdr advanced to next position
// %nfserr_resource: stream buffer space exhausted
//

//
// nfsd4_encode_opaque_fixed - Encode a fixed-length XDR opaque type result
// @xdr: target XDR stream
// @data: pointer to data
// @size: length of data in bytes
//
// Return values:
// %nfs_ok: @data encoded; @xdr advanced to next position
// %nfserr_resource: stream buffer space exhausted
//
// nfsd4_encode_opaque - Encode a variable-length XDR opaque type result
// @xdr: target XDR stream
// @data: pointer to data
// @size: length of data in bytes
//
// Return values:
// %nfs_ok: @data encoded; @xdr advanced to next position
// %nfserr_resource: stream buffer space exhausted
//
// p++ = cpu_to_be32(size);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_compound_state {
    pub current_fh: svc_fh,
    pub save_fh: svc_fh,
    pub replay_owner: *mut nfs4_stateowner,
    pub clp: *mut nfs4_client,
// For sessions DRC
    pub session: *mut nfsd4_session,
    pub slot: *mut nfsd4_slot,
    pub data_offset: c_int,
    pub spo_must_allowed: bool,
    pub iovlen: usize,
    pub minorversion: u32,
    pub status: __be32,
    pub current_stateid: stateid_t,
    pub save_stateid: stateid_t,
// to indicate current and saved state id presents
    pub sid_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_change_info {
    pub atomic: u32,
    pub before_change: u64,
    pub after_change: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_access {
    pub /: *mut *mut u32 ac_req_access; / request,
    pub /: *mut *mut u32 ac_supported; / response,
    pub /: *mut *mut u32 ac_resp_access; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_close {
    pub /: *mut *mut u32 cl_seqid; / request,
    pub /: *mut *mut stateid_t cl_stateid; / request+response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_commit {
    pub /: *mut *mut u64 co_offset; / request,
    pub /: *mut *mut u32 co_count; / request,
    pub /: *mut *mut nfs4_verifier co_verf; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_create {
    pub /: *mut *mut u32 cr_namelen; / request,
    pub /: *mut *mut *mut char  cr_name; / request,
    pub /: *mut *mut u32 cr_type; / request,
    pub datalen: u32,
    pub data: *mut c_char,
    pub first: kvec,
    pub /: *mut *mut } link; / NF4LNK,
    pub specdata1: u32,
    pub specdata2: u32,
    pub /: *mut *mut } dev; / NF4BLK, NF4CHR,
    pub u: },
    pub /: *mut *mut u32 cr_bmval[3]; / request,
    pub /: *mut *mut iattr cr_iattr; / request,
    pub /: *mut *mut int cr_umask; / request,
    pub /: *mut *mut nfsd4_change_info cr_cinfo; / response,
    pub cr_acl: *mut nfs4_acl,
    pub cr_dpacl: *mut posix_acl,
    pub cr_pacl: *mut posix_acl,
    pub cr_label: xdr_netobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_delegreturn {
    pub dr_stateid: stateid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_getattr {
    pub /: *mut *mut u32 ga_bmval[3]; / request,
    pub /: *mut *mut *mut svc_fh ga_fhp; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_link {
    pub /: *mut *mut u32 li_namelen; / request,
    pub /: *mut *mut *mut char  li_name; / request,
    pub /: *mut *mut nfsd4_change_info li_cinfo; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_lock_denied {
    pub ld_clientid: clientid_t,
    pub ld_owner: xdr_netobj,
    pub ld_start: u64,
    pub ld_length: u64,
    pub ld_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_lock {
// request
    pub lk_type: u32,
    pub /: *mut *mut u32 lk_reclaim; / boolean,
    pub lk_offset: u64,
    pub lk_length: u64,
    pub lk_is_new: u32,
    pub open_seqid: u32,
    pub open_stateid: stateid_t,
    pub lock_seqid: u32,
    pub clientid: clientid_t,
    pub owner: xdr_netobj,
    pub new: },
    pub lock_stateid: stateid_t,
    pub lock_seqid: u32,
    pub old: },
    pub v: },
// response
    pub lk_resp_stateid: stateid_t,
    pub lk_denied: nfsd4_lock_denied,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_lockt {
    pub lt_type: u32,
    pub lt_clientid: clientid_t,
    pub lt_owner: xdr_netobj,
    pub lt_offset: u64,
    pub lt_length: u64,
    pub lt_denied: nfsd4_lock_denied,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_locku {
    pub lu_type: u32,
    pub lu_seqid: u32,
    pub lu_stateid: stateid_t,
    pub lu_offset: u64,
    pub lu_length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_lookup {
    pub /: *mut *mut u32 lo_len; / request,
    pub /: *mut *mut *mut char  lo_name; / request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_putfh {
    pub /: *mut *mut u32 pf_fhlen; / request,
    pub /: *mut *mut *mut char pf_fhval; / request,
    pub /: *mut *mut bool no_verify; / represents foreigh fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_getxattr {
    pub /: *mut *mut *mut char getxa_name; / request,
    pub /: *mut *mut u32 getxa_len; / request,
    pub getxa_buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_setxattr {
    pub /: *mut *mut u32 setxa_flags; / request,
    pub /: *mut *mut *mut char setxa_name; / request,
    pub /: *mut *mut *mut char setxa_buf; / request,
    pub /: *mut *mut u32 setxa_len; / request,
    pub /: *mut *mut nfsd4_change_info setxa_cinfo; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_removexattr {
    pub /: *mut *mut *mut char rmxa_name; / request,
    pub /: *mut *mut nfsd4_change_info rmxa_cinfo; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_listxattrs {
    pub /: *mut *mut u64 lsxa_cookie; / request,
    pub /: *mut *mut u32 lsxa_maxcount; / request,
    pub /: *mut *mut *mut char lsxa_buf; / unfiltered buffer (reply),
    pub /: *mut *mut u32 lsxa_len; / unfiltered len (reply),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_open {
    pub /: *mut *mut u32 op_claim_type; / request,
    pub op_fnamelen: u32,
    pub /: *mut *mut *mut char  op_fname; / request - everything but CLAIM_PREV,
    pub /: *mut *mut u32 op_delegate_type; / request - CLAIM_PREV only,
    pub /: *mut *mut stateid_t op_delegate_stateid; / request - response,
    pub /: *mut *mut u32 op_why_no_deleg; / response - DELEG_NONE_EXT only,
    pub /: *mut *mut u32 op_create; / request,
    pub /: *mut *mut u32 op_createmode; / request,
    pub /: *mut *mut int op_umask; / request,
    pub /: *mut *mut u32 op_bmval[3]; / request,
    pub /: *mut *mut iattr op_iattr; / UNCHECKED4, GUARDED4, EXCLUSIVE4_1,
    pub __attribute__((aligned(32))): nfs4_verifier op_verf,
// EXCLUSIVE4
    pub /: *mut *mut clientid_t op_clientid; / request,
    pub /: *mut *mut xdr_netobj op_owner; / request,
    pub /: *mut *mut u32 op_seqid; / request,
    pub /: *mut *mut u32 op_share_access; / request,
    pub /: *mut *mut u32 op_share_deny; / request,
    pub /: *mut *mut u32 op_deleg_want; / request,
    pub /: *mut *mut stateid_t op_stateid; / response,
    pub /: *mut *mut __be32 op_xdr_error; / see nfsd4_open_omfg(),
    pub /: *mut *mut nfsd4_change_info op_cinfo; / response,
    pub /: *mut *mut u32 op_rflags; / response,
    pub /: *mut *mut bool op_recall; / response,
    pub /: *mut *mut bool op_truncate; / used during processing,
    pub /: *mut *mut bool op_created; / used during processing,
    pub /: *mut *mut *mut nfs4_openowner op_openowner; / used during processing,
    pub /: *mut *mut *mut file op_filp; / used during processing,
    pub /: *mut *mut *mut nfs4_file op_file; / used during processing,
    pub /: *mut *mut *mut nfs4_ol_stateid op_stp; / used during processing,
    pub /: *mut *mut *mut nfs4_clnt_odstate op_odstate; / used during processing,
    pub op_acl: *mut nfs4_acl,
    pub op_dpacl: *mut posix_acl,
    pub op_pacl: *mut posix_acl,
    pub op_label: xdr_netobj,
    pub op_rqstp: *mut svc_rqst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_open_confirm {
    pub /: *mut *mut stateid_t oc_req_stateid / request,
    pub /: *mut *mut u32 oc_seqid / request,
    pub /: *mut *mut stateid_t oc_resp_stateid / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_open_downgrade {
    pub od_stateid: stateid_t,
    pub od_seqid: u32,
    pub /: *mut *mut u32 od_share_access; / request,
    pub /: *mut *mut u32 od_deleg_want; / request,
    pub /: *mut *mut u32 od_share_deny; / request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_read {
    pub /: *mut *mut stateid_t rd_stateid; / request,
    pub /: *mut *mut u64 rd_offset; / request,
    pub /: *mut *mut u32 rd_length; / request,
    pub rd_vlen: c_int,
    pub rd_nf: *mut nfsd_file,
    pub /: *mut *mut *mut svc_rqst rd_rqstp; / response,
    pub /: *mut *mut *mut svc_fh rd_fhp; / response,
    pub /: *mut *mut u32 rd_eof; / response,
}

//
// Cache the case-folding properties of @dir so a batched encoder
// (e.g., READDIR) does not re-probe per child. @dir is the
// directory being read, held by the request, so it is stable
// against rename for the duration of the cache's lifetime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_case_attrs_cache {
    pub dir: *mut dentry,
    pub valid: bool,
    pub insensitive: bool,
    pub preserving: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_readdir {
    pub /: *mut *mut u64 rd_cookie; / request,
    pub /: *mut *mut nfs4_verifier rd_verf; / request,
    pub /: *mut *mut u32 rd_dircount; / request,
    pub /: *mut *mut u32 rd_maxcount; / request,
    pub /: *mut *mut u32 rd_bmval[3]; / request,
    pub /: *mut *mut *mut svc_rqst rd_rqstp; / response,
    pub /: *mut *mut *mut svc_fh  rd_fhp; / response,
    pub common: readdir_cd,
    pub xdr: *mut xdr_stream,
    pub cookie_offset: c_int,
    pub rd_case_cache: nfsd_case_attrs_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_release_lockowner {
    pub rl_clientid: clientid_t,
    pub rl_owner: xdr_netobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_readlink {
    pub /: *mut *mut *mut svc_rqst rl_rqstp; / request,
    pub /: *mut *mut *mut svc_fh  rl_fhp; / request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_remove {
    pub /: *mut *mut u32 rm_namelen; / request,
    pub /: *mut *mut *mut char  rm_name; / request,
    pub /: *mut *mut nfsd4_change_info rm_cinfo; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_rename {
    pub /: *mut *mut u32 rn_snamelen; / request,
    pub /: *mut *mut *mut char  rn_sname; / request,
    pub /: *mut *mut u32 rn_tnamelen; / request,
    pub /: *mut *mut *mut char  rn_tname; / request,
    pub /: *mut *mut nfsd4_change_info rn_sinfo; / response,
    pub /: *mut *mut nfsd4_change_info rn_tinfo; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_secinfo {
    pub /: *mut *mut u32 si_namelen; / request,
    pub /: *mut *mut *mut char si_name; / request,
    pub /: *mut *mut *mut svc_export si_exp; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_secinfo_no_name {
    pub /: *mut *mut u32 sin_style; / request,
    pub /: *mut *mut *mut svc_export sin_exp; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_setattr {
    pub /: *mut *mut stateid_t sa_stateid; / request,
    pub /: *mut *mut u32 sa_bmval[3]; / request,
    pub /: *mut *mut iattr sa_iattr; / request,
    pub sa_acl: *mut nfs4_acl,
    pub sa_label: xdr_netobj,
    pub sa_dpacl: *mut posix_acl,
    pub sa_pacl: *mut posix_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_setclientid {
    pub /: *mut *mut nfs4_verifier se_verf; / request,
    pub se_name: xdr_netobj,
    pub /: *mut *mut u32 se_callback_prog; / request,
    pub /: *mut *mut u32 se_callback_netid_len; / request,
    pub /: *mut *mut *mut char  se_callback_netid_val; / request,
    pub /: *mut *mut u32 se_callback_addr_len; / request,
    pub /: *mut *mut *mut char  se_callback_addr_val; / request,
    pub /: *mut *mut u32 se_callback_ident; / request,
    pub /: *mut *mut clientid_t se_clientid; / response,
    pub /: *mut *mut nfs4_verifier se_confirm; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_setclientid_confirm {
    pub sc_clientid: clientid_t,
    pub sc_confirm: nfs4_verifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_test_stateid_id {
    pub ts_id_status: __be32,
    pub ts_id_stateid: stateid_t,
    pub ts_id_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_test_stateid {
    pub ts_num_ids: u32,
    pub ts_stateid_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_free_stateid {
    pub /: *mut *mut stateid_t fr_stateid; / request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_get_dir_delegation {
// request
    pub gdda_signal_deleg_avail: u32,
    pub gdda_notification_types: [u32; 1],
    pub gdda_child_attr_delay: timespec64,
    pub gdda_dir_attr_delay: timespec64,
    pub gdda_child_attributes: [u32; 3],
    pub gdda_dir_attributes: [u32; 3],
// response
    pub gddrnf_status: u32,
    pub gddr_cookieverf: nfs4_verifier,
    pub gddr_stateid: stateid_t,
    pub gddr_notification: [u32; 1],
    pub gddr_child_attributes: [u32; 3],
    pub gddr_dir_attributes: [u32; 3],
    pub gddrnf_will_signal_deleg_avail: bool,
}

// also used for NVERIFY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_verify {
    pub /: *mut *mut u32 ve_bmval[3]; / request,
    pub /: *mut *mut u32 ve_attrlen; / request,
    pub /: *mut *mut *mut char  ve_attrval; / request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_write {
    pub /: *mut *mut stateid_t wr_stateid; / request,
    pub /: *mut *mut u64 wr_offset; / request,
    pub /: *mut *mut u32 wr_stable_how; / request,
    pub /: *mut *mut u32 wr_buflen; / request,
    pub /: *mut *mut xdr_buf wr_payload; / request,
    pub /: *mut *mut u32 wr_bytes_written; / response,
    pub /: *mut *mut u32 wr_how_written; / response,
    pub /: *mut *mut nfs4_verifier wr_verifier; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_exchange_id {
    pub verifier: nfs4_verifier,
    pub clname: xdr_netobj,
    pub flags: u32,
    pub clientid: clientid_t,
    pub seqid: u32,
    pub spa_how: u32,
    pub spo_must_enforce: [u32; 3],
    pub spo_must_allow: [u32; 3],
    pub nii_domain: xdr_netobj,
    pub nii_name: xdr_netobj,
    pub nii_time: timespec64,
    pub server_impl_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_sequence {
    pub /: *mut *mut nfs4_sessionid sessionid; / request/response,
    pub /: *mut *mut u32 seqid; / request/response,
    pub /: *mut *mut u32 slotid; / request/response,
    pub /: *mut *mut u32 maxslots; / request,
    pub /: *mut *mut u32 cachethis; / request,
    pub /: *mut *mut u32 maxslots_response; / response,
    pub /: *mut *mut u32 target_maxslots; / response,
    pub /: *mut *mut u32 status_flags; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_destroy_session {
    pub sessionid: nfs4_sessionid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_destroy_clientid {
    pub clientid: clientid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_reclaim_complete {
    pub rca_one_fs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_deviceid {
    pub fsid_idx: u64,
    pub generation: u32,
}

// q = ( __be64)devid->fsid_idx;
// p++ = ( __be32)devid->generation;
// p++ = xdr_zero;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_layout_seg {
    pub iomode: u32,
    pub offset: u64,
    pub length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_getdeviceinfo {
    pub /: *mut *mut nfsd4_deviceid gd_devid; / request,
    pub /: *mut *mut u32 gd_layout_type; / request,
    pub /: *mut *mut u32 gd_maxcount; / request,
    pub /: *mut *mut u32 gd_notify_types;/ request - response,
    pub /: *mut *mut *mut void gd_device; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_layoutget {
    pub /: *mut *mut u64 lg_minlength; / request,
    pub /: *mut *mut u32 lg_signal; / request,
    pub /: *mut *mut u32 lg_layout_type; / request,
    pub /: *mut *mut u32 lg_maxcount; / request,
    pub /: *mut *mut stateid_t lg_sid; / request/response,
    pub /: *mut *mut nfsd4_layout_seg lg_seg; / request/response,
    pub /: *mut *mut *mut void lg_content; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_layoutcommit {
    pub /: *mut *mut stateid_t lc_sid; / request,
    pub /: *mut *mut nfsd4_layout_seg lc_seg; / request,
    pub /: *mut *mut u32 lc_reclaim; / request,
    pub /: *mut *mut u32 lc_newoffset; / request,
    pub /: *mut *mut u64 lc_last_wr; / request,
    pub /: *mut *mut timespec64 lc_mtime; / request,
    pub /: *mut *mut u32 lc_layout_type; / request,
    pub /: *mut *mut xdr_buf lc_up_layout; / decoded by callback,
    pub /: *mut *mut bool lc_size_chg; / response,
    pub /: *mut *mut u64 lc_newsize; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_layoutreturn {
    pub /: *mut *mut u32 lr_return_type; / request,
    pub /: *mut *mut u32 lr_layout_type; / request,
    pub /: *mut *mut nfsd4_layout_seg lr_seg; / request,
    pub /: *mut *mut u32 lr_reclaim; / request,
    pub /: *mut *mut u32 lrf_body_len; / request,
    pub /: *mut *mut *mut void lrf_body; / request,
    pub /: *mut *mut stateid_t lr_sid; / request/response,
    pub /: *mut *mut bool lrs_present; / response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_fallocate {
// request
    pub falloc_stateid: stateid_t,
    pub falloc_offset: loff_t,
    pub falloc_length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_clone {
// request
    pub cl_src_stateid: stateid_t,
    pub cl_dst_stateid: stateid_t,
    pub cl_src_pos: u64,
    pub cl_dst_pos: u64,
    pub cl_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd42_write_res {
    pub wr_bytes_written: u64,
    pub wr_stable_how: u32,
    pub wr_verifier: nfs4_verifier,
    pub cb_stateid: stateid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_cb_offload {
    pub co_cb: nfsd4_callback,
    pub co_res: nfsd42_write_res,
    pub co_nfserr: __be32,
    pub co_retries: c_uint,
    pub co_fh: knfsd_fh,
    pub co_referring_sessionid: nfs4_sessionid,
    pub co_referring_slotid: u32,
    pub co_referring_seqno: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_copy {
// request
    pub cp_src_stateid: stateid_t,
    pub cp_dst_stateid: stateid_t,
    pub cp_src_pos: u64,
    pub cp_dst_pos: u64,
    pub cp_count: u64,
    pub cp_src: *mut nl4_server,
    pub cp_flags: c_ulong,

// response
    pub nfserr: __be32,
    pub cp_res: nfsd42_write_res,
    pub fh: knfsd_fh,
    pub cp_clp: *mut nfs4_client,
    pub nf_src: *mut nfsd_file,
    pub nf_dst: *mut nfsd_file,
    pub attr_update: bool,
    pub ss_nsui: *mut nfsd4_ssc_umount_item,
    pub c_fh: nfs_fh,
    pub stateid: nfs4_stateid,
    pub cp_nn: *mut nfsd_net,
}

//
// Durable state for an async (background) server-side COPY.
//
// struct nfsd4_copy is transient: it lives in the COMPOUND argument buffer
// and is reused once the op returns. An async COPY outlives the COMPOUND
// (worker kthread, reaper linkage, CB_OFFLOAD), so its params and result are
// snapshotted into the embedded cp_copy and it never points into the request
// buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_async_copy {
    pub /: *mut *mut nfs4_stid cp_stid; / SC_TYPE_COPY, in cl_stateids,
    pub /: *mut *mut nfsd4_copy cp_copy; / operation params + result,
    pub /: *mut *mut list_head copies; / nfs4_client.async_copies,
    pub copy_task: *mut task_struct,
    pub refcount: refcount_t,
    pub cp_ttl: c_uint,
    pub cp_cb_offload: nfsd4_cb_offload,
}

extern "C" {
    pub fn test_bit(_arg: NFSD4_COPY_F_SYNCHRONOUS, _arg: &copy->cp_flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_seek {
// request
    pub seek_stateid: stateid_t,
    pub seek_offset: loff_t,
    pub seek_whence: u32,
// response
    pub seek_eof: u32,
    pub seek_pos: loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_offload_status {
// request
    pub stateid: stateid_t,
// response
    pub count: u64,
    pub status: __be32,
    pub completed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_copy_notify {
// request
    pub cpn_src_stateid: stateid_t,
    pub cpn_dst: *mut nl4_server,
// response
    pub cpn_cnr_stateid: stateid_t,
    pub cpn_lease_time: timespec64,
    pub cpn_src: *mut nl4_server,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_op {
    pub opnum: u32,
    pub status: __be32,
    pub opdesc: *const nfsd4_operation,
    pub replay: *mut nfs4_replay,
#[repr(C)]
#[derive(Copy, Clone)]
pub union nfsd4_op_u {
    pub access: nfsd4_access,
    pub close: nfsd4_close,
    pub commit: nfsd4_commit,
    pub create: nfsd4_create,
    pub delegreturn: nfsd4_delegreturn,
    pub getattr: nfsd4_getattr,
    pub getfh: *mut *mut svc_fh,
    pub link: nfsd4_link,
    pub lock: nfsd4_lock,
    pub lockt: nfsd4_lockt,
    pub locku: nfsd4_locku,
    pub lookup: nfsd4_lookup,
    pub nverify: nfsd4_verify,
    pub open: nfsd4_open,
    pub open_confirm: nfsd4_open_confirm,
    pub open_downgrade: nfsd4_open_downgrade,
    pub putfh: nfsd4_putfh,
    pub read: nfsd4_read,
    pub readdir: nfsd4_readdir,
    pub readlink: nfsd4_readlink,
    pub remove: nfsd4_remove,
    pub rename: nfsd4_rename,
    pub renew: clientid_t,
    pub secinfo: nfsd4_secinfo,
    pub setattr: nfsd4_setattr,
    pub setclientid: nfsd4_setclientid,
    pub setclientid_confirm: nfsd4_setclientid_confirm,
    pub verify: nfsd4_verify,
    pub write: nfsd4_write,
    pub release_lockowner: nfsd4_release_lockowner,
// NFSv4.1
    pub exchange_id: nfsd4_exchange_id,
    pub backchannel_ctl: nfsd4_backchannel_ctl,
    pub bind_conn_to_session: nfsd4_bind_conn_to_session,
    pub create_session: nfsd4_create_session,
    pub destroy_session: nfsd4_destroy_session,
    pub destroy_clientid: nfsd4_destroy_clientid,
    pub sequence: nfsd4_sequence,
    pub reclaim_complete: nfsd4_reclaim_complete,
    pub test_stateid: nfsd4_test_stateid,
    pub free_stateid: nfsd4_free_stateid,
    pub get_dir_delegation: nfsd4_get_dir_delegation,
    pub getdeviceinfo: nfsd4_getdeviceinfo,
    pub layoutget: nfsd4_layoutget,
    pub layoutcommit: nfsd4_layoutcommit,
    pub layoutreturn: nfsd4_layoutreturn,
    pub secinfo_no_name: nfsd4_secinfo_no_name,
// NFSv4.2
    pub allocate: nfsd4_fallocate,
    pub deallocate: nfsd4_fallocate,
    pub clone: nfsd4_clone,
    pub copy: nfsd4_copy,
    pub offload_status: nfsd4_offload_status,
    pub copy_notify: nfsd4_copy_notify,
    pub seek: nfsd4_seek,
    pub getxattr: nfsd4_getxattr,
    pub setxattr: nfsd4_setxattr,
    pub listxattrs: nfsd4_listxattrs,
    pub removexattr: nfsd4_removexattr,
    pub u: },
}

extern "C" {
    pub fn nfsd4_cache_this_op(: *mut nfsd4_op) -> bool;
}
//
// Memory needed just for the duration of processing one compound:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svcxdr_tmpbuf {
    pub next: *mut svcxdr_tmpbuf,
    pub buf: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_compoundargs {
// scratch variables for XDR decode
    pub xdr: *mut xdr_stream,
    pub to_free: *mut svcxdr_tmpbuf,
    pub rqstp: *mut svc_rqst,
    pub tag: *mut *mut c_char,
    pub taglen: u32,
    pub minorversion: u32,
    pub client_opcnt: u32,
    pub opcnt: u32,
    pub splice_ok: bool,
    pub ops: *mut nfsd4_op,
    pub iops: [nfsd4_op; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_compoundres {
// scratch variables for XDR encode
    pub xdr: *mut xdr_stream,
    pub rqstp: *mut *mut svc_rqst,
    pub statusp: *mut __be32,
    pub tag: *mut *mut c_char,
    pub taglen: u32,
    pub opcnt: u32,
    pub cstate: nfsd4_compound_state,
}

extern "C" {
    pub fn nfsd4_max_reply(rqstp: *mut svc_rqst, op: *mut nfsd4_op) -> c_int;
}
extern "C" {
    pub fn warn_on_nonidempotent_op(op: *mut nfsd4_op);
}

extern "C" {
    pub fn nfsd4_mach_creds_match(cl: *mut nfs4_client, rqstp: *mut svc_rqst) -> bool;
}
extern "C" {
    pub fn nfs4svc_decode_compoundargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs4svc_encode_compoundres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfsd4_check_resp_size(: *mut nfsd4_compoundres, _arg: u32) -> __be32;
}
extern "C" {
    pub fn nfsd4_encode_operation(: *mut nfsd4_compoundres, : *mut nfsd4_op);
}
extern "C" {
    pub fn nfsd4_encode_replay(xdr: *mut xdr_stream, op: *mut nfsd4_op);
}
extern "C" {
    pub fn nfsd4_exchange_id_release(u: *mut nfsd4_op_u);
}
extern "C" {
    pub fn nfsd4_sequence_done(resp: *mut nfsd4_compoundres);
}
extern "C" {
    pub fn nfsd4_cstate_clear_replay(cstate: *mut nfsd4_compound_state);
}
extern "C" {
    pub fn nfsd4_lock_release(u: *mut nfsd4_op_u);
}
extern "C" {
    pub fn nfsd4_lockt_release(u: *mut nfsd4_op_u);
}
extern "C" {
    pub fn nfsd4_release_compoundargs(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn nfsd4_bump_seqid(: *mut nfsd4_compound_state, nfserr: __be32);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd4_op_flags {
    ALLOWED_WITHOUT_FH = 1 << 0,    /* No current filehandle required */
    ALLOWED_ON_ABSENT_FS = 1 << 1,  /* ops processed on absent fs */
    ALLOWED_AS_FIRST_OP = 1 << 2,   /* ops reqired first in compound */
// For rfc 5661 section 2.6.3.1.1:
    OP_HANDLES_WRONGSEC = 1 << 3,
    OP_IS_PUTFH_LIKE = 1 << 4,
//
// These are the ops whose result size we estimate before
// encoding, to avoid performing an op then not being able to
// respond or cache a response.  This includes writes and setattrs
// as well as the operations usually called "nonidempotent":
//
    OP_MODIFIES_SOMETHING = 1 << 5,
//
// Cache compounds containing these ops in the xid-based drc:
// We use the DRC for compounds containing non-idempotent
// operations, *except* those that are 4.1-specific (since
// sessions provide their own EOS), and except for stateful
// operations other than setclientid and setclientid_confirm
// (since sequence numbers provide EOS for open, lock, etc in
// the v4.0 case).
//
    OP_CACHEME = 1 << 6,
//
// These are ops which clear current state id.
//
    OP_CLEAR_STATEID = 1 << 7,
// Most ops return only an error on failure; some may do more:
    OP_NONTRIVIAL_ERROR_ENCODE = 1 << 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_operation {
    pub ): *mut nfsd4_op_u,
    pub ): *mut *mut void (op_release)(union nfsd4_op_u,
    pub op_flags: u32,
    pub op_name: *mut c_char,
// Try to get response size before operation
    pub op): *const nfsd4_op,
    pub ): *mut nfsd4_op_u,
    pub ): *mut nfsd4_op_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_cb_recall_any {
    pub ra_cb: nfsd4_callback,
    pub ra_keep: u32,
    pub ra_bmval: [u32; 1],
}
