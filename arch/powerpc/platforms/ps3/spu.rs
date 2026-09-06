//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/spu.c
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
// PS3 Platform spu routines.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
//

// spu_management_ops
//
// enum spe_type - Type of spe to create.
// @SPE_TYPE_LOGICAL: Standard logical spe.
//
// For use with lv1_construct_logical_spe().  The current HV does not support
// any types other than those listed.
//
    enum spe_type {
    SPE_TYPE_LOGICAL = 0,
    };
//
// struct spe_shadow - logical spe shadow register area.
//
// Read-only shadow of spe registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spe_shadow {
    pub padding_0140: [u8; 0x0140],
    pub /: *mut *mut u64 int_status_class0_RW; / 0x0140,
    pub /: *mut *mut u64 int_status_class1_RW; / 0x0148,
    pub /: *mut *mut u64 int_status_class2_RW; / 0x0150,
    pub padding_0158: [u8; 0x0610-0x0158],
    pub /: *mut *mut u64 mfc_dsisr_RW; / 0x0610,
    pub padding_0618: [u8; 0x0620-0x0618],
    pub /: *mut *mut u64 mfc_dar_RW; / 0x0620,
    pub padding_0628: [u8; 0x0800-0x0628],
    pub /: *mut *mut u64 mfc_dsipr_R; / 0x0800,
    pub padding_0808: [u8; 0x0810-0x0808],
    pub /: *mut *mut u64 mfc_lscrr_R; / 0x0810,
    pub padding_0818: [u8; 0x0c00-0x0818],
    pub /: *mut *mut u64 mfc_cer_R; / 0x0c00,
    pub padding_0c08: [u8; 0x0f00-0x0c08],
    pub /: *mut *mut u64 spe_execution_status; / 0x0f00,
    pub padding_0f08: [u8; 0x1000-0x0f08],
}

//
// enum spe_ex_state - Logical spe execution state.
// @SPE_EX_STATE_UNEXECUTABLE: Uninitialized.
// @SPE_EX_STATE_EXECUTABLE: Enabled, not ready.
// @SPE_EX_STATE_EXECUTED: Ready for use.
//
// The execution state (status) of the logical spe as reported in
// struct spe_shadow:spe_execution_status.
//
    enum spe_ex_state {
    SPE_EX_STATE_UNEXECUTABLE = 0,
    SPE_EX_STATE_EXECUTABLE = 2,
    SPE_EX_STATE_EXECUTED = 3,
    };
//
// struct priv1_cache - Cached values of priv1 registers.
// @masks[]: Array of cached spe interrupt masks, indexed by class.
// @sr1: Cached mfc_sr1 register.
// @tclass_id: Cached mfc_tclass_id register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv1_cache {
    pub masks: [u64; 3],
    pub sr1: u64,
    pub tclass_id: u64,
}

