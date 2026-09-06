//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/sgx.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright(c) 2016-20 Intel Corporation.
//

//
// enum sgx_page_flags - page control flags
// @SGX_PAGE_MEASURE:	Measure the page contents with a sequence of
// ENCLS[EEXTEND] operations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_page_flags {
    SGX_PAGE_MEASURE	= 0x01,
}

pub const SGX_MAGIC: c_uint = 0xA4;

//
// struct sgx_enclave_create - parameter structure for the
// %SGX_IOC_ENCLAVE_CREATE ioctl
// @src:	address for the SECS page data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_create {
    pub src: __u64,
}

//
// struct sgx_enclave_add_pages - parameter structure for the
// %SGX_IOC_ENCLAVE_ADD_PAGE ioctl
// @src:	start address for the page data
// @offset:	starting page offset
// @length:	length of the data (multiple of the page size)
// @secinfo:	address for the SECINFO data
// @flags:	page control flags
// @count:	number of bytes added (multiple of the page size)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_add_pages {
    pub src: __u64,
    pub offset: __u64,
    pub length: __u64,
    pub secinfo: __u64,
    pub flags: __u64,
    pub count: __u64,
}

//
// struct sgx_enclave_init - parameter structure for the
// %SGX_IOC_ENCLAVE_INIT ioctl
// @sigstruct:	address for the SIGSTRUCT data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_init {
    pub sigstruct: __u64,
}

//
// struct sgx_enclave_provision - parameter structure for the
// %SGX_IOC_ENCLAVE_PROVISION ioctl
// @fd:		file handle of /dev/sgx_provision
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_provision {
    pub fd: __u64,
}

//
// struct sgx_enclave_restrict_permissions - parameters for ioctl
// %SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS
// @offset:	starting page offset (page aligned relative to enclave base
// address defined in SECS)
// @length:	length of memory (multiple of the page size)
// @permissions:new permission bits for pages in range described by @offset
// and @length
// @result:	(output) SGX result code of ENCLS[EMODPR] function
// @count:	(output) bytes successfully changed (multiple of page size)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_restrict_permissions {
    pub offset: __u64,
    pub length: __u64,
    pub permissions: __u64,
    pub result: __u64,
    pub count: __u64,
}

//
// struct sgx_enclave_modify_types - parameters for ioctl
// %SGX_IOC_ENCLAVE_MODIFY_TYPES
// @offset:	starting page offset (page aligned relative to enclave base
// address defined in SECS)
// @length:	length of memory (multiple of the page size)
// @page_type:	new type for pages in range described by @offset and @length
// @result:	(output) SGX result code of ENCLS[EMODT] function
// @count:	(output) bytes successfully changed (multiple of page size)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_modify_types {
    pub offset: __u64,
    pub length: __u64,
    pub page_type: __u64,
    pub result: __u64,
    pub count: __u64,
}

//
// struct sgx_enclave_remove_pages - %SGX_IOC_ENCLAVE_REMOVE_PAGES parameters
// @offset:	starting page offset (page aligned relative to enclave base
// address defined in SECS)
// @length:	length of memory (multiple of the page size)
// @count:	(output) bytes successfully changed (multiple of page size)
//
// Regular (PT_REG) or TCS (PT_TCS) can be removed from an initialized
// enclave if the system supports SGX2. First, the %SGX_IOC_ENCLAVE_MODIFY_TYPES
// ioctl() should be used to change the page type to PT_TRIM. After that
// succeeds ENCLU[EACCEPT] should be run from within the enclave and then
// %SGX_IOC_ENCLAVE_REMOVE_PAGES can be used to complete the page removal.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_remove_pages {
    pub offset: __u64,
    pub length: __u64,
    pub count: __u64,
}

