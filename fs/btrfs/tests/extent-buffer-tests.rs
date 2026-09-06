//! Automatically rewritten from C to Rust
//! Source: fs/btrfs/tests/extent-buffer-tests.c
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
// Copyright (C) 2013 Fusion IO.  All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn test_btrfs_split_item(sectorsize: u32, nodesize: u32) -> c_int {
    static int test_btrfs_split_item(u32 sectorsize, u32 nodesize)
    {
    struct btrfs_fs_info *fs_info;
    struct btrfs_path *path = core::ptr::null_mut();
    struct btrfs_root *root = core::ptr::null_mut();
    struct extent_buffer *eb;
    char *value = "mary had a little lamb";
    char *split1 = "mary had a little";
    char *split2 = " lamb";
    char *split3 = "mary";
    char *split4 = " had a little";
    char buf[32];
    struct btrfs_key key;
    let mut value_len: u32 = strlen(value);
    let mut ret: c_int = 0;
    test_msg("running btrfs_split_item tests");
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
    path = btrfs_alloc_path();
    if (!path) {
    test_std_err(TEST_ALLOC_PATH);
    ret = -ENOMEM;
    goto out;
    }
    eb = alloc_dummy_extent_buffer(fs_info, nodesize);
    path.nodes[0] = eb;
    if (!eb) {
    test_std_err(TEST_ALLOC_EXTENT_BUFFER);
    ret = -ENOMEM;
    goto out;
    }
    path.slots[0] = 0;
    key.objectid = 0;
    key.type = BTRFS_EXTENT_CSUM_KEY;
    key.offset = 0;
//
// Passing a NULL trans handle is fine here, we have a dummy root eb
// and the tree is a single node (level 0).
//
    btrfs_setup_item_for_insert(core::ptr::null_mut(), root, path, &key, value_len);
    write_extent_buffer(eb, value, btrfs_item_ptr_offset(eb, 0),
    value_len);
    key.offset = 3;
//
// Passing NULL trans here should be safe because we have plenty of
// space in this leaf to split the item without having to split the
// leaf.
//
    ret = btrfs_split_item(core::ptr::null_mut(), root, path, &key, 17);
    if (ret) {
    test_err("split item failed %d", ret);
    goto out;
    }
//
// Read the first slot, it should have the original key and contain only
// 'mary had a little'
//
    btrfs_item_key_to_cpu(eb, &key, 0);
    if (key.objectid != 0 || key.type != BTRFS_EXTENT_CSUM_KEY ||
    key.offset != 0) {
    test_err("invalid key at slot 0");
    ret = -EINVAL;
    goto out;
    }
    if (btrfs_item_size(eb, 0) != strlen(split1)) {
    test_err("invalid len in the first split");
    ret = -EINVAL;
    goto out;
    }
    read_extent_buffer(eb, buf, btrfs_item_ptr_offset(eb, 0),
    strlen(split1));
    if (memcmp(buf, split1, strlen(split1))) {
    test_err(
    "data in the buffer doesn't match what it should in the first split have='%.*s' want '%s'",
    (int)strlen(split1), buf, split1);
    ret = -EINVAL;
    goto out;
    }
    btrfs_item_key_to_cpu(eb, &key, 1);
    if (key.objectid != 0 || key.type != BTRFS_EXTENT_CSUM_KEY ||
    key.offset != 3) {
    test_err("invalid key at slot 1");
    ret = -EINVAL;
    goto out;
    }
    if (btrfs_item_size(eb, 1) != strlen(split2)) {
    test_err("invalid len in the second split");
    ret = -EINVAL;
    goto out;
    }
    read_extent_buffer(eb, buf, btrfs_item_ptr_offset(eb, 1),
    strlen(split2));
    if (memcmp(buf, split2, strlen(split2))) {
    test_err(
    "data in the buffer doesn't match what it should in the second split");
    ret = -EINVAL;
    goto out;
    }
    key.offset = 1;
// Do it again so we test memmoving the other items in the leaf
    ret = btrfs_split_item(core::ptr::null_mut(), root, path, &key, 4);
    if (ret) {
    test_err("second split item failed %d", ret);
    goto out;
    }
    btrfs_item_key_to_cpu(eb, &key, 0);
    if (key.objectid != 0 || key.type != BTRFS_EXTENT_CSUM_KEY ||
    key.offset != 0) {
    test_err("invalid key at slot 0");
    ret = -EINVAL;
    goto out;
    }
    if (btrfs_item_size(eb, 0) != strlen(split3)) {
    test_err("invalid len in the first split");
    ret = -EINVAL;
    goto out;
    }
    read_extent_buffer(eb, buf, btrfs_item_ptr_offset(eb, 0),
    strlen(split3));
    if (memcmp(buf, split3, strlen(split3))) {
    test_err(
    "data in the buffer doesn't match what it should in the third split");
    ret = -EINVAL;
    goto out;
    }
    btrfs_item_key_to_cpu(eb, &key, 1);
    if (key.objectid != 0 || key.type != BTRFS_EXTENT_CSUM_KEY ||
    key.offset != 1) {
    test_err("invalid key at slot 1");
    ret = -EINVAL;
    goto out;
    }
    if (btrfs_item_size(eb, 1) != strlen(split4)) {
    test_err("invalid len in the second split");
    ret = -EINVAL;
    goto out;
    }
    read_extent_buffer(eb, buf, btrfs_item_ptr_offset(eb, 1),
    strlen(split4));
    if (memcmp(buf, split4, strlen(split4))) {
    test_err(
    "data in the buffer doesn't match what it should in the fourth split");
    ret = -EINVAL;
    goto out;
    }
    btrfs_item_key_to_cpu(eb, &key, 2);
    if (key.objectid != 0 || key.type != BTRFS_EXTENT_CSUM_KEY ||
    key.offset != 3) {
    test_err("invalid key at slot 2");
    ret = -EINVAL;
    goto out;
    }
    if (btrfs_item_size(eb, 2) != strlen(split2)) {
    test_err("invalid len in the second split");
    ret = -EINVAL;
    goto out;
    }
    read_extent_buffer(eb, buf, btrfs_item_ptr_offset(eb, 2),
    strlen(split2));
    if (memcmp(buf, split2, strlen(split2))) {
    test_err(
    "data in the buffer doesn't match what it should in the last chunk");
    ret = -EINVAL;
    goto out;
    }
    out:
    btrfs_free_path(path);
    btrfs_free_dummy_root(root);
    btrfs_free_dummy_fs_info(fs_info);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btrfs_test_extent_buffer_operations(sectorsize: u32, nodesize: u32) -> c_int {
    int btrfs_test_extent_buffer_operations(u32 sectorsize, u32 nodesize)
    {
    test_msg("running extent buffer operation tests");
    return test_btrfs_split_item(sectorsize, nodesize);
    }
