//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/9p/client.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// 9P Client Definitions
//
// Copyright (C) 2008 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2007 by Latchesar Ionkov <lucho@ionkov.net>
//

// Number of requests per row
pub const P9_ROW_MAXTAG: c_int = 255;
// DEFAULT MSIZE = 32 pages worth of payload + P9_HDRSZ +
// room for write (16 extra) or read (11 extra) operands.
//

// enum p9_proto_versions - 9P protocol versions
// @p9_proto_legacy: 9P Legacy mode, pre-9P2000.u
// @p9_proto_2000u: 9P2000.u extension
// @p9_proto_2000L: 9P2000.L extension
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_proto_versions {
    p9_proto_legacy,
    p9_proto_2000u,
    p9_proto_2000L,
}

//
// enum p9_trans_status - different states of underlying transports
// @Connected: transport is connected and healthy
// @Disconnected: transport has been disconnected
// @Hung: transport is connected by wedged
//
// This enumeration details the various states a transport
// instatiation can be in.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_trans_status {
    Connected,
    BeginDisconnect,
    Disconnected,
    Hung,
}

//
// enum p9_req_status_t - status of a request
// @REQ_STATUS_ALLOC: request has been allocated but not sent
// @REQ_STATUS_UNSENT: request waiting to be sent
// @REQ_STATUS_SENT: request sent to server
// @REQ_STATUS_RCVD: response received from server
// @REQ_STATUS_FLSHD: request has been flushed
// @REQ_STATUS_ERROR: request encountered an error on the client side
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_req_status_t {
    REQ_STATUS_ALLOC,
    REQ_STATUS_UNSENT,
    REQ_STATUS_SENT,
    REQ_STATUS_RCVD,
    REQ_STATUS_FLSHD,
    REQ_STATUS_ERROR,
}

//
// struct p9_req_t - request slots
// @status: status of this request slot
// @t_err: transport error
// @wq: wait_queue for the client to block on for this request
// @tc: the request fcall structure
// @rc: the response fcall structure
// @req_list: link for higher level objects to chain requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_req_t {
    pub status: c_int,
    pub t_err: c_int,
    pub refcount: refcount_t,
    pub wq: wait_queue_head_t,
    pub tc: p9_fcall,
    pub rc: p9_fcall,
    pub req_list: list_head,
}

//
// struct p9_client - per client instance state
// @lock: protect @fids and @reqs
// @msize: maximum data size negotiated by protocol
// @proto_version: 9P protocol version to use
// @trans_mod: module API instantiated with this client
// @status: connection state
// @trans: tranport instance state and API
// @fids: All active FID handles
// @reqs: All active requests.
// @name: node name used as client id
//
// The client structure is used to keep track of various per-client
// state that has been instantiated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_client {
    pub lock: spinlock_t,
    pub msize: c_uint,
    pub proto_version: c_uchar,
    pub trans_mod: *mut p9_trans_module,
    pub status: p9_trans_status,
    pub trans: *mut c_void,
    pub fcall_cache: *mut kmem_cache,
    pub rfd: c_int,
    pub wfd: c_int,
    pub fd: },
    pub port: u16,
    pub privport: bool,
    pub tcp: },
    pub trans_opts: },
    pub fids: idr,
    pub reqs: idr,
    pub 1]: char name[__NEW_UTS_LEN +,
}

//
// struct p9_fd_opts - holds client options during parsing
// @msize: maximum data size negotiated by protocol
// @prot-Oversion: 9P protocol version to use
// @trans_mod: module API instantiated with this client
//
// These parsed options get transferred into client in
// apply_client_options()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_client_opts {
    pub msize: c_uint,
    pub proto_version: c_uchar,
    pub trans_mod: *mut p9_trans_module,
}

