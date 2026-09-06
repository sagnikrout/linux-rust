//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/smbencrypt.c
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
//
    Unix SMB/Netbios implementation.
    Version 1.9.
    SMB parameters and setup
    Copyright (C) Andrew Tridgell 1992-2000
    Copyright (C) Luke Kenneth Casson Leighton 1996-2000
    Modified by Jeremy Allison 1995.
    Copyright (C) Andrew Bartlett <abartlet@samba.org> 2002-2003
    Modified by Steve French (sfrench@us.ibm.com) 2002-2003
//

// following came from the other byteorder.h to avoid include conflicts

// produce a md4 message digest from data of length n bytes
    static int
    mdfour(unsigned char *md4_hash, unsigned char *link_str, int link_len)
    {
    int rc;
    struct md4_ctx mctx;
    rc = cifs_md4_init(&mctx);
    if (rc) {
    cifs_dbg(VFS, "%s: Could not init MD4\n", __func__);
    goto mdfour_err;
    }
    rc = cifs_md4_update(&mctx, link_str, link_len);
    if (rc) {
    cifs_dbg(VFS, "%s: Could not update MD4\n", __func__);
    goto mdfour_err;
    }
    rc = cifs_md4_final(&mctx, md4_hash);
    if (rc)
    cifs_dbg(VFS, "%s: Could not finalize MD4\n", __func__);
    mdfour_err:
    return rc;
    }
//
// Creates the MD4 Hash of the users password in NT UNICODE.
//
    int
    E_md4hash(const unsigned char *passwd, unsigned char *p16,
    const struct nls_table *codepage)
    {
    int rc;
    int len;
    __le16 wpwd[129];
// Password cannot be longer than 128 characters
    if (passwd) /* Password must be converted to NT unicode */
    len = cifs_strtoUTF16(wpwd, passwd, 128, codepage);
    else {
    len = 0;
// wpwd = 0; /* Ensure string is null terminated
    }
    rc = mdfour(p16, (unsigned char *) wpwd, len * sizeof(__le16));
    memzero_explicit(wpwd, sizeof(wpwd));
    return rc;
    }
