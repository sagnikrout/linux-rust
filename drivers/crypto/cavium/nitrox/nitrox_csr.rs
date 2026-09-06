//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/nitrox/nitrox_csr.h
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


// SPDX-License-Identifier: GPL-2.0

// EMU clusters
pub const NR_CLUSTERS: c_int = 4;
// Maximum cores per cluster,
// varies based on partname
//
pub const AE_CORES_PER_CLUSTER: c_int = 20;
pub const SE_CORES_PER_CLUSTER: c_int = 16;

pub const ZIP_MAX_CORES: c_int = 5;
// BIST registers

pub const UCD_BIST_STATUS: c_uint = 0x12C0070;
pub const NPS_CORE_BIST_REG: c_uint = 0x10000E8;
pub const NPS_CORE_NPC_BIST_REG: c_uint = 0x1000128;
pub const NPS_PKT_SLC_BIST_REG: c_uint = 0x1040088;
pub const NPS_PKT_IN_BIST_REG: c_uint = 0x1040100;
pub const POM_BIST_REG: c_uint = 0x11C0100;
pub const BMI_BIST_REG: c_uint = 0x1140080;

pub const EFL_TOP_BIST_STAT: c_uint = 0x1241090;
pub const BMO_BIST_REG: c_uint = 0x1180080;
pub const LBC_BIST_STATUS: c_uint = 0x1200020;

// EMU registers

// UCD registers

pub const UCD_UCODE_LOAD_BLOCK_NUM: c_uint = 0x12C0010;

// AQM registers
pub const AQM_CTL: c_uint = 0x1300000;
pub const AQM_INT: c_uint = 0x1300008;
pub const AQM_DBELL_OVF_LO: c_uint = 0x1300010;
pub const AQM_DBELL_OVF_HI: c_uint = 0x1300018;
pub const AQM_DBELL_OVF_LO_W1S: c_uint = 0x1300020;
pub const AQM_DBELL_OVF_LO_ENA_W1C: c_uint = 0x1300028;
pub const AQM_DBELL_OVF_LO_ENA_W1S: c_uint = 0x1300030;
pub const AQM_DBELL_OVF_HI_W1S: c_uint = 0x1300038;
pub const AQM_DBELL_OVF_HI_ENA_W1C: c_uint = 0x1300040;
pub const AQM_DBELL_OVF_HI_ENA_W1S: c_uint = 0x1300048;
pub const AQM_DMA_RD_ERR_LO: c_uint = 0x1300050;
pub const AQM_DMA_RD_ERR_HI: c_uint = 0x1300058;
pub const AQM_DMA_RD_ERR_LO_W1S: c_uint = 0x1300060;
pub const AQM_DMA_RD_ERR_LO_ENA_W1C: c_uint = 0x1300068;
pub const AQM_DMA_RD_ERR_LO_ENA_W1S: c_uint = 0x1300070;
pub const AQM_DMA_RD_ERR_HI_W1S: c_uint = 0x1300078;
pub const AQM_DMA_RD_ERR_HI_ENA_W1C: c_uint = 0x1300080;
pub const AQM_DMA_RD_ERR_HI_ENA_W1S: c_uint = 0x1300088;
pub const AQM_EXEC_NA_LO: c_uint = 0x1300090;
pub const AQM_EXEC_NA_HI: c_uint = 0x1300098;
pub const AQM_EXEC_NA_LO_W1S: c_uint = 0x13000A0;
pub const AQM_EXEC_NA_LO_ENA_W1C: c_uint = 0x13000A8;
pub const AQM_EXEC_NA_LO_ENA_W1S: c_uint = 0x13000B0;
pub const AQM_EXEC_NA_HI_W1S: c_uint = 0x13000B8;
pub const AQM_EXEC_NA_HI_ENA_W1C: c_uint = 0x13000C0;
pub const AQM_EXEC_NA_HI_ENA_W1S: c_uint = 0x13000C8;
pub const AQM_EXEC_ERR_LO: c_uint = 0x13000D0;
pub const AQM_EXEC_ERR_HI: c_uint = 0x13000D8;
pub const AQM_EXEC_ERR_LO_W1S: c_uint = 0x13000E0;
pub const AQM_EXEC_ERR_LO_ENA_W1C: c_uint = 0x13000E8;
pub const AQM_EXEC_ERR_LO_ENA_W1S: c_uint = 0x13000F0;
pub const AQM_EXEC_ERR_HI_W1S: c_uint = 0x13000F8;
pub const AQM_EXEC_ERR_HI_ENA_W1C: c_uint = 0x1300100;
pub const AQM_EXEC_ERR_HI_ENA_W1S: c_uint = 0x1300108;
pub const AQM_ECC_INT: c_uint = 0x1300110;
pub const AQM_ECC_INT_W1S: c_uint = 0x1300118;
pub const AQM_ECC_INT_ENA_W1C: c_uint = 0x1300120;
pub const AQM_ECC_INT_ENA_W1S: c_uint = 0x1300128;
pub const AQM_ECC_CTL: c_uint = 0x1300130;
pub const AQM_BIST_STATUS: c_uint = 0x1300138;

pub const AQM_ACTIVITY_STAT_LO: c_uint = 0x1300C80;
pub const AQM_ACTIVITY_STAT_HI: c_uint = 0x1300C88;

pub const AQM_PERF_CTL_LO: c_uint = 0x1301400;
pub const AQM_PERF_CTL_HI: c_uint = 0x1301408;
pub const AQM_PERF_CNT: c_uint = 0x1301410;

// NPS core registers
pub const NPS_CORE_GBL_VFCFG: c_uint = 0x1000000;
pub const NPS_CORE_CONTROL: c_uint = 0x1000008;
pub const NPS_CORE_INT_ACTIVE: c_uint = 0x1000080;
pub const NPS_CORE_INT: c_uint = 0x10000A0;
pub const NPS_CORE_INT_ENA_W1S: c_uint = 0x10000B8;
pub const NPS_STATS_PKT_DMA_RD_CNT: c_uint = 0x1000180;
pub const NPS_STATS_PKT_DMA_WR_CNT: c_uint = 0x1000190;
// NPS packet registers
pub const NPS_PKT_INT: c_uint = 0x1040018;
pub const NPS_PKT_MBOX_INT_LO: c_uint = 0x1040020;
pub const NPS_PKT_MBOX_INT_LO_ENA_W1C: c_uint = 0x1040030;
pub const NPS_PKT_MBOX_INT_LO_ENA_W1S: c_uint = 0x1040038;
pub const NPS_PKT_MBOX_INT_HI: c_uint = 0x1040040;
pub const NPS_PKT_MBOX_INT_HI_ENA_W1C: c_uint = 0x1040050;
pub const NPS_PKT_MBOX_INT_HI_ENA_W1S: c_uint = 0x1040058;
pub const NPS_PKT_IN_RERR_HI: c_uint = 0x1040108;
pub const NPS_PKT_IN_RERR_HI_ENA_W1S: c_uint = 0x1040120;
pub const NPS_PKT_IN_RERR_LO: c_uint = 0x1040128;
pub const NPS_PKT_IN_RERR_LO_ENA_W1S: c_uint = 0x1040140;
pub const NPS_PKT_IN_ERR_TYPE: c_uint = 0x1040148;
pub const NPS_PKT_IN_ERR_TYPE_ENA_W1S: c_uint = 0x1040160;

