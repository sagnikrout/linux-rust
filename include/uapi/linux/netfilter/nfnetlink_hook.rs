//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_hook.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_msg_types {
    NFNL_MSG_HOOK_GET,
    NFNL_MSG_HOOK_MAX,
}

//
// enum nfnl_hook_attributes - netfilter hook netlink attributes
//
// @NFNLA_HOOK_HOOKNUM: netfilter hook number (NLA_U32)
// @NFNLA_HOOK_PRIORITY: netfilter hook priority (NLA_U32)
// @NFNLA_HOOK_DEV: netdevice name (NLA_STRING)
// @NFNLA_HOOK_FUNCTION_NAME: hook function name (NLA_STRING)
// @NFNLA_HOOK_MODULE_NAME: kernel module that registered this hook (NLA_STRING)
// @NFNLA_HOOK_CHAIN_INFO: basechain hook metadata (NLA_NESTED)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_attributes {
    NFNLA_HOOK_UNSPEC,
    NFNLA_HOOK_HOOKNUM,
    NFNLA_HOOK_PRIORITY,
    NFNLA_HOOK_DEV,
    NFNLA_HOOK_FUNCTION_NAME,
    NFNLA_HOOK_MODULE_NAME,
    NFNLA_HOOK_CHAIN_INFO,
    __NFNLA_HOOK_MAX
}

//
// enum nfnl_hook_chain_info_attributes - chain description
//
// @NFNLA_HOOK_INFO_DESC: nft chain and table name (NLA_NESTED)
// @NFNLA_HOOK_INFO_TYPE: chain type (enum nfnl_hook_chaintype) (NLA_U32)
//
// NFNLA_HOOK_INFO_DESC depends on NFNLA_HOOK_INFO_TYPE value:
// NFNL_HOOK_TYPE_NFTABLES: enum nft_table_attributes
// NFNL_HOOK_TYPE_BPF: enum nfnl_hook_bpf_attributes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_chain_info_attributes {
    NFNLA_HOOK_INFO_UNSPEC,
    NFNLA_HOOK_INFO_DESC,
    NFNLA_HOOK_INFO_TYPE,
    __NFNLA_HOOK_INFO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_chain_desc_attributes {
    NFNLA_CHAIN_UNSPEC,
    NFNLA_CHAIN_TABLE,
    NFNLA_CHAIN_FAMILY,
    NFNLA_CHAIN_NAME,
    __NFNLA_CHAIN_MAX,
}

//
// enum nfnl_hook_chaintype - chain type
//
// @NFNL_HOOK_TYPE_NFTABLES: nf_tables base chain
// @NFNL_HOOK_TYPE_BPF: bpf program
// @NFNL_HOOK_TYPE_NFT_FLOWTABLE: nf_tables flowtable
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_chaintype {
    NFNL_HOOK_TYPE_NFTABLES = 0x1,
    NFNL_HOOK_TYPE_BPF,
    NFNL_HOOK_TYPE_NFT_FLOWTABLE,
}

//
// enum nfnl_hook_bpf_attributes - bpf prog description
//
// @NFNLA_HOOK_BPF_ID: bpf program id (NLA_U32)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_hook_bpf_attributes {
    NFNLA_HOOK_BPF_UNSPEC,
    NFNLA_HOOK_BPF_ID,
    __NFNLA_HOOK_BPF_MAX,
}

