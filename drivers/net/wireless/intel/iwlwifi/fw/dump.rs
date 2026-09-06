//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/fw/dump.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2012-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2015-2017 Intel Deutschland GmbH
//

//
// Note: This structure is read from the device with IO accesses,
// and the reading already does the endian conversion. As it is
// read with u32-sized accesses, any members with a different size
// need to be ordered correctly though!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_error_event_table {
    pub /: *mut *mut u32 valid; / (nonzero) valid, (0) log is empty,
    pub /: *mut *mut u32 error_id; / type of error,
    pub /: *mut *mut u32 trm_hw_status0; / TRM HW status,
    pub /: *mut *mut u32 trm_hw_status1; / TRM HW status,
    pub /: *mut *mut u32 blink2; / branch link,
    pub /: *mut *mut u32 ilink1; / interrupt link,
    pub /: *mut *mut u32 ilink2; / interrupt link,
    pub /: *mut *mut u32 data1; / error-specific data,
    pub /: *mut *mut u32 data2; / error-specific data,
    pub /: *mut *mut u32 data3; / error-specific data,
    pub /: *mut *mut u32 bcon_time; / beacon timer,
    pub /: *mut *mut u32 tsf_low; / network timestamp function timer,
    pub /: *mut *mut u32 tsf_hi; / network timestamp function timer,
    pub /: *mut *mut u32 gp1; / GP1 timer register,
    pub /: *mut *mut u32 gp2; / GP2 timer register,
    pub /: *mut *mut u32 fw_rev_type; / firmware revision type,
    pub /: *mut *mut u32 major; / uCode version major,
    pub /: *mut *mut u32 minor; / uCode version minor,
    pub /: *mut *mut u32 hw_ver; / HW Silicon version,
    pub /: *mut *mut u32 brd_ver; / HW board version,
    pub /: *mut *mut u32 log_pc; / log program counter,
    pub /: *mut *mut u32 frame_ptr; / frame pointer,
    pub /: *mut *mut u32 stack_ptr; / stack pointer,
    pub /: *mut *mut u32 hcmd; / last host command header,
    pub LMPM_NIC_ISR0:: *mut *mut u32 isr0; / isr status register,
// rxtx_flag
    pub LMPM_NIC_ISR1:: *mut *mut u32 isr1; / isr status register,
// host_flag
    pub LMPM_NIC_ISR2:: *mut *mut u32 isr2; / isr status register,
// enc_flag
    pub LMPM_NIC_ISR3:: *mut *mut u32 isr3; / isr status register,
// time_flag
    pub LMPM_NIC_ISR4:: *mut *mut u32 isr4; / isr status register,
// wico interrupt
    pub /: *mut *mut u32 last_cmd_id; / last HCMD id handled by the firmware,
    pub /: *mut *mut u32 wait_event; / wait event() caller address,
    pub /: *mut *mut u32 l2p_control; / L2pControlField,
    pub /: *mut *mut u32 l2p_duration; / L2pDurationField,
    pub /: *mut *mut u32 l2p_mhvalid; / L2pMhValidBits,
    pub /: *mut *mut u32 l2p_addr_match; / L2pAddrMatchStat,
    pub on: *mut *mut u32 lmpm_pmg_sel; / indicate which clocks are turned,
// (LMPM_PMG_SEL)
    pub the: *mut *mut u32 u_timestamp; / indicate when the date and time of,
// compilation
    pub /: *mut *mut u32 flow_handler; / FH read/write pointers, RX credit,
    pub /: *mut *mut } __packed / LOG_ERROR_TABLE_API_S_VER_3,
