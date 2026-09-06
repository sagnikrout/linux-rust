//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/main.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
unsafe extern "C" fn set_stklim() -> void __init {
    static void __init set_stklim(void)
    {
    struct rlimit lim;
    if (getrlimit(RLIMIT_STACK, &lim) < 0) {
    perror("getrlimit");
    exit(1);
    }
    if ((lim.rlim_cur == RLIM_INFINITY) || (lim.rlim_cur > STACKSIZE)) {
    lim.rlim_cur = STACKSIZE;
    if (setrlimit(RLIMIT_STACK, &lim) < 0) {
    perror("setrlimit");
    exit(1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn last_ditch_exit(sig: c_int) {
    static void last_ditch_exit(int sig)
    {
    uml_cleanup();
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn install_fatal_handler(sig: c_int) -> void __init {
    static void __init install_fatal_handler(int sig)
    {
    struct sigaction action;
// All signals are enabled in this handler ...
    sigemptyset(&action.sa_mask);
//
// ... including the signal being handled, plus we want the
// handler reset to the default behavior, so that if an exit
// handler is hanging for some reason, the UML will just die
// after this signal is sent a second time.
//
    action.sa_flags = SA_RESETHAND | SA_NODEFER;
    action.sa_restorer = core::ptr::null_mut();
    action.sa_handler = last_ditch_exit;
    if (sigaction(sig, &action, core::ptr::null_mut()) < 0) {
    os_warn("failed to install handler for signal %d "
    "- errno = %d\n", sig, errno);
    exit(1);
    }
    }

#[no_mangle]
unsafe extern "C" fn setup_env_path() -> void __init {
    static void __init setup_env_path(void)
    {
    char *new_path = core::ptr::null_mut();
    char *old_path = core::ptr::null_mut();
    let mut path_len: c_int = 0;
    old_path = getenv("PATH");
//
// if no PATH variable is set or it has an empty value
// just use the default + /usr/lib/uml
//
    if (!old_path || (path_len = strlen(old_path)) == 0) {
    if (putenv("PATH=:/bin:/usr/bin/" UML_LIB_PATH))
    perror("couldn't putenv");
    return;
    }
// append /usr/lib/uml to the existing path
    path_len += strlen("PATH=" UML_LIB_PATH) + 1;
    new_path = malloc(path_len);
    if (!new_path) {
    perror("couldn't malloc to set a new PATH");
    return;
    }
    snprintf(new_path, path_len, "PATH=%s" UML_LIB_PATH, old_path);
    if (putenv(new_path)) {
    perror("couldn't putenv to set a new PATH");
    free(new_path);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char, envp: *mut c_char) -> int __init {
    int __init main(int argc, char **argv, char **envp)
    {
    char **new_argv;
    int ret, i, err;
// Disable randomization and re-exec if it was changed successfully
    ret = personality(PER_LINUX | ADDR_NO_RANDOMIZE);
    if (ret >= 0 && (ret & (PER_LINUX | ADDR_NO_RANDOMIZE)) !=
    (PER_LINUX | ADDR_NO_RANDOMIZE)) {
    char buf[4096] = {};
    ssize_t ret;
    ret = readlink("/proc/self/exe", buf, sizeof(buf));
    if (ret < 0 || ret >= sizeof(buf)) {
    perror("readlink failure");
    exit(1);
    }
    execve(buf, argv, envp);
    }
    set_stklim();
    setup_env_path();
    setsid();
    new_argv = malloc((argc + 1) * sizeof(char *));
    if (new_argv == core::ptr::null_mut()) {
    perror("Mallocing argv");
    exit(1);
    }
    for (i = 0; i < argc; i++) {
    new_argv[i] = strdup(argv[i]);
    if (new_argv[i] == core::ptr::null_mut()) {
    perror("Mallocing an arg");
    exit(1);
    }
    }
    new_argv[argc] = core::ptr::null_mut();
//
// Allow these signals to bring down a UML if all other
// methods of control fail.
//
    install_fatal_handler(SIGINT);
    install_fatal_handler(SIGTERM);
    scan_elf_aux(envp);
    change_sig(SIGPIPE, 0);
    ret = linux_main(argc, argv, envp);
//
// Disable SIGPROF - I have no idea why libc doesn't do this or turn
// off the profiling time, but UML dies with a SIGPROF just before
// exiting when profiling is active.
//
    change_sig(SIGPROF, 0);
//
// This signal stuff used to be in the reboot case.  However,
// sometimes a timer signal can come in when we're halting (reproducably
// when writing out gcov information, presumably because that takes
// some time) and cause a segfault.
//
// stop timers and set timer signal to be ignored
    os_timer_disable(0);
// disable SIGIO for the fds and set SIGIO to be ignored
    err = deactivate_all_fds();
    if (err)
    os_warn("deactivate_all_fds failed, errno = %d\n", -err);
//
// Let any pending signals fire now.  This ensures
// that they won't be delivered after the exec, when
// they are definitely not expected.
//
    unblock_signals();
    os_info("\n");
// Reboot
    if (ret) {
    execvp(new_argv[0], new_argv);
    perror("Failed to exec kernel");
    ret = 1;
    }
    return uml_exitcode;
    }
    extern void *__real_malloc(int);
    extern void __real_free(void *);
// workaround for -Wmissing-prototypes warnings
    void *__wrap_malloc(int size);
    void *__wrap_calloc(int n, int size);
    void __wrap_free(void *ptr);
    void *__wrap_malloc(int size)
    {
    void *ret;
    if (!kmalloc_ok)
    return __real_malloc(size);
#[no_mangle]
pub unsafe extern "C" fn if(UM_KERN_PAGE_SIZE: size <=) -> else {
    else if (size <= UM_KERN_PAGE_SIZE)
// finding contiguous pages can be hard
    ret = uml_kmalloc(size, UM_GFP_KERNEL);
    let mut ret: else = vmalloc(size);
//
// glibc people insist that if malloc fails, errno should be
// set by malloc as well. So we do.
//
    if (ret == core::ptr::null_mut())
    errno = ENOMEM;
    return ret;
    }
    void *__wrap_calloc(int n, int size)
    {
    void *ptr = __wrap_malloc(n * size);
    if (ptr == core::ptr::null_mut())
    return core::ptr::null_mut();
    memset(ptr, 0, n * size);
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn __wrap_free(ptr: *mut c_void) {
    void __wrap_free(void *ptr)
    {
    let mut addr: c_ulong = (unsigned long) ptr;
//
// We need to know how the allocation happened, so it can be correctly
// freed.  This is done by seeing what region of memory the pointer is
// in -
// physical memory - kmalloc/kfree
// kernel virtual memory - vmalloc/vfree
// anywhere else - malloc/free
// If kmalloc is not yet possible, then either high_physmem and/or
// end_vm are still 0 (as at startup), in which case we call free, or
// we have set them, but anyway addr has not been allocated from those
// areas. So, in both cases __real_free is called.
//
// CAN_KMALLOC is checked because it would be bad to free a buffer
// with kmalloc/vmalloc after they have been turned off during
// shutdown.
// XXX: However, we sometimes shutdown CAN_KMALLOC temporarily, so
// there is a possibility for memory leaks.
//
    if ((addr >= uml_physmem) && (addr < high_physmem)) {
    if (kmalloc_ok)
    kfree(ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn if(end_vm): (addr >= start_vm) && (addr <) -> else {
    if (kmalloc_ok)
    vfree(ptr);
    }
    else __real_free(ptr);
    }
