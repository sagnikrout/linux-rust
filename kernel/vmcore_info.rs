//! Automatically rewritten from C to Rust
//! Source: kernel/vmcore_info.c
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
//
// crash.c - kernel crash support code.
// Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
//

// vmcoreinfo stuff
pub static mut vmcoreinfo_data: *mut c_void = core::ptr::null_mut();
    let mut vmcoreinfo_size = 0;
pub static mut vmcoreinfo_note: *mut c_void = core::ptr::null_mut();
// trusted vmcoreinfo, e.g. we can make a copy in the crash memory
pub static mut vmcoreinfo_data_safecopy: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwerr_info {
    pub count: core::sync::atomic::AtomicI32,
    pub timestamp: time64_t,
}

//
// The hwerr_data[] array is declared with global scope so that it remains
// accessible to vmcoreinfo even when Link Time Optimization (LTO) is enabled.
//
    struct hwerr_info hwerr_data[HWERR_RECOV_MAX];
    Elf_Word *append_elf_note(Elf_Word *buf, char *name, unsigned int type,
    void *data, size_t data_len)
    {
    let mut note = buf;
    note.n_namesz = strlen(name) + 1;
    note.n_descsz = data_len;
    note.n_type   = type;
    buf += DIV_ROUND_UP(sizeof!(*note), sizeof!(Elf_Word));
    memcpy(buf, name, note.n_namesz);
    buf += DIV_ROUND_UP(note.n_namesz, sizeof!(Elf_Word));
    memcpy(buf, data, data_len);
    buf += DIV_ROUND_UP(data_len, sizeof!(Elf_Word));
    return buf;
    }
#[no_mangle]
pub unsafe extern "C" fn final_note(buf: *mut Elf_Word) {
    memset(buf, 0, sizeof!(elf_note));
    }