pub const NPS_PKT_SLC_RERR_HI: c_uint = 0x1040208;
pub const NPS_PKT_SLC_RERR_HI_ENA_W1S: c_uint = 0x1040220;
pub const NPS_PKT_SLC_RERR_LO: c_uint = 0x1040228;
pub const NPS_PKT_SLC_RERR_LO_ENA_W1S: c_uint = 0x1040240;
pub const NPS_PKT_SLC_ERR_TYPE: c_uint = 0x1040248;
pub const NPS_PKT_SLC_ERR_TYPE_ENA_W1S: c_uint = 0x1040260;
// Mailbox PF->VF PF Accessible Data registers

// POM registers
pub const POM_INT_ENA_W1S: c_uint = 0x11C0018;

pub const POM_INT: c_uint = 0x11C0000;
pub const POM_PERF_CTL: c_uint = 0x11CC400;
// BMI registers
pub const BMI_INT: c_uint = 0x1140000;
pub const BMI_CTL: c_uint = 0x1140020;
pub const BMI_INT_ENA_W1S: c_uint = 0x1140018;
pub const BMI_NPS_PKT_CNT: c_uint = 0x1140070;
// EFL registers

pub const EFL_RNM_CTL_STATUS: c_uint = 0x1241800;

// BMO registers
pub const BMO_CTL2: c_uint = 0x1180028;
pub const BMO_NPS_SLC_PKT_CNT: c_uint = 0x1180078;
// LBC registers
pub const LBC_INT: c_uint = 0x1200000;
pub const LBC_INVAL_CTL: c_uint = 0x1201010;
pub const LBC_PLM_VF1_64_INT: c_uint = 0x1202008;
pub const LBC_INVAL_STATUS: c_uint = 0x1202010;
pub const LBC_INT_ENA_W1S: c_uint = 0x1203000;
pub const LBC_PLM_VF1_64_INT_ENA_W1S: c_uint = 0x1205008;
pub const LBC_PLM_VF65_128_INT: c_uint = 0x1206008;
pub const LBC_ELM_VF1_64_INT: c_uint = 0x1208000;
pub const LBC_PLM_VF65_128_INT_ENA_W1S: c_uint = 0x1209008;
pub const LBC_ELM_VF1_64_INT_ENA_W1S: c_uint = 0x120B000;
pub const LBC_ELM_VF65_128_INT: c_uint = 0x120C000;
pub const LBC_ELM_VF65_128_INT_ENA_W1S: c_uint = 0x120F000;
pub const RST_BOOT: c_uint = 0x10C1600;
pub const FUS_DAT1: c_uint = 0x10C1408;
// PEM registers
pub const PEM0_INT: c_uint = 0x1080428;
//
// struct ucd_core_eid_ucode_block_num - Core Eid to Ucode Blk Mapping Registers
// @ucode_len: Ucode length identifier 32KB or 64KB
// @ucode_blk: Ucode Block Number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ucd_core_eid_ucode_block_num {
    pub value: u64,

    pub 60: u64 raz_4_63 :,
    pub 1: u64 ucode_len :,
    pub 3: u64 ucode_blk :,

    pub 3: u64 ucode_blk :,
    pub 1: u64 ucode_len :,
    pub 60: u64 raz_4_63 :,

}

//
// struct aqm_grp_execmsk_lo - Available AE engines for the group
// @exec_0_to_39: AE engines 0 to 39 status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqm_grp_execmsk_lo {
    pub value: u64,

    pub 24: u64 raz_40_63 :,
    pub 40: u64 exec_0_to_39 :,

    pub 40: u64 exec_0_to_39 :,
    pub 24: u64 raz_40_63 :,

}

//
// struct aqm_grp_execmsk_hi - Available AE engines for the group
// @exec_40_to_79: AE engines 40 to 79 status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqm_grp_execmsk_hi {
    pub value: u64,

    pub 24: u64 raz_40_63 :,
    pub 40: u64 exec_40_to_79 :,

    pub 40: u64 exec_40_to_79 :,
    pub 24: u64 raz_40_63 :,

}

//
// struct aqmq_drbl - AQM Queue Doorbell Counter Registers
// @dbell_count: Doorbell Counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_drbl {
    pub value: u64,

    pub 32: u64 raz_32_63 :,
    pub 32: u64 dbell_count :,

    pub 32: u64 dbell_count :,
    pub 32: u64 raz_32_63 :,

}

//
// struct aqmq_qsz - AQM Queue Host Queue Size Registers
// @host_queue_size: Size, in numbers of 'aqmq_command_s' command
// of the Host Ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_qsz {
    pub value: u64,

    pub 32: u64 raz_32_63 :,
    pub 32: u64 host_queue_size :,

    pub 32: u64 host_queue_size :,
    pub 32: u64 raz_32_63 :,

}

//
// struct aqmq_cmp_thr - AQM Queue Commands Completed Threshold Registers
// @commands_completed_threshold: Count of 'aqmq_command_s' commands executed
// by AE engines for which completion interrupt is asserted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_cmp_thr {
    pub value: u64,

    pub 32: u64 raz_32_63 :,
    pub 32: u64 commands_completed_threshold :,

    pub 32: u64 commands_completed_threshold :,
    pub 32: u64 raz_32_63 :,

}

//
// struct aqmq_cmp_cnt - AQM Queue Commands Completed Count Registers
// @resend: Bit to request completion interrupt Resend.
// @completion_status: Command completion status of the ring.
// @commands_completed_count: Count of 'aqmq_command_s' commands executed by
// AE engines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_cmp_cnt {
    pub value: u64,

    pub 30: u64 raz_34_63 :,
    pub 1: u64 resend :,
    pub 1: u64 completion_status :,
    pub 32: u64 commands_completed_count :,

    pub 32: u64 commands_completed_count :,
    pub 1: u64 completion_status :,
    pub 1: u64 resend :,
    pub 30: u64 raz_34_63 :,

}

//
// struct aqmq_en - AQM Queue Enable Registers
// @queue_status: 1 = AQMQ is enabled, 0 = AQMQ is disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_en {
    pub value: u64,

    pub 63: u64 raz_1_63 :,
    pub 1: u64 queue_enable :,

    pub 1: u64 queue_enable :,
    pub 63: u64 raz_1_63 :,

}

