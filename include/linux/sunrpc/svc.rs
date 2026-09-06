//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svc.h
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
// linux/include/linux/sunrpc/svc.h
//
// RPC server declarations.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

//
// RPC service thread pool.
//
// Pool of threads and temporary sockets.  Generally there is only
// a single one of these per RPC service, but on NUMA machines those
// services that can benefit from it (i.e. nfs but not lockd) will
// have one pool per NUMA node.  This optimisation reduces cross-
// node traffic on multi-node NUMA NFS servers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_pool {
    pub /: *mut *mut unsigned int sp_id; / pool id; also node id on NUMA,
    pub /: *mut *mut unsigned int sp_nrthreads; / # of threads currently running in pool,
    pub /: *mut *mut unsigned int sp_nrthrmin; / Min number of threads to run per pool,
    pub /: *mut *mut unsigned int sp_nrthrmax; / Max requested number of threads in pool,
    pub /: *mut *mut lwq sp_xprts; / pending transports,
    pub /: *mut *mut list_head sp_all_threads; / all server threads,
    pub /: *mut *mut llist_head sp_idle_threads; / idle server threads,
// statistics on pool operation
    pub sp_messages_arrived: percpu_counter,
    pub sp_sockets_queued: percpu_counter,
    pub sp_threads_woken: percpu_counter,
    pub sp_flags: c_ulong,
    pub ____cacheline_aligned_in_smp: },
// bits for sp_flags
}

//
// RPC service.
//
// An RPC service is a ``daemon,'' possibly multithreaded, which
// receives and processes incoming RPC messages.
// It has one or more transport sockets associated with it, and maintains
// a list of idle threads waiting for input.
//
// We currently do not support more than one RPC program per daemon.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_serv {
    pub /: *mut *mut *mut svc_program  sv_programs; / RPC programs,
    pub /: *mut *mut *mut svc_stat  sv_stats; / RPC statistics,
    pub sv_lock: spinlock_t,
    pub /: *mut *mut unsigned int sv_nprogs; / Number of sv_programs,
    pub /: *mut *mut unsigned int sv_nrthreads; / # of running server threads,
    pub /: *mut *mut unsigned int sv_max_payload; / datagram payload size,
    pub /: *mut *mut unsigned int sv_max_mesg; / max_payload + 1 page for overheads,
    pub /: *mut *mut unsigned int sv_xdrsize; / XDR buffer size,
    pub /: *mut *mut list_head sv_permsocks; / all permanent sockets,
    pub /: *mut *mut list_head sv_tempsocks; / all temporary sockets,
    pub /: *mut *mut int sv_tmpcnt; / count of temporary "valid" sockets,
    pub /: *mut *mut timer_list sv_temptimer; / timer for aging temporary sockets,
    pub /: *mut *mut *mut char  sv_name; / service name,
    pub /: *mut *mut bool sv_is_pooled; / is this a pooled service?,
    pub /: *mut *mut *mut svc_pool  sv_pools; / array of thread pools,
    pub data): *mut *mut int (sv_threadfn)(void,

    pub requests: *mut *mut lwq sv_cb_list; / queue for callback,
// that arrive over the same
// connection
    pub /: *mut *mut bool sv_bc_enabled; / service uses backchannel,

}

// This is used by pool_stats to find and lock an svc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_info {
    pub serv: *mut svc_serv,
    pub mutex: *mut mutex,
}

