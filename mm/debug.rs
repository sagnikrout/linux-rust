//! Automatically rewritten from C to Rust
//! Source: mm/debug.c
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
// mm/debug.c
//
// mm/ specific debug routines.
//

//
// Define EM() and EMe() so that MIGRATE_REASON from trace/events/migrate.h can
// be used to populate migrate_reason_names[].
//

    const char *migrate_reason_names[MR_TYPES] = {
    MIGRATE_REASON
    };
pub static mut trace_print_flags: usize = 0;
pub static mut trace_print_flags: usize = 0;
pub static mut trace_print_flags: usize = 0;

    static const char *page_type_names[] = {
    DEF_PAGETYPE_NAME(slab),
    DEF_PAGETYPE_NAME(hugetlb),
    DEF_PAGETYPE_NAME(offline),
    DEF_PAGETYPE_NAME(guard),
    DEF_PAGETYPE_NAME(table),
    DEF_PAGETYPE_NAME(buddy),
    DEF_PAGETYPE_NAME(unaccepted),
    };
    static const char *page_type_name(unsigned int page_type)
    {
pub static mut i: unsigned = 0;
    if (i >= ARRAY_SIZE!(page_type_names)) {
    return "unknown";
    }
    return page_type_names[i];
    }
#[no_mangle]
pub unsafe extern "C" fn __dump_folio(folio: *mut folio, page: *mut page, pfn: c_ulong, idx: c_ulong) {
    let mut mapping = folio_mapping(folio);
pub static mut mapcount: c_int = 0;
    let mut type = "";
    if (page_mapcount_is_type(mapcount)) {
    mapcount = 0;
    }
    pr_warn!("page: refcount:%d mapcount:%d mapping:%p index:%#lx pfn:%#lx\n",
    folio_ref_count(folio), mapcount, mapping,
    folio.index + idx, pfn);
    if (folio_test_large(folio)) {
pub static mut pincount: c_int = 0;
    if (folio_has_pincount(folio)) {
    pincount = atomic_read(&folio._pincount);
    }
    pr_warn!("head: order:%u mapcount:%d entire_mapcount:%d nr_pages_mapped:%d pincount:%d\n",
    folio_order(folio),
    folio_mapcount(folio),
    folio_entire_mapcount(folio),
    folio_nr_pages_mapped(folio),
    pincount);
    }

    if (folio.memcg_data) {
    pr_warn!("memcg:%lx\n", folio.memcg_data);
    }

    if (folio_test_ksm(folio)) {
    type = "ksm ";
    }

    else if (folio_test_anon(folio)) {
    type = "anon ";
    }

    else if (mapping) {
    dump_mapping(mapping);
    }
    BUILD_BUG_ON!(ARRAY_SIZE!(pageflag_names) != __NR_PAGEFLAGS + 1);
//
// Accessing the pageblock without the zone lock. It could change to
// "isolate" again in the meantime, but since we are just dumping the
// state for debugging, it should be fine to accept a bit of
// inaccuracy here due to racing.
//
    pr_warn!("%sflags: %pGp%s\n", type, &folio.flags,
    is_migrate_cma_folio(folio, pfn) ? " CMA" : "");
    if (page_has_type(&folio.page)) {
    pr_warn!("page_type: %x(%s)\n", folio.page.page_type >> 24,
    page_type_name(folio.page.page_type));
    }
    print_hex_dump(KERN_WARNING, "raw: ", DUMP_PREFIX_NONE, 32,
    sizeof!(unsigned long), page,
    sizeof!(page), false);
    if (folio_test_large(folio)) {
    print_hex_dump(KERN_WARNING, "head: ", DUMP_PREFIX_NONE, 32,
    sizeof!(unsigned long), folio,
    2 * sizeof!(page), false);
    }
    }
