//! Automatically rewritten from C to Rust
//! Source: drivers/bus/mips_cdmm.c
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
// Bus driver for MIPS Common Device Memory Map (CDMM).
//
// Copyright (C) 2014-2015 Imagination Technologies Ltd.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

// Access control and status register fields
pub const CDMM_ACSR_DEVTYPE_SHIFT: c_int = 24;

pub const CDMM_ACSR_DEVSIZE_SHIFT: c_int = 16;

pub const CDMM_ACSR_DEVREV_SHIFT: c_int = 12;

// Each block of device registers is 64 bytes
pub const CDMM_DRB_SIZE: c_int = 64;

// Default physical base address
    static phys_addr_t mips_cdmm_default_base;
// Bus operations
    static const struct mips_cdmm_device_id *
    mips_cdmm_lookup(const struct mips_cdmm_device_id *table,
    struct mips_cdmm_device *dev)
    {
    let mut ret: c_int = 0;
    for (; table.type; ++table) {
    ret = (dev.type == table.type);
    if (ret)
    break;
    }
    return ret ? table : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mips_cdmm_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int mips_cdmm_match(struct device *dev, const struct device_driver *drv)
    {
    struct mips_cdmm_device *cdev = to_mips_cdmm_device(dev);
    const struct mips_cdmm_driver *cdrv = to_mips_cdmm_driver(drv);
    return mips_cdmm_lookup(cdrv.id_table, cdev) != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mips_cdmm_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int mips_cdmm_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct mips_cdmm_device *cdev = to_mips_cdmm_device(dev);
    let mut retval: c_int = 0;
    retval = add_uevent_var(env, "CDMM_CPU=%u", cdev.cpu);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "CDMM_TYPE=0x%02x", cdev.type);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "CDMM_REV=%u", cdev.rev);
    if (retval)
    return retval;
    retval = add_uevent_var(env, "MODALIAS=mipscdmm:t%02X", cdev.type);
    return retval;
    }
// Device attributes

    static ssize_t name##_show(struct device *_dev,				\
    struct device_attribute *attr, char *buf)	\
    {									\
    struct mips_cdmm_device *dev = to_mips_cdmm_device(_dev);	\
    return sprintf(buf, fmt, arg);					\
    }									\
    static DEVICE_ATTR_RO(name);
    CDMM_ATTR(cpu, "%u\n", dev.cpu);
    CDMM_ATTR(type, "0x%02x\n", dev.type);
    CDMM_ATTR(revision, "%u\n", dev.rev);
    CDMM_ATTR(modalias, "mipscdmm:t%02X\n", dev.type);
    CDMM_ATTR(resource, "\t%016llx\t%016llx\t%016lx\n",
    (unsigned long long)dev.res.start,
    (unsigned long long)dev.res.end,
    dev.res.flags);
    static struct attribute *mips_cdmm_dev_attrs[] = {
    &dev_attr_cpu.attr,
    &dev_attr_type.attr,
    &dev_attr_revision.attr,
    &dev_attr_modalias.attr,
    &dev_attr_resource.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(mips_cdmm_dev);
    const struct bus_type mips_cdmm_bustype = {
    .name		= "cdmm",
    .dev_groups	= mips_cdmm_dev_groups,
    .match		= mips_cdmm_match,
    .uevent		= mips_cdmm_uevent,
    };
    EXPORT_SYMBOL_GPL(mips_cdmm_bustype);
//
// Standard driver callback helpers.
//
// All the CDMM driver callbacks need to be executed on the appropriate CPU from
// workqueues. For the standard driver callbacks we need a work function
// (mips_cdmm_{void,int}_work()) to do the actual call from the right CPU, and a
// wrapper function (generated with BUILD_PERCPU_HELPER) to arrange for the work
// function to be called on that CPU.
//
// struct mips_cdmm_work_dev - Data for per-device call work.
// @fn:		CDMM driver callback function to call for the device.
// @dev:	CDMM device to pass to @fn.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mips_cdmm_work_dev {
    pub fn: *mut c_void,
    pub dev: *mut mips_cdmm_device,
}