//
// UMAC error struct - relevant starting from family 8000 chip.
// Note: This structure is read from the device with IO accesses,
// and the reading already does the endian conversion. As it is
// read with u32-sized accesses, any members with a different size
// need to be ordered correctly though!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_umac_error_event_table {
    pub /: *mut *mut u32 valid; / (nonzero) valid, (0) log is empty,
    pub /: *mut *mut u32 error_id; / type of error,
    pub /: *mut *mut u32 blink1; / branch link,
    pub /: *mut *mut u32 blink2; / branch link,
    pub /: *mut *mut u32 ilink1; / interrupt link,
    pub /: *mut *mut u32 ilink2; / interrupt link,
    pub /: *mut *mut u32 data1; / error-specific data,
    pub /: *mut *mut u32 data2; / error-specific data,
    pub /: *mut *mut u32 data3; / error-specific data,
    pub umac_major: u32,
    pub umac_minor: u32,
    pub 27*/: *mut *mut u32 frame_pointer; / core register,
    pub /: *mut *mut u32 stack_pointer; / core register 28,
    pub /: *mut *mut u32 cmd_header; / latest host cmd sent to UMAC,
    pub /: *mut *mut u32 nic_isr_pref; / ISR status register,
    pub __packed: },

#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_umac_error_log(fwrt: *mut iwl_fw_runtime) {
    static void iwl_fwrt_dump_umac_error_log(struct iwl_fw_runtime *fwrt)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub {}: iwl_umac_error_event_table table =,
    pub fwrt->trans->dbg.umac_error_event_table: u32 base =,
    pub pnvm_name: [c_char; MAX_PNVM_NAME],
    if (!base &&
    !(fwrt.trans.dbg.error_event_table_tlv_status &
    IWL_ERROR_EVENT_TABLE_UMAC))
    pub sizeof(table)): iwl_trans_read_mem_bytes(trans, base, &table,,
    if (table.valid)
    pub table.error_id: fwrt->dump.umac_err_id =,
    if (ERROR_START_OFFSET <= table.valid * ERROR_ELEM_SIZE) {
    pub Dump:\n"): IWL_ERR(trans, "Start IWL Error Log,
    IWL_ERR(trans, "Transport status: 0x%08lX, valid: %d\n",
    pub table.valid): fwrt->trans->status,,
    }
    if ((table.error_id & ~FW_SYSASSERT_CPU_MASK) ==
    FW_SYSASSERT_PNVM_MISSING) {
    pub sizeof(pnvm_name)): iwl_pnvm_get_fs_name(trans, pnvm_name,,
    IWL_ERR(fwrt, "PNVM data is missing, please install %s\n",
    }
    IWL_ERR(fwrt, "0x%08X | %s\n", table.error_id,
    pub table.ilink2): IWL_ERR(fwrt, "0x%08X | umac interruptlink2\n",,
    pub table.data1): IWL_ERR(fwrt, "0x%08X | umac data1\n",,
    pub table.data2): IWL_ERR(fwrt, "0x%08X | umac data2\n",,
    pub table.data3): IWL_ERR(fwrt, "0x%08X | umac data3\n",,
    pub table.cmd_header): IWL_ERR(fwrt, "0x%08X | last host cmd\n",,
    }
#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_lmac_error_log(fwrt: *mut iwl_fw_runtime, lmac_num: u8) {
    static void iwl_fwrt_dump_lmac_error_log(struct iwl_fw_runtime *fwrt, u8 lmac_num)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub {}: iwl_error_event_table table =,
    pub fwrt->trans->dbg.lmac_error_event_table[lmac_num]: u32 val, base =,
    if (fwrt.cur_fw_img == IWL_UCODE_INIT) {
    if (!base)
    pub fwrt->fw->init_errlog_ptr: base =,
    } else {
    if (!base)
    pub fwrt->fw->inst_errlog_ptr: base =,
    }
    if (!base) {
    IWL_ERR(fwrt,
    "Not valid error log pointer 0x%08X for %s uCode\n",
    base,
    (fwrt.cur_fw_img == IWL_UCODE_INIT)
    pub "RT"): ? "Init" :,
    }
// check if there is a HW error
    pub base): val = iwl_trans_read_mem32(trans,,
    if (iwl_trans_is_hw_error_value(val)) {
    pub err: c_int,
    pub reading\n"): IWL_ERR(trans, "HW error, resetting before,
// reset the device
    pub iwl_trans_sw_reset(trans): err =,
    if (err)
    pub iwl_trans_activate_nic(trans): err =,
    if (err)
    }
    pub sizeof(table)): iwl_trans_read_mem_bytes(trans, base, &table,,
    if (table.valid)
    pub table.error_id: fwrt->dump.lmac_err_id[lmac_num] =,
    if (ERROR_START_OFFSET <= table.valid * ERROR_ELEM_SIZE) {
    pub Dump:\n"): IWL_ERR(trans, "Start IWL Error Log,
    IWL_ERR(trans, "Transport status: 0x%08lX, valid: %d\n",
    pub table.valid): fwrt->trans->status,,
    }
// Do not change this output - scripts rely on it
    pub fwrt->fw->fw_version): IWL_ERR(fwrt, "Loaded firmware version: %s\n",,
    IWL_ERR(fwrt, "0x%08X | %-28s\n", table.error_id,
    pub table.ilink2): IWL_ERR(fwrt, "0x%08X | interruptlink2\n",,
    pub table.data1): IWL_ERR(fwrt, "0x%08X | data1\n",,
    pub table.data2): IWL_ERR(fwrt, "0x%08X | data2\n",,
    pub table.data3): IWL_ERR(fwrt, "0x%08X | data3\n",,
    }
//
// TCM error struct.
// Note: This structure is read from the device with IO accesses,
// and the reading already does the endian conversion. As it is
// read with u32-sized accesses, any members with a different size
// need to be ordered correctly though!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tcm_error_event_table {
    pub valid: u32,
    pub error_id: u32,
    pub blink2: u32,
    pub ilink1: u32,
    pub ilink2: u32,
    pub data3: u32 data1, data2,,
    pub logpc: u32,
    pub frame_pointer: u32,
    pub stack_pointer: u32,
    pub msgid: u32,
    pub isr: u32,
    pub hw_status: [u32; 5],
    pub sw_status: [u32; 1],
    pub reserved: [u32; 4],
    pub /: *mut *mut } __packed; / TCM_LOG_ERROR_TABLE_API_S_VER_1,
#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_tcm_error_log(fwrt: *mut iwl_fw_runtime, idx: c_int) {
    static void iwl_fwrt_dump_tcm_error_log(struct iwl_fw_runtime *fwrt, int idx)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub {}: iwl_tcm_error_event_table table =,
    pub fwrt->trans->dbg.tcm_error_event_table[idx]: u32 base =,
    u32 flag = idx ? IWL_ERROR_EVENT_TABLE_TCM2 :
    if (!base || !(fwrt.trans.dbg.error_event_table_tlv_status & flag))
    pub sizeof(table)): iwl_trans_read_mem_bytes(trans, base, &table,,
    pub 1): IWL_ERR(fwrt, "TCM%d status:\n", idx +,
    pub table.error_id): IWL_ERR(fwrt, "0x%08X | error ID\n",,
    pub table.ilink2): IWL_ERR(fwrt, "0x%08X | tcm interruptlink2\n",,
    pub table.data1): IWL_ERR(fwrt, "0x%08X | tcm data1\n",,
    pub table.data2): IWL_ERR(fwrt, "0x%08X | tcm data2\n",,
    pub table.data3): IWL_ERR(fwrt, "0x%08X | tcm data3\n",,
    }
