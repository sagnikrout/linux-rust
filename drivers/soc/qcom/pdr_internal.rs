//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/qcom/pdr_internal.h
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

pub const SERVREG_REGISTER_LISTENER_REQ: c_uint = 0x20;
pub const SERVREG_GET_DOMAIN_LIST_REQ: c_uint = 0x21;
pub const SERVREG_STATE_UPDATED_IND_ID: c_uint = 0x22;
pub const SERVREG_SET_ACK_REQ: c_uint = 0x23;
pub const SERVREG_RESTART_PD_REQ: c_uint = 0x24;
pub const SERVREG_LOC_PFR_REQ: c_uint = 0x24;
pub const SERVREG_DOMAIN_LIST_LENGTH: c_int = 32;
pub const SERVREG_RESTART_PD_REQ_MAX_LEN: c_int = 67;
pub const SERVREG_REGISTER_LISTENER_REQ_LEN: c_int = 71;
pub const SERVREG_SET_ACK_REQ_LEN: c_int = 72;
pub const SERVREG_GET_DOMAIN_LIST_REQ_MAX_LEN: c_int = 74;
pub const SERVREG_STATE_UPDATED_IND_MAX_LEN: c_int = 79;
pub const SERVREG_GET_DOMAIN_LIST_RESP_MAX_LEN: c_int = 2389;
pub const SERVREG_LOC_PFR_RESP_MAX_LEN: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_location_entry {
    pub 1]: char name[SERVREG_NAME_LENGTH +,
    pub service_data_valid: u8,
    pub service_data: u32,
    pub instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_get_domain_list_req {
    pub 1]: char service_name[SERVREG_NAME_LENGTH +,
    pub domain_offset_valid: u8,
    pub domain_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_get_domain_list_resp {
    pub resp: qmi_response_type_v01,
    pub total_domains_valid: u8,
    pub total_domains: u16,
    pub db_rev_count_valid: u8,
    pub db_rev_count: u16,
    pub domain_list_valid: u8,
    pub domain_list_len: u32,
    pub domain_list: [servreg_location_entry; SERVREG_DOMAIN_LIST_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_register_listener_req {
    pub enable: u8,
    pub 1]: char service_path[SERVREG_NAME_LENGTH +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_register_listener_resp {
    pub resp: qmi_response_type_v01,
    pub curr_state_valid: u8,
    pub curr_state: servreg_service_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_restart_pd_req {
    pub 1]: char service_path[SERVREG_NAME_LENGTH +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_restart_pd_resp {
    pub resp: qmi_response_type_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_state_updated_ind {
    pub curr_state: servreg_service_state,
    pub 1]: char service_path[SERVREG_NAME_LENGTH +,
    pub transaction_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_set_ack_req {
    pub 1]: char service_path[SERVREG_NAME_LENGTH +,
    pub transaction_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_set_ack_resp {
    pub resp: qmi_response_type_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_loc_pfr_req {
    pub 1]: char service[SERVREG_NAME_LENGTH +,
    pub 1]: char reason[SERVREG_PFR_LENGTH +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servreg_loc_pfr_resp {
    pub rsp: qmi_response_type_v01,
}
