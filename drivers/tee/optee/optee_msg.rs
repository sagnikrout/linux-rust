//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/optee/optee_msg.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Copyright (c) 2015-2021, Linaro Limited
//

//
// This file defines the OP-TEE message protocol (ABI) used to communicate
// with an instance of OP-TEE running in secure world.
//
// This file is divided into two sections.
// 1. Formatting of messages.
// 2. Requests from normal world
//
// Part 1 - formatting of messages
//
pub const OPTEE_MSG_ATTR_TYPE_NONE: c_uint = 0x0;
pub const OPTEE_MSG_ATTR_TYPE_VALUE_INPUT: c_uint = 0x1;
pub const OPTEE_MSG_ATTR_TYPE_VALUE_OUTPUT: c_uint = 0x2;
pub const OPTEE_MSG_ATTR_TYPE_VALUE_INOUT: c_uint = 0x3;
pub const OPTEE_MSG_ATTR_TYPE_RMEM_INPUT: c_uint = 0x5;
pub const OPTEE_MSG_ATTR_TYPE_RMEM_OUTPUT: c_uint = 0x6;
pub const OPTEE_MSG_ATTR_TYPE_RMEM_INOUT: c_uint = 0x7;

pub const OPTEE_MSG_ATTR_TYPE_TMEM_INPUT: c_uint = 0x9;
pub const OPTEE_MSG_ATTR_TYPE_TMEM_OUTPUT: c_uint = 0xa;
pub const OPTEE_MSG_ATTR_TYPE_TMEM_INOUT: c_uint = 0xb;

//
// Meta parameter to be absorbed by the Secure OS and not passed
// to the Trusted Application.
//
// Currently only used with OPTEE_MSG_CMD_OPEN_SESSION.
//

//
// Pointer to a list of pages used to register user-defined SHM buffer.
// Used with OPTEE_MSG_ATTR_TYPE_TMEM_*.
// buf_ptr should point to the beginning of the buffer. Buffer will contain
// list of page addresses. OP-TEE core can reconstruct contiguous buffer from
// that page addresses list. Page addresses are stored as 64 bit values.
// Last entry on a page should point to the next page of buffer.
// Every entry in buffer should point to a 4k page beginning (12 least
// significant bits must be equal to zero).
//
// 12 least significant bits of optee_msg_param.u.tmem.buf_ptr should hold
// page offset of user buffer.
//
// So, entries should be placed like members of this structure:
//
// struct page_data {
// uint64_t pages_array[OPTEE_MSG_NONCONTIG_PAGE_SIZE/sizeof(uint64_t) - 1];
// uint64_t next_page_data;
// };
//
// Structure is designed to exactly fit into the page size
// OPTEE_MSG_NONCONTIG_PAGE_SIZE which is a standard 4KB page.
//
// The size of 4KB is chosen because this is the smallest page size for ARM
// architectures. If REE uses larger pages, it should divide them to 4KB ones.
//

//
// Memory attributes for caching passed with temp memrefs. The actual value
// used is defined outside the message protocol with the exception of
// OPTEE_MSG_ATTR_CACHE_PREDEFINED which means the attributes already
// defined for the memory range should be used. If optee_smc.h is used as
// bearer of this protocol OPTEE_SMC_SHM_* is used for values.
//
pub const OPTEE_MSG_ATTR_CACHE_SHIFT: c_int = 16;

pub const OPTEE_MSG_ATTR_CACHE_PREDEFINED: c_int = 0;
//
// Same values as TEE_LOGIN_* from TEE Internal API
//
pub const OPTEE_MSG_LOGIN_PUBLIC: c_uint = 0x00000000;
pub const OPTEE_MSG_LOGIN_USER: c_uint = 0x00000001;
pub const OPTEE_MSG_LOGIN_GROUP: c_uint = 0x00000002;
pub const OPTEE_MSG_LOGIN_APPLICATION: c_uint = 0x00000004;
pub const OPTEE_MSG_LOGIN_APPLICATION_USER: c_uint = 0x00000005;
pub const OPTEE_MSG_LOGIN_APPLICATION_GROUP: c_uint = 0x00000006;
//
// Page size used in non-contiguous buffer entries
//
pub const OPTEE_MSG_NONCONTIG_PAGE_SIZE: c_int = 4096;
pub const OPTEE_MSG_FMEM_INVALID_GLOBAL_ID: c_uint = 0xffffffffffffffff;
//
// struct optee_msg_param_tmem - temporary memory reference parameter
// @buf_ptr:	address of the buffer
// @size:	size of the buffer
// @shm_ref:	temporary shared memory reference, pointer to a struct tee_shm
//
// Secure and normal world communicates pointers as physical address
// instead of the virtual address. This is because secure and normal world
// have completely independent memory mapping. Normal world can even have a
// hypervisor which need to translate the guest physical address (AKA IPA
// in ARM documentation) to a real physical address before passing the
// structure to secure world.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_param_tmem {
    pub buf_ptr: u64,
    pub size: u64,
    pub shm_ref: u64,
}

