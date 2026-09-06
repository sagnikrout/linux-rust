//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/ruleset.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Landlock LSM - Ruleset management
//
// Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2018-2020 ANSSI
// Copyright © 2026 Cloudflare, Inc.
//

//
// struct landlock_layer - Access rights for a given layer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_layer {
//
// @level: Position of this layer in the layer stack.  Starts from 1.
//
    pub level: u8,
//
// @flags: Bitfield for special flags attached to this rule.
//
// @flags.quiet: Suppresses denial logs for the object covered by
// this rule in this domain.  For filesystem rules, this inherits
// down the file hierarchy.
//
    pub 1: u8 quiet :,
    pub flags: },
//
// @access: Bitfield of allowed actions on the kernel object.  They are
// relative to the object type (e.g. %LANDLOCK_ACTION_FS_READ).
//
    pub access: access_mask_t,
}

//
// union landlock_key - Key of a ruleset's red-black tree
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union landlock_key {
//
// @object: Pointer to identify a kernel object (e.g. an inode).
//
    pub object: *mut landlock_object,
//
// @data: Raw data to identify an arbitrary 32-bit value
// (e.g. a TCP port).
//
    pub data: uintptr_t,
}

//
// enum landlock_key_type - Type of &union landlock_key
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum landlock_key_type {
//
// @LANDLOCK_KEY_INODE: Type of &landlock_rules.root_inode's node keys.
//
    LANDLOCK_KEY_INODE = 1,
//
// @LANDLOCK_KEY_NET_PORT: Type of &landlock_rules.root_net_port's node
// keys.
//
    LANDLOCK_KEY_NET_PORT,
}

//
// struct landlock_id - Unique rule identifier for a ruleset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_id {
//
// @key: Identifies either a kernel object (e.g. an inode) or
// a raw value (e.g. a TCP port).
//
    pub key: landlock_key,
//
// @type: Type of a landlock_ruleset's root tree.
//
    pub type: landlock_key_type,
}

//
// struct landlock_rule - Access rights tied to an object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_rule {
//
// @node: Node in the ruleset's red-black tree.
//
    pub node: rb_node,
//
// @key: A union to identify either a kernel object (e.g. an inode) or
// a raw data value (e.g. a network socket port). This is used as a key
// for this ruleset element.  The pointer is set once and never
// modified.  It always points to an allocated object because each rule
// increments the refcount of its object.
//
    pub key: landlock_key,
//
// @num_layers: Number of entries in @layers.
//
    pub num_layers: u32,
//
// @layers: Stack of layers, from the latest to the newest, implemented
// as a flexible array member (FAM).
//
    pub __counted_by(num_layers): landlock_layer layers[],
}

//
// struct landlock_rules - Red-black tree storage for Landlock rules
//
// This structure holds the rule trees shared by both rulesets and domains.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_rules {
//
// @root_inode: Root of a red-black tree containing &struct
// landlock_rule nodes with inode object.  Immutable for domains.
//
    pub root_inode: rb_root,

//
// @root_net_port: Root of a red-black tree containing &struct
// landlock_rule nodes with network port.  Immutable for domains.
//
    pub root_net_port: rb_root,

//
// @num_rules: Number of non-overlapping (i.e. not for the same object)
// rules in this tree storage.
//
    pub num_rules: u32,
}

//
// struct landlock_ruleset - Landlock ruleset
//
// This data structure must contain unique entries, be updatable, and quick to
// match an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_ruleset {
//
// @rules: Red-black tree storage for rules.
//
    pub rules: landlock_rules,
//
// @lock: Protects against concurrent modifications of @rules, if @usage
// is greater than zero.
//
    pub lock: mutex,
//
// @usage: Number of file descriptors referencing this ruleset.
//
    pub usage: refcount_t,

//
// @version: Counter incremented on each successful
// landlock_add_rule(2), including when it only extends an existing
// rule's access rights.  Used by tracepoints to correlate a domain with
// the exact ruleset state it was created from.  Protected by @lock.
//
    pub version: u32,
//
// @id: Unique identifier for this ruleset, used for tracing.
//
    pub id: u64,

//
// @quiet_masks: Stores the quiet flags for an unmerged ruleset.  For a
// merged domain, this is stored in each layer's struct
// landlock_hierarchy instead.
//
    pub quiet_masks: access_masks,
//
// @handled_masks: Contains the subset of filesystem and network actions
// that are handled by this ruleset.
//
    pub handled_masks: access_masks,
}

extern "C" {
    pub fn landlock_put_ruleset(ruleset: *const *const landlock_ruleset);
}
extern "C" {
    pub fn landlock_free_rules(rules: *const *const landlock_rules);
}
//
// landlock_get_rule_root - Get the root of a rule tree by key type
//
// @rules: The rules storage to look up.
// @key_type: The type of key to select the tree for.
//
// Return: A pointer to the rb_root, or ERR_PTR(-EINVAL) on unknown type.
//

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
