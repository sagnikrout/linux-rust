//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/interrupt.c
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
// PS3 interrupt routines.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
//

//
// struct ps3_bmp - a per cpu irq status and mask bitmap structure
// @status: 256 bit status bitmap indexed by plug
// @unused_1: Alignment
// @mask: 256 bit mask bitmap indexed by plug
// @unused_2: Alignment
//
// The HV maintains per SMT thread mappings of HV outlet to HV plug on
// behalf of the guest.  These mappings are implemented as 256 bit guest
// supplied bitmaps indexed by plug number.  The addresses of the bitmaps
// are registered with the HV through lv1_configure_irq_state_bitmap().
// The HV requires that the 512 bits of status + mask not cross a page
// boundary.  PS3_BMP_MINALIGN is used to define this minimal 64 byte
// alignment.
//
// The HV supports 256 plugs per thread, assigned as {0..255}, for a total
// of 512 plugs supported on a processor.  To simplify the logic this
// implementation equates HV plug value to Linux virq value, constrains each
// interrupt to have a system wide unique plug number, and limits the range
// of the plug values to map into the first dword of the bitmaps.  This
// gives a usable range of plug values of  {NR_IRQS_LEGACY..63}.  Note
// that there is no constraint on how many in this set an individual thread
// can acquire.
//
// The mask is declared as unsigned long so we can use set/clear_bit on it.
//
pub const PS3_BMP_MINALIGN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_bmp {
    struct {
    pub status: u64,
    pub unused_1: [u64; 3],
    pub mask: c_ulong,
    pub unused_2: [u64; 3],
}

    };
//
// struct ps3_private - a per cpu data structure
// @bmp: ps3_bmp structure
// @bmp_lock: Synchronize access to bmp.
// @ipi_debug_brk_mask: Mask for debug break IPIs
// @ppe_id: HV logical_ppe_id
// @thread_id: HV thread_id
// @ipi_mask: Mask of IPI virqs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_private {
// C attribute field omitted
    pub bmp_lock: spinlock_t,
    pub ppe_id: u64,
    pub thread_id: u64,
    pub ipi_debug_brk_mask: c_ulong,
    pub ipi_mask: c_ulong,
}

    static DEFINE_PER_CPU(struct ps3_private, ps3_private);
