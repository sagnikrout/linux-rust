//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_actions_sriov_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// DOC: GUC2PF_RELAY_FROM_VF
//
// This message is used by the GuC firmware to forward a VF2PF `Relay Message`_
// received from the Virtual Function (VF) driver to this Physical Function (PF)
// driver.
//
// This message is always sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_EVENT_                                   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`XE_GUC_ACTION_GUC2PF_RELAY_FROM_VF` = 0x5100      |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VFID** - source VF identifier                              |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **RELAY_ID** - VF/PF message ID                              |
// +---+-------+-----------------+--------------------------------------------+
// | 3 |  31:0 | **RELAY_DATA1** |                                            |
// +---+-------+-----------------+                                            |
// |...|       |                 |       [Embedded `Relay Message`_]          |
// +---+-------+-----------------+                                            |
// | n |  31:0 | **RELAY_DATAx** |                                            |
// +---+-------+-----------------+--------------------------------------------+
//
pub const XE_GUC_ACTION_GUC2PF_RELAY_FROM_VF: c_uint = 0x5100;

//
// DOC: PF2GUC_RELAY_TO_VF
//
// This H2G message is used by the Physical Function (PF) driver to send embedded
// VF2PF `Relay Message`_ to the VF.
//
// This action message must be sent over CTB as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = `GUC_HXG_TYPE_FAST_REQUEST`_                          |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`XE_GUC_ACTION_PF2GUC_RELAY_TO_VF` = 0x5101        |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VFID** - target VF identifier                              |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **RELAY_ID** - VF/PF message ID                              |
// +---+-------+-----------------+--------------------------------------------+
// | 3 |  31:0 | **RELAY_DATA1** |                                            |
// +---+-------+-----------------+                                            |
// |...|       |                 |       [Embedded `Relay Message`_]          |
// +---+-------+-----------------+                                            |
// | n |  31:0 | **RELAY_DATAx** |                                            |
// +---+-------+-----------------+--------------------------------------------+
//
pub const XE_GUC_ACTION_PF2GUC_RELAY_TO_VF: c_uint = 0x5101;

//
// DOC: GUC2VF_RELAY_FROM_PF
//
// This message is used by the GuC firmware to deliver `Relay Message`_ from the
// Physical Function (PF) driver to this Virtual Function (VF) driver.
// See `GuC Relay Communication`_ for details.
//
// This message is always sent over CTB.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_EVENT_                                   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`XE_GUC_ACTION_GUC2VF_RELAY_FROM_PF` = 0x5102      |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **RELAY_ID** - VF/PF message ID                              |
// +---+-------+-----------------+--------------------------------------------+
// | 2 |  31:0 | **RELAY_DATA1** |                                            |
// +---+-------+-----------------+                                            |
// |...|       |                 |       [Embedded `Relay Message`_]          |
// +---+-------+-----------------+                                            |
// | n |  31:0 | **RELAY_DATAx** |                                            |
// +---+-------+-----------------+--------------------------------------------+
//
pub const XE_GUC_ACTION_GUC2VF_RELAY_FROM_PF: c_uint = 0x5102;

//
// DOC: VF2GUC_RELAY_TO_PF
//
// This message is used by the Virtual Function (VF) drivers to communicate with
// the Physical Function (PF) driver and send `Relay Message`_ to the PF driver.
// See `GuC Relay Communication`_ for details.
//
// This message must be sent over CTB.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_ or GUC_HXG_TYPE_FAST_REQUEST_   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`XE_GUC_ACTION_VF2GUC_RELAY_TO_PF` = 0x5103        |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **RELAY_ID** - VF/PF message ID                              |
// +---+-------+-----------------+--------------------------------------------+
// | 2 |  31:0 | **RELAY_DATA1** |                                            |
// +---+-------+-----------------+                                            |
// |...|       |                 |       [Embedded `Relay Message`_]          |
// +---+-------+-----------------+                                            |
// | n |  31:0 | **RELAY_DATAx** |                                            |
// +---+-------+-----------------+--------------------------------------------+
//
pub const XE_GUC_ACTION_VF2GUC_RELAY_TO_PF: c_uint = 0x5103;

//
// DOC: GUC2PF_ADVERSE_EVENT
//
// This message is used by the GuC to notify PF about adverse events.
//
// This G2H message must be sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_EVENT_                                   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_GUC2PF_ADVERSE_EVENT` = 0x5104         |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | DATA1 = **VFID** - VF identifier                             |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | DATA2 = **THRESHOLD** - key of the exceeded threshold        |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_GUC2PF_ADVERSE_EVENT: c_uint = 0x5104;