//
// struct aqmq_activity_stat - AQM Queue Activity Status Registers
// @queue_active: 1 = AQMQ is active, 0 = AQMQ is quiescent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aqmq_activity_stat {
    pub value: u64,

    pub 63: u64 raz_1_63 :,
    pub 1: u64 queue_active :,

    pub 1: u64 queue_active :,
    pub 63: u64 raz_1_63 :,

}

//
// struct emu_fuse_map - EMU Fuse Map Registers
// @ae_fuse: Fuse settings for AE 19..0
// @se_fuse: Fuse settings for SE 15..0
//
// A set bit indicates the unit is fuse disabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union emu_fuse_map {
    pub value: u64,

    pub 1: u64 valid :,
    pub 11: u64 raz_52_62 :,
    pub 20: u64 ae_fuse :,
    pub 16: u64 raz_16_31 :,
    pub 16: u64 se_fuse :,

    pub 16: u64 se_fuse :,
    pub 16: u64 raz_16_31 :,
    pub 20: u64 ae_fuse :,
    pub 11: u64 raz_52_62 :,
    pub 1: u64 valid :,

    pub s: },
}

//
// struct emu_se_enable - Symmetric Engine Enable Registers
// @enable: Individual enables for each of the clusters
// 16 symmetric engines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union emu_se_enable {
    pub value: u64,

    pub 48: u64 raz :,
    pub 16: u64 enable :,

    pub 16: u64 enable :,
    pub 48: u64 raz :,

    pub s: },
}

//
// struct emu_ae_enable - EMU Asymmetric engines.
// @enable: Individual enables for each of the cluster's
// 20 Asymmetric Engines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union emu_ae_enable {
    pub value: u64,

    pub 44: u64 raz :,
    pub 20: u64 enable :,

    pub 20: u64 enable :,
    pub 44: u64 raz :,

    pub s: },
}

//
// struct emu_wd_int_ena_w1s - EMU Interrupt Enable Registers
// @ae_wd: Reads or sets enable for EMU(0..3)_WD_INT[AE_WD]
// @se_wd: Reads or sets enable for EMU(0..3)_WD_INT[SE_WD]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union emu_wd_int_ena_w1s {
    pub value: u64,

    pub 12: u64 raz2 :,
    pub 20: u64 ae_wd :,
    pub 16: u64 raz1 :,
    pub 16: u64 se_wd :,

    pub 16: u64 se_wd :,
    pub 16: u64 raz1 :,
    pub 20: u64 ae_wd :,
    pub 12: u64 raz2 :,

    pub s: },
}

//
// struct emu_ge_int_ena_w1s - EMU Interrupt Enable set registers
// @ae_ge: Reads or sets enable for EMU(0..3)_GE_INT[AE_GE]
// @se_ge: Reads or sets enable for EMU(0..3)_GE_INT[SE_GE]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union emu_ge_int_ena_w1s {
    pub value: u64,

    pub 12: u64 raz_52_63 :,
    pub 20: u64 ae_ge :,
    pub 16: u64 raz_16_31:,
    pub 16: u64 se_ge :,

    pub 16: u64 se_ge :,
    pub 16: u64 raz_16_31:,
    pub 20: u64 ae_ge :,
    pub 12: u64 raz_52_63 :,

    pub s: },
}

//
// struct nps_pkt_slc_ctl - Solicited Packet Out Control Registers
// @rh: Indicates whether to remove or include the response header
// 1 = Include, 0 = Remove
// @z: If set, 8 trailing 0x00 bytes will be added to the end of the
// outgoing packet.
// @enb: Enable for this port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_slc_ctl {
    pub value: u64,

    pub 61: u64 raz :,
    pub 1: u64 rh :,
    pub 1: u64 z :,
    pub 1: u64 enb :,

    pub 1: u64 enb :,
    pub 1: u64 z :,
    pub 1: u64 rh :,
    pub 61: u64 raz :,

    pub s: },
}

//
// struct nps_pkt_slc_cnts - Solicited Packet Out Count Registers
// @slc_int: Returns a 1 when:
// NPS_PKT_SLC(i)_CNTS[CNT] > NPS_PKT_SLC(i)_INT_LEVELS[CNT], or
// NPS_PKT_SLC(i)_CNTS[TIMER] > NPS_PKT_SLC(i)_INT_LEVELS[TIMET].
// To clear the bit, the CNTS register must be written to clear.
// @in_int: Returns a 1 when:
// NPS_PKT_IN(i)_DONE_CNTS[CNT] > NPS_PKT_IN(i)_INT_LEVELS[CNT].
// To clear the bit, the DONE_CNTS register must be written to clear.
// @mbox_int: Returns a 1 when:
// NPS_PKT_MBOX_PF_VF(i)_INT[INTR] is set. To clear the bit,
// write NPS_PKT_MBOX_PF_VF(i)_INT[INTR] with 1.
// @timer: Timer, incremented every 2048 coprocessor clock cycles
// when [CNT] is not zero. The hardware clears both [TIMER] and
// [INT] when [CNT] goes to 0.
// @cnt: Packet counter. Hardware adds to [CNT] as it sends packets out.
// On a write to this CSR, hardware subtracts the amount written to the
// [CNT] field from [CNT].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_slc_cnts {
    pub value: u64,

    pub 1: u64 slc_int :,
    pub 1: u64 uns_int :,
    pub 1: u64 in_int :,
    pub 1: u64 mbox_int :,
    pub 1: u64 resend :,
    pub 5: u64 raz :,
    pub 22: u64 timer :,
    pub 32: u64 cnt :,

    pub 32: u64 cnt :,
    pub 22: u64 timer :,
    pub 5: u64 raz :,
    pub 1: u64 resend :,
    pub 1: u64 mbox_int :,
    pub 1: u64 in_int :,
    pub 1: u64 uns_int :,
    pub 1: u64 slc_int :,

    pub s: },
}

//
// struct nps_pkt_slc_int_levels - Solicited Packet Out Interrupt Levels
// Registers.
// @bmode: Determines whether NPS_PKT_SLC_CNTS[CNT] is a byte or
// packet counter.
// @timet: Output port counter time interrupt threshold.
// @cnt: Output port counter interrupt threshold.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_slc_int_levels {
    pub value: u64,

    pub 1: u64 bmode :,
    pub 9: u64 raz :,
    pub 22: u64 timet :,
    pub 32: u64 cnt :,

    pub 32: u64 cnt :,
    pub 22: u64 timet :,
    pub 9: u64 raz :,
    pub 1: u64 bmode :,

    pub s: },
}

