//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/mpic.c
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


//
// OpenPIC emulation
//
// Copyright (c) 2004 Jocelyn Mayer
// 2011 Alexander Graf
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//

pub const MAX_CPU: c_int = 32;
pub const MAX_SRC: c_int = 256;
pub const MAX_TMR: c_int = 4;
pub const MAX_IPI: c_int = 4;
pub const MAX_MSI: c_int = 8;

pub const VID: c_uint = 0x03	/* MPIC version ID */;
// OpenPIC capability flags

// OpenPIC address map
pub const OPENPIC_REG_SIZE: c_uint = 0x40000;
pub const OPENPIC_GLB_REG_START: c_uint = 0x0;
pub const OPENPIC_GLB_REG_SIZE: c_uint = 0x10F0;
pub const OPENPIC_TMR_REG_START: c_uint = 0x10F0;
pub const OPENPIC_TMR_REG_SIZE: c_uint = 0x220;
pub const OPENPIC_MSI_REG_START: c_uint = 0x1600;
pub const OPENPIC_MSI_REG_SIZE: c_uint = 0x200;
pub const OPENPIC_SUMMARY_REG_START: c_uint = 0x3800;
pub const OPENPIC_SUMMARY_REG_SIZE: c_uint = 0x800;
pub const OPENPIC_SRC_REG_START: c_uint = 0x10000;

pub const OPENPIC_CPU_REG_START: c_uint = 0x20000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mpic_info {
    pub max_ext: c_int,
}

    static struct fsl_mpic_info fsl_mpic_20 = {
    .max_ext = 12,
    };
    static struct fsl_mpic_info fsl_mpic_42 = {
    .max_ext = 12,
    };
pub const FRR_NIRQ_SHIFT: c_int = 16;
pub const FRR_NCPU_SHIFT: c_int = 8;
pub const FRR_VID_SHIFT: c_int = 0;
pub const VID_REVISION_1_2: c_int = 2;
pub const VID_REVISION_1_3: c_int = 3;
pub const VIR_GENERIC: c_uint = 0x00000000	/* Generic Vendor ID */;
pub const GCR_RESET: c_uint = 0x80000000;
pub const GCR_MODE_PASS: c_uint = 0x00000000;
pub const GCR_MODE_MIXED: c_uint = 0x20000000;
pub const GCR_MODE_PROXY: c_uint = 0x60000000;
pub const TBCR_CI: c_uint = 0x80000000	/* count inhibit */;
pub const TCCR_TOG: c_uint = 0x80000000	/* toggles when decrement to zero */;
pub const IDR_EP_SHIFT: c_int = 31;

pub const IDR_CI0_SHIFT: c_int = 30;
pub const IDR_CI1_SHIFT: c_int = 29;
pub const IDR_P1_SHIFT: c_int = 1;
pub const IDR_P0_SHIFT: c_int = 0;
pub const ILR_INTTGT_MASK: c_uint = 0x000000ff;
pub const ILR_INTTGT_INT: c_uint = 0x00;
pub const ILR_INTTGT_CINT: c_uint = 0x01	/* critical */;
pub const ILR_INTTGT_MCP: c_uint = 0x02	/* machine check */;
pub const NUM_OUTPUTS: c_int = 3;
pub const MSIIR_OFFSET: c_uint = 0x140;
pub const MSIIR_SRS_SHIFT: c_int = 29;

pub const MSIIR_IBS_SHIFT: c_int = 24;

