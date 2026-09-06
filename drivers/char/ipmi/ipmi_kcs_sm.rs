//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_kcs_sm.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// ipmi_kcs_sm.c
//
// State machine for handling IPMI KCS interfaces.
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//
// This state machine is taken from the state machine in the IPMI spec,
// pretty much verbatim.  If you have questions about the states, see
// that document.
//

// kcs_debug is a bit-field
// KCS_DEBUG_ENABLE -	turned on for now
// KCS_DEBUG_MSG    -	commands and their responses
// KCS_DEBUG_STATES -	state machine
//
pub const KCS_DEBUG_STATES: c_int = 4;
pub const KCS_DEBUG_MSG: c_int = 2;
pub const KCS_DEBUG_ENABLE: c_int = 1;
    static int kcs_debug;
    module_param(kcs_debug, int, 0644);
    MODULE_PARM_DESC(kcs_debug, "debug bitmask, 1=enable, 2=messages, 4=states");
// The states the KCS driver may be in.
    enum kcs_states {
// The KCS interface is currently doing nothing.
    KCS_IDLE,
//
// We are starting an operation.  The data is in the output
// buffer, but nothing has been done to the interface yet.  This
// was added to the state machine in the spec to wait for the
// initial IBF.
//
    KCS_START_OP,
// We have written a write cmd to the interface.
    KCS_WAIT_WRITE_START,
// We are writing bytes to the interface.
    KCS_WAIT_WRITE,
//
// We have written the write end cmd to the interface, and
// still need to write the last byte.
//
    KCS_WAIT_WRITE_END,
// We are waiting to read data from the interface.
    KCS_WAIT_READ,
//
// State to transition to the error handler, this was added to
// the state machine in the spec to be sure IBF was there.
//
    KCS_ERROR0,
//
// First stage error handler, wait for the interface to
// respond.
//
    KCS_ERROR1,
//
// The abort cmd has been written, wait for the interface to
// respond.
//
    KCS_ERROR2,
//
// We wrote some data to the interface, wait for it to switch
// to read mode.
//
    KCS_ERROR3,
// The hardware failed to follow the state machine.
    KCS_HOSED
    };

// Timeouts in microseconds.

pub const MAX_ERROR_RETRIES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_sm_data {
    pub state: enum kcs_states,
    pub io: *mut si_sm_io,
    pub write_data: [c_uchar; MAX_KCS_WRITE_SIZE],
    pub write_pos: c_int,
    pub write_count: c_int,
    pub orig_write_count: c_int,
    pub read_data: [c_uchar; MAX_KCS_READ_SIZE],
    pub read_pos: c_int,
    pub truncated: c_int,
    pub error_retries: c_uint,
    pub ibf_timeout: c_long,
    pub obf_timeout: c_long,
    pub error0_timeout: c_ulong,
}

    static unsigned int init_kcs_data(struct si_sm_data *kcs,
    struct si_sm_io *io)
    {
    kcs.state = KCS_IDLE;
    kcs.io = io;
    kcs.write_pos = 0;
    kcs.write_count = 0;
    kcs.orig_write_count = 0;
    kcs.read_pos = 0;
    kcs.error_retries = 0;
    kcs.truncated = 0;
    kcs.ibf_timeout = IBF_RETRY_TIMEOUT;
    kcs.obf_timeout = OBF_RETRY_TIMEOUT;
// Reserve 2 I/O bytes.
    return 2;
    }
