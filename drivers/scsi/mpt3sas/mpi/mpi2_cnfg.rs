//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_cnfg.h
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
//
// Copyright 2000-2020 Broadcom Inc. All rights reserved.
//
// Name:  mpi2_cnfg.h
// Title:  MPI Configuration messages and pages
// Creation Date:  November 10, 2006
//
// mpi2_cnfg.h Version:  02.00.47
//
// NOTE: Names (typedefs, defines, etc.) beginning with an MPI25 or Mpi25
// prefix are for use only on MPI v2.5 products, and must not be used
// with MPI v2.0 products. Unless otherwise noted, names beginning with
// MPI2 or Mpi2 are for use with both MPI v2.0 and MPI v2.5 products.
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 06-04-07  02.00.01  Added defines for SAS IO Unit Page 2 PhyFlags.
// Added Manufacturing Page 11.
// Added MPI2_SAS_EXPANDER0_FLAGS_CONNECTOR_END_DEVICE
// define.
// 06-26-07  02.00.02  Adding generic structure for product-specific
// Manufacturing pages: MPI2_CONFIG_PAGE_MANUFACTURING_PS.
// Rework of BIOS Page 2 configuration page.
// Fixed MPI2_BIOSPAGE2_BOOT_DEVICE to be a union of the
// forms.
// Added configuration pages IOC Page 8 and Driver
// Persistent Mapping Page 0.
// 08-31-07  02.00.03  Modified configuration pages dealing with Integrated
// RAID (Manufacturing Page 4, RAID Volume Pages 0 and 1,
// RAID Physical Disk Pages 0 and 1, RAID Configuration
// Page 0).
// Added new value for AccessStatus field of SAS Device
// Page 0 (_SATA_NEEDS_INITIALIZATION).
// 10-31-07  02.00.04  Added missing SEPDevHandle field to
// MPI2_CONFIG_PAGE_SAS_ENCLOSURE_0.
// 12-18-07  02.00.05  Modified IO Unit Page 0 to use 32-bit version fields for
// NVDATA.
// Modified IOC Page 7 to use masks and added field for
// SASBroadcastPrimitiveMasks.
// Added MPI2_CONFIG_PAGE_BIOS_4.
// Added MPI2_CONFIG_PAGE_LOG_0.
// 02-29-08  02.00.06  Modified various names to make them 32-character unique.
// Added SAS Device IDs.
// Updated Integrated RAID configuration pages including
// Manufacturing Page 4, IOC Page 6, and RAID Configuration
// Page 0.
// 05-21-08  02.00.07  Added define MPI2_MANPAGE4_MIX_SSD_SAS_SATA.
// Added define MPI2_MANPAGE4_PHYSDISK_128MB_COERCION.
// Fixed define MPI2_IOCPAGE8_FLAGS_ENCLOSURE_SLOT_MAPPING.
// Added missing MaxNumRoutedSasAddresses field to
// MPI2_CONFIG_PAGE_EXPANDER_0.
// Added SAS Port Page 0.
// Modified structure layout for
// MPI2_CONFIG_PAGE_DRIVER_MAPPING_0.
// 06-27-08  02.00.08  Changed MPI2_CONFIG_PAGE_RD_PDISK_1 to use
// MPI2_RAID_PHYS_DISK1_PATH_MAX to size the array.
// 10-02-08  02.00.09  Changed MPI2_RAID_PGAD_CONFIGNUM_MASK from 0x0000FFFF
// to 0x000000FF.
// Added two new values for the Physical Disk Coercion Size
// bits in the Flags field of Manufacturing Page 4.
// Added product-specific Manufacturing pages 16 to 31.
// Modified Flags bits for controlling write cache on SATA
// drives in IO Unit Page 1.
// Added new bit to AdditionalControlFlags of SAS IO Unit
// Page 1 to control Invalid Topology Correction.
// Added additional defines for RAID Volume Page 0
// VolumeStatusFlags field.
// Modified meaning of RAID Volume Page 0 VolumeSettings
// define for auto-configure of hot-swap drives.
// Added SupportedPhysDisks field to RAID Volume Page 1 and
// added related defines.
// Added PhysDiskAttributes field (and related defines) to
// RAID Physical Disk Page 0.
// Added MPI2_SAS_PHYINFO_PHY_VACANT define.
// Added three new DiscoveryStatus bits for SAS IO Unit
// Page 0 and SAS Expander Page 0.
// Removed multiplexing information from SAS IO Unit pages.
// Added BootDeviceWaitTime field to SAS IO Unit Page 4.
// Removed Zone Address Resolved bit from PhyInfo and from
// Expander Page 0 Flags field.
// Added two new AccessStatus values to SAS Device Page 0
// for indicating routing problems. Added 3 reserved words
// to this page.
// 01-19-09  02.00.10  Fixed defines for GPIOVal field of IO Unit Page 3.
// Inserted missing reserved field into structure for IOC
// Page 6.
// Added more pending task bits to RAID Volume Page 0
// VolumeStatusFlags defines.
// Added MPI2_PHYSDISK0_STATUS_FLAG_NOT_CERTIFIED define.
// Added a new DiscoveryStatus bit for SAS IO Unit Page 0
// and SAS Expander Page 0 to flag a downstream initiator
// when in simplified routing mode.
// Removed SATA Init Failure defines for DiscoveryStatus
// fields of SAS IO Unit Page 0 and SAS Expander Page 0.
// Added MPI2_SAS_DEVICE0_ASTATUS_DEVICE_BLOCKED define.
// Added PortGroups, DmaGroup, and ControlGroup fields to
// SAS Device Page 0.
// 05-06-09  02.00.11  Added structures and defines for IO Unit Page 5 and IO
// Unit Page 6.
// Added expander reduced functionality data to SAS
// Expander Page 0.
// Added SAS PHY Page 2 and SAS PHY Page 3.
// 07-30-09  02.00.12  Added IO Unit Page 7.
// Added new device ids.
// Added SAS IO Unit Page 5.
// Added partial and slumber power management capable flags
// to SAS Device Page 0 Flags field.
// Added PhyInfo defines for power condition.
// Added Ethernet configuration pages.
// 10-28-09  02.00.13  Added MPI2_IOUNITPAGE1_ENABLE_HOST_BASED_DISCOVERY.
// Added SAS PHY Page 4 structure and defines.
// 02-10-10  02.00.14  Modified the comments for the configuration page
// structures that contain an array of data. The host
// should use the "count" field in the page data (e.g. the
// NumPhys field) to determine the number of valid elements
// in the array.
// Added/modified some MPI2_MFGPAGE_DEVID_SAS defines.
// Added PowerManagementCapabilities to IO Unit Page 7.
// Added PortWidthModGroup field to
// MPI2_SAS_IO_UNIT5_PHY_PM_SETTINGS.
// Added MPI2_CONFIG_PAGE_SASIOUNIT_6 and related defines.
// Added MPI2_CONFIG_PAGE_SASIOUNIT_7 and related defines.
// Added MPI2_CONFIG_PAGE_SASIOUNIT_8 and related defines.
// 05-12-10  02.00.15  Added MPI2_RAIDVOL0_STATUS_FLAG_VOL_NOT_CONSISTENT
// define.
// Added MPI2_PHYSDISK0_INCOMPATIBLE_MEDIA_TYPE define.
// Added MPI2_SAS_NEG_LINK_RATE_UNSUPPORTED_PHY define.
// 08-11-10  02.00.16  Removed IO Unit Page 1 device path (multi-pathing)
// defines.
// 11-10-10  02.00.17  Added ReceptacleID field (replacing Reserved1) to
// MPI2_MANPAGE7_CONNECTOR_INFO and reworked defines for
// the Pinout field.
// Added BoardTemperature and BoardTemperatureUnits fields
// to MPI2_CONFIG_PAGE_IO_UNIT_7.
// Added MPI2_CONFIG_EXTPAGETYPE_EXT_MANUFACTURING define
// and MPI2_CONFIG_PAGE_EXT_MAN_PS structure.
// 02-23-11  02.00.18  Added ProxyVF_ID field to MPI2_CONFIG_REQUEST.
// Added IO Unit Page 8, IO Unit Page 9,
// and IO Unit Page 10.
// Added SASNotifyPrimitiveMasks field to
// MPI2_CONFIG_PAGE_IOC_7.
// 03-09-11  02.00.19  Fixed IO Unit Page 10 (to match the spec).
// 05-25-11  02.00.20  Cleaned up a few comments.
// 08-24-11  02.00.21  Marked the IO Unit Page 7 PowerManagementCapabilities
// for PCIe link as obsolete.
// Added SpinupFlags field containing a Disable Spin-up bit
// to the MPI2_SAS_IOUNIT4_SPINUP_GROUP fields of SAS IO
// Unit Page 4.
// 11-18-11  02.00.22  Added define MPI2_IOCPAGE6_CAP_FLAGS_4K_SECTORS_SUPPORT.
// Added UEFIVersion field to BIOS Page 1 and defined new
// BiosOptions bits.
// Incorporating additions for MPI v2.5.
// 11-27-12  02.00.23  Added MPI2_MANPAGE7_FLAG_EVENTREPLAY_SLOT_ORDER.
// Added MPI2_BIOSPAGE1_OPTIONS_MASK_OEM_ID.
// 12-20-12  02.00.24  Marked MPI2_SASIOUNIT1_CONTROL_CLEAR_AFFILIATION as
// obsolete for MPI v2.5 and later.
// Added some defines for 12G SAS speeds.
// 04-09-13  02.00.25  Added MPI2_IOUNITPAGE1_ATA_SECURITY_FREEZE_LOCK.
// Fixed MPI2_IOUNITPAGE5_DMA_CAP_MASK_MAX_REQUESTS to
// match the specification.
// 08-19-13  02.00.26  Added reserved words to MPI2_CONFIG_PAGE_IO_UNIT_7 for
// future use.
// 12-05-13  02.00.27  Added MPI2_MANPAGE7_FLAG_BASE_ENCLOSURE_LEVEL for
// MPI2_CONFIG_PAGE_MAN_7.
// Added EnclosureLevel and ConnectorName fields to
// MPI2_CONFIG_PAGE_SAS_DEV_0.
// Added MPI2_SAS_DEVICE0_FLAGS_ENCL_LEVEL_VALID for
// MPI2_CONFIG_PAGE_SAS_DEV_0.
// Added EnclosureLevel field to
// MPI2_CONFIG_PAGE_SAS_ENCLOSURE_0.
// Added MPI2_SAS_ENCLS0_FLAGS_ENCL_LEVEL_VALID for
// MPI2_CONFIG_PAGE_SAS_ENCLOSURE_0.
// 01-08-14  02.00.28  Added more defines for the BiosOptions field of
// MPI2_CONFIG_PAGE_BIOS_1.
// 06-13-14  02.00.29  Added SSUTimeout field to MPI2_CONFIG_PAGE_BIOS_1, and
// more defines for the BiosOptions field.
// 11-18-14  02.00.30  Updated copyright information.
// Added MPI2_BIOSPAGE1_OPTIONS_ADVANCED_CONFIG.
// Added AdapterOrderAux fields to BIOS Page 3.
// 03-16-15  02.00.31  Updated for MPI v2.6.
// Added Flags field to IO Unit Page 7.
// Added new SAS Phy Event codes
// 05-25-15  02.00.33  Added more defines for the BiosOptions field of
// MPI2_CONFIG_PAGE_BIOS_1.
// 08-25-15  02.00.34  Bumped Header Version.
// 12-18-15  02.00.35  Added SATADeviceWaitTime to SAS IO Unit Page 4.
// 01-21-16  02.00.36  Added/modified MPI2_MFGPAGE_DEVID_SAS defines.
// Added Link field to PCIe Link Pages
// Added EnclosureLevel and ConnectorName to PCIe
// Device Page 0.
// Added define for PCIE IoUnit page 1 max rate shift.
// Added comment for reserved ExtPageTypes.
// Added SAS 4 22.5 gbs speed support.
// Added PCIe 4 16.0 GT/sec speec support.
// Removed AHCI support.
// Removed SOP support.
// Added NegotiatedLinkRate and NegotiatedPortWidth to
// PCIe device page 0.
// 04-10-16  02.00.37  Fixed MPI2_MFGPAGE_DEVID_SAS3616/3708 defines
// 07-01-16  02.00.38  Added Manufacturing page 7 Connector types.
// Changed declaration of ConnectorName in PCIe DevicePage0
// to match SAS DevicePage 0.
// Added SATADeviceWaitTime to IO Unit Page 11.
// Added MPI26_MFGPAGE_DEVID_SAS4008
// Added x16 PCIe width to IO Unit Page 7
// Added LINKFLAGS to control SRIS in PCIe IO Unit page 1
// phy data.
// Added InitStatus to PCIe IO Unit Page 1 header.
// 09-01-16  02.00.39  Added MPI26_CONFIG_PAGE_ENCLOSURE_0 and related defines.
// Added MPI26_ENCLOS_PGAD_FORM_GET_NEXT_HANDLE and
// MPI26_ENCLOS_PGAD_FORM_HANDLE page address formats.
// 02-02-17  02.00.40  Added MPI2_MANPAGE7_SLOT_UNKNOWN.
// Added ChassisSlot field to SAS Enclosure Page 0.
// Added ChassisSlot Valid bit (bit 5) to the Flags field
// in SAS Enclosure Page 0.
// 06-13-17  02.00.41  Added MPI26_MFGPAGE_DEVID_SAS3816 and
// MPI26_MFGPAGE_DEVID_SAS3916 defines.
// Removed MPI26_MFGPAGE_DEVID_SAS4008 define.
// Added MPI26_PCIEIOUNIT1_LINKFLAGS_SRNS_EN define.
// Renamed PI26_PCIEIOUNIT1_LINKFLAGS_EN_SRIS to
// PI26_PCIEIOUNIT1_LINKFLAGS_SRIS_EN.
// Renamed MPI26_PCIEIOUNIT1_LINKFLAGS_DIS_SRIS to
// MPI26_PCIEIOUNIT1_LINKFLAGS_DIS_SEPARATE_REFCLK.
// 09-29-17  02.00.42  Added ControllerResetTO field to PCIe Device Page 2.
// Added NOIOB field to PCIe Device Page 2.
// Added MPI26_PCIEDEV2_CAP_DATA_BLK_ALIGN_AND_GRAN to
// the Capabilities field of PCIe Device Page 2.
// 07-22-18  02.00.43  Added defines for SAS3916 and SAS3816.
// Added WRiteCache defines to IO Unit Page 1.
// Added MaxEnclosureLevel to BIOS Page 1.
// Added OEMRD to SAS Enclosure Page 1.
// Added DMDReportPCIe to PCIe IO Unit Page 1.
// Added Flags field and flags for Retimers to
// PCIe Switch Page 1.
// 08-02-18  02.00.44  Added Slotx2, Slotx4 to ManPage 7.
// 08-15-18  02.00.45  Added ProductSpecific field at end of IOC Page 1
// 08-28-18  02.00.46  Added NVMs Write Cache flag to IOUnitPage1
// Added DMDReport Delay Time defines to
// PCIeIOUnitPage1
// --------------------------------------------------------------------------
// 08-02-18  02.00.44  Added Slotx2, Slotx4 to ManPage 7.
// 08-15-18  02.00.45  Added ProductSpecific field at end of IOC Page 1
// 08-28-18  02.00.46  Added NVMs Write Cache flag to IOUnitPage1
// Added DMDReport Delay Time defines to PCIeIOUnitPage1
// 12-17-18  02.00.47  Swap locations of Slotx2 and Slotx4 in ManPage 7.
// 08-01-19  02.00.49  Add MPI26_MANPAGE7_FLAG_X2_X4_SLOT_INFO_VALID
// Add MPI26_IOUNITPAGE1_NVME_WRCACHE_SHIFT
// 09-13-24  02.00.50  Added PCIe 32 GT/s link rate
//
// Configuration Page Header and defines
//
// Config Page Header
// Extended Config Page Header
// PTR_MPI2_CONFIG_EXTENDED_PAGE_HEADER,
// pMpi2ConfigExtendedPageHeader_t;
// PTR_MPI2_CONFIG_EXT_PAGE_HEADER_UNION,
// pMpi2ConfigPageExtendedHeaderUnion;
// PageType field values