//
// mips_cdmm_void_work() - Call a void returning CDMM driver callback.
// @data:	struct mips_cdmm_work_dev pointer.
//
// A work_on_cpu() callback function to call an arbitrary CDMM driver callback
// function which doesn't return a value.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_void_work(data: *mut c_void) -> c_long {
    static long mips_cdmm_void_work(void *data)
    {
    struct mips_cdmm_work_dev *work = data;
    void (*fn)(struct mips_cdmm_device *) = work.fn;
    fn(work.dev);
    return 0;
    }
//
// mips_cdmm_int_work() - Call an int returning CDMM driver callback.
// @data:	struct mips_cdmm_work_dev pointer.
//
// A work_on_cpu() callback function to call an arbitrary CDMM driver callback
// function which returns an int.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_int_work(data: *mut c_void) -> c_long {
    static long mips_cdmm_int_work(void *data)
    {
    struct mips_cdmm_work_dev *work = data;
    int (*fn)(struct mips_cdmm_device *) = work.fn;
    return fn(work.dev);
    }
// Macro flag: #define _BUILD_RET_void

//
// BUILD_PERCPU_HELPER() - Helper to call a CDMM driver callback on right CPU.
// @_ret:	Return type (void or int).
// @_name:	Name of CDMM driver callback function.
//
// Generates a specific device callback function to call a CDMM driver callback
// function on the appropriate CPU for the device, and if applicable return the
// result.
//

    static _ret mips_cdmm_##_name(struct device *dev)			\
    {									\
    struct mips_cdmm_device *cdev = to_mips_cdmm_device(dev);	\
    struct mips_cdmm_driver *cdrv = to_mips_cdmm_driver(dev.driver); \
    struct mips_cdmm_work_dev work = {				\
    .fn	= cdrv._name,					\
    .dev	= cdev,						\
    };								\
    \
    _BUILD_RET_##_ret work_on_cpu(cdev.cpu,			\
    mips_cdmm_##_ret##_work, &work);	\
    }
// Driver callback functions
    BUILD_PERCPU_HELPER(int, probe)     /* int mips_cdmm_probe(struct device) */
    BUILD_PERCPU_HELPER(int, remove)    /* int mips_cdmm_remove(struct device) */
    BUILD_PERCPU_HELPER(void, shutdown) /* void mips_cdmm_shutdown(struct device) */
// Driver registration
//
// mips_cdmm_driver_register() - Register a CDMM driver.
// @drv:	CDMM driver information.
//
// Register a CDMM driver with the CDMM subsystem. The driver will be informed
// of matching devices which are discovered.
//
// Returns:	0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn mips_cdmm_driver_register(drv: *mut mips_cdmm_driver) -> c_int {
    int mips_cdmm_driver_register(struct mips_cdmm_driver *drv)
    {
    drv.drv.bus = &mips_cdmm_bustype;
    if (drv.probe)
    drv.drv.probe = mips_cdmm_probe;
    if (drv.remove)
    drv.drv.remove = mips_cdmm_remove;
    if (drv.shutdown)
    drv.drv.shutdown = mips_cdmm_shutdown;
    return driver_register(&drv.drv);
    }
    EXPORT_SYMBOL_GPL(mips_cdmm_driver_register);
//
// mips_cdmm_driver_unregister() - Unregister a CDMM driver.
// @drv:	CDMM driver information.
//
// Unregister a CDMM driver from the CDMM subsystem.
//
#[no_mangle]
pub unsafe extern "C" fn mips_cdmm_driver_unregister(drv: *mut mips_cdmm_driver) {
    void mips_cdmm_driver_unregister(struct mips_cdmm_driver *drv)
    {
    driver_unregister(&drv.drv);
    }
    EXPORT_SYMBOL_GPL(mips_cdmm_driver_unregister);
// CDMM initialisation and bus discovery
//
// struct mips_cdmm_bus - Info about CDMM bus.
// @phys:		Physical address at which it is mapped.
// @regs:		Virtual address where registers can be accessed.
// @drbs:		Total number of DRBs.
// @drbs_reserved:	Number of DRBs reserved.
// @discovered:		Whether the devices on the bus have been discovered yet.
// @offline:		Whether the CDMM bus is going offline (or very early
// coming back online), in which case it should be
// reconfigured each time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mips_cdmm_bus {
    pub phys: phys_addr_t,
    pub regs: *mut void __iomem,
    pub drbs: c_uint,
    pub drbs_reserved: c_uint,
    pub discovered: bool,
    pub offline: bool,
}

    static struct mips_cdmm_bus mips_cdmm_boot_bus;
    static DEFINE_PER_CPU(struct mips_cdmm_bus *, mips_cdmm_buses);
    let mut mips_cdmm_next_id: static atomic_t = ATOMIC_INIT(-1);
