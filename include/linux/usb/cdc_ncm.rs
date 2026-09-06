//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/cdc_ncm.h
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
// Copyright (C) ST-Ericsson 2010-2012
// Contact: Alexey Orishko <alexey.orishko@stericsson.com>
// Original author: Hans Petter Selasky <hans.petter.selasky@stericsson.com>
//
// USB Host Driver for Network Control Model (NCM)
// http://www.usb.org/developers/devclass_docs/NCM10.zip
//
// The NCM encoding, decoding and initialization logic
// derives from FreeBSD 8.x. if_cdce.c and if_cdcereg.h
//
// This software is available to you under a choice of one of two
// licenses. You may choose this file to be licensed under the terms
// of the GNU General Public License (GPL) Version 2 or the 2-clause
// BSD license listed below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//
pub const CDC_NCM_COMM_ALTSETTING_NCM: c_int = 0;
pub const CDC_NCM_COMM_ALTSETTING_MBIM: c_int = 1;
pub const CDC_NCM_DATA_ALTSETTING_NCM: c_int = 1;
pub const CDC_NCM_DATA_ALTSETTING_MBIM: c_int = 2;
// CDC NCM subclass 3.3.1
pub const USB_CDC_NCM_NDP16_LENGTH_MIN: c_uint = 0x10;
// CDC NCM subclass 3.3.2
pub const USB_CDC_NCM_NDP32_LENGTH_MIN: c_uint = 0x20;
// Maximum NTB length

// Initial NTB length

// Minimum value for MaxDatagramSize, ch. 6.2.9

// Minimum value for MaxDatagramSize, ch. 8.1.3

// Default value for MaxDatagramSize

//
// Maximum amount of datagrams in NCM Datagram Pointer Table, not counting
// the last NULL entry.
//
pub const CDC_NCM_DPT_DATAGRAMS_MAX: c_int = 40;
// Restart the timer, if amount of datagrams is less than given value
pub const CDC_NCM_RESTART_TIMER_DATAGRAM_CNT: c_int = 3;
pub const CDC_NCM_TIMER_PENDING_CNT: c_int = 2;

// Driver flags
pub const CDC_NCM_FLAG_NDP_TO_END: c_uint = 0x02	/* NDP is placed at end of frame */;
pub const CDC_MBIM_FLAG_AVOID_ALTSETTING_TOGGLE: c_uint = 0x04	/* Avoid altsetting toggle during init */;
pub const CDC_NCM_FLAG_PREFER_NTB32: c_uint = 0x08	/* prefer NDP32 over NDP16 */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdc_ncm_ctx {
    pub ncm_parm: usb_cdc_ncm_ntb_parameters,
    pub tx_timer: hrtimer,
    pub bh: tasklet_struct,
    pub dev: *mut usbnet,
    pub func_desc: *const usb_cdc_ncm_desc,
    pub mbim_desc: *const usb_cdc_mbim_desc,
    pub mbim_extended_desc: *const usb_cdc_mbim_extended_desc,
    pub ether_desc: *const usb_cdc_ether_desc,
    pub control: *mut usb_interface,
    pub data: *mut usb_interface,
    pub tx_curr_skb: *mut sk_buff,
    pub tx_rem_skb: *mut sk_buff,
    pub tx_rem_sign: __le32,
    pub mtx: spinlock_t,
    pub stop: core::sync::atomic::AtomicI32,
    pub drvflags: c_int,
    pub timer_interval: u32,
    pub max_ndp_size: u32,
    pub is_ndp16: bool,
    pub filtering_supported: bool,
    pub delayed_ndp16: *mut usb_cdc_ncm_ndp16,
    pub delayed_ndp32: *mut usb_cdc_ncm_ndp32,
}

// statistics
extern "C" {
    pub fn cdc_ncm_select_altsetting(intf: *mut usb_interface) -> u8;
}
extern "C" {
    pub fn cdc_ncm_change_mtu(net: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_bind_common(dev: *mut usbnet, intf: *mut usb_interface, data_altsetting: u8, drvflags: c_int) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_unbind(dev: *mut usbnet, intf: *mut usb_interface);
}
extern "C" {
    pub fn cdc_ncm_rx_verify_nth16(ctx: *mut cdc_ncm_ctx, skb_in: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_rx_verify_ndp16(skb_in: *mut sk_buff, ndpoffset: c_int) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_rx_verify_nth32(ctx: *mut cdc_ncm_ctx, skb_in: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_rx_verify_ndp32(skb_in: *mut sk_buff, ndpoffset: c_int) -> c_int;
}
extern "C" {
    pub fn cdc_ncm_rx_fixup(dev: *mut usbnet, skb_in: *mut sk_buff) -> c_int;
}
