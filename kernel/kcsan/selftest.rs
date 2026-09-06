//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/selftest.c
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


// SPDX-License-Identifier: GPL-2.0
//
// KCSAN short boot-time selftests.
//
// Copyright (C) 2019, Google LLC.
//

pub const ITERS_PER_TEST: c_int = 2000;
//
// Test watchpoint encode and decode: check that encoding some access's info,
// and then subsequent decode preserves the access's info.
//
#[no_mangle]
unsafe extern "C" fn test_encode_decode() -> bool __init {
    let mut i = 0;
    while (i < ITERS_PER_TEST) {
pub static mut size: usize = 0;
pub static mut is_write: bool = false;
    let mut verif_masked_addr = 0;
    let mut encoded_watchpoint = 0;
    let mut verif_is_write = 0;
    let mut addr = 0;
    let mut verif_size = 0;
    get_random_bytes(&addr, sizeof!(addr));
    if (addr < PAGE_SIZE) {
    addr = PAGE_SIZE;
    }
    if (WARN_ON!(!check_encodable(addr, size))) {
    return false;
    }
    encoded_watchpoint = encode_watchpoint(addr, size, is_write);
// Check special watchpoints
    if (WARN_ON!(decode_watchpoint(INVALID_WATCHPOINT, &verif_masked_addr, &verif_size, &verif_is_write))) {
    return false;
    }
    if (WARN_ON!(decode_watchpoint(CONSUMED_WATCHPOINT, &verif_masked_addr, &verif_size, &verif_is_write))) {
    return false;
    }
// Check decoding watchpoint returns same data
    if (WARN_ON!(!decode_watchpoint(encoded_watchpoint, &verif_masked_addr, &verif_size, &verif_is_write))) {
    return false;
    }
    if (WARN_ON!(verif_masked_addr != (addr & WATCHPOINT_ADDR_MASK))) {
// goto;
    }
    if (WARN_ON!(verif_size != size)) {
// goto;
    }
    if (WARN_ON!(is_write != verif_is_write)) {
// goto;
    }
    continue;
// label;
    pr_err!("%s fail: %s %zu bytes @ %lx . encoded: %lx . %s %zu bytes @ %lx\n",
    __func__, is_write ? "write" : "read", size, addr, encoded_watchpoint,
    verif_is_write ? "write" : "read", verif_size, verif_masked_addr);
    return false;
    }
    return true;
    }
// Test access matching function.
#[no_mangle]
unsafe extern "C" fn test_matching_access() -> bool __init {
    if (WARN_ON!(!matching_access(10, 1, 10, 1))) {
    return false;
    }
    if (WARN_ON!(!matching_access(10, 2, 11, 1))) {
    return false;
    }
    if (WARN_ON!(!matching_access(10, 1, 9, 2))) {
    return false;
    }
    if (WARN_ON!(matching_access(10, 1, 11, 1))) {
    return false;
    }
    if (WARN_ON!(matching_access(9, 1, 10, 1))) {
    return false;
    }
//
// An access of size 0 could match another access, as demonstrated here.
// Rather than add more comparisons to 'matching_access()', which would
// end up in the fast-path for *all* checks, check_access() simply
// returns for all accesses of size 0.
//
    if (WARN_ON!(!matching_access(8, 8, 12, 0))) {
    return false;
    }
    return true;
    }
