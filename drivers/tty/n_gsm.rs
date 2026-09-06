//! Automatically rewritten from C to Rust
//! Source: drivers/tty/n_gsm.c
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
// n_gsm.c GSM 0710 tty multiplexor
// Copyright (c) 2009/10 Intel Corporation
// Copyright (c) 2022/23 Siemens Mobility GmbH
//
// * THIS IS A DEVELOPMENT SNAPSHOT IT IS NOT A FINAL RELEASE
//
// Outgoing path:
// tty -> DLCI fifo -> scheduler -> GSM MUX data queue    ---o-> ldisc
// control message               -> GSM MUX control queue --´
//
// Incoming path:
// ldisc -> gsm_queue() -o--> tty
// `-> gsm_control_response()
//
// TO DO:
// Mostly done:	ioctls for setting modes/timing
// Partly done:	hooks so you can pull off frames to non tty devs
// Restart DLCI 0 when it closes ?
// Improve the tx engine
// Resolve tx side locking by adding a queue_head and routing
// all control traffic via it
// General tidy/document
// Review the locking/move to refcounts more (mux now moved to an
// alloc/free model ready)
// Use newest tty open/close port helpers and install hooks
// What to do about power functions ?
// Termios setting and negotiation
// Do we need a 'which mux are you' ioctl to correlate mux and tty sets
//

    static int debug;
    module_param(debug, int, 0600);
// Module debug bits

// Defaults: these are from the specification

// Use long timers for testing at low speed with debug on

pub const T1: c_int = 100;
pub const T2: c_int = 200;

//
// Semi-arbitrary buffer size limits. 0710 is normally run with 32-64 byte
// limits so this is plenty
//
pub const MAX_MRU: c_int = 1500;
pub const MAX_MTU: c_int = 1500;

// SOF, ADDR, CTRL, LEN1, LEN2, ..., FCS, EOF
pub const PROT_OVERHEAD: c_int = 7;

//
// struct gsm_mux_net	-	network interface
//
// Created when net interface is initialized.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_mux_net {
    pub ref: kref,
    pub dlci: *mut gsm_dlci,
}

//
// Each block of data we have queued to go out is in the form of
// a gsm_msg which holds everything we need in a link layer independent
// format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_msg {
    pub list: list_head,
    pub /: *mut *mut u8 addr; / DLCI address + flags,
    pub /: *mut *mut u8 ctrl; / Control byte + flags,
    pub /: *mut *mut unsigned int len; / Length of data block (can be zero),
    pub /: *mut *mut *mut u8 data; / Points into buffer but not at the start,
    pub buffer: [u8; ],
}

    enum gsm_dlci_state {
    DLCI_CLOSED,
    DLCI_WAITING_CONFIG,	/* Waiting for DLCI configuration from user */
    DLCI_CONFIGURE,		/* Sending PN (for adaption > 1) */
    DLCI_OPENING,		/* Sending SABM not seen UA */
    DLCI_OPEN,		/* SABM/UA complete */
    DLCI_CLOSING,		/* Sending DISC not seen UA/DM */
    };
    enum gsm_dlci_mode {
    DLCI_MODE_ABM,		/* Normal Asynchronous Balanced Mode */
    DLCI_MODE_ADM,		/* Asynchronous Disconnected Mode */
    };
//
// Each active data link has a gsm_dlci structure associated which ties
// the link layer to an optional tty (if the tty side is open). To avoid
// complexity right now these are only ever freed up when the mux is
// shut down.
//
// At the moment we don't free DLCI objects until the mux is torn down
// this avoid object life time issues but might be worth review later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_dlci {
    pub gsm: *mut gsm_mux,
    pub addr: c_int,
    pub state: enum gsm_dlci_state,
    pub mutex: mutex,
// Link layer
    pub mode: enum gsm_dlci_mode,
    pub /: *mut *mut spinlock_t lock; / Protects the internal state,
    pub /: *mut *mut timer_list t1; / Retransmit timer for SABM and UA,
    pub retries: c_int,
// Uplink tty if active
    pub /: *mut *mut tty_port port; / The tty bound to this DLCI if there is one,

    pub /: *mut *mut kfifo fifo; / Queue fifo for the DLCI,
    pub /: *mut *mut int adaption; / Adaption layer in use,
    pub prev_adaption: c_int,
    pub /: *mut *mut u32 modem_rx; / Our incoming virtual modem lines,
    pub /: *mut *mut u32 modem_tx; / Our outgoing modem lines,
    pub mtu: c_uint,
    pub /: *mut *mut bool dead; / Refuse re-open,
// Configuration
    pub /: *mut *mut u8 prio; / Priority,
    pub /: *mut *mut u8 ftype; / Frame type,
    pub /: *mut *mut u8 k; / Window size,
// Flow control
    pub /: *mut *mut bool throttled; / Private copy of throttle state,
    pub /: *mut *mut bool constipated; / Throttle status for outgoing,
// Packetised I/O
    pub /: *mut *mut *mut sk_buff skb; / Frame being sent,
    pub /: *mut *mut sk_buff_head skb_list; / Queued frames,
// Data handling callback
    pub len): *const *const *const *const void (data)(struct gsm_dlci dlci, u8 data, int,
    pub len): *const *const *const *const void (prev_data)(struct gsm_dlci dlci, u8 data, int,
    pub /: *mut *mut *mut net_device net; / network interface, if created,
}

//
// Parameter bits used for parameter negotiation according to 3GPP 27.010
// chapter 5.4.6.3.1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_dlci_param_bits {
    pub d_bits: u8,
    pub i_cl_bits: u8,
    pub p_bits: u8,
    pub t_bits: u8,
    pub n_bits: __le16,
    pub na_bits: u8,
    pub k_bits: u8,
}

    static_assert(sizeof(struct gsm_dlci_param_bits) == 8);

// Total number of supported devices
pub const GSM_TTY_MINORS: c_int = 256;
// DLCI 0, 62/63 are special or reserved see gsmtty_open
pub const NUM_DLCI: c_int = 64;
//
// DLCI 0 is used to pass control blocks out of band of the data
// flow (and with a higher link priority). One command can be outstanding
// at a time and we use this structure to manage them. They are created
// and destroyed by the user context, and updated by the receive paths
// and timers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_control {
    pub /: *mut *mut u8 cmd; / Command we are issuing,
    pub /: *mut *mut *mut u8 data; / Data for the command in case we retransmit,
    pub /: *mut *mut int len; / Length of block for retransmission,
    pub /: *mut *mut int done; / Done flag,
    pub /: *mut *mut int error; / Error if any,
}

    enum gsm_encoding {
    GSM_BASIC_OPT,
    GSM_ADV_OPT,
    };
    enum gsm_mux_state {
    GSM_SEARCH,
    GSM0_ADDRESS,
    GSM0_CONTROL,
    GSM0_LEN0,
    GSM0_LEN1,
    GSM0_DATA,
    GSM0_FCS,
    GSM0_SSOF,
    GSM1_START,
    GSM1_ADDRESS,
    GSM1_CONTROL,
    GSM1_DATA,
    GSM1_OVERRUN,
    };
//
// Each GSM mux we have is represented by this structure. If we are
// operating as an ldisc then we use this structure as our ldisc
// state. We need to sort out lifetimes and locking with respect
// to the gsm mux array. For now we don't free DLCI objects that
// have been instantiated until the mux itself is terminated.
//
// To consider further: tty open versus mux shutdown.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_mux {
    pub /: *mut *mut *mut tty_tty; / The tty our ldisc is bound to,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub num: c_uint,
    pub ref: kref,
// Events on the GSM channel
    pub event: wait_queue_head_t,
// ldisc send work
    pub tx_work: work_struct,
// Bits for GSM mode decoding
// Framing Layer
    pub buf: *mut u8,
    pub state: enum gsm_mux_state,
    pub len: c_uint,
    pub address: c_uint,
    pub count: c_uint,
    pub escape: bool,
    pub encoding: enum gsm_encoding,
    pub control: u8,
    pub fcs: u8,
    pub /: *mut *mut *mut u8 txframe; / TX framing buffer,
// Method for the receiver side
    pub ch): *mut *mut *mut void (receive)(struct gsm_mux gsm, u8,
// Link Layer
    pub mru: c_uint,
    pub mtu: c_uint,
    pub /: *mut *mut int initiator; / Did we initiate connection,
    pub /: *mut *mut bool dead; / Has the mux been shut down,
    pub dlci: [*mut gsm_dlci; NUM_DLCI],
    pub /: *mut *mut int old_c_iflag; / termios c_iflag value before attach,
    pub /: *mut *mut bool constipated; / Asked by remote to shut up,
    pub /: *mut *mut bool has_devices; / Devices were registered,
    pub tx_lock: spinlock_t,
    pub /: *mut *mut unsigned int tx_bytes; / TX data outstanding,
pub const TX_THRESH_HI: c_int = 8192;
pub const TX_THRESH_LO: c_int = 2048;
    pub /: *mut *mut list_head tx_ctrl_list; / Pending control packets,
    pub /: *mut *mut list_head tx_data_list; / Pending data packets,
// Control messages
    pub /: *mut *mut timer_list kick_timer; / Kick TX queuing on timeout,
    pub /: *mut *mut timer_list t2_timer; / Retransmit timer for commands,
    pub /: *mut *mut int cretries; / Command retry counter,
    pub /: *mut *mut *mut gsm_control pending_cmd;/ Our current pending command,
    pub /: *mut *mut spinlock_t control_lock; / Protects the pending command,
// Keep-alive
    pub /: *mut *mut timer_list ka_timer; / Keep-alive response timer,
    pub /: *mut *mut u8 ka_num; / Keep-alive match pattern,
    pub /: *mut *mut signed int ka_retries; / Keep-alive retry counter, -1 if not yet initialized,
// Configuration
    pub /: *mut *mut int adaption; / 1 or 2 supported,
    pub /: *mut *mut u8 ftype; / UI or UIH,
    pub /: *mut *mut int t1, t2; / Timers in 1/100th of a sec,
    pub /: *mut *mut unsigned int t3; / Power wake-up timer in seconds.,
    pub /: *mut *mut int n2; / Retry count,
    pub /: *mut *mut u8 k; / Window size,
    pub /: *mut *mut bool wait_config; / Wait for configuration by ioctl before DLCI open,
    pub /: *mut *mut u32 keep_alive; / Control channel keep-alive in 10ms,
// Statistics (not currently exposed)
    pub bad_fcs: c_ulong,
    pub malformed: c_ulong,
    pub io_error: c_ulong,
    pub open_error: c_ulong,
    pub bad_size: c_ulong,
    pub unsupported: c_ulong,
}

//
// Mux objects - needed so that we can translate a tty index into the
// relevant mux and DLCI.
//

    static struct gsm_mux *gsm_mux[MAX_MUX];	/* GSM muxes */
    static DEFINE_SPINLOCK(gsm_mux_lock);
    static struct tty_driver *gsm_tty_driver;
//
// This section of the driver logic implements the GSM encodings
// both the basic and the 'advanced'. Reliable transport is not
// supported.
//
pub const CR: c_uint = 0x02;
pub const EA: c_uint = 0x01;
pub const PF: c_uint = 0x10;
// I is special: the rest are ..
pub const RR: c_uint = 0x01;
pub const UI: c_uint = 0x03;
pub const RNR: c_uint = 0x05;
pub const REJ: c_uint = 0x09;
pub const DM: c_uint = 0x0F;
pub const SABM: c_uint = 0x2F;
pub const DISC: c_uint = 0x43;
pub const UA: c_uint = 0x63;
pub const UIH: c_uint = 0xEF;
// Channel commands
pub const CMD_NSC: c_uint = 0x09;
pub const CMD_TEST: c_uint = 0x11;
pub const CMD_PSC: c_uint = 0x21;
pub const CMD_RLS: c_uint = 0x29;
pub const CMD_FCOFF: c_uint = 0x31;
pub const CMD_PN: c_uint = 0x41;
pub const CMD_RPN: c_uint = 0x49;
pub const CMD_FCON: c_uint = 0x51;
pub const CMD_CLD: c_uint = 0x61;
pub const CMD_SNC: c_uint = 0x69;
pub const CMD_MSC: c_uint = 0x71;
// Virtual modem bits
pub const MDM_FC: c_uint = 0x01;
pub const MDM_RTC: c_uint = 0x02;
pub const MDM_RTR: c_uint = 0x04;
pub const MDM_IC: c_uint = 0x20;
pub const MDM_DV: c_uint = 0x40;
pub const GSM0_SOF: c_uint = 0xF9;
pub const GSM1_SOF: c_uint = 0x7E;
pub const GSM1_ESCAPE: c_uint = 0x7D;
pub const GSM1_ESCAPE_BITS: c_uint = 0x20;
pub const XON: c_uint = 0x11;
pub const XOFF: c_uint = 0x13;
pub const ISO_IEC_646_MASK: c_uint = 0x7F;
    static const struct tty_port_operations gsm_port_ops;
//
// CRC table for GSM 0710
//
    static const u8 gsm_fcs8[256] = {
    0x00, 0x91, 0xE3, 0x72, 0x07, 0x96, 0xE4, 0x75,
    0x0E, 0x9F, 0xED, 0x7C, 0x09, 0x98, 0xEA, 0x7B,
    0x1C, 0x8D, 0xFF, 0x6E, 0x1B, 0x8A, 0xF8, 0x69,
    0x12, 0x83, 0xF1, 0x60, 0x15, 0x84, 0xF6, 0x67,
    0x38, 0xA9, 0xDB, 0x4A, 0x3F, 0xAE, 0xDC, 0x4D,
    0x36, 0xA7, 0xD5, 0x44, 0x31, 0xA0, 0xD2, 0x43,
    0x24, 0xB5, 0xC7, 0x56, 0x23, 0xB2, 0xC0, 0x51,
    0x2A, 0xBB, 0xC9, 0x58, 0x2D, 0xBC, 0xCE, 0x5F,
    0x70, 0xE1, 0x93, 0x02, 0x77, 0xE6, 0x94, 0x05,
    0x7E, 0xEF, 0x9D, 0x0C, 0x79, 0xE8, 0x9A, 0x0B,
    0x6C, 0xFD, 0x8F, 0x1E, 0x6B, 0xFA, 0x88, 0x19,
    0x62, 0xF3, 0x81, 0x10, 0x65, 0xF4, 0x86, 0x17,
    0x48, 0xD9, 0xAB, 0x3A, 0x4F, 0xDE, 0xAC, 0x3D,
    0x46, 0xD7, 0xA5, 0x34, 0x41, 0xD0, 0xA2, 0x33,
    0x54, 0xC5, 0xB7, 0x26, 0x53, 0xC2, 0xB0, 0x21,
    0x5A, 0xCB, 0xB9, 0x28, 0x5D, 0xCC, 0xBE, 0x2F,
    0xE0, 0x71, 0x03, 0x92, 0xE7, 0x76, 0x04, 0x95,
    0xEE, 0x7F, 0x0D, 0x9C, 0xE9, 0x78, 0x0A, 0x9B,
    0xFC, 0x6D, 0x1F, 0x8E, 0xFB, 0x6A, 0x18, 0x89,
    0xF2, 0x63, 0x11, 0x80, 0xF5, 0x64, 0x16, 0x87,
    0xD8, 0x49, 0x3B, 0xAA, 0xDF, 0x4E, 0x3C, 0xAD,
    0xD6, 0x47, 0x35, 0xA4, 0xD1, 0x40, 0x32, 0xA3,
    0xC4, 0x55, 0x27, 0xB6, 0xC3, 0x52, 0x20, 0xB1,
    0xCA, 0x5B, 0x29, 0xB8, 0xCD, 0x5C, 0x2E, 0xBF,
    0x90, 0x01, 0x73, 0xE2, 0x97, 0x06, 0x74, 0xE5,
    0x9E, 0x0F, 0x7D, 0xEC, 0x99, 0x08, 0x7A, 0xEB,
    0x8C, 0x1D, 0x6F, 0xFE, 0x8B, 0x1A, 0x68, 0xF9,
    0x82, 0x13, 0x61, 0xF0, 0x85, 0x14, 0x66, 0xF7,
    0xA8, 0x39, 0x4B, 0xDA, 0xAF, 0x3E, 0x4C, 0xDD,
    0xA6, 0x37, 0x45, 0xD4, 0xA1, 0x30, 0x42, 0xD3,
    0xB4, 0x25, 0x57, 0xC6, 0xB3, 0x22, 0x50, 0xC1,
    0xBA, 0x2B, 0x59, 0xC8, 0xBD, 0x2C, 0x5E, 0xCF
    };
pub const INIT_FCS: c_uint = 0xFF;
pub const GOOD_FCS: c_uint = 0xCF;
    static void gsm_dlci_close(struct gsm_dlci *dlci);
    static int gsmld_output(struct gsm_mux *gsm, u8 *data, int len);
    static int gsm_modem_update(struct gsm_dlci *dlci, u8 brk);
    static struct gsm_msg *gsm_data_alloc(struct gsm_mux *gsm, u8 addr, int len,
    u8 ctrl);
    static int gsm_send_packet(struct gsm_mux *gsm, struct gsm_msg *msg);
    static struct gsm_dlci *gsm_dlci_alloc(struct gsm_mux *gsm, int addr);
    static void gsmld_write_trigger(struct gsm_mux *gsm);
    static void gsmld_write_task(struct work_struct *work);
    static int gsm_modem_send_initial_msc(struct gsm_dlci *dlci);
