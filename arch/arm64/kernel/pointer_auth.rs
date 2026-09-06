//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/pointer_auth.c
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

#[no_mangle]
pub unsafe extern "C" fn ptrauth_prctl_reset_keys(tsk: *mut task_struct, arg: c_ulong) -> c_int {
    int ptrauth_prctl_reset_keys(struct task_struct *tsk, unsigned long arg)
    {
    struct ptrauth_keys_user *keys = &tsk.thread.keys_user;
    unsigned long addr_key_mask = PR_PAC_APIAKEY | PR_PAC_APIBKEY |
    PR_PAC_APDAKEY | PR_PAC_APDBKEY;
    let mut key_mask: c_ulong = addr_key_mask | PR_PAC_APGAKEY;
    if (!system_supports_address_auth() && !system_supports_generic_auth())
    return -EINVAL;
    if (is_compat_thread(task_thread_info(tsk)))
    return -EINVAL;
    if (!arg) {
    ptrauth_keys_init_user(keys);
    return 0;
    }
    if (arg & ~key_mask)
    return -EINVAL;
    if (((arg & addr_key_mask) && !system_supports_address_auth()) ||
    ((arg & PR_PAC_APGAKEY) && !system_supports_generic_auth()))
    return -EINVAL;
    if (arg & PR_PAC_APIAKEY)
    get_random_bytes(&keys.apia, sizeof(keys.apia));
    if (arg & PR_PAC_APIBKEY)
    get_random_bytes(&keys.apib, sizeof(keys.apib));
    if (arg & PR_PAC_APDAKEY)
    get_random_bytes(&keys.apda, sizeof(keys.apda));
    if (arg & PR_PAC_APDBKEY)
    get_random_bytes(&keys.apdb, sizeof(keys.apdb));
    if (arg & PR_PAC_APGAKEY)
    get_random_bytes(&keys.apga, sizeof(keys.apga));
    ptrauth_keys_install_user(keys);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arg_to_enxx_mask(arg: c_ulong) -> u64 {
    static u64 arg_to_enxx_mask(unsigned long arg)
    {
    let mut sctlr_enxx_mask: u64 = 0;
    WARN_ON(arg & ~PR_PAC_ENABLED_KEYS_MASK);
    if (arg & PR_PAC_APIAKEY)
    sctlr_enxx_mask |= SCTLR_ELx_ENIA;
    if (arg & PR_PAC_APIBKEY)
    sctlr_enxx_mask |= SCTLR_ELx_ENIB;
    if (arg & PR_PAC_APDAKEY)
    sctlr_enxx_mask |= SCTLR_ELx_ENDA;
    if (arg & PR_PAC_APDBKEY)
    sctlr_enxx_mask |= SCTLR_ELx_ENDB;
    return sctlr_enxx_mask;
    }
    int ptrauth_set_enabled_keys(struct task_struct *tsk, unsigned long keys,
    unsigned long enabled)
    {
    u64 sctlr;
    if (!system_supports_address_auth())
    return -EINVAL;
    if (is_compat_thread(task_thread_info(tsk)))
    return -EINVAL;
    if ((keys & ~PR_PAC_ENABLED_KEYS_MASK) || (enabled & ~keys))
    return -EINVAL;
    preempt_disable();
    sctlr = tsk.thread.sctlr_user;
    sctlr &= ~arg_to_enxx_mask(keys);
    sctlr |= arg_to_enxx_mask(enabled);
    tsk.thread.sctlr_user = sctlr;
    if (tsk == current)
    update_sctlr_el1(sctlr);
    preempt_enable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrauth_get_enabled_keys(tsk: *mut task_struct) -> c_int {
    int ptrauth_get_enabled_keys(struct task_struct *tsk)
    {
    let mut retval: c_int = 0;
    if (!system_supports_address_auth())
    return -EINVAL;
    if (is_compat_thread(task_thread_info(tsk)))
    return -EINVAL;
    if (tsk.thread.sctlr_user & SCTLR_ELx_ENIA)
    retval |= PR_PAC_APIAKEY;
    if (tsk.thread.sctlr_user & SCTLR_ELx_ENIB)
    retval |= PR_PAC_APIBKEY;
    if (tsk.thread.sctlr_user & SCTLR_ELx_ENDA)
    retval |= PR_PAC_APDAKEY;
    if (tsk.thread.sctlr_user & SCTLR_ELx_ENDB)
    retval |= PR_PAC_APDBKEY;
    return retval;
    }
