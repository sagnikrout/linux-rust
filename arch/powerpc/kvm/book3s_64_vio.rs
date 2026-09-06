//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_64_vio.c
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
// Copyright 2010 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
// Copyright 2011 David Gibson, IBM Corporation <dwg@au1.ibm.com>
// Copyright 2016 Alexey Kardashevskiy, IBM Corporation <aik@au1.ibm.com>
//

    static struct kvmppc_spapr_tce_table *kvmppc_find_table(struct kvm *kvm,
    unsigned long liobn)
    {
    struct kvmppc_spapr_tce_table *stt;
    list_for_each_entry_lockless(stt, &kvm.arch.spapr_tce_tables, list)
    if (stt.liobn == liobn)
    return stt;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_tce_pages(iommu_pages: c_ulong) -> c_ulong {
    static unsigned long kvmppc_tce_pages(unsigned long iommu_pages)
    {
    return ALIGN(iommu_pages * sizeof(u64), PAGE_SIZE) / PAGE_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_stt_pages(tce_pages: c_ulong) -> c_ulong {
    static unsigned long kvmppc_stt_pages(unsigned long tce_pages)
    {
    unsigned long stt_bytes = sizeof(struct kvmppc_spapr_tce_table) +
    (tce_pages * sizeof(struct page *));
    return tce_pages + ALIGN(stt_bytes, PAGE_SIZE) / PAGE_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn kvm_spapr_tce_iommu_table_free(head: *mut rcu_head) {
    static void kvm_spapr_tce_iommu_table_free(struct rcu_head *head)
    {
    struct kvmppc_spapr_tce_iommu_table *stit = container_of(head,
    struct kvmppc_spapr_tce_iommu_table, rcu);
    iommu_tce_table_put(stit.tbl);
    kfree(stit);
    }
#[no_mangle]
unsafe extern "C" fn kvm_spapr_tce_liobn_put(kref: *mut kref) {
    static void kvm_spapr_tce_liobn_put(struct kref *kref)
    {
    struct kvmppc_spapr_tce_iommu_table *stit = container_of(kref,
    struct kvmppc_spapr_tce_iommu_table, kref);
    list_del_rcu(&stit.next);
    call_rcu(&stit.rcu, kvm_spapr_tce_iommu_table_free);
    }
    void kvm_spapr_tce_release_iommu_group(struct kvm *kvm,
    struct iommu_group *grp)
    {
    int i;
    struct kvmppc_spapr_tce_table *stt;
    struct kvmppc_spapr_tce_iommu_table *stit, *tmp;
    struct iommu_table_group *table_group = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(stt, &kvm.arch.spapr_tce_tables, list) {
    table_group = iommu_group_get_iommudata(grp);
    if (WARN_ON(!table_group))
    continue;
    list_for_each_entry_safe(stit, tmp, &stt.iommu_tables, next) {
    for (i = 0; i < IOMMU_TABLE_GROUP_MAX_TABLES; ++i) {
    if (table_group.tables[i] != stit.tbl)
    continue;
    kref_put(&stit.kref, kvm_spapr_tce_liobn_put);
    }
    }
    cond_resched_rcu();
    }
    rcu_read_unlock();
    }
    long kvm_spapr_tce_attach_iommu_group(struct kvm *kvm, int tablefd,
    struct iommu_group *grp)
    {
    struct kvmppc_spapr_tce_table *stt = core::ptr::null_mut();
    let mut found: bool = false;
    struct iommu_table *tbl = core::ptr::null_mut();
    struct iommu_table_group *table_group;
    long i;
    struct kvmppc_spapr_tce_iommu_table *stit;
    CLASS(fd, f)(tablefd);
    if (fd_empty(f))
    return -EBADF;
    rcu_read_lock();
    list_for_each_entry_rcu(stt, &kvm.arch.spapr_tce_tables, list) {
    if (stt == fd_file(f).private_data) {
    found = true;
    break;
    }
    }
    rcu_read_unlock();
    if (!found)
    return -EINVAL;
    table_group = iommu_group_get_iommudata(grp);
    if (WARN_ON(!table_group))
    return -EFAULT;
    for (i = 0; i < IOMMU_TABLE_GROUP_MAX_TABLES; ++i) {
    struct iommu_table *tbltmp = table_group.tables[i];
    if (!tbltmp)
    continue;
// Make sure hardware table parameters are compatible
    if ((tbltmp.it_page_shift <= stt.page_shift) &&
    (tbltmp.it_offset << tbltmp.it_page_shift ==
    stt.offset << stt.page_shift) &&
    (tbltmp.it_size << tbltmp.it_page_shift >=
    stt.size << stt.page_shift)) {
//
// Reference the table to avoid races with
// add/remove DMA windows.
//
    tbl = iommu_tce_table_get(tbltmp);
    break;
    }
    }
    if (!tbl)
    return -EINVAL;
    rcu_read_lock();
    list_for_each_entry_rcu(stit, &stt.iommu_tables, next) {
    if (tbl != stit.tbl)
    continue;
    if (!kref_get_unless_zero(&stit.kref)) {
// stit is being destroyed
    iommu_tce_table_put(tbl);
    rcu_read_unlock();
    return -ENOTTY;
    }
//
// The table is already known to this KVM, we just increased
// its KVM reference counter and can return.
//
    rcu_read_unlock();
    return 0;
    }
    rcu_read_unlock();
    stit = kzalloc_obj(*stit);
    if (!stit) {
    iommu_tce_table_put(tbl);
    return -ENOMEM;
    }
    stit.tbl = tbl;
    kref_init(&stit.kref);
    list_add_rcu(&stit.next, &stt.iommu_tables);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn release_spapr_tce_table(head: *mut rcu_head) {
    static void release_spapr_tce_table(struct rcu_head *head)
    {
    struct kvmppc_spapr_tce_table *stt = container_of(head,
    struct kvmppc_spapr_tce_table, rcu);
    unsigned long i, npages = kvmppc_tce_pages(stt.size);
    for (i = 0; i < npages; i++)
    if (stt.pages[i])
    __free_page(stt.pages[i]);
    kfree(stt);
    }
    static struct page *kvm_spapr_get_tce_page(struct kvmppc_spapr_tce_table *stt,
    unsigned long sttpage)
    {
    struct page *page = stt.pages[sttpage];
    if (page)
    return page;
    mutex_lock(&stt.alloc_lock);
    page = stt.pages[sttpage];
    if (!page) {
    page = alloc_page(GFP_KERNEL | __GFP_ZERO);
    WARN_ON_ONCE(!page);
    if (page)
    stt.pages[sttpage] = page;
    }
    mutex_unlock(&stt.alloc_lock);
    return page;
    }
#[no_mangle]
unsafe extern "C" fn kvm_spapr_tce_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t kvm_spapr_tce_fault(struct vm_fault *vmf)
    {
    struct kvmppc_spapr_tce_table *stt = vmf.vma.vm_file.private_data;
    struct page *page;
    if (vmf.pgoff >= kvmppc_tce_pages(stt.size))
    return VM_FAULT_SIGBUS;
    page = kvm_spapr_get_tce_page(stt, vmf.pgoff);
    if (!page)
    return VM_FAULT_OOM;
    get_page(page);
    vmf.page = page;
    return 0;
    }
    static const struct vm_operations_struct kvm_spapr_tce_vm_ops = {
    .fault = kvm_spapr_tce_fault,
    };
#[no_mangle]
unsafe extern "C" fn kvm_spapr_tce_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int kvm_spapr_tce_mmap(struct file *file, struct vm_area_struct *vma)
    {
    vma.vm_ops = &kvm_spapr_tce_vm_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_spapr_tce_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int kvm_spapr_tce_release(struct inode *inode, struct file *filp)
    {
    struct kvmppc_spapr_tce_table *stt = filp.private_data;
    struct kvmppc_spapr_tce_iommu_table *stit, *tmp;
    struct kvm *kvm = stt.kvm;
    mutex_lock(&kvm.lock);
    list_del_rcu(&stt.list);
    mutex_unlock(&kvm.lock);
    list_for_each_entry_safe(stit, tmp, &stt.iommu_tables, next) {
    WARN_ON(!kref_read(&stit.kref));
    while (1) {
    if (kref_put(&stit.kref, kvm_spapr_tce_liobn_put))
    break;
    }
    }
    account_locked_vm(kvm.mm,
    kvmppc_stt_pages(kvmppc_tce_pages(stt.size)), false);
    kvm_put_kvm(stt.kvm);
    call_rcu(&stt.rcu, release_spapr_tce_table);
    return 0;
    }
    static const struct file_operations kvm_spapr_tce_fops = {
    .mmap           = kvm_spapr_tce_mmap,
    .release	= kvm_spapr_tce_release,
    };
    int kvm_vm_ioctl_create_spapr_tce(struct kvm *kvm,
    struct kvm_create_spapr_tce_64 *args)
    {
    struct kvmppc_spapr_tce_table *stt = core::ptr::null_mut();
    struct kvmppc_spapr_tce_table *siter;
    struct mm_struct *mm = kvm.mm;
    unsigned long npages;
    int ret;
    if (!args.size || args.page_shift < 12 || args.page_shift > 34 ||
    (args.offset + args.size > (ULLONG_MAX >> args.page_shift)))
    return -EINVAL;
    npages = kvmppc_tce_pages(args.size);
    ret = account_locked_vm(mm, kvmppc_stt_pages(npages), true);
    if (ret)
    return ret;
    ret = -ENOMEM;
    stt = kzalloc_flex(*stt, pages, npages, GFP_KERNEL | __GFP_NOWARN);
    if (!stt)
    goto fail_acct;
    stt.liobn = args.liobn;
    stt.page_shift = args.page_shift;
    stt.offset = args.offset;
    stt.size = args.size;
    stt.kvm = kvm;
    mutex_init(&stt.alloc_lock);
    INIT_LIST_HEAD_RCU(&stt.iommu_tables);
    mutex_lock(&kvm.lock);
// Check this LIOBN hasn't been previously allocated
    ret = 0;
    list_for_each_entry(siter, &kvm.arch.spapr_tce_tables, list) {
    if (siter.liobn == args.liobn) {
    ret = -EBUSY;
    break;
    }
    }
    kvm_get_kvm(kvm);
    if (!ret)
    ret = anon_inode_getfd("kvm-spapr-tce", &kvm_spapr_tce_fops,
    stt, O_RDWR | O_CLOEXEC);
    if (ret >= 0)
    list_add_rcu(&stt.list, &kvm.arch.spapr_tce_tables);
    else
    kvm_put_kvm_no_destroy(kvm);
    mutex_unlock(&kvm.lock);
    if (ret >= 0)
    return ret;
    kfree(stt);
    fail_acct:
    account_locked_vm(mm, kvmppc_stt_pages(npages), false);
    return ret;
    }
    static long kvmppc_tce_to_ua(struct kvm *kvm, unsigned long tce,
    unsigned long *ua)
    {
    let mut gfn: c_ulong = tce >> PAGE_SHIFT;
    struct kvm_memory_slot *memslot;
    memslot = __gfn_to_memslot(kvm_memslots(kvm), gfn);
    if (!memslot)
    return -EINVAL;
// ua = __gfn_to_hva_memslot(memslot, gfn) |
    (tce & ~(PAGE_MASK | TCE_PCI_READ | TCE_PCI_WRITE));
    return 0;
    }
    static long kvmppc_tce_validate(struct kvmppc_spapr_tce_table *stt,
    unsigned long tce)
    {
    let mut gpa: c_ulong = tce & ~(TCE_PCI_READ | TCE_PCI_WRITE);
    let mut dir: enum dma_data_direction = iommu_tce_direction(tce);
    struct kvmppc_spapr_tce_iommu_table *stit;
    let mut ua: c_ulong = 0;
// Allow userspace to poison TCE table
    if (dir == DMA_NONE)
    return H_SUCCESS;
    if (iommu_tce_check_gpa(stt.page_shift, gpa))
    return H_TOO_HARD;
    if (kvmppc_tce_to_ua(stt.kvm, tce, &ua))
    return H_TOO_HARD;
    rcu_read_lock();
    list_for_each_entry_rcu(stit, &stt.iommu_tables, next) {
    let mut hpa: c_ulong = 0;
    struct mm_iommu_table_group_mem_t *mem;
    let mut shift: c_long = stit.tbl.it_page_shift;
    mem = mm_iommu_lookup(stt.kvm.mm, ua, 1ULL << shift);
    if (!mem || mm_iommu_ua_to_hpa(mem, ua, shift, &hpa)) {
    rcu_read_unlock();
    return H_TOO_HARD;
    }
    }
    rcu_read_unlock();
    return H_SUCCESS;
    }
//
// Handles TCE requests for emulated devices.
// Puts guest TCE values to the table and expects user space to convert them.
// Cannot fail so kvmppc_tce_validate must be called before it.
//
    static void kvmppc_tce_put(struct kvmppc_spapr_tce_table *stt,
    unsigned long idx, unsigned long tce)
    {
    struct page *page;
    u64 *tbl;
    unsigned long sttpage;
    idx -= stt.offset;
    sttpage = idx / TCES_PER_PAGE;
    page = stt.pages[sttpage];
    if (!page) {
// We allow any TCE, not just with read|write permissions
    if (!tce)
    return;
    page = kvm_spapr_get_tce_page(stt, sttpage);
    if (!page)
    return;
    }
    tbl = page_to_virt(page);
    tbl[idx % TCES_PER_PAGE] = tce;
    }
    static void kvmppc_clear_tce(struct mm_struct *mm, struct kvmppc_spapr_tce_table *stt,
    struct iommu_table *tbl, unsigned long entry)
    {
    unsigned long i;
    let mut subpages: c_ulong = 1ULL << (stt.page_shift - tbl.it_page_shift);
    let mut io_entry: c_ulong = entry << (stt.page_shift - tbl.it_page_shift);
    for (i = 0; i < subpages; ++i) {
    let mut hpa: c_ulong = 0;
    let mut dir: enum dma_data_direction = DMA_NONE;
    iommu_tce_xchg_no_kill(mm, tbl, io_entry + i, &hpa, &dir);
    }
    }
    static long kvmppc_tce_iommu_mapped_dec(struct kvm *kvm,
    struct iommu_table *tbl, unsigned long entry)
    {
    struct mm_iommu_table_group_mem_t *mem = core::ptr::null_mut();
    let mut pgsize: c_ulong = 1ULL << tbl.it_page_shift;
    __be64 *pua = IOMMU_TABLE_USERSPACE_ENTRY_RO(tbl, entry);
    if (!pua)
    return H_SUCCESS;
    mem = mm_iommu_lookup(kvm.mm, be64_to_cpu(*pua), pgsize);
    if (!mem)
    return H_TOO_HARD;
    mm_iommu_mapped_dec(mem);
// pua = cpu_to_be64(0);
    return H_SUCCESS;
    }
    static long kvmppc_tce_iommu_do_unmap(struct kvm *kvm,
    struct iommu_table *tbl, unsigned long entry)
    {
    let mut dir: enum dma_data_direction = DMA_NONE;
    let mut hpa: c_ulong = 0;
    long ret;
    if (WARN_ON_ONCE(iommu_tce_xchg_no_kill(kvm.mm, tbl, entry, &hpa,
    &dir)))
    return H_TOO_HARD;
    if (dir == DMA_NONE)
    return H_SUCCESS;
    ret = kvmppc_tce_iommu_mapped_dec(kvm, tbl, entry);
    if (ret != H_SUCCESS)
    iommu_tce_xchg_no_kill(kvm.mm, tbl, entry, &hpa, &dir);
    return ret;
    }
    static long kvmppc_tce_iommu_unmap(struct kvm *kvm,
    struct kvmppc_spapr_tce_table *stt, struct iommu_table *tbl,
    unsigned long entry)
    {
    unsigned long i, ret = H_SUCCESS;
    let mut subpages: c_ulong = 1ULL << (stt.page_shift - tbl.it_page_shift);
    let mut io_entry: c_ulong = entry * subpages;
    for (i = 0; i < subpages; ++i) {
    ret = kvmppc_tce_iommu_do_unmap(kvm, tbl, io_entry + i);
    if (ret != H_SUCCESS)
    break;
    }
    iommu_tce_kill(tbl, io_entry, subpages);
    return ret;
    }
    static long kvmppc_tce_iommu_do_map(struct kvm *kvm, struct iommu_table *tbl,
    unsigned long entry, unsigned long ua,
    enum dma_data_direction dir)
    {
    long ret;
    unsigned long hpa;
    __be64 *pua = IOMMU_TABLE_USERSPACE_ENTRY(tbl, entry);
    struct mm_iommu_table_group_mem_t *mem;
    if (!pua)
// it_userspace allocation might be delayed
    return H_TOO_HARD;
    mem = mm_iommu_lookup(kvm.mm, ua, 1ULL << tbl.it_page_shift);
    if (!mem)
// This only handles v2 IOMMU type, v1 is handled via ioctl()
    return H_TOO_HARD;
    if (WARN_ON_ONCE(mm_iommu_ua_to_hpa(mem, ua, tbl.it_page_shift, &hpa)))
    return H_TOO_HARD;
    if (mm_iommu_mapped_inc(mem))
    return H_TOO_HARD;
    ret = iommu_tce_xchg_no_kill(kvm.mm, tbl, entry, &hpa, &dir);
    if (WARN_ON_ONCE(ret)) {
    mm_iommu_mapped_dec(mem);
    return H_TOO_HARD;
    }
    if (dir != DMA_NONE)
    kvmppc_tce_iommu_mapped_dec(kvm, tbl, entry);
// pua = cpu_to_be64(ua);
    return 0;
    }
    static long kvmppc_tce_iommu_map(struct kvm *kvm,
    struct kvmppc_spapr_tce_table *stt, struct iommu_table *tbl,
    unsigned long entry, unsigned long ua,
    enum dma_data_direction dir)
    {
    unsigned long i, pgoff, ret = H_SUCCESS;
    let mut subpages: c_ulong = 1ULL << (stt.page_shift - tbl.it_page_shift);
    let mut io_entry: c_ulong = entry * subpages;
    for (i = 0, pgoff = 0; i < subpages;
    ++i, pgoff += IOMMU_PAGE_SIZE(tbl)) {
    ret = kvmppc_tce_iommu_do_map(kvm, tbl,
    io_entry + i, ua + pgoff, dir);
    if (ret != H_SUCCESS)
    break;
    }
    iommu_tce_kill(tbl, io_entry, subpages);
    return ret;
    }
    long kvmppc_h_put_tce(struct kvm_vcpu *vcpu, unsigned long liobn,
    unsigned long ioba, unsigned long tce)
    {
    struct kvmppc_spapr_tce_table *stt;
    long ret, idx;
    struct kvmppc_spapr_tce_iommu_table *stit;
    unsigned long entry, ua = 0;
    enum dma_data_direction dir;
// udbg_printf("H_PUT_TCE(): liobn=0x%lx ioba=0x%lx, tce=0x%lx\n",
// liobn, ioba, tce);
    stt = kvmppc_find_table(vcpu.kvm, liobn);
    if (!stt)
    return H_TOO_HARD;
    ret = kvmppc_ioba_validate(stt, ioba, 1);
    if (ret != H_SUCCESS)
    return ret;
    idx = srcu_read_lock(&vcpu.kvm.srcu);
    ret = kvmppc_tce_validate(stt, tce);
    if (ret != H_SUCCESS)
    goto unlock_exit;
    dir = iommu_tce_direction(tce);
    if ((dir != DMA_NONE) && kvmppc_tce_to_ua(vcpu.kvm, tce, &ua)) {
    ret = H_PARAMETER;
    goto unlock_exit;
    }
    entry = ioba >> stt.page_shift;
    list_for_each_entry_lockless(stit, &stt.iommu_tables, next) {
    if (dir == DMA_NONE)
    ret = kvmppc_tce_iommu_unmap(vcpu.kvm, stt,
    stit.tbl, entry);
    else
    ret = kvmppc_tce_iommu_map(vcpu.kvm, stt, stit.tbl,
    entry, ua, dir);
    if (ret != H_SUCCESS) {
    kvmppc_clear_tce(vcpu.kvm.mm, stt, stit.tbl, entry);
    goto unlock_exit;
    }
    }
    kvmppc_tce_put(stt, entry, tce);
    unlock_exit:
    srcu_read_unlock(&vcpu.kvm.srcu, idx);
    return ret;
    }
    EXPORT_SYMBOL_GPL(kvmppc_h_put_tce);
    long kvmppc_h_put_tce_indirect(struct kvm_vcpu *vcpu,
    unsigned long liobn, unsigned long ioba,
    unsigned long tce_list, unsigned long npages)
    {
    struct kvmppc_spapr_tce_table *stt;
    long i, ret = H_SUCCESS, idx;
    unsigned long entry, ua = 0;
    u64 __user *tces;
    u64 tce;
    struct kvmppc_spapr_tce_iommu_table *stit;
    stt = kvmppc_find_table(vcpu.kvm, liobn);
    if (!stt)
    return H_TOO_HARD;
    entry = ioba >> stt.page_shift;
//
// SPAPR spec says that the maximum size of the list is 512 TCEs
// so the whole table fits in 4K page
//
    if (npages > 512)
    return H_PARAMETER;
    if (tce_list & (SZ_4K - 1))
    return H_PARAMETER;
    ret = kvmppc_ioba_validate(stt, ioba, npages);
    if (ret != H_SUCCESS)
    return ret;
    idx = srcu_read_lock(&vcpu.kvm.srcu);
    if (kvmppc_tce_to_ua(vcpu.kvm, tce_list, &ua)) {
    ret = H_TOO_HARD;
    goto unlock_exit;
    }
    tces = (u64 __user *) ua;
    for (i = 0; i < npages; ++i) {
    if (get_user(tce, tces + i)) {
    ret = H_TOO_HARD;
    goto unlock_exit;
    }
    tce = be64_to_cpu(tce);
    ret = kvmppc_tce_validate(stt, tce);
    if (ret != H_SUCCESS)
    goto unlock_exit;
    }
    for (i = 0; i < npages; ++i) {
//
// This looks unsafe, because we validate, then regrab
// the TCE from userspace which could have been changed by
// another thread.
//
// But it actually is safe, because the relevant checks will be
// re-executed in the following code.  If userspace tries to
// change this dodgily it will result in a messier failure mode
// but won't threaten the host.
//
    if (get_user(tce, tces + i)) {
    ret = H_TOO_HARD;
    goto unlock_exit;
    }
    tce = be64_to_cpu(tce);
    if (kvmppc_tce_to_ua(vcpu.kvm, tce, &ua)) {
    ret = H_PARAMETER;
    goto unlock_exit;
    }
    list_for_each_entry_lockless(stit, &stt.iommu_tables, next) {
    ret = kvmppc_tce_iommu_map(vcpu.kvm, stt,
    stit.tbl, entry + i, ua,
    iommu_tce_direction(tce));
    if (ret != H_SUCCESS) {
    kvmppc_clear_tce(vcpu.kvm.mm, stt, stit.tbl,
    entry + i);
    goto unlock_exit;
    }
    }
    kvmppc_tce_put(stt, entry + i, tce);
    }
    unlock_exit:
    srcu_read_unlock(&vcpu.kvm.srcu, idx);
    return ret;
    }
    EXPORT_SYMBOL_GPL(kvmppc_h_put_tce_indirect);
    long kvmppc_h_stuff_tce(struct kvm_vcpu *vcpu,
    unsigned long liobn, unsigned long ioba,
    unsigned long tce_value, unsigned long npages)
    {
    struct kvmppc_spapr_tce_table *stt;
    long i, ret;
    struct kvmppc_spapr_tce_iommu_table *stit;
    stt = kvmppc_find_table(vcpu.kvm, liobn);
    if (!stt)
    return H_TOO_HARD;
    ret = kvmppc_ioba_validate(stt, ioba, npages);
    if (ret != H_SUCCESS)
    return ret;
// Check permission bits only to allow userspace poison TCE for debug
    if (tce_value & (TCE_PCI_WRITE | TCE_PCI_READ))
    return H_PARAMETER;
    list_for_each_entry_lockless(stit, &stt.iommu_tables, next) {
    let mut entry: c_ulong = ioba >> stt.page_shift;
    for (i = 0; i < npages; ++i) {
    ret = kvmppc_tce_iommu_unmap(vcpu.kvm, stt,
    stit.tbl, entry + i);
    if (ret == H_SUCCESS)
    continue;
    if (ret == H_TOO_HARD)
    return ret;
    WARN_ON_ONCE(1);
    kvmppc_clear_tce(vcpu.kvm.mm, stt, stit.tbl, entry + i);
    }
    }
    for (i = 0; i < npages; ++i, ioba += (1ULL << stt.page_shift))
    kvmppc_tce_put(stt, ioba >> stt.page_shift, tce_value);
    return ret;
    }
    EXPORT_SYMBOL_GPL(kvmppc_h_stuff_tce);
    long kvmppc_h_get_tce(struct kvm_vcpu *vcpu, unsigned long liobn,
    unsigned long ioba)
    {
    struct kvmppc_spapr_tce_table *stt;
    long ret;
    unsigned long idx;
    struct page *page;
    u64 *tbl;
    stt = kvmppc_find_table(vcpu.kvm, liobn);
    if (!stt)
    return H_TOO_HARD;
    ret = kvmppc_ioba_validate(stt, ioba, 1);
    if (ret != H_SUCCESS)
    return ret;
    idx = (ioba >> stt.page_shift) - stt.offset;
    page = stt.pages[idx / TCES_PER_PAGE];
    if (!page) {
    kvmppc_set_gpr(vcpu, 4, 0);
    return H_SUCCESS;
    }
    tbl = (u64 *)page_address(page);
    kvmppc_set_gpr(vcpu, 4, tbl[idx % TCES_PER_PAGE]);
    return H_SUCCESS;
    }
    EXPORT_SYMBOL_GPL(kvmppc_h_get_tce);