//
// gsm_fcs_add	-	update FCS
// @fcs: Current FCS
// @c: Next data
//
// Update the FCS to include c. Uses the algorithm in the specification
// notes.
//
#[no_mangle]
pub unsafe extern "C" fn gsm_fcs_add(fcs: u8, c: u8) -> u8 {
    static inline u8 gsm_fcs_add(u8 fcs, u8 c)
    {
    return gsm_fcs8[fcs ^ c];
    }
//
// gsm_fcs_add_block	-	update FCS for a block
// @fcs: Current FCS
// @c: buffer of data
// @len: length of buffer
//
// Update the FCS to include c. Uses the algorithm in the specification
// notes.
//
#[no_mangle]
pub unsafe extern "C" fn gsm_fcs_add_block(fcs: u8, c: *mut u8, len: c_int) -> u8 {
    static inline u8 gsm_fcs_add_block(u8 fcs, u8 *c, int len)
    {
    while (len--)
    fcs = gsm_fcs8[fcs ^ *c++];
    return fcs;
    }
//
// gsm_read_ea		-	read a byte into an EA
// @val: variable holding value
// @c: byte going into the EA
//
// Processes one byte of an EA. Updates the passed variable
// and returns 1 if the EA is now completely read
//
#[no_mangle]
unsafe extern "C" fn gsm_read_ea(val: *mut c_uint, c: u8) -> c_int {
    static int gsm_read_ea(unsigned int *val, u8 c)
    {
// Add the next 7 bits into the value
// val <<= 7;
// val |= c >> 1;
// Was this the last byte of the EA 1 = yes
    return c & EA;
    }
//
// gsm_read_ea_val	-	read a value until EA
// @val: variable holding value
// @data: buffer of data
// @dlen: length of data
//
// Processes an EA value. Updates the passed variable and
// returns the processed data length.
//
#[no_mangle]
unsafe extern "C" fn gsm_read_ea_val(val: *mut c_uint, data: *const u8, dlen: c_int) -> c_uint {
    static unsigned int gsm_read_ea_val(unsigned int *val, const u8 *data, int dlen)
    {
    let mut len: c_uint = 0;
    for (; dlen > 0; dlen--) {
    len++;
    if (gsm_read_ea(val, *data++))
    break;
    }
    return len;
    }
//
// gsm_encode_modem	-	encode modem data bits
// @dlci: DLCI to encode from
//
// Returns the correct GSM encoded modem status bits (6 bit field) for
// the current status of the DLCI and attached tty object
//
#[no_mangle]
unsafe extern "C" fn gsm_encode_modem(dlci: *const gsm_dlci) -> u8 {
    static u8 gsm_encode_modem(const struct gsm_dlci *dlci)
    {
    let mut modembits: u8 = 0;
// FC is true flow control not modem bits
    if (dlci.throttled)
    modembits |= MDM_FC;
    if (dlci.modem_tx & TIOCM_DTR)
    modembits |= MDM_RTC;
    if (dlci.modem_tx & TIOCM_RTS)
    modembits |= MDM_RTR;
    if (dlci.modem_tx & TIOCM_RI)
    modembits |= MDM_IC;
    if (dlci.modem_tx & TIOCM_CD || dlci.gsm.initiator)
    modembits |= MDM_DV;
// special mappings for passive side to operate as UE
    if (dlci.modem_tx & TIOCM_OUT1)
    modembits |= MDM_IC;
    if (dlci.modem_tx & TIOCM_OUT2)
    modembits |= MDM_DV;
    return modembits;
    }
    static void gsm_hex_dump_bytes(const char *fname, const u8 *data,
    unsigned long len)
    {
    char *prefix;
    if (!fname) {
    print_hex_dump(KERN_INFO, "", DUMP_PREFIX_NONE, 16, 1, data, len,
    true);
    return;
    }
    prefix = kasprintf(GFP_ATOMIC, "%s: ", fname);
    if (!prefix)
    return;
    print_hex_dump(KERN_INFO, prefix, DUMP_PREFIX_OFFSET, 16, 1, data, len,
    true);
    kfree(prefix);
    }
//
// gsm_encode_params	-	encode DLCI parameters
// @dlci: DLCI to encode from
// @params: buffer to fill with the encoded parameters
//
// Encodes the parameters according to GSM 07.10 section 5.4.6.3.1
// table 3.
//
    static int gsm_encode_params(const struct gsm_dlci *dlci,
    struct gsm_dlci_param_bits *params)
    {
    const struct gsm_mux *gsm = dlci.gsm;
    unsigned int i, cl;
    switch (dlci.ftype) {
    case UIH:
    i = 0; /* UIH */
    break;
    case UI:
    i = 1; /* UI */
    break;
    default:
    pr_debug("unsupported frame type %d\n", dlci.ftype);
    return -EINVAL;
    }
    switch (dlci.adaption) {
    case 1: /* Unstructured */
    cl = 0; /* convergence layer type 1 */
    break;
    case 2: /* Unstructured with modem bits. */
    cl = 1; /* convergence layer type 2 */
    break;
    default:
    pr_debug("unsupported adaption %d\n", dlci.adaption);
    return -EINVAL;
    }
    params.d_bits = FIELD_PREP(PN_D_FIELD_DLCI, dlci.addr);
// UIH, convergence layer type 1
    params.i_cl_bits = FIELD_PREP(PN_I_CL_FIELD_FTYPE, i) |
    FIELD_PREP(PN_I_CL_FIELD_ADAPTION, cl);
    params.p_bits = FIELD_PREP(PN_P_FIELD_PRIO, dlci.prio);
    params.t_bits = FIELD_PREP(PN_T_FIELD_T1, gsm.t1);
    params.n_bits = cpu_to_le16(FIELD_PREP(PN_N_FIELD_N1, dlci.mtu));
    params.na_bits = FIELD_PREP(PN_NA_FIELD_N2, gsm.n2);
    params.k_bits = FIELD_PREP(PN_K_FIELD_K, dlci.k);
    return 0;
    }
//
// gsm_register_devices	-	register all tty devices for a given mux index
//
// @driver: the tty driver that describes the tty devices
// @index:  the mux number is used to calculate the minor numbers of the
// ttys for this mux and may differ from the position in the
// mux array.
//
#[no_mangle]
unsafe extern "C" fn gsm_register_devices(driver: *mut tty_driver, index: c_uint) -> c_int {
    static int gsm_register_devices(struct tty_driver *driver, unsigned int index)
    {
    struct device *dev;
    int i;
    unsigned int base;
    if (!driver || index >= MAX_MUX)
    return -EINVAL;
    base = index * NUM_DLCI; /* first minor for this index */
    for (i = 1; i < NUM_DLCI; i++) {
// Don't register device 0 - this is the control channel
// and not a usable tty interface
//
    dev = tty_register_device(gsm_tty_driver, base + i, core::ptr::null_mut());
    if (IS_ERR(dev)) {
    if (debug & DBG_ERRORS)
    pr_info("%s failed to register device minor %u",
    __func__, base + i);
    for (i--; i >= 1; i--)
    tty_unregister_device(gsm_tty_driver, base + i);
    return PTR_ERR(dev);
    }
    }
    return 0;
    }
//
// gsm_unregister_devices	-	unregister all tty devices for a given mux index
//
// @driver: the tty driver that describes the tty devices
// @index:  the mux number is used to calculate the minor numbers of the
// ttys for this mux and may differ from the position in the
// mux array.
//
    static void gsm_unregister_devices(struct tty_driver *driver,
    unsigned int index)
    {
    int i;
    unsigned int base;
    if (!driver || index >= MAX_MUX)
    return;
    base = index * NUM_DLCI; /* first minor for this index */
    for (i = 1; i < NUM_DLCI; i++) {
// Don't unregister device 0 - this is the control
// channel and not a usable tty interface
//
    tty_unregister_device(gsm_tty_driver, base + i);
    }
    }
//
// gsm_print_packet	-	display a frame for debug
// @hdr: header to print before decode
// @addr: address EA from the frame
// @cr: C/R bit seen as initiator
// @control: control including PF bit
// @data: following data bytes
// @dlen: length of data
//
// Displays a packet in human readable format for debugging purposes. The
// style is based on amateur radio LAP-B dump display.
//
    static void gsm_print_packet(const char *hdr, int addr, int cr,
    u8 control, const u8 *data, int dlen)
    {
    if (!(debug & DBG_DUMP))
    return;
// Only show user payload frames if debug & DBG_PAYLOAD
    if (!(debug & DBG_PAYLOAD) && addr != 0)
    if ((control & ~PF) == UI || (control & ~PF) == UIH)
    return;
    pr_info("%s %d) %c: ", hdr, addr, "RC"[cr]);
    switch (control & ~PF) {
    case SABM:
    pr_cont("SABM");
    break;
    case UA:
    pr_cont("UA");
    break;
    case DISC:
    pr_cont("DISC");
    break;
    case DM:
    pr_cont("DM");
    break;
    case UI:
    pr_cont("UI");
    break;
    case UIH:
    pr_cont("UIH");
    break;
    default:
    if (!(control & 0x01)) {
    pr_cont("I N(S)%d N(R)%d",
    (control & 0x0E) >> 1, (control & 0xE0) >> 5);
    } else switch (control & 0x0F) {
    case RR:
    pr_cont("RR(%d)", (control & 0xE0) >> 5);
    break;
    case RNR:
    pr_cont("RNR(%d)", (control & 0xE0) >> 5);
    break;
    case REJ:
    pr_cont("REJ(%d)", (control & 0xE0) >> 5);
    break;
    default:
    pr_cont("[%02X]", control);
    }
    }
    if (control & PF)
    pr_cont("(P)");
    else
    pr_cont("(F)");
    gsm_hex_dump_bytes(core::ptr::null_mut(), data, dlen);
    }
//
// Link level transmission side
//
// gsm_stuff_frame	-	bytestuff a packet
// @input: input buffer
// @output: output buffer
// @len: length of input
//
// Expand a buffer by bytestuffing it. The worst case size change
// is doubling and the caller is responsible for handing out
// suitable sized buffers.
//
#[no_mangle]
unsafe extern "C" fn gsm_stuff_frame(input: *const u8, output: *mut u8, len: c_int) -> c_int {
    static int gsm_stuff_frame(const u8 *input, u8 *output, int len)
    {
    let mut olen: c_int = 0;
    while (len--) {
    if (*input == GSM1_SOF || *input == GSM1_ESCAPE
    || (*input & ISO_IEC_646_MASK) == XON
    || (*input & ISO_IEC_646_MASK) == XOFF) {
// output++ = GSM1_ESCAPE;
// output++ = *input++ ^ GSM1_ESCAPE_BITS;
    olen++;
    } else
// output++ = *input++;
    olen++;
    }
    return olen;
    }
//
// gsm_send	-	send a control frame
// @gsm: our GSM mux
// @addr: address for control frame
// @cr: command/response bit seen as initiator
// @control:  control byte including PF bit
//
// Format up and transmit a control frame. These should be transmitted
// ahead of data when they are needed.
//
#[no_mangle]
unsafe extern "C" fn gsm_send(gsm: *mut gsm_mux, addr: c_int, cr: c_int, control: c_int) -> c_int {
    static int gsm_send(struct gsm_mux *gsm, int addr, int cr, int control)
    {
    struct gsm_msg *msg;
    u8 *dp;
    int ocr;
    unsigned long flags;
    msg = gsm_data_alloc(gsm, addr, 0, control);
    if (!msg)
    return -ENOMEM;
// toggle C/R coding if not initiator
    ocr = cr ^ (gsm.initiator ? 0 : 1);
    msg.data -= 3;
    dp = msg.data;
// dp++ = (addr << 2) | (ocr << 1) | EA;
// dp++ = control;
    if (gsm.encoding == GSM_BASIC_OPT)
// dp++ = EA; /* Length of data = 0
// dp = 0xFF - gsm_fcs_add_block(INIT_FCS, msg->data, dp - msg->data);
    msg.len = (dp - msg.data) + 1;
    gsm_print_packet("Q.", addr, cr, control, core::ptr::null_mut(), 0);
    spin_lock_irqsave(&gsm.tx_lock, flags);
    list_add_tail(&msg.list, &gsm.tx_ctrl_list);
    gsm.tx_bytes += msg.len;
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    gsmld_write_trigger(gsm);
    return 0;
    }
//
// gsm_dlci_clear_queues	-	remove outstanding data for a DLCI
// @gsm: mux
// @dlci: clear for this DLCI
//
// Clears the data queues for a given DLCI.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_clear_queues(gsm: *mut gsm_mux, dlci: *mut gsm_dlci) {
    static void gsm_dlci_clear_queues(struct gsm_mux *gsm, struct gsm_dlci *dlci)
    {
    struct gsm_msg *msg, *nmsg;
    let mut addr: c_int = dlci.addr;
    unsigned long flags;
// Clear DLCI write fifo first
    spin_lock_irqsave(&dlci.lock, flags);
    kfifo_reset(&dlci.fifo);
    spin_unlock_irqrestore(&dlci.lock, flags);
// Clear data packets in MUX write queue
    spin_lock_irqsave(&gsm.tx_lock, flags);
    list_for_each_entry_safe(msg, nmsg, &gsm.tx_data_list, list) {
    if (msg.addr != addr)
    continue;
    gsm.tx_bytes -= msg.len;
    list_del(&msg.list);
    kfree(msg);
    }
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    }
//
// gsm_response	-	send a control response
// @gsm: our GSM mux
// @addr: address for control frame
// @control:  control byte including PF bit
//
// Format up and transmit a link level response frame.
//
#[no_mangle]
pub unsafe extern "C" fn gsm_response(gsm: *mut gsm_mux, addr: c_int, control: c_int) {
    static inline void gsm_response(struct gsm_mux *gsm, int addr, int control)
    {
    gsm_send(gsm, addr, 0, control);
    }
//
// gsm_command	-	send a control command
// @gsm: our GSM mux
// @addr: address for control frame
// @control:  control byte including PF bit
//
// Format up and transmit a link level command frame.
//
#[no_mangle]
pub unsafe extern "C" fn gsm_command(gsm: *mut gsm_mux, addr: c_int, control: c_int) {
    static inline void gsm_command(struct gsm_mux *gsm, int addr, int control)
    {
    gsm_send(gsm, addr, 1, control);
    }
// Data transmission

//
// gsm_data_alloc		-	allocate data frame
// @gsm: GSM mux
// @addr: DLCI address
// @len: length excluding header and FCS
// @ctrl: control byte
//
// Allocate a new data buffer for sending frames with data. Space is left
// at the front for header bytes but that is treated as an implementation
// detail and not for the high level code to use
//
    static struct gsm_msg *gsm_data_alloc(struct gsm_mux *gsm, u8 addr, int len,
    u8 ctrl)
    {
    struct gsm_msg *m = kmalloc(sizeof(struct gsm_msg) + len + HDR_LEN,
    GFP_ATOMIC);
    if (m == core::ptr::null_mut())
    return core::ptr::null_mut();
    m.data = m.buffer + HDR_LEN - 1;	/* Allow for FCS */
    m.len = len;
    m.addr = addr;
    m.ctrl = ctrl;
    INIT_LIST_HEAD(&m.list);
    return m;
    }
//
// gsm_send_packet	-	sends a single packet
// @gsm: GSM Mux
// @msg: packet to send
//
// The given packet is encoded and sent out. No memory is freed.
// The caller must hold the gsm tx lock.
//
#[no_mangle]
unsafe extern "C" fn gsm_send_packet(gsm: *mut gsm_mux, msg: *mut gsm_msg) -> c_int {
    static int gsm_send_packet(struct gsm_mux *gsm, struct gsm_msg *msg)
    {
    int len, ret;
    if (gsm.encoding == GSM_BASIC_OPT) {
    gsm.txframe[0] = GSM0_SOF;
    memcpy(gsm.txframe + 1, msg.data, msg.len);
    gsm.txframe[msg.len + 1] = GSM0_SOF;
    len = msg.len + 2;
    } else {
    gsm.txframe[0] = GSM1_SOF;
    len = gsm_stuff_frame(msg.data, gsm.txframe + 1, msg.len);
    gsm.txframe[len + 1] = GSM1_SOF;
    len += 2;
    }
    if (debug & DBG_DATA)
    gsm_hex_dump_bytes(__func__, gsm.txframe, len);
    gsm_print_packet("-.", msg.addr, gsm.initiator, msg.ctrl, msg.data,
    msg.len);
    ret = gsmld_output(gsm, gsm.txframe, len);
    if (ret <= 0)
    return ret;
// FIXME: Can eliminate one SOF in many more cases
    gsm.tx_bytes -= msg.len;
    return 0;
    }
//
// gsm_is_flow_ctrl_msg	-	checks if flow control message
// @msg: message to check
//
// Returns true if the given message is a flow control command of the
// control channel. False is returned in any other case.
//
#[no_mangle]
unsafe extern "C" fn gsm_is_flow_ctrl_msg(msg: *mut gsm_msg) -> bool {
    static bool gsm_is_flow_ctrl_msg(struct gsm_msg *msg)
    {
    unsigned int cmd;
    if (msg.addr > 0)
    return false;
    switch (msg.ctrl & ~PF) {
    case UI:
    case UIH:
    cmd = 0;
    if (gsm_read_ea_val(&cmd, msg.data + 2, msg.len - 2) < 1)
    break;
    switch (cmd & ~PF) {
    case CMD_FCOFF:
    case CMD_FCON:
    return true;
    }
    break;
    }
    return false;
    }
//
// gsm_data_kick	-	poke the queue
// @gsm: GSM Mux
//
// The tty device has called us to indicate that room has appeared in
// the transmit queue. Ram more data into the pipe if we have any.
// If we have been flow-stopped by a CMD_FCOFF, then we can only
// send messages on DLCI0 until CMD_FCON. The caller must hold
// the gsm tx lock.
//
#[no_mangle]
unsafe extern "C" fn gsm_data_kick(gsm: *mut gsm_mux) -> c_int {
    static int gsm_data_kick(struct gsm_mux *gsm)
    {
    struct gsm_msg *msg, *nmsg;
    struct gsm_dlci *dlci;
    int ret;
    clear_bit(TTY_DO_WRITE_WAKEUP, &gsm.tty.flags);
// Serialize control messages and control channel messages first
    list_for_each_entry_safe(msg, nmsg, &gsm.tx_ctrl_list, list) {
    if (gsm.constipated && !gsm_is_flow_ctrl_msg(msg))
    continue;
    ret = gsm_send_packet(gsm, msg);
    switch (ret) {
    case -ENOSPC:
    return -ENOSPC;
    case -ENODEV:
// ldisc not open
    gsm.tx_bytes -= msg.len;
    list_del(&msg.list);
    kfree(msg);
    continue;
    default:
    if (ret >= 0) {
    list_del(&msg.list);
    kfree(msg);
    }
    break;
    }
    }
    if (gsm.constipated)
    return -EAGAIN;
// Serialize other channels
    if (list_empty(&gsm.tx_data_list))
    return 0;
    list_for_each_entry_safe(msg, nmsg, &gsm.tx_data_list, list) {
    dlci = gsm.dlci[msg.addr];
// Send only messages for DLCIs with valid state
    if (dlci.state != DLCI_OPEN) {
    gsm.tx_bytes -= msg.len;
    list_del(&msg.list);
    kfree(msg);
    continue;
    }
    ret = gsm_send_packet(gsm, msg);
    switch (ret) {
    case -ENOSPC:
    return -ENOSPC;
    case -ENODEV:
// ldisc not open
    gsm.tx_bytes -= msg.len;
    list_del(&msg.list);
    kfree(msg);
    continue;
    default:
    if (ret >= 0) {
    list_del(&msg.list);
    kfree(msg);
    }
    break;
    }
    }
    return 1;
    }
//
// __gsm_data_queue		-	queue a UI or UIH frame
// @dlci: DLCI sending the data
// @msg: message queued
//
// Add data to the transmit queue and try and get stuff moving
// out of the mux tty if not already doing so. The Caller must hold
// the gsm tx lock.
//
#[no_mangle]
unsafe extern "C" fn __gsm_data_queue(dlci: *mut gsm_dlci, msg: *mut gsm_msg) {
    static void __gsm_data_queue(struct gsm_dlci *dlci, struct gsm_msg *msg)
    {
    struct gsm_mux *gsm = dlci.gsm;
    u8 *dp = msg.data;
    u8 *fcs = dp + msg.len;
// Fill in the header
    if (gsm.encoding == GSM_BASIC_OPT) {
    if (msg.len < 128)
// --dp = (msg->len << 1) | EA;
    else {
// --dp = (msg->len >> 7);	/* bits 7 - 15
// --dp = (msg->len & 127) << 1;	/* bits 0 - 6
    }
    }
// --dp = msg->ctrl;
    if (gsm.initiator)
// --dp = (msg->addr << 2) | CR | EA;
    else
// --dp = (msg->addr << 2) | EA;
// fcs = gsm_fcs_add_block(INIT_FCS, dp , msg->data - dp);
// Ugly protocol layering violation
    if (msg.ctrl == UI || msg.ctrl == (UI|PF))
// fcs = gsm_fcs_add_block(*fcs, msg->data, msg->len);
// fcs = 0xFF - *fcs;
    gsm_print_packet("Q> ", msg.addr, gsm.initiator, msg.ctrl,
    msg.data, msg.len);
// Move the header back and adjust the length, also allow for the FCS
    now tacked on the end */
    msg.len += (msg.data - dp) + 1;
    msg.data = dp;
// Add to the actual output queue
    switch (msg.ctrl & ~PF) {
    case UI:
    case UIH:
    if (msg.addr > 0) {
    list_add_tail(&msg.list, &gsm.tx_data_list);
    break;
    }
    fallthrough;
    default:
    list_add_tail(&msg.list, &gsm.tx_ctrl_list);
    break;
    }
    gsm.tx_bytes += msg.len;
    gsmld_write_trigger(gsm);
    mod_timer(&gsm.kick_timer, jiffies + 10 * gsm.t1 * HZ / 100);
    }
//
// gsm_data_queue		-	queue a UI or UIH frame
// @dlci: DLCI sending the data
// @msg: message queued
//
// Add data to the transmit queue and try and get stuff moving
// out of the mux tty if not already doing so. Take the
// the gsm tx lock and dlci lock.
//
#[no_mangle]
unsafe extern "C" fn gsm_data_queue(dlci: *mut gsm_dlci, msg: *mut gsm_msg) {
    static void gsm_data_queue(struct gsm_dlci *dlci, struct gsm_msg *msg)
    {
    unsigned long flags;
    spin_lock_irqsave(&dlci.gsm.tx_lock, flags);
    __gsm_data_queue(dlci, msg);
    spin_unlock_irqrestore(&dlci.gsm.tx_lock, flags);
    }
//
// gsm_dlci_data_output	-	try and push data out of a DLCI
// @gsm: mux
// @dlci: the DLCI to pull data from
//
// Pull data from a DLCI and send it into the transmit queue if there
// is data. Keep to the MRU of the mux. This path handles the usual tty
// interface which is a byte stream with optional modem data.
//
// Caller must hold the tx_lock of the mux.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_data_output(gsm: *mut gsm_mux, dlci: *mut gsm_dlci) -> c_int {
    static int gsm_dlci_data_output(struct gsm_mux *gsm, struct gsm_dlci *dlci)
    {
    struct gsm_msg *msg;
    u8 *dp;
    int h, len, size;
// for modem bits without break data
    h = ((dlci.adaption == 1) ? 0 : 1);
    len = kfifo_len(&dlci.fifo);
    if (len == 0)
    return 0;
// MTU/MRU count only the data bits but watch adaption mode
    if ((len + h) > dlci.mtu)
    len = dlci.mtu - h;
    size = len + h;
    msg = gsm_data_alloc(gsm, dlci.addr, size, dlci.ftype);
    if (!msg)
    return -ENOMEM;
    dp = msg.data;
    switch (dlci.adaption) {
    case 1: /* Unstructured */
    break;
    case 2: /* Unstructured with modem bits.
// Always one byte as we never send inline break data
//
// dp++ = (gsm_encode_modem(dlci) << 1) | EA;
    break;
    default:
    pr_err("%s: unsupported adaption %d\n", __func__,
    dlci.adaption);
    break;
    }
    WARN_ON(len != kfifo_out_locked(&dlci.fifo, dp, len,
    &dlci.lock));
// Notify upper layer about available send space.
    tty_port_tty_wakeup(&dlci.port);
    __gsm_data_queue(dlci, msg);
// Bytes of data we used up
    return size;
    }
//
// gsm_dlci_data_output_framed  -	try and push data out of a DLCI
// @gsm: mux
// @dlci: the DLCI to pull data from
//
// Pull data from a DLCI and send it into the transmit queue if there
// is data. Keep to the MRU of the mux. This path handles framed data
// queued as skbuffs to the DLCI.
//
// Caller must hold the tx_lock of the mux.
//
    static int gsm_dlci_data_output_framed(struct gsm_mux *gsm,
    struct gsm_dlci *dlci)
    {
    struct gsm_msg *msg;
    u8 *dp;
    int len, size;
    let mut last: c_int = 0, first = 0;
    let mut overhead: c_int = 0;
// One byte per frame is used for B/F flags
    if (dlci.adaption == 4)
    overhead = 1;
// dlci->skb is locked by tx_lock
    if (dlci.skb == core::ptr::null_mut()) {
    dlci.skb = skb_dequeue_tail(&dlci.skb_list);
    if (dlci.skb == core::ptr::null_mut())
    return 0;
    first = 1;
    }
    len = dlci.skb.len + overhead;
// MTU/MRU count only the data bits
    if (len > dlci.mtu) {
    if (dlci.adaption == 3) {
// Over long frame, bin it
    dev_kfree_skb_any(dlci.skb);
    dlci.skb = core::ptr::null_mut();
    return 0;
    }
    len = dlci.mtu;
    } else
    last = 1;
    size = len + overhead;
    msg = gsm_data_alloc(gsm, dlci.addr, size, dlci.ftype);
    if (msg == core::ptr::null_mut()) {
    skb_queue_tail(&dlci.skb_list, dlci.skb);
    dlci.skb = core::ptr::null_mut();
    return -ENOMEM;
    }
    dp = msg.data;
    if (dlci.adaption == 4) { /* Interruptible framed (Packetised Data) */
// Flag byte to carry the start/end info
// dp++ = last << 7 | first << 6 | 1;	/* EA
    len--;
    }
    memcpy(dp, dlci.skb.data, len);
    skb_pull(dlci.skb, len);
    __gsm_data_queue(dlci, msg);
    if (last) {
    dev_kfree_skb_any(dlci.skb);
    dlci.skb = core::ptr::null_mut();
    }
    return size;
    }
//
// gsm_dlci_modem_output	-	try and push modem status out of a DLCI
// @gsm: mux
// @dlci: the DLCI to pull modem status from
// @brk: break signal
//
// Push an empty frame in to the transmit queue to update the modem status
// bits and to transmit an optional break.
//
// Caller must hold the tx_lock of the mux.
//
    static int gsm_dlci_modem_output(struct gsm_mux *gsm, struct gsm_dlci *dlci,
    u8 brk)
    {
    u8 *dp = core::ptr::null_mut();
    struct gsm_msg *msg;
    let mut size: c_int = 0;
// for modem bits without break data
    switch (dlci.adaption) {
    case 1: /* Unstructured */
    break;
    case 2: /* Unstructured with modem bits. */
    size++;
    if (brk > 0)
    size++;
    break;
    default:
    pr_err("%s: unsupported adaption %d\n", __func__,
    dlci.adaption);
    return -EINVAL;
    }
    msg = gsm_data_alloc(gsm, dlci.addr, size, dlci.ftype);
    if (!msg) {
    pr_err("%s: gsm_data_alloc error", __func__);
    return -ENOMEM;
    }
    dp = msg.data;
    switch (dlci.adaption) {
    case 1: /* Unstructured */
    break;
    case 2: /* Unstructured with modem bits. */
    if (brk == 0) {
// dp++ = (gsm_encode_modem(dlci) << 1) | EA;
    } else {
// dp++ = gsm_encode_modem(dlci) << 1;
// dp++ = (brk << 4) | 2 | EA; /* Length, Break, EA
    }
    break;
    default:
// Handled above
    break;
    }
    __gsm_data_queue(dlci, msg);
    return size;
    }
//
// gsm_dlci_data_sweep		-	look for data to send
// @gsm: the GSM mux
//
// Sweep the GSM mux channels in priority order looking for ones with
// data to send. We could do with optimising this scan a bit. We aim
// to fill the queue totally or up to TX_THRESH_HI bytes. Once we hit
// TX_THRESH_LO we get called again
//
// FIXME: We should round robin between groups and in theory you can
// renegotiate DLCI priorities with optional stuff. Needs optimising.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_data_sweep(gsm: *mut gsm_mux) -> c_int {
    static int gsm_dlci_data_sweep(struct gsm_mux *gsm)
    {
// Priority ordering: We should do priority with RR of the groups
    int i, len, ret = 0;
    bool sent;
    struct gsm_dlci *dlci;
    while (gsm.tx_bytes < TX_THRESH_HI) {
    for (sent = false, i = 1; i < NUM_DLCI; i++) {
    dlci = gsm.dlci[i];
// skip unused or blocked channel
    if (!dlci || dlci.constipated)
    continue;
// skip channels with invalid state
    if (dlci.state != DLCI_OPEN)
    continue;
// count the sent data per adaption
    if (dlci.adaption < 3 && !dlci.net)
    len = gsm_dlci_data_output(gsm, dlci);
    else
    len = gsm_dlci_data_output_framed(gsm, dlci);
// on error exit
    if (len < 0)
    return ret;
    if (len > 0) {
    ret++;
    sent = true;
// The lower DLCs can starve the higher DLCs!
    break;
    }
// try next
    }
    if (!sent)
    break;
    }
    return ret;
    }
//
// gsm_dlci_data_kick	-	transmit if possible
// @dlci: DLCI to kick
//
// Transmit data from this DLCI if the queue is empty. We can't rely on
// a tty wakeup except when we filled the pipe so we need to fire off
// new data ourselves in other cases.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_data_kick(dlci: *mut gsm_dlci) {
    static void gsm_dlci_data_kick(struct gsm_dlci *dlci)
    {
    unsigned long flags;
    int sweep;
    if (dlci.constipated)
    return;
    spin_lock_irqsave(&dlci.gsm.tx_lock, flags);
// If we have nothing running then we need to fire up
    sweep = (dlci.gsm.tx_bytes < TX_THRESH_LO);
    if (dlci.gsm.tx_bytes == 0) {
    if (dlci.net)
    gsm_dlci_data_output_framed(dlci.gsm, dlci);
    else
    gsm_dlci_data_output(dlci.gsm, dlci);
    }
    if (sweep)
    gsm_dlci_data_sweep(dlci.gsm);
    spin_unlock_irqrestore(&dlci.gsm.tx_lock, flags);
    }
//
// Control message processing
//
// gsm_control_command	-	send a command frame to a control
// @gsm: gsm channel
// @cmd: the command to use
// @data: data to follow encoded info
// @dlen: length of data
//
// Encode up and queue a UI/UIH frame containing our command.
//
    static int gsm_control_command(struct gsm_mux *gsm, int cmd, const u8 *data,
    int dlen)
    {
    struct gsm_msg *msg;
    struct gsm_dlci *dlci = gsm.dlci[0];
    msg = gsm_data_alloc(gsm, 0, dlen + 2, dlci.ftype);
    if (msg == core::ptr::null_mut())
    return -ENOMEM;
    msg.data[0] = (cmd << 1) | CR | EA;	/* Set C/R */
    msg.data[1] = (dlen << 1) | EA;
    memcpy(msg.data + 2, data, dlen);
    gsm_data_queue(dlci, msg);
    return 0;
    }
//
// gsm_control_reply	-	send a response frame to a control
// @gsm: gsm channel
// @cmd: the command to use
// @data: data to follow encoded info
// @dlen: length of data
//
// Encode up and queue a UI/UIH frame containing our response.
//
    static void gsm_control_reply(struct gsm_mux *gsm, int cmd, const u8 *data,
    int dlen)
    {
    struct gsm_msg *msg;
    struct gsm_dlci *dlci = gsm.dlci[0];
    msg = gsm_data_alloc(gsm, 0, dlen + 2, dlci.ftype);
    if (msg == core::ptr::null_mut())
    return;
    msg.data[0] = (cmd & 0xFE) << 1 | EA;	/* Clear C/R */
    msg.data[1] = (dlen << 1) | EA;
    memcpy(msg.data + 2, data, dlen);
    gsm_data_queue(dlci, msg);
    }
//
// gsm_process_modem	-	process received modem status
// @tty: virtual tty bound to the DLCI
// @dlci: DLCI to affect
// @modem: modem bits (full EA)
// @slen: number of signal octets
//
// Used when a modem control message or line state inline in adaption
// layer 2 is processed. Sort out the local modem state and throttles
//
    static void gsm_process_modem(struct tty_struct *tty, struct gsm_dlci *dlci,
    u32 modem, int slen)
    {
    let mut mlines: c_int = 0;
    let mut brk: u8 = 0;
    int fc;
// The modem status command can either contain one octet (V.24 signals)
// or two octets (V.24 signals + break signals). This is specified in
// section 5.4.6.3.7 of the 07.10 mux spec.
//
    if (slen == 1)
    modem = modem & 0x7f;
    else {
    brk = modem & 0x7f;
    modem = (modem >> 7) & 0x7f;
    }
// Flow control/ready to communicate
    fc = (modem & MDM_FC) || !(modem & MDM_RTR);
    if (fc && !dlci.constipated) {
// Need to throttle our output on this device
    dlci.constipated = true;
    } else if (!fc && dlci.constipated) {
    dlci.constipated = false;
    gsm_dlci_data_kick(dlci);
    }
// Map modem bits
    if (modem & MDM_RTC)
    mlines |= TIOCM_DSR | TIOCM_DTR;
    if (modem & MDM_RTR)
    mlines |= TIOCM_RTS | TIOCM_CTS;
    if (modem & MDM_IC)
    mlines |= TIOCM_RI;
    if (modem & MDM_DV)
    mlines |= TIOCM_CD;
// Carrier drop -> hangup
    if (tty) {
    if ((mlines & TIOCM_CD) == 0 && (dlci.modem_rx & TIOCM_CD))
    if (!C_CLOCAL(tty))
    tty_hangup(tty);
    }
    if (brk & 0x01)
    tty_insert_flip_char(&dlci.port, 0, TTY_BREAK);
    dlci.modem_rx = mlines;
    wake_up_interruptible(&dlci.gsm.event);
    }
//
// gsm_process_negotiation	-	process received parameters
// @gsm: GSM channel
// @addr: DLCI address
// @cr: command/response
// @params: encoded parameters from the parameter negotiation message
//
// Used when the response for our parameter negotiation command was
// received.
//
    static int gsm_process_negotiation(struct gsm_mux *gsm, unsigned int addr,
    unsigned int cr,
    const struct gsm_dlci_param_bits *params)
    {
    struct gsm_dlci *dlci = gsm.dlci[addr];
    unsigned int ftype, i, adaption, prio, n1, k;
    i = FIELD_GET(PN_I_CL_FIELD_FTYPE, params.i_cl_bits);
    adaption = FIELD_GET(PN_I_CL_FIELD_ADAPTION, params.i_cl_bits) + 1;
    prio = FIELD_GET(PN_P_FIELD_PRIO, params.p_bits);
    n1 = FIELD_GET(PN_N_FIELD_N1, get_unaligned_le16(&params.n_bits));
    k = FIELD_GET(PN_K_FIELD_K, params.k_bits);
    if (n1 < MIN_MTU) {
    if (debug & DBG_ERRORS)
    pr_info("%s N1 out of range in PN\n", __func__);
    return -EINVAL;
    }
    switch (i) {
    case 0x00:
    ftype = UIH;
    break;
    case 0x01:
    ftype = UI;
    break;
    case 0x02: /* I frames are not supported */
    if (debug & DBG_ERRORS)
    pr_info("%s unsupported I frame request in PN\n",
    __func__);
    gsm.unsupported++;
    return -EINVAL;
    default:
    if (debug & DBG_ERRORS)
    pr_info("%s i out of range in PN\n", __func__);
    return -EINVAL;
    }
    if (!cr && gsm.initiator) {
    if (adaption != dlci.adaption) {
    if (debug & DBG_ERRORS)
    pr_info("%s invalid adaption %d in PN\n",
    __func__, adaption);
    return -EINVAL;
    }
    if (prio != dlci.prio) {
    if (debug & DBG_ERRORS)
    pr_info("%s invalid priority %d in PN",
    __func__, prio);
    return -EINVAL;
    }
    if (n1 > gsm.mru || n1 > dlci.mtu) {
// We requested a frame size but the other party wants
// to send larger frames. The standard allows only a
// smaller response value than requested (5.4.6.3.1).
//
    if (debug & DBG_ERRORS)
    pr_info("%s invalid N1 %d in PN\n", __func__,
    n1);
    return -EINVAL;
    }
    dlci.mtu = n1;
    if (ftype != dlci.ftype) {
    if (debug & DBG_ERRORS)
    pr_info("%s invalid i %d in PN\n", __func__, i);
    return -EINVAL;
    }
    if (ftype != UI && ftype != UIH && k > dlci.k) {
    if (debug & DBG_ERRORS)
    pr_info("%s invalid k %d in PN\n", __func__, k);
    return -EINVAL;
    }
    dlci.k = k;
    } else if (cr && !gsm.initiator) {
// Only convergence layer type 1 and 2 are supported.
    if (adaption != 1 && adaption != 2) {
    if (debug & DBG_ERRORS)
    pr_info("%s invalid adaption %d in PN\n",
    __func__, adaption);
    return -EINVAL;
    }
    dlci.adaption = adaption;
    if (n1 > gsm.mru) {
// Propose a smaller value
    dlci.mtu = gsm.mru;
    } else if (n1 > MAX_MTU) {
// Propose a smaller value
    dlci.mtu = MAX_MTU;
    } else {
    dlci.mtu = n1;
    }
    dlci.prio = prio;
    dlci.ftype = ftype;
    dlci.k = k;
    } else {
    return -EINVAL;
    }
    return 0;
    }
//
// gsm_control_modem	-	modem status received
// @gsm: GSM channel
// @data: data following command
// @clen: command length
//
// We have received a modem status control message. This is used by
// the GSM mux protocol to pass virtual modem line status and optionally
// to indicate break signals. Unpack it, convert to Linux representation
// and if need be stuff a break message down the tty.
//
#[no_mangle]
unsafe extern "C" fn gsm_control_modem(gsm: *mut gsm_mux, data: *const u8, clen: c_int) {
    static void gsm_control_modem(struct gsm_mux *gsm, const u8 *data, int clen)
    {
    let mut addr: c_uint = 0;
    let mut modem: c_uint = 0;
    struct gsm_dlci *dlci;
    let mut len: c_int = clen;
    let mut cl: c_int = clen;
    const u8 *dp = data;
    struct tty_struct *tty;
    len = gsm_read_ea_val(&addr, data, cl);
    if (len < 1)
    return;
    addr >>= 1;
// Closed port, or invalid ?
    if (addr == 0 || addr >= NUM_DLCI || gsm.dlci[addr] == core::ptr::null_mut())
    return;
    dlci = gsm.dlci[addr];
// Must be at least one byte following the EA
    if ((cl - len) < 1)
    return;
    dp += len;
    cl -= len;
// get the modem status
    len = gsm_read_ea_val(&modem, dp, cl);
    if (len < 1)
    return;
    tty = tty_port_tty_get(&dlci.port);
    gsm_process_modem(tty, dlci, modem, cl);
    if (tty) {
    tty_wakeup(tty);
    tty_kref_put(tty);
    }
    gsm_control_reply(gsm, CMD_MSC, data, clen);
    }
//
// gsm_control_negotiation	-	parameter negotiation received
// @gsm: GSM channel
// @cr: command/response flag
// @data: data following command
// @dlen: data length
//
// We have received a parameter negotiation message. This is used by
// the GSM mux protocol to configure protocol parameters for a new DLCI.
//
    static void gsm_control_negotiation(struct gsm_mux *gsm, unsigned int cr,
    const u8 *data, unsigned int dlen)
    {
    unsigned int addr;
    struct gsm_dlci_param_bits pn_reply;
    struct gsm_dlci *dlci;
    struct gsm_dlci_param_bits *params;
    if (dlen < sizeof(struct gsm_dlci_param_bits)) {
    gsm.open_error++;
    return;
    }
// Invalid DLCI?
    params = (struct gsm_dlci_param_bits *)data;
    addr = FIELD_GET(PN_D_FIELD_DLCI, params.d_bits);
    if (addr == 0 || addr >= NUM_DLCI || !gsm.dlci[addr]) {
    gsm.open_error++;
    return;
    }
    dlci = gsm.dlci[addr];
// Too late for parameter negotiation?
    if ((!cr && dlci.state == DLCI_OPENING) || dlci.state == DLCI_OPEN) {
    gsm.open_error++;
    return;
    }
// Process the received parameters
    if (gsm_process_negotiation(gsm, addr, cr, params) != 0) {
// Negotiation failed. Close the link.
    if (debug & DBG_ERRORS)
    pr_info("%s PN failed\n", __func__);
    gsm.open_error++;
    gsm_dlci_close(dlci);
    return;
    }
    if (cr) {
// Reply command with accepted parameters.
    if (gsm_encode_params(dlci, &pn_reply) == 0)
    gsm_control_reply(gsm, CMD_PN, (const u8 *)&pn_reply,
    sizeof(pn_reply));
#[no_mangle]
pub unsafe extern "C" fn if(DBG_ERRORS: debug &) -> else {
    else if (debug & DBG_ERRORS)
    pr_info("%s PN invalid\n", __func__);
    } else if (dlci.state == DLCI_CONFIGURE) {
// Proceed with link setup by sending SABM before UA
    dlci.state = DLCI_OPENING;
    gsm_command(gsm, dlci.addr, SABM|PF);
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    } else {
    if (debug & DBG_ERRORS)
    pr_info("%s PN in invalid state\n", __func__);
    gsm.open_error++;
    }
    }
//
// gsm_control_rls		-	remote line status
// @gsm: GSM channel
// @data: data bytes
// @clen: data length
//
// The modem sends us a two byte message on the control channel whenever
// it wishes to send us an error state from the virtual link. Stuff
// this into the uplink tty if present
//
#[no_mangle]
unsafe extern "C" fn gsm_control_rls(gsm: *mut gsm_mux, data: *const u8, clen: c_int) {
    static void gsm_control_rls(struct gsm_mux *gsm, const u8 *data, int clen)
    {
    struct tty_port *port;
    let mut addr: c_uint = 0;
    u8 bits;
    let mut len: c_int = clen;
    const u8 *dp = data;
    while (gsm_read_ea(&addr, *dp++) == 0) {
    len--;
    if (len == 0)
    return;
    }
// Must be at least one byte following ea
    len--;
    if (len <= 0)
    return;
    addr >>= 1;
// Closed port, or invalid ?
    if (addr == 0 || addr >= NUM_DLCI || gsm.dlci[addr] == core::ptr::null_mut())
    return;
// No error ?
    bits = *dp;
    if ((bits & 1) == 0)
    return;
    port = &gsm.dlci[addr].port;
    if (bits & 2)
    tty_insert_flip_char(port, 0, TTY_OVERRUN);
    if (bits & 4)
    tty_insert_flip_char(port, 0, TTY_PARITY);
    if (bits & 8)
    tty_insert_flip_char(port, 0, TTY_FRAME);
    tty_flip_buffer_push(port);
    gsm_control_reply(gsm, CMD_RLS, data, clen);
    }
    static void gsm_dlci_begin_close(struct gsm_dlci *dlci);
//
// gsm_control_message	-	DLCI 0 control processing
// @gsm: our GSM mux
// @command:  the command EA
// @data: data beyond the command/length EAs
// @clen: length
//
// Input processor for control messages from the other end of the link.
// Processes the incoming request and queues a response frame or an
// NSC response if not supported
//
    static void gsm_control_message(struct gsm_mux *gsm, unsigned int command,
    const u8 *data, int clen)
    {
    u8 buf[1];
    switch (command) {
    case CMD_CLD: {
    struct gsm_dlci *dlci = gsm.dlci[0];
// Modem wishes to close down
    if (dlci) {
    dlci.dead = true;
    gsm.dead = true;
    gsm_dlci_begin_close(dlci);
    }
    }
    break;
    case CMD_TEST:
// Modem wishes to test, reply with the data
    gsm_control_reply(gsm, CMD_TEST, data, clen);
    break;
    case CMD_FCON:
// Modem can accept data again
    gsm.constipated = false;
    gsm_control_reply(gsm, CMD_FCON, core::ptr::null_mut(), 0);
// Kick the link in case it is idling
    gsmld_write_trigger(gsm);
    break;
    case CMD_FCOFF:
// Modem wants us to STFU
    gsm.constipated = true;
    gsm_control_reply(gsm, CMD_FCOFF, core::ptr::null_mut(), 0);
    break;
    case CMD_MSC:
// Out of band modem line change indicator for a DLCI
    gsm_control_modem(gsm, data, clen);
    break;
    case CMD_RLS:
// Out of band error reception for a DLCI
    gsm_control_rls(gsm, data, clen);
    break;
    case CMD_PSC:
// Modem wishes to enter power saving state
    gsm_control_reply(gsm, CMD_PSC, core::ptr::null_mut(), 0);
    break;
// Optional commands
    case CMD_PN:
// Modem sends a parameter negotiation command
    gsm_control_negotiation(gsm, 1, data, clen);
    break;
// Optional unsupported commands
    case CMD_RPN:	/* Remote port negotiation */
    case CMD_SNC:	/* Service negotiation command */
    gsm.unsupported++;
    fallthrough;
    default:
// Reply to bad commands with an NSC
    buf[0] = command;
    gsm_control_reply(gsm, CMD_NSC, buf, 1);
    break;
    }
    }
//
// gsm_control_response	-	process a response to our control
// @gsm: our GSM mux
// @command: the command (response) EA
// @data: data beyond the command/length EA
// @clen: length
//
// Process a response to an outstanding command. We only allow a single
// control message in flight so this is fairly easy. All the clean up
// is done by the caller, we just update the fields, flag it as done
// and return
//
    static void gsm_control_response(struct gsm_mux *gsm, unsigned int command,
    const u8 *data, int clen)
    {
    struct gsm_control *ctrl;
    struct gsm_dlci *dlci;
    unsigned long flags;
    spin_lock_irqsave(&gsm.control_lock, flags);
    ctrl = gsm.pending_cmd;
    dlci = gsm.dlci[0];
    command |= 1;
// Does the reply match our command
    if (ctrl != core::ptr::null_mut() && (command == ctrl.cmd || command == CMD_NSC)) {
// Our command was replied to, kill the retry timer
    timer_delete(&gsm.t2_timer);
    gsm.pending_cmd = core::ptr::null_mut();
// Rejected by the other end
    if (command == CMD_NSC)
    ctrl.error = -EOPNOTSUPP;
    ctrl.done = 1;
    wake_up(&gsm.event);
// Or did we receive the PN response to our PN command
    } else if (command == CMD_PN) {
    gsm_control_negotiation(gsm, 0, data, clen);
// Or did we receive the TEST response to our TEST command
    } else if (command == CMD_TEST && clen == 1 && *data == gsm.ka_num) {
    gsm.ka_retries = -1; /* trigger new keep-alive message */
    if (dlci && !dlci.dead)
    mod_timer(&gsm.ka_timer, jiffies + gsm.keep_alive * HZ / 100);
    }
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    }
//
// gsm_control_keep_alive	-	check timeout or start keep-alive
// @t: timer contained in our gsm object
//
// Called off the keep-alive timer expiry signaling that our link
// partner is not responding anymore. Link will be closed.
// This is also called to startup our timer.
//
#[no_mangle]
unsafe extern "C" fn gsm_control_keep_alive(t: *mut timer_list) {
    static void gsm_control_keep_alive(struct timer_list *t)
    {
    struct gsm_mux *gsm = timer_container_of(gsm, t, ka_timer);
    unsigned long flags;
    spin_lock_irqsave(&gsm.control_lock, flags);
    if (gsm.ka_num && gsm.ka_retries == 0) {
// Keep-alive expired -> close the link
    if (debug & DBG_ERRORS)
    pr_debug("%s keep-alive timed out\n", __func__);
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    if (gsm.dlci[0])
    gsm_dlci_begin_close(gsm.dlci[0]);
    return;
    } else if (gsm.keep_alive && gsm.dlci[0] && !gsm.dlci[0].dead) {
    if (gsm.ka_retries > 0) {
// T2 expired for keep-alive -> resend
    gsm.ka_retries--;
    } else {
// Start keep-alive timer
    gsm.ka_num++;
    if (!gsm.ka_num)
    gsm.ka_num++;
    gsm.ka_retries = (signed int)gsm.n2;
    }
    gsm_control_command(gsm, CMD_TEST, &gsm.ka_num,
    sizeof(gsm.ka_num));
    mod_timer(&gsm.ka_timer,
    jiffies + gsm.t2 * HZ / 100);
    }
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    }
//
// gsm_control_transmit	-	send control packet
// @gsm: gsm mux
// @ctrl: frame to send
//
// Send out a pending control command (called under control lock)
//
#[no_mangle]
unsafe extern "C" fn gsm_control_transmit(gsm: *mut gsm_mux, ctrl: *mut gsm_control) {
    static void gsm_control_transmit(struct gsm_mux *gsm, struct gsm_control *ctrl)
    {
    gsm_control_command(gsm, ctrl.cmd, ctrl.data, ctrl.len);
    }
//
// gsm_control_retransmit	-	retransmit a control frame
// @t: timer contained in our gsm object
//
// Called off the T2 timer expiry in order to retransmit control frames
// that have been lost in the system somewhere. The control_lock protects
// us from colliding with another sender or a receive completion event.
// In that situation the timer may still occur in a small window but
// gsm->pending_cmd will be NULL and we just let the timer expire.
//
#[no_mangle]
unsafe extern "C" fn gsm_control_retransmit(t: *mut timer_list) {
    static void gsm_control_retransmit(struct timer_list *t)
    {
    struct gsm_mux *gsm = timer_container_of(gsm, t, t2_timer);
    struct gsm_control *ctrl;
    unsigned long flags;
    spin_lock_irqsave(&gsm.control_lock, flags);
    ctrl = gsm.pending_cmd;
    if (ctrl) {
    if (gsm.cretries == 0 || !gsm.dlci[0] || gsm.dlci[0].dead) {
    gsm.pending_cmd = core::ptr::null_mut();
    ctrl.error = -ETIMEDOUT;
    ctrl.done = 1;
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    wake_up(&gsm.event);
    return;
    }
    gsm.cretries--;
    gsm_control_transmit(gsm, ctrl);
    mod_timer(&gsm.t2_timer, jiffies + gsm.t2 * HZ / 100);
    }
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    }
//
// gsm_control_send	-	send a control frame on DLCI 0
// @gsm: the GSM channel
// @command: command  to send including CR bit
// @data: bytes of data (must be kmalloced)
// @clen: length of the block to send
//
// Queue and dispatch a control command. Only one command can be
// active at a time. In theory more can be outstanding but the matching
// gets really complicated so for now stick to one outstanding.
//
    static struct gsm_control *gsm_control_send(struct gsm_mux *gsm,
    unsigned int command, u8 *data, int clen)
    {
    struct gsm_control *ctrl = kzalloc_obj(struct gsm_control, GFP_ATOMIC);
    unsigned long flags;
    if (ctrl == core::ptr::null_mut())
    return core::ptr::null_mut();
    retry:
    wait_event(gsm.event, gsm.pending_cmd == core::ptr::null_mut());
    spin_lock_irqsave(&gsm.control_lock, flags);
    if (gsm.pending_cmd != core::ptr::null_mut()) {
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    goto retry;
    }
    ctrl.cmd = command;
    ctrl.data = data;
    ctrl.len = clen;
    gsm.pending_cmd = ctrl;
// If DLCI0 is in ADM mode skip retries, it won't respond
    if (gsm.dlci[0].mode == DLCI_MODE_ADM)
    gsm.cretries = 0;
    else
    gsm.cretries = gsm.n2;
    mod_timer(&gsm.t2_timer, jiffies + gsm.t2 * HZ / 100);
    gsm_control_transmit(gsm, ctrl);
    spin_unlock_irqrestore(&gsm.control_lock, flags);
    return ctrl;
    }
//
// gsm_control_wait	-	wait for a control to finish
// @gsm: GSM mux
// @control: control we are waiting on
//
// Waits for the control to complete or time out. Frees any used
// resources and returns 0 for success, or an error if the remote
// rejected or ignored the request.
//
#[no_mangle]
unsafe extern "C" fn gsm_control_wait(gsm: *mut gsm_mux, control: *mut gsm_control) -> c_int {
    static int gsm_control_wait(struct gsm_mux *gsm, struct gsm_control *control)
    {
    int err;
    wait_event(gsm.event, control.done == 1);
    err = control.error;
    kfree(control);
    return err;
    }
//
// DLCI level handling: Needs krefs
//
// State transitions and timers
//
// gsm_dlci_close		-	a DLCI has closed
// @dlci: DLCI that closed
//
// Perform processing when moving a DLCI into closed state. If there
// is an attached tty this is hung up
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_close(dlci: *mut gsm_dlci) {
    static void gsm_dlci_close(struct gsm_dlci *dlci)
    {
    timer_delete(&dlci.t1);
    if (debug & DBG_ERRORS)
    pr_debug("DLCI %d goes closed.\n", dlci.addr);
    dlci.state = DLCI_CLOSED;
// Prevent us from sending data before the link is up again
    dlci.constipated = true;
    if (dlci.addr != 0) {
    tty_port_tty_hangup(&dlci.port, false);
    gsm_dlci_clear_queues(dlci.gsm, dlci);
// Ensure that gsmtty_open() can return.
    tty_port_set_initialized(&dlci.port, false);
    wake_up_interruptible(&dlci.port.open_wait);
    } else {
    timer_delete(&dlci.gsm.ka_timer);
    dlci.gsm.dead = true;
    }
// A DLCI 0 close is a MUX termination so we need to kick that
    back to userspace somehow */
    gsm_dlci_data_kick(dlci);
    wake_up_all(&dlci.gsm.event);
    }
//
// gsm_dlci_open		-	a DLCI has opened
// @dlci: DLCI that opened
//
// Perform processing when moving a DLCI into open state.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_open(dlci: *mut gsm_dlci) {
    static void gsm_dlci_open(struct gsm_dlci *dlci)
    {
    struct gsm_mux *gsm = dlci.gsm;
// Note that SABM UA .. SABM UA first UA lost can mean that we go
    open . open */
    timer_delete(&dlci.t1);
// This will let a tty open continue
    dlci.state = DLCI_OPEN;
    dlci.constipated = false;
    if (debug & DBG_ERRORS)
    pr_debug("DLCI %d goes open.\n", dlci.addr);
// Send current modem state
    if (dlci.addr) {
    gsm_modem_send_initial_msc(dlci);
    } else {
// Start keep-alive control
    gsm.ka_num = 0;
    gsm.ka_retries = -1;
    mod_timer(&gsm.ka_timer,
    jiffies + gsm.keep_alive * HZ / 100);
    }
    gsm_dlci_data_kick(dlci);
    wake_up(&dlci.gsm.event);
    }
//
// gsm_dlci_negotiate	-	start parameter negotiation
// @dlci: DLCI to open
//
// Starts the parameter negotiation for the new DLCI. This needs to be done
// before the DLCI initialized the channel via SABM.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_negotiate(dlci: *mut gsm_dlci) -> c_int {
    static int gsm_dlci_negotiate(struct gsm_dlci *dlci)
    {
    struct gsm_mux *gsm = dlci.gsm;
    struct gsm_dlci_param_bits params;
    int ret;
    ret = gsm_encode_params(dlci, &params);
    if (ret != 0)
    return ret;
// We cannot asynchronous wait for the command response with
// gsm_command() and gsm_control_wait() at this point.
//
    ret = gsm_control_command(gsm, CMD_PN, (const u8 *)&params,
    sizeof(params));
    return ret;
    }
//
// gsm_dlci_t1		-	T1 timer expiry
// @t: timer contained in the DLCI that opened
//
// The T1 timer handles retransmits of control frames (essentially of
// SABM and DISC). We resend the command until the retry count runs out
// in which case an opening port goes back to closed and a closing port
// is simply put into closed state (any further frames from the other
// end will get a DM response)
//
// Some control dlci can stay in ADM mode with other dlci working just
// fine. In that case we can just keep the control dlci open after the
// DLCI_OPENING receives DM.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_t1(t: *mut timer_list) {
    static void gsm_dlci_t1(struct timer_list *t)
    {
    struct gsm_dlci *dlci = timer_container_of(dlci, t, t1);
    struct gsm_mux *gsm = dlci.gsm;
    switch (dlci.state) {
    case DLCI_CONFIGURE:
    if (dlci.retries && gsm_dlci_negotiate(dlci) == 0) {
    dlci.retries--;
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    } else {
    gsm.open_error++;
    gsm_dlci_begin_close(dlci); /* prevent half open link */
    }
    break;
    case DLCI_OPENING:
    if (!dlci.addr && gsm.control == (DM | PF)) {
    if (debug & DBG_ERRORS)
    pr_info("DLCI 0 opening in ADM mode.\n");
    dlci.mode = DLCI_MODE_ADM;
    gsm_dlci_open(dlci);
    } else if (dlci.retries) {
    if (!dlci.addr || !gsm.dlci[0] ||
    gsm.dlci[0].state != DLCI_OPENING) {
    dlci.retries--;
    gsm_command(dlci.gsm, dlci.addr, SABM|PF);
    }
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    } else {
    gsm.open_error++;
    gsm_dlci_begin_close(dlci); /* prevent half open link */
    }
    break;
    case DLCI_CLOSING:
    if (dlci.retries) {
    dlci.retries--;
    gsm_command(dlci.gsm, dlci.addr, DISC|PF);
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    } else
    gsm_dlci_close(dlci);
    break;
    default:
    pr_debug("%s: unhandled state: %d\n", __func__, dlci.state);
    break;
    }
    }
//
// gsm_dlci_begin_open	-	start channel open procedure
// @dlci: DLCI to open
//
// Commence opening a DLCI from the Linux side. We issue SABM messages
// to the modem which should then reply with a UA or ADM, at which point
// we will move into open state. Opening is done asynchronously with retry
// running off timers and the responses.
// Parameter negotiation is performed before SABM if required.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_begin_open(dlci: *mut gsm_dlci) {
    static void gsm_dlci_begin_open(struct gsm_dlci *dlci)
    {
    struct gsm_mux *gsm = dlci ? dlci.gsm : core::ptr::null_mut();
    let mut need_pn: bool = false;
    if (!gsm)
    return;
    if (dlci.addr != 0) {
    if (gsm.adaption != 1 || gsm.adaption != dlci.adaption)
    need_pn = true;
    if (dlci.prio != (roundup(dlci.addr + 1, 8) - 1))
    need_pn = true;
    if (gsm.ftype != dlci.ftype)
    need_pn = true;
    }
    switch (dlci.state) {
    case DLCI_CLOSED:
    case DLCI_WAITING_CONFIG:
    case DLCI_CLOSING:
    dlci.retries = gsm.n2;
    if (!need_pn) {
    dlci.state = DLCI_OPENING;
    if (!dlci.addr || !gsm.dlci[0] ||
    gsm.dlci[0].state != DLCI_OPENING)
    gsm_command(gsm, dlci.addr, SABM|PF);
    } else {
// Configure DLCI before setup
    dlci.state = DLCI_CONFIGURE;
    if (gsm_dlci_negotiate(dlci) != 0) {
    gsm_dlci_close(dlci);
    return;
    }
    }
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    break;
    default:
    break;
    }
    }
//
// gsm_dlci_set_opening	-	change state to opening
// @dlci: DLCI to open
//
// Change internal state to wait for DLCI open from initiator side.
// We set off timers and responses upon reception of an SABM.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_set_opening(dlci: *mut gsm_dlci) {
    static void gsm_dlci_set_opening(struct gsm_dlci *dlci)
    {
    switch (dlci.state) {
    case DLCI_CLOSED:
    case DLCI_WAITING_CONFIG:
    case DLCI_CLOSING:
    dlci.state = DLCI_OPENING;
    break;
    default:
    break;
    }
    }
//
// gsm_dlci_set_wait_config	-	wait for channel configuration
// @dlci: DLCI to configure
//
// Wait for a DLCI configuration from the application.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_set_wait_config(dlci: *mut gsm_dlci) {
    static void gsm_dlci_set_wait_config(struct gsm_dlci *dlci)
    {
    switch (dlci.state) {
    case DLCI_CLOSED:
    case DLCI_CLOSING:
    dlci.state = DLCI_WAITING_CONFIG;
    break;
    default:
    break;
    }
    }
//
// gsm_dlci_begin_close	-	start channel open procedure
// @dlci: DLCI to open
//
// Commence closing a DLCI from the Linux side. We issue DISC messages
// to the modem which should then reply with a UA, at which point we
// will move into closed state. Closing is done asynchronously with retry
// off timers. We may also receive a DM reply from the other end which
// indicates the channel was already closed.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_begin_close(dlci: *mut gsm_dlci) {
    static void gsm_dlci_begin_close(struct gsm_dlci *dlci)
    {
    struct gsm_mux *gsm = dlci.gsm;
    if (dlci.state == DLCI_CLOSED || dlci.state == DLCI_CLOSING)
    return;
    dlci.retries = gsm.n2;
    dlci.state = DLCI_CLOSING;
    gsm_command(dlci.gsm, dlci.addr, DISC|PF);
    mod_timer(&dlci.t1, jiffies + gsm.t1 * HZ / 100);
    wake_up_interruptible(&gsm.event);
    }
//
// gsm_dlci_data		-	data arrived
// @dlci: channel
// @data: block of bytes received
// @clen: length of received block
//
// A UI or UIH frame has arrived which contains data for a channel
// other than the control channel. If the relevant virtual tty is
// open we shovel the bits down it, if not we drop them.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_data(dlci: *mut gsm_dlci, data: *const u8, clen: c_int) {
    static void gsm_dlci_data(struct gsm_dlci *dlci, const u8 *data, int clen)
    {
// krefs ..
    struct tty_port *port = &dlci.port;
    struct tty_struct *tty;
    let mut modem: c_uint = 0;
    int len;
    if (debug & DBG_TTY)
    pr_debug("%d bytes for tty\n", clen);
    switch (dlci.adaption)  {
// Unsupported types
    case 4:		/* Packetised interruptible data */
    break;
    case 3:		/* Packetised uininterruptible voice/data */
    break;
    case 2:		/* Asynchronous serial with line state in each frame */
    len = gsm_read_ea_val(&modem, data, clen);
    if (len < 1)
    return;
    tty = tty_port_tty_get(port);
    if (tty) {
    gsm_process_modem(tty, dlci, modem, len);
    tty_wakeup(tty);
    tty_kref_put(tty);
    }
// Skip processed modem data
    data += len;
    clen -= len;
    fallthrough;
    case 1:		/* Line state will go via DLCI 0 controls only */
    default:
    tty_insert_flip_string(port, data, clen);
    tty_flip_buffer_push(port);
    }
    }
//
// gsm_dlci_command	-	data arrived on control channel
// @dlci: channel
// @data: block of bytes received
// @len: length of received block
//
// A UI or UIH frame has arrived which contains data for DLCI 0 the
// control channel. This should contain a command EA followed by
// control data bytes. The command EA contains a command/response bit
// and we divide up the work accordingly.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_command(dlci: *mut gsm_dlci, data: *const u8, len: c_int) {
    static void gsm_dlci_command(struct gsm_dlci *dlci, const u8 *data, int len)
    {
// See what command is involved
    let mut command: c_uint = 0;
    let mut clen: c_uint = 0;
    unsigned int dlen;
// read the command
    dlen = gsm_read_ea_val(&command, data, len);
    len -= dlen;
    data += dlen;
// read any control data
    dlen = gsm_read_ea_val(&clen, data, len);
    len -= dlen;
    data += dlen;
// Malformed command?
    if (clen > len) {
    dlci.gsm.malformed++;
    return;
    }
    if (command & 1)
    gsm_control_message(dlci.gsm, command, data, clen);
    else
    gsm_control_response(dlci.gsm, command, data, clen);
    }
//
// gsm_kick_timer	-	transmit if possible
// @t: timer contained in our gsm object
//
// Transmit data from DLCIs if the queue is empty. We can't rely on
// a tty wakeup except when we filled the pipe so we need to fire off
// new data ourselves in other cases.
//
#[no_mangle]
unsafe extern "C" fn gsm_kick_timer(t: *mut timer_list) {
    static void gsm_kick_timer(struct timer_list *t)
    {
    struct gsm_mux *gsm = timer_container_of(gsm, t, kick_timer);
    unsigned long flags;
    let mut sent: c_int = 0;
    spin_lock_irqsave(&gsm.tx_lock, flags);
// If we have nothing running then we need to fire up
    if (gsm.tx_bytes < TX_THRESH_LO)
    sent = gsm_dlci_data_sweep(gsm);
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    if (sent && debug & DBG_DATA)
    pr_info("%s TX queue stalled\n", __func__);
    }
//
// gsm_dlci_copy_config_values	-	copy DLCI configuration
// @dlci: source DLCI
// @dc: configuration structure to fill
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_copy_config_values(dlci: *mut gsm_dlci, dc: *mut gsm_dlci_config) {
    static void gsm_dlci_copy_config_values(struct gsm_dlci *dlci, struct gsm_dlci_config *dc)
    {
    memset(dc, 0, sizeof(*dc));
    dc.channel = (u32)dlci.addr;
    dc.adaption = (u32)dlci.adaption;
    dc.mtu = (u32)dlci.mtu;
    dc.priority = (u32)dlci.prio;
    if (dlci.ftype == UIH)
    dc.i = 1;
    else
    dc.i = 2;
    dc.k = (u32)dlci.k;
    }
//
// gsm_dlci_config	-	configure DLCI from configuration
// @dlci: DLCI to configure
// @dc: DLCI configuration
// @open: open DLCI after configuration?
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_config(dlci: *mut gsm_dlci, dc: *mut gsm_dlci_config, open: c_int) -> c_int {
    static int gsm_dlci_config(struct gsm_dlci *dlci, struct gsm_dlci_config *dc, int open)
    {
    struct gsm_mux *gsm;
    let mut need_restart: bool = false;
    let mut need_open: bool = false;
    unsigned int i;
//
// Check that userspace doesn't put stuff in here to prevent breakages
// in the future.
//
    for (i = 0; i < ARRAY_SIZE(dc.reserved); i++)
    if (dc.reserved[i])
    return -EINVAL;
    if (!dlci)
    return -EINVAL;
    gsm = dlci.gsm;
// Stuff we don't support yet - I frame transport
    if (dc.adaption != 1 && dc.adaption != 2)
    return -EOPNOTSUPP;
    if (dc.mtu > MAX_MTU || dc.mtu < MIN_MTU || dc.mtu > gsm.mru)
    return -EINVAL;
    if (dc.priority >= 64)
    return -EINVAL;
    if (dc.i == 0 || dc.i > 2)  /* UIH and UI only */
    return -EINVAL;
    if (dc.k > 7)
    return -EINVAL;
    if (dc.flags & ~GSM_FL_RESTART)   /* allow future extensions */
    return -EINVAL;
//
// See what is needed for reconfiguration
//
// Framing fields
    if (dc.adaption != dlci.adaption)
    need_restart = true;
    if (dc.mtu != dlci.mtu)
    need_restart = true;
    if (dc.i != dlci.ftype)
    need_restart = true;
// Requires care
    if (dc.priority != dlci.prio)
    need_restart = true;
    if (dc.flags & GSM_FL_RESTART)
    need_restart = true;
    if ((open && gsm.wait_config) || need_restart)
    need_open = true;
    if (dlci.state == DLCI_WAITING_CONFIG) {
    need_restart = false;
    need_open = true;
    }
//
// Close down what is needed, restart and initiate the new
// configuration.
//
    if (need_restart) {
    gsm_dlci_begin_close(dlci);
    wait_event_interruptible(gsm.event, dlci.state == DLCI_CLOSED);
    if (signal_pending(current))
    return -EINTR;
    }
//
// Setup the new configuration values
//
    dlci.adaption = (int)dc.adaption;
    if (dc.mtu)
    dlci.mtu = (unsigned int)dc.mtu;
    else
    dlci.mtu = gsm.mtu;
    if (dc.priority)
    dlci.prio = (u8)dc.priority;
    else
    dlci.prio = roundup(dlci.addr + 1, 8) - 1;
    if (dc.i == 1)
    dlci.ftype = UIH;
#[no_mangle]
pub unsafe extern "C" fn if(2: dc->i ==) -> else {
    else if (dc.i == 2)
    dlci.ftype = UI;
    if (dc.k)
    dlci.k = (u8)dc.k;
    else
    dlci.k = gsm.k;
    if (need_open) {
    if (gsm.initiator)
    gsm_dlci_begin_open(dlci);
    else
    gsm_dlci_set_opening(dlci);
    }
    return 0;
    }
//
// Allocate/Free DLCI channels
//
// gsm_dlci_alloc		-	allocate a DLCI
// @gsm: GSM mux
// @addr: address of the DLCI
//
// Allocate and install a new DLCI object into the GSM mux.
//
// FIXME: review locking races
//
    static struct gsm_dlci *gsm_dlci_alloc(struct gsm_mux *gsm, int addr)
    {
    struct gsm_dlci *dlci = kzalloc_obj(struct gsm_dlci, GFP_ATOMIC);
    if (dlci == core::ptr::null_mut())
    return core::ptr::null_mut();
    spin_lock_init(&dlci.lock);
    mutex_init(&dlci.mutex);
    if (kfifo_alloc(&dlci.fifo, TX_SIZE, GFP_KERNEL) < 0) {
    kfree(dlci);
    return core::ptr::null_mut();
    }
    skb_queue_head_init(&dlci.skb_list);
    timer_setup(&dlci.t1, gsm_dlci_t1, 0);
    tty_port_init(&dlci.port);
    dlci.port.ops = &gsm_port_ops;
    dlci.gsm = gsm;
    dlci.addr = addr;
    dlci.adaption = gsm.adaption;
    dlci.mtu = gsm.mtu;
    if (addr == 0)
    dlci.prio = 0;
    else
    dlci.prio = roundup(addr + 1, 8) - 1;
    dlci.ftype = gsm.ftype;
    dlci.k = gsm.k;
    dlci.state = DLCI_CLOSED;
    if (addr) {
    dlci.data = gsm_dlci_data;
// Prevent us from sending data before the link is up
    dlci.constipated = true;
    } else {
    dlci.data = gsm_dlci_command;
    }
    gsm.dlci[addr] = dlci;
    return dlci;
    }
//
// gsm_dlci_free		-	free DLCI
// @port: tty port for DLCI to free
//
// Free up a DLCI.
//
// Can sleep.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_free(port: *mut tty_port) {
    static void gsm_dlci_free(struct tty_port *port)
    {
    struct gsm_dlci *dlci = container_of(port, struct gsm_dlci, port);
    timer_shutdown_sync(&dlci.t1);
    dlci.gsm.dlci[dlci.addr] = core::ptr::null_mut();
    kfifo_free(&dlci.fifo);
    while ((dlci.skb = skb_dequeue(&dlci.skb_list)))
    dev_kfree_skb(dlci.skb);
    kfree(dlci);
    }
#[no_mangle]
pub unsafe extern "C" fn dlci_get(dlci: *mut gsm_dlci) {
    static inline void dlci_get(struct gsm_dlci *dlci)
    {
    tty_port_get(&dlci.port);
    }
#[no_mangle]
pub unsafe extern "C" fn dlci_put(dlci: *mut gsm_dlci) {
    static inline void dlci_put(struct gsm_dlci *dlci)
    {
    tty_port_put(&dlci.port);
    }
    static void gsm_destroy_network(struct gsm_dlci *dlci);
//
// gsm_dlci_release		-	release DLCI
// @dlci: DLCI to destroy
//
// Release a DLCI. Actual free is deferred until either
// mux is closed or tty is closed - whichever is last.
//
// Can sleep.
//
#[no_mangle]
unsafe extern "C" fn gsm_dlci_release(dlci: *mut gsm_dlci) {
    static void gsm_dlci_release(struct gsm_dlci *dlci)
    {
    struct tty_struct *tty = tty_port_tty_get(&dlci.port);
    if (tty) {
    mutex_lock(&dlci.mutex);
    gsm_destroy_network(dlci);
    mutex_unlock(&dlci.mutex);
// We cannot use tty_hangup() because in tty_kref_put() the tty
// driver assumes that the hangup queue is free and reuses it to
// queue release_one_tty() -> NULL pointer panic in
// process_one_work().
//
    tty_vhangup(tty);
    tty_port_tty_set(&dlci.port, core::ptr::null_mut());
    tty_kref_put(tty);
    }
    dlci.state = DLCI_CLOSED;
    dlci_put(dlci);
    }
//
// LAPBish link layer logic
//
// gsm_queue		-	a GSM frame is ready to process
// @gsm: pointer to our gsm mux
//
// At this point in time a frame has arrived and been demangled from
// the line encoding. All the differences between the encodings have
// been handled below us and the frame is unpacked into the structures.
// The fcs holds the header FCS but any data FCS must be added here.
//
#[no_mangle]
unsafe extern "C" fn gsm_queue(gsm: *mut gsm_mux) {
    static void gsm_queue(struct gsm_mux *gsm)
    {
    struct gsm_dlci *dlci;
    u8 cr;
    int address;
    if (gsm.fcs != GOOD_FCS) {
    gsm.bad_fcs++;
    if (debug & DBG_DATA)
    pr_debug("BAD FCS %02x\n", gsm.fcs);
    return;
    }
    address = gsm.address >> 1;
    if (address >= NUM_DLCI)
    goto invalid;
    cr = gsm.address & 1;		/* C/R bit */
    cr ^= gsm.initiator ? 0 : 1;	/* Flip so 1 always means command */
    gsm_print_packet("<--", address, cr, gsm.control, gsm.buf, gsm.len);
    dlci = gsm.dlci[address];
    switch (gsm.control) {
    case SABM|PF:
    if (cr == 1) {
    gsm.open_error++;
    goto invalid;
    }
    if (dlci == core::ptr::null_mut())
    dlci = gsm_dlci_alloc(gsm, address);
    if (dlci == core::ptr::null_mut()) {
    gsm.open_error++;
    return;
    }
    if (dlci.dead)
    gsm_response(gsm, address, DM|PF);
    else {
    gsm_response(gsm, address, UA|PF);
    gsm_dlci_open(dlci);
    }
    break;
    case DISC|PF:
    if (cr == 1)
    goto invalid;
    if (dlci == core::ptr::null_mut() || dlci.state == DLCI_CLOSED) {
    gsm_response(gsm, address, DM|PF);
    return;
    }
// Real close complete
    gsm_response(gsm, address, UA|PF);
    gsm_dlci_close(dlci);
    break;
    case UA|PF:
    if (cr == 0 || dlci == core::ptr::null_mut())
    break;
    switch (dlci.state) {
    case DLCI_CLOSING:
    gsm_dlci_close(dlci);
    break;
    case DLCI_OPENING:
    gsm_dlci_open(dlci);
    break;
    default:
    pr_debug("%s: unhandled state: %d\n", __func__,
    dlci.state);
    break;
    }
    break;
    case DM:	/* DM can be valid unsolicited */
    case DM|PF:
    if (cr)
    goto invalid;
    if (dlci == core::ptr::null_mut())
    return;
    gsm_dlci_close(dlci);
    break;
    case UI:
    case UI|PF:
    case UIH:
    case UIH|PF:
    if (dlci == core::ptr::null_mut() || dlci.state != DLCI_OPEN) {
    gsm_response(gsm, address, DM|PF);
    return;
    }
    dlci.data(dlci, gsm.buf, gsm.len);
    break;
    default:
    goto invalid;
    }
    return;
    invalid:
    gsm.malformed++;
    return;
    }
//
// gsm0_receive_state_check_and_fix	-	check and correct receive state
// @gsm: gsm data for this ldisc instance
//
// Ensures that the current receive state is valid for basic option mode.
//
#[no_mangle]
unsafe extern "C" fn gsm0_receive_state_check_and_fix(gsm: *mut gsm_mux) {
    static void gsm0_receive_state_check_and_fix(struct gsm_mux *gsm)
    {
    switch (gsm.state) {
    case GSM_SEARCH:
    case GSM0_ADDRESS:
    case GSM0_CONTROL:
    case GSM0_LEN0:
    case GSM0_LEN1:
    case GSM0_DATA:
    case GSM0_FCS:
    case GSM0_SSOF:
    break;
    default:
    gsm.state = GSM_SEARCH;
    break;
    }
    }
//
// gsm0_receive	-	perform processing for non-transparency
// @gsm: gsm data for this ldisc instance
// @c: character
//
// Receive bytes in gsm mode 0
//
#[no_mangle]
unsafe extern "C" fn gsm0_receive(gsm: *mut gsm_mux, c: u8) {
    static void gsm0_receive(struct gsm_mux *gsm, u8 c)
    {
    unsigned int len;
    gsm0_receive_state_check_and_fix(gsm);
    switch (gsm.state) {
    case GSM_SEARCH:	/* SOF marker */
    if (c == GSM0_SOF) {
    gsm.state = GSM0_ADDRESS;
    gsm.address = 0;
    gsm.len = 0;
    gsm.fcs = INIT_FCS;
    }
    break;
    case GSM0_ADDRESS:	/* Address EA */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    if (gsm_read_ea(&gsm.address, c))
    gsm.state = GSM0_CONTROL;
    break;
    case GSM0_CONTROL:	/* Control Byte */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    gsm.control = c;
    gsm.state = GSM0_LEN0;
    break;
    case GSM0_LEN0:		/* Length EA */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    if (gsm_read_ea(&gsm.len, c)) {
    if (gsm.len > gsm.mru) {
    gsm.bad_size++;
    gsm.state = GSM_SEARCH;
    break;
    }
    gsm.count = 0;
    if (!gsm.len)
    gsm.state = GSM0_FCS;
    else
    gsm.state = GSM0_DATA;
    break;
    }
    gsm.state = GSM0_LEN1;
    break;
    case GSM0_LEN1:
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    len = c;
    gsm.len |= len << 7;
    if (gsm.len > gsm.mru) {
    gsm.bad_size++;
    gsm.state = GSM_SEARCH;
    break;
    }
    gsm.count = 0;
    if (!gsm.len)
    gsm.state = GSM0_FCS;
    else
    gsm.state = GSM0_DATA;
    break;
    case GSM0_DATA:		/* Data */
    gsm.buf[gsm.count++] = c;
    if (gsm.count >= MAX_MRU) {
    gsm.bad_size++;
    gsm.state = GSM_SEARCH;
    } else if (gsm.count >= gsm.len) {
// Calculate final FCS for UI frames over all data
    if ((gsm.control & ~PF) != UIH) {
    gsm.fcs = gsm_fcs_add_block(gsm.fcs, gsm.buf,
    gsm.count);
    }
    gsm.state = GSM0_FCS;
    }
    break;
    case GSM0_FCS:		/* FCS follows the packet */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    gsm.state = GSM0_SSOF;
    break;
    case GSM0_SSOF:
    gsm.state = GSM_SEARCH;
    if (c == GSM0_SOF)
    gsm_queue(gsm);
    else
    gsm.bad_size++;
    break;
    default:
    pr_debug("%s: unhandled state: %d\n", __func__, gsm.state);
    break;
    }
    }
//
// gsm1_receive_state_check_and_fix	-	check and correct receive state
// @gsm: gsm data for this ldisc instance
//
// Ensures that the current receive state is valid for advanced option mode.
//
#[no_mangle]
unsafe extern "C" fn gsm1_receive_state_check_and_fix(gsm: *mut gsm_mux) {
    static void gsm1_receive_state_check_and_fix(struct gsm_mux *gsm)
    {
    switch (gsm.state) {
    case GSM_SEARCH:
    case GSM1_START:
    case GSM1_ADDRESS:
    case GSM1_CONTROL:
    case GSM1_DATA:
    case GSM1_OVERRUN:
    break;
    default:
    gsm.state = GSM_SEARCH;
    break;
    }
    }
//
// gsm1_receive	-	perform processing for non-transparency
// @gsm: gsm data for this ldisc instance
// @c: character
//
// Receive bytes in mode 1 (Advanced option)
//
#[no_mangle]
unsafe extern "C" fn gsm1_receive(gsm: *mut gsm_mux, c: u8) {
    static void gsm1_receive(struct gsm_mux *gsm, u8 c)
    {
    gsm1_receive_state_check_and_fix(gsm);
// handle XON/XOFF
    if ((c & ISO_IEC_646_MASK) == XON) {
    gsm.constipated = true;
    return;
    } else if ((c & ISO_IEC_646_MASK) == XOFF) {
    gsm.constipated = false;
// Kick the link in case it is idling
    gsmld_write_trigger(gsm);
    return;
    }
    if (c == GSM1_SOF) {
// EOF is only valid in frame if we have got to the data state
    if (gsm.state == GSM1_DATA) {
    if (gsm.count < 1) {
// Missing FSC
    gsm.malformed++;
    gsm.state = GSM1_START;
    return;
    }
// Remove the FCS from data
    gsm.count--;
    if ((gsm.control & ~PF) != UIH) {
// Calculate final FCS for UI frames over all
// data but FCS
//
    gsm.fcs = gsm_fcs_add_block(gsm.fcs, gsm.buf,
    gsm.count);
    }
// Add the FCS itself to test against GOOD_FCS
    gsm.fcs = gsm_fcs_add(gsm.fcs, gsm.buf[gsm.count]);
    gsm.len = gsm.count;
    gsm_queue(gsm);
    gsm.state  = GSM1_START;
    return;
    }
// Any partial frame was a runt so go back to start
    if (gsm.state != GSM1_START) {
    if (gsm.state != GSM_SEARCH)
    gsm.malformed++;
    gsm.state = GSM1_START;
    }
// A SOF in GSM_START means we are still reading idling or
    framing bytes */
    return;
    }
    if (c == GSM1_ESCAPE) {
    gsm.escape = true;
    return;
    }
// Only an unescaped SOF gets us out of GSM search
    if (gsm.state == GSM_SEARCH)
    return;
    if (gsm.escape) {
    c ^= GSM1_ESCAPE_BITS;
    gsm.escape = false;
    }
    switch (gsm.state) {
    case GSM1_START:		/* First byte after SOF */
    gsm.address = 0;
    gsm.state = GSM1_ADDRESS;
    gsm.fcs = INIT_FCS;
    fallthrough;
    case GSM1_ADDRESS:	/* Address continuation */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    if (gsm_read_ea(&gsm.address, c))
    gsm.state = GSM1_CONTROL;
    break;
    case GSM1_CONTROL:	/* Control Byte */
    gsm.fcs = gsm_fcs_add(gsm.fcs, c);
    gsm.control = c;
    gsm.count = 0;
    gsm.state = GSM1_DATA;
    break;
    case GSM1_DATA:		/* Data */
    if (gsm.count > gsm.mru || gsm.count > MAX_MRU) {	/* Allow one for the FCS */
    gsm.state = GSM1_OVERRUN;
    gsm.bad_size++;
    } else
    gsm.buf[gsm.count++] = c;
    break;
    case GSM1_OVERRUN:	/* Over-long - eg a dropped SOF */
    break;
    default:
    pr_debug("%s: unhandled state: %d\n", __func__, gsm.state);
    break;
    }
    }
//
// gsm_error		-	handle tty error
// @gsm: ldisc data
//
// Handle an error in the receipt of data for a frame. Currently we just
// go back to hunting for a SOF.
//
// FIXME: better diagnostics ?
//
#[no_mangle]
unsafe extern "C" fn gsm_error(gsm: *mut gsm_mux) {
    static void gsm_error(struct gsm_mux *gsm)
    {
    gsm.state = GSM_SEARCH;
    gsm.io_error++;
    }
//
// gsm_cleanup_mux		-	generic GSM protocol cleanup
// @gsm: our mux
// @disc: disconnect link?
//
// Clean up the bits of the mux which are the same for all framing
// protocols. Remove the mux from the mux table, stop all the timers
// and then shut down each device hanging up the channels as we go.
//
#[no_mangle]
unsafe extern "C" fn gsm_cleanup_mux(gsm: *mut gsm_mux, disc: bool) {
    static void gsm_cleanup_mux(struct gsm_mux *gsm, bool disc)
    {
    int i;
    struct gsm_dlci *dlci;
    struct gsm_msg *txq, *ntxq;
    gsm.dead = true;
    mutex_lock(&gsm.mutex);
    dlci = gsm.dlci[0];
    if (dlci) {
    if (disc && dlci.state != DLCI_CLOSED) {
    gsm_dlci_begin_close(dlci);
    wait_event(gsm.event, dlci.state == DLCI_CLOSED);
    }
    dlci.dead = true;
    }
// Finish outstanding timers, making sure they are done
    timer_delete_sync(&gsm.kick_timer);
    timer_delete_sync(&gsm.t2_timer);
    timer_delete_sync(&gsm.ka_timer);
// Finish writing to ldisc
    flush_work(&gsm.tx_work);
// Free up any link layer users and finally the control channel
    if (gsm.has_devices) {
    gsm_unregister_devices(gsm_tty_driver, gsm.num);
    gsm.has_devices = false;
    }
    for (i = NUM_DLCI - 1; i >= 0; i--)
    if (gsm.dlci[i])
    gsm_dlci_release(gsm.dlci[i]);
    mutex_unlock(&gsm.mutex);
// Now wipe the queues
    tty_ldisc_flush(gsm.tty);
    guard(spinlock_irqsave)(&gsm.tx_lock);
    list_for_each_entry_safe(txq, ntxq, &gsm.tx_ctrl_list, list)
    kfree(txq);
    INIT_LIST_HEAD(&gsm.tx_ctrl_list);
    list_for_each_entry_safe(txq, ntxq, &gsm.tx_data_list, list)
    kfree(txq);
    INIT_LIST_HEAD(&gsm.tx_data_list);
    }
//
// gsm_activate_mux	-	generic GSM setup
// @gsm: our mux
//
// Set up the bits of the mux which are the same for all framing
// protocols. Add the mux to the mux table so it can be opened and
// finally kick off connecting to DLCI 0 on the modem.
//
#[no_mangle]
unsafe extern "C" fn gsm_activate_mux(gsm: *mut gsm_mux) -> c_int {
    static int gsm_activate_mux(struct gsm_mux *gsm)
    {
    struct gsm_dlci *dlci;
    int ret;
    dlci = gsm_dlci_alloc(gsm, 0);
    if (dlci == core::ptr::null_mut())
    return -ENOMEM;
    if (gsm.encoding == GSM_BASIC_OPT)
    gsm.receive = gsm0_receive;
    else
    gsm.receive = gsm1_receive;
    ret = gsm_register_devices(gsm_tty_driver, gsm.num);
    if (ret)
    return ret;
    gsm.has_devices = true;
    gsm.dead = false;		/* Tty opens are now permissible */
    return 0;
    }
//
// gsm_free_mux		-	free up a mux
// @gsm: mux to free
//
// Dispose of allocated resources for a dead mux
//
#[no_mangle]
unsafe extern "C" fn gsm_free_mux(gsm: *mut gsm_mux) {
    static void gsm_free_mux(struct gsm_mux *gsm)
    {
    int i;
    for (i = 0; i < MAX_MUX; i++) {
    if (gsm == gsm_mux[i]) {
    gsm_mux[i] = core::ptr::null_mut();
    break;
    }
    }
    mutex_destroy(&gsm.mutex);
    kfree(gsm.txframe);
    kfree(gsm.buf);
    kfree(gsm);
    }
//
// gsm_free_muxr		-	free up a mux
// @ref: kreference to the mux to free
//
// Dispose of allocated resources for a dead mux
//
#[no_mangle]
unsafe extern "C" fn gsm_free_muxr(ref: *mut kref) {
    static void gsm_free_muxr(struct kref *ref)
    {
    struct gsm_mux *gsm = container_of(ref, struct gsm_mux, ref);
    gsm_free_mux(gsm);
    }
#[no_mangle]
pub unsafe extern "C" fn mux_get(gsm: *mut gsm_mux) {
    static inline void mux_get(struct gsm_mux *gsm)
    {
    unsigned long flags;
    spin_lock_irqsave(&gsm_mux_lock, flags);
    kref_get(&gsm.ref);
    spin_unlock_irqrestore(&gsm_mux_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn mux_put(gsm: *mut gsm_mux) {
    static inline void mux_put(struct gsm_mux *gsm)
    {
    unsigned long flags;
    spin_lock_irqsave(&gsm_mux_lock, flags);
    kref_put(&gsm.ref, gsm_free_muxr);
    spin_unlock_irqrestore(&gsm_mux_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn mux_num_to_base(gsm: *mut gsm_mux) -> c_uint {
    static inline unsigned int mux_num_to_base(struct gsm_mux *gsm)
    {
    return gsm.num * NUM_DLCI;
    }
#[no_mangle]
pub unsafe extern "C" fn mux_line_to_num(line: c_uint) -> c_uint {
    static inline unsigned int mux_line_to_num(unsigned int line)
    {
    return line / NUM_DLCI;
    }
//
// gsm_alloc_mux		-	allocate a mux
//
// Creates a new mux ready for activation.
//
    static struct gsm_mux *gsm_alloc_mux(void)
    {
    int i;
    struct gsm_mux *gsm = kzalloc_obj(struct gsm_mux);
    if (gsm == core::ptr::null_mut())
    return core::ptr::null_mut();
    gsm.buf = kmalloc(MAX_MRU + 1, GFP_KERNEL);
    if (gsm.buf == core::ptr::null_mut()) {
    kfree(gsm);
    return core::ptr::null_mut();
    }
    gsm.txframe = kmalloc(2 * (MAX_MTU + PROT_OVERHEAD - 1), GFP_KERNEL);
    if (gsm.txframe == core::ptr::null_mut()) {
    kfree(gsm.buf);
    kfree(gsm);
    return core::ptr::null_mut();
    }
    spin_lock_init(&gsm.lock);
    mutex_init(&gsm.mutex);
    kref_init(&gsm.ref);
    INIT_LIST_HEAD(&gsm.tx_ctrl_list);
    INIT_LIST_HEAD(&gsm.tx_data_list);
    timer_setup(&gsm.kick_timer, gsm_kick_timer, 0);
    timer_setup(&gsm.t2_timer, gsm_control_retransmit, 0);
    timer_setup(&gsm.ka_timer, gsm_control_keep_alive, 0);
    INIT_WORK(&gsm.tx_work, gsmld_write_task);
    init_waitqueue_head(&gsm.event);
    spin_lock_init(&gsm.control_lock);
    spin_lock_init(&gsm.tx_lock);
    gsm.t1 = T1;
    gsm.t2 = T2;
    gsm.t3 = T3;
    gsm.n2 = N2;
    gsm.k = K;
    gsm.ftype = UIH;
    gsm.adaption = 1;
    gsm.encoding = GSM_ADV_OPT;
    gsm.mru = 64;	/* Default to encoding 1 so these should be 64 */
    gsm.mtu = 64;
    gsm.dead = true;	/* Avoid early tty opens */
    gsm.wait_config = false; /* Disabled */
    gsm.keep_alive = 0;	/* Disabled */
// Store the instance to the mux array or abort if no space is
// available.
//
    spin_lock(&gsm_mux_lock);
    for (i = 0; i < MAX_MUX; i++) {
    if (!gsm_mux[i]) {
    gsm_mux[i] = gsm;
    gsm.num = i;
    break;
    }
    }
    spin_unlock(&gsm_mux_lock);
    if (i == MAX_MUX) {
    mutex_destroy(&gsm.mutex);
    kfree(gsm.txframe);
    kfree(gsm.buf);
    kfree(gsm);
    return core::ptr::null_mut();
    }
    return gsm;
    }
    static void gsm_copy_config_values(struct gsm_mux *gsm,
    struct gsm_config *c)
    {
    memset(c, 0, sizeof(*c));
    c.adaption = gsm.adaption;
    c.encapsulation = gsm.encoding;
    c.initiator = gsm.initiator;
    c.t1 = gsm.t1;
    c.t2 = gsm.t2;
    c.t3 = gsm.t3;
    c.n2 = gsm.n2;
    if (gsm.ftype == UIH)
    c.i = 1;
    else
    c.i = 2;
    pr_debug("Ftype %d i %d\n", gsm.ftype, c.i);
    c.mru = gsm.mru;
    c.mtu = gsm.mtu;
    c.k = gsm.k;
    }
#[no_mangle]
unsafe extern "C" fn gsm_config(gsm: *mut gsm_mux, c: *mut gsm_config) -> c_int {
    static int gsm_config(struct gsm_mux *gsm, struct gsm_config *c)
    {
    let mut need_close: c_int = 0;
    let mut need_restart: c_int = 0;
// Stuff we don't support yet - UI or I frame transport
    if (c.adaption != 1 && c.adaption != 2)
    return -EOPNOTSUPP;
// Check the MRU/MTU range looks sane
    if (c.mru < MIN_MTU || c.mtu < MIN_MTU)
    return -EINVAL;
    if (c.mru > MAX_MRU || c.mtu > MAX_MTU)
    return -EINVAL;
    if (c.t3 > MAX_T3)
    return -EINVAL;
    if (c.n2 > 255)
    return -EINVAL;
    if (c.encapsulation > 1)	/* Basic, advanced, no I */
    return -EINVAL;
    if (c.initiator > 1)
    return -EINVAL;
    if (c.k > MAX_WINDOW_SIZE)
    return -EINVAL;
    if (c.i == 0 || c.i > 2)	/* UIH and UI only */
    return -EINVAL;
//
// See what is needed for reconfiguration
//
// Timing fields
    if (c.t1 != 0 && c.t1 != gsm.t1)
    need_restart = 1;
    if (c.t2 != 0 && c.t2 != gsm.t2)
    need_restart = 1;
    if (c.encapsulation != gsm.encoding)
    need_restart = 1;
    if (c.adaption != gsm.adaption)
    need_restart = 1;
// Requires care
    if (c.initiator != gsm.initiator)
    need_close = 1;
    if (c.mru != gsm.mru)
    need_restart = 1;
    if (c.mtu != gsm.mtu)
    need_restart = 1;
//
// Close down what is needed, restart and initiate the new
// configuration. On the first time there is no DLCI[0]
// and closing or cleaning up is not necessary.
//
    if (need_close || need_restart)
    gsm_cleanup_mux(gsm, true);
    gsm.initiator = c.initiator;
    gsm.mru = c.mru;
    gsm.mtu = c.mtu;
    gsm.encoding = c.encapsulation ? GSM_ADV_OPT : GSM_BASIC_OPT;
    gsm.adaption = c.adaption;
    gsm.n2 = c.n2;
    if (c.i == 1)
    gsm.ftype = UIH;
#[no_mangle]
pub unsafe extern "C" fn if(2: c->i ==) -> else {
    else if (c.i == 2)
    gsm.ftype = UI;
    if (c.t1)
    gsm.t1 = c.t1;
    if (c.t2)
    gsm.t2 = c.t2;
    if (c.t3)
    gsm.t3 = c.t3;
    if (c.k)
    gsm.k = c.k;
//
// FIXME: We need to separate activation/deactivation from adding
// and removing from the mux array
//
    if (gsm.dead) {
    let mut ret: c_int = gsm_activate_mux(gsm);
    if (ret)
    return ret;
    if (gsm.initiator)
    gsm_dlci_begin_open(gsm.dlci[0]);
    }
    return 0;
    }
    static void gsm_copy_config_ext_values(struct gsm_mux *gsm,
    struct gsm_config_ext *ce)
    {
    memset(ce, 0, sizeof(*ce));
    ce.wait_config = gsm.wait_config ? 1 : 0;
    ce.keep_alive = gsm.keep_alive;
    }
#[no_mangle]
unsafe extern "C" fn gsm_config_ext(gsm: *mut gsm_mux, ce: *mut gsm_config_ext) -> c_int {
    static int gsm_config_ext(struct gsm_mux *gsm, struct gsm_config_ext *ce)
    {
    let mut need_restart: bool = false;
    unsigned int i;
//
// Check that userspace doesn't put stuff in here to prevent breakages
// in the future.
//
    for (i = 0; i < ARRAY_SIZE(ce.reserved); i++)
    if (ce.reserved[i])
    return -EINVAL;
    if (ce.flags & ~GSM_FL_RESTART)
    return -EINVAL;
// Requires care
    if (ce.flags & GSM_FL_RESTART)
    need_restart = true;
//
// Close down what is needed, restart and initiate the new
// configuration. On the first time there is no DLCI[0]
// and closing or cleaning up is not necessary.
//
    if (need_restart)
    gsm_cleanup_mux(gsm, true);
//
// Setup the new configuration values
//
    gsm.wait_config = ce.wait_config ? true : false;
    gsm.keep_alive = ce.keep_alive;
    if (gsm.dead) {
    let mut ret: c_int = gsm_activate_mux(gsm);
    if (ret)
    return ret;
    if (gsm.initiator)
    gsm_dlci_begin_open(gsm.dlci[0]);
    }
    return 0;
    }
//
// gsmld_output		-	write to link
// @gsm: our mux
// @data: bytes to output
// @len: size
//
// Write a block of data from the GSM mux to the data channel. This
// will eventually be serialized from above but at the moment isn't.
//
#[no_mangle]
unsafe extern "C" fn gsmld_output(gsm: *mut gsm_mux, data: *mut u8, len: c_int) -> c_int {
    static int gsmld_output(struct gsm_mux *gsm, u8 *data, int len)
    {
    if (tty_write_room(gsm.tty) < len) {
    set_bit(TTY_DO_WRITE_WAKEUP, &gsm.tty.flags);
    return -ENOSPC;
    }
    if (debug & DBG_DATA)
    gsm_hex_dump_bytes(__func__, data, len);
    return gsm.tty.ops.write(gsm.tty, data, len);
    }
//
// gsmld_write_trigger	-	schedule ldisc write task
// @gsm: our mux
//
#[no_mangle]
unsafe extern "C" fn gsmld_write_trigger(gsm: *mut gsm_mux) {
    static void gsmld_write_trigger(struct gsm_mux *gsm)
    {
    if (!gsm || !gsm.dlci[0] || gsm.dlci[0].dead)
    return;
    schedule_work(&gsm.tx_work);
    }
//
// gsmld_write_task	-	ldisc write task
// @work: our tx write work
//
// Writes out data to the ldisc if possible. We are doing this here to
// avoid dead-locking. This returns if no space or data is left for output.
//
#[no_mangle]
unsafe extern "C" fn gsmld_write_task(work: *mut work_struct) {
    static void gsmld_write_task(struct work_struct *work)
    {
    struct gsm_mux *gsm = container_of(work, struct gsm_mux, tx_work);
    unsigned long flags;
    int i, ret;
// All outstanding control channel and control messages and one data
// frame is sent.
//
    ret = -ENODEV;
    spin_lock_irqsave(&gsm.tx_lock, flags);
    if (gsm.tty)
    ret = gsm_data_kick(gsm);
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    if (ret >= 0)
    for (i = 0; i < NUM_DLCI; i++)
    if (gsm.dlci[i])
    tty_port_tty_wakeup(&gsm.dlci[i].port);
    }
//
// gsmld_attach_gsm	-	mode set up
// @tty: our tty structure
// @gsm: our mux
//
// Set up the MUX for basic mode and commence connecting to the
// modem. Currently called from the line discipline set up but
// will need moving to an ioctl path.
//
#[no_mangle]
unsafe extern "C" fn gsmld_attach_gsm(tty: *mut tty_struct, gsm: *mut gsm_mux) {
    static void gsmld_attach_gsm(struct tty_struct *tty, struct gsm_mux *gsm)
    {
    gsm.tty = tty_kref_get(tty);
// Turn off tty XON/XOFF handling to handle it explicitly.
    gsm.old_c_iflag = tty.termios.c_iflag;
    tty.termios.c_iflag &= (IXON | IXOFF);
    }
//
// gsmld_detach_gsm	-	stop doing 0710 mux
// @tty: tty attached to the mux
// @gsm: mux
//
// Shutdown and then clean up the resources used by the line discipline
//
#[no_mangle]
unsafe extern "C" fn gsmld_detach_gsm(tty: *mut tty_struct, gsm: *mut gsm_mux) {
    static void gsmld_detach_gsm(struct tty_struct *tty, struct gsm_mux *gsm)
    {
    WARN_ON(tty != gsm.tty);
// Restore tty XON/XOFF handling.
    gsm.tty.termios.c_iflag = gsm.old_c_iflag;
    tty_kref_put(gsm.tty);
    gsm.tty = core::ptr::null_mut();
    }
    static void gsmld_receive_buf(struct tty_struct *tty, const u8 *cp,
    const u8 *fp, size_t count)
    {
    struct gsm_mux *gsm = tty.disc_data;
    let mut flags: u8 = TTY_NORMAL;
    if (debug & DBG_DATA)
    gsm_hex_dump_bytes(__func__, cp, count);
    for (; count; count--, cp++) {
    if (fp)
    flags = *fp++;
    switch (flags) {
    case TTY_NORMAL:
    if (gsm.receive)
    gsm.receive(gsm, *cp);
    break;
    case TTY_OVERRUN:
    case TTY_BREAK:
    case TTY_PARITY:
    case TTY_FRAME:
    gsm_error(gsm);
    break;
    default:
    WARN_ONCE(1, "%s: unknown flag %d\n",
    tty_name(tty), flags);
    break;
    }
    }
// FASYNC if needed ?
// If clogged call tty_throttle(tty);
    }
//
// gsmld_flush_buffer	-	clean input queue
// @tty:	terminal device
//
// Flush the input buffer. Called when the line discipline is
// being closed, when the tty layer wants the buffer flushed (eg
// at hangup).
//
#[no_mangle]
unsafe extern "C" fn gsmld_flush_buffer(tty: *mut tty_struct) {
    static void gsmld_flush_buffer(struct tty_struct *tty)
    {
    }
//
// gsmld_close		-	close the ldisc for this tty
// @tty: device
//
// Called from the terminal layer when this line discipline is
// being shut down, either because of a close or becsuse of a
// discipline change. The function will not be called while other
// ldisc methods are in progress.
//
#[no_mangle]
unsafe extern "C" fn gsmld_close(tty: *mut tty_struct) {
    static void gsmld_close(struct tty_struct *tty)
    {
    struct gsm_mux *gsm = tty.disc_data;
// The ldisc locks and closes the port before calling our close. This
// means we have no way to do a proper disconnect. We will not bother
// to do one.
//
    gsm_cleanup_mux(gsm, false);
    gsmld_detach_gsm(tty, gsm);
    gsmld_flush_buffer(tty);
// Do other clean up here
    mux_put(gsm);
    }
//
// gsmld_open		-	open an ldisc
// @tty: terminal to open
//
// Called when this line discipline is being attached to the
// terminal device. Can sleep. Called serialized so that no
// other events will occur in parallel. No further open will occur
// until a close.
//
#[no_mangle]
unsafe extern "C" fn gsmld_open(tty: *mut tty_struct) -> c_int {
    static int gsmld_open(struct tty_struct *tty)
    {
    struct gsm_mux *gsm;
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    if (tty.ops.write == core::ptr::null_mut())
    return -EINVAL;
// Attach our ldisc data
    gsm = gsm_alloc_mux();
    if (gsm == core::ptr::null_mut())
    return -ENOMEM;
    tty.disc_data = gsm;
    tty.receive_room = 65536;
// Attach the initial passive connection
    gsmld_attach_gsm(tty, gsm);
// The mux will not be activated yet, we wait for correct
// configuration first.
//
    if (gsm.encoding == GSM_BASIC_OPT)
    gsm.receive = gsm0_receive;
    else
    gsm.receive = gsm1_receive;
    return 0;
    }
//
// gsmld_write_wakeup	-	asynchronous I/O notifier
// @tty: tty device
//
// Required for the ptys, serial driver etc. since processes
// that attach themselves to the master and rely on ASYNC
// IO must be woken up
//
#[no_mangle]
unsafe extern "C" fn gsmld_write_wakeup(tty: *mut tty_struct) {
    static void gsmld_write_wakeup(struct tty_struct *tty)
    {
    struct gsm_mux *gsm = tty.disc_data;
// Queue poll
    gsmld_write_trigger(gsm);
    }
//
// gsmld_read		-	read function for tty
// @tty: tty device
// @file: file object
// @buf: userspace buffer pointer
// @nr: size of I/O
// @cookie: unused
// @offset: unused
//
// Perform reads for the line discipline. We are guaranteed that the
// line discipline will not be closed under us but we may get multiple
// parallel readers and must handle this ourselves. We may also get
// a hangup. Always called in user context, may sleep.
//
// This code must be sure never to sleep through a hangup.
//
    static ssize_t gsmld_read(struct tty_struct *tty, struct file *file, u8 *buf,
    size_t nr, void **cookie, unsigned long offset)
    {
    return -EOPNOTSUPP;
    }
//
// gsmld_write		-	write function for tty
// @tty: tty device
// @file: file object
// @buf: userspace buffer pointer
// @nr: size of I/O
//
// Called when the owner of the device wants to send a frame
// itself (or some other control data). The data is transferred
// as-is and must be properly framed and checksummed as appropriate
// by userspace. Frames are either sent whole or not at all as this
// avoids pain user side.
//
    static ssize_t gsmld_write(struct tty_struct *tty, struct file *file,
    const u8 *buf, size_t nr)
    {
    struct gsm_mux *gsm = tty.disc_data;
    unsigned long flags;
    size_t space;
    int ret;
    if (!gsm)
    return -ENODEV;
    ret = -ENOBUFS;
    spin_lock_irqsave(&gsm.tx_lock, flags);
    space = tty_write_room(tty);
    if (space >= nr)
    ret = tty.ops.write(tty, buf, nr);
    else
    set_bit(TTY_DO_WRITE_WAKEUP, &tty.flags);
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    return ret;
    }
//
// gsmld_poll		-	poll method for N_GSM0710
// @tty: terminal device
// @file: file accessing it
// @wait: poll table
//
// Called when the line discipline is asked to poll() for data or
// for special events. This code is not serialized with respect to
// other events save open/close.
//
// This code must be sure never to sleep through a hangup.
// Called without the kernel lock held - fine
//
    static __poll_t gsmld_poll(struct tty_struct *tty, struct file *file,
    poll_table *wait)
    {
    let mut mask: __poll_t = 0;
    struct gsm_mux *gsm = tty.disc_data;
    poll_wait(file, &tty.read_wait, wait);
    poll_wait(file, &tty.write_wait, wait);
    if (gsm.dead)
    mask |= EPOLLHUP;
    if (tty_hung_up_p(file))
    mask |= EPOLLHUP;
    if (test_bit(TTY_OTHER_CLOSED, &tty.flags))
    mask |= EPOLLHUP;
    if (!tty_is_writelocked(tty) && tty_write_room(tty) > 0)
    mask |= EPOLLOUT | EPOLLWRNORM;
    return mask;
    }
    static int gsmld_ioctl(struct tty_struct *tty, unsigned int cmd,
    unsigned long arg)
    {
    struct gsm_config c;
    struct gsm_config_ext ce;
    struct gsm_dlci_config dc;
    struct gsm_mux *gsm = tty.disc_data;
    unsigned int base, addr;
    struct gsm_dlci *dlci;
    switch (cmd) {
    case GSMIOC_GETCONF:
    gsm_copy_config_values(gsm, &c);
    if (copy_to_user((void __user *)arg, &c, sizeof(c)))
    return -EFAULT;
    return 0;
    case GSMIOC_SETCONF:
    if (copy_from_user(&c, (void __user *)arg, sizeof(c)))
    return -EFAULT;
    return gsm_config(gsm, &c);
    case GSMIOC_GETFIRST:
    base = mux_num_to_base(gsm);
    return put_user(base + 1, (__u32 __user *)arg);
    case GSMIOC_GETCONF_EXT:
    gsm_copy_config_ext_values(gsm, &ce);
    if (copy_to_user((void __user *)arg, &ce, sizeof(ce)))
    return -EFAULT;
    return 0;
    case GSMIOC_SETCONF_EXT:
    if (copy_from_user(&ce, (void __user *)arg, sizeof(ce)))
    return -EFAULT;
    return gsm_config_ext(gsm, &ce);
    case GSMIOC_GETCONF_DLCI:
    if (copy_from_user(&dc, (void __user *)arg, sizeof(dc)))
    return -EFAULT;
    if (dc.channel == 0 || dc.channel >= NUM_DLCI)
    return -EINVAL;
    addr = array_index_nospec(dc.channel, NUM_DLCI);
    dlci = gsm.dlci[addr];
    if (!dlci) {
    dlci = gsm_dlci_alloc(gsm, addr);
    if (!dlci)
    return -ENOMEM;
    }
    gsm_dlci_copy_config_values(dlci, &dc);
    if (copy_to_user((void __user *)arg, &dc, sizeof(dc)))
    return -EFAULT;
    return 0;
    case GSMIOC_SETCONF_DLCI:
    if (copy_from_user(&dc, (void __user *)arg, sizeof(dc)))
    return -EFAULT;
    if (dc.channel == 0 || dc.channel >= NUM_DLCI)
    return -EINVAL;
    addr = array_index_nospec(dc.channel, NUM_DLCI);
    dlci = gsm.dlci[addr];
    if (!dlci) {
    dlci = gsm_dlci_alloc(gsm, addr);
    if (!dlci)
    return -ENOMEM;
    }
    return gsm_dlci_config(dlci, &dc, 0);
    default:
    return n_tty_ioctl_helper(tty, cmd, arg);
    }
    }
//
// Network interface
//
#[no_mangle]
unsafe extern "C" fn gsm_mux_net_open(net: *mut net_device) -> c_int {
    static int gsm_mux_net_open(struct net_device *net)
    {
    pr_debug("%s called\n", __func__);
    netif_start_queue(net);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gsm_mux_net_close(net: *mut net_device) -> c_int {
    static int gsm_mux_net_close(struct net_device *net)
    {
    netif_stop_queue(net);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dlci_net_free(dlci: *mut gsm_dlci) {
    static void dlci_net_free(struct gsm_dlci *dlci)
    {
    if (!dlci.net) {
    WARN_ON(1);
    return;
    }
    dlci.adaption = dlci.prev_adaption;
    dlci.data = dlci.prev_data;
    free_netdev(dlci.net);
    dlci.net = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn net_free(ref: *mut kref) {
    static void net_free(struct kref *ref)
    {
    struct gsm_mux_net *mux_net;
    struct gsm_dlci *dlci;
    mux_net = container_of(ref, struct gsm_mux_net, ref);
    dlci = mux_net.dlci;
    if (dlci.net) {
    unregister_netdev(dlci.net);
    dlci_net_free(dlci);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn muxnet_get(mux_net: *mut gsm_mux_net) {
    static inline void muxnet_get(struct gsm_mux_net *mux_net)
    {
    kref_get(&mux_net.ref);
    }
#[no_mangle]
pub unsafe extern "C" fn muxnet_put(mux_net: *mut gsm_mux_net) {
    static inline void muxnet_put(struct gsm_mux_net *mux_net)
    {
    kref_put(&mux_net.ref, net_free);
    }
    static netdev_tx_t gsm_mux_net_start_xmit(struct sk_buff *skb,
    struct net_device *net)
    {
    struct gsm_mux_net *mux_net = netdev_priv(net);
    struct gsm_dlci *dlci = mux_net.dlci;
    muxnet_get(mux_net);
    skb_queue_head(&dlci.skb_list, skb);
    net.stats.tx_packets++;
    net.stats.tx_bytes += skb.len;
    gsm_dlci_data_kick(dlci);
// And tell the kernel when the last transmit started.
    netif_trans_update(net);
    muxnet_put(mux_net);
    return NETDEV_TX_OK;
    }
// called when a packet did not ack after watchdogtimeout
#[no_mangle]
unsafe extern "C" fn gsm_mux_net_tx_timeout(net: *mut net_device, txqueue: c_uint) {
    static void gsm_mux_net_tx_timeout(struct net_device *net, unsigned int txqueue)
    {
// Tell syslog we are hosed.
    dev_dbg(&net.dev, "Tx timed out.\n");
// Update statistics
    net.stats.tx_errors++;
    }
#[no_mangle]
unsafe extern "C" fn gsm_mux_rx_netchar(dlci: *mut gsm_dlci, in_buf: *const u8, size: c_int) {
    static void gsm_mux_rx_netchar(struct gsm_dlci *dlci, const u8 *in_buf, int size)
    {
    struct net_device *net = dlci.net;
    struct sk_buff *skb;
    struct gsm_mux_net *mux_net = netdev_priv(net);
    muxnet_get(mux_net);
// Allocate an sk_buff
    skb = dev_alloc_skb(size + NET_IP_ALIGN);
    if (!skb) {
// We got no receive buffer.
    net.stats.rx_dropped++;
    muxnet_put(mux_net);
    return;
    }
    skb_reserve(skb, NET_IP_ALIGN);
    skb_put_data(skb, in_buf, size);
    skb.dev = net;
    skb.protocol = htons(ETH_P_IP);
// Ship it off to the kernel
    netif_rx(skb);
// update out statistics
    net.stats.rx_packets++;
    net.stats.rx_bytes += size;
    muxnet_put(mux_net);
    return;
    }
#[no_mangle]
unsafe extern "C" fn gsm_mux_net_init(net: *mut net_device) {
    static void gsm_mux_net_init(struct net_device *net)
    {
    static const struct net_device_ops gsm_netdev_ops = {
    .ndo_open		= gsm_mux_net_open,
    .ndo_stop		= gsm_mux_net_close,
    .ndo_start_xmit		= gsm_mux_net_start_xmit,
    .ndo_tx_timeout		= gsm_mux_net_tx_timeout,
    };
    net.netdev_ops = &gsm_netdev_ops;
// fill in the other fields
    net.watchdog_timeo = GSM_NET_TX_TIMEOUT;
    net.flags = IFF_POINTOPOINT | IFF_NOARP | IFF_MULTICAST;
    net.type = ARPHRD_NONE;
    net.tx_queue_len = 10;
    }
// caller holds the dlci mutex
#[no_mangle]
unsafe extern "C" fn gsm_destroy_network(dlci: *mut gsm_dlci) {
    static void gsm_destroy_network(struct gsm_dlci *dlci)
    {
    struct gsm_mux_net *mux_net;
    pr_debug("destroy network interface\n");
    if (!dlci.net)
    return;
    mux_net = netdev_priv(dlci.net);
    muxnet_put(mux_net);
    }
// caller holds the dlci mutex
#[no_mangle]
unsafe extern "C" fn gsm_create_network(dlci: *mut gsm_dlci, nc: *mut gsm_netconfig) -> c_int {
    static int gsm_create_network(struct gsm_dlci *dlci, struct gsm_netconfig *nc)
    {
    char *netname;
    let mut retval: c_int = 0;
    struct net_device *net;
    struct gsm_mux_net *mux_net;
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
// Already in a non tty mode
    if (dlci.adaption > 2)
    return -EBUSY;
    if (nc.protocol != htons(ETH_P_IP))
    return -EPROTONOSUPPORT;
    if (nc.adaption != 3 && nc.adaption != 4)
    return -EPROTONOSUPPORT;
    pr_debug("create network interface\n");
    netname = "gsm%d";
    if (nc.if_name[0] != '\0')
    netname = nc.if_name;
    net = alloc_netdev(sizeof(struct gsm_mux_net), netname,
    NET_NAME_UNKNOWN, gsm_mux_net_init);
    if (!net) {
    pr_err("alloc_netdev failed\n");
    return -ENOMEM;
    }
    net.mtu = dlci.mtu;
    net.min_mtu = MIN_MTU;
    net.max_mtu = dlci.mtu;
    mux_net = netdev_priv(net);
    mux_net.dlci = dlci;
    kref_init(&mux_net.ref);
    strscpy(nc.if_name, net.name); /* return net name */
// reconfigure dlci for network
    dlci.prev_adaption = dlci.adaption;
    dlci.prev_data = dlci.data;
    dlci.adaption = nc.adaption;
    dlci.data = gsm_mux_rx_netchar;
    dlci.net = net;
    pr_debug("register netdev\n");
    retval = register_netdev(net);
    if (retval) {
    pr_err("network register fail %d\n", retval);
    dlci_net_free(dlci);
    return retval;
    }
    return net.ifindex;	/* return network index */
    }
// Line discipline for real tty
    static struct tty_ldisc_ops tty_ldisc_packet = {
    .owner		 = THIS_MODULE,
    .num		 = N_GSM0710,
    .name            = "n_gsm",
    .open            = gsmld_open,
    .close           = gsmld_close,
    .flush_buffer    = gsmld_flush_buffer,
    .read            = gsmld_read,
    .write           = gsmld_write,
    .ioctl           = gsmld_ioctl,
    .poll            = gsmld_poll,
    .receive_buf     = gsmld_receive_buf,
    .write_wakeup    = gsmld_write_wakeup
    };
//
// Virtual tty side
//
// gsm_modem_upd_via_data	-	send modem bits via convergence layer
// @dlci: channel
// @brk: break signal
//
// Send an empty frame to signal mobile state changes and to transmit the
// break signal for adaption 2.
//
#[no_mangle]
unsafe extern "C" fn gsm_modem_upd_via_data(dlci: *mut gsm_dlci, brk: u8) {
    static void gsm_modem_upd_via_data(struct gsm_dlci *dlci, u8 brk)
    {
    struct gsm_mux *gsm = dlci.gsm;
    unsigned long flags;
    if (dlci.state != DLCI_OPEN || dlci.adaption != 2)
    return;
    spin_lock_irqsave(&gsm.tx_lock, flags);
    gsm_dlci_modem_output(gsm, dlci, brk);
    spin_unlock_irqrestore(&gsm.tx_lock, flags);
    }
//
// gsm_modem_upd_via_msc	-	send modem bits via control frame
// @dlci: channel
// @brk: break signal
//
#[no_mangle]
unsafe extern "C" fn gsm_modem_upd_via_msc(dlci: *mut gsm_dlci, brk: u8) -> c_int {
    static int gsm_modem_upd_via_msc(struct gsm_dlci *dlci, u8 brk)
    {
    u8 modembits[3];
    struct gsm_control *ctrl;
    let mut len: c_int = 2;
    if (dlci.gsm.encoding != GSM_BASIC_OPT)
    return 0;
    modembits[0] = (dlci.addr << 2) | 2 | EA;  /* DLCI, Valid, EA */
    if (!brk) {
    modembits[1] = (gsm_encode_modem(dlci) << 1) | EA;
    } else {
    modembits[1] = gsm_encode_modem(dlci) << 1;
    modembits[2] = (brk << 4) | 2 | EA; /* Length, Break, EA */
    len++;
    }
    ctrl = gsm_control_send(dlci.gsm, CMD_MSC, modembits, len);
    if (ctrl == core::ptr::null_mut())
    return -ENOMEM;
    return gsm_control_wait(dlci.gsm, ctrl);
    }
//
// gsm_modem_send_initial_msc - Send initial modem status message
//
// @dlci: channel
//
// Send an initial MSC message after DLCI open to set the initial
// modem status lines. This is only done for basic mode.
// Does not wait for a response as we cannot block the input queue
// processing.
//
#[no_mangle]
unsafe extern "C" fn gsm_modem_send_initial_msc(dlci: *mut gsm_dlci) -> c_int {
    static int gsm_modem_send_initial_msc(struct gsm_dlci *dlci)
    {
    u8 modembits[2];
    if (dlci.adaption != 1 || dlci.gsm.encoding != GSM_BASIC_OPT)
    return 0;
    modembits[0] = (dlci.addr << 2) | 2 | EA; /* DLCI, Valid, EA */
    modembits[1] = (gsm_encode_modem(dlci) << 1) | EA;
    return gsm_control_command(dlci.gsm, CMD_MSC, (const u8 *)&modembits, 2);
    }
//
// gsm_modem_update	-	send modem status line state
// @dlci: channel
// @brk: break signal
//
#[no_mangle]
unsafe extern "C" fn gsm_modem_update(dlci: *mut gsm_dlci, brk: u8) -> c_int {
    static int gsm_modem_update(struct gsm_dlci *dlci, u8 brk)
    {
    if (dlci.gsm.dead)
    return -EL2HLT;
    if (dlci.adaption == 2) {
// Send convergence layer type 2 empty data frame.
    gsm_modem_upd_via_data(dlci, brk);
    return 0;
    } else if (dlci.gsm.encoding == GSM_BASIC_OPT) {
// Send as MSC control message.
    return gsm_modem_upd_via_msc(dlci, brk);
    }
// Modem status lines are not supported.
    return -EPROTONOSUPPORT;
    }
//
// gsm_wait_modem_change - wait for modem status line change
// @dlci: channel
// @mask: modem status line bits
//
// The function returns if:
// - any given modem status line bit changed
// - the wait event function got interrupted (e.g. by a signal)
// - the underlying DLCI was closed
// - the underlying ldisc device was removed
//
#[no_mangle]
unsafe extern "C" fn gsm_wait_modem_change(dlci: *mut gsm_dlci, mask: u32) -> c_int {
    static int gsm_wait_modem_change(struct gsm_dlci *dlci, u32 mask)
    {
    struct gsm_mux *gsm = dlci.gsm;
    let mut old: u32 = dlci.modem_rx;
    int ret;
    ret = wait_event_interruptible(gsm.event, gsm.dead ||
    dlci.state != DLCI_OPEN ||
    (old ^ dlci.modem_rx) & mask);
    if (gsm.dead)
    return -ENODEV;
    if (dlci.state != DLCI_OPEN)
    return -EL2NSYNC;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gsm_carrier_raised(port: *mut tty_port) -> bool {
    static bool gsm_carrier_raised(struct tty_port *port)
    {
    struct gsm_dlci *dlci = container_of(port, struct gsm_dlci, port);
    struct gsm_mux *gsm = dlci.gsm;
// Not yet open so no carrier info
    if (dlci.state != DLCI_OPEN)
    return false;
    if (debug & DBG_CD_ON)
    return true;
//
// Basic mode with control channel in ADM mode may not respond
// to CMD_MSC at all and modem_rx is empty.
//
    if (gsm.encoding == GSM_BASIC_OPT &&
    gsm.dlci[0].mode == DLCI_MODE_ADM && !dlci.modem_rx)
    return true;
    return dlci.modem_rx & TIOCM_CD;
    }
#[no_mangle]
unsafe extern "C" fn gsm_dtr_rts(port: *mut tty_port, active: bool) {
    static void gsm_dtr_rts(struct tty_port *port, bool active)
    {
    struct gsm_dlci *dlci = container_of(port, struct gsm_dlci, port);
    let mut modem_tx: c_uint = dlci.modem_tx;
    if (active)
    modem_tx |= TIOCM_DTR | TIOCM_RTS;
    else
    modem_tx &= ~(TIOCM_DTR | TIOCM_RTS);
    if (modem_tx != dlci.modem_tx) {
    dlci.modem_tx = modem_tx;
    gsm_modem_update(dlci, 0);
    }
    }
    static const struct tty_port_operations gsm_port_ops = {
    .carrier_raised = gsm_carrier_raised,
    .dtr_rts = gsm_dtr_rts,
    .destruct = gsm_dlci_free,
    };
#[no_mangle]
unsafe extern "C" fn gsmtty_install(driver: *mut tty_driver, tty: *mut tty_struct) -> c_int {
    static int gsmtty_install(struct tty_driver *driver, struct tty_struct *tty)
    {
    struct gsm_mux *gsm;
    struct gsm_dlci *dlci, *dlci0;
    let mut line: c_uint = tty.index;
    let mut mux: c_uint = mux_line_to_num(line);
    let mut alloc: bool = false;
    int ret;
    line = line & 0x3F;
    if (mux >= MAX_MUX)
    return -ENXIO;
// FIXME: we need to lock gsm_mux for lifetimes of ttys eventually
    if (gsm_mux[mux] == core::ptr::null_mut())
    return -EUNATCH;
    if (line == 0 || line > 61)	/* 62/63 reserved */
    return -ECHRNG;
    gsm = gsm_mux[mux];
    if (gsm.dead)
    return -EL2HLT;
// If DLCI 0 is not yet fully open return an error.
    This is ok from a locking
    perspective as we don't have to worry about this
    if DLCI0 is lost */
    mutex_lock(&gsm.mutex);
    dlci0 = gsm.dlci[0];
    if (dlci0 && dlci0.state != DLCI_OPEN) {
    mutex_unlock(&gsm.mutex);
    if (dlci0.state == DLCI_OPENING)
    wait_event(gsm.event, dlci0.state != DLCI_OPENING);
    if (dlci0.state != DLCI_OPEN)
    return -EL2NSYNC;
    mutex_lock(&gsm.mutex);
    }
    dlci = gsm.dlci[line];
    if (dlci == core::ptr::null_mut()) {
    alloc = true;
    dlci = gsm_dlci_alloc(gsm, line);
    }
    if (dlci == core::ptr::null_mut()) {
    mutex_unlock(&gsm.mutex);
    return -ENOMEM;
    }
    ret = tty_port_install(&dlci.port, driver, tty);
    if (ret) {
    if (alloc)
    dlci_put(dlci);
    mutex_unlock(&gsm.mutex);
    return ret;
    }
    dlci_get(dlci);
    dlci_get(gsm.dlci[0]);
    mux_get(gsm);
    tty.driver_data = dlci;
    mutex_unlock(&gsm.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_open(tty: *mut tty_struct, filp: *mut file) -> c_int {
    static int gsmtty_open(struct tty_struct *tty, struct file *filp)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    struct tty_port *port = &dlci.port;
    port.count++;
    tty_port_tty_set(port, tty);
    dlci.modem_rx = 0;
// We could in theory open and close before we wait - eg if we get
    a DM straight back. This is ok as that will have caused a hangup */
    tty_port_set_initialized(port, true);
// Start sending off SABM messages
    if (!dlci.gsm.wait_config) {
// Start sending off SABM messages
    if (dlci.gsm.initiator)
    gsm_dlci_begin_open(dlci);
    else
    gsm_dlci_set_opening(dlci);
    } else {
    gsm_dlci_set_wait_config(dlci);
    }
// And wait for virtual carrier
    return tty_port_block_til_ready(port, tty, filp);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_close(tty: *mut tty_struct, filp: *mut file) {
    static void gsmtty_close(struct tty_struct *tty, struct file *filp)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci == core::ptr::null_mut())
    return;
    if (dlci.state == DLCI_CLOSED)
    return;
    mutex_lock(&dlci.mutex);
    gsm_destroy_network(dlci);
    mutex_unlock(&dlci.mutex);
    if (tty_port_close_start(&dlci.port, tty, filp) == 0)
    return;
    gsm_dlci_begin_close(dlci);
    if (tty_port_initialized(&dlci.port) && C_HUPCL(tty))
    tty_port_lower_dtr_rts(&dlci.port);
    tty_port_close_end(&dlci.port, tty);
    tty_port_tty_set(&dlci.port, core::ptr::null_mut());
    return;
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_hangup(tty: *mut tty_struct) {
    static void gsmtty_hangup(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return;
    tty_port_hangup(&dlci.port);
    gsm_dlci_begin_close(dlci);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_write(tty: *mut tty_struct, buf: *const u8, len: usize) -> isize {
    static ssize_t gsmtty_write(struct tty_struct *tty, const u8 *buf, size_t len)
    {
    int sent;
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return -EINVAL;
// Stuff the bytes into the fifo queue
    sent = kfifo_in_locked(&dlci.fifo, buf, len, &dlci.lock);
// Need to kick the channel
    gsm_dlci_data_kick(dlci);
    return sent;
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int gsmtty_write_room(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return 0;
    return kfifo_avail(&dlci.fifo);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_chars_in_buffer(tty: *mut tty_struct) -> c_uint {
    static unsigned int gsmtty_chars_in_buffer(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return 0;
    return kfifo_len(&dlci.fifo);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_flush_buffer(tty: *mut tty_struct) {
    static void gsmtty_flush_buffer(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    unsigned long flags;
    if (dlci.state == DLCI_CLOSED)
    return;
// Caution needed: If we implement reliable transport classes
    then the data being transmitted can't simply be junked once
    it has first hit the stack. Until then we can just blow it
    away */
    spin_lock_irqsave(&dlci.lock, flags);
    kfifo_reset(&dlci.fifo);
    spin_unlock_irqrestore(&dlci.lock, flags);
// Need to unhook this DLCI from the transmit queue logic
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_wait_until_sent(tty: *mut tty_struct, timeout: c_int) {
    static void gsmtty_wait_until_sent(struct tty_struct *tty, int timeout)
    {
// The FIFO handles the queue so the kernel will do the right
    thing waiting on chars_in_buffer before calling us. No work
    to do here */
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_tiocmget(tty: *mut tty_struct) -> c_int {
    static int gsmtty_tiocmget(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return -EINVAL;
    return dlci.modem_rx;
    }
    static int gsmtty_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    let mut modem_tx: c_uint = dlci.modem_tx;
    if (dlci.state == DLCI_CLOSED)
    return -EINVAL;
    modem_tx &= ~clear;
    modem_tx |= set;
    if (modem_tx != dlci.modem_tx) {
    dlci.modem_tx = modem_tx;
    return gsm_modem_update(dlci, 0);
    }
    return 0;
    }
    static int gsmtty_ioctl(struct tty_struct *tty,
    unsigned int cmd, unsigned long arg)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    struct gsm_netconfig nc;
    struct gsm_dlci_config dc;
    int index;
    if (dlci.state == DLCI_CLOSED)
    return -EINVAL;
    switch (cmd) {
    case GSMIOC_ENABLE_NET:
    if (copy_from_user(&nc, (void __user *)arg, sizeof(nc)))
    return -EFAULT;
    nc.if_name[IFNAMSIZ-1] = '\0';
// return net interface index or error code
    mutex_lock(&dlci.mutex);
    index = gsm_create_network(dlci, &nc);
    mutex_unlock(&dlci.mutex);
    if (copy_to_user((void __user *)arg, &nc, sizeof(nc)))
    return -EFAULT;
    return index;
    case GSMIOC_DISABLE_NET:
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    mutex_lock(&dlci.mutex);
    gsm_destroy_network(dlci);
    mutex_unlock(&dlci.mutex);
    return 0;
    case GSMIOC_GETCONF_DLCI:
    if (copy_from_user(&dc, (void __user *)arg, sizeof(dc)))
    return -EFAULT;
    if (dc.channel != dlci.addr)
    return -EPERM;
    gsm_dlci_copy_config_values(dlci, &dc);
    if (copy_to_user((void __user *)arg, &dc, sizeof(dc)))
    return -EFAULT;
    return 0;
    case GSMIOC_SETCONF_DLCI:
    if (copy_from_user(&dc, (void __user *)arg, sizeof(dc)))
    return -EFAULT;
    if (dc.channel >= NUM_DLCI)
    return -EINVAL;
    if (dc.channel != 0 && dc.channel != dlci.addr)
    return -EPERM;
    return gsm_dlci_config(dlci, &dc, 1);
    case TIOCMIWAIT:
    return gsm_wait_modem_change(dlci, (u32)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static void gsmtty_set_termios(struct tty_struct *tty,
    const struct ktermios *old)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return;
// For the moment its fixed. In actual fact the speed information
    for the virtual channel can be propogated in both directions by
    the RPN control message. This however rapidly gets nasty as we
    then have to remap modem signals each way according to whether
    our virtual cable is null modem etc .. */
    tty_termios_copy_hw(&tty.termios, old);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_throttle(tty: *mut tty_struct) {
    static void gsmtty_throttle(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return;
    if (C_CRTSCTS(tty))
    dlci.modem_tx &= ~TIOCM_RTS;
    dlci.throttled = true;
// Send an MSC with RTS cleared
    gsm_modem_update(dlci, 0);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_unthrottle(tty: *mut tty_struct) {
    static void gsmtty_unthrottle(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    if (dlci.state == DLCI_CLOSED)
    return;
    if (C_CRTSCTS(tty))
    dlci.modem_tx |= TIOCM_RTS;
    dlci.throttled = false;
// Send an MSC with RTS set
    gsm_modem_update(dlci, 0);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_break_ctl(tty: *mut tty_struct, state: c_int) -> c_int {
    static int gsmtty_break_ctl(struct tty_struct *tty, int state)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    int encode = 0;	/* Off */
    if (dlci.state == DLCI_CLOSED)
    return -EINVAL;
    if (state == -1)	/* "On indefinitely" - we can't encode this
    properly */
    encode = 0x0F;
#[no_mangle]
pub unsafe extern "C" fn if(0: state >) -> else {
    encode = state / 200;	/* mS to encoding */
    if (encode > 0x0F)
    encode = 0x0F;	/* Best effort */
    }
    return gsm_modem_update(dlci, encode);
    }
#[no_mangle]
unsafe extern "C" fn gsmtty_cleanup(tty: *mut tty_struct) {
    static void gsmtty_cleanup(struct tty_struct *tty)
    {
    struct gsm_dlci *dlci = tty.driver_data;
    struct gsm_mux *gsm = dlci.gsm;
    dlci_put(dlci);
    dlci_put(gsm.dlci[0]);
    mux_put(gsm);
    }
// Virtual ttys for the demux
    static const struct tty_operations gsmtty_ops = {
    .install		= gsmtty_install,
    .open			= gsmtty_open,
    .close			= gsmtty_close,
    .write			= gsmtty_write,
    .write_room		= gsmtty_write_room,
    .chars_in_buffer	= gsmtty_chars_in_buffer,
    .flush_buffer		= gsmtty_flush_buffer,
    .ioctl			= gsmtty_ioctl,
    .throttle		= gsmtty_throttle,
    .unthrottle		= gsmtty_unthrottle,
    .set_termios		= gsmtty_set_termios,
    .hangup			= gsmtty_hangup,
    .wait_until_sent	= gsmtty_wait_until_sent,
    .tiocmget		= gsmtty_tiocmget,
    .tiocmset		= gsmtty_tiocmset,
    .break_ctl		= gsmtty_break_ctl,
    .cleanup		= gsmtty_cleanup,
    };
#[no_mangle]
unsafe extern "C" fn gsm_init() -> int __init {
    static int __init gsm_init(void)
    {
// Fill in our line protocol discipline, and register it
    let mut status: c_int = tty_register_ldisc(&tty_ldisc_packet);
    if (status != 0) {
    pr_err("n_gsm: can't register line discipline (err = %d)\n",
    status);
    return status;
    }
    gsm_tty_driver = tty_alloc_driver(GSM_TTY_MINORS, TTY_DRIVER_REAL_RAW |
    TTY_DRIVER_DYNAMIC_DEV | TTY_DRIVER_HARDWARE_BREAK);
    if (IS_ERR(gsm_tty_driver)) {
    pr_err("gsm_init: tty allocation failed.\n");
    status = PTR_ERR(gsm_tty_driver);
    goto err_unreg_ldisc;
    }
    gsm_tty_driver.driver_name	= "gsmtty";
    gsm_tty_driver.name		= "gsmtty";
    gsm_tty_driver.major		= 0;	/* Dynamic */
    gsm_tty_driver.minor_start	= 0;
    gsm_tty_driver.type		= TTY_DRIVER_TYPE_SERIAL;
    gsm_tty_driver.subtype	= SERIAL_TYPE_NORMAL;
    gsm_tty_driver.init_termios	= tty_std_termios;
// Fixme
    gsm_tty_driver.init_termios.c_lflag &= ~ECHO;
    tty_set_operations(gsm_tty_driver, &gsmtty_ops);
    if (tty_register_driver(gsm_tty_driver)) {
    pr_err("gsm_init: tty registration failed.\n");
    status = -EBUSY;
    goto err_put_driver;
    }
    pr_debug("gsm_init: loaded as %d,%d.\n",
    gsm_tty_driver.major, gsm_tty_driver.minor_start);
    return 0;
    err_put_driver:
    tty_driver_kref_put(gsm_tty_driver);
    err_unreg_ldisc:
    tty_unregister_ldisc(&tty_ldisc_packet);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn gsm_exit() -> void __exit {
    static void __exit gsm_exit(void)
    {
    tty_unregister_ldisc(&tty_ldisc_packet);
    tty_unregister_driver(gsm_tty_driver);
    tty_driver_kref_put(gsm_tty_driver);
    }
    module_init(gsm_init);
    module_exit(gsm_exit);
    MODULE_DESCRIPTION("GSM 0710 tty multiplexor");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_LDISC(N_GSM0710);
