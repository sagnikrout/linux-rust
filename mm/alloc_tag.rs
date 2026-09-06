//! Automatically rewritten from C to Rust
//! Source: mm/alloc_tag.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub static mut mem_profiling_support: bool = true;

    static bool mem_profiling_support;

//
// Memory allocation profiling is permanently disabled and cannot be enabled.
// Must be called after setup_early_mem_profiling().
//
#[no_mangle]
pub unsafe extern "C" fn mem_alloc_profiling_permanently_disabled() -> bool {
    return !mem_profiling_support;
    }
pub static mut alloc_tag_cttype: *mut c_void = core::ptr::null_mut();

pub static mut struct alloc_tag_counters: usize = 0;
    EXPORT_SYMBOL(_shared_alloc_tag);

pub static mut CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT: usize = 0;
    EXPORT_SYMBOL(mem_alloc_profiling_key);
pub static mut mem_profiling_compressed: usize = 0;
pub static mut kernel_tags: alloc_tag_kernel_section = 0;
    let mut alloc_tag_ref_mask = 0;
    let mut alloc_tag_ref_offs = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocinfo_private {
    pub iter: codetag_iterator,
    pub reported_iter: codetag_iterator,
    pub print_header: bool,
    pub filter: allocinfo_filter,
// ioctl uses a separate iterator not to interfere with reads
    pub ioctl_iter: codetag_iterator,
//     pub /: *mut *mut bool positioned; / seq_open_private() sets to 0,
    pub ioctl_lock: mutex,
}

#[no_mangle]
pub unsafe extern "C" fn allocinfo_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut priv: *mut c_void = core::ptr::null_mut();
pub static mut node: loff_t = 0;
    priv = m.private;
    codetag_lock_module_list(alloc_tag_cttype);
    if (node == 0) {
    priv.print_header = true;
    priv.iter = codetag_get_ct_iter(alloc_tag_cttype);
    } else {
    priv.iter = priv.reported_iter;
    }
    codetag_next_ct(&priv.iter);
    return priv.iter.ct ? priv : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn allocinfo_next(m: *mut seq_file, arg: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut priv = arg;
pub static mut ct: *mut c_void = core::ptr::null_mut();
    priv.reported_iter = priv.iter;
    ct = codetag_next_ct(&priv.iter);
    (*pos)++;
    if (!ct) {
    return core::ptr::null_mut();
    }
    return priv;
    }
#[no_mangle]
unsafe extern "C" fn allocinfo_stop(m: *mut seq_file, arg: *mut c_void) {
    codetag_unlock_module_list(alloc_tag_cttype);
    }
#[no_mangle]
unsafe extern "C" fn print_allocinfo_header(buf: *mut seq_buf) {
// Output format version, so we can change it.
    seq_buf_printf(buf, "allocinfo - version: 2.0\n");
    seq_buf_printf(buf, "#     <size>  <calls> <tag info>\n");
    }
#[no_mangle]
unsafe extern "C" fn alloc_tag_to_text(out: *mut seq_buf, ct: *mut codetag) {
    let mut tag = ct_to_alloc_tag(ct);
pub static mut counter: alloc_tag_counters = 0;
pub static mut bytes: i64 = 0;
    seq_buf_printf(out, "%12lli %8llu ", bytes, counter.calls);
    codetag_to_text(out, ct);
    if (unlikely(alloc_tag_is_inaccurate(tag))) {
    seq_buf_printf(out, " accurate:no");
    }
    seq_buf_putc(out, ' ');
    seq_buf_putc(out, '\n');
    }
#[no_mangle]
unsafe extern "C" fn allocinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut priv = arg;
pub static mut bufp: *mut c_void = core::ptr::null_mut();
pub static mut n: usize = 0;
pub static mut buf: usize = 0;
    seq_buf_init(&buf, bufp, n);
    if (priv.print_header) {
    print_allocinfo_header(&buf);
    priv.print_header = false;
    }
    alloc_tag_to_text(&buf, priv.iter.ct);
    seq_commit(m, seq_buf_used(&buf));
    return 0;
    }
pub static mut seq_operations: usize = 0;
//
// Initializes seq_file operations and allocates private state when opening
// the /proc/allocinfo procfs entry.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = seq_open_private(file, &allocinfo_seq_op,
    sizeof!(allocinfo_private));
    if (!ret) {
    let mut m = file.private_data;
    let mut priv = m.private;
    mutex_init(&priv.ioctl_lock);
    }
    return ret;
    }
//
// Cleans up the seq_file state and frees up the private state allocated in
// allocinfo_open() when closing the /proc/allocinfo file descriptor.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut m = file.private_data;
    let mut priv = m.private;
    mutex_destroy(&priv.ioctl_lock);
    return seq_release_private(inode, file);
    }
//
// Returns a pointer to the suffix of a string so that its length fits within
// ALLOCINFO_STR_SIZE, preserving the trailing characters.
// Function, file and module names often have the same prefixes, therefore
// when filtering by these criteria, we compare the last 64 characters to
// minimize the chances of name collisions
//
    static const char *allocinfo_str(const char *str)
    {
pub static mut len: usize = 0;
// Keep an extra space for the trailing NULL.
    if (len >= ALLOCINFO_STR_SIZE) {
    str += (len - ALLOCINFO_STR_SIZE) + 1;
    }
    return str;
    }
