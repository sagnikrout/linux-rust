//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/ene_ub6250.c
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

    MODULE_DESCRIPTION("Driver for ENE UB6250 reader");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USB_STORAGE");
    MODULE_FIRMWARE(SD_INIT1_FIRMWARE);
    MODULE_FIRMWARE(SD_INIT2_FIRMWARE);
    MODULE_FIRMWARE(SD_RW_FIRMWARE);
    MODULE_FIRMWARE(MS_INIT_FIRMWARE);
    MODULE_FIRMWARE(MSP_RW_FIRMWARE);
    MODULE_FIRMWARE(MS_RW_FIRMWARE);
//
// The table of devices
//

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (flags)}
    static const struct usb_device_id ene_ub6250_usb_ids[] = {

    { }		/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, ene_ub6250_usb_ids);

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
    static const struct us_unusual_dev ene_ub6250_unusual_dev_list[] = {

    { }		/* Terminating entry */
    };

// ENE bin code len
pub const ENE_BIN_CODE_LEN: c_uint = 0x800;
// EnE HW Register
pub const REG_CARD_STATUS: c_uint = 0xFF83;
pub const REG_HW_TRAP1: c_uint = 0xFF89;
// SRB Status
pub const SS_SUCCESS: c_uint = 0x000000	/* No Sense */;
pub const SS_NOT_READY: c_uint = 0x023A00	/* Medium not present */;
pub const SS_MEDIUM_ERR: c_uint = 0x031100	/* Unrecovered read error */;
pub const SS_HW_ERR: c_uint = 0x040800	/* Communication failure */;
pub const SS_ILLEGAL_REQUEST: c_uint = 0x052000	/* Invalid command */;
pub const SS_UNIT_ATTENTION: c_uint = 0x062900	/* Reset occurred */;
// ENE Load FW Pattern
pub const SD_INIT1_PATTERN: c_int = 1;
pub const SD_INIT2_PATTERN: c_int = 2;
pub const SD_RW_PATTERN: c_int = 3;
pub const MS_INIT_PATTERN: c_int = 4;
pub const MSP_RW_PATTERN: c_int = 5;
pub const MS_RW_PATTERN: c_int = 6;
pub const SM_INIT_PATTERN: c_int = 7;
pub const SM_RW_PATTERN: c_int = 8;
pub const FDIR_WRITE: c_int = 0;
pub const FDIR_READ: c_int = 1;
// For MS Card
// Status Register 1
pub const MS_REG_ST1_MB: c_uint = 0x80    /* media busy */;
pub const MS_REG_ST1_FB1: c_uint = 0x40    /* flush busy 1 */;
pub const MS_REG_ST1_DTER: c_uint = 0x20    /* error on data(corrected) */;
pub const MS_REG_ST1_UCDT: c_uint = 0x10    /* unable to correct data */;
pub const MS_REG_ST1_EXER: c_uint = 0x08    /* error on extra(corrected) */;
pub const MS_REG_ST1_UCEX: c_uint = 0x04    /* unable to correct extra */;
pub const MS_REG_ST1_FGER: c_uint = 0x02    /* error on overwrite flag(corrected) */;
pub const MS_REG_ST1_UCFG: c_uint = 0x01    /* unable to correct overwrite flag */;

// Overwrite Area
pub const MS_REG_OVR_BKST: c_uint = 0x80            /* block status */;

pub const MS_REG_OVR_BKST_NG: c_uint = 0x00            /* NG */;
pub const MS_REG_OVR_PGST0: c_uint = 0x40            /* page status */;
pub const MS_REG_OVR_PGST1: c_uint = 0x20;

pub const MS_REG_OVR_PGST_DATA_ERROR: c_uint = 0x00        /* data error */;
pub const MS_REG_OVR_UDST: c_uint = 0x10        /* update status */;
pub const MS_REG_OVR_UDST_UPDATING: c_uint = 0x00        /* updating */;

pub const MS_REG_OVR_RESERVED: c_uint = 0x08;

// Management Flag
pub const MS_REG_MNG_SCMS0: c_uint = 0x20    /* serial copy management system */;
pub const MS_REG_MNG_SCMS1: c_uint = 0x10;

pub const MS_REG_MNG_SCMS_NO_COPY: c_uint = 0x00;
pub const MS_REG_MNG_ATFLG: c_uint = 0x08    /* address transfer table flag */;

pub const MS_REG_MNG_ATFLG_ATTBL: c_uint = 0x00	/* address transfer table */;
pub const MS_REG_MNG_SYSFLG: c_uint = 0x04	/* system flag */;

pub const MS_REG_MNG_SYSFLG_BOOT: c_uint = 0x00	/* system block */;
pub const MS_REG_MNG_RESERVED: c_uint = 0xc3;

pub const MS_MAX_PAGES_PER_BLOCK: c_int = 32;
pub const MS_MAX_INITIAL_ERROR_BLOCKS: c_int = 10;
pub const MS_LIB_BITS_PER_BYTE: c_int = 8;
pub const MS_SYSINF_FORMAT_FAT: c_int = 1;
pub const MS_SYSINF_USAGE_GENERAL: c_int = 0;
pub const MS_SYSINF_MSCLASS_TYPE_1: c_int = 1;

pub const MS_SYSINF_CARDTYPE_RDONLY: c_int = 1;
pub const MS_SYSINF_CARDTYPE_RDWR: c_int = 2;
pub const MS_SYSINF_CARDTYPE_HYBRID: c_int = 3;
pub const MS_SYSINF_SECURITY: c_uint = 0x01;

pub const MS_SYSINF_SECURITY_SUPPORT: c_int = 0;
pub const MS_SYSINF_RESERVED1: c_int = 1;
pub const MS_SYSINF_RESERVED2: c_int = 1;
pub const MS_SYSENT_TYPE_INVALID_BLOCK: c_uint = 0x01;
pub const MS_SYSENT_TYPE_CIS_IDI: c_uint = 0x0a    /* CIS/IDI */;
pub const SIZE_OF_KIRO: c_int = 1024;
pub const BYTE_MASK: c_uint = 0xff;
// ms error code
pub const MS_STATUS_WRITE_PROTECT: c_uint = 0x0106;
pub const MS_STATUS_SUCCESS: c_uint = 0x0000;
pub const MS_ERROR_FLASH_READ: c_uint = 0x8003;
pub const MS_ERROR_FLASH_ERASE: c_uint = 0x8005;
pub const MS_LB_ERROR: c_uint = 0xfff0;
pub const MS_LB_BOOT_BLOCK: c_uint = 0xfff1;
pub const MS_LB_INITIAL_ERROR: c_uint = 0xfff2;
pub const MS_STATUS_SUCCESS_WITH_ECC: c_uint = 0xfff3;
pub const MS_LB_ACQUIRED_ERROR: c_uint = 0xfff4;
pub const MS_LB_NOT_USED_ERASED: c_uint = 0xfff5;
pub const MS_NOCARD_ERROR: c_uint = 0xfff8;
pub const MS_NO_MEMORY_ERROR: c_uint = 0xfff9;
pub const MS_STATUS_INT_ERROR: c_uint = 0xfffa;
pub const MS_STATUS_ERROR: c_uint = 0xfffe;
pub const MS_LB_NOT_USED: c_uint = 0xffff;
pub const MS_REG_MNG_SYSFLG: c_uint = 0x04    /* system flag */;

pub const MS_BOOT_BLOCK_ID: c_uint = 0x0001;
pub const MS_BOOT_BLOCK_FORMAT_VERSION: c_uint = 0x0100;
pub const MS_BOOT_BLOCK_DATA_ENTRIES: c_int = 2;
pub const MS_NUMBER_OF_SYSTEM_ENTRY: c_int = 4;
pub const MS_NUMBER_OF_BOOT_BLOCK: c_int = 2;
pub const MS_BYTES_PER_PAGE: c_int = 512;
pub const MS_LOGICAL_BLOCKS_PER_SEGMENT: c_int = 496;
pub const MS_LOGICAL_BLOCKS_IN_1ST_SEGMENT: c_int = 494;
pub const MS_PHYSICAL_BLOCKS_PER_SEGMENT: c_uint = 0x200 /* 512 */;
pub const MS_PHYSICAL_BLOCKS_PER_SEGMENT_MASK: c_uint = 0x1ff;
// overwrite area
pub const MS_REG_OVR_BKST: c_uint = 0x80		/* block status */;

pub const MS_REG_OVR_BKST_NG: c_uint = 0x00            /* NG */;
// Status Register 1
pub const MS_REG_ST1_DTER: c_uint = 0x20	/* error on data(corrected) */;
pub const MS_REG_ST1_EXER: c_uint = 0x08	/* error on extra(corrected) */;
pub const MS_REG_ST1_FGER: c_uint = 0x02	/* error on overwrite flag(corrected) */;
// MemoryStick Register
// Status Register 0
pub const MS_REG_ST0_WP: c_uint = 0x01	/* write protected */;

pub const MS_LIB_CTRL_RDONLY: c_int = 0;
pub const MS_LIB_CTRL_WRPROTECT: c_int = 1;
// dphy->log table

// SD_STATUS bits

// Bit 7 reserved
// MS_STATUS bits

// Bit 5 reserved

// Bit 7 reserved
// SM_STATUS bits