// ExtPageType field values

//
// PageAddress defines
//
// RAID Volume PageAddress format

// RAID Physical Disk PageAddress format

// SAS Expander PageAddress format

// SAS Device PageAddress format

// SAS PHY PageAddress format

// SAS Port PageAddress format

// SAS Enclosure PageAddress format

// Enclosure PageAddress format

// RAID Configuration PageAddress format

// Driver Persistent Mapping PageAddress format

// Ethernet PageAddress format

// PCIe Switch PageAddress format

// PCIe Device PageAddress format

// PCIe Link PageAddress format

//
// Configuration messages
//
// Configuration Request Message
// values for the Action field

// use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
// Config Reply Message
//
// C o n f i g u r a t i o n    P a g e s
//
// Manufacturing Config pages
//

// MPI v2.0 SAS products

// MPI v2.5 SAS products

// MPI v2.6 SAS Products

// Manufacturing Page 0
// PTR_MPI2_CONFIG_PAGE_MAN_0,
// pMpi2ManufacturingPage0_t;

// Manufacturing Page 1
// PTR_MPI2_CONFIG_PAGE_MAN_1,
// pMpi2ManufacturingPage1_t;

// Manufacturing Page 2
//
// Host code (drivers, BIOS, utilities, etc.) should check Header.PageLength at
// runtime before using HwSettings[].
//
// PTR_MPI2_CONFIG_PAGE_MAN_2,
// pMpi2ManufacturingPage2_t;

