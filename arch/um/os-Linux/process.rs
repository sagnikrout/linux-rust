//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/process.c
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
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
pub unsafe extern "C" fn os_alarm_process(pid: c_int) {
    void os_alarm_process(int pid)
    {
    if (pid <= 0)
    return;
    kill(pid, SIGALRM);
    }
#[no_mangle]
pub unsafe extern "C" fn os_kill_process(pid: c_int, reap_child: c_int) {
    void os_kill_process(int pid, int reap_child)
    {
    if (pid <= 0)
    return;
// Block signals until child is reaped
    block_signals();
    kill(pid, SIGKILL);
    if (reap_child)
    CATCH_EINTR(waitpid(pid, core::ptr::null_mut(), __WALL));
    unblock_signals();
    }
// Kill off a ptraced child by all means available.  kill it normally first,
// then PTRACE_KILL it, then PTRACE_CONT it in case it's in a run state from
// which it can't exit directly.
//
#[no_mangle]
pub unsafe extern "C" fn os_kill_ptraced_process(pid: c_int, reap_child: c_int) {
    void os_kill_ptraced_process(int pid, int reap_child)
    {
    if (pid <= 0)
    return;
// Block signals until child is reaped
    block_signals();
    kill(pid, SIGKILL);
    ptrace(PTRACE_KILL, pid);
    ptrace(PTRACE_CONT, pid);
    if (reap_child)
    CATCH_EINTR(waitpid(pid, core::ptr::null_mut(), __WALL));
    unblock_signals();
    }
#[no_mangle]
pub unsafe extern "C" fn os_reap_child() -> pid_t {
    pid_t os_reap_child(void)
    {
    int status;
// Try to reap a child
    return waitpid(-1, &status, WNOHANG);
    }
// Don't use the glibc version, which caches the result in TLS. It misses some
// syscalls, and also breaks with clone(), which does not unshare the TLS.
//
#[no_mangle]
pub unsafe extern "C" fn os_getpid() -> c_int {
    int os_getpid(void)
    {
    return syscall(__NR_getpid);
    }
    int os_map_memory(void *virt, int fd, unsigned long long off, unsigned long len,
    int r, int w, int x)
    {
    void *loc;
    int prot;
    prot = (r ? PROT_READ : 0) | (w ? PROT_WRITE : 0) |
    (x ? PROT_EXEC : 0);
    loc = mmap64((void *) virt, len, prot, MAP_SHARED | MAP_FIXED,
    fd, off);
    if (loc == MAP_FAILED)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_protect_memory(addr: *mut c_void, len: c_ulong, r: c_int, w: c_int, x: c_int) -> c_int {
    int os_protect_memory(void *addr, unsigned long len, int r, int w, int x)
    {
    int prot = ((r ? PROT_READ : 0) | (w ? PROT_WRITE : 0) |
    (x ? PROT_EXEC : 0));
    if (mprotect(addr, len, prot) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_unmap_memory(addr: *mut c_void, len: c_int) -> c_int {
    int os_unmap_memory(void *addr, int len)
    {
    int err;
    err = munmap(addr, len);
    if (err < 0)
    return -errno;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn os_drop_memory(addr: *mut c_void, length: c_int) -> c_int {
    int os_drop_memory(void *addr, int length)
    {
    int err;
    err = madvise(addr, length, MADV_REMOVE);
    if (err < 0)
    err = -errno;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn can_drop_memory() -> int __init {
    int __init can_drop_memory(void)
    {
    void *addr;
    int fd, ok = 0;
    printk(UM_KERN_INFO "Checking host MADV_REMOVE support...");
    fd = create_mem_file(UM_KERN_PAGE_SIZE);
    if (fd < 0) {
    printk(UM_KERN_ERR "Creating test memory file failed, "
    "err = %d\n", -fd);
    goto out;
    }
    addr = mmap64(core::ptr::null_mut(), UM_KERN_PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_SHARED, fd, 0);
    if (addr == MAP_FAILED) {
    printk(UM_KERN_ERR "Mapping test memory file failed, "
    "err = %d\n", -errno);
    goto out_close;
    }
    if (madvise(addr, UM_KERN_PAGE_SIZE, MADV_REMOVE) != 0) {
    printk(UM_KERN_ERR "MADV_REMOVE failed, err = %d\n", -errno);
    goto out_unmap;
    }
    printk(UM_KERN_CONT "OK\n");
    ok = 1;
    out_unmap:
    munmap(addr, UM_KERN_PAGE_SIZE);
    out_close:
    close(fd);
    out:
    return ok;
    }
#[no_mangle]
pub unsafe extern "C" fn init_new_thread_signals() {
    void init_new_thread_signals(void)
    {
    set_handler(SIGSEGV);
    set_handler(SIGTRAP);
    set_handler(SIGFPE);
    set_handler(SIGILL);
    set_handler(SIGBUS);
    signal(SIGHUP, SIG_IGN);
    set_handler(SIGIO);
// We (currently) only use the child reaper IRQ in seccomp mode
    if (using_seccomp)
    set_handler(SIGCHLD);
    signal(SIGWINCH, SIG_IGN);
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_pdeathsig() {
    void os_set_pdeathsig(void)
    {
    prctl(PR_SET_PDEATHSIG, SIGKILL);
    }
#[no_mangle]
pub unsafe extern "C" fn os_futex_wait(uaddr: *mut c_void, val: c_uint) -> c_int {
    int os_futex_wait(void *uaddr, unsigned int val)
    {
    int r;
    CATCH_EINTR(r = syscall(__NR_futex, uaddr, FUTEX_WAIT, val,
    core::ptr::null_mut(), core::ptr::null_mut(), 0));
    return r < 0 ? -errno : r;
    }
#[no_mangle]
pub unsafe extern "C" fn os_futex_wake(uaddr: *mut c_void) -> c_int {
    int os_futex_wake(void *uaddr)
    {
    int r;
    CATCH_EINTR(r = syscall(__NR_futex, uaddr, FUTEX_WAKE, INT_MAX,
    core::ptr::null_mut(), core::ptr::null_mut(), 0));
    return r < 0 ? -errno : r;
    }
