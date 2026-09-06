//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_fw1.h
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
// Device driver for the SYMBIOS/LSILOGIC 53C8XX and 53C1010 family
// of PCI-SCSI IO processors.
//
// Copyright (C) 1999-2001  Gerard Roudier <groudier@free.fr>
//
// This driver is derived from the Linux sym53c8xx driver.
// Copyright (C) 1998-2000  Gerard Roudier
//
// The sym53c8xx driver is derived from the ncr53c8xx driver that had been
// a port of the FreeBSD ncr driver to Linux-1.2.13.
//
// The original ncr driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
// Copyright (C) 1994  Wolfgang Stanglmeier
//
// Other major contributions:
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// -----------------------------------------------------------------------------
//
// Scripts for SYMBIOS-Processor
//
// We have to know the offsets of all labels before we reach
// them (for forward jumps). Therefore we declare a struct
// here. If you make changes inside the script,
//
// DONT FORGET TO CHANGE THE LENGTHS HERE!
//
// Script fragments which are loaded into the on-chip RAM
// of 825A, 875, 876, 895, 895A, 896 and 1010 chips.
// Must not exceed 4K bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SYM_FWA_SCR {
    pub 11]: u32 start [,
    pub 4]: u32 getjob_begin [,
    pub 5]: u32 _sms_a10 [,
    pub 4]: u32 getjob_end [,
    pub 4]: u32 _sms_a20 [,

    pub 8]: u32 select [,

    pub 6]: u32 select [,

    pub 5]: u32 _sms_a30 [,
    pub 2]: u32 wf_sel_done [,
    pub 2]: u32 send_ident [,

    pub 8]: u32 select2 [,

    pub 2]: u32 select2 [,

    pub 2]: u32 command [,
    pub 28]: u32 dispatch [,
    pub 10]: u32 sel_no_cmd [,
    pub 6]: u32 init [,
    pub 4]: u32 clrack [,
    pub 11]: u32 datai_done [,
    pub 20]: u32 datai_done_wsr [,
    pub 11]: u32 datao_done [,
    pub 6]: u32 datao_done_wss [,
    pub 5]: u32 datai_phase [,
    pub 5]: u32 datao_phase [,
    pub 2]: u32 msg_in [,
    pub 10]: u32 msg_in2 [,

    pub 14]: u32 status [,

    pub 10]: u32 status [,

    pub 6]: u32 complete [,
    pub 8]: u32 complete2 [,
    pub 12]: u32 _sms_a40 [,
    pub 5]: u32 done [,
    pub 5]: u32 _sms_a50 [,
    pub 2]: u32 _sms_a60 [,
    pub 4]: u32 done_end [,
    pub 5]: u32 complete_error [,
    pub 11]: u32 save_dp [,
    pub 7]: u32 restore_dp [,
    pub 11]: u32 disconnect [,
    pub 5]: u32 disconnect2 [,
    pub 3]: u32 _sms_a65 [,

    pub 4]: u32 idle [,

    pub 2]: u32 idle [,

    pub 7]: u32 ungetjob [,

    pub 5]: u32 ungetjob [,

    pub 4]: u32 reselect [,

    pub 2]: u32 reselect [,

    pub 19]: u32 reselected [,
    pub 6]: u32 _sms_a70 [,
    pub 4]: u32 _sms_a80 [,
    pub 25]: u32 reselected1 [,
    pub 4]: u32 _sms_a90 [,
    pub 7]: u32 resel_lun0 [,
    pub 4]: u32 _sms_a100 [,
    pub 8]: u32 resel_tag [,

    pub 23]: u32 _sms_a110 [,

    pub 17]: u32 _sms_a110 [,

    pub 13]: u32 _sms_a110 [,

    pub 2]: u32 _sms_a120 [,
    pub 4]: u32 resel_go [,
    pub 7]: u32 _sms_a130 [,
    pub 2]: u32 resel_dsa [,
    pub 4]: u32 resel_dsa1 [,
    pub 7]: u32 _sms_a140 [,
    pub 4]: u32 resel_no_tag [,
    pub 7]: u32 _sms_a145 [,
    pub 2]: *mut *mut u32 data_in [SYM_CONF_MAX_SG,
    pub 4]: u32 data_in2 [,
    pub 2]: *mut *mut u32 data_out [SYM_CONF_MAX_SG,
    pub 4]: u32 data_out2 [,
    pub 12]: u32 pm0_data [,
    pub 6]: u32 pm0_data_out [,
    pub 7]: u32 pm0_data_end [,
    pub 4]: u32 pm_data_end [,
    pub 4]: u32 _sms_a150 [,
    pub 12]: u32 pm1_data [,
    pub 6]: u32 pm1_data_out [,
    pub 9]: u32 pm1_data_end [,
}

