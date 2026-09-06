//! Automatically rewritten from C Header to Rust Module
//! Source: fs/notify/fanotify/fanotify.h
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

// Possible states of the permission event
//
// 3 dwords are sufficient for most local fs (64bit ino, 32bit generation).
// fh buf should be dword aligned. On 64bit arch, the ext_buf pointer is
// stored in either the first or last 2 dwords.
//

// Fixed size struct for file handle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_fh {
    pub type: u8,
    pub len: u8,
pub const FANOTIFY_FH_FLAG_EXT_BUF: c_int = 1;
    pub flags: u8,
    pub pad: u8,
    pub __aligned(4): },
// Variable size struct for dir file handle + child file handle + name
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_info {
// size of dir_fh/file_fh including fanotify_fh hdr size
    pub dir_fh_totlen: u8,
    pub dir2_fh_totlen: u8,
    pub file_fh_totlen: u8,
    pub name_len: u8,
    pub name2_len: u8,
    pub pad: [u8; 3],
    pub buf: [c_uchar; ],
//
// (struct fanotify_fh) dir_fh starts at buf[0]
// (optional) dir2_fh starts at buf[dir_fh_totlen]
// (optional) file_fh starts at buf[dir_fh_totlen + dir2_fh_totlen]
// name starts at buf[dir_fh_totlen + dir2_fh_totlen + file_fh_totlen]
// ...
//

pub const FANOTIFY_DIR_FH_OFFSET(info): c_int = 0;

    pub __aligned(4): },
    pub FANOTIFY_FH_FLAG_EXT_BUF): return (fh->flags &,
    pub 4): BUILD_BUG_ON(FANOTIFY_FH_HDR_LEN %,
    pub )): *mut *mut *mut return (char )ALIGN((unsigned long)(fh + 1), __alignof__(char,
    pub fanotify_fh_ext_buf_ptr(fh): *mut return,
    pub 1: return fanotify_fh_has_ext_buf(fh) ? fanotify_fh_ext_buf(fh) : fh +,
    pub 0: return,
    pub FANOTIFY_FH_HDR_LEN: return info->dir_fh_totlen -,
    pub 4): BUILD_BUG_ON(offsetof(struct fanotify_info, buf) %,
    pub )FANOTIFY_DIR_FH_BUF(info): *mut return (struct fanotify_fh,
    pub 0: return,
    pub FANOTIFY_FH_HDR_LEN: return info->dir2_fh_totlen -,
    pub )FANOTIFY_DIR2_FH_BUF(info): *mut return (struct fanotify_fh,
    pub 0: return,
    pub FANOTIFY_FH_HDR_LEN: return info->file_fh_totlen -,
    pub )FANOTIFY_FILE_FH_BUF(info): *mut return (struct fanotify_fh,
    pub NULL: return,
    pub FANOTIFY_NAME_BUF(info): return,
    pub NULL: return,
    pub FANOTIFY_NAME2_BUF(info): return,
    pub U8_MAX): BUILD_BUG_ON(FANOTIFY_FH_HDR_LEN + MAX_HANDLE_SZ >,
    pub U8_MAX): BUILD_BUG_ON(NAME_MAX >,
    pub 0: info->dir_fh_totlen =,
    pub 0: info->dir2_fh_totlen =,
    pub 0: info->file_fh_totlen =,
    pub 0: info->name_len =,
    pub 0: info->name2_len =,
// These set/copy helpers MUST be called by order
    pub totlen: info->dir_fh_totlen =,
    pub totlen: info->dir2_fh_totlen =,
    pub totlen: info->file_fh_totlen =,
    pub name->len: info->name_len =,
    pub 1): strscpy(fanotify_info_name(info), name->name, name->len +,
    pub name->len: info->name2_len =,
    pub 1): strscpy(fanotify_info_name2(info), name->name, name->len +,
//
// Common structure for fanotify events. Concrete structs are allocated in
// fanotify_handle_event() and freed when the information is retrieved by
// userspace. The type of event determines how it was allocated, how it will
// be freed and which concrete struct it may be cast to.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fanotify_event_type {
    FANOTIFY_EVENT_TYPE_FID, /* fixed length */
    FANOTIFY_EVENT_TYPE_FID_NAME, /* variable length */
    FANOTIFY_EVENT_TYPE_PATH,
    FANOTIFY_EVENT_TYPE_PATH_PERM,
    FANOTIFY_EVENT_TYPE_OVERFLOW, /* struct fanotify_event */
    FANOTIFY_EVENT_TYPE_FS_ERROR, /* struct fanotify_error_event */
    FANOTIFY_EVENT_TYPE_MNT,
    __FANOTIFY_EVENT_TYPE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event {
    pub fse: fsnotify_event,
    pub /: *mut *mut hlist_node merge_list; / List for hashed merge,
    pub mask: u32,
    pub FANOTIFY_EVENT_TYPE_BITS: unsigned int type :,
    pub FANOTIFY_EVENT_HASH_BITS: unsigned int hash :,
}

// Space for filehandle - access with fanotify_fh_buf() */	\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_fid_event {
    pub fae: fanotify_event,
    pub fsid: __kernel_fsid_t,
    pub FANOTIFY_INLINE_FH_LEN): FANOTIFY_INLINE_FH(object_fh,,
}

extern "C" {
    pub fn container_of(_arg: event, fanotify_fid_event: struct, _arg: fae) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_name_event {
    pub fae: fanotify_event,
    pub fsid: __kernel_fsid_t,
    pub info: fanotify_info,
}

extern "C" {
    pub fn container_of(_arg: event, fanotify_name_event: struct, _arg: fae) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_error_event {
    pub fae: fanotify_event,
    pub /: *mut *mut s32 error; / Error reported by the Filesystem.,
    pub /: *mut *mut u32 err_count; / Suppressed errors count,
    pub /: *mut *mut __kernel_fsid_t fsid; / FSID this error refers to.,
    pub MAX_HANDLE_SZ): FANOTIFY_INLINE_FH(object_fh,,
}

extern "C" {
    pub fn container_of(_arg: event, fanotify_error_event: struct, _arg: fae) -> return;
}
extern "C" {
    pub fn fanotify_info_file_fh(_arg: &FANOTIFY_NE(event)->info) -> return;
}
// For error events, even zeroed fh are reported.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_path_event {
    pub fae: fanotify_event,
    pub path: path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_mnt_event {
    pub fae: fanotify_event,
    pub mnt_id: u64,
}

extern "C" {
    pub fn container_of(_arg: event, fanotify_path_event: struct, _arg: fae) -> return;
}
extern "C" {
    pub fn container_of(_arg: event, fanotify_mnt_event: struct, _arg: fae) -> return;
}

//
// Structure for permission fanotify events. It gets allocated and freed in
// fanotify_handle_event() since we wait there for user response. When the
// information is retrieved by userspace the structure is moved from
// group->notification_list to group->fanotify_data.access_list to wait for
// user response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_perm_event {
    pub fae: fanotify_event,
    pub path: path,
    pub /: *mut *mut loff_t pos; / FANOTIFY_NO_RANGE if unavailable,
    pub count: usize,
    pub /: *mut *mut u32 response; / userspace answer to the event,
    pub /: *mut *mut unsigned short state; / state of the event,
    pub /: *mut *mut unsigned short watchdog_cnt; / already scanned by watchdog?,
    pub /: *mut *mut int fd; / fd we passed to userspace for this event,
    pub /: *mut *mut pid_t recv_pid; / pid of task receiving the event,
    pub hdr: fanotify_response_info_header,
    pub audit_rule: fanotify_response_info_audit_rule,
}

extern "C" {
    pub fn container_of(_arg: event, fanotify_perm_event: struct, _arg: fae) -> return;
}
extern "C" {
    pub fn container_of(_arg: fse, fanotify_event: struct, _arg: fse) -> return;
}
//
// Use 128 size hash table to speed up events merge.
//

//
// Permission events and overflow event do not get merged - don't hash them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_mark {
    pub fsn_mark: fsnotify_mark,
    pub fsid: __kernel_fsid_t,
}

extern "C" {
    pub fn container_of(_arg: mark, fanotify_mark: struct, _arg: fsn_mark) -> return;
}