//
// struct optee_msg_param_rmem - registered memory reference parameter
// @offs:	offset into shared memory reference
// @size:	size of the buffer
// @shm_ref:	shared memory reference, pointer to a struct tee_shm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_param_rmem {
    pub offs: u64,
    pub size: u64,
    pub shm_ref: u64,
}

//
// struct optee_msg_param_fmem - FF-A memory reference parameter
// @offs_low:		lower bits of offset into shared memory reference
// @offs_high:		higher bits of offset into shared memory reference
// @internal_offs:	internal offset into the first page of shared memory
// reference
// @size:		size of the buffer
// @global_id:		global identifier of the shared memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_param_fmem {
    pub offs_low: u32,
    pub offs_high: u16,
    pub internal_offs: u16,
    pub size: u64,
    pub global_id: u64,
}

//
// struct optee_msg_param_value - opaque value parameter
// @a: first opaque value
// @b: second opaque value
// @c: third opaque value
//
// Value parameters are passed unchecked between normal and secure world.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_param_value {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

//
// struct optee_msg_param - parameter used together with struct optee_msg_arg
// @attr:	attributes
// @tmem:	parameter by temporary memory reference
// @rmem:	parameter by registered memory reference
// @fmem:	parameter by FF-A registered memory reference
// @value:	parameter by opaque value
// @octets:	parameter by octet string
// @u:		union holding OP-TEE msg parameter
//
// @attr & OPTEE_MSG_ATTR_TYPE_MASK indicates if tmem, rmem or value is used in
// the union. OPTEE_MSG_ATTR_TYPE_VALUE_* indicates value or octets,
// OPTEE_MSG_ATTR_TYPE_TMEM_* indicates @tmem and
// OPTEE_MSG_ATTR_TYPE_RMEM_* or the alias PTEE_MSG_ATTR_TYPE_FMEM_* indicates
// @rmem or @fmem depending on the conduit.
// OPTEE_MSG_ATTR_TYPE_NONE indicates that none of the members are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_param {
    pub attr: u64,
    pub tmem: optee_msg_param_tmem,
    pub rmem: optee_msg_param_rmem,
    pub fmem: optee_msg_param_fmem,
    pub value: optee_msg_param_value,
    pub octets: [u8; 24],
    pub u: },
}

//
// struct optee_msg_arg - call argument
// @cmd:	command, one of OPTEE_MSG_CMD_* or OPTEE_MSG_RPC_CMD_
// @func:	Trusted Application function, specific to the Trusted
// Application, used if cmd == OPTEE_MSG_CMD_INVOKE_COMMAND
// @session:	in parameter for all OPTEE_MSG_CMD_* except
// OPTEE_MSG_CMD_OPEN_SESSION where it's an output parameter
// instead
// @cancel_id:	cancellation id, a unique value to identify this request
// @pad:	padding for alignment
// @ret:	return value
// @ret_origin:	origin of the return value
// @num_params:	number of parameters supplied to the OS Command
// @params:	the parameters supplied to the OS Command
//
// All normal calls to Trusted OS uses this struct. If cmd requires further
// information than what these fields hold it can be passed as a parameter
// tagged as meta (setting the OPTEE_MSG_ATTR_META bit in corresponding
// attrs field). All parameters tagged as meta have to come first.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_msg_arg {
    pub cmd: u32,
    pub func: u32,
    pub session: u32,
    pub cancel_id: u32,
    pub pad: u32,
    pub ret: u32,
    pub ret_origin: u32,
    pub num_params: u32,
// num_params tells the actual number of element in params
    pub params: [optee_msg_param; ],
}

//
// OPTEE_MSG_GET_ARG_SIZE - return size of struct optee_msg_arg
//
// @num_params: Number of parameters embedded in the struct optee_msg_arg
//
// Returns the size of the struct optee_msg_arg together with the number
// of embedded parameters.
//

