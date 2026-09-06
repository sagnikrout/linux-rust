//! Automatically rewritten from C to Rust
//! Source: net/ceph/string_table.c
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

    static DEFINE_SPINLOCK(string_tree_lock);
    let mut string_tree: static struct rb_root = RB_ROOT;
    struct ceph_string *ceph_find_or_create_string(const char* str, size_t len)
    {
    struct ceph_string *cs, *exist;
    struct rb_node **p, *parent;
    int ret;
    exist = core::ptr::null_mut();
    spin_lock(&string_tree_lock);
    p = &string_tree.rb_node;
    while (*p) {
    exist = rb_entry(*p, struct ceph_string, node);
    ret = ceph_compare_string(exist, str, len);
    if (ret > 0)
    p = &(*p).rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    p = &(*p).rb_right;
    else
    break;
    exist = core::ptr::null_mut();
    }
    if (exist && !kref_get_unless_zero(&exist.kref)) {
    rb_erase(&exist.node, &string_tree);
    RB_CLEAR_NODE(&exist.node);
    exist = core::ptr::null_mut();
    }
    spin_unlock(&string_tree_lock);
    if (exist)
    return exist;
    cs = kmalloc(sizeof(*cs) + len + 1, GFP_NOFS);
    if (!cs)
    return core::ptr::null_mut();
    kref_init(&cs.kref);
    cs.len = len;
    memcpy(cs.str, str, len);
    cs.str[len] = 0;
    retry:
    exist = core::ptr::null_mut();
    parent = core::ptr::null_mut();
    p = &string_tree.rb_node;
    spin_lock(&string_tree_lock);
    while (*p) {
    parent = *p;
    exist = rb_entry(*p, struct ceph_string, node);
    ret = ceph_compare_string(exist, str, len);
    if (ret > 0)
    p = &(*p).rb_left;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    p = &(*p).rb_right;
    else
    break;
    exist = core::ptr::null_mut();
    }
    ret = 0;
    if (!exist) {
    rb_link_node(&cs.node, parent, p);
    rb_insert_color(&cs.node, &string_tree);
    } else if (!kref_get_unless_zero(&exist.kref)) {
    rb_erase(&exist.node, &string_tree);
    RB_CLEAR_NODE(&exist.node);
    ret = -EAGAIN;
    }
    spin_unlock(&string_tree_lock);
    if (ret == -EAGAIN)
    goto retry;
    if (exist) {
    kfree(cs);
    cs = exist;
    }
    return cs;
    }
    EXPORT_SYMBOL(ceph_find_or_create_string);
#[no_mangle]
pub unsafe extern "C" fn ceph_release_string(ref: *mut kref) {
    void ceph_release_string(struct kref *ref)
    {
    struct ceph_string *cs = container_of(ref, struct ceph_string, kref);
    spin_lock(&string_tree_lock);
    if (!RB_EMPTY_NODE(&cs.node)) {
    rb_erase(&cs.node, &string_tree);
    RB_CLEAR_NODE(&cs.node);
    }
    spin_unlock(&string_tree_lock);
    kfree_rcu(cs, rcu);
    }
    EXPORT_SYMBOL(ceph_release_string);
#[no_mangle]
pub unsafe extern "C" fn ceph_strings_empty() -> bool {
    bool ceph_strings_empty(void)
    {
    return RB_EMPTY_ROOT(&string_tree);
    }
