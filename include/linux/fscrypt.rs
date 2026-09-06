//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fscrypt.h
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
// fscrypt.h: declarations for per-file encryption
//
// Filesystems that implement per-file encryption must include this header
// file.
//
// Copyright (C) 2015, Google, Inc.
//
// Written by Michael Halcrow, 2015.
// Modified by Jaegeuk Kim, 2015.
//

//
// The lengths of all file contents blocks must be divisible by this value.
// This is needed to ensure that all contents encryption modes will work, as
// some of the supported modes don't support arbitrarily byte-aligned messages.
//
// Since the needed alignment is 16 bytes, most filesystems will meet this
// requirement naturally, as typical block sizes are powers of 2.  However, if a
// filesystem can generate arbitrarily byte-aligned block lengths (e.g., via
// compression), then it will need to pad to this alignment before encryption.
//
pub const FSCRYPT_CONTENTS_ALIGNMENT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_str {
    pub name: *mut c_uchar,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_name {
    pub usr_fname: *const qstr,
    pub disk_name: fscrypt_str,
    pub hash: u32,
    pub minor_hash: u32,
    pub crypto_buf: fscrypt_str,
    pub is_nokey_name: bool,
}

// Maximum value for the third parameter of fscrypt_operations.set_context().
pub const FSCRYPT_SET_CONTEXT_MAX_SIZE: c_int = 40;
// Maximum supported number of block devices per filesystem
pub const FSCRYPT_MAX_DEVICES: c_int = 8;

// Crypto operations for filesystems
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_operations {
//
// The offset of the pointer to struct fscrypt_inode_info in the
// filesystem-specific part of the inode, relative to the beginning of
// the common part of the inode (the 'struct inode').
//
    pub inode_info_offs: ptrdiff_t,
//
// Set to 1 if the filesystem is block-based.  This causes fs/crypto/ to
// set up the key for regular files as a blk_crypto_key.  The filesystem
// then uses fscrypt_set_bio_crypt_ctx() and similar functions.
//
    pub 1: unsigned int is_block_based :,
//
// Set to 1 if the filesystem uses fscrypt_encrypt_pagecache_blocks().
// This enables the allocation of the bounce page pool it requires.
//
    pub 1: unsigned int needs_bounce_pages :,
//
// If set, then fs/crypto/ will allow the use of encryption settings
// that assume inode numbers fit in 32 bits (i.e.
// FSCRYPT_POLICY_FLAG_IV_INO_LBLK_{32,64}), provided that the other
// prerequisites for these settings are also met.  This is only useful
// if the filesystem wants to support inline encryption hardware that is
// limited to 32-bit or 64-bit data unit numbers and where programming
// keyslots is very slow.
//
    pub 1: unsigned int has_32bit_inodes :,
//
// If set, then fs/crypto/ will allow users to select a crypto data unit
// size that is less than the filesystem block size.  This is done via
// the log2_data_unit_size field of the fscrypt policy.  This flag is
// not compatible with filesystems that encrypt variable-length blocks
// (i.e. blocks that aren't all equal to filesystem's block size), for
// example as a result of compression.  It's also not compatible with
// the fscrypt_encrypt_block_inplace() and
// fscrypt_decrypt_block_inplace() functions.
//
    pub 1: unsigned int supports_subblock_data_units :,
//
// This field exists only for backwards compatibility reasons and should
// only be set by the filesystems that are setting it already.  It
// contains the filesystem-specific key description prefix that is
// accepted for "logon" keys for v1 fscrypt policies.  This
// functionality is deprecated in favor of the generic prefix
// "fscrypt:", which itself is deprecated in favor of the filesystem
// keyring ioctls such as FS_IOC_ADD_ENCRYPTION_KEY.  Filesystems that
// are newly adding fscrypt support should not set this field.
//
    pub legacy_key_prefix: *const c_char,
//
// Get the fscrypt context of the given inode.
//
// @inode: the inode whose context to get
// @ctx: the buffer into which to get the context
// @len: length of the @ctx buffer in bytes
//
// Return: On success, returns the length of the context in bytes; this
// may be less than @len.  On failure, returns -ENODATA if the
// inode doesn't have a context, -ERANGE if the context is
// longer than @len, or another -errno code.
//
    pub len): *mut *mut *mut *mut int (get_context)(struct inode inode, void ctx, size_t,
//
// Set an fscrypt context on the given inode.
//
// @inode: the inode whose context to set.  The inode won't already have
// an fscrypt context.
// @ctx: the context to set
// @len: length of @ctx in bytes (at most FSCRYPT_SET_CONTEXT_MAX_SIZE)
// @fs_data: If called from fscrypt_set_context(), this will be the
// value the filesystem passed to fscrypt_set_context().
// Otherwise (i.e. when called from
// FS_IOC_SET_ENCRYPTION_POLICY) this will be NULL.
//
// i_rwsem will be held for write.
//
// Return: 0 on success, -errno on failure.
//
    pub fs_data): *mut c_void,
//
// Get the dummy fscrypt policy in use on the filesystem (if any).
//
// Filesystems only need to implement this function if they support the
// test_dummy_encryption mount option.
//
// Return: A pointer to the dummy fscrypt policy, if the filesystem is
// mounted with test_dummy_encryption; otherwise NULL.
//
    pub sb): *const *const *const fscrypt_policy (get_dummy_policy)(struct super_block,
//
// Check whether a directory is empty.  i_rwsem will be held for write.
//
    pub inode): *mut *mut bool (empty_dir)(struct inode,
//
// Check whether the filesystem's inode numbers and UUID are stable,
// meaning that they will never be changed even by offline operations
// such as filesystem shrinking and therefore can be used in the
// encryption without the possibility of files becoming unreadable.
//
// Filesystems only need to implement this function if they want to
// support the FSCRYPT_POLICY_FLAG_IV_INO_LBLK_{32,64} flags.  These
// flags are designed to work around the limitations of UFS and eMMC
// inline crypto hardware, and they shouldn't be used in scenarios where
// such hardware isn't being used.
//
// Leaving this NULL is equivalent to always returning false.
//
    pub sb): *mut *mut bool (has_stable_inodes)(struct super_block,
//
// Retrieve the list of block devices to which the filesystem may write
// encrypted file contents.
//
// This writes the block_device pointers to @devs and returns the count
// (between 1 and FSCRYPT_MAX_DEVICES inclusively).
//
// If the filesystem can use multiple block devices (other than block
// devices that aren't used for encrypted file contents, such as
// external journal devices), and wants to support inline encryption,
// then it must implement this function.  Otherwise it's not needed.
//
    pub devs[FSCRYPT_MAX_DEVICES]): *mut block_device,
}

