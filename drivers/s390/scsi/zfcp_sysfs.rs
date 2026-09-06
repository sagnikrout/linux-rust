//! Automatically rewritten from C to Rust
//! Source: drivers/s390/scsi/zfcp_sysfs.c
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
// zfcp device driver
//
// sysfs attributes.
//
// Copyright IBM Corp. 2008, 2026
//

    struct device_attribute dev_attr_##_feat##_##_name = __ATTR(_name, _mode,\
    _show, _store)

    static ssize_t zfcp_sysfs_##_feat##_##_name##_show(struct device *dev,	       \
    struct device_attribute *at,\
    char *buf)		       \
    {									       \
    struct _feat_def *_feat = container_of(dev, struct _feat_def, dev);    \
    \
    return sysfs_emit(buf, _format, _value);			       \
    }									       \
    static ZFCP_DEV_ATTR(_feat, _name, S_IRUGO,				       \
    zfcp_sysfs_##_feat##_##_name##_show, core::ptr::null_mut());

    static ssize_t zfcp_sysfs_##_feat##_##_name##_show(struct device *dev,	       \
    struct device_attribute *at,\
    char *buf)		       \
    {									       \
    return sysfs_emit(buf, _format, _value);			       \
    }									       \
    static ZFCP_DEV_ATTR(_feat, _name, S_IRUGO,				       \
    zfcp_sysfs_##_feat##_##_name##_show, core::ptr::null_mut());

    static ssize_t zfcp_sysfs_adapter_##_name##_show(struct device *dev,	     \
    struct device_attribute *at,\
    char *buf)		     \
    {									     \
    struct ccw_device *cdev = to_ccwdev(dev);			     \
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);	     \
    int i;								     \
    \
    if (!adapter)							     \
    return -ENODEV;						     \
    \
    i = sysfs_emit(buf, _format, _value);				     \
    zfcp_ccw_adapter_put(adapter);					     \
    return i;							     \
    }									     \
    static ZFCP_DEV_ATTR(adapter, _name, S_IRUGO,				     \
    zfcp_sysfs_adapter_##_name##_show, core::ptr::null_mut());
    ZFCP_DEFINE_A_ATTR(status, "0x%08x\n", atomic_read(&adapter.status));
    ZFCP_DEFINE_A_ATTR(peer_wwnn, "0x%016llx\n",
    (unsigned long long) adapter.peer_wwnn);
    ZFCP_DEFINE_A_ATTR(peer_wwpn, "0x%016llx\n",
    (unsigned long long) adapter.peer_wwpn);
    ZFCP_DEFINE_A_ATTR(peer_d_id, "0x%06x\n", adapter.peer_d_id);
    ZFCP_DEFINE_A_ATTR(card_version, "0x%04x\n", adapter.hydra_version);
    ZFCP_DEFINE_A_ATTR(lic_version, "0x%08x\n", adapter.fsf_lic_version);
    ZFCP_DEFINE_A_ATTR(hardware_version, "0x%08x\n", adapter.hardware_version);
    ZFCP_DEFINE_A_ATTR(in_recovery, "%d\n", (atomic_read(&adapter.status) &
    ZFCP_STATUS_COMMON_ERP_INUSE) != 0);
    ZFCP_DEFINE_ATTR(zfcp_port, port, status, "0x%08x\n",
    atomic_read(&port.status));
    ZFCP_DEFINE_ATTR(zfcp_port, port, in_recovery, "%d\n",
    (atomic_read(&port.status) &
    ZFCP_STATUS_COMMON_ERP_INUSE) != 0);
    ZFCP_DEFINE_ATTR_CONST(port, access_denied, "%d\n", 0);
    ZFCP_DEFINE_ATTR(zfcp_unit, unit, status, "0x%08x\n",
    zfcp_unit_sdev_status(unit));
    ZFCP_DEFINE_ATTR(zfcp_unit, unit, in_recovery, "%d\n",
    (zfcp_unit_sdev_status(unit) &
    ZFCP_STATUS_COMMON_ERP_INUSE) != 0);
    ZFCP_DEFINE_ATTR(zfcp_unit, unit, access_denied, "%d\n",
    (zfcp_unit_sdev_status(unit) &
    ZFCP_STATUS_COMMON_ACCESS_DENIED) != 0);
    ZFCP_DEFINE_ATTR_CONST(unit, access_shared, "%d\n", 0);
    ZFCP_DEFINE_ATTR_CONST(unit, access_readonly, "%d\n", 0);
    static ssize_t zfcp_sysfs_port_failed_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct zfcp_port *port = container_of(dev, struct zfcp_port, dev);
    if (atomic_read(&port.status) & ZFCP_STATUS_COMMON_ERP_FAILED)
    return sysfs_emit(buf, "1\n");
    return sysfs_emit(buf, "0\n");
    }
    static ssize_t zfcp_sysfs_port_failed_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zfcp_port *port = container_of(dev, struct zfcp_port, dev);
    unsigned long val;
    if (kstrtoul(buf, 0, &val) || val != 0)
    return -EINVAL;
    zfcp_erp_set_port_status(port, ZFCP_STATUS_COMMON_RUNNING);
    zfcp_erp_port_reopen(port, ZFCP_STATUS_COMMON_ERP_FAILED, "sypfai2");
    zfcp_erp_wait(port.adapter);
    return count;
    }
    static ZFCP_DEV_ATTR(port, failed, S_IWUSR | S_IRUGO,
    zfcp_sysfs_port_failed_show,
    zfcp_sysfs_port_failed_store);
    static ssize_t zfcp_sysfs_unit_failed_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct zfcp_unit *unit = container_of(dev, struct zfcp_unit, dev);
    struct scsi_device *sdev;
    unsigned int status, failed = 1;
    sdev = zfcp_unit_sdev(unit);
    if (sdev) {
    status = atomic_read(&sdev_to_zfcp(sdev).status);
    failed = status & ZFCP_STATUS_COMMON_ERP_FAILED ? 1 : 0;
    scsi_device_put(sdev);
    }
    return sysfs_emit(buf, "%d\n", failed);
    }
    static ssize_t zfcp_sysfs_unit_failed_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zfcp_unit *unit = container_of(dev, struct zfcp_unit, dev);
    unsigned long val;
    struct scsi_device *sdev;
    if (kstrtoul(buf, 0, &val) || val != 0)
    return -EINVAL;
    sdev = zfcp_unit_sdev(unit);
    if (sdev) {
    zfcp_erp_set_lun_status(sdev, ZFCP_STATUS_COMMON_RUNNING);
    zfcp_erp_lun_reopen(sdev, ZFCP_STATUS_COMMON_ERP_FAILED,
    "syufai2");
    zfcp_erp_wait(unit.port.adapter);
    } else
    zfcp_unit_scsi_scan(unit);
    return count;
    }
    static ZFCP_DEV_ATTR(unit, failed, S_IWUSR | S_IRUGO,
    zfcp_sysfs_unit_failed_show,
    zfcp_sysfs_unit_failed_store);
    static ssize_t zfcp_sysfs_adapter_failed_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ccw_device *cdev = to_ccwdev(dev);
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);
    int i;
    if (!adapter)
    return -ENODEV;
    if (atomic_read(&adapter.status) & ZFCP_STATUS_COMMON_ERP_FAILED)
    i = sysfs_emit(buf, "1\n");
    else
    i = sysfs_emit(buf, "0\n");
    zfcp_ccw_adapter_put(adapter);
    return i;
    }
    static ssize_t zfcp_sysfs_adapter_failed_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ccw_device *cdev = to_ccwdev(dev);
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);
    unsigned long val;
    let mut retval: c_int = 0;
    if (!adapter)
    return -ENODEV;
    if (kstrtoul(buf, 0, &val) || val != 0) {
    retval = -EINVAL;
    goto out;
    }
    zfcp_erp_adapter_reset_sync(adapter, "syafai2");
    out:
    zfcp_ccw_adapter_put(adapter);
    return retval ? retval : (ssize_t) count;
    }
    static ZFCP_DEV_ATTR(adapter, failed, S_IWUSR | S_IRUGO,
    zfcp_sysfs_adapter_failed_show,
    zfcp_sysfs_adapter_failed_store);
    static ssize_t zfcp_sysfs_port_rescan_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ccw_device *cdev = to_ccwdev(dev);
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);
    let mut retval: c_int = 0;
    if (!adapter)
    return -ENODEV;
