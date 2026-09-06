//! Automatically rewritten from C to Rust
//! Source: security/apparmor/procattr.c
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
// AppArmor security module
//
// This file contains AppArmor /proc/<pid>/attr/ interface functions
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

//
// aa_getprocattr - Return the label information for @label
// @label: the label to print label info about  (NOT NULL)
// @string: Returns - string containing the label info (NOT NULL)
// @newline: indicates that a newline should be added
//
// Requires: label != NULL && string != NULL
//
// Creates a string containing the label information for @label.
//
// Returns: size of string placed in @string else error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn aa_getprocattr(label: *mut aa_label, string: *mut c_char, newline: bool) -> c_int {
    int aa_getprocattr(struct aa_label *label, char **string, bool newline)
    {
    struct aa_ns *ns = labels_ns(label);
    struct aa_ns *current_ns = aa_get_current_ns();
    int len;
    if (!aa_ns_visible(current_ns, ns, true)) {
    aa_put_ns(current_ns);
    return -EACCES;
    }
    len = aa_label_snxprint(core::ptr::null_mut(), 0, current_ns, label,
    FLAG_SHOW_MODE | FLAG_VIEW_SUBNS |
    FLAG_HIDDEN_UNCONFINED);
    AA_BUG(len < 0);
// string = kmalloc(len + 2, GFP_KERNEL);
    if (!*string) {
    aa_put_ns(current_ns);
    return -ENOMEM;
    }
    len = aa_label_snxprint(*string, len + 2, current_ns, label,
    FLAG_SHOW_MODE | FLAG_VIEW_SUBNS |
    FLAG_HIDDEN_UNCONFINED);
    if (len < 0) {
    kfree(*string);
// string = NULL;
    aa_put_ns(current_ns);
    return len;
    }
    if (newline)
    (*string)[len++] = '\n';
    (*string)[len] = 0;
    aa_put_ns(current_ns);
    return len;
    }
//
// split_token_from_name - separate a string of form  <token>^<name>
// @op: operation being checked
// @args: string to parse  (NOT NULL)
// @token: stores returned parsed token value  (NOT NULL)
//
// Returns: start position of name after token else NULL on failure
//
    static char *split_token_from_name(const char *op, char *args, u64 *token)
    {
    char *name;
// token = simple_strtoull(args, &name, 16);
    if ((name == args) || *name != '^') {
    AA_ERROR("%s: Invalid input '%s'", op, args);
    return ERR_PTR(-EINVAL);
    }
    name++;			/* skip ^ */
    if (!*name)
    name = core::ptr::null_mut();
    return name;
    }
//
// aa_setprocattr_changehat - handle procattr interface to change_hat
// @args: args received from writing to /proc/<pid>/attr/current (NOT NULL)
// @size: size of the args
// @flags: set of flags governing behavior
//
// Returns: %0 or error code if change_hat fails
//
#[no_mangle]
pub unsafe extern "C" fn aa_setprocattr_changehat(args: *mut c_char, size: usize, flags: c_int) -> c_int {
    int aa_setprocattr_changehat(char *args, size_t size, int flags)
    {
    char *hat;
    u64 token;
    const char *hats[16];		/* current hard limit on # of names */
    let mut count: c_int = 0;
    hat = split_token_from_name(OP_CHANGE_HAT, args, &token);
    if (IS_ERR(hat))
    return PTR_ERR(hat);
    if (!hat && !token) {
    AA_ERROR("change_hat: Invalid input, core::ptr::null_mut() hat and core::ptr::null_mut() magic");
    return -EINVAL;
    }
    if (hat) {
// set up hat name vector, args guaranteed null terminated
// at args[size] by setprocattr.
//
// If there are multiple hat names in the buffer each is
// separated by a \0.  Ie. userspace writes them pre tokenized
//
    char *end = args + size;
    for (count = 0; (hat < end) && count < 16; ++count) {
    char *next = hat + strlen(hat) + 1;
    hats[count] = hat;
    AA_DEBUG(DEBUG_DOMAIN,
    "%s: (pid %d) Magic 0x%llx count %d hat '%s'\n"
    , __func__, current.pid, token, count, hat);
    hat = next;
    }
    } else
    AA_DEBUG(DEBUG_DOMAIN,
    "%s: (pid %d) Magic 0x%llx count %d Hat '%s'\n",
    __func__, current.pid, token, count, "<core::ptr::null_mut()>");
    return aa_change_hat(hats, count, token, flags);
    }
