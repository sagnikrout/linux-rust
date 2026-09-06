//! Automatically rewritten from C to Rust
//! Source: fs/hpfs/dentry.c
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
// linux/fs/hpfs/dentry.c
//
// Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
//
// dcache operations
//

//
// Note: the dentry argument is the parent dentry.
//
#[no_mangle]
unsafe extern "C" fn hpfs_hash_dentry(dentry: *const dentry, qstr: *mut qstr) -> c_int {
    static int hpfs_hash_dentry(const struct dentry *dentry, struct qstr *qstr)
    {
    unsigned long	 hash;
    int		 i;
    let mut l: unsigned = qstr.len;
    if (l == 1) if (qstr.name[0]=='.') goto x;
    if (l == 2) if (qstr.name[0]=='.' || qstr.name[1]=='.') goto x;
    hpfs_adjust_length(qstr.name, &l);
// if (hpfs_chk_name(qstr->name,&l))
// return -ENAMETOOLONG;
// return -ENOENT;
    x:
    hash = init_name_hash(dentry);
    for (i = 0; i < l; i++)
    hash = partial_name_hash(hpfs_upcase(hpfs_sb(dentry.d_sb).sb_cp_table,qstr.name[i]), hash);
    qstr.hash = end_name_hash(hash);
    return 0;
    }
    static int hpfs_compare_dentry(const struct dentry *dentry,
    unsigned int len, const char *str, const struct qstr *name)
    {
    let mut al: unsigned = len;
    let mut bl: unsigned = name.len;
    hpfs_adjust_length(str, &al);
// hpfs_adjust_length(b->name, &bl);
//
// 'str' is the nane of an already existing dentry, so the name
// must be valid. 'name' must be validated first.
//
    if (hpfs_chk_name(name.name, &bl))
    return 1;
    if (hpfs_compare_names(dentry.d_sb, str, al, name.name, bl, 0))
    return 1;
    return 0;
    }
    const struct dentry_operations hpfs_dentry_operations = {
    .d_hash		= hpfs_hash_dentry,
    .d_compare	= hpfs_compare_dentry,
    };
