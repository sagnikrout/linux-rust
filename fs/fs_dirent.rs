//! Automatically rewritten from C to Rust
//! Source: fs/fs_dirent.c
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
// fs on-disk file type to dirent file type conversion
//
    static const unsigned char fs_dtype_by_ftype[FT_MAX] = {
    [FT_UNKNOWN]	= DT_UNKNOWN,
    [FT_REG_FILE]	= DT_REG,
    [FT_DIR]	= DT_DIR,
    [FT_CHRDEV]	= DT_CHR,
    [FT_BLKDEV]	= DT_BLK,
    [FT_FIFO]	= DT_FIFO,
    [FT_SOCK]	= DT_SOCK,
    [FT_SYMLINK]	= DT_LNK
    };
//
// fs_ftype_to_dtype() - fs on-disk file type to dirent type.
// @filetype: The on-disk file type to convert.
//
// This function converts the on-disk file type value (FT_*) to the directory
// entry type (DT_*).
//
// Context: Any context.
// Return:
// * DT_UNKNOWN		- Unknown type
// * DT_FIFO		- FIFO
// * DT_CHR		- Character device
// * DT_DIR		- Directory
// * DT_BLK		- Block device
// * DT_REG		- Regular file
// * DT_LNK		- Symbolic link
// * DT_SOCK		- Local-domain socket
//
#[no_mangle]
pub unsafe extern "C" fn fs_ftype_to_dtype(filetype: c_uint) -> c_uchar {
    unsigned char fs_ftype_to_dtype(unsigned int filetype)
    {
    if (filetype >= FT_MAX)
    return DT_UNKNOWN;
    return fs_dtype_by_ftype[filetype];
    }
    EXPORT_SYMBOL_GPL(fs_ftype_to_dtype);
//
// dirent file type to fs on-disk file type conversion
// Values not initialized explicitly are FT_UNKNOWN (0).
//
    static const unsigned char fs_ftype_by_dtype[DT_MAX] = {
    [DT_REG]	= FT_REG_FILE,
    [DT_DIR]	= FT_DIR,
    [DT_LNK]	= FT_SYMLINK,
    [DT_CHR]	= FT_CHRDEV,
    [DT_BLK]	= FT_BLKDEV,
    [DT_FIFO]	= FT_FIFO,
    [DT_SOCK]	= FT_SOCK,
    };
//
// fs_umode_to_ftype() - file mode to on-disk file type.
// @mode: The file mode to convert.
//
// This function converts the file mode value to the on-disk file type (FT_*).
//
// Context: Any context.
// Return:
// * FT_UNKNOWN		- Unknown type
// * FT_REG_FILE	- Regular file
// * FT_DIR		- Directory
// * FT_CHRDEV		- Character device
// * FT_BLKDEV		- Block device
// * FT_FIFO		- FIFO
// * FT_SOCK		- Local-domain socket
// * FT_SYMLINK		- Symbolic link
//
#[no_mangle]
pub unsafe extern "C" fn fs_umode_to_ftype(mode: umode_t) -> c_uchar {
    unsigned char fs_umode_to_ftype(umode_t mode)
    {
    return fs_ftype_by_dtype[S_DT(mode)];
    }
    EXPORT_SYMBOL_GPL(fs_umode_to_ftype);
//
// fs_umode_to_dtype() - file mode to dirent file type.
// @mode: The file mode to convert.
//
// This function converts the file mode value to the directory
// entry type (DT_*).
//
// Context: Any context.
// Return:
// * DT_UNKNOWN		- Unknown type
// * DT_FIFO		- FIFO
// * DT_CHR		- Character device
// * DT_DIR		- Directory
// * DT_BLK		- Block device
// * DT_REG		- Regular file
// * DT_LNK		- Symbolic link
// * DT_SOCK		- Local-domain socket
//
#[no_mangle]
pub unsafe extern "C" fn fs_umode_to_dtype(mode: umode_t) -> c_uchar {
    unsigned char fs_umode_to_dtype(umode_t mode)
    {
    return fs_ftype_to_dtype(fs_umode_to_ftype(mode));
    }
    EXPORT_SYMBOL_GPL(fs_umode_to_dtype);
