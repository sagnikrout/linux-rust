//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/arena.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

//
// bpf_arena is a sparsely populated shared memory region between bpf program and
// user space process.
//
// For example on x86-64 the values could be:
// user_vm_start 7f7d26200000     // picked by mmap()
// kern_vm_start ffffc90001e69000 // picked by get_vm_area()
// For user space all pointers within the arena are normal 8-byte addresses.
// In this example 7f7d26200000 is the address of the first page (pgoff=0).
// The bpf program will access it as: kern_vm_start + lower_32bit_of_user_ptr
// (u32)7f7d26200000 -> 26200000
// hence
// ffffc90001e69000 + 26200000 == ffffc90028069000 is "pgoff=0" within 4Gb
// kernel memory region.
//
// BPF JITs generate the following code to access arena:
// mov eax, eax  // eax has lower 32-bit of user pointer
// mov word ptr [rax + r12 + off], bx
// where r12 == kern_vm_start and off is s16.
// Hence allocate 4Gb + GUARD_SZ/2 on each side.
//
// Initially kernel vm_area and user vma are not populated.
// User space can fault-in any address which will insert the page
// into kernel and user vma.
// bpf program can allocate a page via bpf_arena_alloc_pages() kfunc
// which will insert it into kernel vm_area.
// The later fault-in from user space will populate that page into user vma.
//
// number of bytes addressable by LDX/STX insn with 16-bit 'off' field

// forward_decl: arena_free_pages;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_arena {
    pub map: bpf_map,
    pub user_vm_start: u64,
    pub user_vm_end: u64,
    pub kern_vm: *mut vm_struct,
    pub scratch_page: *mut page,
    pub rt: range_tree,
// protects rt and nr_pages
    pub spinlock: rqspinlock_t,
// number of pages currently populated in the arena
    pub nr_pages: u64,
    pub vma_list: list_head,
// protects vma_list
    pub lock: mutex,
    pub zap_gen: u64,
    pub zap_mutex: mutex,
    pub free_irq: irq_work,
    pub free_work: work_struct,
    pub free_spans: llist_head,
}

// forward_decl: arena_free_worker;
// forward_decl: arena_free_irq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_free_span {
    pub node: llist_node,
    pub uaddr: c_ulong,
    pub page_cnt: u32,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_arena_get_kern_vm_start(arena: *mut bpf_arena) -> u64 {
    return arena ? (u64) (long) arena.kern_vm.addr + GUARD_SZ / 2 : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_get_user_vm_start(arena: *mut bpf_arena) -> u64 {
    return arena ? arena.user_vm_start : 0;
    }
//
// bpf_arena_map_kern_vm_start - kern_vm_start lookup by struct bpf_map
// @map: a BPF_MAP_TYPE_ARENA map
//
// Return @map's kern_vm_start.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_map_kern_vm_start(map: *mut bpf_map) -> u64 {
    return bpf_arena_get_kern_vm_start(container_of!(map, bpf_arena, map));
    }
