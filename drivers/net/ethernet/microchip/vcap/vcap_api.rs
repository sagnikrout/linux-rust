//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/vcap/vcap_api.h
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


// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2022 Microchip Technology Inc. and its subsidiaries.
// Microchip VCAP API
//

// Use the generated API model

// Known users of the VCAP API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_user {
    VCAP_USER_PTP,
    VCAP_USER_MRP,
    VCAP_USER_CFM,
    VCAP_USER_VLAN,
    VCAP_USER_QOS,
    VCAP_USER_VCAP_UTIL,
    VCAP_USER_TC,
    VCAP_USER_TC_EXTRA,

// add new users above here

// used to define VCAP_USER_MAX below
    __VCAP_USER_AFTER_LAST,
    VCAP_USER_MAX = __VCAP_USER_AFTER_LAST - 1,
}

// VCAP information used for displaying data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_statistics {
    pub name: *mut c_char,
    pub count: c_int,
    pub keyfield_set_names: *const *const c_char,
    pub actionfield_set_names: *const *const c_char,
    pub keyfield_names: *const *const c_char,
    pub actionfield_names: *const *const c_char,
}

// VCAP key/action field type, position and width
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_field {
    pub type: u16,
    pub width: u16,
    pub offset: u16,
}

// VCAP keyset or actionset type and width
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_set {
    pub type_id: u8,
    pub sw_per_item: u8,
    pub sw_cnt: u8,
}

// VCAP typegroup position and bitvalue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_typegroup {
    pub offset: u16,
    pub width: u16,
    pub value: u16,
}

// VCAP model data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_info {
    pub /: *mut *mut *mut char name; / user-friendly name,
    pub /: *mut *mut u16 rows; / number of row in instance,
    pub /: *mut *mut u16 sw_count; / maximum subwords used per rule,
    pub /: *mut *mut u16 sw_width; / bits per subword in a keyset,
    pub /: *mut *mut u16 sticky_width; / sticky bits per rule,
    pub /: *mut *mut u16 act_width; / bits per subword in an actionset,
    pub /: *mut *mut u16 default_cnt; / number of default rules,
    pub /: *mut *mut u16 require_cnt_dis; / not used,
    pub /: *mut *mut u16 version; / vcap rtl version,
    pub /: *const *const *const vcap_set keyfield_set; / keysets,
    pub /: *mut *mut int keyfield_set_size; / number of keysets,
    pub /: *const *const *const vcap_set actionfield_set; / actionsets,
    pub /: *mut *mut int actionfield_set_size; / number of actionsets,
// map of keys per keyset
    pub keyfield_set_map: *const vcap_field,
// number of entries in the above map
    pub keyfield_set_map_size: *mut c_int,
// map of actions per actionset
    pub actionfield_set_map: *const vcap_field,
// number of entries in the above map
    pub actionfield_set_map_size: *mut c_int,
// map of keyset typegroups per subword size
    pub keyfield_set_typegroups: *const vcap_typegroup,
// map of actionset typegroups per subword size
    pub actionfield_set_typegroups: *const vcap_typegroup,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_field_type {
    VCAP_FIELD_BIT,
    VCAP_FIELD_U32,
    VCAP_FIELD_U48,
    VCAP_FIELD_U56,
    VCAP_FIELD_U64,
    VCAP_FIELD_U72,
    VCAP_FIELD_U112,
    VCAP_FIELD_U128,
}

// VCAP rule data towards the VCAP cache
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_cache_data {
    pub keystream: *mut u32,
    pub maskstream: *mut u32,
    pub actionstream: *mut u32,
    pub counter: u32,
    pub sticky: bool,
}

// Selects which part of the rule must be updated
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_selection {
    VCAP_SEL_ENTRY = 0x01,
    VCAP_SEL_ACTION = 0x02,
    VCAP_SEL_COUNTER = 0x04,
    VCAP_SEL_ALL = 0xff,
}