//
// RCM error struct.
// Note: This structure is read from the device with IO accesses,
// and the reading already does the endian conversion. As it is
// read with u32-sized accesses, any members with a different size
// need to be ordered correctly though!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rcm_error_event_table {
    pub valid: u32,
    pub error_id: u32,
    pub blink2: u32,
    pub ilink1: u32,
    pub ilink2: u32,
    pub data3: u32 data1, data2,,
    pub logpc: u32,
    pub frame_pointer: u32,
    pub stack_pointer: u32,
    pub msgid: u32,
    pub isr: u32,
    pub frame_hw_status: u32,
    pub mbx_lmac_to_rcm_req: u32,
    pub mbx_rcm_to_lmac_req: u32,
    pub mh_ctl: u32,
    pub mh_addr1_lo: u32,
    pub mh_info: u32,
    pub mh_err: u32,
    pub reserved: [u32; 3],
    pub /: *mut *mut } __packed; / RCM_LOG_ERROR_TABLE_API_S_VER_1,
#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_rcm_error_log(fwrt: *mut iwl_fw_runtime, idx: c_int) {
    static void iwl_fwrt_dump_rcm_error_log(struct iwl_fw_runtime *fwrt, int idx)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub {}: iwl_rcm_error_event_table table =,
    pub fwrt->trans->dbg.rcm_error_event_table[idx]: u32 base =,
    u32 flag = idx ? IWL_ERROR_EVENT_TABLE_RCM2 :
    if (!base || !(fwrt.trans.dbg.error_event_table_tlv_status & flag))
    pub sizeof(table)): iwl_trans_read_mem_bytes(trans, base, &table,,
    pub 1): IWL_ERR(fwrt, "RCM%d status:\n", idx +,
    pub table.error_id): IWL_ERR(fwrt, "0x%08X | error ID\n",,
    pub table.ilink2): IWL_ERR(fwrt, "0x%08X | rcm interruptlink2\n",,
    pub table.data1): IWL_ERR(fwrt, "0x%08X | rcm data1\n",,
    pub table.data2): IWL_ERR(fwrt, "0x%08X | rcm data2\n",,
    pub table.data3): IWL_ERR(fwrt, "0x%08X | rcm data3\n",,
    }
