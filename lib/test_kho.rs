//! Automatically rewritten from C to Rust
//! Source: lib/test_kho.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Test module for KHO
// Copyright (c) 2025 Microsoft Corporation.
//
// Authors:
// Saurabh Sengar <ssengar@microsoft.com>
// Mike Rapoport <rppt@kernel.org>
//

pub const KHO_TEST_MAGIC: c_uint = 0x4b484f21	/* KHO! */;

    let mut max_mem: static long = (PAGE_SIZE << MAX_PAGE_ORDER) * 2;
    module_param(max_mem, long, 0644);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_test_state {
    pub nr_folios: c_uint,
    pub folios: *mut folio,
    pub folios_info: *mut phys_addr_t,
    pub folios_info_phys: kho_vmalloc,
    pub nr_folios_preserved: c_int,
    pub fdt: *mut folio,
    pub csum: __wsum,
}

    static struct kho_test_state kho_test_state;
#[no_mangle]
unsafe extern "C" fn kho_test_unpreserve_data(state: *mut kho_test_state) {
    static void kho_test_unpreserve_data(struct kho_test_state *state)
    {
    for (int i = 0; i < state.nr_folios_preserved; i++)
    kho_unpreserve_folio(state.folios[i]);
    kho_unpreserve_vmalloc(&state.folios_info_phys);
    vfree(state.folios_info);
    }
#[no_mangle]
unsafe extern "C" fn kho_test_preserve_data(state: *mut kho_test_state) -> c_int {
    static int kho_test_preserve_data(struct kho_test_state *state)
    {
    struct kho_vmalloc folios_info_phys;
    phys_addr_t *folios_info;
    int err;
    folios_info = vmalloc_array(state.nr_folios, sizeof(*folios_info));
    if (!folios_info)
    return -ENOMEM;
    err = kho_preserve_vmalloc(folios_info, &folios_info_phys);
    if (err)
    goto err_free_info;
    state.folios_info_phys = folios_info_phys;
    state.folios_info = folios_info;
    for (int i = 0; i < state.nr_folios; i++) {
    struct folio *folio = state.folios[i];
    let mut order: c_uint = folio_order(folio);
    folios_info[i] = virt_to_phys(folio_address(folio)) | order;
    err = kho_preserve_folio(folio);
    if (err)
    goto err_unpreserve;
    state.nr_folios_preserved++;
    }
    return 0;
    err_unpreserve:
//
// kho_test_unpreserve_data frees folio_info, bail out immediately to
// avoid double free
//
    kho_test_unpreserve_data(state);
    return err;
    err_free_info:
    vfree(folios_info);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_prepare_fdt(state: *mut kho_test_state, fdt_size: isize) -> c_int {
    static int kho_test_prepare_fdt(struct kho_test_state *state, ssize_t fdt_size)
    {
    const char compatible[] = KHO_TEST_COMPAT;
    let mut magic: c_uint = KHO_TEST_MAGIC;
    void *fdt = folio_address(state.fdt);
    int err;
    err = fdt_create(fdt, fdt_size);
    err |= fdt_finish_reservemap(fdt);
    err |= fdt_begin_node(fdt, "");
    err |= fdt_property(fdt, "compatible", compatible, sizeof(compatible));
    err |= fdt_property(fdt, "magic", &magic, sizeof(magic));
    err |= fdt_begin_node(fdt, "data");
    err |= fdt_property(fdt, "nr_folios", &state.nr_folios,
    sizeof(state.nr_folios));
    err |= fdt_property(fdt, "folios_info", &state.folios_info_phys,
    sizeof(state.folios_info_phys));
    err |= fdt_property(fdt, "csum", &state.csum, sizeof(state.csum));
    err |= fdt_end_node(fdt);
    err |= fdt_end_node(fdt);
    err |= fdt_finish(fdt);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_preserve(state: *mut kho_test_state) -> c_int {
    static int kho_test_preserve(struct kho_test_state *state)
    {
    ssize_t fdt_size;
    int err;
    fdt_size = state.nr_folios * sizeof(phys_addr_t) + PAGE_SIZE;
    state.fdt = folio_alloc(GFP_KERNEL, get_order(fdt_size));
    if (!state.fdt)
    return -ENOMEM;
    err = kho_preserve_folio(state.fdt);
    if (err)
    goto err_free_fdt;
    err = kho_test_preserve_data(state);
    if (err)
    goto err_unpreserve_fdt;
    err = kho_test_prepare_fdt(state, fdt_size);
    if (err)
    goto err_unpreserve_data;
    err = kho_add_subtree(KHO_TEST_FDT, folio_address(state.fdt),
    fdt_totalsize(folio_address(state.fdt)));
    if (err)
    goto err_unpreserve_data;
    return 0;
    err_unpreserve_data:
    kho_test_unpreserve_data(state);
    err_unpreserve_fdt:
    kho_unpreserve_folio(state.fdt);
    err_free_fdt:
    folio_put(state.fdt);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_generate_data(state: *mut kho_test_state) -> c_int {
    static int kho_test_generate_data(struct kho_test_state *state)
    {
    let mut alloc_size: usize = 0;
    let mut csum: __wsum = 0;
    while (alloc_size < max_mem) {
    let mut order: c_int = get_random_u32() % NR_PAGE_ORDERS;
    struct folio *folio;
    unsigned int size;
    void *addr;
//
// Since get_order() rounds up, make sure that actual
// allocation is smaller so that we won't exceed max_mem
//
    if (alloc_size + (PAGE_SIZE << order) > max_mem) {
    order = get_order(max_mem - alloc_size);
    if (order)
    order--;
    }
    size = PAGE_SIZE << order;
    folio = folio_alloc(GFP_KERNEL | __GFP_NORETRY, order);
    if (!folio)
    goto err_free_folios;
    state.folios[state.nr_folios++] = folio;
    addr = folio_address(folio);
    get_random_bytes(addr, size);
    csum = csum_partial(addr, size, csum);
    alloc_size += size;
    }
    state.csum = csum;
    return 0;
    err_free_folios:
    for (int i = 0; i < state.nr_folios; i++)
    folio_put(state.folios[i]);
    state.nr_folios = 0;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_save() -> c_int {
    static int kho_test_save(void)
    {
    struct kho_test_state *state = &kho_test_state;
    struct folio **folios;
    unsigned long max_nr;
    int err;
    max_mem = PAGE_ALIGN(max_mem);
    max_nr = max_mem >> PAGE_SHIFT;
    folios = kvmalloc_objs(*state.folios, max_nr);
    if (!folios)
    return -ENOMEM;
    state.folios = folios;
    err = kho_test_generate_data(state);
    if (err)
    goto err_free_folios;
    err = kho_test_preserve(state);
    if (err)
    goto err_free_folios;
    return 0;
    err_free_folios:
    kvfree(folios);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_restore_data(fdt: *const c_void, node: c_int) -> c_int {
    static int kho_test_restore_data(const void *fdt, int node)
    {
    const struct kho_vmalloc *folios_info_phys;
    const unsigned int *nr_folios;
    phys_addr_t *folios_info;
    const __wsum *old_csum;
    let mut csum: __wsum = 0;
    int len;
    node = fdt_path_offset(fdt, "/data");
    nr_folios = fdt_getprop(fdt, node, "nr_folios", &len);
    if (!nr_folios || len != sizeof(*nr_folios))
    return -EINVAL;
    old_csum = fdt_getprop(fdt, node, "csum", &len);
    if (!old_csum || len != sizeof(*old_csum))
    return -EINVAL;
    folios_info_phys = fdt_getprop(fdt, node, "folios_info", &len);
    if (!folios_info_phys || len != sizeof(*folios_info_phys))
    return -EINVAL;
    folios_info = kho_restore_vmalloc(folios_info_phys);
    if (!folios_info)
    return -EINVAL;
    for (int i = 0; i < *nr_folios; i++) {
    let mut order: c_uint = folios_info[i] & ~PAGE_MASK;
    let mut phys: phys_addr_t = folios_info[i] & PAGE_MASK;
    let mut size: c_uint = PAGE_SIZE << order;
    struct folio *folio;
    folio = kho_restore_folio(phys);
    if (!folio)
    break;
    if (folio_order(folio) != order)
    break;
    csum = csum_partial(folio_address(folio), size, csum);
    folio_put(folio);
    }
    vfree(folios_info);
    if (csum != *old_csum)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_restore(fdt_phys: phys_addr_t) -> c_int {
    static int kho_test_restore(phys_addr_t fdt_phys)
    {
    void *fdt = phys_to_virt(fdt_phys);
    const unsigned int *magic;
    int node, len, err;
    node = fdt_path_offset(fdt, "/");
    if (node < 0)
    return -EINVAL;
    if (fdt_node_check_compatible(fdt, node, KHO_TEST_COMPAT))
    return -EINVAL;
    magic = fdt_getprop(fdt, node, "magic", &len);
    if (!magic || len != sizeof(*magic))
    return -EINVAL;
    if (*magic != KHO_TEST_MAGIC)
    return -EINVAL;
    err = kho_test_restore_data(fdt, node);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kho_test_init() -> int __init {
    static int __init kho_test_init(void)
    {
    phys_addr_t fdt_phys;
    int err;
    if (!kho_is_enabled())
    return 0;
    err = kho_retrieve_subtree(KHO_TEST_FDT, &fdt_phys, core::ptr::null_mut());
    if (!err) {
    err = kho_test_restore(fdt_phys);
    if (err)
    pr_err("KHO restore failed\n");
    else
    pr_info("KHO restore succeeded\n");
    return err;
    }
    if (err != -ENOENT) {
    pr_warn("failed to retrieve %s FDT: %d\n", KHO_TEST_FDT, err);
    return err;
    }
    return kho_test_save();
    }
    module_init(kho_test_init);
#[no_mangle]
unsafe extern "C" fn kho_test_cleanup() {
    static void kho_test_cleanup(void)
    {
// unpreserve and free the data stored in folios
    kho_test_unpreserve_data(&kho_test_state);
    for (int i = 0; i < kho_test_state.nr_folios; i++)
    folio_put(kho_test_state.folios[i]);
    kvfree(kho_test_state.folios);
// Unpreserve and release the FDT folio
    kho_unpreserve_folio(kho_test_state.fdt);
    folio_put(kho_test_state.fdt);
    }
#[no_mangle]
unsafe extern "C" fn kho_test_exit() -> void __exit {
    static void __exit kho_test_exit(void)
    {
    kho_remove_subtree(folio_address(kho_test_state.fdt));
    kho_test_cleanup();
    }
    module_exit(kho_test_exit);
    MODULE_AUTHOR("Mike Rapoport <rppt@kernel.org>");
    MODULE_DESCRIPTION("KHO test module");
    MODULE_LICENSE("GPL");
