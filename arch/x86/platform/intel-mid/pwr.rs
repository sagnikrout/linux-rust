//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/intel-mid/pwr.c
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
// Intel MID Power Management Unit (PWRMU) device driver
//
// Copyright (C) 2016, Intel Corporation
//
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//
// Intel MID Power Management Unit device driver handles the South Complex PCI
// devices such as GPDMA, SPI, I2C, PWM, and so on. By default PCI core
// modifies bits in PMCSR register in the PCI configuration space. This is not
// enough on some SoCs like Intel Tangier. In such case PCI core sets a new
// power state of the device in question through a PM hook registered in struct
// pci_platform_pm_ops (see drivers/pci/pci-mid.c).
//

// Registers
pub const PM_STS: c_uint = 0x00;
pub const PM_CMD: c_uint = 0x04;
pub const PM_ICS: c_uint = 0x08;

// Bits in PM_STS

// Bits in PM_CMD

// System states

// Trigger variants

// Message to wait for TRIGGER_NC case

// List of commands
pub const CMD_SET_CFG: c_uint = 0x01;
// Bits in PM_ICS

// List of interrupts
pub const INT_INVALID: c_int = 0;
pub const INT_CMD_COMPLETE: c_int = 1;
pub const INT_CMD_ERR: c_int = 2;
pub const INT_WAKE_EVENT: c_int = 3;
pub const INT_LSS_POWER_ERR: c_int = 4;
pub const INT_S0iX_MSG_ERR: c_int = 5;
pub const INT_NO_C6: c_int = 6;
pub const INT_TRIGGER_ERR: c_int = 7;
pub const INT_INACTIVITY: c_int = 8;
// South Complex devices
pub const LSS_MAX_SHARED_DEVS: c_int = 4;
pub const LSS_MAX_DEVS: c_int = 64;

