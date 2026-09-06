//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_cnfg.h
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
// Copyright (c) 2000-2008 LSI Corporation.
//
// Name:  mpi_cnfg.h
// Title:  MPI Config message, structures, and Pages
// Creation Date:  July 27, 2000
//
// mpi_cnfg.h Version:  01.05.18
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 06-08-00  01.00.02  Added _PAGEVERSION definitions for all pages.
// Added FcPhLowestVersion, FcPhHighestVersion, Reserved2
// fields to FC_DEVICE_0 page, updated the page version.
// Changed _FREE_RUNNING_CLOCK to _PACING_TRANSFERS in
// SCSI_PORT_0, SCSI_DEVICE_0 and SCSI_DEVICE_1 pages
// and updated the page versions.
// Added _RESPONSE_ID_MASK definition to SCSI_PORT_1
// page and updated the page version.
// Added Information field and _INFO_PARAMS_NEGOTIATED
// definitionto SCSI_DEVICE_0 page.
// 06-22-00  01.00.03  Removed batch controls from LAN_0 page and updated the
// page version.
// Added BucketsRemaining to LAN_1 page, redefined the
// state values, and updated the page version.
// Revised bus width definitions in SCSI_PORT_0,
// SCSI_DEVICE_0 and SCSI_DEVICE_1 pages.
// 06-30-00  01.00.04  Added MaxReplySize to LAN_1 page and updated the page
// version.
// Moved FC_DEVICE_0 PageAddress description to spec.
// 07-27-00  01.00.05  Corrected the SubsystemVendorID and SubsystemID field
// widths in IOC_0 page and updated the page version.
// 11-02-00  01.01.01  Original release for post 1.0 work
// Added Manufacturing pages, IO Unit Page 2, SCSI SPI
// Port Page 2, FC Port Page 4, FC Port Page 5
// 11-15-00  01.01.02  Interim changes to match proposals
// 12-04-00  01.01.03  Config page changes to match MPI rev 1.00.01.
// 12-05-00  01.01.04  Modified config page actions.
// 01-09-01  01.01.05  Added defines for page address formats.
// Data size for Manufacturing pages 2 and 3 no longer
// defined here.
// Io Unit Page 2 size is fixed at 4 adapters and some
// flags were changed.
// SCSI Port Page 2 Device Settings modified.
// New fields added to FC Port Page 0 and some flags
// cleaned up.
// Removed impedance flash from FC Port Page 1.
// Added FC Port pages 6 and 7.
// 01-25-01  01.01.06  Added MaxInitiators field to FcPortPage0.
// 01-29-01  01.01.07  Changed some defines to make them 32 character unique.
// Added some LinkType defines for FcPortPage0.
// 02-20-01  01.01.08  Started using MPI_POINTER.
// 02-27-01  01.01.09  Replaced MPI_CONFIG_PAGETYPE_SCSI_LUN with
// MPI_CONFIG_PAGETYPE_RAID_VOLUME.
// Added definitions and structures for IOC Page 2 and
// RAID Volume Page 2.
// 03-27-01  01.01.10  Added CONFIG_PAGE_FC_PORT_8 and CONFIG_PAGE_FC_PORT_9.
// CONFIG_PAGE_FC_PORT_3 now supports persistent by DID.
// Added VendorId and ProductRevLevel fields to
// RAIDVOL2_IM_PHYS_ID struct.
// Modified values for MPI_FCPORTPAGE0_FLAGS_ATTACH_
// defines to make them compatible to MPI version 1.0.
// Added structure offset comments.
// 04-09-01  01.01.11  Added some new defines for the PageAddress field and
// removed some obsolete ones.
// Added IO Unit Page 3.
// Modified defines for Scsi Port Page 2.
// Modified RAID Volume Pages.
// 08-08-01  01.02.01  Original release for v1.2 work.
// Added SepID and SepBus to RVP2 IMPhysicalDisk struct.
// Added defines for the SEP bits in RVP2 VolumeSettings.
// Modified the DeviceSettings field in RVP2 to use the
// proper structure.
// Added defines for SES, SAF-TE, and cross channel for
// IOCPage2 CapabilitiesFlags.
// Removed define for MPI_IOUNITPAGE2_FLAGS_RAID_DISABLE.
// Removed define for
// MPI_SCSIPORTPAGE2_PORT_FLAGS_PARITY_ENABLE.
// Added define for MPI_CONFIG_PAGEATTR_RO_PERSISTENT.
// 08-29-01 01.02.02   Fixed value for MPI_MANUFACTPAGE_DEVID_53C1035.
// Added defines for MPI_FCPORTPAGE1_FLAGS_HARD_ALPA_ONLY
// and MPI_FCPORTPAGE1_FLAGS_IMMEDIATE_ERROR_REPLY.
// Removed MPI_SCSIPORTPAGE0_CAP_PACING_TRANSFERS,
// MPI_SCSIDEVPAGE0_NP_PACING_TRANSFERS, and
// MPI_SCSIDEVPAGE1_RP_PACING_TRANSFERS, and
// MPI_SCSIDEVPAGE1_CONF_PPR_ALLOWED.
// Added defines for MPI_SCSIDEVPAGE1_CONF_WDTR_DISALLOWED
// and MPI_SCSIDEVPAGE1_CONF_SDTR_DISALLOWED.
// Added OnBusTimerValue to CONFIG_PAGE_SCSI_PORT_1.
// Added rejected bits to SCSI Device Page 0 Information.
// Increased size of ALPA array in FC Port Page 2 by one
// and removed a one byte reserved field.
// 09-28-01 01.02.03   Swapped NegWireSpeedLow and NegWireSpeedLow in
// CONFIG_PAGE_LAN_1 to match preferred 64-bit ordering.
// Added structures for Manufacturing Page 4, IO Unit
// Page 3, IOC Page 3, IOC Page 4, RAID Volume Page 0, and
// RAID PhysDisk Page 0.
// 10-04-01 01.02.04   Added define for MPI_CONFIG_PAGETYPE_RAID_PHYSDISK.
// Modified some of the new defines to make them 32
// character unique.
// Modified how variable length pages (arrays) are defined.
// Added generic defines for hot spare pools and RAID
// volume types.
// 11-01-01 01.02.05   Added define for MPI_IOUNITPAGE1_DISABLE_IR.
// 03-14-02 01.02.06   Added PCISlotNum field to CONFIG_PAGE_IOC_1 along with
// related define, and bumped the page version define.
// 05-31-02 01.02.07   Added a Flags field to CONFIG_PAGE_IOC_2_RAID_VOL in a
// reserved byte and added a define.
// Added define for
// MPI_RAIDVOL0_STATUS_FLAG_VOLUME_INACTIVE.
// Added new config page: CONFIG_PAGE_IOC_5.
// Added MaxAliases, MaxHardAliases, and NumCurrentAliases
// fields to CONFIG_PAGE_FC_PORT_0.
// Added AltConnector and NumRequestedAliases fields to
// CONFIG_PAGE_FC_PORT_1.
// Added new config page: CONFIG_PAGE_FC_PORT_10.
// 07-12-02 01.02.08   Added more MPI_MANUFACTPAGE_DEVID_ defines.
// Added additional MPI_SCSIDEVPAGE0_NP_ defines.
// Added more MPI_SCSIDEVPAGE1_RP_ defines.
// Added define for
// MPI_SCSIDEVPAGE1_CONF_EXTENDED_PARAMS_ENABLE.
// Added new config page: CONFIG_PAGE_SCSI_DEVICE_3.
// Modified MPI_FCPORTPAGE5_FLAGS_ defines.
// 09-16-02 01.02.09   Added MPI_SCSIDEVPAGE1_CONF_FORCE_PPR_MSG define.
// 11-15-02 01.02.10   Added ConnectedID defines for CONFIG_PAGE_SCSI_PORT_0.
// Added more Flags defines for CONFIG_PAGE_FC_PORT_1.
// Added more Flags defines for CONFIG_PAGE_FC_DEVICE_0.
// 04-01-03 01.02.11   Added RR_TOV field and additional Flags defines for
// CONFIG_PAGE_FC_PORT_1.
// Added define MPI_FCPORTPAGE5_FLAGS_DISABLE to disable
// an alias.
// Added more device id defines.
// 06-26-03 01.02.12   Added MPI_IOUNITPAGE1_IR_USE_STATIC_VOLUME_ID define.
// Added TargetConfig and IDConfig fields to
// CONFIG_PAGE_SCSI_PORT_1.
// Added more PortFlags defines for CONFIG_PAGE_SCSI_PORT_2
// to control DV.
// Added more Flags defines for CONFIG_PAGE_FC_PORT_1.
// In CONFIG_PAGE_FC_DEVICE_0, replaced Reserved1 field
// with ADISCHardALPA.
// Added MPI_FC_DEVICE_PAGE0_PROT_FCP_RETRY define.
// 01-16-04 01.02.13   Added InitiatorDeviceTimeout and InitiatorIoPendTimeout
// fields and related defines to CONFIG_PAGE_FC_PORT_1.
// Added define for
// MPI_FCPORTPAGE1_FLAGS_SOFT_ALPA_FALLBACK.
// Added new fields to the substructures of
// CONFIG_PAGE_FC_PORT_10.
// 04-29-04 01.02.14   Added define for IDP bit for CONFIG_PAGE_SCSI_PORT_0,
// CONFIG_PAGE_SCSI_DEVICE_0, and
// CONFIG_PAGE_SCSI_DEVICE_1. Also bumped Page Version for
// these pages.
// 05-11-04 01.03.01   Added structure for CONFIG_PAGE_INBAND_0.
// 08-19-04 01.05.01   Modified MSG_CONFIG request to support extended config
// pages.
// Added a new structure for extended config page header.
// Added new extended config pages types and structures for
// SAS IO Unit, SAS Expander, SAS Device, and SAS PHY.
// Replaced a reserved byte in CONFIG_PAGE_MANUFACTURING_4
// to add a Flags field.
// Two new Manufacturing config pages (5 and 6).
// Two new bits defined for IO Unit Page 1 Flags field.
// Modified CONFIG_PAGE_IO_UNIT_2 to add three new fields
// to specify the BIOS boot device.
// Four new Flags bits defined for IO Unit Page 2.
// Added IO Unit Page 4.
// Added EEDP Flags settings to IOC Page 1.
// Added new BIOS Page 1 config page.
// 10-05-04 01.05.02   Added define for
// MPI_IOCPAGE1_INITIATOR_CONTEXT_REPLY_DISABLE.
// Added new Flags field to CONFIG_PAGE_MANUFACTURING_5 and
// associated defines.
// Added more defines for SAS IO Unit Page 0
// DiscoveryStatus field.
// Added define for MPI_SAS_IOUNIT0_DS_SUBTRACTIVE_LINK
// and MPI_SAS_IOUNIT0_DS_TABLE_LINK.
// Added defines for Physical Mapping Modes to SAS IO Unit
// Page 2.
// Added define for
// MPI_SAS_DEVICE0_FLAGS_PORT_SELECTOR_ATTACH.
// 10-27-04 01.05.03   Added defines for new SAS PHY page addressing mode.
// Added defines for MaxTargetSpinUp to BIOS Page 1.
// Added 5 new ControlFlags defines for SAS IO Unit
// Page 1.
// Added MaxNumPhysicalMappedIDs field to SAS IO Unit
// Page 2.
// Added AccessStatus field to SAS Device Page 0 and added
// new Flags bits for supported SATA features.
// 12-07-04  01.05.04  Added config page structures for BIOS Page 2, RAID
// Volume Page 1, and RAID Physical Disk Page 1.
// Replaced IO Unit Page 1 BootTargetID,BootBus, and
// BootAdapterNum with reserved field.
// Added DataScrubRate and ResyncRate to RAID Volume
// Page 0.
// Added MPI_SAS_IOUNIT2_FLAGS_RESERVE_ID_0_FOR_BOOT
// define.
// 12-09-04  01.05.05  Added Target Mode Large CDB Enable to FC Port Page 1
// Flags field.
// Added Auto Port Config flag define for SAS IOUNIT
// Page 1 ControlFlags.
// Added Disabled bad Phy define to Expander Page 1
// Discovery Info field.
// Added SAS/SATA device support to SAS IOUnit Page 1
// ControlFlags.
// Added Unsupported device to SAS Dev Page 0 Flags field
// Added disable use SATA Hash Address for SAS IOUNIT
// page 1 in ControlFields.
// 01-15-05  01.05.06  Added defaults for data scrub rate and resync rate to
// Manufacturing Page 4.
// Added new defines for BIOS Page 1 IOCSettings field.
// Added ExtDiskIdentifier field to RAID Physical Disk
// Page 0.
// Added new defines for SAS IO Unit Page 1 ControlFlags
// and to SAS Device Page 0 Flags to control SATA devices.
// Added defines and structures for the new Log Page 0, a
// new type of configuration page.
// 02-09-05  01.05.07  Added InactiveStatus field to RAID Volume Page 0.
// Added WWID field to RAID Volume Page 1.
// Added PhysicalPort field to SAS Expander pages 0 and 1.
// 03-11-05  01.05.08  Removed the EEDP flags from IOC Page 1.
// Added Enclosure/Slot boot device format to BIOS Page 2.
// New status value for RAID Volume Page 0 VolumeStatus
// (VolumeState subfield).
// New value for RAID Physical Page 0 InactiveStatus.
// Added Inactive Volume Member flag RAID Physical Disk
// Page 0 PhysDiskStatus field.
// New physical mapping mode in SAS IO Unit Page 2.
// Added CONFIG_PAGE_SAS_ENCLOSURE_0.
// Added Slot and Enclosure fields to SAS Device Page 0.
// 06-24-05  01.05.09  Added EEDP defines to IOC Page 1.
// Added more RAID type defines to IOC Page 2.
// Added Port Enable Delay settings to BIOS Page 1.
// Added Bad Block Table Full define to RAID Volume Page 0.
// Added Previous State defines to RAID Physical Disk
// Page 0.
// Added Max Sata Targets define for DiscoveryStatus field
// of SAS IO Unit Page 0.
// Added Device Self Test to Control Flags of SAS IO Unit
// Page 1.
// Added Direct Attach Starting Slot Number define for SAS
// IO Unit Page 2.
// Added new fields in SAS Device Page 2 for enclosure
// mapping.
// Added OwnerDevHandle and Flags field to SAS PHY Page 0.
// Added IOC GPIO Flags define to SAS Enclosure Page 0.
// Fixed the value for MPI_SAS_IOUNIT1_CONTROL_DEV_SATA_SUPPORT.
// 08-03-05  01.05.10  Removed ISDataScrubRate and ISResyncRate from
// Manufacturing Page 4.
// Added MPI_IOUNITPAGE1_SATA_WRITE_CACHE_DISABLE bit.
// Added NumDevsPerEnclosure field to SAS IO Unit page 2.
// Added MPI_SAS_IOUNIT2_FLAGS_HOST_ASSIGNED_PHYS_MAP
// define.
// Added EnclosureHandle field to SAS Expander page 0.
// Removed redundant NumTableEntriesProg field from SAS
// Expander Page 1.
// 08-30-05  01.05.11  Added DeviceID for FC949E and changed the DeviceID for
// SAS1078.
// Added more defines for Manufacturing Page 4 Flags field.
// Added more defines for IOCSettings and added
// ExpanderSpinup field to Bios Page 1.
// Added postpone SATA Init bit to SAS IO Unit Page 1
// ControlFlags.
// Changed LogEntry format for Log Page 0.
// 03-27-06  01.05.12  Added two new Flags defines for Manufacturing Page 4.
// Added Manufacturing Page 7.
// Added MPI_IOCPAGE2_CAP_FLAGS_RAID_64_BIT_ADDRESSING.
// Added IOC Page 6.
// Added PrevBootDeviceForm field to CONFIG_PAGE_BIOS_2.
// Added MaxLBAHigh field to RAID Volume Page 0.
// Added Nvdata version fields to SAS IO Unit Page 0.
// Added AdditionalControlFlags, MaxTargetPortConnectTime,
// ReportDeviceMissingDelay, and IODeviceMissingDelay
// fields to SAS IO Unit Page 1.
// 10-11-06  01.05.13  Added NumForceWWID field and ForceWWID array to
// Manufacturing Page 5.
// Added Manufacturing pages 8 through 10.
// Added defines for supported metadata size bits in
// CapabilitiesFlags field of IOC Page 6.
// Added defines for metadata size bits in VolumeSettings
// field of RAID Volume Page 0.
// Added SATA Link Reset settings, Enable SATA Asynchronous
// Notification bit, and HideNonZeroAttachedPhyIdentifiers
// bit to AdditionalControlFlags field of SAS IO Unit
// Page 1.
// Added defines for Enclosure Devices Unmapped and
// Device Limit Exceeded bits in Status field of SAS IO
// Unit Page 2.
// Added more AccessStatus values for SAS Device Page 0.
// Added bit for SATA Asynchronous Notification Support in
// Flags field of SAS Device Page 0.
// 02-28-07  01.05.14  Added ExtFlags field to Manufacturing Page 4.
// Added Disable SMART Polling for CapabilitiesFlags of
// IOC Page 6.
// Added Disable SMART Polling to DeviceSettings of BIOS
// Page 1.
// Added Multi-Port Domain bit for DiscoveryStatus field
// of SAS IO Unit Page.
// Added Multi-Port Domain Illegal flag for SAS IO Unit
// Page 1 AdditionalControlFlags field.
// 05-24-07  01.05.15  Added Hide Physical Disks with Non-Integrated RAID
// Metadata bit to Manufacturing Page 4 ExtFlags field.
// Added Internal Connector to End Device Present bit to
// Expander Page 0 Flags field.
// Fixed define for
// MPI_SAS_EXPANDER1_DISCINFO_BAD_PHY_DISABLED.
// 08-07-07  01.05.16  Added MPI_IOCPAGE6_CAP_FLAGS_MULTIPORT_DRIVE_SUPPORT
// define.
// Added BIOS Page 4 structure.
// Added MPI_RAID_PHYS_DISK1_PATH_MAX define for RAID
// Physical Disk Page 1.
// 01-15-07  01.05.17  Added additional bit defines for ExtFlags field of
// Manufacturing Page 4.
// Added Solid State Drives Supported bit to IOC Page 6
// Capabilities Flags.
// Added new value for AccessStatus field of SAS Device
// Page 0 (_SATA_NEEDS_INITIALIZATION).
// 03-28-08  01.05.18  Defined new bits in Manufacturing Page 4 ExtFlags field
// to control coercion size and the mixing of SAS and SATA
// SSD drives.
// --------------------------------------------------------------------------
//
// C o n f i g    M e s s a g e    a n d    S t r u c t u r e s
//
// PageType field values
//