#[no_mangle]
unsafe extern "C" fn __dump_page(page: *const page) {
pub static mut ps: usize = 0;
    snapshot_page(&ps, page);
    if (!snapshot_page_is_faithful(&ps)) {
    pr_warn!("page does not match folio\n");
    }
    __dump_folio(&ps.folio_snapshot, &ps.page_snapshot, ps.pfn, ps.idx);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_page(page: *const page, reason: *const c_char) {
    if (PagePoisoned(page)) {
    pr_warn!("page:%p is uninitialized and poisoned\n", page);
    }
    else {
    __dump_page(page);
    }
    if (reason) {
    pr_warn!("page dumped because: %s\n", reason);
    }
    dump_page_owner(page);
    }
    EXPORT_SYMBOL(dump_page);

#[no_mangle]
pub unsafe extern "C" fn dump_vma(vma: *const vm_area_struct) {
    pr_emerg("vma %px start %px end %px mm %px\n"
    "prot %lx anon_vma %px vm_ops %px\n"
    "pgoff %lx file %px private_data %px\n"

    "refcnt %x\n"

    "flags: %#lx(%pGv)\n",
    vma, vma.vm_start, vma.vm_end, vma.vm_mm,
    (unsigned long)pgprot_val(vma.vm_page_prot),
    vma.anon_vma, vma.vm_ops, vma_start_pgoff(vma),
    vma.vm_file, vma.vm_private_data,

    refcount_read(&vma.vm_refcnt),

    vma.vm_flags, &vma.vm_flags);
    }
    EXPORT_SYMBOL(dump_vma);
#[no_mangle]
pub unsafe extern "C" fn dump_mm(mm: *const mm_struct) {
    pr_emerg("mm %px task_size %lu\n"
    "mmap_base %lu mmap_legacy_base %lu\n"
    "pgd %px mm_users %d mm_count %d pgtables_bytes %lu map_count %d\n"
    "hiwater_rss %lx hiwater_vm %lx total_vm %lx locked_vm %lx\n"
    "pinned_vm %llx data_vm %lx exec_vm %lx stack_vm %lx\n"
    "start_code %lx end_code %lx start_data %lx end_data %lx\n"
    "start_brk %lx brk %lx start_stack %lx\n"
    "arg_start %lx arg_end %lx env_start %lx env_end %lx\n"
    "binfmt %px flags %*pb\n"

    "ioctx_table %px\n"

    "owner %px "

    "exe_file %px\n"

    "notifier_subscriptions %px\n"

    "numa_next_scan %lu numa_scan_offset %lu numa_scan_seq %d\n"

    "tlb_flush_pending %d\n"
    "def_flags: %*pb(%pGv)\n",
    mm, mm.task_size,
    mm.mmap_base, mm.mmap_legacy_base,
    mm.pgd, atomic_read(&mm.mm_users),
    atomic_read(&mm.mm_count),
    mm_pgtables_bytes(mm),
    mm.map_count,
    mm.hiwater_rss, mm.hiwater_vm, mm.total_vm, mm.locked_vm,
    (u64)atomic64_read(&mm.pinned_vm),
    mm.data_vm, mm.exec_vm, mm.stack_vm,
    mm.start_code, mm.end_code, mm.start_data, mm.end_data,
    mm.start_brk, mm.brk, mm.start_stack,
    mm.arg_start, mm.arg_end, mm.env_start, mm.env_end,
    mm.binfmt, NUM_MM_FLAG_BITS, __mm_flags_get_bitmap(mm),

    mm.ioctx_table,

    mm.owner,

    mm.exe_file,

    mm.notifier_subscriptions,

    mm.numa_next_scan, mm.numa_scan_offset, mm.numa_scan_seq,

    atomic_read(&mm.tlb_flush_pending),
    NUM_VMA_FLAG_BITS, mm.def_vma_flags.__vma_flags,
    &mm.def_vma_flags
    );
    }
    EXPORT_SYMBOL(dump_mm);
#[no_mangle]
pub unsafe extern "C" fn dump_vmg(vmg: *const vma_merge_struct, reason: *const c_char) {
    if (reason) {
    pr_warn!("vmg %px dumped because: %s\n", vmg, reason);
    }
    if (!vmg) {
    pr_warn!("vmg %px state: (core::ptr::null_mut())\n", vmg);
    return;
    }
    pr_warn!("vmg %px state: mm %px pgoff %lx\n"
    "vmi %px [%lx,%lx)\n"
    "prev %px middle %px next %px target %px\n"
    "start %lx end %lx flags %lx\n"
    "file %px anon_vma %px policy %px\n"
    "uffd_ctx %px\n"
    "anon_name %px\n"
    "state %x\n"
    "just_expand %d\n"
    "__adjust_middle_start %d __adjust_next_start %d\n"
    "__remove_middle %d __remove_next %d\n",
    vmg, vmg.mm, vmg.pgoff,
    vmg.vmi, vmg.vmi ? vma_iter_addr(vmg.vmi) : 0,
    vmg.vmi ? vma_iter_end(vmg.vmi) : 0,
    vmg.prev, vmg.middle, vmg.next, vmg.target,
    vmg.start, vmg.end, vmg.vm_flags,
    vmg.file, vmg.anon_vma, vmg.policy,

    vmg.uffd_ctx.ctx,

    0,

    vmg.anon_name,
    (int)vmg.state,
    vmg.just_expand,
    vmg.__adjust_middle_start, vmg.__adjust_next_start,
    vmg.__remove_middle, vmg.__remove_next);
    if (vmg.mm) {
    pr_warn!("vmg %px mm:\n", vmg);
    dump_mm(vmg.mm);
    } else {
    pr_warn!("vmg %px mm: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.prev) {
    pr_warn!("vmg %px prev:\n", vmg);
    dump_vma(vmg.prev);
    } else {
    pr_warn!("vmg %px prev: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.middle) {
    pr_warn!("vmg %px middle:\n", vmg);
    dump_vma(vmg.middle);
    } else {
    pr_warn!("vmg %px middle: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.next) {
    pr_warn!("vmg %px next:\n", vmg);
    dump_vma(vmg.next);
    } else {
    pr_warn!("vmg %px next: (core::ptr::null_mut())\n", vmg);
    }

    if (vmg.vmi) {
    pr_warn!("vmg %px vmi:\n", vmg);
    vma_iter_dump_tree(vmg.vmi);
    } else {
    pr_warn!("vmg %px vmi: (core::ptr::null_mut())\n", vmg);
    }

    }
    EXPORT_SYMBOL(dump_vmg);
pub static mut : bool page_init_poisoning = true;
#[no_mangle]
unsafe extern "C" fn setup_vm_debug(str: *mut c_char) -> c_int {
pub static mut __page_init_poisoning: bool = true;
//
// Calling vm_debug with no arguments is equivalent to requesting
// to enable all debugging options we can control.
//
    if (*str++ != '=' || !*str) {
// goto;
    }
    __page_init_poisoning = false;
    if (*str == '-') {
// goto;
    }
    while (*str) {
    switch (tolower(*str)) {
    case 'p':
    __page_init_poisoning = true;
    break;
// label;
    pr_err!("vm_debug option '%c' unknown. skipped\n",
// str);
    }
    str += 1;
    }
// label;
    if (page_init_poisoning && !__page_init_poisoning) {
    pr_warn!("Page struct poisoning disabled by kernel command line option 'vm_debug'\n");
    }
    page_init_poisoning = __page_init_poisoning;
    return 1;
    }
    __setup!("vm_debug", setup_vm_debug);
#[no_mangle]
pub unsafe extern "C" fn page_init_poison(page: *mut page, size: usize) {
    if (page_init_poisoning) {
    memset(page, PAGE_POISON_PATTERN, size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vma_iter_dump_tree(vmi: *const vma_iterator) {

    mas_dump(&vmi.mas);
    mt_dump(vmi.mas.tree, mt_dump_hex);

    }