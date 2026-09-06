//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_reg.h
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


// bnx2x_reg.h: Qlogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// The registers description starts with the register Access type followed
// by size in bits. For example [RW 32]. The access types are:
// R  - Read only
// RC - Clear on read
// RW - Read/Write
// ST - Statistics register (clear on read)
// W  - Write only
// WB - Wide bus register - the size is over 32 bits and it should be
// read/write in consecutive 32 bits accesses
// WR - Write Clear (write 1 to clear the bit)
//

// [RW 1] Initiate the ATC array - reset all the valid bits
pub const ATC_REG_ATC_INIT_ARRAY: c_uint = 0x1100b8;
// [R 1] ATC initialization done
pub const ATC_REG_ATC_INIT_DONE: c_uint = 0x1100bc;
// [RC 6] Interrupt register #0 read clear
pub const ATC_REG_ATC_INT_STS_CLR: c_uint = 0x1101c0;
// [RW 5] Parity mask register #0 read/write
pub const ATC_REG_ATC_PRTY_MASK: c_uint = 0x1101d8;
// [R 5] Parity register #0 read
pub const ATC_REG_ATC_PRTY_STS: c_uint = 0x1101cc;
// [RC 5] Parity register #0 read clear
pub const ATC_REG_ATC_PRTY_STS_CLR: c_uint = 0x1101d0;
// [RW 19] Interrupt mask register #0 read/write
pub const BRB1_REG_BRB1_INT_MASK: c_uint = 0x60128;
// [R 19] Interrupt register #0 read
pub const BRB1_REG_BRB1_INT_STS: c_uint = 0x6011c;
// [RW 4] Parity mask register #0 read/write
pub const BRB1_REG_BRB1_PRTY_MASK: c_uint = 0x60138;
// [R 4] Parity register #0 read
pub const BRB1_REG_BRB1_PRTY_STS: c_uint = 0x6012c;
// [RC 4] Parity register #0 read clear
pub const BRB1_REG_BRB1_PRTY_STS_CLR: c_uint = 0x60130;
// [RW 10] At address BRB1_IND_FREE_LIST_PRS_CRDT initialize free head. At
// address BRB1_IND_FREE_LIST_PRS_CRDT+1 initialize free tail. At address
// BRB1_IND_FREE_LIST_PRS_CRDT+2 initialize parser initial credit. Warning -
// following reset the first rbc access to this reg must be write; there can
// be no more rbc writes after the first one; there can be any number of rbc
// read following the first write; rbc access not following these rules will
// result in hang condition.
pub const BRB1_REG_FREE_LIST_PRS_CRDT: c_uint = 0x60200;
// [RW 10] The number of free blocks below which the full signal to class 0
// is asserted
pub const BRB1_REG_FULL_0_XOFF_THRESHOLD_0: c_uint = 0x601d0;
pub const BRB1_REG_FULL_0_XOFF_THRESHOLD_1: c_uint = 0x60230;
// [RW 11] The number of free blocks above which the full signal to class 0
// is de-asserted
pub const BRB1_REG_FULL_0_XON_THRESHOLD_0: c_uint = 0x601d4;
pub const BRB1_REG_FULL_0_XON_THRESHOLD_1: c_uint = 0x60234;
// [RW 11] The number of free blocks below which the full signal to class 1
// is asserted
pub const BRB1_REG_FULL_1_XOFF_THRESHOLD_0: c_uint = 0x601d8;
pub const BRB1_REG_FULL_1_XOFF_THRESHOLD_1: c_uint = 0x60238;
// [RW 11] The number of free blocks above which the full signal to class 1
// is de-asserted
pub const BRB1_REG_FULL_1_XON_THRESHOLD_0: c_uint = 0x601dc;
pub const BRB1_REG_FULL_1_XON_THRESHOLD_1: c_uint = 0x6023c;
// [RW 11] The number of free blocks below which the full signal to the LB
// port is asserted
pub const BRB1_REG_FULL_LB_XOFF_THRESHOLD: c_uint = 0x601e0;
// [RW 10] The number of free blocks above which the full signal to the LB
// port is de-asserted
pub const BRB1_REG_FULL_LB_XON_THRESHOLD: c_uint = 0x601e4;
// [RW 10] The number of free blocks above which the High_llfc signal to
pub const BRB1_REG_HIGH_LLFC_HIGH_THRESHOLD_0: c_uint = 0x6014c;
// [RW 10] The number of free blocks below which the High_llfc signal to
pub const BRB1_REG_HIGH_LLFC_LOW_THRESHOLD_0: c_uint = 0x6013c;
// [RW 11] The number of blocks guarantied for the LB port
pub const BRB1_REG_LB_GUARANTIED: c_uint = 0x601ec;
// [RW 11] The hysteresis on the guarantied buffer space for the Lb port
// before signaling XON.
pub const BRB1_REG_LB_GUARANTIED_HYST: c_uint = 0x60264;
// [RW 24] LL RAM data.
pub const BRB1_REG_LL_RAM: c_uint = 0x61000;
// [RW 10] The number of free blocks above which the Low_llfc signal to
pub const BRB1_REG_LOW_LLFC_HIGH_THRESHOLD_0: c_uint = 0x6016c;
// [RW 10] The number of free blocks below which the Low_llfc signal to
pub const BRB1_REG_LOW_LLFC_LOW_THRESHOLD_0: c_uint = 0x6015c;
// [RW 11] The number of blocks guarantied for class 0 in MAC 0. The
// register is applicable only when per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_0_CLASS_0_GUARANTIED: c_uint = 0x60244;
// [RW 11] The hysteresis on the guarantied buffer space for class 0 in MAC
// 1 before signaling XON. The register is applicable only when
// per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_0_CLASS_0_GUARANTIED_HYST: c_uint = 0x60254;
// [RW 11] The number of blocks guarantied for class 1 in MAC 0. The
// register is applicable only when per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_0_CLASS_1_GUARANTIED: c_uint = 0x60248;
// [RW 11] The hysteresis on the guarantied buffer space for class 1in MAC 0
// before signaling XON. The register is applicable only when
// per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_0_CLASS_1_GUARANTIED_HYST: c_uint = 0x60258;
// [RW 11] The number of blocks guarantied for class 0in MAC1.The register
// is applicable only when per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_1_CLASS_0_GUARANTIED: c_uint = 0x6024c;
// [RW 11] The hysteresis on the guarantied buffer space for class 0 in MAC
// 1 before signaling XON. The register is applicable only when
// per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_1_CLASS_0_GUARANTIED_HYST: c_uint = 0x6025c;
// [RW 11] The number of blocks guarantied for class 1 in MAC 1. The
// register is applicable only when per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_1_CLASS_1_GUARANTIED: c_uint = 0x60250;
// [RW 11] The hysteresis on the guarantied buffer space for class 1 in MAC
// 1 before signaling XON. The register is applicable only when
// per_class_guaranty_mode is set.
pub const BRB1_REG_MAC_1_CLASS_1_GUARANTIED_HYST: c_uint = 0x60260;
// [RW 11] The number of blocks guarantied for the MAC port. The register is
// applicable only when per_class_guaranty_mode is reset.
pub const BRB1_REG_MAC_GUARANTIED_0: c_uint = 0x601e8;
pub const BRB1_REG_MAC_GUARANTIED_1: c_uint = 0x60240;
// [R 24] The number of full blocks.
pub const BRB1_REG_NUM_OF_FULL_BLOCKS: c_uint = 0x60090;
// [ST 32] The number of cycles that the write_full signal towards MAC #0
pub const BRB1_REG_NUM_OF_FULL_CYCLES_0: c_uint = 0x600c8;
pub const BRB1_REG_NUM_OF_FULL_CYCLES_1: c_uint = 0x600cc;
pub const BRB1_REG_NUM_OF_FULL_CYCLES_4: c_uint = 0x600d8;
// [ST 32] The number of cycles that the pause signal towards MAC #0 was
pub const BRB1_REG_NUM_OF_PAUSE_CYCLES_0: c_uint = 0x600b8;
pub const BRB1_REG_NUM_OF_PAUSE_CYCLES_1: c_uint = 0x600bc;
// [RW 10] The number of free blocks below which the pause signal to class 0
// is asserted
pub const BRB1_REG_PAUSE_0_XOFF_THRESHOLD_0: c_uint = 0x601c0;
pub const BRB1_REG_PAUSE_0_XOFF_THRESHOLD_1: c_uint = 0x60220;
// [RW 11] The number of free blocks above which the pause signal to class 0
// is de-asserted
pub const BRB1_REG_PAUSE_0_XON_THRESHOLD_0: c_uint = 0x601c4;
pub const BRB1_REG_PAUSE_0_XON_THRESHOLD_1: c_uint = 0x60224;
// [RW 11] The number of free blocks below which the pause signal to class 1
// is asserted
pub const BRB1_REG_PAUSE_1_XOFF_THRESHOLD_0: c_uint = 0x601c8;
pub const BRB1_REG_PAUSE_1_XOFF_THRESHOLD_1: c_uint = 0x60228;
// [RW 11] The number of free blocks above which the pause signal to class 1
// is de-asserted
pub const BRB1_REG_PAUSE_1_XON_THRESHOLD_0: c_uint = 0x601cc;
pub const BRB1_REG_PAUSE_1_XON_THRESHOLD_1: c_uint = 0x6022c;
// [RW 10] Write client 0: De-assert pause threshold. Not Functional
pub const BRB1_REG_PAUSE_HIGH_THRESHOLD_0: c_uint = 0x60078;
pub const BRB1_REG_PAUSE_HIGH_THRESHOLD_1: c_uint = 0x6007c;
// [RW 10] Write client 0: Assert pause threshold.
pub const BRB1_REG_PAUSE_LOW_THRESHOLD_0: c_uint = 0x60068;
// [RW 1] Indicates if to use per-class guaranty mode (new mode) or per-MAC
// guaranty mode (backwards-compatible mode). 0=per-MAC guaranty mode (BC
// mode). 1=per-class guaranty mode (new mode).
pub const BRB1_REG_PER_CLASS_GUARANTY_MODE: c_uint = 0x60268;
// [R 24] The number of full blocks occpied by port.
pub const BRB1_REG_PORT_NUM_OCC_BLOCKS_0: c_uint = 0x60094;
// [RW 1] Reset the design by software.
pub const BRB1_REG_SOFT_RESET: c_uint = 0x600dc;
// [R 5] Used to read the value of the XX protection CAM occupancy counter.
pub const CCM_REG_CAM_OCCUP: c_uint = 0xd0188;
// [RW 1] CM - CFC Interface enable. If 0 - the valid input is disregarded;
pub const CCM_REG_CCM_CFC_IFEN: c_uint = 0xd003c;
// [RW 1] CM - QM Interface enable. If 0 - the acknowledge input is
pub const CCM_REG_CCM_CQM_IFEN: c_uint = 0xd000c;
// [RW 1] If set the Q index; received from the QM is inserted to event ID.
pub const CCM_REG_CCM_CQM_USE_Q: c_uint = 0xd00c0;
// [RW 11] Interrupt mask register #0 read/write
pub const CCM_REG_CCM_INT_MASK: c_uint = 0xd01e4;
// [R 11] Interrupt register #0 read
pub const CCM_REG_CCM_INT_STS: c_uint = 0xd01d8;
// [RW 27] Parity mask register #0 read/write
pub const CCM_REG_CCM_PRTY_MASK: c_uint = 0xd01f4;
// [R 27] Parity register #0 read
pub const CCM_REG_CCM_PRTY_STS: c_uint = 0xd01e8;
// [RC 27] Parity register #0 read clear
pub const CCM_REG_CCM_PRTY_STS_CLR: c_uint = 0xd01ec;
// [RW 3] The size of AG context region 0 in REG-pairs. Designates the MS
pub const CCM_REG_CCM_REG0_SZ: c_uint = 0xd00c4;
// [RW 1] CM - STORM 0 Interface enable. If 0 - the acknowledge input is
pub const CCM_REG_CCM_STORM0_IFEN: c_uint = 0xd0004;
// [RW 1] CM - STORM 1 Interface enable. If 0 - the acknowledge input is
pub const CCM_REG_CCM_STORM1_IFEN: c_uint = 0xd0008;
// [RW 1] CDU AG read Interface enable. If 0 - the request input is
pub const CCM_REG_CDU_AG_RD_IFEN: c_uint = 0xd0030;
// [RW 1] CDU AG write Interface enable. If 0 - the request and valid input
pub const CCM_REG_CDU_AG_WR_IFEN: c_uint = 0xd002c;
// [RW 1] CDU STORM read Interface enable. If 0 - the request input is
pub const CCM_REG_CDU_SM_RD_IFEN: c_uint = 0xd0038;
// [RW 1] CDU STORM write Interface enable. If 0 - the request and valid
pub const CCM_REG_CDU_SM_WR_IFEN: c_uint = 0xd0034;
// [RW 4] CFC output initial credit. Max credit available - 15.Write writes
pub const CCM_REG_CFC_INIT_CRD: c_uint = 0xd0204;
// [RW 2] Auxiliary counter flag Q number 1.
pub const CCM_REG_CNT_AUX1_Q: c_uint = 0xd00c8;
// [RW 2] Auxiliary counter flag Q number 2.
pub const CCM_REG_CNT_AUX2_Q: c_uint = 0xd00cc;
// [RW 28] The CM header value for QM request (primary).
pub const CCM_REG_CQM_CCM_HDR_P: c_uint = 0xd008c;
// [RW 28] The CM header value for QM request (secondary).
pub const CCM_REG_CQM_CCM_HDR_S: c_uint = 0xd0090;
// [RW 1] QM - CM Interface enable. If 0 - the valid input is disregarded;
pub const CCM_REG_CQM_CCM_IFEN: c_uint = 0xd0014;
// [RW 6] QM output initial credit. Max credit available - 32. Write writes
pub const CCM_REG_CQM_INIT_CRD: c_uint = 0xd020c;
// [RW 3] The weight of the QM (primary) input in the WRR mechanism. 0
pub const CCM_REG_CQM_P_WEIGHT: c_uint = 0xd00b8;
// [RW 3] The weight of the QM (secondary) input in the WRR mechanism. 0
pub const CCM_REG_CQM_S_WEIGHT: c_uint = 0xd00bc;
// [RW 1] Input SDM Interface enable. If 0 - the valid input is disregarded;
pub const CCM_REG_CSDM_IFEN: c_uint = 0xd0018;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const CCM_REG_CSDM_LENGTH_MIS: c_uint = 0xd0170;
// [RW 3] The weight of the SDM input in the WRR mechanism. 0 stands for
pub const CCM_REG_CSDM_WEIGHT: c_uint = 0xd00b4;
// [RW 28] The CM header for QM formatting in case of an error in the QM
pub const CCM_REG_ERR_CCM_HDR: c_uint = 0xd0094;
// [RW 8] The Event ID in case the input message ErrorFlg is set.
pub const CCM_REG_ERR_EVNT_ID: c_uint = 0xd0098;
// [RW 8] FIC0 output initial credit. Max credit available - 255. Write
pub const CCM_REG_FIC0_INIT_CRD: c_uint = 0xd0210;
// [RW 8] FIC1 output initial credit. Max credit available - 255.Write
pub const CCM_REG_FIC1_INIT_CRD: c_uint = 0xd0214;
// [RW 1] Arbitration between Input Arbiter groups: 0 - fair Round-Robin; 1
pub const CCM_REG_GR_ARB_TYPE: c_uint = 0xd015c;
// [RW 2] Load (FIC0) channel group priority. The lowest priority is 0; the
pub const CCM_REG_GR_LD0_PR: c_uint = 0xd0164;
// [RW 2] Load (FIC1) channel group priority. The lowest priority is 0; the
pub const CCM_REG_GR_LD1_PR: c_uint = 0xd0168;
// [RW 2] General flags index.
pub const CCM_REG_INV_DONE_Q: c_uint = 0xd0108;
// [RW 4] The number of double REG-pairs(128 bits); loaded from the STORM
pub const CCM_REG_N_SM_CTX_LD_0: c_uint = 0xd004c;
pub const CCM_REG_N_SM_CTX_LD_1: c_uint = 0xd0050;
pub const CCM_REG_N_SM_CTX_LD_2: c_uint = 0xd0054;
pub const CCM_REG_N_SM_CTX_LD_3: c_uint = 0xd0058;
pub const CCM_REG_N_SM_CTX_LD_4: c_uint = 0xd005c;
// [RW 1] Input pbf Interface enable. If 0 - the valid input is disregarded;
pub const CCM_REG_PBF_IFEN: c_uint = 0xd0028;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const CCM_REG_PBF_LENGTH_MIS: c_uint = 0xd0180;
// [RW 3] The weight of the input pbf in the WRR mechanism. 0 stands for
pub const CCM_REG_PBF_WEIGHT: c_uint = 0xd00ac;
pub const CCM_REG_PHYS_QNUM1_0: c_uint = 0xd0134;
pub const CCM_REG_PHYS_QNUM1_1: c_uint = 0xd0138;
pub const CCM_REG_PHYS_QNUM2_0: c_uint = 0xd013c;
pub const CCM_REG_PHYS_QNUM2_1: c_uint = 0xd0140;
pub const CCM_REG_PHYS_QNUM3_0: c_uint = 0xd0144;
pub const CCM_REG_PHYS_QNUM3_1: c_uint = 0xd0148;
pub const CCM_REG_QOS_PHYS_QNUM0_0: c_uint = 0xd0114;
pub const CCM_REG_QOS_PHYS_QNUM0_1: c_uint = 0xd0118;
pub const CCM_REG_QOS_PHYS_QNUM1_0: c_uint = 0xd011c;
pub const CCM_REG_QOS_PHYS_QNUM1_1: c_uint = 0xd0120;
pub const CCM_REG_QOS_PHYS_QNUM2_0: c_uint = 0xd0124;
pub const CCM_REG_QOS_PHYS_QNUM2_1: c_uint = 0xd0128;
pub const CCM_REG_QOS_PHYS_QNUM3_0: c_uint = 0xd012c;
pub const CCM_REG_QOS_PHYS_QNUM3_1: c_uint = 0xd0130;
// [RW 1] STORM - CM Interface enable. If 0 - the valid input is
pub const CCM_REG_STORM_CCM_IFEN: c_uint = 0xd0010;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const CCM_REG_STORM_LENGTH_MIS: c_uint = 0xd016c;
// [RW 3] The weight of the STORM input in the WRR (Weighted Round robin)
extern "C" {
    pub fn 1(prioritised: least prioritised); 2 stands for weight 2 (more) -> weight;
}
pub const CCM_REG_STORM_WEIGHT: c_uint = 0xd009c;
// [RW 1] Input tsem Interface enable. If 0 - the valid input is
pub const CCM_REG_TSEM_IFEN: c_uint = 0xd001c;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const CCM_REG_TSEM_LENGTH_MIS: c_uint = 0xd0174;
// [RW 3] The weight of the input tsem in the WRR mechanism. 0 stands for
pub const CCM_REG_TSEM_WEIGHT: c_uint = 0xd00a0;
// [RW 1] Input usem Interface enable. If 0 - the valid input is
pub const CCM_REG_USEM_IFEN: c_uint = 0xd0024;
// [RC 1] Set when message length mismatch (relative to last indication) at
pub const CCM_REG_USEM_LENGTH_MIS: c_uint = 0xd017c;
// [RW 3] The weight of the input usem in the WRR mechanism. 0 stands for
pub const CCM_REG_USEM_WEIGHT: c_uint = 0xd00a8;
// [RW 1] Input xsem Interface enable. If 0 - the valid input is
pub const CCM_REG_XSEM_IFEN: c_uint = 0xd0020;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const CCM_REG_XSEM_LENGTH_MIS: c_uint = 0xd0178;
// [RW 3] The weight of the input xsem in the WRR mechanism. 0 stands for
pub const CCM_REG_XSEM_WEIGHT: c_uint = 0xd00a4;
// [RW 19] Indirect access to the descriptor table of the XX protection
pub const CCM_REG_XX_DESCR_TABLE: c_uint = 0xd0300;
pub const CCM_REG_XX_DESCR_TABLE_SIZE: c_int = 24;
// [R 7] Used to read the value of XX protection Free counter.
pub const CCM_REG_XX_FREE: c_uint = 0xd0184;
// [RW 6] Initial value for the credit counter; responsible for fulfilling
pub const CCM_REG_XX_INIT_CRD: c_uint = 0xd0220;
// [RW 7] The maximum number of pending messages; which may be stored in XX
pub const CCM_REG_XX_MSG_NUM: c_uint = 0xd0224;
// [RW 8] The Event ID; sent to the STORM in case of XX overflow.
pub const CCM_REG_XX_OVFL_EVNT_ID: c_uint = 0xd0044;
// [RW 18] Indirect access to the XX table of the XX protection mechanism.
pub const CCM_REG_XX_TABLE: c_uint = 0xd0280;
pub const CDU_REG_CDU_CHK_MASK0: c_uint = 0x101000;
pub const CDU_REG_CDU_CHK_MASK1: c_uint = 0x101004;
pub const CDU_REG_CDU_CONTROL0: c_uint = 0x101008;
pub const CDU_REG_CDU_DEBUG: c_uint = 0x101010;
pub const CDU_REG_CDU_GLOBAL_PARAMS: c_uint = 0x101020;
// [RW 7] Interrupt mask register #0 read/write
pub const CDU_REG_CDU_INT_MASK: c_uint = 0x10103c;
// [R 7] Interrupt register #0 read
pub const CDU_REG_CDU_INT_STS: c_uint = 0x101030;
// [RW 5] Parity mask register #0 read/write
pub const CDU_REG_CDU_PRTY_MASK: c_uint = 0x10104c;
// [R 5] Parity register #0 read
pub const CDU_REG_CDU_PRTY_STS: c_uint = 0x101040;
// [RC 5] Parity register #0 read clear
pub const CDU_REG_CDU_PRTY_STS_CLR: c_uint = 0x101044;
// [RC 32] logging of error data in case of a CDU load error:
pub const CDU_REG_ERROR_DATA: c_uint = 0x101014;
// [WB 216] L1TT ram access. each entry has the following format :
pub const CDU_REG_L1TT: c_uint = 0x101800;
// [WB 24] MATT ram access. each entry has the following
pub const CDU_REG_MATT: c_uint = 0x101100;
// [RW 1] when this bit is set the CDU operates in e1hmf mode
pub const CDU_REG_MF_MODE: c_uint = 0x101050;
// [R 1] indication the initializing the activity counter by the hardware
pub const CFC_REG_AC_INIT_DONE: c_uint = 0x104078;
// [RW 13] activity counter ram access
pub const CFC_REG_ACTIVITY_COUNTER: c_uint = 0x104400;
pub const CFC_REG_ACTIVITY_COUNTER_SIZE: c_int = 256;
// [R 1] indication the initializing the cams by the hardware was done.
pub const CFC_REG_CAM_INIT_DONE: c_uint = 0x10407c;
// [RW 2] Interrupt mask register #0 read/write
pub const CFC_REG_CFC_INT_MASK: c_uint = 0x104108;
// [R 2] Interrupt register #0 read
pub const CFC_REG_CFC_INT_STS: c_uint = 0x1040fc;
// [RC 2] Interrupt register #0 read clear
pub const CFC_REG_CFC_INT_STS_CLR: c_uint = 0x104100;
// [RW 4] Parity mask register #0 read/write
pub const CFC_REG_CFC_PRTY_MASK: c_uint = 0x104118;
// [R 4] Parity register #0 read
pub const CFC_REG_CFC_PRTY_STS: c_uint = 0x10410c;
// [RC 4] Parity register #0 read clear
pub const CFC_REG_CFC_PRTY_STS_CLR: c_uint = 0x104110;
// [RW 21] CID cam access (21:1 - Data; alid - 0)
pub const CFC_REG_CID_CAM: c_uint = 0x104800;
pub const CFC_REG_CONTROL0: c_uint = 0x104028;
pub const CFC_REG_DEBUG0: c_uint = 0x104050;
// [RW 14] indicates per error (in #cfc_registers_cfc_error_vector.cfc_error
pub const CFC_REG_DISABLE_ON_ERROR: c_uint = 0x104044;
// [RC 14] CFC error vector. when the CFC detects an internal error it will
pub const CFC_REG_ERROR_VECTOR: c_uint = 0x10403c;
// [WB 93] LCID info ram access
pub const CFC_REG_INFO_RAM: c_uint = 0x105000;
pub const CFC_REG_INFO_RAM_SIZE: c_int = 1024;
pub const CFC_REG_INIT_REG: c_uint = 0x10404c;
pub const CFC_REG_INTERFACES: c_uint = 0x104058;
// [RW 24] {weight_load_client7[2:0] to weight_load_client0[2:0]}. this
pub const CFC_REG_LCREQ_WEIGHTS: c_uint = 0x104084;
// [RW 16] Link List ram access; data = {prev_lcid; ext_lcid}
pub const CFC_REG_LINK_LIST: c_uint = 0x104c00;
pub const CFC_REG_LINK_LIST_SIZE: c_int = 256;
// [R 1] indication the initializing the link list by the hardware was done.
pub const CFC_REG_LL_INIT_DONE: c_uint = 0x104074;
// [R 9] Number of allocated LCIDs which are at empty state
pub const CFC_REG_NUM_LCIDS_ALLOC: c_uint = 0x104020;
// [R 9] Number of Arriving LCIDs in Link List Block
pub const CFC_REG_NUM_LCIDS_ARRIVING: c_uint = 0x104004;
pub const CFC_REG_NUM_LCIDS_INSIDE_PF: c_uint = 0x104120;
// [R 9] Number of Leaving LCIDs in Link List Block
pub const CFC_REG_NUM_LCIDS_LEAVING: c_uint = 0x104018;
pub const CFC_REG_WEAK_ENABLE_PF: c_uint = 0x104124;
// [RW 8] The event id for aggregated interrupt 0
pub const CSDM_REG_AGG_INT_EVENT_0: c_uint = 0xc2038;
pub const CSDM_REG_AGG_INT_EVENT_10: c_uint = 0xc2060;
pub const CSDM_REG_AGG_INT_EVENT_11: c_uint = 0xc2064;
pub const CSDM_REG_AGG_INT_EVENT_12: c_uint = 0xc2068;
pub const CSDM_REG_AGG_INT_EVENT_13: c_uint = 0xc206c;
pub const CSDM_REG_AGG_INT_EVENT_14: c_uint = 0xc2070;
pub const CSDM_REG_AGG_INT_EVENT_15: c_uint = 0xc2074;
pub const CSDM_REG_AGG_INT_EVENT_16: c_uint = 0xc2078;
pub const CSDM_REG_AGG_INT_EVENT_2: c_uint = 0xc2040;
pub const CSDM_REG_AGG_INT_EVENT_3: c_uint = 0xc2044;
pub const CSDM_REG_AGG_INT_EVENT_4: c_uint = 0xc2048;
pub const CSDM_REG_AGG_INT_EVENT_5: c_uint = 0xc204c;
pub const CSDM_REG_AGG_INT_EVENT_6: c_uint = 0xc2050;
pub const CSDM_REG_AGG_INT_EVENT_7: c_uint = 0xc2054;
pub const CSDM_REG_AGG_INT_EVENT_8: c_uint = 0xc2058;
pub const CSDM_REG_AGG_INT_EVENT_9: c_uint = 0xc205c;
// [RW 1] For each aggregated interrupt index whether the mode is normal (0)
pub const CSDM_REG_AGG_INT_MODE_10: c_uint = 0xc21e0;
pub const CSDM_REG_AGG_INT_MODE_11: c_uint = 0xc21e4;
pub const CSDM_REG_AGG_INT_MODE_12: c_uint = 0xc21e8;
pub const CSDM_REG_AGG_INT_MODE_13: c_uint = 0xc21ec;
pub const CSDM_REG_AGG_INT_MODE_14: c_uint = 0xc21f0;
pub const CSDM_REG_AGG_INT_MODE_15: c_uint = 0xc21f4;
pub const CSDM_REG_AGG_INT_MODE_16: c_uint = 0xc21f8;
pub const CSDM_REG_AGG_INT_MODE_6: c_uint = 0xc21d0;
pub const CSDM_REG_AGG_INT_MODE_7: c_uint = 0xc21d4;
pub const CSDM_REG_AGG_INT_MODE_8: c_uint = 0xc21d8;
pub const CSDM_REG_AGG_INT_MODE_9: c_uint = 0xc21dc;
// [RW 13] The start address in the internal RAM for the cfc_rsp lcid
pub const CSDM_REG_CFC_RSP_START_ADDR: c_uint = 0xc2008;
// [RW 16] The maximum value of the completion counter #0
pub const CSDM_REG_CMP_COUNTER_MAX0: c_uint = 0xc201c;
// [RW 16] The maximum value of the completion counter #1
pub const CSDM_REG_CMP_COUNTER_MAX1: c_uint = 0xc2020;
// [RW 16] The maximum value of the completion counter #2
pub const CSDM_REG_CMP_COUNTER_MAX2: c_uint = 0xc2024;
// [RW 16] The maximum value of the completion counter #3
pub const CSDM_REG_CMP_COUNTER_MAX3: c_uint = 0xc2028;
// [RW 13] The start address in the internal RAM for the completion
pub const CSDM_REG_CMP_COUNTER_START_ADDR: c_uint = 0xc200c;
// [RW 32] Interrupt mask register #0 read/write
pub const CSDM_REG_CSDM_INT_MASK_0: c_uint = 0xc229c;
pub const CSDM_REG_CSDM_INT_MASK_1: c_uint = 0xc22ac;
// [R 32] Interrupt register #0 read
pub const CSDM_REG_CSDM_INT_STS_0: c_uint = 0xc2290;
pub const CSDM_REG_CSDM_INT_STS_1: c_uint = 0xc22a0;
// [RW 11] Parity mask register #0 read/write
pub const CSDM_REG_CSDM_PRTY_MASK: c_uint = 0xc22bc;
// [R 11] Parity register #0 read
pub const CSDM_REG_CSDM_PRTY_STS: c_uint = 0xc22b0;
// [RC 11] Parity register #0 read clear
pub const CSDM_REG_CSDM_PRTY_STS_CLR: c_uint = 0xc22b4;
pub const CSDM_REG_ENABLE_IN1: c_uint = 0xc2238;
pub const CSDM_REG_ENABLE_IN2: c_uint = 0xc223c;
pub const CSDM_REG_ENABLE_OUT1: c_uint = 0xc2240;
pub const CSDM_REG_ENABLE_OUT2: c_uint = 0xc2244;
// [RW 4] The initial number of messages that can be sent to the pxp control
pub const CSDM_REG_INIT_CREDIT_PXP_CTRL: c_uint = 0xc24bc;
// [ST 32] The number of ACK after placement messages received
pub const CSDM_REG_NUM_OF_ACK_AFTER_PLACE: c_uint = 0xc227c;
// [ST 32] The number of packet end messages received from the parser
pub const CSDM_REG_NUM_OF_PKT_END_MSG: c_uint = 0xc2274;
// [ST 32] The number of requests received from the pxp async if
pub const CSDM_REG_NUM_OF_PXP_ASYNC_REQ: c_uint = 0xc2278;
// [ST 32] The number of commands received in queue 0
pub const CSDM_REG_NUM_OF_Q0_CMD: c_uint = 0xc2248;
// [ST 32] The number of commands received in queue 10
pub const CSDM_REG_NUM_OF_Q10_CMD: c_uint = 0xc226c;
// [ST 32] The number of commands received in queue 11
pub const CSDM_REG_NUM_OF_Q11_CMD: c_uint = 0xc2270;
// [ST 32] The number of commands received in queue 1
pub const CSDM_REG_NUM_OF_Q1_CMD: c_uint = 0xc224c;
// [ST 32] The number of commands received in queue 3
pub const CSDM_REG_NUM_OF_Q3_CMD: c_uint = 0xc2250;
// [ST 32] The number of commands received in queue 4
pub const CSDM_REG_NUM_OF_Q4_CMD: c_uint = 0xc2254;
// [ST 32] The number of commands received in queue 5
pub const CSDM_REG_NUM_OF_Q5_CMD: c_uint = 0xc2258;
// [ST 32] The number of commands received in queue 6
pub const CSDM_REG_NUM_OF_Q6_CMD: c_uint = 0xc225c;
// [ST 32] The number of commands received in queue 7
pub const CSDM_REG_NUM_OF_Q7_CMD: c_uint = 0xc2260;
// [ST 32] The number of commands received in queue 8
pub const CSDM_REG_NUM_OF_Q8_CMD: c_uint = 0xc2264;
// [ST 32] The number of commands received in queue 9
pub const CSDM_REG_NUM_OF_Q9_CMD: c_uint = 0xc2268;
// [RW 13] The start address in the internal RAM for queue counters
pub const CSDM_REG_Q_COUNTER_START_ADDR: c_uint = 0xc2010;
// [R 1] pxp_ctrl rd_data fifo empty in sdm_dma_rsp block
pub const CSDM_REG_RSP_PXP_CTRL_RDATA_EMPTY: c_uint = 0xc2548;
// [R 1] parser fifo empty in sdm_sync block
pub const CSDM_REG_SYNC_PARSER_EMPTY: c_uint = 0xc2550;
// [R 1] parser serial fifo empty in sdm_sync block
pub const CSDM_REG_SYNC_SYNC_EMPTY: c_uint = 0xc2558;
// [RW 32] Tick for timer counter. Applicable only when
pub const CSDM_REG_TIMER_TICK: c_uint = 0xc2000;
// [RW 5] The number of time_slots in the arbitration cycle
pub const CSEM_REG_ARB_CYCLE_SIZE: c_uint = 0x200034;
// [RW 3] The source that is associated with arbitration element 0. Source
pub const CSEM_REG_ARB_ELEMENT0: c_uint = 0x200020;
// [RW 3] The source that is associated with arbitration element 1. Source
pub const CSEM_REG_ARB_ELEMENT1: c_uint = 0x200024;
// [RW 3] The source that is associated with arbitration element 2. Source
pub const CSEM_REG_ARB_ELEMENT2: c_uint = 0x200028;
// [RW 3] The source that is associated with arbitration element 3. Source
pub const CSEM_REG_ARB_ELEMENT3: c_uint = 0x20002c;
// [RW 3] The source that is associated with arbitration element 4. Source
pub const CSEM_REG_ARB_ELEMENT4: c_uint = 0x200030;
// [RW 32] Interrupt mask register #0 read/write
pub const CSEM_REG_CSEM_INT_MASK_0: c_uint = 0x200110;
pub const CSEM_REG_CSEM_INT_MASK_1: c_uint = 0x200120;
// [R 32] Interrupt register #0 read
pub const CSEM_REG_CSEM_INT_STS_0: c_uint = 0x200104;
pub const CSEM_REG_CSEM_INT_STS_1: c_uint = 0x200114;
// [RW 32] Parity mask register #0 read/write
pub const CSEM_REG_CSEM_PRTY_MASK_0: c_uint = 0x200130;
pub const CSEM_REG_CSEM_PRTY_MASK_1: c_uint = 0x200140;
// [R 32] Parity register #0 read
pub const CSEM_REG_CSEM_PRTY_STS_0: c_uint = 0x200124;
pub const CSEM_REG_CSEM_PRTY_STS_1: c_uint = 0x200134;
// [RC 32] Parity register #0 read clear
pub const CSEM_REG_CSEM_PRTY_STS_CLR_0: c_uint = 0x200128;
pub const CSEM_REG_CSEM_PRTY_STS_CLR_1: c_uint = 0x200138;
pub const CSEM_REG_ENABLE_IN: c_uint = 0x2000a4;
pub const CSEM_REG_ENABLE_OUT: c_uint = 0x2000a8;
// [RW 32] This address space contains all registers and memories that are
pub const CSEM_REG_FAST_MEMORY: c_uint = 0x220000;
// [RW 1] Disables input messages from FIC0 May be updated during run_time
pub const CSEM_REG_FIC0_DISABLE: c_uint = 0x200224;
// [RW 1] Disables input messages from FIC1 May be updated during run_time
pub const CSEM_REG_FIC1_DISABLE: c_uint = 0x200234;
// [RW 15] Interrupt table Read and write access to it is not possible in
pub const CSEM_REG_INT_TABLE: c_uint = 0x200400;
// [ST 24] Statistics register. The number of messages that entered through
pub const CSEM_REG_MSG_NUM_FIC0: c_uint = 0x200000;
// [ST 24] Statistics register. The number of messages that entered through
pub const CSEM_REG_MSG_NUM_FIC1: c_uint = 0x200004;
// [ST 24] Statistics register. The number of messages that were sent to
pub const CSEM_REG_MSG_NUM_FOC0: c_uint = 0x200008;
// [ST 24] Statistics register. The number of messages that were sent to
pub const CSEM_REG_MSG_NUM_FOC1: c_uint = 0x20000c;
// [ST 24] Statistics register. The number of messages that were sent to
pub const CSEM_REG_MSG_NUM_FOC2: c_uint = 0x200010;
// [ST 24] Statistics register. The number of messages that were sent to
pub const CSEM_REG_MSG_NUM_FOC3: c_uint = 0x200014;
// [RW 1] Disables input messages from the passive buffer May be updated
pub const CSEM_REG_PAS_DISABLE: c_uint = 0x20024c;
// [WB 128] Debug only. Passive buffer memory
pub const CSEM_REG_PASSIVE_BUFFER: c_uint = 0x202000;
// [WB 46] pram memory. B45 is parity; b[44:0] - data.
pub const CSEM_REG_PRAM: c_uint = 0x240000;
// [R 16] Valid sleeping threads indication have bit per thread
pub const CSEM_REG_SLEEP_THREADS_VALID: c_uint = 0x20026c;
// [R 1] EXT_STORE FIFO is empty in sem_slow_ls_ext
pub const CSEM_REG_SLOW_EXT_STORE_EMPTY: c_uint = 0x2002a0;
// [RW 16] List of free threads . There is a bit per thread.
pub const CSEM_REG_THREADS_LIST: c_uint = 0x2002e4;
// [RW 3] The arbitration scheme of time_slot 0
pub const CSEM_REG_TS_0_AS: c_uint = 0x200038;
// [RW 3] The arbitration scheme of time_slot 10
pub const CSEM_REG_TS_10_AS: c_uint = 0x200060;
// [RW 3] The arbitration scheme of time_slot 11
pub const CSEM_REG_TS_11_AS: c_uint = 0x200064;
// [RW 3] The arbitration scheme of time_slot 12
pub const CSEM_REG_TS_12_AS: c_uint = 0x200068;
// [RW 3] The arbitration scheme of time_slot 13
pub const CSEM_REG_TS_13_AS: c_uint = 0x20006c;
// [RW 3] The arbitration scheme of time_slot 14
pub const CSEM_REG_TS_14_AS: c_uint = 0x200070;
// [RW 3] The arbitration scheme of time_slot 15
pub const CSEM_REG_TS_15_AS: c_uint = 0x200074;
// [RW 3] The arbitration scheme of time_slot 16
pub const CSEM_REG_TS_16_AS: c_uint = 0x200078;
// [RW 3] The arbitration scheme of time_slot 17
pub const CSEM_REG_TS_17_AS: c_uint = 0x20007c;
// [RW 3] The arbitration scheme of time_slot 18
pub const CSEM_REG_TS_18_AS: c_uint = 0x200080;
// [RW 3] The arbitration scheme of time_slot 1
pub const CSEM_REG_TS_1_AS: c_uint = 0x20003c;
// [RW 3] The arbitration scheme of time_slot 2
pub const CSEM_REG_TS_2_AS: c_uint = 0x200040;
// [RW 3] The arbitration scheme of time_slot 3
pub const CSEM_REG_TS_3_AS: c_uint = 0x200044;
// [RW 3] The arbitration scheme of time_slot 4
pub const CSEM_REG_TS_4_AS: c_uint = 0x200048;
// [RW 3] The arbitration scheme of time_slot 5
pub const CSEM_REG_TS_5_AS: c_uint = 0x20004c;
// [RW 3] The arbitration scheme of time_slot 6
pub const CSEM_REG_TS_6_AS: c_uint = 0x200050;
// [RW 3] The arbitration scheme of time_slot 7
pub const CSEM_REG_TS_7_AS: c_uint = 0x200054;
// [RW 3] The arbitration scheme of time_slot 8
pub const CSEM_REG_TS_8_AS: c_uint = 0x200058;
// [RW 3] The arbitration scheme of time_slot 9
pub const CSEM_REG_TS_9_AS: c_uint = 0x20005c;
// [W 7] VF or PF ID for reset error bit. Values 0-63 reset error bit for 64
// VF; values 64-67 reset error for 4 PF; values 68-127 are not valid.
pub const CSEM_REG_VFPF_ERR_NUM: c_uint = 0x200380;
// [RW 1] Parity mask register #0 read/write
pub const DBG_REG_DBG_PRTY_MASK: c_uint = 0xc0a8;
// [R 1] Parity register #0 read
pub const DBG_REG_DBG_PRTY_STS: c_uint = 0xc09c;
// [RC 1] Parity register #0 read clear
pub const DBG_REG_DBG_PRTY_STS_CLR: c_uint = 0xc0a0;
// [RW 1] When set the DMAE will process the commands as in E1.5. 1.The
// function that is used is always SRC-PCI; 2.VF_Valid = 0; 3.VFID=0;
// 4.Completion function=0; 5.Error handling=0
pub const DMAE_REG_BACKWARD_COMP_EN: c_uint = 0x10207c;
// [RW 32] Commands memory. The address to command X; row Y is to calculated
pub const DMAE_REG_CMD_MEM: c_uint = 0x102400;
pub const DMAE_REG_CMD_MEM_SIZE: c_int = 224;
// [RW 1] If 0 - the CRC-16c initial value is all zeroes; if 1 - the CRC-16c
pub const DMAE_REG_CRC16C_INIT: c_uint = 0x10201c;
// [RW 1] If 0 - the CRC-16 T10 initial value is all zeroes; if 1 - the
pub const DMAE_REG_CRC16T10_INIT: c_uint = 0x102020;
// [RW 2] Interrupt mask register #0 read/write
pub const DMAE_REG_DMAE_INT_MASK: c_uint = 0x102054;
// [RW 4] Parity mask register #0 read/write
pub const DMAE_REG_DMAE_PRTY_MASK: c_uint = 0x102064;
// [R 4] Parity register #0 read
pub const DMAE_REG_DMAE_PRTY_STS: c_uint = 0x102058;
// [RC 4] Parity register #0 read clear
pub const DMAE_REG_DMAE_PRTY_STS_CLR: c_uint = 0x10205c;
// [RW 1] Command 0 go.
pub const DMAE_REG_GO_C0: c_uint = 0x102080;
// [RW 1] Command 1 go.
pub const DMAE_REG_GO_C1: c_uint = 0x102084;
// [RW 1] Command 10 go.
pub const DMAE_REG_GO_C10: c_uint = 0x102088;
// [RW 1] Command 11 go.
pub const DMAE_REG_GO_C11: c_uint = 0x10208c;
// [RW 1] Command 12 go.
pub const DMAE_REG_GO_C12: c_uint = 0x102090;
// [RW 1] Command 13 go.
pub const DMAE_REG_GO_C13: c_uint = 0x102094;
// [RW 1] Command 14 go.
pub const DMAE_REG_GO_C14: c_uint = 0x102098;
// [RW 1] Command 15 go.
pub const DMAE_REG_GO_C15: c_uint = 0x10209c;
// [RW 1] Command 2 go.
pub const DMAE_REG_GO_C2: c_uint = 0x1020a0;
// [RW 1] Command 3 go.
pub const DMAE_REG_GO_C3: c_uint = 0x1020a4;
// [RW 1] Command 4 go.
pub const DMAE_REG_GO_C4: c_uint = 0x1020a8;
// [RW 1] Command 5 go.
pub const DMAE_REG_GO_C5: c_uint = 0x1020ac;
// [RW 1] Command 6 go.
pub const DMAE_REG_GO_C6: c_uint = 0x1020b0;
// [RW 1] Command 7 go.
pub const DMAE_REG_GO_C7: c_uint = 0x1020b4;
// [RW 1] Command 8 go.
pub const DMAE_REG_GO_C8: c_uint = 0x1020b8;
// [RW 1] Command 9 go.
pub const DMAE_REG_GO_C9: c_uint = 0x1020bc;
// [RW 1] DMAE GRC Interface (Target; aster) enable. If 0 - the acknowledge
pub const DMAE_REG_GRC_IFEN: c_uint = 0x102008;
// [RW 1] DMAE PCI Interface (Request; ead; rite) enable. If 0 - the
pub const DMAE_REG_PCI_IFEN: c_uint = 0x102004;
// [RW 4] DMAE- PCI Request Interface initial credit. Write writes the
pub const DMAE_REG_PXP_REQ_INIT_CRD: c_uint = 0x1020c0;
// [RW 8] Aggregation command.
pub const DORQ_REG_AGG_CMD0: c_uint = 0x170060;
// [RW 8] Aggregation command.
pub const DORQ_REG_AGG_CMD1: c_uint = 0x170064;
// [RW 8] Aggregation command.
pub const DORQ_REG_AGG_CMD2: c_uint = 0x170068;
// [RW 8] Aggregation command.
pub const DORQ_REG_AGG_CMD3: c_uint = 0x17006c;
// [RW 28] UCM Header.
pub const DORQ_REG_CMHEAD_RX: c_uint = 0x170050;
// [RW 32] Doorbell address for RBC doorbells (function 0).
pub const DORQ_REG_DB_ADDR0: c_uint = 0x17008c;
// [RW 5] Interrupt mask register #0 read/write
pub const DORQ_REG_DORQ_INT_MASK: c_uint = 0x170180;
// [R 5] Interrupt register #0 read
pub const DORQ_REG_DORQ_INT_STS: c_uint = 0x170174;
// [RC 5] Interrupt register #0 read clear
pub const DORQ_REG_DORQ_INT_STS_CLR: c_uint = 0x170178;
// [RW 2] Parity mask register #0 read/write
pub const DORQ_REG_DORQ_PRTY_MASK: c_uint = 0x170190;
// [R 2] Parity register #0 read
pub const DORQ_REG_DORQ_PRTY_STS: c_uint = 0x170184;
// [RC 2] Parity register #0 read clear
pub const DORQ_REG_DORQ_PRTY_STS_CLR: c_uint = 0x170188;
// [RW 8] The address to write the DPM CID to STORM.
pub const DORQ_REG_DPM_CID_ADDR: c_uint = 0x170044;
// [RW 5] The DPM mode CID extraction offset.
pub const DORQ_REG_DPM_CID_OFST: c_uint = 0x170030;
// [RW 12] The threshold of the DQ FIFO to send the almost full interrupt.
pub const DORQ_REG_DQ_FIFO_AFULL_TH: c_uint = 0x17007c;
// [RW 12] The threshold of the DQ FIFO to send the full interrupt.
pub const DORQ_REG_DQ_FIFO_FULL_TH: c_uint = 0x170078;
// [R 13] Current value of the DQ FIFO fill level according to following
pub const DORQ_REG_DQ_FILL_LVLF: c_uint = 0x1700a4;
// [R 1] DQ FIFO full status. Is set; when FIFO filling level is more or
pub const DORQ_REG_DQ_FULL_ST: c_uint = 0x1700c0;
// [RW 28] The value sent to CM header in the case of CFC load error.
pub const DORQ_REG_ERR_CMHEAD: c_uint = 0x170058;
pub const DORQ_REG_IF_EN: c_uint = 0x170004;
pub const DORQ_REG_MAX_RVFID_SIZE: c_uint = 0x1701ec;
pub const DORQ_REG_MODE_ACT: c_uint = 0x170008;
// [RW 5] The normal mode CID extraction offset.
pub const DORQ_REG_NORM_CID_OFST: c_uint = 0x17002c;
// [RW 28] TCM Header when only TCP context is loaded.
pub const DORQ_REG_NORM_CMHEAD_TX: c_uint = 0x17004c;
// [RW 3] The number of simultaneous outstanding requests to Context Fetch
pub const DORQ_REG_OUTST_REQ: c_uint = 0x17003c;
pub const DORQ_REG_PF_USAGE_CNT: c_uint = 0x1701d0;
pub const DORQ_REG_REGN: c_uint = 0x170038;
// [R 4] Current value of response A counter credit. Initial credit is
pub const DORQ_REG_RSPA_CRD_CNT: c_uint = 0x1700ac;
// [R 4] Current value of response B counter credit. Initial credit is
pub const DORQ_REG_RSPB_CRD_CNT: c_uint = 0x1700b0;
// [RW 4] The initial credit at the Doorbell Response Interface. The write
pub const DORQ_REG_RSP_INIT_CRD: c_uint = 0x170048;
pub const DORQ_REG_RSPB_CRD_CNT: c_uint = 0x1700b0;
pub const DORQ_REG_VF_NORM_CID_BASE: c_uint = 0x1701a0;
pub const DORQ_REG_VF_NORM_CID_OFST: c_uint = 0x1701f4;
pub const DORQ_REG_VF_NORM_CID_WND_SIZE: c_uint = 0x1701a4;
pub const DORQ_REG_VF_NORM_MAX_CID_COUNT: c_uint = 0x1701e4;
pub const DORQ_REG_VF_NORM_VF_BASE: c_uint = 0x1701a8;
// [RW 10] VF type validation mask value
pub const DORQ_REG_VF_TYPE_MASK_0: c_uint = 0x170218;
// [RW 17] VF type validation Min MCID value
pub const DORQ_REG_VF_TYPE_MAX_MCID_0: c_uint = 0x1702d8;
// [RW 17] VF type validation Max MCID value
pub const DORQ_REG_VF_TYPE_MIN_MCID_0: c_uint = 0x170298;
// [RW 10] VF type validation comp value
pub const DORQ_REG_VF_TYPE_VALUE_0: c_uint = 0x170258;
pub const DORQ_REG_VF_USAGE_CT_LIMIT: c_uint = 0x170340;
// [RW 4] Initial activity counter value on the load request; when the
pub const DORQ_REG_SHRT_ACT_CNT: c_uint = 0x170070;
// [RW 28] TCM Header when both ULP and TCP context is loaded.
pub const DORQ_REG_SHRT_CMHEAD: c_uint = 0x170054;

pub const DORQ_REG_VF_USAGE_CNT: c_uint = 0x170320;
pub const HC_REG_AGG_INT_0: c_uint = 0x108050;
pub const HC_REG_AGG_INT_1: c_uint = 0x108054;
pub const HC_REG_ATTN_BIT: c_uint = 0x108120;
pub const HC_REG_ATTN_IDX: c_uint = 0x108100;
pub const HC_REG_ATTN_MSG0_ADDR_L: c_uint = 0x108018;
pub const HC_REG_ATTN_MSG1_ADDR_L: c_uint = 0x108020;
pub const HC_REG_ATTN_NUM_P0: c_uint = 0x108038;
pub const HC_REG_ATTN_NUM_P1: c_uint = 0x10803c;
pub const HC_REG_COMMAND_REG: c_uint = 0x108180;
pub const HC_REG_CONFIG_0: c_uint = 0x108000;
pub const HC_REG_CONFIG_1: c_uint = 0x108004;
pub const HC_REG_FUNC_NUM_P0: c_uint = 0x1080ac;
pub const HC_REG_FUNC_NUM_P1: c_uint = 0x1080b0;
// [RW 3] Parity mask register #0 read/write
pub const HC_REG_HC_PRTY_MASK: c_uint = 0x1080a0;
// [R 3] Parity register #0 read
pub const HC_REG_HC_PRTY_STS: c_uint = 0x108094;
// [RC 3] Parity register #0 read clear
pub const HC_REG_HC_PRTY_STS_CLR: c_uint = 0x108098;
pub const HC_REG_INT_MASK: c_uint = 0x108108;
pub const HC_REG_LEADING_EDGE_0: c_uint = 0x108040;
pub const HC_REG_LEADING_EDGE_1: c_uint = 0x108048;
pub const HC_REG_MAIN_MEMORY: c_uint = 0x108800;
pub const HC_REG_MAIN_MEMORY_SIZE: c_int = 152;
pub const HC_REG_P0_PROD_CONS: c_uint = 0x108200;
pub const HC_REG_P1_PROD_CONS: c_uint = 0x108400;
pub const HC_REG_PBA_COMMAND: c_uint = 0x108140;
pub const HC_REG_PCI_CONFIG_0: c_uint = 0x108010;
pub const HC_REG_PCI_CONFIG_1: c_uint = 0x108014;
pub const HC_REG_STATISTIC_COUNTERS: c_uint = 0x109000;
pub const HC_REG_TRAILING_EDGE_0: c_uint = 0x108044;
pub const HC_REG_TRAILING_EDGE_1: c_uint = 0x10804c;
pub const HC_REG_UC_RAM_ADDR_0: c_uint = 0x108028;
pub const HC_REG_UC_RAM_ADDR_1: c_uint = 0x108030;
pub const HC_REG_USTORM_ADDR_FOR_COALESCE: c_uint = 0x108068;
pub const HC_REG_VQID_0: c_uint = 0x108008;
pub const HC_REG_VQID_1: c_uint = 0x10800c;

pub const IGU_REG_ATTENTION_ACK_BITS: c_uint = 0x130108;
// [R 4] Debug: attn_fsm
pub const IGU_REG_ATTN_FSM: c_uint = 0x130054;
pub const IGU_REG_ATTN_MSG_ADDR_H: c_uint = 0x13011c;
pub const IGU_REG_ATTN_MSG_ADDR_L: c_uint = 0x130120;
// [R 4] Debug: [3] - attention write done message is pending (0-no pending;
// 1-pending). [2:0] = PFID. Pending means attention message was sent; but
// write done didn't receive.
pub const IGU_REG_ATTN_WRITE_DONE_PENDING: c_uint = 0x130030;
pub const IGU_REG_BLOCK_CONFIGURATION: c_uint = 0x130000;
pub const IGU_REG_COMMAND_REG_32LSB_DATA: c_uint = 0x130124;
pub const IGU_REG_COMMAND_REG_CTRL: c_uint = 0x13012c;
// [WB_R 32] Cleanup bit status per SB. 1 = cleanup is set. 0 = cleanup bit
// is clear. The bits in this registers are set and clear via the producer
// command. Data valid only in addresses 0-4. all the rest are zero.
pub const IGU_REG_CSTORM_TYPE_0_SB_CLEANUP: c_uint = 0x130200;
// [R 5] Debug: ctrl_fsm
pub const IGU_REG_CTRL_FSM: c_uint = 0x130064;
// [R 1] data available for error memory. If this bit is clear do not red
// from error_handling_memory.
pub const IGU_REG_ERROR_HANDLING_DATA_VALID: c_uint = 0x130130;
// [RW 11] Parity mask register #0 read/write
pub const IGU_REG_IGU_PRTY_MASK: c_uint = 0x1300a8;
// [R 11] Parity register #0 read
pub const IGU_REG_IGU_PRTY_STS: c_uint = 0x13009c;
// [RC 11] Parity register #0 read clear
pub const IGU_REG_IGU_PRTY_STS_CLR: c_uint = 0x1300a0;
// [R 4] Debug: int_handle_fsm
pub const IGU_REG_INT_HANDLE_FSM: c_uint = 0x130050;
pub const IGU_REG_LEADING_EDGE_LATCH: c_uint = 0x130134;
// [RW 14] mapping CAM; relevant for E2 operating mode only. [0] - valid.
// [6:1] - vector number; [13:7] - FID (if VF - [13] = 0; [12:7] = VF
// number; if PF - [13] = 1; [12:10] = 0; [9:7] = PF number);
pub const IGU_REG_MAPPING_MEMORY: c_uint = 0x131000;
pub const IGU_REG_MAPPING_MEMORY_SIZE: c_int = 136;
pub const IGU_REG_PBA_STATUS_LSB: c_uint = 0x130138;
pub const IGU_REG_PBA_STATUS_MSB: c_uint = 0x13013c;
pub const IGU_REG_PCI_PF_MSI_EN: c_uint = 0x130140;
pub const IGU_REG_PCI_PF_MSIX_EN: c_uint = 0x130144;
pub const IGU_REG_PCI_PF_MSIX_FUNC_MASK: c_uint = 0x130148;
// [WB_R 32] Each bit represent the pending bits status for that SB. 0 = no
// pending; 1 = pending. Pendings means interrupt was asserted; and write
// done was not received. Data valid only in addresses 0-4. all the rest are
// zero.
pub const IGU_REG_PENDING_BITS_STATUS: c_uint = 0x130300;
pub const IGU_REG_PF_CONFIGURATION: c_uint = 0x130154;
// [RW 20] producers only. E2 mode: address 0-135 match to the mapping
// memory; 136 - PF0 default prod; 137 PF1 default prod; 138 - PF2 default
// prod; 139 PF3 default prod; 140 - PF0 - ATTN prod; 141 - PF1 - ATTN prod;
// 142 - PF2 - ATTN prod; 143 - PF3 - ATTN prod; 144-147 reserved. E1.5 mode
// - In backward compatible mode; for non default SB; each even line in the
// memory holds the U producer and each odd line hold the C producer. The
// first 128 producer are for NDSB (PF0 - 0-31; PF1 - 32-63 and so on). The
// last 20 producers are for the DSB for each PF. each PF has five segments
// (the order inside each segment is PF0; PF1; PF2; PF3) - 128-131 U prods;
// 132-135 C prods; 136-139 X prods; 140-143 T prods; 144-147 ATTN prods;
pub const IGU_REG_PROD_CONS_MEMORY: c_uint = 0x132000;
// [R 3] Debug: pxp_arb_fsm
pub const IGU_REG_PXP_ARB_FSM: c_uint = 0x130068;
// [RW 6] Write one for each bit will reset the appropriate memory. When the
// memory reset finished the appropriate bit will be clear. Bit 0 - mapping
// memory; Bit 1 - SB memory; Bit 2 - SB interrupt and mask register; Bit 3
// - MSIX memory; Bit 4 - PBA memory; Bit 5 - statistics;
pub const IGU_REG_RESET_MEMORIES: c_uint = 0x130158;
// [R 4] Debug: sb_ctrl_fsm
pub const IGU_REG_SB_CTRL_FSM: c_uint = 0x13004c;
pub const IGU_REG_SB_INT_BEFORE_MASK_LSB: c_uint = 0x13015c;
pub const IGU_REG_SB_INT_BEFORE_MASK_MSB: c_uint = 0x130160;
pub const IGU_REG_SB_MASK_LSB: c_uint = 0x130164;
pub const IGU_REG_SB_MASK_MSB: c_uint = 0x130168;
// [RW 16] Number of command that were dropped without causing an interrupt
// due to: read access for WO BAR address; or write access for RO BAR
// address or any access for reserved address or PCI function error is set
// and address is not MSIX; PBA or cleanup
pub const IGU_REG_SILENT_DROP: c_uint = 0x13016c;
// [RW 10] Number of MSI/MSIX/ATTN messages sent for the function: 0-63 -
// number of MSIX messages per VF; 64-67 - number of MSI/MSIX messages per
// PF; 68-71 number of ATTN messages per PF
pub const IGU_REG_STATISTIC_NUM_MESSAGE_SENT: c_uint = 0x130800;
// [RW 32] Number of cycles the timer mask masking the IGU interrupt when a
// timer mask command arrives. Value must be bigger than 100.
pub const IGU_REG_TIMER_MASKING_VALUE: c_uint = 0x13003c;
pub const IGU_REG_TRAILING_EDGE_LATCH: c_uint = 0x130104;
pub const IGU_REG_VF_CONFIGURATION: c_uint = 0x130170;
// [WB_R 32] Each bit represent write done pending bits status for that SB
// (MSI/MSIX message was sent and write done was not received yet). 0 =
// clear; 1 = set. Data valid only in addresses 0-4. all the rest are zero.
pub const IGU_REG_WRITE_DONE_PENDING: c_uint = 0x130480;
pub const MCP_A_REG_MCPR_SCRATCH: c_uint = 0x3a0000;
pub const MCP_REG_MCPR_ACCESS_LOCK: c_uint = 0x8009c;
pub const MCP_REG_MCPR_CPU_PROGRAM_COUNTER: c_uint = 0x8501c;
pub const MCP_REG_MCPR_GP_INPUTS: c_uint = 0x800c0;
pub const MCP_REG_MCPR_GP_OENABLE: c_uint = 0x800c8;
pub const MCP_REG_MCPR_GP_OUTPUTS: c_uint = 0x800c4;
pub const MCP_REG_MCPR_IMC_COMMAND: c_uint = 0x85900;
pub const MCP_REG_MCPR_IMC_DATAREG0: c_uint = 0x85920;
pub const MCP_REG_MCPR_IMC_SLAVE_CONTROL: c_uint = 0x85904;
pub const MCP_REG_MCPR_CPU_PROGRAM_COUNTER: c_uint = 0x8501c;
pub const MCP_REG_MCPR_NVM_ACCESS_ENABLE: c_uint = 0x86424;
pub const MCP_REG_MCPR_NVM_ADDR: c_uint = 0x8640c;
pub const MCP_REG_MCPR_NVM_CFG4: c_uint = 0x8642c;
pub const MCP_REG_MCPR_NVM_COMMAND: c_uint = 0x86400;
pub const MCP_REG_MCPR_NVM_READ: c_uint = 0x86410;
pub const MCP_REG_MCPR_NVM_SW_ARB: c_uint = 0x86420;
pub const MCP_REG_MCPR_NVM_WRITE: c_uint = 0x86408;
pub const MCP_REG_MCPR_SCRATCH: c_uint = 0xa0000;

// [R 32] read first 32 bit after inversion of function 0. mapped as
pub const MISC_REG_AEU_AFTER_INVERT_1_FUNC_0: c_uint = 0xa42c;
pub const MISC_REG_AEU_AFTER_INVERT_1_FUNC_1: c_uint = 0xa430;
// [R 32] read first 32 bit after inversion of mcp. mapped as follows: [0]
pub const MISC_REG_AEU_AFTER_INVERT_1_MCP: c_uint = 0xa434;
// [R 32] read second 32 bit after inversion of function 0. mapped as
pub const MISC_REG_AEU_AFTER_INVERT_2_FUNC_0: c_uint = 0xa438;
pub const MISC_REG_AEU_AFTER_INVERT_2_FUNC_1: c_uint = 0xa43c;
// [R 32] read second 32 bit after inversion of mcp. mapped as follows: [0]
pub const MISC_REG_AEU_AFTER_INVERT_2_MCP: c_uint = 0xa440;
// [R 32] read third 32 bit after inversion of function 0. mapped as
pub const MISC_REG_AEU_AFTER_INVERT_3_FUNC_0: c_uint = 0xa444;
pub const MISC_REG_AEU_AFTER_INVERT_3_FUNC_1: c_uint = 0xa448;
// [R 32] read third 32 bit after inversion of mcp. mapped as follows: [0]
pub const MISC_REG_AEU_AFTER_INVERT_3_MCP: c_uint = 0xa44c;
// [R 32] read fourth 32 bit after inversion of function 0. mapped as
pub const MISC_REG_AEU_AFTER_INVERT_4_FUNC_0: c_uint = 0xa450;
pub const MISC_REG_AEU_AFTER_INVERT_4_FUNC_1: c_uint = 0xa454;
// [R 32] read fourth 32 bit after inversion of mcp. mapped as follows: [0]
pub const MISC_REG_AEU_AFTER_INVERT_4_MCP: c_uint = 0xa458;
// [R 32] Read fifth 32 bit after inversion of function 0. Mapped as
// follows: [0] PGLUE config_space; [1] PGLUE misc_flr; [2] PGLUE B RBC
// attention [3] PGLUE B RBC parity; [4] ATC attention; [5] ATC parity; [6]
// CNIG attention (reserved); [7] CNIG parity (reserved); [31-8] Reserved;
pub const MISC_REG_AEU_AFTER_INVERT_5_FUNC_0: c_uint = 0xa700;
// [W 14] write to this register results with the clear of the latched
pub const MISC_REG_AEU_CLR_LATCH_SIGNAL: c_uint = 0xa45c;
// [RW 32] first 32b for enabling the output for function 0 output0. mapped
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_0: c_uint = 0xa06c;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_1: c_uint = 0xa07c;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_2: c_uint = 0xa08c;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_3: c_uint = 0xa09c;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_5: c_uint = 0xa0bc;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_6: c_uint = 0xa0cc;
pub const MISC_REG_AEU_ENABLE1_FUNC_0_OUT_7: c_uint = 0xa0dc;
// [RW 32] first 32b for enabling the output for function 1 output0. mapped
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_0: c_uint = 0xa10c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_1: c_uint = 0xa11c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_2: c_uint = 0xa12c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_3: c_uint = 0xa13c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_5: c_uint = 0xa15c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_6: c_uint = 0xa16c;
pub const MISC_REG_AEU_ENABLE1_FUNC_1_OUT_7: c_uint = 0xa17c;
// [RW 32] first 32b for enabling the output for close the gate nig. mapped
pub const MISC_REG_AEU_ENABLE1_NIG_0: c_uint = 0xa0ec;
pub const MISC_REG_AEU_ENABLE1_NIG_1: c_uint = 0xa18c;
// [RW 32] first 32b for enabling the output for close the gate pxp. mapped
pub const MISC_REG_AEU_ENABLE1_PXP_0: c_uint = 0xa0fc;
pub const MISC_REG_AEU_ENABLE1_PXP_1: c_uint = 0xa19c;
// [RW 32] second 32b for enabling the output for function 0 output0. mapped
pub const MISC_REG_AEU_ENABLE2_FUNC_0_OUT_0: c_uint = 0xa070;
pub const MISC_REG_AEU_ENABLE2_FUNC_0_OUT_1: c_uint = 0xa080;
// [RW 32] second 32b for enabling the output for function 1 output0. mapped
pub const MISC_REG_AEU_ENABLE2_FUNC_1_OUT_0: c_uint = 0xa110;
pub const MISC_REG_AEU_ENABLE2_FUNC_1_OUT_1: c_uint = 0xa120;
// [RW 32] second 32b for enabling the output for close the gate nig. mapped
pub const MISC_REG_AEU_ENABLE2_NIG_0: c_uint = 0xa0f0;
pub const MISC_REG_AEU_ENABLE2_NIG_1: c_uint = 0xa190;
// [RW 32] second 32b for enabling the output for close the gate pxp. mapped
pub const MISC_REG_AEU_ENABLE2_PXP_0: c_uint = 0xa100;
pub const MISC_REG_AEU_ENABLE2_PXP_1: c_uint = 0xa1a0;
// [RW 32] third 32b for enabling the output for function 0 output0. mapped
pub const MISC_REG_AEU_ENABLE3_FUNC_0_OUT_0: c_uint = 0xa074;
pub const MISC_REG_AEU_ENABLE3_FUNC_0_OUT_1: c_uint = 0xa084;
// [RW 32] third 32b for enabling the output for function 1 output0. mapped
pub const MISC_REG_AEU_ENABLE3_FUNC_1_OUT_0: c_uint = 0xa114;
pub const MISC_REG_AEU_ENABLE3_FUNC_1_OUT_1: c_uint = 0xa124;
// [RW 32] third 32b for enabling the output for close the gate nig. mapped
pub const MISC_REG_AEU_ENABLE3_NIG_0: c_uint = 0xa0f4;
pub const MISC_REG_AEU_ENABLE3_NIG_1: c_uint = 0xa194;
// [RW 32] third 32b for enabling the output for close the gate pxp. mapped
pub const MISC_REG_AEU_ENABLE3_PXP_0: c_uint = 0xa104;
pub const MISC_REG_AEU_ENABLE3_PXP_1: c_uint = 0xa1a4;
// [RW 32] fourth 32b for enabling the output for function 0 output0.mapped
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_0: c_uint = 0xa078;
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_2: c_uint = 0xa098;
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_4: c_uint = 0xa0b8;
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_5: c_uint = 0xa0c8;
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_6: c_uint = 0xa0d8;
pub const MISC_REG_AEU_ENABLE4_FUNC_0_OUT_7: c_uint = 0xa0e8;
// [RW 32] fourth 32b for enabling the output for function 1 output0.mapped
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_0: c_uint = 0xa118;
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_2: c_uint = 0xa138;
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_4: c_uint = 0xa158;
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_5: c_uint = 0xa168;
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_6: c_uint = 0xa178;
pub const MISC_REG_AEU_ENABLE4_FUNC_1_OUT_7: c_uint = 0xa188;
// [RW 32] fourth 32b for enabling the output for close the gate nig.mapped
pub const MISC_REG_AEU_ENABLE4_NIG_0: c_uint = 0xa0f8;
pub const MISC_REG_AEU_ENABLE4_NIG_1: c_uint = 0xa198;
// [RW 32] fourth 32b for enabling the output for close the gate pxp.mapped
pub const MISC_REG_AEU_ENABLE4_PXP_0: c_uint = 0xa108;
pub const MISC_REG_AEU_ENABLE4_PXP_1: c_uint = 0xa1a8;
// [RW 32] fifth 32b for enabling the output for function 0 output0. Mapped
// as follows: [0] PGLUE config_space; [1] PGLUE misc_flr; [2] PGLUE B RBC
// attention [3] PGLUE B RBC parity; [4] ATC attention; [5] ATC parity; [6]
// mstat0 attention; [7] mstat0 parity; [8] mstat1 attention; [9] mstat1
// parity; [31-10] Reserved;
pub const MISC_REG_AEU_ENABLE5_FUNC_0_OUT_0: c_uint = 0xa688;
// [RW 32] Fifth 32b for enabling the output for function 1 output0. Mapped
// as follows: [0] PGLUE config_space; [1] PGLUE misc_flr; [2] PGLUE B RBC
// attention [3] PGLUE B RBC parity; [4] ATC attention; [5] ATC parity; [6]
// mstat0 attention; [7] mstat0 parity; [8] mstat1 attention; [9] mstat1
// parity; [31-10] Reserved;
pub const MISC_REG_AEU_ENABLE5_FUNC_1_OUT_0: c_uint = 0xa6b0;
// [RW 1] set/clr general attention 0; this will set/clr bit 94 in the aeu
pub const MISC_REG_AEU_GENERAL_ATTN_0: c_uint = 0xa000;
pub const MISC_REG_AEU_GENERAL_ATTN_1: c_uint = 0xa004;
pub const MISC_REG_AEU_GENERAL_ATTN_10: c_uint = 0xa028;
pub const MISC_REG_AEU_GENERAL_ATTN_11: c_uint = 0xa02c;
pub const MISC_REG_AEU_GENERAL_ATTN_12: c_uint = 0xa030;
pub const MISC_REG_AEU_GENERAL_ATTN_2: c_uint = 0xa008;
pub const MISC_REG_AEU_GENERAL_ATTN_3: c_uint = 0xa00c;
pub const MISC_REG_AEU_GENERAL_ATTN_4: c_uint = 0xa010;
pub const MISC_REG_AEU_GENERAL_ATTN_5: c_uint = 0xa014;
pub const MISC_REG_AEU_GENERAL_ATTN_6: c_uint = 0xa018;
pub const MISC_REG_AEU_GENERAL_ATTN_7: c_uint = 0xa01c;
pub const MISC_REG_AEU_GENERAL_ATTN_8: c_uint = 0xa020;
pub const MISC_REG_AEU_GENERAL_ATTN_9: c_uint = 0xa024;
pub const MISC_REG_AEU_GENERAL_MASK: c_uint = 0xa61c;
// [RW 32] first 32b for inverting the input for function 0; for each bit:
pub const MISC_REG_AEU_INVERTER_1_FUNC_0: c_uint = 0xa22c;
pub const MISC_REG_AEU_INVERTER_1_FUNC_1: c_uint = 0xa23c;
// [RW 32] second 32b for inverting the input for function 0; for each bit:
pub const MISC_REG_AEU_INVERTER_2_FUNC_0: c_uint = 0xa230;
pub const MISC_REG_AEU_INVERTER_2_FUNC_1: c_uint = 0xa240;
// [RW 10] [7:0] = mask 8 attention output signals toward IGU function0;
pub const MISC_REG_AEU_MASK_ATTN_FUNC_0: c_uint = 0xa060;
pub const MISC_REG_AEU_MASK_ATTN_FUNC_1: c_uint = 0xa064;
// [RW 1] If set a system kill occurred
pub const MISC_REG_AEU_SYS_KILL_OCCURRED: c_uint = 0xa610;
// [RW 32] Represent the status of the input vector to the AEU when a system
pub const MISC_REG_AEU_SYS_KILL_STATUS_0: c_uint = 0xa600;
pub const MISC_REG_AEU_SYS_KILL_STATUS_1: c_uint = 0xa604;
pub const MISC_REG_AEU_SYS_KILL_STATUS_2: c_uint = 0xa608;
pub const MISC_REG_AEU_SYS_KILL_STATUS_3: c_uint = 0xa60c;
// [R 4] This field indicates the type of the device. '0' - 2 Ports; '1' - 1
pub const MISC_REG_BOND_ID: c_uint = 0xa400;
// [R 16] These bits indicate the part number for the chip.
pub const MISC_REG_CHIP_NUM: c_uint = 0xa408;
// [R 4] These bits indicate the base revision of the chip. This value
pub const MISC_REG_CHIP_REV: c_uint = 0xa40c;
// [R 14] otp_misc_do[100:0] spare bits collection: 13:11-
// otp_misc_do[100:98]; 10:7 - otp_misc_do[87:84]; 6:3 - otp_misc_do[75:72];
// 2:1 - otp_misc_do[51:50]; 0 - otp_misc_do[1].
pub const MISC_REG_CHIP_TYPE: c_uint = 0xac60;

pub const MISC_REG_CPMU_LP_DR_ENABLE: c_uint = 0xa858;
// [RW 1] FW EEE LPI Enable. When 1 indicates that EEE LPI mode is enabled
// by FW. When 0 indicates that the EEE LPI mode is disabled by FW. Clk
// 25MHz. Reset on hard reset.
pub const MISC_REG_CPMU_LP_FW_ENABLE_P0: c_uint = 0xa84c;
// [RW 32] EEE LPI Idle Threshold. The threshold value for the idle EEE LPI
// counter. Timer tick is 1 us. Clock 25MHz. Reset on hard reset.
pub const MISC_REG_CPMU_LP_IDLE_THR_P0: c_uint = 0xa8a0;
// [RW 18] LPI entry events mask. [0] - Vmain SM Mask. When 1 indicates that
// the Vmain SM end state is disabled. When 0 indicates that the Vmain SM
// end state is enabled. [1] - FW Queues Empty Mask. When 1 indicates that
// the FW command that all Queues are empty is disabled. When 0 indicates
// that the FW command that all Queues are empty is enabled. [2] - FW Early
// Exit Mask / Reserved (Entry mask). When 1 indicates that the FW Early
// Exit command is disabled. When 0 indicates that the FW Early Exit command
// is enabled. This bit applicable only in the EXIT Events Mask registers.
// [3] - PBF Request Mask. When 1 indicates that the PBF Request indication
// is disabled. When 0 indicates that the PBF Request indication is enabled.
// [4] - Tx Request Mask. When =1 indicates that the Tx other Than PBF
// Request indication is disabled. When 0 indicates that the Tx Other Than
// PBF Request indication is enabled. [5] - Rx EEE LPI Status Mask. When 1
// indicates that the RX EEE LPI Status indication is disabled. When 0
// indicates that the RX EEE LPI Status indication is enabled. In the EXIT
// Events Masks registers; this bit masks the falling edge detect of the LPI
// Status (Rx LPI is on - off). [6] - Tx Pause Mask. When 1 indicates that
// the Tx Pause indication is disabled. When 0 indicates that the Tx Pause
// indication is enabled. [7] - BRB1 Empty Mask. When 1 indicates that the
// BRB1 EMPTY indication is disabled. When 0 indicates that the BRB1 EMPTY
// indication is enabled. [8] - QM Idle Mask. When 1 indicates that the QM
// IDLE indication is disabled. When 0 indicates that the QM IDLE indication
// is enabled. (One bit for both VOQ0 and VOQ1). [9] - QM LB Idle Mask. When
// 1 indicates that the QM IDLE indication for LOOPBACK is disabled. When 0
// indicates that the QM IDLE indication for LOOPBACK is enabled. [10] - L1
// Status Mask. When 1 indicates that the L1 Status indication from the PCIE
// CORE is disabled. When 0 indicates that the RX EEE LPI Status indication
// from the PCIE CORE is enabled. In the EXIT Events Masks registers; this
// bit masks the falling edge detect of the L1 status (L1 is on - off). [11]
// - P0 E0 EEE EEE LPI REQ Mask. When =1 indicates that the P0 E0 EEE EEE
// LPI REQ indication is disabled. When =0 indicates that the P0 E0 EEE LPI
// REQ indication is enabled. [12] - P1 E0 EEE LPI REQ Mask. When =1
// indicates that the P0 EEE LPI REQ indication is disabled. When =0
// indicates that the P0 EEE LPI REQ indication is enabled. [13] - P0 E1 EEE
// LPI REQ Mask. When =1 indicates that the P0 EEE LPI REQ indication is
// disabled. When =0 indicates that the P0 EEE LPI REQ indication is
// enabled. [14] - P1 E1 EEE LPI REQ Mask. When =1 indicates that the P0 EEE
// LPI REQ indication is disabled. When =0 indicates that the P0 EEE LPI REQ
// indication is enabled. [15] - L1 REQ Mask. When =1 indicates that the L1
// REQ indication is disabled. When =0 indicates that the L1 indication is
// enabled. [16] - Rx EEE LPI Status Edge Detect Mask. When =1 indicates
// that the RX EEE LPI Status Falling Edge Detect indication is disabled (Rx
// EEE LPI is on - off). When =0 indicates that the RX EEE LPI Status
// Falling Edge Detec indication is enabled (Rx EEE LPI is on - off). This
// bit is applicable only in the EXIT Events Masks registers. [17] - L1
// Status Edge Detect Mask. When =1 indicates that the L1 Status Falling
// Edge Detect indication from the PCIE CORE is disabled (L1 is on - off).
// When =0 indicates that the L1 Status Falling Edge Detect indication from
// the PCIE CORE is enabled (L1 is on - off). This bit is applicable only in
// the EXIT Events Masks registers. Clock 25MHz. Reset on hard reset.
pub const MISC_REG_CPMU_LP_MASK_ENT_P0: c_uint = 0xa880;
// [RW 18] EEE LPI exit events mask. [0] - Vmain SM Mask. When 1 indicates
// that the Vmain SM end state is disabled. When 0 indicates that the Vmain
// SM end state is enabled. [1] - FW Queues Empty Mask. When 1 indicates
// that the FW command that all Queues are empty is disabled. When 0
// indicates that the FW command that all Queues are empty is enabled. [2] -
// FW Early Exit Mask / Reserved (Entry mask). When 1 indicates that the FW
// Early Exit command is disabled. When 0 indicates that the FW Early Exit
// command is enabled. This bit applicable only in the EXIT Events Mask
// registers. [3] - PBF Request Mask. When 1 indicates that the PBF Request
// indication is disabled. When 0 indicates that the PBF Request indication
// is enabled. [4] - Tx Request Mask. When =1 indicates that the Tx other
// Than PBF Request indication is disabled. When 0 indicates that the Tx
// Other Than PBF Request indication is enabled. [5] - Rx EEE LPI Status
// Mask. When 1 indicates that the RX EEE LPI Status indication is disabled.
// When 0 indicates that the RX LPI Status indication is enabled. In the
// EXIT Events Masks registers; this bit masks the falling edge detect of
// the EEE LPI Status (Rx EEE LPI is on - off). [6] - Tx Pause Mask. When 1
// indicates that the Tx Pause indication is disabled. When 0 indicates that
// the Tx Pause indication is enabled. [7] - BRB1 Empty Mask. When 1
// indicates that the BRB1 EMPTY indication is disabled. When 0 indicates
// that the BRB1 EMPTY indication is enabled. [8] - QM Idle Mask. When 1
// indicates that the QM IDLE indication is disabled. When 0 indicates that
// the QM IDLE indication is enabled. (One bit for both VOQ0 and VOQ1). [9]
// - QM LB Idle Mask. When 1 indicates that the QM IDLE indication for
// LOOPBACK is disabled. When 0 indicates that the QM IDLE indication for
// LOOPBACK is enabled. [10] - L1 Status Mask. When 1 indicates that the L1
// Status indication from the PCIE CORE is disabled. When 0 indicates that
// the RX EEE LPI Status indication from the PCIE CORE is enabled. In the
// EXIT Events Masks registers; this bit masks the falling edge detect of
// the L1 status (L1 is on - off). [11] - P0 E0 EEE EEE LPI REQ Mask. When
// =1 indicates that the P0 E0 EEE EEE LPI REQ indication is disabled. When
// =0 indicates that the P0 E0 EEE LPI REQ indication is enabled. [12] - P1
// E0 EEE LPI REQ Mask. When =1 indicates that the P0 EEE LPI REQ indication
// is disabled. When =0 indicates that the P0 EEE LPI REQ indication is
// enabled. [13] - P0 E1 EEE LPI REQ Mask. When =1 indicates that the P0 EEE
// LPI REQ indication is disabled. When =0 indicates that the P0 EEE LPI REQ
// indication is enabled. [14] - P1 E1 EEE LPI REQ Mask. When =1 indicates
// that the P0 EEE LPI REQ indication is disabled. When =0 indicates that
// the P0 EEE LPI REQ indication is enabled. [15] - L1 REQ Mask. When =1
// indicates that the L1 REQ indication is disabled. When =0 indicates that
// the L1 indication is enabled. [16] - Rx EEE LPI Status Edge Detect Mask.
// When =1 indicates that the RX EEE LPI Status Falling Edge Detect
// indication is disabled (Rx EEE LPI is on - off). When =0 indicates that
// the RX EEE LPI Status Falling Edge Detec indication is enabled (Rx EEE
// LPI is on - off). This bit is applicable only in the EXIT Events Masks
// registers. [17] - L1 Status Edge Detect Mask. When =1 indicates that the
// L1 Status Falling Edge Detect indication from the PCIE CORE is disabled
// (L1 is on - off). When =0 indicates that the L1 Status Falling Edge
// Detect indication from the PCIE CORE is enabled (L1 is on - off). This
// bit is applicable only in the EXIT Events Masks registers.Clock 25MHz.
// Reset on hard reset.
pub const MISC_REG_CPMU_LP_MASK_EXT_P0: c_uint = 0xa888;
// [RW 16] EEE LPI Entry Events Counter. A statistic counter with the number
// of counts that the SM entered the EEE LPI state. Clock 25MHz. Read only
// register. Reset on hard reset.
pub const MISC_REG_CPMU_LP_SM_ENT_CNT_P0: c_uint = 0xa8b8;
// [RW 16] EEE LPI Entry Events Counter. A statistic counter with the number
// of counts that the SM entered the EEE LPI state. Clock 25MHz. Read only
// register. Reset on hard reset.
pub const MISC_REG_CPMU_LP_SM_ENT_CNT_P1: c_uint = 0xa8bc;
// [RW 32] The following driver registers(1...16) represent 16 drivers and
pub const MISC_REG_DRIVER_CONTROL_1: c_uint = 0xa510;
pub const MISC_REG_DRIVER_CONTROL_7: c_uint = 0xa3c8;
// [RW 1] e1hmf for WOL. If clr WOL signal o the PXP will be send on bit 0
pub const MISC_REG_E1HMF_MODE: c_uint = 0xa5f8;
// [R 1] Status of four port mode path swap input pin.
pub const MISC_REG_FOUR_PORT_PATH_SWAP: c_uint = 0xa75c;
// [RW 2] 4 port path swap overwrite.[0] - Overwrite control; if it is 0 -
pub const MISC_REG_FOUR_PORT_PATH_SWAP_OVWR: c_uint = 0xa738;
// [R 1] Status of 4 port mode port swap input pin.
pub const MISC_REG_FOUR_PORT_PORT_SWAP: c_uint = 0xa754;
// [RW 2] 4 port port swap overwrite.[0] - Overwrite control; if it is 0 -
pub const MISC_REG_FOUR_PORT_PORT_SWAP_OVWR: c_uint = 0xa734;
// [RW 32] Debug only: spare RW register reset by core reset
pub const MISC_REG_GENERIC_CR_0: c_uint = 0xa460;
pub const MISC_REG_GENERIC_CR_1: c_uint = 0xa464;
// [RW 32] Debug only: spare RW register reset by por reset
pub const MISC_REG_GENERIC_POR_1: c_uint = 0xa474;
// [RW 32] Bit[0]: EPIO MODE SEL: Setting this bit to 1 will allow SW/FW to
pub const MISC_REG_GEN_PURP_HWG: c_uint = 0xa9a0;
// [RW 32] GPIO. [31-28] FLOAT port 0; [27-24] FLOAT port 0; When any of
pub const MISC_REG_GPIO: c_uint = 0xa490;
// [RW 8] These bits enable the GPIO_INTs to signals event to the
pub const MISC_REG_GPIO_EVENT_EN: c_uint = 0xa2bc;
// [RW 32] GPIO INT. [31-28] OLD_CLR port1; [27-24] OLD_CLR port0; Writing a
pub const MISC_REG_GPIO_INT: c_uint = 0xa494;
// [R 28] this field hold the last information that caused reserved
pub const MISC_REG_GRC_RSV_ATTN: c_uint = 0xa3c0;
// [R 28] this field hold the last information that caused timeout
pub const MISC_REG_GRC_TIMEOUT_ATTN: c_uint = 0xa3c4;
// [RW 1] Setting this bit enables a timer in the GRC block to timeout any
pub const MISC_REG_GRC_TIMEOUT_EN: c_uint = 0xa280;
// [RW 28] 28 LSB of LCPLL first register; reset val = 521. inside order of
pub const MISC_REG_LCPLL_CTRL_1: c_uint = 0xa2a4;
pub const MISC_REG_LCPLL_CTRL_REG_2: c_uint = 0xa2a8;
// [RW 1] LCPLL power down. Global register. Active High. Reset on POR
// reset.
pub const MISC_REG_LCPLL_E40_PWRDWN: c_uint = 0xaa74;
// [RW 1] LCPLL VCO reset. Global register. Active Low Reset on POR reset.
pub const MISC_REG_LCPLL_E40_RESETB_ANA: c_uint = 0xaa78;
// [RW 1] LCPLL post-divider reset. Global register. Active Low Reset on POR
// reset.
pub const MISC_REG_LCPLL_E40_RESETB_DIG: c_uint = 0xaa7c;
// [RW 4] Interrupt mask register #0 read/write
pub const MISC_REG_MISC_INT_MASK: c_uint = 0xa388;
// [RW 1] Parity mask register #0 read/write
pub const MISC_REG_MISC_PRTY_MASK: c_uint = 0xa398;
// [R 1] Parity register #0 read
pub const MISC_REG_MISC_PRTY_STS: c_uint = 0xa38c;
// [RC 1] Parity register #0 read clear
pub const MISC_REG_MISC_PRTY_STS_CLR: c_uint = 0xa390;
pub const MISC_REG_NIG_WOL_P0: c_uint = 0xa270;
pub const MISC_REG_NIG_WOL_P1: c_uint = 0xa274;
// [R 1] If set indicate that the pcie_rst_b was asserted without perst
pub const MISC_REG_PCIE_HOT_RESET: c_uint = 0xa618;
// [RW 32] 32 LSB of storm PLL first register; reset val = 0x 071d2911.
pub const MISC_REG_PLL_STORM_CTRL_1: c_uint = 0xa294;
pub const MISC_REG_PLL_STORM_CTRL_2: c_uint = 0xa298;
pub const MISC_REG_PLL_STORM_CTRL_3: c_uint = 0xa29c;
pub const MISC_REG_PLL_STORM_CTRL_4: c_uint = 0xa2a0;
// [R 1] Status of 4 port mode enable input pin.
pub const MISC_REG_PORT4MODE_EN: c_uint = 0xa750;
// [RW 2] 4 port mode enable overwrite.[0] - Overwrite control; if it is 0 -
// the port4mode_en output is equal to 4 port mode input pin; if it is 1 -
// the port4mode_en output is equal to bit[1] of this register; [1] -
// Overwrite value. If bit[0] of this register is 1 this is the value that
// receives the port4mode_en output .
pub const MISC_REG_PORT4MODE_EN_OVWR: c_uint = 0xa720;
// [RW 32] reset reg#2; rite/read one = the specific block is out of reset;
pub const MISC_REG_RESET_REG_1: c_uint = 0xa580;
pub const MISC_REG_RESET_REG_2: c_uint = 0xa590;
// [RW 20] 20 bit GRC address where the scratch-pad of the MCP that is
pub const MISC_REG_SHARED_MEM_ADDR: c_uint = 0xa2b4;
// [RW 32] SPIO. [31-24] FLOAT When any of these bits is written as a '1';

pub const MISC_REG_SPIO: c_uint = 0xa4fc;
// [RW 8] These bits enable the SPIO_INTs to signals event to the IGU/MC.
pub const MISC_REG_SPIO_EVENT_EN: c_uint = 0xa2b8;
// [RW 32] SPIO INT. [31-24] OLD_CLR Writing a '1' to these bit clears the
pub const MISC_REG_SPIO_INT: c_uint = 0xa500;
// [RW 32] reload value for counter 4 if reload; the value will be reload if
pub const MISC_REG_SW_TIMER_RELOAD_VAL_4: c_uint = 0xa2fc;
// [RW 32] the value of the counter for sw timers1-8. there are 8 addresses
pub const MISC_REG_SW_TIMER_VAL: c_uint = 0xa5c0;
// [R 1] Status of two port mode path swap input pin.
pub const MISC_REG_TWO_PORT_PATH_SWAP: c_uint = 0xa758;
// [RW 2] 2 port swap overwrite.[0] - Overwrite control; if it is 0 - the
pub const MISC_REG_TWO_PORT_PATH_SWAP_OVWR: c_uint = 0xa72c;
// [RW 1] Set by the MCP to remember if one or more of the drivers is/are
pub const MISC_REG_UNPREPARED: c_uint = 0xa424;

// [RW 5] MDIO PHY Address. The WC uses this address to determine whether or
// not it is the recipient of the message on the MDIO interface. The value
// is compared to the value on ctrl_md_devad. Drives output
// misc_xgxs0_phy_addr. Global register.
pub const MISC_REG_WC0_CTRL_PHY_ADDR: c_uint = 0xa9cc;
pub const MISC_REG_WC0_RESET: c_uint = 0xac30;
// [RW 2] XMAC Core port mode. Indicates the number of ports on the system
pub const MISC_REG_XMAC_CORE_PORT_MODE: c_uint = 0xa964;
// [RW 2] XMAC PHY port mode. Indicates the number of ports on the Warp
pub const MISC_REG_XMAC_PHY_PORT_MODE: c_uint = 0xa960;
// [RW 32] 1 [47] Packet Size = 64 Write to this register write bits 31:0.
// Reads from this register will clear bits 31:0.
pub const MSTAT_REG_RX_STAT_GR64_LO: c_uint = 0x200;
// [RW 32] 1 [00] Tx Good Packet Count Write to this register write bits
// 31:0. Reads from this register will clear bits 31:0.
pub const MSTAT_REG_TX_STAT_GTXPOK_LO: c_int = 0;

// [RW 1] Input enable for RX_BMAC0 IF
pub const NIG_REG_BMAC0_IN_EN: c_uint = 0x100ac;
// [RW 1] output enable for TX_BMAC0 IF
pub const NIG_REG_BMAC0_OUT_EN: c_uint = 0x100e0;
// [RW 1] output enable for TX BMAC pause port 0 IF
pub const NIG_REG_BMAC0_PAUSE_OUT_EN: c_uint = 0x10110;
// [RW 1] output enable for RX_BMAC0_REGS IF
pub const NIG_REG_BMAC0_REGS_OUT_EN: c_uint = 0x100e8;
// [RW 1] output enable for RX BRB1 port0 IF
pub const NIG_REG_BRB0_OUT_EN: c_uint = 0x100f8;
// [RW 1] Input enable for TX BRB1 pause port 0 IF
pub const NIG_REG_BRB0_PAUSE_IN_EN: c_uint = 0x100c4;
// [RW 1] output enable for RX BRB1 port1 IF
pub const NIG_REG_BRB1_OUT_EN: c_uint = 0x100fc;
// [RW 1] Input enable for TX BRB1 pause port 1 IF
pub const NIG_REG_BRB1_PAUSE_IN_EN: c_uint = 0x100c8;
// [RW 1] output enable for RX BRB1 LP IF
pub const NIG_REG_BRB_LB_OUT_EN: c_uint = 0x10100;
// [WB_W 82] Debug packet to LP from RBC; Data spelling:[63:0] data; 64]
pub const NIG_REG_DEBUG_PACKET_LB: c_uint = 0x10800;
// [RW 1] Input enable for TX Debug packet
pub const NIG_REG_EGRESS_DEBUG_IN_EN: c_uint = 0x100dc;
// [RW 1] If 1 - egress drain mode for port0 is active. In this mode all
pub const NIG_REG_EGRESS_DRAIN0_MODE: c_uint = 0x10060;
// [RW 1] Output enable to EMAC0
pub const NIG_REG_EGRESS_EMAC0_OUT_EN: c_uint = 0x10120;
// [RW 1] MAC configuration for packets of port0. If 1 - all packet outputs
pub const NIG_REG_EGRESS_EMAC0_PORT: c_uint = 0x10058;
// [RW 1] Input enable for TX PBF user packet port0 IF
pub const NIG_REG_EGRESS_PBF0_IN_EN: c_uint = 0x100cc;
// [RW 1] Input enable for TX PBF user packet port1 IF
pub const NIG_REG_EGRESS_PBF1_IN_EN: c_uint = 0x100d0;
// [RW 1] Input enable for TX UMP management packet port0 IF
pub const NIG_REG_EGRESS_UMP0_IN_EN: c_uint = 0x100d4;
// [RW 1] Input enable for RX_EMAC0 IF
pub const NIG_REG_EMAC0_IN_EN: c_uint = 0x100a4;
// [RW 1] output enable for TX EMAC pause port 0 IF
pub const NIG_REG_EMAC0_PAUSE_OUT_EN: c_uint = 0x10118;
// [R 1] status from emac0. This bit is set when MDINT from either the
pub const NIG_REG_EMAC0_STATUS_MISC_MI_INT: c_uint = 0x10494;
// [WB 48] This address space contains BMAC0 registers. The BMAC registers
pub const NIG_REG_INGRESS_BMAC0_MEM: c_uint = 0x10c00;
// [WB 48] This address space contains BMAC1 registers. The BMAC registers
pub const NIG_REG_INGRESS_BMAC1_MEM: c_uint = 0x11000;
// [R 1] FIFO empty in EOP descriptor FIFO of LP in NIG_RX_EOP
pub const NIG_REG_INGRESS_EOP_LB_EMPTY: c_uint = 0x104e0;
// [RW 17] Debug only. RX_EOP_DSCR_lb_FIFO in NIG_RX_EOP. Data
pub const NIG_REG_INGRESS_EOP_LB_FIFO: c_uint = 0x104e4;
// [RW 27] 0 - must be active for Everest A0; 1- for Everest B0 when latch
pub const NIG_REG_LATCH_BC_0: c_uint = 0x16210;
// [RW 27] Latch for each interrupt from Unicore.b[0]
pub const NIG_REG_LATCH_STATUS_0: c_uint = 0x18000;
// [RW 1] led 10g for port 0
pub const NIG_REG_LED_10G_P0: c_uint = 0x10320;
// [RW 1] led 10g for port 1
pub const NIG_REG_LED_10G_P1: c_uint = 0x10324;
// [RW 1] Port0: This bit is set to enable the use of the
pub const NIG_REG_LED_CONTROL_BLINK_RATE_ENA_P0: c_uint = 0x10318;
// [RW 12] Port0: Specifies the period of each blink cycle (on + off) for
pub const NIG_REG_LED_CONTROL_BLINK_RATE_P0: c_uint = 0x10310;
// [RW 1] Port0: If set along with the
pub const NIG_REG_LED_CONTROL_BLINK_TRAFFIC_P0: c_uint = 0x10308;
// [RW 1] Port0: If set overrides hardware control of the Traffic LED. The
pub const NIG_REG_LED_CONTROL_OVERRIDE_TRAFFIC_P0: c_uint = 0x102f8;
// [RW 1] Port0: If set along with the led_control_override_trafic_p0 bit;
pub const NIG_REG_LED_CONTROL_TRAFFIC_P0: c_uint = 0x10300;
// [RW 4] led mode for port0: 0 MAC; 1-3 PHY1; 4 MAC2; 5-7 PHY4; 8-MAC3;
pub const NIG_REG_LED_MODE_P0: c_uint = 0x102f0;
// [RW 3] for port0 enable for llfc ppp and pause. b0 - brb1 enable; b1-
pub const NIG_REG_LLFC_EGRESS_SRC_ENABLE_0: c_uint = 0x16070;
pub const NIG_REG_LLFC_EGRESS_SRC_ENABLE_1: c_uint = 0x16074;
// [RW 1] SAFC enable for port0. This register may get 1 only when
pub const NIG_REG_LLFC_ENABLE_0: c_uint = 0x16208;
pub const NIG_REG_LLFC_ENABLE_1: c_uint = 0x1620c;
// [RW 16] classes are high-priority for port0
pub const NIG_REG_LLFC_HIGH_PRIORITY_CLASSES_0: c_uint = 0x16058;
pub const NIG_REG_LLFC_HIGH_PRIORITY_CLASSES_1: c_uint = 0x1605c;
// [RW 16] classes are low-priority for port0
pub const NIG_REG_LLFC_LOW_PRIORITY_CLASSES_0: c_uint = 0x16060;
pub const NIG_REG_LLFC_LOW_PRIORITY_CLASSES_1: c_uint = 0x16064;
// [RW 1] Output enable of message to LLFC BMAC IF for port0
pub const NIG_REG_LLFC_OUT_EN_0: c_uint = 0x160c8;
pub const NIG_REG_LLFC_OUT_EN_1: c_uint = 0x160cc;
pub const NIG_REG_LLH0_ACPI_PAT_0_CRC: c_uint = 0x1015c;
pub const NIG_REG_LLH0_ACPI_PAT_6_LEN: c_uint = 0x10154;
pub const NIG_REG_LLH0_BRB1_DRV_MASK: c_uint = 0x10244;
pub const NIG_REG_LLH0_BRB1_DRV_MASK_MF: c_uint = 0x16048;
// [RW 1] send to BRB1 if no match on any of RMP rules.
pub const NIG_REG_LLH0_BRB1_NOT_MCP: c_uint = 0x1025c;
// [RW 2] Determine the classification participants. 0: no classification.1:
pub const NIG_REG_LLH0_CLS_TYPE: c_uint = 0x16080;
// [RW 32] cm header for llh0
pub const NIG_REG_LLH0_CM_HEADER: c_uint = 0x1007c;
pub const NIG_REG_LLH0_DEST_IP_0_1: c_uint = 0x101dc;
pub const NIG_REG_LLH0_DEST_MAC_0_0: c_uint = 0x101c0;
// [RW 16] destination TCP address 1. The LLH will look for this address in
pub const NIG_REG_LLH0_DEST_TCP_0: c_uint = 0x10220;
// [RW 16] destination UDP address 1 The LLH will look for this address in
pub const NIG_REG_LLH0_DEST_UDP_0: c_uint = 0x10214;
pub const NIG_REG_LLH0_ERROR_MASK: c_uint = 0x1008c;
// [RW 8] event id for llh0
pub const NIG_REG_LLH0_EVENT_ID: c_uint = 0x10084;
pub const NIG_REG_LLH0_FUNC_EN: c_uint = 0x160fc;
pub const NIG_REG_LLH0_FUNC_MEM: c_uint = 0x16180;
pub const NIG_REG_LLH0_FUNC_MEM_ENABLE: c_uint = 0x16140;
pub const NIG_REG_LLH0_FUNC_VLAN_ID: c_uint = 0x16100;
// [RW 1] Determine the IP version to look for in
pub const NIG_REG_LLH0_IPV4_IPV6_0: c_uint = 0x10208;
// [RW 1] t bit for llh0
pub const NIG_REG_LLH0_T_BIT: c_uint = 0x10074;
// [RW 12] VLAN ID 1. In case of VLAN packet the LLH will look for this ID.
pub const NIG_REG_LLH0_VLAN_ID_0: c_uint = 0x1022c;
// [RW 8] init credit counter for port0 in LLH
pub const NIG_REG_LLH0_XCM_INIT_CREDIT: c_uint = 0x10554;
pub const NIG_REG_LLH0_XCM_MASK: c_uint = 0x10130;
pub const NIG_REG_LLH1_BRB1_DRV_MASK: c_uint = 0x10248;
// [RW 1] send to BRB1 if no match on any of RMP rules.
pub const NIG_REG_LLH1_BRB1_NOT_MCP: c_uint = 0x102dc;
// [RW 2] Determine the classification participants. 0: no classification.1:
pub const NIG_REG_LLH1_CLS_TYPE: c_uint = 0x16084;
// [RW 32] cm header for llh1
pub const NIG_REG_LLH1_CM_HEADER: c_uint = 0x10080;
pub const NIG_REG_LLH1_ERROR_MASK: c_uint = 0x10090;
// [RW 8] event id for llh1
pub const NIG_REG_LLH1_EVENT_ID: c_uint = 0x10088;
pub const NIG_REG_LLH1_FUNC_EN: c_uint = 0x16104;
pub const NIG_REG_LLH1_FUNC_MEM: c_uint = 0x161c0;
pub const NIG_REG_LLH1_FUNC_MEM_ENABLE: c_uint = 0x16160;
pub const NIG_REG_LLH1_FUNC_MEM_SIZE: c_int = 16;
// [RW 1] When this bit is set; the LLH will classify the packet before
// sending it to the BRB or calculating WoL on it. This bit controls port 1
// only. The legacy llh_multi_function_mode bit controls port 0.
pub const NIG_REG_LLH1_MF_MODE: c_uint = 0x18614;
// [RW 8] init credit counter for port1 in LLH
pub const NIG_REG_LLH1_XCM_INIT_CREDIT: c_uint = 0x10564;
pub const NIG_REG_LLH1_XCM_MASK: c_uint = 0x10134;
// [RW 1] When this bit is set; the LLH will expect all packets to be with
pub const NIG_REG_LLH_E1HOV_MODE: c_uint = 0x160d8;
// [RW 16] Outer VLAN type identifier for multi-function mode. In non
// multi-function mode; it will hold the inner VLAN type. Typically 0x8100.
//
pub const NIG_REG_LLH_E1HOV_TYPE_1: c_uint = 0x16028;
// [RW 1] When this bit is set; the LLH will classify the packet before
pub const NIG_REG_LLH_MF_MODE: c_uint = 0x16024;
pub const NIG_REG_MASK_INTERRUPT_PORT0: c_uint = 0x10330;
pub const NIG_REG_MASK_INTERRUPT_PORT1: c_uint = 0x10334;
// [RW 1] Output signal from NIG to EMAC0. When set enables the EMAC0 block.
pub const NIG_REG_NIG_EMAC0_EN: c_uint = 0x1003c;
// [RW 1] Output signal from NIG to EMAC1. When set enables the EMAC1 block.
pub const NIG_REG_NIG_EMAC1_EN: c_uint = 0x10040;
// [RW 1] Output signal from NIG to TX_EMAC0. When set indicates to the
pub const NIG_REG_NIG_INGRESS_EMAC0_NO_CRC: c_uint = 0x10044;
// [R 32] Interrupt register #0 read
pub const NIG_REG_NIG_INT_STS_0: c_uint = 0x103b0;
pub const NIG_REG_NIG_INT_STS_1: c_uint = 0x103c0;
// [RC 32] Interrupt register #0 read clear
pub const NIG_REG_NIG_INT_STS_CLR_0: c_uint = 0x103b4;
// [R 32] Legacy E1 and E1H location for parity error mask register.
pub const NIG_REG_NIG_PRTY_MASK: c_uint = 0x103dc;
// [RW 32] Parity mask register #0 read/write
pub const NIG_REG_NIG_PRTY_MASK_0: c_uint = 0x183c8;
pub const NIG_REG_NIG_PRTY_MASK_1: c_uint = 0x183d8;
// [R 32] Legacy E1 and E1H location for parity error status register.
pub const NIG_REG_NIG_PRTY_STS: c_uint = 0x103d0;
// [R 32] Parity register #0 read
pub const NIG_REG_NIG_PRTY_STS_0: c_uint = 0x183bc;
pub const NIG_REG_NIG_PRTY_STS_1: c_uint = 0x183cc;
// [R 32] Legacy E1 and E1H location for parity error status clear register.
pub const NIG_REG_NIG_PRTY_STS_CLR: c_uint = 0x103d4;
// [RC 32] Parity register #0 read clear
pub const NIG_REG_NIG_PRTY_STS_CLR_0: c_uint = 0x183c0;
pub const NIG_REG_NIG_PRTY_STS_CLR_1: c_uint = 0x183d0;

pub const MCPR_IMC_COMMAND_IMC_STATUS_BITSHIFT: c_int = 16;
pub const MCPR_IMC_COMMAND_OPERATION_BITSHIFT: c_int = 28;
pub const MCPR_IMC_COMMAND_TRANSFER_ADDRESS_BITSHIFT: c_int = 8;
// [RW 6] Bit-map indicating which L2 hdrs may appear after the basic
// Ethernet header.
pub const NIG_REG_P0_HDRS_AFTER_BASIC: c_uint = 0x18038;
// [RW 1] HW PFC enable bit. Set this bit to enable the PFC functionality in
// the NIG. Other flow control modes such as PAUSE and SAFC/LLFC should be
// disabled when this bit is set.
pub const NIG_REG_P0_HWPFC_ENABLE: c_uint = 0x18078;
pub const NIG_REG_P0_LLH_FUNC_MEM2: c_uint = 0x18480;
pub const NIG_REG_P0_LLH_FUNC_MEM2_ENABLE: c_uint = 0x18440;
// [RW 17] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. Bits [15:0] return the sequence ID of the packet. Bit 16
// indicates the validity of the data in the buffer. Writing a 1 to bit 16
// will clear the buffer.
//
pub const NIG_REG_P0_LLH_PTP_HOST_BUF_SEQID: c_uint = 0x1875c;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. This location returns the lower 32 bits of timestamp value.
//
pub const NIG_REG_P0_LLH_PTP_HOST_BUF_TS_LSB: c_uint = 0x18754;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. This location returns the upper 32 bits of timestamp value.
//
pub const NIG_REG_P0_LLH_PTP_HOST_BUF_TS_MSB: c_uint = 0x18758;
// [RW 11] Mask register for the various parameters used in determining PTP
// packet presence. Set each bit to 1 to mask out the particular parameter.
// 0-IPv4 DA 0 of 224.0.1.129. 1-IPv4 DA 1 of 224.0.0.107. 2-IPv6 DA 0 of
// 0xFF0*:0:0:0:0:0:0:181. 3-IPv6 DA 1 of 0xFF02:0:0:0:0:0:0:6B. 4-UDP
// destination port 0 of 319. 5-UDP destination port 1 of 320. 6-MAC
// Ethertype 0 of 0x88F7. 7-configurable MAC Ethertype 1. 8-MAC DA 0 of
// 0x01-1B-19-00-00-00. 9-MAC DA 1 of 0x01-80-C2-00-00-0E. 10-configurable
// MAC DA 2. The reset default is set to mask out all parameters.
//
pub const NIG_REG_P0_LLH_PTP_PARAM_MASK: c_uint = 0x187a0;
// [RW 14] Mask register for the rules used in detecting PTP packets. Set
// each bit to 1 to mask out that particular rule. 0-{IPv4 DA 0; UDP DP 0} .
// 1-{IPv4 DA 0; UDP DP 1} . 2-{IPv4 DA 1; UDP DP 0} . 3-{IPv4 DA 1; UDP DP
// 1} . 4-{IPv6 DA 0; UDP DP 0} . 5-{IPv6 DA 0; UDP DP 1} . 6-{IPv6 DA 1;
// UDP DP 0} . 7-{IPv6 DA 1; UDP DP 1} . 8-{MAC DA 0; Ethertype 0} . 9-{MAC
// DA 1; Ethertype 0} . 10-{MAC DA 0; Ethertype 1} . 11-{MAC DA 1; Ethertype
// 1} . 12-{MAC DA 2; Ethertype 0} . 13-{MAC DA 2; Ethertype 1} . The reset
// default is to mask out all of the rules. Note that rules 0-3 are for IPv4
// packets only and require that the packet is IPv4 for the rules to match.
// Note that rules 4-7 are for IPv6 packets only and require that the packet
// is IPv6 for the rules to match.
//
pub const NIG_REG_P0_LLH_PTP_RULE_MASK: c_uint = 0x187a4;
// [RW 1] Set to 1 to enable PTP packets to be forwarded to the host.
pub const NIG_REG_P0_LLH_PTP_TO_HOST: c_uint = 0x187ac;
// [RW 1] Input enable for RX MAC interface.
pub const NIG_REG_P0_MAC_IN_EN: c_uint = 0x185ac;
// [RW 1] Output enable for TX MAC interface
pub const NIG_REG_P0_MAC_OUT_EN: c_uint = 0x185b0;
// [RW 1] Output enable for TX PAUSE signal to the MAC.
pub const NIG_REG_P0_MAC_PAUSE_OUT_EN: c_uint = 0x185b4;
// [RW 32] Eight 4-bit configurations for specifying which COS (0-15 for
// future expansion) each priorty is to be mapped to. Bits 3:0 specify the
// COS for priority 0. Bits 31:28 specify the COS for priority 7. The 3-bit
// priority field is extracted from the outer-most VLAN in receive packet.
// Only COS 0 and COS 1 are supported in E2.
pub const NIG_REG_P0_PKT_PRIORITY_TO_COS: c_uint = 0x18054;
// [RW 6] Enable for TimeSync feature. Bits [2:0] are for RX side. Bits
// [5:3] are for TX side. Bit 0 enables TimeSync on RX side. Bit 1 enables
// V1 frame format in timesync event detection on RX side. Bit 2 enables V2
// frame format in timesync event detection on RX side. Bit 3 enables
// TimeSync on TX side. Bit 4 enables V1 frame format in timesync event
// detection on TX side. Bit 5 enables V2 frame format in timesync event
// detection on TX side. Note that for HW to detect PTP packet and extract
// data from the packet, at least one of the version bits of that traffic
// direction has to be enabled.
//
pub const NIG_REG_P0_PTP_EN: c_uint = 0x18788;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 0. A
// priority is mapped to COS 0 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS0_PRIORITY_MASK: c_uint = 0x18058;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 1. A
// priority is mapped to COS 1 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS1_PRIORITY_MASK: c_uint = 0x1805c;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 2. A
// priority is mapped to COS 2 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS2_PRIORITY_MASK: c_uint = 0x186b0;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 3. A
// priority is mapped to COS 3 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS3_PRIORITY_MASK: c_uint = 0x186b4;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 4. A
// priority is mapped to COS 4 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS4_PRIORITY_MASK: c_uint = 0x186b8;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 5. A
// priority is mapped to COS 5 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P0_RX_COS5_PRIORITY_MASK: c_uint = 0x186bc;
// [R 1] RX FIFO for receiving data from MAC is empty.
// [RW 15] Specify which of the credit registers the client is to be mapped
// to. Bits[2:0] are for client 0; bits [14:12] are for client 4. For
// clients that are not subject to WFQ credit blocking - their
// specifications here are not used.
pub const NIG_REG_P0_TX_ARB_CLIENT_CREDIT_MAP: c_uint = 0x180f0;
// [RW 32] Specify which of the credit registers the client is to be mapped
// to. This register specifies bits 31:0 of the 36-bit value. Bits[3:0] are
// for client 0; bits [35:32] are for client 8. For clients that are not
// subject to WFQ credit blocking - their specifications here are not used.
// This is a new register (with 2_) added in E3 B0 to accommodate the 9
// input clients to ETS arbiter. The reset default is set for management and
// debug to use credit registers 6, 7, and 8, respectively, and COSes 0-5 to
// use credit registers 0-5 respectively (0x543210876). Note that credit
// registers can not be shared between clients.
pub const NIG_REG_P0_TX_ARB_CLIENT_CREDIT_MAP2_LSB: c_uint = 0x18688;
// [RW 4] Specify which of the credit registers the client is to be mapped
// to. This register specifies bits 35:32 of the 36-bit value. Bits[3:0] are
// for client 0; bits [35:32] are for client 8. For clients that are not
// subject to WFQ credit blocking - their specifications here are not used.
// This is a new register (with 2_) added in E3 B0 to accommodate the 9
// input clients to ETS arbiter. The reset default is set for management and
// debug to use credit registers 6, 7, and 8, respectively, and COSes 0-5 to
// use credit registers 0-5 respectively (0x543210876). Note that credit
// registers can not be shared between clients.
pub const NIG_REG_P0_TX_ARB_CLIENT_CREDIT_MAP2_MSB: c_uint = 0x1868c;
// [RW 5] Specify whether the client competes directly in the strict
// priority arbiter. The bits are mapped according to client ID (client IDs
// are defined in tx_arb_priority_client). Default value is set to enable
// strict priorities for clients 0-2 -- management and debug traffic.
pub const NIG_REG_P0_TX_ARB_CLIENT_IS_STRICT: c_uint = 0x180e8;
// [RW 5] Specify whether the client is subject to WFQ credit blocking. The
// bits are mapped according to client ID (client IDs are defined in
// tx_arb_priority_client). Default value is 0 for not using WFQ credit
// blocking.
pub const NIG_REG_P0_TX_ARB_CLIENT_IS_SUBJECT2WFQ: c_uint = 0x180ec;
// [RW 32] Specify the upper bound that credit register 0 is allowed to
// reach.
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_0: c_uint = 0x1810c;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_1: c_uint = 0x18110;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_2: c_uint = 0x18114;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_3: c_uint = 0x18118;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_4: c_uint = 0x1811c;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_5: c_uint = 0x186a0;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_6: c_uint = 0x186a4;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_7: c_uint = 0x186a8;
pub const NIG_REG_P0_TX_ARB_CREDIT_UPPER_BOUND_8: c_uint = 0x186ac;
// [RW 32] Specify the weight (in bytes) to be added to credit register 0
// when it is time to increment.
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_0: c_uint = 0x180f8;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_1: c_uint = 0x180fc;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_2: c_uint = 0x18100;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_3: c_uint = 0x18104;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_4: c_uint = 0x18108;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_5: c_uint = 0x18690;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_6: c_uint = 0x18694;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_7: c_uint = 0x18698;
pub const NIG_REG_P0_TX_ARB_CREDIT_WEIGHT_8: c_uint = 0x1869c;
// [RW 12] Specify the number of strict priority arbitration slots between
// two round-robin arbitration slots to avoid starvation. A value of 0 means
// no strict priority cycles - the strict priority with anti-starvation
// arbiter becomes a round-robin arbiter.
pub const NIG_REG_P0_TX_ARB_NUM_STRICT_ARB_SLOTS: c_uint = 0x180f4;
// [RW 15] Specify the client number to be assigned to each priority of the
// strict priority arbiter. Priority 0 is the highest priority. Bits [2:0]
// are for priority 0 client; bits [14:12] are for priority 4 client. The
// clients are assigned the following IDs: 0-management; 1-debug traffic
// from this port; 2-debug traffic from other port; 3-COS0 traffic; 4-COS1
// traffic. The reset value[14:0] is set to 0x4688 (15'b100_011_010_001_000)
// for management at priority 0; debug traffic at priorities 1 and 2; COS0
// traffic at priority 3; and COS1 traffic at priority 4.
pub const NIG_REG_P0_TX_ARB_PRIORITY_CLIENT: c_uint = 0x180e4;
// [RW 6] Bit-map indicating which L2 hdrs may appear after the basic
// Ethernet header.
pub const NIG_REG_P1_HDRS_AFTER_BASIC: c_uint = 0x1818c;
pub const NIG_REG_P1_LLH_FUNC_MEM2: c_uint = 0x184c0;
pub const NIG_REG_P1_LLH_FUNC_MEM2_ENABLE: c_uint = 0x18460a;
// [RW 17] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. Bits [15:0] return the sequence ID of the packet. Bit 16
// indicates the validity of the data in the buffer. Writing a 1 to bit 16
// will clear the buffer.
//
pub const NIG_REG_P1_LLH_PTP_HOST_BUF_SEQID: c_uint = 0x18774;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. This location returns the lower 32 bits of timestamp value.
//
pub const NIG_REG_P1_LLH_PTP_HOST_BUF_TS_LSB: c_uint = 0x1876c;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// the host. This location returns the upper 32 bits of timestamp value.
//
pub const NIG_REG_P1_LLH_PTP_HOST_BUF_TS_MSB: c_uint = 0x18770;
// [RW 11] Mask register for the various parameters used in determining PTP
// packet presence. Set each bit to 1 to mask out the particular parameter.
// 0-IPv4 DA 0 of 224.0.1.129. 1-IPv4 DA 1 of 224.0.0.107. 2-IPv6 DA 0 of
// 0xFF0*:0:0:0:0:0:0:181. 3-IPv6 DA 1 of 0xFF02:0:0:0:0:0:0:6B. 4-UDP
// destination port 0 of 319. 5-UDP destination port 1 of 320. 6-MAC
// Ethertype 0 of 0x88F7. 7-configurable MAC Ethertype 1. 8-MAC DA 0 of
// 0x01-1B-19-00-00-00. 9-MAC DA 1 of 0x01-80-C2-00-00-0E. 10-configurable
// MAC DA 2. The reset default is set to mask out all parameters.
//
pub const NIG_REG_P1_LLH_PTP_PARAM_MASK: c_uint = 0x187c8;
// [RW 14] Mask register for the rules used in detecting PTP packets. Set
// each bit to 1 to mask out that particular rule. 0-{IPv4 DA 0; UDP DP 0} .
// 1-{IPv4 DA 0; UDP DP 1} . 2-{IPv4 DA 1; UDP DP 0} . 3-{IPv4 DA 1; UDP DP
// 1} . 4-{IPv6 DA 0; UDP DP 0} . 5-{IPv6 DA 0; UDP DP 1} . 6-{IPv6 DA 1;
// UDP DP 0} . 7-{IPv6 DA 1; UDP DP 1} . 8-{MAC DA 0; Ethertype 0} . 9-{MAC
// DA 1; Ethertype 0} . 10-{MAC DA 0; Ethertype 1} . 11-{MAC DA 1; Ethertype
// 1} . 12-{MAC DA 2; Ethertype 0} . 13-{MAC DA 2; Ethertype 1} . The reset
// default is to mask out all of the rules. Note that rules 0-3 are for IPv4
// packets only and require that the packet is IPv4 for the rules to match.
// Note that rules 4-7 are for IPv6 packets only and require that the packet
// is IPv6 for the rules to match.
//
pub const NIG_REG_P1_LLH_PTP_RULE_MASK: c_uint = 0x187cc;
// [RW 1] Set to 1 to enable PTP packets to be forwarded to the host.
pub const NIG_REG_P1_LLH_PTP_TO_HOST: c_uint = 0x187d4;
// [RW 32] Specify the client number to be assigned to each priority of the
// strict priority arbiter. This register specifies bits 31:0 of the 36-bit
// value. Priority 0 is the highest priority. Bits [3:0] are for priority 0
// client; bits [35-32] are for priority 8 client. The clients are assigned
// the following IDs: 0-management; 1-debug traffic from this port; 2-debug
// traffic from other port; 3-COS0 traffic; 4-COS1 traffic; 5-COS2 traffic;
// 6-COS3 traffic; 7-COS4 traffic; 8-COS5 traffic. The reset value[35:0] is
// set to 0x345678021. This is a new register (with 2_) added in E3 B0 to
// accommodate the 9 input clients to ETS arbiter.
pub const NIG_REG_P0_TX_ARB_PRIORITY_CLIENT2_LSB: c_uint = 0x18680;
// [RW 4] Specify the client number to be assigned to each priority of the
// strict priority arbiter. This register specifies bits 35:32 of the 36-bit
// value. Priority 0 is the highest priority. Bits [3:0] are for priority 0
// client; bits [35-32] are for priority 8 client. The clients are assigned
// the following IDs: 0-management; 1-debug traffic from this port; 2-debug
// traffic from other port; 3-COS0 traffic; 4-COS1 traffic; 5-COS2 traffic;
// 6-COS3 traffic; 7-COS4 traffic; 8-COS5 traffic. The reset value[35:0] is
// set to 0x345678021. This is a new register (with 2_) added in E3 B0 to
// accommodate the 9 input clients to ETS arbiter.
pub const NIG_REG_P0_TX_ARB_PRIORITY_CLIENT2_MSB: c_uint = 0x18684;
// [RW 1] MCP-to-host path enable. Set this bit to enable the routing of MCP
// packets to BRB LB interface to forward the packet to the host. All
// packets from MCP are forwarded to the network when this bit is cleared -
// regardless of the configured destination in tx_mng_destination register.
// When MCP-to-host paths for both ports 0 and 1 are disabled - the arbiter
// for BRB LB interface is bypassed and PBF LB traffic is always selected to
// send to BRB LB.
//
pub const NIG_REG_P0_TX_MNG_HOST_ENABLE: c_uint = 0x182f4;
pub const NIG_REG_P1_HWPFC_ENABLE: c_uint = 0x181d0;
pub const NIG_REG_P1_MAC_IN_EN: c_uint = 0x185c0;
// [RW 1] Output enable for TX MAC interface
pub const NIG_REG_P1_MAC_OUT_EN: c_uint = 0x185c4;
// [RW 1] Output enable for TX PAUSE signal to the MAC.
pub const NIG_REG_P1_MAC_PAUSE_OUT_EN: c_uint = 0x185c8;
// [RW 32] Eight 4-bit configurations for specifying which COS (0-15 for
// future expansion) each priorty is to be mapped to. Bits 3:0 specify the
// COS for priority 0. Bits 31:28 specify the COS for priority 7. The 3-bit
// priority field is extracted from the outer-most VLAN in receive packet.
// Only COS 0 and COS 1 are supported in E2.
pub const NIG_REG_P1_PKT_PRIORITY_TO_COS: c_uint = 0x181a8;
// [RW 6] Enable for TimeSync feature. Bits [2:0] are for RX side. Bits
// [5:3] are for TX side. Bit 0 enables TimeSync on RX side. Bit 1 enables
// V1 frame format in timesync event detection on RX side. Bit 2 enables V2
// frame format in timesync event detection on RX side. Bit 3 enables
// TimeSync on TX side. Bit 4 enables V1 frame format in timesync event
// detection on TX side. Bit 5 enables V2 frame format in timesync event
// detection on TX side. Note that for HW to detect PTP packet and extract
// data from the packet, at least one of the version bits of that traffic
// direction has to be enabled.
//
pub const NIG_REG_P1_PTP_EN: c_uint = 0x187b0;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 0. A
// priority is mapped to COS 0 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P1_RX_COS0_PRIORITY_MASK: c_uint = 0x181ac;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 1. A
// priority is mapped to COS 1 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P1_RX_COS1_PRIORITY_MASK: c_uint = 0x181b0;
// [RW 16] Bit-map indicating which SAFC/PFC priorities to map to COS 2. A
// priority is mapped to COS 2 when the corresponding mask bit is 1. More
// than one bit may be set; allowing multiple priorities to be mapped to one
// COS.
pub const NIG_REG_P1_RX_COS2_PRIORITY_MASK: c_uint = 0x186f8;
// [R 1] RX FIFO for receiving data from MAC is empty.
pub const NIG_REG_P1_RX_MACFIFO_EMPTY: c_uint = 0x1858c;
// [R 1] TLLH FIFO is empty.
pub const NIG_REG_P1_TLLH_FIFO_EMPTY: c_uint = 0x18338;
// [RW 19] Packet TimeSync information that is buffered in 1-deep FIFOs for
// TX side. Bits [15:0] reflect the sequence ID of the packet. Bit 16
// indicates the validity of the data in the buffer. Bit 17 indicates that
// the sequence ID is valid and it is waiting for the TX timestamp value.
// Bit 18 indicates whether the timestamp is from a SW request (value of 1)
// or HW request (value of 0). Writing a 1 to bit 16 will clear the buffer.
//
pub const NIG_REG_P0_TLLH_PTP_BUF_SEQID: c_uint = 0x187e0;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// MCP. This location returns the lower 32 bits of timestamp value.
//
pub const NIG_REG_P0_TLLH_PTP_BUF_TS_LSB: c_uint = 0x187d8;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// MCP. This location returns the upper 32 bits of timestamp value.
//
pub const NIG_REG_P0_TLLH_PTP_BUF_TS_MSB: c_uint = 0x187dc;
// [RW 11] Mask register for the various parameters used in determining PTP
// packet presence. Set each bit to 1 to mask out the particular parameter.
// 0-IPv4 DA 0 of 224.0.1.129. 1-IPv4 DA 1 of 224.0.0.107. 2-IPv6 DA 0 of
// 0xFF0*:0:0:0:0:0:0:181. 3-IPv6 DA 1 of 0xFF02:0:0:0:0:0:0:6B. 4-UDP
// destination port 0 of 319. 5-UDP destination port 1 of 320. 6-MAC
// Ethertype 0 of 0x88F7. 7-configurable MAC Ethertype 1. 8-MAC DA 0 of
// 0x01-1B-19-00-00-00. 9-MAC DA 1 of 0x01-80-C2-00-00-0E. 10-configurable
// MAC DA 2. The reset default is set to mask out all parameters.
//
pub const NIG_REG_P0_TLLH_PTP_PARAM_MASK: c_uint = 0x187f0;
// [RW 14] Mask register for the rules used in detecting PTP packets. Set
// each bit to 1 to mask out that particular rule. 0-{IPv4 DA 0; UDP DP 0} .
// 1-{IPv4 DA 0; UDP DP 1} . 2-{IPv4 DA 1; UDP DP 0} . 3-{IPv4 DA 1; UDP DP
// 1} . 4-{IPv6 DA 0; UDP DP 0} . 5-{IPv6 DA 0; UDP DP 1} . 6-{IPv6 DA 1;
// UDP DP 0} . 7-{IPv6 DA 1; UDP DP 1} . 8-{MAC DA 0; Ethertype 0} . 9-{MAC
// DA 1; Ethertype 0} . 10-{MAC DA 0; Ethertype 1} . 11-{MAC DA 1; Ethertype
// 1} . 12-{MAC DA 2; Ethertype 0} . 13-{MAC DA 2; Ethertype 1} . The reset
// default is to mask out all of the rules.
//
pub const NIG_REG_P0_TLLH_PTP_RULE_MASK: c_uint = 0x187f4;
// [RW 19] Packet TimeSync information that is buffered in 1-deep FIFOs for
// TX side. Bits [15:0] reflect the sequence ID of the packet. Bit 16
// indicates the validity of the data in the buffer. Bit 17 indicates that
// the sequence ID is valid and it is waiting for the TX timestamp value.
// Bit 18 indicates whether the timestamp is from a SW request (value of 1)
// or HW request (value of 0). Writing a 1 to bit 16 will clear the buffer.
//
pub const NIG_REG_P1_TLLH_PTP_BUF_SEQID: c_uint = 0x187ec;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// MCP. This location returns the lower 32 bits of timestamp value.
//
pub const NIG_REG_P1_TLLH_PTP_BUF_TS_LSB: c_uint = 0x187e4;
// [R 32] Packet TimeSync information that is buffered in 1-deep FIFOs for
// MCP. This location returns the upper 32 bits of timestamp value.
//
pub const NIG_REG_P1_TLLH_PTP_BUF_TS_MSB: c_uint = 0x187e8;
// [RW 11] Mask register for the various parameters used in determining PTP
// packet presence. Set each bit to 1 to mask out the particular parameter.
// 0-IPv4 DA 0 of 224.0.1.129. 1-IPv4 DA 1 of 224.0.0.107. 2-IPv6 DA 0 of
// 0xFF0*:0:0:0:0:0:0:181. 3-IPv6 DA 1 of 0xFF02:0:0:0:0:0:0:6B. 4-UDP
// destination port 0 of 319. 5-UDP destination port 1 of 320. 6-MAC
// Ethertype 0 of 0x88F7. 7-configurable MAC Ethertype 1. 8-MAC DA 0 of
// 0x01-1B-19-00-00-00. 9-MAC DA 1 of 0x01-80-C2-00-00-0E. 10-configurable
// MAC DA 2. The reset default is set to mask out all parameters.
//
pub const NIG_REG_P1_TLLH_PTP_PARAM_MASK: c_uint = 0x187f8;
// [RW 14] Mask register for the rules used in detecting PTP packets. Set
// each bit to 1 to mask out that particular rule. 0-{IPv4 DA 0; UDP DP 0} .
// 1-{IPv4 DA 0; UDP DP 1} . 2-{IPv4 DA 1; UDP DP 0} . 3-{IPv4 DA 1; UDP DP
// 1} . 4-{IPv6 DA 0; UDP DP 0} . 5-{IPv6 DA 0; UDP DP 1} . 6-{IPv6 DA 1;
// UDP DP 0} . 7-{IPv6 DA 1; UDP DP 1} . 8-{MAC DA 0; Ethertype 0} . 9-{MAC
// DA 1; Ethertype 0} . 10-{MAC DA 0; Ethertype 1} . 11-{MAC DA 1; Ethertype
// 1} . 12-{MAC DA 2; Ethertype 0} . 13-{MAC DA 2; Ethertype 1} . The reset
// default is to mask out all of the rules.
//
pub const NIG_REG_P1_TLLH_PTP_RULE_MASK: c_uint = 0x187fc;
// [RW 32] Specify which of the credit registers the client is to be mapped
// to. This register specifies bits 31:0 of the 36-bit value. Bits[3:0] are
// for client 0; bits [35:32] are for client 8. For clients that are not
// subject to WFQ credit blocking - their specifications here are not used.
// This is a new register (with 2_) added in E3 B0 to accommodate the 9
// input clients to ETS arbiter. The reset default is set for management and
// debug to use credit registers 6, 7, and 8, respectively, and COSes 0-5 to
// use credit registers 0-5 respectively (0x543210876). Note that credit
// registers can not be shared between clients. Note also that there are
// only COS0-2 in port 1- there is a total of 6 clients in port 1. Only
// credit registers 0-5 are valid. This register should be configured
// appropriately before enabling WFQ.
pub const NIG_REG_P1_TX_ARB_CLIENT_CREDIT_MAP2_LSB: c_uint = 0x186e8;
// [RW 4] Specify which of the credit registers the client is to be mapped
// to. This register specifies bits 35:32 of the 36-bit value. Bits[3:0] are
// for client 0; bits [35:32] are for client 8. For clients that are not
// subject to WFQ credit blocking - their specifications here are not used.
// This is a new register (with 2_) added in E3 B0 to accommodate the 9
// input clients to ETS arbiter. The reset default is set for management and
// debug to use credit registers 6, 7, and 8, respectively, and COSes 0-5 to
// use credit registers 0-5 respectively (0x543210876). Note that credit
// registers can not be shared between clients. Note also that there are
// only COS0-2 in port 1- there is a total of 6 clients in port 1. Only
// credit registers 0-5 are valid. This register should be configured
// appropriately before enabling WFQ.
pub const NIG_REG_P1_TX_ARB_CLIENT_CREDIT_MAP2_MSB: c_uint = 0x186ec;
// [RW 9] Specify whether the client competes directly in the strict
// priority arbiter. The bits are mapped according to client ID (client IDs
// are defined in tx_arb_priority_client2): 0-management; 1-debug traffic
// from this port; 2-debug traffic from other port; 3-COS0 traffic; 4-COS1
// traffic; 5-COS2 traffic; 6-COS3 traffic; 7-COS4 traffic; 8-COS5 traffic.
// Default value is set to enable strict priorities for all clients.
pub const NIG_REG_P1_TX_ARB_CLIENT_IS_STRICT: c_uint = 0x18234;
// [RW 9] Specify whether the client is subject to WFQ credit blocking. The
// bits are mapped according to client ID (client IDs are defined in
// tx_arb_priority_client2): 0-management; 1-debug traffic from this port;
// 2-debug traffic from other port; 3-COS0 traffic; 4-COS1 traffic; 5-COS2
// traffic; 6-COS3 traffic; 7-COS4 traffic; 8-COS5 traffic. Default value is
// 0 for not using WFQ credit blocking.
pub const NIG_REG_P1_TX_ARB_CLIENT_IS_SUBJECT2WFQ: c_uint = 0x18238;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_0: c_uint = 0x18258;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_1: c_uint = 0x1825c;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_2: c_uint = 0x18260;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_3: c_uint = 0x18264;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_4: c_uint = 0x18268;
pub const NIG_REG_P1_TX_ARB_CREDIT_UPPER_BOUND_5: c_uint = 0x186f4;
// [RW 32] Specify the weight (in bytes) to be added to credit register 0
// when it is time to increment.
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_0: c_uint = 0x18244;
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_1: c_uint = 0x18248;
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_2: c_uint = 0x1824c;
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_3: c_uint = 0x18250;
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_4: c_uint = 0x18254;
pub const NIG_REG_P1_TX_ARB_CREDIT_WEIGHT_5: c_uint = 0x186f0;
// [RW 12] Specify the number of strict priority arbitration slots between
pub const NIG_REG_P1_TX_ARB_NUM_STRICT_ARB_SLOTS: c_uint = 0x18240;
// [RW 32] Specify the client number to be assigned to each priority of the
pub const NIG_REG_P1_TX_ARB_PRIORITY_CLIENT2_LSB: c_uint = 0x186e0;
// [RW 4] Specify the client number to be assigned to each priority of the
pub const NIG_REG_P1_TX_ARB_PRIORITY_CLIENT2_MSB: c_uint = 0x186e4;
// [R 1] TX FIFO for transmitting data to MAC is empty.
pub const NIG_REG_P1_TX_MACFIFO_EMPTY: c_uint = 0x18594;
// [RW 1] MCP-to-host path enable. Set this bit to enable the routing of MCP
// packets to BRB LB interface to forward the packet to the host. All
// packets from MCP are forwarded to the network when this bit is cleared -
// regardless of the configured destination in tx_mng_destination register.
//
pub const NIG_REG_P1_TX_MNG_HOST_ENABLE: c_uint = 0x182f8;
// [R 1] FIFO empty status of the MCP TX FIFO used for storing MCP packets
pub const NIG_REG_P1_TX_MNG_HOST_FIFO_EMPTY: c_uint = 0x182b8;
// [RW 32] Specify the upper bound that credit register 0 is allowed to
// reach.
// [RW 1] Pause enable for port0. This register may get 1 only when
pub const NIG_REG_PAUSE_ENABLE_0: c_uint = 0x160c0;
pub const NIG_REG_PAUSE_ENABLE_1: c_uint = 0x160c4;
// [RW 1] Input enable for RX PBF LP IF
pub const NIG_REG_PBF_LB_IN_EN: c_uint = 0x100b4;
// [RW 1] Value of this register will be transmitted to port swap when
pub const NIG_REG_PORT_SWAP: c_uint = 0x10394;
// [RW 1] PPP enable for port0. This register may get 1 only when
// ~safc_enable.safc_enable = 0 and pause_enable.pause_enable =0 for the
// same port
pub const NIG_REG_PPP_ENABLE_0: c_uint = 0x160b0;
pub const NIG_REG_PPP_ENABLE_1: c_uint = 0x160b4;
// [RW 1] output enable for RX parser descriptor IF
pub const NIG_REG_PRS_EOP_OUT_EN: c_uint = 0x10104;
// [RW 1] Input enable for RX parser request IF
pub const NIG_REG_PRS_REQ_IN_EN: c_uint = 0x100b8;
// [RW 5] control to serdes - CL45 DEVAD
pub const NIG_REG_SERDES0_CTRL_MD_DEVAD: c_uint = 0x10370;
// [RW 1] control to serdes; 0 - clause 45; 1 - clause 22
pub const NIG_REG_SERDES0_CTRL_MD_ST: c_uint = 0x1036c;
// [RW 5] control to serdes - CL22 PHY_ADD and CL45 PRTAD
pub const NIG_REG_SERDES0_CTRL_PHY_ADDR: c_uint = 0x10374;
// [R 1] status from serdes0 that inputs to interrupt logic of link status
pub const NIG_REG_SERDES0_STATUS_LINK_STATUS: c_uint = 0x10578;
// [R 32] Rx statistics : In user packets discarded due to BRB backpressure
pub const NIG_REG_STAT0_BRB_DISCARD: c_uint = 0x105f0;
// [R 32] Rx statistics : In user packets truncated due to BRB backpressure
pub const NIG_REG_STAT0_BRB_TRUNCATE: c_uint = 0x105f8;
// [WB_R 36] Tx statistics : Number of packets from emac0 or bmac0 that
pub const NIG_REG_STAT0_EGRESS_MAC_PKT0: c_uint = 0x10750;
// [WB_R 36] Tx statistics : Number of packets from emac0 or bmac0 that
pub const NIG_REG_STAT0_EGRESS_MAC_PKT1: c_uint = 0x10760;
// [R 32] Rx statistics : In user packets discarded due to BRB backpressure
pub const NIG_REG_STAT1_BRB_DISCARD: c_uint = 0x10628;
// [WB_R 36] Tx statistics : Number of packets from emac1 or bmac1 that
pub const NIG_REG_STAT1_EGRESS_MAC_PKT0: c_uint = 0x107a0;
// [WB_R 36] Tx statistics : Number of packets from emac1 or bmac1 that
pub const NIG_REG_STAT1_EGRESS_MAC_PKT1: c_uint = 0x107b0;
// [WB_R 64] Rx statistics : User octets received for LP
pub const NIG_REG_STAT2_BRB_OCTET: c_uint = 0x107e0;
pub const NIG_REG_STATUS_INTERRUPT_PORT0: c_uint = 0x10328;
pub const NIG_REG_STATUS_INTERRUPT_PORT1: c_uint = 0x1032c;
// [RW 1] port swap mux selection. If this register equal to 0 then port
pub const NIG_REG_STRAP_OVERRIDE: c_uint = 0x10398;
// [WB 64] Addresses for TimeSync related registers in the timesync
// generator sub-module.
//
pub const NIG_REG_TIMESYNC_GEN_REG: c_uint = 0x18800;
// [RW 1] output enable for RX_XCM0 IF
pub const NIG_REG_XCM0_OUT_EN: c_uint = 0x100f0;
// [RW 1] output enable for RX_XCM1 IF
pub const NIG_REG_XCM1_OUT_EN: c_uint = 0x100f4;
// [RW 1] control to xgxs - remote PHY in-band MDIO
pub const NIG_REG_XGXS0_CTRL_EXTREMOTEMDIOST: c_uint = 0x10348;
// [RW 5] control to xgxs - CL45 DEVAD
pub const NIG_REG_XGXS0_CTRL_MD_DEVAD: c_uint = 0x1033c;
// [RW 1] control to xgxs; 0 - clause 45; 1 - clause 22
pub const NIG_REG_XGXS0_CTRL_MD_ST: c_uint = 0x10338;
// [RW 5] control to xgxs - CL22 PHY_ADD and CL45 PRTAD
pub const NIG_REG_XGXS0_CTRL_PHY_ADDR: c_uint = 0x10340;
// [R 1] status from xgxs0 that inputs to interrupt logic of link10g.
pub const NIG_REG_XGXS0_STATUS_LINK10G: c_uint = 0x10680;
// [R 4] status from xgxs0 that inputs to interrupt logic of link status
pub const NIG_REG_XGXS0_STATUS_LINK_STATUS: c_uint = 0x10684;
// [RW 2] selection for XGXS lane of port 0 in NIG_MUX block
pub const NIG_REG_XGXS_LANE_SEL_P0: c_uint = 0x102e8;
// [RW 1] selection for port0 for NIG_MUX block : 0 = SerDes; 1 = XGXS
pub const NIG_REG_XGXS_SERDES0_MODE_SEL: c_uint = 0x102e0;

pub const NIG_STATUS_INTERRUPT_PORT0_REG_STATUS_XGXS0_LINK_STATUS_SIZE: c_int = 18;
// [RW 31] The upper bound of the weight of COS0 in the ETS command arbiter.
pub const PBF_REG_COS0_UPPER_BOUND: c_uint = 0x15c05c;
// [RW 31] The upper bound of the weight of COS0 in the ETS command arbiter
// of port 0.
pub const PBF_REG_COS0_UPPER_BOUND_P0: c_uint = 0x15c2cc;
// [RW 31] The upper bound of the weight of COS0 in the ETS command arbiter
// of port 1.
pub const PBF_REG_COS0_UPPER_BOUND_P1: c_uint = 0x15c2e4;
// [RW 31] The weight of COS0 in the ETS command arbiter.
pub const PBF_REG_COS0_WEIGHT: c_uint = 0x15c054;
// [RW 31] The weight of COS0 in port 0 ETS command arbiter.
pub const PBF_REG_COS0_WEIGHT_P0: c_uint = 0x15c2a8;
// [RW 31] The weight of COS0 in port 1 ETS command arbiter.
pub const PBF_REG_COS0_WEIGHT_P1: c_uint = 0x15c2c0;
// [RW 31] The upper bound of the weight of COS1 in the ETS command arbiter.
pub const PBF_REG_COS1_UPPER_BOUND: c_uint = 0x15c060;
// [RW 31] The weight of COS1 in the ETS command arbiter.
pub const PBF_REG_COS1_WEIGHT: c_uint = 0x15c058;
// [RW 31] The weight of COS1 in port 0 ETS command arbiter.
pub const PBF_REG_COS1_WEIGHT_P0: c_uint = 0x15c2ac;
// [RW 31] The weight of COS1 in port 1 ETS command arbiter.
pub const PBF_REG_COS1_WEIGHT_P1: c_uint = 0x15c2c4;
// [RW 31] The weight of COS2 in port 0 ETS command arbiter.
pub const PBF_REG_COS2_WEIGHT_P0: c_uint = 0x15c2b0;
// [RW 31] The weight of COS2 in port 1 ETS command arbiter.
pub const PBF_REG_COS2_WEIGHT_P1: c_uint = 0x15c2c8;
// [RW 31] The weight of COS3 in port 0 ETS command arbiter.
pub const PBF_REG_COS3_WEIGHT_P0: c_uint = 0x15c2b4;
// [RW 31] The weight of COS4 in port 0 ETS command arbiter.
pub const PBF_REG_COS4_WEIGHT_P0: c_uint = 0x15c2b8;
// [RW 31] The weight of COS5 in port 0 ETS command arbiter.
pub const PBF_REG_COS5_WEIGHT_P0: c_uint = 0x15c2bc;
// [R 11] Current credit for the LB queue in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_CREDIT_LB_Q: c_uint = 0x140338;
// [R 11] Current credit for queue 0 in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_CREDIT_Q0: c_uint = 0x14033c;
// [R 11] Current credit for queue 1 in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_CREDIT_Q1: c_uint = 0x140340;
// [RW 1] Disable processing further tasks from port 0 (after ending the
pub const PBF_REG_DISABLE_NEW_TASK_PROC_P0: c_uint = 0x14005c;
// [RW 1] Disable processing further tasks from port 1 (after ending the
pub const PBF_REG_DISABLE_NEW_TASK_PROC_P1: c_uint = 0x140060;
// [RW 1] Disable processing further tasks from port 4 (after ending the
pub const PBF_REG_DISABLE_NEW_TASK_PROC_P4: c_uint = 0x14006c;
pub const PBF_REG_DISABLE_PF: c_uint = 0x1402e8;
pub const PBF_REG_DISABLE_VF: c_uint = 0x1402ec;
// [RW 18] For port 0: For each client that is subject to WFQ (the
// corresponding bit is 1); indicates to which of the credit registers this
// client is mapped. For clients which are not credit blocked; their mapping
// is dont care.
pub const PBF_REG_ETS_ARB_CLIENT_CREDIT_MAP_P0: c_uint = 0x15c288;
// [RW 9] For port 1: For each client that is subject to WFQ (the
// corresponding bit is 1); indicates to which of the credit registers this
// client is mapped. For clients which are not credit blocked; their mapping
// is dont care.
pub const PBF_REG_ETS_ARB_CLIENT_CREDIT_MAP_P1: c_uint = 0x15c28c;
// [RW 6] For port 0: Bit per client to indicate if the client competes in
// the strict priority arbiter directly (corresponding bit = 1); or first
// goes to the RR arbiter (corresponding bit = 0); and then competes in the
// lowest priority in the strict-priority arbiter.
pub const PBF_REG_ETS_ARB_CLIENT_IS_STRICT_P0: c_uint = 0x15c278;
// [RW 3] For port 1: Bit per client to indicate if the client competes in
// the strict priority arbiter directly (corresponding bit = 1); or first
// goes to the RR arbiter (corresponding bit = 0); and then competes in the
// lowest priority in the strict-priority arbiter.
pub const PBF_REG_ETS_ARB_CLIENT_IS_STRICT_P1: c_uint = 0x15c27c;
// [RW 6] For port 0: Bit per client to indicate if the client is subject to
// WFQ credit blocking (corresponding bit = 1).
pub const PBF_REG_ETS_ARB_CLIENT_IS_SUBJECT2WFQ_P0: c_uint = 0x15c280;
// [RW 3] For port 0: Bit per client to indicate if the client is subject to
// WFQ credit blocking (corresponding bit = 1).
pub const PBF_REG_ETS_ARB_CLIENT_IS_SUBJECT2WFQ_P1: c_uint = 0x15c284;
// [RW 16] For port 0: The number of strict priority arbitration slots
// between 2 RR arbitration slots. A value of 0 means no strict priority
// cycles; i.e. the strict-priority w/ anti-starvation arbiter is a RR
// arbiter.
pub const PBF_REG_ETS_ARB_NUM_STRICT_ARB_SLOTS_P0: c_uint = 0x15c2a0;
// [RW 16] For port 1: The number of strict priority arbitration slots
// between 2 RR arbitration slots. A value of 0 means no strict priority
// cycles; i.e. the strict-priority w/ anti-starvation arbiter is a RR
// arbiter.
pub const PBF_REG_ETS_ARB_NUM_STRICT_ARB_SLOTS_P1: c_uint = 0x15c2a4;
// [RW 18] For port 0: Indicates which client is connected to each priority
// in the strict-priority arbiter. Priority 0 is the highest priority, and
// priority 5 is the lowest; to which the RR output is connected to (this is
// not configurable).
pub const PBF_REG_ETS_ARB_PRIORITY_CLIENT_P0: c_uint = 0x15c270;
// [RW 9] For port 1: Indicates which client is connected to each priority
// in the strict-priority arbiter. Priority 0 is the highest priority, and
// priority 5 is the lowest; to which the RR output is connected to (this is
// not configurable).
pub const PBF_REG_ETS_ARB_PRIORITY_CLIENT_P1: c_uint = 0x15c274;
// [RW 1] Indicates that ETS is performed between the COSes in the command
// arbiter. If reset strict priority w/ anti-starvation will be performed
// w/o WFQ.
pub const PBF_REG_ETS_ENABLED: c_uint = 0x15c050;
// [RW 6] Bit-map indicating which L2 hdrs may appear after the basic
// Ethernet header.
pub const PBF_REG_HDRS_AFTER_BASIC: c_uint = 0x15c0a8;
// [RW 6] Bit-map indicating which L2 hdrs may appear after L2 tag 0
pub const PBF_REG_HDRS_AFTER_TAG_0: c_uint = 0x15c0b8;
// [R 1] Removed for E3 B0 - Indicates which COS is conncted to the highest
// priority in the command arbiter.
pub const PBF_REG_HIGH_PRIORITY_COS_NUM: c_uint = 0x15c04c;
pub const PBF_REG_IF_ENABLE_REG: c_uint = 0x140044;
// [RW 1] Init bit. When set the initial credits are copied to the credit
pub const PBF_REG_INIT: c_uint = 0x140000;
// [RW 11] Initial credit for the LB queue in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_INIT_CRD_LB_Q: c_uint = 0x15c248;
// [RW 11] Initial credit for queue 0 in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_INIT_CRD_Q0: c_uint = 0x15c230;
// [RW 11] Initial credit for queue 1 in the tx port buffers in 16 byte
// lines.
pub const PBF_REG_INIT_CRD_Q1: c_uint = 0x15c234;
// [RW 1] Init bit for port 0. When set the initial credit of port 0 is
pub const PBF_REG_INIT_P0: c_uint = 0x140004;
// [RW 1] Init bit for port 1. When set the initial credit of port 1 is
pub const PBF_REG_INIT_P1: c_uint = 0x140008;
// [RW 1] Init bit for port 4. When set the initial credit of port 4 is
pub const PBF_REG_INIT_P4: c_uint = 0x14000c;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// the LB queue. Reset upon init.
pub const PBF_REG_INTERNAL_CRD_FREED_CNT_LB_Q: c_uint = 0x140354;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// queue 0. Reset upon init.
pub const PBF_REG_INTERNAL_CRD_FREED_CNT_Q0: c_uint = 0x140358;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// queue 1. Reset upon init.
pub const PBF_REG_INTERNAL_CRD_FREED_CNT_Q1: c_uint = 0x14035c;
// [RW 1] Enable for mac interface 0.
pub const PBF_REG_MAC_IF0_ENABLE: c_uint = 0x140030;
// [RW 1] Enable for mac interface 1.
pub const PBF_REG_MAC_IF1_ENABLE: c_uint = 0x140034;
// [RW 1] Enable for the loopback interface.
pub const PBF_REG_MAC_LB_ENABLE: c_uint = 0x140040;
// [RW 6] Bit-map indicating which headers must appear in the packet
pub const PBF_REG_MUST_HAVE_HDRS: c_uint = 0x15c0c4;
// [RW 16] The number of strict priority arbitration slots between 2 RR
// arbitration slots. A value of 0 means no strict priority cycles; i.e. the
// strict-priority w/ anti-starvation arbiter is a RR arbiter.
pub const PBF_REG_NUM_STRICT_ARB_SLOTS: c_uint = 0x15c064;
// [RW 10] Port 0 threshold used by arbiter in 16 byte lines used when pause
pub const PBF_REG_P0_ARB_THRSH: c_uint = 0x1400e4;
// [R 11] Current credit for port 0 in the tx port buffers in 16 byte lines.
pub const PBF_REG_P0_CREDIT: c_uint = 0x140200;
// [RW 11] Initial credit for port 0 in the tx port buffers in 16 byte
pub const PBF_REG_P0_INIT_CRD: c_uint = 0x1400d0;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// port 0. Reset upon init.
pub const PBF_REG_P0_INTERNAL_CRD_FREED_CNT: c_uint = 0x140308;
// [R 1] Removed for E3 B0 - Indication that pause is enabled for port 0.
pub const PBF_REG_P0_PAUSE_ENABLE: c_uint = 0x140014;
// [R 8] Removed for E3 B0 - Number of tasks in port 0 task queue.
pub const PBF_REG_P0_TASK_CNT: c_uint = 0x140204;
// [R 32] Removed for E3 B0 - Cyclic counter for number of 8 byte lines
// freed from the task queue of port 0. Reset upon init.
pub const PBF_REG_P0_TQ_LINES_FREED_CNT: c_uint = 0x1402f0;
// [R 12] Number of 8 bytes lines occupied in the task queue of port 0.
pub const PBF_REG_P0_TQ_OCCUPANCY: c_uint = 0x1402fc;
// [R 11] Removed for E3 B0 - Current credit for port 1 in the tx port
// buffers in 16 byte lines.
pub const PBF_REG_P1_CREDIT: c_uint = 0x140208;
// [R 11] Removed for E3 B0 - Initial credit for port 0 in the tx port
// buffers in 16 byte lines.
pub const PBF_REG_P1_INIT_CRD: c_uint = 0x1400d4;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// port 1. Reset upon init.
pub const PBF_REG_P1_INTERNAL_CRD_FREED_CNT: c_uint = 0x14030c;
// [R 8] Removed for E3 B0 - Number of tasks in port 1 task queue.
pub const PBF_REG_P1_TASK_CNT: c_uint = 0x14020c;
// [R 32] Removed for E3 B0 - Cyclic counter for number of 8 byte lines
// freed from the task queue of port 1. Reset upon init.
pub const PBF_REG_P1_TQ_LINES_FREED_CNT: c_uint = 0x1402f4;
// [R 12] Number of 8 bytes lines occupied in the task queue of port 1.
pub const PBF_REG_P1_TQ_OCCUPANCY: c_uint = 0x140300;
// [R 11] Current credit for port 4 in the tx port buffers in 16 byte lines.
pub const PBF_REG_P4_CREDIT: c_uint = 0x140210;
// [RW 11] Initial credit for port 4 in the tx port buffers in 16 byte
pub const PBF_REG_P4_INIT_CRD: c_uint = 0x1400e0;
// [R 32] Cyclic counter for the amount credits in 16 bytes lines added for
// port 4. Reset upon init.
pub const PBF_REG_P4_INTERNAL_CRD_FREED_CNT: c_uint = 0x140310;
// [R 8] Removed for E3 B0 - Number of tasks in port 4 task queue.
pub const PBF_REG_P4_TASK_CNT: c_uint = 0x140214;
// [R 32] Removed for E3 B0 - Cyclic counter for number of 8 byte lines
// freed from the task queue of port 4. Reset upon init.
pub const PBF_REG_P4_TQ_LINES_FREED_CNT: c_uint = 0x1402f8;
// [R 12] Number of 8 bytes lines occupied in the task queue of port 4.
pub const PBF_REG_P4_TQ_OCCUPANCY: c_uint = 0x140304;
// [RW 5] Interrupt mask register #0 read/write
pub const PBF_REG_PBF_INT_MASK: c_uint = 0x1401d4;
// [R 5] Interrupt register #0 read
pub const PBF_REG_PBF_INT_STS: c_uint = 0x1401c8;
// [RW 20] Parity mask register #0 read/write
pub const PBF_REG_PBF_PRTY_MASK: c_uint = 0x1401e4;
// [R 28] Parity register #0 read
pub const PBF_REG_PBF_PRTY_STS: c_uint = 0x1401d8;
// [RC 20] Parity register #0 read clear
pub const PBF_REG_PBF_PRTY_STS_CLR: c_uint = 0x1401dc;
// [RW 16] The Ethernet type value for L2 tag 0
pub const PBF_REG_TAG_ETHERTYPE_0: c_uint = 0x15c090;
// [RW 4] The length of the info field for L2 tag 0. The length is between
// 2B and 14B; in 2B granularity
pub const PBF_REG_TAG_LEN_0: c_uint = 0x15c09c;
// [R 32] Cyclic counter for number of 8 byte lines freed from the LB task
// queue. Reset upon init.
pub const PBF_REG_TQ_LINES_FREED_CNT_LB_Q: c_uint = 0x14038c;
// [R 32] Cyclic counter for number of 8 byte lines freed from the task
// queue 0. Reset upon init.
pub const PBF_REG_TQ_LINES_FREED_CNT_Q0: c_uint = 0x140390;
// [R 32] Cyclic counter for number of 8 byte lines freed from task queue 1.
// Reset upon init.
pub const PBF_REG_TQ_LINES_FREED_CNT_Q1: c_uint = 0x140394;
// [R 13] Number of 8 bytes lines occupied in the task queue of the LB
// queue.
pub const PBF_REG_TQ_OCCUPANCY_LB_Q: c_uint = 0x1403a8;
// [R 13] Number of 8 bytes lines occupied in the task queue of queue 0.
pub const PBF_REG_TQ_OCCUPANCY_Q0: c_uint = 0x1403ac;
// [R 13] Number of 8 bytes lines occupied in the task queue of queue 1.
pub const PBF_REG_TQ_OCCUPANCY_Q1: c_uint = 0x1403b0;
// [RW 16] One of 8 values that should be compared to type in Ethernet
// parsing. If there is a match; the field after Ethernet is the first VLAN.
// Reset value is 0x8100 which is the standard VLAN type. Note that when
// checking second VLAN; type is compared only to 0x8100.
//
pub const PBF_REG_VLAN_TYPE_0: c_uint = 0x15c06c;
// [RW 2] Interrupt mask register #0 read/write
pub const PB_REG_PB_INT_MASK: c_uint = 0x28;
// [R 2] Interrupt register #0 read
pub const PB_REG_PB_INT_STS: c_uint = 0x1c;
// [RW 4] Parity mask register #0 read/write
pub const PB_REG_PB_PRTY_MASK: c_uint = 0x38;
// [R 4] Parity register #0 read
pub const PB_REG_PB_PRTY_STS: c_uint = 0x2c;
// [RC 4] Parity register #0 read clear
pub const PB_REG_PB_PRTY_STS_CLR: c_uint = 0x30;

// [R 8] Config space A attention dirty bits. Each bit indicates that the
// corresponding PF generates config space A attention. Set by PXP. Reset by
// MCP writing 1 to icfg_space_a_request_clr. Note: register contains bits
// from both paths.
pub const PGLUE_B_REG_CFG_SPACE_A_REQUEST: c_uint = 0x9010;
// [R 8] Config space B attention dirty bits. Each bit indicates that the
// corresponding PF generates config space B attention. Set by PXP. Reset by
// MCP writing 1 to icfg_space_b_request_clr. Note: register contains bits
// from both paths.
pub const PGLUE_B_REG_CFG_SPACE_B_REQUEST: c_uint = 0x9014;
// [RW 1] Type A PF enable inbound interrupt table for CSDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_CSDM_INB_INT_A_PF_ENABLE: c_uint = 0x9194;
// [RW 18] Type B VF inbound interrupt table for CSDM: bits[17:9]-mask;
// its[8:0]-address. Bits [1:0] must be zero (DW resolution address).
pub const PGLUE_B_REG_CSDM_INB_INT_B_VF: c_uint = 0x916c;
// [RW 1] Type B VF enable inbound interrupt table for CSDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_CSDM_INB_INT_B_VF_ENABLE: c_uint = 0x919c;
// [RW 16] Start offset of CSDM zone A (queue zone) in the internal RAM
pub const PGLUE_B_REG_CSDM_START_OFFSET_A: c_uint = 0x9100;
// [RW 16] Start offset of CSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_CSDM_START_OFFSET_B: c_uint = 0x9108;
// [RW 5] VF Shift of CSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_CSDM_VF_SHIFT_B: c_uint = 0x9110;
// [RW 1] 0 - Zone A size is 136x32B; 1 - Zone A size is 152x32B.
pub const PGLUE_B_REG_CSDM_ZONE_A_SIZE_PF: c_uint = 0x91ac;
// [R 8] FLR request attention dirty bits for PFs 0 to 7. Each bit indicates
// that the FLR register of the corresponding PF was set. Set by PXP. Reset
// by MCP writing 1 to flr_request_pf_7_0_clr. Note: register contains bits
// from both paths.
pub const PGLUE_B_REG_FLR_REQUEST_PF_7_0: c_uint = 0x9028;
// [W 8] FLR request attention dirty bits clear for PFs 0 to 7. MCP writes 1
// to a bit in this register in order to clear the corresponding bit in
// flr_request_pf_7_0 register. Note: register contains bits from both
// paths.
pub const PGLUE_B_REG_FLR_REQUEST_PF_7_0_CLR: c_uint = 0x9418;
// [R 32] FLR request attention dirty bits for VFs 96 to 127. Each bit
// indicates that the FLR register of the corresponding VF was set. Set by
// PXP. Reset by MCP writing 1 to flr_request_vf_127_96_clr.
pub const PGLUE_B_REG_FLR_REQUEST_VF_127_96: c_uint = 0x9024;
// [R 32] FLR request attention dirty bits for VFs 0 to 31. Each bit
// indicates that the FLR register of the corresponding VF was set. Set by
// PXP. Reset by MCP writing 1 to flr_request_vf_31_0_clr.
pub const PGLUE_B_REG_FLR_REQUEST_VF_31_0: c_uint = 0x9018;
// [R 32] FLR request attention dirty bits for VFs 32 to 63. Each bit
// indicates that the FLR register of the corresponding VF was set. Set by
// PXP. Reset by MCP writing 1 to flr_request_vf_63_32_clr.
pub const PGLUE_B_REG_FLR_REQUEST_VF_63_32: c_uint = 0x901c;
// [R 32] FLR request attention dirty bits for VFs 64 to 95. Each bit
// indicates that the FLR register of the corresponding VF was set. Set by
// PXP. Reset by MCP writing 1 to flr_request_vf_95_64_clr.
pub const PGLUE_B_REG_FLR_REQUEST_VF_95_64: c_uint = 0x9020;
// [R 8] Each bit indicates an incorrect behavior in user RX interface. Bit
// 0 - Target memory read arrived with a correctable error. Bit 1 - Target
// memory read arrived with an uncorrectable error. Bit 2 - Configuration RW
// arrived with a correctable error. Bit 3 - Configuration RW arrived with
// an uncorrectable error. Bit 4 - Completion with Configuration Request
// Retry Status. Bit 5 - Expansion ROM access received with a write request.
// Bit 6 - Completion with pcie_rx_err of 0000; CMPL_STATUS of non-zero; and
// pcie_rx_last not asserted. Bit 7 - Completion with pcie_rx_err of 1010;
// and pcie_rx_last not asserted.
pub const PGLUE_B_REG_INCORRECT_RCV_DETAILS: c_uint = 0x9068;
pub const PGLUE_B_REG_INTERNAL_PFID_ENABLE_MASTER: c_uint = 0x942c;
pub const PGLUE_B_REG_INTERNAL_PFID_ENABLE_TARGET_READ: c_uint = 0x9430;
pub const PGLUE_B_REG_INTERNAL_PFID_ENABLE_TARGET_WRITE: c_uint = 0x9434;
pub const PGLUE_B_REG_INTERNAL_VFID_ENABLE: c_uint = 0x9438;
// [W 7] Writing 1 to each bit in this register clears a corresponding error
// details register and enables logging new error details. Bit 0 - clears
// INCORRECT_RCV_DETAILS; Bit 1 - clears RX_ERR_DETAILS; Bit 2 - clears
// TX_ERR_WR_ADD_31_0 TX_ERR_WR_ADD_63_32 TX_ERR_WR_DETAILS
// TX_ERR_WR_DETAILS2 TX_ERR_RD_ADD_31_0 TX_ERR_RD_ADD_63_32
// TX_ERR_RD_DETAILS TX_ERR_RD_DETAILS2 TX_ERR_WR_DETAILS_ICPL; Bit 3 -
// clears VF_LENGTH_VIOLATION_DETAILS. Bit 4 - clears
// VF_GRC_SPACE_VIOLATION_DETAILS. Bit 5 - clears RX_TCPL_ERR_DETAILS. Bit 6
// - clears TCPL_IN_TWO_RCBS_DETAILS.
pub const PGLUE_B_REG_LATCHED_ERRORS_CLR: c_uint = 0x943c;
// [R 9] Interrupt register #0 read
pub const PGLUE_B_REG_PGLUE_B_INT_STS: c_uint = 0x9298;
// [RC 9] Interrupt register #0 read clear
pub const PGLUE_B_REG_PGLUE_B_INT_STS_CLR: c_uint = 0x929c;
// [RW 2] Parity mask register #0 read/write
pub const PGLUE_B_REG_PGLUE_B_PRTY_MASK: c_uint = 0x92b4;
// [R 2] Parity register #0 read
pub const PGLUE_B_REG_PGLUE_B_PRTY_STS: c_uint = 0x92a8;
// [RC 2] Parity register #0 read clear
pub const PGLUE_B_REG_PGLUE_B_PRTY_STS_CLR: c_uint = 0x92ac;
// [R 13] Details of first request received with error. [2:0] - PFID. [3] -
// VF_VALID. [9:4] - VFID. [11:10] - Error Code - 0 - Indicates Completion
// Timeout of a User Tx non-posted request. 1 - unsupported request. 2 -
// completer abort. 3 - Illegal value for this field. [12] valid - indicates
// if there was a completion error since the last time this register was
// cleared.
pub const PGLUE_B_REG_RX_ERR_DETAILS: c_uint = 0x9080;
// [R 18] Details of first ATS Translation Completion request received with
// error. [2:0] - PFID. [3] - VF_VALID. [9:4] - VFID. [11:10] - Error Code -
// 0 - Indicates Completion Timeout of a User Tx non-posted request. 1 -
// unsupported request. 2 - completer abort. 3 - Illegal value for this
// field. [16:12] - ATC OTB EntryID. [17] valid - indicates if there was a
// completion error since the last time this register was cleared.
pub const PGLUE_B_REG_RX_TCPL_ERR_DETAILS: c_uint = 0x9084;
// [W 8] Debug only - Shadow BME bits clear for PFs 0 to 7. MCP writes 1 to
// a bit in this register in order to clear the corresponding bit in
// shadow_bme_pf_7_0 register. MCP should never use this unless a
// work-around is needed. Note: register contains bits from both paths.
pub const PGLUE_B_REG_SHADOW_BME_PF_7_0_CLR: c_uint = 0x9458;
// [R 8] SR IOV disabled attention dirty bits. Each bit indicates that the
// VF enable register of the corresponding PF is written to 0 and was
// previously 1. Set by PXP. Reset by MCP writing 1 to
// sr_iov_disabled_request_clr. Note: register contains bits from both
// paths.
pub const PGLUE_B_REG_SR_IOV_DISABLED_REQUEST: c_uint = 0x9030;
// [R 32] Indicates the status of tags 32-63. 0 - tags is used - read
// completion did not return yet. 1 - tag is unused. Same functionality as
// pxp2_registers_pgl_exp_rom_data2 for tags 0-31.
pub const PGLUE_B_REG_TAGS_63_32: c_uint = 0x9244;
// [RW 1] Type A PF enable inbound interrupt table for TSDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_TSDM_INB_INT_A_PF_ENABLE: c_uint = 0x9170;
// [RW 16] Start offset of TSDM zone A (queue zone) in the internal RAM
pub const PGLUE_B_REG_TSDM_START_OFFSET_A: c_uint = 0x90c4;
// [RW 16] Start offset of TSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_TSDM_START_OFFSET_B: c_uint = 0x90cc;
// [RW 5] VF Shift of TSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_TSDM_VF_SHIFT_B: c_uint = 0x90d4;
// [RW 1] 0 - Zone A size is 136x32B; 1 - Zone A size is 152x32B.
pub const PGLUE_B_REG_TSDM_ZONE_A_SIZE_PF: c_uint = 0x91a0;
// [R 32] Address [31:0] of first read request not submitted due to error
pub const PGLUE_B_REG_TX_ERR_RD_ADD_31_0: c_uint = 0x9098;
// [R 32] Address [63:32] of first read request not submitted due to error
pub const PGLUE_B_REG_TX_ERR_RD_ADD_63_32: c_uint = 0x909c;
// [R 31] Details of first read request not submitted due to error. [4:0]
// VQID. [5] TREQ. 1 - Indicates the request is a Translation Request.
// [20:8] - Length in bytes. [23:21] - PFID. [24] - VF_VALID. [30:25] -
// VFID.
pub const PGLUE_B_REG_TX_ERR_RD_DETAILS: c_uint = 0x90a0;
// [R 26] Details of first read request not submitted due to error. [15:0]
// Request ID. [19:16] client ID. [20] - last SR. [24:21] - Error type -
// [21] - Indicates was_error was set; [22] - Indicates BME was cleared;
// [23] - Indicates FID_enable was cleared; [24] - Indicates VF with parent
// PF FLR_request or IOV_disable_request dirty bit is set. [25] valid -
// indicates if there was a request not submitted due to error since the
// last time this register was cleared.
pub const PGLUE_B_REG_TX_ERR_RD_DETAILS2: c_uint = 0x90a4;
// [R 32] Address [31:0] of first write request not submitted due to error
pub const PGLUE_B_REG_TX_ERR_WR_ADD_31_0: c_uint = 0x9088;
// [R 32] Address [63:32] of first write request not submitted due to error
pub const PGLUE_B_REG_TX_ERR_WR_ADD_63_32: c_uint = 0x908c;
// [R 31] Details of first write request not submitted due to error. [4:0]
// VQID. [20:8] - Length in bytes. [23:21] - PFID. [24] - VF_VALID. [30:25]
// - VFID.
pub const PGLUE_B_REG_TX_ERR_WR_DETAILS: c_uint = 0x9090;
// [R 26] Details of first write request not submitted due to error. [15:0]
// Request ID. [19:16] client ID. [20] - last SR. [24:21] - Error type -
// [21] - Indicates was_error was set; [22] - Indicates BME was cleared;
// [23] - Indicates FID_enable was cleared; [24] - Indicates VF with parent
// PF FLR_request or IOV_disable_request dirty bit is set. [25] valid -
// indicates if there was a request not submitted due to error since the
// last time this register was cleared.
pub const PGLUE_B_REG_TX_ERR_WR_DETAILS2: c_uint = 0x9094;
// [RW 10] Type A PF/VF inbound interrupt table for USDM: bits[9:5]-mask;
// its[4:0]-address relative to start_offset_a. Bits [1:0] can have any
// value (Byte resolution address).
pub const PGLUE_B_REG_USDM_INB_INT_A_0: c_uint = 0x9128;
pub const PGLUE_B_REG_USDM_INB_INT_A_1: c_uint = 0x912c;
pub const PGLUE_B_REG_USDM_INB_INT_A_2: c_uint = 0x9130;
pub const PGLUE_B_REG_USDM_INB_INT_A_3: c_uint = 0x9134;
pub const PGLUE_B_REG_USDM_INB_INT_A_4: c_uint = 0x9138;
pub const PGLUE_B_REG_USDM_INB_INT_A_5: c_uint = 0x913c;
pub const PGLUE_B_REG_USDM_INB_INT_A_6: c_uint = 0x9140;
// [RW 1] Type A PF enable inbound interrupt table for USDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_USDM_INB_INT_A_PF_ENABLE: c_uint = 0x917c;
// [RW 1] Type A VF enable inbound interrupt table for USDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_USDM_INB_INT_A_VF_ENABLE: c_uint = 0x9180;
// [RW 1] Type B VF enable inbound interrupt table for USDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_USDM_INB_INT_B_VF_ENABLE: c_uint = 0x9184;
// [RW 16] Start offset of USDM zone A (queue zone) in the internal RAM
pub const PGLUE_B_REG_USDM_START_OFFSET_A: c_uint = 0x90d8;
// [RW 16] Start offset of USDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_USDM_START_OFFSET_B: c_uint = 0x90e0;
// [RW 5] VF Shift of USDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_USDM_VF_SHIFT_B: c_uint = 0x90e8;
// [RW 1] 0 - Zone A size is 136x32B; 1 - Zone A size is 152x32B.
pub const PGLUE_B_REG_USDM_ZONE_A_SIZE_PF: c_uint = 0x91a4;
// [R 26] Details of first target VF request accessing VF GRC space that
// failed permission check. [14:0] Address. [15] w_nr: 0 - Read; 1 - Write.
// [21:16] VFID. [24:22] - PFID. [25] valid - indicates if there was a
// request accessing VF GRC space that failed permission check since the
// last time this register was cleared. Permission checks are: function
// permission; R/W permission; address range permission.
pub const PGLUE_B_REG_VF_GRC_SPACE_VIOLATION_DETAILS: c_uint = 0x9234;
// [R 31] Details of first target VF request with length violation (too many
// DWs) accessing BAR0. [12:0] Address in DWs (bits [14:2] of byte address).
// [14:13] BAR. [20:15] VFID. [23:21] - PFID. [29:24] - Length in DWs. [30]
// valid - indicates if there was a request with length violation since the
// last time this register was cleared. Length violations: length of more
// than 2DWs; length of 2DWs and address not QW aligned; window is GRC and
// length is more than 1 DW.
pub const PGLUE_B_REG_VF_LENGTH_VIOLATION_DETAILS: c_uint = 0x9230;
// [R 8] Was_error indication dirty bits for PFs 0 to 7. Each bit indicates
// that there was a completion with uncorrectable error for the
// corresponding PF. Set by PXP. Reset by MCP writing 1 to
// was_error_pf_7_0_clr.
pub const PGLUE_B_REG_WAS_ERROR_PF_7_0: c_uint = 0x907c;
// [W 8] Was_error indication dirty bits clear for PFs 0 to 7. MCP writes 1
// to a bit in this register in order to clear the corresponding bit in
// flr_request_pf_7_0 register.
pub const PGLUE_B_REG_WAS_ERROR_PF_7_0_CLR: c_uint = 0x9470;
// [R 32] Was_error indication dirty bits for VFs 96 to 127. Each bit
// indicates that there was a completion with uncorrectable error for the
// corresponding VF. Set by PXP. Reset by MCP writing 1 to
// was_error_vf_127_96_clr.
pub const PGLUE_B_REG_WAS_ERROR_VF_127_96: c_uint = 0x9078;
// [W 32] Was_error indication dirty bits clear for VFs 96 to 127. MCP
// writes 1 to a bit in this register in order to clear the corresponding
// bit in was_error_vf_127_96 register.
pub const PGLUE_B_REG_WAS_ERROR_VF_127_96_CLR: c_uint = 0x9474;
// [R 32] Was_error indication dirty bits for VFs 0 to 31. Each bit
// indicates that there was a completion with uncorrectable error for the
// corresponding VF. Set by PXP. Reset by MCP writing 1 to
// was_error_vf_31_0_clr.
pub const PGLUE_B_REG_WAS_ERROR_VF_31_0: c_uint = 0x906c;
// [W 32] Was_error indication dirty bits clear for VFs 0 to 31. MCP writes
// 1 to a bit in this register in order to clear the corresponding bit in
// was_error_vf_31_0 register.
pub const PGLUE_B_REG_WAS_ERROR_VF_31_0_CLR: c_uint = 0x9478;
// [R 32] Was_error indication dirty bits for VFs 32 to 63. Each bit
// indicates that there was a completion with uncorrectable error for the
// corresponding VF. Set by PXP. Reset by MCP writing 1 to
// was_error_vf_63_32_clr.
pub const PGLUE_B_REG_WAS_ERROR_VF_63_32: c_uint = 0x9070;
// [W 32] Was_error indication dirty bits clear for VFs 32 to 63. MCP writes
// 1 to a bit in this register in order to clear the corresponding bit in
// was_error_vf_63_32 register.
pub const PGLUE_B_REG_WAS_ERROR_VF_63_32_CLR: c_uint = 0x947c;
// [R 32] Was_error indication dirty bits for VFs 64 to 95. Each bit
// indicates that there was a completion with uncorrectable error for the
// corresponding VF. Set by PXP. Reset by MCP writing 1 to
// was_error_vf_95_64_clr.
pub const PGLUE_B_REG_WAS_ERROR_VF_95_64: c_uint = 0x9074;
// [W 32] Was_error indication dirty bits clear for VFs 64 to 95. MCP writes
// 1 to a bit in this register in order to clear the corresponding bit in
// was_error_vf_95_64 register.
pub const PGLUE_B_REG_WAS_ERROR_VF_95_64_CLR: c_uint = 0x9480;
// [RW 1] Type A PF enable inbound interrupt table for XSDM. 0 - disable; 1
// - enable.
pub const PGLUE_B_REG_XSDM_INB_INT_A_PF_ENABLE: c_uint = 0x9188;
// [RW 16] Start offset of XSDM zone A (queue zone) in the internal RAM
pub const PGLUE_B_REG_XSDM_START_OFFSET_A: c_uint = 0x90ec;
// [RW 16] Start offset of XSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_XSDM_START_OFFSET_B: c_uint = 0x90f4;
// [RW 5] VF Shift of XSDM zone B (legacy zone) in the internal RAM
pub const PGLUE_B_REG_XSDM_VF_SHIFT_B: c_uint = 0x90fc;
// [RW 1] 0 - Zone A size is 136x32B; 1 - Zone A size is 152x32B.
pub const PGLUE_B_REG_XSDM_ZONE_A_SIZE_PF: c_uint = 0x91a8;
pub const PRS_REG_A_PRSU_20: c_uint = 0x40134;
// [R 8] debug only: CFC load request current credit. Transaction based.
pub const PRS_REG_CFC_LD_CURRENT_CREDIT: c_uint = 0x40164;
// [R 8] debug only: CFC search request current credit. Transaction based.
pub const PRS_REG_CFC_SEARCH_CURRENT_CREDIT: c_uint = 0x40168;
// [RW 6] The initial credit for the search message to the CFC interface.
pub const PRS_REG_CFC_SEARCH_INITIAL_CREDIT: c_uint = 0x4011c;
// [RW 24] CID for port 0 if no match
pub const PRS_REG_CID_PORT_0: c_uint = 0x400fc;
// [RW 32] The CM header for flush message where 'load existed' bit in CFC
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_0: c_uint = 0x400dc;
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_1: c_uint = 0x400e0;
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_2: c_uint = 0x400e4;
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_3: c_uint = 0x400e8;
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_4: c_uint = 0x400ec;
pub const PRS_REG_CM_HDR_FLUSH_LOAD_TYPE_5: c_uint = 0x400f0;
// [RW 32] The CM header for flush message where 'load existed' bit in CFC
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_0: c_uint = 0x400bc;
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_1: c_uint = 0x400c0;
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_2: c_uint = 0x400c4;
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_3: c_uint = 0x400c8;
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_4: c_uint = 0x400cc;
pub const PRS_REG_CM_HDR_FLUSH_NO_LOAD_TYPE_5: c_uint = 0x400d0;
// [RW 32] The CM header for a match and packet type 1 for loopback port.
pub const PRS_REG_CM_HDR_LOOPBACK_TYPE_1: c_uint = 0x4009c;
pub const PRS_REG_CM_HDR_LOOPBACK_TYPE_2: c_uint = 0x400a0;
pub const PRS_REG_CM_HDR_LOOPBACK_TYPE_3: c_uint = 0x400a4;
pub const PRS_REG_CM_HDR_LOOPBACK_TYPE_4: c_uint = 0x400a8;
// [RW 32] The CM header for a match and packet type 0. Used in packet start
pub const PRS_REG_CM_HDR_TYPE_0: c_uint = 0x40078;
pub const PRS_REG_CM_HDR_TYPE_1: c_uint = 0x4007c;
pub const PRS_REG_CM_HDR_TYPE_2: c_uint = 0x40080;
pub const PRS_REG_CM_HDR_TYPE_3: c_uint = 0x40084;
pub const PRS_REG_CM_HDR_TYPE_4: c_uint = 0x40088;
// [RW 32] The CM header in case there was not a match on the connection
pub const PRS_REG_CM_NO_MATCH_HDR: c_uint = 0x400b8;
// [RW 1] Indicates if in e1hov mode. 0=non-e1hov mode; 1=e1hov mode.
pub const PRS_REG_E1HOV_MODE: c_uint = 0x401c8;
// [RW 8] The 8-bit event ID for a match and packet type 1. Used in packet
pub const PRS_REG_EVENT_ID_1: c_uint = 0x40054;
pub const PRS_REG_EVENT_ID_2: c_uint = 0x40058;
pub const PRS_REG_EVENT_ID_3: c_uint = 0x4005c;
// [RW 16] The Ethernet type value for FCoE
pub const PRS_REG_FCOE_TYPE: c_uint = 0x401d0;
// [RW 8] Context region for flush packet with packet type 0. Used in CFC
pub const PRS_REG_FLUSH_REGIONS_TYPE_0: c_uint = 0x40004;
pub const PRS_REG_FLUSH_REGIONS_TYPE_1: c_uint = 0x40008;
pub const PRS_REG_FLUSH_REGIONS_TYPE_2: c_uint = 0x4000c;
pub const PRS_REG_FLUSH_REGIONS_TYPE_3: c_uint = 0x40010;
pub const PRS_REG_FLUSH_REGIONS_TYPE_4: c_uint = 0x40014;
pub const PRS_REG_FLUSH_REGIONS_TYPE_5: c_uint = 0x40018;
pub const PRS_REG_FLUSH_REGIONS_TYPE_6: c_uint = 0x4001c;
pub const PRS_REG_FLUSH_REGIONS_TYPE_7: c_uint = 0x40020;
// [RW 6] Bit-map indicating which L2 hdrs may appear after the basic
// Ethernet header.
pub const PRS_REG_HDRS_AFTER_BASIC: c_uint = 0x40238;
// [RW 6] Bit-map indicating which L2 hdrs may appear after the basic
// Ethernet header for port 0 packets.
pub const PRS_REG_HDRS_AFTER_BASIC_PORT_0: c_uint = 0x40270;
pub const PRS_REG_HDRS_AFTER_BASIC_PORT_1: c_uint = 0x40290;
// [R 6] Bit-map indicating which L2 hdrs may appear after L2 tag 0
pub const PRS_REG_HDRS_AFTER_TAG_0: c_uint = 0x40248;
// [RW 6] Bit-map indicating which L2 hdrs may appear after L2 tag 0 for
// port 0 packets
pub const PRS_REG_HDRS_AFTER_TAG_0_PORT_0: c_uint = 0x40280;
pub const PRS_REG_HDRS_AFTER_TAG_0_PORT_1: c_uint = 0x402a0;
// [RW 4] The increment value to send in the CFC load request message
pub const PRS_REG_INC_VALUE: c_uint = 0x40048;
// [RW 6] Bit-map indicating which headers must appear in the packet
pub const PRS_REG_MUST_HAVE_HDRS: c_uint = 0x40254;
// [RW 6] Bit-map indicating which headers must appear in the packet for
// port 0 packets
pub const PRS_REG_MUST_HAVE_HDRS_PORT_0: c_uint = 0x4028c;
pub const PRS_REG_MUST_HAVE_HDRS_PORT_1: c_uint = 0x402ac;
pub const PRS_REG_NIC_MODE: c_uint = 0x40138;
// [RW 8] The 8-bit event ID for cases where there is no match on the
pub const PRS_REG_NO_MATCH_EVENT_ID: c_uint = 0x40070;
// [ST 24] The number of input CFC flush packets
pub const PRS_REG_NUM_OF_CFC_FLUSH_MESSAGES: c_uint = 0x40128;
// [ST 32] The number of cycles the Parser halted its operation since it
pub const PRS_REG_NUM_OF_DEAD_CYCLES: c_uint = 0x40130;
// [ST 24] The number of input packets
pub const PRS_REG_NUM_OF_PACKETS: c_uint = 0x40124;
// [ST 24] The number of input transparent flush packets
pub const PRS_REG_NUM_OF_TRANSPARENT_FLUSH_MESSAGES: c_uint = 0x4012c;
// [RW 8] Context region for received Ethernet packet with a match and
pub const PRS_REG_PACKET_REGIONS_TYPE_0: c_uint = 0x40028;
pub const PRS_REG_PACKET_REGIONS_TYPE_1: c_uint = 0x4002c;
pub const PRS_REG_PACKET_REGIONS_TYPE_2: c_uint = 0x40030;
pub const PRS_REG_PACKET_REGIONS_TYPE_3: c_uint = 0x40034;
pub const PRS_REG_PACKET_REGIONS_TYPE_4: c_uint = 0x40038;
pub const PRS_REG_PACKET_REGIONS_TYPE_5: c_uint = 0x4003c;
pub const PRS_REG_PACKET_REGIONS_TYPE_6: c_uint = 0x40040;
pub const PRS_REG_PACKET_REGIONS_TYPE_7: c_uint = 0x40044;
// [R 2] debug only: Number of pending requests for CAC on port 0.
pub const PRS_REG_PENDING_BRB_CAC0_RQ: c_uint = 0x40174;
// [R 2] debug only: Number of pending requests for header parsing.
pub const PRS_REG_PENDING_BRB_PRS_RQ: c_uint = 0x40170;
// [R 1] Interrupt register #0 read
pub const PRS_REG_PRS_INT_STS: c_uint = 0x40188;
// [RW 8] Parity mask register #0 read/write
pub const PRS_REG_PRS_PRTY_MASK: c_uint = 0x401a4;
// [R 8] Parity register #0 read
pub const PRS_REG_PRS_PRTY_STS: c_uint = 0x40198;
// [RC 8] Parity register #0 read clear
pub const PRS_REG_PRS_PRTY_STS_CLR: c_uint = 0x4019c;
// [RW 8] Context region for pure acknowledge packets. Used in CFC load
pub const PRS_REG_PURE_REGIONS: c_uint = 0x40024;
// [R 32] debug only: Serial number status lsb 32 bits. '1' indicates this
pub const PRS_REG_SERIAL_NUM_STATUS_LSB: c_uint = 0x40154;
// [R 32] debug only: Serial number status msb 32 bits. '1' indicates this
pub const PRS_REG_SERIAL_NUM_STATUS_MSB: c_uint = 0x40158;
// [R 4] debug only: SRC current credit. Transaction based.
pub const PRS_REG_SRC_CURRENT_CREDIT: c_uint = 0x4016c;
// [RW 16] The Ethernet type value for L2 tag 0
pub const PRS_REG_TAG_ETHERTYPE_0: c_uint = 0x401d4;
// [RW 4] The length of the info field for L2 tag 0. The length is between
// 2B and 14B; in 2B granularity
pub const PRS_REG_TAG_LEN_0: c_uint = 0x4022c;
// [R 8] debug only: TCM current credit. Cycle based.
pub const PRS_REG_TCM_CURRENT_CREDIT: c_uint = 0x40160;
// [R 8] debug only: TSDM current credit. Transaction based.
pub const PRS_REG_TSDM_CURRENT_CREDIT: c_uint = 0x4015c;
// [RW 16] One of 8 values that should be compared to type in Ethernet
// parsing. If there is a match; the field after Ethernet is the first VLAN.
// Reset value is 0x8100 which is the standard VLAN type. Note that when
// checking second VLAN; type is compared only to 0x8100.
//
pub const PRS_REG_VLAN_TYPE_0: c_uint = 0x401a8;

// [R 6] Debug only: Number of used entries in the data FIFO
pub const PXP2_REG_HST_DATA_FIFO_STATUS: c_uint = 0x12047c;
// [R 7] Debug only: Number of used entries in the header FIFO
pub const PXP2_REG_HST_HEADER_FIFO_STATUS: c_uint = 0x120478;
pub const PXP2_REG_PGL_ADDR_88_F0: c_uint = 0x120534;
// [R 32] GRC address for configuration access to PCIE config address 0x88.
// any write to this PCIE address will cause a GRC write access to the
// address that's in t this register
pub const PXP2_REG_PGL_ADDR_88_F1: c_uint = 0x120544;
pub const PXP2_REG_PGL_ADDR_8C_F0: c_uint = 0x120538;
// [R 32] GRC address for configuration access to PCIE config address 0x8c.
// any write to this PCIE address will cause a GRC write access to the
// address that's in t this register
pub const PXP2_REG_PGL_ADDR_8C_F1: c_uint = 0x120548;
pub const PXP2_REG_PGL_ADDR_90_F0: c_uint = 0x12053c;
// [R 32] GRC address for configuration access to PCIE config address 0x90.
// any write to this PCIE address will cause a GRC write access to the
// address that's in t this register
pub const PXP2_REG_PGL_ADDR_90_F1: c_uint = 0x12054c;
pub const PXP2_REG_PGL_ADDR_94_F0: c_uint = 0x120540;
// [R 32] GRC address for configuration access to PCIE config address 0x94.
// any write to this PCIE address will cause a GRC write access to the
// address that's in t this register
pub const PXP2_REG_PGL_ADDR_94_F1: c_uint = 0x120550;
pub const PXP2_REG_PGL_CONTROL0: c_uint = 0x120490;
pub const PXP2_REG_PGL_CONTROL1: c_uint = 0x120514;
pub const PXP2_REG_PGL_DEBUG: c_uint = 0x120520;
// [RW 32] third dword data of expansion rom request. this register is
pub const PXP2_REG_PGL_EXP_ROM2: c_uint = 0x120808;
// [RW 32] Inbound interrupt table for CSDM: bits[31:16]-mask;
pub const PXP2_REG_PGL_INT_CSDM_0: c_uint = 0x1204f4;
pub const PXP2_REG_PGL_INT_CSDM_1: c_uint = 0x1204f8;
pub const PXP2_REG_PGL_INT_CSDM_2: c_uint = 0x1204fc;
pub const PXP2_REG_PGL_INT_CSDM_3: c_uint = 0x120500;
pub const PXP2_REG_PGL_INT_CSDM_4: c_uint = 0x120504;
pub const PXP2_REG_PGL_INT_CSDM_5: c_uint = 0x120508;
pub const PXP2_REG_PGL_INT_CSDM_6: c_uint = 0x12050c;
pub const PXP2_REG_PGL_INT_CSDM_7: c_uint = 0x120510;
// [RW 32] Inbound interrupt table for TSDM: bits[31:16]-mask;
pub const PXP2_REG_PGL_INT_TSDM_0: c_uint = 0x120494;
pub const PXP2_REG_PGL_INT_TSDM_1: c_uint = 0x120498;
pub const PXP2_REG_PGL_INT_TSDM_2: c_uint = 0x12049c;
pub const PXP2_REG_PGL_INT_TSDM_3: c_uint = 0x1204a0;
pub const PXP2_REG_PGL_INT_TSDM_4: c_uint = 0x1204a4;
pub const PXP2_REG_PGL_INT_TSDM_5: c_uint = 0x1204a8;
pub const PXP2_REG_PGL_INT_TSDM_6: c_uint = 0x1204ac;
pub const PXP2_REG_PGL_INT_TSDM_7: c_uint = 0x1204b0;
// [RW 32] Inbound interrupt table for USDM: bits[31:16]-mask;
pub const PXP2_REG_PGL_INT_USDM_0: c_uint = 0x1204b4;
pub const PXP2_REG_PGL_INT_USDM_1: c_uint = 0x1204b8;
pub const PXP2_REG_PGL_INT_USDM_2: c_uint = 0x1204bc;
pub const PXP2_REG_PGL_INT_USDM_3: c_uint = 0x1204c0;
pub const PXP2_REG_PGL_INT_USDM_4: c_uint = 0x1204c4;
pub const PXP2_REG_PGL_INT_USDM_5: c_uint = 0x1204c8;
pub const PXP2_REG_PGL_INT_USDM_6: c_uint = 0x1204cc;
pub const PXP2_REG_PGL_INT_USDM_7: c_uint = 0x1204d0;
// [RW 32] Inbound interrupt table for XSDM: bits[31:16]-mask;
pub const PXP2_REG_PGL_INT_XSDM_0: c_uint = 0x1204d4;
pub const PXP2_REG_PGL_INT_XSDM_1: c_uint = 0x1204d8;
pub const PXP2_REG_PGL_INT_XSDM_2: c_uint = 0x1204dc;
pub const PXP2_REG_PGL_INT_XSDM_3: c_uint = 0x1204e0;
pub const PXP2_REG_PGL_INT_XSDM_4: c_uint = 0x1204e4;
pub const PXP2_REG_PGL_INT_XSDM_5: c_uint = 0x1204e8;
pub const PXP2_REG_PGL_INT_XSDM_6: c_uint = 0x1204ec;
pub const PXP2_REG_PGL_INT_XSDM_7: c_uint = 0x1204f0;
// [RW 3] this field allows one function to pretend being another function
pub const PXP2_REG_PGL_PRETEND_FUNC_F0: c_uint = 0x120674;
pub const PXP2_REG_PGL_PRETEND_FUNC_F1: c_uint = 0x120678;
pub const PXP2_REG_PGL_PRETEND_FUNC_F2: c_uint = 0x12067c;
pub const PXP2_REG_PGL_PRETEND_FUNC_F3: c_uint = 0x120680;
pub const PXP2_REG_PGL_PRETEND_FUNC_F4: c_uint = 0x120684;
pub const PXP2_REG_PGL_PRETEND_FUNC_F5: c_uint = 0x120688;
pub const PXP2_REG_PGL_PRETEND_FUNC_F6: c_uint = 0x12068c;
pub const PXP2_REG_PGL_PRETEND_FUNC_F7: c_uint = 0x120690;
// [R 1] this bit indicates that a read request was blocked because of
pub const PXP2_REG_PGL_READ_BLOCKED: c_uint = 0x120568;
pub const PXP2_REG_PGL_TAGS_LIMIT: c_uint = 0x1205a8;
// [R 18] debug only
pub const PXP2_REG_PGL_TXW_CDTS: c_uint = 0x12052c;
// [R 1] this bit indicates that a write request was blocked because of
pub const PXP2_REG_PGL_WRITE_BLOCKED: c_uint = 0x120564;
pub const PXP2_REG_PSWRQ_BW_ADD1: c_uint = 0x1201c0;
pub const PXP2_REG_PSWRQ_BW_ADD10: c_uint = 0x1201e4;
pub const PXP2_REG_PSWRQ_BW_ADD11: c_uint = 0x1201e8;
pub const PXP2_REG_PSWRQ_BW_ADD2: c_uint = 0x1201c4;
pub const PXP2_REG_PSWRQ_BW_ADD28: c_uint = 0x120228;
pub const PXP2_REG_PSWRQ_BW_ADD3: c_uint = 0x1201c8;
pub const PXP2_REG_PSWRQ_BW_ADD6: c_uint = 0x1201d4;
pub const PXP2_REG_PSWRQ_BW_ADD7: c_uint = 0x1201d8;
pub const PXP2_REG_PSWRQ_BW_ADD8: c_uint = 0x1201dc;
pub const PXP2_REG_PSWRQ_BW_ADD9: c_uint = 0x1201e0;
pub const PXP2_REG_PSWRQ_BW_CREDIT: c_uint = 0x12032c;
pub const PXP2_REG_PSWRQ_BW_L1: c_uint = 0x1202b0;
pub const PXP2_REG_PSWRQ_BW_L10: c_uint = 0x1202d4;
pub const PXP2_REG_PSWRQ_BW_L11: c_uint = 0x1202d8;
pub const PXP2_REG_PSWRQ_BW_L2: c_uint = 0x1202b4;
pub const PXP2_REG_PSWRQ_BW_L28: c_uint = 0x120318;
pub const PXP2_REG_PSWRQ_BW_L3: c_uint = 0x1202b8;
pub const PXP2_REG_PSWRQ_BW_L6: c_uint = 0x1202c4;
pub const PXP2_REG_PSWRQ_BW_L7: c_uint = 0x1202c8;
pub const PXP2_REG_PSWRQ_BW_L8: c_uint = 0x1202cc;
pub const PXP2_REG_PSWRQ_BW_L9: c_uint = 0x1202d0;
pub const PXP2_REG_PSWRQ_BW_RD: c_uint = 0x120324;
pub const PXP2_REG_PSWRQ_BW_UB1: c_uint = 0x120238;
pub const PXP2_REG_PSWRQ_BW_UB10: c_uint = 0x12025c;
pub const PXP2_REG_PSWRQ_BW_UB11: c_uint = 0x120260;
pub const PXP2_REG_PSWRQ_BW_UB2: c_uint = 0x12023c;
pub const PXP2_REG_PSWRQ_BW_UB28: c_uint = 0x1202a0;
pub const PXP2_REG_PSWRQ_BW_UB3: c_uint = 0x120240;
pub const PXP2_REG_PSWRQ_BW_UB6: c_uint = 0x12024c;
pub const PXP2_REG_PSWRQ_BW_UB7: c_uint = 0x120250;
pub const PXP2_REG_PSWRQ_BW_UB8: c_uint = 0x120254;
pub const PXP2_REG_PSWRQ_BW_UB9: c_uint = 0x120258;
pub const PXP2_REG_PSWRQ_BW_WR: c_uint = 0x120328;
pub const PXP2_REG_PSWRQ_CDU0_L2P: c_uint = 0x120000;
pub const PXP2_REG_PSWRQ_QM0_L2P: c_uint = 0x120038;
pub const PXP2_REG_PSWRQ_SRC0_L2P: c_uint = 0x120054;
pub const PXP2_REG_PSWRQ_TM0_L2P: c_uint = 0x12001c;
pub const PXP2_REG_PSWRQ_TSDM0_L2P: c_uint = 0x1200e0;
// [RW 32] Interrupt mask register #0 read/write
pub const PXP2_REG_PXP2_INT_MASK_0: c_uint = 0x120578;
// [R 32] Interrupt register #0 read
pub const PXP2_REG_PXP2_INT_STS_0: c_uint = 0x12056c;
pub const PXP2_REG_PXP2_INT_STS_1: c_uint = 0x120608;
// [RC 32] Interrupt register #0 read clear
pub const PXP2_REG_PXP2_INT_STS_CLR_0: c_uint = 0x120570;
// [RW 32] Parity mask register #0 read/write
pub const PXP2_REG_PXP2_PRTY_MASK_0: c_uint = 0x120588;
pub const PXP2_REG_PXP2_PRTY_MASK_1: c_uint = 0x120598;
// [R 32] Parity register #0 read
pub const PXP2_REG_PXP2_PRTY_STS_0: c_uint = 0x12057c;
pub const PXP2_REG_PXP2_PRTY_STS_1: c_uint = 0x12058c;
// [RC 32] Parity register #0 read clear
pub const PXP2_REG_PXP2_PRTY_STS_CLR_0: c_uint = 0x120580;
pub const PXP2_REG_PXP2_PRTY_STS_CLR_1: c_uint = 0x120590;
// [R 1] Debug only: The 'almost full' indication from each fifo (gives
pub const PXP2_REG_RD_ALMOST_FULL_0: c_uint = 0x120424;
// [R 8] Debug only: The blocks counter - number of unused block ids
pub const PXP2_REG_RD_BLK_CNT: c_uint = 0x120418;
// [RW 8] Debug only: Total number of available blocks in Tetris Buffer.
pub const PXP2_REG_RD_BLK_NUM_CFG: c_uint = 0x12040c;
// [RW 2] CDU byte swapping mode configuration for master read requests
pub const PXP2_REG_RD_CDURD_SWAP_MODE: c_uint = 0x120404;
// [RW 1] When '1'; inputs to the PSWRD block are ignored
pub const PXP2_REG_RD_DISABLE_INPUTS: c_uint = 0x120374;
// [R 1] PSWRD internal memories initialization is done
pub const PXP2_REG_RD_INIT_DONE: c_uint = 0x120370;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ10: c_uint = 0x1203a0;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ11: c_uint = 0x1203a4;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ17: c_uint = 0x1203bc;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ18: c_uint = 0x1203c0;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ19: c_uint = 0x1203c4;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ22: c_uint = 0x1203d0;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ25: c_uint = 0x1203dc;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ6: c_uint = 0x120390;
// [RW 8] The maximum number of blocks in Tetris Buffer that can be
pub const PXP2_REG_RD_MAX_BLKS_VQ9: c_uint = 0x12039c;
// [RW 2] PBF byte swapping mode configuration for master read requests
pub const PXP2_REG_RD_PBF_SWAP_MODE: c_uint = 0x1203f4;
// [R 1] Debug only: Indication if delivery ports are idle
pub const PXP2_REG_RD_PORT_IS_IDLE_0: c_uint = 0x12041c;
pub const PXP2_REG_RD_PORT_IS_IDLE_1: c_uint = 0x120420;
// [RW 2] QM byte swapping mode configuration for master read requests
pub const PXP2_REG_RD_QM_SWAP_MODE: c_uint = 0x1203f8;
// [R 7] Debug only: The SR counter - number of unused sub request ids
pub const PXP2_REG_RD_SR_CNT: c_uint = 0x120414;
// [RW 2] SRC byte swapping mode configuration for master read requests
pub const PXP2_REG_RD_SRC_SWAP_MODE: c_uint = 0x120400;
// [RW 7] Debug only: Total number of available PCI read sub-requests. Must
pub const PXP2_REG_RD_SR_NUM_CFG: c_uint = 0x120408;
// [RW 1] Signals the PSWRD block to start initializing internal memories
pub const PXP2_REG_RD_START_INIT: c_uint = 0x12036c;
// [RW 2] TM byte swapping mode configuration for master read requests
pub const PXP2_REG_RD_TM_SWAP_MODE: c_uint = 0x1203fc;
// [RW 10] Bandwidth addition to VQ0 write requests
pub const PXP2_REG_RQ_BW_RD_ADD0: c_uint = 0x1201bc;
// [RW 10] Bandwidth addition to VQ12 read requests
pub const PXP2_REG_RQ_BW_RD_ADD12: c_uint = 0x1201ec;
// [RW 10] Bandwidth addition to VQ13 read requests
pub const PXP2_REG_RQ_BW_RD_ADD13: c_uint = 0x1201f0;
// [RW 10] Bandwidth addition to VQ14 read requests
pub const PXP2_REG_RQ_BW_RD_ADD14: c_uint = 0x1201f4;
// [RW 10] Bandwidth addition to VQ15 read requests
pub const PXP2_REG_RQ_BW_RD_ADD15: c_uint = 0x1201f8;
// [RW 10] Bandwidth addition to VQ16 read requests
pub const PXP2_REG_RQ_BW_RD_ADD16: c_uint = 0x1201fc;
// [RW 10] Bandwidth addition to VQ17 read requests
pub const PXP2_REG_RQ_BW_RD_ADD17: c_uint = 0x120200;
// [RW 10] Bandwidth addition to VQ18 read requests
pub const PXP2_REG_RQ_BW_RD_ADD18: c_uint = 0x120204;
// [RW 10] Bandwidth addition to VQ19 read requests
pub const PXP2_REG_RQ_BW_RD_ADD19: c_uint = 0x120208;
// [RW 10] Bandwidth addition to VQ20 read requests
pub const PXP2_REG_RQ_BW_RD_ADD20: c_uint = 0x12020c;
// [RW 10] Bandwidth addition to VQ22 read requests
pub const PXP2_REG_RQ_BW_RD_ADD22: c_uint = 0x120210;
// [RW 10] Bandwidth addition to VQ23 read requests
pub const PXP2_REG_RQ_BW_RD_ADD23: c_uint = 0x120214;
// [RW 10] Bandwidth addition to VQ24 read requests
pub const PXP2_REG_RQ_BW_RD_ADD24: c_uint = 0x120218;
// [RW 10] Bandwidth addition to VQ25 read requests
pub const PXP2_REG_RQ_BW_RD_ADD25: c_uint = 0x12021c;
// [RW 10] Bandwidth addition to VQ26 read requests
pub const PXP2_REG_RQ_BW_RD_ADD26: c_uint = 0x120220;
// [RW 10] Bandwidth addition to VQ27 read requests
pub const PXP2_REG_RQ_BW_RD_ADD27: c_uint = 0x120224;
// [RW 10] Bandwidth addition to VQ4 read requests
pub const PXP2_REG_RQ_BW_RD_ADD4: c_uint = 0x1201cc;
// [RW 10] Bandwidth addition to VQ5 read requests
pub const PXP2_REG_RQ_BW_RD_ADD5: c_uint = 0x1201d0;
// [RW 10] Bandwidth Typical L for VQ0 Read requests
pub const PXP2_REG_RQ_BW_RD_L0: c_uint = 0x1202ac;
// [RW 10] Bandwidth Typical L for VQ12 Read requests
pub const PXP2_REG_RQ_BW_RD_L12: c_uint = 0x1202dc;
// [RW 10] Bandwidth Typical L for VQ13 Read requests
pub const PXP2_REG_RQ_BW_RD_L13: c_uint = 0x1202e0;
// [RW 10] Bandwidth Typical L for VQ14 Read requests
pub const PXP2_REG_RQ_BW_RD_L14: c_uint = 0x1202e4;
// [RW 10] Bandwidth Typical L for VQ15 Read requests
pub const PXP2_REG_RQ_BW_RD_L15: c_uint = 0x1202e8;
// [RW 10] Bandwidth Typical L for VQ16 Read requests
pub const PXP2_REG_RQ_BW_RD_L16: c_uint = 0x1202ec;
// [RW 10] Bandwidth Typical L for VQ17 Read requests
pub const PXP2_REG_RQ_BW_RD_L17: c_uint = 0x1202f0;
// [RW 10] Bandwidth Typical L for VQ18 Read requests
pub const PXP2_REG_RQ_BW_RD_L18: c_uint = 0x1202f4;
// [RW 10] Bandwidth Typical L for VQ19 Read requests
pub const PXP2_REG_RQ_BW_RD_L19: c_uint = 0x1202f8;
// [RW 10] Bandwidth Typical L for VQ20 Read requests
pub const PXP2_REG_RQ_BW_RD_L20: c_uint = 0x1202fc;
// [RW 10] Bandwidth Typical L for VQ22 Read requests
pub const PXP2_REG_RQ_BW_RD_L22: c_uint = 0x120300;
// [RW 10] Bandwidth Typical L for VQ23 Read requests
pub const PXP2_REG_RQ_BW_RD_L23: c_uint = 0x120304;
// [RW 10] Bandwidth Typical L for VQ24 Read requests
pub const PXP2_REG_RQ_BW_RD_L24: c_uint = 0x120308;
// [RW 10] Bandwidth Typical L for VQ25 Read requests
pub const PXP2_REG_RQ_BW_RD_L25: c_uint = 0x12030c;
// [RW 10] Bandwidth Typical L for VQ26 Read requests
pub const PXP2_REG_RQ_BW_RD_L26: c_uint = 0x120310;
// [RW 10] Bandwidth Typical L for VQ27 Read requests
pub const PXP2_REG_RQ_BW_RD_L27: c_uint = 0x120314;
// [RW 10] Bandwidth Typical L for VQ4 Read requests
pub const PXP2_REG_RQ_BW_RD_L4: c_uint = 0x1202bc;
// [RW 10] Bandwidth Typical L for VQ5 Read- currently not used
pub const PXP2_REG_RQ_BW_RD_L5: c_uint = 0x1202c0;
// [RW 7] Bandwidth upper bound for VQ0 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND0: c_uint = 0x120234;
// [RW 7] Bandwidth upper bound for VQ12 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND12: c_uint = 0x120264;
// [RW 7] Bandwidth upper bound for VQ13 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND13: c_uint = 0x120268;
// [RW 7] Bandwidth upper bound for VQ14 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND14: c_uint = 0x12026c;
// [RW 7] Bandwidth upper bound for VQ15 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND15: c_uint = 0x120270;
// [RW 7] Bandwidth upper bound for VQ16 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND16: c_uint = 0x120274;
// [RW 7] Bandwidth upper bound for VQ17 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND17: c_uint = 0x120278;
// [RW 7] Bandwidth upper bound for VQ18 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND18: c_uint = 0x12027c;
// [RW 7] Bandwidth upper bound for VQ19 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND19: c_uint = 0x120280;
// [RW 7] Bandwidth upper bound for VQ20 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND20: c_uint = 0x120284;
// [RW 7] Bandwidth upper bound for VQ22 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND22: c_uint = 0x120288;
// [RW 7] Bandwidth upper bound for VQ23 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND23: c_uint = 0x12028c;
// [RW 7] Bandwidth upper bound for VQ24 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND24: c_uint = 0x120290;
// [RW 7] Bandwidth upper bound for VQ25 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND25: c_uint = 0x120294;
// [RW 7] Bandwidth upper bound for VQ26 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND26: c_uint = 0x120298;
// [RW 7] Bandwidth upper bound for VQ27 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND27: c_uint = 0x12029c;
// [RW 7] Bandwidth upper bound for VQ4 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND4: c_uint = 0x120244;
// [RW 7] Bandwidth upper bound for VQ5 read requests
pub const PXP2_REG_RQ_BW_RD_UBOUND5: c_uint = 0x120248;
// [RW 10] Bandwidth addition to VQ29 write requests
pub const PXP2_REG_RQ_BW_WR_ADD29: c_uint = 0x12022c;
// [RW 10] Bandwidth addition to VQ30 write requests
pub const PXP2_REG_RQ_BW_WR_ADD30: c_uint = 0x120230;
// [RW 10] Bandwidth Typical L for VQ29 Write requests
pub const PXP2_REG_RQ_BW_WR_L29: c_uint = 0x12031c;
// [RW 10] Bandwidth Typical L for VQ30 Write requests
pub const PXP2_REG_RQ_BW_WR_L30: c_uint = 0x120320;
// [RW 7] Bandwidth upper bound for VQ29
pub const PXP2_REG_RQ_BW_WR_UBOUND29: c_uint = 0x1202a4;
// [RW 7] Bandwidth upper bound for VQ30
pub const PXP2_REG_RQ_BW_WR_UBOUND30: c_uint = 0x1202a8;
// [RW 18] external first_mem_addr field in L2P table for CDU module port 0
pub const PXP2_REG_RQ_CDU0_EFIRST_MEM_ADDR: c_uint = 0x120008;
// [RW 2] Endian mode for cdu
pub const PXP2_REG_RQ_CDU_ENDIAN_M: c_uint = 0x1201a0;
pub const PXP2_REG_RQ_CDU_FIRST_ILT: c_uint = 0x12061c;
pub const PXP2_REG_RQ_CDU_LAST_ILT: c_uint = 0x120620;
// [RW 3] page size in L2P table for CDU module; -4k; -8k; -16k; -32k; -64k;
pub const PXP2_REG_RQ_CDU_P_SIZE: c_uint = 0x120018;
// [R 1] 1' indicates that the requester has finished its internal
pub const PXP2_REG_RQ_CFG_DONE: c_uint = 0x1201b4;
// [RW 2] Endian mode for debug
pub const PXP2_REG_RQ_DBG_ENDIAN_M: c_uint = 0x1201a4;
// [RW 1] When '1'; requests will enter input buffers but wont get out
pub const PXP2_REG_RQ_DISABLE_INPUTS: c_uint = 0x120330;
// [RW 4] Determines alignment of write SRs when a request is split into
// several SRs. 0 - 8B aligned. 1 - 64B aligned. 2 - 128B aligned. 3 - 256B
// aligned. 4 - 512B aligned.
pub const PXP2_REG_RQ_DRAM_ALIGN: c_uint = 0x1205b0;
// [RW 4] Determines alignment of read SRs when a request is split into
// several SRs. 0 - 8B aligned. 1 - 64B aligned. 2 - 128B aligned. 3 - 256B
// aligned. 4 - 512B aligned.
pub const PXP2_REG_RQ_DRAM_ALIGN_RD: c_uint = 0x12092c;
// [RW 1] when set the new alignment method (E2) will be applied; when reset
// the original alignment method (E1 E1H) will be applied
pub const PXP2_REG_RQ_DRAM_ALIGN_SEL: c_uint = 0x120930;
// [RW 1] If 1 ILT failiue will not result in ELT access; An interrupt will
pub const PXP2_REG_RQ_ELT_DISABLE: c_uint = 0x12066c;
// [RW 2] Endian mode for hc
pub const PXP2_REG_RQ_HC_ENDIAN_M: c_uint = 0x1201a8;
// [RW 1] when '0' ILT logic will work as in A0; otherwise B0; for back
pub const PXP2_REG_RQ_ILT_MODE: c_uint = 0x1205b4;
// [WB 53] Onchip address table
pub const PXP2_REG_RQ_ONCHIP_AT: c_uint = 0x122000;
// [WB 53] Onchip address table - B0
pub const PXP2_REG_RQ_ONCHIP_AT_B0: c_uint = 0x128000;
// [RW 13] Pending read limiter threshold; in Dwords
pub const PXP2_REG_RQ_PDR_LIMIT: c_uint = 0x12033c;
// [RW 2] Endian mode for qm
pub const PXP2_REG_RQ_QM_ENDIAN_M: c_uint = 0x120194;
pub const PXP2_REG_RQ_QM_FIRST_ILT: c_uint = 0x120634;
pub const PXP2_REG_RQ_QM_LAST_ILT: c_uint = 0x120638;
// [RW 3] page size in L2P table for QM module; -4k; -8k; -16k; -32k; -64k;
pub const PXP2_REG_RQ_QM_P_SIZE: c_uint = 0x120050;
// [RW 1] 1' indicates that the RBC has finished configuring the PSWRQ
pub const PXP2_REG_RQ_RBC_DONE: c_uint = 0x1201b0;
// [RW 3] Max burst size filed for read requests port 0; 000 - 128B;
pub const PXP2_REG_RQ_RD_MBS0: c_uint = 0x120160;
// [RW 3] Max burst size filed for read requests port 1; 000 - 128B;
pub const PXP2_REG_RQ_RD_MBS1: c_uint = 0x120168;
// [RW 2] Endian mode for src
pub const PXP2_REG_RQ_SRC_ENDIAN_M: c_uint = 0x12019c;
pub const PXP2_REG_RQ_SRC_FIRST_ILT: c_uint = 0x12063c;
pub const PXP2_REG_RQ_SRC_LAST_ILT: c_uint = 0x120640;
// [RW 3] page size in L2P table for SRC module; -4k; -8k; -16k; -32k; -64k;
pub const PXP2_REG_RQ_SRC_P_SIZE: c_uint = 0x12006c;
// [RW 2] Endian mode for tm
pub const PXP2_REG_RQ_TM_ENDIAN_M: c_uint = 0x120198;
pub const PXP2_REG_RQ_TM_FIRST_ILT: c_uint = 0x120644;
pub const PXP2_REG_RQ_TM_LAST_ILT: c_uint = 0x120648;
// [RW 3] page size in L2P table for TM module; -4k; -8k; -16k; -32k; -64k;
pub const PXP2_REG_RQ_TM_P_SIZE: c_uint = 0x120034;
// [R 5] Number of entries in the ufifo; his fifo has l2p completions
pub const PXP2_REG_RQ_UFIFO_NUM_OF_ENTRY: c_uint = 0x12080c;
// [RW 18] external first_mem_addr field in L2P table for USDM module port 0
pub const PXP2_REG_RQ_USDM0_EFIRST_MEM_ADDR: c_uint = 0x120094;
// [R 8] Number of entries occupied by vq 0 in pswrq memory
pub const PXP2_REG_RQ_VQ0_ENTRY_CNT: c_uint = 0x120810;
// [R 8] Number of entries occupied by vq 10 in pswrq memory
pub const PXP2_REG_RQ_VQ10_ENTRY_CNT: c_uint = 0x120818;
// [R 8] Number of entries occupied by vq 11 in pswrq memory
pub const PXP2_REG_RQ_VQ11_ENTRY_CNT: c_uint = 0x120820;
// [R 8] Number of entries occupied by vq 12 in pswrq memory
pub const PXP2_REG_RQ_VQ12_ENTRY_CNT: c_uint = 0x120828;
// [R 8] Number of entries occupied by vq 13 in pswrq memory
pub const PXP2_REG_RQ_VQ13_ENTRY_CNT: c_uint = 0x120830;
// [R 8] Number of entries occupied by vq 14 in pswrq memory
pub const PXP2_REG_RQ_VQ14_ENTRY_CNT: c_uint = 0x120838;
// [R 8] Number of entries occupied by vq 15 in pswrq memory
pub const PXP2_REG_RQ_VQ15_ENTRY_CNT: c_uint = 0x120840;
// [R 8] Number of entries occupied by vq 16 in pswrq memory
pub const PXP2_REG_RQ_VQ16_ENTRY_CNT: c_uint = 0x120848;
// [R 8] Number of entries occupied by vq 17 in pswrq memory
pub const PXP2_REG_RQ_VQ17_ENTRY_CNT: c_uint = 0x120850;
// [R 8] Number of entries occupied by vq 18 in pswrq memory
pub const PXP2_REG_RQ_VQ18_ENTRY_CNT: c_uint = 0x120858;
// [R 8] Number of entries occupied by vq 19 in pswrq memory
pub const PXP2_REG_RQ_VQ19_ENTRY_CNT: c_uint = 0x120860;
// [R 8] Number of entries occupied by vq 1 in pswrq memory
pub const PXP2_REG_RQ_VQ1_ENTRY_CNT: c_uint = 0x120868;
// [R 8] Number of entries occupied by vq 20 in pswrq memory
pub const PXP2_REG_RQ_VQ20_ENTRY_CNT: c_uint = 0x120870;
// [R 8] Number of entries occupied by vq 21 in pswrq memory
pub const PXP2_REG_RQ_VQ21_ENTRY_CNT: c_uint = 0x120878;
// [R 8] Number of entries occupied by vq 22 in pswrq memory
pub const PXP2_REG_RQ_VQ22_ENTRY_CNT: c_uint = 0x120880;
// [R 8] Number of entries occupied by vq 23 in pswrq memory
pub const PXP2_REG_RQ_VQ23_ENTRY_CNT: c_uint = 0x120888;
// [R 8] Number of entries occupied by vq 24 in pswrq memory
pub const PXP2_REG_RQ_VQ24_ENTRY_CNT: c_uint = 0x120890;
// [R 8] Number of entries occupied by vq 25 in pswrq memory
pub const PXP2_REG_RQ_VQ25_ENTRY_CNT: c_uint = 0x120898;
// [R 8] Number of entries occupied by vq 26 in pswrq memory
pub const PXP2_REG_RQ_VQ26_ENTRY_CNT: c_uint = 0x1208a0;
// [R 8] Number of entries occupied by vq 27 in pswrq memory
pub const PXP2_REG_RQ_VQ27_ENTRY_CNT: c_uint = 0x1208a8;
// [R 8] Number of entries occupied by vq 28 in pswrq memory
pub const PXP2_REG_RQ_VQ28_ENTRY_CNT: c_uint = 0x1208b0;
// [R 8] Number of entries occupied by vq 29 in pswrq memory
pub const PXP2_REG_RQ_VQ29_ENTRY_CNT: c_uint = 0x1208b8;
// [R 8] Number of entries occupied by vq 2 in pswrq memory
pub const PXP2_REG_RQ_VQ2_ENTRY_CNT: c_uint = 0x1208c0;
// [R 8] Number of entries occupied by vq 30 in pswrq memory
pub const PXP2_REG_RQ_VQ30_ENTRY_CNT: c_uint = 0x1208c8;
// [R 8] Number of entries occupied by vq 31 in pswrq memory
pub const PXP2_REG_RQ_VQ31_ENTRY_CNT: c_uint = 0x1208d0;
// [R 8] Number of entries occupied by vq 3 in pswrq memory
pub const PXP2_REG_RQ_VQ3_ENTRY_CNT: c_uint = 0x1208d8;
// [R 8] Number of entries occupied by vq 4 in pswrq memory
pub const PXP2_REG_RQ_VQ4_ENTRY_CNT: c_uint = 0x1208e0;
// [R 8] Number of entries occupied by vq 5 in pswrq memory
pub const PXP2_REG_RQ_VQ5_ENTRY_CNT: c_uint = 0x1208e8;
// [R 8] Number of entries occupied by vq 6 in pswrq memory
pub const PXP2_REG_RQ_VQ6_ENTRY_CNT: c_uint = 0x1208f0;
// [R 8] Number of entries occupied by vq 7 in pswrq memory
pub const PXP2_REG_RQ_VQ7_ENTRY_CNT: c_uint = 0x1208f8;
// [R 8] Number of entries occupied by vq 8 in pswrq memory
pub const PXP2_REG_RQ_VQ8_ENTRY_CNT: c_uint = 0x120900;
// [R 8] Number of entries occupied by vq 9 in pswrq memory
pub const PXP2_REG_RQ_VQ9_ENTRY_CNT: c_uint = 0x120908;
// [RW 3] Max burst size filed for write requests port 0; 000 - 128B;
pub const PXP2_REG_RQ_WR_MBS0: c_uint = 0x12015c;
// [RW 3] Max burst size filed for write requests port 1; 000 - 128B;
pub const PXP2_REG_RQ_WR_MBS1: c_uint = 0x120164;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_CDU_MPS: c_uint = 0x1205f0;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_CSDM_MPS: c_uint = 0x1205d0;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_DBG_MPS: c_uint = 0x1205e8;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_DMAE_MPS: c_uint = 0x1205ec;
// [RW 10] if Number of entries in dmae fifo will be higher than this
pub const PXP2_REG_WR_DMAE_TH: c_uint = 0x120368;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_HC_MPS: c_uint = 0x1205c8;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_QM_MPS: c_uint = 0x1205dc;
// [RW 1] 0 - working in A0 mode;  - working in B0 mode
pub const PXP2_REG_WR_REV_MODE: c_uint = 0x120670;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_SRC_MPS: c_uint = 0x1205e4;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_TM_MPS: c_uint = 0x1205e0;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_TSDM_MPS: c_uint = 0x1205d4;
// [RW 10] if Number of entries in usdmdp fifo will be higher than this
pub const PXP2_REG_WR_USDMDP_TH: c_uint = 0x120348;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_USDM_MPS: c_uint = 0x1205cc;
// [RW 2] 0 - 128B;  - 256B;  - 512B;  - 1024B; when the payload in the
pub const PXP2_REG_WR_XSDM_MPS: c_uint = 0x1205d8;
// [R 1] debug only: Indication if PSWHST arbiter is idle
pub const PXP_REG_HST_ARB_IS_IDLE: c_uint = 0x103004;
// [R 8] debug only: A bit mask for all PSWHST arbiter clients. '1' means
pub const PXP_REG_HST_CLIENTS_WAITING_TO_ARB: c_uint = 0x103008;
// [RW 1] When 1; doorbells are discarded and not passed to doorbell queue
pub const PXP_REG_HST_DISCARD_DOORBELLS: c_uint = 0x1030a4;
// [R 1] debug only: '1' means this PSWHST is discarding doorbells. This bit
pub const PXP_REG_HST_DISCARD_DOORBELLS_STATUS: c_uint = 0x1030a0;
// [RW 1] When 1; new internal writes arriving to the block are discarded.
pub const PXP_REG_HST_DISCARD_INTERNAL_WRITES: c_uint = 0x1030a8;
// [R 6] debug only: A bit mask for all PSWHST internal write clients. '1'
pub const PXP_REG_HST_DISCARD_INTERNAL_WRITES_STATUS: c_uint = 0x10309c;
// [WB 160] Used for initialization of the inbound interrupts memory
pub const PXP_REG_HST_INBOUND_INT: c_uint = 0x103800;
// [RW 7] Indirect access to the permission table. The fields are : {Valid;
// VFID[5:0]}
//
pub const PXP_REG_HST_ZONE_PERMISSION_TABLE: c_uint = 0x103400;
// [RW 32] Interrupt mask register #0 read/write
pub const PXP_REG_PXP_INT_MASK_0: c_uint = 0x103074;
pub const PXP_REG_PXP_INT_MASK_1: c_uint = 0x103084;
// [R 32] Interrupt register #0 read
pub const PXP_REG_PXP_INT_STS_0: c_uint = 0x103068;
pub const PXP_REG_PXP_INT_STS_1: c_uint = 0x103078;
// [RC 32] Interrupt register #0 read clear
pub const PXP_REG_PXP_INT_STS_CLR_0: c_uint = 0x10306c;
pub const PXP_REG_PXP_INT_STS_CLR_1: c_uint = 0x10307c;
// [RW 27] Parity mask register #0 read/write
pub const PXP_REG_PXP_PRTY_MASK: c_uint = 0x103094;
// [R 26] Parity register #0 read
pub const PXP_REG_PXP_PRTY_STS: c_uint = 0x103088;
// [RC 27] Parity register #0 read clear
pub const PXP_REG_PXP_PRTY_STS_CLR: c_uint = 0x10308c;
// [RW 4] The activity counter initial increment value sent in the load
pub const QM_REG_ACTCTRINITVAL_0: c_uint = 0x168040;
pub const QM_REG_ACTCTRINITVAL_1: c_uint = 0x168044;
pub const QM_REG_ACTCTRINITVAL_2: c_uint = 0x168048;
pub const QM_REG_ACTCTRINITVAL_3: c_uint = 0x16804c;
// [RW 32] The base logical address (in bytes) of each physical queue. The
pub const QM_REG_BASEADDR: c_uint = 0x168900;
// [RW 32] The base logical address (in bytes) of each physical queue. The
pub const QM_REG_BASEADDR_EXT_A: c_uint = 0x16e100;
// [RW 16] The byte credit cost for each task. This value is for both ports
pub const QM_REG_BYTECRDCOST: c_uint = 0x168234;
// [RW 16] The initial byte credit value for both ports.
pub const QM_REG_BYTECRDINITVAL: c_uint = 0x168238;
// [RW 32] A bit per physical queue. If the bit is cleared then the physical
pub const QM_REG_BYTECRDPORT_LSB: c_uint = 0x168228;
// [RW 32] A bit per physical queue. If the bit is cleared then the physical
pub const QM_REG_BYTECRDPORT_LSB_EXT_A: c_uint = 0x16e520;
// [RW 32] A bit per physical queue. If the bit is cleared then the physical
pub const QM_REG_BYTECRDPORT_MSB: c_uint = 0x168224;
// [RW 32] A bit per physical queue. If the bit is cleared then the physical
pub const QM_REG_BYTECRDPORT_MSB_EXT_A: c_uint = 0x16e51c;
// [RW 16] The byte credit value that if above the QM is considered almost
pub const QM_REG_BYTECREDITAFULLTHR: c_uint = 0x168094;
// [RW 4] The initial credit for interface
pub const QM_REG_CMINITCRD_0: c_uint = 0x1680cc;
pub const QM_REG_BYTECRDCMDQ_0: c_uint = 0x16e6e8;
pub const QM_REG_CMINITCRD_1: c_uint = 0x1680d0;
pub const QM_REG_CMINITCRD_2: c_uint = 0x1680d4;
pub const QM_REG_CMINITCRD_3: c_uint = 0x1680d8;
pub const QM_REG_CMINITCRD_4: c_uint = 0x1680dc;
pub const QM_REG_CMINITCRD_5: c_uint = 0x1680e0;
pub const QM_REG_CMINITCRD_6: c_uint = 0x1680e4;
pub const QM_REG_CMINITCRD_7: c_uint = 0x1680e8;
// [RW 8] A mask bit per CM interface. If this bit is 0 then this interface
pub const QM_REG_CMINTEN: c_uint = 0x1680ec;
// [RW 12] A bit vector which indicates which one of the queues are tied to
pub const QM_REG_CMINTVOQMASK_0: c_uint = 0x1681f4;
pub const QM_REG_CMINTVOQMASK_1: c_uint = 0x1681f8;
pub const QM_REG_CMINTVOQMASK_2: c_uint = 0x1681fc;
pub const QM_REG_CMINTVOQMASK_3: c_uint = 0x168200;
pub const QM_REG_CMINTVOQMASK_4: c_uint = 0x168204;
pub const QM_REG_CMINTVOQMASK_5: c_uint = 0x168208;
pub const QM_REG_CMINTVOQMASK_6: c_uint = 0x16820c;
pub const QM_REG_CMINTVOQMASK_7: c_uint = 0x168210;
// [RW 20] The number of connections divided by 16 which dictates the size
pub const QM_REG_CONNNUM_0: c_uint = 0x168020;
// [R 6] Keep the fill level of the fifo from write client 4
pub const QM_REG_CQM_WRC_FIFOLVL: c_uint = 0x168018;
// [RW 8] The context regions sent in the CFC load request
pub const QM_REG_CTXREG_0: c_uint = 0x168030;
pub const QM_REG_CTXREG_1: c_uint = 0x168034;
pub const QM_REG_CTXREG_2: c_uint = 0x168038;
pub const QM_REG_CTXREG_3: c_uint = 0x16803c;
// [RW 12] The VOQ mask used to select the VOQs which needs to be full for
pub const QM_REG_ENBYPVOQMASK: c_uint = 0x16823c;
// [RW 32] A bit mask per each physical queue. If a bit is set then the
pub const QM_REG_ENBYTECRD_LSB: c_uint = 0x168220;
// [RW 32] A bit mask per each physical queue. If a bit is set then the
pub const QM_REG_ENBYTECRD_LSB_EXT_A: c_uint = 0x16e518;
// [RW 32] A bit mask per each physical queue. If a bit is set then the
pub const QM_REG_ENBYTECRD_MSB: c_uint = 0x16821c;
// [RW 32] A bit mask per each physical queue. If a bit is set then the
pub const QM_REG_ENBYTECRD_MSB_EXT_A: c_uint = 0x16e514;
// [RW 4] If cleared then the secondary interface will not be served by the
pub const QM_REG_ENSEC: c_uint = 0x1680f0;
// [RW 32] NA
pub const QM_REG_FUNCNUMSEL_LSB: c_uint = 0x168230;
// [RW 32] NA
pub const QM_REG_FUNCNUMSEL_MSB: c_uint = 0x16822c;
// [RW 32] A mask register to mask the Almost empty signals which will not
pub const QM_REG_HWAEMPTYMASK_LSB: c_uint = 0x168218;
// [RW 32] A mask register to mask the Almost empty signals which will not
pub const QM_REG_HWAEMPTYMASK_LSB_EXT_A: c_uint = 0x16e510;
// [RW 32] A mask register to mask the Almost empty signals which will not
pub const QM_REG_HWAEMPTYMASK_MSB: c_uint = 0x168214;
// [RW 32] A mask register to mask the Almost empty signals which will not
pub const QM_REG_HWAEMPTYMASK_MSB_EXT_A: c_uint = 0x16e50c;
// [RW 4] The number of outstanding request to CFC
pub const QM_REG_OUTLDREQ: c_uint = 0x168804;
// [RC 1] A flag to indicate that overflow error occurred in one of the
pub const QM_REG_OVFERROR: c_uint = 0x16805c;
// [RC 7] the Q where the overflow occurs
pub const QM_REG_OVFQNUM: c_uint = 0x168058;
// [R 16] Pause state for physical queues 15-0
pub const QM_REG_PAUSESTATE0: c_uint = 0x168410;
// [R 16] Pause state for physical queues 31-16
pub const QM_REG_PAUSESTATE1: c_uint = 0x168414;
// [R 16] Pause state for physical queues 47-32
pub const QM_REG_PAUSESTATE2: c_uint = 0x16e684;
// [R 16] Pause state for physical queues 63-48
pub const QM_REG_PAUSESTATE3: c_uint = 0x16e688;
// [R 16] Pause state for physical queues 79-64
pub const QM_REG_PAUSESTATE4: c_uint = 0x16e68c;
// [R 16] Pause state for physical queues 95-80
pub const QM_REG_PAUSESTATE5: c_uint = 0x16e690;
// [R 16] Pause state for physical queues 111-96
pub const QM_REG_PAUSESTATE6: c_uint = 0x16e694;
// [R 16] Pause state for physical queues 127-112
pub const QM_REG_PAUSESTATE7: c_uint = 0x16e698;
// [RW 2] The PCI attributes field used in the PCI request.
pub const QM_REG_PCIREQAT: c_uint = 0x168054;
pub const QM_REG_PF_EN: c_uint = 0x16e70c;
// [R 24] The number of tasks stored in the QM for the PF. only even
// functions are valid in E2 (odd I registers will be hard wired to 0)
pub const QM_REG_PF_USG_CNT_0: c_uint = 0x16e040;
// [R 16] NOT USED
pub const QM_REG_PORT0BYTECRD: c_uint = 0x168300;
// [R 16] The byte credit of port 1
pub const QM_REG_PORT1BYTECRD: c_uint = 0x168304;
// [RW 3] pci function number of queues 15-0
pub const QM_REG_PQ2PCIFUNC_0: c_uint = 0x16e6bc;
pub const QM_REG_PQ2PCIFUNC_1: c_uint = 0x16e6c0;
pub const QM_REG_PQ2PCIFUNC_2: c_uint = 0x16e6c4;
pub const QM_REG_PQ2PCIFUNC_3: c_uint = 0x16e6c8;
pub const QM_REG_PQ2PCIFUNC_4: c_uint = 0x16e6cc;
pub const QM_REG_PQ2PCIFUNC_5: c_uint = 0x16e6d0;
pub const QM_REG_PQ2PCIFUNC_6: c_uint = 0x16e6d4;
pub const QM_REG_PQ2PCIFUNC_7: c_uint = 0x16e6d8;
// [WB 54] Pointer Table Memory for queues 63-0; The mapping is as follow:
pub const QM_REG_PTRTBL: c_uint = 0x168a00;
// [WB 54] Pointer Table Memory for queues 127-64; The mapping is as follow:
pub const QM_REG_PTRTBL_EXT_A: c_uint = 0x16e200;
// [RW 2] Interrupt mask register #0 read/write
pub const QM_REG_QM_INT_MASK: c_uint = 0x168444;
// [R 2] Interrupt register #0 read
pub const QM_REG_QM_INT_STS: c_uint = 0x168438;
// [RW 12] Parity mask register #0 read/write
pub const QM_REG_QM_PRTY_MASK: c_uint = 0x168454;
// [R 12] Parity register #0 read
pub const QM_REG_QM_PRTY_STS: c_uint = 0x168448;
// [RC 12] Parity register #0 read clear
pub const QM_REG_QM_PRTY_STS_CLR: c_uint = 0x16844c;
// [R 32] Current queues in pipeline: Queues from 32 to 63
pub const QM_REG_QSTATUS_HIGH: c_uint = 0x16802c;
// [R 32] Current queues in pipeline: Queues from 96 to 127
pub const QM_REG_QSTATUS_HIGH_EXT_A: c_uint = 0x16e408;
// [R 32] Current queues in pipeline: Queues from 0 to 31
pub const QM_REG_QSTATUS_LOW: c_uint = 0x168028;
// [R 32] Current queues in pipeline: Queues from 64 to 95
pub const QM_REG_QSTATUS_LOW_EXT_A: c_uint = 0x16e404;
// [R 24] The number of tasks queued for each queue; queues 63-0
pub const QM_REG_QTASKCTR_0: c_uint = 0x168308;
// [R 24] The number of tasks queued for each queue; queues 127-64
pub const QM_REG_QTASKCTR_EXT_A_0: c_uint = 0x16e584;
// [RW 4] Queue tied to VOQ
pub const QM_REG_QVOQIDX_0: c_uint = 0x1680f4;
pub const QM_REG_QVOQIDX_10: c_uint = 0x16811c;
pub const QM_REG_QVOQIDX_100: c_uint = 0x16e49c;
pub const QM_REG_QVOQIDX_101: c_uint = 0x16e4a0;
pub const QM_REG_QVOQIDX_102: c_uint = 0x16e4a4;
pub const QM_REG_QVOQIDX_103: c_uint = 0x16e4a8;
pub const QM_REG_QVOQIDX_104: c_uint = 0x16e4ac;
pub const QM_REG_QVOQIDX_105: c_uint = 0x16e4b0;
pub const QM_REG_QVOQIDX_106: c_uint = 0x16e4b4;
pub const QM_REG_QVOQIDX_107: c_uint = 0x16e4b8;
pub const QM_REG_QVOQIDX_108: c_uint = 0x16e4bc;
pub const QM_REG_QVOQIDX_109: c_uint = 0x16e4c0;
pub const QM_REG_QVOQIDX_11: c_uint = 0x168120;
pub const QM_REG_QVOQIDX_110: c_uint = 0x16e4c4;
pub const QM_REG_QVOQIDX_111: c_uint = 0x16e4c8;
pub const QM_REG_QVOQIDX_112: c_uint = 0x16e4cc;
pub const QM_REG_QVOQIDX_113: c_uint = 0x16e4d0;
pub const QM_REG_QVOQIDX_114: c_uint = 0x16e4d4;
pub const QM_REG_QVOQIDX_115: c_uint = 0x16e4d8;
pub const QM_REG_QVOQIDX_116: c_uint = 0x16e4dc;
pub const QM_REG_QVOQIDX_117: c_uint = 0x16e4e0;
pub const QM_REG_QVOQIDX_118: c_uint = 0x16e4e4;
pub const QM_REG_QVOQIDX_119: c_uint = 0x16e4e8;
pub const QM_REG_QVOQIDX_12: c_uint = 0x168124;
pub const QM_REG_QVOQIDX_120: c_uint = 0x16e4ec;
pub const QM_REG_QVOQIDX_121: c_uint = 0x16e4f0;
pub const QM_REG_QVOQIDX_122: c_uint = 0x16e4f4;
pub const QM_REG_QVOQIDX_123: c_uint = 0x16e4f8;
pub const QM_REG_QVOQIDX_124: c_uint = 0x16e4fc;
pub const QM_REG_QVOQIDX_125: c_uint = 0x16e500;
pub const QM_REG_QVOQIDX_126: c_uint = 0x16e504;
pub const QM_REG_QVOQIDX_127: c_uint = 0x16e508;
pub const QM_REG_QVOQIDX_13: c_uint = 0x168128;
pub const QM_REG_QVOQIDX_14: c_uint = 0x16812c;
pub const QM_REG_QVOQIDX_15: c_uint = 0x168130;
pub const QM_REG_QVOQIDX_16: c_uint = 0x168134;
pub const QM_REG_QVOQIDX_17: c_uint = 0x168138;
pub const QM_REG_QVOQIDX_21: c_uint = 0x168148;
pub const QM_REG_QVOQIDX_22: c_uint = 0x16814c;
pub const QM_REG_QVOQIDX_23: c_uint = 0x168150;
pub const QM_REG_QVOQIDX_24: c_uint = 0x168154;
pub const QM_REG_QVOQIDX_25: c_uint = 0x168158;
pub const QM_REG_QVOQIDX_26: c_uint = 0x16815c;
pub const QM_REG_QVOQIDX_27: c_uint = 0x168160;
pub const QM_REG_QVOQIDX_28: c_uint = 0x168164;
pub const QM_REG_QVOQIDX_29: c_uint = 0x168168;
pub const QM_REG_QVOQIDX_30: c_uint = 0x16816c;
pub const QM_REG_QVOQIDX_31: c_uint = 0x168170;
pub const QM_REG_QVOQIDX_32: c_uint = 0x168174;
pub const QM_REG_QVOQIDX_33: c_uint = 0x168178;
pub const QM_REG_QVOQIDX_34: c_uint = 0x16817c;
pub const QM_REG_QVOQIDX_35: c_uint = 0x168180;
pub const QM_REG_QVOQIDX_36: c_uint = 0x168184;
pub const QM_REG_QVOQIDX_37: c_uint = 0x168188;
pub const QM_REG_QVOQIDX_38: c_uint = 0x16818c;
pub const QM_REG_QVOQIDX_39: c_uint = 0x168190;
pub const QM_REG_QVOQIDX_40: c_uint = 0x168194;
pub const QM_REG_QVOQIDX_41: c_uint = 0x168198;
pub const QM_REG_QVOQIDX_42: c_uint = 0x16819c;
pub const QM_REG_QVOQIDX_43: c_uint = 0x1681a0;
pub const QM_REG_QVOQIDX_44: c_uint = 0x1681a4;
pub const QM_REG_QVOQIDX_45: c_uint = 0x1681a8;
pub const QM_REG_QVOQIDX_46: c_uint = 0x1681ac;
pub const QM_REG_QVOQIDX_47: c_uint = 0x1681b0;
pub const QM_REG_QVOQIDX_48: c_uint = 0x1681b4;
pub const QM_REG_QVOQIDX_49: c_uint = 0x1681b8;
pub const QM_REG_QVOQIDX_5: c_uint = 0x168108;
pub const QM_REG_QVOQIDX_50: c_uint = 0x1681bc;
pub const QM_REG_QVOQIDX_51: c_uint = 0x1681c0;
pub const QM_REG_QVOQIDX_52: c_uint = 0x1681c4;
pub const QM_REG_QVOQIDX_53: c_uint = 0x1681c8;
pub const QM_REG_QVOQIDX_54: c_uint = 0x1681cc;
pub const QM_REG_QVOQIDX_55: c_uint = 0x1681d0;
pub const QM_REG_QVOQIDX_56: c_uint = 0x1681d4;
pub const QM_REG_QVOQIDX_57: c_uint = 0x1681d8;
pub const QM_REG_QVOQIDX_58: c_uint = 0x1681dc;
pub const QM_REG_QVOQIDX_59: c_uint = 0x1681e0;
pub const QM_REG_QVOQIDX_6: c_uint = 0x16810c;
pub const QM_REG_QVOQIDX_60: c_uint = 0x1681e4;
pub const QM_REG_QVOQIDX_61: c_uint = 0x1681e8;
pub const QM_REG_QVOQIDX_62: c_uint = 0x1681ec;
pub const QM_REG_QVOQIDX_63: c_uint = 0x1681f0;
pub const QM_REG_QVOQIDX_64: c_uint = 0x16e40c;
pub const QM_REG_QVOQIDX_65: c_uint = 0x16e410;
pub const QM_REG_QVOQIDX_69: c_uint = 0x16e420;
pub const QM_REG_QVOQIDX_7: c_uint = 0x168110;
pub const QM_REG_QVOQIDX_70: c_uint = 0x16e424;
pub const QM_REG_QVOQIDX_71: c_uint = 0x16e428;
pub const QM_REG_QVOQIDX_72: c_uint = 0x16e42c;
pub const QM_REG_QVOQIDX_73: c_uint = 0x16e430;
pub const QM_REG_QVOQIDX_74: c_uint = 0x16e434;
pub const QM_REG_QVOQIDX_75: c_uint = 0x16e438;
pub const QM_REG_QVOQIDX_76: c_uint = 0x16e43c;
pub const QM_REG_QVOQIDX_77: c_uint = 0x16e440;
pub const QM_REG_QVOQIDX_78: c_uint = 0x16e444;
pub const QM_REG_QVOQIDX_79: c_uint = 0x16e448;
pub const QM_REG_QVOQIDX_8: c_uint = 0x168114;
pub const QM_REG_QVOQIDX_80: c_uint = 0x16e44c;
pub const QM_REG_QVOQIDX_81: c_uint = 0x16e450;
pub const QM_REG_QVOQIDX_85: c_uint = 0x16e460;
pub const QM_REG_QVOQIDX_86: c_uint = 0x16e464;
pub const QM_REG_QVOQIDX_87: c_uint = 0x16e468;
pub const QM_REG_QVOQIDX_88: c_uint = 0x16e46c;
pub const QM_REG_QVOQIDX_89: c_uint = 0x16e470;
pub const QM_REG_QVOQIDX_9: c_uint = 0x168118;
pub const QM_REG_QVOQIDX_90: c_uint = 0x16e474;
pub const QM_REG_QVOQIDX_91: c_uint = 0x16e478;
pub const QM_REG_QVOQIDX_92: c_uint = 0x16e47c;
pub const QM_REG_QVOQIDX_93: c_uint = 0x16e480;
pub const QM_REG_QVOQIDX_94: c_uint = 0x16e484;
pub const QM_REG_QVOQIDX_95: c_uint = 0x16e488;
pub const QM_REG_QVOQIDX_96: c_uint = 0x16e48c;
pub const QM_REG_QVOQIDX_97: c_uint = 0x16e490;
pub const QM_REG_QVOQIDX_98: c_uint = 0x16e494;
pub const QM_REG_QVOQIDX_99: c_uint = 0x16e498;
// [RW 1] Initialization bit command
pub const QM_REG_SOFT_RESET: c_uint = 0x168428;
// [RW 8] The credit cost per every task in the QM. A value per each VOQ
pub const QM_REG_TASKCRDCOST_0: c_uint = 0x16809c;
pub const QM_REG_TASKCRDCOST_1: c_uint = 0x1680a0;
pub const QM_REG_TASKCRDCOST_2: c_uint = 0x1680a4;
pub const QM_REG_TASKCRDCOST_4: c_uint = 0x1680ac;
pub const QM_REG_TASKCRDCOST_5: c_uint = 0x1680b0;
// [R 6] Keep the fill level of the fifo from write client 3
pub const QM_REG_TQM_WRC_FIFOLVL: c_uint = 0x168010;
// [R 6] Keep the fill level of the fifo from write client 2
pub const QM_REG_UQM_WRC_FIFOLVL: c_uint = 0x168008;
// [RC 32] Credit update error register
pub const QM_REG_VOQCRDERRREG: c_uint = 0x168408;
// [R 16] The credit value for each VOQ
pub const QM_REG_VOQCREDIT_0: c_uint = 0x1682d0;
pub const QM_REG_VOQCREDIT_1: c_uint = 0x1682d4;
pub const QM_REG_VOQCREDIT_4: c_uint = 0x1682e0;
// [RW 16] The credit value that if above the QM is considered almost full
pub const QM_REG_VOQCREDITAFULLTHR: c_uint = 0x168090;
// [RW 16] The init and maximum credit for each VoQ
pub const QM_REG_VOQINITCREDIT_0: c_uint = 0x168060;
pub const QM_REG_VOQINITCREDIT_1: c_uint = 0x168064;
pub const QM_REG_VOQINITCREDIT_2: c_uint = 0x168068;
pub const QM_REG_VOQINITCREDIT_4: c_uint = 0x168070;
pub const QM_REG_VOQINITCREDIT_5: c_uint = 0x168074;
// [RW 1] The port of which VOQ belongs
pub const QM_REG_VOQPORT_0: c_uint = 0x1682a0;
pub const QM_REG_VOQPORT_1: c_uint = 0x1682a4;
pub const QM_REG_VOQPORT_2: c_uint = 0x1682a8;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_0_LSB: c_uint = 0x168240;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_0_LSB_EXT_A: c_uint = 0x16e524;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_0_MSB: c_uint = 0x168244;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_0_MSB_EXT_A: c_uint = 0x16e528;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_10_LSB: c_uint = 0x168290;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_10_LSB_EXT_A: c_uint = 0x16e574;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_10_MSB: c_uint = 0x168294;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_10_MSB_EXT_A: c_uint = 0x16e578;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_11_LSB: c_uint = 0x168298;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_11_LSB_EXT_A: c_uint = 0x16e57c;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_11_MSB: c_uint = 0x16829c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_11_MSB_EXT_A: c_uint = 0x16e580;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_1_LSB: c_uint = 0x168248;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_1_LSB_EXT_A: c_uint = 0x16e52c;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_1_MSB: c_uint = 0x16824c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_1_MSB_EXT_A: c_uint = 0x16e530;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_2_LSB: c_uint = 0x168250;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_2_LSB_EXT_A: c_uint = 0x16e534;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_2_MSB: c_uint = 0x168254;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_2_MSB_EXT_A: c_uint = 0x16e538;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_3_LSB: c_uint = 0x168258;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_3_LSB_EXT_A: c_uint = 0x16e53c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_3_MSB_EXT_A: c_uint = 0x16e540;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_4_LSB: c_uint = 0x168260;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_4_LSB_EXT_A: c_uint = 0x16e544;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_4_MSB: c_uint = 0x168264;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_4_MSB_EXT_A: c_uint = 0x16e548;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_5_LSB: c_uint = 0x168268;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_5_LSB_EXT_A: c_uint = 0x16e54c;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_5_MSB: c_uint = 0x16826c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_5_MSB_EXT_A: c_uint = 0x16e550;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_6_LSB: c_uint = 0x168270;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_6_LSB_EXT_A: c_uint = 0x16e554;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_6_MSB: c_uint = 0x168274;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_6_MSB_EXT_A: c_uint = 0x16e558;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_7_LSB: c_uint = 0x168278;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_7_LSB_EXT_A: c_uint = 0x16e55c;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_7_MSB: c_uint = 0x16827c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_7_MSB_EXT_A: c_uint = 0x16e560;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_8_LSB: c_uint = 0x168280;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_8_LSB_EXT_A: c_uint = 0x16e564;
// [RW 32] The physical queue number associated with each VOQ; queues 63-32
pub const QM_REG_VOQQMASK_8_MSB: c_uint = 0x168284;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_8_MSB_EXT_A: c_uint = 0x16e568;
// [RW 32] The physical queue number associated with each VOQ; queues 31-0
pub const QM_REG_VOQQMASK_9_LSB: c_uint = 0x168288;
// [RW 32] The physical queue number associated with each VOQ; queues 95-64
pub const QM_REG_VOQQMASK_9_LSB_EXT_A: c_uint = 0x16e56c;
// [RW 32] The physical queue number associated with each VOQ; queues 127-96
pub const QM_REG_VOQQMASK_9_MSB_EXT_A: c_uint = 0x16e570;
// [RW 32] Wrr weights
pub const QM_REG_WRRWEIGHTS_0: c_uint = 0x16880c;
pub const QM_REG_WRRWEIGHTS_1: c_uint = 0x168810;
pub const QM_REG_WRRWEIGHTS_10: c_uint = 0x168814;
pub const QM_REG_WRRWEIGHTS_11: c_uint = 0x168818;
pub const QM_REG_WRRWEIGHTS_12: c_uint = 0x16881c;
pub const QM_REG_WRRWEIGHTS_13: c_uint = 0x168820;
pub const QM_REG_WRRWEIGHTS_14: c_uint = 0x168824;
pub const QM_REG_WRRWEIGHTS_15: c_uint = 0x168828;
pub const QM_REG_WRRWEIGHTS_16: c_uint = 0x16e000;
pub const QM_REG_WRRWEIGHTS_17: c_uint = 0x16e004;
pub const QM_REG_WRRWEIGHTS_18: c_uint = 0x16e008;
pub const QM_REG_WRRWEIGHTS_19: c_uint = 0x16e00c;
pub const QM_REG_WRRWEIGHTS_2: c_uint = 0x16882c;
pub const QM_REG_WRRWEIGHTS_20: c_uint = 0x16e010;
pub const QM_REG_WRRWEIGHTS_21: c_uint = 0x16e014;
pub const QM_REG_WRRWEIGHTS_22: c_uint = 0x16e018;
pub const QM_REG_WRRWEIGHTS_23: c_uint = 0x16e01c;
pub const QM_REG_WRRWEIGHTS_24: c_uint = 0x16e020;
pub const QM_REG_WRRWEIGHTS_25: c_uint = 0x16e024;
pub const QM_REG_WRRWEIGHTS_26: c_uint = 0x16e028;
pub const QM_REG_WRRWEIGHTS_27: c_uint = 0x16e02c;
pub const QM_REG_WRRWEIGHTS_28: c_uint = 0x16e030;
pub const QM_REG_WRRWEIGHTS_29: c_uint = 0x16e034;
pub const QM_REG_WRRWEIGHTS_3: c_uint = 0x168830;
pub const QM_REG_WRRWEIGHTS_30: c_uint = 0x16e038;
pub const QM_REG_WRRWEIGHTS_31: c_uint = 0x16e03c;
pub const QM_REG_WRRWEIGHTS_4: c_uint = 0x168834;
pub const QM_REG_WRRWEIGHTS_5: c_uint = 0x168838;
pub const QM_REG_WRRWEIGHTS_6: c_uint = 0x16883c;
pub const QM_REG_WRRWEIGHTS_7: c_uint = 0x168840;
pub const QM_REG_WRRWEIGHTS_8: c_uint = 0x168844;
pub const QM_REG_WRRWEIGHTS_9: c_uint = 0x168848;
// [R 6] Keep the fill level of the fifo from write client 1
pub const QM_REG_XQM_WRC_FIFOLVL: c_uint = 0x168000;
// [W 1] reset to parity interrupt
pub const SEM_FAST_REG_PARITY_RST: c_uint = 0x18840;
pub const SRC_REG_COUNTFREE0: c_uint = 0x40500;
// [RW 1] If clr the searcher is compatible to E1 A0 - support only two
pub const SRC_REG_E1HMF_ENABLE: c_uint = 0x404cc;
pub const SRC_REG_FIRSTFREE0: c_uint = 0x40510;
pub const SRC_REG_KEYRSS0_0: c_uint = 0x40408;
pub const SRC_REG_KEYRSS0_7: c_uint = 0x40424;
pub const SRC_REG_KEYRSS1_9: c_uint = 0x40454;
pub const SRC_REG_KEYSEARCH_0: c_uint = 0x40458;
pub const SRC_REG_KEYSEARCH_1: c_uint = 0x4045c;
pub const SRC_REG_KEYSEARCH_2: c_uint = 0x40460;
pub const SRC_REG_KEYSEARCH_3: c_uint = 0x40464;
pub const SRC_REG_KEYSEARCH_4: c_uint = 0x40468;
pub const SRC_REG_KEYSEARCH_5: c_uint = 0x4046c;
pub const SRC_REG_KEYSEARCH_6: c_uint = 0x40470;
pub const SRC_REG_KEYSEARCH_7: c_uint = 0x40474;
pub const SRC_REG_KEYSEARCH_8: c_uint = 0x40478;
pub const SRC_REG_KEYSEARCH_9: c_uint = 0x4047c;
pub const SRC_REG_LASTFREE0: c_uint = 0x40530;
pub const SRC_REG_NUMBER_HASH_BITS0: c_uint = 0x40400;
// [RW 1] Reset internal state machines.
pub const SRC_REG_SOFT_RST: c_uint = 0x4049c;
// [R 3] Interrupt register #0 read
pub const SRC_REG_SRC_INT_STS: c_uint = 0x404ac;
// [RW 3] Parity mask register #0 read/write
pub const SRC_REG_SRC_PRTY_MASK: c_uint = 0x404c8;
// [R 3] Parity register #0 read
pub const SRC_REG_SRC_PRTY_STS: c_uint = 0x404bc;
// [RC 3] Parity register #0 read clear
pub const SRC_REG_SRC_PRTY_STS_CLR: c_uint = 0x404c0;
// [R 4] Used to read the value of the XX protection CAM occupancy counter.
pub const TCM_REG_CAM_OCCUP: c_uint = 0x5017c;
// [RW 1] CDU AG read Interface enable. If 0 - the request input is
pub const TCM_REG_CDU_AG_RD_IFEN: c_uint = 0x50034;
// [RW 1] CDU AG write Interface enable. If 0 - the request and valid input
pub const TCM_REG_CDU_AG_WR_IFEN: c_uint = 0x50030;
// [RW 1] CDU STORM read Interface enable. If 0 - the request input is
pub const TCM_REG_CDU_SM_RD_IFEN: c_uint = 0x5003c;
// [RW 1] CDU STORM write Interface enable. If 0 - the request and valid
pub const TCM_REG_CDU_SM_WR_IFEN: c_uint = 0x50038;
// [RW 4] CFC output initial credit. Max credit available - 15.Write writes
pub const TCM_REG_CFC_INIT_CRD: c_uint = 0x50204;
// [RW 3] The weight of the CP input in the WRR mechanism. 0 stands for
pub const TCM_REG_CP_WEIGHT: c_uint = 0x500c0;
// [RW 1] Input csem Interface enable. If 0 - the valid input is
pub const TCM_REG_CSEM_IFEN: c_uint = 0x5002c;
// [RC 1] Message length mismatch (relative to last indication) at the In#9
pub const TCM_REG_CSEM_LENGTH_MIS: c_uint = 0x50174;
// [RW 3] The weight of the input csem in the WRR mechanism. 0 stands for
pub const TCM_REG_CSEM_WEIGHT: c_uint = 0x500bc;
// [RW 8] The Event ID in case of ErrorFlg is set in the input message.
pub const TCM_REG_ERR_EVNT_ID: c_uint = 0x500a0;
// [RW 28] The CM erroneous header for QM and Timers formatting.
pub const TCM_REG_ERR_TCM_HDR: c_uint = 0x5009c;
// [RW 8] The Event ID for Timers expiration.
pub const TCM_REG_EXPR_EVNT_ID: c_uint = 0x500a4;
// [RW 8] FIC0 output initial credit. Max credit available - 255.Write
pub const TCM_REG_FIC0_INIT_CRD: c_uint = 0x5020c;
// [RW 8] FIC1 output initial credit. Max credit available - 255.Write
pub const TCM_REG_FIC1_INIT_CRD: c_uint = 0x50210;
// [RW 1] Arbitration between Input Arbiter groups: 0 - fair Round-Robin; 1
pub const TCM_REG_GR_ARB_TYPE: c_uint = 0x50114;
// [RW 2] Load (FIC0) channel group priority. The lowest priority is 0; the
pub const TCM_REG_GR_LD0_PR: c_uint = 0x5011c;
// [RW 2] Load (FIC1) channel group priority. The lowest priority is 0; the
pub const TCM_REG_GR_LD1_PR: c_uint = 0x50120;
// [RW 4] The number of double REG-pairs; loaded from the STORM context and
pub const TCM_REG_N_SM_CTX_LD_0: c_uint = 0x50050;
pub const TCM_REG_N_SM_CTX_LD_1: c_uint = 0x50054;
pub const TCM_REG_N_SM_CTX_LD_2: c_uint = 0x50058;
pub const TCM_REG_N_SM_CTX_LD_3: c_uint = 0x5005c;
pub const TCM_REG_N_SM_CTX_LD_4: c_uint = 0x50060;
pub const TCM_REG_N_SM_CTX_LD_5: c_uint = 0x50064;
// [RW 1] Input pbf Interface enable. If 0 - the valid input is disregarded;
pub const TCM_REG_PBF_IFEN: c_uint = 0x50024;
// [RC 1] Message length mismatch (relative to last indication) at the In#7
pub const TCM_REG_PBF_LENGTH_MIS: c_uint = 0x5016c;
// [RW 3] The weight of the input pbf in the WRR mechanism. 0 stands for
pub const TCM_REG_PBF_WEIGHT: c_uint = 0x500b4;
pub const TCM_REG_PHYS_QNUM0_0: c_uint = 0x500e0;
pub const TCM_REG_PHYS_QNUM0_1: c_uint = 0x500e4;
pub const TCM_REG_PHYS_QNUM1_0: c_uint = 0x500e8;
pub const TCM_REG_PHYS_QNUM1_1: c_uint = 0x500ec;
pub const TCM_REG_PHYS_QNUM2_0: c_uint = 0x500f0;
pub const TCM_REG_PHYS_QNUM2_1: c_uint = 0x500f4;
pub const TCM_REG_PHYS_QNUM3_0: c_uint = 0x500f8;
pub const TCM_REG_PHYS_QNUM3_1: c_uint = 0x500fc;
// [RW 1] Input prs Interface enable. If 0 - the valid input is disregarded;
pub const TCM_REG_PRS_IFEN: c_uint = 0x50020;
// [RC 1] Message length mismatch (relative to last indication) at the In#6
pub const TCM_REG_PRS_LENGTH_MIS: c_uint = 0x50168;
// [RW 3] The weight of the input prs in the WRR mechanism. 0 stands for
pub const TCM_REG_PRS_WEIGHT: c_uint = 0x500b0;
// [RW 8] The Event ID for Timers formatting in case of stop done.
pub const TCM_REG_STOP_EVNT_ID: c_uint = 0x500a8;
// [RC 1] Message length mismatch (relative to last indication) at the STORM
pub const TCM_REG_STORM_LENGTH_MIS: c_uint = 0x50160;
// [RW 1] STORM - CM Interface enable. If 0 - the valid input is
pub const TCM_REG_STORM_TCM_IFEN: c_uint = 0x50010;
// [RW 3] The weight of the STORM input in the WRR mechanism. 0 stands for
pub const TCM_REG_STORM_WEIGHT: c_uint = 0x500ac;
// [RW 1] CM - CFC Interface enable. If 0 - the valid input is disregarded;
pub const TCM_REG_TCM_CFC_IFEN: c_uint = 0x50040;
// [RW 11] Interrupt mask register #0 read/write
pub const TCM_REG_TCM_INT_MASK: c_uint = 0x501dc;
// [R 11] Interrupt register #0 read
pub const TCM_REG_TCM_INT_STS: c_uint = 0x501d0;
// [RW 27] Parity mask register #0 read/write
pub const TCM_REG_TCM_PRTY_MASK: c_uint = 0x501ec;
// [R 27] Parity register #0 read
pub const TCM_REG_TCM_PRTY_STS: c_uint = 0x501e0;
// [RC 27] Parity register #0 read clear
pub const TCM_REG_TCM_PRTY_STS_CLR: c_uint = 0x501e4;
// [RW 3] The size of AG context region 0 in REG-pairs. Designates the MS
pub const TCM_REG_TCM_REG0_SZ: c_uint = 0x500d8;
// [RW 1] CM - STORM 0 Interface enable. If 0 - the acknowledge input is
pub const TCM_REG_TCM_STORM0_IFEN: c_uint = 0x50004;
// [RW 1] CM - STORM 1 Interface enable. If 0 - the acknowledge input is
pub const TCM_REG_TCM_STORM1_IFEN: c_uint = 0x50008;
// [RW 1] CM - QM Interface enable. If 0 - the acknowledge input is
pub const TCM_REG_TCM_TQM_IFEN: c_uint = 0x5000c;
// [RW 1] If set the Q index; received from the QM is inserted to event ID.
pub const TCM_REG_TCM_TQM_USE_Q: c_uint = 0x500d4;
// [RW 28] The CM header for Timers expiration command.
pub const TCM_REG_TM_TCM_HDR: c_uint = 0x50098;
// [RW 1] Timers - CM Interface enable. If 0 - the valid input is
pub const TCM_REG_TM_TCM_IFEN: c_uint = 0x5001c;
// [RW 3] The weight of the Timers input in the WRR mechanism. 0 stands for
pub const TCM_REG_TM_WEIGHT: c_uint = 0x500d0;
// [RW 6] QM output initial credit. Max credit available - 32.Write writes
pub const TCM_REG_TQM_INIT_CRD: c_uint = 0x5021c;
// [RW 3] The weight of the QM (primary) input in the WRR mechanism. 0
pub const TCM_REG_TQM_P_WEIGHT: c_uint = 0x500c8;
// [RW 3] The weight of the QM (secondary) input in the WRR mechanism. 0
pub const TCM_REG_TQM_S_WEIGHT: c_uint = 0x500cc;
// [RW 28] The CM header value for QM request (primary).
pub const TCM_REG_TQM_TCM_HDR_P: c_uint = 0x50090;
// [RW 28] The CM header value for QM request (secondary).
pub const TCM_REG_TQM_TCM_HDR_S: c_uint = 0x50094;
// [RW 1] QM - CM Interface enable. If 0 - the valid input is disregarded;
pub const TCM_REG_TQM_TCM_IFEN: c_uint = 0x50014;
// [RW 1] Input SDM Interface enable. If 0 - the valid input is disregarded;
pub const TCM_REG_TSDM_IFEN: c_uint = 0x50018;
// [RC 1] Message length mismatch (relative to last indication) at the SDM
pub const TCM_REG_TSDM_LENGTH_MIS: c_uint = 0x50164;
// [RW 3] The weight of the SDM input in the WRR mechanism. 0 stands for
pub const TCM_REG_TSDM_WEIGHT: c_uint = 0x500c4;
// [RW 1] Input usem Interface enable. If 0 - the valid input is
pub const TCM_REG_USEM_IFEN: c_uint = 0x50028;
// [RC 1] Message length mismatch (relative to last indication) at the In#8
pub const TCM_REG_USEM_LENGTH_MIS: c_uint = 0x50170;
// [RW 3] The weight of the input usem in the WRR mechanism. 0 stands for
pub const TCM_REG_USEM_WEIGHT: c_uint = 0x500b8;
// [RW 21] Indirect access to the descriptor table of the XX protection
pub const TCM_REG_XX_DESCR_TABLE: c_uint = 0x50280;
pub const TCM_REG_XX_DESCR_TABLE_SIZE: c_int = 29;
// [R 6] Use to read the value of XX protection Free counter.
pub const TCM_REG_XX_FREE: c_uint = 0x50178;
// [RW 6] Initial value for the credit counter; responsible for fulfilling
pub const TCM_REG_XX_INIT_CRD: c_uint = 0x50220;
// [RW 6] Maximum link list size (messages locked) per connection in the XX
pub const TCM_REG_XX_MAX_LL_SZ: c_uint = 0x50044;
// [RW 6] The maximum number of pending messages; which may be stored in XX
pub const TCM_REG_XX_MSG_NUM: c_uint = 0x50224;
// [RW 8] The Event ID; sent to the STORM in case of XX overflow.
pub const TCM_REG_XX_OVFL_EVNT_ID: c_uint = 0x50048;
// [RW 16] Indirect access to the XX table of the XX protection mechanism.
pub const TCM_REG_XX_TABLE: c_uint = 0x50240;
// [RW 4] Load value for cfc ac credit cnt.
pub const TM_REG_CFC_AC_CRDCNT_VAL: c_uint = 0x164208;
// [RW 4] Load value for cfc cld credit cnt.
pub const TM_REG_CFC_CLD_CRDCNT_VAL: c_uint = 0x164210;
// [RW 8] Client0 context region.
pub const TM_REG_CL0_CONT_REGION: c_uint = 0x164030;
// [RW 8] Client1 context region.
pub const TM_REG_CL1_CONT_REGION: c_uint = 0x164034;
// [RW 8] Client2 context region.
pub const TM_REG_CL2_CONT_REGION: c_uint = 0x164038;
// [RW 2] Client in High priority client number.
pub const TM_REG_CLIN_PRIOR0_CLIENT: c_uint = 0x164024;
// [RW 4] Load value for clout0 cred cnt.
pub const TM_REG_CLOUT_CRDCNT0_VAL: c_uint = 0x164220;
// [RW 4] Load value for clout1 cred cnt.
pub const TM_REG_CLOUT_CRDCNT1_VAL: c_uint = 0x164228;
// [RW 4] Load value for clout2 cred cnt.
pub const TM_REG_CLOUT_CRDCNT2_VAL: c_uint = 0x164230;
// [RW 1] Enable client0 input.
pub const TM_REG_EN_CL0_INPUT: c_uint = 0x164008;
// [RW 1] Enable client1 input.
pub const TM_REG_EN_CL1_INPUT: c_uint = 0x16400c;
// [RW 1] Enable client2 input.
pub const TM_REG_EN_CL2_INPUT: c_uint = 0x164010;
pub const TM_REG_EN_LINEAR0_TIMER: c_uint = 0x164014;
// [RW 1] Enable real time counter.
pub const TM_REG_EN_REAL_TIME_CNT: c_uint = 0x1640d8;
// [RW 1] Enable for Timers state machines.
pub const TM_REG_EN_TIMERS: c_uint = 0x164000;
// [RW 4] Load value for expiration credit cnt. CFC max number of
pub const TM_REG_EXP_CRDCNT_VAL: c_uint = 0x164238;
// [RW 32] Linear0 logic address.
pub const TM_REG_LIN0_LOGIC_ADDR: c_uint = 0x164240;
// [RW 18] Linear0 Max active cid (in banks of 32 entries).
pub const TM_REG_LIN0_MAX_ACTIVE_CID: c_uint = 0x164048;
// [ST 16] Linear0 Number of scans counter.
pub const TM_REG_LIN0_NUM_SCANS: c_uint = 0x1640a0;
// [WB 64] Linear0 phy address.
pub const TM_REG_LIN0_PHY_ADDR: c_uint = 0x164270;
// [RW 1] Linear0 physical address valid.
pub const TM_REG_LIN0_PHY_ADDR_VALID: c_uint = 0x164248;
pub const TM_REG_LIN0_SCAN_ON: c_uint = 0x1640d0;
// [RW 24] Linear0 array scan timeout.
pub const TM_REG_LIN0_SCAN_TIME: c_uint = 0x16403c;
pub const TM_REG_LIN0_VNIC_UC: c_uint = 0x164128;
// [RW 32] Linear1 logic address.
pub const TM_REG_LIN1_LOGIC_ADDR: c_uint = 0x164250;
// [WB 64] Linear1 phy address.
pub const TM_REG_LIN1_PHY_ADDR: c_uint = 0x164280;
// [RW 1] Linear1 physical address valid.
pub const TM_REG_LIN1_PHY_ADDR_VALID: c_uint = 0x164258;
// [RW 6] Linear timer set_clear fifo threshold.
pub const TM_REG_LIN_SETCLR_FIFO_ALFULL_THR: c_uint = 0x164070;
// [RW 2] Load value for pci arbiter credit cnt.
pub const TM_REG_PCIARB_CRDCNT_VAL: c_uint = 0x164260;
// [RW 20] The amount of hardware cycles for each timer tick.
pub const TM_REG_TIMER_TICK_SIZE: c_uint = 0x16401c;
// [RW 8] Timers Context region.
pub const TM_REG_TM_CONTEXT_REGION: c_uint = 0x164044;
// [RW 1] Interrupt mask register #0 read/write
pub const TM_REG_TM_INT_MASK: c_uint = 0x1640fc;
// [R 1] Interrupt register #0 read
pub const TM_REG_TM_INT_STS: c_uint = 0x1640f0;
// [RW 7] Parity mask register #0 read/write
pub const TM_REG_TM_PRTY_MASK: c_uint = 0x16410c;
// [R 7] Parity register #0 read
pub const TM_REG_TM_PRTY_STS: c_uint = 0x164100;
// [RC 7] Parity register #0 read clear
pub const TM_REG_TM_PRTY_STS_CLR: c_uint = 0x164104;
// [RW 8] The event id for aggregated interrupt 0
pub const TSDM_REG_AGG_INT_EVENT_0: c_uint = 0x42038;
pub const TSDM_REG_AGG_INT_EVENT_1: c_uint = 0x4203c;
pub const TSDM_REG_AGG_INT_EVENT_2: c_uint = 0x42040;
pub const TSDM_REG_AGG_INT_EVENT_3: c_uint = 0x42044;
pub const TSDM_REG_AGG_INT_EVENT_4: c_uint = 0x42048;
// [RW 1] The T bit for aggregated interrupt 0
pub const TSDM_REG_AGG_INT_T_0: c_uint = 0x420b8;
pub const TSDM_REG_AGG_INT_T_1: c_uint = 0x420bc;
// [RW 13] The start address in the internal RAM for the cfc_rsp lcid
pub const TSDM_REG_CFC_RSP_START_ADDR: c_uint = 0x42008;
// [RW 16] The maximum value of the completion counter #0
pub const TSDM_REG_CMP_COUNTER_MAX0: c_uint = 0x4201c;
// [RW 16] The maximum value of the completion counter #1
pub const TSDM_REG_CMP_COUNTER_MAX1: c_uint = 0x42020;
// [RW 16] The maximum value of the completion counter #2
pub const TSDM_REG_CMP_COUNTER_MAX2: c_uint = 0x42024;
// [RW 16] The maximum value of the completion counter #3
pub const TSDM_REG_CMP_COUNTER_MAX3: c_uint = 0x42028;
// [RW 13] The start address in the internal RAM for the completion
pub const TSDM_REG_CMP_COUNTER_START_ADDR: c_uint = 0x4200c;
pub const TSDM_REG_ENABLE_IN1: c_uint = 0x42238;
pub const TSDM_REG_ENABLE_IN2: c_uint = 0x4223c;
pub const TSDM_REG_ENABLE_OUT1: c_uint = 0x42240;
pub const TSDM_REG_ENABLE_OUT2: c_uint = 0x42244;
// [RW 4] The initial number of messages that can be sent to the pxp control
pub const TSDM_REG_INIT_CREDIT_PXP_CTRL: c_uint = 0x424bc;
// [ST 32] The number of ACK after placement messages received
pub const TSDM_REG_NUM_OF_ACK_AFTER_PLACE: c_uint = 0x4227c;
// [ST 32] The number of packet end messages received from the parser
pub const TSDM_REG_NUM_OF_PKT_END_MSG: c_uint = 0x42274;
// [ST 32] The number of requests received from the pxp async if
pub const TSDM_REG_NUM_OF_PXP_ASYNC_REQ: c_uint = 0x42278;
// [ST 32] The number of commands received in queue 0
pub const TSDM_REG_NUM_OF_Q0_CMD: c_uint = 0x42248;
// [ST 32] The number of commands received in queue 10
pub const TSDM_REG_NUM_OF_Q10_CMD: c_uint = 0x4226c;
// [ST 32] The number of commands received in queue 11
pub const TSDM_REG_NUM_OF_Q11_CMD: c_uint = 0x42270;
// [ST 32] The number of commands received in queue 1
pub const TSDM_REG_NUM_OF_Q1_CMD: c_uint = 0x4224c;
// [ST 32] The number of commands received in queue 3
pub const TSDM_REG_NUM_OF_Q3_CMD: c_uint = 0x42250;
// [ST 32] The number of commands received in queue 4
pub const TSDM_REG_NUM_OF_Q4_CMD: c_uint = 0x42254;
// [ST 32] The number of commands received in queue 5
pub const TSDM_REG_NUM_OF_Q5_CMD: c_uint = 0x42258;
// [ST 32] The number of commands received in queue 6
pub const TSDM_REG_NUM_OF_Q6_CMD: c_uint = 0x4225c;
// [ST 32] The number of commands received in queue 7
pub const TSDM_REG_NUM_OF_Q7_CMD: c_uint = 0x42260;
// [ST 32] The number of commands received in queue 8
pub const TSDM_REG_NUM_OF_Q8_CMD: c_uint = 0x42264;
// [ST 32] The number of commands received in queue 9
pub const TSDM_REG_NUM_OF_Q9_CMD: c_uint = 0x42268;
// [RW 13] The start address in the internal RAM for the packet end message
pub const TSDM_REG_PCK_END_MSG_START_ADDR: c_uint = 0x42014;
// [RW 13] The start address in the internal RAM for queue counters
pub const TSDM_REG_Q_COUNTER_START_ADDR: c_uint = 0x42010;
// [R 1] pxp_ctrl rd_data fifo empty in sdm_dma_rsp block
pub const TSDM_REG_RSP_PXP_CTRL_RDATA_EMPTY: c_uint = 0x42548;
// [R 1] parser fifo empty in sdm_sync block
pub const TSDM_REG_SYNC_PARSER_EMPTY: c_uint = 0x42550;
// [R 1] parser serial fifo empty in sdm_sync block
pub const TSDM_REG_SYNC_SYNC_EMPTY: c_uint = 0x42558;
// [RW 32] Tick for timer counter. Applicable only when
pub const TSDM_REG_TIMER_TICK: c_uint = 0x42000;
// [RW 32] Interrupt mask register #0 read/write
pub const TSDM_REG_TSDM_INT_MASK_0: c_uint = 0x4229c;
pub const TSDM_REG_TSDM_INT_MASK_1: c_uint = 0x422ac;
// [R 32] Interrupt register #0 read
pub const TSDM_REG_TSDM_INT_STS_0: c_uint = 0x42290;
pub const TSDM_REG_TSDM_INT_STS_1: c_uint = 0x422a0;
// [RW 11] Parity mask register #0 read/write
pub const TSDM_REG_TSDM_PRTY_MASK: c_uint = 0x422bc;
// [R 11] Parity register #0 read
pub const TSDM_REG_TSDM_PRTY_STS: c_uint = 0x422b0;
// [RC 11] Parity register #0 read clear
pub const TSDM_REG_TSDM_PRTY_STS_CLR: c_uint = 0x422b4;
// [RW 5] The number of time_slots in the arbitration cycle
pub const TSEM_REG_ARB_CYCLE_SIZE: c_uint = 0x180034;
// [RW 3] The source that is associated with arbitration element 0. Source
pub const TSEM_REG_ARB_ELEMENT0: c_uint = 0x180020;
// [RW 3] The source that is associated with arbitration element 1. Source
pub const TSEM_REG_ARB_ELEMENT1: c_uint = 0x180024;
// [RW 3] The source that is associated with arbitration element 2. Source
pub const TSEM_REG_ARB_ELEMENT2: c_uint = 0x180028;
// [RW 3] The source that is associated with arbitration element 3. Source
pub const TSEM_REG_ARB_ELEMENT3: c_uint = 0x18002c;
// [RW 3] The source that is associated with arbitration element 4. Source
pub const TSEM_REG_ARB_ELEMENT4: c_uint = 0x180030;
pub const TSEM_REG_ENABLE_IN: c_uint = 0x1800a4;
pub const TSEM_REG_ENABLE_OUT: c_uint = 0x1800a8;
// [RW 32] This address space contains all registers and memories that are
pub const TSEM_REG_FAST_MEMORY: c_uint = 0x1a0000;
// [RW 1] Disables input messages from FIC0 May be updated during run_time
pub const TSEM_REG_FIC0_DISABLE: c_uint = 0x180224;
// [RW 1] Disables input messages from FIC1 May be updated during run_time
pub const TSEM_REG_FIC1_DISABLE: c_uint = 0x180234;
// [RW 15] Interrupt table Read and write access to it is not possible in
pub const TSEM_REG_INT_TABLE: c_uint = 0x180400;
// [ST 24] Statistics register. The number of messages that entered through
pub const TSEM_REG_MSG_NUM_FIC0: c_uint = 0x180000;
// [ST 24] Statistics register. The number of messages that entered through
pub const TSEM_REG_MSG_NUM_FIC1: c_uint = 0x180004;
// [ST 24] Statistics register. The number of messages that were sent to
pub const TSEM_REG_MSG_NUM_FOC0: c_uint = 0x180008;
// [ST 24] Statistics register. The number of messages that were sent to
pub const TSEM_REG_MSG_NUM_FOC1: c_uint = 0x18000c;
// [ST 24] Statistics register. The number of messages that were sent to
pub const TSEM_REG_MSG_NUM_FOC2: c_uint = 0x180010;
// [ST 24] Statistics register. The number of messages that were sent to
pub const TSEM_REG_MSG_NUM_FOC3: c_uint = 0x180014;
// [RW 1] Disables input messages from the passive buffer May be updated
pub const TSEM_REG_PAS_DISABLE: c_uint = 0x18024c;
// [WB 128] Debug only. Passive buffer memory
pub const TSEM_REG_PASSIVE_BUFFER: c_uint = 0x181000;
// [WB 46] pram memory. B45 is parity; b[44:0] - data.
pub const TSEM_REG_PRAM: c_uint = 0x1c0000;
// [R 8] Valid sleeping threads indication have bit per thread
pub const TSEM_REG_SLEEP_THREADS_VALID: c_uint = 0x18026c;
// [R 1] EXT_STORE FIFO is empty in sem_slow_ls_ext
pub const TSEM_REG_SLOW_EXT_STORE_EMPTY: c_uint = 0x1802a0;
// [RW 8] List of free threads . There is a bit per thread.
pub const TSEM_REG_THREADS_LIST: c_uint = 0x1802e4;
// [RC 32] Parity register #0 read clear
pub const TSEM_REG_TSEM_PRTY_STS_CLR_0: c_uint = 0x180118;
pub const TSEM_REG_TSEM_PRTY_STS_CLR_1: c_uint = 0x180128;
// [RW 3] The arbitration scheme of time_slot 0
pub const TSEM_REG_TS_0_AS: c_uint = 0x180038;
// [RW 3] The arbitration scheme of time_slot 10
pub const TSEM_REG_TS_10_AS: c_uint = 0x180060;
// [RW 3] The arbitration scheme of time_slot 11
pub const TSEM_REG_TS_11_AS: c_uint = 0x180064;
// [RW 3] The arbitration scheme of time_slot 12
pub const TSEM_REG_TS_12_AS: c_uint = 0x180068;
// [RW 3] The arbitration scheme of time_slot 13
pub const TSEM_REG_TS_13_AS: c_uint = 0x18006c;
// [RW 3] The arbitration scheme of time_slot 14
pub const TSEM_REG_TS_14_AS: c_uint = 0x180070;
// [RW 3] The arbitration scheme of time_slot 15
pub const TSEM_REG_TS_15_AS: c_uint = 0x180074;
// [RW 3] The arbitration scheme of time_slot 16
pub const TSEM_REG_TS_16_AS: c_uint = 0x180078;
// [RW 3] The arbitration scheme of time_slot 17
pub const TSEM_REG_TS_17_AS: c_uint = 0x18007c;
// [RW 3] The arbitration scheme of time_slot 18
pub const TSEM_REG_TS_18_AS: c_uint = 0x180080;
// [RW 3] The arbitration scheme of time_slot 1
pub const TSEM_REG_TS_1_AS: c_uint = 0x18003c;
// [RW 3] The arbitration scheme of time_slot 2
pub const TSEM_REG_TS_2_AS: c_uint = 0x180040;
// [RW 3] The arbitration scheme of time_slot 3
pub const TSEM_REG_TS_3_AS: c_uint = 0x180044;
// [RW 3] The arbitration scheme of time_slot 4
pub const TSEM_REG_TS_4_AS: c_uint = 0x180048;
// [RW 3] The arbitration scheme of time_slot 5
pub const TSEM_REG_TS_5_AS: c_uint = 0x18004c;
// [RW 3] The arbitration scheme of time_slot 6
pub const TSEM_REG_TS_6_AS: c_uint = 0x180050;
// [RW 3] The arbitration scheme of time_slot 7
pub const TSEM_REG_TS_7_AS: c_uint = 0x180054;
// [RW 3] The arbitration scheme of time_slot 8
pub const TSEM_REG_TS_8_AS: c_uint = 0x180058;
// [RW 3] The arbitration scheme of time_slot 9
pub const TSEM_REG_TS_9_AS: c_uint = 0x18005c;
// [RW 32] Interrupt mask register #0 read/write
pub const TSEM_REG_TSEM_INT_MASK_0: c_uint = 0x180100;
pub const TSEM_REG_TSEM_INT_MASK_1: c_uint = 0x180110;
// [R 32] Interrupt register #0 read
pub const TSEM_REG_TSEM_INT_STS_0: c_uint = 0x1800f4;
pub const TSEM_REG_TSEM_INT_STS_1: c_uint = 0x180104;
// [RW 32] Parity mask register #0 read/write
pub const TSEM_REG_TSEM_PRTY_MASK_0: c_uint = 0x180120;
pub const TSEM_REG_TSEM_PRTY_MASK_1: c_uint = 0x180130;
// [R 32] Parity register #0 read
pub const TSEM_REG_TSEM_PRTY_STS_0: c_uint = 0x180114;
pub const TSEM_REG_TSEM_PRTY_STS_1: c_uint = 0x180124;
// [W 7] VF or PF ID for reset error bit. Values 0-63 reset error bit for 64
// VF; values 64-67 reset error for 4 PF; values 68-127 are not valid.
pub const TSEM_REG_VFPF_ERR_NUM: c_uint = 0x180380;
// [RW 32] Indirect access to AG context with 32-bits granularity. The bits
// [10:8] of the address should be the offset within the accessed LCID
// context; the bits [7:0] are the accessed LCID.Example: to write to REG10
// LCID100. The RBC address should be 12'ha64.
pub const UCM_REG_AG_CTX: c_uint = 0xe2000;
// [R 5] Used to read the XX protection CAM occupancy counter.
pub const UCM_REG_CAM_OCCUP: c_uint = 0xe0170;
// [RW 1] CDU AG read Interface enable. If 0 - the request input is
pub const UCM_REG_CDU_AG_RD_IFEN: c_uint = 0xe0038;
// [RW 1] CDU AG write Interface enable. If 0 - the request and valid input
pub const UCM_REG_CDU_AG_WR_IFEN: c_uint = 0xe0034;
// [RW 1] CDU STORM read Interface enable. If 0 - the request input is
pub const UCM_REG_CDU_SM_RD_IFEN: c_uint = 0xe0040;
// [RW 1] CDU STORM write Interface enable. If 0 - the request and valid
pub const UCM_REG_CDU_SM_WR_IFEN: c_uint = 0xe003c;
// [RW 4] CFC output initial credit. Max credit available - 15.Write writes
pub const UCM_REG_CFC_INIT_CRD: c_uint = 0xe0204;
// [RW 3] The weight of the CP input in the WRR mechanism. 0 stands for
pub const UCM_REG_CP_WEIGHT: c_uint = 0xe00c4;
// [RW 1] Input csem Interface enable. If 0 - the valid input is
pub const UCM_REG_CSEM_IFEN: c_uint = 0xe0028;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_CSEM_LENGTH_MIS: c_uint = 0xe0160;
// [RW 3] The weight of the input csem in the WRR mechanism. 0 stands for
pub const UCM_REG_CSEM_WEIGHT: c_uint = 0xe00b8;
// [RW 1] Input dorq Interface enable. If 0 - the valid input is
pub const UCM_REG_DORQ_IFEN: c_uint = 0xe0030;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_DORQ_LENGTH_MIS: c_uint = 0xe0168;
// [RW 3] The weight of the input dorq in the WRR mechanism. 0 stands for
pub const UCM_REG_DORQ_WEIGHT: c_uint = 0xe00c0;
// [RW 8] The Event ID in case ErrorFlg input message bit is set.
pub const UCM_REG_ERR_EVNT_ID: c_uint = 0xe00a4;
// [RW 28] The CM erroneous header for QM and Timers formatting.
pub const UCM_REG_ERR_UCM_HDR: c_uint = 0xe00a0;
// [RW 8] The Event ID for Timers expiration.
pub const UCM_REG_EXPR_EVNT_ID: c_uint = 0xe00a8;
// [RW 8] FIC0 output initial credit. Max credit available - 255.Write
pub const UCM_REG_FIC0_INIT_CRD: c_uint = 0xe020c;
// [RW 8] FIC1 output initial credit. Max credit available - 255.Write
pub const UCM_REG_FIC1_INIT_CRD: c_uint = 0xe0210;
// [RW 1] Arbitration between Input Arbiter groups: 0 - fair Round-Robin; 1
pub const UCM_REG_GR_ARB_TYPE: c_uint = 0xe0144;
// [RW 2] Load (FIC0) channel group priority. The lowest priority is 0; the
pub const UCM_REG_GR_LD0_PR: c_uint = 0xe014c;
// [RW 2] Load (FIC1) channel group priority. The lowest priority is 0; the
pub const UCM_REG_GR_LD1_PR: c_uint = 0xe0150;
// [RW 2] The queue index for invalidate counter flag decision.
pub const UCM_REG_INV_CFLG_Q: c_uint = 0xe00e4;
// [RW 5] The number of double REG-pairs; loaded from the STORM context and
pub const UCM_REG_N_SM_CTX_LD_0: c_uint = 0xe0054;
pub const UCM_REG_N_SM_CTX_LD_1: c_uint = 0xe0058;
pub const UCM_REG_N_SM_CTX_LD_2: c_uint = 0xe005c;
pub const UCM_REG_N_SM_CTX_LD_3: c_uint = 0xe0060;
pub const UCM_REG_N_SM_CTX_LD_4: c_uint = 0xe0064;
pub const UCM_REG_N_SM_CTX_LD_5: c_uint = 0xe0068;
pub const UCM_REG_PHYS_QNUM0_0: c_uint = 0xe0110;
pub const UCM_REG_PHYS_QNUM0_1: c_uint = 0xe0114;
pub const UCM_REG_PHYS_QNUM1_0: c_uint = 0xe0118;
pub const UCM_REG_PHYS_QNUM1_1: c_uint = 0xe011c;
pub const UCM_REG_PHYS_QNUM2_0: c_uint = 0xe0120;
pub const UCM_REG_PHYS_QNUM2_1: c_uint = 0xe0124;
pub const UCM_REG_PHYS_QNUM3_0: c_uint = 0xe0128;
pub const UCM_REG_PHYS_QNUM3_1: c_uint = 0xe012c;
// [RW 8] The Event ID for Timers formatting in case of stop done.
pub const UCM_REG_STOP_EVNT_ID: c_uint = 0xe00ac;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_STORM_LENGTH_MIS: c_uint = 0xe0154;
// [RW 1] STORM - CM Interface enable. If 0 - the valid input is
pub const UCM_REG_STORM_UCM_IFEN: c_uint = 0xe0010;
// [RW 3] The weight of the STORM input in the WRR mechanism. 0 stands for
pub const UCM_REG_STORM_WEIGHT: c_uint = 0xe00b0;
// [RW 4] Timers output initial credit. Max credit available - 15.Write
pub const UCM_REG_TM_INIT_CRD: c_uint = 0xe021c;
// [RW 28] The CM header for Timers expiration command.
pub const UCM_REG_TM_UCM_HDR: c_uint = 0xe009c;
// [RW 1] Timers - CM Interface enable. If 0 - the valid input is
pub const UCM_REG_TM_UCM_IFEN: c_uint = 0xe001c;
// [RW 3] The weight of the Timers input in the WRR mechanism. 0 stands for
pub const UCM_REG_TM_WEIGHT: c_uint = 0xe00d4;
// [RW 1] Input tsem Interface enable. If 0 - the valid input is
pub const UCM_REG_TSEM_IFEN: c_uint = 0xe0024;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_TSEM_LENGTH_MIS: c_uint = 0xe015c;
// [RW 3] The weight of the input tsem in the WRR mechanism. 0 stands for
pub const UCM_REG_TSEM_WEIGHT: c_uint = 0xe00b4;
// [RW 1] CM - CFC Interface enable. If 0 - the valid input is disregarded;
pub const UCM_REG_UCM_CFC_IFEN: c_uint = 0xe0044;
// [RW 11] Interrupt mask register #0 read/write
pub const UCM_REG_UCM_INT_MASK: c_uint = 0xe01d4;
// [R 11] Interrupt register #0 read
pub const UCM_REG_UCM_INT_STS: c_uint = 0xe01c8;
// [RW 27] Parity mask register #0 read/write
pub const UCM_REG_UCM_PRTY_MASK: c_uint = 0xe01e4;
// [R 27] Parity register #0 read
pub const UCM_REG_UCM_PRTY_STS: c_uint = 0xe01d8;
// [RC 27] Parity register #0 read clear
pub const UCM_REG_UCM_PRTY_STS_CLR: c_uint = 0xe01dc;
// [RW 2] The size of AG context region 0 in REG-pairs. Designates the MS
pub const UCM_REG_UCM_REG0_SZ: c_uint = 0xe00dc;
// [RW 1] CM - STORM 0 Interface enable. If 0 - the acknowledge input is
pub const UCM_REG_UCM_STORM0_IFEN: c_uint = 0xe0004;
// [RW 1] CM - STORM 1 Interface enable. If 0 - the acknowledge input is
pub const UCM_REG_UCM_STORM1_IFEN: c_uint = 0xe0008;
// [RW 1] CM - Timers Interface enable. If 0 - the valid input is
pub const UCM_REG_UCM_TM_IFEN: c_uint = 0xe0020;
// [RW 1] CM - QM Interface enable. If 0 - the acknowledge input is
pub const UCM_REG_UCM_UQM_IFEN: c_uint = 0xe000c;
// [RW 1] If set the Q index; received from the QM is inserted to event ID.
pub const UCM_REG_UCM_UQM_USE_Q: c_uint = 0xe00d8;
// [RW 6] QM output initial credit. Max credit available - 32.Write writes
pub const UCM_REG_UQM_INIT_CRD: c_uint = 0xe0220;
// [RW 3] The weight of the QM (primary) input in the WRR mechanism. 0
pub const UCM_REG_UQM_P_WEIGHT: c_uint = 0xe00cc;
// [RW 3] The weight of the QM (secondary) input in the WRR mechanism. 0
pub const UCM_REG_UQM_S_WEIGHT: c_uint = 0xe00d0;
// [RW 28] The CM header value for QM request (primary).
pub const UCM_REG_UQM_UCM_HDR_P: c_uint = 0xe0094;
// [RW 28] The CM header value for QM request (secondary).
pub const UCM_REG_UQM_UCM_HDR_S: c_uint = 0xe0098;
// [RW 1] QM - CM Interface enable. If 0 - the valid input is disregarded;
pub const UCM_REG_UQM_UCM_IFEN: c_uint = 0xe0014;
// [RW 1] Input SDM Interface enable. If 0 - the valid input is disregarded;
pub const UCM_REG_USDM_IFEN: c_uint = 0xe0018;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_USDM_LENGTH_MIS: c_uint = 0xe0158;
// [RW 3] The weight of the SDM input in the WRR mechanism. 0 stands for
pub const UCM_REG_USDM_WEIGHT: c_uint = 0xe00c8;
// [RW 1] Input xsem Interface enable. If 0 - the valid input is
pub const UCM_REG_XSEM_IFEN: c_uint = 0xe002c;
// [RC 1] Set when the message length mismatch (relative to last indication)
pub const UCM_REG_XSEM_LENGTH_MIS: c_uint = 0xe0164;
// [RW 3] The weight of the input xsem in the WRR mechanism. 0 stands for
pub const UCM_REG_XSEM_WEIGHT: c_uint = 0xe00bc;
// [RW 20] Indirect access to the descriptor table of the XX protection
pub const UCM_REG_XX_DESCR_TABLE: c_uint = 0xe0280;
pub const UCM_REG_XX_DESCR_TABLE_SIZE: c_int = 27;
// [R 6] Use to read the XX protection Free counter.
pub const UCM_REG_XX_FREE: c_uint = 0xe016c;
// [RW 6] Initial value for the credit counter; responsible for fulfilling
pub const UCM_REG_XX_INIT_CRD: c_uint = 0xe0224;
// [RW 6] The maximum number of pending messages; which may be stored in XX
pub const UCM_REG_XX_MSG_NUM: c_uint = 0xe0228;
// [RW 8] The Event ID; sent to the STORM in case of XX overflow.
pub const UCM_REG_XX_OVFL_EVNT_ID: c_uint = 0xe004c;
// [RW 16] Indirect access to the XX table of the XX protection mechanism.
pub const UCM_REG_XX_TABLE: c_uint = 0xe0300;

pub const UMAC_REG_COMMAND_CONFIG: c_uint = 0x8;
// [RW 16] This is the duration for which MAC must wait to go back to ACTIVE
// state from LPI state when it receives packet for transmission. The
// decrement unit is 1 micro-second.
pub const UMAC_REG_EEE_WAKE_TIMER: c_uint = 0x6c;
// [RW 32] Register Bit 0 refers to Bit 16 of the MAC address; Bit 1 refers
// to bit 17 of the MAC address etc.
pub const UMAC_REG_MAC_ADDR0: c_uint = 0xc;
// [RW 16] Register Bit 0 refers to Bit 0 of the MAC address; Register Bit 1
// refers to Bit 1 of the MAC address etc. Bits 16 to 31 are reserved.
pub const UMAC_REG_MAC_ADDR1: c_uint = 0x10;
// [RW 14] Defines a 14-Bit maximum frame length used by the MAC receive
// logic to check frames.
pub const UMAC_REG_MAXFR: c_uint = 0x14;
pub const UMAC_REG_UMAC_EEE_CTRL: c_uint = 0x64;

// [RW 8] The event id for aggregated interrupt 0
pub const USDM_REG_AGG_INT_EVENT_0: c_uint = 0xc4038;
pub const USDM_REG_AGG_INT_EVENT_1: c_uint = 0xc403c;
pub const USDM_REG_AGG_INT_EVENT_2: c_uint = 0xc4040;
pub const USDM_REG_AGG_INT_EVENT_4: c_uint = 0xc4048;
pub const USDM_REG_AGG_INT_EVENT_5: c_uint = 0xc404c;
pub const USDM_REG_AGG_INT_EVENT_6: c_uint = 0xc4050;
// [RW 1] For each aggregated interrupt index whether the mode is normal (0)
pub const USDM_REG_AGG_INT_MODE_0: c_uint = 0xc41b8;
pub const USDM_REG_AGG_INT_MODE_1: c_uint = 0xc41bc;
pub const USDM_REG_AGG_INT_MODE_4: c_uint = 0xc41c8;
pub const USDM_REG_AGG_INT_MODE_5: c_uint = 0xc41cc;
pub const USDM_REG_AGG_INT_MODE_6: c_uint = 0xc41d0;
// [RW 1] The T bit for aggregated interrupt 5
pub const USDM_REG_AGG_INT_T_5: c_uint = 0xc40cc;
pub const USDM_REG_AGG_INT_T_6: c_uint = 0xc40d0;
// [RW 13] The start address in the internal RAM for the cfc_rsp lcid
pub const USDM_REG_CFC_RSP_START_ADDR: c_uint = 0xc4008;
// [RW 16] The maximum value of the completion counter #0
pub const USDM_REG_CMP_COUNTER_MAX0: c_uint = 0xc401c;
// [RW 16] The maximum value of the completion counter #1
pub const USDM_REG_CMP_COUNTER_MAX1: c_uint = 0xc4020;
// [RW 16] The maximum value of the completion counter #2
pub const USDM_REG_CMP_COUNTER_MAX2: c_uint = 0xc4024;
// [RW 16] The maximum value of the completion counter #3
pub const USDM_REG_CMP_COUNTER_MAX3: c_uint = 0xc4028;
// [RW 13] The start address in the internal RAM for the completion
pub const USDM_REG_CMP_COUNTER_START_ADDR: c_uint = 0xc400c;
pub const USDM_REG_ENABLE_IN1: c_uint = 0xc4238;
pub const USDM_REG_ENABLE_IN2: c_uint = 0xc423c;
pub const USDM_REG_ENABLE_OUT1: c_uint = 0xc4240;
pub const USDM_REG_ENABLE_OUT2: c_uint = 0xc4244;
// [RW 4] The initial number of messages that can be sent to the pxp control
pub const USDM_REG_INIT_CREDIT_PXP_CTRL: c_uint = 0xc44c0;
// [ST 32] The number of ACK after placement messages received
pub const USDM_REG_NUM_OF_ACK_AFTER_PLACE: c_uint = 0xc4280;
// [ST 32] The number of packet end messages received from the parser
pub const USDM_REG_NUM_OF_PKT_END_MSG: c_uint = 0xc4278;
// [ST 32] The number of requests received from the pxp async if
pub const USDM_REG_NUM_OF_PXP_ASYNC_REQ: c_uint = 0xc427c;
// [ST 32] The number of commands received in queue 0
pub const USDM_REG_NUM_OF_Q0_CMD: c_uint = 0xc4248;
// [ST 32] The number of commands received in queue 10
pub const USDM_REG_NUM_OF_Q10_CMD: c_uint = 0xc4270;
// [ST 32] The number of commands received in queue 11
pub const USDM_REG_NUM_OF_Q11_CMD: c_uint = 0xc4274;
// [ST 32] The number of commands received in queue 1
pub const USDM_REG_NUM_OF_Q1_CMD: c_uint = 0xc424c;
// [ST 32] The number of commands received in queue 2
pub const USDM_REG_NUM_OF_Q2_CMD: c_uint = 0xc4250;
// [ST 32] The number of commands received in queue 3
pub const USDM_REG_NUM_OF_Q3_CMD: c_uint = 0xc4254;
// [ST 32] The number of commands received in queue 4
pub const USDM_REG_NUM_OF_Q4_CMD: c_uint = 0xc4258;
// [ST 32] The number of commands received in queue 5
pub const USDM_REG_NUM_OF_Q5_CMD: c_uint = 0xc425c;
// [ST 32] The number of commands received in queue 6
pub const USDM_REG_NUM_OF_Q6_CMD: c_uint = 0xc4260;
// [ST 32] The number of commands received in queue 7
pub const USDM_REG_NUM_OF_Q7_CMD: c_uint = 0xc4264;
// [ST 32] The number of commands received in queue 8
pub const USDM_REG_NUM_OF_Q8_CMD: c_uint = 0xc4268;
// [ST 32] The number of commands received in queue 9
pub const USDM_REG_NUM_OF_Q9_CMD: c_uint = 0xc426c;
// [RW 13] The start address in the internal RAM for the packet end message
pub const USDM_REG_PCK_END_MSG_START_ADDR: c_uint = 0xc4014;
// [RW 13] The start address in the internal RAM for queue counters
pub const USDM_REG_Q_COUNTER_START_ADDR: c_uint = 0xc4010;
// [R 1] pxp_ctrl rd_data fifo empty in sdm_dma_rsp block
pub const USDM_REG_RSP_PXP_CTRL_RDATA_EMPTY: c_uint = 0xc4550;
// [R 1] parser fifo empty in sdm_sync block
pub const USDM_REG_SYNC_PARSER_EMPTY: c_uint = 0xc4558;
// [R 1] parser serial fifo empty in sdm_sync block
pub const USDM_REG_SYNC_SYNC_EMPTY: c_uint = 0xc4560;
// [RW 32] Tick for timer counter. Applicable only when
pub const USDM_REG_TIMER_TICK: c_uint = 0xc4000;
// [RW 32] Interrupt mask register #0 read/write
pub const USDM_REG_USDM_INT_MASK_0: c_uint = 0xc42a0;
pub const USDM_REG_USDM_INT_MASK_1: c_uint = 0xc42b0;
// [R 32] Interrupt register #0 read
pub const USDM_REG_USDM_INT_STS_0: c_uint = 0xc4294;
pub const USDM_REG_USDM_INT_STS_1: c_uint = 0xc42a4;
// [RW 11] Parity mask register #0 read/write
pub const USDM_REG_USDM_PRTY_MASK: c_uint = 0xc42c0;
// [R 11] Parity register #0 read
pub const USDM_REG_USDM_PRTY_STS: c_uint = 0xc42b4;
// [RC 11] Parity register #0 read clear
pub const USDM_REG_USDM_PRTY_STS_CLR: c_uint = 0xc42b8;
// [RW 5] The number of time_slots in the arbitration cycle
pub const USEM_REG_ARB_CYCLE_SIZE: c_uint = 0x300034;
// [RW 3] The source that is associated with arbitration element 0. Source
pub const USEM_REG_ARB_ELEMENT0: c_uint = 0x300020;
// [RW 3] The source that is associated with arbitration element 1. Source
pub const USEM_REG_ARB_ELEMENT1: c_uint = 0x300024;
// [RW 3] The source that is associated with arbitration element 2. Source
pub const USEM_REG_ARB_ELEMENT2: c_uint = 0x300028;
// [RW 3] The source that is associated with arbitration element 3. Source
pub const USEM_REG_ARB_ELEMENT3: c_uint = 0x30002c;
// [RW 3] The source that is associated with arbitration element 4. Source
pub const USEM_REG_ARB_ELEMENT4: c_uint = 0x300030;
pub const USEM_REG_ENABLE_IN: c_uint = 0x3000a4;
pub const USEM_REG_ENABLE_OUT: c_uint = 0x3000a8;
// [RW 32] This address space contains all registers and memories that are
pub const USEM_REG_FAST_MEMORY: c_uint = 0x320000;
// [RW 1] Disables input messages from FIC0 May be updated during run_time
pub const USEM_REG_FIC0_DISABLE: c_uint = 0x300224;
// [RW 1] Disables input messages from FIC1 May be updated during run_time
pub const USEM_REG_FIC1_DISABLE: c_uint = 0x300234;
// [RW 15] Interrupt table Read and write access to it is not possible in
pub const USEM_REG_INT_TABLE: c_uint = 0x300400;
// [ST 24] Statistics register. The number of messages that entered through
pub const USEM_REG_MSG_NUM_FIC0: c_uint = 0x300000;
// [ST 24] Statistics register. The number of messages that entered through
pub const USEM_REG_MSG_NUM_FIC1: c_uint = 0x300004;
// [ST 24] Statistics register. The number of messages that were sent to
pub const USEM_REG_MSG_NUM_FOC0: c_uint = 0x300008;
// [ST 24] Statistics register. The number of messages that were sent to
pub const USEM_REG_MSG_NUM_FOC1: c_uint = 0x30000c;
// [ST 24] Statistics register. The number of messages that were sent to
pub const USEM_REG_MSG_NUM_FOC2: c_uint = 0x300010;
// [ST 24] Statistics register. The number of messages that were sent to
pub const USEM_REG_MSG_NUM_FOC3: c_uint = 0x300014;
// [RW 1] Disables input messages from the passive buffer May be updated
pub const USEM_REG_PAS_DISABLE: c_uint = 0x30024c;
// [WB 128] Debug only. Passive buffer memory
pub const USEM_REG_PASSIVE_BUFFER: c_uint = 0x302000;
// [WB 46] pram memory. B45 is parity; b[44:0] - data.
pub const USEM_REG_PRAM: c_uint = 0x340000;
// [R 16] Valid sleeping threads indication have bit per thread
pub const USEM_REG_SLEEP_THREADS_VALID: c_uint = 0x30026c;
// [R 1] EXT_STORE FIFO is empty in sem_slow_ls_ext
pub const USEM_REG_SLOW_EXT_STORE_EMPTY: c_uint = 0x3002a0;
// [RW 16] List of free threads . There is a bit per thread.
pub const USEM_REG_THREADS_LIST: c_uint = 0x3002e4;
// [RW 3] The arbitration scheme of time_slot 0
pub const USEM_REG_TS_0_AS: c_uint = 0x300038;
// [RW 3] The arbitration scheme of time_slot 10
pub const USEM_REG_TS_10_AS: c_uint = 0x300060;
// [RW 3] The arbitration scheme of time_slot 11
pub const USEM_REG_TS_11_AS: c_uint = 0x300064;
// [RW 3] The arbitration scheme of time_slot 12
pub const USEM_REG_TS_12_AS: c_uint = 0x300068;
// [RW 3] The arbitration scheme of time_slot 13
pub const USEM_REG_TS_13_AS: c_uint = 0x30006c;
// [RW 3] The arbitration scheme of time_slot 14
pub const USEM_REG_TS_14_AS: c_uint = 0x300070;
// [RW 3] The arbitration scheme of time_slot 15
pub const USEM_REG_TS_15_AS: c_uint = 0x300074;
// [RW 3] The arbitration scheme of time_slot 16
pub const USEM_REG_TS_16_AS: c_uint = 0x300078;
// [RW 3] The arbitration scheme of time_slot 17
pub const USEM_REG_TS_17_AS: c_uint = 0x30007c;
// [RW 3] The arbitration scheme of time_slot 18
pub const USEM_REG_TS_18_AS: c_uint = 0x300080;
// [RW 3] The arbitration scheme of time_slot 1
pub const USEM_REG_TS_1_AS: c_uint = 0x30003c;
// [RW 3] The arbitration scheme of time_slot 2
pub const USEM_REG_TS_2_AS: c_uint = 0x300040;
// [RW 3] The arbitration scheme of time_slot 3
pub const USEM_REG_TS_3_AS: c_uint = 0x300044;
// [RW 3] The arbitration scheme of time_slot 4
pub const USEM_REG_TS_4_AS: c_uint = 0x300048;
// [RW 3] The arbitration scheme of time_slot 5
pub const USEM_REG_TS_5_AS: c_uint = 0x30004c;
// [RW 3] The arbitration scheme of time_slot 6
pub const USEM_REG_TS_6_AS: c_uint = 0x300050;
// [RW 3] The arbitration scheme of time_slot 7
pub const USEM_REG_TS_7_AS: c_uint = 0x300054;
// [RW 3] The arbitration scheme of time_slot 8
pub const USEM_REG_TS_8_AS: c_uint = 0x300058;
// [RW 3] The arbitration scheme of time_slot 9
pub const USEM_REG_TS_9_AS: c_uint = 0x30005c;
// [RW 32] Interrupt mask register #0 read/write
pub const USEM_REG_USEM_INT_MASK_0: c_uint = 0x300110;
pub const USEM_REG_USEM_INT_MASK_1: c_uint = 0x300120;
// [R 32] Interrupt register #0 read
pub const USEM_REG_USEM_INT_STS_0: c_uint = 0x300104;
pub const USEM_REG_USEM_INT_STS_1: c_uint = 0x300114;
// [RW 32] Parity mask register #0 read/write
pub const USEM_REG_USEM_PRTY_MASK_0: c_uint = 0x300130;
pub const USEM_REG_USEM_PRTY_MASK_1: c_uint = 0x300140;
// [R 32] Parity register #0 read
pub const USEM_REG_USEM_PRTY_STS_0: c_uint = 0x300124;
pub const USEM_REG_USEM_PRTY_STS_1: c_uint = 0x300134;
// [RC 32] Parity register #0 read clear
pub const USEM_REG_USEM_PRTY_STS_CLR_0: c_uint = 0x300128;
pub const USEM_REG_USEM_PRTY_STS_CLR_1: c_uint = 0x300138;
// [W 7] VF or PF ID for reset error bit. Values 0-63 reset error bit for 64
// VF; values 64-67 reset error for 4 PF; values 68-127 are not valid.
pub const USEM_REG_VFPF_ERR_NUM: c_uint = 0x300380;

pub const VFC_REG_MEMORIES_RST: c_uint = 0x1943c;
// [RW 32] Indirect access to AG context with 32-bits granularity. The bits
// [12:8] of the address should be the offset within the accessed LCID
// context; the bits [7:0] are the accessed LCID.Example: to write to REG10
// LCID100. The RBC address should be 13'ha64.
pub const XCM_REG_AG_CTX: c_uint = 0x28000;
// [RW 2] The queue index for registration on Aux1 counter flag.
pub const XCM_REG_AUX1_Q: c_uint = 0x20134;
// [RW 2] Per each decision rule the queue index to register to.
pub const XCM_REG_AUX_CNT_FLG_Q_19: c_uint = 0x201b0;
// [R 5] Used to read the XX protection CAM occupancy counter.
pub const XCM_REG_CAM_OCCUP: c_uint = 0x20244;
// [RW 1] CDU AG read Interface enable. If 0 - the request input is
pub const XCM_REG_CDU_AG_RD_IFEN: c_uint = 0x20044;
// [RW 1] CDU AG write Interface enable. If 0 - the request and valid input
pub const XCM_REG_CDU_AG_WR_IFEN: c_uint = 0x20040;
// [RW 1] CDU STORM read Interface enable. If 0 - the request input is
pub const XCM_REG_CDU_SM_RD_IFEN: c_uint = 0x2004c;
// [RW 1] CDU STORM write Interface enable. If 0 - the request and valid
pub const XCM_REG_CDU_SM_WR_IFEN: c_uint = 0x20048;
// [RW 4] CFC output initial credit. Max credit available - 15.Write writes
pub const XCM_REG_CFC_INIT_CRD: c_uint = 0x20404;
// [RW 3] The weight of the CP input in the WRR mechanism. 0 stands for
pub const XCM_REG_CP_WEIGHT: c_uint = 0x200dc;
// [RW 1] Input csem Interface enable. If 0 - the valid input is
pub const XCM_REG_CSEM_IFEN: c_uint = 0x20028;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_CSEM_LENGTH_MIS: c_uint = 0x20228;
// [RW 3] The weight of the input csem in the WRR mechanism. 0 stands for
pub const XCM_REG_CSEM_WEIGHT: c_uint = 0x200c4;
// [RW 1] Input dorq Interface enable. If 0 - the valid input is
pub const XCM_REG_DORQ_IFEN: c_uint = 0x20030;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_DORQ_LENGTH_MIS: c_uint = 0x20230;
// [RW 3] The weight of the input dorq in the WRR mechanism. 0 stands for
pub const XCM_REG_DORQ_WEIGHT: c_uint = 0x200cc;
// [RW 8] The Event ID in case the ErrorFlg input message bit is set.
pub const XCM_REG_ERR_EVNT_ID: c_uint = 0x200b0;
// [RW 28] The CM erroneous header for QM and Timers formatting.
pub const XCM_REG_ERR_XCM_HDR: c_uint = 0x200ac;
// [RW 8] The Event ID for Timers expiration.
pub const XCM_REG_EXPR_EVNT_ID: c_uint = 0x200b4;
// [RW 8] FIC0 output initial credit. Max credit available - 255.Write
pub const XCM_REG_FIC0_INIT_CRD: c_uint = 0x2040c;
// [RW 8] FIC1 output initial credit. Max credit available - 255.Write
pub const XCM_REG_FIC1_INIT_CRD: c_uint = 0x20410;
pub const XCM_REG_GLB_DEL_ACK_MAX_CNT_0: c_uint = 0x20118;
pub const XCM_REG_GLB_DEL_ACK_MAX_CNT_1: c_uint = 0x2011c;
pub const XCM_REG_GLB_DEL_ACK_TMR_VAL_0: c_uint = 0x20108;
pub const XCM_REG_GLB_DEL_ACK_TMR_VAL_1: c_uint = 0x2010c;
// [RW 1] Arbitratiojn between Input Arbiter groups: 0 - fair Round-Robin; 1
pub const XCM_REG_GR_ARB_TYPE: c_uint = 0x2020c;
// [RW 2] Load (FIC0) channel group priority. The lowest priority is 0; the
pub const XCM_REG_GR_LD0_PR: c_uint = 0x20214;
// [RW 2] Load (FIC1) channel group priority. The lowest priority is 0; the
pub const XCM_REG_GR_LD1_PR: c_uint = 0x20218;
// [RW 1] Input nig0 Interface enable. If 0 - the valid input is
pub const XCM_REG_NIG0_IFEN: c_uint = 0x20038;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_NIG0_LENGTH_MIS: c_uint = 0x20238;
// [RW 3] The weight of the input nig0 in the WRR mechanism. 0 stands for
pub const XCM_REG_NIG0_WEIGHT: c_uint = 0x200d4;
// [RW 1] Input nig1 Interface enable. If 0 - the valid input is
pub const XCM_REG_NIG1_IFEN: c_uint = 0x2003c;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_NIG1_LENGTH_MIS: c_uint = 0x2023c;
// [RW 5] The number of double REG-pairs; loaded from the STORM context and
pub const XCM_REG_N_SM_CTX_LD_0: c_uint = 0x20060;
pub const XCM_REG_N_SM_CTX_LD_1: c_uint = 0x20064;
pub const XCM_REG_N_SM_CTX_LD_2: c_uint = 0x20068;
pub const XCM_REG_N_SM_CTX_LD_3: c_uint = 0x2006c;
pub const XCM_REG_N_SM_CTX_LD_4: c_uint = 0x20070;
pub const XCM_REG_N_SM_CTX_LD_5: c_uint = 0x20074;
// [RW 1] Input pbf Interface enable. If 0 - the valid input is disregarded;
pub const XCM_REG_PBF_IFEN: c_uint = 0x20034;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_PBF_LENGTH_MIS: c_uint = 0x20234;
// [RW 3] The weight of the input pbf in the WRR mechanism. 0 stands for
pub const XCM_REG_PBF_WEIGHT: c_uint = 0x200d0;
pub const XCM_REG_PHYS_QNUM3_0: c_uint = 0x20100;
pub const XCM_REG_PHYS_QNUM3_1: c_uint = 0x20104;
// [RW 8] The Event ID for Timers formatting in case of stop done.
pub const XCM_REG_STOP_EVNT_ID: c_uint = 0x200b8;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_STORM_LENGTH_MIS: c_uint = 0x2021c;
// [RW 3] The weight of the STORM input in the WRR mechanism. 0 stands for
pub const XCM_REG_STORM_WEIGHT: c_uint = 0x200bc;
// [RW 1] STORM - CM Interface enable. If 0 - the valid input is
pub const XCM_REG_STORM_XCM_IFEN: c_uint = 0x20010;
// [RW 4] Timers output initial credit. Max credit available - 15.Write
pub const XCM_REG_TM_INIT_CRD: c_uint = 0x2041c;
// [RW 3] The weight of the Timers input in the WRR mechanism. 0 stands for
pub const XCM_REG_TM_WEIGHT: c_uint = 0x200ec;
// [RW 28] The CM header for Timers expiration command.
pub const XCM_REG_TM_XCM_HDR: c_uint = 0x200a8;
// [RW 1] Timers - CM Interface enable. If 0 - the valid input is
pub const XCM_REG_TM_XCM_IFEN: c_uint = 0x2001c;
// [RW 1] Input tsem Interface enable. If 0 - the valid input is
pub const XCM_REG_TSEM_IFEN: c_uint = 0x20024;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_TSEM_LENGTH_MIS: c_uint = 0x20224;
// [RW 3] The weight of the input tsem in the WRR mechanism. 0 stands for
pub const XCM_REG_TSEM_WEIGHT: c_uint = 0x200c0;
// [RW 2] The queue index for registration on UNA greater NXT decision rule.
pub const XCM_REG_UNA_GT_NXT_Q: c_uint = 0x20120;
// [RW 1] Input usem Interface enable. If 0 - the valid input is
pub const XCM_REG_USEM_IFEN: c_uint = 0x2002c;
// [RC 1] Message length mismatch (relative to last indication) at the usem
pub const XCM_REG_USEM_LENGTH_MIS: c_uint = 0x2022c;
// [RW 3] The weight of the input usem in the WRR mechanism. 0 stands for
pub const XCM_REG_USEM_WEIGHT: c_uint = 0x200c8;
pub const XCM_REG_WU_DA_CNT_CMD00: c_uint = 0x201d4;
pub const XCM_REG_WU_DA_CNT_CMD01: c_uint = 0x201d8;
pub const XCM_REG_WU_DA_CNT_CMD10: c_uint = 0x201dc;
pub const XCM_REG_WU_DA_CNT_CMD11: c_uint = 0x201e0;
pub const XCM_REG_WU_DA_CNT_UPD_VAL00: c_uint = 0x201e4;
pub const XCM_REG_WU_DA_CNT_UPD_VAL01: c_uint = 0x201e8;
pub const XCM_REG_WU_DA_CNT_UPD_VAL10: c_uint = 0x201ec;
pub const XCM_REG_WU_DA_CNT_UPD_VAL11: c_uint = 0x201f0;
pub const XCM_REG_WU_DA_SET_TMR_CNT_FLG_CMD00: c_uint = 0x201c4;
pub const XCM_REG_WU_DA_SET_TMR_CNT_FLG_CMD01: c_uint = 0x201c8;
pub const XCM_REG_WU_DA_SET_TMR_CNT_FLG_CMD10: c_uint = 0x201cc;
pub const XCM_REG_WU_DA_SET_TMR_CNT_FLG_CMD11: c_uint = 0x201d0;
// [RW 1] CM - CFC Interface enable. If 0 - the valid input is disregarded;
pub const XCM_REG_XCM_CFC_IFEN: c_uint = 0x20050;
// [RW 14] Interrupt mask register #0 read/write
pub const XCM_REG_XCM_INT_MASK: c_uint = 0x202b4;
// [R 14] Interrupt register #0 read
pub const XCM_REG_XCM_INT_STS: c_uint = 0x202a8;
// [RW 30] Parity mask register #0 read/write
pub const XCM_REG_XCM_PRTY_MASK: c_uint = 0x202c4;
// [R 30] Parity register #0 read
pub const XCM_REG_XCM_PRTY_STS: c_uint = 0x202b8;
// [RC 30] Parity register #0 read clear
pub const XCM_REG_XCM_PRTY_STS_CLR: c_uint = 0x202bc;
// [RW 4] The size of AG context region 0 in REG-pairs. Designates the MS
pub const XCM_REG_XCM_REG0_SZ: c_uint = 0x200f4;
// [RW 1] CM - STORM 0 Interface enable. If 0 - the acknowledge input is
pub const XCM_REG_XCM_STORM0_IFEN: c_uint = 0x20004;
// [RW 1] CM - STORM 1 Interface enable. If 0 - the acknowledge input is
pub const XCM_REG_XCM_STORM1_IFEN: c_uint = 0x20008;
// [RW 1] CM - Timers Interface enable. If 0 - the valid input is
pub const XCM_REG_XCM_TM_IFEN: c_uint = 0x20020;
// [RW 1] CM - QM Interface enable. If 0 - the acknowledge input is
pub const XCM_REG_XCM_XQM_IFEN: c_uint = 0x2000c;
// [RW 1] If set the Q index; received from the QM is inserted to event ID.
pub const XCM_REG_XCM_XQM_USE_Q: c_uint = 0x200f0;
// [RW 4] The value by which CFC updates the activity counter at QM bypass.
pub const XCM_REG_XQM_BYP_ACT_UPD: c_uint = 0x200fc;
// [RW 6] QM output initial credit. Max credit available - 32.Write writes
pub const XCM_REG_XQM_INIT_CRD: c_uint = 0x20420;
// [RW 3] The weight of the QM (primary) input in the WRR mechanism. 0
pub const XCM_REG_XQM_P_WEIGHT: c_uint = 0x200e4;
// [RW 3] The weight of the QM (secondary) input in the WRR mechanism. 0
pub const XCM_REG_XQM_S_WEIGHT: c_uint = 0x200e8;
// [RW 28] The CM header value for QM request (primary).
pub const XCM_REG_XQM_XCM_HDR_P: c_uint = 0x200a0;
// [RW 28] The CM header value for QM request (secondary).
pub const XCM_REG_XQM_XCM_HDR_S: c_uint = 0x200a4;
// [RW 1] QM - CM Interface enable. If 0 - the valid input is disregarded;
pub const XCM_REG_XQM_XCM_IFEN: c_uint = 0x20014;
// [RW 1] Input SDM Interface enable. If 0 - the valid input is disregarded;
pub const XCM_REG_XSDM_IFEN: c_uint = 0x20018;
// [RC 1] Set at message length mismatch (relative to last indication) at
pub const XCM_REG_XSDM_LENGTH_MIS: c_uint = 0x20220;
// [RW 3] The weight of the SDM input in the WRR mechanism. 0 stands for
pub const XCM_REG_XSDM_WEIGHT: c_uint = 0x200e0;
// [RW 17] Indirect access to the descriptor table of the XX protection
pub const XCM_REG_XX_DESCR_TABLE: c_uint = 0x20480;
pub const XCM_REG_XX_DESCR_TABLE_SIZE: c_int = 32;
// [R 6] Used to read the XX protection Free counter.
pub const XCM_REG_XX_FREE: c_uint = 0x20240;
// [RW 6] Initial value for the credit counter; responsible for fulfilling
pub const XCM_REG_XX_INIT_CRD: c_uint = 0x20424;
// [RW 6] The maximum number of pending messages; which may be stored in XX
pub const XCM_REG_XX_MSG_NUM: c_uint = 0x20428;
// [RW 8] The Event ID; sent to the STORM in case of XX overflow.
pub const XCM_REG_XX_OVFL_EVNT_ID: c_uint = 0x20058;

pub const XMAC_REG_CLEAR_RX_LSS_STATUS: c_uint = 0x60;
pub const XMAC_REG_CTRL: c_int = 0;
// [RW 16] Upper 48 bits of ctrl_sa register. Used as the SA in PAUSE/PFC
// packets transmitted by the MAC
pub const XMAC_REG_CTRL_SA_HI: c_uint = 0x2c;
// [RW 32] Lower 48 bits of ctrl_sa register. Used as the SA in PAUSE/PFC
// packets transmitted by the MAC
pub const XMAC_REG_CTRL_SA_LO: c_uint = 0x28;
pub const XMAC_REG_EEE_CTRL: c_uint = 0xd8;
pub const XMAC_REG_EEE_TIMERS_HI: c_uint = 0xe4;
pub const XMAC_REG_PAUSE_CTRL: c_uint = 0x68;
pub const XMAC_REG_PFC_CTRL: c_uint = 0x70;
pub const XMAC_REG_PFC_CTRL_HI: c_uint = 0x74;
pub const XMAC_REG_RX_LSS_CTRL: c_uint = 0x50;
pub const XMAC_REG_RX_LSS_STATUS: c_uint = 0x58;
// [RW 14] Maximum packet size in receive direction; exclusive of preamble &
// CRC in strip mode
pub const XMAC_REG_RX_MAX_SIZE: c_uint = 0x40;
pub const XMAC_REG_TX_CTRL: c_uint = 0x20;

// [RW 16] Indirect access to the XX table of the XX protection mechanism.
pub const XCM_REG_XX_TABLE: c_uint = 0x20500;
// [RW 8] The event id for aggregated interrupt 0
pub const XSDM_REG_AGG_INT_EVENT_0: c_uint = 0x166038;
pub const XSDM_REG_AGG_INT_EVENT_1: c_uint = 0x16603c;
pub const XSDM_REG_AGG_INT_EVENT_10: c_uint = 0x166060;
pub const XSDM_REG_AGG_INT_EVENT_11: c_uint = 0x166064;
pub const XSDM_REG_AGG_INT_EVENT_12: c_uint = 0x166068;
pub const XSDM_REG_AGG_INT_EVENT_13: c_uint = 0x16606c;
pub const XSDM_REG_AGG_INT_EVENT_14: c_uint = 0x166070;
pub const XSDM_REG_AGG_INT_EVENT_2: c_uint = 0x166040;
pub const XSDM_REG_AGG_INT_EVENT_3: c_uint = 0x166044;
pub const XSDM_REG_AGG_INT_EVENT_4: c_uint = 0x166048;
pub const XSDM_REG_AGG_INT_EVENT_5: c_uint = 0x16604c;
pub const XSDM_REG_AGG_INT_EVENT_6: c_uint = 0x166050;
pub const XSDM_REG_AGG_INT_EVENT_7: c_uint = 0x166054;
pub const XSDM_REG_AGG_INT_EVENT_8: c_uint = 0x166058;
pub const XSDM_REG_AGG_INT_EVENT_9: c_uint = 0x16605c;
// [RW 1] For each aggregated interrupt index whether the mode is normal (0)
pub const XSDM_REG_AGG_INT_MODE_0: c_uint = 0x1661b8;
pub const XSDM_REG_AGG_INT_MODE_1: c_uint = 0x1661bc;
// [RW 13] The start address in the internal RAM for the cfc_rsp lcid
pub const XSDM_REG_CFC_RSP_START_ADDR: c_uint = 0x166008;
// [RW 16] The maximum value of the completion counter #0
pub const XSDM_REG_CMP_COUNTER_MAX0: c_uint = 0x16601c;
// [RW 16] The maximum value of the completion counter #1
pub const XSDM_REG_CMP_COUNTER_MAX1: c_uint = 0x166020;
// [RW 16] The maximum value of the completion counter #2
pub const XSDM_REG_CMP_COUNTER_MAX2: c_uint = 0x166024;
// [RW 16] The maximum value of the completion counter #3
pub const XSDM_REG_CMP_COUNTER_MAX3: c_uint = 0x166028;
// [RW 13] The start address in the internal RAM for the completion
pub const XSDM_REG_CMP_COUNTER_START_ADDR: c_uint = 0x16600c;
pub const XSDM_REG_ENABLE_IN1: c_uint = 0x166238;
pub const XSDM_REG_ENABLE_IN2: c_uint = 0x16623c;
pub const XSDM_REG_ENABLE_OUT1: c_uint = 0x166240;
pub const XSDM_REG_ENABLE_OUT2: c_uint = 0x166244;
// [RW 4] The initial number of messages that can be sent to the pxp control
pub const XSDM_REG_INIT_CREDIT_PXP_CTRL: c_uint = 0x1664bc;
// [ST 32] The number of ACK after placement messages received
pub const XSDM_REG_NUM_OF_ACK_AFTER_PLACE: c_uint = 0x16627c;
// [ST 32] The number of packet end messages received from the parser
pub const XSDM_REG_NUM_OF_PKT_END_MSG: c_uint = 0x166274;
// [ST 32] The number of requests received from the pxp async if
pub const XSDM_REG_NUM_OF_PXP_ASYNC_REQ: c_uint = 0x166278;
// [ST 32] The number of commands received in queue 0
pub const XSDM_REG_NUM_OF_Q0_CMD: c_uint = 0x166248;
// [ST 32] The number of commands received in queue 10
pub const XSDM_REG_NUM_OF_Q10_CMD: c_uint = 0x16626c;
// [ST 32] The number of commands received in queue 11
pub const XSDM_REG_NUM_OF_Q11_CMD: c_uint = 0x166270;
// [ST 32] The number of commands received in queue 1
pub const XSDM_REG_NUM_OF_Q1_CMD: c_uint = 0x16624c;
// [ST 32] The number of commands received in queue 3
pub const XSDM_REG_NUM_OF_Q3_CMD: c_uint = 0x166250;
// [ST 32] The number of commands received in queue 4
pub const XSDM_REG_NUM_OF_Q4_CMD: c_uint = 0x166254;
// [ST 32] The number of commands received in queue 5
pub const XSDM_REG_NUM_OF_Q5_CMD: c_uint = 0x166258;
// [ST 32] The number of commands received in queue 6
pub const XSDM_REG_NUM_OF_Q6_CMD: c_uint = 0x16625c;
// [ST 32] The number of commands received in queue 7
pub const XSDM_REG_NUM_OF_Q7_CMD: c_uint = 0x166260;
// [ST 32] The number of commands received in queue 8
pub const XSDM_REG_NUM_OF_Q8_CMD: c_uint = 0x166264;
// [ST 32] The number of commands received in queue 9
pub const XSDM_REG_NUM_OF_Q9_CMD: c_uint = 0x166268;
// [RW 13] The start address in the internal RAM for queue counters
pub const XSDM_REG_Q_COUNTER_START_ADDR: c_uint = 0x166010;
// [W 17] Generate an operation after completion; bit-16 is
// AggVectIdx_valid; bits 15:8 are AggVectIdx; bits 7:5 are the TRIG and
// bits 4:0 are the T124Param[4:0]
pub const XSDM_REG_OPERATION_GEN: c_uint = 0x1664c4;
// [R 1] pxp_ctrl rd_data fifo empty in sdm_dma_rsp block
pub const XSDM_REG_RSP_PXP_CTRL_RDATA_EMPTY: c_uint = 0x166548;
// [R 1] parser fifo empty in sdm_sync block
pub const XSDM_REG_SYNC_PARSER_EMPTY: c_uint = 0x166550;
// [R 1] parser serial fifo empty in sdm_sync block
pub const XSDM_REG_SYNC_SYNC_EMPTY: c_uint = 0x166558;
// [RW 32] Tick for timer counter. Applicable only when
pub const XSDM_REG_TIMER_TICK: c_uint = 0x166000;
// [RW 32] Interrupt mask register #0 read/write
pub const XSDM_REG_XSDM_INT_MASK_0: c_uint = 0x16629c;
pub const XSDM_REG_XSDM_INT_MASK_1: c_uint = 0x1662ac;
// [R 32] Interrupt register #0 read
pub const XSDM_REG_XSDM_INT_STS_0: c_uint = 0x166290;
pub const XSDM_REG_XSDM_INT_STS_1: c_uint = 0x1662a0;
// [RW 11] Parity mask register #0 read/write
pub const XSDM_REG_XSDM_PRTY_MASK: c_uint = 0x1662bc;
// [R 11] Parity register #0 read
pub const XSDM_REG_XSDM_PRTY_STS: c_uint = 0x1662b0;
// [RC 11] Parity register #0 read clear
pub const XSDM_REG_XSDM_PRTY_STS_CLR: c_uint = 0x1662b4;
// [RW 5] The number of time_slots in the arbitration cycle
pub const XSEM_REG_ARB_CYCLE_SIZE: c_uint = 0x280034;
// [RW 3] The source that is associated with arbitration element 0. Source
pub const XSEM_REG_ARB_ELEMENT0: c_uint = 0x280020;
// [RW 3] The source that is associated with arbitration element 1. Source
pub const XSEM_REG_ARB_ELEMENT1: c_uint = 0x280024;
// [RW 3] The source that is associated with arbitration element 2. Source
pub const XSEM_REG_ARB_ELEMENT2: c_uint = 0x280028;
// [RW 3] The source that is associated with arbitration element 3. Source
pub const XSEM_REG_ARB_ELEMENT3: c_uint = 0x28002c;
// [RW 3] The source that is associated with arbitration element 4. Source
pub const XSEM_REG_ARB_ELEMENT4: c_uint = 0x280030;
pub const XSEM_REG_ENABLE_IN: c_uint = 0x2800a4;
pub const XSEM_REG_ENABLE_OUT: c_uint = 0x2800a8;
// [RW 32] This address space contains all registers and memories that are
pub const XSEM_REG_FAST_MEMORY: c_uint = 0x2a0000;
// [RW 1] Disables input messages from FIC0 May be updated during run_time
pub const XSEM_REG_FIC0_DISABLE: c_uint = 0x280224;
// [RW 1] Disables input messages from FIC1 May be updated during run_time
pub const XSEM_REG_FIC1_DISABLE: c_uint = 0x280234;
// [RW 15] Interrupt table Read and write access to it is not possible in
pub const XSEM_REG_INT_TABLE: c_uint = 0x280400;
// [ST 24] Statistics register. The number of messages that entered through
pub const XSEM_REG_MSG_NUM_FIC0: c_uint = 0x280000;
// [ST 24] Statistics register. The number of messages that entered through
pub const XSEM_REG_MSG_NUM_FIC1: c_uint = 0x280004;
// [ST 24] Statistics register. The number of messages that were sent to
pub const XSEM_REG_MSG_NUM_FOC0: c_uint = 0x280008;
// [ST 24] Statistics register. The number of messages that were sent to
pub const XSEM_REG_MSG_NUM_FOC1: c_uint = 0x28000c;
// [ST 24] Statistics register. The number of messages that were sent to
pub const XSEM_REG_MSG_NUM_FOC2: c_uint = 0x280010;
// [ST 24] Statistics register. The number of messages that were sent to
pub const XSEM_REG_MSG_NUM_FOC3: c_uint = 0x280014;
// [RW 1] Disables input messages from the passive buffer May be updated
pub const XSEM_REG_PAS_DISABLE: c_uint = 0x28024c;
// [WB 128] Debug only. Passive buffer memory
pub const XSEM_REG_PASSIVE_BUFFER: c_uint = 0x282000;
// [WB 46] pram memory. B45 is parity; b[44:0] - data.
pub const XSEM_REG_PRAM: c_uint = 0x2c0000;
// [R 16] Valid sleeping threads indication have bit per thread
pub const XSEM_REG_SLEEP_THREADS_VALID: c_uint = 0x28026c;
// [R 1] EXT_STORE FIFO is empty in sem_slow_ls_ext
pub const XSEM_REG_SLOW_EXT_STORE_EMPTY: c_uint = 0x2802a0;
// [RW 16] List of free threads . There is a bit per thread.
pub const XSEM_REG_THREADS_LIST: c_uint = 0x2802e4;
// [RW 3] The arbitration scheme of time_slot 0
pub const XSEM_REG_TS_0_AS: c_uint = 0x280038;
// [RW 3] The arbitration scheme of time_slot 10
pub const XSEM_REG_TS_10_AS: c_uint = 0x280060;
// [RW 3] The arbitration scheme of time_slot 11
pub const XSEM_REG_TS_11_AS: c_uint = 0x280064;
// [RW 3] The arbitration scheme of time_slot 12
pub const XSEM_REG_TS_12_AS: c_uint = 0x280068;
// [RW 3] The arbitration scheme of time_slot 13
pub const XSEM_REG_TS_13_AS: c_uint = 0x28006c;
// [RW 3] The arbitration scheme of time_slot 14
pub const XSEM_REG_TS_14_AS: c_uint = 0x280070;
// [RW 3] The arbitration scheme of time_slot 15
pub const XSEM_REG_TS_15_AS: c_uint = 0x280074;
// [RW 3] The arbitration scheme of time_slot 16
pub const XSEM_REG_TS_16_AS: c_uint = 0x280078;
// [RW 3] The arbitration scheme of time_slot 17
pub const XSEM_REG_TS_17_AS: c_uint = 0x28007c;
// [RW 3] The arbitration scheme of time_slot 18
pub const XSEM_REG_TS_18_AS: c_uint = 0x280080;
// [RW 3] The arbitration scheme of time_slot 1
pub const XSEM_REG_TS_1_AS: c_uint = 0x28003c;
// [RW 3] The arbitration scheme of time_slot 2
pub const XSEM_REG_TS_2_AS: c_uint = 0x280040;
// [RW 3] The arbitration scheme of time_slot 3
pub const XSEM_REG_TS_3_AS: c_uint = 0x280044;
// [RW 3] The arbitration scheme of time_slot 4
pub const XSEM_REG_TS_4_AS: c_uint = 0x280048;
// [RW 3] The arbitration scheme of time_slot 5
pub const XSEM_REG_TS_5_AS: c_uint = 0x28004c;
// [RW 3] The arbitration scheme of time_slot 6
pub const XSEM_REG_TS_6_AS: c_uint = 0x280050;
// [RW 3] The arbitration scheme of time_slot 7
pub const XSEM_REG_TS_7_AS: c_uint = 0x280054;
// [RW 3] The arbitration scheme of time_slot 8
pub const XSEM_REG_TS_8_AS: c_uint = 0x280058;
// [RW 3] The arbitration scheme of time_slot 9
pub const XSEM_REG_TS_9_AS: c_uint = 0x28005c;
// [W 7] VF or PF ID for reset error bit. Values 0-63 reset error bit for 64
// VF; values 64-67 reset error for 4 PF; values 68-127 are not valid.
pub const XSEM_REG_VFPF_ERR_NUM: c_uint = 0x280380;
// [RW 32] Interrupt mask register #0 read/write
pub const XSEM_REG_XSEM_INT_MASK_0: c_uint = 0x280110;
pub const XSEM_REG_XSEM_INT_MASK_1: c_uint = 0x280120;
// [R 32] Interrupt register #0 read
pub const XSEM_REG_XSEM_INT_STS_0: c_uint = 0x280104;
pub const XSEM_REG_XSEM_INT_STS_1: c_uint = 0x280114;
// [RW 32] Parity mask register #0 read/write
pub const XSEM_REG_XSEM_PRTY_MASK_0: c_uint = 0x280130;
pub const XSEM_REG_XSEM_PRTY_MASK_1: c_uint = 0x280140;
// [R 32] Parity register #0 read
pub const XSEM_REG_XSEM_PRTY_STS_0: c_uint = 0x280124;
pub const XSEM_REG_XSEM_PRTY_STS_1: c_uint = 0x280134;
// [RC 32] Parity register #0 read clear
pub const XSEM_REG_XSEM_PRTY_STS_CLR_0: c_uint = 0x280128;
pub const XSEM_REG_XSEM_PRTY_STS_CLR_1: c_uint = 0x280138;

pub const EMAC_MDIO_MODE_CLOCK_CNT_BITSHIFT: c_int = 16;

pub const EMAC_REG_EMAC_LED: c_uint = 0xc;
pub const EMAC_REG_EMAC_MAC_MATCH: c_uint = 0x10;
pub const EMAC_REG_EMAC_MDIO_COMM: c_uint = 0xac;
pub const EMAC_REG_EMAC_MDIO_MODE: c_uint = 0xb4;
pub const EMAC_REG_EMAC_MDIO_STATUS: c_uint = 0xb0;
pub const EMAC_REG_EMAC_MODE: c_uint = 0x0;
pub const EMAC_REG_EMAC_RX_MODE: c_uint = 0xc8;
pub const EMAC_REG_EMAC_RX_MTU_SIZE: c_uint = 0x9c;
pub const EMAC_REG_EMAC_RX_STAT_AC: c_uint = 0x180;
pub const EMAC_REG_EMAC_RX_STAT_AC_28: c_uint = 0x1f4;
pub const EMAC_REG_EMAC_RX_STAT_AC_COUNT: c_int = 23;
pub const EMAC_REG_EMAC_TX_MODE: c_uint = 0xbc;
pub const EMAC_REG_EMAC_TX_STAT_AC: c_uint = 0x280;
pub const EMAC_REG_EMAC_TX_STAT_AC_COUNT: c_int = 22;
pub const EMAC_REG_RX_PFC_MODE: c_uint = 0x320;

pub const EMAC_REG_RX_PFC_PARAM: c_uint = 0x324;
pub const EMAC_REG_RX_PFC_PARAM_OPCODE_BITSHIFT: c_int = 0;
pub const EMAC_REG_RX_PFC_PARAM_PRIORITY_EN_BITSHIFT: c_int = 16;
pub const EMAC_REG_RX_PFC_STATS_XOFF_RCVD: c_uint = 0x328;

pub const EMAC_REG_RX_PFC_STATS_XOFF_SENT: c_uint = 0x330;

pub const EMAC_REG_RX_PFC_STATS_XON_RCVD: c_uint = 0x32c;

pub const EMAC_REG_RX_PFC_STATS_XON_SENT: c_uint = 0x334;

pub const MISC_REGISTERS_GPIO_0: c_int = 0;
pub const MISC_REGISTERS_GPIO_1: c_int = 1;
pub const MISC_REGISTERS_GPIO_2: c_int = 2;
pub const MISC_REGISTERS_GPIO_3: c_int = 3;
pub const MISC_REGISTERS_GPIO_CLR_POS: c_int = 16;

pub const MISC_REGISTERS_GPIO_FLOAT_POS: c_int = 24;
pub const MISC_REGISTERS_GPIO_HIGH: c_int = 1;
pub const MISC_REGISTERS_GPIO_INPUT_HI_Z: c_int = 2;
pub const MISC_REGISTERS_GPIO_INT_CLR_POS: c_int = 24;
pub const MISC_REGISTERS_GPIO_INT_OUTPUT_CLR: c_int = 0;
pub const MISC_REGISTERS_GPIO_INT_OUTPUT_SET: c_int = 1;
pub const MISC_REGISTERS_GPIO_INT_SET_POS: c_int = 16;
pub const MISC_REGISTERS_GPIO_LOW: c_int = 0;
pub const MISC_REGISTERS_GPIO_OUTPUT_HIGH: c_int = 1;
pub const MISC_REGISTERS_GPIO_OUTPUT_LOW: c_int = 0;
pub const MISC_REGISTERS_GPIO_PORT_SHIFT: c_int = 4;
pub const MISC_REGISTERS_GPIO_SET_POS: c_int = 8;
pub const MISC_REGISTERS_RESET_REG_1_CLEAR: c_uint = 0x588;

pub const MISC_REGISTERS_RESET_REG_1_SET: c_uint = 0x584;
pub const MISC_REGISTERS_RESET_REG_2_CLEAR: c_uint = 0x598;

pub const MISC_REGISTERS_RESET_REG_2_SET: c_uint = 0x594;

pub const MISC_REGISTERS_RESET_REG_3_CLEAR: c_uint = 0x5a8;

pub const MISC_REGISTERS_RESET_REG_3_SET: c_uint = 0x5a4;
pub const MISC_REGISTERS_SPIO_4: c_int = 4;
pub const MISC_REGISTERS_SPIO_5: c_int = 5;
pub const MISC_REGISTERS_SPIO_7: c_int = 7;
pub const MISC_REGISTERS_SPIO_CLR_POS: c_int = 16;

pub const MISC_REGISTERS_SPIO_FLOAT_POS: c_int = 24;
pub const MISC_REGISTERS_SPIO_INPUT_HI_Z: c_int = 2;
pub const MISC_REGISTERS_SPIO_INT_OLD_SET_POS: c_int = 16;
pub const MISC_REGISTERS_SPIO_OUTPUT_HIGH: c_int = 1;
pub const MISC_REGISTERS_SPIO_OUTPUT_LOW: c_int = 0;
pub const MISC_REGISTERS_SPIO_SET_POS: c_int = 8;
pub const MISC_SPIO_CLR_POS: c_int = 16;

pub const MISC_SPIO_FLOAT_POS: c_int = 24;
pub const MISC_SPIO_INPUT_HI_Z: c_int = 2;
pub const MISC_SPIO_INT_OLD_SET_POS: c_int = 16;
pub const MISC_SPIO_OUTPUT_HIGH: c_int = 1;
pub const MISC_SPIO_OUTPUT_LOW: c_int = 0;
pub const MISC_SPIO_SET_POS: c_int = 8;
pub const MISC_SPIO_SPIO4: c_uint = 0x10;
pub const MISC_SPIO_SPIO5: c_uint = 0x20;
pub const HW_LOCK_MAX_RESOURCE_VALUE: c_int = 31;
pub const HW_LOCK_RESOURCE_DCBX_ADMIN_MIB: c_int = 13;
pub const HW_LOCK_RESOURCE_DRV_FLAGS: c_int = 10;
pub const HW_LOCK_RESOURCE_GPIO: c_int = 1;
pub const HW_LOCK_RESOURCE_MDIO: c_int = 0;
pub const HW_LOCK_RESOURCE_NVRAM: c_int = 12;
pub const HW_LOCK_RESOURCE_PORT0_ATT_MASK: c_int = 3;
pub const HW_LOCK_RESOURCE_RECOVERY_LEADER_0: c_int = 8;
pub const HW_LOCK_RESOURCE_RECOVERY_LEADER_1: c_int = 9;
pub const HW_LOCK_RESOURCE_RECOVERY_REG: c_int = 11;
pub const HW_LOCK_RESOURCE_RESET: c_int = 5;
pub const HW_LOCK_RESOURCE_SPIO: c_int = 2;

pub const RESERVED_GENERAL_ATTENTION_BIT_0: c_int = 0;
pub const EVEREST_GEN_ATTN_IN_USE_MASK: c_uint = 0x7ffe0;
pub const EVEREST_LATCHED_ATTN_IN_USE_MASK: c_uint = 0xffe00000;
pub const RESERVED_GENERAL_ATTENTION_BIT_6: c_int = 6;
pub const RESERVED_GENERAL_ATTENTION_BIT_7: c_int = 7;
pub const RESERVED_GENERAL_ATTENTION_BIT_8: c_int = 8;
pub const RESERVED_GENERAL_ATTENTION_BIT_9: c_int = 9;
pub const RESERVED_GENERAL_ATTENTION_BIT_10: c_int = 10;
pub const RESERVED_GENERAL_ATTENTION_BIT_11: c_int = 11;
pub const RESERVED_GENERAL_ATTENTION_BIT_12: c_int = 12;
pub const RESERVED_GENERAL_ATTENTION_BIT_13: c_int = 13;
pub const RESERVED_GENERAL_ATTENTION_BIT_14: c_int = 14;
pub const RESERVED_GENERAL_ATTENTION_BIT_15: c_int = 15;
pub const RESERVED_GENERAL_ATTENTION_BIT_16: c_int = 16;
pub const RESERVED_GENERAL_ATTENTION_BIT_17: c_int = 17;
pub const RESERVED_GENERAL_ATTENTION_BIT_18: c_int = 18;
pub const RESERVED_GENERAL_ATTENTION_BIT_19: c_int = 19;
pub const RESERVED_GENERAL_ATTENTION_BIT_20: c_int = 20;
pub const RESERVED_GENERAL_ATTENTION_BIT_21: c_int = 21;
// storm asserts attention bits

// mcp error attention bit

// E1H NIG status sync attention mapped to group 4-7

pub const LATCHED_ATTN_RBCR: c_int = 23;
pub const LATCHED_ATTN_RBCT: c_int = 24;
pub const LATCHED_ATTN_RBCN: c_int = 25;
pub const LATCHED_ATTN_RBCU: c_int = 26;
pub const LATCHED_ATTN_RBCP: c_int = 27;
pub const LATCHED_ATTN_TIMEOUT_GRC: c_int = 28;
pub const LATCHED_ATTN_RSVD_GRC: c_int = 29;
pub const LATCHED_ATTN_ROM_PARITY_MCP: c_int = 30;
pub const LATCHED_ATTN_UM_RX_PARITY_MCP: c_int = 31;
pub const LATCHED_ATTN_UM_TX_PARITY_MCP: c_int = 32;
pub const LATCHED_ATTN_SCPAD_PARITY_MCP: c_int = 33;

// Macro flag: #define GENERAL_ATTEN_OFFSET(atten_name)\
//
// This file defines GRC base address for every block.
// This file is included by chipsim, asm microcode and cpp microcode.
// These values are used in Design.xml on regBase attribute
// Use the base with the generated offsets of specific registers.
//
pub const GRCBASE_PXPCS: c_uint = 0x000000;
pub const GRCBASE_PCICONFIG: c_uint = 0x002000;
pub const GRCBASE_PCIREG: c_uint = 0x002400;
pub const GRCBASE_EMAC0: c_uint = 0x008000;
pub const GRCBASE_EMAC1: c_uint = 0x008400;
pub const GRCBASE_DBU: c_uint = 0x008800;
pub const GRCBASE_MISC: c_uint = 0x00A000;
pub const GRCBASE_DBG: c_uint = 0x00C000;
pub const GRCBASE_NIG: c_uint = 0x010000;
pub const GRCBASE_XCM: c_uint = 0x020000;
pub const GRCBASE_PRS: c_uint = 0x040000;
pub const GRCBASE_SRCH: c_uint = 0x040400;
pub const GRCBASE_TSDM: c_uint = 0x042000;
pub const GRCBASE_TCM: c_uint = 0x050000;
pub const GRCBASE_BRB1: c_uint = 0x060000;
pub const GRCBASE_MCP: c_uint = 0x080000;
pub const GRCBASE_UPB: c_uint = 0x0C1000;
pub const GRCBASE_CSDM: c_uint = 0x0C2000;
pub const GRCBASE_USDM: c_uint = 0x0C4000;
pub const GRCBASE_CCM: c_uint = 0x0D0000;
pub const GRCBASE_UCM: c_uint = 0x0E0000;
pub const GRCBASE_CDU: c_uint = 0x101000;
pub const GRCBASE_DMAE: c_uint = 0x102000;
pub const GRCBASE_PXP: c_uint = 0x103000;
pub const GRCBASE_CFC: c_uint = 0x104000;
pub const GRCBASE_HC: c_uint = 0x108000;
pub const GRCBASE_PXP2: c_uint = 0x120000;
pub const GRCBASE_PBF: c_uint = 0x140000;
pub const GRCBASE_UMAC0: c_uint = 0x160000;
pub const GRCBASE_UMAC1: c_uint = 0x160400;
pub const GRCBASE_XPB: c_uint = 0x161000;
pub const GRCBASE_MSTAT0: c_uint = 0x162000;
pub const GRCBASE_MSTAT1: c_uint = 0x162800;
pub const GRCBASE_XMAC0: c_uint = 0x163000;
pub const GRCBASE_XMAC1: c_uint = 0x163800;
pub const GRCBASE_TIMERS: c_uint = 0x164000;
pub const GRCBASE_XSDM: c_uint = 0x166000;
pub const GRCBASE_QM: c_uint = 0x168000;
pub const GRCBASE_DQ: c_uint = 0x170000;
pub const GRCBASE_TSEM: c_uint = 0x180000;
pub const GRCBASE_CSEM: c_uint = 0x200000;
pub const GRCBASE_XSEM: c_uint = 0x280000;
pub const GRCBASE_USEM: c_uint = 0x300000;

// offset of configuration space in the pci core register
pub const PCICFG_OFFSET: c_uint = 0x2000;
pub const PCICFG_VENDOR_ID_OFFSET: c_uint = 0x00;
pub const PCICFG_DEVICE_ID_OFFSET: c_uint = 0x02;
pub const PCICFG_COMMAND_OFFSET: c_uint = 0x04;

pub const PCICFG_STATUS_OFFSET: c_uint = 0x06;
pub const PCICFG_REVISION_ID_OFFSET: c_uint = 0x08;
pub const PCICFG_REVESION_ID_MASK: c_uint = 0xff;
pub const PCICFG_REVESION_ID_ERROR_VAL: c_uint = 0xff;
pub const PCICFG_CACHE_LINE_SIZE: c_uint = 0x0c;
pub const PCICFG_LATENCY_TIMER: c_uint = 0x0d;
pub const PCICFG_BAR_1_LOW: c_uint = 0x10;
pub const PCICFG_BAR_1_HIGH: c_uint = 0x14;
pub const PCICFG_BAR_2_LOW: c_uint = 0x18;
pub const PCICFG_BAR_2_HIGH: c_uint = 0x1c;
pub const PCICFG_SUBSYSTEM_VENDOR_ID_OFFSET: c_uint = 0x2c;
pub const PCICFG_SUBSYSTEM_ID_OFFSET: c_uint = 0x2e;
pub const PCICFG_INT_LINE: c_uint = 0x3c;
pub const PCICFG_INT_PIN: c_uint = 0x3d;
pub const PCICFG_PM_CAPABILITY: c_uint = 0x48;

pub const PCICFG_PM_CSR_OFFSET: c_uint = 0x4c;

pub const PCICFG_MSI_CAP_ID_OFFSET: c_uint = 0x58;

pub const PCICFG_GRC_ADDRESS: c_uint = 0x78;
pub const PCICFG_GRC_DATA: c_uint = 0x80;
pub const PCICFG_ME_REGISTER: c_uint = 0x98;
pub const PCICFG_MSIX_CAP_ID_OFFSET: c_uint = 0xa0;

pub const PCICFG_DEVICE_CONTROL: c_uint = 0xb4;
pub const PCICFG_DEVICE_STATUS: c_uint = 0xb6;

pub const PCICFG_LINK_CONTROL: c_uint = 0xbc;
pub const BAR_USTRORM_INTMEM: c_uint = 0x400000;
pub const BAR_CSTRORM_INTMEM: c_uint = 0x410000;
pub const BAR_XSTRORM_INTMEM: c_uint = 0x420000;
pub const BAR_TSTRORM_INTMEM: c_uint = 0x430000;
// for accessing the IGU in case of status block ACK
pub const BAR_IGU_INTMEM: c_uint = 0x440000;
pub const BAR_DOORBELL_OFFSET: c_uint = 0x800000;
pub const BAR_ME_REGISTER: c_uint = 0x450000;
// config_2 offset
pub const GRC_CONFIG_2_SIZE_REG: c_uint = 0x408;

// config_3 offset
pub const GRC_CONFIG_3_SIZE_REG: c_uint = 0x40c;

pub const GRC_BAR2_CONFIG: c_uint = 0x4e0;

pub const PCI_PM_DATA_A: c_uint = 0x410;
pub const PCI_PM_DATA_B: c_uint = 0x414;
pub const PCI_ID_VAL1: c_uint = 0x434;
pub const PCI_ID_VAL2: c_uint = 0x438;
pub const PCI_ID_VAL3: c_uint = 0x43c;
pub const GRC_CONFIG_REG_VF_MSIX_CONTROL: c_uint = 0x61C;
pub const GRC_CONFIG_REG_PF_INIT_VF: c_uint = 0x624;
pub const GRC_CR_PF_INIT_VF_PF_FIRST_VF_NUM_MASK: c_uint = 0xf;
// First VF_NUM for PF is encoded in this register.
// The number of VFs assigned to a PF is assumed to be a multiple of 8.
// Software should program these bits based on Total Number of VFs \
// programmed for each PF.
// Since registers from 0x000-0x7ff are split across functions, each PF will
// have the same location for the same 4 bits
//
pub const PXPCS_TL_CONTROL_5: c_uint = 0x814;

pub const PXPCS_TL_FUNC345_STAT: c_uint = 0x854;

// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNSPPORT4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_ECRC4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MALF_TLP4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_RX_OFLOW4\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNEXP_CPL4\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MASTER_ABRT4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_CPL_TIMEOUT4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_FC_PRTL4\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_PSND_TLP4\

// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNSPPORT3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_ECRC3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MALF_TLP3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_RX_OFLOW3\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNEXP_CPL3\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MASTER_ABRT3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_CPL_TIMEOUT3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_FC_PRTL3\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_PSND_TLP3\

// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNSPPORT2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_ECRC2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MALF_TLP2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_RX_OFLOW2\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_UNEXP_CPL2\
//
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_MASTER_ABRT2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_CPL_TIMEOUT2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_FC_PRTL2\
// Macro flag: #define PXPCS_TL_FUNC345_STAT_ERR_PSND_TLP2\
pub const PXPCS_TL_FUNC678_STAT: c_uint = 0x85C;

// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNSPPORT7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_ECRC7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MALF_TLP7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_RX_OFLOW7\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNEXP_CPL7\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MASTER_ABRT7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_CPL_TIMEOUT7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_FC_PRTL7\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_PSND_TLP7\

// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNSPPORT6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_ECRC6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MALF_TLP6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_RX_OFLOW6\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNEXP_CPL6\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MASTER_ABRT6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_CPL_TIMEOUT6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_FC_PRTL6\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_PSND_TLP6\

// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNSPPORT5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_ECRC5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MALF_TLP5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_RX_OFLOW5\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_UNEXP_CPL5\
//
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_MASTER_ABRT5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_CPL_TIMEOUT5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_FC_PRTL5\
// Macro flag: #define PXPCS_TL_FUNC678_STAT_ERR_PSND_TLP5\
pub const BAR_USTRORM_INTMEM: c_uint = 0x400000;
pub const BAR_CSTRORM_INTMEM: c_uint = 0x410000;
pub const BAR_XSTRORM_INTMEM: c_uint = 0x420000;
pub const BAR_TSTRORM_INTMEM: c_uint = 0x430000;
// for accessing the IGU in case of status block ACK
pub const BAR_IGU_INTMEM: c_uint = 0x440000;
pub const BAR_DOORBELL_OFFSET: c_uint = 0x800000;
pub const BAR_ME_REGISTER: c_uint = 0x450000;
pub const ME_REG_PF_NUM_SHIFT: c_int = 0;
// Macro flag: #define ME_REG_PF_NUM\

pub const ME_REG_VF_NUM_SHIFT: c_int = 9;

pub const ME_REG_ABS_PF_NUM_SHIFT: c_int = 16;
// Macro flag: #define ME_REG_ABS_PF_NUM\
pub const PXP_VF_ADDR_IGU_START: c_int = 0;
pub const PXP_VF_ADDR_IGU_SIZE: c_uint = 0x3000;
// Macro flag: #define PXP_VF_ADDR_IGU_END\
pub const PXP_VF_ADDR_USDM_QUEUES_START: c_uint = 0x3000;
// Macro flag: #define PXP_VF_ADDR_USDM_QUEUES_SIZE\
// Macro flag: #define PXP_VF_ADDR_USDM_QUEUES_END\
pub const PXP_VF_ADDR_CSDM_GLOBAL_START: c_uint = 0x7600;

// Macro flag: #define PXP_VF_ADDR_CSDM_GLOBAL_END\
pub const PXP_VF_ADDR_DB_START: c_uint = 0x7c00;
pub const PXP_VF_ADDR_DB_SIZE: c_uint = 0x200;
// Macro flag: #define PXP_VF_ADDR_DB_END\
pub const MDIO_REG_BANK_CL73_IEEEB0: c_uint = 0x0;
pub const MDIO_CL73_IEEEB0_CL73_AN_CONTROL: c_uint = 0x0;
pub const MDIO_CL73_IEEEB0_CL73_AN_CONTROL_RESTART_AN: c_uint = 0x0200;
pub const MDIO_CL73_IEEEB0_CL73_AN_CONTROL_AN_EN: c_uint = 0x1000;
pub const MDIO_CL73_IEEEB0_CL73_AN_CONTROL_MAIN_RST: c_uint = 0x8000;
pub const MDIO_REG_BANK_CL73_IEEEB1: c_uint = 0x10;
pub const MDIO_CL73_IEEEB1_AN_ADV1: c_uint = 0x00;
pub const MDIO_CL73_IEEEB1_AN_ADV1_PAUSE: c_uint = 0x0400;
pub const MDIO_CL73_IEEEB1_AN_ADV1_ASYMMETRIC: c_uint = 0x0800;
pub const MDIO_CL73_IEEEB1_AN_ADV1_PAUSE_BOTH: c_uint = 0x0C00;
pub const MDIO_CL73_IEEEB1_AN_ADV1_PAUSE_MASK: c_uint = 0x0C00;
pub const MDIO_CL73_IEEEB1_AN_ADV2: c_uint = 0x01;
pub const MDIO_CL73_IEEEB1_AN_ADV2_ADVR_1000M: c_uint = 0x0000;
pub const MDIO_CL73_IEEEB1_AN_ADV2_ADVR_1000M_KX: c_uint = 0x0020;
pub const MDIO_CL73_IEEEB1_AN_ADV2_ADVR_10G_KX4: c_uint = 0x0040;
pub const MDIO_CL73_IEEEB1_AN_ADV2_ADVR_10G_KR: c_uint = 0x0080;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV1: c_uint = 0x03;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV1_PAUSE: c_uint = 0x0400;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV1_ASYMMETRIC: c_uint = 0x0800;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV1_PAUSE_BOTH: c_uint = 0x0C00;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV1_PAUSE_MASK: c_uint = 0x0C00;
pub const MDIO_CL73_IEEEB1_AN_LP_ADV2: c_uint = 0x04;
pub const MDIO_REG_BANK_RX0: c_uint = 0x80b0;
pub const MDIO_RX0_RX_STATUS: c_uint = 0x10;
pub const MDIO_RX0_RX_STATUS_SIGDET: c_uint = 0x8000;
pub const MDIO_RX0_RX_STATUS_RX_SEQ_DONE: c_uint = 0x1000;
pub const MDIO_RX0_RX_EQ_BOOST: c_uint = 0x1c;
pub const MDIO_RX0_RX_EQ_BOOST_EQUALIZER_CTRL_MASK: c_uint = 0x7;
pub const MDIO_RX0_RX_EQ_BOOST_OFFSET_CTRL: c_uint = 0x10;
pub const MDIO_REG_BANK_RX1: c_uint = 0x80c0;
pub const MDIO_RX1_RX_EQ_BOOST: c_uint = 0x1c;
pub const MDIO_RX1_RX_EQ_BOOST_EQUALIZER_CTRL_MASK: c_uint = 0x7;
pub const MDIO_RX1_RX_EQ_BOOST_OFFSET_CTRL: c_uint = 0x10;
pub const MDIO_REG_BANK_RX2: c_uint = 0x80d0;
pub const MDIO_RX2_RX_EQ_BOOST: c_uint = 0x1c;
pub const MDIO_RX2_RX_EQ_BOOST_EQUALIZER_CTRL_MASK: c_uint = 0x7;
pub const MDIO_RX2_RX_EQ_BOOST_OFFSET_CTRL: c_uint = 0x10;
pub const MDIO_REG_BANK_RX3: c_uint = 0x80e0;
pub const MDIO_RX3_RX_EQ_BOOST: c_uint = 0x1c;
pub const MDIO_RX3_RX_EQ_BOOST_EQUALIZER_CTRL_MASK: c_uint = 0x7;
pub const MDIO_RX3_RX_EQ_BOOST_OFFSET_CTRL: c_uint = 0x10;
pub const MDIO_REG_BANK_RX_ALL: c_uint = 0x80f0;
pub const MDIO_RX_ALL_RX_EQ_BOOST: c_uint = 0x1c;
pub const MDIO_RX_ALL_RX_EQ_BOOST_EQUALIZER_CTRL_MASK: c_uint = 0x7;
pub const MDIO_RX_ALL_RX_EQ_BOOST_OFFSET_CTRL: c_uint = 0x10;
pub const MDIO_REG_BANK_TX0: c_uint = 0x8060;
pub const MDIO_TX0_TX_DRIVER: c_uint = 0x17;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_MASK: c_uint = 0xf000;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_SHIFT: c_int = 12;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_MASK: c_uint = 0x0f00;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_SHIFT: c_int = 8;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_MASK: c_uint = 0x00f0;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_SHIFT: c_int = 4;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_MASK: c_uint = 0x000e;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_SHIFT: c_int = 1;
pub const MDIO_TX0_TX_DRIVER_ICBUF1T: c_int = 1;
pub const MDIO_REG_BANK_TX1: c_uint = 0x8070;
pub const MDIO_TX1_TX_DRIVER: c_uint = 0x17;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_MASK: c_uint = 0xf000;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_SHIFT: c_int = 12;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_MASK: c_uint = 0x0f00;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_SHIFT: c_int = 8;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_MASK: c_uint = 0x00f0;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_SHIFT: c_int = 4;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_MASK: c_uint = 0x000e;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_SHIFT: c_int = 1;
pub const MDIO_TX0_TX_DRIVER_ICBUF1T: c_int = 1;
pub const MDIO_REG_BANK_TX2: c_uint = 0x8080;
pub const MDIO_TX2_TX_DRIVER: c_uint = 0x17;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_MASK: c_uint = 0xf000;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_SHIFT: c_int = 12;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_MASK: c_uint = 0x0f00;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_SHIFT: c_int = 8;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_MASK: c_uint = 0x00f0;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_SHIFT: c_int = 4;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_MASK: c_uint = 0x000e;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_SHIFT: c_int = 1;
pub const MDIO_TX0_TX_DRIVER_ICBUF1T: c_int = 1;
pub const MDIO_REG_BANK_TX3: c_uint = 0x8090;
pub const MDIO_TX3_TX_DRIVER: c_uint = 0x17;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_MASK: c_uint = 0xf000;
pub const MDIO_TX0_TX_DRIVER_PREEMPHASIS_SHIFT: c_int = 12;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_MASK: c_uint = 0x0f00;
pub const MDIO_TX0_TX_DRIVER_IDRIVER_SHIFT: c_int = 8;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_MASK: c_uint = 0x00f0;
pub const MDIO_TX0_TX_DRIVER_IPREDRIVER_SHIFT: c_int = 4;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_MASK: c_uint = 0x000e;
pub const MDIO_TX0_TX_DRIVER_IFULLSPD_SHIFT: c_int = 1;
pub const MDIO_TX0_TX_DRIVER_ICBUF1T: c_int = 1;
pub const MDIO_REG_BANK_XGXS_BLOCK0: c_uint = 0x8000;
pub const MDIO_BLOCK0_XGXS_CONTROL: c_uint = 0x10;
pub const MDIO_REG_BANK_XGXS_BLOCK1: c_uint = 0x8010;
pub const MDIO_BLOCK1_LANE_CTRL0: c_uint = 0x15;
pub const MDIO_BLOCK1_LANE_CTRL1: c_uint = 0x16;
pub const MDIO_BLOCK1_LANE_CTRL2: c_uint = 0x17;
pub const MDIO_BLOCK1_LANE_PRBS: c_uint = 0x19;
pub const MDIO_REG_BANK_XGXS_BLOCK2: c_uint = 0x8100;
pub const MDIO_XGXS_BLOCK2_RX_LN_SWAP: c_uint = 0x10;
pub const MDIO_XGXS_BLOCK2_RX_LN_SWAP_ENABLE: c_uint = 0x8000;
pub const MDIO_XGXS_BLOCK2_RX_LN_SWAP_FORCE_ENABLE: c_uint = 0x4000;
pub const MDIO_XGXS_BLOCK2_TX_LN_SWAP: c_uint = 0x11;
pub const MDIO_XGXS_BLOCK2_TX_LN_SWAP_ENABLE: c_uint = 0x8000;
pub const MDIO_XGXS_BLOCK2_UNICORE_MODE_10G: c_uint = 0x14;
pub const MDIO_XGXS_BLOCK2_UNICORE_MODE_10G_CX4_XGXS: c_uint = 0x0001;
pub const MDIO_XGXS_BLOCK2_UNICORE_MODE_10G_HIGIG_XGXS: c_uint = 0x0010;
pub const MDIO_XGXS_BLOCK2_TEST_MODE_LANE: c_uint = 0x15;
pub const MDIO_REG_BANK_GP_STATUS: c_uint = 0x8120;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1: c_uint = 0x1B;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_CL73_AUTONEG_COMPLETE: c_uint = 0x0001;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_CL37_AUTONEG_COMPLETE: c_uint = 0x0002;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_LINK_STATUS: c_uint = 0x0004;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_DUPLEX_STATUS: c_uint = 0x0008;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_CL73_MR_LP_NP_AN_ABLE: c_uint = 0x0010;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_CL73_LP_NP_BAM_ABLE: c_uint = 0x0020;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_PAUSE_RSOLUTION_TXSIDE: c_uint = 0x0040;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_PAUSE_RSOLUTION_RXSIDE: c_uint = 0x0080;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_MASK: c_uint = 0x3f00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10M: c_uint = 0x0000;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_100M: c_uint = 0x0100;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_1G: c_uint = 0x0200;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_2_5G: c_uint = 0x0300;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_5G: c_uint = 0x0400;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_6G: c_uint = 0x0500;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_HIG: c_uint = 0x0600;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_CX4: c_uint = 0x0700;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_12G_HIG: c_uint = 0x0800;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_12_5G: c_uint = 0x0900;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_13G: c_uint = 0x0A00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_15G: c_uint = 0x0B00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_16G: c_uint = 0x0C00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_1G_KX: c_uint = 0x0D00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_KX4: c_uint = 0x0E00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_KR: c_uint = 0x0F00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_XFI: c_uint = 0x1B00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_20G_DXGXS: c_uint = 0x1E00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_10G_SFI: c_uint = 0x1F00;
pub const MDIO_GP_STATUS_TOP_AN_STATUS1_ACTUAL_SPEED_20G_KR2: c_uint = 0x3900;
pub const MDIO_REG_BANK_10G_PARALLEL_DETECT: c_uint = 0x8130;
pub const MDIO_10G_PARALLEL_DETECT_PAR_DET_10G_STATUS: c_uint = 0x10;
pub const MDIO_10G_PARALLEL_DETECT_PAR_DET_10G_STATUS_PD_LINK: c_uint = 0x8000;
pub const MDIO_10G_PARALLEL_DETECT_PAR_DET_10G_CONTROL: c_uint = 0x11;
pub const MDIO_10G_PARALLEL_DETECT_PAR_DET_10G_CONTROL_PARDET10G_EN: c_uint = 0x1;
pub const MDIO_10G_PARALLEL_DETECT_PAR_DET_10G_LINK: c_uint = 0x13;

pub const MDIO_REG_BANK_SERDES_DIGITAL: c_uint = 0x8300;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1: c_uint = 0x10;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_FIBER_MODE: c_uint = 0x0001;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_TBI_IF: c_uint = 0x0002;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_SIGNAL_DETECT_EN: c_uint = 0x0004;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_INVERT_SIGNAL_DETECT: c_uint = 0x0008;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_AUTODET: c_uint = 0x0010;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL1_MSTR_MODE: c_uint = 0x0020;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL2: c_uint = 0x11;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL2_PRL_DT_EN: c_uint = 0x0001;
pub const MDIO_SERDES_DIGITAL_A_1000X_CONTROL2_AN_FST_TMR: c_uint = 0x0040;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1: c_uint = 0x14;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SGMII: c_uint = 0x0001;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_LINK: c_uint = 0x0002;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_DUPLEX: c_uint = 0x0004;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_MASK: c_uint = 0x0018;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_SHIFT: c_int = 3;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_2_5G: c_uint = 0x0018;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_1G: c_uint = 0x0010;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_100M: c_uint = 0x0008;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS1_SPEED_10M: c_uint = 0x0000;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS2: c_uint = 0x15;
pub const MDIO_SERDES_DIGITAL_A_1000X_STATUS2_AN_DISABLED: c_uint = 0x0002;
pub const MDIO_SERDES_DIGITAL_MISC1: c_uint = 0x18;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_MASK: c_uint = 0xE000;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_25M: c_uint = 0x0000;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_100M: c_uint = 0x2000;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_125M: c_uint = 0x4000;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_156_25M: c_uint = 0x6000;
pub const MDIO_SERDES_DIGITAL_MISC1_REFCLK_SEL_187_5M: c_uint = 0x8000;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_SEL: c_uint = 0x0010;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_MASK: c_uint = 0x000f;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_2_5G: c_uint = 0x0000;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_5G: c_uint = 0x0001;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_6G: c_uint = 0x0002;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_10G_HIG: c_uint = 0x0003;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_10G_CX4: c_uint = 0x0004;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_12G: c_uint = 0x0005;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_12_5G: c_uint = 0x0006;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_13G: c_uint = 0x0007;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_15G: c_uint = 0x0008;
pub const MDIO_SERDES_DIGITAL_MISC1_FORCE_SPEED_16G: c_uint = 0x0009;
pub const MDIO_REG_BANK_OVER_1G: c_uint = 0x8320;
pub const MDIO_OVER_1G_DIGCTL_3_4: c_uint = 0x14;
pub const MDIO_OVER_1G_DIGCTL_3_4_MP_ID_MASK: c_uint = 0xffe0;
pub const MDIO_OVER_1G_DIGCTL_3_4_MP_ID_SHIFT: c_int = 5;
pub const MDIO_OVER_1G_UP1: c_uint = 0x19;
pub const MDIO_OVER_1G_UP1_2_5G: c_uint = 0x0001;
pub const MDIO_OVER_1G_UP1_5G: c_uint = 0x0002;
pub const MDIO_OVER_1G_UP1_6G: c_uint = 0x0004;
pub const MDIO_OVER_1G_UP1_10G: c_uint = 0x0010;
pub const MDIO_OVER_1G_UP1_10GH: c_uint = 0x0008;
pub const MDIO_OVER_1G_UP1_12G: c_uint = 0x0020;
pub const MDIO_OVER_1G_UP1_12_5G: c_uint = 0x0040;
pub const MDIO_OVER_1G_UP1_13G: c_uint = 0x0080;
pub const MDIO_OVER_1G_UP1_15G: c_uint = 0x0100;
pub const MDIO_OVER_1G_UP1_16G: c_uint = 0x0200;
pub const MDIO_OVER_1G_UP2: c_uint = 0x1A;
pub const MDIO_OVER_1G_UP2_IPREDRIVER_MASK: c_uint = 0x0007;
pub const MDIO_OVER_1G_UP2_IDRIVER_MASK: c_uint = 0x0038;
pub const MDIO_OVER_1G_UP2_PREEMPHASIS_MASK: c_uint = 0x03C0;
pub const MDIO_OVER_1G_UP3: c_uint = 0x1B;
pub const MDIO_OVER_1G_UP3_HIGIG2: c_uint = 0x0001;
pub const MDIO_OVER_1G_LP_UP1: c_uint = 0x1C;
pub const MDIO_OVER_1G_LP_UP2: c_uint = 0x1D;
pub const MDIO_OVER_1G_LP_UP2_MR_ADV_OVER_1G_MASK: c_uint = 0x03ff;
pub const MDIO_OVER_1G_LP_UP2_PREEMPHASIS_MASK: c_uint = 0x0780;
pub const MDIO_OVER_1G_LP_UP2_PREEMPHASIS_SHIFT: c_int = 7;
pub const MDIO_OVER_1G_LP_UP3: c_uint = 0x1E;
pub const MDIO_REG_BANK_REMOTE_PHY: c_uint = 0x8330;
pub const MDIO_REMOTE_PHY_MISC_RX_STATUS: c_uint = 0x10;
pub const MDIO_REMOTE_PHY_MISC_RX_STATUS_CL37_FSM_RECEIVED_OVER1G_MSG: c_uint = 0x0010;
pub const MDIO_REMOTE_PHY_MISC_RX_STATUS_CL37_FSM_RECEIVED_BRCM_OUI_MSG: c_uint = 0x0600;
pub const MDIO_REG_BANK_BAM_NEXT_PAGE: c_uint = 0x8350;
pub const MDIO_BAM_NEXT_PAGE_MP5_NEXT_PAGE_CTRL: c_uint = 0x10;
pub const MDIO_BAM_NEXT_PAGE_MP5_NEXT_PAGE_CTRL_BAM_MODE: c_uint = 0x0001;
pub const MDIO_BAM_NEXT_PAGE_MP5_NEXT_PAGE_CTRL_TETON_AN: c_uint = 0x0002;
pub const MDIO_REG_BANK_CL73_USERB0: c_uint = 0x8370;
pub const MDIO_CL73_USERB0_CL73_UCTRL: c_uint = 0x10;
pub const MDIO_CL73_USERB0_CL73_UCTRL_USTAT1_MUXSEL: c_uint = 0x0002;
pub const MDIO_CL73_USERB0_CL73_USTAT1: c_uint = 0x11;
pub const MDIO_CL73_USERB0_CL73_USTAT1_LINK_STATUS_CHECK: c_uint = 0x0100;
pub const MDIO_CL73_USERB0_CL73_USTAT1_AN_GOOD_CHECK_BAM37: c_uint = 0x0400;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL1: c_uint = 0x12;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL1_BAM_EN: c_uint = 0x8000;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL1_BAM_STATION_MNGR_EN: c_uint = 0x4000;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL1_BAM_NP_AFTER_BP_EN: c_uint = 0x2000;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL3: c_uint = 0x14;
pub const MDIO_CL73_USERB0_CL73_BAM_CTRL3_USE_CL73_HCD_MR: c_uint = 0x0001;
pub const MDIO_REG_BANK_AER_BLOCK: c_uint = 0xFFD0;
pub const MDIO_AER_BLOCK_AER_REG: c_uint = 0x1E;
pub const MDIO_REG_BANK_COMBO_IEEE0: c_uint = 0xFFE0;
pub const MDIO_COMBO_IEEE0_MII_CONTROL: c_uint = 0x10;
pub const MDIO_COMBO_IEEO_MII_CONTROL_MAN_SGMII_SP_MASK: c_uint = 0x2040;
pub const MDIO_COMBO_IEEO_MII_CONTROL_MAN_SGMII_SP_10: c_uint = 0x0000;
pub const MDIO_COMBO_IEEO_MII_CONTROL_MAN_SGMII_SP_100: c_uint = 0x2000;
pub const MDIO_COMBO_IEEO_MII_CONTROL_MAN_SGMII_SP_1000: c_uint = 0x0040;
pub const MDIO_COMBO_IEEO_MII_CONTROL_FULL_DUPLEX: c_uint = 0x0100;
pub const MDIO_COMBO_IEEO_MII_CONTROL_RESTART_AN: c_uint = 0x0200;
pub const MDIO_COMBO_IEEO_MII_CONTROL_AN_EN: c_uint = 0x1000;
pub const MDIO_COMBO_IEEO_MII_CONTROL_LOOPBACK: c_uint = 0x4000;
pub const MDIO_COMBO_IEEO_MII_CONTROL_RESET: c_uint = 0x8000;
pub const MDIO_COMBO_IEEE0_MII_STATUS: c_uint = 0x11;
pub const MDIO_COMBO_IEEE0_MII_STATUS_LINK_PASS: c_uint = 0x0004;
pub const MDIO_COMBO_IEEE0_MII_STATUS_AUTONEG_COMPLETE: c_uint = 0x0020;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV: c_uint = 0x14;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_FULL_DUPLEX: c_uint = 0x0020;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_HALF_DUPLEX: c_uint = 0x0040;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_PAUSE_MASK: c_uint = 0x0180;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_PAUSE_NONE: c_uint = 0x0000;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_PAUSE_SYMMETRIC: c_uint = 0x0080;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_PAUSE_ASYMMETRIC: c_uint = 0x0100;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_PAUSE_BOTH: c_uint = 0x0180;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_ADV_NEXT_PAGE: c_uint = 0x8000;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1: c_uint = 0x15;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_NEXT_PAGE: c_uint = 0x8000;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_ACK: c_uint = 0x4000;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_PAUSE_MASK: c_uint = 0x0180;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_PAUSE_NONE: c_uint = 0x0000;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_PAUSE_BOTH: c_uint = 0x0180;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_HALF_DUP_CAP: c_uint = 0x0040;
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_FULL_DUP_CAP: c_uint = 0x0020;
// WhenthelinkpartnerisinSGMIImode(bit0=1),then
pub const MDIO_COMBO_IEEE0_AUTO_NEG_LINK_PARTNER_ABILITY1_SGMII_MODE: c_uint = 0x0001;
pub const MDIO_PMA_DEVAD: c_uint = 0x1;
// ieee
pub const MDIO_PMA_REG_CTRL: c_uint = 0x0;
pub const MDIO_PMA_REG_STATUS: c_uint = 0x1;
pub const MDIO_PMA_REG_10G_CTRL2: c_uint = 0x7;
pub const MDIO_PMA_REG_TX_DISABLE: c_uint = 0x0009;
pub const MDIO_PMA_REG_RX_SD: c_uint = 0xa;
// bcm
pub const MDIO_PMA_REG_BCM_CTRL: c_uint = 0x0096;
pub const MDIO_PMA_REG_FEC_CTRL: c_uint = 0x00ab;
pub const MDIO_PMA_REG_PHY_IDENTIFIER: c_uint = 0xc800;
pub const MDIO_PMA_REG_DIGITAL_CTRL: c_uint = 0xc808;
pub const MDIO_PMA_REG_DIGITAL_STATUS: c_uint = 0xc809;
pub const MDIO_PMA_REG_TX_POWER_DOWN: c_uint = 0xca02;
pub const MDIO_PMA_REG_CMU_PLL_BYPASS: c_uint = 0xca09;
pub const MDIO_PMA_REG_MISC_CTRL: c_uint = 0xca0a;
pub const MDIO_PMA_REG_GEN_CTRL: c_uint = 0xca10;
pub const MDIO_PMA_REG_GEN_CTRL_ROM_RESET_INTERNAL_MP: c_uint = 0x0188;
pub const MDIO_PMA_REG_GEN_CTRL_ROM_MICRO_RESET: c_uint = 0x018a;
pub const MDIO_PMA_REG_M8051_MSGIN_REG: c_uint = 0xca12;
pub const MDIO_PMA_REG_M8051_MSGOUT_REG: c_uint = 0xca13;
pub const MDIO_PMA_REG_ROM_VER1: c_uint = 0xca19;
pub const MDIO_PMA_REG_ROM_VER2: c_uint = 0xca1a;
pub const MDIO_PMA_REG_EDC_FFE_MAIN: c_uint = 0xca1b;
pub const MDIO_PMA_REG_PLL_BANDWIDTH: c_uint = 0xca1d;
pub const MDIO_PMA_REG_PLL_CTRL: c_uint = 0xca1e;
pub const MDIO_PMA_REG_MISC_CTRL0: c_uint = 0xca23;
pub const MDIO_PMA_REG_LRM_MODE: c_uint = 0xca3f;
pub const MDIO_PMA_REG_CDR_BANDWIDTH: c_uint = 0xca46;
pub const MDIO_PMA_REG_MISC_CTRL1: c_uint = 0xca85;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_CTRL: c_uint = 0x8000;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_CTRL_STATUS_MASK: c_uint = 0x000c;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_STATUS_IDLE: c_uint = 0x0000;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_STATUS_COMPLETE: c_uint = 0x0004;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_STATUS_IN_PROGRESS: c_uint = 0x0008;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_STATUS_FAILED: c_uint = 0x000c;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_BYTE_CNT: c_uint = 0x8002;
pub const MDIO_PMA_REG_SFP_TWO_WIRE_MEM_ADDR: c_uint = 0x8003;
pub const MDIO_PMA_REG_8726_TWO_WIRE_DATA_BUF: c_uint = 0xc820;
pub const MDIO_PMA_REG_8726_TWO_WIRE_DATA_MASK: c_uint = 0xff;
pub const MDIO_PMA_REG_8726_TX_CTRL1: c_uint = 0xca01;
pub const MDIO_PMA_REG_8726_TX_CTRL2: c_uint = 0xca05;
pub const MDIO_PMA_REG_8727_TWO_WIRE_SLAVE_ADDR: c_uint = 0x8005;
pub const MDIO_PMA_REG_8727_TWO_WIRE_DATA_BUF: c_uint = 0x8007;
pub const MDIO_PMA_REG_8727_TWO_WIRE_DATA_MASK: c_uint = 0xff;
pub const MDIO_PMA_REG_8727_TX_CTRL1: c_uint = 0xca02;
pub const MDIO_PMA_REG_8727_TX_CTRL2: c_uint = 0xca05;
pub const MDIO_PMA_REG_8727_PCS_OPT_CTRL: c_uint = 0xc808;
pub const MDIO_PMA_REG_8727_GPIO_CTRL: c_uint = 0xc80e;
pub const MDIO_PMA_REG_8727_PCS_GP: c_uint = 0xc842;
pub const MDIO_PMA_REG_8727_OPT_CFG_REG: c_uint = 0xc8e4;
pub const MDIO_AN_REG_8727_MISC_CTRL: c_uint = 0x8309;
pub const MDIO_PMA_REG_8073_CHIP_REV: c_uint = 0xc801;
pub const MDIO_PMA_REG_8073_SPEED_LINK_STATUS: c_uint = 0xc820;
pub const MDIO_PMA_REG_8073_XAUI_WA: c_uint = 0xc841;
pub const MDIO_PMA_REG_8073_OPT_DIGITAL_CTRL: c_uint = 0xcd08;
pub const MDIO_PMA_REG_7101_RESET: c_uint = 0xc000;
pub const MDIO_PMA_REG_7107_LED_CNTL: c_uint = 0xc007;
pub const MDIO_PMA_REG_7107_LINK_LED_CNTL: c_uint = 0xc009;
pub const MDIO_PMA_REG_7101_VER1: c_uint = 0xc026;
pub const MDIO_PMA_REG_7101_VER2: c_uint = 0xc027;
pub const MDIO_PMA_REG_8481_PMD_SIGNAL: c_uint = 0xa811;
pub const MDIO_PMA_REG_8481_LED1_MASK: c_uint = 0xa82c;
pub const MDIO_PMA_REG_8481_LED2_MASK: c_uint = 0xa82f;
pub const MDIO_PMA_REG_8481_LED3_MASK: c_uint = 0xa832;
pub const MDIO_PMA_REG_8481_LED3_BLINK: c_uint = 0xa834;
pub const MDIO_PMA_REG_8481_LED5_MASK: c_uint = 0xa838;
pub const MDIO_PMA_REG_8481_SIGNAL_MASK: c_uint = 0xa835;
pub const MDIO_PMA_REG_8481_LINK_SIGNAL: c_uint = 0xa83b;
pub const MDIO_PMA_REG_8481_LINK_SIGNAL_LED4_ENABLE_MASK: c_uint = 0x800;
pub const MDIO_PMA_REG_8481_LINK_SIGNAL_LED4_ENABLE_SHIFT: c_int = 11;
pub const MDIO_WIS_DEVAD: c_uint = 0x2;
// bcm
pub const MDIO_WIS_REG_LASI_CNTL: c_uint = 0x9002;
pub const MDIO_WIS_REG_LASI_STATUS: c_uint = 0x9005;
pub const MDIO_PCS_DEVAD: c_uint = 0x3;
pub const MDIO_PCS_REG_STATUS: c_uint = 0x0020;
pub const MDIO_PCS_REG_LASI_STATUS: c_uint = 0x9005;
pub const MDIO_PCS_REG_7101_DSP_ACCESS: c_uint = 0xD000;
pub const MDIO_PCS_REG_7101_SPI_MUX: c_uint = 0xD008;
pub const MDIO_PCS_REG_7101_SPI_CTRL_ADDR: c_uint = 0xE12A;

pub const MDIO_PCS_REG_7101_SPI_FIFO_ADDR: c_uint = 0xE02A;

pub const MDIO_PCS_REG_7101_SPI_BYTES_TO_TRANSFER_ADDR: c_uint = 0xE028;
pub const MDIO_XS_DEVAD: c_uint = 0x4;
pub const MDIO_XS_PLL_SEQUENCER: c_uint = 0x8000;
pub const MDIO_XS_SFX7101_XGXS_TEST1: c_uint = 0xc00a;
pub const MDIO_XS_8706_REG_BANK_RX0: c_uint = 0x80bc;
pub const MDIO_XS_8706_REG_BANK_RX1: c_uint = 0x80cc;
pub const MDIO_XS_8706_REG_BANK_RX2: c_uint = 0x80dc;
pub const MDIO_XS_8706_REG_BANK_RX3: c_uint = 0x80ec;
pub const MDIO_XS_8706_REG_BANK_RXA: c_uint = 0x80fc;
pub const MDIO_XS_REG_8073_RX_CTRL_PCIE: c_uint = 0x80FA;
pub const MDIO_AN_DEVAD: c_uint = 0x7;
// ieee
pub const MDIO_AN_REG_CTRL: c_uint = 0x0000;
pub const MDIO_AN_REG_STATUS: c_uint = 0x0001;
pub const MDIO_AN_REG_STATUS_AN_COMPLETE: c_uint = 0x0020;
pub const MDIO_AN_REG_ADV_PAUSE: c_uint = 0x0010;
pub const MDIO_AN_REG_ADV_PAUSE_PAUSE: c_uint = 0x0400;
pub const MDIO_AN_REG_ADV_PAUSE_ASYMMETRIC: c_uint = 0x0800;
pub const MDIO_AN_REG_ADV_PAUSE_BOTH: c_uint = 0x0C00;
pub const MDIO_AN_REG_ADV_PAUSE_MASK: c_uint = 0x0C00;
pub const MDIO_AN_REG_ADV: c_uint = 0x0011;
pub const MDIO_AN_REG_ADV2: c_uint = 0x0012;
pub const MDIO_AN_REG_LP_AUTO_NEG: c_uint = 0x0013;
pub const MDIO_AN_REG_LP_AUTO_NEG2: c_uint = 0x0014;
pub const MDIO_AN_REG_MASTER_STATUS: c_uint = 0x0021;
pub const MDIO_AN_REG_EEE_ADV: c_uint = 0x003c;
pub const MDIO_AN_REG_LP_EEE_ADV: c_uint = 0x003d;
// bcm
pub const MDIO_AN_REG_LINK_STATUS: c_uint = 0x8304;
pub const MDIO_AN_REG_CL37_CL73: c_uint = 0x8370;
pub const MDIO_AN_REG_CL37_AN: c_uint = 0xffe0;
pub const MDIO_AN_REG_CL37_FC_LD: c_uint = 0xffe4;
pub const MDIO_AN_REG_CL37_FC_LP: c_uint = 0xffe5;
pub const MDIO_AN_REG_1000T_STATUS: c_uint = 0xffea;
pub const MDIO_AN_REG_8073_2_5G: c_uint = 0x8329;
pub const MDIO_AN_REG_8073_BAM: c_uint = 0x8350;
pub const MDIO_AN_REG_8481_10GBASE_T_AN_CTRL: c_uint = 0x0020;
pub const MDIO_AN_REG_8481_LEGACY_MII_CTRL: c_uint = 0xffe0;
pub const MDIO_AN_REG_8481_MII_CTRL_FORCE_1G: c_uint = 0x40;
pub const MDIO_AN_REG_8481_LEGACY_MII_STATUS: c_uint = 0xffe1;
pub const MDIO_AN_REG_848xx_ID_MSB: c_uint = 0xffe2;
pub const BCM84858_PHY_ID: c_uint = 0x600d;
pub const MDIO_AN_REG_848xx_ID_LSB: c_uint = 0xffe3;
pub const MDIO_AN_REG_8481_LEGACY_AN_ADV: c_uint = 0xffe4;
pub const MDIO_AN_REG_8481_LEGACY_AN_EXPANSION: c_uint = 0xffe6;
pub const MDIO_AN_REG_8481_1000T_CTRL: c_uint = 0xffe9;
pub const MDIO_AN_REG_8481_1G_100T_EXT_CTRL: c_uint = 0xfff0;
pub const MIDO_AN_REG_8481_EXT_CTRL_FORCE_LEDS_OFF: c_uint = 0x0008;
pub const MDIO_AN_REG_8481_EXPANSION_REG_RD_RW: c_uint = 0xfff5;
pub const MDIO_AN_REG_8481_EXPANSION_REG_ACCESS: c_uint = 0xfff7;
pub const MDIO_AN_REG_8481_AUX_CTRL: c_uint = 0xfff8;
pub const MDIO_AN_REG_8481_LEGACY_SHADOW: c_uint = 0xfffc;
// BCM84823 only
pub const MDIO_CTL_DEVAD: c_uint = 0x1e;
pub const MDIO_CTL_REG_84823_MEDIA: c_uint = 0x401a;
pub const MDIO_CTL_REG_84823_MEDIA_MAC_MASK: c_uint = 0x0018;
// These pins configure the BCM84823 interface to MAC after reset.
pub const MDIO_CTL_REG_84823_CTRL_MAC_XFI: c_uint = 0x0008;
pub const MDIO_CTL_REG_84823_MEDIA_MAC_XAUI_M: c_uint = 0x0010;
// These pins configure the BCM84823 interface to Line after reset.
pub const MDIO_CTL_REG_84823_MEDIA_LINE_MASK: c_uint = 0x0060;
pub const MDIO_CTL_REG_84823_MEDIA_LINE_XAUI_L: c_uint = 0x0020;
pub const MDIO_CTL_REG_84823_MEDIA_LINE_XFI: c_uint = 0x0040;
// When this pin is active high during reset, 10GBASE-T core is power
// down, When it is active low the 10GBASE-T is power up
//
pub const MDIO_CTL_REG_84823_MEDIA_COPPER_CORE_DOWN: c_uint = 0x0080;
pub const MDIO_CTL_REG_84823_MEDIA_PRIORITY_MASK: c_uint = 0x0100;
pub const MDIO_CTL_REG_84823_MEDIA_PRIORITY_COPPER: c_uint = 0x0000;
pub const MDIO_CTL_REG_84823_MEDIA_PRIORITY_FIBER: c_uint = 0x0100;
pub const MDIO_CTL_REG_84823_MEDIA_FIBER_1G: c_uint = 0x1000;
pub const MDIO_CTL_REG_84823_USER_CTRL_REG: c_uint = 0x4005;
pub const MDIO_CTL_REG_84823_USER_CTRL_CMS: c_uint = 0x0080;
pub const MDIO_PMA_REG_84823_CTL_SLOW_CLK_CNT_HIGH: c_uint = 0xa82b;
pub const MDIO_PMA_REG_84823_BLINK_RATE_VAL_15P9HZ: c_uint = 0x2f;
pub const MDIO_PMA_REG_84823_CTL_LED_CTL_1: c_uint = 0xa8e3;
pub const MDIO_PMA_REG_84833_CTL_LED_CTL_1: c_uint = 0xa8ec;
pub const MDIO_PMA_REG_84823_LED3_STRETCH_EN: c_uint = 0x0080;
// BCM84858 only
pub const MDIO_PMA_REG_84858_ALLOW_GPHY_ACT: c_uint = 0x8000;
// BCM84833 only
pub const MDIO_84833_TOP_CFG_FW_REV: c_uint = 0x400f;
pub const MDIO_84833_TOP_CFG_FW_EEE: c_uint = 0x10b1;
pub const MDIO_84833_TOP_CFG_FW_NO_EEE: c_uint = 0x1f81;
pub const MDIO_84833_TOP_CFG_XGPHY_STRAP1: c_uint = 0x401a;
pub const MDIO_84833_SUPER_ISOLATE: c_uint = 0x8000;
// These are mailbox register set used by 84833/84858.
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG0: c_uint = 0x4005;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG1: c_uint = 0x4006;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG2: c_uint = 0x4007;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG3: c_uint = 0x4008;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG4: c_uint = 0x4009;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG26: c_uint = 0x4037;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG27: c_uint = 0x4038;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG28: c_uint = 0x4039;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG29: c_uint = 0x403a;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG30: c_uint = 0x403b;
pub const MDIO_848xx_TOP_CFG_SCRATCH_REG31: c_uint = 0x403c;

// Mailbox command set used by 84833/84858
pub const PHY848xx_CMD_SET_PAIR_SWAP: c_uint = 0x8001;
pub const PHY848xx_CMD_GET_EEE_MODE: c_uint = 0x8008;
pub const PHY848xx_CMD_SET_EEE_MODE: c_uint = 0x8009;
// Mailbox status set used by 84833 only
pub const PHY84833_STATUS_CMD_RECEIVED: c_uint = 0x0001;
pub const PHY84833_STATUS_CMD_IN_PROGRESS: c_uint = 0x0002;
pub const PHY84833_STATUS_CMD_COMPLETE_PASS: c_uint = 0x0004;
pub const PHY84833_STATUS_CMD_COMPLETE_ERROR: c_uint = 0x0008;
pub const PHY84833_STATUS_CMD_OPEN_FOR_CMDS: c_uint = 0x0010;
pub const PHY84833_STATUS_CMD_SYSTEM_BOOT: c_uint = 0x0020;
pub const PHY84833_STATUS_CMD_NOT_OPEN_FOR_CMDS: c_uint = 0x0040;
pub const PHY84833_STATUS_CMD_CLEAR_COMPLETE: c_uint = 0x0080;
pub const PHY84833_STATUS_CMD_OPEN_OVERRIDE: c_uint = 0xa5a5;
// Mailbox Process
pub const PHY84833_MB_PROCESS1: c_int = 1;
pub const PHY84833_MB_PROCESS2: c_int = 2;
pub const PHY84833_MB_PROCESS3: c_int = 3;
// Mailbox status set used by 84858 only
pub const PHY84858_STATUS_CMD_RECEIVED: c_uint = 0x0001;
pub const PHY84858_STATUS_CMD_IN_PROGRESS: c_uint = 0x0002;
pub const PHY84858_STATUS_CMD_COMPLETE_PASS: c_uint = 0x0004;
pub const PHY84858_STATUS_CMD_COMPLETE_ERROR: c_uint = 0x0008;
pub const PHY84858_STATUS_CMD_SYSTEM_BUSY: c_uint = 0xbbbb;
// Warpcore clause 45 addressing
pub const MDIO_WC_DEVAD: c_uint = 0x3;
pub const MDIO_WC_REG_IEEE0BLK_MIICNTL: c_uint = 0x0;
pub const MDIO_WC_REG_IEEE0BLK_AUTONEGNP: c_uint = 0x7;
pub const MDIO_WC_REG_AN_IEEE1BLK_AN_ADVERTISEMENT0: c_uint = 0x10;
pub const MDIO_WC_REG_AN_IEEE1BLK_AN_ADVERTISEMENT1: c_uint = 0x11;
pub const MDIO_WC_REG_AN_IEEE1BLK_AN_ADVERTISEMENT2: c_uint = 0x12;
pub const MDIO_WC_REG_AN_IEEE1BLK_AN_ADV2_FEC_ABILITY: c_uint = 0x4000;
pub const MDIO_WC_REG_AN_IEEE1BLK_AN_ADV2_FEC_REQ: c_uint = 0x8000;
pub const MDIO_WC_REG_PCS_STATUS2: c_uint = 0x0021;
pub const MDIO_WC_REG_PMD_KR_CONTROL: c_uint = 0x0096;
pub const MDIO_WC_REG_XGXSBLK0_XGXSCONTROL: c_uint = 0x8000;
pub const MDIO_WC_REG_XGXSBLK0_MISCCONTROL1: c_uint = 0x800e;
pub const MDIO_WC_REG_XGXSBLK1_DESKEW: c_uint = 0x8010;
pub const MDIO_WC_REG_XGXSBLK1_LANECTRL0: c_uint = 0x8015;
pub const MDIO_WC_REG_XGXSBLK1_LANECTRL1: c_uint = 0x8016;
pub const MDIO_WC_REG_XGXSBLK1_LANECTRL2: c_uint = 0x8017;
pub const MDIO_WC_REG_TX0_ANA_CTRL0: c_uint = 0x8061;
pub const MDIO_WC_REG_TX1_ANA_CTRL0: c_uint = 0x8071;
pub const MDIO_WC_REG_TX2_ANA_CTRL0: c_uint = 0x8081;
pub const MDIO_WC_REG_TX3_ANA_CTRL0: c_uint = 0x8091;
pub const MDIO_WC_REG_TX0_TX_DRIVER: c_uint = 0x8067;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IFIR_OFFSET: c_uint = 0x01;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IFIR_MASK: c_uint = 0x000e;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IPRE_DRIVER_OFFSET: c_uint = 0x04;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IPRE_DRIVER_MASK: c_uint = 0x00f0;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IDRIVER_OFFSET: c_uint = 0x08;
pub const MDIO_WC_REG_TX0_TX_DRIVER_IDRIVER_MASK: c_uint = 0x0f00;
pub const MDIO_WC_REG_TX0_TX_DRIVER_POST2_COEFF_OFFSET: c_uint = 0x0c;
pub const MDIO_WC_REG_TX0_TX_DRIVER_POST2_COEFF_MASK: c_uint = 0x7000;
pub const MDIO_WC_REG_TX1_TX_DRIVER: c_uint = 0x8077;
pub const MDIO_WC_REG_TX2_TX_DRIVER: c_uint = 0x8087;
pub const MDIO_WC_REG_TX3_TX_DRIVER: c_uint = 0x8097;
pub const MDIO_WC_REG_RX0_ANARXCONTROL1G: c_uint = 0x80b9;
pub const MDIO_WC_REG_RX2_ANARXCONTROL1G: c_uint = 0x80d9;
pub const MDIO_WC_REG_RX0_PCI_CTRL: c_uint = 0x80ba;
pub const MDIO_WC_REG_RX1_PCI_CTRL: c_uint = 0x80ca;
pub const MDIO_WC_REG_RX2_PCI_CTRL: c_uint = 0x80da;
pub const MDIO_WC_REG_RX3_PCI_CTRL: c_uint = 0x80ea;
pub const MDIO_WC_REG_RXB_ANA_RX_CONTROL_PCI: c_uint = 0x80fa;
pub const MDIO_WC_REG_XGXSBLK2_UNICORE_MODE_10G: c_uint = 0x8104;
pub const MDIO_WC_REG_XGXS_STATUS3: c_uint = 0x8129;
pub const MDIO_WC_REG_PAR_DET_10G_STATUS: c_uint = 0x8130;
pub const MDIO_WC_REG_PAR_DET_10G_CTRL: c_uint = 0x8131;
pub const MDIO_WC_REG_XGXS_X2_CONTROL2: c_uint = 0x8141;
pub const MDIO_WC_REG_XGXS_X2_CONTROL3: c_uint = 0x8142;
pub const MDIO_WC_REG_XGXS_RX_LN_SWAP1: c_uint = 0x816B;
pub const MDIO_WC_REG_XGXS_TX_LN_SWAP1: c_uint = 0x8169;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_0: c_uint = 0x81d0;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_1: c_uint = 0x81d1;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_2: c_uint = 0x81d2;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_3: c_uint = 0x81d3;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_4: c_uint = 0x81d4;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_4_CL73_AN_CMPL: c_uint = 0x1000;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_4_CL37_AN_CMPL: c_uint = 0x0100;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_4_CL37_LP_AN_CAP: c_uint = 0x0010;
pub const MDIO_WC_REG_GP2_STATUS_GP_2_4_CL37_AN_CAP: c_uint = 0x1;
pub const MDIO_WC_REG_UC_INFO_B0_DEAD_TRAP: c_uint = 0x81EE;
pub const MDIO_WC_REG_UC_INFO_B1_VERSION: c_uint = 0x81F0;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE: c_uint = 0x81F2;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_LANE0_OFFSET: c_uint = 0x0;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE_DEFAULT: c_uint = 0x0;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE_SFP_OPT_LR: c_uint = 0x1;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE_SFP_DAC: c_uint = 0x2;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE_SFP_XLAUI: c_uint = 0x3;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_MODE_LONG_CH_6G: c_uint = 0x4;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_LANE1_OFFSET: c_uint = 0x4;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_LANE2_OFFSET: c_uint = 0x8;
pub const MDIO_WC_REG_UC_INFO_B1_FIRMWARE_LANE3_OFFSET: c_uint = 0xc;
pub const MDIO_WC_REG_UC_INFO_B1_CRC: c_uint = 0x81FE;
pub const MDIO_WC_REG_DSC_SMC: c_uint = 0x8213;
pub const MDIO_WC_REG_DSC2B0_DSC_MISC_CTRL0: c_uint = 0x821e;
pub const MDIO_WC_REG_TX_FIR_TAP: c_uint = 0x82e2;
pub const MDIO_WC_REG_TX_FIR_TAP_PRE_TAP_OFFSET: c_uint = 0x00;
pub const MDIO_WC_REG_TX_FIR_TAP_PRE_TAP_MASK: c_uint = 0x000f;
pub const MDIO_WC_REG_TX_FIR_TAP_MAIN_TAP_OFFSET: c_uint = 0x04;
pub const MDIO_WC_REG_TX_FIR_TAP_MAIN_TAP_MASK: c_uint = 0x03f0;
pub const MDIO_WC_REG_TX_FIR_TAP_POST_TAP_OFFSET: c_uint = 0x0a;
pub const MDIO_WC_REG_TX_FIR_TAP_POST_TAP_MASK: c_uint = 0x7c00;
pub const MDIO_WC_REG_TX_FIR_TAP_ENABLE: c_uint = 0x8000;
pub const MDIO_WC_REG_CL72_USERB0_CL72_TX_FIR_TAP: c_uint = 0x82e2;
pub const MDIO_WC_REG_CL72_USERB0_CL72_MISC1_CONTROL: c_uint = 0x82e3;
pub const MDIO_WC_REG_CL72_USERB0_CL72_OS_DEF_CTRL: c_uint = 0x82e6;
pub const MDIO_WC_REG_CL72_USERB0_CL72_BR_DEF_CTRL: c_uint = 0x82e7;
pub const MDIO_WC_REG_CL72_USERB0_CL72_2P5_DEF_CTRL: c_uint = 0x82e8;
pub const MDIO_WC_REG_CL72_USERB0_CL72_MISC4_CONTROL: c_uint = 0x82ec;
pub const MDIO_WC_REG_SERDESDIGITAL_CONTROL1000X1: c_uint = 0x8300;
pub const MDIO_WC_REG_SERDESDIGITAL_CONTROL1000X2: c_uint = 0x8301;
pub const MDIO_WC_REG_SERDESDIGITAL_CONTROL1000X3: c_uint = 0x8302;
pub const MDIO_WC_REG_SERDESDIGITAL_STATUS1000X1: c_uint = 0x8304;
pub const MDIO_WC_REG_SERDESDIGITAL_MISC1: c_uint = 0x8308;
pub const MDIO_WC_REG_SERDESDIGITAL_MISC2: c_uint = 0x8309;
pub const MDIO_WC_REG_DIGITAL3_UP1: c_uint = 0x8329;
pub const MDIO_WC_REG_DIGITAL3_LP_UP1: c_uint = 0x832c;
pub const MDIO_WC_REG_DIGITAL4_MISC3: c_uint = 0x833c;
pub const MDIO_WC_REG_DIGITAL4_MISC5: c_uint = 0x833e;
pub const MDIO_WC_REG_DIGITAL5_MISC6: c_uint = 0x8345;
pub const MDIO_WC_REG_DIGITAL5_MISC7: c_uint = 0x8349;
pub const MDIO_WC_REG_DIGITAL5_LINK_STATUS: c_uint = 0x834d;
pub const MDIO_WC_REG_DIGITAL5_ACTUAL_SPEED: c_uint = 0x834e;
pub const MDIO_WC_REG_DIGITAL6_MP5_NEXTPAGECTRL: c_uint = 0x8350;
pub const MDIO_WC_REG_CL49_USERB0_CTRL: c_uint = 0x8368;
pub const MDIO_WC_REG_CL73_USERB0_CTRL: c_uint = 0x8370;
pub const MDIO_WC_REG_CL73_USERB0_USTAT: c_uint = 0x8371;
pub const MDIO_WC_REG_CL73_BAM_CTRL1: c_uint = 0x8372;
pub const MDIO_WC_REG_CL73_BAM_CTRL2: c_uint = 0x8373;
pub const MDIO_WC_REG_CL73_BAM_CTRL3: c_uint = 0x8374;
pub const MDIO_WC_REG_CL73_BAM_CODE_FIELD: c_uint = 0x837b;
pub const MDIO_WC_REG_EEE_COMBO_CONTROL0: c_uint = 0x8390;
pub const MDIO_WC_REG_TX66_CONTROL: c_uint = 0x83b0;
pub const MDIO_WC_REG_RX66_CONTROL: c_uint = 0x83c0;
pub const MDIO_WC_REG_RX66_SCW0: c_uint = 0x83c2;
pub const MDIO_WC_REG_RX66_SCW1: c_uint = 0x83c3;
pub const MDIO_WC_REG_RX66_SCW2: c_uint = 0x83c4;
pub const MDIO_WC_REG_RX66_SCW3: c_uint = 0x83c5;
pub const MDIO_WC_REG_RX66_SCW0_MASK: c_uint = 0x83c6;
pub const MDIO_WC_REG_RX66_SCW1_MASK: c_uint = 0x83c7;
pub const MDIO_WC_REG_RX66_SCW2_MASK: c_uint = 0x83c8;
pub const MDIO_WC_REG_RX66_SCW3_MASK: c_uint = 0x83c9;
pub const MDIO_WC_REG_FX100_CTRL1: c_uint = 0x8400;
pub const MDIO_WC_REG_FX100_CTRL3: c_uint = 0x8402;
pub const MDIO_WC_REG_CL82_USERB1_TX_CTRL5: c_uint = 0x8436;
pub const MDIO_WC_REG_CL82_USERB1_TX_CTRL6: c_uint = 0x8437;
pub const MDIO_WC_REG_CL82_USERB1_TX_CTRL7: c_uint = 0x8438;
pub const MDIO_WC_REG_CL82_USERB1_TX_CTRL9: c_uint = 0x8439;
pub const MDIO_WC_REG_CL82_USERB1_RX_CTRL10: c_uint = 0x843a;
pub const MDIO_WC_REG_CL82_USERB1_RX_CTRL11: c_uint = 0x843b;
pub const MDIO_WC_REG_ETA_CL73_OUI1: c_uint = 0x8453;
pub const MDIO_WC_REG_ETA_CL73_OUI2: c_uint = 0x8454;
pub const MDIO_WC_REG_ETA_CL73_OUI3: c_uint = 0x8455;
pub const MDIO_WC_REG_ETA_CL73_LD_BAM_CODE: c_uint = 0x8456;
pub const MDIO_WC_REG_ETA_CL73_LD_UD_CODE: c_uint = 0x8457;
pub const MDIO_WC_REG_MICROBLK_CMD: c_uint = 0xffc2;
pub const MDIO_WC_REG_MICROBLK_DL_STATUS: c_uint = 0xffc5;
pub const MDIO_WC_REG_MICROBLK_CMD3: c_uint = 0xffcc;
pub const MDIO_WC_REG_AERBLK_AER: c_uint = 0xffde;
pub const MDIO_WC_REG_COMBO_IEEE0_MIICTRL: c_uint = 0xffe0;
pub const MDIO_WC_REG_COMBO_IEEE0_MIIISTAT: c_uint = 0xffe1;
pub const MDIO_WC0_XGXS_BLK2_LANE_RESET: c_uint = 0x810A;
pub const MDIO_WC0_XGXS_BLK2_LANE_RESET_RX_BITSHIFT: c_int = 0;
pub const MDIO_WC0_XGXS_BLK2_LANE_RESET_TX_BITSHIFT: c_int = 4;
pub const MDIO_WC0_XGXS_BLK6_XGXS_X2_CONTROL2: c_uint = 0x8141;
pub const DIGITAL5_ACTUAL_SPEED_TX_MASK: c_uint = 0x003f;
// 54618se
pub const MDIO_REG_GPHY_PHYID_LSB: c_uint = 0x3;
pub const MDIO_REG_GPHY_ID_54618SE: c_uint = 0x5cd5;
pub const MDIO_REG_GPHY_CL45_ADDR_REG: c_uint = 0xd;
pub const MDIO_REG_GPHY_CL45_DATA_REG: c_uint = 0xe;
pub const MDIO_REG_GPHY_EEE_RESOLVED: c_uint = 0x803e;
pub const MDIO_REG_GPHY_EXP_ACCESS_GATE: c_uint = 0x15;
pub const MDIO_REG_GPHY_EXP_ACCESS: c_uint = 0x17;
pub const MDIO_REG_GPHY_EXP_ACCESS_TOP: c_uint = 0xd00;
pub const MDIO_REG_GPHY_EXP_TOP_2K_BUF: c_uint = 0x40;
pub const MDIO_REG_GPHY_AUX_STATUS: c_uint = 0x19;
pub const MDIO_REG_INTR_STATUS: c_uint = 0x1a;
pub const MDIO_REG_INTR_MASK: c_uint = 0x1b;

pub const MDIO_REG_GPHY_SHADOW: c_uint = 0x1c;

pub const IGU_FUNC_BASE: c_uint = 0x0400;
pub const IGU_ADDR_MSIX: c_uint = 0x0000;
pub const IGU_ADDR_INT_ACK: c_uint = 0x0200;
pub const IGU_ADDR_PROD_UPD: c_uint = 0x0201;
pub const IGU_ADDR_ATTN_BITS_UPD: c_uint = 0x0202;
pub const IGU_ADDR_ATTN_BITS_SET: c_uint = 0x0203;
pub const IGU_ADDR_ATTN_BITS_CLR: c_uint = 0x0204;
pub const IGU_ADDR_COALESCE_NOW: c_uint = 0x0205;
pub const IGU_ADDR_SIMD_MASK: c_uint = 0x0206;
pub const IGU_ADDR_SIMD_NOMASK: c_uint = 0x0207;
pub const IGU_ADDR_MSI_CTL: c_uint = 0x0210;
pub const IGU_ADDR_MSI_ADDR_LO: c_uint = 0x0211;
pub const IGU_ADDR_MSI_ADDR_HI: c_uint = 0x0212;
pub const IGU_ADDR_MSI_DATA: c_uint = 0x0213;
pub const IGU_USE_REGISTER_ustorm_type_0_sb_cleanup: c_int = 0;
pub const IGU_USE_REGISTER_ustorm_type_1_sb_cleanup: c_int = 1;
pub const IGU_USE_REGISTER_cstorm_type_0_sb_cleanup: c_int = 2;
pub const IGU_USE_REGISTER_cstorm_type_1_sb_cleanup: c_int = 3;
pub const COMMAND_REG_INT_ACK: c_uint = 0x0;
pub const COMMAND_REG_PROD_UPD: c_uint = 0x4;
pub const COMMAND_REG_ATTN_BITS_UPD: c_uint = 0x8;
pub const COMMAND_REG_ATTN_BITS_SET: c_uint = 0xc;
pub const COMMAND_REG_ATTN_BITS_CLR: c_uint = 0x10;
pub const COMMAND_REG_COALESCE_NOW: c_uint = 0x14;
pub const COMMAND_REG_SIMD_MASK: c_uint = 0x18;
pub const COMMAND_REG_SIMD_NOMASK: c_uint = 0x1c;
pub const IGU_MEM_BASE: c_uint = 0x0000;
pub const IGU_MEM_MSIX_BASE: c_uint = 0x0000;
pub const IGU_MEM_MSIX_UPPER: c_uint = 0x007f;
pub const IGU_MEM_MSIX_RESERVED_UPPER: c_uint = 0x01ff;
pub const IGU_MEM_PBA_MSIX_BASE: c_uint = 0x0200;
pub const IGU_MEM_PBA_MSIX_UPPER: c_uint = 0x0200;
pub const IGU_CMD_BACKWARD_COMP_PROD_UPD: c_uint = 0x0201;
pub const IGU_MEM_PBA_MSIX_RESERVED_UPPER: c_uint = 0x03ff;
pub const IGU_CMD_INT_ACK_BASE: c_uint = 0x0400;
// Macro flag: #define IGU_CMD_INT_ACK_UPPER\
pub const IGU_CMD_INT_ACK_RESERVED_UPPER: c_uint = 0x04ff;
pub const IGU_CMD_E2_PROD_UPD_BASE: c_uint = 0x0500;
// Macro flag: #define IGU_CMD_E2_PROD_UPD_UPPER\
pub const IGU_CMD_E2_PROD_UPD_RESERVED_UPPER: c_uint = 0x059f;
pub const IGU_CMD_ATTN_BIT_UPD_UPPER: c_uint = 0x05a0;
pub const IGU_CMD_ATTN_BIT_SET_UPPER: c_uint = 0x05a1;
pub const IGU_CMD_ATTN_BIT_CLR_UPPER: c_uint = 0x05a2;
pub const IGU_REG_SISR_MDPC_WMASK_UPPER: c_uint = 0x05a3;
pub const IGU_REG_SISR_MDPC_WMASK_LSB_UPPER: c_uint = 0x05a4;
pub const IGU_REG_SISR_MDPC_WMASK_MSB_UPPER: c_uint = 0x05a5;
pub const IGU_REG_SISR_MDPC_WOMASK_UPPER: c_uint = 0x05a6;
pub const IGU_REG_RESERVED_UPPER: c_uint = 0x05ff;
// Fields of IGU PF CONFIGURATION REGISTER

// Fields of IGU VF CONFIGURATION REGISTER

pub const IGU_BC_DSB_NUM_SEGS: c_int = 5;
pub const IGU_BC_NDSB_NUM_SEGS: c_int = 2;
pub const IGU_NORM_DSB_NUM_SEGS: c_int = 2;
pub const IGU_NORM_NDSB_NUM_SEGS: c_int = 1;
pub const IGU_BC_BASE_DSB_PROD: c_int = 128;
pub const IGU_NORM_BASE_DSB_PROD: c_int = 136;
// FID (if VF - [6] = 0; [5:0] = VF number; if PF - [6] = 1; \

pub const IGU_FID_ENCODE_IS_PF_SHIFT: c_int = 6;

pub const IGU_REG_MAPPING_MEMORY_VECTOR_SHIFT: c_int = 1;

pub const IGU_REG_MAPPING_MEMORY_FID_SHIFT: c_int = 7;
pub const CDU_REGION_NUMBER_XCM_AG: c_int = 2;
pub const CDU_REGION_NUMBER_UCM_AG: c_int = 4;
// String-to-compress [31:8] = CID (all 24 bits)
// String-to-compress [7:4] = Region
// String-to-compress [3:0] = Type
//

// IdleChk registers
pub const PXP_REG_HST_VF_DISABLED_ERROR_VALID: c_uint = 0x1030bc;
pub const PXP_REG_HST_VF_DISABLED_ERROR_DATA: c_uint = 0x1030b8;
pub const PXP_REG_HST_PER_VIOLATION_VALID: c_uint = 0x1030e0;
pub const PXP_REG_HST_INCORRECT_ACCESS_VALID: c_uint = 0x1030cc;
pub const PXP2_REG_RD_CPL_ERR_DETAILS: c_uint = 0x120778;
pub const PXP2_REG_RD_CPL_ERR_DETAILS2: c_uint = 0x12077c;
pub const PXP2_REG_RQ_GARB: c_uint = 0x120748;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q0: c_uint = 0x15c1bc;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q1: c_uint = 0x15c1c0;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q2: c_uint = 0x15c1c4;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q3: c_uint = 0x15c1c8;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q4: c_uint = 0x15c1cc;
pub const PBF_REG_DISABLE_NEW_TASK_PROC_Q5: c_uint = 0x15c1d0;
pub const PBF_REG_CREDIT_Q2: c_uint = 0x140344;
pub const PBF_REG_CREDIT_Q3: c_uint = 0x140348;
pub const PBF_REG_CREDIT_Q4: c_uint = 0x14034c;
pub const PBF_REG_CREDIT_Q5: c_uint = 0x140350;
pub const PBF_REG_INIT_CRD_Q2: c_uint = 0x15c238;
pub const PBF_REG_INIT_CRD_Q3: c_uint = 0x15c23c;
pub const PBF_REG_INIT_CRD_Q4: c_uint = 0x15c240;
pub const PBF_REG_INIT_CRD_Q5: c_uint = 0x15c244;
pub const PBF_REG_TASK_CNT_Q0: c_uint = 0x140374;
pub const PBF_REG_TASK_CNT_Q1: c_uint = 0x140378;
pub const PBF_REG_TASK_CNT_Q2: c_uint = 0x14037c;
pub const PBF_REG_TASK_CNT_Q3: c_uint = 0x140380;
pub const PBF_REG_TASK_CNT_Q4: c_uint = 0x140384;
pub const PBF_REG_TASK_CNT_Q5: c_uint = 0x140388;
pub const PBF_REG_TASK_CNT_LB_Q: c_uint = 0x140370;
pub const QM_REG_BYTECRD0: c_uint = 0x16e6fc;
pub const QM_REG_BYTECRD1: c_uint = 0x16e700;
pub const QM_REG_BYTECRD2: c_uint = 0x16e704;
pub const QM_REG_BYTECRD3: c_uint = 0x16e7ac;
pub const QM_REG_BYTECRD4: c_uint = 0x16e7b0;
pub const QM_REG_BYTECRD5: c_uint = 0x16e7b4;
pub const QM_REG_BYTECRD6: c_uint = 0x16e7b8;
pub const QM_REG_BYTECRDCMDQ_0: c_uint = 0x16e6e8;
pub const QM_REG_BYTECRDERRREG: c_uint = 0x16e708;
pub const MISC_REG_GRC_TIMEOUT_ATTN_FULL_FID: c_uint = 0xa714;
pub const QM_REG_VOQCREDIT_2: c_uint = 0x1682d8;
pub const QM_REG_VOQCREDIT_3: c_uint = 0x1682dc;
pub const QM_REG_VOQCREDIT_5: c_uint = 0x1682e4;
pub const QM_REG_VOQCREDIT_6: c_uint = 0x1682e8;
pub const QM_REG_VOQINITCREDIT_3: c_uint = 0x16806c;
pub const QM_REG_VOQINITCREDIT_6: c_uint = 0x168078;
pub const QM_REG_FWVOQ0TOHWVOQ: c_uint = 0x16e7bc;
pub const QM_REG_FWVOQ1TOHWVOQ: c_uint = 0x16e7c0;
pub const QM_REG_FWVOQ2TOHWVOQ: c_uint = 0x16e7c4;
pub const QM_REG_FWVOQ3TOHWVOQ: c_uint = 0x16e7c8;
pub const QM_REG_FWVOQ4TOHWVOQ: c_uint = 0x16e7cc;
pub const QM_REG_FWVOQ5TOHWVOQ: c_uint = 0x16e7d0;
pub const QM_REG_FWVOQ6TOHWVOQ: c_uint = 0x16e7d4;
pub const QM_REG_FWVOQ7TOHWVOQ: c_uint = 0x16e7d8;
pub const NIG_REG_INGRESS_EOP_PORT0_EMPTY: c_uint = 0x104ec;
pub const NIG_REG_INGRESS_EOP_PORT1_EMPTY: c_uint = 0x104f8;
pub const NIG_REG_INGRESS_RMP0_DSCR_EMPTY: c_uint = 0x10530;
pub const NIG_REG_INGRESS_RMP1_DSCR_EMPTY: c_uint = 0x10538;
pub const NIG_REG_INGRESS_LB_PBF_DELAY_EMPTY: c_uint = 0x10508;
pub const NIG_REG_EGRESS_MNG0_FIFO_EMPTY: c_uint = 0x10460;
pub const NIG_REG_EGRESS_MNG1_FIFO_EMPTY: c_uint = 0x10474;
pub const NIG_REG_EGRESS_DEBUG_FIFO_EMPTY: c_uint = 0x10418;
pub const NIG_REG_EGRESS_DELAY0_EMPTY: c_uint = 0x10420;
pub const NIG_REG_EGRESS_DELAY1_EMPTY: c_uint = 0x10428;
pub const NIG_REG_LLH0_FIFO_EMPTY: c_uint = 0x10548;
pub const NIG_REG_LLH1_FIFO_EMPTY: c_uint = 0x10558;
pub const NIG_REG_P0_TX_MNG_HOST_FIFO_EMPTY: c_uint = 0x182a8;
pub const NIG_REG_P0_TLLH_FIFO_EMPTY: c_uint = 0x18308;
pub const NIG_REG_P0_HBUF_DSCR_EMPTY: c_uint = 0x18318;
pub const NIG_REG_P1_HBUF_DSCR_EMPTY: c_uint = 0x18348;
pub const NIG_REG_P0_RX_MACFIFO_EMPTY: c_uint = 0x18570;
pub const NIG_REG_P0_TX_MACFIFO_EMPTY: c_uint = 0x18578;
pub const NIG_REG_EGRESS_DELAY2_EMPTY: c_uint = 0x1862c;
pub const NIG_REG_EGRESS_DELAY3_EMPTY: c_uint = 0x18630;
pub const NIG_REG_EGRESS_DELAY4_EMPTY: c_uint = 0x18634;
pub const NIG_REG_EGRESS_DELAY5_EMPTY: c_uint = 0x18638;
//
// Description:
// Calculates crc 8 on a word value: polynomial 0-1-2-8
// Code was translated from Verilog.
// Return:
//
// split the data into 31 bits
// split the crc into 8 bits