//
// ExtPageType field values
//

//
// PageAddress field values
//

//
// Config Request Message
//
// Action field values
//

// Config Reply Message
//
// C o n f i g u r a t i o n    P a g e s
//
// Manufacturing Config pages
//

// Fibre Channel

// SCSI

// SAS

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

// defines for the Flags field

// defines for the ExtFlags field

// defines for the Flags field

// defines for the Pinout field

// defines for the Location field

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check NumPhys at runtime.
//

// defines for the Flags field

//
// IO Unit Config Pages
//

// IO Unit Page 1 Flags defines

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

//
// IOC Config Pages
//

// defines for the Flags field

// IOC Page 2 Volume RAID Type values, also used in RAID Volume pages

// IOC Page 2 Volume Flags values

// IOC Page 2 Capabilities flags

// IOC Page 5 HotSpare Flags

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

// IOC Page 6 Capabilities Flags

//
// BIOS Config Pages
//

// values for the BiosOptions field

// values for the IOCSettings field

// values for the DeviceSettings field

// defines for the ExpanderSpinup field

//
// SCSI Port Config Pages
//

// Configuration values

// TargetConfig values

// PortFlags values

// PortSettings values

//
// SCSI Target Device Config Pages
//

//
// FC Port Config Pages
//

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

// standard MODDEF pin definitions (from GBIC spec.)