// Supported device IDs
pub const PCI_DEVICE_ID_PENWELL: c_uint = 0x0828;
pub const PCI_DEVICE_ID_TANGIER: c_uint = 0x11a1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_pwr_dev {
    pub pdev: *mut pci_dev,
    pub state: pci_power_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_pwr {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub available: bool,
    pub lock: mutex,
    pub lss: [mid_pwr_dev; LSS_MAX_DEVS][LSS_MAX_SHARED_DEVS],
}

    static struct mid_pwr *midpwr;
#[no_mangle]
unsafe extern "C" fn mid_pwr_get_state(pwr: *mut mid_pwr, reg: c_int) -> u32 {
    static u32 mid_pwr_get_state(struct mid_pwr *pwr, int reg)
    {
    return readl(pwr.regs + PM_SSS(reg));
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_set_state(pwr: *mut mid_pwr, reg: c_int, value: u32) {
    static void mid_pwr_set_state(struct mid_pwr *pwr, int reg, u32 value)
    {
    writel(value, pwr.regs + PM_SSC(reg));
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_set_wake(pwr: *mut mid_pwr, reg: c_int, value: u32) {
    static void mid_pwr_set_wake(struct mid_pwr *pwr, int reg, u32 value)
    {
    writel(value, pwr.regs + PM_WKC(reg));
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_interrupt_disable(pwr: *mut mid_pwr) {
    static void mid_pwr_interrupt_disable(struct mid_pwr *pwr)
    {
    writel(~PM_ICS_IE, pwr.regs + PM_ICS);
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_is_busy(pwr: *mut mid_pwr) -> bool {
    static bool mid_pwr_is_busy(struct mid_pwr *pwr)
    {
    return !!(readl(pwr.regs + PM_STS) & PM_STS_BUSY);
    }
// Wait 500ms that the latest PWRMU command finished
#[no_mangle]
unsafe extern "C" fn mid_pwr_wait(pwr: *mut mid_pwr) -> c_int {
    static int mid_pwr_wait(struct mid_pwr *pwr)
    {
    let mut count: c_uint = 500000;
    bool busy;
    do {
    busy = mid_pwr_is_busy(pwr);
    if (!busy)
    return 0;
    udelay(1);
    } while (--count);
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_wait_for_cmd(pwr: *mut mid_pwr, cmd: u8) -> c_int {
    static int mid_pwr_wait_for_cmd(struct mid_pwr *pwr, u8 cmd)
    {
    writel(PM_CMD_CMD(cmd) | PM_CMD_CM_IMMEDIATE, pwr.regs + PM_CMD);
    return mid_pwr_wait(pwr);
    }
#[no_mangle]
unsafe extern "C" fn __update_power_state(pwr: *mut mid_pwr, reg: c_int, bit: c_int, new: c_int) -> c_int {
    static int __update_power_state(struct mid_pwr *pwr, int reg, int bit, int new)
    {
    int curstate;
    u32 power;
    int ret;
// Check if the device is already in desired state
    power = mid_pwr_get_state(pwr, reg);
    curstate = (power >> bit) & 3;
    if (curstate == new)
    return 0;
// Update the power state
    mid_pwr_set_state(pwr, reg, (power & ~(3 << bit)) | (new << bit));
// Send command to SCU
    ret = mid_pwr_wait_for_cmd(pwr, CMD_SET_CFG);
    if (ret)
    return ret;
// Check if the device is already in desired state
    power = mid_pwr_get_state(pwr, reg);
    curstate = (power >> bit) & 3;
    if (curstate != new)
    return -EAGAIN;
    return 0;
    }
    static pci_power_t __find_weakest_power_state(struct mid_pwr_dev *lss,
    struct pci_dev *pdev,
    pci_power_t state)
    {
    let mut weakest: pci_power_t = PCI_D3hot;
    unsigned int j;
// Find device in cache or first free cell
    for (j = 0; j < LSS_MAX_SHARED_DEVS; j++) {
    if (lss[j].pdev == pdev || !lss[j].pdev)
    break;
    }
// Store the desired state in cache
    if (j < LSS_MAX_SHARED_DEVS) {
    lss[j].pdev = pdev;
    lss[j].state = state;
    } else {
    dev_WARN(&pdev.dev, "No room for device in PWRMU LSS cache\n");
    weakest = state;
    }
// Find the power state we may use
    for (j = 0; j < LSS_MAX_SHARED_DEVS; j++) {
    if (lss[j].state < weakest)
    weakest = lss[j].state;
    }
    return weakest;
    }
    static int __set_power_state(struct mid_pwr *pwr, struct pci_dev *pdev,
    pci_power_t state, int id, int reg, int bit)
    {
    const char *name;
    int ret;
    state = __find_weakest_power_state(pwr.lss[id], pdev, state);
    name = pci_power_name(state);
    ret = __update_power_state(pwr, reg, bit, ( int)state);
    if (ret) {
    dev_warn(&pdev.dev, "Can't set power state %s: %d\n", name, ret);
    return ret;
    }
    dev_vdbg(&pdev.dev, "Set power state %s\n", name);
    return 0;
    }
    static int mid_pwr_set_power_state(struct mid_pwr *pwr, struct pci_dev *pdev,
    pci_power_t state)
    {
    int id, reg, bit;
    int ret;
    id = intel_mid_pwr_get_lss_id(pdev);
    if (id < 0)
    return id;
    reg = (id * LSS_PWS_BITS) / 32;
    bit = (id * LSS_PWS_BITS) % 32;
// We support states between PCI_D0 and PCI_D3hot
    if (state < PCI_D0)
    state = PCI_D0;
    if (state > PCI_D3hot)
    state = PCI_D3hot;
    mutex_lock(&pwr.lock);
    ret = __set_power_state(pwr, pdev, state, id, reg, bit);
    mutex_unlock(&pwr.lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_mid_pci_set_power_state(pdev: *mut pci_dev, state: pci_power_t) -> c_int {
    int intel_mid_pci_set_power_state(struct pci_dev *pdev, pci_power_t state)
    {
    struct mid_pwr *pwr = midpwr;
    let mut ret: c_int = 0;
    might_sleep();
    if (pwr && pwr.available)
    ret = mid_pwr_set_power_state(pwr, pdev, state);
    dev_vdbg(&pdev.dev, "set_power_state() returns %d\n", ret);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t {
    pci_power_t intel_mid_pci_get_power_state(struct pci_dev *pdev)
    {
    struct mid_pwr *pwr = midpwr;
    int id, reg, bit;
    u32 power;
    if (!pwr || !pwr.available)
    return PCI_UNKNOWN;
    id = intel_mid_pwr_get_lss_id(pdev);
    if (id < 0)
    return PCI_UNKNOWN;
    reg = (id * LSS_PWS_BITS) / 32;
    bit = (id * LSS_PWS_BITS) % 32;
    power = mid_pwr_get_state(pwr, reg);
    return ( pci_power_t)((power >> bit) & 3);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_mid_pwr_power_off() {
    void intel_mid_pwr_power_off(void)
    {
    struct mid_pwr *pwr = midpwr;
    u32 cmd = PM_CMD_SYS_STATE_S5 |
    PM_CMD_CMD(CMD_SET_CFG) |
    PM_CMD_CM_TRIGGER |
    PM_CMD_CFG_TRIGGER_NC |
    TRIGGER_NC_MSG_2;
// Send command to SCU
    writel(cmd, pwr.regs + PM_CMD);
    mid_pwr_wait(pwr);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_mid_pwr_get_lss_id(pdev: *mut pci_dev) -> c_int {
    int intel_mid_pwr_get_lss_id(struct pci_dev *pdev)
    {
    int vndr;
    u8 id;
//
// Mapping to PWRMU index is kept in the Logical SubSystem ID byte of
// Vendor capability.
//
    vndr = pci_find_capability(pdev, PCI_CAP_ID_VNDR);
    if (!vndr)
    return -EINVAL;
// Read the Logical SubSystem ID byte
    pci_read_config_byte(pdev, vndr + INTEL_MID_PWR_LSS_OFFSET, &id);
    if (!(id & INTEL_MID_PWR_LSS_TYPE))
    return -ENODEV;
    id &= ~INTEL_MID_PWR_LSS_TYPE;
    if (id >= LSS_MAX_DEVS)
    return -ERANGE;
    return id;
    }
#[no_mangle]
unsafe extern "C" fn mid_pwr_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mid_pwr_irq_handler(int irq, void *dev_id)
    {
    struct mid_pwr *pwr = dev_id;
    u32 ics;
    ics = readl(pwr.regs + PM_ICS);
    if (!(ics & PM_ICS_IP))
    return IRQ_NONE;
    writel(ics | PM_ICS_IP, pwr.regs + PM_ICS);
    dev_warn(pwr.dev, "Unexpected IRQ: %#x\n", PM_ICS_INT_STATUS(ics));
    return IRQ_HANDLED;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_pwr_device_info {
    pub pwr): *mut *mut int (set_initial_state)(struct mid_pwr,
}

#[no_mangle]
unsafe extern "C" fn mid_pwr_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int mid_pwr_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct mid_pwr_device_info *info = (void *)id.driver_data;
    struct device *dev = &pdev.dev;
    struct mid_pwr *pwr;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret < 0) {
    dev_err(&pdev.dev, "error: could not enable device\n");
    return ret;
    }
    pwr = devm_kzalloc(dev, sizeof(*pwr), GFP_KERNEL);
    if (!pwr)
    return -ENOMEM;
    pwr.regs = pcim_iomap_region(pdev, 0, "intel_mid_pwr");
    ret = PTR_ERR_OR_ZERO(pwr.regs);
    if (ret) {
    dev_err(&pdev.dev, "Could not request / ioremap I/O-Mem: %d\n", ret);
    return ret;
    }
    pwr.dev = dev;
    pwr.irq = pdev.irq;
    mutex_init(&pwr.lock);
// Disable interrupts
    mid_pwr_interrupt_disable(pwr);
    if (info && info.set_initial_state) {
    ret = info.set_initial_state(pwr);
    if (ret)
    dev_warn(dev, "Can't set initial state: %d\n", ret);
    }
    ret = devm_request_irq(dev, pdev.irq, mid_pwr_irq_handler,
    IRQF_NO_SUSPEND, pci_name(pdev), pwr);
    if (ret)
    return ret;
    pwr.available = true;
    midpwr = pwr;
    pci_set_drvdata(pdev, pwr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mid_set_initial_state(pwr: *mut mid_pwr, states: *const u32) -> c_int {
    static int mid_set_initial_state(struct mid_pwr *pwr, const u32 *states)
    {
    unsigned int i, j;
    int ret;
//
// Enable wake events.
//
// PWRMU supports up to 32 sources for wake up the system. Ungate them
// all here.
//
    mid_pwr_set_wake(pwr, 0, 0xffffffff);
    mid_pwr_set_wake(pwr, 1, 0xffffffff);
//
// Power off South Complex devices.
//
// There is a map (see a note below) of 64 devices with 2 bits per each
// on 32-bit HW registers. The following calls set all devices to one
// known initial state, i.e. PCI_D3hot. This is done in conjunction
// with PMCSR setting in arch/x86/pci/intel_mid_pci.c.
//
// NOTE: The actual device mapping is provided by a platform at run
// time using vendor capability of PCI configuration space.
//
    mid_pwr_set_state(pwr, 0, states[0]);
    mid_pwr_set_state(pwr, 1, states[1]);
    mid_pwr_set_state(pwr, 2, states[2]);
    mid_pwr_set_state(pwr, 3, states[3]);
// Send command to SCU
    ret = mid_pwr_wait_for_cmd(pwr, CMD_SET_CFG);
    if (ret)
    return ret;
    for (i = 0; i < LSS_MAX_DEVS; i++) {
    for (j = 0; j < LSS_MAX_SHARED_DEVS; j++)
    pwr.lss[i][j].state = PCI_D3hot;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pnw_set_initial_state(pwr: *mut mid_pwr) -> c_int {
    static int pnw_set_initial_state(struct mid_pwr *pwr)
    {
// On Penwell SRAM must stay powered on
    static const u32 states[] = {
    0xf00fffff,		/* PM_SSC(0) */
    0xffffffff,		/* PM_SSC(1) */
    0xffffffff,		/* PM_SSC(2) */
    0xffffffff,		/* PM_SSC(3) */
    };
    return mid_set_initial_state(pwr, states);
    }
#[no_mangle]
unsafe extern "C" fn tng_set_initial_state(pwr: *mut mid_pwr) -> c_int {
    static int tng_set_initial_state(struct mid_pwr *pwr)
    {
    static const u32 states[] = {
    0xffffffff,		/* PM_SSC(0) */
    0xffffffff,		/* PM_SSC(1) */
    0xffffffff,		/* PM_SSC(2) */
    0xffffffff,		/* PM_SSC(3) */
    };
    return mid_set_initial_state(pwr, states);
    }
    static const struct mid_pwr_device_info pnw_info = {
    .set_initial_state = pnw_set_initial_state,
    };
    static const struct mid_pwr_device_info tng_info = {
    .set_initial_state = tng_set_initial_state,
    };
// This table should be in sync with the one in drivers/pci/pci-mid.c
    static const struct pci_device_id mid_pwr_pci_ids[] = {
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_PENWELL), (kernel_ulong_t)&pnw_info },
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_TANGIER), (kernel_ulong_t)&tng_info },
    {}
    };
    static struct pci_driver mid_pwr_pci_driver = {
    .name		= "intel_mid_pwr",
    .probe		= mid_pwr_probe,
    .id_table	= mid_pwr_pci_ids,
    };
    builtin_pci_driver(mid_pwr_pci_driver);
