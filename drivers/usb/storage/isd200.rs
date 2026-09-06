//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/isd200.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Transport & Protocol Driver for In-System Design, Inc. ISD200 ASIC
//
// Current development and maintenance:
// (C) 2001-2002 Björn Stenberg (bjorn@haxx.se)
//
// Developed with the assistance of:
// (C) 2002 Alan Stern <stern@rowland.org>
//
// Initial work:
// (C) 2000 In-System Design, Inc. (support@in-system.com)
//
// The ISD200 ASIC does not natively support ATA devices.  The chip
// does implement an interface, the ATA Command Block (ATACB) which provides
// a means of passing ATA commands and ATA register accesses to a device.
//
// History:
//
// 2002-10-19: Removed the specialized transfer routines.
// (Alan Stern <stern@rowland.harvard.edu>)
// 2001-02-24: Removed lots of duplicate code and simplified the structure.
// (bjorn@haxx.se)
// 2002-01-16: Fixed endianness bug so it works on the ppc arch.
// (Luc Saillard <luc@saillard.org>)
// 2002-01-17: All bitfields removed.
// (bjorn@haxx.se)
//
// Include files

    MODULE_DESCRIPTION("Driver for In-System Design, Inc. ISD200 ASIC");
    MODULE_AUTHOR("Björn Stenberg <bjorn@haxx.se>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USB_STORAGE");
    static int isd200_Initialization(struct us_data *us);
//
// The table of devices
//

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (flags) }
    static const struct usb_device_id isd200_usb_ids[] = {

    { }		/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, isd200_usb_ids);

//
// The flags table
//

    vendor_name, product_name, use_protocol, use_transport, \
    init_function, Flags) \
    { \
    .vendorName = vendor_name,	\
    .productName = product_name,	\
    .useProtocol = use_protocol,	\
    .useTransport = use_transport,	\
    .initFunction = init_function,	\
    }
    static const struct us_unusual_dev isd200_unusual_dev_list[] = {

    { }		/* Terminating entry */
    };

// Timeout defines (in Seconds)
pub const ISD200_ENUM_BSY_TIMEOUT: c_int = 35;
pub const ISD200_ENUM_DETECT_TIMEOUT: c_int = 30;
pub const ISD200_DEFAULT_TIMEOUT: c_int = 30;
// device flags
pub const DF_ATA_DEVICE: c_uint = 0x0001;
pub const DF_MEDIA_STATUS_ENABLED: c_uint = 0x0002;
pub const DF_REMOVABLE_MEDIA: c_uint = 0x0004;
// capability bit definitions
pub const CAPABILITY_DMA: c_uint = 0x01;
pub const CAPABILITY_LBA: c_uint = 0x02;
// command_setX bit definitions
pub const COMMANDSET_REMOVABLE: c_uint = 0x02;
pub const COMMANDSET_MEDIA_STATUS: c_uint = 0x10;
// ATA Vendor Specific defines
pub const ATA_ADDRESS_DEVHEAD_STD: c_uint = 0xa0;
pub const ATA_ADDRESS_DEVHEAD_LBA_MODE: c_uint = 0x40;
pub const ATA_ADDRESS_DEVHEAD_SLAVE: c_uint = 0x10;
// Action Select bits
pub const ACTION_SELECT_0: c_uint = 0x01;
pub const ACTION_SELECT_1: c_uint = 0x02;
pub const ACTION_SELECT_2: c_uint = 0x04;
pub const ACTION_SELECT_3: c_uint = 0x08;
pub const ACTION_SELECT_4: c_uint = 0x10;
pub const ACTION_SELECT_5: c_uint = 0x20;
pub const ACTION_SELECT_6: c_uint = 0x40;
pub const ACTION_SELECT_7: c_uint = 0x80;
// Register Select bits
pub const REG_ALTERNATE_STATUS: c_uint = 0x01;
pub const REG_DEVICE_CONTROL: c_uint = 0x01;
pub const REG_ERROR: c_uint = 0x02;
pub const REG_FEATURES: c_uint = 0x02;
pub const REG_SECTOR_COUNT: c_uint = 0x04;
pub const REG_SECTOR_NUMBER: c_uint = 0x08;
pub const REG_CYLINDER_LOW: c_uint = 0x10;
pub const REG_CYLINDER_HIGH: c_uint = 0x20;
pub const REG_DEVICE_HEAD: c_uint = 0x40;
pub const REG_STATUS: c_uint = 0x80;
pub const REG_COMMAND: c_uint = 0x80;
// ATA registers offset definitions
pub const ATA_REG_ERROR_OFFSET: c_int = 1;
pub const ATA_REG_LCYL_OFFSET: c_int = 4;
pub const ATA_REG_HCYL_OFFSET: c_int = 5;
pub const ATA_REG_STATUS_OFFSET: c_int = 7;
// ATA error definitions not in <linux/hdreg.h>
pub const ATA_ERROR_MEDIA_CHANGE: c_uint = 0x20;
// ATA command definitions not in <linux/hdreg.h>
pub const ATA_COMMAND_GET_MEDIA_STATUS: c_uint = 0xDA;
pub const ATA_COMMAND_MEDIA_EJECT: c_uint = 0xED;
// ATA drive control definitions
pub const ATA_DC_DISABLE_INTERRUPTS: c_uint = 0x02;
pub const ATA_DC_RESET_CONTROLLER: c_uint = 0x04;
pub const ATA_DC_REENABLE_CONTROLLER: c_uint = 0x00;
//
// General purpose return codes
//

pub const ISD200_GOOD: c_int = 0;
//
// Transport return codes
//

// driver action codes
pub const ACTION_READ_STATUS: c_int = 0;
pub const ACTION_RESET: c_int = 1;
pub const ACTION_REENABLE: c_int = 2;
pub const ACTION_SOFT_RESET: c_int = 3;
pub const ACTION_ENUM: c_int = 4;
pub const ACTION_IDENTIFY: c_int = 5;
//
// ata_cdb struct
//
    union ata_cdb {
    struct {
    unsigned char SignatureByte0;
    unsigned char SignatureByte1;
    unsigned char ActionSelect;
    unsigned char RegisterSelect;
    unsigned char TransferBlockSize;
    unsigned char WriteData3F6;
    unsigned char WriteData1F1;
    unsigned char WriteData1F2;
    unsigned char WriteData1F3;
    unsigned char WriteData1F4;
    unsigned char WriteData1F5;
    unsigned char WriteData1F6;
    unsigned char WriteData1F7;
    unsigned char Reserved[3];
    } generic;
    struct {
    unsigned char SignatureByte0;
    unsigned char SignatureByte1;
    unsigned char ActionSelect;
    unsigned char RegisterSelect;
    unsigned char TransferBlockSize;
    unsigned char AlternateStatusByte;
    unsigned char ErrorByte;
    unsigned char SectorCountByte;
    unsigned char SectorNumberByte;
    unsigned char CylinderLowByte;
    unsigned char CylinderHighByte;
    unsigned char DeviceHeadByte;
    unsigned char StatusByte;
    unsigned char Reserved[3];
    } read;
    struct {
    unsigned char SignatureByte0;
    unsigned char SignatureByte1;
    unsigned char ActionSelect;
    unsigned char RegisterSelect;
    unsigned char TransferBlockSize;
    unsigned char DeviceControlByte;
    unsigned char FeaturesByte;
    unsigned char SectorCountByte;
    unsigned char SectorNumberByte;
    unsigned char CylinderLowByte;
    unsigned char CylinderHighByte;
    unsigned char DeviceHeadByte;
    unsigned char CommandByte;
    unsigned char Reserved[3];
    } write;
    };
