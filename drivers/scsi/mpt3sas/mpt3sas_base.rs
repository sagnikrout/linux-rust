//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpt3sas_base.h
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
// This is the Fusion MPT base driver providing common API layer interface
// for access to MPT (Message Passing Technology) firmware.
//
// This code is based on drivers/scsi/mpt3sas/mpt3sas_base.h
// Copyright (C) 2012-2014  LSI Corporation
// Copyright (C) 2013-2014 Avago Technologies
// (mailto: MPT-FusionLinux.pdl@avagotech.com)
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
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301,
// USA.
//

// Macro flag: #define MPT3SAS_BASE_H_INCLUDED

// driver versioning info

pub const MPT3SAS_MAJOR_VERSION: c_int = 54;
pub const MPT3SAS_MINOR_VERSION: c_int = 100;
pub const MPT3SAS_BUILD_VERSION: c_int = 00;
pub const MPT3SAS_RELEASE_VERSION: c_int = 00;

pub const MPT2SAS_MAJOR_VERSION: c_int = 20;
pub const MPT2SAS_MINOR_VERSION: c_int = 102;
pub const MPT2SAS_BUILD_VERSION: c_int = 0;
pub const MPT2SAS_RELEASE_VERSION: c_int = 00;
// CoreDump: Default timeout

//
// Set MPT3SAS_SG_DEPTH value based on user input.
//

pub const MPT_MIN_PHYS_SEGMENTS: c_int = 16;
pub const MPT_KDUMP_MIN_PHYS_SEGMENTS: c_int = 32;
pub const MCPU_MAX_CHAINS_PER_IO: c_int = 3;

//
// Generic Defines
//
pub const MPT3SAS_SATA_QUEUE_DEPTH: c_int = 32;
pub const MPT3SAS_SAS_QUEUE_DEPTH: c_int = 254;
pub const MPT3SAS_RAID_QUEUE_DEPTH: c_int = 128;
pub const MPT3SAS_KDUMP_SCSI_IO_DEPTH: c_int = 200;
pub const MPT3SAS_RAID_MAX_SECTORS: c_int = 8192;
pub const MPT3SAS_HOST_PAGE_SIZE_4K: c_int = 12;
pub const MPT3SAS_NVME_QUEUE_DEPTH: c_int = 128;

pub const MPT_STRING_LENGTH: c_int = 64;
pub const MPI_FRAME_START_OFFSET: c_int = 256;

pub const MPT_MAX_CALLBACKS: c_int = 32;
pub const MPT_MAX_HBA_NUM_PHYS: c_int = 32;

// reserved for issuing internally framed scsi io cmds
pub const INTERNAL_SCSIIO_CMDS_COUNT: c_int = 3;
pub const INTERNAL_SCSIIO_FOR_DISCOVERY: c_int = 2;
pub const MPI3_HIM_MASK: c_uint = 0xFFFFFFFF /* mask every bit*/;
pub const MPT3SAS_INVALID_DEVICE_HANDLE: c_uint = 0xFFFF;
pub const MAX_CHAIN_ELEMT_SZ: c_int = 16;
pub const DEFAULT_NUM_FWCHAIN_ELEMTS: c_int = 8;
pub const IO_UNIT_CONTROL_SHUTDOWN_TIMEOUT: c_int = 6;
pub const FW_IMG_HDR_READ_TIMEOUT: c_int = 15;
pub const IOC_OPERATIONAL_WAIT_COUNT: c_int = 10;
//
// NVMe defines
//

pub const NVME_TASK_ABORT_MIN_TIMEOUT: c_int = 6;
pub const NVME_TASK_ABORT_MAX_TIMEOUT: c_int = 60;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3sas_nvme_cmd {
    pub rsvd: [u8; 24],
    pub prp1: __le64,
    pub prp2: __le64,
}

//
// logging format
//

//
// WarpDrive Specific Log codes
//

//
// per target private data
//
pub const MPT_TARGET_FLAGS_RAID_COMPONENT: c_uint = 0x01;
pub const MPT_TARGET_FLAGS_VOLUME: c_uint = 0x02;
pub const MPT_TARGET_FLAGS_DELETED: c_uint = 0x04;
pub const MPT_TARGET_FASTPATH_IO: c_uint = 0x08;
pub const MPT_TARGET_FLAGS_PCIE_DEVICE: c_uint = 0x10;

// Atlas PCIe Switch Management Port

//
// Intel HBA branding
//

//
// Intel HBA SSDIDs
//
pub const MPT2SAS_INTEL_RMS25JB080_SSDID: c_uint = 0x3516;
pub const MPT2SAS_INTEL_RMS25JB040_SSDID: c_uint = 0x3517;
pub const MPT2SAS_INTEL_RMS25KB080_SSDID: c_uint = 0x3518;
pub const MPT2SAS_INTEL_RMS25KB040_SSDID: c_uint = 0x3519;
pub const MPT2SAS_INTEL_RMS25LB040_SSDID: c_uint = 0x351A;
pub const MPT2SAS_INTEL_RMS25LB080_SSDID: c_uint = 0x351B;
pub const MPT2SAS_INTEL_RMS2LL080_SSDID: c_uint = 0x350E;
pub const MPT2SAS_INTEL_RMS2LL040_SSDID: c_uint = 0x350F;
pub const MPT2SAS_INTEL_RS25GB008_SSDID: c_uint = 0x3000;
pub const MPT2SAS_INTEL_SSD910_SSDID: c_uint = 0x3700;
pub const MPT3SAS_INTEL_RMS3JC080_SSDID: c_uint = 0x3521;
pub const MPT3SAS_INTEL_RS3GC008_SSDID: c_uint = 0x3522;
pub const MPT3SAS_INTEL_RS3FC044_SSDID: c_uint = 0x3523;
pub const MPT3SAS_INTEL_RS3UC080_SSDID: c_uint = 0x3524;
//
// Dell HBA branding
//
pub const MPT2SAS_DELL_BRANDING_SIZE: c_int = 32;

//
// Dell HBA SSDIDs
//
pub const MPT2SAS_DELL_6GBPS_SAS_HBA_SSDID: c_uint = 0x1F1C;
pub const MPT2SAS_DELL_PERC_H200_ADAPTER_SSDID: c_uint = 0x1F1D;
pub const MPT2SAS_DELL_PERC_H200_INTEGRATED_SSDID: c_uint = 0x1F1E;
pub const MPT2SAS_DELL_PERC_H200_MODULAR_SSDID: c_uint = 0x1F1F;
pub const MPT2SAS_DELL_PERC_H200_EMBEDDED_SSDID: c_uint = 0x1F20;
pub const MPT2SAS_DELL_PERC_H200_SSDID: c_uint = 0x1F21;
pub const MPT2SAS_DELL_6GBPS_SAS_SSDID: c_uint = 0x1F22;
pub const MPT3SAS_DELL_12G_HBA_SSDID: c_uint = 0x1F46;
//
// Cisco HBA branding
//

//
// Cisco HBA SSSDIDs
//
pub const MPT3SAS_CISCO_12G_8E_HBA_SSDID: c_uint = 0x14C;
pub const MPT3SAS_CISCO_12G_8I_HBA_SSDID: c_uint = 0x154;
pub const MPT3SAS_CISCO_12G_AVILA_HBA_SSDID: c_uint = 0x155;
pub const MPT3SAS_CISCO_12G_COLUSA_MEZZANINE_HBA_SSDID: c_uint = 0x156;
//
// status bits for ioc->diag_buffer_status
//

//
// HP HBA branding
//
pub const MPT2SAS_HP_3PAR_SSVID: c_uint = 0x1590;