// Bits 3-5 reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_cis {
    pub /: *mut *mut u8 bCistplDEVICE[6]; / 0,
    pub /: *mut *mut u8 bCistplDEVICE0C[6]; / 6,
    pub /: *mut *mut u8 bCistplJEDECC[4]; / 12,
    pub /: *mut *mut u8 bCistplMANFID[6]; / 16,
    pub /: *mut *mut u8 bCistplVER1[32]; / 22,
    pub /: *mut *mut u8 bCistplFUNCID[4]; / 54,
    pub /: *mut *mut u8 bCistplFUNCE0[4]; / 58,
    pub /: *mut *mut u8 bCistplFUNCE1[5]; / 62,
    pub /: *mut *mut u8 bCistplCONF[7]; / 67,
    pub /: *mut *mut u8 bCistplCFTBLENT0[10];/ 74,
    pub /: *mut *mut u8 bCistplCFTBLENT1[8]; / 84,
    pub /: *mut *mut u8 bCistplCFTBLENT2[12];/ 92,
    pub /: *mut *mut u8 bCistplCFTBLENT3[8]; / 104,
    pub /: *mut *mut u8 bCistplCFTBLENT4[17];/ 112,
    pub /: *mut *mut u8 bCistplCFTBLENT5[8]; / 129,
    pub /: *mut *mut u8 bCistplCFTBLENT6[17];/ 137,
    pub /: *mut *mut u8 bCistplCFTBLENT7[8]; / 154,
    pub /: *mut *mut u8 bCistplNOLINK[3]; / 162,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_idi {
pub const MS_IDI_GENERAL_CONF: c_uint = 0x848A;
    pub /: *mut *mut u16 wIDIgeneralConfiguration; / 0,
    pub /: *mut *mut u16 wIDInumberOfCylinder; / 1,
    pub /: *mut *mut u16 wIDIreserved0; / 2,
    pub /: *mut *mut u16 wIDInumberOfHead; / 3,
    pub /: *mut *mut u16 wIDIbytesPerTrack; / 4,
    pub /: *mut *mut u16 wIDIbytesPerSector; / 5,
    pub /: *mut *mut u16 wIDIsectorsPerTrack; / 6,
    pub /: *mut *mut u16 wIDItotalSectors[2]; / 7-8 high,low,
    pub /: *mut *mut u16 wIDIreserved1[11]; / 9-19,
    pub /: *mut *mut u16 wIDIbufferType; / 20,
    pub /: *mut *mut u16 wIDIbufferSize; / 21,
    pub /: *mut *mut u16 wIDIlongCmdECC; / 22,
    pub /: *mut *mut u16 wIDIfirmVersion[4]; / 23-26,
    pub /: *mut *mut u16 wIDImodelName[20]; / 27-46,
    pub /: *mut *mut u16 wIDIreserved2; / 47,
    pub /: *mut *mut u16 wIDIlongWordSupported; / 48,
    pub /: *mut *mut u16 wIDIdmaSupported; / 49,
    pub /: *mut *mut u16 wIDIreserved3; / 50,
    pub /: *mut *mut u16 wIDIpioTiming; / 51,
    pub /: *mut *mut u16 wIDIdmaTiming; / 52,
    pub /: *mut *mut u16 wIDItransferParameter; / 53,
    pub /: *mut *mut u16 wIDIformattedCylinder; / 54,
    pub /: *mut *mut u16 wIDIformattedHead; / 55,
    pub /: *mut *mut u16 wIDIformattedSectorsPerTrack;/ 56,
    pub /: *mut *mut u16 wIDIformattedTotalSectors[2];/ 57-58,
    pub /: *mut *mut u16 wIDImultiSector; / 59,
    pub /: *mut *mut u16 wIDIlbaSectors[2]; / 60-61,
    pub /: *mut *mut u16 wIDIsingleWordDMA; / 62,
    pub /: *mut *mut u16 wIDImultiWordDMA; / 63,
    pub /: *mut *mut u16 wIDIreserved4[192]; / 64-255,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_sysent_rec {
    pub dwStart: u32,
    pub dwSize: u32,
    pub bType: u8,
    pub bReserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_sysent {
    pub entry: [ms_bootblock_sysent_rec; MS_NUMBER_OF_SYSTEM_ENTRY],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_sysinf {
    pub /: *mut *mut u8 bMsClass; / must be 1,
    pub /: *mut *mut u8 bCardType; / see below,
    pub /: *mut *mut u16 wBlockSize; / n KB,
    pub /: *mut *mut u16 wBlockNumber; / number of physical block,
    pub /: *mut *mut u16 wTotalBlockNumber; / number of logical block,
    pub /: *mut *mut u16 wPageSize; / must be 0x200,
    pub /: *mut *mut u8 bExtraSize; / 0x10,
    pub bSecuritySupport: u8,
    pub bAssemblyDate: [u8; 8],
    pub bFactoryArea: [u8; 4],
    pub bAssemblyMakerCode: u8,
    pub bAssemblyMachineCode: [u8; 3],
    pub wMemoryMakerCode: u16,
    pub wMemoryDeviceCode: u16,
    pub wMemorySize: u16,
    pub bReserved1: u8,
    pub bReserved2: u8,
    pub bVCC: u8,
    pub bVPP: u8,
    pub wControllerChipNumber: u16,
    pub /: *mut *mut u16 wControllerFunction; / New MS,
    pub /: *mut *mut u8 bReserved3[9]; / New MS,
    pub /: *mut *mut u8 bParallelSupport; / New MS,
    pub /: *mut *mut u16 wFormatValue; / New MS,
    pub bFormatType: u8,
    pub bUsage: u8,
    pub bDeviceType: u8,
    pub bReserved4: [u8; 22],
    pub bFUValue3: u8,
    pub bFUValue4: u8,
    pub bReserved5: [u8; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_header {
    pub wBlockID: u16,
    pub wFormatVersion: u16,
    pub bReserved1: [u8; 184],
    pub bNumberOfDataEntry: u8,
    pub bReserved2: [u8; 179],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_page0 {
    pub header: ms_bootblock_header,
    pub sysent: ms_bootblock_sysent,
    pub sysinf: ms_bootblock_sysinf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_bootblock_cis_idi {
    union {
    pub cis: ms_bootblock_cis,
    pub dmy: [u8; 256],
    pub cis: },
    union {
    pub idi: ms_bootblock_idi,
    pub dmy: [u8; 256],
    pub idi: },
}

// ENE MS Lib struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_lib_type_extdat {
    pub reserved: u8,
    pub intr: u8,
    pub status0: u8,
    pub status1: u8,
    pub ovrflg: u8,
    pub mngflg: u8,
    pub logadr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_lib_ctrl {
    pub flags: u32,
    pub BytesPerSector: u32,
    pub NumberOfCylinder: u32,
    pub SectorsPerCylinder: u32,
    pub /: *mut *mut u16 cardType; / R/W, RO, Hybrid,
    pub blockSize: u16,
    pub PagesPerBlock: u16,
    pub NumberOfPhyBlock: u16,
    pub NumberOfLogBlock: u16,
    pub NumberOfSegment: u16,
    pub /: *mut *mut *mut u16 Phy2LogMap; / phy2log table,
    pub /: *mut *mut *mut u16 Log2PhyMap; / log2phy table,
    pub wrtblk: u16,
    pub MS_LIB_BITS_PER_BYTE]: *mut *mut unsigned char pagemap[(MS_MAX_PAGES_PER_BLOCK + (MS_LIB_BITS_PER_BYTE-1)) /,
    pub blkpag: *mut c_uchar,
    pub blkext: *mut ms_lib_type_extdat,
    pub copybuf: [c_uchar; 512],
}

// SD Block Length
// 2^9 = 512 Bytes, The HW maximum read/write data length
pub const SD_BLOCK_LEN: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ene_ub6250_info {
// I/O bounce buffer
    pub bbuf: *mut u8,
// for 6250 code
    pub SD_Status: u8,
    pub MS_Status: u8,
    pub SM_Status: u8,
// ----- SD Control Data ----------------
// SD_REGISTER SD_Regs;
    pub SD_Block_Mult: u16,
    pub SD_READ_BL_LEN: u8,
    pub SD_C_SIZE: u16,
    pub SD_C_SIZE_MULT: u8,
// SD/MMC New spec.
    pub SD_SPEC_VER: u8,
    pub SD_CSD_VER: u8,
    pub SD20_HIGH_CAPACITY: u8,
    pub HC_C_SIZE: u32,
    pub MMC_SPEC_VER: u8,
    pub MMC_BusWidth: u8,
    pub MMC_HIGH_CAPACITY: u8,
// ----- MS Control Data ----------------
    pub MS_SWWP: bool,
    pub MSP_TotalBlock: u32,
    pub MS_Lib: ms_lib_ctrl,
    pub MS_IsRWPage: bool,
    pub MS_Model: u16,
// ----- SM Control Data ----------------
    pub SM_DeviceID: u8,
    pub SM_CardID: u8,
    pub testbuf: *mut c_uchar,
    pub BIN_FLAG: u8,
    pub bl_num: u32,
    pub SrbStatus: c_int,
// ------Power Managerment ---------------
    pub Power_IsResum: bool,
}

    static int ene_sd_init(struct us_data *us);
    static int ene_ms_init(struct us_data *us);
    static int ene_load_bincode(struct us_data *us, unsigned char flag);
#[no_mangle]
unsafe extern "C" fn ene_ub6250_info_destructor(extra: *mut c_void) {
    static void ene_ub6250_info_destructor(void *extra)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) extra;
    if (!extra)
    return;
    kfree(info.bbuf);
    }
#[no_mangle]
unsafe extern "C" fn ene_send_scsi_cmd(us: *mut us_data, fDir: u8, buf: *mut c_void, use_sg: c_int) -> c_int {
    static int ene_send_scsi_cmd(struct us_data *us, u8 fDir, void *buf, int use_sg)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct bulk_cs_wrap *bcs = (struct bulk_cs_wrap *) us.iobuf;
    int result;
    unsigned int residue;
    let mut cswlen: c_uint = 0, partial = 0;
    let mut transfer_length: c_uint = bcb.DataTransferLength;
// usb_stor_dbg(us, "transport --- ene_send_scsi_cmd\n");
// send cmd to out endpoint
    result = usb_stor_bulk_transfer_buf(us, us.send_bulk_pipe,
    bcb, US_BULK_CB_WRAP_LEN, core::ptr::null_mut());
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "send cmd to out endpoint fail ---\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    if (buf) {
    let mut pipe: c_uint = fDir;
    if (fDir  == FDIR_READ)
    pipe = us.recv_bulk_pipe;
    else
    pipe = us.send_bulk_pipe;
// Bulk
    if (use_sg) {
    result = usb_stor_bulk_srb(us, pipe, us.srb);
    } else {
    result = usb_stor_bulk_transfer_sg(us, pipe, buf,
    transfer_length, 0, &partial);
    }
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "data transfer fail ---\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    }
// Get CSW for device status
    result = usb_stor_bulk_transfer_buf(us, us.recv_bulk_pipe, bcs,
    US_BULK_CS_WRAP_LEN, &cswlen);
    if (result == USB_STOR_XFER_SHORT && cswlen == 0) {
    usb_stor_dbg(us, "Received 0-length CSW; retrying...\n");
    result = usb_stor_bulk_transfer_buf(us, us.recv_bulk_pipe,
    bcs, US_BULK_CS_WRAP_LEN, &cswlen);
    }
    if (result == USB_STOR_XFER_STALLED) {
// get the status again
    usb_stor_dbg(us, "Attempting to get CSW (2nd try)...\n");
    result = usb_stor_bulk_transfer_buf(us, us.recv_bulk_pipe,
    bcs, US_BULK_CS_WRAP_LEN, core::ptr::null_mut());
    }
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
// check bulk status
    residue = le32_to_cpu(bcs.Residue);
//
// try to compute the actual residue, based on how much data
// was really transferred and what the device tells us
//
    if (residue && !(us.fflags & US_FL_IGNORE_RESIDUE)) {
    residue = min(residue, transfer_length);
    if (us.srb != core::ptr::null_mut())
    scsi_set_resid(us.srb, max(scsi_get_resid(us.srb),
    residue));
    }
    if (bcs.Status != US_BULK_STAT_OK)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_request_sense(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int do_scsi_request_sense(struct us_data *us, struct scsi_cmnd *srb)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    unsigned char buf[18];
    memset(buf, 0, 18);
    buf[0] = 0x70;				/* Current error */
    buf[2] = info.SrbStatus >> 16;		/* Sense key */
    buf[7] = 10;				/* Additional length */
    buf[12] = info.SrbStatus >> 8;		/* ASC */
    buf[13] = info.SrbStatus;		/* ASCQ */
    usb_stor_set_xfer_buf(buf, sizeof(buf), srb);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_inquiry(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int do_scsi_inquiry(struct us_data *us, struct scsi_cmnd *srb)
    {
    unsigned char data_ptr[36] = {
    0x00, 0x00, 0x02, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x55,
    0x53, 0x42, 0x32, 0x2E, 0x30, 0x20, 0x20, 0x43, 0x61,
    0x72, 0x64, 0x52, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x30, 0x31, 0x30, 0x30 };
    usb_stor_set_xfer_buf(data_ptr, 36, srb);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn sd_scsi_test_unit_ready(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_test_unit_ready(struct us_data *us, struct scsi_cmnd *srb)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if ((info.SD_Status & SD_Insert) && (info.SD_Status & SD_Ready))
    return USB_STOR_TRANSPORT_GOOD;
    else {
    ene_sd_init(us);
    return USB_STOR_TRANSPORT_GOOD;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn sd_scsi_mode_sense(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_mode_sense(struct us_data *us, struct scsi_cmnd *srb)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    unsigned char mediaNoWP[12] = {
    0x0b, 0x00, 0x00, 0x08, 0x00, 0x00,
    0x71, 0xc0, 0x00, 0x00, 0x02, 0x00 };
    unsigned char mediaWP[12]   = {
    0x0b, 0x00, 0x80, 0x08, 0x00, 0x00,
    0x71, 0xc0, 0x00, 0x00, 0x02, 0x00 };
    if (info.SD_Status & SD_WtP)
    usb_stor_set_xfer_buf(mediaWP, 12, srb);
    else
    usb_stor_set_xfer_buf(mediaNoWP, 12, srb);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn sd_scsi_read_capacity(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_read_capacity(struct us_data *us, struct scsi_cmnd *srb)
    {
    u32	bl_num;
    u32	bl_len;
    let mut offset: c_uint = 0;
    unsigned char    buf[8];
    struct scatterlist *sg = core::ptr::null_mut();
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    usb_stor_dbg(us, "sd_scsi_read_capacity\n");
    if (info.SD_Status & SD_HiCapacity) {
    bl_len = 0x200;
    if (info.SD_Status & SD_IsMMC)
    bl_num = info.HC_C_SIZE-1;
    else
    bl_num = (info.HC_C_SIZE + 1) * 1024 - 1;
    } else {
    bl_len = 1 << (info.SD_READ_BL_LEN);
    bl_num = info.SD_Block_Mult * (info.SD_C_SIZE + 1)
// (1 << (info->SD_C_SIZE_MULT + 2)) - 1;
    }
    info.bl_num = bl_num;
    usb_stor_dbg(us, "bl_len = %x\n", bl_len);
    usb_stor_dbg(us, "bl_num = %x\n", bl_num);
// srb->request_bufflen = 8;
    buf[0] = (bl_num >> 24) & 0xff;
    buf[1] = (bl_num >> 16) & 0xff;
    buf[2] = (bl_num >> 8) & 0xff;
    buf[3] = (bl_num >> 0) & 0xff;
    buf[4] = (bl_len >> 24) & 0xff;
    buf[5] = (bl_len >> 16) & 0xff;
    buf[6] = (bl_len >> 8) & 0xff;
    buf[7] = (bl_len >> 0) & 0xff;
    usb_stor_access_xfer_buf(buf, 8, srb, &sg, &offset, TO_XFER_BUF);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn sd_scsi_read(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_read(struct us_data *us, struct scsi_cmnd *srb)
    {
    int result;
    unsigned char *cdb = srb.cmnd;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u32 bn = ((cdb[2] << 24) & 0xff000000) | ((cdb[3] << 16) & 0x00ff0000) |
    ((cdb[4] << 8) & 0x0000ff00) | ((cdb[5] << 0) & 0x000000ff);
    let mut blen: u16 = ((cdb[7] << 8) & 0xff00) | ((cdb[8] << 0) & 0x00ff);
    let mut bnByte: u32 = bn * 0x200;
    let mut blenByte: u32 = blen * 0x200;
    if (bn > info.bl_num)
    return USB_STOR_TRANSPORT_ERROR;
    result = ene_load_bincode(us, SD_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Load SD RW pattern Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    if (info.SD_Status & SD_HiCapacity)
    bnByte = bn;
// set up the command wrapper
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = blenByte;
    bcb.Flags  = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF1;
    bcb.CDB[5] = (unsigned char)(bnByte);
    bcb.CDB[4] = (unsigned char)(bnByte>>8);
    bcb.CDB[3] = (unsigned char)(bnByte>>16);
    bcb.CDB[2] = (unsigned char)(bnByte>>24);
    result = ene_send_scsi_cmd(us, FDIR_READ, scsi_sglist(srb), 1);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn sd_scsi_write(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_write(struct us_data *us, struct scsi_cmnd *srb)
    {
    int result;
    unsigned char *cdb = srb.cmnd;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u32 bn = ((cdb[2] << 24) & 0xff000000) | ((cdb[3] << 16) & 0x00ff0000) |
    ((cdb[4] << 8) & 0x0000ff00) | ((cdb[5] << 0) & 0x000000ff);
    let mut blen: u16 = ((cdb[7] << 8) & 0xff00) | ((cdb[8] << 0) & 0x00ff);
    let mut bnByte: u32 = bn * 0x200;
    let mut blenByte: u32 = blen * 0x200;
    if (bn > info.bl_num)
    return USB_STOR_TRANSPORT_ERROR;
    result = ene_load_bincode(us, SD_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Load SD RW pattern Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    if (info.SD_Status & SD_HiCapacity)
    bnByte = bn;
// set up the command wrapper
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = blenByte;
    bcb.Flags  = US_BULK_FLAG_OUT;
    bcb.CDB[0] = 0xF0;
    bcb.CDB[5] = (unsigned char)(bnByte);
    bcb.CDB[4] = (unsigned char)(bnByte>>8);
    bcb.CDB[3] = (unsigned char)(bnByte>>16);
    bcb.CDB[2] = (unsigned char)(bnByte>>24);
    result = ene_send_scsi_cmd(us, FDIR_WRITE, scsi_sglist(srb), 1);
    return result;
    }
//
// ENE MS Card
//
#[no_mangle]
unsafe extern "C" fn ms_lib_set_logicalpair(us: *mut us_data, logblk: u16, phyblk: u16) -> c_int {
    static int ms_lib_set_logicalpair(struct us_data *us, u16 logblk, u16 phyblk)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if ((logblk >= info.MS_Lib.NumberOfLogBlock) || (phyblk >= info.MS_Lib.NumberOfPhyBlock))
    return (u32)-1;
    info.MS_Lib.Phy2LogMap[phyblk] = logblk;
    info.MS_Lib.Log2PhyMap[logblk] = phyblk;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_set_logicalblockmark(us: *mut us_data, phyblk: u16, mark: u16) -> c_int {
    static int ms_lib_set_logicalblockmark(struct us_data *us, u16 phyblk, u16 mark)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (phyblk >= info.MS_Lib.NumberOfPhyBlock)
    return (u32)-1;
    info.MS_Lib.Phy2LogMap[phyblk] = mark;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_set_initialerrorblock(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_lib_set_initialerrorblock(struct us_data *us, u16 phyblk)
    {
    return ms_lib_set_logicalblockmark(us, phyblk, MS_LB_INITIAL_ERROR);
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_set_bootblockmark(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_lib_set_bootblockmark(struct us_data *us, u16 phyblk)
    {
    return ms_lib_set_logicalblockmark(us, phyblk, MS_LB_BOOT_BLOCK);
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_free_logicalmap(us: *mut us_data) -> c_int {
    static int ms_lib_free_logicalmap(struct us_data *us)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    kfree(info.MS_Lib.Phy2LogMap);
    info.MS_Lib.Phy2LogMap = core::ptr::null_mut();
    kfree(info.MS_Lib.Log2PhyMap);
    info.MS_Lib.Log2PhyMap = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_alloc_logicalmap(us: *mut us_data) -> c_int {
    static int ms_lib_alloc_logicalmap(struct us_data *us)
    {
    u32  i;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    info.MS_Lib.Phy2LogMap = kmalloc_array(info.MS_Lib.NumberOfPhyBlock,
    sizeof(u16),
    GFP_KERNEL);
    info.MS_Lib.Log2PhyMap = kmalloc_array(info.MS_Lib.NumberOfLogBlock,
    sizeof(u16),
    GFP_KERNEL);
    if ((info.MS_Lib.Phy2LogMap == core::ptr::null_mut()) || (info.MS_Lib.Log2PhyMap == core::ptr::null_mut())) {
    ms_lib_free_logicalmap(us);
    return (u32)-1;
    }
    for (i = 0; i < info.MS_Lib.NumberOfPhyBlock; i++)
    info.MS_Lib.Phy2LogMap[i] = MS_LB_NOT_USED;
    for (i = 0; i < info.MS_Lib.NumberOfLogBlock; i++)
    info.MS_Lib.Log2PhyMap[i] = MS_LB_NOT_USED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_clear_writebuf(us: *mut us_data) {
    static void ms_lib_clear_writebuf(struct us_data *us)
    {
    int i;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    info.MS_Lib.wrtblk = (u16)-1;
    ms_lib_clear_pagemap(info);
    if (info.MS_Lib.blkpag)
    memset(info.MS_Lib.blkpag, 0xff, info.MS_Lib.PagesPerBlock * info.MS_Lib.BytesPerSector);
    if (info.MS_Lib.blkext) {
    for (i = 0; i < info.MS_Lib.PagesPerBlock; i++) {
    info.MS_Lib.blkext[i].status1 = MS_REG_ST1_DEFAULT;
    info.MS_Lib.blkext[i].ovrflg = MS_REG_OVR_DEFAULT;
    info.MS_Lib.blkext[i].mngflg = MS_REG_MNG_DEFAULT;
    info.MS_Lib.blkext[i].logadr = MS_LB_NOT_USED;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ms_count_freeblock(us: *mut us_data, PhyBlock: u16) -> c_int {
    static int ms_count_freeblock(struct us_data *us, u16 PhyBlock)
    {
    u32 Ende, Count;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    Ende = PhyBlock + MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    for (Count = 0; PhyBlock < Ende; PhyBlock++) {
    switch (info.MS_Lib.Phy2LogMap[PhyBlock]) {
    case MS_LB_NOT_USED:
    case MS_LB_NOT_USED_ERASED:
    Count++;
    break;
    default:
    break;
    }
    }
    return Count;
    }
    static int ms_read_readpage(struct us_data *us, u32 PhyBlockAddr,
    u8 PageNum, u32 *PageBuf, struct ms_lib_type_extdat *ExtraDat)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u8 *bbuf = info.bbuf;
    int result;
    let mut bn: u32 = PhyBlockAddr * 0x20 + PageNum;
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
// Read Page Data
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200;
    bcb.Flags      = US_BULK_FLAG_IN;
    bcb.CDB[0]     = 0xF1;
    bcb.CDB[1]     = 0x02; /* in init.c ENE_MSInit() is 0x01 */
    bcb.CDB[5]     = (unsigned char)(bn);
    bcb.CDB[4]     = (unsigned char)(bn>>8);
    bcb.CDB[3]     = (unsigned char)(bn>>16);
    bcb.CDB[2]     = (unsigned char)(bn>>24);
    result = ene_send_scsi_cmd(us, FDIR_READ, PageBuf, 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
// Read Extra Data
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x4;
    bcb.Flags      = US_BULK_FLAG_IN;
    bcb.CDB[0]     = 0xF1;
    bcb.CDB[1]     = 0x03;
    bcb.CDB[5]     = (unsigned char)(PageNum);
    bcb.CDB[4]     = (unsigned char)(PhyBlockAddr);
    bcb.CDB[3]     = (unsigned char)(PhyBlockAddr>>8);
    bcb.CDB[2]     = (unsigned char)(PhyBlockAddr>>16);
    bcb.CDB[6]     = 0x01;
    result = ene_send_scsi_cmd(us, FDIR_READ, bbuf, 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    ExtraDat.reserved = 0;
    ExtraDat.intr     = 0x80;  /* Not yet,fireware support */
    ExtraDat.status0  = 0x10;  /* Not yet,fireware support */
    ExtraDat.status1  = 0x00;  /* Not yet,fireware support */
    ExtraDat.ovrflg   = bbuf[0];
    ExtraDat.mngflg   = bbuf[1];
    ExtraDat.logadr   = memstick_logaddr(bbuf[2], bbuf[3]);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_process_bootblock(us: *mut us_data, PhyBlock: u16, PageData: *mut u8) -> c_int {
    static int ms_lib_process_bootblock(struct us_data *us, u16 PhyBlock, u8 *PageData)
    {
    struct ms_bootblock_sysent *SysEntry;
    struct ms_bootblock_sysinf *SysInfo;
    u32 i, result;
    u8 PageNumber;
    u8 *PageBuffer;
    struct ms_lib_type_extdat ExtraData;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    PageBuffer = kzalloc(MS_BYTES_PER_PAGE * 2, GFP_KERNEL);
    if (PageBuffer == core::ptr::null_mut())
    return (u32)-1;
    result = (u32)-1;
    SysInfo = &(((struct ms_bootblock_page0 *)PageData).sysinf);
    if ((SysInfo.bMsClass != MS_SYSINF_MSCLASS_TYPE_1) ||
    (be16_to_cpu(SysInfo.wPageSize) != MS_SYSINF_PAGE_SIZE) ||
    ((SysInfo.bSecuritySupport & MS_SYSINF_SECURITY) == MS_SYSINF_SECURITY_SUPPORT) ||
    (SysInfo.bReserved1 != MS_SYSINF_RESERVED1) ||
    (SysInfo.bReserved2 != MS_SYSINF_RESERVED2) ||
    (SysInfo.bFormatType != MS_SYSINF_FORMAT_FAT) ||
    (SysInfo.bUsage != MS_SYSINF_USAGE_GENERAL))
    goto exit;
//
    switch (info.MS_Lib.cardType = SysInfo.bCardType) {
    case MS_SYSINF_CARDTYPE_RDONLY:
    ms_lib_ctrl_set(info, MS_LIB_CTRL_RDONLY);
    break;
    case MS_SYSINF_CARDTYPE_RDWR:
    ms_lib_ctrl_reset(info, MS_LIB_CTRL_RDONLY);
    break;
    case MS_SYSINF_CARDTYPE_HYBRID:
    default:
    goto exit;
    }
    info.MS_Lib.blockSize = be16_to_cpu(SysInfo.wBlockSize);
    info.MS_Lib.NumberOfPhyBlock = be16_to_cpu(SysInfo.wBlockNumber);
    info.MS_Lib.NumberOfLogBlock = be16_to_cpu(SysInfo.wTotalBlockNumber)-2;
    info.MS_Lib.PagesPerBlock = info.MS_Lib.blockSize * SIZE_OF_KIRO / MS_BYTES_PER_PAGE;
    info.MS_Lib.NumberOfSegment = info.MS_Lib.NumberOfPhyBlock / MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    info.MS_Model = be16_to_cpu(SysInfo.wMemorySize);
// Allocate to all number of logicalblock and physicalblock
    if (ms_lib_alloc_logicalmap(us))
    goto exit;
// Mark the book block
    ms_lib_set_bootblockmark(us, PhyBlock);
    SysEntry = &(((struct ms_bootblock_page0 *)PageData).sysent);
    for (i = 0; i < MS_NUMBER_OF_SYSTEM_ENTRY; i++) {
    u32  EntryOffset, EntrySize;
    EntryOffset = be32_to_cpu(SysEntry.entry[i].dwStart);
    if (EntryOffset == 0xffffff)
    continue;
    EntrySize = be32_to_cpu(SysEntry.entry[i].dwSize);
    if (EntrySize == 0)
    continue;
    if (EntryOffset + MS_BYTES_PER_PAGE + EntrySize > info.MS_Lib.blockSize * (u32)SIZE_OF_KIRO)
    continue;
    if (i == 0) {
    let mut PrevPageNumber: u8 = 0;
    u16 phyblk;
    if (SysEntry.entry[i].bType != MS_SYSENT_TYPE_INVALID_BLOCK)
    goto exit;
    while (EntrySize > 0) {
    PageNumber = (u8)(EntryOffset / MS_BYTES_PER_PAGE + 1);
    if (PageNumber != PrevPageNumber) {
    switch (ms_read_readpage(us, PhyBlock, PageNumber, (u32 *)PageBuffer, &ExtraData)) {
    case MS_STATUS_SUCCESS:
    break;
    case MS_STATUS_WRITE_PROTECT:
    case MS_ERROR_FLASH_READ:
    case MS_STATUS_ERROR:
    default:
    goto exit;
    }
    PrevPageNumber = PageNumber;
    }
    phyblk = be16_to_cpu(*(u16 *)(PageBuffer + (EntryOffset % MS_BYTES_PER_PAGE)));
    if (phyblk < 0x0fff)
    ms_lib_set_initialerrorblock(us, phyblk);
    EntryOffset += 2;
    EntrySize -= 2;
    }
    } else if (i == 1) {  /* CIS/IDI */
    struct ms_bootblock_idi *idi;
    if (SysEntry.entry[i].bType != MS_SYSENT_TYPE_CIS_IDI)
    goto exit;
    switch (ms_read_readpage(us, PhyBlock, (u8)(EntryOffset / MS_BYTES_PER_PAGE + 1), (u32 *)PageBuffer, &ExtraData)) {
    case MS_STATUS_SUCCESS:
    break;
    case MS_STATUS_WRITE_PROTECT:
    case MS_ERROR_FLASH_READ:
    case MS_STATUS_ERROR:
    default:
    goto exit;
    }
    idi = &((struct ms_bootblock_cis_idi *)(PageBuffer + (EntryOffset % MS_BYTES_PER_PAGE))).idi.idi;
    if (le16_to_cpu(idi.wIDIgeneralConfiguration) != MS_IDI_GENERAL_CONF)
    goto exit;
    info.MS_Lib.BytesPerSector = le16_to_cpu(idi.wIDIbytesPerSector);
    if (info.MS_Lib.BytesPerSector != MS_BYTES_PER_PAGE)
    goto exit;
    }
    } /* End for .. */
    result = 0;
    exit:
    if (result)
    ms_lib_free_logicalmap(us);
    kfree(PageBuffer);
    result = 0;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_free_writebuf(us: *mut us_data) {
    static void ms_lib_free_writebuf(struct us_data *us)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    info.MS_Lib.wrtblk = (u16)-1; /* set to -1 */
// memset((fdoExt)->MS_Lib.pagemap, 0, sizeof((fdoExt)->MS_Lib.pagemap))
    ms_lib_clear_pagemap(info); /* (pdx).MS_Lib.pagemap memset 0 in ms.h */
    if (info.MS_Lib.blkpag) {
    kfree(info.MS_Lib.blkpag);  /* Arnold test ... */
    info.MS_Lib.blkpag = core::ptr::null_mut();
    }
    if (info.MS_Lib.blkext) {
    kfree(info.MS_Lib.blkext);  /* Arnold test ... */
    info.MS_Lib.blkext = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_free_allocatedarea(us: *mut us_data) {
    static void ms_lib_free_allocatedarea(struct us_data *us)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    ms_lib_free_writebuf(us); /* Free MS_Lib.pagemap */
    ms_lib_free_logicalmap(us); /* kfree MS_Lib.Phy2LogMap and MS_Lib.Log2PhyMap */
// set struct us point flag to 0
    info.MS_Lib.flags = 0;
    info.MS_Lib.BytesPerSector = 0;
    info.MS_Lib.SectorsPerCylinder = 0;
    info.MS_Lib.cardType = 0;
    info.MS_Lib.blockSize = 0;
    info.MS_Lib.PagesPerBlock = 0;
    info.MS_Lib.NumberOfPhyBlock = 0;
    info.MS_Lib.NumberOfLogBlock = 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_alloc_writebuf(us: *mut us_data) -> c_int {
    static int ms_lib_alloc_writebuf(struct us_data *us)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    info.MS_Lib.wrtblk = (u16)-1;
    info.MS_Lib.blkpag = kmalloc_array(info.MS_Lib.PagesPerBlock,
    info.MS_Lib.BytesPerSector,
    GFP_KERNEL);
    info.MS_Lib.blkext = kmalloc_objs(struct ms_lib_type_extdat,
    info.MS_Lib.PagesPerBlock);
    if ((info.MS_Lib.blkpag == core::ptr::null_mut()) || (info.MS_Lib.blkext == core::ptr::null_mut())) {
    ms_lib_free_writebuf(us);
    return (u32)-1;
    }
    ms_lib_clear_writebuf(us);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_force_setlogical_pair(us: *mut us_data, logblk: u16, phyblk: u16) -> c_int {
    static int ms_lib_force_setlogical_pair(struct us_data *us, u16 logblk, u16 phyblk)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (logblk == MS_LB_NOT_USED)
    return 0;
    if ((logblk >= info.MS_Lib.NumberOfLogBlock) ||
    (phyblk >= info.MS_Lib.NumberOfPhyBlock))
    return (u32)-1;
    info.MS_Lib.Phy2LogMap[phyblk] = logblk;
    info.MS_Lib.Log2PhyMap[logblk] = phyblk;
    return 0;
    }
    static int ms_read_copyblock(struct us_data *us, u16 oldphy, u16 newphy,
    u16 PhyBlockAddr, u8 PageNum, unsigned char *buf, u16 len)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int result;
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200*len;
    bcb.Flags = US_BULK_FLAG_OUT;
    bcb.CDB[0] = 0xF0;
    bcb.CDB[1] = 0x08;
    bcb.CDB[4] = (unsigned char)(oldphy);
    bcb.CDB[3] = (unsigned char)(oldphy>>8);
    bcb.CDB[2] = 0; /* (BYTE)(oldphy>>16) */
    bcb.CDB[7] = (unsigned char)(newphy);
    bcb.CDB[6] = (unsigned char)(newphy>>8);
    bcb.CDB[5] = 0; /* (BYTE)(newphy>>16) */
    bcb.CDB[9] = (unsigned char)(PhyBlockAddr);
    bcb.CDB[8] = (unsigned char)(PhyBlockAddr>>8);
    bcb.CDB[10] = PageNum;
    result = ene_send_scsi_cmd(us, FDIR_WRITE, buf, 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_read_eraseblock(us: *mut us_data, PhyBlockAddr: u32) -> c_int {
    static int ms_read_eraseblock(struct us_data *us, u32 PhyBlockAddr)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int result;
    let mut bn: u32 = PhyBlockAddr;
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200;
    bcb.Flags = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF2;
    bcb.CDB[1] = 0x06;
    bcb.CDB[4] = (unsigned char)(bn);
    bcb.CDB[3] = (unsigned char)(bn>>8);
    bcb.CDB[2] = (unsigned char)(bn>>16);
    result = ene_send_scsi_cmd(us, FDIR_READ, core::ptr::null_mut(), 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_check_disableblock(us: *mut us_data, PhyBlock: u16) -> c_int {
    static int ms_lib_check_disableblock(struct us_data *us, u16 PhyBlock)
    {
    unsigned char *PageBuf = core::ptr::null_mut();
    let mut result: u16 = MS_STATUS_SUCCESS;
    u16 blk, index = 0;
    struct ms_lib_type_extdat extdat;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    PageBuf = kmalloc(MS_BYTES_PER_PAGE, GFP_KERNEL);
    if (PageBuf == core::ptr::null_mut()) {
    result = MS_NO_MEMORY_ERROR;
    goto exit;
    }
    ms_read_readpage(us, PhyBlock, 1, (u32 *)PageBuf, &extdat);
    do {
    blk = be16_to_cpu(PageBuf[index]);
    if (blk == MS_LB_NOT_USED)
    break;
    if (blk == info.MS_Lib.Log2PhyMap[0]) {
    result = MS_ERROR_FLASH_READ;
    break;
    }
    index++;
    } while (1);
    exit:
    kfree(PageBuf);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_setacquired_errorblock(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_lib_setacquired_errorblock(struct us_data *us, u16 phyblk)
    {
    u16 log;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (phyblk >= info.MS_Lib.NumberOfPhyBlock)
    return (u32)-1;
    log = info.MS_Lib.Phy2LogMap[phyblk];
    if (log < info.MS_Lib.NumberOfLogBlock)
    info.MS_Lib.Log2PhyMap[log] = MS_LB_NOT_USED;
    if (info.MS_Lib.Phy2LogMap[phyblk] != MS_LB_INITIAL_ERROR)
    info.MS_Lib.Phy2LogMap[phyblk] = MS_LB_ACQUIRED_ERROR;
    return 0;
    }
    static int ms_lib_overwrite_extra(struct us_data *us, u32 PhyBlockAddr,
    u8 PageNum, u8 OverwriteFlag)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int result;
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x4;
    bcb.Flags = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF2;
    bcb.CDB[1] = 0x05;
    bcb.CDB[5] = (unsigned char)(PageNum);
    bcb.CDB[4] = (unsigned char)(PhyBlockAddr);
    bcb.CDB[3] = (unsigned char)(PhyBlockAddr>>8);
    bcb.CDB[2] = (unsigned char)(PhyBlockAddr>>16);
    bcb.CDB[6] = OverwriteFlag;
    bcb.CDB[7] = 0xFF;
    bcb.CDB[8] = 0xFF;
    bcb.CDB[9] = 0xFF;
    result = ene_send_scsi_cmd(us, FDIR_READ, core::ptr::null_mut(), 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_error_phyblock(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_lib_error_phyblock(struct us_data *us, u16 phyblk)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (phyblk >= info.MS_Lib.NumberOfPhyBlock)
    return MS_STATUS_ERROR;
    ms_lib_setacquired_errorblock(us, phyblk);
    if (ms_lib_iswritable(info))
    return ms_lib_overwrite_extra(us, phyblk, 0, (u8)(~MS_REG_OVR_BKST & BYTE_MASK));
    return MS_STATUS_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_erase_phyblock(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_lib_erase_phyblock(struct us_data *us, u16 phyblk)
    {
    u16 log;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (phyblk >= info.MS_Lib.NumberOfPhyBlock)
    return MS_STATUS_ERROR;
    log = info.MS_Lib.Phy2LogMap[phyblk];
    if (log < info.MS_Lib.NumberOfLogBlock)
    info.MS_Lib.Log2PhyMap[log] = MS_LB_NOT_USED;
    info.MS_Lib.Phy2LogMap[phyblk] = MS_LB_NOT_USED;
    if (ms_lib_iswritable(info)) {
    switch (ms_read_eraseblock(us, phyblk)) {
    case MS_STATUS_SUCCESS:
    info.MS_Lib.Phy2LogMap[phyblk] = MS_LB_NOT_USED_ERASED;
    return MS_STATUS_SUCCESS;
    case MS_ERROR_FLASH_ERASE:
    case MS_STATUS_INT_ERROR:
    ms_lib_error_phyblock(us, phyblk);
    return MS_ERROR_FLASH_ERASE;
    case MS_STATUS_ERROR:
    default:
    ms_lib_ctrl_set(info, MS_LIB_CTRL_RDONLY); /* MS_LibCtrlSet will used by ENE_MSInit ,need check, and why us to info*/
    ms_lib_setacquired_errorblock(us, phyblk);
    return MS_STATUS_ERROR;
    }
    }
    ms_lib_setacquired_errorblock(us, phyblk);
    return MS_STATUS_SUCCESS;
    }
    static int ms_lib_read_extra(struct us_data *us, u32 PhyBlock,
    u8 PageNum, struct ms_lib_type_extdat *ExtraDat)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u8 *bbuf = info.bbuf;
    int result;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x4;
    bcb.Flags      = US_BULK_FLAG_IN;
    bcb.CDB[0]     = 0xF1;
    bcb.CDB[1]     = 0x03;
    bcb.CDB[5]     = (unsigned char)(PageNum);
    bcb.CDB[4]     = (unsigned char)(PhyBlock);
    bcb.CDB[3]     = (unsigned char)(PhyBlock>>8);
    bcb.CDB[2]     = (unsigned char)(PhyBlock>>16);
    bcb.CDB[6]     = 0x01;
    result = ene_send_scsi_cmd(us, FDIR_READ, bbuf, 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    ExtraDat.reserved = 0;
    ExtraDat.intr     = 0x80;  /* Not yet, waiting for fireware support */
    ExtraDat.status0  = 0x10;  /* Not yet, waiting for fireware support */
    ExtraDat.status1  = 0x00;  /* Not yet, waiting for fireware support */
    ExtraDat.ovrflg   = bbuf[0];
    ExtraDat.mngflg   = bbuf[1];
    ExtraDat.logadr   = memstick_logaddr(bbuf[2], bbuf[3]);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_libsearch_block_from_physical(us: *mut us_data, phyblk: u16) -> c_int {
    static int ms_libsearch_block_from_physical(struct us_data *us, u16 phyblk)
    {
    u16 blk;
    struct ms_lib_type_extdat extdat; /* need check */
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (phyblk >= info.MS_Lib.NumberOfPhyBlock)
    return MS_LB_ERROR;
    for (blk = phyblk + 1; blk != phyblk; blk++) {
    if ((blk & MS_PHYSICAL_BLOCKS_PER_SEGMENT_MASK) == 0)
    blk -= MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    if (info.MS_Lib.Phy2LogMap[blk] == MS_LB_NOT_USED_ERASED) {
    return blk;
    } else if (info.MS_Lib.Phy2LogMap[blk] == MS_LB_NOT_USED) {
    switch (ms_lib_read_extra(us, blk, 0, &extdat)) {
    case MS_STATUS_SUCCESS:
    case MS_STATUS_SUCCESS_WITH_ECC:
    break;
    case MS_NOCARD_ERROR:
    return MS_NOCARD_ERROR;
    case MS_STATUS_INT_ERROR:
    return MS_LB_ERROR;
    case MS_ERROR_FLASH_READ:
    default:
    ms_lib_setacquired_errorblock(us, blk);
    continue;
    } /* End switch */
    if ((extdat.ovrflg & MS_REG_OVR_BKST) != MS_REG_OVR_BKST_OK) {
    ms_lib_setacquired_errorblock(us, blk);
    continue;
    }
    switch (ms_lib_erase_phyblock(us, blk)) {
    case MS_STATUS_SUCCESS:
    return blk;
    case MS_STATUS_ERROR:
    return MS_LB_ERROR;
    case MS_ERROR_FLASH_ERASE:
    default:
    ms_lib_error_phyblock(us, blk);
    break;
    }
    }
    } /* End for */
    return MS_LB_ERROR;
    }
#[no_mangle]
unsafe extern "C" fn ms_libsearch_block_from_logical(us: *mut us_data, logblk: u16) -> c_int {
    static int ms_libsearch_block_from_logical(struct us_data *us, u16 logblk)
    {
    u16 phyblk;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    phyblk = ms_libconv_to_physical(info, logblk);
    if (phyblk >= MS_LB_ERROR) {
    if (logblk >= info.MS_Lib.NumberOfLogBlock)
    return MS_LB_ERROR;
    phyblk = (logblk + MS_NUMBER_OF_BOOT_BLOCK) / MS_LOGICAL_BLOCKS_PER_SEGMENT;
    phyblk *= MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    phyblk += MS_PHYSICAL_BLOCKS_PER_SEGMENT - 1;
    }
    return ms_libsearch_block_from_physical(us, phyblk);
    }
#[no_mangle]
unsafe extern "C" fn ms_scsi_test_unit_ready(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_test_unit_ready(struct us_data *us, struct scsi_cmnd *srb)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)(us.extra);
// pr_info("MS_SCSI_Test_Unit_Ready\n");
    if ((info.MS_Status & MS_Insert) && (info.MS_Status & MS_Ready)) {
    return USB_STOR_TRANSPORT_GOOD;
    } else {
    ene_ms_init(us);
    return USB_STOR_TRANSPORT_GOOD;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_scsi_mode_sense(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_mode_sense(struct us_data *us, struct scsi_cmnd *srb)
    {
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    unsigned char mediaNoWP[12] = {
    0x0b, 0x00, 0x00, 0x08, 0x00, 0x00,
    0x71, 0xc0, 0x00, 0x00, 0x02, 0x00 };
    unsigned char mediaWP[12]   = {
    0x0b, 0x00, 0x80, 0x08, 0x00, 0x00,
    0x71, 0xc0, 0x00, 0x00, 0x02, 0x00 };
    if (info.MS_Status & MS_WtP)
    usb_stor_set_xfer_buf(mediaWP, 12, srb);
    else
    usb_stor_set_xfer_buf(mediaNoWP, 12, srb);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_scsi_read_capacity(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_read_capacity(struct us_data *us, struct scsi_cmnd *srb)
    {
    u32   bl_num;
    u32    bl_len;
    let mut offset: c_uint = 0;
    unsigned char    buf[8];
    struct scatterlist *sg = core::ptr::null_mut();
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    usb_stor_dbg(us, "ms_scsi_read_capacity\n");
    bl_len = 0x200;
    if (info.MS_Status & MS_IsMSPro)
    bl_num = info.MSP_TotalBlock - 1;
    else
    bl_num = info.MS_Lib.NumberOfLogBlock * info.MS_Lib.blockSize * 2 - 1;
    info.bl_num = bl_num;
    usb_stor_dbg(us, "bl_len = %x\n", bl_len);
    usb_stor_dbg(us, "bl_num = %x\n", bl_num);
// srb->request_bufflen = 8;
    buf[0] = (bl_num >> 24) & 0xff;
    buf[1] = (bl_num >> 16) & 0xff;
    buf[2] = (bl_num >> 8) & 0xff;
    buf[3] = (bl_num >> 0) & 0xff;
    buf[4] = (bl_len >> 24) & 0xff;
    buf[5] = (bl_len >> 16) & 0xff;
    buf[6] = (bl_len >> 8) & 0xff;
    buf[7] = (bl_len >> 0) & 0xff;
    usb_stor_access_xfer_buf(buf, 8, srb, &sg, &offset, TO_XFER_BUF);
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_phy_to_log_range(PhyBlock: u16, LogStart: *mut u16, LogEnde: *mut u16) {
    static void ms_lib_phy_to_log_range(u16 PhyBlock, u16 *LogStart, u16 *LogEnde)
    {
    PhyBlock /= MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    if (PhyBlock) {
// LogStart = MS_LOGICAL_BLOCKS_IN_1ST_SEGMENT + (PhyBlock - 1) * MS_LOGICAL_BLOCKS_PER_SEGMENT;/*496
// LogEnde = *LogStart + MS_LOGICAL_BLOCKS_PER_SEGMENT;/*496
    } else {
// LogStart = 0;
// LogEnde = MS_LOGICAL_BLOCKS_IN_1ST_SEGMENT;/*494
    }
    }
    static int ms_lib_read_extrablock(struct us_data *us, u32 PhyBlock,
    u8 PageNum, u8 blen, void *buf)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int     result;
// Read Extra Data
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x4 * blen;
    bcb.Flags      = US_BULK_FLAG_IN;
    bcb.CDB[0]     = 0xF1;
    bcb.CDB[1]     = 0x03;
    bcb.CDB[5]     = (unsigned char)(PageNum);
    bcb.CDB[4]     = (unsigned char)(PhyBlock);
    bcb.CDB[3]     = (unsigned char)(PhyBlock>>8);
    bcb.CDB[2]     = (unsigned char)(PhyBlock>>16);
    bcb.CDB[6]     = blen;
    result = ene_send_scsi_cmd(us, FDIR_READ, buf, 0);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ms_lib_scan_logicalblocknumber(us: *mut us_data, btBlk1st: u16) -> c_int {
    static int ms_lib_scan_logicalblocknumber(struct us_data *us, u16 btBlk1st)
    {
    u16 PhyBlock, newblk, i;
    u16 LogStart, LogEnde;
    struct ms_lib_type_extdat extdat;
    let mut count: u32 = 0, index = 0;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u8 *bbuf = info.bbuf;
    for (PhyBlock = 0; PhyBlock < info.MS_Lib.NumberOfPhyBlock;) {
    ms_lib_phy_to_log_range(PhyBlock, &LogStart, &LogEnde);
    for (i = 0; i < MS_PHYSICAL_BLOCKS_PER_SEGMENT; i++, PhyBlock++) {
    switch (ms_libconv_to_logical(info, PhyBlock)) {
    case MS_STATUS_ERROR:
    continue;
    default:
    break;
    }
    if (count == PhyBlock) {
    ms_lib_read_extrablock(us, PhyBlock, 0, 0x80,
    bbuf);
    count += 0x80;
    }
    index = (PhyBlock % 0x80) * 4;
    extdat.ovrflg = bbuf[index];
    extdat.mngflg = bbuf[index+1];
    extdat.logadr = memstick_logaddr(bbuf[index+2],
    bbuf[index+3]);
    if ((extdat.ovrflg & MS_REG_OVR_BKST) != MS_REG_OVR_BKST_OK) {
    ms_lib_setacquired_errorblock(us, PhyBlock);
    continue;
    }
    if ((extdat.mngflg & MS_REG_MNG_ATFLG) == MS_REG_MNG_ATFLG_ATTBL) {
    ms_lib_erase_phyblock(us, PhyBlock);
    continue;
    }
    if (extdat.logadr != MS_LB_NOT_USED) {
    if ((extdat.logadr < LogStart) || (LogEnde <= extdat.logadr)) {
    ms_lib_erase_phyblock(us, PhyBlock);
    continue;
    }
    newblk = ms_libconv_to_physical(info, extdat.logadr);
    if (newblk != MS_LB_NOT_USED) {
    if (extdat.logadr == 0) {
    ms_lib_set_logicalpair(us, extdat.logadr, PhyBlock);
    if (ms_lib_check_disableblock(us, btBlk1st)) {
    ms_lib_set_logicalpair(us, extdat.logadr, newblk);
    continue;
    }
    }
    ms_lib_read_extra(us, newblk, 0, &extdat);
    if ((extdat.ovrflg & MS_REG_OVR_UDST) == MS_REG_OVR_UDST_UPDATING) {
    ms_lib_erase_phyblock(us, PhyBlock);
    continue;
    } else {
    ms_lib_erase_phyblock(us, newblk);
    }
    }
    ms_lib_set_logicalpair(us, extdat.logadr, PhyBlock);
    }
    }
    } /* End for ... */
    return MS_STATUS_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn ms_scsi_read(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_read(struct us_data *us, struct scsi_cmnd *srb)
    {
    int result;
    unsigned char *cdb = srb.cmnd;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u32 bn = ((cdb[2] << 24) & 0xff000000) | ((cdb[3] << 16) & 0x00ff0000) |
    ((cdb[4] << 8) & 0x0000ff00) | ((cdb[5] << 0) & 0x000000ff);
    let mut blen: u16 = ((cdb[7] << 8) & 0xff00) | ((cdb[8] << 0) & 0x00ff);
    let mut blenByte: u32 = blen * 0x200;
    if (bn > info.bl_num)
    return USB_STOR_TRANSPORT_ERROR;
    if (info.MS_Status & MS_IsMSPro) {
    result = ene_load_bincode(us, MSP_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Load MPS RW pattern Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// set up the command wrapper
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = blenByte;
    bcb.Flags  = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF1;
    bcb.CDB[1] = 0x02;
    bcb.CDB[5] = (unsigned char)(bn);
    bcb.CDB[4] = (unsigned char)(bn>>8);
    bcb.CDB[3] = (unsigned char)(bn>>16);
    bcb.CDB[2] = (unsigned char)(bn>>24);
    result = ene_send_scsi_cmd(us, FDIR_READ, scsi_sglist(srb), 1);
    } else {
    void *buf;
    let mut offset: c_int = 0;
    u16 phyblk, logblk;
    u8 PageNum;
    u16 len;
    u32 blkno;
    buf = kmalloc(blenByte, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return USB_STOR_TRANSPORT_ERROR;
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    pr_info("Load MS RW pattern Fail !!\n");
    result = USB_STOR_TRANSPORT_ERROR;
    goto exit;
    }
    logblk  = (u16)(bn / info.MS_Lib.PagesPerBlock);
    PageNum = (u8)(bn % info.MS_Lib.PagesPerBlock);
    while (1) {
    if (blen > (info.MS_Lib.PagesPerBlock-PageNum))
    len = info.MS_Lib.PagesPerBlock-PageNum;
    else
    len = blen;
    phyblk = ms_libconv_to_physical(info, logblk);
    blkno  = phyblk * 0x20 + PageNum;
// set up the command wrapper
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200 * len;
    bcb.Flags  = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF1;
    bcb.CDB[1] = 0x02;
    bcb.CDB[5] = (unsigned char)(blkno);
    bcb.CDB[4] = (unsigned char)(blkno>>8);
    bcb.CDB[3] = (unsigned char)(blkno>>16);
    bcb.CDB[2] = (unsigned char)(blkno>>24);
    result = ene_send_scsi_cmd(us, FDIR_READ, buf+offset, 0);
    if (result != USB_STOR_XFER_GOOD) {
    pr_info("MS_SCSI_Read --- result = %x\n", result);
    result = USB_STOR_TRANSPORT_ERROR;
    goto exit;
    }
    blen -= len;
    if (blen <= 0)
    break;
    logblk++;
    PageNum = 0;
    offset += MS_BYTES_PER_PAGE*len;
    }
    usb_stor_set_xfer_buf(buf, blenByte, srb);
    exit:
    kfree(buf);
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ms_scsi_write(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_write(struct us_data *us, struct scsi_cmnd *srb)
    {
    int result;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    unsigned char *cdb = srb.cmnd;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u32 bn = ((cdb[2] << 24) & 0xff000000) |
    ((cdb[3] << 16) & 0x00ff0000) |
    ((cdb[4] << 8) & 0x0000ff00) |
    ((cdb[5] << 0) & 0x000000ff);
    let mut blen: u16 = ((cdb[7] << 8) & 0xff00) | ((cdb[8] << 0) & 0x00ff);
    let mut blenByte: u32 = blen * 0x200;
    if (bn > info.bl_num)
    return USB_STOR_TRANSPORT_ERROR;
    if (info.MS_Status & MS_IsMSPro) {
    result = ene_load_bincode(us, MSP_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    pr_info("Load MSP RW pattern Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// set up the command wrapper
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = blenByte;
    bcb.Flags  = US_BULK_FLAG_OUT;
    bcb.CDB[0] = 0xF0;
    bcb.CDB[1] = 0x04;
    bcb.CDB[5] = (unsigned char)(bn);
    bcb.CDB[4] = (unsigned char)(bn>>8);
    bcb.CDB[3] = (unsigned char)(bn>>16);
    bcb.CDB[2] = (unsigned char)(bn>>24);
    result = ene_send_scsi_cmd(us, FDIR_WRITE, scsi_sglist(srb), 1);
    } else {
    void *buf;
    let mut offset: c_int = 0;
    u16 PhyBlockAddr;
    u8 PageNum;
    u16 len, oldphy, newphy;
    buf = kmalloc(blenByte, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return USB_STOR_TRANSPORT_ERROR;
    usb_stor_set_xfer_buf(buf, blenByte, srb);
    result = ene_load_bincode(us, MS_RW_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    pr_info("Load MS RW pattern Fail !!\n");
    result = USB_STOR_TRANSPORT_ERROR;
    goto exit;
    }
    PhyBlockAddr = (u16)(bn / info.MS_Lib.PagesPerBlock);
    PageNum      = (u8)(bn % info.MS_Lib.PagesPerBlock);
    while (1) {
    if (blen > (info.MS_Lib.PagesPerBlock-PageNum))
    len = info.MS_Lib.PagesPerBlock-PageNum;
    else
    len = blen;
    oldphy = ms_libconv_to_physical(info, PhyBlockAddr); /* need check us <. info */
    newphy = ms_libsearch_block_from_logical(us, PhyBlockAddr);
    result = ms_read_copyblock(us, oldphy, newphy, PhyBlockAddr, PageNum, buf+offset, len);
    if (result != USB_STOR_XFER_GOOD) {
    pr_info("MS_SCSI_Write --- result = %x\n", result);
    result =  USB_STOR_TRANSPORT_ERROR;
    goto exit;
    }
    info.MS_Lib.Phy2LogMap[oldphy] = MS_LB_NOT_USED_ERASED;
    ms_lib_force_setlogical_pair(us, PhyBlockAddr, newphy);
    blen -= len;
    if (blen <= 0)
    break;
    PhyBlockAddr++;
    PageNum = 0;
    offset += MS_BYTES_PER_PAGE*len;
    }
    exit:
    kfree(buf);
    }
    return result;
    }
//
// ENE MS Card
//
#[no_mangle]
unsafe extern "C" fn ene_get_card_type(us: *mut us_data, index: u16, buf: *mut c_void) -> c_int {
    static int ene_get_card_type(struct us_data *us, u16 index, void *buf)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int result;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength	= 0x01;
    bcb.Flags			= US_BULK_FLAG_IN;
    bcb.CDB[0]			= 0xED;
    bcb.CDB[2]			= (unsigned char)(index>>8);
    bcb.CDB[3]			= (unsigned char)index;
    result = ene_send_scsi_cmd(us, FDIR_READ, buf, 0);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ene_get_card_status(us: *mut us_data, buf: *mut u8) -> c_int {
    static int ene_get_card_status(struct us_data *us, u8 *buf)
    {
    u16 tmpreg;
    u32 reg4b;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
// usb_stor_dbg(us, "transport --- ENE_ReadSDReg\n");
    reg4b = *(u32 *)&buf[0x18];
    info.SD_READ_BL_LEN = (u8)((reg4b >> 8) & 0x0f);
    tmpreg = (u16) reg4b;
    reg4b = *(u32 *)(&buf[0x14]);
    if ((info.SD_Status & SD_HiCapacity) && !(info.SD_Status & SD_IsMMC))
    info.HC_C_SIZE = (reg4b >> 8) & 0x3fffff;
    info.SD_C_SIZE = ((tmpreg & 0x03) << 10) | (u16)(reg4b >> 22);
    info.SD_C_SIZE_MULT = (u8)(reg4b >> 7)  & 0x07;
    if ((info.SD_Status & SD_HiCapacity) && (info.SD_Status & SD_IsMMC))
    info.HC_C_SIZE = *(u32 *)(&buf[0x100]);
    if (info.SD_READ_BL_LEN > SD_BLOCK_LEN) {
    info.SD_Block_Mult = 1 << (info.SD_READ_BL_LEN-SD_BLOCK_LEN);
    info.SD_READ_BL_LEN = SD_BLOCK_LEN;
    } else {
    info.SD_Block_Mult = 1;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ene_load_bincode(us: *mut us_data, flag: c_uchar) -> c_int {
    static int ene_load_bincode(struct us_data *us, unsigned char flag)
    {
    int err;
    char *fw_name = core::ptr::null_mut();
    unsigned char *buf = core::ptr::null_mut();
    const struct firmware *sd_fw = core::ptr::null_mut();
    let mut result: c_int = USB_STOR_TRANSPORT_ERROR;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    if (info.BIN_FLAG == flag)
    return USB_STOR_TRANSPORT_GOOD;
    switch (flag) {
// For SD
    case SD_INIT1_PATTERN:
    usb_stor_dbg(us, "SD_INIT1_PATTERN\n");
    fw_name = SD_INIT1_FIRMWARE;
    break;
    case SD_INIT2_PATTERN:
    usb_stor_dbg(us, "SD_INIT2_PATTERN\n");
    fw_name = SD_INIT2_FIRMWARE;
    break;
    case SD_RW_PATTERN:
    usb_stor_dbg(us, "SD_RW_PATTERN\n");
    fw_name = SD_RW_FIRMWARE;
    break;
// For MS
    case MS_INIT_PATTERN:
    usb_stor_dbg(us, "MS_INIT_PATTERN\n");
    fw_name = MS_INIT_FIRMWARE;
    break;
    case MSP_RW_PATTERN:
    usb_stor_dbg(us, "MSP_RW_PATTERN\n");
    fw_name = MSP_RW_FIRMWARE;
    break;
    case MS_RW_PATTERN:
    usb_stor_dbg(us, "MS_RW_PATTERN\n");
    fw_name = MS_RW_FIRMWARE;
    break;
    default:
    usb_stor_dbg(us, "----------- Unknown PATTERN ----------\n");
    goto nofw;
    }
    err = request_firmware(&sd_fw, fw_name, &us.pusb_dev.dev);
    if (err) {
    usb_stor_dbg(us, "load firmware %s failed\n", fw_name);
    goto nofw;
    }
    buf = kmemdup(sd_fw.data, sd_fw.size, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    goto nofw;
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = sd_fw.size;
    bcb.Flags = US_BULK_FLAG_OUT;
    bcb.CDB[0] = 0xEF;
    result = ene_send_scsi_cmd(us, FDIR_WRITE, buf, 0);
    if (us.srb != core::ptr::null_mut())
    scsi_set_resid(us.srb, 0);
    info.BIN_FLAG = flag;
    kfree(buf);
    nofw:
    release_firmware(sd_fw);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ms_card_init(us: *mut us_data) -> c_int {
    static int ms_card_init(struct us_data *us)
    {
    u32 result;
    u16 TmpBlock;
    unsigned char *PageBuffer0 = core::ptr::null_mut(), *PageBuffer1 = core::ptr::null_mut();
    struct ms_lib_type_extdat extdat;
    u16 btBlk1st, btBlk2nd;
    u32 btBlk1stErred;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    printk(KERN_INFO "MS_CardInit start\n");
    ms_lib_free_allocatedarea(us); /* Clean buffer and set struct us_data flag to 0 */
// get two PageBuffer
    PageBuffer0 = kmalloc(MS_BYTES_PER_PAGE, GFP_KERNEL);
    PageBuffer1 = kmalloc(MS_BYTES_PER_PAGE, GFP_KERNEL);
    if ((PageBuffer0 == core::ptr::null_mut()) || (PageBuffer1 == core::ptr::null_mut())) {
    result = MS_NO_MEMORY_ERROR;
    goto exit;
    }
    btBlk1st = btBlk2nd = MS_LB_NOT_USED;
    btBlk1stErred = 0;
    for (TmpBlock = 0; TmpBlock < MS_MAX_INITIAL_ERROR_BLOCKS+2; TmpBlock++) {
    switch (ms_read_readpage(us, TmpBlock, 0, (u32 *)PageBuffer0, &extdat)) {
    case MS_STATUS_SUCCESS:
    break;
    case MS_STATUS_INT_ERROR:
    break;
    case MS_STATUS_ERROR:
    default:
    continue;
    }
    if ((extdat.ovrflg & MS_REG_OVR_BKST) == MS_REG_OVR_BKST_NG)
    continue;
    if (((extdat.mngflg & MS_REG_MNG_SYSFLG) == MS_REG_MNG_SYSFLG_USER) ||
    (be16_to_cpu(((struct ms_bootblock_page0 *)PageBuffer0).header.wBlockID) != MS_BOOT_BLOCK_ID) ||
    (be16_to_cpu(((struct ms_bootblock_page0 *)PageBuffer0).header.wFormatVersion) != MS_BOOT_BLOCK_FORMAT_VERSION) ||
    (((struct ms_bootblock_page0 *)PageBuffer0).header.bNumberOfDataEntry != MS_BOOT_BLOCK_DATA_ENTRIES))
    continue;
    if (btBlk1st != MS_LB_NOT_USED) {
    btBlk2nd = TmpBlock;
    break;
    }
    btBlk1st = TmpBlock;
    memcpy(PageBuffer1, PageBuffer0, MS_BYTES_PER_PAGE);
    if (extdat.status1 & (MS_REG_ST1_DTER | MS_REG_ST1_EXER | MS_REG_ST1_FGER))
    btBlk1stErred = 1;
    }
    if (btBlk1st == MS_LB_NOT_USED) {
    result = MS_STATUS_ERROR;
    goto exit;
    }
// write protect
    if ((extdat.status0 & MS_REG_ST0_WP) == MS_REG_ST0_WP_ON)
    ms_lib_ctrl_set(info, MS_LIB_CTRL_WRPROTECT);
    result = MS_STATUS_ERROR;
// 1st Boot Block
    if (btBlk1stErred == 0)
    result = ms_lib_process_bootblock(us, btBlk1st, PageBuffer1);
// 1st
// 2nd Boot Block
    if (result && (btBlk2nd != MS_LB_NOT_USED))
    result = ms_lib_process_bootblock(us, btBlk2nd, PageBuffer0);
    if (result) {
    result = MS_STATUS_ERROR;
    goto exit;
    }
    for (TmpBlock = 0; TmpBlock < btBlk1st; TmpBlock++)
    info.MS_Lib.Phy2LogMap[TmpBlock] = MS_LB_INITIAL_ERROR;
    info.MS_Lib.Phy2LogMap[btBlk1st] = MS_LB_BOOT_BLOCK;
    if (btBlk2nd != MS_LB_NOT_USED) {
    for (TmpBlock = btBlk1st + 1; TmpBlock < btBlk2nd; TmpBlock++)
    info.MS_Lib.Phy2LogMap[TmpBlock] = MS_LB_INITIAL_ERROR;
    info.MS_Lib.Phy2LogMap[btBlk2nd] = MS_LB_BOOT_BLOCK;
    }
    result = ms_lib_scan_logicalblocknumber(us, btBlk1st);
    if (result)
    goto exit;
    for (TmpBlock = MS_PHYSICAL_BLOCKS_PER_SEGMENT;
    TmpBlock < info.MS_Lib.NumberOfPhyBlock;
    TmpBlock += MS_PHYSICAL_BLOCKS_PER_SEGMENT) {
    if (ms_count_freeblock(us, TmpBlock) == 0) {
    ms_lib_ctrl_set(info, MS_LIB_CTRL_WRPROTECT);
    break;
    }
    }
// write
    if (ms_lib_alloc_writebuf(us)) {
    result = MS_NO_MEMORY_ERROR;
    goto exit;
    }
    result = MS_STATUS_SUCCESS;
    exit:
    kfree(PageBuffer1);
    kfree(PageBuffer0);
    printk(KERN_INFO "MS_CardInit end\n");
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ene_ms_init(us: *mut us_data) -> c_int {
    static int ene_ms_init(struct us_data *us)
    {
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    int result;
    u16 MSP_BlockSize, MSP_UserAreaBlocks;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u8 *bbuf = info.bbuf;
    unsigned int s;
    printk(KERN_INFO "transport --- ENE_MSInit\n");
// the same part to test ENE
    result = ene_load_bincode(us, MS_INIT_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    printk(KERN_ERR "Load MS Init Code Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200;
    bcb.Flags      = US_BULK_FLAG_IN;
    bcb.CDB[0]     = 0xF1;
    bcb.CDB[1]     = 0x01;
    result = ene_send_scsi_cmd(us, FDIR_READ, bbuf, 0);
    if (result != USB_STOR_XFER_GOOD) {
    printk(KERN_ERR "Execution MS Init Code Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// the same part to test ENE
    info.MS_Status = bbuf[0];
    s = info.MS_Status;
    if ((s & MS_Insert) && (s & MS_Ready)) {
    printk(KERN_INFO "Insert     = %x\n", !!(s & MS_Insert));
    printk(KERN_INFO "Ready      = %x\n", !!(s & MS_Ready));
    printk(KERN_INFO "IsMSPro    = %x\n", !!(s & MS_IsMSPro));
    printk(KERN_INFO "IsMSPHG    = %x\n", !!(s & MS_IsMSPHG));
    printk(KERN_INFO "WtP= %x\n", !!(s & MS_WtP));
    if (s & MS_IsMSPro) {
    MSP_BlockSize      = (bbuf[6] << 8) | bbuf[7];
    MSP_UserAreaBlocks = (bbuf[10] << 8) | bbuf[11];
    info.MSP_TotalBlock = MSP_BlockSize * MSP_UserAreaBlocks;
    } else {
    ms_card_init(us); /* Card is MS (to ms.c)*/
    }
    usb_stor_dbg(us, "MS Init Code OK !!\n");
    } else {
    usb_stor_dbg(us, "MS Card Not Ready --- %x\n", bbuf[0]);
    return USB_STOR_TRANSPORT_ERROR;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ene_sd_init(us: *mut us_data) -> c_int {
    static int ene_sd_init(struct us_data *us)
    {
    int result;
    struct bulk_cb_wrap *bcb = (struct bulk_cb_wrap *) us.iobuf;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *) us.extra;
    u8 *bbuf = info.bbuf;
    usb_stor_dbg(us, "transport --- ENE_SDInit\n");
// SD Init Part-1
    result = ene_load_bincode(us, SD_INIT1_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Load SD Init Code Part-1 Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.Flags = US_BULK_FLAG_IN;
    bcb.CDB[0] = 0xF2;
    result = ene_send_scsi_cmd(us, FDIR_READ, core::ptr::null_mut(), 0);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Execution SD Init Code Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
// SD Init Part-2
    result = ene_load_bincode(us, SD_INIT2_PATTERN);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Load SD Init Code Part-2 Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    memset(bcb, 0, sizeof(struct bulk_cb_wrap));
    bcb.Signature = cpu_to_le32(US_BULK_CB_SIGN);
    bcb.DataTransferLength = 0x200;
    bcb.Flags              = US_BULK_FLAG_IN;
    bcb.CDB[0]             = 0xF1;
    result = ene_send_scsi_cmd(us, FDIR_READ, bbuf, 0);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_dbg(us, "Execution SD Init Code Fail !!\n");
    return USB_STOR_TRANSPORT_ERROR;
    }
    info.SD_Status = bbuf[0];
    if ((info.SD_Status & SD_Insert) && (info.SD_Status & SD_Ready)) {
    let mut s: c_uint = info.SD_Status;
    ene_get_card_status(us, bbuf);
    usb_stor_dbg(us, "Insert     = %x\n", !!(s & SD_Insert));
    usb_stor_dbg(us, "Ready      = %x\n", !!(s & SD_Ready));
    usb_stor_dbg(us, "IsMMC      = %x\n", !!(s & SD_IsMMC));
    usb_stor_dbg(us, "HiCapacity = %x\n", !!(s & SD_HiCapacity));
    usb_stor_dbg(us, "HiSpeed    = %x\n", !!(s & SD_HiSpeed));
    usb_stor_dbg(us, "WtP        = %x\n", !!(s & SD_WtP));
    } else {
    usb_stor_dbg(us, "SD Card Not Ready --- %x\n", bbuf[0]);
    return USB_STOR_TRANSPORT_ERROR;
    }
    return USB_STOR_TRANSPORT_GOOD;
    }
#[no_mangle]
unsafe extern "C" fn ene_init(us: *mut us_data) -> c_int {
    static int ene_init(struct us_data *us)
    {
    int result;
    u8  misc_reg03;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)(us.extra);
    u8 *bbuf = info.bbuf;
    result = ene_get_card_type(us, REG_CARD_STATUS, bbuf);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    misc_reg03 = bbuf[0];
    if (misc_reg03 & 0x01) {
    if (!(info.SD_Status & SD_Ready)) {
    result = ene_sd_init(us);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    }
    }
    if (misc_reg03 & 0x02) {
    if (!(info.MS_Status & MS_Ready)) {
    result = ene_ms_init(us);
    if (result != USB_STOR_XFER_GOOD)
    return USB_STOR_TRANSPORT_ERROR;
    }
    }
    return result;
    }
// ----- sd_scsi_irp() ---------
#[no_mangle]
unsafe extern "C" fn sd_scsi_irp(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int sd_scsi_irp(struct us_data *us, struct scsi_cmnd *srb)
    {
    int    result;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)us.extra;
    switch (srb.cmnd[0]) {
    case TEST_UNIT_READY:
    result = sd_scsi_test_unit_ready(us, srb);
    break; /* 0x00 */
    case REQUEST_SENSE:
    result = do_scsi_request_sense(us, srb);
    break; /* 0x03 */
    case INQUIRY:
    result = do_scsi_inquiry(us, srb);
    break; /* 0x12 */
    case MODE_SENSE:
    result = sd_scsi_mode_sense(us, srb);
    break; /* 0x1A */
//
    case START_STOP:
    result = SD_SCSI_Start_Stop(us, srb);
    break; //0x1B
//
    case READ_CAPACITY:
    result = sd_scsi_read_capacity(us, srb);
    break; /* 0x25 */
    case READ_10:
    result = sd_scsi_read(us, srb);
    break; /* 0x28 */
    case WRITE_10:
    result = sd_scsi_write(us, srb);
    break; /* 0x2A */
    default:
    info.SrbStatus = SS_ILLEGAL_REQUEST;
    result = USB_STOR_TRANSPORT_FAILED;
    break;
    }
    if (result == USB_STOR_TRANSPORT_GOOD)
    info.SrbStatus = SS_SUCCESS;
    return result;
    }
//
// ms_scsi_irp()
//
#[no_mangle]
unsafe extern "C" fn ms_scsi_irp(us: *mut us_data, srb: *mut scsi_cmnd) -> c_int {
    static int ms_scsi_irp(struct us_data *us, struct scsi_cmnd *srb)
    {
    int result;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)us.extra;
    switch (srb.cmnd[0]) {
    case TEST_UNIT_READY:
    result = ms_scsi_test_unit_ready(us, srb);
    break; /* 0x00 */
    case REQUEST_SENSE:
    result = do_scsi_request_sense(us, srb);
    break; /* 0x03 */
    case INQUIRY:
    result = do_scsi_inquiry(us, srb);
    break; /* 0x12 */
    case MODE_SENSE:
    result = ms_scsi_mode_sense(us, srb);
    break; /* 0x1A */
    case READ_CAPACITY:
    result = ms_scsi_read_capacity(us, srb);
    break; /* 0x25 */
    case READ_10:
    result = ms_scsi_read(us, srb);
    break; /* 0x28 */
    case WRITE_10:
    result = ms_scsi_write(us, srb);
    break;  /* 0x2A */
    default:
    info.SrbStatus = SS_ILLEGAL_REQUEST;
    result = USB_STOR_TRANSPORT_FAILED;
    break;
    }
    if (result == USB_STOR_TRANSPORT_GOOD)
    info.SrbStatus = SS_SUCCESS;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ene_transport(srb: *mut scsi_cmnd, us: *mut us_data) -> c_int {
    static int ene_transport(struct scsi_cmnd *srb, struct us_data *us)
    {
    let mut result: c_int = USB_STOR_XFER_GOOD;
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)(us.extra);
// US_DEBUG(usb_stor_show_command(us, srb));
    scsi_set_resid(srb, 0);
    if (unlikely(!(info.SD_Status & SD_Ready) &&
    !(info.MS_Status & MS_Ready)))
    result = ene_init(us);
    if (result == USB_STOR_XFER_GOOD) {
    result = USB_STOR_TRANSPORT_ERROR;
    if (info.SD_Status & SD_Ready)
    result = sd_scsi_irp(us, srb);
    if (info.MS_Status & MS_Ready)
    result = ms_scsi_irp(us, srb);
    }
    return result;
    }
    static struct scsi_host_template ene_ub6250_host_template;
    static int ene_ub6250_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    int result;
    u8  misc_reg03;
    struct us_data *us;
    struct ene_ub6250_info *info;
    result = usb_stor_probe1(&us, intf, id,
    (id - ene_ub6250_usb_ids) + ene_ub6250_unusual_dev_list,
    &ene_ub6250_host_template);
    if (result)
    return result;
// FIXME: where should the code alloc extra buf ?
    us.extra = kzalloc_obj(struct ene_ub6250_info);
    if (!us.extra)
    return -ENOMEM;
    us.extra_destructor = ene_ub6250_info_destructor;
    info = (struct ene_ub6250_info *)(us.extra);
    info.bbuf = kmalloc(512, GFP_KERNEL);
    if (!info.bbuf) {
    kfree(us.extra);
    return -ENOMEM;
    }
    us.transport_name = "ene_ub6250";
    us.transport = ene_transport;
    us.max_lun = 0;
    result = usb_stor_probe2(us);
    if (result)
    return result;
// probe card type
    result = ene_get_card_type(us, REG_CARD_STATUS, info.bbuf);
    if (result != USB_STOR_XFER_GOOD) {
    usb_stor_disconnect(intf);
    return USB_STOR_TRANSPORT_ERROR;
    }
    misc_reg03 = info.bbuf[0];
    if (!(misc_reg03 & 0x01)) {
    pr_info("ums_eneub6250: This driver only supports SD/MS cards. "
    "It does not support SM cards.\n");
    }
    return result;
    }

#[no_mangle]
unsafe extern "C" fn ene_ub6250_resume(iface: *mut usb_interface) -> c_int {
    static int ene_ub6250_resume(struct usb_interface *iface)
    {
    struct us_data *us = usb_get_intfdata(iface);
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)(us.extra);
    mutex_lock(&us.dev_mutex);
    if (us.suspend_resume_hook)
    (us.suspend_resume_hook)(us, US_RESUME);
    mutex_unlock(&us.dev_mutex);
    info.Power_IsResum = true;
// info->SD_Status &= ~SD_Ready;
    info.SD_Status = 0;
    info.MS_Status = 0;
    info.SM_Status = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ene_ub6250_reset_resume(iface: *mut usb_interface) -> c_int {
    static int ene_ub6250_reset_resume(struct usb_interface *iface)
    {
    struct us_data *us = usb_get_intfdata(iface);
    struct ene_ub6250_info *info = (struct ene_ub6250_info *)(us.extra);
// Report the reset to the SCSI core
    usb_stor_reset_resume(iface);
//
// FIXME: Notify the subdrivers that they need to reinitialize
// the device
//
    info.Power_IsResum = true;
// info->SD_Status &= ~SD_Ready;
    info.SD_Status = 0;
    info.MS_Status = 0;
    info.SM_Status = 0;
    return 0;
    }

    static struct usb_driver ene_ub6250_driver = {
    .name =		DRV_NAME,
    .probe =	ene_ub6250_probe,
    .disconnect =	usb_stor_disconnect,
    .suspend =	usb_stor_suspend,
    .resume =	ene_ub6250_resume,
    .reset_resume =	ene_ub6250_reset_resume,
    .pre_reset =	usb_stor_pre_reset,
    .post_reset =	usb_stor_post_reset,
    .id_table =	ene_ub6250_usb_ids,
    .soft_unbind =	1,
    .no_dynamic_id = 1,
    };
    module_usb_stor_driver(ene_ub6250_driver, ene_ub6250_host_template, DRV_NAME);
