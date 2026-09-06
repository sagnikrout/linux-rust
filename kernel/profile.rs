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

    static atomic_t *prof_buffer;
    static unsigned long prof_len;
    static unsigned short int prof_shift;
    int prof_on __read_mostly;
    EXPORT_SYMBOL_GPL(prof_on);
#[no_mangle]
pub unsafe extern "C" fn profile_setup(str: *mut c_char) -> c_int {
    static const char schedstr[] = "schedule";
    static const char kvmstr[] = "kvm";
    const char *select = core::ptr::null_mut();
    int par;
    if (!strncmp(str, schedstr, strlen(schedstr))) {
    prof_on = SCHED_PROFILING;
    select = schedstr;
    } else if (!strncmp(str, kvmstr, strlen(kvmstr))) {
    prof_on = KVM_PROFILING;
    select = kvmstr;
    } else if (get_option(&str, &par)) {
    prof_shift = clamp(par, 0, BITS_PER_LONG - 1);
    prof_on = CPU_PROFILING;
    pr_info("kernel profiling enabled (shift: %u)\n",
    prof_shift);
    }
    if (select) {
    if (str[strlen(select)] == ',')
    str += strlen(select) + 1;
    if (get_option(&str, &par))
    prof_shift = clamp(par, 0, BITS_PER_LONG - 1);
    pr_info("kernel %s profiling enabled (shift: %u)\n",
    select, prof_shift);
    }
    return 1;
    }
    __setup("profile=", profile_setup);
#[no_mangle]
pub unsafe extern "C" fn profile_init() -> int __ref {
    int buffer_bytes;
    if (!prof_on)
    return 0;
// only text is profiled
    prof_len = (_etext - _stext) >> prof_shift;
    if (!prof_len) {
    pr_warn("profiling shift: %u too large\n", prof_shift);
    prof_on = 0;
    return -EINVAL;
    }
    buffer_bytes = prof_len*sizeof(atomic_t);
    prof_buffer = kzalloc(buffer_bytes, GFP_KERNEL|__GFP_NOWARN);
    if (prof_buffer)
    return 0;
    prof_buffer = alloc_pages_exact(buffer_bytes,
    GFP_KERNEL|__GFP_ZERO|__GFP_NOWARN);
    if (prof_buffer)
    return 0;
    prof_buffer = vzalloc(buffer_bytes);
    if (prof_buffer)
    return 0;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn do_profile_hits(type: c_int, __pc: *mut c_void, nr_hits: c_uint) {
    unsigned long pc;
    pc = ((unsigned long)__pc - (unsigned long)_stext) >> prof_shift;
    if (pc < prof_len)
    atomic_add(nr_hits, &prof_buffer[pc]);
    }
#[no_mangle]
pub unsafe extern "C" fn profile_hits(type: c_int, __pc: *mut c_void, nr_hits: c_uint) {
    if (prof_on != type || !prof_buffer)
    return;
    do_profile_hits(type, __pc, nr_hits);
    }
    EXPORT_SYMBOL_GPL(profile_hits);
#[no_mangle]
pub unsafe extern "C" fn profile_tick(type: c_int) {
    struct pt_regs *regs = get_irq_regs();
// This is the old kernel-only legacy profiling
    if (!user_mode(regs))
    profile_hit(type, (void *)profile_pc(regs));
    }

//
// This function accesses profiling information. The returned data is
// binary: the sampling step and the actual contents of the profile
// buffer. Use of the program readprofile is recommended in order to
// get meaningful info out of these data.
//
    static ssize_t
    read_profile(struct file *file, char __user *buf, size_t count, loff_t *ppos)
    {
    let mut p: c_ulong = *ppos;
    ssize_t read;
    char *pnt;
    let mut sample_step: c_ulong = 1UL << prof_shift;
    if (p >= (prof_len+1)*sizeof(unsigned int))
    return 0;
    if (count > (prof_len+1)*sizeof(unsigned int) - p)
    count = (prof_len+1)*sizeof(unsigned int) - p;
    read = 0;
    while (p < sizeof(unsigned int) && count > 0) {
    if (put_user(*((char *)(&sample_step)+p), buf))
    return -EFAULT;
    buf++; p++; count--; read++;
    }
    pnt = (char *)prof_buffer + p - sizeof(atomic_t);
    if (copy_to_user(buf, (void *)pnt, count))
    return -EFAULT;
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
    static ssize_t write_profile(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {

    if (count == sizeof(int)) {
    unsigned int multiplier;
    if (copy_from_user(&multiplier, buf, sizeof(int)))
    return -EFAULT;
    if (setup_profiling_timer(multiplier))
    return -EINVAL;
    }

    memset(prof_buffer, 0, prof_len * sizeof(atomic_t));
    return count;
    }
    static const struct proc_ops profile_proc_ops = {
    .proc_read	= read_profile,
    .proc_write	= write_profile,
    .proc_lseek	= default_llseek,
    };
#[no_mangle]
pub unsafe extern "C" fn create_proc_profile() -> int __ref {
    struct proc_dir_entry *entry;
    let mut err: c_int = 0;
    if (!prof_on)
    return 0;
    entry = proc_create("profile", S_IWUSR | S_IRUGO,
    core::ptr::null_mut(), &profile_proc_ops);
    if (entry)
    proc_set_size(entry, (1 + prof_len) * sizeof(atomic_t));
    return err;
    }
    subsys_initcall(create_proc_profile);
