//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_tpg.h
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
    pub fn iscsit_load_discovery_tpg() -> c_int;
}
extern "C" {
    pub fn iscsit_release_discovery_tpg();
}
extern "C" {
    pub fn iscsit_get_tpg(: *mut iscsi_portal_group) -> c_int;
}
extern "C" {
    pub fn iscsit_put_tpg(: *mut iscsi_portal_group);
}
extern "C" {
    pub fn iscsit_tpg_add_portal_group(: *mut iscsi_tiqn, : *mut iscsi_portal_group) -> c_int;
}
extern "C" {
    pub fn iscsit_tpg_enable_portal_group(: *mut iscsi_portal_group) -> c_int;
}
extern "C" {
    pub fn iscsit_tpg_disable_portal_group(: *mut iscsi_portal_group, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_authentication(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_login_timeout(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_generate_node_acls(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_default_cmdsn_depth(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_cache_dynamic_acls(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_demo_mode_write_protect(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_prod_mode_write_protect(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_demo_mode_discovery(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_default_erl(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_t10_pi(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_fabric_prot_type(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_tpg_enabled_sendtargets(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_ta_login_keys_workaround(: *mut iscsi_portal_group, _arg: u32) -> c_int;
}
