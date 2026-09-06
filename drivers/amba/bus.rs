//! Automatically rewritten from C to Rust
//! Source: drivers/amba/bus.c
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
// linux/arch/arm/common/amba.c
//
// Copyright (C) 2003 Deep Blue Solutions Ltd, All Rights Reserved.
//

// called on periphid match and class 0x9 coresight device.
    static int
    amba_cs_uci_id_match(const struct amba_id *table, struct amba_device *dev)
    {
    let mut ret: c_int = 0;
    struct amba_cs_uci_id *uci;
    uci = table.data;
// no table data or zero mask - return match on periphid
    if (!uci || (uci.devarch_mask == 0))
    return 1;
// test against read devtype and masked devarch value
    ret = (dev.uci.devtype == uci.devtype) &&
    ((dev.uci.devarch & uci.devarch_mask) == uci.devarch);
    return ret;
    }
    static const struct amba_id *
    amba_lookup(const struct amba_id *table, struct amba_device *dev)
    {
    while (table.mask) {
    if (((dev.periphid & table.mask) == table.id) &&
    ((dev.cid != CORESIGHT_CID) ||
    (amba_cs_uci_id_match(table, dev))))
    return table;
    table++;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn amba_get_enable_pclk(pcdev: *mut amba_device) -> c_int {
    static int amba_get_enable_pclk(struct amba_device *pcdev)
    {
    int ret;
    pcdev.pclk = clk_get(&pcdev.dev, "apb_pclk");
    if (IS_ERR(pcdev.pclk))
    return PTR_ERR(pcdev.pclk);
    ret = clk_prepare_enable(pcdev.pclk);
    if (ret)
    clk_put(pcdev.pclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_put_disable_pclk(pcdev: *mut amba_device) {
    static void amba_put_disable_pclk(struct amba_device *pcdev)
    {
    clk_disable_unprepare(pcdev.pclk);
    clk_put(pcdev.pclk);
    }

    static ssize_t name##_show(struct device *_dev,				\
    struct device_attribute *attr, char *buf)	\
    {									\
    struct amba_device *dev = to_amba_device(_dev);			\
    return sprintf(buf, fmt, arg);					\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    amba_attr_func(id, "%08x\n", dev.periphid);
    amba_attr_func(resource, "\t%016llx\t%016llx\t%016lx\n",
    (unsigned long long)dev.res.start, (unsigned long long)dev.res.end,
    dev.res.flags);
    static struct attribute *amba_dev_attrs[] = {
    &dev_attr_id.attr,
    &dev_attr_resource.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(amba_dev);
#[no_mangle]
unsafe extern "C" fn amba_read_periphid(dev: *mut amba_device) -> c_int {
    static int amba_read_periphid(struct amba_device *dev)
    {
    struct reset_control *rstc;
    u32 size, pid, cid;
    void __iomem *tmp;
    int i, ret;
    ret = dev_pm_domain_attach(&dev.dev, PD_FLAG_ATTACH_POWER_ON);
    if (ret) {
    dev_dbg(&dev.dev, "can't get PM domain: %d\n", ret);
    goto err_out;
    }
    ret = amba_get_enable_pclk(dev);
    if (ret) {
    dev_dbg(&dev.dev, "can't get pclk: %d\n", ret);
    goto err_pm;
    }
//
// Find reset control(s) of the amba bus and de-assert them.
//
    rstc = of_reset_control_array_get_optional_shared(dev.dev.of_node);
    if (IS_ERR(rstc)) {
    ret = PTR_ERR(rstc);
    if (ret != -EPROBE_DEFER)
    dev_err(&dev.dev, "can't get reset: %d\n", ret);
    goto err_clk;
    }
    reset_control_deassert(rstc);
    reset_control_put(rstc);
    size = resource_size(&dev.res);
    tmp = ioremap(dev.res.start, size);
    if (!tmp) {
    ret = -ENOMEM;
    goto err_clk;
    }
//
// Read pid and cid based on size of resource
// they are located at end of region
//
    for (pid = 0, i = 0; i < 4; i++)
    pid |= (readl(tmp + size - 0x20 + 4 * i) & 255) << (i * 8);
    for (cid = 0, i = 0; i < 4; i++)
    cid |= (readl(tmp + size - 0x10 + 4 * i) & 255) << (i * 8);
    if (cid == CORESIGHT_CID) {
// set the base to the start of the last 4k block
    void __iomem *csbase = tmp + size - 4096;
    dev.uci.devarch = readl(csbase + UCI_REG_DEVARCH_OFFSET);
    dev.uci.devtype = readl(csbase + UCI_REG_DEVTYPE_OFFSET) & 0xff;
    }
    if (cid == AMBA_CID || cid == CORESIGHT_CID) {
    dev.periphid = pid;
    dev.cid = cid;
    }
    if (!dev.periphid)
    ret = -ENODEV;
    iounmap(tmp);
    err_clk:
    amba_put_disable_pclk(dev);
    err_pm:
    dev_pm_domain_detach(&dev.dev, true);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int amba_match(struct device *dev, const struct device_driver *drv)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    const struct amba_driver *pcdrv = to_amba_driver(drv);
    int ret;
    mutex_lock(&pcdev.periphid_lock);
    if (!pcdev.periphid) {
    ret = amba_read_periphid(pcdev);
//
// Returning any error other than -EPROBE_DEFER from bus match
// can cause driver registration failure. So, if there's a
// permanent failure in reading pid and cid, simply map it to
// -EPROBE_DEFER.
//
    if (ret) {
    mutex_unlock(&pcdev.periphid_lock);
    return -EPROBE_DEFER;
    }
    dev_set_uevent_suppress(dev, false);
    kobject_uevent(&dev.kobj, KOBJ_ADD);
    }
    mutex_unlock(&pcdev.periphid_lock);
// When driver_override is set, only bind to the matching driver
    ret = device_match_driver_override(dev, drv);
    if (ret >= 0)
    return ret;
    return amba_lookup(pcdrv.id_table, pcdev) != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn amba_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int amba_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct amba_device *pcdev = to_amba_device(dev);
    let mut retval: c_int = 0;
    retval = add_uevent_var(env, "AMBA_ID=%08x", pcdev.periphid);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "MODALIAS=amba:d%08X", pcdev.periphid);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn of_amba_device_decode_irq(dev: *mut amba_device) -> c_int {
    static int of_amba_device_decode_irq(struct amba_device *dev)
    {
    struct device_node *node = dev.dev.of_node;
    int i, irq = 0;
    if (IS_ENABLED(CONFIG_OF_IRQ) && node) {
// Decode the IRQs and address ranges
    for (i = 0; i < AMBA_NR_IRQS; i++) {
    irq = of_irq_get(node, i);
    if (irq < 0) {
    if (irq == -EPROBE_DEFER)
    return irq;
    irq = 0;
    }
    dev.irq[i] = irq;
    }
    }
    return 0;
    }
//
// These are the device model conversion veneers; they convert the
// device model structures to our more specific structures.
//
#[no_mangle]
unsafe extern "C" fn amba_probe(dev: *mut device) -> c_int {
    static int amba_probe(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    struct amba_driver *pcdrv = to_amba_driver(dev.driver);
    const struct amba_id *id = amba_lookup(pcdrv.id_table, pcdev);
    int ret;
    do {
    ret = of_amba_device_decode_irq(pcdev);
    if (ret)
    break;
    ret = of_clk_set_defaults(dev.of_node, false);
    if (ret < 0)
    break;
    ret = dev_pm_domain_attach(dev, PD_FLAG_ATTACH_POWER_ON |
    PD_FLAG_DETACH_POWER_OFF);
    if (ret)
    break;
    ret = amba_get_enable_pclk(pcdev);
    if (ret)
    break;
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    ret = pcdrv.probe(pcdev, id);
    if (ret == 0)
    break;
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_put_noidle(dev);
    amba_put_disable_pclk(pcdev);
    } while (0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_remove(dev: *mut device) {
    static void amba_remove(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    struct amba_driver *drv = to_amba_driver(dev.driver);
    pm_runtime_get_sync(dev);
    if (drv.remove)
    drv.remove(pcdev);
    pm_runtime_put_noidle(dev);
// Undo the runtime PM settings in amba_probe()
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    pm_runtime_put_noidle(dev);
    amba_put_disable_pclk(pcdev);
    }
#[no_mangle]
unsafe extern "C" fn amba_shutdown(dev: *mut device) {
    static void amba_shutdown(struct device *dev)
    {
    struct amba_driver *drv;
    if (!dev.driver)
    return;
    drv = to_amba_driver(dev.driver);
    if (drv.shutdown)
    drv.shutdown(to_amba_device(dev));
    }
#[no_mangle]
unsafe extern "C" fn amba_dma_configure(dev: *mut device) -> c_int {
    static int amba_dma_configure(struct device *dev)
    {
    struct amba_driver *drv = to_amba_driver(dev.driver);
    enum dev_dma_attr attr;
    let mut ret: c_int = 0;
    if (dev.of_node) {
    ret = of_dma_configure(dev, dev.of_node, true);
    } else if (has_acpi_companion(dev)) {
    attr = acpi_get_dma_attr(to_acpi_device_node(dev.fwnode));
    ret = acpi_dma_configure(dev, attr);
    }
// @drv may not be valid when we're called from the IOMMU layer
    if (!ret && dev.driver && !drv.driver_managed_dma) {
    ret = iommu_device_use_default_domain(dev);
    if (ret)
    arch_teardown_dma_ops(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_dma_cleanup(dev: *mut device) {
    static void amba_dma_cleanup(struct device *dev)
    {
    struct amba_driver *drv = to_amba_driver(dev.driver);
    if (!drv.driver_managed_dma)
    iommu_device_unuse_default_domain(dev);
    }

//
// Hooks to provide runtime PM of the pclk (bus clock).  It is safe to
// enable/disable the bus clock at runtime PM suspend/resume as this
// does not result in loss of context.
//
#[no_mangle]
unsafe extern "C" fn amba_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int amba_pm_runtime_suspend(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    let mut ret: c_int = pm_generic_runtime_suspend(dev);
    if (ret == 0 && dev.driver) {
    if (pm_runtime_is_irq_safe(dev))
    clk_disable(pcdev.pclk);
    else
    clk_disable_unprepare(pcdev.pclk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amba_pm_runtime_resume(dev: *mut device) -> c_int {
    static int amba_pm_runtime_resume(struct device *dev)
    {
    struct amba_device *pcdev = to_amba_device(dev);
    int ret;
    if (dev.driver) {
    if (pm_runtime_is_irq_safe(dev))
    ret = clk_enable(pcdev.pclk);
    else
    ret = clk_prepare_enable(pcdev.pclk);
// Failure is probably fatal to the system, but...
    if (ret)
    return ret;
    }
    return pm_generic_runtime_resume(dev);
    }

    static const struct dev_pm_ops amba_pm = {
    SET_RUNTIME_PM_OPS(
    amba_pm_runtime_suspend,
    amba_pm_runtime_resume,
    core::ptr::null_mut()
    )
    };
//
// Primecells are part of the Advanced Microcontroller Bus Architecture,
// so we call the bus "amba".
// DMA configuration for platform and AMBA bus is same. So here we reuse
// platform's DMA config routine.
//
    const struct bus_type amba_bustype = {
    .name		= "amba",
    .dev_groups	= amba_dev_groups,
    .driver_override = true,
    .match		= amba_match,
    .uevent		= amba_uevent,
    .probe		= amba_probe,
    .remove		= amba_remove,
    .shutdown	= amba_shutdown,
    .dma_configure	= amba_dma_configure,
    .dma_cleanup	= amba_dma_cleanup,
    .pm		= &amba_pm,
    };
    EXPORT_SYMBOL_GPL(amba_bustype);
#[no_mangle]
pub unsafe extern "C" fn dev_is_amba(dev: *const device) -> bool {
    bool dev_is_amba(const struct device *dev)
    {
    return dev.bus == &amba_bustype;
    }
    EXPORT_SYMBOL_GPL(dev_is_amba);
#[no_mangle]
unsafe extern "C" fn amba_init() -> int __init {
    static int __init amba_init(void)
    {
    return bus_register(&amba_bustype);
    }
    postcore_initcall(amba_init);
    static int amba_proxy_probe(struct amba_device *adev,
    const struct amba_id *id)
    {
    WARN(1, "Stub driver should never match any device.\n");
    return -ENODEV;
    }
    static const struct amba_id amba_stub_drv_ids[] = {
    { 0, 0 },
    };
    static struct amba_driver amba_proxy_drv = {
    .drv = {
    .name = "amba-proxy",
    },
    .probe = amba_proxy_probe,
    .id_table = amba_stub_drv_ids,
    };
#[no_mangle]
unsafe extern "C" fn amba_stub_drv_init() -> int __init {
    static int __init amba_stub_drv_init(void)
    {
    if (!IS_ENABLED(CONFIG_MODULES))
    return 0;
//
// The amba_match() function will get called only if there is at least
// one amba driver registered. If all amba drivers are modules and are
// only loaded based on uevents, then we'll hit a chicken-and-egg
// situation where amba_match() is waiting on drivers and drivers are
// waiting on amba_match(). So, register a stub driver to make sure
// amba_match() is called even if no amba driver has been registered.
//
    return __amba_driver_register(&amba_proxy_drv, core::ptr::null_mut());
    }
    late_initcall_sync(amba_stub_drv_init);
//
// __amba_driver_register - register an AMBA device driver
// @drv: amba device driver structure
// @owner: owning module/driver
//
// Register an AMBA device driver with the Linux device model
// core.  If devices pre-exist, the drivers probe function will
// be called.
//
    int __amba_driver_register(struct amba_driver *drv,
    struct module *owner)
    {
    if (!drv.probe)
    return -EINVAL;
    drv.drv.owner = owner;
    drv.drv.bus = &amba_bustype;
    return driver_register(&drv.drv);
    }
    EXPORT_SYMBOL(__amba_driver_register);
//
// amba_driver_unregister - remove an AMBA device driver
// @drv: AMBA device driver structure to remove
//
// Unregister an AMBA device driver from the Linux device
// model.  The device model will call the drivers remove function
// for each device the device driver is currently handling.
//
#[no_mangle]
pub unsafe extern "C" fn amba_driver_unregister(drv: *mut amba_driver) {
    void amba_driver_unregister(struct amba_driver *drv)
    {
    driver_unregister(&drv.drv);
    }
    EXPORT_SYMBOL(amba_driver_unregister);
#[no_mangle]
unsafe extern "C" fn amba_device_release(dev: *mut device) {
    static void amba_device_release(struct device *dev)
    {
    struct amba_device *d = to_amba_device(dev);
    fwnode_handle_put(dev_fwnode(&d.dev));
    if (d.res.parent)
    release_resource(&d.res);
    mutex_destroy(&d.periphid_lock);
    kfree(d);
    }
//
// amba_device_add - add a previously allocated AMBA device structure
// @dev: AMBA device allocated by amba_device_alloc
// @parent: resource parent for this devices resources
//
// Claim the resource, and read the device cell ID if not already
// initialized.  Register the AMBA device with the Linux device
// manager.
//
#[no_mangle]
pub unsafe extern "C" fn amba_device_add(dev: *mut amba_device, parent: *mut resource) -> c_int {
    int amba_device_add(struct amba_device *dev, struct resource *parent)
    {
    int ret;
    fwnode_handle_get(dev_fwnode(&dev.dev));
    ret = request_resource(parent, &dev.res);
    if (ret)
    return ret;
// If primecell ID isn't hard-coded, figure it out
    if (!dev.periphid) {
//
// AMBA device uevents require reading its pid and cid
// registers.  To do this, the device must be on, clocked and
// out of reset.  However in some cases those resources might
// not yet be available.  If that's the case, we suppress the
// generation of uevents until we can read the pid and cid
// registers.  See also amba_match().
//
    if (amba_read_periphid(dev))
    dev_set_uevent_suppress(&dev.dev, true);
    }
    ret = device_add(&dev.dev);
    if (ret)
    release_resource(&dev.res);
    return ret;
    }
    EXPORT_SYMBOL_GPL(amba_device_add);
#[no_mangle]
unsafe extern "C" fn amba_device_initialize(dev: *mut amba_device, name: *const c_char) {
    static void amba_device_initialize(struct amba_device *dev, const char *name)
    {
    device_initialize(&dev.dev);
    if (name)
    dev_set_name(&dev.dev, "%s", name);
    dev.dev.release = amba_device_release;
    dev.dev.bus = &amba_bustype;
    dev.dev.dma_mask = &dev.dev.coherent_dma_mask;
    dev.dev.dma_parms = &dev.dma_parms;
    dev.res.name = dev_name(&dev.dev);
    mutex_init(&dev.periphid_lock);
    }
//
// amba_device_alloc - allocate an AMBA device
// @name: sysfs name of the AMBA device
// @base: base of AMBA device
// @size: size of AMBA device
//
// Allocate and initialize an AMBA device structure.  Returns %NULL
// on failure.
//
    struct amba_device *amba_device_alloc(const char *name, resource_size_t base,
    size_t size)
    {
    struct amba_device *dev;
    dev = kzalloc_obj(*dev);
    if (dev) {
    amba_device_initialize(dev, name);
    dev.res.start = base;
    dev.res.end = base + size - 1;
    dev.res.flags = IORESOURCE_MEM;
    }
    return dev;
    }
    EXPORT_SYMBOL_GPL(amba_device_alloc);
//
// amba_device_register - register an AMBA device
// @dev: AMBA device to register
// @parent: parent memory resource
//
// Setup the AMBA device, reading the cell ID if present.
// Claim the resource, and register the AMBA device with
// the Linux device manager.
//
#[no_mangle]
pub unsafe extern "C" fn amba_device_register(dev: *mut amba_device, parent: *mut resource) -> c_int {
    int amba_device_register(struct amba_device *dev, struct resource *parent)
    {
    amba_device_initialize(dev, dev.dev.init_name);
    dev.dev.init_name = core::ptr::null_mut();
    return amba_device_add(dev, parent);
    }
    EXPORT_SYMBOL(amba_device_register);
//
// amba_device_put - put an AMBA device
// @dev: AMBA device to put
//
#[no_mangle]
pub unsafe extern "C" fn amba_device_put(dev: *mut amba_device) {
    void amba_device_put(struct amba_device *dev)
    {
    put_device(&dev.dev);
    }
    EXPORT_SYMBOL_GPL(amba_device_put);
//
// amba_device_unregister - unregister an AMBA device
// @dev: AMBA device to remove
//
// Remove the specified AMBA device from the Linux device
// manager.  All files associated with this object will be
// destroyed, and device drivers notified that the device has
// been removed.  The AMBA device's resources including
// the amba_device structure will be freed once all
// references to it have been dropped.
//
#[no_mangle]
pub unsafe extern "C" fn amba_device_unregister(dev: *mut amba_device) {
    void amba_device_unregister(struct amba_device *dev)
    {
    device_unregister(&dev.dev);
    }
    EXPORT_SYMBOL(amba_device_unregister);
//
// amba_request_regions - request all mem regions associated with device
// @dev: amba_device structure for device
// @name: name, or NULL to use driver name
//
#[no_mangle]
pub unsafe extern "C" fn amba_request_regions(dev: *mut amba_device, name: *const c_char) -> c_int {
    int amba_request_regions(struct amba_device *dev, const char *name)
    {
    let mut ret: c_int = 0;
    u32 size;
    if (!name)
    name = dev.dev.driver.name;
    size = resource_size(&dev.res);
    if (!request_mem_region(dev.res.start, size, name))
    ret = -EBUSY;
    return ret;
    }
    EXPORT_SYMBOL(amba_request_regions);
//
// amba_release_regions - release mem regions associated with device
// @dev: amba_device structure for device
//
// Release regions claimed by a successful call to amba_request_regions.
//
#[no_mangle]
pub unsafe extern "C" fn amba_release_regions(dev: *mut amba_device) {
    void amba_release_regions(struct amba_device *dev)
    {
    u32 size;
    size = resource_size(&dev.res);
    release_mem_region(dev.res.start, size);
    }
    EXPORT_SYMBOL(amba_release_regions);