//
// Script fragments which stay in main memory for all chips
// except for chips that support 8K on-chip RAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SYM_FWB_SCR {
    pub 2]: u32 no_data [,

    pub 18]: u32 sel_for_abort [,

    pub 16]: u32 sel_for_abort [,

    pub 2]: u32 sel_for_abort_1 [,
    pub 12]: u32 msg_in_etc [,
    pub 5]: u32 msg_received [,
    pub 5]: u32 msg_weird_seen [,
    pub 17]: u32 msg_extended [,
    pub 4]: u32 _sms_b10 [,
    pub 6]: u32 msg_bad [,
    pub 4]: u32 msg_weird [,
    pub 8]: u32 msg_weird1 [,
    pub 6]: u32 wdtr_resp [,
    pub 4]: u32 send_wdtr [,
    pub 6]: u32 sdtr_resp [,
    pub 4]: u32 send_sdtr [,
    pub 6]: u32 ppr_resp [,
    pub 4]: u32 send_ppr [,
    pub 4]: u32 nego_bad_phase [,
    pub 4]: u32 msg_out [,
    pub 4]: u32 msg_out_done [,
    pub 3]: u32 data_ovrun [,
    pub 22]: u32 data_ovrun1 [,
    pub 8]: u32 data_ovrun2 [,
    pub 16]: u32 abort_resel [,
    pub 4]: u32 resend_ident [,
    pub 4]: u32 ident_break [,
    pub 4]: u32 ident_break_atn [,
    pub 6]: u32 sdata_in [,
    pub 4]: u32 resel_bad_lun [,
    pub 4]: u32 bad_i_t_l [,
    pub 4]: u32 bad_i_t_l_q [,
    pub 7]: u32 bad_status [,
    pub 4]: u32 wsr_ma_helper [,
// Data area
    pub 1]: u32 zero [,
    pub 1]: u32 scratch [,
    pub 1]: u32 scratch1 [,
    pub 1]: u32 prev_done [,
    pub 1]: u32 done_pos [,
    pub 1]: u32 nextjob [,
    pub 1]: u32 startpos [,
    pub 1]: u32 targtbl [,
}

//
// Script fragments used at initialisations.
// Only runs out of main memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SYM_FWZ_SCR {
    pub 9]: u32 snooptest [,
    pub 2]: u32 snoopend [,
}

// --------------------------< START >----------------------------*/ {
//
// Switch the LED on.
// Will be patched with a NO_OP if LED
// not needed or not desired.
//
// Clear SIGP.
//
// Stop here if the C code wants to perform
// some error recovery procedure manually.
// (Indicate this by setting SEM in ISTAT)
//
// Report to the C code the next position in
// the start queue the SCRIPTS will schedule.
// The C code must not change SCRATCHA.
//
// Start the next job.
//
// @DSA     = start point for this job.
// SCRATCHA = address of this job in the start queue.
//
// We will restore startpos with SCRATCHA if we fails the
// arbitration or if it is the idle job.
//
// The below GETJOB_BEGIN to GETJOB_END section of SCRIPTS
// is a critical path. If it is partially executed, it then
// may happen that the job address is not yet in the DSA
// and the next queue position points to the next JOB.
//
// Copy to a fixed location both the next STARTPOS
// and the current JOB address, using self modifying
// SCRIPTS.
//
// Move the start address to TEMP using self-
// modifying SCRIPTS and jump indirectly to
// that address.
//
// DSA	contains the address of a scheduled
// data structure.
//
// SCRATCHA contains the address of the start queue
// entry which points to the next job.
//
// Set Initiator mode.
//
// (Target mode is left as an exercise for the reader)
//

