//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pid_namespace.h
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

// MAX_PID_NS_LEVEL is needed for limiting size of 'struct pid'
pub const MAX_PID_NS_LEVEL: c_int = 32;

// modes for vm.memfd_noexec sysctl

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pid_namespace {
    pub idr: idr,
    pub rcu: rcu_head,
    pub pid_allocated: c_uint,

    pub memfd_noexec_scope: c_int,

    pub set: ctl_table_set,
    pub sysctls: *mut ctl_table_header,

    pub child_reaper: *mut task_struct,
    pub pid_cachep: *mut kmem_cache,
    pub level: c_uint,
    pub pid_max: c_int,
    pub parent: *mut pid_namespace,

    pub bacct: *mut fs_pin,

    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub /: *mut *mut int reboot; / group exit code if this pidns was rebooted,
    pub ns: ns_common,
    pub work: work_struct,
    pub __randomize_layout: },
    pub init_pid_ns: extern struct pid_namespace,

    pub ns): return container_of(ns, struct pid_namespace,,
    pub ns: return,

    pub MEMFD_NOEXEC_SCOPE_EXEC: int scope =,
    pub ns->parent): for (; ns; ns =,
    pub READ_ONCE(ns->memfd_noexec_scope)): scope = max(scope,,
    pub scope: return,

    pub 0: return,

    pub ns): *mut *mut user_namespace user_ns, pid_namespace,
    pub pid_ns): *mut extern void zap_pid_ns_processes(struct pid_namespace,
    pub cmd): *mut *mut extern int reboot_pid_ns(struct pid_namespace pid_ns, int,
    pub ns): *mut extern void put_pid_ns(struct pid_namespace,
    pub ancestor): *mut pid_namespace,

    pub ns: return,
    pub 0: return,
    pub ERR_PTR(-EINVAL): ns =,
    pub ns: return,
    pub 0: return,
    pub false: return,

    pub tsk): *mut *mut extern struct pid_namespace task_active_pid_ns(struct task_struct,
    pub pidhash_init(void): c_void,
    pub pid_idr_init(void): c_void,
    pub pidns): *mut int register_pidns_sysctls(struct pid_namespace,
    pub pidns): *mut void unregister_pidns_sysctls(struct pid_namespace,
    pub &init_pid_ns: return task_active_pid_ns(tsk) ==,
