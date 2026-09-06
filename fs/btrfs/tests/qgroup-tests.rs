//! Automatically rewritten from C to Rust
//! Source: fs/btrfs/tests/qgroup-tests.c
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
// Copyright (C) 2013 Facebook.  All rights reserved.
//

    static int insert_normal_tree_ref(struct btrfs_root *root, u64 bytenr,
    u64 num_bytes, u64 parent, u64 root_objectid)
    {
    struct btrfs_trans_handle trans;
    struct btrfs_extent_item *item;
    struct btrfs_extent_inline_ref *iref;
    struct btrfs_tree_block_info *block_info;
    BTRFS_PATH_AUTO_FREE(path);
    struct extent_buffer *leaf;
    struct btrfs_key ins;
    let mut size: u32 = sizeof(*item) + sizeof(*iref) + sizeof(*block_info);
    int ret;
    btrfs_init_dummy_trans(&trans, core::ptr::null_mut());
    ins.objectid = bytenr;
    ins.type = BTRFS_EXTENT_ITEM_KEY;
    ins.offset = num_bytes;
    path = btrfs_alloc_path();
    if (!path) {
    test_std_err(TEST_ALLOC_ROOT);
    return -ENOMEM;
    }
    ret = btrfs_insert_empty_item(&trans, root, path, &ins, size);
    if (ret) {
    test_err("couldn't insert ref %d", ret);
    return ret;
    }
    leaf = path.nodes[0];
    item = btrfs_item_ptr(leaf, path.slots[0], struct btrfs_extent_item);
    btrfs_set_extent_refs(leaf, item, 1);
    btrfs_set_extent_generation(leaf, item, 1);
    btrfs_set_extent_flags(leaf, item, BTRFS_EXTENT_FLAG_TREE_BLOCK);
    block_info = (struct btrfs_tree_block_info *)(item + 1);
    btrfs_set_tree_block_level(leaf, block_info, 0);
    iref = (struct btrfs_extent_inline_ref *)(block_info + 1);
    if (parent > 0) {
    btrfs_set_extent_inline_ref_type(leaf, iref,
    BTRFS_SHARED_BLOCK_REF_KEY);
    btrfs_set_extent_inline_ref_offset(leaf, iref, parent);
    } else {
    btrfs_set_extent_inline_ref_type(leaf, iref, BTRFS_TREE_BLOCK_REF_KEY);
    btrfs_set_extent_inline_ref_offset(leaf, iref, root_objectid);
    }
    return 0;
    }
    static int add_tree_ref(struct btrfs_root *root, u64 bytenr, u64 num_bytes,
    u64 parent, u64 root_objectid)
    {
    struct btrfs_trans_handle trans;
    struct btrfs_extent_item *item;
    BTRFS_PATH_AUTO_FREE(path);
    struct btrfs_key key;
    u64 refs;
    int ret;
    btrfs_init_dummy_trans(&trans, core::ptr::null_mut());
    key.objectid = bytenr;
    key.type = BTRFS_EXTENT_ITEM_KEY;
    key.offset = num_bytes;
    path = btrfs_alloc_path();
    if (!path) {
    test_std_err(TEST_ALLOC_ROOT);
    return -ENOMEM;
    }
    ret = btrfs_search_slot(&trans, root, &key, path, 0, 1);
    if (ret) {
    test_err("couldn't find extent ref");
    return ret;
    }
    item = btrfs_item_ptr(path.nodes[0], path.slots[0],
    struct btrfs_extent_item);
    refs = btrfs_extent_refs(path.nodes[0], item);
    btrfs_set_extent_refs(path.nodes[0], item, refs + 1);
    btrfs_release_path(path);
    key.objectid = bytenr;
    if (parent) {
    key.type = BTRFS_SHARED_BLOCK_REF_KEY;
    key.offset = parent;
    } else {
    key.type = BTRFS_TREE_BLOCK_REF_KEY;
    key.offset = root_objectid;
    }
    ret = btrfs_insert_empty_item(&trans, root, path, &key, 0);
    if (ret)
    test_err("failed to insert backref");
    return ret;
    }
    static int remove_extent_item(struct btrfs_root *root, u64 bytenr,
    u64 num_bytes)
    {
    struct btrfs_trans_handle trans;
    struct btrfs_key key;
    BTRFS_PATH_AUTO_FREE(path);
    int ret;
    btrfs_init_dummy_trans(&trans, core::ptr::null_mut());
    key.objectid = bytenr;
    key.type = BTRFS_EXTENT_ITEM_KEY;
    key.offset = num_bytes;
    path = btrfs_alloc_path();
    if (!path) {
    test_std_err(TEST_ALLOC_ROOT);
    return -ENOMEM;
    }
    ret = btrfs_search_slot(&trans, root, &key, path, -1, 1);
    if (ret) {
    test_err("didn't find our key %d", ret);
    return ret;
    }
    btrfs_del_item(&trans, root, path);
    return 0;
    }
    static int remove_extent_ref(struct btrfs_root *root, u64 bytenr,
    u64 num_bytes, u64 parent, u64 root_objectid)
    {
    struct btrfs_trans_handle trans;
    struct btrfs_extent_item *item;
    BTRFS_PATH_AUTO_FREE(path);
    struct btrfs_key key;
    u64 refs;
    int ret;
    btrfs_init_dummy_trans(&trans, core::ptr::null_mut());
    key.objectid = bytenr;
    key.type = BTRFS_EXTENT_ITEM_KEY;
    key.offset = num_bytes;
    path = btrfs_alloc_path();
    if (!path) {
    test_std_err(TEST_ALLOC_ROOT);
    return -ENOMEM;
    }
    ret = btrfs_search_slot(&trans, root, &key, path, 0, 1);
    if (ret) {
    test_err("couldn't find extent ref");
    return ret;
    }
    item = btrfs_item_ptr(path.nodes[0], path.slots[0],
    struct btrfs_extent_item);
    refs = btrfs_extent_refs(path.nodes[0], item);
    btrfs_set_extent_refs(path.nodes[0], item, refs - 1);
    btrfs_release_path(path);
    key.objectid = bytenr;
    if (parent) {
    key.type = BTRFS_SHARED_BLOCK_REF_KEY;
    key.offset = parent;
    } else {
    key.type = BTRFS_TREE_BLOCK_REF_KEY;
    key.offset = root_objectid;
    }
    ret = btrfs_search_slot(&trans, root, &key, path, -1, 1);
    if (ret) {
    test_err("couldn't find backref %d", ret);
    return ret;
    }
    btrfs_del_item(&trans, root, path);
    return ret;
    }
    static int test_no_shared_qgroup(struct btrfs_root *root,
    u32 sectorsize, u32 nodesize)
    {
    let mut ctx: btrfs_backref_walk_ctx = { 0 };
    struct btrfs_trans_handle trans;
    struct btrfs_fs_info *fs_info = root.fs_info;
    struct ulist *old_roots = core::ptr::null_mut();
    struct ulist *new_roots = core::ptr::null_mut();
    int ret;
    btrfs_init_dummy_trans(&trans, fs_info);
    test_msg("running qgroup add/remove tests");
    ret = btrfs_create_qgroup(&trans, BTRFS_FS_TREE_OBJECTID);
    if (ret) {
    test_err("couldn't create a qgroup %d", ret);
    return ret;
    }
    ctx.bytenr = nodesize;
    ctx.trans = &trans;
    ctx.fs_info = fs_info;
//
// Since the test trans doesn't have the complicated delayed refs,
// we can only call btrfs_qgroup_account_extent() directly to test
// quota.
//
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    old_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = insert_normal_tree_ref(root, nodesize, nodesize, 0,
    BTRFS_FS_TREE_OBJECTID);
    if (ret) {
    ulist_free(old_roots);
    return ret;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    ulist_free(old_roots);
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    new_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = btrfs_qgroup_account_extent(&trans, nodesize, nodesize, old_roots,
    new_roots);
    if (ret) {
    test_err("couldn't account space for a qgroup %d", ret);
    return ret;
    }
// btrfs_qgroup_account_extent() always frees the ulists passed to it.
    old_roots = core::ptr::null_mut();
    new_roots = core::ptr::null_mut();
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FS_TREE_OBJECTID,
    nodesize, nodesize)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    old_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = remove_extent_item(root, nodesize, nodesize);
    if (ret) {
    ulist_free(old_roots);
    return -EINVAL;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    ulist_free(old_roots);
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    new_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = btrfs_qgroup_account_extent(&trans, nodesize, nodesize, old_roots,
    new_roots);
    if (ret) {
    test_err("couldn't account space for a qgroup %d", ret);
    return -EINVAL;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FS_TREE_OBJECTID, 0, 0)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    return 0;
    }
//
// Add a ref for two different roots to make sure the shared value comes out
// right, also remove one of the roots and make sure the exclusive count is
// adjusted properly.
//
    static int test_multiple_refs(struct btrfs_root *root,
    u32 sectorsize, u32 nodesize)
    {
    let mut ctx: btrfs_backref_walk_ctx = { 0 };
    struct btrfs_trans_handle trans;
    struct btrfs_fs_info *fs_info = root.fs_info;
    struct ulist *old_roots = core::ptr::null_mut();
    struct ulist *new_roots = core::ptr::null_mut();
    int ret;
    btrfs_init_dummy_trans(&trans, fs_info);
    test_msg("running qgroup multiple refs test");
//
// We have BTRFS_FS_TREE_OBJECTID created already from the
// previous test.
//
    ret = btrfs_create_qgroup(&trans, BTRFS_FIRST_FREE_OBJECTID);
    if (ret) {
    test_err("couldn't create a qgroup %d", ret);
    return ret;
    }
    ctx.bytenr = nodesize;
    ctx.trans = &trans;
    ctx.fs_info = fs_info;
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    old_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = insert_normal_tree_ref(root, nodesize, nodesize, 0,
    BTRFS_FS_TREE_OBJECTID);
    if (ret) {
    ulist_free(old_roots);
    return ret;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    ulist_free(old_roots);
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    new_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = btrfs_qgroup_account_extent(&trans, nodesize, nodesize, old_roots,
    new_roots);
    if (ret) {
    test_err("couldn't account space for a qgroup %d", ret);
    return ret;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FS_TREE_OBJECTID,
    nodesize, nodesize)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    old_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = add_tree_ref(root, nodesize, nodesize, 0,
    BTRFS_FIRST_FREE_OBJECTID);
    if (ret) {
    ulist_free(old_roots);
    return ret;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    ulist_free(old_roots);
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    new_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = btrfs_qgroup_account_extent(&trans, nodesize, nodesize, old_roots,
    new_roots);
    if (ret) {
    test_err("couldn't account space for a qgroup %d", ret);
    return ret;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FS_TREE_OBJECTID,
    nodesize, 0)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FIRST_FREE_OBJECTID,
    nodesize, 0)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    old_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = remove_extent_ref(root, nodesize, nodesize, 0,
    BTRFS_FIRST_FREE_OBJECTID);
    if (ret) {
    ulist_free(old_roots);
    return ret;
    }
    ret = btrfs_find_all_roots(&ctx, false);
    if (ret) {
    ulist_free(old_roots);
    test_err("couldn't find old roots: %d", ret);
    return ret;
    }
    new_roots = ctx.roots;
    ctx.roots = core::ptr::null_mut();
    ret = btrfs_qgroup_account_extent(&trans, nodesize, nodesize, old_roots,
    new_roots);
    if (ret) {
    test_err("couldn't account space for a qgroup %d", ret);
    return ret;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FIRST_FREE_OBJECTID,
    0, 0)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    if (btrfs_verify_qgroup_counts(fs_info, BTRFS_FS_TREE_OBJECTID,
    nodesize, nodesize)) {
    test_err("qgroup counts didn't match expected values");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btrfs_test_qgroups(sectorsize: u32, nodesize: u32) -> c_int {
    int btrfs_test_qgroups(u32 sectorsize, u32 nodesize)
    {
    struct btrfs_fs_info *fs_info = core::ptr::null_mut();
    struct btrfs_root *root;
    struct btrfs_root *tmp_root;
    let mut ret: c_int = 0;
    fs_info = btrfs_alloc_dummy_fs_info(nodesize, sectorsize);
    if (!fs_info) {
    test_std_err(TEST_ALLOC_FS_INFO);
    return -ENOMEM;
    }
    root = btrfs_alloc_dummy_root(fs_info);
    if (IS_ERR(root)) {
    test_std_err(TEST_ALLOC_ROOT);
    ret = PTR_ERR(root);
    goto out;
    }
// We are using this root as our extent root
    root.root_key.objectid = BTRFS_EXTENT_TREE_OBJECTID;
    root.root_key.type = BTRFS_ROOT_ITEM_KEY;
    root.root_key.offset = 0;
    btrfs_global_root_insert(root);
//
// Some of the paths we test assume we have a filled out fs_info, so we
// just need to add the root in there so we don't panic.
//
    root.fs_info.tree_root = root;
    root.fs_info.quota_root = root;
    set_bit(BTRFS_FS_QUOTA_ENABLED, &fs_info.flags);
//
// Can't use bytenr 0, some things freak out
// *cough*backref walking code*cough
//
    root.node = alloc_test_extent_buffer(root.fs_info, nodesize);
    if (IS_ERR(root.node)) {
    test_err("couldn't allocate dummy buffer");
    ret = PTR_ERR(root.node);
    goto out;
    }
    btrfs_set_header_level(root.node, 0);
    btrfs_set_header_nritems(root.node, 0);
    root.alloc_bytenr += 2 * nodesize;
    tmp_root = btrfs_alloc_dummy_root(fs_info);
    if (IS_ERR(tmp_root)) {
    test_std_err(TEST_ALLOC_ROOT);
    ret = PTR_ERR(tmp_root);
    goto out;
    }
    tmp_root.root_key.objectid = BTRFS_FS_TREE_OBJECTID;
    root.fs_info.fs_root = tmp_root;
    ret = btrfs_insert_fs_root(root.fs_info, tmp_root);
    btrfs_put_root(tmp_root);
    if (ret) {
    test_err("couldn't insert fs root %d", ret);
    goto out;
    }
    tmp_root = btrfs_alloc_dummy_root(fs_info);
    if (IS_ERR(tmp_root)) {
    test_std_err(TEST_ALLOC_ROOT);
    ret = PTR_ERR(tmp_root);
    goto out;
    }
    tmp_root.root_key.objectid = BTRFS_FIRST_FREE_OBJECTID;
    ret = btrfs_insert_fs_root(root.fs_info, tmp_root);
    btrfs_put_root(tmp_root);
    if (ret) {
    test_err("couldn't insert subvolume root %d", ret);
    goto out;
    }
    test_msg("running qgroup tests");
    ret = test_no_shared_qgroup(root, sectorsize, nodesize);
    if (ret)
    goto out;
    ret = test_multiple_refs(root, sectorsize, nodesize);
    out:
    btrfs_free_dummy_root(root);
    btrfs_free_dummy_fs_info(fs_info);
    return ret;
    }
