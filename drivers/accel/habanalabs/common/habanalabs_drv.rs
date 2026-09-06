//! Automatically rewritten from C to Rust
//! Source: drivers/accel/habanalabs/common/habanalabs_drv.c
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
// Copyright 2016-2021 HabanaLabs, Ltd.
// All Rights Reserved.
//

// Macro flag: #define CREATE_TRACE_POINTS

    MODULE_AUTHOR(HL_DRIVER_AUTHOR);
    MODULE_DESCRIPTION(HL_DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
    static int hl_major;
    static DEFINE_IDR(hl_devs_idr);
    static DEFINE_MUTEX(hl_devs_idr_lock);

    let mut timeout_locked: static int = HL_DEFAULT_TIMEOUT_LOCKED;
    let mut reset_on_lockup: static int = 1;
    static int memory_scrub;
    let mut boot_error_status_mask: static ulong = ULONG_MAX;
    module_param(timeout_locked, int, 0444);
    MODULE_PARM_DESC(timeout_locked,
    "Device lockup timeout in seconds (0 = disabled, default 30s)");
    module_param(reset_on_lockup, int, 0444);
    MODULE_PARM_DESC(reset_on_lockup,
    "Do device reset on lockup (0 = no, 1 = yes, default yes)");
    module_param(memory_scrub, int, 0444);
    MODULE_PARM_DESC(memory_scrub,
    "Scrub device memory in various states (0 = no, 1 = yes, default no)");
    module_param(boot_error_status_mask, ulong, 0444);
    MODULE_PARM_DESC(boot_error_status_mask,
    "Mask of the error status during device CPU boot (If bitX is cleared then error X is masked. Default all 1's)");
pub const PCI_IDS_GOYA: c_uint = 0x0001;
pub const PCI_IDS_GAUDI: c_uint = 0x1000;
pub const PCI_IDS_GAUDI_SEC: c_uint = 0x1010;
pub const PCI_IDS_GAUDI2: c_uint = 0x1020;
    static const struct pci_device_id ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_HABANALABS, PCI_IDS_GOYA), },
    { PCI_DEVICE(PCI_VENDOR_ID_HABANALABS, PCI_IDS_GAUDI), },
    { PCI_DEVICE(PCI_VENDOR_ID_HABANALABS, PCI_IDS_GAUDI_SEC), },
    { PCI_DEVICE(PCI_VENDOR_ID_HABANALABS, PCI_IDS_GAUDI2), },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, ids);
    static const struct drm_ioctl_desc hl_drm_ioctls[] = {
    DRM_IOCTL_DEF_DRV(HL_INFO, hl_info_ioctl, 0),
    DRM_IOCTL_DEF_DRV(HL_CB, hl_cb_ioctl, 0),
    DRM_IOCTL_DEF_DRV(HL_CS, hl_cs_ioctl, 0),
    DRM_IOCTL_DEF_DRV(HL_WAIT_CS, hl_wait_ioctl, 0),
    DRM_IOCTL_DEF_DRV(HL_MEMORY, hl_mem_ioctl, 0),
    DRM_IOCTL_DEF_DRV(HL_DEBUG, hl_debug_ioctl, 0),
    };
    static const struct file_operations hl_fops = {
    .owner = THIS_MODULE,
    .open = accel_open,
    .release = drm_release,
    .unlocked_ioctl = drm_ioctl,
    .compat_ioctl = drm_compat_ioctl,
    .llseek = noop_llseek,
    .mmap = hl_mmap
    };
    static const struct drm_driver hl_driver = {
    .driver_features = DRIVER_COMPUTE_ACCEL,
    .name = HL_NAME,
    .desc = HL_DRIVER_DESC,
    .major = LINUX_VERSION_MAJOR,
    .minor = LINUX_VERSION_PATCHLEVEL,
    .patchlevel = LINUX_VERSION_SUBLEVEL,
    .fops = &hl_fops,
    .open = hl_device_open,
    .postclose = hl_device_release,
    .ioctls = hl_drm_ioctls,
    .num_ioctls = ARRAY_SIZE(hl_drm_ioctls)
    };
