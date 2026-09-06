//! Automatically rewritten from C to Rust
//! Source: kernel/time/sched_clock.c
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
// Generic sched_clock() support, to extend low level hardware time
// counters to full 64-bit ns values.
//

//
// struct clock_data - all data needed for sched_clock() (including
// registration of a new clock source)
//
// @seq:		Sequence counter for protecting updates. The lowest
// bit is the index for @read_data.
// @read_data:		Data required to read from sched_clock.
// @wrap_kt:		Duration for which clock can run before wrapping.
// @rate:		Tick rate of the registered clock.
// @actual_read_sched_clock: Registered hardware level clock read function.
//
// The ordering of this structure has been chosen to optimize cache
// performance. In particular 'seq' and 'read_data[0]' (combined) should fit
// into a single 64-byte cache line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_data {
    pub seq: seqcount_latch_t,
    pub read_data: [clock_read_data; 2],
    pub wrap_kt: ktime_t,
    pub rate: c_ulong,
    pub (*actual_read_sched_clock)(void): *mut u64,
}

pub static mut sched_clock_timer: usize = 0;
pub static mut irqtime: int = 0;
    core_param!(irqtime, irqtime, int, 0400);
#[no_mangle]
unsafe extern "C" fn jiffy_sched_clock_read() -> u64 notrace {
//
// We don't need to use get_jiffies_64 on 32-bit arches here
// because we register with BITS_PER_LONG
//
    return (u64)(jiffies - INITIAL_JIFFIES);
    }
    static struct clock_data cd ____cacheline_aligned = {
    .read_data[0] = { .mult = NSEC_PER_SEC / HZ,
    .read_sched_clock = jiffy_sched_clock_read, },
    .actual_read_sched_clock = jiffy_sched_clock_read,
    };
