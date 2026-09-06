//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sgx.h
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
// Copyright(c) 2016-20 Intel Corporation.
//
// Intel Software Guard Extensions (SGX) support.
//

//
// This file contains both data structures defined by SGX architecture and Linux
// defined software data structures and functions.  The two should not be mixed
// together for better readability.  The architectural definitions come first.
//
// The SGX specific CPUID function.
pub const SGX_CPUID: c_uint = 0x12;
// EPC enumeration.
pub const SGX_CPUID_EPC: c_int = 2;
// An invalid EPC section, i.e. the end marker.
pub const SGX_CPUID_EPC_INVALID: c_uint = 0x0;
// A valid EPC section.
pub const SGX_CPUID_EPC_SECTION: c_uint = 0x1;
// The bitmask for the EPC section type.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_encls_function {
    ECREATE		= 0x00,
    EADD		= 0x01,
    EINIT		= 0x02,
    EREMOVE		= 0x03,
    EDGBRD		= 0x04,
    EDGBWR		= 0x05,
    EEXTEND		= 0x06,
    ELDU		= 0x08,
    EBLOCK		= 0x09,
    EPA		= 0x0A,
    EWB		= 0x0B,
    ETRACK		= 0x0C,
    EAUG		= 0x0D,
    EMODPR		= 0x0E,
    EMODT		= 0x0F,
    EUPDATESVN	= 0x18,
}

//
// SGX_ENCLS_FAULT_FLAG - flag signifying an ENCLS return code is a trapnr
//
// ENCLS has its own (positive value) error codes and also generates
// ENCLS specific #GP and #PF faults.  And the ENCLS values get munged
// with system error codes as everything percolates back up the stack.
// Unfortunately (for us), we need to precisely identify each unique
// error code, e.g. the action taken if EWB fails varies based on the
// type of fault and on the exact SGX error code, i.e. we can't simply
// convert all faults to -EFAULT.
//
// To make all three error types coexist, we set bit 30 to identify an
// ENCLS fault.  Bit 31 (technically bits N:31) is used to differentiate
// between positive (faults and SGX error codes) and negative (system
// error codes) values.
//
pub const SGX_ENCLS_FAULT_FLAG: c_uint = 0x40000000;
//
// enum sgx_return_code - The return code type for ENCLS, ENCLU and ENCLV
// @SGX_EPC_PAGE_CONFLICT:	Page is being written by other ENCLS function.
// @SGX_NOT_TRACKED:		Previous ETRACK's shootdown sequence has not
// been completed yet.
// @SGX_CHILD_PRESENT:		SECS has child pages present in the EPC.
// @SGX_INVALID_EINITTOKEN:	EINITTOKEN is invalid and enclave signer's
// public key does not match IA32_SGXLEPUBKEYHASH.
// @SGX_PAGE_NOT_MODIFIABLE:	The EPC page cannot be modified because it
// is in the PENDING or MODIFIED state.
// @SGX_INSUFFICIENT_ENTROPY:	Insufficient entropy in RNG.
// @SGX_NO_UPDATE:		EUPDATESVN could not update the CPUSVN because the
// current SVN was not newer than CPUSVN. This is the most
// common error code returned by EUPDATESVN.
// @SGX_UNMASKED_EVENT:		An unmasked event, e.g. INTR, was received
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_return_code {
    SGX_EPC_PAGE_CONFLICT		= 7,
    SGX_NOT_TRACKED			= 11,
    SGX_CHILD_PRESENT		= 13,
    SGX_INVALID_EINITTOKEN		= 16,
    SGX_PAGE_NOT_MODIFIABLE		= 20,
    SGX_INSUFFICIENT_ENTROPY	= 29,
    SGX_NO_UPDATE			= 31,
    SGX_UNMASKED_EVENT		= 128,
}

// The modulus size for 3072-bit RSA keys.
pub const SGX_MODULUS_SIZE: c_int = 384;
//
// enum sgx_miscselect - additional information to an SSA frame
// @SGX_MISC_EXINFO:	Report #PF or #GP to the SSA frame.
//
// Save State Area (SSA) is a stack inside the enclave used to store processor
// state when an exception or interrupt occurs. This enum defines additional
// information stored to an SSA frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_miscselect {
    SGX_MISC_EXINFO		= BIT(0),
}

