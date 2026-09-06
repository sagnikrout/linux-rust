//! Automatically rewritten from C to Rust
//! Source: mm/page_ext.c
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
// struct page extension
//
// This is the feature to manage memory for extended data per page.
//
// Until now, we must modify struct page itself to store extra data per page.
// This requires rebuilding the kernel and it is really time consuming process.
// And, sometimes, rebuild is impossible due to third party module dependency.
// At last, enlarging struct page could cause un-wanted system behaviour change.
//
// This feature is intended to overcome above mentioned problems. This feature
// allocates memory for extended data per page in certain place rather than
// the struct page itself. This memory can be accessed by the accessor
// functions provided by this code. During the boot process, it checks whether
// allocation of huge chunk of memory is needed or not. If not, it avoids
// allocating memory at all. With this advantage, we can include this feature
// into the kernel in default and can avoid rebuild and solve related problems.
//
// To help these things to work well, there are two callbacks for clients. One
// is the need callback which is mandatory if user wants to avoid useless
// memory allocation at boot-time. The other is optional, init callback, which
// is used to do proper initialization after memory is allocated.
//
// The need callback is used to decide whether extended memory allocation is
// needed or not. Sometimes users want to deactivate some features in this
// boot and extra memory would be unnecessary. In this case, to avoid
// allocating huge chunk of memory, each clients represent their need of
// extra memory through the need callback. If one of the need callbacks
// returns true, it means that someone needs extra memory so that
// page extension core should allocates memory for page extension. If
// none of need callbacks return true, memory isn't needed at all in this boot
// and page extension core can skip to allocate memory. As result,
// none of memory is wasted.
//
// When need callback returns true, page_ext checks if there is a request for
// extra memory through size in struct page_ext_operations. If it is non-zero,
// extra space is allocated for each page_ext entry and offset is returned to
// user through offset in struct page_ext_operations.
//
// The init callback is used to do proper initialization after page extension
// is completely initialized. In sparse memory system, extra memory is
// allocated some time later than memmap is allocated. In other words, lifetime
// of memory for page extension isn't same with memmap for struct page.
// Therefore, clients can't store extra data until page extension is
// initialized, even if pages are allocated and used freely. This could
// cause inadequate state of extra data per page, so, to prevent it, client
// can utilize this callback to initialize the state of it correctly.
//

#[no_mangle]
unsafe extern "C" fn need_page_idle() -> bool {
    return true;
    }
    static struct page_ext_operations page_idle_ops __initdata = {
    .need = need_page_idle,
    .need_shared_flags = true,
    };

    static struct page_ext_operations *page_ext_ops[] __initdata = {

    &page_owner_ops,

    &page_idle_ops,

    &page_alloc_tagging_ops,

    &page_table_check_ops,

    &page_iommu_debug_ops,

    };
    let mut page_ext_size = 0;
    static unsigned long total_usage;

//
// To ensure correct allocation tagging for pages, page_ext should be available
// before the first page allocation. Otherwise early task stacks will be
// allocated before page_ext initialization and missing tags will be flagged.
//
pub static mut __meminitdata: bool early_page_ext = true;

    bool early_page_ext __meminitdata;

#[no_mangle]
unsafe extern "C" fn setup_early_page_ext(str: *mut c_char) -> c_int {
    early_page_ext = true;
    return 0;
    }
    early_param!("early_page_ext", setup_early_page_ext);
#[no_mangle]
unsafe extern "C" fn invoke_need_callbacks() -> bool __init {
    let mut i = 0;
pub static mut entries: c_int = 0;
pub static mut need: bool = false;
    while (i < entries) {
    if (page_ext_ops[i].need()) {
    if (page_ext_ops[i].need_shared_flags) {
    page_ext_size = sizeof!(page_ext);
    break;
    }
    }
    }
    while (i < entries) {
    if (page_ext_ops[i].need()) {
    page_ext_ops[i].offset = page_ext_size;
    page_ext_size += page_ext_ops[i].size;
    need = true;
    }
    }
    return need;
    }
