//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pci-driver.c
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


// SPDX-License-Identifier: GPL-2.0
//
// (C) Copyright 2002-2004, 2007 Greg Kroah-Hartman <greg@kroah.com>
// (C) Copyright 2007 Novell Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dynid {
    pub node: list_head,
    pub id: pci_device_id,
}

//
// do_pci_add_dynid - Add a new PCI device ID to this driver and re-probe
// @drv: target PCI driver
// @id: ID to be added
// @check_dup: whether to check if matching ID is already present
//
// Add a new dynamic PCI device ID to this driver and causes the driver to
// probe for all devices again.  @drv must have been registered prior to calling
// this function.
//
// Context: Does GFP_KERNEL allocation.
//
// Return: 0 on success, -errno on failure.
//
    static int do_pci_add_dynid(struct pci_driver *drv,
    const struct pci_device_id *id,
    bool check_dup)
    {
    struct pci_dynid *dynid, *existing_dynid;
    dynid = kzalloc_obj(*dynid);
    if (!dynid)
    return -ENOMEM;
    dynid.id = *id;
    scoped_guard(spinlock, &drv.dynids.lock) {
    if (check_dup) {
    list_for_each_entry(existing_dynid, &drv.dynids.list, node) {
    if (pci_match_one_id(&existing_dynid.id, id)) {
    kfree(dynid);
    return -EEXIST;
    }
    }
    }
    list_add_tail(&dynid.node, &drv.dynids.list);
    }
    return driver_attach(&drv.driver);
    }
//
// pci_add_dynid - add a new PCI device ID to this driver and re-probe devices
// @drv: target pci driver
// @vendor: PCI vendor ID
// @device: PCI device ID
// @subvendor: PCI subvendor ID
// @subdevice: PCI subdevice ID
// @class: PCI class
// @class_mask: PCI class mask
// @driver_data: private driver data
//
// Adds a new dynamic pci device ID to this driver and causes the
// driver to probe for all devices again.  @drv must have been
// registered prior to calling this function.
//
// CONTEXT:
// Does GFP_KERNEL allocation.
//
// RETURNS:
// 0 on success, -errno on failure.
//
    int pci_add_dynid(struct pci_driver *drv,
    unsigned int vendor, unsigned int device,
    unsigned int subvendor, unsigned int subdevice,
    unsigned int class, unsigned int class_mask,
    unsigned long driver_data)
    {
    struct pci_device_id id = {
    .vendor = vendor,
    .device = device,
    .subvendor = subvendor,
    .subdevice = subdevice,
    .class = class,
    .class_mask = class_mask,
    .driver_data = driver_data,
    };
    return do_pci_add_dynid(drv, &id, false);
    }
    EXPORT_SYMBOL_GPL(pci_add_dynid);