//
// struct p9_fd_opts - per-transport options for fd transport
// @rfd: file descriptor for reading (trans=fd)
// @wfd: file descriptor for writing (trans=fd)
// @port: port to connect to (trans=tcp)
// @privport: port is privileged
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_fd_opts {
    pub rfd: c_int,
    pub wfd: c_int,
    pub port: u16,
    pub privport: bool,
}

//
// struct p9_rdma_opts - Collection of mount options for rdma transport
// @port: port of connection
// @privport: Whether a privileged port may be used
// @sq_depth: The requested depth of the SQ. This really doesn't need
// to be any deeper than the number of threads used in the client
// @rq_depth: The depth of the RQ. Should be greater than or equal to SQ depth
// @timeout: Time to wait in msecs for CM events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_rdma_opts {
    pub port: c_short,
    pub privport: bool,
    pub sq_depth: c_int,
    pub rq_depth: c_int,
    pub timeout: c_long,
}

//
// struct p9_session_opts - holds parsed options for v9fs_session_info
// @flags: session options of type &p9_session_flags
// @nodev: set to 1 to disable device mapping
// @debug: debug level
// @afid: authentication handle
// @cache: cache mode of type &p9_cache_bits
// @cachetag: the tag of the cache associated with this session
// @uname: string user name to mount hierarchy as
// @aname: mount specifier for remote hierarchy
// @dfltuid: default numeric userid to mount hierarchy as
// @dfltgid: default numeric groupid to mount hierarchy as
// @uid: if %V9FS_ACCESS_SINGLE, the numeric uid which mounted the hierarchy
// @session_lock_timeout: retry interval for blocking locks
// @ndentry_timeout_ms: Negative dentry lookup cache retention time in ms
//
// This strucure holds options which are parsed and will be transferred
// to the v9fs_session_info structure when mounted, and therefore largely
// duplicates struct v9fs_session_info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_session_opts {
    pub flags: c_uint,
    pub nodev: c_uchar,
    pub debug: c_ushort,
    pub afid: c_uint,
    pub cache: c_uint,
    pub ndentry_timeout_ms: c_uint,

    pub cachetag: *mut c_char,

    pub uname: *mut c_char,
    pub aname: *mut c_char,
    pub dfltuid: kuid_t,
    pub dfltgid: kgid_t,
    pub uid: kuid_t,
    pub session_lock_timeout: c_long,
}

// Used by mount API to store parsed mount options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v9fs_context {
    pub client_opts: p9_client_opts,
    pub fd_opts: p9_fd_opts,
    pub rdma_opts: p9_rdma_opts,
    pub session_opts: p9_session_opts,
}

//
// struct p9_fid - file system entity handle
// @clnt: back pointer to instantiating &p9_client
// @fid: numeric identifier for this handle
// @mode: current mode of this fid (enum?)
// @qid: the &p9_qid server identifier this handle points to
// @iounit: the server reported maximum transaction size for this file
// @uid: the numeric uid of the local user who owns this handle
// @rdir: readdir accounting structure (allocated on demand)
// @dlist: per-dentry fid tracking
//
// TODO: This needs lots of explanation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fid_source {
    FID_FROM_OTHER,
    FID_FROM_INODE,
    FID_FROM_DENTRY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_fid {
    pub clnt: *mut p9_client,
    pub fid: u32,
    pub count: refcount_t,
    pub mode: c_int,
    pub qid: p9_qid,
    pub iounit: u32,
    pub uid: kuid_t,
    pub rdir: *mut c_void,
    pub /: *mut *mut hlist_node dlist; / list of all fids attached to a dentry,
    pub ilist: hlist_node,
}

//
// struct p9_dirent - directory entry structure
// @qid: The p9 server qid for this dirent
// @d_off: offset to the next dirent
// @d_type: type of file
// @d_name: file name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_dirent {
    pub qid: p9_qid,
    pub d_off: u64,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