// Manufacturing Page 3
//
// Host code (drivers, BIOS, utilities, etc.) should check Header.PageLength at
// runtime before using Info[].
//
// PTR_MPI2_CONFIG_PAGE_MAN_3,
// pMpi2ManufacturingPage3_t;

// Manufacturing Page 4
// PTR_MPI2_MANPAGE4_PWR_SAVE_SETTINGS,
// pMpi2ManPage4PwrSaveSettings_t;
// defines for the PowerSaveFlags field

// PTR_MPI2_CONFIG_PAGE_MAN_4,
// pMpi2ManufacturingPage4_t;

// Manufacturing Page 4 Flags field

// Manufacturing Page 5
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using Phy[].
//
// PTR_MPI2_MANUFACTURING5_ENTRY,
// pMpi2Manufacturing5Entry_t;
// PTR_MPI2_CONFIG_PAGE_MAN_5,
// pMpi2ManufacturingPage5_t;

// Manufacturing Page 6
// PTR_MPI2_CONFIG_PAGE_MAN_6,
// pMpi2ManufacturingPage6_t;

// Manufacturing Page 7
// PTR_MPI2_MANPAGE7_CONNECTOR_INFO,
// pMpi2ManPage7ConnectorInfo_t;
// defines for the Pinout field

// defines for the Location field