//
// struct spu_pdata - Platform state variables.
// @spe_id: HV spe id returned by lv1_construct_logical_spe().
// @resource_id: HV spe resource id returned by
// ps3_repository_read_spe_resource_id().
// @priv2_addr: lpar address of spe priv2 area returned by
// lv1_construct_logical_spe().
// @shadow_addr: lpar address of spe register shadow area returned by
// lv1_construct_logical_spe().
// @shadow: Virtual (ioremap) address of spe register shadow area.
// @cache: Cached values of priv1 registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_pdata {
    pub spe_id: u64,
    pub resource_id: u64,
    pub priv2_addr: u64,
    pub shadow_addr: u64,
    pub shadow: *mut spe_shadow __iomem,
    pub cache: priv1_cache,
}

    static struct spu_pdata *spu_pdata(struct spu *spu)
    {
    return spu.pdata;
    }

    _dump_areas(_a, _b, _c, _d, _e, __func__, __LINE__)
    static void _dump_areas(unsigned int spe_id, unsigned long priv2,
    unsigned long problem, unsigned long ls, unsigned long shadow,
    const char* func, int line)
    {
    pr_debug("%s:%d: spe_id:  %xh (%u)\n", func, line, spe_id, spe_id);
    pr_debug("%s:%d: priv2:   %lxh\n", func, line, priv2);
    pr_debug("%s:%d: problem: %lxh\n", func, line, problem);
    pr_debug("%s:%d: ls:      %lxh\n", func, line, ls);
    pr_debug("%s:%d: shadow:  %lxh\n", func, line, shadow);
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_get_spe_id(arg: *mut c_void) -> u64 {
    u64 ps3_get_spe_id(void *arg)
    {
    return spu_pdata(arg).spe_id;
    }
    EXPORT_SYMBOL_GPL(ps3_get_spe_id);
#[no_mangle]
unsafe extern "C" fn get_vas_id() -> unsigned long __init {
    static unsigned long __init get_vas_id(void)
    {
    u64 id;
    lv1_get_logical_ppe_id(&id);
    lv1_get_virtual_address_space_id_of_ppe(&id);
    return id;
    }
#[no_mangle]
unsafe extern "C" fn construct_spu(spu: *mut spu) -> int __init {
    static int __init construct_spu(struct spu *spu)
    {
    int result;
    u64 unused;
    u64 problem_phys;
    u64 local_store_phys;
    result = lv1_construct_logical_spe(PAGE_SHIFT, PAGE_SHIFT, PAGE_SHIFT,
    PAGE_SHIFT, PAGE_SHIFT, get_vas_id(), SPE_TYPE_LOGICAL,
    &spu_pdata(spu).priv2_addr, &problem_phys,
    &local_store_phys, &unused,
    &spu_pdata(spu).shadow_addr,
    &spu_pdata(spu).spe_id);
    spu.problem_phys = problem_phys;
    spu.local_store_phys = local_store_phys;
    if (result) {
    pr_debug("%s:%d: lv1_construct_logical_spe failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn spu_unmap(spu: *mut spu) {
    static void spu_unmap(struct spu *spu)
    {
    iounmap(spu.priv2);
    iounmap(spu.problem);
    iounmap(( u8 __iomem *)spu.local_store);
    iounmap(spu_pdata(spu).shadow);
    }
//
// setup_areas - Map the spu regions into the address space.
//
// The current HV requires the spu shadow regs to be mapped with the
// PTE page protection bits set as read-only.
//
// Returns: %0 on success or -errno on error.
//
#[no_mangle]
unsafe extern "C" fn setup_areas(spu: *mut spu) -> int __init {
    static int __init setup_areas(struct spu *spu)
    {
    spu_pdata(spu).shadow = ioremap_prot(spu_pdata(spu).shadow_addr,
    sizeof(struct spe_shadow),
    pgprot_noncached_wc(PAGE_KERNEL_RO));
    if (!spu_pdata(spu).shadow) {
    pr_debug("%s:%d: ioremap shadow failed\n", __func__, __LINE__);
    goto fail_ioremap;
    }
    spu.local_store = ( void *)ioremap_wc(spu.local_store_phys, LS_SIZE);
    if (!spu.local_store) {
    pr_debug("%s:%d: ioremap local_store failed\n",
    __func__, __LINE__);
    goto fail_ioremap;
    }
    spu.problem = ioremap(spu.problem_phys,
    sizeof(struct spu_problem));
    if (!spu.problem) {
    pr_debug("%s:%d: ioremap problem failed\n", __func__, __LINE__);
    goto fail_ioremap;
    }
    spu.priv2 = ioremap(spu_pdata(spu).priv2_addr,
    sizeof(struct spu_priv2));
    if (!spu.priv2) {
    pr_debug("%s:%d: ioremap priv2 failed\n", __func__, __LINE__);
    goto fail_ioremap;
    }
    dump_areas(spu_pdata(spu).spe_id, spu_pdata(spu).priv2_addr,
    spu.problem_phys, spu.local_store_phys,
    spu_pdata(spu).shadow_addr);
    dump_areas(spu_pdata(spu).spe_id, (unsigned long)spu.priv2,
    (unsigned long)spu.problem, (unsigned long)spu.local_store,
    (unsigned long)spu_pdata(spu).shadow);
    return 0;
    fail_ioremap:
    spu_unmap(spu);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn setup_interrupts(spu: *mut spu) -> int __init {
    static int __init setup_interrupts(struct spu *spu)
    {
    int result;
    result = ps3_spe_irq_setup(PS3_BINDING_CPU_ANY, spu_pdata(spu).spe_id,
    0, &spu.irqs[0]);
    if (result)
    goto fail_alloc_0;
    result = ps3_spe_irq_setup(PS3_BINDING_CPU_ANY, spu_pdata(spu).spe_id,
    1, &spu.irqs[1]);
    if (result)
    goto fail_alloc_1;
    result = ps3_spe_irq_setup(PS3_BINDING_CPU_ANY, spu_pdata(spu).spe_id,
    2, &spu.irqs[2]);
    if (result)
    goto fail_alloc_2;
    return result;
    fail_alloc_2:
    ps3_spe_irq_destroy(spu.irqs[1]);
    fail_alloc_1:
    ps3_spe_irq_destroy(spu.irqs[0]);
    fail_alloc_0:
    spu.irqs[0] = spu.irqs[1] = spu.irqs[2] = 0;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn enable_spu(spu: *mut spu) -> int __init {
    static int __init enable_spu(struct spu *spu)
    {
    int result;
    result = lv1_enable_logical_spe(spu_pdata(spu).spe_id,
    spu_pdata(spu).resource_id);
    if (result) {
    pr_debug("%s:%d: lv1_enable_logical_spe failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    goto fail_enable;
    }
    result = setup_areas(spu);
    if (result)
    goto fail_areas;
    result = setup_interrupts(spu);
    if (result)
    goto fail_interrupts;
    return 0;
    fail_interrupts:
    spu_unmap(spu);
    fail_areas:
    lv1_disable_logical_spe(spu_pdata(spu).spe_id, 0);
    fail_enable:
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ps3_destroy_spu(spu: *mut spu) -> c_int {
    static int ps3_destroy_spu(struct spu *spu)
    {
    int result;
    pr_debug("%s:%d spu_%d\n", __func__, __LINE__, spu.number);
    result = lv1_disable_logical_spe(spu_pdata(spu).spe_id, 0);
    BUG_ON(result);
    ps3_spe_irq_destroy(spu.irqs[2]);
    ps3_spe_irq_destroy(spu.irqs[1]);
    ps3_spe_irq_destroy(spu.irqs[0]);
    spu.irqs[0] = spu.irqs[1] = spu.irqs[2] = 0;
    spu_unmap(spu);
    result = lv1_destruct_logical_spe(spu_pdata(spu).spe_id);
    BUG_ON(result);
    kfree(spu.pdata);
    spu.pdata = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps3_create_spu(spu: *mut spu, data: *mut c_void) -> int __init {
    static int __init ps3_create_spu(struct spu *spu, void *data)
    {
    int result;
    pr_debug("%s:%d spu_%d\n", __func__, __LINE__, spu.number);
    spu.pdata = kzalloc_obj(struct spu_pdata);
    if (!spu.pdata) {
    result = -ENOMEM;
    goto fail_malloc;
    }
    spu_pdata(spu).resource_id = (unsigned long)data;
// Init cached reg values to HV defaults.
    spu_pdata(spu).cache.sr1 = 0x33;
    result = construct_spu(spu);
    if (result)
    goto fail_construct;
// For now, just go ahead and enable it.
    result = enable_spu(spu);
    if (result)
    goto fail_enable;
    while (in_be64(&spu_pdata(spu).shadow.spe_execution_status) !=
    SPE_EX_STATE_EXECUTED)
    cpu_relax();
    return result;
    fail_enable:
    fail_construct:
    ps3_destroy_spu(spu);
    fail_malloc:
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ps3_enumerate_spus(data): *mut *mut int (fn)(void) -> int __init {
    static int __init ps3_enumerate_spus(int (*fn)(void *data))
    {
    int result;
    unsigned int num_resource_id;
    unsigned int i;
    result = ps3_repository_read_num_spu_resource_id(&num_resource_id);
    pr_debug("%s:%d: num_resource_id %u\n", __func__, __LINE__,
    num_resource_id);
//
// For now, just create logical spus equal to the number
// of physical spus reserved for the partition.
//
    for (i = 0; i < num_resource_id; i++) {
    enum ps3_spu_resource_type resource_type;
    unsigned int resource_id;
    result = ps3_repository_read_spu_resource_id(i,
    &resource_type, &resource_id);
    if (result)
    break;
    if (resource_type == PS3_SPU_RESOURCE_TYPE_EXCLUSIVE) {
    result = fn((void*)(unsigned long)resource_id);
    if (result)
    break;
    }
    }
    if (result) {
    printk(KERN_WARNING "%s:%d: Error initializing spus\n",
    __func__, __LINE__);
    return result;
    }
    return num_resource_id;
    }
#[no_mangle]
unsafe extern "C" fn ps3_init_affinity() -> c_int {
    static int ps3_init_affinity(void)
    {
    return 0;
    }
//
// ps3_enable_spu - Enable SPU run control.
//
// An outstanding enhancement for the PS3 would be to add a guard to check
// for incorrect access to the spu problem state when the spu context is
// disabled.  This check could be implemented with a flag added to the spu
// context that would inhibit mapping problem state pages, and a routine
// to unmap spu problem state pages.  When the spu is enabled with
// ps3_enable_spu() the flag would be set allowing pages to be mapped,
// and when the spu is disabled with ps3_disable_spu() the flag would be
// cleared and the mapped problem state pages would be unmapped.
//
#[no_mangle]
unsafe extern "C" fn ps3_enable_spu(ctx: *mut spu_context) {
    static void ps3_enable_spu(struct spu_context *ctx)
    {
    }
#[no_mangle]
unsafe extern "C" fn ps3_disable_spu(ctx: *mut spu_context) {
    static void ps3_disable_spu(struct spu_context *ctx)
    {
    ctx.ops.runcntl_stop(ctx);
    }
    static const struct spu_management_ops spu_management_ps3_ops = {
    .enumerate_spus = ps3_enumerate_spus,
    .create_spu = ps3_create_spu,
    .destroy_spu = ps3_destroy_spu,
    .enable_spu = ps3_enable_spu,
    .disable_spu = ps3_disable_spu,
    .init_affinity = ps3_init_affinity,
    };
// spu_priv1_ops
#[no_mangle]
unsafe extern "C" fn int_mask_and(spu: *mut spu, class: c_int, mask: u64) {
    static void int_mask_and(struct spu *spu, int class, u64 mask)
    {
    u64 old_mask;
// are these serialized by caller???
    old_mask = spu_int_mask_get(spu, class);
    spu_int_mask_set(spu, class, old_mask & mask);
    }
#[no_mangle]
unsafe extern "C" fn int_mask_or(spu: *mut spu, class: c_int, mask: u64) {
    static void int_mask_or(struct spu *spu, int class, u64 mask)
    {
    u64 old_mask;
    old_mask = spu_int_mask_get(spu, class);
    spu_int_mask_set(spu, class, old_mask | mask);
    }
#[no_mangle]
unsafe extern "C" fn int_mask_set(spu: *mut spu, class: c_int, mask: u64) {
    static void int_mask_set(struct spu *spu, int class, u64 mask)
    {
    spu_pdata(spu).cache.masks[class] = mask;
    lv1_set_spe_interrupt_mask(spu_pdata(spu).spe_id, class,
    spu_pdata(spu).cache.masks[class]);
    }
#[no_mangle]
unsafe extern "C" fn int_mask_get(spu: *mut spu, class: c_int) -> u64 {
    static u64 int_mask_get(struct spu *spu, int class)
    {
    return spu_pdata(spu).cache.masks[class];
    }
#[no_mangle]
unsafe extern "C" fn int_stat_clear(spu: *mut spu, class: c_int, stat: u64) {
    static void int_stat_clear(struct spu *spu, int class, u64 stat)
    {
// Note that MFC_DSISR will be cleared when class1[MF] is set.
    lv1_clear_spe_interrupt_status(spu_pdata(spu).spe_id, class,
    stat, 0);
    }
#[no_mangle]
unsafe extern "C" fn int_stat_get(spu: *mut spu, class: c_int) -> u64 {
    static u64 int_stat_get(struct spu *spu, int class)
    {
    u64 stat;
    lv1_get_spe_interrupt_status(spu_pdata(spu).spe_id, class, &stat);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn cpu_affinity_set(spu: *mut spu, cpu: c_int) {
    static void cpu_affinity_set(struct spu *spu, int cpu)
    {
// No support.
    }
#[no_mangle]
unsafe extern "C" fn mfc_dar_get(spu: *mut spu) -> u64 {
    static u64 mfc_dar_get(struct spu *spu)
    {
    return in_be64(&spu_pdata(spu).shadow.mfc_dar_RW);
    }
#[no_mangle]
unsafe extern "C" fn mfc_dsisr_set(spu: *mut spu, dsisr: u64) {
    static void mfc_dsisr_set(struct spu *spu, u64 dsisr)
    {
// Nothing to do, cleared in int_stat_clear().
    }
#[no_mangle]
unsafe extern "C" fn mfc_dsisr_get(spu: *mut spu) -> u64 {
    static u64 mfc_dsisr_get(struct spu *spu)
    {
    return in_be64(&spu_pdata(spu).shadow.mfc_dsisr_RW);
    }
#[no_mangle]
unsafe extern "C" fn mfc_sdr_setup(spu: *mut spu) {
    static void mfc_sdr_setup(struct spu *spu)
    {
// Nothing to do.
    }
#[no_mangle]
unsafe extern "C" fn mfc_sr1_set(spu: *mut spu, sr1: u64) {
    static void mfc_sr1_set(struct spu *spu, u64 sr1)
    {
// Check bits allowed by HV.
    static const u64 allowed = ~(MFC_STATE1_LOCAL_STORAGE_DECODE_MASK
    | MFC_STATE1_PROBLEM_STATE_MASK);
    BUG_ON((sr1 & allowed) != (spu_pdata(spu).cache.sr1 & allowed));
    spu_pdata(spu).cache.sr1 = sr1;
    lv1_set_spe_privilege_state_area_1_register(
    spu_pdata(spu).spe_id,
    offsetof(struct spu_priv1, mfc_sr1_RW),
    spu_pdata(spu).cache.sr1);
    }
#[no_mangle]
unsafe extern "C" fn mfc_sr1_get(spu: *mut spu) -> u64 {
    static u64 mfc_sr1_get(struct spu *spu)
    {
    return spu_pdata(spu).cache.sr1;
    }
#[no_mangle]
unsafe extern "C" fn mfc_tclass_id_set(spu: *mut spu, tclass_id: u64) {
    static void mfc_tclass_id_set(struct spu *spu, u64 tclass_id)
    {
    spu_pdata(spu).cache.tclass_id = tclass_id;
    lv1_set_spe_privilege_state_area_1_register(
    spu_pdata(spu).spe_id,
    offsetof(struct spu_priv1, mfc_tclass_id_RW),
    spu_pdata(spu).cache.tclass_id);
    }
#[no_mangle]
unsafe extern "C" fn mfc_tclass_id_get(spu: *mut spu) -> u64 {
    static u64 mfc_tclass_id_get(struct spu *spu)
    {
    return spu_pdata(spu).cache.tclass_id;
    }
#[no_mangle]
unsafe extern "C" fn tlb_invalidate(spu: *mut spu) {
    static void tlb_invalidate(struct spu *spu)
    {
// Nothing to do.
    }
#[no_mangle]
unsafe extern "C" fn resource_allocation_groupID_set(spu: *mut spu, id: u64) {
    static void resource_allocation_groupID_set(struct spu *spu, u64 id)
    {
// No support.
    }
#[no_mangle]
unsafe extern "C" fn resource_allocation_groupID_get(spu: *mut spu) -> u64 {
    static u64 resource_allocation_groupID_get(struct spu *spu)
    {
    return 0; /* No support. */
    }
#[no_mangle]
unsafe extern "C" fn resource_allocation_enable_set(spu: *mut spu, enable: u64) {
    static void resource_allocation_enable_set(struct spu *spu, u64 enable)
    {
// No support.
    }
#[no_mangle]
unsafe extern "C" fn resource_allocation_enable_get(spu: *mut spu) -> u64 {
    static u64 resource_allocation_enable_get(struct spu *spu)
    {
    return 0; /* No support. */
    }
    static const struct spu_priv1_ops spu_priv1_ps3_ops = {
    .int_mask_and = int_mask_and,
    .int_mask_or = int_mask_or,
    .int_mask_set = int_mask_set,
    .int_mask_get = int_mask_get,
    .int_stat_clear = int_stat_clear,
    .int_stat_get = int_stat_get,
    .cpu_affinity_set = cpu_affinity_set,
    .mfc_dar_get = mfc_dar_get,
    .mfc_dsisr_set = mfc_dsisr_set,
    .mfc_dsisr_get = mfc_dsisr_get,
    .mfc_sdr_setup = mfc_sdr_setup,
    .mfc_sr1_set = mfc_sr1_set,
    .mfc_sr1_get = mfc_sr1_get,
    .mfc_tclass_id_set = mfc_tclass_id_set,
    .mfc_tclass_id_get = mfc_tclass_id_get,
    .tlb_invalidate = tlb_invalidate,
    .resource_allocation_groupID_set = resource_allocation_groupID_set,
    .resource_allocation_groupID_get = resource_allocation_groupID_get,
    .resource_allocation_enable_set = resource_allocation_enable_set,
    .resource_allocation_enable_get = resource_allocation_enable_get,
    };
#[no_mangle]
pub unsafe extern "C" fn ps3_spu_set_platform() {
    void ps3_spu_set_platform(void)
    {
    spu_priv1_ops = &spu_priv1_ps3_ops;
    spu_management_ops = &spu_management_ps3_ops;
    }
