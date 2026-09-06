//! Automatically rewritten from C to Rust
//! Source: kernel/profile.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/profile.c
// Simple profiling. Manages a direct-mapped profile hit count buffer,
// with configurable resolution, support for restricting the cpus on
// which profiling is done, and switching between cpu time and
// schedule() calls via kernel command line parameters passed at boot.
//
// Scheduler profiling support, Arjan van de Ven and Ingo Molnar,
// Red Hat, July 2004
// Consolidation of architecture support code for profiling,
// Nadia Yvette Chambers, Oracle, July 2004
// Amortized hit count accounting via per-cpu open-addressed hashtables
// to resolve timer interrupt livelocks, Nadia Yvette Chambers,
// Oracle, 2004
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct profile_hit {
    pub hits: u32 pc,,
}

pub const PROFILE_GRPSHIFT: c_int = 3;

pub static mut prof_buffer: *mut c_void = core::ptr::null_mut();
    static unsigned long prof_len;
    static unsigned short int prof_shift;
    let mut prof_on = 0;
    EXPORT_SYMBOL_GPL(prof_on);
#[no_mangle]
pub unsafe extern "C" fn profile_setup(str: *mut c_char) -> c_int {
    static const char schedstr[] = "schedule";
    static const char kvmstr[] = "kvm";
    let mut select = core::ptr::null_mut();
    let mut par = 0;
    if (!strncmp(str, schedstr, strlen(schedstr))) {
    prof_on = SCHED_PROFILING;
    select = schedstr;
    } else if (!strncmp(str, kvmstr, strlen(kvmstr))) {
    prof_on = KVM_PROFILING;
    select = kvmstr;
    } else if (get_option(&str, &par)) {
    prof_shift = clamp(par, 0, BITS_PER_LONG - 1);
    prof_on = CPU_PROFILING;
    pr_info!("kernel profiling enabled (shift: %u)\n",
    prof_shift);
    }
    if (select) {
    if (str[strlen(select)] == ',') {
    str += strlen(select) + 1;
    }
    if (get_option(&str, &par)) {
    prof_shift = clamp(par, 0, BITS_PER_LONG - 1);
    }
    pr_info!("kernel %s profiling enabled (shift: %u)\n",
    select, prof_shift);
    }
    return 1;
    }
    __setup!("profile=", profile_setup);
#[no_mangle]
pub unsafe extern "C" fn profile_init() -> int __ref {
    let mut buffer_bytes = 0;
    if (!prof_on) {
    return 0;
    }
// only text is profiled
    prof_len = (_etext - _stext) >> prof_shift;
    if (!prof_len) {
    pr_warn!("profiling shift: %u too large\n", prof_shift);
    prof_on = 0;
    return -EINVAL;
    }
    buffer_bytes = prof_len*sizeof!(atomic_t);
    prof_buffer = kzalloc(buffer_bytes, GFP_KERNEL|__GFP_NOWARN);
    if (prof_buffer) {
    return 0;
    }
    prof_buffer = alloc_pages_exact(buffer_bytes,
    GFP_KERNEL|__GFP_ZERO|__GFP_NOWARN);
    if (prof_buffer) {
    return 0;
    }
    prof_buffer = vzalloc(buffer_bytes);
    if (prof_buffer) {
    return 0;
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn do_profile_hits(type: c_int, __pc: *mut c_void, nr_hits: c_uint) {
    let mut pc = 0;
    pc = ((unsigned long)__pc - (unsigned long)_stext) >> prof_shift;
    if (pc < prof_len) {
    atomic_add(nr_hits, &prof_buffer[pc]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn profile_hits(type: c_int, __pc: *mut c_void, nr_hits: c_uint) {
    if (prof_on != type || !prof_buffer) {
    return;
    }
    do_profile_hits(type, __pc, nr_hits);
    }
    EXPORT_SYMBOL_GPL(profile_hits);
#[no_mangle]
pub unsafe extern "C" fn profile_tick(type: c_int) {
    let mut regs = get_irq_regs();
// This is the old kernel-only legacy profiling
    if (!user_mode(regs)) {
    profile_hit(type, profile_pc(regs));
    }
    }

//
// This function accesses profiling information. The returned data is
// binary: the sampling step and the actual contents of the profile
// buffer. Use of the program readprofile is recommended in order to
// get meaningful info out of these data.
//
#[no_mangle]
pub unsafe extern "C" fn read_profile(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut p: c_ulong = 0;
    let mut read = 0;
pub static mut pnt: *mut c_void = core::ptr::null_mut();
pub static mut sample_step: c_ulong = 0;
    if (p >= (prof_len+1)*sizeof!(unsigned int)) {
    return 0;
    }
    if (count > (prof_len+1)*sizeof!(unsigned int) - p) {
    count = (prof_len+1)*sizeof!(unsigned int) - p;
    }
    read = 0;
    while (p < sizeof!(unsigned int) && count > 0) {
    if (put_user(*((&sample_step)+p), buf)) {
    return -EFAULT;
    }
    buf += 1; p += 1; count -= 1; read += 1;
    }
    pnt = prof_buffer + p - sizeof!(atomic_t);
    if (copy_to_user(buf, pnt, count)) {
    return -EFAULT;
    }
    read += count;
// ppos += read;
    return read;
    }
// default is to not implement this call
#[no_mangle]
pub unsafe extern "C" fn setup_profiling_timer(mult: unsigned) -> int __weak {
    return -EINVAL;
    }
//
// Writing to /proc/profile resets the counters
//
// Writing a 'profiling multiplier' value into it also re-sets the profiling
// interrupt frequency, on architectures that support this.
//
#[no_mangle]
pub unsafe extern "C" fn write_profile(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {

    if (count == sizeof!(int)) {
    let mut multiplier = 0;
    if (copy_from_user(&multiplier, buf, sizeof!(int))) {
    return -EFAULT;
    }
    if (setup_profiling_timer(multiplier)) {
    return -EINVAL;
    }
    }

    memset(prof_buffer, 0, prof_len * sizeof!(atomic_t));
    return count;
    }
pub static mut proc_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn create_proc_profile() -> int __ref {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (!prof_on) {
    return 0;
    }
    entry = proc_create("profile", S_IWUSR | S_IRUGO,
    core::ptr::null_mut(), &profile_proc_ops);
    if (entry) {
    proc_set_size(entry, (1 + prof_len) * sizeof!(atomic_t));
    }
    return err;
    }
    subsys_initcall!(create_proc_profile);