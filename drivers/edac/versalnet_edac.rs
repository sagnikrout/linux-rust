//! Automatically rewritten from C to Rust
//! Source: drivers/edac/versalnet_edac.c
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
// AMD Versal NET memory controller driver
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

// Granularity of reported error in bytes
pub const MC5_ERR_GRAIN: c_int = 1;
pub const MC_GET_DDR_CONFIG_IN_LEN: c_int = 4;

pub const MC5_REGHI_ROW: c_int = 7;
pub const MC5_EACHBIT: c_int = 1;
pub const MC5_ERR_TYPE_CE: c_int = 0;
pub const MC5_ERR_TYPE_UE: c_int = 1;

pub const MC5_X16_BASE: c_int = 256;
pub const MC5_X16_ECC: c_int = 32;

pub const MC5_X32_SIZE: c_int = 576;

pub const ERROR_LEVEL: c_int = 2;
pub const ERROR_ID: c_int = 3;
pub const TOTAL_ERR_LENGTH: c_int = 5;
pub const MSG_ERR_OFFSET: c_int = 8;
pub const MSG_ERR_LENGTH: c_int = 9;
pub const ERROR_DATA: c_int = 10;
pub const MCDI_RESPONSE: c_uint = 0xFF;
pub const REG_MAX: c_int = 152;
pub const ADEC_MAX: c_int = 152;
pub const NUM_CONTROLLERS: c_int = 8;
pub const REGS_PER_CONTROLLER: c_int = 19;
pub const ADEC_NUM: c_int = 19;
pub const BUFFER_SZ: c_int = 80;
pub const XDDR5_BUS_WIDTH_64: c_int = 0;
pub const XDDR5_BUS_WIDTH_32: c_int = 1;
pub const XDDR5_BUS_WIDTH_16: c_int = 2;
pub const MC_NAME_LEN: c_int = 32;
//
// struct ecc_error_info - ECC error log information.
// @burstpos:		Burst position.
// @lrank:		Logical Rank number.
// @rank:		Rank number.
// @group:		Group number.
// @bank:		Bank number.
// @col:		Column number.
// @row:		Row number.
// @rowhi:		Row number higher bits.
// @i:			Combined ECC error vector containing encoded values of burst position,
// rank, bank, column, and row information.
//
    union ecc_error_info {
    struct {
    u32 burstpos:3;
    u32 lrank:4;
    u32 rank:2;
    u32 group:3;
    u32 bank:2;
    u32 col:11;
    u32 row:7;
    u32 rowhi;
    };
    u64 i;
    } __packed;
// Row and column bit positions in the address decoder (ADEC) registers.
    union row_col_mapping {
    struct {
    u32 row0:6;
    u32 row1:6;
    u32 row2:6;
    u32 row3:6;
    u32 row4:6;
    u32 reserved:2;
    };
    struct {
    u32 col1:6;
    u32 col2:6;
    u32 col3:6;
    u32 col4:6;
    u32 col5:6;
    u32 reservedcol:2;
    };
    u32 i;
    } __packed;
//
// struct ecc_status - ECC status information to report.
// @ceinfo:	Correctable errors.
// @ueinfo:	Uncorrected errors.
// @channel:	Channel number.
// @error_type:	Error type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_status {
    pub ceinfo: [union ecc_error_info; 2],
    pub ueinfo: [union ecc_error_info; 2],
    pub channel: u8,
    pub error_type: u8,
}

//
// struct mc_priv - DDR memory controller private instance data.
// @message:		Buffer for framing the event specific info.
// @stat:		ECC status information.
// @error_id:		The error id.
// @error_level:	The error level.
// @dwidth:		Width of data bus excluding ECC bits.
// @part_len:		The support of the message received.
// @regs:		The registers sent on the rpmsg.
// @adec:		Address decode registers.
// @mci:		Memory controller interface.
// @ept:		rpmsg endpoint.
// @mcdi:		The mcdi handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_priv {
    pub message: [c_char; 256],
    pub stat: ecc_status,
    pub error_id: u32,
    pub error_level: u32,
    pub dwidth: u32,
    pub part_len: u32,
    pub regs: [u32; REG_MAX],
    pub adec: [u32; ADEC_MAX],
    pub mci: [*mut mem_ctl_info; NUM_CONTROLLERS],
    pub ept: *mut rpmsg_endpoint,
    pub mcdi: *mut cdx_mcdi,
}

