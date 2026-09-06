//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_smic_sm.c
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
// ipmi_smic_sm.c
//
// The state-machine driver for an IPMI SMIC driver
//
// It started as a copy of Corey Minyard's driver for the KSC interface
// and the kernel patch "mmcdev-patch-245" by HP
//
// modified by:	Hannes Schulz <schulz@schwaar.com>
// ipmi@schwaar.com
//
// Corey Minyard's driver for the KSC interface has the following
// copyright notice:
// Copyright 2002 MontaVista Software Inc.
//
// the kernel patch "mmcdev-patch-245" by HP has the following
// copyright notice:
// (c) Copyright 2001 Grant Grundler (c) Copyright
// 2001 Hewlett-Packard Company
//

// smic_debug is a bit-field
// SMIC_DEBUG_ENABLE -	turned on for now
// SMIC_DEBUG_MSG -	commands and their responses
// SMIC_DEBUG_STATES -	state machine
//
pub const SMIC_DEBUG_STATES: c_int = 4;
pub const SMIC_DEBUG_MSG: c_int = 2;
pub const SMIC_DEBUG_ENABLE: c_int = 1;
    let mut smic_debug: static int = 1;
    module_param(smic_debug, int, 0644);
    MODULE_PARM_DESC(smic_debug, "debug bitmask, 1=enable, 2=messages, 4=states");
    enum smic_states {
    SMIC_IDLE,
    SMIC_START_OP,
    SMIC_OP_OK,
    SMIC_WRITE_START,
    SMIC_WRITE_NEXT,
    SMIC_WRITE_END,
    SMIC_WRITE2READ,
    SMIC_READ_START,
    SMIC_READ_NEXT,
    SMIC_READ_END,
    SMIC_HOSED
    };
pub const MAX_SMIC_READ_SIZE: c_int = 80;
pub const MAX_SMIC_WRITE_SIZE: c_int = 80;
pub const SMIC_MAX_ERROR_RETRIES: c_int = 3;
// Timeouts in microseconds.