//
// Returns the address of the fscrypt info pointer within the
// filesystem-specific part of the inode.  (To save memory on filesystems that
// don't support fscrypt, a field in 'struct inode' itself is no longer used.)
//
// Load the inode's fscrypt info pointer, using a raw dereference.  Since this
// uses a raw dereference with no memory barrier, it is appropriate to use only
// when the caller knows the inode's key setup already happened, resulting in
// non-NULL fscrypt info.  E.g., the file contents en/decryption functions use
// this, since fscrypt_file_open() set up the key.
//
// Pairs with the cmpxchg_release() in fscrypt_setup_encryption_info().
// I.e., another task may publish the fscrypt info concurrently,
// executing a RELEASE barrier.  Use smp_load_acquire() here to safely
// ACQUIRE the memory the other task published.
//
extern "C" {
    pub fn smp_load_acquire(_arg: fscrypt_inode_info_addr(inode)) -> return;
}
//
// fscrypt_needs_contents_encryption() - check whether an inode needs
// contents encryption
// @inode: the inode to check
//
// Return: %true iff the inode is an encrypted regular file and the kernel was
// built with fscrypt support.
//
// If you need to know whether the encrypt bit is set even when the kernel was
// built without fscrypt support, you must use IS_ENCRYPTED() directly instead.
//
extern "C" {
    pub fn IS_ENCRYPTED(S_ISREG(inode->i_mode: inode) &&) -> return;
}
//
// When d_splice_alias() moves a directory's no-key alias to its
// plaintext alias as a result of the encryption key being added,
// DCACHE_NOKEY_NAME must be cleared and there might be an opportunity
// to disable d_revalidate.  Note that we don't have to support the
// inverse operation because fscrypt doesn't allow no-key names to be
// the source or target of a rename().
//
// VFS calls fscrypt_handle_d_move even for non-fscrypt
// filesystems.
//
// Other filesystem features might be handling dentry
// revalidation, in which case it cannot be disabled.
//
// fscrypt_is_nokey_name() - test whether a dentry is a no-key name
// @dentry: the dentry to check
//
// This returns true if the dentry is a no-key dentry.  A no-key dentry is a
// dentry that was created in an encrypted directory that hasn't had its
// encryption key added yet.  Such dentries may be either positive or negative.
//
// When a filesystem is asked to create a new filename in an encrypted directory
// and the new filename's dentry is a no-key dentry, it must fail the operation
// with ENOKEY.  This includes ->create(), ->mkdir(), ->mknod(), ->symlink(),
// ->rename(), and ->link().  (However, ->rename() and ->link() are already
// handled by fscrypt_prepare_rename() and fscrypt_prepare_link().)
//
// This is necessary because creating a filename requires the directory's
// encryption key, but just checking for the key on the directory inode during
// the final filesystem operation doesn't guarantee that the key was available
// during the preceding dentry lookup.  And the key must have already been
// available during the dentry lookup in order for it to have been checked
// whether the filename already exists in the directory and for the new file's
// dentry not to be invalidated due to it incorrectly having the no-key flag.
//
// Return: %true if the dentry is a no-key name
//
// This code tries to only take ->d_lock when necessary to write
// to ->d_flags.  We shouldn't be peeking on d_flags for
// DCACHE_OP_REVALIDATE unlocked, but in the unlikely case
// there is a race, the worst it can happen is that we fail to
// unset DCACHE_OP_REVALIDATE and pay the cost of an extra
// d_revalidate.
//
// Unencrypted dentries and encrypted dentries where the
// key is available are always valid from fscrypt
// perspective. Avoid the cost of calling
// fscrypt_d_revalidate unnecessarily.
//
// crypto.c
extern "C" {
    pub fn fscrypt_free_bounce_page(bounce_page: *mut page);
}
// policy.c
extern "C" {
    pub fn fscrypt_ioctl_set_policy(filp: *mut file, arg: *const void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_get_policy(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_get_policy_ex(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_get_nonce(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_has_permitted_context(parent: *mut inode, child: *mut inode) -> c_int;
}
extern "C" {
    pub fn fscrypt_context_for_new_inode(ctx: *mut c_void, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn fscrypt_set_context(inode: *mut inode, fs_data: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_dummy_policy {
    pub policy: *const fscrypt_policy,
}

// keyring.c
extern "C" {
    pub fn fscrypt_destroy_keyring(sb: *mut super_block);
}
extern "C" {
    pub fn fscrypt_ioctl_add_key(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_remove_key(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_remove_key_all_users(filp: *mut file, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fscrypt_ioctl_get_key_status(filp: *mut file, arg: *mut void __user) -> c_int;
}
// keysetup.c
extern "C" {
    pub fn fscrypt_put_encryption_info(inode: *mut inode);
}
extern "C" {
    pub fn fscrypt_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn fscrypt_drop_inode(inode: *mut inode) -> c_int;
}
// fname.c
extern "C" {
    pub fn fscrypt_fname_free_buffer(crypto_str: *mut fscrypt_str);
}
extern "C" {
    pub fn fscrypt_fname_siphash(dir: *const inode, name: *const qstr) -> u64;
}
// hooks.c
extern "C" {
    pub fn fscrypt_file_open(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn fscrypt_prepare_lookup_partial(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn __fscrypt_prepare_readdir(dir: *mut inode) -> c_int;
}
extern "C" {
    pub fn __fscrypt_prepare_setattr(dentry: *mut dentry, attr: *mut iattr) -> c_int;
}
extern "C" {
    pub fn fscrypt_symlink_getattr(path: *const path, stat: *mut kstat) -> c_int;
}

// crypto.c
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// policy.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_dummy_policy {
}

// keyring.c
// keysetup.c
// fname.c
// Encryption support disabled; use standard comparison
// hooks.c
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

// block.c

extern "C" {
    pub fn fscrypt_limit_io_blocks(inode: *const inode, lblk: u64, nr_blocks: u64) -> u64;
}

//
// fscrypt_has_encryption_key() - check whether an inode has had its key set up
// @inode: the inode to check
//
// Return: %true if the inode has had its encryption key set up, else %false.
//
// Usually this should be preceded by fscrypt_get_encryption_info() to try to
// set up the key first.
//
// fscrypt_prepare_link() - prepare to link an inode into a possibly-encrypted
// directory
// @old_dentry: an existing dentry for the inode being linked
// @dir: the target directory
// @dentry: negative dentry for the target filename
//
// A new link can only be added to an encrypted directory if the directory's
// encryption key is available --- since otherwise we'd have no way to encrypt
// the filename.
//
// We also verify that the link will not violate the constraint that all files
// in an encrypted directory tree use the same encryption policy.
//
// Return: 0 on success, -ENOKEY if the directory's encryption key is missing,
// -EXDEV if the link would result in an inconsistent encryption policy, or
// another -errno code.
//
extern "C" {
    pub fn __fscrypt_prepare_link(_arg: d_inode(old_dentry), _arg: dir, _arg: dentry) -> return;
}
//
// fscrypt_prepare_rename() - prepare for a rename between possibly-encrypted
// directories
// @old_dir: source directory
// @old_dentry: dentry for source file
// @new_dir: target directory
// @new_dentry: dentry for target location (may be negative unless exchanging)
// @flags: rename flags (we care at least about %RENAME_EXCHANGE)
//
// Prepare for ->rename() where the source and/or target directories may be
// encrypted.  A new link can only be added to an encrypted directory if the
// directory's encryption key is available --- since otherwise we'd have no way
// to encrypt the filename.  A rename to an existing name, on the other hand,
// *is* cryptographically possible without the key.  However, we take the more
// conservative approach and just forbid all no-key renames.
//
// We also verify that the rename will not violate the constraint that all files
// in an encrypted directory tree use the same encryption policy.
//
// Return: 0 on success, -ENOKEY if an encryption key is missing, -EXDEV if the
// rename would cause inconsistent encryption policies, or another -errno code.
//
// fscrypt_prepare_lookup() - prepare to lookup a name in a possibly-encrypted
// directory
// @dir: directory being searched
// @dentry: filename being looked up
// @fname: (output) the name to use to search the on-disk directory
//
// Prepare for ->lookup() in a directory which may be encrypted by determining
// the name that will actually be used to search the directory on-disk.  If the
// directory's encryption policy is supported by this kernel and its encryption
// key is available, then the lookup is assumed to be by plaintext name;
// otherwise, it is assumed to be by no-key name.
//
// This will set DCACHE_NOKEY_NAME on the dentry if the lookup is by no-key
// name.  In this case the filesystem must assign the dentry a dentry_operations
// which contains fscrypt_d_revalidate (or contains a d_revalidate method that
// calls fscrypt_d_revalidate), so that the dentry will be invalidated if the
// directory's encryption key is later added.
//
// Return: 0 on success; -ENOENT if the directory's key is unavailable but the
// filename isn't a valid no-key name, so a negative dentry should be created;
// or another -errno code.
//
extern "C" {
    pub fn __fscrypt_prepare_lookup(_arg: dir, _arg: dentry, _arg: fname) -> return;
}
//
// fscrypt_prepare_readdir() - prepare to read a possibly-encrypted directory
// @dir: the directory inode
//
// If the directory is encrypted and it doesn't already have its encryption key
// set up, try to set it up so that the filenames will be listed in plaintext
// form rather than in no-key form.
//
// Return: 0 on success; -errno on error.  Note that the encryption key being
// unavailable is not considered an error.  It is also not an error if
// the encryption policy is unsupported by this kernel; that is treated
// like the key being unavailable, so that files can still be deleted.
//
extern "C" {
    pub fn __fscrypt_prepare_readdir(_arg: dir) -> return;
}
//
// fscrypt_prepare_setattr() - prepare to change a possibly-encrypted inode's
// attributes
// @dentry: dentry through which the inode is being changed
// @attr: attributes to change
//
// Prepare for ->setattr() on a possibly-encrypted inode.  On an encrypted file,
// most attribute changes are allowed even without the encryption key.  However,
// without the encryption key we do have to forbid truncates.  This is needed
// because the size being truncated to may not be a multiple of the filesystem
// block size, and in that case we'd have to decrypt the final block, zero the
// portion past i_size, and re-encrypt it.  (We *could* allow truncating to a
// filesystem block boundary, but it's simpler to just forbid all truncates ---
// and we already forbid all other contents modifications without the key.)
//
// Return: 0 on success, -ENOKEY if the key is missing, or another -errno code
// if a problem occurred while setting up the encryption key.
//
extern "C" {
    pub fn __fscrypt_prepare_setattr(_arg: dentry, _arg: attr) -> return;
}
//
// fscrypt_encrypt_symlink() - encrypt the symlink target if needed
// @inode: symlink inode
// @target: plaintext symlink target
// @len: length of @target excluding null terminator
// @disk_link: (in/out) the on-disk symlink target being prepared
//
// If the symlink target needs to be encrypted, then this function encrypts it
// into @disk_link->name.  fscrypt_prepare_symlink() must have been called
// previously to compute @disk_link->len.  If the filesystem did not allocate a
// buffer for @disk_link->name after calling fscrypt_prepare_link(), then one
// will be kmalloc()'ed and the filesystem will be responsible for freeing it.
//
// Return: 0 on success, -errno on failure
//
extern "C" {
    pub fn __fscrypt_encrypt_symlink(_arg: inode, _arg: target, _arg: len, _arg: disk_link) -> return;
}
