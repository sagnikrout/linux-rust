//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/state.h
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
// Copyright (c) 2001 The Regents of the University of Michigan.
// All rights reserved.
//
// Kendrick Smith <kmsmith@umich.edu>
// Andy Adamson <andros@umich.edu>
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

pub const NFS4_COPYNOTIFY_STID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_referring_call {
    pub __list: list_head,
    pub rc_sequenceid: u32,
    pub rc_slotid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_referring_call_list {
    pub __list: list_head,
    pub rcl_sessionid: nfs4_sessionid,
    pub __nr_referring_calls: c_int,
    pub rcl_referring_calls: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_callback {
    pub cb_clp: *mut nfs4_client,
    pub cb_msg: rpc_message,

    pub cb_flags: c_ulong,
    pub cb_ops: *const nfsd4_callback_ops,
    pub cb_work: work_struct,
    pub cb_seq_status: c_int,
    pub cb_status: c_int,
    pub cb_held_slot: c_int,
    pub cb_nr_referring_call_list: c_int,
    pub cb_referring_call_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_callback_ops {
    pub cb): *mut *mut bool (prepare)(struct nfsd4_callback,
    pub task): *mut *mut *mut int (done)(struct nfsd4_callback cb, struct rpc_task,
    pub cb): *mut *mut void (release)(struct nfsd4_callback,
    pub opcode: u32,
}

//
// A core object that represents a "common" stateid. These are generally
// embedded within the different (more specific) stateid objects and contain
// fields that are of general use to any stateid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_stid {
    pub sc_count: refcount_t,
// A new stateid is added to the cl_stateids idr early before it
// is fully initialised.  Its sc_type is then zero.  After
// initialisation the sc_type it set under cl_lock, and then
// never changes.
//

    pub sc_type: c_ushort,
// nn->deleg_lock protects sc_status for delegation stateids.
// ->cl_lock protects sc_status for open and lock stateids.
// ->st_mutex also protect sc_status for open stateids.
// ->ls_lock protects sc_status for layout stateids.
//
// For an open stateid kept around *only* to process close replays.
// For deleg stateid, kept in idr until last reference is dropped.
//

// For a deleg stateid kept around only to process free_stateid's:

    pub sc_status: c_ushort,
    pub sc_cp_list: list_head,
    pub sc_stateid: stateid_t,
    pub sc_lock: spinlock_t,
    pub sc_client: *mut nfs4_client,
    pub sc_file: *mut nfs4_file,
    pub sc_export: *mut svc_export,
    pub ): *mut *mut void (sc_free)(struct nfs4_stid,
}

// Keep a list of stateids issued by the COPY_NOTIFY, associate it with the
// parent OPEN/LOCK/DELEG stateid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_cpntf_state {
    pub cp_stateid: copy_stateid_t,
    pub /: *mut *mut list_head cp_list; / per parent nfs4_stid,
    pub /: *mut *mut stateid_t cp_p_stateid; / copy of parent's stateid,
    pub /: *mut *mut clientid_t cp_p_clid; / copy of parent's clid,
    pub /: *mut *mut time64_t cpntf_time; / last time stateid used,
}

//
// RFC 7862 Section 4.8 states:
//
// | A copy offload stateid will be valid until either (A) the client
// | or server restarts or (B) the client returns the resource by
// | issuing an OFFLOAD_CANCEL operation or the client replies to a
// | CB_OFFLOAD operation.
//
// Because a client might not reply to a CB_OFFLOAD, or a reply
// might get lost due to connection loss, NFSD purges async copy
// state after a short period to prevent it from accumulating
// over time.
//
pub const NFSD_COPY_INITIAL_TTL: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_cb_fattr {
    pub ncf_getattr: nfsd4_callback,
    pub ncf_cb_status: u32,
// from CB_GETATTR reply
    pub ncf_cb_change: u64,
    pub ncf_cb_fsize: u64,
    pub ncf_cb_mtime: timespec64,
    pub ncf_cb_atime: timespec64,
    pub ncf_file_modified: bool,
    pub ncf_initial_cinfo: u64,
    pub ncf_cur_fsize: u64,
}

//
// FIXME: the current backchannel encoder can't handle a send buffer longer
// than a single page (see bc_malloc/bc_free).
//
pub const NOTIFY4_EVENT_QUEUE_SIZE: c_int = 3;
pub const NOTIFY4_PAGE_ARRAY_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_notify_event {
    pub refcount: refcount_t ne_ref; //,
    pub callback: *mut *mut u32 ne_mask; // FS_ mask from fsnotify,
    pub target: *mut *mut dentry ne_dentry; // dentry reference to,
    pub NULL: *mut *mut inode ne_target; // inode overwritten by rename, or,
    pub rename): u32 ne_namelen; // length of ne_name (old name for a,
    pub 0: u32 ne_newnamelen; // length of new name (rename only), else,
    pub only): char ne_name[]; // entry name, then new name (rename,
}

