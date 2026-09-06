//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/keystone_remoteproc.c
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
// TI Keystone DSP remoteproc driver
//
// Copyright (C) 2015-2017 Texas Instruments Incorporated - http://www.ti.com
//

//
// struct keystone_rproc_mem - internal memory structure
// @cpu_addr: MPU virtual address of the memory region
// @bus_addr: Bus address used to access the memory region
// @dev_addr: Device address of the memory region from DSP view
// @size: Size of the memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keystone_rproc_mem {
    pub cpu_addr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub dev_addr: u32,
    pub size: usize,
}

//
// struct keystone_rproc - keystone remote processor driver structure
// @dev: cached device pointer
// @rproc: remoteproc device handle
// @mem: internal memory regions data
// @num_mems: number of internal memory regions
// @dev_ctrl: device control regmap handle
// @reset: reset control handle
// @boot_offset: boot register offset in @dev_ctrl regmap
// @irq_ring: irq entry for vring
// @irq_fault: irq entry for exception
// @kick_gpio: gpio used for virtio kicks
// @workqueue: workqueue for processing virtio interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keystone_rproc {
    pub dev: *mut device,
    pub rproc: *mut rproc,
    pub mem: *mut keystone_rproc_mem,
    pub num_mems: c_int,
    pub dev_ctrl: *mut regmap,
    pub reset: *mut reset_control,
    pub kick_gpio: *mut gpio_desc,
    pub boot_offset: u32,
    pub irq_ring: c_int,
    pub irq_fault: c_int,
    pub workqueue: work_struct,
}

