//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/optee/optee_private.h
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
// Copyright (c) 2015-2021, 2023 Linaro Limited
//

pub const OPTEE_MAX_ARG_SIZE: c_int = 1024;
// Some Global Platform error codes used in this driver
pub const TEEC_SUCCESS: c_uint = 0x00000000;
pub const TEEC_ERROR_BAD_PARAMETERS: c_uint = 0xFFFF0006;
pub const TEEC_ERROR_ITEM_NOT_FOUND: c_uint = 0xFFFF0008;
pub const TEEC_ERROR_NOT_SUPPORTED: c_uint = 0xFFFF000A;
pub const TEEC_ERROR_COMMUNICATION: c_uint = 0xFFFF000E;
pub const TEEC_ERROR_OUT_OF_MEMORY: c_uint = 0xFFFF000C;
pub const TEEC_ERROR_BUSY: c_uint = 0xFFFF000D;
pub const TEEC_ERROR_SHORT_BUFFER: c_uint = 0xFFFF0010;
// API Return Codes are from the GP TEE Internal Core API Specification
pub const TEE_ERROR_TIMEOUT: c_uint = 0xFFFF3001;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE: c_uint = 0xF0100003;
pub const TEEC_ORIGIN_COMMS: c_uint = 0x00000002;
//
// This value should be larger than the number threads in secure world to
// meet the need from secure world. The number of threads in secure world
// are usually not even close to 255 so we should be safe for now.
//
pub const OPTEE_DEFAULT_MAX_NOTIF_VALUE: c_int = 255;
//
// struct optee_call_waiter - TEE entry may need to wait for a free TEE thread
// @list_node:	reference in waiters list
// @c:		waiting completion reference
// @sys_thread:	true if waiter belongs to a system thread
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_call_waiter {
    pub list_node: list_head,
    pub c: completion,
    pub sys_thread: bool,
}

//
// struct optee_call_queue - OP-TEE call queue management
// @mutex:			serializes access to this struct
// @waiters:			list of threads waiting to enter OP-TEE
// @total_thread_count:		overall number of thread context in OP-TEE or 0
// @free_thread_count:		number of threads context free in OP-TEE
// @sys_thread_req_count:	number of registered system thread sessions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_call_queue {
// Serializes access to this struct
    pub mutex: mutex,
    pub waiters: list_head,
    pub total_thread_count: c_int,
    pub free_thread_count: c_int,
    pub sys_thread_req_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_notif {
    pub max_key: u_int,
// Serializes access to the elements below in this struct
    pub lock: spinlock_t,
    pub db: list_head,
    pub bitmap: *mut u_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_shm_arg_cache {
    pub flags: u32,
// Serializes access to this struct
    pub mutex: mutex,
    pub shm_args: list_head,
}

//
// struct optee_supp - supplicant synchronization struct
// @mutex:	held while accessing content of this struct
// @ctx:	the context of current connected supplicant.
// if !NULL the supplicant device is available for use,
// else busy
// @req_id:	current request id if supplicant is doing synchronous
// communication, else -1
// @reqs:	queued request not yet retrieved by supplicant
// @idr:	IDR holding all requests currently being processed
// by supplicant
// @reqs_c:	completion used by supplicant when waiting for a
// request to be queued.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_supp {
// Serializes access to this struct
    pub mutex: mutex,
    pub ctx: *mut tee_context,
    pub req_id: c_int,
    pub reqs: list_head,
    pub idr: idr,
    pub reqs_c: completion,
}

//
// struct optee_pcpu - per cpu notif private struct passed to work functions
// @optee:	optee device reference
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_pcpu {
    pub optee: *mut optee,
}

//
// struct optee_smc - optee smc communication struct
// @invoke_fn:		handler function to invoke secure monitor
// @memremaped_shm:	virtual address of memory in shared memory pool
// @sec_caps:		secure world capabilities defined by
// OPTEE_SMC_SEC_CAP_* in optee_smc.h
// @notif_irq:		interrupt used as async notification by OP-TEE or 0
// @optee_pcpu:		per_cpu optee instance for per cpu work or NULL
// @notif_pcpu_wq:	workqueue for per cpu asynchronous notification or NULL
// @notif_pcpu_work:	work for per cpu asynchronous notification
// @notif_cpuhp_state:	CPU hotplug state assigned for pcpu interrupt management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_smc {
    pub invoke_fn: *mut optee_invoke_fn,
    pub memremaped_shm: *mut c_void,
    pub sec_caps: u32,
    pub notif_irq: c_uint,
    pub optee_pcpu: *mut optee_pcpu __percpu,
    pub notif_pcpu_wq: *mut workqueue_struct,
    pub notif_pcpu_work: work_struct,
    pub notif_cpuhp_state: c_uint,
}

