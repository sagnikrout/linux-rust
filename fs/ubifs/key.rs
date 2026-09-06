//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ubifs/key.h
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
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation.
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Adrian Hunter
//
// This header contains various key-related definitions and helper function.
// UBIFS allows several key schemes, so we access key fields only via these
// helpers. At the moment only one key scheme is supported.
//
// Simple key scheme
// ~~~~~~~~~~~~~~~~~
//
// Keys are 64-bits long. First 32-bits are inode number (parent inode number
// in case of direntry key). Next 3 bits are node type. The last 29 bits are
// 4KiB offset in case of inode node, and direntry hash in case of a direntry
// node. We use "r5" hash borrowed from reiserfs.
//
// Lot's of the key helpers require a struct ubifs_info *c as the first parameter.
// But we are not using it at all currently. That's designed for future extensions of
// different c->key_format. But right now, there is only one key type, UBIFS_SIMPLE_KEY_FMT.
//
// key_mask_hash - mask a valid hash value.
// @hash: value to be masked
//
// We use hash values as offset in directories, so values %0 and %1 are
// reserved for "." and "..". %2 is reserved for "end of readdir" marker. This
// function makes sure the reserved values are not used.
//
// key_r5_hash - R5 hash function (borrowed from reiserfs).
// @s: direntry name
// @len: name length
//
extern "C" {
    pub fn key_mask_hash(_arg: a) -> return;
}
//
// key_test_hash - testing hash function.
// @str: direntry name
// @len: name length
//
extern "C" {
    pub fn key_mask_hash(_arg: a) -> return;
}
//
// ino_key_init - initialize inode key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
//
// ino_key_init_flash - initialize on-flash inode key.
// @c: UBIFS file-system description object
// @k: key to initialize
// @inum: inode number
//
// lowest_ino_key - get the lowest possible inode key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
//
// highest_ino_key - get the highest possible inode key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
//
// dent_key_init - initialize directory entry key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: parent inode number
// @nm: direntry name and length. Not a string when encrypted!
//
// dent_key_init_hash - initialize directory entry key without re-calculating
// hash function.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: parent inode number
// @hash: direntry name hash
//
// dent_key_init_flash - initialize on-flash directory entry key.
// @c: UBIFS file-system description object
// @k: key to initialize
// @inum: parent inode number
// @nm: direntry name and length
//
// lowest_dent_key - get the lowest possible directory entry key.
// @c: UBIFS file-system description object
// @key: where to store the lowest key
// @inum: parent inode number
//
// xent_key_init - initialize extended attribute entry key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: host inode number
// @nm: extended attribute entry name and length
//
// xent_key_init_flash - initialize on-flash extended attribute entry key.
// @c: UBIFS file-system description object
// @k: key to initialize
// @inum: host inode number
// @nm: extended attribute entry name and length
//
// lowest_xent_key - get the lowest possible extended attribute entry key.
// @c: UBIFS file-system description object
// @key: where to store the lowest key
// @inum: host inode number
//
// data_key_init - initialize data key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
// @block: block number
//
// highest_data_key - get the highest possible data key for an inode.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
//
// trun_key_init - initialize truncation node key.
// @c: UBIFS file-system description object
// @key: key to initialize
// @inum: inode number
//
// Note, UBIFS does not have truncation keys on the media and this function is
// only used for purposes of replay.
//
// invalid_key_init - initialize invalid node key.
// @c: UBIFS file-system description object
// @key: key to initialize
//
// This is a helper function which marks a @key object as invalid.
//
// key_type - get key type.
// @c: UBIFS file-system description object
// @key: key to get type of
//
// key_type_flash - get type of a on-flash formatted key.
// @c: UBIFS file-system description object
// @k: key to get type of
//
// key_inum - fetch inode number from key.
// @c: UBIFS file-system description object
// @k: key to fetch inode number from
//
// key_inum_flash - fetch inode number from an on-flash formatted key.
// @c: UBIFS file-system description object
// @k: key to fetch inode number from
//
extern "C" {
    pub fn le32_to_cpu(_arg: key->j32[0]) -> return;
}
//
// key_hash - get directory entry hash.
// @c: UBIFS file-system description object
// @key: the key to get hash from
//
// key_hash_flash - get directory entry hash from an on-flash formatted key.
// @c: UBIFS file-system description object
// @k: the key to get hash from
//
// key_block - get data block number.
// @c: UBIFS file-system description object
// @key: the key to get the block number from
//
// key_block_flash - get data block number from an on-flash formatted key.
// @c: UBIFS file-system description object
// @k: the key to get the block number from
//
// key_read - transform a key to in-memory format.
// @c: UBIFS file-system description object
// @from: the key to transform
// @to: the key to store the result
//
// key_write - transform a key from in-memory format.
// @c: UBIFS file-system description object
// @from: the key to transform
// @to: the key to store the result
//
// key_write_idx - transform a key from in-memory format for the index.
// @c: UBIFS file-system description object
// @from: the key to transform
// @to: the key to store the result
//
// key_copy - copy a key.
// @c: UBIFS file-system description object
// @from: the key to copy from
// @to: the key to copy to
//
// keys_cmp - compare keys.
// @c: UBIFS file-system description object
// @key1: the first key to compare
// @key2: the second key to compare
//
// This function compares 2 keys and returns %-1 if @key1 is less than
// @key2, %0 if the keys are equivalent and %1 if @key1 is greater than @key2.
//
// keys_eq - determine if keys are equivalent.
// @c: UBIFS file-system description object
// @key1: the first key to compare
// @key2: the second key to compare
//
// This function compares 2 keys and returns %1 if @key1 is equal to @key2 and
// %0 if not.
//
// is_hash_key - is a key vulnerable to hash collisions.
// @c: UBIFS file-system description object
// @key: key
//
// This function returns %1 if @key is a hashed key or %0 otherwise.
//
// key_max_inode_size - get maximum file size allowed by current key format.
// @c: UBIFS file-system description object
//
