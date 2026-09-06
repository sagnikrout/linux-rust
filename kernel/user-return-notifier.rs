//! Automatically rewritten from C to Rust
//! Source: kernel/user-return-notifier.c
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

    static DEFINE_PER_CPU(struct hlist_head, return_notifier_list);
//
// Request a notification when the current cpu returns to userspace.  Must be
// called in atomic context.  The notifier will also be called in atomic
// context.
//
#[no_mangle]
pub unsafe extern "C" fn user_return_notifier_register(urn: *mut user_return_notifier) {
    void user_return_notifier_register(struct user_return_notifier *urn)
    {
    set_tsk_thread_flag(current, TIF_USER_RETURN_NOTIFY);
    hlist_add_head(&urn.link, this_cpu_ptr(&return_notifier_list));
    }
    EXPORT_SYMBOL_GPL(user_return_notifier_register);
//
// Removes a registered user return notifier.  Must be called from atomic
// context, and from the same cpu registration occurred in.
//
#[no_mangle]
pub unsafe extern "C" fn user_return_notifier_unregister(urn: *mut user_return_notifier) {
    void user_return_notifier_unregister(struct user_return_notifier *urn)
    {
    hlist_del(&urn.link);
    if (hlist_empty(this_cpu_ptr(&return_notifier_list)))
    clear_tsk_thread_flag(current, TIF_USER_RETURN_NOTIFY);
    }
    EXPORT_SYMBOL_GPL(user_return_notifier_unregister);
// Calls registered user return notifiers
#[no_mangle]
pub unsafe extern "C" fn fire_user_return_notifiers() {
    void fire_user_return_notifiers(void)
    {
    struct user_return_notifier *urn;
    struct hlist_node *tmp2;
    struct hlist_head *head;
    head = &get_cpu_var(return_notifier_list);
    hlist_for_each_entry_safe(urn, tmp2, head, link)
    urn.on_user_return(urn);
    put_cpu_var(return_notifier_list);
    }
