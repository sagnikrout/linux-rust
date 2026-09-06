//! Automatically rewritten from C to Rust
//! Source: net/nfc/hci/llc_nop.c
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
// nop (passthrough) Link Layer Control
//
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_nop {
    pub hdev: *mut nfc_hci_dev,
    pub xmit_to_drv: xmit_to_drv_t,
    pub rcv_to_hci: rcv_to_hci_t,
    pub tx_headroom: c_int,
    pub tx_tailroom: c_int,
    pub llc_failure: llc_failure_t,
}

    static void *llc_nop_init(struct nfc_hci_dev *hdev, xmit_to_drv_t xmit_to_drv,
    rcv_to_hci_t rcv_to_hci, int tx_headroom,
    int tx_tailroom, int *rx_headroom, int *rx_tailroom,
    llc_failure_t llc_failure)
    {
    struct llc_nop *llc_nop;
// rx_headroom = 0;
// rx_tailroom = 0;
    llc_nop = kzalloc_obj(struct llc_nop);
    if (llc_nop == core::ptr::null_mut())
    return core::ptr::null_mut();
    llc_nop.hdev = hdev;
    llc_nop.xmit_to_drv = xmit_to_drv;
    llc_nop.rcv_to_hci = rcv_to_hci;
    llc_nop.tx_headroom = tx_headroom;
    llc_nop.tx_tailroom = tx_tailroom;
    llc_nop.llc_failure = llc_failure;
    return llc_nop;
    }
#[no_mangle]
unsafe extern "C" fn llc_nop_deinit(llc: *mut nfc_llc) {
    static void llc_nop_deinit(struct nfc_llc *llc)
    {
    kfree(nfc_llc_get_data(llc));
    }
#[no_mangle]
unsafe extern "C" fn llc_nop_start(llc: *mut nfc_llc) -> c_int {
    static int llc_nop_start(struct nfc_llc *llc)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn llc_nop_stop(llc: *mut nfc_llc) -> c_int {
    static int llc_nop_stop(struct nfc_llc *llc)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn llc_nop_rcv_from_drv(llc: *mut nfc_llc, skb: *mut sk_buff) {
    static void llc_nop_rcv_from_drv(struct nfc_llc *llc, struct sk_buff *skb)
    {
    struct llc_nop *llc_nop = nfc_llc_get_data(llc);
    llc_nop.rcv_to_hci(llc_nop.hdev, skb);
    }
#[no_mangle]
unsafe extern "C" fn llc_nop_xmit_from_hci(llc: *mut nfc_llc, skb: *mut sk_buff) -> c_int {
    static int llc_nop_xmit_from_hci(struct nfc_llc *llc, struct sk_buff *skb)
    {
    struct llc_nop *llc_nop = nfc_llc_get_data(llc);
    return llc_nop.xmit_to_drv(llc_nop.hdev, skb);
    }
    static const struct nfc_llc_ops llc_nop_ops = {
    .init = llc_nop_init,
    .deinit = llc_nop_deinit,
    .start = llc_nop_start,
    .stop = llc_nop_stop,
    .rcv_from_drv = llc_nop_rcv_from_drv,
    .xmit_from_hci = llc_nop_xmit_from_hci,
    };
#[no_mangle]
pub unsafe extern "C" fn nfc_llc_nop_register() -> c_int {
    int nfc_llc_nop_register(void)
    {
    return nfc_llc_register(LLC_NOP_NAME, &llc_nop_ops);
    }
