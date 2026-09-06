//! Automatically rewritten from C to Rust
//! Source: kernel/events/uprobes.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// User-space Probes (UProbes)
//
// Copyright (C) IBM Corporation, 2008-2012
// Authors:
// Srikar Dronamraju
// Jim Keniston
// Copyright (C) 2011-2012 Red Hat, Inc., Peter Zijlstra
//

pub static mut uprobes_tree: rb_root = 0;
//
// allows us to skip the uprobe_mmap if there are no uprobe events active
// at this time.  Probably a fine grained per inode count is better?
//

pub static mut uprobes_treelock: usize = 0;	/* serialize rbtree access */
pub static mut uprobes_seqcount: seqcount_rwlock_t = 0;
pub const UPROBES_HASH_SZ: c_int = 13;
// serialize uprobe->pending_list
    static struct mutex uprobes_mmap_mutex[UPROBES_HASH_SZ];

pub static mut dup_mmap_sem: usize = 0;
// Covers return_instance's uprobe lifetime.
pub static mut uretprobes_srcu: usize = 0;
// Have a copy of original instruction
pub const UPROBE_COPY_INSN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe {
//     pub /: *mut *mut rb_node rb_node; / node in the rb tree,
    pub ref: refcount_t,
    pub register_rwsem: rw_semaphore,
    pub consumer_rwsem: rw_semaphore,
    pub pending_list: list_head,
    pub consumers: list_head,
//     pub /: *mut *mut *mut inode inode; / Also hold a ref to inode,
    union {
    pub rcu: rcu_head,
    pub work: work_struct,
}

    let mut offset = 0;
    let mut ref_ctr_offset = 0;
    let mut flags = 0;		/* "unsigned long" so bitops work */
//
// The generic code assumes that it has two members of unknown type
// owned by the arch-specific code:
//
// insn -	copy_insn() saves the original instruction here for
// arch_uprobe_analyze_insn().
//
// ixol -	potentially modified instruction to execute out of
// line, copied to xol_area by xol_get_insn_slot().
//
pub static mut arch: usize = 0;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_uprobe {
    pub list: list_head,
    pub uprobe: *mut uprobe,
    pub mm: *mut mm_struct,
}

pub static mut delayed_uprobe_lock: usize = 0;
pub static mut delayed_uprobe_list: usize = 0;
//
// Execute out of line area: anonymous executable mapping installed
// by the probed task to execute the copy of the original instruction
// mangled by set_swbp().
//
// On a breakpoint hit, thread contests for a slot.  It frees the
// slot after singlestep. Currently a fixed number of slots are
// allocated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xol_area {
//     pub /: *mut *mut wait_queue_head_t wq; / if all slots are busy,
//     pub /: *mut *mut *mut unsigned long bitmap; / 0 = free slot,
    pub page: *mut page,
//
// We keep the vma's vm_start rather than a pointer to the vma
// itself.  The probed process or a naughty kernel module could make
// the vma go away, and we must handle that reasonably gracefully.
//
//     pub /: *mut *mut unsigned long vaddr; / Page(s) of instruction slots,
}

#[no_mangle]
unsafe extern "C" fn uprobe_warn(t: *mut task_struct, msg: *const c_char) {
    pr_warn!("uprobe: %s:%d failed to %s\n", t.comm, t.pid, msg);
    }
//
// valid_vma: Verify if the specified vma is an executable vma
// Relax restrictions while unregistering: vm_flags might have
// changed after breakpoint was inserted.
// - is_register: indicates if we are in register context.
// - Return 1 if the specified virtual address is in an
// executable vma.
//
#[no_mangle]
unsafe extern "C" fn valid_vma(vma: *mut vm_area_struct, is_register: bool) -> bool {
pub static mut flags: vm_flags_t = 0;
    if (is_register) {
    flags |= VM_WRITE;
    }
    return vma.vm_file && (vma.vm_flags & flags) == VM_MAYEXEC;
    }
