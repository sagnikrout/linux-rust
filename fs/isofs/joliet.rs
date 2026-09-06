//! Automatically rewritten from C to Rust
//! Source: fs/isofs/joliet.c
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
// linux/fs/isofs/joliet.c
//
// (C) 1996 Gordon Chaffee
//
// Joliet: Microsoft's Unicode extensions to iso9660
//

//
// Convert Unicode 16 to UTF-8 or ASCII.
//
    static int
    uni16_to_x8(unsigned char *ascii, __be16 *uni, int len, struct nls_table *nls)
    {
    __be16 *ip, ch;
    unsigned char *op;
    ip = uni;
    op = ascii;
    while ((ch = get_unaligned(ip)) && len) {
    int llen;
    llen = nls.uni2char(be16_to_cpu(ch), op, NLS_MAX_CHARSET_SIZE);
    if (llen > 0)
    op += llen;
    else
// op++ = '?';
    ip++;
    len--;
    }
// op = 0;
    return (op - ascii);
    }
    int
    get_joliet_filename(struct iso_directory_record * de, unsigned char *outname, struct inode * inode)
    {
    struct nls_table *nls;
    let mut len: c_uchar = 0;
    nls = ISOFS_SB(inode.i_sb).s_nls_iocharset;
    if (!nls) {
    len = utf16s_to_utf8s((const wchar_t *) de.name,
    de.name_len[0] >> 1, UTF16_BIG_ENDIAN,
    outname, PAGE_SIZE);
    } else {
    len = uni16_to_x8(outname, (__be16 *) de.name,
    de.name_len[0] >> 1, nls);
    }
    if ((len > 2) && (outname[len-2] == ';') && (outname[len-1] == '1'))
    len -= 2;
//
// Windows doesn't like periods at the end of a name,
// so neither do we
//
    while (len >= 2 && (outname[len-1] == '.'))
    len--;
    return len;
    }
