//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/esas2r/esas2r.h
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
// linux/drivers/scsi/esas2r/esas2r.h
// For use with ATTO ExpressSAS R6xx SAS/SATA RAID controllers
//
// Copyright (c) 2001-2013 ATTO Technology, Inc.
// (mailto:linuxdrivers@attotech.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// NO WARRANTY
// THE PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED INCLUDING, WITHOUT
// LIMITATION, ANY WARRANTIES OR CONDITIONS OF TITLE, NON-INFRINGEMENT,
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Each Recipient is
// solely responsible for determining the appropriateness of using and
// distributing the Program and assumes all risks associated with its
// exercise of rights under this Agreement, including but not limited to
// the risks and costs of program errors, damage to or loss of data,
// programs or equipment, and unavailability or interruption of operations.
//
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301,
// USA.
//

// Global Variables
pub const SGL_PG_SZ_MIN: c_int = 64;
pub const SGL_PG_SZ_MAX: c_int = 1024;
pub const NUM_SGL_MIN: c_int = 8;
pub const NUM_SGL_MAX: c_int = 2048;
pub const NUM_REQ_MIN: c_int = 4;
pub const NUM_REQ_MAX: c_int = 256;
pub const NUM_AE_MIN: c_int = 2;
pub const NUM_AE_MAX: c_int = 8;
// Macro defintions
pub const ESAS2R_MAX_ID: c_int = 255;
pub const MAX_ADAPTERS: c_int = 32;

pub const ESAS2R_MAX_DEVICES: c_int = 32;

pub const ESAS2R_MAJOR_REV: c_int = 1;
pub const ESAS2R_MINOR_REV: c_int = 00;

pub const ESAS2R_DEFAULT_SGL_PAGE_SIZE: c_int = 384;
pub const ESAS2R_DEFAULT_CMD_PER_LUN: c_int = 64;
pub const ESAS2R_DEFAULT_NUM_SG_LISTS: c_int = 1024;

pub const ESAS2R_SGL_ALIGN: c_int = 16;
pub const ESAS2R_LIST_ALIGN: c_int = 16;

pub const ESAS2R_DATA_BUF_LEN: c_int = 256;
pub const ESAS2R_DEFAULT_TMO: c_int = 5000;
pub const ESAS2R_DISC_BUF_LEN: c_int = 512;
pub const ESAS2R_FWCOREDUMP_SZ: c_uint = 0x80000;
pub const ESAS2R_NUM_PHYS: c_int = 8;
pub const ESAS2R_TARG_ID_INV: c_uint = 0xFFFF;

pub const ESAS2R_INT_DIS_MASK: c_int = 0;
pub const ESAS2R_MAX_TARGETS: c_int = 256;
pub const ESAS2R_KOBJ_NAME_LEN: c_int = 20;
// u16 (WORD) component macros

// u32 (DWORD) component macros

// macro to get the lowest nonzero bit of a value

// These functions are provided to access the chip's control registers.
// The register is specified by its byte offset from the register base
// for the adapter.
//

// This function is provided to access the chip's data window.   The
// register is specified by its byte offset from the window base
// for the adapter.
//

// ATTO vendor and device Ids
pub const ATTO_VENDOR_ID: c_uint = 0x117C;
pub const ATTO_DID_INTEL_IOP348: c_uint = 0x002C;
pub const ATTO_DID_MV_88RC9580: c_uint = 0x0049;
pub const ATTO_DID_MV_88RC9580TS: c_uint = 0x0066;
pub const ATTO_DID_MV_88RC9580TSE: c_uint = 0x0067;
pub const ATTO_DID_MV_88RC9580TL: c_uint = 0x0068;
// ATTO subsystem device Ids
pub const ATTO_SSDID_TBT: c_uint = 0x4000;
pub const ATTO_TSSC_3808: c_uint = 0x4066;
pub const ATTO_TSSC_3808E: c_uint = 0x4067;
pub const ATTO_TLSH_1068: c_uint = 0x4068;
pub const ATTO_ESAS_R680: c_uint = 0x0049;
pub const ATTO_ESAS_R608: c_uint = 0x004A;
pub const ATTO_ESAS_R60F: c_uint = 0x004B;
pub const ATTO_ESAS_R6F0: c_uint = 0x004C;
pub const ATTO_ESAS_R644: c_uint = 0x004D;
pub const ATTO_ESAS_R648: c_uint = 0x004E;
//
// flash definitions & structures
// define the code types
//
pub const FBT_CPYR: c_uint = 0xAA00;
pub const FBT_SETUP: c_uint = 0xAA02;
pub const FBT_FLASH_VER: c_uint = 0xAA04;
// offsets to various locations in flash

