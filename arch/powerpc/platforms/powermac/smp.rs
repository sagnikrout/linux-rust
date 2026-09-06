//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/smp.c
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
// SMP support for power macintosh.
//
// We support both the old "powersurge" SMP architecture
// and the current Core99 (G4 PowerMac) machines.
//
// Note that we don't support the very first rev. of
// Apple/DayStar 2 CPUs board, the one with the funky
// watchdog. Hopefully, none of these should be there except
// maybe internally to Apple. I should probably still add some
// code to detect this card though and disable SMP. --BenH.
//
// Support Macintosh G4 SMP by Troy Benjegerdes (hozer@drgw.net)
// and Ben Herrenschmidt <benh@kernel.crashing.org>.
//
// Support for DayStar quad CPU cards
// Copyright (C) XLR8, Inc. 1994-2000
//

// Macro flag: #define DBG(fmt...)

    extern void __secondary_start_pmac_0(void);
    static void (*pmac_tb_freeze)(int freeze);
    static u64 timebase;
    static int tb_req;

//
// Powersurge (old powermac SMP) support.
//
// Addresses for powersurge registers
pub const HAMMERHEAD_BASE: c_uint = 0xf8000000;
pub const HHEAD_CONFIG: c_uint = 0x90;
pub const HHEAD_SEC_INTR: c_uint = 0xc0;
// register for interrupting the primary processor on the powersurge
// N.B. this is actually the ethernet ROM!
pub const PSURGE_PRI_INTR: c_uint = 0xf3019000;
// register for storing the start address for the secondary processor
// N.B. this is the PCI config space address register for the 1st bridge
pub const PSURGE_START: c_uint = 0xf2800000;
// Daystar/XLR8 4-CPU card
pub const PSURGE_QUAD_REG_ADDR: c_uint = 0xf8800000;
pub const PSURGE_QUAD_IRQ_SET: c_int = 0;
pub const PSURGE_QUAD_IRQ_CLR: c_int = 1;
pub const PSURGE_QUAD_IRQ_PRIMARY: c_int = 2;
pub const PSURGE_QUAD_CKSTOP_CTL: c_int = 3;
pub const PSURGE_QUAD_PRIMARY_ARB: c_int = 4;
pub const PSURGE_QUAD_BOARD_ID: c_int = 6;
pub const PSURGE_QUAD_WHICH_CPU: c_int = 7;
pub const PSURGE_QUAD_CKSTOP_RDBK: c_int = 8;
pub const PSURGE_QUAD_RESET_CTL: c_int = 11;

// virtual addresses for the above
    static volatile u8 __iomem *hhead_base;
    static volatile u8 __iomem *quad_base;
    static volatile u32 __iomem *psurge_pri_intr;
    static volatile u8 __iomem *psurge_sec_intr;
    static volatile u32 __iomem *psurge_start;
// values for psurge_type

pub const PSURGE_DUAL: c_int = 0;
pub const PSURGE_QUAD_OKEE: c_int = 1;
pub const PSURGE_QUAD_COTTON: c_int = 2;
pub const PSURGE_QUAD_ICEGRASS: c_int = 3;
// what sort of powersurge board we have
    let mut psurge_type: static int = PSURGE_NONE;
// irq for secondary cpus to report
    static struct irq_domain *psurge_host;
    int psurge_secondary_virq;
