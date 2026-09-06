//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/vcap/vcap_api_private.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_rule_state {
    VCAP_RS_PERMANENT, /* the rule is always stored in HW */
    VCAP_RS_ENABLED, /* enabled in HW but can be disabled */
    VCAP_RS_DISABLED, /* disabled (stored in SW) and can be enabled */
}

// Private VCAP API rule data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_rule_internal {
    pub /: *mut *mut vcap_rule data; / provided by the client,
    pub /: *mut *mut list_head list; / the vcap admin list of rules,
    pub /: *mut *mut *mut vcap_admin admin; / vcap hw instance,
    pub /: *mut *mut *mut net_device ndev; / the interface that the rule applies to,
    pub /: *mut *mut *mut vcap_control vctrl; / the client control,
    pub /: *mut *mut u32 sort_key; / defines the position in the VCAP,
    pub /: *mut *mut int keyset_sw; / subwords in a keyset,
    pub /: *mut *mut int actionset_sw; / subwords in an actionset,
    pub /: *mut *mut int keyset_sw_regs; / registers in a subword in an keyset,
    pub /: *mut *mut int actionset_sw_regs; / registers in a subword in an actionset,
    pub /: *mut *mut int size; / the size of the rule: max(entry, action),
    pub /: *mut *mut u32 addr; / address in the VCAP at insertion,
    pub /: *mut *mut u32 counter_id; / counter id (if a dedicated counter is available),
    pub /: *mut *mut vcap_counter counter; / last read counter value,
    pub /: *mut *mut vcap_rule_state state; / rule storage state,
}

// Bit iterator for the VCAP cache streams
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_stream_iter {
    pub /: *mut *mut u32 offset; / bit offset from the stream start,
    pub /: *mut *mut u32 sw_width; / subword width in bits,
    pub /: *mut *mut u32 regs_per_sw; / registers per subword,
    pub /: *mut *mut u32 reg_idx; / current register index,
    pub /: *mut *mut u32 reg_bitpos; / bit offset in current register,
    pub /: *const *const *const vcap_typegroup tg; / current typegroup,
}

// Check that the control has a valid set of callbacks
extern "C" {
    pub fn vcap_api_check(ctrl: *mut vcap_control) -> c_int;
}
// Serialize access to the vcap instances of a control
extern "C" {
    pub fn vcap_lock(admin: *mut vcap_admin);
}
extern "C" {
    pub fn vcap_unlock(admin: *mut vcap_admin);
}
// Erase the VCAP cache area used or encoding and decoding
extern "C" {
    pub fn vcap_erase_cache(ri: *mut vcap_rule_internal);
}
// Iterator functionality
extern "C" {
    pub fn vcap_iter_next(itr: *mut vcap_stream_iter);
}
extern "C" {
    pub fn vcap_iter_update(itr: *mut vcap_stream_iter);
}
// Keyset and keyfield functionality
// Return the number of keyfields in the keyset
// Return the typegroup table for the matching keyset (using subword size)
// Return the list of keyfields for the keyset
// Actionset and actionfield functionality
// Return the actionset information for the actionset
// Return the number of actionfields in the actionset
// Return the typegroup table for the matching actionset (using subword size)
// Return the list of actionfields for the actionset
// Map actionset id to a string with the actionset name
// Map key field id to a string with the key name
// Read key data from a VCAP address and discover if there are any rule keysets
// here
//
// Verify that the typegroup information, subword count, keyset and type id
// are in sync and correct, return the list of matching keysets
//
// Get the keysets that matches the rule key type/mask
// Decode a rule from the VCAP cache and return a copy
