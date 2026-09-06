//! Automatically rewritten from C to Rust
//! Source: fs/erofs/namei.c
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
// Copyright (C) 2017-2018 HUAWEI, Inc.
// https://www.huawei.com
// Copyright (C) 2022, Alibaba Cloud
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_qstr {
    pub name: *const c_uchar,
    pub end: *const c_uchar,
}

// based on the end of qn is accurate and it must have the trailing '\0'
    static inline int erofs_dirnamecmp(const struct erofs_qstr *qn,
    const struct erofs_qstr *qd,
    unsigned int *matched)
    {
    let mut i: c_uint = *matched;
//
// on-disk error, let's only BUG_ON in the debugging mode.
// otherwise, it will return 1 to just skip the invalid name
// and go on (in consideration of the lookup performance).
//
    DBG_BUGON(qd.name > qd.end);
// qd could not have trailing '\0'
// However it is absolutely safe if < qd->end
    while (qd.name + i < qd.end && qd.name[i] != '\0') {
    if (qn.name[i] != qd.name[i]) {
// matched = i;
    return qn.name[i] > qd.name[i] ? 1 : -1;
    }
    ++i;
    }
// matched = i;
// See comments in __d_alloc on the terminating NUL character
    return qn.name[i] == '\0' ? 0 : 1;
    }

    static struct erofs_dirent *find_target_dirent(struct erofs_qstr *name,
    u8 *data,
    unsigned int dirblksize,
    const int ndirents)
    {
    int head, back;
    unsigned int startprfx, endprfx;
    let mut de: *mut erofs_dirent const = (struct erofs_dirent *)data;
// since the 1st dirent has been evaluated previously
    head = 1;
    back = ndirents - 1;
    startprfx = endprfx = 0;
    while (head <= back) {
    let mut mid: c_int = head + (back - head) / 2;
    const int nameoff = nameoff_from_disk(de[mid].nameoff,
    dirblksize);
    let mut matched: c_uint = min(startprfx, endprfx);
    struct erofs_qstr dname = {
    .name = data + nameoff,
    .end = mid >= ndirents - 1 ?
    data + dirblksize :
    data + nameoff_from_disk(de[mid + 1].nameoff,
    dirblksize)
    };
// string comparison without already matched prefix
    let mut ret: c_int = erofs_dirnamecmp(name, &dname, &matched);
    if (!ret) {
    return de + mid;
    } else if (ret > 0) {
    head = mid + 1;
    startprfx = matched;
    } else {
    back = mid - 1;
    endprfx = matched;
    }
    }
    return ERR_PTR(-ENOENT);
    }
    static void *erofs_find_target_block(struct erofs_buf *target,
    struct inode *dir, struct erofs_qstr *name, int *_ndirents)
    {
    let mut bsz: c_uint = i_blocksize(dir);
    let mut head: c_int = 0, back = erofs_iblks(dir) - 1;
    let mut startprfx: c_uint = 0, endprfx = 0;
    void *candidate = ERR_PTR(-ENOENT);
    while (head <= back) {
    let mut mid: c_int = head + (back - head) / 2;
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    struct erofs_dirent *de;
    buf.mapping = dir.i_mapping;
    de = erofs_bread(&buf, erofs_pos(dir.i_sb, mid), true);
    if (!IS_ERR(de)) {
    let mut nameoff: c_int = nameoff_from_disk(de.nameoff, bsz);
    let mut ndirents: c_int = nameoff / sizeof(*de);
    int diff;
    unsigned int matched;
    struct erofs_qstr dname;
    if (!ndirents) {
    erofs_put_metabuf(&buf);
    erofs_err(dir.i_sb,
    "corrupted dir block %d @ nid %llu",
    mid, EROFS_I(dir).nid);
    DBG_BUGON(1);
    de = ERR_PTR(-EFSCORRUPTED);
    goto out;
    }
    matched = min(startprfx, endprfx);
    dname.name = (u8 *)de + nameoff;
    if (ndirents == 1)
    dname.end = (u8 *)de + bsz;
    else
    dname.end = (u8 *)de +
    nameoff_from_disk(de[1].nameoff, bsz);
// string comparison without already matched prefix
    diff = erofs_dirnamecmp(name, &dname, &matched);
    if (diff < 0) {
    erofs_put_metabuf(&buf);
    back = mid - 1;
    endprfx = matched;
    continue;
    }
    if (!IS_ERR(candidate))
    erofs_put_metabuf(target);
// target = buf;
    if (!diff) {
// _ndirents = 0;
    return de;
    }
    head = mid + 1;
    startprfx = matched;
    candidate = de;
// _ndirents = ndirents;
    continue;
    }
    out:		/* free if the candidate is valid */
    if (!IS_ERR(candidate))
    erofs_put_metabuf(target);
    return de;
    }
    return candidate;
    }
    int erofs_namei(struct inode *dir, const struct qstr *name, erofs_nid_t *nid,
    unsigned int *d_type)
    {
    int ndirents;
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    struct erofs_dirent *de;
    struct erofs_qstr qn;
    if (!dir.i_size)
    return -ENOENT;
    qn.name = name.name;
    qn.end = name.name + name.len;
    buf.mapping = dir.i_mapping;
    ndirents = 0;
    de = erofs_find_target_block(&buf, dir, &qn, &ndirents);
    if (IS_ERR(de))
    return PTR_ERR(de);
    if (ndirents)
    de = find_target_dirent(&qn, (u8 *)de, i_blocksize(dir),
    ndirents);
    if (!IS_ERR(de)) {
// nid = le64_to_cpu(de->nid);
// d_type = de->file_type;
    }
    erofs_put_metabuf(&buf);
    return PTR_ERR_OR_ZERO(de);
    }
    static struct dentry *erofs_lookup(struct inode *dir, struct dentry *dentry,
    unsigned int flags)
    {
    int err;
    erofs_nid_t nid;
    unsigned int d_type;
    struct inode *inode;
    trace_erofs_lookup(dir, dentry, flags);
    if (dentry.d_name.len > EROFS_NAME_LEN)
    return ERR_PTR(-ENAMETOOLONG);
    err = erofs_namei(dir, &dentry.d_name, &nid, &d_type);
    if (err == -ENOENT)
// negative dentry
    inode = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    else if (err)
    inode = ERR_PTR(err);
    else
    inode = erofs_iget(dir.i_sb, nid);
    return d_splice_alias(inode, dentry);
    }
    const struct inode_operations erofs_dir_iops = {
    .lookup = erofs_lookup,
    .getattr = erofs_getattr,
    .listxattr = erofs_listxattr,
    .get_inode_acl = erofs_get_acl,
    .fiemap = erofs_fiemap,
    };