pub const FI_NVR_2KB: c_uint = 0x0800;
pub const FI_NVR_8KB: c_uint = 0x2000;
pub const FM_BUF_SZ: c_uint = 0x800;
//
// marvell frey (88R9580) register definitions
// chip revision identifiers
//
pub const MVR_FREY_B2: c_uint = 0xB2;
//
// memory window definitions.  window 0 is the data window with definitions
// of MW_DATA_XXX.  window 1 is the register window with definitions of
// MW_REG_XXX.
//

//
// the following registers are for the communication
// list interface (AKA message unit (MU))
//

pub const MU_ILC_NUMBER_SHIFT: c_int = 16;

pub const MU_OLC_NUMBER_SHIFT: c_int = 16;

//
// the maximum size of the communication lists is two greater than the
// maximum amount of VDA requests.  the extra are to prevent queue overflow.
//
pub const ESAS2R_MAX_NUM_REQS: c_int = 256;
pub const ESAS2R_NUM_EXTRA: c_int = 2;

//
// the following registers are for the CPU interface
//

// PCI express registers accessed via window 1

// structures
// inbound list dynamic source entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_inbound_list_source_entry {
    pub address: u64,
    pub length: u32,
pub const HWILSE_INTERFACE_F0: c_uint = 0x00000000;
    pub reserved: u32,
}

// PCI data structure in expansion ROM images
pub const CODE_TYPE_PC: c_int = 0;
pub const CODE_TYPE_OPEN: c_int = 1;
pub const CODE_TYPE_EFI: c_int = 3;
pub const INDICATOR_LAST: c_uint = 0x80;
pub const EFI_ROM_SIG: c_uint = 0x00000EF1;
pub const EFI_IMAGE_APP: c_int = 10;
pub const EFI_IMAGE_BSD: c_int = 11;
pub const EFI_IMAGE_RTD: c_int = 12;
pub const EFI_MACHINE_IA32: c_uint = 0x014c;
pub const EFI_MACHINE_IA64: c_uint = 0x0200;
pub const EFI_MACHINE_X64: c_uint = 0x8664;
pub const EFI_MACHINE_EBC: c_uint = 0x0EBC;
pub const EFI_UNCOMPRESSED: c_uint = 0x0000;
pub const EFI_COMPRESSED: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_component_header {
    pub img_type: u8,
pub const CH_IT_FW: c_uint = 0x00;
pub const CH_IT_NVR: c_uint = 0x01;
pub const CH_IT_BIOS: c_uint = 0x02;
pub const CH_IT_MAC: c_uint = 0x03;
pub const CH_IT_CFG: c_uint = 0x04;
pub const CH_IT_EFI: c_uint = 0x05;
    pub status: u8,
pub const CH_STAT_PENDING: c_uint = 0xff;
pub const CH_STAT_FAILED: c_uint = 0x00;
pub const CH_STAT_SUCCESS: c_uint = 0x01;
pub const CH_STAT_RETRY: c_uint = 0x02;
pub const CH_STAT_INVALID: c_uint = 0x03;
    pub pad: [u8; 2],
    pub version: u32,
    pub length: u32,
    pub image_offset: u32,
}

