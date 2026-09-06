//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_intf.c
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
// ipmi_si.c
//
// The interface to the IPMI driver for the system interfaces (KCS, SMIC,
// BT).
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
// Copyright 2006 IBM Corp., Christian Krafft <krafft@de.ibm.com>
//
// This file holds the "policy" for the interface to the SMI state
// machine.  It does the configuration, handles timers and interrupts,
// and drives the real SMI state machine.
//

// Measure times between events in the driver.

// Call every 10 ms.
pub const SI_TIMEOUT_TIME_USEC: c_int = 10000;

    short timeout */

    enum si_intf_state {
    SI_NORMAL,
    SI_GETTING_FLAGS,
    SI_GETTING_EVENTS,
    SI_CLEARING_FLAGS,
    SI_GETTING_MESSAGES,
    SI_CHECKING_ENABLES,
    SI_SETTING_ENABLES,
    SI_HOSED
// FIXME - add watchdog stuff.
    };
// Some BT-specific defines we need here.
pub const IPMI_BT_INTMASK_REG: c_int = 2;
pub const IPMI_BT_INTMASK_CLEAR_IRQ_BIT: c_int = 2;
pub const IPMI_BT_INTMASK_ENABLE_IRQ_BIT: c_int = 1;
// 'invalid' to allow a firmware-specified interface to be disabled
    const char *const si_to_str[] = { "invalid", "kcs", "smic", "bt", core::ptr::null_mut() };
    let mut ipmi_kcs_si_info: ipmi_match_info = { .type = SI_KCS };
    let mut ipmi_smic_si_info: ipmi_match_info = { .type = SI_SMIC };
    let mut ipmi_bt_si_info: ipmi_match_info = { .type = SI_BT };
    static bool initialized;