//
// FC Device Config Pages
//

//
// RAID Volume Config Pages
//

// RAID Volume Page 0 VolumeStatus defines

// RAID Volume Page 0 VolumeSettings defines

// RAID Volume Page 0 HotSparePool defines, also used in RAID Physical Disk

// values for RAID Volume Page 0 InactiveStatus field

//
// RAID Physical Disk Config Pages
//
// RAID Physical Disk PhysDiskStatus flags

// RAID Physical Disk Page 1 Flags field defines

//
// LAN Config Pages
//

//
// Inband Config Pages
//

//
// SAS IO Unit Config Pages
//

// values for SAS IO Unit Page 0 PortFlags

// values for SAS IO Unit Page 0 PhyFlags

// values for SAS IO Unit Page 0 NegotiatedLinkRate

// see mpi_sas.h for values for SAS IO Unit Page 0 ControllerPhyDeviceInfo values
// values for SAS IO Unit Page 0 DiscoveryStatus

//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check Header.PageLength at runtime.
//

// values for SAS IO Unit Page 1 ControlFlags

// values for SAS IO Unit Page 1 AdditionalControlFlags

// defines for SAS IO Unit Page 1 ReportDeviceMissingDelay

// values for SAS IO Unit Page 1 PortFlags

// values for SAS IO Unit Page 0 PhyFlags