// SMIC Flags Register Bits
pub const SMIC_RX_DATA_READY: c_uint = 0x80;
pub const SMIC_TX_DATA_READY: c_uint = 0x40;
//
// SMIC_SMI and SMIC_EVM_DATA_AVAIL are only used by
// a few systems, and then only by Systems Management
// Interrupts, not by the OS.  Always ignore these bits.
//
pub const SMIC_SMI: c_uint = 0x10;
pub const SMIC_EVM_DATA_AVAIL: c_uint = 0x08;
pub const SMIC_SMS_DATA_AVAIL: c_uint = 0x04;
pub const SMIC_FLAG_BSY: c_uint = 0x01;
// SMIC Error Codes
pub const EC_NO_ERROR: c_uint = 0x00;
pub const EC_ABORTED: c_uint = 0x01;
pub const EC_ILLEGAL_CONTROL: c_uint = 0x02;
pub const EC_NO_RESPONSE: c_uint = 0x03;
pub const EC_ILLEGAL_COMMAND: c_uint = 0x04;
pub const EC_BUFFER_FULL: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_sm_data {
    pub state: enum smic_states,
    pub io: *mut si_sm_io,
    pub write_data: [c_uchar; MAX_SMIC_WRITE_SIZE],
    pub write_pos: c_int,
    pub write_count: c_int,
    pub orig_write_count: c_int,
    pub read_data: [c_uchar; MAX_SMIC_READ_SIZE],
    pub read_pos: c_int,
    pub truncated: c_int,
    pub error_retries: c_uint,
    pub smic_timeout: c_long,
}

    static unsigned int init_smic_data(struct si_sm_data *smic,
    struct si_sm_io *io)
    {
    smic.state = SMIC_IDLE;
    smic.io = io;
    smic.write_pos = 0;
    smic.write_count = 0;
    smic.orig_write_count = 0;
    smic.read_pos = 0;
    smic.error_retries = 0;
    smic.truncated = 0;
    smic.smic_timeout = SMIC_RETRY_TIMEOUT;
// We use 3 bytes of I/O.
    return 3;
    }
    static int start_smic_transaction(struct si_sm_data *smic,
    unsigned char *data, unsigned int size)
    {
    unsigned int i;
    if (size < 2)
    return IPMI_REQ_LEN_INVALID_ERR;
    if (size > MAX_SMIC_WRITE_SIZE)
    return IPMI_REQ_LEN_EXCEEDED_ERR;
    if ((smic.state != SMIC_IDLE) && (smic.state != SMIC_HOSED)) {
    dev_warn(smic.io.dev,
    "SMIC in invalid state %d\n", smic.state);
    return IPMI_NOT_IN_MY_STATE_ERR;
    }
    if (smic_debug & SMIC_DEBUG_MSG) {
    dev_dbg(smic.io.dev, "%s -", __func__);
    for (i = 0; i < size; i++)
    pr_cont(" %02x", data[i]);
    pr_cont("\n");
    }
    smic.error_retries = 0;
    memcpy(smic.write_data, data, size);
    smic.write_count = size;
    smic.orig_write_count = size;
    smic.write_pos = 0;
    smic.read_pos = 0;
    smic.state = SMIC_START_OP;
    smic.smic_timeout = SMIC_RETRY_TIMEOUT;
    return 0;
    }
    static int smic_get_result(struct si_sm_data *smic,
    unsigned char *data, unsigned int length)
    {
    int i;
    if (smic_debug & SMIC_DEBUG_MSG) {
    dev_dbg(smic.io.dev, "smic_get result -");
    for (i = 0; i < smic.read_pos; i++)
    pr_cont(" %02x", smic.read_data[i]);
    pr_cont("\n");
    }
    if (length < smic.read_pos) {
    smic.read_pos = length;
    smic.truncated = 1;
    }
    memcpy(data, smic.read_data, smic.read_pos);
    if ((length >= 3) && (smic.read_pos < 3)) {
    data[2] = IPMI_ERR_UNSPECIFIED;
    smic.read_pos = 3;
    }
    if (smic.truncated) {
    data[2] = IPMI_ERR_MSG_TRUNCATED;
    smic.truncated = 0;
    }
    return smic.read_pos;
    }
