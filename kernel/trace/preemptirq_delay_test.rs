//! Automatically rewritten from C to Rust
//! Source: kernel/trace/preemptirq_delay_test.c
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
// Preempt / IRQ disable delay thread to test latency tracers
//
// Copyright (C) 2018 Joel Fernandes (Google) <joel@joelfernandes.org>
//

pub static mut delay: ulong = 100;
    static char test_mode[12] = "irq";
pub static mut burst_size: uint = 1;
pub static mut cpu_affinity: int = 0;
    module_param_named!(delay, delay, ulong, 0444);
    module_param_string!(test_mode, test_mode, 12, 0444);
    module_param_named!(burst_size, burst_size, uint, 0444);
    module_param_named!(cpu_affinity, cpu_affinity, int, 0444);
    MODULE_PARM_DESC(delay, "Period in microseconds (100 us default)");
    MODULE_PARM_DESC(test_mode, "Mode of the test such as preempt, irq, or alternate (default irq)");
    MODULE_PARM_DESC(burst_size, "The size of a burst (default 1)");
    MODULE_PARM_DESC(cpu_affinity, "Cpu num test is running on");
pub static mut done: usize = 0;
#[no_mangle]
unsafe extern "C" fn busy_wait(time: c_ulong) {
    u64 start, end;
    start = trace_clock_local();
    do {
    end = trace_clock_local();
    if (kthread_should_stop()) {
    break;
    }
    } while ((end - start) < (time * 1000));
    }
#[no_mangle]
unsafe extern "C" fn irqoff_test() -> __always_inline void {
    let mut flags = 0;
    local_irq_save(flags);
    busy_wait(delay);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn preemptoff_test() -> __always_inline void {
    preempt_disable();
    busy_wait(delay);
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn execute_preemptirqtest(idx: c_int) {
    if (!strcmp(test_mode, "irq")) {
    irqoff_test();
    }

    else if (!strcmp(test_mode, "preempt")) {
    preemptoff_test();
    }
if true {
    if (idx % 2 == 0) {
    irqoff_test();
    }
    else {
    preemptoff_test();
    }
    }
    }

    static void preemptirqtest_##POSTFIX(int idx)	
    {						
    execute_preemptirqtest(idx);		
    }						
//
// We create 10 different functions, so that we can get 10 different
// backtraces.
//
    DECLARE_TESTFN(0)
    DECLARE_TESTFN(1)
    DECLARE_TESTFN(2)
    DECLARE_TESTFN(3)
    DECLARE_TESTFN(4)
    DECLARE_TESTFN(5)
    DECLARE_TESTFN(6)
    DECLARE_TESTFN(7)
    DECLARE_TESTFN(8)
    DECLARE_TESTFN(9)
    static void (*testfuncs[])(int)  = {
    preemptirqtest_0,
    preemptirqtest_1,
    preemptirqtest_2,
    preemptirqtest_3,
    preemptirqtest_4,
    preemptirqtest_5,
    preemptirqtest_6,
    preemptirqtest_7,
    preemptirqtest_8,
    preemptirqtest_9,
    };

#[no_mangle]
unsafe extern "C" fn preemptirq_delay_run(data: *mut c_void) -> c_int {
    let mut i = 0;
pub static mut s: c_int = 0;
    let mut cpu_mask;
    if (!alloc_cpumask_var(&cpu_mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    if (cpu_affinity > -1) {
pub static mut cpu: c_uint = 0;
    if (cpu >= nr_cpu_ids || !cpu_possible(cpu)) {
    pr_err!("cpu_affinity:%d, invalid CPU\n", cpu_affinity);
// goto;
    }
    cpumask_clear(cpu_mask);
    cpumask_set_cpu(cpu_affinity, cpu_mask);
    if (set_cpus_allowed_ptr(current, cpu_mask)) {
    pr_err!("cpu_affinity:%d, failed\n", cpu_affinity);
    }
    }
    for (i = 0; i < s; i++) {
    (testfuncs[i])(i);
    }
// label;
    complete(&done);
    set_current_state(TASK_INTERRUPTIBLE);
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    free_cpumask_var(cpu_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preemptirq_run_test() -> c_int {
pub static mut task: *mut c_void = core::ptr::null_mut();
    char task_name[50];
    init_completion(&done);
    snprintf(task_name, sizeof!(task_name), "%s_test", test_mode);
    task =  kthread_run(preemptirq_delay_run, core::ptr::null_mut(), task_name);
    if (IS_ERR(task)) {
    return PTR_ERR(task);
    }
    if (task) {
    wait_for_completion(&done);
    kthread_stop(task);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trigger_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut ret = 0;
    ret = preemptirq_run_test();
    if (ret) {
    return ret;
    }
    return count;
    }
    static struct kobj_attribute trigger_attribute =
    __ATTR(trigger, 0200, core::ptr::null_mut(), trigger_store);
    static struct attribute *attrs[] = {
    &trigger_attribute.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
pub static mut preemptirq_delay_kobj: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn preemptirq_delay_init() -> c_int {
    let mut retval = 0;
    retval = preemptirq_run_test();
    if (retval != 0) {
    return retval;
    }
    preemptirq_delay_kobj = kobject_create_and_add("preemptirq_delay_test",
    kernel_kobj);
    if (!preemptirq_delay_kobj) {
    return -ENOMEM;
    }
    retval = sysfs_create_group(preemptirq_delay_kobj, &attr_group);
    if (retval) {
    kobject_put(preemptirq_delay_kobj);
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn preemptirq_delay_exit()  {
    kobject_put(preemptirq_delay_kobj);
    }
    module_init!(preemptirq_delay_init)
    module_exit!(preemptirq_delay_exit)
    MODULE_DESCRIPTION("Preempt / IRQ disable delay thread to test latency tracers");
    MODULE_LICENSE("GPL v2");