//
// DOC: GUC2PF_VF_STATE_NOTIFY
//
// The GUC2PF_VF_STATE_NOTIFY message is used by the GuC to notify PF about change
// of the VF state.
//
// This G2H message is sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_EVENT_                                   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_GUC2PF_VF_STATE_NOTIFY` = 0x5106       |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | DATA1 = **VFID** - VF identifier                             |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | DATA2 = **EVENT** - notification event:                      |
// |   |       |                                                              |
// |   |       |   - _`GUC_PF_NOTIFY_VF_ENABLE` = 1 (only if VFID = 0)        |
// |   |       |   - _`GUC_PF_NOTIFY_VF_FLR` = 1                              |
// |   |       |   - _`GUC_PF_NOTIFY_VF_FLR_DONE` = 2                         |
// |   |       |   - _`GUC_PF_NOTIFY_VF_PAUSE_DONE` = 3                       |
// |   |       |   - _`GUC_PF_NOTIFY_VF_FIXUP_DONE` = 4                       |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_GUC2PF_VF_STATE_NOTIFY: c_uint = 0x5106u;

//
// DOC: VF2GUC_MATCH_VERSION
//
// This action is used to match VF interface version used by VF and GuC.
//
// This message must be sent as `MMIO HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_VF2GUC_MATCH_VERSION` = 0x5500         |
// +---+-------+--------------------------------------------------------------+
// | 1 | 31:24 | **BRANCH** - branch ID of the VF interface                   |
// |   |       | (use BRANCH_ANY to request latest version supported by GuC)  |
// |   +-------+--------------------------------------------------------------+
// |   | 23:16 | **MAJOR** - major version of the VF interface                |
// |   |       | (use MAJOR_ANY to request latest version supported by GuC)   |
// |   +-------+--------------------------------------------------------------+
// |   |  15:8 | **MINOR** - minor version of the VF interface                |
// |   |       | (use MINOR_ANY to request latest version supported by GuC)   |
// |   +-------+--------------------------------------------------------------+
// |   |   7:0 | **MBZ**                                                      |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = MBZ                                                  |
// +---+-------+--------------------------------------------------------------+
// | 1 | 31:24 | **BRANCH** - branch ID of the VF interface                   |
// |   +-------+--------------------------------------------------------------+
// |   | 23:16 | **MAJOR** - major version of the VF interface                |
// |   +-------+--------------------------------------------------------------+
// |   |  15:8 | **MINOR** - minor version of the VF interface                |
// |   +-------+--------------------------------------------------------------+
// |   |   7:0 | **PATCH** - patch version of the VF interface                |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_VF2GUC_MATCH_VERSION: c_uint = 0x5500u;

pub const GUC_VERSION_BRANCH_ANY: c_int = 0;

pub const GUC_VERSION_MAJOR_ANY: c_int = 0;

pub const GUC_VERSION_MINOR_ANY: c_int = 0;

//
// DOC: PF2GUC_UPDATE_VGT_POLICY
//
// This message is used by the PF to set `GuC VGT Policy KLVs`_.
//
// This message must be sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_PF2GUC_UPDATE_VGT_POLICY` = 0x5502     |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **CFG_ADDR_LO** - dword aligned GGTT offset that             |
// |   |       | represents the start of `GuC VGT Policy KLVs`_ list.         |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **CFG_ADDR_HI** - upper 32 bits of above offset.             |
// +---+-------+--------------------------------------------------------------+
// | 3 |  31:0 | **CFG_SIZE** - size (in dwords) of the config buffer         |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **COUNT** - number of KLVs successfully applied              |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_PF2GUC_UPDATE_VGT_POLICY: c_uint = 0x5502u;

//
// DOC: PF2GUC_UPDATE_VF_CFG
//
// The `PF2GUC_UPDATE_VF_CFG`_ message is used by PF to provision single VF in GuC.
//
// This message must be sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_PF2GUC_UPDATE_VF_CFG` = 0x5503         |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VFID** - identifier of the VF that the KLV                 |
// |   |       | configurations are being applied to                          |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **CFG_ADDR_LO** - dword aligned GGTT offset that represents  |
// |   |       | the start of a list of virtualization related KLV configs    |
// |   |       | that are to be applied to the VF.                            |
// |   |       | If this parameter is zero, the list is not parsed.           |
// |   |       | If full configs address parameter is zero and configs_size is|
// |   |       | zero associated VF config shall be reset to its default state|
// +---+-------+--------------------------------------------------------------+
// | 3 |  31:0 | **CFG_ADDR_HI** - upper 32 bits of configs address.          |
// +---+-------+--------------------------------------------------------------+
// | 4 |  31:0 | **CFG_SIZE** - size (in dwords) of the config buffer         |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **COUNT** - number of KLVs successfully applied              |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_PF2GUC_UPDATE_VF_CFG: c_uint = 0x5503u;

