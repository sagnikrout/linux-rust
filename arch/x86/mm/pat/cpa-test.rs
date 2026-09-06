//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/pat/cpa-test.c
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
//
// self test for change_page_attr.
//
// Clears the a test pte bit on random pages in the direct mapping,
// then reverts and compares page tables forwards and afterwards.
//

//
// Only print the results of the first pass:
//
    let mut print: static __read_mostly int = 1;
    enum {
    NTEST			= 3 * 100,
    NPAGES			= 100,

    LPS			= (1 << PMD_SHIFT),

    LPS			= (1 << PMD_SHIFT),

    LPS			= (1 << 22),

    GPS			= (1<<30)
    };

#[no_mangle]
unsafe extern "C" fn pte_testbit(pte: pte_t) -> c_int {
    static int pte_testbit(pte_t pte)
    {
    return pte_flags(pte) & _PAGE_SOFTW1;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct split_state {
    pub exec: long lpg, gpg, spg,,
    pub max_exec: long min_exec,,
}

#[no_mangle]
unsafe extern "C" fn print_split(s: *mut split_state) -> c_int {
    static int print_split(struct split_state *s)
    {
    long i, expected, missed = 0;
    let mut err: c_int = 0;
    s.lpg = s.gpg = s.spg = s.exec = 0;
    s.min_exec = ~0UL;
    s.max_exec = 0;
    for (i = 0; i < max_pfn_mapped; ) {
    let mut addr: c_ulong = (unsigned long)__va(i << PAGE_SHIFT);
    unsigned int level;
    pte_t *pte;
    pte = lookup_address(addr, &level);
    if (!pte) {
    missed++;
    i++;
    continue;
    }
    if (level == PG_LEVEL_1G && sizeof(long) == 8) {
    s.gpg++;
    i += GPS/PAGE_SIZE;
    } else if (level == PG_LEVEL_2M) {
    if ((pte_val(*pte) & _PAGE_PRESENT) && !(pte_val(*pte) & _PAGE_PSE)) {
    printk(KERN_ERR
    "%lx level %d but not PSE %Lx\n",
    addr, level, (u64)pte_val(*pte));
    err = 1;
    }
    s.lpg++;
    i += LPS/PAGE_SIZE;
    } else {
    s.spg++;
    i++;
    }
    if (!(pte_val(*pte) & _PAGE_NX)) {
    s.exec++;
    if (addr < s.min_exec)
    s.min_exec = addr;
    if (addr > s.max_exec)
    s.max_exec = addr;
    }
    }
    if (print) {
    printk(KERN_INFO
    " 4k %lu large %lu gb %lu x %lu[%lx-%lx] miss %lu\n",
    s.spg, s.lpg, s.gpg, s.exec,
    s.min_exec != ~0UL ? s.min_exec : 0,
    s.max_exec, missed);
    }
    expected = (s.gpg*GPS + s.lpg*LPS)/PAGE_SIZE + s.spg + missed;
    if (expected != i) {
    printk(KERN_ERR "CPA max_pfn_mapped %lu but expected %lu\n",
    max_pfn_mapped, expected);
    return 1;
    }
    return err;
    }
    static unsigned long addr[NTEST];
    static unsigned int len[NTEST];
    static struct page *pages[NPAGES];
    static unsigned long addrs[NPAGES];
// Change the global bit on random pages in the direct mapping
#[no_mangle]
unsafe extern "C" fn pageattr_test() -> c_int {
    static int pageattr_test(void)
    {
    struct split_state sa, sb, sc;
    unsigned long *bm;
    pte_t *pte, pte0;
    let mut failed: c_int = 0;
    unsigned int level;
    int i, k;
    int err;
    if (print)
    printk(KERN_INFO "CPA self-test:\n");
    bm = vzalloc((max_pfn_mapped + 7) / 8);
    if (!bm) {
    printk(KERN_ERR "CPA Cannot vmalloc bitmap\n");
    return -ENOMEM;
    }
    failed += print_split(&sa);
    for (i = 0; i < NTEST; i++) {
    let mut pfn: c_ulong = get_random_u32_below(max_pfn_mapped);
    addr[i] = (unsigned long)__va(pfn << PAGE_SHIFT);
    len[i] = get_random_u32_below(NPAGES);
    len[i] = min_t(unsigned long, len[i], max_pfn_mapped - pfn - 1);
    if (len[i] == 0)
    len[i] = 1;
    pte = core::ptr::null_mut();
    pte0 = pfn_pte(0, __pgprot(0)); /* shut gcc up */
    for (k = 0; k < len[i]; k++) {
    pte = lookup_address(addr[i] + k*PAGE_SIZE, &level);
    if (!pte || pgprot_val(pte_pgprot(*pte)) == 0 ||
    !(pte_val(*pte) & _PAGE_PRESENT)) {
    addr[i] = 0;
    break;
    }
    if (k == 0) {
    pte0 = *pte;
    } else {
    if (pgprot_val(pte_pgprot(*pte)) !=
    pgprot_val(pte_pgprot(pte0))) {
    len[i] = k;
    break;
    }
    }
    if (test_bit(pfn + k, bm)) {
    len[i] = k;
    break;
    }
    __set_bit(pfn + k, bm);
    addrs[k] = addr[i] + k*PAGE_SIZE;
    pages[k] = pfn_to_page(pfn + k);
    }
    if (!addr[i] || !pte || !k) {
    addr[i] = 0;
    continue;
    }
    switch (i % 3) {
    case 0:
    err = change_page_attr_set(&addr[i], len[i], PAGE_CPA_TEST, 0);
    break;
    case 1:
    err = change_page_attr_set(addrs, len[i], PAGE_CPA_TEST, 1);
    break;
    case 2:
    err = cpa_set_pages_array(pages, len[i], PAGE_CPA_TEST);
    break;
    }
    if (err < 0) {
    printk(KERN_ERR "CPA %d failed %d\n", i, err);
    failed++;
    }
    pte = lookup_address(addr[i], &level);
    if (!pte || !pte_testbit(*pte) || pte_huge(*pte)) {
    printk(KERN_ERR "CPA %lx: bad pte %Lx\n", addr[i],
    pte ? (u64)pte_val(*pte) : 0ULL);
    failed++;
    }
    if (level != PG_LEVEL_4K) {
    printk(KERN_ERR "CPA %lx: unexpected level %d\n",
    addr[i], level);
    failed++;
    }
    }
    vfree(bm);
    failed += print_split(&sb);
    for (i = 0; i < NTEST; i++) {
    if (!addr[i])
    continue;
    pte = lookup_address(addr[i], &level);
    if (!pte) {
    printk(KERN_ERR "CPA lookup of %lx failed\n", addr[i]);
    failed++;
    continue;
    }
    err = change_page_attr_clear(&addr[i], len[i], PAGE_CPA_TEST, 0);
    if (err < 0) {
    printk(KERN_ERR "CPA reverting failed: %d\n", err);
    failed++;
    }
    pte = lookup_address(addr[i], &level);
    if (!pte || pte_testbit(*pte)) {
    printk(KERN_ERR "CPA %lx: bad pte after revert %Lx\n",
    addr[i], pte ? (u64)pte_val(*pte) : 0ULL);
    failed++;
    }
    }
    failed += print_split(&sc);
    if (failed) {
    WARN(1, KERN_ERR "NOT PASSED. Please report.\n");
    return -EINVAL;
    } else {
    if (print)
    printk(KERN_INFO "ok.\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_pageattr_test(__unused: *mut c_void) -> c_int {
    static int do_pageattr_test(void *__unused)
    {
    while (!kthread_should_stop()) {
    schedule_timeout_interruptible(HZ*30);
    if (pageattr_test() < 0)
    break;
    if (print)
    print--;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn start_pageattr_test() -> c_int {
    static int start_pageattr_test(void)
    {
    struct task_struct *p;
    p = kthread_create(do_pageattr_test, core::ptr::null_mut(), "pageattr-test");
    if (!IS_ERR(p))
    wake_up_process(p);
    else
    WARN_ON(1);
    return 0;
    }
    device_initcall(start_pageattr_test);
