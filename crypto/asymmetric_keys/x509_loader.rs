//! Automatically rewritten from C to Rust
//! Source: crypto/asymmetric_keys/x509_loader.c
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

    int x509_load_certificate_list(const u8 cert_list[],
    const unsigned long list_size,
    const struct key *keyring)
    {
    key_ref_t key;
    const u8 *p, *end;
    size_t plen;
    p = cert_list;
    end = p + list_size;
    while (p < end) {
// Each cert begins with an ASN.1 SEQUENCE tag and must be more
// than 256 bytes in size.
//
    if (end - p < 4)
    goto dodgy_cert;
    if (p[0] != 0x30 ||
    p[1] != 0x82)
    goto dodgy_cert;
    plen = (p[2] << 8) | p[3];
    plen += 4;
    if (plen > end - p)
    goto dodgy_cert;
    key = key_create_or_update(make_key_ref(keyring, 1),
    "asymmetric",
    core::ptr::null_mut(),
    p,
    plen,
    ((KEY_POS_ALL & ~KEY_POS_SETATTR) |
    KEY_USR_VIEW | KEY_USR_READ),
    KEY_ALLOC_NOT_IN_QUOTA |
    KEY_ALLOC_BUILT_IN |
    KEY_ALLOC_BYPASS_RESTRICTION);
    if (IS_ERR(key)) {
    pr_err("Problem loading in-kernel X.509 certificate (%ld)\n",
    PTR_ERR(key));
    } else {
    pr_notice("Loaded X.509 cert '%s'\n",
    key_ref_to_ptr(key).description);
    key_ref_put(key);
    }
    p += plen;
    }
    return 0;
    dodgy_cert:
    pr_err("Problem parsing in-kernel X.509 certificate list\n");
    return 0;
    }
    EXPORT_SYMBOL_GPL(x509_load_certificate_list);
