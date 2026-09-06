//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/proc_fs.h
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
// The proc filesystem constants/structures
//

//
// All /proc entries using this ->proc_ops instance are never removed.
//
// If in doubt, ignore this flag.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_ops {
    pub proc_flags: c_uint,
    pub ): *mut *mut *mut int (proc_open)(struct inode , struct file,
    pub ): *mut *mut *mut *mut ssize_t (proc_read)(struct file , char __user , size_t, loff_t,
    pub ): *mut *mut *mut ssize_t (proc_read_iter)(struct kiocb , struct iov_iter,
    pub ): *const *const *const *const ssize_t (proc_write)(struct file , char __user , size_t, loff_t,
// mandatory unless nonseekable_open() or equivalent is used
    pub int): *mut *mut *mut loff_t (proc_lseek)(struct file , loff_t,,
    pub ): *mut *mut *mut int (proc_release)(struct inode , struct file,
    pub ): *mut *mut *mut __poll_t (proc_poll)(struct file , struct poll_table_struct,
    pub long): *mut *mut *mut long (proc_ioctl)(struct file , unsigned int, unsigned,

    pub long): *mut *mut *mut long (proc_compat_ioctl)(struct file , unsigned int, unsigned,

    pub ): *mut *mut *mut int (proc_mmap)(struct file , struct vm_area_struct,
    pub long): *mut *mut *mut unsigned long (proc_get_unmapped_area)(struct file , unsigned long, unsigned long, unsigned long, unsigned,
    pub __randomize_layout: },
// definitions for hide_pid field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proc_hidepid {
    HIDEPID_OFF	  = 0,
    HIDEPID_NO_ACCESS = 1,
    HIDEPID_INVISIBLE = 2,
    HIDEPID_NOT_PTRACEABLE = 4, /* Limit pids to only ptraceable pids */
}

// definitions for proc mount option pidonly
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proc_pidonly {
    PROC_PIDONLY_OFF = 0,
    PROC_PIDONLY_ON  = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_fs_info {
    pub pid_ns: *mut pid_namespace,
    pub pid_gid: kgid_t,
    pub mounter_cred: *const cred,
    pub hide_pid: proc_hidepid,
    pub pidonly: proc_pidonly,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn int(: *mut *mut proc_write_t)(struct file, : *mut c_char, _arg: usize) -> typedef;
}
extern "C" {
    pub fn proc_root_init();
}
extern "C" {
    pub fn proc_flush_pid(: *mut pid);
}

extern "C" {
    pub fn proc_set_size(: *mut proc_dir_entry, _arg: loff_t);
}
extern "C" {
    pub fn proc_set_user(: *mut proc_dir_entry, _arg: kuid_t, _arg: kgid_t);
}
//
// Obtain the private data passed by user through proc_create_data() or
// related.
//
extern "C" {
    pub fn proc_remove(: *mut proc_dir_entry);
}
extern "C" {
    pub fn remove_proc_entry(: *const c_char, : *mut proc_dir_entry);
}
extern "C" {
    pub fn remove_proc_subtree(: *const c_char, : *mut proc_dir_entry) -> c_int;
}

extern "C" {
    pub fn bpf_iter_init_seq_net(priv_data: *mut c_void, aux: *mut bpf_iter_aux_info) -> c_int;
}
extern "C" {
    pub fn bpf_iter_fini_seq_net(priv_data: *mut c_void);
}

//
// The architecture which selects CONFIG_PROC_PID_ARCH_STATUS must
// provide proc_pid_arch_status() definition.
//

extern "C" {
    pub fn arch_report_meminfo(m: *mut seq_file);
}
extern "C" {
    pub fn arch_proc_pid_thread_features(m: *mut seq_file, task: *mut task_struct);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EBADF) -> return;
}

extern "C" {
    pub fn _proc_mkdir(_arg: name, _arg: 0, _arg: parent, _arg: net, _arg: true) -> return;
}
// get the associated pid namespace for a file in procfs
extern "C" {
    pub fn proc_ns_file(file: *const file) -> bool;
}

extern "C" {
    pub fn impl_proc_make_permanent(pde: *mut proc_dir_entry);
}

// Don't give matches to modules.

