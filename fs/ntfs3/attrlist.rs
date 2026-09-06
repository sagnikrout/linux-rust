//! Automatically rewritten from C to Rust
//! Source: fs/ntfs3/attrlist.c
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
// Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
//

//
// al_is_valid_le
//
// Return: True if @le is valid.
//
    static inline bool al_is_valid_le(const struct ntfs_inode *ni,
    struct ATTR_LIST_ENTRY *le)
    {
    ni = ni.base;
    if (!le || !ni.attr_list.le || !ni.attr_list.size)
    return false;
    return PtrOffset(ni.attr_list.le, le) + le16_to_cpu(le.size) <=
    ni.attr_list.size;
    }
#[no_mangle]
pub unsafe extern "C" fn al_destroy(ni: *mut ntfs_inode) {
    void al_destroy(struct ntfs_inode *ni)
    {
    ni = ni.base;
    run_close(&ni.attr_list.run);
    kvfree(ni.attr_list.le);
    ni.attr_list.le = core::ptr::null_mut();
    ni.attr_list.size = 0;
    ni.attr_list.dirty = false;
    }
//
// ntfs_load_attr_list
//
// This method makes sure that the ATTRIB list, if present,
// has been properly set up.
//
#[no_mangle]
pub unsafe extern "C" fn ntfs_load_attr_list(ni: *mut ntfs_inode, attr: *mut ATTRIB) -> c_int {
    int ntfs_load_attr_list(struct ntfs_inode *ni, struct ATTRIB *attr)
    {
    int err;
    size_t lsize;
    void *le = core::ptr::null_mut();
    ni = ni.base;
    if (ni.attr_list.size)
    return 0;
    if (!attr.non_res) {
    lsize = le32_to_cpu(attr.res.data_size);
    if (!lsize) {
    err = -EINVAL;
    goto out;
    }
// attr is resident: lsize < record_size (1K or 4K)
    le = kvmalloc(al_aligned(lsize), GFP_KERNEL);
    if (!le) {
    err = -ENOMEM;
    goto out;
    }
    memcpy(le, resident_data(attr), lsize);
    } else if (attr.nres.svcn) {
    err = -EINVAL;
    goto out;
    } else {
    let mut run_off: u16 = le16_to_cpu(attr.nres.run_off);
    lsize = le64_to_cpu(attr.nres.data_size);
    if (!lsize) {
    err = -EINVAL;
    goto out;
    }
    run_init(&ni.attr_list.run);
    if (run_off > le32_to_cpu(attr.size)) {
    err = -EINVAL;
    goto out;
    }
    err = run_unpack_ex(&ni.attr_list.run, ni.mi.sbi, ni.mi.rno,
    0, le64_to_cpu(attr.nres.evcn), 0,
    Add2Ptr(attr, run_off),
    le32_to_cpu(attr.size) - run_off);
    if (err < 0)
    goto out;
// attr is nonresident.
// The worst case:
// 1T (2^40) extremely fragmented file.
// cluster = 4K (2^12) => 2^28 fragments
// 2^9 fragments per one record => 2^19 records
// 2^5 bytes of ATTR_LIST_ENTRY per one record => 2^24 bytes.
//
// the result is 16M bytes per attribute list.
// Use kvmalloc to allocate in range [several Kbytes - dozen Mbytes]
//
    le = kvmalloc(al_aligned(lsize), GFP_KERNEL);
    if (!le) {
    err = -ENOMEM;
    goto out;
    }
    err = ntfs_read_run_nb(ni.mi.sbi, &ni.attr_list.run, 0, le,
    lsize, core::ptr::null_mut());
    if (err)
    goto out;
    }
    ni.attr_list.size = lsize;
    ni.attr_list.le = le;
    return 0;
    out:
    ni.attr_list.le = le;
    al_destroy(ni);
    return err;
    }
//
// al_enumerate
//
// Return:
// * The next list le.
// * If @le is NULL then return the first le.
//
    struct ATTR_LIST_ENTRY *al_enumerate(struct ntfs_inode *ni,
    struct ATTR_LIST_ENTRY *le)
    {
    size_t off;
    u16 sz;
    let mut le_min_size: unsigned = le_size(0);
    if (!le) {
    le = ni.attr_list.le;
    } else {
    sz = le16_to_cpu(le.size);
    if (sz < le_min_size) {
// Impossible 'cause we should not return such le.
    return core::ptr::null_mut();
    }
    le = Add2Ptr(le, sz);
    }
// Check boundary.
    off = PtrOffset(ni.attr_list.le, le);
    if (off + le_min_size > ni.attr_list.size) {
// The regular end of list.
    return core::ptr::null_mut();
    }
    sz = le16_to_cpu(le.size);
// Check le for errors.
    if (sz < le_min_size || off + sz > ni.attr_list.size ||
    sz < le.name_off + le.name_len * sizeof(short)) {
    return core::ptr::null_mut();
    }
    return le;
    }
//
// al_find_le
//
// Find the first le in the list which matches type, name and VCN.
//
// Return: NULL if not found.
//
    struct ATTR_LIST_ENTRY *al_find_le(struct ntfs_inode *ni,
    struct ATTR_LIST_ENTRY *le,
    const struct ATTRIB *attr)
    {
    let mut svcn: CLST = attr_svcn(attr);
    return al_find_ex(ni, le, attr.type, attr_name(attr), attr.name_len,
    &svcn);
    }
//
// al_find_ex
//
// Find the first le in the list which matches type, name and VCN.
//
// Return: NULL if not found.
//
    struct ATTR_LIST_ENTRY *al_find_ex(struct ntfs_inode *ni,
    struct ATTR_LIST_ENTRY *le,
    enum ATTR_TYPE type, const __le16 *name,
    u8 name_len, const CLST *vcn)
    {
    struct ATTR_LIST_ENTRY *ret = core::ptr::null_mut();
    let mut type_in: u32 = le32_to_cpu(type);
    ni = ni.base;
    while ((le = al_enumerate(ni, le))) {
    u64 le_vcn;
    let mut diff: c_int = le32_to_cpu(le.type) - type_in;
// List entries are sorted by type, name and VCN.
    if (diff < 0)
    continue;
    if (diff > 0)
    return ret;
    if (le.name_len != name_len)
    continue;
    le_vcn = le64_to_cpu(le.vcn);
    if (!le_vcn) {
//
// Compare entry names only for entry with vcn == 0.
//
    diff = ntfs_cmp_names(le_name(le), name_len, name,
    name_len, ni.mi.sbi.upcase,
    true);
    if (diff < 0)
    continue;
    if (diff > 0)
    return ret;
    }
    if (!vcn)
    return le;
    if (*vcn == le_vcn)
    return le;
    if (*vcn < le_vcn)
    return ret;
    ret = le;
    }
    return ret;
    }
//
// al_find_le_to_insert
//
// Find the first list entry which matches type, name and VCN.
//
    static struct ATTR_LIST_ENTRY *al_find_le_to_insert(struct ntfs_inode *ni,
    enum ATTR_TYPE type,
    const __le16 *name,
    u8 name_len, CLST vcn)
    {
    struct ATTR_LIST_ENTRY *le = core::ptr::null_mut(), *prev;
    let mut type_in: u32 = le32_to_cpu(type);
    ni = ni.base;
// List entries are sorted by type, name and VCN.
    while ((le = al_enumerate(ni, prev = le))) {
    let mut diff: c_int = le32_to_cpu(le.type) - type_in;
    if (diff < 0)
    continue;
    if (diff > 0)
    return le;
    if (!le.vcn) {
//
// Compare entry names only for entry with vcn == 0.
//
    diff = ntfs_cmp_names(le_name(le), le.name_len, name,
    name_len, ni.mi.sbi.upcase,
    true);
    if (diff < 0)
    continue;
    if (diff > 0)
    return le;
    }
    if (le64_to_cpu(le.vcn) >= vcn)
    return le;
    }
    return prev ? Add2Ptr(prev, le16_to_cpu(prev.size)) : ni.attr_list.le;
    }
//
// al_add_le
//
// Add an "attribute list entry" to the list.
//
    int al_add_le(struct ntfs_inode *ni, enum ATTR_TYPE type, const __le16 *name,
    u8 name_len, CLST svcn, __le16 id, const struct MFT_REF *ref,
    struct ATTR_LIST_ENTRY **new_le)
    {
    int err;
    struct ATTRIB *attr;
    struct ATTR_LIST_ENTRY *le;
    size_t off;
    u16 sz;
    size_t asize, new_asize, old_size;
    u64 new_size;
    typeof(ni.attr_list) *al = &ni.attr_list;
    ni = ni.base;
//
// Compute the size of the new 'le'
//
    sz = le_size(name_len);
    old_size = al.size;
    new_size = old_size + sz;
    asize = al_aligned(old_size);
    new_asize = al_aligned(new_size);
// Scan forward to the point at which the new 'le' should be inserted.
    le = al_find_le_to_insert(ni, type, name, name_len, svcn);
    off = PtrOffset(al.le, le);
    if (new_size > asize) {
    void *ptr = kmalloc(new_asize, GFP_NOFS);
    if (!ptr)
    return -ENOMEM;
    memcpy(ptr, al.le, off);
    memcpy(Add2Ptr(ptr, off + sz), le, old_size - off);
    le = Add2Ptr(ptr, off);
    kvfree(al.le);
    al.le = ptr;
    } else {
    memmove(Add2Ptr(le, sz), le, old_size - off);
    }
// new_le = le;
    al.size = new_size;
    le.type = type;
    le.size = cpu_to_le16(sz);
    le.name_len = name_len;
    le.name_off = offsetof(struct ATTR_LIST_ENTRY, name);
    le.vcn = cpu_to_le64(svcn);
    le.ref = *ref;
    le.id = id;
    memcpy(le.name, name, sizeof(short) * name_len);
    err = attr_set_size_ex(ni, ATTR_LIST, core::ptr::null_mut(), 0, &al.run, new_size,
    &new_size, true, &attr, false);
    if (err) {
// Undo memmove above.
    memmove(le, Add2Ptr(le, sz), old_size - off);
    al.size = old_size;
    return err;
    }
    al.dirty = true;
    if (attr && attr.non_res) {
    err = ntfs_sb_write_run(ni.mi.sbi, &al.run, 0, al.le,
    al.size, 0);
    if (err)
    return err;
    al.dirty = false;
    }
    return 0;
    }
//
// al_remove_le - Remove @le from attribute list.
//
#[no_mangle]
pub unsafe extern "C" fn al_remove_le(ni: *mut ntfs_inode, le: *mut ATTR_LIST_ENTRY) -> bool {
    bool al_remove_le(struct ntfs_inode *ni, struct ATTR_LIST_ENTRY *le)
    {
    u16 size;
    size_t off;
    typeof(ni.attr_list) *al;
    ni = ni.base;
    al = &ni.attr_list;
    if (!al_is_valid_le(ni, le))
    return false;
// Save on stack the size of 'le'
    size = le16_to_cpu(le.size);
    off = PtrOffset(al.le, le);
    memmove(le, Add2Ptr(le, size), al.size - (off + size));
    al.size -= size;
    al.dirty = true;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn al_update(ni: *mut ntfs_inode, sync: c_int) -> c_int {
    int al_update(struct ntfs_inode *ni, int sync)
    {
    int err;
    struct ATTRIB *attr;
    typeof(ni.attr_list) *al;
    ni = ni.base;
    al = &ni.attr_list;
    if (!al.dirty || !al.size)
    return 0;
//
// Attribute list increased on demand in al_add_le.
// Attribute list decreased here.
//
    err = attr_set_size_ex(ni, ATTR_LIST, core::ptr::null_mut(), 0, &al.run, al.size, core::ptr::null_mut(),
    false, &attr, false);
    if (err)
    goto out;
    if (!attr.non_res) {
    memcpy(resident_data(attr), al.le, al.size);
    } else {
    err = ntfs_sb_write_run(ni.mi.sbi, &al.run, 0, al.le,
    al.size, sync);
    if (err)
    goto out;
    attr.nres.valid_size = attr.nres.data_size;
    }
    ni.mi.dirty = true;
    al.dirty = false;
    out:
    return err;
    }