#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_iml_error_log(fwrt: *mut iwl_fw_runtime) {
    static void iwl_fwrt_dump_iml_error_log(struct iwl_fw_runtime *fwrt)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub data1: u32 error,,
    if (fwrt.trans.mac_cfg.device_family >= IWL_DEVICE_FAMILY_22000) {
    pub UMAG_SB_CPU_2_STATUS: error =,
    pub UMAG_SB_CPU_1_STATUS: data1 =,
    } else if (fwrt.trans.mac_cfg.device_family >=
    IWL_DEVICE_FAMILY_8000) {
    pub SB_CPU_2_STATUS: error =,
    pub SB_CPU_1_STATUS: data1 =,
    } else {
    }
    pub error): error = iwl_read_umac_prph(trans,,
    pub dump:\n"): IWL_ERR(trans, "IML/ROM,
    if (error & 0xFFFF0000)
    pub 16): IWL_ERR(trans, "0x%04X | IML/ROM SYSASSERT\n", error >>,
    pub error): IWL_ERR(fwrt, "0x%08X | IML/ROM error/state\n",,
    IWL_ERR(fwrt, "0x%08X | IML/ROM data1\n",
    pub data1)): iwl_read_umac_prph(trans,,
    if (fwrt.trans.mac_cfg.device_family >= IWL_DEVICE_FAMILY_22000)
    IWL_ERR(fwrt, "0x%08X | IML/ROM WFPM_AUTH_KEY_0\n",
    pub SB_MODIFY_CFG_FLAG)): iwl_read_umac_prph(trans,,
    }

