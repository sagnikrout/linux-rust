//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/clnt.h
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
// linux/include/linux/sunrpc/clnt.h
//
// Declarations for the high-level RPC client interface
//
// Copyright (C) 1995, 1996, Olaf Kirch <okir@monad.swb.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_sysfs_client {
    pub kobject: kobject,
    pub net: *mut net,
    pub clnt: *mut rpc_clnt,
    pub xprt_switch: *mut rpc_xprt_switch,
}

//
// The high-level client handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_clnt {
    pub /: *mut *mut refcount_t cl_count; / Number of references,
    pub /: *mut *mut unsigned int cl_clid; / client id,
    pub /: *mut *mut list_head cl_clients; / Global list of clients,
    pub /: *mut *mut list_head cl_tasks; / List of tasks,
    pub /: *mut *mut atomic_t cl_pid; / task PID counter,
    pub /: *mut *mut spinlock_t cl_lock; / spinlock,
    pub /: *mut *mut *mut rpc_xprt __rcu  cl_xprt; / transport,
    pub /: *const *const *const rpc_procinfo cl_procinfo; / procedure info,
    pub /: *mut *mut cl_maxproc; / max procedure number,
    pub /: *mut *mut *mut rpc_auth  cl_auth; / authenticator,
    pub /: *mut *mut *mut rpc_stat  cl_stats; / per-program statistics,
    pub /: *mut *mut *mut rpc_iostats  cl_metrics; / per-client statistics,
    pub 1: cl_netunreach_fatal :,
// Treat ENETUNREACH errors as fatal
    pub /: *mut *mut xprtsec_parms cl_xprtsec; / transport security policy,
    pub /: *mut *mut *mut rpc_rtt  cl_rtt; / RTO estimator data,
    pub /: *const *const *const rpc_timeout cl_timeout; / Timeout strategy,
    pub /: *mut *mut atomic_t cl_swapper; / swapfile count,
    pub /: *mut *mut int cl_nodelen; / nodename length,
    pub cl_nodename: [c_char; UNX_MAXNODENAME+1],
    pub cl_pipedir_objects: rpc_pipe_dir_head,
    pub /: *mut *mut *mut rpc_clnt  cl_parent; / Points to parent of clones,
    pub cl_rtt_default: rpc_rtt,
    pub cl_timeout_default: rpc_timeout,
    pub cl_program: *const rpc_program,
    pub /: *const *const *const char  cl_principal; / use for machine cred,

    pub /: *mut *mut *mut dentry cl_debugfs; / debugfs directory,

    pub /: *mut *mut *mut rpc_sysfs_client cl_sysfs; / sysfs directory,
// cl_work is only needed after cl_xpi is no longer used,
// and that are of similar size
//
    pub cl_xpi: rpc_xprt_iter,
    pub cl_work: work_struct,
}

//
// General RPC program info
//
pub const RPC_MAXVERSION: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_program {
    pub /: *const *const *const char  name; / protocol name,
    pub /: *mut *mut u32 number; / program number,
    pub /: *mut *mut unsigned int nrvers; / number of versions,
    pub /: *const *const *const *const rpc_version  version; / version array,
    pub /: *mut *mut *mut rpc_stat  stats; / statistics,
    pub /: *const *const *const char  pipe_dir_name; / path to rpc_pipefs dir,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_version {
    pub /: *mut *mut u32 number; / version number,
    pub /: *mut *mut unsigned int nrprocs; / number of procs,
    pub /: *const *const *const rpc_procinfo procs; / procedure array,
    pub /: *mut *mut *mut unsigned int counts; / call counts,
}

//
// Procedure information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_procinfo {
    pub /: *mut *mut u32 p_proc; / RPC procedure number,
    pub /: *mut *mut kxdreproc_t p_encode; / XDR encode function,
    pub /: *mut *mut kxdrdproc_t p_decode; / XDR decode function,
    pub /: *mut *mut unsigned int p_arglen; / argument hdr length (u32),
    pub /: *mut *mut unsigned int p_replen; / reply hdr length (u32),
    pub /: *mut *mut unsigned int p_timer; / Which RTT timer to use,
    pub /: *mut *mut u32 p_statidx; / Which procedure to account,
    pub /: *const *const *const char  p_name; / name of procedure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_create_args {
    pub net: *mut net,
    pub protocol: c_int,
    pub address: *mut sockaddr,
    pub addrsize: usize,
    pub saddress: *mut sockaddr,
    pub timeout: *const rpc_timeout,
    pub servername: *const c_char,
    pub nodename: *const c_char,
    pub program: *const rpc_program,
    pub stats: *mut rpc_stat,
    pub /: *mut *mut u32 prognumber; / overrides program->number,
    pub version: u32,
    pub authflavor: rpc_authflavor_t,
    pub nconnect: u32,
    pub flags: c_ulong,
    pub client_name: *mut c_char,
    pub /: *mut *mut *mut svc_xprt bc_xprt; / NFSv4.1 backchannel,
    pub cred: *const cred,
    pub max_connect: c_uint,
    pub xprtsec: xprtsec_parms,
    pub connect_timeout: c_ulong,
    pub reconnect_timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_add_xprt_test {
    pub calldata): *mut c_void,
    pub data: *mut c_void,
}

// Values for "flags" field

extern "C" {
    pub fn rpc_shutdown_client(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_hold_client(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_release_client(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_task_release_transport(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_task_release_client(: *mut rpc_task);
}
extern "C" {
    pub fn rpcb_create_local(: *mut net) -> c_int;
}
extern "C" {
    pub fn rpcb_put_local(: *mut net);
}
extern "C" {
    pub fn rpcb_register(: *mut net, _arg: u32, _arg: u32, _arg: c_int, short: unsigned) -> c_int;
}
extern "C" {
    pub fn rpcb_getport_async(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_call_start(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_restart_call_prepare(: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn rpc_restart_call(: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn rpc_setbufsize(: *mut rpc_clnt, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn rpc_net_ns(: *mut rpc_clnt) -> *mut net;
}
extern "C" {
    pub fn rpc_max_payload(: *mut rpc_clnt) -> usize;
}
extern "C" {
    pub fn rpc_max_bc_payload(: *mut rpc_clnt) -> usize;
}
extern "C" {
    pub fn rpc_num_bc_slots(: *mut rpc_clnt) -> c_uint;
}
extern "C" {
    pub fn rpc_force_rebind(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_peeraddr(: *mut rpc_clnt, : *mut sockaddr, _arg: usize) -> usize;
}
extern "C" {
    pub fn rpc_localaddr(: *mut rpc_clnt, : *mut sockaddr, _arg: usize) -> c_int;
}
extern "C" {
    pub fn rpc_clnt_manage_trunked_xprts(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_clnt_xprt_switch_add_xprt(: *mut rpc_clnt, : *mut rpc_xprt);
}
extern "C" {
    pub fn rpc_clnt_xprt_switch_remove_xprt(: *mut rpc_clnt, : *mut rpc_xprt);
}
extern "C" {
    pub fn rpc_clnt_xprt_set_online(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn rpc_clnt_disconnect(clnt: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_cleanup_clids();
}
