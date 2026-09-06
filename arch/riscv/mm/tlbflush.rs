//! Automatically rewritten from C to Rust
//! Source: arch/riscv/mm/tlbflush.c
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
// Flush entire TLB if number of entries to be flushed is greater
// than the threshold below.
//
    let mut __read_mostly: unsigned long tlb_flush_all_threshold = 64;
    static void local_flush_tlb_range_threshold_asid(unsigned long start,
    unsigned long size,
    unsigned long stride,
    unsigned long asid)
    {
    let mut nr_ptes_in_range: c_ulong = DIV_ROUND_UP(size, stride);
    int i;
    if (nr_ptes_in_range > tlb_flush_all_threshold) {
    local_flush_tlb_all_asid(asid);
    return;
    }
    if (has_svinval()) {
    local_sfence_w_inval();
    for (i = 0; i < nr_ptes_in_range; ++i) {
    local_sinval_vma(start, asid);
    start += stride;
    }
    local_sfence_inval_ir();
    return;
    }
    for (i = 0; i < nr_ptes_in_range; ++i) {
    local_flush_tlb_page_asid(start, asid);
    start += stride;
    }
    }
    static inline void local_flush_tlb_range_asid(unsigned long start,
    unsigned long size, unsigned long stride, unsigned long asid)
    {
    if (size <= stride)
    local_flush_tlb_page_asid(start, asid);
#[no_mangle]
pub unsafe extern "C" fn if(FLUSH_TLB_MAX_SIZE: size ==) -> else {
    else if (size == FLUSH_TLB_MAX_SIZE)
    local_flush_tlb_all_asid(asid);
    else
    local_flush_tlb_range_threshold_asid(start, size, stride, asid);
    }
// Flush a range of kernel pages without broadcasting
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    void local_flush_tlb_kernel_range(unsigned long start, unsigned long end)
    {
    local_flush_tlb_range_asid(start, end - start, PAGE_SIZE, FLUSH_TLB_NO_ASID);
    }
#[no_mangle]
unsafe extern "C" fn __ipi_flush_tlb_all(info: *mut c_void) {
    static void __ipi_flush_tlb_all(void *info)
    {
    local_flush_tlb_all();
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_all() {
    void flush_tlb_all(void)
    {
    if (num_online_cpus() < 2)
    local_flush_tlb_all();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: riscv_use_sbi_for_rfence()) -> else {
    else if (riscv_use_sbi_for_rfence())
    sbi_remote_sfence_vma_asid(core::ptr::null_mut(), 0, FLUSH_TLB_MAX_SIZE, FLUSH_TLB_NO_ASID);
    else
    on_each_cpu(__ipi_flush_tlb_all, core::ptr::null_mut(), 1);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flush_tlb_range_data {
    pub asid: c_ulong,
    pub start: c_ulong,
    pub size: c_ulong,
    pub stride: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn __ipi_flush_tlb_range_asid(info: *mut c_void) {
    static void __ipi_flush_tlb_range_asid(void *info)
    {
    struct flush_tlb_range_data *d = info;
    local_flush_tlb_range_asid(d.start, d.size, d.stride, d.asid);
    }
    static void __flush_tlb_range(struct mm_struct *mm,
    const struct cpumask *cmask,
    unsigned long start, unsigned long size,
    unsigned long stride)
    {
    let mut asid: c_ulong = get_mm_asid(mm);
    unsigned int cpu;
    if (cpumask_empty(cmask))
    return;
    cpu = get_cpu();
// Check if the TLB flush needs to be sent to other CPUs.
    if (cpumask_any_but(cmask, cpu) >= nr_cpu_ids) {
    local_flush_tlb_range_asid(start, size, stride, asid);
    } else if (riscv_use_sbi_for_rfence()) {
    sbi_remote_sfence_vma_asid(cmask, start, size, asid);
    } else {
    struct flush_tlb_range_data ftd;
    ftd.asid = asid;
    ftd.start = start;
    ftd.size = size;
    ftd.stride = stride;
    on_each_cpu_mask(cmask, __ipi_flush_tlb_range_asid, &ftd, 1);
    }
    put_cpu();
    if (mm)
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, start + size);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    void flush_tlb_mm(struct mm_struct *mm)
    {
    __flush_tlb_range(mm, mm_cpumask(mm), 0, FLUSH_TLB_MAX_SIZE, PAGE_SIZE);
    }
    void flush_tlb_mm_range(struct mm_struct *mm,
    unsigned long start, unsigned long end,
    unsigned int page_size)
    {
    __flush_tlb_range(mm, mm_cpumask(mm), start, end - start, page_size);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong) {
    void flush_tlb_page(struct vm_area_struct *vma, unsigned long addr)
    {
    __flush_tlb_range(vma.vm_mm, mm_cpumask(vma.vm_mm),
    addr, PAGE_SIZE, PAGE_SIZE);
    }
    void flush_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    unsigned long stride_size;
    if (!is_vm_hugetlb_page(vma)) {
    stride_size = PAGE_SIZE;
    } else {
    stride_size = huge_page_size(hstate_vma(vma));
//
// As stated in the privileged specification, every PTE in a
// NAPOT region must be invalidated, so reset the stride in that
// case.
//
    if (has_svnapot()) {
    if (stride_size >= PGDIR_SIZE)
    stride_size = PGDIR_SIZE;
#[no_mangle]
pub unsafe extern "C" fn if(P4D_SIZE: stride_size >=) -> else {
    else if (stride_size >= P4D_SIZE)
    stride_size = P4D_SIZE;
#[no_mangle]
pub unsafe extern "C" fn if(PUD_SIZE: stride_size >=) -> else {
    else if (stride_size >= PUD_SIZE)
    stride_size = PUD_SIZE;
#[no_mangle]
pub unsafe extern "C" fn if(PMD_SIZE: stride_size >=) -> else {
    else if (stride_size >= PMD_SIZE)
    stride_size = PMD_SIZE;
    else
    stride_size = PAGE_SIZE;
    }
    }
    __flush_tlb_range(vma.vm_mm, mm_cpumask(vma.vm_mm),
    start, end - start, stride_size);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    void flush_tlb_kernel_range(unsigned long start, unsigned long end)
    {
    __flush_tlb_range(core::ptr::null_mut(), cpu_online_mask,
    start, end - start, PAGE_SIZE);
    }

    void flush_pmd_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    __flush_tlb_range(vma.vm_mm, mm_cpumask(vma.vm_mm),
    start, end - start, PMD_SIZE);
    }
    void flush_pud_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    __flush_tlb_range(vma.vm_mm, mm_cpumask(vma.vm_mm),
    start, end - start, PUD_SIZE);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_tlbbatch_should_defer(mm: *mut mm_struct) -> bool {
    bool arch_tlbbatch_should_defer(struct mm_struct *mm)
    {
    return true;
    }
    void arch_tlbbatch_add_pending(struct arch_tlbflush_unmap_batch *batch,
    struct mm_struct *mm, unsigned long start, unsigned long end)
    {
    cpumask_or(&batch.cpumask, &batch.cpumask, mm_cpumask(mm));
    mmu_notifier_arch_invalidate_secondary_tlbs(mm, start, end);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_tlbbatch_flush(batch: *mut arch_tlbflush_unmap_batch) {
    void arch_tlbbatch_flush(struct arch_tlbflush_unmap_batch *batch)
    {
    __flush_tlb_range(core::ptr::null_mut(), &batch.cpumask,
    0, FLUSH_TLB_MAX_SIZE, PAGE_SIZE);
    cpumask_clear(&batch.cpumask);
    }