//
// Correct memory barrier instrumentation is critical to avoiding false
// positives: simple test to check at boot certain barriers are always properly
// instrumented. See kcsan_test for a more complete test.
//
pub static mut test_spinlock: usize = 0;
#[no_mangle]
unsafe extern "C" fn test_barrier() -> bool __init {

    let mut reorder_access = &current.kcsan_ctx.reorder_access;

    let mut reorder_access = core::ptr::null_mut();

pub static mut ret: bool = true;
pub static mut arch_spinlock: arch_spinlock_t = 0;
    let mut dummy;
    let mut test_var = 0;
    if (!reorder_access || !IS_ENABLED!(CONFIG_SMP)) {
    return true;
    }

    do {											
    reorder_access.type = (access_type) | KCSAN_ACCESS_SCOPED;			
    reorder_access.size = 1;							
    barrier;									
    if (reorder_access.size != 0) {						
    pr_err!("improperly instrumented type=(" #access_type "): " name "\n");	
    ret = false;								
    }										
    } while (0)

    kcsan_nestable_atomic_begin(); /* No watchpoints in called functions. */
    KCSAN_CHECK_READ_BARRIER(mb());
    KCSAN_CHECK_READ_BARRIER(rmb());
    KCSAN_CHECK_READ_BARRIER(smp_mb());
    KCSAN_CHECK_READ_BARRIER(smp_rmb());
    KCSAN_CHECK_READ_BARRIER(dma_rmb());
    KCSAN_CHECK_READ_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_READ_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_READ_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_READ_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_READ_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_READ_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_READ_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_READ_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_READ_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_READ_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_READ_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_READ_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_READ_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_WRITE_BARRIER(mb());
    KCSAN_CHECK_WRITE_BARRIER(wmb());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb());
    KCSAN_CHECK_WRITE_BARRIER(smp_wmb());
    KCSAN_CHECK_WRITE_BARRIER(dma_wmb());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_WRITE_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_WRITE_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_WRITE_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_WRITE_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_WRITE_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_WRITE_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_WRITE_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_WRITE_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_WRITE_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_RW_BARRIER(mb());
    KCSAN_CHECK_RW_BARRIER(wmb());
    KCSAN_CHECK_RW_BARRIER(rmb());
    KCSAN_CHECK_RW_BARRIER(smp_mb());
    KCSAN_CHECK_RW_BARRIER(smp_wmb());
    KCSAN_CHECK_RW_BARRIER(smp_rmb());
    KCSAN_CHECK_RW_BARRIER(dma_wmb());
    KCSAN_CHECK_RW_BARRIER(dma_rmb());
    KCSAN_CHECK_RW_BARRIER(smp_mb__before_atomic());
    KCSAN_CHECK_RW_BARRIER(smp_mb__after_atomic());
    KCSAN_CHECK_RW_BARRIER(smp_mb__after_spinlock());
    KCSAN_CHECK_RW_BARRIER(smp_store_mb(test_var, 0));
    KCSAN_CHECK_RW_BARRIER(smp_store_release(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(xchg(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(xchg_release(&test_var, 0));
    KCSAN_CHECK_RW_BARRIER(cmpxchg(&test_var, 0,  0));
    KCSAN_CHECK_RW_BARRIER(cmpxchg_release(&test_var, 0,  0));
    KCSAN_CHECK_RW_BARRIER(atomic_set_release(&dummy, 0));
    KCSAN_CHECK_RW_BARRIER(atomic_add_return(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_add_return_release(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_fetch_add(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(atomic_fetch_add_release(1, &dummy));
    KCSAN_CHECK_RW_BARRIER(test_and_set_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(test_and_clear_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(test_and_change_bit(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(clear_bit_unlock(0, &test_var));
    KCSAN_CHECK_RW_BARRIER(__clear_bit_unlock(0, &test_var));
    arch_spin_lock(&arch_spinlock);
    KCSAN_CHECK_RW_BARRIER(arch_spin_unlock(&arch_spinlock));
    spin_lock(&test_spinlock);
    KCSAN_CHECK_RW_BARRIER(spin_unlock(&test_spinlock));
    KCSAN_CHECK_RW_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    KCSAN_CHECK_READ_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    KCSAN_CHECK_WRITE_BARRIER(xor_unlock_is_negative_byte(1, &test_var));
    kcsan_nestable_atomic_end();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kcsan_selftest() -> c_int {
pub static mut passed: c_int = 0;
pub static mut total: c_int = 0;

    do {                                                                   
    total += 1;                                                       
    if (do_test())                                                  {
    passed += 1;                                              
    }
    else {
    pr_err!("selftest: " #do_test " failed");               
    }
    } while (0)
    RUN_TEST(test_encode_decode);
    RUN_TEST(test_matching_access);
    RUN_TEST(test_barrier);
    pr_info!("selftest: %d/%d tests passed\n", passed, total);
    if (passed != total) {
    panic("selftests failed");
    }
    return 0;
    }
    postcore_initcall!(kcsan_selftest);