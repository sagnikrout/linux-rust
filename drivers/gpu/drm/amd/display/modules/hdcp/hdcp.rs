//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/hdcp/hdcp.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_trans_input_result {
    UNKNOWN = 0,
    PASS,
    FAIL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_transition_input_hdcp1 {
    pub bksv_read: u8,
    pub bksv_validation: u8,
    pub create_session: u8,
    pub an_write: u8,
    pub aksv_write: u8,
    pub ainfo_write: u8,
    pub bcaps_read: u8,
    pub r0p_read: u8,
    pub rx_validation: u8,
    pub encryption: u8,
    pub link_maintenance: u8,
    pub ready_check: u8,
    pub bstatus_read: u8,
    pub max_cascade_check: u8,
    pub max_devs_check: u8,
    pub device_count_check: u8,
    pub ksvlist_read: u8,
    pub vp_read: u8,
    pub ksvlist_vp_validation: u8,
    pub hdcp_capable_dp: u8,
    pub binfo_read_dp: u8,
    pub r0p_available_dp: u8,
    pub link_integrity_check: u8,
    pub reauth_request_check: u8,
    pub stream_encryption_dp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_transition_input_hdcp2 {
    pub hdcp2version_read: u8,
    pub hdcp2_capable_check: u8,
    pub create_session: u8,
    pub ake_init_prepare: u8,
    pub ake_init_write: u8,
    pub rxstatus_read: u8,
    pub ake_cert_available: u8,
    pub ake_cert_read: u8,
    pub ake_cert_validation: u8,
    pub stored_km_write: u8,
    pub no_stored_km_write: u8,
    pub h_prime_available: u8,
    pub h_prime_read: u8,
    pub pairing_available: u8,
    pub pairing_info_read: u8,
    pub h_prime_validation: u8,
    pub lc_init_prepare: u8,
    pub lc_init_write: u8,
    pub l_prime_available_poll: u8,
    pub l_prime_read: u8,
    pub l_prime_combo_read: u8,
    pub l_prime_validation: u8,
    pub eks_prepare: u8,
    pub eks_write: u8,
    pub enable_encryption: u8,
    pub reauth_request_check: u8,
    pub rx_id_list_read: u8,
    pub device_count_check: u8,
    pub rx_id_list_validation: u8,
    pub repeater_auth_ack_write: u8,
    pub prepare_stream_manage: u8,
    pub stream_manage_write: u8,
    pub stream_ready_available: u8,
    pub stream_ready_read: u8,
    pub stream_ready_validation: u8,
    pub rx_caps_read_dp: u8,
    pub content_stream_type_write: u8,
    pub link_integrity_check_dp: u8,
    pub stream_encryption_dp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mod_hdcp_transition_input {
    pub hdcp1: mod_hdcp_transition_input_hdcp1,
    pub hdcp2: mod_hdcp_transition_input_hdcp2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_message_hdcp1 {
    pub an: [u8; 8],
    pub aksv: [u8; 5],
    pub ainfo: u8,
    pub bksv: [u8; 5],
    pub r0p: u16,
    pub bcaps: u8,
    pub bstatus: u16,
    pub ksvlist: [u8; 635],
    pub ksvlist_size: u16,
    pub vp: [u8; 20],
    pub binfo_dp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_message_hdcp2 {
    pub hdcp2version_hdmi: u8,
    pub rxcaps_dp: [u8; 3],
    pub rxstatus: [u8; 2],
    pub ake_init: [u8; 12],
    pub ake_cert: [u8; 534],
    pub ake_no_stored_km: [u8; 129],
    pub ake_stored_km: [u8; 33],
    pub ake_h_prime: [u8; 33],
    pub ake_pairing_info: [u8; 17],
    pub lc_init: [u8; 9],
    pub lc_l_prime: [u8; 33],
    pub ske_eks: [u8; 25],
    pub 31: *mut *mut uint8_t rx_id_list[177]; // 22 + 5,
    pub rx_id_list_size: u16,
    pub repeater_auth_ack: [u8; 17],
    pub 31: *mut *mut uint8_t repeater_auth_stream_manage[68]; // 6 + 2,
    pub stream_manage_size: u16,
    pub repeater_auth_stream_ready: [u8; 33],
    pub rxstatus_dp: u8,
    pub content_stream_type_dp: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mod_hdcp_message {
    pub hdcp1: mod_hdcp_message_hdcp1,
    pub hdcp2: mod_hdcp_message_hdcp2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_auth_counters {
    pub stream_management_retry_count: u8,
}

// contains values per connection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_connection {
    pub link: mod_hdcp_link,
    pub is_repeater: u8,
    pub is_km_stored: u8,
    pub is_hdcp1_revoked: u8,
    pub is_hdcp2_revoked: u8,
    pub trace: mod_hdcp_trace,
    pub hdcp1_retry_count: u8,
    pub hdcp2_retry_count: u8,
}

// contains values per authentication cycle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_authentication {
    pub id: u32,
    pub msg: mod_hdcp_message,
    pub trans_input: mod_hdcp_transition_input,
    pub count: mod_hdcp_auth_counters,
}

// contains values per state change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_state {
    pub id: u8,
    pub stay_count: u32,
}

// per event in a state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_event_context {
    pub event: mod_hdcp_event,
    pub rx_id_list_ready: u8,
    pub unexpected_event: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp {
// per link
    pub config: mod_hdcp_config,
// per connection
    pub connection: mod_hdcp_connection,
// per displays
    pub displays: [mod_hdcp_display; MAX_NUM_OF_DISPLAYS],
// per authentication attempt
    pub auth: mod_hdcp_authentication,
// per state in an authentication
    pub state: mod_hdcp_state,
// reserved memory buffer
    pub buf: [u8; 2025],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_initial_state_id {
    HDCP_UNINITIALIZED = 0x0,
    HDCP_INITIAL_STATE_START = HDCP_UNINITIALIZED,
    HDCP_INITIALIZED,
    HDCP_CP_NOT_DESIRED,
    HDCP_INITIAL_STATE_END = HDCP_CP_NOT_DESIRED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_hdcp1_state_id {
    HDCP1_STATE_START = HDCP_INITIAL_STATE_END,
    H1_A0_WAIT_FOR_ACTIVE_RX,
    H1_A1_EXCHANGE_KSVS,
    H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER,
    H1_A45_AUTHENTICATED,
    H1_A8_WAIT_FOR_READY,
    H1_A9_READ_KSV_LIST,
    HDCP1_STATE_END = H1_A9_READ_KSV_LIST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_hdcp1_dp_state_id {
    HDCP1_DP_STATE_START = HDCP1_STATE_END,
    D1_A0_DETERMINE_RX_HDCP_CAPABLE,
    D1_A1_EXCHANGE_KSVS,
    D1_A23_WAIT_FOR_R0_PRIME,
    D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER,
    D1_A4_AUTHENTICATED,
    D1_A6_WAIT_FOR_READY,
    D1_A7_READ_KSV_LIST,
    HDCP1_DP_STATE_END = D1_A7_READ_KSV_LIST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_hdcp2_state_id {
    HDCP2_STATE_START = HDCP1_DP_STATE_END,
    H2_A0_KNOWN_HDCP2_CAPABLE_RX,
    H2_A1_SEND_AKE_INIT,
    H2_A1_VALIDATE_AKE_CERT,
    H2_A1_SEND_NO_STORED_KM,
    H2_A1_READ_H_PRIME,
    H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME,
    H2_A1_SEND_STORED_KM,
    H2_A1_VALIDATE_H_PRIME,
    H2_A2_LOCALITY_CHECK,
    H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER,
    H2_ENABLE_ENCRYPTION,
    H2_A5_AUTHENTICATED,
    H2_A6_WAIT_FOR_RX_ID_LIST,
    H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK,
    H2_A9_SEND_STREAM_MANAGEMENT,
    H2_A9_VALIDATE_STREAM_READY,
    HDCP2_STATE_END = H2_A9_VALIDATE_STREAM_READY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_hdcp_hdcp2_dp_state_id {
    HDCP2_DP_STATE_START = HDCP2_STATE_END,
    D2_A0_DETERMINE_RX_HDCP_CAPABLE,
    D2_A1_SEND_AKE_INIT,
    D2_A1_VALIDATE_AKE_CERT,
    D2_A1_SEND_NO_STORED_KM,
    D2_A1_READ_H_PRIME,
    D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME,
    D2_A1_SEND_STORED_KM,
    D2_A1_VALIDATE_H_PRIME,
    D2_A2_LOCALITY_CHECK,
    D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER,
    D2_SEND_CONTENT_STREAM_TYPE,
    D2_ENABLE_ENCRYPTION,
    D2_A5_AUTHENTICATED,
    D2_A6_WAIT_FOR_RX_ID_LIST,
    D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK,
    D2_A9_SEND_STREAM_MANAGEMENT,
    D2_A9_VALIDATE_STREAM_READY,
    HDCP2_DP_STATE_END = D2_A9_VALIDATE_STREAM_READY,
    HDCP_STATE_END = HDCP2_DP_STATE_END,
}

// hdcp1 executions and transitions
extern "C" {
    pub fn mod_hdcp_status(hdcp: *mut *mut mod_hdcp_action)(struct mod_hdcp) -> typedef enum;
}
// hdcp2 executions and transitions
// log functions
extern "C" {
    pub fn mod_hdcp_log_ddc_trace(hdcp: *mut mod_hdcp);
}
// TODO: add adjustment log
// psp functions
extern "C" {
    pub fn mod_hdcp_hdcp1_create_session(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp1_destroy_session(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp1_validate_rx(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp1_enable_encryption(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp1_validate_ksvlist_vp(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp1_link_maintenance(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_create_session(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_destroy_session(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_prepare_ake_init(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_validate_ake_cert(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_validate_h_prime(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_prepare_lc_init(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_validate_l_prime(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_prepare_eks(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_enable_encryption(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_hdcp2_validate_rx_id_list(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
// ddc functions
extern "C" {
    pub fn mod_hdcp_read_bksv(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_bcaps(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_bstatus(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_r0p(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_ksvlist(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_vp(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_binfo(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_aksv(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_ainfo(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_an(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_hdcp2version(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_rxcaps(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_rxstatus(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_ake_cert(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_h_prime(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_pairing_info(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_l_prime(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_rx_id_list(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_read_stream_ready(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_ake_init(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_no_stored_km(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_stored_km(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_lc_init(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_eks(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_repeater_auth_ack(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_stream_manage(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_content_type(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_clear_cp_irq_status(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
extern "C" {
    pub fn mod_hdcp_write_poll_read_lc_fw(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
}
// hdcp version helpers
// hdcp state helpers
// callback timer should be reset per state
// transition operation helpers
// status = MOD_HDCP_STATUS_RESET_NEEDED;
// connection topology helpers