// defines for the Slot field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using ConnectorInfo[].
//
// PTR_MPI2_CONFIG_PAGE_MAN_7,
// pMpi2ManufacturingPage7_t;

// defines for the Flags field

//
// Generic structure to use for product-specific manufacturing pages
// (currently Manufacturing Page 8 through Manufacturing Page 31).
//
// PTR_MPI2_CONFIG_PAGE_MAN_PS,
// pMpi2ManufacturingPagePS_t;

//
// IO Unit Config Pages
//
// IO Unit Page 0
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_0,

// IO Unit Page 1
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_1,

// IO Unit Page 1 Flags defines

// IO Unit Page 3
//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// 36 and check the value returned for GPIOCount at runtime.
//

// PTR_MPI2_CONFIG_PAGE_IO_UNIT_3,

// defines for IO Unit Page 3 GPIOVal field

// IO Unit Page 5
//
// Upper layer code (drivers, utilities, etc.) should check the value returned
// for NumDmaEngines at runtime before using DmaEngineCapabilities[].
//
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_5,

// defines for IO Unit Page 5 DmaEngineCapabilities field

// IO Unit Page 6
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_6,

// defines for IO Unit Page 6 Flags field

// IO Unit Page 7
// reserved prior to MPI v2.6
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_7,

// defines for IO Unit Page 7 CurrentPowerMode and PreviousPowerMode fields