//
// And try to select this target.
//
// Now there are 4 possibilities:
//
// (1) The chip loses arbitration.
// This is ok, because it will try again,
// when the bus becomes idle.
// (But beware of the timeout function!)
//
// (2) The chip is reselected.
// Then the script processor takes the jump
// to the RESELECT label.
//
// (3) The chip wins arbitration.
// Then it will execute SCRIPTS instruction until
// the next instruction that checks SCSI phase.
// Then will stop and wait for selection to be
// complete or selection time-out to occur.
//
// After having won arbitration, the SCRIPTS
// processor is able to execute instructions while
// the SCSI core is performing SCSI selection.
//
// Copy the CCB header to a fixed location
// in the HCB using self-modifying SCRIPTS.
//
// Initialize the status register
//
// Selection complete.
// Send the IDENTIFY and possibly the TAG message
// and negotiation message if present.
//

//
// Set IMMEDIATE ARBITRATION if we have been given
// a hint to do so. (Some job to do after this one).
//

//
// Anticipate the COMMAND phase.
// This is the PHASE we expect at this point.
//
// ... and send the command
//
// MSG_IN is the only phase that shall be
// entered at least once for each (re)selection.
// So we test it first.
//
// Discard as many illegal phases as
// required and tell the C code about.
//
// The target does not switch to command
// phase after IDENTIFY has been sent.
//
// If it stays in MSG OUT phase send it
// the IDENTIFY again.
//
// If target does not switch to MSG IN phase
// and we sent a negotiation, assert the
// failure immediately.
//
// Jump to dispatcher.
//
// Wait for the SCSI RESET signal to be
// inactive before restarting operations,
// since the chip may hang on SEL_ATN
// if SCSI RESET is active.
//
// Terminate possible pending message phase.
//
// Save current pointer to LASTP.
//
// If the SWIDE is not full, jump to dispatcher.
// We anticipate a STATUS phase.
//
// The SWIDE is full.
// Clear this condition.
//
// We are expecting an IGNORE RESIDUE message
// from the device, otherwise we are in data
// overrun condition. Check against MSG_IN phase.
//
// We are in MSG_IN phase,
// Read the first byte of the message.
// If it is not an IGNORE RESIDUE message,
// signal overrun and jump to message
// processing.
//
// We got the message we expected.
// Read the 2nd byte, and jump to dispatcher.
//
// Save current pointer to LASTP.
//
// If the SODL is not full jump to dispatcher.
// We anticipate a STATUS phase.
//
// The SODL is full, clear this condition.
//
// And signal a DATA UNDERRUN condition
// to the C code.
//
// Jump to current pointer.
//
// Jump to current pointer.
//
// Get the first byte of the message.
//
// The script processor doesn't negate the
// ACK signal after this transfer.
//
// Check first against 1 byte messages
// that we handle from SCRIPTS.
//
// We handle all other messages from the
// C code, so no need to waste on-chip RAM
// for those ones.
//
// get the status
//

//
// If STATUS is not GOOD, clear IMMEDIATE ARBITRATION,
// since we may have to tamper the start queue from
// the C code.
//

//
// save status to scsi_status.
// mark as complete.
//
// Anticipate the MESSAGE PHASE for
// the TASK COMPLETE message.
//
// Complete message.
//
// When we terminate the cycle by clearing ACK,
// the target may disconnect immediately.
//
// We don't want to be told of an "unexpected disconnect",
// so we disable this feature.
//
// Terminate cycle ...
//
// ... and wait for the disconnect.
//
// Save host status.
//
// Move back the CCB header using self-modifying
// SCRIPTS.
//
// Some bridges may reorder DMA writes to memory.
// We donnot want the CPU to deal with completions
// without all the posted write having been flushed
// to memory. This DUMMY READ should flush posted
// buffers prior to the CPU having to deal with
// completions.
//
// If command resulted in not GOOD status,
// call the C code if needed.
//
// If we performed an auto-sense, call
// the C code to synchronyze task aborts
// with UNIT ATTENTION conditions.
//
// Copy the DSA to the DONE QUEUE and
// signal completion to the host.
// If we are interrupted between DONE
// and DONE_END, we must reset, otherwise
// the completed CCB may be lost.
//
// The instruction below reads the DONE QUEUE next
// free position from memory.
// In addition it ensures that all PCI posted writes
// are flushed and so the DSA value of the done
// CCB is visible by the CPU before INTFLY is raised.
//
// Clear ACK immediately.
// No need to delay it.
//
// Keep track we received a SAVE DP, so
// we will switch to the other PM context
// on the next PM since the DP may point
// to the current PM context.
//
// SAVE_DP message:
// Copy LASTP to SAVEP.
//
// Anticipate the MESSAGE PHASE for
// the DISCONNECT message.
//
// Clear ACK immediately.
// No need to delay it.
//
// Copy SAVEP to LASTP.
//
// DISCONNECTing  ...
//
// disable the "unexpected disconnect" feature,
// and remove the ACK signal.
//
// Wait for the disconnect.
//
// Status is: DISCONNECTED.
//
// Save host status.
//
// Move back the CCB header using self-modifying
// SCRIPTS.
//
// Nothing to do?
// Switch the LED off and wait for reselect.
// Will be patched with a NO_OP if LED
// not needed or not desired.
//