//
// struct nps_pkt_inst - NPS Packet Interrupt Register
// @in_err: Set when any NPS_PKT_IN_RERR_HI/LO bit and
// corresponding NPS_PKT_IN_RERR_*_ENA_* bit are bot set.
// @uns_err: Set when any NSP_PKT_UNS_RERR_HI/LO bit and
// corresponding NPS_PKT_UNS_RERR_*_ENA_* bit are both set.
// @slc_er: Set when any NSP_PKT_SLC_RERR_HI/LO bit and
// corresponding NPS_PKT_SLC_RERR_*_ENA_* bit are both set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_int {
    pub value: u64,

    pub 54: u64 raz :,
    pub 1: u64 uns_wto :,
    pub 1: u64 in_err :,
    pub 1: u64 uns_err :,
    pub 1: u64 slc_err :,
    pub 1: u64 in_dbe :,
    pub 1: u64 in_sbe :,
    pub 1: u64 uns_dbe :,
    pub 1: u64 uns_sbe :,
    pub 1: u64 slc_dbe :,
    pub 1: u64 slc_sbe :,

    pub 1: u64 slc_sbe :,
    pub 1: u64 slc_dbe :,
    pub 1: u64 uns_sbe :,
    pub 1: u64 uns_dbe :,
    pub 1: u64 in_sbe :,
    pub 1: u64 in_dbe :,
    pub 1: u64 slc_err :,
    pub 1: u64 uns_err :,
    pub 1: u64 in_err :,
    pub 1: u64 uns_wto :,
    pub 54: u64 raz :,

    pub s: },
}

//
// struct nps_pkt_in_done_cnts - Input instruction ring counts registers
// @slc_cnt: Returns a 1 when:
// NPS_PKT_SLC(i)_CNTS[CNT] > NPS_PKT_SLC(i)_INT_LEVELS[CNT], or
// NPS_PKT_SLC(i)_CNTS[TIMER] > NPS_PKT_SCL(i)_INT_LEVELS[TIMET]
// To clear the bit, the CNTS register must be
// written to clear the underlying condition
// @uns_int: Return a 1 when:
// NPS_PKT_UNS(i)_CNTS[CNT] > NPS_PKT_UNS(i)_INT_LEVELS[CNT], or
// NPS_PKT_UNS(i)_CNTS[TIMER] > NPS_PKT_UNS(i)_INT_LEVELS[TIMET]
// To clear the bit, the CNTS register must be
// written to clear the underlying condition
// @in_int: Returns a 1 when:
// NPS_PKT_IN(i)_DONE_CNTS[CNT] > NPS_PKT_IN(i)_INT_LEVELS[CNT]
// To clear the bit, the DONE_CNTS register
// must be written to clear the underlying condition
// @mbox_int: Returns a 1 when:
// NPS_PKT_MBOX_PF_VF(i)_INT[INTR] is set.
// To clear the bit, write NPS_PKT_MBOX_PF_VF(i)_INT[INTR]
// with 1.
// @resend: A write of 1 will resend an MSI-X interrupt message if any
// of the following conditions are true for this ring "i".
// NPS_PKT_SLC(i)_CNTS[CNT] > NPS_PKT_SLC(i)_INT_LEVELS[CNT]
// NPS_PKT_SLC(i)_CNTS[TIMER] > NPS_PKT_SLC(i)_INT_LEVELS[TIMET]
// NPS_PKT_UNS(i)_CNTS[CNT] > NPS_PKT_UNS(i)_INT_LEVELS[CNT]
// NPS_PKT_UNS(i)_CNTS[TIMER] > NPS_PKT_UNS(i)_INT_LEVELS[TIMET]
// NPS_PKT_IN(i)_DONE_CNTS[CNT] > NPS_PKT_IN(i)_INT_LEVELS[CNT]
// NPS_PKT_MBOX_PF_VF(i)_INT[INTR] is set
// @cnt: Packet counter. Hardware adds to [CNT] as it reads
// packets. On a write to this CSR, hardware substracts the
// amount written to the [CNT] field from [CNT], which will
// clear PKT_IN(i)_INT_STATUS[INTR] if [CNT] becomes <=
// NPS_PKT_IN(i)_INT_LEVELS[CNT]. This register should be
// cleared before enabling a ring by reading the current
// value and writing it back.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_in_done_cnts {
    pub value: u64,

    pub 1: u64 slc_int :,
    pub 1: u64 uns_int :,
    pub 1: u64 in_int :,
    pub 1: u64 mbox_int :,
    pub 1: u64 resend :,
    pub 27: u64 raz :,
    pub 32: u64 cnt :,

    pub 32: u64 cnt :,
    pub 27: u64 raz :,
    pub 1: u64 resend :,
    pub 1: u64 mbox_int :,
    pub 1: u64 in_int :,
    pub 1: u64 uns_int :,
    pub 1: u64 slc_int :,

    pub s: },
}

//
// struct nps_pkt_in_instr_ctl - Input Instruction Ring Control Registers.
// @is64b: If 1, the ring uses 64-byte instructions. If 0, the
// ring uses 32-byte instructions.
// @enb: Enable for the input ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_in_instr_ctl {
    pub value: u64,

    pub 62: u64 raz :,
    pub 1: u64 is64b :,
    pub 1: u64 enb :,

    pub 1: u64 enb :,
    pub 1: u64 is64b :,
    pub 62: u64 raz :,

    pub s: },
}

//
// struct nps_pkt_in_instr_rsize - Input instruction ring size registers
// @rsize: Ring size (number of instructions)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_in_instr_rsize {
    pub value: u64,

    pub 32: u64 raz :,
    pub 32: u64 rsize :,

    pub 32: u64 rsize :,
    pub 32: u64 raz :,

    pub s: },
}

//
// struct nps_pkt_in_instr_baoff_dbell - Input instruction ring
// base address offset and doorbell registers
// @aoff: Address offset. The offset from the NPS_PKT_IN_INSTR_BADDR
// where the next pointer is read.
// @dbell: Pointer list doorbell count. Write operations to this field
// increments the present value here. Read operations return the
// present value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_pkt_in_instr_baoff_dbell {
    pub value: u64,

    pub 32: u64 aoff :,
    pub 32: u64 dbell :,

    pub 32: u64 dbell :,
    pub 32: u64 aoff :,

    pub s: },
}