//
// mips_cdmm_get_bus() - Get the per-CPU CDMM bus information.
//
// Get information about the per-CPU CDMM bus, if the bus is present.
//
// The caller must prevent migration to another CPU, either by disabling
// pre-emption or by running from a pinned kernel thread.
//
// Returns:	Pointer to CDMM bus information for the current CPU.
// May return ERR_PTR(-errno) in case of error, so check with
// IS_ERR().
//
    static struct mips_cdmm_bus *mips_cdmm_get_bus(void)
    {
    struct mips_cdmm_bus *bus, **bus_p;
    unsigned long flags;
    unsigned int cpu;
    if (!cpu_has_cdmm)
    return ERR_PTR(-ENODEV);
    cpu = smp_processor_id();
// Avoid early use of per-cpu primitives before initialised
    if (cpu == 0)
    return &mips_cdmm_boot_bus;
// Get bus pointer
    bus_p = per_cpu_ptr(&mips_cdmm_buses, cpu);
    local_irq_save(flags);
    bus = *bus_p;
// Attempt allocation if NULL
    if (unlikely(!bus)) {
    bus = kzalloc_obj(*bus, GFP_ATOMIC);
    if (unlikely(!bus))
    bus = ERR_PTR(-ENOMEM);
    else
// bus_p = bus;
    }
    local_irq_restore(flags);
    return bus;
    }
//
// mips_cdmm_cur_base() - Find current physical base address of CDMM region.
//
// Returns:	Physical base address of CDMM region according to cdmmbase CP0
// register, or 0 if the CDMM region is disabled.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_cur_base() -> phys_addr_t {
    static phys_addr_t mips_cdmm_cur_base(void)
    {
    let mut cdmmbase: c_ulong = read_c0_cdmmbase();
    if (!(cdmmbase & MIPS_CDMMBASE_EN))
    return 0;
    return (cdmmbase >> MIPS_CDMMBASE_ADDR_SHIFT)
    << MIPS_CDMMBASE_ADDR_START;
    }
//
// mips_cdmm_phys_base() - Choose a physical base address for CDMM region.
//
// Picking a suitable physical address at which to map the CDMM region is
// platform specific, so this weak function can be overridden by platform
// code to pick a suitable value if none is configured by the bootloader.
// By default this method tries to find a CDMM-specific node in the system
// dtb. Note that this won't work for early serial console.
//
#[no_mangle]
pub unsafe extern "C" fn mips_cdmm_phys_base() -> phys_addr_t __weak {
    phys_addr_t __weak mips_cdmm_phys_base(void)
    {
    struct device_node *np;
    struct resource res;
    int err;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "mti,mips-cdmm");
    if (np) {
    err = of_address_to_resource(np, 0, &res);
    of_node_put(np);
    if (!err)
    return res.start;
    }
    return 0;
    }