//
// For a rename, the new name is snapshotted at event-alloc time and stored
// immediately after the (NUL-terminated) old name in ne_name[]. ne_dentry can
// be renamed again before the CB_NOTIFY work runs, so the new name must not be
// read from the live dentry at encode time.
//
// Represents a directory delegation. The callback is for handling CB_NOTIFYs.
// As notifications from fsnotify come in, allocate a new event, take the ncn_lock,
// and add it to the ncn_evt queue. The CB_NOTIFY prepare handler will take the
// lock, clean out the list and process it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_cb_notify {
    pub count: spinlock_t ncn_lock; // protects the evt queue and,
    pub ncn_evt: int ncn_evt_cnt; // count of events in,
    pub ncn_nf: int ncn_nf_cnt; // count of valid entries in,
    pub events: *mut *mut nfsd_notify_event ncn_evt[NOTIFY4_EVENT_QUEUE_SIZE]; // list of,
    pub encoding: *mut *mut page ncn_pages[NOTIFY4_PAGE_ARRAY_SIZE]; // for,
    pub sent: *mut *mut notify4 ncn_nf; // array of notify4's to be,
    pub fail?: bool ncn_encode_err; // did encoding,
    pub callback: nfsd4_callback ncn_cb; // notify4,
}

//
// Represents a delegation stateid. The nfs4_client holds references to these
// and they are put when it is being destroyed or when the delegation is
// returned by the client:
//
// o 1 reference as long as a delegation is still in force (taken when it's
// alloc'd, put when it's returned or revoked)
//
// o 1 reference as long as a recall rpc is in progress (taken when the lease
// is broken, put when the rpc exits)
//
// o 1 more ephemeral reference for each nfsd thread currently doing something
// with that delegation without holding the cl_lock
//
// If the server attempts to recall a delegation and the client doesn't do so
// before a timeout, the server may also revoke the delegation. In that case,
// the object will either be destroyed (v4.0) or moved to a per-client list of
// revoked delegations (v4.1+).
//
// This object is a superset of the nfs4_stid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_delegation {
    pub /: *mut *mut nfs4_stid dl_stid; / must be first field,
    pub dl_perfile: list_head,
    pub dl_perclnt: list_head,
    pub /: *mut *mut list_head dl_recall_lru; / delegation recalled,
    pub dl_clnt_odstate: *mut nfs4_clnt_odstate,
    pub dl_time: time64_t,
    pub dl_type: u32,
// For recall:
    pub dl_retries: c_int,
    pub dl_recall: nfsd4_callback,
    pub dl_recalled: bool,
    pub dl_written: bool,
    pub dl_setattr: bool,
// for CB_GETATTR
    pub dl_cb_fattr: nfs4_cb_fattr,
// for CB_NOTIFY
    pub dl_cb_notify: nfsd4_cb_notify,
}

// For delegated timestamps
// For dir delegations

// client delegation callback info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_cb_conn {
// SETCLIENTID info
    pub cb_addr: sockaddr_storage,
    pub cb_saddr: sockaddr_storage,
    pub cb_addrlen: usize,
    pub case: *mut *mut u32 cb_prog; / used only in 4.0,
    pub /: *mut *mut u32 cb_ident; / minorversion 0 only,
    pub /: *mut *mut *mut svc_xprt cb_xprt; / minorversion 1 only,
}

