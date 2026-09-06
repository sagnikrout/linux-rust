//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/chsc.h
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
//
// Copyright IBM Corp. 2020
//
// Author(s): Alexandra Winter <wintera@linux.ibm.com>
//
// Interface for Channel Subsystem Call
//

// struct from linux/notifier.h
//
// Operation codes for CHSC PNSO:
// PNSO_OC_NET_BRIDGE_INFO - only addresses that are visible to a bridgeport
// PNSO_OC_NET_ADDR_INFO   - all addresses
//
pub const PNSO_OC_NET_BRIDGE_INFO: c_int = 0;
pub const PNSO_OC_NET_ADDR_INFO: c_int = 3;
//
// struct chsc_pnso_naid_l2 - network address information descriptor
// @nit:  Network interface token
// @addr_lnid: network address and logical network id (VLAN ID)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_pnso_naid_l2 {
    pub nit: u64,
    pub addr_lnid: { u8 mac[6]; u16 lnid; },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_pnso_resume_token {
    pub t1: u64,
    pub t2: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_pnso_naihdr {
    pub resume_token: chsc_pnso_resume_token,
    pub instance: u32,
    pub naids: u8,
    pub reserved: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_pnso_area {
    pub request: chsc_header,
    pub m:1: u8,
    pub ssid:2: u8,
    pub fmt:4: u8,
    pub sch: u16,
    pub cssid: u8,
    pub oc: u8,
    pub resume_token: chsc_pnso_resume_token,
    pub n:1: u32,
    pub reserved: [u32; 3],
    pub response: chsc_header,
    pub naihdr: chsc_pnso_naihdr,
    pub entries: [chsc_pnso_naid_l2; ],
    pub __aligned(PAGE_SIZE): } __packed,
//
// notifier interface - registered notifiers gets called on
// the following events:
// - ap config changed (CHSC_NOTIFY_AP_CFG)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chsc_notify_type {
    CHSC_NOTIFY_AP_CFG = 3,
}

    pub nb): *mut int chsc_notifier_register(struct notifier_block,
    pub nb): *mut int chsc_notifier_unregister(struct notifier_block,