// Copy a string and trim from the beginning if it's too long
#[no_mangle]
unsafe extern "C" fn allocinfo_copy_str(dest: *mut c_char, src: *const c_char) {
    strscpy_pad(dest, allocinfo_str(src), ALLOCINFO_STR_SIZE);
    }
// Compare two strings and only consider the trimmed suffix if s1 is too long
#[no_mangle]
unsafe extern "C" fn allocinfo_cmp_str(str: *const c_char, template: *const c_char) -> c_int {
    return strncmp(allocinfo_str(str), template, ALLOCINFO_STR_SIZE);
    }
// Fetch the per-CPU counters
#[no_mangle]
pub unsafe extern "C" fn allocinfo_prefetch_counters(ct: *mut codetag) -> alloc_tag_counters {
    return alloc_tag_read(ct_to_alloc_tag(ct));
    }
//
// Populates the UAPI allocinfo_tag_data structure with active runtime
// profiling counters extracted from the given kernel codetag.
//
#[no_mangle]
pub unsafe extern "C" fn allocinfo_to_params(ct: *mut codetag, data: *mut allocinfo_tag_data, counters: *mut alloc_tag_counters) {
    if (ct.modname) {
    allocinfo_copy_str(data.tag.modname, ct.modname);
    }
    else {
    data.tag.modname[0] = '\0';
    }
    allocinfo_copy_str(data.tag.function, ct.function);
    allocinfo_copy_str(data.tag.filename, ct.filename);
    data.tag.lineno = ct.lineno;
    data.counter.bytes = counters.bytes;
    data.counter.calls = counters.calls;
    data.counter.accurate = !alloc_tag_is_inaccurate(ct_to_alloc_tag(ct));
    }
//
// Retrieves the unique content ID representing the current allocation tag module
// layout, allowing userspace to detect if modules were loaded / unloaded.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_content_id(m: *mut seq_file, arg: *mut c_void ) -> c_int {
pub static mut params: usize = 0;
    codetag_lock_module_list(alloc_tag_cttype);
    params.id = codetag_get_content_id(alloc_tag_cttype);
    codetag_unlock_module_list(alloc_tag_cttype);
    if (copy_to_user(arg, &params, sizeof!(params))) {
    return -EFAULT;
    }
    return 0;
    }
//
// Verifies whether a given codetag satisfies the active filtering criteria by
// matching its characteristics against the specified filter.
//
#[no_mangle]
pub unsafe extern "C" fn matches_filter(ct: *mut codetag, filter: *mut allocinfo_filter, counters: *mut alloc_tag_counters, fetched_counters: *mut bool) -> bool {
    let mut inaccurate = 0;
    if (!filter || !filter.mask) {
    return true;
    }
    if (filter.mask & ALLOCINFO_FILTER_MASK_MODNAME) {
// user wants to filter by modname but ct->modname is NULL
    if (!ct.modname) {
// validate if user was attempting to filter for built-in allocations
    if (filter.fields.modname[0] != '\0') {
    return false;
    }
    } else if (allocinfo_cmp_str(ct.modname, filter.fields.modname)) {
    return false;
    }
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_FUNCTION) &&
    ct.function && allocinfo_cmp_str(ct.function, filter.fields.function)) {
    return false;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_FILENAME) &&
    ct.filename && allocinfo_cmp_str(ct.filename, filter.fields.filename)) {
    return false;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_LINENO) &&
    ct.lineno != filter.fields.lineno) {
    return false;
    }
    if (filter.mask & ALLOCINFO_FILTER_MASK_INACCURATE) {
    inaccurate = !!(ct.flags & CODETAG_FLAG_INACCURATE);
    if (inaccurate != !!(filter.inaccurate)) {
    return false;
    }
    }
    if (filter.mask & (ALLOCINFO_FILTER_MASK_MIN_SIZE | ALLOCINFO_FILTER_MASK_MAX_SIZE)) {
    if (!*fetched_counters) {
// counters = allocinfo_prefetch_counters(ct);
// fetched_counters = true;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_MIN_SIZE) &&
    counters.bytes < filter.min_size) {
    return false;
    }
    if ((filter.mask & ALLOCINFO_FILTER_MASK_MAX_SIZE) &&
    counters.bytes > filter.max_size) {
    return false;
    }
    }
    return true;
    }
//
// Seeks the ioctl iterator to the specified 0-indexed tag position, reads its
// profiling data and returns it to userspace.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_at(m: *mut seq_file, arg: *mut c_void ) -> c_int {
pub static mut priv: *mut c_void = core::ptr::null_mut();
pub static mut ct: *mut c_void = core::ptr::null_mut();
pub static mut params: allocinfo_get_at = 0;
    let mut skip_count = 0;
pub static mut counters: usize = 0;
    let mut fetched_counters = 0;
    if (copy_from_user(&params, arg, sizeof!(params))) {
    return -EFAULT;
    }
    if (params.filter.mask & ~ALLOCINFO_FILTER_MASKS) {
    return -EINVAL;
    }
    if ((params.filter.mask & ALLOCINFO_FILTER_MASK_MIN_SIZE) &&
    (params.filter.mask & ALLOCINFO_FILTER_MASK_MAX_SIZE) &&
    params.filter.min_size > params.filter.max_size) {
    return -EINVAL;
    }
    priv = m.private;
    mutex_lock(&priv.ioctl_lock);
    codetag_lock_module_list(alloc_tag_cttype);
    if (params.pos >= codetag_get_count(alloc_tag_cttype)) {
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    return -ENOENT;
    }
    skip_count = params.pos;
    if (params.filter.mask) {
    priv.filter = params.filter;
    }
    else {
    priv.filter.mask = 0;
    }
