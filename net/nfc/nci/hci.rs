//! Automatically rewritten from C to Rust
//! Source: net/nfc/nci/hci.c
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
// The NFC Controller Interface is the communication protocol between an
// NFC Controller (NFCC) and a Device Host (DH).
// This is the HCI over NCI implementation, as specified in the 10.2
// section of the NCI 1.1 specification.
//
// Copyright (C) 2014  STMicroelectronics SAS. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_data {
    pub conn_id: u8,
    pub pipe: u8,
    pub cmd: u8,
    pub data: *const u8,
    pub data_len: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_create_pipe_params {
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_create_pipe_resp {
    pub src_host: u8,
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
    pub pipe: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_delete_pipe_noti {
    pub pipe: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hci_all_pipe_cleared_noti {
    pub host: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hcp_message {
    pub /: *mut *mut u8 header; / type -cmd,evt,rsp- + instruction,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_hcp_packet {
    pub /: *mut *mut u8 header; / cbit+pipe,
    pub message: nci_hcp_message,
    pub __packed: },
pub const NCI_HCI_ANY_SET_PARAMETER: c_uint = 0x01;
pub const NCI_HCI_ANY_GET_PARAMETER: c_uint = 0x02;
pub const NCI_HCI_ANY_CLOSE_PIPE: c_uint = 0x04;
pub const NCI_HCI_ADM_CLEAR_ALL_PIPE: c_uint = 0x14;
pub const NCI_HFP_NO_CHAINING: c_uint = 0x80;
pub const NCI_NFCEE_ID_HCI: c_uint = 0x80;
pub const NCI_EVT_HOT_PLUG: c_uint = 0x03;
pub const NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY: c_uint = 0x01;
pub const NCI_HCI_ADM_CREATE_PIPE: c_uint = 0x10;
pub const NCI_HCI_ADM_DELETE_PIPE: c_uint = 0x11;
// HCP headers
pub const NCI_HCI_HCP_PACKET_HEADER_LEN: c_int = 1;
pub const NCI_HCI_HCP_MESSAGE_HEADER_LEN: c_int = 1;
pub const NCI_HCI_HCP_HEADER_LEN: c_int = 2;
// HCP types
pub const NCI_HCI_HCP_COMMAND: c_uint = 0x00;
pub const NCI_HCI_HCP_EVENT: c_uint = 0x01;
pub const NCI_HCI_HCP_RESPONSE: c_uint = 0x02;
pub const NCI_HCI_ADM_NOTIFY_PIPE_CREATED: c_uint = 0x12;
pub const NCI_HCI_ADM_NOTIFY_PIPE_DELETED: c_uint = 0x13;
pub const NCI_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED: c_uint = 0x15;
pub const NCI_HCI_FRAGMENT: c_uint = 0x7f;

    ((instr) & 0x3f))

#[no_mangle]
unsafe extern "C" fn nci_hci_result_to_errno(result: u8) -> c_int {
    static int nci_hci_result_to_errno(u8 result)
    {
    switch (result) {
    case NCI_HCI_ANY_OK:
    pub 0: return,
    case NCI_HCI_ANY_E_REG_PAR_UNKNOWN:
    pub -EOPNOTSUPP: return,
    case NCI_HCI_ANY_E_TIMEOUT:
    pub -ETIME: return,
    default:
    pub -1: return,
    }
    }
// HCI core
#[no_mangle]
unsafe extern "C" fn nci_hci_reset_pipes(hdev: *mut nci_hci_dev) {
    static void nci_hci_reset_pipes(struct nci_hci_dev *hdev)
    {
    pub i: c_int,
    pub {: for (i = 0; i < NCI_HCI_MAX_PIPES; i++),
    pub NCI_HCI_INVALID_GATE: hdev->pipes[i].gate =,
    pub NCI_HCI_INVALID_HOST: hdev->pipes[i].host =,
    }
    pub sizeof(hdev->gate2pipe)): memset(hdev->gate2pipe, NCI_HCI_INVALID_PIPE,,
    }
#[no_mangle]
unsafe extern "C" fn nci_hci_reset_pipes_per_host(ndev: *mut nci_dev, host: u8) {
    static void nci_hci_reset_pipes_per_host(struct nci_dev *ndev, u8 host)
    {
    pub i: c_int,
    pub {: for (i = 0; i < NCI_HCI_MAX_PIPES; i++),
    if (ndev.hci_dev.pipes[i].host == host) {
    pub NCI_HCI_INVALID_GATE: ndev->hci_dev->pipes[i].gate =,
    pub NCI_HCI_INVALID_HOST: ndev->hci_dev->pipes[i].host =,
    }
    }
    }
// Fragment HCI data over NCI packet.
// NFC Forum NCI 10.2.2 Data Exchange:
// The payload of the Data Packets sent on the Logical Connection SHALL be
// valid HCP packets, as defined within [ETSI_102622]. Each Data Packet SHALL
// contain a single HCP packet. NCI Segmentation and Reassembly SHALL NOT be
// applied to Data Messages in either direction. The HCI fragmentation mechanism
// is used if required.
//
    static int nci_hci_send_data(struct nci_dev *ndev, u8 pipe,
    const u8 data_type, const u8 *data,
    size_t data_len)
    {
    pub conn_info: *const nci_conn_info,
    pub skb: *mut sk_buff,
    pub r: int len, i,,
    pub pipe: u8 cb =,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub 0: i =,
    skb = nci_skb_alloc(ndev, conn_info.max_pkt_payload_len +
    pub GFP_ATOMIC): NCI_DATA_HDR_SIZE,,
    if (!skb)
    pub -ENOMEM: return,
    pub 2): skb_reserve(skb, NCI_DATA_HDR_SIZE +,
// (u8 *)skb_push(skb, 1) = data_type;
    do {
// If last packet add NCI_HFP_NO_CHAINING
    if (i + conn_info.max_pkt_payload_len -
    (skb.len + 1) >= data_len) {
    pub NCI_HFP_NO_CHAINING: cb |=,
    pub i: len = data_len -,
    } else {
    pub 1: len = conn_info->max_pkt_payload_len - skb->len -,
    }
// (u8 *)skb_push(skb, 1) = cb;
    if (len > 0)
    pub len): skb_put_data(skb, data + i,,
    pub skb): r = nci_send_data(ndev, conn_info->conn_id,,
    if (r < 0)
    pub r: return,
    pub len: i +=,
    if (i < data_len) {
    skb = nci_skb_alloc(ndev,
    conn_info.max_pkt_payload_len +
    pub GFP_ATOMIC): NCI_DATA_HDR_SIZE,,
    if (!skb)
    pub -ENOMEM: return,
    pub 1): skb_reserve(skb, NCI_DATA_HDR_SIZE +,
    }
    pub data_len): } while (i <,
    pub i: return,
    }
#[no_mangle]
unsafe extern "C" fn nci_hci_send_data_req(ndev: *mut nci_dev, opt: *const c_void) {
    static void nci_hci_send_data_req(struct nci_dev *ndev, const void *opt)
    {
    pub opt: *const *const nci_data data =,
    nci_hci_send_data(ndev, data.pipe, data.cmd,
    pub data->data_len): data->data,,
    }
    int nci_hci_send_event(struct nci_dev *ndev, u8 gate, u8 event,
    const u8 *param, size_t param_len)
    {
    pub ndev->hci_dev->gate2pipe[gate]: u8 pipe =,
    if (pipe == NCI_HCI_INVALID_PIPE)
    pub -EADDRNOTAVAIL: return,
    return nci_hci_send_data(ndev, pipe,
    NCI_HCP_HEADER(NCI_HCI_HCP_EVENT, event),
    pub param_len): param,,
    }
    int nci_hci_send_cmd(struct nci_dev *ndev, u8 gate, u8 cmd,
    const u8 *param, size_t param_len,
    struct sk_buff **skb)
    {
    pub message: *const nci_hcp_message,
    pub conn_info: *const nci_conn_info,
    pub data: nci_data,
    pub r: c_int,
    pub ndev->hci_dev->gate2pipe[gate]: u8 pipe =,
    if (pipe == NCI_HCI_INVALID_PIPE)
    pub -EADDRNOTAVAIL: return,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub conn_info->conn_id: data.conn_id =,
    pub pipe: data.pipe =,
    pub cmd): data.cmd = NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND,,
    pub param: data.data =,
    pub param_len: data.data_len =,
    r = nci_request(ndev, nci_hci_send_data_req, &data,
    if (r == NCI_STATUS_OK) {
    pub )conn_info->rx_skb->data: *mut message = (struct nci_hcp_message,
    r = nci_hci_result_to_errno(
    pub NCI_HCI_HCP_MESSAGE_HEADER_LEN): skb_pull(conn_info->rx_skb,,
    if (!r && skb)
// skb = conn_info->rx_skb;
    }
    pub r: return,
    }
#[no_mangle]
pub unsafe extern "C" fn nci_hci_clear_all_pipes(ndev: *mut nci_dev) -> c_int {
    int nci_hci_clear_all_pipes(struct nci_dev *ndev)
    {
    pub r: c_int,
    r = nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE,
    pub NULL): NCI_HCI_ADM_CLEAR_ALL_PIPE, NULL, 0,,
    if (r < 0)
    pub r: return,
    pub r: return,
    }
    static void nci_hci_event_received(struct nci_dev *ndev, u8 pipe,
    u8 event, struct sk_buff *skb)
    {
    if (ndev.ops.hci_event_received)
    pub skb): ndev->ops->hci_event_received(ndev, pipe, event,,
    }
    static void nci_hci_cmd_received(struct nci_dev *ndev, u8 pipe,
    u8 cmd, struct sk_buff *skb)
    {
    pub ndev->hci_dev->pipes[pipe].gate: u8 gate =,
    pub ~NCI_HCI_FRAGMENT: u8 status = NCI_HCI_ANY_OK |,
    pub new_pipe: u8 dest_gate,,
    pub create_info: *mut nci_hci_create_pipe_resp,
    pub delete_info: *mut nci_hci_delete_pipe_noti,
    pub cleared_info: *mut nci_hci_all_pipe_cleared_noti,
    pub cmd): pr_debug("from gate %x pipe %x cmd %x\n", gate, pipe,,
    switch (cmd) {
    case NCI_HCI_ADM_NOTIFY_PIPE_CREATED:
    if (skb.len != 5) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
    pub )skb->data: *mut create_info = (struct nci_hci_create_pipe_resp,
    pub create_info->dest_gate: dest_gate =,
    pub create_info->pipe: new_pipe =,
    if (new_pipe >= NCI_HCI_MAX_PIPES) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
// Save the new created pipe and bind with local gate,
// the description for skb->data[3] is destination gate id
// but since we received this cmd from host controller, we
// are the destination and it is our local gate
//
    pub new_pipe: ndev->hci_dev->gate2pipe[dest_gate] =,
    pub dest_gate: ndev->hci_dev->pipes[new_pipe].gate =,
    ndev.hci_dev.pipes[new_pipe].host =
    case NCI_HCI_ANY_OPEN_PIPE:
// If the pipe is not created report an error
    if (gate == NCI_HCI_INVALID_GATE) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
    case NCI_HCI_ADM_NOTIFY_PIPE_DELETED:
    if (skb.len != 1) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
    pub )skb->data: *mut delete_info = (struct nci_hci_delete_pipe_noti,
    if (delete_info.pipe >= NCI_HCI_MAX_PIPES) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
    ndev.hci_dev.pipes[delete_info.pipe].gate =
    ndev.hci_dev.pipes[delete_info.pipe].host =
    case NCI_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED:
    if (skb.len != 1) {
    pub NCI_HCI_ANY_E_NOK: status =,
    pub exit: goto,
    }
    cleared_info =
    pub )skb->data: *mut (struct nci_hci_all_pipe_cleared_noti,
    pub cleared_info->host): nci_hci_reset_pipes_per_host(ndev,,
    default:
    pub gate): pr_debug("Discarded unknown cmd %x to gate %x\n", cmd,,
    }
    if (ndev.ops.hci_cmd_received)
    pub skb): ndev->ops->hci_cmd_received(ndev, pipe, cmd,,
    exit:
    pub 0): nci_hci_send_data(ndev, pipe, status, NULL,,
    }
    static void nci_hci_resp_received(struct nci_dev *ndev, u8 pipe,
    struct sk_buff *skb)
    {
    pub conn_info: *mut nci_conn_info,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub exit: goto,
    pub skb: conn_info->rx_skb =,
    exit:
    pub NCI_STATUS_OK): nci_req_complete(ndev,,
    }
// Receive hcp message for pipe, with type and cmd.
// skb contains optional message data only.
//
    static void nci_hci_hcp_message_rx(struct nci_dev *ndev, u8 pipe,
    u8 type, u8 instruction, struct sk_buff *skb)
    {
    switch (type) {
    case NCI_HCI_HCP_RESPONSE:
    pub skb): nci_hci_resp_received(ndev, pipe,,
    case NCI_HCI_HCP_COMMAND:
    pub skb): nci_hci_cmd_received(ndev, pipe, instruction,,
    case NCI_HCI_HCP_EVENT:
    pub skb): nci_hci_event_received(ndev, pipe, instruction,,
    default:
    pr_err("UNKNOWN MSG Type %d, instruction=%d\n",
    pub instruction): type,,
    }
    pub NCI_STATUS_OK): nci_req_complete(ndev,,
    }
#[no_mangle]
unsafe extern "C" fn nci_hci_msg_rx_work(work: *mut work_struct) {
    static void nci_hci_msg_rx_work(struct work_struct *work)
    {
    struct nci_hci_dev *hdev =
    pub msg_rx_work): container_of(work, struct nci_hci_dev,,
    pub skb: *mut sk_buff,
    pub message: *const nci_hcp_message,
    pub instruction: u8 pipe, type,,
    pub {: for (; (skb = skb_dequeue(&hdev->msg_rx_queue)); kcov_remote_stop()),
    pub NCI_HCP_MSG_GET_PIPE(skb->data[0]): pipe =,
    pub NCI_HCI_HCP_PACKET_HEADER_LEN): skb_pull(skb,,
    pub )skb->data: *mut message = (struct nci_hcp_message,
    pub NCI_HCP_MSG_GET_TYPE(message->header): type =,
    pub NCI_HCP_MSG_GET_CMD(message->header): instruction =,
    pub NCI_HCI_HCP_MESSAGE_HEADER_LEN): skb_pull(skb,,
    nci_hci_hcp_message_rx(hdev.ndev, pipe,
    pub skb): type, instruction,,
    }
    }
    void nci_hci_data_received_cb(void *context,
    struct sk_buff *skb, int err)
    {
    pub )context: *mut *mut nci_dev ndev = (nci_dev,
    pub packet: *mut nci_hcp_packet,
    pub type: u8 pipe,,
    pub hcp_skb: *mut sk_buff,
    pub frag_skb: *mut sk_buff,
    pub msg_len: c_int,
    if (err) {
    pub err): nci_req_complete(ndev,,
    }
    if (!pskb_may_pull(skb, NCI_HCI_HCP_PACKET_HEADER_LEN)) {
    }
    pub )skb->data: *mut packet = (struct nci_hcp_packet,
    if ((packet.header & ~NCI_HCI_FRAGMENT) == 0) {
    pub skb): skb_queue_tail(&ndev->hci_dev->rx_hcp_frags,,
    }
// it's the last fragment. Does it need re-aggregation?
    if (skb_queue_len(&ndev.hci_dev.rx_hcp_frags)) {
    pub NCI_HCP_MSG_GET_PIPE(packet->header): pipe =,
    pub skb): skb_queue_tail(&ndev->hci_dev->rx_hcp_frags,,
    pub 0: msg_len =,
    skb_queue_walk(&ndev.hci_dev.rx_hcp_frags, frag_skb) {
    msg_len += (frag_skb.len -
    }
    hcp_skb = nfc_alloc_recv_skb(NCI_HCI_HCP_PACKET_HEADER_LEN +
    pub GFP_KERNEL): msg_len,,
    if (!hcp_skb) {
    pub -ENOMEM): nci_req_complete(ndev,,
    }
    pub pipe): skb_put_u8(hcp_skb,,
    skb_queue_walk(&ndev.hci_dev.rx_hcp_frags, frag_skb) {
    pub NCI_HCI_HCP_PACKET_HEADER_LEN: msg_len = frag_skb->len -,
    skb_put_data(hcp_skb,
    frag_skb.data + NCI_HCI_HCP_PACKET_HEADER_LEN,
    }
    } else {
    pub NCI_HCI_FRAGMENT: packet->header &=,
    pub skb: hcp_skb =,
    }
// if this is a response, dispatch immediately to
// unblock waiting cmd context. Otherwise, enqueue to dispatch
// in separate context where handler can also execute command.
//
    if (!pskb_may_pull(hcp_skb, NCI_HCI_HCP_HEADER_LEN)) {
    }
    pub )hcp_skb->data: *mut packet = (struct nci_hcp_packet,
    pub NCI_HCP_MSG_GET_TYPE(packet->message.header): type =,
    if (type == NCI_HCI_HCP_RESPONSE) {
    pub NCI_HCP_MSG_GET_PIPE(packet->header): pipe =,
    pub NCI_HCI_HCP_PACKET_HEADER_LEN): skb_pull(hcp_skb,,
    nci_hci_hcp_message_rx(ndev, pipe, type,
    pub hcp_skb): NCI_STATUS_OK,,
    } else {
    pub hcp_skb): skb_queue_tail(&ndev->hci_dev->msg_rx_queue,,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn nci_hci_open_pipe(ndev: *mut nci_dev, pipe: u8) -> c_int {
    int nci_hci_open_pipe(struct nci_dev *ndev, u8 pipe)
    {
    pub data: nci_data,
    pub conn_info: *const nci_conn_info,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub conn_info->conn_id: data.conn_id =,
    pub pipe: data.pipe =,
    data.cmd = NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND,
    pub NULL: data.data =,
    pub 0: data.data_len =,
    return nci_request(ndev, nci_hci_send_data_req, &data,
    }
    static u8 nci_hci_create_pipe(struct nci_dev *ndev, u8 dest_host,
    u8 dest_gate, int *result)
    {
    pub pipe: u8,
    pub skb: *mut sk_buff,
    pub params: nci_hci_create_pipe_params,
    pub resp: *const nci_hci_create_pipe_resp,
    pub dest_gate): pr_debug("gate=%d\n",,
    pub NCI_HCI_ADMIN_GATE: params.src_gate =,
    pub dest_host: params.dest_host =,
    pub dest_gate: params.dest_gate =,
// result = nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE,
    NCI_HCI_ADM_CREATE_PIPE,
    pub &skb): *mut *mut (u8 )&params, sizeof(params),,
    if (*result < 0)
    pub NCI_HCI_INVALID_PIPE: return,
    pub )skb->data: *mut resp = (struct nci_hci_create_pipe_resp,
    pub resp->pipe: pipe =,
    pub pipe): pr_debug("pipe created=%d\n",,
    if (pipe >= NCI_HCI_MAX_PIPES)
    pub NCI_HCI_INVALID_PIPE: pipe =,
    pub pipe: return,
    }
#[no_mangle]
unsafe extern "C" fn nci_hci_delete_pipe(ndev: *mut nci_dev, pipe: u8) -> c_int {
    static int nci_hci_delete_pipe(struct nci_dev *ndev, u8 pipe)
    {
    return nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE,
    pub NULL): NCI_HCI_ADM_DELETE_PIPE, &pipe, 1,,
    }
    int nci_hci_set_param(struct nci_dev *ndev, u8 gate, u8 idx,
    const u8 *param, size_t param_len)
    {
    pub message: *const nci_hcp_message,
    pub conn_info: *const nci_conn_info,
    pub data: nci_data,
    pub r: c_int,
    pub tmp: *mut u8,
    pub ndev->hci_dev->gate2pipe[gate]: u8 pipe =,
    pub gate): pr_debug("idx=%d to gate %d\n", idx,,
    if (pipe == NCI_HCI_INVALID_PIPE)
    pub -EADDRNOTAVAIL: return,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub GFP_KERNEL): tmp = kmalloc(1 + param_len,,
    if (!tmp)
    pub -ENOMEM: return,
// tmp = idx;
    pub param_len): memcpy(tmp + 1, param,,
    pub conn_info->conn_id: data.conn_id =,
    pub pipe: data.pipe =,
    data.cmd = NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND,
    pub tmp: data.data =,
    pub 1: data.data_len = param_len +,
    r = nci_request(ndev, nci_hci_send_data_req, &data,
    if (r == NCI_STATUS_OK) {
    pub )conn_info->rx_skb->data: *mut message = (struct nci_hcp_message,
    r = nci_hci_result_to_errno(
    pub NCI_HCI_HCP_MESSAGE_HEADER_LEN): skb_pull(conn_info->rx_skb,,
    }
    pub r: return,
    }
    int nci_hci_get_param(struct nci_dev *ndev, u8 gate, u8 idx,
    struct sk_buff **skb)
    {
    pub message: *const nci_hcp_message,
    pub conn_info: *const nci_conn_info,
    pub data: nci_data,
    pub r: c_int,
    pub ndev->hci_dev->gate2pipe[gate]: u8 pipe =,
    pub gate): pr_debug("idx=%d to gate %d\n", idx,,
    if (pipe == NCI_HCI_INVALID_PIPE)
    pub -EADDRNOTAVAIL: return,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub conn_info->conn_id: data.conn_id =,
    pub pipe: data.pipe =,
    data.cmd = NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND,
    pub &idx: data.data =,
    pub 1: data.data_len =,
    r = nci_request(ndev, nci_hci_send_data_req, &data,
    if (r == NCI_STATUS_OK) {
    pub )conn_info->rx_skb->data: *mut message = (struct nci_hcp_message,
    r = nci_hci_result_to_errno(
    pub NCI_HCI_HCP_MESSAGE_HEADER_LEN): skb_pull(conn_info->rx_skb,,
    if (!r && skb)
// skb = conn_info->rx_skb;
    }
    pub r: return,
    }
    int nci_hci_connect_gate(struct nci_dev *ndev,
    u8 dest_host, u8 dest_gate, u8 pipe)
    {
    pub false: bool pipe_created =,
    pub r: c_int,
    if (pipe == NCI_HCI_DO_NOT_OPEN_PIPE)
    pub 0: return,
    if (ndev.hci_dev.gate2pipe[dest_gate] != NCI_HCI_INVALID_PIPE)
    pub -EADDRINUSE: return,
    if (pipe != NCI_HCI_INVALID_PIPE)
    pub open_pipe: goto,
    switch (dest_gate) {
    case NCI_HCI_LINK_MGMT_GATE:
    pub NCI_HCI_LINK_MGMT_PIPE: pipe =,
    case NCI_HCI_ADMIN_GATE:
    pub NCI_HCI_ADMIN_PIPE: pipe =,
    default:
    pub &r): pipe = nci_hci_create_pipe(ndev, dest_host, dest_gate,,
    if (pipe == NCI_HCI_INVALID_PIPE)
    pub r: return,
    pub true: pipe_created =,
    }
    open_pipe:
    pub pipe): r = nci_hci_open_pipe(ndev,,
    if (r < 0) {
    if (pipe_created) {
    if (nci_hci_delete_pipe(ndev, pipe) < 0) {
// TODO: Cannot clean by deleting pipe...
// -> inconsistent state
//
    }
    }
    pub r: return,
    }
    pub dest_gate: ndev->hci_dev->pipes[pipe].gate =,
    pub dest_host: ndev->hci_dev->pipes[pipe].host =,
    pub pipe: ndev->hci_dev->gate2pipe[dest_gate] =,
    pub 0: return,
    }
    static int nci_hci_dev_connect_gates(struct nci_dev *ndev,
    u8 gate_count,
    const struct nci_hci_gate *gates)
    {
    pub r: c_int,
    while (gate_count--) {
    r = nci_hci_connect_gate(ndev, gates.dest_host,
    pub gates->pipe): gates->gate,,
    if (r < 0)
    pub r: return,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn nci_hci_dev_session_init(ndev: *mut nci_dev) -> c_int {
    int nci_hci_dev_session_init(struct nci_dev *ndev)
    {
    pub conn_info: *mut nci_conn_info,
    pub skb: *mut sk_buff,
    pub r: c_int,
    pub 0: ndev->hci_dev->count_pipes =,
    pub 0: ndev->hci_dev->expected_pipes =,
    pub ndev->hci_dev->conn_info: conn_info =,
    if (!conn_info)
    pub -EPROTO: return,
    pub nci_hci_data_received_cb: conn_info->data_exchange_cb =,
    pub ndev: conn_info->data_exchange_cb_context =,
    if (ndev.hci_dev.init_data.gates[0].gate != NCI_HCI_ADMIN_GATE)
    pub -EPROTO: return,
    r = nci_hci_connect_gate(ndev,
    ndev.hci_dev.init_data.gates[0].dest_host,
    ndev.hci_dev.init_data.gates[0].gate,
    if (r < 0)
    pub r: return,
    r = nci_hci_get_param(ndev, NCI_HCI_ADMIN_GATE,
    pub &skb): NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY,,
    if (r < 0)
    pub r: return,
    if (skb.len &&
    skb.len == strlen(ndev.hci_dev.init_data.session_id) &&
    !memcmp(ndev.hci_dev.init_data.session_id, skb.data, skb.len) &&
    ndev.ops.hci_load_session) {
// Restore gate<->pipe table from some proprietary location.
    pub ndev->ops->hci_load_session(ndev): r =,
    } else {
    pub nci_hci_clear_all_pipes(ndev): r =,
    if (r < 0)
    pub exit: goto,
    r = nci_hci_dev_connect_gates(ndev,
    ndev.hci_dev.init_data.gate_count,
    if (r < 0)
    pub exit: goto,
    r = nci_hci_set_param(ndev, NCI_HCI_ADMIN_GATE,
    NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY,
    ndev.hci_dev.init_data.session_id,
    }
    exit:
    pub r: return,
    }
    struct nci_hci_dev *nci_hci_allocate(struct nci_dev *ndev)
    {
    pub hdev: *mut nci_hci_dev,
    pub kzalloc_obj(*hdev): *mut hdev =,
    if (!hdev)
    pub NULL: return,
    pub nci_hci_msg_rx_work): INIT_WORK(&hdev->msg_rx_work,,
    pub ndev: hdev->ndev =,
    pub hdev: return,
    }
#[no_mangle]
pub unsafe extern "C" fn nci_hci_deallocate(ndev: *mut nci_dev) {
    void nci_hci_deallocate(struct nci_dev *ndev)
    {
    }
