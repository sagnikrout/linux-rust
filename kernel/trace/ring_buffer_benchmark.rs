//! Automatically rewritten from C to Rust
//! Source: kernel/trace/ring_buffer_benchmark.c
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
// ring buffer tester and benchmark
//
// Copyright (C) 2009 Steven Rostedt <srostedt@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_page {
    pub ts: u64,
    pub commit: local_t,
    pub data: [c_char; 4080],
}

// run time and sleep time in seconds

pub const SLEEP_TIME: c_int = 10;
// number of events for writer to wake up the reader
pub static mut wakeup_interval: int = 100;
    static int reader_finish;
pub static mut read_start: usize = 0;
pub static mut read_done: usize = 0;
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut producer: *mut c_void = core::ptr::null_mut();
pub static mut consumer: *mut c_void = core::ptr::null_mut();
    static unsigned long read;
    static unsigned int disable_reader;
    module_param!(disable_reader, uint, 0644);
    MODULE_PARM_DESC(disable_reader, "only run producer");
pub static mut write_iteration: unsigned int = 50;
    module_param!(write_iteration, uint, 0644);
    MODULE_PARM_DESC(write_iteration, "# of writes between timestamp readings");
pub static mut producer_nice: int = 0;
pub static mut consumer_nice: int = 0;
    static int producer_fifo;
    static int consumer_fifo;
    module_param!(producer_nice, int, 0644);
    MODULE_PARM_DESC(producer_nice, "nice prio for producer");
    module_param!(consumer_nice, int, 0644);
    MODULE_PARM_DESC(consumer_nice, "nice prio for consumer");
    module_param!(producer_fifo, int, 0644);
    MODULE_PARM_DESC(producer_fifo, "use fifo for producer: 0 - disabled, 1 - low prio, 2 - fifo");
    module_param!(consumer_fifo, int, 0644);
    MODULE_PARM_DESC(consumer_fifo, "use fifo for consumer: 0 - disabled, 1 - low prio, 2 - fifo");
    static int read_events;
    static int test_error;

    do {					
    if (!test_error) {		
    test_error = 1;		
    WARN_ON!(1);		
    }				
    } while (0)
    enum event_status {
    EVENT_FOUND,
    EVENT_DROPPED,
    };
#[no_mangle]
unsafe extern "C" fn break_test() -> bool {
    return test_error || kthread_should_stop();
    }
#[no_mangle]
unsafe extern "C" fn read_event(cpu: c_int) -> enum event_status {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut ts = 0;
    event = ring_buffer_consume(buffer, cpu, &ts, core::ptr::null_mut());
    if (!event) {
    return EVENT_DROPPED;
    }
    entry = ring_buffer_event_data(event);
    if (*entry != cpu) {
    TEST_ERROR();
    return EVENT_DROPPED;
    }
    read += 1;
    return EVENT_FOUND;
    }
