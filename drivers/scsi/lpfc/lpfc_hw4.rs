//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_hw4.h
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


//
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2026 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2009-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//

// Macros to deal with bit fields. Each bit field must have 3 #defines
// associated with it (_SHIFT, _MASK, and _WORD).
// EG. For a bit field that is in the 7th bit of the "field4" field of a
// structure and is 2 bits in size the following #defines must exist:
// struct temp {
// uint32_t	field1;
// uint32_t	field2;
// uint32_t	field3;
// uint32_t	field4;
// #define example_bit_field_SHIFT		7
// #define example_bit_field_MASK		0x03
// #define example_bit_field_WORD		field4
// uint32_t	field5;
// };
// Then the macros below may be used to get or set the value of that field.
// EG. To get the value of the bit field from the above example:
// struct temp t1;
// value = bf_get(example_bit_field, &t1);
// And then to set that bit field:
// bf_set(example_bit_field, &t1, 2);
// Or clear that bit field:
// bf_set(example_bit_field, &t1, 0);
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_address {
    pub addr_lo: u32,
    pub addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_intf {
    pub word0: u32,
pub const lpfc_sli_intf_valid_SHIFT: c_int = 29;
pub const lpfc_sli_intf_valid_MASK: c_uint = 0x00000007;

pub const LPFC_SLI_INTF_VALID: c_int = 6;
pub const lpfc_sli_intf_sli_hint2_SHIFT: c_int = 24;
pub const lpfc_sli_intf_sli_hint2_MASK: c_uint = 0x0000001F;

pub const LPFC_SLI_INTF_SLI_HINT2_NONE: c_int = 0;
pub const lpfc_sli_intf_sli_hint1_SHIFT: c_int = 16;
pub const lpfc_sli_intf_sli_hint1_MASK: c_uint = 0x000000FF;

pub const LPFC_SLI_INTF_SLI_HINT1_NONE: c_int = 0;
pub const LPFC_SLI_INTF_SLI_HINT1_1: c_int = 1;
pub const LPFC_SLI_INTF_SLI_HINT1_2: c_int = 2;
pub const lpfc_sli_intf_if_type_SHIFT: c_int = 12;
pub const lpfc_sli_intf_if_type_MASK: c_uint = 0x0000000F;

pub const LPFC_SLI_INTF_IF_TYPE_0: c_int = 0;
pub const LPFC_SLI_INTF_IF_TYPE_1: c_int = 1;
pub const LPFC_SLI_INTF_IF_TYPE_2: c_int = 2;
pub const LPFC_SLI_INTF_IF_TYPE_6: c_int = 6;
pub const lpfc_sli_intf_sli_family_SHIFT: c_int = 8;
pub const lpfc_sli_intf_sli_family_MASK: c_uint = 0x0000000F;

pub const LPFC_SLI_INTF_FAMILY_BE2: c_uint = 0x0;
pub const LPFC_SLI_INTF_ASIC_ID: c_uint = 0x1	/* Refer to ASIC_ID register */;
pub const LPFC_SLI_INTF_FAMILY_BE3: c_uint = 0x3;
pub const LPFC_SLI_INTF_FAMILY_LNCR_A0: c_uint = 0xa;
pub const LPFC_SLI_INTF_FAMILY_LNCR_B0: c_uint = 0xb;
pub const LPFC_SLI_INTF_FAMILY_G6: c_uint = 0xc;
pub const LPFC_SLI_INTF_FAMILY_G7: c_uint = 0xd;
pub const LPFC_SLI_INTF_FAMILY_G7P: c_uint = 0xe;
pub const lpfc_sli_intf_slirev_SHIFT: c_int = 4;
pub const lpfc_sli_intf_slirev_MASK: c_uint = 0x0000000F;

pub const LPFC_SLI_INTF_REV_SLI3: c_int = 3;
pub const LPFC_SLI_INTF_REV_SLI4: c_int = 4;
pub const lpfc_sli_intf_func_type_SHIFT: c_int = 0;
pub const lpfc_sli_intf_func_type_MASK: c_uint = 0x00000001;

pub const LPFC_SLI_INTF_IF_TYPE_PHYS: c_int = 0;
pub const LPFC_SLI_INTF_IF_TYPE_VIRT: c_int = 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_asic_id {
    pub word0: u32,
pub const lpfc_asic_id_gen_num_SHIFT: c_int = 8;
pub const lpfc_asic_id_gen_num_MASK: c_uint = 0x000000FF;

pub const LPFC_SLI_INTF_FAMILY_G8: c_uint = 0x10;
pub const lpfc_asic_id_rev_num_SHIFT: c_int = 0;
pub const lpfc_asic_id_rev_num_MASK: c_uint = 0x000000FF;

}

pub const LPFC_SLI4_MB_WORD_COUNT: c_int = 64;
pub const LPFC_MAX_MQ_PAGE: c_int = 8;
pub const LPFC_MAX_WQ_PAGE_V0: c_int = 4;
pub const LPFC_MAX_WQ_PAGE: c_int = 8;
pub const LPFC_MAX_RQ_PAGE: c_int = 8;
pub const LPFC_MAX_CQ_PAGE: c_int = 4;
pub const LPFC_MAX_EQ_PAGE: c_int = 8;

pub const LPFC_VFR_PAGE_SIZE: c_uint = 0x1000 /* 4KB BAR2 per-VF register page size */;
// Define SLI4 Alignment requirements.
pub const LPFC_ALIGN_16_BYTE: c_int = 16;
pub const LPFC_ALIGN_64_BYTE: c_int = 64;
pub const SLI4_PAGE_SIZE: c_int = 4096;
// Define SLI4 specific definitions.
pub const LPFC_MQ_CQE_BYTE_OFFSET: c_int = 256;
pub const LPFC_MBX_CMD_HDR_LENGTH: c_int = 16;
pub const LPFC_MBX_ERROR_RANGE: c_uint = 0x4000;
pub const LPFC_BMBX_BIT1_ADDR_HI: c_uint = 0x2;
pub const LPFC_BMBX_BIT1_ADDR_LO: c_int = 0;
pub const LPFC_RPI_HDR_COUNT: c_int = 64;
pub const LPFC_HDR_TEMPLATE_SIZE: c_int = 4096;
pub const LPFC_RPI_ALLOC_ERROR: c_uint = 0xFFFF;
pub const LPFC_FCF_RECORD_WD_CNT: c_int = 132;
pub const LPFC_ENTIRE_FCF_DATABASE: c_int = 0;
pub const LPFC_DFLT_FCF_INDEX: c_int = 0;
// Virtual function numbers
pub const LPFC_VF0: c_int = 0;
pub const LPFC_VF1: c_int = 1;
pub const LPFC_VF2: c_int = 2;
pub const LPFC_VF3: c_int = 3;
pub const LPFC_VF4: c_int = 4;
pub const LPFC_VF5: c_int = 5;
pub const LPFC_VF6: c_int = 6;
pub const LPFC_VF7: c_int = 7;
pub const LPFC_VF8: c_int = 8;
pub const LPFC_VF9: c_int = 9;
pub const LPFC_VF10: c_int = 10;
pub const LPFC_VF11: c_int = 11;
pub const LPFC_VF12: c_int = 12;
pub const LPFC_VF13: c_int = 13;
pub const LPFC_VF14: c_int = 14;
pub const LPFC_VF15: c_int = 15;
pub const LPFC_VF16: c_int = 16;
pub const LPFC_VF17: c_int = 17;
pub const LPFC_VF18: c_int = 18;
pub const LPFC_VF19: c_int = 19;
pub const LPFC_VF20: c_int = 20;
pub const LPFC_VF21: c_int = 21;
pub const LPFC_VF22: c_int = 22;
pub const LPFC_VF23: c_int = 23;
pub const LPFC_VF24: c_int = 24;
pub const LPFC_VF25: c_int = 25;
pub const LPFC_VF26: c_int = 26;
pub const LPFC_VF27: c_int = 27;
pub const LPFC_VF28: c_int = 28;
pub const LPFC_VF29: c_int = 29;
pub const LPFC_VF30: c_int = 30;
pub const LPFC_VF31: c_int = 31;
// PCI function numbers
pub const LPFC_PCI_FUNC0: c_int = 0;
pub const LPFC_PCI_FUNC1: c_int = 1;
pub const LPFC_PCI_FUNC2: c_int = 2;
pub const LPFC_PCI_FUNC3: c_int = 3;
pub const LPFC_PCI_FUNC4: c_int = 4;
// SLI4 interface type-2 PDEV_CTL register
pub const LPFC_CTL_PDEV_CTL_OFFSET: c_uint = 0x414;
pub const LPFC_CTL_PDEV_CTL_DRST: c_uint = 0x00000001;
pub const LPFC_CTL_PDEV_CTL_FRST: c_uint = 0x00000002;
pub const LPFC_CTL_PDEV_CTL_DD: c_uint = 0x00000004;
pub const LPFC_CTL_PDEV_CTL_LC: c_uint = 0x00000008;
pub const LPFC_CTL_PDEV_CTL_FRL_ALL: c_uint = 0x00;
pub const LPFC_CTL_PDEV_CTL_FRL_FC_FCOE: c_uint = 0x10;
pub const LPFC_CTL_PDEV_CTL_FRL_NIC: c_uint = 0x20;
pub const LPFC_CTL_PDEV_CTL_DDL_RAS: c_uint = 0x1000000;

// Active interrupt test count
pub const LPFC_ACT_INTR_CNT: c_int = 4;
// Algrithmns for scheduling FCP commands to WQs
pub const LPFC_FCP_SCHED_BY_HDWQ: c_int = 0;
pub const LPFC_FCP_SCHED_BY_CPU: c_int = 1;
// Algrithmns for NameServer Query after RSCN
pub const LPFC_NS_QUERY_GID_FT: c_int = 0;
pub const LPFC_NS_QUERY_GID_PT: c_int = 1;
// Delay Multiplier constant
pub const LPFC_DMULT_CONST: c_int = 651042;
pub const LPFC_DMULT_MAX: c_int = 1023;
// Configuration of Interrupts / sec for entire HBA port
pub const LPFC_MIN_IMAX: c_int = 5000;
pub const LPFC_MAX_IMAX: c_int = 5000000;
pub const LPFC_DEF_IMAX: c_int = 0;
pub const LPFC_MAX_AUTO_EQ_DELAY: c_int = 120;
pub const LPFC_EQ_DELAY_STEP: c_int = 15;
pub const LPFC_EQD_ISR_TRIGGER: c_int = 20000;
// 1s intervals
pub const LPFC_EQ_DELAY_MSECS: c_int = 1000;
pub const LPFC_MIN_CPU_MAP: c_int = 0;
pub const LPFC_MAX_CPU_MAP: c_int = 1;
pub const LPFC_HBA_CPU_MAP: c_int = 1;
// PORT_CAPABILITIES constants.
pub const LPFC_MAX_SUPPORTED_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ulp_bde64_word3 {
    ULP_BDE64_SIZE_MASK		= 0xffffff,

    ULP_BDE64_TYPE_SHIFT		= 24,
    ULP_BDE64_TYPE_MASK		= (0xff << ULP_BDE64_TYPE_SHIFT),

// BDE (Host_resident)
    ULP_BDE64_TYPE_BDE_64		= (0x00 << ULP_BDE64_TYPE_SHIFT),
// Immediate Data BDE
    ULP_BDE64_TYPE_BDE_IMMED	= (0x01 << ULP_BDE64_TYPE_SHIFT),
// BDE (Port-resident)
    ULP_BDE64_TYPE_BDE_64P		= (0x02 << ULP_BDE64_TYPE_SHIFT),
// Input BDE (Host-resident)
    ULP_BDE64_TYPE_BDE_64I		= (0x08 << ULP_BDE64_TYPE_SHIFT),
// Input BDE (Port-resident)
    ULP_BDE64_TYPE_BDE_64IP		= (0x0A << ULP_BDE64_TYPE_SHIFT),
// BLP (Host-resident)
    ULP_BDE64_TYPE_BLP_64		= (0x40 << ULP_BDE64_TYPE_SHIFT),
// BLP (Port-resident)
    ULP_BDE64_TYPE_BLP_64P		= (0x42 << ULP_BDE64_TYPE_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_bde64_le {
    pub /: *mut *mut __le32 type_size; / type 31:24, size 23:0,
    pub addr_low: __le32,
    pub addr_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_bde64 {
#[repr(C)]
#[derive(Copy, Clone)]
pub union ULP_BDE_TUS {
    pub w: u32,

    pub SUPPORTED: *mut *mut uint32_t bdeFlags:8; / BDE Flags 0 IS A,
    pub /: *mut *mut uint32_t bdeSize:24; / Size of buffer (in bytes),

    pub /: *mut *mut uint32_t bdeSize:24; / Size of buffer (in bytes),
    pub SUPPORTED: *mut *mut uint32_t bdeFlags:8; / BDE Flags 0 IS A,

pub const BUFF_TYPE_BDE_64: c_uint = 0x00	/* BDE (Host_resident) */;
pub const BUFF_TYPE_BDE_IMMED: c_uint = 0x01	/* Immediate Data BDE */;
pub const BUFF_TYPE_BDE_64P: c_uint = 0x02	/* BDE (Port-resident) */;
pub const BUFF_TYPE_BDE_64I: c_uint = 0x08	/* Input BDE (Host-resident) */;
pub const BUFF_TYPE_BDE_64IP: c_uint = 0x0A	/* Input BDE (Port-resident) */;
pub const BUFF_TYPE_BLP_64: c_uint = 0x40	/* BLP (Host-resident) */;
pub const BUFF_TYPE_BLP_64P: c_uint = 0x42	/* BLP (Port-resident) */;
    pub f: },
    pub tus: },
    pub addrLow: u32,
    pub addrHigh: u32,
}

// Maximun size of immediate data that can fit into a 128 byte WQE
pub const LPFC_MAX_BDE_IMM_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_flags {
    pub word0: u32,
pub const lpfc_idx_rsrc_rdy_SHIFT: c_int = 0;
pub const lpfc_idx_rsrc_rdy_MASK: c_uint = 0x00000001;

pub const LPFC_IDX_RSRC_RDY: c_int = 1;
pub const lpfc_rpi_rsrc_rdy_SHIFT: c_int = 1;
pub const lpfc_rpi_rsrc_rdy_MASK: c_uint = 0x00000001;

pub const LPFC_RPI_RSRC_RDY: c_int = 1;
pub const lpfc_vpi_rsrc_rdy_SHIFT: c_int = 2;
pub const lpfc_vpi_rsrc_rdy_MASK: c_uint = 0x00000001;

pub const LPFC_VPI_RSRC_RDY: c_int = 1;
pub const lpfc_vfi_rsrc_rdy_SHIFT: c_int = 3;
pub const lpfc_vfi_rsrc_rdy_MASK: c_uint = 0x00000001;

pub const LPFC_VFI_RSRC_RDY: c_int = 1;
pub const lpfc_ftr_ashdr_SHIFT: c_int = 4;
pub const lpfc_ftr_ashdr_MASK: c_uint = 0x00000001;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_bls_rsp {
    pub /: *mut *mut uint32_t word0_rsvd; / Word0 must be reserved,
    pub word1: u32,
pub const lpfc_abts_orig_SHIFT: c_int = 0;
pub const lpfc_abts_orig_MASK: c_uint = 0x00000001;

pub const LPFC_ABTS_UNSOL_RSP: c_int = 1;
pub const LPFC_ABTS_UNSOL_INT: c_int = 0;
    pub word2: u32,
pub const lpfc_abts_rxid_SHIFT: c_int = 0;
pub const lpfc_abts_rxid_MASK: c_uint = 0x0000FFFF;

pub const lpfc_abts_oxid_SHIFT: c_int = 16;
pub const lpfc_abts_oxid_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,
pub const lpfc_vndr_code_SHIFT: c_int = 0;
pub const lpfc_vndr_code_MASK: c_uint = 0x000000FF;

pub const lpfc_rsn_expln_SHIFT: c_int = 8;
pub const lpfc_rsn_expln_MASK: c_uint = 0x000000FF;

pub const lpfc_rsn_code_SHIFT: c_int = 16;
pub const lpfc_rsn_code_MASK: c_uint = 0x000000FF;

    pub word4: u32,
    pub /: *mut *mut uint32_t word5_rsvd; / Word5 must be reserved,
}

// event queue entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_eqe {
    pub word0: u32,
pub const lpfc_eqe_resource_id_SHIFT: c_int = 16;
pub const lpfc_eqe_resource_id_MASK: c_uint = 0x0000FFFF;

pub const lpfc_eqe_minor_code_SHIFT: c_int = 4;
pub const lpfc_eqe_minor_code_MASK: c_uint = 0x00000FFF;

pub const lpfc_eqe_major_code_SHIFT: c_int = 1;
pub const lpfc_eqe_major_code_MASK: c_uint = 0x00000007;

pub const lpfc_eqe_valid_SHIFT: c_int = 0;
pub const lpfc_eqe_valid_MASK: c_uint = 0x00000001;

}

// completion queue entry structure (common fields for all cqe types)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cqe {
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub word3: u32,
pub const lpfc_cqe_valid_SHIFT: c_int = 31;
pub const lpfc_cqe_valid_MASK: c_uint = 0x00000001;

pub const lpfc_cqe_code_SHIFT: c_int = 16;
pub const lpfc_cqe_code_MASK: c_uint = 0x000000FF;

}

// Completion Queue Entry Status Codes
pub const CQE_STATUS_SUCCESS: c_uint = 0x0;
pub const CQE_STATUS_FCP_RSP_FAILURE: c_uint = 0x1;
pub const CQE_STATUS_REMOTE_STOP: c_uint = 0x2;
pub const CQE_STATUS_LOCAL_REJECT: c_uint = 0x3;
pub const CQE_STATUS_NPORT_RJT: c_uint = 0x4;
pub const CQE_STATUS_FABRIC_RJT: c_uint = 0x5;
pub const CQE_STATUS_NPORT_BSY: c_uint = 0x6;
pub const CQE_STATUS_FABRIC_BSY: c_uint = 0x7;
pub const CQE_STATUS_INTERMED_RSP: c_uint = 0x8;
pub const CQE_STATUS_LS_RJT: c_uint = 0x9;
pub const CQE_STATUS_CMD_REJECT: c_uint = 0xb;
pub const CQE_STATUS_FCP_TGT_LENCHECK: c_uint = 0xc;
pub const CQE_STATUS_NEED_BUFF_ENTRY: c_uint = 0xf;
pub const CQE_STATUS_DI_ERROR: c_uint = 0x16;
// Status returned by hardware (valid only if status = CQE_STATUS_SUCCESS).
pub const CQE_HW_STATUS_NO_ERR: c_uint = 0x0;
pub const CQE_HW_STATUS_UNDERRUN: c_uint = 0x1;
pub const CQE_HW_STATUS_OVERRUN: c_uint = 0x2;
// Completion Queue Entry Codes
pub const CQE_CODE_COMPL_WQE: c_uint = 0x1;
pub const CQE_CODE_RELEASE_WQE: c_uint = 0x2;
pub const CQE_CODE_RECEIVE: c_uint = 0x4;
pub const CQE_CODE_XRI_ABORTED: c_uint = 0x5;
pub const CQE_CODE_RECEIVE_V1: c_uint = 0x9;
pub const CQE_CODE_NVME_ERSP: c_uint = 0xd;
//
// Define mask value for xri_aborted and wcqe completed CQE extended status.
// Currently, extended status is limited to 9 bits (0x0 -> 0x103) .
//
pub const WCQE_PARAM_MASK: c_uint = 0x1FF;
// completion queue entry for wqe completions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_wcqe_complete {
    pub word0: u32,
pub const lpfc_wcqe_c_request_tag_SHIFT: c_int = 16;
pub const lpfc_wcqe_c_request_tag_MASK: c_uint = 0x0000FFFF;

pub const lpfc_wcqe_c_status_SHIFT: c_int = 8;
pub const lpfc_wcqe_c_status_MASK: c_uint = 0x000000FF;

pub const lpfc_wcqe_c_hw_status_SHIFT: c_int = 0;
pub const lpfc_wcqe_c_hw_status_MASK: c_uint = 0x000000FF;

pub const lpfc_wcqe_c_ersp0_SHIFT: c_int = 0;
pub const lpfc_wcqe_c_ersp0_MASK: c_uint = 0x0000FFFF;

    pub total_data_placed: u32,
pub const lpfc_wcqe_c_cmf_cg_SHIFT: c_int = 31;
pub const lpfc_wcqe_c_cmf_cg_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_cmf_bw_SHIFT: c_int = 0;
pub const lpfc_wcqe_c_cmf_bw_MASK: c_uint = 0x0FFFFFFF;

    pub parameter: u32,
pub const lpfc_wcqe_c_enc_SHIFT: c_int = 31;
pub const lpfc_wcqe_c_enc_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_enc_lvl_SHIFT: c_int = 30;
pub const lpfc_wcqe_c_enc_lvl_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_bg_edir_SHIFT: c_int = 5;
pub const lpfc_wcqe_c_bg_edir_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_bg_tdpv_SHIFT: c_int = 3;
pub const lpfc_wcqe_c_bg_tdpv_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_bg_re_SHIFT: c_int = 2;
pub const lpfc_wcqe_c_bg_re_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_bg_ae_SHIFT: c_int = 1;
pub const lpfc_wcqe_c_bg_ae_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_bg_ge_SHIFT: c_int = 0;
pub const lpfc_wcqe_c_bg_ge_MASK: c_uint = 0x00000001;

    pub word3: u32,

pub const lpfc_wcqe_c_xb_SHIFT: c_int = 28;
pub const lpfc_wcqe_c_xb_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_pv_SHIFT: c_int = 27;
pub const lpfc_wcqe_c_pv_MASK: c_uint = 0x00000001;

pub const lpfc_wcqe_c_priority_SHIFT: c_int = 24;
pub const lpfc_wcqe_c_priority_MASK: c_uint = 0x00000007;

pub const lpfc_wcqe_c_sqhead_SHIFT: c_int = 0;
pub const lpfc_wcqe_c_sqhead_MASK: c_uint = 0x0000FFFF;

}

// completion queue entry for wqe release
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_wcqe_release {
    pub reserved0: u32,
    pub reserved1: u32,
    pub word2: u32,
pub const lpfc_wcqe_r_wq_id_SHIFT: c_int = 16;
pub const lpfc_wcqe_r_wq_id_MASK: c_uint = 0x0000FFFF;

pub const lpfc_wcqe_r_wqe_index_SHIFT: c_int = 0;
pub const lpfc_wcqe_r_wqe_index_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_wcqe_xri_aborted {
    pub word0: u32,
pub const lpfc_wcqe_xa_status_SHIFT: c_int = 8;
pub const lpfc_wcqe_xa_status_MASK: c_uint = 0x000000FF;

    pub parameter: u32,
    pub word2: u32,
pub const lpfc_wcqe_xa_remote_xid_SHIFT: c_int = 16;
pub const lpfc_wcqe_xa_remote_xid_MASK: c_uint = 0x0000FFFF;

pub const lpfc_wcqe_xa_xri_SHIFT: c_int = 0;
pub const lpfc_wcqe_xa_xri_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,

pub const lpfc_wcqe_xa_ia_SHIFT: c_int = 30;
pub const lpfc_wcqe_xa_ia_MASK: c_uint = 0x00000001;

pub const CQE_XRI_ABORTED_IA_REMOTE: c_int = 0;
pub const CQE_XRI_ABORTED_IA_LOCAL: c_int = 1;
pub const lpfc_wcqe_xa_br_SHIFT: c_int = 29;
pub const lpfc_wcqe_xa_br_MASK: c_uint = 0x00000001;

pub const CQE_XRI_ABORTED_BR_BA_ACC: c_int = 0;
pub const CQE_XRI_ABORTED_BR_BA_RJT: c_int = 1;
pub const lpfc_wcqe_xa_eo_SHIFT: c_int = 28;
pub const lpfc_wcqe_xa_eo_MASK: c_uint = 0x00000001;

pub const CQE_XRI_ABORTED_EO_REMOTE: c_int = 0;
pub const CQE_XRI_ABORTED_EO_LOCAL: c_int = 1;

}

// completion queue entry structure for rqe completion
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rcqe {
    pub word0: u32,
pub const lpfc_rcqe_iv_SHIFT: c_int = 31;
pub const lpfc_rcqe_iv_MASK: c_uint = 0x00000001;

pub const lpfc_rcqe_status_SHIFT: c_int = 8;
pub const lpfc_rcqe_status_MASK: c_uint = 0x000000FF;

pub const FC_STATUS_RQ_SUCCESS: c_uint = 0x10 /* Async receive successful */;
pub const FC_STATUS_RQ_BUF_LEN_EXCEEDED: c_uint = 0x11 /* payload truncated */;
pub const FC_STATUS_INSUFF_BUF_NEED_BUF: c_uint = 0x12 /* Insufficient buffers */;
pub const FC_STATUS_INSUFF_BUF_FRM_DISC: c_uint = 0x13 /* Frame Discard */;
pub const FC_STATUS_RQ_DMA_FAILURE: c_uint = 0x14 /* DMA failure */;
    pub word1: u32,
pub const lpfc_rcqe_fcf_id_v1_SHIFT: c_int = 0;
pub const lpfc_rcqe_fcf_id_v1_MASK: c_uint = 0x0000003F;

    pub word2: u32,
pub const lpfc_rcqe_length_SHIFT: c_int = 16;
pub const lpfc_rcqe_length_MASK: c_uint = 0x0000FFFF;

pub const lpfc_rcqe_rq_id_SHIFT: c_int = 6;
pub const lpfc_rcqe_rq_id_MASK: c_uint = 0x000003FF;

pub const lpfc_rcqe_fcf_id_SHIFT: c_int = 0;
pub const lpfc_rcqe_fcf_id_MASK: c_uint = 0x0000003F;

pub const lpfc_rcqe_rq_id_v1_SHIFT: c_int = 0;
pub const lpfc_rcqe_rq_id_v1_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,

pub const lpfc_rcqe_port_SHIFT: c_int = 30;
pub const lpfc_rcqe_port_MASK: c_uint = 0x00000001;

pub const lpfc_rcqe_hdr_length_SHIFT: c_int = 24;
pub const lpfc_rcqe_hdr_length_MASK: c_uint = 0x0000001F;

pub const lpfc_rcqe_eof_SHIFT: c_int = 8;
pub const lpfc_rcqe_eof_MASK: c_uint = 0x000000FF;

pub const FCOE_EOFn: c_uint = 0x41;
pub const FCOE_EOFt: c_uint = 0x42;
pub const FCOE_EOFni: c_uint = 0x49;
pub const FCOE_EOFa: c_uint = 0x50;
pub const lpfc_rcqe_sof_SHIFT: c_int = 0;
pub const lpfc_rcqe_sof_MASK: c_uint = 0x000000FF;

pub const FCOE_SOFi2: c_uint = 0x2d;
pub const FCOE_SOFi3: c_uint = 0x2e;
pub const FCOE_SOFn2: c_uint = 0x35;
pub const FCOE_SOFn3: c_uint = 0x36;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rqe {
    pub address_hi: u32,
    pub address_lo: u32,
}

// buffer descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bde4 {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub word2: u32,
pub const lpfc_bde4_last_SHIFT: c_int = 31;
pub const lpfc_bde4_last_MASK: c_uint = 0x00000001;

pub const lpfc_bde4_sge_offset_SHIFT: c_int = 0;
pub const lpfc_bde4_sge_offset_MASK: c_uint = 0x000003FF;

    pub word3: u32,
pub const lpfc_bde4_length_SHIFT: c_int = 0;
pub const lpfc_bde4_length_MASK: c_uint = 0x000000FF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_register {
    pub word0: u32,
}

pub const LPFC_PORT_SEM_UE_RECOVERABLE: c_uint = 0xE000;
pub const LPFC_PORT_SEM_MASK: c_uint = 0xF000;
// The following are config space register offsets
pub const LPFC_ASIC_ID_OFFSET: c_uint = 0x0308;
// The following BAR0 Registers apply to SLI4 if_type 0 UCNAs.
pub const LPFC_UERR_STATUS_HI: c_uint = 0x00A4;
pub const LPFC_UERR_STATUS_LO: c_uint = 0x00A0;
pub const LPFC_UE_MASK_HI: c_uint = 0x00AC;
pub const LPFC_UE_MASK_LO: c_uint = 0x00A8;
// The following BAR0 register sets are defined for if_type 0 and 2 UCNAs.
pub const LPFC_SLI_INTF: c_uint = 0x0058;
pub const LPFC_CTL_PORT_SEM_OFFSET: c_uint = 0x400;
pub const lpfc_port_smphr_perr_SHIFT: c_int = 31;
pub const lpfc_port_smphr_perr_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_sfi_SHIFT: c_int = 30;
pub const lpfc_port_smphr_sfi_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_nip_SHIFT: c_int = 29;
pub const lpfc_port_smphr_nip_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_ipc_SHIFT: c_int = 28;
pub const lpfc_port_smphr_ipc_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_scr1_SHIFT: c_int = 27;
pub const lpfc_port_smphr_scr1_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_scr2_SHIFT: c_int = 26;
pub const lpfc_port_smphr_scr2_MASK: c_uint = 0x1;

pub const lpfc_port_smphr_host_scratch_SHIFT: c_int = 16;
pub const lpfc_port_smphr_host_scratch_MASK: c_uint = 0xFF;

pub const lpfc_port_smphr_port_status_SHIFT: c_int = 0;
pub const lpfc_port_smphr_port_status_MASK: c_uint = 0xFFFF;

pub const LPFC_POST_STAGE_POWER_ON_RESET: c_uint = 0x0000;
pub const LPFC_POST_STAGE_AWAITING_HOST_RDY: c_uint = 0x0001;
pub const LPFC_POST_STAGE_HOST_RDY: c_uint = 0x0002;
pub const LPFC_POST_STAGE_BE_RESET: c_uint = 0x0003;
pub const LPFC_POST_STAGE_SEEPROM_CS_START: c_uint = 0x0100;
pub const LPFC_POST_STAGE_SEEPROM_CS_DONE: c_uint = 0x0101;
pub const LPFC_POST_STAGE_DDR_CONFIG_START: c_uint = 0x0200;
pub const LPFC_POST_STAGE_DDR_CONFIG_DONE: c_uint = 0x0201;
pub const LPFC_POST_STAGE_DDR_CALIBRATE_START: c_uint = 0x0300;
pub const LPFC_POST_STAGE_DDR_CALIBRATE_DONE: c_uint = 0x0301;
pub const LPFC_POST_STAGE_DDR_TEST_START: c_uint = 0x0400;
pub const LPFC_POST_STAGE_DDR_TEST_DONE: c_uint = 0x0401;
pub const LPFC_POST_STAGE_REDBOOT_INIT_START: c_uint = 0x0600;
pub const LPFC_POST_STAGE_REDBOOT_INIT_DONE: c_uint = 0x0601;
pub const LPFC_POST_STAGE_FW_IMAGE_LOAD_START: c_uint = 0x0700;
pub const LPFC_POST_STAGE_FW_IMAGE_LOAD_DONE: c_uint = 0x0701;
pub const LPFC_POST_STAGE_ARMFW_START: c_uint = 0x0800;
pub const LPFC_POST_STAGE_DHCP_QUERY_START: c_uint = 0x0900;
pub const LPFC_POST_STAGE_DHCP_QUERY_DONE: c_uint = 0x0901;
pub const LPFC_POST_STAGE_BOOT_TARGET_DISCOVERY_START: c_uint = 0x0A00;
pub const LPFC_POST_STAGE_BOOT_TARGET_DISCOVERY_DONE: c_uint = 0x0A01;
pub const LPFC_POST_STAGE_RC_OPTION_SET: c_uint = 0x0B00;
pub const LPFC_POST_STAGE_SWITCH_LINK: c_uint = 0x0B01;
pub const LPFC_POST_STAGE_SEND_ICDS_MESSAGE: c_uint = 0x0B02;
pub const LPFC_POST_STAGE_PERFROM_TFTP: c_uint = 0x0B03;
pub const LPFC_POST_STAGE_PARSE_XML: c_uint = 0x0B04;
pub const LPFC_POST_STAGE_DOWNLOAD_IMAGE: c_uint = 0x0B05;
pub const LPFC_POST_STAGE_FLASH_IMAGE: c_uint = 0x0B06;
pub const LPFC_POST_STAGE_RC_DONE: c_uint = 0x0B07;
pub const LPFC_POST_STAGE_REBOOT_SYSTEM: c_uint = 0x0B08;
pub const LPFC_POST_STAGE_MAC_ADDRESS: c_uint = 0x0C00;
pub const LPFC_POST_STAGE_PORT_READY: c_uint = 0xC000;
pub const LPFC_POST_STAGE_PORT_UE: c_uint = 0xF000;
pub const LPFC_CTL_PORT_STA_OFFSET: c_uint = 0x404;
pub const lpfc_sliport_status_err_SHIFT: c_int = 31;
pub const lpfc_sliport_status_err_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_end_SHIFT: c_int = 30;
pub const lpfc_sliport_status_end_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_oti_SHIFT: c_int = 29;
pub const lpfc_sliport_status_oti_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_dip_SHIFT: c_int = 25;
pub const lpfc_sliport_status_dip_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_rn_SHIFT: c_int = 24;
pub const lpfc_sliport_status_rn_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_rdy_SHIFT: c_int = 23;
pub const lpfc_sliport_status_rdy_MASK: c_uint = 0x1;

pub const lpfc_sliport_status_pldv_SHIFT: c_int = 0;
pub const lpfc_sliport_status_pldv_MASK: c_uint = 0x1;

pub const CFG_PLD: c_uint = 0x3C;
pub const MAX_IF_TYPE_2_RESETS: c_int = 6;
pub const LPFC_CTL_PORT_CTL_OFFSET: c_uint = 0x408;
pub const lpfc_sliport_ctrl_end_SHIFT: c_int = 30;
pub const lpfc_sliport_ctrl_end_MASK: c_uint = 0x1;

pub const LPFC_SLIPORT_LITTLE_ENDIAN: c_int = 0;
pub const LPFC_SLIPORT_BIG_ENDIAN: c_int = 1;
pub const lpfc_sliport_ctrl_ip_SHIFT: c_int = 27;
pub const lpfc_sliport_ctrl_ip_MASK: c_uint = 0x1;

pub const LPFC_SLIPORT_INIT_PORT: c_int = 1;
pub const LPFC_CTL_PORT_ER1_OFFSET: c_uint = 0x40C;
pub const LPFC_CTL_PORT_ER2_OFFSET: c_uint = 0x410;
pub const LPFC_CTL_PORT_EQ_DELAY_OFFSET: c_uint = 0x418;
pub const lpfc_sliport_eqdelay_delay_SHIFT: c_int = 16;
pub const lpfc_sliport_eqdelay_delay_MASK: c_uint = 0xffff;

pub const lpfc_sliport_eqdelay_id_SHIFT: c_int = 0;
pub const lpfc_sliport_eqdelay_id_MASK: c_uint = 0xfff;

pub const LPFC_SEC_TO_USEC: c_int = 1000000;
pub const LPFC_SEC_TO_MSEC: c_int = 1000;

// The following Registers apply to SLI4 if_type 0 UCNAs. They typically
// reside in BAR 2.
//
pub const LPFC_SLIPORT_IF0_SMPHR: c_uint = 0x00AC;
pub const LPFC_IMR_MASK_ALL: c_uint = 0xFFFFFFFF;
pub const LPFC_ISCR_CLEAR_ALL: c_uint = 0xFFFFFFFF;
pub const LPFC_HST_ISR0: c_uint = 0x0C18;
pub const LPFC_HST_ISR1: c_uint = 0x0C1C;
pub const LPFC_HST_ISR2: c_uint = 0x0C20;
pub const LPFC_HST_ISR3: c_uint = 0x0C24;
pub const LPFC_HST_ISR4: c_uint = 0x0C28;
pub const LPFC_HST_IMR0: c_uint = 0x0C48;
pub const LPFC_HST_IMR1: c_uint = 0x0C4C;
pub const LPFC_HST_IMR2: c_uint = 0x0C50;
pub const LPFC_HST_IMR3: c_uint = 0x0C54;
pub const LPFC_HST_IMR4: c_uint = 0x0C58;
pub const LPFC_HST_ISCR0: c_uint = 0x0C78;
pub const LPFC_HST_ISCR1: c_uint = 0x0C7C;
pub const LPFC_HST_ISCR2: c_uint = 0x0C80;
pub const LPFC_HST_ISCR3: c_uint = 0x0C84;
pub const LPFC_HST_ISCR4: c_uint = 0x0C88;

//
// The Doorbell registers defined here exist in different BAR
// register sets depending on the UCNA Port's reported if_type
// value.  For UCNA ports running SLI4 and if_type 0, they reside in
// BAR4.  For UCNA ports running SLI4 and if_type 2, they reside in
// BAR0.  For FC ports running SLI4 and if_type 6, they reside in
// BAR2. The offsets and base address are different,  so the driver
// has to compute the register addresses accordingly
//
pub const LPFC_ULP0_RQ_DOORBELL: c_uint = 0x00A0;
pub const LPFC_ULP1_RQ_DOORBELL: c_uint = 0x00C0;
pub const LPFC_IF6_RQ_DOORBELL: c_uint = 0x0080;
pub const lpfc_rq_db_list_fm_num_posted_SHIFT: c_int = 24;
pub const lpfc_rq_db_list_fm_num_posted_MASK: c_uint = 0x00FF;

pub const lpfc_rq_db_list_fm_index_SHIFT: c_int = 16;
pub const lpfc_rq_db_list_fm_index_MASK: c_uint = 0x00FF;

pub const lpfc_rq_db_list_fm_id_SHIFT: c_int = 0;
pub const lpfc_rq_db_list_fm_id_MASK: c_uint = 0xFFFF;

pub const lpfc_rq_db_ring_fm_num_posted_SHIFT: c_int = 16;
pub const lpfc_rq_db_ring_fm_num_posted_MASK: c_uint = 0x3FFF;

pub const lpfc_rq_db_ring_fm_id_SHIFT: c_int = 0;
pub const lpfc_rq_db_ring_fm_id_MASK: c_uint = 0xFFFF;

pub const LPFC_ULP0_WQ_DOORBELL: c_uint = 0x0040;
pub const LPFC_ULP1_WQ_DOORBELL: c_uint = 0x0060;
pub const lpfc_wq_db_list_fm_num_posted_SHIFT: c_int = 24;
pub const lpfc_wq_db_list_fm_num_posted_MASK: c_uint = 0x00FF;

pub const lpfc_wq_db_list_fm_index_SHIFT: c_int = 16;
pub const lpfc_wq_db_list_fm_index_MASK: c_uint = 0x00FF;

pub const lpfc_wq_db_list_fm_id_SHIFT: c_int = 0;
pub const lpfc_wq_db_list_fm_id_MASK: c_uint = 0xFFFF;

pub const lpfc_wq_db_ring_fm_num_posted_SHIFT: c_int = 16;
pub const lpfc_wq_db_ring_fm_num_posted_MASK: c_uint = 0x3FFF;

pub const lpfc_wq_db_ring_fm_id_SHIFT: c_int = 0;
pub const lpfc_wq_db_ring_fm_id_MASK: c_uint = 0xFFFF;

pub const LPFC_IF6_WQ_DOORBELL: c_uint = 0x0040;
pub const lpfc_if6_wq_db_list_fm_num_posted_SHIFT: c_int = 24;
pub const lpfc_if6_wq_db_list_fm_num_posted_MASK: c_uint = 0x00FF;

pub const lpfc_if6_wq_db_list_fm_dpp_SHIFT: c_int = 23;
pub const lpfc_if6_wq_db_list_fm_dpp_MASK: c_uint = 0x0001;

pub const lpfc_if6_wq_db_list_fm_dpp_id_SHIFT: c_int = 16;
pub const lpfc_if6_wq_db_list_fm_dpp_id_MASK: c_uint = 0x001F;

pub const lpfc_if6_wq_db_list_fm_id_SHIFT: c_int = 0;
pub const lpfc_if6_wq_db_list_fm_id_MASK: c_uint = 0xFFFF;

pub const LPFC_EQCQ_DOORBELL: c_uint = 0x0120;
pub const lpfc_eqcq_doorbell_se_SHIFT: c_int = 31;
pub const lpfc_eqcq_doorbell_se_MASK: c_uint = 0x0001;

pub const LPFC_EQCQ_SOLICIT_ENABLE_OFF: c_int = 0;
pub const LPFC_EQCQ_SOLICIT_ENABLE_ON: c_int = 1;
pub const lpfc_eqcq_doorbell_arm_SHIFT: c_int = 29;
pub const lpfc_eqcq_doorbell_arm_MASK: c_uint = 0x0001;

pub const lpfc_eqcq_doorbell_num_released_SHIFT: c_int = 16;
pub const lpfc_eqcq_doorbell_num_released_MASK: c_uint = 0x1FFF;

pub const lpfc_eqcq_doorbell_qt_SHIFT: c_int = 10;
pub const lpfc_eqcq_doorbell_qt_MASK: c_uint = 0x0001;

pub const LPFC_QUEUE_TYPE_COMPLETION: c_int = 0;
pub const LPFC_QUEUE_TYPE_EVENT: c_int = 1;
pub const lpfc_eqcq_doorbell_eqci_SHIFT: c_int = 9;
pub const lpfc_eqcq_doorbell_eqci_MASK: c_uint = 0x0001;

pub const lpfc_eqcq_doorbell_cqid_lo_SHIFT: c_int = 0;
pub const lpfc_eqcq_doorbell_cqid_lo_MASK: c_uint = 0x03FF;

pub const lpfc_eqcq_doorbell_cqid_hi_SHIFT: c_int = 11;
pub const lpfc_eqcq_doorbell_cqid_hi_MASK: c_uint = 0x001F;

pub const lpfc_eqcq_doorbell_eqid_lo_SHIFT: c_int = 0;
pub const lpfc_eqcq_doorbell_eqid_lo_MASK: c_uint = 0x01FF;

pub const lpfc_eqcq_doorbell_eqid_hi_SHIFT: c_int = 11;
pub const lpfc_eqcq_doorbell_eqid_hi_MASK: c_uint = 0x001F;

pub const LPFC_CQID_HI_FIELD_SHIFT: c_int = 10;
pub const LPFC_EQID_HI_FIELD_SHIFT: c_int = 9;
pub const LPFC_IF6_CQ_DOORBELL: c_uint = 0x00C0;
pub const lpfc_if6_cq_doorbell_se_SHIFT: c_int = 31;
pub const lpfc_if6_cq_doorbell_se_MASK: c_uint = 0x0001;

pub const LPFC_IF6_CQ_SOLICIT_ENABLE_OFF: c_int = 0;
pub const LPFC_IF6_CQ_SOLICIT_ENABLE_ON: c_int = 1;
pub const lpfc_if6_cq_doorbell_arm_SHIFT: c_int = 29;
pub const lpfc_if6_cq_doorbell_arm_MASK: c_uint = 0x0001;

pub const lpfc_if6_cq_doorbell_num_released_SHIFT: c_int = 16;
pub const lpfc_if6_cq_doorbell_num_released_MASK: c_uint = 0x1FFF;

pub const lpfc_if6_cq_doorbell_cqid_SHIFT: c_int = 0;
pub const lpfc_if6_cq_doorbell_cqid_MASK: c_uint = 0xFFFF;

pub const LPFC_IF6_EQ_DOORBELL: c_uint = 0x0120;
pub const lpfc_if6_eq_doorbell_io_SHIFT: c_int = 31;
pub const lpfc_if6_eq_doorbell_io_MASK: c_uint = 0x0001;

pub const LPFC_IF6_EQ_INTR_OVERRIDE_OFF: c_int = 0;
pub const LPFC_IF6_EQ_INTR_OVERRIDE_ON: c_int = 1;
pub const lpfc_if6_eq_doorbell_arm_SHIFT: c_int = 29;
pub const lpfc_if6_eq_doorbell_arm_MASK: c_uint = 0x0001;

pub const lpfc_if6_eq_doorbell_num_released_SHIFT: c_int = 16;
pub const lpfc_if6_eq_doorbell_num_released_MASK: c_uint = 0x1FFF;

pub const lpfc_if6_eq_doorbell_eqid_SHIFT: c_int = 0;
pub const lpfc_if6_eq_doorbell_eqid_MASK: c_uint = 0x0FFF;

pub const LPFC_BMBX: c_uint = 0x0160;
pub const lpfc_bmbx_addr_SHIFT: c_int = 2;
pub const lpfc_bmbx_addr_MASK: c_uint = 0x3FFFFFFF;

pub const lpfc_bmbx_hi_SHIFT: c_int = 1;
pub const lpfc_bmbx_hi_MASK: c_uint = 0x0001;

pub const lpfc_bmbx_rdy_SHIFT: c_int = 0;
pub const lpfc_bmbx_rdy_MASK: c_uint = 0x0001;

pub const LPFC_MQ_DOORBELL: c_uint = 0x0140;
pub const LPFC_IF6_MQ_DOORBELL: c_uint = 0x0160;
pub const lpfc_mq_doorbell_num_posted_SHIFT: c_int = 16;
pub const lpfc_mq_doorbell_num_posted_MASK: c_uint = 0x3FFF;

pub const lpfc_mq_doorbell_id_SHIFT: c_int = 0;
pub const lpfc_mq_doorbell_id_MASK: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_cfg_mhdr {
    pub word1: u32,
pub const lpfc_mbox_hdr_emb_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_emb_MASK: c_uint = 0x00000001;

pub const lpfc_mbox_hdr_sge_cnt_SHIFT: c_int = 3;
pub const lpfc_mbox_hdr_sge_cnt_MASK: c_uint = 0x0000001F;

    pub payload_length: u32,
    pub tag_lo: u32,
    pub tag_hi: u32,
    pub reserved5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lpfc_sli4_cfg_shdr {
    pub word6: u32,
pub const lpfc_mbox_hdr_opcode_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_opcode_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_subsystem_SHIFT: c_int = 8;
pub const lpfc_mbox_hdr_subsystem_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_port_number_SHIFT: c_int = 16;
pub const lpfc_mbox_hdr_port_number_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_domain_SHIFT: c_int = 24;
pub const lpfc_mbox_hdr_domain_MASK: c_uint = 0x000000FF;

    pub timeout: u32,
    pub request_length: u32,
    pub word9: u32,
pub const lpfc_mbox_hdr_version_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_version_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_pf_num_SHIFT: c_int = 16;
pub const lpfc_mbox_hdr_pf_num_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_vh_num_SHIFT: c_int = 24;
pub const lpfc_mbox_hdr_vh_num_MASK: c_uint = 0x000000FF;

pub const LPFC_Q_CREATE_VERSION_2: c_int = 2;
pub const LPFC_Q_CREATE_VERSION_1: c_int = 1;
pub const LPFC_Q_CREATE_VERSION_0: c_int = 0;
pub const LPFC_OPCODE_VERSION_0: c_int = 0;
pub const LPFC_OPCODE_VERSION_1: c_int = 1;
    pub request: },
    pub word6: u32,
pub const lpfc_mbox_hdr_opcode_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_opcode_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_subsystem_SHIFT: c_int = 8;
pub const lpfc_mbox_hdr_subsystem_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_domain_SHIFT: c_int = 24;
pub const lpfc_mbox_hdr_domain_MASK: c_uint = 0x000000FF;

    pub word7: u32,
pub const lpfc_mbox_hdr_status_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_status_MASK: c_uint = 0x000000FF;

pub const lpfc_mbox_hdr_add_status_SHIFT: c_int = 8;
pub const lpfc_mbox_hdr_add_status_MASK: c_uint = 0x000000FF;

pub const LPFC_ADD_STATUS_INCOMPAT_OBJ: c_uint = 0xA2;
pub const lpfc_mbox_hdr_add_status_2_SHIFT: c_int = 16;
pub const lpfc_mbox_hdr_add_status_2_MASK: c_uint = 0x000000FF;

pub const LPFC_ADD_STATUS_2_INCOMPAT_FLASH: c_uint = 0x01;
pub const LPFC_ADD_STATUS_2_INCORRECT_ASIC: c_uint = 0x02;
    pub response_length: u32,
    pub actual_response_length: u32,
    pub response: },
}

// Mailbox Header structures.
// struct mbox_header is defined for first generation SLI4_CFG mailbox
// calls deployed for BE-based ports.
//
// struct sli4_mbox_header is defined for second generation SLI4
// ports that don't deploy the SLI4_CFG mechanism.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_header {
    pub cfg_mhdr: lpfc_sli4_cfg_mhdr,
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
}

pub const LPFC_EXTENT_LOCAL: c_int = 0;
pub const LPFC_TIMEOUT_DEFAULT: c_int = 0;
pub const LPFC_EXTENT_VERSION_DEFAULT: c_int = 0;
// Subsystem Definitions
pub const LPFC_MBOX_SUBSYSTEM_NA: c_uint = 0x0;
pub const LPFC_MBOX_SUBSYSTEM_COMMON: c_uint = 0x1;
pub const LPFC_MBOX_SUBSYSTEM_LOWLEVEL: c_uint = 0xB;
pub const LPFC_MBOX_SUBSYSTEM_FCOE: c_uint = 0xC;
// Device Specific Definitions
// The HOST ENDIAN defines are in Big Endian format.
pub const HOST_ENDIAN_LOW_WORD0: c_uint = 0xFF3412FF;
pub const HOST_ENDIAN_HIGH_WORD1: c_uint = 0xFF7856FF;
// Common Opcodes
pub const LPFC_MBOX_OPCODE_NA: c_uint = 0x00;
pub const LPFC_MBOX_OPCODE_CQ_CREATE: c_uint = 0x0C;
pub const LPFC_MBOX_OPCODE_EQ_CREATE: c_uint = 0x0D;
pub const LPFC_MBOX_OPCODE_MQ_CREATE: c_uint = 0x15;
pub const LPFC_MBOX_OPCODE_GET_CNTL_ATTRIBUTES: c_uint = 0x20;
pub const LPFC_MBOX_OPCODE_NOP: c_uint = 0x21;
pub const LPFC_MBOX_OPCODE_MODIFY_EQ_DELAY: c_uint = 0x29;
pub const LPFC_MBOX_OPCODE_MQ_DESTROY: c_uint = 0x35;
pub const LPFC_MBOX_OPCODE_CQ_DESTROY: c_uint = 0x36;
pub const LPFC_MBOX_OPCODE_EQ_DESTROY: c_uint = 0x37;
pub const LPFC_MBOX_OPCODE_QUERY_FW_CFG: c_uint = 0x3A;
pub const LPFC_MBOX_OPCODE_FUNCTION_RESET: c_uint = 0x3D;
pub const LPFC_MBOX_OPCODE_SET_PHYSICAL_LINK_CONFIG: c_uint = 0x3E;
pub const LPFC_MBOX_OPCODE_SET_BOOT_CONFIG: c_uint = 0x43;
pub const LPFC_MBOX_OPCODE_SET_BEACON_CONFIG: c_uint = 0x45;
pub const LPFC_MBOX_OPCODE_GET_BEACON_CONFIG: c_uint = 0x46;
pub const LPFC_MBOX_OPCODE_GET_PORT_NAME: c_uint = 0x4D;
pub const LPFC_MBOX_OPCODE_MQ_CREATE_EXT: c_uint = 0x5A;
pub const LPFC_MBOX_OPCODE_GET_VPD_DATA: c_uint = 0x5B;
pub const LPFC_MBOX_OPCODE_SET_HOST_DATA: c_uint = 0x5D;
pub const LPFC_MBOX_OPCODE_SEND_ACTIVATION: c_uint = 0x73;
pub const LPFC_MBOX_OPCODE_RESET_LICENSES: c_uint = 0x74;
pub const LPFC_MBOX_OPCODE_REG_CONGESTION_BUF: c_uint = 0x8E;
pub const LPFC_MBOX_OPCODE_GET_RSRC_EXTENT_INFO: c_uint = 0x9A;
pub const LPFC_MBOX_OPCODE_GET_ALLOC_RSRC_EXTENT: c_uint = 0x9B;
pub const LPFC_MBOX_OPCODE_ALLOC_RSRC_EXTENT: c_uint = 0x9C;
pub const LPFC_MBOX_OPCODE_DEALLOC_RSRC_EXTENT: c_uint = 0x9D;
pub const LPFC_MBOX_OPCODE_GET_FUNCTION_CONFIG: c_uint = 0xA0;
pub const LPFC_MBOX_OPCODE_GET_PROFILE_CAPACITIES: c_uint = 0xA1;
pub const LPFC_MBOX_OPCODE_GET_PROFILE_CONFIG: c_uint = 0xA4;
pub const LPFC_MBOX_OPCODE_SET_PROFILE_CONFIG: c_uint = 0xA5;
pub const LPFC_MBOX_OPCODE_GET_PROFILE_LIST: c_uint = 0xA6;
pub const LPFC_MBOX_OPCODE_SET_ACT_PROFILE: c_uint = 0xA8;
pub const LPFC_MBOX_OPCODE_GET_FACTORY_PROFILE_CONFIG: c_uint = 0xA9;
pub const LPFC_MBOX_OPCODE_READ_OBJECT: c_uint = 0xAB;
pub const LPFC_MBOX_OPCODE_WRITE_OBJECT: c_uint = 0xAC;
pub const LPFC_MBOX_OPCODE_READ_OBJECT_LIST: c_uint = 0xAD;
pub const LPFC_MBOX_OPCODE_DELETE_OBJECT: c_uint = 0xAE;
pub const LPFC_MBOX_OPCODE_GET_SLI4_PARAMETERS: c_uint = 0xB5;
pub const LPFC_MBOX_OPCODE_SET_FEATURES: c_uint = 0xBF;
// FCoE Opcodes
pub const LPFC_MBOX_OPCODE_FCOE_WQ_CREATE: c_uint = 0x01;
pub const LPFC_MBOX_OPCODE_FCOE_WQ_DESTROY: c_uint = 0x02;
pub const LPFC_MBOX_OPCODE_FCOE_POST_SGL_PAGES: c_uint = 0x03;
pub const LPFC_MBOX_OPCODE_FCOE_REMOVE_SGL_PAGES: c_uint = 0x04;
pub const LPFC_MBOX_OPCODE_FCOE_RQ_CREATE: c_uint = 0x05;
pub const LPFC_MBOX_OPCODE_FCOE_RQ_DESTROY: c_uint = 0x06;
pub const LPFC_MBOX_OPCODE_FCOE_READ_FCF_TABLE: c_uint = 0x08;
pub const LPFC_MBOX_OPCODE_FCOE_ADD_FCF: c_uint = 0x09;
pub const LPFC_MBOX_OPCODE_FCOE_DELETE_FCF: c_uint = 0x0A;
pub const LPFC_MBOX_OPCODE_FCOE_POST_HDR_TEMPLATE: c_uint = 0x0B;
pub const LPFC_MBOX_OPCODE_FCOE_REDISCOVER_FCF: c_uint = 0x10;
pub const LPFC_MBOX_OPCODE_FCOE_CQ_CREATE_SET: c_uint = 0x1D;
pub const LPFC_MBOX_OPCODE_FCOE_SET_FCLINK_SETTINGS: c_uint = 0x21;
pub const LPFC_MBOX_OPCODE_FCOE_LINK_DIAG_STATE: c_uint = 0x22;
pub const LPFC_MBOX_OPCODE_FCOE_LINK_DIAG_LOOPBACK: c_uint = 0x23;
pub const LPFC_MBOX_OPCODE_FCOE_FC_SET_TRUNK_MODE: c_uint = 0x42;
// Low level Opcodes
pub const LPFC_MBOX_OPCODE_SET_DIAG_LOG_OPTION: c_uint = 0x37;
// Mailbox command structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_context {
    pub word0: u32,
pub const lpfc_eq_context_size_SHIFT: c_int = 31;
pub const lpfc_eq_context_size_MASK: c_uint = 0x00000001;

pub const LPFC_EQE_SIZE_4: c_uint = 0x0;
pub const LPFC_EQE_SIZE_16: c_uint = 0x1;
pub const lpfc_eq_context_valid_SHIFT: c_int = 29;
pub const lpfc_eq_context_valid_MASK: c_uint = 0x00000001;

pub const lpfc_eq_context_autovalid_SHIFT: c_int = 28;
pub const lpfc_eq_context_autovalid_MASK: c_uint = 0x00000001;

    pub word1: u32,
pub const lpfc_eq_context_count_SHIFT: c_int = 26;
pub const lpfc_eq_context_count_MASK: c_uint = 0x00000003;

pub const LPFC_EQ_CNT_256: c_uint = 0x0;
pub const LPFC_EQ_CNT_512: c_uint = 0x1;
pub const LPFC_EQ_CNT_1024: c_uint = 0x2;
pub const LPFC_EQ_CNT_2048: c_uint = 0x3;
pub const LPFC_EQ_CNT_4096: c_uint = 0x4;
    pub word2: u32,
pub const lpfc_eq_context_delay_multi_SHIFT: c_int = 13;
pub const lpfc_eq_context_delay_multi_MASK: c_uint = 0x000003FF;

    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_delay_info {
    pub eq_id: u32,
    pub phase: u32,
    pub delay_multi: u32,
}

pub const LPFC_MAX_EQ_DELAY_EQID_CNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgl_page_pairs {
    pub sgl_pg0_addr_lo: u32,
    pub sgl_pg0_addr_hi: u32,
    pub sgl_pg1_addr_lo: u32,
    pub sgl_pg1_addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_post_sgl_pages {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_post_sgl_pages_xri_SHIFT: c_int = 0;
pub const lpfc_post_sgl_pages_xri_MASK: c_uint = 0x0000FFFF;

pub const lpfc_post_sgl_pages_xricnt_SHIFT: c_int = 16;
pub const lpfc_post_sgl_pages_xricnt_MASK: c_uint = 0x0000FFFF;
    pub sgl_pg_pairs: [sgl_page_pairs; 1],
}

// word0 of page-1 struct shares the same SHIFT/MASK/WORD defines as above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_post_uembed_sgl_page1 {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word0: u32,
    pub sgl_pg_pairs: sgl_page_pairs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_sge {
    pub pa_lo: u32,
    pub pa_hi: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_host_buf {
    pub length: u32,
    pub pa_lo: u32,
    pub pa_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_nembed_cmd {
    pub cfg_mhdr: lpfc_sli4_cfg_mhdr,
pub const LPFC_SLI4_MBX_SGE_MAX_PAGES: c_int = 19;
    pub sge: [lpfc_mbx_sge; LPFC_SLI4_MBX_SGE_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_nembed_sge_virt {
    pub addr: [*mut c_void; LPFC_SLI4_MBX_SGE_MAX_PAGES],
}

pub const LPFC_MBX_OBJECT_NAME_LEN_DW: c_int = 26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_object {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_rd_object_rlen_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_object_rlen_MASK: c_uint = 0x00FFFFFF;

    pub rd_object_offset: u32,
    pub rd_object_name: [__le32; LPFC_MBX_OBJECT_NAME_LEN_DW],
    pub rd_object_cnt: u32,
    pub rd_object_hbuf: [lpfc_mbx_host_buf; 4],
    pub request: },
    pub rd_object_actual_rlen: u32,
    pub word1: u32,
pub const lpfc_mbx_rd_object_eof_SHIFT: c_int = 31;
pub const lpfc_mbx_rd_object_eof_MASK: c_uint = 0x1;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_eq_create {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_eq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_eq_create_num_pages_MASK: c_uint = 0x0000FFFF;

    pub context: eq_context,
    pub page: [dma_address; LPFC_MAX_EQ_PAGE],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_eq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_eq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_modify_eq_delay {
    pub header: mbox_header,
    pub num_eq: u32,
    pub eq: [eq_delay_info; LPFC_MAX_EQ_DELAY_EQID_CNT],
    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_eq_destroy {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_eq_destroy_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_eq_destroy_q_id_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_nop {
    pub header: mbox_header,
    pub context: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_ras_fwlog {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_fwlog_enable_SHIFT: c_int = 0;
pub const lpfc_fwlog_enable_MASK: c_uint = 0x00000001;

pub const lpfc_fwlog_loglvl_SHIFT: c_int = 8;
pub const lpfc_fwlog_loglvl_MASK: c_uint = 0x0000000F;

pub const lpfc_fwlog_ra_SHIFT: c_int = 15;
pub const lpfc_fwlog_ra_WORD: c_uint = 0x00000008;
pub const lpfc_fwlog_buffcnt_SHIFT: c_int = 16;
pub const lpfc_fwlog_buffcnt_MASK: c_uint = 0x000000FF;

pub const lpfc_fwlog_buffsz_SHIFT: c_int = 24;
pub const lpfc_fwlog_buffsz_MASK: c_uint = 0x000000FF;

    pub word5: u32,
pub const lpfc_fwlog_acqe_SHIFT: c_int = 0;
pub const lpfc_fwlog_acqe_MASK: c_uint = 0x0000FFFF;

pub const lpfc_fwlog_cqid_SHIFT: c_int = 16;
pub const lpfc_fwlog_cqid_MASK: c_uint = 0x0000FFFF;

pub const LPFC_MAX_FWLOG_PAGE: c_int = 16;
    pub lwpd: dma_address,
    pub buff_fwlog: [dma_address; LPFC_MAX_FWLOG_PAGE],
    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_context {
    pub word0: u32,
pub const lpfc_cq_context_event_SHIFT: c_int = 31;
pub const lpfc_cq_context_event_MASK: c_uint = 0x00000001;

pub const lpfc_cq_context_valid_SHIFT: c_int = 29;
pub const lpfc_cq_context_valid_MASK: c_uint = 0x00000001;

pub const lpfc_cq_context_count_SHIFT: c_int = 27;
pub const lpfc_cq_context_count_MASK: c_uint = 0x00000003;

pub const LPFC_CQ_CNT_256: c_uint = 0x0;
pub const LPFC_CQ_CNT_512: c_uint = 0x1;
pub const LPFC_CQ_CNT_1024: c_uint = 0x2;
pub const LPFC_CQ_CNT_WORD7: c_uint = 0x3;
pub const lpfc_cq_context_cqe_sz_SHIFT: c_int = 25;
pub const lpfc_cq_context_cqe_sz_MASK: c_uint = 0x00000003;

pub const lpfc_cq_context_autovalid_SHIFT: c_int = 15;
pub const lpfc_cq_context_autovalid_MASK: c_uint = 0x00000001;

    pub word1: u32,

pub const lpfc_cq_eq_id_MASK: c_uint = 0x000000FF;

pub const lpfc_cq_eq_id_2_MASK: c_uint = 0x0000FFFF;

    pub /: *mut *mut uint32_t lpfc_cq_context_count; / Version 2 Only,
    pub reserved1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_cq_create {
    pub header: mbox_header,
    pub word0: u32,

pub const lpfc_mbx_cq_create_page_size_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_cq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_num_pages_MASK: c_uint = 0x0000FFFF;

    pub context: cq_context,
    pub page: [dma_address; LPFC_MAX_CQ_PAGE],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_cq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_cq_create_set {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word0: u32,

pub const lpfc_mbx_cq_create_set_page_size_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_cq_create_set_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_num_pages_MASK: c_uint = 0x0000FFFF;

    pub word1: u32,
pub const lpfc_mbx_cq_create_set_evt_SHIFT: c_int = 31;
pub const lpfc_mbx_cq_create_set_evt_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_cq_create_set_valid_SHIFT: c_int = 29;
pub const lpfc_mbx_cq_create_set_valid_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_cq_create_set_cqecnt_SHIFT: c_int = 27;
pub const lpfc_mbx_cq_create_set_cqecnt_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_cq_create_set_cqe_size_SHIFT: c_int = 25;
pub const lpfc_mbx_cq_create_set_cqe_size_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_cq_create_set_autovalid_SHIFT: c_int = 15;
pub const lpfc_mbx_cq_create_set_autovalid_MASK: c_uint = 0x0000001;

pub const lpfc_mbx_cq_create_set_nodelay_SHIFT: c_int = 14;
pub const lpfc_mbx_cq_create_set_nodelay_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_cq_create_set_clswm_SHIFT: c_int = 12;
pub const lpfc_mbx_cq_create_set_clswm_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_cq_create_set_cqe_cnt_hi_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_cqe_cnt_hi_MASK: c_uint = 0x0000001F;

    pub word2: u32,
pub const lpfc_mbx_cq_create_set_arm_SHIFT: c_int = 31;
pub const lpfc_mbx_cq_create_set_arm_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_cq_create_set_cqe_cnt_lo_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_cqe_cnt_lo_MASK: c_uint = 0x00007FFF;

pub const lpfc_mbx_cq_create_set_num_cq_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_num_cq_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,
pub const lpfc_mbx_cq_create_set_eq_id1_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id1_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id0_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id0_MASK: c_uint = 0x0000FFFF;

    pub word4: u32,
pub const lpfc_mbx_cq_create_set_eq_id3_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id3_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id2_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id2_MASK: c_uint = 0x0000FFFF;

    pub word5: u32,
pub const lpfc_mbx_cq_create_set_eq_id5_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id5_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id4_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id4_MASK: c_uint = 0x0000FFFF;

    pub word6: u32,
pub const lpfc_mbx_cq_create_set_eq_id7_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id7_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id6_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id6_MASK: c_uint = 0x0000FFFF;

    pub word7: u32,
pub const lpfc_mbx_cq_create_set_eq_id9_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id9_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id8_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id8_MASK: c_uint = 0x0000FFFF;

    pub word8: u32,
pub const lpfc_mbx_cq_create_set_eq_id11_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id11_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id10_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id10_MASK: c_uint = 0x0000FFFF;

    pub word9: u32,
pub const lpfc_mbx_cq_create_set_eq_id13_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id13_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id12_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id12_MASK: c_uint = 0x0000FFFF;

    pub word10: u32,
pub const lpfc_mbx_cq_create_set_eq_id15_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_eq_id15_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_eq_id14_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_eq_id14_MASK: c_uint = 0x0000FFFF;
    pub page: [dma_address; 1],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_cq_create_set_num_alloc_SHIFT: c_int = 16;
pub const lpfc_mbx_cq_create_set_num_alloc_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_cq_create_set_base_id_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_create_set_base_id_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_cq_destroy {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_cq_destroy_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_cq_destroy_q_id_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wq_context {
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_wq_create {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_wq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_num_pages_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_wq_create_dua_SHIFT: c_int = 8;
pub const lpfc_mbx_wq_create_dua_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_cq_id_SHIFT: c_int = 16;
pub const lpfc_mbx_wq_create_cq_id_MASK: c_uint = 0x0000FFFF;
    pub page: [dma_address; LPFC_MAX_WQ_PAGE_V0],
    pub word9: u32,
pub const lpfc_mbx_wq_create_bua_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_bua_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_ulp_num_SHIFT: c_int = 8;
pub const lpfc_mbx_wq_create_ulp_num_MASK: c_uint = 0x000000FF;

    pub request: },
    pub /: *mut *mut uint32_t word0; / Word 0 is the same as in v0,
    pub word1: u32,
pub const lpfc_mbx_wq_create_page_size_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_page_size_MASK: c_uint = 0x000000FF;

pub const LPFC_WQ_PAGE_SIZE_4096: c_uint = 0x1;
pub const lpfc_mbx_wq_create_dpp_req_SHIFT: c_int = 15;
pub const lpfc_mbx_wq_create_dpp_req_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_doe_SHIFT: c_int = 14;
pub const lpfc_mbx_wq_create_doe_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_toe_SHIFT: c_int = 13;
pub const lpfc_mbx_wq_create_toe_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_wqe_size_SHIFT: c_int = 8;
pub const lpfc_mbx_wq_create_wqe_size_MASK: c_uint = 0x0000000F;

pub const LPFC_WQ_WQE_SIZE_64: c_uint = 0x5;
pub const LPFC_WQ_WQE_SIZE_128: c_uint = 0x6;
pub const lpfc_mbx_wq_create_wqe_count_SHIFT: c_int = 16;
pub const lpfc_mbx_wq_create_wqe_count_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
    pub page: [dma_address; LPFC_MAX_WQ_PAGE-1],
    pub request_1: },
    pub word0: u32,
pub const lpfc_mbx_wq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub doorbell_offset: u32,
    pub word2: u32,
pub const lpfc_mbx_wq_create_bar_set_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_bar_set_MASK: c_uint = 0x0000FFFF;

pub const WQ_PCI_BAR_0_AND_1: c_uint = 0x00;
pub const WQ_PCI_BAR_2_AND_3: c_uint = 0x01;
pub const WQ_PCI_BAR_4_AND_5: c_uint = 0x02;
pub const lpfc_mbx_wq_create_db_format_SHIFT: c_int = 16;
pub const lpfc_mbx_wq_create_db_format_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub word0: u32,
pub const lpfc_mbx_wq_create_dpp_rsp_SHIFT: c_int = 31;
pub const lpfc_mbx_wq_create_dpp_rsp_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_wq_create_v1_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_v1_q_id_MASK: c_uint = 0x0000FFFF;

    pub word1: u32,
pub const lpfc_mbx_wq_create_v1_bar_set_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_v1_bar_set_MASK: c_uint = 0x0000000F;

    pub doorbell_offset: u32,
    pub word3: u32,
pub const lpfc_mbx_wq_create_dpp_id_SHIFT: c_int = 16;
pub const lpfc_mbx_wq_create_dpp_id_MASK: c_uint = 0x0000001F;

pub const lpfc_mbx_wq_create_dpp_bar_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_create_dpp_bar_MASK: c_uint = 0x0000000F;

    pub dpp_offset: u32,
    pub response_1: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_wq_destroy {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_wq_destroy_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_wq_destroy_q_id_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

pub const LPFC_HDR_BUF_SIZE: c_int = 128;
pub const LPFC_DATA_BUF_SIZE: c_int = 2048;
pub const LPFC_NVMET_DATA_BUF_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_context {
    pub word0: u32,

pub const lpfc_rq_context_rqe_count_MASK: c_uint = 0x0000000F;

pub const lpfc_rq_context_rqe_count_1_MASK: c_uint = 0x0000FFFF;

pub const lpfc_rq_context_rqe_size_MASK: c_uint = 0x0000000F;

pub const LPFC_RQE_SIZE_8: c_int = 2;
pub const LPFC_RQE_SIZE_16: c_int = 3;
pub const LPFC_RQE_SIZE_32: c_int = 4;
pub const LPFC_RQE_SIZE_64: c_int = 5;
pub const LPFC_RQE_SIZE_128: c_int = 6;

pub const lpfc_rq_context_page_size_MASK: c_uint = 0x000000FF;

pub const LPFC_RQ_PAGE_SIZE_4096: c_uint = 0x1;
    pub word1: u32,

pub const lpfc_rq_context_data_size_MASK: c_uint = 0x0000FFFF;

pub const lpfc_rq_context_hdr_size_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_rq_context_cq_id_SHIFT: c_int = 16;
pub const lpfc_rq_context_cq_id_MASK: c_uint = 0x0000FFFF;

pub const lpfc_rq_context_buf_size_SHIFT: c_int = 0;
pub const lpfc_rq_context_buf_size_MASK: c_uint = 0x0000FFFF;

pub const lpfc_rq_context_base_cq_MASK: c_uint = 0x0000FFFF;

    pub /: *mut *mut uint32_t buffer_size; / Version 1 Only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_rq_create {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_rq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_num_pages_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_dua_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_dua_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_bqu_SHIFT: c_int = 17;
pub const lpfc_mbx_rq_create_bqu_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_ulp_num_SHIFT: c_int = 24;
pub const lpfc_mbx_rq_create_ulp_num_MASK: c_uint = 0x000000FF;

    pub context: rq_context,
    pub page: [dma_address; LPFC_MAX_RQ_PAGE],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_rq_create_q_cnt_v2_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_q_cnt_v2_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub doorbell_offset: u32,
    pub word2: u32,
pub const lpfc_mbx_rq_create_bar_set_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_bar_set_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_db_format_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_db_format_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_rq_create_v2 {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word0: u32,
pub const lpfc_mbx_rq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_num_pages_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_rq_cnt_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_rq_cnt_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_rq_create_dua_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_dua_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_bqu_SHIFT: c_int = 17;
pub const lpfc_mbx_rq_create_bqu_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_ulp_num_SHIFT: c_int = 24;
pub const lpfc_mbx_rq_create_ulp_num_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_rq_create_dim_SHIFT: c_int = 29;
pub const lpfc_mbx_rq_create_dim_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_dfd_SHIFT: c_int = 30;
pub const lpfc_mbx_rq_create_dfd_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_create_dnb_SHIFT: c_int = 31;
pub const lpfc_mbx_rq_create_dnb_MASK: c_uint = 0x00000001;

    pub context: rq_context,
    pub page: [dma_address; 1],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_rq_create_q_cnt_v2_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_q_cnt_v2_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub doorbell_offset: u32,
    pub word2: u32,
pub const lpfc_mbx_rq_create_bar_set_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_create_bar_set_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rq_create_db_format_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_create_db_format_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_rq_destroy {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_rq_destroy_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_destroy_q_id_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mq_context {
    pub word0: u32,

pub const lpfc_mq_context_cq_id_MASK: c_uint = 0x000003FF;

pub const lpfc_mq_context_ring_size_SHIFT: c_int = 16;
pub const lpfc_mq_context_ring_size_MASK: c_uint = 0x0000000F;

pub const LPFC_MQ_RING_SIZE_16: c_uint = 0x5;
pub const LPFC_MQ_RING_SIZE_32: c_uint = 0x6;
pub const LPFC_MQ_RING_SIZE_64: c_uint = 0x7;
pub const LPFC_MQ_RING_SIZE_128: c_uint = 0x8;
    pub word1: u32,
pub const lpfc_mq_context_valid_SHIFT: c_int = 31;
pub const lpfc_mq_context_valid_MASK: c_uint = 0x00000001;

    pub reserved2: u32,
    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_mq_create {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_mq_create_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_mq_create_num_pages_MASK: c_uint = 0x0000FFFF;

    pub context: mq_context,
    pub page: [dma_address; LPFC_MAX_MQ_PAGE],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_mq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_mq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_mq_create_ext {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_mq_create_ext_num_pages_SHIFT: c_int = 0;
pub const lpfc_mbx_mq_create_ext_num_pages_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_mq_create_ext_cq_id_MASK: c_uint = 0x0000FFFF;

    pub async_evt_bmap: u32,

pub const lpfc_mbx_mq_create_ext_async_evt_link_MASK: c_uint = 0x00000001;

pub const LPFC_EVT_CODE_LINK_NO_LINK: c_uint = 0x0;
pub const LPFC_EVT_CODE_LINK_10_MBIT: c_uint = 0x1;
pub const LPFC_EVT_CODE_LINK_100_MBIT: c_uint = 0x2;
pub const LPFC_EVT_CODE_LINK_1_GBIT: c_uint = 0x3;
pub const LPFC_EVT_CODE_LINK_10_GBIT: c_uint = 0x4;

pub const lpfc_mbx_mq_create_ext_async_evt_fip_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_mq_create_ext_async_evt_group5_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_mq_create_ext_async_evt_fc_MASK: c_uint = 0x00000001;

pub const LPFC_EVT_CODE_FC_NO_LINK: c_uint = 0x0;
pub const LPFC_EVT_CODE_FC_1_GBAUD: c_uint = 0x1;
pub const LPFC_EVT_CODE_FC_2_GBAUD: c_uint = 0x2;
pub const LPFC_EVT_CODE_FC_4_GBAUD: c_uint = 0x4;
pub const LPFC_EVT_CODE_FC_8_GBAUD: c_uint = 0x8;
pub const LPFC_EVT_CODE_FC_10_GBAUD: c_uint = 0xA;
pub const LPFC_EVT_CODE_FC_16_GBAUD: c_uint = 0x10;

pub const lpfc_mbx_mq_create_ext_async_evt_sli_MASK: c_uint = 0x00000001;

    pub context: mq_context,
    pub page: [dma_address; LPFC_MAX_MQ_PAGE],
    pub request: },
    pub word0: u32,
pub const lpfc_mbx_mq_create_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_mq_create_q_id_MASK: c_uint = 0x0000FFFF;

    pub response: },
    pub u: },
pub const LPFC_ASYNC_EVENT_LINK_STATE: c_uint = 0x2;
pub const LPFC_ASYNC_EVENT_FCF_STATE: c_uint = 0x4;
pub const LPFC_ASYNC_EVENT_GROUP5: c_uint = 0x20;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_mq_destroy {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_mq_destroy_q_id_SHIFT: c_int = 0;
pub const lpfc_mbx_mq_destroy_q_id_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub word0: u32,
    pub response: },
    pub u: },
}

// Start Gen 2 SLI4 Mailbox definitions:
// Define allocate-ready Gen 2 SLI4 FCoE Resource Extent Types.
pub const LPFC_RSC_TYPE_FCOE_VFI: c_uint = 0x20;
pub const LPFC_RSC_TYPE_FCOE_VPI: c_uint = 0x21;
pub const LPFC_RSC_TYPE_FCOE_RPI: c_uint = 0x22;
pub const LPFC_RSC_TYPE_FCOE_XRI: c_uint = 0x23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_rsrc_extent_info {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_mbx_get_rsrc_extent_info_type_SHIFT: c_int = 0;
pub const lpfc_mbx_get_rsrc_extent_info_type_MASK: c_uint = 0x0000FFFF;

    pub req: },
    pub word4: u32,
pub const lpfc_mbx_get_rsrc_extent_info_cnt_SHIFT: c_int = 0;
pub const lpfc_mbx_get_rsrc_extent_info_cnt_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_get_rsrc_extent_info_size_SHIFT: c_int = 16;
pub const lpfc_mbx_get_rsrc_extent_info_size_MASK: c_uint = 0x0000FFFF;

    pub rsp: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_query_fw_config {
    pub header: mbox_header,
    pub config_number: u32,
pub const LPFC_FC_FCOE: c_uint = 0x00000007;
    pub asic_revision: u32,
    pub physical_port: u32,
    pub function_mode: u32,
pub const LPFC_FC_INI_MODE: c_uint = 0x00000040;
pub const LPFC_FC_TGT_MODE: c_uint = 0x00000080;
pub const LPFC_DUA_MODE: c_uint = 0x00000800;
    pub oper_mode: u32,
    pub rsvd9: [u32; 2],
    pub wqid_base: u32,
    pub wqid_tot: u32,
    pub rqid_base: u32,
    pub rqid_tot: u32,
    pub rsvd15: [u32; 19],
    pub function_capabilities: u32,
    pub cqid_base: u32,
    pub cqid_tot: u32,
    pub eqid_base: u32,
    pub eqid_tot: u32,
    pub rsvd39: [u32; 4],
    pub rsp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_beacon_config {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_mbx_set_beacon_port_num_SHIFT: c_int = 0;
pub const lpfc_mbx_set_beacon_port_num_MASK: c_uint = 0x0000003F;

pub const lpfc_mbx_set_beacon_port_type_SHIFT: c_int = 6;
pub const lpfc_mbx_set_beacon_port_type_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_set_beacon_state_SHIFT: c_int = 8;
pub const lpfc_mbx_set_beacon_state_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_set_beacon_duration_SHIFT: c_int = 16;
pub const lpfc_mbx_set_beacon_duration_MASK: c_uint = 0x000000FF;

// COMMON_SET_BEACON_CONFIG_V1
pub const lpfc_mbx_set_beacon_duration_v1_SHIFT: c_int = 16;
pub const lpfc_mbx_set_beacon_duration_v1_MASK: c_uint = 0x0000FFFF;

    pub /: *mut *mut uint32_t word5; / RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_id_range {
    pub word5: u32,
pub const lpfc_mbx_rsrc_id_word4_0_SHIFT: c_int = 0;
pub const lpfc_mbx_rsrc_id_word4_0_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rsrc_id_word4_1_SHIFT: c_int = 16;
pub const lpfc_mbx_rsrc_id_word4_1_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_link_diag_state {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_set_diag_state_diag_SHIFT: c_int = 0;
pub const lpfc_mbx_set_diag_state_diag_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_diag_state_diag_bit_valid_SHIFT: c_int = 2;
pub const lpfc_mbx_set_diag_state_diag_bit_valid_MASK: c_uint = 0x00000001;

pub const LPFC_DIAG_STATE_DIAG_BIT_VALID_NO_CHANGE: c_int = 0;
pub const LPFC_DIAG_STATE_DIAG_BIT_VALID_CHANGE: c_int = 1;
pub const lpfc_mbx_set_diag_state_link_num_SHIFT: c_int = 16;
pub const lpfc_mbx_set_diag_state_link_num_MASK: c_uint = 0x0000003F;

pub const lpfc_mbx_set_diag_state_link_type_SHIFT: c_int = 22;
pub const lpfc_mbx_set_diag_state_link_type_MASK: c_uint = 0x00000003;

    pub req: },
    pub word0: u32,
    pub rsp: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_link_diag_loopback {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_set_diag_lpbk_type_SHIFT: c_int = 0;
pub const lpfc_mbx_set_diag_lpbk_type_MASK: c_uint = 0x00000003;

pub const LPFC_DIAG_LOOPBACK_TYPE_DISABLE: c_uint = 0x0;
pub const LPFC_DIAG_LOOPBACK_TYPE_INTERNAL: c_uint = 0x1;
pub const LPFC_DIAG_LOOPBACK_TYPE_SERDES: c_uint = 0x2;
pub const LPFC_DIAG_LOOPBACK_TYPE_EXTERNAL_TRUNKED: c_uint = 0x3;
pub const lpfc_mbx_set_diag_lpbk_link_num_SHIFT: c_int = 16;
pub const lpfc_mbx_set_diag_lpbk_link_num_MASK: c_uint = 0x0000003F;

pub const lpfc_mbx_set_diag_lpbk_link_type_SHIFT: c_int = 22;
pub const lpfc_mbx_set_diag_lpbk_link_type_MASK: c_uint = 0x00000003;

    pub req: },
    pub word0: u32,
    pub rsp: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_run_link_diag_test {
    pub header: mbox_header,
    pub word0: u32,
pub const lpfc_mbx_run_diag_test_link_num_SHIFT: c_int = 16;
pub const lpfc_mbx_run_diag_test_link_num_MASK: c_uint = 0x0000003F;

pub const lpfc_mbx_run_diag_test_link_type_SHIFT: c_int = 22;
pub const lpfc_mbx_run_diag_test_link_type_MASK: c_uint = 0x00000003;

    pub word1: u32,
pub const lpfc_mbx_run_diag_test_test_id_SHIFT: c_int = 0;
pub const lpfc_mbx_run_diag_test_test_id_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_run_diag_test_loops_SHIFT: c_int = 16;
pub const lpfc_mbx_run_diag_test_loops_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_mbx_run_diag_test_test_ver_SHIFT: c_int = 0;
pub const lpfc_mbx_run_diag_test_test_ver_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_run_diag_test_err_act_SHIFT: c_int = 16;
pub const lpfc_mbx_run_diag_test_err_act_MASK: c_uint = 0x000000FF;

    pub req: },
    pub word0: u32,
    pub rsp: },
    pub u: },
}

//
// struct lpfc_mbx_alloc_rsrc_extents:
// A mbox is generically 256 bytes long. An SLI4_CONFIG mailbox requires
// 6 words of header + 4 words of shared subcommand header +
// 1 words of Extent-Opcode-specific header = 11 words or 44 bytes total.
//
// An embedded version of SLI4_CONFIG therefore has 256 - 44 = 212 bytes
// for extents payload.
//
// 212/2 (bytes per extent) = 106 extents.
// 106/2 (extents per word) = 53 words.
// lpfc_id_range id is statically size to 53.
//
// This mailbox definition is used for ALLOC or GET_ALLOCATED
// extent ranges.  For ALLOC, the type and cnt are required.
// For GET_ALLOCATED, only the type is required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_alloc_rsrc_extents {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_mbx_alloc_rsrc_extents_type_SHIFT: c_int = 0;
pub const lpfc_mbx_alloc_rsrc_extents_type_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_alloc_rsrc_extents_cnt_SHIFT: c_int = 16;
pub const lpfc_mbx_alloc_rsrc_extents_cnt_MASK: c_uint = 0x0000FFFF;

    pub req: },
    pub word4: u32,
pub const lpfc_mbx_rsrc_cnt_SHIFT: c_int = 0;
pub const lpfc_mbx_rsrc_cnt_MASK: c_uint = 0x0000FFFF;
    pub id: [lpfc_id_range; 53],
    pub rsp: },
    pub u: },
}

//
// This is the non-embedded version of ALLOC or GET RSRC_EXTENTS. Word4 in this
// structure shares the same SHIFT/MASK/WORD defines provided in the
// mbx_alloc_rsrc_extents and mbx_get_alloc_rsrc_extents, word4, provided in
// the structures defined above.  This non-embedded structure provides for the
// maximum number of extents supported by the port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_nembed_rsrc_extent {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word4: u32,
    pub id: lpfc_id_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_dealloc_rsrc_extents {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_mbx_dealloc_rsrc_extents_type_SHIFT: c_int = 0;
pub const lpfc_mbx_dealloc_rsrc_extents_type_MASK: c_uint = 0x0000FFFF;

    pub req: },
}

// Start SLI4 FCoE specific mbox structures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_post_hdr_tmpl {
    pub header: mbox_header,
    pub word10: u32,
pub const lpfc_mbx_post_hdr_tmpl_rpi_offset_SHIFT: c_int = 0;
pub const lpfc_mbx_post_hdr_tmpl_rpi_offset_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_post_hdr_tmpl_page_cnt_SHIFT: c_int = 16;
pub const lpfc_mbx_post_hdr_tmpl_page_cnt_MASK: c_uint = 0x0000FFFF;

    pub rpi_paddr_lo: u32,
    pub rpi_paddr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_sge {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub word2: u32,
pub const lpfc_sli4_sge_offset_SHIFT: c_int = 0;
pub const lpfc_sli4_sge_offset_MASK: c_uint = 0x07FFFFFF;

pub const lpfc_sli4_sge_type_SHIFT: c_int = 27;
pub const lpfc_sli4_sge_type_MASK: c_uint = 0x0000000F;

pub const LPFC_SGE_TYPE_DATA: c_uint = 0x0;
pub const LPFC_SGE_TYPE_DIF: c_uint = 0x4;
pub const LPFC_SGE_TYPE_LSP: c_uint = 0x5;
pub const LPFC_SGE_TYPE_PEDIF: c_uint = 0x6;
pub const LPFC_SGE_TYPE_PESEED: c_uint = 0x7;
pub const LPFC_SGE_TYPE_DISEED: c_uint = 0x8;
pub const LPFC_SGE_TYPE_ENC: c_uint = 0x9;
pub const LPFC_SGE_TYPE_ATM: c_uint = 0xA;
pub const LPFC_SGE_TYPE_SKIP: c_uint = 0xC;

pub const lpfc_sli4_sge_last_MASK: c_uint = 0x00000001;

    pub sge_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_sge_le {
    pub addr_hi: __le32,
    pub addr_lo: __le32,
    pub word2: __le32,
    pub sge_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_hybrid_sgl {
    pub list_node: list_head,
    pub dma_sgl: *mut sli4_sge,
    pub dma_phys_sgl: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmd_rsp_buf {
    pub list_node: list_head,
// for storing cmd/rsp dma alloc'ed virt_addr
    pub fcp_cmnd: *mut fcp_cmnd,
    pub fcp_rsp: *mut fcp_rsp,
// for storing this cmd/rsp's dma mapped phys addr from per CPU pool
    pub fcp_cmd_rsp_dma_handle: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_sge_diseed {
    pub ref_tag: u32,
    pub ref_tag_tran: u32,
    pub word2: u32,
pub const lpfc_sli4_sge_dif_apptran_SHIFT: c_int = 0;
pub const lpfc_sli4_sge_dif_apptran_MASK: c_uint = 0x0000FFFF;

pub const lpfc_sli4_sge_dif_af_SHIFT: c_int = 24;
pub const lpfc_sli4_sge_dif_af_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_na_SHIFT: c_int = 25;
pub const lpfc_sli4_sge_dif_na_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_hi_SHIFT: c_int = 26;
pub const lpfc_sli4_sge_dif_hi_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_type_SHIFT: c_int = 27;
pub const lpfc_sli4_sge_dif_type_MASK: c_uint = 0x0000000F;

pub const lpfc_sli4_sge_dif_last_MASK: c_uint = 0x00000001;

    pub word3: u32,
pub const lpfc_sli4_sge_dif_apptag_SHIFT: c_int = 0;
pub const lpfc_sli4_sge_dif_apptag_MASK: c_uint = 0x0000FFFF;

pub const lpfc_sli4_sge_dif_bs_SHIFT: c_int = 16;
pub const lpfc_sli4_sge_dif_bs_MASK: c_uint = 0x00000007;

pub const lpfc_sli4_sge_dif_ai_SHIFT: c_int = 19;
pub const lpfc_sli4_sge_dif_ai_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_me_SHIFT: c_int = 20;
pub const lpfc_sli4_sge_dif_me_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_re_SHIFT: c_int = 21;
pub const lpfc_sli4_sge_dif_re_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_ce_SHIFT: c_int = 22;
pub const lpfc_sli4_sge_dif_ce_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_nr_SHIFT: c_int = 23;
pub const lpfc_sli4_sge_dif_nr_MASK: c_uint = 0x00000001;

pub const lpfc_sli4_sge_dif_oprx_SHIFT: c_int = 24;
pub const lpfc_sli4_sge_dif_oprx_MASK: c_uint = 0x0000000F;

pub const lpfc_sli4_sge_dif_optx_SHIFT: c_int = 28;
pub const lpfc_sli4_sge_dif_optx_MASK: c_uint = 0x0000000F;

// optx and oprx use BG_OP_IN defines in lpfc_hw.h
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcf_record {
    pub max_rcv_size: u32,
    pub fka_adv_period: u32,
    pub fip_priority: u32,
    pub word3: u32,
pub const lpfc_fcf_record_mac_0_SHIFT: c_int = 0;
pub const lpfc_fcf_record_mac_0_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_mac_1_SHIFT: c_int = 8;
pub const lpfc_fcf_record_mac_1_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_mac_2_SHIFT: c_int = 16;
pub const lpfc_fcf_record_mac_2_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_mac_3_SHIFT: c_int = 24;
pub const lpfc_fcf_record_mac_3_MASK: c_uint = 0x000000FF;

    pub word4: u32,
pub const lpfc_fcf_record_mac_4_SHIFT: c_int = 0;
pub const lpfc_fcf_record_mac_4_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_mac_5_SHIFT: c_int = 8;
pub const lpfc_fcf_record_mac_5_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fcf_avail_SHIFT: c_int = 16;
pub const lpfc_fcf_record_fcf_avail_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_mac_addr_prov_SHIFT: c_int = 24;
pub const lpfc_fcf_record_mac_addr_prov_MASK: c_uint = 0x000000FF;

    pub word5: u32,
pub const lpfc_fcf_record_fab_name_0_SHIFT: c_int = 0;
pub const lpfc_fcf_record_fab_name_0_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_1_SHIFT: c_int = 8;
pub const lpfc_fcf_record_fab_name_1_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_2_SHIFT: c_int = 16;
pub const lpfc_fcf_record_fab_name_2_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_3_SHIFT: c_int = 24;
pub const lpfc_fcf_record_fab_name_3_MASK: c_uint = 0x000000FF;

    pub word6: u32,
pub const lpfc_fcf_record_fab_name_4_SHIFT: c_int = 0;
pub const lpfc_fcf_record_fab_name_4_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_5_SHIFT: c_int = 8;
pub const lpfc_fcf_record_fab_name_5_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_6_SHIFT: c_int = 16;
pub const lpfc_fcf_record_fab_name_6_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fab_name_7_SHIFT: c_int = 24;
pub const lpfc_fcf_record_fab_name_7_MASK: c_uint = 0x000000FF;

    pub word7: u32,
pub const lpfc_fcf_record_fc_map_0_SHIFT: c_int = 0;
pub const lpfc_fcf_record_fc_map_0_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fc_map_1_SHIFT: c_int = 8;
pub const lpfc_fcf_record_fc_map_1_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fc_map_2_SHIFT: c_int = 16;
pub const lpfc_fcf_record_fc_map_2_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_fcf_valid_SHIFT: c_int = 24;
pub const lpfc_fcf_record_fcf_valid_MASK: c_uint = 0x00000001;

pub const lpfc_fcf_record_fcf_fc_SHIFT: c_int = 25;
pub const lpfc_fcf_record_fcf_fc_MASK: c_uint = 0x00000001;

pub const lpfc_fcf_record_fcf_sol_SHIFT: c_int = 31;
pub const lpfc_fcf_record_fcf_sol_MASK: c_uint = 0x00000001;

    pub word8: u32,
pub const lpfc_fcf_record_fcf_index_SHIFT: c_int = 0;
pub const lpfc_fcf_record_fcf_index_MASK: c_uint = 0x0000FFFF;

pub const lpfc_fcf_record_fcf_state_SHIFT: c_int = 16;
pub const lpfc_fcf_record_fcf_state_MASK: c_uint = 0x0000FFFF;
    pub vlan_bitmap: [u8; 512],
    pub word137: u32,
pub const lpfc_fcf_record_switch_name_0_SHIFT: c_int = 0;
pub const lpfc_fcf_record_switch_name_0_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_1_SHIFT: c_int = 8;
pub const lpfc_fcf_record_switch_name_1_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_2_SHIFT: c_int = 16;
pub const lpfc_fcf_record_switch_name_2_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_3_SHIFT: c_int = 24;
pub const lpfc_fcf_record_switch_name_3_MASK: c_uint = 0x000000FF;

    pub word138: u32,
pub const lpfc_fcf_record_switch_name_4_SHIFT: c_int = 0;
pub const lpfc_fcf_record_switch_name_4_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_5_SHIFT: c_int = 8;
pub const lpfc_fcf_record_switch_name_5_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_6_SHIFT: c_int = 16;
pub const lpfc_fcf_record_switch_name_6_MASK: c_uint = 0x000000FF;

pub const lpfc_fcf_record_switch_name_7_SHIFT: c_int = 24;
pub const lpfc_fcf_record_switch_name_7_MASK: c_uint = 0x000000FF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_fcf_tbl {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word10: u32,
pub const lpfc_mbx_read_fcf_tbl_indx_SHIFT: c_int = 0;
pub const lpfc_mbx_read_fcf_tbl_indx_MASK: c_uint = 0x0000FFFF;

    pub request: },
    pub eventag: u32,
    pub response: },
    pub u: },
    pub word11: u32,
pub const lpfc_mbx_read_fcf_tbl_nxt_vindx_SHIFT: c_int = 0;
pub const lpfc_mbx_read_fcf_tbl_nxt_vindx_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_add_fcf_tbl_entry {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub word10: u32,
pub const lpfc_mbx_add_fcf_tbl_fcfi_SHIFT: c_int = 0;
pub const lpfc_mbx_add_fcf_tbl_fcfi_MASK: c_uint = 0x0000FFFF;

    pub fcf_sge: lpfc_mbx_sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_del_fcf_tbl_entry {
    pub header: mbox_header,
    pub word10: u32,
pub const lpfc_mbx_del_fcf_tbl_count_SHIFT: c_int = 0;
pub const lpfc_mbx_del_fcf_tbl_count_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_del_fcf_tbl_index_SHIFT: c_int = 16;
pub const lpfc_mbx_del_fcf_tbl_index_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_redisc_fcf_tbl {
    pub header: mbox_header,
    pub word10: u32,
pub const lpfc_mbx_redisc_fcf_count_SHIFT: c_int = 0;
pub const lpfc_mbx_redisc_fcf_count_MASK: c_uint = 0x0000FFFF;

    pub resvd: u32,
    pub word12: u32,
pub const lpfc_mbx_redisc_fcf_index_SHIFT: c_int = 0;
pub const lpfc_mbx_redisc_fcf_index_MASK: c_uint = 0x0000FFFF;

}

// Status field for embedded SLI_CONFIG mailbox command
pub const STATUS_SUCCESS: c_uint = 0x0;
pub const STATUS_FAILED: c_uint = 0x1;
pub const STATUS_ILLEGAL_REQUEST: c_uint = 0x2;
pub const STATUS_ILLEGAL_FIELD: c_uint = 0x3;
pub const STATUS_INSUFFICIENT_BUFFER: c_uint = 0x4;
pub const STATUS_UNAUTHORIZED_REQUEST: c_uint = 0x5;
pub const STATUS_FLASHROM_SAVE_FAILED: c_uint = 0x17;
pub const STATUS_FLASHROM_RESTORE_FAILED: c_uint = 0x18;
pub const STATUS_ICCBINDEX_ALLOC_FAILED: c_uint = 0x1a;
pub const STATUS_IOCTLHANDLE_ALLOC_FAILED: c_uint = 0x1b;
pub const STATUS_INVALID_PHY_ADDR_FROM_OSM: c_uint = 0x1c;
pub const STATUS_INVALID_PHY_ADDR_LEN_FROM_OSM: c_uint = 0x1d;
pub const STATUS_ASSERT_FAILED: c_uint = 0x1e;
pub const STATUS_INVALID_SESSION: c_uint = 0x1f;
pub const STATUS_INVALID_CONNECTION: c_uint = 0x20;
pub const STATUS_BTL_PATH_EXCEEDS_OSM_LIMIT: c_uint = 0x21;
pub const STATUS_BTL_NO_FREE_SLOT_PATH: c_uint = 0x24;
pub const STATUS_BTL_NO_FREE_SLOT_TGTID: c_uint = 0x25;
pub const STATUS_OSM_DEVSLOT_NOT_FOUND: c_uint = 0x26;
pub const STATUS_FLASHROM_READ_FAILED: c_uint = 0x27;
pub const STATUS_POLL_IOCTL_TIMEOUT: c_uint = 0x28;
pub const STATUS_ERROR_ACITMAIN: c_uint = 0x2a;
pub const STATUS_REBOOT_REQUIRED: c_uint = 0x2c;
pub const STATUS_FCF_IN_USE: c_uint = 0x3a;
pub const STATUS_FCF_TABLE_EMPTY: c_uint = 0x43;
//
// Additional status field for embedded SLI_CONFIG mailbox
// command.
//
pub const ADD_STATUS_OPERATION_ALREADY_ACTIVE: c_uint = 0x67;
pub const ADD_STATUS_FW_NOT_SUPPORTED: c_uint = 0xEB;
pub const ADD_STATUS_INVALID_REQUEST: c_uint = 0x4B;
pub const ADD_STATUS_INVALID_OBJECT_NAME: c_uint = 0xA0;
pub const ADD_STATUS_FW_DOWNLOAD_HW_DISABLED: c_uint = 0x58;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_sli4_config {
    pub header: mbox_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_init_vfi {
    pub word1: u32,
pub const lpfc_init_vfi_vr_SHIFT: c_int = 31;
pub const lpfc_init_vfi_vr_MASK: c_uint = 0x00000001;

pub const lpfc_init_vfi_vt_SHIFT: c_int = 30;
pub const lpfc_init_vfi_vt_MASK: c_uint = 0x00000001;

pub const lpfc_init_vfi_vf_SHIFT: c_int = 29;
pub const lpfc_init_vfi_vf_MASK: c_uint = 0x00000001;

pub const lpfc_init_vfi_vp_SHIFT: c_int = 28;
pub const lpfc_init_vfi_vp_MASK: c_uint = 0x00000001;

pub const lpfc_init_vfi_vfi_SHIFT: c_int = 0;
pub const lpfc_init_vfi_vfi_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_init_vfi_vpi_SHIFT: c_int = 16;
pub const lpfc_init_vfi_vpi_MASK: c_uint = 0x0000FFFF;

pub const lpfc_init_vfi_fcfi_SHIFT: c_int = 0;
pub const lpfc_init_vfi_fcfi_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,
pub const lpfc_init_vfi_pri_SHIFT: c_int = 13;
pub const lpfc_init_vfi_pri_MASK: c_uint = 0x00000007;

pub const lpfc_init_vfi_vf_id_SHIFT: c_int = 1;
pub const lpfc_init_vfi_vf_id_MASK: c_uint = 0x00000FFF;

    pub word4: u32,
pub const lpfc_init_vfi_hop_count_SHIFT: c_int = 24;
pub const lpfc_init_vfi_hop_count_MASK: c_uint = 0x000000FF;

}

pub const MBX_VFI_IN_USE: c_uint = 0x9F02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_reg_vfi {
    pub word1: u32,
pub const lpfc_reg_vfi_upd_SHIFT: c_int = 29;
pub const lpfc_reg_vfi_upd_MASK: c_uint = 0x00000001;

pub const lpfc_reg_vfi_vp_SHIFT: c_int = 28;
pub const lpfc_reg_vfi_vp_MASK: c_uint = 0x00000001;

pub const lpfc_reg_vfi_vfi_SHIFT: c_int = 0;
pub const lpfc_reg_vfi_vfi_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_reg_vfi_vpi_SHIFT: c_int = 16;
pub const lpfc_reg_vfi_vpi_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_vfi_fcfi_SHIFT: c_int = 0;
pub const lpfc_reg_vfi_fcfi_MASK: c_uint = 0x0000FFFF;
    pub wwn: [u32; 2],
    pub bde: ulp_bde64,
    pub e_d_tov: u32,
    pub r_a_tov: u32,
    pub word10: u32,
pub const lpfc_reg_vfi_nport_id_SHIFT: c_int = 0;
pub const lpfc_reg_vfi_nport_id_MASK: c_uint = 0x00FFFFFF;

pub const lpfc_reg_vfi_bbcr_SHIFT: c_int = 27;
pub const lpfc_reg_vfi_bbcr_MASK: c_uint = 0x00000001;

pub const lpfc_reg_vfi_bbscn_SHIFT: c_int = 28;
pub const lpfc_reg_vfi_bbscn_MASK: c_uint = 0x0000000F;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_init_vpi {
    pub word1: u32,
pub const lpfc_init_vpi_vfi_SHIFT: c_int = 16;
pub const lpfc_init_vpi_vfi_MASK: c_uint = 0x0000FFFF;

pub const lpfc_init_vpi_vpi_SHIFT: c_int = 0;
pub const lpfc_init_vpi_vpi_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_vpi {
    pub word1_rsvd: u32,
    pub word2: u32,
pub const lpfc_mbx_read_vpi_vnportid_SHIFT: c_int = 0;
pub const lpfc_mbx_read_vpi_vnportid_MASK: c_uint = 0x00FFFFFF;

    pub word3_rsvd: u32,
    pub word4: u32,
pub const lpfc_mbx_read_vpi_acq_alpa_SHIFT: c_int = 0;
pub const lpfc_mbx_read_vpi_acq_alpa_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_pb_SHIFT: c_int = 15;
pub const lpfc_mbx_read_vpi_pb_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_vpi_spec_alpa_SHIFT: c_int = 16;
pub const lpfc_mbx_read_vpi_spec_alpa_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_ns_SHIFT: c_int = 30;
pub const lpfc_mbx_read_vpi_ns_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_read_vpi_hl_SHIFT: c_int = 31;
pub const lpfc_mbx_read_vpi_hl_MASK: c_uint = 0x00000001;

    pub word5_rsvd: u32,
    pub word6: u32,
pub const lpfc_mbx_read_vpi_vpi_SHIFT: c_int = 0;
pub const lpfc_mbx_read_vpi_vpi_MASK: c_uint = 0x0000FFFF;

    pub word7: u32,
pub const lpfc_mbx_read_vpi_mac_0_SHIFT: c_int = 0;
pub const lpfc_mbx_read_vpi_mac_0_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_mac_1_SHIFT: c_int = 8;
pub const lpfc_mbx_read_vpi_mac_1_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_mac_2_SHIFT: c_int = 16;
pub const lpfc_mbx_read_vpi_mac_2_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_mac_3_SHIFT: c_int = 24;
pub const lpfc_mbx_read_vpi_mac_3_MASK: c_uint = 0x000000FF;

    pub word8: u32,
pub const lpfc_mbx_read_vpi_mac_4_SHIFT: c_int = 0;
pub const lpfc_mbx_read_vpi_mac_4_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_mac_5_SHIFT: c_int = 8;
pub const lpfc_mbx_read_vpi_mac_5_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_read_vpi_vlan_tag_SHIFT: c_int = 16;
pub const lpfc_mbx_read_vpi_vlan_tag_MASK: c_uint = 0x00000FFF;

pub const lpfc_mbx_read_vpi_vv_SHIFT: c_int = 28;
pub const lpfc_mbx_read_vpi_vv_MASK: c_uint = 0x0000001;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_unreg_vfi {
    pub word1_rsvd: u32,
    pub word2: u32,
pub const lpfc_unreg_vfi_vfi_SHIFT: c_int = 0;
pub const lpfc_unreg_vfi_vfi_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_resume_rpi {
    pub word1: u32,
pub const lpfc_resume_rpi_index_SHIFT: c_int = 0;
pub const lpfc_resume_rpi_index_MASK: c_uint = 0x0000FFFF;

pub const lpfc_resume_rpi_ii_SHIFT: c_int = 30;
pub const lpfc_resume_rpi_ii_MASK: c_uint = 0x00000003;

pub const RESUME_INDEX_RPI: c_int = 0;
pub const RESUME_INDEX_VPI: c_int = 1;
pub const RESUME_INDEX_VFI: c_int = 2;
pub const RESUME_INDEX_FCFI: c_int = 3;
    pub event_tag: u32,
}

pub const REG_FCF_INVALID_QID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_reg_fcfi {
    pub word1: u32,
pub const lpfc_reg_fcfi_info_index_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_info_index_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_fcfi_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_fcfi_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_reg_fcfi_rq_id1_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rq_id1_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_rq_id0_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_rq_id0_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,
pub const lpfc_reg_fcfi_rq_id3_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rq_id3_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_rq_id2_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_rq_id2_MASK: c_uint = 0x0000FFFF;

    pub word4: u32,
pub const lpfc_reg_fcfi_type_match0_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_type_match0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_type_mask0_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_type_mask0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_match0_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_rctl_match0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_mask0_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rctl_mask0_MASK: c_uint = 0x000000FF;

    pub word5: u32,
pub const lpfc_reg_fcfi_type_match1_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_type_match1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_type_mask1_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_type_mask1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_match1_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_rctl_match1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_mask1_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rctl_mask1_MASK: c_uint = 0x000000FF;

    pub word6: u32,
pub const lpfc_reg_fcfi_type_match2_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_type_match2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_type_mask2_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_type_mask2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_match2_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_rctl_match2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_mask2_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rctl_mask2_MASK: c_uint = 0x000000FF;

    pub word7: u32,
pub const lpfc_reg_fcfi_type_match3_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_type_match3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_type_mask3_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_type_mask3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_match3_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_rctl_match3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_rctl_mask3_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_rctl_mask3_MASK: c_uint = 0x000000FF;

    pub word8: u32,
pub const lpfc_reg_fcfi_mam_SHIFT: c_int = 13;
pub const lpfc_reg_fcfi_mam_MASK: c_uint = 0x00000003;

pub const lpfc_reg_fcfi_vv_SHIFT: c_int = 12;
pub const lpfc_reg_fcfi_vv_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_vlan_tag_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_vlan_tag_MASK: c_uint = 0x00000FFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_reg_fcfi_mrq {
    pub word1: u32,
pub const lpfc_reg_fcfi_mrq_info_index_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_info_index_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_mrq_fcfi_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_fcfi_MASK: c_uint = 0x0000FFFF;

    pub word2: u32,
pub const lpfc_reg_fcfi_mrq_rq_id1_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rq_id1_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_mrq_rq_id0_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_rq_id0_MASK: c_uint = 0x0000FFFF;

    pub word3: u32,
pub const lpfc_reg_fcfi_mrq_rq_id3_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rq_id3_MASK: c_uint = 0x0000FFFF;

pub const lpfc_reg_fcfi_mrq_rq_id2_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_rq_id2_MASK: c_uint = 0x0000FFFF;

    pub word4: u32,
pub const lpfc_reg_fcfi_mrq_type_match0_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_mrq_type_match0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_type_mask0_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_type_mask0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_match0_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_mrq_rctl_match0_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_mask0_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rctl_mask0_MASK: c_uint = 0x000000FF;

    pub word5: u32,
pub const lpfc_reg_fcfi_mrq_type_match1_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_mrq_type_match1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_type_mask1_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_type_mask1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_match1_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_mrq_rctl_match1_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_mask1_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rctl_mask1_MASK: c_uint = 0x000000FF;

    pub word6: u32,
pub const lpfc_reg_fcfi_mrq_type_match2_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_mrq_type_match2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_type_mask2_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_type_mask2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_match2_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_mrq_rctl_match2_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_mask2_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rctl_mask2_MASK: c_uint = 0x000000FF;

    pub word7: u32,
pub const lpfc_reg_fcfi_mrq_type_match3_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_mrq_type_match3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_type_mask3_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_type_mask3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_match3_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_mrq_rctl_match3_MASK: c_uint = 0x000000FF;

pub const lpfc_reg_fcfi_mrq_rctl_mask3_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_rctl_mask3_MASK: c_uint = 0x000000FF;

    pub word8: u32,
pub const lpfc_reg_fcfi_mrq_ptc7_SHIFT: c_int = 31;
pub const lpfc_reg_fcfi_mrq_ptc7_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc6_SHIFT: c_int = 30;
pub const lpfc_reg_fcfi_mrq_ptc6_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc5_SHIFT: c_int = 29;
pub const lpfc_reg_fcfi_mrq_ptc5_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc4_SHIFT: c_int = 28;
pub const lpfc_reg_fcfi_mrq_ptc4_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc3_SHIFT: c_int = 27;
pub const lpfc_reg_fcfi_mrq_ptc3_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc2_SHIFT: c_int = 26;
pub const lpfc_reg_fcfi_mrq_ptc2_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc1_SHIFT: c_int = 25;
pub const lpfc_reg_fcfi_mrq_ptc1_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_ptc0_SHIFT: c_int = 24;
pub const lpfc_reg_fcfi_mrq_ptc0_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt7_SHIFT: c_int = 23;
pub const lpfc_reg_fcfi_mrq_pt7_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt6_SHIFT: c_int = 22;
pub const lpfc_reg_fcfi_mrq_pt6_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt5_SHIFT: c_int = 21;
pub const lpfc_reg_fcfi_mrq_pt5_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt4_SHIFT: c_int = 20;
pub const lpfc_reg_fcfi_mrq_pt4_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt3_SHIFT: c_int = 19;
pub const lpfc_reg_fcfi_mrq_pt3_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt2_SHIFT: c_int = 18;
pub const lpfc_reg_fcfi_mrq_pt2_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt1_SHIFT: c_int = 17;
pub const lpfc_reg_fcfi_mrq_pt1_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_pt0_SHIFT: c_int = 16;
pub const lpfc_reg_fcfi_mrq_pt0_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_xmv_SHIFT: c_int = 15;
pub const lpfc_reg_fcfi_mrq_xmv_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_mode_SHIFT: c_int = 13;
pub const lpfc_reg_fcfi_mrq_mode_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_vv_SHIFT: c_int = 12;
pub const lpfc_reg_fcfi_mrq_vv_MASK: c_uint = 0x00000001;

pub const lpfc_reg_fcfi_mrq_vlan_tag_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_vlan_tag_MASK: c_uint = 0x00000FFF;

    pub word9: u32,
pub const lpfc_reg_fcfi_mrq_policy_SHIFT: c_int = 12;
pub const lpfc_reg_fcfi_mrq_policy_MASK: c_uint = 0x0000000F;

pub const lpfc_reg_fcfi_mrq_filter_SHIFT: c_int = 8;
pub const lpfc_reg_fcfi_mrq_filter_MASK: c_uint = 0x0000000F;

pub const lpfc_reg_fcfi_mrq_npairs_SHIFT: c_int = 0;
pub const lpfc_reg_fcfi_mrq_npairs_MASK: c_uint = 0x000000FF;

    pub word10: u32,
    pub word11: u32,
    pub word12: u32,
    pub word13: u32,
    pub word14: u32,
    pub word15: u32,
    pub word16: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_unreg_fcfi {
    pub word1_rsv: u32,
    pub word2: u32,
pub const lpfc_unreg_fcfi_SHIFT: c_int = 0;
pub const lpfc_unreg_fcfi_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_rev {
    pub word1: u32,
pub const lpfc_mbx_rd_rev_sli_lvl_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_rev_sli_lvl_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_rd_rev_fcoe_SHIFT: c_int = 20;
pub const lpfc_mbx_rd_rev_fcoe_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_rev_cee_ver_SHIFT: c_int = 21;
pub const lpfc_mbx_rd_rev_cee_ver_MASK: c_uint = 0x00000003;

pub const LPFC_PREDCBX_CEE_MODE: c_int = 0;
pub const LPFC_DCBX_CEE_MODE: c_int = 1;
pub const lpfc_mbx_rd_rev_vpd_SHIFT: c_int = 29;
pub const lpfc_mbx_rd_rev_vpd_MASK: c_uint = 0x00000001;

    pub first_hw_rev: u32,
pub const LPFC_G7_ASIC_1: c_uint = 0xd;
    pub second_hw_rev: u32,
    pub word4_rsvd: u32,
    pub third_hw_rev: u32,
    pub word6: u32,
pub const lpfc_mbx_rd_rev_fcph_low_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_rev_fcph_low_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_rd_rev_fcph_high_SHIFT: c_int = 8;
pub const lpfc_mbx_rd_rev_fcph_high_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_rd_rev_ftr_lvl_low_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_rev_ftr_lvl_low_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_rd_rev_ftr_lvl_high_SHIFT: c_int = 24;
pub const lpfc_mbx_rd_rev_ftr_lvl_high_MASK: c_uint = 0x000000FF;

    pub word7_rsvd: u32,
    pub fw_id_rev: u32,
    pub fw_name: [u8; 16],
    pub ulp_fw_id_rev: u32,
    pub ulp_fw_name: [u8; 16],
    pub word18_47_rsvd: [u32; 30],
    pub word48: u32,
pub const lpfc_mbx_rd_rev_avail_len_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_rev_avail_len_MASK: c_uint = 0x00FFFFFF;

    pub vpd_paddr_low: u32,
    pub vpd_paddr_high: u32,
    pub avail_vpd_len: u32,
    pub rsvd_52_63: [u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_read_config {
    pub word1: u32,
pub const lpfc_mbx_rd_conf_extnts_inuse_SHIFT: c_int = 31;
pub const lpfc_mbx_rd_conf_extnts_inuse_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_fawwpn_SHIFT: c_int = 30;
pub const lpfc_mbx_rd_conf_fawwpn_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_wcs_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_acs_MASK: c_uint = 0x00000001;

    pub word2: u32,
pub const lpfc_mbx_rd_conf_lnk_numb_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_lnk_numb_MASK: c_uint = 0x0000003F;

pub const lpfc_mbx_rd_conf_lnk_type_SHIFT: c_int = 6;
pub const lpfc_mbx_rd_conf_lnk_type_MASK: c_uint = 0x00000003;

pub const LPFC_LNK_TYPE_GE: c_int = 0;
pub const LPFC_LNK_TYPE_FC: c_int = 1;
pub const lpfc_mbx_rd_conf_lnk_ldv_SHIFT: c_int = 8;
pub const lpfc_mbx_rd_conf_lnk_ldv_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_trunk_SHIFT: c_int = 12;
pub const lpfc_mbx_rd_conf_trunk_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_rd_conf_pt_SHIFT: c_int = 20;
pub const lpfc_mbx_rd_conf_pt_MASK: c_uint = 0x00000003;

pub const lpfc_mbx_rd_conf_tf_SHIFT: c_int = 22;
pub const lpfc_mbx_rd_conf_tf_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_ptv_SHIFT: c_int = 23;
pub const lpfc_mbx_rd_conf_ptv_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rd_conf_topology_SHIFT: c_int = 24;
pub const lpfc_mbx_rd_conf_topology_MASK: c_uint = 0x000000FF;

    pub word3: u32,
pub const lpfc_mbx_rd_conf_fedif_SHIFT: c_int = 6;
pub const lpfc_mbx_rd_conf_fedif_MASK: c_uint = 0x00000001;

    pub word4: u32,
pub const lpfc_mbx_rd_conf_e_d_tov_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_e_d_tov_MASK: c_uint = 0x0000FFFF;

    pub rsvd_5: u32,
    pub word6: u32,
pub const lpfc_mbx_rd_conf_r_a_tov_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_r_a_tov_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_link_speed_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_link_speed_MASK: c_uint = 0x0000FFFF;

    pub rsvd_7: u32,
    pub word8: u32,
pub const lpfc_mbx_rd_conf_bbscn_min_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_bbscn_min_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_rd_conf_bbscn_max_SHIFT: c_int = 4;
pub const lpfc_mbx_rd_conf_bbscn_max_MASK: c_uint = 0x0000000F;

pub const lpfc_mbx_rd_conf_bbscn_def_SHIFT: c_int = 8;
pub const lpfc_mbx_rd_conf_bbscn_def_MASK: c_uint = 0x0000000F;

    pub word9: u32,
pub const lpfc_mbx_rd_conf_lmt_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_lmt_MASK: c_uint = 0x0000FFFF;

    pub rsvd_10: u32,
    pub rsvd_11: u32,
    pub word12: u32,
pub const lpfc_mbx_rd_conf_xri_base_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_xri_base_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_xri_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_xri_count_MASK: c_uint = 0x0000FFFF;

    pub word13: u32,
pub const lpfc_mbx_rd_conf_rpi_base_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_rpi_base_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_rpi_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_rpi_count_MASK: c_uint = 0x0000FFFF;

    pub word14: u32,
pub const lpfc_mbx_rd_conf_vpi_base_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_vpi_base_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_vpi_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_vpi_count_MASK: c_uint = 0x0000FFFF;

    pub word15: u32,
pub const lpfc_mbx_rd_conf_vfi_base_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_vfi_base_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_vfi_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_vfi_count_MASK: c_uint = 0x0000FFFF;

    pub word16: u32,
pub const lpfc_mbx_rd_conf_fcfi_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_fcfi_count_MASK: c_uint = 0x0000FFFF;

    pub word17: u32,
pub const lpfc_mbx_rd_conf_rq_count_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_rq_count_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_eq_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_eq_count_MASK: c_uint = 0x0000FFFF;

    pub word18: u32,
pub const lpfc_mbx_rd_conf_wq_count_SHIFT: c_int = 0;
pub const lpfc_mbx_rd_conf_wq_count_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mbx_rd_conf_cq_count_SHIFT: c_int = 16;
pub const lpfc_mbx_rd_conf_cq_count_MASK: c_uint = 0x0000FFFF;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_request_features {
    pub word1: u32,
pub const lpfc_mbx_rq_ftr_qry_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_ftr_qry_MASK: c_uint = 0x00000001;

    pub word2: u32,
pub const lpfc_mbx_rq_ftr_rq_iaab_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_ftr_rq_iaab_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_npiv_SHIFT: c_int = 1;
pub const lpfc_mbx_rq_ftr_rq_npiv_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_dif_SHIFT: c_int = 2;
pub const lpfc_mbx_rq_ftr_rq_dif_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_vf_SHIFT: c_int = 3;
pub const lpfc_mbx_rq_ftr_rq_vf_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_fcpi_SHIFT: c_int = 4;
pub const lpfc_mbx_rq_ftr_rq_fcpi_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_fcpt_SHIFT: c_int = 5;
pub const lpfc_mbx_rq_ftr_rq_fcpt_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_fcpc_SHIFT: c_int = 6;
pub const lpfc_mbx_rq_ftr_rq_fcpc_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_ifip_SHIFT: c_int = 7;
pub const lpfc_mbx_rq_ftr_rq_ifip_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_iaar_SHIFT: c_int = 9;
pub const lpfc_mbx_rq_ftr_rq_iaar_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_mrqp_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_ftr_rq_mrqp_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rq_ashdr_SHIFT: c_int = 17;
pub const lpfc_mbx_rq_ftr_rq_ashdr_MASK: c_uint = 0x00000001;

    pub word3: u32,
pub const lpfc_mbx_rq_ftr_rsp_iaab_SHIFT: c_int = 0;
pub const lpfc_mbx_rq_ftr_rsp_iaab_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_npiv_SHIFT: c_int = 1;
pub const lpfc_mbx_rq_ftr_rsp_npiv_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_dif_SHIFT: c_int = 2;
pub const lpfc_mbx_rq_ftr_rsp_dif_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_vf_SHIFT: c_int = 3;
pub const lpfc_mbx_rq_ftr_rsp_vf__MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_fcpi_SHIFT: c_int = 4;
pub const lpfc_mbx_rq_ftr_rsp_fcpi_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_fcpt_SHIFT: c_int = 5;
pub const lpfc_mbx_rq_ftr_rsp_fcpt_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_fcpc_SHIFT: c_int = 6;
pub const lpfc_mbx_rq_ftr_rsp_fcpc_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_ifip_SHIFT: c_int = 7;
pub const lpfc_mbx_rq_ftr_rsp_ifip_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_mrqp_SHIFT: c_int = 16;
pub const lpfc_mbx_rq_ftr_rsp_mrqp_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_rq_ftr_rsp_ashdr_SHIFT: c_int = 17;
pub const lpfc_mbx_rq_ftr_rsp_ashdr_MASK: c_uint = 0x00000001;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_memory_dump_type3 {
    pub word1: u32,
pub const lpfc_mbx_memory_dump_type3_type_SHIFT: c_int = 0;
pub const lpfc_mbx_memory_dump_type3_type_MASK: c_uint = 0x0000000f;

pub const lpfc_mbx_memory_dump_type3_link_SHIFT: c_int = 24;
pub const lpfc_mbx_memory_dump_type3_link_MASK: c_uint = 0x000000ff;

    pub word2: u32,
pub const lpfc_mbx_memory_dump_type3_page_no_SHIFT: c_int = 0;
pub const lpfc_mbx_memory_dump_type3_page_no_MASK: c_uint = 0x0000ffff;

pub const lpfc_mbx_memory_dump_type3_offset_SHIFT: c_int = 16;
pub const lpfc_mbx_memory_dump_type3_offset_MASK: c_uint = 0x0000ffff;

    pub word3: u32,
pub const lpfc_mbx_memory_dump_type3_length_SHIFT: c_int = 0;
pub const lpfc_mbx_memory_dump_type3_length_MASK: c_uint = 0x00ffffff;

    pub addr_lo: u32,
    pub addr_hi: u32,
    pub return_len: u32,
}

pub const DMP_PAGE_A0: c_uint = 0xa0;
pub const DMP_PAGE_A2: c_uint = 0xa2;
pub const DMP_SFF_PAGE_A0_SIZE: c_int = 256;
pub const DMP_SFF_PAGE_A2_SIZE: c_int = 256;
pub const SFP_WAVELENGTH_LC1310: c_int = 1310;
pub const SFP_WAVELENGTH_LL1550: c_int = 1550;
//
// * SFF-8472 TABLE 3.4
//
pub const SFF_PG0_CONNECTOR_UNKNOWN: c_uint = 0x00   /* Unknown  */;
pub const SFF_PG0_CONNECTOR_SC: c_uint = 0x01   /* SC       */;
pub const SFF_PG0_CONNECTOR_FC_COPPER1: c_uint = 0x02   /* FC style 1 copper connector */;
pub const SFF_PG0_CONNECTOR_FC_COPPER2: c_uint = 0x03   /* FC style 2 copper connector */;
pub const SFF_PG0_CONNECTOR_BNC: c_uint = 0x04   /* BNC / TNC */;
pub const SFF_PG0_CONNECTOR__FC_COAX: c_uint = 0x05   /* FC coaxial headers */;
pub const SFF_PG0_CONNECTOR_FIBERJACK: c_uint = 0x06   /* FiberJack */;
pub const SFF_PG0_CONNECTOR_LC: c_uint = 0x07   /* LC        */;
pub const SFF_PG0_CONNECTOR_MT: c_uint = 0x08   /* MT - RJ   */;
pub const SFF_PG0_CONNECTOR_MU: c_uint = 0x09   /* MU        */;
pub const SFF_PG0_CONNECTOR_SF: c_uint = 0x0A   /* SG        */;
pub const SFF_PG0_CONNECTOR_OPTICAL_PIGTAIL: c_uint = 0x0B /* Optical pigtail */;
pub const SFF_PG0_CONNECTOR_OPTICAL_PARALLEL: c_uint = 0x0C /* MPO Parallel Optic */;
pub const SFF_PG0_CONNECTOR_HSSDC_II: c_uint = 0x20   /* HSSDC II */;
pub const SFF_PG0_CONNECTOR_COPPER_PIGTAIL: c_uint = 0x21 /* Copper pigtail */;
pub const SFF_PG0_CONNECTOR_RJ45: c_uint = 0x22  /* RJ45 */;
// SFF-8472 Table 3.1 Diagnostics: Data Fields Address/Page A0
pub const SSF_IDENTIFIER: c_int = 0;
pub const SSF_EXT_IDENTIFIER: c_int = 1;
pub const SSF_CONNECTOR: c_int = 2;
pub const SSF_TRANSCEIVER_CODE_B0: c_int = 3;
pub const SSF_TRANSCEIVER_CODE_B1: c_int = 4;
pub const SSF_TRANSCEIVER_CODE_B2: c_int = 5;
pub const SSF_TRANSCEIVER_CODE_B3: c_int = 6;
pub const SSF_TRANSCEIVER_CODE_B4: c_int = 7;
pub const SSF_TRANSCEIVER_CODE_B5: c_int = 8;
pub const SSF_TRANSCEIVER_CODE_B6: c_int = 9;
pub const SSF_TRANSCEIVER_CODE_B7: c_int = 10;
pub const SSF_ENCODING: c_int = 11;
pub const SSF_BR_NOMINAL: c_int = 12;
pub const SSF_RATE_IDENTIFIER: c_int = 13;
pub const SSF_LENGTH_9UM_KM: c_int = 14;
pub const SSF_LENGTH_9UM: c_int = 15;
pub const SSF_LENGTH_50UM_OM2: c_int = 16;
pub const SSF_LENGTH_62UM_OM1: c_int = 17;
pub const SFF_LENGTH_COPPER: c_int = 18;
pub const SSF_LENGTH_50UM_OM3: c_int = 19;
pub const SSF_VENDOR_NAME: c_int = 20;
pub const SSF_TRANSCEIVER2: c_int = 36;
pub const SSF_VENDOR_OUI: c_int = 37;
pub const SSF_VENDOR_PN: c_int = 40;
pub const SSF_VENDOR_REV: c_int = 56;
pub const SSF_WAVELENGTH_B1: c_int = 60;
pub const SSF_WAVELENGTH_B0: c_int = 61;
pub const SSF_CC_BASE: c_int = 63;
pub const SSF_OPTIONS_B1: c_int = 64;
pub const SSF_OPTIONS_B0: c_int = 65;
pub const SSF_BR_MAX: c_int = 66;
pub const SSF_BR_MIN: c_int = 67;
pub const SSF_VENDOR_SN: c_int = 68;
pub const SSF_DATE_CODE: c_int = 84;
pub const SSF_MONITORING_TYPEDIAGNOSTIC: c_int = 92;
pub const SSF_ENHANCED_OPTIONS: c_int = 93;
pub const SFF_8472_COMPLIANCE: c_int = 94;
pub const SSF_CC_EXT: c_int = 95;
pub const SSF_A0_VENDOR_SPECIFIC: c_int = 96;
// SFF-8472 Table 3.1a Diagnostics: Data Fields Address/Page A2
pub const SSF_TEMP_HIGH_ALARM: c_int = 0;
pub const SSF_TEMP_LOW_ALARM: c_int = 2;
pub const SSF_TEMP_HIGH_WARNING: c_int = 4;
pub const SSF_TEMP_LOW_WARNING: c_int = 6;
pub const SSF_VOLTAGE_HIGH_ALARM: c_int = 8;
pub const SSF_VOLTAGE_LOW_ALARM: c_int = 10;
pub const SSF_VOLTAGE_HIGH_WARNING: c_int = 12;
pub const SSF_VOLTAGE_LOW_WARNING: c_int = 14;
pub const SSF_BIAS_HIGH_ALARM: c_int = 16;
pub const SSF_BIAS_LOW_ALARM: c_int = 18;
pub const SSF_BIAS_HIGH_WARNING: c_int = 20;
pub const SSF_BIAS_LOW_WARNING: c_int = 22;
pub const SSF_TXPOWER_HIGH_ALARM: c_int = 24;
pub const SSF_TXPOWER_LOW_ALARM: c_int = 26;
pub const SSF_TXPOWER_HIGH_WARNING: c_int = 28;
pub const SSF_TXPOWER_LOW_WARNING: c_int = 30;
pub const SSF_RXPOWER_HIGH_ALARM: c_int = 32;
pub const SSF_RXPOWER_LOW_ALARM: c_int = 34;
pub const SSF_RXPOWER_HIGH_WARNING: c_int = 36;
pub const SSF_RXPOWER_LOW_WARNING: c_int = 38;
pub const SSF_EXT_CAL_CONSTANTS: c_int = 56;
pub const SSF_CC_DMI: c_int = 95;
pub const SFF_TEMPERATURE_B1: c_int = 96;
pub const SFF_TEMPERATURE_B0: c_int = 97;
pub const SFF_VCC_B1: c_int = 98;
pub const SFF_VCC_B0: c_int = 99;
pub const SFF_TX_BIAS_CURRENT_B1: c_int = 100;
pub const SFF_TX_BIAS_CURRENT_B0: c_int = 101;
pub const SFF_TXPOWER_B1: c_int = 102;
pub const SFF_TXPOWER_B0: c_int = 103;
pub const SFF_RXPOWER_B1: c_int = 104;
pub const SFF_RXPOWER_B0: c_int = 105;
pub const SSF_STATUS_CONTROL: c_int = 110;
pub const SSF_ALARM_FLAGS: c_int = 112;
pub const SSF_WARNING_FLAGS: c_int = 116;
pub const SSF_EXT_TATUS_CONTROL_B1: c_int = 118;
pub const SSF_EXT_TATUS_CONTROL_B0: c_int = 119;
pub const SSF_A2_VENDOR_SPECIFIC: c_int = 120;
pub const SSF_USER_EEPROM: c_int = 128;
pub const SSF_VENDOR_CONTROL: c_int = 148;
//
// Transceiver codes Fibre Channel SFF-8472
// Table 3.5.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte0 {
    pub inifiband:4: u8,
    pub teng_ethernet:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte1 {
    pub sonet:6: u8,
    pub escon:2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte2 {
    pub soNet:8: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte3 {
    pub ethernet:8: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte4 {
    pub fc_el_lo:1: u8,
    pub fc_lw_laser:1: u8,
    pub fc_sw_laser:1: u8,
    pub fc_md_distance:1: u8,
    pub fc_lg_distance:1: u8,
    pub fc_int_distance:1: u8,
    pub fc_short_distance:1: u8,
    pub fc_vld_distance:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte5 {
    pub reserved1:1: u8,
    pub reserved2:1: u8,
    pub /: *mut *mut uint8_t fc_sfp_active:1; / Active cable,
    pub /: *mut *mut uint8_t fc_sfp_passive:1; / Passive cable,
    pub /: *mut *mut uint8_t fc_lw_laser:1; / Longwave laser,
    pub fc_sw_laser_sl:1: u8,
    pub fc_sw_laser_sn:1: u8,
    pub /: *mut *mut uint8_t fc_el_hi:1; / Electrical enclosure high bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte6 {
    pub /: *mut *mut uint8_t fc_tm_sm:1; / Single Mode,
    pub reserved:1: u8,
    pub /: *mut *mut uint8_t fc_tm_m6:1; / Multimode, 62.5um (M6),
    pub /: *mut *mut uint8_t fc_tm_tv:1; / Video Coax (TV),
    pub /: *mut *mut uint8_t fc_tm_mi:1; / Miniature Coax (MI),
    pub /: *mut *mut uint8_t fc_tm_tp:1; / Twisted Pair (TP),
    pub /: *mut *mut uint8_t fc_tm_tw:1; / Twin Axial Pair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_trasnceiver_codes_byte7 {
    pub /: *mut *mut uint8_t fc_sp_100MB:1; / 100 MB/sec,
    pub speed_chk_ecc:1: u8,
    pub /: *mut *mut uint8_t fc_sp_200mb:1; / 200 MB/sec,
    pub /: *mut *mut uint8_t fc_sp_3200MB:1; / 3200 MB/sec,
    pub /: *mut *mut uint8_t fc_sp_400MB:1; / 400 MB/sec,
    pub /: *mut *mut uint8_t fc_sp_1600MB:1; / 1600 MB/sec,
    pub /: *mut *mut uint8_t fc_sp_800MB:1; / 800 MB/sec,
    pub /: *mut *mut uint8_t fc_sp_1200MB:1; / 1200 MB/sec,
}

// User writable non-volatile memory, SFF-8472 Table 3.20
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_eeprom {
    pub vendor_name: [u8; 16],
    pub vendor_oui: [u8; 3],
    pub vendor_pn: [u8; 816],
    pub vendor_rev: [u8; 4],
    pub vendor_sn: [u8; 16],
    pub datecode: [u8; 6],
    pub lot_code: [u8; 2],
    pub reserved191: [u8; 57],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_parameters {
    pub word0: u32,
pub const cfg_prot_type_SHIFT: c_int = 0;
pub const cfg_prot_type_MASK: c_uint = 0x000000FF;

    pub word1: u32,
pub const cfg_ft_SHIFT: c_int = 0;
pub const cfg_ft_MASK: c_uint = 0x00000001;

pub const cfg_sli_rev_SHIFT: c_int = 4;
pub const cfg_sli_rev_MASK: c_uint = 0x0000000f;

pub const cfg_sli_family_SHIFT: c_int = 8;
pub const cfg_sli_family_MASK: c_uint = 0x0000000f;

pub const cfg_if_type_SHIFT: c_int = 12;
pub const cfg_if_type_MASK: c_uint = 0x0000000f;

pub const cfg_sli_hint_1_SHIFT: c_int = 16;
pub const cfg_sli_hint_1_MASK: c_uint = 0x000000ff;

pub const cfg_sli_hint_2_SHIFT: c_int = 24;
pub const cfg_sli_hint_2_MASK: c_uint = 0x0000001f;

    pub word2: u32,
pub const cfg_eqav_SHIFT: c_int = 31;
pub const cfg_eqav_MASK: c_uint = 0x00000001;

    pub word3: u32,
    pub word4: u32,
pub const cfg_cqv_SHIFT: c_int = 14;
pub const cfg_cqv_MASK: c_uint = 0x00000003;

pub const cfg_cqpsize_SHIFT: c_int = 16;
pub const cfg_cqpsize_MASK: c_uint = 0x000000ff;

pub const cfg_cqav_SHIFT: c_int = 31;
pub const cfg_cqav_MASK: c_uint = 0x00000001;

    pub word5: u32,
    pub word6: u32,
pub const cfg_mqv_SHIFT: c_int = 14;
pub const cfg_mqv_MASK: c_uint = 0x00000003;

    pub word7: u32,
    pub word8: u32,
pub const cfg_wqpcnt_SHIFT: c_int = 0;
pub const cfg_wqpcnt_MASK: c_uint = 0x0000000f;

pub const cfg_wqsize_SHIFT: c_int = 8;
pub const cfg_wqsize_MASK: c_uint = 0x0000000f;

pub const cfg_wqv_SHIFT: c_int = 14;
pub const cfg_wqv_MASK: c_uint = 0x00000003;

pub const cfg_wqpsize_SHIFT: c_int = 16;
pub const cfg_wqpsize_MASK: c_uint = 0x000000ff;

    pub word9: u32,
    pub word10: u32,
pub const cfg_rqv_SHIFT: c_int = 14;
pub const cfg_rqv_MASK: c_uint = 0x00000003;

    pub word11: u32,
pub const cfg_rq_db_window_SHIFT: c_int = 28;
pub const cfg_rq_db_window_MASK: c_uint = 0x0000000f;

    pub word12: u32,
pub const cfg_fcoe_SHIFT: c_int = 0;
pub const cfg_fcoe_MASK: c_uint = 0x00000001;

pub const cfg_ext_SHIFT: c_int = 1;
pub const cfg_ext_MASK: c_uint = 0x00000001;

pub const cfg_hdrr_SHIFT: c_int = 2;
pub const cfg_hdrr_MASK: c_uint = 0x00000001;

pub const cfg_phwq_SHIFT: c_int = 15;
pub const cfg_phwq_MASK: c_uint = 0x00000001;

pub const cfg_oas_SHIFT: c_int = 25;
pub const cfg_oas_MASK: c_uint = 0x00000001;

pub const cfg_loopbk_scope_SHIFT: c_int = 28;
pub const cfg_loopbk_scope_MASK: c_uint = 0x0000000f;

    pub sge_supp_len: u32,
    pub word14: u32,
pub const cfg_sgl_page_cnt_SHIFT: c_int = 0;
pub const cfg_sgl_page_cnt_MASK: c_uint = 0x0000000f;

pub const cfg_sgl_page_size_SHIFT: c_int = 8;
pub const cfg_sgl_page_size_MASK: c_uint = 0x000000ff;

pub const cfg_sgl_pp_align_SHIFT: c_int = 16;
pub const cfg_sgl_pp_align_MASK: c_uint = 0x000000ff;

    pub word15: u32,
    pub word16: u32,
    pub word17: u32,
    pub word18: u32,
    pub word19: u32,
pub const cfg_ext_embed_cb_SHIFT: c_int = 0;
pub const cfg_ext_embed_cb_MASK: c_uint = 0x00000001;

pub const cfg_mds_diags_SHIFT: c_int = 1;
pub const cfg_mds_diags_MASK: c_uint = 0x00000001;

pub const cfg_nvme_SHIFT: c_int = 3;
pub const cfg_nvme_MASK: c_uint = 0x00000001;

pub const cfg_xib_SHIFT: c_int = 4;
pub const cfg_xib_MASK: c_uint = 0x00000001;

pub const cfg_xpsgl_SHIFT: c_int = 6;
pub const cfg_xpsgl_MASK: c_uint = 0x00000001;

pub const cfg_eqdr_SHIFT: c_int = 8;
pub const cfg_eqdr_MASK: c_uint = 0x00000001;

pub const cfg_nosr_SHIFT: c_int = 9;
pub const cfg_nosr_MASK: c_uint = 0x00000001;

pub const cfg_bv1s_SHIFT: c_int = 10;
pub const cfg_bv1s_MASK: c_uint = 0x00000001;

pub const cfg_nsler_SHIFT: c_int = 12;
pub const cfg_nsler_MASK: c_uint = 0x00000001;

pub const cfg_pvl_SHIFT: c_int = 13;
pub const cfg_pvl_MASK: c_uint = 0x00000001;

    pub word20: u32,
pub const cfg_max_tow_xri_SHIFT: c_int = 0;
pub const cfg_max_tow_xri_MASK: c_uint = 0x0000ffff;

    pub word21: u32,
pub const cfg_mi_ver_SHIFT: c_int = 0;
pub const cfg_mi_ver_MASK: c_uint = 0x0000ffff;

pub const cfg_cmf_SHIFT: c_int = 24;
pub const cfg_cmf_MASK: c_uint = 0x000000ff;

    pub mib_size: u32,
    pub /: *mut *mut uint32_t word23; / RESERVED,
    pub word24: u32,
pub const cfg_frag_field_offset_SHIFT: c_int = 0;
pub const cfg_frag_field_offset_MASK: c_uint = 0x0000ffff;

pub const cfg_frag_field_size_SHIFT: c_int = 16;
pub const cfg_frag_field_size_MASK: c_uint = 0x0000ffff;

    pub word25: u32,
pub const cfg_sgl_field_offset_SHIFT: c_int = 0;
pub const cfg_sgl_field_offset_MASK: c_uint = 0x0000ffff;

pub const cfg_sgl_field_size_SHIFT: c_int = 16;
pub const cfg_sgl_field_size_MASK: c_uint = 0x0000ffff;

    pub /: *mut *mut uint32_t word26; / Chain SGE initial value LOW,
    pub /: *mut *mut uint32_t word27; / Chain SGE initial value HIGH,
pub const LPFC_NODELAY_MAX_IO: c_int = 32;
}

pub const LPFC_SET_UE_RECOVERY: c_uint = 0x10;
pub const LPFC_SET_MDS_DIAGS: c_uint = 0x12;
pub const LPFC_SET_DUAL_DUMP: c_uint = 0x1e;
pub const LPFC_SET_CGN_SIGNAL: c_uint = 0x1f;
pub const LPFC_SET_ENABLE_MI: c_uint = 0x21;
pub const LPFC_SET_LD_SIGNAL: c_uint = 0x23;
pub const LPFC_SET_ENABLE_CMF: c_uint = 0x24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_feature {
    pub header: mbox_header,
    pub feature: u32,
    pub param_len: u32,
    pub word6: u32,
pub const lpfc_mbx_set_feature_UER_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_UER_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_feature_mds_SHIFT: c_int = 2;
pub const lpfc_mbx_set_feature_mds_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_feature_mds_deep_loopbk_SHIFT: c_int = 1;
pub const lpfc_mbx_set_feature_mds_deep_loopbk_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_feature_CGN_warn_freq_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_CGN_warn_freq_MASK: c_uint = 0x0000ffff;

pub const lpfc_mbx_set_feature_dd_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_dd_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_feature_ddquery_SHIFT: c_int = 1;
pub const lpfc_mbx_set_feature_ddquery_MASK: c_uint = 0x00000001;

pub const LPFC_DISABLE_DUAL_DUMP: c_int = 0;
pub const LPFC_ENABLE_DUAL_DUMP: c_int = 1;
pub const LPFC_QUERY_OP_DUAL_DUMP: c_int = 2;
pub const lpfc_mbx_set_feature_cmf_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_cmf_MASK: c_uint = 0x00000001;

pub const lpfc_mbx_set_feature_lds_qry_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_lds_qry_MASK: c_uint = 0x00000001;

pub const LPFC_QUERY_LDS_OP: c_int = 1;
pub const lpfc_mbx_set_feature_mi_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_mi_MASK: c_uint = 0x0000ffff;

pub const lpfc_mbx_set_feature_milunq_SHIFT: c_int = 16;
pub const lpfc_mbx_set_feature_milunq_MASK: c_uint = 0x0000ffff;

    pub word7: u32,
pub const lpfc_mbx_set_feature_UERP_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_UERP_MASK: c_uint = 0x0000ffff;

pub const lpfc_mbx_set_feature_UESR_SHIFT: c_int = 16;
pub const lpfc_mbx_set_feature_UESR_MASK: c_uint = 0x0000ffff;

pub const lpfc_mbx_set_feature_CGN_alarm_freq_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_CGN_alarm_freq_MASK: c_uint = 0x0000ffff;

    pub word8: u32,
pub const lpfc_mbx_set_feature_CGN_acqe_freq_SHIFT: c_int = 0;
pub const lpfc_mbx_set_feature_CGN_acqe_freq_MASK: c_uint = 0x000000ff;

    pub word9: u32,
    pub word10: u32,
}

pub const LPFC_SET_HOST_OS_DRIVER_VERSION: c_uint = 0x2;
pub const LPFC_SET_HOST_DATE_TIME: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_host_date_time {
    pub word6: u32,

pub const lpfc_mbx_set_host_month_SHIFT: c_int = 16;
pub const lpfc_mbx_set_host_month_MASK: c_uint = 0xFF;

pub const lpfc_mbx_set_host_day_SHIFT: c_int = 8;
pub const lpfc_mbx_set_host_day_MASK: c_uint = 0xFF;

pub const lpfc_mbx_set_host_year_SHIFT: c_int = 0;
pub const lpfc_mbx_set_host_year_MASK: c_uint = 0xFF;
    pub word7: u32,

pub const lpfc_mbx_set_host_hour_SHIFT: c_int = 16;
pub const lpfc_mbx_set_host_hour_MASK: c_uint = 0xFF;

pub const lpfc_mbx_set_host_min_SHIFT: c_int = 8;
pub const lpfc_mbx_set_host_min_MASK: c_uint = 0xFF;

pub const lpfc_mbx_set_host_sec_SHIFT: c_int = 0;
pub const lpfc_mbx_set_host_sec_MASK: c_uint = 0xFF;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_host_data {
pub const LPFC_HOST_OS_DRIVER_VERSION_SIZE: c_int = 48;
    pub header: mbox_header,
    pub param_id: u32,
    pub param_len: u32,
    pub data: [u8; LPFC_HOST_OS_DRIVER_VERSION_SIZE],
    pub tm: lpfc_mbx_set_host_date_time,
    pub un: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_set_trunk_mode {
    pub header: mbox_header,
    pub word0: u32,

pub const lpfc_mbx_set_trunk_mode_SHIFT: c_int = 0;
pub const lpfc_mbx_set_trunk_mode_MASK: c_uint = 0xFF;
    pub word1: u32,
    pub word2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_sli4_parameters {
    pub header: mbox_header,
    pub sli4_parameters: lpfc_sli4_parameters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_reg_congestion_buf {
    pub header: mbox_header,
    pub word0: u32,

pub const lpfc_mbx_reg_cgn_buf_type_SHIFT: c_int = 0;
pub const lpfc_mbx_reg_cgn_buf_type_MASK: c_uint = 0xFF;

pub const lpfc_mbx_reg_cgn_buf_cnt_SHIFT: c_int = 16;
pub const lpfc_mbx_reg_cgn_buf_cnt_MASK: c_uint = 0xFF;
    pub word1: u32,
    pub length: u32,
    pub addr_lo: u32,
    pub addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rscr_desc_generic {
pub const LPFC_RSRC_DESC_WSIZE: c_int = 22;
    pub desc: [u32; LPFC_RSRC_DESC_WSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rsrc_desc_pcie {
    pub word0: u32,
pub const lpfc_rsrc_desc_pcie_type_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_pcie_type_MASK: c_uint = 0x000000ff;

pub const LPFC_RSRC_DESC_TYPE_PCIE: c_uint = 0x40;
pub const lpfc_rsrc_desc_pcie_length_SHIFT: c_int = 8;
pub const lpfc_rsrc_desc_pcie_length_MASK: c_uint = 0x000000ff;

    pub word1: u32,
pub const lpfc_rsrc_desc_pcie_pfnum_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_pcie_pfnum_MASK: c_uint = 0x000000ff;

    pub reserved: u32,
    pub word3: u32,
pub const lpfc_rsrc_desc_pcie_sriov_sta_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_pcie_sriov_sta_MASK: c_uint = 0x000000ff;

pub const lpfc_rsrc_desc_pcie_pf_sta_SHIFT: c_int = 8;
pub const lpfc_rsrc_desc_pcie_pf_sta_MASK: c_uint = 0x000000ff;

pub const lpfc_rsrc_desc_pcie_pf_type_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_pcie_pf_type_MASK: c_uint = 0x000000ff;

    pub word4: u32,
pub const lpfc_rsrc_desc_pcie_nr_virtfn_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_pcie_nr_virtfn_MASK: c_uint = 0x0000ffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rsrc_desc_fcfcoe {
    pub word0: u32,
pub const lpfc_rsrc_desc_fcfcoe_type_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_type_MASK: c_uint = 0x000000ff;

pub const LPFC_RSRC_DESC_TYPE_FCFCOE: c_uint = 0x43;
pub const lpfc_rsrc_desc_fcfcoe_length_SHIFT: c_int = 8;
pub const lpfc_rsrc_desc_fcfcoe_length_MASK: c_uint = 0x000000ff;

pub const LPFC_RSRC_DESC_TYPE_FCFCOE_V0_RSVD: c_int = 0;
pub const LPFC_RSRC_DESC_TYPE_FCFCOE_V0_LENGTH: c_int = 72;
pub const LPFC_RSRC_DESC_TYPE_FCFCOE_V1_LENGTH: c_int = 88;
    pub word1: u32,
pub const lpfc_rsrc_desc_fcfcoe_vfnum_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_vfnum_MASK: c_uint = 0x000000ff;

pub const lpfc_rsrc_desc_fcfcoe_pfnum_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_pfnum_MASK: c_uint = 0x000007ff;

    pub word2: u32,
pub const lpfc_rsrc_desc_fcfcoe_rpi_cnt_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_rpi_cnt_MASK: c_uint = 0x0000ffff;

pub const lpfc_rsrc_desc_fcfcoe_xri_cnt_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_xri_cnt_MASK: c_uint = 0x0000ffff;

    pub word3: u32,
pub const lpfc_rsrc_desc_fcfcoe_wq_cnt_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_wq_cnt_MASK: c_uint = 0x0000ffff;

pub const lpfc_rsrc_desc_fcfcoe_rq_cnt_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_rq_cnt_MASK: c_uint = 0x0000ffff;

    pub word4: u32,
pub const lpfc_rsrc_desc_fcfcoe_cq_cnt_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_cq_cnt_MASK: c_uint = 0x0000ffff;

pub const lpfc_rsrc_desc_fcfcoe_vpi_cnt_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_vpi_cnt_MASK: c_uint = 0x0000ffff;

    pub word5: u32,
pub const lpfc_rsrc_desc_fcfcoe_fcfi_cnt_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_fcfi_cnt_MASK: c_uint = 0x0000ffff;

pub const lpfc_rsrc_desc_fcfcoe_vfi_cnt_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_vfi_cnt_MASK: c_uint = 0x0000ffff;

    pub word6: u32,
    pub word7: u32,
    pub word8: u32,
    pub word9: u32,
    pub word10: u32,
    pub word11: u32,
    pub word12: u32,
    pub word13: u32,
pub const lpfc_rsrc_desc_fcfcoe_lnk_nr_SHIFT: c_int = 0;
pub const lpfc_rsrc_desc_fcfcoe_lnk_nr_MASK: c_uint = 0x0000003f;

pub const lpfc_rsrc_desc_fcfcoe_lnk_tp_SHIFT: c_int = 6;
pub const lpfc_rsrc_desc_fcfcoe_lnk_tp_MASK: c_uint = 0x00000003;

pub const lpfc_rsrc_desc_fcfcoe_lmc_SHIFT: c_int = 8;
pub const lpfc_rsrc_desc_fcfcoe_lmc_MASK: c_uint = 0x00000001;

pub const lpfc_rsrc_desc_fcfcoe_lld_SHIFT: c_int = 9;
pub const lpfc_rsrc_desc_fcfcoe_lld_MASK: c_uint = 0x00000001;

pub const lpfc_rsrc_desc_fcfcoe_eq_cnt_SHIFT: c_int = 16;
pub const lpfc_rsrc_desc_fcfcoe_eq_cnt_MASK: c_uint = 0x0000ffff;

// extended FC/FCoE Resource Descriptor when length = 88 bytes
    pub bw_min: u32,
    pub bw_max: u32,
    pub iops_min: u32,
    pub iops_max: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_func_cfg {
pub const LPFC_RSRC_DESC_MAX_NUM: c_int = 2;
    pub rsrc_desc_count: u32,
    pub desc: [lpfc_rscr_desc_generic; LPFC_RSRC_DESC_MAX_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_func_cfg {
    pub header: mbox_header,
pub const LPFC_CFG_TYPE_PERSISTENT_OVERRIDE: c_uint = 0x0;
pub const LPFC_CFG_TYPE_FACTURY_DEFAULT: c_uint = 0x1;
pub const LPFC_CFG_TYPE_CURRENT_ACTIVE: c_uint = 0x2;
    pub func_cfg: lpfc_func_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_prof_cfg {
pub const LPFC_RSRC_DESC_MAX_NUM: c_int = 2;
    pub rsrc_desc_count: u32,
    pub desc: [lpfc_rscr_desc_generic; LPFC_RSRC_DESC_MAX_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_prof_cfg {
    pub header: mbox_header,
pub const LPFC_CFG_TYPE_PERSISTENT_OVERRIDE: c_uint = 0x0;
pub const LPFC_CFG_TYPE_FACTURY_DEFAULT: c_uint = 0x1;
pub const LPFC_CFG_TYPE_CURRENT_ACTIVE: c_uint = 0x2;
    pub word10: u32,
pub const lpfc_mbx_get_prof_cfg_prof_id_SHIFT: c_int = 0;
pub const lpfc_mbx_get_prof_cfg_prof_id_MASK: c_uint = 0x000000ff;

pub const lpfc_mbx_get_prof_cfg_prof_tp_SHIFT: c_int = 8;
pub const lpfc_mbx_get_prof_cfg_prof_tp_MASK: c_uint = 0x00000003;

    pub request: },
    pub prof_cfg: lpfc_prof_cfg,
    pub response: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_controller_attribute {
    pub version_string: [u32; 8],
    pub manufacturer_name: [u32; 8],
    pub rsvd16: u32,
    pub word17: u32,
pub const lpfc_cntl_attr_flash_id_SHIFT: c_int = 16;
pub const lpfc_cntl_attr_flash_id_MASK: c_uint = 0x000000ff;

pub const lpfc_cntl_attr_boot_enable_SHIFT: c_int = 24;
pub const lpfc_cntl_attr_boot_enable_MASK: c_uint = 0x00000001;
    pub rsvd18: [u32; 2],
    pub ncsi_ver_str: [u32; 3],
    pub rsvd23: u32,
    pub model_number: [u32; 8],
    pub description: [u32; 16],
    pub serial_number: [u32; 8],
    pub ipl_name: [u32; 5],
    pub rsvd61: [u32; 3],
    pub fw_ver_str: [u32; 8],
    pub bios_ver_str: [u32; 8],
    pub redboot_ver_str: [u32; 8],
    pub driver_ver_str: [u32; 8],
    pub flash_fw_ver_str: [u32; 8],
    pub functionality: u32,
    pub word105: u32,
pub const lpfc_cntl_attr_asic_rev_SHIFT: c_int = 16;
pub const lpfc_cntl_attr_asic_rev_MASK: c_uint = 0x000000ff;
    pub rsvd106: [u32; 3],
    pub word109: u32,
pub const lpfc_cntl_attr_hba_port_cnt_SHIFT: c_int = 24;
pub const lpfc_cntl_attr_hba_port_cnt_MASK: c_uint = 0x000000ff;

    pub rsvd110: u32,
    pub word111: u32,
pub const lpfc_cntl_attr_hba_status_SHIFT: c_int = 8;
pub const lpfc_cntl_attr_hba_status_MASK: c_uint = 0x000000ff;

pub const lpfc_cntl_attr_lnk_numb_SHIFT: c_int = 24;
pub const lpfc_cntl_attr_lnk_numb_MASK: c_uint = 0x0000003f;

pub const lpfc_cntl_attr_lnk_type_SHIFT: c_int = 30;
pub const lpfc_cntl_attr_lnk_type_MASK: c_uint = 0x00000003;
    pub rsvd112: [u32; 9],
    pub word121: u32,
pub const lpfc_cntl_attr_asic_gen_SHIFT: c_int = 8;
pub const lpfc_cntl_attr_asic_gen_MASK: c_uint = 0x000000ff;
    pub rsvd122: [u32; 3],
    pub word125: u32,
pub const lpfc_cntl_attr_pci_vendor_id_SHIFT: c_int = 0;
pub const lpfc_cntl_attr_pci_vendor_id_MASK: c_uint = 0x0000ffff;

pub const lpfc_cntl_attr_pci_device_id_SHIFT: c_int = 16;
pub const lpfc_cntl_attr_pci_device_id_MASK: c_uint = 0x0000ffff;

    pub word126: u32,
pub const lpfc_cntl_attr_pci_subvdr_id_SHIFT: c_int = 0;
pub const lpfc_cntl_attr_pci_subvdr_id_MASK: c_uint = 0x0000ffff;

pub const lpfc_cntl_attr_pci_subsys_id_SHIFT: c_int = 16;
pub const lpfc_cntl_attr_pci_subsys_id_MASK: c_uint = 0x0000ffff;

    pub word127: u32,
pub const lpfc_cntl_attr_pci_bus_num_SHIFT: c_int = 0;
pub const lpfc_cntl_attr_pci_bus_num_MASK: c_uint = 0x000000ff;

pub const lpfc_cntl_attr_pci_dev_num_SHIFT: c_int = 8;
pub const lpfc_cntl_attr_pci_dev_num_MASK: c_uint = 0x000000ff;

pub const lpfc_cntl_attr_pci_fnc_num_SHIFT: c_int = 16;
pub const lpfc_cntl_attr_pci_fnc_num_MASK: c_uint = 0x000000ff;
    pub rsvd128: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_cntl_attributes {
    pub cfg_shdr: lpfc_sli4_cfg_shdr,
    pub cntl_attr: lpfc_controller_attribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_get_port_name {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_mbx_get_port_name_lnk_type_SHIFT: c_int = 0;
pub const lpfc_mbx_get_port_name_lnk_type_MASK: c_uint = 0x00000003;

    pub request: },
    pub word4: u32,
pub const lpfc_mbx_get_port_name_name0_SHIFT: c_int = 0;
pub const lpfc_mbx_get_port_name_name0_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_get_port_name_name1_SHIFT: c_int = 8;
pub const lpfc_mbx_get_port_name_name1_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_get_port_name_name2_SHIFT: c_int = 16;
pub const lpfc_mbx_get_port_name_name2_MASK: c_uint = 0x000000FF;

pub const lpfc_mbx_get_port_name_name3_SHIFT: c_int = 24;
pub const lpfc_mbx_get_port_name_name3_MASK: c_uint = 0x000000FF;

pub const LPFC_LINK_NUMBER_0: c_int = 0;
pub const LPFC_LINK_NUMBER_1: c_int = 1;
pub const LPFC_LINK_NUMBER_2: c_int = 2;
pub const LPFC_LINK_NUMBER_3: c_int = 3;
    pub response: },
    pub u: },
}

// Mailbox Completion Queue Error Messages
pub const MB_CQE_STATUS_SUCCESS: c_uint = 0x0;
pub const MB_CQE_STATUS_INSUFFICIENT_PRIVILEGES: c_uint = 0x1;
pub const MB_CQE_STATUS_INVALID_PARAMETER: c_uint = 0x2;
pub const MB_CQE_STATUS_INSUFFICIENT_RESOURCES: c_uint = 0x3;
pub const MB_CEQ_STATUS_QUEUE_FLUSHING: c_uint = 0x4;
pub const MB_CQE_STATUS_DMA_FAILED: c_uint = 0x5;
pub const LPFC_MBX_WR_CONFIG_MAX_BDE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbx_wr_object {
    pub header: mbox_header,
    pub word4: u32,
pub const lpfc_wr_object_eof_SHIFT: c_int = 31;
pub const lpfc_wr_object_eof_MASK: c_uint = 0x00000001;

pub const lpfc_wr_object_eas_SHIFT: c_int = 29;
pub const lpfc_wr_object_eas_MASK: c_uint = 0x00000001;

pub const lpfc_wr_object_write_length_SHIFT: c_int = 0;
pub const lpfc_wr_object_write_length_MASK: c_uint = 0x00FFFFFF;

    pub write_offset: u32,
    pub object_name: [u32; LPFC_MBX_OBJECT_NAME_LEN_DW],
    pub bde_count: u32,
    pub bde: [ulp_bde64; LPFC_MBX_WR_CONFIG_MAX_BDE],
    pub request: },
    pub actual_write_length: u32,
    pub word5: u32,
pub const lpfc_wr_object_change_status_SHIFT: c_int = 0;
pub const lpfc_wr_object_change_status_MASK: c_uint = 0x000000FF;

pub const LPFC_CHANGE_STATUS_NO_RESET_NEEDED: c_uint = 0x00;
pub const LPFC_CHANGE_STATUS_PHYS_DEV_RESET: c_uint = 0x01;
pub const LPFC_CHANGE_STATUS_FW_RESET: c_uint = 0x02;
pub const LPFC_CHANGE_STATUS_PORT_MIGRATION: c_uint = 0x04;
pub const LPFC_CHANGE_STATUS_PCI_RESET: c_uint = 0x05;
pub const lpfc_wr_object_csf_SHIFT: c_int = 8;
pub const lpfc_wr_object_csf_MASK: c_uint = 0x00000001;

    pub response: },
    pub u: },
}

// mailbox queue entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mqe {
    pub word0: u32,
pub const lpfc_mqe_status_SHIFT: c_int = 16;
pub const lpfc_mqe_status_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mqe_command_SHIFT: c_int = 8;
pub const lpfc_mqe_command_MASK: c_uint = 0x000000FF;

    pub 1]: uint32_t mb_words[LPFC_SLI4_MB_WORD_COUNT -,
// sli4 mailbox commands
    pub sli4_config: lpfc_mbx_sli4_config,
    pub init_vfi: lpfc_mbx_init_vfi,
    pub reg_vfi: lpfc_mbx_reg_vfi,
    pub unreg_vfi: lpfc_mbx_reg_vfi,
    pub init_vpi: lpfc_mbx_init_vpi,
    pub resume_rpi: lpfc_mbx_resume_rpi,
    pub read_fcf_tbl: lpfc_mbx_read_fcf_tbl,
    pub add_fcf_entry: lpfc_mbx_add_fcf_tbl_entry,
    pub del_fcf_entry: lpfc_mbx_del_fcf_tbl_entry,
    pub redisc_fcf_tbl: lpfc_mbx_redisc_fcf_tbl,
    pub reg_fcfi: lpfc_mbx_reg_fcfi,
    pub reg_fcfi_mrq: lpfc_mbx_reg_fcfi_mrq,
    pub unreg_fcfi: lpfc_mbx_unreg_fcfi,
    pub mq_create: lpfc_mbx_mq_create,
    pub mq_create_ext: lpfc_mbx_mq_create_ext,
    pub read_object: lpfc_mbx_read_object,
    pub eq_create: lpfc_mbx_eq_create,
    pub eq_delay: lpfc_mbx_modify_eq_delay,
    pub cq_create: lpfc_mbx_cq_create,
    pub cq_create_set: lpfc_mbx_cq_create_set,
    pub wq_create: lpfc_mbx_wq_create,
    pub rq_create: lpfc_mbx_rq_create,
    pub rq_create_v2: lpfc_mbx_rq_create_v2,
    pub mq_destroy: lpfc_mbx_mq_destroy,
    pub eq_destroy: lpfc_mbx_eq_destroy,
    pub cq_destroy: lpfc_mbx_cq_destroy,
    pub wq_destroy: lpfc_mbx_wq_destroy,
    pub rq_destroy: lpfc_mbx_rq_destroy,
    pub rsrc_extent_info: lpfc_mbx_get_rsrc_extent_info,
    pub alloc_rsrc_extents: lpfc_mbx_alloc_rsrc_extents,
    pub dealloc_rsrc_extents: lpfc_mbx_dealloc_rsrc_extents,
    pub post_sgl_pages: lpfc_mbx_post_sgl_pages,
    pub nembed_cmd: lpfc_mbx_nembed_cmd,
    pub read_rev: lpfc_mbx_read_rev,
    pub read_vpi: lpfc_mbx_read_vpi,
    pub rd_config: lpfc_mbx_read_config,
    pub req_ftrs: lpfc_mbx_request_features,
    pub hdr_tmpl: lpfc_mbx_post_hdr_tmpl,
    pub query_fw_cfg: lpfc_mbx_query_fw_config,
    pub beacon_config: lpfc_mbx_set_beacon_config,
    pub get_sli4_parameters: lpfc_mbx_get_sli4_parameters,
    pub reg_congestion_buf: lpfc_mbx_reg_congestion_buf,
    pub link_diag_state: lpfc_mbx_set_link_diag_state,
    pub link_diag_loopback: lpfc_mbx_set_link_diag_loopback,
    pub link_diag_test: lpfc_mbx_run_link_diag_test,
    pub get_func_cfg: lpfc_mbx_get_func_cfg,
    pub get_prof_cfg: lpfc_mbx_get_prof_cfg,
    pub wr_object: lpfc_mbx_wr_object,
    pub get_port_name: lpfc_mbx_get_port_name,
    pub set_feature: lpfc_mbx_set_feature,
    pub mem_dump_type3: lpfc_mbx_memory_dump_type3,
    pub set_host_data: lpfc_mbx_set_host_data,
    pub set_trunk_mode: lpfc_mbx_set_trunk_mode,
    pub nop: lpfc_mbx_nop,
    pub ras_fwlog: lpfc_mbx_set_ras_fwlog,
    pub un: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mcqe {
    pub word0: u32,
pub const lpfc_mcqe_status_SHIFT: c_int = 0;
pub const lpfc_mcqe_status_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mcqe_ext_status_SHIFT: c_int = 16;
pub const lpfc_mcqe_ext_status_MASK: c_uint = 0x0000FFFF;

    pub mcqe_tag0: u32,
    pub mcqe_tag1: u32,
    pub trailer: u32,
pub const lpfc_trailer_valid_SHIFT: c_int = 31;
pub const lpfc_trailer_valid_MASK: c_uint = 0x00000001;

pub const lpfc_trailer_async_SHIFT: c_int = 30;
pub const lpfc_trailer_async_MASK: c_uint = 0x00000001;

pub const lpfc_trailer_hpi_SHIFT: c_int = 29;
pub const lpfc_trailer_hpi_MASK: c_uint = 0x00000001;

pub const lpfc_trailer_completed_SHIFT: c_int = 28;
pub const lpfc_trailer_completed_MASK: c_uint = 0x00000001;

pub const lpfc_trailer_consumed_SHIFT: c_int = 27;
pub const lpfc_trailer_consumed_MASK: c_uint = 0x00000001;

pub const lpfc_trailer_type_SHIFT: c_int = 16;
pub const lpfc_trailer_type_MASK: c_uint = 0x000000FF;

pub const lpfc_trailer_code_SHIFT: c_int = 8;
pub const lpfc_trailer_code_MASK: c_uint = 0x000000FF;

pub const LPFC_TRAILER_CODE_LINK: c_uint = 0x1;
pub const LPFC_TRAILER_CODE_FCOE: c_uint = 0x2;
pub const LPFC_TRAILER_CODE_DCBX: c_uint = 0x3;
pub const LPFC_TRAILER_CODE_GRP5: c_uint = 0x5;
pub const LPFC_TRAILER_CODE_FC: c_uint = 0x10;
pub const LPFC_TRAILER_CODE_SLI: c_uint = 0x11;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_link {
    pub word0: u32,
pub const lpfc_acqe_link_speed_SHIFT: c_int = 24;
pub const lpfc_acqe_link_speed_MASK: c_uint = 0x000000FF;

pub const LPFC_ASYNC_LINK_SPEED_ZERO: c_uint = 0x0;
pub const LPFC_ASYNC_LINK_SPEED_10MBPS: c_uint = 0x1;
pub const LPFC_ASYNC_LINK_SPEED_100MBPS: c_uint = 0x2;
pub const LPFC_ASYNC_LINK_SPEED_1GBPS: c_uint = 0x3;
pub const LPFC_ASYNC_LINK_SPEED_10GBPS: c_uint = 0x4;
pub const LPFC_ASYNC_LINK_SPEED_20GBPS: c_uint = 0x5;
pub const LPFC_ASYNC_LINK_SPEED_25GBPS: c_uint = 0x6;
pub const LPFC_ASYNC_LINK_SPEED_40GBPS: c_uint = 0x7;
pub const LPFC_ASYNC_LINK_SPEED_100GBPS: c_uint = 0x8;
pub const lpfc_acqe_link_duplex_SHIFT: c_int = 16;
pub const lpfc_acqe_link_duplex_MASK: c_uint = 0x000000FF;

pub const LPFC_ASYNC_LINK_DUPLEX_NONE: c_uint = 0x0;
pub const LPFC_ASYNC_LINK_DUPLEX_HALF: c_uint = 0x1;
pub const LPFC_ASYNC_LINK_DUPLEX_FULL: c_uint = 0x2;
pub const lpfc_acqe_link_status_SHIFT: c_int = 8;
pub const lpfc_acqe_link_status_MASK: c_uint = 0x000000FF;

pub const LPFC_ASYNC_LINK_STATUS_DOWN: c_uint = 0x0;
pub const LPFC_ASYNC_LINK_STATUS_UP: c_uint = 0x1;
pub const LPFC_ASYNC_LINK_STATUS_LOGICAL_DOWN: c_uint = 0x2;
pub const LPFC_ASYNC_LINK_STATUS_LOGICAL_UP: c_uint = 0x3;
pub const lpfc_acqe_link_type_SHIFT: c_int = 6;
pub const lpfc_acqe_link_type_MASK: c_uint = 0x00000003;

pub const lpfc_acqe_link_number_SHIFT: c_int = 0;
pub const lpfc_acqe_link_number_MASK: c_uint = 0x0000003F;

    pub word1: u32,
pub const lpfc_acqe_link_fault_SHIFT: c_int = 0;
pub const lpfc_acqe_link_fault_MASK: c_uint = 0x000000FF;

pub const LPFC_ASYNC_LINK_FAULT_NONE: c_uint = 0x0;
pub const LPFC_ASYNC_LINK_FAULT_LOCAL: c_uint = 0x1;
pub const LPFC_ASYNC_LINK_FAULT_REMOTE: c_uint = 0x2;
pub const LPFC_ASYNC_LINK_FAULT_LR_LRR: c_uint = 0x3;
pub const lpfc_acqe_logical_link_speed_SHIFT: c_int = 16;
pub const lpfc_acqe_logical_link_speed_MASK: c_uint = 0x0000FFFF;

    pub event_tag: u32,
    pub trailer: u32,
pub const LPFC_LINK_EVENT_TYPE_PHYSICAL: c_uint = 0x0;
pub const LPFC_LINK_EVENT_TYPE_VIRTUAL: c_uint = 0x1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_fip {
    pub index: u32,
    pub word1: u32,
pub const lpfc_acqe_fip_fcf_count_SHIFT: c_int = 0;
pub const lpfc_acqe_fip_fcf_count_MASK: c_uint = 0x0000FFFF;

pub const lpfc_acqe_fip_event_type_SHIFT: c_int = 16;
pub const lpfc_acqe_fip_event_type_MASK: c_uint = 0x0000FFFF;

    pub event_tag: u32,
    pub trailer: u32,
pub const LPFC_FIP_EVENT_TYPE_NEW_FCF: c_uint = 0x1;
pub const LPFC_FIP_EVENT_TYPE_FCF_TABLE_FULL: c_uint = 0x2;
pub const LPFC_FIP_EVENT_TYPE_FCF_DEAD: c_uint = 0x3;
pub const LPFC_FIP_EVENT_TYPE_CVL: c_uint = 0x4;
pub const LPFC_FIP_EVENT_TYPE_FCF_PARAM_MOD: c_uint = 0x5;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_dcbx {
    pub tlv_ttl: u32,
    pub reserved: u32,
    pub event_tag: u32,
    pub trailer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_grp5 {
    pub word0: u32,
pub const lpfc_acqe_grp5_type_SHIFT: c_int = 6;
pub const lpfc_acqe_grp5_type_MASK: c_uint = 0x00000003;

pub const lpfc_acqe_grp5_number_SHIFT: c_int = 0;
pub const lpfc_acqe_grp5_number_MASK: c_uint = 0x0000003F;

    pub word1: u32,
pub const lpfc_acqe_grp5_llink_spd_SHIFT: c_int = 16;
pub const lpfc_acqe_grp5_llink_spd_MASK: c_uint = 0x0000FFFF;

    pub event_tag: u32,
    pub trailer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_fc_la {
    pub word0: u32,
pub const lpfc_acqe_fc_la_speed_SHIFT: c_int = 24;
pub const lpfc_acqe_fc_la_speed_MASK: c_uint = 0x000000FF;

pub const LPFC_FC_LA_SPEED_UNKNOWN: c_uint = 0x0;
pub const LPFC_FC_LA_SPEED_1G: c_uint = 0x1;
pub const LPFC_FC_LA_SPEED_2G: c_uint = 0x2;
pub const LPFC_FC_LA_SPEED_4G: c_uint = 0x4;
pub const LPFC_FC_LA_SPEED_8G: c_uint = 0x8;
pub const LPFC_FC_LA_SPEED_10G: c_uint = 0xA;
pub const LPFC_FC_LA_SPEED_16G: c_uint = 0x10;
pub const LPFC_FC_LA_SPEED_32G: c_uint = 0x20;
pub const LPFC_FC_LA_SPEED_64G: c_uint = 0x21;
pub const LPFC_FC_LA_SPEED_128G: c_uint = 0x22;
pub const LPFC_FC_LA_SPEED_256G: c_uint = 0x23;
pub const lpfc_acqe_fc_la_topology_SHIFT: c_int = 16;
pub const lpfc_acqe_fc_la_topology_MASK: c_uint = 0x000000FF;

pub const LPFC_FC_LA_TOP_UNKOWN: c_uint = 0x0;
pub const LPFC_FC_LA_TOP_P2P: c_uint = 0x1;
pub const LPFC_FC_LA_TOP_FCAL: c_uint = 0x2;
pub const LPFC_FC_LA_TOP_INTERNAL_LOOP: c_uint = 0x3;
pub const LPFC_FC_LA_TOP_SERDES_LOOP: c_uint = 0x4;
pub const lpfc_acqe_fc_la_att_type_SHIFT: c_int = 8;
pub const lpfc_acqe_fc_la_att_type_MASK: c_uint = 0x000000FF;

pub const LPFC_FC_LA_TYPE_LINK_UP: c_uint = 0x1;
pub const LPFC_FC_LA_TYPE_LINK_DOWN: c_uint = 0x2;
pub const LPFC_FC_LA_TYPE_NO_HARD_ALPA: c_uint = 0x3;
pub const LPFC_FC_LA_TYPE_MDS_LINK_DOWN: c_uint = 0x4;
pub const LPFC_FC_LA_TYPE_MDS_LOOPBACK: c_uint = 0x5;
pub const LPFC_FC_LA_TYPE_UNEXP_WWPN: c_uint = 0x6;
pub const LPFC_FC_LA_TYPE_TRUNKING_EVENT: c_uint = 0x7;
pub const LPFC_FC_LA_TYPE_ACTIVATE_FAIL: c_uint = 0x8;
pub const LPFC_FC_LA_TYPE_LINK_RESET_PRTCL_EVT: c_uint = 0x9;
pub const lpfc_acqe_fc_la_port_type_SHIFT: c_int = 6;
pub const lpfc_acqe_fc_la_port_type_MASK: c_uint = 0x00000003;

pub const LPFC_LINK_TYPE_ETHERNET: c_uint = 0x0;
pub const LPFC_LINK_TYPE_FC: c_uint = 0x1;
pub const lpfc_acqe_fc_la_port_number_SHIFT: c_int = 0;
pub const lpfc_acqe_fc_la_port_number_MASK: c_uint = 0x0000003F;

// Attention Type is 0x07 (Trunking Event) word0
pub const lpfc_acqe_fc_la_trunk_link_status_port0_SHIFT: c_int = 16;
pub const lpfc_acqe_fc_la_trunk_link_status_port0_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_link_status_port1_SHIFT: c_int = 17;
pub const lpfc_acqe_fc_la_trunk_link_status_port1_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_link_status_port2_SHIFT: c_int = 18;
pub const lpfc_acqe_fc_la_trunk_link_status_port2_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_link_status_port3_SHIFT: c_int = 19;
pub const lpfc_acqe_fc_la_trunk_link_status_port3_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_config_port0_SHIFT: c_int = 20;
pub const lpfc_acqe_fc_la_trunk_config_port0_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_config_port1_SHIFT: c_int = 21;
pub const lpfc_acqe_fc_la_trunk_config_port1_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_config_port2_SHIFT: c_int = 22;
pub const lpfc_acqe_fc_la_trunk_config_port2_MASK: c_uint = 0x0000001;

pub const lpfc_acqe_fc_la_trunk_config_port3_SHIFT: c_int = 23;
pub const lpfc_acqe_fc_la_trunk_config_port3_MASK: c_uint = 0x0000001;

    pub word1: u32,
pub const lpfc_acqe_fc_la_llink_spd_SHIFT: c_int = 16;
pub const lpfc_acqe_fc_la_llink_spd_MASK: c_uint = 0x0000FFFF;

pub const lpfc_acqe_fc_la_fault_SHIFT: c_int = 0;
pub const lpfc_acqe_fc_la_fault_MASK: c_uint = 0x000000FF;

pub const lpfc_acqe_fc_la_link_status_SHIFT: c_int = 8;
pub const lpfc_acqe_fc_la_link_status_MASK: c_uint = 0x0000007F;

pub const lpfc_acqe_fc_la_trunk_fault_SHIFT: c_int = 0;
pub const lpfc_acqe_fc_la_trunk_fault_MASK: c_uint = 0x0000000F;

pub const lpfc_acqe_fc_la_trunk_linkmask_SHIFT: c_int = 4;
pub const lpfc_acqe_fc_la_trunk_linkmask_MASK: c_uint = 0x000000F;

pub const LPFC_FC_LA_FAULT_NONE: c_uint = 0x0;
pub const LPFC_FC_LA_FAULT_LOCAL: c_uint = 0x1;
pub const LPFC_FC_LA_FAULT_REMOTE: c_uint = 0x2;
    pub event_tag: u32,
    pub trailer: u32,
pub const LPFC_FC_LA_EVENT_TYPE_FC_LINK: c_uint = 0x1;
pub const LPFC_FC_LA_EVENT_TYPE_SHARED_LINK: c_uint = 0x2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_misconfigured_event {
    pub word0: u32,
pub const lpfc_sli_misconfigured_port0_state_SHIFT: c_int = 0;
pub const lpfc_sli_misconfigured_port0_state_MASK: c_uint = 0x000000FF;

pub const lpfc_sli_misconfigured_port1_state_SHIFT: c_int = 8;
pub const lpfc_sli_misconfigured_port1_state_MASK: c_uint = 0x000000FF;

pub const lpfc_sli_misconfigured_port2_state_SHIFT: c_int = 16;
pub const lpfc_sli_misconfigured_port2_state_MASK: c_uint = 0x000000FF;

pub const lpfc_sli_misconfigured_port3_state_SHIFT: c_int = 24;
pub const lpfc_sli_misconfigured_port3_state_MASK: c_uint = 0x000000FF;

    pub word1: u32,
pub const lpfc_sli_misconfigured_port0_op_SHIFT: c_int = 0;
pub const lpfc_sli_misconfigured_port0_op_MASK: c_uint = 0x00000001;

pub const lpfc_sli_misconfigured_port0_severity_SHIFT: c_int = 1;
pub const lpfc_sli_misconfigured_port0_severity_MASK: c_uint = 0x00000003;

pub const lpfc_sli_misconfigured_port1_op_SHIFT: c_int = 8;
pub const lpfc_sli_misconfigured_port1_op_MASK: c_uint = 0x00000001;

pub const lpfc_sli_misconfigured_port1_severity_SHIFT: c_int = 9;
pub const lpfc_sli_misconfigured_port1_severity_MASK: c_uint = 0x00000003;

pub const lpfc_sli_misconfigured_port2_op_SHIFT: c_int = 16;
pub const lpfc_sli_misconfigured_port2_op_MASK: c_uint = 0x00000001;

pub const lpfc_sli_misconfigured_port2_severity_SHIFT: c_int = 17;
pub const lpfc_sli_misconfigured_port2_severity_MASK: c_uint = 0x00000003;

pub const lpfc_sli_misconfigured_port3_op_SHIFT: c_int = 24;
pub const lpfc_sli_misconfigured_port3_op_MASK: c_uint = 0x00000001;

pub const lpfc_sli_misconfigured_port3_severity_SHIFT: c_int = 25;
pub const lpfc_sli_misconfigured_port3_severity_MASK: c_uint = 0x00000003;

    pub theEvent: },
pub const LPFC_SLI_EVENT_STATUS_VALID: c_uint = 0x00;
pub const LPFC_SLI_EVENT_STATUS_NOT_PRESENT: c_uint = 0x01;
pub const LPFC_SLI_EVENT_STATUS_WRONG_TYPE: c_uint = 0x02;
pub const LPFC_SLI_EVENT_STATUS_UNSUPPORTED: c_uint = 0x03;
pub const LPFC_SLI_EVENT_STATUS_UNQUALIFIED: c_uint = 0x04;
pub const LPFC_SLI_EVENT_STATUS_UNCERTIFIED: c_uint = 0x05;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_cgn_signal {
    pub word0: u32,
pub const lpfc_warn_acqe_SHIFT: c_int = 0;
pub const lpfc_warn_acqe_MASK: c_uint = 0x7FFFFFFF;

pub const lpfc_imm_acqe_SHIFT: c_int = 31;
pub const lpfc_imm_acqe_MASK: c_uint = 0x1;

    pub alarm_cnt: u32,
    pub word2: u32,
    pub trailer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_acqe_sli {
    pub event_data1: u32,
    pub event_data2: u32,
    pub event_data3: u32,
    pub trailer: u32,
pub const LPFC_SLI_EVENT_TYPE_PORT_ERROR: c_uint = 0x1;
pub const LPFC_SLI_EVENT_TYPE_OVER_TEMP: c_uint = 0x2;
pub const LPFC_SLI_EVENT_TYPE_NORM_TEMP: c_uint = 0x3;
pub const LPFC_SLI_EVENT_TYPE_NVLOG_POST: c_uint = 0x4;
pub const LPFC_SLI_EVENT_TYPE_DIAG_DUMP: c_uint = 0x5;
pub const LPFC_SLI_EVENT_TYPE_MISCONFIGURED: c_uint = 0x9;
pub const LPFC_SLI_EVENT_TYPE_REMOTE_DPORT: c_uint = 0xA;
pub const LPFC_SLI_EVENT_TYPE_PORT_PARAMS_CHG: c_uint = 0xE;
pub const LPFC_SLI_EVENT_TYPE_MISCONF_FAWWN: c_uint = 0xF;
pub const LPFC_SLI_EVENT_TYPE_EEPROM_FAILURE: c_uint = 0x10;
pub const LPFC_SLI_EVENT_TYPE_CGN_SIGNAL: c_uint = 0x11;
pub const LPFC_SLI_EVENT_TYPE_RD_SIGNAL: c_uint = 0x12;
pub const LPFC_SLI_EVENT_TYPE_RESET_CM_STATS: c_uint = 0x13;
}

//
// Define the bootstrap mailbox (bmbx) region used to communicate
// mailbox command between the host and port. The mailbox consists
// of a payload area of 256 bytes and a completion queue of length
// 16 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bmbx_create {
    pub mqe: lpfc_mqe,
    pub mcqe: lpfc_mcqe,
}

pub const SGL_ALIGN_SZ: c_int = 64;
pub const SGL_PAGE_SIZE: c_int = 4096;
// align SGL addr on a size boundary - adjust address up
pub const NO_XRI: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wqe_common {
    pub word6: u32,
pub const wqe_xri_tag_SHIFT: c_int = 0;
pub const wqe_xri_tag_MASK: c_uint = 0x0000FFFF;

pub const wqe_ctxt_tag_SHIFT: c_int = 16;
pub const wqe_ctxt_tag_MASK: c_uint = 0x0000FFFF;

    pub word7: u32,
pub const wqe_dif_SHIFT: c_int = 0;
pub const wqe_dif_MASK: c_uint = 0x00000003;

pub const LPFC_WQE_DIF_PASSTHRU: c_int = 1;
pub const LPFC_WQE_DIF_STRIP: c_int = 2;
pub const LPFC_WQE_DIF_INSERT: c_int = 3;
pub const wqe_ct_SHIFT: c_int = 2;
pub const wqe_ct_MASK: c_uint = 0x00000003;

pub const wqe_status_SHIFT: c_int = 4;
pub const wqe_status_MASK: c_uint = 0x0000000f;

pub const wqe_cmnd_SHIFT: c_int = 8;
pub const wqe_cmnd_MASK: c_uint = 0x000000ff;

pub const wqe_class_SHIFT: c_int = 16;
pub const wqe_class_MASK: c_uint = 0x00000007;

pub const wqe_ar_SHIFT: c_int = 19;
pub const wqe_ar_MASK: c_uint = 0x00000001;

pub const wqe_pu_SHIFT: c_int = 20;
pub const wqe_pu_MASK: c_uint = 0x00000003;

pub const wqe_erp_SHIFT: c_int = 22;
pub const wqe_erp_MASK: c_uint = 0x00000001;

pub const wqe_lnk_SHIFT: c_int = 23;
pub const wqe_lnk_MASK: c_uint = 0x00000001;

pub const wqe_tmo_SHIFT: c_int = 24;
pub const wqe_tmo_MASK: c_uint = 0x000000ff;

    pub /: *mut *mut uint32_t abort_tag; / word 8 in WQE,
    pub word9: u32,
pub const wqe_reqtag_SHIFT: c_int = 0;
pub const wqe_reqtag_MASK: c_uint = 0x0000FFFF;

pub const wqe_temp_rpi_SHIFT: c_int = 16;
pub const wqe_temp_rpi_MASK: c_uint = 0x0000FFFF;

pub const wqe_rcvoxid_SHIFT: c_int = 16;
pub const wqe_rcvoxid_MASK: c_uint = 0x0000FFFF;

pub const wqe_sof_SHIFT: c_int = 24;
pub const wqe_sof_MASK: c_uint = 0x000000FF;

pub const wqe_eof_SHIFT: c_int = 16;
pub const wqe_eof_MASK: c_uint = 0x000000FF;

    pub word10: u32,
pub const wqe_ebde_cnt_SHIFT: c_int = 0;
pub const wqe_ebde_cnt_MASK: c_uint = 0x0000000f;

pub const wqe_xchg_SHIFT: c_int = 4;
pub const wqe_xchg_MASK: c_uint = 0x00000001;

pub const LPFC_SCSI_XCHG: c_uint = 0x0;
pub const LPFC_NVME_XCHG: c_uint = 0x1;
pub const wqe_appid_SHIFT: c_int = 5;
pub const wqe_appid_MASK: c_uint = 0x00000001;

pub const wqe_oas_SHIFT: c_int = 6;
pub const wqe_oas_MASK: c_uint = 0x00000001;

pub const wqe_lenloc_SHIFT: c_int = 7;
pub const wqe_lenloc_MASK: c_uint = 0x00000003;

pub const LPFC_WQE_LENLOC_NONE: c_int = 0;
pub const LPFC_WQE_LENLOC_WORD3: c_int = 1;
pub const LPFC_WQE_LENLOC_WORD12: c_int = 2;
pub const LPFC_WQE_LENLOC_WORD4: c_int = 3;
pub const wqe_qosd_SHIFT: c_int = 9;
pub const wqe_qosd_MASK: c_uint = 0x00000001;

pub const wqe_xbl_SHIFT: c_int = 11;
pub const wqe_xbl_MASK: c_uint = 0x00000001;

pub const wqe_iod_SHIFT: c_int = 13;
pub const wqe_iod_MASK: c_uint = 0x00000001;

pub const LPFC_WQE_IOD_NONE: c_int = 0;
pub const LPFC_WQE_IOD_WRITE: c_int = 0;
pub const LPFC_WQE_IOD_READ: c_int = 1;
pub const wqe_dbde_SHIFT: c_int = 14;
pub const wqe_dbde_MASK: c_uint = 0x00000001;

pub const wqe_wqes_SHIFT: c_int = 15;
pub const wqe_wqes_MASK: c_uint = 0x00000001;

// Note that this field overlaps above fields
pub const wqe_wqid_SHIFT: c_int = 1;
pub const wqe_wqid_MASK: c_uint = 0x00007fff;

pub const wqe_pri_SHIFT: c_int = 16;
pub const wqe_pri_MASK: c_uint = 0x00000007;

pub const wqe_pv_SHIFT: c_int = 19;
pub const wqe_pv_MASK: c_uint = 0x00000001;

pub const wqe_xc_SHIFT: c_int = 21;
pub const wqe_xc_MASK: c_uint = 0x00000001;

pub const wqe_sr_SHIFT: c_int = 22;
pub const wqe_sr_MASK: c_uint = 0x00000001;

pub const wqe_ccpe_SHIFT: c_int = 23;
pub const wqe_ccpe_MASK: c_uint = 0x00000001;

pub const wqe_ccp_SHIFT: c_int = 24;
pub const wqe_ccp_MASK: c_uint = 0x000000ff;

    pub word11: u32,
pub const wqe_cmd_type_SHIFT: c_int = 0;
pub const wqe_cmd_type_MASK: c_uint = 0x0000000f;

pub const wqe_els_id_SHIFT: c_int = 4;
pub const wqe_els_id_MASK: c_uint = 0x00000007;

pub const wqe_irsp_SHIFT: c_int = 4;
pub const wqe_irsp_MASK: c_uint = 0x00000001;

pub const wqe_sup_SHIFT: c_int = 6;
pub const wqe_sup_MASK: c_uint = 0x00000001;

pub const wqe_ffrq_SHIFT: c_int = 6;
pub const wqe_ffrq_MASK: c_uint = 0x00000001;

pub const wqe_wqec_SHIFT: c_int = 7;
pub const wqe_wqec_MASK: c_uint = 0x00000001;

pub const wqe_irsplen_SHIFT: c_int = 8;
pub const wqe_irsplen_MASK: c_uint = 0x0000000f;

pub const wqe_cqid_SHIFT: c_int = 16;
pub const wqe_cqid_MASK: c_uint = 0x0000ffff;

pub const LPFC_WQE_CQ_ID_DEFAULT: c_uint = 0xffff;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wqe_did {
    pub word5: u32,
pub const wqe_els_did_SHIFT: c_int = 0;
pub const wqe_els_did_MASK: c_uint = 0x00FFFFFF;

pub const wqe_xmit_bls_pt_SHIFT: c_int = 28;
pub const wqe_xmit_bls_pt_MASK: c_uint = 0x00000003;

pub const wqe_xmit_bls_ar_SHIFT: c_int = 30;
pub const wqe_xmit_bls_ar_MASK: c_uint = 0x00000001;

pub const wqe_xmit_bls_xo_SHIFT: c_int = 31;
pub const wqe_xmit_bls_xo_MASK: c_uint = 0x00000001;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_wqe_generic {
    pub bde: ulp_bde64,
    pub word3: u32,
    pub word4: u32,
    pub word5: u32,
    pub wqe_com: wqe_common,
    pub payload: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum els_request64_wqe_word11 {
    LPFC_ELS_ID_DEFAULT,
    LPFC_ELS_ID_LOGO,
    LPFC_ELS_ID_FDISC,
    LPFC_ELS_ID_FLOGI,
    LPFC_ELS_ID_PLOGI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_request64_wqe {
    pub bde: ulp_bde64,
    pub payload_len: u32,
    pub word4: u32,
pub const els_req64_sid_SHIFT: c_int = 0;
pub const els_req64_sid_MASK: c_uint = 0x00FFFFFF;

pub const els_req64_sp_SHIFT: c_int = 24;
pub const els_req64_sp_MASK: c_uint = 0x00000001;

pub const els_req64_vf_SHIFT: c_int = 25;
pub const els_req64_vf_MASK: c_uint = 0x00000001;

    pub wqe_dest: wqe_did,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub word12: u32,
pub const els_req64_vfid_SHIFT: c_int = 1;
pub const els_req64_vfid_MASK: c_uint = 0x00000FFF;

pub const els_req64_pri_SHIFT: c_int = 13;
pub const els_req64_pri_MASK: c_uint = 0x00000007;

    pub word13: u32,
pub const els_req64_hopcnt_SHIFT: c_int = 24;
pub const els_req64_hopcnt_MASK: c_uint = 0x000000ff;

    pub word14: u32,
    pub max_response_payload_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmit_els_rsp64_wqe {
    pub bde: ulp_bde64,
    pub response_payload_len: u32,
    pub word4: u32,
pub const els_rsp64_sid_SHIFT: c_int = 0;
pub const els_rsp64_sid_MASK: c_uint = 0x00FFFFFF;

pub const els_rsp64_sp_SHIFT: c_int = 24;
pub const els_rsp64_sp_MASK: c_uint = 0x00000001;

    pub wqe_dest: wqe_did,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub word12: u32,
pub const wqe_rsp_temp_rpi_SHIFT: c_int = 0;
pub const wqe_rsp_temp_rpi_MASK: c_uint = 0x0000FFFF;
    pub rsvd_13_15: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmit_bls_rsp64_wqe {
    pub payload0: u32,
// Payload0 for BA_ACC
pub const xmit_bls_rsp64_acc_seq_id_SHIFT: c_int = 16;
pub const xmit_bls_rsp64_acc_seq_id_MASK: c_uint = 0x000000ff;

pub const xmit_bls_rsp64_acc_seq_id_vald_SHIFT: c_int = 24;
pub const xmit_bls_rsp64_acc_seq_id_vald_MASK: c_uint = 0x000000ff;

// Payload0 for BA_RJT
pub const xmit_bls_rsp64_rjt_vspec_SHIFT: c_int = 0;
pub const xmit_bls_rsp64_rjt_vspec_MASK: c_uint = 0x000000ff;

pub const xmit_bls_rsp64_rjt_expc_SHIFT: c_int = 8;
pub const xmit_bls_rsp64_rjt_expc_MASK: c_uint = 0x000000ff;

pub const xmit_bls_rsp64_rjt_rsnc_SHIFT: c_int = 16;
pub const xmit_bls_rsp64_rjt_rsnc_MASK: c_uint = 0x000000ff;

    pub word1: u32,
pub const xmit_bls_rsp64_rxid_SHIFT: c_int = 0;
pub const xmit_bls_rsp64_rxid_MASK: c_uint = 0x0000ffff;

pub const xmit_bls_rsp64_oxid_SHIFT: c_int = 16;
pub const xmit_bls_rsp64_oxid_MASK: c_uint = 0x0000ffff;

    pub word2: u32,
pub const xmit_bls_rsp64_seqcnthi_SHIFT: c_int = 0;
pub const xmit_bls_rsp64_seqcnthi_MASK: c_uint = 0x0000ffff;

pub const xmit_bls_rsp64_seqcntlo_SHIFT: c_int = 16;
pub const xmit_bls_rsp64_seqcntlo_MASK: c_uint = 0x0000ffff;

    pub rsrvd3: u32,
    pub rsrvd4: u32,
    pub wqe_dest: wqe_did,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub word12: u32,
pub const xmit_bls_rsp64_temprpi_SHIFT: c_int = 0;
pub const xmit_bls_rsp64_temprpi_MASK: c_uint = 0x0000ffff;
    pub rsvd_13_15: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wqe_rctl_dfctl {
    pub word5: u32,
pub const wqe_si_SHIFT: c_int = 2;
pub const wqe_si_MASK: c_uint = 0x000000001;

pub const wqe_la_SHIFT: c_int = 3;
pub const wqe_la_MASK: c_uint = 0x000000001;

pub const wqe_xo_SHIFT: c_int = 6;
pub const wqe_xo_MASK: c_uint = 0x000000001;

pub const wqe_ls_SHIFT: c_int = 7;
pub const wqe_ls_MASK: c_uint = 0x000000001;

pub const wqe_dfctl_SHIFT: c_int = 8;
pub const wqe_dfctl_MASK: c_uint = 0x0000000ff;

pub const wqe_type_SHIFT: c_int = 16;
pub const wqe_type_MASK: c_uint = 0x0000000ff;

pub const wqe_rctl_SHIFT: c_int = 24;
pub const wqe_rctl_MASK: c_uint = 0x0000000ff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmit_seq64_wqe {
    pub bde: ulp_bde64,
    pub rsvd3: u32,
    pub relative_offset: u32,
    pub wge_ctl: wqe_rctl_dfctl,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub xmit_len: u32,
    pub rsvd_12_15: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmit_bcast64_wqe {
    pub bde: ulp_bde64,
    pub seq_payload_len: u32,
    pub rsvd4: u32,
    pub /: *mut *mut wqe_rctl_dfctl wge_ctl; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub rsvd_12_15: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen_req64_wqe {
    pub bde: ulp_bde64,
    pub request_payload_len: u32,
    pub relative_offset: u32,
    pub /: *mut *mut wqe_rctl_dfctl wge_ctl; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub rsvd_12_14: [u32; 3],
    pub max_response_payload_len: u32,
}

// Define NVME PRLI request to fabric. NVME is a
// fabric-only protocol.
// Updated to red-lined v1.08 on Sept 16, 2016
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvme_prli {
    pub word1: u32,
// The Response Code is defined in the FCP PRLI lpfc_hw.h
pub const prli_acc_rsp_code_SHIFT: c_int = 8;
pub const prli_acc_rsp_code_MASK: c_uint = 0x0000000f;

pub const prli_estabImagePair_SHIFT: c_int = 13;
pub const prli_estabImagePair_MASK: c_uint = 0x00000001;

pub const prli_type_code_ext_SHIFT: c_int = 16;
pub const prli_type_code_ext_MASK: c_uint = 0x000000ff;

pub const prli_type_code_SHIFT: c_int = 24;
pub const prli_type_code_MASK: c_uint = 0x000000ff;

    pub word_rsvd2: u32,
    pub word_rsvd3: u32,
    pub word4: u32,
pub const prli_fba_SHIFT: c_int = 0;
pub const prli_fba_MASK: c_uint = 0x00000001;

pub const prli_disc_SHIFT: c_int = 3;
pub const prli_disc_MASK: c_uint = 0x00000001;

pub const prli_tgt_SHIFT: c_int = 4;
pub const prli_tgt_MASK: c_uint = 0x00000001;

pub const prli_init_SHIFT: c_int = 5;
pub const prli_init_MASK: c_uint = 0x00000001;

pub const prli_conf_SHIFT: c_int = 7;
pub const prli_conf_MASK: c_uint = 0x00000001;

pub const prli_nsler_SHIFT: c_int = 8;
pub const prli_nsler_MASK: c_uint = 0x00000001;

    pub word5: u32,
pub const prli_fb_sz_SHIFT: c_int = 0;
pub const prli_fb_sz_MASK: c_uint = 0x0000ffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_xri_wqe {
    pub /: *mut *mut uint32_t rsrvd[5]; / words 0-4,
    pub /: *mut *mut wqe_did wqe_dest; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t rsvd_12_15[4]; / word 12-15,
}

pub const T_REQUEST_TAG: c_int = 3;
pub const T_XRI_TAG: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmf_sync_wqe {
    pub rsrvd: [u32; 3],
    pub word3: u32,
pub const cmf_sync_interval_SHIFT: c_int = 0;
pub const cmf_sync_interval_MASK: c_uint = 0x00000ffff;

pub const cmf_sync_afpin_SHIFT: c_int = 16;
pub const cmf_sync_afpin_MASK: c_uint = 0x000000001;

pub const cmf_sync_asig_SHIFT: c_int = 17;
pub const cmf_sync_asig_MASK: c_uint = 0x000000001;

pub const cmf_sync_op_SHIFT: c_int = 20;
pub const cmf_sync_op_MASK: c_uint = 0x00000000f;

pub const cmf_sync_ver_SHIFT: c_int = 24;
pub const cmf_sync_ver_MASK: c_uint = 0x0000000ff;

pub const LPFC_CMF_SYNC_VER: c_int = 1;
    pub event_tag: u32,
    pub word5: u32,
pub const cmf_sync_wsigmax_SHIFT: c_int = 0;
pub const cmf_sync_wsigmax_MASK: c_uint = 0x00000ffff;

pub const cmf_sync_wsigcnt_SHIFT: c_int = 16;
pub const cmf_sync_wsigcnt_MASK: c_uint = 0x00000ffff;

    pub word6: u32,
    pub word7: u32,
pub const cmf_sync_cmnd_SHIFT: c_int = 8;
pub const cmf_sync_cmnd_MASK: c_uint = 0x0000000ff;

    pub word8: u32,
    pub word9: u32,
pub const cmf_sync_reqtag_SHIFT: c_int = 0;
pub const cmf_sync_reqtag_MASK: c_uint = 0x00000ffff;

pub const cmf_sync_wfpinmax_SHIFT: c_int = 16;
pub const cmf_sync_wfpinmax_MASK: c_uint = 0x0000000ff;

pub const cmf_sync_wfpincnt_SHIFT: c_int = 24;
pub const cmf_sync_wfpincnt_MASK: c_uint = 0x0000000ff;

    pub word10: u32,
pub const cmf_sync_qosd_SHIFT: c_int = 9;
pub const cmf_sync_qosd_MASK: c_uint = 0x00000001;

    pub word11: u32,
pub const cmf_sync_cmd_type_SHIFT: c_int = 0;
pub const cmf_sync_cmd_type_MASK: c_uint = 0x0000000f;

pub const cmf_sync_wqec_SHIFT: c_int = 7;
pub const cmf_sync_wqec_MASK: c_uint = 0x00000001;

pub const cmf_sync_cqid_SHIFT: c_int = 16;
pub const cmf_sync_cqid_MASK: c_uint = 0x0000ffff;

    pub read_bytes: u32,
    pub word13: u32,
pub const cmf_sync_period_SHIFT: c_int = 24;
pub const cmf_sync_period_MASK: c_uint = 0x000000ff;

    pub word14: u32,
    pub word15: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_cmd_wqe {
    pub rsrvd: [u32; 3],
    pub word3: u32,
pub const abort_cmd_ia_SHIFT: c_int = 0;
pub const abort_cmd_ia_MASK: c_uint = 0x000000001;

pub const abort_cmd_criteria_SHIFT: c_int = 8;
pub const abort_cmd_criteria_MASK: c_uint = 0x0000000ff;

    pub rsrvd4: u32,
    pub rsrvd5: u32,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t rsvd_12_15[4]; / word 12-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_iwrite64_wqe {
    pub bde: ulp_bde64,
    pub word3: u32,
pub const cmd_buff_len_SHIFT: c_int = 16;
pub const cmd_buff_len_MASK: c_uint = 0x00000ffff;

// Note: payload_offset_len field depends on ASIC support
pub const payload_offset_len_SHIFT: c_int = 0;
pub const payload_offset_len_MASK: c_uint = 0x0000ffff;

    pub total_xfer_len: u32,
    pub initial_xfer_len: u32,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub rsrvd12: u32,
    pub /: *mut *mut ulp_bde64 ph_bde; / words 13-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_iread64_wqe {
    pub bde: ulp_bde64,
    pub word3: u32,
pub const cmd_buff_len_SHIFT: c_int = 16;
pub const cmd_buff_len_MASK: c_uint = 0x00000ffff;

// Note: payload_offset_len field depends on ASIC support
pub const payload_offset_len_SHIFT: c_int = 0;
pub const payload_offset_len_MASK: c_uint = 0x0000ffff;

    pub /: *mut *mut uint32_t total_xfer_len; / word 4,
    pub /: *mut *mut uint32_t rsrvd5; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub rsrvd12: u32,
    pub /: *mut *mut ulp_bde64 ph_bde; / words 13-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_icmnd64_wqe {
    pub /: *mut *mut ulp_bde64 bde; / words 0-2,
    pub word3: u32,
pub const cmd_buff_len_SHIFT: c_int = 16;
pub const cmd_buff_len_MASK: c_uint = 0x00000ffff;

// Note: payload_offset_len field depends on ASIC support
pub const payload_offset_len_SHIFT: c_int = 0;
pub const payload_offset_len_MASK: c_uint = 0x0000ffff;

    pub /: *mut *mut uint32_t rsrvd4; / word 4,
    pub /: *mut *mut uint32_t rsrvd5; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t rsvd_12_15[4]; / word 12-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_trsp64_wqe {
    pub bde: ulp_bde64,
    pub response_len: u32,
    pub rsvd_4_5: [u32; 2],
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t rsvd_12_15[4]; / word 12-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_tsend64_wqe {
    pub bde: ulp_bde64,
    pub payload_offset_len: u32,
    pub relative_offset: u32,
    pub reserved: u32,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t fcp_data_len; / word 12,
    pub /: *mut *mut uint32_t rsvd_13_15[3]; / word 13-15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_treceive64_wqe {
    pub bde: ulp_bde64,
    pub payload_offset_len: u32,
    pub relative_offset: u32,
    pub reserved: u32,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t fcp_data_len; / word 12,
    pub /: *mut *mut uint32_t rsvd_13_15[3]; / word 13-15,
}

pub const TXRDY_PAYLOAD_LEN: c_int = 12;
pub const CMD_SEND_FRAME: c_uint = 0xE1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_frame_wqe {
    pub /: *mut *mut ulp_bde64 bde; / words 0-2,
    pub /: *mut *mut uint32_t frame_len; / word 3,
    pub /: *mut *mut uint32_t fc_hdr_wd0; / word 4,
    pub /: *mut *mut uint32_t fc_hdr_wd1; / word 5,
    pub /: *mut *mut wqe_common wqe_com; / words 6-11,
    pub /: *mut *mut uint32_t fc_hdr_wd2; / word 12,
    pub /: *mut *mut uint32_t fc_hdr_wd3; / word 13,
    pub /: *mut *mut uint32_t fc_hdr_wd4; / word 14,
    pub /: *mut *mut uint32_t fc_hdr_wd5; / word 15,
}

pub const ELS_RDF_REG_TAG_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_els_rdf_reg_desc {
    pub /: *mut *mut fc_df_desc_fpin_reg_hdr reg_desc; / descriptor header,
    pub desc_tags: [__be32; ELS_RDF_REG_TAG_CNT],
// tags in reg_desc
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_els_rdf_req {
    pub /: *mut *mut fc_els_rdf_hdr rdf; / hdr up to descriptors,
    pub /: *mut *mut lpfc_els_rdf_reg_desc reg_d1; / 1st descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_els_rdf_rsp {
    pub /: *mut *mut fc_els_rdf_resp_hdr rdf_resp; / hdr up to descriptors,
    pub /: *mut *mut lpfc_els_rdf_reg_desc reg_d1; / 1st descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lpfc_wqe {
    pub words: [u32; 16],
    pub generic: lpfc_wqe_generic,
    pub fcp_icmd: fcp_icmnd64_wqe,
    pub fcp_iread: fcp_iread64_wqe,
    pub fcp_iwrite: fcp_iwrite64_wqe,
    pub abort_cmd: abort_cmd_wqe,
    pub cmf_sync: cmf_sync_wqe,
    pub create_xri: create_xri_wqe,
    pub xmit_bcast64: xmit_bcast64_wqe,
    pub xmit_sequence: xmit_seq64_wqe,
    pub xmit_bls_rsp: xmit_bls_rsp64_wqe,
    pub xmit_els_rsp: xmit_els_rsp64_wqe,
    pub els_req: els_request64_wqe,
    pub gen_req: gen_req64_wqe,
    pub fcp_trsp: fcp_trsp64_wqe,
    pub fcp_tsend: fcp_tsend64_wqe,
    pub fcp_treceive: fcp_treceive64_wqe,
    pub send_frame: send_frame_wqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lpfc_wqe128 {
    pub words: [u32; 32],
    pub generic: lpfc_wqe_generic,
    pub fcp_icmd: fcp_icmnd64_wqe,
    pub fcp_iread: fcp_iread64_wqe,
    pub fcp_iwrite: fcp_iwrite64_wqe,
    pub abort_cmd: abort_cmd_wqe,
    pub cmf_sync: cmf_sync_wqe,
    pub create_xri: create_xri_wqe,
    pub xmit_bcast64: xmit_bcast64_wqe,
    pub xmit_sequence: xmit_seq64_wqe,
    pub xmit_bls_rsp: xmit_bls_rsp64_wqe,
    pub xmit_els_rsp: xmit_els_rsp64_wqe,
    pub els_req: els_request64_wqe,
    pub gen_req: gen_req64_wqe,
    pub fcp_trsp: fcp_trsp64_wqe,
    pub fcp_tsend: fcp_tsend64_wqe,
    pub fcp_treceive: fcp_treceive64_wqe,
    pub send_frame: send_frame_wqe,
}

pub const MAGIC_NUMBER_G6: c_uint = 0xFEAA0003;
pub const MAGIC_NUMBER_G7: c_uint = 0xFEAA0005;
pub const MAGIC_NUMBER_G7P: c_uint = 0xFEAA0020;
pub const MAGIC_NUMBER_G8: c_uint = 0xFEAA0070;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_grp_hdr {
    pub size: u32,
    pub magic_number: u32,
    pub word2: u32,
pub const lpfc_grp_hdr_file_type_SHIFT: c_int = 24;
pub const lpfc_grp_hdr_file_type_MASK: c_uint = 0x000000FF;

pub const lpfc_grp_hdr_id_SHIFT: c_int = 16;
pub const lpfc_grp_hdr_id_MASK: c_uint = 0x000000FF;
    pub rev_name: [u8; 128],
    pub date: [u8; 12],
    pub revision: [u8; 32],
}

// Defines for WQE command type
pub const FCP_COMMAND: c_uint = 0x0;
pub const NVME_READ_CMD: c_uint = 0x0;
pub const FCP_COMMAND_DATA_OUT: c_uint = 0x1;
pub const NVME_WRITE_CMD: c_uint = 0x1;
pub const COMMAND_DATA_IN: c_uint = 0x0;
pub const COMMAND_DATA_OUT: c_uint = 0x1;
pub const FCP_COMMAND_TRECEIVE: c_uint = 0x2;
pub const FCP_COMMAND_TRSP: c_uint = 0x3;
pub const FCP_COMMAND_TSEND: c_uint = 0x7;
pub const OTHER_COMMAND: c_uint = 0x8;
pub const CMF_SYNC_COMMAND: c_uint = 0xA;
pub const ELS_COMMAND_NON_FIP: c_uint = 0xC;
pub const ELS_COMMAND_FIP: c_uint = 0xD;
pub const LPFC_NVME_EMBED_CMD: c_uint = 0x0;
pub const LPFC_NVME_EMBED_WRITE: c_uint = 0x1;
pub const LPFC_NVME_EMBED_READ: c_uint = 0x2;
// WQE Commands
pub const CMD_ABORT_XRI_WQE: c_uint = 0x0F;
pub const CMD_XMIT_SEQUENCE64_WQE: c_uint = 0x82;
pub const CMD_XMIT_BCAST64_WQE: c_uint = 0x84;
pub const CMD_ELS_REQUEST64_WQE: c_uint = 0x8A;
pub const CMD_XMIT_ELS_RSP64_WQE: c_uint = 0x95;
pub const CMD_XMIT_BLS_RSP64_WQE: c_uint = 0x97;
pub const CMD_FCP_IWRITE64_WQE: c_uint = 0x98;
pub const CMD_FCP_IREAD64_WQE: c_uint = 0x9A;
pub const CMD_FCP_ICMND64_WQE: c_uint = 0x9C;
pub const CMD_FCP_TSEND64_WQE: c_uint = 0x9F;
pub const CMD_FCP_TRECEIVE64_WQE: c_uint = 0xA1;
pub const CMD_FCP_TRSP64_WQE: c_uint = 0xA3;
pub const CMD_GEN_REQUEST64_WQE: c_uint = 0xC2;
pub const CMD_CMF_SYNC_WQE: c_uint = 0xE8;
pub const CMD_WQE_MASK: c_uint = 0xff;
pub const LPFC_FW_DUMP: c_int = 1;
pub const LPFC_FW_RESET: c_int = 2;
pub const LPFC_DV_RESET: c_int = 3;
// On some kernels, enum fc_ls_tlv_dtag does not have
// these 2 enums defined, on other kernels it does.
// To get aound this we need to add these 2 defines here.
//

pub const ELS_DTAG_LNK_FAULT_CAP: c_uint = 0x0001000D;

pub const ELS_DTAG_CG_SIGNAL_CAP: c_uint = 0x0001000F;

//
// Initializer useful for decoding FPIN string table.
//

// Used for logging FPIN messages
pub const LPFC_FPIN_WWPN_LINE_SZ: c_int = 128;
pub const LPFC_FPIN_WWPN_LINE_CNT: c_int = 6;
pub const LPFC_FPIN_WWPN_NUM_LINE: c_int = 6;