//
// Inquiry data structure. This is the data returned from the target
// after it receives an inquiry.
//
// This structure may be extended by the number of bytes specified
// in the field AdditionalLength. The defined size constant only
// includes fields through ProductRevisionLevel.
//
// DeviceType field
//
pub const DIRECT_ACCESS_DEVICE: c_uint = 0x00    /* disks */;
pub const DEVICE_REMOVABLE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_data {
    pub DeviceType: c_uchar,
    pub DeviceTypeModifier: c_uchar,
    pub Versions: c_uchar,
    pub Format: c_uchar,
    pub AdditionalLength: c_uchar,
    pub Reserved: [c_uchar; 2],
    pub Capability: c_uchar,
    pub VendorId: [c_uchar; 8],
    pub ProductId: [c_uchar; 16],
    pub ProductRevisionLevel: [c_uchar; 4],
    pub VendorSpecific: [c_uchar; 20],
    pub Reserved3: [c_uchar; 40],
// C attribute field omitted
//
// INQUIRY data buffer size
//
pub const INQUIRYDATABUFFERSIZE: c_int = 36;
//
// ISD200 CONFIG data struct
//
pub const ATACFG_TIMING: c_uint = 0x0f;
pub const ATACFG_ATAPI_RESET: c_uint = 0x10;
pub const ATACFG_MASTER: c_uint = 0x20;
pub const ATACFG_BLOCKSIZE: c_uint = 0xa0;
pub const ATACFGE_LAST_LUN: c_uint = 0x07;
pub const ATACFGE_DESC_OVERRIDE: c_uint = 0x08;
pub const ATACFGE_STATE_SUSPEND: c_uint = 0x10;
pub const ATACFGE_SKIP_BOOT: c_uint = 0x20;
pub const ATACFGE_CONF_DESC2: c_uint = 0x40;
pub const ATACFGE_INIT_STATUS: c_uint = 0x80;
pub const CFG_CAPABILITY_SRST: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isd200_config {
    pub EventNotification: c_uchar,
    pub ExternalClock: c_uchar,
    pub ATAInitTimeout: c_uchar,
    pub ATAConfig: c_uchar,
    pub ATAMajorCommand: c_uchar,
    pub ATAMinorCommand: c_uchar,
    pub ATAExtraConfig: c_uchar,
    pub Capability: c_uchar,
// C attribute field omitted
//
// ISD200 driver information struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isd200_info {
    pub InquiryData: inquiry_data,
    pub id: *mut u16,
    pub ConfigData: isd200_config,
    pub RegsBuf: *mut c_uchar,
    pub ATARegs: [c_uchar; 8],
    pub DeviceHead: c_uchar,
    pub DeviceFlags: c_uchar,
// maximum number of LUNs supported
    pub MaxLUNs: c_uchar,
    pub cmnd: [c_uchar; MAX_COMMAND_SIZE],
    pub srb: scsi_cmnd,
    pub sg: scatterlist,
}

//
// Read Capacity Data - returned in Big Endian format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_capacity_data {
    pub LogicalBlockAddress: __be32,
    pub BytesPerBlock: __be32,
}

//
// Read Block Limits Data - returned in Big Endian format
// This structure returns the maximum and minimum block
// size for a TAPE device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_block_limits {
    pub Reserved: c_uchar,
    pub BlockMaximumSize: [c_uchar; 3],
    pub BlockMinimumSize: [c_uchar; 2],
}

