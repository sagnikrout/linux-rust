//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/hci_ath.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Atheros Communication Bluetooth HCIATH3K UART protocol
//
// HCIATH3K (HCI Atheros AR300x Protocol) is a Atheros Communication's
// power management protocol extension to H4 to support AR300x Bluetooth Chip.
//
// Copyright (c) 2009-2010 Atheros Communications Inc.
//
// Acknowledgements:
// This file is based on hci_h4.c, which was written
// by Maxim Krasnyansky and Marcel Holtmann.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_struct {
    pub hu: *mut hci_uart,
    pub cur_sleep: c_uint,
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
    pub ctxtsw: work_struct,
}

pub const OP_WRITE_TAG: c_uint = 0x01;
pub const INDEX_BDADDR: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_vendor_cmd {
    pub opcode: __u8,
    pub index: __le16,
    pub len: __u8,
    pub data: [__u8; 251],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn ath_wakeup_ar3k(tty: *mut tty_struct) -> c_int {
    static int ath_wakeup_ar3k(struct tty_struct *tty)
    {
    pub tty->driver->ops->tiocmget(tty): int status =,
    if (status & TIOCM_CTS)
    pub status: return,
// Clear RTS first
    pub TIOCM_RTS): tty->driver->ops->tiocmset(tty, 0x00,,
// Set RTS, wake up board
    pub 0x00): tty->driver->ops->tiocmset(tty, TIOCM_RTS,,
    pub tty->driver->ops->tiocmget(tty): status =,
    pub status: return,
    }
#[no_mangle]
unsafe extern "C" fn ath_hci_uart_work(work: *mut work_struct) {
    static void ath_hci_uart_work(struct work_struct *work)
    {
    pub status: c_int,
    pub ath: *mut ath_struct,
    pub hu: *mut hci_uart,
    pub tty: *mut tty_struct,
    pub ctxtsw): ath = container_of(work, struct ath_struct,,
    pub ath->hu: hu =,
    pub hu->tty: tty =,
// verify and wake up controller
    if (ath.cur_sleep) {
    pub ath_wakeup_ar3k(tty): status =,
    if (!(status & TIOCM_CTS))
    }
// Ready to send Data
    pub &hu->tx_state): clear_bit(HCI_UART_SENDING,,
    }
#[no_mangle]
unsafe extern "C" fn ath_open(hu: *mut hci_uart) -> c_int {
    static int ath_open(struct hci_uart *hu)
    {
    pub ath: *mut ath_struct,
    pub hu): BT_DBG("hu %p",,
    if (!hci_uart_has_flow_control(hu))
    pub -EOPNOTSUPP: return,
    pub kzalloc_obj(*ath): *mut ath =,
    if (!ath)
    pub -ENOMEM: return,
    pub ath: hu->priv =,
    pub hu: ath->hu =,
    pub ath_hci_uart_work): INIT_WORK(&ath->ctxtsw,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath_close(hu: *mut hci_uart) -> c_int {
    static int ath_close(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut ath_ath =,
    pub hu): BT_DBG("hu %p",,
    pub NULL: hu->priv =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath_flush(hu: *mut hci_uart) -> c_int {
    static int ath_flush(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut ath_ath =,
    pub hu): BT_DBG("hu %p",,
    pub 0: return,
    }
    static int ath_vendor_cmd(struct hci_dev *hdev, uint8_t opcode, uint16_t index,
    const void *data, size_t dlen)
    {
    pub skb: *mut sk_buff,
    pub cmd: ath_vendor_cmd,
    if (dlen > sizeof(cmd.data))
    pub -EINVAL: return,
    pub opcode: cmd.opcode =,
    pub cpu_to_le16(index): cmd.index =,
    pub dlen: cmd.len =,
    pub dlen): memcpy(cmd.data, data,,
    pub HCI_INIT_TIMEOUT): skb = __hci_cmd_sync(hdev, 0xfc0b, dlen + 4, &cmd,,
    if (IS_ERR(skb))
    pub PTR_ERR(skb): return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ath_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int ath_set_bdaddr(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    return ath_vendor_cmd(hdev, OP_WRITE_TAG, INDEX_BDADDR, bdaddr,
    }
#[no_mangle]
unsafe extern "C" fn ath_setup(hu: *mut hci_uart) -> c_int {
    static int ath_setup(struct hci_uart *hu)
    {
    pub hu): BT_DBG("hu %p",,
    pub ath_set_bdaddr: hu->hdev->set_bdaddr =,
    pub 0: return,
    }
    static const struct h4_recv_pkt ath_recv_pkts[] = {
    { H4_RECV_ACL,   .recv = hci_recv_frame },
    { H4_RECV_SCO,   .recv = hci_recv_frame },
    { H4_RECV_EVENT, .recv = hci_recv_frame },
}

#[no_mangle]
unsafe extern "C" fn ath_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    static int ath_recv(struct hci_uart *hu, const void *data, int count)
    {
    struct ath_struct *ath = hu.priv;
    if (!ath)
    return -ENODEV;
    ath.rx_skb = h4_recv_buf(hu, ath.rx_skb, data, count,
    ath_recv_pkts, ARRAY_SIZE(ath_recv_pkts));
    if (IS_ERR(ath.rx_skb)) {
    let mut err: c_int = PTR_ERR(ath.rx_skb);
    bt_dev_err(hu.hdev, "Frame reassembly failed (%d)", err);
    ath.rx_skb = core::ptr::null_mut();
    return err;
    }
    return count;
    }
pub const HCI_OP_ATH_SLEEP: c_uint = 0xFC04;
#[no_mangle]
unsafe extern "C" fn ath_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    static int ath_enqueue(struct hci_uart *hu, struct sk_buff *skb)
    {
    struct ath_struct *ath = hu.priv;
    if (hci_skb_pkt_type(skb) == HCI_SCODATA_PKT) {
    kfree_skb(skb);
    return 0;
    }
// Update power management enable flag with parameters of
// HCI sleep enable vendor specific HCI command.
//
    if (hci_skb_pkt_type(skb) == HCI_COMMAND_PKT) {
    struct hci_command_hdr *hdr = (void *)skb.data;
    if (__le16_to_cpu(hdr.opcode) == HCI_OP_ATH_SLEEP)
    ath.cur_sleep = skb.data[HCI_COMMAND_HDR_SIZE];
    }
    BT_DBG("hu %p skb %p", hu, skb);
// Prepend skb with frame type
    memcpy(skb_push(skb, 1), &hci_skb_pkt_type(skb), 1);
    skb_queue_tail(&ath.txq, skb);
    set_bit(HCI_UART_SENDING, &hu.tx_state);
    schedule_work(&ath.ctxtsw);
    return 0;
    }
    static struct sk_buff *ath_dequeue(struct hci_uart *hu)
    {
    struct ath_struct *ath = hu.priv;
    return skb_dequeue(&ath.txq);
    }
    static const struct hci_uart_proto athp = {
    .id		= HCI_UART_ATH3K,
    .name		= "ATH3K",
    .manufacturer	= 69,
    .open		= ath_open,
    .close		= ath_close,
    .flush		= ath_flush,
    .setup		= ath_setup,
    .recv		= ath_recv,
    .enqueue	= ath_enqueue,
    .dequeue	= ath_dequeue,
    };
#[no_mangle]
pub unsafe extern "C" fn ath_init() -> int __init {
    int __init ath_init(void)
    {
    return hci_uart_register_proto(&athp);
    }
#[no_mangle]
pub unsafe extern "C" fn ath_deinit() -> int __exit {
    int __exit ath_deinit(void)
    {
    return hci_uart_unregister_proto(&athp);
    }