#[no_mangle]
unsafe extern "C" fn invoke_init_callbacks()  {
    let mut i = 0;
pub static mut entries: c_int = 0;
    while (i < entries) {
    if (page_ext_ops[i].init) {
    page_ext_ops[i].init();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_entry(base: *mut c_void, index: c_ulong) -> *mut c_void {
    return base + page_ext_size * index;
    }

#[no_mangle]
pub unsafe extern "C" fn page_ext_init_flatmem_late()  {
    invoke_init_callbacks();
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_page_ext(page: *mut page) -> *mut c_void {
pub static mut pfn: c_ulong = 0;
    let mut index = 0;
pub static mut base: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(!rcu_read_lock_held());
    base = NODE_DATA(page_to_nid(page)).node_page_ext;
//
// The sanity checks the page allocator does upon freeing a
// page can reach here before the page_ext arrays are
// allocated when feeding a range of pages to the allocator
// for the first time during bootup or memory hotplug.
//
    if (unlikely(!base)) {
    return core::ptr::null_mut();
    }
    index = pfn - round_down(node_start_pfn(page_to_nid(page)),
    MAX_ORDER_NR_PAGES);
    return get_entry(base, index);
    }
#[no_mangle]
unsafe extern "C" fn alloc_node_page_ext(nid: c_int) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut table_size = 0;
    let mut nr_pages = 0;
    nr_pages = NODE_DATA(nid).node_spanned_pages;
    if (!nr_pages) {
    return 0;
    }
//
// Need extra space if node range is not aligned with
// MAX_ORDER_NR_PAGES. When page allocator's buddy algorithm
// checks buddy's status, range could be out of exact node range.
//
    if (!IS_ALIGNED(node_start_pfn(nid), MAX_ORDER_NR_PAGES) ||
    !IS_ALIGNED(node_end_pfn(nid), MAX_ORDER_NR_PAGES)) {
    nr_pages += MAX_ORDER_NR_PAGES;
    }
    table_size = page_ext_size * nr_pages;
    base = memblock_alloc_try_nid(
    table_size, PAGE_SIZE, __pa(MAX_DMA_ADDRESS),
    MEMBLOCK_ALLOC_ACCESSIBLE, nid);
    if (!base) {
    return -ENOMEM;
    }
    NODE_DATA(nid).node_page_ext = base;
    total_usage += table_size;
    memmap_boot_pages_add(DIV_ROUND_UP(table_size, PAGE_SIZE));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn page_ext_init_flatmem()  {
    let mut nid = 0;
    let mut fail = 0;
    if (!invoke_need_callbacks()) {
    return;
    }
    for_each_online_node(nid)  {
    fail = alloc_node_page_ext(nid);
    if (fail) {
// goto;
    }
    }
    pr_info!("allocated %ld bytes of page_ext\n", total_usage);
    return;
// label;
    pr_crit("allocation of page_ext failed.\n");
    panic("Out of memory");
    }

#[no_mangle]
unsafe extern "C" fn page_ext_invalid(page_ext: *mut page_ext) -> bool {
    return !page_ext || (((unsigned long)page_ext & PAGE_EXT_INVALID) == PAGE_EXT_INVALID);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: lookup_page_ext
pub unsafe extern "C" fn lookup_page_ext_dup(page: *mut page) -> *mut c_void {
pub static mut pfn: c_ulong = 0;
    let mut section = __pfn_to_section(pfn);
    let mut page_ext = READ_ONCE(section.page_ext);
    WARN_ON_ONCE!(!rcu_read_lock_held());
//
// The sanity checks the page allocator does upon freeing a
// page can reach here before the page_ext arrays are
// allocated when feeding a range of pages to the allocator
// for the first time during bootup or memory hotplug.
//
    if (page_ext_invalid(page_ext)) {
    return core::ptr::null_mut();
    }
    return get_entry(page_ext, pfn);
    }
#[no_mangle]
unsafe extern "C" fn alloc_page_ext(size: usize, nid: c_int) -> *mut c_void {
pub static mut flags: gfp_t = 0;
    let mut addr = core::ptr::null_mut();
    addr = alloc_pages_exact_nid(nid, size, flags);
    if (addr) {
    kmemleak_alloc(addr, size, 1, flags);
    }
    else {
    addr = vzalloc_node(size, nid);
    }
    if (addr) {
    memmap_pages_add(DIV_ROUND_UP(size, PAGE_SIZE));
    }
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn init_section_page_ext(pfn: c_ulong, nid: c_int) -> int __meminit {
pub static mut section: *mut c_void = core::ptr::null_mut();
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut table_size = 0;
    section = __pfn_to_section(pfn);
    if (section.page_ext) {
    return 0;
    }
    table_size = page_ext_size * PAGES_PER_SECTION;
    base = alloc_page_ext(table_size, nid);
//
// The value stored in section->page_ext is (base - pfn)
// and it does not point to the memory block allocated above,
// causing kmemleak false positives.
//
    kmemleak_not_leak(base);
    if (!base) {
    pr_err!("page ext allocation failure\n");
    return -ENOMEM;
    }
//
// The passed "pfn" may not be aligned to SECTION.  For the calculation
// we need to apply a mask.
//
    pfn &= PAGE_SECTION_MASK;
    section.page_ext = base - page_ext_size * pfn;
    total_usage += table_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_page_ext(addr: *mut c_void) {
    let mut table_size = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    table_size = page_ext_size * PAGES_PER_SECTION;
    memmap_pages_add(-1L * (DIV_ROUND_UP(table_size, PAGE_SIZE)));
    if (is_vmalloc_addr(addr)) {
    vfree(addr);
    } else {
    page = virt_to_page(addr);
    BUG_ON!(PageReserved(page));
    kmemleak_free(addr);
    free_pages_exact(addr, table_size);
    }
    }
#[no_mangle]
unsafe extern "C" fn __free_page_ext(pfn: c_ulong) {
pub static mut ms: *mut c_void = core::ptr::null_mut();
pub static mut base: *mut c_void = core::ptr::null_mut();
    ms = __pfn_to_section(pfn);
    if (!ms || !ms.page_ext) {
    return;
    }
    base = READ_ONCE(ms.page_ext);
//
// page_ext here can be valid while doing the roll back
// operation in online_page_ext().
//
    if (page_ext_invalid(base)) {
    base = base - PAGE_EXT_INVALID;
    }
    WRITE_ONCE(ms.page_ext, core::ptr::null_mut());
    base = get_entry(base, pfn);
    free_page_ext(base);
    }
#[no_mangle]
unsafe extern "C" fn __invalidate_page_ext(pfn: c_ulong) {
pub static mut ms: *mut c_void = core::ptr::null_mut();
pub static mut val: *mut c_void = core::ptr::null_mut();
    ms = __pfn_to_section(pfn);
    if (!ms || !ms.page_ext) {
    return;
    }
    val = ms.page_ext + PAGE_EXT_INVALID;
    WRITE_ONCE(ms.page_ext, val);
    }
    static int __meminit online_page_ext(unsigned long start_pfn,
    unsigned long nr_pages)
    {
pub static mut nid: c_int = 0;
    unsigned long start, end, pfn;
pub static mut fail: c_int = 0;
    start = SECTION_ALIGN_DOWN(start_pfn);
    end = SECTION_ALIGN_UP(start_pfn + nr_pages);
    for (pfn = start; !fail && pfn < end; pfn += PAGES_PER_SECTION) {
    fail = init_section_page_ext(pfn, nid);
    }
    if (!fail) {
    return 0;
    }
// rollback
    end = pfn - PAGES_PER_SECTION;
    for (pfn = start; pfn < end; pfn += PAGES_PER_SECTION) {
    __free_page_ext(pfn);
    }
    return -ENOMEM;
    }
    static void __meminit offline_page_ext(unsigned long start_pfn,
    unsigned long nr_pages)
    {
    unsigned long start, end, pfn;
    start = SECTION_ALIGN_DOWN(start_pfn);
    end = SECTION_ALIGN_UP(start_pfn + nr_pages);
//
// Freeing of page_ext is done in 3 steps to avoid
// use-after-free of it:
// 1) Traverse all the sections and mark their page_ext
// as invalid.
// 2) Wait for all the existing users of page_ext who
// started before invalidation to finish.
// 3) Free the page_ext.
//
    for (pfn = start; pfn < end; pfn += PAGES_PER_SECTION) {
    __invalidate_page_ext(pfn);
    }
    synchronize_rcu();
    for (pfn = start; pfn < end; pfn += PAGES_PER_SECTION) {
    __free_page_ext(pfn);
    }
    }
    static int __meminit page_ext_callback(notifier_block *self,
    unsigned long action, void *arg)
    {
    let mut mn = arg;
pub static mut ret: c_int = 0;
    match (action) {
    MEM_GOING_ONLINE => {
    ret = online_page_ext(mn.start_pfn, mn.nr_pages);
    // break;
    }
    MEM_OFFLINE => {
    offline_page_ext(mn.start_pfn,
    mn.nr_pages);
    // break;
    }
    MEM_CANCEL_ONLINE => {
    offline_page_ext(mn.start_pfn,
    mn.nr_pages);
    // break;
    }
    MEM_GOING_OFFLINE => {
    // break;
    }
    MEM_ONLINE => {
    }
    MEM_CANCEL_OFFLINE => {
    // break;
    }
    }
    return notifier_from_errno(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn page_ext_init()  {
    let mut pfn = 0;
    let mut nid = 0;
    if (!invoke_need_callbacks()) {
    return;
    }
    for_each_node_state(nid, N_MEMORY) {
    unsigned long start_pfn, end_pfn;
    start_pfn = node_start_pfn(nid);
    end_pfn = node_end_pfn(nid);
//
// start_pfn and end_pfn may not be aligned to SECTION and the
// page->flags of out of node pages are not initialized.  So we
// scan [start_pfn, the biggest section's pfn < end_pfn) here.
//
    for (pfn = start_pfn; pfn < end_pfn;
    pfn = ALIGN(pfn + 1, PAGES_PER_SECTION)) {
    if (!pfn_valid(pfn)) {
    continue;
    }
//
// Nodes's pfns can be overlapping.
// We know some arch can have a nodes layout such as
// -------------pfn-------------->
// N0 | N1 | N2 | N0 | N1 | N2|....
//
    if (pfn_to_nid(pfn) != nid) {
    continue;
    }
    if (init_section_page_ext(pfn, nid)) {
// goto;
    }
    cond_resched();
    }
    }
    hotplug_memory_notifier(page_ext_callback, DEFAULT_CALLBACK_PRI);
    pr_info!("allocated %ld bytes of page_ext\n", total_usage);
    invoke_init_callbacks();
    return;
// label;
    panic("Out of memory");
    }

//
// page_ext_lookup() - Lookup a page extension for a PFN.
// @pfn: PFN of the page we're interested in.
//
// Must be called with RCU read lock taken and @pfn must be valid.
//
// Return: NULL if no page_ext exists for this page.
//
#[no_mangle]
pub unsafe extern "C" fn page_ext_lookup(pfn: c_ulong) -> *mut c_void {
    return lookup_page_ext(pfn_to_page(pfn));
    }
//
// page_ext_get() - Get the extended information for a page.
// @page: The page we're interested in.
//
// Ensures that the page_ext will remain valid until page_ext_put()
// is called.
//
// Return: NULL if no page_ext exists for this page.
// Context: Any context.  Caller may not sleep until they have called
// page_ext_put().
//
#[no_mangle]
pub unsafe extern "C" fn page_ext_get(page: *mut page) -> *mut c_void {
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    page_ext = lookup_page_ext(page);
    if (!page_ext) {
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    return page_ext;
    }
//
// page_ext_from_phys() - Get the page_ext structure for a physical address.
// @phys: The physical address to query.
//
// This function safely gets the `struct page_ext` associated with a given
// physical address. It performs validation to ensure the address corresponds
// to a valid, online struct page before attempting to access it.
// It returns NULL for MMIO, ZONE_DEVICE, holes and offline memory.
//
// Return: NULL if no page_ext exists for this physical address.
// Context: Any context.  Caller may not sleep until they have called
// page_ext_put().
//
#[no_mangle]
pub unsafe extern "C" fn page_ext_from_phys(phys: phys_addr_t) -> *mut c_void {
    let mut page = pfn_to_online_page(__phys_to_pfn(phys));
    if (!page) {
    return core::ptr::null_mut();
    }
    return page_ext_get(page);
    }
//
// page_ext_put() - Working with page extended information is done.
// @page_ext: Page extended information received from page_ext_get().
//
// The page extended information of the page may not be valid after this
// function is called.
//
// Return: None.
// Context: Any context with corresponding page_ext_get() is called.
//
#[no_mangle]
pub unsafe extern "C" fn page_ext_put(page_ext: *mut page_ext) {
    if (unlikely(!page_ext)) {
    return;
    }
    rcu_read_unlock();
    }