//
// HO HBA SSDIDs
//
pub const MPT2SAS_HP_2_4_INTERNAL_SSDID: c_uint = 0x0041;
pub const MPT2SAS_HP_2_4_EXTERNAL_SSDID: c_uint = 0x0042;
pub const MPT2SAS_HP_1_4_INTERNAL_1_4_EXTERNAL_SSDID: c_uint = 0x0043;
pub const MPT2SAS_HP_EMBEDDED_2_4_INTERNAL_SSDID: c_uint = 0x0044;
pub const MPT2SAS_HP_DAUGHTER_2_4_INTERNAL_SSDID: c_uint = 0x0046;
//
// Combined Reply Queue constants,
// There are twelve Supplemental Reply Post Host Index Registers
// and each register is at offset 0x10 bytes from the previous one.
//

pub const MPT3_SUP_REPLY_POST_HOST_INDEX_REG_COUNT_G3: c_int = 12;
pub const MPT3_SUP_REPLY_POST_HOST_INDEX_REG_COUNT_G35: c_int = 16;

pub const MPT3_MIN_IRQS: c_int = 1;
// OEM Identifiers

// GENERIC Flags 0

// High IOPs definitions
pub const MPT3SAS_DEVICE_HIGH_IOPS_DEPTH: c_int = 8;
pub const MPT3SAS_HIGH_IOPS_REPLY_QUEUES: c_int = 8;
pub const MPT3SAS_HIGH_IOPS_BATCH_COUNT: c_int = 16;
pub const MPT3SAS_GEN35_MAX_MSIX_QUEUES: c_int = 128;
pub const RDPQ_MAX_INDEX_IN_ONE_CHUNK: c_int = 16;
// OEM Specific Flags will come from OEM specific header files
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mpi2ManufacturingPage10_t {
    pub /: *mut *mut MPI2_CONFIG_PAGE_HEADER Header; / 00h,
    pub /: *mut *mut U8 OEMIdentifier; / 04h,
    pub /: *mut *mut U8 Reserved1; / 05h,
    pub /: *mut *mut U16 Reserved2; / 08h,
    pub /: *mut *mut U32 Reserved3; / 0Ch,
    pub /: *mut *mut U32 GenericFlags0; / 10h,
    pub /: *mut *mut U32 GenericFlags1; / 14h,
    pub /: *mut *mut U32 Reserved4; / 18h,
    pub /: *mut *mut U32 OEMSpecificFlags0; / 1Ch,
    pub /: *mut *mut U32 OEMSpecificFlags1; / 20h,
    pub 60h*/: *mut *mut U32 Reserved5[18]; / 24h -,
}

// Miscellaneous options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mpi2ManufacturingPage11_t {
    pub /: *mut *mut MPI2_CONFIG_PAGE_HEADER Header; / 00h,
    pub /: *mut *mut __le32 Reserved1; / 04h,
    pub /: *mut *mut u8 Reserved2; / 08h,
    pub /: *mut *mut u8 EEDPTagMode; / 09h,
    pub /: *mut *mut u8 Reserved3; / 0Ah,
    pub /: *mut *mut u8 Reserved4; / 0Bh,
    pub /: *mut *mut __le32 Reserved5[8]; / 0Ch-2Ch,
    pub /: *mut *mut u16 AddlFlags2; / 2Ch,
    pub /: *mut *mut u8 AddlFlags3; / 2Eh,
    pub /: *mut *mut u8 Reserved6; / 2Fh,
    pub /: *mut *mut __le32 Reserved7[7]; / 30h - 4Bh,
    pub /: *mut *mut u8 NVMeAbortTO; / 4Ch,
    pub /: *mut *mut u8 NumPerDevEvents; / 4Dh,
    pub /: *mut *mut u8 HostTraceBufferDecrementSizeKB; / 4Eh,
    pub /: *mut *mut u8 HostTraceBufferFlags; / 4Fh,
    pub /: *mut *mut u16 HostTraceBufferMaxSizeKB; / 50h,
    pub /: *mut *mut u16 HostTraceBufferMinSizeKB; / 52h,
    pub /: *mut *mut u8 CoreDumpTOSec; / 54h,
    pub /: *mut *mut u8 TimeSyncInterval; / 55h,
    pub /: *mut *mut u16 Reserved9; / 56h,
    pub /: *mut *mut __le32 Reserved10; / 58h,
}

//
// struct MPT3SAS_TARGET - starget private hostdata
// @starget: starget object
// @sas_address: target sas address
// @raid_device: raid_device pointer to access volume data
// @handle: device handle
// @num_luns: number luns
// @flags: MPT_TARGET_FLAGS_XXX flags
// @deleted: target flaged for deletion
// @tm_busy: target is busy with TM request.
// @port: hba port entry containing target's port number info
// @sas_dev: The sas_device associated with this target
// @pcie_dev: The pcie device associated with this target
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPT3SAS_TARGET {
    pub starget: *mut scsi_target,
    pub sas_address: u64,
    pub raid_device: *mut _raid_device,
    pub handle: u16,
    pub num_luns: c_int,
    pub flags: u32,
    pub deleted: u8,
    pub tm_busy: u8,
    pub port: *mut hba_port,
    pub sas_dev: *mut _sas_device,
    pub pcie_dev: *mut _pcie_device,
}

//
// per device private data
//
pub const MPT_DEVICE_FLAGS_INIT: c_uint = 0x01;

//
// struct MPT3SAS_DEVICE - sdev private hostdata
// @sas_target: starget private hostdata
// @lun: lun number
// @flags: MPT_DEVICE_XXX flags
// @configured_lun: lun is configured
// @block: device is in SDEV_BLOCK state
// @tlr_snoop_check: flag used in determining whether to disable TLR
// @eedp_enable: eedp support enable bit
// @eedp_type: 0(type_1), 1(type_2), 2(type_3)
// @eedp_block_length: block size
// @ata_command_pending: SATL passthrough outstanding for device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPT3SAS_DEVICE {
    pub sas_target: *mut MPT3SAS_TARGET,
    pub lun: c_uint,
    pub flags: u32,
    pub configured_lun: u8,
    pub block: u8,
    pub deleted: u8,
    pub tlr_snoop_check: u8,
    pub ignore_delay_remove: u8,
// Iopriority Command Handling
    pub ncq_prio_enable: u8,
//
// Bug workaround for SATL handling: the mpt2/3sas firmware
// doesn't return BUSY or TASK_SET_FULL for subsequent
// commands while a SATL pass through is in operation as the
// spec requires, it simply does nothing with them until the
// pass through completes, causing them possibly to timeout if
// the passthrough is a long executing command (like format or
// secure erase).  This variable allows us to do the right
// thing while a SATL command is pending.
//
    pub ata_command_pending: c_ulong,
}

pub const MPT3_CMD_NOT_USED: c_uint = 0x8000	/* free */;
pub const MPT3_CMD_COMPLETE: c_uint = 0x0001	/* completed */;
pub const MPT3_CMD_PENDING: c_uint = 0x0002	/* pending */;
pub const MPT3_CMD_REPLY_VALID: c_uint = 0x0004	/* reply is valid */;
pub const MPT3_CMD_RESET: c_uint = 0x0008	/* host reset dropped the command */;
pub const MPT3_CMD_COMPLETE_ASYNC: c_uint = 0x0010  /* tells whether cmd completes in same thread or not */;
//
// struct _internal_cmd - internal commands struct
// @mutex: mutex
// @done: completion
// @reply: reply message pointer
// @sense: sense data
// @status: MPT3_CMD_XXX status
// @smid: system message id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _internal_cmd {
    pub mutex: mutex,
    pub done: completion,
    pub reply: *mut c_void,
    pub sense: *mut c_void,
    pub status: u16,
    pub smid: u16,
}