extern "C" {
    pub fn container_of(_arg: s, nfs4_delegation: struct, _arg: dl_stid) -> return;
}
// Maximum number of slots per session.  This is for sanity-check only.
// It could be increased if we had a mechanism to shutdown misbehaving clients.
// A large number can be needed to get good throughput on high-latency servers.
//
pub const NFSD_MAX_SLOTS_PER_SESSION: c_int = 2048;
// Maximum  session per slot cache size
pub const NFSD_SLOT_CACHE_SIZE: c_int = 2048;
// Maximum number of NFSD_SLOT_CACHE_SIZE slots per session
pub const NFSD_CACHE_SIZE_SLOTS_PER_SESSION: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_slot {
    pub sl_seqid: u32,
    pub sl_status: __be32,
    pub sl_cred: svc_cred,
    pub sl_index: u32,
    pub sl_datalen: u32,
    pub sl_opcnt: u16,
    pub sl_generation: u16,

    pub sl_flags: u8,
    pub sl_data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_channel_attrs {
    pub headerpadsz: u32,
    pub maxreq_sz: u32,
    pub maxresp_sz: u32,
    pub maxresp_cached: u32,
    pub maxops: u32,
    pub maxreqs: u32,
    pub nr_rdma_attrs: u32,
    pub rdma_attrs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_cb_sec {
    pub /: *mut *mut u32 flavor; / (u32)(-1) used to mean "no valid flavor",
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_create_session {
    pub clientid: clientid_t,
    pub sessionid: nfs4_sessionid,
    pub seqid: u32,
    pub flags: u32,
    pub fore_channel: nfsd4_channel_attrs,
    pub back_channel: nfsd4_channel_attrs,
    pub callback_prog: u32,
    pub cb_sec: nfsd4_cb_sec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_backchannel_ctl {
    pub bc_cb_program: u32,
    pub bc_cb_sec: nfsd4_cb_sec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_bind_conn_to_session {
    pub sessionid: nfs4_sessionid,
    pub dir: u32,
}

// The single slot clientid cache structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_clid_slot {
    pub sl_seqid: u32,
    pub sl_status: __be32,
    pub sl_cr_ses: nfsd4_create_session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_conn {
    pub cn_persession: list_head,
    pub cn_xprt: *mut svc_xprt,
    pub cn_xpt_user: svc_xpt_user,
    pub cn_session: *mut nfsd4_session,
// CDFC4_FORE, CDFC4_BACK:
    pub cn_flags: c_uchar,
}

// Maximum number of slots that nfsd will use in the backchannel

//
// Representation of a v4.1+ session. These are refcounted in a similar fashion
// to the nfs4_client. References are only taken when the server is actively
// working on the object (primarily during the processing of compounds).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_session {
    pub se_ref: core::sync::atomic::AtomicI32,
    pub se_lock: spinlock_t,
    pub /: *mut *mut u32 se_cb_slot_avail; / bitmap of available slots,
    pub /: *mut *mut u32 se_cb_highest_slot; / highest slot client wants,
    pub se_cb_prog: u32,
    pub /: *mut *mut list_head se_hash; / hash by sessionid,
    pub se_perclnt: list_head,
    pub /: *mut *mut list_head se_all_sessions;/ global list of sessions,
    pub se_client: *mut nfs4_client,
    pub se_sessionid: nfs4_sessionid,
    pub se_fchannel: nfsd4_channel_attrs,
    pub se_cb_sec: nfsd4_cb_sec,
    pub se_conns: list_head,
    pub se_cb_seq_nr: [u32; NFSD_BC_SLOT_TABLE_SIZE],
    pub /: *mut *mut xarray se_slots; / forward channel slots,
    pub se_slot_gen: u16,
    pub se_dead: bool,
    pub se_target_maxslots: u32,
    pub rcu_head: rcu_head,
}

// formatted contents of nfs4_sessionid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_sessionid {
    pub clientid: clientid_t,
    pub sequence: u32,
    pub reserved: u32,
}

// Length of MD5 digest as hex, plus terminating '\0'

//
// State                Meaning                  Where set
// --------------------------------------------------------------------------
// | NFSD4_ACTIVE      | Confirmed, active    | Default                     |
// |------------------- ----------------------------------------------------|
// | NFSD4_COURTESY    | Courtesy state.      | nfs4_get_client_reaplist    |
// |                   | Lease/lock/share     |                             |
// |                   | reservation conflict |                             |
// |                   | can cause Courtesy   |                             |
// |                   | client to be expired |                             |
// |------------------------------------------------------------------------|
// | NFSD4_EXPIRABLE   | Courtesy client to be| nfs4_laundromat             |
// |                   | expired by Laundromat| try_to_expire_client        |
// |                   | due to conflict      |                             |
// |------------------------------------------------------------------------|
//
// struct nfs4_client - one per client.  Clientids live here.
//
// The initial object created by an NFS client using SETCLIENTID (for NFSv4.0)
// or EXCHANGE_ID (for NFSv4.1+). These objects are refcounted and timestamped.
// Each nfsd_net_ns object contains a set of these and they are tracked via
// short and long form clientid. They are hashed and searched for under the
// per-nfsd_net client_lock spinlock.
//
// References to it are only held during the processing of compounds, and in
// certain other operations. In their "resting state" they have a refcount of
// 0. If they are not renewed within a lease period, they become eligible for
// destruction by the laundromat.
//
// These objects can also be destroyed if the client sends certain forms of
// SETCLIENTID or EXCHANGE_ID operations.
//
// Care is taken *not* to do this however when the objects have an elevated
// refcount.
//
// o Each nfs4_client is hashed by clientid
//
// o Each nfs4_clients is also hashed by name (the opaque quantity initially
// sent by the client to identify itself).
//
// o cl_perclient list is used to ensure no dangling stateowner references
// when we expire the nfs4_client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_client {
    pub /: *mut *mut list_head cl_idhash; / hash by cl_clientid.id,
    pub /: *mut *mut rb_node cl_namenode; / link into by-name trees,
    pub cl_ownerstr_hashtbl: *mut list_head,
    pub cl_openowners: list_head,
    pub /: *mut *mut idr cl_stateids; / stateid lookup,
    pub cl_delegations: list_head,
    pub /: *mut *mut list_head cl_revoked; / unacknowledged, revoked 4.1 state,
    pub /: *mut *mut list_head cl_lru; / tail queue,

    pub /: *mut *mut list_head cl_lo_states; / outstanding layout states,
    pub cl_fence_retry_warn: bool,

    pub /: *mut *mut xdr_netobj cl_name; / id generated by client,
    pub /: *mut *mut nfs4_verifier cl_verifier; / generated by client,
    pub /: *mut *mut time64_t cl_time; / time of last lease renewal,
    pub /: *mut *mut sockaddr_storage cl_addr; / client ipaddress,
    pub /: *mut *mut bool cl_mach_cred; / SP4_MACH_CRED in force,
    pub /: *mut *mut svc_cred cl_cred; / setclientid principal,
    pub /: *mut *mut clientid_t cl_clientid; / generated by server,
    pub /: *mut *mut nfs4_verifier cl_confirm; / generated by server,
    pub cl_minorversion: u32,
    pub /: *mut *mut atomic_t cl_admin_revoked; / count of admin-revoked states,
// NFSv4.1 client implementation id:
    pub cl_nii_domain: xdr_netobj,
    pub cl_nii_name: xdr_netobj,
    pub cl_nii_time: timespec64,
// for v4.0 and v4.1 callbacks:
    pub cl_cb_conn: nfs4_cb_conn,

    pub cl_flags: c_ulong,
    pub cl_callback_wq: *mut workqueue_struct,
    pub cl_cb_cred: *const cred,
    pub cl_cb_client: *mut rpc_clnt,
    pub cl_cb_ident: u32,
pub const NFSD4_CB_UP: c_int = 0;
pub const NFSD4_CB_UNKNOWN: c_int = 1;
pub const NFSD4_CB_DOWN: c_int = 2;
pub const NFSD4_CB_FAULT: c_int = 3;
    pub cl_cb_state: c_int,
    pub cl_cb_null: nfsd4_callback,
    pub cl_cb_session: *mut nfsd4_session __rcu,
// for all client information that callback code might need:
    pub cl_lock: spinlock_t,
// for nfs41
    pub cl_sessions: list_head,
    pub /: *mut *mut nfsd4_clid_slot cl_cs_slot; / create_session slot,
    pub cl_exchange_flags: u32,
// number of rpc's in progress over an associated session:
    pub cl_rpc_users: core::sync::atomic::AtomicI32,
    pub cl_nfsdfs: nfsdfs_client,
    pub cl_spo_must_allow: nfs4_op_map,
// debugging info directory under nfsd/clients/ :
    pub cl_nfsd_dentry: *mut dentry,
// 'info' file within that directory. Ref is not counted,
// but will remain valid iff cl_nfsd_dentry != NULL
//
    pub cl_nfsd_info_dentry: *mut dentry,
    pub /: *mut *mut rpc_wait_queue cl_cb_waitq; / backchannel callers may,
// wait here for slots
    pub net: *mut net,
    pub /: *mut *mut list_head async_copies; / list of async copies,
    pub /: *mut *mut spinlock_t async_lock; / lock for async copies,
    pub /: *mut *mut atomic_t cl_cb_inflight; / Outstanding callbacks,
    pub cl_state: c_uint,
    pub cl_delegs_in_recall: core::sync::atomic::AtomicI32,
    pub cl_ra: *mut nfsd4_cb_recall_any,
    pub cl_ra_time: time64_t,

    pub cl_dev_fences: xarray,
    pub cl_fence_mutex: mutex,

}

// struct nfs4_client_reset
// one per old client. Populates reset_str_hashtbl. Filled from conf_id_hashtbl
// upon lease reset, or from upcall to state_daemon (to read in state
// from non-volitile storage) upon reboot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_client_reclaim {
    pub /: *mut *mut list_head cr_strhash; / hash by cr_name,
    pub /: *mut *mut *mut nfs4_client cr_clp; / pointer to associated clp,
    pub /: *mut *mut xdr_netobj cr_name; / recovery dir name,
    pub cr_princhash: xdr_netobj,
}

//
// REPLAY_ISIZE is sized for an OPEN response with delegation:
// 4(status) + 8(stateid) + 20(changeinfo) + 4(rflags) +
// 8(verifier) + 4(deleg. type) + 8(deleg. stateid) +
// 4(deleg. recall flag) + 20(deleg. space limit) +
// ~32(deleg. ace) = 112 bytes
//
// Some responses can exceed this. A LOCK denial includes the conflicting
// lock owner, which can be up to 1024 bytes (NFS4_OPAQUE_LIMIT). When a
// response exceeds REPLAY_ISIZE, a buffer is dynamically allocated. If
// that allocation fails, only rp_status is saved. Enlarging this constant
// increases the size of every nfs4_stateowner.
//
pub const NFSD4_REPLAY_ISIZE: c_int = 112;
//
// Replay buffer, where the result of the last seqid-mutating operation
// is cached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_replay {
    pub rp_status: __be32,
    pub rp_buflen: c_uint,
    pub /: *mut *mut *mut char rp_buf; / rp_ibuf or kmalloc'd,
    pub rp_openfh: knfsd_fh,
    pub rp_locked: c_int,
    pub rp_ibuf: [c_char; NFSD4_REPLAY_ISIZE],
}

