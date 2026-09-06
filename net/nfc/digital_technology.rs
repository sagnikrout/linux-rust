//! Automatically rewritten from C to Rust
//! Source: net/nfc/digital_technology.c
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
// NFC Digital Protocol stack
// Copyright (c) 2013, Intel Corporation.
//

pub const DIGITAL_CMD_SENS_REQ: c_uint = 0x26;
pub const DIGITAL_CMD_ALL_REQ: c_uint = 0x52;
pub const DIGITAL_CMD_SEL_REQ_CL1: c_uint = 0x93;
pub const DIGITAL_CMD_SEL_REQ_CL2: c_uint = 0x95;
pub const DIGITAL_CMD_SEL_REQ_CL3: c_uint = 0x97;
pub const DIGITAL_SDD_REQ_SEL_PAR: c_uint = 0x20;
pub const DIGITAL_SDD_RES_CT: c_uint = 0x88;
pub const DIGITAL_SDD_RES_LEN: c_int = 5;
pub const DIGITAL_SEL_RES_LEN: c_int = 1;

    ((!((sens_res) & 0x001F) && (((sens_res) & 0x0C00) == 0x0C00)) || \
    (((sens_res) & 0x001F) && ((sens_res) & 0x0C00) != 0x0C00))
pub const DIGITAL_MIFARE_READ_RES_LEN: c_int = 16;
pub const DIGITAL_MIFARE_ACK_RES: c_uint = 0x0A;
pub const DIGITAL_CMD_SENSB_REQ: c_uint = 0x05;

pub const DIGITAL_CMD_SENSB_RES: c_uint = 0x50;
pub const DIGITAL_CMD_ATTRIB_REQ: c_uint = 0x1D;

pub const DIGITAL_ATTRIB_P2_MAX_FRAME_256: c_uint = 0x8;

pub const DIGITAL_CMD_SENSF_REQ: c_uint = 0x00;
pub const DIGITAL_CMD_SENSF_RES: c_uint = 0x01;
pub const DIGITAL_SENSF_RES_MIN_LENGTH: c_int = 17;
pub const DIGITAL_SENSF_RES_RD_AP_B1: c_uint = 0x00;
pub const DIGITAL_SENSF_RES_RD_AP_B2: c_uint = 0x8F;
pub const DIGITAL_SENSF_REQ_RC_NONE: c_int = 0;
pub const DIGITAL_SENSF_REQ_RC_SC: c_int = 1;
pub const DIGITAL_SENSF_REQ_RC_AP: c_int = 2;
pub const DIGITAL_CMD_ISO15693_INVENTORY_REQ: c_uint = 0x01;

    (!((flags) & DIGITAL_ISO15693_RES_FLAG_ERROR))
pub const DIGITAL_ISO_DEP_I_PCB: c_uint = 0x02;

pub const DIGITAL_ISO_DEP_I_BLOCK: c_uint = 0x00;

    static const u8 digital_ats_fsc[] = {
    16,  24,  32,  40,  48,  64,  96, 128,
    };

