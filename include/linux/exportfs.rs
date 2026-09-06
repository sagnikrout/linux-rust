//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/exportfs.h
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
pub const LINUX_EXPORTFS_H: c_int = 1;

// limit the handle size to NFSv4 handle size now
pub const MAX_HANDLE_SZ: c_int = 128;
//
// The fileid_type identifies how the file within the filesystem is encoded.
// In theory this is freely set and parsed by the filesystem, but we try to
// stick to conventions so we can share some generic code and don't confuse
// sniffers like ethereal/wireshark.
//
// The filesystem must not use the value '0' or '0xff'.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fid_type {
//
// The root, or export point, of the filesystem.
// (Never actually passed down to the filesystem.
//
    FILEID_ROOT = 0,

//
// 32bit inode number, 32 bit generation number.
//
    FILEID_INO32_GEN = 1,

//
// 32bit inode number, 32 bit generation number,
// 32 bit parent directory inode number.
//
    FILEID_INO32_GEN_PARENT = 2,

//
// 64 bit object ID, 64 bit root object ID,
// 32 bit generation number.
//
    FILEID_BTRFS_WITHOUT_PARENT = 0x4d,

//
// 64 bit object ID, 64 bit root object ID,
// 32 bit generation number,
// 64 bit parent object ID, 32 bit parent generation.
//
    FILEID_BTRFS_WITH_PARENT = 0x4e,

//
// 64 bit object ID, 64 bit root object ID,
// 32 bit generation number,
// 64 bit parent object ID, 32 bit parent generation,
// 64 bit parent root object ID.
//
    FILEID_BTRFS_WITH_PARENT_ROOT = 0x4f,

//
// 32 bit block number, 16 bit partition reference,
// 16 bit unused, 32 bit generation number.
//
    FILEID_UDF_WITHOUT_PARENT = 0x51,

//
// 32 bit block number, 16 bit partition reference,
// 16 bit unused, 32 bit generation number,
// 32 bit parent block number, 32 bit parent generation number
//
    FILEID_UDF_WITH_PARENT = 0x52,

//
// 64 bit checkpoint number, 64 bit inode number,
// 32 bit generation number.
//
    FILEID_NILFS_WITHOUT_PARENT = 0x61,

//
// 64 bit checkpoint number, 64 bit inode number,
// 32 bit generation number, 32 bit parent generation.
// 64 bit parent inode number.
//
    FILEID_NILFS_WITH_PARENT = 0x62,

//
// 32 bit generation number, 40 bit i_pos.
//
    FILEID_FAT_WITHOUT_PARENT = 0x71,

//
// 32 bit generation number, 40 bit i_pos,
// 32 bit parent generation number, 40 bit parent i_pos
//
    FILEID_FAT_WITH_PARENT = 0x72,

//
// 64 bit inode number, 32 bit generation number.
//
    FILEID_INO64_GEN = 0x81,

//
// 64 bit inode number, 32 bit generation number,
// 64 bit parent inode number, 32 bit parent generation.
//
    FILEID_INO64_GEN_PARENT = 0x82,

//
// 128 bit child FID (struct lu_fid)
// 128 bit parent FID (struct lu_fid)
//
    FILEID_LUSTRE = 0x97,

//
// 64 bit inode number, 32 bit subvolume, 32 bit generation number:
//
    FILEID_BCACHEFS_WITHOUT_PARENT = 0xb1,
    FILEID_BCACHEFS_WITH_PARENT = 0xb2,

//
// 64 bit namespace identifier, 32 bit namespace type, 32 bit inode number.
//
    FILEID_NSFS = 0xf1,

//
// 64 bit unique kernfs id
//
    FILEID_KERNFS = 0xfe,

