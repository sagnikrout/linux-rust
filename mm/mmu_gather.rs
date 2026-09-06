//! Automatically rewritten from C to Rust
//! Source: mm/mmu_gather.c
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

#[no_mangle]
unsafe extern "C" fn tlb_next_batch(tlb: *mut mmu_gather) -> bool {
pub static mut batch: *mut c_void = core::ptr::null_mut();
// Limit batching if we have delayed rmaps pending
    if (tlb.delayed_rmap && tlb.active != &tlb.local) {
    return false;
    }
    batch = tlb.active;
    if (batch.next) {
    tlb.active = batch.next;
    return true;
    }
    if (tlb.batch_count == MAX_GATHER_BATCH_COUNT) {
    return false;
    }
    batch = __get_free_page(GFP_NOWAIT);
    if (!batch) {
    return false;
    }
    tlb.batch_count += 1;
    batch.next = core::ptr::null_mut();
    batch.nr   = 0;
    batch.max  = MAX_GATHER_BATCH;
    tlb.active.next = batch;
    tlb.active = batch;
    return true;
    }

#[no_mangle]
unsafe extern "C" fn tlb_flush_rmap_batch(batch: *mut mmu_gather_batch, vma: *mut vm_area_struct) {
    let mut pages = batch.encoded_pages;
    while (i < batch.nr) {
    let mut enc = pages[i];
    if (encoded_page_flags(enc) & ENCODED_PAGE_BIT_DELAY_RMAP) {
    let mut page = encoded_page_ptr(enc);
pub static mut nr_pages: c_uint = 1;
    if (unlikely(encoded_page_flags(enc) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT)) {
    nr_pages = encoded_nr_pages(pages[++i]);
    }
    folio_remove_rmap_ptes(page_folio(page), page, nr_pages,
    vma);
    }
    }
    }
//
// tlb_flush_rmaps - do pending rmap removals after we have flushed the TLB
// @tlb: the current mmu_gather
// @vma: The memory area from which the pages are being removed.
//
// Note that because of how tlb_next_batch() above works, we will
// never start multiple new batches with pending delayed rmaps, so
// we only need to walk through the current active batch and the
// original local one.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_flush_rmaps(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    if (!tlb.delayed_rmap) {
    return;
    }
    tlb_flush_rmap_batch(&tlb.local, vma);
    if (tlb.active != &tlb.local) {
    tlb_flush_rmap_batch(tlb.active, vma);
    }
    tlb.delayed_rmap = 0;
    }

