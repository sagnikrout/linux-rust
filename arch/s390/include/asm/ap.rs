//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ap.h
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
// Adjunct processor (AP) interfaces
//
// Copyright IBM Corp. 2017
//
// Author(s): Tony Krowiak <akrowia@linux.vnet.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Harald Freudenberger <freude@de.ibm.com>
//

//
// The ap_qid_t identifier of an ap queue.
// If the AP facilities test (APFT) facility is available,
// card and queue index are 8 bit values, otherwise
// card index is 6 bit and queue index a 4 bit value.
//
pub type ap_qid_t = c_uint;

//
// struct ap_queue_status - Holds the AP queue status.
// @queue_empty: Shows if queue is empty
// @replies_waiting: Waiting replies
// @queue_full: Is 1 if the queue is full
// @irq_enabled: Shows if interrupts are enabled for the AP
// @response_code: Holds the 8 bit response code
//
// The ap queue status word is returned by all three AP functions
// (PQAP, NQAP and DQAP).  There's a set of flags in the first
// byte, followed by a 1 byte response code.
//
// For convenience the 'value' field is a 32 bit access of the
// whole status and the 'status_bits' and 'rc' fields comprise
// the leftmost 8 status bits and the response_code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_queue_status {
    pub 32: unsigned int value :,
    pub 8: unsigned int status_bits :,
    pub 8: unsigned int rc :,
    pub 16: unsigned int :,
}

//
// AP queue status reg union to access the reg1
// register with the lower 32 bits comprising the
// ap queue status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ap_queue_status_reg {
    pub value: c_ulong,
    pub _pad: u32,
    pub status: ap_queue_status,
}

//
// ap_instructions_available() - Test if AP instructions are available.
//
// Returns true if the AP instructions are installed, otherwise false.
//
// TAPQ register GR2 response struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_tapq_hwinfo {
    pub value: c_ulong,
    pub /: *mut *mut unsigned int fac : 32; / facility bits,
    pub /: *mut *mut unsigned int apinfo : 32; / ap type, ...,
}

//
// Convenience defines to be used with the bs field from struct ap_tapq_gr2
//
pub const AP_BS_Q_USABLE: c_int = 0;
pub const AP_BS_Q_USABLE_NO_SECURE_KEY: c_int = 1;
pub const AP_BS_Q_AVAIL_FOR_BINDING: c_int = 2;
pub const AP_BS_Q_UNUSABLE: c_int = 3;
//
// ap_tapq(): Test adjunct processor queue.
// @qid: The AP queue number
// @info: Pointer to tapq hwinfo struct
//
// Returns AP queue status structure.
//
// ap_test_queue(): Test adjunct processor queue.
// @qid: The AP queue number
// @tbit: Test facilities bit
// @info: Ptr to tapq gr2 struct
//
// Returns AP queue status structure.
//
extern "C" {
    pub fn ap_tapq(_arg: qid, _arg: info) -> return;
}
//
// ap_pqap_rapq(): Reset adjunct processor queue.
// @qid: The AP queue number
// @fbit: if != 0 set F bit
//
// Returns AP queue status structure.
//
// ap_pqap_zapq(): Reset and zeroize adjunct processor queue.
// @qid: The AP queue number
// @fbit: if != 0 set F bit
//
// Returns AP queue status structure.
//
// struct ap_config_info - convenience struct for AP crypto
// config info as returned by the ap_qci() function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_config_info {
    pub flags: c_uint,
    pub /: *mut *mut unsigned int apsc : 1; / S bit,
    pub /: *mut *mut unsigned int apxa : 1; / N bit,
    pub /: *mut *mut unsigned int qact : 1; / C bit,
    pub /: *mut *mut unsigned int rc8a : 1; / R bit,
    pub 4: unsigned int :,
    pub /: *mut *mut unsigned int apsb : 1; / B bit,
    pub 23: unsigned int :,
}

