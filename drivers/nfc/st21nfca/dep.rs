//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/st21nfca/dep.c
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
// Copyright (C) 2014  STMicroelectronics SAS. All rights reserved.
//

pub const ST21NFCA_NFCIP1_INITIATOR: c_uint = 0x00;
pub const ST21NFCA_NFCIP1_REQ: c_uint = 0xd4;
pub const ST21NFCA_NFCIP1_RES: c_uint = 0xd5;
pub const ST21NFCA_NFCIP1_ATR_REQ: c_uint = 0x00;
pub const ST21NFCA_NFCIP1_ATR_RES: c_uint = 0x01;
pub const ST21NFCA_NFCIP1_PSL_REQ: c_uint = 0x04;
pub const ST21NFCA_NFCIP1_PSL_RES: c_uint = 0x05;
pub const ST21NFCA_NFCIP1_DEP_REQ: c_uint = 0x06;
pub const ST21NFCA_NFCIP1_DEP_RES: c_uint = 0x07;

    ((pfb) & ST21NFCA_NFC_DEP_PFB_TIMEOUT_BIT)

pub const ST21NFCA_NFC_DEP_PFB_TIMEOUT_BIT: c_uint = 0x10;

    ((pfb) & ST21NFCA_NFC_DEP_PFB_TIMEOUT_BIT)
pub const ST21NFCA_NFC_DEP_PFB_I_PDU: c_uint = 0x00;
pub const ST21NFCA_NFC_DEP_PFB_ACK_NACK_PDU: c_uint = 0x40;
pub const ST21NFCA_NFC_DEP_PFB_SUPERVISOR_PDU: c_uint = 0x80;
pub const ST21NFCA_ATR_REQ_MIN_SIZE: c_int = 17;
pub const ST21NFCA_ATR_REQ_MAX_SIZE: c_int = 65;
pub const ST21NFCA_LR_BITS_PAYLOAD_SIZE_254B: c_uint = 0x30;
pub const ST21NFCA_GB_BIT: c_uint = 0x02;
pub const ST21NFCA_EVT_SEND_DATA: c_uint = 0x10;
pub const ST21NFCA_EVT_FIELD_ON: c_uint = 0x11;
pub const ST21NFCA_EVT_CARD_DEACTIVATED: c_uint = 0x12;
pub const ST21NFCA_EVT_CARD_ACTIVATED: c_uint = 0x13;
pub const ST21NFCA_EVT_FIELD_OFF: c_uint = 0x14;
pub const ST21NFCA_EVT_CARD_F_BITRATE: c_uint = 0x16;
pub const ST21NFCA_EVT_READER_F_BITRATE: c_uint = 0x13;

