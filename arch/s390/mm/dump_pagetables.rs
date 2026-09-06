//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/dump_pagetables.c
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

    static unsigned long max_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_marker {
    pub is_start: c_int,
    pub start_address: c_ulong,
    pub size: c_ulong,
    pub name: *const c_char,
}

    static struct addr_marker *markers;
    static unsigned int markers_cnt;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_state {
    pub ptdump: ptdump_state,
    pub seq: *mut seq_file,
    pub level: c_int,
    pub current_prot: c_uint,
    pub check_wx: bool,
    pub wx_pages: c_ulong,
    pub start_address: c_ulong,
    pub marker: *const addr_marker,
}

    ({						\
    struct seq_file *__m = (m);		\
    \
    if (__m)				\
    seq_printf(__m, fmt, ##args);	\
    })

    ({						\
    struct seq_file *__m = (m);		\
    \
    if (__m)				\
    seq_puts(__m, fmt);		\
    })
#[no_mangle]
unsafe extern "C" fn print_prot(m: *mut seq_file, pr: c_uint, level: c_int) {
    static void print_prot(struct seq_file *m, unsigned int pr, int level)
    {
    static const char * const level_name[] =
    { "ASCE", "PGD", "PUD", "PMD", "PTE" };
    pt_dump_seq_printf(m, "%s ", level_name[level]);
    if (pr & _PAGE_INVALID) {
    pt_dump_seq_printf(m, "I\n");
    return;
    }
    pt_dump_seq_puts(m, (pr & _PAGE_PROTECT) ? "RO " : "RW ");
    pt_dump_seq_puts(m, (pr & _PAGE_NOEXEC) ? "NX\n" : "X\n");
    }
#[no_mangle]
unsafe extern "C" fn note_prot_wx(st: *mut pg_state, addr: c_ulong) {
    static void note_prot_wx(struct pg_state *st, unsigned long addr)
    {
    if (!st.check_wx)
    return;
    if (st.current_prot & _PAGE_INVALID)
    return;
    if (st.current_prot & _PAGE_PROTECT)
    return;
    if (st.current_prot & _PAGE_NOEXEC)
    return;
//
// The first lowcore page is W+X if spectre mitigations are using
// trampolines or the BEAR enhancements facility is not installed,
// in which case we have two lpswe instructions in lowcore that need
// to be executable.
//
    if (addr == PAGE_SIZE && (nospec_uses_trampoline() || !cpu_has_bear()))
    return;
    WARN_ONCE(IS_ENABLED(CONFIG_DEBUG_WX),
    "s390/mm: Found insecure W+X mapping at address %pS\n",
    (void *)st.start_address);
    st.wx_pages += (addr - st.start_address) / PAGE_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn note_page_update_state(st: *mut pg_state, addr: c_ulong, prot: c_uint, level: c_int) {
    static void note_page_update_state(struct pg_state *st, unsigned long addr, unsigned int prot, int level)
    {
    struct seq_file *m = st.seq;
    while (addr >= st.marker[1].start_address) {
    st.marker++;
    pt_dump_seq_printf(m, "---[ %s %s ]---\n", st.marker.name,
    st.marker.is_start ? "Start" : "End");
    }
    st.start_address = addr;
    st.current_prot = prot;
    st.level = level;
    }
#[no_mangle]
unsafe extern "C" fn note_page(pt_st: *mut ptdump_state, addr: c_ulong, level: c_int, val: u64) {
    static void note_page(struct ptdump_state *pt_st, unsigned long addr, int level, u64 val)
    {
    let mut width: c_int = sizeof(unsigned long) * 2;
    static const char units[] = "KMGTPE";
    const char *unit = units;
    unsigned long delta;
    struct pg_state *st;
    struct seq_file *m;
    unsigned int prot;
    st = container_of(pt_st, struct pg_state, ptdump);
    m = st.seq;
    prot = val & (_PAGE_PROTECT | _PAGE_NOEXEC);
    if (level == 4 && (val & _PAGE_INVALID))
    prot = _PAGE_INVALID;
// For pmd_none() & friends val gets passed as zero.
    if (level != 4 && !val)
    prot = _PAGE_INVALID;
// Final flush from generic code.
    if (level == -1)
    addr = max_addr;
    if (st.level == -1) {
    pt_dump_seq_puts(m, "---[ Kernel Virtual Address Space ]---\n");
    note_page_update_state(st, addr, prot, level);
    } else if (prot != st.current_prot || level != st.level ||
    addr >= st.marker[1].start_address) {
    note_prot_wx(st, addr);
    pt_dump_seq_printf(m, "0x%0*lx-0x%0*lx ",
    width, st.start_address,
    width, addr);
    delta = (addr - st.start_address) >> 10;
    while (!(delta & 0x3ff) && unit[1]) {
    delta >>= 10;
    unit++;
    }
    pt_dump_seq_printf(m, "%9lu%c ", delta, *unit);
    print_prot(m, st.current_prot, st.level);
    note_page_update_state(st, addr, prot, level);
    }
    }
#[no_mangle]
unsafe extern "C" fn note_page_pte(pt_st: *mut ptdump_state, addr: c_ulong, pte: pte_t) {
    static void note_page_pte(struct ptdump_state *pt_st, unsigned long addr, pte_t pte)
    {
    note_page(pt_st, addr, 4, pte_val(pte));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pmd(pt_st: *mut ptdump_state, addr: c_ulong, pmd: pmd_t) {
    static void note_page_pmd(struct ptdump_state *pt_st, unsigned long addr, pmd_t pmd)
    {
    note_page(pt_st, addr, 3, pmd_val(pmd));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pud(pt_st: *mut ptdump_state, addr: c_ulong, pud: pud_t) {
    static void note_page_pud(struct ptdump_state *pt_st, unsigned long addr, pud_t pud)
    {
    note_page(pt_st, addr, 2, pud_val(pud));
    }
#[no_mangle]
unsafe extern "C" fn note_page_p4d(pt_st: *mut ptdump_state, addr: c_ulong, p4d: p4d_t) {
    static void note_page_p4d(struct ptdump_state *pt_st, unsigned long addr, p4d_t p4d)
    {
    note_page(pt_st, addr, 1, p4d_val(p4d));
    }
#[no_mangle]
unsafe extern "C" fn note_page_pgd(pt_st: *mut ptdump_state, addr: c_ulong, pgd: pgd_t) {
    static void note_page_pgd(struct ptdump_state *pt_st, unsigned long addr, pgd_t pgd)
    {
    note_page(pt_st, addr, 0, pgd_val(pgd));
    }
#[no_mangle]
unsafe extern "C" fn note_page_flush(pt_st: *mut ptdump_state) {
    static void note_page_flush(struct ptdump_state *pt_st)
    {
    let mut pte_zero: pte_t = {0};
    note_page(pt_st, 0, -1, pte_val(pte_zero));
    }
#[no_mangle]
pub unsafe extern "C" fn ptdump_check_wx() -> bool {
    bool ptdump_check_wx(void)
    {
    struct pg_state st = {
    .ptdump = {
    .note_page_pte = note_page_pte,
    .note_page_pmd = note_page_pmd,
    .note_page_pud = note_page_pud,
    .note_page_p4d = note_page_p4d,
    .note_page_pgd = note_page_pgd,
    .note_page_flush = note_page_flush,
    .range = (struct ptdump_range[]) {
    {.start = 0, .end = max_addr},
    {.start = 0, .end = 0},
    }
    },
    .seq = core::ptr::null_mut(),
    .level = -1,
    .current_prot = 0,
    .check_wx = true,
    .wx_pages = 0,
    .start_address = 0,
    .marker = (struct addr_marker[]) {
    { .start_address =  0, .name = core::ptr::null_mut()},
    { .start_address = -1, .name = core::ptr::null_mut()},
    },
    };
    if (!cpu_has_nx())
    return true;
    ptdump_walk_pgd(&st.ptdump, &init_mm, core::ptr::null_mut());
    if (st.wx_pages) {
    pr_warn("Checked W+X mappings: FAILED, %lu W+X pages found\n", st.wx_pages);
    return false;
    } else {
    pr_info("Checked W+X mappings: passed, no %sW+X pages found\n",
    (nospec_uses_trampoline() || !cpu_has_bear()) ?
    "unexpected " : "");
    return true;
    }
    }

#[no_mangle]
unsafe extern "C" fn ptdump_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_show(struct seq_file *m, void *v)
    {
    struct pg_state st = {
    .ptdump = {
    .note_page_pte = note_page_pte,
    .note_page_pmd = note_page_pmd,
    .note_page_pud = note_page_pud,
    .note_page_p4d = note_page_p4d,
    .note_page_pgd = note_page_pgd,
    .note_page_flush = note_page_flush,
    .range = (struct ptdump_range[]) {
    {.start = 0, .end = max_addr},
    {.start = 0, .end = 0},
    }
    },
    .seq = m,
    .level = -1,
    .current_prot = 0,
    .check_wx = false,
    .wx_pages = 0,
    .start_address = 0,
    .marker = markers,
    };
    mutex_lock(&cpa_mutex);
    ptdump_walk_pgd(&st.ptdump, &init_mm, core::ptr::null_mut());
    mutex_unlock(&cpa_mutex);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump);

#[no_mangle]
unsafe extern "C" fn ptdump_cmp(a: *const c_void, b: *const c_void) -> c_int {
    static int ptdump_cmp(const void *a, const void *b)
    {
    const struct addr_marker *ama = a;
    const struct addr_marker *amb = b;
    if (ama.start_address > amb.start_address)
    return 1;
    if (ama.start_address < amb.start_address)
    return -1;
//
// If the start addresses of two markers are identical sort markers in an
// order that considers areas contained within other areas correctly.
//
    if (ama.is_start && amb.is_start) {
    if (ama.size > amb.size)
    return -1;
    if (ama.size < amb.size)
    return 1;
    return 0;
    }
    if (!ama.is_start && !amb.is_start) {
    if (ama.size > amb.size)
    return 1;
    if (ama.size < amb.size)
    return -1;
    return 0;
    }
    if (ama.is_start)
    return 1;
    if (amb.is_start)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn add_marker(start: c_ulong, end: c_ulong, name: *const c_char) -> c_int {
    static int add_marker(unsigned long start, unsigned long end, const char *name)
    {
    struct addr_marker *new;
    size_t newsize;
    newsize = (markers_cnt + 2) * sizeof(*markers);
    new = kvrealloc(markers, newsize, GFP_KERNEL);
    if (!new)
    return -ENOMEM;
    markers = new;
    markers[markers_cnt].is_start = 1;
    markers[markers_cnt].start_address = start;
    markers[markers_cnt].size = end - start;
    markers[markers_cnt].name = name;
    markers_cnt++;
    markers[markers_cnt].is_start = 0;
    markers[markers_cnt].start_address = end;
    markers[markers_cnt].size = end - start;
    markers[markers_cnt].name = name;
    markers_cnt++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pt_dump_init() -> c_int {
    static int pt_dump_init(void)
    {

    let mut kfence_start: c_ulong = (unsigned long)__kfence_pool;

    let mut lowcore: c_ulong = (unsigned long)get_lowcore();
    int rc;
//
// Figure out the maximum virtual address being accessible with the
// kernel ASCE. We need this to keep the page table walker functions
// from accessing non-existent entries.
//
    max_addr = (get_lowcore().kernel_asce.val & _REGION_ENTRY_TYPE_MASK) >> 2;
    max_addr = 1UL << (max_addr * 11 + 31);
// start + end markers - must be added first
    rc = add_marker(0, -1UL, core::ptr::null_mut());
    rc |= add_marker((unsigned long)_stext, (unsigned long)_end, "Kernel Image");
    rc |= add_marker(lowcore, lowcore + sizeof(struct lowcore), "Lowcore");
    rc |= add_marker(__identity_base, __identity_base + ident_map_size, "Identity Mapping");
    rc |= add_marker((unsigned long)__samode31, (unsigned long)__eamode31, "Amode31 Area");
    rc |= add_marker(MODULES_VADDR, MODULES_END, "Modules Area");
    rc |= add_marker(__abs_lowcore, __abs_lowcore + ABS_LOWCORE_MAP_SIZE, "Lowcore Area");
    rc |= add_marker(__memcpy_real_area, __memcpy_real_area + MEMCPY_REAL_SIZE, "Real Memory Copy Area");
    rc |= add_marker((unsigned long)vmemmap, (unsigned long)vmemmap + vmemmap_size, "vmemmap Area");
    rc |= add_marker(VMALLOC_START, VMALLOC_END, "vmalloc Area");

    rc |= add_marker(kfence_start, kfence_start + KFENCE_POOL_SIZE, "KFence Pool");

    rc |= add_marker(KMSAN_VMALLOC_SHADOW_START, KMSAN_VMALLOC_SHADOW_END, "Kmsan vmalloc Shadow");
    rc |= add_marker(KMSAN_VMALLOC_ORIGIN_START, KMSAN_VMALLOC_ORIGIN_END, "Kmsan vmalloc Origins");
    rc |= add_marker(KMSAN_MODULES_SHADOW_START, KMSAN_MODULES_SHADOW_END, "Kmsan Modules Shadow");
    rc |= add_marker(KMSAN_MODULES_ORIGIN_START, KMSAN_MODULES_ORIGIN_END, "Kmsan Modules Origins");

    rc |= add_marker(KASAN_SHADOW_START, KASAN_SHADOW_END, "Kasan Shadow");

    if (rc)
    goto error;
    sort(&markers[1], markers_cnt - 1, sizeof(*markers), ptdump_cmp, core::ptr::null_mut());

    debugfs_create_file("kernel_page_tables", 0400, core::ptr::null_mut(), core::ptr::null_mut(), &ptdump_fops);

    return 0;
    error:
    kvfree(markers);
    return -ENOMEM;
    }
    device_initcall(pt_dump_init);