//
// ps3_chip_mask - Set an interrupt mask bit in ps3_bmp.
// @virq: The assigned Linux virq.
//
// Sets ps3_bmp.mask and calls lv1_did_update_interrupt_mask().
//
#[no_mangle]
unsafe extern "C" fn ps3_chip_mask(d: *mut irq_data) {
    static void ps3_chip_mask(struct irq_data *d)
    {
    struct ps3_private *pd = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    DBG("%s:%d: thread_id %llu, virq %d\n", __func__, __LINE__,
    pd.thread_id, d.irq);
    local_irq_save(flags);
    clear_bit(63 - d.irq, &pd.bmp.mask);
    lv1_did_update_interrupt_mask(pd.ppe_id, pd.thread_id);
    local_irq_restore(flags);
    }
//
// ps3_chip_unmask - Clear an interrupt mask bit in ps3_bmp.
// @virq: The assigned Linux virq.
//
// Clears ps3_bmp.mask and calls lv1_did_update_interrupt_mask().
//
#[no_mangle]
unsafe extern "C" fn ps3_chip_unmask(d: *mut irq_data) {
    static void ps3_chip_unmask(struct irq_data *d)
    {
    struct ps3_private *pd = irq_data_get_irq_chip_data(d);
    unsigned long flags;
    DBG("%s:%d: thread_id %llu, virq %d\n", __func__, __LINE__,
    pd.thread_id, d.irq);
    local_irq_save(flags);
    set_bit(63 - d.irq, &pd.bmp.mask);
    lv1_did_update_interrupt_mask(pd.ppe_id, pd.thread_id);
    local_irq_restore(flags);
    }
//
// ps3_chip_eoi - HV end-of-interrupt.
// @virq: The assigned Linux virq.
//
// Calls lv1_end_of_interrupt_ext().
//
#[no_mangle]
unsafe extern "C" fn ps3_chip_eoi(d: *mut irq_data) {
    static void ps3_chip_eoi(struct irq_data *d)
    {
    const struct ps3_private *pd = irq_data_get_irq_chip_data(d);
// non-IPIs are EOIed here.
    if (!test_bit(63 - d.irq, &pd.ipi_mask))
    lv1_end_of_interrupt_ext(pd.ppe_id, pd.thread_id, d.irq);
    }
//
// ps3_irq_chip - Represents the ps3_bmp as a Linux struct irq_chip.
//
    static struct irq_chip ps3_irq_chip = {
    .name = "ps3",
    .irq_mask = ps3_chip_mask,
    .irq_unmask = ps3_chip_unmask,
    .irq_eoi = ps3_chip_eoi,
    };
//
// ps3_virq_setup - virq related setup.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @outlet: The HV outlet from the various create outlet routines.
// @virq: The assigned Linux virq.
//
// Calls irq_create_mapping() to get a virq and sets the chip data to
// ps3_private data.
//
    static int ps3_virq_setup(enum ps3_cpu_binding cpu, unsigned long outlet,
    unsigned int *virq)
    {
    int result;
    struct ps3_private *pd;
// This defines the default interrupt distribution policy.
    if (cpu == PS3_BINDING_CPU_ANY)
    cpu = 0;
    pd = &per_cpu(ps3_private, cpu);
// virq = irq_create_mapping(NULL, outlet);
    if (!*virq) {
    FAIL("%s:%d: irq_create_mapping failed: outlet %lu\n",
    __func__, __LINE__, outlet);
    result = -ENOMEM;
    goto fail_create;
    }
    DBG("%s:%d: outlet %lu => cpu %u, virq %u\n", __func__, __LINE__,
    outlet, cpu, *virq);
    result = irq_set_chip_data(*virq, pd);
    if (result) {
    FAIL("%s:%d: irq_set_chip_data failed\n",
    __func__, __LINE__);
    goto fail_set;
    }
    ps3_chip_mask(irq_get_irq_data(*virq));
    return result;
    fail_set:
    irq_dispose_mapping(*virq);
    fail_create:
    return result;
    }
//
// ps3_virq_destroy - virq related teardown.
// @virq: The assigned Linux virq.
//
// Clears chip data and calls irq_dispose_mapping() for the virq.
//
#[no_mangle]
unsafe extern "C" fn ps3_virq_destroy(virq: c_uint) -> c_int {
    static int ps3_virq_destroy(unsigned int virq)
    {
    const struct ps3_private *pd = irq_get_chip_data(virq);
    DBG("%s:%d: ppe_id %llu, thread_id %llu, virq %u\n", __func__,
    __LINE__, pd.ppe_id, pd.thread_id, virq);
    irq_set_chip_data(virq, core::ptr::null_mut());
    irq_dispose_mapping(virq);
    DBG("%s:%d <-\n", __func__, __LINE__);
    return 0;
    }
//
// ps3_irq_plug_setup - Generic outlet and virq related setup.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @outlet: The HV outlet from the various create outlet routines.
// @virq: The assigned Linux virq.
//
// Sets up virq and connects the irq plug.
//
    int ps3_irq_plug_setup(enum ps3_cpu_binding cpu, unsigned long outlet,
    unsigned int *virq)
    {
    int result;
    struct ps3_private *pd;
    result = ps3_virq_setup(cpu, outlet, virq);
    if (result) {
    FAIL("%s:%d: ps3_virq_setup failed\n", __func__, __LINE__);
    goto fail_setup;
    }
    pd = irq_get_chip_data(*virq);
// Binds outlet to cpu + virq.
    result = lv1_connect_irq_plug_ext(pd.ppe_id, pd.thread_id, *virq,
    outlet, 0);
    if (result) {
    FAIL("%s:%d: lv1_connect_irq_plug_ext failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    result = -EPERM;
    goto fail_connect;
    }
    return result;
    fail_connect:
    ps3_virq_destroy(*virq);
    fail_setup:
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_irq_plug_setup);
//
// ps3_irq_plug_destroy - Generic outlet and virq related teardown.
// @virq: The assigned Linux virq.
//
// Disconnects the irq plug and tears down virq.
// Do not call for system bus event interrupts setup with
// ps3_sb_event_receive_port_setup().
//
#[no_mangle]
pub unsafe extern "C" fn ps3_irq_plug_destroy(virq: c_uint) -> c_int {
    int ps3_irq_plug_destroy(unsigned int virq)
    {
    int result;
    const struct ps3_private *pd = irq_get_chip_data(virq);
    DBG("%s:%d: ppe_id %llu, thread_id %llu, virq %u\n", __func__,
    __LINE__, pd.ppe_id, pd.thread_id, virq);
    ps3_chip_mask(irq_get_irq_data(virq));
    result = lv1_disconnect_irq_plug_ext(pd.ppe_id, pd.thread_id, virq);
    if (result)
    FAIL("%s:%d: lv1_disconnect_irq_plug_ext failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    ps3_virq_destroy(virq);
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_irq_plug_destroy);
//
// ps3_event_receive_port_setup - Setup an event receive port.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @virq: The assigned Linux virq.
//
// The virq can be used with lv1_connect_interrupt_event_receive_port() to
// arrange to receive interrupts from system-bus devices, or with
// ps3_send_event_locally() to signal events.
//
#[no_mangle]
pub unsafe extern "C" fn ps3_event_receive_port_setup(cpu: enum ps3_cpu_binding, virq: *mut c_uint) -> c_int {
    int ps3_event_receive_port_setup(enum ps3_cpu_binding cpu, unsigned int *virq)
    {
    int result;
    u64 outlet;
    result = lv1_construct_event_receive_port(&outlet);
    if (result) {
    FAIL("%s:%d: lv1_construct_event_receive_port failed: %s\n",
    __func__, __LINE__, ps3_result(result));
// virq = 0;
    return result;
    }
    result = ps3_irq_plug_setup(cpu, outlet, virq);
    BUG_ON(result);
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_event_receive_port_setup);
//
// ps3_event_receive_port_destroy - Destroy an event receive port.
// @virq: The assigned Linux virq.
//
// Since ps3_event_receive_port_destroy destroys the receive port outlet,
// SB devices need to call disconnect_interrupt_event_receive_port() before
// this.
//
#[no_mangle]
pub unsafe extern "C" fn ps3_event_receive_port_destroy(virq: c_uint) -> c_int {
    int ps3_event_receive_port_destroy(unsigned int virq)
    {
    int result;
    DBG(" . %s:%d virq %u\n", __func__, __LINE__, virq);
    ps3_chip_mask(irq_get_irq_data(virq));
    result = lv1_destruct_event_receive_port(virq_to_hw(virq));
    if (result)
    FAIL("%s:%d: lv1_destruct_event_receive_port failed: %s\n",
    __func__, __LINE__, ps3_result(result));
//
// Don't call ps3_virq_destroy() here since ps3_smp_cleanup_cpu()
// calls from interrupt context (smp_call_function) when kexecing.
//
    DBG(" <- %s:%d\n", __func__, __LINE__);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_send_event_locally(virq: c_uint) -> c_int {
    int ps3_send_event_locally(unsigned int virq)
    {
    return lv1_send_event_locally(virq_to_hw(virq));
    }
//
// ps3_sb_event_receive_port_setup - Setup a system bus event receive port.
// @dev: The system bus device instance.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @virq: The assigned Linux virq.
//
// An event irq represents a virtual device interrupt.  The interrupt_id
// coresponds to the software interrupt number.
//
    int ps3_sb_event_receive_port_setup(struct ps3_system_bus_device *dev,
    enum ps3_cpu_binding cpu, unsigned int *virq)
    {
// this should go in system-bus.c
    int result;
    result = ps3_event_receive_port_setup(cpu, virq);
    if (result)
    return result;
    result = lv1_connect_interrupt_event_receive_port(dev.bus_id,
    dev.dev_id, virq_to_hw(*virq), dev.interrupt_id);
    if (result) {
    FAIL("%s:%d: lv1_connect_interrupt_event_receive_port"
    " failed: %s\n", __func__, __LINE__,
    ps3_result(result));
    ps3_event_receive_port_destroy(*virq);
// virq = 0;
    return result;
    }
    DBG("%s:%d: interrupt_id %u, virq %u\n", __func__, __LINE__,
    dev.interrupt_id, *virq);
    return 0;
    }
    EXPORT_SYMBOL(ps3_sb_event_receive_port_setup);
    int ps3_sb_event_receive_port_destroy(struct ps3_system_bus_device *dev,
    unsigned int virq)
    {
// this should go in system-bus.c
    int result;
    DBG(" . %s:%d: interrupt_id %u, virq %u\n", __func__, __LINE__,
    dev.interrupt_id, virq);
    result = lv1_disconnect_interrupt_event_receive_port(dev.bus_id,
    dev.dev_id, virq_to_hw(virq), dev.interrupt_id);
    if (result)
    FAIL("%s:%d: lv1_disconnect_interrupt_event_receive_port"
    " failed: %s\n", __func__, __LINE__,
    ps3_result(result));
    result = ps3_event_receive_port_destroy(virq);
    BUG_ON(result);
//
// ps3_event_receive_port_destroy() destroys the IRQ plug,
// so don't call ps3_irq_plug_destroy() here.
//
    result = ps3_virq_destroy(virq);
    BUG_ON(result);
    DBG(" <- %s:%d\n", __func__, __LINE__);
    return result;
    }
    EXPORT_SYMBOL(ps3_sb_event_receive_port_destroy);
//
// ps3_io_irq_setup - Setup a system bus io irq.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @interrupt_id: The device interrupt id read from the system repository.
// @virq: The assigned Linux virq.
//
// An io irq represents a non-virtualized device interrupt.  interrupt_id
// coresponds to the interrupt number of the interrupt controller.
//
    int ps3_io_irq_setup(enum ps3_cpu_binding cpu, unsigned int interrupt_id,
    unsigned int *virq)
    {
    int result;
    u64 outlet;
    result = lv1_construct_io_irq_outlet(interrupt_id, &outlet);
    if (result) {
    FAIL("%s:%d: lv1_construct_io_irq_outlet failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    result = ps3_irq_plug_setup(cpu, outlet, virq);
    BUG_ON(result);
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_io_irq_setup);
#[no_mangle]
pub unsafe extern "C" fn ps3_io_irq_destroy(virq: c_uint) -> c_int {
    int ps3_io_irq_destroy(unsigned int virq)
    {
    int result;
    let mut outlet: c_ulong = virq_to_hw(virq);
    ps3_chip_mask(irq_get_irq_data(virq));
//
// lv1_destruct_io_irq_outlet() will destroy the IRQ plug,
// so call ps3_irq_plug_destroy() first.
//
    result = ps3_irq_plug_destroy(virq);
    BUG_ON(result);
    result = lv1_destruct_io_irq_outlet(outlet);
    if (result)
    FAIL("%s:%d: lv1_destruct_io_irq_outlet failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_io_irq_destroy);
//
// ps3_vuart_irq_setup - Setup the system virtual uart virq.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @virt_addr_bmp: The caller supplied virtual uart interrupt bitmap.
// @virq: The assigned Linux virq.
//
// The system supports only a single virtual uart, so multiple calls without
// freeing the interrupt will return a wrong state error.
//
    int ps3_vuart_irq_setup(enum ps3_cpu_binding cpu, void* virt_addr_bmp,
    unsigned int *virq)
    {
    int result;
    u64 outlet;
    u64 lpar_addr;
    BUG_ON(!is_kernel_addr((u64)virt_addr_bmp));
    lpar_addr = ps3_mm_phys_to_lpar(__pa(virt_addr_bmp));
    result = lv1_configure_virtual_uart_irq(lpar_addr, &outlet);
    if (result) {
    FAIL("%s:%d: lv1_configure_virtual_uart_irq failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    result = ps3_irq_plug_setup(cpu, outlet, virq);
    BUG_ON(result);
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_vuart_irq_setup);
#[no_mangle]
pub unsafe extern "C" fn ps3_vuart_irq_destroy(virq: c_uint) -> c_int {
    int ps3_vuart_irq_destroy(unsigned int virq)
    {
    int result;
    ps3_chip_mask(irq_get_irq_data(virq));
    result = lv1_deconfigure_virtual_uart_irq();
    if (result) {
    FAIL("%s:%d: lv1_configure_virtual_uart_irq failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    result = ps3_irq_plug_destroy(virq);
    BUG_ON(result);
    return result;
    }
    EXPORT_SYMBOL_GPL(ps3_vuart_irq_destroy);
//
// ps3_spe_irq_setup - Setup an spe virq.
// @cpu: enum ps3_cpu_binding indicating the cpu the interrupt should be
// serviced on.
// @spe_id: The spe_id returned from lv1_construct_logical_spe().
// @class: The spe interrupt class {0,1,2}.
// @virq: The assigned Linux virq.
//
    int ps3_spe_irq_setup(enum ps3_cpu_binding cpu, unsigned long spe_id,
    unsigned int class, unsigned int *virq)
    {
    int result;
    u64 outlet;
    BUG_ON(class > 2);
    result = lv1_get_spe_irq_outlet(spe_id, class, &outlet);
    if (result) {
    FAIL("%s:%d: lv1_get_spe_irq_outlet failed: %s\n",
    __func__, __LINE__, ps3_result(result));
    return result;
    }
    result = ps3_irq_plug_setup(cpu, outlet, virq);
    BUG_ON(result);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_spe_irq_destroy(virq: c_uint) -> c_int {
    int ps3_spe_irq_destroy(unsigned int virq)
    {
    int result;
    ps3_chip_mask(irq_get_irq_data(virq));
    result = ps3_irq_plug_destroy(virq);
    BUG_ON(result);
    return result;
    }

pub const PS3_PLUG_MAX: c_int = 63;

    static void _dump_64_bmp(const char *header, const u64 *p, unsigned cpu,
    const char* func, int line)
    {
    pr_debug("%s:%d: %s %u {%04llx_%04llx_%04llx_%04llx}\n",
    func, line, header, cpu,
// p >> 48, (*p >> 32) & 0xffff, (*p >> 16) & 0xffff,
// p & 0xffff);
    }
    static void __maybe_unused _dump_256_bmp(const char *header,
    const u64 *p, unsigned cpu, const char* func, int line)
    {
    pr_debug("%s:%d: %s %u {%016llx:%016llx:%016llx:%016llx}\n",
    func, line, header, cpu, p[0], p[1], p[2], p[3]);
    }

#[no_mangle]
unsafe extern "C" fn _dump_bmp(pd: *mut *mut ps3_private, func: *const *const c_char, line: c_int) {
    static void _dump_bmp(struct ps3_private* pd, const char* func, int line)
    {
    unsigned long flags;
    spin_lock_irqsave(&pd.bmp_lock, flags);
    _dump_64_bmp("stat", &pd.bmp.status, pd.thread_id, func, line);
    _dump_64_bmp("mask", (u64*)&pd.bmp.mask, pd.thread_id, func, line);
    spin_unlock_irqrestore(&pd.bmp_lock, flags);
    }

    static void __maybe_unused _dump_mask(struct ps3_private *pd,
    const char* func, int line)
    {
    unsigned long flags;
    spin_lock_irqsave(&pd.bmp_lock, flags);
    _dump_64_bmp("mask", (u64*)&pd.bmp.mask, pd.thread_id, func, line);
    spin_unlock_irqrestore(&pd.bmp_lock, flags);
    }

    static void dump_bmp(struct ps3_private* pd) {};

    static int ps3_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hwirq)
    {
    DBG("%s:%d: hwirq %lu, virq %u\n", __func__, __LINE__, hwirq,
    virq);
    irq_set_chip_and_handler(virq, &ps3_irq_chip, handle_fasteoi_irq);
    return 0;
    }
    static int ps3_host_match(struct irq_domain *h, struct device_node *np,
    enum irq_domain_bus_token bus_token)
    {
// Match all
    return 1;
    }
    static const struct irq_domain_ops ps3_host_ops = {
    .map = ps3_host_map,
    .match = ps3_host_match,
    };
#[no_mangle]
pub unsafe extern "C" fn ps3_register_ipi_debug_brk(cpu: c_uint, virq: c_uint) -> void __init {
    void __init ps3_register_ipi_debug_brk(unsigned int cpu, unsigned int virq)
    {
    struct ps3_private *pd = &per_cpu(ps3_private, cpu);
    set_bit(63 - virq, &pd.ipi_debug_brk_mask);
    DBG("%s:%d: cpu %u, virq %u, mask %lxh\n", __func__, __LINE__,
    cpu, virq, pd.ipi_debug_brk_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_register_ipi_irq(cpu: c_uint, virq: c_uint) -> void __init {
    void __init ps3_register_ipi_irq(unsigned int cpu, unsigned int virq)
    {
    struct ps3_private *pd = &per_cpu(ps3_private, cpu);
    set_bit(63 - virq, &pd.ipi_mask);
    DBG("%s:%d: cpu %u, virq %u, ipi_mask %lxh\n", __func__, __LINE__,
    cpu, virq, pd.ipi_mask);
    }
#[no_mangle]
unsafe extern "C" fn ps3_get_irq() -> c_uint {
    static unsigned int ps3_get_irq(void)
    {
    struct ps3_private *pd = this_cpu_ptr(&ps3_private);
    let mut x: u64 = (pd.bmp.status & pd.bmp.mask);
    unsigned int plug;
// check for ipi break first to stop this cpu ASAP
    if (x & pd.ipi_debug_brk_mask)
    x &= pd.ipi_debug_brk_mask;
    asm volatile("cntlzd %0,%1" : "=r" (plug) : "r" (x));
    plug &= 0x3f;
    if (unlikely(!plug)) {
    DBG("%s:%d: no plug found: thread_id %llu\n", __func__,
    __LINE__, pd.thread_id);
    dump_bmp(&per_cpu(ps3_private, 0));
    dump_bmp(&per_cpu(ps3_private, 1));
    return 0;
    }

    if (unlikely(plug < NR_IRQS_LEGACY || plug > PS3_PLUG_MAX)) {
    dump_bmp(&per_cpu(ps3_private, 0));
    dump_bmp(&per_cpu(ps3_private, 1));
    BUG();
    }

// IPIs are EOIed here.
    if (test_bit(63 - plug, &pd.ipi_mask))
    lv1_end_of_interrupt_ext(pd.ppe_id, pd.thread_id, plug);
    return plug;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_init_IRQ() -> void __init {
    void __init ps3_init_IRQ(void)
    {
    int result;
    unsigned cpu;
    struct irq_domain *host;
    host = irq_domain_create_nomap(core::ptr::null_mut(), PS3_PLUG_MAX + 1, &ps3_host_ops, core::ptr::null_mut());
    irq_set_default_domain(host);
    for_each_possible_cpu(cpu) {
    struct ps3_private *pd = &per_cpu(ps3_private, cpu);
    lv1_get_logical_ppe_id(&pd.ppe_id);
    pd.thread_id = get_hard_smp_processor_id(cpu);
    spin_lock_init(&pd.bmp_lock);
    DBG("%s:%d: ppe_id %llu, thread_id %llu, bmp %lxh\n",
    __func__, __LINE__, pd.ppe_id, pd.thread_id,
    ps3_mm_phys_to_lpar(__pa(&pd.bmp)));
    result = lv1_configure_irq_state_bitmap(pd.ppe_id,
    pd.thread_id, ps3_mm_phys_to_lpar(__pa(&pd.bmp)));
    if (result)
    FAIL("%s:%d: lv1_configure_irq_state_bitmap failed:"
    " %s\n", __func__, __LINE__,
    ps3_result(result));
    }
    ppc_md.get_irq = ps3_get_irq;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_shutdown_IRQ(cpu: c_int) {
    void ps3_shutdown_IRQ(int cpu)
    {
    int result;
    u64 ppe_id;
    let mut thread_id: u64 = get_hard_smp_processor_id(cpu);
    lv1_get_logical_ppe_id(&ppe_id);
    result = lv1_configure_irq_state_bitmap(ppe_id, thread_id, 0);
    DBG("%s:%d: lv1_configure_irq_state_bitmap (%llu:%llu/%d) %s\n", __func__,
    __LINE__, ppe_id, thread_id, cpu, ps3_result(result));
    }