//
// struct nps_core_int_ena_w1s - NPS core interrupt enable set register
// @host_nps_wr_err: Reads or sets enable for
// NPS_CORE_INT[HOST_NPS_WR_ERR].
// @npco_dma_malform: Reads or sets enable for
// NPS_CORE_INT[NPCO_DMA_MALFORM].
// @exec_wr_timeout: Reads or sets enable for
// NPS_CORE_INT[EXEC_WR_TIMEOUT].
// @host_wr_timeout: Reads or sets enable for
// NPS_CORE_INT[HOST_WR_TIMEOUT].
// @host_wr_err: Reads or sets enable for
// NPS_CORE_INT[HOST_WR_ERR]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_core_int_ena_w1s {
    pub value: u64,

    pub 55: u64 raz4 :,
    pub 1: u64 host_nps_wr_err :,
    pub 1: u64 npco_dma_malform :,
    pub 1: u64 exec_wr_timeout :,
    pub 1: u64 host_wr_timeout :,
    pub 1: u64 host_wr_err :,
    pub 1: u64 raz3 :,
    pub 1: u64 raz2 :,
    pub 1: u64 raz1 :,
    pub 1: u64 raz0 :,

    pub 1: u64 raz0 :,
    pub 1: u64 raz1 :,
    pub 1: u64 raz2 :,
    pub 1: u64 raz3 :,
    pub 1: u64 host_wr_err :,
    pub 1: u64 host_wr_timeout :,
    pub 1: u64 exec_wr_timeout :,
    pub 1: u64 npco_dma_malform :,
    pub 1: u64 host_nps_wr_err :,
    pub 55: u64 raz4 :,

    pub s: },
}

//
// struct nps_core_gbl_vfcfg - Global VF Configuration Register.
// @ilk_disable: When set, this bit indicates that the ILK interface has
// been disabled.
// @obaf: BMO allocation control
// 0 = allocate per queue
// 1 = allocate per VF
// @ibaf: BMI allocation control
// 0 = allocate per queue
// 1 = allocate per VF
// @zaf: ZIP allocation control
// 0 = allocate per queue
// 1 = allocate per VF
// @aeaf: AE allocation control
// 0 = allocate per queue
// 1 = allocate per VF
// @seaf: SE allocation control
// 0 = allocation per queue
// 1 = allocate per VF
// @cfg: VF/PF mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_core_gbl_vfcfg {
    pub value: u64,

    pub :55: u64 raz,
    pub :1: u64 ilk_disable,
    pub :1: u64 obaf,
    pub :1: u64 ibaf,
    pub :1: u64 zaf,
    pub :1: u64 aeaf,
    pub :1: u64 seaf,
    pub :3: u64 cfg,

    pub :3: u64 cfg,
    pub :1: u64 seaf,
    pub :1: u64 aeaf,
    pub :1: u64 zaf,
    pub :1: u64 ibaf,
    pub :1: u64 obaf,
    pub :1: u64 ilk_disable,
    pub :55: u64 raz,

    pub s: },
}

//
// struct nps_core_int_active - NPS Core Interrupt Active Register
// @resend: Resend MSI-X interrupt if needs to handle interrupts
// Sofware can set this bit and then exit the ISR.
// @ocla: Set when any OCLA(0)_INT and corresponding OCLA(0_INT_ENA_W1C
// bit are set
// @mbox: Set when any NPS_PKT_MBOX_INT_LO/HI and corresponding
// NPS_PKT_MBOX_INT_LO_ENA_W1C/HI_ENA_W1C bits are set
// @emu: bit i is set in [EMU] when any EMU(i)_INT bit is set
// @bmo: Set when any BMO_INT bit is set
// @bmi: Set when any BMI_INT bit is set or when any non-RO
// BMI_INT and corresponding BMI_INT_ENA_W1C bits are both set
// @aqm: Set when any AQM_INT bit is set
// @zqm: Set when any ZQM_INT bit is set
// @efl: Set when any EFL_INT RO bit is set or when any non-RO EFL_INT
// and corresponding EFL_INT_ENA_W1C bits are both set
// @ilk: Set when any ILK_INT bit is set
// @lbc: Set when any LBC_INT RO bit is set or when any non-RO LBC_INT
// and corresponding LBC_INT_ENA_W1C bits are bot set
// @pem: Set when any PEM(0)_INT RO bit is set or when any non-RO
// PEM(0)_INT and corresponding PEM(0)_INT_ENA_W1C bit are both set
// @ucd: Set when any UCD_INT bit is set
// @zctl: Set when any ZIP_INT RO bit is set or when any non-RO ZIP_INT
// and corresponding ZIP_INT_ENA_W1C bits are both set
// @lbm: Set when any LBM_INT bit is set
// @nps_pkt: Set when any NPS_PKT_INT bit is set
// @nps_core: Set when any NPS_CORE_INT RO bit is set or when non-RO
// NPS_CORE_INT and corresponding NSP_CORE_INT_ENA_W1C bits are both set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nps_core_int_active {
    pub value: u64,

    pub 1: u64 resend :,
    pub 43: u64 raz :,
    pub 1: u64 ocla :,
    pub 1: u64 mbox :,
    pub 4: u64 emu :,
    pub 1: u64 bmo :,
    pub 1: u64 bmi :,
    pub 1: u64 aqm :,
    pub 1: u64 zqm :,
    pub 1: u64 efl :,
    pub 1: u64 ilk :,
    pub 1: u64 lbc :,
    pub 1: u64 pem :,
    pub 1: u64 pom :,
    pub 1: u64 ucd :,
    pub 1: u64 zctl :,
    pub 1: u64 lbm :,
    pub 1: u64 nps_pkt :,
    pub 1: u64 nps_core :,

    pub 1: u64 nps_core :,
    pub 1: u64 nps_pkt :,
    pub 1: u64 lbm :,
    pub 1: u64 zctl:,
    pub 1: u64 ucd :,
    pub 1: u64 pom :,
    pub 1: u64 pem :,
    pub 1: u64 lbc :,
    pub 1: u64 ilk :,
    pub 1: u64 efl :,
    pub 1: u64 zqm :,
    pub 1: u64 aqm :,
    pub 1: u64 bmi :,
    pub 1: u64 bmo :,
    pub 4: u64 emu :,
    pub 1: u64 mbox :,
    pub 1: u64 ocla :,
    pub 43: u64 raz :,
    pub 1: u64 resend :,

    pub s: },
}

//
// struct efl_core_int - EFL Interrupt Registers
// @epci_decode_err: EPCI decoded a transacation that was unknown
// This error should only occurred when there is a micrcode/SE error
// and should be considered fatal
// @ae_err: An AE uncorrectable error occurred.
// See EFL_CORE(0..3)_AE_ERR_INT
// @se_err: An SE uncorrectable error occurred.
// See EFL_CORE(0..3)_SE_ERR_INT
// @dbe: Double-bit error occurred in EFL
// @sbe: Single-bit error occurred in EFL
// @d_left: Asserted when new POM-Header-BMI-data is
// being sent to an Exec, and that Exec has Not read all BMI
// data associated with the previous POM header
// @len_ovr: Asserted when an Exec-Read is issued that is more than
// 14 greater in length that the BMI data left to be read
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union efl_core_int {
    pub value: u64,

    pub 57: u64 raz :,
    pub 1: u64 epci_decode_err :,
    pub 1: u64 ae_err :,
    pub 1: u64 se_err :,
    pub 1: u64 dbe :,
    pub 1: u64 sbe :,
    pub 1: u64 d_left :,
    pub 1: u64 len_ovr :,

    pub 1: u64 len_ovr :,
    pub 1: u64 d_left :,
    pub 1: u64 sbe :,
    pub 1: u64 dbe :,
    pub 1: u64 se_err :,
    pub 1: u64 ae_err :,
    pub 1: u64 epci_decode_err :,
    pub 57: u64 raz :,

    pub s: },
}

