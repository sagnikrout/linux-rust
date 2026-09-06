//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_tx.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_htt_wbm_tx_status {
    pub acked: bool,
    pub ack_rssi: i8,
}

extern "C" {
    pub fn ath12k_dp_tx_put_bank_profile(dp: *mut ath12k_dp, bank_id: u8);
}
extern "C" {
    pub fn ath12k_dp_tx_encap_nwifi(skb: *mut sk_buff);
}
extern "C" {
    pub fn ath12k_dp_tx_get_tid(skb: *mut sk_buff) -> u8;
}
extern "C" {
    pub fn ath12k_dp_tx_crypto_iv_len(enc_type: hal_encrypt_type) -> u8;
}
extern "C" {
    pub fn ath12k_dp_tx_crypto_icv_len(enc_type: hal_encrypt_type) -> u8;
}
extern "C" {
    pub fn ath12k_dp_tx_align_payload(dp: *mut ath12k_dp, pskb: *mut sk_buff) -> c_int;
}
