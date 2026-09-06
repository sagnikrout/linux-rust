//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/scb.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// scb flags
pub const SCB_WMECAP: c_uint = 0x0040;
pub const SCB_HTCAP: c_uint = 0x10000	/* HT (MIMO) capable device */;
pub const SCB_IS40: c_uint = 0x80000	/* 40MHz capable */;
pub const SCB_STBCCAP: c_uint = 0x40000000	/* STBC Capable */;
pub const SCB_MAGIC: c_uint = 0xbeefcafe;
// structure to store per-tid state for the ampdu initiator
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_ampdu_tid_ini {
// tx retry count; indexed by seq modulo
    pub txretry: [u8; AMPDU_TX_BA_MAX_WSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_ampdu {
    pub /: *mut *mut u8 max_pdu; / max pdus allowed in ampdu,
    pub /: *mut *mut u8 release; / # of mpdus released at a time,
    pub /: *mut *mut u32 max_rx_ampdu_bytes; / max ampdu rcv length; 8k, 16k, 32k, 64k,
//
// This could easily be a ini[] pointer and we keep this info in wl
// itself instead of having mac80211 hold it for us. Also could be made
// dynamic per tid instead of static.
//
// initiator info - per tid (NUMPRIO):
    pub ini: [scb_ampdu_tid_ini; AMPDU_MAX_SCB_TID],
}

// station control block - one per remote MAC address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb {
    pub magic: u32,
    pub /: *mut *mut u32 flags; / various bit flags as defined below,
    pub /: *mut *mut u16 seqctl[NUMPRIO]; / seqctl of last received frame (for dups),
    pub /: *mut *mut u16 seqnum[NUMPRIO];/ WME: driver maintained sw seqnum per priority,
    pub /: *mut *mut scb_ampdu scb_ampdu; / AMPDU state including per tid info,
}
