//! Automatically rewritten from C to Rust
//! Source: kernel/time/timer_list.c
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
// List pending timers
//
// Copyright(C) 2006, Red Hat, Inc., Ingo Molnar
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_list_iter {
    pub cpu: c_int,
    pub second_pass: bool,
    pub now: ktime_t,
}

//
// This allows printing both to /proc/timer_list and
// to the console (on SysRq-Q):
//
    __printf(2, 3)
#[no_mangle]
unsafe extern "C" fn SEQ_printf(m: *mut seq_file, fmt: *const c_char, ...) {
    let mut args;
    va_start(args, fmt);
    if (m) {
    seq_vprintf(m, fmt, args);
    }
    else {
    vprintk(fmt, args);
    }
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn print_timer(m: *mut seq_file, taddr: *mut hrtimer, timer: *mut hrtimer, idx: c_int, now: ktime_t) {
    SEQ_printf(m, " #%d: <%p>, %ps", idx, taddr, ACCESS_PRIVATE(timer, function));
    SEQ_printf(m, ", S:%02x", timer.is_queued);
    SEQ_printf(m, "\n");
    SEQ_printf(m, " # expires at %lld-%lld nsecs [in %lld to %lld nsecs]\n",
    (long long)hrtimer_get_softexpires(timer),
    (long long)hrtimer_get_expires(timer),
    (long long)ktime_sub(hrtimer_get_softexpires(timer), now),
    (long long)ktime_sub(hrtimer_get_expires(timer), now));
    }
#[no_mangle]
unsafe extern "C" fn print_active_timers(m: *mut seq_file, base: *mut hrtimer_clock_base, now: ktime_t) {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    struct hrtimer *timer, tmp;
pub static mut next: c_ulong = 0;
    let mut flags = 0;
// label;
    i = 0;
    touch_nmi_watchdog();
    raw_spin_lock_irqsave(&base.cpu_base.lock, flags);
    curr = timerqueue_linked_first(&base.active);
//
// Crude but we have to do this O(N*N) thing, because
// we have to unlock the base when printing:
//
    while (curr && i < next) {
    curr = timerqueue_linked_next(curr);
    i += 1;
    }
    if (curr) {
    timer = container_of!(curr, hrtimer, node);
    tmp = *timer;
    raw_spin_unlock_irqrestore(&base.cpu_base.lock, flags);
    print_timer(m, timer, &tmp, i, now);
    next += 1;
// goto;
    }
    raw_spin_unlock_irqrestore(&base.cpu_base.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn print_base(m: *mut seq_file, base: *mut hrtimer_clock_base, now: ktime_t) {
    SEQ_printf(m, "  .base:       %p\n", base);
    SEQ_printf(m, "  .index:      %d\n", base.index);
    SEQ_printf(m, "  .resolution: %u nsecs\n", hrtimer_resolution);

    SEQ_printf(m, "  .offset:     %lld nsecs\n",
    (long long) base.offset);

    SEQ_printf(m,   "active timers:\n");
    print_active_timers(m, base, ktime_add(now, base.offset));
    }
#[no_mangle]
unsafe extern "C" fn print_cpu(m: *mut seq_file, cpu: c_int, now: ktime_t) {
    let mut cpu_base = &per_cpu(hrtimer_bases, cpu);
    let mut i = 0;
    SEQ_printf(m, "cpu: %d\n", cpu);
    while (i < HRTIMER_MAX_CLOCK_BASES) {
    SEQ_printf(m, " clock %d:\n", i);
    print_base(m, cpu_base.clock_base + i, now);
    }

    SEQ_printf(m, "  .%-15s: %llu\n", #x, 
    (unsigned long long)DIAG_READ(cpu_base.x))

    SEQ_printf(m, "  .%-15s: %lld nsecs\n", #x, (long long)DIAG_READ(cpu_base.x))

    P_ktime(expires_next);
    P(hres_active);
    P(nr_events);
    P(nr_retries);
    P(nr_hangs);
    P(max_hang_time);

    SEQ_printf(m, "  .%-15s: %llu\n", #x, 
    (unsigned long long)DIAG_READ(ts.x))

    SEQ_printf(m, "  .%-15s: %lld nsecs\n", #x, (long long)DIAG_READ(ts.x))

    SEQ_printf(m, "  .%-15s: %d\n", #x, !!(DIAG_READ(ts.flags) & (f)))
    {
    let mut ts = tick_get_tick_sched(cpu);
    P_flag(nohz, TS_FLAG_NOHZ);
    P_flag(highres, TS_FLAG_HIGHRES);
    P_ktime(last_tick);
    P_flag(tick_stopped, TS_FLAG_STOPPED);
    P(idle_calls);
    P(idle_sleeps);
    P_ktime(idle_entrytime);
    P_ktime(idle_waketime);
    P(last_jiffies);
    P(next_timer);
    P_ktime(idle_expires);
    SEQ_printf(m, "jiffies: %llu\n",
    (unsigned long long)jiffies);
    }

    SEQ_printf(m, "\n");
    }

#[no_mangle]
pub unsafe extern "C" fn print_tickdevice(m: *mut seq_file, td: *mut tick_device, cpu: c_int) {
    let mut dev = td.evtdev;
    touch_nmi_watchdog();
    SEQ_printf(m, "Tick Device: mode:     %d\n", td.mode);
    if (cpu < 0) {
    SEQ_printf(m, "Broadcast device\n");
    }
    else {
    SEQ_printf(m, "Per CPU device: %d\n", cpu);
    }
    SEQ_printf(m, "Clock Event Device: ");
    if (!dev) {
    SEQ_printf(m, "<core::ptr::null_mut()>\n");
    return;
    }
    SEQ_printf(m, "%s\n", dev.name);
    SEQ_printf(m, " max_delta_ns:   %llu\n",
    (unsigned long long) dev.max_delta_ns);
    SEQ_printf(m, " min_delta_ns:   %llu\n",
    (unsigned long long) dev.min_delta_ns);
    SEQ_printf(m, " mult:           %u\n", dev.mult);
    SEQ_printf(m, " shift:          %u\n", dev.shift);
    SEQ_printf(m, " mode:           %d\n", clockevent_get_state(dev));
    SEQ_printf(m, " next_event:     %lld nsecs\n", (long long)dev.next_event);
    SEQ_printf(m, " set_next_event: %ps\n", dev.set_next_event);
    if (dev.set_state_shutdown) {
    SEQ_printf(m, " shutdown:       %ps\n",
    dev.set_state_shutdown);
    }
    if (dev.set_state_periodic) {
    SEQ_printf(m, " periodic:       %ps\n",
    dev.set_state_periodic);
    }
    if (dev.set_state_oneshot) {
    SEQ_printf(m, " oneshot:        %ps\n",
    dev.set_state_oneshot);
    }
    if (dev.set_state_oneshot_stopped) {
    SEQ_printf(m, " oneshot stopped: %ps\n",
    dev.set_state_oneshot_stopped);
    }
    if (dev.tick_resume) {
    SEQ_printf(m, " resume:         %ps\n",
    dev.tick_resume);
    }
    SEQ_printf(m, " event_handler:  %ps\n", dev.event_handler);
    SEQ_printf(m, "\n");
    SEQ_printf(m, " retries:        %lu\n", dev.retries);

    if (cpu >= 0) {
    let mut wd = tick_get_wakeup_device(cpu);
    SEQ_printf(m, "Wakeup Device: %s\n", wd ? wd.name : "<core::ptr::null_mut()>");
    }

    SEQ_printf(m, "\n");
    }
#[no_mangle]
unsafe extern "C" fn timer_list_show_tickdevices_header(m: *mut seq_file) {

    print_tickdevice(m, tick_get_broadcast_device(), -1);
    SEQ_printf(m, "tick_broadcast_mask: %*pb\n",
    cpumask_pr_args(tick_get_broadcast_mask()));

    SEQ_printf(m, "tick_broadcast_oneshot_mask: %*pb\n",
    cpumask_pr_args(tick_get_broadcast_oneshot_mask()));

    SEQ_printf(m, "\n");

    }

#[no_mangle]
pub unsafe extern "C" fn timer_list_header(m: *mut seq_file, now: ktime_t) {
    SEQ_printf(m, "Timer List Version: v0.11\n");
    SEQ_printf(m, "HRTIMER_MAX_CLOCK_BASES: %d\n", HRTIMER_MAX_CLOCK_BASES);
    SEQ_printf(m, "now at %lld nsecs\n", (long long)now);
    SEQ_printf(m, "\n");
    }
#[no_mangle]
pub unsafe extern "C" fn sysrq_timer_list_show() {
pub static mut now: ktime_t = 0;
    let mut cpu = 0;
    timer_list_header(core::ptr::null_mut(), now);
    for_each_online_cpu(cpu) {
    print_cpu(core::ptr::null_mut(), cpu, now);
    }

    timer_list_show_tickdevices_header(core::ptr::null_mut());
    for_each_online_cpu(cpu) {
    print_tickdevice(core::ptr::null_mut(), tick_get_device(cpu), cpu);
    }

    return;
    }

#[no_mangle]
unsafe extern "C" fn timer_list_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut iter = v;
    if (iter.cpu == -1 && !iter.second_pass) {
    timer_list_header(m, iter.now);
    }

    else if (!iter.second_pass) {
    print_cpu(m, iter.cpu, iter.now);
    }


    else if (iter.cpu == -1 && iter.second_pass) {
    timer_list_show_tickdevices_header(m);
    }
    else {
    print_tickdevice(m, tick_get_device(iter.cpu), iter.cpu);
    }

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn move_iter(iter: *mut timer_list_iter, offset: loff_t) -> *mut c_void {
    while (offset) {
    iter.cpu = cpumask_next(iter.cpu, cpu_online_mask);
    if (iter.cpu >= nr_cpu_ids) {

    if (!iter.second_pass) {
    iter.cpu = -1;
    iter.second_pass = true;
    } else {
    return core::ptr::null_mut();
    }

    return core::ptr::null_mut();

    }
    }
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn timer_list_start(file: *mut seq_file, offset: *mut loff_t) -> *mut c_void {
    let mut iter = file.private;
    if (!*offset) {
    iter.now = ktime_get();
    }
    iter.cpu = -1;
    iter.second_pass = false;
    return move_iter(iter, *offset);
    }
#[no_mangle]
pub unsafe extern "C" fn timer_list_next(file: *mut seq_file, v: *mut c_void, offset: *mut loff_t) -> *mut c_void {
    let mut iter = file.private;
    ++*offset;
    return move_iter(iter, 1);
    }
#[no_mangle]
unsafe extern "C" fn timer_list_stop(seq: *mut seq_file, v: *mut c_void) {
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_timer_list_procfs() -> c_int {
pub static mut pe: *mut c_void = core::ptr::null_mut();
    pe = proc_create_seq_private("timer_list", 0400, core::ptr::null_mut(), &timer_list_sops,
    sizeof!(timer_list_iter), core::ptr::null_mut());
    if (!pe) {
    return -ENOMEM;
    }
    return 0;
    }
    __initcall!(init_timer_list_procfs);