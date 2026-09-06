//! Automatically rewritten from C to Rust
//! Source: drivers/target/iscsi/iscsi_target_transport.c
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

    static LIST_HEAD(g_transport_list);
    static DEFINE_MUTEX(transport_mutex);
    struct iscsit_transport *iscsit_get_transport(int type)
    {
    struct iscsit_transport *t;
    mutex_lock(&transport_mutex);
    list_for_each_entry(t, &g_transport_list, t_node) {
    if (t.transport_type == type) {
    if (t.owner && !try_module_get(t.owner)) {
    t = core::ptr::null_mut();
    }
    mutex_unlock(&transport_mutex);
    return t;
    }
    }
    mutex_unlock(&transport_mutex);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn iscsit_put_transport(t: *mut iscsit_transport) {
    void iscsit_put_transport(struct iscsit_transport *t)
    {
    module_put(t.owner);
    }
#[no_mangle]
pub unsafe extern "C" fn iscsit_register_transport(t: *mut iscsit_transport) {
    void iscsit_register_transport(struct iscsit_transport *t)
    {
    INIT_LIST_HEAD(&t.t_node);
    mutex_lock(&transport_mutex);
    list_add_tail(&t.t_node, &g_transport_list);
    mutex_unlock(&transport_mutex);
    pr_debug("Registered iSCSI transport: %s\n", t.name);
    }
    EXPORT_SYMBOL(iscsit_register_transport);
#[no_mangle]
pub unsafe extern "C" fn iscsit_unregister_transport(t: *mut iscsit_transport) {
    void iscsit_unregister_transport(struct iscsit_transport *t)
    {
    mutex_lock(&transport_mutex);
    list_del(&t.t_node);
    mutex_unlock(&transport_mutex);
    pr_debug("Unregistered iSCSI transport: %s\n", t.name);
    }
    EXPORT_SYMBOL(iscsit_unregister_transport);
