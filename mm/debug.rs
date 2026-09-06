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
    const struct trace_print_flags pageflag_names[] = {
    __def_pageflag_names,
    {0, core::ptr::null_mut()}
    };
    const struct trace_print_flags gfpflag_names[] = {
    __def_gfpflag_names,
    {0, core::ptr::null_mut()}
    };
    const struct trace_print_flags vmaflag_names[] = {
    __def_vmaflag_names,
    {0, core::ptr::null_mut()}
    };

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
    let mut i: unsigned = (page_type >> 24) - 0xf0;
    if (i >= ARRAY_SIZE(page_type_names))
    return "unknown";
    return page_type_names[i];
    }
    static void __dump_folio(const struct folio *folio, const struct page *page,
    unsigned long pfn, unsigned long idx)
    {
    struct address_space *mapping = folio_mapping(folio);
    let mut mapcount: c_int = atomic_read(&page._mapcount) + 1;
    char *type = "";
    if (page_mapcount_is_type(mapcount))
    mapcount = 0;
    pr_warn("page: refcount:%d mapcount:%d mapping:%p index:%#lx pfn:%#lx\n",
    folio_ref_count(folio), mapcount, mapping,
    folio.index + idx, pfn);
    if (folio_test_large(folio)) {
    let mut pincount: c_int = 0;
    if (folio_has_pincount(folio))
    pincount = atomic_read(&folio._pincount);
    pr_warn("head: order:%u mapcount:%d entire_mapcount:%d nr_pages_mapped:%d pincount:%d\n",
    folio_order(folio),
    folio_mapcount(folio),
    folio_entire_mapcount(folio),
    folio_nr_pages_mapped(folio),
    pincount);
    }

    if (folio.memcg_data)
    pr_warn("memcg:%lx\n", folio.memcg_data);

    if (folio_test_ksm(folio))
    type = "ksm ";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: folio_test_anon(folio)) -> else {
    else if (folio_test_anon(folio))
    type = "anon ";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mapping) -> else {
    else if (mapping)
    dump_mapping(mapping);
    BUILD_BUG_ON(ARRAY_SIZE(pageflag_names) != __NR_PAGEFLAGS + 1);
//
// Accessing the pageblock without the zone lock. It could change to
// "isolate" again in the meantime, but since we are just dumping the
// state for debugging, it should be fine to accept a bit of
// inaccuracy here due to racing.
//
    pr_warn("%sflags: %pGp%s\n", type, &folio.flags,
    is_migrate_cma_folio(folio, pfn) ? " CMA" : "");
    if (page_has_type(&folio.page))
    pr_warn("page_type: %x(%s)\n", folio.page.page_type >> 24,
    page_type_name(folio.page.page_type));
    print_hex_dump(KERN_WARNING, "raw: ", DUMP_PREFIX_NONE, 32,
    sizeof(unsigned long), page,
    sizeof(struct page), false);
    if (folio_test_large(folio))
    print_hex_dump(KERN_WARNING, "head: ", DUMP_PREFIX_NONE, 32,
    sizeof(unsigned long), folio,
    2 * sizeof(struct page), false);
    }