#[no_mangle]
unsafe extern "C" fn update_vmcoreinfo_note() {
    let mut buf = vmcoreinfo_note;
    if (!vmcoreinfo_size) {
    return;
    }
    buf = append_elf_note(buf, VMCOREINFO_NOTE_NAME, 0, vmcoreinfo_data,
    vmcoreinfo_size);
    final_note(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn crash_update_vmcoreinfo_safecopy(ptr: *mut c_void) {
    if (ptr) {
    memcpy(ptr, vmcoreinfo_data, vmcoreinfo_size);
    }
    vmcoreinfo_data_safecopy = ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_save_vmcoreinfo() {
    if (!vmcoreinfo_note) {
    return;
    }
// Use the safe copy to generate vmcoreinfo note if have
    if (vmcoreinfo_data_safecopy) {
    vmcoreinfo_data = vmcoreinfo_data_safecopy;
    }
    vmcoreinfo_append_str("CRASHTIME=%lld\n", ktime_get_real_seconds());
    update_vmcoreinfo_note();
    }
#[no_mangle]
pub unsafe extern "C" fn vmcoreinfo_append_str(fmt: *const c_char, ...) {
    let mut args;
    char buf[0x50];
    let mut r = 0;
    va_start(args, fmt);
    r = vscnprintf(buf, sizeof!(buf), fmt, args);
    va_end(args);
    r = min(r, (size_t)VMCOREINFO_BYTES - vmcoreinfo_size);
    memcpy(&vmcoreinfo_data[vmcoreinfo_size], buf, r);
    vmcoreinfo_size += r;
    WARN_ONCE(vmcoreinfo_size == VMCOREINFO_BYTES,
    "vmcoreinfo data exceeds allocated size, truncating");
    }
//
// provide an empty default implementation here -- architecture
// code may override this
//
#[no_mangle]
pub unsafe extern "C" fn arch_crash_save_vmcoreinfo() -> void __weak {}
#[no_mangle]
pub unsafe extern "C" fn paddr_vmcoreinfo_note() -> phys_addr_t __weak {
    return __pa(vmcoreinfo_note);
    }
    EXPORT_SYMBOL(paddr_vmcoreinfo_note);
#[no_mangle]
pub unsafe extern "C" fn hwerr_log_error_type(src: hwerr_error_type) {
    if (src < 0 || src >= HWERR_RECOV_MAX) {
    return;
    }
    atomic_inc(&hwerr_data[src].count);
    WRITE_ONCE(hwerr_data[src].timestamp, ktime_get_real_seconds());
    }
    EXPORT_SYMBOL_GPL(hwerr_log_error_type);
#[no_mangle]
unsafe extern "C" fn crash_save_vmcoreinfo_init() -> c_int {
    let mut order = 0;
    order = get_order(VMCOREINFO_BYTES);
    vmcoreinfo_data = __get_free_pages(GFP_KERNEL | __GFP_ZERO, order);
    if (!vmcoreinfo_data) {
    pr_warn!("Memory allocation for vmcoreinfo_data failed\n");
    return -ENOMEM;
    }
    vmcoreinfo_note = alloc_pages_exact(VMCOREINFO_NOTE_SIZE,
    GFP_KERNEL | __GFP_ZERO);
    if (!vmcoreinfo_note) {
    free_pages((unsigned long)vmcoreinfo_data, order);
    vmcoreinfo_data = core::ptr::null_mut();
    pr_warn!("Memory allocation for vmcoreinfo_note failed\n");
    return -ENOMEM;
    }
    VMCOREINFO_OSRELEASE(init_uts_ns.name.release);
    VMCOREINFO_BUILD_ID();
    VMCOREINFO_PAGESIZE(PAGE_SIZE);
    VMCOREINFO_SYMBOL(init_uts_ns);
    VMCOREINFO_OFFSET(uts_namespace, name);
    VMCOREINFO_SYMBOL(node_online_map);

    VMCOREINFO_SYMBOL_ARRAY(swapper_pg_dir);

    VMCOREINFO_SYMBOL(_stext);
    vmcoreinfo_append_str("NUMBER(VMALLOC_START)=0x%lx\n", (unsigned long) VMALLOC_START);

    VMCOREINFO_SYMBOL(mem_map);
    VMCOREINFO_SYMBOL(contig_page_data);

    VMCOREINFO_SYMBOL_ARRAY(vmemmap);

    VMCOREINFO_SYMBOL_ARRAY(mem_section);
    VMCOREINFO_LENGTH(mem_section, NR_SECTION_ROOTS);
    VMCOREINFO_STRUCT_SIZE(mem_section);
    VMCOREINFO_OFFSET(mem_section, section_mem_map);
    VMCOREINFO_NUMBER(SECTION_SIZE_BITS);
    VMCOREINFO_NUMBER(MAX_PHYSMEM_BITS);

    VMCOREINFO_STRUCT_SIZE(page);
    VMCOREINFO_STRUCT_SIZE(pglist_data);
    VMCOREINFO_STRUCT_SIZE(zone);
    VMCOREINFO_STRUCT_SIZE(free_area);
    VMCOREINFO_STRUCT_SIZE(list_head);
    VMCOREINFO_SIZE(nodemask_t);
    VMCOREINFO_OFFSET(page, flags);
    VMCOREINFO_OFFSET(page, _refcount);
    VMCOREINFO_OFFSET(page, mapping);
    VMCOREINFO_OFFSET(page, lru);
    VMCOREINFO_OFFSET(page, _mapcount);
    VMCOREINFO_OFFSET(page, private);
    VMCOREINFO_OFFSET(page, compound_info);
    VMCOREINFO_OFFSET(pglist_data, node_zones);
    VMCOREINFO_OFFSET(pglist_data, nr_zones);

    VMCOREINFO_OFFSET(pglist_data, node_mem_map);

    VMCOREINFO_OFFSET(pglist_data, node_start_pfn);
    VMCOREINFO_OFFSET(pglist_data, node_spanned_pages);
    VMCOREINFO_OFFSET(pglist_data, node_id);
    VMCOREINFO_OFFSET(zone, free_area);
    VMCOREINFO_OFFSET(zone, vm_stat);
    VMCOREINFO_OFFSET(zone, spanned_pages);
    VMCOREINFO_OFFSET(free_area, free_list);
    VMCOREINFO_OFFSET(list_head, next);
    VMCOREINFO_OFFSET(list_head, prev);
    VMCOREINFO_LENGTH(zone.free_area, NR_PAGE_ORDERS);
    log_buf_vmcoreinfo_setup();
    VMCOREINFO_LENGTH(free_area.free_list, MIGRATE_TYPES);
    VMCOREINFO_NUMBER(NR_FREE_PAGES);
    VMCOREINFO_NUMBER(PG_lru);
    VMCOREINFO_NUMBER(PG_private);
    VMCOREINFO_NUMBER(PG_swapcache);
    VMCOREINFO_NUMBER(PG_swapbacked);

    VMCOREINFO_NUMBER(PAGE_SLAB_MAPCOUNT_VALUE);

    VMCOREINFO_NUMBER(PG_hwpoison);

    VMCOREINFO_NUMBER(PG_head_mask);

    VMCOREINFO_NUMBER(PAGE_BUDDY_MAPCOUNT_VALUE);

    VMCOREINFO_NUMBER(PAGE_HUGETLB_MAPCOUNT_VALUE);

    VMCOREINFO_NUMBER(PAGE_OFFLINE_MAPCOUNT_VALUE);

    VMCOREINFO_NUMBER(PAGE_UNACCEPTED_MAPCOUNT_VALUE);

    VMCOREINFO_SYMBOL(kallsyms_names);
    VMCOREINFO_SYMBOL(kallsyms_num_syms);
    VMCOREINFO_SYMBOL(kallsyms_token_table);
    VMCOREINFO_SYMBOL(kallsyms_token_index);
    VMCOREINFO_SYMBOL(kallsyms_offsets);

    arch_crash_save_vmcoreinfo();
    update_vmcoreinfo_note();
    return 0;
    }
    subsys_initcall!(crash_save_vmcoreinfo_init);