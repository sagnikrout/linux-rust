//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/cpumask.c
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
// Copyright (c) 2023 Meta, Inc

//
// struct bpf_cpumask - refcounted BPF cpumask wrapper structure
// @cpumask:	The actual cpumask embedded in the struct.
// @usage:	Object reference counter. When the refcount goes to 0, the
// memory is released back to the BPF allocator, which provides
// RCU safety.
//
// Note that we explicitly embed a cpumask_t rather than a cpumask_var_t.  This
// is done to avoid confusing the verifier due to the typedef of cpumask_var_t
// changing depending on whether CONFIG_CPUMASK_OFFSTACK is defined or not. See
// the details in <linux/cpumask.h>. The consequence is that this structure is
// likely a bit larger than it needs to be when CONFIG_CPUMASK_OFFSTACK is
// defined due to embedding the whole NR_CPUS-size bitmap, but the extra memory
// overhead is minimal. For the more typical case of CONFIG_CPUMASK_OFFSTACK
// not being defined, the structure is the same size regardless.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_cpumask {
    pub cpumask: cpumask_t,
    pub usage: refcount_t,
}

pub static mut bpf_cpumask_ma: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpu_valid(cpu: u32) -> bool {
    return cpu < nr_cpu_ids;
    }
    __bpf_kfunc_start_defs();
//
// bpf_cpumask_create() - Create a mutable BPF cpumask.
//
// Allocates a cpumask that can be queried, mutated, acquired, and released by
// a BPF program. The cpumask returned by this function must either be embedded
// in a map as a kptr, or freed with bpf_cpumask_release().
//
// bpf_cpumask_create() allocates memory using the BPF memory allocator, and
// will not block. It may return NULL if no memory is available.
//
// Return:
// * A pointer to a new struct bpf_cpumask instance on success.
// * NULL if the BPF memory allocator is out of memory.
//
    __bpf_kfunc struct bpf_cpumask *bpf_cpumask_create(void)
    {
pub static mut cpumask: *mut c_void = core::ptr::null_mut();
// cpumask must be the first element so struct bpf_cpumask be cast to struct cpumask.
    BUILD_BUG_ON!(offsetof(bpf_cpumask, cpumask) != 0);
    cpumask = bpf_mem_cache_alloc(&bpf_cpumask_ma);
    if (!cpumask) {
    return core::ptr::null_mut();
    }
    memset(cpumask, 0, sizeof!(*cpumask));
    refcount_set(&cpumask.usage, 1);
    return cpumask;
    }
//
// bpf_cpumask_acquire() - Acquire a reference to a BPF cpumask.
// @cpumask: The BPF cpumask being acquired. The cpumask must be a trusted
// pointer.
//
// Acquires a reference to a BPF cpumask. The cpumask returned by this function
// must either be embedded in a map as a kptr, or freed with
// bpf_cpumask_release().
//
// Return:
// * The struct bpf_cpumask pointer passed to the function.
//
    __bpf_kfunc struct bpf_cpumask *bpf_cpumask_acquire(bpf_cpumask *cpumask)
    {
    refcount_inc(&cpumask.usage);
    return cpumask;
    }
//
// bpf_cpumask_release() - Release a previously acquired BPF cpumask.
// @cpumask: The cpumask being released.
//
// Releases a previously acquired reference to a BPF cpumask. When the final
// reference of the BPF cpumask has been released, it is subsequently freed in
// an RCU callback in the BPF memory allocator.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_release(cpumask: *mut bpf_cpumask) -> __bpf_kfunc void {
    if (!refcount_dec_and_test(&cpumask.usage)) {
    return;
    }
    bpf_mem_cache_free_rcu(&bpf_cpumask_ma, cpumask);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_release_dtor(cpumask: *mut c_void) -> __bpf_kfunc void {
    bpf_cpumask_release(cpumask);
    }
    CFI_NOSEAL(bpf_cpumask_release_dtor);