//
// If `scsi_host` is missing, we can't schedule `scan_work`, as it
// makes use of the corresponding fc_host object. But this state is
// only possible if xconfig/xport data has never completed yet,
// and we couldn't successfully scan for ports anyway.
//
    if (adapter.scsi_host == core::ptr::null_mut()) {
    retval = -ENODEV;
    goto out;
    }
//
// Users wish is our command: immediately schedule and flush a
// worker to conduct a synchronous port scan, that is, neither
// a random delay nor a rate limit is applied here.
//
    queue_delayed_work(adapter.work_queue, &adapter.scan_work, 0);
    flush_delayed_work(&adapter.scan_work);
    out:
    zfcp_ccw_adapter_put(adapter);
    return retval ? retval : (ssize_t) count;
    }
    static ZFCP_DEV_ATTR(adapter, port_rescan, S_IWUSR, core::ptr::null_mut(),
    zfcp_sysfs_port_rescan_store);
    DEFINE_MUTEX(zfcp_sysfs_port_units_mutex);
#[no_mangle]
unsafe extern "C" fn zfcp_sysfs_port_set_removing(port: *const *const zfcp_port) {
    static void zfcp_sysfs_port_set_removing(struct zfcp_port *const port)
    {
    lockdep_assert_held(&zfcp_sysfs_port_units_mutex);
    atomic_set(&port.units, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn zfcp_sysfs_port_is_removing(port: *const *const zfcp_port) -> bool {
    bool zfcp_sysfs_port_is_removing(const struct zfcp_port *const port)
    {
    lockdep_assert_held(&zfcp_sysfs_port_units_mutex);
    return atomic_read(&port.units) == -1;
    }
#[no_mangle]
unsafe extern "C" fn zfcp_sysfs_port_in_use(port: *const *const zfcp_port) -> bool {
    static bool zfcp_sysfs_port_in_use(struct zfcp_port *const port)
    {
    let mut adapter: *mut zfcp_adapter const = port.adapter;
    unsigned long flags;
    struct scsi_device *sdev;
    let mut in_use: bool = true;
    mutex_lock(&zfcp_sysfs_port_units_mutex);
    if (atomic_read(&port.units) > 0)
    goto unlock_port_units_mutex; /* zfcp_unit(s) under port */
    spin_lock_irqsave(adapter.scsi_host.host_lock, flags);
    __shost_for_each_device(sdev, adapter.scsi_host) {
    const struct zfcp_scsi_dev *zsdev = sdev_to_zfcp(sdev);
    if (sdev.sdev_state == SDEV_DEL ||
    sdev.sdev_state == SDEV_CANCEL)
    continue;
    if (zsdev.port != port)
    continue;
// alive scsi_device under port of interest
    goto unlock_host_lock;
    }
// port is about to be removed, so no more unit_add or sdev_init
    zfcp_sysfs_port_set_removing(port);
    in_use = false;
    unlock_host_lock:
    spin_unlock_irqrestore(adapter.scsi_host.host_lock, flags);
    unlock_port_units_mutex:
    mutex_unlock(&zfcp_sysfs_port_units_mutex);
    return in_use;
    }
    static ssize_t zfcp_sysfs_port_remove_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ccw_device *cdev = to_ccwdev(dev);
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);
    struct zfcp_port *port;
    u64 wwpn;
    let mut retval: c_int = -EINVAL;
    if (!adapter)
    return -ENODEV;
    if (kstrtoull(buf, 0, (unsigned long long *) &wwpn))
    goto out;
    port = zfcp_get_port_by_wwpn(adapter, wwpn);
    if (!port)
    goto out;
    else
    retval = 0;
    if (zfcp_sysfs_port_in_use(port)) {
    retval = -EBUSY;
    put_device(&port.dev); /* undo zfcp_get_port_by_wwpn() */
    goto out;
    }
    write_lock_irq(&adapter.port_list_lock);
    list_del(&port.list);
    write_unlock_irq(&adapter.port_list_lock);
    zfcp_erp_port_shutdown(port, 0, "syprs_1");
    device_unregister(&port.dev);
    put_device(&port.dev); /* undo zfcp_get_port_by_wwpn() */
    out:
    zfcp_ccw_adapter_put(adapter);
    return retval ? retval : (ssize_t) count;
    }
    static ZFCP_DEV_ATTR(adapter, port_remove, S_IWUSR, core::ptr::null_mut(),
    zfcp_sysfs_port_remove_store);
    static ssize_t
    zfcp_sysfs_adapter_diag_max_age_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(to_ccwdev(dev));
    ssize_t rc;
    if (!adapter)
    return -ENODEV;
    rc = sysfs_emit(buf, "%lu\n", adapter.diagnostics.max_age);
    zfcp_ccw_adapter_put(adapter);
    return rc;
    }
    static ssize_t
    zfcp_sysfs_adapter_diag_max_age_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(to_ccwdev(dev));
    unsigned long max_age;
    ssize_t rc;
    if (!adapter)
    return -ENODEV;
    rc = kstrtoul(buf, 10, &max_age);
    if (rc != 0)
    goto out;
    adapter.diagnostics.max_age = max_age;
    rc = count;
    out:
    zfcp_ccw_adapter_put(adapter);
    return rc;
    }
    static ZFCP_DEV_ATTR(adapter, diag_max_age, 0644,
    zfcp_sysfs_adapter_diag_max_age_show,
    zfcp_sysfs_adapter_diag_max_age_store);
    static ssize_t zfcp_sysfs_adapter_fc_security_show(
    struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct ccw_device *cdev = to_ccwdev(dev);
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(cdev);
    unsigned int status;
    int i;
    if (!adapter)
    return -ENODEV;
//
// Adapter status COMMON_OPEN implies xconf data and xport data
// was done. Adapter FC Endpoint Security capability remains
// unchanged in case of COMMON_ERP_FAILED (e.g. due to local link
// down).
//
    status = atomic_read(&adapter.status);
    if (0 == (status & ZFCP_STATUS_COMMON_OPEN))
    i = sysfs_emit(buf, "unknown\n");
#[no_mangle]
pub unsafe extern "C" fn if(FSF_FEATURE_FC_SECURITY): !(adapter->adapter_features &) -> else {
    else if (!(adapter.adapter_features & FSF_FEATURE_FC_SECURITY))
    i = sysfs_emit(buf, "unsupported\n");
    else {
    i = zfcp_fsf_scnprint_fc_security(
    buf, PAGE_SIZE - 1, adapter.fc_security_algorithms,
    ZFCP_FSF_PRINT_FMT_LIST);
    i += sysfs_emit_at(buf, i, "\n");
    }
    zfcp_ccw_adapter_put(adapter);
    return i;
    }
    static ZFCP_DEV_ATTR(adapter, fc_security, S_IRUGO,
    zfcp_sysfs_adapter_fc_security_show,
    core::ptr::null_mut());
    static struct attribute *zfcp_adapter_attrs[] = {
    &dev_attr_adapter_failed.attr,
    &dev_attr_adapter_in_recovery.attr,
    &dev_attr_adapter_port_remove.attr,
    &dev_attr_adapter_port_rescan.attr,
    &dev_attr_adapter_peer_wwnn.attr,
    &dev_attr_adapter_peer_wwpn.attr,
    &dev_attr_adapter_peer_d_id.attr,
    &dev_attr_adapter_card_version.attr,
    &dev_attr_adapter_lic_version.attr,
    &dev_attr_adapter_status.attr,
    &dev_attr_adapter_hardware_version.attr,
    &dev_attr_adapter_diag_max_age.attr,
    &dev_attr_adapter_fc_security.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group zfcp_sysfs_adapter_attr_group = {
    .attrs = zfcp_adapter_attrs,
    };
    static ssize_t zfcp_sysfs_unit_add_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zfcp_port *port = container_of(dev, struct zfcp_port, dev);
    struct zfcp_adapter *adapter = port.adapter;
    let mut fcp_lun: u64 = 0;
    let mut retval: c_int = -EINVAL;
    if (kstrtoull(buf, 0, (unsigned long long *)&fcp_lun)) {
    zfcp_dbf_hba_uas("syuast1", 3, adapter, port.wwpn,
    fcp_lun, retval);
    return retval;
    }
    flush_work(&port.rport_work);
    retval = zfcp_unit_add(port, fcp_lun);
    if (retval) {
    zfcp_dbf_hba_uas("syuast2", 3, adapter, port.wwpn,
    fcp_lun, retval);
    return retval;
    }
    return count;
    }
    static DEVICE_ATTR(unit_add, S_IWUSR, core::ptr::null_mut(), zfcp_sysfs_unit_add_store);
    static ssize_t zfcp_sysfs_unit_remove_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct zfcp_port *port = container_of(dev, struct zfcp_port, dev);
    u64 fcp_lun;
    if (kstrtoull(buf, 0, (unsigned long long *) &fcp_lun))
    return -EINVAL;
    if (zfcp_unit_remove(port, fcp_lun))
    return -EINVAL;
    return count;
    }
    static DEVICE_ATTR(unit_remove, S_IWUSR, core::ptr::null_mut(), zfcp_sysfs_unit_remove_store);
    static ssize_t zfcp_sysfs_port_fc_security_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct zfcp_port *port = container_of(dev, struct zfcp_port, dev);
    struct zfcp_adapter *adapter = port.adapter;
    let mut status: c_uint = atomic_read(&port.status);
    int i;
    if (0 == (status & ZFCP_STATUS_COMMON_OPEN) ||
    0 == (status & ZFCP_STATUS_COMMON_UNBLOCKED) ||
    0 == (status & ZFCP_STATUS_PORT_PHYS_OPEN) ||
    0 != (status & ZFCP_STATUS_PORT_LINK_TEST) ||
    0 != (status & ZFCP_STATUS_COMMON_ERP_FAILED) ||
    0 != (status & ZFCP_STATUS_COMMON_ACCESS_BOXED))
    i = sysfs_emit(buf, "unknown\n");
#[no_mangle]
pub unsafe extern "C" fn if(FSF_FEATURE_FC_SECURITY): !(adapter->adapter_features &) -> else {
    else if (!(adapter.adapter_features & FSF_FEATURE_FC_SECURITY))
    i = sysfs_emit(buf, "unsupported\n");
    else {
    i = zfcp_fsf_scnprint_fc_security(
    buf, PAGE_SIZE - 1, port.connection_info,
    ZFCP_FSF_PRINT_FMT_SINGLEITEM);
    i += sysfs_emit_at(buf, i, "\n");
    }
    return i;
    }
    static ZFCP_DEV_ATTR(port, fc_security, S_IRUGO,
    zfcp_sysfs_port_fc_security_show,
    core::ptr::null_mut());
    static struct attribute *zfcp_port_attrs[] = {
    &dev_attr_unit_add.attr,
    &dev_attr_unit_remove.attr,
    &dev_attr_port_failed.attr,
    &dev_attr_port_in_recovery.attr,
    &dev_attr_port_status.attr,
    &dev_attr_port_access_denied.attr,
    &dev_attr_port_fc_security.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group zfcp_port_attr_group = {
    .attrs = zfcp_port_attrs,
    };
    const struct attribute_group *zfcp_port_attr_groups[] = {
    &zfcp_port_attr_group,
    core::ptr::null_mut(),
    };
    static struct attribute *zfcp_unit_attrs[] = {
    &dev_attr_unit_failed.attr,
    &dev_attr_unit_in_recovery.attr,
    &dev_attr_unit_status.attr,
    &dev_attr_unit_access_denied.attr,
    &dev_attr_unit_access_shared.attr,
    &dev_attr_unit_access_readonly.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group zfcp_unit_attr_group = {
    .attrs = zfcp_unit_attrs,
    };
    const struct attribute_group *zfcp_unit_attr_groups[] = {
    &zfcp_unit_attr_group,
    core::ptr::null_mut(),
    };

    static ssize_t								\
    zfcp_sysfs_unit_##_name##_latency_show(struct device *dev,		\
    struct device_attribute *attr,	\
    char *buf) {			\
    struct scsi_device *sdev = to_scsi_device(dev);			\
    struct zfcp_scsi_dev *zfcp_sdev = sdev_to_zfcp(sdev);		\
    struct zfcp_latencies *lat = &zfcp_sdev.latencies;		\
    struct zfcp_adapter *adapter = zfcp_sdev.port.adapter;	\
    unsigned long long fsum, fmin, fmax, csum, cmin, cmax, cc;	\
    \
    spin_lock_bh(&lat.lock);					\
    fsum = lat._name.fabric.sum * adapter.timer_ticks;		\
    fmin = lat._name.fabric.min * adapter.timer_ticks;		\
    fmax = lat._name.fabric.max * adapter.timer_ticks;		\
    csum = lat._name.channel.sum * adapter.timer_ticks;		\
    cmin = lat._name.channel.min * adapter.timer_ticks;		\
    cmax = lat._name.channel.max * adapter.timer_ticks;		\
    cc  = lat._name.counter;					\
    spin_unlock_bh(&lat.lock);					\
    \
    do_div(fsum, 1000);						\
    do_div(fmin, 1000);						\
    do_div(fmax, 1000);						\
    do_div(csum, 1000);						\
    do_div(cmin, 1000);						\
    do_div(cmax, 1000);						\
    \
    return sysfs_emit(buf, "%llu %llu %llu %llu %llu %llu %llu\n",	\
    fmin, fmax, fsum, cmin, cmax, csum, cc);	\
    }									\
    static ssize_t								\
    zfcp_sysfs_unit_##_name##_latency_store(struct device *dev,		\
    struct device_attribute *attr,	\
    const char *buf, size_t count)	\
    {									\
    struct scsi_device *sdev = to_scsi_device(dev);			\
    struct zfcp_scsi_dev *zfcp_sdev = sdev_to_zfcp(sdev);		\
    struct zfcp_latencies *lat = &zfcp_sdev.latencies;		\
    unsigned long flags;						\
    \
    spin_lock_irqsave(&lat.lock, flags);				\
    lat._name.fabric.sum = 0;					\
    lat._name.fabric.min = 0xFFFFFFFF;				\
    lat._name.fabric.max = 0;					\
    lat._name.channel.sum = 0;					\
    lat._name.channel.min = 0xFFFFFFFF;				\
    lat._name.channel.max = 0;					\
    lat._name.counter = 0;						\
    spin_unlock_irqrestore(&lat.lock, flags);			\
    \
    return (ssize_t) count;						\
    }									\
    static DEVICE_ATTR(_name##_latency, S_IWUSR | S_IRUGO,			\
    zfcp_sysfs_unit_##_name##_latency_show,		\
    zfcp_sysfs_unit_##_name##_latency_store);
    ZFCP_DEFINE_LATENCY_ATTR(read);
    ZFCP_DEFINE_LATENCY_ATTR(write);
    ZFCP_DEFINE_LATENCY_ATTR(cmd);

    static ssize_t zfcp_sysfs_scsi_##_name##_show(struct device *dev,	\
    struct device_attribute *attr,\
    char *buf)                 \
    {                                                                        \
    struct scsi_device *sdev = to_scsi_device(dev);			 \
    struct zfcp_scsi_dev *zfcp_sdev = sdev_to_zfcp(sdev);		 \
    \
    return sysfs_emit(buf, _format, _value);			 \
    }									 \
    static DEVICE_ATTR(_name, S_IRUGO, zfcp_sysfs_scsi_##_name##_show, core::ptr::null_mut());
    ZFCP_DEFINE_SCSI_ATTR(hba_id, "%s\n",
    dev_name(&zfcp_sdev.port.adapter.ccw_device.dev));
    ZFCP_DEFINE_SCSI_ATTR(wwpn, "0x%016llx\n",
    (unsigned long long) zfcp_sdev.port.wwpn);
    static ssize_t zfcp_sysfs_scsi_fcp_lun_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct scsi_device *sdev = to_scsi_device(dev);
    return sysfs_emit(buf, "0x%016llx\n", zfcp_scsi_dev_lun(sdev));
    }
    static DEVICE_ATTR(fcp_lun, S_IRUGO, zfcp_sysfs_scsi_fcp_lun_show, core::ptr::null_mut());
    ZFCP_DEFINE_SCSI_ATTR(zfcp_access_denied, "%d\n",
    (atomic_read(&zfcp_sdev.status) &
    ZFCP_STATUS_COMMON_ACCESS_DENIED) != 0);
    static ssize_t zfcp_sysfs_scsi_zfcp_failed_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct scsi_device *sdev = to_scsi_device(dev);
    let mut status: c_uint = atomic_read(&sdev_to_zfcp(sdev).status);
    let mut failed: c_uint = status & ZFCP_STATUS_COMMON_ERP_FAILED ? 1 : 0;
    return sysfs_emit(buf, "%d\n", failed);
    }
    static ssize_t zfcp_sysfs_scsi_zfcp_failed_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct scsi_device *sdev = to_scsi_device(dev);
    unsigned long val;
    if (kstrtoul(buf, 0, &val) || val != 0)
    return -EINVAL;
    zfcp_erp_set_lun_status(sdev, ZFCP_STATUS_COMMON_RUNNING);
    zfcp_erp_lun_reopen(sdev, ZFCP_STATUS_COMMON_ERP_FAILED,
    "syufai3");
    zfcp_erp_wait(sdev_to_zfcp(sdev).port.adapter);
    return count;
    }
    static DEVICE_ATTR(zfcp_failed, S_IWUSR | S_IRUGO,
    zfcp_sysfs_scsi_zfcp_failed_show,
    zfcp_sysfs_scsi_zfcp_failed_store);
    ZFCP_DEFINE_SCSI_ATTR(zfcp_in_recovery, "%d\n",
    (atomic_read(&zfcp_sdev.status) &
    ZFCP_STATUS_COMMON_ERP_INUSE) != 0);
    ZFCP_DEFINE_SCSI_ATTR(zfcp_status, "0x%08x\n",
    atomic_read(&zfcp_sdev.status));
    static struct attribute *zfcp_sdev_attrs[] = {
    &dev_attr_fcp_lun.attr,
    &dev_attr_wwpn.attr,
    &dev_attr_hba_id.attr,
    &dev_attr_read_latency.attr,
    &dev_attr_write_latency.attr,
    &dev_attr_cmd_latency.attr,
    &dev_attr_zfcp_access_denied.attr,
    &dev_attr_zfcp_failed.attr,
    &dev_attr_zfcp_in_recovery.attr,
    &dev_attr_zfcp_status.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group zfcp_sysfs_sdev_attr_group = {
    .attrs = zfcp_sdev_attrs
    };
    const struct attribute_group *zfcp_sysfs_sdev_attr_groups[] = {
    &zfcp_sysfs_sdev_attr_group,
    core::ptr::null_mut()
    };
    static ssize_t zfcp_sysfs_adapter_util_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct Scsi_Host *scsi_host = dev_to_shost(dev);
    struct fsf_qtcb_bottom_port *qtcb_port;
    struct zfcp_adapter *adapter;
    int retval;
    adapter = (struct zfcp_adapter *) scsi_host.hostdata[0];
    if (!(adapter.adapter_features & FSF_FEATURE_MEASUREMENT_DATA))
    return -EOPNOTSUPP;
    qtcb_port = kzalloc_obj(struct fsf_qtcb_bottom_port);
    if (!qtcb_port)
    return -ENOMEM;
    retval = zfcp_fsf_exchange_port_data_sync(adapter.qdio, qtcb_port);
    if (retval == 0 || retval == -EAGAIN)
    retval = sysfs_emit(buf, "%u %u %u\n", qtcb_port.cp_util,
    qtcb_port.cb_util, qtcb_port.a_util);
    kfree(qtcb_port);
    return retval;
    }
    static DEVICE_ATTR(utilization, S_IRUGO, zfcp_sysfs_adapter_util_show, core::ptr::null_mut());
    static int zfcp_sysfs_adapter_ex_config(struct device *dev,
    struct fsf_statistics_info *stat_inf)
    {
    struct Scsi_Host *scsi_host = dev_to_shost(dev);
    struct fsf_qtcb_bottom_config *qtcb_config;
    struct zfcp_adapter *adapter;
    int retval;
    adapter = (struct zfcp_adapter *) scsi_host.hostdata[0];
    if (!(adapter.adapter_features & FSF_FEATURE_MEASUREMENT_DATA))
    return -EOPNOTSUPP;
    qtcb_config = kzalloc_obj(struct fsf_qtcb_bottom_config);
    if (!qtcb_config)
    return -ENOMEM;
    retval = zfcp_fsf_exchange_config_data_sync(adapter.qdio, qtcb_config);
    if (retval == 0 || retval == -EAGAIN)
// stat_inf = qtcb_config->stat_info;
    kfree(qtcb_config);
    return retval;
    }

    static ssize_t zfcp_sysfs_adapter_##_name##_show(struct device *dev,	\
    struct device_attribute *attr,\
    char *buf)		\
    {									\
    struct fsf_statistics_info stat_info;				\
    int retval;							\
    \
    retval = zfcp_sysfs_adapter_ex_config(dev, &stat_info);		\
    if (retval)							\
    return retval;						\
    \
    return sysfs_emit(buf, _format, ## _arg);			\
    }									\
    static DEVICE_ATTR(_name, S_IRUGO, zfcp_sysfs_adapter_##_name##_show, core::ptr::null_mut());
    ZFCP_SHOST_ATTR(requests, "%llu %llu %llu\n",
    (unsigned long long) stat_info.input_req,
    (unsigned long long) stat_info.output_req,
    (unsigned long long) stat_info.control_req);
    ZFCP_SHOST_ATTR(megabytes, "%llu %llu\n",
    (unsigned long long) stat_info.input_mb,
    (unsigned long long) stat_info.output_mb);
    ZFCP_SHOST_ATTR(seconds_active, "%llu\n",
    (unsigned long long) stat_info.seconds_act);
    static ssize_t zfcp_sysfs_adapter_q_full_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct Scsi_Host *scsi_host = class_to_shost(dev);
    struct zfcp_qdio *qdio =
    ((struct zfcp_adapter *) scsi_host.hostdata[0]).qdio;
    u64 util;
    spin_lock_bh(&qdio.stat_lock);
    util = qdio.req_q_util;
    spin_unlock_bh(&qdio.stat_lock);
    return sysfs_emit(buf, "%d %llu\n", atomic_read(&qdio.req_q_full),
    (unsigned long long)util);
    }
    static DEVICE_ATTR(queue_full, S_IRUGO, zfcp_sysfs_adapter_q_full_show, core::ptr::null_mut());
    static struct attribute *zfcp_sysfs_shost_attrs[] = {
    &dev_attr_utilization.attr,
    &dev_attr_requests.attr,
    &dev_attr_megabytes.attr,
    &dev_attr_seconds_active.attr,
    &dev_attr_queue_full.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group zfcp_sysfs_shost_attr_group = {
    .attrs = zfcp_sysfs_shost_attrs
    };
    const struct attribute_group *zfcp_sysfs_shost_attr_groups[] = {
    &zfcp_sysfs_shost_attr_group,
    core::ptr::null_mut()
    };
    static ssize_t zfcp_sysfs_adapter_diag_b2b_credit_show(
    struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct zfcp_adapter *adapter = zfcp_ccw_adapter_by_cdev(to_ccwdev(dev));
    struct zfcp_diag_header *diag_hdr;
    struct fc_els_flogi *nsp;
    let mut rc: isize = -ENOLINK;
    unsigned long flags;
    unsigned int status;
    if (!adapter)
    return -ENODEV;
    status = atomic_read(&adapter.status);
    if (0 == (status & ZFCP_STATUS_COMMON_OPEN) ||
    0 == (status & ZFCP_STATUS_COMMON_UNBLOCKED) ||
    0 != (status & ZFCP_STATUS_COMMON_ERP_FAILED))
    goto out;
    diag_hdr = &adapter.diagnostics.config_data.header;
    rc = zfcp_diag_update_buffer_limited(
    adapter, diag_hdr, zfcp_diag_update_config_data_buffer);
    if (rc != 0)
    goto out;
    spin_lock_irqsave(&diag_hdr.access_lock, flags);
// nport_serv_param doesn't contain the ELS_Command code
    nsp = (struct fc_els_flogi *)((unsigned long)
    adapter.diagnostics.config_data
    .data.nport_serv_param -
    sizeof(u32));
    rc = sysfs_emit(buf, "%hu\n", be16_to_cpu(nsp.fl_csp.sp_bb_cred));
    spin_unlock_irqrestore(&diag_hdr.access_lock, flags);
    out:
    zfcp_ccw_adapter_put(adapter);
    return rc;
    }
    static ZFCP_DEV_ATTR(adapter_diag, b2b_credit, 0400,
    zfcp_sysfs_adapter_diag_b2b_credit_show, core::ptr::null_mut());

    static ssize_t zfcp_sysfs_adapter_diag_sfp_##_name##_show(	       \
    struct device *dev, struct device_attribute *attr, char *buf)  \
    {								       \
    struct zfcp_adapter *const adapter =			       \
    zfcp_ccw_adapter_by_cdev(to_ccwdev(dev));	       \
    struct zfcp_diag_header *diag_hdr;			       \
    ssize_t rc = -ENOLINK;					       \
    unsigned long flags;					       \
    unsigned int status;					       \
    \
    if (!adapter)						       \
    return -ENODEV;					       \
    \
    status = atomic_read(&adapter.status);			       \
    if (0 == (status & ZFCP_STATUS_COMMON_OPEN) ||		       \
    0 == (status & ZFCP_STATUS_COMMON_UNBLOCKED) ||	       \
    0 != (status & ZFCP_STATUS_COMMON_ERP_FAILED))	       \
    goto out;					       \
    \
    if (!zfcp_diag_support_sfp(adapter)) {			       \
    rc = -EOPNOTSUPP;				       \
    goto out;					       \
    }							       \
    \
    diag_hdr = &adapter.diagnostics.port_data.header;	       \
    \
    rc = zfcp_diag_update_buffer_limited(			       \
    adapter, diag_hdr, zfcp_diag_update_port_data_buffer); \
    if (rc != 0)						       \
    goto out;					       \
    \
    spin_lock_irqsave(&diag_hdr.access_lock, flags);	       \
    rc = sysfs_emit(					       \
    buf, _prtfmt "\n",				       \
    adapter.diagnostics.port_data.data._qtcb_member);    \
    spin_unlock_irqrestore(&diag_hdr.access_lock, flags);	       \
    \
    out:								       \
    zfcp_ccw_adapter_put(adapter);				       \
    return rc;						       \
    }								       \
    static ZFCP_DEV_ATTR(adapter_diag_sfp, _name, 0400,		       \
    zfcp_sysfs_adapter_diag_sfp_##_name##_show, core::ptr::null_mut())
    ZFCP_DEFINE_DIAG_SFP_ATTR(temperature, temperature, "%hd");
    ZFCP_DEFINE_DIAG_SFP_ATTR(vcc, vcc, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(tx_bias, tx_bias, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(tx_power, tx_power, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(rx_power, rx_power, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(port_tx_type, sfp_flags.port_tx_type, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(optical_port, sfp_flags.optical_port, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(sfp_invalid, sfp_flags.sfp_invalid, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(connector_type, sfp_flags.connector_type, "%hu");
    ZFCP_DEFINE_DIAG_SFP_ATTR(fec_active, sfp_flags.fec_active, "%hu");
    static struct attribute *zfcp_sysfs_diag_attrs[] = {
    &dev_attr_adapter_diag_sfp_temperature.attr,
    &dev_attr_adapter_diag_sfp_vcc.attr,
    &dev_attr_adapter_diag_sfp_tx_bias.attr,
    &dev_attr_adapter_diag_sfp_tx_power.attr,
    &dev_attr_adapter_diag_sfp_rx_power.attr,
    &dev_attr_adapter_diag_sfp_port_tx_type.attr,
    &dev_attr_adapter_diag_sfp_optical_port.attr,
    &dev_attr_adapter_diag_sfp_sfp_invalid.attr,
    &dev_attr_adapter_diag_sfp_connector_type.attr,
    &dev_attr_adapter_diag_sfp_fec_active.attr,
    &dev_attr_adapter_diag_b2b_credit.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group zfcp_sysfs_diag_attr_group = {
    .name = "diagnostics",
    .attrs = zfcp_sysfs_diag_attrs,
    };
    const struct attribute_group *zfcp_sysfs_adapter_attr_groups[] = {
    &zfcp_sysfs_adapter_attr_group,
    &zfcp_sysfs_diag_attr_group,
    core::ptr::null_mut(),
    };