#[no_mangle]
unsafe extern "C" fn offset_to_vaddr(vma: *mut vm_area_struct, offset: loff_t) -> c_ulong {
    return vma.vm_start + offset -
    ((loff_t)vma_start_pgoff(vma) << PAGE_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn vaddr_to_offset(vma: *mut vm_area_struct, vaddr: c_ulong) -> loff_t {
    return ((loff_t)vma_start_pgoff(vma) << PAGE_SHIFT) +
    (vaddr - vma.vm_start);
    }
//
// is_swbp_insn - check if instruction is breakpoint instruction.
// @insn: instruction to be checked.
// Default implementation of is_swbp_insn
// Returns true if @insn is a breakpoint instruction.
//
#[no_mangle]
pub unsafe extern "C" fn is_swbp_insn(insn: *mut uprobe_opcode_t) -> bool __weak {
    let mut insn = = UPROBE_SWBP_INSN;
    }
//
// is_trap_insn - check if instruction is breakpoint instruction.
// @insn: instruction to be checked.
// Default implementation of is_trap_insn
// Returns true if @insn is a breakpoint instruction.
//
// This function is needed for the case where an architecture has multiple
// trap instructions (like powerpc).
//
#[no_mangle]
pub unsafe extern "C" fn is_trap_insn(insn: *mut uprobe_opcode_t) -> bool __weak {
    return is_swbp_insn(insn);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_copy_from_page(page: *mut page, vaddr: c_ulong, dst: *mut c_void, len: c_int) {
    let mut kaddr = kmap_local_page(page);
    memcpy(dst, kaddr + (vaddr & ~PAGE_MASK), len);
    kunmap_local(kaddr);
    }
#[no_mangle]
unsafe extern "C" fn copy_to_page(page: *mut page, vaddr: c_ulong, src: *const c_void, len: c_int) {
    let mut kaddr = kmap_local_page(page);
    memcpy(kaddr + (vaddr & ~PAGE_MASK), src, len);
    kunmap_local(kaddr);
    }
#[no_mangle]
pub unsafe extern "C" fn verify_opcode(page: *mut page, vaddr: c_ulong, insn: *mut uprobe_opcode_t, nbytes: c_int, data: *mut c_void) -> c_int {
    let mut old_opcode;
    let mut is_swbp = 0;
//
// Note: We only check if the old_opcode is UPROBE_SWBP_INSN here.
// We do not check if it is any other 'trap variant' which could
// be conditional trap instruction such as the one powerpc supports.
//
// The logic is that we do not care if the underlying instruction
// is a trap variant; uprobes always wins over any other (gdb)
// breakpoint.
//
    uprobe_copy_from_page(page, vaddr, &old_opcode, UPROBE_SWBP_INSN_SIZE);
    is_swbp = is_swbp_insn(&old_opcode);
    if (is_swbp_insn(insn)) {
    if (is_swbp)		/* register: already installed? */ {
    return 0;
    }
    } else {
    if (!is_swbp)		/* unregister: was it changed by us? */ {
    return 0;
    }
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn delayed_uprobe_check(uprobe: *mut uprobe, mm: *mut mm_struct) -> *mut c_void {
pub static mut du: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(du, &delayed_uprobe_list, list) {
    if (du.uprobe == uprobe && du.mm == mm)
    return du;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn delayed_uprobe_add(uprobe: *mut uprobe, mm: *mut mm_struct) -> c_int {
pub static mut du: *mut c_void = core::ptr::null_mut();
    if (delayed_uprobe_check(uprobe, mm)) {
    return 0;
    }
    du = kzalloc_obj(*du);
    if (!du) {
    return -ENOMEM;
    }
    du.uprobe = uprobe;
    du.mm = mm;
    list_add(&du.list, &delayed_uprobe_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn delayed_uprobe_delete(du: *mut delayed_uprobe) {
    if (WARN_ON!(!du)) {
    return;
    }
    list_del(&du.list);
    kfree(du);
    }
#[no_mangle]
unsafe extern "C" fn delayed_uprobe_remove(uprobe: *mut uprobe, mm: *mut mm_struct) {
    let mut pos = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
pub static mut du: *mut c_void = core::ptr::null_mut();
    if (!uprobe && !mm) {
    return;
    }
    list_for_each_safe(pos, q, &delayed_uprobe_list) {
    du = list_entry(pos, delayed_uprobe, list);
    if (uprobe && du.uprobe != uprobe) {
    continue;
    }
    if (mm && du.mm != mm) {
    continue;
    }
    delayed_uprobe_delete(du);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn valid_ref_ctr_vma(uprobe: *mut uprobe, vma: *mut vm_area_struct) -> bool {
pub static mut vaddr: c_ulong = 0;
    return uprobe.ref_ctr_offset &&
    vma.vm_file &&
    file_inode(vma.vm_file) == uprobe.inode &&
    (vma.vm_flags & (VM_WRITE|VM_SHARED)) == VM_WRITE &&
    vma.vm_start <= vaddr &&
    vma.vm_end > vaddr;
    }
#[no_mangle]
pub unsafe extern "C" fn find_ref_ctr_vma(uprobe: *mut uprobe, mm: *mut mm_struct) -> *mut c_void {
    VMA_ITERATOR(vmi, mm, 0);
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    for_each_vma(vmi, tmp) {
    if (valid_ref_ctr_vma(uprobe, tmp))
    return tmp;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __update_ref_ctr(mm: *mut mm_struct, vaddr: c_ulong, d: c_short) -> c_int {
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (!vaddr || !d) {
    return -EINVAL;
    }
    ret = get_user_pages_remote(mm, vaddr, 1,
    FOLL_WRITE, &page, core::ptr::null_mut());
    if (unlikely(ret <= 0)) {
//
// We are asking for 1 page. If get_user_pages_remote() fails,
// it may return 0, in that case we have to return error.
//
pub static mut ret: return = 0;
    }
    kaddr = kmap_local_page(page);
    ptr = kaddr + (vaddr & ~PAGE_MASK);
    if (unlikely(*ptr + d < 0)) {
    pr_warn!("ref_ctr going negative. vaddr: 0x%lx, "
    "curr val: %d, delta: %d\n", vaddr, *ptr, d);
    ret = -EINVAL;
// goto;
    }
// ptr += d;
    ret = 0;
// label;
    kunmap_local(kaddr);
    put_page(page);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn update_ref_ctr_warn(uprobe: *mut uprobe, mm: *mut mm_struct, d: c_short) {
    pr_warn!("ref_ctr %s failed for inode: 0x%llx offset: "
    "0x%llx ref_ctr_offset: 0x%llx of mm: 0x%p\n",
    d > 0 ? "increment" : "decrement", uprobe.inode.i_ino,
    (unsigned long long) uprobe.offset,
    (unsigned long long) uprobe.ref_ctr_offset, mm);
    }
#[no_mangle]
pub unsafe extern "C" fn update_ref_ctr(uprobe: *mut uprobe, mm: *mut mm_struct, d: c_short) -> c_int {
pub static mut rc_vma: *mut c_void = core::ptr::null_mut();
    let mut rc_vaddr = 0;
pub static mut ret: c_int = 0;
    rc_vma = find_ref_ctr_vma(uprobe, mm);
    if (rc_vma) {
    rc_vaddr = offset_to_vaddr(rc_vma, uprobe.ref_ctr_offset);
    ret = __update_ref_ctr(mm, rc_vaddr, d);
    if (ret) {
    update_ref_ctr_warn(uprobe, mm, d);
    }
    if (d > 0) {
    return ret;
    }
    }
    mutex_lock(&delayed_uprobe_lock);
    if (d > 0) {
    ret = delayed_uprobe_add(uprobe, mm);
    }
    else {
    delayed_uprobe_remove(uprobe, mm);
    }
    mutex_unlock(&delayed_uprobe_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn orig_page_is_identical(vma: *mut vm_area_struct, vaddr: c_ulong, page: *mut page, pmd_mappable: *mut bool) -> bool {
pub static mut index: pgoff_t = 0;
    let mut orig_folio = filemap_get_folio(vma.vm_file.f_mapping,
    index);
pub static mut orig_page: *mut c_void = core::ptr::null_mut();
    let mut identical = 0;
    if (IS_ERR(orig_folio)) {
    return false;
    }
    orig_page = folio_file_page(orig_folio, index);
// pmd_mappable = folio_test_pmd_mappable(orig_folio);
    identical = folio_test_uptodate(orig_folio) &&
    pages_identical(page, orig_page);
    folio_put(orig_folio);
    return identical;
    }
#[no_mangle]
pub unsafe extern "C" fn __uprobe_write(vma: *mut vm_area_struct, fw: *mut folio_walk, folio: *mut folio, insn_vaddr: c_ulong, insn: *mut uprobe_opcode_t, nbytes: c_int, is_register: bool) -> c_int {
pub static mut vaddr: c_ulong = 0;
    let mut pmd_mappable = 0;
// For now, we'll only handle PTE-mapped folios.
    if (fw.level != FW_LEVEL_PTE) {
    return -EFAULT;
    }
//
// See can_follow_write_pte(): we'd actually prefer a writable PTE here,
// but the VMA might not be writable.
//
    if (!pte_write(fw.pte)) {
    if (!PageAnonExclusive(fw.page)) {
    return -EFAULT;
    }
    if (unlikely(userfaultfd_pte_wp(vma, fw.pte))) {
    return -EFAULT;
    }
// SOFTDIRTY is handled via pte_mkdirty() below.
    }
//
// We'll temporarily unmap the page and flush the TLB, such that we can
// modify the page atomically.
//
    flush_cache_page(vma, vaddr, pte_pfn(fw.pte));
    fw.pte = ptep_clear_flush(vma, vaddr, fw.ptep);
    copy_to_page(fw.page, insn_vaddr, insn, nbytes);
//
// When unregistering, we may only zap a PTE if uffd is disabled and
// there are no unexpected folio references ...
//
    if (is_register || userfaultfd_missing(vma) ||
    (folio_ref_count(folio) != folio_expected_ref_count(folio) + 1)) {
// goto;
    }
//
// ... and the mapped page is identical to the original page that
// would get faulted in on next access.
//
    if (!orig_page_is_identical(vma, vaddr, fw.page, &pmd_mappable)) {
// goto;
    }
    dec_mm_counter(vma.vm_mm, MM_ANONPAGES);
    folio_remove_rmap_pte(folio, fw.page, vma);
    if (!folio_mapped(folio) && folio_test_swapcache(folio) &&
    folio_trylock(folio)) {
    folio_free_swap(folio);
    folio_unlock(folio);
    }
    folio_put(folio);
    return pmd_mappable;
// label;
//
// Make sure that our copy_to_page() changes become visible before the
// set_pte_at() write.
//
    smp_wmb();
// We modified the page. Make sure to mark the PTE dirty.
    set_pte_at(vma.vm_mm, vaddr, fw.ptep, pte_mkdirty(fw.pte));
    return 0;
    }
//
// NOTE:
// Expect the breakpoint instruction to be the smallest size instruction for
// the architecture. If an arch has variable length instruction and the
// breakpoint instruction is not of the smallest length instruction
// supported by that architecture then we need to modify is_trap_at_addr and
// uprobe_write_opcode accordingly. This would never be a problem for archs
// that have fixed length instructions.
//
// uprobe_write_opcode - write the opcode at a given virtual address.
// @auprobe: arch specific probepoint information.
// @vma: the probed virtual memory area.
// @opcode_vaddr: the virtual address to store the opcode.
// @opcode: opcode to be written at @opcode_vaddr.
//
// Called with mm->mmap_lock held for write.
// Return 0 (success) or a negative errno.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_write_opcode(auprobe: *mut arch_uprobe, vma: *mut vm_area_struct, opcode_vaddr: c_ulong, opcode: uprobe_opcode_t, is_register: bool) -> c_int {
    return uprobe_write(auprobe, vma, opcode_vaddr, &opcode, UPROBE_SWBP_INSN_SIZE,
    verify_opcode, is_register, true /* do_update_ref_ctr */, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_write(auprobe: *mut arch_uprobe, vma: *mut vm_area_struct, insn_vaddr: c_ulong, insn: *mut uprobe_opcode_t, nbytes: c_int, verify: uprobe_write_verify_t, is_register: bool, do_update_ref_ctr: bool, data: *mut c_void) -> c_int {
pub static mut vaddr: c_ulong = 0;
    let mut mm = vma.vm_mm;
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    int ret, ref_ctr_updated = 0;
pub static mut gup_flags: c_uint = 0;
pub static mut range: usize = 0;
pub static mut fw: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    uprobe = container_of!(auprobe, uprobe, arch);
    if (WARN_ON_ONCE!(!vma_is_cow_mapping(vma))) {
    return -EINVAL;
    }
//
// When registering, we have to break COW to get an exclusive anonymous
// page that we can safely modify. Use FOLL_WRITE to trigger a write
// fault if required. When unregistering, we might be lucky and the
// anon page is already gone. So defer write faults until really
// required. Use FOLL_SPLIT_PMD, because __uprobe_write()
// cannot deal with PMDs yet.
//
    if (is_register) {
    gup_flags |= FOLL_WRITE | FOLL_SPLIT_PMD;
    }
// label;
    ret = get_user_pages_remote(mm, vaddr, 1, gup_flags, &page, core::ptr::null_mut());
    if (ret <= 0) {
// goto;
    }
    folio = page_folio(page);
    ret = verify(page, insn_vaddr, insn, nbytes, data);
    if (ret <= 0) {
    folio_put(folio);
// goto;
    }
// We are going to replace instruction, update ref_ctr.
    if (do_update_ref_ctr && !ref_ctr_updated && uprobe.ref_ctr_offset) {
    ret = update_ref_ctr(uprobe, mm, is_register ? 1 : -1);
    if (ret) {
    folio_put(folio);
// goto;
    }
    ref_ctr_updated = 1;
    }
    ret = 0;
    if (unlikely(!folio_test_anon(folio) || folio_is_zone_device(folio))) {
    VM_WARN_ON_ONCE(is_register);
    folio_put(folio);
// goto;
    }
    if (!is_register) {
//
// In the common case, we'll be able to zap the page when
// unregistering. So trigger MMU notifiers now, as we won't
// be able to do it under PTL.
//
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, mm,
    vaddr, vaddr + PAGE_SIZE);
    mmu_notifier_invalidate_range_start(&range);
    }
    ret = -EAGAIN;
// Walk the page tables again, to perform the actual update.
    if (folio_walk_start(&fw, vma, vaddr, 0)) {
    if (fw.page == page) {
    ret = __uprobe_write(vma, &fw, folio, insn_vaddr, insn, nbytes, is_register);
    }
    folio_walk_end(&fw, vma);
    }
    if (!is_register) {
    mmu_notifier_invalidate_range_end(&range);
    }
    folio_put(folio);
    match (ret) {
    -EFAULT => {
    gup_flags |= FOLL_WRITE | FOLL_SPLIT_PMD;
    fallthrough;
    }
    -EAGAIN => {
// goto;
    }
    _ => {
    // break;
    }
    }
// label;
// Revert back reference counter if instruction update failed.
    if (do_update_ref_ctr && ret < 0 && ref_ctr_updated) {
    update_ref_ctr(uprobe, mm, is_register ? -1 : 1);
    }
// try collapse pmd for compound page
    if (ret > 0) {
    collapse_pte_mapped_thp(mm, vaddr, false);
    }
    return ret < 0 ? ret : 0;
    }
//
// set_swbp - store breakpoint at a given address.
// @auprobe: arch specific probepoint information.
// @vma: the probed virtual memory area.
// @vaddr: the virtual address to insert the opcode.
//
// For mm @mm, store the breakpoint instruction at @vaddr.
// Return 0 (success) or a negative errno.
//
    int __weak set_swbp(arch_uprobe *auprobe, vm_area_struct *vma,
    unsigned long vaddr)
    {
    return uprobe_write_opcode(auprobe, vma, vaddr, UPROBE_SWBP_INSN, true);
    }
//
// set_orig_insn - Restore the original instruction.
// @vma: the probed virtual memory area.
// @auprobe: arch specific probepoint information.
// @vaddr: the virtual address to insert the opcode.
//
// For mm @mm, restore the original opcode (opcode) at @vaddr.
// Return 0 (success) or a negative errno.
//
    int __weak set_orig_insn(arch_uprobe *auprobe, vm_area_struct *vma, unsigned long vaddr)
    {
    return uprobe_write_opcode(auprobe, vma, vaddr,
// &auprobe->insn, false);
    }
// uprobe should have guaranteed positive refcount
#[no_mangle]
pub unsafe extern "C" fn get_uprobe(uprobe: *mut uprobe) -> *mut c_void {
    refcount_inc(&uprobe.ref);
    return uprobe;
    }
//
// uprobe should have guaranteed lifetime, which can be either of:
// - caller already has refcount taken (and wants an extra one);
// - uprobe is RCU protected and won't be freed until after grace period;
// - we are holding uprobes_treelock (for read or write, doesn't matter).
//
#[no_mangle]
pub unsafe extern "C" fn try_get_uprobe(uprobe: *mut uprobe) -> *mut c_void {
    if (refcount_inc_not_zero(&uprobe.ref)) {
    return uprobe;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_is_active(uprobe: *mut uprobe) -> bool {
    return !RB_EMPTY_NODE(&uprobe.rb_node);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_free_rcu_tasks_trace(rcu: *mut rcu_head) {
    let mut uprobe = container_of!(rcu, uprobe, rcu);
    kfree(uprobe);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_free_srcu(rcu: *mut rcu_head) {
    let mut uprobe = container_of!(rcu, uprobe, rcu);
    call_rcu_tasks_trace(&uprobe.rcu, uprobe_free_rcu_tasks_trace);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_free_deferred(work: *mut work_struct) {
    let mut uprobe = container_of!(work, uprobe, work);
    write_lock(&uprobes_treelock);
    if (uprobe_is_active(uprobe)) {
    write_seqcount_begin(&uprobes_seqcount);
    rb_erase(&uprobe.rb_node, &uprobes_tree);
    write_seqcount_end(&uprobes_seqcount);
    }
    write_unlock(&uprobes_treelock);
//
// If application munmap(exec_vma) before uprobe_unregister()
// gets called, we don't get a chance to remove uprobe from
// delayed_uprobe_list from remove_breakpoint(). Do it here.
//
    mutex_lock(&delayed_uprobe_lock);
    delayed_uprobe_remove(uprobe, core::ptr::null_mut());
    mutex_unlock(&delayed_uprobe_lock);
// start srcu -> rcu_tasks_trace -> kfree chain
    call_srcu(&uretprobes_srcu, &uprobe.rcu, uprobe_free_srcu);
    }
#[no_mangle]
unsafe extern "C" fn put_uprobe(uprobe: *mut uprobe) {
    if (!refcount_dec_and_test(&uprobe.ref)) {
    return;
    }
    INIT_WORK(&uprobe.work, uprobe_free_deferred);
    schedule_work(&uprobe.work);
    }
// Initialize hprobe as SRCU-protected "leased" uprobe
#[no_mangle]
pub unsafe extern "C" fn hprobe_init_leased(hprobe: *mut hprobe, uprobe: *mut uprobe, srcu_scp: *mut srcu_ctr) {
    WARN_ON!(!uprobe);
    hprobe.state = HPROBE_LEASED;
    hprobe.uprobe = uprobe;
    hprobe.srcu_scp = srcu_scp;
    }
// Initialize hprobe as refcounted ("stable") uprobe (uprobe can be NULL).
#[no_mangle]
unsafe extern "C" fn hprobe_init_stable(hprobe: *mut hprobe, uprobe: *mut uprobe) {
    hprobe.state = uprobe ? HPROBE_STABLE : HPROBE_GONE;
    hprobe.uprobe = uprobe;
    hprobe.srcu_scp = core::ptr::null_mut();
    }
//
// hprobe_consume() fetches hprobe's underlying uprobe and detects whether
// uprobe is SRCU protected or is refcounted. hprobe_consume() can be
// used only once for a given hprobe.
//
// Caller has to call hprobe_finalize() and pass previous hprobe_state, so
// that hprobe_finalize() can perform SRCU unlock or put uprobe, whichever
// is appropriate.
//
#[no_mangle]
pub unsafe extern "C" fn hprobe_consume(hprobe: *mut hprobe, hstate: *mut hprobe_state) -> *mut c_void {
// hstate = xchg(&hprobe->state, HPROBE_CONSUMED);
    match (*hstate) {
    HPROBE_LEASED => {
    }
    HPROBE_STABLE => {
    return hprobe.uprobe;
    }
    HPROBE_GONE => {
    }
    HPROBE_CONSUMED => {
    return core::ptr::null_mut();
    }
    _ => {
    WARN(1, "hprobe invalid state %d", *hstate);
    return core::ptr::null_mut();
    }
    }
    }
//
// Reset hprobe state and, if hprobe was LEASED, release SRCU lock.
// hprobe_finalize() can only be used from current context after
// hprobe_consume() call (which determines uprobe and hstate value).
//
#[no_mangle]
unsafe extern "C" fn hprobe_finalize(hprobe: *mut hprobe, hstate: hprobe_state) {
    match (hstate) {
    HPROBE_LEASED => {
    srcu_up_read_fast(&uretprobes_srcu, hprobe.srcu_scp);
    // break;
    }
    HPROBE_STABLE => {
    put_uprobe(hprobe.uprobe);
    // break;
    }
    HPROBE_GONE => {
    }
    HPROBE_CONSUMED => {
    // break;
    }
    _ => {
    WARN(1, "hprobe invalid state %d", hstate);
    // break;
    }
    }
    }
//
// Attempt to switch (atomically) uprobe from being SRCU protected (LEASED)
// to refcounted (STABLE) state. Competes with hprobe_consume(); only one of
// them can win the race to perform SRCU unlocking. Whoever wins must perform
// SRCU unlock.
//
// Returns underlying valid uprobe or NULL, if there was no underlying uprobe
// to begin with or we failed to bump its refcount and it's going away.
//
// Returned non-NULL uprobe can be still safely used within an ongoing SRCU
// locked region. If `get` is true, it's guaranteed that non-NULL uprobe has
// an extra refcount for caller to assume and use. Otherwise, it's not
// guaranteed that returned uprobe has a positive refcount, so caller has to
// attempt try_get_uprobe(), if it needs to preserve uprobe beyond current
// SRCU lock region. See dup_utask().
//
#[no_mangle]
pub unsafe extern "C" fn hprobe_expire(hprobe: *mut hprobe, get: bool) -> *mut c_void {
    enum hprobe_state hstate;
//
// Caller should guarantee that return_instance is not going to be
// freed from under us. This can be achieved either through holding
// rcu_read_lock() or by owning return_instance in the first place.
//
// Underlying uprobe is itself protected from reuse by SRCU, so ensure
// SRCU lock is held properly.
//
    lockdep_assert(srcu_read_lock_held(&uretprobes_srcu));
    hstate = READ_ONCE(hprobe.state);
    match (hstate) {
    HPROBE_STABLE => {
// uprobe has positive refcount, bump refcount, if necessary
    return get ? get_uprobe(hprobe.uprobe) : hprobe.uprobe;
    }
    HPROBE_GONE => {
//
// SRCU was unlocked earlier and we didn't manage to take
// uprobe refcnt, so it's effectively NULL
//
    return core::ptr::null_mut();
    }
    HPROBE_CONSUMED => {
//
// uprobe was consumed, so it's effectively NULL as far as
// uretprobe processing logic is concerned
//
    return core::ptr::null_mut();
    }
    HPROBE_LEASED => {
    let mut uprobe = try_get_uprobe(hprobe.uprobe);
//
// Try to switch hprobe state, guarding against
// hprobe_consume() or another hprobe_expire() racing with us.
// Note, if we failed to get uprobe refcount, we use special
// HPROBE_GONE state to signal that hprobe->uprobe shouldn't
// be used as it will be freed after SRCU is unlocked.
//
    if (try_cmpxchg(&hprobe.state, &hstate, uprobe ? HPROBE_STABLE : HPROBE_GONE)) {
// We won the race, we are the ones to unlock SRCU
    srcu_up_read_fast(&uretprobes_srcu, hprobe.srcu_scp);
    return get && uprobe ? get_uprobe(uprobe) : uprobe;
    }
//
// We lost the race, undo refcount bump (if it ever happened),
// unless caller would like an extra refcount anyways.
//
    if (uprobe && !get) {
    put_uprobe(uprobe);
    }
//
// Even if hprobe_consume() or another hprobe_expire() wins
// the state update race and unlocks SRCU from under us, we
// still have a guarantee that underyling uprobe won't be
// freed due to ongoing caller's SRCU lock region, so we can
// return it regardless. Also, if `get` was true, we also have
// an extra ref for the caller to own. This is used in dup_utask().
//
    return uprobe;
    }
    }
// label;
    WARN(1, "unknown hprobe state %d", hstate);
    return core::ptr::null_mut();
    }
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn uprobe_cmp(l_inode: *mut inode, l_offset: loff_t, r: *mut uprobe) -> c_int {
    if (l_inode < r.inode) {
    return -1;
    }
    if (l_inode > r.inode) {
    return 1;
    }
    if (l_offset < r.offset) {
    return -1;
    }
    if (l_offset > r.offset) {
    return 1;
    }
    return 0;
    }

    rb_entry((node), uprobe, rb_node)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __uprobe_key {
    pub inode: *mut inode,
    pub offset: loff_t,
}

#[no_mangle]
pub unsafe extern "C" fn __uprobe_cmp_key(key: *const c_void, b: *const rb_node) -> c_int {
    let mut a = key;
    return uprobe_cmp(a.inode, a.offset, __node_2_uprobe(b));
    }
#[no_mangle]
pub unsafe extern "C" fn __uprobe_cmp(a: *mut rb_node, b: *const rb_node) -> c_int {
    let mut u = __node_2_uprobe(a);
    return uprobe_cmp(u.inode, u.offset, __node_2_uprobe(b));
    }
//
// Assumes being inside RCU protected region.
// No refcount is taken on returned uprobe.
//
#[no_mangle]
pub unsafe extern "C" fn find_uprobe_rcu(inode: *mut inode, offset: loff_t) -> *mut c_void {
pub static mut __uprobe_key: usize = 0;
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut seq = 0;
    lockdep_assert(rcu_read_lock_trace_held());
    do {
    seq = read_seqcount_begin(&uprobes_seqcount);
    node = rb_find_rcu(&key, &uprobes_tree, __uprobe_cmp_key);
//
// Lockless RB-tree lookups can result only in false negatives.
// If the element is found, it is correct and can be returned
// under RCU protection. If we find nothing, we need to
// validate that seqcount didn't change. If it did, we have to
// try again as we might have missed the element (false
// negative). If seqcount is unchanged, search truly failed.
//
    if (node) {
    return __node_2_uprobe(node);
    }
    } while (read_seqcount_retry(&uprobes_seqcount, seq));
    return core::ptr::null_mut();
    }
//
// Attempt to insert a new uprobe into uprobes_tree.
//
// If uprobe already exists (for given inode+offset), we just increment
// refcount of previously existing uprobe.
//
// If not, a provided new instance of uprobe is inserted into the tree (with
// assumed initial refcount == 1).
//
// In any case, we return a uprobe instance that ends up being in uprobes_tree.
// Caller has to clean up new uprobe instance, if it ended up not being
// inserted into the tree.
//
// We assume that uprobes_treelock is held for writing.
//
#[no_mangle]
pub unsafe extern "C" fn __insert_uprobe(uprobe: *mut uprobe) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
// label;
    node = rb_find_add_rcu(&uprobe.rb_node, &uprobes_tree, __uprobe_cmp);
    if (node) {
    let mut u = __node_2_uprobe(node);
    if (!try_get_uprobe(u)) {
    rb_erase(node, &uprobes_tree);
    RB_CLEAR_NODE(&u.rb_node);
// goto;
    }
    return u;
    }
    return uprobe;
    }
//
// Acquire uprobes_treelock and insert uprobe into uprobes_tree
// (or reuse existing one, see __insert_uprobe() comments above).
//
#[no_mangle]
pub unsafe extern "C" fn insert_uprobe(uprobe: *mut uprobe) -> *mut c_void {
pub static mut u: *mut c_void = core::ptr::null_mut();
    write_lock(&uprobes_treelock);
    write_seqcount_begin(&uprobes_seqcount);
    u = __insert_uprobe(uprobe);
    write_seqcount_end(&uprobes_seqcount);
    write_unlock(&uprobes_treelock);
    return u;
    }
#[no_mangle]
pub unsafe extern "C" fn ref_ctr_mismatch_warn(cur_uprobe: *mut uprobe, uprobe: *mut uprobe) {
    pr_warn!("ref_ctr_offset mismatch. inode: 0x%llx offset: 0x%llx "
    "ref_ctr_offset(old): 0x%llx ref_ctr_offset(new): 0x%llx\n",
    uprobe.inode.i_ino, (unsigned long long) uprobe.offset,
    (unsigned long long) cur_uprobe.ref_ctr_offset,
    (unsigned long long) uprobe.ref_ctr_offset);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_uprobe(inode: *mut inode, offset: loff_t, ref_ctr_offset: loff_t) -> *mut c_void {
    let mut uprobe = core::ptr::null_mut();
    let mut cur_uprobe = core::ptr::null_mut();
    uprobe = kzalloc_obj(uprobe);
    if (!uprobe) {
    return ERR_PTR(-ENOMEM);
    }
    uprobe.inode = inode;
    uprobe.offset = offset;
    uprobe.ref_ctr_offset = ref_ctr_offset;
    INIT_LIST_HEAD(&uprobe.consumers);
    init_rwsem(&uprobe.register_rwsem);
    init_rwsem(&uprobe.consumer_rwsem);
    RB_CLEAR_NODE(&uprobe.rb_node);
    refcount_set(&uprobe.ref, 1);
// add to uprobes_tree, sorted on inode:offset
    cur_uprobe = insert_uprobe(uprobe);
// a uprobe exists for this inode:offset combination
    if (cur_uprobe != uprobe) {
    if (cur_uprobe.ref_ctr_offset != uprobe.ref_ctr_offset) {
    ref_ctr_mismatch_warn(cur_uprobe, uprobe);
    put_uprobe(cur_uprobe);
    kfree(uprobe);
    return ERR_PTR(-EINVAL);
    }
    kfree(uprobe);
    uprobe = cur_uprobe;
    }
    return uprobe;
    }
#[no_mangle]
unsafe extern "C" fn consumer_add(uprobe: *mut uprobe, uc: *mut uprobe_consumer) {
    static atomic64_t id;
    down_write(&uprobe.consumer_rwsem);
    list_add_rcu(&uc.cons_node, &uprobe.consumers);
    uc.id = (__u64) atomic64_inc_return(&id);
    up_write(&uprobe.consumer_rwsem);
    }
//
// For uprobe @uprobe, delete the consumer @uc.
// Should never be called with consumer that's not part of @uprobe->consumers.
//
#[no_mangle]
unsafe extern "C" fn consumer_del(uprobe: *mut uprobe, uc: *mut uprobe_consumer) {
    down_write(&uprobe.consumer_rwsem);
    list_del_rcu(&uc.cons_node);
    up_write(&uprobe.consumer_rwsem);
    }
#[no_mangle]
pub unsafe extern "C" fn __copy_insn(mapping: *mut address_space, filp: *mut file, insn: *mut c_void, nbytes: c_int, offset: loff_t) -> c_int {
pub static mut page: *mut c_void = core::ptr::null_mut();
//
// Ensure that the page that has the original instruction is populated
// and in page-cache. If ->read_folio == NULL it must be shmem_mapping(),
// see uprobe_register().
//
    if (mapping.a_ops.read_folio) {
    page = read_mapping_page(mapping, offset >> PAGE_SHIFT, filp);
    }
    else {
    page = shmem_read_mapping_page(mapping, offset >> PAGE_SHIFT);
    }
    if (IS_ERR(page)) {
    return PTR_ERR(page);
    }
    uprobe_copy_from_page(page, offset, insn, nbytes);
    put_page(page);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn copy_insn(uprobe: *mut uprobe, filp: *mut file) -> c_int {
    let mut mapping = uprobe.inode.i_mapping;
pub static mut offs: loff_t = 0;
    let mut insn = &uprobe.arch.insn;
pub static mut size: c_int = 0;
    int len, err = -EIO;
// Copy only available bytes, -EIO if nothing was read
    do {
    if (offs >= i_size_read(uprobe.inode)) {
    break;
    }
    len = min_t(int, size, PAGE_SIZE - (offs & ~PAGE_MASK));
    err = __copy_insn(mapping, filp, insn, len, offs);
    if (err) {
    break;
    }
    insn += len;
    offs += len;
    size -= len;
    } while (size);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_uprobe(uprobe: *mut uprobe, file: *mut file, mm: *mut mm_struct, vaddr: c_ulong) -> c_int {
pub static mut ret: c_int = 0;
    if (test_bit(UPROBE_COPY_INSN, &uprobe.flags)) {
    return ret;
    }
// TODO: move this into _register, until then we abuse this sem.
    down_write(&uprobe.consumer_rwsem);
    if (test_bit(UPROBE_COPY_INSN, &uprobe.flags)) {
// goto;
    }
    ret = copy_insn(uprobe, file);
    if (ret) {
// goto;
    }
    ret = -ENOTSUPP;
    if (is_trap_insn(&uprobe.arch.insn)) {
// goto;
    }
    ret = arch_uprobe_analyze_insn(&uprobe.arch, mm, vaddr);
    if (ret) {
// goto;
    }
    smp_wmb(); /* pairs with the smp_rmb() in handle_swbp() */
    set_bit(UPROBE_COPY_INSN, &uprobe.flags);
// label;
    up_write(&uprobe.consumer_rwsem);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn consumer_filter(uc: *mut uprobe_consumer, mm: *mut mm_struct) -> bool {
    return !uc.filter || uc.filter(uc, mm);
    }
#[no_mangle]
unsafe extern "C" fn filter_chain(uprobe: *mut uprobe, mm: *mut mm_struct) -> bool {
pub static mut uc: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = false;
    down_read(&uprobe.consumer_rwsem);
    list_for_each_entry(uc, &uprobe.consumers, cons_node) {
    ret = consumer_filter(uc, mm);
    if (ret) {
    break;
    }
    }
    up_read(&uprobe.consumer_rwsem);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn install_breakpoint(uprobe: *mut uprobe, vma: *mut vm_area_struct, vaddr: c_ulong) -> c_int {
    let mut mm = vma.vm_mm;
    let mut first_uprobe = 0;
    let mut ret = 0;
    ret = prepare_uprobe(uprobe, vma.vm_file, mm, vaddr);
    if (ret) {
    return ret;
    }
//
// set MMF_HAS_UPROBES in advance for uprobe_pre_sstep_notifier(),
// the task can hit this breakpoint right after __replace_page().
//
    first_uprobe = !mm_flags_test(MMF_HAS_UPROBES, mm);
    if (first_uprobe) {
    mm_flags_set(MMF_HAS_UPROBES, mm);
    }
    ret = set_swbp(&uprobe.arch, vma, vaddr);
    if (!ret) {
    mm_flags_clear(MMF_RECALC_UPROBES, mm);
    }

    else if (first_uprobe) {
    mm_flags_clear(MMF_HAS_UPROBES, mm);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_breakpoint(uprobe: *mut uprobe, vma: *mut vm_area_struct, vaddr: c_ulong) -> c_int {
    let mut mm = vma.vm_mm;
    mm_flags_set(MMF_RECALC_UPROBES, mm);
    return set_orig_insn(&uprobe.arch, vma, vaddr);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_info {
    pub next: *mut map_info,
    pub mm: *mut mm_struct,
    pub vaddr: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn free_map_info(info: *mut map_info) -> *mut c_void {
    let mut next = info.next;
    kfree(info);
    return next;
    }
#[no_mangle]
pub unsafe extern "C" fn build_map_info(mapping: *mut address_space, offset: loff_t, is_register: bool) -> *mut c_void {
pub static mut pgoff: c_ulong = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut curr = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut more: c_int = 0;
// label;
    i_mmap_lock_read(mapping);
    mapping_rmap_tree_foreach(vma, mapping, pgoff, pgoff) {
    if (!valid_vma(vma, is_register)) {
    continue;
    }
    if (!prev && !more) {
//
// Needs GFP_NOWAIT to avoid i_mmap_rwsem recursion through
// reclaim. This is optimistic, no harm done if it fails.
//
    prev = kmalloc_obj(map_info,
    GFP_NOWAIT | __GFP_NOMEMALLOC);
    if (prev) {
    prev.next = core::ptr::null_mut();
    }
    }
    if (!prev) {
    more += 1;
    continue;
    }
    if (!mmget_not_zero(vma.vm_mm)) {
    continue;
    }
    info = prev;
    prev = prev.next;
    info.next = curr;
    curr = info;
    info.mm = vma.vm_mm;
    info.vaddr = offset_to_vaddr(vma, offset);
    }
    i_mmap_unlock_read(mapping);
    if (!more) {
// goto;
    }
    prev = curr;
    while (curr) {
    mmput(curr.mm);
    curr = curr.next;
    }
    do {
    info = kmalloc_obj(map_info);
    if (!info) {
    curr = ERR_PTR(-ENOMEM);
// goto;
    }
    info.next = prev;
    prev = info;
    } while (--more);
// goto;
// label;
    while (prev) {
    prev = free_map_info(prev);
    }
    return curr;
    }
#[no_mangle]
pub unsafe extern "C" fn register_for_each_vma(uprobe: *mut uprobe, new: *mut uprobe_consumer) -> c_int {
pub static mut is_register: bool = false;
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    percpu_down_write(&dup_mmap_sem);
    info = build_map_info(uprobe.inode.i_mapping,
    uprobe.offset, is_register);
    if (IS_ERR(info)) {
    err = PTR_ERR(info);
// goto;
    }
    while (info) {
    let mut mm = info.mm;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    if (err && is_register) {
// goto;
    }
//
// We take mmap_lock for writing to avoid the race with
// find_active_uprobe_rcu() which takes mmap_lock for reading.
// Thus this install_breakpoint() can not make
// is_trap_at_addr() true right after find_uprobe_rcu()
// returns NULL in find_active_uprobe_rcu().
//
    mmap_write_lock(mm);
    if (check_stable_address_space(mm)) {
// goto;
    }
    vma = find_vma(mm, info.vaddr);
    if (!vma || !valid_vma(vma, is_register) ||
    file_inode(vma.vm_file) != uprobe.inode) {
// goto;
    }
    if (vma.vm_start > info.vaddr ||
    vaddr_to_offset(vma, info.vaddr) != uprobe.offset) {
// goto;
    }
    if (is_register) {
// consult only the "caller", new consumer.
    if (consumer_filter(new, mm)) {
    err = install_breakpoint(uprobe, vma, info.vaddr);
    }
    } else if (mm_flags_test(MMF_HAS_UPROBES, mm)) {
    if (!filter_chain(uprobe, mm)) {
    err |= remove_breakpoint(uprobe, vma, info.vaddr);
    }
    }
// label;
    mmap_write_unlock(mm);
// label;
    mmput(mm);
    info = free_map_info(info);
    }
// label;
    percpu_up_write(&dup_mmap_sem);
    return err;
    }
//
// uprobe_unregister_nosync - unregister an already registered probe.
// @uprobe: uprobe to remove
// @uc: identify which probe if multiple probes are colocated.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_unregister_nosync(uprobe: *mut uprobe, uc: *mut uprobe_consumer) {
    let mut err = 0;
    down_write(&uprobe.register_rwsem);
    consumer_del(uprobe, uc);
    err = register_for_each_vma(uprobe, core::ptr::null_mut());
    up_write(&uprobe.register_rwsem);
// TODO : cant unregister? schedule a worker thread
    if (unlikely(err)) {
    uprobe_warn(current, "unregister, leaking uprobe");
    return;
    }
    put_uprobe(uprobe);
    }
    EXPORT_SYMBOL_GPL(uprobe_unregister_nosync);
#[no_mangle]
pub unsafe extern "C" fn uprobe_unregister_sync() {
//
// Now that handler_chain() and handle_uretprobe_chain() iterate over
// uprobe->consumers list under RCU protection without holding
// uprobe->register_rwsem, we need to wait for RCU grace period to
// make sure that we can't call into just unregistered
// uprobe_consumer's callbacks anymore. If we don't do that, fast and
// unlucky enough caller can free consumer's memory and cause
// handler_chain() or handle_uretprobe_chain() to do an use-after-free.
//
    synchronize_rcu_tasks_trace();
    synchronize_srcu(&uretprobes_srcu);
    }
    EXPORT_SYMBOL_GPL(uprobe_unregister_sync);
//
// uprobe_register - register a probe
// @inode: the file in which the probe has to be placed.
// @offset: offset from the start of the file.
// @ref_ctr_offset: offset of SDT marker / reference counter
// @uc: information on howto handle the probe..
//
// Apart from the access refcount, uprobe_register() takes a creation
// refcount (thro alloc_uprobe) if and only if this @uprobe is getting
// inserted into the rbtree (i.e first consumer for a @inode:@offset
// tuple).  Creation refcount stops uprobe_unregister from freeing the
// @uprobe even before the register operation is complete. Creation
// refcount is released when the last @uc for the @uprobe
// unregisters. Caller of uprobe_register() is required to keep @inode
// (and the containing mount) referenced.
//
// Return: pointer to the new uprobe on success or an ERR_PTR on failure.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_register(inode: *mut inode, offset: loff_t, ref_ctr_offset: loff_t, uc: *mut uprobe_consumer) -> *mut c_void {
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Uprobe must have at least one set consumer
    if (!uc.handler && !uc.ret_handler) {
    return ERR_PTR(-EINVAL);
    }
// copy_insn() uses read_mapping_page() or shmem_read_mapping_page()
    if (!inode.i_mapping.a_ops.read_folio &&
    !shmem_mapping(inode.i_mapping)) {
    return ERR_PTR(-EIO);
    }
// Racy, just to catch the obvious mistakes
    if (offset > i_size_read(inode)) {
    return ERR_PTR(-EINVAL);
    }
//
// This ensures that uprobe_copy_from_page(), copy_to_page() and
// __update_ref_ctr() can't cross page boundary.
//
    if (!IS_ALIGNED(offset, UPROBE_SWBP_INSN_SIZE)) {
    return ERR_PTR(-EINVAL);
    }
    if (!IS_ALIGNED(ref_ctr_offset, sizeof!(short))) {
    return ERR_PTR(-EINVAL);
    }
    uprobe = alloc_uprobe(inode, offset, ref_ctr_offset);
    if (IS_ERR(uprobe)) {
    return uprobe;
    }
    down_write(&uprobe.register_rwsem);
    consumer_add(uprobe, uc);
    ret = register_for_each_vma(uprobe, uc);
    up_write(&uprobe.register_rwsem);
    if (ret) {
    uprobe_unregister_nosync(uprobe, uc);
//
// Registration might have partially succeeded, so we can have
// this consumer being called right at this time. We need to
// sync here. It's ok, it's unlikely slow path.
//
    uprobe_unregister_sync();
    return ERR_PTR(ret);
    }
    return uprobe;
    }
    EXPORT_SYMBOL_GPL(uprobe_register);
//
// uprobe_apply - add or remove the breakpoints according to @uc->filter
// @uprobe: uprobe which "owns" the breakpoint
// @uc: consumer which wants to add more or remove some breakpoints
// @add: add or remove the breakpoints
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_apply(uprobe: *mut uprobe, uc: *mut uprobe_consumer, add: bool) -> c_int {
pub static mut con: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    down_write(&uprobe.register_rwsem);
    rcu_read_lock_trace();
    list_for_each_entry_rcu(con, &uprobe.consumers, cons_node, rcu_read_lock_trace_held()) {
    if (con == uc) {
    ret = register_for_each_vma(uprobe, add ? uc : core::ptr::null_mut());
    break;
    }
    }
    rcu_read_unlock_trace();
    up_write(&uprobe.register_rwsem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unapply_uprobe(uprobe: *mut uprobe, mm: *mut mm_struct) -> c_int {
    VMA_ITERATOR(vmi, mm, 0);
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    mmap_write_lock(mm);
    for_each_vma(vmi, vma) {
    let mut vaddr = 0;
    let mut offset = 0;
    if (!valid_vma(vma, false) ||
    file_inode(vma.vm_file) != uprobe.inode) {
    continue;
    }
    offset = (loff_t)vma_start_pgoff(vma) << PAGE_SHIFT;
    if (uprobe.offset <  offset ||
    uprobe.offset >= offset + vma.vm_end - vma.vm_start) {
    continue;
    }
    vaddr = offset_to_vaddr(vma, uprobe.offset);
    err |= remove_breakpoint(uprobe, vma, vaddr);
    }
    mmap_write_unlock(mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn find_node_in_range(inode: *mut inode, min: loff_t, max: loff_t) -> *mut c_void {
    let mut n = uprobes_tree.rb_node;
    while (n) {
    let mut u = rb_entry(n, uprobe, rb_node);
    if (inode < u.inode) {
    n = n.rb_left;
    } else if (inode > u.inode) {
    n = n.rb_right;
    } else {
    if (max < u.offset) {
    n = n.rb_left;
    }

    else if (min > u.offset) {
    n = n.rb_right;
    }
    else {
    break;
    }
    }
    }
    return n;
    }
//
// For a given range in vma, build a list of probes that need to be inserted.
//
#[no_mangle]
pub unsafe extern "C" fn build_probe_list(inode: *mut inode, vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, head: *mut list_head) {
    loff_t min, max;
    let mut n = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
pub static mut u: *mut c_void = core::ptr::null_mut();
    INIT_LIST_HEAD(head);
    min = vaddr_to_offset(vma, start);
    max = min + (end - start) - 1;
    read_lock(&uprobes_treelock);
    n = find_node_in_range(inode, min, max);
    if (n) {
    for (t = n; t; t = rb_prev(t)) {
    u = rb_entry(t, uprobe, rb_node);
    if (u.inode != inode || u.offset < min) {
    break;
    }
// if uprobe went away, it's safe to ignore it
    if (try_get_uprobe(u)) {
    list_add(&u.pending_list, head);
    }
    }
    while ((t = rb_next(t))) {
    u = rb_entry(t, uprobe, rb_node);
    if (u.inode != inode || u.offset > max) {
    break;
    }
// if uprobe went away, it's safe to ignore it
    if (try_get_uprobe(u)) {
    list_add(&u.pending_list, head);
    }
    }
    }
    read_unlock(&uprobes_treelock);
    }
// @vma contains reference counter, not the probed instruction.
#[no_mangle]
unsafe extern "C" fn delayed_ref_ctr_inc(vma: *mut vm_area_struct) -> c_int {
    let mut pos = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
pub static mut du: *mut c_void = core::ptr::null_mut();
    let mut vaddr = 0;
pub static mut ret: c_int = 0;
    mutex_lock(&delayed_uprobe_lock);
    list_for_each_safe(pos, q, &delayed_uprobe_list) {
    du = list_entry(pos, delayed_uprobe, list);
    if (du.mm != vma.vm_mm ||
    !valid_ref_ctr_vma(du.uprobe, vma)) {
    continue;
    }
    vaddr = offset_to_vaddr(vma, du.uprobe.ref_ctr_offset);
    ret = __update_ref_ctr(vma.vm_mm, vaddr, 1);
    if (ret) {
    update_ref_ctr_warn(du.uprobe, vma.vm_mm, 1);
    if (!err) {
    err = ret;
    }
    }
    delayed_uprobe_delete(du);
    }
    mutex_unlock(&delayed_uprobe_lock);
    return err;
    }
//
// Called from mmap_region/vma_merge with mm->mmap_lock acquired.
//
// Currently we ignore all errors and always return 0, the callers
// can't handle the failure anyway.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_mmap(vma: *mut vm_area_struct) -> c_int {
pub static mut tmp_list: usize = 0;
    let mut uprobe = core::ptr::null_mut();
    let mut u = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
    if (no_uprobe_events()) {
    return 0;
    }
    if (vma.vm_file &&
    (vma.vm_flags & (VM_WRITE|VM_SHARED)) == VM_WRITE &&
    mm_flags_test(MMF_HAS_UPROBES, vma.vm_mm)) {
    delayed_ref_ctr_inc(vma);
    }
    if (!valid_vma(vma, true)) {
    return 0;
    }
    inode = file_inode(vma.vm_file);
    if (!inode) {
    return 0;
    }
    mutex_lock(uprobes_mmap_hash(inode));
    build_probe_list(inode, vma, vma.vm_start, vma.vm_end, &tmp_list);
//
// We can race with uprobe_unregister(), this uprobe can be already
// removed. But in this case filter_chain() must return false, all
// consumers have gone away.
//
    list_for_each_entry_safe(uprobe, u, &tmp_list, pending_list) {
    if (!fatal_signal_pending(current) &&
    filter_chain(uprobe, vma.vm_mm)) {
pub static mut vaddr: c_ulong = 0;
    install_breakpoint(uprobe, vma, vaddr);
    }
    put_uprobe(uprobe);
    }
    mutex_unlock(uprobes_mmap_hash(inode));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vma_has_uprobes(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) -> bool {
    loff_t min, max;
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
    inode = file_inode(vma.vm_file);
    min = vaddr_to_offset(vma, start);
    max = min + (end - start) - 1;
    read_lock(&uprobes_treelock);
    n = find_node_in_range(inode, min, max);
    read_unlock(&uprobes_treelock);
    return !!n;
    }
//
// Called in context of a munmap of a vma.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_munmap(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) {
    if (no_uprobe_events() || !valid_vma(vma, false)) {
    return;
    }
    if (!atomic_read(&vma.vm_mm.mm_users)) /* called by mmput() ? */ {
    return;
    }
    if (!mm_flags_test(MMF_HAS_UPROBES, vma.vm_mm) ||
    mm_flags_test(MMF_RECALC_UPROBES, vma.vm_mm)) {
    return;
    }
    if (vma_has_uprobes(vma, start, end)) {
    mm_flags_set(MMF_RECALC_UPROBES, vma.vm_mm);
    }
    }
    static vm_fault_t xol_fault(const struct vm_special_mapping *sm, vm_area_struct *vma, vm_fault *vmf)
    {
    let mut area = vma.vm_mm.uprobes_state.xol_area;
    vmf.page = area.page;
    get_page(vmf.page);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xol_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> c_int {
    return -EPERM;
    }
pub static mut vm_special_mapping: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_get_xol_area() -> unsigned long __weak {
// Try to map as high as possible, this is only a hint.
    return get_unmapped_area(core::ptr::null_mut(), TASK_SIZE - PAGE_SIZE, PAGE_SIZE, 0, 0);
    }
// Slot allocation for XOL
#[no_mangle]
unsafe extern "C" fn xol_add_vma(mm: *mut mm_struct, area: *mut xol_area) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
    if (mm.uprobes_state.xol_area) {
    ret = -EALREADY;
// goto;
    }
    if (!area.vaddr) {
    area.vaddr = arch_uprobe_get_xol_area();
    if (IS_ERR_VALUE(area.vaddr)) {
    ret = area.vaddr;
// goto;
    }
    }
    vma = _install_special_mapping(mm, area.vaddr, PAGE_SIZE,
    VM_EXEC|VM_MAYEXEC|VM_DONTCOPY|VM_IO|
    VM_SEALED_SYSMAP,
    &xol_mapping);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
// goto;
    }
    ret = 0;
// pairs with get_xol_area()
    smp_store_release(&mm.uprobes_state.xol_area, area); /* ^^^ */
// label;
    mmap_write_unlock(mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uretprobe_trampoline(psize: *mut c_ulong) -> *mut c_void {
pub static mut insn: uprobe_opcode_t = 0;
// psize = UPROBE_SWBP_INSN_SIZE;
    return &insn;
    }
#[no_mangle]
pub unsafe extern "C" fn __create_xol_area(vaddr: c_ulong) -> *mut c_void {
    let mut mm = current.mm;
    let mut insns_size = 0;
pub static mut area: *mut c_void = core::ptr::null_mut();
pub static mut insns: *mut c_void = core::ptr::null_mut();
    area = kzalloc_obj(*area);
    if (unlikely(!area)) {
// goto;
    }
    area.bitmap = kcalloc(BITS_TO_LONGS(UINSNS_PER_PAGE), sizeof!(long),
    GFP_KERNEL);
    if (!area.bitmap) {
// goto;
    }
    area.page = alloc_page(GFP_HIGHUSER | __GFP_ZERO);
    if (!area.page) {
// goto;
    }
    area.vaddr = vaddr;
    init_waitqueue_head(&area.wq);
// Reserve the 1st slot for get_trampoline_vaddr()
    set_bit(0, area.bitmap);
    insns = arch_uretprobe_trampoline(&insns_size);
    arch_uprobe_copy_ixol(area.page, 0, insns, insns_size);
    if (!xol_add_vma(mm, area)) {
    return area;
    }
    __free_page(area.page);
// label;
    kfree(area.bitmap);
// label;
    kfree(area);
// label;
    return core::ptr::null_mut();
    }
//
// get_xol_area - Allocate process's xol_area if necessary.
// This area will be used for storing instructions for execution out of line.
//
// Returns the allocated area or NULL.
//
#[no_mangle]
pub unsafe extern "C" fn get_xol_area() -> *mut c_void {
    let mut mm = current.mm;
pub static mut area: *mut c_void = core::ptr::null_mut();
    if (!mm.uprobes_state.xol_area) {
    __create_xol_area(0);
    }
// Pairs with xol_add_vma() smp_store_release()
    area = READ_ONCE(mm.uprobes_state.xol_area); /* ^^^ */
    return area;
    }
//
// uprobe_clear_state - Free the area allocated for slots.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_clear_state(mm: *mut mm_struct) {
    let mut area = mm.uprobes_state.xol_area;
    mutex_lock(&delayed_uprobe_lock);
    delayed_uprobe_remove(core::ptr::null_mut(), mm);
    mutex_unlock(&delayed_uprobe_lock);
    if (!area) {
    return;
    }
    put_page(area.page);
    kfree(area.bitmap);
    kfree(area);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_start_dup_mmap() {
    percpu_down_read(&dup_mmap_sem);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_end_dup_mmap() {
    percpu_up_read(&dup_mmap_sem);
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_dup_mmap(oldmm: *mut mm_struct, newmm: *mut mm_struct) {
    if (mm_flags_test(MMF_HAS_UPROBES, oldmm)) {
    mm_flags_set(MMF_HAS_UPROBES, newmm);
// unconditionally, dup_mmap() skips VM_DONTCOPY vmas
    mm_flags_set(MMF_RECALC_UPROBES, newmm);
    }
    }
#[no_mangle]
unsafe extern "C" fn xol_get_slot_nr(area: *mut xol_area) -> c_ulong {
    let mut slot_nr = 0;
    slot_nr = find_first_zero_bit(area.bitmap, UINSNS_PER_PAGE);
    if (slot_nr < UINSNS_PER_PAGE) {
    if (!test_and_set_bit(slot_nr, area.bitmap)) {
    return slot_nr;
    }
    }
    return UINSNS_PER_PAGE;
    }
//
// xol_get_insn_slot - allocate a slot for xol.
//
#[no_mangle]
unsafe extern "C" fn xol_get_insn_slot(uprobe: *mut uprobe, utask: *mut uprobe_task) -> bool {
    let mut area = get_xol_area();
    let mut slot_nr = 0;
    if (!area) {
    return false;
    }
    wait_event(area.wq, (slot_nr = xol_get_slot_nr(area)) < UINSNS_PER_PAGE);
    utask.xol_vaddr = area.vaddr + slot_nr * UPROBE_XOL_SLOT_BYTES;
    arch_uprobe_copy_ixol(area.page, utask.xol_vaddr,
    &uprobe.arch.ixol, sizeof!(uprobe.arch.ixol));
    return true;
    }
//
// xol_free_insn_slot - free the slot allocated by xol_get_insn_slot()
//
#[no_mangle]
unsafe extern "C" fn xol_free_insn_slot(utask: *mut uprobe_task) {
    let mut area = current.mm.uprobes_state.xol_area;
pub static mut offset: c_ulong = 0;
    let mut slot_nr = 0;
    utask.xol_vaddr = 0;
// xol_vaddr must fit into [area->vaddr, area->vaddr + PAGE_SIZE)
    if (WARN_ON_ONCE!(offset >= PAGE_SIZE)) {
    return;
    }
    slot_nr = offset / UPROBE_XOL_SLOT_BYTES;
    clear_bit(slot_nr, area.bitmap);
    smp_mb__after_atomic(); /* pairs with prepare_to_wait() */
    if (waitqueue_active(&area.wq)) {
    wake_up(&area.wq);
    }
    }
    void __weak arch_uprobe_copy_ixol(page *page, unsigned long vaddr,
    void *src, unsigned long len)
    {
// Initialize the slot
    copy_to_page(page, vaddr, src, len);
//
// We probably need flush_icache_user_page() but it needs vma.
// This should work on most of architectures by default. If
// architecture needs to do something different it can define
// its own version of the function.
//
    flush_dcache_page(page);
    }
//
// uprobe_get_swbp_addr - compute address of swbp given post-swbp regs
// @regs: Reflects the saved state of the task after it has hit a breakpoint
// instruction.
// Return the address of the breakpoint instruction.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> unsigned long __weak {
    return instruction_pointer(regs) - UPROBE_SWBP_INSN_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_get_trap_addr(regs: *mut pt_regs) -> c_ulong {
    let mut utask = current.utask;
    if (unlikely(utask && utask.active_uprobe)) {
    return utask.vaddr;
    }
    return instruction_pointer(regs);
    }
#[no_mangle]
unsafe extern "C" fn ri_pool_push(utask: *mut uprobe_task, ri: *mut return_instance) {
    ri.cons_cnt = 0;
    ri.next = utask.ri_pool;
    utask.ri_pool = ri;
    }
#[no_mangle]
pub unsafe extern "C" fn ri_pool_pop(utask: *mut uprobe_task) -> *mut c_void {
    let mut ri = utask.ri_pool;
    if (likely(ri)) {
    utask.ri_pool = ri.next;
    }
    return ri;
    }
#[no_mangle]
unsafe extern "C" fn ri_free(ri: *mut return_instance) {
    kfree(ri.extra_consumers);
    kfree_rcu(ri, rcu);
    }
#[no_mangle]
pub unsafe extern "C" fn free_ret_instance(utask: *mut uprobe_task, ri: *mut return_instance, cleanup_hprobe: bool) {
    let mut seq: c_uint = 0;
    if (cleanup_hprobe) {
    enum hprobe_state hstate;
    (void)hprobe_consume(&ri.hprobe, &hstate);
    hprobe_finalize(&ri.hprobe, hstate);
    }
//
// At this point return_instance is unlinked from utask's
// return_instances list and this has become visible to ri_timer().
// If seqcount now indicates that ri_timer's return instance
// processing loop isn't active, we can return ri into the pool of
// to-be-reused return instances for future uretprobes. If ri_timer()
// happens to be running right now, though, we fallback to safety and
// just perform RCU-delated freeing of ri.
// Admittedly, this is a rather simple use of seqcount, but it nicely
// abstracts away all the necessary memory barriers, so we use
// a well-supported kernel primitive here.
//
    if (raw_seqcount_try_begin(&utask.ri_seqcount, seq)) {
// immediate reuse of ri without RCU GP is OK
    ri_pool_push(utask, ri);
    } else {
// we might be racing with ri_timer(), so play it safe
    ri_free(ri);
    }
    }
//
// Called with no locks held.
// Called in context of an exiting or an exec-ing thread.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_free_utask(t: *mut task_struct) {
    let mut utask = t.utask;
    let mut ri = core::ptr::null_mut();
    let mut ri_next = core::ptr::null_mut();
    if (!utask) {
    return;
    }
    t.utask = core::ptr::null_mut();
    WARN_ON_ONCE!(utask.active_uprobe || utask.xol_vaddr);
    timer_delete_sync(&utask.ri_timer);
    ri = utask.return_instances;
    while (ri) {
    ri_next = ri.next;
    free_ret_instance(utask, ri, true /* cleanup_hprobe */);
    ri = ri_next;
    }
// free_ret_instance() above might add to ri_pool, so this loop should come last
    ri = utask.ri_pool;
    while (ri) {
    ri_next = ri.next;
    ri_free(ri);
    ri = ri_next;
    }
    kfree(utask);
    }

    for (pos = rcu_dereference_raw(head); pos; pos = rcu_dereference_raw(pos.next)) {
#[no_mangle]
unsafe extern "C" fn ri_timer(timer: *mut timer_list) {
    }
    let mut utask = container_of!(timer, uprobe_task, ri_timer);
pub static mut ri: *mut c_void = core::ptr::null_mut();
// SRCU protects uprobe from reuse for the cmpxchg() inside hprobe_expire().
    guard(srcu_fast_updown)(&uretprobes_srcu);
// RCU protects return_instance from freeing.
    guard(rcu)();
//
// See free_ret_instance() for notes on seqcount use.
// We also employ raw API variants to avoid lockdep false-positive
// warning complaining about enabled preemption. The timer can only be
// invoked once for a uprobe_task. Therefore there can only be one
// writer. The reader does not require an even sequence count to make
// progress, so it is OK to remain preemptible on PREEMPT_RT.
//
    raw_write_seqcount_begin(&utask.ri_seqcount);
    for_each_ret_instance_rcu(ri, utask.return_instances) {
    hprobe_expire(&ri.hprobe, false);
    }
    raw_write_seqcount_end(&utask.ri_seqcount);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_utask() -> *mut c_void {
pub static mut utask: *mut c_void = core::ptr::null_mut();
    utask = kzalloc_obj(*utask);
    if (!utask) {
    return core::ptr::null_mut();
    }
    timer_setup(&utask.ri_timer, ri_timer, 0);
    seqcount_init(&utask.ri_seqcount);
    return utask;
    }
//
// Allocate a uprobe_task object for the task if necessary.
// Called when the thread hits a breakpoint.
//
// Returns:
// - pointer to new uprobe_task on success
// - NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn get_utask() -> *mut c_void {
    if (!current.utask) {
    current.utask = alloc_utask();
    }
    return current.utask;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_return_instance(utask: *mut uprobe_task) -> *mut c_void {
pub static mut ri: *mut c_void = core::ptr::null_mut();
    ri = ri_pool_pop(utask);
    if (ri) {
    return ri;
    }
    ri = kzalloc_obj(*ri);
    if (!ri) {
    return ZERO_SIZE_PTR;
    }
    return ri;
    }
#[no_mangle]
pub unsafe extern "C" fn dup_return_instance(old: *mut return_instance) -> *mut c_void {
pub static mut ri: *mut c_void = core::ptr::null_mut();
    ri = kmemdup(old, sizeof!(*ri), GFP_KERNEL);
    if (!ri) {
    return core::ptr::null_mut();
    }
    if (unlikely(old.cons_cnt > 1)) {
    ri.extra_consumers = kmemdup(old.extra_consumers,
    sizeof!(ri.extra_consumers[0]) * (old.cons_cnt - 1),
    GFP_KERNEL);
    if (!ri.extra_consumers) {
    kfree(ri);
    return core::ptr::null_mut();
    }
    }
    return ri;
    }
#[no_mangle]
unsafe extern "C" fn dup_utask(t: *mut task_struct, o_utask: *mut uprobe_task) -> c_int {
pub static mut n_utask: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut o = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    n_utask = alloc_utask();
    if (!n_utask) {
    return -ENOMEM;
    }
    t.utask = n_utask;
// protect uprobes from freeing, we'll need try_get_uprobe() them
    guard(srcu_fast_updown)(&uretprobes_srcu);
    p = &n_utask.return_instances;
    while (o) {
    n = dup_return_instance(o);
    if (!n) {
    return -ENOMEM;
    }
// if uprobe is non-NULL, we'll have an extra refcount for uprobe
    uprobe = hprobe_expire(&o.hprobe, true);
//
// New utask will have stable properly refcounted uprobe or
// NULL. Even if we failed to get refcounted uprobe, we still
// need to preserve full set of return_instances for proper
// uretprobe handling and nesting in forked task.
//
    hprobe_init_stable(&n.hprobe, uprobe);
    n.next = core::ptr::null_mut();
    rcu_assign_pointer(*p, n);
    p = &n.next;
    n_utask.depth += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dup_xol_work(work: *mut callback_head) {
    if (current.flags & PF_EXITING) {
    return;
    }
    if (!__create_xol_area(current.utask.dup_xol_addr) &&
    !fatal_signal_pending(current)) {
    uprobe_warn(current, "dup xol area");
    }
    }
//
// Called in context of a new clone/fork from copy_process.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_copy_process(t: *mut task_struct, flags: u64) {
    let mut utask = current.utask;
    let mut mm = current.mm;
pub static mut area: *mut c_void = core::ptr::null_mut();
    t.utask = core::ptr::null_mut();
    if (!utask || !utask.return_instances) {
    return;
    }
    if (mm == t.mm && !(flags & CLONE_VFORK)) {
    return;
    }
    if (dup_utask(t, utask)) {
    return uprobe_warn(t, "dup ret instances");
    }
// The task can fork() after dup_xol_work() fails
    area = mm.uprobes_state.xol_area;
    if (!area) {
    return uprobe_warn(t, "dup xol area");
    }
    if (mm == t.mm) {
    return;
    }
    t.utask.dup_xol_addr = area.vaddr;
    init_task_work(&t.utask.dup_xol_work, dup_xol_work);
    task_work_add(t, &t.utask.dup_xol_work, TWA_RESUME);
    }
//
// Current area->vaddr notion assume the trampoline address is always
// equal area->vaddr.
//
// Returns -1 in case the xol_area is not allocated.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_get_trampoline_vaddr() -> c_ulong {
pub static mut trampoline_vaddr: c_ulong = 0;
pub static mut area: *mut c_void = core::ptr::null_mut();
// Pairs with xol_add_vma() smp_store_release()
    area = READ_ONCE(current.mm.uprobes_state.xol_area); /* ^^^ */
    if (area) {
    trampoline_vaddr = area.vaddr;
    }
    return trampoline_vaddr;
    }
#[no_mangle]
pub unsafe extern "C" fn cleanup_return_instances(utask: *mut uprobe_task, chained: bool, regs: *mut pt_regs) {
    let mut ri = utask.return_instances, *ri_next;
pub static mut ctx: rp_check = 0;
    while (ri && !arch_uretprobe_is_alive(ri, ctx, regs)) {
    ri_next = ri.next;
    rcu_assign_pointer(utask.return_instances, ri_next);
    utask.depth -= 1;
    free_ret_instance(utask, ri, true /* cleanup_hprobe */);
    ri = ri_next;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_uretprobe(uprobe: *mut uprobe, regs: *mut pt_regs, ri: *mut return_instance) {
    let mut utask = current.utask;
    unsigned long orig_ret_vaddr, trampoline_vaddr;
    let mut srcu_scp = core::ptr::null_mut();
    let mut chained = 0;
    if (!get_xol_area()) {
// goto;
    }
    if (utask.depth >= MAX_URETPROBE_DEPTH) {
    printk_ratelimited("uprobe: omit uretprobe due to"
    " nestedness limit pid/tgid=%d/%d\n",
    current.pid, current.tgid);
// goto;
    }
    trampoline_vaddr = uprobe_get_trampoline_vaddr();
    orig_ret_vaddr = arch_uretprobe_hijack_return_addr(trampoline_vaddr, regs);
    if (orig_ret_vaddr == -1) {
// goto;
    }
// drop the entries invalidated by longjmp()
    chained = (orig_ret_vaddr == trampoline_vaddr);
    cleanup_return_instances(utask, chained, regs);
//
// We don't want to keep trampoline address in stack, rather keep the
// original return address of first caller thru all the consequent
// instances. This also makes breakpoint unwrapping easier.
//
    if (chained) {
    if (!utask.return_instances) {
//
// This situation is not possible. Likely we have an
// attack from user-space.
//
    uprobe_warn(current, "handle tail call");
// goto;
    }
    orig_ret_vaddr = utask.return_instances.orig_ret_vaddr;
    }
//
// Use srcu_down_read_fast() because the SRCU lock survives a switch to
// user space and can be unlocked from a different context by ri_timer()
// or dup_utask().
//
    srcu_scp = srcu_down_read_fast(&uretprobes_srcu);
    ri.func = instruction_pointer(regs);
    ri.stack = user_stack_pointer(regs);
    ri.orig_ret_vaddr = orig_ret_vaddr;
    ri.chained = chained;
    utask.depth += 1;
    hprobe_init_leased(&ri.hprobe, uprobe, srcu_scp);
    ri.next = utask.return_instances;
    rcu_assign_pointer(utask.return_instances, ri);
    mod_timer(&utask.ri_timer, jiffies + RI_TIMER_PERIOD);
    return;
// label;
    ri_free(ri);
    }
// Prepare to single-step probed instruction out of line.
#[no_mangle]
pub unsafe extern "C" fn pre_ssout(uprobe: *mut uprobe, regs: *mut pt_regs, bp_vaddr: c_ulong) -> c_int {
    let mut utask = current.utask;
    let mut err = 0;
    if (!try_get_uprobe(uprobe)) {
    return -EINVAL;
    }
    if (!xol_get_insn_slot(uprobe, utask)) {
    err = -ENOMEM;
// goto;
    }
    utask.vaddr = bp_vaddr;
    err = arch_uprobe_pre_xol(&uprobe.arch, regs);
    if (unlikely(err)) {
    xol_free_insn_slot(utask);
// goto;
    }
    utask.active_uprobe = uprobe;
    utask.state = UTASK_SSTEP;
    return 0;
// label;
    put_uprobe(uprobe);
    return err;
    }
//
// If we are singlestepping, then ensure this thread is not connected to
// non-fatal signals until completion of singlestep.  When xol insn itself
// triggers the signal,  restart the original insn even if the task is
// already SIGKILL'ed (since coredump should report the correct ip).  This
// is even more important if the task has a handler for SIGSEGV/etc, The
// _same_ instruction should be repeated again after return from the signal
// handler, and SSTEP can never finish in this case.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_deny_signal() -> bool {
    let mut t = current;
    let mut utask = t.utask;
    if (likely(!utask || !utask.active_uprobe)) {
    return false;
    }
    WARN_ON_ONCE!(utask.state != UTASK_SSTEP);
    if (task_sigpending(t)) {
    utask.signal_denied = true;
    clear_tsk_thread_flag(t, TIF_SIGPENDING);
    if (__fatal_signal_pending(t) || arch_uprobe_xol_was_trapped(t)) {
    utask.state = UTASK_SSTEP_TRAPPED;
    set_tsk_thread_flag(t, TIF_UPROBE);
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mmf_recalc_uprobes(mm: *mut mm_struct) {
    VMA_ITERATOR(vmi, mm, 0);
pub static mut vma: *mut c_void = core::ptr::null_mut();
    for_each_vma(vmi, vma) {
    if (!valid_vma(vma, false)) {
    continue;
    }
//
// This is not strictly accurate, we can race with
// uprobe_unregister() and see the already removed
// uprobe if delete_uprobe() was not yet called.
// Or this uprobe can be filtered out.
//
    if (vma_has_uprobes(vma, vma.vm_start, vma.vm_end)) {
    return;
    }
    }
    mm_flags_clear(MMF_HAS_UPROBES, mm);
    }
#[no_mangle]
unsafe extern "C" fn is_trap_at_addr(mm: *mut mm_struct, vaddr: c_ulong) -> c_int {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut opcode;
    let mut result = 0;
    if (WARN_ON_ONCE!(!IS_ALIGNED(vaddr, UPROBE_SWBP_INSN_SIZE))) {
    return -EINVAL;
    }
    pagefault_disable();
    result = __get_user(opcode, vaddr);
    pagefault_enable();
    if (likely(result == 0)) {
// goto;
    }
    result = get_user_pages(vaddr, 1, FOLL_FORCE, &page);
    if (result < 0) {
    return result;
    }
    uprobe_copy_from_page(page, vaddr, &opcode, UPROBE_SWBP_INSN_SIZE);
    put_page(page);
// label;
// This needs to return true for any variant of the trap insn
    return is_trap_insn(&opcode);
    }
#[no_mangle]
pub unsafe extern "C" fn find_active_uprobe_speculative(bp_vaddr: c_ulong) -> *mut c_void {
    let mut mm = current.mm;
    let mut uprobe = core::ptr::null_mut();
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut vm_file: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    let mut seq = 0;
    guard(rcu)();
    if (!mmap_lock_speculate_try_begin(mm, &seq)) {
    return core::ptr::null_mut();
    }
    vma = vma_lookup(mm, bp_vaddr);
    if (!vma) {
    return core::ptr::null_mut();
    }
//
// vm_file memory can be reused for another instance of struct file,
// but can't be freed from under us, so it's safe to read fields from
// it, even if the values are some garbage values; ultimately
// find_uprobe_rcu() + mmap_lock_speculation_end() check will ensure
// that whatever we speculatively found is correct
//
    vm_file = READ_ONCE(vma.vm_file);
    if (!vm_file) {
    return core::ptr::null_mut();
    }
    offset = (loff_t)(vma_start_pgoff(vma) << PAGE_SHIFT) +
    (bp_vaddr - vma.vm_start);
    uprobe = find_uprobe_rcu(vm_file.f_inode, offset);
    if (!uprobe) {
    return core::ptr::null_mut();
    }
// now double check that nothing about MM changed
    if (mmap_lock_speculate_retry(mm, seq)) {
    return core::ptr::null_mut();
    }
    return uprobe;
    }
// assumes being inside RCU protected region
#[no_mangle]
pub unsafe extern "C" fn find_active_uprobe_rcu(bp_vaddr: c_ulong, is_swbp: *mut c_int) -> *mut c_void {
    let mut mm = current.mm;
    let mut uprobe = core::ptr::null_mut();
pub static mut vma: *mut c_void = core::ptr::null_mut();
    uprobe = find_active_uprobe_speculative(bp_vaddr);
    if (uprobe) {
    return uprobe;
    }
    mmap_read_lock(mm);
    vma = vma_lookup(mm, bp_vaddr);
    if (vma) {
    if (vma.vm_file) {
    let mut inode = file_inode(vma.vm_file);
pub static mut offset: loff_t = 0;
    uprobe = find_uprobe_rcu(inode, offset);
    }
    if (!uprobe) {
// is_swbp = is_trap_at_addr(mm, bp_vaddr);
    }
    } else {
// is_swbp = -EFAULT;
    }
    if (!uprobe && mm_flags_test_and_clear(MMF_RECALC_UPROBES, mm)) {
    mmf_recalc_uprobes(mm);
    }
    mmap_read_unlock(mm);
    return uprobe;
    }
#[no_mangle]
pub unsafe extern "C" fn push_consumer(ri: *mut return_instance, id: __u64, cookie: __u64) -> *mut c_void {
pub static mut ric: *mut c_void = core::ptr::null_mut();
    if (unlikely(ri == ZERO_SIZE_PTR)) {
    return ri;
    }
    if (unlikely(ri.cons_cnt > 0)) {
    ric = krealloc(ri.extra_consumers, sizeof!(*ric) * ri.cons_cnt, GFP_KERNEL);
    if (!ric) {
    ri_free(ri);
    return ZERO_SIZE_PTR;
    }
    ri.extra_consumers = ric;
    }
    ric = likely(ri.cons_cnt == 0) ? &ri.consumer : &ri.extra_consumers[ri.cons_cnt - 1];
    ric.id = id;
    ric.cookie = cookie;
    ri.cons_cnt += 1;
    return ri;
    }
#[no_mangle]
pub unsafe extern "C" fn return_consumer_find(ri: *mut return_instance, iter: *mut c_int, id: c_int) -> *mut c_void {
pub static mut ric: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    while (idx < ri.cons_cnt) {
    ric = likely(idx == 0) ? &ri.consumer : &ri.extra_consumers[idx - 1];
    if (ric.id == id) {
// iter = idx + 1;
    return ric;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ignore_ret_handler(rc: c_int) -> bool {
pub static mut rc: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn handler_chain(uprobe: *mut uprobe, regs: *mut pt_regs) {
pub static mut uc: *mut c_void = core::ptr::null_mut();
pub static mut has_consumers: bool = false;
    let mut ri = core::ptr::null_mut();
    let mut utask = current.utask;
    utask.auprobe = &uprobe.arch;
    list_for_each_entry_rcu(uc, &uprobe.consumers, cons_node, rcu_read_lock_trace_held()) {
pub static mut session: bool = false;
pub static mut cookie: __u64 = 0;
pub static mut rc: c_int = 0;
    if (uc.handler) {
    rc = uc.handler(uc, regs, &cookie);
    WARN(rc < 0 || rc > 2,
    "bad rc=0x%x from %ps()\n", rc, uc.handler);
    }
    remove &= rc == UPROBE_HANDLER_REMOVE;
    has_consumers = true;
    if (!uc.ret_handler || ignore_ret_handler(rc)) {
    continue;
    }
    if (!ri) {
    ri = alloc_return_instance(utask);
    }
    if (session) {
    ri = push_consumer(ri, uc.id, cookie);
    }
    }
    utask.auprobe = core::ptr::null_mut();
    if (!ZERO_OR_NULL_PTR(ri)) {
    prepare_uretprobe(uprobe, regs, ri);
    }
    if (remove && has_consumers) {
    down_read(&uprobe.register_rwsem);
// re-check that removal is still required, this time under lock
    if (!filter_chain(uprobe, current.mm)) {
    WARN_ON!(!uprobe_is_active(uprobe));
    unapply_uprobe(uprobe, current.mm);
    }
    up_read(&uprobe.register_rwsem);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe_chain(ri: *mut return_instance, uprobe: *mut uprobe, regs: *mut pt_regs) {
pub static mut ric: *mut c_void = core::ptr::null_mut();
pub static mut uc: *mut c_void = core::ptr::null_mut();
pub static mut ric_idx: c_int = 0;
// all consumers unsubscribed meanwhile
    if (unlikely(!uprobe)) {
    return;
    }
    rcu_read_lock_trace();
    list_for_each_entry_rcu(uc, &uprobe.consumers, cons_node, rcu_read_lock_trace_held()) {
pub static mut session: bool = false;
    if (uc.ret_handler) {
    ric = return_consumer_find(ri, &ric_idx, uc.id);
    if (!session || ric) {
    uc.ret_handler(uc, ri.func, regs, ric ? &ric.cookie : core::ptr::null_mut());
    }
    }
    }
    rcu_read_unlock_trace();
    }
#[no_mangle]
pub unsafe extern "C" fn find_next_ret_chain(ri: *mut return_instance) -> *mut c_void {
    let mut chained = 0;
    do {
    chained = ri.chained;
    ri = ri.next;	/* can't be core::ptr::null_mut() if chained */
    } while (chained);
    return ri;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_handle_trampoline(regs: *mut pt_regs) {
pub static mut utask: *mut c_void = core::ptr::null_mut();
    let mut ri = core::ptr::null_mut();
    let mut ri_next = core::ptr::null_mut();
    let mut next_chain = core::ptr::null_mut();
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    enum hprobe_state hstate;
    let mut valid = 0;
    utask = current.utask;
    if (!utask) {
// goto;
    }
    ri = utask.return_instances;
    if (!ri) {
// goto;
    }
    do {
//
// We should throw out the frames invalidated by longjmp().
// If this chain is valid, then the next one should be alive
// or NULL; the latter case means that nobody but ri->func
// could hit this trampoline on return. TODO: sigaltstack().
//
    next_chain = find_next_ret_chain(ri);
    valid = !next_chain || arch_uretprobe_is_alive(next_chain, RP_CHECK_RET, regs);
    instruction_pointer_set(regs, ri.orig_ret_vaddr);
    do {
// pop current instance from the stack of pending return instances,
// as it's not pending anymore: we just fixed up original
// instruction pointer in regs and are about to call handlers;
// this allows fixup_uretprobe_trampoline_entries() to properly fix up
// captured stack traces from uretprobe handlers, in which pending
// trampoline addresses on the stack are replaced with correct
// original return addresses
//
    ri_next = ri.next;
    rcu_assign_pointer(utask.return_instances, ri_next);
    utask.depth -= 1;
    uprobe = hprobe_consume(&ri.hprobe, &hstate);
    if (valid) {
    handle_uretprobe_chain(ri, uprobe, regs);
    }
    hprobe_finalize(&ri.hprobe, hstate);
// We already took care of hprobe, no need to waste more time on that.
    free_ret_instance(utask, ri, false /* !cleanup_hprobe */);
    ri = ri_next;
    } while (ri != next_chain);
    } while (!valid);
    return;
// label;
    uprobe_warn(current, "handle uretprobe, sending SIGILL.");
    force_sig(SIGILL);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_ignore(aup: *mut arch_uprobe, regs: *mut pt_regs) -> bool __weak {
    return false;
    }
    bool __weak arch_uretprobe_is_alive(return_instance *ret, enum rp_check ctx, pt_regs *regs)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_uprobe_optimize(auprobe: *mut arch_uprobe, vaddr: c_ulong) -> void __weak {
    }
//
// Run handler and ask thread to singlestep.
// Ensure all non-fatal signals cannot interrupt thread while it singlesteps.
//
#[no_mangle]
unsafe extern "C" fn handle_swbp(regs: *mut pt_regs) {
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    let mut bp_vaddr = 0;
    let mut is_swbp = 0;
    bp_vaddr = uprobe_get_swbp_addr(regs);
    if (bp_vaddr == uprobe_get_trampoline_vaddr()) {
    return uprobe_handle_trampoline(regs);
    }
    rcu_read_lock_trace();
    uprobe = find_active_uprobe_rcu(bp_vaddr, &is_swbp);
    if (!uprobe) {
    if (is_swbp > 0) {
// No matching uprobe; signal SIGTRAP.
    force_sig(SIGTRAP);
    } else {
//
// Either we raced with uprobe_unregister() or we can't
// access this memory. The latter is only possible if
// another thread plays with our ->mm. In both cases
// we can simply restart. If this vma was unmapped we
// can pretend this insn was not executed yet and get
// the (correct) SIGSEGV after restart.
//
    instruction_pointer_set(regs, bp_vaddr);
    }
// goto;
    }
// change it in advance for ->handler() and restart
    instruction_pointer_set(regs, bp_vaddr);
//
// TODO: move copy_insn/etc into _register and remove this hack.
// After we hit the bp, _unregister + _register can install the
// new and not-yet-analyzed uprobe at the same address, restart.
//
    if (unlikely(!test_bit(UPROBE_COPY_INSN, &uprobe.flags))) {
// goto;
    }
//
// Pairs with the smp_wmb() in prepare_uprobe().
//
// Guarantees that if we see the UPROBE_COPY_INSN bit set, then
// we must also see the stores to &uprobe->arch performed by the
// prepare_uprobe() call.
//
    smp_rmb();
// Tracing handlers use ->utask to communicate with fetch methods
    if (!get_utask()) {
// goto;
    }
    if (arch_uprobe_ignore(&uprobe.arch, regs)) {
// goto;
    }
    handler_chain(uprobe, regs);
// Try to optimize after first hit.
    arch_uprobe_optimize(&uprobe.arch, bp_vaddr);
//
// If user decided to take execution elsewhere, it makes little sense
// to execute the original instruction, so let's skip it.
//
    if (instruction_pointer(regs) != bp_vaddr) {
// goto;
    }
    if (arch_uprobe_skip_sstep(&uprobe.arch, regs)) {
// goto;
    }
    if (pre_ssout(uprobe, regs, bp_vaddr)) {
// goto;
    }
// label;
// arch_uprobe_skip_sstep() succeeded, or restart if can't singlestep
    rcu_read_unlock_trace();
    }
#[no_mangle]
pub unsafe extern "C" fn handle_syscall_uprobe(regs: *mut pt_regs, bp_vaddr: c_ulong) {
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
    let mut is_swbp = 0;
    guard(rcu_tasks_trace)();
    uprobe = find_active_uprobe_rcu(bp_vaddr, &is_swbp);
    if (!uprobe) {
    return;
    }
    if (!get_utask()) {
    return;
    }
    if (arch_uprobe_ignore(&uprobe.arch, regs)) {
    return;
    }
    handler_chain(uprobe, regs);
    }
//
// Perform required fix-ups and disable singlestep.
// Allow pending signals to take effect.
//
#[no_mangle]
unsafe extern "C" fn handle_singlestep(utask: *mut uprobe_task, regs: *mut pt_regs) {
pub static mut uprobe: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    uprobe = utask.active_uprobe;
    if (utask.state == UTASK_SSTEP_ACK) {
    err = arch_uprobe_post_xol(&uprobe.arch, regs);
    }

    else if (utask.state == UTASK_SSTEP_TRAPPED) {
    arch_uprobe_abort_xol(&uprobe.arch, regs);
    }
    else {
    WARN_ON_ONCE!(1);
    }
    put_uprobe(uprobe);
    utask.active_uprobe = core::ptr::null_mut();
    utask.state = UTASK_RUNNING;
    xol_free_insn_slot(utask);
    if (utask.signal_denied) {
    set_thread_flag(TIF_SIGPENDING);
    utask.signal_denied = false;
    }
    if (unlikely(err)) {
    uprobe_warn(current, "execute the probed insn, sending SIGILL.");
    force_sig(SIGILL);
    }
    }
//
// On breakpoint hit, breakpoint notifier sets the TIF_UPROBE flag and
// allows the thread to return from interrupt. After that handle_swbp()
// sets utask->active_uprobe.
//
// On singlestep exception, singlestep notifier sets the TIF_UPROBE flag
// and allows the thread to return from interrupt.
//
// While returning to userspace, thread notices the TIF_UPROBE flag and calls
// uprobe_notify_resume().
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_notify_resume(regs: *mut pt_regs) {
pub static mut utask: *mut c_void = core::ptr::null_mut();
    clear_thread_flag(TIF_UPROBE);
    utask = current.utask;
    if (utask && utask.active_uprobe) {
    handle_singlestep(utask, regs);
    }
    else {
    handle_swbp(regs);
    }
    }
//
// uprobe_pre_sstep_notifier gets called from interrupt context as part of
// notifier mechanism. Set TIF_UPROBE flag and indicate breakpoint hit.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_pre_sstep_notifier(regs: *mut pt_regs) -> c_int {
    if (!current.mm) {
    return 0;
    }
    if (!mm_flags_test(MMF_HAS_UPROBES, current.mm) &&
    (!current.utask || !current.utask.return_instances)) {
    return 0;
    }
    set_thread_flag(TIF_UPROBE);
    return 1;
    }
//
// uprobe_post_sstep_notifier gets called in interrupt context as part of notifier
// mechanism. Set TIF_UPROBE flag and indicate completion of singlestep.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_post_sstep_notifier(regs: *mut pt_regs) -> c_int {
    let mut utask = current.utask;
    if (!current.mm || !utask || !utask.active_uprobe) {
// task is currently not uprobed
    return 0;
    }
    utask.state = UTASK_SSTEP_ACK;
    set_thread_flag(TIF_UPROBE);
    return 1;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn uprobes_init()  {
    let mut i = 0;
    for (i = 0; i < UPROBES_HASH_SZ; i++) {
    mutex_init(&uprobes_mmap_mutex[i]);
    }
    BUG_ON!(register_die_notifier(&uprobe_exception_nb));