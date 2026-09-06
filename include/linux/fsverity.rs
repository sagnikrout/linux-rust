//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsverity.h
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
// fs-verity: read-only file-based authenticity protection
//
// This header declares the interface between the fs/verity/ support layer and
// filesystems that support fs-verity.
//
// Copyright 2019 Google LLC
//

//
// Largest digest size among all hash algorithms supported by fs-verity.
// Currently assumed to be <= size of fsverity_descriptor::root_hash.
//

// Arbitrary limit to bound the kmalloc() size.  Can be changed.
pub const FS_VERITY_MAX_DESCRIPTOR_SIZE: c_int = 16384;
// Verity operations for filesystems
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_operations {
//
// Begin enabling verity on the given file.
//
// @filp: a readonly file descriptor for the file
//
// The filesystem must do any needed filesystem-specific preparations
// for enabling verity, e.g. evicting inline data.  It also must return
// -EBUSY if verity is already being enabled on the given file.
//
// i_rwsem is held for write.
//
// Return: 0 on success, -errno on failure
//
    pub filp): *mut *mut int (begin_enable_verity)(struct file,
//
// End enabling verity on the given file.
//
// @filp: a readonly file descriptor for the file
// @desc: the verity descriptor to write, or NULL on failure
// @desc_size: size of verity descriptor, or 0 on failure
// @merkle_tree_size: total bytes the Merkle tree took up
//
// If desc == NULL, then enabling verity failed and the filesystem only
// must do any necessary cleanups.  Else, it must also store the given
// verity descriptor to a fs-specific location associated with the inode
// and do any fs-specific actions needed to mark the inode as a verity
// inode, e.g. setting a bit in the on-disk inode.  The filesystem is
// also responsible for setting the S_VERITY flag in the VFS inode.
//
// i_rwsem is held for write, but it may have been dropped between
// ->begin_enable_verity() and ->end_enable_verity().
//
// Return: 0 on success, -errno on failure
//
    pub merkle_tree_size): size_t desc_size, u64,
//
// Get the verity descriptor of the given inode.
//
// @inode: an inode with the S_VERITY flag set
// @buf: buffer in which to place the verity descriptor
// @bufsize: size of @buf, or 0 to retrieve the size only
//
// If bufsize == 0, then the size of the verity descriptor is returned.
// Otherwise the verity descriptor is written to 'buf' and its actual
// size is returned; -ERANGE is returned if it's too large.  This may be
// called by multiple processes concurrently on the same inode.
//
// Return: the size on success, -errno on failure
//
    pub bufsize): usize,
//
// Read a Merkle tree page of the given inode.
//
// @inode: the inode
// @index: 0-based index of the page within the Merkle tree
//
// This can be called at any time on an open verity file.  It may be
// called by multiple processes concurrently, even with the same page.
//
// Note that this must retrieve a *page*, not necessarily a *block*.
//
// Return: the page on success, ERR_PTR() on failure
//
    pub index): pgoff_t,
//
// Perform readahead of a Merkle tree for the given inode.
//
// @inode: the inode
// @index: 0-based index of the first page within the Merkle tree
// @nr_pages: number of pages to be read ahead.
//
// This can be called at any time on an open verity file.  It may be
// called by multiple processes concurrently, even with the same range.
//
// Optional method so that ->read_merkle_tree_page preferably finds
// cached data instead of issuing dependent I/O.
//
    pub nr_pages): c_ulong,
//
// Write a Merkle tree block to the given file.
//
// @file: the file for which the Merkle tree is being built
// @buf: the Merkle tree block to write
// @pos: the position of the block in the Merkle tree (in bytes)
// @size: the Merkle tree block size (in bytes)
//
// This is only called between ->begin_enable_verity() and
// ->end_enable_verity().
//
// Return: 0 on success, -errno on failure
//
    pub size): u64 pos, unsigned int,
}

//
// fsverity_active() - do reads from the inode need to go through fs-verity?
// @inode: inode to check
//
// This checks whether the inode's verity info has been set, and reads need
// to verify the file data.
//
// Return: true if reads need to go through fs-verity, otherwise false
//
// This pairs with the try_cmpxchg in set_mask_bits()
// used to set the S_VERITY bit in i_flags.
//
// fsverity_get_info - get fsverity information for an inode
// @inode: inode to operate on.
//
// This gets the fsverity_info for @inode if it exists.  Safe to call without
// knowin that a fsverity_info exist for @inode, including on file systems that
// do not support fsverity.
//
extern "C" {
    pub fn __fsverity_get_info(_arg: inode) -> return;
}
// enable.c
extern "C" {
    pub fn fsverity_ioctl_enable(filp: *mut file, arg: *const void __user) -> c_int;
}
// measure.c
extern "C" {
    pub fn fsverity_ioctl_measure(filp: *mut file, arg: *mut void __user) -> c_int;
}
// open.c
extern "C" {
    pub fn __fsverity_file_open(inode: *mut inode, filp: *mut file) -> c_int;
}
// read_metadata.c
extern "C" {
    pub fn fsverity_ioctl_read_metadata(filp: *mut file, uarg: *const void __user) -> c_int;
}
// verify.c
extern "C" {
    pub fn fsverity_verify_bio(vi: *mut fsverity_info, bio: *mut bio);
}
extern "C" {
    pub fn fsverity_enqueue_verify_work(work: *mut work_struct);
}

// enable.c
// measure.c
//
// fsverity is not enabled in the kernel configuration, so always report
// that the file doesn't have fsverity enabled (digest size 0).
//
// open.c
// read_metadata.c
// verify.c

extern "C" {
    pub fn fsverity_verify_blocks(_arg: vi, _arg: folio, _arg: folio_size(folio), _arg: 0) -> return;
}
//
// fsverity_file_open() - prepare to open a verity file
// @inode: the inode being opened
// @filp: the struct file being set up
//
// When opening a verity file, deny the open if it is for writing.  Otherwise,
// set up the inode's verity info if not already done.
//
// When combined with fscrypt, this must be called after fscrypt_file_open().
// Otherwise, we won't have the key set up to decrypt the verity metadata.
//
// Return: 0 on success, -errno on failure
//
extern "C" {
    pub fn __fsverity_file_open(_arg: inode, _arg: filp) -> return;
}
extern "C" {
    pub fn fsverity_cleanup_inode(inode: *mut inode);
}
