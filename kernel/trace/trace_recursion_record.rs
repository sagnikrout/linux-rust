//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_recursion_record.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct recursed_functions {
    pub ip: c_ulong,
    pub parent_ip: c_ulong,
}

    static struct recursed_functions recursed_functions[CONFIG_FTRACE_RECORD_RECURSION_SIZE];
    static atomic_t nr_records;
//
// Cache the last found function. Yes, updates to this is racey, but
// so is memory cache ;-)
//
    static unsigned long cached_function;
#[no_mangle]
pub unsafe extern "C" fn ftrace_record_recursion(ip: c_ulong, parent_ip: c_ulong) {
pub static mut index: c_int = 0;
    let mut i = 0;
    let mut old = 0;
// label;
// First check the last one recorded
    if (ip == cached_function) {
    return;
    }
    i = atomic_read(&nr_records);
// nr_records is -1 when clearing records
    smp_mb__after_atomic();
    if (i < 0) {
    return;
    }
//
// If there's two writers and this writer comes in second,
// the cmpxchg() below to update the ip will fail. Then this
// writer will try again. It is possible that index will now
// be greater than nr_records. This is because the writer
// that succeeded has not updated the nr_records yet.
// This writer could keep trying again until the other writer
// updates nr_records. But if the other writer takes an
// interrupt, and that interrupt locks up that CPU, we do
// not want this CPU to lock up due to the recursion protection,
// and have a bug report showing this CPU as the cause of
// locking up the computer. To not lose this record, this
// writer will simply use the next position to update the
// recursed_functions, and it will update the nr_records
// accordingly.
//
    if (index < i) {
    index = i;
    }
    if (index >= CONFIG_FTRACE_RECORD_RECURSION_SIZE) {
    return;
    }
    while (i >= 0) {
    if (recursed_functions[i].ip == ip) {
    cached_function = ip;
    return;
    }
    }
    cached_function = ip;
//
// We only want to add a function if it hasn't been added before.
// Add to the current location before incrementing the count.
// If it fails to add, then increment the index (save in i)
// and try again.
//
    old = cmpxchg(&recursed_functions[index].ip, 0, ip);
    if (old != 0) {
// Did something else already added this for us?
    if (old == ip) {
    return;
    }
// Try the next location (use i for the next index)
    index += 1;
// goto;
    }
    recursed_functions[index].parent_ip = parent_ip;
//
// It's still possible that we could race with the clearing
// CPU0                                    CPU1
// ----                                    ----
// ip = func
// nr_records = -1;
// recursed_functions[0] = 0;
// i = -1
// if (i < 0)
// nr_records = 0;
// (new recursion detected)
// recursed_functions[0] = func
// cmpxchg(recursed_functions[0],
func, 0)
//
// But the worse that could happen is that we get a zero in
// the recursed_functions array, and it's likely that "func" will
// be recorded again.
//
    i = atomic_read(&nr_records);
    smp_mb__after_atomic();
    if (i < 0) {
    cmpxchg(&recursed_functions[index].ip, ip, 0);
    }

    else if (i <= index) {
    atomic_cmpxchg(&nr_records, i, index + 1);
    }
    }
    EXPORT_SYMBOL_GPL(ftrace_record_recursion);
pub static mut recursed_function_lock: usize = 0;
pub static mut tseq: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn recursed_function_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
    let mut index = 0;
    mutex_lock(&recursed_function_lock);
    index = atomic_read(&nr_records);
    if (*pos < index) {
    ret = &recursed_functions[*pos];
    }
    tseq = kzalloc_obj(*tseq);
    if (!tseq) {
    return ERR_PTR(-ENOMEM);
    }
    trace_seq_init(tseq);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn recursed_function_seq_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut index = 0;
    let mut p = 0;
    index = atomic_read(&nr_records);
    p = ++(*pos);
    return p < index ? &recursed_functions[p] : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_seq_stop(m: *mut seq_file, v: *mut c_void) {
    kfree(tseq);
    mutex_unlock(&recursed_function_lock);
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut record = v;
pub static mut ret: c_int = 0;
    if (record) {
    trace_seq_print_sym(tseq, record.parent_ip, true);
    trace_seq_puts(tseq, ":\t");
    trace_seq_print_sym(tseq, record.ip, true);
    trace_seq_putc(tseq, '\n');
    ret = trace_print_seq(m, tseq);
    }
    return ret;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn recursed_function_open(inode: *mut inode, file: *mut file) -> c_int {
    guard(mutex)(&recursed_function_lock);
// If this file was opened for write, then erase contents
    if ((file.f_mode & FMODE_WRITE) && (file.f_flags & O_TRUNC)) {
// disable updating records
    atomic_set(&nr_records, -1);
    smp_mb__after_atomic();
    memset(recursed_functions, 0, sizeof!(recursed_functions));
    smp_wmb();
// enable them again
    atomic_set(&nr_records, 0);
    }
    if (file.f_mode & FMODE_READ) {
    return seq_open(file, &recursed_function_seq_ops);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn recursed_function_write(file: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    return count;
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_release(inode: *mut inode, file: *mut file) -> c_int {
    if (file.f_mode & FMODE_READ) {
    seq_release(inode, file);
    }
    return 0;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn create_recursed_functions() -> __init static int {
    trace_create_file("recursed_functions", TRACE_MODE_WRITE,
    core::ptr::null_mut(), core::ptr::null_mut(), &recursed_functions_fops);
    return 0;
    }
    fs_initcall!(create_recursed_functions);