//
// Address decoder (ADEC) registers to match the order in which the register
// information is received from the firmware.
//
    enum adec_info {
    CONF = 0,
    ADEC0,
    ADEC1,
    ADEC2,
    ADEC3,
    ADEC4,
    ADEC5,
    ADEC6,
    ADEC7,
    ADEC8,
    ADEC9,
    ADEC10,
    ADEC11,
    ADEC12,
    ADEC13,
    ADEC14,
    ADEC15,
    ADEC16,
    ADECILC,
    };
    enum reg_info {
    ISR = 0,
    IMR,
    ECCR0_ERR_STATUS,
    ECCR0_ADDR_LO,
    ECCR0_ADDR_HI,
    ECCR0_DATA_LO,
    ECCR0_DATA_HI,
    ECCR0_PAR,
    ECCR1_ERR_STATUS,
    ECCR1_ADDR_LO,
    ECCR1_ADDR_HI,
    ECCR1_DATA_LO,
    ECCR1_DATA_HI,
    ECCR1_PAR,
    XMPU_ERR,
    XMPU_ERR_ADDR_L0,
    XMPU_ERR_ADDR_HI,
    XMPU_ERR_AXI_ID,
    ADEC_CHK_ERR_LOG,
    };
#[no_mangle]
unsafe extern "C" fn get_ddr_info(error_data: *mut u32, priv: *mut mc_priv) -> bool {
    static bool get_ddr_info(u32 *error_data, struct mc_priv *priv)
    {
    u32 reglo, reghi, parity, eccr0_val, eccr1_val, isr;
    struct ecc_status *p;
    isr = error_data[ISR];
    if (!(isr & (MC5_IRQ_UE_MASK | MC5_IRQ_CE_MASK)))
    return false;
    eccr0_val = error_data[ECCR0_ERR_STATUS];
    eccr1_val = error_data[ECCR1_ERR_STATUS];
    if (!eccr0_val && !eccr1_val)
    return false;
    p = &priv.stat;
    if (!eccr0_val)
    p.channel = 1;
    else
    p.channel = 0;
    reglo = error_data[ECCR0_ADDR_LO];
    reghi = error_data[ECCR0_ADDR_HI];
    if (isr & MC5_IRQ_CE_MASK)
    p.ceinfo[0].i = reglo | (u64)reghi << 32;
#[no_mangle]
pub unsafe extern "C" fn if(MC5_IRQ_UE_MASK: isr &) -> else {
    else if (isr & MC5_IRQ_UE_MASK)
    p.ueinfo[0].i = reglo | (u64)reghi << 32;
    parity = error_data[ECCR0_PAR];
    edac_dbg(2, "ERR DATA: 0x%08X%08X PARITY: 0x%08X\n",
    reghi, reglo, parity);
    reglo = error_data[ECCR1_ADDR_LO];
    reghi = error_data[ECCR1_ADDR_HI];
    if (isr & MC5_IRQ_CE_MASK)
    p.ceinfo[1].i = reglo | (u64)reghi << 32;
#[no_mangle]
pub unsafe extern "C" fn if(MC5_IRQ_UE_MASK: isr &) -> else {
    else if (isr & MC5_IRQ_UE_MASK)
    p.ueinfo[1].i = reglo | (u64)reghi << 32;
    parity = error_data[ECCR1_PAR];
    edac_dbg(2, "ERR DATA: 0x%08X%08X PARITY: 0x%08X\n",
    reghi, reglo, parity);
    return true;
    }
//
// convert_to_physical - Convert @error_data to a physical address.
// @priv:	DDR memory controller private instance data.
// @pinf:	ECC error info structure.
// @controller:	Controller number of the MC5
// @error_data:	the DDRMC5 ADEC address decoder register data
//
// Return: physical address of the DDR memory.
//
    static unsigned long convert_to_physical(struct mc_priv *priv,
    union ecc_error_info pinf,
    int controller, int *error_data)
    {
    u32 row, blk, rsh_req_addr, interleave, ilc_base_ctrl_add, ilc_himem_en, reg, offset;
    u64 high_mem_base, high_mem_offset, low_mem_offset, ilcmem_base;
    let mut err_addr: c_ulong = 0, addr;
    union row_col_mapping cols;
    union row_col_mapping rows;
    u32 col_bit_0;
    row = pinf.rowhi << MC5_REGHI_ROW | pinf.row;
    offset = controller * ADEC_NUM;
    reg = error_data[ADEC6];
    rows.i = reg;
    err_addr |= (row & BIT(0)) << rows.row0;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row1;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row2;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row3;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row4;
    row >>= MC5_EACHBIT;
    reg = error_data[ADEC7];
    rows.i = reg;
    err_addr |= (row & BIT(0)) << rows.row0;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row1;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row2;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row3;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row4;
    row >>= MC5_EACHBIT;
    reg = error_data[ADEC8];
    rows.i = reg;
    err_addr |= (row & BIT(0)) << rows.row0;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row1;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row2;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row3;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row4;
    reg = error_data[ADEC9];
    rows.i = reg;
    err_addr |= (row & BIT(0)) << rows.row0;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row1;
    row >>= MC5_EACHBIT;
    err_addr |= (row & BIT(0)) << rows.row2;
    row >>= MC5_EACHBIT;
    col_bit_0 = FIELD_GET(MASK_24, error_data[ADEC9]);
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << col_bit_0;
    cols.i = error_data[ADEC10];
    err_addr |= (pinf.col & 1) << cols.col1;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col2;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col3;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col4;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col5;
    pinf.col >>= 1;
    cols.i = error_data[ADEC11];
    err_addr |= (pinf.col & 1) << cols.col1;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col2;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col3;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col4;
    pinf.col >>= 1;
    err_addr |= (pinf.col & 1) << cols.col5;
    pinf.col >>= 1;
    reg = error_data[ADEC12];
    err_addr |= (pinf.bank & BIT(0)) << (reg & MASK_0);
    pinf.bank >>= MC5_EACHBIT;
    err_addr |= (pinf.bank & BIT(0)) << FIELD_GET(MC5_BANK1_MASK, reg);
    pinf.bank >>= MC5_EACHBIT;
    err_addr |= (pinf.bank & BIT(0)) << FIELD_GET(MC5_GRP_0_MASK, reg);
    pinf.group >>= MC5_EACHBIT;
    err_addr |= (pinf.bank & BIT(0)) << FIELD_GET(MC5_GRP_1_MASK, reg);
    pinf.group >>= MC5_EACHBIT;
    err_addr |= (pinf.bank & BIT(0)) << FIELD_GET(MASK_24, reg);
    pinf.group >>= MC5_EACHBIT;
    reg = error_data[ADEC4];
    err_addr |= (pinf.rank & BIT(0)) << (reg & MASK_0);
    pinf.rank >>= MC5_EACHBIT;
    err_addr |= (pinf.rank & BIT(0)) << FIELD_GET(MC5_RANK_1_MASK, reg);
    pinf.rank >>= MC5_EACHBIT;
    reg = error_data[ADEC5];
    err_addr |= (pinf.lrank & BIT(0)) << (reg & MASK_0);
    pinf.lrank >>= MC5_EACHBIT;
    err_addr |= (pinf.lrank & BIT(0)) << FIELD_GET(MC5_LRANK_1_MASK, reg);
    pinf.lrank >>= MC5_EACHBIT;
    err_addr |= (pinf.lrank & BIT(0)) << FIELD_GET(MC5_LRANK_2_MASK, reg);
    pinf.lrank >>= MC5_EACHBIT;
    err_addr |= (pinf.lrank & BIT(0)) << FIELD_GET(MASK_24, reg);
    pinf.lrank >>= MC5_EACHBIT;
    high_mem_base = (priv.adec[ADEC2 + offset] & MC5_MEM_MASK) * MC5_HIMEM_BASE;
    interleave = priv.adec[ADEC13 + offset] & MC5_INTERLEAVE_SEL;
    high_mem_offset = priv.adec[ADEC3 + offset] & MC5_MEM_MASK;
    low_mem_offset = priv.adec[ADEC1 + offset] & MC5_MEM_MASK;
    reg = priv.adec[ADEC14 + offset];
    ilc_himem_en = !!(reg & MC5_ILC_HIMEM_EN);
    ilcmem_base = (reg & MC5_ILC_MEM) * SZ_1M;
    if (ilc_himem_en)
    ilc_base_ctrl_add = ilcmem_base - high_mem_offset;
    else
    ilc_base_ctrl_add = ilcmem_base - low_mem_offset;
    if (priv.dwidth == DEV_X16) {
    blk = err_addr / MC5_X16_SIZE;
    rsh_req_addr = (blk << 8) + ilc_base_ctrl_add;
    err_addr = rsh_req_addr * interleave * 2;
    } else {
    blk = err_addr / MC5_X32_SIZE;
    rsh_req_addr = (blk << 9) + ilc_base_ctrl_add;
    err_addr = rsh_req_addr * interleave * 2;
    }
    if ((priv.adec[ADEC2 + offset] & MC5_HIGH_MEM_EN) && err_addr >= high_mem_base)
    addr = err_addr - high_mem_offset;
    else
    addr = err_addr - low_mem_offset;
    return addr;
    }
//
// handle_error - Handle errors.
// @priv:	DDR memory controller private instance data.
// @stat:	ECC status structure.
// @ctl_num:	Controller number of the MC5
// @error_data:	the MC5 ADEC address decoder register data
//
// Handles ECC correctable and uncorrectable errors.
//
    static void handle_error(struct mc_priv  *priv, struct ecc_status *stat,
    int ctl_num, int *error_data)
    {
    union ecc_error_info pinf;
    struct mem_ctl_info *mci;
    unsigned long pa;
    phys_addr_t pfn;
    int err;
    if (WARN_ON_ONCE(ctl_num >= NUM_CONTROLLERS))
    return;
    mci = priv.mci[ctl_num];
    if (stat.error_type == MC5_ERR_TYPE_CE) {
    pinf = stat.ceinfo[stat.channel];
    snprintf(priv.message, sizeof(priv.message),
    "Error type:%s Controller %d Addr at %lx\n",
    "CE", ctl_num, convert_to_physical(priv, pinf, ctl_num, error_data));
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci,
    1, 0, 0, 0, 0, 0, -1,
    priv.message, "");
    }
    if (stat.error_type == MC5_ERR_TYPE_UE) {
    pinf = stat.ueinfo[stat.channel];
    snprintf(priv.message, sizeof(priv.message),
    "Error type:%s controller %d Addr at %lx\n",
    "UE", ctl_num, convert_to_physical(priv, pinf, ctl_num, error_data));
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci,
    1, 0, 0, 0, 0, 0, -1,
    priv.message, "");
    pa = convert_to_physical(priv, pinf, ctl_num, error_data);
    pfn = PHYS_PFN(pa);
    if (IS_ENABLED(CONFIG_MEMORY_FAILURE)) {
    err = memory_failure(pfn, MF_ACTION_REQUIRED);
    if (err)
    edac_dbg(2, "memory_failure() error: %d", err);
    else
    edac_dbg(2, "Poison page at PA 0x%lx\n", pa);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mc_init(mci: *mut mem_ctl_info, dev: *mut device) {
    static void mc_init(struct mem_ctl_info *mci, struct device *dev)
    {
    struct mc_priv *priv = mci.pvt_info;
    struct csrow_info *csi;
    struct dimm_info *dimm;
    u32 row;
    int ch;
// Initialize controller capabilities and configuration
    mci.mtype_cap = MEM_FLAG_DDR5;
    mci.edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED;
    mci.scrub_cap = SCRUB_HW_SRC;
    mci.scrub_mode = SCRUB_NONE;
    mci.edac_cap = EDAC_FLAG_SECDED;
    mci.ctl_name = "VersalNET DDR5";
    mci.dev_name = dev_name(dev);
    mci.mod_name = "versalnet_edac";
    edac_op_state = EDAC_OPSTATE_INT;
    for (row = 0; row < mci.nr_csrows; row++) {
    csi = mci.csrows[row];
    for (ch = 0; ch < csi.nr_channels; ch++) {
    dimm = csi.channels[ch].dimm;
    dimm.edac_mode = EDAC_SECDED;
    dimm.mtype = MEM_DDR5;
    dimm.grain = MC5_ERR_GRAIN;
    dimm.dtype = priv.dwidth;
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn mcdi_rpc_timeout(cdx: *mut cdx_mcdi, cmd: c_uint) -> c_uint {
    static unsigned int mcdi_rpc_timeout(struct cdx_mcdi *cdx, unsigned int cmd)
    {
    return MCDI_RPC_TIMEOUT;
    }
    static void mcdi_request(struct cdx_mcdi *cdx,
    const struct cdx_dword *hdr, size_t hdr_len,
    const struct cdx_dword *sdu, size_t sdu_len)
    {
    void *send_buf;
    int ret;
    send_buf = kzalloc(hdr_len + sdu_len, GFP_KERNEL);
    if (!send_buf)
    return;
    memcpy(send_buf, hdr, hdr_len);
    memcpy(send_buf + hdr_len, sdu, sdu_len);
    ret = rpmsg_send(cdx.ept, send_buf, hdr_len + sdu_len);
    if (ret)
    dev_err(&cdx.rpdev.dev, "Failed to send rpmsg data: %d\n", ret);
    kfree(send_buf);
    }
    static const struct cdx_mcdi_ops mcdi_ops = {
    .mcdi_rpc_timeout = mcdi_rpc_timeout,
    .mcdi_request = mcdi_request,
    };
#[no_mangle]
unsafe extern "C" fn get_ddr_config(index: u32, buffer: *mut u32, amd_mcdi: *mut cdx_mcdi) {
    static void get_ddr_config(u32 index, u32 *buffer, struct cdx_mcdi *amd_mcdi)
    {
    size_t outlen;
    int ret;
    MCDI_DECLARE_BUF(inbuf, MC_GET_DDR_CONFIG_IN_LEN);
    MCDI_DECLARE_BUF(outbuf, BUFFER_SZ);
    MCDI_SET_DWORD(inbuf, EDAC_GET_DDR_CONFIG_IN_CONTROLLER_INDEX, index);
    ret = cdx_mcdi_rpc(amd_mcdi, MC_CMD_EDAC_GET_DDR_CONFIG, inbuf, sizeof(inbuf),
    outbuf, sizeof(outbuf), &outlen);
    if (!ret)
    memcpy(buffer, MCDI_PTR(outbuf, GET_DDR_CONFIG),
    (ADEC_NUM * 4));
    }
#[no_mangle]
unsafe extern "C" fn setup_mcdi(mc_priv: *mut mc_priv) -> c_int {
    static int setup_mcdi(struct mc_priv *mc_priv)
    {
    struct cdx_mcdi *amd_mcdi;
    int ret, i;
    amd_mcdi = kzalloc_obj(*amd_mcdi);
    if (!amd_mcdi)
    return -ENOMEM;
    amd_mcdi.mcdi_ops = &mcdi_ops;
    ret = cdx_mcdi_init(amd_mcdi);
    if (ret) {
    kfree(amd_mcdi);
    return ret;
    }
    amd_mcdi.ept = mc_priv.ept;
    mc_priv.mcdi = amd_mcdi;
    for (i = 0; i < NUM_CONTROLLERS; i++)
    get_ddr_config(i, &mc_priv.adec[ADEC_NUM * i], amd_mcdi);
    return 0;
    }
    static const guid_t amd_versalnet_guid = GUID_INIT(0x82678888, 0xa556, 0x44f2,
    0xb8, 0xb4, 0x45, 0x56, 0x2e,
    0x8c, 0x5b, 0xec);
    static int rpmsg_cb(struct rpmsg_device *rpdev, void *data,
    int len, void *priv, u32 src)
    {
    struct mc_priv *mc_priv = dev_get_drvdata(&rpdev.dev);
    const guid_t *sec_type = &guid_null;
    u32 length, offset, error_id;
    u32 *result = (u32 *)data;
    struct ecc_status *p;
    int i, j, k, sec_sev;
    const char *err_str;
    u32 *adec_data;
    if (*(u8 *)data == MCDI_RESPONSE) {
    cdx_mcdi_process_cmd(mc_priv.mcdi, (struct cdx_dword *)data, len);
    return 0;
    }
    sec_sev = result[ERROR_LEVEL];
    error_id = result[ERROR_ID];
    length = result[MSG_ERR_LENGTH];
    offset = result[MSG_ERR_OFFSET];
//
// The data can come in two stretches. Construct the regs from two
// messages. The offset indicates the offset from which the data is to
// be taken.
//
    for (i = 0 ; i < length; i++) {
    k = offset + i;
    j = ERROR_DATA + i;
    mc_priv.regs[k] = result[j];
    }
    if (result[TOTAL_ERR_LENGTH] > length) {
    if (!mc_priv.part_len)
    mc_priv.part_len = length;
    else
    mc_priv.part_len += length;
    if (mc_priv.part_len < result[TOTAL_ERR_LENGTH])
    return 0;
    mc_priv.part_len = 0;
    }
    mc_priv.error_id = error_id;
    mc_priv.error_level = result[ERROR_LEVEL];
    switch (error_id) {
    case 5:		err_str = "General Software Non-Correctable error"; break;
    case 6:		err_str = "CFU error"; break;
    case 7:		err_str = "CFRAME error"; break;
    case 10:	err_str = "DDRMC Microblaze Correctable ECC error"; break;
    case 11:	err_str = "DDRMC Microblaze Non-Correctable ECC error"; break;
    case 15:	err_str = "MMCM error"; break;
    case 16:	err_str = "HNICX Correctable error"; break;
    case 17:	err_str = "HNICX Non-Correctable error"; break;
    case 18:
    p = &mc_priv.stat;
    memset(p, 0, sizeof(struct ecc_status));
    p.error_type = MC5_ERR_TYPE_CE;
    for (i = 0 ; i < NUM_CONTROLLERS; i++) {
    if (get_ddr_info(&mc_priv.regs[i * REGS_PER_CONTROLLER], mc_priv)) {
    adec_data = mc_priv.adec + ADEC_NUM * i;
    handle_error(mc_priv, &mc_priv.stat, i, adec_data);
    }
    }
    return 0;
    case 19:
    p = &mc_priv.stat;
    memset(p, 0, sizeof(struct ecc_status));
    p.error_type = MC5_ERR_TYPE_UE;
    for (i = 0 ; i < NUM_CONTROLLERS; i++) {
    if (get_ddr_info(&mc_priv.regs[i * REGS_PER_CONTROLLER], mc_priv)) {
    adec_data = mc_priv.adec + ADEC_NUM * i;
    handle_error(mc_priv, &mc_priv.stat, i, adec_data);
    }
    }
    return 0;
    case 21:	err_str = "GT Non-Correctable error"; break;
    case 22:	err_str = "PL Sysmon Correctable error"; break;
    case 23:	err_str = "PL Sysmon Non-Correctable error"; break;
    case 111:	err_str = "LPX unexpected dfx activation error"; break;
    case 114:	err_str = "INT_LPD Non-Correctable error"; break;
    case 116:	err_str = "INT_OCM Non-Correctable error"; break;
    case 117:	err_str = "INT_FPD Correctable error"; break;
    case 118:	err_str = "INT_FPD Non-Correctable error"; break;
    case 120:	err_str = "INT_IOU Non-Correctable error"; break;
    case 123:	err_str = "err_int_irq from APU GIC Distributor"; break;
    case 124:	err_str = "fault_int_irq from APU GIC Distribute"; break;
    case 132 ... 139: err_str = "FPX SPLITTER error"; break;
    case 140:	err_str = "APU Cluster 0 error"; break;
    case 141:	err_str = "APU Cluster 1 error"; break;
    case 142:	err_str = "APU Cluster 2 error"; break;
    case 143:	err_str = "APU Cluster 3 error"; break;
    case 145:	err_str = "WWDT1 LPX error"; break;
    case 147:	err_str = "IPI error"; break;
    case 152 ... 153: err_str = "AFIFS error"; break;
    case 154 ... 155: err_str = "LPX glitch error"; break;
    case 185 ... 186: err_str = "FPX AFIFS error"; break;
    case 195 ... 199: err_str = "AFIFM error"; break;
    case 108:	err_str = "PSM Correctable error"; break;
    case 59:	err_str = "PMC correctable error"; break;
    case 60:	err_str = "PMC Un correctable error"; break;
    case 43 ... 47:	err_str = "PMC Sysmon error"; break;
    case 163 ... 184: err_str = "RPU error"; break;
    case 148:	err_str = "OCM0 correctable error"; break;
    case 149:	err_str = "OCM1 correctable error"; break;
    case 150:	err_str = "OCM0 Un-correctable error"; break;
    case 151:	err_str = "OCM1 Un-correctable error"; break;
    case 189:	err_str = "PSX_CMN_3 PD block consolidated error"; break;
    case 191:	err_str = "FPD_INT_WRAP PD block consolidated error"; break;
    case 232:	err_str = "CRAM Un-Correctable error"; break;
    default:	err_str = "VERSAL_EDAC_ERR_ID: %d"; break;
    }
    snprintf(mc_priv.message,
    sizeof(mc_priv.message),
    "[VERSAL_EDAC_ERR_ID: %d] Error type: %s", error_id, err_str);
// Convert to bytes
    length = result[TOTAL_ERR_LENGTH] * 4;
    log_non_standard_event(sec_type, &amd_versalnet_guid, mc_priv.message,
    sec_sev, (void *)&mc_priv.regs, length);
    return 0;
    }
    static struct rpmsg_device_id amd_rpmsg_id_table[] = {
    { .name = "error_ipc" },
    { },
    };
    MODULE_DEVICE_TABLE(rpmsg, amd_rpmsg_id_table);
#[no_mangle]
unsafe extern "C" fn rpmsg_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int rpmsg_probe(struct rpmsg_device *rpdev)
    {
    struct rpmsg_channel_info chinfo;
    struct mc_priv *pg;
    pg = (struct mc_priv *)amd_rpmsg_id_table[0].driver_data;
    chinfo.src = RPMSG_ADDR_ANY;
    chinfo.dst = rpdev.dst;
    strscpy(chinfo.name, amd_rpmsg_id_table[0].name,
    strlen(amd_rpmsg_id_table[0].name));
    pg.ept = rpmsg_create_ept(rpdev, rpmsg_cb, core::ptr::null_mut(), chinfo);
    if (!pg.ept)
    return dev_err_probe(&rpdev.dev, -ENXIO, "Failed to create ept for channel %s\n",
    chinfo.name);
    dev_set_drvdata(&rpdev.dev, pg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_remove(rpdev: *mut rpmsg_device) {
    static void rpmsg_remove(struct rpmsg_device *rpdev)
    {
    struct mc_priv *mc_priv = dev_get_drvdata(&rpdev.dev);
    rpmsg_destroy_ept(mc_priv.ept);
    dev_set_drvdata(&rpdev.dev, core::ptr::null_mut());
    }
    static struct rpmsg_driver amd_rpmsg_driver = {
    .drv.name = KBUILD_MODNAME,
    .probe = rpmsg_probe,
    .remove = rpmsg_remove,
    .callback = rpmsg_cb,
    .id_table = amd_rpmsg_id_table,
    };
#[no_mangle]
unsafe extern "C" fn versal_edac_release(dev: *mut device) {
    static void versal_edac_release(struct device *dev)
    {
    kfree(dev);
    }
#[no_mangle]
unsafe extern "C" fn remove_one_mc(priv: *mut mc_priv, i: c_int) {
    static void remove_one_mc(struct mc_priv *priv, int i)
    {
    struct mem_ctl_info *mci;
    mci = priv.mci[i];
    device_unregister(mci.pdev);
    edac_mc_del_mc(mci.pdev);
    edac_mc_free(mci);
    }
#[no_mangle]
unsafe extern "C" fn init_one_mc(priv: *mut mc_priv, pdev: *mut platform_device, i: c_int) -> c_int {
    static int init_one_mc(struct mc_priv *priv, struct platform_device *pdev, int i)
    {
    u32 num_chans, rank, dwidth, config;
    struct edac_mc_layer layers[2];
    struct mem_ctl_info *mci;
    char name[MC_NAME_LEN];
    struct device *dev;
    enum dev_type dt;
    int rc;
    config = priv.adec[CONF + i * ADEC_NUM];
    num_chans = FIELD_GET(MC5_NUM_CHANS_MASK, config);
    rank = 1 << FIELD_GET(MC5_RANK_MASK, config);
    dwidth = FIELD_GET(MC5_BUS_WIDTH_MASK, config);
    switch (dwidth) {
    case XDDR5_BUS_WIDTH_16:
    dt = DEV_X16;
    break;
    case XDDR5_BUS_WIDTH_32:
    dt = DEV_X32;
    break;
    case XDDR5_BUS_WIDTH_64:
    dt = DEV_X64;
    break;
    default:
    dt = DEV_UNKNOWN;
    }
    if (dt == DEV_UNKNOWN)
    return 0;
// Find the first enabled device and register that one.
    layers[0].type = EDAC_MC_LAYER_CHIP_SELECT;
    layers[0].size = rank;
    layers[0].is_virt_csrow = true;
    layers[1].type = EDAC_MC_LAYER_CHANNEL;
    layers[1].size = num_chans;
    layers[1].is_virt_csrow = false;
    rc = -ENOMEM;
    dev = kzalloc_obj(*dev);
    if (!dev)
    return rc;
    mci = edac_mc_alloc(i, ARRAY_SIZE(layers), layers, sizeof(struct mc_priv));
    if (!mci) {
    edac_printk(KERN_ERR, EDAC_MC, "Failed memory allocation for MC%d\n", i);
    goto err_dev_free;
    }
    sprintf(name, "versal-net-ddrmc5-edac-%d", i);
    dev.init_name = name;
    dev.release = versal_edac_release;
    rc = device_register(dev);
    if (rc)
    goto err_mc_free;
    mci.pdev = dev;
    mc_init(mci, dev);
    rc = edac_mc_add_mc(mci);
    if (rc) {
    edac_printk(KERN_ERR, EDAC_MC, "Failed to register MC%d with EDAC core\n", i);
    goto err_unreg;
    }
    priv.mci[i] = mci;
    priv.dwidth = dt;
    platform_set_drvdata(pdev, priv);
    return 0;
    err_unreg:
    device_unregister(mci.pdev);
    err_mc_free:
    edac_mc_free(mci);
    err_dev_free:
    kfree(dev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn init_versalnet(priv: *mut mc_priv, pdev: *mut platform_device) -> c_int {
    static int init_versalnet(struct mc_priv *priv, struct platform_device *pdev)
    {
    int rc, i;
    for (i = 0; i < NUM_CONTROLLERS; i++) {
    rc = init_one_mc(priv, pdev, i);
    if (rc) {
    while (i--)
    remove_one_mc(priv, i);
    return rc;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_versalnet(priv: *mut mc_priv) {
    static void remove_versalnet(struct mc_priv *priv)
    {
    for (int i = 0; i < NUM_CONTROLLERS; i++)
    remove_one_mc(priv, i);
    }
#[no_mangle]
unsafe extern "C" fn mc_probe(pdev: *mut platform_device) -> c_int {
    static int mc_probe(struct platform_device *pdev)
    {
    struct mc_priv *priv;
    struct rproc *rp;
    int rc;
    struct device_node *r5_core_node __free(device_node) =
    of_parse_phandle(pdev.dev.of_node, "amd,rproc", 0);
    if (!r5_core_node) {
    dev_err(&pdev.dev, "amd,rproc: invalid phandle\n");
    return -EINVAL;
    }
    rp = rproc_get_by_phandle(r5_core_node.phandle);
    if (!rp)
    return -EPROBE_DEFER;
    rc = rproc_boot(rp);
    if (rc) {
    dev_err(&pdev.dev, "Failed to attach to remote processor\n");
    goto err_rproc_boot;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    rc = -ENOMEM;
    goto err_alloc;
    }
    amd_rpmsg_id_table[0].driver_data = (kernel_ulong_t)priv;
    rc = register_rpmsg_driver(&amd_rpmsg_driver);
    if (rc) {
    edac_printk(KERN_ERR, EDAC_MC, "Failed to register RPMsg driver: %d\n", rc);
    goto err_alloc;
    }
    rc = setup_mcdi(priv);
    if (rc)
    goto err_unreg;
    priv.mcdi.r5_rproc = rp;
    rc = init_versalnet(priv, pdev);
    if (rc)
    goto err_init;
    return 0;
    err_init:
    cdx_mcdi_finish(priv.mcdi);
    kfree(priv.mcdi);
    err_unreg:
    unregister_rpmsg_driver(&amd_rpmsg_driver);
    err_alloc:
    rproc_shutdown(rp);
    err_rproc_boot:
    rproc_put(rp);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mc_remove(pdev: *mut platform_device) {
    static void mc_remove(struct platform_device *pdev)
    {
    struct mc_priv *priv = platform_get_drvdata(pdev);
    unregister_rpmsg_driver(&amd_rpmsg_driver);
    remove_versalnet(priv);
    rproc_shutdown(priv.mcdi.r5_rproc);
    cdx_mcdi_finish(priv.mcdi);
    kfree(priv.mcdi);
    }
    static const struct of_device_id amd_edac_match[] = {
    { .compatible = "xlnx,versal-net-ddrmc5", },
    {}
    };
    MODULE_DEVICE_TABLE(of, amd_edac_match);
    static struct platform_driver amd_ddr_edac_mc_driver = {
    .driver = {
    .name = "versal-net-edac",
    .of_match_table = amd_edac_match,
    },
    .probe = mc_probe,
    .remove = mc_remove,
    };
    module_platform_driver(amd_ddr_edac_mc_driver);
    MODULE_AUTHOR("AMD Inc");
    MODULE_DESCRIPTION("Versal NET EDAC driver");
    MODULE_LICENSE("GPL");
