//! Automatically rewritten from C to Rust
//! Source: fs/proc/page.c
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


// SPDX-License-Identifier: GPL-2.0

    enum kpage_operation {
    KPAGE_FLAGS,
    KPAGE_COUNT,
    KPAGE_CGROUP,
    };
#[no_mangle]
pub unsafe extern "C" fn get_max_dump_pfn() -> c_ulong {
    static inline unsigned long get_max_dump_pfn(void)
    {

//
// The memmap of early sections is completely populated and marked
// online even if max_pfn does not fall on a section boundary -
// pfn_to_online_page() will succeed on all pages. Allow inspecting
// these memmaps.
//
    return round_up(max_pfn, PAGES_PER_SECTION);

    return max_pfn;

    }
#[no_mangle]
unsafe extern "C" fn get_kpage_count(page: *const page) -> u64 {
    static u64 get_kpage_count(const struct page *page)
    {
    struct page_snapshot ps;
    u64 ret;
    snapshot_page(&ps, page);
    if (IS_ENABLED(CONFIG_PAGE_MAPCOUNT))
    ret = folio_precise_page_mapcount(&ps.folio_snapshot,
    &ps.page_snapshot);
    else
    ret = folio_average_page_mapcount(&ps.folio_snapshot);
    return ret;
    }
    static ssize_t kpage_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos,
    enum kpage_operation op)
    {
    let mut max_dump_pfn: c_ulong = get_max_dump_pfn();
    u64 __user *out = (u64 __user *)buf;
    struct page *page;
    let mut src: c_ulong = *ppos;
    unsigned long pfn;
    let mut ret: isize = 0;
    u64 info;
    pfn = src / KPMSIZE;
    if (src & KPMMASK || count & KPMMASK)
    return -EINVAL;
    if (src >= max_dump_pfn * KPMSIZE)
    return 0;
    count = min_t(unsigned long, count, (max_dump_pfn * KPMSIZE) - src);
    while (count > 0) {
//
// TODO: ZONE_DEVICE support requires to identify
// memmaps that were actually initialized.
//
    page = pfn_to_online_page(pfn);
    if (page) {
    switch (op) {
    case KPAGE_FLAGS:
    info = stable_page_flags(page);
    break;
    case KPAGE_COUNT:
    info = get_kpage_count(page);
    break;
    case KPAGE_CGROUP:
    info = page_cgroup_ino(page);
    break;
    default:
    info = 0;
    break;
    }
    } else
    info = 0;
    if (put_user(info, out)) {
    ret = -EFAULT;
    break;
    }
    pfn++;
    out++;
    count -= KPMSIZE;
    cond_resched();
    }
// ppos += (char __user *)out - buf;
    if (!ret)
    ret = (char __user *)out - buf;
    return ret;
    }
// /proc/kpagecount - an array exposing page mapcounts
//
// Each entry is a u64 representing the corresponding
// physical page mapcount.
//
    static ssize_t kpagecount_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    return kpage_read(file, buf, count, ppos, KPAGE_COUNT);
    }
    static const struct proc_ops kpagecount_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_lseek	= mem_lseek,
    .proc_read	= kpagecount_read,
    };