//
// struct _sas_device - attached device information
// @list: sas device list
// @starget: starget object
// @sas_address: device sas address
// @device_name: retrieved from the SAS IDENTIFY frame.
// @handle: device handle
// @sas_address_parent: sas address of parent expander or sas host
// @enclosure_handle: enclosure handle
// @enclosure_logical_id: enclosure logical identifier
// @volume_handle: volume handle (valid when hidden raid member)
// @volume_wwid: volume unique identifier
// @device_info: bitfield provides detailed info about the device
// @id: target id
// @channel: target channel
// @slot: number number
// @phy: phy identifier provided in sas device page 0
// @responding: used in _scsih_sas_device_mark_responding
// @fast_path: fast path feature enable bit
// @pfa_led_on: flag for PFA LED status
// @pend_sas_rphy_add: flag to check if device is in sas_rphy_add()
// addition routine.
// @chassis_slot: chassis slot
// @is_chassis_slot_valid: chassis slot valid or not
// @port: hba port entry containing device's port number info
// @rphy: device's sas_rphy address used to identify this device structure in
// target_alloc callback function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sas_device {
    pub list: list_head,
    pub starget: *mut scsi_target,
    pub sas_address: u64,
    pub device_name: u64,
    pub handle: u16,
    pub sas_address_parent: u64,
    pub enclosure_handle: u16,
    pub enclosure_logical_id: u64,
    pub volume_handle: u16,
    pub volume_wwid: u64,
    pub device_info: u32,
    pub id: c_int,
    pub channel: c_int,
    pub slot: u16,
    pub phy: u8,
    pub responding: u8,
    pub fast_path: u8,
    pub pfa_led_on: u8,
    pub pend_sas_rphy_add: u8,
    pub enclosure_level: u8,
    pub chassis_slot: u8,
    pub is_chassis_slot_valid: u8,
    pub connector_name: [u8; 5],
    pub ssd_device: u8,
    pub refcount: kref,
    pub port_type: u8,
    pub port: *mut hba_port,
    pub rphy: *mut sas_rphy,
}

//
// struct _pcie_device - attached PCIe device information
// @list: pcie device list
// @starget: starget object
// @wwid: device WWID
// @handle: device handle
// @device_info: bitfield provides detailed info about the device
// @id: target id
// @channel: target channel
// @slot: slot number
// @port_num: port number
// @responding: used in _scsih_pcie_device_mark_responding
// @fast_path: fast path feature enable bit
// @nvme_mdts: MaximumDataTransferSize from PCIe Device Page 2 for
// NVMe device only
// @enclosure_handle: enclosure handle
// @enclosure_logical_id: enclosure logical identifier
// @enclosure_level: The level of device's enclosure from the controller
// @connector_name: ASCII value of the Connector's name
// @serial_number: pointer of serial number string allocated runtime
// @access_status: Device's Access Status
// @shutdown_latency: NVMe device's RTD3 Entry Latency
// @refcount: reference count for deletion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _pcie_device {
    pub list: list_head,
    pub starget: *mut scsi_target,
    pub wwid: u64,
    pub handle: u16,
    pub device_info: u32,
    pub id: c_int,
    pub channel: c_int,
    pub slot: u16,
    pub port_num: u8,
    pub responding: u8,
    pub fast_path: u8,
    pub nvme_mdts: u32,
    pub enclosure_handle: u16,
    pub enclosure_logical_id: u64,
    pub enclosure_level: u8,
    pub connector_name: [u8; 4],
    pub serial_number: *mut u8,
    pub reset_timeout: u8,
    pub access_status: u8,
    pub shutdown_latency: u16,
    pub refcount: kref,
}

//
// pcie_device_get - Increment the pcie device reference count
//
// @p: pcie_device object
//
// When ever this function called it will increment the
// reference count of the pcie device for which this function called.
//
// pcie_device_free - Release the pcie device object
// @r - kref object
//
// Free's the pcie device object. It will be called when reference count
// reaches to zero.
//
// pcie_device_put - Decrement the pcie device reference count
//
// @p: pcie_device object
//
// When ever this function called it will decrement the
// reference count of the pcie device for which this function called.
//
// When refernce count reaches to Zero, this will call pcie_device_free to the
// pcie_device object.
//
// struct _raid_device - raid volume link list
// @list: sas device list
// @starget: starget object
// @sdev: scsi device struct (volumes are single lun)
// @wwid: unique identifier for the volume
// @handle: device handle
// @block_size: Block size of the volume
// @id: target id
// @channel: target channel
// @volume_type: the raid level
// @device_info: bitfield provides detailed info about the hidden components
// @num_pds: number of hidden raid components
// @responding: used in _scsih_raid_device_mark_responding
// @percent_complete: resync percent complete
// @direct_io_enabled: Whether direct io to PDs are allowed or not
// @stripe_exponent: X where 2powX is the stripe sz in blocks
// @block_exponent: X where 2powX is the block sz in bytes
// @max_lba: Maximum number of LBA in the volume
// @stripe_sz: Stripe Size of the volume
// @device_info: Device info of the volume member disk
// @pd_handle: Array of handles of the physical drives for direct I/O in le16
//
pub const MPT_MAX_WARPDRIVE_PDS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _raid_device {
    pub list: list_head,
    pub starget: *mut scsi_target,
    pub sdev: *mut scsi_device,
    pub wwid: u64,
    pub handle: u16,
    pub block_sz: u16,
    pub id: c_int,
    pub channel: c_int,
    pub volume_type: u8,
    pub num_pds: u8,
    pub responding: u8,
    pub percent_complete: u8,
    pub direct_io_enabled: u8,
    pub stripe_exponent: u8,
    pub block_exponent: u8,
    pub max_lba: u64,
    pub stripe_sz: u32,
    pub device_info: u32,
    pub pd_handle: [u16; MPT_MAX_WARPDRIVE_PDS],
}

//
// struct _boot_device - boot device info
//
// @channel: sas, raid, or pcie channel
// @device: holds pointer for struct _sas_device, struct _raid_device or
// struct _pcie_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _boot_device {
    pub channel: c_int,
    pub device: *mut c_void,
}

//
// struct _sas_port - wide/narrow sas port information
// @port_list: list of ports belonging to expander
// @num_phys: number of phys belonging to this port
// @remote_identify: attached device identification
// @rphy: sas transport rphy object
// @port: sas transport wide/narrow port object
// @hba_port: hba port entry containing port's port number info
// @phy_list: _sas_phy list objects belonging to this port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sas_port {
    pub port_list: list_head,
    pub num_phys: u8,
    pub remote_identify: sas_identify,
    pub rphy: *mut sas_rphy,
    pub port: *mut sas_port,
    pub hba_port: *mut hba_port,
    pub phy_list: list_head,
}

//
// struct _sas_phy - phy information
// @port_siblings: list of phys belonging to a port
// @identify: phy identification
// @remote_identify: attached device identification
// @phy: sas transport phy object
// @phy_id: unique phy id
// @handle: device handle for this phy
// @attached_handle: device handle for attached device
// @phy_belongs_to_port: port has been created for this phy
// @port: hba port entry containing port number info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sas_phy {
    pub port_siblings: list_head,
    pub identify: sas_identify,
    pub remote_identify: sas_identify,
    pub phy: *mut sas_phy,
    pub phy_id: u8,
    pub handle: u16,
    pub attached_handle: u16,
    pub phy_belongs_to_port: u8,
    pub hba_vphy: u8,
    pub port: *mut hba_port,
}

