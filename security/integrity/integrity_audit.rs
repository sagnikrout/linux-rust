//! Automatically rewritten from C to Rust
//! Source: security/integrity/integrity_audit.c
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
// Copyright (C) 2008 IBM Corporation
// Author: Mimi Zohar <zohar@us.ibm.com>
//
// File: integrity_audit.c
// Audit calls for the integrity subsystem
//

    static int integrity_audit_info;
// ima_audit_setup - enable informational auditing messages
#[no_mangle]
unsafe extern "C" fn integrity_audit_setup(str: *mut c_char) -> int __init {
    static int __init integrity_audit_setup(char *str)
    {
    unsigned long audit;
    if (!kstrtoul(str, 0, &audit))
    integrity_audit_info = audit ? 1 : 0;
    return 1;
    }
    __setup("integrity_audit=", integrity_audit_setup);
    void integrity_audit_msg(int audit_msgno, struct inode *inode,
    const unsigned char *fname, const char *op,
    const char *cause, int result, int audit_info)
    {
    integrity_audit_message(audit_msgno, inode, fname, op, cause,
    result, audit_info, 0);
    }
    void integrity_audit_message(int audit_msgno, struct inode *inode,
    const unsigned char *fname, const char *op,
    const char *cause, int result, int audit_info,
    int errno)
    {
    struct audit_buffer *ab;
    char name[TASK_COMM_LEN];
    if (!integrity_audit_info && audit_info == 1)	/* Skip info messages */
    return;
    ab = audit_log_start(audit_context(), GFP_KERNEL, audit_msgno);
    if (!ab)
    return;
    audit_log_format(ab, "pid=%d uid=%u auid=%u ses=%u",
    task_pid_nr(current),
    from_kuid(&init_user_ns, current_uid()),
    from_kuid(&init_user_ns, audit_get_loginuid(current)),
    audit_get_sessionid(current));
    audit_log_task_context(ab);
    audit_log_format(ab, " op=%s cause=%s comm=", op, cause);
    audit_log_untrustedstring(ab, get_task_comm(name, current));
    if (fname) {
    audit_log_format(ab, " name=");
    audit_log_untrustedstring(ab, fname);
    }
    if (inode) {
    audit_log_format(ab, " dev=");
    audit_log_untrustedstring(ab, inode.i_sb.s_id);
    audit_log_format(ab, " ino=%llu", inode.i_ino);
    }
    audit_log_format(ab, " res=%d errno=%d", !result, errno);
    audit_log_end(ab);
    }
