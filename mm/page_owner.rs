//! Automatically rewritten from C to Rust
//! Source: mm/page_owner.c
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
// TODO: teach PAGE_OWNER_STACK_DEPTH (__dump_page_owner and save_stack)
// to use off stack temporal storage
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_owner {
    pub order: c_ushort,
    pub last_migrate_reason: c_short,
    pub gfp_mask: gfp_t,
    pub handle: depot_stack_handle_t,
    pub free_handle: depot_stack_handle_t,
    pub ts_nsec: u64,
    pub free_ts_nsec: u64,
    pub comm: [c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub tgid: pid_t,
    pub free_pid: pid_t,
    pub free_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack {
    pub stack_record: *mut stack_record,
    pub next: *mut stack,
}

pub static mut dummy_stack: usize = 0;
pub static mut failure_stack: usize = 0;
pub static mut stack_list: *mut c_void = core::ptr::null_mut();
pub static mut stack_list_lock: usize = 0;
pub const STACK_PRINT_FLAG_STACK: c_uint = 0x1;
pub const STACK_PRINT_FLAG_PAGES: c_uint = 0x2;
pub const STACK_PRINT_FLAG_HANDLE: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_print_ctx {
    pub stack: *mut stack,
    pub flags: u8,
}

    enum page_owner_print_mode {
    PAGE_OWNER_PRINT_STACK,
    PAGE_OWNER_PRINT_HANDLE,
    PAGE_OWNER_PRINT_STACK_HANDLE,
    };
    static const char * const page_owner_print_mode_strings[] = {
    [PAGE_OWNER_PRINT_STACK]	= "stack",
    [PAGE_OWNER_PRINT_HANDLE]	= "handle",
    [PAGE_OWNER_PRINT_STACK_HANDLE]	= "stack_handle",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_owner_filter_state {
    pub print_mode: page_owner_print_mode,
    pub nid_filter: nodemask_t,
    pub nid_filter_enabled: bool,
}

    static bool page_owner_enabled __initdata;
pub static mut page_owner_inited: usize = 0;
    static depot_stack_handle_t dummy_handle;
    static depot_stack_handle_t failure_handle;
    static depot_stack_handle_t early_handle;
// forward_decl: init_early_allocated_pages;
#[no_mangle]
pub unsafe extern "C" fn set_current_in_page_owner() {
//
// Avoid recursion.
//
// We might need to allocate more memory from page_owner code, so make
// sure to signal it in order to avoid recursion.
//
    current.in_page_owner = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn unset_current_in_page_owner() {
    current.in_page_owner = 0;
    }
#[no_mangle]
unsafe extern "C" fn early_page_owner_param(buf: *mut c_char) -> c_int {
pub static mut ret: c_int = 0;
    if (page_owner_enabled) {
    stack_depot_request_early_init();
    }
    return ret;
    }
    early_param!("page_owner", early_page_owner_param);
#[no_mangle]
unsafe extern "C" fn need_page_owner() -> __init bool {
    return page_owner_enabled;
    }
#[no_mangle]
unsafe extern "C" fn create_dummy_stack() -> __always_inline depot_stack_handle_t {
    unsigned long entries[4];
    let mut nr_entries = 0;
    nr_entries = stack_trace_save(entries, ARRAY_SIZE!(entries), 0);
    return stack_depot_save(entries, nr_entries, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn register_dummy_stack() -> noinline void {
    dummy_handle = create_dummy_stack();
    }
#[no_mangle]
unsafe extern "C" fn register_failure_stack() -> noinline void {
    failure_handle = create_dummy_stack();
    }
#[no_mangle]
unsafe extern "C" fn register_early_stack() -> noinline void {
    early_handle = create_dummy_stack();
    }
#[no_mangle]
unsafe extern "C" fn init_page_owner() -> __init void {
    if (!page_owner_enabled) {
    return;
    }
    register_dummy_stack();
    register_failure_stack();
    register_early_stack();
    init_early_allocated_pages();
// Initialize dummy and failure stacks and link them to stack_list
    dummy_stack.stack_record = __stack_depot_get_stack_record(dummy_handle);
    failure_stack.stack_record = __stack_depot_get_stack_record(failure_handle);
    if (dummy_stack.stack_record) {
    refcount_set(&dummy_stack.stack_record.count, 1);
    }
    if (failure_stack.stack_record) {
    refcount_set(&failure_stack.stack_record.count, 1);
    }
    dummy_stack.next = &failure_stack;
    stack_list = &dummy_stack;
    static_branch_enable(&page_owner_inited);
    }
pub static mut page_ext_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn get_page_owner(page_ext: *mut page_ext) -> *mut c_void {
    return page_ext_data(page_ext, &page_owner_ops);
    }
#[no_mangle]
unsafe extern "C" fn save_stack(flags: gfp_t) -> noinline depot_stack_handle_t {
    unsigned long entries[PAGE_OWNER_STACK_DEPTH];
    let mut handle;
    let mut nr_entries = 0;
    if (current.in_page_owner) {
    return dummy_handle;
    }
    set_current_in_page_owner();
    nr_entries = stack_trace_save(entries, ARRAY_SIZE!(entries), 2);
    handle = stack_depot_save(entries, nr_entries, flags);
    if (!handle) {
    handle = failure_handle;
    }
    unset_current_in_page_owner();
    return handle;
    }
#[no_mangle]
pub unsafe extern "C" fn add_stack_record_to_list(stack_record: *mut stack_record, gfp_mask: gfp_t) {
    let mut flags = 0;
pub static mut stack: *mut c_void = core::ptr::null_mut();
    if (!gfpflags_allow_spinning(gfp_mask)) {
    return;
    }
    set_current_in_page_owner();
    stack = kmalloc_obj(*stack, gfp_nested_mask(gfp_mask));
    if (!stack) {
    unset_current_in_page_owner();
    return;
    }
    unset_current_in_page_owner();
    stack.stack_record = stack_record;
    stack.next = core::ptr::null_mut();
    spin_lock_irqsave(&stack_list_lock, flags);
    stack.next = stack_list;
//
// This pairs with smp_load_acquire() from function
// stack_start(). This guarantees that stack_start()
// will see an updated stack_list before starting to
// traverse the list.
//
    smp_store_release(&stack_list, stack);
    spin_unlock_irqrestore(&stack_list_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_stack_record_count(handle: depot_stack_handle_t, gfp_mask: gfp_t, nr_base_pages: c_int) {
    let mut stack_record = __stack_depot_get_stack_record(handle);
    if (!stack_record) {
    return;
    }
//
// New stack_record's that do not use STACK_DEPOT_FLAG_GET start
// with REFCOUNT_SATURATED to catch spurious increments of their
// refcount.
// Since we do not use STACK_DEPOT_FLAG_GET API, let us
// set a refcount of 1 ourselves.
//
    if (refcount_read(&stack_record.count) == REFCOUNT_SATURATED) {
pub static mut old: c_int = 0;
    if (atomic_try_cmpxchg_relaxed(&stack_record.count.refs, &old, 1)) {
// Add the new stack_record to our list
    add_stack_record_to_list(stack_record, gfp_mask);
    }
    }
    refcount_add(nr_base_pages, &stack_record.count);
    }
#[no_mangle]
pub unsafe extern "C" fn dec_stack_record_count(handle: depot_stack_handle_t, nr_base_pages: c_int) {
    let mut stack_record = __stack_depot_get_stack_record(handle);
    if (!stack_record) {
    return;
    }
    if (refcount_sub_and_test(nr_base_pages, &stack_record.count)) {
    pr_warn!("%s: refcount went to 0 for %u handle\n", __func__,
    handle);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __update_page_owner_handle(page: *mut page, handle: depot_stack_handle_t, order: c_ushort, gfp_mask: gfp_t, last_migrate_reason: c_short, ts_nsec: u64, pid: pid_t, tgid: pid_t, comm: *mut c_char) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    for_each_page_ext(page, 1 << order, page_ext, iter) {
    page_owner = get_page_owner(page_ext);
    page_owner.handle = handle;
    page_owner.order = order;
    page_owner.gfp_mask = gfp_mask;
    page_owner.last_migrate_reason = last_migrate_reason;
    page_owner.pid = pid;
    page_owner.tgid = tgid;
    page_owner.ts_nsec = ts_nsec;
    strscpy(page_owner.comm, comm,
    sizeof!(page_owner.comm));
    __set_bit(PAGE_EXT_OWNER, &page_ext.flags);
    __set_bit(PAGE_EXT_OWNER_ALLOCATED, &page_ext.flags);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __update_page_owner_free_handle(page: *mut page, handle: depot_stack_handle_t, order: c_ushort, pid: pid_t, tgid: pid_t, free_ts_nsec: u64) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    for_each_page_ext(page, 1 << order, page_ext, iter) {
    page_owner = get_page_owner(page_ext);
// Only __reset_page_owner() wants to clear the bit
    if (handle) {
    __clear_bit(PAGE_EXT_OWNER_ALLOCATED, &page_ext.flags);
    page_owner.free_handle = handle;
    }
    page_owner.free_ts_nsec = free_ts_nsec;
    page_owner.free_pid = current.pid;
    page_owner.free_tgid = current.tgid;
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __reset_page_owner(page: *mut page, order: c_ushort) {
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
    let mut handle;
    let mut alloc_handle;
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
pub static mut free_ts_nsec: u64 = 0;
    page_ext = page_ext_get(page);
    if (unlikely(!page_ext)) {
    return;
    }
    page_owner = get_page_owner(page_ext);
    alloc_handle = page_owner.handle;
    page_ext_put(page_ext);
//
// Do not specify GFP_NOWAIT to make gfpflags_allow_spinning() == false
// to prevent issues in stack_depot_save().
// This is similar to alloc_pages_nolock() gfp flags, but only used
// to signal stack_depot to avoid spin_locks.
//
    handle = save_stack(__GFP_NOWARN);
    __update_page_owner_free_handle(page, handle, order, current.pid,
    current.tgid, free_ts_nsec);
    if (alloc_handle != early_handle) {
//
// early_handle is being set as a handle for all those
// early allocated pages. See init_pages_in_zone().
// Since their refcount is not being incremented because
// the machinery is not ready yet, we cannot decrement
// their refcount either.
//
    dec_stack_record_count(alloc_handle, 1 << order);
    }
    }
    noinline void __set_page_owner(page *page, unsigned short order,
    gfp_t gfp_mask)
    {
pub static mut ts_nsec: u64 = 0;
    let mut handle;
    handle = save_stack(gfp_mask);
    __update_page_owner_handle(page, handle, order, gfp_mask, MR_NEVER,
    ts_nsec, current.pid, current.tgid,
    current.comm);
    inc_stack_record_count(handle, gfp_mask, 1 << order);
    }
#[no_mangle]
pub unsafe extern "C" fn __folio_set_owner_migrate_reason(folio: *mut folio, reason: migrate_reason) {
    let mut page_ext = page_ext_get(&folio.page);
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    if (unlikely(!page_ext)) {
    return;
    }
    page_owner = get_page_owner(page_ext);
    page_owner.last_migrate_reason = reason;
    page_ext_put(page_ext);
    }
#[no_mangle]
pub unsafe extern "C" fn __split_page_owner(page: *mut page, old_order: c_int, new_order: c_int) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    for_each_page_ext(page, 1 << old_order, page_ext, iter) {
    page_owner = get_page_owner(page_ext);
    page_owner.order = new_order;
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __folio_copy_owner(newfolio: *mut folio, old: *mut folio) {
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
pub static mut old_page_owner: *mut c_void = core::ptr::null_mut();
pub static mut new_page_owner: *mut c_void = core::ptr::null_mut();
    let mut migrate_handle;
    page_ext = page_ext_get(&old.page);
    if (unlikely(!page_ext)) {
    return;
    }
    old_page_owner = get_page_owner(page_ext);
    page_ext_put(page_ext);
    page_ext = page_ext_get(&newfolio.page);
    if (unlikely(!page_ext)) {
    return;
    }
    new_page_owner = get_page_owner(page_ext);
    page_ext_put(page_ext);
    migrate_handle = new_page_owner.handle;
    __update_page_owner_handle(&newfolio.page, old_page_owner.handle,
    old_page_owner.order, old_page_owner.gfp_mask,
    old_page_owner.last_migrate_reason,
    old_page_owner.ts_nsec, old_page_owner.pid,
    old_page_owner.tgid, old_page_owner.comm);
//
// Do not proactively clear PAGE_EXT_OWNER{_ALLOCATED} bits as the folio
// will be freed after migration. Keep them until then as they may be
// useful.
//
    __update_page_owner_free_handle(&newfolio.page, 0, old_page_owner.order,
    old_page_owner.free_pid,
    old_page_owner.free_tgid,
    old_page_owner.free_ts_nsec);
//
// We linked the original stack to the new folio, we need to do the same
// for the new one and the old folio otherwise there will be an imbalance
// when subtracting those pages from the stack.
//
    rcu_read_lock();
    for_each_page_ext(&old.page, 1 << new_page_owner.order, page_ext, iter) {
    old_page_owner = get_page_owner(page_ext);
    old_page_owner.handle = migrate_handle;
    }
    rcu_read_unlock();
    }
//
// Check if a page is a buddy page and advance @pfn past the entire buddy block.
// This safely reads the buddy order without the zone lock, which may cause us
// to skip less than the full buddy block, but that is acceptable for page owner
// iteration purposes.
//
// The lockless read of buddy_order_unsafe() can also return a garbage order if
// the page is concurrently allocated and PageBuddy is cleared between the check
// and the read. Clamp the advance at the next MAX_ORDER_NR_PAGES boundary so
// that a bogus order cannot carry @pfn into an unvalidated memory section,
// which would break callers that rely on boundary-aligned pfn_valid() checks.
//
// Return: true if the page was skipped (caller should continue its loop),
// false if the page is not a buddy page and should be processed normally.
//
#[no_mangle]
pub unsafe extern "C" fn skip_buddy_pages(pfn: *mut c_ulong, page: *mut page) -> bool {
    let mut order = 0;
    if (!PageBuddy(page)) {
    return false;
    }
    order = buddy_order_unsafe(page);
    if (order <= MAX_PAGE_ORDER) {
pub static mut new_pfn: c_ulong = 0;
pub static mut boundary: c_ulong = 0;
// pfn = min(new_pfn, boundary) - 1;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn pagetypeinfo_showmixedcount_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, block_end_pfn;
pub static mut end_pfn: c_ulong = 0;
    unsigned long count[MIGRATE_TYPES] = { 0, };
    let mut pageblock_mt = 0;
    let mut page_mt = 0;
    let mut i = 0;
// Scan block by block. First and last block may be incomplete
    pfn = zone.zone_start_pfn;
//
// Walk the zone in pageblock_nr_pages steps. If a page block spans
// a zone boundary, it will be double counted between zones. This does
// not matter as the mixed block count will still be correct
//
    while (pfn < end_pfn) {
    page = pfn_to_online_page(pfn);
    if (!page) {
    pfn = ALIGN(pfn + 1, MAX_ORDER_NR_PAGES);
    continue;
    }
    block_end_pfn = pageblock_end_pfn(pfn);
    block_end_pfn = min(block_end_pfn, end_pfn);
    pageblock_mt = get_pageblock_migratetype(page);
    while (pfn < block_end_pfn) {
// The pageblock is online, no need to recheck.
    page = pfn_to_page(pfn);
    if (page_zone(page) != zone) {
    continue;
    }
    if (skip_buddy_pages(&pfn, page)) {
    continue;
    }
    if (PageReserved(page)) {
    continue;
    }
    page_ext = page_ext_get(page);
    if (unlikely(!page_ext)) {
    continue;
    }
    if (!test_bit(PAGE_EXT_OWNER_ALLOCATED, &page_ext.flags)) {
// goto;
    }
    page_owner = get_page_owner(page_ext);
    page_mt = gfp_migratetype(page_owner.gfp_mask);
    if (pageblock_mt != page_mt) {
    if (is_migrate_cma(pageblock_mt)) {
    count[MIGRATE_MOVABLE]++;
    }
    else {
    count[pageblock_mt]++;
    }
    pfn = block_end_pfn;
    page_ext_put(page_ext);
    break;
    }
    pfn += (1UL << page_owner.order) - 1;
// label;
    page_ext_put(page_ext);
    }
    }
// Print counts
    seq_printf(m, "Node %d, zone %8s ", pgdat.node_id, zone.name);
    for (i = 0; i < MIGRATE_TYPES; i++) {
    seq_printf(m, "%12lu ", count[i]);
    }
    seq_putc(m, '\n');
    }

//
// Looking for memcg information and print it out
//
#[no_mangle]
pub unsafe extern "C" fn print_page_owner_memcg(kbuf: *mut c_char, count: size_t, ret: c_int, page: *mut page) -> c_int {
    let mut memcg_data = 0;
pub static mut objcg: *mut c_void = core::ptr::null_mut();
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut online = 0;
    char name[80];
    rcu_read_lock();
    memcg_data = READ_ONCE(page.memcg_data);
    if (!memcg_data || PageTail(page)) {
// goto;
    }
    if (memcg_data & MEMCG_DATA_OBJEXTS) {
    ret += scnprintf(kbuf + ret, count - ret,
    "Slab cache page\n");
// goto;
    }
    objcg = (memcg_data & ~OBJEXTS_FLAGS_MASK);
    memcg = objcg ? obj_cgroup_memcg(objcg) : core::ptr::null_mut();
    if (!memcg) {
// goto;
    }
    online = css_is_online(&memcg.css);
    cgroup_name(memcg.css.cgroup, name, sizeof!(name));
    ret += scnprintf(kbuf + ret, count - ret,
    "Charged %sto %smemcg %s\n",
    (memcg_data & MEMCG_DATA_KMEM) ? "(via objcg) " : "",
    online ? "" : "offline ",
    name);
// label;
    rcu_read_unlock();
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: print_page_owner_memcg
pub unsafe extern "C" fn print_page_owner_memcg_dup(kbuf: *mut c_char, count: size_t, ret: c_int, page: *mut page) -> c_int {
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn print_page_owner(buf: *mut c_char, count: size_t, pfn: c_ulong, page: *mut page, page_owner: *mut page_owner, handle: depot_stack_handle_t, state: *mut page_owner_filter_state) -> ssize_t {
    let mut ret = 0;
    let mut pageblock_mt = 0;
    let mut page_mt = 0;
pub static mut kbuf: *mut c_void = core::ptr::null_mut();
    enum page_owner_print_mode print_mode;
    count = min_t(size_t, count, PAGE_SIZE);
    kbuf = kmalloc(count, GFP_KERNEL);
    if (!kbuf) {
    return -ENOMEM;
    }
    print_mode = state.print_mode;
    ret = scnprintf(kbuf, count,
    "Page allocated via order %u, mask %#x(%pGg), pid %d, tgid %d (%s), ts %llu ns\n",
    page_owner.order, page_owner.gfp_mask,
    &page_owner.gfp_mask, page_owner.pid,
    page_owner.tgid, page_owner.comm,
    page_owner.ts_nsec);
// Print information relevant to grouping pages by mobility
    pageblock_mt = get_pageblock_migratetype(page);
    page_mt  = gfp_migratetype(page_owner.gfp_mask);
    ret += scnprintf(kbuf + ret, count - ret,
    "PFN 0x%lx type %s Block %lu type %s Flags %pGp\n",
    pfn,
    migratetype_names[page_mt],
    pfn >> pageblock_order,
    migratetype_names[pageblock_mt],
    &page.flags.f);
    if (print_mode != PAGE_OWNER_PRINT_HANDLE) {
    ret += stack_depot_snprint(handle, kbuf + ret, count - ret, 0);
    if (ret >= count) {
// goto;
    }
    }
    if (print_mode != PAGE_OWNER_PRINT_STACK) {
    ret += scnprintf(kbuf + ret, count - ret, "handle: %u\n",
    handle);
    if (ret >= count) {
// goto;
    }
    }
    if (page_owner.last_migrate_reason != MR_NEVER) {
    ret += scnprintf(kbuf + ret, count - ret,
    "Page has been migrated, last migrate reason: %s\n",
    migrate_reason_names[page_owner.last_migrate_reason]);
    }
    ret = print_page_owner_memcg(kbuf, count, ret, page);
    ret += snprintf(kbuf + ret, count - ret, "\n");
    if (ret >= count) {
// goto;
    }
    if (copy_to_user(buf, kbuf, ret)) {
    ret = -EFAULT;
    }
    kfree(kbuf);
    return ret;
// label;
    kfree(kbuf);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn __dump_page_owner(page: *const page) {
    let mut page_ext = page_ext_get(page);
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    let mut handle;
    let mut gfp_mask;
    let mut mt = 0;
    if (unlikely(!page_ext)) {
    pr_alert("There is not page extension available.\n");
    return;
    }
    page_owner = get_page_owner(page_ext);
    gfp_mask = page_owner.gfp_mask;
    mt = gfp_migratetype(gfp_mask);
    if (!test_bit(PAGE_EXT_OWNER, &page_ext.flags)) {
    pr_alert("page_owner info is not present (never set?)\n");
    page_ext_put(page_ext);
    return;
    }
    if (test_bit(PAGE_EXT_OWNER_ALLOCATED, &page_ext.flags)) {
    pr_alert("page_owner tracks the page as allocated\n");
    }
    else {
    pr_alert("page_owner tracks the page as freed\n");
    }
    pr_alert("page last allocated via order %u, migratetype %s, gfp_mask %#x(%pGg), pid %d, tgid %d (%s), ts %llu\n",
    page_owner.order, migratetype_names[mt], gfp_mask, &gfp_mask,
    page_owner.pid, page_owner.tgid, page_owner.comm,
    page_owner.ts_nsec);
    handle = READ_ONCE(page_owner.handle);
    if (!handle) {
    pr_alert("page_owner allocation stack trace missing\n");
    }
    else {
    stack_depot_print(handle);
    }
    handle = READ_ONCE(page_owner.free_handle);
    if (!handle) {
    pr_alert("page_owner free stack trace missing\n");
    } else {
    pr_alert("page last free pid %d tgid %d ts %llu stack trace:\n",
    page_owner.free_pid, page_owner.free_tgid,
    page_owner.free_ts_nsec);
    stack_depot_print(handle);
    }
    if (page_owner.last_migrate_reason != MR_NEVER) {
    pr_alert("page has been migrated, last migrate reason: %s\n",
    migrate_reason_names[page_owner.last_migrate_reason]);
    }
    page_ext_put(page_ext);
    }
#[no_mangle]
pub unsafe extern "C" fn read_page_owner(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut pfn = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page_owner: *mut c_void = core::ptr::null_mut();
    let mut handle;
    let mut state = file.private_data;
    if (!static_branch_unlikely(&page_owner_inited)) {
    return -EINVAL;
    }
    page = core::ptr::null_mut();
    if (*ppos == 0) {
    pfn = min_low_pfn;
    }
    else {
    pfn = *ppos;
    }
// Find a valid PFN or the start of a MAX_ORDER_NR_PAGES area
    while (!pfn_valid(pfn) && (pfn & (MAX_ORDER_NR_PAGES - 1)) != 0) {
    pfn += 1;
    }
// Find an allocated page
    while (pfn < max_pfn) {
//
// This temporary page_owner is required so
// that we can avoid the context switches while holding
// the rcu lock and copying the page owner information to
// user through copy_to_user() or GFP_KERNEL allocations.
//
pub static mut page_owner_tmp: usize = 0;
//
// If the new page is in a new MAX_ORDER_NR_PAGES area,
// validate the area as existing, skip it if not
//
    if ((pfn & (MAX_ORDER_NR_PAGES - 1)) == 0 && !pfn_valid(pfn)) {
    pfn += MAX_ORDER_NR_PAGES - 1;
    continue;
    }
    page = pfn_to_page(pfn);
    if (skip_buddy_pages(&pfn, page)) {
    continue;
    }
    page_ext = page_ext_get(page);
    if (unlikely(!page_ext)) {
    continue;
    }
//
// Some pages could be missed by concurrent allocation or free,
// because we don't hold the zone lock.
//
    if (!test_bit(PAGE_EXT_OWNER, &page_ext.flags)) {
// goto;
    }
//
// Although we do have the info about past allocation of free
// pages, it's not relevant for current memory usage.
//
    if (!test_bit(PAGE_EXT_OWNER_ALLOCATED, &page_ext.flags)) {
// goto;
    }
    page_owner = get_page_owner(page_ext);
//
// Don't print "tail" pages of high-order allocations as that
// would inflate the stats.
//
    if (!IS_ALIGNED(pfn, 1 << page_owner.order)) {
// goto;
    }
//
// Access to page_ext->handle isn't synchronous so we should
// be careful to access it.
//
    handle = READ_ONCE(page_owner.handle);
    if (!handle) {
// goto;
    }
    if (state.nid_filter_enabled) {
    let mut nid = 0;
pub static mut page_flags: memdesc_flags_t = 0;
//
// Bypass PF_POISONED_CHECK() in page_to_nid() to avoid
// VM_BUG_ON when accessing poisoned pages.
//
    if (page_flags.f == PAGE_POISON_PATTERN) {
// goto;
    }
    nid = memdesc_nid(&page_flags);
    if (!node_isset(nid, state.nid_filter)) {
// goto;
    }
    }
// Record the next PFN to read in the file offset
// ppos = pfn + 1;
    page_owner_tmp = *page_owner;
    page_ext_put(page_ext);
    return print_page_owner(buf, count, pfn, page,
    &page_owner_tmp, handle, state);
// label;
    page_ext_put(page_ext);
    cond_resched();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lseek_page_owner(file: *mut file, offset: loff_t, orig: c_int) -> loff_t {
    match (orig) {
    SEEK_SET => {
    file.f_pos = offset;
    // break;
    }
    SEEK_CUR => {
    file.f_pos += offset;
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return file.f_pos;
    }
#[no_mangle]
unsafe extern "C" fn init_pages_in_zone(zone: *mut zone) {
pub static mut pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
pub static mut count: c_ulong = 0;
//
// Walk the zone in pageblock_nr_pages steps. If a page block spans
// a zone boundary, it will be double counted between zones. This does
// not matter as the mixed block count will still be correct
//
    while (pfn < end_pfn) {
    let mut block_end_pfn = 0;
    if (!pfn_valid(pfn)) {
    pfn = ALIGN(pfn + 1, MAX_ORDER_NR_PAGES);
    continue;
    }
    block_end_pfn = pageblock_end_pfn(pfn);
    block_end_pfn = min(block_end_pfn, end_pfn);
    while (pfn < block_end_pfn) {
    let mut page = pfn_to_page(pfn);
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
    if (page_zone(page) != zone) {
    continue;
    }
    if (skip_buddy_pages(&pfn, page)) {
    continue;
    }
    if (PageReserved(page)) {
    continue;
    }
    page_ext = page_ext_get(page);
    if (unlikely(!page_ext)) {
    continue;
    }
// Maybe overlapping zone
    if (test_bit(PAGE_EXT_OWNER, &page_ext.flags)) {
// goto;
    }
// Found early allocated page
    __update_page_owner_handle(page, early_handle, 0, 0,
    MR_NEVER, local_clock(), current.pid,
    current.tgid, current.comm);
    count += 1;
// label;
    page_ext_put(page_ext);
    }
    cond_resched();
    }
    pr_info!("Node %d, zone %8s: page owner found early allocated %lu pages\n",
    zone.zone_pgdat.node_id, zone.name, count);
    }
#[no_mangle]
unsafe extern "C" fn init_early_allocated_pages() {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    init_pages_in_zone(zone);
    }
    }
#[no_mangle]
unsafe extern "C" fn page_owner_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut state: *mut c_void = core::ptr::null_mut();
    state = kzalloc_obj(*state);
    if (!state) {
    return -ENOMEM;
    }
    state.print_mode = PAGE_OWNER_PRINT_STACK;
    nodes_clear(state.nid_filter);
    state.nid_filter_enabled = false;
    file.private_data = state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn page_owner_release(inode: *mut inode, file: *mut file) -> c_int {
    kfree(file.private_data);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn page_owner_write(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut kbuf: *mut c_void = core::ptr::null_mut();
pub static mut orig: *mut c_void = core::ptr::null_mut();
pub static mut token: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut state = file.private_data;
    enum page_owner_print_mode new_print_mode;
    let mut new_nid_filter;
    let mut new_nid_filter_enabled = 0;
//
// Maximum input length for filter commands:
// - 32: print_mode command max length is 17 ("mode=stack_handle")
// with sufficient buffer
// - 6 * MAX_NUMNODES: worst case for nid list
// Worst case per node: ",NNNNN" (comma + 5-digit node number) = 6 bytes
//
    if (count > 32 + 6 * MAX_NUMNODES) {
    return -EINVAL;
    }
    kbuf = memdup_user_nul(buf, count);
    if (IS_ERR(kbuf)) {
    return PTR_ERR(kbuf);
    }
    orig = kbuf;
    new_print_mode = state.print_mode;
    new_nid_filter = state.nid_filter;
    new_nid_filter_enabled = state.nid_filter_enabled;
    while ((token = strsep(&kbuf, " \t\n")) != core::ptr::null_mut()) {
    if (*token == '\0') {
    continue;
    }
    if (!strncmp(token, "mode=", 5)) {
    ret = sysfs_match_string(page_owner_print_mode_strings,
    token + 5);
    if (ret < 0) {
// goto;
    }
    new_print_mode = ret;
    } else if (!strncmp(token, "nid=", 4)) {
    ret = nodelist_parse(token + 4, new_nid_filter);
    if (ret < 0) {
// goto;
    }
    if (nodes_empty(new_nid_filter)) {
    ret = -EINVAL;
// goto;
    }
//
// We want to filter memory allocations by numa nodes, so make sure
// that the specified nodes have memory.
//
    if (!nodes_subset(new_nid_filter, node_states[N_MEMORY])) {
    ret = -EINVAL;
// goto;
    }
    new_nid_filter_enabled = true;
    } else {
    ret = -EINVAL;
// goto;
    }
    }
// Commit all filter changes
    state.print_mode = new_print_mode;
    state.nid_filter = new_nid_filter;
    state.nid_filter_enabled = new_nid_filter_enabled;
    ret = count;
// label;
    kfree(orig);
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn stack_start(m: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
pub static mut stack: *mut c_void = core::ptr::null_mut();
    let mut ctx = m.private;
    if (*ppos == -1UL) {
    return core::ptr::null_mut();
    }
    if (!*ppos) {
//
// This pairs with smp_store_release() from function
// add_stack_record_to_list(), so we get a consistent
// value of stack_list.
//
    stack = smp_load_acquire(&stack_list);
    ctx.stack = stack;
    } else {
    stack = ctx.stack;
    }
    return stack;
    }
#[no_mangle]
pub unsafe extern "C" fn stack_next(m: *mut seq_file, v: *mut c_void, ppos: *mut loff_t) -> *mut c_void {
    let mut stack = v;
    let mut ctx = m.private;
    stack = stack.next;
// ppos = stack ? *ppos + 1 : -1UL;
    ctx.stack = stack;
    return stack;
    }
    static unsigned long pages_threshold;
#[no_mangle]
unsafe extern "C" fn stack_print(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut nr_base_pages = 0;
    let mut stack = v;
pub static mut entries: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
    let mut stack_record = stack.stack_record;
    let mut ctx = m.private;
    if (!stack.stack_record) {
    return 0;
    }
    nr_base_pages = refcount_read(&stack_record.count) - 1;
    if (ctx.flags & STACK_PRINT_FLAG_PAGES &&
    (nr_base_pages < 1 || nr_base_pages < pages_threshold)) {
    return 0;
    }
    if (ctx.flags & STACK_PRINT_FLAG_STACK) {
    nr_entries = stack_record.size;
    entries = stack_record.entries;
    for (i = 0; i < nr_entries; i++) {
    seq_printf(m, " %pS\n", entries[i]);
    }
    }
    if (ctx.flags & STACK_PRINT_FLAG_HANDLE) {
    seq_printf(m, "handle: %d\n", stack_record.handle.handle);
    }
    if (ctx.flags & STACK_PRINT_FLAG_PAGES) {
    seq_printf(m, "nr_base_pages: %d\n", nr_base_pages);
    }
    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stack_stop(m: *mut seq_file, v: *mut c_void) {
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn stack_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = seq_open_private(file, &stack_op,
    sizeof!(stack_print_ctx));
    if (!ret) {
    let mut m = file.private_data;
    let mut ctx = m.private;
    ctx.flags = (uintptr_t) inode.i_private;
    }
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn threshold_get(data: *mut c_void, val: *mut u64) -> c_int {
// val = READ_ONCE(pages_threshold);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn threshold_set(data: *mut c_void, val: u64) -> c_int {
    WRITE_ONCE(pages_threshold, val);
    return 0;
    }
pub static mut threshold_fops: usize = 0;
#[no_mangle]
unsafe extern "C" fn pageowner_init() -> c_int {
pub static mut dir: *mut c_void = core::ptr::null_mut();
    if (!static_branch_unlikely(&page_owner_inited)) {
    pr_info!("page_owner is disabled\n");
    return 0;
    }
    debugfs_create_file("page_owner", 0600, core::ptr::null_mut(), core::ptr::null_mut(), &page_owner_fops);
    dir = debugfs_create_dir("page_owner_stacks", core::ptr::null_mut());
    debugfs_create_file("show_stacks", 0400, dir,
    (STACK_PRINT_FLAG_STACK |
    STACK_PRINT_FLAG_PAGES),
    &stack_fops);
    debugfs_create_file("show_handles", 0400, dir,
    (STACK_PRINT_FLAG_HANDLE |
    STACK_PRINT_FLAG_PAGES),
    &stack_fops);
    debugfs_create_file("show_stacks_handles", 0400, dir,
    (STACK_PRINT_FLAG_STACK |
    STACK_PRINT_FLAG_HANDLE),
    &stack_fops);
    debugfs_create_file("count_threshold", 0600, dir, core::ptr::null_mut(),
    &threshold_fops);
    return 0;
    }
    late_initcall!(pageowner_init)