//
// struct _sas_node - sas_host/expander information
// @list: list of expanders
// @parent_dev: parent device class
// @num_phys: number phys belonging to this sas_host/expander
// @sas_address: sas address of this sas_host/expander
// @handle: handle for this sas_host/expander
// @sas_address_parent: sas address of parent expander or sas host
// @enclosure_handle: handle for this a member of an enclosure
// @device_info: bitwise defining capabilities of this sas_host/expander
// @responding: used in _scsih_expander_device_mark_responding
// @nr_phys_allocated: Allocated memory for this many count phys
// @phy: a list of phys that make up this sas_host/expander
// @sas_port_list: list of ports attached to this sas_host/expander
// @port: hba port entry containing node's port number info
// @rphy: sas_rphy object of this expander
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sas_node {
    pub list: list_head,
    pub parent_dev: *mut device,
    pub num_phys: u8,
    pub sas_address: u64,
    pub handle: u16,
    pub sas_address_parent: u64,
    pub enclosure_handle: u16,
    pub enclosure_logical_id: u64,
    pub responding: u8,
    pub nr_phys_allocated: u8,
    pub port: *mut hba_port,
    pub phy: *mut _sas_phy,
    pub sas_port_list: list_head,
    pub rphy: *mut sas_rphy,
}

//
// struct _enclosure_node - enclosure information
// @list: list of enclosures
// @pg0: enclosure pg0;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _enclosure_node {
    pub list: list_head,
    pub pg0: Mpi2SasEnclosurePage0_t,
}

//
// enum reset_type - reset state
// @FORCE_BIG_HAMMER: issue diagnostic reset
// @SOFT_RESET: issue message_unit_reset, if fails to to big hammer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_type {
    FORCE_BIG_HAMMER,
    SOFT_RESET,
}

//
// struct pcie_sg_list - PCIe SGL buffer (contiguous per I/O)
// @pcie_sgl: PCIe native SGL for NVMe devices
// @pcie_sgl_dma: physical address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_sg_list {
    pub pcie_sgl: *mut c_void,
    pub pcie_sgl_dma: dma_addr_t,
}

//
// struct chain_tracker - firmware chain tracker
// @chain_buffer: chain buffer
// @chain_buffer_dma: physical address
// @tracker_list: list of free request (ioc->free_chain_list)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain_tracker {
    pub chain_buffer: *mut c_void,
    pub chain_buffer_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain_lookup {
    pub chains_per_smid: *mut chain_tracker,
    pub chain_offset: core::sync::atomic::AtomicI32,
}

//
// struct scsiio_tracker - scsi mf request tracker
// @smid: system message id
// @cb_idx: callback index
// @direct_io: To indicate whether I/O is direct (WARPDRIVE)
// @chain_list: list of associated firmware chain tracker
// @msix_io: IO's msix
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsiio_tracker {
    pub smid: u16,
    pub scmd: *mut scsi_cmnd,
    pub cb_idx: u8,
    pub direct_io: u8,
    pub pcie_sg_list: pcie_sg_list,
    pub chain_list: list_head,
    pub msix_io: u16,
}

//
// struct request_tracker - firmware request tracker
// @smid: system message id
// @cb_idx: callback index
// @tracker_list: list of free request (ioc->free_list)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_tracker {
    pub smid: u16,
    pub cb_idx: u8,
    pub tracker_list: list_head,
}

//
// struct _tr_list - target reset list
// @handle: device handle
// @state: state machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tr_list {
    pub list: list_head,
    pub handle: u16,
    pub state: u16,
}

//
// struct _sc_list - delayed SAS_IO_UNIT_CONTROL message list
// @handle: device handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sc_list {
    pub list: list_head,
    pub handle: u16,
}

//
// struct _event_ack_list - delayed event acknowledgment list
// @Event: Event ID
// @EventContext: used to track the event uniquely
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _event_ack_list {
    pub list: list_head,
    pub Event: U16,
    pub EventContext: U32,
}

//
// struct adapter_reply_queue - the reply queue struct
// @ioc: per adapter object
// @msix_index: msix index into vector table
// @vector: irq vector
// @reply_post_host_index: head index in the pool where FW completes IO
// @reply_post_free: reply post base virt address
// @name: the name registered to request_irq()
// @busy: isr is actively processing replies on another cpu
// @os_irq: irq number
// @irqpoll: irq_poll object
// @irq_poll_scheduled: Tells whether irq poll is scheduled or not
// @is_iouring_poll_q: Tells whether reply queues is assigned
// to io uring poll queues or not
// @list: this list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_reply_queue {
    pub ioc: *mut MPT3SAS_ADAPTER,
    pub msix_index: u8,
    pub reply_post_host_index: u32,
    pub reply_post_free: *mut Mpi2ReplyDescriptorsUnion_t,
    pub name: [c_char; MPT_NAME_LENGTH],
    pub busy: core::sync::atomic::AtomicI32,
    pub os_irq: u32,
    pub irqpoll: irq_poll,
    pub irq_poll_scheduled: bool,
    pub irq_line_enable: bool,
    pub is_iouring_poll_q: bool,
    pub list: list_head,
}

//
// struct io_uring_poll_queue - the io uring poll queue structure
// @busy: Tells whether io uring poll queue is busy or not
// @pause: Tells whether IOs are paused on io uring poll queue or not
// @reply_q: reply queue mapped for io uring poll queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_poll_queue {
    pub busy: core::sync::atomic::AtomicI32,
    pub pause: core::sync::atomic::AtomicI32,
    pub reply_q: *mut adapter_reply_queue,
}

extern "C" {
    pub fn void(paddr: *mut *mut MPT_ADD_SGE)(void, flags_length: u32, dma_addr: dma_addr_t) -> typedef;
}
// SAS3.0 support
// SAS3.5 support
// To support atomic and non atomic descriptors
extern "C" {
    pub fn void(ioc: *mut *mut PUT_SMID_DEFAULT) (struct MPT3SAS_ADAPTER, smid: u16) -> typedef;
}
extern "C" {
    pub fn u32(addr: *const *const BASE_READ_REG) (void __iomem) -> typedef;
}
//
// To get high iops reply queue's msix index when high iops mode is enabled
// else get the msix index of general reply queues.
//
// IOC Facts and Port Facts converted from little endian to cpu
#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_version_union {
    pub Struct: MPI2_VERSION_STRUCT,
    pub Word: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3sas_facts {
    pub MsgVersion: u16,
    pub HeaderVersion: u16,
    pub IOCNumber: u8,
    pub VP_ID: u8,
    pub VF_ID: u8,
    pub IOCExceptions: u16,
    pub IOCStatus: u16,
    pub IOCLogInfo: u32,
    pub MaxChainDepth: u8,
    pub WhoInit: u8,
    pub NumberOfPorts: u8,
    pub MaxMSIxVectors: u8,
    pub RequestCredit: u16,
    pub ProductID: u16,
    pub IOCCapabilities: u32,
    pub FWVersion: mpi3_version_union,
    pub IOCRequestFrameSize: u16,
    pub IOCMaxChainSegmentSize: u16,
    pub MaxInitiators: u16,
    pub MaxTargets: u16,
    pub MaxSasExpanders: u16,
    pub MaxEnclosures: u16,
    pub ProtocolFlags: u16,
    pub HighPriorityCredit: u16,
    pub MaxReplyDescriptorPostQueueDepth: u16,
    pub ReplyFrameSize: u8,
    pub MaxVolumes: u8,
    pub MaxDevHandle: u16,
    pub MaxPersistentEntries: u16,
    pub MinDevHandle: u16,
    pub CurrentHostPageSize: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3sas_port_facts {
    pub PortNumber: u8,
    pub VP_ID: u8,
    pub VF_ID: u8,
    pub PortType: u8,
    pub MaxPostedCmdBuffers: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reply_post_struct {
    pub reply_post_free: *mut Mpi2ReplyDescriptorsUnion_t,
    pub reply_post_free_dma: dma_addr_t,
}

//
// struct virtual_phy - vSES phy structure
// sas_address: SAS Address of vSES device
// phy_mask: vSES device's phy number
// flags: flags used to manage this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtual_phy {
    pub list: list_head,
    pub sas_address: u64,
    pub phy_mask: u32,
    pub flags: u8,
}

pub const MPT_VPHY_FLAG_DIRTY_PHY: c_uint = 0x01;
//
// struct hba_port - Saves each HBA's Wide/Narrow port info
// @sas_address: sas address of this wide/narrow port's attached device
// @phy_mask: HBA PHY's belonging to this port
// @port_id: port number
// @flags: hba port flags
// @vphys_mask : mask of vSES devices Phy number
// @vphys_list : list containing vSES device structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hba_port {
    pub list: list_head,
    pub sas_address: u64,
    pub phy_mask: u32,
    pub port_id: u8,
    pub flags: u8,
    pub vphys_mask: u32,
    pub vphys_list: list_head,
}