//
// Set IMMEDIATE ARBITRATION, for the next time.
// This will give us better chance to win arbitration
// for the job we just wanted to do.
//

//
// We are not able to restart the SCRIPTS if we are
// interrupted and these instruction haven't been
// all executed. BTW, this is very unlikely to
// happen, but we check that from the C code.
//

//
// Make sure we are in initiator mode.
//

//
// Sleep waiting for a reselection.
//
// Switch the LED on.
// Will be patched with a NO_OP if LED
// not needed or not desired.
//
// load the target id into the sdid
//
// Load the target control block address
//
// Copy the TCB header to a fixed place in
// the HCB.
//
// We expect MESSAGE IN phase.
// If not, get help from the C code.
//
// Load the synchronous transfer registers.
//
// Get the IDENTIFY message.
//
// If IDENTIFY LUN #0, use a faster path
// to find the LCB structure.
//
// If message isn't an IDENTIFY,
// tell the C code about.
//
// It is an IDENTIFY message,
// Load the LUN control block address.
//
// LUN 0 special case (but usual one :))
//
// Jump indirectly to the reselect action for this LUN.
// (lcb.head.resel_sa assumed at offset zero of lcb).
//
// In normal situations, we jump to RESEL_TAG or RESEL_NO_TAG
//
// ACK the IDENTIFY previously received.
//
// It shall be a tagged command.
// Read SIMPLE+TAG.
// The C code will deal with errors.
// Aggressive optimization, isn't it? :)
//
// Copy the LCB header to a fixed place in
// the HCB using self-modifying SCRIPTS.
//
// Load the pointer to the tagged task
// table for this LUN.
//
// The SIDL still contains the TAG value.
// Aggressive optimization, isn't it? :):)
//

//
// Retrieve the DSA of this task.
// JUMP indirectly to the restart point of the CCB.
//
// Move 'ccb.phys.head.go' action to
// scratch/scratch1. So scratch1 will
// contain the 'restart' field of the
// 'go' structure.
//
// In normal situations we branch to RESEL_DSA
//
// ACK the IDENTIFY or TAG previously received.
//
// Copy the CCB header to a fixed location
// in the HCB using self-modifying SCRIPTS.
//
// Initialize the status register
//
// Jump to dispatcher.
//
// Copy the LCB header to a fixed place in
// the HCB using self-modifying SCRIPTS.
//
// Load the DSA with the unique ITL task.
//
// Because the size depends on the
// #define SYM_CONF_MAX_SG parameter,
// it is filled in at runtime.
//
// ##===========< i=0; i<SYM_CONF_MAX_SG >=========
// ||	SCR_CHMOV_TBL ^ SCR_DATA_IN,
// ||		offsetof (struct sym_dsb, data[ i]),
// ##==========================================
//
// Because the size depends on the
// #define SYM_CONF_MAX_SG parameter,
// it is filled in at runtime.
//
// ##===========< i=0; i<SYM_CONF_MAX_SG >=========
// ||	SCR_CHMOV_TBL ^ SCR_DATA_OUT,
// ||		offsetof (struct sym_dsb, data[ i]),
// ##==========================================
//
// Read our host flags to SFBR, so we will be able
// to check against the data direction we expect.
//
// Check against actual DATA PHASE.
//
// Actual phase is DATA IN.
// Check against expected direction.
//
// Keep track we are moving data from the
// PM0 DATA mini-script.
//
// Move the data to memory.
//
// Actual phase is DATA OUT.
// Check against expected direction.
//
// Keep track we are moving data from the
// PM0 DATA mini-script.
//
// Move the data from memory.
//
// Clear the flag that told we were moving
// data from the PM0 DATA mini-script.
//
// Return to the previous DATA script which
// is guaranteed by design (if no bug) to be
// the main DATA script for this transfer.
//
// Read our host flags to SFBR, so we will be able
// to check against the data direction we expect.
//
// Check against actual DATA PHASE.
//
// Actual phase is DATA IN.
// Check against expected direction.
//
// Keep track we are moving data from the
// PM1 DATA mini-script.
//
// Move the data to memory.
//
// Actual phase is DATA OUT.
// Check against expected direction.
//
// Keep track we are moving data from the
// PM1 DATA mini-script.
//
// Move the data from memory.
//
// Clear the flag that told we were moving
// data from the PM1 DATA mini-script.
//
// Return to the previous DATA script which
// is guaranteed by design (if no bug) to be
// the main DATA script for this transfer.
//
// -------------------------< NO_DATA >--------------------------*/ {
//
// We are jumped here by the C code, if we have
// some target to reset or some disconnected
// job to abort. Since error recovery is a serious
// busyness, we will really reset the SCSI BUS, if
// case of a SCSI interrupt occurring in this path.
//