#[no_mangle]
pub unsafe extern "C" fn read_status(kcs: *mut si_sm_data) -> c_uchar {
    static inline unsigned char read_status(struct si_sm_data *kcs)
    {
    return kcs.io.inputb(kcs.io, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn read_data(kcs: *mut si_sm_data) -> c_uchar {
    static inline unsigned char read_data(struct si_sm_data *kcs)
    {
    return kcs.io.inputb(kcs.io, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn write_cmd(kcs: *mut si_sm_data, data: c_uchar) {
    static inline void write_cmd(struct si_sm_data *kcs, unsigned char data)
    {
    kcs.io.outputb(kcs.io, 1, data);
    }
#[no_mangle]
pub unsafe extern "C" fn write_data(kcs: *mut si_sm_data, data: c_uchar) {
    static inline void write_data(struct si_sm_data *kcs, unsigned char data)
    {
    kcs.io.outputb(kcs.io, 0, data);
    }
// Control codes.
pub const KCS_GET_STATUS_ABORT: c_uint = 0x60;
pub const KCS_WRITE_START: c_uint = 0x61;
pub const KCS_WRITE_END: c_uint = 0x62;
pub const KCS_READ_BYTE: c_uint = 0x68;
// Status bits.

pub const KCS_IDLE_STATE: c_int = 0;
pub const KCS_READ_STATE: c_int = 1;
pub const KCS_WRITE_STATE: c_int = 2;
pub const KCS_ERROR_STATE: c_int = 3;

#[no_mangle]
pub unsafe extern "C" fn write_next_byte(kcs: *mut si_sm_data) {
    static inline void write_next_byte(struct si_sm_data *kcs)
    {
    write_data(kcs, kcs.write_data[kcs.write_pos]);
    (kcs.write_pos)++;
    (kcs.write_count)--;
    }
#[no_mangle]
pub unsafe extern "C" fn start_error_recovery(kcs: *mut si_sm_data, reason: *mut c_char) {
    static inline void start_error_recovery(struct si_sm_data *kcs, char *reason)
    {
    (kcs.error_retries)++;
    if (kcs.error_retries > MAX_ERROR_RETRIES) {
    if (kcs_debug & KCS_DEBUG_ENABLE)
    dev_dbg(kcs.io.dev, "ipmi_kcs_sm: kcs hosed: %s\n",
    reason);
    kcs.state = KCS_HOSED;
    } else {
    kcs.error0_timeout = jiffies + ERROR0_OBF_WAIT_JIFFIES;
    kcs.state = KCS_ERROR0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn read_next_byte(kcs: *mut si_sm_data) {
    static inline void read_next_byte(struct si_sm_data *kcs)
    {
    if (kcs.read_pos >= MAX_KCS_READ_SIZE) {
// Throw the data away and mark it truncated.
    read_data(kcs);
    kcs.truncated = 1;
    } else {
    kcs.read_data[kcs.read_pos] = read_data(kcs);
    (kcs.read_pos)++;
    }
    write_data(kcs, KCS_READ_BYTE);
    }
    static inline int check_ibf(struct si_sm_data *kcs, unsigned char status,
    long time)
    {
    if (GET_STATUS_IBF(status)) {
    kcs.ibf_timeout -= time;
    if (kcs.ibf_timeout < 0) {
    start_error_recovery(kcs, "IBF not ready in time");
    kcs.ibf_timeout = IBF_RETRY_TIMEOUT;
    return 1;
    }
    return 0;
    }
    kcs.ibf_timeout = IBF_RETRY_TIMEOUT;
    return 1;
    }
    static inline int check_obf(struct si_sm_data *kcs, unsigned char status,
    long time)
    {
    if (!GET_STATUS_OBF(status)) {
    kcs.obf_timeout -= time;
    if (kcs.obf_timeout < 0) {
    kcs.obf_timeout = OBF_RETRY_TIMEOUT;
    start_error_recovery(kcs, "OBF not ready in time");
    return 1;
    }
    return 0;
    }
    kcs.obf_timeout = OBF_RETRY_TIMEOUT;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn clear_obf(kcs: *mut si_sm_data, status: c_uchar) {
    static void clear_obf(struct si_sm_data *kcs, unsigned char status)
    {
    if (GET_STATUS_OBF(status))
    read_data(kcs);
    }
#[no_mangle]
unsafe extern "C" fn restart_kcs_transaction(kcs: *mut si_sm_data) {
    static void restart_kcs_transaction(struct si_sm_data *kcs)
    {
    kcs.write_count = kcs.orig_write_count;
    kcs.write_pos = 0;
    kcs.read_pos = 0;
    kcs.state = KCS_WAIT_WRITE_START;
    kcs.ibf_timeout = IBF_RETRY_TIMEOUT;
    kcs.obf_timeout = OBF_RETRY_TIMEOUT;
    write_cmd(kcs, KCS_WRITE_START);
    }
    static int start_kcs_transaction(struct si_sm_data *kcs, unsigned char *data,
    unsigned int size)
    {
    unsigned int i;
    if (size < 2)
    return IPMI_REQ_LEN_INVALID_ERR;
    if (size > MAX_KCS_WRITE_SIZE)
    return IPMI_REQ_LEN_EXCEEDED_ERR;
    if ((kcs.state != KCS_IDLE) && (kcs.state != KCS_HOSED)) {
    dev_warn(kcs.io.dev, "KCS in invalid state %d\n", kcs.state);
    return IPMI_NOT_IN_MY_STATE_ERR;
    }
    if (kcs_debug & KCS_DEBUG_MSG) {
    dev_dbg(kcs.io.dev, "%s -", __func__);
    for (i = 0; i < size; i++)
    pr_cont(" %02x", data[i]);
    pr_cont("\n");
    }
    kcs.error_retries = 0;
    memcpy(kcs.write_data, data, size);
    kcs.write_count = size;
    kcs.orig_write_count = size;
    kcs.write_pos = 0;
    kcs.read_pos = 0;
    kcs.state = KCS_START_OP;
    kcs.ibf_timeout = IBF_RETRY_TIMEOUT;
    kcs.obf_timeout = OBF_RETRY_TIMEOUT;
    return 0;
    }
    static int get_kcs_result(struct si_sm_data *kcs, unsigned char *data,
    unsigned int length)
    {
    if (length < kcs.read_pos) {
    kcs.read_pos = length;
    kcs.truncated = 1;
    }
    memcpy(data, kcs.read_data, kcs.read_pos);
    if ((length >= 3) && (kcs.read_pos < 3)) {
// Guarantee that we return at least 3 bytes, with an
    error in the third byte if it is too short. */
    data[2] = IPMI_ERR_UNSPECIFIED;
    kcs.read_pos = 3;
    }
    if (kcs.truncated) {
//
// Report a truncated error.  We might overwrite
// another error, but that's too bad, the user needs
// to know it was truncated.
//
    data[2] = IPMI_ERR_MSG_TRUNCATED;
    kcs.truncated = 0;
    }
    return kcs.read_pos;
    }
//
// This implements the state machine defined in the IPMI manual, see
// that for details on how this works.  Divide that flowchart into
// sections delimited by "Wait for IBF" and this will become clear.
//
#[no_mangle]
unsafe extern "C" fn kcs_event(kcs: *mut si_sm_data, time: c_long) -> enum si_sm_result {
    static enum si_sm_result kcs_event(struct si_sm_data *kcs, long time)
    {
    unsigned char status;
    unsigned char state;
    status = read_status(kcs);
    if (kcs_debug & KCS_DEBUG_STATES)
    dev_dbg(kcs.io.dev,
    "KCS: State = %d, %x\n", kcs.state, status);
// All states wait for ibf, so just do it here.
    if (!check_ibf(kcs, status, time))
    return SI_SM_CALL_WITH_DELAY;
// Just about everything looks at the KCS state, so grab that, too.
    state = GET_STATUS_STATE(status);
    switch (kcs.state) {
    case KCS_IDLE:
// If there's and interrupt source, turn it off.
    clear_obf(kcs, status);
    if (GET_STATUS_ATN(status))
    return SI_SM_ATTN;
    else
    return SI_SM_IDLE;
    case KCS_START_OP:
    if (state != KCS_IDLE_STATE) {
    start_error_recovery(kcs,
    "State machine not idle at start");
    break;
    }
    clear_obf(kcs, status);
    write_cmd(kcs, KCS_WRITE_START);
    kcs.state = KCS_WAIT_WRITE_START;
    break;
    case KCS_WAIT_WRITE_START:
    if (state != KCS_WRITE_STATE) {
    start_error_recovery(
    kcs,
    "Not in write state at write start");
    break;
    }
    read_data(kcs);
    if (kcs.write_count == 1) {
    write_cmd(kcs, KCS_WRITE_END);
    kcs.state = KCS_WAIT_WRITE_END;
    } else {
    write_next_byte(kcs);
    kcs.state = KCS_WAIT_WRITE;
    }
    break;
    case KCS_WAIT_WRITE:
    if (state != KCS_WRITE_STATE) {
    start_error_recovery(kcs,
    "Not in write state for write");
    break;
    }
    clear_obf(kcs, status);
    if (kcs.write_count == 1) {
    write_cmd(kcs, KCS_WRITE_END);
    kcs.state = KCS_WAIT_WRITE_END;
    } else {
    write_next_byte(kcs);
    }
    break;
    case KCS_WAIT_WRITE_END:
    if (state != KCS_WRITE_STATE) {
    start_error_recovery(kcs,
    "Not in write state"
    " for write end");
    break;
    }
    clear_obf(kcs, status);
    write_next_byte(kcs);
    kcs.state = KCS_WAIT_READ;
    break;
    case KCS_WAIT_READ:
    if ((state != KCS_READ_STATE) && (state != KCS_IDLE_STATE)) {
    start_error_recovery(
    kcs,
    "Not in read or idle in read state");
    break;
    }
    if (state == KCS_READ_STATE) {
    if (!check_obf(kcs, status, time))
    return SI_SM_CALL_WITH_DELAY;
    read_next_byte(kcs);
    } else {
//
// We don't implement this exactly like the state
// machine in the spec.  Some broken hardware
// does not write the final dummy byte to the
// read register.  Thus obf will never go high
// here.  We just go straight to idle, and we
// handle clearing out obf in idle state if it
// happens to come in.
//
    clear_obf(kcs, status);
    kcs.orig_write_count = 0;
    kcs.state = KCS_IDLE;
    return SI_SM_TRANSACTION_COMPLETE;
    }
    break;
    case KCS_ERROR0:
    clear_obf(kcs, status);
    status = read_status(kcs);
    if (GET_STATUS_OBF(status))
// controller isn't responding
    if (time_before(jiffies, kcs.error0_timeout))
    return SI_SM_CALL_WITH_TICK_DELAY;
    write_cmd(kcs, KCS_GET_STATUS_ABORT);
    kcs.state = KCS_ERROR1;
    break;
    case KCS_ERROR1:
    clear_obf(kcs, status);
    write_data(kcs, 0);
    kcs.state = KCS_ERROR2;
    break;
    case KCS_ERROR2:
    if (state != KCS_READ_STATE) {
    start_error_recovery(kcs,
    "Not in read state for error2");
    break;
    }
    if (!check_obf(kcs, status, time))
    return SI_SM_CALL_WITH_DELAY;
    clear_obf(kcs, status);
    write_data(kcs, KCS_READ_BYTE);
    kcs.state = KCS_ERROR3;
    break;
    case KCS_ERROR3:
    if (state != KCS_IDLE_STATE) {
    start_error_recovery(kcs,
    "Not in idle state for error3");
    break;
    }
    if (!check_obf(kcs, status, time))
    return SI_SM_CALL_WITH_DELAY;
    clear_obf(kcs, status);
    if (kcs.orig_write_count) {
    restart_kcs_transaction(kcs);
    } else {
    kcs.state = KCS_IDLE;
    return SI_SM_TRANSACTION_COMPLETE;
    }
    break;
    case KCS_HOSED:
    break;
    }
    if (kcs.state == KCS_HOSED) {
    init_kcs_data(kcs, kcs.io);
    return SI_SM_HOSED;
    }
    return SI_SM_CALL_WITHOUT_DELAY;
    }
#[no_mangle]
unsafe extern "C" fn kcs_size() -> c_int {
    static int kcs_size(void)
    {
    return sizeof(struct si_sm_data);
    }
#[no_mangle]
unsafe extern "C" fn kcs_detect(kcs: *mut si_sm_data) -> c_int {
    static int kcs_detect(struct si_sm_data *kcs)
    {
//
// It's impossible for the KCS status register to be all 1's,
// (assuming a properly functioning, self-initialized BMC)
// but that's what you get from reading a bogus address, so we
// test that first.
//
    if (read_status(kcs) == 0xff)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kcs_cleanup(kcs: *mut si_sm_data) {
    static void kcs_cleanup(struct si_sm_data *kcs)
    {
    }
    const struct si_sm_handlers kcs_smi_handlers = {
    .init_data         = init_kcs_data,
    .start_transaction = start_kcs_transaction,
    .get_result        = get_kcs_result,
    .event             = kcs_event,
    .detect            = kcs_detect,
    .cleanup           = kcs_cleanup,
    .size              = kcs_size,
    };
