//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/esas2r/atioctl.h
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


// linux/drivers/scsi/esas2r/atioctl.h
// ATTO IOCTL Handling
//
// Copyright (c) 2001-2013 ATTO Technology, Inc.
// (mailto:linuxdrivers@attotech.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
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
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

pub const EXPRESS_IOCTL_SIGNATURE_SIZE: c_int = 8;
// structure definitions for IOCTls
pub const IOCTL_SUCCESS: c_int = 0;
pub const IOCTL_ERR_INVCMD: c_int = 101;
pub const IOCTL_INIT_FAILED: c_int = 102;
pub const IOCTL_NOT_IMPLEMENTED: c_int = 103;
pub const IOCTL_BAD_CHANNEL: c_int = 104;
pub const IOCTL_TARGET_OVERRUN: c_int = 105;
pub const IOCTL_TARGET_NOT_ENABLED: c_int = 106;
pub const IOCTL_BAD_FLASH_IMGTYPE: c_int = 107;
pub const IOCTL_OUT_OF_RESOURCES: c_int = 108;
pub const IOCTL_GENERAL_ERROR: c_int = 109;
pub const IOCTL_INVALID_PARAM: c_int = 110;
//
// NOTE - if channel == 0xFF, the request is
// handled on the adapter it came in on.
//
pub const MAX_NODE_NAMES: c_int = 256;
pub const FUNC_FW_DOWNLOAD: c_uint = 0x09;
pub const FUNC_FW_UPLOAD: c_uint = 0x12;
pub const FW_IMG_FW: c_uint = 0x01;
pub const FW_IMG_BIOS: c_uint = 0x02;
pub const FW_IMG_NVR: c_uint = 0x03;
pub const FW_IMG_RAW: c_uint = 0x04;
pub const FW_IMG_FM_API: c_uint = 0x05;
pub const FW_IMG_FS_API: c_uint = 0x06;
pub const MAX_CHANNEL: c_int = 256;
//
// CSMI control codes
// class independent
//
pub const CSMI_CC_GET_DRVR_INFO: c_int = 1;
pub const CSMI_CC_GET_CNTLR_CFG: c_int = 2;
pub const CSMI_CC_GET_CNTLR_STS: c_int = 3;
pub const CSMI_CC_FW_DOWNLOAD: c_int = 4;
// RAID class
pub const CSMI_CC_GET_RAID_INFO: c_int = 10;
pub const CSMI_CC_GET_RAID_CFG: c_int = 11;
// HBA class
pub const CSMI_CC_GET_PHY_INFO: c_int = 20;
pub const CSMI_CC_SET_PHY_INFO: c_int = 21;
pub const CSMI_CC_GET_LINK_ERRORS: c_int = 22;
pub const CSMI_CC_SMP_PASSTHRU: c_int = 23;
pub const CSMI_CC_SSP_PASSTHRU: c_int = 24;
pub const CSMI_CC_STP_PASSTHRU: c_int = 25;
pub const CSMI_CC_GET_SATA_SIG: c_int = 26;
pub const CSMI_CC_GET_SCSI_ADDR: c_int = 27;
pub const CSMI_CC_GET_DEV_ADDR: c_int = 28;
pub const CSMI_CC_TASK_MGT: c_int = 29;
pub const CSMI_CC_GET_CONN_INFO: c_int = 30;
// PHY class
pub const CSMI_CC_PHY_CTRL: c_int = 60;
//
// CSMI status codes
// class independent
//
pub const CSMI_STS_SUCCESS: c_int = 0;
pub const CSMI_STS_FAILED: c_int = 1;
pub const CSMI_STS_BAD_CTRL_CODE: c_int = 2;
pub const CSMI_STS_INV_PARAM: c_int = 3;
pub const CSMI_STS_WRITE_ATTEMPTED: c_int = 4;
// RAID class
pub const CSMI_STS_INV_RAID_SET: c_int = 1000;
// HBA class

pub const CSMI_STS_PHY_UNCHANGEABLE: c_int = 2000;
pub const CSMI_STS_INV_LINK_RATE: c_int = 2001;
pub const CSMI_STS_INV_PHY: c_int = 2002;
pub const CSMI_STS_INV_PHY_FOR_PORT: c_int = 2003;
pub const CSMI_STS_PHY_UNSELECTABLE: c_int = 2004;
pub const CSMI_STS_SELECT_PHY_OR_PORT: c_int = 2005;
pub const CSMI_STS_INV_PORT: c_int = 2006;
pub const CSMI_STS_PORT_UNSELECTABLE: c_int = 2007;
pub const CSMI_STS_CONNECTION_FAILED: c_int = 2008;
pub const CSMI_STS_NO_SATA_DEV: c_int = 2009;
pub const CSMI_STS_NO_SATA_SIGNATURE: c_int = 2010;
pub const CSMI_STS_SCSI_EMULATION: c_int = 2011;
pub const CSMI_STS_NOT_AN_END_DEV: c_int = 2012;
pub const CSMI_STS_NO_SCSI_ADDR: c_int = 2013;
pub const CSMI_STS_NO_DEV_ADDR: c_int = 2014;
// CSMI class independent structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_driver_info {
    pub name: [c_char; 81],
    pub description: [c_char; 81],
    pub major_rev: u16,
    pub minor_rev: u16,
    pub build_rev: u16,
    pub release_rev: u16,
    pub csmi_major_rev: u16,
    pub csmi_minor_rev: u16,