extern "C" {
    pub fn svc_destroy(svcp: *mut svc_serv);
}
//
// Maximum payload size supported by a kernel RPC server.
// This is use to determine the max number of pages nfsd is
// willing to return in a single READ operation.
//
// These happen to all be powers of 2, which is not strictly
// necessary but helps enforce the real limitation, which is
// that they should be multiples of PAGE_SIZE.
//
// For UDP transports, a block plus NFS,RPC, and UDP headers
// has to fit into the IP datagram limit of 64K.  The largest
// feasible number for all known page sizes is probably 48K,
// but we choose 32K here.  This is the same as the historical
// Linux limit; someone who cares more about NFS/UDP performance
// can test a larger number.
//
// For non-UDP transports we have more freedom.  A size of 4MB is
// chosen to accommodate clients that support larger I/O sizes.
//
extern "C" {
    pub fn svc_max_payload(rqstp: *const svc_rqst) -> u32;
}
//
// RPC Call and Reply messages each have their own page array.
// rq_pages holds the incoming Call message; rq_respages holds
// the outgoing Reply message. Both arrays are sized to
// svc_serv_maxpages() entries and are allocated dynamically.
//
// Pages are sent using ->sendmsg with MSG_SPLICE_PAGES so each
// server thread needs to allocate more to replace those used in
// sending.
//
// rq_pages request page contract:
//
// Transport receive paths that move request data pages out of
// rq_pages -- TCP multi-fragment reassembly (svc_tcp_save_pages)
// and RDMA Read I/O (svc_rdma_clear_rqst_pages) -- NULL those
// entries to prevent svc_rqst_release_pages() from freeing pages
// still in transport use, and set rq_pages_nfree to the count.
// svc_alloc_arg() refills only that many rq_pages entries.
//
// For rq_respages, svc_rqst_release_pages() NULLs entries in
// [rq_respages, rq_next_page) after each RPC. svc_alloc_arg()
// refills only that range.
//
// xdr_buf holds responses; the structure fits NFS read responses
// (header, data pages, optional tail) and enables sharing of
// client-side routines.
//
// The xdr_buf.head kvec always points to the first page in the
// rq_*pages list. The xdr_buf.pages pointer points to the second
// page on that list. xdr_buf.tail points to the end of the first
// page. This assumes that the non-page part of an rpc reply will
// fit in a page - NFSd ensures this. lockd also has no trouble.
//
// svc_serv_maxpages - maximum count of pages needed for one RPC message
// @serv: RPC service context
//
// Returns a count of pages or vectors that can hold the maximum
// size RPC message for @serv.
//
// Each page array can hold at most one payload plus two
// overhead pages (one for the RPC header, one for tail data).
// nfsd_splice_actor() might need an extra page when a READ
// payload is not page-aligned.
//
// The context of a single thread, including the request currently being
// processed.
//
// RPC programs are free to use rq_private to stash thread-local information.
// The sunrpc layer will not access it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rqst {
    pub /: *mut *mut list_head rq_all; / all threads list,
    pub /: *mut *mut llist_node rq_idle; / On the idle list,
    pub /: *mut *mut rcu_head rq_rcu_head; / for RCU deferred kfree,
    pub /: *mut *mut *mut svc_xprt  rq_xprt; / transport ptr,
    pub /: *mut *mut sockaddr_storage rq_addr; / peer address,
    pub rq_addrlen: usize,
    pub request: *mut *mut sockaddr_storage rq_daddr; / dest addr of,
// - reply from here
    pub rq_daddrlen: usize,
    pub /: *mut *mut *mut svc_serv  rq_server; / RPC service definition,
    pub /: *mut *mut *mut svc_pool  rq_pool; / thread pool,
    pub /: *const *const *const svc_procedure rq_procinfo;/ procedure info,
    pub /: *mut *mut *mut auth_ops  rq_authop; / authentication flavour,
    pub /: *mut *mut svc_cred rq_cred; / auth info,
    pub /: *mut *mut *mut void  rq_xprt_ctxt; / transport specific context ptr,
    pub /: *mut *mut *mut svc_deferred_reqrq_deferred; / deferred request we are replaying,
    pub rq_arg: xdr_buf,
    pub rq_arg_stream: xdr_stream,
    pub rq_res_stream: xdr_stream,
    pub rq_scratch_folio: *mut folio,
    pub rq_res: xdr_buf,
    pub /: *mut *mut unsigned long rq_maxpages; / entries per page array,
    pub /: *mut *mut unsigned long rq_pages_nfree; / rq_pages entries NULLed by transport,
    pub /: *mut *mut *mut *mut page  rq_pages; / Call buffer pages,
    pub /: *mut *mut *mut *mut page  rq_respages; / Reply buffer pages,
    pub /: *mut *mut *mut *mut page  rq_next_page; / next reply page to use,
    pub /: *mut *mut *mut *mut page  rq_page_end; / one past the last reply page,
    pub rq_fbatch: folio_batch,
    pub rq_bvec: *mut bio_vec,
    pub /: *mut *mut __be32 rq_xid; / transmission id,
    pub /: *mut *mut u32 rq_prog; / program number,
    pub /: *mut *mut u32 rq_vers; / program version,
    pub /: *mut *mut u32 rq_proc; / procedure number,
    pub /: *mut *mut u32 rq_prot; / IP protocol,
    pub /: *mut *mut unsigned long rq_flags; / flags field,
    pub /: *mut *mut ktime_t rq_qtime; / enqueue time,
    pub /: *mut *mut *mut void  rq_argp; / decoded arguments,
    pub /: *mut *mut *mut void  rq_resp; / xdr'd results,
    pub rq_accept_statp: *mut __be32,
    pub /: *mut *mut *mut void  rq_auth_data; / flavor-specific data,
    pub /: *mut *mut __be32 rq_auth_stat; / authentication status,
    pub code: *mut *mut int rq_auth_slack; / extra space xdr,
