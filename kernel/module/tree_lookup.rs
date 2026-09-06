//! Automatically rewritten from C to Rust
//! Source: kernel/module/tree_lookup.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Modules tree lookup
//
// Copyright (C) 2015 Peter Zijlstra
// Copyright (C) 2015 Rusty Russell
//

//
// Use a latched RB-tree for __module_address(); this allows us to use
// RCU lookups of the address from any context.
//
// This is conditional on PERF_EVENTS || TRACING || CFI because those can
// really hit __module_address() hard by doing a lot of stack unwinding;
// potentially from NMI context.
//
#[no_mangle]
unsafe extern "C" fn __mod_tree_val(n: *mut latch_tree_node) -> __always_inline unsigned long {
    static __always_inline unsigned long __mod_tree_val(struct latch_tree_node *n)
    {
    struct module_memory *mod_mem = container_of(n, struct module_memory, mtn.node);
    return (unsigned long)mod_mem.base;
    }
#[no_mangle]
unsafe extern "C" fn __mod_tree_size(n: *mut latch_tree_node) -> __always_inline unsigned long {
    static __always_inline unsigned long __mod_tree_size(struct latch_tree_node *n)
    {
    struct module_memory *mod_mem = container_of(n, struct module_memory, mtn.node);
    return (unsigned long)mod_mem.size;
    }
    static __always_inline bool
    mod_tree_less(struct latch_tree_node *a, struct latch_tree_node *b)
    {
    return __mod_tree_val(a) < __mod_tree_val(b);
    }
    static __always_inline int
    mod_tree_comp(void *key, struct latch_tree_node *n)
    {
    let mut val: c_ulong = (unsigned long)key;
    unsigned long start, end;
    start = __mod_tree_val(n);
    if (val < start)
    return -1;
    end = start + __mod_tree_size(n);
    if (val >= end)
    return 1;
    return 0;
    }
    static const struct latch_tree_ops mod_tree_ops = {
    .less = mod_tree_less,
    .comp = mod_tree_comp,
    };
#[no_mangle]
unsafe extern "C" fn __mod_tree_insert(node: *mut mod_tree_node, tree: *mut mod_tree_root) -> noinline void {
    static noinline void __mod_tree_insert(struct mod_tree_node *node, struct mod_tree_root *tree)
    {
    latch_tree_insert(&node.node, &tree.root, &mod_tree_ops);
    }
#[no_mangle]
unsafe extern "C" fn __mod_tree_remove(node: *mut mod_tree_node, tree: *mut mod_tree_root) {
    static void __mod_tree_remove(struct mod_tree_node *node, struct mod_tree_root *tree)
    {
    latch_tree_erase(&node.node, &tree.root, &mod_tree_ops);
    }
//
// These modifications: insert, remove_init and remove; are serialized by the
// module_mutex.
//
#[no_mangle]
pub unsafe extern "C" fn mod_tree_insert(mod: *mut module) {
    void mod_tree_insert(struct module *mod)
    {
    for_each_mod_mem_type(type) {
    mod.mem[type].mtn.mod = mod;
    if (mod.mem[type].size)
    __mod_tree_insert(&mod.mem[type].mtn, &mod_tree);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mod_tree_remove_init(mod: *mut module) {
    void mod_tree_remove_init(struct module *mod)
    {
    for_class_mod_mem_type(type, init) {
    if (mod.mem[type].size)
    __mod_tree_remove(&mod.mem[type].mtn, &mod_tree);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mod_tree_remove(mod: *mut module) {
    void mod_tree_remove(struct module *mod)
    {
    for_each_mod_mem_type(type) {
    if (mod.mem[type].size)
    __mod_tree_remove(&mod.mem[type].mtn, &mod_tree);
    }
    }
    struct module *mod_find(unsigned long addr, struct mod_tree_root *tree)
    {
    struct latch_tree_node *ltn;
    ltn = latch_tree_find((void *)addr, &tree.root, &mod_tree_ops);
    if (!ltn)
    return core::ptr::null_mut();
    return container_of(ltn, struct mod_tree_node, node).mod;
    }
