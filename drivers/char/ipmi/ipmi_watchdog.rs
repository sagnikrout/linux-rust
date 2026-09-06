//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_watchdog.c
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
// ipmi_watchdog.c
//
// A watchdog timer based upon the IPMI interface.
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//

//
// This is ugly, but I've determined that x86 is the only architecture
// that can reasonably support the IPMI NMI watchdog timeout at this
// time.  If another architecture adds this capability somehow, it
// will have to be a somewhat different mechanism and I have no idea
// how it will work.  So in the unlikely event that another
// architecture supports this, we can figure out a good generic
// mechanism for it at that time.
//

// Macro flag: #define HAVE_DIE_NMI

//
// The IPMI command/response information for the watchdog timer.
//
// values for byte 1 of the set command, byte 2 of the get response.

    byte = ((byte) & 0xf8) | ((use) & 0x7)

pub const WDOG_TIMER_USE_BIOS_FRB2: c_int = 1;
pub const WDOG_TIMER_USE_BIOS_POST: c_int = 2;
pub const WDOG_TIMER_USE_OS_LOAD: c_int = 3;
pub const WDOG_TIMER_USE_SMS_OS: c_int = 4;
pub const WDOG_TIMER_USE_OEM: c_int = 5;
// values for byte 2 of the set command, byte 3 of the get response.

    byte = ((byte) & 0x8f) | (((use) & 0x7) << 4)

pub const WDOG_PRETIMEOUT_NONE: c_int = 0;
pub const WDOG_PRETIMEOUT_SMI: c_int = 1;
pub const WDOG_PRETIMEOUT_NMI: c_int = 2;
pub const WDOG_PRETIMEOUT_MSG_INT: c_int = 3;
// Operations that can be performed on a pretimout.
pub const WDOG_PREOP_NONE: c_int = 0;
pub const WDOG_PREOP_PANIC: c_int = 1;
// Cause data to be available to read.  Doesn't work in NMI mode.
pub const WDOG_PREOP_GIVE_DATA: c_int = 2;
// Actions to perform on a full timeout.

    byte = ((byte) & 0xf8) | ((use) & 0x7)

pub const WDOG_TIMEOUT_NONE: c_int = 0;
pub const WDOG_TIMEOUT_RESET: c_int = 1;
pub const WDOG_TIMEOUT_POWER_DOWN: c_int = 2;
pub const WDOG_TIMEOUT_POWER_CYCLE: c_int = 3;
//
// Byte 3 of the get command, byte 4 of the get response is the
// pre-timeout in seconds.
//
// Bits for setting byte 4 of the set command, byte 5 of the get response.

//
// Setting/getting the watchdog timer value.  This is for bytes 5 and
// 6 (the timeout time) of the set command, and bytes 6 and 7 (the
// timeout time) and 8 and 9 (the current countdown value) of the
// response.  The timeout value is given in seconds (in the command it
// is 100ms intervals).
//

    (byte1) = (((val) * 10) & 0xff), (byte2) = (((val) * 10) >> 8)

    (((byte1) | ((byte2) << 8)) / 10)
pub const IPMI_WDOG_RESET_TIMER: c_uint = 0x22;
pub const IPMI_WDOG_SET_TIMER: c_uint = 0x24;
pub const IPMI_WDOG_GET_TIMER: c_uint = 0x25;
pub const IPMI_WDOG_TIMER_NOT_INIT_RESP: c_uint = 0x80;
    static DEFINE_MUTEX(ipmi_watchdog_mutex);
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    static struct ipmi_user *watchdog_user;
    static int watchdog_ifnum;
// Default the timeout to 10 seconds.
    let mut timeout: static int = 10;
// The pre-timeout is disabled by default.
    static int pretimeout;
// Default timeout to set on panic
    let mut panic_wdt_timeout: static int = 255;
// Default action is to reset the board on a timeout.
    let mut action_val: static unsigned char = WDOG_TIMEOUT_RESET;
    static char action[16] = "reset";
    let mut preaction_val: static unsigned char = WDOG_PRETIMEOUT_NONE;
    static char preaction[16] = "pre_none";
    let mut preop_val: static unsigned char = WDOG_PREOP_NONE;
    static char preop[16] = "preop_none";
    static DEFINE_MUTEX(ipmi_read_mutex);
    static char data_to_read;
    static DECLARE_WAIT_QUEUE_HEAD(read_q);
    static struct fasync_struct *fasync_q;
    static atomic_t pretimeout_since_last_heartbeat;
    static char expect_close;
    let mut ifnum_to_use: static int = -1;
// Parameters to ipmi_set_timeout
pub const IPMI_SET_TIMEOUT_NO_HB: c_int = 0;
pub const IPMI_SET_TIMEOUT_HB_IF_NECESSARY: c_int = 1;
pub const IPMI_SET_TIMEOUT_FORCE_HB: c_int = 2;
    static int ipmi_set_timeout(int do_heartbeat);
    static void ipmi_register_watchdog(int ipmi_intf);
    static void ipmi_unregister_watchdog(int ipmi_intf);
//
// If true, the driver will start running as soon as it is configured
// and ready.
//
    static int start_now;