//
// struct optee_ffa -  FFA communication struct
// @ffa_dev:		FFA device, contains the destination id, the id of
// OP-TEE in secure world
// @bottom_half_value:	notification ID used for bottom half signalling or
// U32_MAX if unused
// @mutex:		serializes access to @global_ids
// @global_ids:		FF-A shared memory global handle translation
// @notif_wq:		workqueue for FF-A asynchronous notification
// @notif_work:		work for FF-A asynchronous notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_ffa {
    pub ffa_dev: *mut ffa_device,
    pub bottom_half_value: u32,
// Serializes access to @global_ids
    pub mutex: mutex,
    pub global_ids: rhashtable,
    pub notif_wq: *mut workqueue_struct,
    pub notif_work: work_struct,
}

//
// struct optee_revision - OP-TEE OS revision reported by secure world
// @os_major:		OP-TEE OS major version
// @os_minor:		OP-TEE OS minor version
// @os_build_id:	OP-TEE OS build identifier (0 if unspecified)
//
// Values come from OPTEE_SMC_CALL_GET_OS_REVISION (SMC ABI) or
// OPTEE_FFA_GET_OS_VERSION (FF-A ABI); this is the trusted OS revision, not an
// FF-A ABI version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_revision {
    pub os_major: u32,
    pub os_minor: u32,
    pub os_build_id: u64,
}

extern "C" {
    pub fn optee_get_revision(teedev: *mut tee_device, buf: *mut c_char, len: usize) -> c_int;
}
//
// struct optee_ops - OP-TEE driver internal operations
// @do_call_with_arg:	enters OP-TEE in secure world
// @to_msg_param:	converts from struct tee_param to OPTEE_MSG parameters
// @from_msg_param:	converts from OPTEE_MSG parameters to struct tee_param
// @lend_protmem:	lends physically contiguous memory as restricted
// memory, inaccessible by the kernel
// @reclaim_protmem:	reclaims restricted memory previously lent with
// @lend_protmem() and makes it accessible by the
// kernel again
//
// These OPs are only supposed to be used internally in the OP-TEE driver
// as a way of abstracting the different methods of entering OP-TEE in
// secure world.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_ops {
    pub system_thread): bool,
    pub params): *const size_t num_params, struct tee_param,
    pub msg_params): *const optee_msg_param,
    pub use_case): u32,
    pub protmem): *mut *mut *mut int (reclaim_protmem)(struct optee optee, struct tee_shm,
}

//
// struct optee - main service struct
// @supp_teedev:		supplicant device
// @teedev:			client device
// @ops:			internal callbacks for different ways to reach
// secure world
// @ctx:			driver internal TEE context
// @smc:			specific to SMC ABI
// @ffa:			specific to FF-A ABI
// @shm_arg_cache:		shared memory cache argument
// @call_queue:			queue of threads waiting to call @invoke_fn
// @notif:			notification synchronization struct
// @supp:			supplicant synchronization struct for RPC to
// supplicant
// @pool:			shared memory pool
// @rpmb_dev_mutex:		mutex protecting @rpmb_dev
// @rpmb_dev:			current RPMB device or NULL
// @rpmb_intf:			RPMB notifier block
// @rpc_param_count:		if > 0 number of RPC parameters to make room for
// @scan_bus_done:		flag if device registation was already done
// @rpmb_scan_bus_done:		flag if device registation of RPMB dependent
// devices was already done
// @in_kernel_rpmb_routing:	flag if OP-TEE supports in-kernel RPMB routing
// @scan_bus_work:		workq to scan optee bus and register optee
// drivers
// @rpmb_scan_bus_work:		workq to for an RPMB device and to scan optee
// bus and register RPMB dependent optee drivers
// @revision:			OP-TEE OS revision
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee {
    pub supp_teedev: *mut tee_device,
    pub teedev: *mut tee_device,
    pub ops: *const optee_ops,
    pub ctx: *mut tee_context,
    pub smc: optee_smc,
    pub ffa: optee_ffa,
}