//
// struct efl_core_int_ena_w1s - EFL core interrupt enable set register
// @epci_decode_err: Reads or sets enable for
// EFL_CORE(0..3)_INT[EPCI_DECODE_ERR].
// @d_left: Reads or sets enable for
// EFL_CORE(0..3)_INT[D_LEFT].
// @len_ovr: Reads or sets enable for
// EFL_CORE(0..3)_INT[LEN_OVR].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union efl_core_int_ena_w1s {
    pub value: u64,

    pub 57: u64 raz_7_63 :,
    pub 1: u64 epci_decode_err :,
    pub 4: u64 raz_2_5 :,
    pub 1: u64 d_left :,
    pub 1: u64 len_ovr :,

    pub 1: u64 len_ovr :,
    pub 1: u64 d_left :,
    pub 4: u64 raz_2_5 :,
    pub 1: u64 epci_decode_err :,
    pub 57: u64 raz_7_63 :,

    pub s: },
}

//
// struct efl_rnm_ctl_status - RNM Control and Status Register
// @ent_sel: Select input to RNM FIFO
// @exp_ent: Exported entropy enable for random number generator
// @rng_rst: Reset to RNG. Setting this bit to 1 cancels the generation
// of the current random number.
// @rnm_rst: Reset the RNM. Setting this bit to 1 clears all sorted numbers
// in the random number memory.
// @rng_en: Enabled the output of the RNG.
// @ent_en: Entropy enable for random number generator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union efl_rnm_ctl_status {
    pub value: u64,

    pub 55: u64 raz_9_63 :,
    pub 4: u64 ent_sel :,
    pub 1: u64 exp_ent :,
    pub 1: u64 rng_rst :,
    pub 1: u64 rnm_rst :,
    pub 1: u64 rng_en :,
    pub 1: u64 ent_en :,

    pub 1: u64 ent_en :,
    pub 1: u64 rng_en :,
    pub 1: u64 rnm_rst :,
    pub 1: u64 rng_rst :,
    pub 1: u64 exp_ent :,
    pub 4: u64 ent_sel :,
    pub 55: u64 raz_9_63 :,

    pub s: },
}

//
// struct bmi_ctl - BMI control register
// @ilk_hdrq_thrsh: Maximum number of header queue locations
// that ILK packets may consume. When the threshold is
// exceeded ILK_XOFF is sent to the BMI_X2P_ARB.
// @nps_hdrq_thrsh: Maximum number of header queue locations
// that NPS packets may consume. When the threshold is
// exceeded NPS_XOFF is sent to the BMI_X2P_ARB.
// @totl_hdrq_thrsh: Maximum number of header queue locations
// that the sum of ILK and NPS packets may consume.
// @ilk_free_thrsh: Maximum number of buffers that ILK packet
// flows may consume before ILK_XOFF is sent to the BMI_X2P_ARB.
// @nps_free_thrsh: Maximum number of buffers that NPS packet
// flows may consume before NPS XOFF is sent to the BMI_X2p_ARB.
// @totl_free_thrsh: Maximum number of buffers that bot ILK and NPS
// packet flows may consume before both NPS_XOFF and ILK_XOFF
// are asserted to the BMI_X2P_ARB.
// @max_pkt_len: Maximum packet length, integral number of 256B
// buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bmi_ctl {
    pub value: u64,

    pub 8: u64 raz_56_63 :,
    pub 8: u64 ilk_hdrq_thrsh :,
    pub 8: u64 nps_hdrq_thrsh :,
    pub 8: u64 totl_hdrq_thrsh :,
    pub 8: u64 ilk_free_thrsh :,
    pub 8: u64 nps_free_thrsh :,
    pub 8: u64 totl_free_thrsh :,
    pub 8: u64 max_pkt_len :,

    pub 8: u64 max_pkt_len :,
    pub 8: u64 totl_free_thrsh :,
    pub 8: u64 nps_free_thrsh :,
    pub 8: u64 ilk_free_thrsh :,
    pub 8: u64 totl_hdrq_thrsh :,
    pub 8: u64 nps_hdrq_thrsh :,
    pub 8: u64 ilk_hdrq_thrsh :,
    pub 8: u64 raz_56_63 :,

    pub s: },
}

//
// struct bmi_int_ena_w1s - BMI interrupt enable set register
// @ilk_req_oflw: Reads or sets enable for
// BMI_INT[ILK_REQ_OFLW].
// @nps_req_oflw: Reads or sets enable for
// BMI_INT[NPS_REQ_OFLW].
// @fpf_undrrn: Reads or sets enable for
// BMI_INT[FPF_UNDRRN].
// @eop_err_ilk: Reads or sets enable for
// BMI_INT[EOP_ERR_ILK].
// @eop_err_nps: Reads or sets enable for
// BMI_INT[EOP_ERR_NPS].
// @sop_err_ilk: Reads or sets enable for
// BMI_INT[SOP_ERR_ILK].
// @sop_err_nps: Reads or sets enable for
// BMI_INT[SOP_ERR_NPS].
// @pkt_rcv_err_ilk: Reads or sets enable for
// BMI_INT[PKT_RCV_ERR_ILK].
// @pkt_rcv_err_nps: Reads or sets enable for
// BMI_INT[PKT_RCV_ERR_NPS].
// @max_len_err_ilk: Reads or sets enable for
// BMI_INT[MAX_LEN_ERR_ILK].
// @max_len_err_nps: Reads or sets enable for
// BMI_INT[MAX_LEN_ERR_NPS].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bmi_int_ena_w1s {
    pub value: u64,

    pub 51: u64 raz_13_63 :,
    pub 1: u64 ilk_req_oflw :,
    pub 1: u64 nps_req_oflw :,
    pub 1: u64 raz_10 :,
    pub 1: u64 raz_9 :,
    pub 1: u64 fpf_undrrn :,
    pub 1: u64 eop_err_ilk :,
    pub 1: u64 eop_err_nps :,
    pub 1: u64 sop_err_ilk :,
    pub 1: u64 sop_err_nps :,
    pub 1: u64 pkt_rcv_err_ilk :,
    pub 1: u64 pkt_rcv_err_nps :,
    pub 1: u64 max_len_err_ilk :,
    pub 1: u64 max_len_err_nps :,

    pub 1: u64 max_len_err_nps :,
    pub 1: u64 max_len_err_ilk :,
    pub 1: u64 pkt_rcv_err_nps :,
    pub 1: u64 pkt_rcv_err_ilk :,
    pub 1: u64 sop_err_nps :,
    pub 1: u64 sop_err_ilk :,
    pub 1: u64 eop_err_nps :,
    pub 1: u64 eop_err_ilk :,
    pub 1: u64 fpf_undrrn :,
    pub 1: u64 raz_9 :,
    pub 1: u64 raz_10 :,
    pub 1: u64 nps_req_oflw :,
    pub 1: u64 ilk_req_oflw :,
    pub 51: u64 raz_13_63 :,

    pub s: },
}