#[no_mangle]
unsafe extern "C" fn cyc_to_ns(cyc: u64, mult: u32, shift: u32) -> __always_inline u64 {
    return (cyc * mult) >> shift;
    }
    notrace struct clock_read_data *sched_clock_read_begin(unsigned int *seq)
    {
// seq = read_seqcount_latch(&cd.seq);
    return cd.read_data + (*seq & 1);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_read_retry(seq: c_uint) -> notrace int {
    return read_seqcount_latch_retry(&cd.seq, seq);
    }
#[no_mangle]
unsafe extern "C" fn __sched_clock() -> __always_inline unsigned long long {
pub static mut rd: *mut c_void = core::ptr::null_mut();
    let mut seq = 0;
    u64 cyc, res;
    do {
    seq = raw_read_seqcount_latch(&cd.seq);
    rd = cd.read_data + (seq & 1);
    cyc = (rd.read_sched_clock() - rd.epoch_cyc) &
    rd.sched_clock_mask;
    res = rd.epoch_ns + cyc_to_ns(cyc, rd.mult, rd.shift);
    } while (raw_read_seqcount_latch_retry(&cd.seq, seq));
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_noinstr() -> unsigned long long noinstr {
    return __sched_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock() -> unsigned long long notrace {
    unsigned long long ns;
    preempt_disable_notrace();
//
// All of __sched_clock() is a seqcount_latch reader critical section,
// but relies on the raw helpers which are uninstrumented. For KCSAN,
// mark all accesses in __sched_clock() as atomic.
//
    kcsan_nestable_atomic_begin();
    ns = __sched_clock();
    kcsan_nestable_atomic_end();
    preempt_enable_notrace();
    return ns;
    }
//
// Updating the data required to read the clock.
//
// sched_clock() will never observe mis-matched data even if called from
// an NMI. We do this by maintaining an odd/even copy of the data and
// steering sched_clock() to one or the other using a sequence counter.
// In order to preserve the data cache profile of sched_clock() as much
// as possible the system reverts back to the even copy when the update
// completes; the odd copy is used *only* during an update.
//
#[no_mangle]
unsafe extern "C" fn update_clock_read_data(rd: *mut clock_read_data) {
// steer readers towards the odd copy
    write_seqcount_latch_begin(&cd.seq);
// now its safe for us to update the normal (even) copy
    cd.read_data[0] = *rd;
// switch readers back to the even copy
    write_seqcount_latch(&cd.seq);
// update the backup (odd) copy with the new data
    cd.read_data[1] = *rd;
    write_seqcount_latch_end(&cd.seq);
    }
//
// Atomically update the sched_clock() epoch.
//
#[no_mangle]
unsafe extern "C" fn update_sched_clock() {
    let mut cyc = 0;
    let mut ns = 0;
pub static mut rd: usize = 0;
    rd = cd.read_data[0];
    cyc = cd.actual_read_sched_clock();
    ns = rd.epoch_ns + cyc_to_ns((cyc - rd.epoch_cyc) & rd.sched_clock_mask, rd.mult, rd.shift);
    rd.epoch_ns = ns;
    rd.epoch_cyc = cyc;
    update_clock_read_data(&rd);
    }
#[no_mangle]
unsafe extern "C" fn sched_clock_poll(hrt: *mut hrtimer) -> enum hrtimer_restart {
    update_sched_clock();
    hrtimer_forward_now(hrt, cd.wrap_kt);
    return HRTIMER_RESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_register((*read)(void): *mut u64, bits: c_int, rate: c_ulong) {
#[no_mangle]
#[no_mangle]
// duplicate fn: sched_clock_register
pub unsafe extern "C" fn sched_clock_register_dup(bits: c_int, rate: c_ulong) {
    u64 res, wrap, new_mask, new_epoch, cyc, ns;
    u32 new_mult, new_shift;
    unsigned long r, flags;
    let mut r_unit = 0;
pub static mut rd: usize = 0;
    if (cd.rate > rate) {
    return;
    }
// Cannot register a sched_clock with interrupts on
    local_irq_save(flags);
// Calculate the mult/shift to convert counter ticks to ns.
    clocks_calc_mult_shift(&new_mult, &new_shift, rate, NSEC_PER_SEC, 3600);
    new_mask = CLOCKSOURCE_MASK(bits);
    cd.rate = rate;
// Calculate how many nanosecs until we risk wrapping
    wrap = clocks_calc_max_nsecs(new_mult, new_shift, 0, new_mask, core::ptr::null_mut());
    cd.wrap_kt = ns_to_ktime(wrap);
    rd = cd.read_data[0];
// Update epoch for new counter and update 'epoch_ns' from old counter
    new_epoch = read();
    cyc = cd.actual_read_sched_clock();
    ns = rd.epoch_ns + cyc_to_ns((cyc - rd.epoch_cyc) & rd.sched_clock_mask, rd.mult, rd.shift);
    cd.actual_read_sched_clock = read;
    rd.read_sched_clock	= read;
    rd.sched_clock_mask	= new_mask;
    rd.mult			= new_mult;
    rd.shift		= new_shift;
    rd.epoch_cyc		= new_epoch;
    rd.epoch_ns		= ns;
    update_clock_read_data(&rd);
    if (ACCESS_PRIVATE(&sched_clock_timer, function) != core::ptr::null_mut()) {
// update timeout for clock wrap
    hrtimer_start(&sched_clock_timer, cd.wrap_kt,
    HRTIMER_MODE_REL_HARD);
    }
    r = rate;
    if (r >= 4000000) {
    r = DIV_ROUND_CLOSEST(r, 1000000);
    r_unit = 'M';
    } else if (r >= 4000) {
    r = DIV_ROUND_CLOSEST(r, 1000);
    r_unit = 'k';
    } else {
    r_unit = ' ';
    }
// Calculate the ns resolution of this counter
    res = cyc_to_ns(1ULL, new_mult, new_shift);
    pr_info!("sched_clock: %u bits at %lu%cHz, resolution %lluns, wraps every %lluns\n",
    bits, r, r_unit, res, wrap);
// Enable IRQ time accounting if we have a fast enough sched_clock()
    if (irqtime > 0 || (irqtime == -1 && rate >= 1000000)) {
    enable_sched_clock_irqtime();
    }
    local_irq_restore(flags);
    pr_debug!("Registered %pS as sched_clock source\n", read);
    }
    EXPORT_SYMBOL_GPL(sched_clock_register);
#[no_mangle]
pub unsafe extern "C" fn generic_sched_clock_init()  {
//
// If no sched_clock() function has been provided at that point,
// make it the final one.
//
    if (cd.actual_read_sched_clock == jiffy_sched_clock_read) {
    sched_clock_register(jiffy_sched_clock_read, BITS_PER_LONG, HZ);
    }
    update_sched_clock();
//
// Start the timer to keep sched_clock() properly updated and
// sets the initial epoch.
//
    hrtimer_setup(&sched_clock_timer, sched_clock_poll, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(&sched_clock_timer, cd.wrap_kt, HRTIMER_MODE_REL_HARD);
    }
//
// Clock read function for use when the clock is suspended.
//
// This function makes it appear to sched_clock() as if the clock
// stopped counting at its last update.
//
// This function must only be called from the critical
// section in sched_clock(). It relies on the read_seqcount_retry()
// at the end of the critical section to be sure we observe the
// correct copy of 'epoch_cyc'.
//
#[no_mangle]
unsafe extern "C" fn suspended_sched_clock_read() -> u64 notrace {
pub static mut seq: c_uint = 0;
    return cd.read_data[seq & 1].epoch_cyc;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_suspend() -> c_int {
    let mut rd = &cd.read_data[0];
    update_sched_clock();
    hrtimer_cancel(&sched_clock_timer);
    rd.read_sched_clock = suspended_sched_clock_read;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_clock_syscore_suspend(data: *mut c_void) -> c_int {
    return sched_clock_suspend();
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_resume() {
    let mut rd = &cd.read_data[0];
    rd.epoch_cyc = cd.actual_read_sched_clock();
    hrtimer_start(&sched_clock_timer, cd.wrap_kt, HRTIMER_MODE_REL_HARD);
    rd.read_sched_clock = cd.actual_read_sched_clock;
    }
#[no_mangle]
unsafe extern "C" fn sched_clock_syscore_resume(data: *mut c_void) {
    sched_clock_resume();
    }
pub static mut syscore_ops: usize = 0;
pub static mut syscore: usize = 0;
#[no_mangle]
unsafe extern "C" fn sched_clock_syscore_init() -> c_int {
    register_syscore(&sched_clock_syscore);
    return 0;
    }
    device_initcall!(sched_clock_syscore_init);
}