#[no_mangle]
unsafe extern "C" fn __dump_page(page: *const page) {
    static void __dump_page(const struct page *page)
    {
    struct page_snapshot ps;
    snapshot_page(&ps, page);
    if (!snapshot_page_is_faithful(&ps))
    pr_warn("page does not match folio\n");
    __dump_folio(&ps.folio_snapshot, &ps.page_snapshot, ps.pfn, ps.idx);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_page(page: *const page, reason: *const c_char) {
    void dump_page(const struct page *page, const char *reason)
    {
    if (PagePoisoned(page))
    pr_warn("page:%p is uninitialized and poisoned\n", page);
    else
    __dump_page(page);
    if (reason)
    pr_warn("page dumped because: %s\n", reason);
    dump_page_owner(page);
    }
    EXPORT_SYMBOL(dump_page);

#[no_mangle]
pub unsafe extern "C" fn dump_vma(vma: *const vm_area_struct) {
    void dump_vma(const struct vm_area_struct *vma)
    {
    pr_emerg("vma %px start %px end %px mm %px\n"
    "prot %lx anon_vma %px vm_ops %px\n"
    "pgoff %lx file %px private_data %px\n"

    "refcnt %x\n"

    "flags: %#lx(%pGv)\n",
    vma, (void *)vma.vm_start, (void *)vma.vm_end, vma.vm_mm,
    (unsigned long)pgprot_val(vma.vm_page_prot),
    vma.anon_vma, vma.vm_ops, vma_start_pgoff(vma),
    vma.vm_file, vma.vm_private_data,

    refcount_read(&vma.vm_refcnt),

    vma.vm_flags, &vma.vm_flags);
    }
    EXPORT_SYMBOL(dump_vma);
#[no_mangle]
pub unsafe extern "C" fn dump_mm(mm: *const mm_struct) {
    void dump_mm(const struct mm_struct *mm)
    {
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
    void dump_vmg(const struct vma_merge_struct *vmg, const char *reason)
    {
    if (reason)
    pr_warn("vmg %px dumped because: %s\n", vmg, reason);
    if (!vmg) {
    pr_warn("vmg %px state: (core::ptr::null_mut())\n", vmg);
    return;
    }
    pr_warn("vmg %px state: mm %px pgoff %lx\n"
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

    (void *)0,

    vmg.anon_name,
    (int)vmg.state,
    vmg.just_expand,
    vmg.__adjust_middle_start, vmg.__adjust_next_start,
    vmg.__remove_middle, vmg.__remove_next);
    if (vmg.mm) {
    pr_warn("vmg %px mm:\n", vmg);
    dump_mm(vmg.mm);
    } else {
    pr_warn("vmg %px mm: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.prev) {
    pr_warn("vmg %px prev:\n", vmg);
    dump_vma(vmg.prev);
    } else {
    pr_warn("vmg %px prev: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.middle) {
    pr_warn("vmg %px middle:\n", vmg);
    dump_vma(vmg.middle);
    } else {
    pr_warn("vmg %px middle: (core::ptr::null_mut())\n", vmg);
    }
    if (vmg.next) {
    pr_warn("vmg %px next:\n", vmg);
    dump_vma(vmg.next);
    } else {
    pr_warn("vmg %px next: (core::ptr::null_mut())\n", vmg);
    }

    if (vmg.vmi) {
    pr_warn("vmg %px vmi:\n", vmg);
    vma_iter_dump_tree(vmg.vmi);
    } else {
    pr_warn("vmg %px vmi: (core::ptr::null_mut())\n", vmg);
    }

    }
    EXPORT_SYMBOL(dump_vmg);
    let mut __read_mostly: static bool page_init_poisoning = true;
#[no_mangle]
unsafe extern "C" fn setup_vm_debug(str: *mut c_char) -> int __init {
    static int __init setup_vm_debug(char *str)
    {
    let mut __page_init_poisoning: bool = true;
//
// Calling vm_debug with no arguments is equivalent to requesting
// to enable all debugging options we can control.
//
    if (*str++ != '=' || !*str)
    goto out;
    __page_init_poisoning = false;
    if (*str == '-')
    goto out;
    while (*str) {
    switch (tolower(*str)) {
    case 'p':
    __page_init_poisoning = true;
    break;
    default:
    pr_err("vm_debug option '%c' unknown. skipped\n",
// str);
    }
    str++;
    }
    out:
    if (page_init_poisoning && !__page_init_poisoning)
    pr_warn("Page struct poisoning disabled by kernel command line option 'vm_debug'\n");
    page_init_poisoning = __page_init_poisoning;
    return 1;
    }
    __setup("vm_debug", setup_vm_debug);
#[no_mangle]
pub unsafe extern "C" fn page_init_poison(page: *mut page, size: usize) {
    void page_init_poison(struct page *page, size_t size)
    {
    if (page_init_poisoning)
    memset(page, PAGE_POISON_PATTERN, size);
    }
#[no_mangle]
pub unsafe extern "C" fn vma_iter_dump_tree(vmi: *const vma_iterator) {
    void vma_iter_dump_tree(const struct vma_iterator *vmi)
    {

    mas_dump(&vmi.mas);
    mt_dump(vmi.mas.tree, mt_dump_hex);

    }