pub const SGX_SSA_GPRS_SIZE: c_int = 184;
pub const SGX_SSA_MISC_EXINFO_SIZE: c_int = 16;
//
// enum sgx_attribute - the attributes field in &struct sgx_secs
// @SGX_ATTR_INIT:		Enclave can be entered (is initialized).
// @SGX_ATTR_DEBUG:		Allow ENCLS(EDBGRD) and ENCLS(EDBGWR).
// @SGX_ATTR_MODE64BIT:		Tell that this a 64-bit enclave.
// @SGX_ATTR_PROVISIONKEY:      Allow to use provisioning keys for remote
// attestation.
// @SGX_ATTR_KSS:		Allow to use key separation and sharing (KSS).
// @SGX_ATTR_EINITTOKENKEY:	Allow to use token signing key that is used to
// sign cryptographic tokens that can be passed to
// EINIT as an authorization to run an enclave.
// @SGX_ATTR_ASYNC_EXIT_NOTIFY:	Allow enclaves to be notified after an
// asynchronous exit has occurred.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_attribute {
    SGX_ATTR_INIT		   = BIT(0),
    SGX_ATTR_DEBUG		   = BIT(1),
    SGX_ATTR_MODE64BIT	   = BIT(2),
// BIT(3) is reserved
    SGX_ATTR_PROVISIONKEY	   = BIT(4),
    SGX_ATTR_EINITTOKENKEY	   = BIT(5),
// BIT(6) is for CET
    SGX_ATTR_KSS		   = BIT(7),
// BIT(8) is reserved
// BIT(9) is reserved
    SGX_ATTR_ASYNC_EXIT_NOTIFY = BIT(10),
}

//
// struct sgx_secs - SGX Enclave Control Structure (SECS)
// @size:		size of the address space
// @base:		base address of the  address space
// @ssa_frame_size:	size of an SSA frame
// @miscselect:		additional information stored to an SSA frame
// @attributes:		attributes for enclave
// @xfrm:		XSave-Feature Request Mask (subset of XCR0)
// @mrenclave:		SHA256-hash of the enclave contents
// @mrsigner:		SHA256-hash of the public key used to sign the SIGSTRUCT
// @config_id:		a user-defined value that is used in key derivation
// @isv_prod_id:	a user-defined value that is used in key derivation
// @isv_svn:		a user-defined value that is used in key derivation
// @config_svn:		a user-defined value that is used in key derivation
//
// SGX Enclave Control Structure (SECS) is a special enclave page that is not
// visible in the address space. In fact, this structure defines the address
// range and other global attributes for the enclave and it is the first EPC
// page created for any enclave. It is moved from a temporary buffer to an EPC
// by the means of ENCLS[ECREATE] function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_secs {
    pub size: u64,
    pub base: u64,
    pub ssa_frame_size: u32,
    pub miscselect: u32,
    pub reserved1: [u8; 24],
    pub attributes: u64,
    pub xfrm: u64,
    pub mrenclave: [u32; 8],
    pub reserved2: [u8; 32],
    pub mrsigner: [u32; 8],
    pub reserved3: [u8; 32],
    pub config_id: [u32; 16],
    pub isv_prod_id: u16,
    pub isv_svn: u16,
    pub config_svn: u16,
    pub reserved4: [u8; 3834],
    pub __packed: },
//
// enum sgx_tcs_flags - execution flags for TCS
// @SGX_TCS_DBGOPTIN:	If enabled allows single-stepping and breakpoints
// inside an enclave. It is cleared by EADD but can
// be set later with EDBGWR.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_tcs_flags {
    SGX_TCS_DBGOPTIN	= 0x01,
}