#[no_mangle]
unsafe extern "C" fn read_page(cpu: c_int) -> enum event_status {
pub static mut bpage: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut rpage: *mut c_void = core::ptr::null_mut();
    let mut commit = 0;
    let mut page_size = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut inc = 0;
    let mut i = 0;
    bpage = ring_buffer_alloc_read_page(buffer, cpu);
    if (IS_ERR(bpage)) {
    return EVENT_DROPPED;
    }
    page_size = ring_buffer_subbuf_size_get(buffer);
    ret = ring_buffer_read_page(buffer, bpage, page_size, cpu, 1);
    if (ret >= 0) {
    rpage = ring_buffer_read_page_data(bpage);
// The commit may have missed event flags set, clear them
    commit = local_read(&rpage.commit) & 0xfffff;
    while (i < commit && !test_error ) {
    if (i >= (page_size - offsetof(rb_page, data))) {
    TEST_ERROR();
    break;
    }
    inc = -1;
    event = &rpage.data[i];
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
// failed writes may be discarded events
    if (!event.time_delta) {
    TEST_ERROR();
    }
    inc = event.array[0] + 4;
    // break;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
    inc = 8;
    // break;
    }
    0 => {
    entry = ring_buffer_event_data(event);
    if (*entry != cpu) {
    TEST_ERROR();
    // break;
    }
    read += 1;
    if (!event.array[0]) {
    TEST_ERROR();
    // break;
    }
    inc = event.array[0] + 4;
    // break;
    }
    _ => {
    entry = ring_buffer_event_data(event);
    if (*entry != cpu) {
    TEST_ERROR();
    // break;
    }
    read += 1;
    inc = ((event.type_len + 1) * 4);
    }
    }
    if (test_error) {
    break;
    }
    if (inc <= 0) {
    TEST_ERROR();
    break;
    }
    }
    }
    ring_buffer_free_read_page(buffer, cpu, bpage);
    if (ret < 0) {
    return EVENT_DROPPED;
    }
    return EVENT_FOUND;
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_consumer() {
// toggle between reading pages and events
    read_events ^= 1;
    read = 0;
//
// Continue running until the producer specifically asks to stop
// and is ready for the completion.
//
    while (!READ_ONCE(reader_finish)) {
pub static mut found: c_int = 1;
    while (found && !test_error) {
    let mut cpu = 0;
    found = 0;
    for_each_online_cpu(cpu) {
    enum event_status stat;
    if (read_events) {
    stat = read_event(cpu);
    }
    else {
    stat = read_page(cpu);
    }
    if (test_error) {
    break;
    }
    if (stat == EVENT_FOUND) {
    found = 1;
    }
    }
    }
// Wait till the producer wakes us up when there is more data
// available or when the producer wants us to finish reading.
//
    set_current_state(TASK_INTERRUPTIBLE);
    if (reader_finish) {
    break;
    }
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    reader_finish = 0;
    complete(&read_done);
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_producer() {
    ktime_t start_time, end_time, timeout;
    unsigned long long time;
    unsigned long long entries;
    unsigned long long overruns;
pub static mut missed: c_ulong = 0;
pub static mut hit: c_ulong = 0;
    let mut avg = 0;
pub static mut cnt: c_int = 0;
//
// Hammer the buffer for 10 secs (this may
// make the system stall)
//
    trace_printk("Starting ring buffer hammer\n");
    start_time = ktime_get();
    timeout = ktime_add_ns(start_time, RUN_TIME * NSEC_PER_SEC);
    do {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < write_iteration) {
    event = ring_buffer_lock_reserve(buffer, 10);
    if (!event) {
    missed += 1;
    } else {
    hit += 1;
    entry = ring_buffer_event_data(event);
// entry = smp_processor_id();
    ring_buffer_unlock_commit(buffer);
    }
    }
    end_time = ktime_get();
    cnt += 1;
    if (consumer && !(cnt % wakeup_interval)) {
    wake_up_process(consumer);
    }

//
// If we are a non preempt kernel, the 10 seconds run will
// stop everything while it runs. Instead, we will call
// cond_resched and also add any time that was lost by a
// reschedule.
//
// Do a cond resched at the same frequency we would wake up
// the reader.
//
    if (cnt % wakeup_interval) {
    cond_resched();
    }

    } while (ktime_before(end_time, timeout) && !break_test());
    trace_printk("End ring buffer hammer\n");
    if (consumer) {
// Init both completions here to avoid races
    init_completion(&read_start);
    init_completion(&read_done);
// the completions must be visible before the finish var
    smp_wmb();
    reader_finish = 1;
    wake_up_process(consumer);
    wait_for_completion(&read_done);
    }
    time = ktime_us_delta(end_time, start_time);
    entries = ring_buffer_entries(buffer);
    overruns = ring_buffer_overruns(buffer);
    if (test_error) {
    trace_printk("ERROR!\n");
    }
    if (!disable_reader) {
    if (consumer_fifo) {
    trace_printk("Running Consumer at SCHED_FIFO %s\n",
    str_low_high(consumer_fifo == 1));
    }
    else {
    trace_printk("Running Consumer at nice: %d\n",
    consumer_nice);
    }
    }
    if (producer_fifo) {
    trace_printk("Running Producer at SCHED_FIFO %s\n",
    str_low_high(producer_fifo == 1));
    }
    else {
    trace_printk("Running Producer at nice: %d\n",
    producer_nice);
    }
// Let the user know that the test is running at low priority
    if (!producer_fifo && !consumer_fifo &&
    producer_nice == MAX_NICE && consumer_nice == MAX_NICE) {
    trace_printk("WARNING!!! This test is running at lowest priority.\n");
    }
    trace_printk("Time:     %lld (usecs)\n", time);
    trace_printk("Overruns: %lld\n", overruns);
    if (disable_reader) {
    trace_printk("Read:     (reader disabled)\n");
    }
    else {
    trace_printk("Read:     %ld  (by %s)\n", read,
    read_events ? "events" : "pages");
    }
    trace_printk("Entries:  %lld\n", entries);
    trace_printk("Total:    %lld\n", entries + overruns + read);
    trace_printk("Missed:   %ld\n", missed);
    trace_printk("Hit:      %ld\n", hit);
// Convert time from usecs to millisecs
    do_div(time, USEC_PER_MSEC);
    if (time) {
    hit /= (long)time;
    }
    else {
    trace_printk("TIME IS ZERO??\n");
    }
    trace_printk("Entries per millisec: %ld\n", hit);
    if (hit) {
// Calculate the average time in nanosecs
    avg = NSEC_PER_MSEC / hit;
    trace_printk("%ld ns per entry\n", avg);
    }
    if (missed) {
    if (time) {
    missed /= (long)time;
    }
    trace_printk("Total iterations per millisec: %ld\n",
    hit + missed);
// it is possible that hit + missed will overflow and be zero
    if (!(hit + missed)) {
    trace_printk("hit + missed overflowed and totalled zero!\n");
    hit -= 1; /* make it non zero */
    }
// Calculate the average time in nanosecs
    avg = NSEC_PER_MSEC / (hit + missed);
    trace_printk("%ld ns per entry\n", avg);
    }
    }
#[no_mangle]
unsafe extern "C" fn wait_to_die() {
    set_current_state(TASK_INTERRUPTIBLE);
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_consumer_thread(arg: *mut c_void) -> c_int {
    while (!break_test()) {
    complete(&read_start);
    ring_buffer_consumer();
    set_current_state(TASK_INTERRUPTIBLE);
    if (break_test()) {
    break;
    }
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    if (!kthread_should_stop()) {
    wait_to_die();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_producer_thread(arg: *mut c_void) -> c_int {
    while (!break_test()) {
    ring_buffer_reset(buffer);
    if (consumer) {
    wake_up_process(consumer);
    wait_for_completion(&read_start);
    }
    ring_buffer_producer();
    if (break_test()) {
// goto;
    }
    trace_printk("Sleeping for 10 secs\n");
    set_current_state(TASK_INTERRUPTIBLE);
    if (break_test()) {
// goto;
    }
    schedule_timeout(HZ * SLEEP_TIME);
    }
// label;
    __set_current_state(TASK_RUNNING);
    if (!kthread_should_stop()) {
    wait_to_die();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_benchmark_init() -> c_int {
    let mut ret = 0;
// make a one meg buffer in overwrite mode
    buffer = ring_buffer_alloc(1000000, RB_FL_OVERWRITE);
    if (!buffer) {
    return -ENOMEM;
    }
    if (!disable_reader) {
    consumer = kthread_create(ring_buffer_consumer_thread,
    core::ptr::null_mut(), "rb_consumer");
    ret = PTR_ERR(consumer);
    if (IS_ERR(consumer)) {
// goto;
    }
    }
    producer = kthread_run(ring_buffer_producer_thread,
    core::ptr::null_mut(), "rb_producer");
    ret = PTR_ERR(producer);
    if (IS_ERR(producer)) {
// goto;
    }
//
// Run them as low-prio background tasks by default:
//
    if (!disable_reader) {
    if (consumer_fifo >= 2) {
    sched_set_fifo(consumer);
    }

    else if (consumer_fifo == 1) {
    sched_set_fifo_low(consumer);
    }
    else {
    set_user_nice(consumer, consumer_nice);
    }
    }
    if (producer_fifo >= 2) {
    sched_set_fifo(producer);
    }

    else if (producer_fifo == 1) {
    sched_set_fifo_low(producer);
    }
    else {
    set_user_nice(producer, producer_nice);
    }
    return 0;
// label;
    if (consumer) {
    kthread_stop(consumer);
    }
// label;
    ring_buffer_free(buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ring_buffer_benchmark_exit()  {
    kthread_stop(producer);
    if (consumer) {
    kthread_stop(consumer);
    }
    ring_buffer_free(buffer);
    }
    module_init!(ring_buffer_benchmark_init);
    module_exit!(ring_buffer_benchmark_exit);
    MODULE_AUTHOR("Steven Rostedt");
    MODULE_DESCRIPTION("ring_buffer_benchmark");
    MODULE_LICENSE("GPL");