#[no_mangle]
pub unsafe extern "C" fn read_smic_flags(smic: *mut si_sm_data) -> c_uchar {
    static inline unsigned char read_smic_flags(struct si_sm_data *smic)
    {
    return smic.io.inputb(smic.io, 2);
    }
#[no_mangle]
pub unsafe extern "C" fn read_smic_status(smic: *mut si_sm_data) -> c_uchar {
    static inline unsigned char read_smic_status(struct si_sm_data *smic)
    {
    return smic.io.inputb(smic.io, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn read_smic_data(smic: *mut si_sm_data) -> c_uchar {
    static inline unsigned char read_smic_data(struct si_sm_data *smic)
    {
    return smic.io.inputb(smic.io, 0);
    }
    static inline void write_smic_flags(struct si_sm_data *smic,
    unsigned char   flags)
    {
    smic.io.outputb(smic.io, 2, flags);
    }
    static inline void write_smic_control(struct si_sm_data *smic,
    unsigned char   control)
    {
    smic.io.outputb(smic.io, 1, control);
    }
    static inline void write_si_sm_data(struct si_sm_data *smic,
    unsigned char   data)
    {
    smic.io.outputb(smic.io, 0, data);
    }
#[no_mangle]
pub unsafe extern "C" fn start_error_recovery(smic: *mut si_sm_data, reason: *mut c_char) {
    static inline void start_error_recovery(struct si_sm_data *smic, char *reason)
    {
    (smic.error_retries)++;
    if (smic.error_retries > SMIC_MAX_ERROR_RETRIES) {
    if (smic_debug & SMIC_DEBUG_ENABLE)
    pr_warn("ipmi_smic_drv: smic hosed: %s\n", reason);
    smic.state = SMIC_HOSED;
    } else {
    smic.write_count = smic.orig_write_count;
    smic.write_pos = 0;
    smic.read_pos = 0;
    smic.state = SMIC_START_OP;
    smic.smic_timeout = SMIC_RETRY_TIMEOUT;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn write_next_byte(smic: *mut si_sm_data) {
    static inline void write_next_byte(struct si_sm_data *smic)
    {
    write_si_sm_data(smic, smic.write_data[smic.write_pos]);
    (smic.write_pos)++;
    (smic.write_count)--;
    }
#[no_mangle]
pub unsafe extern "C" fn read_next_byte(smic: *mut si_sm_data) {
    static inline void read_next_byte(struct si_sm_data *smic)
    {
    if (smic.read_pos >= MAX_SMIC_READ_SIZE) {
    read_smic_data(smic);
    smic.truncated = 1;
    } else {
    smic.read_data[smic.read_pos] = read_smic_data(smic);
    smic.read_pos++;
    }
    }
// SMIC Control/Status Code Components
pub const SMIC_GET_STATUS: c_uint = 0x00	/* Control form's name */;
pub const SMIC_READY: c_uint = 0x00	/* Status  form's name */;
pub const SMIC_WR_START: c_uint = 0x01	/* Unified Control/Status names... */;
pub const SMIC_WR_NEXT: c_uint = 0x02;
pub const SMIC_WR_END: c_uint = 0x03;
pub const SMIC_RD_START: c_uint = 0x04;
pub const SMIC_RD_NEXT: c_uint = 0x05;
pub const SMIC_RD_END: c_uint = 0x06;
pub const SMIC_CODE_MASK: c_uint = 0x0f;
pub const SMIC_CONTROL: c_uint = 0x00;
pub const SMIC_STATUS: c_uint = 0x80;
pub const SMIC_CS_MASK: c_uint = 0x80;
pub const SMIC_SMS: c_uint = 0x40;
pub const SMIC_SMM: c_uint = 0x60;
pub const SMIC_STREAM_MASK: c_uint = 0x60;
// SMIC Control Codes

// SMIC Status Codes

// these are the control/status codes we actually use
    SMIC_CC_SMS_GET_STATUS	0x40
    SMIC_CC_SMS_WR_START	0x41
    SMIC_CC_SMS_WR_NEXT	0x42
    SMIC_CC_SMS_WR_END	0x43
    SMIC_CC_SMS_RD_START	0x44
    SMIC_CC_SMS_RD_NEXT	0x45
    SMIC_CC_SMS_RD_END	0x46
    SMIC_SC_SMS_READY	0xC0
    SMIC_SC_SMS_WR_START	0xC1
    SMIC_SC_SMS_WR_NEXT	0xC2
    SMIC_SC_SMS_WR_END	0xC3
    SMIC_SC_SMS_RD_START	0xC4
    SMIC_SC_SMS_RD_NEXT	0xC5
    SMIC_SC_SMS_RD_END	0xC6
//
#[no_mangle]
unsafe extern "C" fn smic_event(smic: *mut si_sm_data, time: c_long) -> enum si_sm_result {
    static enum si_sm_result smic_event(struct si_sm_data *smic, long time)
    {
    unsigned char status;
    unsigned char flags;
    unsigned char data;
    if (smic.state == SMIC_HOSED) {
    init_smic_data(smic, smic.io);
    return SI_SM_HOSED;
    }
    if (smic.state != SMIC_IDLE) {
    if (smic_debug & SMIC_DEBUG_STATES)
    dev_dbg(smic.io.dev,
    "%s - smic.smic_timeout = %ld, time = %ld\n",
    __func__, smic.smic_timeout, time);
//
// FIXME: smic_event is sometimes called with time >
// SMIC_RETRY_TIMEOUT
//
    if (time < SMIC_RETRY_TIMEOUT) {
    smic.smic_timeout -= time;
    if (smic.smic_timeout < 0) {
    start_error_recovery(smic, "smic timed out.");
    return SI_SM_CALL_WITH_DELAY;
    }
    }
    }
    flags = read_smic_flags(smic);
    if (flags & SMIC_FLAG_BSY)
    return SI_SM_CALL_WITH_DELAY;
    status = read_smic_status(smic);
    if (smic_debug & SMIC_DEBUG_STATES)
    dev_dbg(smic.io.dev,
    "%s - state = %d, flags = 0x%02x, status = 0x%02x\n",
    __func__, smic.state, flags, status);
    switch (smic.state) {
    case SMIC_IDLE:
// in IDLE we check for available messages
    if (flags & SMIC_SMS_DATA_AVAIL)
    return SI_SM_ATTN;
    return SI_SM_IDLE;
    case SMIC_START_OP:
// sanity check whether smic is really idle
    write_smic_control(smic, SMIC_CC_SMS_GET_STATUS);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_OP_OK;
    break;
    case SMIC_OP_OK:
    if (status != SMIC_SC_SMS_READY) {
// this should not happen
    start_error_recovery(smic,
    "state = SMIC_OP_OK,"
    " status != SMIC_SC_SMS_READY");
    return SI_SM_CALL_WITH_DELAY;
    }
// OK so far; smic is idle let us start ...
    write_smic_control(smic, SMIC_CC_SMS_WR_START);
    write_next_byte(smic);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_WRITE_START;
    break;
    case SMIC_WRITE_START:
    if (status != SMIC_SC_SMS_WR_START) {
    start_error_recovery(smic,
    "state = SMIC_WRITE_START, "
    "status != SMIC_SC_SMS_WR_START");
    return SI_SM_CALL_WITH_DELAY;
    }
//
// we must not issue WR_(NEXT|END) unless
// TX_DATA_READY is set
//
    if (flags & SMIC_TX_DATA_READY) {
    if (smic.write_count == 1) {
// last byte
    write_smic_control(smic, SMIC_CC_SMS_WR_END);
    smic.state = SMIC_WRITE_END;
    } else {
    write_smic_control(smic, SMIC_CC_SMS_WR_NEXT);
    smic.state = SMIC_WRITE_NEXT;
    }
    write_next_byte(smic);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    } else
    return SI_SM_CALL_WITH_DELAY;
    break;
    case SMIC_WRITE_NEXT:
    if (status != SMIC_SC_SMS_WR_NEXT) {
    start_error_recovery(smic,
    "state = SMIC_WRITE_NEXT, "
    "status != SMIC_SC_SMS_WR_NEXT");
    return SI_SM_CALL_WITH_DELAY;
    }
// this is the same code as in SMIC_WRITE_START
    if (flags & SMIC_TX_DATA_READY) {
    if (smic.write_count == 1) {
    write_smic_control(smic, SMIC_CC_SMS_WR_END);
    smic.state = SMIC_WRITE_END;
    } else {
    write_smic_control(smic, SMIC_CC_SMS_WR_NEXT);
    smic.state = SMIC_WRITE_NEXT;
    }
    write_next_byte(smic);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    } else
    return SI_SM_CALL_WITH_DELAY;
    break;
    case SMIC_WRITE_END:
    if (status != SMIC_SC_SMS_WR_END) {
    start_error_recovery(smic,
    "state = SMIC_WRITE_END, "
    "status != SMIC_SC_SMS_WR_END");
    return SI_SM_CALL_WITH_DELAY;
    }
// data register holds an error code
    data = read_smic_data(smic);
    if (data != 0) {
    if (smic_debug & SMIC_DEBUG_ENABLE)
    dev_dbg(smic.io.dev,
    "SMIC_WRITE_END: data = %02x\n",
    data);
    start_error_recovery(smic,
    "state = SMIC_WRITE_END, "
    "data != SUCCESS");
    return SI_SM_CALL_WITH_DELAY;
    } else
    smic.state = SMIC_WRITE2READ;
    break;
    case SMIC_WRITE2READ:
//
// we must wait for RX_DATA_READY to be set before we
// can continue
//
    if (flags & SMIC_RX_DATA_READY) {
    write_smic_control(smic, SMIC_CC_SMS_RD_START);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_READ_START;
    } else
    return SI_SM_CALL_WITH_DELAY;
    break;
    case SMIC_READ_START:
    if (status != SMIC_SC_SMS_RD_START) {
    start_error_recovery(smic,
    "state = SMIC_READ_START, "
    "status != SMIC_SC_SMS_RD_START");
    return SI_SM_CALL_WITH_DELAY;
    }
    if (flags & SMIC_RX_DATA_READY) {
    read_next_byte(smic);
    write_smic_control(smic, SMIC_CC_SMS_RD_NEXT);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_READ_NEXT;
    } else
    return SI_SM_CALL_WITH_DELAY;
    break;
    case SMIC_READ_NEXT:
    switch (status) {
//
// smic tells us that this is the last byte to be read
// --> clean up
//
    case SMIC_SC_SMS_RD_END:
    read_next_byte(smic);
    write_smic_control(smic, SMIC_CC_SMS_RD_END);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_READ_END;
    break;
    case SMIC_SC_SMS_RD_NEXT:
    if (flags & SMIC_RX_DATA_READY) {
    read_next_byte(smic);
    write_smic_control(smic, SMIC_CC_SMS_RD_NEXT);
    write_smic_flags(smic, flags | SMIC_FLAG_BSY);
    smic.state = SMIC_READ_NEXT;
    } else
    return SI_SM_CALL_WITH_DELAY;
    break;
    default:
    start_error_recovery(
    smic,
    "state = SMIC_READ_NEXT, "
    "status != SMIC_SC_SMS_RD_(NEXT|END)");
    return SI_SM_CALL_WITH_DELAY;
    }
    break;
    case SMIC_READ_END:
    if (status != SMIC_SC_SMS_READY) {
    start_error_recovery(smic,
    "state = SMIC_READ_END, "
    "status != SMIC_SC_SMS_READY");
    return SI_SM_CALL_WITH_DELAY;
    }
    data = read_smic_data(smic);
// data register holds an error code
    if (data != 0) {
    if (smic_debug & SMIC_DEBUG_ENABLE)
    dev_dbg(smic.io.dev,
    "SMIC_READ_END: data = %02x\n",
    data);
    start_error_recovery(smic,
    "state = SMIC_READ_END, "
    "data != SUCCESS");
    return SI_SM_CALL_WITH_DELAY;
    } else {
    smic.state = SMIC_IDLE;
    return SI_SM_TRANSACTION_COMPLETE;
    }
    case SMIC_HOSED:
    init_smic_data(smic, smic.io);
    return SI_SM_HOSED;
    default:
    if (smic_debug & SMIC_DEBUG_ENABLE) {
    dev_dbg(smic.io.dev,
    "smic.state = %d\n", smic.state);
    start_error_recovery(smic, "state = UNKNOWN");
    return SI_SM_CALL_WITH_DELAY;
    }
    }
    smic.smic_timeout = SMIC_RETRY_TIMEOUT;
    return SI_SM_CALL_WITHOUT_DELAY;
    }
#[no_mangle]
unsafe extern "C" fn smic_detect(smic: *mut si_sm_data) -> c_int {
    static int smic_detect(struct si_sm_data *smic)
    {
//
// It's impossible for the SMIC fnags register to be all 1's,
// (assuming a properly functioning, self-initialized BMC)
// but that's what you get from reading a bogus address, so we
// test that first.
//
    if (read_smic_flags(smic) == 0xff)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smic_cleanup(kcs: *mut si_sm_data) {
    static void smic_cleanup(struct si_sm_data *kcs)
    {
    }
#[no_mangle]
unsafe extern "C" fn smic_size() -> c_int {
    static int smic_size(void)
    {
    return sizeof(struct si_sm_data);
    }
    const struct si_sm_handlers smic_smi_handlers = {
    .init_data         = init_smic_data,
    .start_transaction = start_smic_transaction,
    .get_result        = smic_get_result,
    .event             = smic_event,
    .detect            = smic_detect,
    .cleanup           = smic_cleanup,
    .size              = smic_size,
    };
