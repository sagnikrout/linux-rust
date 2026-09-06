//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_tables_offload.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_offload_reg_flags {
    NFT_OFFLOAD_F_NETWORK2HOST	= (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_offload_reg {
    pub key: u32,
    pub len: u32,
    pub base_offset: u32,
    pub offset: u32,
    pub flags: u32,
    pub data: nft_data,
    pub mask: nft_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_offload_dep_type {
    NFT_OFFLOAD_DEP_UNSPEC	= 0,
    NFT_OFFLOAD_DEP_NETWORK,
    NFT_OFFLOAD_DEP_TRANSPORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_offload_ctx {
    pub type: nft_offload_dep_type,
    pub l3num: __be16,
    pub protonum: u8,
    pub dep: },
    pub num_actions: c_uint,
    pub net: *mut net,
    pub 1]: nft_offload_reg regs[NFT_REG32_15 +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_flow_key {
    pub basic: flow_dissector_key_basic,
    pub control: flow_dissector_key_control,
    pub ipv4: flow_dissector_key_ipv4_addrs,
    pub ipv6: flow_dissector_key_ipv6_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_flow_match {
    pub dissector: flow_dissector,
    pub key: nft_flow_key,
    pub mask: nft_flow_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_flow_rule {
    pub proto: __be16,
    pub match: nft_flow_match,
    pub rule: *mut flow_rule,
}

extern "C" {
    pub fn nft_flow_rule_stats(chain: *const nft_chain, rule: *const nft_rule) -> c_int;
}
extern "C" {
    pub fn nft_flow_rule_destroy(flow: *mut nft_flow_rule);
}
extern "C" {
    pub fn nft_flow_rule_offload_commit(net: *mut net) -> c_int;
}

extern "C" {
    pub fn nft_chain_offload_support(basechain: *const nft_base_chain) -> bool;
}
extern "C" {
    pub fn nft_offload_init() -> c_int;
}
extern "C" {
    pub fn nft_offload_exit();
}