//
// Part 2 - requests from normal world
//
// Return the following UID if using API specified in this file without
// further extensions:
// 384fb3e0-e7f8-11e3-af63-0002a5d5c51b.
// Represented in 4 32-bit words in OPTEE_MSG_UID_0, OPTEE_MSG_UID_1,
// OPTEE_MSG_UID_2, OPTEE_MSG_UID_3.
//
// In the case where the OP-TEE image is loaded by the kernel, this will
// initially return an alternate UID to reflect that we are communicating with
// the TF-A image loading service at that time instead of OP-TEE. That UID is:
// a3fbeab1-1246-315d-c7c4-06b9c03cbea4.
// Represented in 4 32-bit words in OPTEE_MSG_IMAGE_LOAD_UID_0,
// OPTEE_MSG_IMAGE_LOAD_UID_1, OPTEE_MSG_IMAGE_LOAD_UID_2,
// OPTEE_MSG_IMAGE_LOAD_UID_3.
//
pub const OPTEE_MSG_UID_0: c_uint = 0x384fb3e0;
pub const OPTEE_MSG_UID_1: c_uint = 0xe7f811e3;
pub const OPTEE_MSG_UID_2: c_uint = 0xaf630002;
pub const OPTEE_MSG_UID_3: c_uint = 0xa5d5c51b;
pub const OPTEE_MSG_IMAGE_LOAD_UID_0: c_uint = 0xa3fbeab1;
pub const OPTEE_MSG_IMAGE_LOAD_UID_1: c_uint = 0x1246315d;
pub const OPTEE_MSG_IMAGE_LOAD_UID_2: c_uint = 0xc7c406b9;
pub const OPTEE_MSG_IMAGE_LOAD_UID_3: c_uint = 0xc03cbea4;
pub const OPTEE_MSG_FUNCID_CALLS_UID: c_uint = 0xFF01;
//
// Returns 2.0 if using API specified in this file without further
// extensions. Represented in 2 32-bit words in OPTEE_MSG_REVISION_MAJOR
// and OPTEE_MSG_REVISION_MINOR
//
pub const OPTEE_MSG_REVISION_MAJOR: c_int = 2;
pub const OPTEE_MSG_REVISION_MINOR: c_int = 0;
pub const OPTEE_MSG_FUNCID_CALLS_REVISION: c_uint = 0xFF03;
//
// Get UUID of Trusted OS.
//
// Used by non-secure world to figure out which Trusted OS is installed.
// Note that returned UUID is the UUID of the Trusted OS, not of the API.
//
// Returns UUID in 4 32-bit words in the same way as
// OPTEE_MSG_FUNCID_CALLS_UID described above.
//
pub const OPTEE_MSG_OS_OPTEE_UUID_0: c_uint = 0x486178e0;
pub const OPTEE_MSG_OS_OPTEE_UUID_1: c_uint = 0xe7f811e3;
pub const OPTEE_MSG_OS_OPTEE_UUID_2: c_uint = 0xbc5e0002;
pub const OPTEE_MSG_OS_OPTEE_UUID_3: c_uint = 0xa5d5c51b;
pub const OPTEE_MSG_FUNCID_GET_OS_UUID: c_uint = 0x0000;
//
// Get revision of Trusted OS.
//
// Used by non-secure world to figure out which version of the Trusted OS
// is installed. Note that the returned revision is the revision of the
// Trusted OS, not of the API.
//
// Returns revision in 2 32-bit words in the same way as
// OPTEE_MSG_CALLS_REVISION described above.
//
pub const OPTEE_MSG_FUNCID_GET_OS_REVISION: c_uint = 0x0001;
//
// Values used in OPTEE_MSG_CMD_LEND_PROTMEM below
// OPTEE_MSG_PROTMEM_RESERVED		Reserved
// OPTEE_MSG_PROTMEM_SECURE_VIDEO_PLAY	Secure Video Playback
// OPTEE_MSG_PROTMEM_TRUSTED_UI		Trused UI
// OPTEE_MSG_PROTMEM_SECURE_VIDEO_RECORD	Secure Video Recording
//
pub const OPTEE_MSG_PROTMEM_RESERVED: c_int = 0;
pub const OPTEE_MSG_PROTMEM_SECURE_VIDEO_PLAY: c_int = 1;
pub const OPTEE_MSG_PROTMEM_TRUSTED_UI: c_int = 2;
pub const OPTEE_MSG_PROTMEM_SECURE_VIDEO_RECORD: c_int = 3;
//
// Do a secure call with struct optee_msg_arg as argument
// The OPTEE_MSG_CMD_* below defines what goes in struct optee_msg_arg::cmd
//
// OPTEE_MSG_CMD_OPEN_SESSION opens a session to a Trusted Application.
// The first two parameters are tagged as meta, holding two value
// parameters to pass the following information:
// param[0].u.value.a-b uuid of Trusted Application
// param[1].u.value.a-b uuid of Client
// param[1].u.value.c Login class of client OPTEE_MSG_LOGIN_
//
// OPTEE_MSG_CMD_INVOKE_COMMAND invokes a command a previously opened
// session to a Trusted Application.  struct optee_msg_arg::func is Trusted
// Application function, specific to the Trusted Application.
//
// OPTEE_MSG_CMD_CLOSE_SESSION closes a previously opened session to
// Trusted Application.
//
// OPTEE_MSG_CMD_CANCEL cancels a currently invoked command.
//
// OPTEE_MSG_CMD_REGISTER_SHM registers a shared memory reference. The
// information is passed as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_TMEM_INPUT
// [| OPTEE_MSG_ATTR_NONCONTIG]
// [in] param[0].u.tmem.buf_ptr		physical address (of first fragment)
// [in] param[0].u.tmem.size		size (of first fragment)
// [in] param[0].u.tmem.shm_ref		holds shared memory reference
//
// OPTEE_MSG_CMD_UNREGISTER_SHM unregisters a previously registered shared
// memory reference. The information is passed as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_RMEM_INPUT
// [in] param[0].u.rmem.shm_ref		holds shared memory reference
// [in] param[0].u.rmem.offs		0
// [in] param[0].u.rmem.size		0
//
// OPTEE_MSG_CMD_DO_BOTTOM_HALF does the scheduled bottom half processing
// of a driver.
//
// OPTEE_MSG_CMD_STOP_ASYNC_NOTIF informs secure world that from now is
// normal world unable to process asynchronous notifications. Typically
// used when the driver is shut down.
//
// OPTEE_MSG_CMD_LEND_PROTMEM lends protected memory. The passed normal
// physical memory is protected from normal world access. The memory
// should be unmapped prior to this call since it becomes inaccessible
// during the request.
// Parameters are passed as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_VALUE_INPUT
// [in] param[0].u.value.a		OPTEE_MSG_PROTMEM_* defined above
// [in] param[1].attr			OPTEE_MSG_ATTR_TYPE_TMEM_INPUT
// [in] param[1].u.tmem.buf_ptr		physical address
// [in] param[1].u.tmem.size		size
// [in] param[1].u.tmem.shm_ref		holds protected memory reference
//
// OPTEE_MSG_CMD_RECLAIM_PROTMEM reclaims a previously lent protected
// memory reference. The physical memory is accessible by the normal world
// after this function has return and can be mapped again. The information
// is passed as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_VALUE_INPUT
// [in] param[0].u.value.a		holds protected memory cookie
//
// OPTEE_MSG_CMD_GET_PROTMEM_CONFIG get configuration for a specific
// protected memory use case. Parameters are passed as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_VALUE_INOUT
// [in] param[0].value.a		OPTEE_MSG_PROTMEM_
// [in] param[1].attr			OPTEE_MSG_ATTR_TYPE_{R,F}MEM_OUTPUT
// [in] param[1].u.{r,f}mem		Buffer or NULL
// [in] param[1].u.{r,f}mem.size	Provided size of buffer or 0 for query
// output for the protected use case:
// [out] param[0].value.a		Minimal size of protected memory
// [out] param[0].value.b		Required alignment of size and start of
// protected memory
// [out] param[0].value.c               PA width, max 64
// [out] param[1].{r,f}mem.size		Size of output data
// [out] param[1].{r,f}mem		If non-NULL, contains an array of
// uint32_t memory attributes that must be
// included when lending memory for this
// use case
//
// OPTEE_MSG_CMD_ASSIGN_PROTMEM assigns use-case to protected memory
// previously lent using the FFA_LEND framework ABI. Parameters are passed
// as:
// [in] param[0].attr			OPTEE_MSG_ATTR_TYPE_VALUE_INPUT
// [in] param[0].u.value.a		holds protected memory cookie
// [in] param[0].u.value.b		OPTEE_MSG_PROTMEM_* defined above
//
pub const OPTEE_MSG_CMD_OPEN_SESSION: c_int = 0;
pub const OPTEE_MSG_CMD_INVOKE_COMMAND: c_int = 1;
pub const OPTEE_MSG_CMD_CLOSE_SESSION: c_int = 2;
pub const OPTEE_MSG_CMD_CANCEL: c_int = 3;
pub const OPTEE_MSG_CMD_REGISTER_SHM: c_int = 4;
pub const OPTEE_MSG_CMD_UNREGISTER_SHM: c_int = 5;
pub const OPTEE_MSG_CMD_DO_BOTTOM_HALF: c_int = 6;
pub const OPTEE_MSG_CMD_STOP_ASYNC_NOTIF: c_int = 7;
pub const OPTEE_MSG_CMD_LEND_PROTMEM: c_int = 8;
pub const OPTEE_MSG_CMD_RECLAIM_PROTMEM: c_int = 9;
pub const OPTEE_MSG_CMD_GET_PROTMEM_CONFIG: c_int = 10;
pub const OPTEE_MSG_CMD_ASSIGN_PROTMEM: c_int = 11;
pub const OPTEE_MSG_FUNCID_CALL_WITH_ARG: c_uint = 0x0004;
