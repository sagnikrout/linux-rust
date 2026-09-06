//! Automatically rewritten from C to Rust
//! Source: tools/mm/page-types.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// page-types: Tool for querying page flags
//
// Copyright (C) 2009 Intel corporation
//
// Authors: Wu Fengguang <fengguang.wu@intel.com>
//
pub const _FILE_OFFSET_BITS: c_int = 64;
// Macro flag: #define _GNU_SOURCE

//
// pagemap kernel ABI bits
//
pub const PM_ENTRY_BYTES: c_int = 8;
pub const PM_PFRAME_BITS: c_int = 55;

pub const MAX_SWAPFILES_SHIFT: c_int = 5;

//
// kernel page flags
//
pub const KPF_BYTES: c_int = 8;

// [32-] kernel hacking assistances
pub const KPF_RESERVED: c_int = 32;
pub const KPF_MLOCKED: c_int = 33;
pub const KPF_OWNER_2: c_int = 34;
pub const KPF_PRIVATE: c_int = 35;
pub const KPF_PRIVATE_2: c_int = 36;
pub const KPF_OWNER_PRIVATE: c_int = 37;
pub const KPF_ARCH: c_int = 38;

pub const KPF_SOFTDIRTY: c_int = 40;
pub const KPF_ARCH_2: c_int = 41;
// [47-] take some arbitrary free slots for expanding overloaded flags
// not part of kernel API
//
pub const KPF_ANON_EXCLUSIVE: c_int = 47;
pub const KPF_READAHEAD: c_int = 48;
pub const KPF_SLUB_FROZEN: c_int = 50;
pub const KPF_SLUB_DEBUG: c_int = 51;
pub const KPF_FILE: c_int = 61;
pub const KPF_SWAP: c_int = 62;
pub const KPF_MMAP_EXCLUSIVE: c_int = 63;

    static const char * const page_flag_names[] = {
    [KPF_LOCKED]		= "L:locked",
    [KPF_ERROR]		= "E:error",
    [KPF_REFERENCED]	= "R:referenced",
    [KPF_UPTODATE]		= "U:uptodate",
    [KPF_DIRTY]		= "D:dirty",
    [KPF_LRU]		= "l:lru",
    [KPF_ACTIVE]		= "A:active",
    [KPF_SLAB]		= "S:slab",
    [KPF_WRITEBACK]		= "W:writeback",
    [KPF_RECLAIM]		= "I:reclaim",
    [KPF_BUDDY]		= "B:buddy",
    [KPF_MMAP]		= "M:mmap",
    [KPF_ANON]		= "a:anonymous",
    [KPF_SWAPCACHE]		= "s:swapcache",
    [KPF_SWAPBACKED]	= "b:swapbacked",
    [KPF_COMPOUND_HEAD]	= "H:compound_head",
    [KPF_COMPOUND_TAIL]	= "T:compound_tail",
    [KPF_HUGE]		= "G:huge",
    [KPF_UNEVICTABLE]	= "u:unevictable",
    [KPF_HWPOISON]		= "X:hwpoison",
    [KPF_NOPAGE]		= "n:nopage",
    [KPF_KSM]		= "x:ksm",
    [KPF_THP]		= "t:thp",
    [KPF_OFFLINE]		= "o:offline",
    [KPF_PGTABLE]		= "g:pgtable",
    [KPF_ZERO_PAGE]		= "z:zero_page",
    [KPF_IDLE]              = "i:idle_page",
    [KPF_RESERVED]		= "r:reserved",
    [KPF_MLOCKED]		= "m:mlocked",
    [KPF_OWNER_2]		= "d:owner_2",
    [KPF_PRIVATE]		= "P:private",
    [KPF_PRIVATE_2]		= "p:private_2",
    [KPF_OWNER_PRIVATE]	= "O:owner_private",
    [KPF_ARCH]		= "h:arch",
    [KPF_SOFTDIRTY]		= "f:softdirty",
    [KPF_ARCH_2]		= "H:arch_2",
    [KPF_ANON_EXCLUSIVE]	= "d:anon_exclusive",
    [KPF_READAHEAD]		= "I:readahead",
    [KPF_SLUB_FROZEN]	= "A:slub_frozen",
    [KPF_SLUB_DEBUG]	= "E:slub_debug",
    [KPF_FILE]		= "F:file",
    [KPF_SWAP]		= "w:swap",
    [KPF_MMAP_EXCLUSIVE]	= "1:mmap_exclusive",
    };
//
// data structures
//
    static int		opt_raw;	/* for kernel developers */
    static int		opt_list;	/* list pages (in ranges) */
    static int		opt_mark_idle;	/* set accessed bit */
    static int		opt_no_summary;	/* don't show summary */
    static pid_t		opt_pid;	/* process to walk */
    const char		*opt_file;	/* file or directory path */
    static uint64_t		opt_cgroup;	/* cgroup inode */
    static int		opt_list_cgroup;/* list page cgroup */
    static int		opt_list_mapcnt;/* list page map count */
    static const char	*opt_kpageflags;/* kpageflags file to parse */
pub const MAX_ADDR_RANGES: c_int = 1024;
    static int		nr_addr_ranges;
    static unsigned long	opt_offset[MAX_ADDR_RANGES];
    static unsigned long	opt_size[MAX_ADDR_RANGES];
pub const MAX_VMAS: c_int = 10240;
    static int		nr_vmas;
    static unsigned long	pg_start[MAX_VMAS];
    static unsigned long	pg_end[MAX_VMAS];
