//! Automatically rewritten from C to Rust
//! Source: mm/mmap_lock.c
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
// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL(mmap_lock_start_locking);
    EXPORT_TRACEPOINT_SYMBOL(mmap_lock_acquire_returned);
    EXPORT_TRACEPOINT_SYMBOL(mmap_lock_released);

//
// Trace calls must be in a separate file, as otherwise there's a circular
// dependency between linux/mmap_lock.h and trace/events/mmap_lock.h.
//
#[no_mangle]
pub unsafe extern "C" fn __mmap_lock_do_trace_start_locking(mm: *mut mm_struct, write: bool) {
    trace_mmap_lock_start_locking(mm, write);
    }
    EXPORT_SYMBOL(__mmap_lock_do_trace_start_locking);
#[no_mangle]
pub unsafe extern "C" fn __mmap_lock_do_trace_acquire_returned(mm: *mut mm_struct, write: bool, success: bool) {
    trace_mmap_lock_acquire_returned(mm, write, success);
    }
    EXPORT_SYMBOL(__mmap_lock_do_trace_acquire_returned);
#[no_mangle]
pub unsafe extern "C" fn __mmap_lock_do_trace_released(mm: *mut mm_struct, write: bool) {
    trace_mmap_lock_released(mm, write);
    }
    EXPORT_SYMBOL(__mmap_lock_do_trace_released);

// State shared across __vma_[start, end]_exclude_readers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_exclude_readers_state {
// Input parameters.
    pub vma: *mut vm_area_struct,
//     pub /: *mut *mut int state; / TASK_KILLABLE or TASK_UNINTERRUPTIBLE.,
    pub detaching: bool,
// Output parameters.
    pub detached: bool,
//     pub /: *mut *mut bool exclusive; / Are we exclusively locked?,
}

//
// Now that all readers have been evicted, mark the VMA as being out of the
// 'exclude readers' state.
//
#[no_mangle]
unsafe extern "C" fn __vma_end_exclude_readers(ves: *mut vma_exclude_readers_state) {
    let mut vma = ves.vma;
    VM_WARN_ON_ONCE(ves.detached);
    ves.detached = refcount_sub_and_test(VM_REFCNT_EXCLUDE_READERS_FLAG,
    &vma.vm_refcnt);
    __vma_lockdep_release_exclusive(vma);
    }
#[no_mangle]
unsafe extern "C" fn get_target_refcnt(ves: *mut vma_exclude_readers_state) -> c_uint {
pub static mut tgt: c_uint = 0;
    return tgt | VM_REFCNT_EXCLUDE_READERS_FLAG;
    }
