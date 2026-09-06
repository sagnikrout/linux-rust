//! Automatically rewritten from C to Rust
//! Source: drivers/firewire/init_ohci1394_dma.c
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
// init_ohci1394_dma.c - Initializes physical DMA on all OHCI 1394 controllers
//
// Copyright (C) 2006-2007      Bernhard Kaindl <bk@suse.de>
//
// Derived from drivers/ieee1394/ohci1394.c and arch/x86/kernel/early-quirks.c
// this file has functions to:
// - scan the PCI very early on boot for all OHCI 1394-compliant controllers
// - reset and initialize them and make them join the IEEE1394 bus and
// - enable physical DMA on them to allow remote debugging
//
// All code and data is marked as __init and __initdata, respective as
// during boot, all OHCI1394 controllers may be claimed by the firewire
// stack and at this point, this code should not touch them anymore.
//
// To use physical DMA after the initialization of the firewire stack,
// be sure that the stack enables it and (re-)attach after the bus reset
// which may be caused by the firewire stack initialization.
//

    int __initdata init_ohci1394_dma_early;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci {
    pub registers: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn reg_write(ohci: *const ohci, offset: c_int, data: u32) {
    static inline void reg_write(const struct ohci *ohci, int offset, u32 data)
    {
    writel(data, ohci.registers + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_read(ohci: *const ohci, offset: c_int) -> u32 {
    static inline u32 reg_read(const struct ohci *ohci, int offset)
    {
    return readl(ohci.registers + offset);
    }

// Reads a PHY register of an OHCI-1394 controller
#[no_mangle]
pub unsafe extern "C" fn get_phy_reg(ohci: *mut ohci, addr: u8) -> u8 __init {
    static inline u8 __init get_phy_reg(struct ohci *ohci, u8 addr)
    {
    int i;
    u32 r;
    reg_write(ohci, OHCI1394_PhyControl, (addr << 8) | 0x00008000);
    for (i = 0; i < OHCI_LOOP_COUNT; i++) {
    if (reg_read(ohci, OHCI1394_PhyControl) & 0x80000000)
    break;
    mdelay(1);
    }
    r = reg_read(ohci, OHCI1394_PhyControl);
    return (r & 0x00ff0000) >> 16;
    }
// Writes to a PHY register of an OHCI-1394 controller
#[no_mangle]
pub unsafe extern "C" fn set_phy_reg(ohci: *mut ohci, addr: u8, data: u8) -> void __init {
    static inline void __init set_phy_reg(struct ohci *ohci, u8 addr, u8 data)
    {
    int i;
    reg_write(ohci, OHCI1394_PhyControl, (addr << 8) | data | 0x00004000);
    for (i = 0; i < OHCI_LOOP_COUNT; i++) {
    if (!(reg_read(ohci, OHCI1394_PhyControl) & 0x00004000))
    break;
    mdelay(1);
    }
    }
// Resets an OHCI-1394 controller (for sane state before initialization)
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_soft_reset(ohci: *mut ohci) -> void __init {
    static inline void __init init_ohci1394_soft_reset(struct ohci *ohci)
    {
    int i;
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_softReset);
    for (i = 0; i < OHCI_LOOP_COUNT; i++) {
    if (!(reg_read(ohci, OHCI1394_HCControlSet)
    & OHCI1394_HCControl_softReset))
    break;
    mdelay(1);
    }
    }
pub const OHCI1394_MAX_AT_REQ_RETRIES: c_uint = 0xf;
pub const OHCI1394_MAX_AT_RESP_RETRIES: c_uint = 0x2;
pub const OHCI1394_MAX_PHYS_RESP_RETRIES: c_uint = 0x8;
// Basic OHCI-1394 register and port inititalization
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_initialize(ohci: *mut ohci) -> void __init {
    static inline void __init init_ohci1394_initialize(struct ohci *ohci)
    {
    u32 bus_options;
    int num_ports, i;
// Put some defaults to these undefined bus options
    bus_options = reg_read(ohci, OHCI1394_BusOptions);
    bus_options |=  0x60000000; /* Enable CMC and ISC */
    bus_options &= ~0x00ff0000; /* XXX: Set cyc_clk_acc to zero for now */
    bus_options &= ~0x18000000; /* Disable PMC and BMC */
    reg_write(ohci, OHCI1394_BusOptions, bus_options);
// Set the bus number
    reg_write(ohci, OHCI1394_NodeID, 0x0000ffc0);
// Enable posted writes
    reg_write(ohci, OHCI1394_HCControlSet,
    OHCI1394_HCControl_postedWriteEnable);
// Clear link control register
    reg_write(ohci, OHCI1394_LinkControlClear, 0xffffffff);
// enable phys
    reg_write(ohci, OHCI1394_LinkControlSet,
    OHCI1394_LinkControl_rcvPhyPkt);
// Don't accept phy packets into AR request context
    reg_write(ohci, OHCI1394_LinkControlClear, 0x00000400);
// Clear the Isochonouys interrupt masks
    reg_write(ohci, OHCI1394_IsoRecvIntMaskClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoRecvIntEventClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoXmitIntMaskClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IsoXmitIntEventClear, 0xffffffff);
// Accept asynchronous transfer requests from all nodes for now
    reg_write(ohci, OHCI1394_AsReqFilterHiSet, 0x80000000);
// Specify asynchronous transfer retries
    reg_write(ohci, OHCI1394_ATRetries,
    OHCI1394_MAX_AT_REQ_RETRIES |
    (OHCI1394_MAX_AT_RESP_RETRIES<<4) |
    (OHCI1394_MAX_PHYS_RESP_RETRIES<<8));
// We don't want hardware swapping
    reg_write(ohci, OHCI1394_HCControlClear,
    OHCI1394_HCControl_noByteSwapData);
// Enable link
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_linkEnable);
// If anything is connected to a port, make sure it is enabled
    num_ports = get_phy_reg(ohci, 2) & 0xf;
    for (i = 0; i < num_ports; i++) {
    unsigned int status;
    set_phy_reg(ohci, 7, i);
    status = get_phy_reg(ohci, 8);
    if (status & 0x20)
    set_phy_reg(ohci, 8, status & ~1);
    }
    }
//
// init_ohci1394_wait_for_busresets - wait until bus resets are completed
// @ohci: Pointer to the OHCI-1394 controller structure
//
// OHCI1394 initialization itself and any device going on- or offline
// and any cable issue cause a IEEE1394 bus reset. The OHCI1394 spec
// specifies that physical DMA is disabled on each bus reset and it
// has to be enabled after each bus reset when needed. We resort
// to polling here because on early boot, we have no interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_wait_for_busresets(ohci: *mut ohci) -> void __init {
    static inline void __init init_ohci1394_wait_for_busresets(struct ohci *ohci)
    {
    int i, events;
    for (i = 0; i < 9; i++) {
    mdelay(200);
    events = reg_read(ohci, OHCI1394_IntEventSet);
    if (events & OHCI1394_busReset)
    reg_write(ohci, OHCI1394_IntEventClear,
    OHCI1394_busReset);
    }
    }
//
// init_ohci1394_enable_physical_dma - Enable physical DMA for remote debugging
// @ohci: Pointer to the OHCI-1394 controller structure
//
// This enables remote DMA access over IEEE1394 from every host for the low
// 4GB of address space. DMA accesses above 4GB are not available currently.
//
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_enable_physical_dma(ohci: *mut ohci) -> void __init {
    static inline void __init init_ohci1394_enable_physical_dma(struct ohci *ohci)
    {
    reg_write(ohci, OHCI1394_PhyReqFilterHiSet, 0xffffffff);
    reg_write(ohci, OHCI1394_PhyReqFilterLoSet, 0xffffffff);
    reg_write(ohci, OHCI1394_PhyUpperBound, 0xffff0000);
    }
//
// init_ohci1394_reset_and_init_dma - init controller and enable DMA
// @ohci: Pointer to the OHCI-1394 controller structure
//
// This initializes the given controller and enables physical DMA engine in it.
//
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_reset_and_init_dma(ohci: *mut ohci) -> void __init {
    static inline void __init init_ohci1394_reset_and_init_dma(struct ohci *ohci)
    {
// Start off with a soft reset, clears everything to a sane state.
    init_ohci1394_soft_reset(ohci);
// Accessing some registers without LPS enabled may cause lock up
    reg_write(ohci, OHCI1394_HCControlSet, OHCI1394_HCControl_LPS);
// Disable and clear interrupts
    reg_write(ohci, OHCI1394_IntEventClear, 0xffffffff);
    reg_write(ohci, OHCI1394_IntMaskClear, 0xffffffff);
    mdelay(50); /* Wait 50msec to make sure we have full link enabled */
    init_ohci1394_initialize(ohci);
//
// The initialization causes at least one IEEE1394 bus reset. Enabling
// physical DMA only works *after* *all* bus resets have calmed down:
//
    init_ohci1394_wait_for_busresets(ohci);
// We had to wait and do this now if we want to debug early problems
    init_ohci1394_enable_physical_dma(ohci);
    }
//
// init_ohci1394_controller - Map the registers of the controller and init DMA
// @num: PCI bus number
// @slot: PCI device number
// @func: PCI function number
//
// This maps the registers of the specified controller and initializes it
//
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_controller(num: c_int, slot: c_int, func: c_int) -> void __init {
    static inline void __init init_ohci1394_controller(int num, int slot, int func)
    {
    unsigned long ohci_base;
    struct ohci ohci;
    printk(KERN_INFO "init_ohci1394_dma: initializing OHCI-1394"
    " at %02x:%02x.%x\n", num, slot, func);
    ohci_base = read_pci_config(num, slot, func, PCI_BASE_ADDRESS_0+(0<<2))
    & PCI_BASE_ADDRESS_MEM_MASK;
    set_fixmap_nocache(FIX_OHCI1394_BASE, ohci_base);
    ohci.registers = (void __iomem *)fix_to_virt(FIX_OHCI1394_BASE);
    init_ohci1394_reset_and_init_dma(&ohci);
    }
//
// init_ohci1394_dma_on_all_controllers - scan for OHCI1394 controllers and init DMA on them
// Scans the whole PCI space for OHCI1394 controllers and inits DMA on them
//
#[no_mangle]
pub unsafe extern "C" fn init_ohci1394_dma_on_all_controllers() -> void __init {
    void __init init_ohci1394_dma_on_all_controllers(void)
    {
    int num, slot, func;
    u32 class;
    if (!early_pci_allowed())
    return;
// Poor man's PCI discovery, the only thing we can do at early boot
    for (num = 0; num < 32; num++) {
    for (slot = 0; slot < 32; slot++) {
    for (func = 0; func < 8; func++) {
    class = read_pci_config(num, slot, func,
    PCI_CLASS_REVISION);
    if (class == 0xffffffff)
    continue; /* No device at this func */
    if (class>>8 != PCI_CLASS_SERIAL_FIREWIRE_OHCI)
    continue; /* Not an OHCI-1394 device */
    init_ohci1394_controller(num, slot, func);
    break; /* Assume one controller per device */
    }
    }
    }
    printk(KERN_INFO "init_ohci1394_dma: finished initializing OHCI DMA\n");
    }
//
// setup_ohci1394_dma - enables early OHCI1394 DMA initialization
// @opt: Kernel command line parameter string
//
#[no_mangle]
unsafe extern "C" fn setup_ohci1394_dma(opt: *mut c_char) -> int __init {
    static int __init setup_ohci1394_dma(char *opt)
    {
    if (!strcmp(opt, "early"))
    init_ohci1394_dma_early = 1;
    return 0;
    }
// passing ohci1394_dma=early on boot causes early OHCI1394 DMA initialization
    early_param("ohci1394_dma", setup_ohci1394_dma);