// should leave in head
// for krb5i, krb5p.
//
    pub outq: *mut *mut int rq_reserved; / space on socket,
// reserved for this request
//
    pub /: *mut *mut ktime_t rq_stime; / start time,
    pub for: *mut *mut cache_req rq_chandle; / handle passed to caches,
// request delaying
//
// Catering to nfsd
    pub /: *mut *mut *mut auth_domain  rq_client; / RPC peer info,
    pub /: *mut *mut *mut auth_domain  rq_gssclient; / "gss/"-style peer info,
    pub /: *mut *mut *mut task_rq_task; / service thread,
    pub backchannel's: *mut *mut *mut net rq_bc_net; / pointer to,
// net namespace
//
    pub inidicate: *mut *mut int rq_err; / Thread sets this to,
// initialisation success.
//
    pub bc_to_initval: c_ulong,
    pub bc_to_retries: c_uint,
    pub /: *mut *mut unsigned int rq_status_counter; / RPC processing counter,
    pub /: *mut *mut *mut void rq_private; / For use by the service thread,
}

// bits for rq_flags

//
// Rigorous type checking on sockaddr type conversions
//
// svc_thread_should_stop - check if this thread should stop
// @rqstp: the thread that might need to stop
//
// To stop an svc thread, the pool flags SP_NEED_VICTIM and SP_VICTIM_REMAINS
// are set.  The first thread which sees SP_NEED_VICTIM clears it, becoming
// the victim using this function.  It should then promptly call
// svc_exit_thread() to complete the process, clearing SP_VICTIM_REMAINS
// so the task waiting for a thread to exit can wake and continue.
//
// Return values:
// %true: caller should invoke svc_exit_thread()
// %false: caller should do nothing
//
extern "C" {
    pub fn test_bit(_arg: RQ_VICTIM, _arg: &rqstp->rq_flags) -> return;
}
//
// svc_thread_init_status - report whether thread has initialised successfully
// @rqstp: the thread in question
// @err: errno code
//
// After performing any initialisation that could fail, and before starting
// normal work, each sunrpc svc_thread must call svc_thread_init_status()
// with an appropriate error, or zero.
//
// If zero is passed, the thread is ready and must continue until
// svc_thread_should_stop() returns true.  If a non-zero error is passed
// the call will not return - the thread will exit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_deferred_req {
    pub /: *mut *mut u32 prot; / protocol (UDP or TCP),
    pub xprt: *mut svc_xprt,
    pub /: *mut *mut sockaddr_storage addr; / where reply must go,
    pub addrlen: usize,
    pub /: *mut *mut sockaddr_storage daddr; / where reply must come from,
    pub daddrlen: usize,
    pub xprt_ctxt: *mut c_void,
    pub handle: cache_deferred_req,
    pub argslen: c_int,
    pub args: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_process_info {
    pub rqstp): *mut *mut int (dispatch)(struct svc_rqst,
    pub lovers: c_uint,
    pub hivers: c_uint,
    pub mismatch: },
}

//
// RPC program - an array of these can use the same transport endpoint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_program {
    pub /: *mut *mut u32 pg_prog; / program number,
    pub /: *mut *mut unsigned int pg_lovers; / lowest version,
    pub /: *mut *mut unsigned int pg_hivers; / highest version,
    pub /: *mut *mut unsigned int pg_nvers; / number of versions,
    pub /: *const *const *const *const svc_version pg_vers; / version array,
    pub /: *mut *mut *mut char  pg_name; / service name,
    pub /: *mut *mut *mut char  pg_class; / class name: services sharing authentication,
    pub rqstp): *mut *mut svc_auth_status (pg_authenticate)(struct svc_rqst,
    pub ): *mut svc_process_info,
    pub port): c_ushort,
}

//
// RPC program version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_version {
    pub /: *mut *mut u32 vs_vers; / version number,
    pub /: *mut *mut u32 vs_nproc; / number of procedures,
    pub /: *const *const *const svc_procedure vs_proc; / per-procedure info,
    pub /: *mut *mut u32 vs_xdrsize; / xdrsize needed for this version,