// Put the DSP processor into reset
#[no_mangle]
unsafe extern "C" fn keystone_rproc_dsp_reset(ksproc: *mut keystone_rproc) {
    static void keystone_rproc_dsp_reset(struct keystone_rproc *ksproc)
    {
    reset_control_assert(ksproc.reset);
    }
// Configure the boot address and boot the DSP processor
#[no_mangle]
unsafe extern "C" fn keystone_rproc_dsp_boot(ksproc: *mut keystone_rproc, boot_addr: u32) -> c_int {
    static int keystone_rproc_dsp_boot(struct keystone_rproc *ksproc, u32 boot_addr)
    {
    int ret;
    if (boot_addr & (SZ_1K - 1)) {
    dev_err(ksproc.dev, "invalid boot address 0x%x, must be aligned on a 1KB boundary\n",
    boot_addr);
    return -EINVAL;
    }
    ret = regmap_write(ksproc.dev_ctrl, ksproc.boot_offset, boot_addr);
    if (ret) {
    dev_err(ksproc.dev, "regmap_write of boot address failed, status = %d\n",
    ret);
    return ret;
    }
    reset_control_deassert(ksproc.reset);
    return 0;
    }
//
// Process the remoteproc exceptions
//
// The exception reporting on Keystone DSP remote processors is very simple
// compared to the equivalent processors on the OMAP family, it is notified
// through a software-designed specific interrupt source in the IPC interrupt
// generation register.
//
// This function just invokes the rproc_report_crash to report the exception
// to the remoteproc driver core, to trigger a recovery.
//
#[no_mangle]
unsafe extern "C" fn keystone_rproc_exception_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keystone_rproc_exception_interrupt(int irq, void *dev_id)
    {
    struct keystone_rproc *ksproc = dev_id;
    rproc_report_crash(ksproc.rproc, RPROC_FATAL_ERROR);
    return IRQ_HANDLED;
    }
//
// Main virtqueue message workqueue function
//
// This function is executed upon scheduling of the keystone remoteproc
// driver's workqueue. The workqueue is scheduled by the vring ISR handler.
//
// There is no payload message indicating the virtqueue index as is the
// case with mailbox-based implementations on OMAP family. As such, this
// handler processes both the Tx and Rx virtqueue indices on every invocation.
// The rproc_vq_interrupt function can detect if there are new unprocessed
// messages or not (returns IRQ_NONE vs IRQ_HANDLED), but there is no need
// to check for these return values. The index 0 triggering will process all
// pending Rx buffers, and the index 1 triggering will process all newly
// available Tx buffers and will wakeup any potentially blocked senders.
//
// NOTE:
// 1. A payload could be added by using some of the source bits in the
// IPC interrupt generation registers, but this would need additional
// changes to the overall IPC stack, and currently there are no benefits
// of adapting that approach.
// 2. The current logic is based on an inherent design assumption of supporting
// only 2 vrings, but this can be changed if needed.
//
#[no_mangle]
unsafe extern "C" fn handle_event(work: *mut work_struct) {
    static void handle_event(struct work_struct *work)
    {
    struct keystone_rproc *ksproc =
    container_of(work, struct keystone_rproc, workqueue);
    rproc_vq_interrupt(ksproc.rproc, 0);
    rproc_vq_interrupt(ksproc.rproc, 1);
    }
//
// Interrupt handler for processing vring kicks from remote processor
//
#[no_mangle]
unsafe extern "C" fn keystone_rproc_vring_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keystone_rproc_vring_interrupt(int irq, void *dev_id)
    {
    struct keystone_rproc *ksproc = dev_id;
    schedule_work(&ksproc.workqueue);
    return IRQ_HANDLED;
    }
//
// Power up the DSP remote processor.
//
// This function will be invoked only after the firmware for this rproc
// was loaded, parsed successfully, and all of its resource requirements
// were met.
//
#[no_mangle]
unsafe extern "C" fn keystone_rproc_start(rproc: *mut rproc) -> c_int {
    static int keystone_rproc_start(struct rproc *rproc)
    {
    struct keystone_rproc *ksproc = rproc.priv;
    int ret;
    INIT_WORK(&ksproc.workqueue, handle_event);
    enable_irq(ksproc.irq_ring);
    enable_irq(ksproc.irq_fault);
    ret = keystone_rproc_dsp_boot(ksproc, rproc.bootaddr);
    if (ret) {
    flush_work(&ksproc.workqueue);
    return ret;
    }
    return 0;
    }
//
// Stop the DSP remote processor.
//
// This function puts the DSP processor into reset, and finishes processing
// of any pending messages.
//
#[no_mangle]
unsafe extern "C" fn keystone_rproc_stop(rproc: *mut rproc) -> c_int {
    static int keystone_rproc_stop(struct rproc *rproc)
    {
    struct keystone_rproc *ksproc = rproc.priv;
    keystone_rproc_dsp_reset(ksproc);
    disable_irq(ksproc.irq_fault);
    disable_irq(ksproc.irq_ring);
    flush_work(&ksproc.workqueue);
    return 0;
    }
//
// Kick the remote processor to notify about pending unprocessed messages.
// The vqid usage is not used and is inconsequential, as the kick is performed
// through a simulated GPIO (a bit in an IPC interrupt-triggering register),
// the remote processor is expected to process both its Tx and Rx virtqueues.
//
#[no_mangle]
unsafe extern "C" fn keystone_rproc_kick(rproc: *mut rproc, vqid: c_int) {
    static void keystone_rproc_kick(struct rproc *rproc, int vqid)
    {
    struct keystone_rproc *ksproc = rproc.priv;
    if (!ksproc.kick_gpio)
    return;
    gpiod_set_value(ksproc.kick_gpio, 1);
    }
//
// Custom function to translate a DSP device address (internal RAMs only) to a
// kernel virtual address.  The DSPs can access their RAMs at either an internal
// address visible only from a DSP, or at the SoC-level bus address. Both these
// addresses need to be looked through for translation. The translated addresses
// can be used either by the remoteproc core for loading (when using kernel
// remoteproc loader), or by any rpmsg bus drivers.
//
    static void *keystone_rproc_da_to_va(struct rproc *rproc, u64 da, size_t len, bool *is_iomem)
    {
    struct keystone_rproc *ksproc = rproc.priv;
    void __iomem *va = core::ptr::null_mut();
    phys_addr_t bus_addr;
    u32 dev_addr, offset;
    size_t size;
    int i;
    if (len == 0)
    return core::ptr::null_mut();
    for (i = 0; i < ksproc.num_mems; i++) {
    bus_addr = ksproc.mem[i].bus_addr;
    dev_addr = ksproc.mem[i].dev_addr;
    size = ksproc.mem[i].size;
    if (da < KEYSTONE_RPROC_LOCAL_ADDRESS_MASK) {
// handle DSP-view addresses
    if ((da >= dev_addr) &&
    ((da + len) <= (dev_addr + size))) {
    offset = da - dev_addr;
    va = ksproc.mem[i].cpu_addr + offset;
    break;
    }
    } else {
// handle SoC-view addresses
    if ((da >= bus_addr) &&
    (da + len) <= (bus_addr + size)) {
    offset = da - bus_addr;
    va = ksproc.mem[i].cpu_addr + offset;
    break;
    }
    }
    }
    return ( void *)va;
    }
    static const struct rproc_ops keystone_rproc_ops = {
    .start		= keystone_rproc_start,
    .stop		= keystone_rproc_stop,
    .kick		= keystone_rproc_kick,
    .da_to_va	= keystone_rproc_da_to_va,
    };
    static int keystone_rproc_of_get_memories(struct platform_device *pdev,
    struct keystone_rproc *ksproc)
    {
    static const char * const mem_names[] = {"l2sram", "l1pram", "l1dram"};
    struct device *dev = &pdev.dev;
    struct resource *res;
    let mut num_mems: c_int = 0;
    int i;
    num_mems = ARRAY_SIZE(mem_names);
    ksproc.mem = devm_kcalloc(ksproc.dev, num_mems,
    sizeof(*ksproc.mem), GFP_KERNEL);
    if (!ksproc.mem)
    return -ENOMEM;
    for (i = 0; i < num_mems; i++) {
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    mem_names[i]);
    ksproc.mem[i].cpu_addr = devm_ioremap_resource(dev, res);
    if (IS_ERR(ksproc.mem[i].cpu_addr)) {
    dev_err(dev, "failed to parse and map %s memory\n",
    mem_names[i]);
    return PTR_ERR(ksproc.mem[i].cpu_addr);
    }
    ksproc.mem[i].bus_addr = res.start;
    ksproc.mem[i].dev_addr =
    res.start & KEYSTONE_RPROC_LOCAL_ADDRESS_MASK;
    ksproc.mem[i].size = resource_size(res);
// zero out memories to start in a pristine state
    memset(( void *)ksproc.mem[i].cpu_addr, 0,
    ksproc.mem[i].size);
    }
    ksproc.num_mems = num_mems;
    return 0;
    }
    static int keystone_rproc_of_get_dev_syscon(struct platform_device *pdev,
    struct keystone_rproc *ksproc)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    if (!of_property_present(np, "ti,syscon-dev")) {
    dev_err(dev, "ti,syscon-dev property is absent\n");
    return -EINVAL;
    }
    ksproc.dev_ctrl = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-dev",
    1, &ksproc.boot_offset);
    if (IS_ERR(ksproc.dev_ctrl))
    return PTR_ERR(ksproc.dev_ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_rproc_mem_release(data: *mut c_void) {
    static void keystone_rproc_mem_release(void *data)
    {
    struct device *dev = data;
    of_reserved_mem_device_release(dev);
    }
#[no_mangle]
unsafe extern "C" fn keystone_rproc_pm_runtime_put(data: *mut c_void) {
    static void keystone_rproc_pm_runtime_put(void *data)
    {
    struct device *dev = data;
    pm_runtime_put_sync(dev);
    }
#[no_mangle]
unsafe extern "C" fn keystone_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int keystone_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct keystone_rproc *ksproc;
    struct rproc *rproc;
    int dsp_id;
    char *fw_name = core::ptr::null_mut();
    let mut ret: c_int = 0;
    if (!np) {
    dev_err(dev, "only DT-based devices are supported\n");
    return -ENODEV;
    }
    dsp_id = of_alias_get_id(np, "rproc");
    if (dsp_id < 0) {
    dev_warn(dev, "device does not have an alias id\n");
    return dsp_id;
    }
// construct a custom default fw name - subject to change in future
    fw_name = devm_kasprintf(dev, GFP_KERNEL, "keystone-dsp%d-fw", dsp_id);
    if (!fw_name)
    return -ENOMEM;
    rproc = devm_rproc_alloc(dev, dev_name(dev), &keystone_rproc_ops,
    fw_name, sizeof(*ksproc));
    if (!rproc)
    return -ENOMEM;
    rproc.has_iommu = false;
    ksproc = rproc.priv;
    ksproc.rproc = rproc;
    ksproc.dev = dev;
    ret = keystone_rproc_of_get_dev_syscon(pdev, ksproc);
    if (ret)
    return ret;
    ksproc.reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(ksproc.reset))
    return PTR_ERR(ksproc.reset);
// enable clock for accessing DSP internal memories
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to enable runtime PM\n");
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to enable clock\n");
    ret = devm_add_action_or_reset(dev, keystone_rproc_pm_runtime_put, dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add disable pm devm action\n");
    ret = keystone_rproc_of_get_memories(pdev, ksproc);
    if (ret)
    return ret;
    ksproc.irq_ring = platform_get_irq_byname(pdev, "vring");
    if (ksproc.irq_ring < 0)
    return ksproc.irq_ring;
    ret = devm_request_irq(dev, ksproc.irq_ring, keystone_rproc_vring_interrupt,
    IRQF_NO_AUTOEN, dev_name(dev), ksproc);
    if (ret)
    return ret;
    ksproc.irq_fault = platform_get_irq_byname(pdev, "exception");
    if (ksproc.irq_fault < 0)
    return ksproc.irq_fault;
    ret = devm_request_irq(dev, ksproc.irq_fault, keystone_rproc_exception_interrupt,
    IRQF_NO_AUTOEN, dev_name(dev), ksproc);
    if (ret)
    return ret;
    ksproc.kick_gpio = devm_gpiod_get(dev, "kick", GPIOD_ASIS);
    ret = PTR_ERR_OR_ZERO(ksproc.kick_gpio);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get gpio for virtio kicks\n");
    ret = of_reserved_mem_device_init(dev);
    if (ret) {
    dev_warn(dev, "device does not have specific CMA pool\n");
    } else {
    ret = devm_add_action_or_reset(dev, keystone_rproc_mem_release, dev);
    if (ret)
    return ret;
    }
// ensure the DSP is in reset before loading firmware
    ret = reset_control_status(ksproc.reset);
    if (ret < 0) {
    return dev_err_probe(dev, ret, "failed to get reset status\n");
    } else if (ret == 0) {
    WARN(1, "device is not in reset\n");
    keystone_rproc_dsp_reset(ksproc);
    }
    ret = devm_rproc_add(dev, rproc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register device with remoteproc core\n");
    return 0;
    }
    static const struct of_device_id keystone_rproc_of_match[] = {
    { .compatible = "ti,k2hk-dsp", },
    { .compatible = "ti,k2l-dsp", },
    { .compatible = "ti,k2e-dsp", },
    { .compatible = "ti,k2g-dsp", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, keystone_rproc_of_match);
    static struct platform_driver keystone_rproc_driver = {
    .probe	= keystone_rproc_probe,
    .driver	= {
    .name = "keystone-rproc",
    .of_match_table = keystone_rproc_of_match,
    },
    };
    module_platform_driver(keystone_rproc_driver);
    MODULE_AUTHOR("Suman Anna <s-anna@ti.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("TI Keystone DSP Remoteproc driver");
