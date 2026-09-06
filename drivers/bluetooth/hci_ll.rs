//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/hci_ll.c
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
// Texas Instruments' Bluetooth HCILL UART protocol
//
// HCILL (HCI Low Level) is a Texas Instruments' power management
// protocol extension to H4.
//
// Copyright (C) 2007 Texas Instruments, Inc.
//
// Written by Ohad Ben-Cohen <ohad@bencohen.org>
//
// Acknowledgements:
// This file is based on hci_h4.c, which was written
// by Maxim Krasnyansky and Marcel Holtmann.
//

// Vendor-specific HCI commands
pub const HCI_VS_WRITE_BD_ADDR: c_uint = 0xfc06;
pub const HCI_VS_UPDATE_UART_HCI_BAUDRATE: c_uint = 0xff36;
// HCILL commands
pub const HCILL_GO_TO_SLEEP_IND: c_uint = 0x30;
pub const HCILL_GO_TO_SLEEP_ACK: c_uint = 0x31;
pub const HCILL_WAKE_UP_IND: c_uint = 0x32;
pub const HCILL_WAKE_UP_ACK: c_uint = 0x33;
// HCILL states
    enum hcill_states_e {
    HCILL_ASLEEP,
    HCILL_ASLEEP_TO_AWAKE,
    HCILL_AWAKE,
    HCILL_AWAKE_TO_ASLEEP
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ll_device {
    pub hu: hci_uart,
    pub serdev: *mut serdev_device,
    pub enable_gpio: *mut gpio_desc,
    pub ext_clk: *mut clk,
    pub bdaddr: bdaddr_t,
    pub broken_enhanced_setup: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ll_struct {
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
    pub /: *mut *mut spinlock_t hcill_lock; / HCILL state lock,
    pub /: *mut *mut unsigned long hcill_state; / HCILL power state,
    pub /: *mut *mut sk_buff_head tx_wait_q; / HCILL wait queue,
}

//
// Builds and sends an HCILL command packet.
// These are very simple packets with only 1 cmd byte
//
#[no_mangle]
unsafe extern "C" fn send_hcill_cmd(cmd: u8, hu: *mut hci_uart) -> c_int {
    static int send_hcill_cmd(u8 cmd, struct hci_uart *hu)
    {
    let mut err: c_int = 0;
    struct sk_buff *skb = core::ptr::null_mut();
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p cmd 0x%x", hu, cmd);
// allocate packet
    skb = bt_skb_alloc(1, GFP_ATOMIC);
    if (!skb) {
    BT_ERR("cannot allocate memory for HCILL packet");
    err = -ENOMEM;
    goto out;
    }
// prepare packet
    skb_put_u8(skb, cmd);
// send packet
    skb_queue_tail(&ll.txq, skb);
    out:
    return err;
    }
// Initialize protocol
#[no_mangle]
unsafe extern "C" fn ll_open(hu: *mut hci_uart) -> c_int {
    static int ll_open(struct hci_uart *hu)
    {
    struct ll_struct *ll;
    BT_DBG("hu %p", hu);
    ll = kzalloc_obj(*ll);
    if (!ll)
    return -ENOMEM;
    skb_queue_head_init(&ll.txq);
    skb_queue_head_init(&ll.tx_wait_q);
    spin_lock_init(&ll.hcill_lock);
    ll.hcill_state = HCILL_AWAKE;
    hu.priv = ll;
    if (hu.serdev) {
    struct ll_device *lldev = serdev_device_get_drvdata(hu.serdev);
    if (!IS_ERR(lldev.ext_clk))
    clk_prepare_enable(lldev.ext_clk);
    }
    return 0;
    }
// Flush protocol data
#[no_mangle]
unsafe extern "C" fn ll_flush(hu: *mut hci_uart) -> c_int {
    static int ll_flush(struct hci_uart *hu)
    {
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p", hu);
    skb_queue_purge(&ll.tx_wait_q);
    skb_queue_purge(&ll.txq);
    return 0;
    }
// Close protocol
#[no_mangle]
unsafe extern "C" fn ll_close(hu: *mut hci_uart) -> c_int {
    static int ll_close(struct hci_uart *hu)
    {
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p", hu);
    skb_queue_purge(&ll.tx_wait_q);
    skb_queue_purge(&ll.txq);
    kfree_skb(ll.rx_skb);
    if (hu.serdev) {
    struct ll_device *lldev = serdev_device_get_drvdata(hu.serdev);
    gpiod_set_value_cansleep(lldev.enable_gpio, 0);
    clk_disable_unprepare(lldev.ext_clk);
    }
    hu.priv = core::ptr::null_mut();
    kfree(ll);
    return 0;
    }
//
// internal function, which does common work of the device wake up process:
// 1. places all pending packets (waiting in tx_wait_q list) in txq list.
// 2. changes internal state to HCILL_AWAKE.
// Note: assumes that hcill_lock spinlock is taken,
// shouldn't be called otherwise!
//
#[no_mangle]
unsafe extern "C" fn __ll_do_awake(ll: *mut ll_struct) {
    static void __ll_do_awake(struct ll_struct *ll)
    {
    struct sk_buff *skb = core::ptr::null_mut();
    while ((skb = skb_dequeue(&ll.tx_wait_q)))
    skb_queue_tail(&ll.txq, skb);
    ll.hcill_state = HCILL_AWAKE;
    }
//
// Called upon a wake-up-indication from the device
//
#[no_mangle]
unsafe extern "C" fn ll_device_want_to_wakeup(hu: *mut hci_uart) {
    static void ll_device_want_to_wakeup(struct hci_uart *hu)
    {
    unsigned long flags;
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p", hu);
// lock hcill state
    spin_lock_irqsave(&ll.hcill_lock, flags);
    switch (ll.hcill_state) {
    case HCILL_ASLEEP_TO_AWAKE:
//
// This state means that both the host and the BRF chip
// have simultaneously sent a wake-up-indication packet.
// Traditionally, in this case, receiving a wake-up-indication
// was enough and an additional wake-up-ack wasn't needed.
// This has changed with the BRF6350, which does require an
// explicit wake-up-ack. Other BRF versions, which do not
// require an explicit ack here, do accept it, thus it is
// perfectly safe to always send one.
//
    BT_DBG("dual wake-up-indication");
    fallthrough;
    case HCILL_ASLEEP:
// acknowledge device wake up
    if (send_hcill_cmd(HCILL_WAKE_UP_ACK, hu) < 0) {
    BT_ERR("cannot acknowledge device wake up");
    goto out;
    }
    break;
    default:
// any other state is illegal
    BT_ERR("received HCILL_WAKE_UP_IND in state %ld",
    ll.hcill_state);
    break;
    }
// send pending packets and change state to HCILL_AWAKE
    __ll_do_awake(ll);
    out:
    spin_unlock_irqrestore(&ll.hcill_lock, flags);
// actually send the packets
    hci_uart_tx_wakeup(hu);
    }
//
// Called upon a sleep-indication from the device
//
#[no_mangle]
unsafe extern "C" fn ll_device_want_to_sleep(hu: *mut hci_uart) {
    static void ll_device_want_to_sleep(struct hci_uart *hu)
    {
    unsigned long flags;
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p", hu);
// lock hcill state
    spin_lock_irqsave(&ll.hcill_lock, flags);
// sanity check
    if (ll.hcill_state != HCILL_AWAKE)
    BT_ERR("ERR: HCILL_GO_TO_SLEEP_IND in state %ld",
    ll.hcill_state);
// acknowledge device sleep
    if (send_hcill_cmd(HCILL_GO_TO_SLEEP_ACK, hu) < 0) {
    BT_ERR("cannot acknowledge device sleep");
    goto out;
    }
// update state
    ll.hcill_state = HCILL_ASLEEP;
    out:
    spin_unlock_irqrestore(&ll.hcill_lock, flags);
// actually send the sleep ack packet
    hci_uart_tx_wakeup(hu);
    }
//
// Called upon wake-up-acknowledgement from the device
//
#[no_mangle]
unsafe extern "C" fn ll_device_woke_up(hu: *mut hci_uart) {
    static void ll_device_woke_up(struct hci_uart *hu)
    {
    unsigned long flags;
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p", hu);
// lock hcill state
    spin_lock_irqsave(&ll.hcill_lock, flags);
// sanity check
    if (ll.hcill_state != HCILL_ASLEEP_TO_AWAKE)
    BT_ERR("received HCILL_WAKE_UP_ACK in state %ld",
    ll.hcill_state);
// send pending packets and change state to HCILL_AWAKE
    __ll_do_awake(ll);
    spin_unlock_irqrestore(&ll.hcill_lock, flags);
// actually send the packets
    hci_uart_tx_wakeup(hu);
    }
// Enqueue frame for transmission (padding, crc, etc)
// may be called from two simultaneous tasklets
#[no_mangle]
unsafe extern "C" fn ll_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    static int ll_enqueue(struct hci_uart *hu, struct sk_buff *skb)
    {
    let mut flags: c_ulong = 0;
    struct ll_struct *ll = hu.priv;
    BT_DBG("hu %p skb %p", hu, skb);
// Prepend skb with frame type
    memcpy(skb_push(skb, 1), &hci_skb_pkt_type(skb), 1);
// lock hcill state
    spin_lock_irqsave(&ll.hcill_lock, flags);
// act according to current state
    switch (ll.hcill_state) {
    case HCILL_AWAKE:
    BT_DBG("device awake, sending normally");
    skb_queue_tail(&ll.txq, skb);
    break;
    case HCILL_ASLEEP:
    BT_DBG("device asleep, waking up and queueing packet");
// save packet for later
    skb_queue_tail(&ll.tx_wait_q, skb);
// awake device
    if (send_hcill_cmd(HCILL_WAKE_UP_IND, hu) < 0) {
    BT_ERR("cannot wake up device");
    break;
    }
    ll.hcill_state = HCILL_ASLEEP_TO_AWAKE;
    break;
    case HCILL_ASLEEP_TO_AWAKE:
    BT_DBG("device waking up, queueing packet");
// transient state; just keep packet for later
    skb_queue_tail(&ll.tx_wait_q, skb);
    break;
    default:
    BT_ERR("illegal hcill state: %ld (losing packet)",
    ll.hcill_state);
    dev_kfree_skb_irq(skb);
    break;
    }
    spin_unlock_irqrestore(&ll.hcill_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ll_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int ll_recv_frame(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct hci_uart *hu = hci_get_drvdata(hdev);
    struct ll_struct *ll = hu.priv;
    switch (hci_skb_pkt_type(skb)) {
    case HCILL_GO_TO_SLEEP_IND:
    BT_DBG("HCILL_GO_TO_SLEEP_IND packet");
    ll_device_want_to_sleep(hu);
    break;
    case HCILL_GO_TO_SLEEP_ACK:
// shouldn't happen
    bt_dev_err(hdev, "received HCILL_GO_TO_SLEEP_ACK in state %ld",
    ll.hcill_state);
    break;
    case HCILL_WAKE_UP_IND:
    BT_DBG("HCILL_WAKE_UP_IND packet");
    ll_device_want_to_wakeup(hu);
    break;
    case HCILL_WAKE_UP_ACK:
    BT_DBG("HCILL_WAKE_UP_ACK packet");
    ll_device_woke_up(hu);
    break;
    }
    kfree_skb(skb);
    return 0;
    }

    .type = HCILL_GO_TO_SLEEP_IND, \
    .hlen = 0, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = 0

    .type = HCILL_GO_TO_SLEEP_ACK, \
    .hlen = 0, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = 0

    .type = HCILL_WAKE_UP_IND, \
    .hlen = 0, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = 0

    .type = HCILL_WAKE_UP_ACK, \
    .hlen = 0, \
    .loff = 0, \
    .lsize = 0, \
    .maxlen = 0
    static const struct h4_recv_pkt ll_recv_pkts[] = {
    { H4_RECV_ACL,       .recv = hci_recv_frame },
    { H4_RECV_SCO,       .recv = hci_recv_frame },
    { H4_RECV_EVENT,     .recv = hci_recv_frame },
    { LL_RECV_SLEEP_IND, .recv = ll_recv_frame  },
    { LL_RECV_SLEEP_ACK, .recv = ll_recv_frame  },
    { LL_RECV_WAKE_IND,  .recv = ll_recv_frame  },
    { LL_RECV_WAKE_ACK,  .recv = ll_recv_frame  },
    };
// Recv data
#[no_mangle]
unsafe extern "C" fn ll_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    static int ll_recv(struct hci_uart *hu, const void *data, int count)
    {
    struct ll_struct *ll = hu.priv;
    if (!test_bit(HCI_UART_REGISTERED, &hu.flags))
    return -EUNATCH;
    ll.rx_skb = h4_recv_buf(hu, ll.rx_skb, data, count,
    ll_recv_pkts, ARRAY_SIZE(ll_recv_pkts));
    if (IS_ERR(ll.rx_skb)) {
    let mut err: c_int = PTR_ERR(ll.rx_skb);
    bt_dev_err(hu.hdev, "Frame reassembly failed (%d)", err);
    ll.rx_skb = core::ptr::null_mut();
    return err;
    }
    return count;
    }
    static struct sk_buff *ll_dequeue(struct hci_uart *hu)
    {
    struct ll_struct *ll = hu.priv;
    return skb_dequeue(&ll.txq);
    }

#[no_mangle]
unsafe extern "C" fn read_local_version(hdev: *mut hci_dev) -> c_int {
    static int read_local_version(struct hci_dev *hdev)
    {
    let mut err: c_int = 0;
    let mut version: c_ushort = 0;
    struct sk_buff *skb;
    struct hci_rp_read_local_version *ver;
    skb = __hci_cmd_sync(hdev, HCI_OP_READ_LOCAL_VERSION, 0, core::ptr::null_mut(),
    HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    bt_dev_err(hdev, "Reading TI version information failed (%ld)",
    PTR_ERR(skb));
    return PTR_ERR(skb);
    }
    if (skb.len != sizeof(*ver)) {
    err = -EILSEQ;
    goto out;
    }
    ver = (struct hci_rp_read_local_version *)skb.data;
    if (le16_to_cpu(ver.manufacturer) != 13) {
    err = -ENODEV;
    goto out;
    }
    version = le16_to_cpu(ver.lmp_subver);
    out:
    if (err)
    bt_dev_err(hdev, "Failed to read TI version info: %d", err);
    kfree_skb(skb);
    return err ? err : version;
    }
    static int send_command_from_firmware(struct ll_device *lldev,
    struct hci_command *cmd)
    {
    struct sk_buff *skb;
    if (cmd.opcode == HCI_VS_UPDATE_UART_HCI_BAUDRATE) {
// ignore remote change
// baud rate HCI VS command
//
    bt_dev_warn(lldev.hu.hdev,
    "change remote baud rate command in firmware");
    return 0;
    }
    if (cmd.prefix != 1)
    bt_dev_dbg(lldev.hu.hdev, "command type %d", cmd.prefix);
    skb = __hci_cmd_sync(lldev.hu.hdev, cmd.opcode, cmd.plen,
    &cmd.speed, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    bt_dev_err(lldev.hu.hdev, "send command failed");
    return PTR_ERR(skb);
    }
    kfree_skb(skb);
    return 0;
    }
//
// download_firmware -
// internal function which parses through the .bts firmware
// script file intreprets SEND, DELAY actions only as of now
//
#[no_mangle]
unsafe extern "C" fn download_firmware(lldev: *mut ll_device) -> c_int {
    static int download_firmware(struct ll_device *lldev)
    {
    unsigned short chip, min_ver, maj_ver;
    int version, err, len;
    unsigned char *ptr, *action_ptr;
    unsigned char bts_scr_name[40];	/* 40 char long bts scr name? */
    const struct firmware *fw;
    struct hci_command *cmd;
    version = read_local_version(lldev.hu.hdev);
    if (version < 0)
    return version;
    chip = (version & 0x7C00) >> 10;
    min_ver = (version & 0x007F);
    maj_ver = (version & 0x0380) >> 7;
    if (version & 0x8000)
    maj_ver |= 0x0008;
    snprintf(bts_scr_name, sizeof(bts_scr_name),
    "ti-connectivity/TIInit_%d.%d.%d.bts",
    chip, maj_ver, min_ver);
    err = request_firmware(&fw, bts_scr_name, &lldev.serdev.dev);
    if (err || !fw.data || !fw.size) {
    bt_dev_err(lldev.hu.hdev, "request_firmware failed(errno %d) for %s",
    err, bts_scr_name);
    if (!err)
    release_firmware(fw);
    return -EINVAL;
    }
    ptr = (void *)fw.data;
    len = fw.size;
// bts_header to remove out magic number and
// version
//
    ptr += sizeof(struct bts_header);
    len -= sizeof(struct bts_header);
    while (len > 0 && ptr) {
    bt_dev_dbg(lldev.hu.hdev, " action size %d, type %d ",
    ((struct bts_action *)ptr).size,
    ((struct bts_action *)ptr).type);
    action_ptr = &(((struct bts_action *)ptr).data[0]);
    switch (((struct bts_action *)ptr).type) {
    case ACTION_SEND_COMMAND:	/* action send */
    bt_dev_dbg(lldev.hu.hdev, "S");
    cmd = (struct hci_command *)action_ptr;
    err = send_command_from_firmware(lldev, cmd);
    if (err)
    goto out_rel_fw;
    break;
    case ACTION_WAIT_EVENT:  /* wait */
// no need to wait as command was synchronous
    bt_dev_dbg(lldev.hu.hdev, "W");
    break;
    case ACTION_DELAY:	/* sleep */
    bt_dev_info(lldev.hu.hdev, "sleep command in scr");
    msleep(((struct bts_action_delay *)action_ptr).msec);
    break;
    }
    len -= (sizeof(struct bts_action) +
    ((struct bts_action *)ptr).size);
    ptr += sizeof(struct bts_action) +
    ((struct bts_action *)ptr).size;
    }
    out_rel_fw:
// fw download complete
    release_firmware(fw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ll_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int ll_set_bdaddr(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    bdaddr_t bdaddr_swapped;
    struct sk_buff *skb;
// HCI_VS_WRITE_BD_ADDR (at least on a CC2560A chip) expects the BD
// address to be MSB first, but bdaddr_t has the convention of being
// LSB first.
//
    baswap(&bdaddr_swapped, bdaddr);
    skb = __hci_cmd_sync(hdev, HCI_VS_WRITE_BD_ADDR, sizeof(bdaddr_t),
    &bdaddr_swapped, HCI_INIT_TIMEOUT);
    if (!IS_ERR(skb))
    kfree_skb(skb);
    return PTR_ERR_OR_ZERO(skb);
    }
#[no_mangle]
unsafe extern "C" fn ll_setup(hu: *mut hci_uart) -> c_int {
    static int ll_setup(struct hci_uart *hu)
    {
    int err, retry = 3;
    struct ll_device *lldev;
    struct serdev_device *serdev = hu.serdev;
    u32 speed;
    if (!serdev)
    return 0;
    lldev = serdev_device_get_drvdata(serdev);
    hu.hdev.set_bdaddr = ll_set_bdaddr;
    serdev_device_set_flow_control(serdev, true);
    do {
// Reset the Bluetooth device
    gpiod_set_value_cansleep(lldev.enable_gpio, 0);
    msleep(5);
    gpiod_set_value_cansleep(lldev.enable_gpio, 1);
    mdelay(100);
    err = serdev_device_wait_for_cts(serdev, true, 200);
    if (err) {
    bt_dev_err(hu.hdev, "Failed to get CTS");
    return err;
    }
    err = download_firmware(lldev);
    if (!err)
    break;
// Toggle BT_EN and retry
    bt_dev_err(hu.hdev, "download firmware failed, retrying...");
    } while (retry--);
    if (err)
    return err;
// Set BD address if one was specified at probe
    if (!bacmp(&lldev.bdaddr, BDADDR_NONE)) {
// This means that there was an error getting the BD address
// during probe, so mark the device as having a bad address.
//
    hci_set_quirk(hu.hdev, HCI_QUIRK_INVALID_BDADDR);
    } else if (bacmp(&lldev.bdaddr, BDADDR_ANY)) {
    err = ll_set_bdaddr(hu.hdev, &lldev.bdaddr);
    if (err)
    hci_set_quirk(hu.hdev, HCI_QUIRK_INVALID_BDADDR);
    }
    if (lldev.broken_enhanced_setup)
    hci_set_quirk(hu.hdev,
    HCI_QUIRK_BROKEN_ENHANCED_SETUP_SYNC_CONN);
// Operational speed if any
    if (hu.oper_speed)
    speed = hu.oper_speed;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: hu->proto->oper_speed) -> else {
    else if (hu.proto.oper_speed)
    speed = hu.proto.oper_speed;
    else
    speed = 0;
    if (speed) {
    let mut speed_le: __le32 = cpu_to_le32(speed);
    struct sk_buff *skb;
    skb = __hci_cmd_sync(hu.hdev, HCI_VS_UPDATE_UART_HCI_BAUDRATE,
    sizeof(speed_le), &speed_le,
    HCI_INIT_TIMEOUT);
    if (!IS_ERR(skb)) {
    kfree_skb(skb);
    serdev_device_set_baudrate(serdev, speed);
    }
    }
    return 0;
    }
    static const struct hci_uart_proto llp;
#[no_mangle]
unsafe extern "C" fn hci_ti_probe(serdev: *mut serdev_device) -> c_int {
    static int hci_ti_probe(struct serdev_device *serdev)
    {
    struct hci_uart *hu;
    struct ll_device *lldev;
    struct nvmem_cell *bdaddr_cell;
    let mut max_speed: u32 = 3000000;
    lldev = devm_kzalloc(&serdev.dev, sizeof(struct ll_device), GFP_KERNEL);
    if (!lldev)
    return -ENOMEM;
    hu = &lldev.hu;
    serdev_device_set_drvdata(serdev, lldev);
    lldev.serdev = hu.serdev = serdev;
    lldev.enable_gpio = devm_gpiod_get_optional(&serdev.dev,
    "enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(lldev.enable_gpio))
    return PTR_ERR(lldev.enable_gpio);
    lldev.ext_clk = devm_clk_get(&serdev.dev, "ext_clock");
    if (IS_ERR(lldev.ext_clk) && PTR_ERR(lldev.ext_clk) != -ENOENT)
    return PTR_ERR(lldev.ext_clk);
    of_property_read_u32(serdev.dev.of_node, "max-speed", &max_speed);
    hci_uart_set_speeds(hu, 115200, max_speed);
    if (of_device_is_compatible(serdev.dev.of_node, "ti,wl1831-st") ||
    of_device_is_compatible(serdev.dev.of_node, "ti,wl1835-st") ||
    of_device_is_compatible(serdev.dev.of_node, "ti,wl1837-st"))
    lldev.broken_enhanced_setup = true;
// optional BD address from nvram
    bdaddr_cell = nvmem_cell_get(&serdev.dev, "bd-address");
    if (IS_ERR(bdaddr_cell)) {
    let mut err: c_int = PTR_ERR(bdaddr_cell);
    if (err == -EPROBE_DEFER)
    return err;
// ENOENT means there is no matching nvmem cell and ENOSYS
// means that nvmem is not enabled in the kernel configuration.
//
    if (err != -ENOENT && err != -ENOSYS) {
// If there was some other error, give userspace a
// chance to fix the problem instead of failing to load
// the driver. Using BDADDR_NONE as a flag that is
// tested later in the setup function.
//
    dev_warn(&serdev.dev,
    "Failed to get \"bd-address\" nvmem cell (%d)\n",
    err);
    bacpy(&lldev.bdaddr, BDADDR_NONE);
    }
    } else {
    bdaddr_t *bdaddr;
    size_t len;
    bdaddr = nvmem_cell_read(bdaddr_cell, &len);
    nvmem_cell_put(bdaddr_cell);
    if (IS_ERR(bdaddr)) {
    dev_err(&serdev.dev, "Failed to read nvmem bd-address\n");
    return PTR_ERR(bdaddr);
    }
    if (len != sizeof(bdaddr_t)) {
    dev_err(&serdev.dev, "Invalid nvmem bd-address length\n");
    kfree(bdaddr);
    return -EINVAL;
    }
// As per the device tree bindings, the value from nvmem is
// expected to be MSB first, but in the kernel it is expected
// that bdaddr_t is LSB first.
//
    baswap(&lldev.bdaddr, bdaddr);
    kfree(bdaddr);
    }
    return hci_uart_register_device(hu, &llp);
    }
#[no_mangle]
unsafe extern "C" fn hci_ti_remove(serdev: *mut serdev_device) {
    static void hci_ti_remove(struct serdev_device *serdev)
    {
    struct ll_device *lldev = serdev_device_get_drvdata(serdev);
    hci_uart_unregister_device(&lldev.hu);
    }
    static const struct of_device_id hci_ti_of_match[] = {
    { .compatible = "ti,cc2560" },
    { .compatible = "ti,wl1271-st" },
    { .compatible = "ti,wl1273-st" },
    { .compatible = "ti,wl1281-st" },
    { .compatible = "ti,wl1283-st" },
    { .compatible = "ti,wl1285-st" },
    { .compatible = "ti,wl1801-st" },
    { .compatible = "ti,wl1805-st" },
    { .compatible = "ti,wl1807-st" },
    { .compatible = "ti,wl1831-st" },
    { .compatible = "ti,wl1835-st" },
    { .compatible = "ti,wl1837-st" },
    {},
    };
    MODULE_DEVICE_TABLE(of, hci_ti_of_match);
    static struct serdev_device_driver hci_ti_drv = {
    .driver		= {
    .name	= "hci-ti",
    .of_match_table = hci_ti_of_match,
    },
    .probe	= hci_ti_probe,
    .remove	= hci_ti_remove,
    };

    static const struct hci_uart_proto llp = {
    .id		= HCI_UART_LL,
    .name		= "LL",
    .setup		= ll_setup,
    .open		= ll_open,
    .close		= ll_close,
    .recv		= ll_recv,
    .enqueue	= ll_enqueue,
    .dequeue	= ll_dequeue,
    .flush		= ll_flush,
    };
#[no_mangle]
pub unsafe extern "C" fn ll_init() -> int __init {
    int __init ll_init(void)
    {
    serdev_device_driver_register(&hci_ti_drv);
    return hci_uart_register_proto(&llp);
    }
#[no_mangle]
pub unsafe extern "C" fn ll_deinit() -> int __exit {
    int __exit ll_deinit(void)
    {
    serdev_device_driver_unregister(&hci_ti_drv);
    return hci_uart_unregister_proto(&llp);
    }