// hba port flags
pub const HBA_PORT_FLAG_DIRTY_PORT: c_uint = 0x01;
pub const HBA_PORT_FLAG_NEW_PORT: c_uint = 0x02;
pub const MULTIPATH_DISABLED_PORT_ID: c_uint = 0xFF;
//
// struct htb_rel_query - diagnostic buffer release reason
// @unique_id - unique id associated with this buffer.
// @buffer_rel_condition - Release condition ioctl/sysfs/reset
// @reserved
// @trigger_type - Master/Event/scsi/MPI
// @trigger_info_dwords - Data Correspondig to trigger type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htb_rel_query {
    pub buffer_rel_condition: u16,
    pub reserved: u16,
    pub trigger_type: u32,
    pub trigger_info_dwords: [u32; 2],
}

// Buffer_rel_condition bit fields
// Bit 0 - Diag Buffer not Released

// Bit 0 - Diag Buffer Released

//
// Bit 1 - Diag Buffer Released by IOCTL,
// This bit is valid only if Bit 0 is one
//

//
// Bit 2 - Diag Buffer Released by Trigger,
// This bit is valid only if Bit 0 is one
//

//
// Bit 3 - Diag Buffer Released by SysFs,
// This bit is valid only if Bit 0 is one
//

// DIAG RESET Master trigger flags
pub const MPT_DIAG_RESET_ISSUED_BY_DRIVER: c_uint = 0x00000000;
pub const MPT_DIAG_RESET_ISSUED_BY_USER: c_uint = 0x00000001;
extern "C" {
    pub fn void(ioc: *mut *mut MPT3SAS_FLUSH_RUNNING_CMDS)(struct MPT3SAS_ADAPTER) -> typedef;
}
//
// struct MPT3SAS_ADAPTER - per adapter struct
// @list: ioc_list
// @shost: shost object
// @id: unique adapter id
// @cpu_count: number online cpus
// @name: generic ioc string
// @tmp_string: tmp string used for logging
// @pdev: pci pdev object
// @pio_chip: physical io register space
// @chip: memory mapped register space
// @chip_phys: physical addrss prior to mapping
// @logging_level: see mpt3sas_debug.h
// @fwfault_debug: debuging FW timeouts
// @ir_firmware: IR firmware present
// @bars: bitmask of BAR's that must be configured
// @mask_interrupts: ignore interrupt
// @pci_access_mutex: Mutex to synchronize ioctl, sysfs show path and
// pci resource handling
// @fault_reset_work_q: fw fault workqueue
// @fault_reset_work: fw fault work
// @firmware_event_thread: fw event work queue
// @fw_event_lock:
// @fw_event_list: list of fw events
// @current_evet: current processing firmware event
// @fw_event_cleanup: set to one while cleaning up the fw events
// @aen_event_read_flag: event log was read
// @broadcast_aen_busy: broadcast aen waiting to be serviced
// @shost_recovery: host reset in progress
// @ioc_reset_in_progress_lock:
// @ioc_link_reset_in_progress: phy/hard reset in progress
// @ignore_loginfos: ignore loginfos during task management
// @remove_host: flag for when driver unloads, to avoid sending dev resets
// @pci_error_recovery: flag to prevent ioc access until slot reset completes
// @wait_for_discovery_to_complete: flag set at driver load time when
// waiting on reporting devices
// @is_driver_loading: flag set at driver load time
// @port_enable_failed: flag set when port enable has failed
// @start_scan: flag set from scan_start callback, cleared from _mpt3sas_fw_work
// @start_scan_failed: means port enable failed, return's the ioc_status
// @msix_enable: flag indicating msix is enabled
// @msix_vector_count: number msix vectors
// @cpu_msix_table: table for mapping cpus to msix index
// @cpu_msix_table_sz: table size
// @total_io_cnt: Gives total IO count, used to load balance the interrupts
// @ioc_coredump_loop: will have non-zero value when FW is in CoreDump state
// @timestamp_update_count: Counter to fire timeSync command
// time_sync_interval: Time sync interval read from man page 11
// @high_iops_outstanding: used to load balance the interrupts
// within high iops reply queues
// @msix_load_balance: Enables load balancing of interrupts across
// the multiple MSIXs
// @schedule_dead_ioc_flush_running_cmds: callback to flush pending commands
// @thresh_hold: Max number of reply descriptors processed
// before updating Host Index
// @iopoll_q_start_index: starting index of io uring poll queues
// in reply queue list
// @drv_internal_flags: Bit map internal to driver
// @drv_support_bitmap: driver's supported feature bit map
// @use_32bit_dma: Flag to use 32 bit consistent dma mask
// @scsi_io_cb_idx: shost generated commands
// @tm_cb_idx: task management commands
// @scsih_cb_idx: scsih internal commands
// @transport_cb_idx: transport internal commands
// @ctl_cb_idx: clt internal commands
// @base_cb_idx: base internal commands
// @config_cb_idx: base internal commands
// @tm_tr_cb_idx : device removal target reset handshake
// @tm_tr_volume_cb_idx : volume removal target reset
// @base_cmds:
// @transport_cmds:
// @scsih_cmds:
// @tm_cmds:
// @ctl_cmds:
// @config_cmds:
// @base_add_sg_single: handler for either 32/64 bit sgl's
// @event_type: bits indicating which events to log
// @event_context: unique id for each logged event
// @event_log: event log pointer
// @event_masks: events that are masked
// @max_shutdown_latency: timeout value for NVMe shutdown operation,
// which is equal that NVMe drive's RTD3 Entry Latency
// which has reported maximum RTD3 Entry Latency value
// among attached NVMe drives.
// @facts: static facts data
// @prev_fw_facts: previous fw facts data
// @pfacts: static port facts data
// @manu_pg0: static manufacturing page 0
// @manu_pg10: static manufacturing page 10
// @manu_pg11: static manufacturing page 11
// @bios_pg2: static bios page 2
// @bios_pg3: static bios page 3
// @ioc_pg8: static ioc page 8
// @iounit_pg0: static iounit page 0
// @iounit_pg1: static iounit page 1
// @sas_hba: sas host object
// @sas_expander_list: expander object list
// @enclosure_list: enclosure object list
// @sas_node_lock:
// @sas_device_list: sas device object list
// @sas_device_init_list: sas device object list (used only at init time)
// @sas_device_lock:
// @pcie_device_list: pcie device object list
// @pcie_device_init_list: pcie device object list (used only at init time)
// @pcie_device_lock:
// @io_missing_delay: time for IO completed by fw when PDR enabled
// @device_missing_delay: time for device missing by fw when PDR enabled
// @sas_id : used for setting volume target IDs
// @pcie_target_id: used for setting pcie target IDs
// @blocking_handles: bitmask used to identify which devices need blocking
// @pd_handles : bitmask for PD handles
// @pd_handles_sz : size of pd_handle bitmask
// @config_page_sz: config page size
// @config_page: reserve memory for config page payload
// @config_page_dma:
// @hba_queue_depth: hba request queue depth
// @sge_size: sg element size for either 32/64 bit
// @scsiio_depth: SCSI_IO queue depth
// @request_sz: per request frame size
// @request: pool of request frames
// @request_dma:
// @request_dma_sz:
// @scsi_lookup: firmware request tracker list
// @scsi_lookup_lock:
// @free_list: free list of request
// @pending_io_count:
// @reset_wq:
// @chain: pool of chains
// @chain_dma:
// @max_sges_in_main_message: number sg elements in main message
// @max_sges_in_chain_message: number sg elements per chain
// @chains_needed_per_io: max chains per io
// @chain_depth: total chains allocated
// @chain_segment_sz: gives the max number of
// SGEs accommodate on single chain buffer
// @hi_priority_smid:
// @hi_priority:
// @hi_priority_dma:
// @hi_priority_depth:
// @hpr_lookup:
// @hpr_free_list:
// @internal_smid:
// @internal:
// @internal_dma:
// @internal_depth:
// @internal_lookup:
// @internal_free_list:
// @sense: pool of sense
// @sense_dma:
// @sense_dma_pool:
// @reply_depth: hba reply queue depth:
// @reply_sz: per reply frame size:
// @reply: pool of replys:
// @reply_dma:
// @reply_dma_pool:
// @reply_free_queue_depth: reply free depth
// @reply_free: pool for reply free queue (32 bit addr)
// @reply_free_dma:
// @reply_free_dma_pool:
// @reply_free_host_index: tail index in pool to insert free replys
// @reply_post_queue_depth: reply post queue depth
// @reply_post_struct: struct for reply_post_free physical & virt address
// @rdpq_array_capable: FW supports multiple reply queue addresses in ioc_init
// @rdpq_array_enable: rdpq_array support is enabled in the driver
// @rdpq_array_enable_assigned: this ensures that rdpq_array_enable flag
// is assigned only ones
// @reply_queue_count: number of reply queue's
// @reply_queue_list: link list contaning the reply queue info
// @msix96_vector: 96 MSI-X vector support
// @replyPostRegisterIndex: index of next position in Reply Desc Post Queue
// @delayed_tr_list: target reset link list
// @delayed_tr_volume_list: volume target reset link list
// @delayed_sc_list:
// @delayed_event_ack_list:
// @temp_sensors_count: flag to carry the number of temperature sensors
// @pci_access_mutex: Mutex to synchronize ioctl,sysfs show path and
// pci resource handling. PCI resource freeing will lead to free
// vital hardware/memory resource, which might be in use by cli/sysfs
// path functions resulting in Null pointer reference followed by kernel
// crash. To avoid the above race condition we use mutex syncrhonization
// which ensures the syncrhonization between cli/sysfs_show path.
// @atomic_desc_capable: Atomic Request Descriptor support.
// @GET_MSIX_INDEX: Get the msix index of high iops queues.
// @multipath_on_hba: flag to determine multipath on hba is enabled or not
// @port_table_list: list containing HBA's wide/narrow port's info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPT3SAS_ADAPTER {
    pub list: list_head,
    pub shost: *mut Scsi_Host,
    pub id: u8,
    pub cpu_count: c_int,
    pub name: [c_char; MPT_NAME_LENGTH],
    pub 8]: char driver_name[MPT_NAME_LENGTH -,
    pub tmp_string: [c_char; MPT_STRING_LENGTH],
    pub pdev: *mut pci_dev,
    pub chip: *mut Mpi2SystemInterfaceRegs_t __iomem,
    pub chip_phys: phys_addr_t,
    pub logging_level: c_int,
    pub fwfault_debug: c_int,
    pub ir_firmware: u8,
    pub bars: c_int,
    pub mask_interrupts: u8,
// fw fault handler
    pub fault_reset_work_q: *mut workqueue_struct,
    pub fault_reset_work: delayed_work,
// fw event handler
    pub firmware_event_thread: *mut workqueue_struct,
    pub fw_event_lock: spinlock_t,
    pub fw_event_list: list_head,
    pub current_event: *mut fw_event_work,
    pub fw_events_cleanup: u8,
// misc flags
    pub aen_event_read_flag: c_int,
    pub broadcast_aen_busy: u8,
    pub broadcast_aen_pending: u16,
    pub shost_recovery: u8,
    pub got_task_abort_from_ioctl: u8,
    pub reset_in_progress_mutex: mutex,
    pub hostdiag_unlock_mutex: mutex,
    pub ioc_reset_in_progress_lock: spinlock_t,
    pub ioc_link_reset_in_progress: u8,
    pub ignore_loginfos: u8,
    pub remove_host: u8,
    pub pci_error_recovery: u8,
    pub wait_for_discovery_to_complete: u8,
    pub is_driver_loading: u8,
    pub port_enable_failed: u8,
    pub start_scan: u8,
    pub start_scan_failed: u16,
    pub msix_enable: u8,
    pub msix_vector_count: u16,
    pub cpu_msix_table: *mut u8,
    pub cpu_msix_table_sz: u16,
    pub reply_post_host_index: *mut resource_size_t __iomem,
    pub ioc_reset_count: u32,
    pub schedule_dead_ioc_flush_running_cmds: MPT3SAS_FLUSH_RUNNING_CMDS,
    pub non_operational_loop: u32,
    pub ioc_coredump_loop: u8,
    pub timestamp_update_count: u32,
    pub time_sync_interval: u32,
    pub total_io_cnt: core::sync::atomic::AtomicI64,
    pub high_iops_outstanding: core::sync::atomic::AtomicI64,
    pub msix_load_balance: bool,
    pub thresh_hold: u16,
    pub high_iops_queues: u8,
    pub iopoll_q_start_index: u8,
    pub drv_internal_flags: u32,
    pub drv_support_bitmap: u32,
    pub dma_mask: u32,
    pub enable_sdev_max_qd: bool,
    pub use_32bit_dma: bool,
    pub io_uring_poll_queues: *mut io_uring_poll_queue,
// internal commands, callback index
    pub scsi_io_cb_idx: u8,
    pub tm_cb_idx: u8,
    pub transport_cb_idx: u8,
    pub scsih_cb_idx: u8,
    pub ctl_cb_idx: u8,
    pub base_cb_idx: u8,
    pub port_enable_cb_idx: u8,
    pub config_cb_idx: u8,
    pub tm_tr_cb_idx: u8,
    pub tm_tr_volume_cb_idx: u8,
    pub tm_sas_control_cb_idx: u8,
    pub base_cmds: _internal_cmd,
    pub port_enable_cmds: _internal_cmd,
    pub transport_cmds: _internal_cmd,
    pub scsih_cmds: _internal_cmd,
    pub tm_cmds: _internal_cmd,
    pub ctl_cmds: _internal_cmd,
    pub config_cmds: _internal_cmd,
    pub base_add_sg_single: MPT_ADD_SGE,
// function ptr for either IEEE or MPI sg elements
    pub build_sg_scmd: MPT_BUILD_SG_SCMD,
    pub build_sg: MPT_BUILD_SG,
    pub build_zero_len_sge: MPT_BUILD_ZERO_LEN_SGE,
    pub sge_size_ieee: u16,
    pub hba_mpi_version_belonged: u16,
// function ptr for MPI sg elements only
    pub build_sg_mpi: MPT_BUILD_SG,
    pub build_zero_len_sge_mpi: MPT_BUILD_ZERO_LEN_SGE,
// function ptr for NVMe PRP elements only
    pub build_nvme_prp: NVME_BUILD_PRP,
// event log
    pub event_type: [u32; MPI2_EVENT_NOTIFY_EVENTMASK_WORDS],
    pub event_context: u32,
    pub event_log: *mut c_void,
    pub event_masks: [u32; MPI2_EVENT_NOTIFY_EVENTMASK_WORDS],
    pub tm_custom_handling: u8,
    pub nvme_abort_timeout: u8,
    pub max_shutdown_latency: u16,
    pub max_wideport_qd: u16,
    pub max_narrowport_qd: u16,
    pub max_nvme_qd: u16,
    pub max_sata_qd: u8,
// static config pages
    pub facts: mpt3sas_facts,
    pub prev_fw_facts: mpt3sas_facts,
    pub pfacts: *mut mpt3sas_port_facts,
    pub manu_pg0: Mpi2ManufacturingPage0_t,
    pub manu_pg10: Mpi2ManufacturingPage10_t,
    pub manu_pg11: Mpi2ManufacturingPage11_t,
    pub bios_pg2: Mpi2BiosPage2_t,
    pub bios_pg3: Mpi2BiosPage3_t,
    pub ioc_pg8: Mpi2IOCPage8_t,
    pub iounit_pg0: Mpi2IOUnitPage0_t,
    pub iounit_pg1: Mpi2IOUnitPage1_t,
    pub ioc_pg1_copy: Mpi2IOCPage1_t,
    pub req_boot_device: _boot_device,
    pub req_alt_boot_device: _boot_device,
    pub current_boot_device: _boot_device,
// sas hba, expander, and device list
    pub sas_hba: _sas_node,
    pub sas_expander_list: list_head,
    pub enclosure_list: list_head,
    pub sas_node_lock: spinlock_t,
    pub sas_device_list: list_head,
    pub sas_device_init_list: list_head,
    pub sas_device_lock: spinlock_t,
    pub pcie_device_list: list_head,
    pub pcie_device_init_list: list_head,
    pub pcie_device_lock: spinlock_t,
    pub raid_device_list: list_head,
    pub raid_device_lock: spinlock_t,
    pub io_missing_delay: u8,
    pub device_missing_delay: u16,
    pub sas_id: c_int,
    pub pcie_target_id: c_int,
    pub blocking_handles: *mut c_void,
    pub pd_handles: *mut c_void,
    pub pd_handles_sz: u16,
    pub pend_os_device_add: *mut c_void,
    pub pend_os_device_add_sz: u16,
// config page
    pub config_page_sz: u16,
    pub config_page: *mut c_void,
    pub config_page_dma: dma_addr_t,
    pub config_vaddr: *mut c_void,
// scsiio request
    pub hba_queue_depth: u16,
    pub sge_size: u16,
    pub scsiio_depth: u16,
    pub request_sz: u16,
    pub request: *mut u8,
    pub request_dma: dma_addr_t,
    pub request_dma_sz: u32,
    pub pcie_sg_lookup: *mut pcie_sg_list,
    pub scsi_lookup_lock: spinlock_t,
    pub pending_io_count: c_int,
    pub reset_wq: wait_queue_head_t,
    pub io_queue_num: *mut u16,
// PCIe SGL
    pub pcie_sgl_dma_pool: *mut dma_pool,
// Host Page Size
    pub page_size: u32,
// chain
    pub chain_lookup: *mut chain_lookup,
    pub free_chain_list: list_head,
    pub chain_dma_pool: *mut dma_pool,
    pub chain_pages: c_ulong,
    pub max_sges_in_main_message: u16,
    pub max_sges_in_chain_message: u16,
    pub chains_needed_per_io: u16,
    pub chain_depth: u32,
    pub chain_segment_sz: u16,
    pub chains_per_prp_buffer: u16,
// hi-priority queue
    pub hi_priority_smid: u16,
    pub hi_priority: *mut u8,
    pub hi_priority_dma: dma_addr_t,
    pub hi_priority_depth: u16,
    pub hpr_lookup: *mut request_tracker,
    pub hpr_free_list: list_head,
// internal queue
    pub internal_smid: u16,
    pub internal: *mut u8,
    pub internal_dma: dma_addr_t,
    pub internal_depth: u16,
    pub internal_lookup: *mut request_tracker,
    pub internal_free_list: list_head,
// sense
    pub sense: *mut u8,
    pub sense_dma: dma_addr_t,
    pub sense_dma_pool: *mut dma_pool,
// reply
    pub reply_sz: u16,
    pub reply: *mut u8,
    pub reply_dma: dma_addr_t,
    pub reply_dma_max_address: u32,
    pub reply_dma_min_address: u32,
    pub reply_dma_pool: *mut dma_pool,
// reply free queue
    pub reply_free_queue_depth: u16,
    pub reply_free: *mut __le32,
    pub reply_free_dma: dma_addr_t,
    pub reply_free_dma_pool: *mut dma_pool,
    pub reply_free_host_index: u32,
// reply post queue
    pub reply_post_queue_depth: u16,
    pub reply_post: *mut reply_post_struct,
    pub rdpq_array_capable: u8,
    pub rdpq_array_enable: u8,
    pub rdpq_array_enable_assigned: u8,
    pub reply_post_free_dma_pool: *mut dma_pool,
    pub reply_post_free_array_dma_pool: *mut dma_pool,
    pub reply_post_free_array: *mut Mpi2IOCInitRDPQArrayEntry,
    pub reply_post_free_array_dma: dma_addr_t,
    pub reply_queue_count: u8,
    pub reply_queue_list: list_head,
    pub combined_reply_queue: u8,
    pub combined_reply_index_count: u8,
    pub smp_affinity_enable: u8,
// reply post register index
    pub replyPostRegisterIndex: *mut resource_size_t __iomem,
    pub delayed_tr_list: list_head,
    pub delayed_tr_volume_list: list_head,
    pub delayed_sc_list: list_head,
    pub delayed_event_ack_list: list_head,
    pub temp_sensors_count: u8,
    pub pci_access_mutex: mutex,
// diag buffer support
    pub diag_buffer: [*mut u8; MPI2_DIAG_BUF_TYPE_COUNT],
    pub diag_buffer_sz: [u32; MPI2_DIAG_BUF_TYPE_COUNT],
    pub diag_buffer_dma: [dma_addr_t; MPI2_DIAG_BUF_TYPE_COUNT],
    pub diag_buffer_status: [u8; MPI2_DIAG_BUF_TYPE_COUNT],
    pub unique_id: [u32; MPI2_DIAG_BUF_TYPE_COUNT],
    pub product_specific: [u32; MPI2_DIAG_BUF_TYPE_COUNT][23],
    pub diagnostic_flags: [u32; MPI2_DIAG_BUF_TYPE_COUNT],
    pub ring_buffer_offset: u32,
    pub ring_buffer_sz: u32,
    pub htb_rel: htb_rel_query,
    pub reset_from_user: u8,
    pub is_warpdrive: u8,
    pub is_mcpu_endpoint: u8,
    pub hide_ir_msg: u8,
    pub mfg_pg10_hide_flag: u8,
    pub hide_drives: u8,
    pub diag_trigger_lock: spinlock_t,
    pub diag_trigger_active: u8,
    pub atomic_desc_capable: u8,
    pub base_readl: BASE_READ_REG,
    pub base_readl_ext_retry: BASE_READ_REG,
    pub diag_trigger_master: SL_WH_MASTER_TRIGGER_T,
    pub diag_trigger_event: SL_WH_EVENT_TRIGGERS_T,
    pub diag_trigger_scsi: SL_WH_SCSI_TRIGGERS_T,
    pub diag_trigger_mpi: SL_WH_MPI_TRIGGERS_T,
    pub supports_trigger_pages: u8,
    pub device_remove_in_progress: *mut c_void,
    pub device_remove_in_progress_sz: u16,
    pub is_gen35_ioc: u8,
    pub is_aero_ioc: u8,
    pub debugfs_root: *mut dentry,
    pub ioc_dump: *mut dentry,

    pub hwmon: *mut mpt3sas_hwmon,

    pub put_smid_scsi_io: PUT_SMID_IO_FP_HIP,
    pub put_smid_fast_path: PUT_SMID_IO_FP_HIP,
    pub put_smid_hi_priority: PUT_SMID_IO_FP_HIP,
    pub put_smid_default: PUT_SMID_DEFAULT,
    pub get_msix_index_for_smlio: GET_MSIX_INDEX,
    pub multipath_on_hba: u8,
    pub port_table_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpt3sas_debugfs_buffer {
    pub buf: *mut c_void,
    pub len: u32,
}

pub const MPT_DRV_SUPPORT_BITMAP_MEMMOVE: c_uint = 0x00000001;
pub const MPT_DRV_SUPPORT_BITMAP_ADDNLQUERY: c_uint = 0x00000002;
pub const MPT_DRV_INTERNAL_FIRST_PE_ISSUED: c_uint = 0x00000001;
//
// struct ATTO_SAS_NVRAM - ATTO NVRAM settings stored
// in Manufacturing page 1 used to get
// ATTO SasAddr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTO_SAS_NVRAM {
    pub Signature: [u8; 4],
    pub Version: u8,
pub const ATTO_SASNVR_VERSION: c_int = 0;
    pub Checksum: u8,
pub const ATTO_SASNVR_CKSUM_SEED: c_uint = 0x5A;
    pub Pad: [u8; 10],
    pub SasAddr: [u8; 8],
pub const ATTO_SAS_ADDR_ALIGN: c_int = 64;
    pub Reserved: [u8; 232],
}

