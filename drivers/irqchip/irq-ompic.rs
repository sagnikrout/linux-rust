//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ompic.c
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
// Open Multi-Processor Interrupt Controller driver
//
// Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
// Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// The ompic device handles IPI communication between cores in multi-core
// OpenRISC systems.
//
// Registers
//
// For each CPU the ompic has 2 registers. The control register for sending
// and acking IPIs and the status register for receiving IPIs. The register
// layouts are as follows:
//
// Control register
// +---------+---------+----------+---------+
// | 31      | 30      | 29 .. 16 | 15 .. 0 |
// ----------+---------+----------+----------
// | IRQ ACK | IRQ GEN | DST CORE | DATA    |
// +---------+---------+----------+---------+
//
// Status register
// +----------+-------------+----------+---------+
// | 31       | 30          | 29 .. 16 | 15 .. 0 |
// -----------+-------------+----------+---------+
// | Reserved | IRQ Pending | SRC CORE | DATA    |
// +----------+-------------+----------+---------+
//
// Architecture
//
// - The ompic generates a level interrupt to the CPU PIC when a message is
// ready.  Messages are delivered via the memory bus.
// - The ompic does not have any interrupt input lines.
// - The ompic is wired to the same irq line on each core.
// - Devices are wired to the same irq line on each core.
//
// +---------+                         +---------+
// | CPU     |                         | CPU     |
// |  Core 0 |<==\ (memory access) /==>|  Core 1 |
// |  [ PIC ]|   |                 |   |  [ PIC ]|
// +----^-^--+   |                 |   +----^-^--+
// | |      v                 v        | |
// <====|=|=================================|=|==> (memory bus)
// | |      ^                  ^       | |
// (ipi  | +------|---------+--------|-------|-+ (device irq)
// irq  |        |         |        |       |
// core0)| +------|---------|--------|-------+ (ipi irq core1)
// | |      |         |        |
// +----o-o-+    |    +--------+    |
// | ompic  |<===/    | Device |<===
// |  IPI   |         +--------+
// +--------+
//

pub const OMPIC_CPUBYTES: c_int = 8;

    DEFINE_PER_CPU(unsigned long, ops);
    static void __iomem *ompic_base;
    static DEFINE_PER_CPU_READ_MOSTLY(int, ipi_dummy_dev);
#[no_mangle]
pub unsafe extern "C" fn ompic_readreg(base: *mut void __iomem, offset: loff_t) -> u32 {
    static inline u32 ompic_readreg(void __iomem *base, loff_t offset)
    {
    return ioread32be(base + offset);
    }
#[no_mangle]
unsafe extern "C" fn ompic_writereg(base: *mut void __iomem, offset: loff_t, data: u32) {
    static void ompic_writereg(void __iomem *base, loff_t offset, u32 data)
    {
    iowrite32be(data, base + offset);
    }
    static void ompic_raise_softirq(const struct cpumask *mask,
    unsigned int ipi_msg)
    {
    unsigned int dst_cpu;
    let mut src_cpu: c_uint = smp_processor_id();
    for_each_cpu(dst_cpu, mask) {
    set_bit(ipi_msg, &per_cpu(ops, dst_cpu));
//
// On OpenRISC the atomic set_bit() call implies a memory
// barrier.  Otherwise we would need: smp_wmb(); paired
// with the read in ompic_ipi_handler.
//
    ompic_writereg(ompic_base, OMPIC_CTRL(src_cpu),
    OMPIC_CTRL_IRQ_GEN |
    OMPIC_CTRL_DST(dst_cpu) |
    OMPIC_DATA(1));
    }
    }
#[no_mangle]
unsafe extern "C" fn ompic_ipi_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ompic_ipi_handler(int irq, void *dev_id)
    {
    let mut cpu: c_uint = smp_processor_id();
    unsigned long *pending_ops = &per_cpu(ops, cpu);
    unsigned long ops;
    ompic_writereg(ompic_base, OMPIC_CTRL(cpu), OMPIC_CTRL_IRQ_ACK);
    while ((ops = xchg(pending_ops, 0)) != 0) {
//
// On OpenRISC the atomic xchg() call implies a memory
// barrier.  Otherwise we may need an smp_rmb(); paired
// with the write in ompic_raise_softirq.
//
    do {
    unsigned long ipi_msg;
    ipi_msg = __ffs(ops);
    ops &= ~(1UL << ipi_msg);
    handle_IPI(ipi_msg);
    } while (ops);
    }
    return IRQ_HANDLED;
    }
    static int __init ompic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    struct resource res;
    int irq;
    int ret;
// Validate the DT
    if (ompic_base) {
    pr_err("ompic: duplicate ompic's are not supported");
    return -EEXIST;
    }
    if (of_address_to_resource(node, 0, &res)) {
    pr_err("ompic: reg property requires an address and size");
    return -EINVAL;
    }
    if (resource_size(&res) < (num_possible_cpus() * OMPIC_CPUBYTES)) {
    pr_err("ompic: reg size, currently %d must be at least %d",
    resource_size(&res),
    (num_possible_cpus() * OMPIC_CPUBYTES));
    return -EINVAL;
    }
// Setup the device
    ompic_base = ioremap(res.start, resource_size(&res));
    if (!ompic_base) {
    pr_err("ompic: unable to map registers");
    return -ENOMEM;
    }
    irq = irq_of_parse_and_map(node, 0);
    if (irq <= 0) {
    pr_err("ompic: unable to parse device irq");
    ret = -EINVAL;
    goto out_unmap;
    }
    irq_set_percpu_devid(irq);
    ret = request_percpu_irq(irq, ompic_ipi_handler, "ompic_ipi",
    &ipi_dummy_dev);
    if (ret) {
    pr_err("ompic: failed to request irq %d, error: %d",
    irq, ret);
    goto out_irq_disp;
    }
    set_smp_cross_call(ompic_raise_softirq, irq);
    return 0;
    out_irq_disp:
    irq_dispose_mapping(irq);
    out_unmap:
    iounmap(ompic_base);
    ompic_base = core::ptr::null_mut();
    return ret;
    }
    IRQCHIP_DECLARE(ompic, "openrisc,ompic", ompic_of_init);