//
// struct bmo_ctl2 - BMO Control2 Register
// @arb_sel: Determines P2X Arbitration
// @ilk_buf_thrsh: Maximum number of buffers that the
// ILK packet flows may consume before ILK XOFF is
// asserted to the POM.
// @nps_slc_buf_thrsh: Maximum number of buffers that the
// NPS_SLC packet flow may consume before NPS_SLC XOFF is
// asserted to the POM.
// @nps_uns_buf_thrsh: Maximum number of buffers that the
// NPS_UNS packet flow may consume before NPS_UNS XOFF is
// asserted to the POM.
// @totl_buf_thrsh: Maximum number of buffers that ILK, NPS_UNS and
// NPS_SLC packet flows may consume before NPS_UNS XOFF, NSP_SLC and
// ILK_XOFF are all asserted POM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bmo_ctl2 {
    pub value: u64,

    pub 1: u64 arb_sel :,
    pub 31: u64 raz_32_62 :,
    pub 8: u64 ilk_buf_thrsh :,
    pub 8: u64 nps_slc_buf_thrsh :,
    pub 8: u64 nps_uns_buf_thrsh :,
    pub 8: u64 totl_buf_thrsh :,

    pub 8: u64 totl_buf_thrsh :,
    pub 8: u64 nps_uns_buf_thrsh :,
    pub 8: u64 nps_slc_buf_thrsh :,
    pub 8: u64 ilk_buf_thrsh :,
    pub 31: u64 raz_32_62 :,
    pub 1: u64 arb_sel :,

    pub s: },
}

//
// struct pom_int_ena_w1s - POM interrupt enable set register
// @illegal_intf: Reads or sets enable for POM_INT[ILLEGAL_INTF].
// @illegal_dport: Reads or sets enable for POM_INT[ILLEGAL_DPORT].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pom_int_ena_w1s {
    pub value: u64,

    pub 60: u64 raz2 :,
    pub 1: u64 illegal_intf :,
    pub 1: u64 illegal_dport :,
    pub 1: u64 raz1 :,
    pub 1: u64 raz0 :,

    pub 1: u64 raz0 :,
    pub 1: u64 raz1 :,
    pub 1: u64 illegal_dport :,
    pub 1: u64 illegal_intf :,
    pub 60: u64 raz2 :,

    pub s: },
}

//
// struct lbc_inval_ctl - LBC invalidation control register
// @wait_timer: Wait timer for wait state. [WAIT_TIMER] must
// always be written with its reset value.
// @cam_inval_start: Software should write [CAM_INVAL_START]=1
// to initiate an LBC cache invalidation. After this, software
// should read LBC_INVAL_STATUS until LBC_INVAL_STATUS[DONE] is set.
// LBC hardware clears [CAVM_INVAL_START] before software can
// observed LBC_INVAL_STATUS[DONE] to be set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union lbc_inval_ctl {
    pub value: u64,

    pub 48: u64 raz2 :,
    pub 8: u64 wait_timer :,
    pub 6: u64 raz1 :,
    pub 1: u64 cam_inval_start :,
    pub 1: u64 raz0 :,

    pub 1: u64 raz0 :,
    pub 1: u64 cam_inval_start :,
    pub 6: u64 raz1 :,
    pub 8: u64 wait_timer :,
    pub 48: u64 raz2 :,

    pub s: },
}

//
// struct lbc_int_ena_w1s - LBC interrupt enable set register
// @cam_hard_err: Reads or sets enable for LBC_INT[CAM_HARD_ERR].
// @cam_inval_abort: Reads or sets enable for LBC_INT[CAM_INVAL_ABORT].
// @over_fetch_err: Reads or sets enable for LBC_INT[OVER_FETCH_ERR].
// @cache_line_to_err: Reads or sets enable for
// LBC_INT[CACHE_LINE_TO_ERR].
// @cam_soft_err: Reads or sets enable for
// LBC_INT[CAM_SOFT_ERR].
// @dma_rd_err: Reads or sets enable for
// LBC_INT[DMA_RD_ERR].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union lbc_int_ena_w1s {
    pub value: u64,

    pub 54: u64 raz_10_63 :,
    pub 1: u64 cam_hard_err :,
    pub 1: u64 cam_inval_abort :,
    pub 1: u64 over_fetch_err :,
    pub 1: u64 cache_line_to_err :,
    pub 4: u64 raz_2_5 :,
    pub 1: u64 cam_soft_err :,
    pub 1: u64 dma_rd_err :,

    pub 1: u64 dma_rd_err :,
    pub 1: u64 cam_soft_err :,
    pub 4: u64 raz_2_5 :,
    pub 1: u64 cache_line_to_err :,
    pub 1: u64 over_fetch_err :,
    pub 1: u64 cam_inval_abort :,
    pub 1: u64 cam_hard_err :,
    pub 54: u64 raz_10_63 :,

    pub s: },
}

