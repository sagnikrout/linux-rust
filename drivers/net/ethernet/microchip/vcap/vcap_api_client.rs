//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/vcap/vcap_api_client.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2022 Microchip Technology Inc. and its subsidiaries.
// Microchip VCAP API
//

// Client supplied VCAP rule key control part
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_keyfield_ctrl {
    pub /: *mut *mut list_head list; / For insertion into a rule,
    pub key: vcap_key_field,
    pub type: vcap_field_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u1_key {
    pub value: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u32_key {
    pub value: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u48_key {
    pub value: [u8; 6],
    pub mask: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u56_key {
    pub value: [u8; 7],
    pub mask: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u64_key {
    pub value: [u8; 8],
    pub mask: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u72_key {
    pub value: [u8; 9],
    pub mask: [u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u112_key {
    pub value: [u8; 14],
    pub mask: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u128_key {
    pub value: [u8; 16],
    pub mask: [u8; 16],
}

// Client supplied VCAP rule field data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_keyfield_data {
    pub u1: vcap_u1_key,
    pub u32: vcap_u32_key,
    pub u48: vcap_u48_key,
    pub u56: vcap_u56_key,
    pub u64: vcap_u64_key,
    pub u72: vcap_u72_key,
    pub u112: vcap_u112_key,
    pub u128: vcap_u128_key,
}

// Client supplied VCAP rule key (value, mask)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_keyfield {
    pub ctrl: vcap_client_keyfield_ctrl,
    pub data: vcap_client_keyfield_data,
}

// Client supplied VCAP rule action control part
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_actionfield_ctrl {
    pub /: *mut *mut list_head list; / For insertion into a rule,
    pub action: vcap_action_field,
    pub type: vcap_field_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u1_action {
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u32_action {
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u48_action {
    pub value: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u56_action {
    pub value: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u64_action {
    pub value: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u72_action {
    pub value: [u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u112_action {
    pub value: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_u128_action {
    pub value: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_actionfield_data {
    pub u1: vcap_u1_action,
    pub u32: vcap_u32_action,
    pub u48: vcap_u48_action,
    pub u56: vcap_u56_action,
    pub u64: vcap_u64_action,
    pub u72: vcap_u72_action,
    pub u112: vcap_u112_action,
    pub u128: vcap_u128_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_client_actionfield {
    pub ctrl: vcap_client_actionfield_ctrl,
    pub data: vcap_client_actionfield_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_bit {
    VCAP_BIT_ANY,
    VCAP_BIT_0,
    VCAP_BIT_1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_counter {
    pub value: u32,
    pub sticky: bool,
}

// Enable/Disable the VCAP instance lookups
// VCAP rule operations
// Allocate a rule and fill in the basic information
// Free mem of a rule owned by client
extern "C" {
    pub fn vcap_free_rule(rule: *mut vcap_rule);
}
// Validate a rule before adding it to the VCAP
extern "C" {
    pub fn vcap_val_rule(rule: *mut vcap_rule, l3_proto: u16) -> c_int;
}
// Add rule to a VCAP instance
extern "C" {
    pub fn vcap_add_rule(rule: *mut vcap_rule) -> c_int;
}
// Delete rule in a VCAP instance
extern "C" {
    pub fn vcap_del_rule(vctrl: *mut vcap_control, ndev: *mut net_device, id: u32) -> c_int;
}
// Make a full copy of an existing rule with a new rule id
// Get rule from a VCAP instance
// Update existing rule
extern "C" {
    pub fn vcap_mod_rule(rule: *mut vcap_rule) -> c_int;
}
// Update the keyset for the rule
// Update the actionset for the rule
// Set a rule counter id (for certain VCAPs only)
extern "C" {
    pub fn vcap_rule_set_counter_id(rule: *mut vcap_rule, counter_id: u32);
}
// VCAP rule field operations
// Get number of rules in a vcap instance lookup chain id range
extern "C" {
    pub fn vcap_admin_rule_count(admin: *mut vcap_admin, cid: c_int) -> c_int;
}
// VCAP rule counter operations
extern "C" {
    pub fn vcap_rule_set_counter(rule: *mut vcap_rule, ctr: *mut vcap_counter) -> c_int;
}
extern "C" {
    pub fn vcap_rule_get_counter(rule: *mut vcap_rule, ctr: *mut vcap_counter) -> c_int;
}
// VCAP lookup operations
// Convert a chain id to a VCAP lookup index
extern "C" {
    pub fn vcap_chain_id_to_lookup(admin: *mut vcap_admin, cur_cid: c_int) -> c_int;
}
// Lookup a vcap instance using chain id
// Find information on a key field in a rule
// Find a rule id with a provided cookie
extern "C" {
    pub fn vcap_lookup_rule_by_cookie(vctrl: *mut vcap_control, cookie: u64) -> c_int;
}
// Calculate the value used for chaining VCAP rules
extern "C" {
    pub fn vcap_chain_offset(vctrl: *mut vcap_control, from_cid: c_int, to_cid: c_int) -> c_int;
}
// Is the next chain id in the following lookup, possible in another VCAP
extern "C" {
    pub fn vcap_is_next_lookup(vctrl: *mut vcap_control, cur_cid: c_int, next_cid: c_int) -> bool;
}
// Is this chain id the last lookup of all VCAPs
extern "C" {
    pub fn vcap_is_last_chain(vctrl: *mut vcap_control, cid: c_int, ingress: bool) -> bool;
}
// Match a list of keys against the keysets available in a vcap type
// Return the keyset information for the keyset
// Copy to host byte order
extern "C" {
    pub fn vcap_netbytes_copy(dst: *mut u8, src: *mut u8, count: c_int);
}
// Convert validation error code into tc extack error message
extern "C" {
    pub fn vcap_set_tc_exterr(fco: *mut flow_cls_offload, vrule: *mut vcap_rule);
}
// Cleanup a VCAP instance
extern "C" {
    pub fn vcap_del_rules(vctrl: *mut vcap_control, admin: *mut vcap_admin) -> c_int;
}
// Add a keyset to a keyset list
// Drop keys in a keylist and any keys that are not supported by the keyset
// map keyset id to a string with the keyset name
// map key field id to a string with the key name
// Modify a 32 bit key field with value and mask in the rule
// Modify a 32 bit action field with value in the rule
// Get a 32 bit key field value and mask from the rule
// Remove a key field with value and mask in the rule
extern "C" {
    pub fn vcap_rule_rem_key(rule: *mut vcap_rule, key: vcap_key_field) -> c_int;
}
// Select the keyset from the list that results in the smallest rule size
