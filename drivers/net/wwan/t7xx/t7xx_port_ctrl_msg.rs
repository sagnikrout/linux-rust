//! Automatically rewritten from C to Rust
//! Source: drivers/net/wwan/t7xx/t7xx_port_ctrl_msg.c
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Haijun Liu <haijun.liu@mediatek.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
// Moises Veleta <moises.veleta@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Eliot Lee <eliot.lee@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_msg {
    pub head_pattern: __le32,
    pub info: __le32,
    pub tail_pattern: __le32,
    pub data: [__le32; ],
}

#[no_mangle]
unsafe extern "C" fn port_ctl_send_msg_to_md(port: *mut t7xx_port, msg: c_uint, ex_msg: c_uint) -> c_int {
    static int port_ctl_send_msg_to_md(struct t7xx_port *port, unsigned int msg, unsigned int ex_msg)
    {
    struct sk_buff *skb;
    int ret;
    skb = t7xx_ctrl_alloc_skb(0);
    if (!skb)
    return -ENOMEM;
    ret = t7xx_port_send_ctl_skb(port, skb, msg, ex_msg);
    if (ret)
    dev_kfree_skb_any(skb);
    return ret;
    }
    static int fsm_ee_message_handler(struct t7xx_port *port, struct t7xx_fsm_ctl *ctl,
    struct sk_buff *skb)
    {
    struct ctrl_msg_header *ctrl_msg_h = (struct ctrl_msg_header *)skb.data;
    struct device *dev = &ctl.md.t7xx_dev.pdev.dev;
    enum md_state md_state;
    let mut ret: c_int = -EINVAL;
    md_state = t7xx_fsm_get_md_state(ctl);
    if (md_state != MD_STATE_EXCEPTION) {
    dev_err(dev, "Receive invalid MD_EX %x when MD state is %d\n",
    ctrl_msg_h.ex_msg, md_state);
    return -EINVAL;
    }
    switch (le32_to_cpu(ctrl_msg_h.ctrl_msg_id)) {
    case CTL_ID_MD_EX:
    if (le32_to_cpu(ctrl_msg_h.ex_msg) != MD_EX_CHK_ID) {
    dev_err(dev, "Receive invalid MD_EX %x\n", ctrl_msg_h.ex_msg);
    break;
    }
    ret = port_ctl_send_msg_to_md(port, CTL_ID_MD_EX, MD_EX_CHK_ID);
    if (ret) {
    dev_err(dev, "Failed to send exception message to modem\n");
    break;
    }
    ret = t7xx_fsm_append_event(ctl, FSM_EVENT_MD_EX, core::ptr::null_mut(), 0);
    if (ret)
    dev_err(dev, "Failed to append Modem Exception event");
    break;
    case CTL_ID_MD_EX_ACK:
    if (le32_to_cpu(ctrl_msg_h.ex_msg) != MD_EX_CHK_ACK_ID) {
    dev_err(dev, "Receive invalid MD_EX_ACK %x\n", ctrl_msg_h.ex_msg);
    break;
    }
    ret = t7xx_fsm_append_event(ctl, FSM_EVENT_MD_EX_REC_OK, core::ptr::null_mut(), 0);
    if (ret)
    dev_err(dev, "Failed to append Modem Exception Received event");
    break;
    case CTL_ID_MD_EX_PASS:
    ret = t7xx_fsm_append_event(ctl, FSM_EVENT_MD_EX_PASS, core::ptr::null_mut(), 0);
    if (ret)
    dev_err(dev, "Failed to append Modem Exception Passed event");
    break;
    case CTL_ID_DRV_VER_ERROR:
    dev_err(dev, "AP/MD driver version mismatch\n");
    }
    return ret;
    }
//
// t7xx_port_enum_msg_handler() - Parse the port enumeration message to create/remove nodes.
// @md: Modem context.
// @msg: Message.
// @msg_len:	Length of @msg in bytes.
//
// Used to control create/remove device node.
//
// Return:
// * 0		- Success.
// * -EFAULT	- Message check failure.
//
#[no_mangle]
pub unsafe extern "C" fn t7xx_port_enum_msg_handler(md: *mut t7xx_modem, msg: *mut c_void, msg_len: usize) -> c_int {
    int t7xx_port_enum_msg_handler(struct t7xx_modem *md, void *msg, size_t msg_len)
    {
    struct device *dev = &md.t7xx_dev.pdev.dev;
    unsigned int version, port_count, i;
    struct port_msg *port_msg = msg;
    if (msg_len < sizeof(*port_msg)) {
    dev_err(dev, "Port enum msg too short for header: need %zu, have %zu\n",
    sizeof(*port_msg), msg_len);
    return -EINVAL;
    }
    version = FIELD_GET(PORT_MSG_VERSION, le32_to_cpu(port_msg.info));
    if (version != PORT_ENUM_VER ||
    le32_to_cpu(port_msg.head_pattern) != PORT_ENUM_HEAD_PATTERN ||
    le32_to_cpu(port_msg.tail_pattern) != PORT_ENUM_TAIL_PATTERN) {
    dev_err(dev, "Invalid port control message %x:%x:%x\n",
    version, le32_to_cpu(port_msg.head_pattern),
    le32_to_cpu(port_msg.tail_pattern));
    return -EFAULT;
    }
    port_count = FIELD_GET(PORT_MSG_PRT_CNT, le32_to_cpu(port_msg.info));
    if (msg_len < struct_size(port_msg, data, port_count)) {
    dev_err(dev, "Port enum msg too short: need %zu, have %zu\n",
    struct_size(port_msg, data, port_count), msg_len);
    return -EINVAL;
    }
    for (i = 0; i < port_count; i++) {
    let mut port_info: u32 = le32_to_cpu(port_msg.data[i]);
    unsigned int ch_id;
    bool en_flag;
    ch_id = FIELD_GET(PORT_INFO_CH_ID, port_info);
    en_flag = port_info & PORT_INFO_ENFLG;
    if (t7xx_port_proxy_chl_enable_disable(md.port_prox, ch_id, en_flag))
    dev_dbg(dev, "Port:%x not found\n", ch_id);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn control_msg_handler(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int {
    static int control_msg_handler(struct t7xx_port *port, struct sk_buff *skb)
    {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    struct t7xx_fsm_ctl *ctl = port.t7xx_dev.md.fsm_ctl;
    struct ctrl_msg_header *ctrl_msg_h;
    let mut ret: c_int = 0;
    ctrl_msg_h = (struct ctrl_msg_header *)skb.data;
    switch (le32_to_cpu(ctrl_msg_h.ctrl_msg_id)) {
    case CTL_ID_HS2_MSG:
    skb_pull(skb, sizeof(*ctrl_msg_h));
    if (port_conf.rx_ch == PORT_CH_CONTROL_RX ||
    port_conf.rx_ch == PORT_CH_AP_CONTROL_RX) {
    int event = port_conf.rx_ch == PORT_CH_CONTROL_RX ?
    FSM_EVENT_MD_HS2 : FSM_EVENT_AP_HS2;
    ret = t7xx_fsm_append_event(ctl, event, skb.data,
    le32_to_cpu(ctrl_msg_h.data_length));
    if (ret)
    dev_err(port.dev, "Failed to append Handshake 2 event");
    }
    dev_kfree_skb_any(skb);
    break;
    case CTL_ID_MD_EX:
    case CTL_ID_MD_EX_ACK:
    case CTL_ID_MD_EX_PASS:
    case CTL_ID_DRV_VER_ERROR:
    ret = fsm_ee_message_handler(port, ctl, skb);
    dev_kfree_skb_any(skb);
    break;
    case CTL_ID_PORT_ENUM:
    skb_pull(skb, sizeof(*ctrl_msg_h));
    ret = t7xx_port_enum_msg_handler(ctl.md, (struct port_msg *)skb.data, skb.len);
    if (!ret)
    ret = port_ctl_send_msg_to_md(port, CTL_ID_PORT_ENUM, 0);
    else
    ret = port_ctl_send_msg_to_md(port, CTL_ID_PORT_ENUM,
    PORT_ENUM_VER_MISMATCH);
    break;
    default:
    ret = -EINVAL;
    dev_err(port.dev, "Unknown control message ID to FSM %x\n",
    le32_to_cpu(ctrl_msg_h.ctrl_msg_id));
    break;
    }
    if (ret)
    dev_err(port.dev, "%s control message handle error: %d\n", port_conf.name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn port_ctl_rx_thread(arg: *mut c_void) -> c_int {
    static int port_ctl_rx_thread(void *arg)
    {
    while (!kthread_should_stop()) {
    struct t7xx_port *port = arg;
    struct sk_buff *skb;
    unsigned long flags;
    spin_lock_irqsave(&port.rx_wq.lock, flags);
    if (skb_queue_empty(&port.rx_skb_list) &&
    wait_event_interruptible_locked_irq(port.rx_wq,
    !skb_queue_empty(&port.rx_skb_list) ||
    kthread_should_stop())) {
    spin_unlock_irqrestore(&port.rx_wq.lock, flags);
    continue;
    }
    if (kthread_should_stop()) {
    spin_unlock_irqrestore(&port.rx_wq.lock, flags);
    break;
    }
    skb = __skb_dequeue(&port.rx_skb_list);
    spin_unlock_irqrestore(&port.rx_wq.lock, flags);
    control_msg_handler(port, skb);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn port_ctl_init(port: *mut t7xx_port) -> c_int {
    static int port_ctl_init(struct t7xx_port *port)
    {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    port.thread = kthread_run(port_ctl_rx_thread, port, "%s", port_conf.name);
    if (IS_ERR(port.thread)) {
    dev_err(port.dev, "Failed to start port control thread\n");
    return PTR_ERR(port.thread);
    }
    port.rx_length_th = CTRL_QUEUE_MAXLEN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn port_ctl_uninit(port: *mut t7xx_port) {
    static void port_ctl_uninit(struct t7xx_port *port)
    {
    unsigned long flags;
    struct sk_buff *skb;
    if (port.thread)
    kthread_stop(port.thread);
    spin_lock_irqsave(&port.rx_wq.lock, flags);
    port.rx_length_th = 0;
    while ((skb = __skb_dequeue(&port.rx_skb_list)) != core::ptr::null_mut())
    dev_kfree_skb_any(skb);
    spin_unlock_irqrestore(&port.rx_wq.lock, flags);
    }
    struct port_ops ctl_port_ops = {
    .init = port_ctl_init,
    .recv_skb = t7xx_port_enqueue_skb,
    .uninit = port_ctl_uninit,
    };