//
// bpf_cpumask_first() - Get the index of the first nonzero bit in the cpumask.
// @cpumask: The cpumask being queried.
//
// Find the index of the first nonzero bit of the cpumask. A struct bpf_cpumask
// pointer may be safely passed to this function.
//
// Return:
// * The index of the first nonzero bit in the struct cpumask.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_first(cpumask: *const cpumask) -> __bpf_kfunc u32 {
    return cpumask_first(cpumask);
    }
//
// bpf_cpumask_first_zero() - Get the index of the first unset bit in the
// cpumask.
// @cpumask: The cpumask being queried.
//
// Find the index of the first unset bit of the cpumask. A struct bpf_cpumask
// pointer may be safely passed to this function.
//
// Return:
// * The index of the first zero bit in the struct cpumask.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> __bpf_kfunc u32 {
    return cpumask_first_zero(cpumask);
    }
//
// bpf_cpumask_first_and() - Return the index of the first nonzero bit from the
// AND of two cpumasks.
// @src1: The first cpumask.
// @src2: The second cpumask.
//
// Find the index of the first nonzero bit of the AND of two cpumasks.
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
// Return:
// * The index of the first bit that is nonzero in both cpumask instances.
//
    __bpf_kfunc u32 bpf_cpumask_first_and(const struct cpumask *src1,
    const struct cpumask *src2)
    {
    return cpumask_first_and(src1, src2);
    }
//
// bpf_cpumask_set_cpu() - Set a bit for a CPU in a BPF cpumask.
// @cpu: The CPU to be set in the cpumask.
// @cpumask: The BPF cpumask in which a bit is being set.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> __bpf_kfunc void {
    if (!cpu_valid(cpu)) {
    return;
    }
    cpumask_set_cpu(cpu, cpumask);
    }
//
// bpf_cpumask_clear_cpu() - Clear a bit for a CPU in a BPF cpumask.
// @cpu: The CPU to be cleared from the cpumask.
// @cpumask: The BPF cpumask in which a bit is being cleared.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> __bpf_kfunc void {
    if (!cpu_valid(cpu)) {
    return;
    }
    cpumask_clear_cpu(cpu, cpumask);
    }
//
// bpf_cpumask_test_cpu() - Test whether a CPU is set in a cpumask.
// @cpu: The CPU being queried for.
// @cpumask: The cpumask being queried for containing a CPU.
//
// Return:
// * true  - @cpu is set in the cpumask
// * false - @cpu was not set in the cpumask, or @cpu is an invalid cpu.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> __bpf_kfunc bool {
    if (!cpu_valid(cpu)) {
    return false;
    }
    return cpumask_test_cpu(cpu, cpumask);
    }
//
// bpf_cpumask_test_and_set_cpu() - Atomically test and set a CPU in a BPF cpumask.
// @cpu: The CPU being set and queried for.
// @cpumask: The BPF cpumask being set and queried for containing a CPU.
//
// Return:
// * true  - @cpu is set in the cpumask
// * false - @cpu was not set in the cpumask, or @cpu is invalid.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_test_and_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> __bpf_kfunc bool {
    if (!cpu_valid(cpu)) {
    return false;
    }
    return cpumask_test_and_set_cpu(cpu, cpumask);
    }
//
// bpf_cpumask_test_and_clear_cpu() - Atomically test and clear a CPU in a BPF
// cpumask.
// @cpu: The CPU being cleared and queried for.
// @cpumask: The BPF cpumask being cleared and queried for containing a CPU.
//
// Return:
// * true  - @cpu is set in the cpumask
// * false - @cpu was not set in the cpumask, or @cpu is invalid.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_test_and_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> __bpf_kfunc bool {
    if (!cpu_valid(cpu)) {
    return false;
    }
    return cpumask_test_and_clear_cpu(cpu, cpumask);
    }
//
// bpf_cpumask_setall() - Set all of the bits in a BPF cpumask.
// @cpumask: The BPF cpumask having all of its bits set.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_setall(cpumask: *mut bpf_cpumask) -> __bpf_kfunc void {
    cpumask_setall(cpumask);
    }
//
// bpf_cpumask_clear() - Clear all of the bits in a BPF cpumask.
// @cpumask: The BPF cpumask being cleared.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_clear(cpumask: *mut bpf_cpumask) -> __bpf_kfunc void {
    cpumask_clear(cpumask);
    }