pub const CSMI_MAJOR_REV_0_81: c_int = 0;
pub const CSMI_MINOR_REV_0_81: c_int = 81;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_pci_bus_addr {
    pub bus_num: u8,
    pub device_num: u8,
    pub function_num: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_cntlr_cfg {
    pub base_io_addr: u32,
    pub base_memaddr_lo: u32,
    pub base_memaddr_hi: u32,
}

pub const CSMI_SLOT_NUM_UNKNOWN: c_uint = 0xFFFF;
pub const CSMI_CNTLR_CLASS_HBA: c_int = 5;
pub const CSMI_BUS_TYPE_PCI: c_int = 3;
pub const CSMI_BUS_TYPE_PCMCIA: c_int = 4;
pub const CSMI_CNTLRF_SAS_HBA: c_uint = 0x00000001;
pub const CSMI_CNTLRF_SAS_RAID: c_uint = 0x00000002;
pub const CSMI_CNTLRF_SATA_HBA: c_uint = 0x00000004;
pub const CSMI_CNTLRF_SATA_RAID: c_uint = 0x00000008;
pub const CSMI_CNTLRF_FWD_SUPPORT: c_uint = 0x00010000;
pub const CSMI_CNTLRF_FWD_ONLINE: c_uint = 0x00020000;
pub const CSMI_CNTLRF_FWD_SRESET: c_uint = 0x00040000;
pub const CSMI_CNTLRF_FWD_HRESET: c_uint = 0x00080000;
pub const CSMI_CNTLRF_FWD_RROM: c_uint = 0x00100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_cntlr_sts {
    pub status: u32,
pub const CSMI_CNTLR_STS_GOOD: c_int = 1;
pub const CSMI_CNTLR_STS_FAILED: c_int = 2;
pub const CSMI_CNTLR_STS_OFFLINE: c_int = 3;
pub const CSMI_CNTLR_STS_POWEROFF: c_int = 4;
    pub offline_reason: u32,
pub const CSMI_OFFLINE_NO_REASON: c_int = 0;
pub const CSMI_OFFLINE_INITIALIZING: c_int = 1;
pub const CSMI_OFFLINE_BUS_DEGRADED: c_int = 2;
pub const CSMI_OFFLINE_BUS_FAILURE: c_int = 3;
    pub reserved: [u8; 28],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_fw_download {
    pub buffer_len: u32,
    pub download_flags: u32,
pub const CSMI_FWDF_VALIDATE: c_uint = 0x00000001;
pub const CSMI_FWDF_SOFT_RESET: c_uint = 0x00000002;
pub const CSMI_FWDF_HARD_RESET: c_uint = 0x00000004;
    pub reserved: [u8; 32],
    pub status: u16,
pub const CSMI_FWD_STS_SUCCESS: c_int = 0;
pub const CSMI_FWD_STS_FAILED: c_int = 1;
pub const CSMI_FWD_STS_USING_RROM: c_int = 2;
pub const CSMI_FWD_STS_REJECT: c_int = 3;
pub const CSMI_FWD_STS_DOWNREV: c_int = 4;
    pub severity: u16,
pub const CSMI_FWD_SEV_INFO: c_int = 0;
pub const CSMI_FWD_SEV_WARNING: c_int = 1;
pub const CSMI_FWD_SEV_ERROR: c_int = 2;
pub const CSMI_FWD_SEV_FATAL: c_int = 3;
}

// CSMI RAID class structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_raid_info {
    pub num_raid_sets: u32,
    pub max_drivesper_set: u32,
    pub reserved: [u8; 92],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_raid_drives {
    pub model: [c_char; 40],
    pub firmware: [c_char; 8],
    pub serial_num: [c_char; 40],
    pub sas_addr: [u8; 8],
    pub lun: [u8; 8],
    pub drive_sts: u8,
pub const CSMI_DRV_STS_OK: c_int = 0;
pub const CSMI_DRV_STS_REBUILDING: c_int = 1;
pub const CSMI_DRV_STS_FAILED: c_int = 2;
pub const CSMI_DRV_STS_DEGRADED: c_int = 3;
    pub drive_usage: u8,
pub const CSMI_DRV_USE_NOT_USED: c_int = 0;
pub const CSMI_DRV_USE_MEMBER: c_int = 1;
pub const CSMI_DRV_USE_SPARE: c_int = 2;
    pub /: *mut *mut u8 reserved[30]; / spec says 22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_raid_cfg {
    pub raid_set_index: u32,
    pub capacity: u32,
    pub stripe_size: u32,
    pub raid_type: u8,
    pub status: u8,
    pub information: u8,
    pub drive_cnt: u8,
    pub reserved: [u8; 20],
    pub drives: [atto_csmi_raid_drives; 1],
}

// CSMI HBA class structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_phy_entity {
    pub ident_frame: [u8; 0x1C],
    pub port_id: u8,
    pub neg_link_rate: u8,
    pub min_link_rate: u8,
    pub max_link_rate: u8,
    pub phy_change_cnt: u8,
    pub auto_discover: u8,
pub const CSMI_DISC_NOT_SUPPORTED: c_uint = 0x00;
pub const CSMI_DISC_NOT_STARTED: c_uint = 0x01;
pub const CSMI_DISC_IN_PROGRESS: c_uint = 0x02;
pub const CSMI_DISC_COMPLETE: c_uint = 0x03;
pub const CSMI_DISC_ERROR: c_uint = 0x04;
    pub reserved: [u8; 2],
    pub attach_ident_frame: [u8; 0x1C],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_phy_info {
    pub number_of_phys: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_set_phy_info {
    pub phy_id: u8,
    pub neg_link_rate: u8,
pub const CSMI_NEG_RATE_NEGOTIATE: c_uint = 0x00;
pub const CSMI_NEG_RATE_PHY_DIS: c_uint = 0x01;
    pub prog_minlink_rate: u8,
    pub prog_maxlink_rate: u8,
    pub signal_class: u8,
pub const CSMI_SIG_CLASS_UNKNOWN: c_uint = 0x00;
pub const CSMI_SIG_CLASS_DIRECT: c_uint = 0x01;
pub const CSMI_SIG_CLASS_SERVER: c_uint = 0x02;
pub const CSMI_SIG_CLASS_ENCLOSURE: c_uint = 0x03;
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_link_errors {
    pub phy_id: u8,
    pub reset_cnts: u8,
pub const CSMI_RESET_CNTS_NO: c_uint = 0x00;
pub const CSMI_RESET_CNTS_YES: c_uint = 0x01;
    pub reserved: [u8; 2],
    pub inv_dw_cnt: u32,
    pub disp_err_cnt: u32,
    pub loss_ofdw_sync_cnt: u32,
    pub phy_reseterr_cnt: u32,
//
// The following field has been added by ATTO for ease of
// implementation of additional statistics.  Drivers must validate
// the length of the IOCTL payload prior to filling them in so CSMI
// complaint applications function correctly.
//
    pub crc_err_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_smp_passthru {
    pub phy_id: u8,
    pub port_id: u8,
    pub conn_rate: u8,
    pub reserved: u8,
    pub dest_sas_addr: [u8; 8],
    pub req_len: u32,
    pub smp_req: [u8; 1020],
    pub conn_sts: u8,
    pub reserved2: [u8; 3],
    pub rsp_len: u32,
    pub smp_rsp: [u8; 1020],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_ssp_passthru_sts {
    pub conn_sts: u8,
    pub reserved: [u8; 3],
    pub data_present: u8,
    pub status: u8,
    pub rsp_length: u16,
    pub rsp: [u8; 256],
    pub data_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_ssp_passthru {
    pub phy_id: u8,
    pub port_id: u8,
    pub conn_rate: u8,
    pub reserved: u8,
    pub dest_sas_addr: [u8; 8],
    pub lun: [u8; 8],
    pub cdb_len: u8,
    pub add_cdb_len: u8,
    pub reserved2: [u8; 2],
    pub cdb: [u8; 16],
    pub flags: u32,
pub const CSMI_SSPF_DD_READ: c_uint = 0x00000001;
pub const CSMI_SSPF_DD_WRITE: c_uint = 0x00000002;
pub const CSMI_SSPF_DD_UNSPECIFIED: c_uint = 0x00000004;
pub const CSMI_SSPF_TA_SIMPLE: c_uint = 0x00000000;
pub const CSMI_SSPF_TA_HEAD_OF_Q: c_uint = 0x00000010;
pub const CSMI_SSPF_TA_ORDERED: c_uint = 0x00000020;
pub const CSMI_SSPF_TA_ACA: c_uint = 0x00000040;
    pub add_cdb: [u8; 24],
    pub data_len: u32,
    pub sts: atto_csmi_ssp_passthru_sts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_stp_passthru_sts {
    pub conn_sts: u8,
    pub reserved: [u8; 3],
    pub sts_fis: [u8; 20],
    pub scr: [u32; 16],
    pub data_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_stp_passthru {
    pub phy_id: u8,
    pub port_id: u8,
    pub conn_rate: u8,
    pub reserved: u8,
    pub dest_sas_addr: [u8; 8],
    pub reserved2: [u8; 4],
    pub command_fis: [u8; 20],
    pub flags: u32,
pub const CSMI_STPF_DD_READ: c_uint = 0x00000001;
pub const CSMI_STPF_DD_WRITE: c_uint = 0x00000002;
pub const CSMI_STPF_DD_UNSPECIFIED: c_uint = 0x00000004;
pub const CSMI_STPF_PIO: c_uint = 0x00000010;
pub const CSMI_STPF_DMA: c_uint = 0x00000020;
pub const CSMI_STPF_PACKET: c_uint = 0x00000040;
pub const CSMI_STPF_DMA_QUEUED: c_uint = 0x00000080;
pub const CSMI_STPF_EXECUTE_DIAG: c_uint = 0x00000100;
pub const CSMI_STPF_RESET_DEVICE: c_uint = 0x00000200;
    pub data_len: u32,
    pub sts: atto_csmi_stp_passthru_sts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_sata_sig {
    pub phy_id: u8,
    pub reserved: [u8; 3],
    pub reg_dth_fis: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_scsi_addr {
    pub sas_addr: [u8; 8],
    pub sas_lun: [u8; 8],
    pub host_index: u8,
    pub path_id: u8,
    pub target_id: u8,
    pub lun: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_dev_addr {
    pub host_index: u8,
    pub path_id: u8,
    pub target_id: u8,
    pub lun: u8,
    pub sas_addr: [u8; 8],
    pub sas_lun: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_task_mgmt {
    pub host_index: u8,
    pub path_id: u8,
    pub target_id: u8,
    pub lun: u8,
    pub flags: u32,
pub const CSMI_TMF_TASK_IU: c_uint = 0x00000001;
pub const CSMI_TMF_HARD_RST: c_uint = 0x00000002;
pub const CSMI_TMF_SUPPRESS_RSLT: c_uint = 0x00000004;
    pub queue_tag: u32,
    pub reserved: u32,
    pub task_mgt_func: u8,
    pub reserved2: [u8; 7],
    pub information: u32,
pub const CSMI_TM_INFO_TEST: c_int = 1;
pub const CSMI_TM_INFO_EXCEEDED: c_int = 2;
pub const CSMI_TM_INFO_DEMAND: c_int = 3;
pub const CSMI_TM_INFO_TRIGGER: c_int = 4;
    pub sts: atto_csmi_ssp_passthru_sts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_get_conn_info {
    pub pinout: u32,
pub const CSMI_CON_UNKNOWN: c_uint = 0x00000001;
pub const CSMI_CON_SFF_8482: c_uint = 0x00000002;
pub const CSMI_CON_SFF_8470_LANE_1: c_uint = 0x00000100;
pub const CSMI_CON_SFF_8470_LANE_2: c_uint = 0x00000200;
pub const CSMI_CON_SFF_8470_LANE_3: c_uint = 0x00000400;
pub const CSMI_CON_SFF_8470_LANE_4: c_uint = 0x00000800;
pub const CSMI_CON_SFF_8484_LANE_1: c_uint = 0x00010000;
pub const CSMI_CON_SFF_8484_LANE_2: c_uint = 0x00020000;
pub const CSMI_CON_SFF_8484_LANE_3: c_uint = 0x00040000;
pub const CSMI_CON_SFF_8484_LANE_4: c_uint = 0x00080000;
    pub connector: [u8; 16],
    pub location: u8,
pub const CSMI_CON_INTERNAL: c_uint = 0x02;
pub const CSMI_CON_EXTERNAL: c_uint = 0x04;
pub const CSMI_CON_SWITCHABLE: c_uint = 0x08;
pub const CSMI_CON_AUTO: c_uint = 0x10;
    pub reserved: [u8; 15],
}

// CSMI PHY class structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_character {
    pub type_flags: u8,
pub const CSMI_CTF_POS_DISP: c_uint = 0x01;
pub const CSMI_CTF_NEG_DISP: c_uint = 0x02;
pub const CSMI_CTF_CTRL_CHAR: c_uint = 0x04;
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_pc_ctrl {
    pub type: u8,
pub const CSMI_PC_TYPE_UNDEFINED: c_uint = 0x00;
pub const CSMI_PC_TYPE_SATA: c_uint = 0x01;
pub const CSMI_PC_TYPE_SAS: c_uint = 0x02;
    pub rate: u8,
    pub reserved: [u8; 6],
    pub vendor_unique: [u32; 8],
    pub tx_flags: u32,
pub const CSMI_PC_TXF_PREEMP_DIS: c_uint = 0x00000001;
    pub tx_amplitude: signed char,
    pub tx_preemphasis: signed char,
    pub tx_slew_rate: signed char,
    pub tx_reserved: [signed char; 13],
    pub tx_vendor_unique: [u8; 64],
    pub rx_flags: u32,
pub const CSMI_PC_RXF_EQ_DIS: c_uint = 0x00000001;
    pub rx_threshold: signed char,
    pub rx_equalization_gain: signed char,
    pub rx_reserved: [signed char; 14],
    pub rx_vendor_unique: [u8; 64],
    pub pattern_flags: u32,
pub const CSMI_PC_PATF_FIXED: c_uint = 0x00000001;
pub const CSMI_PC_PATF_DIS_SCR: c_uint = 0x00000002;
pub const CSMI_PC_PATF_DIS_ALIGN: c_uint = 0x00000004;
pub const CSMI_PC_PATF_DIS_SSC: c_uint = 0x00000008;
    pub fixed_pattern: u8,
pub const CSMI_PC_FP_CJPAT: c_uint = 0x00000001;
pub const CSMI_PC_FP_ALIGN: c_uint = 0x00000002;
    pub user_pattern_len: u8,
    pub pattern_reserved: [u8; 6],
    pub user_pattern_buffer: [atto_csmi_character; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi_phy_ctrl {
    pub function: u32,
pub const CSMI_PC_FUNC_GET_SETUP: c_uint = 0x00000100;
    pub phy_id: u8,
    pub len_of_cntl: u16,
    pub num_of_cntls: u8,
    pub reserved: [u8; 4],
    pub link_flags: u32,
pub const CSMI_PHY_ACTIVATE_CTRL: c_uint = 0x00000001;
pub const CSMI_PHY_UPD_SPINUP_RATE: c_uint = 0x00000002;
pub const CSMI_PHY_AUTO_COMWAKE: c_uint = 0x00000004;
    pub spinup_rate: u8,
    pub link_reserved: [u8; 7],
    pub vendor_unique: [u32; 8],
    pub control: [atto_csmi_pc_ctrl; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_ioctl_csmi {
    pub drvr_info: atto_csmi_get_driver_info,
    pub cntlr_cfg: atto_csmi_get_cntlr_cfg,
    pub cntlr_sts: atto_csmi_get_cntlr_sts,
    pub fw_dwnld: atto_csmi_fw_download,
    pub raid_info: atto_csmi_get_raid_info,
    pub raid_cfg: atto_csmi_get_raid_cfg,
    pub get_phy_info: atto_csmi_get_phy_info,
    pub set_phy_info: atto_csmi_set_phy_info,
    pub link_errs: atto_csmi_get_link_errors,
    pub smp_pass_thru: atto_csmi_smp_passthru,
    pub ssp_pass_thru: atto_csmi_ssp_passthru,
    pub stp_pass_thru: atto_csmi_stp_passthru,
    pub tsk_mgt: atto_csmi_task_mgmt,
    pub sata_sig: atto_csmi_get_sata_sig,
    pub scsi_addr: atto_csmi_get_scsi_addr,
    pub dev_addr: atto_csmi_get_dev_addr,
    pub conn_info: [atto_csmi_get_conn_info; 32],
    pub phy_ctrl: atto_csmi_phy_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_csmi {
    pub control_code: u32,
    pub status: u32,
    pub data: atto_ioctl_csmi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_module_info {
    pub adapter: *mut c_void,
    pub pci_dev: *mut c_void,
    pub scsi_host: *mut c_void,
    pub host_no: c_ushort,
    pub node_name: u64,
    pub port_name: u64,
}

pub const ATTO_FUNC_GET_ADAP_INFO: c_uint = 0x00;
pub const ATTO_VER_GET_ADAP_INFO0: c_int = 0;

pub const ATTO_GAI_PCILW_UNKNOWN: c_uint = 0x00;
pub const ATTO_GAI_PCILS_UNKNOWN: c_uint = 0x00;
pub const ATTO_GAI_PCILS_GEN1: c_uint = 0x01;
pub const ATTO_GAI_PCILS_GEN2: c_uint = 0x02;
pub const ATTO_GAI_PCILS_GEN3: c_uint = 0x03;
pub const ATTO_GAI_PCIIM_UNKNOWN: c_uint = 0x00;
pub const ATTO_GAI_PCIIM_LEGACY: c_uint = 0x01;
pub const ATTO_GAI_PCIIM_MSI: c_uint = 0x02;
pub const ATTO_GAI_PCIIM_MSIX: c_uint = 0x03;
pub const ATTO_GAI_AT_EPCIU320: c_uint = 0x00;
pub const ATTO_GAI_AT_ESASRAID: c_uint = 0x01;
pub const ATTO_GAI_AT_ESASRAID2: c_uint = 0x02;
pub const ATTO_GAI_AT_ESASHBA: c_uint = 0x03;
pub const ATTO_GAI_AT_ESASHBA2: c_uint = 0x04;
pub const ATTO_GAI_AT_CELERITY: c_uint = 0x05;
pub const ATTO_GAI_AT_CELERITY8: c_uint = 0x06;
pub const ATTO_GAI_AT_FASTFRAME: c_uint = 0x07;
pub const ATTO_GAI_AT_ESASHBA3: c_uint = 0x08;
pub const ATTO_GAI_AT_CELERITY16: c_uint = 0x09;
pub const ATTO_GAI_AT_TLSASHBA: c_uint = 0x0A;
pub const ATTO_GAI_AT_ESASHBA4: c_uint = 0x0B;
pub const ATTO_GAI_AF_DEGRADED: c_uint = 0x01;
pub const ATTO_GAI_AF_SPT_SUPP: c_uint = 0x02;
pub const ATTO_GAI_AF_DEVADDR_SUPP: c_uint = 0x04;
pub const ATTO_GAI_AF_PHYCTRL_SUPP: c_uint = 0x08;
pub const ATTO_GAI_AF_TEST_SUPP: c_uint = 0x10;
pub const ATTO_GAI_AF_DIAG_SUPP: c_uint = 0x20;
pub const ATTO_GAI_AF_VIRT_SES: c_uint = 0x40;
pub const ATTO_GAI_AF_CONN_CTRL: c_uint = 0x80;
pub const ATTO_GAI_AF2_FCOE_SUPP: c_uint = 0x01;
pub const ATTO_GAI_AF2_NIC_SUPP: c_uint = 0x02;
pub const ATTO_GAI_AF2_LOCATE_SUPP: c_uint = 0x04;
pub const ATTO_GAI_AF2_ADAP_CTRL_SUPP: c_uint = 0x08;
pub const ATTO_GAI_AF2_DEV_INFO_SUPP: c_uint = 0x10;
pub const ATTO_GAI_AF2_NPIV_SUPP: c_uint = 0x20;
pub const ATTO_GAI_AF2_MP_SUPP: c_uint = 0x40;
pub const ATTO_GAI_TF_MEM_RW: c_uint = 0x00000001;
pub const ATTO_GAI_TF_TRACE: c_uint = 0x00000002;
pub const ATTO_GAI_TF_SCSI_PASS_THRU: c_uint = 0x00000004;
pub const ATTO_GAI_TF_GET_DEV_ADDR: c_uint = 0x00000008;
pub const ATTO_GAI_TF_PHY_CTRL: c_uint = 0x00000010;
pub const ATTO_GAI_TF_CONN_CTRL: c_uint = 0x00000020;
pub const ATTO_GAI_TF_GET_DEV_INFO: c_uint = 0x00000040;
pub const ATTO_FUNC_GET_ADAP_ADDR: c_uint = 0x01;
pub const ATTO_VER_GET_ADAP_ADDR0: c_int = 0;

pub const ATTO_GAA_AT_PORT: c_uint = 0x00;
pub const ATTO_GAA_AT_NODE: c_uint = 0x01;
pub const ATTO_GAA_AT_CURR_MAC: c_uint = 0x02;
pub const ATTO_GAA_AT_PERM_MAC: c_uint = 0x03;
pub const ATTO_GAA_AT_VNIC: c_uint = 0x04;
pub const ATTO_FUNC_MEM_RW: c_uint = 0x02;
pub const ATTO_VER_MEM_RW0: c_int = 0;

pub const ATTO_FUNC_TRACE: c_uint = 0x03;
pub const ATTO_VER_TRACE0: c_int = 0;
pub const ATTO_VER_TRACE1: c_int = 1;

pub const ATTO_TRC_TF_GET_INFO: c_uint = 0x00;
pub const ATTO_TRC_TF_ENABLE: c_uint = 0x01;
pub const ATTO_TRC_TF_DISABLE: c_uint = 0x02;
pub const ATTO_TRC_TF_SET_MASK: c_uint = 0x03;
pub const ATTO_TRC_TF_UPLOAD: c_uint = 0x04;
pub const ATTO_TRC_TF_RESET: c_uint = 0x05;
pub const ATTO_TRC_TT_DRIVER: c_uint = 0x00;
pub const ATTO_TRC_TT_FWCOREDUMP: c_uint = 0x01;
pub const ATTO_FUNC_SCSI_PASS_THRU: c_uint = 0x04;
pub const ATTO_VER_SCSI_PASS_THRU0: c_int = 0;

pub const ATTO_SPT_RS_SUCCESS: c_uint = 0x00;
pub const ATTO_SPT_RS_FAILED: c_uint = 0x01;
pub const ATTO_SPT_RS_OVERRUN: c_uint = 0x02;
pub const ATTO_SPT_RS_UNDERRUN: c_uint = 0x03;
pub const ATTO_SPT_RS_NO_DEVICE: c_uint = 0x04;
pub const ATTO_SPT_RS_NO_LUN: c_uint = 0x05;
pub const ATTO_SPT_RS_TIMEOUT: c_uint = 0x06;
pub const ATTO_SPT_RS_BUS_RESET: c_uint = 0x07;
pub const ATTO_SPT_RS_ABORTED: c_uint = 0x08;
pub const ATTO_SPT_RS_BUSY: c_uint = 0x09;
pub const ATTO_SPT_RS_DEGRADED: c_uint = 0x0A;
pub const ATTO_SPTF_DATA_IN: c_uint = 0x00000001;
pub const ATTO_SPTF_DATA_OUT: c_uint = 0x00000002;
pub const ATTO_SPTF_SIMPLE_Q: c_uint = 0x00000004;
pub const ATTO_SPTF_HEAD_OF_Q: c_uint = 0x00000008;
pub const ATTO_SPTF_ORDERED_Q: c_uint = 0x00000010;
pub const ATTO_FUNC_GET_DEV_ADDR: c_uint = 0x05;
pub const ATTO_VER_GET_DEV_ADDR0: c_int = 0;

pub const ATTO_GDA_AT_PORT: c_uint = 0x00;
pub const ATTO_GDA_AT_NODE: c_uint = 0x01;
pub const ATTO_GDA_AT_MAC: c_uint = 0x02;
pub const ATTO_GDA_AT_PORTID: c_uint = 0x03;
pub const ATTO_GDA_AT_UNIQUE: c_uint = 0x04;
// The following functions are supported by firmware but do not have any
// associated driver structures
//
pub const ATTO_FUNC_PHY_CTRL: c_uint = 0x06;
pub const ATTO_FUNC_CONN_CTRL: c_uint = 0x0C;
pub const ATTO_FUNC_ADAP_CTRL: c_uint = 0x0E;
pub const ATTO_VER_ADAP_CTRL0: c_int = 0;

pub const ATTO_AC_AF_HARD_RST: c_uint = 0x00;
pub const ATTO_AC_AF_GET_STATE: c_uint = 0x01;
pub const ATTO_AC_AF_GET_TEMP: c_uint = 0x02;
pub const ATTO_AC_AS_UNKNOWN: c_uint = 0x00;
pub const ATTO_AC_AS_OK: c_uint = 0x01;
pub const ATTO_AC_AS_RST_SCHED: c_uint = 0x02;
pub const ATTO_AC_AS_RST_IN_PROG: c_uint = 0x03;
pub const ATTO_AC_AS_RST_DISC: c_uint = 0x04;
pub const ATTO_AC_AS_DEGRADED: c_uint = 0x05;
pub const ATTO_AC_AS_DISABLED: c_uint = 0x06;
pub const ATTO_AC_AS_TEMP: c_uint = 0x07;
pub const ATTO_AC_TS_UNSUPP: c_uint = 0x00;
pub const ATTO_AC_TS_UNKNOWN: c_uint = 0x01;
pub const ATTO_AC_TS_INIT_FAILED: c_uint = 0x02;
pub const ATTO_AC_TS_NORMAL: c_uint = 0x03;
pub const ATTO_AC_TS_OUT_OF_RANGE: c_uint = 0x04;
pub const ATTO_AC_TS_FAULT: c_uint = 0x05;
pub const ATTO_FUNC_GET_DEV_INFO: c_uint = 0x0F;
pub const ATTO_VER_GET_DEV_INFO0: c_int = 0;

pub const ATTO_SDI_MAX_PHYS_WIDE_PORT: c_int = 16;

pub const ATTO_SDI_SAS_LVL_INV: c_uint = 0xFF;

pub const ATTO_SDI_DT_END_DEVICE: c_int = 0;
pub const ATTO_SDI_DT_EXPANDER: c_int = 1;
pub const ATTO_SDI_DT_PORT_MULT: c_int = 2;
pub const ATTO_SDI_LF_DIRECT: c_uint = 0x01;
pub const ATTO_SDI_LF_EXPANDER: c_uint = 0x02;
pub const ATTO_SDI_LF_PORT_MULT: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub union atto_hba_device_info {
    pub sas_dev_info: atto_hba_sas_device_info,
}

pub const ATTO_GDI_IT_UNKNOWN: c_uint = 0x00;
pub const ATTO_GDI_IT_SAS: c_uint = 0x01;
pub const ATTO_GDI_IT_FC: c_uint = 0x02;
pub const ATTO_GDI_IT_FCOE: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atto_ioctl {
    pub version: u8,
    pub /: *mut *mut u8 function; / ATTO_FUNC_XXX,
    pub status: u8,
pub const ATTO_STS_SUCCESS: c_uint = 0x00;
pub const ATTO_STS_FAILED: c_uint = 0x01;
pub const ATTO_STS_INV_VERSION: c_uint = 0x02;
pub const ATTO_STS_OUT_OF_RSRC: c_uint = 0x03;
pub const ATTO_STS_INV_FUNC: c_uint = 0x04;
pub const ATTO_STS_UNSUPPORTED: c_uint = 0x05;
pub const ATTO_STS_INV_ADAPTER: c_uint = 0x06;
pub const ATTO_STS_INV_DRVR_VER: c_uint = 0x07;
pub const ATTO_STS_INV_PARAM: c_uint = 0x08;
pub const ATTO_STS_TIMEOUT: c_uint = 0x09;
pub const ATTO_STS_NOT_APPL: c_uint = 0x0A;
pub const ATTO_STS_DEGRADED: c_uint = 0x0B;
    pub flags: u8,
pub const HBAF_TUNNEL: c_uint = 0x01;
    pub data_length: u32,
    pub reserved2: [u8; 56],
    pub byte: [u8; 1],
    pub get_adap_info: atto_hba_get_adapter_info,
    pub get_adap_addr: atto_hba_get_adapter_address,
    pub scsi_pass_thru: atto_hba_scsi_pass_thru,
    pub get_dev_addr: atto_hba_get_device_address,
    pub adap_ctrl: atto_hba_adap_ctrl,
    pub get_dev_info: atto_hba_get_device_info,
    pub trace: atto_hba_trace,
    pub data: },
}

pub const ATTO_VDA_SCSI_VER0: c_int = 0;

pub const ATTO_VDA_FLASH_VER0: c_int = 0;

pub const ATTO_VDA_DIAG_VER0: c_int = 0;

pub const ATTO_VDA_CLI_VER0: c_int = 0;

pub const ATTO_VDA_SMP_VER0: c_int = 0;

pub const ATTO_VDA_CFG_VER0: c_int = 0;

pub const ATTO_VDA_MGT_VER0: c_int = 0;

pub const ATTO_VDA_GSV_VER0: c_int = 0;

pub const ATTO_VDA_VER_UNSUPPORTED: c_uint = 0xFF;
pub const ATTO_SMP_VERSION0: c_int = 0;
pub const ATTO_SMP_VERSION1: c_int = 1;
pub const ATTO_SMP_VERSION2: c_int = 2;

pub const ATTO_SMP_FUNC_DISC_SMP: c_uint = 0x00;
pub const ATTO_SMP_FUNC_DISC_TARG: c_uint = 0x01;
pub const ATTO_SMP_FUNC_SEND_CMD: c_uint = 0x02;
pub const ATTO_SMP_FUNC_DISC_TARG_DIRECT: c_uint = 0x03;
pub const ATTO_SMP_FUNC_SEND_CMD_DIRECT: c_uint = 0x04;
pub const ATTO_SMP_FUNC_DISC_SMP_DIRECT: c_uint = 0x05;
pub const ATTO_SMP_STS_SUCCESS: c_uint = 0x00;
pub const ATTO_SMP_STS_FAILURE: c_uint = 0x01;
pub const ATTO_SMP_STS_RESCAN: c_uint = 0x02;
pub const ATTO_SMP_STS_NOT_FOUND: c_uint = 0x03;
pub const ATTO_SMPF_ROOT_EXP: c_uint = 0x01 /* expander direct attached */;
// The struct associated with the code is listed after the definition
pub const EXPRESS_IOCTL_MIN: c_uint = 0x4500;
pub const EXPRESS_IOCTL_RW_FIRMWARE: c_uint = 0x4500            /* FIRMWARERW    */;
pub const EXPRESS_IOCTL_READ_PARAMS: c_uint = 0x4501            /* PARAMRW       */;
pub const EXPRESS_IOCTL_WRITE_PARAMS: c_uint = 0x4502            /* PARAMRW       */;
pub const EXPRESS_IOCTL_FC_API: c_uint = 0x4503            /* internal      */;
pub const EXPRESS_IOCTL_GET_CHANNELS: c_uint = 0x4504            /* CHANNELLIST   */;
pub const EXPRESS_IOCTL_CHAN_INFO: c_uint = 0x4505            /* CHANNELINFO   */;
pub const EXPRESS_IOCTL_DEFAULT_PARAMS: c_uint = 0x4506            /* PARAMRW       */;
pub const EXPRESS_ADDR_MEMORY: c_uint = 0x4507            /* MEMADDR       */;
pub const EXPRESS_RW_MEMORY: c_uint = 0x4508            /* MEMRW         */;
pub const EXPRESS_TSDK_DUMP: c_uint = 0x4509            /* TSDKDUMP      */;
pub const EXPRESS_IOCTL_SMP: c_uint = 0x450A            /* IOCTL_SMP     */;
pub const EXPRESS_CSMI: c_uint = 0x450B            /* CSMI          */;
pub const EXPRESS_IOCTL_HBA: c_uint = 0x450C            /* IOCTL_HBA     */;
pub const EXPRESS_IOCTL_VDA: c_uint = 0x450D            /* IOCTL_VDA     */;
pub const EXPRESS_IOCTL_GET_ID: c_uint = 0x450E            /* GET_ID        */;
pub const EXPRESS_IOCTL_GET_MOD_INFO: c_uint = 0x450F            /* MODULE_INFO   */;
pub const EXPRESS_IOCTL_MAX: c_uint = 0x450F;
