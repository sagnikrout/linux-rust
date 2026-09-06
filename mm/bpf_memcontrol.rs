//! Automatically rewritten from C to Rust
//! Source: mm/bpf_memcontrol.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Memory Controller-related BPF kfuncs and auxiliary code
//
// Author: Roman Gushchin <roman.gushchin@linux.dev>
//

    __bpf_kfunc_start_defs();
//
// bpf_get_root_mem_cgroup - Returns a pointer to the root memory cgroup
//
// The function has KF_ACQUIRE semantics, even though the root memory
// cgroup is never destroyed after being created and doesn't require
// reference counting. And it's perfectly safe to pass it to
// bpf_put_mem_cgroup()
//
// Return: A pointer to the root memory cgroup.
//
    __bpf_kfunc struct mem_cgroup *bpf_get_root_mem_cgroup(void)
    {
    if (mem_cgroup_disabled()) {
    return core::ptr::null_mut();
    }
// css_get() is not needed
    return root_mem_cgroup;
    }
//
// bpf_get_mem_cgroup - Get a reference to a memory cgroup
// @css: pointer to the css structure
//
// It's fine to pass a css which belongs to any cgroup controller,
// e.g. unified hierarchy's main css.
//
// Implements KF_ACQUIRE semantics.
//
// Return: A pointer to a mem_cgroup structure after bumping
// the corresponding css's reference counter.
//
    __bpf_kfunc struct mem_cgroup *
    bpf_get_mem_cgroup(cgroup_subsys_state *css)
    {
    let mut memcg = core::ptr::null_mut();
pub static mut rcu_unlock: bool = false;
    if (mem_cgroup_disabled() || !root_mem_cgroup) {
    return core::ptr::null_mut();
    }
    if (root_mem_cgroup.css.ss != css.ss) {
    let mut cgroup = css.cgroup;
pub static mut ssid: c_int = 0;
    rcu_read_lock();
    rcu_unlock = true;
    css = rcu_dereference_raw(cgroup.subsys[ssid]);
    }
    if (css && css_tryget(css)) {
    memcg = container_of!(css, mem_cgroup, css);
    }
    if (rcu_unlock) {
    rcu_read_unlock();
    }
    return memcg;
    }
//
// bpf_put_mem_cgroup - Put a reference to a memory cgroup
// @memcg: memory cgroup to release
//
// Releases a previously acquired memcg reference.
// Implements KF_RELEASE semantics.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_put_mem_cgroup(memcg: *mut mem_cgroup) -> __bpf_kfunc void {
    css_put(&memcg.css);
    }
//
// bpf_mem_cgroup_vm_events - Read memory cgroup's vm event counter
// @memcg: memory cgroup
// @event: event id
//
// Allows to read memory cgroup event counters.
//
// Return: The current value of the corresponding events counter.
//
    __bpf_kfunc unsigned long bpf_mem_cgroup_vm_events(mem_cgroup *memcg,
    enum vm_event_item event)
    {
    if (unlikely(!memcg_vm_event_item_valid(event))) {
    return (unsigned long)-1;
    }
    return memcg_events(memcg, event);
    }
//
// bpf_mem_cgroup_usage - Read memory cgroup's usage
// @memcg: memory cgroup
//
// Please, note that the root memory cgroup it special and is exempt
// from the memory accounting. The returned value is a sum of sub-cgroup's
// usages and it not reflecting the size of the root memory cgroup itself.
// If you need to get an approximation, you can use root level statistics:
// e.g. NR_FILE_PAGES + NR_ANON_MAPPED.
//
// Return: The current memory cgroup size in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_usage(memcg: *mut mem_cgroup) -> __bpf_kfunc unsigned long {
    return page_counter_read(&memcg.memory) * PAGE_SIZE;
    }
//
// bpf_mem_cgroup_memory_events - Read memory cgroup's memory event value
// @memcg: memory cgroup
// @event: memory event id
//
// Return: The current value of the memory event counter.
//
    __bpf_kfunc unsigned long bpf_mem_cgroup_memory_events(mem_cgroup *memcg,
    enum memcg_memory_event event)
    {
    if (unlikely(event >= MEMCG_NR_MEMORY_EVENTS)) {
    return (unsigned long)-1;
    }
    return atomic_long_read(&memcg.memory_events[event]);
    }
//
// bpf_mem_cgroup_page_state - Read memory cgroup's page state counter
// @memcg: memory cgroup
// @idx: counter idx
//
// Allows to read memory cgroup statistics. The output is in bytes.
//
// Return: The value of the page state counter in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_page_state(memcg: *mut mem_cgroup, idx: c_int) -> __bpf_kfunc unsigned long {
    if (unlikely(!memcg_stat_item_valid(idx))) {
    return (unsigned long)-1;
    }
    return memcg_page_state_output(memcg, idx);
    }
//
// bpf_mem_cgroup_flush_stats - Flush memory cgroup's statistics
// @memcg: memory cgroup
//
// Propagate memory cgroup's statistics up the cgroup tree.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_flush_stats(memcg: *mut mem_cgroup) -> __bpf_kfunc void {
    mem_cgroup_flush_stats(memcg);
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(bpf_memcontrol_kfuncs)
    BTF_ID_FLAGS(func, bpf_get_root_mem_cgroup, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_get_mem_cgroup, KF_ACQUIRE | KF_RET_NULL | KF_RCU)
    BTF_ID_FLAGS(func, bpf_put_mem_cgroup, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_vm_events)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_memory_events)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_usage)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_page_state)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_flush_stats, KF_SLEEPABLE)
    BTF_KFUNCS_END(bpf_memcontrol_kfuncs)
pub static mut btf_kfunc_id_set: usize = 0;
#[no_mangle]
unsafe extern "C" fn bpf_memcontrol_init() -> c_int {
    let mut err = 0;
    err = register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC,
    &bpf_memcontrol_kfunc_set);
    if (err) {
    pr_warn!("error while registering bpf memcontrol kfuncs: %d", err);
    }
    return err;
    }
    late_initcall!(bpf_memcontrol_init);