//
// Mark the VMA as being in a state of excluding readers, check to see if any
// VMA read locks are indeed held, and if so wait for them to be released.
//
// Note that this function pairs with vma_refcount_put() which will wake up this
// thread when it detects that the last reader has released its lock.
//
// The ves->state parameter ought to be set to TASK_UNINTERRUPTIBLE in cases
// where we wish the thread to sleep uninterruptibly or TASK_KILLABLE if a fatal
// signal is permitted to kill it.
//
// The function sets the ves->exclusive parameter to true if readers were
// excluded, or false if the VMA was detached or an error arose on wait.
//
// If the function indicates an exclusive lock was acquired via ves->exclusive
// the caller is required to invoke __vma_end_exclude_readers() once the
// exclusive state is no longer required.
//
// If ves->state is set to something other than TASK_UNINTERRUPTIBLE, the
// function may also return -EINTR to indicate a fatal signal was received while
// waiting.  Otherwise, the function returns 0.
//
#[no_mangle]
unsafe extern "C" fn __vma_start_exclude_readers(ves: *mut vma_exclude_readers_state) -> c_int {
    let mut vma = ves.vma;
pub static mut tgt_refcnt: c_uint = 0;
pub static mut err: c_int = 0;
    mmap_assert_write_locked(vma.vm_mm);
//
// If vma is detached then only vma_mark_attached() can raise the
// vm_refcnt. mmap_write_lock prevents racing with vma_mark_attached().
//
// See the comment describing the vm_area_struct->vm_refcnt field for
// details of possible refcnt values.
//
    if (!refcount_add_not_zero(VM_REFCNT_EXCLUDE_READERS_FLAG, &vma.vm_refcnt)) {
    ves.detached = true;
    return 0;
    }
    __vma_lockdep_acquire_exclusive(vma);
    err = rcuwait_wait_event(&vma.vm_mm.vma_writer_wait,
    refcount_read(&vma.vm_refcnt) == tgt_refcnt,
    ves.state);
    if (err) {
    __vma_end_exclude_readers(ves);
    return err;
    }
    __vma_lockdep_stat_mark_acquired(vma);
    ves.exclusive = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __vma_start_write(vma: *mut vm_area_struct, state: c_int) -> c_int {
pub static mut mm_lock_seq: c_uint = 0;
pub static mut vma_exclude_readers_state: usize = 0;
    let mut err = 0;
    err = __vma_start_exclude_readers(&ves);
    if (err) {
    WARN_ON_ONCE!(ves.detached);
    return err;
    }
//
// We should use WRITE_ONCE() here because we can have concurrent reads
// from the early lockless pessimistic check in vma_start_read().
// We don't really care about the correctness of that early check, but
// we should use WRITE_ONCE() for cleanliness and to keep KCSAN happy.
//
    WRITE_ONCE(vma.vm_lock_seq, mm_lock_seq);
    if (ves.exclusive) {
    __vma_end_exclude_readers(&ves);
// VMA should remain attached.
    WARN_ON_ONCE!(ves.detached);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(__vma_start_write);
#[no_mangle]
pub unsafe extern "C" fn __vma_exclude_readers_for_detach(vma: *mut vm_area_struct) {
pub static mut vma_exclude_readers_state: usize = 0;
    let mut err = 0;
//
// Wait until the VMA is detached with no readers. Since we hold the VMA
// write lock, the only read locks that might be present are those from
// threads trying to acquire the read lock and incrementing the
// reference count before realising the write lock is held and
// decrementing it.
//
    err = __vma_start_exclude_readers(&ves);
    if (!err && ves.exclusive) {
//
// Once this is complete, no readers can increment the
// reference count, and the VMA is marked detached.
//
    __vma_end_exclude_readers(&ves);
    }
// If an error arose but we were detached anyway, we don't care.
    WARN_ON_ONCE!(!ves.detached);
    }
//
// Try to read-lock a vma. The function is allowed to occasionally yield false
// locked result to avoid performance overhead, in which case we fall back to
// using mmap_lock. The function should never yield false unlocked result.
// False locked result is possible if mm_lock_seq overflows or if vma gets
// reused and attached to a different mm before we lock it.
// Returns the vma on success, NULL on failure to lock and EAGAIN if vma got
// detached.
//
// IMPORTANT: RCU lock must be held upon entering the function, but upon error
// IT IS RELEASED. The caller must handle this correctly.
//
#[no_mangle]
pub unsafe extern "C" fn vma_start_read(mm: *mut mm_struct, vma: *mut vm_area_struct) -> *mut c_void {
pub static mut other_mm: *mut c_void = core::ptr::null_mut();
    let mut oldcnt = 0;
    RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "no rcu lock held");
//
// Check before locking. A race might cause false locked result.
// We can use READ_ONCE() for the mm_lock_seq here, and don't need
// ACQUIRE semantics, because this is just a lockless check whose result
// we don't rely on for anything - the mm_lock_seq read against which we
// need ordering is below.
//
    if (READ_ONCE(vma.vm_lock_seq) == READ_ONCE(mm.mm_lock_seq.sequence)) {
    vma = core::ptr::null_mut();
// goto;
    }
//
// If VM_REFCNT_EXCLUDE_READERS_FLAG is set,
// __refcount_inc_not_zero_limited_acquire() will fail because
// VM_REFCNT_LIMIT is less than VM_REFCNT_EXCLUDE_READERS_FLAG.
//
// Acquire fence is required here to avoid reordering against later
// vm_lock_seq check and checks inside lock_vma_under_rcu().
//
    if (unlikely(!__refcount_inc_not_zero_limited_acquire(&vma.vm_refcnt, &oldcnt,
    VM_REFCNT_LIMIT))) {
// return EAGAIN if vma got detached from under us
    vma = oldcnt ? core::ptr::null_mut() : ERR_PTR(-EAGAIN);
// goto;
    }
    __vma_lockdep_acquire_read(vma);
    if (unlikely(vma.vm_mm != mm)) {
// goto;
    }
//
// Overflow of vm_lock_seq/mm_lock_seq might produce false locked result.
// False unlocked result is impossible because we modify and check
// vma->vm_lock_seq under vma->vm_refcnt protection and mm->mm_lock_seq
// modification invalidates all existing locks.
//
// We must use ACQUIRE semantics for the mm_lock_seq so that if we are
// racing with vma_end_write_all(), we only start reading from the VMA
// after it has been unlocked.
// This pairs with RELEASE semantics in vma_end_write_all().
//
    if (unlikely(vma.vm_lock_seq == raw_read_seqcount(&mm.mm_lock_seq))) {
    vma_refcount_put(vma);
    vma = core::ptr::null_mut();
// goto;
    }
    return vma;
// label;
    rcu_read_unlock();
    return vma;
// label;
//
// If vma got attached to another mm from under us, that mm is not
// stable and can be freed in the narrow window after vma->vm_refcnt
// is dropped and before rcuwait_wake_up(mm) is called. Grab it before
// releasing vma->vm_refcnt.
//
    other_mm = vma.vm_mm; /* use a copy as vma can be freed after we drop vm_refcnt */
// __mmdrop() is a heavy operation, do it after dropping RCU lock.
    rcu_read_unlock();
    mmgrab(other_mm);
    vma_refcount_put(vma);
    mmdrop(other_mm);
    return core::ptr::null_mut();
    }
//
// Lookup and lock a VMA under RCU protection. Returned VMA is guaranteed to be
// stable and not isolated. If the VMA is not found or is being modified the
// function returns NULL.
//
#[no_mangle]
pub unsafe extern "C" fn lock_vma_under_rcu(mm: *mut mm_struct, address: c_ulong) -> *mut c_void {
    MA_STATE(mas, &mm.mm_mt, address, address);
pub static mut vma: *mut c_void = core::ptr::null_mut();
// label;
    rcu_read_lock();
    vma = mas_walk(&mas);
    if (!vma) {
    rcu_read_unlock();
// goto;
    }
    vma = vma_start_read(mm, vma);
    if (IS_ERR_OR_NULL(vma)) {
// Check if the VMA got isolated after we found it
    if (PTR_ERR(vma) == -EAGAIN) {
    count_vm_vma_lock_event(VMA_LOCK_MISS);
// The area was replaced with another one
    mas_set(&mas, address);
// goto;
    }
// Failed to lock the VMA
// goto;
    }
//
// At this point, we have a stable reference to a VMA: The VMA is
// locked and we know it hasn't already been isolated.
// From here on, we can access the VMA without worrying about which
// fields are accessible for RCU readers.
//
    rcu_read_unlock();
// Check if the vma we locked is the right one.
    if (unlikely(address < vma.vm_start || address >= vma.vm_end)) {
    vma_end_read(vma);
// goto;
    }
    return vma;
// label;
    count_vm_vma_lock_event(VMA_LOCK_ABORT);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn lock_next_vma_under_mmap_lock(mm: *mut mm_struct, vmi: *mut vma_iterator, from_addr: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = mmap_read_lock_killable(mm);
    if (ret) {
    return ERR_PTR(ret);
    }
// Lookup the vma at the last position again under mmap_read_lock
    vma_iter_set(vmi, from_addr);
    vma = vma_next(vmi);
    if (vma) {
// Very unlikely vma->vm_refcnt overflow case
    if (unlikely(!vma_start_read_locked(vma))) {
    vma = ERR_PTR(-EAGAIN);
    }
    }
    mmap_read_unlock(mm);
    return vma;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_next_vma(mm: *mut mm_struct, vmi: *mut vma_iterator, from_addr: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut mm_wr_seq = 0;
    let mut mmap_unlocked = 0;
    RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "no rcu read lock held");
// label;
// Start mmap_lock speculation in case we need to verify the vma later
    mmap_unlocked = mmap_lock_speculate_try_begin(mm, &mm_wr_seq);
    vma = vma_next(vmi);
    if (!vma) {
    return core::ptr::null_mut();
    }
    vma = vma_start_read(mm, vma);
    if (IS_ERR_OR_NULL(vma)) {
//
// Retry immediately if the vma gets detached from under us.
// Infinite loop should not happen because the vma we find will
// have to be constantly knocked out from under us.
//
    if (PTR_ERR(vma) == -EAGAIN) {
// reset to search from the last address
    rcu_read_lock();
    vma_iter_set(vmi, from_addr);
// goto;
    }
// goto;
    }
// Verify the vma is not behind the last search position.
    if (unlikely(from_addr >= vma.vm_end)) {
// goto;
    }
//
// vma can be ahead of the last search position but we need to verify
// it was not shrunk after we found it and another vma has not been
// installed ahead of it. Otherwise we might observe a gap that should
// not be there.
//
    if (from_addr < vma.vm_start) {
// Verify only if the address space might have changed since vma lookup.
    if (!mmap_unlocked || mmap_lock_speculate_retry(mm, mm_wr_seq)) {
    vma_iter_set(vmi, from_addr);
    if (vma != vma_next(vmi)) {
// goto;
    }
    }
    }
    return vma;
// label;
    rcu_read_unlock();
    vma_end_read(vma);
// label;
    vma = lock_next_vma_under_mmap_lock(mm, vmi, from_addr);
    rcu_read_lock();
// Reinitialize the iterator after re-entering rcu read section
    vma_iter_set(vmi, IS_ERR_OR_NULL(vma) ? from_addr : vma.vm_end);
    return vma;
    }

#[no_mangle]
pub unsafe extern "C" fn get_mmap_lock_carefully(mm: *mut mm_struct, regs: *mut pt_regs) -> bool {
    if (likely(mmap_read_trylock(mm))) {
    return true;
    }
    if (regs && !user_mode(regs)) {
pub static mut ip: c_ulong = 0;
    if (!search_exception_tables(ip)) {
    return false;
    }
    }
    return !mmap_read_lock_killable(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn mmap_upgrade_trylock(mm: *mut mm_struct) -> bool {
//
// We don't have this operation yet.
//
// It should be easy enough to do: it's basically a
// atomic_long_try_cmpxchg_acquire()
// from RWSEM_READER_BIAS -> RWSEM_WRITER_LOCKED, but
// it also needs the proper lockdep magic etc.
//
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn upgrade_mmap_lock_carefully(mm: *mut mm_struct, regs: *mut pt_regs) -> bool {
    mmap_read_unlock(mm);
    if (regs && !user_mode(regs)) {
pub static mut ip: c_ulong = 0;
    if (!search_exception_tables(ip)) {
    return false;
    }
    }
    return !mmap_write_lock_killable(mm);
    }
//
// Helper for page fault handling.
//
// This is kind of equivalent to "mmap_read_lock()" followed
// by "find_extend_vma()", except it's a lot more careful about
// the locking (and will drop the lock on failure).
//
// For example, if we have a kernel bug that causes a page
// fault, we don't want to just use mmap_read_lock() to get
// the mm lock, because that would deadlock if the bug were
// to happen while we're holding the mm lock for writing.
//
// So this checks the exception tables on kernel faults in
// order to only do this all for instructions that are actually
// expected to fault.
//
// We can also actually take the mm lock for writing if we
// need to extend the vma, which helps the VM layer a lot.
//
#[no_mangle]
pub unsafe extern "C" fn lock_mm_and_find_vma(mm: *mut mm_struct, addr: c_ulong, regs: *mut pt_regs) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    if (!get_mmap_lock_carefully(mm, regs)) {
    return core::ptr::null_mut();
    }
    vma = find_vma(mm, addr);
    if (likely(vma && (vma.vm_start <= addr))) {
    return vma;
    }
//
// Well, dang. We might still be successful, but only
// if we can extend a vma to do so.
//
    if (!vma || !(vma.vm_flags & VM_GROWSDOWN)) {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }
//
// We can try to upgrade the mmap lock atomically,
// in which case we can continue to use the vma
// we already looked up.
//
// Otherwise we'll have to drop the mmap lock and
// re-take it, and also look up the vma again,
// re-checking it.
//
    if (!mmap_upgrade_trylock(mm)) {
    if (!upgrade_mmap_lock_carefully(mm, regs)) {
    return core::ptr::null_mut();
    }
    vma = find_vma(mm, addr);
    if (!vma) {
// goto;
    }
    if (vma.vm_start <= addr) {
// goto;
    }
    if (!(vma.vm_flags & VM_GROWSDOWN)) {
// goto;
    }
    }
    if (expand_stack_locked(vma, addr)) {
// goto;
    }
// label;
    mmap_write_downgrade(mm);
    return vma;
// label;
    mmap_write_unlock(mm);
    return core::ptr::null_mut();
    }

//
// At least xtensa ends up having protection faults even with no
// MMU.. No stack expansion, at least.
//
#[no_mangle]
#[no_mangle]
// duplicate fn: lock_mm_and_find_vma
pub unsafe extern "C" fn lock_mm_and_find_vma_dup(mm: *mut mm_struct, addr: c_ulong, regs: *mut pt_regs) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    mmap_read_lock(mm);
    vma = vma_lookup(mm, addr);
    if (!vma) {
    mmap_read_unlock(mm);
    }
    return vma;
    }