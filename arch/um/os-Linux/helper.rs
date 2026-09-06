//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/helper.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct helper_data {
    pub (*pre_exec)(void*): *mut c_void,
    pub pre_data: *mut c_void,
    pub argv: *mut c_char,
    pub fd: c_int,
    pub buf: *mut c_char,
}

#[no_mangle]
unsafe extern "C" fn helper_child(arg: *mut c_void) -> c_int {
    static int helper_child(void *arg)
    {
    struct helper_data *data = arg;
    char **argv = data.argv;
    int err, ret;
    if (data.pre_exec != core::ptr::null_mut())
    (*data.pre_exec)(data.pre_data);
    err = execvp_noalloc(data.buf, argv[0], argv);
// If the exec succeeds, we don't get here
    CATCH_EINTR(ret = write(data.fd, &err, sizeof(err)));
    return 0;
    }
// Returns either the pid of the child process we run or -E* on failure.
#[no_mangle]
pub unsafe extern "C" fn run_helper(): *mut *mut void (pre_exec)(void, pre_data: *mut c_void, argv: *mut c_char) -> c_int {
    int run_helper(void (*pre_exec)(void *), void *pre_data, char **argv)
    {
    struct helper_data data;
    unsigned long stack, sp;
    int pid, fds[2], ret, n;
    stack = alloc_stack(0, __uml_cant_sleep());
    if (stack == 0)
    return -ENOMEM;
    ret = socketpair(AF_UNIX, SOCK_STREAM, 0, fds);
    if (ret < 0) {
    ret = -errno;
    printk(UM_KERN_ERR "run_helper : pipe failed, errno = %d\n",
    errno);
    goto out_free;
    }
    ret = os_set_exec_close(fds[1]);
    if (ret < 0) {
    printk(UM_KERN_ERR "run_helper : setting FD_CLOEXEC failed, "
    "ret = %d\n", -ret);
    goto out_close;
    }
    sp = stack + UM_KERN_PAGE_SIZE;
    data.pre_exec = pre_exec;
    data.pre_data = pre_data;
    data.argv = argv;
    data.fd = fds[1];
    data.buf = __uml_cant_sleep() ? uml_kmalloc(PATH_MAX, UM_GFP_ATOMIC) :
    uml_kmalloc(PATH_MAX, UM_GFP_KERNEL);
    pid = clone(helper_child, (void *) sp, CLONE_VM, &data);
    if (pid < 0) {
    ret = -errno;
    printk(UM_KERN_ERR "run_helper : clone failed, errno = %d\n",
    errno);
    goto out_free2;
    }
    close(fds[1]);
    fds[1] = -1;
//
// Read the errno value from the child, if the exec failed, or get 0 if
// the exec succeeded because the pipe fd was set as close-on-exec.
//
    n = read(fds[0], &ret, sizeof(ret));
    if (n == 0) {
    ret = pid;
    } else {
    if (n < 0) {
    n = -errno;
    printk(UM_KERN_ERR "run_helper : read on pipe failed, "
    "ret = %d\n", -n);
    ret = n;
    }
    CATCH_EINTR(waitpid(pid, core::ptr::null_mut(), __WALL));
    }
    if (ret < 0)
    printk(UM_KERN_ERR "run_helper : failed to exec %s on host: %s\n",
    argv[0], strerror(-ret));
    out_free2:
    kfree(data.buf);
    out_close:
    if (fds[1] != -1)
    close(fds[1]);
    close(fds[0]);
    out_free:
    free_stack(stack, 0);
    return ret;
    }
    int run_helper_thread(int (*proc)(void *), void *arg, unsigned int flags,
    unsigned long *stack_out)
    {
    unsigned long stack, sp;
    int pid, status, err;
// To share memory space, use os_run_helper_thread() instead.
    if (flags & CLONE_VM)
    return -EINVAL;
    stack = alloc_stack(0, __uml_cant_sleep());
    if (stack == 0)
    return -ENOMEM;
    sp = stack + UM_KERN_PAGE_SIZE;
    pid = clone(proc, (void *) sp, flags, arg);
    if (pid < 0) {
    err = -errno;
    printk(UM_KERN_ERR "run_helper_thread : clone failed, "
    "errno = %d\n", errno);
    return err;
    }
    if (stack_out == core::ptr::null_mut()) {
    CATCH_EINTR(pid = waitpid(pid, &status, __WALL));
    if (pid < 0) {
    err = -errno;
    printk(UM_KERN_ERR "run_helper_thread - wait failed, "
    "errno = %d\n", errno);
    pid = err;
    }
    if (!WIFEXITED(status) || (WEXITSTATUS(status) != 0))
    printk(UM_KERN_ERR "run_helper_thread - thread "
    "returned status 0x%x\n", status);
    free_stack(stack, 0);
    } else
// stack_out = stack;
    return pid;
    }
#[no_mangle]
pub unsafe extern "C" fn helper_wait(pid: c_int) -> c_int {
    int helper_wait(int pid)
    {
    int ret, status;
    let mut wflags: c_int = __WALL;
    CATCH_EINTR(ret = waitpid(pid, &status, wflags));
    if (ret < 0) {
    printk(UM_KERN_ERR "helper_wait : waitpid process %d failed, "
    "errno = %d\n", pid, errno);
    return -errno;
    } else if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
    printk(UM_KERN_ERR "helper_wait : process %d exited with "
    "status 0x%x\n", pid, status);
    return -ECHILD;
    } else
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_helper_thread {
    pub handle: pthread_t,
}

    int os_run_helper_thread(struct os_helper_thread **td_out,
    void *(*routine)(void *), void *arg)
    {
    struct os_helper_thread *td;
    sigset_t sigset, oset;
    int err, flags;
    flags = __uml_cant_sleep() ? UM_GFP_ATOMIC : UM_GFP_KERNEL;
    td = uml_kmalloc(sizeof(*td), flags);
    if (!td)
    return -ENOMEM;
    sigfillset(&sigset);
    if (sigprocmask(SIG_SETMASK, &sigset, &oset) < 0) {
    err = -errno;
    kfree(td);
    return err;
    }
    err = pthread_create(&td.handle, core::ptr::null_mut(), routine, arg);
    if (sigprocmask(SIG_SETMASK, &oset, core::ptr::null_mut()) < 0)
    panic("Failed to restore the signal mask: %d", errno);
    if (err != 0)
    kfree(td);
    else
// td_out = td;
    return -err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_kill_helper_thread(td: *mut os_helper_thread) {
    void os_kill_helper_thread(struct os_helper_thread *td)
    {
    pthread_cancel(td.handle);
    pthread_join(td.handle, core::ptr::null_mut());
    kfree(td);
    }
#[no_mangle]
pub unsafe extern "C" fn os_fix_helper_thread_signals() {
    void os_fix_helper_thread_signals(void)
    {
    sigset_t sigset;
    sigemptyset(&sigset);
    sigaddset(&sigset, SIGWINCH);
    sigaddset(&sigset, SIGPIPE);
    sigaddset(&sigset, SIGPROF);
    sigaddset(&sigset, SIGINT);
    sigaddset(&sigset, SIGTERM);
    sigaddset(&sigset, SIGCHLD);
    sigaddset(&sigset, SIGALRM);
    sigaddset(&sigset, SIGIO);
    sigaddset(&sigset, SIGUSR1);
    pthread_sigmask(SIG_SETMASK, &sigset, core::ptr::null_mut());
    }
