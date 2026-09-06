//! Automatically rewritten from C to Rust
//! Source: kernel/crash_reserve.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// crash.c - kernel crash support code.
// Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
//

// Location of the reserved area for the crash kernel
pub static mut resource: usize = 0;
pub static mut resource: usize = 0;
//
// parsing the "crashkernel" commandline
//
// this code is intended to be called from architecture specific code
//
// This function parses command lines in the format
//
// crashkernel=ramsize-range:size[,...][@offset]
//
// The function returns 0 on success and -EINVAL on failure.
//
    static int __init parse_crashkernel_mem(char *cmdline,
    unsigned long long system_ram,
    unsigned long long *crash_size,
    unsigned long long *crash_base)
    {
    let mut cur = cmdline, *tmp;
pub static mut total_mem: c_ulonglong = 0;
//
// Firmware sometimes reserves some memory regions for its own use,
// so the system memory size is less than the actual physical memory
// size. Work around this by rounding up the total size to 128M,
// which is enough for most test cases.
//
    total_mem = roundup(total_mem, SZ_128M);
// for each entry of the comma-separated list
    do {
    unsigned long long start, end = ULLONG_MAX, size;
// get the start of the range
    start = memparse(cur, &tmp);
    if (cur == tmp) {
    pr_warn!("crashkernel: Memory value expected\n");
    return -EINVAL;
    }
    cur = tmp;
    if (*cur != '-') {
    pr_warn!("crashkernel: '-' expected\n");
    return -EINVAL;
    }
    cur += 1;
// if no ':' is here, than we read the end
    if (*cur != ':') {
    end = memparse(cur, &tmp);
    if (cur == tmp) {
    pr_warn!("crashkernel: Memory value expected\n");
    return -EINVAL;
    }
    cur = tmp;
    if (end <= start) {
    pr_warn!("crashkernel: end <= start\n");
    return -EINVAL;
    }
    }
    if (*cur != ':') {
    pr_warn!("crashkernel: ':' expected\n");
    return -EINVAL;
    }
    cur += 1;
    size = memparse(cur, &tmp);
    if (cur == tmp) {
    pr_warn!("crashkernel: Memory value expected\n");
    return -EINVAL;
    }
    cur = tmp;
    if (size >= total_mem) {
    pr_warn!("crashkernel: invalid size\n");
    return -EINVAL;
    }
// match ?
    if (total_mem >= start && total_mem < end) {
// crash_size = size;
    break;
    }
    } while (*cur++ == ',');
    if (*crash_size > 0) {
    while (*cur && *cur != ' ' && *cur != '@') {
    cur += 1;
    }
    if (*cur == '@') {
    cur += 1;
// crash_base = memparse(cur, &tmp);
    if (cur == tmp) {
    pr_warn!("crashkernel: Memory value expected after '@'\n");
    return -EINVAL;
    }
    }
    } else {
    pr_info!("crashkernel size resulted in zero bytes\n");
    }
    return 0;
    }
//
// That function parses "simple" (old) crashkernel command lines like
//
// crashkernel=size[@offset]
//
// It returns 0 on success and -EINVAL on failure.
//
    static int __init parse_crashkernel_simple(char *cmdline,
    unsigned long long *crash_size,
    unsigned long long *crash_base)
    {
    let mut cur = cmdline;
// crash_size = memparse(cmdline, &cur);
    if (cmdline == cur) {
    pr_warn!("crashkernel: memory value expected\n");
    return -EINVAL;
    }
    if (*cur == '@') {
// crash_base = memparse(cur+1, &cur);
    }
if true {
    pr_warn!("crashkernel: unrecognized char: %c\n", *cur);
    return -EINVAL;
    }
    return 0;
    }