#[no_mangle]
unsafe extern "C" fn get_current_cpu() -> c_int {
    static int get_current_cpu(void)
    {

    struct kvm_vcpu *vcpu = current.thread.kvm_vcpu;
    return vcpu ? vcpu.arch.irq_cpu_id : -1;

// XXX
    return -1;

    }
    static int openpic_cpu_write_internal(void *opaque, gpa_t addr,
    u32 val, int idx);
    static int openpic_cpu_read_internal(void *opaque, gpa_t addr,
    u32 *ptr, int idx);
    static inline void write_IRQreg_idr(struct openpic *opp, int n_IRQ,
    uint32_t val);
    enum irq_type {
    IRQ_TYPE_NORMAL = 0,
    IRQ_TYPE_FSLINT,	/* FSL internal interrupt -- level only */
    IRQ_TYPE_FSLSPECIAL,	/* FSL timer/IPI interrupt, edge, no polarity */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_queue {
// Round up to the nearest 64 IRQs so that the queue length
// won't change when moving between 32 and 64 bit hosts.
//
    pub ~63)]: unsigned long queue[BITS_TO_LONGS((MAX_IRQ + 63) &,
    pub next: c_int,
    pub priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_source {
    pub /: *mut *mut uint32_t ivpr; / IRQ vector/priority register,
    pub /: *mut *mut uint32_t idr; / IRQ destination register,
    pub /: *mut *mut uint32_t destmask; / bitmap of CPU destinations,
    pub last_cpu: c_int,
    pub /: *mut *mut int output; / IRQ level, e.g. ILR_INTTGT_INT,
    pub /: *mut *mut int pending; / TRUE if IRQ is pending,
    pub type: enum irq_type,
    pub /: *mut *mut bool level:1; / level-triggered,
    pub /: *mut *mut bool nomask:1; / critical interrupts ignore mask on some FSL MPICs,
}

pub const IVPR_MASK_SHIFT: c_int = 31;

pub const IVPR_ACTIVITY_SHIFT: c_int = 30;

pub const IVPR_MODE_SHIFT: c_int = 29;

pub const IVPR_POLARITY_SHIFT: c_int = 23;

pub const IVPR_SENSE_SHIFT: c_int = 22;

// IDR[EP/CI] are only for FSL MPIC prior to v4.0
pub const IDR_EP: c_uint = 0x80000000	/* external pin */;
pub const IDR_CI: c_uint = 0x40000000	/* critical interrupt */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_dest {
    pub vcpu: *mut kvm_vcpu,
    pub /: *mut *mut int32_t ctpr; / CPU current task priority,
    pub raised: irq_queue,
    pub servicing: irq_queue,
// Count of IRQ sources asserting on non-INT outputs
    pub outputs_active: [u32; NUM_OUTPUTS],
}

pub const MAX_MMIO_REGIONS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct openpic {
    pub kvm: *mut kvm,
    pub dev: *mut kvm_device,
    pub mmio: kvm_io_device,
    pub mmio_regions: [*const mem_reg; MAX_MMIO_REGIONS],
    pub num_mmio_regions: c_int,
    pub reg_base: gpa_t,
    pub lock: spinlock_t,
// Behavior control
    pub fsl: *mut fsl_mpic_info,
    pub model: u32,
    pub flags: u32,
    pub nb_irqs: u32,
    pub vid: u32,
    pub /: *mut *mut uint32_t vir; / Vendor identification register,
    pub vector_mask: u32,
    pub tfrr_reset: u32,
    pub ivpr_reset: u32,
    pub idr_reset: u32,
    pub brr1: u32,
    pub mpic_mode_mask: u32,
// Global registers
    pub /: *mut *mut uint32_t frr; / Feature reporting register,
    pub /: *mut *mut uint32_t gcr; / Global configuration register,
    pub /: *mut *mut uint32_t pir; / Processor initialization register,
    pub /: *mut *mut uint32_t spve; / Spurious vector register,
    pub /: *mut *mut uint32_t tfrr; / Timer frequency reporting register,
// Source registers
    pub src: [irq_source; MAX_IRQ],
// Local registers per output pin
    pub dst: [irq_dest; MAX_CPU],
    pub nb_cpus: u32,
// Timer registers
    struct {
    pub /: *mut *mut uint32_t tccr; / Global timer current count register,
    pub /: *mut *mut uint32_t tbcr; / Global timer base count register,
    pub timers: [}; MAX_TMR],
// Shared MSI registers
    struct {
    pub /: *mut *mut uint32_t msir; / Shared Message Signaled Interrupt Register,
    pub msi: [}; MAX_MSI],
    pub max_irq: u32,
    pub irq_ipi0: u32,
    pub irq_tim0: u32,
    pub irq_msi: u32,
}

    static void mpic_irq_raise(struct openpic *opp, struct irq_dest *dst,
    int output)
    {
    struct kvm_interrupt irq = {
    .irq = KVM_INTERRUPT_SET_LEVEL,
    };
    if (!dst.vcpu) {
    pr_debug("%s: destination cpu %d does not exist\n",
    __func__, (int)(dst - &opp.dst[0]));
    return;
    }
    pr_debug("%s: cpu %d output %d\n", __func__, dst.vcpu.arch.irq_cpu_id,
    output);
    if (output != ILR_INTTGT_INT)	/* TODO */
    return;
    kvm_vcpu_ioctl_interrupt(dst.vcpu, &irq);
    }
    static void mpic_irq_lower(struct openpic *opp, struct irq_dest *dst,
    int output)
    {
    if (!dst.vcpu) {
    pr_debug("%s: destination cpu %d does not exist\n",
    __func__, (int)(dst - &opp.dst[0]));
    return;
    }
    pr_debug("%s: cpu %d output %d\n", __func__, dst.vcpu.arch.irq_cpu_id,
    output);
    if (output != ILR_INTTGT_INT)	/* TODO */
    return;
    kvmppc_core_dequeue_external(dst.vcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn IRQ_setbit(q: *mut irq_queue, n_IRQ: c_int) {
    static inline void IRQ_setbit(struct irq_queue *q, int n_IRQ)
    {
    set_bit(n_IRQ, q.queue);
    }
#[no_mangle]
pub unsafe extern "C" fn IRQ_resetbit(q: *mut irq_queue, n_IRQ: c_int) {
    static inline void IRQ_resetbit(struct irq_queue *q, int n_IRQ)
    {
    clear_bit(n_IRQ, q.queue);
    }
#[no_mangle]
unsafe extern "C" fn IRQ_check(opp: *mut openpic, q: *mut irq_queue) {
    static void IRQ_check(struct openpic *opp, struct irq_queue *q)
    {
    let mut irq: c_int = -1;
    let mut next: c_int = -1;
    let mut priority: c_int = -1;
    for (;;) {
    irq = find_next_bit(q.queue, opp.max_irq, irq + 1);
    if (irq == opp.max_irq)
    break;
    pr_debug("IRQ_check: irq %d set ivpr_pr=%d pr=%d\n",
    irq, IVPR_PRIORITY(opp.src[irq].ivpr), priority);
    if (IVPR_PRIORITY(opp.src[irq].ivpr) > priority) {
    next = irq;
    priority = IVPR_PRIORITY(opp.src[irq].ivpr);
    }
    }
    q.next = next;
    q.priority = priority;
    }
#[no_mangle]
unsafe extern "C" fn IRQ_get_next(opp: *mut openpic, q: *mut irq_queue) -> c_int {
    static int IRQ_get_next(struct openpic *opp, struct irq_queue *q)
    {
// XXX: optimize
    IRQ_check(opp, q);
    return q.next;
    }
    static void IRQ_local_pipe(struct openpic *opp, int n_CPU, int n_IRQ,
    bool active, bool was_active)
    {
    struct irq_dest *dst;
    struct irq_source *src;
    int priority;
    dst = &opp.dst[n_CPU];
    src = &opp.src[n_IRQ];
    pr_debug("%s: IRQ %d active %d was %d\n",
    __func__, n_IRQ, active, was_active);
    if (src.output != ILR_INTTGT_INT) {
    pr_debug("%s: output %d irq %d active %d was %d count %d\n",
    __func__, src.output, n_IRQ, active, was_active,
    dst.outputs_active[src.output]);
// On Freescale MPIC, critical interrupts ignore priority,
// IACK, EOI, etc.  Before MPIC v4.1 they also ignore
// masking.
//
    if (active) {
    if (!was_active &&
    dst.outputs_active[src.output]++ == 0) {
    pr_debug("%s: Raise OpenPIC output %d cpu %d irq %d\n",
    __func__, src.output, n_CPU, n_IRQ);
    mpic_irq_raise(opp, dst, src.output);
    }
    } else {
    if (was_active &&
    --dst.outputs_active[src.output] == 0) {
    pr_debug("%s: Lower OpenPIC output %d cpu %d irq %d\n",
    __func__, src.output, n_CPU, n_IRQ);
    mpic_irq_lower(opp, dst, src.output);
    }
    }
    return;
    }
    priority = IVPR_PRIORITY(src.ivpr);
// Even if the interrupt doesn't have enough priority,
// it is still raised, in case ctpr is lowered later.
//
    if (active)
    IRQ_setbit(&dst.raised, n_IRQ);
    else
    IRQ_resetbit(&dst.raised, n_IRQ);
    IRQ_check(opp, &dst.raised);
    if (active && priority <= dst.ctpr) {
    pr_debug("%s: IRQ %d priority %d too low for ctpr %d on CPU %d\n",
    __func__, n_IRQ, priority, dst.ctpr, n_CPU);
    active = 0;
    }
    if (active) {
    if (IRQ_get_next(opp, &dst.servicing) >= 0 &&
    priority <= dst.servicing.priority) {
    pr_debug("%s: IRQ %d is hidden by servicing IRQ %d on CPU %d\n",
    __func__, n_IRQ, dst.servicing.next, n_CPU);
    } else {
    pr_debug("%s: Raise OpenPIC INT output cpu %d irq %d/%d\n",
    __func__, n_CPU, n_IRQ, dst.raised.next);
    mpic_irq_raise(opp, dst, ILR_INTTGT_INT);
    }
    } else {
    IRQ_get_next(opp, &dst.servicing);
    if (dst.raised.priority > dst.ctpr &&
    dst.raised.priority > dst.servicing.priority) {
    pr_debug("%s: IRQ %d inactive, IRQ %d prio %d above %d/%d, CPU %d\n",
    __func__, n_IRQ, dst.raised.next,
    dst.raised.priority, dst.ctpr,
    dst.servicing.priority, n_CPU);
// IRQ line stays asserted
    } else {
    pr_debug("%s: IRQ %d inactive, current prio %d/%d, CPU %d\n",
    __func__, n_IRQ, dst.ctpr,
    dst.servicing.priority, n_CPU);
    mpic_irq_lower(opp, dst, ILR_INTTGT_INT);
    }
    }
    }
// update pic state because registers for n_IRQ have changed value
#[no_mangle]
unsafe extern "C" fn openpic_update_irq(opp: *mut openpic, n_IRQ: c_int) {
    static void openpic_update_irq(struct openpic *opp, int n_IRQ)
    {
    struct irq_source *src;
    bool active, was_active;
    int i;
    src = &opp.src[n_IRQ];
    active = src.pending;
    if ((src.ivpr & IVPR_MASK_MASK) && !src.nomask) {
// Interrupt source is disabled
    pr_debug("%s: IRQ %d is disabled\n", __func__, n_IRQ);
    active = false;
    }
    was_active = !!(src.ivpr & IVPR_ACTIVITY_MASK);
//
// We don't have a similar check for already-active because
// ctpr may have changed and we need to withdraw the interrupt.
//
    if (!active && !was_active) {
    pr_debug("%s: IRQ %d is already inactive\n", __func__, n_IRQ);
    return;
    }
    if (active)
    src.ivpr |= IVPR_ACTIVITY_MASK;
    else
    src.ivpr &= ~IVPR_ACTIVITY_MASK;
    if (src.destmask == 0) {
// No target
    pr_debug("%s: IRQ %d has no target\n", __func__, n_IRQ);
    return;
    }
    if (src.destmask == (1 << src.last_cpu)) {
// Only one CPU is allowed to receive this IRQ
    IRQ_local_pipe(opp, src.last_cpu, n_IRQ, active, was_active);
    } else if (!(src.ivpr & IVPR_MODE_MASK)) {
// Directed delivery mode
    for (i = 0; i < opp.nb_cpus; i++) {
    if (src.destmask & (1 << i)) {
    IRQ_local_pipe(opp, i, n_IRQ, active,
    was_active);
    }
    }
    } else {
// Distributed delivery mode
    for (i = src.last_cpu + 1; i != src.last_cpu; i++) {
    if (i == opp.nb_cpus)
    i = 0;
    if (src.destmask & (1 << i)) {
    IRQ_local_pipe(opp, i, n_IRQ, active,
    was_active);
    src.last_cpu = i;
    break;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn openpic_set_irq(opaque: *mut c_void, n_IRQ: c_int, level: c_int) {
    static void openpic_set_irq(void *opaque, int n_IRQ, int level)
    {
    struct openpic *opp = opaque;
    struct irq_source *src;
    if (n_IRQ >= MAX_IRQ) {
    WARN_ONCE(1, "%s: IRQ %d out of range\n", __func__, n_IRQ);
    return;
    }
    src = &opp.src[n_IRQ];
    pr_debug("openpic: set irq %d = %d ivpr=0x%08x\n",
    n_IRQ, level, src.ivpr);
    if (src.level) {
// level-sensitive irq
    src.pending = level;
    openpic_update_irq(opp, n_IRQ);
    } else {
// edge-sensitive irq
    if (level) {
    src.pending = 1;
    openpic_update_irq(opp, n_IRQ);
    }
    if (src.output != ILR_INTTGT_INT) {
// Edge-triggered interrupts shouldn't be used
// with non-INT delivery, but just in case,
// try to make it do something sane rather than
// cause an interrupt storm.  This is close to
// what you'd probably see happen in real hardware.
//
    src.pending = 0;
    openpic_update_irq(opp, n_IRQ);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn openpic_reset(opp: *mut openpic) {
    static void openpic_reset(struct openpic *opp)
    {
    int i;
    opp.gcr = GCR_RESET;
// Initialise controller registers
    opp.frr = ((opp.nb_irqs - 1) << FRR_NIRQ_SHIFT) |
    (opp.vid << FRR_VID_SHIFT);
    opp.pir = 0;
    opp.spve = -1 & opp.vector_mask;
    opp.tfrr = opp.tfrr_reset;
// Initialise IRQ sources
    for (i = 0; i < opp.max_irq; i++) {
    opp.src[i].ivpr = opp.ivpr_reset;
    switch (opp.src[i].type) {
    case IRQ_TYPE_NORMAL:
    opp.src[i].level =
    !!(opp.ivpr_reset & IVPR_SENSE_MASK);
    break;
    case IRQ_TYPE_FSLINT:
    opp.src[i].ivpr |= IVPR_POLARITY_MASK;
    break;
    case IRQ_TYPE_FSLSPECIAL:
    break;
    }
    write_IRQreg_idr(opp, i, opp.idr_reset);
    }
// Initialise IRQ destinations
    for (i = 0; i < MAX_CPU; i++) {
    opp.dst[i].ctpr = 15;
    memset(&opp.dst[i].raised, 0, sizeof(struct irq_queue));
    opp.dst[i].raised.next = -1;
    memset(&opp.dst[i].servicing, 0, sizeof(struct irq_queue));
    opp.dst[i].servicing.next = -1;
    }
// Initialise timers
    for (i = 0; i < MAX_TMR; i++) {
    opp.timers[i].tccr = 0;
    opp.timers[i].tbcr = TBCR_CI;
    }
// Go out of RESET state
    opp.gcr = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn read_IRQreg_idr(opp: *mut openpic, n_IRQ: c_int) -> u32 {
    static inline uint32_t read_IRQreg_idr(struct openpic *opp, int n_IRQ)
    {
    return opp.src[n_IRQ].idr;
    }
#[no_mangle]
pub unsafe extern "C" fn read_IRQreg_ilr(opp: *mut openpic, n_IRQ: c_int) -> u32 {
    static inline uint32_t read_IRQreg_ilr(struct openpic *opp, int n_IRQ)
    {
    if (opp.flags & OPENPIC_FLAG_ILR)
    return opp.src[n_IRQ].output;
    return 0xffffffff;
    }
#[no_mangle]
pub unsafe extern "C" fn read_IRQreg_ivpr(opp: *mut openpic, n_IRQ: c_int) -> u32 {
    static inline uint32_t read_IRQreg_ivpr(struct openpic *opp, int n_IRQ)
    {
    return opp.src[n_IRQ].ivpr;
    }
    static inline void write_IRQreg_idr(struct openpic *opp, int n_IRQ,
    uint32_t val)
    {
    struct irq_source *src = &opp.src[n_IRQ];
    let mut normal_mask: u32 = (1UL << opp.nb_cpus) - 1;
    let mut crit_mask: u32 = 0;
    let mut mask: u32 = normal_mask;
    let mut crit_shift: c_int = IDR_EP_SHIFT - opp.nb_cpus;
    int i;
    if (opp.flags & OPENPIC_FLAG_IDR_CRIT) {
    crit_mask = mask << crit_shift;
    mask |= crit_mask | IDR_EP;
    }
    src.idr = val & mask;
    pr_debug("Set IDR %d to 0x%08x\n", n_IRQ, src.idr);
    if (opp.flags & OPENPIC_FLAG_IDR_CRIT) {
    if (src.idr & crit_mask) {
    if (src.idr & normal_mask) {
    pr_debug("%s: IRQ configured for multiple output types, using critical\n",
    __func__);
    }
    src.output = ILR_INTTGT_CINT;
    src.nomask = true;
    src.destmask = 0;
    for (i = 0; i < opp.nb_cpus; i++) {
    let mut n_ci: c_int = IDR_CI0_SHIFT - i;
    if (src.idr & (1UL << n_ci))
    src.destmask |= 1UL << i;
    }
    } else {
    src.output = ILR_INTTGT_INT;
    src.nomask = false;
    src.destmask = src.idr & normal_mask;
    }
    } else {
    src.destmask = src.idr;
    }
    }
    static inline void write_IRQreg_ilr(struct openpic *opp, int n_IRQ,
    uint32_t val)
    {
    if (opp.flags & OPENPIC_FLAG_ILR) {
    struct irq_source *src = &opp.src[n_IRQ];
    src.output = val & ILR_INTTGT_MASK;
    pr_debug("Set ILR %d to 0x%08x, output %d\n", n_IRQ, src.idr,
    src.output);
// TODO: on MPIC v4.0 only, set nomask for non-INT
    }
    }
    static inline void write_IRQreg_ivpr(struct openpic *opp, int n_IRQ,
    uint32_t val)
    {
    uint32_t mask;
// NOTE when implementing newer FSL MPIC models: starting with v4.0,
// the polarity bit is read-only on internal interrupts.
//
    mask = IVPR_MASK_MASK | IVPR_PRIORITY_MASK | IVPR_SENSE_MASK |
    IVPR_POLARITY_MASK | opp.vector_mask;
// ACTIVITY bit is read-only
    opp.src[n_IRQ].ivpr =
    (opp.src[n_IRQ].ivpr & IVPR_ACTIVITY_MASK) | (val & mask);
// For FSL internal interrupts, The sense bit is reserved and zero,
// and the interrupt is always level-triggered.  Timers and IPIs
// have no sense or polarity bits, and are edge-triggered.
//
    switch (opp.src[n_IRQ].type) {
    case IRQ_TYPE_NORMAL:
    opp.src[n_IRQ].level =
    !!(opp.src[n_IRQ].ivpr & IVPR_SENSE_MASK);
    break;
    case IRQ_TYPE_FSLINT:
    opp.src[n_IRQ].ivpr &= ~IVPR_SENSE_MASK;
    break;
    case IRQ_TYPE_FSLSPECIAL:
    opp.src[n_IRQ].ivpr &= ~(IVPR_POLARITY_MASK | IVPR_SENSE_MASK);
    break;
    }
    openpic_update_irq(opp, n_IRQ);
    pr_debug("Set IVPR %d to 0x%08x . 0x%08x\n", n_IRQ, val,
    opp.src[n_IRQ].ivpr);
    }
#[no_mangle]
unsafe extern "C" fn openpic_gcr_write(opp: *mut openpic, val: u64) {
    static void openpic_gcr_write(struct openpic *opp, uint64_t val)
    {
    if (val & GCR_RESET) {
    openpic_reset(opp);
    return;
    }
    opp.gcr &= ~opp.mpic_mode_mask;
    opp.gcr |= val & opp.mpic_mode_mask;
    }
#[no_mangle]
unsafe extern "C" fn openpic_gbl_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_gbl_write(void *opaque, gpa_t addr, u32 val)
    {
    struct openpic *opp = opaque;
    let mut err: c_int = 0;
    pr_debug("%s: addr %#llx <= %08x\n", __func__, addr, val);
    if (addr & 0xF)
    return 0;
    switch (addr) {
    case 0x00:	/* Block Revision Register1 (BRR1) is Readonly */
    break;
    case 0x40:
    case 0x50:
    case 0x60:
    case 0x70:
    case 0x80:
    case 0x90:
    case 0xA0:
    case 0xB0:
    err = openpic_cpu_write_internal(opp, addr, val,
    get_current_cpu());
    break;
    case 0x1000:		/* FRR */
    break;
    case 0x1020:		/* GCR */
    openpic_gcr_write(opp, val);
    break;
    case 0x1080:		/* VIR */
    break;
    case 0x1090:		/* PIR */
//
// This register is used to reset a CPU core --
// let userspace handle it.
//
    err = -ENXIO;
    break;
    case 0x10A0:		/* IPI_IVPR */
    case 0x10B0:
    case 0x10C0:
    case 0x10D0: {
    int idx;
    idx = (addr - 0x10A0) >> 4;
    write_IRQreg_ivpr(opp, opp.irq_ipi0 + idx, val);
    break;
    }
    case 0x10E0:		/* SPVE */
    opp.spve = val & opp.vector_mask;
    break;
    default:
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn openpic_gbl_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_gbl_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    struct openpic *opp = opaque;
    u32 retval;
    let mut err: c_int = 0;
    pr_debug("%s: addr %#llx\n", __func__, addr);
    retval = 0xFFFFFFFF;
    if (addr & 0xF)
    goto out;
    switch (addr) {
    case 0x1000:		/* FRR */
    retval = opp.frr;
    retval |= (opp.nb_cpus - 1) << FRR_NCPU_SHIFT;
    break;
    case 0x1020:		/* GCR */
    retval = opp.gcr;
    break;
    case 0x1080:		/* VIR */
    retval = opp.vir;
    break;
    case 0x1090:		/* PIR */
    retval = 0x00000000;
    break;
    case 0x00:		/* Block Revision Register1 (BRR1) */
    retval = opp.brr1;
    break;
    case 0x40:
    case 0x50:
    case 0x60:
    case 0x70:
    case 0x80:
    case 0x90:
    case 0xA0:
    case 0xB0:
    err = openpic_cpu_read_internal(opp, addr,
    &retval, get_current_cpu());
    break;
    case 0x10A0:		/* IPI_IVPR */
    case 0x10B0:
    case 0x10C0:
    case 0x10D0:
    {
    int idx;
    idx = (addr - 0x10A0) >> 4;
    retval = read_IRQreg_ivpr(opp, opp.irq_ipi0 + idx);
    }
    break;
    case 0x10E0:		/* SPVE */
    retval = opp.spve;
    break;
    default:
    break;
    }
    out:
    pr_debug("%s: => 0x%08x\n", __func__, retval);
// ptr = retval;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn openpic_tmr_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_tmr_write(void *opaque, gpa_t addr, u32 val)
    {
    struct openpic *opp = opaque;
    int idx;
    addr += 0x10f0;
    pr_debug("%s: addr %#llx <= %08x\n", __func__, addr, val);
    if (addr & 0xF)
    return 0;
    if (addr == 0x10f0) {
// TFRR
    opp.tfrr = val;
    return 0;
    }
    idx = (addr >> 6) & 0x3;
    addr = addr & 0x30;
    switch (addr & 0x30) {
    case 0x00:		/* TCCR */
    break;
    case 0x10:		/* TBCR */
    if ((opp.timers[idx].tccr & TCCR_TOG) != 0 &&
    (val & TBCR_CI) == 0 &&
    (opp.timers[idx].tbcr & TBCR_CI) != 0)
    opp.timers[idx].tccr &= ~TCCR_TOG;
    opp.timers[idx].tbcr = val;
    break;
    case 0x20:		/* TVPR */
    write_IRQreg_ivpr(opp, opp.irq_tim0 + idx, val);
    break;
    case 0x30:		/* TDR */
    write_IRQreg_idr(opp, opp.irq_tim0 + idx, val);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_tmr_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_tmr_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    struct openpic *opp = opaque;
    let mut retval: u32 = -1;
    int idx;
    pr_debug("%s: addr %#llx\n", __func__, addr);
    if (addr & 0xF)
    goto out;
    idx = (addr >> 6) & 0x3;
    if (addr == 0x0) {
// TFRR
    retval = opp.tfrr;
    goto out;
    }
    switch (addr & 0x30) {
    case 0x00:		/* TCCR */
    retval = opp.timers[idx].tccr;
    break;
    case 0x10:		/* TBCR */
    retval = opp.timers[idx].tbcr;
    break;
    case 0x20:		/* TIPV */
    retval = read_IRQreg_ivpr(opp, opp.irq_tim0 + idx);
    break;
    case 0x30:		/* TIDE (TIDR) */
    retval = read_IRQreg_idr(opp, opp.irq_tim0 + idx);
    break;
    }
    out:
    pr_debug("%s: => 0x%08x\n", __func__, retval);
// ptr = retval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_src_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_src_write(void *opaque, gpa_t addr, u32 val)
    {
    struct openpic *opp = opaque;
    int idx;
    pr_debug("%s: addr %#llx <= %08x\n", __func__, addr, val);
    addr = addr & 0xffff;
    idx = addr >> 5;
    switch (addr & 0x1f) {
    case 0x00:
    write_IRQreg_ivpr(opp, idx, val);
    break;
    case 0x10:
    write_IRQreg_idr(opp, idx, val);
    break;
    case 0x18:
    write_IRQreg_ilr(opp, idx, val);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_src_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_src_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    struct openpic *opp = opaque;
    uint32_t retval;
    int idx;
    pr_debug("%s: addr %#llx\n", __func__, addr);
    retval = 0xFFFFFFFF;
    addr = addr & 0xffff;
    idx = addr >> 5;
    switch (addr & 0x1f) {
    case 0x00:
    retval = read_IRQreg_ivpr(opp, idx);
    break;
    case 0x10:
    retval = read_IRQreg_idr(opp, idx);
    break;
    case 0x18:
    retval = read_IRQreg_ilr(opp, idx);
    break;
    }
    pr_debug("%s: => 0x%08x\n", __func__, retval);
// ptr = retval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_msi_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_msi_write(void *opaque, gpa_t addr, u32 val)
    {
    struct openpic *opp = opaque;
    let mut idx: c_int = opp.irq_msi;
    int srs, ibs;
    pr_debug("%s: addr %#llx <= 0x%08x\n", __func__, addr, val);
    if (addr & 0xF)
    return 0;
    switch (addr) {
    case MSIIR_OFFSET:
    srs = val >> MSIIR_SRS_SHIFT;
    idx += srs;
    ibs = (val & MSIIR_IBS_MASK) >> MSIIR_IBS_SHIFT;
    opp.msi[srs].msir |= 1 << ibs;
    openpic_set_irq(opp, idx, 1);
    break;
    default:
// most registers are read-only, thus ignored
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_msi_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_msi_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    struct openpic *opp = opaque;
    let mut r: u32 = 0;
    int i, srs;
    pr_debug("%s: addr %#llx\n", __func__, addr);
    if (addr & 0xF)
    return -ENXIO;
    srs = addr >> 4;
    switch (addr) {
    case 0x00:
    case 0x10:
    case 0x20:
    case 0x30:
    case 0x40:
    case 0x50:
    case 0x60:
    case 0x70:		/* MSIRs */
    r = opp.msi[srs].msir;
// Clear on read
    opp.msi[srs].msir = 0;
    openpic_set_irq(opp, opp.irq_msi + srs, 0);
    break;
    case 0x120:		/* MSISR */
    for (i = 0; i < MAX_MSI; i++)
    r |= (opp.msi[i].msir ? 1 : 0) << i;
    break;
    }
    pr_debug("%s: => 0x%08x\n", __func__, r);
// ptr = r;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_summary_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_summary_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    let mut r: u32 = 0;
    pr_debug("%s: addr %#llx\n", __func__, addr);
// TODO: EISR/EIMR
// ptr = r;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_summary_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_summary_write(void *opaque, gpa_t addr, u32 val)
    {
    pr_debug("%s: addr %#llx <= 0x%08x\n", __func__, addr, val);
// TODO: EISR/EIMR
    return 0;
    }
    static int openpic_cpu_write_internal(void *opaque, gpa_t addr,
    u32 val, int idx)
    {
    struct openpic *opp = opaque;
    struct irq_source *src;
    struct irq_dest *dst;
    int s_IRQ, n_IRQ;
    pr_debug("%s: cpu %d addr %#llx <= 0x%08x\n", __func__, idx,
    addr, val);
    if (idx < 0)
    return 0;
    if (addr & 0xF)
    return 0;
    dst = &opp.dst[idx];
    addr &= 0xFF0;
    switch (addr) {
    case 0x40:		/* IPIDR */
    case 0x50:
    case 0x60:
    case 0x70:
    idx = (addr - 0x40) >> 4;
// we use IDE as mask which CPUs to deliver the IPI to still.
    opp.src[opp.irq_ipi0 + idx].destmask |= val;
    openpic_set_irq(opp, opp.irq_ipi0 + idx, 1);
    openpic_set_irq(opp, opp.irq_ipi0 + idx, 0);
    break;
    case 0x80:		/* CTPR */
    dst.ctpr = val & 0x0000000F;
    pr_debug("%s: set CPU %d ctpr to %d, raised %d servicing %d\n",
    __func__, idx, dst.ctpr, dst.raised.priority,
    dst.servicing.priority);
    if (dst.raised.priority <= dst.ctpr) {
    pr_debug("%s: Lower OpenPIC INT output cpu %d due to ctpr\n",
    __func__, idx);
    mpic_irq_lower(opp, dst, ILR_INTTGT_INT);
    } else if (dst.raised.priority > dst.servicing.priority) {
    pr_debug("%s: Raise OpenPIC INT output cpu %d irq %d\n",
    __func__, idx, dst.raised.next);
    mpic_irq_raise(opp, dst, ILR_INTTGT_INT);
    }
    break;
    case 0x90:		/* WHOAMI */
// Read-only register
    break;
    case 0xA0:		/* IACK */
// Read-only register
    break;
    case 0xB0: {		/* EOI */
    int notify_eoi;
    pr_debug("EOI\n");
    s_IRQ = IRQ_get_next(opp, &dst.servicing);
    if (s_IRQ < 0) {
    pr_debug("%s: EOI with no interrupt in service\n",
    __func__);
    break;
    }
    IRQ_resetbit(&dst.servicing, s_IRQ);
// Notify listeners that the IRQ is over
    notify_eoi = s_IRQ;
// Set up next servicing IRQ
    s_IRQ = IRQ_get_next(opp, &dst.servicing);
// Check queued interrupts.
    n_IRQ = IRQ_get_next(opp, &dst.raised);
    src = &opp.src[n_IRQ];
    if (n_IRQ != -1 &&
    (s_IRQ == -1 ||
    IVPR_PRIORITY(src.ivpr) > dst.servicing.priority)) {
    pr_debug("Raise OpenPIC INT output cpu %d irq %d\n",
    idx, n_IRQ);
    mpic_irq_raise(opp, dst, ILR_INTTGT_INT);
    }
    spin_unlock(&opp.lock);
    kvm_notify_acked_irq(opp.kvm, 0, notify_eoi);
    spin_lock(&opp.lock);
    break;
    }
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_cpu_write(opaque: *mut c_void, addr: gpa_t, val: u32) -> c_int {
    static int openpic_cpu_write(void *opaque, gpa_t addr, u32 val)
    {
    struct openpic *opp = opaque;
    return openpic_cpu_write_internal(opp, addr, val,
    (addr & 0x1f000) >> 12);
    }
    static uint32_t openpic_iack(struct openpic *opp, struct irq_dest *dst,
    int cpu)
    {
    struct irq_source *src;
    int retval, irq;
    pr_debug("Lower OpenPIC INT output\n");
    mpic_irq_lower(opp, dst, ILR_INTTGT_INT);
    irq = IRQ_get_next(opp, &dst.raised);
    pr_debug("IACK: irq=%d\n", irq);
    if (irq == -1)
// No more interrupt pending
    return opp.spve;
    src = &opp.src[irq];
    if (!(src.ivpr & IVPR_ACTIVITY_MASK) ||
    !(IVPR_PRIORITY(src.ivpr) > dst.ctpr)) {
    pr_err("%s: bad raised IRQ %d ctpr %d ivpr 0x%08x\n",
    __func__, irq, dst.ctpr, src.ivpr);
    openpic_update_irq(opp, irq);
    retval = opp.spve;
    } else {
// IRQ enter servicing state
    IRQ_setbit(&dst.servicing, irq);
    retval = IVPR_VECTOR(opp, src.ivpr);
    }
    if (!src.level) {
// edge-sensitive IRQ
    src.ivpr &= ~IVPR_ACTIVITY_MASK;
    src.pending = 0;
    IRQ_resetbit(&dst.raised, irq);
    }
    if ((irq >= opp.irq_ipi0) && (irq < (opp.irq_ipi0 + MAX_IPI))) {
    src.destmask &= ~(1 << cpu);
    if (src.destmask && !src.level) {
// trigger on CPUs that didn't know about it yet
    openpic_set_irq(opp, irq, 1);
    openpic_set_irq(opp, irq, 0);
// if all CPUs knew about it, set active bit again
    src.ivpr |= IVPR_ACTIVITY_MASK;
    }
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mpic_set_epr(vcpu: *mut kvm_vcpu) {
    void kvmppc_mpic_set_epr(struct kvm_vcpu *vcpu)
    {
    struct openpic *opp = vcpu.arch.mpic;
    let mut cpu: c_int = vcpu.arch.irq_cpu_id;
    unsigned long flags;
    spin_lock_irqsave(&opp.lock, flags);
    if ((opp.gcr & opp.mpic_mode_mask) == GCR_MODE_PROXY)
    kvmppc_set_epr(vcpu, openpic_iack(opp, &opp.dst[cpu], cpu));
    spin_unlock_irqrestore(&opp.lock, flags);
    }
    static int openpic_cpu_read_internal(void *opaque, gpa_t addr,
    u32 *ptr, int idx)
    {
    struct openpic *opp = opaque;
    struct irq_dest *dst;
    uint32_t retval;
    pr_debug("%s: cpu %d addr %#llx\n", __func__, idx, addr);
    retval = 0xFFFFFFFF;
    if (idx < 0)
    goto out;
    if (addr & 0xF)
    goto out;
    dst = &opp.dst[idx];
    addr &= 0xFF0;
    switch (addr) {
    case 0x80:		/* CTPR */
    retval = dst.ctpr;
    break;
    case 0x90:		/* WHOAMI */
    retval = idx;
    break;
    case 0xA0:		/* IACK */
    retval = openpic_iack(opp, dst, idx);
    break;
    case 0xB0:		/* EOI */
    retval = 0;
    break;
    default:
    break;
    }
    pr_debug("%s: => 0x%08x\n", __func__, retval);
    out:
// ptr = retval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn openpic_cpu_read(opaque: *mut c_void, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int openpic_cpu_read(void *opaque, gpa_t addr, u32 *ptr)
    {
    struct openpic *opp = opaque;
    return openpic_cpu_read_internal(opp, addr, ptr,
    (addr & 0x1f000) >> 12);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_reg {
    pub ptr): *mut *mut *mut int (read)(void opaque, gpa_t addr, u32,
    pub val): *mut *mut *mut int (write)(void opaque, gpa_t addr, u32,
    pub start_addr: gpa_t,
    pub size: c_int,
}

    static const struct mem_reg openpic_gbl_mmio = {
    .write = openpic_gbl_write,
    .read = openpic_gbl_read,
    .start_addr = OPENPIC_GLB_REG_START,
    .size = OPENPIC_GLB_REG_SIZE,
    };
    static const struct mem_reg openpic_tmr_mmio = {
    .write = openpic_tmr_write,
    .read = openpic_tmr_read,
    .start_addr = OPENPIC_TMR_REG_START,
    .size = OPENPIC_TMR_REG_SIZE,
    };
    static const struct mem_reg openpic_cpu_mmio = {
    .write = openpic_cpu_write,
    .read = openpic_cpu_read,
    .start_addr = OPENPIC_CPU_REG_START,
    .size = OPENPIC_CPU_REG_SIZE,
    };
    static const struct mem_reg openpic_src_mmio = {
    .write = openpic_src_write,
    .read = openpic_src_read,
    .start_addr = OPENPIC_SRC_REG_START,
    .size = OPENPIC_SRC_REG_SIZE,
    };
    static const struct mem_reg openpic_msi_mmio = {
    .read = openpic_msi_read,
    .write = openpic_msi_write,
    .start_addr = OPENPIC_MSI_REG_START,
    .size = OPENPIC_MSI_REG_SIZE,
    };
    static const struct mem_reg openpic_summary_mmio = {
    .read = openpic_summary_read,
    .write = openpic_summary_write,
    .start_addr = OPENPIC_SUMMARY_REG_START,
    .size = OPENPIC_SUMMARY_REG_SIZE,
    };
#[no_mangle]
unsafe extern "C" fn add_mmio_region(opp: *mut openpic, mr: *const mem_reg) {
    static void add_mmio_region(struct openpic *opp, const struct mem_reg *mr)
    {
    if (opp.num_mmio_regions >= MAX_MMIO_REGIONS) {
    WARN(1, "kvm mpic: too many mmio regions\n");
    return;
    }
    opp.mmio_regions[opp.num_mmio_regions++] = mr;
    }
#[no_mangle]
unsafe extern "C" fn fsl_common_init(opp: *mut openpic) {
    static void fsl_common_init(struct openpic *opp)
    {
    int i;
    let mut virq: c_int = MAX_SRC;
    add_mmio_region(opp, &openpic_msi_mmio);
    add_mmio_region(opp, &openpic_summary_mmio);
    opp.vid = VID_REVISION_1_2;
    opp.vir = VIR_GENERIC;
    opp.vector_mask = 0xFFFF;
    opp.tfrr_reset = 0;
    opp.ivpr_reset = IVPR_MASK_MASK;
    opp.idr_reset = 1 << 0;
    opp.max_irq = MAX_IRQ;
    opp.irq_ipi0 = virq;
    virq += MAX_IPI;
    opp.irq_tim0 = virq;
    virq += MAX_TMR;
    BUG_ON(virq > MAX_IRQ);
    opp.irq_msi = 224;
    for (i = 0; i < opp.fsl.max_ext; i++)
    opp.src[i].level = false;
// Internal interrupts, including message and MSI
    for (i = 16; i < MAX_SRC; i++) {
    opp.src[i].type = IRQ_TYPE_FSLINT;
    opp.src[i].level = true;
    }
// timers and IPIs
    for (i = MAX_SRC; i < virq; i++) {
    opp.src[i].type = IRQ_TYPE_FSLSPECIAL;
    opp.src[i].level = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_mpic_read_internal(opp: *mut openpic, addr: gpa_t, ptr: *mut u32) -> c_int {
    static int kvm_mpic_read_internal(struct openpic *opp, gpa_t addr, u32 *ptr)
    {
    int i;
    for (i = 0; i < opp.num_mmio_regions; i++) {
    const struct mem_reg *mr = opp.mmio_regions[i];
    if (mr.start_addr > addr || addr >= mr.start_addr + mr.size)
    continue;
    return mr.read(opp, addr - mr.start_addr, ptr);
    }
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn kvm_mpic_write_internal(opp: *mut openpic, addr: gpa_t, val: u32) -> c_int {
    static int kvm_mpic_write_internal(struct openpic *opp, gpa_t addr, u32 val)
    {
    int i;
    for (i = 0; i < opp.num_mmio_regions; i++) {
    const struct mem_reg *mr = opp.mmio_regions[i];
    if (mr.start_addr > addr || addr >= mr.start_addr + mr.size)
    continue;
    return mr.write(opp, addr - mr.start_addr, val);
    }
    return -ENXIO;
    }
    static int kvm_mpic_read(struct kvm_vcpu *vcpu,
    struct kvm_io_device *this,
    gpa_t addr, int len, void *ptr)
    {
    struct openpic *opp = container_of(this, struct openpic, mmio);
    int ret;
    union {
    u32 val;
    u8 bytes[4];
    } u;
    if (addr & (len - 1)) {
    pr_debug("%s: bad alignment %llx/%d\n",
    __func__, addr, len);
    return -EINVAL;
    }
    spin_lock_irq(&opp.lock);
    ret = kvm_mpic_read_internal(opp, addr - opp.reg_base, &u.val);
    spin_unlock_irq(&opp.lock);
//
// Technically only 32-bit accesses are allowed, but be nice to
// people dumping registers a byte at a time -- it works in real
// hardware (reads only, not writes).
//
    if (len == 4) {
// (u32 *)ptr = u.val;
    pr_debug("%s: addr %llx ret %d len 4 val %x\n",
    __func__, addr, ret, u.val);
    } else if (len == 1) {
// (u8 *)ptr = u.bytes[addr & 3];
    pr_debug("%s: addr %llx ret %d len 1 val %x\n",
    __func__, addr, ret, u.bytes[addr & 3]);
    } else {
    pr_debug("%s: bad length %d\n", __func__, len);
    return -EINVAL;
    }
    return ret;
    }
    static int kvm_mpic_write(struct kvm_vcpu *vcpu,
    struct kvm_io_device *this,
    gpa_t addr, int len, const void *ptr)
    {
    struct openpic *opp = container_of(this, struct openpic, mmio);
    int ret;
    if (len != 4) {
    pr_debug("%s: bad length %d\n", __func__, len);
    return -EOPNOTSUPP;
    }
    if (addr & 3) {
    pr_debug("%s: bad alignment %llx/%d\n", __func__, addr, len);
    return -EOPNOTSUPP;
    }
    spin_lock_irq(&opp.lock);
    ret = kvm_mpic_write_internal(opp, addr - opp.reg_base,
// (const u32 *)ptr);
    spin_unlock_irq(&opp.lock);
    pr_debug("%s: addr %llx ret %d val %x\n",
    __func__, addr, ret, *(const u32 *)ptr);
    return ret;
    }
    static const struct kvm_io_device_ops mpic_mmio_ops = {
    .read = kvm_mpic_read,
    .write = kvm_mpic_write,
    };
#[no_mangle]
unsafe extern "C" fn map_mmio(opp: *mut openpic) {
    static void map_mmio(struct openpic *opp)
    {
    kvm_iodevice_init(&opp.mmio, &mpic_mmio_ops);
    kvm_io_bus_register_dev(opp.kvm, KVM_MMIO_BUS,
    opp.reg_base, OPENPIC_REG_SIZE,
    &opp.mmio);
    }
#[no_mangle]
unsafe extern "C" fn unmap_mmio(opp: *mut openpic) {
    static void unmap_mmio(struct openpic *opp)
    {
    kvm_io_bus_unregister_dev(opp.kvm, KVM_MMIO_BUS, &opp.mmio);
    }
#[no_mangle]
unsafe extern "C" fn set_base_addr(opp: *mut openpic, attr: *mut kvm_device_attr) -> c_int {
    static int set_base_addr(struct openpic *opp, struct kvm_device_attr *attr)
    {
    u64 base;
    if (copy_from_user(&base, (u64 __user *)(long)attr.addr, sizeof(u64)))
    return -EFAULT;
    if (base & 0x3ffff) {
    pr_debug("kvm mpic %s: KVM_DEV_MPIC_BASE_ADDR %08llx not aligned\n",
    __func__, base);
    return -EINVAL;
    }
    if (base == opp.reg_base)
    return 0;
    mutex_lock(&opp.kvm.slots_lock);
    unmap_mmio(opp);
    opp.reg_base = base;
    pr_debug("kvm mpic %s: KVM_DEV_MPIC_BASE_ADDR %08llx\n",
    __func__, base);
    if (base == 0)
    goto out;
    map_mmio(opp);
    out:
    mutex_unlock(&opp.kvm.slots_lock);
    return 0;
    }
pub const ATTR_SET: c_int = 0;
pub const ATTR_GET: c_int = 1;
#[no_mangle]
unsafe extern "C" fn access_reg(opp: *mut openpic, addr: gpa_t, val: *mut u32, type: c_int) -> c_int {
    static int access_reg(struct openpic *opp, gpa_t addr, u32 *val, int type)
    {
    int ret;
    if (addr & 3)
    return -ENXIO;
    spin_lock_irq(&opp.lock);
    if (type == ATTR_SET)
    ret = kvm_mpic_write_internal(opp, addr, *val);
    else
    ret = kvm_mpic_read_internal(opp, addr, val);
    spin_unlock_irq(&opp.lock);
    pr_debug("%s: type %d addr %llx val %x\n", __func__, type, addr, *val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpic_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    static int mpic_set_attr(struct kvm_device *dev, struct kvm_device_attr *attr)
    {
    struct openpic *opp = dev.private;
    u32 attr32;
    switch (attr.group) {
    case KVM_DEV_MPIC_GRP_MISC:
    switch (attr.attr) {
    case KVM_DEV_MPIC_BASE_ADDR:
    return set_base_addr(opp, attr);
    }
    break;
    case KVM_DEV_MPIC_GRP_REGISTER:
    if (get_user(attr32, (u32 __user *)(long)attr.addr))
    return -EFAULT;
    return access_reg(opp, attr.attr, &attr32, ATTR_SET);
    case KVM_DEV_MPIC_GRP_IRQ_ACTIVE:
    if (attr.attr > MAX_SRC)
    return -EINVAL;
    if (get_user(attr32, (u32 __user *)(long)attr.addr))
    return -EFAULT;
    if (attr32 != 0 && attr32 != 1)
    return -EINVAL;
    spin_lock_irq(&opp.lock);
    openpic_set_irq(opp, attr.attr, attr32);
    spin_unlock_irq(&opp.lock);
    return 0;
    }
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn mpic_get_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    static int mpic_get_attr(struct kvm_device *dev, struct kvm_device_attr *attr)
    {
    struct openpic *opp = dev.private;
    u64 attr64;
    u32 attr32;
    int ret;
    switch (attr.group) {
    case KVM_DEV_MPIC_GRP_MISC:
    switch (attr.attr) {
    case KVM_DEV_MPIC_BASE_ADDR:
    mutex_lock(&opp.kvm.slots_lock);
    attr64 = opp.reg_base;
    mutex_unlock(&opp.kvm.slots_lock);
    if (copy_to_user((u64 __user *)(long)attr.addr,
    &attr64, sizeof(u64)))
    return -EFAULT;
    return 0;
    }
    break;
    case KVM_DEV_MPIC_GRP_REGISTER:
    ret = access_reg(opp, attr.attr, &attr32, ATTR_GET);
    if (ret)
    return ret;
    if (put_user(attr32, (u32 __user *)(long)attr.addr))
    return -EFAULT;
    return 0;
    case KVM_DEV_MPIC_GRP_IRQ_ACTIVE:
    if (attr.attr > MAX_SRC)
    return -EINVAL;
    spin_lock_irq(&opp.lock);
    attr32 = opp.src[attr.attr].pending;
    spin_unlock_irq(&opp.lock);
    if (put_user(attr32, (u32 __user *)(long)attr.addr))
    return -EFAULT;
    return 0;
    }
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn mpic_has_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    static int mpic_has_attr(struct kvm_device *dev, struct kvm_device_attr *attr)
    {
    switch (attr.group) {
    case KVM_DEV_MPIC_GRP_MISC:
    switch (attr.attr) {
    case KVM_DEV_MPIC_BASE_ADDR:
    return 0;
    }
    break;
    case KVM_DEV_MPIC_GRP_REGISTER:
    return 0;
    case KVM_DEV_MPIC_GRP_IRQ_ACTIVE:
    if (attr.attr > MAX_SRC)
    break;
    return 0;
    }
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn mpic_destroy(dev: *mut kvm_device) {
    static void mpic_destroy(struct kvm_device *dev)
    {
    struct openpic *opp = dev.private;
    dev.kvm.arch.mpic = core::ptr::null_mut();
    kfree(opp);
    kfree(dev);
    }
#[no_mangle]
unsafe extern "C" fn mpic_set_default_irq_routing(opp: *mut openpic) -> c_int {
    static int mpic_set_default_irq_routing(struct openpic *opp)
    {
    struct kvm_irq_routing_entry *routing;
// Create a nop default map, so that dereferencing it still works
    routing = kzalloc_obj(*routing);
    if (!routing)
    return -ENOMEM;
    kvm_set_irq_routing(opp.kvm, routing, 0, 0);
    kfree(routing);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpic_create(dev: *mut kvm_device, type: u32) -> c_int {
    static int mpic_create(struct kvm_device *dev, u32 type)
    {
    struct openpic *opp;
    int ret;
// We only support one MPIC at a time for now
    if (dev.kvm.arch.mpic)
    return -EINVAL;
    opp = kzalloc_obj(struct openpic);
    if (!opp)
    return -ENOMEM;
    dev.private = opp;
    opp.kvm = dev.kvm;
    opp.dev = dev;
    opp.model = type;
    spin_lock_init(&opp.lock);
    add_mmio_region(opp, &openpic_gbl_mmio);
    add_mmio_region(opp, &openpic_tmr_mmio);
    add_mmio_region(opp, &openpic_src_mmio);
    add_mmio_region(opp, &openpic_cpu_mmio);
    switch (opp.model) {
    case KVM_DEV_TYPE_FSL_MPIC_20:
    opp.fsl = &fsl_mpic_20;
    opp.brr1 = 0x00400200;
    opp.flags |= OPENPIC_FLAG_IDR_CRIT;
    opp.nb_irqs = 80;
    opp.mpic_mode_mask = GCR_MODE_MIXED;
    fsl_common_init(opp);
    break;
    case KVM_DEV_TYPE_FSL_MPIC_42:
    opp.fsl = &fsl_mpic_42;
    opp.brr1 = 0x00400402;
    opp.flags |= OPENPIC_FLAG_ILR;
    opp.nb_irqs = 196;
    opp.mpic_mode_mask = GCR_MODE_PROXY;
    fsl_common_init(opp);
    break;
    default:
    ret = -ENODEV;
    goto err;
    }
    ret = mpic_set_default_irq_routing(opp);
    if (ret)
    goto err;
    openpic_reset(opp);
    smp_wmb();
    dev.kvm.arch.mpic = opp;
    return 0;
    err:
    kfree(opp);
    return ret;
    }
    struct kvm_device_ops kvm_mpic_ops = {
    .name = "kvm-mpic",
    .create = mpic_create,
    .destroy = mpic_destroy,
    .set_attr = mpic_set_attr,
    .get_attr = mpic_get_attr,
    .has_attr = mpic_has_attr,
    };
    int kvmppc_mpic_connect_vcpu(struct kvm_device *dev, struct kvm_vcpu *vcpu,
    u32 cpu)
    {
    struct openpic *opp = dev.private;
    let mut ret: c_int = 0;
    if (dev.ops != &kvm_mpic_ops)
    return -EPERM;
    if (opp.kvm != vcpu.kvm)
    return -EPERM;
    if (cpu < 0 || cpu >= MAX_CPU)
    return -EPERM;
    spin_lock_irq(&opp.lock);
    if (opp.dst[cpu].vcpu) {
    ret = -EEXIST;
    goto out;
    }
    if (vcpu.arch.irq_type) {
    ret = -EBUSY;
    goto out;
    }
    opp.dst[cpu].vcpu = vcpu;
    opp.nb_cpus = max(opp.nb_cpus, cpu + 1);
    vcpu.arch.mpic = opp;
    vcpu.arch.irq_cpu_id = cpu;
    vcpu.arch.irq_type = KVMPPC_IRQ_MPIC;
// This might need to be changed if GCR gets extended
    if (opp.mpic_mode_mask == GCR_MODE_PROXY)
    vcpu.arch.epr_flags |= KVMPPC_EPR_KERNEL;
    out:
    spin_unlock_irq(&opp.lock);
    return ret;
    }
//
// This should only happen immediately before the mpic is destroyed,
// so we shouldn't need to worry about anything still trying to
// access the vcpu pointer.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_mpic_disconnect_vcpu(opp: *mut openpic, vcpu: *mut kvm_vcpu) {
    void kvmppc_mpic_disconnect_vcpu(struct openpic *opp, struct kvm_vcpu *vcpu)
    {
    BUG_ON(!opp.dst[vcpu.arch.irq_cpu_id].vcpu);
    opp.dst[vcpu.arch.irq_cpu_id].vcpu = core::ptr::null_mut();
    }
//
// Return value:
// < 0   Interrupt was ignored (masked or not delivered for other reasons)
// = 0   Interrupt was coalesced (previous irq is still pending)
// > 0   Number of CPUs interrupt was delivered to
//
    static int mpic_set_irq(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level,
    bool line_status)
    {
    let mut irq: u32 = e.irqchip.pin;
    struct openpic *opp = kvm.arch.mpic;
    unsigned long flags;
    spin_lock_irqsave(&opp.lock, flags);
    openpic_set_irq(opp, irq, level);
    spin_unlock_irqrestore(&opp.lock, flags);
// All code paths we care about don't check for the return value
    return 0;
    }
    int kvm_set_msi(struct kvm_kernel_irq_routing_entry *e,
    struct kvm *kvm, int irq_source_id, int level, bool line_status)
    {
    struct openpic *opp = kvm.arch.mpic;
    unsigned long flags;
    spin_lock_irqsave(&opp.lock, flags);
//
// XXX We ignore the target address for now, as we only support
// a single MSI bank.
//
    openpic_msi_write(kvm.arch.mpic, MSIIR_OFFSET, e.msi.data);
    spin_unlock_irqrestore(&opp.lock, flags);
// All code paths we care about don't check for the return value
    return 0;
    }
    int kvm_set_routing_entry(struct kvm *kvm,
    struct kvm_kernel_irq_routing_entry *e,
    const struct kvm_irq_routing_entry *ue)
    {
    let mut r: c_int = -EINVAL;
    switch (ue.type) {
    case KVM_IRQ_ROUTING_IRQCHIP:
    e.set = mpic_set_irq;
    e.irqchip.irqchip = ue.u.irqchip.irqchip;
    e.irqchip.pin = ue.u.irqchip.pin;
    if (e.irqchip.pin >= KVM_IRQCHIP_NUM_PINS ||
    e.irqchip.irqchip >= KVM_NR_IRQCHIPS)
    goto out;
    break;
    case KVM_IRQ_ROUTING_MSI:
    e.set = kvm_set_msi;
    e.msi.address_lo = ue.u.msi.address_lo;
    e.msi.address_hi = ue.u.msi.address_hi;
    e.msi.data = ue.u.msi.data;
    break;
    default:
    goto out;
    }
    r = 0;
    out:
    return r;
    }