extern "C" {
    pub fn nfs4_replay_free_cache(rp: *mut nfs4_replay);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_stateowner_operations {
    pub ): *mut *mut void (so_unhash)(struct nfs4_stateowner,
    pub ): *mut *mut void (so_free)(struct nfs4_stateowner,
}

//
// A core object that represents either an open or lock owner. The object and
// lock owner objects have one of these embedded within them. Refcounts and
// other fields common to both owner types are contained within these
// structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_stateowner {
    pub so_strhash: list_head,
    pub so_stateids: list_head,
    pub so_client: *mut nfs4_client,
    pub so_ops: *const nfs4_stateowner_operations,
// after increment in nfsd4_bump_seqid, represents the next
// sequence id expected from the client:
    pub so_count: core::sync::atomic::AtomicI32,
    pub so_seqid: u32,
    pub /: *mut *mut xdr_netobj so_owner; / open owner name,
    pub so_replay: nfs4_replay,
    pub so_is_open_owner: bool,
}

//
// When a file is opened, the client provides an open state owner opaque string
// that indicates the "owner" of that open. These objects are refcounted.
// References to it are held by each open state associated with it. This object
// is a superset of the nfs4_stateowner struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_openowner {
    pub /: *mut *mut nfs4_stateowner oo_owner; / must be first field,
    pub oo_perclient: list_head,
