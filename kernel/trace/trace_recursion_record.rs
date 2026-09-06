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
    void ftrace_record_recursion(unsigned long ip, unsigned long parent_ip)
    {
    let mut index: c_int = 0;
    int i;
    unsigned long old;
    again:
// First check the last one recorded
    if (ip == cached_function)
    return;
    i = atomic_read(&nr_records);
// nr_records is -1 when clearing records
    smp_mb__after_atomic();
    if (i < 0)
    return;
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
    if (index < i)
    index = i;
    if (index >= CONFIG_FTRACE_RECORD_RECURSION_SIZE)
    return;
    for (i = index - 1; i >= 0; i--) {
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
    if (old == ip)
    return;
// Try the next location (use i for the next index)
    index++;
    goto again;
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
// func, 0)
//
// But the worse that could happen is that we get a zero in
// the recursed_functions array, and it's likely that "func" will
// be recorded again.
//
    i = atomic_read(&nr_records);
    smp_mb__after_atomic();
    if (i < 0)
    cmpxchg(&recursed_functions[index].ip, ip, 0);
#[no_mangle]
pub unsafe extern "C" fn if(index: i <=) -> else {
    else if (i <= index)
    atomic_cmpxchg(&nr_records, i, index + 1);
    }
    EXPORT_SYMBOL_GPL(ftrace_record_recursion);
    static DEFINE_MUTEX(recursed_function_lock);
    static struct trace_seq *tseq;
    static void *recursed_function_seq_start(struct seq_file *m, loff_t *pos)
    {
    void *ret = core::ptr::null_mut();
    int index;
    mutex_lock(&recursed_function_lock);
    index = atomic_read(&nr_records);
    if (*pos < index) {
    ret = &recursed_functions[*pos];
    }
    tseq = kzalloc_obj(*tseq);
    if (!tseq)
    return ERR_PTR(-ENOMEM);
    trace_seq_init(tseq);
    return ret;
    }
    static void *recursed_function_seq_next(struct seq_file *m, void *v, loff_t *pos)
    {
    int index;
    int p;
    index = atomic_read(&nr_records);
    p = ++(*pos);
    return p < index ? &recursed_functions[p] : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_seq_stop(m: *mut seq_file, v: *mut c_void) {
    static void recursed_function_seq_stop(struct seq_file *m, void *v)
    {
    kfree(tseq);
    mutex_unlock(&recursed_function_lock);
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int recursed_function_seq_show(struct seq_file *m, void *v)
    {
    struct recursed_functions *record = v;
    let mut ret: c_int = 0;
    if (record) {
    trace_seq_print_sym(tseq, record.parent_ip, true);
    trace_seq_puts(tseq, ":\t");
    trace_seq_print_sym(tseq, record.ip, true);
    trace_seq_putc(tseq, '\n');
    ret = trace_print_seq(m, tseq);
    }
    return ret;
    }
    static const struct seq_operations recursed_function_seq_ops = {
    .start  = recursed_function_seq_start,
    .next   = recursed_function_seq_next,
    .stop   = recursed_function_seq_stop,
    .show   = recursed_function_seq_show
    };
#[no_mangle]
unsafe extern "C" fn recursed_function_open(inode: *mut inode, file: *mut file) -> c_int {
    static int recursed_function_open(struct inode *inode, struct file *file)
    {
    guard(mutex)(&recursed_function_lock);
// If this file was opened for write, then erase contents
    if ((file.f_mode & FMODE_WRITE) && (file.f_flags & O_TRUNC)) {
// disable updating records
    atomic_set(&nr_records, -1);
    smp_mb__after_atomic();
    memset(recursed_functions, 0, sizeof(recursed_functions));
    smp_wmb();
// enable them again
    atomic_set(&nr_records, 0);
    }
    if (file.f_mode & FMODE_READ)
    return seq_open(file, &recursed_function_seq_ops);
    return 0;
    }
    static ssize_t recursed_function_write(struct file *file,
    const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    return count;
    }
#[no_mangle]
unsafe extern "C" fn recursed_function_release(inode: *mut inode, file: *mut file) -> c_int {
    static int recursed_function_release(struct inode *inode, struct file *file)
    {
    if (file.f_mode & FMODE_READ)
    seq_release(inode, file);
    return 0;
    }
    static const struct file_operations recursed_functions_fops = {
    .open           = recursed_function_open,
    .write		= recursed_function_write,
    .read           = seq_read,
    .llseek         = seq_lseek,
    .release        = recursed_function_release,
    };
#[no_mangle]
pub unsafe extern "C" fn create_recursed_functions() -> __init static int {
    __init static int create_recursed_functions(void)
    {
    trace_create_file("recursed_functions", TRACE_MODE_WRITE,
    core::ptr::null_mut(), core::ptr::null_mut(), &recursed_functions_fops);
    return 0;
    }
    fs_initcall(create_recursed_functions);