//
// mips_cdmm_setup() - Ensure the CDMM bus is initialised and usable.
// @bus:	Pointer to bus information for current CPU.
// IS_ERR(bus) is checked, so no need for caller to check.
//
// The caller must prevent migration to another CPU, either by disabling
// pre-emption or by running from a pinned kernel thread.
//
// Returns	0 on success, -errno on failure.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_setup(bus: *mut mips_cdmm_bus) -> c_int {
    static int mips_cdmm_setup(struct mips_cdmm_bus *bus)
    {
    unsigned long cdmmbase, flags;
    let mut ret: c_int = 0;
    if (IS_ERR(bus))
    return PTR_ERR(bus);
    local_irq_save(flags);
// Don't set up bus a second time unless marked offline
    if (bus.offline) {
// If CDMM region is still set up, nothing to do
    if (bus.phys == mips_cdmm_cur_base())
    goto out;
//
// The CDMM region isn't set up as expected, so it needs
// reconfiguring, but then we can stop checking it.
//
    bus.offline = false;
    } else if (bus.phys > 1) {
    goto out;
    }
// If the CDMM region is already configured, inherit that setup
    if (!bus.phys)
    bus.phys = mips_cdmm_cur_base();
// Otherwise, ask platform code for suggestions
    if (!bus.phys)
    bus.phys = mips_cdmm_phys_base();
// Otherwise, copy what other CPUs have done
    if (!bus.phys)
    bus.phys = mips_cdmm_default_base;
// Otherwise, complain once
    if (!bus.phys) {
    bus.phys = 1;
//
// If you hit this, either your bootloader needs to set up the
// CDMM on the boot CPU, or else you need to implement
// mips_cdmm_phys_base() for your platform (see asm/cdmm.h).
//
    pr_err("cdmm%u: Failed to choose a physical base\n",
    smp_processor_id());
    }
// Already complained?
    if (bus.phys == 1) {
    ret = -ENOMEM;
    goto out;
    }
// Record our success for other CPUs to copy
    mips_cdmm_default_base = bus.phys;
    pr_debug("cdmm%u: Enabling CDMM region at %pa\n",
    smp_processor_id(), &bus.phys);
// Enable CDMM
    cdmmbase = read_c0_cdmmbase();
    cdmmbase &= (1ul << MIPS_CDMMBASE_ADDR_SHIFT) - 1;
    cdmmbase |= (bus.phys >> MIPS_CDMMBASE_ADDR_START)
    << MIPS_CDMMBASE_ADDR_SHIFT;
    cdmmbase |= MIPS_CDMMBASE_EN;
    write_c0_cdmmbase(cdmmbase);
    tlbw_use_hazard();
    bus.regs = (void __iomem *)CKSEG1ADDR(bus.phys);
    bus.drbs = 1 + ((cdmmbase & MIPS_CDMMBASE_SIZE) >>
    MIPS_CDMMBASE_SIZE_SHIFT);
    bus.drbs_reserved = !!(cdmmbase & MIPS_CDMMBASE_CI);
    out:
    local_irq_restore(flags);
    return ret;
    }
//
// mips_cdmm_early_probe() - Minimally probe for a specific device on CDMM.
// @dev_type:	CDMM type code to look for.
//
// Minimally configure the in-CPU Common Device Memory Map (CDMM) and look for a
// specific device. This can be used to find a device very early in boot for
// example to configure an early FDC console device.
//
// The caller must prevent migration to another CPU, either by disabling
// pre-emption or by running from a pinned kernel thread.
//
// Returns:	MMIO pointer to device memory. The caller can read the ACSR
// register to find more information about the device (such as the
// version number or the number of blocks).
// May return IOMEM_ERR_PTR(-errno) in case of error, so check with
// IS_ERR().
//
    void __iomem *mips_cdmm_early_probe(unsigned int dev_type)
    {
    struct mips_cdmm_bus *bus;
    void __iomem *cdmm;
    u32 acsr;
    unsigned int drb, type, size;
    int err;
    if (WARN_ON(!dev_type))
    return IOMEM_ERR_PTR(-ENODEV);
    bus = mips_cdmm_get_bus();
    err = mips_cdmm_setup(bus);
    if (err)
    return IOMEM_ERR_PTR(err);
// Skip the first block if it's reserved for more registers
    drb = bus.drbs_reserved;
    cdmm = bus.regs;
// Look for a specific device type
    for (; drb < bus.drbs; drb += size + 1) {
    acsr = __raw_readl(cdmm + drb * CDMM_DRB_SIZE);
    type = (acsr & CDMM_ACSR_DEVTYPE) >> CDMM_ACSR_DEVTYPE_SHIFT;
    if (type == dev_type)
    return cdmm + drb * CDMM_DRB_SIZE;
    size = (acsr & CDMM_ACSR_DEVSIZE) >> CDMM_ACSR_DEVSIZE_SHIFT;
    }
    return IOMEM_ERR_PTR(-ENODEV);
    }
    EXPORT_SYMBOL_GPL(mips_cdmm_early_probe);