pub const FI_REL_VER_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_flash_img_v0 {
    pub fi_version: u8,
pub const FI_VERSION_0: c_int = 00;
    pub status: u8,
    pub adap_typ: u8,
    pub action: u8,
    pub length: u32,
    pub checksum: u16,
    pub driver_error: u16,
    pub flags: u16,
    pub num_comps: u16,
pub const FI_NUM_COMPS_V0: c_int = 5;
    pub rel_version: [u8; FI_REL_VER_SZ],
    pub cmp_hdr: [esas2r_component_header; FI_NUM_COMPS_V0],
    pub scratch_buf: [u8; FM_BUF_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_flash_img {
    pub fi_version: u8,
pub const FI_VERSION_1: c_int = 01;
    pub status: u8,
pub const FI_STAT_SUCCESS: c_uint = 0x00;
pub const FI_STAT_FAILED: c_uint = 0x01;
pub const FI_STAT_REBOOT: c_uint = 0x02;
pub const FI_STAT_ADAPTYP: c_uint = 0x03;
pub const FI_STAT_INVALID: c_uint = 0x04;
pub const FI_STAT_CHKSUM: c_uint = 0x05;
pub const FI_STAT_LENGTH: c_uint = 0x06;
pub const FI_STAT_UNKNOWN: c_uint = 0x07;
pub const FI_STAT_IMG_VER: c_uint = 0x08;
pub const FI_STAT_BUSY: c_uint = 0x09;
pub const FI_STAT_DUAL: c_uint = 0x0A;
pub const FI_STAT_MISSING: c_uint = 0x0B;
pub const FI_STAT_UNSUPP: c_uint = 0x0C;
pub const FI_STAT_ERASE: c_uint = 0x0D;
pub const FI_STAT_FLASH: c_uint = 0x0E;
pub const FI_STAT_DEGRADED: c_uint = 0x0F;
    pub adap_typ: u8,
pub const FI_AT_UNKNWN: c_uint = 0xFF;
pub const FI_AT_SUN_LAKE: c_uint = 0x0B;
pub const FI_AT_MV_9580: c_uint = 0x0F;
    pub action: u8,
pub const FI_ACT_DOWN: c_uint = 0x00;
pub const FI_ACT_UP: c_uint = 0x01;
pub const FI_ACT_UPSZ: c_uint = 0x02;
pub const FI_ACT_MAX: c_uint = 0x02;
pub const FI_ACT_DOWN1: c_uint = 0x80;
    pub length: u32,
    pub checksum: u16,
    pub driver_error: u16,
    pub flags: u16,
pub const FI_FLG_NVR_DEF: c_uint = 0x0001;
    pub num_comps: u16,
pub const FI_NUM_COMPS_V1: c_int = 6;
    pub rel_version: [u8; FI_REL_VER_SZ],
    pub cmp_hdr: [esas2r_component_header; FI_NUM_COMPS_V1],
    pub scratch_buf: [u8; FM_BUF_SZ],
}

// definitions for flash script (FS) commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_ioctlfs_command {
    pub command: u8,
pub const ESAS2R_FS_CMD_ERASE: c_int = 0;
pub const ESAS2R_FS_CMD_READ: c_int = 1;
pub const ESAS2R_FS_CMD_BEGINW: c_int = 2;
pub const ESAS2R_FS_CMD_WRITE: c_int = 3;
pub const ESAS2R_FS_CMD_COMMIT: c_int = 4;
pub const ESAS2R_FS_CMD_CANCEL: c_int = 5;
    pub checksum: u8,
    pub reserved: [u8; 2],
    pub flash_addr: u32,
    pub length: u32,
    pub image_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_ioctl_fs {
    pub version: u8,
pub const ESAS2R_FS_VER: c_int = 0;
    pub status: u8,
    pub driver_error: u8,
    pub adap_type: u8,
pub const ESAS2R_FS_AT_ESASRAID2: c_int = 3;
pub const ESAS2R_FS_AT_TSSASRAID2: c_int = 4;
pub const ESAS2R_FS_AT_TSSASRAID2E: c_int = 5;
pub const ESAS2R_FS_AT_TLSASHBA: c_int = 6;
    pub driver_ver: u8,
    pub reserved: [u8; 11],
    pub command: esas2r_ioctlfs_command,
    pub data: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_sas_nvram {
    pub signature: [u8; 4],
    pub version: u8,
pub const SASNVR_VERSION_0: c_uint = 0x00;

    pub checksum: u8,
pub const SASNVR_CKSUM_SEED: c_uint = 0x5A;
    pub max_lun_for_target: u8,
    pub pci_latency: u8,
pub const SASNVR_PCILAT_DIS: c_uint = 0x00;
pub const SASNVR_PCILAT_MIN: c_uint = 0x10;
pub const SASNVR_PCILAT_MAX: c_uint = 0xF8;
    pub options1: u8,
pub const SASNVR1_BOOT_DRVR: c_uint = 0x01;
pub const SASNVR1_BOOT_SCAN: c_uint = 0x02;
pub const SASNVR1_DIS_PCI_MWI: c_uint = 0x04;
pub const SASNVR1_FORCE_ORD_Q: c_uint = 0x08;
pub const SASNVR1_CACHELINE_0: c_uint = 0x10;
pub const SASNVR1_DIS_DEVSORT: c_uint = 0x20;
pub const SASNVR1_PWR_MGT_EN: c_uint = 0x40;
pub const SASNVR1_WIDEPORT: c_uint = 0x80;
    pub options2: u8,
pub const SASNVR2_SINGLE_BUS: c_uint = 0x01;
pub const SASNVR2_SLOT_BIND: c_uint = 0x02;
pub const SASNVR2_EXP_PROG: c_uint = 0x04;
pub const SASNVR2_CMDTHR_LUN: c_uint = 0x08;
pub const SASNVR2_HEARTBEAT: c_uint = 0x10;
pub const SASNVR2_INT_CONNECT: c_uint = 0x20;
pub const SASNVR2_SW_MUX_CTRL: c_uint = 0x40;
pub const SASNVR2_DISABLE_NCQ: c_uint = 0x80;
    pub int_coalescing: u8,
pub const SASNVR_COAL_DIS: c_uint = 0x00;
pub const SASNVR_COAL_LOW: c_uint = 0x01;
pub const SASNVR_COAL_MED: c_uint = 0x02;
pub const SASNVR_COAL_HI: c_uint = 0x03;
    pub cmd_throttle: u8,
pub const SASNVR_CMDTHR_NONE: c_uint = 0x00;
    pub dev_wait_time: u8,
    pub dev_wait_count: u8,
    pub spin_up_delay: u8,
pub const SASNVR_SPINUP_MAX: c_uint = 0x14;
    pub ssp_align_rate: u8,
    pub sas_addr: [u8; 8],
    pub phy_speed: [u8; 16],
pub const SASNVR_SPEED_AUTO: c_uint = 0x00;
pub const SASNVR_SPEED_1_5GB: c_uint = 0x01;
pub const SASNVR_SPEED_3GB: c_uint = 0x02;
pub const SASNVR_SPEED_6GB: c_uint = 0x03;
pub const SASNVR_SPEED_12GB: c_uint = 0x04;
    pub phy_mux: [u8; 16],
pub const SASNVR_MUX_DISABLED: c_uint = 0x00;
pub const SASNVR_MUX_1_5GB: c_uint = 0x01;
pub const SASNVR_MUX_3GB: c_uint = 0x02;
pub const SASNVR_MUX_6GB: c_uint = 0x03;
    pub phy_flags: [u8; 16],
pub const SASNVR_PHF_DISABLED: c_uint = 0x01;
pub const SASNVR_PHF_RD_ONLY: c_uint = 0x02;
    pub sort_type: u8,
pub const SASNVR_SORT_SAS_ADDR: c_uint = 0x00;
pub const SASNVR_SORT_H308_CONN: c_uint = 0x01;
pub const SASNVR_SORT_PHY_ID: c_uint = 0x02;
pub const SASNVR_SORT_SLOT_ID: c_uint = 0x03;
    pub dpm_reqcmd_lmt: u8,
    pub dpm_stndby_time: u8,
    pub dpm_active_time: u8,
    pub phy_target_id: [u8; 16],
pub const SASNVR_PTI_DISABLED: c_uint = 0xFF;
    pub virt_ses_mode: u8,
pub const SASNVR_VSMH_DISABLED: c_uint = 0x00;
    pub read_write_mode: u8,
pub const SASNVR_RWM_DEFAULT: c_uint = 0x00;
    pub link_down_to: u8,
    pub reserved: [u8; 0xA1],
}

extern "C" {
    pub fn u32(sgc: *mut *mut PGETPHYSADDR) (struct esas2r_sg_context, addr: *mut u64) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_sg_context {
    pub adapter: *mut esas2r_adapter,
    pub first_req: *mut esas2r_request,
    pub length: u32,
    pub cur_offset: *mut u8,
    pub get_phys_addr: PGETPHYSADDR,
    pub curr: *mut atto_vda_sge,
    pub last: *mut atto_vda_sge,
    pub limit: *mut atto_vda_sge,
    pub chain: *mut atto_vda_sge,
    pub a64: },
    pub curr: *mut atto_physical_region_description,
    pub chain: *mut atto_physical_region_description,
    pub sgl_max_cnt: u32,
    pub sge_cnt: u32,
    pub prd: },
    pub sge: },
    pub cur_sgel: *mut scatterlist,
    pub exp_offset: *mut u8,
    pub num_sgel: c_int,
    pub sgel_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_target {
    pub flags: u8,
pub const TF_PASS_THRU: c_uint = 0x01;
pub const TF_USED: c_uint = 0x02;
    pub new_target_state: u8,
    pub target_state: u8,
    pub buffered_target_state: u8,
pub const TS_NOT_PRESENT: c_uint = 0x00;
pub const TS_PRESENT: c_uint = 0x05;
pub const TS_LUN_CHANGE: c_uint = 0x06;
pub const TS_INVALID: c_uint = 0xFF;
    pub block_size: u32,
    pub inter_block: u32,
    pub inter_byte: u32,
    pub virt_targ_id: u16,
    pub phys_targ_id: u16,
    pub identifier_len: u8,
    pub sas_addr: u64,
    pub identifier: [u8; 60],
    pub lu_event: atto_vda_ae_lu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_request {
    pub comp_list: list_head,
    pub req_list: list_head,
    pub vrq: *mut atto_vda_req,
    pub vrq_md: *mut esas2r_mem_desc,
    pub data_buf: *mut c_void,
    pub vda_rsp_data: *mut atto_vda_rsp_data,
}

pub const RQ_TIMEOUT_S1: c_uint = 0xFFFFFFFF;
pub const RQ_TIMEOUT_S2: c_uint = 0xFFFFFFFE;
pub const RQ_MAX_TIMEOUT: c_uint = 0xFFFFFFFD;
pub const RT_INI_REQ: c_uint = 0x01;
pub const RT_DISC_REQ: c_uint = 0x02;
pub const RF_1ST_IBLK_BASE: c_uint = 0x04;
pub const RF_FAILURE_OK: c_uint = 0x08;
pub const RQ_SIZE_DEFAULT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_flash_context {
    pub fi: *mut esas2r_flash_img,
    pub interrupt_cb: RQCALLBK,
    pub sgc_offset: *mut u8,
    pub scratch: *mut u8,
    pub fi_hdr_len: u32,
    pub task: u8,
pub const FMTSK_ERASE_BOOT: c_int = 0;
pub const FMTSK_WRTBIOS: c_int = 1;
pub const FMTSK_READBIOS: c_int = 2;
pub const FMTSK_WRTMAC: c_int = 3;
pub const FMTSK_READMAC: c_int = 4;
pub const FMTSK_WRTEFI: c_int = 5;
pub const FMTSK_READEFI: c_int = 6;
pub const FMTSK_WRTCFG: c_int = 7;
pub const FMTSK_READCFG: c_int = 8;
    pub func: u8,
    pub num_comps: u16,
    pub cmp_len: u32,
    pub flsh_addr: u32,
    pub curr_len: u32,
    pub comp_typ: u8,
    pub sgc: esas2r_sg_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_disc_context {
    pub disc_evt: u8,
pub const DCDE_DEV_CHANGE: c_uint = 0x01;
pub const DCDE_DEV_SCAN: c_uint = 0x02;
    pub state: u8,
pub const DCS_DEV_RMV: c_uint = 0x00;
pub const DCS_DEV_ADD: c_uint = 0x01;
pub const DCS_BLOCK_DEV_SCAN: c_uint = 0x02;
pub const DCS_RAID_GRP_INFO: c_uint = 0x03;
pub const DCS_PART_INFO: c_uint = 0x04;
pub const DCS_PT_DEV_INFO: c_uint = 0x05;
pub const DCS_PT_DEV_ADDR: c_uint = 0x06;
pub const DCS_DISC_DONE: c_uint = 0xFF;
    pub flags: u16,
pub const DCF_DEV_CHANGE: c_uint = 0x0001;
pub const DCF_DEV_SCAN: c_uint = 0x0002;
pub const DCF_POLLED: c_uint = 0x8000;
    pub interleave: u32,
    pub block_size: u32,
    pub dev_ix: u16,
    pub part_num: u8,
    pub raid_grp_ix: u8,
    pub raid_grp_name: [c_char; 16],
    pub curr_targ: *mut esas2r_target,
    pub curr_virt_id: u16,
    pub curr_phys_id: u16,
    pub scan_gen: u8,
    pub dev_addr_type: u8,
    pub sas_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_mem_desc {
    pub next_desc: list_head,
    pub virt_addr: *mut c_void,
    pub phys_addr: u64,
    pub pad: *mut c_void,
    pub esas2r_data: *mut c_void,
    pub esas2r_param: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_event_type {
    fw_event_null,
    fw_event_lun_change,
    fw_event_present,
    fw_event_not_present,
    fw_event_vda_ae
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_vda_ae {
    pub signature: u32,
pub const ESAS2R_VDA_EVENT_SIG: c_uint = 0x4154544F;
    pub bus_number: u8,
    pub devfn: u8,
    pub pad: [u8; 2],
    pub vda_ae: atto_vda_ae,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_fw_event_work {
    pub list: list_head,
    pub work: delayed_work,
    pub a: *mut esas2r_adapter,
    pub type: fw_event_type,
    pub esas2r_vda_ae)]: u8 data[sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state {
    FW_INVALID_ST,
    FW_STATUS_ST,
    FW_COMMAND_ST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_firmware {
    pub state: state,
    pub header: esas2r_flash_img,
    pub data: *mut u8,
    pub phys: u64,
    pub orig_len: c_int,
    pub header_buff: *mut c_void,
    pub header_buff_phys: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esas2r_adapter {
    pub targetdb: [esas2r_target; ESAS2R_MAX_TARGETS],
    pub targetdb_end: *mut esas2r_target,
    pub regs: *mut c_uchar,
    pub data_window: *mut c_uchar,
    pub flags: c_long,
pub const AF_PORT_CHANGE: c_int = 0;
pub const AF_CHPRST_NEEDED: c_int = 1;
pub const AF_CHPRST_PENDING: c_int = 2;
pub const AF_CHPRST_DETECTED: c_int = 3;
pub const AF_BUSRST_NEEDED: c_int = 4;
pub const AF_BUSRST_PENDING: c_int = 5;
pub const AF_BUSRST_DETECTED: c_int = 6;
pub const AF_DISABLED: c_int = 7;
pub const AF_FLASH_LOCK: c_int = 8;
pub const AF_OS_RESET: c_int = 9;
pub const AF_FLASHING: c_int = 10;
pub const AF_POWER_MGT: c_int = 11;
pub const AF_NVR_VALID: c_int = 12;
pub const AF_DEGRADED_MODE: c_int = 13;
pub const AF_DISC_PENDING: c_int = 14;
pub const AF_TASKLET_SCHEDULED: c_int = 15;
pub const AF_HEARTBEAT: c_int = 16;
pub const AF_HEARTBEAT_ENB: c_int = 17;
pub const AF_NOT_PRESENT: c_int = 18;
pub const AF_CHPRST_STARTED: c_int = 19;
pub const AF_FIRST_INIT: c_int = 20;
pub const AF_POWER_DOWN: c_int = 21;
pub const AF_DISC_IN_PROG: c_int = 22;
pub const AF_COMM_LIST_TOGGLE: c_int = 23;
pub const AF_LEGACY_SGE_MODE: c_int = 24;
pub const AF_DISC_POLLED: c_int = 25;
    pub flags2: c_long,
pub const AF2_SERIAL_FLASH: c_int = 0;
pub const AF2_DEV_SCAN: c_int = 1;
pub const AF2_DEV_CNT_OK: c_int = 2;
pub const AF2_COREDUMP_AVAIL: c_int = 3;
pub const AF2_COREDUMP_SAVED: c_int = 4;
pub const AF2_VDA_POWER_DOWN: c_int = 5;
pub const AF2_THUNDERLINK: c_int = 6;
pub const AF2_THUNDERBOLT: c_int = 7;
pub const AF2_INIT_DONE: c_int = 8;
pub const AF2_INT_PENDING: c_int = 9;
pub const AF2_TIMER_TICK: c_int = 10;
pub const AF2_IRQ_CLAIMED: c_int = 11;
pub const AF2_MSI_ENABLED: c_int = 12;
    pub disable_cnt: core::sync::atomic::AtomicI32,
    pub dis_ints_cnt: core::sync::atomic::AtomicI32,
    pub int_stat: u32,
    pub int_mask: u32,
    pub outbound_copy: *mut u32 volatile,
    pub avail_request: list_head,
    pub request_lock: spinlock_t,
    pub sg_list_lock: spinlock_t,
    pub queue_lock: spinlock_t,
    pub mem_lock: spinlock_t,
    pub free_sg_list_head: list_head,
    pub sg_list_mds: *mut esas2r_mem_desc,
    pub active_list: list_head,
    pub defer_list: list_head,
    pub req_table: *mut esas2r_request,
    pub prev_dev_cnt: u16,
    pub heartbeat_time: u32,

}

pub const ESAS2R_INIT_MSG_START: c_int = 1;
pub const ESAS2R_INIT_MSG_INIT: c_int = 2;
pub const ESAS2R_INIT_MSG_GET_INIT: c_int = 3;
pub const ESAS2R_INIT_MSG_REINIT: c_int = 4;

//
// intr_mode stores the interrupt mode currently being used by this
// adapter. it is based on the interrupt_mode module parameter, but
// can be changed based on the ability (or not) to utilize the
// mode requested by the parameter.
//
pub const INTR_MODE_LEGACY: c_int = 0;
pub const INTR_MODE_MSI: c_int = 1;
pub const INTR_MODE_MSIX: c_int = 2;
//
// Function Declarations
// SCSI functions
//
extern "C" {
    pub fn esas2r_ioctl_handler(hostdata: *mut c_void, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn esas2r_ioctl(dev: *mut scsi_device, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn esas2r_show_info(m: *mut seq_file, sh: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn esas2r_proc_ioctl(fp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// SCSI error handler (eh) functions
extern "C" {
    pub fn esas2r_eh_abort(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn esas2r_device_reset(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn esas2r_host_reset(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn esas2r_bus_reset(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn esas2r_target_reset(cmd: *mut scsi_cmnd) -> c_int;
}
// Internal functions
extern "C" {
    pub fn esas2r_read_fw(a: *mut esas2r_adapter, buf: *mut c_char, off: c_long, count: c_int) -> c_int;
}
extern "C" {
    pub fn esas2r_read_vda(a: *mut esas2r_adapter, buf: *mut c_char, off: c_long, count: c_int) -> c_int;
}
extern "C" {
    pub fn esas2r_read_fs(a: *mut esas2r_adapter, buf: *mut c_char, off: c_long, count: c_int) -> c_int;
}
extern "C" {
    pub fn esas2r_adapter_tasklet(context: c_ulong);
}
extern "C" {
    pub fn esas2r_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn esas2r_msi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn esas2r_kickoff_timer(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_fw_event_off(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_fw_event_on(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_reset_detected(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_req_status_to_error(req_stat: u8) -> c_int;
}
extern "C" {
    pub fn esas2r_kill_adapter(i: c_int);
}
extern "C" {
    pub fn esas2r_free_request(a: *mut esas2r_adapter, rq: *mut esas2r_request);
}
extern "C" {
    pub fn esas2r_get_uncached_size(a: *mut esas2r_adapter) -> u32;
}
extern "C" {
    pub fn esas2r_check_adapter(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_init_adapter_hw(a: *mut esas2r_adapter, init_poll: bool) -> bool;
}
extern "C" {
    pub fn esas2r_start_request(a: *mut esas2r_adapter, rq: *mut esas2r_request);
}
extern "C" {
    pub fn esas2r_do_tasklet_tasks(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_adapter_interrupt(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_do_deferred_processes(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_reset_bus(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_reset_adapter(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_timer_tick(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_build_ae_req(a: *mut esas2r_adapter, rq: *mut esas2r_request);
}
extern "C" {
    pub fn esas2r_power_down(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_power_up(a: *mut esas2r_adapter, init_poll: bool) -> bool;
}
extern "C" {
    pub fn esas2r_wait_request(a: *mut esas2r_adapter, rq: *mut esas2r_request);
}
extern "C" {
    pub fn esas2r_map_data_window(a: *mut esas2r_adapter, addr_lo: u32) -> u32;
}
extern "C" {
    pub fn esas2r_force_interrupt(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_process_adapter_reset(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_ae_complete(a: *mut esas2r_adapter, rq: *mut esas2r_request);
}
extern "C" {
    pub fn esas2r_read_flash_rev(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_read_image_type(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_nvram_read_direct(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_nvram_validate(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_nvram_set_defaults(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_print_flash_rev(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_send_reset_ae(a: *mut esas2r_adapter, pwr_mgt: bool);
}
extern "C" {
    pub fn esas2r_init_msgs(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_is_adapter_present(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_nuxi_mgt_data(function: u8, data: *mut c_void);
}
extern "C" {
    pub fn esas2r_nuxi_cfg_data(function: u8, data: *mut c_void);
}
extern "C" {
    pub fn esas2r_nuxi_ae_data(ae: *mut atto_vda_ae);
}
extern "C" {
    pub fn esas2r_reset_chip(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_polled_interrupt(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_targ_db_initialize(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_targ_db_remove_all(a: *mut esas2r_adapter, notify: bool);
}
extern "C" {
    pub fn esas2r_targ_db_report_changes(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_targ_db_remove(a: *mut esas2r_adapter, t: *mut esas2r_target);
}
extern "C" {
    pub fn esas2r_targ_db_find_next_present(a: *mut esas2r_adapter, target_id: u16) -> u16;
}
extern "C" {
    pub fn esas2r_targ_db_get_tgt_cnt(a: *mut esas2r_adapter) -> u16;
}
extern "C" {
    pub fn esas2r_disc_initialize(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_disc_start_waiting(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_disc_check_for_work(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_disc_check_complete(a: *mut esas2r_adapter);
}
extern "C" {
    pub fn esas2r_disc_queue_event(a: *mut esas2r_adapter, disc_evt: u8);
}
extern "C" {
    pub fn esas2r_disc_start_port(a: *mut esas2r_adapter) -> bool;
}
extern "C" {
    pub fn esas2r_set_degraded_mode(a: *mut esas2r_adapter, error_str: *mut c_char) -> bool;
}
// Inline functions
// Allocate a chip scatter/gather list entry
// Initialize a scatter/gather context
//
// set the limit pointer such that an SGE pointer above this value
// would be the first one to overflow the SGL.
//
// clear the outbound response
//
// clear the size of the VDA request.  esas2r_build_sg_list() will
// only allow the size of the request to grow.  there are some
// management requests that go through there twice and the second
// time through sets a smaller request size.  if this is not modified
// at all we'll set it to the size of the entire VDA request.
//
// req_table entry should be NULL at this point - if not, halt
// fill in the table for this handle so we can get back to the
// request.
//
// add a reference number to the handle to make it unique (until it
// wraps of course) while preserving the least significant word
//
// the following formats a SCSI request.  the caller can override as
// necessary.  clear_vda_request can be called to clear the VDA
// request for another type of request.
//
// clear out sg_list_offset and chain_offset
// set the sense buffer to be the data payload buffer
//
// Build the scatter/gather list for an I/O request according to the
// specifications placed in the esas2r_sg_context.  The caller must initialize
// struct esas2r_sg_context prior to the initial call by calling
// esas2r_sgc_init()
//
// Schedule a TASKLET to perform non-interrupt tasks that may require delays
// or long completion times.
//
// make sure we don't schedule twice
// Set the initial state for resetting the adapter on the next pass through
// esas2r_do_deferred.
//
// See if an interrupt is pending on the adapter.
// Build and start an asynchronous event request
// sysfs handlers
