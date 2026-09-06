//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_efi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 IBM Corporation
//

// secureboot arch rules
    static const char * const sb_arch_rules[] = {

    "appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig",

    "measure func=KEXEC_KERNEL_CHECK",

    "appraise func=MODULE_CHECK appraise_type=imasig",

    "appraise func=POLICY_CHECK appraise_type=imasig",

    "measure func=CRITICAL_DATA label=ima_policy",

    "measure func=MODULE_CHECK",
    core::ptr::null_mut()
    };
    const char * const *arch_get_ima_policy(void)
    {
    if (IS_ENABLED(CONFIG_IMA_ARCH_POLICY) && arch_get_secureboot()) {
    set_module_sig_enforced();
    set_kexec_sig_enforced();
    return sb_arch_rules;
    }
    return core::ptr::null_mut();
    }
