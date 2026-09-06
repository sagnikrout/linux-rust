//! Automatically rewritten from C to Rust
//! Source: rust/helpers/security.c
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

    __rust_helper void rust_helper_security_cred_getsecid(const struct cred *c,
    u32 *secid)
    {
    security_cred_getsecid(c, secid);
    }
    __rust_helper int rust_helper_security_secid_to_secctx(u32 secid,
    struct lsm_context *cp)
    {
    return security_secid_to_secctx(secid, cp);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_security_release_secctx(cp: *mut lsm_context) -> __rust_helper void {
    __rust_helper void rust_helper_security_release_secctx(struct lsm_context *cp)
    {
    security_release_secctx(cp);
    }
    __rust_helper int
    rust_helper_security_binder_set_context_mgr(const struct cred *mgr)
    {
    return security_binder_set_context_mgr(mgr);
    }
    __rust_helper int
    rust_helper_security_binder_transaction(const struct cred *from,
    const struct cred *to)
    {
    return security_binder_transaction(from, to);
    }
    __rust_helper int
    rust_helper_security_binder_transfer_binder(const struct cred *from,
    const struct cred *to)
    {
    return security_binder_transfer_binder(from, to);
    }
    __rust_helper int rust_helper_security_binder_transfer_file(
    const struct cred *from, const struct cred *to, const struct file *file)
    {
    return security_binder_transfer_file(from, to, file);
    }
