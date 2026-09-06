//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/helpers.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
//

// If kernel subsystem is allowing eBPF programs to call this function,
// inside its own verifier_ops->get_func_proto() callback it should return
// bpf_map_lookup_elem_proto, so that verifier can properly check the arguments
//
// Different map implementations will rely on rcu in map methods
// lookup/update/delete, therefore eBPF programs must run under rcu lock
// if program is allowed to access maps, so check rcu_read_lock_held() or
// rcu_read_lock_trace_held() in all three functions.
//
    BPF_CALL_2(bpf_map_lookup_elem, bpf_map *, map, void *, key)
    {
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    return (unsigned long) map.ops.map_lookup_elem(map, key);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_map_update_elem, bpf_map *, map, void *, key,
    void *, value, u64, flags)
    {
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    return map.ops.map_update_elem(map, key, value, flags);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_map_delete_elem, bpf_map *, map, void *, key)
    {
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    return map.ops.map_delete_elem(map, key);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_map_push_elem, bpf_map *, map, void *, value, u64, flags)
    {
    return map.ops.map_push_elem(map, value, flags);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_map_pop_elem, bpf_map *, map, void *, value)
    {
    return map.ops.map_pop_elem(map, value);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_map_peek_elem, bpf_map *, map, void *, value)
    {
    return map.ops.map_peek_elem(map, value);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_map_lookup_percpu_elem, bpf_map *, map, void *, key, u32, cpu)
    {
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    return (unsigned long) map.ops.map_lookup_percpu_elem(map, key, cpu);
    }
