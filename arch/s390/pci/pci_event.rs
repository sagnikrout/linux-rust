//! Automatically rewritten from C to Rust
//! Source: arch/s390/pci/pci_event.c
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
// Copyright IBM Corp. 2012
//
// Author(s):
// Jan Glauber <jang@linux.vnet.ibm.com>
//

// Content Code Description for PCI Function Availability
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_ccdf_avail {
    pub reserved1: u32,
    pub /: *mut *mut u32 fh; / function handle,
    pub /: *mut *mut u32 fid; / function id,
    pub reserved2: u32,
    pub reserved3: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u16,
    pub /: *mut *mut u16 pec; / PCI event code,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn ers_result_indicates_abort(ers_res: pci_ers_result_t) -> bool {
    static inline bool ers_result_indicates_abort(pci_ers_result_t ers_res)
    {
    switch (ers_res) {
    case PCI_ERS_RESULT_CAN_RECOVER:
    case PCI_ERS_RESULT_RECOVERED:
    case PCI_ERS_RESULT_NEED_RESET:
    case PCI_ERS_RESULT_NONE:
    pub false: return,
    default:
    pub true: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn is_driver_supported(driver: *mut pci_driver) -> bool {
    static bool is_driver_supported(struct pci_driver *driver)
    {
    if (!driver || !driver.err_handler)
    pub false: return,
    if (!driver.err_handler.error_detected)
    pub false: return,
    pub true: return,
    }
    static int zpci_store_pci_error(struct pci_dev *pdev,
    struct zpci_ccdf_err *ccdf)
    {
    pub to_zpci(pdev): *mut *mut zpci_dev zdev =,
    pub i: c_int,
    if (!zdev.pending_errs.mediated_recovery)
    pub -EINVAL: return,
    if (zdev.pending_errs.count >= ZPCI_ERR_PENDING_MAX) {
    dev_warn_ratelimited(&pdev.dev,
    "%s: Maximum number (%d) of pending error events queued\n",
    pci_name(pdev),
    pub -ENOMEM: return,
    }
    pub ZPCI_ERR_PENDING_MAX: i = zdev->pending_errs.tail %,
    pub zpci_ccdf_err)): memcpy(&zdev->pending_errs.err[i], ccdf, sizeof(struct,
    pub 0: return,
    }
    int zpci_get_pending_error(struct zpci_dev *zdev,
    struct zpci_ccdf_err *ccdf)
    {
    pub head: c_int,
    if (!zdev.pending_errs.count)
    pub -ENOMSG: return,
    pub ZPCI_ERR_PENDING_MAX: head = zdev->pending_errs.head %,
    memcpy(ccdf, &zdev.pending_errs.err[head],
    pub zpci_ccdf_err)): sizeof(struct,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_start_mediated_recovery(zdev: *mut zpci_dev) {
    void zpci_start_mediated_recovery(struct zpci_dev *zdev)
    {
    pub true: zdev->pending_errs.mediated_recovery =,
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_stop_mediated_recovery(zdev: *mut zpci_dev) {
    void zpci_stop_mediated_recovery(struct zpci_dev *zdev)
    {
    pub false: zdev->pending_errs.mediated_recovery =,
    if (zdev.pending_errs.count)
    pr_info("Unhandled PCI error events count=%d for PCI function 0x%x\n",
    pub zdev->fid): zdev->pending_errs.count,,
    pub zpci_ccdf_pending)): memset(&zdev->pending_errs, 0, sizeof(struct,
    }
    static pci_ers_result_t zpci_event_notify_error_detected(struct pci_dev *pdev,
    struct pci_driver *driver)
    {
    pub PCI_ERS_RESULT_DISCONNECT: pci_ers_result_t ers_res =,
    pub pdev->error_state): ers_res = driver->err_handler->error_detected(pdev,,
    pub ers_res): pci_uevent_ers(pdev,,
    if (ers_result_indicates_abort(ers_res))
    pub pci_name(pdev)): pr_info("%s: Automatic recovery failed after initial reporting\n",,
#[no_mangle]
pub unsafe extern "C" fn if(PCI_ERS_RESULT_NEED_RESET: ers_res ==) -> else {
    else if (ers_res == PCI_ERS_RESULT_NEED_RESET)
    pub pci_name(pdev)): pr_debug("%s: Driver needs reset to recover\n",,
    pub ers_res: return,
    }
    static pci_ers_result_t zpci_event_do_error_state_clear(struct pci_dev *pdev,
    struct pci_driver *driver)
    {
    pub PCI_ERS_RESULT_DISCONNECT: pci_ers_result_t ers_res =,
    pub to_zpci(pdev): *mut *mut zpci_dev zdev =,
    pub rc: c_int,
// The underlying device may have been disabled by the event
    if (!zdev_enabled(zdev))
    pub PCI_ERS_RESULT_NEED_RESET: return,
    pub pci_name(pdev)): pr_info("%s: Unblocking device access for examination\n",,
    pub zpci_reset_load_store_blocked(zdev): rc =,
    if (rc) {
    pub pci_name(pdev)): pr_err("%s: Unblocking device access failed\n",,
// Let's try a full reset instead
    pub PCI_ERS_RESULT_NEED_RESET: return,
    }
    if (driver.err_handler.mmio_enabled)
    pub driver->err_handler->mmio_enabled(pdev): ers_res =,
    else
    pub PCI_ERS_RESULT_NONE: ers_res =,
    if (ers_result_indicates_abort(ers_res)) {
    pr_info("%s: Automatic recovery failed after MMIO re-enable\n",
    pub ers_res: return,
    } else if (ers_res == PCI_ERS_RESULT_NEED_RESET) {
    pub pci_name(pdev)): pr_debug("%s: Driver needs reset to recover\n",,
    pub ers_res: return,
    }
    pub pci_name(pdev)): pr_debug("%s: Unblocking DMA\n",,
    pub zpci_clear_error_state(zdev): rc =,
    if (!rc) {
    pub pci_channel_io_normal: pdev->error_state =,
    } else {
    pub pci_name(pdev)): pr_err("%s: Unblocking DMA failed\n",,
// Let's try a full reset instead
    pub PCI_ERS_RESULT_NEED_RESET: return,
    }
    pub ers_res: return,
    }
    static pci_ers_result_t zpci_event_do_reset(struct pci_dev *pdev,
    struct pci_driver *driver)
    {
    pub PCI_ERS_RESULT_DISCONNECT: pci_ers_result_t ers_res =,
    pub pci_name(pdev)): pr_info("%s: Initiating reset\n",,
    if (zpci_hot_reset_device(to_zpci(pdev))) {
    pub pci_name(pdev)): pr_err("%s: The reset request failed\n",,
    pub ers_res: return,
    }
    pub pci_channel_io_normal: pdev->error_state =,
    if (driver.err_handler.slot_reset)
    pub driver->err_handler->slot_reset(pdev): ers_res =,
    else
    pub PCI_ERS_RESULT_NONE: ers_res =,
    if (ers_result_indicates_abort(ers_res)) {
    pub pci_name(pdev)): pr_info("%s: Automatic recovery failed after slot reset\n",,
    pub ers_res: return,
    }
    pub ers_res: return,
    }
// zpci_event_attempt_error_recovery - Try to recover the given PCI function
// @pdev: PCI function to recover currently in the error state
//
// We follow the scheme outlined in Documentation/PCI/pci-error-recovery.rst.
// With the simplification that recovery always happens per function
// and the platform determines which functions are affected for
// multi-function devices.
//
    static pci_ers_result_t zpci_event_attempt_error_recovery(struct pci_dev *pdev,
    struct zpci_ccdf_err *ccdf)
    {
    pub PCI_ERS_RESULT_DISCONNECT: pci_ers_result_t ers_res =,
    pub to_zpci(pdev): *mut *mut zpci_dev zdev =,
    pub false: bool mediated_recovery =,
    pub "success": *mut *mut char status_str =,
    pub driver: *mut pci_driver,
    pub rc: c_int,
//
// Ensure that the PCI function is not removed concurrently, no driver
// is unbound or probed and that userspace can't access its
// configuration space while we perform recovery.
//
    if (pdev.error_state == pci_channel_io_perm_failure) {
    pub PCI_ERS_RESULT_DISCONNECT: ers_res =,
    pub out_unlock: goto,
    }
    pub pci_channel_io_frozen: pdev->error_state =,
    pub to_pci_driver(pdev->dev.driver): driver =,
    if (!is_driver_supported(driver)) {
    if (!driver) {
    pr_info("%s: Cannot be recovered because no driver is bound to the device\n",
    pub driver)": status_str = "failed (no,
    } else {
    pr_info("%s: The %s driver bound to the device does not support error recovery\n",
    pci_name(pdev),
    pub support)": status_str = "failed (no driver,
    }
    pub out_unlock: goto,
    }
    pub ccdf): rc = zpci_store_pci_error(pdev,,
    if (!rc || rc == -ENOMEM)
    pub true: mediated_recovery =,
    pub driver): ers_res = zpci_event_notify_error_detected(pdev,,
    if (ers_result_indicates_abort(ers_res)) {
    pub detection)": status_str = "failed (abort on,
    pub out_unlock: goto,
    }
    if (mediated_recovery) {
    pr_info("%s: Leaving recovery of pass-through device to user-space\n",
    pub PCI_ERS_RESULT_RECOVERED: ers_res =,
    pub progress": status_str = "in,
    pub out_unlock: goto,
    }
    if (ers_res != PCI_ERS_RESULT_NEED_RESET) {
    pub driver): ers_res = zpci_event_do_error_state_clear(pdev,,
    if (ers_result_indicates_abort(ers_res)) {
    pub enable)": status_str = "failed (abort on MMIO,
    pub out_unlock: goto,
    }
    }
    if (ers_res == PCI_ERS_RESULT_NEED_RESET)
    pub driver): ers_res = zpci_event_do_reset(pdev,,
//
// ers_res can be PCI_ERS_RESULT_NONE either because the driver
// decided to return it, indicating that it abstains from voting
// on how to recover, or because it didn't implement the callback.
// Both cases assume, that if there is nothing else causing a
// disconnect, we recovered successfully.
//
    if (ers_res == PCI_ERS_RESULT_NONE)
    pub PCI_ERS_RESULT_RECOVERED: ers_res =,
    if (ers_res != PCI_ERS_RESULT_RECOVERED) {
    pub PCI_ERS_RESULT_DISCONNECT): pci_uevent_ers(pdev,,
    pub required\n",: pr_err("%s: Automatic recovery failed; operator intervention is,
    pub recover)": status_str = "failed (driver can't,
    pub out_unlock: goto,
    }
    pub pci_name(pdev)): pr_info("%s: The device is ready to resume operations\n",,
    if (driver.err_handler.resume)
    pub PCI_ERS_RESULT_RECOVERED): pci_uevent_ers(pdev,,
    out_unlock:
    pub status_str): zpci_report_status(zdev, "recovery",,
    pub ers_res: return,
    }
// zpci_event_io_failure - Report PCI channel failure state to driver
// @pdev: PCI function for which to report
// @es: PCI channel failure state to report
//
    static void zpci_event_io_failure(struct pci_dev *pdev, pci_channel_state_t es,
    struct zpci_ccdf_err *ccdf)
    {
    pub driver: *mut pci_driver,
    pub es: pdev->error_state =,
    pub ccdf): zpci_store_pci_error(pdev,,
    pub to_pci_driver(pdev->dev.driver): driver =,
    if (driver && driver.err_handler && driver.err_handler.error_detected)
    pub pdev->error_state): driver->err_handler->error_detected(pdev,,
    }
#[no_mangle]
unsafe extern "C" fn __zpci_event_print_error(pdev: *mut pci_dev, ccdf: *mut zpci_ccdf_err) {
    static void __zpci_event_print_error(struct pci_dev *pdev, struct zpci_ccdf_err *ccdf)
    {
    pr_err("%s: Event 0x%x reports an error for PCI function 0x%x\n",
    pub ccdf->fid): pdev ? pci_name(pdev) : "n/a", ccdf->pec,,
    }
#[no_mangle]
unsafe extern "C" fn __zpci_event_error(ccdf: *mut zpci_ccdf_err) {
    static void __zpci_event_error(struct zpci_ccdf_err *ccdf)
    {
    pub get_zdev_by_fid(ccdf->fid): *mut *mut zpci_dev zdev =,
    pub NULL: *mut *mut pci_dev pdev =,
    pub ers_res: pci_ers_result_t,
    pub 0: u32 fh =,
    pub rc: c_int,
    zpci_dbg(3, "err fid:%x, fh:%x, pec:%x\n",
    pub ccdf->pec): ccdf->fid, ccdf->fh,,
    pub CCDF:\n"): zpci_err("error,
    pub sizeof(*ccdf)): *mut zpci_err_hex(ccdf,,
    if (!zdev)
    pub ccdf): return __zpci_event_print_error(NULL,,
    pub &fh): rc = clp_refresh_fh(zdev->fid,,
    if (rc)
    pub no_pdev: goto,
    if (!fh || ccdf.fh != fh) {
// Ignore events with stale handles
    zpci_dbg(3, "err fid:%x, fh:%x (stale %x)\n",
    pub ccdf->fh): ccdf->fid, fh,,
    pub no_pdev: goto,
    }
    pub ccdf->fh): zpci_update_fh(zdev,,
    if (zdev.zbus.bus)
    pub zdev->devfn): pdev = pci_get_slot(zdev->zbus->bus,,
    pub ccdf): __zpci_event_print_error(pdev,,
    if (!pdev)
    pub no_pdev: goto,
    switch (ccdf.pec) {
    case 0x002a: /* Error event concerns FMB */
    case 0x002b:
    case 0x002c:
    case 0x0040: /* Service Action or Error Recovery Failed */
    case 0x003b:
    pub ccdf): zpci_event_io_failure(pdev, pci_channel_io_perm_failure,,
    default: /* PCI function left in the error state attempt to recover */
    pub ccdf): ers_res = zpci_event_attempt_error_recovery(pdev,,
    if (ers_res != PCI_ERS_RESULT_RECOVERED)
    pub ccdf): zpci_event_io_failure(pdev, pci_channel_io_perm_failure,,
    }
    no_pdev:
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_event_error(data: *mut c_void) {
    void zpci_event_error(void *data)
    {
    if (zpci_is_enabled())
    }
#[no_mangle]
unsafe extern "C" fn zpci_event_hard_deconfigured(zdev: *mut zpci_dev, fh: u32) {
    static void zpci_event_hard_deconfigured(struct zpci_dev *zdev, u32 fh)
    {
    pub fh): zpci_update_fh(zdev,,
// Give the driver a hint that the function is
// already unusable.
//
    pub true): zpci_bus_remove_device(zdev,,
// Even though the device is already gone we still
// need to free zPCI resources as part of the disable.
//
    if (zdev_enabled(zdev))
    pub ZPCI_FN_STATE_STANDBY: zdev->state =,
    }
#[no_mangle]
unsafe extern "C" fn zpci_event_reappear(zdev: *mut zpci_dev) {
    static void zpci_event_reappear(struct zpci_dev *zdev)
    {
//
// The zdev is in the reserved state. This means that it was presumed to
// go away but there are still undropped references. Now, the platform
// announced its availability again. Bring back the lingering zdev
// to standby. This is safe because we hold a temporary reference
// now so that it won't go away. Account for the re-appearance of the
// underlying device by incrementing the reference count.
//
    pub ZPCI_FN_STATE_STANDBY: zdev->state =,
    pub zdev->fh): zpci_dbg(1, "rea fid:%x, fh:%x\n", zdev->fid,,
    }
#[no_mangle]
unsafe extern "C" fn zpci_event_avail_any_device(ccdf: *mut zpci_ccdf_avail) -> bool {
    static bool zpci_event_avail_any_device(struct zpci_ccdf_avail *ccdf)
    {
// 0x0306 - No handle or fid stored
    if (ccdf.pec != 0x0306)
    pub false: return,
// 0x308 or 0x302 for multiple devices
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn zpci_event_avail_new_device(ccdf: *mut zpci_ccdf_avail) {
    static void zpci_event_avail_new_device(struct zpci_ccdf_avail *ccdf)
    {
    pub zdev: *mut zpci_dev,
    switch (ccdf.pec) {
    case 0x0301: /* Reserved|Standby . Configured */
    pub ZPCI_FN_STATE_CONFIGURED): zdev = zpci_create_device(ccdf->fid, ccdf->fh,,
    if (IS_ERR(zdev))
    if (zpci_add_device(zdev)) {
    }
    pub ccdf->fh): zpci_scan_configured_device(zdev,,
    case 0x0302: /* Reserved . Standby */
    pub ZPCI_FN_STATE_STANDBY): zdev = zpci_create_device(ccdf->fid, ccdf->fh,,
    if (IS_ERR(zdev))
    if (zpci_add_device(zdev)) {
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn zpci_event_avail_existing_device(zdev: *mut zpci_dev, ccdf: *mut zpci_ccdf_avail) {
    static void zpci_event_avail_existing_device(struct zpci_dev *zdev, struct zpci_ccdf_avail *ccdf)
    {
    pub state: enum zpci_state,
    switch (ccdf.pec) {
    case 0x0301: /* Reserved|Standby . Configured */
    if (zdev.state == ZPCI_FN_STATE_RESERVED)
// the configuration request may be stale
#[no_mangle]
pub unsafe extern "C" fn if(ZPCI_FN_STATE_STANDBY: zdev->state !=) -> else {
    else if (zdev.state != ZPCI_FN_STATE_STANDBY)
    pub ZPCI_FN_STATE_CONFIGURED: zdev->state =,
    pub ccdf->fh): zpci_scan_configured_device(zdev,,
    case 0x0302: /* Reserved . Standby */
    if (zdev.state == ZPCI_FN_STATE_RESERVED)
    pub ccdf->fh): zpci_update_fh(zdev,,
    case 0x0303: /* Deconfiguration requested */
// The event may have been queued before we configured
// the device.
//
    if (zdev.state != ZPCI_FN_STATE_CONFIGURED)
    pub ccdf->fh): zpci_update_fh(zdev,,
    case 0x0304: /* Configured . Standby|Reserved */
// The event may have been queued before we configured
// the device.:
//
    if (zdev.state == ZPCI_FN_STATE_CONFIGURED)
    pub ccdf->fh): zpci_event_hard_deconfigured(zdev,,
// The 0x0304 event may immediately reserve the device
    if (!clp_get_state(zdev.fid, &state) &&
    state == ZPCI_FN_STATE_RESERVED) {
    }
    case 0x0308: /* Standby . Reserved */
    }
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_event_availability(data: *mut c_void) {
    void zpci_event_availability(void *data)
    {
    pub data: *mut *mut zpci_ccdf_avail ccdf =,
    pub zdev: *mut zpci_dev,
    if (!zpci_is_enabled())
    zpci_dbg(3, "avl fid:%x, fh:%x, pec:%x\n",
    pub ccdf->pec): ccdf->fid, ccdf->fh,,
    if (zpci_event_avail_any_device(ccdf))
    pub get_zdev_by_fid(ccdf->fid): zdev =,
    if (!zdev)
    pub zpci_event_avail_new_device(ccdf): return,
    pub ccdf): zpci_event_avail_existing_device(zdev,,
    }
