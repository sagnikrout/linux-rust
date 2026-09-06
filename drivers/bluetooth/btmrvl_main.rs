//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/btmrvl_main.c
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
// Marvell Bluetooth driver
//
// Copyright (C) 2009, Marvell International Ltd.
//

//
// This function is called by interface specific interrupt handler.
// It updates Power Save & Host Sleep states, and wakes up the main
// thread.
//
#[no_mangle]
pub unsafe extern "C" fn btmrvl_interrupt(priv: *mut btmrvl_private) {
    void btmrvl_interrupt(struct btmrvl_private *priv)
    {
    priv.adapter.ps_state = PS_AWAKE;
    priv.adapter.wakeup_tries = 0;
    priv.adapter.int_count++;
    if (priv.adapter.hs_state == HS_ACTIVATED) {
    BT_DBG("BT: HS DEACTIVATED in ISR!");
    priv.adapter.hs_state = HS_DEACTIVATED;
    }
    wake_up_interruptible(&priv.main_thread.wait_q);
    }
    EXPORT_SYMBOL_GPL(btmrvl_interrupt);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_check_evtpkt(priv: *mut btmrvl_private, skb: *mut sk_buff) -> bool {
    bool btmrvl_check_evtpkt(struct btmrvl_private *priv, struct sk_buff *skb)
    {
    struct hci_event_hdr *hdr = (void *) skb.data;
    if (skb.len < sizeof(*hdr))
    return true;
    if (hdr.evt == HCI_EV_CMD_COMPLETE) {
    struct hci_ev_cmd_complete *ec;
    u16 opcode;
    if (hdr.plen < sizeof(*ec) ||
    skb.len < HCI_EVENT_HDR_SIZE + sizeof(*ec))
    return true;
    ec = (void *) (skb.data + HCI_EVENT_HDR_SIZE);
    opcode = __le16_to_cpu(ec.opcode);
    if (priv.btmrvl_dev.sendcmdflag) {
    priv.btmrvl_dev.sendcmdflag = false;
    priv.adapter.cmd_complete = true;
    wake_up_interruptible(&priv.adapter.cmd_wait_q);
    if (hci_opcode_ogf(opcode) == 0x3F) {
    BT_DBG("vendor event skipped: opcode=%#4.4x",
    opcode);
    kfree_skb(skb);
    return false;
    }
    }
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(btmrvl_check_evtpkt);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_process_event(priv: *mut btmrvl_private, skb: *mut sk_buff) -> c_int {
    int btmrvl_process_event(struct btmrvl_private *priv, struct sk_buff *skb)
    {
    struct btmrvl_adapter *adapter = priv.adapter;
    struct btmrvl_event *event;
    let mut ret: c_int = 0;
    if (skb.len <= offsetof(typeof(*event), data[0]))
    return -EINVAL;
    event = (struct btmrvl_event *) skb.data;
    if (event.ec != 0xff) {
    BT_DBG("Not Marvell Event=%x", event.ec);
    ret = -EINVAL;
    goto exit;
    }
    switch (event.data[0]) {
    case BT_EVENT_AUTO_SLEEP_MODE:
    if (skb.len <= offsetof(typeof(*event), data[2]))
    return -EINVAL;
    if (!event.data[2]) {
    if (event.data[1] == BT_PS_ENABLE)
    adapter.psmode = 1;
    else
    adapter.psmode = 0;
    BT_DBG("PS Mode:%s",
    str_enable_disable(adapter.psmode));
    } else {
    BT_DBG("PS Mode command failed");
    }
    break;
    case BT_EVENT_HOST_SLEEP_CONFIG:
    if (skb.len <= offsetof(typeof(*event), data[3]))
    return -EINVAL;
    if (!event.data[3])
    BT_DBG("gpio=%x, gap=%x", event.data[1],
    event.data[2]);
    else
    BT_DBG("HSCFG command failed");
    break;
    case BT_EVENT_HOST_SLEEP_ENABLE:
    if (skb.len <= offsetof(typeof(*event), data[1]))
    return -EINVAL;
    if (!event.data[1]) {
    adapter.hs_state = HS_ACTIVATED;
    if (adapter.psmode)
    adapter.ps_state = PS_SLEEP;
    wake_up_interruptible(&adapter.event_hs_wait_q);
    BT_DBG("HS ACTIVATED!");
    } else {
    BT_DBG("HS Enable failed");
    }
    break;
    case BT_EVENT_MODULE_CFG_REQ:
    if (skb.len <= offsetof(typeof(*event), data[2]))
    return -EINVAL;
    if (priv.btmrvl_dev.sendcmdflag &&
    event.data[1] == MODULE_BRINGUP_REQ) {
    BT_DBG("EVENT:%s",
    ((event.data[2] == MODULE_BROUGHT_UP) ||
    (event.data[2] == MODULE_ALREADY_UP)) ?
    "Bring-up succeed" : "Bring-up failed");
    } else if (priv.btmrvl_dev.sendcmdflag &&
    event.data[1] == MODULE_SHUTDOWN_REQ) {
    BT_DBG("EVENT:%s", (event.data[2]) ?
    "Shutdown failed" : "Shutdown succeed");
    } else {
    BT_DBG("BT_CMD_MODULE_CFG_REQ resp for APP");
    ret = -EINVAL;
    }
    break;
    case BT_EVENT_POWER_STATE:
    if (skb.len <= offsetof(typeof(*event), data[1]))
    return -EINVAL;
    if (event.data[1] == BT_PS_SLEEP)
    adapter.ps_state = PS_SLEEP;
    BT_DBG("EVENT:%s",
    (adapter.ps_state) ? "PS_SLEEP" : "PS_AWAKE");
    break;
    default:
    BT_DBG("Unknown Event=%d", event.data[0]);
    ret = -EINVAL;
    break;
    }
    exit:
    if (!ret)
    kfree_skb(skb);
    return ret;
    }
    EXPORT_SYMBOL_GPL(btmrvl_process_event);
    static int btmrvl_send_sync_cmd(struct btmrvl_private *priv, u16 opcode,
    const void *param, u8 len)
    {
    struct sk_buff *skb;
    struct hci_command_hdr *hdr;
    if (priv.surprise_removed) {
    BT_ERR("Card is removed");
    return -EFAULT;
    }
    skb = bt_skb_alloc(HCI_COMMAND_HDR_SIZE + len, GFP_KERNEL);
    if (!skb) {
    BT_ERR("No free skb");
    return -ENOMEM;
    }
    hdr = skb_put(skb, HCI_COMMAND_HDR_SIZE);
    hdr.opcode = cpu_to_le16(opcode);
    hdr.plen = len;
    if (len)
    skb_put_data(skb, param, len);
    hci_skb_pkt_type(skb) = MRVL_VENDOR_PKT;
    skb_queue_head(&priv.adapter.tx_queue, skb);
    priv.btmrvl_dev.sendcmdflag = true;
    priv.adapter.cmd_complete = false;
    wake_up_interruptible(&priv.main_thread.wait_q);
    if (!wait_event_interruptible_timeout(priv.adapter.cmd_wait_q,
    priv.adapter.cmd_complete ||
    priv.surprise_removed,
    WAIT_UNTIL_CMD_RESP))
    return -ETIMEDOUT;
    if (priv.surprise_removed)
    return -EFAULT;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btmrvl_send_module_cfg_cmd(priv: *mut btmrvl_private, subcmd: u8) -> c_int {
    int btmrvl_send_module_cfg_cmd(struct btmrvl_private *priv, u8 subcmd)
    {
    int ret;
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_MODULE_CFG_REQ, &subcmd, 1);
    if (ret)
    BT_ERR("module_cfg_cmd(%x) failed", subcmd);
    return ret;
    }
    EXPORT_SYMBOL_GPL(btmrvl_send_module_cfg_cmd);
#[no_mangle]
unsafe extern "C" fn btmrvl_enable_sco_routing_to_host(priv: *mut btmrvl_private) -> c_int {
    static int btmrvl_enable_sco_routing_to_host(struct btmrvl_private *priv)
    {
    int ret;
    let mut subcmd: u8 = 0;
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_ROUTE_SCO_TO_HOST, &subcmd, 1);
    if (ret)
    BT_ERR("BT_CMD_ROUTE_SCO_TO_HOST command failed: %#x", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn btmrvl_pscan_window_reporting(priv: *mut btmrvl_private, subcmd: u8) -> c_int {
    int btmrvl_pscan_window_reporting(struct btmrvl_private *priv, u8 subcmd)
    {
    struct btmrvl_sdio_card *card = priv.btmrvl_dev.card;
    int ret;
    if (!card.support_pscan_win_report)
    return 0;
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_PSCAN_WIN_REPORT_ENABLE,
    &subcmd, 1);
    if (ret)
    BT_ERR("PSCAN_WIN_REPORT_ENABLE command failed: %#x", ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(btmrvl_pscan_window_reporting);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_send_hscfg_cmd(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_send_hscfg_cmd(struct btmrvl_private *priv)
    {
    int ret;
    u8 param[2];
    param[0] = (priv.btmrvl_dev.gpio_gap & 0xff00) >> 8;
    param[1] = (u8) (priv.btmrvl_dev.gpio_gap & 0x00ff);
    BT_DBG("Sending HSCFG Command, gpio=0x%x, gap=0x%x",
    param[0], param[1]);
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_HOST_SLEEP_CONFIG, param, 2);
    if (ret)
    BT_ERR("HSCFG command failed");
    return ret;
    }
    EXPORT_SYMBOL_GPL(btmrvl_send_hscfg_cmd);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_enable_ps(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_enable_ps(struct btmrvl_private *priv)
    {
    int ret;
    u8 param;
    if (priv.btmrvl_dev.psmode)
    param = BT_PS_ENABLE;
    else
    param = BT_PS_DISABLE;
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_AUTO_SLEEP_MODE, &param, 1);
    if (ret)
    BT_ERR("PSMODE command failed");
    return 0;
    }
    EXPORT_SYMBOL_GPL(btmrvl_enable_ps);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_enable_hs(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_enable_hs(struct btmrvl_private *priv)
    {
    struct btmrvl_adapter *adapter = priv.adapter;
    int ret;
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_HOST_SLEEP_ENABLE, core::ptr::null_mut(), 0);
    if (ret) {
    BT_ERR("Host sleep enable command failed");
    return ret;
    }
    ret = wait_event_interruptible_timeout(adapter.event_hs_wait_q,
    adapter.hs_state ||
    priv.surprise_removed,
    WAIT_UNTIL_HS_STATE_CHANGED);
    if (ret < 0 || priv.surprise_removed) {
    BT_ERR("event_hs_wait_q terminated (%d): %d,%d,%d",
    ret, adapter.hs_state, adapter.ps_state,
    adapter.wakeup_tries);
    } else if (!ret) {
    BT_ERR("hs_enable timeout: %d,%d,%d", adapter.hs_state,
    adapter.ps_state, adapter.wakeup_tries);
    ret = -ETIMEDOUT;
    } else {
    BT_DBG("host sleep enabled: %d,%d,%d", adapter.hs_state,
    adapter.ps_state, adapter.wakeup_tries);
    ret = 0;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(btmrvl_enable_hs);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_prepare_command(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_prepare_command(struct btmrvl_private *priv)
    {
    let mut ret: c_int = 0;
    if (priv.btmrvl_dev.hscfgcmd) {
    priv.btmrvl_dev.hscfgcmd = 0;
    btmrvl_send_hscfg_cmd(priv);
    }
    if (priv.btmrvl_dev.pscmd) {
    priv.btmrvl_dev.pscmd = 0;
    btmrvl_enable_ps(priv);
    }
    if (priv.btmrvl_dev.hscmd) {
    priv.btmrvl_dev.hscmd = 0;
    if (priv.btmrvl_dev.hsmode) {
    ret = btmrvl_enable_hs(priv);
    } else {
    ret = priv.hw_wakeup_firmware(priv);
    priv.adapter.hs_state = HS_DEACTIVATED;
    BT_DBG("BT: HS DEACTIVATED due to host activity!");
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_tx_pkt(priv: *mut btmrvl_private, skb: *mut sk_buff) -> c_int {
    static int btmrvl_tx_pkt(struct btmrvl_private *priv, struct sk_buff *skb)
    {
    let mut ret: c_int = 0;
    if (!skb || !skb.data)
    return -EINVAL;
    if (!skb.len || ((skb.len + BTM_HEADER_LEN) > BTM_UPLD_SIZE)) {
    BT_ERR("Tx Error: Bad skb length %d : %d",
    skb.len, BTM_UPLD_SIZE);
    return -EINVAL;
    }
    skb_push(skb, BTM_HEADER_LEN);
// header type: byte[3]
// HCI_COMMAND = 1, ACL_DATA = 2, SCO_DATA = 3, 0xFE = Vendor
// header length: byte[2][1][0]
//
    skb.data[0] = (skb.len & 0x0000ff);
    skb.data[1] = (skb.len & 0x00ff00) >> 8;
    skb.data[2] = (skb.len & 0xff0000) >> 16;
    skb.data[3] = hci_skb_pkt_type(skb);
    if (priv.hw_host_to_card)
    ret = priv.hw_host_to_card(priv, skb.data, skb.len);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_init_adapter(priv: *mut btmrvl_private) {
    static void btmrvl_init_adapter(struct btmrvl_private *priv)
    {
    int buf_size;
    skb_queue_head_init(&priv.adapter.tx_queue);
    priv.adapter.ps_state = PS_AWAKE;
    buf_size = ALIGN_SZ(SDIO_BLOCK_SIZE, BTSDIO_DMA_ALIGN);
    priv.adapter.hw_regs_buf = kzalloc(buf_size, GFP_KERNEL);
    if (!priv.adapter.hw_regs_buf) {
    priv.adapter.hw_regs = core::ptr::null_mut();
    BT_ERR("Unable to allocate buffer for hw_regs.");
    } else {
    priv.adapter.hw_regs =
    (u8 *)ALIGN_ADDR(priv.adapter.hw_regs_buf,
    BTSDIO_DMA_ALIGN);
    BT_DBG("hw_regs_buf=%p hw_regs=%p",
    priv.adapter.hw_regs_buf, priv.adapter.hw_regs);
    }
    init_waitqueue_head(&priv.adapter.cmd_wait_q);
    init_waitqueue_head(&priv.adapter.event_hs_wait_q);
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_free_adapter(priv: *mut btmrvl_private) {
    static void btmrvl_free_adapter(struct btmrvl_private *priv)
    {
    skb_queue_purge(&priv.adapter.tx_queue);
    kfree(priv.adapter.hw_regs_buf);
    kfree(priv.adapter);
    priv.adapter = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int btmrvl_send_frame(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    BT_DBG("type=%d, len=%d", hci_skb_pkt_type(skb), skb.len);
    if (priv.adapter.is_suspending || priv.adapter.is_suspended) {
    BT_ERR("%s: Device is suspending or suspended", __func__);
    return -EBUSY;
    }
    switch (hci_skb_pkt_type(skb)) {
    case HCI_COMMAND_PKT:
    hdev.stat.cmd_tx++;
    break;
    case HCI_ACLDATA_PKT:
    hdev.stat.acl_tx++;
    break;
    case HCI_SCODATA_PKT:
    hdev.stat.sco_tx++;
    break;
    }
    skb_queue_tail(&priv.adapter.tx_queue, skb);
    if (!priv.adapter.is_suspended)
    wake_up_interruptible(&priv.main_thread.wait_q);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_flush(hdev: *mut hci_dev) -> c_int {
    static int btmrvl_flush(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    skb_queue_purge(&priv.adapter.tx_queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_close(hdev: *mut hci_dev) -> c_int {
    static int btmrvl_close(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    skb_queue_purge(&priv.adapter.tx_queue);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_open(hdev: *mut hci_dev) -> c_int {
    static int btmrvl_open(struct hci_dev *hdev)
    {
    return 0;
    }
    static int btmrvl_download_cal_data(struct btmrvl_private *priv,
    u8 *data, int len)
    {
    int ret;
    data[0] = 0x00;
    data[1] = 0x00;
    data[2] = 0x00;
    data[3] = len;
    print_hex_dump_bytes("Calibration data: ",
    DUMP_PREFIX_OFFSET, data, BT_CAL_HDR_LEN + len);
    ret = btmrvl_send_sync_cmd(priv, BT_CMD_LOAD_CONFIG_DATA, data,
    BT_CAL_HDR_LEN + len);
    if (ret)
    BT_ERR("Failed to download calibration data");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_check_device_tree(priv: *mut btmrvl_private) -> c_int {
    static int btmrvl_check_device_tree(struct btmrvl_private *priv)
    {
    struct device_node *dt_node;
    struct btmrvl_sdio_card *card = priv.btmrvl_dev.card;
    u8 cal_data[BT_CAL_HDR_LEN + BT_CAL_DATA_SIZE];
    let mut ret: c_int = 0;
    u16 gpio, gap;
    if (card.plt_of_node) {
    dt_node = card.plt_of_node;
    ret = of_property_read_u16(dt_node, "marvell,wakeup-pin",
    &gpio);
    if (ret)
    gpio = (priv.btmrvl_dev.gpio_gap & 0xff00) >> 8;
    ret = of_property_read_u16(dt_node, "marvell,wakeup-gap-ms",
    &gap);
    if (ret)
    gap = (u8)(priv.btmrvl_dev.gpio_gap & 0x00ff);
    priv.btmrvl_dev.gpio_gap = (gpio << 8) + gap;
    ret = of_property_read_u8_array(dt_node, "marvell,cal-data",
    cal_data + BT_CAL_HDR_LEN,
    BT_CAL_DATA_SIZE);
    if (ret)
    return ret;
    BT_DBG("Use cal data from device tree");
    ret = btmrvl_download_cal_data(priv, cal_data,
    BT_CAL_DATA_SIZE);
    if (ret)
    BT_ERR("Fail to download calibrate data");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_setup(hdev: *mut hci_dev) -> c_int {
    static int btmrvl_setup(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    int ret;
    ret = btmrvl_send_module_cfg_cmd(priv, MODULE_BRINGUP_REQ);
    if (ret)
    return ret;
    priv.btmrvl_dev.gpio_gap = 0xfffe;
    btmrvl_check_device_tree(priv);
    btmrvl_enable_sco_routing_to_host(priv);
    btmrvl_pscan_window_reporting(priv, 0x01);
    priv.btmrvl_dev.psmode = 1;
    btmrvl_enable_ps(priv);
    btmrvl_send_hscfg_cmd(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int btmrvl_set_bdaddr(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    struct sk_buff *skb;
    long ret;
    u8 buf[8];
    buf[0] = MRVL_VENDOR_PKT;
    buf[1] = sizeof(bdaddr_t);
    memcpy(buf + 2, bdaddr, sizeof(bdaddr_t));
    skb = __hci_cmd_sync(hdev, BT_CMD_SET_BDADDR, sizeof(buf), buf,
    HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    ret = PTR_ERR(skb);
    BT_ERR("%s: changing btmrvl device address failed (%ld)",
    hdev.name, ret);
    return ret;
    }
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btmrvl_wakeup(hdev: *mut hci_dev) -> bool {
    static bool btmrvl_wakeup(struct hci_dev *hdev)
    {
    struct btmrvl_private *priv = hci_get_drvdata(hdev);
    struct btmrvl_sdio_card *card = priv.btmrvl_dev.card;
    return device_may_wakeup(&card.func.dev);
    }
//
// This function handles the event generated by firmware, rx data
// received from firmware, and tx data sent from kernel.
//
#[no_mangle]
unsafe extern "C" fn btmrvl_service_main_thread(data: *mut c_void) -> c_int {
    static int btmrvl_service_main_thread(void *data)
    {
    struct btmrvl_thread *thread = data;
    struct btmrvl_private *priv = thread.priv;
    struct btmrvl_adapter *adapter = priv.adapter;
    wait_queue_entry_t wait;
    struct sk_buff *skb;
    ulong flags;
    init_waitqueue_entry(&wait, current);
    for (;;) {
    add_wait_queue(&thread.wait_q, &wait);
    set_current_state(TASK_INTERRUPTIBLE);
    if (kthread_should_stop() || priv.surprise_removed) {
    BT_DBG("main_thread: break from main thread");
    break;
    }
    if (adapter.wakeup_tries ||
    ((!adapter.int_count) &&
    (!priv.btmrvl_dev.tx_dnld_rdy ||
    skb_queue_empty(&adapter.tx_queue)))) {
    BT_DBG("main_thread is sleeping...");
    schedule();
    }
    set_current_state(TASK_RUNNING);
    remove_wait_queue(&thread.wait_q, &wait);
    BT_DBG("main_thread woke up");
    if (kthread_should_stop() || priv.surprise_removed) {
    BT_DBG("main_thread: break from main thread");
    break;
    }
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (adapter.int_count) {
    adapter.int_count = 0;
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    priv.hw_process_int_status(priv);
    } else if (adapter.ps_state == PS_SLEEP &&
    !skb_queue_empty(&adapter.tx_queue)) {
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    adapter.wakeup_tries++;
    priv.hw_wakeup_firmware(priv);
    continue;
    } else {
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    }
    if (adapter.ps_state == PS_SLEEP)
    continue;
    if (!priv.btmrvl_dev.tx_dnld_rdy ||
    priv.adapter.is_suspended)
    continue;
    skb = skb_dequeue(&adapter.tx_queue);
    if (skb) {
    if (btmrvl_tx_pkt(priv, skb))
    priv.btmrvl_dev.hcidev.stat.err_tx++;
    else
    priv.btmrvl_dev.hcidev.stat.byte_tx += skb.len;
    kfree_skb(skb);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btmrvl_register_hdev(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_register_hdev(struct btmrvl_private *priv)
    {
    struct hci_dev *hdev = core::ptr::null_mut();
    struct btmrvl_sdio_card *card = priv.btmrvl_dev.card;
    int ret;
    hdev = hci_alloc_dev();
    if (!hdev) {
    BT_ERR("Can not allocate HCI device");
    goto err_hdev;
    }
    priv.btmrvl_dev.hcidev = hdev;
    hci_set_drvdata(hdev, priv);
    hdev.bus   = HCI_SDIO;
    hdev.open  = btmrvl_open;
    hdev.close = btmrvl_close;
    hdev.flush = btmrvl_flush;
    hdev.send  = btmrvl_send_frame;
    hdev.setup = btmrvl_setup;
    hdev.set_bdaddr = btmrvl_set_bdaddr;
    hdev.wakeup = btmrvl_wakeup;
    SET_HCIDEV_DEV(hdev, &card.func.dev);
    ret = hci_register_dev(hdev);
    if (ret < 0) {
    BT_ERR("Can not register HCI device");
    goto err_hci_register_dev;
    }

    btmrvl_debugfs_init(hdev);

    return 0;
    err_hci_register_dev:
    hci_free_dev(hdev);
    err_hdev:
// Stop the thread servicing the interrupts
    kthread_stop(priv.main_thread.task);
    btmrvl_free_adapter(priv);
    kfree(priv);
    return -ENOMEM;
    }
    EXPORT_SYMBOL_GPL(btmrvl_register_hdev);
    struct btmrvl_private *btmrvl_add_card(void *card)
    {
    struct btmrvl_private *priv;
    priv = kzalloc_obj(*priv);
    if (!priv) {
    BT_ERR("Can not allocate priv");
    goto err_priv;
    }
    priv.adapter = kzalloc_obj(*priv.adapter);
    if (!priv.adapter) {
    BT_ERR("Allocate buffer for btmrvl_adapter failed!");
    goto err_adapter;
    }
    btmrvl_init_adapter(priv);
    BT_DBG("Starting kthread...");
    priv.main_thread.priv = priv;
    spin_lock_init(&priv.driver_lock);
    init_waitqueue_head(&priv.main_thread.wait_q);
    priv.main_thread.task = kthread_run(btmrvl_service_main_thread,
    &priv.main_thread, "btmrvl_main_service");
    if (IS_ERR(priv.main_thread.task))
    goto err_thread;
    priv.btmrvl_dev.card = card;
    priv.btmrvl_dev.tx_dnld_rdy = true;
    return priv;
    err_thread:
    btmrvl_free_adapter(priv);
    err_adapter:
    kfree(priv);
    err_priv:
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(btmrvl_add_card);
#[no_mangle]
pub unsafe extern "C" fn btmrvl_remove_card(priv: *mut btmrvl_private) -> c_int {
    int btmrvl_remove_card(struct btmrvl_private *priv)
    {
    struct hci_dev *hdev;
    hdev = priv.btmrvl_dev.hcidev;
    wake_up_interruptible(&priv.adapter.cmd_wait_q);
    wake_up_interruptible(&priv.adapter.event_hs_wait_q);
    kthread_stop(priv.main_thread.task);

    btmrvl_debugfs_remove(hdev);

    hci_unregister_dev(hdev);
    hci_free_dev(hdev);
    priv.btmrvl_dev.hcidev = core::ptr::null_mut();
    btmrvl_free_adapter(priv);
    kfree(priv);
    return 0;
    }
    EXPORT_SYMBOL_GPL(btmrvl_remove_card);
    MODULE_AUTHOR("Marvell International Ltd.");
    MODULE_DESCRIPTION("Marvell Bluetooth driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL v2");