pub const SUFFIX_HIGH: c_int = 0;
pub const SUFFIX_LOW: c_int = 1;
pub const SUFFIX_CMA: c_int = 2;
pub const SUFFIX_NULL: c_int = 3;
    static __initdata char *suffix_tbl[] = {
    [SUFFIX_HIGH] = ",high",
    [SUFFIX_LOW]  = ",low",
    [SUFFIX_CMA]  = ",cma",
    [SUFFIX_NULL] = core::ptr::null_mut(),
    };
//
// That function parses "suffix"  crashkernel command lines like
//
// crashkernel=size,[high|low|cma]
//
// It returns 0 on success and -EINVAL on failure.
//
    static int __init parse_crashkernel_suffix(char *cmdline,
    unsigned long long *crash_size,
    const char *suffix)
    {
    let mut cur = cmdline;
// crash_size = memparse(cmdline, &cur);
    if (cmdline == cur) {
    pr_warn!("crashkernel: memory value expected\n");
    return -EINVAL;
    }
// check with suffix
    if (strncmp(cur, suffix, strlen(suffix))) {
    pr_warn!("crashkernel: unrecognized char: %c\n", *cur);
    return -EINVAL;
    }
    cur += strlen(suffix);
    if (*cur != ' ' && *cur != '\0') {
    pr_warn!("crashkernel: unrecognized char: %c\n", *cur);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_last_crashkernel() {
    let mut p = cmdline, *ck_cmdline = core::ptr::null_mut();
// find crashkernel and use the last one if there are more
    p = strstr(p, name);
    while (p) {
    let mut end_p = strchr(p, ' ');
    let mut q = core::ptr::null_mut();
    if (!end_p) {
    end_p = p + strlen(p);
    }
    if (!suffix) {
    let mut i = 0;
// skip the one with any known suffix
    while (suffix_tbl[i]) {
    q = end_p - strlen(suffix_tbl[i]);
    if (!strncmp(q, suffix_tbl[i],
    strlen(suffix_tbl[i]))) {
// goto;
    }
    }
    ck_cmdline = p;
    } else {
    q = end_p - strlen(suffix);
    if (!strncmp(q, suffix, strlen(suffix))) {
    ck_cmdline = p;
    }
    }
// label;
    p = strstr(p+1, name);
    }
    return ck_cmdline;
    }
    static int __init __parse_crashkernel(char *cmdline,
    unsigned long long system_ram,
    unsigned long long *crash_size,
    unsigned long long *crash_base,
    const char *suffix)
    {
    let mut first_colon = core::ptr::null_mut();
    let mut first_space = core::ptr::null_mut();
    let mut ck_cmdline = core::ptr::null_mut();
    let mut name = "crashkernel=";
// BUG_ON;
// crash_size = 0;
// crash_base = 0;
    ck_cmdline = get_last_crashkernel(cmdline, name, suffix);
    if (!ck_cmdline) {
    return -ENOENT;
    }
    ck_cmdline += strlen(name);
    if (suffix) {
    return parse_crashkernel_suffix(ck_cmdline, crash_size,
    suffix);
    }
//
// if the commandline contains a ':', then that's the extended
// syntax -- if not, it must be the classic syntax
//
    first_colon = strchr(ck_cmdline, ':');
    first_space = strchr(ck_cmdline, ' ');
    if (first_colon && (!first_space || first_colon < first_space)) {
    return parse_crashkernel_mem(ck_cmdline, system_ram,
    crash_size, crash_base);
    }
    return parse_crashkernel_simple(ck_cmdline, crash_size, crash_base);
    }
//
// That function is the entry point for command line parsing and should be
// called from the arch-specific code.
//
// If crashkernel=,high|low is supported on architecture, non-NULL values
// should be passed to parameters 'low_size' and 'high'.
//
    int __init parse_crashkernel(char *cmdline,
    unsigned long long system_ram,
    unsigned long long *crash_size,
    unsigned long long *crash_base,
    unsigned long long *low_size,
    unsigned long long *cma_size,
    bool *high)
    {
    let mut ret = 0;
    unsigned long long __always_unused cma_base;
// crashkernel=X[@offset]
    ret = __parse_crashkernel(cmdline, system_ram, crash_size,
    crash_base, core::ptr::null_mut());

//
// If non-NULL 'high' passed in and no normal crashkernel
// setting detected, try parsing crashkernel=,high|low.
//
    if (high && ret == -ENOENT) {
    ret = __parse_crashkernel(cmdline, 0, crash_size,
    crash_base, suffix_tbl[SUFFIX_HIGH]);
    if (ret || !*crash_size) {
    return -EINVAL;
    }
//
// crashkernel=Y,low can be specified or not, but invalid value
// is not allowed.
//
    ret = __parse_crashkernel(cmdline, 0, low_size,
    crash_base, suffix_tbl[SUFFIX_LOW]);
    if (ret == -ENOENT) {
// low_size = DEFAULT_CRASH_KERNEL_LOW_SIZE;
    ret = 0;
    } else if (ret) {
    return ret;
    }
// high = true;
    }
//
// optional CMA reservation
// cma_base is ignored
//
    if (cma_size) {
    __parse_crashkernel(cmdline, 0, cma_size,
    &cma_base, suffix_tbl[SUFFIX_CMA]);
    }

    if (!*crash_size) {
    ret = -EINVAL;
    }
    if (*crash_size >= system_ram) {
    ret = -EINVAL;
    }
    return ret;
    }
//
// Add a dummy early_param handler to mark crashkernel= as a known command line
// parameter and suppress incorrect warnings in init/main.c.
//
#[no_mangle]
unsafe extern "C" fn parse_crashkernel_dummy(arg: *mut c_char) -> c_int {
    return 0;
    }
    early_param!("crashkernel", parse_crashkernel_dummy);

#[no_mangle]
unsafe extern "C" fn reserve_crashkernel_low(low_size: c_ulonglong) -> c_int {

    unsigned long long low_base;
    low_base = memblock_phys_alloc_range(low_size, CRASH_ALIGN, 0, CRASH_ADDR_LOW_MAX);
    if (!low_base) {
    pr_err!("cannot allocate crashkernel low memory (size:0x%llx).\n", low_size);
    return -ENOMEM;
    }
    pr_info!("crashkernel low memory reserved: 0x%08llx - 0x%08llx (%lld MB)\n",
    low_base, low_base + low_size, low_size >> 20);
    crashk_low_res.start = low_base;
    crashk_low_res.end   = low_base + low_size - 1;

    insert_resource(&iomem_resource, &crashk_low_res);

    return 0;
    }
    void __init reserve_crashkernel_generic(unsigned long long crash_size,
    unsigned long long crash_base,
    unsigned long long crash_low_size,
    bool high)
    {
pub static mut search_end: c_ulonglong = 0;
pub static mut fixed_base: bool = false;
// User specifies base address explicitly.
    if (crash_base) {
    fixed_base = true;
    search_base = crash_base;
    search_end = crash_base + crash_size;
    } else if (high) {
    search_base = CRASH_ADDR_LOW_MAX;
    search_end = CRASH_ADDR_HIGH_MAX;
    }
// label;
    crash_base = memblock_phys_alloc_range(crash_size, CRASH_ALIGN,
    search_base, search_end);
    if (!crash_base) {
//
// For crashkernel=size[KMG]@offset[KMG], print out failure
// message if can't reserve the specified region.
//
    if (fixed_base) {
    pr_warn!("crashkernel reservation failed - memory is in use.\n");
    return;
    }
//
// For crashkernel=size[KMG], if the first attempt was for
// low memory, fall back to high memory, the minimum required
// low memory will be reserved later.
//
    if (!high && search_end == CRASH_ADDR_LOW_MAX) {
    search_end = CRASH_ADDR_HIGH_MAX;
    search_base = CRASH_ADDR_LOW_MAX;
    crash_low_size = DEFAULT_CRASH_KERNEL_LOW_SIZE;
// goto;
    }
//
// For crashkernel=size[KMG],high, if the first attempt was
// for high memory, fall back to low memory.
//
    if (high && search_end == CRASH_ADDR_HIGH_MAX) {
    search_end = CRASH_ADDR_LOW_MAX;
    search_base = 0;
    if (search_end != CRASH_ADDR_HIGH_MAX) {
// goto;
    }
    }
    pr_warn!("cannot allocate crashkernel (size:0x%llx)\n",
    crash_size);
    return;
    }
    if ((crash_base >= CRASH_ADDR_LOW_MAX) &&
    crash_low_size && reserve_crashkernel_low(crash_low_size)) {
    memblock_phys_free(crash_base, crash_size);
    return;
    }
    pr_info!("crashkernel reserved: 0x%016llx - 0x%016llx (%lld MB)\n",
    crash_base, crash_base + crash_size, crash_size >> 20);
//
// The crashkernel memory will be removed from the kernel linear
// map. Inform kmemleak so that it won't try to access it.
//
    kmemleak_ignore_phys(crash_base);
    if (crashk_low_res.end) {
    kmemleak_ignore_phys(crashk_low_res.start);
    }
    crashk_res.start = crash_base;
    crashk_res.end = crash_base + crash_size - 1;

    insert_resource(&iomem_resource, &crashk_res);

    }
    struct range crashk_cma_ranges[CRASHKERNEL_CMA_RANGES_MAX];

    let mut crashk_cma_cnt = 0;
#[no_mangle]
pub unsafe extern "C" fn reserve_crashkernel_cma(cma_size: c_ulonglong) -> c_int {
pub static mut request_size: c_ulonglong = 0;
pub static mut reserved_size: c_ulonglong = 0;
    if (!cma_size) {
    return;
    }
    while (cma_size > reserved_size &&
    crashk_cma_cnt < CRASHKERNEL_CMA_RANGES_MAX) {
    let mut res = core::ptr::null_mut();
    if (cma_declare_contiguous(0, request_size, 0, 0, 0, false,
    "crashkernel", &res)) {
// reservation failed, try half-sized blocks
    if (request_size <= PAGE_SIZE) {
    break;
    }
    request_size = roundup(request_size / 2, PAGE_SIZE);
    continue;
    }
    crashk_cma_ranges[crashk_cma_cnt].start = cma_get_base(res);
    crashk_cma_ranges[crashk_cma_cnt].end =
    crashk_cma_ranges[crashk_cma_cnt].start +
    cma_get_size(res) - 1;
    crashk_cma_cnt += 1;
    reserved_size += request_size;
    }
    if (cma_size > reserved_size) {
    pr_warn!("crashkernel CMA reservation failed: %lld MB requested, %lld MB reserved in %d ranges\n",
    cma_size >> 20, reserved_size >> 20, crashk_cma_cnt);
    }
    else {
    pr_info!("crashkernel CMA reserved: %lld MB in %d ranges\n",
    reserved_size >> 20, crashk_cma_cnt);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: reserve_crashkernel_cma
pub unsafe extern "C" fn reserve_crashkernel_cma_dup(cma_size: c_ulonglong) -> c_int {
    if (cma_size) {
    pr_warn!("crashkernel CMA reservation not supported\n");
    }
    }

#[no_mangle]
unsafe extern "C" fn insert_crashkernel_resources() -> __init int {
    if (!arch_add_crash_res_to_iomem()) {
    return 0;
    }
    if (crashk_res.start < crashk_res.end) {
    insert_resource(&iomem_resource, &crashk_res);
    }
    if (crashk_low_res.start < crashk_low_res.end) {
    insert_resource(&iomem_resource, &crashk_low_res);
    }
    return 0;
    }
// early_initcall;