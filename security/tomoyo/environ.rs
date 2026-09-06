//! Automatically rewritten from C to Rust
//! Source: security/tomoyo/environ.c
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
// security/tomoyo/environ.c
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

//
// tomoyo_check_env_acl - Check permission for environment variable's name.
//
// @r:   Pointer to "struct tomoyo_request_info".
// @ptr: Pointer to "struct tomoyo_acl_info".
//
// Returns true if granted, false otherwise.
//
    static bool tomoyo_check_env_acl(struct tomoyo_request_info *r,
    const struct tomoyo_acl_info *ptr)
    {
    const struct tomoyo_env_acl *acl =
    container_of(ptr, typeof(*acl), head);
    return tomoyo_path_matches_pattern(r.param.environ.name, acl.env);
    }
//
// tomoyo_audit_env_log - Audit environment variable name log.
//
// @r: Pointer to "struct tomoyo_request_info".
//
// Returns 0 on success, negative value otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_audit_env_log(r: *mut tomoyo_request_info) -> c_int {
    static int tomoyo_audit_env_log(struct tomoyo_request_info *r)
    __must_hold_shared(&tomoyo_ss)
    {
    return tomoyo_supervisor(r, "misc env %s\n",
    r.param.environ.name.name);
    }
//
// tomoyo_env_perm - Check permission for environment variable's name.
//
// @r:   Pointer to "struct tomoyo_request_info".
// @env: The name of environment variable.
//
// Returns 0 on success, negative value otherwise.
//
// Caller holds tomoyo_read_lock().
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_env_perm(r: *mut tomoyo_request_info, env: *const c_char) -> c_int {
    int tomoyo_env_perm(struct tomoyo_request_info *r, const char *env)
    {
    struct tomoyo_path_info environ;
    int error;
    if (!env || !*env)
    return 0;
    environ.name = env;
    tomoyo_fill_path_info(&environ);
    r.param_type = TOMOYO_TYPE_ENV_ACL;
    r.param.environ.name = &environ;
    do {
    tomoyo_check_acl(r, tomoyo_check_env_acl);
    error = tomoyo_audit_env_log(r);
    } while (error == TOMOYO_RETRY_REQUEST);
    return error;
    }
//
// tomoyo_same_env_acl - Check for duplicated "struct tomoyo_env_acl" entry.
//
// @a: Pointer to "struct tomoyo_acl_info".
// @b: Pointer to "struct tomoyo_acl_info".
//
// Returns true if @a == @b, false otherwise.
//
    static bool tomoyo_same_env_acl(const struct tomoyo_acl_info *a,
    const struct tomoyo_acl_info *b)
    {
    const struct tomoyo_env_acl *p1 = container_of(a, typeof(*p1), head);
    const struct tomoyo_env_acl *p2 = container_of(b, typeof(*p2), head);
    return p1.env == p2.env;
    }
//
// tomoyo_write_env - Write "struct tomoyo_env_acl" list.
//
// @param: Pointer to "struct tomoyo_acl_param".
//
// Returns 0 on success, negative value otherwise.
//
// Caller holds tomoyo_read_lock().
//
#[no_mangle]
unsafe extern "C" fn tomoyo_write_env(param: *mut tomoyo_acl_param) -> c_int {
    static int tomoyo_write_env(struct tomoyo_acl_param *param)
    {
    let mut e: tomoyo_env_acl = { .head.type = TOMOYO_TYPE_ENV_ACL };
    let mut error: c_int = -ENOMEM;
    const char *data = tomoyo_read_token(param);
    if (!tomoyo_correct_word(data) || strchr(data, '='))
    return -EINVAL;
    e.env = tomoyo_get_name(data);
    if (!e.env)
    return error;
    error = tomoyo_update_domain(&e.head, sizeof(e), param,
    tomoyo_same_env_acl, core::ptr::null_mut());
    tomoyo_put_name(e.env);
    return error;
    }
//
// tomoyo_write_misc - Update environment variable list.
//
// @param: Pointer to "struct tomoyo_acl_param".
//
// Returns 0 on success, negative value otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_write_misc(param: *mut tomoyo_acl_param) -> c_int {
    int tomoyo_write_misc(struct tomoyo_acl_param *param)
    {
    if (tomoyo_str_starts(&param.data, "env "))
    return tomoyo_write_env(param);
    return -EINVAL;
    }