pub const ATTO_SAS_ADDR_DEVNAME_BIAS: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ATTO_SAS_ADDRESS {
    pub b: [U8; 8],
    pub w: [U16; 4],
    pub d: [U32; 2],
    pub q: U64,
}

// base shared API
// spinlock on list operations over IOCs
// Case: when multiple warpdrive cards(IOCs) are in use
// Each IOC will added to the ioc list structure on initialization.
// Watchdog threads run at regular intervals to check IOC for any
// fault conditions which will trigger the dead_ioc thread to
// deallocate pci resource, resulting deleting the IOC netry from list,
// this deletion need to protected by spinlock to enusre that
// ioc removal is syncrhonized, if not synchronized it might lead to
// list_del corruption as the ioc list is traversed in cli path.
//
extern "C" {
    pub fn mpt3sas_base_start_watchdog(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_stop_watchdog(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_attach(ioc: *mut MPT3SAS_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mpt3sas_base_detach(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_map_resources(ioc: *mut MPT3SAS_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mpt3sas_base_free_resources(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_free_enclosure_list(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_get_pcie_sgl_dma(ioc: *mut MPT3SAS_ADAPTER, smid: u16) -> dma_addr_t;
}
extern "C" {
    pub fn mpt3sas_base_sync_reply_irqs(ioc: *mut MPT3SAS_ADAPTER, poll: u8);
}
extern "C" {
    pub fn mpt3sas_base_mask_interrupts(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_unmask_interrupts(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_put_smid_nvme_encap(ioc: *mut MPT3SAS_ADAPTER, smid: u16);
}
extern "C" {
    pub fn mpt3sas_base_put_smid_default(ioc: *mut MPT3SAS_ADAPTER, smid: u16);
}
// hi-priority queue
extern "C" {
    pub fn mpt3sas_base_get_smid_hpr(ioc: *mut MPT3SAS_ADAPTER, cb_idx: u8) -> u16;
}
extern "C" {
    pub fn mpt3sas_base_get_smid(ioc: *mut MPT3SAS_ADAPTER, cb_idx: u8) -> u16;
}
extern "C" {
    pub fn mpt3sas_base_free_smid(ioc: *mut MPT3SAS_ADAPTER, smid: u16);
}
extern "C" {
    pub fn mpt3sas_base_initialize_callback_handler();
}
extern "C" {
    pub fn mpt3sas_base_register_callback_handler(cb_func: MPT_CALLBACK) -> u8;
}
extern "C" {
    pub fn mpt3sas_base_release_callback_handler(cb_idx: u8);
}
extern "C" {
    pub fn mpt3sas_base_get_iocstate(ioc: *mut MPT3SAS_ADAPTER, cooked: c_int) -> u32;
}
extern "C" {
    pub fn mpt3sas_base_fault_info(ioc: *mut MPT3SAS_ADAPTER, fault_code: u16);
}

extern "C" {
    pub fn mpt3sas_base_coredump_info(ioc: *mut MPT3SAS_ADAPTER, fault_code: u16);
}

extern "C" {
    pub fn mpt3sas_halt_firmware(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_port_enable(ioc: *mut MPT3SAS_ADAPTER) -> c_int;
}

extern "C" {
    pub fn mpt3sas_wait_for_ioc(ioc: *mut MPT3SAS_ADAPTER, wait_count: c_int) -> c_int;
}
extern "C" {
    pub fn mpt3sas_base_make_ioc_ready(ioc: *mut MPT3SAS_ADAPTER, type: reset_type) -> c_int;
}
extern "C" {
    pub fn mpt3sas_base_free_irq(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_disable_msix(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_blk_mq_poll(shost: *mut Scsi_Host, queue_num: c_uint) -> c_int;
}
extern "C" {
    pub fn mpt3sas_base_pause_mq_polling(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_resume_mq_polling(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_base_lock_host_diagnostic(ioc: *mut MPT3SAS_ADAPTER);
}
// scsih shared API
extern "C" {
    pub fn mpt3sas_scsih_pre_reset_handler(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_scsih_reset_done_handler(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_scsih_set_tm_flag(ioc: *mut MPT3SAS_ADAPTER, handle: u16);
}
extern "C" {
    pub fn mpt3sas_scsih_clear_tm_flag(ioc: *mut MPT3SAS_ADAPTER, handle: u16);
}
extern "C" {
    pub fn mpt3sas_port_enable_complete(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_scsih_change_queue_depth(sdev: *mut scsi_device, qdepth: c_int);
}
// config shared API
// mpi_reply, Mpi2BiosPage2_t *config_page);
// mpi_reply, Mpi2BiosPage3_t *config_page);
// mpi_reply, Mpi2IOUnitPage0_t *config_page);
// mpi_reply, Mpi2IOUnitPage1_t *config_page);
// mpi_reply, Mpi2IOUnitPage8_t *config_page);
// mpi_reply, Mpi2IOCPage1_t *config_page);
// mpi_reply, Mpi2IOCPage8_t *config_page);
// mpi_reply, Mpi2SasPhyPage0_t *config_page, u32 phy_number);
// mpi_reply, Mpi2SasPhyPage1_t *config_page, u32 phy_number);
// ctl shared API
extern "C" {
    pub fn mpt3sas_ctl_init(hbas_to_enumerate: c_ushort);
}
extern "C" {
    pub fn mpt3sas_ctl_exit(hbas_to_enumerate: c_ushort);
}
extern "C" {
    pub fn mpt3sas_ctl_release(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_ctl_pre_reset_handler(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_ctl_clear_outstanding_ioctls(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_ctl_reset_done_handler(ioc: *mut MPT3SAS_ADAPTER);
}
// transport shared API
// mpt3sas_phy, Mpi2SasPhyPage0_t phy_pg0, struct device *parent_dev);
// trigger data externs
// warpdrive APIs
extern "C" {
    pub fn mpt3sas_get_num_volumes(ioc: *mut MPT3SAS_ADAPTER) -> u8;
}
extern "C" {
    pub fn mpt3sas_setup_debugfs(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_destroy_debugfs(ioc: *mut MPT3SAS_ADAPTER);
}
extern "C" {
    pub fn mpt3sas_init_debugfs();
}
extern "C" {
    pub fn mpt3sas_exit_debugfs();
}

extern "C" {
    pub fn mpt3sas_hwmon_register(ioc: *mut MPT3SAS_ADAPTER) -> c_int;
}
extern "C" {
    pub fn mpt3sas_hwmon_unregister(ioc: *mut MPT3SAS_ADAPTER);
}

//
// _scsih_is_pcie_scsi_device - determines if device is an pcie scsi device
// @device_info: bitfield providing information about the device.
// Context: none
//
// Returns 1 if scsi device.
//