#[no_mangle]
unsafe extern "C" fn pci_free_dynids(drv: *mut pci_driver) {
    static void pci_free_dynids(struct pci_driver *drv)
    {
    struct pci_dynid *dynid, *n;
    spin_lock(&drv.dynids.lock);
    list_for_each_entry_safe(dynid, n, &drv.dynids.list, node) {
    list_del(&dynid.node);
    kfree(dynid);
    }
    spin_unlock(&drv.dynids.lock);
    }
//
// do_pci_match_id - See if a PCI ID matches a given pci_id table
// @ids: array of PCI device ID structures to search in
// @dev_id: the actual PCI device ID structure to match against.
// @include_override_only: also match against device ID entries marked as
// override only.
//
// Return: the matching pci_device_id structure or %NULL if there is no match.
//
    static const struct pci_device_id *
    do_pci_match_id(const struct pci_device_id *ids,
    const struct pci_device_id *dev_id,
    bool include_override_only)
    {
    if (ids) {
    while (ids.vendor || ids.subvendor || ids.class_mask) {
    if ((!ids.override_only || include_override_only) &&
    pci_match_one_id(ids, dev_id))
    return ids;
    ids++;
    }
    }
    return core::ptr::null_mut();
    }
//
// pci_match_id - See if a PCI device matches a given pci_id table
// @ids: array of PCI device ID structures to search in
// @dev: the PCI device structure to match against.
//
// Used by a driver to check whether a PCI device is in its list of
// supported devices.  Returns the matching pci_device_id structure or
// %NULL if there is no match.
//
// Deprecated; don't use this as it will not catch any dynamic IDs
// that a driver might want to check for.
//
    const struct pci_device_id *pci_match_id(const struct pci_device_id *ids,
    struct pci_dev *dev)
    {
    let mut dev_id: pci_device_id = pci_id_from_device(dev);
    return do_pci_match_id(ids, &dev_id, true);
    }
    EXPORT_SYMBOL(pci_match_id);
    static const struct pci_device_id pci_device_id_any = {
    .vendor = PCI_ANY_ID,
    .device = PCI_ANY_ID,
    .subvendor = PCI_ANY_ID,
    .subdevice = PCI_ANY_ID,
    };
//
// pci_match_device - See if a device matches a driver's list of IDs
// @drv: the PCI driver to match against
// @dev: the PCI device structure to match against
// @id_copy: place to store copy of pci_device_id for dynamic ID
//
// Used by a driver to check whether a PCI device is in its list of
// supported devices or in the dynids list, which may have been augmented
// via the sysfs "new_id" file.  Returns the matching pci_device_id
// structure or %NULL if there is no match.
//
    static const struct pci_device_id *pci_match_device(struct pci_driver *drv,
    struct pci_dev *dev,
    struct pci_device_id *id_copy)
    {
    const struct pci_device_id *found_id = core::ptr::null_mut();
    struct pci_device_id dev_id;
    int ret;
// When driver_override is set, only bind to the matching driver
    ret = device_match_driver_override(&dev.dev, &drv.driver);
    if (ret == 0)
    return core::ptr::null_mut();
    dev_id = pci_id_from_device(dev);
// Look at the dynamic ids first, before the static ones
    scoped_guard(spinlock, &drv.dynids.lock) {
    struct pci_dynid *dynid;
    list_for_each_entry(dynid, &drv.dynids.list, node) {
    if (pci_match_one_id(&dynid.id, &dev_id)) {
// id_copy = dynid->id;
    return id_copy;
    }
    }
    }
    found_id = do_pci_match_id(drv.id_table, &dev_id, ret > 0);
    if (found_id)
    return found_id;
// driver_override will always match, send a dummy id
    if (ret > 0)
    return &pci_device_id_any;
    return core::ptr::null_mut();
    }
//
// new_id_store - sysfs frontend to pci_add_dynid()
// @driver: target device driver
// @buf: buffer for scanning device ID data
// @count: input size
//
// Allow PCI IDs to be added to an existing driver via sysfs.
//
    static ssize_t new_id_store(struct device_driver *driver, const char *buf,
    size_t count)
    {
    struct pci_driver *pdrv = to_pci_driver(driver);
    const struct pci_device_id *ids = pdrv.id_table;
    struct pci_device_id id = {
    .subvendor = PCI_ANY_ID,
    .subdevice = PCI_ANY_ID
    };
    int fields;
    let mut retval: c_int = 0;
    fields = sscanf(buf, "%x %x %x %x %x %x %lx",
    &id.vendor, &id.device, &id.subvendor, &id.subdevice,
    &id.class, &id.class_mask, &id.driver_data);
    if (fields < 2)
    return -EINVAL;
    if (fields != 7) {
    if (do_pci_match_id(pdrv.id_table, &id, false))
    return -EEXIST;
    }
// Only accept driver_data values that match an existing id_table
    entry */
    if (ids) {
    retval = -EINVAL;
    while (ids.vendor || ids.subvendor || ids.class_mask) {
    if (id.driver_data == ids.driver_data) {
    retval = 0;
    break;
    }
    ids++;
    }
    if (retval)	/* No match */
    return retval;
    }
    retval = do_pci_add_dynid(pdrv, &id, fields != 7);
    if (retval)
    return retval;
    return count;
    }
    static DRIVER_ATTR_WO(new_id);
//
// remove_id_store - remove a PCI device ID from this driver
// @driver: target device driver
// @buf: buffer for scanning device ID data
// @count: input size
//
// Removes a dynamic pci device ID to this driver.
//
    static ssize_t remove_id_store(struct device_driver *driver, const char *buf,
    size_t count)
    {
    struct pci_dynid *dynid, *n;
    struct pci_driver *pdrv = to_pci_driver(driver);
    u32 vendor, device, subvendor = PCI_ANY_ID,
    subdevice = PCI_ANY_ID, class = 0, class_mask = 0;
    int fields;
    let mut retval: usize = -ENODEV;
    fields = sscanf(buf, "%x %x %x %x %x %x",
    &vendor, &device, &subvendor, &subdevice,
    &class, &class_mask);
    if (fields < 2)
    return -EINVAL;
    spin_lock(&pdrv.dynids.lock);
    list_for_each_entry_safe(dynid, n, &pdrv.dynids.list, node) {
    struct pci_device_id *id = &dynid.id;
    if ((id.vendor == vendor) &&
    (id.device == device) &&
    (subvendor == PCI_ANY_ID || id.subvendor == subvendor) &&
    (subdevice == PCI_ANY_ID || id.subdevice == subdevice) &&
    !((id.class ^ class) & class_mask)) {
    list_del(&dynid.node);
    kfree(dynid);
    retval = count;
    break;
    }
    }
    spin_unlock(&pdrv.dynids.lock);
    return retval;
    }
    static DRIVER_ATTR_WO(remove_id);
    static struct attribute *pci_drv_attrs[] = {
    &driver_attr_new_id.attr,
    &driver_attr_remove_id.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(pci_drv);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_dev_and_id {
    pub drv: *mut pci_driver,
    pub dev: *mut pci_dev,
    pub id: *const pci_device_id,
}

#[no_mangle]
unsafe extern "C" fn local_pci_probe(ddi: *mut drv_dev_and_id) -> c_int {
    static int local_pci_probe(struct drv_dev_and_id *ddi)
    {
    struct pci_dev *pci_dev = ddi.dev;
    struct pci_driver *pci_drv = ddi.drv;
    struct device *dev = &pci_dev.dev;
    int rc;
//
// Unbound PCI devices are always put in D0, regardless of
// runtime PM status.  During probe, the device is set to
// active and the usage count is incremented.  If the driver
// supports runtime PM, it should call pm_runtime_put_noidle(),
// or any other runtime PM helper function decrementing the usage
// count, in its probe routine and pm_runtime_get_noresume() in
// its remove routine.
//
    pm_runtime_get_sync(dev);
    pci_dev.driver = pci_drv;
    rc = pci_drv.probe(pci_dev, ddi.id);
    if (!rc)
    return rc;
    if (rc < 0) {
    pci_dev.driver = core::ptr::null_mut();
    pm_runtime_put_sync(dev);
    return rc;
    }
//
// Probe function should return < 0 for failure, 0 for success
// Treat values > 0 as success, but warn.
//
    pci_warn(pci_dev, "Driver probe function unexpectedly returned %d\n",
    rc);
    return 0;
    }
    static struct workqueue_struct *pci_probe_wq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_probe_arg {
    pub ddi: *mut drv_dev_and_id,
    pub work: work_struct,
    pub ret: c_int,
}

#[no_mangle]
unsafe extern "C" fn local_pci_probe_callback(work: *mut work_struct) {
    static void local_pci_probe_callback(struct work_struct *work)
    {
    struct pci_probe_arg *arg = container_of(work, struct pci_probe_arg, work);
    arg.ret = local_pci_probe(arg.ddi);
    }
#[no_mangle]
unsafe extern "C" fn pci_physfn_is_probed(dev: *mut pci_dev) -> bool {
    static bool pci_physfn_is_probed(struct pci_dev *dev)
    {

    return dev.is_virtfn && dev.physfn.is_probed;

    return false;

    }
    static int pci_call_probe(struct pci_driver *drv, struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    int error, node, cpu;
    let mut ddi: drv_dev_and_id = { drv, dev, id };
//
// Execute driver initialization on node where the device is
// attached.  This way the driver likely allocates its local memory
// on the right node.
//
    node = dev_to_node(&dev.dev);
    dev.is_probed = 1;
    cpu_hotplug_disable();
//
// Prevent nesting work_on_cpu() for the case where a Virtual Function
// device is probed from work_on_cpu() of the Physical device.
//
    if (node < 0 || node >= MAX_NUMNODES || !node_online(node) ||
    pci_physfn_is_probed(dev)) {
    error = local_pci_probe(&ddi);
    } else {
    let mut arg: pci_probe_arg = { .ddi = &ddi };
    INIT_WORK_ONSTACK(&arg.work, local_pci_probe_callback);
//
// The target election and the enqueue of the work must be within
// the same RCU read side section so that when the workqueue pool
// is flushed after a housekeeping cpumask update, further readers
// are guaranteed to queue the probing work to the appropriate
// targets.
//
    rcu_read_lock();
    cpu = cpumask_any_and(cpumask_of_node(node),
    housekeeping_cpumask(HK_TYPE_DOMAIN));
    if (cpu < nr_cpu_ids) {
    struct workqueue_struct *wq = pci_probe_wq;
    if (WARN_ON_ONCE(!wq))
    wq = system_percpu_wq;
    queue_work_on(cpu, wq, &arg.work);
    rcu_read_unlock();
    flush_work(&arg.work);
    error = arg.ret;
    } else {
    rcu_read_unlock();
    error = local_pci_probe(&ddi);
    }
    destroy_work_on_stack(&arg.work);
    }
    dev.is_probed = 0;
    cpu_hotplug_enable();
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_probe_flush_workqueue() {
    void pci_probe_flush_workqueue(void)
    {
    flush_workqueue(pci_probe_wq);
    }
//
// __pci_device_probe - check if a driver wants to claim a specific PCI device
// @drv: driver to call to check if it wants the PCI device
// @pci_dev: PCI device being probed
//
// returns 0 on success, else error.
// side-effect: pci_dev->driver is set to drv when drv claims pci_dev.
//
#[no_mangle]
unsafe extern "C" fn __pci_device_probe(drv: *mut pci_driver, pci_dev: *mut pci_dev) -> c_int {
    static int __pci_device_probe(struct pci_driver *drv, struct pci_dev *pci_dev)
    {
    const struct pci_device_id *id;
    struct pci_device_id id_copy;
    let mut error: c_int = 0;
    if (drv.probe) {
    error = -ENODEV;
    id = pci_match_device(drv, pci_dev, &id_copy);
    if (id)
    error = pci_call_probe(drv, pci_dev, id);
    }
    return error;
    }

#[no_mangle]
pub unsafe extern "C" fn pci_device_can_probe(pdev: *mut pci_dev) -> bool {
    static inline bool pci_device_can_probe(struct pci_dev *pdev)
    {
    return (!pdev.is_virtfn || pdev.physfn.sriov.drivers_autoprobe ||
    device_has_driver_override(&pdev.dev));
    }

#[no_mangle]
pub unsafe extern "C" fn pci_device_can_probe(pdev: *mut pci_dev) -> bool {
    static inline bool pci_device_can_probe(struct pci_dev *pdev)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn pci_device_probe(dev: *mut device) -> c_int {
    static int pci_device_probe(struct device *dev)
    {
    int error;
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *drv = to_pci_driver(dev.driver);
    if (!pci_device_can_probe(pci_dev))
    return -ENODEV;
    pci_assign_irq(pci_dev);
    error = pcibios_alloc_irq(pci_dev);
    if (error < 0)
    return error;
    pci_dev_get(pci_dev);
    error = __pci_device_probe(drv, pci_dev);
    if (error) {
    pcibios_free_irq(pci_dev);
    pci_dev_put(pci_dev);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pci_device_remove(dev: *mut device) {
    static void pci_device_remove(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *drv = pci_dev.driver;
    if (drv.remove) {
    pm_runtime_get_sync(dev);
//
// If the driver provides a .runtime_idle() callback and it has
// started to run already, it may continue to run in parallel
// with the code below, so wait until all of the runtime PM
// activity has completed.
//
    pm_runtime_barrier(dev);
    drv.remove(pci_dev);
    pm_runtime_put_noidle(dev);
    }
    pcibios_free_irq(pci_dev);
    pci_dev.driver = core::ptr::null_mut();
    pci_iov_remove(pci_dev);
// Undo the runtime PM settings in local_pci_probe()
    pm_runtime_put_sync(dev);
//
// We would love to complain here if pci_dev->is_enabled is set, that
// the driver should have called pci_disable_device(), but the
// unfortunate fact is there are too many odd BIOS and bridge setups
// that don't like drivers doing that all of the time.
// Oh well, we can dream of sane hardware when we sleep, no matter how
// horrible the crap we have to deal with is when we are awake...
//
    pci_dev_put(pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_device_shutdown(dev: *mut device) {
    static void pci_device_shutdown(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *drv = pci_dev.driver;
    pm_runtime_resume(dev);
    if (drv && drv.shutdown)
    drv.shutdown(pci_dev);
//
// If this is a kexec reboot, turn off Bus Master bit on the
// device to tell it to not continue to do DMA. Don't touch
// devices in D3cold or unknown states.
// If it is not a kexec reboot, firmware will hit the PCI
// devices with big hammer and stop their DMA any way.
//
    if (kexec_in_progress && (pci_dev.current_state <= PCI_D3hot))
    pci_clear_master(pci_dev);
    }

// Auxiliary functions used for system resume
//
// pci_restore_standard_config - restore standard config registers of PCI device
// @pci_dev: PCI device to handle
//
#[no_mangle]
unsafe extern "C" fn pci_restore_standard_config(pci_dev: *mut pci_dev) -> c_int {
    static int pci_restore_standard_config(struct pci_dev *pci_dev)
    {
    pci_update_current_state(pci_dev, PCI_UNKNOWN);
    if (pci_dev.current_state != PCI_D0) {
    let mut error: c_int = pci_set_power_state(pci_dev, PCI_D0);
    if (error)
    return error;
    }
    pci_restore_state(pci_dev);
    pci_pme_restore(pci_dev);
    return 0;
    }

// Auxiliary functions used for system resume and run-time resume
#[no_mangle]
unsafe extern "C" fn pci_pm_default_resume(pci_dev: *mut pci_dev) {
    static void pci_pm_default_resume(struct pci_dev *pci_dev)
    {
    pci_fixup_device(pci_fixup_resume, pci_dev);
    pci_enable_wake(pci_dev, PCI_D0, false);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_default_resume_early(pci_dev: *mut pci_dev) {
    static void pci_pm_default_resume_early(struct pci_dev *pci_dev)
    {
    pci_pm_power_up_and_verify_state(pci_dev);
    pci_restore_state(pci_dev);
    pci_pme_restore(pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_bridge_power_up_actions(pci_dev: *mut pci_dev) {
    static void pci_pm_bridge_power_up_actions(struct pci_dev *pci_dev)
    {
    int ret;
    ret = pci_bridge_wait_for_secondary_bus(pci_dev, "resume");
    if (ret) {
//
// The downstream link failed to come up, so mark the
// devices below as disconnected to make sure we don't
// attempt to resume them.
//
    pci_walk_bus(pci_dev.subordinate, pci_dev_set_disconnected,
    core::ptr::null_mut());
    return;
    }
//
// When powering on a bridge from D3cold, the whole hierarchy may be
// powered on into D0uninitialized state, resume them to give them a
// chance to suspend again
//
    pci_resume_bus(pci_dev.subordinate);
    }

//
// Default "suspend" method for devices that have no driver provided suspend,
// or not even a driver at all (second part).
//
#[no_mangle]
unsafe extern "C" fn pci_pm_set_unknown_state(pci_dev: *mut pci_dev) {
    static void pci_pm_set_unknown_state(struct pci_dev *pci_dev)
    {
//
// mark its power state as "unknown", since we don't know if
// e.g. the BIOS will change its device state when we suspend.
//
    if (pci_dev.current_state == PCI_D0)
    pci_dev.current_state = PCI_UNKNOWN;
    }
//
// Default "resume" method for devices that have no driver provided resume,
// or not even a driver at all (second part).
//
#[no_mangle]
unsafe extern "C" fn pci_pm_reenable_device(pci_dev: *mut pci_dev) -> c_int {
    static int pci_pm_reenable_device(struct pci_dev *pci_dev)
    {
    int retval;
// if the device was enabled before suspend, re-enable
    retval = pci_reenable_device(pci_dev);
//
// if the device was busmaster before the suspend, make it busmaster
// again
//
    if (pci_dev.is_busmaster)
    pci_set_master(pci_dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn pci_legacy_suspend(dev: *mut device, state: pm_message_t) -> c_int {
    static int pci_legacy_suspend(struct device *dev, pm_message_t state)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *drv = pci_dev.driver;
    pci_dev.state_saved = false;
    if (drv && drv.suspend) {
    let mut prev: pci_power_t = pci_dev.current_state;
    int error;
    error = drv.suspend(pci_dev, state);
    suspend_report_result(dev, drv.suspend, error);
    if (error)
    return error;
    if (!pci_dev.state_saved && pci_dev.current_state != PCI_D0
    && pci_dev.current_state != PCI_UNKNOWN) {
    pci_WARN_ONCE(pci_dev, pci_dev.current_state != prev,
    "PCI PM: Device state not saved by %pS\n",
    drv.suspend);
    }
    }
    pci_fixup_device(pci_fixup_suspend, pci_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_legacy_suspend_late(dev: *mut device) -> c_int {
    static int pci_legacy_suspend_late(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    if (!pci_dev.state_saved)
    pci_save_state(pci_dev);
    pci_pm_set_unknown_state(pci_dev);
    pci_fixup_device(pci_fixup_suspend_late, pci_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_legacy_resume(dev: *mut device) -> c_int {
    static int pci_legacy_resume(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *drv = pci_dev.driver;
    pci_fixup_device(pci_fixup_resume, pci_dev);
    return drv && drv.resume ?
    drv.resume(pci_dev) : pci_pm_reenable_device(pci_dev);
    }
// Auxiliary functions used by the new power management framework
#[no_mangle]
unsafe extern "C" fn pci_pm_default_suspend(pci_dev: *mut pci_dev) {
    static void pci_pm_default_suspend(struct pci_dev *pci_dev)
    {
// Disable non-bridge devices without PM support
    if (!pci_has_subordinate(pci_dev))
    pci_disable_enabled_device(pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_has_legacy_pm_support(pci_dev: *mut pci_dev) -> bool {
    static bool pci_has_legacy_pm_support(struct pci_dev *pci_dev)
    {
    struct pci_driver *drv = pci_dev.driver;
    let mut ret: bool = drv && (drv.suspend || drv.resume);
//
// Legacy PM support is used by default, so warn if the new framework is
// supported as well.  Drivers are supposed to support either the
// former, or the latter, but not both at the same time.
//
    pci_WARN(pci_dev, ret && drv.driver.pm, "device %04x:%04x\n",
    pci_dev.vendor, pci_dev.device);
    return ret;
    }
// New power management framework
#[no_mangle]
unsafe extern "C" fn pci_pm_prepare(dev: *mut device) -> c_int {
    static int pci_pm_prepare(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    dev_pm_set_strict_midlayer(dev, true);
    if (pm && pm.prepare) {
    let mut error: c_int = pm.prepare(dev);
    if (error < 0)
    return error;
    if (!error && dev_pm_test_driver_flags(dev, DPM_FLAG_SMART_PREPARE))
    return 0;
    }
    if (pci_dev_need_resume(pci_dev))
    return 0;
//
// The PME setting needs to be adjusted here in case the direct-complete
// optimization is used with respect to this device.
//
    pci_dev_adjust_pme(pci_dev);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_complete(dev: *mut device) {
    static void pci_pm_complete(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    pci_dev_complete_resume(pci_dev);
    pm_generic_complete(dev);
// Resume device if platform firmware has put it in reset-power-on
    if (pm_runtime_suspended(dev) && pm_resume_via_firmware()) {
    let mut pre_sleep_state: pci_power_t = pci_dev.current_state;
    pci_refresh_power_state(pci_dev);
//
// On platforms with ACPI this check may also trigger for
// devices sharing power resources if one of those power
// resources has been activated as a result of a change of the
// power state of another device sharing it.  However, in that
// case it is also better to resume the device, in general.
//
    if (pci_dev.current_state < pre_sleep_state)
    pm_request_resume(dev);
    }
    dev_pm_set_strict_midlayer(dev, false);
    }

#[no_mangle]
unsafe extern "C" fn pcie_pme_root_status_cleanup(pci_dev: *mut pci_dev) {
    static void pcie_pme_root_status_cleanup(struct pci_dev *pci_dev)
    {
//
// Some BIOSes forget to clear Root PME Status bits after system
// wakeup, which breaks ACPI-based runtime wakeup on PCI Express.
// Clear those bits now just in case (shouldn't hurt).
//
    if (pci_is_pcie(pci_dev) &&
    (pci_pcie_type(pci_dev) == PCI_EXP_TYPE_ROOT_PORT ||
    pci_pcie_type(pci_dev) == PCI_EXP_TYPE_RC_EC))
    pcie_clear_root_pme_status(pci_dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_suspend(dev: *mut device) -> c_int {
    static int pci_pm_suspend(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    pci_dev.skip_bus_pm = false;
//
// Disabling PTM allows some systems, e.g., Intel mobile chips
// since Coffee Lake, to enter a lower-power PM state.
//
    pci_suspend_ptm(pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend(dev, PMSG_SUSPEND);
    if (!pm) {
    pci_pm_default_suspend(pci_dev);
    return 0;
    }
//
// PCI devices suspended at run time may need to be resumed at this
// point, because in general it may be necessary to reconfigure them for
// system suspend.  Namely, if the device is expected to wake up the
// system from the sleep state, it may have to be reconfigured for this
// purpose, or if the device is not expected to wake up the system from
// the sleep state, it should be prevented from signaling wakeup events
// going forward.
//
// Also if the driver of the device does not indicate that its system
// suspend callbacks can cope with runtime-suspended devices, it is
// better to resume the device from runtime suspend here.
//
    if (!dev_pm_smart_suspend(dev) || pci_dev_need_resume(pci_dev)) {
    pm_runtime_resume(dev);
    pci_dev.state_saved = false;
    } else {
    pci_dev_adjust_pme(pci_dev);
    }
    if (pm.suspend) {
    let mut prev: pci_power_t = pci_dev.current_state;
    int error;
    error = pm.suspend(dev);
    suspend_report_result(dev, pm.suspend, error);
    if (error)
    return error;
    if (!pci_dev.state_saved && pci_dev.current_state != PCI_D0
    && pci_dev.current_state != PCI_UNKNOWN) {
    pci_WARN_ONCE(pci_dev, pci_dev.current_state != prev,
    "PCI PM: State of device not saved by %pS\n",
    pm.suspend);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_suspend_late(dev: *mut device) -> c_int {
    static int pci_pm_suspend_late(struct device *dev)
    {
    if (dev_pm_skip_suspend(dev))
    return 0;
    pci_fixup_device(pci_fixup_suspend, to_pci_dev(dev));
    return pm_generic_suspend_late(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_suspend_noirq(dev: *mut device) -> c_int {
    static int pci_pm_suspend_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    if (dev_pm_skip_suspend(dev))
    return 0;
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend_late(dev);
    if (!pm) {
    pci_save_state(pci_dev);
    goto set_unknown;
    }
    if (pm.suspend_noirq) {
    let mut prev: pci_power_t = pci_dev.current_state;
    int error;
    error = pm.suspend_noirq(dev);
    suspend_report_result(dev, pm.suspend_noirq, error);
    if (error)
    return error;
    if (!pci_dev.state_saved && pci_dev.current_state != PCI_D0
    && pci_dev.current_state != PCI_UNKNOWN) {
    pci_WARN_ONCE(pci_dev, pci_dev.current_state != prev,
    "PCI PM: State of device not saved by %pS\n",
    pm.suspend_noirq);
    goto Fixup;
    }
    }
    if (!pci_dev.state_saved) {
    pci_save_state(pci_dev);
//
// If the device is a bridge with a child in D0 below it,
// it needs to stay in D0, so check skip_bus_pm to avoid
// putting it into a low-power state in that case.
//
    if (!pci_dev.skip_bus_pm && pci_power_manageable(pci_dev))
    pci_prepare_to_sleep(pci_dev);
    }
    pci_dbg(pci_dev, "PCI PM: Suspend power state: %s\n",
    pci_power_name(pci_dev.current_state));
    if (pci_dev.current_state == PCI_D0) {
    pci_dev.skip_bus_pm = true;
//
// Per PCI PM r1.2, table 6-1, a bridge must be in D0 if any
// downstream device is in D0, so avoid changing the power state
// of the parent bridge by setting the skip_bus_pm flag for it.
//
    if (pci_dev.bus.self)
    pci_dev.bus.self.skip_bus_pm = true;
    }
    if (pci_dev.skip_bus_pm && pm_suspend_no_platform()) {
    pci_dbg(pci_dev, "PCI PM: Skipped\n");
    goto Fixup;
    }
    set_unknown:
    pci_pm_set_unknown_state(pci_dev);
//
// Some BIOSes from ASUS have a bug: If a USB EHCI host controller's
// PCI COMMAND register isn't 0, the BIOS assumes that the controller
// hasn't been quiesced and tries to turn it off.  If the controller
// is already in D3, this can hang or cause memory corruption.
//
// Since the value of the COMMAND register doesn't matter once the
// device has been suspended, we can safely set it to 0 here.
//
    if (pci_dev.class == PCI_CLASS_SERIAL_USB_EHCI)
    pci_write_config_word(pci_dev, PCI_COMMAND, 0);
    Fixup:
    pci_fixup_device(pci_fixup_suspend_late, pci_dev);
//
// If the target system sleep state is suspend-to-idle, it is sufficient
// to check whether or not the device's wakeup settings are good for
// runtime PM.  Otherwise, the pm_resume_via_firmware() check will cause
// pci_pm_complete() to take care of fixing up the device's state
// anyway, if need be.
//
    if (device_can_wakeup(dev) && !device_may_wakeup(dev))
    dev.power.may_skip_resume = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_resume_noirq(dev: *mut device) -> c_int {
    static int pci_pm_resume_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    let mut prev_state: pci_power_t = pci_dev.current_state;
    let mut skip_bus_pm: bool = pci_dev.skip_bus_pm;
    if (dev_pm_skip_resume(dev))
    return 0;
//
// In the suspend-to-idle case, devices left in D0 during suspend will
// stay in D0, so it is not necessary to restore or update their
// configuration here and attempting to put them into D0 again is
// pointless, so avoid doing that.
//
    if (!(skip_bus_pm && pm_suspend_no_platform()))
    pci_pm_default_resume_early(pci_dev);
    pci_fixup_device(pci_fixup_resume_early, pci_dev);
    pcie_pme_root_status_cleanup(pci_dev);
    if (!skip_bus_pm && prev_state == PCI_D3cold)
    pci_pm_bridge_power_up_actions(pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return 0;
    if (pm && pm.resume_noirq)
    return pm.resume_noirq(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_resume_early(dev: *mut device) -> c_int {
    static int pci_pm_resume_early(struct device *dev)
    {
    if (dev_pm_skip_resume(dev))
    return 0;
    return pm_generic_resume_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_resume(dev: *mut device) -> c_int {
    static int pci_pm_resume(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
//
// This is necessary for the suspend error path in which resume is
// called without restoring the standard config registers of the device.
//
    if (pci_dev.state_saved)
    pci_restore_standard_config(pci_dev);
    pci_resume_ptm(pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_resume(dev);
    pci_pm_default_resume(pci_dev);
    if (pm) {
    if (pm.resume)
    return pm.resume(dev);
    } else {
    pci_pm_reenable_device(pci_dev);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pci_pm_freeze(dev: *mut device) -> c_int {
    static int pci_pm_freeze(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend(dev, PMSG_FREEZE);
    if (!pm) {
    pci_pm_default_suspend(pci_dev);
    if (!pm_runtime_suspended(dev))
    pci_dev.state_saved = false;
    return 0;
    }
//
// Resume all runtime-suspended devices before creating a snapshot
// image of system memory, because the restore kernel generally cannot
// be expected to always handle them consistently and they need to be
// put into the runtime-active metastate during system resume anyway,
// so it is better to ensure that the state saved in the image will be
// always consistent with that.
//
    pm_runtime_resume(dev);
    pci_dev.state_saved = false;
    if (pm.freeze) {
    int error;
    error = pm.freeze(dev);
    suspend_report_result(dev, pm.freeze, error);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_freeze_noirq(dev: *mut device) -> c_int {
    static int pci_pm_freeze_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend_late(dev);
    if (pm && pm.freeze_noirq) {
    int error;
    error = pm.freeze_noirq(dev);
    suspend_report_result(dev, pm.freeze_noirq, error);
    if (error)
    return error;
    }
    if (!pci_dev.state_saved)
    pci_save_state(pci_dev);
    pci_pm_set_unknown_state(pci_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_thaw_noirq(dev: *mut device) -> c_int {
    static int pci_pm_thaw_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
//
// The pm->thaw_noirq() callback assumes the device has been
// returned to D0 and its config state has been restored.
//
// In addition, pci_restore_state() restores MSI-X state in MMIO
// space, which requires the device to be in D0, so return it to D0
// in case the driver's "freeze" callbacks put it into a low-power
// state.
//
    pci_pm_power_up_and_verify_state(pci_dev);
    pci_restore_state(pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return 0;
    if (pm && pm.thaw_noirq)
    return pm.thaw_noirq(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_thaw(dev: *mut device) -> c_int {
    static int pci_pm_thaw(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    let mut error: c_int = 0;
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_resume(dev);
    if (pm) {
    if (pm.thaw)
    error = pm.thaw(dev);
    } else {
    pci_pm_reenable_device(pci_dev);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_poweroff(dev: *mut device) -> c_int {
    static int pci_pm_poweroff(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend(dev, PMSG_HIBERNATE);
    if (!pm) {
    pci_pm_default_suspend(pci_dev);
    return 0;
    }
// The reason to do that is the same as in pci_pm_suspend().
    if (!dev_pm_smart_suspend(dev) || pci_dev_need_resume(pci_dev)) {
    pm_runtime_resume(dev);
    pci_dev.state_saved = false;
    } else {
    pci_dev_adjust_pme(pci_dev);
    }
    if (pm.poweroff) {
    int error;
    error = pm.poweroff(dev);
    suspend_report_result(dev, pm.poweroff, error);
    if (error)
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_poweroff_late(dev: *mut device) -> c_int {
    static int pci_pm_poweroff_late(struct device *dev)
    {
    if (dev_pm_skip_suspend(dev))
    return 0;
    pci_fixup_device(pci_fixup_suspend, to_pci_dev(dev));
    return pm_generic_poweroff_late(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_poweroff_noirq(dev: *mut device) -> c_int {
    static int pci_pm_poweroff_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    if (dev_pm_skip_suspend(dev))
    return 0;
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_suspend_late(dev);
    if (!pm) {
    pci_fixup_device(pci_fixup_suspend_late, pci_dev);
    return 0;
    }
    if (pm.poweroff_noirq) {
    int error;
    error = pm.poweroff_noirq(dev);
    suspend_report_result(dev, pm.poweroff_noirq, error);
    if (error)
    return error;
    }
    if (!pci_dev.state_saved && !pci_has_subordinate(pci_dev))
    pci_prepare_to_sleep(pci_dev);
//
// The reason for doing this here is the same as for the analogous code
// in pci_pm_suspend_noirq().
//
    if (pci_dev.class == PCI_CLASS_SERIAL_USB_EHCI)
    pci_write_config_word(pci_dev, PCI_COMMAND, 0);
    pci_fixup_device(pci_fixup_suspend_late, pci_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_restore_noirq(dev: *mut device) -> c_int {
    static int pci_pm_restore_noirq(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    pci_pm_default_resume_early(pci_dev);
    pci_fixup_device(pci_fixup_resume_early, pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return 0;
    if (pm && pm.restore_noirq)
    return pm.restore_noirq(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_restore(dev: *mut device) -> c_int {
    static int pci_pm_restore(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
//
// This is necessary for the hibernation error path in which restore is
// called without restoring the standard config registers of the device.
//
    if (pci_dev.state_saved)
    pci_restore_standard_config(pci_dev);
    if (pci_has_legacy_pm_support(pci_dev))
    return pci_legacy_resume(dev);
    pci_pm_default_resume(pci_dev);
    if (pm) {
    if (pm.restore)
    return pm.restore(dev);
    } else {
    pci_pm_reenable_device(pci_dev);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pci_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int pci_pm_runtime_suspend(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    let mut prev: pci_power_t = pci_dev.current_state;
    int error;
    pci_suspend_ptm(pci_dev);
//
// If pci_dev->driver is not set (unbound), we leave the device in D0,
// but it may go to D3cold when the bridge above it runtime suspends.
// Save its config space in case that happens.
//
    if (!pci_dev.driver) {
    pci_save_state(pci_dev);
    return 0;
    }
    pci_dev.state_saved = false;
    if (pm && pm.runtime_suspend) {
    error = pm.runtime_suspend(dev);
//
// -EBUSY and -EAGAIN is used to request the runtime PM core
// to schedule a new suspend, so log the event only with debug
// log level.
//
    if (error == -EBUSY || error == -EAGAIN) {
    pci_dbg(pci_dev, "can't suspend now (%ps returned %d)\n",
    pm.runtime_suspend, error);
    return error;
    } else if (error) {
    pci_err(pci_dev, "can't suspend (%ps returned %d)\n",
    pm.runtime_suspend, error);
    return error;
    }
    }
    pci_fixup_device(pci_fixup_suspend, pci_dev);
    if (pm && pm.runtime_suspend
    && !pci_dev.state_saved && pci_dev.current_state != PCI_D0
    && pci_dev.current_state != PCI_UNKNOWN) {
    pci_WARN_ONCE(pci_dev, pci_dev.current_state != prev,
    "PCI PM: State of device not saved by %pS\n",
    pm.runtime_suspend);
    return 0;
    }
    if (!pci_dev.state_saved) {
    pci_save_state(pci_dev);
    pci_finish_runtime_suspend(pci_dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_runtime_resume(dev: *mut device) -> c_int {
    static int pci_pm_runtime_resume(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    let mut prev_state: pci_power_t = pci_dev.current_state;
    let mut error: c_int = 0;
//
// Restoring config space is necessary even if the device is not bound
// to a driver because although we left it in D0, it may have gone to
// D3cold when the bridge above it runtime suspended.
//
    pci_pm_default_resume_early(pci_dev);
    pci_resume_ptm(pci_dev);
    if (!pci_dev.driver)
    return 0;
    pci_fixup_device(pci_fixup_resume_early, pci_dev);
    pci_pm_default_resume(pci_dev);
    if (prev_state == PCI_D3cold)
    pci_pm_bridge_power_up_actions(pci_dev);
    if (pm && pm.runtime_resume)
    error = pm.runtime_resume(dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn pci_pm_runtime_idle(dev: *mut device) -> c_int {
    static int pci_pm_runtime_idle(struct device *dev)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
//
// If pci_dev->driver is not set (unbound), the device should
// always remain in D0 regardless of the runtime PM status
//
    if (!pci_dev.driver)
    return 0;
    if (pm && pm.runtime_idle)
    return pm.runtime_idle(dev);
    return 0;
    }
    static const struct dev_pm_ops pci_dev_pm_ops = {
    .prepare = pci_pm_prepare,
    .complete = pci_pm_complete,
    .suspend = pci_pm_suspend,
    .suspend_late = pci_pm_suspend_late,
    .resume = pci_pm_resume,
    .resume_early = pci_pm_resume_early,
    .freeze = pci_pm_freeze,
    .thaw = pci_pm_thaw,
    .poweroff = pci_pm_poweroff,
    .poweroff_late = pci_pm_poweroff_late,
    .restore = pci_pm_restore,
    .suspend_noirq = pci_pm_suspend_noirq,
    .resume_noirq = pci_pm_resume_noirq,
    .freeze_noirq = pci_pm_freeze_noirq,
    .thaw_noirq = pci_pm_thaw_noirq,
    .poweroff_noirq = pci_pm_poweroff_noirq,
    .restore_noirq = pci_pm_restore_noirq,
    .runtime_suspend = pci_pm_runtime_suspend,
    .runtime_resume = pci_pm_runtime_resume,
    .runtime_idle = pci_pm_runtime_idle,
    };

//
// __pci_register_driver - register a new pci driver
// @drv: the driver structure to register
// @owner: owner module of drv
// @mod_name: module name string
//
// Adds the driver structure to the list of registered drivers.
// Returns a negative value on error, otherwise 0.
// If no error occurred, the driver remains registered even if
// no device was claimed during registration.
//
    int __pci_register_driver(struct pci_driver *drv, struct module *owner,
    const char *mod_name)
    {
// initialize common driver fields
    drv.driver.name = drv.name;
    drv.driver.bus = &pci_bus_type;
    drv.driver.owner = owner;
    drv.driver.mod_name = mod_name;
    drv.driver.groups = drv.groups;
    drv.driver.dev_groups = drv.dev_groups;
    spin_lock_init(&drv.dynids.lock);
    INIT_LIST_HEAD(&drv.dynids.list);
// register with core
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL(__pci_register_driver);
//
// pci_unregister_driver - unregister a pci driver
// @drv: the driver structure to unregister
//
// Deletes the driver structure from the list of registered PCI drivers,
// gives it a chance to clean up by calling its remove() function for
// each device it was responsible for, and marks those devices as
// driverless.
//
#[no_mangle]
pub unsafe extern "C" fn pci_unregister_driver(drv: *mut pci_driver) {
    void pci_unregister_driver(struct pci_driver *drv)
    {
    driver_unregister(&drv.driver);
    pci_free_dynids(drv);
    }
    EXPORT_SYMBOL(pci_unregister_driver);
    static struct pci_driver pci_compat_driver = {
    .name = "compat"
    };
//
// pci_dev_driver - get the pci_driver of a device
// @dev: the device to query
//
// Returns the appropriate pci_driver structure or %NULL if there is no
// registered driver for the device.
//
    struct pci_driver *pci_dev_driver(const struct pci_dev *dev)
    {
    int i;
    if (dev.driver)
    return dev.driver;
    for (i = 0; i <= PCI_ROM_RESOURCE; i++)
    if (dev.resource[i].flags & IORESOURCE_BUSY)
    return &pci_compat_driver;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(pci_dev_driver);
//
// pci_bus_match - Tell if a PCI device structure has a matching PCI device id structure
// @dev: the PCI device structure to match against
// @drv: the device driver to search for matching PCI device id structures
//
// Used by a driver to check whether a PCI device present in the
// system is in its list of supported devices. Returns the matching
// pci_device_id structure or %NULL if there is no match.
//
#[no_mangle]
unsafe extern "C" fn pci_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int pci_bus_match(struct device *dev, const struct device_driver *drv)
    {
    struct pci_dev *pci_dev = to_pci_dev(dev);
    struct pci_driver *pci_drv;
    const struct pci_device_id *found_id;
    struct pci_device_id id_copy;
    if (pci_dev_binding_disallowed(pci_dev))
    return 0;
    pci_drv = (struct pci_driver *)to_pci_driver(drv);
    found_id = pci_match_device(pci_drv, pci_dev, &id_copy);
    if (found_id)
    return 1;
    return 0;
    }
//
// pci_dev_get - increments the reference count of the pci device structure
// @dev: the device being referenced
//
// Each live reference to a device should be refcounted.
//
// Drivers for PCI devices should normally record such references in
// their probe() methods, when they bind to a device, and release
// them by calling pci_dev_put(), in their disconnect() methods.
//
// A pointer to the device with the incremented reference counter is returned.
//
    struct pci_dev *pci_dev_get(struct pci_dev *dev)
    {
    if (dev)
    get_device(&dev.dev);
    return dev;
    }
    EXPORT_SYMBOL(pci_dev_get);
//
// pci_dev_put - release a use of the pci device structure
// @dev: device that's been disconnected
//
// Must be called when a user of a device is finished with it.  When the last
// user of the device calls this function, the memory of the device is freed.
//
#[no_mangle]
pub unsafe extern "C" fn pci_dev_put(dev: *mut pci_dev) {
    void pci_dev_put(struct pci_dev *dev)
    {
    if (dev)
    put_device(&dev.dev);
    }
    EXPORT_SYMBOL(pci_dev_put);
#[no_mangle]
unsafe extern "C" fn pci_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int pci_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct pci_dev *pdev;
    if (!dev)
    return -ENODEV;
    pdev = to_pci_dev(dev);
    if (add_uevent_var(env, "PCI_CLASS=%04X", pdev.class))
    return -ENOMEM;
    if (add_uevent_var(env, "PCI_ID=%04X:%04X", pdev.vendor, pdev.device))
    return -ENOMEM;
    if (add_uevent_var(env, "PCI_SUBSYS_ID=%04X:%04X", pdev.subsystem_vendor,
    pdev.subsystem_device))
    return -ENOMEM;
    if (add_uevent_var(env, "PCI_SLOT_NAME=%s", pci_name(pdev)))
    return -ENOMEM;
    if (add_uevent_var(env, "MODALIAS=pci:v%08Xd%08Xsv%08Xsd%08Xbc%02Xsc%02Xi%02X",
    pdev.vendor, pdev.device,
    pdev.subsystem_vendor, pdev.subsystem_device,
    (u8)(pdev.class >> 16), (u8)(pdev.class >> 8),
    (u8)(pdev.class)))
    return -ENOMEM;
    return 0;
    }

//
// pci_uevent_ers - emit a uevent during recovery path of PCI device
// @pdev: PCI device undergoing error recovery
// @err_type: type of error event
//
#[no_mangle]
pub unsafe extern "C" fn pci_uevent_ers(pdev: *mut pci_dev, err_type: enum pci_ers_result) {
    void pci_uevent_ers(struct pci_dev *pdev, enum pci_ers_result err_type)
    {
    let mut idx: c_int = 0;
    char *envp[3];
    switch (err_type) {
    case PCI_ERS_RESULT_NONE:
    case PCI_ERS_RESULT_CAN_RECOVER:
    case PCI_ERS_RESULT_NEED_RESET:
    envp[idx++] = "ERROR_EVENT=BEGIN_RECOVERY";
    envp[idx++] = "DEVICE_ONLINE=0";
    break;
    case PCI_ERS_RESULT_RECOVERED:
    envp[idx++] = "ERROR_EVENT=SUCCESSFUL_RECOVERY";
    envp[idx++] = "DEVICE_ONLINE=1";
    break;
    case PCI_ERS_RESULT_DISCONNECT:
    envp[idx++] = "ERROR_EVENT=FAILED_RECOVERY";
    envp[idx++] = "DEVICE_ONLINE=0";
    break;
    default:
    break;
    }
    if (idx > 0) {
    envp[idx++] = core::ptr::null_mut();
    kobject_uevent_env(&pdev.dev.kobj, KOBJ_CHANGE, envp);
    }
    }

#[no_mangle]
unsafe extern "C" fn pci_bus_num_vf(dev: *mut device) -> c_int {
    static int pci_bus_num_vf(struct device *dev)
    {
    return pci_num_vf(to_pci_dev(dev));
    }
//
// pci_dma_configure - Setup DMA configuration
// @dev: ptr to dev structure
//
// Function to update PCI devices's DMA configuration using the same
// info from the OF node or ACPI node of host bridge's parent (if any).
//
#[no_mangle]
unsafe extern "C" fn pci_dma_configure(dev: *mut device) -> c_int {
    static int pci_dma_configure(struct device *dev)
    {
    const struct device_driver *drv = READ_ONCE(dev.driver);
    struct device *bridge;
    let mut ret: c_int = 0;
    bridge = pci_get_host_bridge_device(to_pci_dev(dev));
    if (IS_ENABLED(CONFIG_OF) && bridge.parent &&
    bridge.parent.of_node) {
    ret = of_dma_configure(dev, bridge.parent.of_node, true);
    } else if (has_acpi_companion(bridge)) {
    struct acpi_device *adev = to_acpi_device_node(bridge.fwnode);
    ret = acpi_dma_configure(dev, acpi_get_dma_attr(adev));
    }
//
// Attempt to enable ACS regardless of capability because some Root
// Ports (e.g. those quirked with *_intel_pch_acs_*) do not have
// the standard ACS capability but still support ACS via those
// quirks.
//
    pci_enable_acs(to_pci_dev(dev));
    pci_put_host_bridge_device(bridge);
// @drv may not be valid when we're called from the IOMMU layer
    if (!ret && drv && !to_pci_driver(drv).driver_managed_dma) {
    ret = iommu_device_use_default_domain(dev);
    if (ret)
    arch_teardown_dma_ops(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci_dma_cleanup(dev: *mut device) {
    static void pci_dma_cleanup(struct device *dev)
    {
    struct pci_driver *driver = to_pci_driver(dev.driver);
    if (!driver.driver_managed_dma)
    iommu_device_unuse_default_domain(dev);
    }
//
// pci_device_irq_get_affinity - get IRQ affinity mask for device
// @dev: ptr to dev structure
// @irq_vec: interrupt vector number
//
// Return the CPU affinity mask for @dev and @irq_vec.
//
    static const struct cpumask *pci_device_irq_get_affinity(struct device *dev,
    unsigned int irq_vec)
    {
    return pci_irq_get_affinity(to_pci_dev(dev), irq_vec);
    }
    const struct bus_type pci_bus_type = {
    .name		= "pci",
    .driver_override = true,
    .match		= pci_bus_match,
    .uevent		= pci_uevent,
    .probe		= pci_device_probe,
    .remove		= pci_device_remove,
    .shutdown	= pci_device_shutdown,
    .irq_get_affinity = pci_device_irq_get_affinity,
    .dev_groups	= pci_dev_groups,
    .bus_groups	= pci_bus_groups,
    .drv_groups	= pci_drv_groups,
    .pm		= PCI_PM_OPS_PTR,
    .num_vf		= pci_bus_num_vf,
    .dma_configure	= pci_dma_configure,
    .dma_cleanup	= pci_dma_cleanup,
    };
    EXPORT_SYMBOL(pci_bus_type);
#[no_mangle]
unsafe extern "C" fn pci_driver_init() -> int __init {
    static int __init pci_driver_init(void)
    {
    int ret;
    pci_probe_wq = alloc_workqueue("sync_wq", WQ_PERCPU, 0);
    if (!pci_probe_wq)
    return -ENOMEM;
    ret = bus_register(&pci_bus_type);
    if (ret)
    return ret;

    ret = bus_register(&pcie_port_bus_type);
    if (ret)
    return ret;

    dma_debug_add_bus(&pci_bus_type);
    return 0;
    }
    postcore_initcall(pci_driver_init);