//
// ap_qci(): Get AP configuration data
//
// Returns 0 on success, or -EOPNOTSUPP.
//
// struct ap_qirq_ctrl - convenient struct for easy invocation
// of the ap_aqic() function. This struct is passed as GR1
// parameter to the PQAP(AQIC) instruction. For details please
// see the AR documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ap_qirq_ctrl {
    pub value: c_ulong,
    pub 8: unsigned int :,
    pub /: *mut *mut unsigned int zone : 8; / zone info,
    pub /: *mut *mut unsigned int ir : 1; / ir flag: enable (1) or disable (0) irq,
    pub 4: unsigned int :,
    pub /: *mut *mut unsigned int gisc : 3; / guest isc field,
    pub 6: unsigned int :,
    pub /: *mut *mut unsigned int gf : 2; / gisa format,
    pub 1: unsigned int :,
    pub /: *mut *mut unsigned int gisa : 27; / gisa origin,
    pub 1: unsigned int :,
    pub /: *mut *mut unsigned int isc : 3; / irq sub class,
}

//
// ap_aqic(): Control interruption for a specific AP.
// @qid: The AP queue number
// @qirqctrl: struct ap_qirq_ctrl (64 bit value)
// @pa_ind: Physical address of the notification indicator byte
//
// Returns AP queue status.
//
// union ap_qact_ap_info - used together with the
// ap_aqic() function to provide a convenient way
// to handle the ap info needed by the qact function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ap_qact_ap_info {
    pub val: c_ulong,
    pub 3: unsigned int :,
    pub 3: unsigned int mode :,
    pub 26: unsigned int :,
    pub 8: unsigned int cat :,
    pub 8: unsigned int :,
    pub ver: [c_uchar; 2],
}

//
// ap_qact(): Query AP compatibility type.
// @qid: The AP queue number
// @apinfo: On input the info about the AP queue. On output the
// alternate AP queue info provided by the qact function
// in GR2 is stored in.
//
// Returns AP queue status. Check response_code field for failures.
//
// ap_bapq(): SE bind AP queue.
// @qid: The AP queue number
//
// Returns AP queue status structure.
//
// Invoking this function in a non-SE environment
// may case a specification exception.
//
// ap_aapq(): SE associate AP queue.
// @qid: The AP queue number
// @sec_idx: The secret index
//
// Returns AP queue status structure.
//
// Invoking this function in a non-SE environment
// may case a specification exception.
//
// ap_nqap(): Send message to adjunct processor queue.
// @qid: The AP queue number
// @psmid: The program supplied message identifier
// @msg: The message text
// @length: The message length
//
// Returns AP queue status structure.
// Condition code 1 on NQAP can't happen because the L bit is 1.
// Condition code 2 on NQAP also means the send is incomplete,
// because a segment boundary was reached. The NQAP is repeated.
//
// ap_dqap(): Receive message from adjunct processor queue.
// @qid: The AP queue number
// @psmid: Pointer to program supplied message identifier
// @msg: Pointer to message buffer
// @msglen: Message buffer size
// @length: Pointer to length of actually written bytes
// @reslength: Residual length on return
// @resgr0: input: gr0 value (only used if != 0), output: residual gr0 content
//
// Returns AP queue status structure.
// Condition code 1 on DQAP means the receive has taken place
// but only partially.	The response is incomplete, hence the
// DQAP is repeated.
// Condition code 2 on DQAP also means the receive is incomplete,
// this time because a segment boundary was reached. Again, the
// DQAP is repeated.
// Note that gpr2 is used by the DQAP instruction to keep track of
// any 'residual' length, in case the instruction gets interrupted.
// Hence it gets zeroed before the instruction.
// If the message does not fit into the buffer, this function will
// return with a truncated message and the reply in the firmware queue
// is not removed. This is indicated to the caller with an
// ap_queue_status response_code value of all bits on (0xFF) and (if
// the reslength ptr is given) the remaining length is stored in
// *reslength and (if the resgr0 ptr is given) the updated gr0 value
// for further processing of this msg entry is stored in *resgr0. The
// caller needs to detect this situation and should invoke ap_dqap
// with a valid resgr0 ptr and a value in there != 0 to indicate that
// *resgr0 is to be used instead of qid to further process this entry.
//
// reslength = reg2;
//
// Partially complete, status in gr1 is not set.
// Signal the caller that this dqap is only partially received
// with a special status response code 0xFF and *resgr0 updated
//
// resgr0 = reg0;
// psmid = (rp1.even << 32) + rp1.odd;
// resgr0 = 0;
// update *length with the nr of bytes stored into the msg buffer
// length = msglen - rp2.odd;
