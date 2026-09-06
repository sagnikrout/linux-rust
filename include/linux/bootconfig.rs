//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bootconfig.h
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
// Extra Boot Config
// Copyright (C) 2019 Linaro Ltd.
// Author: Masami Hiramatsu <mhiramat@kernel.org>
//

extern "C" {
    pub fn cmdline_has_extra_options() -> bool __init;
}

//
// NOTE: This is only for tools/bootconfig, because tools/bootconfig will
// run the parser sanity test.
// This does NOT mean linux/bootconfig.h is available in the user space.
// However, if you change this file, please make sure the tools/bootconfig
// has no issue on building and running.
//

pub const BOOTCONFIG_MAGIC_LEN: c_int = 12;
pub const BOOTCONFIG_ALIGN_SHIFT: c_int = 2;

//
// xbc_calc_checksum() - Calculate checksum of bootconfig
// @data: Bootconfig data.
// @size: The size of the bootconfig data.
//
// Calculate the checksum value of the bootconfig data.
// The checksum will be used with the BOOTCONFIG_MAGIC and the size for
// embedding the bootconfig in the initrd image.
//
// XBC tree node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbc_node {
    pub next: u16,
    pub child: u16,
    pub parent: u16,
    pub data: u16,
// C attribute field omitted
pub const XBC_KEY: c_int = 0;

// Maximum size of boot config is 32KB - 1

pub const XBC_NODE_MAX: c_int = 8192;
pub const XBC_KEYLEN_MAX: c_int = 256;
pub const XBC_DEPTH_MAX: c_int = 16;
// Node tree access raw APIs
    pub xbc_root_node(void): *mut *mut xbc_node  __init,
    pub node): *mut uint16_t __init xbc_node_index(struct xbc_node,
    pub node): *mut *mut xbc_node  __init xbc_node_get_parent(xbc_node,
    pub node): *mut *mut xbc_node  __init xbc_node_get_child(xbc_node,
    pub node): *mut *mut xbc_node  __init xbc_node_get_next(xbc_node,
    pub node): *const *const char  __init xbc_node_get_data(struct xbc_node,
//
// xbc_node_is_value() - Test the node is a value node
// @node: An XBC node.
//
// Test the @node is a value node and return true if a value node, false if not.
//
    pub XBC_VALUE: return node->data &,
//
// xbc_node_is_key() - Test the node is a key node
// @node: An XBC node.
//
// Test the @node is a key node and return true if a key node, false if not.
//
    pub !xbc_node_is_value(node): return,
//
// xbc_node_is_array() - Test the node is an arraied value node
// @node: An XBC node.
//
// Test the @node is an arraied value node.
//
    pub 0: return xbc_node_is_value(node) && node->child !=,
//
// xbc_node_is_leaf() - Test the node is a leaf key node
// @node: An XBC node.
//
// Test the @node is a leaf key node which is a key node and has a value node
// or no child. Returns true if it is a leaf node, or false if not.
// Note that the leaf node can have subkey nodes in addition to the
// value node.
//
    pub xbc_node_is_value(xbc_node_get_child(node))): (!node->child ||,
// Tree-based key-value access APIs
    pub key): *const c_char,
    pub vnode): *mut xbc_node,
    pub leaf): *mut xbc_node,
    pub leaf): *mut xbc_node,