// values for SAS IO Unit Page 0 MaxMinLinkRate

// see mpi_sas.h for values for SAS IO Unit Page 1 ControllerPhyDeviceInfo values

// values for SAS IO Unit Page 2 Status field

// values for SAS IO Unit Page 2 Flags field

// Physical Mapping Modes

//
// SAS Expander Config Pages
//

// values for SAS Expander Page 0 DiscoveryStatus field

// values for SAS Expander Page 0 Flags field

// use MPI_SAS_PHY0_PRATE_ defines for ProgrammedLinkRate
// use MPI_SAS_PHY0_HWRATE_ defines for HwLinkRate
// use MPI_SAS_PHY0_PHYINFO_ defines for PhyInfo
// see mpi_sas.h for values for SAS Expander Page 1 AttachedDeviceInfo values
// values for SAS Expander Page 1 DiscoveryInfo field

// values for SAS Expander Page 1 NegotiatedLinkRate field

//
// SAS Device Config Pages
//

// values for SAS Device Page 0 AccessStatus field

// specific values for SATA Init failures

// values for SAS Device Page 0 Flags field

// see mpi_sas.h for values for SAS Device Page 0 DeviceInfo values

// defines for SAS Device Page 2 EnclosureMapping field

//
// SAS PHY Config Pages
//

// values for SAS PHY Page 0 ProgrammedLinkRate field

// values for SAS PHY Page 0 HwLinkRate field

// values for SAS PHY Page 0 Flags field

// values for SAS PHY Page 0 PhyInfo field

//
// SAS Enclosure Config Pages
//

// values for SAS Enclosure Page 0 Flags field

//
// Log Config Pages
//
// Host code (drivers, BIOS, utilities, etc.) should leave this define set to
// one and check NumLogEntries at runtime.
//

// values for Log Page 0 LogEntry LogEntryQualifier field

