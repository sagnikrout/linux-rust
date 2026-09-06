//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifsproto.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) International Business Machines  Corp., 2002,2008
// Author(s): Steve French (sfrench@us.ibm.com)
//

//
// All Prototypes
//
extern "C" {
    pub fn cifs_buf_release(buf_to_free: *mut c_void);
}
extern "C" {
    pub fn cifs_small_buf_release(buf_to_free: *mut c_void);
}
extern "C" {
    pub fn free_rsp_buf(resp_buftype: c_int, rsp: *mut c_void);
}
extern "C" {
    pub fn _get_xid() -> c_uint;
}
extern "C" {
    pub fn _free_xid(xid: c_uint);
}

extern "C" {
    pub fn init_cifs_idmap() -> c_int;
}
extern "C" {
    pub fn exit_cifs_idmap();
}
extern "C" {
    pub fn init_cifs_spnego() -> c_int;
}
extern "C" {
    pub fn exit_cifs_spnego();
}
extern "C" {
    pub fn __getname() -> return;
}
extern "C" {
    pub fn delete_mid(server: *mut TCP_Server_Info, mid: *mut mid_q_entry);
}
extern "C" {
    pub fn smb3_parse_devname(devname: *const c_char, ctx: *mut smb3_fs_context) -> c_int;
}
extern "C" {
    pub fn cifs_ipaddr_cmp(srcaddr: *mut sockaddr, rhs: *mut sockaddr) -> c_int;
}
extern "C" {
    pub fn cifs_match_ipaddr(srcaddr: *mut sockaddr, rhs: *mut sockaddr) -> bool;
}
extern "C" {
    pub fn cifs_discard_remaining_data(server: *mut TCP_Server_Info) -> c_int;
}
extern "C" {
    pub fn wait_for_response(server: *mut TCP_Server_Info, mid: *mut mid_q_entry) -> c_int;
}
extern "C" {
    pub fn smb2_query_server_interfaces(work: *mut work_struct);
}
extern "C" {
    pub fn cifs_reconnect(server: *mut TCP_Server_Info, mark_smb_session: bool) -> c_int;
}
extern "C" {
    pub fn backup_cred(cifs_sb: *mut cifs_sb_info) -> bool;
}
extern "C" {
    pub fn cifs_convert_address(dst: *mut sockaddr, src: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn cifs_set_port(addr: *mut sockaddr, port: unsigned short int);
}
extern "C" {
    pub fn cifs_NTtimeToUnix(ntutc: __le64) -> timespec64;
}
extern "C" {
    pub fn cifs_UnixTimeToNT(t: timespec64) -> u64;
}
extern "C" {
    pub fn cnvrtDosUnixTm(le_date: __le16, le_time: __le16, offset: c_int) -> timespec64;
}
extern "C" {
    pub fn cifs_set_oplock_level(cinode: *mut cifsInodeInfo, oplock: __u32);
}
extern "C" {
    pub fn cifs_get_writer(cinode: *mut cifsInodeInfo) -> c_int;
}
extern "C" {
    pub fn cifs_put_writer(cinode: *mut cifsInodeInfo);
}
extern "C" {
    pub fn cifs_done_oplock_break(cinode: *mut cifsInodeInfo);
}
extern "C" {
    pub fn cifs_push_mandatory_locks(cfile: *mut cifsFileInfo) -> c_int;
}
extern "C" {
    pub fn cifs_down_write(sem: *mut rw_semaphore);
}
extern "C" {
    pub fn cifs_fill_uniqueid(sb: *mut super_block, fattr: *mut cifs_fattr);
}
extern "C" {
    pub fn setup_authusers_ACE(pntace: *mut smb_ace) -> c_uint;
}
extern "C" {
    pub fn setup_special_user_owner_ACE(pntace: *mut smb_ace) -> c_uint;
}
extern "C" {
    pub fn cifs_setup_cifs_sb(cifs_sb: *mut cifs_sb_info) -> c_int;
}
extern "C" {
    pub fn cifs_mount_put_conns(mnt_ctx: *mut cifs_mount_ctx);
}
extern "C" {
    pub fn cifs_mount_get_session(mnt_ctx: *mut cifs_mount_ctx) -> c_int;
}
extern "C" {
    pub fn cifs_is_path_remote(mnt_ctx: *mut cifs_mount_ctx) -> c_int;
}
extern "C" {
    pub fn cifs_mount_get_tcon(mnt_ctx: *mut cifs_mount_ctx) -> c_int;
}
extern "C" {
    pub fn cifs_match_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn cifs_mount(cifs_sb: *mut cifs_sb_info, ctx: *mut smb3_fs_context) -> c_int;
}
extern "C" {
    pub fn cifs_umount(cifs_sb: *mut cifs_sb_info);
}
extern "C" {
    pub fn cifs_mark_open_files_invalid(tcon: *mut cifs_tcon);
}
extern "C" {
    pub fn cifs_reopen_persistent_handles(tcon: *mut cifs_tcon);
}
extern "C" {
    pub fn cifs_del_pending_open(open: *mut cifs_pending_open);
}
extern "C" {
    pub fn cifs_del_deferred_close(cfile: *mut cifsFileInfo);
}
extern "C" {
    pub fn cifs_close_deferred_file(cifs_inode: *mut cifsInodeInfo);
}
extern "C" {
    pub fn cifs_close_all_deferred_files(tcon: *mut cifs_tcon);
}
extern "C" {
    pub fn cifs_close_all_deferred_files_sb(cifs_sb: *mut cifs_sb_info);
}
extern "C" {
    pub fn cifs_put_tcp_session(server: *mut TCP_Server_Info, from_reconnect: c_int);
}
extern "C" {
    pub fn cifs_put_tcon(tcon: *mut cifs_tcon, trace: smb3_tcon_ref_trace);
}
extern "C" {
    pub fn cifs_release_automount_timer();
}
extern "C" {
    pub fn cifs_proc_init();
}
extern "C" {
    pub fn cifs_proc_clean();
}
extern "C" {
    pub fn cifs_move_llist(source: *mut list_head, dest: *mut list_head);
}
extern "C" {
    pub fn cifs_free_llist(llist: *mut list_head);
}
extern "C" {
    pub fn cifs_del_lock_waiters(lock: *mut cifsLockInfo);
}
extern "C" {
    pub fn cifs_tree_connect(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn sesInfoFree(buf_to_free: *mut cifs_ses);
}
extern "C" {
    pub fn tconInfoFree(tcon: *mut cifs_tcon, trace: smb3_tcon_ref_trace);
}
extern "C" {
    pub fn setup_ntlmv2_rsp(ses: *mut cifs_ses, nls_cp: *const nls_table) -> c_int;
}
extern "C" {
    pub fn cifs_crypto_secmech_release(server: *mut TCP_Server_Info);
}
extern "C" {
    pub fn calc_seckey(ses: *mut cifs_ses) -> c_int;
}

extern "C" {
    pub fn cifs_autodisable_serverino(cifs_sb: *mut cifs_sb_info, reason: *const c_char, rc: c_int);
}
extern "C" {
    pub fn couldbe_mf_symlink(fattr: *const cifs_fattr) -> bool;
}
extern "C" {
    pub fn __cifs_put_smb_ses(ses: *mut cifs_ses);
}
extern "C" {
    pub fn cifs_try_adding_channels(ses: *mut cifs_ses) -> c_int;
}
extern "C" {
    pub fn is_ses_using_iface(ses: *mut cifs_ses, iface: *mut cifs_server_iface) -> bool;
}
extern "C" {
    pub fn extract_unc_hostname(unc: *const c_char, h: *const c_char, len: *mut usize);
}
extern "C" {
    pub fn copy_path_name(dst: *mut c_char, src: *const c_char) -> c_int;
}
extern "C" {
    pub fn cifs_put_tcp_super(sb: *mut super_block);
}
extern "C" {
    pub fn cifs_update_super_prepath(cifs_sb: *mut cifs_sb_info, prefix: *mut c_char) -> c_int;
}
extern "C" {
    pub fn wire_mode_to_posix(wire: u32, is_dir: bool) -> umode_t;
}

// islink = false;

extern "C" {
    pub fn cifs_wait_for_server_reconnect(server: *mut TCP_Server_Info, retry: bool) -> c_int;
}
// Get an active reference of @ses and its children.
//
// NOTE: make sure to call this function when incrementing reference count of
// @ses to ensure that any DFS root session attached to it (@ses->dfs_root_ses)
// will also get its reference count incremented.
//
// cifs_put_smb_ses() will put all references, so call it when you're done.
//
// The first rqst has a transform header where the first 20 bytes are
// not part of the encrypted blob.
//
// Assumes the first rqst has a transform header as the first iov.
// I.e.
// rqst[0].rq_iov[0]  is transform header
// rqst[0].rq_iov[1+] data to be encrypted/decrypted
// rqst[1+].rq_iov[0+] data to be encrypted/decrypted
//
// We really don't want a mixture of pinned and unpinned pages
// in the sglist.  It's hard to keep track of which is what.
// Instead, we convert to a BVEC-type iterator higher up.
//
extern "C" {
    pub fn smb_EIO(_arg: smb_eio_trace_user_iter) -> return;
}
// We also don't want to have any extra refs or pins to clean
// up in the sglist.
//
extern "C" {
    pub fn smb_EIO(_arg: smb_eio_trace_extract_will_pin) -> return;
}
// We can not use the normal sg_set_buf() as we will sometimes pass a
// stack object as buf.
//
extern "C" {
    pub fn __cifs_get_writable_file(_arg: cifs_inode, _arg: find_flags, _arg: 0, _arg: ret_file) -> return;
}
extern "C" {
    pub fn __find_readable_file(_arg: cinode, _arg: find_flags, _arg: 0) -> return;
}
