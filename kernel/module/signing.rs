//! Automatically rewritten from C to Rust
//! Source: kernel/module/signing.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Module signature checker
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    let mut sig_enforce: static bool = IS_ENABLED(CONFIG_MODULE_SIG_FORCE);
    module_param(sig_enforce, bool_enable_only, 0644);
//
// Export sig_enforce kernel cmdline parameter to allow other subsystems rely
// on that instead of directly to CONFIG_MODULE_SIG_FORCE config.
//
#[no_mangle]
pub unsafe extern "C" fn is_module_sig_enforced() -> bool {
    bool is_module_sig_enforced(void)
    {
    return sig_enforce;
    }
    EXPORT_SYMBOL(is_module_sig_enforced);
#[no_mangle]
pub unsafe extern "C" fn set_module_sig_enforced() {
    void set_module_sig_enforced(void)
    {
    sig_enforce = true;
    }
//
// Verify the signature on a module.
//
#[no_mangle]
pub unsafe extern "C" fn mod_verify_sig(mod: *const c_void, info: *mut load_info) -> c_int {
    int mod_verify_sig(const void *mod, struct load_info *info)
    {
    struct module_signature ms;
    size_t sig_len, modlen = info.len;
    int ret;
    pr_devel("==>%s(,%zu)\n", __func__, modlen);
    if (modlen <= sizeof(ms))
    return -EBADMSG;
    memcpy(&ms, mod + (modlen - sizeof(ms)), sizeof(ms));
    ret = mod_check_sig(&ms, modlen, "module");
    if (ret)
    return ret;
    sig_len = be32_to_cpu(ms.sig_len);
    modlen -= sig_len + sizeof(ms);
    info.len = modlen;
    return verify_pkcs7_signature(mod, modlen, mod + modlen, sig_len,
    VERIFY_USE_SECONDARY_KEYRING,
    VERIFYING_MODULE_SIGNATURE,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn module_sig_check(info: *mut load_info, flags: c_int) -> c_int {
    int module_sig_check(struct load_info *info, int flags)
    {
    let mut err: c_int = -ENODATA;
    let mut markerlen: c_ulong = sizeof(MODULE_SIGNATURE_MARKER) - 1;
    const char *reason;
    const void *mod = info.hdr;
    bool mangled_module = flags & (MODULE_INIT_IGNORE_MODVERSIONS |
    MODULE_INIT_IGNORE_VERMAGIC);
//
// Do not allow mangled modules as a module with version information
// removed is no longer the module that was signed.
//
    if (!mangled_module &&
    info.len > markerlen &&
    memcmp(mod + info.len - markerlen, MODULE_SIGNATURE_MARKER, markerlen) == 0) {
// We truncate the module to discard the signature
    info.len -= markerlen;
    err = mod_verify_sig(mod, info);
    if (!err) {
    info.sig_ok = true;
    return 0;
    }
    }
//
// We don't permit modules to be loaded into the trusted kernels
// without a valid signature on them, but if we're not enforcing,
// certain errors are non-fatal.
//
    switch (err) {
    case -ENODATA:
    reason = "unsigned module";
    break;
    case -ENOPKG:
    reason = "module with unsupported crypto";
    break;
    case -ENOKEY:
    reason = "module with unavailable key";
    break;
    default:
//
// All other errors are fatal, including lack of memory,
// unparseable signatures, and signature check failures --
// even if signatures aren't required.
//
    return err;
    }
    if (is_module_sig_enforced()) {
    pr_notice("Loading of %s is rejected\n", reason);
    return -EKEYREJECTED;
    }
    return security_locked_down(LOCKDOWN_MODULE_SIGNATURE);
    }