//
// mips_cdmm_release() - Release a removed CDMM device.
// @dev:	Device object
//
// Clean up the struct mips_cdmm_device for an unused CDMM device. This is
// called automatically by the driver core when a device is removed.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_release(dev: *mut device) {
    static void mips_cdmm_release(struct device *dev)
    {
    struct mips_cdmm_device *cdev = to_mips_cdmm_device(dev);
    kfree(cdev);
    }
//
// mips_cdmm_bus_discover() - Discover the devices on the CDMM bus.
// @bus:	CDMM bus information, must already be set up.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_bus_discover(bus: *mut mips_cdmm_bus) {
    static void mips_cdmm_bus_discover(struct mips_cdmm_bus *bus)
    {
    void __iomem *cdmm;
    u32 acsr;
    unsigned int drb, type, size, rev;
    struct mips_cdmm_device *dev;
    let mut cpu: c_uint = smp_processor_id();
    let mut ret: c_int = 0;
    let mut id: c_int = 0;
// Skip the first block if it's reserved for more registers
    drb = bus.drbs_reserved;
    cdmm = bus.regs;
// Discover devices
    bus.discovered = true;
    pr_info("cdmm%u discovery (%u blocks)\n", cpu, bus.drbs);
    for (; drb < bus.drbs; drb += size + 1) {
    acsr = __raw_readl(cdmm + drb * CDMM_DRB_SIZE);
    type = (acsr & CDMM_ACSR_DEVTYPE) >> CDMM_ACSR_DEVTYPE_SHIFT;
    size = (acsr & CDMM_ACSR_DEVSIZE) >> CDMM_ACSR_DEVSIZE_SHIFT;
    rev  = (acsr & CDMM_ACSR_DEVREV)  >> CDMM_ACSR_DEVREV_SHIFT;
    if (!type)
    continue;
    pr_info("cdmm%u-%u: @%u (%#x..%#x), type 0x%02x, rev %u\n",
    cpu, id, drb, drb * CDMM_DRB_SIZE,
    (drb + size + 1) * CDMM_DRB_SIZE - 1,
    type, rev);
    dev = kzalloc_obj(*dev);
    if (!dev)
    break;
    dev.cpu = cpu;
    dev.res.start = bus.phys + drb * CDMM_DRB_SIZE;
    dev.res.end = bus.phys +
    (drb + size + 1) * CDMM_DRB_SIZE - 1;
    dev.res.flags = IORESOURCE_MEM;
    dev.type = type;
    dev.rev = rev;
    dev.dev.parent = get_cpu_device(cpu);
    dev.dev.bus = &mips_cdmm_bustype;
    dev.dev.id = atomic_inc_return(&mips_cdmm_next_id);
    dev.dev.release = mips_cdmm_release;
    dev_set_name(&dev.dev, "cdmm%u-%u", cpu, id);
    ++id;
    ret = device_register(&dev.dev);
    if (ret)
    put_device(&dev.dev);
    }
    }