pub const ST21NFCA_CARD_BITRATE_212: c_uint = 0x01;
pub const ST21NFCA_CARD_BITRATE_424: c_uint = 0x02;
pub const ST21NFCA_DEFAULT_TIMEOUT: c_uint = 0x0a;

    __LINE__, req)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_atr_req {
    pub length: u8,
    pub cmd0: u8,
    pub cmd1: u8,
    pub nfcid3: [u8; NFC_NFCID3_MAXSIZE],
    pub did: u8,
    pub bsi: u8,
    pub bri: u8,
    pub ppi: u8,
    pub gbi: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_atr_res {
    pub length: u8,
    pub cmd0: u8,
    pub cmd1: u8,
    pub nfcid3: [u8; NFC_NFCID3_MAXSIZE],
    pub did: u8,
    pub bsi: u8,
    pub bri: u8,
    pub to: u8,
    pub ppi: u8,
    pub gbi: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_psl_req {
    pub length: u8,
    pub cmd0: u8,
    pub cmd1: u8,
    pub did: u8,
    pub brs: u8,
    pub fsl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_psl_res {
    pub length: u8,
    pub cmd0: u8,
    pub cmd1: u8,
    pub did: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_dep_req_res {
    pub length: u8,
    pub cmd0: u8,
    pub cmd1: u8,
    pub pfb: u8,
    pub did: u8,
    pub nad: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn st21nfca_tx_work(work: *mut work_struct) {
    static void st21nfca_tx_work(struct work_struct *work)
    {
    struct st21nfca_hci_info *info = container_of(work,
    struct st21nfca_hci_info,
    pub dev: *mut nfc_dev,
    pub skb: *mut sk_buff,
    pub info->hdev->ndev: dev =,
    pub info->dep_info.tx_pending: skb =,
    nfc_hci_send_cmd_async(info.hdev, ST21NFCA_RF_READER_F_GATE,
    ST21NFCA_WR_XCHG_DATA, skb.data, skb.len,
    pub info): info->async_cb,,
    }
    static void st21nfca_im_send_pdu(struct st21nfca_hci_info *info,
    struct sk_buff *skb)
    {
    pub skb: info->dep_info.tx_pending =,
    }
    static int st21nfca_tm_send_atr_res(struct nfc_hci_dev *hdev,
    struct st21nfca_atr_req *atr_req)
    {
    pub atr_res: *mut st21nfca_atr_res,
    pub skb: *mut sk_buff,
    pub gb_len: usize,
    pub r: c_int,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub st21nfca_atr_req): gb_len = atr_req->length - sizeof(struct,
    pub GFP_KERNEL): skb = alloc_skb(atr_req->length + 1,,
    if (!skb)
    pub -ENOMEM: return,
    pub st21nfca_atr_res)): skb_put(skb, sizeof(struct,
    pub )skb->data: *mut atr_res = (struct st21nfca_atr_res,
    pub st21nfca_atr_res)): memset(atr_res, 0, sizeof(struct,
    pub 1: atr_res->length = atr_req->length +,
    pub ST21NFCA_NFCIP1_RES: atr_res->cmd0 =,
    pub ST21NFCA_NFCIP1_ATR_RES: atr_res->cmd1 =,
    pub 6): memcpy(atr_res->nfcid3, atr_req->nfcid3,,
    pub 0x00: atr_res->bsi =,
    pub 0x00: atr_res->bri =,
    pub ST21NFCA_DEFAULT_TIMEOUT: atr_res->to =,
    pub ST21NFCA_LR_BITS_PAYLOAD_SIZE_254B: atr_res->ppi =,
    if (gb_len) {
    pub gb_len): skb_put(skb,,
    pub ST21NFCA_GB_BIT: atr_res->ppi |=,
    pub gb_len): memcpy(atr_res->gbi, atr_req->gbi,,
    r = nfc_set_remote_general_bytes(hdev.ndev, atr_res.gbi,
    if (r < 0) {
    pub r: return,
    }
    }
    pub 0: info->dep_info.curr_nfc_dep_pni =,
    r = nfc_hci_send_event(hdev, ST21NFCA_RF_CARD_F_GATE,
    pub skb->len): ST21NFCA_EVT_SEND_DATA, skb->data,,
    pub r: return,
    }
    static int st21nfca_tm_recv_atr_req(struct nfc_hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub atr_req: *mut st21nfca_atr_req,
    pub gb_len: usize,
    pub r: c_int,
    pub 1): skb_trim(skb, skb->len -,
    if (!skb.len)
    pub -EIO: return,
    if (skb.len < ST21NFCA_ATR_REQ_MIN_SIZE)
    pub -EPROTO: return,
    pub )skb->data: *mut atr_req = (struct st21nfca_atr_req,
    if (atr_req.length < sizeof(struct st21nfca_atr_req))
    pub -EPROTO: return,
    if (atr_req.length > skb.len)
    pub -EPROTO: return,
    pub atr_req): r = st21nfca_tm_send_atr_res(hdev,,
    if (r)
    pub r: return,
    pub st21nfca_atr_req): gb_len = skb->len - sizeof(struct,
    r = nfc_tm_activated(hdev.ndev, NFC_PROTO_NFC_DEP_MASK,
    pub gb_len): NFC_COMM_PASSIVE, atr_req->gbi,,
    if (r)
    pub r: return,
    pub 0: return,
    }
    static int st21nfca_tm_send_psl_res(struct nfc_hci_dev *hdev,
    struct st21nfca_psl_req *psl_req)
    {
    pub psl_res: *mut st21nfca_psl_res,
    pub skb: *mut sk_buff,
    pub 0}: u8 bitrate[2] = {0,,
    pub r: c_int,
    pub GFP_KERNEL): skb = alloc_skb(sizeof(struct st21nfca_psl_res),,
    if (!skb)
    pub -ENOMEM: return,
    pub st21nfca_psl_res)): skb_put(skb, sizeof(struct,
    pub )skb->data: *mut psl_res = (struct st21nfca_psl_res,
    pub st21nfca_psl_res): psl_res->length = sizeof(struct,
    pub ST21NFCA_NFCIP1_RES: psl_res->cmd0 =,
    pub ST21NFCA_NFCIP1_PSL_RES: psl_res->cmd1 =,
    pub psl_req->did: psl_res->did =,
    r = nfc_hci_send_event(hdev, ST21NFCA_RF_CARD_F_GATE,
    pub skb->len): ST21NFCA_EVT_SEND_DATA, skb->data,,
    if (r < 0)
    pub error: goto,
//
// ST21NFCA only support P2P passive.
// PSL_REQ BRS value != 0 has only a meaning to
// change technology to type F.
// We change to BITRATE 424Kbits.
// In other case switch to BITRATE 106Kbits.
//
    if (ST21NFCA_PSL_REQ_SEND_SPEED(psl_req.brs) &&
    ST21NFCA_PSL_REQ_RECV_SPEED(psl_req.brs)) {
    pub ST21NFCA_CARD_BITRATE_424: bitrate[0] =,
    pub ST21NFCA_CARD_BITRATE_424: bitrate[1] =,
    }
// Send an event to change bitrate change event to card f
    r = nfc_hci_send_event(hdev, ST21NFCA_RF_CARD_F_GATE,
    pub 2): ST21NFCA_EVT_CARD_F_BITRATE, bitrate,,
    error:
    pub r: return,
    }
    static int st21nfca_tm_recv_psl_req(struct nfc_hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub psl_req: *mut st21nfca_psl_req,
    pub 1): skb_trim(skb, skb->len -,
    if (!skb.len)
    pub -EIO: return,
    pub )skb->data: *mut psl_req = (struct st21nfca_psl_req,
    if (skb.len < sizeof(struct st21nfca_psl_req))
    pub -EIO: return,
    pub psl_req): return st21nfca_tm_send_psl_res(hdev,,
    }
#[no_mangle]
pub unsafe extern "C" fn st21nfca_tm_send_dep_res(hdev: *mut nfc_hci_dev, skb: *mut sk_buff) -> c_int {
    int st21nfca_tm_send_dep_res(struct nfc_hci_dev *hdev, struct sk_buff *skb)
    {
    pub r: c_int,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
// (u8 *)skb_push(skb, 1) = info->dep_info.curr_nfc_dep_pni;
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_DEP_RES;
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_RES;
// (u8 *)skb_push(skb, 1) = skb->len;
    r = nfc_hci_send_event(hdev, ST21NFCA_RF_CARD_F_GATE,
    pub skb->len): ST21NFCA_EVT_SEND_DATA, skb->data,,
    pub r: return,
    }
    static int st21nfca_tm_recv_dep_req(struct nfc_hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub dep_req: *mut st21nfca_dep_req_res,
    pub size: u8,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub 1): skb_trim(skb, skb->len -,
    pub 4: size =,
    pub )skb->data: *mut dep_req = (struct st21nfca_dep_req_res,
    if (skb.len < size)
    pub -EIO: return,
    if (ST21NFCA_NFC_DEP_DID_BIT_SET(dep_req.pfb))
    if (ST21NFCA_NFC_DEP_NAD_BIT_SET(dep_req.pfb))
    if (skb.len < size)
    pub -EIO: return,
// Receiving DEP_REQ - Decoding
    switch (ST21NFCA_NFC_DEP_PFB_TYPE(dep_req.pfb)) {
    case ST21NFCA_NFC_DEP_PFB_I_PDU:
    info.dep_info.curr_nfc_dep_pni =
    case ST21NFCA_NFC_DEP_PFB_ACK_NACK_PDU:
    pub PDU\n"): pr_err("Received a ACK/NACK,
    case ST21NFCA_NFC_DEP_PFB_SUPERVISOR_PDU:
    pub PDU\n"): pr_err("Received a SUPERVISOR,
    }
    pub size): skb_pull(skb,,
    pub skb): return nfc_tm_data_received(hdev->ndev,,
    }
    static int st21nfca_tm_event_send_data(struct nfc_hci_dev *hdev,
    struct sk_buff *skb)
    {
    pub cmd1: u8 cmd0,,
    pub r: c_int,
    pub skb->data[1]: cmd0 =,
    switch (cmd0) {
    case ST21NFCA_NFCIP1_REQ:
    pub skb->data[2]: cmd1 =,
    switch (cmd1) {
    case ST21NFCA_NFCIP1_ATR_REQ:
    pub skb): r = st21nfca_tm_recv_atr_req(hdev,,
    case ST21NFCA_NFCIP1_PSL_REQ:
    pub skb): r = st21nfca_tm_recv_psl_req(hdev,,
    case ST21NFCA_NFCIP1_DEP_REQ:
    pub skb): r = st21nfca_tm_recv_dep_req(hdev,,
    default:
    pub 1: return,
    }
    default:
    pub 1: return,
    }
    pub r: return,
    }
//
// Returns:
// <= 0: driver handled the event, skb consumed
// 1: driver does not handle the event, please do standard processing
//
    int st21nfca_dep_event_received(struct nfc_hci_dev *hdev,
    u8 event, struct sk_buff *skb)
    {
    pub 0: int r =,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub event): pr_debug("dep event: %d\n",,
    switch (event) {
    case ST21NFCA_EVT_CARD_ACTIVATED:
    pub 0: info->dep_info.curr_nfc_dep_pni =,
    case ST21NFCA_EVT_CARD_DEACTIVATED:
    case ST21NFCA_EVT_FIELD_ON:
    case ST21NFCA_EVT_FIELD_OFF:
    case ST21NFCA_EVT_SEND_DATA:
    pub skb): r = st21nfca_tm_event_send_data(hdev,,
    if (r < 0)
    pub r: return,
    pub 0: return,
    default:
    pub gate\n"): nfc_err(&hdev->ndev->dev, "Unexpected event on card f,
    pub 1: return,
    }
    pub r: return,
    }
    static void st21nfca_im_send_psl_req(struct nfc_hci_dev *hdev, u8 did, u8 bsi,
    u8 bri, u8 lri)
    {
    pub skb: *mut sk_buff,
    pub psl_req: *mut st21nfca_psl_req,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    skb =
    pub GFP_KERNEL): alloc_skb(sizeof(struct st21nfca_psl_req) + 1,,
    if (!skb)
    pub 1): skb_reserve(skb,,
    pub st21nfca_psl_req)): skb_put(skb, sizeof(struct,
    pub skb->data: *mut *mut psl_req = (struct st21nfca_psl_req ),
    pub st21nfca_psl_req): psl_req->length = sizeof(struct,
    pub ST21NFCA_NFCIP1_REQ: psl_req->cmd0 =,
    pub ST21NFCA_NFCIP1_PSL_REQ: psl_req->cmd1 =,
    pub did: psl_req->did =,
    pub 0x03): psl_req->brs = (0x30 & bsi << 4) | (bri &,
    pub lri: psl_req->fsl =,
// (u8 *)skb_push(skb, 1) = info->dep_info.to | 0x10;
    pub skb): st21nfca_im_send_pdu(info,,
    }
pub const ST21NFCA_CB_TYPE_READER_F: c_int = 1;
    static void st21nfca_im_recv_atr_res_cb(void *context, struct sk_buff *skb,
    int err)
    {
    pub context: *mut *mut st21nfca_hci_info info =,
    pub atr_res: *mut st21nfca_atr_res,
    pub r: c_int,
    if (err != 0)
    if (!skb)
    switch (info.async_cb_type) {
    case ST21NFCA_CB_TYPE_READER_F:
    pub 1): skb_trim(skb, skb->len -,
    pub )skb->data: *mut atr_res = (struct st21nfca_atr_res,
    r = nfc_set_remote_general_bytes(info.hdev.ndev,
    atr_res.gbi,
    pub st21nfca_atr_res)): skb->len - sizeof(struct,
    if (r < 0)
    if (atr_res.to >= 0x0e)
    pub 0x0e: info->dep_info.to =,
    else
    pub 1: info->dep_info.to = atr_res->to +,
    pub 0x10: info->dep_info.to |=,
    r = nfc_dep_link_is_up(info.hdev.ndev, info.dep_info.idx,
    pub NFC_RF_INITIATOR): NFC_COMM_PASSIVE,,
    if (r < 0)
    pub 0: info->dep_info.curr_nfc_dep_pni =,
    if (ST21NFCA_PP2LRI(atr_res.ppi) != info.dep_info.lri)
    st21nfca_im_send_psl_req(info.hdev, atr_res.did,
    atr_res.bsi, atr_res.bri,
    default:
    }
    }
#[no_mangle]
pub unsafe extern "C" fn st21nfca_im_send_atr_req(hdev: *mut nfc_hci_dev, gb: *mut u8, gb_len: usize) -> c_int {
    int st21nfca_im_send_atr_req(struct nfc_hci_dev *hdev, u8 *gb, size_t gb_len)
    {
    pub skb: *mut sk_buff,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub atr_req: *mut st21nfca_atr_req,
    pub target: *mut nfc_target,
    pub size: c_uint,
    pub ST21NFCA_DEFAULT_TIMEOUT: info->dep_info.to =,
    pub gb_len: size = ST21NFCA_ATR_REQ_MIN_SIZE +,
    if (size > ST21NFCA_ATR_REQ_MAX_SIZE) {
    pub -EINVAL: return,
    }
    skb =
    pub GFP_KERNEL): alloc_skb(sizeof(struct st21nfca_atr_req) + gb_len + 1,,
    if (!skb)
    pub -ENOMEM: return,
    pub 1): skb_reserve(skb,,
    pub st21nfca_atr_req)): skb_put(skb, sizeof(struct,
    pub )skb->data: *mut atr_req = (struct st21nfca_atr_req,
    pub st21nfca_atr_req)): memset(atr_req, 0, sizeof(struct,
    pub ST21NFCA_NFCIP1_REQ: atr_req->cmd0 =,
    pub ST21NFCA_NFCIP1_ATR_REQ: atr_req->cmd1 =,
    pub NFC_NFCID3_MAXSIZE): memset(atr_req->nfcid3, 0,,
    pub hdev->ndev->targets: target =,
    if (target.sensf_res_len > 0)
    memcpy(atr_req.nfcid3, target.sensf_res,
    else
    pub NFC_NFCID3_MAXSIZE): get_random_bytes(atr_req->nfcid3,,
    pub 0x0: atr_req->did =,
    pub 0x00: atr_req->bsi =,
    pub 0x00: atr_req->bri =,
    pub ST21NFCA_LR_BITS_PAYLOAD_SIZE_254B: atr_req->ppi =,
    if (gb_len) {
    pub ST21NFCA_GB_BIT: atr_req->ppi |=,
    pub gb_len): skb_put_data(skb, gb,,
    }
    pub hdev->gb_len: atr_req->length = sizeof(struct st21nfca_atr_req) +,
// (u8 *)skb_push(skb, 1) = info->dep_info.to | 0x10; /* timeout
    pub ST21NFCA_CB_TYPE_READER_F: info->async_cb_type =,
    pub info: info->async_cb_context =,
    pub st21nfca_im_recv_atr_res_cb: info->async_cb =,
    pub atr_req->bri: info->dep_info.bri =,
    pub atr_req->bsi: info->dep_info.bsi =,
    pub ST21NFCA_PP2LRI(atr_req->ppi): info->dep_info.lri =,
    return nfc_hci_send_cmd_async(hdev, ST21NFCA_RF_READER_F_GATE,
    ST21NFCA_WR_XCHG_DATA, skb.data,
    pub info): skb->len, info->async_cb,,
    }
    static void st21nfca_im_recv_dep_res_cb(void *context, struct sk_buff *skb,
    int err)
    {
    pub context: *mut *mut st21nfca_hci_info info =,
    pub dep_res: *mut st21nfca_dep_req_res,
    pub size: c_int,
    if (err != 0)
    if (!skb)
    switch (info.async_cb_type) {
    case ST21NFCA_CB_TYPE_READER_F:
    pub )skb->data: *mut dep_res = (struct st21nfca_dep_req_res,
    pub 3: size =,
    if (skb.len < size)
    pub exit: goto,
    if (ST21NFCA_NFC_DEP_DID_BIT_SET(dep_res.pfb))
    if (ST21NFCA_NFC_DEP_NAD_BIT_SET(dep_res.pfb))
    if (skb.len < size)
    pub exit: goto,
    pub 1): skb_trim(skb, skb->len -,
// Receiving DEP_REQ - Decoding
    switch (ST21NFCA_NFC_DEP_PFB_TYPE(dep_res.pfb)) {
    case ST21NFCA_NFC_DEP_PFB_ACK_NACK_PDU:
    pub PDU\n"): pr_err("Received a ACK/NACK,
    case ST21NFCA_NFC_DEP_PFB_I_PDU:
    info.dep_info.curr_nfc_dep_pni =
    pub 1): ST21NFCA_NFC_DEP_PFB_PNI(dep_res->pfb +,
    pub size): skb_pull(skb,,
    pub skb): nfc_tm_data_received(info->hdev->ndev,,
    case ST21NFCA_NFC_DEP_PFB_SUPERVISOR_PDU:
    pub PDU\n"): pr_err("Received a SUPERVISOR,
    pub size): skb_pull(skb,,
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_DEP_REQ;
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_REQ;
// (u8 *)skb_push(skb, 1) = skb->len;
// (u8 *)skb_push(skb, 1) = info->dep_info.to | 0x10;
    pub skb): st21nfca_im_send_pdu(info,,
    }
    default:
    }
    exit:
    }
#[no_mangle]
pub unsafe extern "C" fn st21nfca_im_send_dep_req(hdev: *mut nfc_hci_dev, skb: *mut sk_buff) -> c_int {
    int st21nfca_im_send_dep_req(struct nfc_hci_dev *hdev, struct sk_buff *skb)
    {
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub ST21NFCA_CB_TYPE_READER_F: info->async_cb_type =,
    pub info: info->async_cb_context =,
    pub st21nfca_im_recv_dep_res_cb: info->async_cb =,
// (u8 *)skb_push(skb, 1) = info->dep_info.curr_nfc_dep_pni;
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_DEP_REQ;
// (u8 *)skb_push(skb, 1) = ST21NFCA_NFCIP1_REQ;
// (u8 *)skb_push(skb, 1) = skb->len;
// (u8 *)skb_push(skb, 1) = info->dep_info.to | 0x10;
    return nfc_hci_send_cmd_async(hdev, ST21NFCA_RF_READER_F_GATE,
    ST21NFCA_WR_XCHG_DATA,
    skb.data, skb.len,
    pub info): info->async_cb,,
    }
#[no_mangle]
pub unsafe extern "C" fn st21nfca_dep_init(hdev: *mut nfc_hci_dev) {
    void st21nfca_dep_init(struct nfc_hci_dev *hdev)
    {
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    pub st21nfca_tx_work): INIT_WORK(&info->dep_info.tx_work,,
    pub 0: info->dep_info.curr_nfc_dep_pni =,
    pub 0: info->dep_info.idx =,
    pub ST21NFCA_DEFAULT_TIMEOUT: info->dep_info.to =,
    }
#[no_mangle]
pub unsafe extern "C" fn st21nfca_dep_deinit(hdev: *mut nfc_hci_dev) {
    void st21nfca_dep_deinit(struct nfc_hci_dev *hdev)
    {
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    }