#[no_mangle]
unsafe extern "C" fn set_param_timeout(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int set_param_timeout(const char *val, const struct kernel_param *kp)
    {
    char *endp;
    int  l;
    let mut rv: c_int = 0;
    if (!val)
    return -EINVAL;
    l = simple_strtoul(val, &endp, 0);
    if (endp == val)
    return -EINVAL;
// ((int *)kp->arg) = l;
    if (watchdog_user)
    rv = ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY);
    return rv;
    }
    static const struct kernel_param_ops param_ops_timeout = {
    .set = set_param_timeout,
    .get = param_get_int,
    };

    typedef int (*action_fn)(const char *intval, char *outval);
    static int action_op(const char *inval, char *outval);
    static int preaction_op(const char *inval, char *outval);
    static int preop_op(const char *inval, char *outval);
    static void check_parms(void);
#[no_mangle]
unsafe extern "C" fn set_param_str(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int set_param_str(const char *val, const struct kernel_param *kp)
    {
    let mut fn: action_fn = (action_fn) kp.arg;
    let mut rv: c_int = 0;
    char       valcp[16];
    char       *s;
    strscpy(valcp, val, 16);
    s = strstrip(valcp);
    rv = fn(s, core::ptr::null_mut());
    if (rv)
    goto out;
    check_parms();
    if (watchdog_user)
    rv = ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY);
    out:
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn get_param_str(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    static int get_param_str(char *buffer, const struct kernel_param *kp)
    {
    let mut fn: action_fn = (action_fn) kp.arg;
    int rv, len;
    rv = fn(core::ptr::null_mut(), buffer);
    if (rv)
    return rv;
    len = strlen(buffer);
    buffer[len++] = '\n';
    buffer[len] = 0;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn set_param_wdog_ifnum(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int set_param_wdog_ifnum(const char *val, const struct kernel_param *kp)
    {
    let mut rv: c_int = param_set_int(val, kp);
    if (rv)
    return rv;
    if ((ifnum_to_use < 0) || (ifnum_to_use == watchdog_ifnum))
    return 0;
    ipmi_unregister_watchdog(watchdog_ifnum);
    ipmi_register_watchdog(ifnum_to_use);
    return 0;
    }
    static const struct kernel_param_ops param_ops_wdog_ifnum = {
    .set = set_param_wdog_ifnum,
    .get = param_get_int,
    };

    static const struct kernel_param_ops param_ops_str = {
    .set = set_param_str,
    .get = get_param_str,
    };
    module_param(ifnum_to_use, wdog_ifnum, 0644);
    MODULE_PARM_DESC(ifnum_to_use, "The interface number to use for the watchdog "
    "timer.  Setting to -1 defaults to the first registered "
    "interface");
    module_param(timeout, timeout, 0644);
    MODULE_PARM_DESC(timeout, "Timeout value in seconds.");
    module_param(pretimeout, timeout, 0644);
    MODULE_PARM_DESC(pretimeout, "Pretimeout value in seconds.");
    module_param(panic_wdt_timeout, timeout, 0644);
    MODULE_PARM_DESC(panic_wdt_timeout, "Timeout value on kernel panic in seconds.");
    module_param_cb(action, &param_ops_str, action_op, 0644);
    MODULE_PARM_DESC(action, "Timeout action. One of: "
    "reset, none, power_cycle, power_off.");
    module_param_cb(preaction, &param_ops_str, preaction_op, 0644);
    MODULE_PARM_DESC(preaction, "Pretimeout action.  One of: "
    "pre_none, pre_smi, pre_nmi, pre_int.");
    module_param_cb(preop, &param_ops_str, preop_op, 0644);
    MODULE_PARM_DESC(preop, "Pretimeout driver operation.  One of: "
    "preop_none, preop_panic, preop_give_data.");
    module_param(start_now, int, 0444);
    MODULE_PARM_DESC(start_now, "Set to 1 to start the watchdog as"
    "soon as the driver is loaded.");
    module_param(nowayout, bool, 0644);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started "
    "(default=CONFIG_WATCHDOG_NOWAYOUT)");
// Default state of the timer.
    let mut ipmi_watchdog_state: static unsigned char = WDOG_TIMEOUT_NONE;
// Is someone using the watchdog?  Only one user is allowed.
    static unsigned long ipmi_wdog_open;
//
// If set to 1, the heartbeat command will set the state to reset and
// start the timer.  The timer doesn't normally run when the driver is
// first opened until the heartbeat is set the first time, this
// variable is used to accomplish this.
//
    static int ipmi_start_timer_on_heartbeat;
// IPMI version of the BMC.
    static unsigned char ipmi_version_major;
    static unsigned char ipmi_version_minor;
// If a pretimeout occurs, this is used to allow only one panic to happen.
    let mut preop_panic_excl: static atomic_t = ATOMIC_INIT(-1);

    static int testing_nmi;
    static int nmi_handler_registered;

    static int __ipmi_heartbeat(void);
//
// We use a mutex to make sure that only one thing can send a set a
// message at one time.  The mutex is claimed when a message is sent
// and freed when both the send and receive messages are free.
//
    let mut msg_tofree: static atomic_t = ATOMIC_INIT(0);
    static DECLARE_COMPLETION(msg_wait);
#[no_mangle]
unsafe extern "C" fn msg_free_smi(msg: *mut ipmi_smi_msg) {
    static void msg_free_smi(struct ipmi_smi_msg *msg)
    {
    if (atomic_dec_and_test(&msg_tofree)) {
    if (!oops_in_progress)
    complete(&msg_wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn msg_free_recv(msg: *mut ipmi_recv_msg) {
    static void msg_free_recv(struct ipmi_recv_msg *msg)
    {
    if (atomic_dec_and_test(&msg_tofree)) {
    if (!oops_in_progress)
    complete(&msg_wait);
    }
    }
    let mut smi_msg: static struct ipmi_smi_msg = INIT_IPMI_SMI_MSG(msg_free_smi);
    let mut recv_msg: static struct ipmi_recv_msg = INIT_IPMI_RECV_MSG(msg_free_recv);
    static int __ipmi_set_timeout(struct ipmi_smi_msg  *smi_msg,
    struct ipmi_recv_msg *recv_msg,
    int                  *send_heartbeat_now)
    {
    struct kernel_ipmi_msg            msg;
    unsigned char                     data[6];
    let mut rv: c_int = 0;
    struct ipmi_system_interface_addr addr;
    let mut hbnow: c_int = 0;
    data[0] = 0;
    WDOG_SET_TIMER_USE(data[0], WDOG_TIMER_USE_SMS_OS);
    if (ipmi_watchdog_state != WDOG_TIMEOUT_NONE) {
    if ((ipmi_version_major > 1) ||
    ((ipmi_version_major == 1) && (ipmi_version_minor >= 5))) {
// This is an IPMI 1.5-only feature.
    data[0] |= WDOG_DONT_STOP_ON_SET;
    } else {
//
// In ipmi 1.0, setting the timer stops the watchdog, we
// need to start it back up again.
//
    hbnow = 1;
    }
    }
    data[1] = 0;
    WDOG_SET_TIMEOUT_ACT(data[1], ipmi_watchdog_state);
    if ((pretimeout > 0) && (ipmi_watchdog_state != WDOG_TIMEOUT_NONE)) {
    WDOG_SET_PRETIMEOUT_ACT(data[1], preaction_val);
    data[2] = pretimeout;
    } else {
    WDOG_SET_PRETIMEOUT_ACT(data[1], WDOG_PRETIMEOUT_NONE);
    data[2] = 0; /* No pretimeout. */
    }
    data[3] = 0;
    WDOG_SET_TIMEOUT(data[4], data[5], timeout);
    addr.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
    addr.channel = IPMI_BMC_CHANNEL;
    addr.lun = 0;
    msg.netfn = 0x06;
    msg.cmd = IPMI_WDOG_SET_TIMER;
    msg.data = data;
    msg.data_len = sizeof(data);
    if (smi_msg)
    rv = ipmi_request_supply_msgs(watchdog_user,
    (struct ipmi_addr *) &addr,
    0,
    &msg,
    core::ptr::null_mut(),
    smi_msg,
    recv_msg,
    1);
    else
    ipmi_panic_request_and_wait(watchdog_user,
    (struct ipmi_addr *) &addr, &msg);
    if (rv)
    pr_warn("set timeout error: %d\n", rv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: send_heartbeat_now) -> else {
    else if (send_heartbeat_now)
// send_heartbeat_now = hbnow;
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn _ipmi_set_timeout(do_heartbeat: c_int) -> c_int {
    static int _ipmi_set_timeout(int do_heartbeat)
    {
    int send_heartbeat_now;
    int rv;
    if (!watchdog_user)
    return -ENODEV;
    atomic_set(&msg_tofree, 2);
    rv = __ipmi_set_timeout(&smi_msg, &recv_msg, &send_heartbeat_now);
    if (rv) {
    atomic_set(&msg_tofree, 0);
    return rv;
    }
    wait_for_completion(&msg_wait);
    if ((do_heartbeat == IPMI_SET_TIMEOUT_FORCE_HB)
    || ((send_heartbeat_now)
    && (do_heartbeat == IPMI_SET_TIMEOUT_HB_IF_NECESSARY)))
    rv = __ipmi_heartbeat();
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_set_timeout(do_heartbeat: c_int) -> c_int {
    static int ipmi_set_timeout(int do_heartbeat)
    {
    int rv;
    mutex_lock(&ipmi_watchdog_mutex);
    rv = _ipmi_set_timeout(do_heartbeat);
    mutex_unlock(&ipmi_watchdog_mutex);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn panic_halt_ipmi_heartbeat() {
    static void panic_halt_ipmi_heartbeat(void)
    {
    struct kernel_ipmi_msg             msg;
    struct ipmi_system_interface_addr addr;
//
// Don't reset the timer if we have the timer turned off, that
// re-enables the watchdog.
//
    if (ipmi_watchdog_state == WDOG_TIMEOUT_NONE)
    return;
    addr.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
    addr.channel = IPMI_BMC_CHANNEL;
    addr.lun = 0;
    msg.netfn = 0x06;
    msg.cmd = IPMI_WDOG_RESET_TIMER;
    msg.data = core::ptr::null_mut();
    msg.data_len = 0;
    ipmi_panic_request_and_wait(watchdog_user, (struct ipmi_addr *) &addr,
    &msg);
    }
//
// Special call, doesn't claim any locks.  This is only to be called
// at panic or halt time, in run-to-completion mode, when the caller
// is the only CPU and the only thing that will be going is these IPMI
// calls.
//
#[no_mangle]
unsafe extern "C" fn panic_halt_ipmi_set_timeout() {
    static void panic_halt_ipmi_set_timeout(void)
    {
    int send_heartbeat_now;
    int rv;
    rv = __ipmi_set_timeout(core::ptr::null_mut(), core::ptr::null_mut(), &send_heartbeat_now);
    if (rv) {
    pr_warn("Unable to extend the watchdog timeout\n");
    } else {
    if (send_heartbeat_now)
    panic_halt_ipmi_heartbeat();
    }
    }
#[no_mangle]
unsafe extern "C" fn __ipmi_heartbeat() -> c_int {
    static int __ipmi_heartbeat(void)
    {
    struct kernel_ipmi_msg msg;
    int rv;
    struct ipmi_system_interface_addr addr;
    let mut timeout_retries: c_int = 0;
    restart:
//
// Don't reset the timer if we have the timer turned off, that
// re-enables the watchdog.
//
    if (ipmi_watchdog_state == WDOG_TIMEOUT_NONE)
    return 0;
    atomic_set(&msg_tofree, 2);
    addr.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
    addr.channel = IPMI_BMC_CHANNEL;
    addr.lun = 0;
    msg.netfn = 0x06;
    msg.cmd = IPMI_WDOG_RESET_TIMER;
    msg.data = core::ptr::null_mut();
    msg.data_len = 0;
    rv = ipmi_request_supply_msgs(watchdog_user,
    (struct ipmi_addr *) &addr,
    0,
    &msg,
    core::ptr::null_mut(),
    &smi_msg,
    &recv_msg,
    1);
    if (rv) {
    atomic_set(&msg_tofree, 0);
    pr_warn("heartbeat send failure: %d\n", rv);
    return rv;
    }
// Wait for the heartbeat to be sent.
    wait_for_completion(&msg_wait);
    if (recv_msg.msg.data[0] == IPMI_WDOG_TIMER_NOT_INIT_RESP)  {
    timeout_retries++;
    if (timeout_retries > 3) {
    pr_err("Unable to restore the IPMI watchdog's settings, giving up\n");
    rv = -EIO;
    goto out;
    }
//
// The timer was not initialized, that means the BMC was
// probably reset and lost the watchdog information.  Attempt
// to restore the timer's info.  Note that we still hold
// the heartbeat lock, to keep a heartbeat from happening
// in this process, so must say no heartbeat to avoid a
// deadlock on this mutex
//
    rv = _ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    if (rv) {
    pr_err("Unable to send the command to set the watchdog's settings, giving up\n");
    goto out;
    }
// Might need a heartbeat send, go ahead and do it.
    goto restart;
    } else if (recv_msg.msg.data[0] != 0) {
//
// Got an error in the heartbeat response.  It was already
// reported in ipmi_wdog_msg_handler, but we should return
// an error here.
//
    rv = -EINVAL;
    }
    out:
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn _ipmi_heartbeat() -> c_int {
    static int _ipmi_heartbeat(void)
    {
    int rv;
    if (!watchdog_user)
    return -ENODEV;
    if (ipmi_start_timer_on_heartbeat) {
    ipmi_start_timer_on_heartbeat = 0;
    ipmi_watchdog_state = action_val;
    rv = _ipmi_set_timeout(IPMI_SET_TIMEOUT_FORCE_HB);
    } else if (atomic_cmpxchg(&pretimeout_since_last_heartbeat, 1, 0)) {
//
// A pretimeout occurred, make sure we set the timeout.
// We don't want to set the action, though, we want to
// leave that alone (thus it can't be combined with the
// above operation.
//
    rv = _ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY);
    } else {
    rv = __ipmi_heartbeat();
    }
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_heartbeat() -> c_int {
    static int ipmi_heartbeat(void)
    {
    int rv;
    mutex_lock(&ipmi_watchdog_mutex);
    rv = _ipmi_heartbeat();
    mutex_unlock(&ipmi_watchdog_mutex);
    return rv;
    }
    static const struct watchdog_info ident = {
    .options	= 0,	/* WDIOF_SETTIMEOUT, */
    .firmware_version = 1,
    .identity	= "IPMI"
    };
    static int ipmi_ioctl(struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    int i;
    int val;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    i = copy_to_user(argp, &ident, sizeof(ident));
    return i ? -EFAULT : 0;
    case WDIOC_SETTIMEOUT:
    i = copy_from_user(&val, argp, sizeof(int));
    if (i)
    return -EFAULT;
    timeout = val;
    return _ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY);
    case WDIOC_GETTIMEOUT:
    i = copy_to_user(argp, &timeout, sizeof(timeout));
    if (i)
    return -EFAULT;
    return 0;
    case WDIOC_SETPRETIMEOUT:
    i = copy_from_user(&val, argp, sizeof(int));
    if (i)
    return -EFAULT;
    pretimeout = val;
    return _ipmi_set_timeout(IPMI_SET_TIMEOUT_HB_IF_NECESSARY);
    case WDIOC_GETPRETIMEOUT:
    i = copy_to_user(argp, &pretimeout, sizeof(pretimeout));
    if (i)
    return -EFAULT;
    return 0;
    case WDIOC_KEEPALIVE:
    return _ipmi_heartbeat();
    case WDIOC_SETOPTIONS:
    i = copy_from_user(&val, argp, sizeof(int));
    if (i)
    return -EFAULT;
    if (val & WDIOS_DISABLECARD) {
    ipmi_watchdog_state = WDOG_TIMEOUT_NONE;
    _ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    ipmi_start_timer_on_heartbeat = 0;
    }
    if (val & WDIOS_ENABLECARD) {
    ipmi_watchdog_state = action_val;
    _ipmi_set_timeout(IPMI_SET_TIMEOUT_FORCE_HB);
    }
    return 0;
    case WDIOC_GETSTATUS:
    val = 0;
    i = copy_to_user(argp, &val, sizeof(val));
    if (i)
    return -EFAULT;
    return 0;
    default:
    return -ENOIOCTLCMD;
    }
    }
    static long ipmi_unlocked_ioctl(struct file *file,
    unsigned int cmd,
    unsigned long arg)
    {
    int ret;
    mutex_lock(&ipmi_watchdog_mutex);
    ret = ipmi_ioctl(file, cmd, arg);
    mutex_unlock(&ipmi_watchdog_mutex);
    return ret;
    }
    static ssize_t ipmi_write(struct file *file,
    const char  __user *buf,
    size_t      len,
    loff_t      *ppos)
    {
    int rv;
    if (len) {
    if (!nowayout) {
    size_t i;
// In case it was set long ago
    expect_close = 0;
    for (i = 0; i != len; i++) {
    char c;
    if (get_user(c, buf + i))
    return -EFAULT;
    if (c == 'V')
    expect_close = 42;
    }
    }
    rv = ipmi_heartbeat();
    if (rv)
    return rv;
    }
    return len;
    }
    static ssize_t ipmi_read(struct file *file,
    char        __user *buf,
    size_t      count,
    loff_t      *ppos)
    {
    let mut rv: c_int = 0;
    wait_queue_entry_t wait;
    if (count <= 0)
    return 0;
//
// Reading returns if the pretimeout has gone off, and it only does
// it once per pretimeout.
//
    mutex_lock(&ipmi_read_mutex);
    if (!data_to_read) {
    if (file.f_flags & O_NONBLOCK) {
    rv = -EAGAIN;
    goto out;
    }
    init_waitqueue_entry(&wait, current);
    add_wait_queue(&read_q, &wait);
    while (!data_to_read && !signal_pending(current)) {
    set_current_state(TASK_INTERRUPTIBLE);
    mutex_unlock(&ipmi_read_mutex);
    schedule();
    mutex_lock(&ipmi_read_mutex);
    }
    remove_wait_queue(&read_q, &wait);
    if (signal_pending(current)) {
    rv = -ERESTARTSYS;
    goto out;
    }
    }
    data_to_read = 0;
    out:
    mutex_unlock(&ipmi_read_mutex);
    if (rv == 0) {
    if (copy_to_user(buf, &data_to_read, 1))
    rv = -EFAULT;
    else
    rv = 1;
    }
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_open(ino: *mut inode, filep: *mut file) -> c_int {
    static int ipmi_open(struct inode *ino, struct file *filep)
    {
    switch (iminor(ino)) {
    case WATCHDOG_MINOR:
    if (test_and_set_bit(0, &ipmi_wdog_open))
    return -EBUSY;
//
// Don't start the timer now, let it start on the
// first heartbeat.
//
    ipmi_start_timer_on_heartbeat = 1;
    return stream_open(ino, filep);
    default:
    return (-ENODEV);
    }
    }
#[no_mangle]
unsafe extern "C" fn ipmi_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t ipmi_poll(struct file *file, poll_table *wait)
    {
    let mut mask: __poll_t = 0;
    poll_wait(file, &read_q, wait);
    mutex_lock(&ipmi_read_mutex);
    if (data_to_read)
    mask |= (EPOLLIN | EPOLLRDNORM);
    mutex_unlock(&ipmi_read_mutex);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_fasync(fd: c_int, file: *mut file, on: c_int) -> c_int {
    static int ipmi_fasync(int fd, struct file *file, int on)
    {
    int result;
    result = fasync_helper(fd, file, on, &fasync_q);
    return (result);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_close(ino: *mut inode, filep: *mut file) -> c_int {
    static int ipmi_close(struct inode *ino, struct file *filep)
    {
    if (iminor(ino) == WATCHDOG_MINOR) {
    if (expect_close == 42) {
    mutex_lock(&ipmi_watchdog_mutex);
    ipmi_watchdog_state = WDOG_TIMEOUT_NONE;
    _ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    mutex_unlock(&ipmi_watchdog_mutex);
    } else {
    pr_crit("Unexpected close, not stopping watchdog!\n");
    ipmi_heartbeat();
    }
    clear_bit(0, &ipmi_wdog_open);
    }
    expect_close = 0;
    return 0;
    }
    static const struct file_operations ipmi_wdog_fops = {
    .owner   = THIS_MODULE,
    .read    = ipmi_read,
    .poll    = ipmi_poll,
    .write   = ipmi_write,
    .unlocked_ioctl = ipmi_unlocked_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .open    = ipmi_open,
    .release = ipmi_close,
    .fasync  = ipmi_fasync,
    };
    static struct miscdevice ipmi_wdog_miscdev = {
    .minor		= WATCHDOG_MINOR,
    .name		= "watchdog",
    .fops		= &ipmi_wdog_fops
    };
    static void ipmi_wdog_msg_handler(struct ipmi_recv_msg *msg,
    void                 *handler_data)
    {
    if (msg.msg.cmd == IPMI_WDOG_RESET_TIMER &&
    msg.msg.data[0] == IPMI_WDOG_TIMER_NOT_INIT_RESP)
    pr_info("response: The IPMI controller appears to have been reset, will attempt to reinitialize the watchdog timer\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: msg->msg.data[0] !=) -> else {
    else if (msg.msg.data[0] != 0)
    pr_err("response: Error %x on cmd %x\n",
    msg.msg.data[0],
    msg.msg.cmd);
    ipmi_free_recv_msg(msg);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_wdog_pretimeout_handler(handler_data: *mut c_void) {
    static void ipmi_wdog_pretimeout_handler(void *handler_data)
    {
    if (preaction_val != WDOG_PRETIMEOUT_NONE) {
    if (preop_val == WDOG_PREOP_PANIC) {
    if (atomic_inc_and_test(&preop_panic_excl))
    panic("Watchdog pre-timeout");
    } else if (preop_val == WDOG_PREOP_GIVE_DATA) {
    mutex_lock(&ipmi_read_mutex);
    data_to_read = 1;
    wake_up_interruptible(&read_q);
    kill_fasync(&fasync_q, SIGIO, POLL_IN);
    mutex_unlock(&ipmi_read_mutex);
    }
    }
//
// On some machines, the heartbeat will give an error and not
// work unless we re-enable the timer.  So do so.
//
    atomic_set(&pretimeout_since_last_heartbeat, 1);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_wdog_panic_handler(user_data: *mut c_void) {
    static void ipmi_wdog_panic_handler(void *user_data)
    {
    static int panic_event_handled;
//
// On a panic, if we have a panic timeout, make sure to extend
// the watchdog timer to a reasonable value to complete the
// panic, if the watchdog timer is running.  Plus the
// pretimeout is meaningless at panic time.
//
    if (watchdog_user && !panic_event_handled &&
    ipmi_watchdog_state != WDOG_TIMEOUT_NONE) {
// Make sure we do this only once.
    panic_event_handled = 1;
    timeout = panic_wdt_timeout;
    pretimeout = 0;
    panic_halt_ipmi_set_timeout();
    }
    }
    static const struct ipmi_user_hndl ipmi_hndlrs = {
    .ipmi_recv_hndl           = ipmi_wdog_msg_handler,
    .ipmi_watchdog_pretimeout = ipmi_wdog_pretimeout_handler,
    .ipmi_panic_handler       = ipmi_wdog_panic_handler
    };
#[no_mangle]
unsafe extern "C" fn ipmi_register_watchdog(ipmi_intf: c_int) {
    static void ipmi_register_watchdog(int ipmi_intf)
    {
    let mut rv: c_int = -EBUSY;
    if (watchdog_user)
    goto out;
    if ((ifnum_to_use >= 0) && (ifnum_to_use != ipmi_intf))
    goto out;
    watchdog_ifnum = ipmi_intf;
    rv = ipmi_create_user(ipmi_intf, &ipmi_hndlrs, core::ptr::null_mut(), &watchdog_user);
    if (rv < 0) {
    pr_crit("Unable to register with ipmi\n");
    goto out;
    }
    rv = ipmi_get_version(watchdog_user,
    &ipmi_version_major,
    &ipmi_version_minor);
    if (rv) {
    pr_warn("Unable to get IPMI version, assuming 1.0\n");
    ipmi_version_major = 1;
    ipmi_version_minor = 0;
    }
    rv = misc_register(&ipmi_wdog_miscdev);
    if (rv < 0) {
    ipmi_destroy_user(watchdog_user);
    watchdog_user = core::ptr::null_mut();
    pr_crit("Unable to register misc device\n");
    }

    if (nmi_handler_registered) {
    let mut old_pretimeout: c_int = pretimeout;
    let mut old_timeout: c_int = timeout;
    let mut old_preop_val: c_int = preop_val;
//
// Set the pretimeout to go off in a second and give
// ourselves plenty of time to stop the timer.
//
    ipmi_watchdog_state = WDOG_TIMEOUT_RESET;
    preop_val = WDOG_PREOP_NONE; /* Make sure nothing happens */
    pretimeout = 99;
    timeout = 100;
    testing_nmi = 1;
    rv = ipmi_set_timeout(IPMI_SET_TIMEOUT_FORCE_HB);
    if (rv) {
    pr_warn("Error starting timer to test NMI: 0x%x.  The NMI pretimeout will likely not work\n",
    rv);
    rv = 0;
    goto out_restore;
    }
    msleep(1500);
    if (testing_nmi != 2) {
    pr_warn("IPMI NMI didn't seem to occur.  The NMI pretimeout will likely not work\n");
    }
    out_restore:
    testing_nmi = 0;
    preop_val = old_preop_val;
    pretimeout = old_pretimeout;
    timeout = old_timeout;
    }

    out:
    if ((start_now) && (rv == 0)) {
// Run from startup, so start the timer now.
    start_now = 0; /* Disable this function after first startup. */
    ipmi_watchdog_state = action_val;
    ipmi_set_timeout(IPMI_SET_TIMEOUT_FORCE_HB);
    pr_info("Starting now!\n");
    } else {
// Stop the timer now.
    ipmi_watchdog_state = WDOG_TIMEOUT_NONE;
    ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    }
    }
#[no_mangle]
unsafe extern "C" fn ipmi_unregister_watchdog(ipmi_intf: c_int) {
    static void ipmi_unregister_watchdog(int ipmi_intf)
    {
    struct ipmi_user *loc_user = watchdog_user;
    if (!loc_user)
    return;
    if (watchdog_ifnum != ipmi_intf)
    return;
// Make sure no one can call us any more.
    misc_deregister(&ipmi_wdog_miscdev);
    watchdog_user = core::ptr::null_mut();
//
// Wait to make sure the message makes it out.  The lower layer has
// pointers to our buffers, we want to make sure they are done before
// we release our memory.
//
    while (atomic_read(&msg_tofree))
    msg_free_smi(core::ptr::null_mut());
    mutex_lock(&ipmi_watchdog_mutex);
// Disconnect from IPMI.
    ipmi_destroy_user(loc_user);
// If it comes back, restart it properly.
    ipmi_start_timer_on_heartbeat = 1;
    mutex_unlock(&ipmi_watchdog_mutex);
    }

    static int
    ipmi_nmi(unsigned int val, struct pt_regs *regs)
    {
//
// If we get here, it's an NMI that's not a memory or I/O
// error.  We can't truly tell if it's from IPMI or not
// without sending a message, and sending a message is almost
// impossible because of locking.
//
    if (testing_nmi) {
    testing_nmi = 2;
    return NMI_HANDLED;
    }
// If we are not expecting a timeout, ignore it.
    if (ipmi_watchdog_state == WDOG_TIMEOUT_NONE)
    return NMI_DONE;
    if (preaction_val != WDOG_PRETIMEOUT_NMI)
    return NMI_DONE;
//
// If no one else handled the NMI, we assume it was the IPMI
// watchdog.
//
    if (preop_val == WDOG_PREOP_PANIC) {
// On some machines, the heartbeat will give
    an error and not work unless we re-enable
    the timer.   So do so. */
    atomic_set(&pretimeout_since_last_heartbeat, 1);
    if (atomic_inc_and_test(&preop_panic_excl))
    nmi_panic(regs, "pre-timeout");
    }
    return NMI_HANDLED;
    }

    static int wdog_reboot_handler(struct notifier_block *this,
    unsigned long         code,
    void                  *unused)
    {
    static int reboot_event_handled;
    if ((watchdog_user) && (!reboot_event_handled)) {
// Make sure we only do this once.
    reboot_event_handled = 1;
    if (code == SYS_POWER_OFF || code == SYS_HALT) {
// Disable the WDT if we are shutting down.
    ipmi_watchdog_state = WDOG_TIMEOUT_NONE;
    ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    } else if (ipmi_watchdog_state != WDOG_TIMEOUT_NONE) {
// Set a long timer to let the reboot happen or
    reset if it hangs, but only if the watchdog
    timer was already running. */
    if (timeout < 120)
    timeout = 120;
    pretimeout = 0;
    ipmi_watchdog_state = WDOG_TIMEOUT_RESET;
    ipmi_set_timeout(IPMI_SET_TIMEOUT_NO_HB);
    }
    }
    return NOTIFY_OK;
    }
    static struct notifier_block wdog_reboot_notifier = {
    .notifier_call	= wdog_reboot_handler,
    .next		= core::ptr::null_mut(),
    .priority	= 0
    };
#[no_mangle]
unsafe extern "C" fn ipmi_new_smi(if_num: c_int, device: *mut device) {
    static void ipmi_new_smi(int if_num, struct device *device)
    {
    ipmi_register_watchdog(if_num);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_smi_gone(if_num: c_int) {
    static void ipmi_smi_gone(int if_num)
    {
    ipmi_unregister_watchdog(if_num);
    }
    static struct ipmi_smi_watcher smi_watcher = {
    .owner    = THIS_MODULE,
    .new_smi  = ipmi_new_smi,
    .smi_gone = ipmi_smi_gone
    };
#[no_mangle]
unsafe extern "C" fn action_op_set_val(inval: *const c_char) -> c_int {
    static int action_op_set_val(const char *inval)
    {
    if (strcmp(inval, "reset") == 0)
    action_val = WDOG_TIMEOUT_RESET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "none") ==) -> else {
    else if (strcmp(inval, "none") == 0)
    action_val = WDOG_TIMEOUT_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "power_cycle") ==) -> else {
    else if (strcmp(inval, "power_cycle") == 0)
    action_val = WDOG_TIMEOUT_POWER_CYCLE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "power_off") ==) -> else {
    else if (strcmp(inval, "power_off") == 0)
    action_val = WDOG_TIMEOUT_POWER_DOWN;
    else
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn action_op(inval: *const c_char, outval: *mut c_char) -> c_int {
    static int action_op(const char *inval, char *outval)
    {
    int rv;
    if (outval)
    strcpy(outval, action);
    if (!inval)
    return 0;
    rv = action_op_set_val(inval);
    if (!rv)
    strcpy(action, inval);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn preaction_op_set_val(inval: *const c_char) -> c_int {
    static int preaction_op_set_val(const char *inval)
    {
    if (strcmp(inval, "pre_none") == 0)
    preaction_val = WDOG_PRETIMEOUT_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "pre_smi") ==) -> else {
    else if (strcmp(inval, "pre_smi") == 0)
    preaction_val = WDOG_PRETIMEOUT_SMI;

#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "pre_nmi") ==) -> else {
    else if (strcmp(inval, "pre_nmi") == 0)
    preaction_val = WDOG_PRETIMEOUT_NMI;

#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "pre_int") ==) -> else {
    else if (strcmp(inval, "pre_int") == 0)
    preaction_val = WDOG_PRETIMEOUT_MSG_INT;
    else
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preaction_op(inval: *const c_char, outval: *mut c_char) -> c_int {
    static int preaction_op(const char *inval, char *outval)
    {
    int rv;
    if (outval)
    strcpy(outval, preaction);
    if (!inval)
    return 0;
    rv = preaction_op_set_val(inval);
    if (!rv)
    strcpy(preaction, inval);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preop_op_set_val(inval: *const c_char) -> c_int {
    static int preop_op_set_val(const char *inval)
    {
    if (strcmp(inval, "preop_none") == 0)
    preop_val = WDOG_PREOP_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "preop_panic") ==) -> else {
    else if (strcmp(inval, "preop_panic") == 0)
    preop_val = WDOG_PREOP_PANIC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(inval, 0: "preop_give_data") ==) -> else {
    else if (strcmp(inval, "preop_give_data") == 0)
    preop_val = WDOG_PREOP_GIVE_DATA;
    else
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preop_op(inval: *const c_char, outval: *mut c_char) -> c_int {
    static int preop_op(const char *inval, char *outval)
    {
    int rv;
    if (outval)
    strcpy(outval, preop);
    if (!inval)
    return 0;
    rv = preop_op_set_val(inval);
    if (!rv)
    strcpy(preop, inval);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_parms() {
    static void check_parms(void)
    {

    let mut do_nmi: c_int = 0;
    int rv;
    if (preaction_val == WDOG_PRETIMEOUT_NMI) {
    do_nmi = 1;
    if (preop_val == WDOG_PREOP_GIVE_DATA) {
    pr_warn("Pretimeout op is to give data but NMI pretimeout is enabled, setting pretimeout op to none\n");
    preop_op("preop_none", core::ptr::null_mut());
    do_nmi = 0;
    }
    }
    if (do_nmi && !nmi_handler_registered) {
    rv = register_nmi_handler(NMI_UNKNOWN, ipmi_nmi, 0,
    "ipmi");
    if (rv) {
    pr_warn("Can't register nmi handler\n");
    return;
    } else
    nmi_handler_registered = 1;
    } else if (!do_nmi && nmi_handler_registered) {
    unregister_nmi_handler(NMI_UNKNOWN, "ipmi");
    nmi_handler_registered = 0;
    }

    }
#[no_mangle]
unsafe extern "C" fn ipmi_wdog_init() -> int __init {
    static int __init ipmi_wdog_init(void)
    {
    int rv;
    if (action_op_set_val(action)) {
    action_op("reset", core::ptr::null_mut());
    pr_info("Unknown action '%s', defaulting to reset\n", action);
    }
    if (preaction_op_set_val(preaction)) {
    preaction_op("pre_none", core::ptr::null_mut());
    pr_info("Unknown preaction '%s', defaulting to none\n",
    preaction);
    }
    if (preop_op_set_val(preop)) {
    preop_op("preop_none", core::ptr::null_mut());
    pr_info("Unknown preop '%s', defaulting to none\n", preop);
    }
    check_parms();
    register_reboot_notifier(&wdog_reboot_notifier);
    rv = ipmi_smi_watcher_register(&smi_watcher);
    if (rv) {

    if (nmi_handler_registered)
    unregister_nmi_handler(NMI_UNKNOWN, "ipmi");

    unregister_reboot_notifier(&wdog_reboot_notifier);
    pr_warn("can't register smi watcher\n");
    return rv;
    }
    pr_info("driver initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_wdog_exit() -> void __exit {
    static void __exit ipmi_wdog_exit(void)
    {
    ipmi_smi_watcher_unregister(&smi_watcher);
    ipmi_unregister_watchdog(watchdog_ifnum);

    if (nmi_handler_registered)
    unregister_nmi_handler(NMI_UNKNOWN, "ipmi");

    unregister_reboot_notifier(&wdog_reboot_notifier);
    }
    module_exit(ipmi_wdog_exit);
    module_init(ipmi_wdog_init);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Corey Minyard <minyard@mvista.com>");
    MODULE_DESCRIPTION("watchdog timer based upon the IPMI interface.");
