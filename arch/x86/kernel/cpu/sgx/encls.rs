//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/sgx/encls.h
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

// Retrieve the encoded trapnr from the specified return code.

// Issue a WARN() about an ENCLS function.

//
// encls_faulted() - Check if an ENCLS leaf faulted given an error code
// @ret:	the return value of an ENCLS leaf function call
//
// Return:
// - true:	ENCLS leaf faulted.
// - false:	Otherwise.
//
// encls_failed() - Check if an ENCLS function failed
// @ret:	the return value of an ENCLS function call
//
// Check if an ENCLS function failed. This happens when the function causes a
// fault that is not caused by an EPCM conflict or when the function returns a
// non-zero value.
//
// __encls_ret_N - encode an ENCLS function that returns an error code in EAX
// @rax:	function number
// @inputs:	asm inputs for the function
//
// Emit assembly for an ENCLS function that returns an error code, e.g. EREMOVE.
// And because SGX isn't complex enough as it is, function that return an error
// code also modify flags.
//
// Return:
// 0 on success,
// SGX error code on failure
//

//
// __encls_N - encode an ENCLS function that doesn't return an error code
// @rax:	function number
// @rbx_out:	optional output variable
// @inputs:	asm inputs for the function
//
// Emit assembly for an ENCLS function that does not return an error code, e.g.
// ECREATE.  Leaves without error codes either succeed or fault.  @rbx_out is an
// optional parameter for use by EDGBRD, which returns the requested value in
// RBX.
//
// Return:
// 0 on success,
// trapnr with SGX_ENCLS_FAULT_FLAG set on fault
//

// Initialize an EPC page into an SGX Enclave Control Structure (SECS) page.
extern "C" {
    pub fn __encls_2(_arg: ECREATE, _arg: pginfo, _arg: secs) -> return;
}
// Hash a 256 byte region of an enclave page to SECS:MRENCLAVE.
extern "C" {
    pub fn __encls_2(_arg: EEXTEND, _arg: secs, _arg: addr) -> return;
}
//
// Associate an EPC page to an enclave either as a REG or TCS page
// populated with the provided data.
//
extern "C" {
    pub fn __encls_2(_arg: EADD, _arg: pginfo, _arg: addr) -> return;
}
// Finalize enclave build, initialize enclave for user code execution.
extern "C" {
    pub fn __encls_ret_3(_arg: EINIT, _arg: sigstruct, _arg: secs, _arg: token) -> return;
}
// Disassociate EPC page from its enclave and mark it as unused.
extern "C" {
    pub fn __encls_ret_1(_arg: EREMOVE, _arg: addr) -> return;
}
// Copy data to an EPC page belonging to a debug enclave.
extern "C" {
    pub fn __encls_2(_arg: EDGBWR, _arg: *mut data, _arg: addr) -> return;
}
// Copy data from an EPC page belonging to a debug enclave.
extern "C" {
    pub fn __encls_1_1(_arg: EDGBRD, _arg: *mut data, _arg: addr) -> return;
}
// Track that software has completed the required TLB address clears.
extern "C" {
    pub fn __encls_ret_1(_arg: ETRACK, _arg: addr) -> return;
}
// Load, verify, and unblock an EPC page.
extern "C" {
    pub fn __encls_ret_3(_arg: ELDU, _arg: pginfo, _arg: addr, _arg: va) -> return;
}
// Make EPC page inaccessible to enclave, ready to be written to memory.
extern "C" {
    pub fn __encls_ret_1(_arg: EBLOCK, _arg: addr) -> return;
}
// Initialize an EPC page into a Version Array (VA) page.
extern "C" {
    pub fn __encls_2(_arg: EPA, _arg: rbx, _arg: addr) -> return;
}
// Invalidate an EPC page and write it out to main memory.
extern "C" {
    pub fn __encls_ret_3(_arg: EWB, _arg: pginfo, _arg: addr, _arg: va) -> return;
}
// Restrict the EPCM permissions of an EPC page.
extern "C" {
    pub fn __encls_ret_2(_arg: EMODPR, _arg: secinfo, _arg: addr) -> return;
}
// Change the type of an EPC page.
extern "C" {
    pub fn __encls_ret_2(_arg: EMODT, _arg: secinfo, _arg: addr) -> return;
}
// Zero a page of EPC memory and add it to an initialized enclave.
extern "C" {
    pub fn __encls_2(_arg: EAUG, _arg: pginfo, _arg: addr) -> return;
}
// Attempt to update CPUSVN at runtime.
extern "C" {
    pub fn __encls_ret_1(_arg: EUPDATESVN, _arg: "") -> return;
}
