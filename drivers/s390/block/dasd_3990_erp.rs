//! Automatically rewritten from C to Rust
//! Source: drivers/s390/block/dasd_3990_erp.c
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
// Author(s)......: Horst  Hummel    <Horst.Hummel@de.ibm.com>
// Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 2000, 2001
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DCTL_data {
    pub /: *mut *mut unsigned char subcommand; / e.g Inhibit Write, Enable Write,...,
    pub /: *mut *mut unsigned char modifier; / Subcommand modifier,
    pub /: *mut *mut unsigned short res; / reserved,
// C attribute field omitted
//
// SECTION ERP HANDLING
//
// 24 and 32 byte sense ERP functions
//
// DASD_3990_ERP_CLEANUP
//
// DESCRIPTION
// Removes the already build but not necessary ERP request and sets
// the status of the original cqr / erp to the given (final) status
//
// PARAMETER
// erp		request to be blocked
// final_status	either DASD_CQR_DONE or DASD_CQR_FAILED
//
// RETURN VALUES
// cqr		original cqr
//
    static struct dasd_ccw_req *
    dasd_3990_erp_cleanup(struct dasd_ccw_req * erp, char final_status)
    {
    pub erp->refers: *mut *mut dasd_ccw_req cqr =,
    pub erp->memdev): dasd_free_erp_request(erp,,
    pub final_status: cqr->status =,
    pub cqr: return,
    }				/* end dasd_3990_erp_cleanup */
//
// DASD_3990_ERP_BLOCK_QUEUE
//
// DESCRIPTION
// Block the given device request queue to prevent from further
// processing until the started timer has expired or an related
// interrupt was received.
//
#[no_mangle]
unsafe extern "C" fn dasd_3990_erp_block_queue(erp: *mut dasd_ccw_req, expires: c_int) {
    static void dasd_3990_erp_block_queue(struct dasd_ccw_req *erp, int expires)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub flags: c_ulong,
    DBF_DEV_EVENT(DBF_INFO, device,
    pub expires/HZ): "blocking request queue for %is",,
    pub flags): spin_lock_irqsave(get_ccwdev_lock(device->cdev),,
    pub DASD_STOPPED_PENDING): dasd_device_set_stop_bits(device,,
    pub flags): spin_unlock_irqrestore(get_ccwdev_lock(device->cdev),,
    pub DASD_CQR_FILLED: erp->status =,
    if (erp.block)
    pub expires): dasd_block_set_timer(erp->block,,
    else
    pub expires): dasd_device_set_timer(device,,
    }
//
// DASD_3990_ERP_INT_REQ
//
// DESCRIPTION
// Handles 'Intervention Required' error.
// This means either device offline or not installed.
//
// PARAMETER
// erp		current erp
// RETURN VALUES
// erp		modified erp
//
    static struct dasd_ccw_req *
    dasd_3990_erp_int_req(struct dasd_ccw_req * erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
// first time set initial retry counter and erp_function
// and retry once without blocking queue
// (this enables easier enqueing of the cqr)
    if (erp.function != dasd_3990_erp_int_req) {
    pub 256: erp->retries =,
    pub dasd_3990_erp_int_req: erp->function =,
    } else {
// issue a message and wait for 'device ready' interrupt
    dev_err(&device.cdev.dev,
    "is offline or not installed - "
    pub REQUIRED!!\n"): "INTERVENTION,
    pub 60*HZ): *mut dasd_3990_erp_block_queue(erp,,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_int_req */
//
// DASD_3990_ERP_ALTERNATE_PATH
//
// DESCRIPTION
// Repeat the operation on a different channel path.
// If all alternate paths have been tried, the request is posted with a
// permanent error.
//
// PARAMETER
// erp		pointer to the current ERP
//
// RETURN VALUES
// erp		modified pointer to the ERP
//
    static void
    dasd_3990_erp_alternate_path(struct dasd_ccw_req * erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub opm: __u8,
    pub flags: c_ulong,
// try alternate valid path
    pub flags): spin_lock_irqsave(get_ccwdev_lock(device->cdev),,
    pub ccw_device_get_path_mask(device->cdev): opm =,
    pub flags): spin_unlock_irqrestore(get_ccwdev_lock(device->cdev),,
    if (erp.lpm == 0)
    erp.lpm = dasd_path_get_opm(device) &
    else
    pub ~(erp->irb.esw.esw0.sublog.lpum): erp->lpm &=,
    if ((erp.lpm & opm) != 0x00) {
    DBF_DEV_EVENT(DBF_WARNING, device,
    "try alternate lpm=%x (lpum=%x / opm=%x)",
    pub opm): erp->lpm, erp->irb.esw.esw0.sublog.lpum,,
// reset status to submit the request again...
    pub DASD_CQR_FILLED: erp->status =,
    pub 10: erp->retries =,
    } else {
    dev_err(&device.cdev.dev,
    "The DASD cannot be reached on any path (lpum=%x"
    pub opm): "/opm=%x)\n", erp->irb.esw.esw0.sublog.lpum,,
// post request with permanent error
    pub DASD_CQR_FAILED: erp->status =,
    }
    }				/* end dasd_3990_erp_alternate_path */
//
// DASD_3990_ERP_DCTL
//
// DESCRIPTION
// Setup cqr to do the Diagnostic Control (DCTL) command with an
// Inhibit Write subcommand (0x20) and the given modifier.
//
// PARAMETER
// erp		pointer to the current (failed) ERP
// modifier		subcommand modifier
//
// RETURN VALUES
// dctl_cqr		pointer to NEW dctl_cqr
//
    static struct dasd_ccw_req *
    dasd_3990_erp_DCTL(struct dasd_ccw_req * erp, char modifier)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub DCTL_data: *mut DCTL_data,
    pub ccw: *mut ccw1,
    pub dctl_cqr: *mut dasd_ccw_req,
    dctl_cqr = dasd_alloc_erp_request(erp.magic, 1,
    sizeof(struct DCTL_data),
    if (IS_ERR(dctl_cqr)) {
    dev_err(&device.cdev.dev,
    pub DCTL-CQR\n"): "Unable to allocate,
    pub DASD_CQR_FAILED: erp->status =,
    pub erp: return,
    }
    pub dctl_cqr->data: DCTL_data =,
    pub /: *mut *mut DCTL_data->subcommand = 0x02; / Inhibit Write,
    pub modifier: DCTL_data->modifier =,
    pub dctl_cqr->cpaddr: ccw =,
    pub ccw1)): memset(ccw, 0, sizeof(struct,
    pub CCW_CMD_DCTL: ccw->cmd_code =,
    pub 4: ccw->count =,
    pub virt_to_dma32(DCTL_data): ccw->cda =,
    pub erp->flags: dctl_cqr->flags =,
    pub dasd_3990_erp_DCTL: dctl_cqr->function =,
    pub erp: dctl_cqr->refers =,
    pub device: dctl_cqr->startdev =,
    pub device: dctl_cqr->memdev =,
    pub erp->magic: dctl_cqr->magic =,
    pub HZ: *mut *mut *mut dctl_cqr->expires = 5  60,
    pub 2: dctl_cqr->retries =,
    pub get_tod_clock(): dctl_cqr->buildclk =,
    pub DASD_CQR_FILLED: dctl_cqr->status =,
    pub dctl_cqr: return,
    }				/* end dasd_3990_erp_DCTL */
//
// DASD_3990_ERP_ACTION_1
//
// DESCRIPTION
// Setup ERP to do the ERP action 1 (see Reference manual).
// Repeat the operation on a different channel path.
// As deviation from the recommended recovery action, we reset the path mask
// after we have tried each path and go through all paths a second time.
// This will cover situations where only one path at a time is actually down,
// but all paths fail and recover just with the same sequence and timing as
// we try to use them (flapping links).
// If all alternate paths have been tried twice, the request is posted with
// a permanent error.
//
// PARAMETER
// erp		pointer to the current ERP
//
// RETURN VALUES
// erp		pointer to the ERP
//
    static struct dasd_ccw_req *dasd_3990_erp_action_1_sec(struct dasd_ccw_req *erp)
    {
    pub dasd_3990_erp_action_1_sec: erp->function =,
    pub erp: return,
    }
    static struct dasd_ccw_req *dasd_3990_erp_action_1(struct dasd_ccw_req *erp)
    {
    pub dasd_3990_erp_action_1: erp->function =,
    if (erp.status == DASD_CQR_FAILED &&
    !test_bit(DASD_CQR_VERIFY_PATH, &erp.flags)) {
    pub DASD_CQR_FILLED: erp->status =,
    pub 10: erp->retries =,
    pub dasd_path_get_opm(erp->startdev): erp->lpm =,
    pub dasd_3990_erp_action_1_sec: erp->function =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_action_1(b) */
//
// DASD_3990_ERP_ACTION_4
//
// DESCRIPTION
// Setup ERP to do the ERP action 4 (see Reference manual).
// Set the current request to PENDING to block the CQR queue for that device
// until the state change interrupt appears.
// Use a timer (20 seconds) to retry the cqr if the interrupt is still
// missing.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the current ERP
//
// RETURN VALUES
// erp		pointer to the ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_action_4(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
// first time set initial retry counter and erp_function
// and retry once without waiting for state change pending
// interrupt (this enables easier enqueing of the cqr)
    if (erp.function != dasd_3990_erp_action_4) {
    DBF_DEV_EVENT(DBF_INFO, device, "%s",
    pub retry"): "dasd_3990_erp_action_4: first time,
    pub 256: erp->retries =,
    pub dasd_3990_erp_action_4: erp->function =,
    } else {
    if (sense && (sense[25] == 0x1D)) { /* state change pending */
    DBF_DEV_EVENT(DBF_INFO, device,
    "waiting for state change pending "
    "interrupt, %d retries left",
    pub 30*HZ): *mut dasd_3990_erp_block_queue(erp,,
    } else if (sense && (sense[25] == 0x1E)) {	/* busy */
    DBF_DEV_EVENT(DBF_INFO, device,
    "busy - redriving request later, "
    "%d retries left",
    pub HZ): dasd_3990_erp_block_queue(erp,,
    } else {
// no state change pending - retry
    DBF_DEV_EVENT(DBF_INFO, device,
    "redriving request immediately, "
    "%d retries left",
    pub DASD_CQR_FILLED: erp->status =,
    }
    }
    pub erp: return,
    }				/* end dasd_3990_erp_action_4 */
//
// 24 byte sense ERP functions (only)
//
// DASD_3990_ERP_ACTION_5
//
// DESCRIPTION
// Setup ERP to do the ERP action 5 (see Reference manual).
// NOTE: Further handling is done in xxx_further_erp after the retries.
//
// PARAMETER
// erp		pointer to the current ERP
//
// RETURN VALUES
// erp		pointer to the ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_action_5(struct dasd_ccw_req * erp)
    {
// first of all retry
    pub 10: erp->retries =,
    pub dasd_3990_erp_action_5: erp->function =,
    pub erp: return,
    }				/* end dasd_3990_erp_action_5 */
//
// DASD_3990_HANDLE_ENV_DATA
//
// DESCRIPTION
// Handles 24 byte 'Environmental data present'.
// Does a analysis of the sense data (message Format)
// and prints the error messages.
//
// PARAMETER
// sense		current sense data
//
// RETURN VALUES
// void
//
    static void
    dasd_3990_handle_env_data(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub 0xF0): char msg_format = (sense[7] &,
    pub 0x0F): char msg_no = (sense[7] &,
    switch (msg_format) {
    case 0x00:		/* Format 0 - Program or System Checks */
    if (sense[1] & 0x10) {	/* check message to operator bit */
    switch (msg_no) {
    case 0x00:	/* No Message */
    case 0x01:
    dev_warn(&device.cdev.dev,
    pub Command\n"): "FORMAT 0 - Invalid,
    case 0x02:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Invalid Command "
    case 0x03:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - CCW Count less than "
    case 0x04:
    dev_warn(&device.cdev.dev,
    pub Parameter\n"): "FORMAT 0 - Invalid,
    case 0x05:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Diagnostic of Special"
    pub Mask\n"): " Command Violates File,
    case 0x07:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Channel Returned with "
    pub CCW\n"): "Incorrect retry,
    case 0x08:
    dev_warn(&device.cdev.dev,
    pub Notification\n"): "FORMAT 0 - Reset,
    case 0x09:
    dev_warn(&device.cdev.dev,
    pub Restart\n"): "FORMAT 0 - Storage Path,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Channel requested "
    pub sense[8]): "... %02x\n",,
    case 0x0B:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Invalid Defective/"
    pub Pointer\n"): "Alternate Track,
    case 0x0C:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - DPS Installation "
    case 0x0E:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Command Invalid on "
    pub Address\n"): "Secondary,
    case 0x0F:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Status Not As "
    "Required: reason %02x\n",
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 0 -,
    }
    } else {
    switch (msg_no) {
    case 0x00:	/* No Message */
    case 0x01:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Device Error "
    case 0x02:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 0 -,
    case 0x03:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Device Fenced - "
    pub sense[4]): "device = %02x\n",,
    case 0x04:
    dev_warn(&device.cdev.dev,
    "FORMAT 0 - Data Pinned for "
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 0 -,
    }
    }
    case 0x10:		/* Format 1 - Device Equipment Checks */
    switch (msg_no) {
    case 0x00:	/* No Message */
    case 0x01:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Device Status 1 not as "
    case 0x03:
    dev_warn(&device.cdev.dev,
    pub missing\n"): "FORMAT 1 - Index,
    case 0x04:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Interruption cannot be "
    case 0x05:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Device did not respond to "
    case 0x06:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Device check-2 error or Set "
    pub complete\n"): "Sector is not,
    case 0x07:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Head address does not "
    case 0x08:
    dev_warn(&device.cdev.dev,
    pub valid\n"): "FORMAT 1 - Device status 1 not,
    case 0x09:
    dev_warn(&device.cdev.dev,
    pub ready\n"): "FORMAT 1 - Device not,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Track physical address did "
    pub compare\n"): "not,
    case 0x0B:
    dev_warn(&device.cdev.dev,
    pub bit\n"): "FORMAT 1 - Missing device address,
    case 0x0C:
    dev_warn(&device.cdev.dev,
    pub off\n"): "FORMAT 1 - Drive motor switch is,
    case 0x0D:
    dev_warn(&device.cdev.dev,
    pub incomplete\n"): "FORMAT 1 - Seek,
    case 0x0E:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Cylinder address did not "
    case 0x0F:
    dev_warn(&device.cdev.dev,
    "FORMAT 1 - Offset active cannot be "
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 1 -,
    }
    case 0x20:		/* Format 2 - 3990 Equipment Checks */
    switch (msg_no) {
    case 0x08:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 2 - 3990 check-2,
    case 0x0E:
    dev_warn(&device.cdev.dev,
    pub errors\n"): "FORMAT 2 - Support facility,
    case 0x0F:
    dev_warn(&device.cdev.dev,
    "FORMAT 2 - Microcode detected error "
    "%02x\n",
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 2 -,
    }
    case 0x30:		/* Format 3 - 3990 Control Checks */
    switch (msg_no) {
    case 0x0F:
    dev_warn(&device.cdev.dev,
    pub terminated\n"): "FORMAT 3 - Allegiance,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 3 -,
    }
    case 0x40:		/* Format 4 - Data Checks */
    switch (msg_no) {
    case 0x00:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 4 - Home address area,
    case 0x01:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 4 - Count area,
    case 0x02:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 4 - Key area,
    case 0x03:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 4 - Data area,
    case 0x04:
    dev_warn(&device.cdev.dev,
    "FORMAT 4 - No sync byte in home address "
    case 0x05:
    dev_warn(&device.cdev.dev,
    "FORMAT 4 - No sync byte in count address "
    case 0x06:
    dev_warn(&device.cdev.dev,
    pub area\n"): "FORMAT 4 - No sync byte in key,
    case 0x07:
    dev_warn(&device.cdev.dev,
    pub area\n"): "FORMAT 4 - No sync byte in data,
    case 0x08:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - Home address area error;,
    pub active\n"): "offset,
    case 0x09:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - Count area error; offset,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - Key area error; offset,
    case 0x0B:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - Data area error;,
    pub active\n"): "offset,
    case 0x0C:
    dev_warn(&device.cdev.dev,
    "FORMAT 4 - No sync byte in home "
    pub active\n"): "address area; offset,
    case 0x0D:
    dev_warn(&device.cdev.dev,
    "FORMAT 4 - No sync byte in count "
    pub active\n"): "address area; offset,
    case 0x0E:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - No sync byte in key area;,
    pub active\n"): "offset,
    case 0x0F:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 4 - No sync byte in data area;,
    pub active\n"): "offset,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 4 -,
    }
    case 0x50:  /* Format 5 - Data Check with displacement information */
    switch (msg_no) {
    case 0x00:
    dev_warn(&device.cdev.dev,
    "FORMAT 5 - Data Check in the "
    pub area\n"): "home address,
    case 0x01:
    dev_warn(&device.cdev.dev,
    "FORMAT 5 - Data Check in the count "
    case 0x02:
    dev_warn(&device.cdev.dev,
    pub area\n"): "FORMAT 5 - Data Check in the key,
    case 0x03:
    dev_warn(&device.cdev.dev,
    "FORMAT 5 - Data Check in the data "
    case 0x08:
    dev_warn(&device.cdev.dev,
    "FORMAT 5 - Data Check in the "
    pub active\n"): "home address area; offset,
    case 0x09:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 5 - Data Check in the count area;,
    pub active\n"): "offset,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 5 - Data Check in the key area;,
    pub active\n"): "offset,
    case 0x0B:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 5 - Data Check in the data area;,
    pub active\n"): "offset,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 5 -,
    }
    case 0x60:  /* Format 6 - Usage Statistics/Overrun Errors */
    switch (msg_no) {
    case 0x00:
    dev_warn(&device.cdev.dev,
    pub A\n"): "FORMAT 6 - Overrun on channel,
    case 0x01:
    dev_warn(&device.cdev.dev,
    pub B\n"): "FORMAT 6 - Overrun on channel,
    case 0x02:
    dev_warn(&device.cdev.dev,
    pub C\n"): "FORMAT 6 - Overrun on channel,
    case 0x03:
    dev_warn(&device.cdev.dev,
    pub D\n"): "FORMAT 6 - Overrun on channel,
    case 0x04:
    dev_warn(&device.cdev.dev,
    pub E\n"): "FORMAT 6 - Overrun on channel,
    case 0x05:
    dev_warn(&device.cdev.dev,
    pub F\n"): "FORMAT 6 - Overrun on channel,
    case 0x06:
    dev_warn(&device.cdev.dev,
    pub G\n"): "FORMAT 6 - Overrun on channel,
    case 0x07:
    dev_warn(&device.cdev.dev,
    pub H\n"): "FORMAT 6 - Overrun on channel,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 6 -,
    }
    case 0x70:  /* Format 7 - Device Connection Control Checks */
    switch (msg_no) {
    case 0x00:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - RCC initiated by a connection "
    pub alert\n"): "check,
    case 0x01:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - RCC 1 sequence not "
    case 0x02:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - RCC 1 and RCC 2 sequences not "
    case 0x03:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - Invalid tag-in during "
    pub sequence\n"): "selection,
    case 0x04:
    dev_warn(&device.cdev.dev,
    pub required\n"): "FORMAT 7 - extra RCC,
    case 0x05:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - Invalid DCC selection "
    pub timeout\n"): "response or,
    case 0x06:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 7 - Missing end operation; device,
    pub complete\n"): "transfer,
    case 0x07:
    dev_warn(&device.cdev.dev,
    pub ": "FORMAT 7 - Missing end operation; device,
    pub incomplete\n"): "transfer,
    case 0x08:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - Invalid tag-in for an "
    pub sequence\n"): "immediate command,
    case 0x09:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - Invalid tag-in for an "
    pub sequence\n"): "extended command,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - 3990 microcode time out when "
    pub selection\n"): "stopping,
    case 0x0B:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - No response to selection "
    pub interruption\n"): "after a poll,
    case 0x0C:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - Permanent path error (DASD "
    pub available)\n"): "controller not,
    case 0x0D:
    dev_warn(&device.cdev.dev,
    "FORMAT 7 - DASD controller not available"
    pub chain\n"): " on disconnected command,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 7 -,
    }
    case 0x80:  /* Format 8 - Additional Device Equipment Checks */
    switch (msg_no) {
    case 0x00:	/* No Message */
    case 0x01:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - Error correction code "
    pub fault\n"): "hardware,
    case 0x03:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - Unexpected end operation "
    pub code\n"): "response,
    case 0x04:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - End operation with transfer "
    pub zero\n"): "count not,
    case 0x05:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - End operation with transfer "
    pub zero\n"): "count,
    case 0x06:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - DPS checks after a system "
    pub reset\n"): "reset or selective,
    case 0x07:
    dev_warn(&device.cdev.dev,
    pub filled\n"): "FORMAT 8 - DPS cannot be,
    case 0x08:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - Short busy time-out during "
    pub selection\n"): "device,
    case 0x09:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - DASD controller failed to "
    pub latch\n"): "set or reset the long busy,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT 8 - No interruption from device "
    pub chain\n"): "during a command,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 8 -,
    }
    case 0x90:  /* Format 9 - Device Read, Write, and Seek Checks */
    switch (msg_no) {
    case 0x00:
    pub /: *mut *mut break; / No Message,
    case 0x06:
    dev_warn(&device.cdev.dev,
    pub error\n"): "FORMAT 9 - Device check-2,
    case 0x07:
    dev_warn(&device.cdev.dev,
    "FORMAT 9 - Head address did not "
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT 9 - Track physical address did "
    pub oriented\n"): "not compare while,
    case 0x0E:
    dev_warn(&device.cdev.dev,
    "FORMAT 9 - Cylinder address did not "
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT 9 -,
    }
    case 0xF0:		/* Format F - Cache Storage Checks */
    switch (msg_no) {
    case 0x00:
    dev_warn(&device.cdev.dev,
    pub Terminated\n"): "FORMAT F - Operation,
    case 0x01:
    dev_warn(&device.cdev.dev,
    pub Error\n"): "FORMAT F - Subsystem Processing,
    case 0x02:
    dev_warn(&device.cdev.dev,
    "FORMAT F - Cache or nonvolatile storage "
    pub failure\n"): "equipment,
    case 0x04:
    dev_warn(&device.cdev.dev,
    pub terminated\n"): "FORMAT F - Caching,
    case 0x06:
    dev_warn(&device.cdev.dev,
    "FORMAT F - Cache fast write access not "
    case 0x07:
    dev_warn(&device.cdev.dev,
    pub incorrect\n"): "FORMAT F - Track format,
    case 0x09:
    dev_warn(&device.cdev.dev,
    pub reinitiated\n"): "FORMAT F - Caching,
    case 0x0A:
    dev_warn(&device.cdev.dev,
    "FORMAT F - Nonvolatile storage "
    case 0x0B:
    dev_warn(&device.cdev.dev,
    pub duplex\n"): "FORMAT F - Volume is suspended,
// call extended error reporting (EER)
    dasd_eer_write(device, erp.refers,
    case 0x0C:
    dev_warn(&device.cdev.dev,
    "FORMAT F - Subsystem status cannot be "
    case 0x0D:
    dev_warn(&device.cdev.dev,
    "FORMAT F - Caching status reset to "
    case 0x0E:
    dev_warn(&device.cdev.dev,
    pub inhibited\n"): "FORMAT F - DASD Fast Write,
    default:
    dev_warn(&device.cdev.dev,
    pub Reserved\n"): "FORMAT F -,
    }
    default:
    dev_err(&device.cdev.dev,
    pub msg_format): "Unknown message format %02x",,
    }			/* end switch message format */
    }				/* end dasd_3990_handle_env_data */
//
// DASD_3990_ERP_COM_REJ
//
// DESCRIPTION
// Handles 24 byte 'Command Reject' error.
//
// PARAMETER
// erp		current erp_head
// sense		current sense data
//
// RETURN VALUES
// erp		'new' erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_com_rej(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_com_rej: erp->function =,
// env data present (ACTION 10 - retry should work)
    if (sense[2] & SNS2_ENV_DATA_PRESENT) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub present"): "Command Reject - environmental data,
    pub sense): dasd_3990_handle_env_data(erp,,
    pub 5: erp->retries =,
    } else if (sense[1] & SNS1_WRITE_INHIBITED) {
    dev_err(&device.cdev.dev, "An I/O request was rejected"
    pub inhibited\n"): " because writing is,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    } else if (sense[7] == SNS7_INVALID_ON_SEC) {
    pub device\n"): dev_err(&device->cdev->dev, "An I/O request was rejected on a copy pair secondary,
// suppress dump of sense data for this error
    pub &erp->refers->flags): set_bit(DASD_CQR_SUPPRESS_CR,,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    } else {
    if (!test_bit(DASD_CQR_SUPPRESS_CR, &erp.flags))
    dev_err(&device.cdev.dev,
    pub rejected\n"): "An I/O command request was,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_com_rej */
//
// DASD_3990_ERP_BUS_OUT
//
// DESCRIPTION
// Handles 24 byte 'Bus Out Parity Check' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_bus_out(struct dasd_ccw_req * erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
// first time set initial retry counter and erp_function
// and retry once without blocking queue
// (this enables easier enqueing of the cqr)
    if (erp.function != dasd_3990_erp_bus_out) {
    pub 256: erp->retries =,
    pub dasd_3990_erp_bus_out: erp->function =,
    } else {
// issue a message and wait for 'device ready' interrupt
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "bus out parity error or BOPC requested by "
    pub 60*HZ): *mut dasd_3990_erp_block_queue(erp,,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_bus_out */
//
// DASD_3990_ERP_EQUIP_CHECK
//
// DESCRIPTION
// Handles 24 byte 'Equipment Check' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_equip_check(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_equip_check: erp->function =,
    if (sense[1] & SNS1_WRITE_INHIBITED) {
    pub encountered\n"): dev_err(&device->cdev->dev, "Write inhibited path,
    pub dasd_3990_erp_action_1(erp): erp =,
    } else if (sense[2] & SNS2_ENV_DATA_PRESENT) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub present"): "Equipment Check - " "environmental data,
    pub sense): dasd_3990_handle_env_data(erp,,
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    } else if (sense[1] & SNS1_PERM_ERR) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Equipment Check - retry exhausted or "
    pub dasd_3990_erp_action_1(erp): erp =,
    } else {
// all other equipment checks - Action 5
// rest is done when retries == 0
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub error"): "Equipment check or processing,
    pub dasd_3990_erp_action_5(erp): erp =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_equip_check */
//
// DASD_3990_ERP_DATA_CHECK
//
// DESCRIPTION
// Handles 24 byte 'Data Check' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_data_check(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_data_check: erp->function =,
    if (sense[2] & SNS2_CORRECTABLE) {	/* correctable data check */
// issue message that the data has been corrected
    dev_emerg(&device.cdev.dev,
    "Data recovered during retry with PCI "
    pub active\n"): "fetch mode,
// not possible to handle this situation in Linux
    panic("No way to inform application about the possibly "
    pub data"): "incorrect,
    } else if (sense[2] & SNS2_ENV_DATA_PRESENT) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Uncorrectable data check recovered secondary "
    pub pair"): "addr of duplex,
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    } else if (sense[1] & SNS1_PERM_ERR) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Uncorrectable data check with internal "
    pub exhausted"): "retry,
    pub dasd_3990_erp_action_1(erp): erp =,
    } else {
// all other data checks
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Uncorrectable data check with retry count "
    pub dasd_3990_erp_action_5(erp): erp =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_data_check */
//
// DASD_3990_ERP_OVERRUN
//
// DESCRIPTION
// Handles 24 byte 'Overrun' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_overrun(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_overrun: erp->function =,
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Overrun - service overrun or overrun"
    pub channel"): " error requested by,
    pub dasd_3990_erp_action_5(erp): erp =,
    pub erp: return,
    }				/* end dasd_3990_erp_overrun */
//
// DASD_3990_ERP_INV_FORMAT
//
// DESCRIPTION
// Handles 24 byte 'Invalid Track Format' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_inv_format(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_inv_format: erp->function =,
    if (sense[2] & SNS2_ENV_DATA_PRESENT) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Track format error when destaging or "
    pub data"): "staging,
    pub sense): dasd_3990_handle_env_data(erp,,
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    } else {
    pub valid\n"): dev_err(&device->cdev->dev, "Track format is not,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_inv_format */
//
// DASD_3990_ERP_EOC
//
// DESCRIPTION
// Handles 24 byte 'End-of-Cylinder' error.
//
// PARAMETER
// erp		already added default erp
// RETURN VALUES
// erp		pointer to original (failed) cqr.
//
    static struct dasd_ccw_req *
    dasd_3990_erp_EOC(struct dasd_ccw_req * default_erp, char *sense)
    {
    pub default_erp->startdev: *mut *mut dasd_device device =,
    dev_err(&device.cdev.dev,
    pub inconsistent\n"): "The cylinder data for accessing the DASD is,
// implement action 7 - BUG
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(default_erp,,
    }				/* end dasd_3990_erp_EOC */
//
// DASD_3990_ERP_ENV_DATA
//
// DESCRIPTION
// Handles 24 byte 'Environmental-Data Present' error.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_env_data(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_env_data: erp->function =,
    pub present"): DBF_DEV_EVENT(DBF_WARNING, device, "%s", "Environmental data,
    pub sense): dasd_3990_handle_env_data(erp,,
// don't retry on disabled interface
    if (sense[7] != 0x0F) {
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    } else {
    pub DASD_CQR_FILLED: erp->status =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_env_data */
//
// DASD_3990_ERP_NO_REC
//
// DESCRIPTION
// Handles 24 byte 'No Record Found' error.
//
// PARAMETER
// erp		already added default ERP
//
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_no_rec(struct dasd_ccw_req * default_erp, char *sense)
    {
    pub default_erp->startdev: *mut *mut dasd_device device =,
//
// In some cases the 'No Record Found' error might be expected and
// log messages shouldn't be written then.
// Check if the according suppress bit is set.
//
    if (!test_bit(DASD_CQR_SUPPRESS_NRF, &default_erp.flags))
    dev_err(&device.cdev.dev,
    pub found\n"): "The specified record was not,
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(default_erp,,
    }				/* end dasd_3990_erp_no_rec */
//
// DASD_3990_ERP_FILE_PROT
//
// DESCRIPTION
// Handles 24 byte 'File Protected' error.
// Note: Seek related recovery is not implemented because
// wee don't use the seek command yet.
//
// PARAMETER
// erp		current erp_head
// RETURN VALUES
// erp		new erp_head - pointer to new ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_file_prot(struct dasd_ccw_req * erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    dev_err(&device.cdev.dev,
    pub error\n"): "Accessing the DASD failed because of a hardware,
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(erp,,
    }				/* end dasd_3990_erp_file_prot */
//
// DASD_3990_ERP_INSPECT_ALIAS
//
// DESCRIPTION
// Checks if the original request was started on an alias device.
// If yes, it modifies the original and the erp request so that
// the erp request can be started on a base device.
//
// PARAMETER
// erp		pointer to the currently created default ERP
//
// RETURN VALUES
// erp		pointer to the modified ERP, or NULL
//
    static struct dasd_ccw_req *dasd_3990_erp_inspect_alias(
    struct dasd_ccw_req *erp)
    {
    pub erp->refers: *mut *mut dasd_ccw_req cqr =,
    pub sense: *mut c_char,
    if (cqr.block &&
    (cqr.block.base != cqr.startdev)) {
    pub dasd_get_sense(&erp->refers->irb): sense =,
//
// dynamic pav may have changed base alias mapping
//
    if (!test_bit(DASD_FLAG_OFFLINE, &cqr.startdev.flags) && sense
    && (sense[0] == 0x10) && (sense[7] == 0x0F)
    && (sense[8] == 0x67)) {
//
// remove device from alias handling to prevent new
// requests from being scheduled on the
// wrong alias device
//
// schedule worker to reload device
    }
    if (cqr.startdev.features & DASD_FEATURE_ERPLOG) {
    DBF_DEV_EVENT(DBF_ERR, cqr.startdev,
    "ERP on alias device for request %p,"
    " recover on base device %s", cqr,
    }
    pub cqr->block->base: erp->startdev =,
    pub dasd_3990_erp_inspect_alias: erp->function =,
    pub erp: return,
    } else
    pub NULL: return,
    }
//
// DASD_3990_ERP_INSPECT_24
//
// DESCRIPTION
// Does a detailed inspection of the 24 byte sense data
// and sets up a related error recovery action.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created default ERP
//
// RETURN VALUES
// erp		pointer to the (addtitional) ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_inspect_24(struct dasd_ccw_req * erp, char *sense)
    {
    pub NULL: *mut *mut dasd_ccw_req erp_filled =,
// Check sense for ....
// 'Command Reject'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_CMD_REJECT)) {
    pub sense): erp_filled = dasd_3990_erp_com_rej(erp,,
    }
// 'Intervention Required'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_INTERVENTION_REQ)) {
    pub dasd_3990_erp_int_req(erp): erp_filled =,
    }
// 'Bus Out Parity Check'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_BUS_OUT_CHECK)) {
    pub dasd_3990_erp_bus_out(erp): erp_filled =,
    }
// 'Equipment Check'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_EQUIPMENT_CHECK)) {
    pub sense): erp_filled = dasd_3990_erp_equip_check(erp,,
    }
// 'Data Check'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_DATA_CHECK)) {
    pub sense): erp_filled = dasd_3990_erp_data_check(erp,,
    }
// 'Overrun'
    if ((erp_filled == core::ptr::null_mut()) && (sense[0] & SNS0_OVERRUN)) {
    pub sense): erp_filled = dasd_3990_erp_overrun(erp,,
    }
// 'Invalid Track Format'
    if ((erp_filled == core::ptr::null_mut()) && (sense[1] & SNS1_INV_TRACK_FORMAT)) {
    pub sense): erp_filled = dasd_3990_erp_inv_format(erp,,
    }
// 'End-of-Cylinder'
    if ((erp_filled == core::ptr::null_mut()) && (sense[1] & SNS1_EOC)) {
    pub sense): erp_filled = dasd_3990_erp_EOC(erp,,
    }
// 'Environmental Data'
    if ((erp_filled == core::ptr::null_mut()) && (sense[2] & SNS2_ENV_DATA_PRESENT)) {
    pub sense): erp_filled = dasd_3990_erp_env_data(erp,,
    }
// 'No Record Found'
    if ((erp_filled == core::ptr::null_mut()) && (sense[1] & SNS1_NO_REC_FOUND)) {
    pub sense): erp_filled = dasd_3990_erp_no_rec(erp,,
    }
// 'File Protected'
    if ((erp_filled == core::ptr::null_mut()) && (sense[1] & SNS1_FILE_PROTECTED)) {
    pub dasd_3990_erp_file_prot(erp): erp_filled =,
    }
// other (unknown) error - do default ERP
    if (erp_filled == core::ptr::null_mut()) {
    pub erp: erp_filled =,
    }
    pub erp_filled: return,
    }				/* END dasd_3990_erp_inspect_24 */
//
// 32 byte sense ERP functions (only)
//
// DASD_3990_ERPACTION_10_32
//
// DESCRIPTION
// Handles 32 byte 'Action 10' of Single Program Action Codes.
// Just retry and if retry doesn't work, return with error.
//
// PARAMETER
// erp		current erp_head
// sense		current sense data
// RETURN VALUES
// erp		modified erp_head
//
    static struct dasd_ccw_req *
    dasd_3990_erp_action_10_32(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub 256: erp->retries =,
    pub dasd_3990_erp_action_10_32: erp->function =,
    pub requested"): DBF_DEV_EVENT(DBF_WARNING, device, "%s", "Perform logging,
    pub erp: return,
    }				/* end dasd_3990_erp_action_10_32 */
//
// DASD_3990_ERP_ACTION_1B_32
//
// DESCRIPTION
// Handles 32 byte 'Action 1B' of Single Program Action Codes.
// A write operation could not be finished because of an unexpected
// condition.
// The already created 'default erp' is used to get the link to
// the erp chain, but it can not be used for this recovery
// action because it contains no DE/LO data space.
//
// PARAMETER
// default_erp	already added default erp.
// sense		current sense data
//
// RETURN VALUES
// erp		new erp or
// default_erp in case of imprecise ending or error
//
    static struct dasd_ccw_req *
    dasd_3990_erp_action_1B_32(struct dasd_ccw_req * default_erp, char *sense)
    {
    pub default_erp->startdev: *mut *mut dasd_device device =,
    pub 0: dma32_t cpa =,
    pub cqr: *mut dasd_ccw_req,
    pub erp: *mut dasd_ccw_req,
    pub DE_data: *mut DE_eckd_data,
    pub PFX_data: *mut PFX_eckd_data,
    pub /: *mut *mut *mut char LO_data; / LO_eckd_data_t,
    pub oldccw: *mut *mut ccw1 ccw,,
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub condition"): "Write not finished because of unexpected,
    pub dasd_3990_erp_action_1B_32: default_erp->function =,
// determine the original cqr
    pub default_erp: cqr =,
    while (cqr.refers != core::ptr::null_mut()) {
    pub cqr->refers: cqr =,
    }
    if (scsw_is_tm(&cqr.irb.scsw)) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "32 bit sense, action 1B is not defined"
    pub retry"): " in transport mode - just,
    pub default_erp: return,
    }
// for imprecise ending just do default erp
    if (sense[1] & 0x01) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub retry"): "Imprecise ending is set - just,
    pub default_erp: return,
    }
// determine the address of the CCW to be restarted
// Imprecise ending is not set -> addr from IRB-SCSW
    pub default_erp->refers->irb.scsw.cmd.cpa: cpa =,
    if (cpa == 0) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Unable to determine address of the CCW "
    pub restarted"): "to be,
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(default_erp,,
    }
// Build new ERP request including DE/LO
    erp = dasd_alloc_erp_request(cqr.magic,
    2 + 1,/* DE/LO + TIC */
    sizeof(struct DE_eckd_data) +
    pub device): sizeof(struct LO_eckd_data),,
    if (IS_ERR(erp)) {
    DBF_DEV_EVENT(DBF_ERR, device, "%s",
    pub 32)"): "Unable to allocate ERP request (1B,
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(default_erp,,
    }
// use original DE
    pub erp->data: DE_data =,
    pub cqr->cpaddr: oldccw =,
    if (oldccw.cmd_code == DASD_ECKD_CCW_PFX) {
    pub cqr->data: PFX_data =,
    memcpy(DE_data, &PFX_data.define_extent,
    pub DE_eckd_data)): sizeof(struct,
    } else
    pub DE_eckd_data)): memcpy(DE_data, cqr->data, sizeof(struct,
// create LO
    pub DE_eckd_data): LO_data = erp->data + sizeof(struct,
    if ((sense[3] == 0x01) && (LO_data[1] & 0x01)) {
// should not
    pub DASD_CQR_FAILED): return dasd_3990_erp_cleanup(default_erp,,
    }
    if ((sense[7] & 0x3F) == 0x01) {
// operation code is WRITE DATA -> data area orientation
    pub 0x81: LO_data[0] =,
    } else if ((sense[7] & 0x3F) == 0x03) {
// operation code is FORMAT WRITE -> index orientation
    pub 0xC3: LO_data[0] =,
    } else {
    pub /: *mut *mut LO_data[0] = sense[7]; / operation,
    }
    pub /: *mut *mut LO_data[1] = sense[8]; / auxiliary,
    pub sense: [LO_data[2] =; 9],
    pub /: *mut *mut LO_data[3] = sense[3]; / count,
    pub /: *mut *mut LO_data[4] = sense[29]; / seek_addr.cyl,
    pub /: *mut *mut LO_data[5] = sense[30]; / seek_addr.cyl 2nd byte,
    pub /: *mut *mut LO_data[7] = sense[31]; / seek_addr.head 2nd byte,
    pub 8): memcpy(&(LO_data[8]), &(sense[11]),,
// create DE ccw
    pub erp->cpaddr: ccw =,
    pub ccw1)): memset(ccw, 0, sizeof(struct,
    pub DASD_ECKD_CCW_DEFINE_EXTENT: ccw->cmd_code =,
    pub CCW_FLAG_CC: ccw->flags =,
    pub 16: ccw->count =,
    pub virt_to_dma32(DE_data): ccw->cda =,
// create LO ccw
    pub ccw1)): memset(ccw, 0, sizeof(struct,
    pub DASD_ECKD_CCW_LOCATE_RECORD: ccw->cmd_code =,
    pub CCW_FLAG_CC: ccw->flags =,
    pub 16: ccw->count =,
    pub virt_to_dma32(LO_data): ccw->cda =,
// TIC to the failed ccw
    pub CCW_CMD_TIC: ccw->cmd_code =,
    pub cpa: ccw->cda =,
// fill erp related fields
    pub default_erp->flags: erp->flags =,
    pub dasd_3990_erp_action_1B_32: erp->function =,
    pub default_erp->refers: erp->refers =,
    pub device: erp->startdev =,
    pub device: erp->memdev =,
    pub default_erp->magic: erp->magic =,
    pub default_erp->expires: erp->expires =,
    pub 256: erp->retries =,
    pub get_tod_clock(): erp->buildclk =,
    pub DASD_CQR_FILLED: erp->status =,
// remove the default erp
    pub device): dasd_free_erp_request(default_erp,,
    pub erp: return,
    }				/* end dasd_3990_erp_action_1B_32 */
//
// DASD_3990_UPDATE_1B
//
// DESCRIPTION
// Handles the update to the 32 byte 'Action 1B' of Single Program
// Action Codes in case the first action was not successful.
// The already created 'previous_erp' is the currently not successful
// ERP.
//
// PARAMETER
// previous_erp	already created previous erp.
// sense		current sense data
// RETURN VALUES
// erp		modified erp
//
    static struct dasd_ccw_req *
    dasd_3990_update_1B(struct dasd_ccw_req * previous_erp, char *sense)
    {
    pub previous_erp->startdev: *mut *mut dasd_device device =,
    pub 0: dma32_t cpa =,
    pub cqr: *mut dasd_ccw_req,
    pub erp: *mut dasd_ccw_req,
    pub /: *mut *mut *mut char LO_data; / struct LO_eckd_data,
    pub ccw: *mut ccw1,
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Write not finished because of unexpected condition"
    pub on"): " - follow,
// determine the original cqr
    pub previous_erp: cqr =,
    while (cqr.refers != core::ptr::null_mut()) {
    pub cqr->refers: cqr =,
    }
    if (scsw_is_tm(&cqr.irb.scsw)) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "32 bit sense, action 1B, update,"
    pub retry"): " in transport mode - just,
    pub previous_erp: return,
    }
// for imprecise ending just do default erp
    if (sense[1] & 0x01) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub retry"): "Imprecise ending is set - just,
    pub DASD_CQR_FILLED: previous_erp->status =,
    pub previous_erp: return,
    }
// determine the address of the CCW to be restarted
// Imprecise ending is not set -> addr from IRB-SCSW
    pub previous_erp->irb.scsw.cmd.cpa: cpa =,
    if (cpa == 0) {
    dev_err(&device.cdev.dev,
    pub CCW\n"): "Unable to determine address of to be restarted,
    pub DASD_CQR_FAILED: previous_erp->status =,
    pub previous_erp: return,
    }
    pub previous_erp: erp =,
// update the LO with the new returned sense data
    pub DE_eckd_data): LO_data = erp->data + sizeof(struct,
    if ((sense[3] == 0x01) && (LO_data[1] & 0x01)) {
// should not happen
    pub DASD_CQR_FAILED: previous_erp->status =,
    pub previous_erp: return,
    }
    if ((sense[7] & 0x3F) == 0x01) {
// operation code is WRITE DATA -> data area orientation
    pub 0x81: LO_data[0] =,
    } else if ((sense[7] & 0x3F) == 0x03) {
// operation code is FORMAT WRITE -> index orientation
    pub 0xC3: LO_data[0] =,
    } else {
    pub /: *mut *mut LO_data[0] = sense[7]; / operation,
    }
    pub /: *mut *mut LO_data[1] = sense[8]; / auxiliary,
    pub sense: [LO_data[2] =; 9],
    pub /: *mut *mut LO_data[3] = sense[3]; / count,
    pub /: *mut *mut LO_data[4] = sense[29]; / seek_addr.cyl,
    pub /: *mut *mut LO_data[5] = sense[30]; / seek_addr.cyl 2nd byte,
    pub /: *mut *mut LO_data[7] = sense[31]; / seek_addr.head 2nd byte,
    pub 8): memcpy(&(LO_data[8]), &(sense[11]),,
// TIC to the failed ccw
    pub /: *mut *mut ccw = erp->cpaddr; / addr of DE ccw,
    pub /: *mut *mut ccw++; / addr of LE ccw,
    pub /: *mut *mut ccw++; / addr of TIC ccw,
    pub cpa: ccw->cda =,
    pub DASD_CQR_FILLED: erp->status =,
    pub erp: return,
    }				/* end dasd_3990_update_1B */
//
// DASD_3990_ERP_COMPOUND_RETRY
//
// DESCRIPTION
// Handles the compound ERP action retry code.
// NOTE: At least one retry is done even if zero is specified
// by the sense data. This makes enqueueing of the request
// easier.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created ERP
//
// RETURN VALUES
// erp		modified ERP pointer
//
    static void
    dasd_3990_erp_compound_retry(struct dasd_ccw_req * erp, char *sense)
    {
    switch (sense[25] & 0x03) {
    case 0x00:		/* no not retry */
    pub 1: erp->retries =,
    case 0x01:		/* retry 2 times */
    pub 2: erp->retries =,
    case 0x02:		/* retry 10 times */
    pub 10: erp->retries =,
    case 0x03:		/* retry 256 times */
    pub 256: erp->retries =,
    default:
    }
    pub dasd_3990_erp_compound_retry: erp->function =,
    }				/* end dasd_3990_erp_compound_retry */
//
// DASD_3990_ERP_COMPOUND_PATH
//
// DESCRIPTION
// Handles the compound ERP action for retry on alternate
// channel path.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created ERP
//
// RETURN VALUES
// erp		modified ERP pointer
//
    static void
    dasd_3990_erp_compound_path(struct dasd_ccw_req * erp, char *sense)
    {
    if (sense[25] & DASD_SENSE_BIT_3) {
    if (erp.status == DASD_CQR_FAILED &&
    !test_bit(DASD_CQR_VERIFY_PATH, &erp.flags)) {
// reset the lpm and the status to be able to
// try further actions.
    pub dasd_path_get_opm(erp->startdev): erp->lpm =,
    pub DASD_CQR_NEED_ERP: erp->status =,
    }
    }
    pub dasd_3990_erp_compound_path: erp->function =,
    }				/* end dasd_3990_erp_compound_path */
//
// DASD_3990_ERP_COMPOUND_CODE
//
// DESCRIPTION
// Handles the compound ERP action for retry code.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created ERP
//
// RETURN VALUES
// erp		NEW ERP pointer
//
    static struct dasd_ccw_req *
    dasd_3990_erp_compound_code(struct dasd_ccw_req * erp, char *sense)
    {
    if (sense[25] & DASD_SENSE_BIT_2) {
    switch (sense[28]) {
    case 0x17:
// issue a Diagnostic Control command with an
// Inhibit Write subcommand and controller modifier
    pub 0x20): erp = dasd_3990_erp_DCTL(erp,,
    case 0x25:
// wait for 5 seconds and retry again
    pub 1: erp->retries =,
    pub 5*HZ): *mut dasd_3990_erp_block_queue (erp,,
    default:
// should not happen - continue
    }
    }
    pub dasd_3990_erp_compound_code: erp->function =,
    pub erp: return,
    }				/* end dasd_3990_erp_compound_code */
//
// DASD_3990_ERP_COMPOUND_CONFIG
//
// DESCRIPTION
// Handles the compound ERP action for configuration
// dependent error.
// Note: duplex handling is not implemented (yet).
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created ERP
//
// RETURN VALUES
// erp		modified ERP pointer
//
    static void
    dasd_3990_erp_compound_config(struct dasd_ccw_req * erp, char *sense)
    {
    if ((sense[25] & DASD_SENSE_BIT_1) && (sense[26] & DASD_SENSE_BIT_2)) {
    pub erp->startdev: *mut *mut dasd_device device =,
    dev_err(&device.cdev.dev,
    pub occurred\n"): "Compound configuration error,
    }
    pub dasd_3990_erp_compound_config: erp->function =,
    }				/* end dasd_3990_erp_compound_config */
//
// DASD_3990_ERP_COMPOUND
//
// DESCRIPTION
// Does the further compound program action if
// compound retry was not successful.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the current (failed) ERP
//
// RETURN VALUES
// erp		(additional) ERP pointer
//
    static struct dasd_ccw_req *
    dasd_3990_erp_compound(struct dasd_ccw_req * erp, char *sense)
    {
    if ((erp.function == dasd_3990_erp_compound_retry) &&
    (erp.status == DASD_CQR_NEED_ERP)) {
    pub sense): dasd_3990_erp_compound_path(erp,,
    }
    if ((erp.function == dasd_3990_erp_compound_path) &&
    (erp.status == DASD_CQR_NEED_ERP)) {
    pub sense): erp = dasd_3990_erp_compound_code(erp,,
    }
    if ((erp.function == dasd_3990_erp_compound_code) &&
    (erp.status == DASD_CQR_NEED_ERP)) {
    pub sense): dasd_3990_erp_compound_config(erp,,
    }
// if no compound action ERP specified, the request failed
    if (erp.status == DASD_CQR_NEED_ERP)
    pub DASD_CQR_FAILED: erp->status =,
    pub erp: return,
    }				/* end dasd_3990_erp_compound */
//
// DASD_3990_ERP_HANDLE_SIM
//
// DESCRIPTION
// inspects the SIM SENSE data and starts an appropriate action
//
// PARAMETER
// sense	   sense data of the actual error
//
// RETURN VALUES
// none
//
    void
    dasd_3990_erp_handle_sim(struct dasd_device *device, char *sense)
    {
// print message according to log or message to operator mode
    if ((sense[24] & DASD_SIM_MSG_TO_OP) || (sense[1] & 0x10)) {
// print SIM SRC from RefCode
    dev_err(&device.cdev.dev, "SIM - SRC: "
    "%02x%02x%02x%02x\n", sense[22],
    pub sense[12]): sense[23], sense[11],,
    } else if (sense[24] & DASD_SIM_LOG) {
// print SIM SRC Refcode
    dev_warn(&device.cdev.dev, "log SIM - SRC: "
    "%02x%02x%02x%02x\n", sense[22],
    pub sense[12]): sense[23], sense[11],,
    }
    }
//
// DASD_3990_ERP_INSPECT_32
//
// DESCRIPTION
// Does a detailed inspection of the 32 byte sense data
// and sets up a related error recovery action.
//
// PARAMETER
// sense		sense data of the actual error
// erp		pointer to the currently created default ERP
//
// RETURN VALUES
// erp_filled		pointer to the ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_inspect_32(struct dasd_ccw_req * erp, char *sense)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_3990_erp_inspect_32: erp->function =,
// check for SIM sense data
    if ((sense[6] & DASD_SIM_SENSE) == DASD_SIM_SENSE)
    pub sense): dasd_3990_erp_handle_sim(device,,
    if (sense[25] & DASD_SENSE_BIT_0) {
// compound program action codes (byte25 bit 0 == '1')
    pub sense): dasd_3990_erp_compound_retry(erp,,
    } else {
// single program action codes (byte25 bit 0 == '0')
    switch (sense[25]) {
    case 0x00:	/* success - use default ERP for retries */
    DBF_DEV_EVENT(DBF_DEBUG, device, "%s",
    "ERP called for successful request"
    pub retry"): " - just,
    case 0x01:	/* fatal error */
    dev_err(&device.cdev.dev,
    pub DASD\n"): "ERP failed for the,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    case 0x02:	/* intervention required */
    case 0x03:	/* intervention required during dual copy */
    pub dasd_3990_erp_int_req(erp): erp =,
    case 0x0F:
    dev_err(&device.cdev.dev,
    pub occurred\n"): "Update write command error,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    case 0x10:  /* logging required for other channel program */
    pub sense): erp = dasd_3990_erp_action_10_32(erp,,
    case 0x15:
    dev_err(&device.cdev.dev,
    pub occurred\n"): "Track outside defined extent error,
    pub DASD_CQR_FAILED): erp = dasd_3990_erp_cleanup(erp,,
    case 0x1B:	/* unexpected condition during write */
    pub sense): erp = dasd_3990_erp_action_1B_32(erp,,
    case 0x1C:	/* invalid data */
    dev_emerg(&device.cdev.dev,
    "Data recovered during retry with PCI "
    pub active\n"): "fetch mode,
// not possible to handle this situation in Linux
    panic
    ("Invalid data - No way to inform application "
    pub data"): "about the possibly incorrect,
    case 0x1D:	/* state-change pending */
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "A State change pending condition exists "
    pub device"): "for the subsystem or,
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    case 0x1E:	/* busy */
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    "Busy condition exists "
    pub device"): "for the subsystem or,
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    default:	/* all others errors - default erp  */
    }
    }
    pub erp: return,
    }				/* end dasd_3990_erp_inspect_32 */
#[no_mangle]
unsafe extern "C" fn dasd_3990_erp_disable_path(device: *mut dasd_device, lpum: __u8) {
    static void dasd_3990_erp_disable_path(struct dasd_device *device, __u8 lpum)
    {
    pub pathmask_to_pos(lpum): int pos =,
    if (!(device.features & DASD_FEATURE_PATH_AUTODISABLE)) {
    dev_err(&device.cdev.dev,
    "Path %x.%02x (pathmask %02x) is operational despite excessive IFCCs\n",
    pub lpum): device->path[pos].cssid, device->path[pos].chpid,,
    pub out: goto,
    }
// no remaining path, cannot disable
    if (!(dasd_path_get_opm(device) & ~lpum)) {
    dev_err(&device.cdev.dev,
    "Last path %x.%02x (pathmask %02x) is operational despite excessive IFCCs\n",
    pub lpum): device->path[pos].cssid, device->path[pos].chpid,,
    pub out: goto,
    }
    dev_err(&device.cdev.dev,
    "Path %x.%02x (pathmask %02x) is disabled - IFCC threshold exceeded\n",
    pub lpum): device->path[pos].cssid, device->path[pos].chpid,,
    pub lpum): dasd_path_remove_opm(device,,
    pub lpum): dasd_path_add_ifccpm(device,,
    out:
    pub 0: device->path[pos].errorclk =,
    pub 0): atomic_set(&device->path[pos].error_count,,
    }
#[no_mangle]
unsafe extern "C" fn dasd_3990_erp_account_error(erp: *mut dasd_ccw_req) {
    static void dasd_3990_erp_account_error(struct dasd_ccw_req *erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub erp->refers->irb.esw.esw1.lpum: __u8 lpum =,
    pub pathmask_to_pos(lpum): int pos =,
    pub clk: c_ulong,
    if (!device.path_thrhld)
    pub get_tod_clock(): clk =,
//
// check if the last error is longer ago than the timeout,
// if so reset error state
//
    if ((tod_to_ns(clk - device.path[pos].errorclk) / NSEC_PER_SEC)
    >= device.path_interval) {
    pub 0): atomic_set(&device->path[pos].error_count,,
    pub 0: device->path[pos].errorclk =,
    }
    pub clk: device->path[pos].errorclk =,
// threshold exceeded disable path if possible
    if (atomic_read(&device.path[pos].error_count) >=
    device.path_thrhld)
    pub lpum): dasd_3990_erp_disable_path(device,,
    }
//
// main ERP control functions (24 and 32 byte sense)
//
// DASD_3990_ERP_CONTROL_CHECK
//
// DESCRIPTION
// Does a generic inspection if a control check occurred and sets up
// the related error recovery procedure
//
// PARAMETER
// erp		pointer to the currently created default ERP
//
// RETURN VALUES
// erp_filled		pointer to the erp
//
    static struct dasd_ccw_req *
    dasd_3990_erp_control_check(struct dasd_ccw_req *erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    if (scsw_cstat(&erp.refers.irb.scsw) & (SCHN_STAT_INTF_CTRL_CHK
    | SCHN_STAT_CHN_CTRL_CHK)) {
    DBF_DEV_EVENT(DBF_WARNING, device, "%s",
    pub check"): "channel or interface control,
    pub NULL): erp = dasd_3990_erp_action_4(erp,,
    }
    pub erp: return,
    }
//
// DASD_3990_ERP_INSPECT
//
// DESCRIPTION
// Does a detailed inspection for sense data by calling either
// the 24-byte or the 32-byte inspection routine.
//
// PARAMETER
// erp		pointer to the currently created default ERP
// RETURN VALUES
// erp_new		contens was possibly modified
//
    static struct dasd_ccw_req *
    dasd_3990_erp_inspect(struct dasd_ccw_req *erp)
    {
    pub NULL: *mut *mut dasd_ccw_req erp_new =,
    pub sense: *mut c_char,
// if this problem occurred on an alias retry on base
    pub dasd_3990_erp_inspect_alias(erp): erp_new =,
    if (erp_new)
    pub erp_new: return,
// sense data are located in the refers record of the
// already set up new ERP !
// check if concurrent sens is available
//
    pub dasd_get_sense(&erp->refers->irb): sense =,
    if (!sense)
    pub dasd_3990_erp_control_check(erp): erp_new =,
// distinguish between 24 and 32 byte sense data
#[no_mangle]
pub unsafe extern "C" fn if(DASD_SENSE_BIT_0: sense[27] &) -> else {
// inspect the 24 byte sense data
    pub sense): erp_new = dasd_3990_erp_inspect_24(erp,,
    } else {
// inspect the 32 byte sense data
    pub sense): erp_new = dasd_3990_erp_inspect_32(erp,,
    }	/* end distinguish between 24 and 32 byte sense data */
    pub erp_new: return,
    }
//
// DASD_3990_ERP_ADD_ERP
//
// DESCRIPTION
// This function adds an additional request block (ERP) to the head of
// the given cqr (or erp).
// For a command mode cqr the erp is initialized as an default erp
// (retry TIC).
// For transport mode we make a copy of the original TCW (points to
// the original TCCB, TIDALs, etc.) but give it a fresh
// TSB so the original sense data will not be changed.
//
// PARAMETER
// cqr		head of the current ERP-chain (or single cqr if
// first error)
// RETURN VALUES
// erp		pointer to new ERP-chain head
//
    static struct dasd_ccw_req *dasd_3990_erp_add_erp(struct dasd_ccw_req *cqr)
    {
    pub cqr->startdev: *mut *mut dasd_device device =,
    pub ccw: *mut ccw1,
    pub erp: *mut dasd_ccw_req,
    pub datasize: int cplength,,
    pub tcw: *mut tcw,
    pub tsb: *mut tsb,
    if (cqr.cpmode == 1) {
    pub 0: cplength =,
// TCW needs to be 64 byte aligned, so leave enough room
    pub tsb): datasize = 64 + sizeof(struct tcw) + sizeof(struct,
    } else {
    pub 2: cplength =,
    pub 0: datasize =,
    }
// allocate additional request block
    erp = dasd_alloc_erp_request(cqr.magic,
    pub device): cplength, datasize,,
    if (IS_ERR(erp)) {
    if (cqr.retries <= 0) {
    DBF_DEV_EVENT(DBF_ERR, device, "%s",
    pub request"): "Unable to allocate ERP,
    pub DASD_CQR_FAILED: cqr->status =,
    pub get_tod_clock(): cqr->stopclk =,
    } else {
    DBF_DEV_EVENT(DBF_ERR, device,
    "Unable to allocate ERP request "
    "(%i retries left)",
    pub 3)): dasd_block_set_timer(device->block, (HZ <<,
    }
    pub erp: return,
    }
    pub cqr->cpaddr: ccw =,
    if (cqr.cpmode == 1) {
// make a shallow copy of the original tcw but set new tsb
    pub 1: erp->cpmode =,
    pub 64): erp->cpaddr = PTR_ALIGN(erp->data,,
    pub erp->cpaddr: tcw =,
    pub &tcw[1]: *mut *mut tsb = (struct tsb ),
// tcw = *((struct tcw *)cqr->cpaddr);
    pub virt_to_dma64(tsb): tcw->tsb =,
    } else if (ccw.cmd_code == DASD_ECKD_CCW_PSF) {
// PSF cannot be chained from NOOP/TIC
    pub cqr->cpaddr: erp->cpaddr =,
    } else {
// initialize request with default TIC to current ERP/CQR
    pub erp->cpaddr: ccw =,
    pub CCW_CMD_NOOP: ccw->cmd_code =,
    pub CCW_FLAG_CC: ccw->flags =,
    pub CCW_CMD_TIC: ccw->cmd_code =,
    pub virt_to_dma32(cqr->cpaddr): ccw->cda =,
    }
    pub cqr->flags: erp->flags =,
    pub dasd_3990_erp_add_erp: erp->function =,
    pub cqr: erp->refers =,
    pub device: erp->startdev =,
    pub device: erp->memdev =,
    pub cqr->block: erp->block =,
    pub cqr->filldata: erp->filldata =,
    pub cqr->magic: erp->magic =,
    pub cqr->expires: erp->expires =,
    pub device->default_retries: erp->retries =,
    pub get_tod_clock(): erp->buildclk =,
    pub DASD_CQR_FILLED: erp->status =,
    pub erp: return,
    }
//
// DASD_3990_ERP_ADDITIONAL_ERP
//
// DESCRIPTION
// An additional ERP is needed to handle the current error.
// Add ERP to the head of the ERP-chain containing the ERP processing
// determined based on the sense data.
//
// PARAMETER
// cqr		head of the current ERP-chain (or single cqr if
// first error)
//
// RETURN VALUES
// erp		pointer to new ERP-chain head
//
    static struct dasd_ccw_req *
    dasd_3990_erp_additional_erp(struct dasd_ccw_req * cqr)
    {
    pub NULL: *mut *mut dasd_ccw_req erp =,
// add erp and initialize with default TIC
    pub dasd_3990_erp_add_erp(cqr): erp =,
    if (IS_ERR(erp))
    pub erp: return,
// inspect sense, determine specific ERP if possible
    if (erp != cqr) {
    pub dasd_3990_erp_inspect(erp): erp =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_additional_erp */
//
// DASD_3990_ERP_ERROR_MATCH
//
// DESCRIPTION
// Check if the device status of the given cqr is the same.
// This means that the failed CCW and the relevant sense data
// must match.
// I don't distinguish between 24 and 32 byte sense because in case of
// 24 byte sense byte 25 and 27 is set as well.
//
// PARAMETER
// cqr1		first cqr, which will be compared with the
// cqr2		second cqr.
//
// RETURN VALUES
// match		'boolean' for match found
// returns 1 if match found, otherwise 0.
//
    static int dasd_3990_erp_error_match(struct dasd_ccw_req *cqr1,
    struct dasd_ccw_req *cqr2)
    {
    pub sense2: *mut *mut char sense1,,
    if (cqr1.startdev != cqr2.startdev)
    pub 0: return,
    pub dasd_get_sense(&cqr1->irb): sense1 =,
    pub dasd_get_sense(&cqr2->irb): sense2 =,
// one request has sense data, the other not -> no match, return 0
    if (!sense1 != !sense2)
    pub 0: return,
// no sense data in both cases -> check cstat for IFCC
    if (!sense1 && !sense2)	{
    if ((scsw_cstat(&cqr1.irb.scsw) & (SCHN_STAT_INTF_CTRL_CHK |
    SCHN_STAT_CHN_CTRL_CHK)) ==
    (scsw_cstat(&cqr2.irb.scsw) & (SCHN_STAT_INTF_CTRL_CHK |
    SCHN_STAT_CHN_CTRL_CHK)))
    pub ifcc*/: *mut *mut return 1; / match with,
    }
// check sense data; byte 0-2,25,27
    if (!(sense1 && sense2 &&
    (memcmp(sense1, sense2, 3) == 0) &&
    (sense1[27] == sense2[27]) &&
    (sense1[25] == sense2[25]))) {
    pub /: *mut *mut return 0; / sense doesn't match,
    }
    pub /: *mut *mut return 1; / match,
    }				/* end dasd_3990_erp_error_match */
//
// DASD_3990_ERP_IN_ERP
//
// DESCRIPTION
// check if the current error already happened before.
// quick exit if current cqr is not an ERP (cqr->refers=NULL)
//
// PARAMETER
// cqr		failed cqr (either original cqr or already an erp)
//
// RETURN VALUES
// erp		erp-pointer to the already defined error
// recovery procedure OR
// NULL if a 'new' error occurred.
//
    static struct dasd_ccw_req *
    dasd_3990_erp_in_erp(struct dasd_ccw_req *cqr)
    {
    struct dasd_ccw_req *erp_head = cqr,	/* save erp chain head */
// erp_match = NULL;	/* save erp chain head
    pub /: *mut *mut int match = 0; / 'boolean' for matching error found,
    if (cqr.refers == core::ptr::null_mut()) {	/* return if not in erp */
    pub NULL: return,
    }
// check the erp/cqr chain for current error
    do {
    pub cqr->refers): match = dasd_3990_erp_error_match(erp_head,,
    pub /: *mut *mut erp_match = cqr; / save possible matching erp,
    pub /: *mut *mut cqr = cqr->refers; / check next erp/cqr in queue,
    pub (!match)): } while ((cqr->refers != NULL) &&,
    if (!match) {
    pub /: *mut *mut return NULL; / no match was found,
    }
    pub /: *mut *mut return erp_match; / return address of matching erp,
    }				/* END dasd_3990_erp_in_erp */
//
// DASD_3990_ERP_FURTHER_ERP (24 & 32 byte sense)
//
// DESCRIPTION
// No retry is left for the current ERP. Check what has to be done
// with the ERP.
// - do further defined ERP action or
// - wait for interrupt or
// - exit with permanent error
//
// PARAMETER
// erp		ERP which is in progress with no retry left
//
// RETURN VALUES
// erp		modified/additional ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_further_erp(struct dasd_ccw_req *erp)
    {
    pub erp->startdev: *mut *mut dasd_device device =,
    pub dasd_get_sense(&erp->irb): *mut *mut char sense =,
// check for 24 byte sense ERP
    if ((erp.function == dasd_3990_erp_bus_out) ||
    (erp.function == dasd_3990_erp_action_1) ||
    (erp.function == dasd_3990_erp_action_4)) {
    pub dasd_3990_erp_action_1(erp): erp =,
    } else if (erp.function == dasd_3990_erp_action_1_sec) {
    pub dasd_3990_erp_action_1_sec(erp): erp =,
    } else if (erp.function == dasd_3990_erp_action_5) {
// retries have not been successful
// prepare erp for retry on different channel path
    pub dasd_3990_erp_action_1(erp): erp =,
    if (sense && !(sense[2] & DASD_SENSE_BIT_0)) {
// issue a Diagnostic Control command with an
// Inhibit Write subcommand
    switch (sense[25]) {
    case 0x17:
    case 0x57:{	/* controller */
    pub 0x20): erp = dasd_3990_erp_DCTL(erp,,
    }
    case 0x18:
    case 0x58:{	/* channel path */
    pub 0x40): erp = dasd_3990_erp_DCTL(erp,,
    }
    case 0x19:
    case 0x59:{	/* storage director */
    pub 0x80): erp = dasd_3990_erp_DCTL(erp,,
    }
    default:
    DBF_DEV_EVENT(DBF_WARNING, device,
    "invalid subcommand modifier 0x%x "
    "for Diagnostic Control Command",
    }
    }
// check for 32 byte sense ERP
    } else if (sense &&
    ((erp.function == dasd_3990_erp_compound_retry) ||
    (erp.function == dasd_3990_erp_compound_path) ||
    (erp.function == dasd_3990_erp_compound_code) ||
    (erp.function == dasd_3990_erp_compound_config))) {
    pub sense): erp = dasd_3990_erp_compound(erp,,
    } else {
//
// No retry left and no additional special handling
// necessary
//
    dev_err(&device.cdev.dev,
    pub erp): "ERP %px has run out of retries and failed\n",,
    pub DASD_CQR_FAILED: erp->status =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_further_erp */
//
// DASD_3990_ERP_HANDLE_MATCH_ERP
//
// DESCRIPTION
// An error occurred again and an ERP has been detected which is already
// used to handle this error (e.g. retries).
// All prior ERP's are asumed to be successful and therefore removed
// from queue.
// If retry counter of matching erp is already 0, it is checked if further
// action is needed (besides retry) or if the ERP has failed.
//
// PARAMETER
// erp_head		first ERP in ERP-chain
// erp		ERP that handles the actual error.
// (matching erp)
//
// RETURN VALUES
// erp		modified/additional ERP
//
    static struct dasd_ccw_req *
    dasd_3990_erp_handle_match_erp(struct dasd_ccw_req *erp_head,
    struct dasd_ccw_req *erp)
    {
    pub erp_head->startdev: *mut *mut dasd_device device =,
    pub /: *mut *mut *mut dasd_ccw_req erp_done = erp_head; / finished req,
    pub /: *mut *mut *mut dasd_ccw_req erp_free = NULL; / req to be freed,
// loop over successful ERPs and remove them from chanq
    while (erp_done != erp) {
    if (erp_done == core::ptr::null_mut())	/* end of chain reached */
    pub lost\n"): panic("Programming error in ERP! The original request was,
// remove the request from the device queue
    pub erp_done: erp_free =,
    pub erp_done->refers: erp_done =,
// free the finished erp request
    pub erp_free->memdev): dasd_free_erp_request(erp_free,,
    }			/* end while */
    if (erp.retries > 0) {
    pub dasd_get_sense(&erp->refers->irb): *mut *mut char sense =,
// check for special retries
    if (sense && erp.function == dasd_3990_erp_action_4) {
    pub sense): erp = dasd_3990_erp_action_4(erp,,
    } else if (sense &&
    erp.function == dasd_3990_erp_action_1B_32) {
    pub sense): erp = dasd_3990_update_1B(erp,,
    } else if (sense && erp.function == dasd_3990_erp_int_req) {
    pub dasd_3990_erp_int_req(erp): erp =,
    } else {
// simple retry
    DBF_DEV_EVENT(DBF_DEBUG, device,
    "%i retries left for erp %p",
    pub erp): erp->retries,,
// handle the request again...
    pub DASD_CQR_FILLED: erp->status =,
    }
    } else {
// no retry left - check for further necessary action
// if no further actions, handle rest as permanent error
    pub dasd_3990_erp_further_erp(erp): erp =,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_handle_match_erp */
//
// DASD_3990_ERP_ACTION
//
// DESCRIPTION
// control routine for 3990 erp actions.
// Has to be called with the queue lock (namely the s390_irq_lock) acquired.
//
// PARAMETER
// cqr		failed cqr (either original cqr or already an erp)
//
// RETURN VALUES
// erp		erp-pointer to the head of the ERP action chain.
// This means:
// - either a ptr to an additional ERP cqr or
// - the original given cqr (which's status might
// be modified)
//
    struct dasd_ccw_req *
    dasd_3990_erp_action(struct dasd_ccw_req * cqr)
    {
    pub NULL: *mut *mut dasd_ccw_req erp =,
    pub cqr->startdev: *mut *mut dasd_device device =,
    pub NULL: *mut *mut dasd_ccw_req temp_erp =,
    if (device.features & DASD_FEATURE_ERPLOG) {
// print current erp_chain
    dev_err(&device.cdev.dev,
    pub ERP-ACTION\n"): "ERP chain at BEGINNING of,
    pub cqr: for (temp_erp =,
    pub {: temp_erp != NULL; temp_erp = temp_erp->refers),
    dev_err(&device.cdev.dev,
    "ERP %px (%02x) refers to %px\n",
    pub temp_erp->refers): temp_erp, temp_erp->status,,
    }
    }
// double-check if current erp/cqr was successful
    if ((scsw_cstat(&cqr.irb.scsw) == 0x00) &&
    (scsw_dstat(&cqr.irb.scsw) ==
    (DEV_STAT_CHN_END | DEV_STAT_DEV_END))) {
    DBF_DEV_EVENT(DBF_DEBUG, device,
    "ERP called for successful request %p"
    pub cqr): " - NO ERP necessary",,
    pub DASD_CQR_DONE: cqr->status =,
    pub cqr: return,
    }
// check if error happened before
    pub dasd_3990_erp_in_erp(cqr): erp =,
    if (erp == core::ptr::null_mut()) {
// no matching erp found - set up erp
    pub dasd_3990_erp_additional_erp(cqr): erp =,
    if (IS_ERR(erp))
    pub erp: return,
    } else {
// matching erp found - set all leading erp's to DONE
    pub erp): erp = dasd_3990_erp_handle_match_erp(cqr,,
    }
//
// For path verification work we need to stick with the path that was
// originally chosen so that the per path configuration data is
// assigned correctly.
//
    if (test_bit(DASD_CQR_VERIFY_PATH, &erp.flags) && cqr.lpm) {
    pub cqr->lpm: erp->lpm =,
    }
    if (device.features & DASD_FEATURE_ERPLOG) {
// print current erp_chain
    dev_err(&device.cdev.dev,
    pub ERP-ACTION\n"): "ERP chain at END of,
    pub erp: for (temp_erp =,
    pub {: temp_erp != NULL; temp_erp = temp_erp->refers),
    dev_err(&device.cdev.dev,
    "ERP %px (%02x) refers to %px\n",
    pub temp_erp->refers): temp_erp, temp_erp->status,,
    }
    }
// enqueue ERP request if it's a new one
    if (list_empty(&erp.blocklist)) {
    pub DASD_CQR_IN_ERP: cqr->status =,
// add erp request before the cqr
    pub &cqr->blocklist): list_add_tail(&erp->blocklist,,
    }
    pub erp: return,
    }				/* end dasd_3990_erp_action */