pub static mut bpf_func_proto: usize = 0;
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_get_smp_processor_id)
    {
    return smp_processor_id();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_get_numa_node_id)
    {
    return numa_node_id();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_ktime_get_ns)
    {
// NMI safe access to clock monotonic
    return ktime_get_mono_fast_ns();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_ktime_get_boot_ns)
    {
// NMI safe access to clock boottime
    return ktime_get_boot_fast_ns();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_ktime_get_coarse_ns)
    {
    return ktime_get_coarse_ns();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_ktime_get_tai_ns)
    {
// NMI safe access to clock tai
    return ktime_get_tai_fast_ns();
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_get_current_pid_tgid)
    {
    let mut task = current;
    if (unlikely(!task)) {
    return -EINVAL;
    }
    return (u64) task.tgid << 32 | task.pid;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_0(bpf_get_current_uid_gid)
    {
    let mut task = current;
    let mut uid;
    let mut gid;
    if (unlikely(!task)) {
    return -EINVAL;
    }
    current_uid_gid(&uid, &gid);
    return (u64) from_kgid(&init_user_ns, gid) << 32 |
    from_kuid(&init_user_ns, uid);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_get_current_comm, char *, buf, u32, size)
    {
    let mut task = current;
    if (unlikely(!task)) {
// goto;
    }
// Verifier guarantees that size > 0
    strscpy_pad(buf, task.comm, size);
    return 0;
// label;
    memset(buf, 0, size);
    return -EINVAL;
    }
pub static mut bpf_func_proto: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn __bpf_spin_lock(lock: *mut bpf_spin_lock) {
    let mut l = lock;
    union {
    let mut val = 0;
    let mut lock;
    } u = { .lock = __ARCH_SPIN_LOCK_UNLOCKED };
    compiletime_assert(u.val == 0, "__ARCH_SPIN_LOCK_UNLOCKED not 0");
    BUILD_BUG_ON!(sizeof!(*l) != sizeof!(__u32));
    BUILD_BUG_ON!(sizeof!(*lock) != sizeof!(__u32));
    preempt_disable();
    arch_spin_lock(l);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_spin_unlock(lock: *mut bpf_spin_lock) {
    let mut l = lock;
    arch_spin_unlock(l);
    preempt_enable();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __bpf_spin_lock
pub unsafe extern "C" fn __bpf_spin_lock_dup(lock: *mut bpf_spin_lock) {
    let mut l = lock;
    BUILD_BUG_ON!(sizeof!(*l) != sizeof!(*lock));
    do {
    atomic_cond_read_relaxed(l, !VAL);
    } while (atomic_xchg(l, 1));
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: __bpf_spin_unlock
pub unsafe extern "C" fn __bpf_spin_unlock_dup(lock: *mut bpf_spin_lock) {
    let mut l = lock;
    atomic_set_release(l, 0);
    }

pub static mut unsigned long: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_spin_lock_irqsave(lock: *mut bpf_spin_lock) {
    let mut flags = 0;
    local_irq_save(flags);
    __bpf_spin_lock(lock);
    __this_cpu_write(irqsave_flags, flags);
    }
    NOTRACE_BPF_CALL_1(bpf_spin_lock, bpf_spin_lock *, lock)
    {
    __bpf_spin_lock_irqsave(lock);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_spin_unlock_irqrestore(lock: *mut bpf_spin_lock) {
    let mut flags = 0;
    flags = __this_cpu_read(irqsave_flags);
    __bpf_spin_unlock(lock);
    local_irq_restore(flags);
    }
    NOTRACE_BPF_CALL_1(bpf_spin_unlock, bpf_spin_lock *, lock)
    {
    __bpf_spin_unlock_irqrestore(lock);
    return 0;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn copy_map_value_locked(map: *mut bpf_map, dst: *mut c_void, src: *mut c_void, lock_src: bool) {
pub static mut lock: *mut c_void = core::ptr::null_mut();
    if (lock_src) {
    lock = src + map.record.spin_lock_off;
    }
    else {
    lock = dst + map.record.spin_lock_off;
    }
    preempt_disable();
    __bpf_spin_lock_irqsave(lock);
    copy_map_value(map, dst, src);
    __bpf_spin_unlock_irqrestore(lock);
    preempt_enable();
    }
    BPF_CALL_0(bpf_jiffies64)
    {
    return get_jiffies_64();
    }
pub static mut bpf_func_proto: usize = 0;

    BPF_CALL_0(bpf_get_current_cgroup_id)
    {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    let mut cgrp_id = 0;
    rcu_read_lock();
    cgrp = task_dfl_cgroup(current);
    cgrp_id = cgroup_id(cgrp);
    rcu_read_unlock();
    return cgrp_id;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_1(bpf_get_current_ancestor_cgroup_id, int, ancestor_level)
    {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
pub static mut ancestor: *mut c_void = core::ptr::null_mut();
    let mut cgrp_id = 0;
    rcu_read_lock();
    cgrp = task_dfl_cgroup(current);
    ancestor = cgroup_ancestor(cgrp, ancestor_level);
    cgrp_id = ancestor ? cgroup_id(ancestor) : 0;
    rcu_read_unlock();
    return cgrp_id;
    }
pub static mut bpf_func_proto: usize = 0;

pub const BPF_STRTOX_BASE_MASK: c_uint = 0x1F;
#[no_mangle]
pub unsafe extern "C" fn __bpf_strtoull(buf: *mut c_char, buf_len: size_t, flags: u64, res: *mut unsigned long long, is_negative: *mut bool) -> c_int {
pub static mut base: c_uint = 0;
    let mut cur_buf = buf;
pub static mut cur_len: usize = 0;
    let mut consumed = 0;
    let mut val_len = 0;
    char str[64];
    if (!buf || !buf_len || !res || !is_negative) {
    return -EINVAL;
    }
    if (base != 0 && base != 8 && base != 10 && base != 16) {
    return -EINVAL;
    }
    if (flags & ~BPF_STRTOX_BASE_MASK) {
    return -EINVAL;
    }
    while (cur_buf < buf + buf_len && isspace(*cur_buf)) {
    cur_buf += 1;
    }
// is_negative = (cur_buf < buf + buf_len && *cur_buf == '-');
    if (*is_negative) {
    cur_buf += 1;
    }
    consumed = cur_buf - buf;
    cur_len -= consumed;
    if (!cur_len) {
    return -EINVAL;
    }
    cur_len = min(cur_len, sizeof!(str) - 1);
    memcpy(str, cur_buf, cur_len);
    str[cur_len] = '\0';
    cur_buf = str;
    cur_buf = _parse_integer_fixup_radix(cur_buf, &base);
    val_len = _parse_integer(cur_buf, base, res);
    if (val_len & KSTRTOX_OVERFLOW) {
    return -ERANGE;
    }
    if (val_len == 0) {
    return -EINVAL;
    }
    cur_buf += val_len;
    consumed += cur_buf - str;
    return consumed;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_strtoll(buf: *mut c_char, buf_len: size_t, flags: u64, res: *mut long long) -> c_int {
    unsigned long long _res;
    let mut is_negative = 0;
    let mut err = 0;
    err = __bpf_strtoull(buf, buf_len, flags, &_res, &is_negative);
    if (err < 0) {
    return err;
    }
    if (is_negative) {
    if ((long long)-_res > 0) {
    return -ERANGE;
    }
// res = -_res;
    } else {
    if ((long long)_res < 0) {
    return -ERANGE;
    }
// res = _res;
    }
    return err;
    }
    BPF_CALL_4(bpf_strtol, const char *, buf, size_t, buf_len, u64, flags,
    s64 *, res)
    {
    long long _res;
    let mut err = 0;
// res = 0;
    err = __bpf_strtoll(buf, buf_len, flags, &_res);
    if (err < 0) {
    return err;
    }
// res = _res;
    return err;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_strtoul, const char *, buf, size_t, buf_len, u64, flags,
    u64 *, res)
    {
    unsigned long long _res;
    let mut is_negative = 0;
    let mut err = 0;
// res = 0;
    err = __bpf_strtoull(buf, buf_len, flags, &_res, &is_negative);
    if (err < 0) {
    return err;
    }
    if (is_negative) {
    return -EINVAL;
    }
// res = _res;
    return err;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_strncmp, const char *, s1, u32, s1_sz, const char *, s2)
    {
    return strncmp(s1, s2, s1_sz);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_4(bpf_get_ns_current_pid_tgid, u64, dev, u64, ino, bpf_pidns_info *, nsdata, u32, size)
    {
    let mut task = current;
pub static mut pidns: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (unlikely(size != sizeof!(bpf_pidns_info))) {
// goto;
    }
    if (unlikely((u64)(dev_t)dev != dev)) {
// goto;
    }
    if (unlikely(!task)) {
// goto;
    }
    pidns = task_active_pid_ns(task);
    if (unlikely(!pidns)) {
    err = -ENOENT;
// goto;
    }
    if (!ns_match(&pidns.ns, (dev_t)dev, ino)) {
// goto;
    }
    nsdata.pid = task_pid_nr_ns(task, pidns);
    nsdata.tgid = task_tgid_nr_ns(task, pidns);
    return 0;
// label;
    memset(nsdata, 0, (size_t) size);
    return err;
    }
pub static mut bpf_func_proto: usize = 0;
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_5(bpf_event_output_data, void *, ctx, bpf_map *, map,
    u64, flags, void *, data, u64, size)
    {
    if (unlikely(flags & ~(BPF_F_INDEX_MASK))) {
    return -EINVAL;
    }
    return bpf_event_output(map, flags, data, size, core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_copy_from_user, void *, dst, u32, size,
    const void  *, user_ptr)
    {
pub static mut ret: c_int = 0;
    if (unlikely(ret)) {
    memset(dst, 0, size);
    ret = -EFAULT;
    }
    return ret;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_5(bpf_copy_from_user_task, void *, dst, u32, size,
    const void  *, user_ptr, task_struct *, tsk, u64, flags)
    {
    let mut ret = 0;
// flags is not used yet
    if (unlikely(flags)) {
    return -EINVAL;
    }
    if (unlikely(!size)) {
    return 0;
    }
    ret = access_process_vm(tsk, (unsigned long)user_ptr, dst, size, 0);
    if (ret == size) {
    return 0;
    }
    memset(dst, 0, size);
// Return -EFAULT for partial read
    return ret < 0 ? ret : -EFAULT;
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_2(bpf_per_cpu_ptr, const void *, ptr, u32, cpu)
    {
    if (cpu >= nr_cpu_ids) {
    return (unsigned long)core::ptr::null_mut();
    }
    return (unsigned long)per_cpu_ptr((const uintptr_t)ptr, cpu);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_1(bpf_this_cpu_ptr, const void *, percpu_ptr)
    {
    return (unsigned long)this_cpu_ptr((const uintptr_t)percpu_ptr);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_trace_copy_string(buf: *mut c_char, unsafe_ptr: *mut c_void, fmt_ptype: c_char, bufsz: size_t) -> c_int {
    let mut user_ptr = unsafe_ptr;
    buf[0] = 0;
    match (fmt_ptype) {
    's' => {

    if ((unsigned long)unsafe_ptr < TASK_SIZE) {
    return strncpy_from_user_nofault(buf, user_ptr, bufsz);
    }
    fallthrough;

    }
    'k' => {
    return strncpy_from_kernel_nofault(buf, unsafe_ptr, bufsz);
    }
    'u' => {
    return strncpy_from_user_nofault(buf, user_ptr, bufsz);
    }
    }
    return -EINVAL;
    }
// Support executing three nested bprintf helper calls on a given CPU
pub const MAX_BPRINTF_NEST_LEVEL: c_int = 3;
pub static mut struct bpf_bprintf_buffers[MAX_BPRINTF_NEST_LEVEL]: usize = 0;
pub static mut int: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_try_get_buffers(bufs: *mut bpf_bprintf_buffers) -> c_int {
    let mut nest_level = 0;
    preempt_disable();
    nest_level = this_cpu_inc_return(bpf_bprintf_nest_level);
    if (WARN_ON_ONCE!(nest_level > MAX_BPRINTF_NEST_LEVEL)) {
    this_cpu_dec(bpf_bprintf_nest_level);
    preempt_enable();
    return -EBUSY;
    }
// bufs = this_cpu_ptr(&bpf_bprintf_bufs[nest_level - 1]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_put_buffers() {
    if (WARN_ON_ONCE!(this_cpu_read(bpf_bprintf_nest_level) == 0)) {
    return;
    }
    this_cpu_dec(bpf_bprintf_nest_level);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_bprintf_cleanup(data: *mut bpf_bprintf_data) {
    if (!data.bin_args && !data.buf) {
    return;
    }
    bpf_put_buffers();
    }
//
// bpf_bprintf_prepare - Generic pass on format strings for bprintf-like helpers
//
// Returns a negative value if fmt is an invalid format string or 0 otherwise.
//
// This can be used in two ways:
// - Format string verification only: when data->get_bin_args is false
// - Arguments preparation: in addition to the above verification, it writes in
// data->bin_args a binary representation of arguments usable by bstr_printf
// where pointers from BPF have been sanitized.
//
// In argument preparation mode, if 0 is returned, safe temporary buffers are
// allocated and bpf_bprintf_cleanup should be called to free them after use.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_bprintf_prepare(fmt: *mut c_char, fmt_size: u32, raw_args: *mut u64, num_args: u32, data: *mut bpf_bprintf_data) -> c_int {
pub static mut get_buffers: bool = false;
    let mut unsafe_ptr = core::ptr::null_mut(), *tmp_buf = core::ptr::null_mut(), *tmp_buf_end, *fmt_end;
    let mut buffers = core::ptr::null_mut();
    size_t sizeof_cur_arg, sizeof_cur_ip;
    int err, i, num_spec = 0;
    let mut cur_arg = 0;
    char fmt_ptype, cur_ip[16], ip_spec[] = "%pXX";
    fmt_end = strnchr(fmt, fmt_size, 0);
    if (!fmt_end) {
    return -EINVAL;
    }
    fmt_size = fmt_end - fmt;
    if (get_buffers && bpf_try_get_buffers(&buffers)) {
    return -EBUSY;
    }
    if (data.get_bin_args) {
    if (num_args) {
    tmp_buf = buffers.bin_args;
    }
    tmp_buf_end = tmp_buf + MAX_BPRINTF_BIN_ARGS;
    data.bin_args = tmp_buf;
    }
    if (data.get_buf) {
    data.buf = buffers.buf;
    }
    while (i < fmt_size) {
pub static mut c: c_uchar = 0;
//
// Permit bytes >= 0x80 in plain text so UTF-8 literals can pass
// through unchanged, while still rejecting ASCII control bytes.
//
    if (isascii(c) && !isprint(c) && !isspace(c)) {
    err = -EINVAL;
// goto;
    }
    if (fmt[i] != '%') {
    continue;
    }
    if (fmt[i + 1] == '%') {
    i += 1;
    continue;
    }
    if (num_spec >= num_args) {
    err = -EINVAL;
// goto;
    }
// The string is zero-terminated so if fmt[i] != 0, we can
// always access fmt[i + 1], in the worst case it will be a 0
//
    i += 1;
    c = fmt[i];
//
// The format parser below only understands ASCII conversion
// specifiers and modifiers, so reject non-ASCII after '%'.
//
    if (!isascii(c)) {
    err = -EINVAL;
// goto;
    }
// skip optional "[0 +-][num]" width formatting field
    while (fmt[i] == '0' || fmt[i] == '+'  || fmt[i] == '-' ||
    fmt[i] == ' ') {
    i += 1;
    }
    if (fmt[i] >= '1' && fmt[i] <= '9') {
    i += 1;
    while (fmt[i] >= '0' && fmt[i] <= '9') {
    i += 1;
    }
    }
    if (fmt[i] == 'p') {
    sizeof_cur_arg = sizeof!(long);
    if (fmt[i + 1] == 0 || isspace(fmt[i + 1]) ||
    ispunct(fmt[i + 1])) {
    if (tmp_buf) {
    cur_arg = raw_args[num_spec];
    }
// goto;
    }
    if ((fmt[i + 1] == 'k' || fmt[i + 1] == 'u') &&
    fmt[i + 2] == 's') {
    fmt_ptype = fmt[i + 1];
    i += 2;
// goto;
    }
    if (fmt[i + 1] == 'K' ||
    fmt[i + 1] == 'x' || fmt[i + 1] == 's' ||
    fmt[i + 1] == 'S') {
    if (tmp_buf) {
    cur_arg = raw_args[num_spec];
    }
    i += 1;
// goto;
    }
    if (fmt[i + 1] == 'B') {
    if (tmp_buf)  {
    err = snprintf(tmp_buf,
    (tmp_buf_end - tmp_buf),
    "%pB",
    (long)raw_args[num_spec]);
    tmp_buf += (err + 1);
    }
    i += 1;
    num_spec += 1;
    continue;
    }
// only support "%pI4", "%pi4", "%pI6" and "%pi6".
    if ((fmt[i + 1] != 'i' && fmt[i + 1] != 'I') ||
    (fmt[i + 2] != '4' && fmt[i + 2] != '6')) {
    err = -EINVAL;
// goto;
    }
    i += 2;
    if (!tmp_buf) {
// goto;
    }
    sizeof_cur_ip = (fmt[i] == '4') ? 4 : 16;
    if (tmp_buf_end - tmp_buf < sizeof_cur_ip) {
    err = -ENOSPC;
// goto;
    }
    unsafe_ptr = (long)raw_args[num_spec];
    err = copy_from_kernel_nofault(cur_ip, unsafe_ptr,
    sizeof_cur_ip);
    if (err < 0) {
    memset(cur_ip, 0, sizeof_cur_ip);
    }
// hack: bstr_printf expects IP addresses to be
// pre-formatted as strings, ironically, the easiest way
// to do that is to call snprintf.
//
    ip_spec[2] = fmt[i - 1];
    ip_spec[3] = fmt[i];
    err = snprintf(tmp_buf, tmp_buf_end - tmp_buf,
    ip_spec, &cur_ip);
    tmp_buf += err + 1;
    num_spec += 1;
    continue;
    } else if (fmt[i] == 's') {
    fmt_ptype = fmt[i];
// label;
    if (fmt[i + 1] != 0 &&
    !isspace(fmt[i + 1]) &&
    !ispunct(fmt[i + 1])) {
    err = -EINVAL;
// goto;
    }
    if (!tmp_buf) {
// goto;
    }
    if (tmp_buf_end == tmp_buf) {
    err = -ENOSPC;
// goto;
    }
    unsafe_ptr = (long)raw_args[num_spec];
    err = bpf_trace_copy_string(tmp_buf, unsafe_ptr,
    fmt_ptype,
    tmp_buf_end - tmp_buf);
    if (err < 0) {
    tmp_buf[0] = '\0';
    err = 1;
    }
    tmp_buf += err;
    num_spec += 1;
    continue;
    } else if (fmt[i] == 'c') {
    if (!tmp_buf) {
// goto;
    }
    if (tmp_buf_end == tmp_buf) {
    err = -ENOSPC;
// goto;
    }
// tmp_buf = raw_args[num_spec];
    tmp_buf += 1;
    num_spec += 1;
    continue;
    }
    sizeof_cur_arg = sizeof!(int);
    if (fmt[i] == 'l') {
    sizeof_cur_arg = sizeof!(long);
    i += 1;
    }
    if (fmt[i] == 'l') {
    sizeof_cur_arg = sizeof!(long long);
    i += 1;
    }
    if (fmt[i] != 'i' && fmt[i] != 'd' && fmt[i] != 'u' &&
    fmt[i] != 'x' && fmt[i] != 'X') {
    err = -EINVAL;
// goto;
    }
    if (tmp_buf) {
    cur_arg = raw_args[num_spec];
    }
// label;
    if (tmp_buf) {
    tmp_buf = PTR_ALIGN(tmp_buf, sizeof!(u32));
    if (tmp_buf_end - tmp_buf < sizeof_cur_arg) {
    err = -ENOSPC;
// goto;
    }
    if (sizeof_cur_arg == 8) {
// tmp_buf = *&cur_arg;
// (tmp_buf + 4) = *(&cur_arg + 1);
    } else {
// tmp_buf = (u32)(long)cur_arg;
    }
    tmp_buf += sizeof_cur_arg;
    }
    num_spec += 1;
    }
    err = 0;
// label;
    if (err) {
    bpf_bprintf_cleanup(data);
    }
    return err;
    }
    BPF_CALL_5(bpf_snprintf, char *, str, u32, str_size, char *, fmt,
    const void *, args, u32, data_len)
    {
pub static mut bpf_bprintf_data: usize = 0;
    let mut err = 0;
    let mut num_args = 0;
    if (data_len % 8 || data_len > MAX_BPRINTF_VARARGS * 8 ||
    (data_len && !args)) {
    return -EINVAL;
    }
    num_args = data_len / 8;
// ARG_PTR_TO_CONST_STR guarantees that fmt is zero-terminated so we
// can safely give an unbounded size.
//
    err = bpf_bprintf_prepare(fmt, UINT_MAX, args, num_args, &data);
    if (err < 0) {
    return err;
    }
    err = bstr_printf(str, str_size, fmt, data.bin_args);
    bpf_bprintf_cleanup(&data);
    return err + 1;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn map_key_from_value(map: *mut bpf_map, value: *mut c_void, arr_idx: *mut u32) -> *mut c_void {
    if (map.map_type == BPF_MAP_TYPE_ARRAY) {
    let mut array = container_of!(map, bpf_array, map);
// arr_idx = (value - array->value) / array->elem_size;
    return arr_idx;
    }
    return value - round_up(map.key_size, 8);
    }
    enum bpf_async_type {
    BPF_ASYNC_TYPE_TIMER = 0,
    BPF_ASYNC_TYPE_WQ,
    };
    enum bpf_async_op {
    BPF_ASYNC_START,
    BPF_ASYNC_CANCEL
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_async_cmd {
    pub node: llist_node,
    pub nsec: u64,
    pub mode: u32,
    pub op: bpf_async_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_async_cb {
    pub map: *mut bpf_map,
    pub prog: *mut bpf_prog,
    pub callback_fn: *mut c_void ,
    pub value: *mut c_void,
    pub rcu: rcu_head,
    pub flags: u64,
    pub worker: irq_work,
    pub refcnt: refcount_t,
    pub type: bpf_async_type,
    pub async_cmds: llist_head,
}

// BPF map elements can contain 'struct bpf_timer'.
// Such map owns all of its BPF timers.
// 'struct bpf_timer' is allocated as part of map element allocation
// and it's zero initialized.
// That space is used to keep 'struct bpf_async_kern'.
// bpf_timer_init() allocates 'struct bpf_hrtimer', inits hrtimer, and
// remembers 'struct bpf_map *' pointer it's part of.
// bpf_timer_set_callback() increments prog refcnt and assign bpf callback_fn.
// bpf_timer_start() arms the timer.
// If user space reference to a map goes to zero at this point
// ops->map_release_uref callback is responsible for cancelling the timers,
// freeing their memory, and decrementing prog's refcnts.
// bpf_timer_cancel() cancels the timer and decrements prog's refcnt.
// Inner maps can contain bpf timers as well. ops->map_release_uref is
// freeing the timers when inner map is replaced or deleted by user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_hrtimer {
    pub cb: bpf_async_cb,
    pub timer: hrtimer,
    pub cancelling: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_work {
    pub cb: bpf_async_cb,
    pub work: work_struct,
}

// the actual struct hidden inside uapi struct bpf_timer and bpf_wq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_async_kern {
    union {
    pub cb: *mut bpf_async_cb,
    pub timer: *mut bpf_hrtimer,
    pub work: *mut bpf_work,
}

    } __attribute__((aligned(8)));
pub static mut struct bpf_hrtimer *: usize = 0;
// forward_decl: bpf_async_refcount_put;
#[no_mangle]
unsafe extern "C" fn bpf_timer_cb(hrtimer: *mut hrtimer) -> enum hrtimer_restart {
    let mut t = container_of!(hrtimer, bpf_hrtimer, timer);
    let mut map = t.cb.map;
    let mut value = t.cb.value;
    let mut callback_fn;
pub static mut key: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    BTF_TYPE_EMIT(bpf_timer);
    callback_fn = rcu_dereference_check(t.cb.callback_fn, rcu_read_lock_bh_held());
    if (!callback_fn) {
// goto;
    }
// bpf_timer_cb() runs in hrtimer_run_softirq. It doesn't migrate and
// cannot be preempted by another bpf_timer_cb() on the same cpu.
// Remember the timer this callback is servicing to prevent
// deadlock if callback_fn() calls bpf_timer_cancel() or
// bpf_map_delete_elem() on the same timer.
//
    this_cpu_write(hrtimer_running, t);
    key = map_key_from_value(map, value, &idx);
    callback_fn((u64)(long)map, (u64)(long)key, (u64)(long)value, 0, 0);
// The verifier checked that return value is zero.
    this_cpu_write(hrtimer_running, core::ptr::null_mut());
// label;
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn bpf_wq_work(work: *mut work_struct) {
    let mut w = container_of!(work, bpf_work, work);
    let mut cb = &w.cb;
    let mut map = cb.map;
    let mut callback_fn;
    let mut value = cb.value;
pub static mut key: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    BTF_TYPE_EMIT(bpf_wq);
    callback_fn = READ_ONCE(cb.callback_fn);
    if (!callback_fn) {
    return;
    }
    key = map_key_from_value(map, value, &idx);
    rcu_read_lock_trace();
    migrate_disable();
    callback_fn((u64)(long)map, (u64)(long)key, (u64)(long)value, 0, 0);
    migrate_enable();
    rcu_read_unlock_trace();
    }
#[no_mangle]
unsafe extern "C" fn bpf_async_cb_rcu_free(rcu: *mut rcu_head) {
    let mut cb = container_of!(rcu, bpf_async_cb, rcu);
//
// Drop the last reference to prog only after RCU GP, as set_callback()
// may race with cancel_and_free()
//
    if (cb.prog) {
    bpf_prog_put(cb.prog);
    }
    kfree_nolock(cb);
    }
// Callback from call_rcu_tasks_trace, chains to call_rcu for final free
#[no_mangle]
unsafe extern "C" fn bpf_async_cb_rcu_tasks_trace_free(rcu: *mut rcu_head) {
    let mut cb = container_of!(rcu, bpf_async_cb, rcu);
    let mut t = container_of!(cb, bpf_hrtimer, cb);
    let mut w = container_of!(cb, bpf_work, cb);
pub static mut retry: bool = false;
//
// bpf_async_cancel_and_free() tried to cancel timer/wq, but it
// could have raced with timer/wq_start. Now refcnt is zero and
// srcu/rcu GP completed. Cancel timer/wq again.
//
    match (cb.type) {
    BPF_ASYNC_TYPE_TIMER => {
    if (hrtimer_try_to_cancel(&t.timer) < 0) {
    retry = true;
    }
    // break;
    }
    BPF_ASYNC_TYPE_WQ => {
    if (!cancel_work(&w.work) && work_busy(&w.work)) {
    retry = true;
    }
    // break;
    }
    }
    if (retry) {
//
// hrtimer or wq callback may still be running. It must be
// in rcu_tasks_trace or rcu CS, so wait for GP again.
// It won't retry forever, since refcnt zero prevents all
// operations on timer/wq.
//
    call_rcu_tasks_trace(&cb.rcu, bpf_async_cb_rcu_tasks_trace_free);
    return;
    }
// RCU Tasks Trace grace period implies RCU grace period.
    bpf_async_cb_rcu_free(rcu);
    }
#[no_mangle]
unsafe extern "C" fn worker_for_call_rcu(work: *mut irq_work) {
    let mut cb = container_of!(work, bpf_async_cb, worker);
    call_rcu_tasks_trace(&cb.rcu, bpf_async_cb_rcu_tasks_trace_free);
    }
#[no_mangle]
unsafe extern "C" fn bpf_async_refcount_put(cb: *mut bpf_async_cb) {
    if (!refcount_dec_and_test(&cb.refcnt)) {
    return;
    }
    if (irqs_disabled()) {
    cb.worker = IRQ_WORK_INIT(worker_for_call_rcu);
    irq_work_queue(&cb.worker);
    } else {
    call_rcu_tasks_trace(&cb.rcu, bpf_async_cb_rcu_tasks_trace_free);
    }
    }
// forward_decl: bpf_async_cancel_and_free;
// forward_decl: bpf_async_irq_worker;
#[no_mangle]
pub unsafe extern "C" fn __bpf_async_init(async: *mut bpf_async_kern, map: *mut bpf_map, flags: u64, type: bpf_async_type) -> c_int {
    let mut cb = core::ptr::null_mut();
    let mut old_cb = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut w: *mut c_void = core::ptr::null_mut();
    let mut clockid;
    let mut size = 0;
    match (type) {
    BPF_ASYNC_TYPE_TIMER => {
    size = sizeof!(bpf_hrtimer);
    // break;
    }
    BPF_ASYNC_TYPE_WQ => {
    size = sizeof!(bpf_work);
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    old_cb = READ_ONCE(async.cb);
    if (old_cb) {
    return -EBUSY;
    }
    cb = bpf_map_kmalloc_nolock(map, size, 0, map.numa_node);
    if (!cb) {
    return -ENOMEM;
    }
    match (type) {
    BPF_ASYNC_TYPE_TIMER => {
    clockid = flags & (MAX_CLOCKS - 1);
    t = cb;
    atomic_set(&t.cancelling, 0);
    hrtimer_setup(&t.timer, bpf_timer_cb, clockid, HRTIMER_MODE_REL_SOFT);
    cb.value = async - map.record.timer_off;
    // break;
    }
    BPF_ASYNC_TYPE_WQ => {
    w = cb;
    INIT_WORK(&w.work, bpf_wq_work);
    cb.value = async - map.record.wq_off;
    // break;
    }
    }
    cb.map = map;
    cb.prog = core::ptr::null_mut();
    cb.flags = flags;
    cb.worker = IRQ_WORK_INIT(bpf_async_irq_worker);
    init_llist_head(&cb.async_cmds);
    refcount_set(&cb.refcnt, 1); /* map's reference */
    cb.type = type;
    rcu_assign_pointer(cb.callback_fn, core::ptr::null_mut());
    old_cb = cmpxchg(&async.cb, core::ptr::null_mut(), cb);
    if (old_cb) {
// Lost the race to initialize this bpf_async_kern, drop the allocated object
    kfree_nolock(cb);
    return -EBUSY;
    }
// Guarantee the order between async->cb and map->usercnt. So
// when there are concurrent uref release and bpf timer init, either
// bpf_timer_cancel_and_free() called by uref release reads a no-NULL
// timer or atomic64_read() below returns a zero usercnt.
//
    smp_mb();
    if (!atomic64_read(&map.usercnt)) {
// maps with timers must be either held by user space
// or pinned in bpffs.
//
    bpf_async_cancel_and_free(async);
    return -EPERM;
    }
    return 0;
    }
    BPF_CALL_3(bpf_timer_init, bpf_async_kern *, timer, bpf_map *, map,
    u64, flags)
    {
pub static mut clockid: clock_t = 0;
    BUILD_BUG_ON!(MAX_CLOCKS != 16);
    BUILD_BUG_ON!(sizeof!(bpf_async_kern) > sizeof!(bpf_timer));
    BUILD_BUG_ON!(__alignof__(bpf_async_kern) != __alignof__(bpf_timer));
    if (flags >= MAX_CLOCKS ||
// similar to timerfd except _ALARM variants are not supported
    (clockid != CLOCK_MONOTONIC &&
    clockid != CLOCK_REALTIME &&
    clockid != CLOCK_BOOTTIME)) {
    return -EINVAL;
    }
    return __bpf_async_init(timer, map, flags, BPF_ASYNC_TYPE_TIMER);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_async_update_prog_callback(cb: *mut bpf_async_cb, prog: *mut bpf_prog, callback_fn: *mut c_void) -> c_int {
pub static mut prev: *mut c_void = core::ptr::null_mut();
// Acquire a guard reference on prog to prevent it from being freed during the loop
    if (prog) {
    prog = bpf_prog_inc_not_zero(prog);
    if (IS_ERR(prog)) {
    return PTR_ERR(prog);
    }
    }
    do {
    if (prog) {
    prog = bpf_prog_inc_not_zero(prog);
    }
    prev = xchg(&cb.prog, prog);
    rcu_assign_pointer(cb.callback_fn, callback_fn);
//
// Release previous prog, make sure that if other CPU is contending,
// to set bpf_prog, references are not leaked as each iteration acquires and
// releases one reference.
//
    if (prev) {
    bpf_prog_put(prev);
    }
    } while (READ_ONCE(cb.prog) != prog ||
    READ_ONCE(cb.callback_fn) != callback_fn);
    if (prog) {
    bpf_prog_put(prog);
    }
    return 0;
    }
pub static mut struct bpf_async_cb *: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_async_schedule_op(cb: *mut bpf_async_cb, op: bpf_async_op, nsec: u64, timer_mode: u32) -> c_int {
//
// Do not schedule another operation on this cpu if it's in irq_work
// callback that is processing async_cmds queue. Otherwise the following
// loop is possible:
// bpf_timer_start() -> bpf_async_schedule_op() -> irq_work_queue().
// irqrestore -> bpf_async_irq_worker() -> tracepoint -> bpf_timer_start().
//
    if (this_cpu_read(async_cb_running) == cb) {
    bpf_async_refcount_put(cb);
    return -EDEADLK;
    }
    let mut cmd = kmalloc_nolock(sizeof!(*cmd), 0, NUMA_NO_NODE);
    if (!cmd) {
    bpf_async_refcount_put(cb);
    return -ENOMEM;
    }
    init_llist_node(&cmd.node);
    cmd.nsec = nsec;
    cmd.mode = timer_mode;
    cmd.op = op;
    if (llist_add(&cmd.node, &cb.async_cmds)) {
    irq_work_queue(&cb.worker);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_async_set_callback(async: *mut bpf_async_kern, callback_fn: *mut c_void, prog: *mut bpf_prog) -> c_int {
pub static mut cb: *mut c_void = core::ptr::null_mut();
    cb = READ_ONCE(async.cb);
    if (!cb) {
    return -EINVAL;
    }
    return bpf_async_update_prog_callback(cb, prog, callback_fn);
    }
    BPF_CALL_3(bpf_timer_set_callback, bpf_async_kern *, timer, void *, callback_fn, bpf_prog_aux *, aux)
    {
    return __bpf_async_set_callback(timer, callback_fn, aux.prog);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
unsafe extern "C" fn defer_timer_wq_op() -> bool {
    return in_hardirq() || irqs_disabled();
    }
    BPF_CALL_3(bpf_timer_start, bpf_async_kern *, async, u64, nsecs, u64, flags)
    {
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut mode = 0;
    if (flags & ~(BPF_F_TIMER_ABS | BPF_F_TIMER_CPU_PIN)) {
    return -EINVAL;
    }
    t = READ_ONCE(async.timer);
    if (!t || !READ_ONCE(t.cb.prog)) {
    return -EINVAL;
    }
    if (flags & BPF_F_TIMER_ABS) {
    mode = HRTIMER_MODE_ABS_SOFT;
    }
    else {
    mode = HRTIMER_MODE_REL_SOFT;
    }
    if (flags & BPF_F_TIMER_CPU_PIN) {
    mode |= HRTIMER_MODE_PINNED;
    }
//
// bpf_async_cancel_and_free() could have dropped refcnt to zero. In
// such case BPF progs are not allowed to arm the timer to prevent UAF.
//
    if (!refcount_inc_not_zero(&t.cb.refcnt)) {
    return -ENOENT;
    }
    if (!defer_timer_wq_op()) {
    hrtimer_start(&t.timer, ns_to_ktime(nsecs), mode);
    bpf_async_refcount_put(&t.cb);
    return 0;
    } else {
    return bpf_async_schedule_op(&t.cb, BPF_ASYNC_START, nsecs, mode);
    }
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_1(bpf_timer_cancel, bpf_async_kern *, async)
    {
    let mut t = core::ptr::null_mut();
    let mut cur_t = core::ptr::null_mut();
pub static mut inc: bool = false;
pub static mut ret: c_int = 0;
    if (defer_timer_wq_op()) {
    return -EOPNOTSUPP;
    }
    t = READ_ONCE(async.timer);
    if (!t) {
    return -EINVAL;
    }
    cur_t = this_cpu_read(hrtimer_running);
    if (cur_t == t) {
// If bpf callback_fn is trying to bpf_timer_cancel()
// its own timer the hrtimer_cancel() will deadlock
// since it waits for callback_fn to finish.
//
    return -EDEADLK;
    }
// Only account in-flight cancellations when invoked from a timer
// callback, since we want to avoid waiting only if other _callbacks_
// are waiting on us, to avoid introducing lockups. Non-callback paths
// are ok, since nobody would synchronously wait for their completion.
//
    if (!cur_t) {
// goto;
    }
    atomic_inc(&t.cancelling);
// Need full barrier after relaxed atomic_inc
    smp_mb__after_atomic();
    inc = true;
    if (atomic_read(&cur_t.cancelling)) {
// We're cancelling timer t, while some other timer callback is
// attempting to cancel us. In such a case, it might be possible
// that timer t belongs to the other callback, or some other
// callback waiting upon it (creating transitive dependencies
// upon us), and we will enter a deadlock if we continue
// cancelling and waiting for it synchronously, since it might
// do the same. Bail!
//
    atomic_dec(&t.cancelling);
    return -EDEADLK;
    }
// label;
    bpf_async_update_prog_callback(&t.cb, core::ptr::null_mut(), core::ptr::null_mut());
// Cancel the timer and wait for associated callback to finish
// if it was running.
//
    ret = hrtimer_cancel(&t.timer);
    if (inc) {
    atomic_dec(&t.cancelling);
    }
    return ret;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn bpf_async_process_op(cb: *mut bpf_async_cb, op: u32, timer_nsec: u64, timer_mode: u32) {
    match (cb.type) {
    BPF_ASYNC_TYPE_TIMER => {
    let mut t = container_of!(cb, bpf_hrtimer, cb);
    match (op) {
    BPF_ASYNC_START => {
    hrtimer_start(&t.timer, ns_to_ktime(timer_nsec), timer_mode);
    // break;
    }
    BPF_ASYNC_CANCEL => {
    hrtimer_try_to_cancel(&t.timer);
    // break;
    }
    }
    break;
    }
    case BPF_ASYNC_TYPE_WQ: {
    let mut w = container_of!(cb, bpf_work, cb);
    match (op) {
    BPF_ASYNC_START => {
    schedule_work(&w.work);
    // break;
    }
    BPF_ASYNC_CANCEL => {
    cancel_work(&w.work);
    // break;
    }
    }
    break;
    }
    }
    bpf_async_refcount_put(cb);
    }
#[no_mangle]
unsafe extern "C" fn bpf_async_irq_worker(work: *mut irq_work) {
    let mut cb = container_of!(work, bpf_async_cb, worker);
    let mut pos = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
    list = llist_del_all(&cb.async_cmds);
    if (!list) {
    return;
    }
    list = llist_reverse_order(list);
    this_cpu_write(async_cb_running, cb);
    llist_for_each_safe(pos, n, list) {
pub static mut cmd: *mut c_void = core::ptr::null_mut();
    cmd = container_of!(pos, bpf_async_cmd, node);
    bpf_async_process_op(cb, cmd.op, cmd.nsec, cmd.mode);
    kfree_nolock(cmd);
    }
    this_cpu_write(async_cb_running, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn bpf_async_cancel_and_free(async: *mut bpf_async_kern) {
pub static mut cb: *mut c_void = core::ptr::null_mut();
    if (!READ_ONCE(async.cb)) {
    return;
    }
    cb = xchg(&async.cb, core::ptr::null_mut());
    if (!cb) {
    return;
    }
    bpf_async_update_prog_callback(cb, core::ptr::null_mut(), core::ptr::null_mut());
//
// No refcount_inc_not_zero(&cb->refcnt) here. Dropping the last
// refcnt. Either synchronously or asynchronously in irq_work.
//
    if (!defer_timer_wq_op()) {
    bpf_async_process_op(cb, BPF_ASYNC_CANCEL, 0, 0);
    } else {
    (void)bpf_async_schedule_op(cb, BPF_ASYNC_CANCEL, 0, 0);
//
// bpf_async_schedule_op() either enqueues allocated cmd into llist
// or fails with ENOMEM and drop the last refcnt.
// This is unlikely, but safe, since bpf_async_cb_rcu_tasks_trace_free()
// callback will do additional timer/wq_cancel due to races anyway.
//
    }
    }
//
// This function is called by map_delete/update_elem for individual element and
// by ops->map_release_uref when the user space reference to a map reaches zero.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_timer_cancel_and_free(val: *mut c_void) {
    bpf_async_cancel_and_free(val);
    }
//
// This function is called by map_delete/update_elem for individual element and
// by ops->map_release_uref when the user space reference to a map reaches zero.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_wq_cancel_and_free(val: *mut c_void) {
    bpf_async_cancel_and_free(val);
    }
    BPF_CALL_2(bpf_kptr_xchg, void *, dst, void *, ptr)
    {
    let mut kptr = dst;
// This helper may be inlined by verifier.
    return xchg(kptr, (unsigned long)ptr);
    }
// Unlike other PTR_TO_BTF_ID helpers the btf_id in bpf_kptr_xchg()
// helper is determined dynamically by the verifier. Use BPF_PTR_POISON to
// denote type that verifier will determine.
//
pub static mut bpf_func_proto: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_dynptr_file_impl {
    pub freader: freader,
// 64 bit offset and size overriding 32 bit ones in bpf_dynptr_kern
    pub offset: u64,
    pub size: u64,
}

// Since the upper 8 bits of dynptr->size is reserved, the
// maximum supported size is 2^24 - 1.
//

pub const DYNPTR_TYPE_SHIFT: c_int = 28;
pub const DYNPTR_SIZE_MASK: c_uint = 0xFFFFFF;

#[no_mangle]
pub unsafe extern "C" fn __bpf_dynptr_is_rdonly(ptr: *const bpf_dynptr_kern) -> bool {
    return ptr.size & DYNPTR_RDONLY_BIT;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_set_rdonly(ptr: *mut bpf_dynptr_kern) {
    ptr.size |= DYNPTR_RDONLY_BIT;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dynptr_set_type(ptr: *mut bpf_dynptr_kern, type: bpf_dynptr_type) {
    ptr.size |= type << DYNPTR_TYPE_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dynptr_get_type(ptr: *const bpf_dynptr_kern) -> enum bpf_dynptr_type {
    return (ptr.size & ~(DYNPTR_RDONLY_BIT)) >> DYNPTR_TYPE_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_dynptr_size(ptr: *const bpf_dynptr_kern) -> u64 {
    if (bpf_dynptr_get_type(ptr) == BPF_DYNPTR_TYPE_FILE) {
    let mut df = ptr.data;
    return df.size;
    }
    return ptr.size & DYNPTR_SIZE_MASK;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dynptr_advance_offset(ptr: *mut bpf_dynptr_kern, off: u64) {
    if (bpf_dynptr_get_type(ptr) == BPF_DYNPTR_TYPE_FILE) {
    let mut df = ptr.data;
    df.offset += off;
    return;
    }
    ptr.offset += off;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dynptr_set_size(ptr: *mut bpf_dynptr_kern, new_size: u64) {
pub static mut metadata: u32 = 0;
    if (bpf_dynptr_get_type(ptr) == BPF_DYNPTR_TYPE_FILE) {
    let mut df = ptr.data;
    df.size = new_size;
    return;
    }
    ptr.size = (u32)new_size | metadata;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_check_size(size: u64) -> c_int {
    return size > DYNPTR_MAX_SIZE ? -E2BIG : 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_file_fetch_bytes(df: *mut bpf_dynptr_file_impl, offset: u64, buf: *mut c_void, len: u64) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (!buf) {
    return -EINVAL;
    }
    df.freader.buf = buf;
    df.freader.buf_sz = len;
    ptr = freader_fetch(&df.freader, offset + df.offset, len);
    if (!ptr) {
    return df.freader.err;
    }
    if (ptr != buf) /* Force copying into the buffer */ {
    memcpy(buf, ptr, len);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_init(ptr: *mut bpf_dynptr_kern, data: *mut c_void, type: bpf_dynptr_type, offset: u32, size: u32) {
    ptr.data = data;
    ptr.offset = offset;
    ptr.size = size;
    bpf_dynptr_set_type(ptr, type);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_set_null(ptr: *mut bpf_dynptr_kern) {
    memset(ptr, 0, sizeof!(*ptr));
    }
    BPF_CALL_4(bpf_dynptr_from_mem, void *, data, u64, size, u64, flags, bpf_dynptr_kern *, ptr)
    {
    let mut err = 0;
    BTF_TYPE_EMIT(bpf_dynptr);
    err = bpf_dynptr_check_size(size);
    if (err) {
// goto;
    }
// flags is currently unsupported
    if (flags) {
    err = -EINVAL;
// goto;
    }
    bpf_dynptr_init(ptr, data, BPF_DYNPTR_TYPE_LOCAL, 0, size);
    return 0;
// label;
    bpf_dynptr_set_null(ptr);
    return err;
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_dynptr_read(dst: *mut c_void, len: u64, src: *mut bpf_dynptr_kern, offset: u64, flags: u64) -> c_int {
    enum bpf_dynptr_type type;
    let mut err = 0;
    if (!src.data || flags) {
    return -EINVAL;
    }
    err = bpf_dynptr_check_off_len(src, offset, len);
    if (err) {
    return err;
    }
    type = bpf_dynptr_get_type(src);
    match (type) {
    BPF_DYNPTR_TYPE_LOCAL => {
    }
    BPF_DYNPTR_TYPE_RINGBUF => {
// Source and destination may possibly overlap, hence use memmove to
// copy the data. E.g. bpf_dynptr_from_mem may create two dynptr
// pointing to overlapping PTR_TO_MAP_VALUE regions.
//
    memmove(dst, src.data + src.offset + offset, len);
    return 0;
    }
    BPF_DYNPTR_TYPE_SKB => {
    return __bpf_skb_load_bytes(src.data, src.offset + offset, dst, len);
    }
    BPF_DYNPTR_TYPE_XDP => {
    return __bpf_xdp_load_bytes(src.data, src.offset + offset, dst, len);
    }
    BPF_DYNPTR_TYPE_SKB_META => {
    memmove(dst, bpf_skb_meta_pointer(src.data, src.offset + offset), len);
    return 0;
    }
    BPF_DYNPTR_TYPE_FILE => {
    return bpf_file_fetch_bytes(src.data, offset, dst, len);
    }
    _ => {
    WARN_ONCE(true, "bpf_dynptr_read: unknown dynptr type %d\n", type);
    return -EFAULT;
    }
    }
    }
    BPF_CALL_5(bpf_dynptr_read, void *, dst, u64, len, const struct bpf_dynptr_kern *, src,
    u64, offset, u64, flags)
    {
    return __bpf_dynptr_read(dst, len, src, offset, flags);
    }
pub static mut bpf_func_proto: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __bpf_dynptr_write(dst: *mut bpf_dynptr_kern, offset: u64, src: *mut c_void, len: u64, flags: u64) -> c_int {
    enum bpf_dynptr_type type;
    let mut err = 0;
    if (!dst.data || __bpf_dynptr_is_rdonly(dst)) {
    return -EINVAL;
    }
    err = bpf_dynptr_check_off_len(dst, offset, len);
    if (err) {
    return err;
    }
    type = bpf_dynptr_get_type(dst);
    match (type) {
    BPF_DYNPTR_TYPE_LOCAL => {
    }
    BPF_DYNPTR_TYPE_RINGBUF => {
    if (flags) {
    return -EINVAL;
    }
// Source and destination may possibly overlap, hence use memmove to
// copy the data. E.g. bpf_dynptr_from_mem may create two dynptr
// pointing to overlapping PTR_TO_MAP_VALUE regions.
//
    memmove(dst.data + dst.offset + offset, src, len);
    return 0;
    }
    BPF_DYNPTR_TYPE_SKB => {
    return __bpf_skb_store_bytes(dst.data, dst.offset + offset, src, len,
    flags);
    }
    BPF_DYNPTR_TYPE_XDP => {
    if (flags) {
    return -EINVAL;
    }
    return __bpf_xdp_store_bytes(dst.data, dst.offset + offset, src, len);
    }
    BPF_DYNPTR_TYPE_SKB_META => {
    return __bpf_skb_meta_store_bytes(dst.data, dst.offset + offset, src,
    len, flags);
    }
    _ => {
    WARN_ONCE(true, "bpf_dynptr_write: unknown dynptr type %d\n", type);
    return -EFAULT;
    }
    }
    }
    BPF_CALL_5(bpf_dynptr_write, const struct bpf_dynptr_kern *, dst, u64, offset, void *, src,
    u64, len, u64, flags)
    {
    return __bpf_dynptr_write(dst, offset, src, len, flags);
    }
pub static mut bpf_func_proto: usize = 0;
    BPF_CALL_3(bpf_dynptr_data, const struct bpf_dynptr_kern *, ptr, u64, offset, u64, len)
    {
    enum bpf_dynptr_type type;
    let mut err = 0;
    if (!ptr.data) {
    return 0;
    }
    err = bpf_dynptr_check_off_len(ptr, offset, len);
    if (err) {
    return 0;
    }
    if (__bpf_dynptr_is_rdonly(ptr)) {
    return 0;
    }
    type = bpf_dynptr_get_type(ptr);
    match (type) {
    BPF_DYNPTR_TYPE_LOCAL => {
    }
    BPF_DYNPTR_TYPE_RINGBUF => {
    return (unsigned long)(ptr.data + ptr.offset + offset);
    }
    BPF_DYNPTR_TYPE_SKB => {
    }
    BPF_DYNPTR_TYPE_XDP => {
    }
    BPF_DYNPTR_TYPE_SKB_META => {
// skb and xdp dynptrs should use bpf_dynptr_slice / bpf_dynptr_slice_rdwr
    return 0;
    }
    _ => {
    WARN_ONCE(true, "bpf_dynptr_data: unknown dynptr type %d\n", type);
    return 0;
    }
    }
    }
pub static mut bpf_func_proto: usize = 0;
    const struct bpf_func_proto bpf_get_current_task_proto __weak;
    const struct bpf_func_proto bpf_get_current_task_btf_proto __weak;
    const struct bpf_func_proto bpf_probe_read_user_proto __weak;
    const struct bpf_func_proto bpf_probe_read_user_str_proto __weak;
    const struct bpf_func_proto bpf_probe_read_kernel_proto __weak;
    const struct bpf_func_proto bpf_probe_read_kernel_str_proto __weak;
    const struct bpf_func_proto bpf_task_pt_regs_proto __weak;
    const struct bpf_func_proto bpf_perf_event_read_proto __weak;
    const struct bpf_func_proto bpf_send_signal_proto __weak;
    const struct bpf_func_proto bpf_send_signal_thread_proto __weak;
    const struct bpf_func_proto bpf_get_task_stack_sleepable_proto __weak;
    const struct bpf_func_proto bpf_get_task_stack_proto __weak;
    const struct bpf_func_proto bpf_get_branch_snapshot_proto __weak;
    const struct bpf_func_proto *
    bpf_base_func_proto(enum bpf_func_id func_id, const struct bpf_prog *prog)
    {
    match (func_id) {
    BPF_FUNC_map_lookup_elem => {
    return &bpf_map_lookup_elem_proto;
    }
    BPF_FUNC_map_update_elem => {
    return &bpf_map_update_elem_proto;
    }
    BPF_FUNC_map_delete_elem => {
    return &bpf_map_delete_elem_proto;
    }
    BPF_FUNC_map_push_elem => {
    return &bpf_map_push_elem_proto;
    }
    BPF_FUNC_map_pop_elem => {
    return &bpf_map_pop_elem_proto;
    }
    BPF_FUNC_map_peek_elem => {
    return &bpf_map_peek_elem_proto;
    }
    BPF_FUNC_map_lookup_percpu_elem => {
    return &bpf_map_lookup_percpu_elem_proto;
    }
    BPF_FUNC_get_prandom_u32 => {
    return &bpf_get_prandom_u32_proto;
    }
    BPF_FUNC_get_smp_processor_id => {
    return &bpf_get_raw_smp_processor_id_proto;
    }
    BPF_FUNC_get_numa_node_id => {
    return &bpf_get_numa_node_id_proto;
    }
    BPF_FUNC_tail_call => {
    return &bpf_tail_call_proto;
    }
    BPF_FUNC_ktime_get_ns => {
    return &bpf_ktime_get_ns_proto;
    }
    BPF_FUNC_ktime_get_boot_ns => {
    return &bpf_ktime_get_boot_ns_proto;
    }
    BPF_FUNC_ktime_get_tai_ns => {
    return &bpf_ktime_get_tai_ns_proto;
    }
    BPF_FUNC_ringbuf_output => {
    return &bpf_ringbuf_output_proto;
    }
    BPF_FUNC_ringbuf_reserve => {
    return &bpf_ringbuf_reserve_proto;
    }
    BPF_FUNC_ringbuf_submit => {
    return &bpf_ringbuf_submit_proto;
    }
    BPF_FUNC_ringbuf_discard => {
    return &bpf_ringbuf_discard_proto;
    }
    BPF_FUNC_ringbuf_query => {
    return &bpf_ringbuf_query_proto;
    }
    BPF_FUNC_strncmp => {
    return &bpf_strncmp_proto;
    }
    BPF_FUNC_strtol => {
    return &bpf_strtol_proto;
    }
    BPF_FUNC_strtoul => {
    return &bpf_strtoul_proto;
    }
    BPF_FUNC_get_current_pid_tgid => {
    return &bpf_get_current_pid_tgid_proto;
    }
    BPF_FUNC_get_ns_current_pid_tgid => {
    return &bpf_get_ns_current_pid_tgid_proto;
    }
    BPF_FUNC_get_current_uid_gid => {
    return &bpf_get_current_uid_gid_proto;
    }
    _ => {
    // break;
    }
    }
    if (!bpf_token_capable(prog.aux.token, CAP_BPF)) {
    return core::ptr::null_mut();
    }
    match (func_id) {
    BPF_FUNC_spin_lock => {
    return &bpf_spin_lock_proto;
    }
    BPF_FUNC_spin_unlock => {
    return &bpf_spin_unlock_proto;
    }
    BPF_FUNC_jiffies64 => {
    return &bpf_jiffies64_proto;
    }
    BPF_FUNC_per_cpu_ptr => {
    return &bpf_per_cpu_ptr_proto;
    }
    BPF_FUNC_this_cpu_ptr => {
    return &bpf_this_cpu_ptr_proto;
    }
    BPF_FUNC_timer_init => {
    return &bpf_timer_init_proto;
    }
    BPF_FUNC_timer_set_callback => {
    return &bpf_timer_set_callback_proto;
    }
    BPF_FUNC_timer_start => {
    return &bpf_timer_start_proto;
    }
    BPF_FUNC_timer_cancel => {
    return &bpf_timer_cancel_proto;
    }
    BPF_FUNC_kptr_xchg => {
    return &bpf_kptr_xchg_proto;
    }
    BPF_FUNC_for_each_map_elem => {
    return &bpf_for_each_map_elem_proto;
    }
    BPF_FUNC_loop => {
    return &bpf_loop_proto;
    }
    BPF_FUNC_user_ringbuf_drain => {
    return &bpf_user_ringbuf_drain_proto;
    }
    BPF_FUNC_ringbuf_reserve_dynptr => {
    return &bpf_ringbuf_reserve_dynptr_proto;
    }
    BPF_FUNC_ringbuf_submit_dynptr => {
    return &bpf_ringbuf_submit_dynptr_proto;
    }
    BPF_FUNC_ringbuf_discard_dynptr => {
    return &bpf_ringbuf_discard_dynptr_proto;
    }
    BPF_FUNC_dynptr_from_mem => {
    return &bpf_dynptr_from_mem_proto;
    }
    BPF_FUNC_dynptr_read => {
    return &bpf_dynptr_read_proto;
    }
    BPF_FUNC_dynptr_write => {
    return &bpf_dynptr_write_proto;
    }
    BPF_FUNC_dynptr_data => {
    return &bpf_dynptr_data_proto;

    }
    BPF_FUNC_cgrp_storage_get => {
    return &bpf_cgrp_storage_get_proto;
    }
    BPF_FUNC_cgrp_storage_delete => {
    return &bpf_cgrp_storage_delete_proto;
    }
    BPF_FUNC_get_current_cgroup_id => {
    return &bpf_get_current_cgroup_id_proto;
    }
    BPF_FUNC_get_current_ancestor_cgroup_id => {
    return &bpf_get_current_ancestor_cgroup_id_proto;
    }
    BPF_FUNC_current_task_under_cgroup => {
    return &bpf_current_task_under_cgroup_proto;

    }
    BPF_FUNC_get_cgroup_classid => {
    return &bpf_get_cgroup_classid_curr_proto;

    }
    BPF_FUNC_task_storage_get => {
    return &bpf_task_storage_get_proto;
    }
    BPF_FUNC_task_storage_delete => {
    return &bpf_task_storage_delete_proto;
    }
    _ => {
    // break;
    }
    }
    if (!bpf_token_capable(prog.aux.token, CAP_PERFMON)) {
    return core::ptr::null_mut();
    }
    match (func_id) {
    BPF_FUNC_trace_printk => {
    return bpf_get_trace_printk_proto();
    }
    BPF_FUNC_get_current_task => {
    return &bpf_get_current_task_proto;
    }
    BPF_FUNC_get_current_task_btf => {
    return &bpf_get_current_task_btf_proto;
    }
    BPF_FUNC_get_current_comm => {
    return &bpf_get_current_comm_proto;
    }
    BPF_FUNC_probe_read_user => {
    return &bpf_probe_read_user_proto;
    }
    BPF_FUNC_probe_read_kernel => {
    return security_locked_down(LOCKDOWN_BPF_READ_KERNEL) < 0 ?
    core::ptr::null_mut() : &bpf_probe_read_kernel_proto;
    }
    BPF_FUNC_probe_read_user_str => {
    return &bpf_probe_read_user_str_proto;
    }
    BPF_FUNC_probe_read_kernel_str => {
    return security_locked_down(LOCKDOWN_BPF_READ_KERNEL) < 0 ?
    core::ptr::null_mut() : &bpf_probe_read_kernel_str_proto;
    }
    BPF_FUNC_copy_from_user => {
    return &bpf_copy_from_user_proto;
    }
    BPF_FUNC_copy_from_user_task => {
    return &bpf_copy_from_user_task_proto;
    }
    BPF_FUNC_snprintf_btf => {
    return &bpf_snprintf_btf_proto;
    }
    BPF_FUNC_snprintf => {
    return &bpf_snprintf_proto;
    }
    BPF_FUNC_task_pt_regs => {
    return &bpf_task_pt_regs_proto;
    }
    BPF_FUNC_trace_vprintk => {
    return bpf_get_trace_vprintk_proto();
    }
    BPF_FUNC_perf_event_read_value => {
    return bpf_get_perf_event_read_value_proto();
    }
    BPF_FUNC_perf_event_read => {
    return &bpf_perf_event_read_proto;
    }
    BPF_FUNC_send_signal => {
    return &bpf_send_signal_proto;
    }
    BPF_FUNC_send_signal_thread => {
    return &bpf_send_signal_thread_proto;
    }
    BPF_FUNC_get_task_stack => {
    return prog.sleepable ? &bpf_get_task_stack_sleepable_proto
    : &bpf_get_task_stack_proto;
    }
    BPF_FUNC_get_branch_snapshot => {
    return &bpf_get_branch_snapshot_proto;
    }
    BPF_FUNC_find_vma => {
    return &bpf_find_vma_proto;
    }
    _ => {
    return core::ptr::null_mut();
    }
    }
    }
    EXPORT_SYMBOL_GPL(bpf_base_func_proto);
#[no_mangle]
pub unsafe extern "C" fn bpf_list_head_free(field: *mut btf_field, list_head: *mut c_void, spin_lock: *mut bpf_spin_lock) {
    let mut head = list_head, drain, *pos, *n;
    BUILD_BUG_ON!(sizeof!(list_head) > sizeof!(bpf_list_head));
    BUILD_BUG_ON!(__alignof__(list_head) > __alignof__(bpf_list_head));
    INIT_LIST_HEAD(&drain);
// Do the actual list draining outside the lock to not hold the lock for
// too long, and also prevent deadlocks if tracing programs end up
// executing on entry/exit of functions called inside the critical
// section, and end up doing map ops that call bpf_list_head_free for
// the same map value again.
//
    __bpf_spin_lock_irqsave(spin_lock);
    if (!head.next || list_empty(head)) {
// goto;
    }
    list_for_each_safe(pos, n, head) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = container_of!(pos, bpf_list_node_kern, list_head);
    WRITE_ONCE(node.owner, BPF_PTR_POISON);
    list_move_tail(pos, &drain);
    }
// label;
    INIT_LIST_HEAD(head);
    __bpf_spin_unlock_irqrestore(spin_lock);
    while (!list_empty(&drain)) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    pos = drain.next;
    node = container_of!(pos, bpf_list_node_kern, list_head);
    list_del_init(pos);
// Ensure __bpf_list_add() sees the node as unlinked.
    smp_store_release(&node.owner, core::ptr::null_mut());
// The contained type can also have resources, including a
// bpf_list_head which needs to be freed.
//
    __bpf_obj_drop_impl(pos - field.graph_root.node_offset,
    field.graph_root.value_rec, false);
    }
    }
// Like rbtree_postorder_for_each_entry_safe, but 'pos' and 'n' are
// 'rb_node *', so field name of rb_node within containing struct is not
// needed.
//
// Since bpf_rb_tree's node type has a corresponding struct btf_field with
// graph_root.node_offset, it's not necessary to know field name
// or type of node struct
//

    for (pos = rb_first_postorder(root); 
    pos && ({ n = rb_next_postorder(pos); 1; }); 
    pos = n) {
#[no_mangle]
pub unsafe extern "C" fn bpf_rb_root_free(field: *mut btf_field, rb_root: *mut c_void, spin_lock: *mut bpf_spin_lock) {
    }
    struct rb_root_cached orig_root, *root = rb_root;
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut obj: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(sizeof!(rb_root_cached) > sizeof!(bpf_rb_root));
    BUILD_BUG_ON!(__alignof__(rb_root_cached) > __alignof__(bpf_rb_root));
    __bpf_spin_lock_irqsave(spin_lock);
    orig_root = *root;
    bpf_rbtree_postorder_for_each_entry_safe(pos, n, &orig_root.rb_root) {
    node = rb_entry(pos, bpf_rb_node_kern, rb_node);
    WRITE_ONCE(node.owner, BPF_PTR_POISON);
    }
// root = RB_ROOT_CACHED;
    __bpf_spin_unlock_irqrestore(spin_lock);
    bpf_rbtree_postorder_for_each_entry_safe(pos, n, &orig_root.rb_root) {
    obj = pos;
    obj -= field.graph_root.node_offset;
    node = rb_entry(pos, bpf_rb_node_kern, rb_node);
    RB_CLEAR_NODE(pos);
// Ensure __bpf_rbtree_add() sees the node as unlinked.
    smp_store_release(&node.owner, core::ptr::null_mut());
    __bpf_obj_drop_impl(obj, field.graph_root.value_rec, false);
    }
    }
    __bpf_kfunc_start_defs();
//
// bpf_obj_new() - allocate an object described by program BTF
// @local_type_id__k: type ID in program BTF
// @meta: verifier-supplied struct metadata
//
// Allocate an object of the type identified by @local_type_id__k and
// initialize its special fields. BPF programs can use
// bpf_core_type_id_local() to provide @local_type_id__k. The verifier
// rewrites @meta; BPF programs do not set it.
//
// Return: Pointer to the allocated object, or %NULL on failure.
//
    __bpf_kfunc void *bpf_obj_new(u64 local_type_id__k, btf_struct_meta *meta)
    {
pub static mut size: u64 = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = bpf_mem_alloc(&bpf_global_ma, size);
    if (!p) {
    return core::ptr::null_mut();
    }
    if (meta) {
    bpf_obj_init(meta.record, p);
    }
    return p;
    }
    __bpf_kfunc void *bpf_obj_new_impl(u64 local_type_id__k, void *meta__ign)
    {
    return bpf_obj_new(local_type_id__k, meta__ign);
    }
//
// bpf_percpu_obj_new() - allocate a percpu object described by program BTF
// @local_type_id__k: type ID in program BTF
// @meta: verifier-supplied struct metadata
//
// Allocate a percpu object of the type identified by @local_type_id__k. BPF
// programs can use bpf_core_type_id_local() to provide @local_type_id__k.
// The verifier rewrites @meta; BPF programs do not set it.
//
// Return: Pointer to the allocated percpu object, or %NULL on failure.
//
    __bpf_kfunc void *bpf_percpu_obj_new(u64 local_type_id__k, btf_struct_meta *meta)
    {
pub static mut size: u64 = 0;
// The verifier has ensured that meta must be NULL
    return bpf_mem_alloc(&bpf_global_percpu_ma, size);
    }
    __bpf_kfunc void *bpf_percpu_obj_new_impl(u64 local_type_id__k, void *meta__ign)
    {
    return bpf_percpu_obj_new(local_type_id__k, meta__ign);
    }
// Must be called under migrate_disable(), as required by bpf_mem_free
#[no_mangle]
pub unsafe extern "C" fn __bpf_obj_drop_impl(p: *mut c_void, rec: *const btf_record, percpu: bool) {
pub static mut ma: *mut c_void = core::ptr::null_mut();
    if (rec && rec.refcount_off >= 0 &&
    !refcount_dec_and_test((p + rec.refcount_off))) {
// Object is refcounted and refcount_dec didn't result in 0
// refcount. Return without freeing the object
//
    return;
    }
    if (rec) {
    bpf_obj_free_fields(rec, p);
    }
    if (percpu) {
    ma = &bpf_global_percpu_ma;
    }
    else {
    ma = &bpf_global_ma;
    }
    bpf_mem_free_rcu(ma, p);
    }
//
// bpf_obj_drop() - drop a previously allocated object
// @p__alloc: object to free
// @meta: verifier-supplied struct metadata
//
// Destroy special fields in @p__alloc as needed and free the object. The
// verifier rewrites @meta; BPF programs do not set it.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_drop(p__alloc: *mut c_void, meta: *mut btf_struct_meta) -> __bpf_kfunc void {
    let mut p = p__alloc;
    __bpf_obj_drop_impl(p, meta ? meta.record : core::ptr::null_mut(), false);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_drop_impl(p__alloc: *mut c_void, meta__ign: *mut c_void) -> __bpf_kfunc void {
    return bpf_obj_drop(p__alloc, meta__ign);
    }
//
// bpf_percpu_obj_drop() - drop a previously allocated percpu object
// @p__alloc: percpu object to free
// @meta: verifier-supplied struct metadata
//
// Free @p__alloc. The verifier rewrites @meta; BPF programs do not set it.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_obj_drop(p__alloc: *mut c_void, meta: *mut btf_struct_meta) -> __bpf_kfunc void {
// The verifier has ensured that meta must be NULL
    bpf_mem_free_rcu(&bpf_global_percpu_ma, p__alloc);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_percpu_obj_drop_impl(p__alloc: *mut c_void, meta__ign: *mut c_void) -> __bpf_kfunc void {
    bpf_percpu_obj_drop(p__alloc, meta__ign);
    }
//
// bpf_refcount_acquire() - turn a local kptr into an owning reference
// @p__refcounted_kptr: non-owning local kptr
// @meta: verifier-supplied struct metadata
//
// Increment the refcount for @p__refcounted_kptr. The verifier rewrites
// @meta; BPF programs do not set it.
//
// Return: Owning reference to @p__refcounted_kptr, or %NULL on failure.
//
    __bpf_kfunc void *bpf_refcount_acquire(void *p__refcounted_kptr, btf_struct_meta *meta)
    {
pub static mut ref: *mut c_void = core::ptr::null_mut();
// Could just cast directly to refcount_t *, but need some code using
// bpf_refcount type so that it is emitted in vmlinux BTF
//
    ref = (p__refcounted_kptr + meta.record.refcount_off);
    if (!refcount_inc_not_zero(ref)) {
    return core::ptr::null_mut();
    }
// Verifier strips KF_RET_NULL if input is owned ref, see is_kfunc_ret_null
// in verifier.c
//
    return p__refcounted_kptr;
    }
    __bpf_kfunc void *bpf_refcount_acquire_impl(void *p__refcounted_kptr, void *meta__ign)
    {
    return bpf_refcount_acquire(p__refcounted_kptr, meta__ign);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_list_add(node: *mut bpf_list_node_kern, head: *mut bpf_list_head, prev_ptr: *mut *mut list_head, rec: *mut btf_record, off: u64) -> c_int {
    let mut n = &node.list_head, *h = head;
pub static mut prev: *mut c_void = core::ptr::null_mut();
// If list_head was 0-initialized by map, bpf_obj_init_field wasn't
// called on its fields, so init here
//
    if (unlikely(!h.next)) {
    INIT_LIST_HEAD(h);
    }
    prev = *prev_ptr;
// When prev is not the list head, it must be a node in this list.
    if (prev != h) {
    let mut prev_kn = container_of!(prev, bpf_list_node_kern, list_head);
    if (unlikely(READ_ONCE(prev_kn.owner) != head)) {
// goto;
    }
    }
// node->owner != NULL implies !list_empty(n), no need to separately
// check the latter
//
    if (cmpxchg(&node.owner, core::ptr::null_mut(), BPF_PTR_POISON)) {
// goto;
    }
    list_add(n, prev);
    WRITE_ONCE(node.owner, head);
    return 0;
// label;
// Only called from BPF prog, no need to migrate_disable
    __bpf_obj_drop_impl(n - off, rec, false);
    return -EINVAL;
    }
//
// bpf_list_push_front() - add a node to the front of a BPF linked list
// @head: list head
// @node: node to insert
// @meta: verifier-supplied struct metadata
// @off: verifier-supplied offset of @node within the containing object
//
// Insert @node at the front of @head. The verifier rewrites @meta and @off;
// BPF programs do not set them.
//
// Return: 0 on success, or %-EINVAL if @node is already linked.
//
    __bpf_kfunc int bpf_list_push_front(bpf_list_head *head, bpf_list_node *node, btf_struct_meta *meta,
    u64 off)
    {
    let mut n = node;
    let mut h = head;
    return __bpf_list_add(n, head, &h, meta ? meta.record : core::ptr::null_mut(), off);
    }
    __bpf_kfunc int bpf_list_push_front_impl(bpf_list_head *head, bpf_list_node *node,
    void *meta__ign, u64 off)
    {
    return bpf_list_push_front(head, node, meta__ign, off);
    }
//
// bpf_list_push_back() - add a node to the back of a BPF linked list
// @head: list head
// @node: node to insert
// @meta: verifier-supplied struct metadata
// @off: verifier-supplied offset of @node within the containing object
//
// Insert @node at the back of @head. The verifier rewrites @meta and @off;
// BPF programs do not set them.
//
// Return: 0 on success, or %-EINVAL if @node is already linked.
//
    __bpf_kfunc int bpf_list_push_back(bpf_list_head *head, bpf_list_node *node, btf_struct_meta *meta,
    u64 off)
    {
    let mut n = node;
    let mut h = head;
    return __bpf_list_add(n, head, &h.prev, meta ? meta.record : core::ptr::null_mut(), off);
    }
    __bpf_kfunc int bpf_list_push_back_impl(bpf_list_head *head, bpf_list_node *node,
    void *meta__ign, u64 off)
    {
    return bpf_list_push_back(head, node, meta__ign, off);
    }
    __bpf_kfunc int bpf_list_add(bpf_list_head *head, bpf_list_node *new, bpf_list_node *prev__nonown_allowed, btf_struct_meta *meta, u64 off)
    {
    let mut n = new, *p = prev__nonown_allowed;
    let mut prev_ptr = &p.list_head;
    return __bpf_list_add(n, head, &prev_ptr, meta ? meta.record : core::ptr::null_mut(), off);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_list_del(head: *mut bpf_list_head, n: *mut list_head) -> *mut c_void {
    let mut h = head;
pub static mut node: *mut c_void = core::ptr::null_mut();
// If list_head was 0-initialized by map, bpf_obj_init_field wasn't
// called on its fields, so init here
//
    if (unlikely(!h.next)) {
    INIT_LIST_HEAD(h);
    return core::ptr::null_mut();
    }
    if (list_empty(h)) {
    return core::ptr::null_mut();
    }
    node = container_of!(n, bpf_list_node_kern, list_head);
    if (unlikely(READ_ONCE(node.owner) != head)) {
    return core::ptr::null_mut();
    }
    list_del_init(n);
// Ensure __bpf_list_add() sees the node as unlinked.
    smp_store_release(&node.owner, core::ptr::null_mut());
    return n;
    }
    __bpf_kfunc struct bpf_list_node *bpf_list_pop_front(bpf_list_head *head)
    {
    let mut h = head;
    return __bpf_list_del(head, h.next);
    }
    __bpf_kfunc struct bpf_list_node *bpf_list_pop_back(bpf_list_head *head)
    {
    let mut h = head;
    return __bpf_list_del(head, h.prev);
    }
    __bpf_kfunc struct bpf_list_node *bpf_list_del(bpf_list_head *head, bpf_list_node *node__nonown_allowed)
    {
    let mut kn = node__nonown_allowed;
// verifier guarantees node is a list node rather than list head
    return __bpf_list_del(head, &kn.list_head);
    }
    __bpf_kfunc struct bpf_list_node *bpf_list_front(bpf_list_head *head)
    {
    let mut h = head;
    if (list_empty(h) || unlikely(!h.next)) {
    return core::ptr::null_mut();
    }
    return h.next;
    }
    __bpf_kfunc struct bpf_list_node *bpf_list_back(bpf_list_head *head)
    {
    let mut h = head;
    if (list_empty(h) || unlikely(!h.next)) {
    return core::ptr::null_mut();
    }
    return h.prev;
    }
    __bpf_kfunc bool bpf_list_is_first(bpf_list_head *head, bpf_list_node *node__nonown_allowed)
    {
    let mut h = head;
    let mut kn = node__nonown_allowed;
    if (READ_ONCE(kn.owner) != head) {
    return false;
    }
    return list_is_first(&kn.list_head, h);
    }
    __bpf_kfunc bool bpf_list_is_last(bpf_list_head *head, bpf_list_node *node__nonown_allowed)
    {
    let mut h = head;
    let mut kn = node__nonown_allowed;
    if (READ_ONCE(kn.owner) != head) {
    return false;
    }
    return list_is_last(&kn.list_head, h);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_list_empty(head: *mut bpf_list_head) -> __bpf_kfunc bool {
    let mut h = head;
// If list_head was 0-initialized by map, bpf_obj_init_field wasn't
// called on its fields, so init here
//
    if (unlikely(!h.next)) {
    INIT_LIST_HEAD(h);
    }
    return list_empty(h);
    }
    __bpf_kfunc struct bpf_rb_node *bpf_rbtree_remove(bpf_rb_root *root, bpf_rb_node *node)
    {
    let mut node_internal = node;
    let mut r = root;
    let mut n = &node_internal.rb_node;
// node_internal->owner != root implies either RB_EMPTY_NODE(n) or
// n is owned by some other tree. No need to check RB_EMPTY_NODE(n)
//
    if (READ_ONCE(node_internal.owner) != root) {
    return core::ptr::null_mut();
    }
    rb_erase_cached(n, r);
    RB_CLEAR_NODE(n);
    WRITE_ONCE(node_internal.owner, core::ptr::null_mut());
    return n;
    }
// Need to copy rbtree_add_cached's logic here because our 'less' is a BPF
// program
//
#[no_mangle]
pub unsafe extern "C" fn __bpf_rbtree_add(root: *mut bpf_rb_root, node: *mut bpf_rb_node_kern, less: *mut c_void, rec: *mut btf_record, off: u64) -> c_int {
    let mut link = &(root).rb_root.rb_node;
    let mut parent = core::ptr::null_mut(), *n = &node.rb_node;
pub static mut cb: bpf_callback_t = 0;
pub static mut leftmost: bool = true;
// node->owner != NULL implies !RB_EMPTY_NODE(n), no need to separately
// check the latter
//
    if (cmpxchg(&node.owner, core::ptr::null_mut(), BPF_PTR_POISON)) {
// Only called from BPF prog, no need to migrate_disable
    __bpf_obj_drop_impl(n - off, rec, false);
    return -EINVAL;
    }
    while (*link) {
    parent = *link;
    if (cb((uintptr_t)node, (uintptr_t)parent, 0, 0, 0)) {
    link = &parent.rb_left;
    } else {
    link = &parent.rb_right;
    leftmost = false;
    }
    }
    rb_link_node(n, parent, link);
    rb_insert_color_cached(n, root, leftmost);
    WRITE_ONCE(node.owner, root);
    return 0;
    }
//
// bpf_rbtree_add() - add a node to a BPF rbtree
// @root: tree root
// @node: node to insert
// @less: comparator used to order nodes
// @meta: verifier-supplied struct metadata
// @off: verifier-supplied offset of @node within the containing object
//
// Insert @node into @root using @less. The verifier rewrites @meta and @off;
// BPF programs do not set them.
//
// Return: 0 on success, or %-EINVAL if @node is already linked in a tree.
//
    __bpf_kfunc int bpf_rbtree_add(bpf_rb_root *root, bpf_rb_node *node,
    bool (less)(bpf_rb_node *a, const struct bpf_rb_node *b), btf_struct_meta *meta,
    u64 off)
    {
    let mut n = node;
    return __bpf_rbtree_add(root, n, less, meta ? meta.record : core::ptr::null_mut(), off);
    }
    __bpf_kfunc int bpf_rbtree_add_impl(bpf_rb_root *root, bpf_rb_node *node,
    bool (less)(bpf_rb_node *a, const struct bpf_rb_node *b),
    void *meta__ign, u64 off)
    {
    return bpf_rbtree_add(root, node, less, meta__ign, off);
    }
    __bpf_kfunc struct bpf_rb_node *bpf_rbtree_first(bpf_rb_root *root)
    {
    let mut r = root;
    return rb_first_cached(r);
    }
    __bpf_kfunc struct bpf_rb_node *bpf_rbtree_root(bpf_rb_root *root)
    {
    let mut r = root;
    return r.rb_root.rb_node;
    }
    __bpf_kfunc struct bpf_rb_node *bpf_rbtree_left(bpf_rb_root *root, bpf_rb_node *node)
    {
    let mut node_internal = node;
    if (READ_ONCE(node_internal.owner) != root) {
    return core::ptr::null_mut();
    }
    return node_internal.rb_node.rb_left;
    }
    __bpf_kfunc struct bpf_rb_node *bpf_rbtree_right(bpf_rb_root *root, bpf_rb_node *node)
    {
    let mut node_internal = node;
    if (READ_ONCE(node_internal.owner) != root) {
    return core::ptr::null_mut();
    }
    return node_internal.rb_node.rb_right;
    }
//
// bpf_task_acquire - Acquire a reference to a task. A task acquired by this
// kfunc which is not stored in a map as a kptr, must be released by calling
// bpf_task_release().
// @p: The task on which a reference is being acquired.
//
    __bpf_kfunc struct task_struct *bpf_task_acquire(task_struct *p)
    {
    if (refcount_inc_not_zero(&p.rcu_users)) {
    return p;
    }
    return core::ptr::null_mut();
    }
//
// bpf_task_release - Release the reference acquired on a task.
// @p: The task on which a reference is being released.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_task_release(p: *mut task_struct) -> __bpf_kfunc void {
    put_task_struct_rcu_user(p);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_release_dtor(p: *mut c_void) -> __bpf_kfunc void {
    put_task_struct_rcu_user(p);
    }
    CFI_NOSEAL(bpf_task_release_dtor);

//
// bpf_cgroup_acquire - Acquire a reference to a cgroup. A cgroup acquired by
// this kfunc which is not stored in a map as a kptr, must be released by
// calling bpf_cgroup_release().
// @cgrp: The cgroup on which a reference is being acquired.
//
    __bpf_kfunc struct cgroup *bpf_cgroup_acquire(cgroup *cgrp)
    {
    return cgroup_tryget(cgrp) ? cgrp : core::ptr::null_mut();
    }
//
// bpf_cgroup_release - Release the reference acquired on a cgroup.
// If this kfunc is invoked in an RCU read region, the cgroup is guaranteed to
// not be freed until the current grace period has ended, even if its refcount
// drops to 0.
// @cgrp: The cgroup on which a reference is being released.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_release(cgrp: *mut cgroup) -> __bpf_kfunc void {
    cgroup_put(cgrp);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cgroup_release_dtor(cgrp: *mut c_void) -> __bpf_kfunc void {
    cgroup_put(cgrp);
    }
    CFI_NOSEAL(bpf_cgroup_release_dtor);
//
// bpf_cgroup_ancestor - Perform a lookup on an entry in a cgroup's ancestor
// array. A cgroup returned by this kfunc which is not subsequently stored in a
// map, must be released by calling bpf_cgroup_release().
// @cgrp: The cgroup for which we're performing a lookup.
// @level: The level of ancestor to look up.
//
    __bpf_kfunc struct cgroup *bpf_cgroup_ancestor(cgroup *cgrp, int level)
    {
pub static mut ancestor: *mut c_void = core::ptr::null_mut();
    if (level > cgrp.level || level < 0) {
    return core::ptr::null_mut();
    }
// cgrp's refcnt could be 0 here, but ancestors can still be accessed
    ancestor = cgrp.ancestors[level];
    if (!cgroup_tryget(ancestor)) {
    return core::ptr::null_mut();
    }
    return ancestor;
    }
//
// bpf_cgroup_from_id - Find a cgroup from its ID. A cgroup returned by this
// kfunc which is not subsequently stored in a map, must be released by calling
// bpf_cgroup_release().
// @cgid: cgroup id.
//
    __bpf_kfunc struct cgroup *bpf_cgroup_from_id(u64 cgid)
    {
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    cgrp = __cgroup_get_from_id(cgid);
    if (IS_ERR(cgrp)) {
    return core::ptr::null_mut();
    }
    return cgrp;
    }
//
// bpf_task_under_cgroup - wrap task_under_cgroup_hierarchy() as a kfunc, test
// task's membership of cgroup ancestry.
// @task: the task to be tested
// @ancestor: possible ancestor of @task's cgroup
//
// Tests whether @task's default cgroup hierarchy is a descendant of @ancestor.
// It follows all the same rules as cgroup_is_descendant, and only applies
// to the default hierarchy.
//
    __bpf_kfunc long bpf_task_under_cgroup(task_struct *task, cgroup *ancestor)
    {
    let mut ret = 0;
    rcu_read_lock();
    ret = task_under_cgroup_hierarchy(task, ancestor);
    rcu_read_unlock();
    return ret;
    }
    BPF_CALL_2(bpf_current_task_under_cgroup, bpf_map *, map, u32, idx)
    {
    let mut array = container_of!(map, bpf_array, map);
pub static mut cgrp: *mut c_void = core::ptr::null_mut();
    if (unlikely(idx >= array.map.max_entries)) {
    return -E2BIG;
    }
    cgrp = READ_ONCE(array.ptrs[idx]);
    if (unlikely(!cgrp)) {
    return -EAGAIN;
    }
    return task_under_cgroup_hierarchy(current, cgrp);
    }
pub static mut bpf_func_proto: usize = 0;
//
// bpf_task_get_cgroup1 - Acquires the associated cgroup of a task within a
// specific cgroup1 hierarchy. The cgroup1 hierarchy is identified by its
// hierarchy ID.
// @task: The target task
// @hierarchy_id: The ID of a cgroup1 hierarchy
//
// On success, the cgroup is returen. On failure, NULL is returned.
//
    __bpf_kfunc struct cgroup *
    bpf_task_get_cgroup1(task_struct *task, int hierarchy_id)
    {
    let mut cgrp = task_get_cgroup1(task, hierarchy_id);
    if (IS_ERR(cgrp)) {
    return core::ptr::null_mut();
    }
    return cgrp;
    }

//
// bpf_task_from_pid - Find a struct task_struct from its pid by looking it up
// in the root pid namespace idr. If a task is returned, it must either be
// stored in a map, or released with bpf_task_release().
// @pid: The pid of the task being looked up.
//
    __bpf_kfunc struct task_struct *bpf_task_from_pid(s32 pid)
    {
pub static mut p: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    p = find_task_by_pid_ns(pid, &init_pid_ns);
    if (p) {
    p = bpf_task_acquire(p);
    }
    rcu_read_unlock();
    return p;
    }
//
// bpf_task_from_vpid - Find a struct task_struct from its vpid by looking it up
// in the pid namespace of the current task. If a task is returned, it must
// either be stored in a map, or released with bpf_task_release().
// @vpid: The vpid of the task being looked up.
//
    __bpf_kfunc struct task_struct *bpf_task_from_vpid(s32 vpid)
    {
pub static mut p: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    if (!task_active_pid_ns(current)) {
    return core::ptr::null_mut();
    }
    p = find_task_by_vpid(vpid);
    if (p) {
    p = bpf_task_acquire(p);
    }
    return p;
    }
//
// bpf_dynptr_slice() - Obtain a read-only pointer to the dynptr data.
// @p: The dynptr whose data slice to retrieve
// @offset: Offset into the dynptr
// @buffer__nullable: User-provided buffer to copy contents into.  May be NULL
// @buffer__szk: Size (in bytes) of the buffer if present. This is the
// length of the requested slice. This must be a constant.
//
// For non-skb and non-xdp type dynptrs, there is no difference between
// bpf_dynptr_slice and bpf_dynptr_data.
//
// If buffer__nullable is NULL, the call will fail if buffer_opt was needed.
//
// If the intention is to write to the data slice, please use
// bpf_dynptr_slice_rdwr.
//
// The user must check that the returned pointer is not null before using it.
//
// Please note that in the case of skb and xdp dynptrs, bpf_dynptr_slice
// does not change the underlying packet data pointers, so a call to
// bpf_dynptr_slice will not invalidate any ctx->data/data_end pointers in
// the bpf program.
//
// Return: NULL if the call failed (eg invalid dynptr), pointer to a read-only
// data slice (can be either direct pointer to the data or a pointer to the user
// provided buffer, with its contents containing the data, if unable to obtain
// direct pointer)
//
    __bpf_kfunc void *bpf_dynptr_slice(const struct bpf_dynptr *p, u64 offset,
    void *buffer__nullable, u64 buffer__szk)
    {
    let mut ptr = p;
    enum bpf_dynptr_type type;
pub static mut len: u64 = 0;
    let mut err = 0;
    if (!ptr.data) {
    return core::ptr::null_mut();
    }
    err = bpf_dynptr_check_off_len(ptr, offset, len);
    if (err) {
    return core::ptr::null_mut();
    }
    type = bpf_dynptr_get_type(ptr);
    match (type) {
    BPF_DYNPTR_TYPE_LOCAL => {
    }
    BPF_DYNPTR_TYPE_RINGBUF => {
    return ptr.data + ptr.offset + offset;
    }
    BPF_DYNPTR_TYPE_SKB => {
    if (buffer__nullable) {
    return skb_header_pointer(ptr.data, ptr.offset + offset, len, buffer__nullable);
    }
    else {
    return skb_pointer_if_linear(ptr.data, ptr.offset + offset, len);
    }
    }
    BPF_DYNPTR_TYPE_XDP => {
    {
    let mut xdp_ptr = bpf_xdp_pointer(ptr.data, ptr.offset + offset, len);
    if (!IS_ERR_OR_NULL(xdp_ptr)) {
    return xdp_ptr;
    }
    if (!buffer__nullable) {
    return core::ptr::null_mut();
    }
    bpf_xdp_copy_buf(ptr.data, ptr.offset + offset, buffer__nullable, len, false);
    return buffer__nullable;
    }
    }
    BPF_DYNPTR_TYPE_SKB_META => {
    return bpf_skb_meta_pointer(ptr.data, ptr.offset + offset);
    }
    BPF_DYNPTR_TYPE_FILE => {
    err = bpf_file_fetch_bytes(ptr.data, offset, buffer__nullable, buffer__szk);
    return err ? core::ptr::null_mut() : buffer__nullable;
    }
    _ => {
    WARN_ONCE(true, "unknown dynptr type %d\n", type);
    return core::ptr::null_mut();
    }
    }
    }
//
// bpf_dynptr_slice_rdwr() - Obtain a writable pointer to the dynptr data.
// @p: The dynptr whose data slice to retrieve
// @offset: Offset into the dynptr
// @buffer__nullable: User-provided buffer to copy contents into. May be NULL
// @buffer__szk: Size (in bytes) of the buffer if present. This is the
// length of the requested slice. This must be a constant.
//
// For non-skb and non-xdp type dynptrs, there is no difference between
// bpf_dynptr_slice and bpf_dynptr_data.
//
// If buffer__nullable is NULL, the call will fail if buffer_opt was needed.
//
// The returned pointer is writable and may point to either directly the dynptr
// data at the requested offset or to the buffer if unable to obtain a direct
// data pointer to (example: the requested slice is to the paged area of an skb
// packet). In the case where the returned pointer is to the buffer, the user
// is responsible for persisting writes through calling bpf_dynptr_write(). This
// usually looks something like this pattern:
//
// struct eth_hdr *eth = bpf_dynptr_slice_rdwr(&dynptr, 0, buffer, sizeof!(buffer));
// if (!eth)
// return TC_ACT_SHOT;
//
// // mutate eth header
//
// if (eth == buffer)
// bpf_dynptr_write(&ptr, 0, buffer, sizeof!(buffer), 0);
//
// Please note that, as in the example above, the user must check that the
// returned pointer is not null before using it.
//
// Please also note that in the case of skb and xdp dynptrs, bpf_dynptr_slice_rdwr
// does not change the underlying packet data pointers, so a call to
// bpf_dynptr_slice_rdwr will not invalidate any ctx->data/data_end pointers in
// the bpf program.
//
// Return: NULL if the call failed (eg invalid dynptr), pointer to a
// data slice (can be either direct pointer to the data or a pointer to the user
// provided buffer, with its contents containing the data, if unable to obtain
// direct pointer)
//
    __bpf_kfunc void *bpf_dynptr_slice_rdwr(const struct bpf_dynptr *p, u64 offset,
    void *buffer__nullable, u64 buffer__szk)
    {
    let mut ptr = p;
    if (!ptr.data || __bpf_dynptr_is_rdonly(ptr)) {
    return core::ptr::null_mut();
    }
// bpf_dynptr_slice_rdwr is the same logic as bpf_dynptr_slice.
//
// For skb-type dynptrs, it is safe to write into the returned pointer
// if the bpf program allows skb data writes. There are two possibilities
// that may occur when calling bpf_dynptr_slice_rdwr:
//
// 1) The requested slice is in the head of the skb. In this case, the
// returned pointer is directly to skb data, and if the skb is cloned, the
// verifier will have uncloned it (see bpf_unclone_prologue()) already.
// The pointer can be directly written into.
//
// 2) Some portion of the requested slice is in the paged buffer area.
// In this case, the requested data will be copied out into the buffer
// and the returned pointer will be a pointer to the buffer. The skb
// will not be pulled. To persist the write, the user will need to call
// bpf_dynptr_write(), which will pull the skb and commit the write.
//
// Similarly for xdp programs, if the requested slice is not across xdp
// fragments, then a direct pointer will be returned, otherwise the data
// will be copied out into the buffer and the user will need to call
// bpf_dynptr_write() to commit changes.
//
    return bpf_dynptr_slice(p, offset, buffer__nullable, buffer__szk);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_adjust(p: *mut bpf_dynptr, start: u64, end: u64) -> __bpf_kfunc int {
    let mut ptr = p;
    let mut size = 0;
    if (!ptr.data || start > end) {
    return -EINVAL;
    }
    size = __bpf_dynptr_size(ptr);
    if (start > size || end > size) {
    return -ERANGE;
    }
    bpf_dynptr_advance_offset(ptr, start);
    bpf_dynptr_set_size(ptr, end - start);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_is_null(p: *const bpf_dynptr) -> __bpf_kfunc bool {
    let mut ptr = p;
    return !ptr.data;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_is_rdonly(p: *const bpf_dynptr) -> __bpf_kfunc bool {
    let mut ptr = p;
    if (!ptr.data) {
    return false;
    }
    return __bpf_dynptr_is_rdonly(ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_size(p: *const bpf_dynptr) -> __bpf_kfunc u64 {
    let mut ptr = p;
    if (!ptr.data) {
    return -EINVAL;
    }
    return __bpf_dynptr_size(ptr);
    }
    __bpf_kfunc int bpf_dynptr_clone(const struct bpf_dynptr *p, bpf_dynptr *clone__uninit)
    {
    let mut clone = clone__uninit;
    let mut ptr = p;
    if (!ptr.data) {
    bpf_dynptr_set_null(clone);
    return -EINVAL;
    }
// clone = *ptr;
    return 0;
    }
//
// bpf_dynptr_copy() - Copy data from one dynptr to another.
// @dst_ptr: Destination dynptr - where data should be copied to
// @dst_off: Offset into the destination dynptr
// @src_ptr: Source dynptr - where data should be copied from
// @src_off: Offset into the source dynptr
// @size: Length of the data to copy from source to destination
//
// Copies data from source dynptr to destination dynptr.
// Returns 0 on success; negative error, otherwise.
//
    __bpf_kfunc int bpf_dynptr_copy(const struct bpf_dynptr *dst_ptr, u64 dst_off,
    const struct bpf_dynptr *src_ptr, u64 src_off, u64 size)
    {
    let mut dst = dst_ptr;
    let mut src = src_ptr;
    let mut src_slice = core::ptr::null_mut();
    let mut dst_slice = core::ptr::null_mut();
    char buf[256];
    let mut off = 0;
    src_slice = bpf_dynptr_slice(src_ptr, src_off, core::ptr::null_mut(), size);
    dst_slice = bpf_dynptr_slice_rdwr(dst_ptr, dst_off, core::ptr::null_mut(), size);
    if (src_slice && dst_slice) {
    memmove(dst_slice, src_slice, size);
    return 0;
    }
    if (src_slice) {
    return __bpf_dynptr_write(dst, dst_off, src_slice, size, 0);
    }
    if (dst_slice) {
    return __bpf_dynptr_read(dst_slice, size, src, src_off, 0);
    }
    if (bpf_dynptr_check_off_len(dst, dst_off, size) ||
    bpf_dynptr_check_off_len(src, src_off, size)) {
    return -E2BIG;
    }
    off = 0;
    while (off < size) {
pub static mut chunk_sz: u64 = 0;
    let mut err = 0;
    err = __bpf_dynptr_read(buf, chunk_sz, src, src_off + off, 0);
    if (err) {
    return err;
    }
    err = __bpf_dynptr_write(dst, dst_off + off, buf, chunk_sz, 0);
    if (err) {
    return err;
    }
    off += chunk_sz;
    }
    return 0;
    }
//
// bpf_dynptr_memset() - Fill dynptr memory with a constant byte.
// @p: Destination dynptr - where data will be filled
// @offset: Offset into the dynptr to start filling from
// @size: Number of bytes to fill
// @val: Constant byte to fill the memory with
//
// Fills the @size bytes of the memory area pointed to by @p
// at @offset with the constant byte @val.
// Returns 0 on success; negative error, otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_memset(p: *const bpf_dynptr, offset: u64, size: u64, val: u8) -> __bpf_kfunc int {
    let mut ptr = p;
    u64 chunk_sz, write_off;
    char buf[256];
    void* slice;
    let mut err = 0;
    slice = bpf_dynptr_slice_rdwr(p, offset, core::ptr::null_mut(), size);
    if (likely(slice)) {
    memset(slice, val, size);
    return 0;
    }
    if (__bpf_dynptr_is_rdonly(ptr)) {
    return -EINVAL;
    }
    err = bpf_dynptr_check_off_len(ptr, offset, size);
    if (err) {
    return err;
    }
// Non-linear data under the dynptr, write from a local buffer
    chunk_sz = min_t(u64, sizeof!(buf), size);
    memset(buf, val, chunk_sz);
    while (write_off < size) {
    chunk_sz = min_t(u64, sizeof!(buf), size - write_off);
    err = __bpf_dynptr_write(ptr, offset + write_off, buf, chunk_sz, 0);
    if (err) {
    return err;
    }
    }
    return 0;
    }
    __bpf_kfunc void *bpf_cast_to_kern_ctx(void *obj)
    {
    return obj;
    }
    __bpf_kfunc void *bpf_rdonly_cast(const void *obj__ign, u32 btf_id__k)
    {
    return obj__ign;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_rcu_read_lock() -> __bpf_kfunc void {
    rcu_read_lock();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_rcu_read_unlock() -> __bpf_kfunc void {
    rcu_read_unlock();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_throw_ctx {
    pub aux: *mut bpf_prog_aux,
    pub sp: u64,
    pub bp: u64,
    pub cnt: c_int,
}

#[no_mangle]
unsafe extern "C" fn bpf_stack_walker(cookie: *mut c_void, ip: u64, sp: u64, bp: u64) -> bool {
    let mut ctx = cookie;
pub static mut prog: *mut c_void = core::ptr::null_mut();
//
// The RCU read lock is held to safely traverse the latch tree, but we
// don't need its protection when accessing the prog, since it has an
// active stack frame on the current stack trace, and won't disappear.
//
    rcu_read_lock();
    prog = bpf_prog_ksym_find(ip);
    rcu_read_unlock();
    if (!prog) {
    return !ctx.cnt;
    }
    ctx.cnt += 1;
    if (bpf_is_subprog(prog)) {
    return true;
    }
    ctx.aux = prog.aux;
    ctx.sp = sp;
    ctx.bp = bp;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_throw(cookie: u64) -> __bpf_kfunc void {
pub static mut ctx: bpf_throw_ctx = 0;
    arch_bpf_stack_walk(bpf_stack_walker, &ctx);
    WARN_ON_ONCE!(!ctx.aux);
    if (ctx.aux) {
    WARN_ON_ONCE!(!ctx.aux.exception_boundary);
    }
    WARN_ON_ONCE!(!ctx.bp);
    WARN_ON_ONCE!(!ctx.cnt);
//
// Prevent KASAN false positives for CONFIG_KASAN_STACK by unpoisoning
// deeper stack depths than ctx.sp as we do not return from bpf_throw,
// which skips compiler generated instrumentation to do the same. Some
// architectures cannot recover sp while unwinding, so fall back to bp.
//
    kasan_unpoison_task_stack_below((long)(ctx.sp ?: ctx.bp));
    ctx.aux.bpf_exception_cb(cookie, ctx.sp + ctx.aux.stack_arg_sp_adjust, ctx.bp, 0, 0);
    WARN(1, "A call to BPF exception callback should never return\n");
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_wq_init(wq: *mut bpf_wq, p__const_map: *mut c_void, flags: c_uint) -> __bpf_kfunc int {
    let mut async = wq;
    let mut map = p__const_map;
    BUILD_BUG_ON!(sizeof!(bpf_async_kern) > sizeof!(bpf_wq));
    BUILD_BUG_ON!(__alignof__(bpf_async_kern) != __alignof__(bpf_wq));
    if (flags) {
    return -EINVAL;
    }
    return __bpf_async_init(async, map, flags, BPF_ASYNC_TYPE_WQ);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_wq_start(wq: *mut bpf_wq, flags: c_uint) -> __bpf_kfunc int {
    let mut async = wq;
pub static mut w: *mut c_void = core::ptr::null_mut();
    if (flags) {
    return -EINVAL;
    }
    w = READ_ONCE(async.work);
    if (!w || !READ_ONCE(w.cb.prog)) {
    return -EINVAL;
    }
    if (!refcount_inc_not_zero(&w.cb.refcnt)) {
    return -ENOENT;
    }
    if (!defer_timer_wq_op()) {
    schedule_work(&w.work);
    bpf_async_refcount_put(&w.cb);
    return 0;
    } else {
    return bpf_async_schedule_op(&w.cb, BPF_ASYNC_START, 0, 0);
    }
    }
    __bpf_kfunc int bpf_wq_set_callback(bpf_wq *wq,
    int (callback_fn)(void *map, int *key, void *value),
    unsigned int flags, bpf_prog_aux *aux)
    {
    let mut async = wq;
    if (flags) {
    return -EINVAL;
    }
    return __bpf_async_set_callback(async, callback_fn, aux.prog);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_preempt_disable() -> __bpf_kfunc void {
    preempt_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_preempt_enable() -> __bpf_kfunc void {
    preempt_enable();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_bits {
    pub __opaque: [__u64; 2],
    pub __aligned(8): },
pub const BITS_ITER_NR_WORDS_MAX: c_int = 511;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_bits_kern {
    union {
    pub bits: *mut __u64,
    pub bits_copy: __u64,
}

    let mut nr_bits = 0;
    let mut bit = 0;
    } __aligned(8);
// On 64-bit hosts, unsigned long and u64 have the same size, so passing
// a u64 pointer and an unsigned long pointer to find_next_bit() will
// return the same result, as both point to the same 8-byte area.
//
// For 32-bit little-endian hosts, using a u64 pointer or unsigned long
// pointer also makes no difference. This is because the first iterated
// unsigned long is composed of bits 0-31 of the u64 and the second unsigned
// long is composed of bits 32-63 of the u64.
//
// However, for 32-bit big-endian hosts, this is not the case. The first
// iterated unsigned long will be bits 32-63 of the u64, so swap these two
// ulong values within the u64.
//
#[no_mangle]
unsafe extern "C" fn swap_ulong_in_u64(bits: *mut u64, nr: c_uint) {

    let mut i = 0;
    for (i = 0; i < nr; i++) {
    bits[i] = (bits[i] >> 32) | ((u64)(u32)bits[i] << 32);
    }

    }
//
// bpf_iter_bits_new() - Initialize a new bits iterator for a given memory area
// @it: The new bpf_iter_bits to be created
// @unsafe_ptr__ign: A pointer pointing to a memory area to be iterated over
// @nr_words: The size of the specified memory area, measured in 8-byte units.
// The maximum value of @nr_words is @BITS_ITER_NR_WORDS_MAX. This limit may be
// further reduced by the BPF memory allocator implementation.
//
// This function initializes a new bpf_iter_bits structure for iterating over
// a memory area which is specified by the @unsafe_ptr__ign and @nr_words. It
// copies the data of the memory area to the newly created bpf_iter_bits @it for
// subsequent iteration operations.
//
// On success, 0 is returned. On failure, ERR is returned.
//
    __bpf_kfunc int
    bpf_iter_bits_new(bpf_iter_bits *it, const u64 *unsafe_ptr__ign, u32 nr_words)
    {
    let mut kit = it;
pub static mut nr_bytes: u32 = 0;
pub static mut nr_bits: u32 = 0;
    let mut err = 0;
    BUILD_BUG_ON!(sizeof!(bpf_iter_bits_kern) != sizeof!(bpf_iter_bits));
    BUILD_BUG_ON!(__alignof__(bpf_iter_bits_kern) !=
    __alignof__(bpf_iter_bits));
    kit.nr_bits = 0;
    kit.bits_copy = 0;
    kit.bit = -1;
    if (!unsafe_ptr__ign || !nr_words) {
    return -EINVAL;
    }
    if (nr_words > BITS_ITER_NR_WORDS_MAX) {
    return -E2BIG;
    }
// Optimization for u64 mask
    if (nr_bits == 64) {
    err = bpf_probe_read_kernel_common(&kit.bits_copy, nr_bytes, unsafe_ptr__ign);
    if (err) {
    return -EFAULT;
    }
    swap_ulong_in_u64(&kit.bits_copy, nr_words);
    kit.nr_bits = nr_bits;
    return 0;
    }
    if (bpf_mem_alloc_check_size(false, nr_bytes)) {
    return -E2BIG;
    }
// Fallback to memalloc
    kit.bits = bpf_mem_alloc(&bpf_global_ma, nr_bytes);
    if (!kit.bits) {
    return -ENOMEM;
    }
    err = bpf_probe_read_kernel_common(kit.bits, nr_bytes, unsafe_ptr__ign);
    if (err) {
    bpf_mem_free(&bpf_global_ma, kit.bits);
    return err;
    }
    swap_ulong_in_u64(kit.bits, nr_words);
    kit.nr_bits = nr_bits;
    return 0;
    }
//
// bpf_iter_bits_next() - Get the next bit in a bpf_iter_bits
// @it: The bpf_iter_bits to be checked
//
// This function returns a pointer to a number representing the value of the
// next bit in the bits.
//
// If there are no further bits available, it returns NULL.
//
    __bpf_kfunc int *bpf_iter_bits_next(bpf_iter_bits *it)
    {
    let mut kit = it;
pub static mut bit: c_int = 0;
pub static mut bits: *mut c_void = core::ptr::null_mut();
    if (!nr_bits || bit >= nr_bits) {
    return core::ptr::null_mut();
    }
    bits = nr_bits == 64 ? &kit.bits_copy : kit.bits;
    bit = find_next_bit(bits, nr_bits, bit + 1);
    if (bit >= nr_bits) {
    kit.bit = bit;
    return core::ptr::null_mut();
    }
    kit.bit = bit;
    return &kit.bit;
    }
//
// bpf_iter_bits_destroy() - Destroy a bpf_iter_bits
// @it: The bpf_iter_bits to be destroyed
//
// Destroy the resource associated with the bpf_iter_bits.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_iter_bits_destroy(it: *mut bpf_iter_bits) -> __bpf_kfunc void {
    let mut kit = it;
    if (kit.nr_bits <= 64) {
    return;
    }
    bpf_mem_free(&bpf_global_ma, kit.bits);
    }
//
// bpf_copy_from_user_str() - Copy a string from an unsafe user address
// @dst:             Destination address, in kernel space.  This buffer must be
// at least @dst__sz bytes long.
// @dst__sz:         Maximum number of bytes to copy, includes the trailing NUL.
// @unsafe_ptr__ign: Source address, in user space.
// @flags:           The only supported flag is BPF_F_PAD_ZEROS
//
// Copies a NUL-terminated string from userspace to BPF space. If user string is
// too long this will still ensure zero termination in the dst buffer unless
// buffer size is 0.
//
// If BPF_F_PAD_ZEROS flag is set, memset the tail of @dst to 0 on success and
// memset all of @dst on failure.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_copy_from_user_str(dst: *mut c_void, dst__sz: u32, unsafe_ptr__ign: *const c_void , flags: u64) -> __bpf_kfunc int {
    let mut ret = 0;
    if (unlikely(flags & ~BPF_F_PAD_ZEROS)) {
    return -EINVAL;
    }
    if (unlikely(!dst__sz)) {
    return 0;
    }
    ret = strncpy_from_user(dst, unsafe_ptr__ign, dst__sz - 1);
    if (ret < 0) {
    if (flags & BPF_F_PAD_ZEROS) {
    memset(dst, 0, dst__sz);
    }
    return ret;
    }
    if (flags & BPF_F_PAD_ZEROS) {
    memset(dst + ret, 0, dst__sz - ret);
    }
    else {
    (dst)[ret] = '\0';
    }
    return ret + 1;
    }
//
// bpf_copy_from_user_task_str() - Copy a string from an task's address space
// @dst:             Destination address, in kernel space.  This buffer must be
// at least @dst__sz bytes long.
// @dst__sz:         Maximum number of bytes to copy, includes the trailing NUL.
// @unsafe_ptr__ign: Source address in the task's address space.
// @tsk:             The task whose address space will be used
// @flags:           The only supported flag is BPF_F_PAD_ZEROS
//
// Copies a NUL terminated string from a task's address space to @dst__sz
// buffer. If user string is too long this will still ensure zero termination
// in the @dst__sz buffer unless buffer size is 0.
//
// If BPF_F_PAD_ZEROS flag is set, memset the tail of @dst__sz to 0 on success
// and memset all of @dst__sz on failure.
//
// Return: The number of copied bytes on success including the NUL terminator.
// A negative error code on failure.
//
    __bpf_kfunc int bpf_copy_from_user_task_str(void *dst, u32 dst__sz,
    const void  *unsafe_ptr__ign, task_struct *tsk, u64 flags)
    {
    let mut ret = 0;
    if (unlikely(flags & ~BPF_F_PAD_ZEROS)) {
    return -EINVAL;
    }
    if (unlikely(dst__sz == 0)) {
    return 0;
    }
    ret = copy_remote_vm_str(tsk, (unsigned long)unsafe_ptr__ign, dst, dst__sz, 0);
    if (ret < 0) {
    if (flags & BPF_F_PAD_ZEROS) {
    memset(dst, 0, dst__sz);
    }
    return ret;
    }
    if (flags & BPF_F_PAD_ZEROS) {
    memset(dst + ret, 0, dst__sz - ret);
    }
    return ret + 1;
    }
// Keep unsigned long in prototype so that kfunc is usable when emitted to
// vmlinux.h in BPF programs directly, but note that while in BPF prog, the
// unsigned long always points to 8-byte region on stack, the kernel may only
// read and write the 4-bytes on 32-bit.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_local_irq_save(flags__irq_flag: *mut c_ulong) -> __bpf_kfunc void {
    local_irq_save(*flags__irq_flag);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_irq_restore(flags__irq_flag: *mut c_ulong) -> __bpf_kfunc void {
    local_irq_restore(*flags__irq_flag);
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_trap() -> __bpf_kfunc void {
    }
//
// Kfuncs for string operations.
//
// Since strings are not necessarily %NUL-terminated, we cannot directly call
// in-kernel implementations. Instead, we open-code the implementations using
// __get_kernel_nofault instead of plain dereference to make them safe.
//
#[no_mangle]
unsafe extern "C" fn __bpf_strncasecmp(s1: *const c_char, s2: *const c_char, ignore_case: bool, len: usize) -> c_int {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut i = 0;
    if (!copy_from_kernel_nofault_allowed(s1, 1) ||
    !copy_from_kernel_nofault_allowed(s2, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < len && i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&c1, s1, char, err_out);
    __get_kernel_nofault(&c2, s2, char, err_out);
    if (ignore_case) {
    c1 = tolower(c1);
    c2 = tolower(c2);
    }
    if (c1 != c2) {
    return c1 < c2 ? -1 : 1;
    }
    if (c1 == '\0') {
    return 0;
    }
    s1 += 1;
    s2 += 1;
    }
pub static mut i: return = 0;
// label;
    return -EFAULT;
    }
//
// bpf_strcmp - Compare two strings
// @s1__ign: One string
// @s2__ign: Another string
//
// Return:
// * %0       - Strings are equal
// * %-1      - @s1__ign is smaller
// * %1       - @s2__ign is smaller
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of strings is too large
// * %-ERANGE - One of strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strcmp(s1__ign: *const c_char, s2__ign: *const c_char) -> __bpf_kfunc int {
    return __bpf_strncasecmp(s1__ign, s2__ign, false, XATTR_SIZE_MAX);
    }
//
// bpf_strcasecmp - Compare two strings, ignoring the case of the characters
// @s1__ign: One string
// @s2__ign: Another string
//
// Return:
// * %0       - Strings are equal
// * %-1      - @s1__ign is smaller
// * %1       - @s2__ign is smaller
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of strings is too large
// * %-ERANGE - One of strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strcasecmp(s1__ign: *const c_char, s2__ign: *const c_char) -> __bpf_kfunc int {
    return __bpf_strncasecmp(s1__ign, s2__ign, true, XATTR_SIZE_MAX);
    }
//
// bpf_strncasecmp - Compare two length-limited strings, ignoring case
// @s1__ign: One string
// @s2__ign: Another string
// @len: The maximum number of characters to compare
//
// Return:
// * %0       - Strings are equal
// * %-1      - @s1__ign is smaller
// * %1       - @s2__ign is smaller
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of strings is too large
// * %-ERANGE - One of strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strncasecmp(s1__ign: *const c_char, s2__ign: *const c_char, len: usize) -> __bpf_kfunc int {
    return __bpf_strncasecmp(s1__ign, s2__ign, true, len);
    }
//
// bpf_strnchr - Find a character in a length limited string
// @s__ign: The string to be searched
// @count: The number of characters to be searched
// @c: The character to search for
//
// Note that the %NUL-terminator is considered part of the string, and can
// be searched for.
//
// Return:
// * >=0      - Index of the first occurrence of @c within @s__ign
// * %-ENOENT - @c not found in the first @count characters of @s__ign
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strnchr(s__ign: *const c_char, count: usize, c: c_char) -> __bpf_kfunc int {
    let mut sc = 0;
    let mut i = 0;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < count && i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&sc, s__ign, char, err_out);
    if (sc == c) {
    return i;
    }
    if (sc == '\0') {
    return -ENOENT;
    }
    s__ign += 1;
    }
pub static mut i: return = 0;
// label;
    return -EFAULT;
    }
//
// bpf_strchr - Find the first occurrence of a character in a string
// @s__ign: The string to be searched
// @c: The character to search for
//
// Note that the %NUL-terminator is considered part of the string, and can
// be searched for.
//
// Return:
// * >=0      - The index of the first occurrence of @c within @s__ign
// * %-ENOENT - @c not found in @s__ign
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strchr(s__ign: *const c_char, c: c_char) -> __bpf_kfunc int {
    return bpf_strnchr(s__ign, XATTR_SIZE_MAX, c);
    }
//
// bpf_strchrnul - Find and return a character in a string, or end of string
// @s__ign: The string to be searched
// @c: The character to search for
//
// Return:
// * >=0      - Index of the first occurrence of @c within @s__ign or index of
// the null byte at the end of @s__ign when @c is not found
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strchrnul(s__ign: *const c_char, c: c_char) -> __bpf_kfunc int {
    let mut sc = 0;
    let mut i = 0;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&sc, s__ign, char, err_out);
    if (sc == '\0' || sc == c) {
    return i;
    }
    s__ign += 1;
    }
    return -E2BIG;
// label;
    return -EFAULT;
    }
//
// bpf_strrchr - Find the last occurrence of a character in a string
// @s__ign: The string to be searched
// @c: The character to search for
//
// Return:
// * >=0      - Index of the last occurrence of @c within @s__ign
// * %-ENOENT - @c not found in @s__ign
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strrchr(s__ign: *const c_char, c: c_int) -> __bpf_kfunc int {
    let mut sc = 0;
    int i, last = -ENOENT;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&sc, s__ign, char, err_out);
    if (sc == c) {
    last = i;
    }
    if (sc == '\0') {
    return last;
    }
    s__ign += 1;
    }
    return -E2BIG;
// label;
    return -EFAULT;
    }
//
// bpf_strnlen - Calculate the length of a length-limited string
// @s__ign: The string
// @count: The maximum number of characters to count
//
// Return:
// * >=0      - The length of @s__ign
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strnlen(s__ign: *const c_char, count: usize) -> __bpf_kfunc int {
    let mut c = 0;
    let mut i = 0;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < count && i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&c, s__ign, char, err_out);
    if (c == '\0') {
    return i;
    }
    s__ign += 1;
    }
pub static mut i: return = 0;
// label;
    return -EFAULT;
    }
//
// bpf_strlen - Calculate the length of a string
// @s__ign: The string
//
// Return:
// * >=0      - The length of @s__ign
// * %-EFAULT - Cannot read @s__ign
// * %-E2BIG  - @s__ign is too large
// * %-ERANGE - @s__ign is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strlen(s__ign: *const c_char) -> __bpf_kfunc int {
    return bpf_strnlen(s__ign, XATTR_SIZE_MAX);
    }
//
// bpf_strspn - Calculate the length of the initial substring of @s__ign which
// only contains letters in @accept__ign
// @s__ign: The string to be searched
// @accept__ign: The string to search for
//
// Return:
// * >=0      - The length of the initial substring of @s__ign which only
// contains letters from @accept__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strspn(s__ign: *const c_char, accept__ign: *const c_char) -> __bpf_kfunc int {
    let mut cs = 0;
    let mut ca = 0;
    let mut i = 0;
    let mut j = 0;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1) ||
    !copy_from_kernel_nofault_allowed(accept__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&cs, s__ign, char, err_out);
    if (cs == '\0') {
    return i;
    }
    while (j < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&ca, accept__ign + j, char, err_out);
    if (cs == ca || ca == '\0') {
    break;
    }
    }
    if (j == XATTR_SIZE_MAX) {
    return -E2BIG;
    }
    if (ca == '\0') {
    return i;
    }
    s__ign += 1;
    }
    return -E2BIG;
// label;
    return -EFAULT;
    }
//
// bpf_strcspn - Calculate the length of the initial substring of @s__ign which
// does not contain letters in @reject__ign
// @s__ign: The string to be searched
// @reject__ign: The string to search for
//
// Return:
// * >=0      - The length of the initial substring of @s__ign which does not
// contain letters from @reject__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strcspn(s__ign: *const c_char, reject__ign: *const c_char) -> __bpf_kfunc int {
    let mut cs = 0;
    let mut cr = 0;
    let mut i = 0;
    let mut j = 0;
    if (!copy_from_kernel_nofault_allowed(s__ign, 1) ||
    !copy_from_kernel_nofault_allowed(reject__ign, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&cs, s__ign, char, err_out);
    if (cs == '\0') {
    return i;
    }
    while (j < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&cr, reject__ign + j, char, err_out);
    if (cs == cr || cr == '\0') {
    break;
    }
    }
    if (j == XATTR_SIZE_MAX) {
    return -E2BIG;
    }
    if (cr != '\0') {
    return i;
    }
    s__ign += 1;
    }
    return -E2BIG;
// label;
    return -EFAULT;
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_strnstr(s1: *mut c_char, s2: *mut c_char, len: size_t, ignore_case: bool) -> c_int {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut i = 0;
    let mut j = 0;
    if (!copy_from_kernel_nofault_allowed(s1, 1) ||
    !copy_from_kernel_nofault_allowed(s2, 1)) {
    return -ERANGE;
    }
    guard(pagefault)();
    while (i < XATTR_SIZE_MAX) {
    while (i + j <= len && j < XATTR_SIZE_MAX) {
    __get_kernel_nofault(&c2, s2 + j, char, err_out);
    if (c2 == '\0') {
    return i;
    }
//
// We allow reading an extra byte from s2 (note the
// `i + j <= len` above) to cover the case when s2 is
// a suffix of the first len chars of s1.
//
    if (i + j == len) {
    break;
    }
    __get_kernel_nofault(&c1, s1 + j, char, err_out);
    if (ignore_case) {
    c1 = tolower(c1);
    c2 = tolower(c2);
    }
    if (c1 == '\0') {
    return -ENOENT;
    }
    if (c1 != c2) {
    break;
    }
    }
    if (j == XATTR_SIZE_MAX) {
    return -E2BIG;
    }
    if (i + j == len) {
    return -ENOENT;
    }
    s1 += 1;
    }
    return -E2BIG;
// label;
    return -EFAULT;
    }
//
// bpf_strstr - Find the first substring in a string
// @s1__ign: The string to be searched
// @s2__ign: The string to search for
//
// Return:
// * >=0      - Index of the first character of the first occurrence of @s2__ign
// within @s1__ign
// * %-ENOENT - @s2__ign is not a substring of @s1__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strstr(s1__ign: *const c_char, s2__ign: *const c_char) -> __bpf_kfunc int {
    return __bpf_strnstr(s1__ign, s2__ign, XATTR_SIZE_MAX, false);
    }
//
// bpf_strcasestr - Find the first substring in a string, ignoring the case of
// the characters
// @s1__ign: The string to be searched
// @s2__ign: The string to search for
//
// Return:
// * >=0      - Index of the first character of the first occurrence of @s2__ign
// within @s1__ign
// * %-ENOENT - @s2__ign is not a substring of @s1__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
#[no_mangle]
pub unsafe extern "C" fn bpf_strcasestr(s1__ign: *const c_char, s2__ign: *const c_char) -> __bpf_kfunc int {
    return __bpf_strnstr(s1__ign, s2__ign, XATTR_SIZE_MAX, true);
    }
//
// bpf_strnstr - Find the first substring in a length-limited string
// @s1__ign: The string to be searched
// @s2__ign: The string to search for
// @len: the maximum number of characters to search
//
// Return:
// * >=0      - Index of the first character of the first occurrence of @s2__ign
// within the first @len characters of @s1__ign
// * %-ENOENT - @s2__ign not found in the first @len characters of @s1__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
    __bpf_kfunc int bpf_strnstr(const char *s1__ign, const char *s2__ign,
    size_t len)
    {
    return __bpf_strnstr(s1__ign, s2__ign, len, false);
    }
//
// bpf_strncasestr - Find the first substring in a length-limited string,
// ignoring the case of the characters
// @s1__ign: The string to be searched
// @s2__ign: The string to search for
// @len: the maximum number of characters to search
//
// Return:
// * >=0      - Index of the first character of the first occurrence of @s2__ign
// within the first @len characters of @s1__ign
// * %-ENOENT - @s2__ign not found in the first @len characters of @s1__ign
// * %-EFAULT - Cannot read one of the strings
// * %-E2BIG  - One of the strings is too large
// * %-ERANGE - One of the strings is outside of kernel address space
//
    __bpf_kfunc int bpf_strncasestr(const char *s1__ign, const char *s2__ign,
    size_t len)
    {
    return __bpf_strnstr(s1__ign, s2__ign, len, true);
    }

//
// bpf_lookup_user_key - lookup a key by its serial
// @serial: key handle serial number
// @flags: lookup-specific flags
//
// Search a key with a given *serial* and the provided *flags*.
// If found, increment the reference count of the key by one, and
// return it in the bpf_key structure.
//
// The bpf_key structure must be passed to bpf_key_put() when done
// with it, so that the key reference count is decremented and the
// bpf_key structure is freed.
//
// Permission checks are deferred to the time the key is used by
// one of the available key-specific kfuncs.
//
// Set *flags* with KEY_LOOKUP_CREATE, to attempt creating a requested
// special keyring (e.g. session keyring), if it doesn't yet exist.
// Set *flags* with KEY_LOOKUP_PARTIAL, to lookup a key without waiting
// for the key construction, and to retrieve uninstantiated keys (keys
// without data attached to them).
//
// Return: a bpf_key pointer with a valid key pointer if the key is found, a
// NULL pointer otherwise.
//
    __bpf_kfunc struct bpf_key *bpf_lookup_user_key(s32 serial, u64 flags)
    {
    let mut key_ref;
pub static mut bkey: *mut c_void = core::ptr::null_mut();
    if (flags & ~KEY_LOOKUP_ALL) {
    return core::ptr::null_mut();
    }
//
// Permission check is deferred until the key is used, as the
// intent of the caller is unknown here.
//
    key_ref = lookup_user_key(serial, flags, KEY_DEFER_PERM_CHECK);
    if (IS_ERR(key_ref)) {
    return core::ptr::null_mut();
    }
    bkey = kmalloc_obj(*bkey);
    if (!bkey) {
    key_put(key_ref_to_ptr(key_ref));
    return core::ptr::null_mut();
    }
    bkey.key = key_ref_to_ptr(key_ref);
    bkey.has_ref = true;
    return bkey;
    }
//
// bpf_lookup_system_key - lookup a key by a system-defined ID
// @id: key ID
//
// Obtain a bpf_key structure with a key pointer set to the passed key ID.
// The key pointer is marked as invalid, to prevent bpf_key_put() from
// attempting to decrement the key reference count on that pointer. The key
// pointer set in such way is currently understood only by
// verify_pkcs7_signature().
//
// Set *id* to one of the values defined in include/linux/verification.h:
// 0 for the primary keyring (immutable keyring of system keys);
// VERIFY_USE_SECONDARY_KEYRING for both the primary and secondary keyring
// (where keys can be added only if they are vouched for by existing keys
// in those keyrings); VERIFY_USE_PLATFORM_KEYRING for the platform
// keyring (primarily used by the integrity subsystem to verify a kexec'ed
// kerned image and, possibly, the initramfs signature).
//
// Return: a bpf_key pointer with an invalid key pointer set from the
// pre-determined ID on success, a NULL pointer otherwise
//
    __bpf_kfunc struct bpf_key *bpf_lookup_system_key(u64 id)
    {
pub static mut bkey: *mut c_void = core::ptr::null_mut();
    if (system_keyring_id_check(id) < 0) {
    return core::ptr::null_mut();
    }
    bkey = kmalloc_obj(*bkey, GFP_ATOMIC);
    if (!bkey) {
    return core::ptr::null_mut();
    }
    bkey.key = (unsigned long)id;
    bkey.has_ref = false;
    return bkey;
    }
//
// bpf_key_put - decrement key reference count if key is valid and free bpf_key
// @bkey: bpf_key structure
//
// Decrement the reference count of the key inside *bkey*, if the pointer
// is valid, and free *bkey*.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_key_put(bkey: *mut bpf_key) -> __bpf_kfunc void {
    if (bkey.has_ref) {
    key_put(bkey.key);
    }
    kfree(bkey);
    }
//
// bpf_verify_pkcs7_signature - verify a PKCS#7 signature
// @data_p: data to verify
// @sig_p: signature of the data
// @trusted_keyring: keyring with keys trusted for signature verification
//
// Verify the PKCS#7 signature *sig_ptr* against the supplied *data_ptr
// with keys in a keyring referenced by *trusted_keyring*.
//
// Return: 0 on success, a negative value on error.
//
    __bpf_kfunc int bpf_verify_pkcs7_signature(const struct bpf_dynptr *data_p,
    const struct bpf_dynptr *sig_p, bpf_key *trusted_keyring)
    {

    let mut data_ptr = data_p;
    let mut sig_ptr = sig_p;
    let mut data = core::ptr::null_mut();
    let mut sig = core::ptr::null_mut();
    u32 data_len, sig_len;
    let mut ret = 0;
    if (trusted_keyring.has_ref) {
//
// Do the permission check deferred in bpf_lookup_user_key().
// See bpf_lookup_user_key() for more details.
//
// A call to key_task_permission() here would be redundant, as
// it is already done by keyring_search() called by
// find_asymmetric_key().
//
    ret = key_validate(trusted_keyring.key);
    if (ret < 0) {
    return ret;
    }
    }
    data_len = __bpf_dynptr_size(data_ptr);
    data = __bpf_dynptr_data(data_ptr, data_len);
    if (!data) {
    return -EINVAL;
    }
    sig_len = __bpf_dynptr_size(sig_ptr);
    sig = __bpf_dynptr_data(sig_ptr, sig_len);
    if (!sig) {
    return -EINVAL;
    }
    return verify_pkcs7_signature(data, data_len, sig, sig_len,
    trusted_keyring.key,
    VERIFYING_BPF_SIGNATURE, core::ptr::null_mut(),
    core::ptr::null_mut());

    return -EOPNOTSUPP;

    }

    typedef int (*bpf_task_work_callback_t)(bpf_map *map, void *key, void *value);
    enum bpf_task_work_state {
// bpf_task_work is ready to be used
    BPF_TW_STANDBY = 0,
// irq work scheduling in progress
    BPF_TW_PENDING,
// task work scheduling in progress
    BPF_TW_SCHEDULING,
// task work is scheduled successfully
    BPF_TW_SCHEDULED,
// callback is running
    BPF_TW_RUNNING,
// associated BPF map value is deleted
    BPF_TW_FREED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_task_work_ctx {
    pub state: bpf_task_work_state,
    pub refcnt: refcount_t,
    pub work: callback_head,
    pub irq_work: irq_work,
// bpf_prog that schedules task work
    pub prog: *mut bpf_prog,
// task for which callback is scheduled
    pub task: *mut task_struct,
// the map and map value associated with this context
    pub map: *mut bpf_map,
    pub map_val: *mut c_void,
    pub mode: task_work_notify_mode,
    pub callback_fn: bpf_callback_t,
    pub rcu: rcu_head,
    pub __aligned(8): },
// Actual type for struct bpf_task_work
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_task_work_kern {
    pub ctx: *mut bpf_task_work_ctx,
}

#[no_mangle]
unsafe extern "C" fn bpf_task_work_ctx_reset(ctx: *mut bpf_task_work_ctx) {
    if (ctx.prog) {
    bpf_prog_put(ctx.prog);
    ctx.prog = core::ptr::null_mut();
    }
    if (ctx.task) {
    bpf_task_release(ctx.task);
    ctx.task = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_ctx_tryget(ctx: *mut bpf_task_work_ctx) -> bool {
    return refcount_inc_not_zero(&ctx.refcnt);
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_destroy(irq_work: *mut irq_work) {
    let mut ctx = container_of!(irq_work, bpf_task_work_ctx, irq_work);
    bpf_task_work_ctx_reset(ctx);
    kfree_rcu(ctx, rcu);
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_ctx_put(ctx: *mut bpf_task_work_ctx) {
    if (!refcount_dec_and_test(&ctx.refcnt)) {
    return;
    }
    if (irqs_disabled()) {
    ctx.irq_work = IRQ_WORK_INIT(bpf_task_work_destroy);
    irq_work_queue(&ctx.irq_work);
    } else {
    bpf_task_work_destroy(&ctx.irq_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_cancel(ctx: *mut bpf_task_work_ctx) {
//
// Scheduled task_work callback holds ctx ref, so if we successfully
// cancelled, we put that ref on callback's behalf. If we couldn't
// cancel, callback will inevitably run or has already completed
// running, and it would have taken care of its ctx ref itself.
//
    if (task_work_cancel(ctx.task, &ctx.work)) {
    bpf_task_work_ctx_put(ctx);
    }
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_callback(cb: *mut callback_head) {
    let mut ctx = container_of!(cb, bpf_task_work_ctx, work);
    enum bpf_task_work_state state;
    let mut idx = 0;
pub static mut key: *mut c_void = core::ptr::null_mut();
// Read lock is needed to protect ctx and map key/value access
    guard(rcu_tasks_trace)();
//
// This callback may start running before bpf_task_work_irq() switched to
// SCHEDULED state, so handle both transition variants SCHEDULING|SCHEDULED -> RUNNING.
//
    state = cmpxchg(&ctx.state, BPF_TW_SCHEDULING, BPF_TW_RUNNING);
    if (state == BPF_TW_SCHEDULED) {
    state = cmpxchg(&ctx.state, BPF_TW_SCHEDULED, BPF_TW_RUNNING);
    }
    if (state == BPF_TW_FREED) {
    bpf_task_work_ctx_put(ctx);
    return;
    }
    key = map_key_from_value(ctx.map, ctx.map_val, &idx);
    migrate_disable();
    ctx.callback_fn((u64)(long)ctx.map, (u64)(long)key,
    (u64)(long)ctx.map_val, 0, 0);
    migrate_enable();
    bpf_task_work_ctx_reset(ctx);
    (void)cmpxchg(&ctx.state, BPF_TW_RUNNING, BPF_TW_STANDBY);
    bpf_task_work_ctx_put(ctx);
    }
#[no_mangle]
unsafe extern "C" fn bpf_task_work_irq(irq_work: *mut irq_work) {
    let mut ctx = container_of!(irq_work, bpf_task_work_ctx, irq_work);
    enum bpf_task_work_state state;
    let mut err = 0;
    guard(rcu)();
    if (cmpxchg(&ctx.state, BPF_TW_PENDING, BPF_TW_SCHEDULING) != BPF_TW_PENDING) {
    bpf_task_work_ctx_put(ctx);
    return;
    }
    err = task_work_add(ctx.task, &ctx.work, ctx.mode);
    if (err) {
    bpf_task_work_ctx_reset(ctx);
//
// try to switch back to STANDBY for another task_work reuse, but we might have
// gone to FREED already, which is fine as we already cleaned up after ourselves
//
    (void)cmpxchg(&ctx.state, BPF_TW_SCHEDULING, BPF_TW_STANDBY);
    bpf_task_work_ctx_put(ctx);
    return;
    }
//
// It's technically possible for just scheduled task_work callback to
// complete running by now, going SCHEDULING -> RUNNING and then
// dropping its ctx refcount. Instead of capturing an extra ref just
// to protect below ctx->state access, we rely on rcu_read_lock
// above to prevent kfree_rcu from freeing ctx before we return.
//
    state = cmpxchg(&ctx.state, BPF_TW_SCHEDULING, BPF_TW_SCHEDULED);
    if (state == BPF_TW_FREED) {
    bpf_task_work_cancel(ctx); /* clean up if we switched into FREED state */
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_work_fetch_ctx(tw: *mut bpf_task_work, map: *mut bpf_map) -> *mut c_void {
    let mut twk = tw;
    let mut ctx = core::ptr::null_mut();
    let mut old_ctx = core::ptr::null_mut();
    ctx = READ_ONCE(twk.ctx);
    if (ctx) {
    return ctx;
    }
    ctx = bpf_map_kmalloc_nolock(map, sizeof!(*ctx), 0, NUMA_NO_NODE);
    if (!ctx) {
    return ERR_PTR(-ENOMEM);
    }
    memset(ctx, 0, sizeof!(*ctx));
    refcount_set(&ctx.refcnt, 1); /* map's own ref */
    ctx.state = BPF_TW_STANDBY;
    old_ctx = cmpxchg(&twk.ctx, core::ptr::null_mut(), ctx);
    if (old_ctx) {
//
// tw->ctx is set by concurrent BPF program, release allocated
// memory and try to reuse already set context.
//
    kfree_nolock(ctx);
    return old_ctx;
    }
    return ctx; /* Success */
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_work_acquire_ctx(tw: *mut bpf_task_work, map: *mut bpf_map) -> *mut c_void {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
//
// Sleepable BPF programs hold rcu_read_lock_trace but not
// regular rcu_read_lock. Since kfree_rcu waits for regular
// RCU GP, the ctx can be freed while we're between reading
// the pointer and incrementing the refcount. Take regular
// rcu_read_lock to prevent kfree_rcu from freeing the ctx
// before we can tryget it.
//
    scoped_guard(rcu) {
    ctx = bpf_task_work_fetch_ctx(tw, map);
    if (IS_ERR(ctx)) {
    return ctx;
    }
// try to get ref for task_work callback to hold
    if (!bpf_task_work_ctx_tryget(ctx)) {
    return ERR_PTR(-EBUSY);
    }
    }
    if (cmpxchg(&ctx.state, BPF_TW_STANDBY, BPF_TW_PENDING) != BPF_TW_STANDBY) {
// lost acquiring race or map_release_uref() stole it from us, put ref and bail
    bpf_task_work_ctx_put(ctx);
    return ERR_PTR(-EBUSY);
    }
//
// If no process or bpffs is holding a reference to the map, no new callbacks should be
// scheduled. This does not address any race or correctness issue, but rather is a policy
// choice: dropping user references should stop everything.
//
    if (!atomic64_read(&map.usercnt)) {
// drop ref we just got for task_work callback itself
    bpf_task_work_ctx_put(ctx);
// transfer map's ref into cancel_and_free()
    bpf_task_work_cancel_and_free(tw);
    return ERR_PTR(-EBUSY);
    }
    return ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_work_schedule(task: *mut task_struct, tw: *mut bpf_task_work, map: *mut bpf_map, callback_fn: *mut c_void, aux: *mut bpf_prog_aux, mode: task_work_notify_mode) -> c_int {
pub static mut prog: *mut c_void = core::ptr::null_mut();
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    BTF_TYPE_EMIT(bpf_task_work);
    prog = bpf_prog_inc_not_zero(aux.prog);
    if (IS_ERR(prog)) {
    return -EBADF;
    }
    task = bpf_task_acquire(task);
    if (!task) {
    err = -EBADF;
// goto;
    }
    ctx = bpf_task_work_acquire_ctx(tw, map);
    if (IS_ERR(ctx)) {
    err = PTR_ERR(ctx);
// goto;
    }
    ctx.task = task;
    ctx.callback_fn = (bpf_callback_t)callback_fn;
    ctx.prog = prog;
    ctx.mode = mode;
    ctx.map = map;
    ctx.map_val = tw - map.record.task_work_off;
    init_task_work(&ctx.work, bpf_task_work_callback);
    init_irq_work(&ctx.irq_work, bpf_task_work_irq);
    irq_work_queue(&ctx.irq_work);
    return 0;
// label;
    bpf_task_release(task);
// label;
    bpf_prog_put(prog);
    return err;
    }
//
// bpf_task_work_schedule_signal - Schedule BPF callback using task_work_add with TWA_SIGNAL
// mode
// @task: Task struct for which callback should be scheduled
// @tw: Pointer to struct bpf_task_work in BPF map value for internal bookkeeping
// @map__const_map: bpf_map that embeds struct bpf_task_work in the values
// @callback: pointer to BPF subprogram to call
// @aux: pointer to bpf_prog_aux of the caller BPF program, implicitly set by the verifier
//
// Return: 0 if task work has been scheduled successfully, negative error code otherwise
//
    __bpf_kfunc int bpf_task_work_schedule_signal(task_struct *task, bpf_task_work *tw,
    void *map__const_map, bpf_task_work_callback_t callback, bpf_prog_aux *aux)
    {
    return bpf_task_work_schedule(task, tw, map__const_map, callback, aux, TWA_SIGNAL);
    }
//
// bpf_task_work_schedule_resume - Schedule BPF callback using task_work_add with TWA_RESUME
// mode
// @task: Task struct for which callback should be scheduled
// @tw: Pointer to struct bpf_task_work in BPF map value for internal bookkeeping
// @map__const_map: bpf_map that embeds struct bpf_task_work in the values
// @callback: pointer to BPF subprogram to call
// @aux: pointer to bpf_prog_aux of the caller BPF program, implicitly set by the verifier
//
// Return: 0 if task work has been scheduled successfully, negative error code otherwise
//
    __bpf_kfunc int bpf_task_work_schedule_resume(task_struct *task, bpf_task_work *tw,
    void *map__const_map, bpf_task_work_callback_t callback, bpf_prog_aux *aux)
    {
    return bpf_task_work_schedule(task, tw, map__const_map, callback, aux, TWA_RESUME);
    }
#[no_mangle]
pub unsafe extern "C" fn make_file_dynptr(file: *mut file, flags: u32, may_sleep: bool, ptr: *mut bpf_dynptr_kern) -> c_int {
pub static mut state: *mut c_void = core::ptr::null_mut();
// flags is currently unsupported
    if (flags) {
    bpf_dynptr_set_null(ptr);
    return -EINVAL;
    }
    state = kmalloc_nolock(sizeof!(*state), 0, NUMA_NO_NODE);
    if (!state) {
    bpf_dynptr_set_null(ptr);
    return -ENOMEM;
    }
    state.offset = 0;
    state.size = U64_MAX; /* Don't restrict size, as file may change anyways */
    freader_init_from_file(&state.freader, core::ptr::null_mut(), 0, file, may_sleep);
    bpf_dynptr_init(ptr, state, BPF_DYNPTR_TYPE_FILE, 0, 0);
    bpf_dynptr_set_rdonly(ptr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_from_file(file: *mut file, flags: u32, ptr__uninit: *mut bpf_dynptr) -> __bpf_kfunc int {
    return make_file_dynptr(file, flags, false, ptr__uninit);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_from_file_sleepable(file: *mut file, flags: u32, ptr__uninit: *mut bpf_dynptr) -> c_int {
    return make_file_dynptr(file, flags, true, ptr__uninit);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_dynptr_file_discard(dynptr: *mut bpf_dynptr) -> __bpf_kfunc int {
    let mut ptr = dynptr;
    let mut df = ptr.data;
    if (!df) {
    return 0;
    }
    freader_cleanup(&df.freader);
    kfree_nolock(df);
    bpf_dynptr_set_null(ptr);
    return 0;
    }
//
// bpf_timer_cancel_async - try to deactivate a timer
// @timer:	bpf_timer to stop
//
// Returns:
//
// *  0 when the timer was not active
// *  1 when the timer was active
// * -1 when the timer is currently executing the callback function and
// cannot be stopped
// * -ECANCELED when the timer will be cancelled asynchronously
// * -ENOMEM when out of memory
// * -EINVAL when the timer was not initialized
// * -ENOENT when this kfunc is racing with timer deletion
//
#[no_mangle]
pub unsafe extern "C" fn bpf_timer_cancel_async(timer: *mut bpf_timer) -> __bpf_kfunc int {
    let mut async = timer;
pub static mut cb: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    cb = READ_ONCE(async.cb);
    if (!cb) {
    return -EINVAL;
    }
//
// Unlike hrtimer_start() it's ok to synchronously call
// hrtimer_try_to_cancel() when refcnt reached zero, but deferring to
// irq_work is not, since irq callback may execute after RCU GP and
// cb could be freed at that time. Check for refcnt zero for
// consistency.
//
    if (!refcount_inc_not_zero(&cb.refcnt)) {
    return -ENOENT;
    }
    if (!defer_timer_wq_op()) {
    let mut t = container_of!(cb, bpf_hrtimer, cb);
    ret = hrtimer_try_to_cancel(&t.timer);
    bpf_async_refcount_put(cb);
    return ret;
    } else {
    ret = bpf_async_schedule_op(cb, BPF_ASYNC_CANCEL, 0, 0);
    return ret ? ret : -ECANCELED;
    }
    }
    __bpf_kfunc_end_defs();
#[no_mangle]
unsafe extern "C" fn bpf_task_work_cancel_scheduled(irq_work: *mut irq_work) {
    let mut ctx = container_of!(irq_work, bpf_task_work_ctx, irq_work);
    bpf_task_work_cancel(ctx); /* this might put task_work callback's ref */
    bpf_task_work_ctx_put(ctx); /* and here we put map's own ref that was transferred to us */
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_work_cancel_and_free(val: *mut c_void) {
    let mut twk = val;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    enum bpf_task_work_state state;
    ctx = xchg(&twk.ctx, core::ptr::null_mut());
    if (!ctx) {
    return;
    }
    state = xchg(&ctx.state, BPF_TW_FREED);
    if (state == BPF_TW_SCHEDULED) {
// run in irq_work to avoid locks in NMI
    init_irq_work(&ctx.irq_work, bpf_task_work_cancel_scheduled);
    irq_work_queue(&ctx.irq_work);
    return;
    }
    bpf_task_work_ctx_put(ctx); /* put bpf map's ref */
    }
    BTF_KFUNCS_START(generic_btf_ids)

    BTF_ID_FLAGS(func, crash_kexec, KF_DESTRUCTIVE)

    BTF_ID_FLAGS(func, bpf_obj_new, KF_ACQUIRE | KF_RET_NULL | KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_obj_new_impl, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_percpu_obj_new, KF_ACQUIRE | KF_RET_NULL | KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_percpu_obj_new_impl, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_obj_drop, KF_RELEASE | KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_obj_drop_impl, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_percpu_obj_drop, KF_RELEASE | KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_percpu_obj_drop_impl, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_refcount_acquire,
    KF_ACQUIRE | KF_RET_NULL | KF_RCU | KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_refcount_acquire_impl,
    KF_ACQUIRE | KF_RET_NULL | KF_RCU | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_push_front, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_push_front_impl, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_push_back, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_push_back_impl, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_add, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_pop_front, KF_ACQUIRE | KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_pop_back, KF_ACQUIRE | KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_del, KF_ACQUIRE | KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_front, KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_back, KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_is_first, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_is_last, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_list_empty, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_task_acquire, KF_ACQUIRE | KF_RCU | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_task_release, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_rbtree_remove, KF_ACQUIRE | KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_add, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_add_impl, KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_first, KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_root, KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_left, KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_rbtree_right, KF_RET_NULL | KF_SPINLOCK_SAFE)

    BTF_ID_FLAGS(func, bpf_cgroup_acquire, KF_ACQUIRE | KF_RCU | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_cgroup_release, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_cgroup_ancestor, KF_ACQUIRE | KF_RCU | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_cgroup_from_id, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_task_under_cgroup, KF_RCU)
    BTF_ID_FLAGS(func, bpf_task_get_cgroup1, KF_ACQUIRE | KF_RCU | KF_RET_NULL)

    BTF_ID_FLAGS(func, bpf_task_from_pid, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_task_from_vpid, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_throw)

    BTF_ID_FLAGS(func, bpf_send_signal_task)

    BTF_ID_FLAGS(func, bpf_lookup_user_key, KF_ACQUIRE | KF_RET_NULL | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_lookup_system_key, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_key_put, KF_RELEASE)

    BTF_ID_FLAGS(func, bpf_verify_pkcs7_signature, KF_SLEEPABLE)

    BTF_ID_FLAGS(func, bpf_get_lowcore)

    BTF_KFUNCS_END(generic_btf_ids)
pub static mut btf_kfunc_id_set: usize = 0;
    BTF_ID_LIST(generic_dtor_ids)
    BTF_ID(struct, task_struct)
    BTF_ID(func, bpf_task_release_dtor)

    BTF_ID(struct, cgroup)
    BTF_ID(func, bpf_cgroup_release_dtor)

    BTF_KFUNCS_START(common_btf_ids)
    BTF_ID_FLAGS(func, bpf_cast_to_kern_ctx, KF_FASTCALL)
    BTF_ID_FLAGS(func, bpf_rdonly_cast, KF_FASTCALL)
    BTF_ID_FLAGS(func, bpf_rcu_read_lock)
    BTF_ID_FLAGS(func, bpf_rcu_read_unlock)
    BTF_ID_FLAGS(func, bpf_dynptr_slice, KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_dynptr_slice_rdwr, KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_num_new, KF_ITER_NEW | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_iter_num_next, KF_ITER_NEXT | KF_RET_NULL | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_iter_num_destroy, KF_ITER_DESTROY | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_iter_task_vma_new, KF_ITER_NEW | KF_RCU)
    BTF_ID_FLAGS(func, bpf_iter_task_vma_next, KF_ITER_NEXT | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_task_vma_destroy, KF_ITER_DESTROY)

    BTF_ID_FLAGS(func, bpf_iter_css_task_new, KF_ITER_NEW)
    BTF_ID_FLAGS(func, bpf_iter_css_task_next, KF_ITER_NEXT | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_css_task_destroy, KF_ITER_DESTROY)
    BTF_ID_FLAGS(func, bpf_iter_css_new, KF_ITER_NEW | KF_RCU_PROTECTED)
    BTF_ID_FLAGS(func, bpf_iter_css_next, KF_ITER_NEXT | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_css_destroy, KF_ITER_DESTROY)

    BTF_ID_FLAGS(func, bpf_iter_task_new, KF_ITER_NEW | KF_RCU_PROTECTED)
    BTF_ID_FLAGS(func, bpf_iter_task_next, KF_ITER_NEXT | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_task_destroy, KF_ITER_DESTROY)
    BTF_ID_FLAGS(func, bpf_dynptr_adjust)
    BTF_ID_FLAGS(func, bpf_dynptr_is_null)
    BTF_ID_FLAGS(func, bpf_dynptr_is_rdonly)
    BTF_ID_FLAGS(func, bpf_dynptr_size)
    BTF_ID_FLAGS(func, bpf_dynptr_clone)
    BTF_ID_FLAGS(func, bpf_dynptr_copy)
    BTF_ID_FLAGS(func, bpf_dynptr_memset)

    BTF_ID_FLAGS(func, bpf_modify_return_test_tp)

    BTF_ID_FLAGS(func, bpf_wq_init)
    BTF_ID_FLAGS(func, bpf_wq_set_callback, KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_wq_start)
    BTF_ID_FLAGS(func, bpf_preempt_disable)
    BTF_ID_FLAGS(func, bpf_preempt_enable)
    BTF_ID_FLAGS(func, bpf_iter_bits_new, KF_ITER_NEW)
    BTF_ID_FLAGS(func, bpf_iter_bits_next, KF_ITER_NEXT | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_iter_bits_destroy, KF_ITER_DESTROY)
    BTF_ID_FLAGS(func, bpf_copy_from_user_str, KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_copy_from_user_task_str, KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_get_kmem_cache)
    BTF_ID_FLAGS(func, bpf_iter_kmem_cache_new, KF_ITER_NEW | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_iter_kmem_cache_next, KF_ITER_NEXT | KF_RET_NULL | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_iter_kmem_cache_destroy, KF_ITER_DESTROY | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_local_irq_save)
    BTF_ID_FLAGS(func, bpf_local_irq_restore)

    BTF_ID_FLAGS(func, bpf_probe_read_user_dynptr)
    BTF_ID_FLAGS(func, bpf_probe_read_kernel_dynptr)
    BTF_ID_FLAGS(func, bpf_probe_read_user_str_dynptr)
    BTF_ID_FLAGS(func, bpf_probe_read_kernel_str_dynptr)
    BTF_ID_FLAGS(func, bpf_copy_from_user_dynptr, KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_copy_from_user_str_dynptr, KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_copy_from_user_task_dynptr, KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_copy_from_user_task_str_dynptr, KF_SLEEPABLE)

    BTF_ID_FLAGS(func, bpf_iter_dmabuf_new, KF_ITER_NEW | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_iter_dmabuf_next, KF_ITER_NEXT | KF_RET_NULL | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_iter_dmabuf_destroy, KF_ITER_DESTROY | KF_SLEEPABLE)

    BTF_ID_FLAGS(func, __bpf_trap)
    BTF_ID_FLAGS(func, bpf_strcmp);
    BTF_ID_FLAGS(func, bpf_strcasecmp);
    BTF_ID_FLAGS(func, bpf_strncasecmp);
    BTF_ID_FLAGS(func, bpf_strchr);
    BTF_ID_FLAGS(func, bpf_strchrnul);
    BTF_ID_FLAGS(func, bpf_strnchr);
    BTF_ID_FLAGS(func, bpf_strrchr);
    BTF_ID_FLAGS(func, bpf_strlen);
    BTF_ID_FLAGS(func, bpf_strnlen);
    BTF_ID_FLAGS(func, bpf_strspn);
    BTF_ID_FLAGS(func, bpf_strcspn);
    BTF_ID_FLAGS(func, bpf_strstr);
    BTF_ID_FLAGS(func, bpf_strcasestr);
    BTF_ID_FLAGS(func, bpf_strnstr);
    BTF_ID_FLAGS(func, bpf_strncasestr);

    BTF_ID_FLAGS(func, bpf_cgroup_read_xattr, KF_RCU)

    BTF_ID_FLAGS(func, bpf_stream_vprintk, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_stream_print_stack, KF_IMPLICIT_ARGS | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_task_work_schedule_signal, KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_task_work_schedule_resume, KF_IMPLICIT_ARGS)
    BTF_ID_FLAGS(func, bpf_dynptr_from_file)
    BTF_ID_FLAGS(func, bpf_dynptr_file_discard, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_timer_cancel_async)
    BTF_KFUNCS_END(common_btf_ids)
pub static mut btf_kfunc_id_set: usize = 0;
#[no_mangle]
unsafe extern "C" fn kfunc_init() -> c_int {
    let mut ret = 0;
pub static mut btf_id_dtor_kfunc: usize = 0;
    ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &generic_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &generic_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, &generic_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &generic_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &generic_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_CGROUP_SKB, &generic_kfunc_set);
    ret = ret ?: register_btf_id_dtor_kfuncs(generic_dtors,
    ARRAY_SIZE!(generic_dtors),
    THIS_MODULE);
    return ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC, &common_kfunc_set);
    }
    late_initcall!(kfunc_init);
// Get a pointer to dynptr data up to len bytes for read only access. If
// the dynptr doesn't have continuous data up to len bytes, return NULL.
//
    const void *__bpf_dynptr_data(const struct bpf_dynptr_kern *ptr, u64 len)
    {
    let mut p = ptr;
    return bpf_dynptr_slice(p, 0, core::ptr::null_mut(), len);
    }
// Get a pointer to dynptr data up to len bytes for read write access. If
// the dynptr doesn't have continuous data up to len bytes, or the dynptr
// is read only, return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn __bpf_dynptr_data_rw(ptr: *mut bpf_dynptr_kern, len: u64) -> *mut c_void {
    if (__bpf_dynptr_is_rdonly(ptr)) {
    return core::ptr::null_mut();
    }
    return __bpf_dynptr_data(ptr, len);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_map_free_internal_structs(map: *mut bpf_map, val: *mut c_void) {
    if (btf_record_has_field(map.record, BPF_TIMER)) {
    bpf_obj_free_timer(map.record, val);
    }
    if (btf_record_has_field(map.record, BPF_WORKQUEUE)) {
    bpf_obj_free_workqueue(map.record, val);
    }
    if (btf_record_has_field(map.record, BPF_TASK_WORK)) {
    bpf_obj_free_task_work(map.record, val);
    }
    }