//
// bpf_prog_arena - return the bpf_map of the arena referenced by @prog
// @prog: a loaded BPF program
//
// The verifier enforces at most one arena per program and stores it in
// prog->aux->arena. Return that arena's underlying bpf_map, or NULL if
// @prog does not reference an arena.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_arena(prog: *mut bpf_prog) -> *mut c_void {
    let mut arena = prog.aux.arena;
    return arena ? &arena.map : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn arena_map_peek_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_push_elem(map: *mut bpf_map, value: *mut c_void, flags: u64) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_pop_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_delete_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn compute_pgoff(arena: *mut bpf_arena, uaddr: c_long) -> c_long {
    return (u32)(uaddr - (u32)arena.user_vm_start) >> PAGE_SHIFT;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apply_range_data {
    pub arena: *mut bpf_arena,
    pub pages: *mut page,
    pub i: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clear_range_data {
    pub arena: *mut bpf_arena,
    pub free_pages: *mut llist_head,
}

#[no_mangle]
unsafe extern "C" fn apply_range_set_cb(pte: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    let mut d = data;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pteval;
    if (!data) {
    return 0;
    }
    page = d.pages[d.i];
// paranoia, similar to vmap_pages_pte_range()
    if (WARN_ON_ONCE!(!pfn_valid(page_to_pfn(page)))) {
    return -EINVAL;
    }
    pteval = mk_pte(page, PAGE_KERNEL);

//
// Kernel-fault recovery may have installed the scratch page here, and
// some architectures (arm64) prohibit valid->valid PTE transitions.
// Install atomically into a none slot. If scratch is present, clear it
// and flush_tlb_before_set() (break-before-make) before retrying.
//
    while (!ptep_try_set(pte, pteval)) {
pub static mut old: pte_t = 0;
    if (pte_none(old)) {
    continue;
    }
    if (WARN_ON_ONCE!(pte_page(old) != d.arena.scratch_page)) {
    return -EBUSY;
    }
    ptep_get_and_clear(&init_mm, addr, pte);
    flush_tlb_before_set(addr);
    }

//
// Without ptep_try_set() there is no atomic installer, but such arches
// also do not wire up bpf_arena_handle_page_fault(), so no scratch page
// is ever installed and the slot is always none here.
//
    if (unlikely(!pte_none(ptep_get(pte)))) {
    return -EBUSY;
    }
    set_pte_at(&init_mm, addr, pte, pteval);

    d.i += 1;
    WRITE_ONCE(d.arena.nr_pages, d.arena.nr_pages + 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flush_vmap_cache(start: c_ulong, size: c_ulong) {
    flush_cache_vmap(start, start + size);
    }
#[no_mangle]
unsafe extern "C" fn apply_range_clear_cb(pte: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    let mut d = data;
    let mut old_pte;
pub static mut page: *mut c_void = core::ptr::null_mut();
//
// Pairs with ptep_try_set() in the kernel-fault scratch installer.
// Both sides must be atomic.
//
    old_pte = ptep_get_and_clear(&init_mm, addr, pte);
    if (pte_none(old_pte) || !pte_present(old_pte)) {
    return 0;
    }
    page = pte_page(old_pte);
    if (WARN_ON_ONCE!(!page)) {
    return -EINVAL;
    }
//
// Skip the per-arena scratch page. A kernel fault on an unallocated uaddr
// scratches its PTE. A later bpf_arena_free_pages() over that range walks
// here. Without the skip, scratch_page would be freed.
//
    if (page == d.arena.scratch_page) {
    return 0;
    }
    __llist_add(&page.pcp_llist, d.free_pages);
    WRITE_ONCE(d.arena.nr_pages, d.arena.nr_pages - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apply_range_set_scratch_cb(pte: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    let mut scratch_page = data;
    if (!pte_none(ptep_get(pte))) {
    return 0;
    }
//
// Best-effort install. ptep_try_set() returns false only if another
// installer (real allocation or concurrent fault) won the cmpxchg.
// Their PTE is already valid, so the access retry succeeds.
//
// No flush_tlb_kernel_range() needed. Stale "not mapped" entries just
// cause one extra re-fault through this same path.
//
    ptep_try_set(pte, mk_pte(scratch_page, PAGE_KERNEL));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn populate_pgtable_except_pte(arena: *mut bpf_arena) -> c_int {
// Populate intermediates for the recovery range (4 GiB + upper half-guard).
    return apply_to_page_range(&init_mm, bpf_arena_get_kern_vm_start(arena),
    SZ_4G + GUARD_SZ / 2, apply_range_set_cb, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn arena_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
pub static mut kern_vm: *mut c_void = core::ptr::null_mut();
pub static mut numa_node: c_int = 0;
pub static mut arena: *mut c_void = core::ptr::null_mut();
    let mut vm_range = 0;
pub static mut err: c_int = 0;
    if (!bpf_jit_supports_arena()) {
    return ERR_PTR(-EOPNOTSUPP);
    }
    if (attr.key_size || attr.value_size || attr.max_entries == 0 ||
// BPF_F_MMAPABLE must be set
    !(attr.map_flags & BPF_F_MMAPABLE) ||
// No unsupported flags present
    (attr.map_flags & ~(BPF_F_SEGV_ON_FAULT | BPF_F_MMAPABLE | BPF_F_NO_USER_CONV))) {
    return ERR_PTR(-EINVAL);
    }
    if (attr.map_extra & ~PAGE_MASK) {
// If non-zero the map_extra is an expected user VMA start address
    return ERR_PTR(-EINVAL);
    }
    vm_range = (u64)attr.max_entries * PAGE_SIZE;
    if (vm_range > SZ_4G) {
    return ERR_PTR(-E2BIG);
    }
    if ((attr.map_extra >> 32) != ((attr.map_extra + vm_range - 1) >> 32)) {
// user vma must not cross 32-bit boundary
    return ERR_PTR(-ERANGE);
    }
    kern_vm = get_vm_area(KERN_VM_SZ, VM_SPARSE | VM_USERMAP);
    if (!kern_vm) {
    return ERR_PTR(-ENOMEM);
    }
    arena = bpf_map_area_alloc(sizeof!(*arena), numa_node);
    if (!arena) {
// goto;
    }
    arena.kern_vm = kern_vm;
    arena.user_vm_start = attr.map_extra;
    if (arena.user_vm_start) {
    arena.user_vm_end = arena.user_vm_start + vm_range;
    }
    INIT_LIST_HEAD(&arena.vma_list);
    init_llist_head(&arena.free_spans);
    init_irq_work(&arena.free_irq, arena_free_irq);
    INIT_WORK(&arena.free_work, arena_free_worker);
    bpf_map_init_from_attr(&arena.map, attr);
    err = bpf_map_alloc_pages(&arena.map, NUMA_NO_NODE, 1, &arena.scratch_page);
    if (err) {
// goto;
    }
    range_tree_init(&arena.rt);
    err = range_tree_set(&arena.rt, 0, attr.max_entries);
    if (err) {
// goto;
    }
    mutex_init(&arena.lock);
    mutex_init(&arena.zap_mutex);
    raw_res_spin_lock_init(&arena.spinlock);
    err = populate_pgtable_except_pte(arena);
    if (err) {
// goto;
    }
    return &arena.map;
// label;
    range_tree_destroy(&arena.rt);
// label;
    __free_page(arena.scratch_page);
// label;
    bpf_map_area_free(arena);
// label;
    free_vm_area(kern_vm);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn existing_page_cb(ptep: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    let mut arena = data;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pte;
    pte = ptep_get(ptep);
    if (!pte_present(pte)) /* sanity check */ {
    return 0;
    }
    page = pte_page(pte);
//
// Skip the scratch page. The walk is page-table-driven, not range-tree-driven,
// so it can visit scratch PTEs at uaddrs the BPF program never allocated.
//
    if (page == arena.scratch_page) {
    return 0;
    }
//
// We do not update pte here:
// 1. Nobody should be accessing bpf_arena's range outside of a kernel bug
// 2. TLB flushing is batched or deferred. Even if we clear pte,
// the TLB entries can stick around and continue to permit access to
// the freed page. So it all relies on 1.
//
    __free_page(page);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_free(map: *mut bpf_map) {
    let mut arena = container_of!(map, bpf_arena, map);
//
// Check that user vma-s are not around when bpf map is freed.
// mmap() holds vm_file which holds bpf_map refcnt.
// munmap() must have happened on vma followed by arena_vm_close()
// which would clear arena->vma_list.
//
    if (WARN_ON_ONCE!(!list_empty(&arena.vma_list))) {
    return;
    }
// Ensure no pending deferred frees
    irq_work_sync(&arena.free_irq);
    flush_work(&arena.free_work);
//
// free_vm_area() calls remove_vm_area() that calls free_unmap_vmap_area().
// It unmaps everything from vmalloc area and clears pgtables.
// Call apply_to_existing_page_range() first to find populated ptes and
// free those pages.
//
    apply_to_existing_page_range(&init_mm, bpf_arena_get_kern_vm_start(arena),
    SZ_4G + GUARD_SZ / 2, existing_page_cb, arena);
    free_vm_area(arena.kern_vm);
    range_tree_destroy(&arena.rt);
    __free_page(arena.scratch_page);
    bpf_map_area_free(arena);
    }
#[no_mangle]
pub unsafe extern "C" fn arena_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn arena_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> c_long {
    return -EOPNOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn arena_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut arena = container_of!(map, bpf_arena, map);
    return (u64)READ_ONCE(arena.nr_pages) << PAGE_SHIFT;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_list {
    pub vma: *mut vm_area_struct,
    pub head: list_head,
    pub mmap_count: refcount_t,
    pub zap_gen: u64,
}

#[no_mangle]
unsafe extern "C" fn remember_vma(arena: *mut bpf_arena, vma: *mut vm_area_struct) -> c_int {
pub static mut vml: *mut c_void = core::ptr::null_mut();
    vml = kmalloc_obj(*vml);
    if (!vml) {
    return -ENOMEM;
    }
    refcount_set(&vml.mmap_count, 1);
    vma.vm_private_data = vml;
    vml.vma = vma;
    vml.zap_gen = 0;
    list_add(&vml.head, &arena.vma_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arena_vm_open(vma: *mut vm_area_struct) {
    let mut vml = vma.vm_private_data;
    refcount_inc(&vml.mmap_count);
    }
#[no_mangle]
unsafe extern "C" fn arena_vm_may_split(vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn arena_vm_mremap(vma: *mut vm_area_struct) -> c_int {
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn arena_vm_close(vma: *mut vm_area_struct) {
    let mut map = vma.vm_file.private_data;
    let mut arena = container_of!(map, bpf_arena, map);
    let mut vml = vma.vm_private_data;
    if (!refcount_dec_and_test(&vml.mmap_count)) {
    return;
    }
    guard(mutex)(&arena.lock);
// update link list under lock
    list_del(&vml.head);
    vma.vm_private_data = core::ptr::null_mut();
    kfree(vml);
    }
#[no_mangle]
unsafe extern "C" fn arena_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut map = vmf.vma.vm_file.private_data;
    let mut arena = container_of!(map, bpf_arena, map);
    let mut new_memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut kbase = 0;
    let mut kaddr = 0;
    let mut flags = 0;
    let mut ret = 0;
    kbase = bpf_arena_get_kern_vm_start(arena);
    kaddr = kbase + (u32)(vmf.address);
    if (raw_res_spin_lock_irqsave(&arena.spinlock, flags)) {
//
// A failed lock means a possible deadlock was detected. Don't
// return VM_FAULT_RETRY: this handler never took mmap_lock, but
// the fault path would re-take it on retry and deadlock. Fail.
//
    return VM_FAULT_SIGBUS;
    }
    page = vmalloc_to_page(kaddr);
    if (page) {
    if (page == arena.scratch_page) {
// BPF triggered scratch here; don't lazy-alloc over it
    }
// goto;
// already have a page vmap-ed
// goto;
    }
    bpf_map_memcg_enter(&arena.map, &old_memcg, &new_memcg);
    if (arena.map.map_flags & BPF_F_SEGV_ON_FAULT) {
// User space requested to segfault when page is not allocated by bpf prog
// goto;
    }
    ret = range_tree_clear(&arena.rt, vmf.pgoff, 1);
    if (ret) {
// goto;
    }
pub static mut data: apply_range_data = 0;
// Account into memcg of the process that created bpf_arena
    ret = bpf_map_alloc_pages(map, NUMA_NO_NODE, 1, &page);
    if (ret) {
    range_tree_set(&arena.rt, vmf.pgoff, 1);
// goto;
    }
    ret = apply_to_page_range(&init_mm, kaddr, PAGE_SIZE, apply_range_set_cb, &data);
    if (ret) {
    range_tree_set(&arena.rt, vmf.pgoff, 1);
    free_pages_nolock(page, 0);
// goto;
    }
    flush_vmap_cache(kaddr, PAGE_SIZE);
    bpf_map_memcg_exit(old_memcg, new_memcg);
// label;
    page_ref_add(page, 1);
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
    vmf.page = page;
    return 0;
// label;
    bpf_map_memcg_exit(old_memcg, new_memcg);
// label;
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
    return VM_FAULT_SIGSEGV;
    }
pub static mut vm_operations_struct: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn arena_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong) -> c_ulong {
    let mut map = filp.private_data;
    let mut arena = container_of!(map, bpf_arena, map);
    let mut ret = 0;
    if (pgoff) {
    return -EINVAL;
    }
    if (len > SZ_4G) {
    return -E2BIG;
    }
// if user_vm_start was specified at arena creation time
    if (arena.user_vm_start) {
    if (len > arena.user_vm_end - arena.user_vm_start) {
    return -E2BIG;
    }
    if (len != arena.user_vm_end - arena.user_vm_start) {
    return -EINVAL;
    }
    if (addr != arena.user_vm_start) {
    return -EINVAL;
    }
    }
    ret = mm_get_unmapped_area(filp, addr, len * 2, 0, flags);
    if (IS_ERR_VALUE(ret)) {
    return ret;
    }
    if ((ret >> 32) == ((ret + len - 1) >> 32)) {
    return ret;
    }
    if (WARN_ON_ONCE!(arena.user_vm_start)) {
// checks at map creation time should prevent this
    return -EFAULT;
    }
    return round_up(ret, SZ_4G);
    }
#[no_mangle]
unsafe extern "C" fn arena_map_mmap(map: *mut bpf_map, vma: *mut vm_area_struct) -> c_int {
    let mut arena = container_of!(map, bpf_arena, map);
    guard(mutex)(&arena.lock);
    if (arena.user_vm_start && arena.user_vm_start != vma.vm_start) {
//
// If map_extra was not specified at arena creation time then
// 1st user process can do mmap(NULL, ...) to pick user_vm_start
// 2nd user process must pass the same addr to mmap(addr, MAP_FIXED..);
    }
// or
// specify addr in map_extra and
// use the same addr later with mmap(addr, MAP_FIXED..);
//
    return -EBUSY;
    if (arena.user_vm_end && arena.user_vm_end != vma.vm_end) {
// all user processes must have the same size of mmap-ed region
    return -EBUSY;
    }
// Earlier checks should prevent this
    if (WARN_ON_ONCE!(vma.vm_end - vma.vm_start > SZ_4G || vma.vm_pgoff)) {
    return -EFAULT;
    }
    if (remember_vma(arena, vma)) {
    return -ENOMEM;
    }
    arena.user_vm_start = vma.vm_start;
    arena.user_vm_end = vma.vm_end;
//
// bpf_map_mmap() checks that it's being mmaped as VM_SHARED and
// clears VM_MAYEXEC. Set VM_DONTEXPAND to avoid potential change
// of user_vm_start. Set VM_DONTCOPY to prevent arena VMA from
// being copied into the child process on fork.
//
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTCOPY);
    vma.vm_ops = &arena_vm_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arena_map_direct_value_addr(map: *const bpf_map, imm: *mut u64, off: u32) -> c_int {
    let mut arena = container_of!(map, bpf_arena, map);
    if ((u64)off >= arena.user_vm_end - arena.user_vm_start) {
    return -ERANGE;
    }
// imm = (unsigned long)arena->user_vm_start;
    return 0;
    }
    BTF_ID_LIST_SINGLE(bpf_arena_map_btf_ids, struct, bpf_arena)
pub static mut bpf_map_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn clear_lo32(val: u64) -> u64 {
    return val & ~(u64)~0U;
    }
//
// Allocate pages and vmap them into kernel vmalloc area.
// Later the pages will be mmaped into user space vma.
//
#[no_mangle]
pub unsafe extern "C" fn arena_alloc_pages(arena: *mut bpf_arena, uaddr: c_long, page_cnt: c_long, node_id: c_int, sleepable: bool) -> c_long {
// user_vm_end/start are fixed before bpf prog runs
pub static mut page_cnt_max: c_long = 0;
pub static mut kern_vm_start: u64 = 0;
    let mut new_memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
pub static mut data: usize = 0;
    let mut pages = core::ptr::null_mut();
    long remaining, mapped = 0;
    let mut alloc_pages = 0;
    let mut flags = 0;
pub static mut pgoff: c_long = 0;
    let mut uaddr32 = 0;
    let mut ret = 0;
    let mut i = 0;
    if (node_id != NUMA_NO_NODE &&
    ((unsigned int)node_id >= nr_node_ids || !node_online(node_id))) {
    return 0;
    }
    if (page_cnt > page_cnt_max) {
    return 0;
    }
    if (uaddr) {
    if (uaddr & ~PAGE_MASK) {
    return 0;
    }
    pgoff = compute_pgoff(arena, uaddr);
    if (pgoff > page_cnt_max - page_cnt) {
// requested address will be outside of user VMA
    return 0;
    }
    }
    bpf_map_memcg_enter(&arena.map, &old_memcg, &new_memcg);
// Cap allocation size to KMALLOC_MAX_CACHE_SIZE so kmalloc_nolock() can succeed.
    alloc_pages = min(page_cnt, KMALLOC_MAX_CACHE_SIZE / sizeof!);
    pages = kmalloc_nolock(alloc_pages * sizeof!, __GFP_ACCOUNT, NUMA_NO_NODE);
    if (!pages) {
    bpf_map_memcg_exit(old_memcg, new_memcg);
    return 0;
    }
    data.arena = arena;
    data.pages = pages;
    if (raw_res_spin_lock_irqsave(&arena.spinlock, flags)) {
// goto;
    }
    if (uaddr) {
    ret = is_range_tree_set(&arena.rt, pgoff, page_cnt);
    if (ret) {
// goto;
    }
    ret = range_tree_clear(&arena.rt, pgoff, page_cnt);
    } else {
    ret = pgoff = range_tree_find(&arena.rt, page_cnt);
    if (pgoff >= 0) {
    ret = range_tree_clear(&arena.rt, pgoff, page_cnt);
    }
    }
    if (ret) {
// goto;
    }
    remaining = page_cnt;
    uaddr32 = (u32)(arena.user_vm_start + pgoff * PAGE_SIZE);
    while (remaining) {
pub static mut this_batch: c_long = 0;
// zeroing is needed, since alloc_pages_bulk() only fills in non-zero entries
    memset(pages, 0, this_batch * sizeof!);
    ret = bpf_map_alloc_pages(&arena.map, node_id, this_batch, pages);
    if (ret) {
// goto;
    }
//
// Earlier checks made sure that uaddr32 + page_cnt * PAGE_SIZE - 1
// will not overflow 32-bit. Lower 32-bit need to represent
// contiguous user address range.
// Map these pages at kern_vm_start base.
// kern_vm_start + uaddr32 + page_cnt * PAGE_SIZE - 1 can overflow
// lower 32-bit and it's ok.
//
    data.i = 0;
    ret = apply_to_page_range(&init_mm,
    kern_vm_start + uaddr32 + (mapped << PAGE_SHIFT),
    this_batch << PAGE_SHIFT, apply_range_set_cb, &data);
    if (ret) {
// data.i pages were mapped, account them and free the remaining
    mapped += data.i;
    for (i = data.i; i < this_batch; i++) {
    free_pages_nolock(pages[i], 0);
    }
// goto;
    }
    mapped += this_batch;
    remaining -= this_batch;
    }
    flush_vmap_cache(kern_vm_start + uaddr32, mapped << PAGE_SHIFT);
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
    kfree_nolock(pages);
    bpf_map_memcg_exit(old_memcg, new_memcg);
    return clear_lo32(arena.user_vm_start) + uaddr32;
// label;
    range_tree_set(&arena.rt, pgoff + mapped, page_cnt - mapped);
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
    if (mapped) {
    flush_vmap_cache(kern_vm_start + uaddr32, mapped << PAGE_SHIFT);
    arena_free_pages(arena, uaddr32, mapped, sleepable);
    }
// goto;
// label;
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
// label;
    kfree_nolock(pages);
    bpf_map_memcg_exit(old_memcg, new_memcg);
    return 0;
    }
//
// If page is present in vmalloc area, unmap it from vmalloc area,
// unmap it from all user space vma-s,
// and free it.
//
#[no_mangle]
unsafe extern "C" fn zap_pages(arena: *mut bpf_arena, uaddr: c_long, page_cnt: c_long) {
pub static mut size: c_ulong = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut vml: *mut c_void = core::ptr::null_mut();
    let mut vm_start = 0;
    let mut my_gen = 0;
//
// Taking mmap_read_lock() under arena->lock would deadlock against
// arena_vm_close(), which runs with mmap_write_lock held and then
// acquires arena->lock. Drop arena->lock for mmap_read_lock().
//
// Use per-call my_gen, recorded in vml->zap_gen, to remember which
// vmls this invocation has already processed across the lock drop.
// Hold zap_mutex around the whole walk so concurrent zap_pages()
// callers cannot overwrite each other's marks on shared vmls --
// otherwise call B's mark would make call A skip a vml that A has
// not yet zapped for A's uaddr range.
//
    mutex_lock(&arena.zap_mutex);
    mutex_lock(&arena.lock);
    my_gen = ++arena.zap_gen;
    for (;;) {
    mm = core::ptr::null_mut();
    list_for_each_entry(vml, &arena.vma_list, head) {
    if (vml.zap_gen >= my_gen) {
    continue;
    }
    vml.zap_gen = my_gen;
    if (!mmget_not_zero(vml.vma.vm_mm)) {
    continue;
    }
    mm = vml.vma.vm_mm;
    vm_start = vml.vma.vm_start;
    break;
    }
    if (!mm) {
    break;
    }
    mutex_unlock(&arena.lock);
    mmap_read_lock(mm);
//
// Re-resolve: while we waited the VMA could have been unmapped
// and a different mapping installed at the same address.
//
    vma = find_vma(mm, vm_start);
    if (vma && vma.vm_start == vm_start &&
    vma.vm_file && vma.vm_file.private_data == &arena.map) {
    zap_vma_range(vma, uaddr, size);
    }
    mmap_read_unlock(mm);
    mmput(mm);
    mutex_lock(&arena.lock);
    }
    mutex_unlock(&arena.lock);
    mutex_unlock(&arena.zap_mutex);
    }
#[no_mangle]
unsafe extern "C" fn arena_free_pages(arena: *mut bpf_arena, uaddr: c_long, page_cnt: c_long, sleepable: bool) {
    let mut new_memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
    u64 full_uaddr, uaddr_end;
    let mut kaddr = 0;
    let mut pgoff = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut free_pages: usize = 0;
    let mut pos = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut cdata: usize = 0;
    let mut flags = 0;
pub static mut ret: c_int = 0;
// only aligned lower 32-bit are relevant
    uaddr = (u32)uaddr;
    uaddr &= PAGE_MASK;
    kaddr = bpf_arena_get_kern_vm_start(arena) + uaddr;
    full_uaddr = clear_lo32(arena.user_vm_start) + uaddr;
    if (full_uaddr < arena.user_vm_start) {
    return;
    }
    uaddr_end = min(arena.user_vm_end, full_uaddr + (page_cnt << PAGE_SHIFT));
    if (full_uaddr >= uaddr_end) {
    return;
    }
    page_cnt = (uaddr_end - full_uaddr) >> PAGE_SHIFT;
    pgoff = compute_pgoff(arena, uaddr);
    bpf_map_memcg_enter(&arena.map, &old_memcg, &new_memcg);
    if (!sleepable) {
// goto;
    }
    ret = raw_res_spin_lock_irqsave(&arena.spinlock, flags);
// Can't proceed without holding the spinlock so defer the free
    if (ret) {
// goto;
    }
    range_tree_set(&arena.rt, pgoff, page_cnt);
    init_llist_head(&free_pages);
    cdata.arena = arena;
    cdata.free_pages = &free_pages;
// clear ptes and collect struct pages
    apply_to_existing_page_range(&init_mm, kaddr, page_cnt << PAGE_SHIFT,
    apply_range_clear_cb, &cdata);
// drop the lock to do the tlb flush and zap pages
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
// ensure no stale TLB entries
    flush_tlb_kernel_range(kaddr, kaddr + (page_cnt * PAGE_SIZE));
    if (page_cnt > 1) {
// bulk zap if multiple pages being freed
    zap_pages(arena, full_uaddr, page_cnt);
    }
    llist_for_each_safe(pos, t, __llist_del_all(&free_pages)) {
    page = llist_entry(pos, page, pcp_llist);
    if (page_cnt == 1 && page_ref_count(page) > 1) /* maybe mapped by user space */ {
// Optimization for the common case of page_cnt==1:
// If page wasn't mapped into some user vma there
// is no need to call zap_pages which is slow. When
// page_cnt is big it's faster to do the batched zap.
//
    zap_pages(arena, full_uaddr, 1);
    }
    __free_page(page);
    }
    bpf_map_memcg_exit(old_memcg, new_memcg);
    return;
// label;
    s = kmalloc_nolock(sizeof!(arena_free_span), __GFP_ACCOUNT, -1);
    bpf_map_memcg_exit(old_memcg, new_memcg);
    if (!s) {
//
// If allocation fails in non-sleepable context, pages are intentionally left
// inaccessible (leaked) until the arena is destroyed. Cleanup or retries are not
// possible here, so we intentionally omit them for safety.
//
    return;
    }
    s.page_cnt = page_cnt;
    s.uaddr = uaddr;
    llist_add(&s.node, &arena.free_spans);
    irq_work_queue(&arena.free_irq);
    }
//
// Reserve an arena virtual address range without populating it. This call stops
// bpf_arena_alloc_pages from adding pages to this range.
//
#[no_mangle]
unsafe extern "C" fn arena_reserve_pages(arena: *mut bpf_arena, uaddr: c_long, page_cnt: u32) -> c_int {
pub static mut page_cnt_max: c_long = 0;
    let mut new_memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
    let mut flags = 0;
    let mut pgoff = 0;
    let mut ret = 0;
    if (uaddr & ~PAGE_MASK) {
    return 0;
    }
    pgoff = compute_pgoff(arena, uaddr);
    if (pgoff + page_cnt > page_cnt_max) {
    return -EINVAL;
    }
    if (raw_res_spin_lock_irqsave(&arena.spinlock, flags)) {
    return -EBUSY;
    }
// Cannot guard already allocated pages.
    ret = is_range_tree_set(&arena.rt, pgoff, page_cnt);
    if (ret) {
    ret = -EBUSY;
// goto;
    }
// "Allocate" the region to prevent it from being allocated.
    bpf_map_memcg_enter(&arena.map, &old_memcg, &new_memcg);
    ret = range_tree_clear(&arena.rt, pgoff, page_cnt);
    bpf_map_memcg_exit(old_memcg, new_memcg);
// label;
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arena_free_worker(work: *mut work_struct) {
    let mut arena = container_of!(work, bpf_arena, free_work);
    let mut new_memcg = core::ptr::null_mut();
    let mut old_memcg = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    u64 arena_vm_start, user_vm_start;
pub static mut free_pages: usize = 0;
pub static mut cdata: usize = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut full_uaddr = 0;
    let mut kaddr = 0;
    let mut page_cnt = 0;
    let mut pgoff = 0;
    let mut flags = 0;
    if (raw_res_spin_lock_irqsave(&arena.spinlock, flags)) {
    schedule_work(work);
    return;
    }
    bpf_map_memcg_enter(&arena.map, &old_memcg, &new_memcg);
    init_llist_head(&free_pages);
    cdata.arena = arena;
    cdata.free_pages = &free_pages;
    arena_vm_start = bpf_arena_get_kern_vm_start(arena);
    user_vm_start = bpf_arena_get_user_vm_start(arena);
    list = llist_del_all(&arena.free_spans);
    llist_for_each(pos, list) {
    s = llist_entry(pos, arena_free_span, node);
    page_cnt = s.page_cnt;
    kaddr = arena_vm_start + s.uaddr;
    pgoff = compute_pgoff(arena, s.uaddr);
// clear ptes and collect pages in free_pages llist
    apply_to_existing_page_range(&init_mm, kaddr, page_cnt << PAGE_SHIFT,
    apply_range_clear_cb, &cdata);
    range_tree_set(&arena.rt, pgoff, page_cnt);
    }
    raw_res_spin_unlock_irqrestore(&arena.spinlock, flags);
// Iterate the list again without holding spinlock to do the tlb flush and zap_pages
    llist_for_each_safe(pos, t, list) {
    s = llist_entry(pos, arena_free_span, node);
    page_cnt = s.page_cnt;
    full_uaddr = clear_lo32(user_vm_start) + s.uaddr;
    kaddr = arena_vm_start + s.uaddr;
// ensure no stale TLB entries
    flush_tlb_kernel_range(kaddr, kaddr + (page_cnt * PAGE_SIZE));
// remove pages from user vmas
    zap_pages(arena, full_uaddr, page_cnt);
    kfree_nolock(s);
    }
// free all pages collected by apply_to_existing_page_range() in the first loop
    llist_for_each_safe(pos, t, __llist_del_all(&free_pages)) {
    page = llist_entry(pos, page, pcp_llist);
    __free_page(page);
    }
    bpf_map_memcg_exit(old_memcg, new_memcg);
    }
#[no_mangle]
unsafe extern "C" fn arena_free_irq(iw: *mut irq_work) {
    let mut arena = container_of!(iw, bpf_arena, free_irq);
    schedule_work(&arena.free_work);
    }
    __bpf_kfunc_start_defs();
    __bpf_kfunc void *bpf_arena_alloc_pages(void *p__map, void *addr__ign, u32 page_cnt,
    int node_id, u64 flags)
    {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA || flags || !page_cnt) {
    return core::ptr::null_mut();
    }
    return arena_alloc_pages(arena, (long)addr__ign, page_cnt, node_id, true);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_alloc_pages_non_sleepable(p__map: *mut c_void, addr__ign: *mut c_void, page_cnt: u32, node_id: c_int, flags: u64) -> *mut c_void {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA || flags || !page_cnt) {
    return core::ptr::null_mut();
    }
    return arena_alloc_pages(arena, (long)addr__ign, page_cnt, node_id, false);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_alloc_pages_sleepable(p__map: *mut c_void, addr__ign: *mut c_void, page_cnt: u32, node_id: c_int, flags: u64) -> *mut c_void {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA || flags || !page_cnt) {
    return core::ptr::null_mut();
    }
    return arena_alloc_pages(arena, (long)addr__ign, page_cnt, node_id, true);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_free_pages(p__map: *mut c_void, ptr__ign: *mut c_void, page_cnt: u32) -> __bpf_kfunc void {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA || !page_cnt || !ptr__ign) {
    return;
    }
    arena_free_pages(arena, (long)ptr__ign, page_cnt, true);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_free_pages_non_sleepable(p__map: *mut c_void, ptr__ign: *mut c_void, page_cnt: u32) {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA || !page_cnt || !ptr__ign) {
    return;
    }
    arena_free_pages(arena, (long)ptr__ign, page_cnt, false);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_reserve_pages(p__map: *mut c_void, ptr__ign: *mut c_void, page_cnt: u32) -> __bpf_kfunc int {
    let mut map = p__map;
    let mut arena = container_of!(map, bpf_arena, map);
    if (map.map_type != BPF_MAP_TYPE_ARENA) {
    return -EINVAL;
    }
    if (!page_cnt) {
    return 0;
    }
    return arena_reserve_pages(arena, (long)ptr__ign, page_cnt);
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(arena_kfuncs)
    BTF_ID_FLAGS(func, bpf_arena_alloc_pages, KF_ARENA_RET | KF_ARENA_ARG2 | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_arena_free_pages, KF_ARENA_ARG2 | KF_SPINLOCK_SAFE)
    BTF_ID_FLAGS(func, bpf_arena_reserve_pages, KF_ARENA_ARG2 | KF_SPINLOCK_SAFE)
    BTF_KFUNCS_END(arena_kfuncs)
pub static mut btf_kfunc_id_set: usize = 0;
#[no_mangle]
unsafe extern "C" fn kfunc_init() -> c_int {
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC, &common_kfunc_set);
    }
    late_initcall!(kfunc_init);
#[no_mangle]
pub unsafe extern "C" fn __bpf_prog_report_arena_violation(prog: *mut bpf_prog, write: bool, addr: c_ulong, fault_ip: c_ulong) {
pub static mut ss: usize = 0;
    let mut user_vm_start = 0;
// Use main prog for stream access
    prog = prog.aux.main_prog_aux.prog;
    user_vm_start = bpf_arena_get_user_vm_start(prog.aux.arena);
    addr += clear_lo32(user_vm_start);
    bpf_stream_stage(ss, prog, BPF_STDERR, ({
    bpf_stream_printk(ss, "ERROR: Arena %s access at unmapped address 0x%lx\n",
    write ? "WRITE" : "READ", addr);
    bpf_stream_dump_stack(ss);
    }));
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_arena_handle_page_fault(addr: c_ulong, is_write: bool, fault_ip: c_ulong) -> bool {
pub static mut arena: *mut c_void = core::ptr::null_mut();
pub static mut prog: *mut c_void = core::ptr::null_mut();
    let mut kbase = 0;
pub static mut page_addr: c_ulong = 0;
    prog = bpf_prog_find_from_stack();
    if (!prog) {
    return false;
    }
    arena = prog.aux.arena;
// a prog not using arena may be on stack, so arena can be NULL
    if (!arena) {
    return false;
    }
    kbase = bpf_arena_get_kern_vm_start(arena);
//
// Recovery covers the 4 GiB mappable band plus the upper half-guard.
// Lower guard is unreachable from kfuncs; an address there indicates
// a different bug class - leave it to the regular kernel oops path.
//
    if (page_addr < kbase || page_addr >= kbase + SZ_4G + GUARD_SZ / 2) {
    return false;
    }
    apply_to_page_range(&init_mm, page_addr, PAGE_SIZE,
    apply_range_set_scratch_cb, arena.scratch_page);
    flush_vmap_cache(page_addr, PAGE_SIZE);
    __bpf_prog_report_arena_violation(prog, is_write, page_addr - kbase, fault_ip);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_report_arena_violation(write: bool, addr: c_ulong, fault_ip: c_ulong) {
pub static mut prog: *mut c_void = core::ptr::null_mut();
//
// The RCU read lock is held to safely traverse the latch tree, but we
// don't need its protection when accessing the prog, since it will not
// disappear while we are handling the fault.
//
    rcu_read_lock();
    prog = bpf_prog_ksym_find(fault_ip);
    rcu_read_unlock();
    if (!prog) {
    return;
    }
    __bpf_prog_report_arena_violation(prog, write, addr, fault_ip);
    }