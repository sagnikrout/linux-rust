//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/sys.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// Syscall definitions for NOLIBC (those in man(2))
// Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
//
// make sure to include all global symbols

// system includes

// Syscall return helper: takes the syscall value in argument and checks for an
// error in it. This may only be used with signed returns (int or long), but
// not with pointers. An error is any value < 0. When an error is encountered,
// -ret is set into errno and -1 is returned. Otherwise the returned value is
// passed as-is with its type preserved.
//

// Syscall ENOSYS helper: Avoids unused-parameter warnings, provides compile
// time validation and a debugging hook.
//

extern "C" {
    pub fn __nolibc_enosys(syscall: *const c_char, ...) -> c_int;
}

//
// Helper for 32-bit machines where a 64-bit syscall arg needs to be split into
// two 32-bit parts while making sure the order of the low/high parts are correct
// for the endianness:
// __NOLIBC_LLARGPART(x, 0), __NOLIBC_LLARGPART(x, 1)
//

// Functions in this file only describe syscalls. They're declared static so
// that the compiler usually decides to inline them while still being allowed
// to pass a pointer to one of their instances. Each syscall exists in two
// versions:
// - the "internal" ones, which matches the raw syscall interface at the
// kernel level, which may sometimes slightly differ from the documented
// libc-level ones. For example most of them return either a valid value
// or -errno. All of these are prefixed with "_sys_". They may be called
// by non-portable applications if desired.
//
// - the "exported" ones, whose interface must closely match the one
// documented in man(2), that applications are supposed to expect. These
// ones rely on the internal ones, and set errno.
//
// Each syscall will be defined with the two functions, sorted in alphabetical
// order applied to the exported names.
//
// In case of doubt about the relevance of a function here, only those which
// set errno should be defined here. Wrappers like those appearing in man(3)
// should not be placed here.
//
// int brk(void *addr);
// void *sbrk(intptr_t inc)
//
// first call to find current end
//
// int chdir(const char *path);
// int fchdir(int fildes);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_chdir, _arg: path) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_chdir(path)) -> return;
}
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_fchdir, _arg: fildes) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_fchdir(fildes)) -> return;
}
//
// int chmod(const char *path, mode_t mode);
//

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_fchmodat, _arg: AT_FDCWD, _arg: path, _arg: mode, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_chmod, _arg: path, _arg: mode) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_chmod(path, _arg: mode)) -> return;
}
//
// int chown(const char *path, uid_t owner, gid_t group);
//

extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_fchownat, _arg: AT_FDCWD, _arg: path, _arg: owner, _arg: group, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_chown, _arg: path, _arg: owner, _arg: group) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_chown(path, _arg: owner, _arg: group)) -> return;
}
//
// int chroot(const char *path);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_chroot, _arg: path) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_chroot(path)) -> return;
}
//
// int close(int fd);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_close, _arg: fd) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_close(fd)) -> return;
}
//
// int dup(int fd);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_dup, _arg: fd) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_dup(fd)) -> return;
}
//
// int dup2(int old, int new);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_dup3, _arg: old, _arg: new, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_dup2, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_dup2(old, _arg: new)) -> return;
}
//
// int dup3(int old, int new, int flags);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_dup3, _arg: old, _arg: new, _arg: flags) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_dup3(old, _arg: new, _arg: flags)) -> return;
}

//
// int execve(const char *filename, char *const argv[], char *const envp[]);
//
extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_execve, _arg: filename, _arg: argv, _arg: envp) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_execve(filename, _arg: argv, _arg: envp)) -> return;
}
//
// void exit(int status);
//
// pid_t fork(void);
//

// note: some archs only have clone() and not fork(). Different archs
// have a different API, but most archs have the flags on first arg and
// will not use the rest with no other flag.
//
extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_clone, _arg: SIGCHLD, _arg: 0, _arg: 0, _arg: 0, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_fork) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_fork()) -> return;
}

// See the note in _sys_fork().
extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_clone, SIGCHLD: CLONE_VM | CLONE_VFORK |, _arg: 0, _arg: 0, _arg: 0, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_vfork) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_vfork()) -> return;
}
//
// int fsync(int fd);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_fsync, _arg: fd) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_fsync(fd)) -> return;
}
//
// int getdents64(int fd, struct linux_dirent64 *dirp, int count);
//
extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_getdents64, _arg: fd, _arg: dirp, _arg: count) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_getdents64(fd, _arg: dirp, _arg: count)) -> return;
}
//
// uid_t geteuid(void);
//

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_geteuid32) -> return;
}

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_geteuid) -> return;
}

extern "C" {
    pub fn _sys_geteuid() -> return;
}
//
// pid_t getpgid(pid_t pid);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_getpgid, _arg: pid) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_getpgid(pid)) -> return;
}
//
// pid_t getpgrp(void);
//
extern "C" {
    pub fn _sys_getpgid(_arg: 0) -> return;
}
extern "C" {
    pub fn _sys_getpgrp() -> return;
}
//
// pid_t getpid(void);
//
extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_getpid) -> return;
}
extern "C" {
    pub fn _sys_getpid() -> return;
}
//
// pid_t getppid(void);
//
extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_getppid) -> return;
}
extern "C" {
    pub fn _sys_getppid() -> return;
}
//
// pid_t gettid(void);
//
extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_gettid) -> return;
}
extern "C" {
    pub fn _sys_gettid() -> return;
}

