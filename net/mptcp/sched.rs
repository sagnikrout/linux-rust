//! Automatically rewritten from C to Rust
//! Source: net/mptcp/sched.c
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
// Multipath TCP
//
// Copyright (c) 2022, SUSE.
//

    static DEFINE_SPINLOCK(mptcp_sched_list_lock);
    static LIST_HEAD(mptcp_sched_list);
#[no_mangle]
unsafe extern "C" fn mptcp_sched_default_get_send(msk: *mut mptcp_sock) -> c_int {
    static int mptcp_sched_default_get_send(struct mptcp_sock *msk)
    {
    struct sock *ssk;
    ssk = mptcp_subflow_get_send(msk);
    if (!ssk)
    return -EINVAL;
    mptcp_subflow_set_scheduled(mptcp_subflow_ctx(ssk), true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mptcp_sched_default_get_retrans(msk: *mut mptcp_sock) -> c_int {
    static int mptcp_sched_default_get_retrans(struct mptcp_sock *msk)
    {
    struct sock *ssk;
    ssk = mptcp_subflow_get_retrans(msk);
    if (!ssk)
    return -EINVAL;
    mptcp_subflow_set_scheduled(mptcp_subflow_ctx(ssk), true);
    return 0;
    }
    static struct mptcp_sched_ops mptcp_sched_default = {
    .get_send	= mptcp_sched_default_get_send,
    .get_retrans	= mptcp_sched_default_get_retrans,
    .name		= "default",
    .owner		= THIS_MODULE,
    };
// Must be called with rcu read lock held
    struct mptcp_sched_ops *mptcp_sched_find(const char *name)
    {
    struct mptcp_sched_ops *sched, *ret = core::ptr::null_mut();
    list_for_each_entry_rcu(sched, &mptcp_sched_list, list) {
    if (!strcmp(sched.name, name)) {
    ret = sched;
    break;
    }
    }
    return ret;
    }
// Build string with list of available scheduler values.
// Similar to tcp_get_available_congestion_control()
//
#[no_mangle]
pub unsafe extern "C" fn mptcp_get_available_schedulers(buf: *mut c_char, maxlen: usize) {
    void mptcp_get_available_schedulers(char *buf, size_t maxlen)
    {
    struct mptcp_sched_ops *sched;
    let mut offs: usize = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(sched, &mptcp_sched_list, list) {
    offs += snprintf(buf + offs, maxlen - offs,
    "%s%s",
    offs == 0 ? "" : " ", sched.name);
    if (WARN_ON_ONCE(offs >= maxlen))
    break;
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_validate_scheduler(sched: *mut mptcp_sched_ops) -> c_int {
    int mptcp_validate_scheduler(struct mptcp_sched_ops *sched)
    {
    if (!sched.get_send) {
    pr_err("%s does not implement required ops\n", sched.name);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_register_scheduler(sched: *mut mptcp_sched_ops) -> c_int {
    int mptcp_register_scheduler(struct mptcp_sched_ops *sched)
    {
    int ret;
    ret = mptcp_validate_scheduler(sched);
    if (ret)
    return ret;
    spin_lock(&mptcp_sched_list_lock);
    if (mptcp_sched_find(sched.name)) {
    spin_unlock(&mptcp_sched_list_lock);
    return -EEXIST;
    }
    list_add_tail_rcu(&sched.list, &mptcp_sched_list);
    spin_unlock(&mptcp_sched_list_lock);
    pr_debug("%s registered\n", sched.name);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_unregister_scheduler(sched: *mut mptcp_sched_ops) {
    void mptcp_unregister_scheduler(struct mptcp_sched_ops *sched)
    {
    if (sched == &mptcp_sched_default)
    return;
    spin_lock(&mptcp_sched_list_lock);
    list_del_rcu(&sched.list);
    spin_unlock(&mptcp_sched_list_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_sched_init() {
    void mptcp_sched_init(void)
    {
    mptcp_register_scheduler(&mptcp_sched_default);
    }
    int mptcp_init_sched(struct mptcp_sock *msk,
    struct mptcp_sched_ops *sched)
    {
    if (!sched)
    sched = &mptcp_sched_default;
    if (!bpf_try_module_get(sched, sched.owner))
    return -EBUSY;
    msk.sched = sched;
    if (msk.sched.init)
    msk.sched.init(msk);
    pr_debug("sched=%s\n", msk.sched.name);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_release_sched(msk: *mut mptcp_sock) {
    void mptcp_release_sched(struct mptcp_sock *msk)
    {
    struct mptcp_sched_ops *sched = msk.sched;
    if (!sched)
    return;
    msk.sched = core::ptr::null_mut();
    if (sched.release)
    sched.release(msk);
    bpf_module_put(sched, sched.owner);
    }
    void mptcp_subflow_set_scheduled(struct mptcp_subflow_context *subflow,
    bool scheduled)
    {
    WRITE_ONCE(subflow.scheduled, scheduled);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_sched_get_send(msk: *mut mptcp_sock) -> c_int {
    int mptcp_sched_get_send(struct mptcp_sock *msk)
    {
    struct mptcp_subflow_context *subflow;
    msk_owned_by_me(msk);
// the following check is moved out of mptcp_subflow_get_send
    if (__mptcp_check_fallback(msk)) {
    if (msk.first &&
    __tcp_can_send(msk.first) &&
    sk_stream_memory_free(msk.first)) {
    mptcp_subflow_set_scheduled(mptcp_subflow_ctx(msk.first), true);
    return 0;
    }
    return -EINVAL;
    }
    mptcp_for_each_subflow(msk, subflow) {
    if (READ_ONCE(subflow.scheduled))
    return 0;
    }
    if (msk.sched == &mptcp_sched_default || !msk.sched)
    return mptcp_sched_default_get_send(msk);
    return msk.sched.get_send(msk);
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_sched_get_retrans(msk: *mut mptcp_sock) -> c_int {
    int mptcp_sched_get_retrans(struct mptcp_sock *msk)
    {
    struct mptcp_subflow_context *subflow;
    msk_owned_by_me(msk);
// the following check is moved out of mptcp_subflow_get_retrans
    if (__mptcp_check_fallback(msk))
    return -EINVAL;
    mptcp_for_each_subflow(msk, subflow) {
    if (READ_ONCE(subflow.scheduled))
    return 0;
    }
    if (msk.sched == &mptcp_sched_default || !msk.sched)
    return mptcp_sched_default_get_retrans(msk);
    if (msk.sched.get_retrans)
    return msk.sched.get_retrans(msk);
    return msk.sched.get_send(msk);
    }