#[no_mangle]
unsafe extern "C" fn iwl_fwrt_dump_fseq_regs(fwrt: *mut iwl_fw_runtime) {
    static void iwl_fwrt_dump_fseq_regs(struct iwl_fw_runtime *fwrt)
    {
    pub fwrt->trans: *mut *mut iwl_trans trans =,
    pub i: c_int,
    struct {
    pub addr: u32,
    pub str: *const c_char,
    } fseq_regs[] = {
    FSEQ_REG(FSEQ_ERROR_CODE),
    FSEQ_REG(FSEQ_TOP_INIT_VERSION),
    FSEQ_REG(FSEQ_CNVIO_INIT_VERSION),
    FSEQ_REG(FSEQ_OTP_VERSION),
    FSEQ_REG(FSEQ_TOP_CONTENT_VERSION),
    FSEQ_REG(FSEQ_ALIVE_TOKEN),
    FSEQ_REG(FSEQ_CNVI_ID),
    FSEQ_REG(FSEQ_CNVR_ID),
    FSEQ_REG(CNVI_AUX_MISC_CHIP),
    FSEQ_REG(CNVR_AUX_MISC_CHIP),
    FSEQ_REG(CNVR_SCU_SD_REGS_SD_REG_DIG_DCDC_VTRIM),
    FSEQ_REG(CNVR_SCU_SD_REGS_SD_REG_ACTIVE_VDIG_MIRROR),
    FSEQ_REG(FSEQ_PREV_CNVIO_INIT_VERSION),
    FSEQ_REG(FSEQ_WIFI_FSEQ_VERSION),
    FSEQ_REG(FSEQ_BT_FSEQ_VERSION),
    FSEQ_REG(FSEQ_CLASS_TP_VERSION),
}

    if (!iwl_trans_grab_nic_access(trans))
    return;
    IWL_ERR(fwrt, "Fseq Registers:\n");
    for (i = 0; i < ARRAY_SIZE(fseq_regs); i++)
    IWL_ERR(fwrt, "0x%08X | %s\n",
    iwl_read_prph_no_grab(trans, fseq_regs[i].addr),
    fseq_regs[i].str);
    iwl_trans_release_nic_access(trans);
    }
#[no_mangle]
pub unsafe extern "C" fn iwl_fwrt_dump_error_logs(fwrt: *mut iwl_fw_runtime) {
    void iwl_fwrt_dump_error_logs(struct iwl_fw_runtime *fwrt)
    {
    struct iwl_pc_data *pc_data;
    u32 count;
    if (!iwl_trans_device_enabled(fwrt.trans)) {
    IWL_ERR(fwrt,
    "DEVICE_ENABLED bit is not set. Aborting dump.\n");
    return;
    }
    iwl_fwrt_dump_lmac_error_log(fwrt, 0);
    if (fwrt.trans.dbg.lmac_error_event_table[1])
    iwl_fwrt_dump_lmac_error_log(fwrt, 1);
    iwl_fwrt_dump_umac_error_log(fwrt);
    iwl_fwrt_dump_tcm_error_log(fwrt, 0);
    iwl_fwrt_dump_rcm_error_log(fwrt, 0);
    if (fwrt.trans.dbg.tcm_error_event_table[1])
    iwl_fwrt_dump_tcm_error_log(fwrt, 1);
    if (fwrt.trans.dbg.rcm_error_event_table[1])
    iwl_fwrt_dump_rcm_error_log(fwrt, 1);
    iwl_fwrt_dump_iml_error_log(fwrt);
    iwl_fwrt_dump_fseq_regs(fwrt);
    if (fwrt.trans.mac_cfg.device_family >= IWL_DEVICE_FAMILY_22000) {
    pc_data = fwrt.trans.dbg.pc_data;
    if (!iwl_trans_grab_nic_access(fwrt.trans))
    return;
    for (count = 0; count < fwrt.trans.dbg.num_pc;
    count++, pc_data++)
    IWL_ERR(fwrt, "%s: 0x%x\n",
    pc_data.pc_name,
    iwl_read_prph_no_grab(fwrt.trans,
    pc_data.pc_address));
    iwl_trans_release_nic_access(fwrt.trans);
    }
    if (fwrt.trans.mac_cfg.device_family >= IWL_DEVICE_FAMILY_BZ) {
    let mut scratch: u32 = iwl_read32(fwrt.trans, CSR_FUNC_SCRATCH);
    IWL_ERR(fwrt, "Function Scratch status:\n");
    IWL_ERR(fwrt, "0x%08X | Func Scratch\n", scratch);
    }
    }
    IWL_EXPORT_SYMBOL(iwl_fwrt_dump_error_logs);
#[no_mangle]
pub unsafe extern "C" fn iwl_fwrt_read_err_table(trans: *mut iwl_trans, base: u32, err_id: *mut u32) -> bool {
    bool iwl_fwrt_read_err_table(struct iwl_trans *trans, u32 base, u32 *err_id)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_table_start {
// cf. struct iwl_error_event_table
    pub valid: u32,
    pub err_id: __le32,
    pub {}: } err_info =,
    pub ret: c_int,
    if (err_id)
// err_id = 0;
    if (!base)
    pub false: return,
    ret = iwl_trans_read_mem_bytes(trans, base,
    pub sizeof(err_info)): &err_info,,
    if (ret)
    pub true: return,
    if (err_info.valid && err_id)
// err_id = le32_to_cpu(err_info.err_id);
    pub !!err_info.valid: return,
    }