//
// We keep around openowners a little while after last close,
// which saves clients from having to confirm, and allows us to
// handle close replays if they come soon enough.  The close_lru
// is a list of such openowners, to be reaped by the laundromat
// thread eventually if they remain unused:
//
    pub oo_close_lru: list_head,
    pub oo_last_closed_stid: *mut nfs4_ol_stateid,
    pub /: *mut *mut time64_t oo_time; / time of placement on so_close_lru,
pub const NFS4_OO_CONFIRMED: c_int = 1;
    pub oo_flags: c_uchar,
}

//
// Represents a generic "lockowner". Similar to an openowner. References to it
// are held by the lock stateids that are created on its behalf. This object is
// a superset of the nfs4_stateowner struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_lockowner {
    pub /: *mut *mut nfs4_stateowner lo_owner; / must be first element,
    pub /: *mut *mut list_head lo_blocked; / blocked file_locks,
}

extern "C" {
    pub fn container_of(_arg: so, nfs4_openowner: struct, _arg: oo_owner) -> return;
}
extern "C" {
    pub fn container_of(_arg: so, nfs4_lockowner: struct, _arg: lo_owner) -> return;
}
//
// Per-client state indicating no. of opens and outstanding delegations
// on a file from a particular client.'od' stands for 'open & delegation'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_clnt_odstate {
    pub co_client: *mut nfs4_client,
    pub co_file: *mut nfs4_file,
    pub co_perfile: list_head,
    pub co_odcount: refcount_t,
}