//
// get_asic_type - translate device id to asic type
//
// @hdev: pointer to habanalabs device structure.
//
// Translate device id and revision id to asic type.
// In case of unidentified device, return -1
//
#[no_mangle]
unsafe extern "C" fn get_asic_type(hdev: *mut hl_device) -> enum hl_asic_type {
    static enum hl_asic_type get_asic_type(struct hl_device *hdev)
    {
    struct pci_dev *pdev = hdev.pdev;
    let mut asic_type: enum hl_asic_type = ASIC_INVALID;
    switch (pdev.device) {
    case PCI_IDS_GOYA:
    asic_type = ASIC_GOYA;
    break;
    case PCI_IDS_GAUDI:
    asic_type = ASIC_GAUDI;
    break;
    case PCI_IDS_GAUDI_SEC:
    asic_type = ASIC_GAUDI_SEC;
    break;
    case PCI_IDS_GAUDI2:
    switch (pdev.revision) {
    case REV_ID_A:
    asic_type = ASIC_GAUDI2;
    break;
    case REV_ID_B:
    asic_type = ASIC_GAUDI2B;
    break;
    case REV_ID_C:
    asic_type = ASIC_GAUDI2C;
    break;
    case REV_ID_D:
    asic_type = ASIC_GAUDI2D;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return asic_type;
    }
#[no_mangle]
unsafe extern "C" fn is_asic_secured(asic_type: enum hl_asic_type) -> bool {
    static bool is_asic_secured(enum hl_asic_type asic_type)
    {
    switch (asic_type) {
    case ASIC_GAUDI_SEC:
    return true;
    default:
    return false;
    }
    }
//
// hl_device_open() - open function for habanalabs device.
// @ddev: pointer to DRM device structure.
// @file: pointer to DRM file private data structure.
//
// Called when process opens an habanalabs device.
//
#[no_mangle]
pub unsafe extern "C" fn hl_device_open(ddev: *mut drm_device, file_priv: *mut drm_file) -> c_int {
    int hl_device_open(struct drm_device *ddev, struct drm_file *file_priv)
    {
    struct hl_device *hdev = to_hl_device(ddev);
    enum hl_device_status status;
    struct hl_fpriv *hpriv;
    int rc;
    hpriv = kzalloc_obj(*hpriv);
    if (!hpriv)
    return -ENOMEM;
    hpriv.hdev = hdev;
    mutex_init(&hpriv.notifier_event.lock);
    mutex_init(&hpriv.restore_phase_mutex);
    mutex_init(&hpriv.ctx_lock);
    kref_init(&hpriv.refcount);
    hl_ctx_mgr_init(&hpriv.ctx_mgr);
    hl_mem_mgr_init(hpriv.hdev.dev, &hpriv.mem_mgr);
    hpriv.taskpid = get_task_pid(current, PIDTYPE_PID);
    mutex_lock(&hdev.fpriv_list_lock);
    if (!hl_device_operational(hdev, &status)) {
    dev_dbg_ratelimited(hdev.dev,
    "Can't open %s because it is %s\n",
    dev_name(hdev.dev), hdev.status[status]);
    if (status == HL_DEVICE_STATUS_IN_RESET ||
    status == HL_DEVICE_STATUS_IN_RESET_AFTER_DEVICE_RELEASE)
    rc = -EAGAIN;
    else
    rc = -EPERM;
    goto out_err;
    }
    if (hdev.is_in_dram_scrub) {
    dev_dbg_ratelimited(hdev.dev,
    "Can't open %s during dram scrub\n",
    dev_name(hdev.dev));
    rc = -EAGAIN;
    goto out_err;
    }
    if (hdev.compute_ctx_in_release) {
    dev_dbg_ratelimited(hdev.dev,
    "Can't open %s because another user is still releasing it\n",
    dev_name(hdev.dev));
    rc = -EAGAIN;
    goto out_err;
    }
    if (hdev.is_compute_ctx_active) {
    dev_dbg_ratelimited(hdev.dev,
    "Can't open %s because another user is working on it\n",
    dev_name(hdev.dev));
    rc = -EBUSY;
    goto out_err;
    }
    rc = hl_ctx_create(hdev, hpriv);
    if (rc) {
    dev_err(hdev.dev, "Failed to create context %d\n", rc);
    goto out_err;
    }
    list_add(&hpriv.dev_node, &hdev.fpriv_list);
    mutex_unlock(&hdev.fpriv_list_lock);
    hdev.asic_funcs.send_device_activity(hdev, true);
    hl_debugfs_add_file(hpriv);
    hl_enable_err_info_capture(&hdev.captured_err_info);
    hdev.open_counter++;
    hdev.last_successful_open_jif = jiffies;
    hdev.last_successful_open_ktime = ktime_get();
    file_priv.driver_priv = hpriv;
    hpriv.file_priv = file_priv;
    return 0;
    out_err:
    mutex_unlock(&hdev.fpriv_list_lock);
    hl_mem_mgr_fini(&hpriv.mem_mgr, core::ptr::null_mut());
    hl_mem_mgr_idr_destroy(&hpriv.mem_mgr);
    hl_ctx_mgr_fini(hpriv.hdev, &hpriv.ctx_mgr);
    mutex_destroy(&hpriv.ctx_lock);
    mutex_destroy(&hpriv.restore_phase_mutex);
    mutex_destroy(&hpriv.notifier_event.lock);
    put_pid(hpriv.taskpid);
    kfree(hpriv);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn hl_device_open_ctrl(inode: *mut inode, filp: *mut file) -> c_int {
    int hl_device_open_ctrl(struct inode *inode, struct file *filp)
    {
    struct hl_device *hdev;
    struct hl_fpriv *hpriv;
    int rc;
    mutex_lock(&hl_devs_idr_lock);
    hdev = idr_find(&hl_devs_idr, iminor(inode));
    mutex_unlock(&hl_devs_idr_lock);
    if (!hdev) {
    pr_err("Couldn't find device %d:%d\n",
    imajor(inode), iminor(inode));
    return -ENXIO;
    }
    hpriv = kzalloc_obj(*hpriv);
    if (!hpriv)
    return -ENOMEM;
// Prevent other routines from reading partial hpriv data by
// initializing hpriv fields before inserting it to the list
//
    hpriv.hdev = hdev;
    filp.private_data = hpriv;
    nonseekable_open(inode, filp);
    hpriv.taskpid = get_task_pid(current, PIDTYPE_PID);
    mutex_lock(&hdev.fpriv_ctrl_list_lock);
    if (!hl_ctrl_device_operational(hdev, core::ptr::null_mut())) {
    dev_dbg_ratelimited(hdev.dev_ctrl,
    "Can't open %s because it is disabled\n",
    dev_name(hdev.dev_ctrl));
    rc = -EPERM;
    goto out_err;
    }
    list_add(&hpriv.dev_node, &hdev.fpriv_ctrl_list);
    mutex_unlock(&hdev.fpriv_ctrl_list_lock);
    return 0;
    out_err:
    mutex_unlock(&hdev.fpriv_ctrl_list_lock);
    filp.private_data = core::ptr::null_mut();
    put_pid(hpriv.taskpid);
    kfree(hpriv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn set_driver_behavior_per_device(hdev: *mut hl_device) {
    static void set_driver_behavior_per_device(struct hl_device *hdev)
    {
    hdev.nic_ports_mask = 0;
    hdev.fw_components = FW_TYPE_ALL_TYPES;
    hdev.cpu_queues_enable = 1;
    hdev.pldm = 0;
    hdev.hard_reset_on_fw_events = 1;
    hdev.bmc_enable = 1;
    hdev.reset_on_preboot_fail = 1;
    hdev.heartbeat = 1;
    }
#[no_mangle]
unsafe extern "C" fn copy_kernel_module_params_to_device(hdev: *mut hl_device) {
    static void copy_kernel_module_params_to_device(struct hl_device *hdev)
    {
    hdev.asic_prop.fw_security_enabled = is_asic_secured(hdev.asic_type);
    hdev.major = hl_major;
    hdev.memory_scrub = memory_scrub;
    hdev.reset_on_lockup = reset_on_lockup;
    hdev.boot_error_status_mask = boot_error_status_mask;
    }
#[no_mangle]
unsafe extern "C" fn fixup_device_params_per_asic(hdev: *mut hl_device, timeout: c_int) {
    static void fixup_device_params_per_asic(struct hl_device *hdev, int timeout)
    {
    switch (hdev.asic_type) {
    case ASIC_GAUDI:
    case ASIC_GAUDI_SEC:
// If user didn't request a different timeout than the default one, we have
// a different default timeout for Gaudi
//
    if (timeout == HL_DEFAULT_TIMEOUT_LOCKED)
    hdev.timeout_jiffies = secs_to_jiffies(GAUDI_DEFAULT_TIMEOUT_LOCKED);
    hdev.reset_upon_device_release = 0;
    break;
    case ASIC_GOYA:
    hdev.reset_upon_device_release = 0;
    break;
    default:
    hdev.reset_upon_device_release = 1;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn fixup_device_params(hdev: *mut hl_device) -> c_int {
    static int fixup_device_params(struct hl_device *hdev)
    {
    int tmp_timeout;
    tmp_timeout = timeout_locked;
    hdev.fw_poll_interval_usec = HL_FW_STATUS_POLL_INTERVAL_USEC;
    hdev.fw_comms_poll_interval_usec = HL_FW_STATUS_POLL_INTERVAL_USEC;
    if (tmp_timeout)
    hdev.timeout_jiffies = secs_to_jiffies(tmp_timeout);
    else
    hdev.timeout_jiffies = MAX_SCHEDULE_TIMEOUT;
    hdev.stop_on_err = true;
    hdev.reset_info.curr_reset_cause = HL_RESET_CAUSE_UNKNOWN;
    hdev.reset_info.prev_reset_trigger = HL_RESET_TRIGGER_DEFAULT;
// Enable only after the initialization of the device
    hdev.disabled = true;
    if (!(hdev.fw_components & FW_TYPE_PREBOOT_CPU) &&
    (hdev.fw_components & ~FW_TYPE_PREBOOT_CPU)) {
    pr_err("Preboot must be set along with other components");
    return -EINVAL;
    }
// If CPU queues not enabled, no way to do heartbeat
    if (!hdev.cpu_queues_enable)
    hdev.heartbeat = 0;
    fixup_device_params_per_asic(hdev, tmp_timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn allocate_device_id(hdev: *mut hl_device) -> c_int {
    static int allocate_device_id(struct hl_device *hdev)
    {
    int id;
    mutex_lock(&hl_devs_idr_lock);
    id = idr_alloc(&hl_devs_idr, hdev, 0, HL_MAX_MINORS, GFP_KERNEL);
    mutex_unlock(&hl_devs_idr_lock);
    if (id < 0) {
    if (id == -ENOSPC)
    pr_err("too many devices in the system\n");
    return -EBUSY;
    }
    hdev.id = id;
//
// Firstly initialized with the internal device ID.
// Will be updated later after the DRM device registration to hold the minor ID.
//
    hdev.cdev_idx = hdev.id;
    return 0;
    }
//
// create_hdev - create habanalabs device instance
//
// @dev: will hold the pointer to the new habanalabs device structure
// @pdev: pointer to the pci device
//
// Allocate memory for habanalabs device and initialize basic fields
// Identify the ASIC type
// Allocate ID (minor) for the device (only for real devices)
//
#[no_mangle]
unsafe extern "C" fn create_hdev(dev: *mut hl_device, pdev: *mut pci_dev) -> c_int {
    static int create_hdev(struct hl_device **dev, struct pci_dev *pdev)
    {
    struct hl_device *hdev;
    int rc;
// dev = NULL;
    hdev = devm_drm_dev_alloc(&pdev.dev, &hl_driver, struct hl_device, drm);
    if (IS_ERR(hdev))
    return PTR_ERR(hdev);
    hdev.dev = hdev.drm.dev;
// Will be NULL in case of simulator device
    hdev.pdev = pdev;
// Assign status description string
    strscpy(hdev.status[HL_DEVICE_STATUS_OPERATIONAL], "operational", HL_STR_MAX);
    strscpy(hdev.status[HL_DEVICE_STATUS_IN_RESET], "in reset", HL_STR_MAX);
    strscpy(hdev.status[HL_DEVICE_STATUS_MALFUNCTION], "disabled", HL_STR_MAX);
    strscpy(hdev.status[HL_DEVICE_STATUS_NEEDS_RESET], "needs reset", HL_STR_MAX);
    strscpy(hdev.status[HL_DEVICE_STATUS_IN_DEVICE_CREATION],
    "in device creation", HL_STR_MAX);
    strscpy(hdev.status[HL_DEVICE_STATUS_IN_RESET_AFTER_DEVICE_RELEASE],
    "in reset after device release", HL_STR_MAX);
// First, we must find out which ASIC are we handling. This is needed
// to configure the behavior of the driver (kernel parameters)
//
    hdev.asic_type = get_asic_type(hdev);
    if (hdev.asic_type == ASIC_INVALID) {
    dev_err(&pdev.dev, "Unsupported ASIC\n");
    rc = -ENODEV;
    goto out_err;
    }
    copy_kernel_module_params_to_device(hdev);
    set_driver_behavior_per_device(hdev);
    fixup_device_params(hdev);
    rc = allocate_device_id(hdev);
    if (rc)
    goto out_err;
// dev = hdev;
    return 0;
    out_err:
    return rc;
    }
//
// destroy_hdev - destroy habanalabs device instance
//
// @dev: pointer to the habanalabs device structure
//
#[no_mangle]
unsafe extern "C" fn destroy_hdev(hdev: *mut hl_device) {
    static void destroy_hdev(struct hl_device *hdev)
    {
// Remove device from the device list
    mutex_lock(&hl_devs_idr_lock);
    idr_remove(&hl_devs_idr, hdev.id);
    mutex_unlock(&hl_devs_idr_lock);
    }
#[no_mangle]
unsafe extern "C" fn hl_pmops_suspend(dev: *mut device) -> c_int {
    static int hl_pmops_suspend(struct device *dev)
    {
    struct hl_device *hdev = dev_get_drvdata(dev);
    pr_debug("Going to suspend PCI device\n");
    if (!hdev) {
    pr_err("device pointer is core::ptr::null_mut() in suspend\n");
    return 0;
    }
    return hl_device_suspend(hdev);
    }
#[no_mangle]
unsafe extern "C" fn hl_pmops_resume(dev: *mut device) -> c_int {
    static int hl_pmops_resume(struct device *dev)
    {
    struct hl_device *hdev = dev_get_drvdata(dev);
    pr_debug("Going to resume PCI device\n");
    if (!hdev) {
    pr_err("device pointer is core::ptr::null_mut() in resume\n");
    return 0;
    }
    return hl_device_resume(hdev);
    }
//
// hl_pci_probe - probe PCI habanalabs devices
//
// @pdev: pointer to pci device
// @id: pointer to pci device id structure
//
// Standard PCI probe function for habanalabs device.
// Create a new habanalabs device and initialize it according to the
// device's type
//
#[no_mangle]
unsafe extern "C" fn hl_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int hl_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct hl_device *hdev;
    int rc;
    dev_info(&pdev.dev, HL_NAME
    " device found [%04x:%04x] (rev %x)\n",
    (int)pdev.vendor, (int)pdev.device, (int)pdev.revision);
    rc = create_hdev(&hdev, pdev);
    if (rc)
    return rc;
    pci_set_drvdata(pdev, hdev);
    rc = hl_device_init(hdev);
    if (rc) {
    dev_err(&pdev.dev, "Fatal error during habanalabs device init\n");
    rc = -ENODEV;
    goto disable_device;
    }
    return 0;
    disable_device:
    pci_set_drvdata(pdev, core::ptr::null_mut());
    destroy_hdev(hdev);
    return rc;
    }
//
// hl_pci_remove - remove PCI habanalabs devices
//
// @pdev: pointer to pci device
//
// Standard PCI remove function for habanalabs device
//
#[no_mangle]
unsafe extern "C" fn hl_pci_remove(pdev: *mut pci_dev) {
    static void hl_pci_remove(struct pci_dev *pdev)
    {
    struct hl_device *hdev;
    hdev = pci_get_drvdata(pdev);
    if (!hdev)
    return;
    hl_device_fini(hdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    destroy_hdev(hdev);
    }
//
// hl_pci_err_detected - a PCI bus error detected on this device
//
// @pdev: pointer to pci device
// @state: PCI error type
//
// Called by the PCI subsystem whenever a non-correctable
// PCI bus error is detected
//
    static pci_ers_result_t
    hl_pci_err_detected(struct pci_dev *pdev, pci_channel_state_t state)
    {
    struct hl_device *hdev = pci_get_drvdata(pdev);
    enum pci_ers_result result;
    switch (state) {
    case pci_channel_io_normal:
    dev_warn(hdev.dev, "PCI normal state error detected\n");
    return PCI_ERS_RESULT_CAN_RECOVER;
    case pci_channel_io_frozen:
    dev_warn(hdev.dev, "PCI frozen state error detected\n");
    result = PCI_ERS_RESULT_NEED_RESET;
    break;
    case pci_channel_io_perm_failure:
    dev_warn(hdev.dev, "PCI failure state error detected\n");
    result = PCI_ERS_RESULT_DISCONNECT;
    break;
    default:
    result = PCI_ERS_RESULT_NONE;
    }
    hdev.asic_funcs.halt_engines(hdev, true, false);
    return result;
    }
//
// hl_pci_err_resume - resume after a PCI slot reset
//
// @pdev: pointer to pci device
//
#[no_mangle]
unsafe extern "C" fn hl_pci_err_resume(pdev: *mut pci_dev) {
    static void hl_pci_err_resume(struct pci_dev *pdev)
    {
    struct hl_device *hdev = pci_get_drvdata(pdev);
    dev_warn(hdev.dev, "Resuming device after PCI slot reset\n");
    hl_device_resume(hdev);
    }
//
// hl_pci_err_slot_reset - a PCI slot reset has just happened
//
// @pdev: pointer to pci device
//
// Determine if the driver can recover from the PCI slot reset
//
#[no_mangle]
unsafe extern "C" fn hl_pci_err_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t hl_pci_err_slot_reset(struct pci_dev *pdev)
    {
    struct hl_device *hdev = pci_get_drvdata(pdev);
    dev_warn(hdev.dev, "PCI slot reset detected\n");
    return PCI_ERS_RESULT_RECOVERED;
    }
#[no_mangle]
unsafe extern "C" fn hl_pci_reset_prepare(pdev: *mut pci_dev) {
    static void hl_pci_reset_prepare(struct pci_dev *pdev)
    {
    struct hl_device *hdev;
    hdev = pci_get_drvdata(pdev);
    if (!hdev)
    return;
    hdev.disabled = true;
    }
#[no_mangle]
unsafe extern "C" fn hl_pci_reset_done(pdev: *mut pci_dev) {
    static void hl_pci_reset_done(struct pci_dev *pdev)
    {
    struct hl_device *hdev;
    u32 flags;
    hdev = pci_get_drvdata(pdev);
    if (!hdev)
    return;
//
// Schedule a thread to trigger hard reset.
// The reason for this handler, is for rare cases where the driver is up
// and FLR occurs. This is valid only when working with no VM, so FW handles FLR
// and resets the device. FW will go back preboot stage, so driver needs to perform
// hard reset in order to load FW fit again.
//
    flags = HL_DRV_RESET_HARD | HL_DRV_RESET_BYPASS_REQ_TO_FW;
    hl_device_reset(hdev, flags);
    }
    static const struct dev_pm_ops hl_pm_ops = {
    .suspend = hl_pmops_suspend,
    .resume = hl_pmops_resume,
    };
    static const struct pci_error_handlers hl_pci_err_handler = {
    .error_detected = hl_pci_err_detected,
    .slot_reset = hl_pci_err_slot_reset,
    .resume = hl_pci_err_resume,
    .reset_prepare = hl_pci_reset_prepare,
    .reset_done = hl_pci_reset_done,
    };
    static struct pci_driver hl_pci_driver = {
    .name = HL_NAME,
    .id_table = ids,
    .probe = hl_pci_probe,
    .remove = hl_pci_remove,
    .shutdown = hl_pci_remove,
    .driver = {
    .name = HL_NAME,
    .pm = &hl_pm_ops,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .err_handler = &hl_pci_err_handler,
    };
//
// hl_init - Initialize the habanalabs kernel driver
//
#[no_mangle]
unsafe extern "C" fn hl_init() -> int __init {
    static int __init hl_init(void)
    {
    int rc;
    dev_t dev;
    pr_info("loading driver\n");
    rc = alloc_chrdev_region(&dev, 0, HL_MAX_MINORS, HL_NAME);
    if (rc < 0) {
    pr_err("unable to get major\n");
    return rc;
    }
    hl_major = MAJOR(dev);
    rc = pci_register_driver(&hl_pci_driver);
    if (rc) {
    pr_err("failed to register pci device\n");
    goto remove_major;
    }
    pr_debug("driver loaded\n");
    return 0;
    remove_major:
    unregister_chrdev_region(MKDEV(hl_major, 0), HL_MAX_MINORS);
    return rc;
    }
//
// hl_exit - Release all resources of the habanalabs kernel driver
//
#[no_mangle]
unsafe extern "C" fn hl_exit() -> void __exit {
    static void __exit hl_exit(void)
    {
    pci_unregister_driver(&hl_pci_driver);
    unregister_chrdev_region(MKDEV(hl_major, 0), HL_MAX_MINORS);
    idr_destroy(&hl_devs_idr);
    pr_debug("driver removed\n");
    }
    module_init(hl_init);
    module_exit(hl_exit);
