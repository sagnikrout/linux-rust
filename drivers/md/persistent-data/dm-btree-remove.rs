//! Automatically rewritten from C to Rust
//! Source: drivers/md/persistent-data/dm-btree-remove.c
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
// Copyright (C) 2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

//
// Removing an entry from a btree
// ==============================
//
// A very important constraint for our btree is that no node, except the
// root, may have fewer than a certain number of entries.
// (MIN_ENTRIES <= nr_entries <= MAX_ENTRIES).
//
// Ensuring this is complicated by the way we want to only ever hold the
// locks on 2 nodes concurrently, and only change nodes in a top to bottom
// fashion.
//
// Each node may have a left or right sibling.  When decending the spine,
// if a node contains only MIN_ENTRIES then we try and increase this to at
// least MIN_ENTRIES + 1.  We do this in the following ways:
//
// [A] No siblings => this can only happen if the node is the root, in which
// case we copy the childs contents over the root.
//
// [B] No left sibling
// ==> rebalance(node, right sibling)
//
// [C] No right sibling
// ==> rebalance(left sibling, node)
//
// [D] Both siblings, total_entries(left, node, right) <= DEL_THRESHOLD
// ==> delete node adding it's contents to left and right
//
// [E] Both siblings, total_entries(left, node, right) > DEL_THRESHOLD
// ==> rebalance(left, node, right)
//
// After these operations it's possible that the our original node no
// longer contains the desired sub tree.  For this reason this rebalancing
// is performed on the children of the current node.  This also avoids
// having a special case for the root.
//
// Once this rebalancing has occurred we can then step into the child node
// for internal nodes.  Or delete the entry for leaf nodes.
//
// Some little utilities for moving node data around.
//
#[no_mangle]
unsafe extern "C" fn node_shift(n: *mut btree_node, shift: c_int) {
    static void node_shift(struct btree_node *n, int shift)
    {
    let mut nr_entries: u32 = le32_to_cpu(n.header.nr_entries);
    let mut value_size: u32 = le32_to_cpu(n.header.value_size);
    if (shift < 0) {
    shift = -shift;
    BUG_ON(shift > nr_entries);
    BUG_ON((void *) key_ptr(n, shift) >= value_ptr(n, shift));
    memmove(key_ptr(n, 0),
    key_ptr(n, shift),
    (nr_entries - shift) * sizeof(__le64));
    memmove(value_ptr(n, 0),
    value_ptr(n, shift),
    (nr_entries - shift) * value_size);
    } else {
    BUG_ON(nr_entries + shift > le32_to_cpu(n.header.max_entries));
    memmove(key_ptr(n, shift),
    key_ptr(n, 0),
    nr_entries * sizeof(__le64));
    memmove(value_ptr(n, shift),
    value_ptr(n, 0),
    nr_entries * value_size);
    }
    }
#[no_mangle]
unsafe extern "C" fn node_copy(left: *mut btree_node, right: *mut btree_node, shift: c_int) -> c_int {
    static int node_copy(struct btree_node *left, struct btree_node *right, int shift)
    {
    let mut nr_left: u32 = le32_to_cpu(left.header.nr_entries);
    let mut value_size: u32 = le32_to_cpu(left.header.value_size);
    if (value_size != le32_to_cpu(right.header.value_size)) {
    DMERR("mismatched value size");
    return -EILSEQ;
    }
    if (shift < 0) {
    shift = -shift;
    if (nr_left + shift > le32_to_cpu(left.header.max_entries)) {
    DMERR("bad shift");
    return -EINVAL;
    }
    memcpy(key_ptr(left, nr_left),
    key_ptr(right, 0),
    shift * sizeof(__le64));
    memcpy(value_ptr(left, nr_left),
    value_ptr(right, 0),
    shift * value_size);
    } else {
    if (shift > le32_to_cpu(right.header.max_entries)) {
    DMERR("bad shift");
    return -EINVAL;
    }
    memcpy(key_ptr(right, 0),
    key_ptr(left, nr_left - shift),
    shift * sizeof(__le64));
    memcpy(value_ptr(right, 0),
    value_ptr(left, nr_left - shift),
    shift * value_size);
    }
    return 0;
    }
//
// Delete a specific entry from a leaf node.
//
#[no_mangle]
unsafe extern "C" fn delete_at(n: *mut btree_node, index: c_uint) {
    static void delete_at(struct btree_node *n, unsigned int index)
    {
    let mut nr_entries: c_uint = le32_to_cpu(n.header.nr_entries);
    let mut nr_to_copy: c_uint = nr_entries - (index + 1);
    let mut value_size: u32 = le32_to_cpu(n.header.value_size);
    BUG_ON(index >= nr_entries);
    if (nr_to_copy) {
    memmove(key_ptr(n, index),
    key_ptr(n, index + 1),
    nr_to_copy * sizeof(__le64));
    memmove(value_ptr(n, index),
    value_ptr(n, index + 1),
    nr_to_copy * value_size);
    }
    n.header.nr_entries = cpu_to_le32(nr_entries - 1);
    }
#[no_mangle]
unsafe extern "C" fn merge_threshold(n: *mut btree_node) -> c_uint {
    static unsigned int merge_threshold(struct btree_node *n)
    {
    return le32_to_cpu(n.header.max_entries) / 3;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child {
    pub index: c_uint,
    pub block: *mut dm_block,
    pub n: *mut btree_node,
}

    static int init_child(struct dm_btree_info *info, struct dm_btree_value_type *vt,
    struct btree_node *parent,
    unsigned int index, struct child *result)
    {
    int r, inc;
    dm_block_t root;
    result.index = index;
    root = value64(parent, index);
    r = dm_tm_shadow_block(info.tm, root, &btree_node_validator,
    &result.block, &inc);
    if (r)
    return r;
    result.n = dm_block_data(result.block);
    if (inc)
    inc_children(info.tm, result.n, vt);
// ((__le64 *) value_ptr(parent, index)) =
    cpu_to_le64(dm_block_location(result.block));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_child(info: *mut dm_btree_info, c: *mut child) {
    static void exit_child(struct dm_btree_info *info, struct child *c)
    {
    dm_tm_unlock(info.tm, c.block);
    }
#[no_mangle]
unsafe extern "C" fn shift(left: *mut btree_node, right: *mut btree_node, count: c_int) -> c_int {
    static int shift(struct btree_node *left, struct btree_node *right, int count)
    {
    int r;
    let mut nr_left: u32 = le32_to_cpu(left.header.nr_entries);
    let mut nr_right: u32 = le32_to_cpu(right.header.nr_entries);
    let mut max_entries: u32 = le32_to_cpu(left.header.max_entries);
    let mut r_max_entries: u32 = le32_to_cpu(right.header.max_entries);
    if (max_entries != r_max_entries) {
    DMERR("node max_entries mismatch");
    return -EILSEQ;
    }
    if (nr_left - count > max_entries) {
    DMERR("node shift out of bounds");
    return -EINVAL;
    }
    if (nr_right + count > max_entries) {
    DMERR("node shift out of bounds");
    return -EINVAL;
    }
    if (!count)
    return 0;
    if (count > 0) {
    node_shift(right, count);
    r = node_copy(left, right, count);
    if (r)
    return r;
    } else {
    r = node_copy(left, right, count);
    if (r)
    return r;
    node_shift(right, count);
    }
    left.header.nr_entries = cpu_to_le32(nr_left - count);
    right.header.nr_entries = cpu_to_le32(nr_right + count);
    return 0;
    }
    static int __rebalance2(struct dm_btree_info *info, struct btree_node *parent,
    struct child *l, struct child *r)
    {
    int ret;
    struct btree_node *left = l.n;
    struct btree_node *right = r.n;
    let mut nr_left: u32 = le32_to_cpu(left.header.nr_entries);
    let mut nr_right: u32 = le32_to_cpu(right.header.nr_entries);
//
// Ensure the number of entries in each child will be greater
// than or equal to (max_entries / 3 + 1), so no matter which
// child is used for removal, the number will still be not
// less than (max_entries / 3).
//
    let mut threshold: c_uint = 2 * (merge_threshold(left) + 1);
    if (nr_left + nr_right < threshold) {
//
// Merge
//
    node_copy(left, right, -nr_right);
    left.header.nr_entries = cpu_to_le32(nr_left + nr_right);
    delete_at(parent, r.index);
//
// We need to decrement the right block, but not it's
// children, since they're still referenced by left.
//
    dm_tm_dec(info.tm, dm_block_location(r.block));
    } else {
//
// Rebalance.
//
    let mut target_left: c_uint = (nr_left + nr_right) / 2;
    ret = shift(left, right, nr_left - target_left);
    if (ret)
    return ret;
// key_ptr(parent, r->index) = right->keys[0];
    }
    return 0;
    }
    static int rebalance2(struct shadow_spine *s, struct dm_btree_info *info,
    struct dm_btree_value_type *vt, unsigned int left_index)
    {
    int r;
    struct btree_node *parent;
    struct child left, right;
    parent = dm_block_data(shadow_current(s));
    r = init_child(info, vt, parent, left_index, &left);
    if (r)
    return r;
    r = init_child(info, vt, parent, left_index + 1, &right);
    if (r) {
    exit_child(info, &left);
    return r;
    }
    r = __rebalance2(info, parent, &left, &right);
    exit_child(info, &left);
    exit_child(info, &right);
    return r;
    }
//
// We dump as many entries from center as possible into left, then the rest
// in right, then rebalance2.  This wastes some cpu, but I want something
// simple atm.
//
    static int delete_center_node(struct dm_btree_info *info, struct btree_node *parent,
    struct child *l, struct child *c, struct child *r,
    struct btree_node *left, struct btree_node *center, struct btree_node *right,
    uint32_t nr_left, uint32_t nr_center, uint32_t nr_right)
    {
    let mut max_entries: u32 = le32_to_cpu(left.header.max_entries);
    let mut shift: c_uint = min(max_entries - nr_left, nr_center);
    if (nr_left + shift > max_entries) {
    DMERR("node shift out of bounds");
    return -EINVAL;
    }
    node_copy(left, center, -shift);
    left.header.nr_entries = cpu_to_le32(nr_left + shift);
    if (shift != nr_center) {
    shift = nr_center - shift;
    if ((nr_right + shift) > max_entries) {
    DMERR("node shift out of bounds");
    return -EINVAL;
    }
    node_shift(right, shift);
    node_copy(center, right, shift);
    right.header.nr_entries = cpu_to_le32(nr_right + shift);
    }
// key_ptr(parent, r->index) = right->keys[0];
    delete_at(parent, c.index);
    r.index--;
    dm_tm_dec(info.tm, dm_block_location(c.block));
    return __rebalance2(info, parent, l, r);
    }
//
// Redistributes entries among 3 sibling nodes.
//
    static int redistribute3(struct dm_btree_info *info, struct btree_node *parent,
    struct child *l, struct child *c, struct child *r,
    struct btree_node *left, struct btree_node *center, struct btree_node *right,
    uint32_t nr_left, uint32_t nr_center, uint32_t nr_right)
    {
    int s, ret;
    let mut max_entries: u32 = le32_to_cpu(left.header.max_entries);
    let mut total: c_uint = nr_left + nr_center + nr_right;
    let mut target_right: c_uint = total / 3;
    let mut remainder: c_uint = (target_right * 3) != total;
    let mut target_left: c_uint = target_right + remainder;
    BUG_ON(target_left > max_entries);
    BUG_ON(target_right > max_entries);
    if (nr_left < nr_right) {
    s = nr_left - target_left;
    if (s < 0 && nr_center < -s) {
// not enough in central node
    ret = shift(left, center, -nr_center);
    if (ret)
    return ret;
    s += nr_center;
    ret = shift(left, right, s);
    if (ret)
    return ret;
    nr_right += s;
    } else {
    ret = shift(left, center, s);
    if (ret)
    return ret;
    }
    ret = shift(center, right, target_right - nr_right);
    if (ret)
    return ret;
    } else {
    s = target_right - nr_right;
    if (s > 0 && nr_center < s) {
// not enough in central node
    ret = shift(center, right, nr_center);
    if (ret)
    return ret;
    s -= nr_center;
    ret = shift(left, right, s);
    if (ret)
    return ret;
    nr_left -= s;
    } else {
    ret = shift(center, right, s);
    if (ret)
    return ret;
    }
    ret = shift(left, center, nr_left - target_left);
    if (ret)
    return ret;
    }
// key_ptr(parent, c->index) = center->keys[0];
// key_ptr(parent, r->index) = right->keys[0];
    return 0;
    }
    static int __rebalance3(struct dm_btree_info *info, struct btree_node *parent,
    struct child *l, struct child *c, struct child *r)
    {
    struct btree_node *left = l.n;
    struct btree_node *center = c.n;
    struct btree_node *right = r.n;
    let mut nr_left: u32 = le32_to_cpu(left.header.nr_entries);
    let mut nr_center: u32 = le32_to_cpu(center.header.nr_entries);
    let mut nr_right: u32 = le32_to_cpu(right.header.nr_entries);
    let mut threshold: c_uint = merge_threshold(left) * 4 + 1;
    if ((left.header.max_entries != center.header.max_entries) ||
    (center.header.max_entries != right.header.max_entries)) {
    DMERR("bad btree metadata, max_entries differ");
    return -EILSEQ;
    }
    if ((nr_left + nr_center + nr_right) < threshold) {
    return delete_center_node(info, parent, l, c, r, left, center, right,
    nr_left, nr_center, nr_right);
    }
    return redistribute3(info, parent, l, c, r, left, center, right,
    nr_left, nr_center, nr_right);
    }
    static int rebalance3(struct shadow_spine *s, struct dm_btree_info *info,
    struct dm_btree_value_type *vt, unsigned int left_index)
    {
    int r;
    struct btree_node *parent = dm_block_data(shadow_current(s));
    struct child left, center, right;
//
// FIXME: fill out an array?
//
    r = init_child(info, vt, parent, left_index, &left);
    if (r)
    return r;
    r = init_child(info, vt, parent, left_index + 1, &center);
    if (r) {
    exit_child(info, &left);
    return r;
    }
    r = init_child(info, vt, parent, left_index + 2, &right);
    if (r) {
    exit_child(info, &left);
    exit_child(info, &center);
    return r;
    }
    r = __rebalance3(info, parent, &left, &center, &right);
    exit_child(info, &left);
    exit_child(info, &center);
    exit_child(info, &right);
    return r;
    }
    static int rebalance_children(struct shadow_spine *s,
    struct dm_btree_info *info,
    struct dm_btree_value_type *vt, uint64_t key)
    {
    int i, r, has_left_sibling, has_right_sibling;
    struct btree_node *n;
    n = dm_block_data(shadow_current(s));
    if (le32_to_cpu(n.header.nr_entries) == 1) {
    struct dm_block *child;
    int is_shared;
    let mut b: dm_block_t = value64(n, 0);
    r = dm_tm_block_is_shared(info.tm, b, &is_shared);
    if (r)
    return r;
    r = dm_tm_read_lock(info.tm, b, &btree_node_validator, &child);
    if (r)
    return r;
    if (is_shared)
    inc_children(info.tm, dm_block_data(child), vt);
    memcpy(n, dm_block_data(child),
    dm_bm_block_size(dm_tm_get_bm(info.tm)));
    dm_tm_dec(info.tm, dm_block_location(child));
    dm_tm_unlock(info.tm, child);
    return 0;
    }
    i = lower_bound(n, key);
    if (i < 0)
    return -ENODATA;
    has_left_sibling = i > 0;
    has_right_sibling = i < (le32_to_cpu(n.header.nr_entries) - 1);
    if (!has_left_sibling)
    r = rebalance2(s, info, vt, i);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !has_right_sibling) -> else {
    else if (!has_right_sibling)
    r = rebalance2(s, info, vt, i - 1);
    else
    r = rebalance3(s, info, vt, i - 1);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn do_leaf(n: *mut btree_node, key: u64, index: *mut c_uint) -> c_int {
    static int do_leaf(struct btree_node *n, uint64_t key, unsigned int *index)
    {
    let mut i: c_int = lower_bound(n, key);
    if ((i < 0) ||
    (i >= le32_to_cpu(n.header.nr_entries)) ||
    (le64_to_cpu(n.keys[i]) != key))
    return -ENODATA;
// index = i;
    return 0;
    }
//
// Prepares for removal from one level of the hierarchy.  The caller must
// call delete_at() to remove the entry at index.
//
    static int remove_raw(struct shadow_spine *s, struct dm_btree_info *info,
    struct dm_btree_value_type *vt, dm_block_t root,
    uint64_t key, unsigned int *index)
    {
    let mut i: c_int = *index, r;
    struct btree_node *n;
    for (;;) {
    r = shadow_step(s, root, vt);
    if (r < 0)
    break;
//
// We have to patch up the parent node, ugly, but I don't
// see a way to do this automatically as part of the spine
// op.
//
    if (shadow_has_parent(s)) {
    let mut location: __le64 = cpu_to_le64(dm_block_location(shadow_current(s)));
    memcpy(value_ptr(dm_block_data(shadow_parent(s)), i),
    &location, sizeof(__le64));
    }
    n = dm_block_data(shadow_current(s));
    if (le32_to_cpu(n.header.flags) & LEAF_NODE)
    return do_leaf(n, key, index);
    r = rebalance_children(s, info, vt, key);
    if (r)
    break;
    n = dm_block_data(shadow_current(s));
    if (le32_to_cpu(n.header.flags) & LEAF_NODE)
    return do_leaf(n, key, index);
    i = lower_bound(n, key);
//
// We know the key is present, or else
// rebalance_children would have returned
// -ENODATA
//
    root = value64(n, i);
    }
    return r;
    }
    int dm_btree_remove(struct dm_btree_info *info, dm_block_t root,
    uint64_t *keys, dm_block_t *new_root)
    {
    unsigned int level, last_level = info.levels - 1;
    let mut index: c_int = 0, r = 0;
    struct shadow_spine spine;
    struct btree_node *n;
    struct dm_btree_value_type le64_vt;
    init_le64_type(info.tm, &le64_vt);
    init_shadow_spine(&spine, info);
    for (level = 0; level < info.levels; level++) {
    r = remove_raw(&spine, info,
    (level == last_level ?
    &info.value_type : &le64_vt),
    root, keys[level], (unsigned int *)&index);
    if (r < 0)
    break;
    n = dm_block_data(shadow_current(&spine));
    if (level != last_level) {
    root = value64(n, index);
    continue;
    }
    BUG_ON(index < 0 || index >= le32_to_cpu(n.header.nr_entries));
    if (info.value_type.dec)
    info.value_type.dec(info.value_type.context,
    value_ptr(n, index), 1);
    delete_at(n, index);
    }
    if (!r)
// new_root = shadow_root(&spine);
    exit_shadow_spine(&spine);
    return r;
    }
    EXPORT_SYMBOL_GPL(dm_btree_remove);
// ----------------------------------------------------------------
    static int remove_nearest(struct shadow_spine *s, struct dm_btree_info *info,
    struct dm_btree_value_type *vt, dm_block_t root,
    uint64_t key, int *index)
    {
    let mut i: c_int = *index, r;
    struct btree_node *n;
    for (;;) {
    r = shadow_step(s, root, vt);
    if (r < 0)
    break;
//
// We have to patch up the parent node, ugly, but I don't
// see a way to do this automatically as part of the spine
// op.
//
    if (shadow_has_parent(s)) {
    let mut location: __le64 = cpu_to_le64(dm_block_location(shadow_current(s)));
    memcpy(value_ptr(dm_block_data(shadow_parent(s)), i),
    &location, sizeof(__le64));
    }
    n = dm_block_data(shadow_current(s));
    if (le32_to_cpu(n.header.flags) & LEAF_NODE) {
// index = lower_bound(n, key);
    return 0;
    }
    r = rebalance_children(s, info, vt, key);
    if (r)
    break;
    n = dm_block_data(shadow_current(s));
    if (le32_to_cpu(n.header.flags) & LEAF_NODE) {
// index = lower_bound(n, key);
    return 0;
    }
    i = lower_bound(n, key);
//
// We know the key is present, or else
// rebalance_children would have returned
// -ENODATA
//
    root = value64(n, i);
    }
    return r;
    }
    static int remove_one(struct dm_btree_info *info, dm_block_t root,
    uint64_t *keys, uint64_t end_key,
    dm_block_t *new_root, unsigned int *nr_removed)
    {
    unsigned int level, last_level = info.levels - 1;
    let mut index: c_int = 0, r = 0;
    struct shadow_spine spine;
    struct btree_node *n;
    struct dm_btree_value_type le64_vt;
    uint64_t k;
    init_le64_type(info.tm, &le64_vt);
    init_shadow_spine(&spine, info);
    for (level = 0; level < last_level; level++) {
    r = remove_raw(&spine, info, &le64_vt,
    root, keys[level], (unsigned int *) &index);
    if (r < 0)
    goto out;
    n = dm_block_data(shadow_current(&spine));
    root = value64(n, index);
    }
    r = remove_nearest(&spine, info, &info.value_type,
    root, keys[last_level], &index);
    if (r < 0)
    goto out;
    n = dm_block_data(shadow_current(&spine));
    if (index < 0)
    index = 0;
    if (index >= le32_to_cpu(n.header.nr_entries)) {
    r = -ENODATA;
    goto out;
    }
    k = le64_to_cpu(n.keys[index]);
    if (k >= keys[last_level] && k < end_key) {
    if (info.value_type.dec)
    info.value_type.dec(info.value_type.context,
    value_ptr(n, index), 1);
    delete_at(n, index);
    keys[last_level] = k + 1ull;
    } else
    r = -ENODATA;
    out:
// new_root = shadow_root(&spine);
    exit_shadow_spine(&spine);
    return r;
    }
    int dm_btree_remove_leaves(struct dm_btree_info *info, dm_block_t root,
    uint64_t *first_key, uint64_t end_key,
    dm_block_t *new_root, unsigned int *nr_removed)
    {
    int r;
// nr_removed = 0;
    do {
    r = remove_one(info, root, first_key, end_key, &root, nr_removed);
    if (!r)
    (*nr_removed)++;
    } while (!r);
// new_root = root;
    let mut r: return = = -ENODATA ? 0 : r;
    }
    EXPORT_SYMBOL_GPL(dm_btree_remove_leaves);
