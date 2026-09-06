//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/vgic/vgic-debug.c
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
// Copyright (C) 2016 Linaro
// Author: Christoffer Dall <christoffer.dall@linaro.org>
//

//
// Structure to control looping through the entire vgic state.  We start at
// zero for each field and move upwards.  So, if dist_id is 0 we print the
// distributor info.  When dist_id is 1, we have already printed it and move
// on.
//
// When vcpu_id < nr_cpus we print the vcpu info until vcpu_id == nr_cpus and
// so on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_state_iter {
    pub nr_cpus: c_int,
    pub nr_spis: c_int,
    pub dist_id: c_int,
    pub vcpu_id: c_int,
    pub intid: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn iter_next(kvm: *mut kvm, iter: *mut vgic_state_iter) {
    static void iter_next(struct kvm *kvm, struct vgic_state_iter *iter)
    {
    struct vgic_dist *dist = &kvm.arch.vgic;
    if (iter.dist_id == 0) {
    iter.dist_id++;
    return;
    }
//
// Let the xarray drive the iterator after the last SPI, as the iterator
// has exhausted the sequentially-allocated INTID space.
//
    if (iter.intid >= (iter.nr_spis + VGIC_NR_PRIVATE_IRQS - 1)) {
    if (iter.intid == VGIC_LPI_MAX_INTID + 1)
    return;
    rcu_read_lock();
    if (!xa_find_after(&dist.lpi_xa, &iter.intid,
    VGIC_LPI_MAX_INTID, XA_PRESENT))
    iter.intid = VGIC_LPI_MAX_INTID + 1;
    rcu_read_unlock();
    return;
    }
    iter.intid++;
    if (iter.intid == VGIC_NR_PRIVATE_IRQS &&
    ++iter.vcpu_id < iter.nr_cpus)
    iter.intid = 0;
    }
#[no_mangle]
unsafe extern "C" fn vgic_count_lpis(kvm: *mut kvm) -> c_int {
    static int vgic_count_lpis(struct kvm *kvm)
    {
    struct vgic_dist *dist = &kvm.arch.vgic;
    struct vgic_irq *irq;
    unsigned long intid;
    let mut nr_lpis: c_int = 0;
    rcu_read_lock();
    xa_for_each(&dist.lpi_xa, intid, irq)
    nr_lpis++;
    rcu_read_unlock();
    return nr_lpis;
    }
    static void iter_init(struct kvm *kvm, struct vgic_state_iter *iter,
    loff_t pos)
    {
    let mut nr_cpus: c_int = atomic_read(&kvm.online_vcpus);
    memset(iter, 0, sizeof(*iter));
    iter.nr_cpus = nr_cpus;
    iter.nr_spis = kvm.arch.vgic.nr_spis;
// Fast forward to the right position if needed
    while (pos--)
    iter_next(kvm, iter);
    }
#[no_mangle]
unsafe extern "C" fn end_of_vgic(iter: *mut vgic_state_iter) -> bool {
    static bool end_of_vgic(struct vgic_state_iter *iter)
    {
    return iter.dist_id > 0 &&
    iter.vcpu_id == iter.nr_cpus &&
    iter.intid >= (iter.nr_spis + VGIC_NR_PRIVATE_IRQS) &&
    iter.intid > VGIC_LPI_MAX_INTID;
    }
    static void *vgic_debug_start(struct seq_file *s, loff_t *pos)
    {
    struct kvm *kvm = s.private;
    struct vgic_state_iter *iter;
    iter = kmalloc_obj(*iter);
    if (!iter)
    return ERR_PTR(-ENOMEM);
    iter_init(kvm, iter, *pos);
    if (end_of_vgic(iter)) {
    kfree(iter);
    iter = core::ptr::null_mut();
    }
    return iter;
    }
    static void *vgic_debug_next(struct seq_file *s, void *v, loff_t *pos)
    {
    struct kvm *kvm = s.private;
    struct vgic_state_iter *iter = v;
    ++*pos;
    iter_next(kvm, iter);
    if (end_of_vgic(iter)) {
    kfree(iter);
    iter = core::ptr::null_mut();
    }
    return iter;
    }
#[no_mangle]
unsafe extern "C" fn vgic_debug_stop(s: *mut seq_file, v: *mut c_void) {
    static void vgic_debug_stop(struct seq_file *s, void *v)
    {
    struct vgic_state_iter *iter = v;
    if (IS_ERR_OR_NULL(v))
    return;
    kfree(iter);
    }
    static void print_dist_state(struct seq_file *s, struct vgic_dist *dist,
    struct vgic_state_iter *iter)
    {
    let mut v3: bool = dist.vgic_model == KVM_DEV_TYPE_ARM_VGIC_V3;
    struct kvm *kvm = s.private;
    seq_printf(s, "Distributor\n");
    seq_printf(s, "===========\n");
    seq_printf(s, "vgic_model:\t%s\n", v3 ? "GICv3" : "GICv2");
    seq_printf(s, "nr_spis:\t%d\n", dist.nr_spis);
    if (v3)
    seq_printf(s, "nr_lpis:\t%d\n", vgic_count_lpis(kvm));
    seq_printf(s, "enabled:\t%d\n", dist.enabled);
    seq_printf(s, "\n");
    seq_printf(s, "P=pending_latch, L=line_level, A=active\n");
    seq_printf(s, "E=enabled, H=hw, C=config (level=1, edge=0)\n");
    seq_printf(s, "G=group\n");
    }
    static void print_header(struct seq_file *s, struct vgic_irq *irq,
    struct kvm_vcpu *vcpu)
    {
    let mut id: c_int = 0;
    char *hdr = "SPI ";
    if (vcpu) {
    hdr = "VCPU";
    id = vcpu.vcpu_idx;
    }
    seq_printf(s, "\n");
    seq_printf(s, "%s%2d TYP   ID TGT_ID PLAEHCG     HWID   TARGET SRC PRI VCPU_ID\n", hdr, id);
    seq_printf(s, "----------------------------------------------------------------\n");
    }
    static void print_irq_state(struct seq_file *s, struct vgic_irq *irq,
    struct kvm_vcpu *vcpu)
    {
    char *type;
    bool pending;
    if (irq.intid < VGIC_NR_SGIS)
    type = "SGI";
#[no_mangle]
pub unsafe extern "C" fn if(VGIC_NR_PRIVATE_IRQS: irq->intid <) -> else {
    else if (irq.intid < VGIC_NR_PRIVATE_IRQS)
    type = "PPI";
#[no_mangle]
pub unsafe extern "C" fn if(VGIC_MAX_SPI: irq->intid <) -> else {
    else if (irq.intid < VGIC_MAX_SPI)
    type = "SPI";
    else
    type = "LPI";
    if (irq.intid ==0 || irq.intid == VGIC_NR_PRIVATE_IRQS)
    print_header(s, irq, vcpu);
    pending = irq.pending_latch;
    if (irq.hw && vgic_irq_is_sgi(irq.intid)) {
    int err;
    err = irq_get_irqchip_state(irq.host_irq,
    IRQCHIP_STATE_PENDING,
    &pending);
    WARN_ON_ONCE(err);
    }
    seq_printf(s, "       %s %4d "
    "    %2d "
    "%d%d%d%d%d%d%d "
    "%8d "
    "%8x "
    " %2x "
    "%3d "
    "     %2d "
    "\n",
    type, irq.intid,
    (irq.target_vcpu) ? irq.target_vcpu.vcpu_idx : -1,
    pending,
    irq.line_level,
    irq.active,
    irq.enabled,
    irq.hw,
    irq.config == VGIC_CONFIG_LEVEL,
    irq.group,
    irq.hwintid,
    irq.mpidr,
    irq.source,
    irq.priority,
    (irq.vcpu) ? irq.vcpu.vcpu_idx : -1);
    }
#[no_mangle]
unsafe extern "C" fn vgic_debug_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int vgic_debug_show(struct seq_file *s, void *v)
    {
    struct kvm *kvm = s.private;
    struct vgic_state_iter *iter = v;
    struct vgic_irq *irq;
    struct kvm_vcpu *vcpu = core::ptr::null_mut();
    unsigned long flags;
    if (iter.dist_id == 0) {
    print_dist_state(s, &kvm.arch.vgic, iter);
    return 0;
    }
    if (!kvm.arch.vgic.initialized)
    return 0;
    if (iter.vcpu_id < iter.nr_cpus)
    vcpu = kvm_get_vcpu(kvm, iter.vcpu_id);
    if (iter.intid < VGIC_NR_PRIVATE_IRQS)
    irq = vgic_get_vcpu_irq(vcpu, iter.intid);
    else
    irq = vgic_get_irq(kvm, iter.intid);
    if (!irq)
    return 0;
    raw_spin_lock_irqsave(&irq.irq_lock, flags);
    print_irq_state(s, irq, vcpu);
    raw_spin_unlock_irqrestore(&irq.irq_lock, flags);
    vgic_put_irq(kvm, irq);
    return 0;
    }
    static const struct seq_operations vgic_debug_sops = {
    .start = vgic_debug_start,
    .next  = vgic_debug_next,
    .stop  = vgic_debug_stop,
    .show  = vgic_debug_show
    };
    DEFINE_SEQ_ATTRIBUTE(vgic_debug);
#[no_mangle]
pub unsafe extern "C" fn vgic_debug_init(kvm: *mut kvm) {
    void vgic_debug_init(struct kvm *kvm)
    {
    debugfs_create_file("vgic-state", 0444, kvm.debugfs_dentry, kvm,
    &vgic_debug_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_debug_destroy(kvm: *mut kvm) {
    void vgic_debug_destroy(struct kvm *kvm)
    {
    }
//
// struct vgic_its_iter - Iterator for traversing VGIC ITS device tables.
// @dev: Pointer to the current its_device being processed.
// @ite: Pointer to the current its_ite within the device being processed.
//
// This structure is used to maintain the current position during iteration
// over the ITS device tables. It holds pointers to both the current device
// and the current ITE within that device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_its_iter {
    pub dev: *mut its_device,
    pub ite: *mut its_ite,
}

//
// end_of_iter - Checks if the iterator has reached the end.
// @iter: The iterator to check.
//
// When the iterator completed processing the final ITE in the last device
// table, it was marked to indicate the end of iteration by setting its
// device and ITE pointers to NULL.
// This function checks whether the iterator was marked as end.
//
// Return: True if the iterator is marked as end, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn end_of_iter(iter: *mut vgic_its_iter) -> bool {
    static inline bool end_of_iter(struct vgic_its_iter *iter)
    {
    return !iter.dev && !iter.ite;
    }
//
// vgic_its_iter_next - Advances the iterator to the next entry in the ITS tables.
// @its: The VGIC ITS structure.
// @iter: The iterator to advance.
//
// This function moves the iterator to the next ITE within the current device,
// or to the first ITE of the next device if the current ITE is the last in
// the device. If the current device is the last device, the iterator is set
// to indicate the end of iteration.
//
#[no_mangle]
unsafe extern "C" fn vgic_its_iter_next(its: *mut vgic_its, iter: *mut vgic_its_iter) {
    static void vgic_its_iter_next(struct vgic_its *its, struct vgic_its_iter *iter)
    {
    struct its_device *dev = iter.dev;
    struct its_ite *ite = iter.ite;
    if (!ite || list_is_last(&ite.ite_list, &dev.itt_head)) {
    if (list_is_last(&dev.dev_list, &its.device_list)) {
    dev = core::ptr::null_mut();
    ite = core::ptr::null_mut();
    } else {
    dev = list_next_entry(dev, dev_list);
    ite = list_first_entry_or_null(&dev.itt_head,
    struct its_ite,
    ite_list);
    }
    } else {
    ite = list_next_entry(ite, ite_list);
    }
    iter.dev = dev;
    iter.ite = ite;
    }
//
// vgic_its_debug_start - Start function for the seq_file interface.
// @s: The seq_file structure.
// @pos: The starting position (offset).
//
// This function initializes the iterator to the beginning of the ITS tables
// and advances it to the specified position. It acquires the its_lock mutex
// to protect shared data.
//
// Return: An iterator pointer on success, NULL if no devices are found or
// the end of the list is reached, or ERR_PTR(-ENOMEM) on memory
// allocation failure.
//
    static void *vgic_its_debug_start(struct seq_file *s, loff_t *pos)
    {
    struct vgic_its *its = s.private;
    struct vgic_its_iter *iter;
    struct its_device *dev;
    let mut offset: loff_t = *pos;
    mutex_lock(&its.its_lock);
    dev = list_first_entry_or_null(&its.device_list,
    struct its_device, dev_list);
    if (!dev)
    return core::ptr::null_mut();
    iter = kmalloc_obj(*iter);
    if (!iter)
    return ERR_PTR(-ENOMEM);
    iter.dev = dev;
    iter.ite = list_first_entry_or_null(&dev.itt_head,
    struct its_ite, ite_list);
    while (!end_of_iter(iter) && offset--)
    vgic_its_iter_next(its, iter);
    if (end_of_iter(iter)) {
    kfree(iter);
    return core::ptr::null_mut();
    }
    return iter;
    }
//
// vgic_its_debug_next - Next function for the seq_file interface.
// @s: The seq_file structure.
// @v: The current iterator.
// @pos: The current position (offset).
//
// This function advances the iterator to the next entry and increments the
// position.
//
// Return: An iterator pointer on success, or NULL if the end of the list is
// reached.
//
    static void *vgic_its_debug_next(struct seq_file *s, void *v, loff_t *pos)
    {
    struct vgic_its *its = s.private;
    struct vgic_its_iter *iter = v;
    ++*pos;
    vgic_its_iter_next(its, iter);
    if (end_of_iter(iter)) {
    kfree(iter);
    return core::ptr::null_mut();
    }
    return iter;
    }
//
// vgic_its_debug_stop - Stop function for the seq_file interface.
// @s: The seq_file structure.
// @v: The current iterator.
//
// This function frees the iterator and releases the its_lock mutex.
//
#[no_mangle]
unsafe extern "C" fn vgic_its_debug_stop(s: *mut seq_file, v: *mut c_void) {
    static void vgic_its_debug_stop(struct seq_file *s, void *v)
    {
    struct vgic_its *its = s.private;
    struct vgic_its_iter *iter = v;
    if (!IS_ERR_OR_NULL(iter))
    kfree(iter);
    mutex_unlock(&its.its_lock);
    }
//
// vgic_its_debug_show - Show function for the seq_file interface.
// @s: The seq_file structure.
// @v: The current iterator.
//
// This function formats and prints the ITS table entry information to the
// seq_file output.
//
// Return: 0 on success.
//
#[no_mangle]
unsafe extern "C" fn vgic_its_debug_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int vgic_its_debug_show(struct seq_file *s, void *v)
    {
    struct vgic_its_iter *iter = v;
    struct its_device *dev = iter.dev;
    struct its_ite *ite = iter.ite;
    if (!ite)
    return 0;
    if (list_is_first(&ite.ite_list, &dev.itt_head)) {
    seq_printf(s, "\n");
    seq_printf(s, "Device ID: 0x%x, Event ID Range: [0 - %llu]\n",
    dev.device_id, BIT_ULL(dev.num_eventid_bits) - 1);
    seq_printf(s, "EVENT_ID    INTID  HWINTID   TARGET   COL_ID HW\n");
    seq_printf(s, "-----------------------------------------------\n");
    }
    if (ite.irq && ite.collection) {
    seq_printf(s, "%8u %8u %8u %8u %8u %2d\n",
    ite.event_id, ite.irq.intid, ite.irq.hwintid,
    ite.collection.target_addr,
    ite.collection.collection_id, ite.irq.hw);
    }
    return 0;
    }
    static const struct seq_operations vgic_its_debug_sops = {
    .start = vgic_its_debug_start,
    .next  = vgic_its_debug_next,
    .stop  = vgic_its_debug_stop,
    .show  = vgic_its_debug_show
    };
    DEFINE_SEQ_ATTRIBUTE(vgic_its_debug);
//
// vgic_its_debug_init - Initializes the debugfs interface for VGIC ITS.
// @dev: The KVM device structure.
//
// This function creates a debugfs file named "vgic-its-state@%its_base"
// to expose the ITS table information.
//
// Return: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn vgic_its_debug_init(dev: *mut kvm_device) -> c_int {
    int vgic_its_debug_init(struct kvm_device *dev)
    {
    struct vgic_its *its = dev.private;
    char *name;
    name = kasprintf(GFP_KERNEL, "vgic-its-state@%llx", (u64)its.vgic_its_base);
    if (!name)
    return -ENOMEM;
    debugfs_create_file(name, 0444, dev.kvm.debugfs_dentry, its, &vgic_its_debug_fops);
    kfree(name);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_its_debug_destroy(dev: *mut kvm_device) {
    void vgic_its_debug_destroy(struct kvm_device *dev)
    {
    }
