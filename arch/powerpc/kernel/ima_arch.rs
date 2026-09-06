//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ima_arch.c
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
// Copyright (C) 2019 IBM Corporation
// Author: Nayna Jain
//

//
// The "secure_rules" are enabled only on "secureboot" enabled systems.
// These rules verify the file signatures against known good values.
// The "appraise_type=imasig|modsig" option allows the known good signature
// to be stored as an xattr or as an appended signature.
//
// To avoid duplicate signature verification as much as possible, the IMA
// policy rule for module appraisal is added only if CONFIG_MODULE_SIG
// is not enabled.
//
    static const char *const secure_rules[] = {
    "appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig|modsig",

    "appraise func=MODULE_CHECK appraise_type=imasig|modsig",

    core::ptr::null_mut()
    };
//
// The "trusted_rules" are enabled only on "trustedboot" enabled systems.
// These rules add the kexec kernel image and kernel modules file hashes to
// the IMA measurement list.
//
    static const char *const trusted_rules[] = {
    "measure func=KEXEC_KERNEL_CHECK",
    "measure func=MODULE_CHECK",
    core::ptr::null_mut()
    };
//
// The "secure_and_trusted_rules" contains rules for both the secure boot and
// trusted boot. The "template=ima-modsig" option includes the appended
// signature, when available, in the IMA measurement list.
//
    static const char *const secure_and_trusted_rules[] = {
    "measure func=KEXEC_KERNEL_CHECK template=ima-modsig",
    "measure func=MODULE_CHECK template=ima-modsig",
    "appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig|modsig",

    "appraise func=MODULE_CHECK appraise_type=imasig|modsig",

    core::ptr::null_mut()
    };
//
// Returns the relevant IMA arch-specific policies based on the system secure
// boot state.
//
    const char *const *arch_get_ima_policy(void)
    {
    if (is_ppc_secureboot_enabled()) {
    set_module_sig_enforced();
    if (is_ppc_trustedboot_enabled())
    return secure_and_trusted_rules;
    else
    return secure_rules;
    } else if (is_ppc_trustedboot_enabled()) {
    return trusted_rules;
    }
    return core::ptr::null_mut();
    }