//
// Filesystems must not use 0xff file ID.
//
    FILEID_INVALID = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fid {
    pub ino: u32,
    pub gen: u32,
    pub parent_ino: u32,
    pub parent_gen: u32,
    pub i32: },
    pub ino: u64,
    pub gen: u32,
    pub i64: } __packed,
    pub block: u32,
    pub partref: u16,
    pub parent_partref: u16,
    pub generation: u32,
    pub parent_block: u32,
    pub parent_generation: u32,
    pub udf: },
    pub raw): DECLARE_FLEX_ARRAY(__u32,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum handle_to_path_flags {
    HANDLE_CHECK_PERMS   = (1 << 0),
    HANDLE_CHECK_SUBTREE = (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct handle_to_path_ctx {
    pub root: path,
    pub flags: handle_to_path_flags,
    pub fh_flags: c_uint,
}

pub const EXPORT_FH_CONNECTABLE: c_uint = 0x1 /* Encode file handle with parent */;
pub const EXPORT_FH_FID: c_uint = 0x2 /* File handle may be non-decodeable */;
pub const EXPORT_FH_DIR_ONLY: c_uint = 0x4 /* Only decode file handle for a directory */;
//
// Filesystems use only lower 8 bits of file_handle type for fid_type.
// name_to_handle_at() uses upper 16 bits of type as user flags to be
// interpreted by open_by_handle_at().
//
pub const FILEID_USER_FLAGS_MASK: c_uint = 0xffff0000;

// Flags supported in encoded handle_type that is exported to user
pub const FILEID_IS_CONNECTABLE: c_uint = 0x10000;
pub const FILEID_IS_DIR: c_uint = 0x20000;

//
// struct export_operations - for nfsd to communicate with file systems
// @encode_fh:      encode a file handle fragment from a dentry
// @fh_to_dentry:   find the implied object and get a dentry for it
// @fh_to_parent:   find the implied object's parent and get a dentry for it
// @get_name:       find the name for a given inode in a given directory
// @get_parent:     find the parent of a given directory
// @commit_metadata: commit metadata changes to stable storage
//
// Methods for open_by_handle(2) syscall with special kernel file systems:
// @permission:     custom permission for opening a file by handle
// @open:           custom open routine for opening file by handle
//
// See Documentation/filesystems/nfs/exporting.rst for details on how to use
// this interface correctly and the definition of the flags.
//
// @encode_fh:
// @encode_fh should store in the file handle fragment @fh (using at most
// @max_len bytes) information that can be used by @decode_fh to recover the
// file referred to by the &struct dentry @de.  If @flag has CONNECTABLE bit
// set, the encode_fh() should store sufficient information so that a good
// attempt can be made to find not only the file but also it's place in the
// filesystem.   This typically means storing a reference to de->d_parent in
// the filehandle fragment.  encode_fh() should return the fileid_type on
// success and on error returns 255 (if the space needed to encode fh is
// greater than @max_len*4 bytes). On error @max_len contains the minimum
// size(in 4 byte unit) needed to encode the file handle.
//
// @fh_to_dentry:
// @fh_to_dentry is given a &struct super_block (@sb) and a file handle
// fragment (@fh, @fh_len). It should return a &struct dentry which refers
// to the same file that the file handle fragment refers to.  If it cannot,
// it should return a %NULL pointer if the file cannot be found, or an
// %ERR_PTR error code of %ENOMEM if a memory allocation failure occurred.
// Any other error code is treated like %NULL, and will cause an %ESTALE error
// for callers of exportfs_decode_fh().
// Any suitable dentry can be returned including, if necessary, a new dentry
// created with d_alloc_root.  The caller can then find any other extant
// dentries by following the d_alias links.
//
// @fh_to_parent:
// Same as @fh_to_dentry, except that it returns a pointer to the parent
// dentry if it was encoded into the filehandle fragment by @encode_fh.
//
// @get_name:
// @get_name should find a name for the given @child in the given @parent
// directory.  The name should be stored in the @name (with the
// understanding that it is already pointing to a %NAME_MAX + 1 sized
// buffer.   get_name() should return %0 on success, a negative error code
// or error.  @get_name will be called without @parent->i_rwsem held.
//
// @get_parent:
// @get_parent should find the parent directory for the given @child which
// is also a directory.  In the event that it cannot be found, or storage
// space cannot be allocated, a %ERR_PTR should be returned.
//
// @permission:
// Allow filesystems to specify a custom permission function for the
// open_by_handle_at(2) syscall instead of the default permission check.
// This custom permission function is not respected by nfsd.
//
// @open:
// Allow filesystems to specify a custom open function for the
// open_by_handle_at(2) syscall instead of the default file_open_root().
// This custom open function is not respected by nfsd.
//
// @commit_metadata:
// @commit_metadata should commit metadata changes to stable storage.
//
// @flags:
// Allows the filesystem to communicate to nfsd that it may want to do things
// differently when dealing with it.
//
// @block_ops:
// Operations for layout grants to block on the underlying device.
//
// Locking rules:
// get_parent is called with child->d_inode->i_rwsem down
// get_name is not (which is possibly inconsistent)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct export_operations {
    pub parent): *mut inode,
    pub fh_type): int fh_len, int,
    pub fh_type): int fh_len, int,
    pub child): *mut dentry,
    pub child): *mut *mut *mut dentry  (get_parent)(dentry,
    pub inode): *mut *mut int (commit_metadata)(struct inode,
    pub oflags): *mut *mut *mut int (permission)(struct handle_to_path_ctx ctx, unsigned int,
    pub oflags): *const *const *const *const file  (open)(path path, unsigned int,

//

    pub flags: c_ulong,

    pub block_ops: *const exportfs_block_ops,

}

//
// exportfs_cannot_lock() - check if export implements file locking
// @export_ops:	the nfs export operations to check
//
// Returns true if the export does not support file locking.
//
// Do not allow nfs export for filesystems with custom ->open() or
// ->permission() ops, which nfsd does not respect (e.g. pidfs, nsfs).
//
// If a non-decodeable file handle was requested, we only need to make
// sure that filesystem did not opt-out of encoding fid.
//
extern "C" {
    pub fn exportfs_can_encode_fid(_arg: nop) -> return;
}
// Normal file handles cannot be created without export ops
//
// If a connectable file handle was requested, we need to make sure that
// filesystem can also decode connected file handles.
//
// If a decodeable file handle was requested, we need to make sure that
// filesystem can also decode file handles.
//
extern "C" {
    pub fn exportfs_can_decode_fh(_arg: nop) -> return;
}
//
// Generic helpers for filesystems.
//
