//! Automatically rewritten from C to Rust
//! Source: fs/afs/dir_search.c
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
// Search a directory's hash table.
//
// Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// https://tools.ietf.org/html/draft-keiser-afs3-directory-object-00
//

//
// Calculate the name hash.
//
#[no_mangle]
pub unsafe extern "C" fn afs_dir_hash_name(name: *const qstr) -> c_uint {
    unsigned int afs_dir_hash_name(const struct qstr *name)
    {
    const unsigned char *p = name.name;
    let mut hash: c_uint = 0, i;
    int bucket;
    for (i = 0; i < name.len; i++)
    hash = (hash * 173) + p[i];
    bucket = hash & (AFS_DIR_HASHTBL_SIZE - 1);
    if (hash > INT_MAX) {
    bucket = AFS_DIR_HASHTBL_SIZE - bucket;
    bucket &= (AFS_DIR_HASHTBL_SIZE - 1);
    }
    return bucket;
    }
//
// Reset a directory iterator.
//
#[no_mangle]
unsafe extern "C" fn afs_dir_reset_iter(iter: *mut afs_dir_iter) -> bool {
    static bool afs_dir_reset_iter(struct afs_dir_iter *iter)
    {
    let mut i_size: c_ulonglong = i_size_read(&iter.dvnode.netfs.inode);
    unsigned int nblocks;
// Work out the maximum number of steps we can take.
    nblocks = umin(i_size / AFS_DIR_BLOCK_SIZE, AFS_DIR_MAX_BLOCKS);
    if (!nblocks)
    return false;
    iter.loop_check = nblocks * (AFS_DIR_SLOTS_PER_BLOCK - AFS_DIR_RESV_BLOCKS);
    iter.prev_entry = 0; /* Hash head is previous */
    return true;
    }
//
// Initialise a directory iterator for looking up a name.
//
#[no_mangle]
pub unsafe extern "C" fn afs_dir_init_iter(iter: *mut afs_dir_iter, name: *const qstr) -> bool {
    bool afs_dir_init_iter(struct afs_dir_iter *iter, const struct qstr *name)
    {
    iter.nr_slots = afs_dir_calc_slots(name.len);
    iter.bucket = afs_dir_hash_name(name);
    return afs_dir_reset_iter(iter);
    }
//
// Get a specific block.
//
    union afs_xdr_dir_block *afs_dir_find_block(struct afs_dir_iter *iter, size_t block)
    {
    struct folio_queue *fq = iter.fq;
    struct afs_vnode *dvnode = iter.dvnode;
    struct folio *folio;
    let mut blpos: usize = block * AFS_DIR_BLOCK_SIZE;
    let mut blend: usize = (block + 1) * AFS_DIR_BLOCK_SIZE, fpos = iter.fpos;
    let mut slot: c_int = iter.fq_slot;
    _enter("%zx,%d", block, slot);
    if (iter.block) {
    kunmap_local(iter.block);
    iter.block = core::ptr::null_mut();
    }
    if (dvnode.directory_size < blend)
    goto fail;
    if (!fq || blpos < fpos) {
    fq = dvnode.directory;
    slot = 0;
    fpos = 0;
    }
// Search the folio queue for the folio containing the block...
    for (; fq; fq = fq.next) {
    for (; slot < folioq_count(fq); slot++) {
    let mut fsize: usize = folioq_folio_size(fq, slot);
    if (blend <= fpos + fsize) {
// ... and then return the mapped block.
    folio = folioq_folio(fq, slot);
    if (WARN_ON_ONCE(folio_pos(folio) != fpos))
    goto fail;
    iter.fq = fq;
    iter.fq_slot = slot;
    iter.fpos = fpos;
    iter.block = kmap_local_folio(folio, blpos - fpos);
    return iter.block;
    }
    fpos += fsize;
    }
    slot = 0;
    }
    fail:
    iter.fq = core::ptr::null_mut();
    iter.fq_slot = 0;
    afs_invalidate_dir(dvnode, afs_dir_invalid_edit_get_block);
    return core::ptr::null_mut();
    }
//
// Search through a directory bucket.
//
    int afs_dir_search_bucket(struct afs_dir_iter *iter, const struct qstr *name,
    struct afs_fid *_fid)
    {
    const union afs_xdr_dir_block *meta;
    unsigned int entry;
    let mut ret: c_int = -ESTALE;
    meta = afs_dir_find_block(iter, 0);
    if (!meta)
    return -ESTALE;
    entry = ntohs(meta.meta.hashtable[iter.bucket & (AFS_DIR_HASHTBL_SIZE - 1)]);
    _enter("%x,%x", iter.bucket, entry);
    while (entry) {
    const union afs_xdr_dir_block *block;
    const union afs_xdr_dirent *dire;
    let mut blnum: c_uint = entry / AFS_DIR_SLOTS_PER_BLOCK;
    let mut slot: c_uint = entry % AFS_DIR_SLOTS_PER_BLOCK;
    let mut resv: c_uint = (blnum == 0 ? AFS_DIR_RESV_BLOCKS0 : AFS_DIR_RESV_BLOCKS);
    _debug("search %x", entry);
    if (slot < resv) {
    kdebug("slot out of range h=%x rs=%2x sl=%2x-%2x",
    iter.bucket, resv, slot, slot + iter.nr_slots - 1);
    goto bad;
    }
    block = afs_dir_find_block(iter, blnum);
    if (!block)
    goto bad;
    dire = &block.dirents[slot];
    if (slot + iter.nr_slots <= AFS_DIR_SLOTS_PER_BLOCK &&
    memcmp(dire.u.name, name.name, name.len) == 0 &&
    dire.u.name[name.len] == '\0') {
    _fid.vnode  = ntohl(dire.u.vnode);
    _fid.unique = ntohl(dire.u.unique);
    ret = entry;
    goto found;
    }
    iter.prev_entry = entry;
    entry = ntohs(dire.u.hash_next);
    if (!--iter.loop_check) {
    kdebug("dir chain loop h=%x", iter.bucket);
    goto bad;
    }
    }
    ret = -ENOENT;
    found:
    if (iter.block) {
    kunmap_local(iter.block);
    iter.block = core::ptr::null_mut();
    }
    bad:
    if (ret == -ESTALE)
    afs_invalidate_dir(iter.dvnode, afs_dir_invalid_iter_stale);
    _leave(" = %d", ret);
    return ret;
    }
//
// Search the appropriate hash chain in the contents of an AFS directory.
//
    int afs_dir_search(struct afs_vnode *dvnode, const struct qstr *name,
    struct afs_fid *_fid, afs_dataversion_t *_dir_version)
    {
    let mut iter: afs_dir_iter = { .dvnode = dvnode, };
    int ret, retry_limit = 3;
    _enter("{%llu},,,", dvnode.netfs.inode.i_ino);
    if (!afs_dir_init_iter(&iter, name))
    return -ENOENT;
    do {
    if (--retry_limit < 0) {
    pr_warn("afs_read_dir(): Too many retries\n");
    ret = -ESTALE;
    break;
    }
    ret = afs_read_dir(dvnode, core::ptr::null_mut());
    if (ret < 0) {
    if (ret != -ESTALE)
    break;
    if (test_bit(AFS_VNODE_DELETED, &dvnode.flags)) {
    ret = -ESTALE;
    break;
    }
    continue;
    }
// _dir_version = inode_peek_iversion_raw(&dvnode->netfs.inode);
    ret = afs_dir_search_bucket(&iter, name, _fid);
    up_read(&dvnode.validate_lock);
    if (ret == -ESTALE)
    afs_dir_reset_iter(&iter);
    } while (ret == -ESTALE);
    _leave(" = %d", ret);
    return ret;
    }