//
// struct lbc_int - LBC interrupt summary register
// @cam_hard_err: indicates a fatal hardware error.
// It requires system reset.
// When [CAM_HARD_ERR] is set, LBC stops logging any new information in
// LBC_POM_MISS_INFO_LOG,
// LBC_POM_MISS_ADDR_LOG,
// LBC_EFL_MISS_INFO_LOG, and
// LBC_EFL_MISS_ADDR_LOG.
// Software should sample them.
// @cam_inval_abort: indicates a fatal hardware error.
// System reset is required.
// @over_fetch_err: indicates a fatal hardware error
// System reset is required
// @cache_line_to_err: is a debug feature.
// This timeout interrupt bit tells the software that
// a cacheline in LBC has non-zero usage and the context
// has not been used for greater than the
// LBC_TO_CNT[TO_CNT] time interval.
// @sbe: Memory SBE error. This is recoverable via ECC.
// See LBC_ECC_INT for more details.
// @dbe: Memory DBE error. This is a fatal and requires a
// system reset.
// @pref_dat_len_mismatch_err: Summary bit for context length
// mismatch errors.
// @rd_dat_len_mismatch_err: Summary bit for SE read data length
// greater than data prefect length errors.
// @cam_soft_err: is recoverable. Software must complete a
// LBC_INVAL_CTL[CAM_INVAL_START] invalidation sequence and
// then clear [CAM_SOFT_ERR].
// @dma_rd_err: A context prefect read of host memory returned with
// a read error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union lbc_int {
    pub value: u64,

    pub 54: u64 raz_10_63 :,
    pub 1: u64 cam_hard_err :,
    pub 1: u64 cam_inval_abort :,
    pub 1: u64 over_fetch_err :,
    pub 1: u64 cache_line_to_err :,
    pub 1: u64 sbe :,
    pub 1: u64 dbe :,
    pub 1: u64 pref_dat_len_mismatch_err :,
    pub 1: u64 rd_dat_len_mismatch_err :,
    pub 1: u64 cam_soft_err :,
    pub 1: u64 dma_rd_err :,

    pub 1: u64 dma_rd_err :,
    pub 1: u64 cam_soft_err :,
    pub 1: u64 rd_dat_len_mismatch_err :,
    pub 1: u64 pref_dat_len_mismatch_err :,
    pub 1: u64 dbe :,
    pub 1: u64 sbe :,
    pub 1: u64 cache_line_to_err :,
    pub 1: u64 over_fetch_err :,
    pub 1: u64 cam_inval_abort :,
    pub 1: u64 cam_hard_err :,
    pub 54: u64 raz_10_63 :,

    pub s: },
}

//
// struct lbc_inval_status: LBC Invalidation status register
// @cam_clean_entry_complete_cnt: The number of entries that are
// cleaned up successfully.
// @cam_clean_entry_cnt: The number of entries that have the CAM
// inval command issued.
// @cam_inval_state: cam invalidation FSM state
// @cam_inval_abort: cam invalidation abort
// @cam_rst_rdy: lbc_cam reset ready
// @done: LBC clears [DONE] when
// LBC_INVAL_CTL[CAM_INVAL_START] is written with a one,
// and sets [DONE] when it completes the invalidation
// sequence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union lbc_inval_status {
    pub value: u64,

    pub 23: u64 raz3 :,
    pub 9: u64 cam_clean_entry_complete_cnt :,
    pub 7: u64 raz2 :,
    pub 9: u64 cam_clean_entry_cnt :,
    pub 5: u64 raz1 :,
    pub 3: u64 cam_inval_state :,
    pub 5: u64 raz0 :,
    pub 1: u64 cam_inval_abort :,
    pub 1: u64 cam_rst_rdy :,
    pub 1: u64 done :,

    pub 1: u64 done :,
    pub 1: u64 cam_rst_rdy :,
    pub 1: u64 cam_inval_abort :,
    pub 5: u64 raz0 :,
    pub 3: u64 cam_inval_state :,
    pub 5: u64 raz1 :,
    pub 9: u64 cam_clean_entry_cnt :,
    pub 7: u64 raz2 :,
    pub 9: u64 cam_clean_entry_complete_cnt :,
    pub 23: u64 raz3 :,

    pub s: },
}

//
// struct rst_boot: RST Boot Register
// @jtcsrdis: when set, internal CSR access via JTAG TAP controller
// is disabled
// @jt_tst_mode: JTAG test mode
// @io_supply: I/O power supply setting based on IO_VDD_SELECT pin:
// 0x1 = 1.8V
// 0x2 = 2.5V
// 0x4 = 3.3V
// All other values are reserved
// @pnr_mul: clock multiplier
// @lboot: last boot cause mask, resets only with PLL_DC_OK
// @rboot: determines whether core 0 remains in reset after
// chip cold or warm or soft reset
// @rboot_pin: read only access to REMOTE_BOOT pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union rst_boot {
    pub value: u64,

    pub 1: u64 raz_63 :,
    pub 1: u64 jtcsrdis :,
    pub 3: u64 raz_59_61 :,
    pub 1: u64 jt_tst_mode :,
    pub 18: u64 raz_40_57 :,
    pub 3: u64 io_supply :,
    pub 7: u64 raz_30_36 :,
    pub 6: u64 pnr_mul :,
    pub 12: u64 raz_12_23 :,
    pub 10: u64 lboot :,
    pub 1: u64 rboot :,
    pub 1: u64 rboot_pin :,

    pub 1: u64 rboot_pin :,
    pub 1: u64 rboot :,
    pub 10: u64 lboot :,
    pub 12: u64 raz_12_23 :,
    pub 6: u64 pnr_mul :,
    pub 7: u64 raz_30_36 :,
    pub 3: u64 io_supply :,
    pub 18: u64 raz_40_57 :,
    pub 1: u64 jt_tst_mode :,
    pub 3: u64 raz_59_61 :,
    pub 1: u64 jtcsrdis :,
    pub 1: u64 raz_63 :,

}

//
// struct fus_dat1: Fuse Data 1 Register
// @pll_mul: main clock PLL multiplier hardware limit
// @pll_half_dis: main clock PLL control
// @efus_lck: efuse lockdown
// @zip_info: ZIP information
// @bar2_sz_conf: when zero, BAR2 size conforms to
// PCIe specification
// @efus_ign: efuse ignore
// @nozip: ZIP disable
// @pll_alt_matrix: select alternate PLL matrix
// @pll_bwadj_denom: select CLKF denominator for
// BWADJ value
// @chip_id: chip ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fus_dat1 {
    pub value: u64,

    pub 7: u64 raz_57_63 :,
    pub 3: u64 pll_mul :,
    pub 1: u64 pll_half_dis :,
    pub 10: u64 raz_43_52 :,
    pub 3: u64 efus_lck :,
    pub 14: u64 raz_26_39 :,
    pub 5: u64 zip_info :,
    pub 1: u64 bar2_sz_conf :,
    pub 1: u64 efus_ign :,
    pub 1: u64 nozip :,
    pub 7: u64 raz_11_17 :,
    pub 1: u64 pll_alt_matrix :,
    pub 2: u64 pll_bwadj_denom :,
    pub 8: u64 chip_id :,

    pub 8: u64 chip_id :,
    pub 2: u64 pll_bwadj_denom :,
    pub 1: u64 pll_alt_matrix :,
    pub 7: u64 raz_11_17 :,
    pub 1: u64 nozip :,
    pub 1: u64 efus_ign :,
    pub 1: u64 bar2_sz_conf :,
    pub 5: u64 zip_info :,
    pub 14: u64 raz_26_39 :,
    pub 3: u64 efus_lck :,
    pub 10: u64 raz_43_52 :,
    pub 1: u64 pll_half_dis :,
    pub 3: u64 pll_mul :,
    pub 7: u64 raz_57_63 :,

}
