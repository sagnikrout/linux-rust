//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_tables_core.h
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

extern "C" {
    pub fn nf_tables_core_module_init() -> c_int;
}
extern "C" {
    pub fn nf_tables_core_module_exit();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_bitwise_fast_expr {
    pub mask: u32,
    pub xor: u32,
    pub sreg: u8,
    pub dreg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_cmp_fast_expr {
    pub data: u32,
    pub mask: u32,
    pub sreg: u8,
    pub len: u8,
    pub inv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_cmp16_fast_expr {
    pub data: nft_data,
    pub mask: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub inv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_immediate_expr {
    pub data: nft_data,
    pub dreg: u8,
    pub dlen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_ct {
    pub key:8: nft_ct_keys,
    pub dir:8: ip_conntrack_dir,
    pub len: u8,
    pub dreg: u8,
    pub sreg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_payload {
    pub base:8: nft_payload_bases,
    pub offset: u16,
    pub len: u8,
    pub dreg: u8,
}

// called from nft_pipapo_avx2.c
// called from nft_set_pipapo.c
extern "C" {
    pub fn nft_counter_init_seqcount();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_inner_tun_ctx {
    pub cookie: c_ulong,
    pub type: u16,
    pub inner_tunoff: u16,
    pub inner_lloff: u16,
    pub inner_nhoff: u16,
    pub inner_thoff: u16,
    pub llproto: __be16,
    pub l4proto: u8,
    pub flags: u8,
}

extern "C" {
    pub fn nft_payload_inner_offset(pkt: *const nft_pktinfo) -> c_int;
}