pub const DIGITAL_ATS_MAX_FSC: c_int = 256;
pub const DIGITAL_RATS_BYTE1: c_uint = 0xE0;
pub const DIGITAL_RATS_PARAM: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sdd_res {
    pub nfcid1: [u8; 4],
    pub bcc: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sel_req {
    pub sel_cmd: u8,
    pub b2: u8,
    pub nfcid1: [u8; 4],
    pub bcc: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sensb_req {
    pub cmd: u8,
    pub afi: u8,
    pub param: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sensb_res {
    pub cmd: u8,
    pub nfcid0: [u8; 4],
    pub app_data: [u8; 4],
    pub proto_info: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_attrib_req {
    pub cmd: u8,
    pub nfcid0: [u8; 4],
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
    pub param4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_attrib_res {
    pub mbli_did: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sensf_req {
    pub cmd: u8,
    pub sc1: u8,
    pub sc2: u8,
    pub rc: u8,
    pub tsn: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_sensf_res {
    pub cmd: u8,
    pub nfcid2: [u8; 8],
    pub pad0: [u8; 2],
    pub pad1: [u8; 3],
    pub mrti_check: u8,
    pub mrti_update: u8,
    pub pad2: u8,
    pub rd: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_iso15693_inv_req {
    pub flags: u8,
    pub cmd: u8,
    pub mask_len: u8,
    pub mask: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_iso15693_inv_res {
    pub flags: u8,
    pub dsfid: u8,
    pub uid: u64,
    pub __packed: },
    static int digital_in_send_sdd_req(struct nfc_digital_dev *ddev,
    pub target): *mut nfc_target,
    int digital_in_iso_dep_pull_sod(struct nfc_digital_dev *ddev,
    struct sk_buff *skb)
    {
    pub pcb: u8,
    pub block_type: u8,
    if (skb.len < 1)
    pub -EIO: return,
    pub skb->data: *mut pcb =,
    pub DIGITAL_ISO_DEP_PCB_TYPE(pcb): block_type =,
// No support fo R-block nor S-block
    if (block_type != DIGITAL_ISO_DEP_I_BLOCK) {
    pub supported\n"): pr_err("ISO_DEP R-block and S-block not,
    pub -EIO: return,
    }
    if (DIGITAL_ISO_DEP_BLOCK_HAS_DID(pcb)) {
    pub supported\n"): pr_err("DID field in ISO_DEP PCB not,
    pub -EIO: return,
    }
    pub 1): skb_pull(skb,,
    pub 0: return,
    }
    int digital_in_iso_dep_push_sod(struct nfc_digital_dev *ddev,
    struct sk_buff *skb)
    {
//
// Chaining not supported so skb->len + 1 PCB byte + 2 CRC bytes must
// not be greater than remote FSC
//
    if (skb.len + 3 > ddev.target_fsc)
    pub -EIO: return,
    pub 1): skb_push(skb,,
// skb->data = DIGITAL_ISO_DEP_I_PCB | ddev->curr_nfc_dep_pni;
    ddev.curr_nfc_dep_pni =
    pub 1): DIGITAL_ISO_DEP_PNI(ddev->curr_nfc_dep_pni +,
    pub 0: return,
    }
    static void digital_in_recv_ats(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub arg: *mut *mut nfc_target target =,
    pub fsdi: u8,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len < 2) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub DIGITAL_ATS_FSCI(resp->data[1]): fsdi =,
    if (fsdi >= 8)
    pub DIGITAL_ATS_MAX_FSC: ddev->target_fsc =,
    else
    pub digital_ats_fsc: [ddev->target_fsc =; fsdi],
    pub 0: ddev->curr_nfc_dep_pni =,
    pub NFC_PROTO_ISO14443): rc = digital_target_found(ddev, target,,
    exit:
    if (rc)
    }
    static int digital_in_send_rats(struct nfc_digital_dev *ddev,
    struct nfc_target *target)
    {
    pub rc: c_int,
    pub skb: *mut sk_buff,
    pub 2): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub DIGITAL_RATS_BYTE1): skb_put_u8(skb,,
    pub DIGITAL_RATS_PARAM): skb_put_u8(skb,,
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_ats,
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_sel_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub arg: *mut *mut nfc_target target =,
    pub rc: c_int,
    pub sel_res: u8,
    pub nfc_proto: u8,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (!DIGITAL_DRV_CAPS_IN_CRC(ddev)) {
    pub digital_skb_check_crc_a(resp): rc =,
    if (rc) {
    pub exit: goto,
    }
    }
    if (resp.len != DIGITAL_SEL_RES_LEN) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub resp->data[0]: sel_res =,
    if (!DIGITAL_SEL_RES_NFCID1_COMPLETE(sel_res)) {
    pub target): rc = digital_in_send_sdd_req(ddev,,
    if (rc)
    pub exit: goto,
    pub exit_free_skb: goto,
    }
    pub sel_res: target->sel_res =,
    if (DIGITAL_SEL_RES_IS_T2T(sel_res)) {
    pub NFC_PROTO_MIFARE: nfc_proto =,
    } else if (DIGITAL_SEL_RES_IS_NFC_DEP(sel_res)) {
    pub NFC_PROTO_NFC_DEP: nfc_proto =,
    } else if (DIGITAL_SEL_RES_IS_T4T(sel_res)) {
    pub target): rc = digital_in_send_rats(ddev,,
    if (rc)
    pub exit: goto,
//
// Skip target_found and don't free it for now. This will be
// done when receiving the ATS
//
    pub exit_free_skb: goto,
    } else {
    pub -EOPNOTSUPP: rc =,
    pub exit: goto,
    }
    pub nfc_proto): rc = digital_target_found(ddev, target,,
    exit:
    exit_free_skb:
    if (rc)
    }
    static int digital_in_send_sel_req(struct nfc_digital_dev *ddev,
    struct nfc_target *target,
    struct digital_sdd_res *sdd_res)
    {
    pub skb: *mut sk_buff,
    pub sel_req: *mut digital_sel_req,
    pub sel_cmd: u8,
    pub rc: c_int,
    pub digital_sel_req)): skb = digital_skb_alloc(ddev, sizeof(struct,
    if (!skb)
    pub -ENOMEM: return,
    pub digital_sel_req)): skb_put(skb, sizeof(struct,
    pub )skb->data: *mut sel_req = (struct digital_sel_req,
    if (target.nfcid1_len <= 4)
    pub DIGITAL_CMD_SEL_REQ_CL1: sel_cmd =,
#[no_mangle]
pub unsafe extern "C" fn if(10: target->nfcid1_len <) -> else {
    else if (target.nfcid1_len < 10)
    pub DIGITAL_CMD_SEL_REQ_CL2: sel_cmd =,
    else
    pub DIGITAL_CMD_SEL_REQ_CL3: sel_cmd =,
    pub sel_cmd: sel_req->sel_cmd =,
    pub 0x70: sel_req->b2 =,
    pub 4): memcpy(sel_req->nfcid1, sdd_res->nfcid1,,
    pub sdd_res->bcc: sel_req->bcc =,
    if (DIGITAL_DRV_CAPS_IN_CRC(ddev)) {
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub exit: goto,
    } else {
    }
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_sel_res,
    exit:
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_sdd_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub arg: *mut *mut nfc_target target =,
    pub sdd_res: *mut digital_sdd_res,
    pub rc: c_int,
    pub size: u8 offset,,
    pub bcc: u8 i,,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len < DIGITAL_SDD_RES_LEN) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    pub )resp->data: *mut sdd_res = (struct digital_sdd_res,
    pub i++): for (i = 0, bcc = 0; i < 4;,
    pub sdd_res->nfcid1[i]: bcc ^=,
    if (bcc != sdd_res.bcc) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    if (sdd_res.nfcid1[0] == DIGITAL_SDD_RES_CT) {
    pub 1: offset =,
    pub 3: size =,
    } else {
    pub 0: offset =,
    pub 4: size =,
    }
    if (target.nfcid1_len + size > NFC_NFCID1_MAXSIZE) {
    pub -EPROTO: rc =,
    pub exit: goto,
    }
    memcpy(target.nfcid1 + target.nfcid1_len, sdd_res.nfcid1 + offset,
    pub size: target->nfcid1_len +=,
    pub sdd_res): rc = digital_in_send_sel_req(ddev, target,,
    exit:
    if (rc) {
    }
    }
    static int digital_in_send_sdd_req(struct nfc_digital_dev *ddev,
    struct nfc_target *target)
    {
    pub rc: c_int,
    pub skb: *mut sk_buff,
    pub sel_cmd: u8,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub rc: return,
    pub 2): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    if (target.nfcid1_len == 0)
    pub DIGITAL_CMD_SEL_REQ_CL1: sel_cmd =,
#[no_mangle]
pub unsafe extern "C" fn if(3: target->nfcid1_len ==) -> else {
    else if (target.nfcid1_len == 3)
    pub DIGITAL_CMD_SEL_REQ_CL2: sel_cmd =,
    else
    pub DIGITAL_CMD_SEL_REQ_CL3: sel_cmd =,
    pub sel_cmd): skb_put_u8(skb,,
    pub DIGITAL_SDD_REQ_SEL_PAR): skb_put_u8(skb,,
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_sdd_res,
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_sens_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub NULL: *mut *mut nfc_target target =,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len < sizeof(u16)) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub nfc_target): target = kzalloc_obj(struct,
    if (!target) {
    pub -ENOMEM: rc =,
    pub exit: goto,
    }
    pub )resp->data): *mut *mut target->sens_res = __le16_to_cpu((__le16,
    if (!DIGITAL_SENS_RES_IS_VALID(target.sens_res)) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    if (DIGITAL_SENS_RES_IS_T1T(target.sens_res))
    pub NFC_PROTO_JEWEL): rc = digital_target_found(ddev, target,,
    else
    pub target): rc = digital_in_send_sdd_req(ddev,,
    exit:
    if (rc) {
    }
    }
#[no_mangle]
pub unsafe extern "C" fn digital_in_send_sens_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_in_send_sens_req(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub skb: *mut sk_buff,
    pub rc: c_int,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,
    if (rc)
    pub rc: return,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub rc: return,
    pub 1): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub DIGITAL_CMD_SENS_REQ): skb_put_u8(skb,,
    pub NULL): rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_sens_res,,
    if (rc)
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn digital_in_recv_mifare_res(resp: *mut sk_buff) -> c_int {
    int digital_in_recv_mifare_res(struct sk_buff *resp)
    {
// Successful READ command response is 16 data bytes + 2 CRC bytes long.
// Since the driver can't differentiate a ACK/NACK response from a valid
// READ response, the CRC calculation must be handled at digital level
// even if the driver supports it for this technology.
//
    if (resp.len == DIGITAL_MIFARE_READ_RES_LEN + DIGITAL_CRC_LEN) {
    if (digital_skb_check_crc_a(resp)) {
    pub -EIO: return,
    }
    pub 0: return,
    }
// ACK response (i.e. successful WRITE).
    if (resp.len == 1 && resp.data[0] == DIGITAL_MIFARE_ACK_RES) {
    pub 0: resp->data[0] =,
    pub 0: return,
    }
// NACK and any other responses are treated as error.
    pub -EIO: return,
    }
    static void digital_in_recv_attrib_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub arg: *mut *mut nfc_target target =,
    pub attrib_res: *mut digital_attrib_res,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len < sizeof(*attrib_res)) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub )resp->data: *mut attrib_res = (struct digital_attrib_res,
    if (attrib_res.mbli_did & 0x0f) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub NFC_PROTO_ISO14443_B): rc = digital_target_found(ddev, target,,
    exit:
    if (rc)
    }
    static int digital_in_send_attrib_req(struct nfc_digital_dev *ddev,
    struct nfc_target *target,
    struct digital_sensb_res *sensb_res)
    {
    pub attrib_req: *mut digital_attrib_req,
    pub skb: *mut sk_buff,
    pub rc: c_int,
    pub sizeof(*attrib_req)): *mut skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub sizeof(*attrib_req)): *mut attrib_req = skb_put(skb,,
    pub DIGITAL_CMD_ATTRIB_REQ: attrib_req->cmd =,
    memcpy(attrib_req.nfcid0, sensb_res.nfcid0,
    attrib_req.param1 = DIGITAL_ATTRIB_P1_TR0_DEFAULT |
    attrib_req.param2 = DIGITAL_ATTRIB_P2_LISTEN_POLL_1 |
    DIGITAL_ATTRIB_P2_POLL_LISTEN_1 |
    pub 0x07: attrib_req->param3 = sensb_res->proto_info[1] &,
    pub DIGITAL_ATTRIB_P4_DID(0): attrib_req->param4 =,
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_attrib_res,
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_sensb_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub NULL: *mut *mut nfc_target target =,
    pub sensb_res: *mut digital_sensb_res,
    pub fsci: u8,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len != sizeof(*sensb_res)) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub )resp->data: *mut sensb_res = (struct digital_sensb_res,
    if (sensb_res.cmd != DIGITAL_CMD_SENSB_RES) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    if (!(sensb_res.proto_info[1] & BIT(0))) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    if (sensb_res.proto_info[1] & BIT(3)) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    pub DIGITAL_SENSB_FSCI(sensb_res->proto_info[1]): fsci =,
    if (fsci >= 8)
    pub DIGITAL_ATS_MAX_FSC: ddev->target_fsc =,
    else
    pub digital_ats_fsc: [ddev->target_fsc =; fsci],
    pub nfc_target): target = kzalloc_obj(struct,
    if (!target) {
    pub -ENOMEM: rc =,
    pub exit: goto,
    }
    pub sensb_res): rc = digital_in_send_attrib_req(ddev, target,,
    exit:
    if (rc) {
    }
    }
#[no_mangle]
pub unsafe extern "C" fn digital_in_send_sensb_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_in_send_sensb_req(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub sensb_req: *mut digital_sensb_req,
    pub skb: *mut sk_buff,
    pub rc: c_int,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,
    if (rc)
    pub rc: return,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub rc: return,
    pub sizeof(*sensb_req)): *mut skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub sizeof(*sensb_req)): *mut sensb_req = skb_put(skb,,
    pub DIGITAL_CMD_SENSB_REQ: sensb_req->cmd =,
    pub /: *mut *mut sensb_req->afi = 0x00; / All families and sub-families,
    pub DIGITAL_SENSB_N(0): sensb_req->param =,
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_sensb_res,
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_sensf_res(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub rc: c_int,
    pub proto: u8,
    pub target: nfc_target,
    pub sensf_res: *mut digital_sensf_res,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (resp.len < DIGITAL_SENSF_RES_MIN_LENGTH) {
    pub -EIO: rc =,
    pub exit: goto,
    }
    if (!DIGITAL_DRV_CAPS_IN_CRC(ddev)) {
    pub digital_skb_check_crc_f(resp): rc =,
    if (rc) {
    pub exit: goto,
    }
    }
    pub 1): skb_pull(resp,,
    pub nfc_target)): memset(&target, 0, sizeof(struct,
    pub )resp->data: *mut sensf_res = (struct digital_sensf_res,
    pub NFC_SENSF_RES_MAXSIZE): resp->len = min_t(unsigned int, resp->len,,
    pub resp->len): memcpy(target.sensf_res, sensf_res,,
    pub resp->len: target.sensf_res_len =,
    pub NFC_NFCID2_MAXSIZE): memcpy(target.nfcid2, sensf_res->nfcid2,,
    pub NFC_NFCID2_MAXSIZE: target.nfcid2_len =,
    if (target.nfcid2[0] == DIGITAL_SENSF_NFCID2_NFC_DEP_B1 &&
    target.nfcid2[1] == DIGITAL_SENSF_NFCID2_NFC_DEP_B2)
    pub NFC_PROTO_NFC_DEP: proto =,
    else
    pub NFC_PROTO_FELICA: proto =,
    pub proto): rc = digital_target_found(ddev, &target,,
    exit:
    if (rc)
    }
#[no_mangle]
pub unsafe extern "C" fn digital_in_send_sensf_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_in_send_sensf_req(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub sensf_req: *mut digital_sensf_req,
    pub skb: *mut sk_buff,
    pub rc: c_int,
    pub size: u8,
    pub rf_tech): rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,,
    if (rc)
    pub rc: return,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub rc: return,
    pub digital_sensf_req): size = sizeof(struct,
    pub size): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub size): skb_put(skb,,
    pub )skb->data: *mut sensf_req = (struct digital_sensf_req,
    pub DIGITAL_CMD_SENSF_REQ: sensf_req->cmd =,
    pub 0xFF: sensf_req->sc1 =,
    pub 0xFF: sensf_req->sc2 =,
    pub 0: sensf_req->rc =,
    pub 0: sensf_req->tsn =,
// (u8 *)skb_push(skb, 1) = size + 1;
    if (!DIGITAL_DRV_CAPS_IN_CRC(ddev))
    rc = digital_in_send_cmd(ddev, skb, 30, digital_in_recv_sensf_res,
    if (rc)
    pub rc: return,
    }
    static void digital_in_recv_iso15693_inv_res(struct nfc_digital_dev *ddev,
    void *arg, struct sk_buff *resp)
    {
    pub res: *mut digital_iso15693_inv_res,
    pub NULL: *mut *mut nfc_target target =,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub out_free_skb: goto,
    }
    if (resp.len != sizeof(*res)) {
    pub -EIO: rc =,
    pub out_free_skb: goto,
    }
    pub )resp->data: *mut res = (struct digital_iso15693_inv_res,
    if (!DIGITAL_ISO15693_RES_IS_VALID(res.flags)) {
    pub 10.3.1"): PROTOCOL_ERR("ISO15693 -,
    pub -EINVAL: rc =,
    pub out_free_skb: goto,
    }
    pub kzalloc_obj(*target): *mut target =,
    if (!target) {
    pub -ENOMEM: rc =,
    pub out_free_skb: goto,
    }
    pub 1: target->is_iso15693 =,
    pub res->dsfid: target->iso15693_dsfid =,
    pub sizeof(target->iso15693_uid)): memcpy(target->iso15693_uid, &res->uid,,
    pub NFC_PROTO_ISO15693): rc = digital_target_found(ddev, target,,
    out_free_skb:
    if (rc)
    }
#[no_mangle]
pub unsafe extern "C" fn digital_in_send_iso15693_inv_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_in_send_iso15693_inv_req(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub req: *mut digital_iso15693_inv_req,
    pub skb: *mut sk_buff,
    pub rc: c_int,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,
    if (rc)
    pub rc: return,
    rc = digital_in_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc)
    pub rc: return,
    pub sizeof(*req)): *mut skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub /: *mut *mut *mut skb_put(skb, sizeof(req) - sizeof(req->mask)); / No mask,
    pub )skb->data: *mut req = (struct digital_iso15693_inv_req,
// Single sub-carrier, high data rate, no AFI, single slot
// Inventory command
//
    req.flags = DIGITAL_ISO15693_REQ_FLAG_DATA_RATE |
    DIGITAL_ISO15693_REQ_FLAG_INVENTORY |
    pub DIGITAL_CMD_ISO15693_INVENTORY_REQ: req->cmd =,
    pub 0: req->mask_len =,
    rc = digital_in_send_cmd(ddev, skb, 30,
    pub NULL): digital_in_recv_iso15693_inv_res,,
    if (rc)
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn digital_tg_send_sel_res(ddev: *mut nfc_digital_dev) -> c_int {
    static int digital_tg_send_sel_res(struct nfc_digital_dev *ddev)
    {
    pub skb: *mut sk_buff,
    pub rc: c_int,
    pub 1): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub DIGITAL_SEL_RES_NFC_DEP): skb_put_u8(skb,,
    if (!DIGITAL_DRV_CAPS_TG_CRC(ddev))
    rc = digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc) {
    pub rc: return,
    }
    rc = digital_tg_send_cmd(ddev, skb, 300, digital_tg_recv_atr_req,
    if (rc)
    pub rc: return,
    }
    static void digital_tg_recv_sel_req(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (!DIGITAL_DRV_CAPS_TG_CRC(ddev)) {
    pub digital_skb_check_crc_a(resp): rc =,
    if (rc) {
    pub exit: goto,
    }
    }
// Silently ignore SEL_REQ content and send a SEL_RES for NFC-DEP
    pub digital_tg_send_sel_res(ddev): rc =,
    exit:
    if (rc)
    }
#[no_mangle]
unsafe extern "C" fn digital_tg_send_sdd_res(ddev: *mut nfc_digital_dev) -> c_int {
    static int digital_tg_send_sdd_res(struct nfc_digital_dev *ddev)
    {
    pub skb: *mut sk_buff,
    pub sdd_res: *mut digital_sdd_res,
    pub i: int rc,,
    pub digital_sdd_res)): skb = digital_skb_alloc(ddev, sizeof(struct,
    if (!skb)
    pub -ENOMEM: return,
    pub digital_sdd_res)): skb_put(skb, sizeof(struct,
    pub )skb->data: *mut sdd_res = (struct digital_sdd_res,
    pub 0x08: sdd_res->nfcid1[0] =,
    pub 3): get_random_bytes(sdd_res->nfcid1 + 1,,
    pub 0: sdd_res->bcc =,
    pub i++): for (i = 0; i < 4;,
    pub sdd_res->nfcid1[i]: sdd_res->bcc ^=,
    rc = digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc) {
    pub rc: return,
    }
    rc = digital_tg_send_cmd(ddev, skb, 300, digital_tg_recv_sel_req,
    if (rc)
    pub rc: return,
    }
    static void digital_tg_recv_sdd_req(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub sdd_req: *mut u8,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    pub resp->data: sdd_req =,
    if (resp.len < 2 || sdd_req[0] != DIGITAL_CMD_SEL_REQ_CL1 ||
    sdd_req[1] != DIGITAL_SDD_REQ_SEL_PAR) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    pub digital_tg_send_sdd_res(ddev): rc =,
    exit:
    if (rc)
    }
#[no_mangle]
unsafe extern "C" fn digital_tg_send_sens_res(ddev: *mut nfc_digital_dev) -> c_int {
    static int digital_tg_send_sens_res(struct nfc_digital_dev *ddev)
    {
    pub skb: *mut sk_buff,
    pub sens_res: *mut u8,
    pub rc: c_int,
    pub 2): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub 2): sens_res = skb_put(skb,,
    pub 0xFF: sens_res[0] = (DIGITAL_SENS_RES_NFC_DEP >> 8) &,
    pub 0xFF: sens_res[1] = DIGITAL_SENS_RES_NFC_DEP &,
    rc = digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    if (rc) {
    pub rc: return,
    }
    rc = digital_tg_send_cmd(ddev, skb, 300, digital_tg_recv_sdd_req,
    if (rc)
    pub rc: return,
    }
    void digital_tg_recv_sens_req(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub sens_req: u8,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    pub resp->data[0]: sens_req =,
    if (!resp.len || (sens_req != DIGITAL_CMD_SENS_REQ &&
    sens_req != DIGITAL_CMD_ALL_REQ)) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    pub digital_tg_send_sens_res(ddev): rc =,
    exit:
    if (rc)
    }
    static void digital_tg_recv_atr_or_sensf_req(struct nfc_digital_dev *ddev,
    void *arg, struct sk_buff *resp)
    {
    if (!IS_ERR(resp) && (resp.len >= 2) &&
    (resp.data[1] == DIGITAL_CMD_SENSF_REQ))
    pub resp): digital_tg_recv_sensf_req(ddev, arg,,
    else
    pub resp): digital_tg_recv_atr_req(ddev, arg,,
    }
    static int digital_tg_send_sensf_res(struct nfc_digital_dev *ddev,
    struct digital_sensf_req *sensf_req)
    {
    pub skb: *mut sk_buff,
    pub size: u8,
    pub rc: c_int,
    pub sensf_res: *mut digital_sensf_res,
    pub digital_sensf_res): size = sizeof(struct,
    if (sensf_req.rc == DIGITAL_SENSF_REQ_RC_NONE)
    pub sizeof(sensf_res->rd): size -=,
    pub size): skb = digital_skb_alloc(ddev,,
    if (!skb)
    pub -ENOMEM: return,
    pub size): skb_put(skb,,
    pub )skb->data: *mut sensf_res = (struct digital_sensf_res,
    pub size): memset(sensf_res, 0,,
    pub DIGITAL_CMD_SENSF_RES: sensf_res->cmd =,
    pub DIGITAL_SENSF_NFCID2_NFC_DEP_B1: sensf_res->nfcid2[0] =,
    pub DIGITAL_SENSF_NFCID2_NFC_DEP_B2: sensf_res->nfcid2[1] =,
    pub 6): get_random_bytes(&sensf_res->nfcid2[2],,
    switch (sensf_req.rc) {
    case DIGITAL_SENSF_REQ_RC_SC:
    pub sensf_req->sc1: sensf_res->rd[0] =,
    pub sensf_req->sc2: sensf_res->rd[1] =,
    case DIGITAL_SENSF_REQ_RC_AP:
    pub DIGITAL_SENSF_RES_RD_AP_B1: sensf_res->rd[0] =,
    pub DIGITAL_SENSF_RES_RD_AP_B2: sensf_res->rd[1] =,
    }
// (u8 *)skb_push(skb, sizeof(u8)) = size + 1;
    if (!DIGITAL_DRV_CAPS_TG_CRC(ddev))
    rc = digital_tg_send_cmd(ddev, skb, 300,
    pub NULL): digital_tg_recv_atr_or_sensf_req,,
    if (rc)
    pub rc: return,
    }
    void digital_tg_recv_sensf_req(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub sensf_req: *mut digital_sensf_req,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub PTR_ERR(resp): rc =,
    pub NULL: resp =,
    pub exit: goto,
    }
    if (!DIGITAL_DRV_CAPS_TG_CRC(ddev)) {
    pub digital_skb_check_crc_f(resp): rc =,
    if (rc) {
    pub exit: goto,
    }
    }
    if (resp.len != sizeof(struct digital_sensf_req) + 1) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    pub 1): skb_pull(resp,,
    pub )resp->data: *mut sensf_req = (struct digital_sensf_req,
    if (sensf_req.cmd != DIGITAL_CMD_SENSF_REQ) {
    pub -EINVAL: rc =,
    pub exit: goto,
    }
    pub sensf_req): rc = digital_tg_send_sensf_res(ddev,,
    exit:
    if (rc)
    }
#[no_mangle]
unsafe extern "C" fn digital_tg_config_nfca(ddev: *mut nfc_digital_dev) -> c_int {
    static int digital_tg_config_nfca(struct nfc_digital_dev *ddev)
    {
    pub rc: c_int,
    rc = digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,
    if (rc)
    pub rc: return,
    return digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    }
#[no_mangle]
pub unsafe extern "C" fn digital_tg_listen_nfca(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_tg_listen_nfca(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub rc: c_int,
    pub digital_tg_config_nfca(ddev): rc =,
    if (rc)
    pub rc: return,
    pub NULL): return digital_tg_listen(ddev, 300, digital_tg_recv_sens_req,,
    }
#[no_mangle]
unsafe extern "C" fn digital_tg_config_nfcf(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    static int digital_tg_config_nfcf(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub rc: c_int,
    pub rf_tech): rc = digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,,
    if (rc)
    pub rc: return,
    return digital_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    }
#[no_mangle]
pub unsafe extern "C" fn digital_tg_listen_nfcf(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int {
    int digital_tg_listen_nfcf(struct nfc_digital_dev *ddev, u8 rf_tech)
    {
    pub rc: c_int,
    pub rf_tech): rc = digital_tg_config_nfcf(ddev,,
    if (rc)
    pub rc: return,
    pub NULL): return digital_tg_listen(ddev, 300, digital_tg_recv_sensf_req,,
    }
    void digital_tg_recv_md_req(struct nfc_digital_dev *ddev, void *arg,
    struct sk_buff *resp)
    {
    pub rf_tech: u8,
    pub rc: c_int,
    if (IS_ERR(resp)) {
    pub NULL: resp =,
    pub exit_free_skb: goto,
    }
    pub &rf_tech): rc = ddev->ops->tg_get_rf_tech(ddev,,
    if (rc)
    pub exit_free_skb: goto,
    switch (rf_tech) {
    case NFC_DIGITAL_RF_TECH_106A:
    pub digital_tg_config_nfca(ddev): rc =,
    if (rc)
    pub exit_free_skb: goto,
    pub resp): digital_tg_recv_sens_req(ddev, arg,,
    case NFC_DIGITAL_RF_TECH_212F:
    case NFC_DIGITAL_RF_TECH_424F:
    pub rf_tech): rc = digital_tg_config_nfcf(ddev,,
    if (rc)
    pub exit_free_skb: goto,
    pub resp): digital_tg_recv_sensf_req(ddev, arg,,
    default:
    pub exit_free_skb: goto,
    }
    exit_free_skb:
    }