// defines for IO Unit Page 7 PCIeWidth field

// defines for IO Unit Page 7 PCIeSpeed field

// defines for IO Unit Page 7 ProcessorState field

// defines for IO Unit Page 7 PowerManagementCapabilities field

// obsolete names for the PowerManagementCapabilities bits (above)

// defines for IO Unit Page 7 IOCTemperatureUnits field

// defines for IO Unit Page 7 IOCSpeed field

// defines for IO Unit Page 7 BoardTemperatureUnits field

// defines for IO Unit Page 7 Flags field

// IO Unit Page 8

// defines for IO Unit Page 8 Sensor Flags field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumSensors at runtime before using Sensor[].
//
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_8,

// IO Unit Page 9
// defines for IO Unit Page 9 Sensor Flags field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumSensors at runtime before using Sensor[].
//
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_9,

// IO Unit Page 10
// PTR_MPI2_IOUNIT10_FUNCTION,
// pMpi2IOUnit10Function_t;
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumFunctions at runtime before using Function[].
//
// PTR_MPI2_CONFIG_PAGE_IO_UNIT_10,

// IO Unit Page 11 (for MPI v2.6 and later)
// PTR_MPI26_IOUNIT11_SPINUP_GROUP,
// pMpi26IOUnit11SpinupGroup_t;
// defines for IO Unit Page 11 SpinupFlags

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// four and check the value returned for NumPhys at runtime.
//

// PTR_MPI26_CONFIG_PAGE_IO_UNIT_11,
// pMpi26IOUnitPage11_t;

// defines for Flags field

// defines for PHY field

//
// IOC Config Pages
//
// IOC Page 0
// PTR_MPI2_CONFIG_PAGE_IOC_0,

// IOC Page 1
// PTR_MPI2_CONFIG_PAGE_IOC_1,

// defines for IOC Page 1 Flags field

// IOC Page 6
// PTR_MPI2_CONFIG_PAGE_IOC_6,

// defines for IOC Page 6 CapabilitiesFlags

// IOC Page 7

// PTR_MPI2_CONFIG_PAGE_IOC_7,

// IOC Page 8
// PTR_MPI2_CONFIG_PAGE_IOC_8,

// defines for IOC Page 8 Flags field

// defines for IOC Page 8 IRVolumeMappingFlags

//
// BIOS Config Pages
//
// BIOS Page 1
// PTR_MPI2_CONFIG_PAGE_BIOS_1,

// values for BIOS Page 1 BiosOptions field

// values for BIOS Page 1 IOCSettings field

// values for BIOS Page 1 DeviceSettings field

// defines for BIOS Page 1 UEFIVersion field