pub const SGX_TCS_RESERVED_SIZE: c_int = 4024;
//
// struct sgx_tcs - Thread Control Structure (TCS)
// @state:		used to mark an entered TCS
// @flags:		execution flags (cleared by EADD)
// @ssa_offset:		SSA stack offset relative to the enclave base
// @ssa_index:		the current SSA frame index (cleard by EADD)
// @nr_ssa_frames:	the number of frame in the SSA stack
// @entry_offset:	entry point offset relative to the enclave base
// @exit_addr:		address outside the enclave to exit on an exception or
// interrupt
// @fs_offset:		offset relative to the enclave base to become FS
// segment inside the enclave
// @gs_offset:		offset relative to the enclave base to become GS
// segment inside the enclave
// @fs_limit:		size to become a new FS-limit (only 32-bit enclaves)
// @gs_limit:		size to become a new GS-limit (only 32-bit enclaves)
//
// Thread Control Structure (TCS) is an enclave page visible in its address
// space that defines an entry point inside the enclave. A thread enters inside
// an enclave by supplying address of TCS to ENCLU(EENTER). A TCS can be entered
// by only one thread at a time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_tcs {
    pub state: u64,
    pub flags: u64,
    pub ssa_offset: u64,
    pub ssa_index: u32,
    pub nr_ssa_frames: u32,
    pub entry_offset: u64,
    pub exit_addr: u64,
    pub fs_offset: u64,
    pub gs_offset: u64,
    pub fs_limit: u32,
    pub gs_limit: u32,
    pub reserved: [u8; SGX_TCS_RESERVED_SIZE],
    pub __packed: },
//
// struct sgx_pageinfo - an enclave page descriptor
// @addr:	address of the enclave page
// @contents:	pointer to the page contents
// @metadata:	pointer either to a SECINFO or PCMD instance
// @secs:	address of the SECS page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_pageinfo {
    pub addr: u64,
    pub contents: u64,
    pub metadata: u64,
    pub secs: u64,
    pub __aligned(32): } __packed,
//
// enum sgx_page_type - bits in the SECINFO flags defining the page type
// @SGX_PAGE_TYPE_SECS:	a SECS page
// @SGX_PAGE_TYPE_TCS:	a TCS page
// @SGX_PAGE_TYPE_REG:	a regular page
// @SGX_PAGE_TYPE_VA:	a VA page
// @SGX_PAGE_TYPE_TRIM:	a page in trimmed state
//
// Make sure when making changes to this enum that its values can still fit
// in the bitfield within &struct sgx_encl_page
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_page_type {
    SGX_PAGE_TYPE_SECS,
    SGX_PAGE_TYPE_TCS,
    SGX_PAGE_TYPE_REG,
    SGX_PAGE_TYPE_VA,
    SGX_PAGE_TYPE_TRIM,
}

pub const SGX_NR_PAGE_TYPES: c_int = 5;

//
// enum sgx_secinfo_flags - the flags field in &struct sgx_secinfo
// @SGX_SECINFO_R:	allow read
// @SGX_SECINFO_W:	allow write
// @SGX_SECINFO_X:	allow execution
// @SGX_SECINFO_SECS:	a SECS page
// @SGX_SECINFO_TCS:	a TCS page
// @SGX_SECINFO_REG:	a regular page
// @SGX_SECINFO_VA:	a VA page
// @SGX_SECINFO_TRIM:	a page in trimmed state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_secinfo_flags {
    SGX_SECINFO_R			= BIT(0),
    SGX_SECINFO_W			= BIT(1),
    SGX_SECINFO_X			= BIT(2),
    SGX_SECINFO_SECS		= (SGX_PAGE_TYPE_SECS << 8),
    SGX_SECINFO_TCS			= (SGX_PAGE_TYPE_TCS << 8),
    SGX_SECINFO_REG			= (SGX_PAGE_TYPE_REG << 8),
    SGX_SECINFO_VA			= (SGX_PAGE_TYPE_VA << 8),
    SGX_SECINFO_TRIM		= (SGX_PAGE_TYPE_TRIM << 8),
}

