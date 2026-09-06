//! Automatically rewritten from C to Rust
//! Source: fs/init.c
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
// Routines that mimic syscalls, but don't use the user address space or file
// descriptors.  Only for init/ and related early init code.
//

#[no_mangle]
pub unsafe extern "C" fn init_pivot_root(new_root: *const c_char, put_old: *const c_char) -> int __init {
    int __init init_pivot_root(const char *new_root, const char *put_old)
    {
    struct path new_path __free(path_put) = {};
    struct path old_path __free(path_put) = {};
    int ret;
    ret = kern_path(new_root, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &new_path);
    if (ret)
    return ret;
    ret = kern_path(put_old, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &old_path);
    if (ret)
    return ret;
    return path_pivot_root(&new_path, &old_path);
    }
    int __init init_mount(const char *dev_name, const char *dir_name,
    const char *type_page, unsigned long flags, void *data_page)
    {
    struct path path;
    int ret;
    ret = kern_path(dir_name, LOOKUP_FOLLOW, &path);
    if (ret)
    return ret;
    ret = path_mount(dev_name, &path, type_page, flags, data_page);
    path_put(&path);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn init_umount(name: *const c_char, flags: c_int) -> int __init {
    int __init init_umount(const char *name, int flags)
    {
    let mut lookup_flags: c_int = LOOKUP_MOUNTPOINT;
    struct path path;
    int ret;
    if (!(flags & UMOUNT_NOFOLLOW))
    lookup_flags |= LOOKUP_FOLLOW;
    ret = kern_path(name, lookup_flags, &path);
    if (ret)
    return ret;
    return path_umount(&path, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn init_chdir(filename: *const c_char) -> int __init {
    int __init init_chdir(const char *filename)
    {
    struct path path;
    int error;
    error = kern_path(filename, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &path);
    if (error)
    return error;
    error = path_permission(&path, MAY_EXEC | MAY_CHDIR);
    if (!error)
    set_fs_pwd(current.fs, &path);
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_chroot(filename: *const c_char) -> int __init {
    int __init init_chroot(const char *filename)
    {
    struct path path;
    int error;
    error = kern_path(filename, LOOKUP_FOLLOW | LOOKUP_DIRECTORY, &path);
    if (error)
    return error;
    error = path_permission(&path, MAY_EXEC | MAY_CHDIR);
    if (error)
    goto dput_and_out;
    error = -EPERM;
    if (!ns_capable(current_user_ns(), CAP_SYS_CHROOT))
    goto dput_and_out;
    error = security_path_chroot(&path);
    if (error)
    goto dput_and_out;
    set_fs_root(current.fs, &path);
    dput_and_out:
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_chown(filename: *const c_char, user: uid_t, group: gid_t, flags: c_int) -> int __init {
    int __init init_chown(const char *filename, uid_t user, gid_t group, int flags)
    {
    let mut lookup_flags: c_int = (flags & AT_SYMLINK_NOFOLLOW) ? 0 : LOOKUP_FOLLOW;
    struct path path;
    int error;
    error = kern_path(filename, lookup_flags, &path);
    if (error)
    return error;
    error = mnt_want_write(path.mnt);
    if (!error) {
    error = chown_common(&path, user, group);
    mnt_drop_write(path.mnt);
    }
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_chmod(filename: *const c_char, mode: umode_t) -> int __init {
    int __init init_chmod(const char *filename, umode_t mode)
    {
    struct path path;
    int error;
    error = kern_path(filename, LOOKUP_FOLLOW, &path);
    if (error)
    return error;
    error = chmod_common(&path, mode);
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_eaccess(filename: *const c_char) -> int __init {
    int __init init_eaccess(const char *filename)
    {
    struct path path;
    int error;
    error = kern_path(filename, LOOKUP_FOLLOW, &path);
    if (error)
    return error;
    error = path_permission(&path, MAY_ACCESS);
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_stat(filename: *const c_char, stat: *mut kstat, flags: c_int) -> int __init {
    int __init init_stat(const char *filename, struct kstat *stat, int flags)
    {
    let mut lookup_flags: c_int = (flags & AT_SYMLINK_NOFOLLOW) ? 0 : LOOKUP_FOLLOW;
    struct path path;
    int error;
    error = kern_path(filename, lookup_flags, &path);
    if (error)
    return error;
    error = vfs_getattr(&path, stat, STATX_BASIC_STATS,
    flags | AT_NO_AUTOMOUNT);
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_mknod(filename: *const c_char, mode: umode_t, dev: c_uint) -> int __init {
    int __init init_mknod(const char *filename, umode_t mode, unsigned int dev)
    {
    CLASS(filename_kernel, name)(filename);
    return filename_mknodat(AT_FDCWD, name, mode, dev);
    }
#[no_mangle]
pub unsafe extern "C" fn init_link(oldname: *const c_char, newname: *const c_char) -> int __init {
    int __init init_link(const char *oldname, const char *newname)
    {
    CLASS(filename_kernel, old)(oldname);
    CLASS(filename_kernel, new)(newname);
    return filename_linkat(AT_FDCWD, old, AT_FDCWD, new, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn init_symlink(oldname: *const c_char, newname: *const c_char) -> int __init {
    int __init init_symlink(const char *oldname, const char *newname)
    {
    CLASS(filename_kernel, old)(oldname);
    CLASS(filename_kernel, new)(newname);
    return filename_symlinkat(old, AT_FDCWD, new);
    }
#[no_mangle]
pub unsafe extern "C" fn init_unlink(pathname: *const c_char) -> int __init {
    int __init init_unlink(const char *pathname)
    {
    CLASS(filename_kernel, name)(pathname);
    return filename_unlinkat(AT_FDCWD, name);
    }
#[no_mangle]
pub unsafe extern "C" fn init_mkdir(pathname: *const c_char, mode: umode_t) -> int __init {
    int __init init_mkdir(const char *pathname, umode_t mode)
    {
    CLASS(filename_kernel, name)(pathname);
    return filename_mkdirat(AT_FDCWD, name, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn init_rmdir(pathname: *const c_char) -> int __init {
    int __init init_rmdir(const char *pathname)
    {
    CLASS(filename_kernel, name)(pathname);
    return filename_rmdir(AT_FDCWD, name);
    }
#[no_mangle]
pub unsafe extern "C" fn init_utimes(filename: *mut c_char, ts: *mut timespec64) -> int __init {
    int __init init_utimes(char *filename, struct timespec64 *ts)
    {
    struct path path;
    int error;
    error = kern_path(filename, 0, &path);
    if (error)
    return error;
    error = vfs_utimes(&path, ts);
    path_put(&path);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn init_dup(file: *mut file) -> int __init {
    int __init init_dup(struct file *file)
    {
    int fd;
    fd = get_unused_fd_flags(0);
    if (fd < 0)
    return fd;
    fd_install(fd, get_file(file));
    return 0;
    }