// BIOS Page 2
// PTR_MPI2_BOOT_DEVICE_ADAPTER_ORDER,
// pMpi2BootDeviceAdapterOrder_t;
// PTR_MPI2_BOOT_DEVICE_SAS_WWID,
// pMpi2BootDeviceSasWwid_t;
// PTR_MPI2_BOOT_DEVICE_ENCLOSURE_SLOT,
// pMpi2BootDeviceEnclosureSlot_t;
// PTR_MPI2_BOOT_DEVICE_DEVICE_NAME,
// pMpi2BootDeviceDeviceName_t;
// PTR_MPI2_BIOSPAGE2_BOOT_DEVICE,
// pMpi2BiosPage2BootDevice_t;

// values for BIOS Page 2 BootDeviceForm fields

// BIOS Page 3

// PTR_MPI2_CONFIG_PAGE_BIOS_3,

// values for BIOS Page 3 GlobalFlags

// BIOS Page 4
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using Phy[].
//

//
// RAID Volume Config Pages
//
// RAID Volume Page 0
// defines for the PhysDiskMap field

// pMpi2RaidVol0Settings_t;
// RAID Volume Page 0 HotSparePool defines, also used in RAID Physical Disk

// RAID Volume Page 0 VolumeSettings defines

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhysDisks at runtime before using PhysDisk[].
//
// PTR_MPI2_CONFIG_PAGE_RAID_VOL_0,

// values for RAID VolumeState

// values for RAID VolumeType

// values for RAID Volume Page 0 VolumeStatusFlags field

// values for RAID Volume Page 0 SupportedPhysDisks field

// values for RAID Volume Page 0 InactiveStatus field

// RAID Volume Page 1
// PTR_MPI2_CONFIG_PAGE_RAID_VOL_1,

//
// RAID Physical Disk Config Pages
//
// RAID Physical Disk Page 0
// PTR_MPI2_RAIDPHYSDISK0_SETTINGS,
// pMpi2RaidPhysDisk0Settings_t;
// use MPI2_RAID_HOT_SPARE_POOL_ defines for the HotSparePool field
// PTR_MPI2_RAIDPHYSDISK0_INQUIRY_DATA,
// pMpi2RaidPhysDisk0InquiryData_t;
// PTR_MPI2_CONFIG_PAGE_RD_PDISK_0,
// pMpi2RaidPhysDiskPage0_t;

// PhysDiskState defines

// OfflineReason defines

// IncompatibleReason defines

// PhysDiskAttributes defines

// PhysDiskStatusFlags defines

// RAID Physical Disk Page 1
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhysDiskPaths at runtime before using PhysicalDiskPath[].
//
// pMpi2RaidPhysDisk1Path_t;
// RAID Physical Disk Page 1 Physical Disk Path Flags field defines

// PTR_MPI2_CONFIG_PAGE_RD_PDISK_1,
// pMpi2RaidPhysDiskPage1_t;

//
// values for fields used by several types of SAS Config Pages
//
// values for NegotiatedLinkRates fields

// link rates used for Negotiated Physical and Logical Link Rate

// values for AttachedPhyInfo fields

// values for PhyInfo fields

// values for SAS ProgrammedLinkRate fields

// values for SAS HwLinkRate fields

//
// SAS IO Unit Config Pages
//
// SAS IO Unit Page 0
// PTR_MPI2_SAS_IO_UNIT0_PHY_DATA,
// pMpi2SasIOUnit0PhyData_t;
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using PhyData[].
//
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_0,

// values for SAS IO Unit Page 0 PortFlags

// values for SAS IO Unit Page 0 PhyFlags

// use MPI2_SAS_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// see mpi2_sas.h for values for
// SAS IO Unit Page 0 ControllerPhyDeviceInfo values
// values for SAS IO Unit Page 0 DiscoveryStatus

// SAS IO Unit Page 1
// PTR_MPI2_SAS_IO_UNIT1_PHY_DATA,
// pMpi2SasIOUnit1PhyData_t;
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using PhyData[].
//
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_1,

// values for SAS IO Unit Page 1 ControlFlags

// values for SAS IO Unit Page 1 AdditionalControlFlags

// defines for SAS IO Unit Page 1 ReportDeviceMissingDelay

// values for SAS IO Unit Page 1 PortFlags

// values for SAS IO Unit Page 1 PhyFlags

// values for SAS IO Unit Page 1 MaxMinLinkRate

// see mpi2_sas.h for values for
// SAS IO Unit Page 1 ControllerPhyDeviceInfo values
// SAS IO Unit Page 4 (for MPI v2.5 and earlier)
// PTR_MPI2_SAS_IOUNIT4_SPINUP_GROUP,
// pMpi2SasIOUnit4SpinupGroup_t;
// defines for SAS IO Unit Page 4 SpinupFlags

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check the value returned for NumPhys at runtime.
//

// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_4,

// defines for Flags field

// defines for PHY field

// SAS IO Unit Page 5
// PTR_MPI2_SAS_IO_UNIT5_PHY_PM_SETTINGS,
// pMpi2SasIOUnit5PhyPmSettings_t;
// defines for ControlFlags field