//
// bpf_cpumask_and() - AND two cpumasks and store the result.
// @dst: The BPF cpumask where the result is being stored.
// @src1: The first input.
// @src2: The second input.
//
// Return:
// * true  - @dst has at least one bit set following the operation
// * false - @dst is empty following the operation
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
    __bpf_kfunc bool bpf_cpumask_and(bpf_cpumask *dst,
    const struct cpumask *src1,
    const struct cpumask *src2)
    {
    return cpumask_and(dst, src1, src2);
    }
//
// bpf_cpumask_or() - OR two cpumasks and store the result.
// @dst: The BPF cpumask where the result is being stored.
// @src1: The first input.
// @src2: The second input.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
    __bpf_kfunc void bpf_cpumask_or(bpf_cpumask *dst,
    const struct cpumask *src1,
    const struct cpumask *src2)
    {
    cpumask_or(dst, src1, src2);
    }
//
// bpf_cpumask_xor() - XOR two cpumasks and store the result.
// @dst: The BPF cpumask where the result is being stored.
// @src1: The first input.
// @src2: The second input.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
    __bpf_kfunc void bpf_cpumask_xor(bpf_cpumask *dst,
    const struct cpumask *src1,
    const struct cpumask *src2)
    {
    cpumask_xor(dst, src1, src2);
    }
//
// bpf_cpumask_equal() - Check two cpumasks for equality.
// @src1: The first input.
// @src2: The second input.
//
// Return:
// * true   - @src1 and @src2 have the same bits set.
// * false  - @src1 and @src2 differ in at least one bit.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_equal(src1: *const cpumask, src2: *const cpumask) -> __bpf_kfunc bool {
    return cpumask_equal(src1, src2);
    }
//
// bpf_cpumask_intersects() - Check two cpumasks for overlap.
// @src1: The first input.
// @src2: The second input.
//
// Return:
// * true   - @src1 and @src2 have at least one of the same bits set.
// * false  - @src1 and @src2 don't have any of the same bits set.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_intersects(src1: *const cpumask, src2: *const cpumask) -> __bpf_kfunc bool {
    return cpumask_intersects(src1, src2);
    }
//
// bpf_cpumask_subset() - Check if a cpumask is a subset of another.
// @src1: The first cpumask being checked as a subset.
// @src2: The second cpumask being checked as a superset.
//
// Return:
// * true   - All of the bits of @src1 are set in @src2.
// * false  - At least one bit in @src1 is not set in @src2.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_subset(src1: *const cpumask, src2: *const cpumask) -> __bpf_kfunc bool {
    return cpumask_subset(src1, src2);
    }
//
// bpf_cpumask_empty() - Check if a cpumask is empty.
// @cpumask: The cpumask being checked.
//
// Return:
// * true   - None of the bits in @cpumask are set.
// * false  - At least one bit in @cpumask is set.
//
// A struct bpf_cpumask pointer may be safely passed to @cpumask.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_empty(cpumask: *const cpumask) -> __bpf_kfunc bool {
    return cpumask_empty(cpumask);
    }
//
// bpf_cpumask_full() - Check if a cpumask has all bits set.
// @cpumask: The cpumask being checked.
//
// Return:
// * true   - All of the bits in @cpumask are set.
// * false  - At least one bit in @cpumask is cleared.
//
// A struct bpf_cpumask pointer may be safely passed to @cpumask.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_full(cpumask: *const cpumask) -> __bpf_kfunc bool {
    return cpumask_full(cpumask);
    }
//
// bpf_cpumask_copy() - Copy the contents of a cpumask into a BPF cpumask.
// @dst: The BPF cpumask being copied into.
// @src: The cpumask being copied.
//
// A struct bpf_cpumask pointer may be safely passed to @src.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_copy(dst: *mut bpf_cpumask, src: *const cpumask) -> __bpf_kfunc void {
    cpumask_copy(dst, src);
    }
