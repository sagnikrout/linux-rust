//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/sigio.c
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
// Copyright (C) 2002 - 2008 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

//
// Protected by sigio_lock(), also used by sigio_cleanup, which is an
// exitcall.
//
    static struct os_helper_thread *write_sigio_td;
    let mut epollfd: static int = -1;
pub const MAX_EPOLL_EVENTS: c_int = 64;
    static struct epoll_event epoll_events[MAX_EPOLL_EVENTS];
    static void *write_sigio_thread(void *unused)
    {
    let mut pid: c_int = getpid();
    int r;
    os_fix_helper_thread_signals();
    while (1) {
    r = epoll_wait(epollfd, epoll_events, MAX_EPOLL_EVENTS, -1);
    if (r < 0) {
    if (errno == EINTR)
    continue;
    printk(UM_KERN_ERR "%s: epoll_wait failed, errno = %d\n",
    __func__, errno);
    }
    CATCH_EINTR(r = syscall(__NR_tgkill, pid, pid, SIGIO));
    if (r < 0)
    printk(UM_KERN_ERR "%s: tgkill failed, errno = %d\n",
    __func__, errno);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __add_sigio_fd(fd: c_int) -> c_int {
    int __add_sigio_fd(int fd)
    {
    struct epoll_event event = {
    .data.fd = fd,
    .events = EPOLLIN | EPOLLET,
    };
    int r;
    CATCH_EINTR(r = epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &event));
    return r < 0 ? -errno : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn add_sigio_fd(fd: c_int) -> c_int {
    int add_sigio_fd(int fd)
    {
    int err;
    sigio_lock();
    err = __add_sigio_fd(fd);
    sigio_unlock();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn __ignore_sigio_fd(fd: c_int) -> c_int {
    int __ignore_sigio_fd(int fd)
    {
    struct epoll_event event;
    int r;
    CATCH_EINTR(r = epoll_ctl(epollfd, EPOLL_CTL_DEL, fd, &event));
    return r < 0 ? -errno : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ignore_sigio_fd(fd: c_int) -> c_int {
    int ignore_sigio_fd(int fd)
    {
    int err;
    sigio_lock();
    err = __ignore_sigio_fd(fd);
    sigio_unlock();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn write_sigio_workaround() {
    static void write_sigio_workaround(void)
    {
    int err;
    sigio_lock();
    if (write_sigio_td)
    goto out;
    epollfd = epoll_create(MAX_EPOLL_EVENTS);
    if (epollfd < 0) {
    printk(UM_KERN_ERR "%s: epoll_create failed, errno = %d\n",
    __func__, errno);
    goto out;
    }
    err = os_run_helper_thread(&write_sigio_td, write_sigio_thread, core::ptr::null_mut());
    if (err < 0) {
    printk(UM_KERN_ERR "%s: os_run_helper_thread failed, errno = %d\n",
    __func__, -err);
    close(epollfd);
    epollfd = -1;
    goto out;
    }
    out:
    sigio_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn sigio_broken() {
    void sigio_broken(void)
    {
    write_sigio_workaround();
    }
// Changed during early boot
    static int pty_output_sigio;
#[no_mangle]
pub unsafe extern "C" fn maybe_sigio_broken(fd: c_int) {
    void maybe_sigio_broken(int fd)
    {
    if (!isatty(fd))
    return;
    if (pty_output_sigio)
    return;
    sigio_broken();
    }
#[no_mangle]
unsafe extern "C" fn sigio_cleanup() {
    static void sigio_cleanup(void)
    {
    if (!write_sigio_td)
    return;
    os_kill_helper_thread(write_sigio_td);
    write_sigio_td = core::ptr::null_mut();
    }
    __uml_exitcall(sigio_cleanup);
// Used as a flag during SIGIO testing early in boot
    static int got_sigio;
#[no_mangle]
unsafe extern "C" fn handler(sig: c_int) -> void __init {
    static void __init handler(int sig)
    {
    got_sigio = 1;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct openpty_arg {
    pub master: c_int,
    pub slave: c_int,
    pub err: c_int,
}

#[no_mangle]
unsafe extern "C" fn openpty_cb(arg: *mut c_void) {
    static void openpty_cb(void *arg)
    {
    struct openpty_arg *info = arg;
    info.err = 0;
    if (openpty(&info.master, &info.slave, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()))
    info.err = -errno;
    }
#[no_mangle]
unsafe extern "C" fn async_pty(master: c_int, slave: c_int) -> c_int {
    static int async_pty(int master, int slave)
    {
    int flags;
    flags = fcntl(master, F_GETFL);
    if (flags < 0)
    return -errno;
    if ((fcntl(master, F_SETFL, flags | O_NONBLOCK | O_ASYNC) < 0) ||
    (fcntl(master, F_SETOWN, os_getpid()) < 0))
    return -errno;
    if ((fcntl(slave, F_SETFL, flags | O_NONBLOCK) < 0))
    return -errno;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_one_sigio((*proc)(int: *mut c_void, _arg: int)) -> void __init {
    static void __init check_one_sigio(void (*proc)(int, int))
    {
    struct sigaction old, new;
    let mut pty: openpty_arg = { .master = -1, .slave = -1 };
    int master, slave, err;
    initial_thread_cb(openpty_cb, &pty);
    if (pty.err) {
    printk(UM_KERN_ERR "check_one_sigio failed, errno = %d\n",
    -pty.err);
    return;
    }
    master = pty.master;
    slave = pty.slave;
    if ((master == -1) || (slave == -1)) {
    printk(UM_KERN_ERR "check_one_sigio failed to allocate a "
    "pty\n");
    return;
    }
// Not now, but complain so we now where we failed.
    err = raw(master);
    if (err < 0) {
    printk(UM_KERN_ERR "check_one_sigio : raw failed, errno = %d\n",
    -err);
    return;
    }
    err = async_pty(master, slave);
    if (err < 0) {
    printk(UM_KERN_ERR "check_one_sigio : sigio_async failed, "
    "err = %d\n", -err);
    return;
    }
    if (sigaction(SIGIO, core::ptr::null_mut(), &old) < 0) {
    printk(UM_KERN_ERR "check_one_sigio : sigaction 1 failed, "
    "errno = %d\n", errno);
    return;
    }
    new = old;
    new.sa_handler = handler;
    if (sigaction(SIGIO, &new, core::ptr::null_mut()) < 0) {
    printk(UM_KERN_ERR "check_one_sigio : sigaction 2 failed, "
    "errno = %d\n", errno);
    return;
    }
    got_sigio = 0;
    (*proc)(master, slave);
    close(master);
    close(slave);
    if (sigaction(SIGIO, &old, core::ptr::null_mut()) < 0)
    printk(UM_KERN_ERR "check_one_sigio : sigaction 3 failed, "
    "errno = %d\n", errno);
    }
#[no_mangle]
unsafe extern "C" fn tty_output(master: c_int, slave: c_int) {
    static void tty_output(int master, int slave)
    {
    int n;
    char buf[512];
    printk(UM_KERN_INFO "Checking that host ptys support output SIGIO...");
    memset(buf, 0, sizeof(buf));
    while (write(master, buf, sizeof(buf)) > 0) ;
    if (errno != EAGAIN)
    printk(UM_KERN_ERR "tty_output : write failed, errno = %d\n",
    errno);
    while (((n = read(slave, buf, sizeof(buf))) > 0) &&
    !({ barrier(); got_sigio; }))
    ;
    if (got_sigio) {
    printk(UM_KERN_CONT "Yes\n");
    pty_output_sigio = 1;
    } else if (n == -EAGAIN)
    printk(UM_KERN_CONT "No, enabling workaround\n");
    else
    printk(UM_KERN_CONT "tty_output : read failed, err = %d\n", n);
    }
#[no_mangle]
unsafe extern "C" fn check_sigio() -> void __init {
    static void __init check_sigio(void)
    {
    if ((access("/dev/ptmx", R_OK) < 0) &&
    (access("/dev/ptyp0", R_OK) < 0)) {
    printk(UM_KERN_WARNING "No pseudo-terminals available - "
    "skipping pty SIGIO check\n");
    return;
    }
    check_one_sigio(tty_output);
    }
// Here because it only does the SIGIO testing for now
#[no_mangle]
pub unsafe extern "C" fn os_check_bugs() -> void __init {
    void __init os_check_bugs(void)
    {
    check_sigio();
    }