// Commands towards the VCAP cache
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_command {
    VCAP_CMD_WRITE = 0,
    VCAP_CMD_READ = 1,
    VCAP_CMD_MOVE_DOWN = 2,
    VCAP_CMD_MOVE_UP = 3,
    VCAP_CMD_INITIALIZE = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_rule_error {
    VCAP_ERR_NONE = 0,  /* No known error */
    VCAP_ERR_NO_ADMIN,  /* No admin instance */
    VCAP_ERR_NO_NETDEV,  /* No netdev instance */
    VCAP_ERR_NO_KEYSET_MATCH, /* No keyset matched the rule keys */
    VCAP_ERR_NO_ACTIONSET_MATCH, /* No actionset matched the rule actions */
    VCAP_ERR_NO_PORT_KEYSET_MATCH, /* No port keyset matched the rule keys */
}

// Administration of each VCAP instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_admin {
    pub /: *mut *mut list_head list; / for insertion in vcap_control,
    pub /: *mut *mut list_head rules; / list of rules,
    pub /: *mut *mut list_head enabled; / list of enabled ports,
    pub /: *mut *mut *mut vcap_control vctrl; / the control instance owning this vcap,
    pub /: *mut *mut vcap_type vtype; / type of vcap,
    pub /: *mut *mut int vinst; / instance number within the same type,
    pub /: *mut *mut int first_cid; / first chain id in this vcap,
    pub /: *mut *mut int last_cid; / last chain id in this vcap,
    pub /: *mut *mut int tgt_inst; / hardware instance number,
    pub /: *mut *mut int lookups; / number of lookups in this vcap type,
    pub /: *mut *mut int lookups_per_instance; / number of lookups in this instance,
    pub /: *mut *mut int last_valid_addr; / top of address range to be used,
    pub /: *mut *mut int first_valid_addr; / bottom of address range to be used,
    pub /: *mut *mut int last_used_addr; / address of lowest added rule,
    pub /: *mut *mut bool w32be; / vcap uses "32bit-word big-endian" encoding,
    pub /: *mut *mut bool ingress; / chain traffic direction,
    pub /: *mut *mut vcap_cache_data cache; / encoded rule data,
}

// Client supplied VCAP rule data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_rule {
    pub /: *mut *mut int vcap_chain_id; / chain used for this rule,
    pub /: *mut *mut vcap_user user; / rule owner,
    pub priority: u16,
    pub /: *mut *mut u32 id; / vcap rule id, must be unique, 0 will auto-generate a value,
    pub /: *mut *mut u64 cookie; / used by the client to identify the rule,
    pub /: *mut *mut list_head keyfields; / list of vcap_client_keyfield,
    pub /: *mut *mut list_head actionfields; / list of vcap_client_actionfield,
    pub /: *mut *mut vcap_keyfield_set keyset; / keyset used: may be derived from fields,
    pub /: *mut *mut vcap_actionfield_set actionset; / actionset used: may be derived from fields,
    pub /: *mut *mut vcap_rule_error exterr; / extended error - used by TC,
    pub /: *mut *mut u64 client; / space for client defined data,
}

// List of keysets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_keyset_list {
    pub /: *mut *mut int max; / size of the keyset list,
    pub /: *mut *mut int cnt; / count of keysets actually in the list,
    pub /: *mut *mut *mut vcap_keyfield_set keysets; / the list of keysets,
}

// List of actionsets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_actionset_list {
    pub /: *mut *mut int max; / size of the actionset list,
    pub /: *mut *mut int cnt; / count of actionsets actually in the list,
    pub /: *mut *mut *mut vcap_actionfield_set actionsets; / the list of actionsets,
}

// Client output printf-like function with destination
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_output_print {
    pub ...): *const *const *const *const void (prf)(void out, char fmt,,
    pub dst: *mut c_void,
}

// Client supplied VCAP callback operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_operations {
// validate port keyset operation
    pub l3_proto): u16,
// add default rule fields for the selected keyset operations
    pub rule): *mut vcap_rule,
// cache operations
    pub admin): *mut (struct vcap_admin,
    pub count): u32 idx, u32,
    pub count): u32,
// block operations
    pub count): u32,
    pub addr): u32,
    pub count): c_int,
// informational
    pub out): *mut vcap_output_print,
}

// VCAP API Client control interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_control {
    pub /: *const *const *const vcap_operations ops; / client supplied operations,
    pub /: *const *const *const vcap_info vcaps; / client supplied vcap models,
    pub /: *const *const *const vcap_statistics stats; / client supplied vcap stats,
    pub /: *mut *mut list_head list; / list of vcap instances,
    pub /: *mut *mut mutex lock; / serialize access to all vcap instances,
}