// Don't register with rpcbind
    pub vs_hidden: bool,
// Don't care if the rpcbind registration fails
    pub vs_rpcb_optnl: bool,
// Need xprt with congestion control
    pub vs_need_cong_ctrl: bool,
// Dispatch function
    pub rqstp): *mut *mut int (vs_dispatch)(struct svc_rqst,
}

//
// RPC procedure info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_procedure {
// process the request:
    pub ): *mut *mut __be32 (pc_func)(struct svc_rqst,
// XDR decode args:
    pub xdr): *mut xdr_stream,
// XDR encode result:
    pub xdr): *mut xdr_stream,
// XDR free result:
    pub ): *mut *mut void (pc_release)(struct svc_rqst,
    pub /: *mut *mut unsigned int pc_argsize; / argument struct size,
    pub /: *mut *mut unsigned int pc_argzero; / how much of argument to clear,
    pub /: *mut *mut unsigned int pc_ressize; / result struct size,
    pub /: *mut *mut unsigned int pc_cachetype; / cache info (NFS),
    pub /: *mut *mut unsigned int pc_xdrressize; / maximum size of XDR reply,
    pub /: *const *const *const char  pc_name; / for display,
}

//
// Function prototypes.
//
extern "C" {
    pub fn sunrpc_set_pool_mode(val: *const c_char) -> c_int;
}
extern "C" {
    pub fn sunrpc_get_pool_mode(val: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn svc_rpcb_cleanup(serv: *mut svc_serv, net: *mut net);
}
extern "C" {
    pub fn svc_bind(serv: *mut svc_serv, net: *mut net) -> c_int;
}
extern "C" {
    pub fn svc_rqst_release_pages(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn svc_new_thread(serv: *mut svc_serv, pool: *mut svc_pool) -> c_int;
}
extern "C" {
    pub fn svc_exit_thread(: *mut svc_rqst);
}
extern "C" {
    pub fn svc_serv_maxthreads(serv: *const svc_serv) -> c_uint;
}
extern "C" {
    pub fn svc_pool_stats_open(si: *mut svc_info, file: *mut file) -> c_int;
}
extern "C" {
    pub fn svc_process(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn svc_process_bc(req: *mut rpc_rqst, rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn svc_wake_up(: *mut svc_serv);
}
extern "C" {
    pub fn svc_reserve(rqstp: *mut svc_rqst, space: c_int);
}
extern "C" {
    pub fn svc_pool_wake_idle_thread(pool: *mut svc_pool);
}
extern "C" {
    pub fn svc_serv_nrpools(serv: *const svc_serv) -> c_uint;
}
extern "C" {
    pub fn svc_print_addr(: *mut svc_rqst, : *mut c_char, _arg: usize) -> *mut c_char;
}
extern "C" {
    pub fn svc_proc_name(rqstp: *const svc_rqst) -> *const c_char;
}

//
// svc_rqst_page_release - release a page associated with an RPC transaction
// @rqstp: RPC transaction context
// @page: page to release
//
// Released pages are batched and freed together, reducing
// allocator pressure under heavy RPC workloads.
//
// When we want to reduce the size of the reserved space in the response
// buffer, we need to take into account the size of any checksum data that
// may be at the end of the packet. This is difficult to determine exactly
// for all cases without actually generating the checksum, so we just use a
// static value.
//
// svcxdr_init_decode - Prepare an xdr_stream for Call decoding
// @rqstp: controlling server RPC transaction context
//
// svcxdr_init_encode - Prepare an xdr_stream for svc Reply encoding
// @rqstp: controlling server RPC transaction context
//
// svcxdr_encode_opaque_pages - Insert pages into an xdr_stream
// @xdr: xdr_stream to be updated
// @pages: array of pages to insert
// @base: starting offset of first data byte in @pages
// @len: number of data bytes in @pages to insert
//
// After the @pages are added, the tail iovec is instantiated pointing
// to end of the head buffer, and the stream is set up to encode
// subsequent items into the tail.
//
// svcxdr_set_auth_slack -
// @rqstp: RPC transaction
// @slack: buffer space to reserve for the transaction's security flavor
//
// Set the request's slack space requirement, and set aside that much
// space in the rqstp's rq_res.head for use when the auth wraps the Reply.
//
// svcxdr_set_accept_stat - Reserve space for the accept_stat field
// @rqstp: RPC transaction context
//
// Return values:
// %true: Success
// %false: No response buffer space was available
//
// rqstp->rq_accept_statp = rpc_success;
