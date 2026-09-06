//! Automatically rewritten from C to Rust
//! Source: fs/hostfs/hostfs_user_exp.c
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


    EXPORT_SYMBOL_GPL(stat_file);
    EXPORT_SYMBOL_GPL(access_file);
    EXPORT_SYMBOL_GPL(open_file);
    EXPORT_SYMBOL_GPL(open_dir);
    EXPORT_SYMBOL_GPL(seek_dir);
    EXPORT_SYMBOL_GPL(read_dir);
    EXPORT_SYMBOL_GPL(read_file);
    EXPORT_SYMBOL_GPL(write_file);
    EXPORT_SYMBOL_GPL(lseek_file);
    EXPORT_SYMBOL_GPL(fsync_file);
    EXPORT_SYMBOL_GPL(replace_file);
    EXPORT_SYMBOL_GPL(close_file);
    EXPORT_SYMBOL_GPL(close_dir);
    EXPORT_SYMBOL_GPL(file_create);
    EXPORT_SYMBOL_GPL(set_attr);
    EXPORT_SYMBOL_GPL(make_symlink);
    EXPORT_SYMBOL_GPL(unlink_file);
    EXPORT_SYMBOL_GPL(do_mkdir);
    EXPORT_SYMBOL_GPL(hostfs_do_rmdir);
    EXPORT_SYMBOL_GPL(do_mknod);
    EXPORT_SYMBOL_GPL(link_file);
    EXPORT_SYMBOL_GPL(hostfs_do_readlink);
    EXPORT_SYMBOL_GPL(rename_file);
    EXPORT_SYMBOL_GPL(rename2_file);
    EXPORT_SYMBOL_GPL(do_statfs);
