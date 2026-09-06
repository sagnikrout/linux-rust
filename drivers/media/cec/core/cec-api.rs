//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/core/cec-api.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// cec-api.c - HDMI Consumer Electronics Control framework - API
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

    static inline struct cec_devnode *cec_devnode_data(struct file *filp)
    {
    struct cec_fh *fh = filp.private_data;
    return &fh.adap.devnode;
    }
// CEC file operations
    static __poll_t cec_poll(struct file *filp,
    struct poll_table_struct *poll)
    {
    struct cec_fh *fh = filp.private_data;
    struct cec_adapter *adap = fh.adap;
    let mut res: __poll_t = 0;
    poll_wait(filp, &fh.wait, poll);
    if (!cec_is_registered(adap))
    return EPOLLERR | EPOLLHUP | EPOLLPRI;
    mutex_lock(&adap.lock);
    if (adap.is_configured &&
    adap.transmit_queue_sz < CEC_MAX_MSG_TX_QUEUE_SZ)
    res |= EPOLLOUT | EPOLLWRNORM;
    if (fh.queued_msgs)
    res |= EPOLLIN | EPOLLRDNORM;
    if (fh.total_queued_events)
    res |= EPOLLPRI;
    mutex_unlock(&adap.lock);
    return res;
    }
    static bool cec_is_busy(const struct cec_adapter *adap,
    const struct cec_fh *fh)
    {
    let mut valid_initiator: bool = adap.cec_initiator && adap.cec_initiator == fh;
    let mut valid_follower: bool = adap.cec_follower && adap.cec_follower == fh;
//
// Exclusive initiators and followers can always access the CEC adapter
//
    if (valid_initiator || valid_follower)
    return false;
//
// All others can only access the CEC adapter if there is no
// exclusive initiator and they are in INITIATOR mode.
//
    return adap.cec_initiator ||
    fh.mode_initiator == CEC_MODE_NO_INITIATOR;
    }
    static long cec_adap_g_caps(struct cec_adapter *adap,
    struct cec_caps __user *parg)
    {
    let mut caps: cec_caps = {};
    strscpy(caps.driver, adap.devnode.dev.parent.driver.name,
    sizeof(caps.driver));
    strscpy(caps.name, adap.name, sizeof(caps.name));
    caps.available_log_addrs = adap.available_log_addrs;
    caps.capabilities = adap.capabilities;
    caps.version = LINUX_VERSION_CODE;
    if (copy_to_user(parg, &caps, sizeof(caps)))
    return -EFAULT;
    return 0;
    }
    static long cec_adap_g_phys_addr(struct cec_adapter *adap,
    __u16 __user *parg)
    {
    u16 phys_addr;
    mutex_lock(&adap.lock);
    phys_addr = adap.phys_addr;
    mutex_unlock(&adap.lock);
    if (copy_to_user(parg, &phys_addr, sizeof(phys_addr)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cec_validate_phys_addr(phys_addr: u16) -> c_int {
    static int cec_validate_phys_addr(u16 phys_addr)
    {
    int i;
    if (phys_addr == CEC_PHYS_ADDR_INVALID)
    return 0;
    for (i = 0; i < 16; i += 4)
    if (phys_addr & (0xf << i))
    break;
    if (i == 16)
    return 0;
    for (i += 4; i < 16; i += 4)
    if ((phys_addr & (0xf << i)) == 0)
    return -EINVAL;
    return 0;
    }
    static long cec_adap_s_phys_addr(struct cec_adapter *adap, struct cec_fh *fh,
    bool block, __u16 __user *parg)
    {
    u16 phys_addr;
    long err;
    if (!(adap.capabilities & CEC_CAP_PHYS_ADDR))
    return -ENOTTY;
    if (copy_from_user(&phys_addr, parg, sizeof(phys_addr)))
    return -EFAULT;
    err = cec_validate_phys_addr(phys_addr);
    if (err)
    return err;
    mutex_lock(&adap.lock);
    if (cec_is_busy(adap, fh))
    err = -EBUSY;
    else
    __cec_s_phys_addr(adap, phys_addr, block);
    mutex_unlock(&adap.lock);
    return err;
    }
    static long cec_adap_g_log_addrs(struct cec_adapter *adap,
    struct cec_log_addrs __user *parg)
    {
    struct cec_log_addrs log_addrs;
    mutex_lock(&adap.lock);
//
// We use memcpy here instead of assignment since there is a
// hole at the end of struct cec_log_addrs that an assignment
// might ignore. So when we do copy_to_user() we could leak
// one byte of memory.
//
    memcpy(&log_addrs, &adap.log_addrs, sizeof(log_addrs));
    if (!adap.is_configured)
    memset(log_addrs.log_addr, CEC_LOG_ADDR_INVALID,
    sizeof(log_addrs.log_addr));
    mutex_unlock(&adap.lock);
    if (copy_to_user(parg, &log_addrs, sizeof(log_addrs)))
    return -EFAULT;
    return 0;
    }
    static long cec_adap_s_log_addrs(struct cec_adapter *adap, struct cec_fh *fh,
    bool block, struct cec_log_addrs __user *parg)
    {
    struct cec_log_addrs log_addrs;
    let mut err: c_long = -EBUSY;
    if (!(adap.capabilities & CEC_CAP_LOG_ADDRS))
    return -ENOTTY;
    if (copy_from_user(&log_addrs, parg, sizeof(log_addrs)))
    return -EFAULT;
    log_addrs.flags &= CEC_LOG_ADDRS_FL_ALLOW_UNREG_FALLBACK |
    CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU |
    CEC_LOG_ADDRS_FL_CDC_ONLY;
    mutex_lock(&adap.lock);
    if (!adap.is_claiming_log_addrs && !adap.is_configuring &&
    (!log_addrs.num_log_addrs || !adap.is_configured) &&
    !cec_is_busy(adap, fh)) {
    err = __cec_s_log_addrs(adap, &log_addrs, block);
    if (!err)
    log_addrs = adap.log_addrs;
    }
    mutex_unlock(&adap.lock);
    if (err)
    return err;
    if (copy_to_user(parg, &log_addrs, sizeof(log_addrs)))
    return -EFAULT;
    return 0;
    }
    static long cec_adap_g_connector_info(struct cec_adapter *adap,
    struct cec_log_addrs __user *parg)
    {
    let mut ret: c_int = 0;
    if (!(adap.capabilities & CEC_CAP_CONNECTOR_INFO))
    return -ENOTTY;
    mutex_lock(&adap.lock);
    if (copy_to_user(parg, &adap.conn_info, sizeof(adap.conn_info)))
    ret = -EFAULT;
    mutex_unlock(&adap.lock);
    return ret;
    }
    static long cec_transmit(struct cec_adapter *adap, struct cec_fh *fh,
    bool block, struct cec_msg __user *parg)
    {
    let mut msg: cec_msg = {};
    let mut err: c_long = 0;
    if (!(adap.capabilities & CEC_CAP_TRANSMIT))
    return -ENOTTY;
    if (copy_from_user(&msg, parg, sizeof(msg)))
    return -EFAULT;
    mutex_lock(&adap.lock);
    if (adap.log_addrs.num_log_addrs == 0)
    err = -EPERM;
#[no_mangle]
pub unsafe extern "C" fn if(!msg_is_raw(&msg): adap->is_configuring &&) -> else {
    else if (adap.is_configuring && !msg_is_raw(&msg))
    err = -ENONET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cec_is_busy(adap, _arg: fh)) -> else {
    else if (cec_is_busy(adap, fh))
    err = -EBUSY;
    else
    err = cec_transmit_msg_fh(adap, &msg, fh, block);
    mutex_unlock(&adap.lock);
    if (err)
    return err;
    if (copy_to_user(parg, &msg, sizeof(msg)))
    return -EFAULT;
    return 0;
    }
// Called by CEC_RECEIVE: wait for a message to arrive
#[no_mangle]
unsafe extern "C" fn cec_receive_msg(fh: *mut cec_fh, msg: *mut cec_msg, block: bool) -> c_int {
    static int cec_receive_msg(struct cec_fh *fh, struct cec_msg *msg, bool block)
    {
    let mut timeout: u32 = msg.timeout;
    int res;
    do {
    mutex_lock(&fh.lock);
// Are there received messages queued up?
    if (fh.queued_msgs) {
// Yes, return the first one
    struct cec_msg_entry *entry =
    list_first_entry(&fh.msgs,
    struct cec_msg_entry, list);
    list_del(&entry.list);
// msg = entry->msg;
    kfree(entry);
    fh.queued_msgs--;
    mutex_unlock(&fh.lock);
// restore original timeout value
    msg.timeout = timeout;
    return 0;
    }
// No, return EAGAIN in non-blocking mode or wait
    mutex_unlock(&fh.lock);
// Return when in non-blocking mode
    if (!block)
    return -EAGAIN;
    if (msg.timeout) {
// The user specified a timeout
    res = wait_event_interruptible_timeout(fh.wait,
    fh.queued_msgs,
    msecs_to_jiffies(msg.timeout));
    if (res == 0)
    res = -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(0: res >) -> else {
    else if (res > 0)
    res = 0;
    } else {
// Wait indefinitely
    res = wait_event_interruptible(fh.wait,
    fh.queued_msgs);
    }
// Exit on error, otherwise loop to get the new message
    } while (!res);
    return res;
    }
    static long cec_receive(struct cec_adapter *adap, struct cec_fh *fh,
    bool block, struct cec_msg __user *parg)
    {
    let mut msg: cec_msg = {};
    long err;
    if (copy_from_user(&msg, parg, sizeof(msg)))
    return -EFAULT;
    err = cec_receive_msg(fh, &msg, block);
    if (err)
    return err;
    msg.flags = 0;
    if (copy_to_user(parg, &msg, sizeof(msg)))
    return -EFAULT;
    return 0;
    }
    static long cec_dqevent(struct cec_adapter *adap, struct cec_fh *fh,
    bool block, struct cec_event __user *parg)
    {
    struct cec_event_entry *ev = core::ptr::null_mut();
    let mut ts: u64 = ~0ULL;
    unsigned int i;
    unsigned int ev_idx;
    let mut err: c_long = 0;
    mutex_lock(&fh.lock);
    while (!fh.total_queued_events && block) {
    mutex_unlock(&fh.lock);
    err = wait_event_interruptible(fh.wait,
    fh.total_queued_events);
    if (err)
    return err;
    mutex_lock(&fh.lock);
    }
// Find the oldest event
    for (i = 0; i < CEC_NUM_EVENTS; i++) {
    struct cec_event_entry *entry =
    list_first_entry_or_null(&fh.events[i],
    struct cec_event_entry, list);
    if (entry && entry.ev.ts <= ts) {
    ev = entry;
    ev_idx = i;
    ts = ev.ev.ts;
    }
    }
    if (!ev) {
    err = -EAGAIN;
    goto unlock;
    }
    list_del(&ev.list);
    if (copy_to_user(parg, &ev.ev, sizeof(ev.ev)))
    err = -EFAULT;
    kfree(ev);
    fh.queued_events[ev_idx]--;
    fh.total_queued_events--;
    unlock:
    mutex_unlock(&fh.lock);
    return err;
    }
    static long cec_g_mode(struct cec_adapter *adap, struct cec_fh *fh,
    u32 __user *parg)
    {
    let mut mode: u32 = fh.mode_initiator | fh.mode_follower;
    if (copy_to_user(parg, &mode, sizeof(mode)))
    return -EFAULT;
    return 0;
    }
    static long cec_s_mode(struct cec_adapter *adap, struct cec_fh *fh,
    u32 __user *parg)
    {
    u32 mode;
    u8 mode_initiator;
    u8 mode_follower;
    let mut send_pin_event: bool = false;
    let mut err: c_long = 0;
    if (copy_from_user(&mode, parg, sizeof(mode)))
    return -EFAULT;
    if (mode & ~(CEC_MODE_INITIATOR_MSK | CEC_MODE_FOLLOWER_MSK)) {
    dprintk(1, "%s: invalid mode bits set\n", __func__);
    return -EINVAL;
    }
    mode_initiator = mode & CEC_MODE_INITIATOR_MSK;
    mode_follower = mode & CEC_MODE_FOLLOWER_MSK;
    if (mode_initiator > CEC_MODE_EXCL_INITIATOR ||
    mode_follower > CEC_MODE_MONITOR_ALL) {
    dprintk(1, "%s: unknown mode\n", __func__);
    return -EINVAL;
    }
    if (mode_follower == CEC_MODE_MONITOR_ALL &&
    !(adap.capabilities & CEC_CAP_MONITOR_ALL)) {
    dprintk(1, "%s: MONITOR_ALL not supported\n", __func__);
    return -EINVAL;
    }
    if (mode_follower == CEC_MODE_MONITOR_PIN &&
    !(adap.capabilities & CEC_CAP_MONITOR_PIN)) {
    dprintk(1, "%s: MONITOR_PIN not supported\n", __func__);
    return -EINVAL;
    }
// Follower modes should always be able to send CEC messages
    if ((mode_initiator == CEC_MODE_NO_INITIATOR ||
    !(adap.capabilities & CEC_CAP_TRANSMIT)) &&
    mode_follower >= CEC_MODE_FOLLOWER &&
    mode_follower <= CEC_MODE_EXCL_FOLLOWER_PASSTHRU) {
    dprintk(1, "%s: cannot transmit\n", __func__);
    return -EINVAL;
    }
// Monitor modes require CEC_MODE_NO_INITIATOR
    if (mode_initiator && mode_follower >= CEC_MODE_MONITOR_PIN) {
    dprintk(1, "%s: monitor modes require NO_INITIATOR\n",
    __func__);
    return -EINVAL;
    }
// Monitor modes require CAP_NET_ADMIN
    if (mode_follower >= CEC_MODE_MONITOR_PIN && !capable(CAP_NET_ADMIN))
    return -EPERM;
    mutex_lock(&adap.lock);
//
// You can't become exclusive follower if someone else already
// has that job.
//
    if ((mode_follower == CEC_MODE_EXCL_FOLLOWER ||
    mode_follower == CEC_MODE_EXCL_FOLLOWER_PASSTHRU) &&
    adap.cec_follower && adap.cec_follower != fh)
    err = -EBUSY;
//
// You can't become exclusive initiator if someone else already
// has that job.
//
    if (mode_initiator == CEC_MODE_EXCL_INITIATOR &&
    adap.cec_initiator && adap.cec_initiator != fh)
    err = -EBUSY;
    if (!err) {
    let mut old_mon_all: bool = fh.mode_follower == CEC_MODE_MONITOR_ALL;
    let mut new_mon_all: bool = mode_follower == CEC_MODE_MONITOR_ALL;
    if (old_mon_all != new_mon_all) {
    if (new_mon_all)
    err = cec_monitor_all_cnt_inc(adap);
    else
    cec_monitor_all_cnt_dec(adap);
    }
    }
    if (!err) {
    let mut old_mon_pin: bool = fh.mode_follower == CEC_MODE_MONITOR_PIN;
    let mut new_mon_pin: bool = mode_follower == CEC_MODE_MONITOR_PIN;
    if (old_mon_pin != new_mon_pin) {
    send_pin_event = new_mon_pin;
    if (new_mon_pin)
    err = cec_monitor_pin_cnt_inc(adap);
    else
    cec_monitor_pin_cnt_dec(adap);
    }
    }
    if (err) {
    mutex_unlock(&adap.lock);
    return err;
    }
    if (fh.mode_follower == CEC_MODE_FOLLOWER)
    adap.follower_cnt--;
    if (mode_follower == CEC_MODE_FOLLOWER)
    adap.follower_cnt++;
    if (send_pin_event) {
    struct cec_event ev = {
    .flags = CEC_EVENT_FL_INITIAL_STATE,
    };
    ev.event = adap.cec_pin_is_high ? CEC_EVENT_PIN_CEC_HIGH :
    CEC_EVENT_PIN_CEC_LOW;
    cec_queue_event_fh(fh, &ev, 0);
    }
    if (mode_follower == CEC_MODE_EXCL_FOLLOWER ||
    mode_follower == CEC_MODE_EXCL_FOLLOWER_PASSTHRU) {
    adap.passthrough =
    mode_follower == CEC_MODE_EXCL_FOLLOWER_PASSTHRU;
    adap.cec_follower = fh;
    } else if (adap.cec_follower == fh) {
    adap.passthrough = false;
    adap.cec_follower = core::ptr::null_mut();
    }
    if (mode_initiator == CEC_MODE_EXCL_INITIATOR)
    adap.cec_initiator = fh;
#[no_mangle]
pub unsafe extern "C" fn if(fh: adap->cec_initiator ==) -> else {
    else if (adap.cec_initiator == fh)
    adap.cec_initiator = core::ptr::null_mut();
    fh.mode_initiator = mode_initiator;
    fh.mode_follower = mode_follower;
    mutex_unlock(&adap.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cec_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long cec_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    struct cec_fh *fh = filp.private_data;
    struct cec_adapter *adap = fh.adap;
    let mut block: bool = !(filp.f_flags & O_NONBLOCK);
    void __user *parg = (void __user *)arg;
    if (!cec_is_registered(adap))
    return -ENODEV;
    switch (cmd) {
    case CEC_ADAP_G_CAPS:
    return cec_adap_g_caps(adap, parg);
    case CEC_ADAP_G_PHYS_ADDR:
    return cec_adap_g_phys_addr(adap, parg);
    case CEC_ADAP_S_PHYS_ADDR:
    return cec_adap_s_phys_addr(adap, fh, block, parg);
    case CEC_ADAP_G_LOG_ADDRS:
    return cec_adap_g_log_addrs(adap, parg);
    case CEC_ADAP_S_LOG_ADDRS:
    return cec_adap_s_log_addrs(adap, fh, block, parg);
    case CEC_ADAP_G_CONNECTOR_INFO:
    return cec_adap_g_connector_info(adap, parg);
    case CEC_TRANSMIT:
    return cec_transmit(adap, fh, block, parg);
    case CEC_RECEIVE:
    return cec_receive(adap, fh, block, parg);
    case CEC_DQEVENT:
    return cec_dqevent(adap, fh, block, parg);
    case CEC_G_MODE:
    return cec_g_mode(adap, fh, parg);
    case CEC_S_MODE:
    return cec_s_mode(adap, fh, parg);
    default:
    return -ENOTTY;
    }
    }
#[no_mangle]
unsafe extern "C" fn cec_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int cec_open(struct inode *inode, struct file *filp)
    {
    struct cec_devnode *devnode =
    container_of(inode.i_cdev, struct cec_devnode, cdev);
    struct cec_adapter *adap = to_cec_adapter(devnode);
    struct cec_fh *fh = kzalloc_obj(*fh);
//
// Initial events that are automatically sent when the cec device is
// opened.
//
    struct cec_event ev = {
    .event = CEC_EVENT_STATE_CHANGE,
    .flags = CEC_EVENT_FL_INITIAL_STATE,
    };
    unsigned int i;
    int err;
    if (!fh)
    return -ENOMEM;
    INIT_LIST_HEAD(&fh.msgs);
    INIT_LIST_HEAD(&fh.xfer_list);
    for (i = 0; i < CEC_NUM_EVENTS; i++)
    INIT_LIST_HEAD(&fh.events[i]);
    mutex_init(&fh.lock);
    init_waitqueue_head(&fh.wait);
    fh.mode_initiator = CEC_MODE_INITIATOR;
    fh.adap = adap;
    err = cec_get_device(adap);
    if (err) {
    kfree(fh);
    return err;
    }
    filp.private_data = fh;
// Queue up initial state events
    ev.state_change.phys_addr = adap.phys_addr;
    ev.state_change.log_addr_mask = adap.log_addrs.log_addr_mask;
    ev.state_change.have_conn_info =
    adap.conn_info.type != CEC_CONNECTOR_TYPE_NO_CONNECTOR;
    cec_queue_event_fh(fh, &ev, 0);

    if (adap.pin && adap.pin.ops.read_hpd &&
    !adap.devnode.unregistered) {
    err = adap.pin.ops.read_hpd(adap);
    if (err >= 0) {
    ev.event = err ? CEC_EVENT_PIN_HPD_HIGH :
    CEC_EVENT_PIN_HPD_LOW;
    cec_queue_event_fh(fh, &ev, 0);
    }
    }
    if (adap.pin && adap.pin.ops.read_5v &&
    !adap.devnode.unregistered) {
    err = adap.pin.ops.read_5v(adap);
    if (err >= 0) {
    ev.event = err ? CEC_EVENT_PIN_5V_HIGH :
    CEC_EVENT_PIN_5V_LOW;
    cec_queue_event_fh(fh, &ev, 0);
    }
    }

    mutex_lock(&devnode.lock);
    mutex_lock(&devnode.lock_fhs);
    list_add(&fh.list, &devnode.fhs);
    mutex_unlock(&devnode.lock_fhs);
    mutex_unlock(&devnode.lock);
    return 0;
    }
// Override for the release function
#[no_mangle]
unsafe extern "C" fn cec_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int cec_release(struct inode *inode, struct file *filp)
    {
    struct cec_devnode *devnode = cec_devnode_data(filp);
    struct cec_adapter *adap = to_cec_adapter(devnode);
    struct cec_fh *fh = filp.private_data;
    unsigned int i;
    mutex_lock(&adap.lock);
    if (adap.cec_initiator == fh)
    adap.cec_initiator = core::ptr::null_mut();
    if (adap.cec_follower == fh) {
    adap.cec_follower = core::ptr::null_mut();
    adap.passthrough = false;
    }
    if (fh.mode_follower == CEC_MODE_FOLLOWER)
    adap.follower_cnt--;
    if (fh.mode_follower == CEC_MODE_MONITOR_PIN)
    cec_monitor_pin_cnt_dec(adap);
    if (fh.mode_follower == CEC_MODE_MONITOR_ALL)
    cec_monitor_all_cnt_dec(adap);
    mutex_unlock(&adap.lock);
    mutex_lock(&devnode.lock);
    mutex_lock(&devnode.lock_fhs);
    list_del(&fh.list);
    mutex_unlock(&devnode.lock_fhs);
    mutex_unlock(&devnode.lock);
// Unhook pending transmits from this filehandle.
    mutex_lock(&adap.lock);
    while (!list_empty(&fh.xfer_list)) {
    struct cec_data *data =
    list_first_entry(&fh.xfer_list, struct cec_data, xfer_list);
    data.blocking = false;
    data.fh = core::ptr::null_mut();
    list_del_init(&data.xfer_list);
    }
    mutex_unlock(&adap.lock);
    mutex_lock(&fh.lock);
    while (!list_empty(&fh.msgs)) {
    struct cec_msg_entry *entry =
    list_first_entry(&fh.msgs, struct cec_msg_entry, list);
    list_del(&entry.list);
    kfree(entry);
    }
    for (i = 0; i < CEC_NUM_EVENTS; i++) {
    while (!list_empty(&fh.events[i])) {
    struct cec_event_entry *entry =
    list_first_entry(&fh.events[i],
    struct cec_event_entry, list);
    list_del(&entry.list);
    kfree(entry);
    }
    }
    mutex_unlock(&fh.lock);
    kfree(fh);
    cec_put_device(adap);
    filp.private_data = core::ptr::null_mut();
    return 0;
    }
    const struct file_operations cec_devnode_fops = {
    .owner = THIS_MODULE,
    .open = cec_open,
    .unlocked_ioctl = cec_ioctl,
    .compat_ioctl = cec_ioctl,
    .release = cec_release,
    .poll = cec_poll,
    };