//
// Set and clear IPIs for powersurge.
//
#[no_mangle]
pub unsafe extern "C" fn psurge_set_ipi(cpu: c_int) {
    static inline void psurge_set_ipi(int cpu)
    {
    if (psurge_type == PSURGE_NONE)
    return;
    if (cpu == 0)
    in_be32(psurge_pri_intr);
#[no_mangle]
pub unsafe extern "C" fn if(PSURGE_DUAL: psurge_type ==) -> else {
    else if (psurge_type == PSURGE_DUAL)
    out_8(psurge_sec_intr, 0);
    else
    PSURGE_QUAD_OUT(PSURGE_QUAD_IRQ_SET, 1 << cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn psurge_clr_ipi(cpu: c_int) {
    static inline void psurge_clr_ipi(int cpu)
    {
    if (cpu > 0) {
    switch(psurge_type) {
    case PSURGE_DUAL:
    out_8(psurge_sec_intr, ~0);
    break;
    case PSURGE_NONE:
    break;
    default:
    PSURGE_QUAD_OUT(PSURGE_QUAD_IRQ_CLR, 1 << cpu);
    }
    }
    }
//
// On powersurge (old SMP powermac architecture) we don't have
// separate IPIs for separate messages like openpic does.  Instead
// use the generic demux helpers
// -- paulus.
//
#[no_mangle]
unsafe extern "C" fn psurge_ipi_intr(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t psurge_ipi_intr(int irq, void *d)
    {
    psurge_clr_ipi(smp_processor_id());
    smp_ipi_demux();
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_cause_ipi(cpu: c_int) {
    static void smp_psurge_cause_ipi(int cpu)
    {
    psurge_set_ipi(cpu);
    }
    static int psurge_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(virq, &dummy_irq_chip, handle_percpu_irq);
    return 0;
    }
    static const struct irq_domain_ops psurge_host_ops = {
    .map	= psurge_host_map,
    };
#[no_mangle]
unsafe extern "C" fn psurge_secondary_ipi_init() -> int __init {
    static int __init psurge_secondary_ipi_init(void)
    {
    let mut rc: c_int = -ENOMEM;
    psurge_host = irq_domain_create_nomap(core::ptr::null_mut(), ~0, &psurge_host_ops, core::ptr::null_mut());
    if (psurge_host)
    psurge_secondary_virq = irq_create_direct_mapping(psurge_host);
    if (psurge_secondary_virq)
    rc = request_irq(psurge_secondary_virq, psurge_ipi_intr,
    IRQF_PERCPU | IRQF_NO_THREAD, "IPI", core::ptr::null_mut());
    if (rc)
    pr_err("Failed to setup secondary cpu IPI\n");
    return rc;
    }
//
// Determine a quad card presence. We read the board ID register, we
// force the data bus to change to something else, and we read it again.
// It it's stable, then the register probably exist (ugh !)
//
#[no_mangle]
unsafe extern "C" fn psurge_quad_probe() -> int __init {
    static int __init psurge_quad_probe(void)
    {
    int type;
    unsigned int i;
    type = PSURGE_QUAD_IN(PSURGE_QUAD_BOARD_ID);
    if (type < PSURGE_QUAD_OKEE || type > PSURGE_QUAD_ICEGRASS
    || type != PSURGE_QUAD_IN(PSURGE_QUAD_BOARD_ID))
    return PSURGE_DUAL;
// looks OK, try a slightly more rigorous test
// bogus is not necessarily cacheline-aligned,
    though I don't suppose that really matters.  -- paulus */
    for (i = 0; i < 100; i++) {
    volatile u32 bogus[8];
    bogus[(0+i)%8] = 0x00000000;
    bogus[(1+i)%8] = 0x55555555;
    bogus[(2+i)%8] = 0xFFFFFFFF;
    bogus[(3+i)%8] = 0xAAAAAAAA;
    bogus[(4+i)%8] = 0x33333333;
    bogus[(5+i)%8] = 0xCCCCCCCC;
    bogus[(6+i)%8] = 0xCCCCCCCC;
    bogus[(7+i)%8] = 0x33333333;
    wmb();
    asm volatile("dcbf 0,%0" : : "r" (bogus) : "memory");
    mb();
    if (type != PSURGE_QUAD_IN(PSURGE_QUAD_BOARD_ID))
    return PSURGE_DUAL;
    }
    return type;
    }
#[no_mangle]
unsafe extern "C" fn psurge_quad_init() -> void __init {
    static void __init psurge_quad_init(void)
    {
    int procbits;
    if (ppc_md.progress) ppc_md.progress("psurge_quad_init", 0x351);
    procbits = ~PSURGE_QUAD_IN(PSURGE_QUAD_WHICH_CPU);
    if (psurge_type == PSURGE_QUAD_ICEGRASS)
    PSURGE_QUAD_BIS(PSURGE_QUAD_RESET_CTL, procbits);
    else
    PSURGE_QUAD_BIC(PSURGE_QUAD_CKSTOP_CTL, procbits);
    mdelay(33);
    out_8(psurge_sec_intr, ~0);
    PSURGE_QUAD_OUT(PSURGE_QUAD_IRQ_CLR, procbits);
    PSURGE_QUAD_BIS(PSURGE_QUAD_RESET_CTL, procbits);
    if (psurge_type != PSURGE_QUAD_ICEGRASS)
    PSURGE_QUAD_BIS(PSURGE_QUAD_CKSTOP_CTL, procbits);
    PSURGE_QUAD_BIC(PSURGE_QUAD_PRIMARY_ARB, procbits);
    mdelay(33);
    PSURGE_QUAD_BIC(PSURGE_QUAD_RESET_CTL, procbits);
    mdelay(33);
    PSURGE_QUAD_BIS(PSURGE_QUAD_PRIMARY_ARB, procbits);
    mdelay(33);
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_probe() -> void __init {
    static void __init smp_psurge_probe(void)
    {
    int i, ncpus;
    struct device_node *dn;
//
// The powersurge cpu board can be used in the generation
// of powermacs that have a socket for an upgradeable cpu card,
// including the 7500, 8500, 9500, 9600.
// The device tree doesn't tell you if you have 2 cpus because
// OF doesn't know anything about the 2nd processor.
// Instead we look for magic bits in magic registers,
// in the hammerhead memory controller in the case of the
// dual-cpu powersurge board.  -- paulus.
//
    dn = of_find_node_by_name(core::ptr::null_mut(), "hammerhead");
    if (dn == core::ptr::null_mut())
    return;
    of_node_put(dn);
    hhead_base = ioremap(HAMMERHEAD_BASE, 0x800);
    quad_base = ioremap(PSURGE_QUAD_REG_ADDR, 1024);
    psurge_sec_intr = hhead_base + HHEAD_SEC_INTR;
    psurge_type = psurge_quad_probe();
    if (psurge_type != PSURGE_DUAL) {
    psurge_quad_init();
// All released cards using this HW design have 4 CPUs
    ncpus = 4;
// No sure how timebase sync works on those, let's use SW
    smp_ops.give_timebase = smp_generic_give_timebase;
    smp_ops.take_timebase = smp_generic_take_timebase;
    } else {
    iounmap(quad_base);
    if ((in_8(hhead_base + HHEAD_CONFIG) & 0x02) == 0) {
// not a dual-cpu card
    iounmap(hhead_base);
    psurge_type = PSURGE_NONE;
    return;
    }
    ncpus = 2;
    }
    if (psurge_secondary_ipi_init())
    return;
    psurge_start = ioremap(PSURGE_START, 4);
    psurge_pri_intr = ioremap(PSURGE_PRI_INTR, 4);
// This is necessary because OF doesn't know about the
// secondary cpu(s), and thus there aren't nodes in the
// device tree for them, and smp_setup_cpu_maps hasn't
// set their bits in cpu_present_mask.
//
    if (ncpus > NR_CPUS)
    ncpus = NR_CPUS;
    for (i = 1; i < ncpus ; ++i)
    set_cpu_present(i, true);
    if (ppc_md.progress) ppc_md.progress("smp_psurge_probe - done", 0x352);
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_kick_cpu(nr: c_int) -> int __init {
    static int __init smp_psurge_kick_cpu(int nr)
    {
    let mut start: c_ulong = __pa(__secondary_start_pmac_0) + nr * 8;
    unsigned long a, flags;
    int i, j;
// Defining this here is evil ... but I prefer hiding that
// crap to avoid giving people ideas that they can do the
// same.
//
    extern volatile unsigned int cpu_callin_map[NR_CPUS];
// may need to flush here if secondary bats aren't setup
    for (a = KERNELBASE; a < KERNELBASE + 0x800000; a += 32)
    asm volatile("dcbf 0,%0" : : "r" (a) : "memory");
    asm volatile("sync");
    if (ppc_md.progress) ppc_md.progress("smp_psurge_kick_cpu", 0x353);
// This is going to freeze the timeebase, we disable interrupts
    local_irq_save(flags);
    out_be32(psurge_start, start);
    mb();
    psurge_set_ipi(nr);
//
// We can't use udelay here because the timebase is now frozen.
//
    for (i = 0; i < 2000; ++i)
    asm volatile("nop" : : : "memory");
    psurge_clr_ipi(nr);
//
// Also, because the timebase is frozen, we must not return to the
// caller which will try to do udelay's etc... Instead, we wait -here-
// for the CPU to callin.
//
    for (i = 0; i < 100000 && !cpu_callin_map[nr]; ++i) {
    for (j = 1; j < 10000; j++)
    asm volatile("nop" : : : "memory");
    asm volatile("sync" : : : "memory");
    }
    if (!cpu_callin_map[nr])
    goto stuck;
// And we do the TB sync here too for standard dual CPU cards
    if (psurge_type == PSURGE_DUAL) {
    while(!tb_req)
    barrier();
    tb_req = 0;
    mb();
    timebase = get_tb();
    mb();
    while (timebase)
    barrier();
    mb();
    }
    stuck:
// now interrupt the secondary, restarting both TBs
    if (psurge_type == PSURGE_DUAL)
    psurge_set_ipi(1);
    if (ppc_md.progress) ppc_md.progress("smp_psurge_kick_cpu - done", 0x354);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_setup_cpu(cpu_nr: c_int) -> void __init {
    static void __init smp_psurge_setup_cpu(int cpu_nr)
    {
    let mut flags: c_ulong = IRQF_PERCPU | IRQF_NO_THREAD;
    int irq;
    if (cpu_nr != 0 || !psurge_start)
    return;
// reset the entry point so if we get another intr we won't
// try to startup again
    out_be32(psurge_start, 0x100);
    irq = irq_create_mapping(core::ptr::null_mut(), 30);
    if (request_irq(irq, psurge_ipi_intr, flags, "primary IPI", core::ptr::null_mut()))
    printk(KERN_ERR "Couldn't get primary IPI interrupt");
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_take_timebase() -> void __init {
    static void __init smp_psurge_take_timebase(void)
    {
    if (psurge_type != PSURGE_DUAL)
    return;
    tb_req = 1;
    mb();
    while (!timebase)
    barrier();
    mb();
    set_tb(timebase >> 32, timebase & 0xffffffff);
    timebase = 0;
    mb();
    set_dec(tb_ticks_per_jiffy/2);
    }
#[no_mangle]
unsafe extern "C" fn smp_psurge_give_timebase() -> void __init {
    static void __init smp_psurge_give_timebase(void)
    {
// Nothing to do here
    }
// PowerSurge-style Macs
    struct smp_ops_t psurge_smp_ops = {
    .message_pass	= core::ptr::null_mut(),	/* Use smp_muxed_ipi_message_pass */
    .cause_ipi	= smp_psurge_cause_ipi,
    .cause_nmi_ipi	= core::ptr::null_mut(),
    .probe		= smp_psurge_probe,
    .kick_cpu	= smp_psurge_kick_cpu,
    .setup_cpu	= smp_psurge_setup_cpu,
    .give_timebase	= smp_psurge_give_timebase,
    .take_timebase	= smp_psurge_take_timebase,
    };

//
// Core 99 and later support
//
#[no_mangle]
unsafe extern "C" fn smp_core99_give_timebase() {
    static void smp_core99_give_timebase(void)
    {
    unsigned long flags;
    local_irq_save(flags);
    while(!tb_req)
    barrier();
    tb_req = 0;
    (*pmac_tb_freeze)(1);
    mb();
    timebase = get_tb();
    mb();
    while (timebase)
    barrier();
    mb();
    (*pmac_tb_freeze)(0);
    mb();
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_take_timebase() {
    static void smp_core99_take_timebase(void)
    {
    unsigned long flags;
    local_irq_save(flags);
    tb_req = 1;
    mb();
    while (!timebase)
    barrier();
    mb();
    set_tb(timebase >> 32, timebase & 0xffffffff);
    timebase = 0;
    mb();
    local_irq_restore(flags);
    }

//
// G5s enable/disable the timebase via an i2c-connected clock chip.
//
    static struct pmac_i2c_bus *pmac_tb_clock_chip_host;
    static u8 pmac_tb_pulsar_addr;
#[no_mangle]
unsafe extern "C" fn smp_core99_cypress_tb_freeze(freeze: c_int) {
    static void smp_core99_cypress_tb_freeze(int freeze)
    {
    u8 data;
    int rc;
// Strangely, the device-tree says address is 0xd2, but darwin
// accesses 0xd0 ...
//
    pmac_i2c_setmode(pmac_tb_clock_chip_host,
    pmac_i2c_mode_combined);
    rc = pmac_i2c_xfer(pmac_tb_clock_chip_host,
    0xd0 | pmac_i2c_read,
    1, 0x81, &data, 1);
    if (rc != 0)
    goto bail;
    data = (data & 0xf3) | (freeze ? 0x00 : 0x0c);
    pmac_i2c_setmode(pmac_tb_clock_chip_host, pmac_i2c_mode_stdsub);
    rc = pmac_i2c_xfer(pmac_tb_clock_chip_host,
    0xd0 | pmac_i2c_write,
    1, 0x81, &data, 1);
    bail:
    if (rc != 0) {
    printk("Cypress Timebase %s rc: %d\n",
    freeze ? "freeze" : "unfreeze", rc);
    panic("Timebase freeze failed !\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_pulsar_tb_freeze(freeze: c_int) {
    static void smp_core99_pulsar_tb_freeze(int freeze)
    {
    u8 data;
    int rc;
    pmac_i2c_setmode(pmac_tb_clock_chip_host,
    pmac_i2c_mode_combined);
    rc = pmac_i2c_xfer(pmac_tb_clock_chip_host,
    pmac_tb_pulsar_addr | pmac_i2c_read,
    1, 0x2e, &data, 1);
    if (rc != 0)
    goto bail;
    data = (data & 0x88) | (freeze ? 0x11 : 0x22);
    pmac_i2c_setmode(pmac_tb_clock_chip_host, pmac_i2c_mode_stdsub);
    rc = pmac_i2c_xfer(pmac_tb_clock_chip_host,
    pmac_tb_pulsar_addr | pmac_i2c_write,
    1, 0x2e, &data, 1);
    bail:
    if (rc != 0) {
    printk(KERN_ERR "Pulsar Timebase %s rc: %d\n",
    freeze ? "freeze" : "unfreeze", rc);
    panic("Timebase freeze failed !\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_setup_i2c_hwsync(ncpus: c_int) -> void __init {
    static void __init smp_core99_setup_i2c_hwsync(int ncpus)
    {
    struct device_node *cc = core::ptr::null_mut();
    struct device_node *p;
    const char *name = core::ptr::null_mut();
    const u32 *reg;
    int ok;
// Look for the clock chip
    for_each_node_by_name(cc, "i2c-hwclock") {
    p = of_get_parent(cc);
    ok = p && of_device_is_compatible(p, "uni-n-i2c");
    of_node_put(p);
    if (!ok)
    continue;
    pmac_tb_clock_chip_host = pmac_i2c_find_bus(cc);
    if (pmac_tb_clock_chip_host == core::ptr::null_mut())
    continue;
    reg = of_get_property(cc, "reg", core::ptr::null_mut());
    if (reg == core::ptr::null_mut())
    continue;
    switch (*reg) {
    case 0xd2:
    if (of_device_is_compatible(cc,"pulsar-legacy-slewing")) {
    pmac_tb_freeze = smp_core99_pulsar_tb_freeze;
    pmac_tb_pulsar_addr = 0xd2;
    name = "Pulsar";
    } else if (of_device_is_compatible(cc, "cy28508")) {
    pmac_tb_freeze = smp_core99_cypress_tb_freeze;
    name = "Cypress";
    }
    break;
    case 0xd4:
    pmac_tb_freeze = smp_core99_pulsar_tb_freeze;
    pmac_tb_pulsar_addr = 0xd4;
    name = "Pulsar";
    break;
    }
    if (pmac_tb_freeze != core::ptr::null_mut()) {
    of_node_put(cc);
    break;
    }
    }
    if (pmac_tb_freeze != core::ptr::null_mut()) {
// Open i2c bus for synchronous access
    if (pmac_i2c_open(pmac_tb_clock_chip_host, 1)) {
    printk(KERN_ERR "Failed top open i2c bus for clock"
    " sync, fallback to software sync !\n");
    goto no_i2c_sync;
    }
    printk(KERN_INFO "Processor timebase sync using %s i2c clock\n",
    name);
    return;
    }
    no_i2c_sync:
    pmac_tb_freeze = core::ptr::null_mut();
    pmac_tb_clock_chip_host = core::ptr::null_mut();
    }
//
// Newer G5s uses a platform function
//
#[no_mangle]
unsafe extern "C" fn smp_core99_pfunc_tb_freeze(freeze: c_int) {
    static void smp_core99_pfunc_tb_freeze(int freeze)
    {
    struct device_node *cpus;
    struct pmf_args args;
    cpus = of_find_node_by_path("/cpus");
    BUG_ON(cpus == core::ptr::null_mut());
    args.count = 1;
    args.u[0].v = !freeze;
    pmf_call_function(cpus, "cpu-timebase", &args);
    of_node_put(cpus);
    }

//
// SMP G4 use a GPIO to enable/disable the timebase.
//
    static unsigned int core99_tb_gpio;	/* Timebase freeze GPIO */
#[no_mangle]
unsafe extern "C" fn smp_core99_gpio_tb_freeze(freeze: c_int) {
    static void smp_core99_gpio_tb_freeze(int freeze)
    {
    if (freeze)
    pmac_call_feature(PMAC_FTR_WRITE_GPIO, core::ptr::null_mut(), core99_tb_gpio, 4);
    else
    pmac_call_feature(PMAC_FTR_WRITE_GPIO, core::ptr::null_mut(), core99_tb_gpio, 0);
    pmac_call_feature(PMAC_FTR_READ_GPIO, core::ptr::null_mut(), core99_tb_gpio, 0);
    }

#[no_mangle]
unsafe extern "C" fn core99_init_caches(cpu: c_int) {
    static void core99_init_caches(int cpu)
    {

// L2 and L3 cache settings to pass from CPU0 to CPU1 on G4 cpus
    static long int core99_l2_cache;
    static long int core99_l3_cache;
    if (!cpu_has_feature(CPU_FTR_L2CR))
    return;
    if (cpu == 0) {
    core99_l2_cache = _get_L2CR();
    printk("CPU0: L2CR is %lx\n", core99_l2_cache);
    } else {
    printk("CPU%d: L2CR was %lx\n", cpu, _get_L2CR());
    _set_L2CR(0);
    _set_L2CR(core99_l2_cache);
    printk("CPU%d: L2CR set to %lx\n", cpu, core99_l2_cache);
    }
    if (!cpu_has_feature(CPU_FTR_L3CR))
    return;
    if (cpu == 0){
    core99_l3_cache = _get_L3CR();
    printk("CPU0: L3CR is %lx\n", core99_l3_cache);
    } else {
    printk("CPU%d: L3CR was %lx\n", cpu, _get_L3CR());
    _set_L3CR(0);
    _set_L3CR(core99_l3_cache);
    printk("CPU%d: L3CR set to %lx\n", cpu, core99_l3_cache);
    }

    }
#[no_mangle]
unsafe extern "C" fn smp_core99_setup(ncpus: c_int) -> void __init {
    static void __init smp_core99_setup(int ncpus)
    {

// i2c based HW sync on some G5s
    if (of_machine_is_compatible("PowerMac7,2") ||
    of_machine_is_compatible("PowerMac7,3") ||
    of_machine_is_compatible("RackMac3,1"))
    smp_core99_setup_i2c_hwsync(ncpus);
// pfunc based HW sync on recent G5s
    if (pmac_tb_freeze == core::ptr::null_mut()) {
    struct device_node *cpus =
    of_find_node_by_path("/cpus");
    if (cpus &&
    of_property_read_bool(cpus, "platform-cpu-timebase")) {
    pmac_tb_freeze = smp_core99_pfunc_tb_freeze;
    printk(KERN_INFO "Processor timebase sync using"
    " platform function\n");
    }
    of_node_put(cpus);
    }

// GPIO based HW sync on ppc32 Core99
    if (pmac_tb_freeze == core::ptr::null_mut() && !of_machine_is_compatible("MacRISC4")) {
    struct device_node *cpu;
    const u32 *tbprop = core::ptr::null_mut();
    core99_tb_gpio = KL_GPIO_TB_ENABLE;	/* default value */
    cpu = of_find_node_by_type(core::ptr::null_mut(), "cpu");
    if (cpu != core::ptr::null_mut()) {
    tbprop = of_get_property(cpu, "timebase-enable", core::ptr::null_mut());
    if (tbprop)
    core99_tb_gpio = *tbprop;
    of_node_put(cpu);
    }
    pmac_tb_freeze = smp_core99_gpio_tb_freeze;
    printk(KERN_INFO "Processor timebase sync using"
    " GPIO 0x%02x\n", core99_tb_gpio);
    }

// No timebase sync, fallback to software
    if (pmac_tb_freeze == core::ptr::null_mut()) {
    smp_ops.give_timebase = smp_generic_give_timebase;
    smp_ops.take_timebase = smp_generic_take_timebase;
    printk(KERN_INFO "Processor timebase sync using software\n");
    }

    {
    int i;
// XXX should get this from reg properties
    for (i = 1; i < ncpus; ++i)
    set_hard_smp_processor_id(i, i);
    }

// 32 bits SMP can't NAP
    if (!of_machine_is_compatible("MacRISC4"))
    powersave_nap = 0;
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_probe() -> void __init {
    static void __init smp_core99_probe(void)
    {
    struct device_node *cpus;
    let mut ncpus: c_int = 0;
    if (ppc_md.progress) ppc_md.progress("smp_core99_probe", 0x345);
// Count CPUs in the device-tree
    for_each_node_by_type(cpus, "cpu")
    ++ncpus;
    printk(KERN_INFO "PowerMac SMP probe found %d cpus\n", ncpus);
// Nothing more to do if less than 2 of them
    if (ncpus <= 1)
    return;
// We need to perform some early initialisations before we can start
// setting up SMP as we are running before initcalls
//
    pmac_pfunc_base_install();
    pmac_i2c_init();
// Setup various bits like timebase sync method, ability to nap, ...
    smp_core99_setup(ncpus);
// Install IPIs
    mpic_request_ipis();
// Collect l2cr and l3cr values from CPU 0
    core99_init_caches(0);
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_kick_cpu(nr: c_int) -> c_int {
    static int smp_core99_kick_cpu(int nr)
    {
    unsigned int save_vector;
    unsigned long target, flags;
    unsigned int *vector = (unsigned int *)(PAGE_OFFSET+0x100);
    if (nr < 0 || nr > 3)
    return -ENOENT;
    if (ppc_md.progress)
    ppc_md.progress("smp_core99_kick_cpu", 0x346);
    local_irq_save(flags);
// Save reset vector
    save_vector = *vector;
// Setup fake reset vector that does
// b __secondary_start_pmac_0 + nr*8
//
    target = (unsigned long) __secondary_start_pmac_0 + nr * 8;
    patch_branch(vector, target, BRANCH_SET_LINK);
// Put some life in our friend
    pmac_call_feature(PMAC_FTR_RESET_CPU, core::ptr::null_mut(), nr, 0);
// FIXME: We wait a bit for the CPU to take the exception, I should
// instead wait for the entry code to set something for me. Well,
// ideally, all that crap will be done in prom.c and the CPU left
// in a RAM-based wait loop like CHRP.
//
    mdelay(1);
// Restore our exception vector
    patch_uint(vector, save_vector);
    local_irq_restore(flags);
    if (ppc_md.progress) ppc_md.progress("smp_core99_kick_cpu done", 0x347);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_setup_cpu(cpu_nr: c_int) {
    static void smp_core99_setup_cpu(int cpu_nr)
    {
// Setup L2/L3
    if (cpu_nr != 0)
    core99_init_caches(cpu_nr);
// Setup openpic
    mpic_setup_this_cpu();
    }

    static unsigned int smp_core99_host_open;
#[no_mangle]
unsafe extern "C" fn smp_core99_cpu_prepare(cpu: c_uint) -> c_int {
    static int smp_core99_cpu_prepare(unsigned int cpu)
    {
    int rc;
// Open i2c bus if it was used for tb sync
    if (pmac_tb_clock_chip_host && !smp_core99_host_open) {
    rc = pmac_i2c_open(pmac_tb_clock_chip_host, 1);
    if (rc) {
    pr_err("Failed to open i2c bus for time sync\n");
    return notifier_from_errno(rc);
    }
    smp_core99_host_open = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smp_core99_cpu_online(cpu: c_uint) -> c_int {
    static int smp_core99_cpu_online(unsigned int cpu)
    {
// Close i2c bus if it was used for tb sync
    if (pmac_tb_clock_chip_host && smp_core99_host_open) {
    pmac_i2c_close(pmac_tb_clock_chip_host);
    smp_core99_host_open = 0;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn smp_core99_bringup_done() -> void __init {
    static void __init smp_core99_bringup_done(void)
    {
// Close i2c bus if it was used for tb sync
    if (pmac_tb_clock_chip_host)
    pmac_i2c_close(pmac_tb_clock_chip_host);
// If we didn't start the second CPU, we must take
// it off the bus.
//
    if (of_machine_is_compatible("MacRISC4") &&
    num_online_cpus() < 2) {
    set_cpu_present(1, false);
    g5_phy_disable_cpu1();
    }

    cpuhp_setup_state_nocalls(CPUHP_POWERPC_PMAC_PREPARE,
    "powerpc/pmac:prepare", smp_core99_cpu_prepare,
    core::ptr::null_mut());
    cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "powerpc/pmac:online",
    smp_core99_cpu_online, core::ptr::null_mut());

    if (ppc_md.progress)
    ppc_md.progress("smp_core99_bringup_done", 0x349);
    }

#[no_mangle]
unsafe extern "C" fn smp_core99_cpu_disable() -> c_int {
    static int smp_core99_cpu_disable(void)
    {
    let mut rc: c_int = generic_cpu_disable();
    if (rc)
    return rc;
    mpic_cpu_set_priority(0xf);
    cleanup_cpu_mmu_context();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pmac_cpu_offline_self() {
    static void pmac_cpu_offline_self(void)
    {
    let mut cpu: c_int = smp_processor_id();
    local_irq_disable();
    idle_task_exit();
    pr_debug("CPU%d offline\n", cpu);
    generic_set_cpu_dead(cpu);
    smp_wmb();
    mb();
    low_cpu_offline_self();
    }

#[no_mangle]
unsafe extern "C" fn pmac_cpu_offline_self() {
    static void pmac_cpu_offline_self(void)
    {
    let mut cpu: c_int = smp_processor_id();
    local_irq_disable();
    idle_task_exit();
//
// turn off as much as possible, we'll be
// kicked out as this will only be invoked
// on core99 platforms for now ...
//
    printk(KERN_INFO "CPU#%d offline\n", cpu);
    generic_set_cpu_dead(cpu);
    smp_wmb();
//
// Re-enable interrupts. The NAP code needs to enable them
// anyways, do it now so we deal with the case where one already
// happened while soft-disabled.
// We shouldn't get any external interrupts, only decrementer, and the
// decrementer handler is safe for use on offline CPUs
//
    local_irq_enable();
    while (1) {
// let's not take timer interrupts too often ...
    set_dec(0x7fffffff);
// Enter NAP mode
    power4_idle();
    }
    }

// Core99 Macs (dual G4s and G5s)
    static struct smp_ops_t core99_smp_ops = {
    .message_pass	= smp_mpic_message_pass,
    .probe		= smp_core99_probe,

    .bringup_done	= smp_core99_bringup_done,

    .kick_cpu	= smp_core99_kick_cpu,
    .setup_cpu	= smp_core99_setup_cpu,
    .give_timebase	= smp_core99_give_timebase,
    .take_timebase	= smp_core99_take_timebase,

    .cpu_disable	= smp_core99_cpu_disable,
    .cpu_die	= generic_cpu_die,

    };
#[no_mangle]
pub unsafe extern "C" fn pmac_setup_smp() -> void __init {
    void __init pmac_setup_smp(void)
    {
    struct device_node *np;
// Check for Core99
    np = of_find_node_by_name(core::ptr::null_mut(), "uni-n");
    if (!np)
    np = of_find_node_by_name(core::ptr::null_mut(), "u3");
    if (!np)
    np = of_find_node_by_name(core::ptr::null_mut(), "u4");
    if (np) {
    of_node_put(np);
    smp_ops = &core99_smp_ops;
    }

    else {
// We have to set bits in cpu_possible_mask here since the
// secondary CPU(s) aren't in the device tree. Various
// things won't be initialized for CPUs not in the possible
// map, so we really need to fix it up here.
//
    int cpu;
    for (cpu = 1; cpu < 4 && cpu < NR_CPUS; ++cpu)
    set_cpu_possible(cpu, true);
    smp_ops = &psurge_smp_ops;
    }

    smp_ops.cpu_offline_self = pmac_cpu_offline_self;

    }