//
// bpf_cpumask_any_distribute() - Return a random set CPU from a cpumask.
// @cpumask: The cpumask being queried.
//
// Return:
// * A random set bit within [0, num_cpus) if at least one bit is set.
// * >= num_cpus if no bit is set.
//
// A struct bpf_cpumask pointer may be safely passed to @src.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_any_distribute(cpumask: *const cpumask) -> __bpf_kfunc u32 {
    return cpumask_any_distribute(cpumask);
    }
//
// bpf_cpumask_any_and_distribute() - Return a random set CPU from the AND of
// two cpumasks.
// @src1: The first cpumask.
// @src2: The second cpumask.
//
// Return:
// * A random set bit within [0, num_cpus) from the AND of two cpumasks, if at
// least one bit is set.
// * >= num_cpus if no bit is set.
//
// struct bpf_cpumask pointers may be safely passed to @src1 and @src2.
//
    __bpf_kfunc u32 bpf_cpumask_any_and_distribute(const struct cpumask *src1,
    const struct cpumask *src2)
    {
    return cpumask_any_and_distribute(src1, src2);
    }
//
// bpf_cpumask_weight() - Return the number of bits in @cpumask.
// @cpumask: The cpumask being queried.
//
// Count the number of set bits in the given cpumask.
//
// Return:
// * The number of bits set in the mask.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_weight(cpumask: *const cpumask) -> __bpf_kfunc u32 {
    return cpumask_weight(cpumask);
    }
//
// bpf_cpumask_populate() - Populate the CPU mask from the contents of
// a BPF memory region.
//
// @cpumask: The cpumask being populated.
// @src: The BPF memory holding the bit pattern.
// @src__sz: Length of the BPF memory region in bytes.
//
// Return:
// * 0 if the struct bpf_cpumask * instance was populated successfully.
// * -EACCES if the memory region is too small to populate the cpumask.
// * -EINVAL if the memory region is not aligned to the size of a long
// and the architecture does not support efficient unaligned accesses.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_cpumask_populate(cpumask: *mut bpf_cpumask, src: *mut c_void, src__sz: usize) -> __bpf_kfunc int {
pub static mut source: c_ulong = 0;
// The memory region must be large enough to populate the entire CPU mask.
    if (src__sz < bitmap_size(nr_cpu_ids)) {
    return -EACCES;
    }
// If avoiding unaligned accesses, the input region must be aligned to the nearest long.
    if (!IS_ENABLED!(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS) &&
    !IS_ALIGNED(source, sizeof!(long))) {
    return -EINVAL;
    }
    bitmap_copy(cpumask_bits(&cpumask.cpumask), src, nr_cpu_ids);
    return 0;
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(cpumask_kfunc_btf_ids)
    BTF_ID_FLAGS(func, bpf_cpumask_create, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_cpumask_release, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_cpumask_acquire, KF_ACQUIRE)
    BTF_ID_FLAGS(func, bpf_cpumask_first, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_first_zero, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_first_and, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_set_cpu, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_clear_cpu, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_test_cpu, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_test_and_set_cpu, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_test_and_clear_cpu, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_setall, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_clear, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_and, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_or, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_xor, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_equal, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_intersects, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_subset, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_empty, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_full, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_copy, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_any_distribute, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_any_and_distribute, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_weight, KF_RCU)
    BTF_ID_FLAGS(func, bpf_cpumask_populate, KF_RCU)
    BTF_KFUNCS_END(cpumask_kfunc_btf_ids)
pub static mut btf_kfunc_id_set: usize = 0;
    BTF_ID_LIST(cpumask_dtor_ids)
    BTF_ID(struct, bpf_cpumask)
    BTF_ID(func, bpf_cpumask_release_dtor)
#[no_mangle]
unsafe extern "C" fn cpumask_kfunc_init() -> c_int {
    let mut ret = 0;
pub static mut btf_id_dtor_kfunc: usize = 0;
    ret = bpf_mem_alloc_init(&bpf_cpumask_ma, sizeof!(bpf_cpumask), false);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &cpumask_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &cpumask_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &cpumask_kfunc_set);
    return  ret ?: register_btf_id_dtor_kfuncs(cpumask_dtors,
    ARRAY_SIZE!(cpumask_dtors),
    THIS_MODULE);
    }
    late_initcall!(cpumask_kfunc_init);