extern "C" {
    pub fn p9_show_client_options(m: *mut seq_file, clnt: *mut p9_client) -> c_int;
}
extern "C" {
    pub fn p9_client_statfs(fid: *mut p9_fid, sb: *mut p9_rstatfs) -> c_int;
}
extern "C" {
    pub fn p9_client_destroy(clnt: *mut p9_client);
}
extern "C" {
    pub fn p9_client_disconnect(clnt: *mut p9_client);
}
extern "C" {
    pub fn p9_client_begin_disconnect(clnt: *mut p9_client);
}
extern "C" {
    pub fn p9_client_open(fid: *mut p9_fid, mode: c_int) -> c_int;
}
extern "C" {
    pub fn p9_client_link(fid: *mut p9_fid, oldfid: *mut p9_fid, newname: *const c_char) -> c_int;
}
extern "C" {
    pub fn p9_client_clunk(fid: *mut p9_fid) -> c_int;
}
extern "C" {
    pub fn p9_client_fsync(fid: *mut p9_fid, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn p9_client_remove(fid: *mut p9_fid) -> c_int;
}
extern "C" {
    pub fn p9_client_unlinkat(dfid: *mut p9_fid, name: *const c_char, flags: c_int) -> c_int;
}
extern "C" {
    pub fn p9_client_read(fid: *mut p9_fid, offset: u64, to: *mut iov_iter, err: *mut c_int) -> c_int;
}
extern "C" {
    pub fn p9_client_write(fid: *mut p9_fid, offset: u64, from: *mut iov_iter, err: *mut c_int) -> c_int;
}
extern "C" {
    pub fn p9_client_write_subreq(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn p9_client_readdir(fid: *mut p9_fid, data: *mut c_char, count: u32, offset: u64) -> c_int;
}
extern "C" {
    pub fn p9_client_wstat(fid: *mut p9_fid, wst: *mut p9_wstat) -> c_int;
}
extern "C" {
    pub fn p9_client_setattr(fid: *mut p9_fid, attr: *mut p9_iattr_dotl) -> c_int;
}
extern "C" {
    pub fn p9_client_lock_dotl(fid: *mut p9_fid, flock: *mut p9_flock, status: *mut u8) -> c_int;
}
extern "C" {
    pub fn p9_client_getlock_dotl(fid: *mut p9_fid, fl: *mut p9_getlock) -> c_int;
}
extern "C" {
    pub fn p9_fcall_fini(fc: *mut p9_fcall);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &r->refcount) -> return;
}
extern "C" {
    pub fn p9_req_put(c: *mut p9_client, r: *mut p9_req_t) -> c_int;
}
// We cannot have the real tracepoints in header files,
// use a wrapper function
extern "C" {
    pub fn do_trace_9p_fid_get(fid: *mut p9_fid);
}
extern "C" {
    pub fn do_trace_9p_fid_put(fid: *mut p9_fid);
}
// fid reference counting helpers:
// - fids used for any length of time should always be referenced through
// p9_fid_get(), and released with p9_fid_put()
// - v9fs_fid_lookup() or similar will automatically call get for you
// and also require a put
// - the *_fid_add() helpers will stash the fid in the inode,
// at which point it is the responsibility of evict_inode()
// to call the put
// - the last put will automatically send a clunk to the server
//
extern "C" {
    pub fn p9_client_clunk(_arg: fid) -> return;
}
extern "C" {
    pub fn p9_client_cb(c: *mut p9_client, req: *mut p9_req_t, status: c_int);
}
extern "C" {
    pub fn p9stat_free(stbuf: *mut p9_wstat);
}
extern "C" {
    pub fn p9_is_proto_dotu(clnt: *mut p9_client) -> c_int;
}
extern "C" {
    pub fn p9_is_proto_dotl(clnt: *mut p9_client) -> c_int;
}
extern "C" {
    pub fn p9_client_readlink(fid: *mut p9_fid, target: *mut c_char) -> c_int;
}
extern "C" {
    pub fn p9_client_init() -> c_int;
}
extern "C" {
    pub fn p9_client_exit();
}
