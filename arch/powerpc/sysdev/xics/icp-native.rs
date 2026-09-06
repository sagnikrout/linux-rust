//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/icp-native.c
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
// Copyright 2011 IBM Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_ipl {
    union {
    pub word: u32,
    pub bytes: [u8; 4],
    pub xirr_poll: },
    union {
    pub word: u32,
    pub bytes: [u8; 4],
    pub xirr: },
    pub dummy: u32,
    union {
    pub word: u32,
    pub bytes: [u8; 4],
    pub qirr: },
    pub link_a: u32,
    pub link_b: u32,
    pub link_c: u32,
}

    static struct icp_ipl __iomem *icp_native_regs[NR_CPUS];
#[no_mangle]
pub unsafe extern "C" fn icp_native_get_xirr() -> c_uint {
    static inline unsigned int icp_native_get_xirr(void)
    {
    let mut cpu: c_int = smp_processor_id();
    unsigned int xirr;
// Handled an interrupt latched by KVM
    xirr = kvmppc_get_xics_latch();
    if (xirr)
    return xirr;
    return in_be32(&icp_native_regs[cpu].xirr.word);
    }
#[no_mangle]
pub unsafe extern "C" fn icp_native_set_xirr(value: c_uint) {
    static inline void icp_native_set_xirr(unsigned int value)
    {
    let mut cpu: c_int = smp_processor_id();
    out_be32(&icp_native_regs[cpu].xirr.word, value);
    }
#[no_mangle]
pub unsafe extern "C" fn icp_native_set_cppr(value: u8) {
    static inline void icp_native_set_cppr(u8 value)
    {
    let mut cpu: c_int = smp_processor_id();
    out_8(&icp_native_regs[cpu].xirr.bytes[0], value);
    }
#[no_mangle]
pub unsafe extern "C" fn icp_native_set_qirr(n_cpu: c_int, value: u8) {
    static inline void icp_native_set_qirr(int n_cpu, u8 value)
    {
    out_8(&icp_native_regs[n_cpu].qirr.bytes[0], value);
    }
#[no_mangle]
unsafe extern "C" fn icp_native_set_cpu_priority(cppr: c_uchar) {
    static void icp_native_set_cpu_priority(unsigned char cppr)
    {
    xics_set_base_cppr(cppr);
    icp_native_set_cppr(cppr);
    iosync();
    }
#[no_mangle]
pub unsafe extern "C" fn icp_native_eoi(d: *mut irq_data) {
    void icp_native_eoi(struct irq_data *d)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    iosync();
    icp_native_set_xirr((xics_pop_cppr() << 24) | hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn icp_native_teardown_cpu() {
    static void icp_native_teardown_cpu(void)
    {
    let mut cpu: c_int = smp_processor_id();
// Clear any pending IPI
    icp_native_set_qirr(cpu, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn icp_native_flush_ipi() {
    static void icp_native_flush_ipi(void)
    {
// We take the ipi irq but and never return so we
// need to EOI the IPI, but want to leave our priority 0
//
// should we check all the other interrupts too?
// should we be flagging idle loop instead?
// or creating some task to be scheduled?
//
    icp_native_set_xirr((0x00 << 24) | XICS_IPI);
    }
#[no_mangle]
unsafe extern "C" fn icp_native_get_irq() -> c_uint {
    static unsigned int icp_native_get_irq(void)
    {
    let mut xirr: c_uint = icp_native_get_xirr();
    let mut vec: c_uint = xirr & 0x00ffffff;
    unsigned int irq;
    if (vec == XICS_IRQ_SPURIOUS)
    return 0;
    irq = irq_find_mapping(xics_host, vec);
    if (likely(irq)) {
    xics_push_cppr(vec);
    return irq;
    }
// We don't have a linux mapping, so have rtas mask it.
    xics_mask_unknown_vec(vec);
// We might learn about it later, so EOI it
    icp_native_set_xirr(xirr);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn icp_native_cause_ipi(cpu: c_int) {
    static void icp_native_cause_ipi(int cpu)
    {
    kvmppc_set_host_ipi(cpu);
    icp_native_set_qirr(cpu, IPI_PRIORITY);
    }
//
// Called when an interrupt is received on an off-line CPU to
// clear the interrupt, so that the CPU can go back to nap mode.
//
#[no_mangle]
pub unsafe extern "C" fn icp_native_flush_interrupt() {
    void icp_native_flush_interrupt(void)
    {
    let mut xirr: c_uint = icp_native_get_xirr();
    let mut vec: c_uint = xirr & 0x00ffffff;
    if (vec == XICS_IRQ_SPURIOUS)
    return;
    if (vec == XICS_IPI) {
// Clear pending IPI
    let mut cpu: c_int = smp_processor_id();
    kvmppc_clear_host_ipi(cpu);
    icp_native_set_qirr(cpu, 0xff);
    } else {
    pr_err("XICS: hw interrupt 0x%x to offline cpu, disabling\n",
    vec);
    xics_mask_unknown_vec(vec);
    }
// EOI the interrupt
    icp_native_set_xirr(xirr);
    }
#[no_mangle]
pub unsafe extern "C" fn xics_wake_cpu(cpu: c_int) {
    void xics_wake_cpu(int cpu)
    {
    icp_native_set_qirr(cpu, IPI_PRIORITY);
    }
    EXPORT_SYMBOL_GPL(xics_wake_cpu);
#[no_mangle]
unsafe extern "C" fn icp_native_ipi_action(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t icp_native_ipi_action(int irq, void *dev_id)
    {
    let mut cpu: c_int = smp_processor_id();
    kvmppc_clear_host_ipi(cpu);
    icp_native_set_qirr(cpu, 0xff);
    return smp_ipi_demux();
    }

    static int __init icp_native_map_one_cpu(int hw_id, unsigned long addr,
    unsigned long size)
    {
    char *rname;
    int i, cpu = -1;
// This may look gross but it's good enough for now, we don't quite
// have a hard -> linux processor id matching.
//
    for_each_possible_cpu(i) {
    if (!cpu_present(i))
    continue;
    if (hw_id == get_hard_smp_processor_id(i)) {
    cpu = i;
    break;
    }
    }
// Fail, skip that CPU. Don't print, it's normal, some XICS come up
// with way more entries in there than you have CPUs
//
    if (cpu == -1)
    return 0;
    rname = kasprintf(GFP_KERNEL, "CPU %d [0x%x] Interrupt Presentation",
    cpu, hw_id);
    if (!rname)
    return -ENOMEM;
    if (!request_mem_region(addr, size, rname)) {
    pr_warn("icp_native: Could not reserve ICP MMIO for CPU %d, interrupt server #0x%x\n",
    cpu, hw_id);
    return -EBUSY;
    }
    icp_native_regs[cpu] = ioremap(addr, size);
    kvmppc_set_xics_phys(cpu, addr);
    if (!icp_native_regs[cpu]) {
    pr_warn("icp_native: Failed ioremap for CPU %d, interrupt server #0x%x, addr %#lx\n",
    cpu, hw_id, addr);
    release_mem_region(addr, size);
    return -ENOMEM;
    }
    return 0;
    }
    static int __init icp_native_init_one_node(struct device_node *np,
    unsigned int *indx)
    {
    unsigned int ilen;
    const __be32 *ireg;
    int i;
    int num_reg;
    let mut num_servers: c_int = 0;
// This code does the theorically broken assumption that the interrupt
// server numbers are the same as the hard CPU numbers.
// This happens to be the case so far but we are playing with fire...
// should be fixed one of these days. -BenH.
//
    ireg = of_get_property(np, "ibm,interrupt-server-ranges", &ilen);
// Do that ever happen ? we'll know soon enough... but even good'old
// f80 does have that property ..
//
    WARN_ON((ireg == core::ptr::null_mut()) || (ilen != 2*sizeof(u32)));
    if (ireg) {
// indx = of_read_number(ireg, 1);
    if (ilen >= 2*sizeof(u32))
    num_servers = of_read_number(ireg + 1, 1);
    }
    num_reg = of_address_count(np);
    if (num_servers && (num_servers != num_reg)) {
    pr_err("icp_native: ICP reg len (%d) != num servers (%d)",
    num_reg, num_servers);
    return -1;
    }
    for (i = 0; i < num_reg; i++) {
    struct resource r;
    int err;
    err = of_address_to_resource(np, i, &r);
    if (err) {
    pr_err("icp_native: Could not translate ICP MMIO"
    " for interrupt server 0x%x (%d)\n", *indx, err);
    return -1;
    }
    if (icp_native_map_one_cpu(*indx, r.start, resource_size(&r)))
    return -1;
    (*indx)++;
    }
    return 0;
    }
    static const struct icp_ops icp_native_ops = {
    .get_irq	= icp_native_get_irq,
    .eoi		= icp_native_eoi,
    .set_priority	= icp_native_set_cpu_priority,
    .teardown_cpu	= icp_native_teardown_cpu,
    .flush_ipi	= icp_native_flush_ipi,

    .ipi_action	= icp_native_ipi_action,
    .cause_ipi	= icp_native_cause_ipi,

    };
#[no_mangle]
pub unsafe extern "C" fn icp_native_init() -> int __init {
    int __init icp_native_init(void)
    {
    struct device_node *np;
    let mut indx: u32 = 0;
    let mut found: c_int = 0;
    for_each_compatible_node(np, core::ptr::null_mut(), "ibm,ppc-xicp")
    if (icp_native_init_one_node(np, &indx) == 0)
    found = 1;
    if (!found) {
    for_each_node_by_type(np,
    "PowerPC-External-Interrupt-Presentation") {
    if (icp_native_init_one_node(np, &indx) == 0)
    found = 1;
    }
    }
    if (found == 0)
    return -ENODEV;
    icp_ops = &icp_native_ops;
    return 0;
    }