//
// typedef sgx_enclave_user_handler_t - Exit handler function accepted by
// __vdso_sgx_enter_enclave()
// @rdi:	RDI at the time of EEXIT, undefined on AEX
// @rsi:	RSI at the time of EEXIT, undefined on AEX
// @rdx:	RDX at the time of EEXIT, undefined on AEX
// @rsp:	RSP (untrusted) at the time of EEXIT or AEX
// @r8:		R8 at the time of EEXIT, undefined on AEX
// @r9:		R9 at the time of EEXIT, undefined on AEX
// @run:	The run instance given by the caller
//
// The register parameters contain the snapshot of their values at enclave
// exit. An invalid ENCLU function number will cause -EINVAL to be returned
// to the caller.
//
// Return:
// - <= 0:	The given value is returned back to the caller.
// - > 0:	ENCLU function to invoke, either EENTER or ERESUME.
//
// struct sgx_enclave_run - the execution context of __vdso_sgx_enter_enclave()
// @tcs:			TCS used to enter the enclave
// @function:			The last seen ENCLU function (EENTER, ERESUME or EEXIT)
// @exception_vector:		The interrupt vector of the exception
// @exception_error_code:	The exception error code pulled out of the stack
// @exception_addr:		The address that triggered the exception
// @user_handler:		User provided callback run on exception
// @user_data:			Data passed to the user handler
// @reserved:			Reserved for future extensions
//
// If @user_handler is provided, the handler will be invoked on all return paths
// of the normal flow.  The user handler may transfer control, e.g. via a
// longjmp() call or a C++ exception, without returning to
// __vdso_sgx_enter_enclave().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_enclave_run {
    pub tcs: __u64,
    pub function: __u32,
    pub exception_vector: __u16,
    pub exception_error_code: __u16,
    pub exception_addr: __u64,
    pub user_handler: __u64,
    pub user_data: __u64,
    pub reserved: [__u8; 216],
}

//
// typedef vdso_sgx_enter_enclave_t - Prototype for __vdso_sgx_enter_enclave(),
// a vDSO function to enter an SGX enclave.
// @rdi:	Pass-through value for RDI
// @rsi:	Pass-through value for RSI
// @rdx:	Pass-through value for RDX
// @function:	ENCLU function, must be EENTER or ERESUME
// @r8:		Pass-through value for R8
// @r9:		Pass-through value for R9
// @run:	struct sgx_enclave_run, must be non-NULL
//
// NOTE: __vdso_sgx_enter_enclave() does not ensure full compliance with the
// x86-64 ABI, e.g. doesn't handle XSAVE state.  Except for non-volatile
// general purpose registers, EFLAGS.DF, and RSP alignment, preserving/setting
// state in accordance with the x86-64 ABI is the responsibility of the enclave
// and its runtime, i.e. __vdso_sgx_enter_enclave() cannot be called from C
// code without careful consideration by both the enclave and its runtime.
//
// All general purpose registers except RAX, RBX and RCX are passed as-is to the
// enclave.  RAX, RBX and RCX are consumed by EENTER and ERESUME and are loaded
// with @function, asynchronous exit pointer, and @run.tcs respectively.
//
// RBP and the stack are used to anchor __vdso_sgx_enter_enclave() to the
// pre-enclave state, e.g. to retrieve @run.exception and @run.user_handler
// after an enclave exit.  All other registers are available for use by the
// enclave and its runtime, e.g. an enclave can push additional data onto the
// stack (and modify RSP) to pass information to the optional user handler (see
// below).
//
// Most exceptions reported on ENCLU, including those that occur within the
// enclave, are fixed up and reported synchronously instead of being delivered
// via a standard signal. Debug Exceptions (#DB) and Breakpoints (#BP) are
// never fixed up and are always delivered via standard signals. On synchronously
// reported exceptions, -EFAULT is returned and details about the exception are
// recorded in @run.exception, the optional sgx_enclave_exception struct.
//
// Return:
// - 0:		ENCLU function was successfully executed.
// - -EINVAL:	Invalid ENCL number (neither EENTER nor ERESUME).
//