//
// CPU hotplug and initialisation
//
// All the CDMM driver callbacks need to be executed on the appropriate CPU from
// workqueues. For the CPU callbacks, they need to be called for all devices on
// that CPU, so the work function calls bus_for_each_dev, using a helper
// (generated with BUILD_PERDEV_HELPER) to call the driver callback if the
// device's CPU matches.
//
// BUILD_PERDEV_HELPER() - Helper to call a CDMM driver callback if CPU matches.
// @_name:	Name of CDMM driver callback function.
//
// Generates a bus_for_each_dev callback function to call a specific CDMM driver
// callback function for the device if the device's CPU matches that pointed to
// by the data argument.
//
// This is used for informing drivers for all devices on a given CPU of some
// event (such as the CPU going online/offline).
//
// It is expected to already be called from the appropriate CPU.
//

    static int mips_cdmm_##_name##_helper(struct device *dev, void *data)	\
    {									\
    struct mips_cdmm_device *cdev = to_mips_cdmm_device(dev);	\
    struct mips_cdmm_driver *cdrv;					\
    unsigned int cpu = *(unsigned int *)data;			\
    \
    if (cdev.cpu != cpu || !dev.driver)				\
    return 0;						\
    \
    cdrv = to_mips_cdmm_driver(dev.driver);			\
    if (!cdrv._name)						\
    return 0;						\
    return cdrv._name(cdev);					\
    }
// bus_for_each_dev callback helper functions
    BUILD_PERDEV_HELPER(cpu_down)       /* int mips_cdmm_cpu_down_helper(...) */
    BUILD_PERDEV_HELPER(cpu_up)         /* int mips_cdmm_cpu_up_helper(...) */
//
// mips_cdmm_cpu_down_prep() - Callback for CPUHP DOWN_PREP:
// Tear down the CDMM bus.
// @cpu:	unsigned int CPU number.
//
// This function is executed on the hotplugged CPU and calls the CDMM
// driver cpu_down callback for all devices on that CPU.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_cpu_down_prep(cpu: c_uint) -> c_int {
    static int mips_cdmm_cpu_down_prep(unsigned int cpu)
    {
    struct mips_cdmm_bus *bus;
    long ret;
// Inform all the devices on the bus
    ret = bus_for_each_dev(&mips_cdmm_bustype, core::ptr::null_mut(), &cpu,
    mips_cdmm_cpu_down_helper);
//
// While bus is offline, each use of it should reconfigure it just in
// case it is first use when coming back online again.
//
    bus = mips_cdmm_get_bus();
    if (!IS_ERR(bus))
    bus.offline = true;
    return ret;
    }
//
// mips_cdmm_cpu_online() - Callback for CPUHP ONLINE: Bring up the CDMM bus.
// @cpu:	unsigned int CPU number.
//
// This work_on_cpu callback function is executed on a given CPU to discover
// CDMM devices on that CPU, or to call the CDMM driver cpu_up callback for all
// devices already discovered on that CPU.
//
// It is used as work_on_cpu callback function during
// initialisation. When CPUs are brought online the function is
// invoked directly on the hotplugged CPU.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_cpu_online(cpu: c_uint) -> c_int {
    static int mips_cdmm_cpu_online(unsigned int cpu)
    {
    struct mips_cdmm_bus *bus;
    long ret;
    bus = mips_cdmm_get_bus();
    ret = mips_cdmm_setup(bus);
    if (ret)
    return ret;
// Bus now set up, so we can drop the offline flag if still set
    bus.offline = false;
    if (!bus.discovered)
    mips_cdmm_bus_discover(bus);
    else
// Inform all the devices on the bus
    ret = bus_for_each_dev(&mips_cdmm_bustype, core::ptr::null_mut(), &cpu,
    mips_cdmm_cpu_up_helper);
    return ret;
    }
//
// mips_cdmm_init() - Initialise CDMM bus.
//
// Initialise CDMM bus, discover CDMM devices for online CPUs, and arrange for
// hotplug notifications so the CDMM drivers can be kept up to date.
//
#[no_mangle]
unsafe extern "C" fn mips_cdmm_init() -> int __init {
    static int __init mips_cdmm_init(void)
    {
    int ret;
// Register the bus
    ret = bus_register(&mips_cdmm_bustype);
    if (ret)
    return ret;
// We want to be notified about new CPUs
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "bus/cdmm:online",
    mips_cdmm_cpu_online, mips_cdmm_cpu_down_prep);
    if (ret < 0)
    pr_warn("cdmm: Failed to register CPU notifier\n");
    return ret;
    }
    subsys_initcall(mips_cdmm_init);