extern "C" {
    pub fn getauxval(key: c_ulong) -> static unsigned long;
}
//
// int getpagesize(void);
//
extern "C" {
    pub fn __sysret(-ENOENT: (int)getauxval(AT_PAGESZ) ?:) -> return;
}

//
// uid_t getuid(void);
//

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_getuid32) -> return;
}

extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_getuid) -> return;
}

extern "C" {
    pub fn _sys_getuid() -> return;
}
//
// int kill(pid_t pid, int signal);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_kill, _arg: pid, _arg: signal) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_kill(pid, _arg: signal)) -> return;
}
//
// int link(const char *old, const char *new);
//

extern "C" {
    pub fn __nolibc_syscall5(_arg: __NR_linkat, _arg: AT_FDCWD, _arg: old, _arg: AT_FDCWD, _arg: new, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_link, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_link(old, _arg: new)) -> return;
}
//
// off_t lseek(int fd, off_t offset, int whence);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_lseek, _arg: fd, _arg: offset, _arg: whence) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_lseek(fd, _arg: offset, _arg: whence)) -> return;
}
//
// int mkdir(const char *path, mode_t mode);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_mkdirat, _arg: AT_FDCWD, _arg: path, _arg: mode) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_mkdir, _arg: path, _arg: mode) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_mkdir(path, _arg: mode)) -> return;
}
//
// int rmdir(const char *path);
//

extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_rmdir, _arg: path) -> return;
}

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_unlinkat, _arg: AT_FDCWD, _arg: path, _arg: AT_REMOVEDIR) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_rmdir(path)) -> return;
}
//
// int mknod(const char *path, mode_t mode, dev_t dev);
//

extern "C" {
    pub fn __nolibc_syscall4(_arg: __NR_mknodat, _arg: AT_FDCWD, _arg: path, _arg: mode, _arg: dev) -> return;
}

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_mknod, _arg: path, _arg: mode, _arg: dev) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_mknod(path, _arg: mode, _arg: dev)) -> return;
}
//
// int pipe2(int pipefd[2], int flags);
// int pipe(int pipefd[2]);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_pipe2, _arg: pipefd, _arg: flags) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_pipe2(pipefd, _arg: flags)) -> return;
}
extern "C" {
    pub fn pipe2(_arg: pipefd, _arg: 0) -> return;
}
//
// int pivot_root(const char *new, const char *old);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_pivot_root, _arg: new, _arg: old) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_pivot_root(new, _arg: old)) -> return;
}
//
// ssize_t read(int fd, void *buf, size_t count);
//
extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_read, _arg: fd, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_read(fd, _arg: buf, _arg: count)) -> return;
}
//
// int sched_yield(void);
//
extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_sched_yield) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_sched_yield()) -> return;
}
//
// int setpgid(pid_t pid, pid_t pgid);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_setpgid, _arg: pid, _arg: pgid) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_setpgid(pid, _arg: pgid)) -> return;
}
//
// pid_t setpgrp(void)
//
extern "C" {
    pub fn setpgid(_arg: 0, _arg: 0) -> return;
}
//
// pid_t setsid(void);
//
extern "C" {
    pub fn __nolibc_syscall0(_arg: __NR_setsid) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_setsid()) -> return;
}
//
// int symlink(const char *old, const char *new);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_symlinkat, _arg: old, _arg: AT_FDCWD, _arg: new) -> return;
}

extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_symlink, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_symlink(old, _arg: new)) -> return;
}
//
// mode_t umask(mode_t mode);
//
extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_umask, _arg: mode) -> return;
}
extern "C" {
    pub fn _sys_umask(_arg: mode) -> return;
}
//
// int umount2(const char *path, int flags);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_umount2, _arg: path, _arg: flags) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_umount2(path, _arg: flags)) -> return;
}
//
// int unlink(const char *path);
//

extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_unlinkat, _arg: AT_FDCWD, _arg: path, _arg: 0) -> return;
}

extern "C" {
    pub fn __nolibc_syscall1(_arg: __NR_unlink, _arg: path) -> return;
}

extern "C" {
    pub fn __sysret(_arg: _sys_unlink(path)) -> return;
}
//
// ssize_t write(int fd, const void *buf, size_t count);
//
extern "C" {
    pub fn __nolibc_syscall3(_arg: __NR_write, _arg: fd, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_write(fd, _arg: buf, _arg: count)) -> return;
}
//
// int memfd_create(const char *name, unsigned int flags);
//
extern "C" {
    pub fn __nolibc_syscall2(_arg: __NR_memfd_create, _arg: name, _arg: flags) -> return;
}
extern "C" {
    pub fn __sysret(_arg: _sys_memfd_create(name, _arg: flags)) -> return;
}