// defines for PortWidthModeGroup field

// defines for InactivityTimerExponent field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using SASPhyPowerManagementSettings[].
//
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_5,

// SAS IO Unit Page 6
// PTR_MPI2_SAS_IO_UNIT6_PORT_WIDTH_MOD_GROUP_STATUS,
// pMpi2SasIOUnit6PortWidthModGroupStatus_t;
// defines for CurrentStatus field

// defines for CurrentModulation field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumGroups at runtime before using PortWidthModulationGroupStatus[].
//
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_6,

// SAS IO Unit Page 7
// PTR_MPI2_SAS_IO_UNIT7_PORT_WIDTH_MOD_GROUP_SETTINGS,
// pMpi2SasIOUnit7PortWidthModGroupSettings_t;
// defines for Flags field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumGroups at runtime before using PortWidthModulationGroupSettings[].
//
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_7,

// SAS IO Unit Page 8
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT_8,

// defines for PowerManagementCapabilities field

// defines for TxRxSleepStatus field

// SAS IO Unit Page 16
// PTR_MPI2_CONFIG_PAGE_SASIOUNIT16,

//
// SAS Expander Config Pages
//
// SAS Expander Page 0
// PTR_MPI2_CONFIG_PAGE_EXPANDER_0,

// values for SAS Expander Page 0 DiscoveryStatus field

// values for SAS Expander Page 0 Flags field

// SAS Expander Page 1
// PTR_MPI2_CONFIG_PAGE_EXPANDER_1,

// use MPI2_SAS_PRATE_ defines for the ProgrammedLinkRate field
// use MPI2_SAS_HWRATE_ defines for the HwLinkRate field
// use MPI2_SAS_PHYINFO_ for the PhyInfo field
// see mpi2_sas.h for the MPI2_SAS_DEVICE_INFO_ defines
// used for the AttachedDeviceInfo field
// use MPI2_SAS_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// values for SAS Expander Page 1 DiscoveryInfo field

// use MPI2_SAS_APHYINFO_ defines for AttachedPhyInfo field
//
// SAS Device Config Pages
//
// SAS Device Page 0
// PTR_MPI2_CONFIG_PAGE_SAS_DEV_0,
// pMpi2SasDevicePage0_t;

// values for SAS Device Page 0 AccessStatus field

// specific values for SATA Init failures

// see mpi2_sas.h for values for SAS Device Page 0 DeviceInfo values
// values for SAS Device Page 0 Flags field

// SAS Device Page 1
// PTR_MPI2_CONFIG_PAGE_SAS_DEV_1,
// pMpi2SasDevicePage1_t;

//
// SAS PHY Config Pages
//
// SAS PHY Page 0
// PTR_MPI2_CONFIG_PAGE_SAS_PHY_0,

// use MPI2_SAS_APHYINFO_ defines for AttachedPhyInfo field
// use MPI2_SAS_PRATE_ defines for the ProgrammedLinkRate field
// use MPI2_SAS_HWRATE_ defines for the HwLinkRate field
// values for SAS PHY Page 0 Flags field

// use MPI2_SAS_PHYINFO_ for the PhyInfo field
// use MPI2_SAS_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// SAS PHY Page 1
// PTR_MPI2_CONFIG_PAGE_SAS_PHY_1,

// SAS PHY Page 2
// use MPI2_SASPHY3_EVENT_CODE_ for the PhyEventCode field
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhyEvents at runtime before using PhyEvent[].
//
// PTR_MPI2_CONFIG_PAGE_SAS_PHY_2,
// pMpi2SasPhyPage2_t;

// SAS PHY Page 3
// PTR_MPI2_SASPHY3_PHY_EVENT_CONFIG,
// pMpi2SasPhy3PhyEventConfig_t;
// values for PhyEventCode field

// Following codes are product specific and in MPI v2.6 and later

// values for the CounterType field

// values for the TimeUnits field

// values for the ThresholdFlags field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhyEvents at runtime before using PhyEventConfig[].
//
// PTR_MPI2_CONFIG_PAGE_SAS_PHY_3,

// SAS PHY Page 4
// PTR_MPI2_CONFIG_PAGE_SAS_PHY_4,

// values for the Flags field

//
// SAS Port Config Pages
//
// SAS Port Page 0
// PTR_MPI2_CONFIG_PAGE_SAS_PORT_0,

// see mpi2_sas.h for values for SAS Port Page 0 DeviceInfo values
//
// SAS Enclosure Config Pages
//
// SAS Enclosure Page 0
// PTR_MPI2_CONFIG_PAGE_SAS_ENCLOSURE_0,
// PTR_MPI26_CONFIG_PAGE_ENCLOSURE_0,