//
// struct sgx_secinfo - describes attributes of an EPC page
// @flags:	permissions and type
//
// Used together with ENCLS leaves that add or modify an EPC page to an
// enclave to define page permissions and type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_secinfo {
    pub flags: u64,
    pub reserved: [u8; 56],
    pub __aligned(64): } __packed,
pub const SGX_PCMD_RESERVED_SIZE: c_int = 40;
//
// struct sgx_pcmd - Paging Crypto Metadata (PCMD)
// @enclave_id:	enclave identifier
// @mac:	MAC over PCMD, page contents and isvsvn
//
// PCMD is stored for every swapped page to the regular memory. When ELDU loads
// the page back it recalculates the MAC by using a isvsvn number stored in a
// VA page. Together these two structures bring integrity and rollback
// protection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_pcmd {
    pub secinfo: sgx_secinfo,
    pub enclave_id: u64,
    pub reserved: [u8; SGX_PCMD_RESERVED_SIZE],
    pub mac: [u8; 16],
    pub __aligned(128): } __packed,
pub const SGX_SIGSTRUCT_RESERVED1_SIZE: c_int = 84;
pub const SGX_SIGSTRUCT_RESERVED2_SIZE: c_int = 20;
pub const SGX_SIGSTRUCT_RESERVED3_SIZE: c_int = 32;
pub const SGX_SIGSTRUCT_RESERVED4_SIZE: c_int = 12;
//
// struct sgx_sigstruct_header -  defines author of the enclave
// @header1:		constant byte string
// @vendor:		must be either 0x0000 or 0x8086
// @date:		YYYYMMDD in BCD
// @header2:		constant byte string
// @swdefined:		software defined value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_sigstruct_header {
    pub header1: [u64; 2],
    pub vendor: u32,
    pub date: u32,
    pub header2: [u64; 2],
    pub swdefined: u32,
    pub reserved1: [u8; 84],
    pub __packed: },
//
// struct sgx_sigstruct_body - defines contents of the enclave
// @miscselect:		additional information stored to an SSA frame
// @misc_mask:		required miscselect in SECS
// @attributes:		attributes for enclave
// @xfrm:		XSave-Feature Request Mask (subset of XCR0)
// @attributes_mask:	required attributes in SECS
// @xfrm_mask:		required XFRM in SECS
// @mrenclave:		SHA256-hash of the enclave contents
// @isvprodid:		a user-defined value that is used in key derivation
// @isvsvn:		a user-defined value that is used in key derivation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_sigstruct_body {
    pub miscselect: u32,
    pub misc_mask: u32,
    pub reserved2: [u8; 20],
    pub attributes: u64,
    pub xfrm: u64,
    pub attributes_mask: u64,
    pub xfrm_mask: u64,
    pub mrenclave: [u8; 32],
    pub reserved3: [u8; 32],
    pub isvprodid: u16,
    pub isvsvn: u16,
    pub __packed: },
//
// struct sgx_sigstruct - an enclave signature
// @header:		defines author of the enclave
// @modulus:		the modulus of the public key
// @exponent:		the exponent of the public key
// @signature:		the signature calculated over the fields except modulus,
// @body:		defines contents of the enclave
// @q1:			a value used in RSA signature verification
// @q2:			a value used in RSA signature verification
//
// Header and body are the parts that are actual signed. The remaining fields
// define the signature of the enclave.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_sigstruct {
    pub header: sgx_sigstruct_header,
    pub modulus: [u8; SGX_MODULUS_SIZE],
    pub exponent: u32,
    pub signature: [u8; SGX_MODULUS_SIZE],
    pub body: sgx_sigstruct_body,
    pub reserved4: [u8; 12],
    pub q1: [u8; SGX_MODULUS_SIZE],
    pub q2: [u8; SGX_MODULUS_SIZE],
    pub __packed: },
pub const SGX_LAUNCH_TOKEN_SIZE: c_int = 304;
//
// Do not put any hardware-defined SGX structure representations below this
// comment!
//

    pub trapnr): *mut c_int,
    pub trapnr): *mut *mut *mut void __user secs, u64 lepubkeyhash, int,

    pub attribute_fd): c_uint,
