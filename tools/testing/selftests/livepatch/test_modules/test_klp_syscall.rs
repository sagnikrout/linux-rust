//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_modules/test_klp_syscall.c
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
// Copyright (C) 2017-2023 SUSE
// Authors: Libor Pechacek <lpechacek@suse.cz>
// Nicolai Stange <nstange@suse.de>
// Marcos Paulo de Souza <mpdesouza@suse.com>
//

//
// Before CONFIG_ARCH_HAS_SYSCALL_WRAPPER was introduced there were no
// prefixes for system calls.
// powerpc set this config based on configs, so it can be enabled or not.
//

// Macro flag: #define FN_PREFIX

// Do not set a prefix for architectures that do not enable wrappers.
// Macro flag: #define FN_PREFIX

// Protects klp_pids
    static DEFINE_MUTEX(kpid_mutex);
    static unsigned int npids, npids_pending;
    static int klp_pids[NR_CPUS];
    module_param_array(klp_pids, int, &npids_pending, 0);
    MODULE_PARM_DESC(klp_pids, "Array of pids to be transitioned to livepatched state.");
    static ssize_t npids_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "%u\n", npids_pending);
    }
    let mut klp_attr: static struct kobj_attribute = __ATTR_RO(npids);
    static struct kobject *klp_kobj;
#[no_mangle]
unsafe extern "C" fn lp_sys_getpid() -> asmlinkage long {
    static asmlinkage long lp_sys_getpid(void)
    {
    int i;
    mutex_lock(&kpid_mutex);
    if (npids_pending > 0) {
    for (i = 0; i < npids; i++) {
    if (current.pid == klp_pids[i]) {
    klp_pids[i] = 0;
    npids_pending--;
    break;
    }
    }
    }
    mutex_unlock(&kpid_mutex);
    return task_tgid_vnr(current);
    }
    static struct klp_func vmlinux_funcs[] = {
    {
    .old_name = __stringify(FN_PREFIX) "sys_getpid",
    .new_func = lp_sys_getpid,
    }, {}
    };
    static struct klp_object objs[] = {
    {
// name being NULL means vmlinux
    .funcs = vmlinux_funcs,
    }, {}
    };
    static struct klp_patch patch = {
    .mod = THIS_MODULE,
    .objs = objs,
    };
#[no_mangle]
unsafe extern "C" fn livepatch_init() -> c_int {
    static int livepatch_init(void)
    {
    int ret;
    klp_kobj = kobject_create_and_add("test_klp_syscall", kernel_kobj);
    if (!klp_kobj)
    return -ENOMEM;
    ret = sysfs_create_file(klp_kobj, &klp_attr.attr);
    if (ret) {
    kobject_put(klp_kobj);
    return ret;
    }
//
// Save the number pids to transition to livepatched state before the
// number of pending pids is decremented.
//
    npids = npids_pending;
    ret = klp_enable_patch(&patch);
    if (ret)
    kobject_put(klp_kobj);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn livepatch_exit() {
    static void livepatch_exit(void)
    {
    kobject_put(klp_kobj);
    }
    module_init(livepatch_init);
    module_exit(livepatch_exit);
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
    MODULE_AUTHOR("Libor Pechacek <lpechacek@suse.cz>");
    MODULE_AUTHOR("Nicolai Stange <nstange@suse.de>");
    MODULE_AUTHOR("Marcos Paulo de Souza <mpdesouza@suse.com>");
    MODULE_DESCRIPTION("Livepatch test: syscall transition");