//
// Indexes into stats[] in smi_info below.
//
    enum si_stat_indexes {
//
// Number of times the driver requested a timer while an operation
// was in progress.
//
    SI_STAT_short_timeouts = 0,
//
// Number of times the driver requested a timer while nothing was in
// progress.
//
    SI_STAT_long_timeouts,
// Number of times the interface was idle while being polled.
    SI_STAT_idles,
// Number of interrupts the driver handled.
    SI_STAT_interrupts,
// Number of time the driver got an ATTN from the hardware.
    SI_STAT_attentions,
// Number of times the driver requested flags from the hardware.
    SI_STAT_flag_fetches,
// Number of times the hardware didn't follow the state machine.
    SI_STAT_hosed_count,
// Number of completed messages.
    SI_STAT_complete_transactions,
// Number of IPMI events received from the hardware.
    SI_STAT_events,
// Number of watchdog pretimeouts.
    SI_STAT_watchdog_pretimeouts,
// Number of asynchronous messages received.
    SI_STAT_incoming_messages,
// This *must* remain last, add new values above this.
    SI_NUM_STATS
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_info {
    pub si_num: c_int,
    pub intf: *mut ipmi_smi,
    pub si_sm: *mut si_sm_data,
    pub handlers: *const si_sm_handlers,
    pub si_lock: spinlock_t,
    pub waiting_msg: *mut ipmi_smi_msg,
    pub curr_msg: *mut ipmi_smi_msg,
    pub si_state: enum si_intf_state,
//
// Used to handle the various types of I/O that can occur with
// IPMI
//
    pub io: si_sm_io,
//
// Per-OEM handler, called from handle_flags().  Returns 1
// when handle_flags() needs to be re-run or 0 indicating it
// set si_state itself.
//
    pub smi_info): *mut *mut int (oem_data_avail_handler)(struct smi_info,
//
// Flags from the last GET_MSG_FLAGS command, used when an ATTN
// is set to hold the flags until we are done handling everything
// from the flags.
//
pub const RECEIVE_MSG_AVAIL: c_uint = 0x01;
pub const EVENT_MSG_BUFFER_FULL: c_uint = 0x02;
pub const WDT_PRE_TIMEOUT_INT: c_uint = 0x08;
pub const OEM0_DATA_AVAIL: c_uint = 0x20;
pub const OEM1_DATA_AVAIL: c_uint = 0x40;
pub const OEM2_DATA_AVAIL: c_uint = 0x80;

    OEM1_DATA_AVAIL | \
    OEM2_DATA_AVAIL)
    pub msg_flags: c_uchar,
// When requesting events and messages, don't do it forever.
    pub num_requests_in_a_row: c_uint,
    pub last_was_flag_fetch: bool,
// Does the BMC have an event buffer?
    pub has_event_buffer: bool,
//
// If set to true, this will request events the next time the
// state machine is idle.
//
    pub req_events: core::sync::atomic::AtomicI32,
//
// If true, run the state machine to completion on every send
// call.  Generally used after a panic to make sure stuff goes
// out.
//
    pub run_to_completion: bool,
// The timer for this si.
    pub si_timer: timer_list,
// This flag is set, if the timer can be set
    pub timer_can_start: bool,
// This flag is set, if the timer is running (timer_pending() isn't enough)
    pub timer_running: bool,
// The time (in jiffies) the last timeout occurred at.
    pub last_timeout_jiffies: c_ulong,
// Are we waiting for the events, pretimeouts, received msgs?
    pub need_watch: core::sync::atomic::AtomicI32,
//
// The driver will disable interrupts when it gets into a
// situation where it cannot handle messages due to lack of
// memory.  Once that situation clears up, it will re-enable
// interrupts.
//
    pub interrupt_disabled: bool,
//
// Does the BMC support events?
//
    pub supports_event_msg_buff: bool,
//
// Can we disable interrupts the global enables receive irq
// bit?  There are currently two forms of brokenness, some
// systems cannot disable the bit (which is technically within
// the spec but a bad idea) and some systems have the bit
// forced to zero even though interrupts work (which is
// clearly outside the spec).  The next bool tells which form
// of brokenness is present.
//
    pub cannot_disable_irq: bool,
//
// Some systems are broken and cannot set the irq enable
// bit, even if they support interrupts.
//
    pub irq_enable_broken: bool,
// Is the driver in maintenance mode?
    pub in_maintenance_mode: bool,
//
// Did we get an attention that we did not handle?
//
    pub got_attn: bool,
// From the get device id response...
    pub device_id: ipmi_device_id,
// Have we added the device group to the device?
    pub dev_group_added: bool,
// Counters and things for the proc filesystem.
    pub stats: [core::sync::atomic::AtomicI32; SI_NUM_STATS],
    pub thread: *mut task_struct,
    pub init_work: work_struct,
    pub link: list_head,
}

    atomic_inc(&(smi).stats[SI_STAT_ ## stat])

    ((unsigned int) atomic_read(&(smi).stats[SI_STAT_ ## stat]))
pub const IPMI_MAX_INTFS: c_int = 4;
    static int force_kipmid[IPMI_MAX_INTFS];
    static int num_force_kipmid;
    static unsigned int kipmid_max_busy_us[IPMI_MAX_INTFS];
    static int num_max_busy_us;
    let mut unload_when_empty: static bool = true;
    static int try_smi_init(struct smi_info *smi);
    static void cleanup_one_si(struct smi_info *smi_info);
    static void cleanup_ipmi_si(void);
    static void smi_init_work_fn(struct work_struct *work);

#[no_mangle]
pub unsafe extern "C" fn debug_timestamp(smi_info: *mut smi_info, msg: *mut c_char) {
    void debug_timestamp(struct smi_info *smi_info, char *msg)
    {
    struct timespec64 t;
    ktime_get_ts64(&t);
    dev_dbg(smi_info.io.dev, "**%s: %ptSp\n", msg, &t);
    }

    static ATOMIC_NOTIFIER_HEAD(xaction_notifier_list);
#[no_mangle]
unsafe extern "C" fn register_xaction_notifier(nb: *mut notifier_block) -> c_int {
    static int register_xaction_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_register(&xaction_notifier_list, nb);
    }
    static void deliver_recv_msg(struct smi_info *smi_info,
    struct ipmi_smi_msg *msg)
    {
// Deliver the message to the upper layer.
    ipmi_smi_msg_received(smi_info.intf, msg);
    }
#[no_mangle]
unsafe extern "C" fn return_hosed_msg(smi_info: *mut smi_info, cCode: c_int) {
    static void return_hosed_msg(struct smi_info *smi_info, int cCode)
    {
    struct ipmi_smi_msg *msg = smi_info.curr_msg;
    if (cCode < 0 || cCode > IPMI_ERR_UNSPECIFIED)
    cCode = IPMI_ERR_UNSPECIFIED;
// else use it as is
// Make it a response
    msg.rsp[0] = msg.data[0] | 4;
    msg.rsp[1] = msg.data[1];
    msg.rsp[2] = cCode;
    msg.rsp_size = 3;
    smi_info.curr_msg = core::ptr::null_mut();
    deliver_recv_msg(smi_info, msg);
    }
#[no_mangle]
unsafe extern "C" fn start_next_msg(smi_info: *mut smi_info) -> enum si_sm_result {
    static enum si_sm_result start_next_msg(struct smi_info *smi_info)
    {
    int rv;
    if (!smi_info.waiting_msg) {
    smi_info.curr_msg = core::ptr::null_mut();
    rv = SI_SM_IDLE;
    } else {
    int err;
    smi_info.curr_msg = smi_info.waiting_msg;
    smi_info.waiting_msg = core::ptr::null_mut();
    debug_timestamp(smi_info, "Start2");
    err = atomic_notifier_call_chain(&xaction_notifier_list,
    0, smi_info);
    if (err & NOTIFY_STOP_MASK) {
    rv = SI_SM_CALL_WITHOUT_DELAY;
    goto out;
    }
    err = smi_info.handlers.start_transaction(
    smi_info.si_sm,
    smi_info.curr_msg.data,
    smi_info.curr_msg.data_size);
    if (err)
    return_hosed_msg(smi_info, err);
    rv = SI_SM_CALL_WITHOUT_DELAY;
    }
    out:
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn smi_mod_timer(smi_info: *mut smi_info, new_val: c_ulong) {
    static void smi_mod_timer(struct smi_info *smi_info, unsigned long new_val)
    {
    if (!smi_info.timer_can_start)
    return;
    smi_info.last_timeout_jiffies = jiffies;
    mod_timer(&smi_info.si_timer, new_val);
    smi_info.timer_running = true;
    }
//
// Start a new message and (re)start the timer and thread.
//
    static void start_new_msg(struct smi_info *smi_info, unsigned char *msg,
    unsigned int size)
    {
    smi_mod_timer(smi_info, jiffies + SI_TIMEOUT_JIFFIES);
    if (smi_info.thread)
    wake_up_process(smi_info.thread);
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, size);
    }
#[no_mangle]
unsafe extern "C" fn start_check_enables(smi_info: *mut smi_info) {
    static void start_check_enables(struct smi_info *smi_info)
    {
    unsigned char msg[2];
    msg[0] = (IPMI_NETFN_APP_REQUEST << 2);
    msg[1] = IPMI_GET_BMC_GLOBAL_ENABLES_CMD;
    start_new_msg(smi_info, msg, 2);
    smi_info.si_state = SI_CHECKING_ENABLES;
    }
#[no_mangle]
unsafe extern "C" fn start_clear_flags(smi_info: *mut smi_info) {
    static void start_clear_flags(struct smi_info *smi_info)
    {
    unsigned char msg[3];
// Make sure the watchdog pre-timeout flag is not set at startup.
    msg[0] = (IPMI_NETFN_APP_REQUEST << 2);
    msg[1] = IPMI_CLEAR_MSG_FLAGS_CMD;
    msg[2] = WDT_PRE_TIMEOUT_INT;
    start_new_msg(smi_info, msg, 3);
    smi_info.si_state = SI_CLEARING_FLAGS;
    }
#[no_mangle]
unsafe extern "C" fn start_get_flags(smi_info: *mut smi_info) {
    static void start_get_flags(struct smi_info *smi_info)
    {
    unsigned char msg[2];
    msg[0] = (IPMI_NETFN_APP_REQUEST << 2);
    msg[1] = IPMI_GET_MSG_FLAGS_CMD;
    start_new_msg(smi_info, msg, 2);
    smi_info.si_state = SI_GETTING_FLAGS;
    }
#[no_mangle]
unsafe extern "C" fn start_getting_msg_queue(smi_info: *mut smi_info) {
    static void start_getting_msg_queue(struct smi_info *smi_info)
    {
    smi_info.curr_msg.data[0] = (IPMI_NETFN_APP_REQUEST << 2);
    smi_info.curr_msg.data[1] = IPMI_GET_MSG_CMD;
    smi_info.curr_msg.data_size = 2;
    start_new_msg(smi_info, smi_info.curr_msg.data,
    smi_info.curr_msg.data_size);
    if (smi_info.si_state != SI_GETTING_MESSAGES) {
    smi_info.num_requests_in_a_row = 0;
    smi_info.si_state = SI_GETTING_MESSAGES;
    }
    }
#[no_mangle]
unsafe extern "C" fn start_getting_events(smi_info: *mut smi_info) {
    static void start_getting_events(struct smi_info *smi_info)
    {
    smi_info.curr_msg.data[0] = (IPMI_NETFN_APP_REQUEST << 2);
    smi_info.curr_msg.data[1] = IPMI_READ_EVENT_MSG_BUFFER_CMD;
    smi_info.curr_msg.data_size = 2;
    start_new_msg(smi_info, smi_info.curr_msg.data,
    smi_info.curr_msg.data_size);
    if (smi_info.si_state != SI_GETTING_EVENTS) {
    smi_info.num_requests_in_a_row = 0;
    smi_info.si_state = SI_GETTING_EVENTS;
    }
    }
//
// When we have a situtaion where we run out of memory and cannot
// allocate messages, we just leave them in the BMC and run the system
// polled until we can allocate some memory.  Once we have some
// memory, we will re-enable the interrupt.
//
// Note that we cannot just use disable_irq(), since the interrupt may
// be shared.
//
#[no_mangle]
pub unsafe extern "C" fn disable_si_irq(smi_info: *mut smi_info) -> bool {
    static inline bool disable_si_irq(struct smi_info *smi_info)
    {
    if ((smi_info.io.irq) && (!smi_info.interrupt_disabled)) {
    smi_info.interrupt_disabled = true;
    start_check_enables(smi_info);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn enable_si_irq(smi_info: *mut smi_info) -> bool {
    static inline bool enable_si_irq(struct smi_info *smi_info)
    {
    if ((smi_info.io.irq) && (smi_info.interrupt_disabled)) {
    smi_info.interrupt_disabled = false;
    start_check_enables(smi_info);
    return true;
    }
    return false;
    }
//
// Allocate a message.  If unable to allocate, start the interrupt
// disable process and return NULL.  If able to allocate but
// interrupts are disabled, free the message and return NULL after
// starting the interrupt enable process.
//
    static struct ipmi_smi_msg *alloc_msg_handle_irq(struct smi_info *smi_info)
    {
    struct ipmi_smi_msg *msg;
    msg = ipmi_alloc_smi_msg();
    if (!msg) {
    if (!disable_si_irq(smi_info))
    smi_info.si_state = SI_NORMAL;
    } else if (enable_si_irq(smi_info)) {
    ipmi_free_smi_msg(msg);
    msg = core::ptr::null_mut();
    }
    return msg;
    }
#[no_mangle]
unsafe extern "C" fn handle_flags(smi_info: *mut smi_info) {
    static void handle_flags(struct smi_info *smi_info)
    {
    retry:
    if (smi_info.msg_flags & WDT_PRE_TIMEOUT_INT) {
// Watchdog pre-timeout
    smi_inc_stat(smi_info, watchdog_pretimeouts);
    start_clear_flags(smi_info);
    smi_info.msg_flags &= ~WDT_PRE_TIMEOUT_INT;
    ipmi_smi_watchdog_pretimeout(smi_info.intf);
    } else if (smi_info.msg_flags & RECEIVE_MSG_AVAIL) {
// Messages available.
    smi_info.curr_msg = alloc_msg_handle_irq(smi_info);
    if (!smi_info.curr_msg) {
    smi_info.si_state = SI_NORMAL;
    return;
    }
    start_getting_msg_queue(smi_info);
    } else if (smi_info.msg_flags & EVENT_MSG_BUFFER_FULL) {
// Events available.
    smi_info.curr_msg = alloc_msg_handle_irq(smi_info);
    if (!smi_info.curr_msg) {
    smi_info.si_state = SI_NORMAL;
    return;
    }
    start_getting_events(smi_info);
    } else if (smi_info.msg_flags & OEM_DATA_AVAIL &&
    smi_info.oem_data_avail_handler) {
    if (smi_info.oem_data_avail_handler(smi_info))
    goto retry;
    } else
    smi_info.si_state = SI_NORMAL;
    }
//
// Global enables we care about.
//

    IPMI_BMC_EVT_MSG_INTR)
    static u8 current_global_enables(struct smi_info *smi_info, u8 base,
    bool *irq_on)
    {
    let mut enables: u8 = 0;
    if (smi_info.supports_event_msg_buff)
    enables |= IPMI_BMC_EVT_MSG_BUFF;
    if (((smi_info.io.irq && !smi_info.interrupt_disabled) ||
    smi_info.cannot_disable_irq) &&
    !smi_info.irq_enable_broken)
    enables |= IPMI_BMC_RCV_MSG_INTR;
    if (smi_info.supports_event_msg_buff &&
    smi_info.io.irq && !smi_info.interrupt_disabled &&
    !smi_info.irq_enable_broken)
    enables |= IPMI_BMC_EVT_MSG_INTR;
// irq_on = enables & (IPMI_BMC_EVT_MSG_INTR | IPMI_BMC_RCV_MSG_INTR);
    return enables;
    }
#[no_mangle]
unsafe extern "C" fn check_bt_irq(smi_info: *mut smi_info, irq_on: bool) {
    static void check_bt_irq(struct smi_info *smi_info, bool irq_on)
    {
    let mut irqstate: u8 = smi_info.io.inputb(&smi_info.io, IPMI_BT_INTMASK_REG);
    irqstate &= IPMI_BT_INTMASK_ENABLE_IRQ_BIT;
    if ((bool)irqstate == irq_on)
    return;
    if (irq_on)
    smi_info.io.outputb(&smi_info.io, IPMI_BT_INTMASK_REG,
    IPMI_BT_INTMASK_ENABLE_IRQ_BIT);
    else
    smi_info.io.outputb(&smi_info.io, IPMI_BT_INTMASK_REG, 0);
    }
#[no_mangle]
unsafe extern "C" fn handle_transaction_done(smi_info: *mut smi_info) {
    static void handle_transaction_done(struct smi_info *smi_info)
    {
    struct ipmi_smi_msg *msg;
    debug_timestamp(smi_info, "Done");
    switch (smi_info.si_state) {
    case SI_NORMAL:
    if (!smi_info.curr_msg)
    break;
    smi_info.curr_msg.rsp_size
    = smi_info.handlers.get_result(
    smi_info.si_sm,
    smi_info.curr_msg.rsp,
    IPMI_MAX_MSG_LENGTH);
//
// Do this here becase deliver_recv_msg() releases the
// lock, and a new message can be put in during the
// time the lock is released.
//
    msg = smi_info.curr_msg;
    smi_info.curr_msg = core::ptr::null_mut();
    deliver_recv_msg(smi_info, msg);
    break;
    case SI_GETTING_FLAGS:
    {
    unsigned char msg[4];
    unsigned int  len;
// We got the flags from the SMI, now handle them.
    len = smi_info.handlers.get_result(smi_info.si_sm, msg, 4);
    if (msg[2] != 0) {
// Error fetching flags, just give up for now.
    smi_info.si_state = SI_NORMAL;
    } else if (len < 4) {
//
// Hmm, no flags.  That's technically illegal, but
// don't use uninitialized data.
//
    smi_info.si_state = SI_NORMAL;
    } else {
    smi_info.msg_flags = msg[3];
    smi_info.last_was_flag_fetch = true;
    handle_flags(smi_info);
    }
    break;
    }
    case SI_CLEARING_FLAGS:
    {
    unsigned char msg[3];
// We cleared the flags.
    smi_info.handlers.get_result(smi_info.si_sm, msg, 3);
    if (msg[2] != 0) {
// Error clearing flags
    dev_warn_ratelimited(smi_info.io.dev,
    "Error clearing flags: %2.2x\n", msg[2]);
    }
    smi_info.si_state = SI_NORMAL;
    break;
    }
    case SI_GETTING_EVENTS:
    {
    smi_info.curr_msg.rsp_size
    = smi_info.handlers.get_result(
    smi_info.si_sm,
    smi_info.curr_msg.rsp,
    IPMI_MAX_MSG_LENGTH);
//
// Do this here becase deliver_recv_msg() releases the
// lock, and a new message can be put in during the
// time the lock is released.
//
    msg = smi_info.curr_msg;
    smi_info.curr_msg = core::ptr::null_mut();
//
// It appears some BMCs, with no event data, return no
// data in the message and not a 0x80 error as the
// spec says they should.  Shut down processing if
// the data is not the right length.
//
    if (msg.rsp[2] != 0 || msg.rsp_size != 19) {
// Error getting event, probably done.
    msg.done(msg);
// Take off the event flag.
    smi_info.msg_flags &= ~EVENT_MSG_BUFFER_FULL;
    handle_flags(smi_info);
    } else {
    smi_inc_stat(smi_info, events);
    smi_info.num_requests_in_a_row++;
    if (smi_info.num_requests_in_a_row > 10)
// Stop if we do this too many times.
    smi_info.msg_flags &= ~EVENT_MSG_BUFFER_FULL;
//
// Do this before we deliver the message
// because delivering the message releases the
// lock and something else can mess with the
// state.
//
    handle_flags(smi_info);
    deliver_recv_msg(smi_info, msg);
    }
    break;
    }
    case SI_GETTING_MESSAGES:
    {
    smi_info.curr_msg.rsp_size
    = smi_info.handlers.get_result(
    smi_info.si_sm,
    smi_info.curr_msg.rsp,
    IPMI_MAX_MSG_LENGTH);
//
// Do this here becase deliver_recv_msg() releases the
// lock, and a new message can be put in during the
// time the lock is released.
//
    msg = smi_info.curr_msg;
    smi_info.curr_msg = core::ptr::null_mut();
    if (msg.rsp[2] != 0) {
// Error getting event, probably done.
    msg.done(msg);
// Take off the msg flag.
    smi_info.msg_flags &= ~RECEIVE_MSG_AVAIL;
    handle_flags(smi_info);
    } else {
    smi_inc_stat(smi_info, incoming_messages);
    smi_info.num_requests_in_a_row++;
    if (smi_info.num_requests_in_a_row > 10)
// Stop if we do this too many times.
    smi_info.msg_flags &= ~RECEIVE_MSG_AVAIL;
//
// Do this before we deliver the message
// because delivering the message releases the
// lock and something else can mess with the
// state.
//
    handle_flags(smi_info);
    deliver_recv_msg(smi_info, msg);
    }
    break;
    }
    case SI_CHECKING_ENABLES:
    {
    unsigned char msg[4];
    u8 enables;
    bool irq_on;
// We got the flags from the SMI, now handle them.
    smi_info.handlers.get_result(smi_info.si_sm, msg, 4);
    if (msg[2] != 0) {
    dev_warn_ratelimited(smi_info.io.dev,
    "Couldn't get irq info: %x,\n"
    "Maybe ok, but ipmi might run very slowly.\n",
    msg[2]);
    smi_info.si_state = SI_NORMAL;
    break;
    }
    enables = current_global_enables(smi_info, 0, &irq_on);
    if (smi_info.io.si_info.type == SI_BT)
// BT has its own interrupt enable bit.
    check_bt_irq(smi_info, irq_on);
    if (enables != (msg[3] & GLOBAL_ENABLES_MASK)) {
// Enables are not correct, fix them.
    msg[0] = (IPMI_NETFN_APP_REQUEST << 2);
    msg[1] = IPMI_SET_BMC_GLOBAL_ENABLES_CMD;
    msg[2] = enables | (msg[3] & ~GLOBAL_ENABLES_MASK);
    smi_info.handlers.start_transaction(
    smi_info.si_sm, msg, 3);
    smi_info.si_state = SI_SETTING_ENABLES;
    } else if (smi_info.supports_event_msg_buff) {
    smi_info.curr_msg = ipmi_alloc_smi_msg();
    if (!smi_info.curr_msg) {
    smi_info.si_state = SI_NORMAL;
    break;
    }
    start_getting_events(smi_info);
    } else {
    smi_info.si_state = SI_NORMAL;
    }
    break;
    }
    case SI_SETTING_ENABLES:
    {
    unsigned char msg[4];
    smi_info.handlers.get_result(smi_info.si_sm, msg, 4);
    if (msg[2] != 0)
    dev_warn_ratelimited(smi_info.io.dev,
    "Could not set the global enables: 0x%x.\n",
    msg[2]);
    if (smi_info.supports_event_msg_buff) {
    smi_info.curr_msg = ipmi_alloc_smi_msg();
    if (!smi_info.curr_msg) {
    smi_info.si_state = SI_NORMAL;
    break;
    }
    start_getting_events(smi_info);
    } else {
    smi_info.si_state = SI_NORMAL;
    }
    break;
    }
    case SI_HOSED: /* Shouldn't happen. */
    break;
    }
    }
//
// Called on timeouts and events.  Timeouts should pass the elapsed
// time, interrupts should pass in zero.  Must be called with
// si_lock held and interrupts disabled.
//
    static enum si_sm_result smi_event_handler(struct smi_info *smi_info,
    int time)
    {
    enum si_sm_result si_sm_result;
    restart:
    if (smi_info.si_state == SI_HOSED)
// Just in case, hosed state is only left from the timeout.
    return SI_SM_HOSED;
//
// There used to be a loop here that waited a little while
// (around 25us) before giving up.  That turned out to be
// pointless, the minimum delays I was seeing were in the 300us
// range, which is far too long to wait in an interrupt.  So
// we just run until the state machine tells us something
// happened or it needs a delay.
//
    si_sm_result = smi_info.handlers.event(smi_info.si_sm, time);
    time = 0;
    while (si_sm_result == SI_SM_CALL_WITHOUT_DELAY)
    si_sm_result = smi_info.handlers.event(smi_info.si_sm, 0);
    if (si_sm_result == SI_SM_TRANSACTION_COMPLETE) {
    smi_inc_stat(smi_info, complete_transactions);
    handle_transaction_done(smi_info);
    goto restart;
    } else if (si_sm_result == SI_SM_HOSED) {
    smi_inc_stat(smi_info, hosed_count);
//
// Do the before return_hosed_msg, because that
// releases the lock.  We just disable operations for
// a while and retry in hosed state.
//
    smi_info.si_state = SI_HOSED;
    if (smi_info.curr_msg != core::ptr::null_mut()) {
//
// If we were handling a user message, format
// a response to send to the upper layer to
// tell it about the error.
//
    return_hosed_msg(smi_info, IPMI_BUS_ERR);
    }
    if (smi_info.waiting_msg != core::ptr::null_mut()) {
// Also handle if there was a message waiting.
    smi_info.curr_msg = smi_info.waiting_msg;
    smi_info.waiting_msg = core::ptr::null_mut();
    return_hosed_msg(smi_info, IPMI_BUS_ERR);
    }
    smi_mod_timer(smi_info, jiffies + SI_TIMEOUT_HOSED);
    goto out;
    }
//
// If we are currently idle, or if the last thing that was
// done was a flag fetch and there is a message pending, try
// to start the next message.
//
// We do the waiting message check to avoid a stuck flag
// completely wedging the driver.  Let a message through
// in between flag operations if that happens.
//
    if (si_sm_result == SI_SM_IDLE ||
    (si_sm_result == SI_SM_ATTN && smi_info.waiting_msg &&
    smi_info.last_was_flag_fetch)) {
    smi_info.last_was_flag_fetch = false;
    smi_inc_stat(smi_info, idles);
    si_sm_result = start_next_msg(smi_info);
    if (si_sm_result != SI_SM_IDLE)
    goto restart;
    }
//
// We prefer handling attn over new messages.  But don't do
// this if there is not yet an upper layer to handle anything.
//
    if (si_sm_result == SI_SM_ATTN || smi_info.got_attn) {
    if (smi_info.si_state != SI_NORMAL) {
//
// We got an ATTN, but we are doing something else.
// Handle the ATTN later.
//
    smi_info.got_attn = true;
    } else {
    smi_info.got_attn = false;
    smi_inc_stat(smi_info, attentions);
//
// Got a attn, send down a get message flags to see
// what's causing it.  It would be better to handle
// this in the upper layer, but due to the way
// interrupts work with the SMI, that's not really
// possible.
//
    start_get_flags(smi_info);
    goto restart;
    }
    }
    if ((si_sm_result == SI_SM_IDLE)
    && (atomic_read(&smi_info.req_events))) {
//
// We are idle and the upper layer requested that I fetch
// events, so do so.
//
    atomic_set(&smi_info.req_events, 0);
//
// Take this opportunity to check the interrupt and
// message enable state for the BMC.  The BMC can be
// asynchronously reset, and may thus get interrupts
// disable and messages disabled.
//
    if (smi_info.supports_event_msg_buff || smi_info.io.irq) {
    start_check_enables(smi_info);
    } else {
    smi_info.curr_msg = alloc_msg_handle_irq(smi_info);
    if (!smi_info.curr_msg)
    goto out;
    start_getting_events(smi_info);
    }
    goto restart;
    }
    if (si_sm_result == SI_SM_IDLE && smi_info.timer_running) {
// Ok it if fails, the timer will just go off.
    if (timer_delete(&smi_info.si_timer))
    smi_info.timer_running = false;
    }
    out:
    return si_sm_result;
    }
#[no_mangle]
unsafe extern "C" fn check_start_timer_thread(smi_info: *mut smi_info) {
    static void check_start_timer_thread(struct smi_info *smi_info)
    {
    if (smi_info.si_state == SI_NORMAL && smi_info.curr_msg == core::ptr::null_mut()) {
    smi_mod_timer(smi_info, jiffies + SI_TIMEOUT_JIFFIES);
    if (smi_info.thread)
    wake_up_process(smi_info.thread);
    start_next_msg(smi_info);
    smi_event_handler(smi_info, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn flush_messages(send_info: *mut c_void) {
    static void flush_messages(void *send_info)
    {
    struct smi_info *smi_info = send_info;
    enum si_sm_result result;
//
// Currently, this function is called only in run-to-completion
// mode.  This means we are single-threaded, no need for locks.
//
    result = smi_event_handler(smi_info, 0);
    while (result != SI_SM_IDLE && result != SI_SM_HOSED) {
    udelay(SI_SHORT_TIMEOUT_USEC);
    result = smi_event_handler(smi_info, SI_SHORT_TIMEOUT_USEC);
    }
    }
#[no_mangle]
unsafe extern "C" fn sender(send_info: *mut c_void, msg: *mut ipmi_smi_msg) -> c_int {
    static int sender(void *send_info, struct ipmi_smi_msg *msg)
    {
    struct smi_info   *smi_info = send_info;
    unsigned long     flags;
    let mut rv: c_int = IPMI_CC_NO_ERROR;
    debug_timestamp(smi_info, "Enqueue");
//
// Check here for run to completion mode.  A check under lock is
// later.
//
    if (smi_info.si_state == SI_HOSED)
    return IPMI_BUS_ERR;
    if (smi_info.run_to_completion) {
//
// If we are running to completion, start it.  Upper
// layer will call flush_messages to clear it out.
//
    smi_info.waiting_msg = msg;
    return IPMI_CC_NO_ERROR;
    }
    spin_lock_irqsave(&smi_info.si_lock, flags);
    if (smi_info.si_state == SI_HOSED) {
    rv = IPMI_BUS_ERR;
    } else {
    BUG_ON(smi_info.waiting_msg);
    smi_info.waiting_msg = msg;
    check_start_timer_thread(smi_info);
    }
    spin_unlock_irqrestore(&smi_info.si_lock, flags);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn set_run_to_completion(send_info: *mut c_void, i_run_to_completion: bool) {
    static void set_run_to_completion(void *send_info, bool i_run_to_completion)
    {
    struct smi_info   *smi_info = send_info;
    smi_info.run_to_completion = i_run_to_completion;
    if (i_run_to_completion)
    flush_messages(smi_info);
    }
//
// Use -1 as a special constant to tell that we are spinning in kipmid
// looking for something and not delaying between checks
//

    static inline bool ipmi_thread_busy_wait(enum si_sm_result smi_result,
    const struct smi_info *smi_info,
    ktime_t *busy_until)
    {
    let mut max_busy_us: c_uint = 0;
    if (smi_info.si_num < num_max_busy_us)
    max_busy_us = kipmid_max_busy_us[smi_info.si_num];
    if (max_busy_us == 0 || smi_result != SI_SM_CALL_WITH_DELAY)
// busy_until = IPMI_TIME_NOT_BUSY;
#[no_mangle]
pub unsafe extern "C" fn if(IPMI_TIME_NOT_BUSY: *mut *mut busy_until ==) -> else {
// busy_until = ktime_get() + max_busy_us * NSEC_PER_USEC;
    } else {
    if (unlikely(ktime_get() > *busy_until)) {
// busy_until = IPMI_TIME_NOT_BUSY;
    return false;
    }
    }
    return true;
    }
//
// A busy-waiting loop for speeding up IPMI operation.
//
// Lousy hardware makes this hard.  This is only enabled for systems
// that are not BT and do not have interrupts.  It starts spinning
// when an operation is complete or until max_busy tells it to stop
// (if that is enabled).  See the paragraph on kimid_max_busy_us in
// Documentation/driver-api/ipmi.rst for details.
//
#[no_mangle]
unsafe extern "C" fn ipmi_thread(data: *mut c_void) -> c_int {
    static int ipmi_thread(void *data)
    {
    struct smi_info *smi_info = data;
    unsigned long flags;
    enum si_sm_result smi_result;
    let mut busy_until: ktime_t = IPMI_TIME_NOT_BUSY;
    set_user_nice(current, MAX_NICE);
    while (!kthread_should_stop()) {
    int busy_wait;
    spin_lock_irqsave(&(smi_info.si_lock), flags);
    smi_result = smi_event_handler(smi_info, 0);
//
// If the driver is doing something, there is a possible
// race with the timer.  If the timer handler see idle,
// and the thread here sees something else, the timer
// handler won't restart the timer even though it is
// required.  So start it here if necessary.
//
    if (smi_result != SI_SM_IDLE && !smi_info.timer_running)
    smi_mod_timer(smi_info, jiffies + SI_TIMEOUT_JIFFIES);
    spin_unlock_irqrestore(&(smi_info.si_lock), flags);
    busy_wait = ipmi_thread_busy_wait(smi_result, smi_info,
    &busy_until);
    if (smi_result == SI_SM_CALL_WITHOUT_DELAY) {
    ; /* do nothing */
    } else if (smi_result == SI_SM_CALL_WITH_DELAY && busy_wait) {
//
// In maintenance mode we run as fast as
// possible to allow firmware updates to
// complete as fast as possible, but normally
// don't bang on the scheduler.
//
    if (smi_info.in_maintenance_mode)
    schedule();
    else
    usleep_range(100, 200);
    } else if (smi_result == SI_SM_IDLE) {
    if (atomic_read(&smi_info.need_watch)) {
    schedule_timeout_interruptible(100);
    } else {
// Wait to be woken up when we are needed.
    __set_current_state(TASK_INTERRUPTIBLE);
    schedule();
    }
    } else {
    schedule_timeout_interruptible(1);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn poll(send_info: *mut c_void) {
    static void poll(void *send_info)
    {
    struct smi_info *smi_info = send_info;
    let mut flags: c_ulong = 0;
    let mut run_to_completion: bool = smi_info.run_to_completion;
//
// Make sure there is some delay in the poll loop so we can
// drive time forward and timeout things.
//
    udelay(10);
    if (!run_to_completion)
    spin_lock_irqsave(&smi_info.si_lock, flags);
    smi_event_handler(smi_info, 10);
    if (!run_to_completion)
    spin_unlock_irqrestore(&smi_info.si_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn request_events(send_info: *mut c_void) {
    static void request_events(void *send_info)
    {
    struct smi_info *smi_info = send_info;
    if (!smi_info.has_event_buffer)
    return;
    atomic_set(&smi_info.req_events, 1);
    }
#[no_mangle]
unsafe extern "C" fn set_need_watch(send_info: *mut c_void, watch_mask: c_uint) {
    static void set_need_watch(void *send_info, unsigned int watch_mask)
    {
    struct smi_info *smi_info = send_info;
    unsigned long flags;
    int enable;
    enable = !!watch_mask;
    atomic_set(&smi_info.need_watch, enable);
    spin_lock_irqsave(&smi_info.si_lock, flags);
    check_start_timer_thread(smi_info);
    spin_unlock_irqrestore(&smi_info.si_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn smi_timeout(t: *mut timer_list) {
    static void smi_timeout(struct timer_list *t)
    {
    struct smi_info   *smi_info = timer_container_of(smi_info, t,
    si_timer);
    enum si_sm_result smi_result;
    unsigned long     flags;
    unsigned long     jiffies_now;
    long              time_diff;
    long		  timeout;
    spin_lock_irqsave(&(smi_info.si_lock), flags);
    debug_timestamp(smi_info, "Timer");
    if (smi_info.si_state == SI_HOSED)
// Try something to see if the BMC is now operational.
    start_get_flags(smi_info);
    jiffies_now = jiffies;
    time_diff = (((long)jiffies_now - (long)smi_info.last_timeout_jiffies)
// SI_USEC_PER_JIFFY);
    smi_result = smi_event_handler(smi_info, time_diff);
    if (smi_info.si_state == SI_HOSED) {
    timeout = jiffies + SI_TIMEOUT_HOSED;
    } else if ((smi_info.io.irq) && (!smi_info.interrupt_disabled)) {
// Running with interrupts, only do long timeouts.
    timeout = jiffies + SI_TIMEOUT_JIFFIES;
    smi_inc_stat(smi_info, long_timeouts);
    } else if (smi_result == SI_SM_CALL_WITH_DELAY) {
//
// If the state machine asks for a short delay, then shorten
// the timer timeout.
//
    smi_inc_stat(smi_info, short_timeouts);
    timeout = jiffies + 1;
    } else {
    smi_inc_stat(smi_info, long_timeouts);
    timeout = jiffies + SI_TIMEOUT_JIFFIES;
    }
    if (smi_result != SI_SM_IDLE)
    smi_mod_timer(smi_info, timeout);
    else
    smi_info.timer_running = false;
    spin_unlock_irqrestore(&(smi_info.si_lock), flags);
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    irqreturn_t ipmi_si_irq_handler(int irq, void *data)
    {
    struct smi_info *smi_info = data;
    unsigned long   flags;
    if (smi_info.io.si_info.type == SI_BT)
// We need to clear the IRQ flag for the BT interface.
    smi_info.io.outputb(&smi_info.io, IPMI_BT_INTMASK_REG,
    IPMI_BT_INTMASK_CLEAR_IRQ_BIT
    | IPMI_BT_INTMASK_ENABLE_IRQ_BIT);
    spin_lock_irqsave(&(smi_info.si_lock), flags);
    smi_inc_stat(smi_info, interrupts);
    debug_timestamp(smi_info, "Interrupt");
    smi_event_handler(smi_info, 0);
    spin_unlock_irqrestore(&(smi_info.si_lock), flags);
    return IRQ_HANDLED;
    }
    static int smi_start_processing(void            *send_info,
    struct ipmi_smi *intf)
    {
    struct smi_info *new_smi = send_info;
    let mut enable: c_int = 0;
    new_smi.intf = intf;
// Set up the timer that drives the interface.
    timer_setup(&new_smi.si_timer, smi_timeout, 0);
    new_smi.timer_can_start = true;
    smi_mod_timer(new_smi, jiffies + SI_TIMEOUT_JIFFIES);
// Try to claim any interrupts.
    if (new_smi.io.irq_setup) {
    new_smi.io.irq_handler_data = new_smi;
    new_smi.io.irq_setup(&new_smi.io);
    }
//
// Check if the user forcefully enabled the daemon.
//
    if (new_smi.si_num < num_force_kipmid)
    enable = force_kipmid[new_smi.si_num];
//
// The BT interface is efficient enough to not need a thread,
// and there is no need for a thread if we have interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn if(!new_smi->io.irq: new_smi->io.si_info->type != SI_BT &&) -> else {
    else if (new_smi.io.si_info.type != SI_BT && !new_smi.io.irq)
    enable = 1;
    if (enable) {
    new_smi.thread = kthread_run(ipmi_thread, new_smi,
    "kipmi%d", new_smi.si_num);
    if (IS_ERR(new_smi.thread)) {
    dev_notice(new_smi.io.dev,
    "Could not start kernel thread due to error %ld, only using timers to drive the interface\n",
    PTR_ERR(new_smi.thread));
    new_smi.thread = core::ptr::null_mut();
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_smi_info(send_info: *mut c_void, data: *mut ipmi_smi_info) -> c_int {
    static int get_smi_info(void *send_info, struct ipmi_smi_info *data)
    {
    struct smi_info *smi = send_info;
    data.addr_src = smi.io.addr_source;
    data.dev = smi.io.dev;
    data.addr_info = smi.io.addr_info;
    get_device(smi.io.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_maintenance_mode(send_info: *mut c_void, enable: bool) {
    static void set_maintenance_mode(void *send_info, bool enable)
    {
    struct smi_info   *smi_info = send_info;
    if (!enable)
    atomic_set(&smi_info.req_events, 0);
    smi_info.in_maintenance_mode = enable;
    }
    static void shutdown_smi(void *send_info);
    static const struct ipmi_smi_handlers handlers = {
    .owner                  = THIS_MODULE,
    .start_processing       = smi_start_processing,
    .shutdown               = shutdown_smi,
    .get_smi_info		= get_smi_info,
    .sender			= sender,
    .request_events		= request_events,
    .set_need_watch		= set_need_watch,
    .set_maintenance_mode   = set_maintenance_mode,
    .set_run_to_completion  = set_run_to_completion,
    .flush_messages		= flush_messages,
    .poll			= poll,
    };
    static LIST_HEAD(smi_infos);
    static DEFINE_MUTEX(smi_infos_lock);
    static int smi_num; /* Used to sequence the SMIs */
    static const char * const addr_space_to_str[] = { "i/o", "mem" };
    module_param_array(force_kipmid, int, &num_force_kipmid, 0);
    MODULE_PARM_DESC(force_kipmid,
    "Force the kipmi daemon to be enabled (1) or disabled(0).  Normally the IPMI driver auto-detects this, but the value may be overridden by this parm.");
    module_param(unload_when_empty, bool, 0);
    MODULE_PARM_DESC(unload_when_empty,
    "Unload the module if no interfaces are specified or found, default is 1.  Setting to 0 is useful for hot add of devices using hotmod.");
    module_param_array(kipmid_max_busy_us, uint, &num_max_busy_us, 0644);
    MODULE_PARM_DESC(kipmid_max_busy_us,
    "Max time (in microseconds) to busy-wait for IPMI data before sleeping. 0 (default) means to wait forever. Set to 100-500 if kipmid is using up a lot of CPU time.");
#[no_mangle]
pub unsafe extern "C" fn ipmi_irq_finish_setup(io: *mut si_sm_io) {
    void ipmi_irq_finish_setup(struct si_sm_io *io)
    {
    if (io.si_info.type == SI_BT)
// Enable the interrupt in the BT interface.
    io.outputb(io, IPMI_BT_INTMASK_REG,
    IPMI_BT_INTMASK_ENABLE_IRQ_BIT);
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_irq_start_cleanup(io: *mut si_sm_io) {
    void ipmi_irq_start_cleanup(struct si_sm_io *io)
    {
    if (io.si_info.type == SI_BT)
// Disable the interrupt in the BT interface.
    io.outputb(io, IPMI_BT_INTMASK_REG, 0);
    }
#[no_mangle]
unsafe extern "C" fn std_irq_cleanup(io: *mut si_sm_io) {
    static void std_irq_cleanup(struct si_sm_io *io)
    {
    ipmi_irq_start_cleanup(io);
    free_irq(io.irq, io.irq_handler_data);
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_std_irq_setup(io: *mut si_sm_io) -> c_int {
    int ipmi_std_irq_setup(struct si_sm_io *io)
    {
    int rv;
    if (!io.irq)
    return 0;
    rv = request_irq(io.irq,
    ipmi_si_irq_handler,
    IRQF_SHARED,
    SI_DEVICE_NAME,
    io.irq_handler_data);
    if (rv) {
    dev_warn(io.dev, "%s unable to claim interrupt %d, running polled\n",
    SI_DEVICE_NAME, io.irq);
    io.irq = 0;
    } else {
    io.irq_cleanup = std_irq_cleanup;
    ipmi_irq_finish_setup(io);
    dev_info(io.dev, "Using irq %d\n", io.irq);
    }
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn wait_for_msg_done(smi_info: *mut smi_info) -> c_int {
    static int wait_for_msg_done(struct smi_info *smi_info)
    {
    enum si_sm_result     smi_result;
    smi_result = smi_info.handlers.event(smi_info.si_sm, 0);
    for (;;) {
    if (smi_result == SI_SM_CALL_WITH_DELAY ||
    smi_result == SI_SM_CALL_WITH_TICK_DELAY) {
    schedule_timeout_uninterruptible(1);
    smi_result = smi_info.handlers.event(
    smi_info.si_sm, jiffies_to_usecs(1));
    } else if (smi_result == SI_SM_CALL_WITHOUT_DELAY) {
    smi_result = smi_info.handlers.event(
    smi_info.si_sm, 0);
    } else
    break;
    }
    if (smi_result == SI_SM_HOSED)
//
// We couldn't get the state machine to run, so whatever's at
// the port is probably not an IPMI SMI interface.
//
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn try_get_dev_id(smi_info: *mut smi_info) -> c_int {
    static int try_get_dev_id(struct smi_info *smi_info)
    {
    unsigned char         msg[2];
    unsigned char         *resp;
    unsigned long         resp_len;
    let mut rv: c_int = 0;
    let mut retry_count: c_uint = 0;
    resp = kmalloc(IPMI_MAX_MSG_LENGTH, GFP_KERNEL);
    if (!resp)
    return -ENOMEM;
//
// Do a Get Device ID command, since it comes back with some
// useful info.
//
    msg[0] = IPMI_NETFN_APP_REQUEST << 2;
    msg[1] = IPMI_GET_DEVICE_ID_CMD;
    retry:
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, 2);
    rv = wait_for_msg_done(smi_info);
    if (rv)
    goto out;
    resp_len = smi_info.handlers.get_result(smi_info.si_sm,
    resp, IPMI_MAX_MSG_LENGTH);
// Check and record info from the get device id, in case we need it.
    rv = ipmi_demangle_device_id(resp[0] >> 2, resp[1],
    resp + 2, resp_len - 2, &smi_info.device_id);
    if (rv) {
// record completion code
    let mut cc: c_uchar = *(resp + 2);
    if (cc != IPMI_CC_NO_ERROR &&
    ++retry_count <= GET_DEVICE_ID_MAX_RETRY) {
    dev_warn_ratelimited(smi_info.io.dev,
    "BMC returned 0x%2.2x, retry get bmc device id\n",
    cc);
    goto retry;
    }
    }
    out:
    kfree(resp);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn get_global_enables(smi_info: *mut smi_info, enables: *mut u8) -> c_int {
    static int get_global_enables(struct smi_info *smi_info, u8 *enables)
    {
    unsigned char         msg[3];
    unsigned char         *resp;
    unsigned long         resp_len;
    int                   rv;
    resp = kmalloc(IPMI_MAX_MSG_LENGTH, GFP_KERNEL);
    if (!resp)
    return -ENOMEM;
    msg[0] = IPMI_NETFN_APP_REQUEST << 2;
    msg[1] = IPMI_GET_BMC_GLOBAL_ENABLES_CMD;
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, 2);
    rv = wait_for_msg_done(smi_info);
    if (rv) {
    dev_warn(smi_info.io.dev,
    "Error getting response from get global enables command: %d\n",
    rv);
    goto out;
    }
    resp_len = smi_info.handlers.get_result(smi_info.si_sm,
    resp, IPMI_MAX_MSG_LENGTH);
    if (resp_len < 4 ||
    resp[0] != (IPMI_NETFN_APP_REQUEST | 1) << 2 ||
    resp[1] != IPMI_GET_BMC_GLOBAL_ENABLES_CMD   ||
    resp[2] != 0) {
    dev_warn(smi_info.io.dev,
    "Invalid return from get global enables command: %ld %x %x %x\n",
    resp_len, resp[0], resp[1], resp[2]);
    rv = -EINVAL;
    goto out;
    } else {
// enables = resp[3];
    }
    out:
    kfree(resp);
    return rv;
    }
//
// Returns 1 if it gets an error from the command.
//
#[no_mangle]
unsafe extern "C" fn set_global_enables(smi_info: *mut smi_info, enables: u8) -> c_int {
    static int set_global_enables(struct smi_info *smi_info, u8 enables)
    {
    unsigned char         msg[3];
    unsigned char         *resp;
    unsigned long         resp_len;
    int                   rv;
    resp = kmalloc(IPMI_MAX_MSG_LENGTH, GFP_KERNEL);
    if (!resp)
    return -ENOMEM;
    msg[0] = IPMI_NETFN_APP_REQUEST << 2;
    msg[1] = IPMI_SET_BMC_GLOBAL_ENABLES_CMD;
    msg[2] = enables;
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, 3);
    rv = wait_for_msg_done(smi_info);
    if (rv) {
    dev_warn(smi_info.io.dev,
    "Error getting response from set global enables command: %d\n",
    rv);
    goto out;
    }
    resp_len = smi_info.handlers.get_result(smi_info.si_sm,
    resp, IPMI_MAX_MSG_LENGTH);
    if (resp_len < 3 ||
    resp[0] != (IPMI_NETFN_APP_REQUEST | 1) << 2 ||
    resp[1] != IPMI_SET_BMC_GLOBAL_ENABLES_CMD) {
    dev_warn(smi_info.io.dev,
    "Invalid return from set global enables command: %ld %x %x\n",
    resp_len, resp[0], resp[1]);
    rv = -EINVAL;
    goto out;
    }
    if (resp[2] != 0)
    rv = 1;
    out:
    kfree(resp);
    return rv;
    }
//
// Some BMCs do not support clearing the receive irq bit in the global
// enables (even if they don't support interrupts on the BMC).  Check
// for this and handle it properly.
//
#[no_mangle]
unsafe extern "C" fn check_clr_rcv_irq(smi_info: *mut smi_info) {
    static void check_clr_rcv_irq(struct smi_info *smi_info)
    {
    let mut enables: u8 = 0;
    int rv;
    rv = get_global_enables(smi_info, &enables);
    if (!rv) {
    if ((enables & IPMI_BMC_RCV_MSG_INTR) == 0)
// Already clear, should work ok.
    return;
    enables &= ~IPMI_BMC_RCV_MSG_INTR;
    rv = set_global_enables(smi_info, enables);
    }
    if (rv < 0) {
    dev_err(smi_info.io.dev,
    "Cannot check clearing the rcv irq: %d\n", rv);
    return;
    }
    if (rv) {
//
// An error when setting the event buffer bit means
// clearing the bit is not supported.
//
    dev_warn(smi_info.io.dev,
    "The BMC does not support clearing the recv irq bit, compensating, but the BMC needs to be fixed.\n");
    smi_info.cannot_disable_irq = true;
    }
    }
//
// Some BMCs do not support setting the interrupt bits in the global
// enables even if they support interrupts.  Clearly bad, but we can
// compensate.
//
#[no_mangle]
unsafe extern "C" fn check_set_rcv_irq(smi_info: *mut smi_info) {
    static void check_set_rcv_irq(struct smi_info *smi_info)
    {
    let mut enables: u8 = 0;
    int rv;
    if (!smi_info.io.irq)
    return;
    rv = get_global_enables(smi_info, &enables);
    if (!rv) {
    enables |= IPMI_BMC_RCV_MSG_INTR;
    rv = set_global_enables(smi_info, enables);
    }
    if (rv < 0) {
    dev_err(smi_info.io.dev,
    "Cannot check setting the rcv irq: %d\n", rv);
    return;
    }
    if (rv) {
//
// An error when setting the event buffer bit means
// setting the bit is not supported.
//
    dev_warn(smi_info.io.dev,
    "The BMC does not support setting the recv irq bit, compensating, but the BMC needs to be fixed.\n");
    smi_info.cannot_disable_irq = true;
    smi_info.irq_enable_broken = true;
    }
    }
#[no_mangle]
unsafe extern "C" fn try_enable_event_buffer(smi_info: *mut smi_info) -> c_int {
    static int try_enable_event_buffer(struct smi_info *smi_info)
    {
    unsigned char         msg[3];
    unsigned char         *resp;
    unsigned long         resp_len;
    let mut rv: c_int = 0;
    resp = kmalloc(IPMI_MAX_MSG_LENGTH, GFP_KERNEL);
    if (!resp)
    return -ENOMEM;
    msg[0] = IPMI_NETFN_APP_REQUEST << 2;
    msg[1] = IPMI_GET_BMC_GLOBAL_ENABLES_CMD;
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, 2);
    rv = wait_for_msg_done(smi_info);
    if (rv) {
    pr_warn("Error getting response from get global enables command, the event buffer is not enabled\n");
    goto out;
    }
    resp_len = smi_info.handlers.get_result(smi_info.si_sm,
    resp, IPMI_MAX_MSG_LENGTH);
    if (resp_len < 4 ||
    resp[0] != (IPMI_NETFN_APP_REQUEST | 1) << 2 ||
    resp[1] != IPMI_GET_BMC_GLOBAL_ENABLES_CMD   ||
    resp[2] != 0) {
    pr_warn("Invalid return from get global enables command, cannot enable the event buffer\n");
    rv = -EINVAL;
    goto out;
    }
    if (resp[3] & IPMI_BMC_EVT_MSG_BUFF) {
// buffer is already enabled, nothing to do.
    smi_info.supports_event_msg_buff = true;
    goto out;
    }
    msg[0] = IPMI_NETFN_APP_REQUEST << 2;
    msg[1] = IPMI_SET_BMC_GLOBAL_ENABLES_CMD;
    msg[2] = resp[3] | IPMI_BMC_EVT_MSG_BUFF;
    smi_info.handlers.start_transaction(smi_info.si_sm, msg, 3);
    rv = wait_for_msg_done(smi_info);
    if (rv) {
    pr_warn("Error getting response from set global, enables command, the event buffer is not enabled\n");
    goto out;
    }
    resp_len = smi_info.handlers.get_result(smi_info.si_sm,
    resp, IPMI_MAX_MSG_LENGTH);
    if (resp_len < 3 ||
    resp[0] != (IPMI_NETFN_APP_REQUEST | 1) << 2 ||
    resp[1] != IPMI_SET_BMC_GLOBAL_ENABLES_CMD) {
    pr_warn("Invalid return from get global, enables command, not enable the event buffer\n");
    rv = -EINVAL;
    goto out;
    }
    if (resp[2] != 0)
//
// An error when setting the event buffer bit means
// that the event buffer is not supported.
//
    rv = -ENOENT;
    else
    smi_info.supports_event_msg_buff = true;
    out:
    kfree(resp);
    return rv;
    }

    static ssize_t name##_show(struct device *dev,			\
    struct device_attribute *attr,		\
    char *buf)					\
    {									\
    struct smi_info *smi_info = dev_get_drvdata(dev);		\
    \
    return sysfs_emit(buf, "%u\n", smi_get_stat(smi_info, name));	\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    static ssize_t type_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct smi_info *smi_info = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%s\n", si_to_str[smi_info.io.si_info.type]);
    }
    static DEVICE_ATTR_RO(type);
    static ssize_t interrupts_enabled_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct smi_info *smi_info = dev_get_drvdata(dev);
    let mut enabled: c_int = smi_info.io.irq && !smi_info.interrupt_disabled;
    return sysfs_emit(buf, "%d\n", enabled);
    }
    static DEVICE_ATTR_RO(interrupts_enabled);
    IPMI_SI_ATTR(short_timeouts);
    IPMI_SI_ATTR(long_timeouts);
    IPMI_SI_ATTR(idles);
    IPMI_SI_ATTR(interrupts);
    IPMI_SI_ATTR(attentions);
    IPMI_SI_ATTR(flag_fetches);
    IPMI_SI_ATTR(hosed_count);
    IPMI_SI_ATTR(complete_transactions);
    IPMI_SI_ATTR(events);
    IPMI_SI_ATTR(watchdog_pretimeouts);
    IPMI_SI_ATTR(incoming_messages);
    static ssize_t params_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct smi_info *smi_info = dev_get_drvdata(dev);
    return sysfs_emit(buf,
    "%s,%s,0x%lx,rsp=%d,rsi=%d,rsh=%d,irq=%d,ipmb=%d\n",
    si_to_str[smi_info.io.si_info.type],
    addr_space_to_str[smi_info.io.addr_space],
    smi_info.io.addr_data,
    smi_info.io.regspacing,
    smi_info.io.regsize,
    smi_info.io.regshift,
    smi_info.io.irq,
    smi_info.io.slave_addr);
    }
    static DEVICE_ATTR_RO(params);
    static struct attribute *ipmi_si_dev_attrs[] = {
    &dev_attr_type.attr,
    &dev_attr_interrupts_enabled.attr,
    &dev_attr_short_timeouts.attr,
    &dev_attr_long_timeouts.attr,
    &dev_attr_idles.attr,
    &dev_attr_interrupts.attr,
    &dev_attr_attentions.attr,
    &dev_attr_flag_fetches.attr,
    &dev_attr_hosed_count.attr,
    &dev_attr_complete_transactions.attr,
    &dev_attr_events.attr,
    &dev_attr_watchdog_pretimeouts.attr,
    &dev_attr_incoming_messages.attr,
    &dev_attr_params.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ipmi_si_dev_attr_group = {
    .attrs		= ipmi_si_dev_attrs,
    };
//
// oem_data_avail_to_receive_msg_avail
// @info - smi_info structure with msg_flags set
//
// Converts flags from OEM_DATA_AVAIL to RECEIVE_MSG_AVAIL
// Returns 1 indicating need to re-run handle_flags().
//
#[no_mangle]
unsafe extern "C" fn oem_data_avail_to_receive_msg_avail(smi_info: *mut smi_info) -> c_int {
    static int oem_data_avail_to_receive_msg_avail(struct smi_info *smi_info)
    {
    smi_info.msg_flags = ((smi_info.msg_flags & ~OEM_DATA_AVAIL) |
    RECEIVE_MSG_AVAIL);
    return 1;
    }
//
// setup_dell_poweredge_oem_data_handler
// @info - smi_info.device_id must be populated
//
// Systems that match, but have firmware version < 1.40 may assert
// OEM0_DATA_AVAIL on their own, without being told via Set Flags that
// it's safe to do so.  Such systems will de-assert OEM1_DATA_AVAIL
// upon receipt of IPMI_GET_MSG_CMD, so we should treat these flags
// as RECEIVE_MSG_AVAIL instead.
//
// As Dell has no plans to release IPMI 1.5 firmware that *ever
// assert the OEM[012] bits, and if it did, the driver would have to
// change to handle that properly, we don't actually check for the
// firmware version.
// Device ID = 0x20                BMC on PowerEdge 8G servers
// Device Revision = 0x80
// Firmware Revision1 = 0x01       BMC version 1.40
// Firmware Revision2 = 0x40       BCD encoded
// IPMI Version = 0x51             IPMI 1.5
// Manufacturer ID = A2 02 00      Dell IANA
//
// Additionally, PowerEdge systems with IPMI < 1.5 may also assert
// OEM0_DATA_AVAIL and needs to be treated as RECEIVE_MSG_AVAIL.
//
pub const DELL_POWEREDGE_8G_BMC_DEVICE_ID: c_uint = 0x20;
pub const DELL_POWEREDGE_8G_BMC_DEVICE_REV: c_uint = 0x80;
pub const DELL_POWEREDGE_8G_BMC_IPMI_VERSION: c_uint = 0x51;
pub const DELL_IANA_MFR_ID: c_uint = 0x0002a2;
#[no_mangle]
unsafe extern "C" fn setup_dell_poweredge_oem_data_handler(smi_info: *mut smi_info) {
    static void setup_dell_poweredge_oem_data_handler(struct smi_info *smi_info)
    {
    struct ipmi_device_id *id = &smi_info.device_id;
    if (id.manufacturer_id == DELL_IANA_MFR_ID) {
    if (id.device_id       == DELL_POWEREDGE_8G_BMC_DEVICE_ID  &&
    id.device_revision == DELL_POWEREDGE_8G_BMC_DEVICE_REV &&
    id.ipmi_version   == DELL_POWEREDGE_8G_BMC_IPMI_VERSION) {
    smi_info.oem_data_avail_handler =
    oem_data_avail_to_receive_msg_avail;
    } else if (ipmi_version_major(id) < 1 ||
    (ipmi_version_major(id) == 1 &&
    ipmi_version_minor(id) < 5)) {
    smi_info.oem_data_avail_handler =
    oem_data_avail_to_receive_msg_avail;
    }
    }
    }
pub const CANNOT_RETURN_REQUESTED_LENGTH: c_uint = 0xCA;
#[no_mangle]
unsafe extern "C" fn return_hosed_msg_badsize(smi_info: *mut smi_info) {
    static void return_hosed_msg_badsize(struct smi_info *smi_info)
    {
    struct ipmi_smi_msg *msg = smi_info.curr_msg;
// Make it a response
    msg.rsp[0] = msg.data[0] | 4;
    msg.rsp[1] = msg.data[1];
    msg.rsp[2] = CANNOT_RETURN_REQUESTED_LENGTH;
    msg.rsp_size = 3;
    smi_info.curr_msg = core::ptr::null_mut();
    deliver_recv_msg(smi_info, msg);
    }
//
// dell_poweredge_bt_xaction_handler
// @info - smi_info.device_id must be populated
//
// Dell PowerEdge servers with the BT interface (x6xx and 1750) will
// not respond to a Get SDR command if the length of the data
// requested is exactly 0x3A, which leads to command timeouts and no
// data returned.  This intercepts such commands, and causes userspace
// callers to try again with a different-sized buffer, which succeeds.
//
pub const STORAGE_NETFN: c_uint = 0x0A;
pub const STORAGE_CMD_GET_SDR: c_uint = 0x23;
    static int dell_poweredge_bt_xaction_handler(struct notifier_block *self,
    unsigned long unused,
    void *in)
    {
    struct smi_info *smi_info = in;
    unsigned char *data = smi_info.curr_msg.data;
    let mut size: c_uint = smi_info.curr_msg.data_size;
    if (size >= 8 &&
    (data[0]>>2) == STORAGE_NETFN &&
    data[1] == STORAGE_CMD_GET_SDR &&
    data[7] == 0x3A) {
    return_hosed_msg_badsize(smi_info);
    return NOTIFY_STOP;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block dell_poweredge_bt_xaction_notifier = {
    .notifier_call	= dell_poweredge_bt_xaction_handler,
    };
//
// setup_dell_poweredge_bt_xaction_handler
// @info - smi_info.device_id must be filled in already
//
// Fills in smi_info.device_id.start_transaction_pre_hook
// when we know what function to use there.
//
    static void
    setup_dell_poweredge_bt_xaction_handler(struct smi_info *smi_info)
    {
    struct ipmi_device_id *id = &smi_info.device_id;
    if (id.manufacturer_id == DELL_IANA_MFR_ID &&
    smi_info.io.si_info.type == SI_BT)
    register_xaction_notifier(&dell_poweredge_bt_xaction_notifier);
    }
//
// setup_oem_data_handler
// @info - smi_info.device_id must be filled in already
//
// Fills in smi_info.device_id.oem_data_available_handler
// when we know what function to use there.
//
#[no_mangle]
unsafe extern "C" fn setup_oem_data_handler(smi_info: *mut smi_info) {
    static void setup_oem_data_handler(struct smi_info *smi_info)
    {
    setup_dell_poweredge_oem_data_handler(smi_info);
    }
#[no_mangle]
unsafe extern "C" fn setup_xaction_handlers(smi_info: *mut smi_info) {
    static void setup_xaction_handlers(struct smi_info *smi_info)
    {
    setup_dell_poweredge_bt_xaction_handler(smi_info);
    }
#[no_mangle]
unsafe extern "C" fn check_for_broken_irqs(smi_info: *mut smi_info) {
    static void check_for_broken_irqs(struct smi_info *smi_info)
    {
    check_clr_rcv_irq(smi_info);
    check_set_rcv_irq(smi_info);
    }
#[no_mangle]
pub unsafe extern "C" fn stop_timer_and_thread(smi_info: *mut smi_info) {
    static inline void stop_timer_and_thread(struct smi_info *smi_info)
    {
    if (smi_info.thread != core::ptr::null_mut()) {
    kthread_stop(smi_info.thread);
    smi_info.thread = core::ptr::null_mut();
    }
    smi_info.timer_can_start = false;
    timer_delete_sync(&smi_info.si_timer);
    }
    static struct smi_info *find_dup_si(struct smi_info *info)
    {
    struct smi_info *e;
    list_for_each_entry(e, &smi_infos, link) {
    if (e.io.addr_space != info.io.addr_space)
    continue;
    if (e.io.addr_data == info.io.addr_data) {
//
// This is a cheap hack, ACPI doesn't have a defined
// slave address but SMBIOS does.  Pick it up from
// any source that has it available.
//
    if (info.io.slave_addr && !e.io.slave_addr)
    e.io.slave_addr = info.io.slave_addr;
    return e;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_add_smi(io: *mut si_sm_io) -> c_int {
    int ipmi_si_add_smi(struct si_sm_io *io)
    {
    let mut rv: c_int = 0;
    struct smi_info *new_smi, *dup;
//
// If the user gave us a hard-coded device at the same
// address, they presumably want us to use it and not what is
// in the firmware.
//
    if (io.addr_source != SI_HARDCODED && io.addr_source != SI_HOTMOD &&
    ipmi_si_hardcode_match(io.addr_space, io.addr_data)) {
    dev_info(io.dev,
    "Hard-coded device at this address already exists");
    return -ENODEV;
    }
    if (!io.io_setup) {
    if (IS_ENABLED(CONFIG_HAS_IOPORT) &&
    io.addr_space == IPMI_IO_ADDR_SPACE) {
    io.io_setup = ipmi_si_port_setup;
    } else if (io.addr_space == IPMI_MEM_ADDR_SPACE) {
    io.io_setup = ipmi_si_mem_setup;
    } else {
    return -EINVAL;
    }
    }
    new_smi = kzalloc_obj(*new_smi);
    if (!new_smi)
    return -ENOMEM;
    spin_lock_init(&new_smi.si_lock);
    INIT_WORK(&new_smi.init_work, smi_init_work_fn);
    new_smi.io = *io;
    mutex_lock(&smi_infos_lock);
    dup = find_dup_si(new_smi);
    if (dup) {
    if (new_smi.io.addr_source == SI_ACPI &&
    dup.io.addr_source == SI_SMBIOS) {
// We prefer ACPI over SMBIOS.
    dev_info(dup.io.dev,
    "Removing SMBIOS-specified %s state machine in favor of ACPI\n",
    si_to_str[new_smi.io.si_info.type]);
    list_del(&dup.link);
    mutex_unlock(&smi_infos_lock);
    cleanup_one_si(dup);
    mutex_lock(&smi_infos_lock);
    } else {
    dev_info(new_smi.io.dev,
    "%s-specified %s state machine: duplicate\n",
    ipmi_addr_src_to_str(new_smi.io.addr_source),
    si_to_str[new_smi.io.si_info.type]);
    rv = -EBUSY;
    kfree(new_smi);
    goto out_err;
    }
    }
    pr_info("Adding %s-specified %s state machine\n",
    ipmi_addr_src_to_str(new_smi.io.addr_source),
    si_to_str[new_smi.io.si_info.type]);
    list_add_tail(&new_smi.link, &smi_infos);
    if (initialized) {
    if (IS_ENABLED(CONFIG_IPMI_SI_ASYNC_INIT))
    queue_work(system_dfl_wq, &new_smi.init_work);
    else
    rv = try_smi_init(new_smi);
    }
    out_err:
    mutex_unlock(&smi_infos_lock);
    return rv;
    }
//
// Try to start up an interface.  Must be called with smi_infos_lock
// held, primarily to keep smi_num consistent, we only one to do these
// one at a time.
//
#[no_mangle]
unsafe extern "C" fn try_smi_init(new_smi: *mut smi_info) -> c_int {
    static int try_smi_init(struct smi_info *new_smi)
    {
    let mut rv: c_int = 0;
    int i;
    pr_info("Trying %s-specified %s state machine at %s address 0x%lx, slave address 0x%x, irq %d\n",
    ipmi_addr_src_to_str(new_smi.io.addr_source),
    si_to_str[new_smi.io.si_info.type],
    addr_space_to_str[new_smi.io.addr_space],
    new_smi.io.addr_data,
    new_smi.io.slave_addr, new_smi.io.irq);
    switch (new_smi.io.si_info.type) {
    case SI_KCS:
    new_smi.handlers = &kcs_smi_handlers;
    break;
    case SI_SMIC:
    new_smi.handlers = &smic_smi_handlers;
    break;
    case SI_BT:
    new_smi.handlers = &bt_smi_handlers;
    break;
    default:
// No support for anything else yet.
    rv = -EIO;
    goto out_err;
    }
    new_smi.si_num = smi_num;
// Do this early so it's available for logs.
    if (!new_smi.io.dev) {
    pr_err("IPMI interface added with no device\n");
    rv = -EIO;
    goto out_err;
    }
// Allocate the state machine's data and initialize it.
    new_smi.si_sm = kmalloc(new_smi.handlers.size(), GFP_KERNEL);
    if (!new_smi.si_sm) {
    rv = -ENOMEM;
    goto out_err;
    }
    new_smi.io.io_size = new_smi.handlers.init_data(new_smi.si_sm,
    &new_smi.io);
// Now that we know the I/O size, we can set up the I/O.
    rv = new_smi.io.io_setup(&new_smi.io);
    if (rv) {
    dev_err(new_smi.io.dev, "Could not set up I/O space\n");
    goto out_err;
    }
// Do low-level detection first.
    if (new_smi.handlers.detect(new_smi.si_sm)) {
    if (new_smi.io.addr_source)
    dev_err(new_smi.io.dev,
    "Interface detection failed\n");
    rv = -ENODEV;
    goto out_err;
    }
//
// Attempt a get device id command.  If it fails, we probably
// don't have a BMC here.
//
    rv = try_get_dev_id(new_smi);
    if (rv) {
    if (new_smi.io.addr_source)
    dev_err(new_smi.io.dev,
    "There appears to be no BMC at this location\n");
    goto out_err;
    }
    setup_oem_data_handler(new_smi);
    setup_xaction_handlers(new_smi);
    check_for_broken_irqs(new_smi);
    new_smi.waiting_msg = core::ptr::null_mut();
    new_smi.curr_msg = core::ptr::null_mut();
    atomic_set(&new_smi.req_events, 0);
    new_smi.run_to_completion = false;
    for (i = 0; i < SI_NUM_STATS; i++)
    atomic_set(&new_smi.stats[i], 0);
    new_smi.interrupt_disabled = true;
    atomic_set(&new_smi.need_watch, 0);
    rv = try_enable_event_buffer(new_smi);
    if (rv == 0)
    new_smi.has_event_buffer = true;
//
// Start clearing the flags before we enable interrupts or the
// timer to avoid racing with the timer.
//
    start_clear_flags(new_smi);
//
// IRQ is defined to be set when non-zero.  req_events will
// cause a global flags check that will enable interrupts.
//
    if (new_smi.io.irq) {
    new_smi.interrupt_disabled = false;
    atomic_set(&new_smi.req_events, 1);
    }
    dev_set_drvdata(new_smi.io.dev, new_smi);
    rv = device_add_group(new_smi.io.dev, &ipmi_si_dev_attr_group);
    if (rv) {
    dev_err(new_smi.io.dev,
    "Unable to add device attributes: error %d\n",
    rv);
    goto out_err;
    }
    new_smi.dev_group_added = true;
    rv = ipmi_register_smi(&handlers,
    new_smi,
    new_smi.io.dev,
    new_smi.io.slave_addr);
    if (rv) {
    dev_err(new_smi.io.dev,
    "Unable to register device: error %d\n",
    rv);
    goto out_err;
    }
// Don't increment till we know we have succeeded.
    smi_num++;
    dev_info(new_smi.io.dev, "IPMI %s interface initialized\n",
    si_to_str[new_smi.io.si_info.type]);
    WARN_ON(new_smi.io.dev.init_name != core::ptr::null_mut());
    out_err:
    if (rv && new_smi.io.io_cleanup) {
    new_smi.io.io_cleanup(&new_smi.io);
    new_smi.io.io_cleanup = core::ptr::null_mut();
    }
    if (rv && new_smi.si_sm) {
    kfree(new_smi.si_sm);
    new_smi.si_sm = core::ptr::null_mut();
    }
    return rv;
    }
//
// Devices in the same address space at the same address are the same.
//
#[no_mangle]
unsafe extern "C" fn ipmi_smi_info_same(e1: *mut smi_info, e2: *mut smi_info) -> bool __init {
    static bool __init ipmi_smi_info_same(struct smi_info *e1, struct smi_info *e2)
    {
    return (e1.io.addr_space == e2.io.addr_space &&
    e1.io.addr_data == e2.io.addr_data);
    }
#[no_mangle]
unsafe extern "C" fn smi_init_work_fn(work: *mut work_struct) {
    static void smi_init_work_fn(struct work_struct *work)
    {
    struct smi_info *smi = container_of(work, struct smi_info, init_work);
    mutex_lock(&smi_infos_lock);
    try_smi_init(smi);
    mutex_unlock(&smi_infos_lock);
    }
#[no_mangle]
unsafe extern "C" fn init_ipmi_si() -> int __init {
    static int __init init_ipmi_si(void)
    {
    struct smi_info *e, *e2;
    if (initialized)
    return 0;
    ipmi_hardcode_init();
    pr_info("IPMI System Interface driver\n");
    ipmi_si_platform_init();
    ipmi_si_pci_init();
    ipmi_si_ls2k_init();
    ipmi_si_parisc_init();
    mutex_lock(&smi_infos_lock);
//
// Scan through all the devices.  We prefer devices with
// interrupts, so go through those first in case there are any
// duplicates that don't have the interrupt set.
//
    list_for_each_entry(e, &smi_infos, link) {
    let mut dup: bool = false;
// Register ones with interrupts first.
    if (!e.io.irq)
    continue;
//
// Go through the ones we have already seen to see if this
// is a dup.
//
    list_for_each_entry(e2, &smi_infos, link) {
    if (e2 == e)
    break;
    if (e2.io.irq && ipmi_smi_info_same(e, e2)) {
    dup = true;
    break;
    }
    }
    if (!dup) {
    if (IS_ENABLED(CONFIG_IPMI_SI_ASYNC_INIT))
    queue_work(system_unbound_wq, &e.init_work);
    else
    try_smi_init(e);
    }
    }
//
// Now try devices without interrupts.
//
    list_for_each_entry(e, &smi_infos, link) {
    let mut dup: bool = false;
    if (e.io.irq)
    continue;
//
// Go through the ones we have already seen to see if
// this is a dup.  We have already looked at the ones
// with interrupts.
//
    list_for_each_entry(e2, &smi_infos, link) {
    if (!e2.io.irq)
    continue;
    if (ipmi_smi_info_same(e, e2)) {
    dup = true;
    break;
    }
    }
    list_for_each_entry(e2, &smi_infos, link) {
    if (e2 == e)
    break;
    if (ipmi_smi_info_same(e, e2)) {
    dup = true;
    break;
    }
    }
    if (!dup) {
    if (IS_ENABLED(CONFIG_IPMI_SI_ASYNC_INIT))
    queue_work(system_unbound_wq, &e.init_work);
    else
    try_smi_init(e);
    }
    }
    initialized = true;
    mutex_unlock(&smi_infos_lock);
    mutex_lock(&smi_infos_lock);
    if (unload_when_empty && list_empty(&smi_infos)) {
    mutex_unlock(&smi_infos_lock);
    cleanup_ipmi_si();
    pr_warn("Unable to find any System Interface(s)\n");
    return -ENODEV;
    } else {
    mutex_unlock(&smi_infos_lock);
    return 0;
    }
    }
    module_init(init_ipmi_si);
#[no_mangle]
unsafe extern "C" fn wait_msg_processed(smi_info: *mut smi_info) {
    static void wait_msg_processed(struct smi_info *smi_info)
    {
    unsigned long jiffies_now;
    long time_diff;
    while (smi_info.si_state != SI_HOSED &&
    (smi_info.curr_msg || (smi_info.si_state != SI_NORMAL))) {
    jiffies_now = jiffies;
    time_diff = (((long)jiffies_now - (long)smi_info.last_timeout_jiffies)
// SI_USEC_PER_JIFFY);
    smi_event_handler(smi_info, time_diff);
    schedule_timeout_uninterruptible(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn shutdown_smi(send_info: *mut c_void) {
    static void shutdown_smi(void *send_info)
    {
    struct smi_info *smi_info = send_info;
    if (smi_info.dev_group_added) {
    device_remove_group(smi_info.io.dev, &ipmi_si_dev_attr_group);
    smi_info.dev_group_added = false;
    }
    if (smi_info.io.dev)
    dev_set_drvdata(smi_info.io.dev, core::ptr::null_mut());
//
// Make sure that interrupts, the timer and the thread are
// stopped and will not run again.
//
    smi_info.interrupt_disabled = true;
    if (smi_info.io.irq_cleanup) {
    smi_info.io.irq_cleanup(&smi_info.io);
    smi_info.io.irq_cleanup = core::ptr::null_mut();
    }
    stop_timer_and_thread(smi_info);
//
// Wait until we know that we are out of any interrupt
// handlers might have been running before we freed the
// interrupt.
//
    synchronize_rcu();
//
// Timeouts are stopped, now make sure the interrupts are off
// in the BMC.  Note that timers and CPU interrupts are off,
// so no need for locks.
//
    wait_msg_processed(smi_info);
    if (smi_info.handlers)
    disable_si_irq(smi_info);
    wait_msg_processed(smi_info);
    if (smi_info.handlers)
    smi_info.handlers.cleanup(smi_info.si_sm);
    if (smi_info.io.io_cleanup) {
    smi_info.io.io_cleanup(&smi_info.io);
    smi_info.io.io_cleanup = core::ptr::null_mut();
    }
    kfree(smi_info.si_sm);
    smi_info.si_sm = core::ptr::null_mut();
    smi_info.intf = core::ptr::null_mut();
    }
//
// Must be called with smi_info unlinked from smi_infos and smi_infos_lock released.
//
#[no_mangle]
unsafe extern "C" fn cleanup_one_si(smi_info: *mut smi_info) {
    static void cleanup_one_si(struct smi_info *smi_info)
    {
    if (!smi_info)
    return;
    if (IS_ENABLED(CONFIG_IPMI_SI_ASYNC_INIT))
    cancel_work_sync(&smi_info.init_work);
    ipmi_unregister_smi(smi_info.intf);
    kfree(smi_info);
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_remove_by_dev(dev: *mut device) {
    void ipmi_si_remove_by_dev(struct device *dev)
    {
    struct smi_info *e = core::ptr::null_mut(), *tmp;
    mutex_lock(&smi_infos_lock);
    list_for_each_entry(tmp, &smi_infos, link) {
    if (tmp.io.dev == dev) {
    e = tmp;
    list_del(&e.link);
    break;
    }
    }
    mutex_unlock(&smi_infos_lock);
    if (e)
    cleanup_one_si(e);
    }
    struct device *ipmi_si_remove_by_data(int addr_space, enum si_type si_type,
    unsigned long addr)
    {
// remove
    struct smi_info *e, *tmp_e;
    struct device *dev = core::ptr::null_mut();
    LIST_HEAD(to_clean);
    mutex_lock(&smi_infos_lock);
    list_for_each_entry_safe(e, tmp_e, &smi_infos, link) {
    if (e.io.addr_space != addr_space)
    continue;
    if (e.io.si_info.type != si_type)
    continue;
    if (e.io.addr_data == addr) {
    dev = get_device(e.io.dev);
    list_move_tail(&e.link, &to_clean);
    }
    }
    mutex_unlock(&smi_infos_lock);
    list_for_each_entry_safe(e, tmp_e, &to_clean, link) {
    list_del(&e.link);
    cleanup_one_si(e);
    }
    return dev;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_ipmi_si() {
    static void cleanup_ipmi_si(void)
    {
    struct smi_info *e, *tmp_e;
    LIST_HEAD(to_clean);
    if (!initialized)
    return;
    ipmi_si_pci_shutdown();
    ipmi_si_ls2k_shutdown();
    ipmi_si_parisc_shutdown();
    ipmi_si_platform_shutdown();
    mutex_lock(&smi_infos_lock);
    list_splice_init(&smi_infos, &to_clean);
    mutex_unlock(&smi_infos_lock);
    list_for_each_entry_safe(e, tmp_e, &to_clean, link) {
    list_del(&e.link);
    cleanup_one_si(e);
    }
    ipmi_si_hardcode_exit();
    ipmi_si_hotmod_exit();
    }
    module_exit(cleanup_ipmi_si);
    MODULE_ALIAS("platform:dmi-ipmi-si");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Corey Minyard <minyard@mvista.com>");
    MODULE_DESCRIPTION("Interface to the IPMI driver for the KCS, SMIC, and BT system interfaces.");