// values for SAS Enclosure Page 0 Flags field

// Values for Enclosure Page 0 Flags field

//
// Log Config Page
//
// Log Page 0
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumLogEntries at runtime before using LogEntry[].
//

// values for Log Page 0 LogEntry LogEntryQualifier field

//
// RAID Config Page
//
// RAID Page 0
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumElements at runtime before using ConfigElement[].
//
// PTR_MPI2_RAIDCONFIG0_CONFIG_ELEMENT,
// pMpi2RaidConfig0ConfigElement_t;
// values for the ElementFlags field

// PTR_MPI2_CONFIG_PAGE_RAID_CONFIGURATION_0,
// pMpi2RaidConfigurationPage0_t;

// values for RAID Configuration Page 0 Flags field

//
// Driver Persistent Mapping Config Pages
//
// Driver Persistent Mapping Page 0
// PTR_MPI2_CONFIG_PAGE_DRIVER_MAP0_ENTRY,
// PTR_MPI2_CONFIG_PAGE_DRIVER_MAPPING_0,

// values for Driver Persistent Mapping Page 0 MappingInformation field

//
// Ethernet Config Pages
//
// Ethernet Page 0
// IP address (union of IPv4 and IPv6)

// PTR_MPI2_CONFIG_PAGE_ETHERNET_0,

// values for Ethernet Page 0 Status field

// values for Ethernet Page 0 MediaState field

// Ethernet Page 1
// PTR_MPI2_CONFIG_PAGE_ETHERNET_1,

// values for Ethernet Page 1 Flags field

// values for Ethernet Page 1 MediaState field

//
// Extended Manufacturing Config Pages
//
// Generic structure to use for product-specific extended manufacturing pages
// (currently Extended Manufacturing Page 40 through Extended Manufacturing
// Page 60).
//
// PTR_MPI2_CONFIG_PAGE_EXT_MAN_PS,
// pMpi2ExtManufacturingPagePS_t;
// PageVersion should be provided by product-specific code
//
// values for fields used by several types of PCIe Config Pages
//
// values for NegotiatedLinkRates fields

// link rates used for Negotiated Physical Link Rate

//
// PCIe IO Unit Config Pages (MPI v2.6 and later)
//
// PCIe IO Unit Page 0
// PTR_MPI26_PCIE_IO_UNIT0_PHY_DATA,
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using PhyData[].
//
// PTR_MPI26_CONFIG_PAGE_PIOUNIT_0,

// values for PCIe IO Unit Page 0 LinkFlags

// values for PCIe IO Unit Page 0 PhyFlags

// use MPI26_PCIE_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// see mpi2_pci.h for values for PCIe IO Unit Page 0 ControllerPhyDeviceInfo
// values
//
// values for PCIe IO Unit Page 0 EnumerationStatus

// PCIe IO Unit Page 1
// PTR_MPI26_PCIE_IO_UNIT1_PHY_DATA,
// values for LinkFlags

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumPhys at runtime before using PhyData[].
//
// PTR_MPI26_CONFIG_PAGE_PIOUNIT_1,

// values for PCIe IO Unit Page 1 PhyFlags

// values for PCIe IO Unit Page 1 MaxMinLinkRate

// values for PCIe IO Unit Page 1 DMDReportPCIe

// see mpi2_pci.h for values for PCIe IO Unit Page 0 ControllerPhyDeviceInfo
// values
//
// PCIe Switch Config Pages (MPI v2.6 and later)
//
// PCIe Switch Page 0

// PCIe Switch Page 1

// use MPI26_PCIE_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// defines for the Flags field

//
// PCIe Device Config Pages (MPI v2.6 and later)
//
// PCIe Device Page 0

// values for PCIe Device Page 0 AccessStatus field

// see mpi2_pci.h for the MPI26_PCIE_DEVINFO_ defines used for the DeviceInfo
// field
//
// values for PCIe Device Page 0 Flags field

// values for PCIe Device Page 0 SupportedLinkRates field

// use MPI26_PCIE_NEG_LINK_RATE_ defines for the NegotiatedLinkRate field
// PCIe Device Page 2

// defines for PCIe Device Page 2 Capabilities field

// Defines for the NOIOB field

//
// PCIe Link Config Pages (MPI v2.6 and later)
//
// PCIe Link Page 1

// PCIe Link Page 2
// use MPI26_PCIELINK3_EVTCODE_ for the LinkEventCode field
//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumLinkEvents at runtime before using LinkEvent[].
//

// PCIe Link Page 3
// values for LinkEventCode field

// values for the CounterType field

// values for the TimeUnits field

// values for the ThresholdFlags field

//
// Host code (drivers, BIOS, utilities, etc.) should check the value returned
// for NumLinkEvents at runtime before using LinkEventConfig[].
//