#[no_mangle]
pub unsafe extern "C" fn kpf_copy_bit(kflags: u64, ubit: c_int, kbit: c_int) -> u64 {
    static inline u64 kpf_copy_bit(u64 kflags, int ubit, int kbit)
    {
    return ((kflags >> kbit) & 1) << ubit;
    }
#[no_mangle]
pub unsafe extern "C" fn stable_page_flags(page: *const page) -> u64 {
    u64 stable_page_flags(const struct page *page)
    {
    const struct folio *folio;
    struct page_snapshot ps;
    unsigned long k;
    let mut u: u64 = 0;
//
// pseudo flag: KPF_NOPAGE
// it differentiates a memory hole from a page with no flags
//
    if (!page)
    return BIT_ULL(KPF_NOPAGE);
    snapshot_page(&ps, page);
    folio = &ps.folio_snapshot;
    k = folio.flags.f;
//
// pseudo flags for the well known (anonymous) memory mapped pages
//
    if (folio_mapped(folio))
    u |= BIT_ULL(KPF_MMAP);
    if (folio_test_anon(folio)) {
    u |= BIT_ULL(KPF_ANON);
    if (folio_test_ksm(folio))
    u |= BIT_ULL(KPF_KSM);
    }
//
// compound pages: export both head/tail info
// they together define a compound page's start/end pos and order
//
    if (ps.idx == 0)
    u |= kpf_copy_bit(k, KPF_COMPOUND_HEAD, PG_head);
    else
    u |= BIT_ULL(KPF_COMPOUND_TAIL);
    if (folio_test_hugetlb(folio))
    u |= BIT_ULL(KPF_HUGE);
    else if (folio_test_large(folio) &&
    folio_test_large_rmappable(folio)) {
// Note: we indicate any THPs here, not just PMD-sized ones
    u |= BIT_ULL(KPF_THP);
    } else if (is_huge_zero_pfn(ps.pfn)) {
    u |= BIT_ULL(KPF_ZERO_PAGE);
    u |= BIT_ULL(KPF_THP);
    } else if (is_zero_pfn(ps.pfn)) {
    u |= BIT_ULL(KPF_ZERO_PAGE);
    }
    if (ps.flags & PAGE_SNAPSHOT_PG_BUDDY)
    u |= BIT_ULL(KPF_BUDDY);
    if (ps.flags & PAGE_SNAPSHOT_PG_IDLE)
    u |= BIT_ULL(KPF_IDLE);
    if (folio_test_offline(folio))
    u |= BIT_ULL(KPF_OFFLINE);
    if (folio_test_pgtable(folio))
    u |= BIT_ULL(KPF_PGTABLE);
    if (folio_test_slab(folio))
    u |= BIT_ULL(KPF_SLAB);
    u |= kpf_copy_bit(k, KPF_LOCKED,	PG_locked);
    u |= kpf_copy_bit(k, KPF_DIRTY,		PG_dirty);
    u |= kpf_copy_bit(k, KPF_UPTODATE,	PG_uptodate);
    u |= kpf_copy_bit(k, KPF_WRITEBACK,	PG_writeback);
    u |= kpf_copy_bit(k, KPF_LRU,		PG_lru);
    u |= kpf_copy_bit(k, KPF_REFERENCED,	PG_referenced);
    u |= kpf_copy_bit(k, KPF_ACTIVE,	PG_active);
    u |= kpf_copy_bit(k, KPF_RECLAIM,	PG_reclaim);
    if (folio_test_swapcache(folio))
    u |= BIT_ULL(KPF_SWAPCACHE);
    u |= kpf_copy_bit(k, KPF_SWAPBACKED,	PG_swapbacked);
    u |= kpf_copy_bit(k, KPF_UNEVICTABLE,	PG_unevictable);
    u |= kpf_copy_bit(k, KPF_MLOCKED,	PG_mlocked);

    if (u & BIT_ULL(KPF_HUGE))
    u |= kpf_copy_bit(k, KPF_HWPOISON,	PG_hwpoison);
    else
    u |= kpf_copy_bit(ps.page_snapshot.flags.f, KPF_HWPOISON, PG_hwpoison);

    u |= kpf_copy_bit(k, KPF_RESERVED,	PG_reserved);
    u |= kpf_copy_bit(k, KPF_OWNER_2,	PG_owner_2);
    u |= kpf_copy_bit(k, KPF_PRIVATE,	PG_private);
    u |= kpf_copy_bit(k, KPF_PRIVATE_2,	PG_private_2);
    u |= kpf_copy_bit(k, KPF_OWNER_PRIVATE,	PG_owner_priv_1);
    u |= kpf_copy_bit(k, KPF_ARCH,		PG_arch_1);

    u |= kpf_copy_bit(k, KPF_ARCH_2,	PG_arch_2);

    u |= kpf_copy_bit(k, KPF_ARCH_3,	PG_arch_3);

    return u;
    }
    EXPORT_SYMBOL_GPL(stable_page_flags);
// /proc/kpageflags - an array exposing page flags
//
// Each entry is a u64 representing the corresponding
// physical page flags.
//
    static ssize_t kpageflags_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    return kpage_read(file, buf, count, ppos, KPAGE_FLAGS);
    }
    static const struct proc_ops kpageflags_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_lseek	= mem_lseek,
    .proc_read	= kpageflags_read,
    };

    static ssize_t kpagecgroup_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    return kpage_read(file, buf, count, ppos, KPAGE_CGROUP);
    }
    static const struct proc_ops kpagecgroup_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_lseek	= mem_lseek,
    .proc_read	= kpagecgroup_read,
    };

#[no_mangle]
unsafe extern "C" fn proc_page_init() -> int __init {
    static int __init proc_page_init(void)
    {
    proc_create("kpagecount", S_IRUSR, core::ptr::null_mut(), &kpagecount_proc_ops);
    proc_create("kpageflags", S_IRUSR, core::ptr::null_mut(), &kpageflags_proc_ops);

    proc_create("kpagecgroup", S_IRUSR, core::ptr::null_mut(), &kpagecgroup_proc_ops);

    return 0;
    }
    fs_initcall(proc_page_init);