//
// Sense Data Format
//
pub const SENSE_ERRCODE: c_uint = 0x7f;
pub const SENSE_ERRCODE_VALID: c_uint = 0x80;
pub const SENSE_FLAG_SENSE_KEY: c_uint = 0x0f;
pub const SENSE_FLAG_BAD_LENGTH: c_uint = 0x20;
pub const SENSE_FLAG_END_OF_MEDIA: c_uint = 0x40;
pub const SENSE_FLAG_FILE_MARK: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sense_data {
    pub ErrorCode: c_uchar,
    pub SegmentNumber: c_uchar,
    pub Flags: c_uchar,
    pub Information: [c_uchar; 4],
    pub AdditionalSenseLength: c_uchar,
    pub CommandSpecificInformation: [c_uchar; 4],
    pub AdditionalSenseCode: c_uchar,
    pub AdditionalSenseCodeQualifier: c_uchar,
    pub FieldReplaceableUnitCode: c_uchar,
    pub SenseKeySpecific: [c_uchar; 3],
// C attribute field omitted
//
// Default request sense buffer size
//
pub const SENSE_BUFFER_SIZE: c_int = 18;
//
// Helper routines
//
// isd200_build_sense
//
// Builds an artificial sense buffer to report the results of a
// failed command.
//
// RETURNS:
// void
//
#[no_mangle]
unsafe extern "C" fn isd200_build_sense(us: *mut us_data, srb: *mut scsi_cmnd) {
    static void isd200_build_sense(struct us_data *us, struct scsi_cmnd *srb)
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub &srb->sense_buffer[0]: *mut *mut *mut sense_data buf = (sense_data ),
    pub info->ATARegs[ATA_REG_ERROR_OFFSET]: unsigned char error =,
    if(error & ATA_ERROR_MEDIA_CHANGE) {
    pub SENSE_ERRCODE_VALID: buf->ErrorCode = 0x70 |,
    pub 0xb: buf->AdditionalSenseLength =,
    pub UNIT_ATTENTION: buf->Flags =,
    pub 0: buf->AdditionalSenseCode =,
    pub 0: buf->AdditionalSenseCodeQualifier =,
    } else if (error & ATA_MCR) {
    pub SENSE_ERRCODE_VALID: buf->ErrorCode = 0x70 |,
    pub 0xb: buf->AdditionalSenseLength =,
    pub UNIT_ATTENTION: buf->Flags =,
    pub 0: buf->AdditionalSenseCode =,
    pub 0: buf->AdditionalSenseCodeQualifier =,
    } else if (error & ATA_TRK0NF) {
    pub SENSE_ERRCODE_VALID: buf->ErrorCode = 0x70 |,
    pub 0xb: buf->AdditionalSenseLength =,
    pub NOT_READY: buf->Flags =,
    pub 0: buf->AdditionalSenseCode =,
    pub 0: buf->AdditionalSenseCodeQualifier =,
    } else if (error & ATA_UNC) {
    pub SENSE_ERRCODE_VALID: buf->ErrorCode = 0x70 |,
    pub 0xb: buf->AdditionalSenseLength =,
    pub DATA_PROTECT: buf->Flags =,
    pub 0: buf->AdditionalSenseCode =,
    pub 0: buf->AdditionalSenseCodeQualifier =,
    } else {
    pub 0: buf->ErrorCode =,
    pub 0: buf->AdditionalSenseLength =,
    pub 0: buf->Flags =,
    pub 0: buf->AdditionalSenseCode =,
    pub 0: buf->AdditionalSenseCodeQualifier =,
    }
    }
//
// Transport routines
//
// isd200_set_srb(), isd200_srb_set_bufflen()
//
// Two helpers to facilitate in initialization of scsi_cmnd structure
// Will need to change when struct scsi_cmnd changes
//
    static void isd200_set_srb(struct isd200_info *info,
    enum dma_data_direction dir, void* buff, unsigned bufflen)
    {
    pub &info->srb: *mut *mut scsi_cmnd srb =,
    if (buff)
    pub bufflen): sg_init_one(&info->sg, buff,,
    pub dir: srb->sc_data_direction =,
    pub NULL: srb->sdb.table.sgl = buff ? &info->sg :,
    pub bufflen: srb->sdb.length =,
    pub 0: srb->sdb.table.nents = buff ? 1 :,
    }
#[no_mangle]
unsafe extern "C" fn isd200_srb_set_bufflen(srb: *mut scsi_cmnd, bufflen: unsigned) {
    static void isd200_srb_set_bufflen(struct scsi_cmnd *srb, unsigned bufflen)
    {
    pub bufflen: srb->sdb.length =,
    }
//
// isd200_action
//
// Routine for sending commands to the isd200
//
// RETURNS:
// ISD status code
//
    static int isd200_action( struct us_data *us, int action,
    void* pointer, int value )
    {
    pub ata: union ata_cdb,
// static to prevent this large struct being placed on the valuable stack
    pub srb_dev: static struct scsi_device,
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub &info->srb: *mut *mut scsi_cmnd srb =,
    pub status: c_int,
    pub sizeof(ata)): memset(&ata, 0,,
    pub MAX_COMMAND_SIZE): memcpy(srb->cmnd, info->cmnd,,
    pub &srb_dev: srb->device =,
    pub info->ConfigData.ATAMajorCommand: ata.generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ata.generic.SignatureByte1 =,
    pub 1: ata.generic.TransferBlockSize =,
    switch ( action ) {
    case ACTION_READ_STATUS:
    pub isd200_action(READ_STATUS)\n"): usb_stor_dbg(us, ",
    pub ACTION_SELECT_0|ACTION_SELECT_2: ata.generic.ActionSelect =,
    ata.generic.RegisterSelect =
    REG_CYLINDER_LOW | REG_CYLINDER_HIGH |
    pub REG_ERROR: REG_STATUS |,
    pub value): isd200_set_srb(info, DMA_FROM_DEVICE, pointer,,
    case ACTION_ENUM:
    pub value): usb_stor_dbg(us, " isd200_action(ENUM,0x%02x)\n",,
    ata.generic.ActionSelect = ACTION_SELECT_1|ACTION_SELECT_2|
    ACTION_SELECT_3|ACTION_SELECT_4|
    pub REG_DEVICE_HEAD: ata.generic.RegisterSelect =,
    pub value: ata.write.DeviceHeadByte =,
    pub 0): isd200_set_srb(info, DMA_NONE, NULL,,
    case ACTION_RESET:
    pub isd200_action(RESET)\n"): usb_stor_dbg(us, ",
    ata.generic.ActionSelect = ACTION_SELECT_1|ACTION_SELECT_2|
    pub REG_DEVICE_CONTROL: ata.generic.RegisterSelect =,
    pub ATA_DC_RESET_CONTROLLER: ata.write.DeviceControlByte =,
    pub 0): isd200_set_srb(info, DMA_NONE, NULL,,
    case ACTION_REENABLE:
    pub isd200_action(REENABLE)\n"): usb_stor_dbg(us, ",
    ata.generic.ActionSelect = ACTION_SELECT_1|ACTION_SELECT_2|
    pub REG_DEVICE_CONTROL: ata.generic.RegisterSelect =,
    pub ATA_DC_REENABLE_CONTROLLER: ata.write.DeviceControlByte =,
    pub 0): isd200_set_srb(info, DMA_NONE, NULL,,
    case ACTION_SOFT_RESET:
    pub isd200_action(SOFT_RESET)\n"): usb_stor_dbg(us, ",
    pub ACTION_SELECT_1|ACTION_SELECT_5: ata.generic.ActionSelect =,
    pub REG_COMMAND: ata.generic.RegisterSelect = REG_DEVICE_HEAD |,
    pub info->DeviceHead: ata.write.DeviceHeadByte =,
    pub ATA_CMD_DEV_RESET: ata.write.CommandByte =,
    pub 0): isd200_set_srb(info, DMA_NONE, NULL,,
    case ACTION_IDENTIFY:
    pub isd200_action(IDENTIFY)\n"): usb_stor_dbg(us, ",
    pub REG_COMMAND: ata.generic.RegisterSelect =,
    pub ATA_CMD_ID_ATA: ata.write.CommandByte =,
    isd200_set_srb(info, DMA_FROM_DEVICE, info.id,
    pub 2): *mut *mut ATA_ID_WORDS,
    default:
    pub action): usb_stor_dbg(us, "Error: Undefined action %d\n",,
    pub ISD200_ERROR: return,
    }
    pub sizeof(ata.generic)): memcpy(srb->cmnd, &ata,,
    pub sizeof(ata.generic): srb->cmd_len =,
    pub us): status = usb_stor_Bulk_transport(srb,,
    if (status == USB_STOR_TRANSPORT_GOOD)
    pub ISD200_GOOD: status =,
    else {
    usb_stor_dbg(us, "   isd200_action(0x%02x) error: %d\n",
    pub status): action,,
    pub ISD200_ERROR: status =,
// need to reset device here
    }
    pub status: return,
    }
//
// isd200_read_regs
//
// Read ATA Registers
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_read_regs(us: *mut us_data) -> c_int {
    static int isd200_read_regs( struct us_data *us )
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub ISD200_GOOD: int retStatus =,
    pub transferStatus: c_int,
    pub isd200_IssueATAReadRegs\n"): usb_stor_dbg(us, "Entering,
    transferStatus = isd200_action( us, ACTION_READ_STATUS,
    pub ): info->RegsBuf, sizeof(info->ATARegs),
    if (transferStatus != ISD200_TRANSPORT_GOOD) {
    pub registers\n"): usb_stor_dbg(us, " Error reading ATA,
    pub ISD200_ERROR: retStatus =,
    } else {
    pub sizeof(info->ATARegs)): memcpy(info->ATARegs, info->RegsBuf,,
    usb_stor_dbg(us, "   Got ATA Register[ATA_REG_ERROR_OFFSET] = 0x%x\n",
    }
    pub retStatus: return,
    }
//
// Invoke the transport and basic error-handling/recovery methods
//
// This is used by the protocol layers to actually send the message to
// the device and receive the response.
//
    static void isd200_invoke_transport( struct us_data *us,
    struct scsi_cmnd *srb,
    union ata_cdb *ataCdb )
    {
    pub 0: int need_auto_sense =,
    pub transferStatus: c_int,
    pub result: c_int,
// send the command to the transport layer
    pub sizeof(ataCdb->generic)): memcpy(srb->cmnd, ataCdb,,
    pub sizeof(ataCdb->generic): srb->cmd_len =,
    pub us): transferStatus = usb_stor_Bulk_transport(srb,,
//
// if the command gets aborted by the higher layers, we need to
// short-circuit all other processing
//
    if (test_bit(US_FLIDX_TIMED_OUT, &us.dflags)) {
    pub aborted\n"): usb_stor_dbg(us, "-- command was,
    pub Handle_Abort: goto,
    }
    switch (transferStatus) {
    case USB_STOR_TRANSPORT_GOOD:
// Indicate a good result
    pub SAM_STAT_GOOD: srb->result =,
    case USB_STOR_TRANSPORT_NO_SENSE:
    pub failure\n"): usb_stor_dbg(us, "-- transport indicates protocol,
    pub SAM_STAT_CHECK_CONDITION: srb->result =,
    case USB_STOR_TRANSPORT_FAILED:
    pub failure\n"): usb_stor_dbg(us, "-- transport indicates command,
    pub 1: need_auto_sense =,
    case USB_STOR_TRANSPORT_ERROR:
    pub error\n"): usb_stor_dbg(us, "-- transport indicates transport,
    pub 16: srb->result = DID_ERROR <<,
// Need reset here
    default:
    pub error\n"): usb_stor_dbg(us, "-- transport indicates unknown,
    pub 16: srb->result = DID_ERROR <<,
// Need reset here
    }
    if ((scsi_get_resid(srb) > 0) &&
    !((srb.cmnd[0] == REQUEST_SENSE) ||
    (srb.cmnd[0] == INQUIRY) ||
    (srb.cmnd[0] == MODE_SENSE) ||
    (srb.cmnd[0] == LOG_SENSE) ||
    (srb.cmnd[0] == MODE_SENSE_10))) {
    pub transfer\n"): usb_stor_dbg(us, "-- unexpectedly short,
    pub 1: need_auto_sense =,
    }
    if (need_auto_sense) {
    pub isd200_read_regs(us): result =,
    if (test_bit(US_FLIDX_TIMED_OUT, &us.dflags)) {
    pub aborted\n"): usb_stor_dbg(us, "-- auto-sense,
    pub Handle_Abort: goto,
    }
    if (result == ISD200_GOOD) {
    pub srb): isd200_build_sense(us,,
    pub SAM_STAT_CHECK_CONDITION: srb->result =,
// If things are really okay, then let's show that
    if ((srb.sense_buffer[2] & 0xf) == 0x0)
    pub SAM_STAT_GOOD: srb->result =,
    } else {
    pub 16: srb->result = DID_ERROR <<,
// Need reset here
    }
    }
//
// Regardless of auto-sense, if we _know_ we have an error
// condition, show that in the result code
//
    if (transferStatus == USB_STOR_TRANSPORT_FAILED)
    pub SAM_STAT_CHECK_CONDITION: srb->result =,
//
// abort processing: the bulk-only transport requires a reset
// following an abort
//
    Handle_Abort:
    pub 16: srb->result = DID_ABORT <<,
// permit the reset transfer to take place
    pub &us->dflags): clear_bit(US_FLIDX_ABORTING,,
// Need reset here
    }

#[no_mangle]
unsafe extern "C" fn isd200_log_config(us: *mut us_data, info: *mut isd200_info) {
    static void isd200_log_config(struct us_data *us, struct isd200_info *info)
    {
    usb_stor_dbg(us, "      Event Notification: 0x%x\n",
    usb_stor_dbg(us, "      External Clock: 0x%x\n",
    usb_stor_dbg(us, "      ATA Init Timeout: 0x%x\n",
    usb_stor_dbg(us, "      ATAPI Command Block Size: 0x%x\n",
    pub 6): (info->ConfigData.ATAConfig & ATACFG_BLOCKSIZE) >>,
    usb_stor_dbg(us, "      Master/Slave Selection: 0x%x\n",
    pub ATACFG_MASTER): info->ConfigData.ATAConfig &,
    usb_stor_dbg(us, "      ATAPI Reset: 0x%x\n",
    pub ATACFG_ATAPI_RESET): info->ConfigData.ATAConfig &,
    usb_stor_dbg(us, "      ATA Timing: 0x%x\n",
    pub ATACFG_TIMING): info->ConfigData.ATAConfig &,
    usb_stor_dbg(us, "      ATA Major Command: 0x%x\n",
    usb_stor_dbg(us, "      ATA Minor Command: 0x%x\n",
    usb_stor_dbg(us, "      Init Status: 0x%x\n",
    pub ATACFGE_INIT_STATUS): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      Config Descriptor 2: 0x%x\n",
    pub ATACFGE_CONF_DESC2): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      Skip Device Boot: 0x%x\n",
    pub ATACFGE_SKIP_BOOT): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      ATA 3 State Suspend: 0x%x\n",
    pub ATACFGE_STATE_SUSPEND): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      Descriptor Override: 0x%x\n",
    pub ATACFGE_DESC_OVERRIDE): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      Last LUN Identifier: 0x%x\n",
    pub ATACFGE_LAST_LUN): info->ConfigData.ATAExtraConfig &,
    usb_stor_dbg(us, "      SRST Enable: 0x%x\n",
    pub CFG_CAPABILITY_SRST): info->ConfigData.ATAExtraConfig &,
    }

//
// isd200_write_config
//
// Write the ISD200 Configuration data
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_write_config(us: *mut us_data) -> c_int {
    static int isd200_write_config( struct us_data *us )
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub ISD200_GOOD: int retStatus =,
    pub result: c_int,

    pub isd200_write_config\n"): usb_stor_dbg(us, "Entering,
    pub Data:\n"): usb_stor_dbg(us, " Writing the following ISD200 Config,
    pub info): isd200_log_config(us,,

// let's send the command via the control pipe
    result = usb_stor_ctrl_transfer(
    us,
    us.send_ctrl_pipe,
    0x01,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    0x0000,
    0x0002,
    (void *) &info.ConfigData,
    if (result >= 0) {
    pub successfully\n"): usb_stor_dbg(us, " ISD200 Config Data was written,
    } else {
    pub failed!\n"): usb_stor_dbg(us, " Request to write ISD200 Config Data,
    pub ISD200_ERROR: retStatus =,
    }
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_write_config %08X\n",,
    pub retStatus: return,
    }
//
// isd200_read_config
//
// Reads the ISD200 Configuration data
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_read_config(us: *mut us_data) -> c_int {
    static int isd200_read_config( struct us_data *us )
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub ISD200_GOOD: int retStatus =,
    pub result: c_int,
    pub isd200_read_config\n"): usb_stor_dbg(us, "Entering,
// read the configuration information from ISD200.  Use this to
// determine what the special ATA CDB bytes are.
    result = usb_stor_ctrl_transfer(
    us,
    us.recv_ctrl_pipe,
    0x02,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x0000,
    0x0002,
    (void *) &info.ConfigData,
    if (result >= 0) {
    pub Data:\n"): usb_stor_dbg(us, " Retrieved the following ISD200 Config,

    pub info): isd200_log_config(us,,

    } else {
    pub failed!\n"): usb_stor_dbg(us, " Request to get ISD200 Config Data,
    pub ISD200_ERROR: retStatus =,
    }
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_read_config %08X\n",,
    pub retStatus: return,
    }
//
// isd200_atapi_soft_reset
//
// Perform an Atapi Soft Reset on the device
//
// RETURNS:
// NT status code
//
#[no_mangle]
unsafe extern "C" fn isd200_atapi_soft_reset(us: *mut us_data) -> c_int {
    static int isd200_atapi_soft_reset( struct us_data *us )
    {
    pub ISD200_GOOD: int retStatus =,
    pub transferStatus: c_int,
    pub isd200_atapi_soft_reset\n"): usb_stor_dbg(us, "Entering,
    pub ): transferStatus = isd200_action( us, ACTION_SOFT_RESET, NULL, 0,
    if (transferStatus != ISD200_TRANSPORT_GOOD) {
    pub Reset\n"): usb_stor_dbg(us, " Error issuing Atapi Soft,
    pub ISD200_ERROR: retStatus =,
    }
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_atapi_soft_reset %08X\n",,
    pub retStatus: return,
    }
//
// isd200_srst
//
// Perform an SRST on the device
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_srst(us: *mut us_data) -> c_int {
    static int isd200_srst( struct us_data *us )
    {
    pub ISD200_GOOD: int retStatus =,
    pub transferStatus: c_int,
    pub isd200_SRST\n"): usb_stor_dbg(us, "Entering,
    pub ): transferStatus = isd200_action( us, ACTION_RESET, NULL, 0,
// check to see if this request failed
    if (transferStatus != ISD200_TRANSPORT_GOOD) {
    pub SRST\n"): usb_stor_dbg(us, " Error issuing,
    pub ISD200_ERROR: retStatus =,
    } else {
// delay 10ms to give the drive a chance to see it
    pub ): transferStatus = isd200_action( us, ACTION_REENABLE, NULL, 0,
    if (transferStatus != ISD200_TRANSPORT_GOOD) {
    pub reset\n"): usb_stor_dbg(us, " Error taking drive out of,
    pub ISD200_ERROR: retStatus =,
    } else {
// delay 50ms to give the drive a chance to recover after SRST
    }
    }
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_srst %08X\n",,
    pub retStatus: return,
    }
//
// isd200_try_enum
//
// Helper function for isd200_manual_enum(). Does ENUM and READ_STATUS
// and tries to analyze the status registers
//
// RETURNS:
// ISD status code
//
    static int isd200_try_enum(struct us_data *us, unsigned char master_slave,
    int detect )
    {
    pub ISD200_GOOD: int status =,
    pub endTime: c_ulong,
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub info->RegsBuf: *mut *mut unsigned char regs =,
    pub 0: int recheckAsMaster =,
    if ( detect )
    pub HZ: *mut *mut endTime = jiffies + ISD200_ENUM_DETECT_TIMEOUT,
    else
    pub HZ: *mut *mut endTime = jiffies + ISD200_ENUM_BSY_TIMEOUT,
// loop until we detect !BSY or timeout
    while(1) {
    pub ): status = isd200_action( us, ACTION_ENUM, NULL, master_slave,
    if ( status != ISD200_GOOD )
    status = isd200_action( us, ACTION_READ_STATUS,
    pub ): regs, 8,
    if ( status != ISD200_GOOD )
    if (!detect) {
    if (regs[ATA_REG_STATUS_OFFSET] & ATA_BUSY) {
    usb_stor_dbg(us, "   %s status is still BSY, try again...\n",
    master_slave == ATA_ADDRESS_DEVHEAD_STD ?
    pub "Slave"): "Master" :,
    } else {
    usb_stor_dbg(us, "   %s status !BSY, continue with next operation\n",
    master_slave == ATA_ADDRESS_DEVHEAD_STD ?
    pub "Slave"): "Master" :,
    }
    }
// check for ATA_BUSY and
// ATA_DF (workaround ATA Zip drive) and
// ATA_ERR (workaround for Archos CD-ROM)
    else if (regs[ATA_REG_STATUS_OFFSET] &
    (ATA_BUSY | ATA_DF | ATA_ERR)) {
    pub again...\n"): usb_stor_dbg(us, " Status indicates it is not ready, try,
    }
// check for DRDY, ATA devices set DRDY after SRST
#[no_mangle]
pub unsafe extern "C" fn if(ATA_DRDY: regs[ATA_REG_STATUS_OFFSET] &) -> else {
    pub device\n"): usb_stor_dbg(us, " Identified ATA,
    pub DF_ATA_DEVICE: info->DeviceFlags |=,
    pub master_slave: info->DeviceHead =,
    }
//
// check Cylinder High/Low to
// determine if it is an ATAPI device
//
    else if (regs[ATA_REG_HCYL_OFFSET] == 0xEB &&
    regs[ATA_REG_LCYL_OFFSET] == 0x14) {
//
// It seems that the RICOH
// MP6200A CD/RW drive will
// report itself okay as a
// slave when it is really a
// master. So this check again
// as a master device just to
// make sure it doesn't report
// itself okay as a master also
//
    if ((master_slave & ATA_ADDRESS_DEVHEAD_SLAVE) &&
    !recheckAsMaster) {
    pub master\n"): usb_stor_dbg(us, " Identified ATAPI device as slave. Rechecking again as,
    pub 1: recheckAsMaster =,
    pub ATA_ADDRESS_DEVHEAD_STD: master_slave =,
    } else {
    pub device\n"): usb_stor_dbg(us, " Identified ATAPI,
    pub master_slave: info->DeviceHead =,
    pub isd200_atapi_soft_reset(us): status =,
    }
    } else {
    pub Weird\n"): usb_stor_dbg(us, " Not ATA, not ATAPI -,
    }
// check for timeout on this request
    if (time_after_eq(jiffies, endTime)) {
    if (!detect)
    pub operation...\n"): usb_stor_dbg(us, " BSY check timeout, just continue with next,
    else
    pub timeout!\n"): usb_stor_dbg(us, " Device detect,
    }
    }
    pub status: return,
    }
//
// isd200_manual_enum
//
// Determines if the drive attached is an ATA or ATAPI and if it is a
// master or slave.
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_manual_enum(us: *mut us_data) -> c_int {
    static int isd200_manual_enum(struct us_data *us)
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub ISD200_GOOD: int retStatus =,
    pub isd200_manual_enum\n"): usb_stor_dbg(us, "Entering,
    pub isd200_read_config(us): retStatus =,
    if (retStatus == ISD200_GOOD) {
    pub isslave: c_int,
// master or slave?
    pub 0): retStatus = isd200_try_enum( us, ATA_ADDRESS_DEVHEAD_STD,,
    if (retStatus == ISD200_GOOD)
    pub 0): retStatus = isd200_try_enum( us, ATA_ADDRESS_DEVHEAD_SLAVE,,
    if (retStatus == ISD200_GOOD) {
    pub isd200_srst(us): retStatus =,
    if (retStatus == ISD200_GOOD)
// ata or atapi?
    pub 1): retStatus = isd200_try_enum( us, ATA_ADDRESS_DEVHEAD_STD,,
    }
    pub 0: isslave = (info->DeviceHead & ATA_ADDRESS_DEVHEAD_SLAVE) ? 1 :,
    if (!(info.ConfigData.ATAConfig & ATACFG_MASTER)) {
    usb_stor_dbg(us, "   Setting Master/Slave selection to %d\n",
    pub 0x3f: info->ConfigData.ATAConfig &=,
    pub (isslave<<6): info->ConfigData.ATAConfig |=,
    pub isd200_write_config(us): retStatus =,
    }
    }
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_manual_enum %08X\n",,
    }
#[no_mangle]
unsafe extern "C" fn isd200_fix_driveid(id: *mut u16) {
    static void isd200_fix_driveid(u16 *id)
    {

    pub i: c_int,
    pub i++): for (i = 0; i < ATA_ID_WORDS;,
    pub __le16_to_cpu(id[i]): id[i] =,

    }
#[no_mangle]
unsafe extern "C" fn isd200_dump_driveid(us: *mut us_data, id: *mut u16) {
    static void isd200_dump_driveid(struct us_data *us, u16 *id)
    {
    pub Structure:\n"): usb_stor_dbg(us, " Identify Data,
    pub id[ATA_ID_CONFIG]): usb_stor_dbg(us, " config = 0x%x\n",,
    pub id[ATA_ID_CYLS]): usb_stor_dbg(us, " cyls = 0x%x\n",,
    pub id[ATA_ID_HEADS]): usb_stor_dbg(us, " heads = 0x%x\n",,
    pub id[4]): usb_stor_dbg(us, " track_bytes = 0x%x\n",,
    pub id[5]): usb_stor_dbg(us, " sector_bytes = 0x%x\n",,
    pub id[ATA_ID_SECTORS]): usb_stor_dbg(us, " sectors = 0x%x\n",,
    pub )&id[ATA_ID_SERNO]): *mut *mut usb_stor_dbg(us, " serial_no[0] = 0x%x\n", (char,
    pub id[20]): usb_stor_dbg(us, " buf_type = 0x%x\n",,
    pub id[ATA_ID_BUF_SIZE]): usb_stor_dbg(us, " buf_size = 0x%x\n",,
    pub id[22]): usb_stor_dbg(us, " ecc_bytes = 0x%x\n",,
    pub )&id[ATA_ID_FW_REV]): *mut *mut usb_stor_dbg(us, " fw_rev[0] = 0x%x\n", (char,
    pub )&id[ATA_ID_PROD]): *mut *mut usb_stor_dbg(us, " model[0] = 0x%x\n", (char,
    pub 0xff): usb_stor_dbg(us, " max_multsect = 0x%x\n", id[ATA_ID_MAX_MULTSECT] &,
    pub id[ATA_ID_DWORD_IO]): usb_stor_dbg(us, " dword_io = 0x%x\n",,
    pub 8): usb_stor_dbg(us, " capability = 0x%x\n", id[ATA_ID_CAPABILITY] >>,
    pub 8): usb_stor_dbg(us, " tPIO = 0x%x\n", id[ATA_ID_OLD_PIO_MODES] >>,
    pub 8): usb_stor_dbg(us, " tDMA = 0x%x\n", id[ATA_ID_OLD_DMA_MODES] >>,
    pub id[ATA_ID_FIELD_VALID]): usb_stor_dbg(us, " field_valid = 0x%x\n",,
    pub id[ATA_ID_CUR_CYLS]): usb_stor_dbg(us, " cur_cyls = 0x%x\n",,
    pub id[ATA_ID_CUR_HEADS]): usb_stor_dbg(us, " cur_heads = 0x%x\n",,
    pub id[ATA_ID_CUR_SECTORS]): usb_stor_dbg(us, " cur_sectors = 0x%x\n",,
    pub 57)): usb_stor_dbg(us, " cur_capacity = 0x%x\n", ata_id_u32(id,,
    pub 0xff): usb_stor_dbg(us, " multsect = 0x%x\n", id[ATA_ID_MULTSECT] &,
    pub ATA_ID_LBA_CAPACITY)): usb_stor_dbg(us, " lba_capacity = 0x%x\n", ata_id_u32(id,,
    pub id[ATA_ID_COMMAND_SET_1]): usb_stor_dbg(us, " command_set_1 = 0x%x\n",,
    pub id[ATA_ID_COMMAND_SET_2]): usb_stor_dbg(us, " command_set_2 = 0x%x\n",,
    }
//
// isd200_get_inquiry_data
//
// Get inquiry data
//
// RETURNS:
// ISD status code
//
#[no_mangle]
unsafe extern "C" fn isd200_get_inquiry_data(us: *mut us_data) -> c_int {
    static int isd200_get_inquiry_data( struct us_data *us )
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub retStatus: c_int,
    pub info->id: *mut *mut u16 id =,
    pub isd200_get_inquiry_data\n"): usb_stor_dbg(us, "Entering,
// set default to Master
    pub ATA_ADDRESS_DEVHEAD_STD: info->DeviceHead =,
// attempt to manually enumerate this device
    pub isd200_manual_enum(us): retStatus =,
    if (retStatus == ISD200_GOOD) {
    pub transferStatus: c_int,
// check for an ATA device
    if (info.DeviceFlags & DF_ATA_DEVICE) {
// this must be an ATA device
// perform an ATA Command Identify
    transferStatus = isd200_action( us, ACTION_IDENTIFY,
    pub 2): *mut *mut id, ATA_ID_WORDS,
    if (transferStatus != ISD200_TRANSPORT_GOOD) {
// Error issuing ATA Command Identify
    pub Identify\n"): usb_stor_dbg(us, " Error issuing ATA Command,
    pub ISD200_ERROR: retStatus =,
    } else {
// ATA Command Identify successful
    pub i: c_int,
    pub src: *mut __be16,
    pub dest: *mut __u16,
    pub id): isd200_dump_driveid(us,,
// Prevent division by 0 in isd200_scsi_to_ata()
    if (id[ATA_ID_HEADS] == 0 || id[ATA_ID_SECTORS] == 0) {
    pub data\n"): usb_stor_dbg(us, " Invalid ATA Identify,
    pub ISD200_ERROR: retStatus =,
    pub Done: goto,
    }
    pub sizeof(info->InquiryData)): memset(&info->InquiryData, 0,,
// Standard IDE interface only supports disks
    pub DIRECT_ACCESS_DEVICE: info->InquiryData.DeviceType =,
// The length must be at least 36 (5 + 31)
    pub 0x1F: info->InquiryData.AdditionalLength =,
    if (id[ATA_ID_COMMAND_SET_1] & COMMANDSET_MEDIA_STATUS) {
// set the removable bit
    pub DEVICE_REMOVABLE: info->InquiryData.DeviceTypeModifier =,
    pub DF_REMOVABLE_MEDIA: info->DeviceFlags |=,
    }
// Fill in vendor identification fields
    pub )&id[ATA_ID_PROD]: *mut src = (__be16,
    pub (__u16*)info->InquiryData.VendorId: *mut dest =,
    pub i++): for (i = 0; i < 4;,
    pub be16_to_cpu(src[i]): dest[i] =,
    pub 8/2]: *mut *mut src = (__be16 )&id[ATA_ID_PROD +,
    pub (__u16*)info->InquiryData.ProductId: *mut dest =,
    pub (i=0;i<8;i++): for,
    pub be16_to_cpu(src[i]): dest[i] =,
    pub )&id[ATA_ID_FW_REV]: *mut src = (__be16,
    pub (__u16*)info->InquiryData.ProductRevisionLevel: *mut dest =,
    pub (i=0;i<2;i++): for,
    pub be16_to_cpu(src[i]): dest[i] =,
// determine if it supports Media Status Notification
    if (id[ATA_ID_COMMAND_SET_2] & COMMANDSET_MEDIA_STATUS) {
    pub Notification\n"): usb_stor_dbg(us, " Device supports Media Status,
//
// Indicate that it is enabled, even
// though it is not.
// This allows the lock/unlock of the
// media to work correctly.
//
    pub DF_MEDIA_STATUS_ENABLED: info->DeviceFlags |=,
    }
    else
    pub ~DF_MEDIA_STATUS_ENABLED: info->DeviceFlags &=,
    }
    } else {
//
// this must be an ATAPI device
// use an ATAPI protocol (Transparent SCSI)
//
    pub SCSI": us->protocol_name = "Transparent,
    pub usb_stor_transparent_scsi_command: us->proto_handler =,
    usb_stor_dbg(us, "Protocol changed to: %s\n",
// Free driver structure
    pub NULL: us->extra =,
    pub NULL: us->extra_destructor =,
    }
    }
    Done:
    pub retStatus): usb_stor_dbg(us, "Leaving isd200_get_inquiry_data %08X\n",,
    }
//
// isd200_scsi_to_ata
//
// Translate SCSI commands to ATA commands.
//
// RETURNS:
// 1 if the command needs to be sent to the transport layer
// 0 otherwise
//
    static int isd200_scsi_to_ata(struct scsi_cmnd *srb, struct us_data *us,
    union ata_cdb * ataCdb)
    {
    pub )us->extra: *mut *mut isd200_info info = (isd200_info,
    pub info->id: *mut *mut u16 id =,
    pub 1: int sendToTransport =,
    pub head: unsigned char sectnum,,
    pub cylinder: c_ushort,
    pub lba: c_ulong,
    pub blockCount: c_ulong,
    pub }: unsigned char senseData[8] = { 0, 0, 0, 0, 0, 0, 0, 0,
    pub ata_cdb)): memset(ataCdb, 0, sizeof(union,
// SCSI Command
    switch (srb.cmnd[0]) {
    case INQUIRY:
    pub INQUIRY\n"): usb_stor_dbg(us, " ATA OUT -,
// copy InquiryData
    usb_stor_set_xfer_buf((unsigned char *) &info.InquiryData,
    pub srb): sizeof(info->InquiryData),,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    case MODE_SENSE:
    pub SCSIOP_MODE_SENSE\n"): usb_stor_dbg(us, " ATA OUT -,
// Initialize the return buffer
    pub srb): usb_stor_set_xfer_buf(senseData, sizeof(senseData),,
    if (info.DeviceFlags & DF_MEDIA_STATUS_ENABLED)
    {
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    pub REG_COMMAND: ataCdb->generic.RegisterSelect =,
    pub ATA_COMMAND_GET_MEDIA_STATUS: ataCdb->write.CommandByte =,
    pub 0): isd200_srb_set_bufflen(srb,,
    } else {
    pub okay\n"): usb_stor_dbg(us, " Media Status not supported, just report,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    }
    case TEST_UNIT_READY:
    pub SCSIOP_TEST_UNIT_READY\n"): usb_stor_dbg(us, " ATA OUT -,
    if (info.DeviceFlags & DF_MEDIA_STATUS_ENABLED)
    {
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    pub REG_COMMAND: ataCdb->generic.RegisterSelect =,
    pub ATA_COMMAND_GET_MEDIA_STATUS: ataCdb->write.CommandByte =,
    pub 0): isd200_srb_set_bufflen(srb,,
    } else {
    pub okay\n"): usb_stor_dbg(us, " Media Status not supported, just report,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    }
    case READ_CAPACITY:
    {
    pub capacity: c_ulong,
    pub readCapacityData: read_capacity_data,
    pub SCSIOP_READ_CAPACITY\n"): usb_stor_dbg(us, " ATA OUT -,
    if (ata_id_has_lba(id))
    pub 1: capacity = ata_id_u32(id, ATA_ID_LBA_CAPACITY) -,
    else
    capacity = (id[ATA_ID_HEADS] * id[ATA_ID_CYLS] *
    pub 1: id[ATA_ID_SECTORS]) -,
    pub cpu_to_be32(capacity): readCapacityData.LogicalBlockAddress =,
    pub cpu_to_be32(0x200): readCapacityData.BytesPerBlock =,
    usb_stor_set_xfer_buf((unsigned char *) &readCapacityData,
    pub srb): sizeof(readCapacityData),,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    }
    case READ_10:
    pub SCSIOP_READ\n"): usb_stor_dbg(us, " ATA OUT -,
    pub )&srb->cmnd[2]): *mut *mut lba = be32_to_cpu((__be32,
    pub long)srb->cmnd[8]: blockCount = (unsigned long)srb->cmnd[7]<<8 | (unsigned,
    if (ata_id_has_lba(id)) {
    pub char)(lba): sectnum = (unsigned,
    pub short)(lba>>8): cylinder = (unsigned,
    pub 0x0F): head = ATA_ADDRESS_DEVHEAD_LBA_MODE | (unsigned char)(lba>>24 &,
    } else {
    pub 1): sectnum = (u8)((lba % id[ATA_ID_SECTORS]) +,
    cylinder = (u16)(lba / (id[ATA_ID_SECTORS] *
    head = (u8)((lba / id[ATA_ID_SECTORS]) %
    }
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    ataCdb.generic.RegisterSelect =
    REG_SECTOR_COUNT | REG_SECTOR_NUMBER |
    REG_CYLINDER_LOW | REG_CYLINDER_HIGH |
    pub REG_COMMAND: REG_DEVICE_HEAD |,
    pub char)blockCount: ataCdb->write.SectorCountByte = (unsigned,
    pub sectnum: ataCdb->write.SectorNumberByte =,
    pub char)(cylinder>>8): ataCdb->write.CylinderHighByte = (unsigned,
    pub char)cylinder: ataCdb->write.CylinderLowByte = (unsigned,
    pub ATA_ADDRESS_DEVHEAD_STD): ataCdb->write.DeviceHeadByte = (head |,
    pub ATA_CMD_PIO_READ: ataCdb->write.CommandByte =,
    case WRITE_10:
    pub SCSIOP_WRITE\n"): usb_stor_dbg(us, " ATA OUT -,
    pub )&srb->cmnd[2]): *mut *mut lba = be32_to_cpu((__be32,
    pub long)srb->cmnd[8]: blockCount = (unsigned long)srb->cmnd[7]<<8 | (unsigned,
    if (ata_id_has_lba(id)) {
    pub char)(lba): sectnum = (unsigned,
    pub short)(lba>>8): cylinder = (unsigned,
    pub 0x0F): head = ATA_ADDRESS_DEVHEAD_LBA_MODE | (unsigned char)(lba>>24 &,
    } else {
    pub 1): sectnum = (u8)((lba % id[ATA_ID_SECTORS]) +,
    cylinder = (u16)(lba / (id[ATA_ID_SECTORS] *
    head = (u8)((lba / id[ATA_ID_SECTORS]) %
    }
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    ataCdb.generic.RegisterSelect =
    REG_SECTOR_COUNT | REG_SECTOR_NUMBER |
    REG_CYLINDER_LOW | REG_CYLINDER_HIGH |
    pub REG_COMMAND: REG_DEVICE_HEAD |,
    pub char)blockCount: ataCdb->write.SectorCountByte = (unsigned,
    pub sectnum: ataCdb->write.SectorNumberByte =,
    pub char)(cylinder>>8): ataCdb->write.CylinderHighByte = (unsigned,
    pub char)cylinder: ataCdb->write.CylinderLowByte = (unsigned,
    pub ATA_ADDRESS_DEVHEAD_STD): ataCdb->write.DeviceHeadByte = (head |,
    pub ATA_CMD_PIO_WRITE: ataCdb->write.CommandByte =,
    case ALLOW_MEDIUM_REMOVAL:
    pub SCSIOP_MEDIUM_REMOVAL\n"): usb_stor_dbg(us, " ATA OUT -,
    if (info.DeviceFlags & DF_REMOVABLE_MEDIA) {
    usb_stor_dbg(us, "   srb.cmnd[4] = 0x%X\n",
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    pub REG_COMMAND: ataCdb->generic.RegisterSelect =,
    ataCdb.write.CommandByte = (srb.cmnd[4] & 0x1) ?
    pub ATA_CMD_MEDIA_UNLOCK: ATA_CMD_MEDIA_LOCK :,
    pub 0): isd200_srb_set_bufflen(srb,,
    } else {
    pub okay\n"): usb_stor_dbg(us, " Not removable media, just report,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    }
    case START_STOP:
    pub SCSIOP_START_STOP_UNIT\n"): usb_stor_dbg(us, " ATA OUT -,
    pub srb->cmnd[4]): usb_stor_dbg(us, " srb->cmnd[4] = 0x%X\n",,
    if ((srb.cmnd[4] & 0x3) == 0x2) {
    pub Eject\n"): usb_stor_dbg(us, " Media,
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 0: ataCdb->generic.TransferBlockSize =,
    pub REG_COMMAND: ataCdb->generic.RegisterSelect =,
    pub ATA_COMMAND_MEDIA_EJECT: ataCdb->write.CommandByte =,
    } else if ((srb.cmnd[4] & 0x3) == 0x1) {
    pub Status\n"): usb_stor_dbg(us, " Get Media,
    pub info->ConfigData.ATAMajorCommand: ataCdb->generic.SignatureByte0 =,
    pub info->ConfigData.ATAMinorCommand: ataCdb->generic.SignatureByte1 =,
    pub 1: ataCdb->generic.TransferBlockSize =,
    pub REG_COMMAND: ataCdb->generic.RegisterSelect =,
    pub ATA_COMMAND_GET_MEDIA_STATUS: ataCdb->write.CommandByte =,
    pub 0): isd200_srb_set_bufflen(srb,,
    } else {
    pub okay\n"): usb_stor_dbg(us, " Nothing to do, just report,
    pub SAM_STAT_GOOD: srb->result =,
    pub 0: sendToTransport =,
    }
    default:
    usb_stor_dbg(us, "Unsupported SCSI command - 0x%X\n",
    pub 16: srb->result = DID_ERROR <<,
    pub 0: sendToTransport =,
    }
    }
//
// isd200_free_info
//
// Frees the driver structure.
//
#[no_mangle]
unsafe extern "C" fn isd200_free_info_ptrs(info_: *mut c_void) {
    static void isd200_free_info_ptrs(void *info_)
    {
    pub info_: *mut *mut *mut isd200_info info = (isd200_info ),
    if (info) {
    }
    }
//
// isd200_init_info
//
// Allocates (if necessary) and initializes the driver structure.
//
// RETURNS:
// error status code
//
#[no_mangle]
unsafe extern "C" fn isd200_init_info(us: *mut us_data) -> c_int {
    static int isd200_init_info(struct us_data *us)
    {
    pub info: *mut isd200_info,
    pub isd200_info): info = kzalloc_obj(struct,
    if (!info)
    pub -ENOMEM: return,
    pub GFP_KERNEL): *mut *mut info->id = kzalloc(ATA_ID_WORDS  2,,
    pub GFP_KERNEL): info->RegsBuf = kmalloc(sizeof(info->ATARegs),,
    pub GFP_KERNEL): info->srb.sense_buffer = kmalloc(SCSI_SENSE_BUFFERSIZE,,
    if (!info.id || !info.RegsBuf || !info.srb.sense_buffer) {
    pub -ENOMEM: return,
    }
    pub info: us->extra =,
    pub isd200_free_info_ptrs: us->extra_destructor =,
    pub 0: return,
    }
//
// Initialization for the ISD200
//
#[no_mangle]
unsafe extern "C" fn isd200_Initialization(us: *mut us_data) -> c_int {
    static int isd200_Initialization(struct us_data *us)
    {
    pub 0: int rc =,
    pub Initialization...\n"): usb_stor_dbg(us, "ISD200,
// Initialize ISD200 info struct
    if (isd200_init_info(us) < 0) {
    pub struct\n"): usb_stor_dbg(us, "ERROR Initializing ISD200 Info,
    pub -ENOMEM: rc =,
    } else {
// Get device specific data
    if (isd200_get_inquiry_data(us) != ISD200_GOOD) {
    pub Failure\n"): usb_stor_dbg(us, "ISD200 Initialization,
    pub -EINVAL: rc =,
    } else {
    pub complete\n"): usb_stor_dbg(us, "ISD200 Initialization,
    }
    }
    pub rc: return,
    }
//
// Protocol and Transport for the ISD200 ASIC
//
// This protocol and transport are for ATA devices connected to an ISD200
// ASIC.  An ATAPI device that is connected as a slave device will be
// detected in the driver initialization function and the protocol will
// be changed to an ATAPI protocol (Transparent SCSI).
//
#[no_mangle]
unsafe extern "C" fn isd200_ata_command(srb: *mut scsi_cmnd, us: *mut us_data) {
    static void isd200_ata_command(struct scsi_cmnd *srb, struct us_data *us)
    {
    pub orig_bufflen: int sendToTransport,,
    pub ataCdb: union ata_cdb,
// Make sure driver was initialized
    if (us.extra == core::ptr::null_mut()) {
    pub initialized\n"): usb_stor_dbg(us, "ERROR Driver not,
    pub 16: srb->result = DID_ERROR <<,
    }
    pub 0): scsi_set_resid(srb,,
// scsi_bufflen might change in protocol translation to ata
    pub scsi_bufflen(srb): orig_bufflen =,
    pub &ataCdb): sendToTransport = isd200_scsi_to_ata(srb, us,,
// send the command to the transport layer
    if (sendToTransport)
    pub &ataCdb): isd200_invoke_transport(us, srb,,
    pub orig_bufflen): isd200_srb_set_bufflen(srb,,
    }
    pub isd200_host_template: static struct scsi_host_template,
    static int isd200_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    pub us: *mut us_data,
    pub result: c_int,
    result = usb_stor_probe1(&us, intf, id,
    (id - isd200_usb_ids) + isd200_unusual_dev_list,
    if (result)
    pub result: return,
    pub ATA/ATAPI": us->protocol_name = "ISD200,
    pub isd200_ata_command: us->proto_handler =,
    pub usb_stor_probe2(us): result =,
    pub result: return,
    }
    static struct usb_driver isd200_driver = {
    .name =		DRV_NAME,
    .probe =	isd200_probe,
    .disconnect =	usb_stor_disconnect,
    .suspend =	usb_stor_suspend,
    .resume =	usb_stor_resume,
    .reset_resume =	usb_stor_reset_resume,
    .pre_reset =	usb_stor_pre_reset,
    .post_reset =	usb_stor_post_reset,
    .id_table =	isd200_usb_ids,
    .soft_unbind =	1,
    .no_dynamic_id = 1,
}

    module_usb_stor_driver(isd200_driver, isd200_host_template, DRV_NAME);