//
// xbc_find_value() - Find a value which matches the key
// @key: Search key
// @vnode: A container pointer of XBC value node.
//
// Search a value whose key matches @key from whole of XBC tree and return
// the value if found. Found value node is stored in *@vnode.
// Note that this can return 0-length string and store NULL in *@vnode for
// key-only (non-value) entry.
//
    pub vnode): return xbc_node_find_value(NULL, key,,
//
// xbc_find_node() - Find a node which matches the key
// @key: Search key
//
// Search a (key) node whose key matches @key from whole of XBC tree and
// return the node if found. If not found, returns NULL.
//
    pub key): return xbc_node_find_subkey(NULL,,
//
// xbc_node_get_subkey() - Return the first subkey node if exists
// @node: Parent node
//
// Return the first subkey node of the @node. If the @node has no child
// or only value node, this will return NULL.
//
    pub xbc_node_get_child(node): *mut *mut xbc_node child =,
    pub xbc_node_get_next(child): return,
    pub child: return,
//
// xbc_array_for_each_value() - Iterate value nodes on an array
// @anode: An XBC arraied value node
// @value: A value
//
// Iterate array value nodes and values starts from @anode. This is expected to
// be used with xbc_find_value() and xbc_node_find_value(), so that user can
// process each array entry node.
//

    pub \: for (value = xbc_node_get_data(anode); anode != NULL ;,
//
// xbc_node_for_each_child() - Iterate child nodes
// @parent: An XBC node.
// @child: Iterated XBC node.
//
// Iterate child nodes of @parent. Each child nodes are stored to @child.
// The @child can be mixture of a value node and subkey nodes.
//

    pub \: for (child = xbc_node_get_child(parent); child != NULL ;,
//
// xbc_node_for_each_subkey() - Iterate child subkey nodes
// @parent: An XBC node.
// @child: Iterated XBC node.
//
// Iterate subkey nodes of @parent. Each child nodes are stored to @child.
// The @child is only the subkey node.
//

    pub \: for (child = xbc_node_get_subkey(parent); child != NULL ;,
//
// xbc_node_for_each_array_value() - Iterate array entries of geven key
// @node: An XBC node.
// @key: A key string searched under @node
// @anode: Iterated XBC node of array entry.
// @value: Iterated value of array entry.
//
// Iterate array entries of given @key under @node. Each array entry node
// is stored to @anode and @value. If the @node doesn't have @key node,
// it does nothing.
// Note that even if the found key node has only one value (not array)
// this executes block once. However, if the found key node has no value
// (key-only node), this does nothing. So don't use this for testing the
// key-value pair existence.
//

    pub \: for (value = xbc_node_find_value(node, key, &anode); value != NULL;,
//
// xbc_node_for_each_key_value() - Iterate key-value pairs under a node
// @node: An XBC node.
// @knode: Iterated key node
// @value: Iterated value string
//
// Iterate key-value pairs under @node. Each key node and value string are
// stored in @knode and @value respectively.
//

    pub &knode);\: for (knode = NULL, value = xbc_node_find_next_key_value(node,,
    pub &knode)): knode != NULL; value = xbc_node_find_next_key_value(node,,
//
// xbc_for_each_key_value() - Iterate key-value pairs
// @knode: Iterated key node
// @value: Iterated value string
//
// Iterate key-value pairs in whole XBC tree. Each key node and value string
// are stored in @knode and @value respectively.
//

// Compose partial key
    pub size): *mut *mut *mut xbc_node node, char buf, size_t,
// Render key/value pairs under @root as a flat cmdline string
    pub root): *mut *mut int __init xbc_snprint_cmdline(char buf, size_t size, struct xbc_node,
//
// xbc_node_compose_key() - Compose full key string of the XBC node
// @node: An XBC node.
// @buf: A buffer to store the key.
// @size: The size of the @buf.
//
// Compose the full-length key of the @node into @buf. Returns the total
// length of the key stored in @buf. Or returns -EINVAL if @node is NULL,
// and -ERANGE if the key depth is deeper than max depth.
//
    pub size): return xbc_node_compose_key_after(NULL, node, buf,,
// XBC node initializer
    pub epos): *const *const *const *const int __init xbc_init(char buf, size_t size, char emsg, int,
// XBC node and size information
    pub data_size): *mut *mut int __init xbc_get_info(int node_size, size_t,
// XBC cleanup data structures
    pub early): void __init _xbc_exit(bool,
// XBC embedded bootconfig data in kernel

    pub size): *const *const char  __init xbc_get_embedded_bootconfig(size_t,

    pub NULL: return,

// Bootconfig opt-in detection, shared by setup_arch() and setup_boot_config()

    pub end_offset): *const *const bool __init bootconfig_cmdline_requested(char boot_cmdline, int,

// Build-time-rendered bootconfig cmdline prepended in setup_arch()

    pub size): *mut *mut void __init xbc_prepend_embedded_cmdline(char dst, size_t,
    pub xbc_embedded_cmdline_applied(void): bool __init,

    pub }: static inline bool xbc_embedded_cmdline_applied(void) { return false;,

