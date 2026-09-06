//! Automatically rewritten from C to Rust
//! Source: kernel/locking/spinlock.c
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
// Copyright (2004) Linus Torvalds
//
// Author: Zwane Mwaikambo <zwane@fsmlabs.com>
//
// Copyright (2004, 2005) Ingo Molnar
//
// This file contains the spinlock/rwlock implementations for the
// SMP and the DEBUG_SPINLOCK cases. (UP-nondebug inlines them)
//
// Note that some architectures have special knowledge about the
// stack frames of these functions in their profile_pc. If you
// change anything significant here that could change the stack
// frame contact the architecture maintainers.
//

pub static mut struct mmiowb_state: usize = 0;
    EXPORT_PER_CPU_SYMBOL(__mmiowb_state);

//
// If lockdep is enabled then we use the non-preemption spin-ops
// even on CONFIG_PREEMPT, because lockdep assumes that interrupts are
// not re-enabled during lock-acquire (which the preempt-spin-ops do):
//

//
// The __lock_function inlines are taken from
// spinlock : include/linux/spinlock_api_smp.h
// rwlock   : include/linux/rwlock_api_smp.h
//

//
// Some architectures can relax in favour of the CPU owning the lock.
//

//
// We build the __lock_function inlines here. They are too large for
// inlining all over the place, but here is only one user per function
// which embeds them into the calling _lock_function below.
//
// This could be a long-held lock. We both prepare to spin for a long
// time (making _this_ CPU preemptible if possible), and we also signal
// towards that other CPU that it should break the lock ASAP.
//

    static void __lockfunc __raw_##op##_lock(locktype##_t *lock)		
    lock_ctx_op(lock)						
    {									
    for (;;) {							
    preempt_disable();					
    if (likely(do_raw_##op##_trylock(lock)))		 {
    break;						
    }
    preempt_enable();					
    
    arch_##op##_relax(&lock.raw_lock);			
    }								
    }									
    
    static unsigned long __lockfunc __raw_##op##_lock_irqsave(locktype##_t *lock) 
    lock_ctx_op(lock)						
    {									
    let mut flags = 0;						
    
    for (;;) {							
    preempt_disable();					
    local_irq_save(flags);					
    if (likely(do_raw_##op##_trylock(lock)))		 {
    break;						
    }
    local_irq_restore(flags);				
    preempt_enable();					
    
    arch_##op##_relax(&lock.raw_lock);			
    }								
    
    return flags;							
    }									
    
    static void __lockfunc __raw_##op##_lock_irq(locktype##_t *lock)	
    lock_ctx_op(lock)						
    {									
    _raw_##op##_lock_irqsave(lock);					
    }									
    
    static void __lockfunc __raw_##op##_lock_bh(locktype##_t *lock)		
    lock_ctx_op(lock)						
    {									
    let mut flags = 0;						
    
// */	
// Careful: we must exclude softirqs too, hence the	*/	
// irq-disabling. We use the generic preemption-aware	*/	
// function:						*/	
// 
    flags = _raw_##op##_lock_irqsave(lock);				
    local_bh_disable();						
    local_irq_restore(flags);					
    }									
//
// Build preemption-friendly versions of the following
// lock-spinning functions:
//
// __[spin|read|write]_lock()
// __[spin|read|write]_lock_irq()
// __[spin|read|write]_lock_irqsave()
// __[spin|read|write]_lock_bh()
//
    BUILD_LOCK_OPS(spin, raw_spinlock, __acquires);
// No rwlock_t variants for now, so just build this function by hand
#[no_mangle]
unsafe extern "C" fn __raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t) -> void __lockfunc {
    for (;;) {
    preempt_disable();
    local_interrupt_disable();
    if (likely(do_raw_spin_trylock(lock))) {
    break;
    }
    local_interrupt_enable();
    preempt_enable();
    arch_spin_relax(&lock.raw_lock);
    }
    }

    BUILD_LOCK_OPS(read, rwlock, __acquires_shared);
    BUILD_LOCK_OPS(write, rwlock, __acquires);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_trylock(lock: *mut raw_spinlock_t) -> noinline int __lockfunc {
    return __raw_spin_trylock(lock);
    }
    EXPORT_SYMBOL(_raw_spin_trylock);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> noinline int __lockfunc {
    return __raw_spin_trylock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_spin_trylock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_lock(lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> noinline unsigned long __lockfunc {
    return __raw_spin_lock_irqsave(lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock_irqsave);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock_irq(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_lock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_lock_irq_disable(lock);
    }
    EXPORT_SYMBOL_GPL(_raw_spin_lock_irq_disable);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock_bh(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_lock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_unlock(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_unlock(lock);
    }
    EXPORT_SYMBOL(_raw_spin_unlock);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong) -> noinline void __lockfunc {
    __raw_spin_unlock_irqrestore(lock, flags);
    }
    EXPORT_SYMBOL(_raw_spin_unlock_irqrestore);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_unlock_irq(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_unlock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_spin_unlock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_unlock_irq_enable(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_unlock_irq_enable(lock);
    }
    EXPORT_SYMBOL_GPL(_raw_spin_unlock_irq_enable);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_unlock_bh(lock: *mut raw_spinlock_t) -> noinline void __lockfunc {
    __raw_spin_unlock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_spin_unlock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_trylock(lock: *mut rwlock_t) -> noinline int __lockfunc {
    return __raw_read_trylock(lock);
    }
    EXPORT_SYMBOL(_raw_read_trylock);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_lock(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_lock(lock);
    }
    EXPORT_SYMBOL(_raw_read_lock);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_lock_irqsave(lock: *mut rwlock_t) -> noinline unsigned long __lockfunc {
    return __raw_read_lock_irqsave(lock);
    }
    EXPORT_SYMBOL(_raw_read_lock_irqsave);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_lock_irq(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_lock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_read_lock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_lock_bh(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_lock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_read_lock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_unlock(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_unlock(lock);
    }
    EXPORT_SYMBOL(_raw_read_unlock);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong) -> noinline void __lockfunc {
    __raw_read_unlock_irqrestore(lock, flags);
    }
    EXPORT_SYMBOL(_raw_read_unlock_irqrestore);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_unlock_irq(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_unlock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_read_unlock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_read_unlock_bh(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_read_unlock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_read_unlock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_trylock(lock: *mut rwlock_t) -> noinline int __lockfunc {
    return __raw_write_trylock(lock);
    }
    EXPORT_SYMBOL(_raw_write_trylock);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_lock(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_lock(lock);
    }
    EXPORT_SYMBOL(_raw_write_lock);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_lock_nested(lock: *mut rwlock_t, subclass: c_int) -> void __lockfunc {
    __raw_write_lock_nested(lock, subclass);
    }
    EXPORT_SYMBOL(_raw_write_lock_nested);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_lock_irqsave(lock: *mut rwlock_t) -> noinline unsigned long __lockfunc {
    return __raw_write_lock_irqsave(lock);
    }
    EXPORT_SYMBOL(_raw_write_lock_irqsave);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_lock_irq(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_lock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_write_lock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_lock_bh(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_lock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_write_lock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_unlock(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_unlock(lock);
    }
    EXPORT_SYMBOL(_raw_write_unlock);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_unlock_irqrestore(lock: *mut rwlock_t, flags: c_ulong) -> noinline void __lockfunc {
    __raw_write_unlock_irqrestore(lock, flags);
    }
    EXPORT_SYMBOL(_raw_write_unlock_irqrestore);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_unlock_irq(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_unlock_irq(lock);
    }
    EXPORT_SYMBOL(_raw_write_unlock_irq);

#[no_mangle]
pub unsafe extern "C" fn _raw_write_unlock_bh(lock: *mut rwlock_t) -> noinline void __lockfunc {
    __raw_write_unlock_bh(lock);
    }
    EXPORT_SYMBOL(_raw_write_unlock_bh);

#[no_mangle]
pub unsafe extern "C" fn _raw_spin_lock_nested(lock: *mut raw_spinlock_t, subclass: c_int) -> void __lockfunc {
    preempt_disable();
    spin_acquire(&lock.dep_map, subclass, 0, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock_nested);
    unsigned long __lockfunc _raw_spin_lock_irqsave_nested(raw_spinlock_t *lock,
    int subclass)
    {
    let mut flags = 0;
    local_irq_save(flags);
    preempt_disable();
    spin_acquire(&lock.dep_map, subclass, 0, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
    return flags;
    }
    EXPORT_SYMBOL(_raw_spin_lock_irqsave_nested);
    void __lockfunc _raw_spin_lock_nest_lock(raw_spinlock_t *lock, lockdep_map *nest_lock)
    {
    preempt_disable();
    spin_acquire_nest(&lock.dep_map, 0, 0, nest_lock, _RET_IP_);
    LOCK_CONTENDED(lock, do_raw_spin_trylock, do_raw_spin_lock);
    }
    EXPORT_SYMBOL(_raw_spin_lock_nest_lock);

#[no_mangle]
pub unsafe extern "C" fn in_lock_functions(addr: c_ulong) -> notrace int {
// Linker adds these: start and end of __lockfunc functions
    extern char __lock_text_start[], __lock_text_end[];
    return addr >= (unsigned long)__lock_text_start
    && addr < (unsigned long)__lock_text_end;
    }
    EXPORT_SYMBOL(in_lock_functions);

#[no_mangle]
pub unsafe extern "C" fn lockdep_assert_in_softirq_func() -> void notrace {
    lockdep_assert_in_softirq();
    }
    EXPORT_SYMBOL(lockdep_assert_in_softirq_func);