pub const MAX_BIT_FILTERS: c_int = 64;
    static int		nr_bit_filters;
    static uint64_t		opt_mask[MAX_BIT_FILTERS];
    static uint64_t		opt_bits[MAX_BIT_FILTERS];
    static int		page_size;
    static int		pagemap_fd;
    static int		kpageflags_fd;
    let mut kpagecount_fd: static int = -1;
    let mut kpagecgroup_fd: static int = -1;
    let mut page_idle_fd: static int = -1;
    static int		opt_hwpoison;
    static int		opt_unpoison;
    static const char	*hwpoison_debug_fs;
    static int		hwpoison_inject_fd;
    static int		hwpoison_forget_fd;
pub const HASH_SHIFT: c_int = 13;

    static unsigned long	total_pages;
    static unsigned long	nr_pages[HASH_SIZE];
    static uint64_t		page_flags[HASH_SIZE];
//
// helper functions
//

    type __min1 = (x);			\
    type __min2 = (y);			\
    __min1 < __min2 ? __min1 : __min2; })

    type __max1 = (x);			\
    type __max2 = (y);			\
    __max1 > __max2 ? __max1 : __max2; })
#[no_mangle]
unsafe extern "C" fn pages2mb(pages: c_ulong) -> c_ulong {
    static unsigned long pages2mb(unsigned long pages)
    {
    return (pages * page_size) >> 20;
    }
#[no_mangle]
unsafe extern "C" fn fatal(x: *const c_char, ...) {
    static void fatal(const char *x, ...)
    {
    va_list ap;
    va_start(ap, x);
    vfprintf(stderr, x, ap);
    va_end(ap);
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn checked_open(pathname: *const c_char, flags: c_int) -> c_int {
    static int checked_open(const char *pathname, int flags)
    {
    let mut fd: c_int = open(pathname, flags);
    if (fd < 0) {
    perror(pathname);
    exit(EXIT_FAILURE);
    }
    return fd;
    }
//
// pagemap/kpageflags routines
//
    static unsigned long do_u64_read(int fd, const char *name,
    uint64_t *buf,
    unsigned long index,
    unsigned long count)
    {
    long bytes;
    if (index > ULONG_MAX / 8)
    fatal("index overflow: %lu\n", index);
    bytes = pread(fd, buf, count * 8, (off_t)index * 8);
    if (bytes < 0) {
    perror(name);
    exit(EXIT_FAILURE);
    }
    if (bytes % 8)
    fatal("partial read: %lu bytes\n", bytes);
    return bytes / 8;
    }
    static unsigned long kpageflags_read(uint64_t *buf,
    unsigned long index,
    unsigned long pages)
    {
    return do_u64_read(kpageflags_fd, opt_kpageflags, buf, index, pages);
    }
    static unsigned long kpagecgroup_read(uint64_t *buf,
    unsigned long index,
    unsigned long pages)
    {
    if (kpagecgroup_fd < 0)
    return pages;
    return do_u64_read(kpagecgroup_fd, opt_kpageflags, buf, index, pages);
    }
    static unsigned long kpagecount_read(uint64_t *buf,
    unsigned long index,
    unsigned long pages)
    {
    return kpagecount_fd < 0 ? pages :
    do_u64_read(kpagecount_fd, PROC_KPAGECOUNT,
    buf, index, pages);
    }
    static unsigned long pagemap_read(uint64_t *buf,
    unsigned long index,
    unsigned long pages)
    {
    return do_u64_read(pagemap_fd, "/proc/pid/pagemap", buf, index, pages);
    }
#[no_mangle]
unsafe extern "C" fn pagemap_pfn(val: u64) -> c_ulong {
    static unsigned long pagemap_pfn(uint64_t val)
    {
    unsigned long pfn;
    if (val & PM_PRESENT)
    pfn = PM_PFRAME(val);
    else
    pfn = 0;
    return pfn;
    }
#[no_mangle]
unsafe extern "C" fn pagemap_swap_offset(val: u64) -> c_ulong {
    static unsigned long pagemap_swap_offset(uint64_t val)
    {
    return val & PM_SWAP ? PM_SWAP_OFFSET(val) : 0;
    }
//
// page flag names
//
    static char *page_flag_name(uint64_t flags)
    {
    static char buf[65];
    int present;
    size_t i, j;
    for (i = 0, j = 0; i < ARRAY_SIZE(page_flag_names); i++) {
    present = (flags >> i) & 1;
    if (!page_flag_names[i]) {
    if (present)
    fatal("unknown flag bit %d\n", i);
    continue;
    }
    buf[j++] = present ? page_flag_names[i][0] : '_';
    }
    return buf;
    }
    static char *page_flag_longname(uint64_t flags)
    {
    static char buf[1024];
    size_t i, n;
    for (i = 0, n = 0; i < ARRAY_SIZE(page_flag_names); i++) {
    if (!page_flag_names[i])
    continue;
    if ((flags >> i) & 1)
    n += snprintf(buf + n, sizeof(buf) - n, "%s,",
    page_flag_names[i] + 2);
    }
    if (n)
    n--;
    buf[n] = '\0';
    return buf;
    }
//
// page list and summary
//
    static void show_page_range(unsigned long voffset, unsigned long offset,
    unsigned long size, uint64_t flags,
    uint64_t cgroup, uint64_t mapcnt)
    {
    static uint64_t      flags0;
    static uint64_t	     cgroup0;
    static uint64_t      mapcnt0;
    static unsigned long voff;
    static unsigned long index;
    static unsigned long count;
    if (flags == flags0 && cgroup == cgroup0 && mapcnt == mapcnt0 &&
    offset == index + count && size && voffset == voff + count) {
    count += size;
    return;
    }
    if (count) {
    if (opt_pid)
    printf("%lx\t", voff);
    if (opt_file)
    printf("%lx\t", voff);
    if (opt_list_cgroup)
    printf("@%" PRIu64 "\t", cgroup0);
    if (opt_list_mapcnt)
    printf("%" PRIu64 "\t", mapcnt0);
    printf("%lx\t%lx\t%s\n",
    index, count, page_flag_name(flags0));
    }
    flags0 = flags;
    cgroup0 = cgroup;
    mapcnt0 = mapcnt;
    index  = offset;
    voff   = voffset;
    count  = size;
    }
#[no_mangle]
unsafe extern "C" fn flush_page_range() {
    static void flush_page_range(void)
    {
    show_page_range(0, 0, 0, 0, 0, 0);
    }
    static void show_page(unsigned long voffset, unsigned long offset,
    uint64_t flags, uint64_t cgroup, uint64_t mapcnt)
    {
    if (opt_pid)
    printf("%lx\t", voffset);
    if (opt_file)
    printf("%lx\t", voffset);
    if (opt_list_cgroup)
    printf("@%" PRIu64 "\t", cgroup);
    if (opt_list_mapcnt)
    printf("%" PRIu64 "\t", mapcnt);
    printf("%lx\t%s\n", offset, page_flag_name(flags));
    }
#[no_mangle]
unsafe extern "C" fn show_summary() {
    static void show_summary(void)
    {
    size_t i;
    printf("             flags\tpage-count       MB"
    "  symbolic-flags\t\t\tlong-symbolic-flags\n");
    for (i = 0; i < ARRAY_SIZE(nr_pages); i++) {
    if (nr_pages[i])
    printf("0x%016llx\t%10lu %8lu  %s\t%s\n",
    (unsigned long long)page_flags[i],
    nr_pages[i],
    pages2mb(nr_pages[i]),
    page_flag_name(page_flags[i]),
    page_flag_longname(page_flags[i]));
    }
    printf("             total\t%10lu %8lu\n",
    total_pages, pages2mb(total_pages));
    }
//
// page flag filters
//
#[no_mangle]
unsafe extern "C" fn bit_mask_ok(flags: u64) -> c_int {
    static int bit_mask_ok(uint64_t flags)
    {
    int i;
    for (i = 0; i < nr_bit_filters; i++) {
    if (opt_bits[i] == KPF_ALL_BITS) {
    if ((flags & opt_mask[i]) == 0)
    return 0;
    } else {
    if ((flags & opt_mask[i]) != opt_bits[i])
    return 0;
    }
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn expand_overloaded_flags(flags: u64, pme: u64) -> u64 {
    static uint64_t expand_overloaded_flags(uint64_t flags, uint64_t pme)
    {
// Anonymous pages use PG_owner_2 for anon_exclusive
    if ((flags & BIT(ANON)) && (flags & BIT(OWNER_2)))
    flags ^= BIT(OWNER_2) | BIT(ANON_EXCLUSIVE);
// SLUB overloads several page flags
    if (flags & BIT(SLAB)) {
    if (flags & BIT(ACTIVE))
    flags ^= BIT(ACTIVE) | BIT(SLUB_FROZEN);
    if (flags & BIT(ERROR))
    flags ^= BIT(ERROR) | BIT(SLUB_DEBUG);
    }
// PG_reclaim is overloaded as PG_readahead in the read path
    if ((flags & (BIT(RECLAIM) | BIT(WRITEBACK))) == BIT(RECLAIM))
    flags ^= BIT(RECLAIM) | BIT(READAHEAD);
    if (pme & PM_SOFT_DIRTY)
    flags |= BIT(SOFTDIRTY);
    if (pme & PM_FILE)
    flags |= BIT(FILE);
    if (pme & PM_SWAP)
    flags |= BIT(SWAP);
    if (pme & PM_MMAP_EXCLUSIVE)
    flags |= BIT(MMAP_EXCLUSIVE);
    return flags;
    }
#[no_mangle]
unsafe extern "C" fn well_known_flags(flags: u64) -> u64 {
    static uint64_t well_known_flags(uint64_t flags)
    {
// hide flags intended only for kernel hacker
    flags &= ~KPF_HACKERS_BITS;
// hide non-hugeTLB compound pages
    if ((flags & BITS_COMPOUND) && !(flags & BIT(HUGE)))
    flags &= ~BITS_COMPOUND;
    return flags;
    }
#[no_mangle]
unsafe extern "C" fn kpageflags_flags(flags: u64, pme: u64) -> u64 {
    static uint64_t kpageflags_flags(uint64_t flags, uint64_t pme)
    {
    if (opt_raw)
    flags = expand_overloaded_flags(flags, pme);
    else
    flags = well_known_flags(flags);
    return flags;
    }
//
// page actions
//
#[no_mangle]
unsafe extern "C" fn prepare_hwpoison_fd() {
    static void prepare_hwpoison_fd(void)
    {
    char buf[MAX_PATH + 1];
    hwpoison_debug_fs = debugfs__mount();
    if (!hwpoison_debug_fs) {
    perror("mount debugfs");
    exit(EXIT_FAILURE);
    }
    if (opt_hwpoison && !hwpoison_inject_fd) {
    snprintf(buf, MAX_PATH, "%s/hwpoison/corrupt-pfn",
    hwpoison_debug_fs);
    hwpoison_inject_fd = checked_open(buf, O_WRONLY);
    }
    if (opt_unpoison && !hwpoison_forget_fd) {
    snprintf(buf, MAX_PATH, "%s/hwpoison/unpoison-pfn",
    hwpoison_debug_fs);
    hwpoison_forget_fd = checked_open(buf, O_WRONLY);
    }
    }
#[no_mangle]
unsafe extern "C" fn hwpoison_page(offset: c_ulong) -> c_int {
    static int hwpoison_page(unsigned long offset)
    {
    char buf[100];
    int len;
    len = sprintf(buf, "0x%lx\n", offset);
    len = write(hwpoison_inject_fd, buf, len);
    if (len < 0) {
    perror("hwpoison inject");
    return len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unpoison_page(offset: c_ulong) -> c_int {
    static int unpoison_page(unsigned long offset)
    {
    char buf[100];
    int len;
    len = sprintf(buf, "0x%lx\n", offset);
    len = write(hwpoison_forget_fd, buf, len);
    if (len < 0) {
    perror("hwpoison forget");
    return len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mark_page_idle(offset: c_ulong) -> c_int {
    static int mark_page_idle(unsigned long offset)
    {
    static unsigned long off;
    static uint64_t buf;
    int len;
    if ((offset / 64 == off / 64) || buf == 0) {
    buf |= 1UL << (offset % 64);
    off = offset;
    return 0;
    }
    len = pwrite(page_idle_fd, &buf, 8, 8 * (off / 64));
    if (len < 0) {
    perror("mark page idle");
    return len;
    }
    buf = 1UL << (offset % 64);
    off = offset;
    return 0;
    }
//
// page frame walker
//
#[no_mangle]
unsafe extern "C" fn hash_slot(flags: u64) -> usize {
    static size_t hash_slot(uint64_t flags)
    {
    let mut k: usize = HASH_KEY(flags);
    size_t i;
// Explicitly reserve slot 0 for flags 0: the following logic
// cannot distinguish an unoccupied slot from slot (flags==0).
//
    if (flags == 0)
    return 0;
// search through the remaining (HASH_SIZE-1) slots
    for (i = 1; i < ARRAY_SIZE(page_flags); i++, k++) {
    if (!k || k >= ARRAY_SIZE(page_flags))
    k = 1;
    if (page_flags[k] == 0) {
    page_flags[k] = flags;
    return k;
    }
    if (page_flags[k] == flags)
    return k;
    }
    fatal("hash table full: bump up HASH_SHIFT?\n");
    exit(EXIT_FAILURE);
    }
    static void add_page(unsigned long voffset, unsigned long offset,
    uint64_t flags, uint64_t cgroup, uint64_t mapcnt,
    uint64_t pme)
    {
    flags = kpageflags_flags(flags, pme);
    if (!bit_mask_ok(flags))
    return;
    if (opt_cgroup && cgroup != (uint64_t)opt_cgroup)
    return;
    if (opt_hwpoison)
    hwpoison_page(offset);
    if (opt_unpoison)
    unpoison_page(offset);
    if (opt_mark_idle)
    mark_page_idle(offset);
    if (opt_list == 1)
    show_page_range(voffset, offset, 1, flags, cgroup, mapcnt);
#[no_mangle]
pub unsafe extern "C" fn if(2: opt_list ==) -> else {
    else if (opt_list == 2)
    show_page(voffset, offset, flags, cgroup, mapcnt);
    nr_pages[hash_slot(flags)]++;
    total_pages++;
    }

    static void walk_pfn(unsigned long voffset,
    unsigned long index,
    unsigned long count,
    uint64_t pme)
    {
    uint64_t buf[KPAGEFLAGS_BATCH];
    uint64_t cgi[KPAGEFLAGS_BATCH];
    uint64_t cnt[KPAGEFLAGS_BATCH];
    unsigned long batch;
    unsigned long pages;
    unsigned long i;
//
// kpagecgroup_read() reads only if kpagecgroup were opened, but
// /proc/kpagecgroup might even not exist, so it's better to fill
// them with zeros here.
//
    if (count == 1)
    cgi[0] = 0;
    else
    memset(cgi, 0, sizeof cgi);
    while (count) {
    batch = min_t(unsigned long, count, KPAGEFLAGS_BATCH);
    pages = kpageflags_read(buf, index, batch);
    if (pages == 0)
    break;
    if (kpagecgroup_read(cgi, index, pages) != pages)
    fatal("kpagecgroup returned fewer pages than expected");
    if (kpagecount_read(cnt, index, pages) != pages)
    fatal("kpagecount returned fewer pages than expected");
    for (i = 0; i < pages; i++)
    add_page(voffset + i, index + i,
    buf[i], cgi[i], cnt[i], pme);
    index += pages;
    count -= pages;
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_swap(voffset: c_ulong, pme: u64) {
    static void walk_swap(unsigned long voffset, uint64_t pme)
    {
    let mut flags: u64 = kpageflags_flags(0, pme);
    if (!bit_mask_ok(flags))
    return;
    if (opt_cgroup)
    return;
    if (opt_list == 1)
    show_page_range(voffset, pagemap_swap_offset(pme),
    1, flags, 0, 0);
#[no_mangle]
pub unsafe extern "C" fn if(2: opt_list ==) -> else {
    else if (opt_list == 2)
    show_page(voffset, pagemap_swap_offset(pme), flags, 0, 0);
    nr_pages[hash_slot(flags)]++;
    total_pages++;
    }

#[no_mangle]
unsafe extern "C" fn walk_vma(index: c_ulong, count: c_ulong) {
    static void walk_vma(unsigned long index, unsigned long count)
    {
    uint64_t buf[PAGEMAP_BATCH];
    unsigned long batch;
    unsigned long pages;
    unsigned long pfn;
    unsigned long i;
    while (count) {
    batch = min_t(unsigned long, count, PAGEMAP_BATCH);
    pages = pagemap_read(buf, index, batch);
    if (pages == 0)
    break;
    for (i = 0; i < pages; i++) {
    pfn = pagemap_pfn(buf[i]);
    if (pfn)
    walk_pfn(index + i, pfn, 1, buf[i]);
    if (buf[i] & PM_SWAP)
    walk_swap(index + i, buf[i]);
    }
    index += pages;
    count -= pages;
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_task(index: c_ulong, count: c_ulong) {
    static void walk_task(unsigned long index, unsigned long count)
    {
    let mut end: c_ulong = index + count;
    unsigned long start;
    let mut i: c_int = 0;
    while (index < end) {
    while (pg_end[i] <= index)
    if (++i >= nr_vmas)
    return;
    if (pg_start[i] >= end)
    return;
    start = max_t(unsigned long, pg_start[i], index);
    index = min_t(unsigned long, pg_end[i], end);
    assert(start < index);
    walk_vma(start, index - start);
    }
    }
#[no_mangle]
unsafe extern "C" fn add_addr_range(offset: c_ulong, size: c_ulong) {
    static void add_addr_range(unsigned long offset, unsigned long size)
    {
    if (nr_addr_ranges >= MAX_ADDR_RANGES)
    fatal("too many addr ranges\n");
    opt_offset[nr_addr_ranges] = offset;
    opt_size[nr_addr_ranges] = min_t(unsigned long, size, ULONG_MAX-offset);
    nr_addr_ranges++;
    }
#[no_mangle]
unsafe extern "C" fn walk_addr_ranges() {
    static void walk_addr_ranges(void)
    {
    int i;
    kpageflags_fd = checked_open(opt_kpageflags, O_RDONLY);
    if (!nr_addr_ranges)
    add_addr_range(0, ULONG_MAX);
    for (i = 0; i < nr_addr_ranges; i++)
    if (!opt_pid)
    walk_pfn(opt_offset[i], opt_offset[i], opt_size[i], 0);
    else
    walk_task(opt_offset[i], opt_size[i]);
    if (opt_mark_idle)
    mark_page_idle(0);
    close(kpageflags_fd);
    }
//
// user interface
//
    static const char *page_flag_type(uint64_t flag)
    {
    if (flag & KPF_HACKERS_BITS)
    return "(r)";
    if (flag & KPF_OVERLOADED_BITS)
    return "(o)";
    return "   ";
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    size_t i, j;
    printf(
    "page-types [options]\n"
    "            -r|--raw                   Raw mode, for kernel developers\n"
    "            -d|--describe flags        Describe flags\n"
    "            -a|--addr    addr-spec     Walk a range of pages\n"
    "            -b|--bits    bits-spec     Walk pages with specified bits\n"
    "            -c|--cgroup  path|@inode   Walk pages within memory cgroup\n"
    "            -p|--pid     pid           Walk process address space\n"
    "            -f|--file    filename      Walk file address space\n"
    "            -i|--mark-idle             Mark pages idle\n"
    "            -l|--list                  Show page details in ranges\n"
    "            -L|--list-each             Show page details one by one\n"
    "            -C|--list-cgroup           Show cgroup inode for pages\n"
    "            -M|--list-mapcnt           Show page map count\n"
    "            -N|--no-summary            Don't show summary info\n"
    "            -X|--hwpoison              hwpoison pages\n"
    "            -x|--unpoison              unpoison pages\n"
    "            -F|--kpageflags filename   kpageflags file to parse\n"
    "            -h|--help                  Show this usage message\n"
    "flags:\n"
    "            0x10                       bitfield format, e.g.\n"
    "            anon                       bit-name, e.g.\n"
    "            0x10,anon                  comma-separated list, e.g.\n"
    "addr-spec:\n"
    "            N                          one page at offset N (unit: pages)\n"
    "            N+M                        pages range from N to N+M-1\n"
    "            N,M                        pages range from N to M-1\n"
    "            N,                         pages range from N to end\n"
    "            ,M                         pages range from 0 to M-1\n"
    "bits-spec:\n"
    "            bit1,bit2                  (flags & (bit1|bit2)) != 0\n"
    "            bit1,bit2=bit1             (flags & (bit1|bit2)) == bit1\n"
    "            bit1,~bit2                 (flags & (bit1|bit2)) == bit1\n"
    "            =bit1,bit2                 flags == (bit1|bit2)\n"
    "bit-names:\n"
    );
    for (i = 0, j = 0; i < ARRAY_SIZE(page_flag_names); i++) {
    if (!page_flag_names[i])
    continue;
    printf("%16s%s", page_flag_names[i] + 2,
    page_flag_type(1ULL << i));
    if (++j > 3) {
    j = 0;
    putchar('\n');
    }
    }
    printf("\n                                   "
    "(r) raw mode bits  (o) overloaded bits\n");
    }
#[no_mangle]
unsafe extern "C" fn parse_number(str: *const c_char) -> c_ulonglong {
    static unsigned long long parse_number(const char *str)
    {
    unsigned long long n;
    n = strtoll(str, core::ptr::null_mut(), 0);
    if (n == 0 && str[0] != '0')
    fatal("invalid name or number: %s\n", str);
    return n;
    }
#[no_mangle]
unsafe extern "C" fn parse_pid(str: *const c_char) {
    static void parse_pid(const char *str)
    {
    FILE *file;
    char buf[5000];
    opt_pid = parse_number(str);
    sprintf(buf, "/proc/%d/pagemap", opt_pid);
    pagemap_fd = checked_open(buf, O_RDONLY);
    sprintf(buf, "/proc/%d/maps", opt_pid);
    file = fopen(buf, "r");
    if (!file) {
    perror(buf);
    exit(EXIT_FAILURE);
    }
    while (fgets(buf, sizeof(buf), file) != core::ptr::null_mut()) {
    unsigned long vm_start;
    unsigned long vm_end;
    unsigned long long pgoff;
    int major, minor;
    char r, w, x, s;
    unsigned long ino;
    int n;
    n = sscanf(buf, "%lx-%lx %c%c%c%c %llx %x:%x %lu",
    &vm_start,
    &vm_end,
    &r, &w, &x, &s,
    &pgoff,
    &major, &minor,
    &ino);
    if (n < 10) {
    fprintf(stderr, "unexpected line: %s\n", buf);
    continue;
    }
    pg_start[nr_vmas] = vm_start / page_size;
    pg_end[nr_vmas] = vm_end / page_size;
    if (++nr_vmas >= MAX_VMAS) {
    fprintf(stderr, "too many VMAs\n");
    break;
    }
    }
    fclose(file);
    }
#[no_mangle]
unsafe extern "C" fn show_file(name: *const c_char, st: *const stat) {
    static void show_file(const char *name, const struct stat *st)
    {
    let mut size: c_ulonglong = st.st_size;
    char atime[64], mtime[64];
    let mut now: c_long = time(core::ptr::null_mut());
    printf("%s\tInode: %u\tSize: %llu (%llu pages)\n",
    name, (unsigned)st.st_ino,
    size, (size + page_size - 1) / page_size);
    strftime(atime, sizeof(atime), "%c", localtime(&st.st_atime));
    strftime(mtime, sizeof(mtime), "%c", localtime(&st.st_mtime));
    printf("Modify: %s (%ld seconds ago)\nAccess: %s (%ld seconds ago)\n",
    mtime, now - st.st_mtime,
    atime, now - st.st_atime);
    }
    static sigjmp_buf sigbus_jmp;
    static void * volatile sigbus_addr;
#[no_mangle]
unsafe extern "C" fn sigbus_handler(sig: c_int, info: *mut siginfo_t, ucontex: *mut c_void) {
    static void sigbus_handler(int sig, siginfo_t *info, void *ucontex)
    {
    (void)sig;
    (void)ucontex;
    sigbus_addr = info ? info.si_addr : core::ptr::null_mut();
    siglongjmp(sigbus_jmp, 1);
    }
    static struct sigaction sigbus_action = {
    .sa_sigaction = sigbus_handler,
    .sa_flags = SA_SIGINFO,
    };
    static void walk_file_range(const char *name, int fd,
    unsigned long off, unsigned long end)
    {
    uint8_t vec[PAGEMAP_BATCH];
    uint64_t buf[PAGEMAP_BATCH], flags;
    let mut cgroup: u64 = 0;
    let mut mapcnt: u64 = 0;
    unsigned long nr_pages, pfn, i;
    ssize_t len;
    void *ptr;
    let mut first: c_int = 1;
    for (; off < end; off += len) {
    nr_pages = (end - off + page_size - 1) / page_size;
    if (nr_pages > PAGEMAP_BATCH)
    nr_pages = PAGEMAP_BATCH;
    len = nr_pages * page_size;
    ptr = mmap(core::ptr::null_mut(), len, PROT_READ, MAP_SHARED, fd, off);
    if (ptr == MAP_FAILED)
    fatal("mmap failed: %s", name);
// determine cached pages
    if (mincore(ptr, len, vec))
    fatal("mincore failed: %s", name);
// turn off readahead
    if (madvise(ptr, len, MADV_RANDOM))
    fatal("madvise failed: %s", name);
    if (sigsetjmp(sigbus_jmp, 1)) {
    end = off + (sigbus_addr ? sigbus_addr - ptr : 0);
    fprintf(stderr, "got sigbus at offset %lld: %s\n",
    (long long)end, name);
    goto got_sigbus;
    }
// populate ptes
    for (i = 0; i < nr_pages ; i++) {
    if (vec[i] & 1)
    (void)*(volatile int *)(ptr + i * page_size);
    }
    got_sigbus:
// turn off harvesting reference bits
    if (madvise(ptr, len, MADV_SEQUENTIAL))
    fatal("madvise failed: %s", name);
    if (pagemap_read(buf, (unsigned long)ptr / page_size,
    nr_pages) != nr_pages)
    fatal("cannot read pagemap");
    munmap(ptr, len);
    for (i = 0; i < nr_pages; i++) {
    pfn = pagemap_pfn(buf[i]);
    if (!pfn)
    continue;
    if (!kpageflags_read(&flags, pfn, 1))
    continue;
    if (!kpagecgroup_read(&cgroup, pfn, 1))
    fatal("kpagecgroup_read failed");
    if (!kpagecount_read(&mapcnt, pfn, 1))
    fatal("kpagecount_read failed");
    if (first && opt_list) {
    first = 0;
    flush_page_range();
    }
    add_page(off / page_size + i, pfn,
    flags, cgroup, mapcnt, buf[i]);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_file(name: *const c_char, st: *const stat) {
    static void walk_file(const char *name, const struct stat *st)
    {
    int i;
    int fd;
    fd = checked_open(name, O_RDONLY|O_NOATIME|O_NOFOLLOW);
    if (!nr_addr_ranges)
    add_addr_range(0, st.st_size / page_size);
    for (i = 0; i < nr_addr_ranges; i++)
    walk_file_range(name, fd, opt_offset[i] * page_size,
    (opt_offset[i] + opt_size[i]) * page_size);
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_tree(name: *const c_char, st: *const stat, type: c_int, f: *mut FTW) -> c_int {
    int walk_tree(const char *name, const struct stat *st, int type, struct FTW *f)
    {
    (void)f;
    switch (type) {
    case FTW_F:
    if (S_ISREG(st.st_mode))
    walk_file(name, st);
    break;
    case FTW_DNR:
    fprintf(stderr, "cannot read dir: %s\n", name);
    break;
    }
    return 0;
    }
    struct stat st;
#[no_mangle]
unsafe extern "C" fn walk_page_cache() {
    static void walk_page_cache(void)
    {
    kpageflags_fd = checked_open(opt_kpageflags, O_RDONLY);
    pagemap_fd = checked_open("/proc/self/pagemap", O_RDONLY);
    sigaction(SIGBUS, &sigbus_action, core::ptr::null_mut());
    if (stat(opt_file, &st))
    fatal("stat failed: %s\n", opt_file);
    if (S_ISREG(st.st_mode)) {
    walk_file(opt_file, &st);
    } else if (S_ISDIR(st.st_mode)) {
// do not follow symlinks and mountpoints
    if (nftw(opt_file, walk_tree, 64, FTW_MOUNT | FTW_PHYS) < 0)
    fatal("nftw failed: %s\n", opt_file);
    } else
    fatal("unhandled file type: %s\n", opt_file);
    close(kpageflags_fd);
    close(pagemap_fd);
    signal(SIGBUS, SIG_DFL);
    }
#[no_mangle]
unsafe extern "C" fn parse_file(name: *const c_char) {
    static void parse_file(const char *name)
    {
    opt_file = name;
    }
#[no_mangle]
unsafe extern "C" fn parse_cgroup(path: *const c_char) {
    static void parse_cgroup(const char *path)
    {
    if (path[0] == '@') {
    opt_cgroup = parse_number(path + 1);
    return;
    }
    struct stat st;
    if (stat(path, &st))
    fatal("stat failed: %s: %m\n", path);
    if (!S_ISDIR(st.st_mode))
    fatal("cgroup supposed to be a directory: %s\n", path);
    opt_cgroup = st.st_ino;
    }
#[no_mangle]
unsafe extern "C" fn parse_addr_range(optarg: *const c_char) {
    static void parse_addr_range(const char *optarg)
    {
    unsigned long offset;
    unsigned long size;
    char *p;
    p = strchr(optarg, ',');
    if (!p)
    p = strchr(optarg, '+');
    if (p == optarg) {
    offset = 0;
    size   = parse_number(p + 1);
    } else if (p) {
    offset = parse_number(optarg);
    if (p[1] == '\0')
    size = ULONG_MAX;
    else {
    size = parse_number(p + 1);
    if (*p == ',') {
    if (size < offset)
    fatal("invalid range: %lu,%lu\n",
    offset, size);
    size -= offset;
    }
    }
    } else {
    offset = parse_number(optarg);
    size   = 1;
    }
    add_addr_range(offset, size);
    }
#[no_mangle]
unsafe extern "C" fn add_bits_filter(mask: u64, bits: u64) {
    static void add_bits_filter(uint64_t mask, uint64_t bits)
    {
    if (nr_bit_filters >= MAX_BIT_FILTERS)
    fatal("too much bit filters\n");
    opt_mask[nr_bit_filters] = mask;
    opt_bits[nr_bit_filters] = bits;
    nr_bit_filters++;
    }
#[no_mangle]
unsafe extern "C" fn parse_flag_name(str: *const c_char, len: c_int) -> u64 {
    static uint64_t parse_flag_name(const char *str, int len)
    {
    size_t i;
    if (!*str || !len)
    return 0;
    if (len <= 8 && !strncmp(str, "compound", len))
    return BITS_COMPOUND;
    for (i = 0; i < ARRAY_SIZE(page_flag_names); i++) {
    if (!page_flag_names[i])
    continue;
    if (!strncmp(str, page_flag_names[i] + 2, len))
    return 1ULL << i;
    }
    return parse_number(str);
    }
#[no_mangle]
unsafe extern "C" fn parse_flag_names(str: *const c_char, all: c_int) -> u64 {
    static uint64_t parse_flag_names(const char *str, int all)
    {
    const char *p    = str;
    let mut flags: u64 = 0;
    while (1) {
    if (*p == ',' || *p == '=' || *p == '\0') {
    if ((*str != '~') || (*str == '~' && all && *++str))
    flags |= parse_flag_name(str, p - str);
    if (*p != ',')
    break;
    str = p + 1;
    }
    p++;
    }
    return flags;
    }
#[no_mangle]
unsafe extern "C" fn parse_bits_mask(optarg: *const c_char) {
    static void parse_bits_mask(const char *optarg)
    {
    uint64_t mask;
    uint64_t bits;
    const char *p;
    p = strchr(optarg, '=');
    if (p == optarg) {
    mask = KPF_ALL_BITS;
    bits = parse_flag_names(p + 1, 0);
    } else if (p) {
    mask = parse_flag_names(optarg, 0);
    bits = parse_flag_names(p + 1, 0);
    } else if (strchr(optarg, '~')) {
    mask = parse_flag_names(optarg, 1);
    bits = parse_flag_names(optarg, 0);
    } else {
    mask = parse_flag_names(optarg, 0);
    bits = KPF_ALL_BITS;
    }
    add_bits_filter(mask, bits);
    }
#[no_mangle]
unsafe extern "C" fn parse_kpageflags(name: *const c_char) {
    static void parse_kpageflags(const char *name)
    {
    opt_kpageflags = name;
    }
#[no_mangle]
unsafe extern "C" fn describe_flags(optarg: *const c_char) {
    static void describe_flags(const char *optarg)
    {
    let mut flags: u64 = parse_flag_names(optarg, 0);
    printf("0x%016llx\t%s\t%s\n",
    (unsigned long long)flags,
    page_flag_name(flags),
    page_flag_longname(flags));
    }
    static const struct option opts[] = {
    { "raw"       , 0, core::ptr::null_mut(), 'r' },
    { "pid"       , 1, core::ptr::null_mut(), 'p' },
    { "file"      , 1, core::ptr::null_mut(), 'f' },
    { "addr"      , 1, core::ptr::null_mut(), 'a' },
    { "bits"      , 1, core::ptr::null_mut(), 'b' },
    { "cgroup"    , 1, core::ptr::null_mut(), 'c' },
    { "describe"  , 1, core::ptr::null_mut(), 'd' },
    { "mark-idle" , 0, core::ptr::null_mut(), 'i' },
    { "list"      , 0, core::ptr::null_mut(), 'l' },
    { "list-each" , 0, core::ptr::null_mut(), 'L' },
    { "list-cgroup", 0, core::ptr::null_mut(), 'C' },
    { "list-mapcnt", 0, core::ptr::null_mut(), 'M' },
    { "no-summary", 0, core::ptr::null_mut(), 'N' },
    { "hwpoison"  , 0, core::ptr::null_mut(), 'X' },
    { "unpoison"  , 0, core::ptr::null_mut(), 'x' },
    { "kpageflags", 1, core::ptr::null_mut(), 'F' },
    { "help"      , 0, core::ptr::null_mut(), 'h' },
    { core::ptr::null_mut()        , 0, core::ptr::null_mut(), 0 }
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int c;
    page_size = getpagesize();
    while ((c = getopt_long(argc, argv,
    "rp:f:a:b:d:c:CilLMNXxF:h",
    opts, core::ptr::null_mut())) != -1) {
    switch (c) {
    case 'r':
    opt_raw = 1;
    break;
    case 'p':
    parse_pid(optarg);
    break;
    case 'f':
    parse_file(optarg);
    break;
    case 'a':
    parse_addr_range(optarg);
    break;
    case 'b':
    parse_bits_mask(optarg);
    break;
    case 'c':
    parse_cgroup(optarg);
    break;
    case 'C':
    opt_list_cgroup = 1;
    break;
    case 'd':
    describe_flags(optarg);
    exit(0);
    case 'i':
    opt_mark_idle = 1;
    break;
    case 'l':
    opt_list = 1;
    break;
    case 'L':
    opt_list = 2;
    break;
    case 'M':
    opt_list_mapcnt = 1;
    break;
    case 'N':
    opt_no_summary = 1;
    break;
    case 'X':
    opt_hwpoison = 1;
    prepare_hwpoison_fd();
    break;
    case 'x':
    opt_unpoison = 1;
    prepare_hwpoison_fd();
    break;
    case 'F':
    parse_kpageflags(optarg);
    break;
    case 'h':
    usage();
    exit(0);
    default:
    usage();
    exit(1);
    }
    }
    if (!opt_kpageflags)
    opt_kpageflags = PROC_KPAGEFLAGS;
    if (opt_cgroup || opt_list_cgroup)
    kpagecgroup_fd = checked_open(PROC_KPAGECGROUP, O_RDONLY);
    if (opt_list && opt_list_mapcnt)
    kpagecount_fd = checked_open(PROC_KPAGECOUNT, O_RDONLY);
    if (opt_mark_idle)
    page_idle_fd = checked_open(SYS_KERNEL_MM_PAGE_IDLE, O_RDWR);
    if (opt_list && opt_pid)
    printf("voffset\t");
    if (opt_list && opt_file)
    printf("foffset\t");
    if (opt_list && opt_list_cgroup)
    printf("cgroup\t");
    if (opt_list && opt_list_mapcnt)
    printf("map-cnt\t");
    if (opt_list == 1)
    printf("offset\tlen\tflags\n");
    if (opt_list == 2)
    printf("offset\tflags\n");
    if (opt_file)
    walk_page_cache();
    else
    walk_addr_ranges();
    if (opt_list == 1)
    flush_page_range();
    if (opt_no_summary)
    return 0;
    if (opt_list)
    printf("\n\n");
    if (opt_file) {
    show_file(opt_file, &st);
    printf("\n");
    }
    show_summary();
    if (opt_list_mapcnt)
    close(kpagecount_fd);
    if (page_idle_fd >= 0)
    close(page_idle_fd);
    return 0;
    }
