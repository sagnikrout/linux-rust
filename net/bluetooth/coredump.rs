//! Automatically rewritten from C to Rust
//! Source: net/bluetooth/coredump.c
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
// Copyright (C) 2023 Google Corporation
//

    enum hci_devcoredump_pkt_type {
    HCI_DEVCOREDUMP_PKT_INIT,
    HCI_DEVCOREDUMP_PKT_SKB,
    HCI_DEVCOREDUMP_PKT_PATTERN,
    HCI_DEVCOREDUMP_PKT_COMPLETE,
    HCI_DEVCOREDUMP_PKT_ABORT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_devcoredump_skb_cb {
    pub pkt_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_devcoredump_skb_pattern {
    pub pattern: u8,
    pub len: u32,
    pub __packed: },

    bt_dev_dbg(hdev, \
    "Unexpected packet (%d) for state %s.", \
    hci_dmp_cb(skb).pkt_type, \
    hci_devcd_state_name(hdev.dump.state))
#[no_mangle]
unsafe extern "C" fn hci_devcd_update_hdr_state(buf: *mut c_char, size: usize, state: c_int) -> c_int {
    static int hci_devcd_update_hdr_state(char *buf, size_t size, int state)
    {
    pub 0: int len =,
    if (!buf)
    pub 0: return,
    pub state): len = scnprintf(buf, size, "Bluetooth devcoredump\nState: %d\n",,
    pub /: *mut *mut return len + 1; / scnprintf adds \0 at the end upon state rewrite,
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_update_state(hdev: *mut hci_dev, state: c_int) -> c_int {
    static int hci_devcd_update_state(struct hci_dev *hdev, int state)
    {
    bt_dev_dbg(hdev, "Updating devcoredump state from %s to %s.",
    hci_devcd_state_name(hdev.dump.state),
    pub state: hdev->dump.state =,
    return hci_devcd_update_hdr_state(hdev.dump.head,
    pub state): hdev->dump.alloc_size,,
    }
#[no_mangle]
unsafe extern "C" fn hci_devcd_mkheader(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int hci_devcd_mkheader(struct hci_dev *hdev, struct sk_buff *skb)
    {
    pub hdr: [c_char; 80],
    pub hdr_len: c_int,
    hdr_len = hci_devcd_update_hdr_state(hdr, sizeof(hdr),
    pub hdr_len): skb_put_data(skb, hdr,,
    if (hdev.dump.dmp_hdr)
    pub skb): hdev->dump.dmp_hdr(hdev,,
    pub strlen(HCI_DEVCD_HDR_END_MARKER)): skb_put_data(skb, HCI_DEVCD_HDR_END_MARKER,,
    pub skb->len: return,
    }
// Do not call with hci_dev_lock since this calls driver code.
#[no_mangle]
unsafe extern "C" fn hci_devcd_notify(hdev: *mut hci_dev, state: c_int) {
    static void hci_devcd_notify(struct hci_dev *hdev, int state)
    {
    if (hdev.dump.notify_change)
    pub state): hdev->dump.notify_change(hdev,,
    }
// Call with hci_dev_lock only.
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_reset(hdev: *mut hci_dev) {
    void hci_devcd_reset(struct hci_dev *hdev)
    {
    pub NULL: hdev->dump.head =,
    pub NULL: hdev->dump.tail =,
    pub 0: hdev->dump.alloc_size =,
    pub HCI_DEVCOREDUMP_IDLE): hci_devcd_update_state(hdev,,
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_free(hdev: *mut hci_dev) {
    static void hci_devcd_free(struct hci_dev *hdev)
    {
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_alloc(hdev: *mut hci_dev, size: u32) -> c_int {
    static int hci_devcd_alloc(struct hci_dev *hdev, u32 size)
    {
    pub vmalloc(size): hdev->dump.head =,
    if (!hdev.dump.head)
    pub -ENOMEM: return,
    pub size: hdev->dump.alloc_size =,
    pub hdev->dump.head: hdev->dump.tail =,
    pub size: hdev->dump.end = hdev->dump.head +,
    pub HCI_DEVCOREDUMP_IDLE): hci_devcd_update_state(hdev,,
    pub 0: return,
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_copy(hdev: *mut hci_dev, buf: *mut c_char, size: u32) -> bool {
    static bool hci_devcd_copy(struct hci_dev *hdev, char *buf, u32 size)
    {
    if (hdev.dump.tail + size > hdev.dump.end)
    pub false: return,
    pub size): memcpy(hdev->dump.tail, buf,,
    pub size: hdev->dump.tail +=,
    pub true: return,
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_memset(hdev: *mut hci_dev, pattern: u8, len: u32) -> bool {
    static bool hci_devcd_memset(struct hci_dev *hdev, u8 pattern, u32 len)
    {
    if (hdev.dump.tail + len > hdev.dump.end)
    pub false: return,
    pub len): memset(hdev->dump.tail, pattern,,
    pub len: hdev->dump.tail +=,
    pub true: return,
    }
// Call with hci_dev_lock only.
#[no_mangle]
unsafe extern "C" fn hci_devcd_prepare(hdev: *mut hci_dev, dump_size: u32) -> c_int {
    static int hci_devcd_prepare(struct hci_dev *hdev, u32 dump_size)
    {
    pub skb: *mut sk_buff,
    pub dump_hdr_size: c_int,
    pub 0: int err =,
    pub GFP_ATOMIC): skb = alloc_skb(HCI_DEVCD_HDR_SIZE_MAX,,
    if (!skb)
    pub -ENOMEM: return,
    pub skb): dump_hdr_size = hci_devcd_mkheader(hdev,,
    if (hci_devcd_alloc(hdev, dump_hdr_size + dump_size)) {
    pub -ENOMEM: err =,
    pub hdr_free: goto,
    }
// Insert the device header
    if (!hci_devcd_copy(hdev, skb.data, skb.len)) {
    pub header"): bt_dev_err(hdev, "Failed to insert,
    pub -ENOMEM: err =,
    pub hdr_free: goto,
    }
    hdr_free:
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn hci_devcd_handle_pkt_init(hdev: *mut hci_dev, skb: *mut sk_buff) {
    static void hci_devcd_handle_pkt_init(struct hci_dev *hdev, struct sk_buff *skb)
    {
    pub dump_size: u32,
    if (hdev.dump.state != HCI_DEVCOREDUMP_IDLE) {
    }
    if (skb.len != sizeof(dump_size)) {
    pub pkt"): bt_dev_dbg(hdev, "Invalid dump init,
    }
    pub 4)): dump_size = get_unaligned_le32(skb_pull_data(skb,,
    if (!dump_size) {
    pub pkt"): bt_dev_err(hdev, "Zero size dump init,
    }
    if (hci_devcd_prepare(hdev, dump_size)) {
    pub dump"): bt_dev_err(hdev, "Failed to prepare for,
    }
    pub HCI_DEVCOREDUMP_ACTIVE): hci_devcd_update_state(hdev,,
    queue_delayed_work(hdev.workqueue, &hdev.dump.dump_timeout,
    }
#[no_mangle]
unsafe extern "C" fn hci_devcd_handle_pkt_skb(hdev: *mut hci_dev, skb: *mut sk_buff) {
    static void hci_devcd_handle_pkt_skb(struct hci_dev *hdev, struct sk_buff *skb)
    {
    if (hdev.dump.state != HCI_DEVCOREDUMP_ACTIVE) {
    }
    if (!hci_devcd_copy(hdev, skb.data, skb.len))
    pub skb"): bt_dev_dbg(hdev, "Failed to insert,
    }
    static void hci_devcd_handle_pkt_pattern(struct hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub pattern: *mut hci_devcoredump_skb_pattern,
    if (hdev.dump.state != HCI_DEVCOREDUMP_ACTIVE) {
    }
    if (skb.len != sizeof(*pattern)) {
    pub skb"): bt_dev_dbg(hdev, "Invalid pattern,
    }
    pub sizeof(*pattern)): *mut pattern = skb_pull_data(skb,,
    if (!hci_devcd_memset(hdev, pattern.pattern, pattern.len))
    pub pattern"): bt_dev_dbg(hdev, "Failed to set,
    }
#[no_mangle]
unsafe extern "C" fn hci_devcd_dump(hdev: *mut hci_dev) {
    static void hci_devcd_dump(struct hci_dev *hdev)
    {
    pub skb: *mut sk_buff,
    pub size: u32,
    pub hci_devcd_state_name(hdev->dump.state)): bt_dev_dbg(hdev, "state %s",,
    pub hdev->dump.head: size = hdev->dump.tail -,
// Send a copy to monitor as a diagnostic packet
    pub GFP_ATOMIC): skb = bt_skb_alloc(size,,
    if (skb) {
    pub size): skb_put_data(skb, hdev->dump.head,,
    pub skb): hci_recv_diag(hdev,,
    }
// Emit a devcoredump with the available data
    pub GFP_KERNEL): dev_coredumpv(&hdev->dev, hdev->dump.head, size,,
    }
    static void hci_devcd_handle_pkt_complete(struct hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub dump_size: u32,
    if (hdev.dump.state != HCI_DEVCOREDUMP_ACTIVE) {
    }
    pub HCI_DEVCOREDUMP_DONE): hci_devcd_update_state(hdev,,
    pub hdev->dump.head: dump_size = hdev->dump.tail -,
    bt_dev_dbg(hdev, "complete with size %u (expect %zu)", dump_size,
    }
    static void hci_devcd_handle_pkt_abort(struct hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub dump_size: u32,
    if (hdev.dump.state != HCI_DEVCOREDUMP_ACTIVE) {
    }
    pub HCI_DEVCOREDUMP_ABORT): hci_devcd_update_state(hdev,,
    pub hdev->dump.head: dump_size = hdev->dump.tail -,
    bt_dev_dbg(hdev, "aborted with size %u (expect %zu)", dump_size,
    }
// Bluetooth devcoredump state machine.
//
// Devcoredump states:
//
// HCI_DEVCOREDUMP_IDLE: The default state.
//
// HCI_DEVCOREDUMP_ACTIVE: A devcoredump will be in this state once it has
// been initialized using hci_devcd_init(). Once active, the driver
// can append data using hci_devcd_append() or insert a pattern
// using hci_devcd_append_pattern().
//
// HCI_DEVCOREDUMP_DONE: Once the dump collection is complete, the drive
// can signal the completion using hci_devcd_complete(). A
// devcoredump is generated indicating the completion event and
// then the state machine is reset to the default state.
//
// HCI_DEVCOREDUMP_ABORT: The driver can cancel ongoing dump collection in
// case of any error using hci_devcd_abort(). A devcoredump is
// still generated with the available data indicating the abort
// event and then the state machine is reset to the default state.
//
// HCI_DEVCOREDUMP_TIMEOUT: A timeout timer for HCI_DEVCOREDUMP_TIMEOUT sec
// is started during devcoredump initialization. Once the timeout
// occurs, the driver is notified, a devcoredump is generated with
// the available data indicating the timeout event and then the
// state machine is reset to the default state.
//
// The driver must register using hci_devcd_register() before using the hci
// devcoredump APIs.
//
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_rx(work: *mut work_struct) {
    void hci_devcd_rx(struct work_struct *work)
    {
    pub dump.dump_rx): *mut *mut hci_dev hdev = container_of(work, hci_dev,,
    pub skb: *mut sk_buff,
    pub start_state: c_int,
    while ((skb = skb_dequeue(&hdev.dump.dump_q))) {
// Return if timeout occurs. The timeout handler function
// hci_devcd_timeout() will report the available dump data.
//
    if (hdev.dump.state == HCI_DEVCOREDUMP_TIMEOUT) {
    }
    pub hdev->dump.state: start_state =,
    switch (hci_dmp_cb(skb).pkt_type) {
    case HCI_DEVCOREDUMP_PKT_INIT:
    pub skb): hci_devcd_handle_pkt_init(hdev,,
    case HCI_DEVCOREDUMP_PKT_SKB:
    pub skb): hci_devcd_handle_pkt_skb(hdev,,
    case HCI_DEVCOREDUMP_PKT_PATTERN:
    pub skb): hci_devcd_handle_pkt_pattern(hdev,,
    case HCI_DEVCOREDUMP_PKT_COMPLETE:
    pub skb): hci_devcd_handle_pkt_complete(hdev,,
    case HCI_DEVCOREDUMP_PKT_ABORT:
    pub skb): hci_devcd_handle_pkt_abort(hdev,,
    default:
    bt_dev_dbg(hdev, "Unknown packet (%d) for state %s.",
    hci_dmp_cb(skb).pkt_type,
    }
// Notify the driver about any state changes before resetting
// the state machine
//
    if (start_state != hdev.dump.state)
    pub hdev->dump.state): hci_devcd_notify(hdev,,
// Reset the state machine if the devcoredump is complete
    if (hdev.dump.state == HCI_DEVCOREDUMP_DONE ||
    hdev.dump.state == HCI_DEVCOREDUMP_ABORT)
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_timeout(work: *mut work_struct) {
    void hci_devcd_timeout(struct work_struct *work)
    {
    struct hci_dev *hdev = container_of(work, struct hci_dev,
    pub dump_size: u32,
    pub HCI_DEVCOREDUMP_TIMEOUT): hci_devcd_notify(hdev,,
    pub HCI_DEVCOREDUMP_TIMEOUT): hci_devcd_update_state(hdev,,
    pub hdev->dump.head: dump_size = hdev->dump.tail -,
    bt_dev_dbg(hdev, "timeout with size %u (expect %zu)", dump_size,
    }
    int hci_devcd_register(struct hci_dev *hdev, coredump_t coredump,
    dmp_hdr_t dmp_hdr, notify_change_t notify_change)
    {
// Driver must implement coredump() and dmp_hdr() functions for
// bluetooth devcoredump. The coredump() should trigger a coredump
// event on the controller when the device's coredump sysfs entry is
// written to. The dmp_hdr() should create a dump header to identify
// the controller/fw/driver info.
//
    if (!coredump || !dmp_hdr)
    pub -EINVAL: return,
    pub coredump: hdev->dump.coredump =,
    pub dmp_hdr: hdev->dump.dmp_hdr =,
    pub notify_change: hdev->dump.notify_change =,
    pub true: hdev->dump.supported =,
    pub DEVCOREDUMP_TIMEOUT: hdev->dump.timeout =,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_enabled(hdev: *mut hci_dev) -> bool {
    static inline bool hci_devcd_enabled(struct hci_dev *hdev)
    {
    pub hdev->dump.supported: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_init(hdev: *mut hci_dev, dump_size: u32) -> c_int {
    int hci_devcd_init(struct hci_dev *hdev, u32 dump_size)
    {
    pub skb: *mut sk_buff,
    if (!hci_devcd_enabled(hdev))
    pub -EOPNOTSUPP: return,
    pub GFP_ATOMIC): skb = alloc_skb(sizeof(dump_size),,
    if (!skb)
    pub -ENOMEM: return,
    pub HCI_DEVCOREDUMP_PKT_INIT: hci_dmp_cb(skb)->pkt_type =,
    pub 4)): put_unaligned_le32(dump_size, skb_put(skb,,
    pub skb): skb_queue_tail(&hdev->dump.dump_q,,
    pub &hdev->dump.dump_rx): queue_work(hdev->workqueue,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_append(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    int hci_devcd_append(struct hci_dev *hdev, struct sk_buff *skb)
    {
    if (!skb)
    pub -ENOMEM: return,
    if (!hci_devcd_enabled(hdev)) {
    pub -EOPNOTSUPP: return,
    }
    pub HCI_DEVCOREDUMP_PKT_SKB: hci_dmp_cb(skb)->pkt_type =,
    pub skb): skb_queue_tail(&hdev->dump.dump_q,,
    pub &hdev->dump.dump_rx): queue_work(hdev->workqueue,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_append_pattern(hdev: *mut hci_dev, pattern: u8, len: u32) -> c_int {
    int hci_devcd_append_pattern(struct hci_dev *hdev, u8 pattern, u32 len)
    {
    pub p: hci_devcoredump_skb_pattern,
    pub skb: *mut sk_buff,
    if (!hci_devcd_enabled(hdev))
    pub -EOPNOTSUPP: return,
    pub GFP_ATOMIC): skb = alloc_skb(sizeof(p),,
    if (!skb)
    pub -ENOMEM: return,
    pub pattern: p.pattern =,
    pub len: p.len =,
    pub HCI_DEVCOREDUMP_PKT_PATTERN: hci_dmp_cb(skb)->pkt_type =,
    pub sizeof(p)): skb_put_data(skb, &p,,
    pub skb): skb_queue_tail(&hdev->dump.dump_q,,
    pub &hdev->dump.dump_rx): queue_work(hdev->workqueue,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_complete(hdev: *mut hci_dev) -> c_int {
    int hci_devcd_complete(struct hci_dev *hdev)
    {
    pub skb: *mut sk_buff,
    if (!hci_devcd_enabled(hdev))
    pub -EOPNOTSUPP: return,
    pub GFP_ATOMIC): skb = alloc_skb(0,,
    if (!skb)
    pub -ENOMEM: return,
    pub HCI_DEVCOREDUMP_PKT_COMPLETE: hci_dmp_cb(skb)->pkt_type =,
    pub skb): skb_queue_tail(&hdev->dump.dump_q,,
    pub &hdev->dump.dump_rx): queue_work(hdev->workqueue,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hci_devcd_abort(hdev: *mut hci_dev) -> c_int {
    int hci_devcd_abort(struct hci_dev *hdev)
    {
    pub skb: *mut sk_buff,
    if (!hci_devcd_enabled(hdev))
    pub -EOPNOTSUPP: return,
    pub GFP_ATOMIC): skb = alloc_skb(0,,
    if (!skb)
    pub -ENOMEM: return,
    pub HCI_DEVCOREDUMP_PKT_ABORT: hci_dmp_cb(skb)->pkt_type =,
    pub skb): skb_queue_tail(&hdev->dump.dump_q,,
    pub &hdev->dump.dump_rx): queue_work(hdev->workqueue,,
    pub 0: return,
    }
    const char *hci_devcd_state_name(enum devcoredump_state state)
    {
    pub "Unknown": *const *const char state_name =,
    switch (state) {
    case HCI_DEVCOREDUMP_IDLE:
    pub "IDLE": state_name =,
    case HCI_DEVCOREDUMP_ACTIVE:
    pub "ACTIVE": state_name =,
    case HCI_DEVCOREDUMP_DONE:
    pub "DONE": state_name =,
    case HCI_DEVCOREDUMP_ABORT:
    pub "ABORT": state_name =,
    case HCI_DEVCOREDUMP_TIMEOUT:
    pub "TIMEOUT": state_name =,
    default:
    }
    pub state_name: return,
    }