// Protects rpmb_dev pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_session {
    pub list_node: list_head,
    pub session_id: u32,
    pub use_sys_thread: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_context_data {
// Serializes access to this struct
    pub mutex: mutex,
    pub sess_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_rpc_param {
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
}

// Holds context that is preserved during one STD call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_call_ctx {
// information about pages list used in last allocation
    pub pages_list: *mut c_void,
    pub num_entries: usize,
}

extern "C" {
    pub fn optee_set_dma_mask(optee: *mut optee, pa_width: u_int) -> c_int;
}
extern "C" {
    pub fn optee_notif_init(optee: *mut optee, max_key: u_int) -> c_int;
}
extern "C" {
    pub fn optee_notif_uninit(optee: *mut optee);
}
extern "C" {
    pub fn optee_notif_wait(optee: *mut optee, key: u_int, timeout: u32) -> c_int;
}
extern "C" {
    pub fn optee_notif_send(optee: *mut optee, key: u_int) -> c_int;
}
extern "C" {
    pub fn optee_supp_init(supp: *mut optee_supp);
}
extern "C" {
    pub fn optee_supp_uninit(supp: *mut optee_supp);
}
extern "C" {
    pub fn optee_supp_release(supp: *mut optee_supp);
}
extern "C" {
    pub fn optee_system_session(ctx: *mut tee_context, session: u32) -> c_int;
}
extern "C" {
    pub fn optee_close_session(ctx: *mut tee_context, session: u32) -> c_int;
}
extern "C" {
    pub fn optee_cancel_req(ctx: *mut tee_context, cancel_id: u32, session: u32) -> c_int;
}
pub const PTA_CMD_GET_DEVICES: c_uint = 0x0;
pub const PTA_CMD_GET_DEVICES_SUPP: c_uint = 0x1;
pub const PTA_CMD_GET_DEVICES_RPMB: c_uint = 0x2;
extern "C" {
    pub fn optee_enumerate_devices(func: u32) -> c_int;
}
extern "C" {
    pub fn optee_unregister_devices();
}
extern "C" {
    pub fn optee_bus_scan_rpmb(work: *mut work_struct);
}
extern "C" {
    pub fn optee_set_dev_group(optee: *mut optee);
}
extern "C" {
    pub fn optee_remove_common(optee: *mut optee);
}
extern "C" {
    pub fn optee_open(ctx: *mut tee_context, cap_memref_null: bool) -> c_int;
}
extern "C" {
    pub fn optee_release(ctx: *mut tee_context);
}
extern "C" {
    pub fn optee_release_supp(ctx: *mut tee_context);
}
extern "C" {
    pub fn optee_cq_init(cq: *mut optee_call_queue, thread_count: c_int);
}
extern "C" {
    pub fn optee_check_mem_type(start: c_ulong, num_pages: usize) -> c_int;
}
extern "C" {
    pub fn optee_shm_arg_cache_init(optee: *mut optee, flags: u32);
}
extern "C" {
    pub fn optee_shm_arg_cache_uninit(optee: *mut optee);
}
extern "C" {
    pub fn optee_msg_arg_size(rpc_param_count: usize) -> usize;
}
extern "C" {
    pub fn optee_rpc_cmd_free_suppl(ctx: *mut tee_context, shm: *mut tee_shm);
}
extern "C" {
    pub fn optee_do_bottom_half(ctx: *mut tee_context) -> c_int;
}
extern "C" {
    pub fn optee_stop_async_notif(ctx: *mut tee_context) -> c_int;
}
//
// Small helpers
//
// reg0 = val >> 32;
// reg1 = val;
// Registration of the ABIs
extern "C" {
    pub fn optee_smc_abi_register() -> c_int;
}
extern "C" {
    pub fn optee_smc_abi_unregister();
}
extern "C" {
    pub fn optee_ffa_abi_register() -> c_int;
}
extern "C" {
    pub fn optee_ffa_abi_unregister();
}
