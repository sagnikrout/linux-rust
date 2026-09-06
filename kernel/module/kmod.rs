//! Automatically rewritten from C to Rust
//! Source: kernel/module/kmod.c
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
// kmod - the kernel module loader
//
// Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
//

//
// Assuming:
//
// threads = div64_u64((u64) totalram_pages * (u64) PAGE_SIZE,
// (u64) THREAD_SIZE * 8UL);
//
// If you need less than 50 threads would mean we're dealing with systems
// smaller than 3200 pages. This assumes you are capable of having ~13M memory,
// and this would only be an upper limit, after which the OOM killer would take
// effect. Systems like these are very unlikely if modules are enabled.
//
pub const MAX_KMOD_CONCURRENT: c_int = 50;
    static DEFINE_SEMAPHORE(kmod_concurrent_max, MAX_KMOD_CONCURRENT);
//
// This is a restriction on having *all* MAX_KMOD_CONCURRENT threads
// running at the same time without returning. When this happens we
// believe you've somehow ended up with a recursive module dependency
// creating a loop.
//
// We have no option but to fail.
//
// Userspace should proactively try to detect and prevent these.
//
pub const MAX_KMOD_ALL_BUSY_TIMEOUT: c_int = 5;
//
    modprobe_path is set via /proc/sys.
//
    char modprobe_path[KMOD_PATH_LEN] = CONFIG_MODPROBE_PATH;
#[no_mangle]
unsafe extern "C" fn free_modprobe_argv(info: *mut subprocess_info) {
    static void free_modprobe_argv(struct subprocess_info *info)
    {
    kfree(info.argv[3]); /* check call_modprobe() */
    kfree(info.argv);
    }
#[no_mangle]
unsafe extern "C" fn call_modprobe(orig_module_name: *mut c_char, wait: c_int) -> c_int {
    static int call_modprobe(char *orig_module_name, int wait)
    {
    struct subprocess_info *info;
    static char *envp[] = {
    "HOME=/",
    "TERM=linux",
    "PATH=/sbin:/usr/sbin:/bin:/usr/bin",
    core::ptr::null_mut()
    };
    char *module_name;
    int ret;
    char **argv = kmalloc(sizeof(char *[5]), GFP_KERNEL);
    if (!argv)
    goto out;
    module_name = kstrdup(orig_module_name, GFP_KERNEL);
    if (!module_name)
    goto free_argv;
    argv[0] = modprobe_path;
    argv[1] = "-q";
    argv[2] = "--";
    argv[3] = module_name;	/* check free_modprobe_argv() */
    argv[4] = core::ptr::null_mut();
    info = call_usermodehelper_setup(modprobe_path, argv, envp, GFP_KERNEL,
    core::ptr::null_mut(), free_modprobe_argv, core::ptr::null_mut());
    if (!info)
    goto free_module_name;
    ret = call_usermodehelper_exec(info, wait | UMH_KILLABLE);
    kmod_dup_request_announce(orig_module_name, ret);
    return ret;
    free_module_name:
    kfree(module_name);
    free_argv:
    kfree(argv);
    out:
    kmod_dup_request_announce(orig_module_name, -ENOMEM);
    return -ENOMEM;
    }
//
// __request_module - try to load a kernel module
// @wait: wait (or not) for the operation to complete
// @fmt: printf style format string for the name of the module
// @...: arguments as specified in the format string
//
// Load a module using the user mode module loader. The function returns
// zero on success or a negative errno code or positive exit code from
// "modprobe" on failure. Note that a successful module load does not mean
// the module did not then unload and exit on an error of its own. Callers
// must check that the service they requested is now available not blindly
// invoke it.
//
// If module auto-loading support is disabled then this function
// simply returns -ENOENT.
//
#[no_mangle]
pub unsafe extern "C" fn __request_module(wait: bool, fmt: *const c_char, ...) -> c_int {
    int __request_module(bool wait, const char *fmt, ...)
    {
    va_list args;
    char module_name[MODULE_NAME_LEN];
    int ret, dup_ret;
//
// We don't allow synchronous module loading from async.  Module
// init may invoke async_synchronize_full() which will end up
// waiting for this task which already is waiting for the module
// loading to complete, leading to a deadlock.
//
    WARN_ON_ONCE(wait && current_is_async());
    if (!modprobe_path[0])
    return -ENOENT;
    va_start(args, fmt);
    ret = vsnprintf(module_name, MODULE_NAME_LEN, fmt, args);
    va_end(args);
    if (ret >= MODULE_NAME_LEN)
    return -ENAMETOOLONG;
    ret = security_kernel_module_request(module_name);
    if (ret)
    return ret;
    ret = down_timeout(&kmod_concurrent_max, MAX_KMOD_ALL_BUSY_TIMEOUT * HZ);
    if (ret) {
    pr_warn_ratelimited("request_module: modprobe %s cannot be processed, kmod busy with %d threads for more than %d seconds now",
    module_name, MAX_KMOD_CONCURRENT, MAX_KMOD_ALL_BUSY_TIMEOUT);
    return ret;
    }
    trace_module_request(module_name, wait, _RET_IP_);
    if (kmod_dup_request_exists_wait(module_name, wait, &dup_ret)) {
    ret = dup_ret;
    goto out;
    }
    ret = call_modprobe(module_name, wait ? UMH_WAIT_PROC : UMH_WAIT_EXEC);
    out:
    up(&kmod_concurrent_max);
    return ret;
    }
    EXPORT_SYMBOL(__request_module);