// Find the codetag
    priv.ioctl_iter = codetag_get_ct_iter(alloc_tag_cttype);
    ct = codetag_next_ct(&priv.ioctl_iter);
    while (ct) {
    fetched_counters = false;
    if (matches_filter(ct, &priv.filter, &counters, &fetched_counters)) {
    if (skip_count == 0) {
    break;
    }
    skip_count -= 1;
    }
    ct = codetag_next_ct(&priv.ioctl_iter);
    }
    if (ct) {
    if (!fetched_counters) {
    counters = allocinfo_prefetch_counters(ct);
    }
    allocinfo_to_params(ct, &params.data, &counters);
    priv.positioned = true;
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    if (!ct) {
    return -ENOENT;
    }
    if (copy_to_user(arg, &params, sizeof!(params))) {
    return -EFAULT;
    }
    return 0;
    }
//
// Advances the ioctl iterator to the next allocation tag in the sequence and
// returns its profiling data to userspace.
//
#[no_mangle]
unsafe extern "C" fn allocinfo_ioctl_get_next(m: *mut seq_file, arg: *mut c_void ) -> c_int {
pub static mut priv: *mut c_void = core::ptr::null_mut();
pub static mut ct: *mut c_void = core::ptr::null_mut();
pub static mut params: usize = 0;
pub static mut ret: c_int = 0;
pub static mut counters: usize = 0;
    let mut fetched_counters = 0;
    memset(&params, 0, sizeof!(params));
    priv = m.private;
    mutex_lock(&priv.ioctl_lock);
    codetag_lock_module_list(alloc_tag_cttype);
    if (!priv.positioned) {
    priv.ioctl_iter = codetag_get_ct_iter(alloc_tag_cttype);
    priv.positioned = true;
    }
    ct = codetag_next_ct(&priv.ioctl_iter);
    while (ct) {
    fetched_counters = false;
    if (matches_filter(ct, &priv.filter, &counters, &fetched_counters)) {
    break;
    }
    ct = codetag_next_ct(&priv.ioctl_iter);
    }
    if (ct) {
    if (!fetched_counters) {
    counters = allocinfo_prefetch_counters(ct);
    }
    allocinfo_to_params(ct, &params, &counters);
    }
    if (!ct) {
    priv.positioned = false;
    ret = -ENOENT;
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    mutex_unlock(&priv.ioctl_lock);
    if (ret == 0) {
    if (copy_to_user(arg, &params, sizeof!(params))) {
    return -EFAULT;
    }
    }
    return ret;
    }
//
// Entry point ioctl function for /proc/allocinfo routing requests to fetch the
// layout content ID, seek to a specific tag, or read sequential tags.
//
#[no_mangle]
pub unsafe extern "C" fn allocinfo_ioctl(file: *mut file, cmd: c_uint, __arg: c_ulong) -> c_long {
    let mut arg = __arg;
    let mut ret = 0;
    match (cmd) {
    ALLOCINFO_IOC_CONTENT_ID => {
    ret = allocinfo_ioctl_get_content_id(file.private_data, arg);
    // break;
    }
    ALLOCINFO_IOC_GET_AT => {
    ret = allocinfo_ioctl_get_at(file.private_data, arg);
    // break;
    }
    ALLOCINFO_IOC_GET_NEXT => {
    ret = allocinfo_ioctl_get_next(file.private_data, arg);
    // break;
    }
    _ => {
    ret = -ENOIOCTLCMD;
    // break;
    }
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn allocinfo_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    return allocinfo_ioctl(file, cmd, (unsigned long)compat_ptr(arg));
    }

pub static mut proc_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_top_users(tags: *mut codetag_bytes, count: usize, can_sleep: bool) -> usize {
pub static mut iter: usize = 0;
pub static mut ct: *mut c_void = core::ptr::null_mut();
pub static mut n: usize = 0;
    unsigned int i, nr = 0;
    if (IS_ERR_OR_NULL(alloc_tag_cttype)) {
    return 0;
    }
    if (can_sleep) {
    codetag_lock_module_list(alloc_tag_cttype);
    }

    else if (!codetag_trylock_module_list(alloc_tag_cttype)) {
    return 0;
    }
    iter = codetag_get_ct_iter(alloc_tag_cttype);
    while ((ct = codetag_next_ct(&iter))) {
pub static mut counter: alloc_tag_counters = 0;
    n.ct	= ct;
    n.bytes = counter.bytes;
    for (i = 0; i < nr; i++) {
    if (n.bytes > tags[i].bytes)
    break;
    }
    if (i < count) {
    nr -= nr == count;
    memmove(&tags[i + 1],
    &tags[i],
    sizeof!(tags[0]) * (nr - i));
    nr += 1;
    tags[i] = n;
    }
    }
    codetag_unlock_module_list(alloc_tag_cttype);
    return nr;
    }
#[no_mangle]
pub unsafe extern "C" fn pgalloc_tag_split(folio: *mut folio, old_order: c_int, new_order: c_int) {
    let mut i = 0;
pub static mut tag: *mut c_void = core::ptr::null_mut();
pub static mut nr_pages: c_uint = 0;
    if (!mem_alloc_profiling_enabled()) {
    return;
    }
    tag = __pgalloc_tag_get(&folio.page);
    if (!tag) {
    return;
    }
    while (i < (1 << old_order)) {
    union pgtag_ref_handle handle;
    union codetag_ref ref;
    if (get_page_tag_ref(folio_page(folio, i), &ref, &handle)) {
// Set new reference to point to the original tag
    alloc_tag_ref_set(&ref, tag);
    update_page_tag_ref(handle, &ref);
    put_page_tag_ref(handle);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pgalloc_tag_swap(new: *mut folio, old: *mut folio) {
    union pgtag_ref_handle handle_old, handle_new;
    union codetag_ref ref_old, ref_new;
    let mut tag_old = core::ptr::null_mut();
    let mut tag_new = core::ptr::null_mut();
    if (!mem_alloc_profiling_enabled()) {
    return;
    }
    tag_old = __pgalloc_tag_get(&old.page);
    if (!tag_old) {
    return;
    }
    tag_new = __pgalloc_tag_get(&new.page);
    if (!tag_new) {
    return;
    }
    if (!get_page_tag_ref(&old.page, &ref_old, &handle_old)) {
    return;
    }
    if (!get_page_tag_ref(&new.page, &ref_new, &handle_new)) {
    put_page_tag_ref(handle_old);
    return;
    }
//
// Clear tag references to avoid debug warning when using
// __alloc_tag_ref_set() with non-empty reference.
//
    set_codetag_empty(&ref_old);
    set_codetag_empty(&ref_new);
// swap tags
    __alloc_tag_ref_set(&ref_old, tag_new);
    update_page_tag_ref(handle_old, &ref_old);
    __alloc_tag_ref_set(&ref_new, tag_old);
    update_page_tag_ref(handle_new, &ref_new);
    put_page_tag_ref(handle_old);
    put_page_tag_ref(handle_new);
    }
#[no_mangle]
unsafe extern "C" fn shutdown_mem_profiling(remove_file: bool) {
    if (mem_alloc_profiling_enabled()) {
    static_branch_disable(&mem_alloc_profiling_key);
    }
    if (!mem_profiling_support) {
    return;
    }
    if (remove_file) {
    remove_proc_entry(ALLOCINFO_FILE_NAME, core::ptr::null_mut());
    }
    mem_profiling_support = false;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_sec_init()  {
pub static mut last_codetag: *mut c_void = core::ptr::null_mut();
    if (!mem_profiling_support) {
    return;
    }
    if (!static_key_enabled(&mem_profiling_compressed)) {
    return;
    }
    kernel_tags.first_tag = kallsyms_lookup_name(
    SECTION_START(ALLOC_TAG_SECTION_NAME));
    last_codetag = kallsyms_lookup_name(
    SECTION_STOP(ALLOC_TAG_SECTION_NAME));
    kernel_tags.count = last_codetag - kernel_tags.first_tag;
// Check if kernel tags fit into page flags
    if (kernel_tags.count > (1UL << NR_UNUSED_PAGEFLAG_BITS)) {
    shutdown_mem_profiling(false); /* allocinfo file does not exist yet */
    pr_err!("%lu allocation tags cannot be references using %d available page flag bits. Memory allocation profiling is disabled!\n",
    kernel_tags.count, NR_UNUSED_PAGEFLAG_BITS);
    return;
    }
    alloc_tag_ref_offs = (LRU_REFS_PGOFF - NR_UNUSED_PAGEFLAG_BITS);
    alloc_tag_ref_mask = ((1UL << NR_UNUSED_PAGEFLAG_BITS) - 1);
    pr_debug!("Memory allocation profiling compression is using %d page flag bits!\n",
    NR_UNUSED_PAGEFLAG_BITS);
    }

pub static mut mod_area_mt: maple_tree = 0;
pub static mut vm_module_tags: *mut c_void = core::ptr::null_mut();
// A dummy object used to indicate an unloaded module
pub static mut unloaded_mod: usize = 0;
// A dummy object used to indicate a module prepended area
pub static mut prepend_mod: usize = 0;
pub static mut module_tags: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_align(val: c_ulong) -> c_ulong {
    if (!static_key_enabled(&mem_profiling_compressed)) {
// No alignment requirements when we are not indexing the tags
    return val;
    }
    if (val % sizeof!(alloc_tag) == 0) {
    return val;
    }
    return ((val / sizeof!(alloc_tag)) + 1) * sizeof!(alloc_tag);
    }
#[no_mangle]
unsafe extern "C" fn ensure_alignment(align: c_ulong, prepend: *mut c_uint) -> bool {
    if (!static_key_enabled(&mem_profiling_compressed)) {
// No alignment requirements when we are not indexing the tags
    return true;
    }
//
// If alloc_tag size is not a multiple of required alignment, tag
// indexing does not work.
//
    if (!IS_ALIGNED(sizeof!(alloc_tag), align)) {
    return false;
    }
// Ensure prepend consumes multiple of alloc_tag-sized blocks
    if (*prepend) {
// prepend = alloc_tag_align(*prepend);
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn tags_addressable() -> bool {
    let mut tag_idx_count = 0;
    if (!static_key_enabled(&mem_profiling_compressed)) {
    return true; /* with page_ext tags are always addressable */
    }
    tag_idx_count = CODETAG_ID_FIRST + kernel_tags.count +
    module_tags.size / sizeof!(alloc_tag);
    return tag_idx_count < (1UL << NR_UNUSED_PAGEFLAG_BITS);
    }
#[no_mangle]
unsafe extern "C" fn needs_section_mem(mod: *mut module, size: c_ulong) -> bool {
    if (!mem_profiling_support) {
    return false;
    }
    return size >= sizeof!(alloc_tag);
    }
#[no_mangle]
pub unsafe extern "C" fn clean_unused_counters(start_tag: *mut alloc_tag, end_tag: *mut alloc_tag) -> bool {
pub static mut tag: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = true;
    while (tag <= end_tag) {
pub static mut counter: usize = 0;
    if (!tag.counters) {
    continue;
    }
    counter = alloc_tag_read(tag);
    if (!counter.bytes) {
    free_percpu(tag.counters);
    tag.counters = core::ptr::null_mut();
    } else {
    ret = false;
    }
    }
    return ret;
    }
// Called with mod_area_mt locked
#[no_mangle]
unsafe extern "C" fn clean_unused_module_areas_locked() {
    MA_STATE(mas, &mod_area_mt, 0, module_tags.size);
pub static mut val: *mut c_void = core::ptr::null_mut();
    mas_for_each(&mas, val, module_tags.size) {
pub static mut start_tag: *mut c_void = core::ptr::null_mut();
pub static mut end_tag: *mut c_void = core::ptr::null_mut();
    if (val != &unloaded_mod) {
    continue;
    }
// Release area if all tags are unused
    start_tag = (module_tags.start_addr + mas.index);
    end_tag = (module_tags.start_addr + mas.last);
    if (clean_unused_counters(start_tag, end_tag)) {
    mas_erase(&mas);
    }
    }
    }
// Called with mod_area_mt locked
#[no_mangle]
pub unsafe extern "C" fn find_aligned_area(mas: *mut ma_state, section_size: c_ulong, size: c_ulong, prepend: c_uint, align: c_ulong) -> bool {
pub static mut cleanup_done: bool = false;
// label;
// Try finding exact size and hope the start is aligned
    if (!mas_empty_area(mas, 0, section_size - 1, prepend + size)) {
    if (IS_ALIGNED(mas.index + prepend, align)) {
    return true;
    }
// Try finding larger area to align later
    mas_reset(mas);
    if (!mas_empty_area(mas, 0, section_size - 1,
    size + prepend + align - 1)) {
    return true;
    }
    }
// No free area, try cleanup stale data and repeat the search once
    if (!cleanup_done) {
    clean_unused_module_areas_locked();
    cleanup_done = true;
    mas_reset(mas);
// goto;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vm_module_tags_populate() -> c_int {
    let mut phys_end = ALIGN_DOWN(module_tags.start_addr, PAGE_SIZE) +
    (vm_module_tags.nr_pages << PAGE_SHIFT);
pub static mut new_end: c_ulong = 0;
    if (phys_end < new_end) {
    let mut next_page = vm_module_tags.pages + vm_module_tags.nr_pages;
pub static mut old_shadow_end: c_ulong = 0;
pub static mut new_shadow_end: c_ulong = 0;
    let mut more_pages = 0;
pub static mut nr: c_ulong = 0;
    more_pages = ALIGN(new_end - phys_end, PAGE_SIZE) >> PAGE_SHIFT;
    while (nr < more_pages) {
    let mut allocated = 0;
    allocated = alloc_pages_bulk_node(GFP_KERNEL | __GFP_NOWARN,
    NUMA_NO_NODE, more_pages - nr, next_page + nr);
    if (!allocated) {
    break;
    }
    nr += allocated;
    }
    if (nr < more_pages ||
    vmap_pages_range(phys_end, phys_end + (nr << PAGE_SHIFT), PAGE_KERNEL,
    next_page, PAGE_SHIFT) < 0) {
pub static mut arg: release_pages_arg = 0;
// Clean up and error out
    release_pages(arg, nr);
    return -ENOMEM;
    }
    vm_module_tags.nr_pages += nr;
//
// Kasan allocates 1 byte of shadow for every 8 bytes of data.
// When kasan_alloc_module_shadow allocates shadow memory,
// its unit of allocation is a page.
// Therefore, here we need to align to MODULE_ALIGN.
//
    if (old_shadow_end < new_shadow_end) {
    kasan_alloc_module_shadow(old_shadow_end,
    new_shadow_end - old_shadow_end,
    GFP_KERNEL);
    }
    }
//
// Mark the pages as accessible, now that they are mapped.
// With hardware tag-based KASAN, marking is skipped for
// non-VM_ALLOC mappings, see __kasan_unpoison_vmalloc().
//
    kasan_unpoison_vmalloc(module_tags.start_addr,
    new_end - module_tags.start_addr,
    KASAN_VMALLOC_PROT_NORMAL);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn reserve_module_tags(mod: *mut module, size: c_ulong, prepend: c_uint, align: c_ulong) -> *mut c_void {
pub static mut section_size: c_ulong = 0;
    MA_STATE(mas, &mod_area_mt, 0, section_size - 1);
    let mut offset = 0;
    let mut ret = core::ptr::null_mut();
// If no tags return error
    if (size < sizeof!(alloc_tag)) {
    return ERR_PTR(-EINVAL);
    }
//
// align is always power of 2, so we can use IS_ALIGNED and ALIGN.
// align 0 or 1 means no alignment, to simplify set to 1.
//
    if (!align) {
    align = 1;
    }
    if (!ensure_alignment(align, &prepend)) {
    shutdown_mem_profiling(true);
    pr_err!("%s: alignment %lu is incompatible with allocation tag indexing. Memory allocation profiling is disabled!\n",
    mod.name, align);
    return ERR_PTR(-EINVAL);
    }
    mas_lock(&mas);
    if (!find_aligned_area(&mas, section_size, size, prepend, align)) {
    ret = ERR_PTR(-ENOMEM);
// goto;
    }
// Mark found area as reserved
    offset = mas.index;
    offset += prepend;
    offset = ALIGN(offset, align);
    if (offset != mas.index) {
pub static mut pad_start: c_ulong = 0;
    mas.last = offset - 1;
    mas_store(&mas, &prepend_mod);
    if (mas_is_err(&mas)) {
    ret = ERR_PTR(xa_err(mas.node));
// goto;
    }
    mas.index = offset;
    mas.last = offset + size - 1;
    mas_store(&mas, mod);
    if (mas_is_err(&mas)) {
    mas.index = pad_start;
    mas_erase(&mas);
    ret = ERR_PTR(xa_err(mas.node));
    }
    } else {
    mas.last = offset + size - 1;
    mas_store(&mas, mod);
    if (mas_is_err(&mas)) {
    ret = ERR_PTR(xa_err(mas.node));
    }
    }
// label;
    mas_unlock(&mas);
    if (IS_ERR(ret)) {
    return ret;
    }
    if (module_tags.size < offset + size) {
    let mut grow_res = 0;
    module_tags.size = offset + size;
    if (mem_alloc_profiling_enabled() && !tags_addressable()) {
    shutdown_mem_profiling(true);
    pr_warn!("With module %s there are too many tags to fit in %d page flag bits. Memory allocation profiling is disabled!\n",
    mod.name, NR_UNUSED_PAGEFLAG_BITS);
    }
    grow_res = vm_module_tags_populate();
    if (grow_res) {
    shutdown_mem_profiling(true);
    pr_err!("Failed to allocate memory for allocation tags in the module %s. Memory allocation profiling is disabled!\n",
    mod.name);
    return ERR_PTR(grow_res);
    }
    }
    return (module_tags.start_addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn release_module_tags(mod: *mut module, used: bool) {
    MA_STATE(mas, &mod_area_mt, module_tags.size, module_tags.size);
pub static mut start_tag: *mut c_void = core::ptr::null_mut();
pub static mut end_tag: *mut c_void = core::ptr::null_mut();
pub static mut val: *mut c_void = core::ptr::null_mut();
    mas_lock(&mas);
    mas_for_each_rev(&mas, val, 0)
    if (val == mod) {
    break;
    }
    if (!val) /* module not found */ {
// goto;
    }
    if (!used) {
// goto;
    }
    start_tag = (module_tags.start_addr + mas.index);
    end_tag = (module_tags.start_addr + mas.last);
    if (!clean_unused_counters(start_tag, end_tag)) {
pub static mut tag: *mut c_void = core::ptr::null_mut();
    while (tag <= end_tag) {
pub static mut counter: usize = 0;
    if (!tag.counters) {
    continue;
    }
    counter = alloc_tag_read(tag);
    pr_info!("%s:%u module %s func:%s has %llu allocated at module unload\n",
    tag.ct.filename, tag.ct.lineno, tag.ct.modname,
    tag.ct.function, counter.bytes);
    }
    } else {
    used = false;
    }
// label;
    mas_store(&mas, used ? &unloaded_mod : core::ptr::null_mut());
    val = mas_prev_range(&mas, 0);
    if (val == &prepend_mod) {
    mas_store(&mas, core::ptr::null_mut());
    }
// label;
    mas_unlock(&mas);
    }
#[no_mangle]
unsafe extern "C" fn load_module(mod: *mut module, start: *mut codetag, stop: *mut codetag) -> c_int {
// Allocate module alloc_tag percpu counters
pub static mut start_tag: *mut c_void = core::ptr::null_mut();
pub static mut stop_tag: *mut c_void = core::ptr::null_mut();
pub static mut tag: *mut c_void = core::ptr::null_mut();
// percpu counters for core allocations are already statically allocated
    if (!mod) {
    return 0;
    }
    start_tag = ct_to_alloc_tag(start);
    stop_tag = ct_to_alloc_tag(stop);
    while (tag < stop_tag) {
    WARN_ON!(tag.counters);
    tag.counters = alloc_percpu(alloc_tag_counters);
    if (!tag.counters) {
    while (--tag >= start_tag) {
    free_percpu(tag.counters);
    tag.counters = core::ptr::null_mut();
    }
    pr_err!("Failed to allocate memory for allocation tag percpu counters in the module %s\n",
    mod.name);
    return -ENOMEM;
    }
//
// Avoid a kmemleak false positive. The pointer to the counters is stored
// in the alloc_tag section of the module and cannot be directly accessed.
//
    kmemleak_ignore_percpu(tag.counters);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn replace_module(mod: *mut module, new_mod: *mut module) {
    MA_STATE(mas, &mod_area_mt, 0, module_tags.size);
pub static mut val: *mut c_void = core::ptr::null_mut();
    mas_lock(&mas);
    mas_for_each(&mas, val, module_tags.size) {
    if (val != mod) {
    continue;
    }
    mas_store_gfp(&mas, new_mod, GFP_KERNEL);
    break;
    }
    mas_unlock(&mas);
    }
#[no_mangle]
unsafe extern "C" fn alloc_mod_tags_mem() -> c_int {
// Map space to copy allocation tags
    vm_module_tags = execmem_vmap(MODULE_ALLOC_TAG_VMAP_SIZE);
    if (!vm_module_tags) {
    pr_err!("Failed to map %lu bytes for module allocation tags\n",
    MODULE_ALLOC_TAG_VMAP_SIZE);
    module_tags.start_addr = 0;
    return -ENOMEM;
    }
    vm_module_tags.pages = kmalloc_objs(page *,
    get_vm_area_size(vm_module_tags) >> PAGE_SHIFT,
    GFP_KERNEL | __GFP_ZERO);
    if (!vm_module_tags.pages) {
    free_vm_area(vm_module_tags);
    return -ENOMEM;
    }
    module_tags.start_addr = (unsigned long)vm_module_tags.addr;
    module_tags.end_addr = module_tags.start_addr + MODULE_ALLOC_TAG_VMAP_SIZE;
// Ensure the base is alloc_tag aligned when required for indexing
    module_tags.start_addr = alloc_tag_align(module_tags.start_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_mod_tags_mem()  {
pub static mut arg: release_pages_arg = 0;
    module_tags.start_addr = 0;
    release_pages(arg, vm_module_tags.nr_pages);
    kfree(vm_module_tags.pages);
    free_vm_area(vm_module_tags);
    }

#[no_mangle]
pub unsafe extern "C" fn alloc_mod_tags_mem() -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn free_mod_tags_mem() {}

// See: Documentation/mm/allocation-profiling.rst
#[no_mangle]
unsafe extern "C" fn setup_early_mem_profiling(str: *mut c_char) -> c_int {
pub static mut compressed: bool = false;
    let mut enable = 0;
    if (!str || !str[0]) {
    return -EINVAL;
    }
    if (!strncmp(str, "never", 5)) {
    enable = false;
    mem_profiling_support = false;
    pr_info!("Memory allocation profiling is disabled!\n");
    } else {
    let mut token = strsep(&str, ",");
    if (kstrtobool(token, &enable)) {
    return -EINVAL;
    }
    if (str) {
    if (strcmp(str, "compressed")) {
    return -EINVAL;
    }
    compressed = true;
    }
    mem_profiling_support = true;
    pr_info!("Memory allocation profiling is enabled %s compression and is turned %s!\n",
    compressed ? "with" : "without", str_on_off(enable));
    }
    if (enable != mem_alloc_profiling_enabled()) {
    if (enable) {
    static_branch_enable(&mem_alloc_profiling_key);
    }
    else {
    static_branch_disable(&mem_alloc_profiling_key);
    }
    }
    if (compressed != static_key_enabled(&mem_profiling_compressed)) {
    if (compressed) {
    static_branch_enable(&mem_profiling_compressed);
    }
    else {
    static_branch_disable(&mem_profiling_compressed);
    }
    }
    return 0;
    }
    early_param!("sysctl.vm.mem_profiling", setup_early_mem_profiling);
#[no_mangle]
unsafe extern "C" fn need_page_alloc_tagging() -> __init bool {
    if (static_key_enabled(&mem_profiling_compressed)) {
    return false;
    }
    return mem_profiling_support;
    }

//
// Track page allocations before page_ext is initialized.
// Some pages are allocated before page_ext becomes available, leaving
// their codetag uninitialized. Track these early PFNs so we can clear
// their codetag refs later to avoid warnings when they are freed.
//
// Each page is cast to a pfn_pool: the first few bytes hold metadata
// (next pointer and slot count), the remainder stores PFNs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfn_pool {
    pub next: *mut pfn_pool,
    pub count: core::sync::atomic::AtomicI32,
    pub pfns: [c_ulong; 0],
}

    sizeof!(unsigned long))
pub static mut current_pfn_pool: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn __alloc_tag_add_early_pfn(pfn: c_ulong)  {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    do {
    pool = READ_ONCE(current_pfn_pool);
    if (!pool || atomic_read(&pool.count) >= PFN_POOL_SIZE) {
    let mut new_page = __alloc_pages(__GFP_HIGH, 0, numa_mem_id(),
    core::ptr::null_mut(), ALLOC_NO_CODETAG);
pub static mut new: *mut c_void = core::ptr::null_mut();
    if (!new_page) {
    pr_warn_once("early PFN tracking page allocation failed\n");
    return;
    }
    new = page_address(new_page);
    new.next = pool;
    atomic_set(&new.count, 0);
    if (cmpxchg(&current_pfn_pool, pool, new) != pool) {
    clear_page_tag_ref(new_page);
    __free_page(new_page);
    continue;
    }
    pool = new;
    }
    idx = atomic_read(&pool.count);
    if (idx >= PFN_POOL_SIZE) {
    continue;
    }
    if (atomic_cmpxchg(&pool.count, idx, idx + 1) == idx) {
    break;
    }
    } while (1);
    pool.pfns[idx] = pfn;
    }
    typedef void alloc_tag_add_func(unsigned long pfn);
    static alloc_tag_add_func  *alloc_tag_add_early_pfn_ptr __refdata =
    RCU_INITIALIZER(__alloc_tag_add_early_pfn);
#[no_mangle]
pub unsafe extern "C" fn alloc_tag_add_early_pfn(pfn: c_ulong, alloc_flags: c_uint) {
pub static mut alloc_tag_add: *mut c_void = core::ptr::null_mut();
    if (static_key_enabled(&mem_profiling_compressed)) {
    return;
    }
// Skip allocations for the tracking list itself to avoid recursion.
    if (alloc_flags & ALLOC_NO_CODETAG) {
    return;
    }
    rcu_read_lock();
    alloc_tag_add = rcu_dereference(alloc_tag_add_early_pfn_ptr);
    if (alloc_tag_add) {
    alloc_tag_add(pfn);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn clear_early_alloc_pfn_tag_refs()  {
    let mut pool = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (static_key_enabled(&mem_profiling_compressed)) {
    return;
    }
    rcu_assign_pointer(alloc_tag_add_early_pfn_ptr, core::ptr::null_mut());
// Make sure we are not racing with __alloc_tag_add_early_pfn()
    synchronize_rcu();
    while (pool) {
pub static mut nr_pfns: c_int = 0;
    while (i < nr_pfns) {
pub static mut pfn: c_ulong = 0;
    if (pfn_valid(pfn)) {
    union pgtag_ref_handle handle;
    union codetag_ref ref;
    if (get_page_tag_ref(pfn_to_page(pfn), &ref, &handle)) {
//
// An early-allocated page could be freed and reallocated
// after its page_ext is initialized but before we clear it.
// In that case, it already has a valid tag set.
// We should not overwrite that valid tag
// with CODETAG_EMPTY.
//
// Note: there is still a small race window between checking
// ref.ct and calling set_codetag_empty(). We accept this
// race as it's unlikely and the extra complexity of atomic
// cmpxchg is not worth it for this debug-only code path.
//
    if (ref.ct) {
    put_page_tag_ref(handle);
    continue;
    }
    set_codetag_empty(&ref);
    update_page_tag_ref(handle, &ref);
    put_page_tag_ref(handle);
    }
    }
    }
    next = pool.next;
    page = virt_to_page(pool);
    clear_page_tag_ref(page);
    __free_page(page);
    }
    }

    static inline void __init clear_early_alloc_pfn_tag_refs(void) {}

#[no_mangle]
unsafe extern "C" fn init_page_alloc_tagging() -> __init void {
    clear_early_alloc_pfn_tag_refs();
    }
pub static mut page_ext_operations: usize = 0;
    EXPORT_SYMBOL(page_alloc_tagging_ops);

//
// Not using proc_do_static_key() directly to prevent enabling profiling
// after it was shut down.
//
#[no_mangle]
pub unsafe extern "C" fn proc_mem_profiling_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    if (write) {
//
// Call from do_sysctl_args() which is a no-op since the same
// value was already set by setup_early_mem_profiling.
// Return success to avoid warnings from do_sysctl_args().
//
    if (!current.mm) {
    return 0;
    }

// User can't toggle profiling while debugging
    return -EACCES;

    if (!mem_profiling_support) {
    return -EINVAL;
    }
    }
    return proc_do_static_key(table, write, buffer, lenp, ppos);
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn sysctl_init()  {
    register_sysctl_init("vm", memory_allocation_profiling_sysctls);
    }

#[no_mangle]
pub unsafe extern "C" fn sysctl_init() {}

#[no_mangle]
unsafe extern "C" fn alloc_tag_init() -> c_int {
pub static mut codetag_type_desc: usize = 0;
    let mut res = 0;
    sysctl_init();
    if (!mem_profiling_support) {
    pr_info!("Memory allocation profiling is not supported!\n");
    return 0;
    }
    if (!proc_create(ALLOCINFO_FILE_NAME, 0400, core::ptr::null_mut(), &allocinfo_proc_ops)) {
    pr_err!("Failed to create %s file\n", ALLOCINFO_FILE_NAME);
    shutdown_mem_profiling(false);
    return -ENOMEM;
    }
    res = alloc_mod_tags_mem();
    if (res) {
    pr_err!("Failed to reserve address space for module tags, errno = %d\n", res);
    shutdown_mem_profiling(true);
    return res;
    }
    alloc_tag_cttype = codetag_register_type(&desc);
    if (IS_ERR(alloc_tag_cttype)) {
    pr_err!("Allocation tags registration failed, errno = %pe\n", alloc_tag_cttype);
    free_mod_tags_mem();
    shutdown_mem_profiling(true);
    return PTR_ERR(alloc_tag_cttype);
    }
    return 0;
    }
    module_init!(alloc_tag_init);