//
// nfs4_file: a file opened by some number of (open) nfs4_stateowners.
//
// These objects are global. nfsd keeps one instance of a nfs4_file per
// filehandle (though it may keep multiple file descriptors for each). Each
// inode can have multiple filehandles associated with it, so there is
// (potentially) a many to one relationship between this struct and struct
// inode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_file {
    pub fi_ref: refcount_t,
    pub fi_inode: *mut *mut inode,
    pub fi_aliased: bool,
    pub fi_lock: spinlock_t,
    pub fi_rlist: rhlist_head,
    pub fi_stateids: list_head,
    pub fi_delegations: list_head,
    pub fi_rcu: rcu_head,
}

// One each for O_RDONLY, O_WRONLY, O_RDWR:
//
// Each open or lock stateid contributes 0-4 to the counts
// below depending on which bits are set in st_access_bitmap:
// 1 to fi_access[O_RDONLY] if NFS4_SHARE_ACCES_READ is set
// + 1 to fi_access[O_WRONLY] if NFS4_SHARE_ACCESS_WRITE is set
// + 1 to both of the above if NFS4_SHARE_ACCESS_BOTH is set.
//

//
// A generic struct representing either a open or lock stateid. The nfs4_client
// holds a reference to each of these objects, and they in turn hold a
// reference to their respective stateowners. The client's reference is
// released in response to a close or unlock (depending on whether it's an open
// or lock stateid) or when the client is being destroyed.
//
// In the case of v4.0 open stateids, these objects are preserved for a little
// while after close in order to handle CLOSE replays. Those are eventually
// reclaimed via a LRU scheme by the laundromat.
//
// This object is a superset of the nfs4_stid. "ol" stands for "Open or Lock".
// Better suggestions welcome.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ol_stateid {
    pub st_stid: nfs4_stid,
    pub st_perfile: list_head,
    pub st_perstateowner: list_head,
    pub st_locks: list_head,
    pub st_stateowner: *mut nfs4_stateowner,
    pub st_clnt_odstate: *mut nfs4_clnt_odstate,
//
// These bitmasks use 3 separate bits for READ, ALLOW, and BOTH; see the
// comment above bmap_to_share_mode() for explanation:
//
    pub st_access_bmap: c_uchar,
    pub st_deny_bmap: c_uchar,
    pub st_openstp: *mut nfs4_ol_stateid,
    pub st_mutex: mutex,
}