//
// We might end up freeing a lot of pages. Reschedule on a regular
// basis to avoid soft lockups in configurations without full
// preemption enabled. The magic number of 512 folios seems to work.
//
pub const MAX_NR_FOLIOS_PER_FREE: c_int = 512;
#[no_mangle]
unsafe extern "C" fn __tlb_batch_free_encoded_pages(batch: *mut mmu_gather_batch) {
    let mut pages = batch.encoded_pages;
    let mut nr = 0;
    let mut nr_pages = 0;
    while (batch.nr) {
    if (!page_poisoning_enabled_static() && !want_init_on_free()) {
    nr = min(MAX_NR_FOLIOS_PER_FREE, batch.nr);
//
// Make sure we cover page + nr_pages, and don't leave
// nr_pages behind when capping the number of entries.
//
    if (unlikely(encoded_page_flags(pages[nr - 1]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT)) {
    nr += 1;
    }
    } else {
//
// With page poisoning and init_on_free, the time it
// takes to free memory grows proportionally with the
// actual memory size. Therefore, limit based on the
// actual memory size and not the number of involved
// folios.
//
    while (nr < batch.nr && nr_pages < MAX_NR_FOLIOS_PER_FREE) {
    if (unlikely(encoded_page_flags(pages[nr]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT)) {
    nr_pages += encoded_nr_pages(pages[++nr]);
    }
    else {
    nr_pages += 1;
    }
    }
    }
    free_pages_and_swap_cache(pages, nr);
    pages += nr;
    batch.nr -= nr;
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn tlb_batch_pages_flush(tlb: *mut mmu_gather) {
pub static mut batch: *mut c_void = core::ptr::null_mut();
    for (batch = &tlb.local; batch && batch.nr; batch = batch.next) {
    __tlb_batch_free_encoded_pages(batch);
    }
    tlb.active = &tlb.local;
    }
#[no_mangle]
unsafe extern "C" fn tlb_batch_list_free(tlb: *mut mmu_gather) {
    let mut batch = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    while (batch) {
    next = batch.next;
    free_pages((unsigned long)batch, 0);
    }
    tlb.local.next = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_folio_pages_size(tlb: *mut mmu_gather, page: *mut page, nr_pages: c_uint, delay_rmap: bool, page_size: c_int) -> bool {
pub static mut flags: c_int = 0;
pub static mut batch: *mut c_void = core::ptr::null_mut();
    VM_BUG_ON(!tlb.end);

    VM_WARN_ON(tlb.page_size != page_size);
    VM_WARN_ON_ONCE(nr_pages != 1 && page_size != PAGE_SIZE);
    VM_WARN_ON_ONCE(page_folio(page) != page_folio(page + nr_pages - 1));

    batch = tlb.active;
//
// Add the page and check if we are full. If so
// force a flush.
//
    if (likely(nr_pages == 1)) {
    batch.encoded_pages[batch.nr++] = encode_page(page, flags);
    } else {
    flags |= ENCODED_PAGE_BIT_NR_PAGES_NEXT;
    batch.encoded_pages[batch.nr++] = encode_page(page, flags);
    batch.encoded_pages[batch.nr++] = encode_nr_pages(nr_pages);
    }
//
// Make sure that we can always add another "page" + "nr_pages",
// requiring two entries instead of only a single one.
//
    if (batch.nr >= batch.max - 1) {
    if (!tlb_next_batch(tlb)) {
    return true;
    }
    batch = tlb.active;
    }
    VM_BUG_ON_PAGE(batch.nr > batch.max - 1, page);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_folio_pages(tlb: *mut mmu_gather, page: *mut page, nr_pages: c_uint, delay_rmap: bool) -> bool {
    return __tlb_remove_folio_pages_size(tlb, page, nr_pages, delay_rmap,
    PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: c_int) -> bool {
    return __tlb_remove_folio_pages_size(tlb, page, 1, false, page_size);
    }

#[no_mangle]
unsafe extern "C" fn __tlb_remove_table_free(batch: *mut mmu_table_batch) {
    let mut i = 0;
    for (i = 0; i < batch.nr; i++) {
    __tlb_remove_table(batch.tables[i]);
    }
    free_page((unsigned long)batch);
    }

//
// Semi RCU freeing of the page directories.
//
// This is needed by some architectures to implement software pagetable walkers.
//
// gup_fast() and other software pagetable walkers do a lockless page-table
// walk and therefore needs some synchronization with the freeing of the page
// directories. The chosen means to accomplish that is by disabling IRQs over
// the walk.
//
// Architectures that use IPIs to flush TLBs will then automagically DTRT,
// since we unlink the page, flush TLBs, free the page. Since the disabling of
// IRQs delays the completion of the TLB flush we can never observe an already
// freed page.
//
// Not all systems IPI every CPU for this purpose:
//
// - Some architectures have HW support for cross-CPU synchronisation of TLB
// flushes, so there's no IPI at all.
//
// - Paravirt guests can do this TLB flushing in the hypervisor, or coordinate
// with the hypervisor to defer flushing on preempted vCPUs.
//
// Such systems need to delay the freeing by some other means, this is that
// means.
//
// What we do is batch the freed directory pages (tables) and RCU free them.
// We use the sched RCU variant, as that guarantees that IRQ/preempt disabling
// holds off grace periods.
//
// However, in order to batch these pages we need to allocate storage, this
// allocation is deep inside the MM code and can thus easily fail on memory
// pressure. To guarantee progress we fall back to single table freeing, see
// the implementation of tlb_remove_table_one().
//
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_smp_sync(arg: *mut c_void) {
// Simply deliver the interrupt
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table_sync_one() {
//
// This isn't an RCU grace period and hence the page-tables cannot be
// assumed to be actually RCU-freed.
//
// It is however sufficient for software page-table walkers that rely on
// IRQ disabling.
//
    smp_call_function(tlb_remove_table_smp_sync, core::ptr::null_mut(), 1);
    }
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_rcu(head: *mut rcu_head) {
    __tlb_remove_table_free(container_of!(head, mmu_table_batch, rcu));
    }
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_free(batch: *mut mmu_table_batch) {
    call_rcu(&batch.rcu, tlb_remove_table_rcu);
    }
//
// tlb_remove_table_sync_rcu - synchronize with software page-table walkers
//
// Like tlb_remove_table_sync_one() but uses RCU grace period instead of IPI
// broadcast. Use in slow paths where sleeping is acceptable.
//
// Software/Lockless page-table walkers use local_irq_disable(), which is also
// an RCU read-side critical section. synchronize_rcu() waits for all such
// sections, providing the same guarantee as tlb_remove_table_sync_one() but
// without disrupting all CPUs with IPIs.
//
// Do not use for freeing memory. Use RCU callbacks instead to avoid latency
// spikes.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table_sync_rcu() {
    synchronize_rcu();
    }

#[no_mangle]
unsafe extern "C" fn tlb_remove_table_free(batch: *mut mmu_table_batch) {
    __tlb_remove_table_free(batch);
    }

//
// If we want tlb_remove_table() to imply TLB invalidates.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_table_invalidate(tlb: *mut mmu_gather) {
    if (tlb_needs_table_invalidate()) {
//
// Invalidate page-table caches used by hardware walkers. Then
// we still need to RCU-sched wait while freeing the pages
// because software walkers can still be in-flight.
//
    tlb_flush_mmu_tlbonly(tlb);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_table_one_rcu(head: *mut rcu_head) {
pub static mut ptdesc: *mut c_void = core::ptr::null_mut();
    ptdesc = container_of!(head, ptdesc, pt_rcu_head);
    __tlb_remove_table(ptdesc);
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_table_one(table: *mut c_void) {
pub static mut ptdesc: *mut c_void = core::ptr::null_mut();
    ptdesc = table;
    call_rcu(&ptdesc.pt_rcu_head, __tlb_remove_table_one_rcu);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __tlb_remove_table_one
pub unsafe extern "C" fn __tlb_remove_table_one_dup(table: *mut c_void) {
    tlb_remove_table_sync_rcu();
    __tlb_remove_table(table);
    }

#[no_mangle]
unsafe extern "C" fn tlb_remove_table_one(table: *mut c_void) {
    __tlb_remove_table_one(table);
    }
#[no_mangle]
unsafe extern "C" fn tlb_table_flush(tlb: *mut mmu_gather) {
    let mut batch = &tlb.batch;
    if (*batch) {
    tlb_table_invalidate(tlb);
    tlb_remove_table_free(*batch);
// batch = NULL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut c_void) {
    let mut batch = &tlb.batch;
    if (*batch == core::ptr::null_mut()) {
// batch = __get_free_page(GFP_NOWAIT);
    if (*batch == core::ptr::null_mut()) {
    tlb_table_invalidate(tlb);
    tlb_remove_table_one(table);
    return;
    }
    (*batch).nr = 0;
    }
    (*batch).tables[(*batch).nr++] = table;
    if ((*batch).nr == MAX_TABLE_BATCH) {
    tlb_table_flush(tlb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_table_init(tlb: *mut mmu_gather) {
    tlb.batch = core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn tlb_table_flush(tlb: *mut mmu_gather) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: tlb_table_init
pub unsafe extern "C" fn tlb_table_init_dup(tlb: *mut mmu_gather) { }

#[no_mangle]
unsafe extern "C" fn tlb_flush_mmu_free(tlb: *mut mmu_gather) {
    tlb_table_flush(tlb);

    tlb_batch_pages_flush(tlb);

    }
#[no_mangle]
pub unsafe extern "C" fn tlb_flush_mmu(tlb: *mut mmu_gather) {
    tlb_flush_mmu_tlbonly(tlb);
    tlb_flush_mmu_free(tlb);
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct, fullmm: bool) {
    tlb.mm = mm;
    tlb.fullmm = fullmm;

    tlb.need_flush_all = 0;
    tlb.local.next = core::ptr::null_mut();
    tlb.local.nr   = 0;
    tlb.local.max  = ARRAY_SIZE!(tlb.__pages);
    tlb.active     = &tlb.local;
    tlb.batch_count = 0;

    tlb.delayed_rmap = 0;
    tlb_table_init(tlb);

    tlb.page_size = 0;

    tlb.vma_pfn = 0;
    tlb.fully_unshared_tables = 0;
    __tlb_reset_range(tlb);
    inc_tlb_flush_pending(tlb.mm);
    }
//
// tlb_gather_mmu - initialize an mmu_gather structure for page-table tear-down
// @tlb: the mmu_gather structure to initialize
// @mm: the mm_struct of the target address space
//
// Called to initialize an (on-stack) mmu_gather structure for page-table
// tear-down from @mm.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct) {
    __tlb_gather_mmu(tlb, mm, false);
    }
//
// tlb_gather_mmu_fullmm - initialize an mmu_gather structure for page-table tear-down
// @tlb: the mmu_gather structure to initialize
// @mm: the mm_struct of the target address space
//
// In this case, @mm is without users and we're going to destroy the
// full address space (exit/execve).
//
// Called to initialize an (on-stack) mmu_gather structure for page-table
// tear-down from @mm.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu_fullmm(tlb: *mut mmu_gather, mm: *mut mm_struct) {
    __tlb_gather_mmu(tlb, mm, true);
    }
//
// tlb_gather_mmu_vma - initialize an mmu_gather structure for operating on a
// single VMA
// @tlb: the mmu_gather structure to initialize
// @vma: the vm_area_struct
//
// Called to initialize an (on-stack) mmu_gather structure for operating on
// a single VMA. In contrast to tlb_gather_mmu(), calling this function will
// not require another call to tlb_start_vma(). In contrast to tlb_start_vma(),
// this function will *not* call flush_cache_range().
//
// For hugetlb VMAs, this function will also initialize the mmu_gather
// page_size accordingly, not requiring a separate call to
// tlb_change_page_size().
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu_vma(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    tlb_gather_mmu(tlb, vma.vm_mm);
    tlb_update_vma_flags(tlb, vma);
    if (is_vm_hugetlb_page(vma)) {
// All entries have the same size.
    tlb_change_page_size(tlb, huge_page_size(hstate_vma(vma)));
    }
    }
//
// tlb_finish_mmu - finish an mmu_gather structure
// @tlb: the mmu_gather structure to finish
//
// Called at the end of the shootdown operation to free up any resources that
// were required.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_finish_mmu(tlb: *mut mmu_gather) {
//
// We expect an earlier huge_pmd_unshare_flush() call to sort this out,
// due to complicated locking requirements with page table unsharing.
//
    VM_WARN_ON_ONCE(tlb.fully_unshared_tables);
//
// If there are parallel threads are doing PTE changes on same range
// under non-exclusive lock (e.g., mmap_lock read-side) but defer TLB
// flush by batching, one thread may end up seeing inconsistent PTEs
// and result in having stale TLB entries.  So flush TLB forcefully
// if we detect parallel PTE batching threads.
//
// However, some syscalls, e.g. munmap(), may free page tables, this
// needs force flush everything in the given range. Otherwise this
// may result in having stale TLB entries for some architectures,
// e.g. aarch64, that could specify flush what level TLB.
//
    if (mm_tlb_flush_nested(tlb.mm)) {
//
// The aarch64 yields better performance with fullmm by
// avoiding multiple CPUs spamming TLBI messages at the
// same time.
//
// On x86 non-fullmm doesn't yield significant difference
// against fullmm.
//
    tlb.fullmm = 1;
    __tlb_reset_range(tlb);
    tlb.freed_tables = 1;
    }
    tlb_flush_mmu(tlb);

    tlb_batch_list_free(tlb);

    dec_tlb_flush_pending(tlb.mm);
    }