//
// Set initiator mode.
//

//
// And try to select this target.
//
// Wait for the selection to complete or
// the selection to time out.
//
// Call the C code.
//
// The C code should let us continue here.
// Send the 'kiss of death' message.
// We expect an immediate disconnect once
// the target has eaten the message.
//
// Tell the C code that we are done.
//
// Jump at scheduler.
//
// If it is an EXTENDED (variable size message)
// Handle it.
//
// Let the C code handle any other
// 1 byte message.
//
// We donnot handle 2 bytes messages from SCRIPTS.
// So, let the C code deal with these ones too.
//
// Clear ACK and get the next byte
// assumed to be the message length.
//
// Try to catch some unlikely situations as 0 length
// or too large the length.
//
// We donnot handle extended messages from SCRIPTS.
// Read the amount of data corresponding to the
// message length and call the C code.
//
// unimplemented message - reject it.
//
// weird message received
// ignore all MSG IN phases and reject it.
//
// let the target fetch our answer.
//
// Send the M_X_WIDE_REQ
//
// let the target fetch our answer.
//
// Send the M_X_SYNC_REQ
//
// let the target fetch our answer.
//
// Send the M_X_PPR_REQ
//
// The target requests a message.
// We donnot send messages that may
// require the device to go to bus free.
//
// ... wait for the next phase
// if it's a message out, send it again, ...
//
// Let the C code be aware of the
// sent message and clear the message.
//
// ... and process the next phase
//
// Zero scratcha that will count the
// extras bytes.
//
// The target may want to transfer too much data.
//
// If phase is DATA OUT write 1 byte and count it.
//
// If WSR is set, clear this condition, and
// count this byte.
//
// Finally check against DATA IN phase.
// Signal data overrun to the C code
// and jump to dispatcher if not so.
// Read 1 byte otherwise and count it.
//
// Count this byte.
// This will allow to return a negative
// residual to user.
//
// .. and repeat as required.
//
// send the abort/abortag/reset message
// we expect an immediate disconnect
//
// The target stays in MSG OUT phase after having acked
// Identify [+ Tag [+ Extended message ]]. Targets shall
// behave this way on parity error.
// We must send it again all the messages.
//
// Message is an IDENTIFY, but lun is unknown.
// Signal problem to C code for logging the event.
// Send a M_ABORT to clear all pending tasks.
//
// We donnot have a task for that I_T_L.
// Signal problem to C code for logging the event.
// Send a M_ABORT message.
//
// We donnot have a task that matches the tag.
// Signal problem to C code for logging the event.
// Send a M_ABORTTAG message.
//
// Anything different from INTERMEDIATE
// CONDITION MET should be a bad SCSI status,
// given that GOOD status has already been tested.
// Call the C code.
//
// Helper for the C code when WSR bit is set.
// Perform the move of the residual byte.
//
// -------------------------< SNOOPTEST >------------------------*/{
//
// Read the variable.
//
// Write the variable.
//
// Read back the variable.
//
// And stop.
//
