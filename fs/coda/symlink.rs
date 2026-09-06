//! Automatically rewritten from C to Rust
//! Source: fs/coda/symlink.c
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
// Symlink inode operations for Coda filesystem
// Original version: (C) 1996 P. Braam and M. Callahan
// Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
//
// Carnegie Mellon encourages users to contribute improvements to
// the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
//

#[no_mangle]
unsafe extern "C" fn coda_symlink_filler(file: *mut file, folio: *mut folio) -> c_int {
    static int coda_symlink_filler(struct file *file, struct folio *folio)
    {
    struct inode *inode = folio.mapping.host;
    int error;
    struct coda_inode_info *cii;
    let mut len: c_uint = PAGE_SIZE;
    char *p = folio_address(folio);
    cii = ITOC(inode);
    error = venus_readlink(inode.i_sb, &cii.c_fid, p, &len);
    folio_end_read(folio, error == 0);
    return error;
    }
    const struct address_space_operations coda_symlink_aops = {
    .read_folio	= coda_symlink_filler,
    };
