//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/fwsignal.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2012 Broadcom Corporation
//
// enum brcmf_fws_fifo - fifo indices used by dongle firmware.
//
// @BRCMF_FWS_FIFO_FIRST: first fifo, ie. background.
// @BRCMF_FWS_FIFO_AC_BK: fifo for background traffic.
// @BRCMF_FWS_FIFO_AC_BE: fifo for best-effort traffic.
// @BRCMF_FWS_FIFO_AC_VI: fifo for video traffic.
// @BRCMF_FWS_FIFO_AC_VO: fifo for voice traffic.
// @BRCMF_FWS_FIFO_BCMC: fifo for broadcast/multicast (AP only).
// @BRCMF_FWS_FIFO_ATIM: fifo for ATIM (AP only).
// @BRCMF_FWS_FIFO_COUNT: number of fifos.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_fws_fifo {
    BRCMF_FWS_FIFO_FIRST,
    BRCMF_FWS_FIFO_AC_BK = BRCMF_FWS_FIFO_FIRST,
    BRCMF_FWS_FIFO_AC_BE,
    BRCMF_FWS_FIFO_AC_VI,
    BRCMF_FWS_FIFO_AC_VO,
    BRCMF_FWS_FIFO_BCMC,
    BRCMF_FWS_FIFO_ATIM,
    BRCMF_FWS_FIFO_COUNT
}

extern "C" {
    pub fn brcmf_fws_detach(fws: *mut brcmf_fws_info);
}
extern "C" {
    pub fn brcmf_fws_debugfs_create(drvr: *mut brcmf_pub);
}
extern "C" {
    pub fn brcmf_fws_queue_skbs(fws: *mut brcmf_fws_info) -> bool;
}
extern "C" {
    pub fn brcmf_fws_fc_active(fws: *mut brcmf_fws_info) -> bool;
}
extern "C" {
    pub fn brcmf_fws_hdrpull(ifp: *mut brcmf_if, siglen: i16, skb: *mut sk_buff);
}
extern "C" {
    pub fn brcmf_fws_process_skb(ifp: *mut brcmf_if, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn brcmf_fws_reset_interface(ifp: *mut brcmf_if);
}
extern "C" {
    pub fn brcmf_fws_add_interface(ifp: *mut brcmf_if);
}
extern "C" {
    pub fn brcmf_fws_del_interface(ifp: *mut brcmf_if);
}
extern "C" {
    pub fn brcmf_fws_bus_blocked(drvr: *mut brcmf_pub, flow_blocked: bool);
}
extern "C" {
    pub fn brcmf_fws_rxreorder(ifp: *mut brcmf_if, skb: *mut sk_buff);
}
