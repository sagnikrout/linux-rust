//! Automatically rewritten from C Header to Rust Module
//! Source: net/sunrpc/auth_gss/gss_rpc_xdr.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// GSS Proxy upcall module
//
// Copyright (C) 2012 Simo Sorce <simo@redhat.com>
//

pub type gssx_buffer = xdr_netobj;
pub type utf8string = xdr_netobj;
pub type gssx_OID = xdr_netobj;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gssx_cred_usage {
    GSSX_C_INITIATE = 1,
    GSSX_C_ACCEPT = 2,
    GSSX_C_BOTH = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_option {
    pub option: gssx_buffer,
    pub value: gssx_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_option_array {
    pub count: u32,
    pub data: *mut gssx_option,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_status {
    pub major_status: u64,
    pub mech: gssx_OID,
    pub minor_status: u64,
    pub major_status_string: utf8string,
    pub minor_status_string: utf8string,
    pub server_ctx: gssx_buffer,
    pub options: gssx_option_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_call_ctx {
    pub locale: utf8string,
    pub server_ctx: gssx_buffer,
    pub options: gssx_option_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_name_attr {
    pub attr: gssx_buffer,
    pub value: gssx_buffer,
    pub extensions: gssx_option_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_name_attr_array {
    pub count: u32,
    pub data: *mut gssx_name_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_name {
    pub display_name: gssx_buffer,
}

pub type gssx_name = gssx_name;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_cred_element {
    pub MN: gssx_name,
    pub mech: gssx_OID,
    pub cred_usage: u32,
    pub initiator_time_rec: u64,
    pub acceptor_time_rec: u64,
    pub options: gssx_option_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_cred_element_array {
    pub count: u32,
    pub data: *mut gssx_cred_element,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_cred {
    pub desired_name: gssx_name,
    pub elements: gssx_cred_element_array,
    pub cred_handle_reference: gssx_buffer,
    pub needs_release: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_ctx {
    pub exported_context_token: gssx_buffer,
    pub state: gssx_buffer,
    pub need_release: u32,
    pub mech: gssx_OID,
    pub src_name: gssx_name,
    pub targ_name: gssx_name,
    pub lifetime: u64,
    pub ctx_flags: u64,
    pub locally_initiated: u32,
    pub open: u32,
    pub options: gssx_option_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_cb {
    pub initiator_addrtype: u64,
    pub initiator_address: gssx_buffer,
    pub acceptor_addrtype: u64,
    pub acceptor_address: gssx_buffer,
    pub application_data: gssx_buffer,
}

// This structure is not defined in the protocol.
// It is used in the kernel to carry around a big buffer
// as a set of pages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssp_in_token {
    pub /: *mut *mut *mut *mut page pages; / Array of contiguous pages,
    pub /: *mut *mut unsigned int page_base; / Start of page data,
    pub /: *mut *mut unsigned int page_len; / Length of page data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_arg_accept_sec_context {
    pub call_ctx: gssx_call_ctx,
    pub context_handle: *mut gssx_ctx,
    pub cred_handle: *mut gssx_cred,
    pub input_token: gssp_in_token,
    pub input_cb: *mut gssx_cb,
    pub ret_deleg_cred: u32,
    pub options: gssx_option_array,
    pub pages: *mut page,
    pub npages: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gssx_res_accept_sec_context {
    pub status: gssx_status,
    pub context_handle: *mut gssx_ctx,
    pub output_token: *mut gssx_buffer,
// struct gssx_cred *delegated_cred_handle; not used in kernel
    pub options: gssx_option_array,
}

// non implemented calls are set to 0 size
pub const GSSX_ARG_indicate_mechs_sz: c_int = 0;
pub const GSSX_RES_indicate_mechs_sz: c_int = 0;
pub const GSSX_ARG_get_call_context_sz: c_int = 0;
pub const GSSX_RES_get_call_context_sz: c_int = 0;
pub const GSSX_ARG_import_and_canon_name_sz: c_int = 0;
pub const GSSX_RES_import_and_canon_name_sz: c_int = 0;
pub const GSSX_ARG_export_cred_sz: c_int = 0;
pub const GSSX_RES_export_cred_sz: c_int = 0;
pub const GSSX_ARG_import_cred_sz: c_int = 0;
pub const GSSX_RES_import_cred_sz: c_int = 0;
pub const GSSX_ARG_acquire_cred_sz: c_int = 0;
pub const GSSX_RES_acquire_cred_sz: c_int = 0;
pub const GSSX_ARG_store_cred_sz: c_int = 0;
pub const GSSX_RES_store_cred_sz: c_int = 0;
pub const GSSX_ARG_init_sec_context_sz: c_int = 0;
pub const GSSX_RES_init_sec_context_sz: c_int = 0;

// somewhat arbitrary numbers but large enough (we ignore some of the data
// sent down, but it is part of the protocol so we need enough space to take
// it in)

pub const GSSX_max_output_handle_sz: c_int = 128;
pub const GSSX_max_oid_sz: c_int = 16;
pub const GSSX_max_princ_sz: c_int = 256;

pub const GSSX_max_output_token_sz: c_int = 1024;
// grouplist not included; we allocate separate pages for that:

pub const GSSX_ARG_release_handle_sz: c_int = 0;
pub const GSSX_RES_release_handle_sz: c_int = 0;
pub const GSSX_ARG_get_mic_sz: c_int = 0;
pub const GSSX_RES_get_mic_sz: c_int = 0;
pub const GSSX_ARG_verify_sz: c_int = 0;
pub const GSSX_RES_verify_sz: c_int = 0;
pub const GSSX_ARG_wrap_sz: c_int = 0;
pub const GSSX_RES_wrap_sz: c_int = 0;
pub const GSSX_ARG_unwrap_sz: c_int = 0;
pub const GSSX_RES_unwrap_sz: c_int = 0;
pub const GSSX_ARG_wrap_size_limit_sz: c_int = 0;
pub const GSSX_RES_wrap_size_limit_sz: c_int = 0;
