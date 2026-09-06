//! Automatically rewritten from C to Rust
//! Source: kernel/printk/printk_ringbuffer_kunit_test.c
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
// This KUnit tests the data integrity of the lockless printk_ringbuffer.
// From multiple CPUs it writes messages of varying length and content while
// a reader validates the correctness of the messages.
//
// IMPORTANT: The more CPUs you can use for this KUnit, the better!
//
// The test works by starting "num_online_cpus() - 1" writer threads, each
// pinned to their own CPU. Each writer thread loops, writing data of varying
// length into a printk_ringbuffer as fast as possible. The data content is
// an embedded data struct followed by string content repeating the byte:
//
// 'A' + CPUID
//
// The reader is running on the remaining online CPU, or if there is only one
// CPU on the same as the writer.
// It ensures that the embedded struct content is consistent with the string
// and that the string * is terminated and is composed of the same repeating
// byte as its first byte.
//
// Because the threads are running in such tight loops, they will call
// cond_resched() from time to time so the system stays functional.
//
// If the reader encounters an error, the test is aborted and some
// information about the error is reported.
// The runtime of the test can be configured with the runtime_ms module parameter.
//
// Note that the test is performed on a separate printk_ringbuffer instance
// and not the instance used by printk().
//
pub static mut runtime_ms: unsigned long = 0;
    module_param!(runtime_ms, ulong, 0400);
// test data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prbtest_rbdata {
    pub size: c_uint,
    pub __counted_by(size): char text[],
}