//
// DOC: PF2GUC_VF_CONTROL
//
// The PF2GUC_VF_CONTROL message is used by the PF to trigger VF state change
// maintained by the GuC.
//
// This H2G message must be sent as `CTB HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_PF2GUC_VF_CONTROL_CMD` = 0x5506        |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | DATA1 = **VFID** - VF identifier                             |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | DATA2 = **COMMAND** - control command:                       |
// |   |       |                                                              |
// |   |       |   - _`GUC_PF_TRIGGER_VF_PAUSE` = 1                           |
// |   |       |   - _`GUC_PF_TRIGGER_VF_RESUME` = 2                          |
// |   |       |   - _`GUC_PF_TRIGGER_VF_STOP` = 3                            |
// |   |       |   - _`GUC_PF_TRIGGER_VF_FLR_START` = 4                       |
// |   |       |   - _`GUC_PF_TRIGGER_VF_FLR_FINISH` = 5                      |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = MBZ                                                  |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_PF2GUC_VF_CONTROL: c_uint = 0x5506u;

//
// DOC: VF2GUC_VF_RESET
//
// This action is used by VF to reset GuC's VF state.
//
// This message must be sent as `MMIO HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MBZ                                                  |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_VF2GUC_VF_RESET` = 0x5507              |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = MBZ                                                  |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_VF2GUC_VF_RESET: c_uint = 0x5507u;

//
// DOC: VF2GUC_RESFIX_DONE
//
// This action is used by VF to inform the GuC that the VF KMD has completed
// post-migration recovery steps. From GuC VF compatibility 1.27.0 onwards, it
// shall only be sent after posting RESFIX_START and that both @MARKER fields
// must match.
//
// This message must be sent as `MMIO HXG Message`_.
//
// Updated since GuC VF compatibility 1.27.0.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MARKER = MBZ (only prior 1.27.0)                     |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MARKER - can't be zero (1.27.0+)                     |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_VF2GUC_RESFIX_DONE` = 0x5508           |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = MBZ                                                  |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_VF2GUC_RESFIX_DONE: c_uint = 0x5508u;

//
// DOC: VF2GUC_QUERY_SINGLE_KLV
//
// This action is used by VF to query value of the single KLV data.
//
// This message must be sent as `MMIO HXG Message`_.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_VF2GUC_QUERY_SINGLE_KLV` = 0x5509      |
// +---+-------+--------------------------------------------------------------+
// | 1 | 31:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **KEY** - key for which value is requested                   |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | MBZ                                                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **LENGTH** - length of data in dwords                        |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VALUE32** - bits 31:0 of value if **LENGTH** >= 1          |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **VALUE64** - bits 63:32 of value if **LENGTH** >= 2         |
// +---+-------+--------------------------------------------------------------+
// | 3 |  31:0 | **VALUE96** - bits 95:64 of value if **LENGTH** >= 3         |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_VF2GUC_QUERY_SINGLE_KLV: c_uint = 0x5509u;

//
// DOC: PF2GUC_SAVE_RESTORE_VF
//
// This message is used by the PF to migrate VF info state maintained by the GuC.
//
// This message must be sent as `CTB HXG Message`_.
//
// Available since GuC version 70.25.0
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = **OPCODE** - operation to take:                      |
// |   |       |                                                              |
// |   |       |   - _`GUC_PF_OPCODE_VF_SAVE` = 0                             |
// |   |       |   - _`GUC_PF_OPCODE_VF_RESTORE` = 1                          |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_PF2GUC_SAVE_RESTORE_VF` = 0x550B       |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VFID** - VF identifier                                     |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **ADDR_LO** - lower 32-bits of GGTT offset to the buffer     |
// |   |       | where the VF info will be save to or restored from.          |
// +---+-------+--------------------------------------------------------------+
// | 3 |  31:0 | **ADDR_HI** - upper 32-bits of GGTT offset to the buffer     |
// |   |       | where the VF info will be save to or restored from.          |
// +---+-------+--------------------------------------------------------------+
// | 4 |  27:0 | **SIZE** - size of the buffer (in dwords)                    |
// |   +-------+--------------------------------------------------------------+
// |   | 31:28 | MBZ                                                          |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = **USED** - size of used buffer space (in dwords)     |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_PF2GUC_SAVE_RESTORE_VF: c_uint = 0x550Bu;

//
// DOC: VF2GUC_RESFIX_START
//
// This action is used by VF to inform the GuC that the VF KMD will be starting
// post-migration recovery fixups. The @MARKER sent with this action must match
// with the MARKER posted in the VF2GUC_RESFIX_DONE message.
//
// This message must be sent as `MMIO HXG Message`_.
//
// Available since GuC VF compatibility 1.27.0.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_HOST_                                |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 = MARKER - can't be zero                               |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION = _`GUC_ACTION_VF2GUC_RESFIX_START` = 0x550F          |
// +---+-------+--------------------------------------------------------------+
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN = GUC_HXG_ORIGIN_GUC_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | DATA0 = MBZ                                                  |
// +---+-------+--------------------------------------------------------------+
//
pub const GUC_ACTION_VF2GUC_RESFIX_START: c_uint = 0x550Fu;