extern "C" {
    pub fn container_of(_arg: s, nfs4_ol_stateid: struct, _arg: st_stid) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_layout_stateid {
    pub ls_stid: nfs4_stid,
    pub ls_perclnt: list_head,
    pub ls_perfile: list_head,
    pub ls_lock: spinlock_t,
    pub ls_layouts: list_head,
    pub ls_layout_type: u32,
    pub ls_file: *mut nfsd_file,
    pub ls_recall: nfsd4_callback,
    pub ls_recall_sid: stateid_t,
    pub ls_recalled: bool,
    pub ls_mutex: mutex,
    pub ls_fence_work: delayed_work,
    pub ls_fence_delay: c_uint,
    pub ls_fenced: bool,
    pub ls_fence_inflight: bool,
}

extern "C" {
    pub fn container_of(_arg: s, nfs4_layout_stateid: struct, _arg: ls_stid) -> return;
}
// flags for preprocess_seqid_op()
pub const RD_STATE: c_uint = 0x00000010;
pub const WR_STATE: c_uint = 0x00000020;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd4_cb_op {
    NFSPROC4_CLNT_CB_NULL = 0,
    NFSPROC4_CLNT_CB_RECALL,
    NFSPROC4_CLNT_CB_LAYOUT,
    NFSPROC4_CLNT_CB_OFFLOAD,
    NFSPROC4_CLNT_CB_SEQUENCE,
    NFSPROC4_CLNT_CB_NOTIFY_LOCK,
    NFSPROC4_CLNT_CB_RECALL_ANY,
    NFSPROC4_CLNT_CB_GETATTR,
    NFSPROC4_CLNT_CB_NOTIFY,
}

// Returns true iff a is later than b:
//
// When a client tries to get a lock on a file, we set one of these objects
// on the blocking lock. When the lock becomes free, we can then issue a
// CB_NOTIFY_LOCK to the server.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_blocked_lock {
    pub nbl_list: list_head,
    pub nbl_lru: list_head,
    pub nbl_time: time64_t,
    pub nbl_lock: file_lock,
    pub nbl_fh: knfsd_fh,
    pub nbl_cb: nfsd4_callback,
    pub nbl_kref: kref,
}

extern "C" {
    pub fn nfs4_put_stid(s: *mut nfs4_stid);
}
extern "C" {
    pub fn nfs4_inc_and_copy_stateid(dst: *mut stateid_t, stid: *mut nfs4_stid);
}
extern "C" {
    pub fn nfs4_remove_reclaim_record(: *mut nfs4_client_reclaim, : *mut nfsd_net);
}
extern "C" {
    pub fn nfs4_release_reclaim(: *mut nfsd_net);
}
extern "C" {
    pub fn nfs4_check_open_reclaim(: *mut nfs4_client) -> __be32;
}
extern "C" {
    pub fn nfsd4_probe_callback(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_probe_callback_sync(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_change_callback(clp: *mut nfs4_client, : *mut nfs4_cb_conn);
}
extern "C" {
    pub fn nfsd41_cb_destroy_referring_call_list(cb: *mut nfsd4_callback);
}
extern "C" {
    pub fn nfsd4_run_cb(cb: *mut nfsd4_callback) -> bool;
}
extern "C" {
    pub fn nfsd4_shutdown_callback(: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_shutdown_copy(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_put_client(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_async_copy_reaper(nn: *mut nfsd_net);
}
extern "C" {
    pub fn nfsd4_has_active_async_copies(clp: *mut nfs4_client) -> bool;
}
extern "C" {
    pub fn nfsd_update_cmtime_attr(f: *mut file, flags: c_uint);
}
extern "C" {
    pub fn nfs4_has_reclaimed_state(name: xdr_netobj, nn: *mut nfsd_net) -> bool;
}
extern "C" {
    pub fn put_nfs4_file(fi: *mut nfs4_file);
}

extern "C" {
    pub fn nfsd4_revoke_states(nn: *mut nfsd_net, sb: *mut super_block);
}
extern "C" {
    pub fn nfsd4_revoke_export_states(nn: *mut nfsd_net, path: *const path);
}
extern "C" {
    pub fn nfsd4_cancel_copy_by_sb(net: *mut net, sb: *mut super_block);
}
extern "C" {
    pub fn nfsd_net_cb_init(nn: *mut nfsd_net) -> c_int;
}
extern "C" {
    pub fn nfsd_net_cb_shutdown(nn: *mut nfsd_net);
}

// grace period management
extern "C" {
    pub fn nfsd4_force_end_grace(nn: *mut nfsd_net) -> bool;
}
// nfs4recover operations
extern "C" {
    pub fn nfsd4_client_tracking_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nfsd4_client_tracking_exit(net: *mut net);
}
extern "C" {
    pub fn nfsd4_client_record_create(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_client_record_remove(clp: *mut nfs4_client);
}
extern "C" {
    pub fn nfsd4_client_record_check(clp: *mut nfs4_client) -> c_int;
}
extern "C" {
    pub fn nfsd4_record_grace_done(nn: *mut nfsd_net);
}