pub const MAX_RBDATA_TEXT_SIZE: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prbtest_data {
    pub test: *mut kunit,
    pub ringbuffer: *mut printk_ringbuffer,
// used by writers to signal reader of new records
    pub new_record_wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prbtest_thread_data {
    pub num: c_ulong,
    pub test_data: *mut prbtest_data,
}

#[no_mangle]
unsafe extern "C" fn prbtest_fail_record(test: *mut kunit, dat: *const prbtest_rbdata, seq: u64) {
    let mut len = 0;
    len = dat.size - 1;
    KUNIT_FAIL(test, "BAD RECORD: seq=%llu size=%u text=%.*s\n",
    seq, dat.size,
    len < MAX_RBDATA_TEXT_SIZE ? len : -1,
    len < MAX_RBDATA_TEXT_SIZE ? dat.text : "<invalid>");
    }
#[no_mangle]
unsafe extern "C" fn prbtest_check_data(dat: *const prbtest_rbdata) -> bool {
    let mut len = 0;
// Sane size? At least one character + trailing '\0'
    if (dat.size < 2 || dat.size > MAX_RBDATA_TEXT_SIZE) {
    return false;
    }
    len = dat.size - 1;
    if (dat.text[len] != '\0') {
    return false;
    }
// String repeats with the same character?
    while (len--) {
    if (dat.text[len] != dat.text[0]) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn prbtest_writer(data: *mut c_void) -> c_int {
    let mut tr = data;
pub static mut text_id: c_char = 0;
pub static mut e: usize = 0;
pub static mut dat: *mut c_void = core::ptr::null_mut();
    u32 record_size, text_size;
pub static mut count: c_ulong = 0;
pub static mut r: usize = 0;
    kunit_info(tr.test_data.test, "start thread %03lu (writer)\n", tr.num);
    for (;;) {
// ensure at least 1 character + trailing '\0'
    text_size = get_random_u32_inclusive(2, MAX_RBDATA_TEXT_SIZE);
    if (WARN_ON_ONCE!(text_size < 2)) {
    text_size = 2;
    }
    if (WARN_ON_ONCE!(text_size > MAX_RBDATA_TEXT_SIZE)) {
    text_size = MAX_RBDATA_TEXT_SIZE;
    }
    record_size = sizeof!(prbtest_rbdata) + text_size;
    WARN_ON_ONCE!(record_size > MAX_PRB_RECORD_SIZE);
// specify the text sizes for reservation
    prb_rec_init_wr(&r, record_size);
//
// Reservation can fail if:
//
// - No free descriptor is available.
// - The buffer is full, and the oldest record is reserved
// but not yet committed.
//
// It actually happens in this test because all CPUs are trying
// to write an unbounded number of messages in a tight loop.
// These failures are intentionally ignored because this test
// focuses on races, ringbuffer consistency, and pushing system
// usability limits.
//
    if (prb_reserve(&e, tr.test_data.ringbuffer, &r)) {
    r.info.text_len = record_size;
    dat = r.text_buf;
    dat.size = text_size;
    memset(dat.text, text_id, text_size - 1);
    dat.text[text_size - 1] = '\0';
    prb_commit(&e);
    wake_up_interruptible(&tr.test_data.new_record_wait);
    }
    if ((count++ & 0x3fff) == 0) {
    cond_resched();
    }
    if (kthread_should_stop()) {
    break;
    }
    }
    kunit_info(tr.test_data.test, "end thread %03lu: wrote=%lu\n", tr.num, count);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prbtest_wakeup_timer {
    pub timer: timer_list,
    pub task: *mut task_struct,
}

#[no_mangle]
unsafe extern "C" fn prbtest_wakeup_callback(timer: *mut timer_list) {
    let mut wakeup = timer_container_of(wakeup, timer, timer);
    set_tsk_thread_flag(wakeup.task, TIF_NOTIFY_SIGNAL);
    wake_up_process(wakeup.task);
    }
#[no_mangle]
unsafe extern "C" fn prbtest_reader(test_data: *mut prbtest_data, timeout_ms: c_ulong) -> c_int {
pub static mut wakeup: usize = 0;
    char text_buf[MAX_PRB_RECORD_SIZE];
pub static mut count: c_ulong = 0;
pub static mut info: usize = 0;
pub static mut r: usize = 0;
pub static mut seq: u64 = 0;
    wakeup.task = current;
    timer_setup_on_stack(&wakeup.timer, prbtest_wakeup_callback, 0);
    mod_timer(&wakeup.timer, jiffies + msecs_to_jiffies(timeout_ms));
    prb_rec_init_rd(&r, &info, text_buf, sizeof!(text_buf));
    kunit_info(test_data.test, "start reader\n");
    while (!wait_event_interruptible(test_data.new_record_wait,
    prb_read_valid(test_data.ringbuffer, seq, &r))) {
// check/track the sequence
    if (info.seq < seq) {
    KUNIT_FAIL(test_data.test, "BAD SEQ READ: request=%llu read=%llu\n",
    seq, info.seq);
    }
    if (!prbtest_check_data(r.text_buf)) {
    prbtest_fail_record(test_data.test,
    r.text_buf, info.seq);
    }
    if ((count++ & 0x3fff) == 0) {
    cond_resched();
    }
    seq = info.seq + 1;
    }
    timer_delete_sync(&wakeup.timer);
    timer_destroy_on_stack(&wakeup.timer);
    kunit_info(test_data.test, "end reader: read=%lu seq=%llu\n", count, info.seq);
    return 0;
    }
    KUNIT_DEFINE_ACTION_WRAPPER(prbtest_cpumask_cleanup, free_cpumask_var, cpumask *);
    KUNIT_DEFINE_ACTION_WRAPPER(prbtest_kthread_cleanup, kthread_stop, task_struct *);
#[no_mangle]
unsafe extern "C" fn prbtest_add_cpumask_cleanup(test: *mut kunit, mask: cpumask_var_t) {
    let mut err = 0;
    err = kunit_add_action_or_reset(test, prbtest_cpumask_cleanup, mask);
    KUNIT_ASSERT_EQ(test, err, 0);
    }
#[no_mangle]
unsafe extern "C" fn prbtest_add_kthread_cleanup(test: *mut kunit, kthread: *mut task_struct) {
    let mut err = 0;
    err = kunit_add_action_or_reset(test, prbtest_kthread_cleanup, kthread);
    KUNIT_ASSERT_EQ(test, err, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn prbtest_prb_reinit(rb: *mut printk_ringbuffer) {
    prb_init(rb, rb.text_data_ring.data, rb.text_data_ring.size_bits, rb.desc_ring.descs,
    rb.desc_ring.count_bits, rb.desc_ring.infos);
    }
#[no_mangle]
unsafe extern "C" fn test_readerwriter(test: *mut kunit) {
// Equivalent to CONFIG_LOG_BUF_SHIFT=13
pub static mut test_rb: usize = 0;
pub static mut thread_data: *mut c_void = core::ptr::null_mut();
pub static mut test_data: *mut c_void = core::ptr::null_mut();
pub static mut thread: *mut c_void = core::ptr::null_mut();
    let mut test_cpus;
    let mut cpu = 0;
    let mut reader_cpu = 0;
    KUNIT_ASSERT_TRUE(test, alloc_cpumask_var(&test_cpus, GFP_KERNEL));
    prbtest_add_cpumask_cleanup(test, test_cpus);
    cpus_read_lock();
//
// Failure of KUNIT_ASSERT() kills the current task
// so it can not be called while the CPU hotplug lock is held.
// Instead use a snapshot of the online CPUs.
// If they change during test execution it is unfortunate but not a grave error.
//
    cpumask_copy(test_cpus, cpu_online_mask);
    cpus_read_unlock();
// One CPU is for the reader, all others are writers
    reader_cpu = cpumask_first(test_cpus);
    if (cpumask_weight(test_cpus) == 1) {
    kunit_warn(test, "more than one CPU is recommended");
    }
    else {
    cpumask_clear_cpu(reader_cpu, test_cpus);
    }
// KUnit test can get restarted more times.
    prbtest_prb_reinit(&test_rb);
    test_data = kunit_kmalloc(test, sizeof!(*test_data), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, test_data);
    test_data.test = test;
    test_data.ringbuffer = &test_rb;
    init_waitqueue_head(&test_data.new_record_wait);
    kunit_info(test, "running for %lu ms\n", runtime_ms);
    for_each_cpu(cpu, test_cpus) {
    thread_data = kunit_kmalloc(test, sizeof!(*thread_data), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, thread_data);
    thread_data.test_data = test_data;
    thread_data.num = cpu;
    thread = kthread_run_on_cpu(prbtest_writer, thread_data, cpu,
    "prbtest writer %u");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, thread);
    prbtest_add_kthread_cleanup(test, thread);
    }
    kunit_info(test, "starting test\n");
    set_cpus_allowed_ptr(current, cpumask_of(reader_cpu));
    prbtest_reader(test_data, runtime_ms);
    kunit_info(test, "completed test\n");
    }
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suite(prb_test_suite);
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
    MODULE_AUTHOR("John Ogness <john.ogness@linutronix.de>");
    MODULE_DESCRIPTION("printk_ringbuffer KUnit test");
    MODULE_LICENSE("GPL");