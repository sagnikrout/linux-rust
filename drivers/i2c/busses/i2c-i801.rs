//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-i801.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
    Copyright (c) 1998 - 2002  Frodo Looijaard <frodol@dds.nl>,
    Philip Edelbrock <phil@netroedge.com>, and Mark D. Studebaker
    <mdsxyz123@yahoo.com>
    Copyright (C) 2007 - 2014  Jean Delvare <jdelvare@suse.de>
    Copyright (C) 2010         Intel Corporation,
    David Woodhouse <dwmw2@infradead.org>
//
// Supports the following Intel I/O Controller Hubs (ICH):
//
// I/O			Block	I2C
// region	SMBus	Block	proc.	block
// Chip name			PCI ID	size	PEC	buffer	call	read
// ---------------------------------------------------------------------------
// 82801AA (ICH)		0x2413	16	no	no	no	no
// 82801AB (ICH0)		0x2423	16	no	no	no	no
// 82801BA (ICH2)		0x2443	16	no	no	no	no
// 82801CA (ICH3)		0x2483	32	soft	no	no	no
// 82801DB (ICH4)		0x24c3	32	hard	yes	no	no
// 82801E (ICH5)		0x24d3	32	hard	yes	yes	yes
// 6300ESB			0x25a4	32	hard	yes	yes	yes
// 82801F (ICH6)		0x266a	32	hard	yes	yes	yes
// 6310ESB/6320ESB		0x269b	32	hard	yes	yes	yes
// 82801G (ICH7)		0x27da	32	hard	yes	yes	yes
// 82801H (ICH8)		0x283e	32	hard	yes	yes	yes
// 82801I (ICH9)		0x2930	32	hard	yes	yes	yes
// EP80579 (Tolapai)		0x5032	32	hard	yes	yes	yes
// ICH10			0x3a30	32	hard	yes	yes	yes
// ICH10			0x3a60	32	hard	yes	yes	yes
// 5/3400 Series (PCH)		0x3b30	32	hard	yes	yes	yes
// 6 Series (PCH)		0x1c22	32	hard	yes	yes	yes
// Patsburg (PCH)		0x1d22	32	hard	yes	yes	yes
// Patsburg (PCH) IDF		0x1d70	32	hard	yes	yes	yes
// Patsburg (PCH) IDF		0x1d71	32	hard	yes	yes	yes
// Patsburg (PCH) IDF		0x1d72	32	hard	yes	yes	yes
// DH89xxCC (PCH)		0x2330	32	hard	yes	yes	yes
// Panther Point (PCH)		0x1e22	32	hard	yes	yes	yes
// Lynx Point (PCH)		0x8c22	32	hard	yes	yes	yes
// Lynx Point-LP (PCH)		0x9c22	32	hard	yes	yes	yes
// Avoton (SOC)			0x1f3c	32	hard	yes	yes	yes
// Wellsburg (PCH)		0x8d22	32	hard	yes	yes	yes
// Wellsburg (PCH) MS		0x8d7d	32	hard	yes	yes	yes
// Wellsburg (PCH) MS		0x8d7e	32	hard	yes	yes	yes
// Wellsburg (PCH) MS		0x8d7f	32	hard	yes	yes	yes
// Coleto Creek (PCH)		0x23b0	32	hard	yes	yes	yes
// Wildcat Point (PCH)		0x8ca2	32	hard	yes	yes	yes
// Wildcat Point-LP (PCH)	0x9ca2	32	hard	yes	yes	yes
// BayTrail (SOC)		0x0f12	32	hard	yes	yes	yes
// Braswell (SOC)		0x2292	32	hard	yes	yes	yes
// Sunrise Point-H (PCH) 	0xa123  32	hard	yes	yes	yes
// Sunrise Point-LP (PCH)	0x9d23	32	hard	yes	yes	yes
// DNV (SOC)			0x19df	32	hard	yes	yes	yes
// Emmitsburg (PCH)		0x1bc9	32	hard	yes	yes	yes
// Broxton (SOC)		0x5ad4	32	hard	yes	yes	yes
// Lewisburg (PCH)		0xa1a3	32	hard	yes	yes	yes
// Lewisburg Supersku (PCH)	0xa223	32	hard	yes	yes	yes
// Kaby Lake PCH-H (PCH)	0xa2a3	32	hard	yes	yes	yes
// Gemini Lake (SOC)		0x31d4	32	hard	yes	yes	yes
// Cannon Lake-H (PCH)		0xa323	32	hard	yes	yes	yes
// Cannon Lake-LP (PCH)		0x9da3	32	hard	yes	yes	yes
// Cedar Fork (PCH)		0x18df	32	hard	yes	yes	yes
// Ice Lake-LP (PCH)		0x34a3	32	hard	yes	yes	yes
// Ice Lake-N (PCH)		0x38a3	32	hard	yes	yes	yes
// Comet Lake (PCH)		0x02a3	32	hard	yes	yes	yes
// Comet Lake-H (PCH)		0x06a3	32	hard	yes	yes	yes
// Elkhart Lake (PCH)		0x4b23	32	hard	yes	yes	yes
// Tiger Lake-LP (PCH)		0xa0a3	32	hard	yes	yes	yes
// Tiger Lake-H (PCH)		0x43a3	32	hard	yes	yes	yes
// Jasper Lake (SOC)		0x4da3	32	hard	yes	yes	yes
// Comet Lake-V (PCH)		0xa3a3	32	hard	yes	yes	yes
// Alder Lake-S (PCH)		0x7aa3	32	hard	yes	yes	yes
// Alder Lake-P (PCH)		0x51a3	32	hard	yes	yes	yes
// Alder Lake-M (PCH)		0x54a3	32	hard	yes	yes	yes
// Raptor Lake-S (PCH)		0x7a23	32	hard	yes	yes	yes
// Meteor Lake-P (SOC)		0x7e22	32	hard	yes	yes	yes
// Meteor Lake SoC-S (SOC)	0xae22	32	hard	yes	yes	yes
// Meteor Lake PCH-S (PCH)	0x7f23	32	hard	yes	yes	yes
// Birch Stream (SOC)		0x5796	32	hard	yes	yes	yes
// Arrow Lake-H (SOC)		0x7722	32	hard	yes	yes	yes
// Panther Lake-H (SOC)		0xe322	32	hard	yes	yes	yes
// Panther Lake-P (SOC)		0xe422	32	hard	yes	yes	yes
// Wildcat Lake-U (SOC)		0x4d22	32	hard	yes	yes	yes
// Diamond Rapids (SOC)		0x5827	32	hard	yes	yes	yes
// Nova Lake-S (PCH)		0x6e23	32	hard	yes	yes	yes
//
// Features supported by this driver:
// Software PEC				no
// Hardware PEC				yes
// Block buffer				yes
// Block process call transaction	yes
// I2C block read transaction		yes (doesn't use the block buffer)
// Target mode				no
// SMBus Host Notify			yes
// Interrupt processing			yes
//
// See the file Documentation/i2c/busses/i2c-i801.rst for details.
//

// I801 SMBus address offsets

// PCI Address Constants
pub const SMBBAR_MMIO: c_int = 0;
pub const SMBBAR: c_int = 4;
pub const SMBHSTCFG: c_uint = 0x040;
pub const TCOBASE: c_uint = 0x050;
pub const TCOCTL: c_uint = 0x054;
pub const SBREG_SMBCTRL: c_uint = 0xc6000c;
pub const SBREG_SMBCTRL_DNV: c_uint = 0xcf000c;
// Host configuration bits for SMBHSTCFG

// TCO configuration bits for TCOCTL

// Auxiliary status register bits, ICH4+ only

// Auxiliary control register bits, ICH4+ only

// I801 command constants
pub const I801_QUICK: c_uint = 0x00;
pub const I801_BYTE: c_uint = 0x04;
pub const I801_BYTE_DATA: c_uint = 0x08;
pub const I801_WORD_DATA: c_uint = 0x0C;
pub const I801_PROC_CALL: c_uint = 0x10;
pub const I801_BLOCK_DATA: c_uint = 0x14;
pub const I801_I2C_BLOCK_DATA: c_uint = 0x18	/* ICH5 and later */;
pub const I801_BLOCK_PROC_CALL: c_uint = 0x1C;
// I801 Host Control register bits

// I801 Hosts Status register bits

// Host Notify Status register bits

// Host Notify Command register bits

    SMBHSTSTS_DEV_ERR)

    STATUS_ERROR_FLAGS)

// Older devices have their ID defined in <linux/pci_ids.h>
pub const PCI_DEVICE_ID_INTEL_COMETLAKE_SMBUS: c_uint = 0x02a3;
pub const PCI_DEVICE_ID_INTEL_COMETLAKE_H_SMBUS: c_uint = 0x06a3;
pub const PCI_DEVICE_ID_INTEL_BAYTRAIL_SMBUS: c_uint = 0x0f12;
pub const PCI_DEVICE_ID_INTEL_CDF_SMBUS: c_uint = 0x18df;
pub const PCI_DEVICE_ID_INTEL_DNV_SMBUS: c_uint = 0x19df;
pub const PCI_DEVICE_ID_INTEL_EBG_SMBUS: c_uint = 0x1bc9;
pub const PCI_DEVICE_ID_INTEL_COUGARPOINT_SMBUS: c_uint = 0x1c22;
pub const PCI_DEVICE_ID_INTEL_PATSBURG_SMBUS: c_uint = 0x1d22;
// Patsburg also has three 'Integrated Device Function' SMBus controllers
pub const PCI_DEVICE_ID_INTEL_PATSBURG_SMBUS_IDF0: c_uint = 0x1d70;
pub const PCI_DEVICE_ID_INTEL_PATSBURG_SMBUS_IDF1: c_uint = 0x1d71;
pub const PCI_DEVICE_ID_INTEL_PATSBURG_SMBUS_IDF2: c_uint = 0x1d72;
pub const PCI_DEVICE_ID_INTEL_PANTHERPOINT_SMBUS: c_uint = 0x1e22;
pub const PCI_DEVICE_ID_INTEL_AVOTON_SMBUS: c_uint = 0x1f3c;
pub const PCI_DEVICE_ID_INTEL_BRASWELL_SMBUS: c_uint = 0x2292;
pub const PCI_DEVICE_ID_INTEL_DH89XXCC_SMBUS: c_uint = 0x2330;
pub const PCI_DEVICE_ID_INTEL_COLETOCREEK_SMBUS: c_uint = 0x23b0;
pub const PCI_DEVICE_ID_INTEL_GEMINILAKE_SMBUS: c_uint = 0x31d4;
pub const PCI_DEVICE_ID_INTEL_ICELAKE_LP_SMBUS: c_uint = 0x34a3;
pub const PCI_DEVICE_ID_INTEL_ICELAKE_N_SMBUS: c_uint = 0x38a3;
pub const PCI_DEVICE_ID_INTEL_5_3400_SERIES_SMBUS: c_uint = 0x3b30;
pub const PCI_DEVICE_ID_INTEL_TIGERLAKE_H_SMBUS: c_uint = 0x43a3;
pub const PCI_DEVICE_ID_INTEL_ELKHART_LAKE_SMBUS: c_uint = 0x4b23;
pub const PCI_DEVICE_ID_INTEL_WILDCAT_LAKE_U_SMBUS: c_uint = 0x4d22;
pub const PCI_DEVICE_ID_INTEL_JASPER_LAKE_SMBUS: c_uint = 0x4da3;
pub const PCI_DEVICE_ID_INTEL_ALDER_LAKE_P_SMBUS: c_uint = 0x51a3;
pub const PCI_DEVICE_ID_INTEL_ALDER_LAKE_M_SMBUS: c_uint = 0x54a3;
pub const PCI_DEVICE_ID_INTEL_BIRCH_STREAM_SMBUS: c_uint = 0x5796;
pub const PCI_DEVICE_ID_INTEL_DIAMOND_RAPIDS_SMBUS: c_uint = 0x5827;
pub const PCI_DEVICE_ID_INTEL_BROXTON_SMBUS: c_uint = 0x5ad4;
pub const PCI_DEVICE_ID_INTEL_NOVA_LAKE_S_SMBUS: c_uint = 0x6e23;
pub const PCI_DEVICE_ID_INTEL_ARROW_LAKE_H_SMBUS: c_uint = 0x7722;
pub const PCI_DEVICE_ID_INTEL_RAPTOR_LAKE_S_SMBUS: c_uint = 0x7a23;
pub const PCI_DEVICE_ID_INTEL_ALDER_LAKE_S_SMBUS: c_uint = 0x7aa3;
pub const PCI_DEVICE_ID_INTEL_METEOR_LAKE_P_SMBUS: c_uint = 0x7e22;
pub const PCI_DEVICE_ID_INTEL_METEOR_LAKE_PCH_S_SMBUS: c_uint = 0x7f23;
pub const PCI_DEVICE_ID_INTEL_LYNXPOINT_SMBUS: c_uint = 0x8c22;
pub const PCI_DEVICE_ID_INTEL_WILDCATPOINT_SMBUS: c_uint = 0x8ca2;
pub const PCI_DEVICE_ID_INTEL_WELLSBURG_SMBUS: c_uint = 0x8d22;
pub const PCI_DEVICE_ID_INTEL_WELLSBURG_SMBUS_MS0: c_uint = 0x8d7d;
pub const PCI_DEVICE_ID_INTEL_WELLSBURG_SMBUS_MS1: c_uint = 0x8d7e;
pub const PCI_DEVICE_ID_INTEL_WELLSBURG_SMBUS_MS2: c_uint = 0x8d7f;
pub const PCI_DEVICE_ID_INTEL_LYNXPOINT_LP_SMBUS: c_uint = 0x9c22;
pub const PCI_DEVICE_ID_INTEL_WILDCATPOINT_LP_SMBUS: c_uint = 0x9ca2;
pub const PCI_DEVICE_ID_INTEL_SUNRISEPOINT_LP_SMBUS: c_uint = 0x9d23;
pub const PCI_DEVICE_ID_INTEL_CANNONLAKE_LP_SMBUS: c_uint = 0x9da3;
pub const PCI_DEVICE_ID_INTEL_TIGERLAKE_LP_SMBUS: c_uint = 0xa0a3;
pub const PCI_DEVICE_ID_INTEL_SUNRISEPOINT_H_SMBUS: c_uint = 0xa123;
pub const PCI_DEVICE_ID_INTEL_LEWISBURG_SMBUS: c_uint = 0xa1a3;
pub const PCI_DEVICE_ID_INTEL_LEWISBURG_SSKU_SMBUS: c_uint = 0xa223;
pub const PCI_DEVICE_ID_INTEL_KABYLAKE_PCH_H_SMBUS: c_uint = 0xa2a3;
pub const PCI_DEVICE_ID_INTEL_CANNONLAKE_H_SMBUS: c_uint = 0xa323;
pub const PCI_DEVICE_ID_INTEL_COMETLAKE_V_SMBUS: c_uint = 0xa3a3;
pub const PCI_DEVICE_ID_INTEL_METEOR_LAKE_SOC_S_SMBUS: c_uint = 0xae22;
pub const PCI_DEVICE_ID_INTEL_PANTHER_LAKE_H_SMBUS: c_uint = 0xe322;
pub const PCI_DEVICE_ID_INTEL_PANTHER_LAKE_P_SMBUS: c_uint = 0xe422;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i801_mux_config {
    pub gpio_chip: *mut c_char,
    pub values: [unsigned; 3],
    pub n_values: c_int,
    pub /: *mut *mut unsigned gpios[2]; / Relative to gpio_chip->base,
    pub n_gpios: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i801_priv {
    pub adapter: i2c_adapter,
    pub smba: *mut void __iomem,
    pub original_hstcfg: c_uchar,
    pub original_hstcnt: c_uchar,
    pub original_slvcmd: c_uchar,
    pub pci_dev: *mut pci_dev,
    pub features: c_uint,
// isr processing
    pub done: completion,
    pub status: u8,
// Command state used by isr for byte-by-byte block transactions
    pub cmd: u8,
    pub is_read: bool,
    pub count: c_int,
    pub len: c_int,
    pub data: *mut u8,

    pub mux_pdev: *mut platform_device,
    pub lookup: *mut gpiod_lookup_table,
    pub mux_notifier_block: notifier_block,

    pub tco_pdev: *mut platform_device,
//
// If set to true the host controller registers are reserved for
// ACPI AML use. Needs extra protection by acpi_lock.
//
    pub acpi_reserved: bool,
    pub acpi_lock: mutex,
}

// Not really a feature, but it's convenient to handle it as such

    static const char *i801_feature_names[] = {
    "SMBus PEC",
    "Block buffer",
    "Block process call",
    "I2C block read",
    "Interrupt",
    "SMBus Host Notify",
    };
    static unsigned int disable_features;
    module_param(disable_features, uint, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(disable_features, "Disable selected driver features:\n"
    "\t\t  0x01  disable SMBus PEC\n"
    "\t\t  0x02  disable the block buffer\n"
    "\t\t  0x08  disable the I2C block read functionality\n"
    "\t\t  0x10  don't use interrupts\n"
    "\t\t  0x20  disable SMBus Host Notify ");
// Wait for BUSY being cleared and either INTR or an error flag being set
#[no_mangle]
unsafe extern "C" fn i801_wait_intr(priv: *mut i801_priv) -> c_int {
    static int i801_wait_intr(struct i801_priv *priv)
    {
    let mut timeout: c_ulong = jiffies + priv.adapter.timeout;
    int status, busy;
    do {
    usleep_range(250, 500);
    status = ioread8(SMBHSTSTS(priv));
    busy = status & SMBHSTSTS_HOST_BUSY;
    status &= STATUS_ERROR_FLAGS | SMBHSTSTS_INTR;
    if (!busy && status)
    return status & STATUS_ERROR_FLAGS;
    } while (time_is_after_eq_jiffies(timeout));
    return -ETIMEDOUT;
    }
// Wait for either BYTE_DONE or an error flag being set
#[no_mangle]
unsafe extern "C" fn i801_wait_byte_done(priv: *mut i801_priv) -> c_int {
    static int i801_wait_byte_done(struct i801_priv *priv)
    {
    let mut timeout: c_ulong = jiffies + priv.adapter.timeout;
    int status;
    do {
    usleep_range(250, 500);
    status = ioread8(SMBHSTSTS(priv));
    if (status & (STATUS_ERROR_FLAGS | SMBHSTSTS_BYTE_DONE))
    return status & STATUS_ERROR_FLAGS;
    } while (time_is_after_eq_jiffies(timeout));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn i801_get_block_len(priv: *mut i801_priv) -> c_int {
    static int i801_get_block_len(struct i801_priv *priv)
    {
    let mut len: u8 = ioread8(SMBHSTDAT0(priv));
    if (len < 1 || len > I2C_SMBUS_BLOCK_MAX) {
    pci_err(priv.pci_dev, "Illegal SMBus block read size %u\n", len);
    return -EPROTO;
    }
    return len;
    }
#[no_mangle]
unsafe extern "C" fn i801_check_and_clear_pec_error(priv: *mut i801_priv) -> c_int {
    static int i801_check_and_clear_pec_error(struct i801_priv *priv)
    {
    u8 status;
    if (!(priv.features & FEATURE_SMBUS_PEC))
    return 0;
    status = ioread8(SMBAUXSTS(priv)) & SMBAUXSTS_CRCE;
    if (status) {
    iowrite8(status, SMBAUXSTS(priv));
    return -EBADMSG;
    }
    return 0;
    }
// Make sure the SMBus host is ready to start transmitting.
    Return 0 if it is, -EBUSY if it is not. */
#[no_mangle]
unsafe extern "C" fn i801_check_pre(priv: *mut i801_priv) -> c_int {
    static int i801_check_pre(struct i801_priv *priv)
    {
    int status, result;
    status = ioread8(SMBHSTSTS(priv));
    if (status & SMBHSTSTS_HOST_BUSY) {
    pci_err(priv.pci_dev, "SMBus is busy, can't use it!\n");
    return -EBUSY;
    }
    status &= STATUS_FLAGS;
    if (status) {
    pci_dbg(priv.pci_dev, "Clearing status flags (%02x)\n", status);
    iowrite8(status, SMBHSTSTS(priv));
    }
//
// Clear CRC status if needed.
// During normal operation, i801_check_post() takes care
// of it after every operation.  We do it here only in case
// the hardware was already in this state when the driver
// started.
//
    result = i801_check_and_clear_pec_error(priv);
    if (result)
    pci_dbg(priv.pci_dev, "Clearing aux status flag CRCE\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i801_check_post(priv: *mut i801_priv, status: c_int) -> c_int {
    static int i801_check_post(struct i801_priv *priv, int status)
    {
    let mut result: c_int = 0;
//
// If the SMBus is still busy, we give up
//
    if (unlikely(status < 0)) {
// try to stop the current command
    iowrite8(SMBHSTCNT_KILL, SMBHSTCNT(priv));
    status = i801_wait_intr(priv);
    iowrite8(0, SMBHSTCNT(priv));
// Check if it worked
    if (status < 0 || !(status & SMBHSTSTS_FAILED))
    pci_dbg(priv.pci_dev, "Failed terminating the transaction\n");
    return -ETIMEDOUT;
    }
    if (status & SMBHSTSTS_FAILED) {
    result = -EIO;
    pci_err(priv.pci_dev, "Transaction failed\n");
    }
    if (status & SMBHSTSTS_DEV_ERR) {
//
// This may be a PEC error, check and clear it.
//
// AUXSTS is handled differently from HSTSTS.
// For HSTSTS, i801_isr() or i801_wait_intr()
// has already cleared the error bits in hardware,
// and we are passed a copy of the original value
// in "status".
// For AUXSTS, the hardware register is left
// for us to handle here.
// This is asymmetric, slightly iffy, but safe,
// since all this code is serialized and the CRCE
// bit is harmless as long as it's cleared before
// the next operation.
//
    result = i801_check_and_clear_pec_error(priv);
    if (result) {
    pci_dbg(priv.pci_dev, "PEC error\n");
    } else {
    result = -ENXIO;
    pci_dbg(priv.pci_dev, "No response\n");
    }
    }
    if (status & SMBHSTSTS_BUS_ERR) {
    result = -EAGAIN;
    pci_dbg(priv.pci_dev, "Lost arbitration\n");
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn i801_transaction(priv: *mut i801_priv, xact: c_int) -> c_int {
    static int i801_transaction(struct i801_priv *priv, int xact)
    {
    unsigned long result;
    const struct i2c_adapter *adap = &priv.adapter;
    if (priv.features & FEATURE_IRQ) {
    reinit_completion(&priv.done);
    iowrite8(xact | SMBHSTCNT_INTREN | SMBHSTCNT_START,
    SMBHSTCNT(priv));
    result = wait_for_completion_timeout(&priv.done, adap.timeout);
    return result ? priv.status : -ETIMEDOUT;
    }
    iowrite8(xact | SMBHSTCNT_START, SMBHSTCNT(priv));
    return i801_wait_intr(priv);
    }
    static int i801_block_transaction_by_block(struct i801_priv *priv,
    union i2c_smbus_data *data,
    char read_write, int command)
    {
    int len, status, xact;
    switch (command) {
    case I2C_SMBUS_BLOCK_PROC_CALL:
    xact = I801_BLOCK_PROC_CALL;
    break;
    case I2C_SMBUS_BLOCK_DATA:
    xact = I801_BLOCK_DATA;
    break;
    default:
    return -EOPNOTSUPP;
    }
// Set block buffer mode
    iowrite8(ioread8(SMBAUXCTL(priv)) | SMBAUXCTL_E32B, SMBAUXCTL(priv));
    if (read_write == I2C_SMBUS_WRITE) {
    len = data.block[0];
    iowrite8(len, SMBHSTDAT0(priv));
    ioread8(SMBHSTCNT(priv));	/* reset the data buffer index */
    iowrite8_rep(SMBBLKDAT(priv), data.block + 1, len);
    }
    status = i801_transaction(priv, xact);
    if (status)
    goto out;
    if (read_write == I2C_SMBUS_READ ||
    command == I2C_SMBUS_BLOCK_PROC_CALL) {
    len = i801_get_block_len(priv);
    if (len < 0) {
    status = len;
    goto out;
    }
    data.block[0] = len;
    ioread8(SMBHSTCNT(priv));	/* reset the data buffer index */
    ioread8_rep(SMBBLKDAT(priv), data.block + 1, len);
    }
    out:
    iowrite8(ioread8(SMBAUXCTL(priv)) & ~SMBAUXCTL_E32B, SMBAUXCTL(priv));
    return status;
    }
#[no_mangle]
unsafe extern "C" fn i801_isr_byte_done(priv: *mut i801_priv) {
    static void i801_isr_byte_done(struct i801_priv *priv)
    {
    if (priv.is_read) {
//
// At transfer start i801_smbus_block_transaction() marks
// the block length as invalid. Check for this sentinel value
// and read the block length from SMBHSTDAT0.
//
    if (priv.len == SMBUS_LEN_SENTINEL) {
    priv.len = i801_get_block_len(priv);
    if (priv.len < 0)
// FIXME: Recover
    priv.len = I2C_SMBUS_BLOCK_MAX;
    priv.data[-1] = priv.len;
    }
// Read next byte
    if (priv.count < priv.len)
    priv.data[priv.count++] = ioread8(SMBBLKDAT(priv));
    else
    pci_dbg(priv.pci_dev, "Discarding extra byte on block read\n");
// Set LAST_BYTE for last byte of read transaction
    if (priv.count == priv.len - 1)
    iowrite8(priv.cmd | SMBHSTCNT_LAST_BYTE,
    SMBHSTCNT(priv));
    } else if (priv.count < priv.len - 1) {
// Write next byte, except for IRQ after last byte
    iowrite8(priv.data[++priv.count], SMBBLKDAT(priv));
    }
    }
#[no_mangle]
unsafe extern "C" fn i801_host_notify_isr(priv: *mut i801_priv) -> irqreturn_t {
    static irqreturn_t i801_host_notify_isr(struct i801_priv *priv)
    {
    unsigned short addr;
    addr = ioread8(SMBNTFDADD(priv)) >> 1;
//
// With the tested platforms, reading SMBNTFDDAT (22 + (p)->smba)
// always returns 0. Our current implementation doesn't provide
// data, so we just ignore it.
//
    i2c_handle_smbus_host_notify(&priv.adapter, addr);
// clear Host Notify bit and return
    iowrite8(SMBSLVSTS_HST_NTFY_STS, SMBSLVSTS(priv));
    return IRQ_HANDLED;
    }
//
// There are three kinds of interrupts:
//
// 1) i801 signals transaction completion with one of these interrupts:
// INTR - Success
// DEV_ERR - Invalid command, NAK or communication timeout
// BUS_ERR - SMI# transaction collision
// FAILED - transaction was canceled due to a KILL request
// When any of these occur, update ->status and signal completion.
//
// 2) For byte-by-byte (I2C read/write) transactions, one BYTE_DONE interrupt
// occurs for each byte of a byte-by-byte to prepare the next byte.
//
// 3) Host Notify interrupts
//
#[no_mangle]
unsafe extern "C" fn i801_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i801_isr(int irq, void *dev_id)
    {
    struct i801_priv *priv = dev_id;
    u16 pcists;
    u8 status;
// Confirm this is our interrupt
    pci_read_config_word(priv.pci_dev, PCI_STATUS, &pcists);
    if (!(pcists & PCI_STATUS_INTERRUPT))
    return IRQ_NONE;
    if (priv.features & FEATURE_HOST_NOTIFY) {
    status = ioread8(SMBSLVSTS(priv));
    if (status & SMBSLVSTS_HST_NTFY_STS)
    return i801_host_notify_isr(priv);
    }
    status = ioread8(SMBHSTSTS(priv));
    if ((status & (SMBHSTSTS_BYTE_DONE | STATUS_ERROR_FLAGS)) == SMBHSTSTS_BYTE_DONE)
    i801_isr_byte_done(priv);
//
// Clear IRQ sources: SMB_ALERT status is set after signal assertion
// independently of the interrupt generation being blocked or not
// so clear it always when the status is set.
//
    status &= STATUS_FLAGS | SMBHSTSTS_SMBALERT_STS;
    iowrite8(status, SMBHSTSTS(priv));
    status &= STATUS_ERROR_FLAGS | SMBHSTSTS_INTR;
    if (status) {
    priv.status = status & STATUS_ERROR_FLAGS;
    complete(&priv.done);
    }
    return IRQ_HANDLED;
    }
//
// For "byte-by-byte" block transactions:
// I2C write uses cmd=I801_BLOCK_DATA, I2C_EN=1
// I2C read uses cmd=I801_I2C_BLOCK_DATA
//
    static int i801_block_transaction_byte_by_byte(struct i801_priv *priv,
    union i2c_smbus_data *data,
    char read_write, int command)
    {
    int i, len;
    int smbcmd;
    int status;
    unsigned long result;
    const struct i2c_adapter *adap = &priv.adapter;
    if (command == I2C_SMBUS_BLOCK_PROC_CALL)
    return -EOPNOTSUPP;
    len = data.block[0];
    if (read_write == I2C_SMBUS_WRITE) {
    iowrite8(len, SMBHSTDAT0(priv));
    iowrite8(data.block[1], SMBBLKDAT(priv));
    }
    if (command == I2C_SMBUS_I2C_BLOCK_DATA &&
    read_write == I2C_SMBUS_READ)
    smbcmd = I801_I2C_BLOCK_DATA;
    else
    smbcmd = I801_BLOCK_DATA;
    if (priv.features & FEATURE_IRQ) {
    priv.is_read = (read_write == I2C_SMBUS_READ);
    if (len == 1 && priv.is_read)
    smbcmd |= SMBHSTCNT_LAST_BYTE;
    priv.cmd = smbcmd | SMBHSTCNT_INTREN;
    priv.len = len;
    priv.count = 0;
    priv.data = &data.block[1];
    reinit_completion(&priv.done);
    iowrite8(priv.cmd | SMBHSTCNT_START, SMBHSTCNT(priv));
    result = wait_for_completion_timeout(&priv.done, adap.timeout);
    return result ? priv.status : -ETIMEDOUT;
    }
    if (len == 1 && read_write == I2C_SMBUS_READ)
    smbcmd |= SMBHSTCNT_LAST_BYTE;
    iowrite8(smbcmd | SMBHSTCNT_START, SMBHSTCNT(priv));
    for (i = 1; i <= len; i++) {
    status = i801_wait_byte_done(priv);
    if (status)
    return status;
//
// At transfer start i801_smbus_block_transaction() marks
// the block length as invalid. Check for this sentinel value
// and read the block length from SMBHSTDAT0.
//
    if (len == SMBUS_LEN_SENTINEL) {
    len = i801_get_block_len(priv);
    if (len < 0) {
// Recover
    while (ioread8(SMBHSTSTS(priv)) &
    SMBHSTSTS_HOST_BUSY)
    iowrite8(SMBHSTSTS_BYTE_DONE,
    SMBHSTSTS(priv));
    iowrite8(SMBHSTSTS_INTR, SMBHSTSTS(priv));
    return -EPROTO;
    }
    data.block[0] = len;
    }
    if (read_write == I2C_SMBUS_READ) {
    data.block[i] = ioread8(SMBBLKDAT(priv));
    if (i == len - 1)
    iowrite8(smbcmd | SMBHSTCNT_LAST_BYTE, SMBHSTCNT(priv));
    }
    if (read_write == I2C_SMBUS_WRITE && i+1 <= len)
    iowrite8(data.block[i+1], SMBBLKDAT(priv));
// signals SMBBLKDAT ready
    iowrite8(SMBHSTSTS_BYTE_DONE, SMBHSTSTS(priv));
    }
    return i801_wait_intr(priv);
    }
#[no_mangle]
unsafe extern "C" fn i801_set_hstadd(priv: *mut i801_priv, addr: u8, read_write: c_char) {
    static void i801_set_hstadd(struct i801_priv *priv, u8 addr, char read_write)
    {
    iowrite8((addr << 1) | (read_write & 0x01), SMBHSTADD(priv));
    }
// Single value transaction function
    static int i801_simple_transaction(struct i801_priv *priv, union i2c_smbus_data *data,
    u8 addr, u8 hstcmd, char read_write, int command)
    {
    int xact, ret;
    switch (command) {
    case I2C_SMBUS_QUICK:
    i801_set_hstadd(priv, addr, read_write);
    xact = I801_QUICK;
    break;
    case I2C_SMBUS_BYTE:
    i801_set_hstadd(priv, addr, read_write);
    if (read_write == I2C_SMBUS_WRITE)
    iowrite8(hstcmd, SMBHSTCMD(priv));
    xact = I801_BYTE;
    break;
    case I2C_SMBUS_BYTE_DATA:
    i801_set_hstadd(priv, addr, read_write);
    if (read_write == I2C_SMBUS_WRITE)
    iowrite8(data.byte, SMBHSTDAT0(priv));
    iowrite8(hstcmd, SMBHSTCMD(priv));
    xact = I801_BYTE_DATA;
    break;
    case I2C_SMBUS_WORD_DATA:
    i801_set_hstadd(priv, addr, read_write);
    if (read_write == I2C_SMBUS_WRITE) {
    iowrite8(data.word & 0xff, SMBHSTDAT0(priv));
    iowrite8((data.word & 0xff00) >> 8, SMBHSTDAT1(priv));
    }
    iowrite8(hstcmd, SMBHSTCMD(priv));
    xact = I801_WORD_DATA;
    break;
    case I2C_SMBUS_PROC_CALL:
    i801_set_hstadd(priv, addr, I2C_SMBUS_WRITE);
    iowrite8(data.word & 0xff, SMBHSTDAT0(priv));
    iowrite8((data.word & 0xff00) >> 8, SMBHSTDAT1(priv));
    iowrite8(hstcmd, SMBHSTCMD(priv));
    read_write = I2C_SMBUS_READ;
    xact = I801_PROC_CALL;
    break;
    default:
    pci_err(priv.pci_dev, "Unsupported transaction %d\n", command);
    return -EOPNOTSUPP;
    }
    ret = i801_transaction(priv, xact);
    if (ret || read_write == I2C_SMBUS_WRITE)
    return ret;
    switch (command) {
    case I2C_SMBUS_BYTE:
    case I2C_SMBUS_BYTE_DATA:
    data.byte = ioread8(SMBHSTDAT0(priv));
    break;
    case I2C_SMBUS_WORD_DATA:
    case I2C_SMBUS_PROC_CALL:
    data.word = ioread8(SMBHSTDAT0(priv)) +
    (ioread8(SMBHSTDAT1(priv)) << 8);
    break;
    }
    return 0;
    }
    static int i801_smbus_block_transaction(struct i801_priv *priv, union i2c_smbus_data *data,
    u8 addr, u8 hstcmd, char read_write, int command)
    {
    if (read_write == I2C_SMBUS_READ && command == I2C_SMBUS_BLOCK_DATA)
// Mark block length as invalid
    data.block[0] = SMBUS_LEN_SENTINEL;
#[no_mangle]
pub unsafe extern "C" fn if(I2C_SMBUS_BLOCK_MAX: data->block[0] < 1 || data->block[0] >) -> else {
    else if (data.block[0] < 1 || data.block[0] > I2C_SMBUS_BLOCK_MAX)
    return -EPROTO;
    if (command == I2C_SMBUS_BLOCK_PROC_CALL)
// Needs to be flagged as write transaction
    i801_set_hstadd(priv, addr, I2C_SMBUS_WRITE);
    else
    i801_set_hstadd(priv, addr, read_write);
    iowrite8(hstcmd, SMBHSTCMD(priv));
    if (priv.features & FEATURE_BLOCK_BUFFER)
    return i801_block_transaction_by_block(priv, data, read_write, command);
    else
    return i801_block_transaction_byte_by_byte(priv, data, read_write, command);
    }
    static int i801_i2c_block_transaction(struct i801_priv *priv, union i2c_smbus_data *data,
    u8 addr, u8 hstcmd, char read_write, int command)
    {
    int result;
    u8 hostc;
    if (data.block[0] < 1 || data.block[0] > I2C_SMBUS_BLOCK_MAX)
    return -EPROTO;
//
// NB: page 240 of ICH5 datasheet shows that the R/#W bit should be cleared here,
// even when reading. However if SPD Write Disable is set (Lynx Point and later),
// the read will fail if we don't set the R/#W bit.
//
    i801_set_hstadd(priv, addr,
    priv.original_hstcfg & SMBHSTCFG_SPD_WD ? read_write : I2C_SMBUS_WRITE);
// NB: page 240 of ICH5 datasheet shows that DATA1 is the cmd field when reading
    if (read_write == I2C_SMBUS_READ)
    iowrite8(hstcmd, SMBHSTDAT1(priv));
    else
    iowrite8(hstcmd, SMBHSTCMD(priv));
    if (read_write == I2C_SMBUS_WRITE) {
// set I2C_EN bit in configuration register
    pci_read_config_byte(priv.pci_dev, SMBHSTCFG, &hostc);
    pci_write_config_byte(priv.pci_dev, SMBHSTCFG, hostc | SMBHSTCFG_I2C_EN);
    } else if (!(priv.features & FEATURE_I2C_BLOCK_READ)) {
    pci_err(priv.pci_dev, "I2C block read is unsupported!\n");
    return -EOPNOTSUPP;
    }
// Block buffer isn't supported for I2C block transactions
    result = i801_block_transaction_byte_by_byte(priv, data, read_write, command);
// restore saved configuration register value
    if (read_write == I2C_SMBUS_WRITE)
    pci_write_config_byte(priv.pci_dev, SMBHSTCFG, hostc);
    return result;
    }
// Return negative errno on error.
    static s32 i801_access(struct i2c_adapter *adap, u16 addr,
    unsigned short flags, char read_write, u8 command,
    int size, union i2c_smbus_data *data)
    {
    int hwpec, ret;
    struct i801_priv *priv = i2c_get_adapdata(adap);
    mutex_lock(&priv.acpi_lock);
    if (priv.acpi_reserved) {
    mutex_unlock(&priv.acpi_lock);
    return -EBUSY;
    }
    pm_runtime_get_sync(&priv.pci_dev.dev);
    ret = i801_check_pre(priv);
    if (ret)
    goto out;
    hwpec = (priv.features & FEATURE_SMBUS_PEC) && (flags & I2C_CLIENT_PEC)
    && size != I2C_SMBUS_QUICK
    && size != I2C_SMBUS_I2C_BLOCK_DATA;
    if (hwpec)	/* enable/disable hardware PEC */
    iowrite8(ioread8(SMBAUXCTL(priv)) | SMBAUXCTL_CRC, SMBAUXCTL(priv));
    else
    iowrite8(ioread8(SMBAUXCTL(priv)) & (~SMBAUXCTL_CRC),
    SMBAUXCTL(priv));
    if (size == I2C_SMBUS_BLOCK_DATA || size == I2C_SMBUS_BLOCK_PROC_CALL)
    ret = i801_smbus_block_transaction(priv, data, addr, command, read_write, size);
#[no_mangle]
pub unsafe extern "C" fn if(I2C_SMBUS_I2C_BLOCK_DATA: size ==) -> else {
    else if (size == I2C_SMBUS_I2C_BLOCK_DATA)
    ret = i801_i2c_block_transaction(priv, data, addr, command, read_write, size);
    else
    ret = i801_simple_transaction(priv, data, addr, command, read_write, size);
    ret = i801_check_post(priv, ret);
// Some BIOSes don't like it when PEC is enabled at reboot or resume
// time, so we forcibly disable it after every transaction.
//
    if (hwpec)
    iowrite8(ioread8(SMBAUXCTL(priv)) & ~SMBAUXCTL_CRC, SMBAUXCTL(priv));
//
// Unlock the SMBus device for use by BIOS/ACPI,
// and clear status flags if not done already.
//
    iowrite8(SMBHSTSTS_INUSE_STS | STATUS_FLAGS, SMBHSTSTS(priv));
    out:
    pm_runtime_put_autosuspend(&priv.pci_dev.dev);
    mutex_unlock(&priv.acpi_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i801_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 i801_func(struct i2c_adapter *adapter)
    {
    struct i801_priv *priv = i2c_get_adapdata(adapter);
    return I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_PROC_CALL |
    I2C_FUNC_SMBUS_BLOCK_DATA | I2C_FUNC_SMBUS_WRITE_I2C_BLOCK |
    ((priv.features & FEATURE_SMBUS_PEC) ? I2C_FUNC_SMBUS_PEC : 0) |
    ((priv.features & FEATURE_BLOCK_PROC) ?
    I2C_FUNC_SMBUS_BLOCK_PROC_CALL : 0) |
    ((priv.features & FEATURE_I2C_BLOCK_READ) ?
    I2C_FUNC_SMBUS_READ_I2C_BLOCK : 0) |
    ((priv.features & FEATURE_HOST_NOTIFY) ?
    I2C_FUNC_SMBUS_HOST_NOTIFY : 0);
    }
#[no_mangle]
unsafe extern "C" fn i801_enable_host_notify(adapter: *mut i2c_adapter) {
    static void i801_enable_host_notify(struct i2c_adapter *adapter)
    {
    struct i801_priv *priv = i2c_get_adapdata(adapter);
    if (!(priv.features & FEATURE_HOST_NOTIFY))
    return;
//
// Enable host notify interrupt and block the generation of interrupt
// from the SMB_ALERT signal because the driver does not support
// SMBus Alert.
//
    iowrite8(SMBSLVCMD_HST_NTFY_INTREN | SMBSLVCMD_SMBALERT_DISABLE |
    priv.original_slvcmd, SMBSLVCMD(priv));
// clear Host Notify bit to allow a new notification
    iowrite8(SMBSLVSTS_HST_NTFY_STS, SMBSLVSTS(priv));
    }
#[no_mangle]
unsafe extern "C" fn i801_disable_host_notify(priv: *mut i801_priv) {
    static void i801_disable_host_notify(struct i801_priv *priv)
    {
    if (!(priv.features & FEATURE_HOST_NOTIFY))
    return;
    iowrite8(priv.original_slvcmd, SMBSLVCMD(priv));
    }
    static const struct i2c_algorithm smbus_algorithm = {
    .smbus_xfer	= i801_access,
    .functionality	= i801_func,
    };

    FEATURE_HOST_NOTIFY)

    FEATURE_I2C_BLOCK_READ | FEATURE_IRQ)
    static const struct pci_device_id i801_ids[] = {
    { PCI_DEVICE_DATA(INTEL, 82801AA_3,			0)				 },
    { PCI_DEVICE_DATA(INTEL, 82801AB_3,			0)				 },
    { PCI_DEVICE_DATA(INTEL, 82801BA_2,			0)				 },
    { PCI_DEVICE_DATA(INTEL, 82801CA_3,			FEATURE_HOST_NOTIFY)		 },
    { PCI_DEVICE_DATA(INTEL, 82801DB_3,			FEATURES_ICH4)			 },
    { PCI_DEVICE_DATA(INTEL, 82801EB_3,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ESB_4,				FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH6_16,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH7_17,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ESB2_17,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH8_5,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH9_6,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, EP80579_1,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH10_4,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ICH10_5,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, 5_3400_SERIES_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, COUGARPOINT_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, PATSBURG_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, PATSBURG_SMBUS_IDF0,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, PATSBURG_SMBUS_IDF1,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, PATSBURG_SMBUS_IDF2,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, DH89XXCC_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, PANTHERPOINT_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, LYNXPOINT_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, LYNXPOINT_LP_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, AVOTON_SMBUS,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, WELLSBURG_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, WELLSBURG_SMBUS_MS0,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, WELLSBURG_SMBUS_MS1,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, WELLSBURG_SMBUS_MS2,		FEATURES_ICH5 | FEATURE_IDF)	 },
    { PCI_DEVICE_DATA(INTEL, COLETOCREEK_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, GEMINILAKE_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, WILDCATPOINT_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, WILDCATPOINT_LP_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, BAYTRAIL_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, BRASWELL_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, SUNRISEPOINT_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, SUNRISEPOINT_LP_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, CDF_SMBUS,			FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, DNV_SMBUS,			FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, EBG_SMBUS,			FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, BROXTON_SMBUS,			FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, LEWISBURG_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, LEWISBURG_SSKU_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, KABYLAKE_PCH_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, CANNONLAKE_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, CANNONLAKE_LP_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, ICELAKE_LP_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, ICELAKE_N_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, COMETLAKE_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, COMETLAKE_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, COMETLAKE_V_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_SPT) },
    { PCI_DEVICE_DATA(INTEL, ELKHART_LAKE_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, TIGERLAKE_LP_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, TIGERLAKE_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, JASPER_LAKE_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, ALDER_LAKE_S_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, ALDER_LAKE_P_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, ALDER_LAKE_M_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, RAPTOR_LAKE_S_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, METEOR_LAKE_P_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, METEOR_LAKE_SOC_S_SMBUS,	FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, METEOR_LAKE_PCH_S_SMBUS,	FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, BIRCH_STREAM_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, DIAMOND_RAPIDS_SMBUS,		FEATURES_ICH5)			 },
    { PCI_DEVICE_DATA(INTEL, ARROW_LAKE_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, PANTHER_LAKE_H_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, PANTHER_LAKE_P_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, WILDCAT_LAKE_U_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { PCI_DEVICE_DATA(INTEL, NOVA_LAKE_S_SMBUS,		FEATURES_ICH5 | FEATURE_TCO_CNL) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, i801_ids);

    static unsigned char apanel_addr __ro_after_init;
// Scan the system ROM for the signature "FJKEYINF"
    static __init const void __iomem *bios_signature(const void __iomem *bios)
    {
    ssize_t offset;
    const unsigned char signature[] = "FJKEYINF";
    for (offset = 0; offset < 0x10000; offset += 0x10) {
    if (check_signature(bios + offset, signature,
    sizeof(signature)-1))
    return bios + offset;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn input_apanel_init() -> void __init {
    static void __init input_apanel_init(void)
    {
    void __iomem *bios;
    const void __iomem *p;
    bios = ioremap(0xF0000, 0x10000); /* Can't fail */
    p = bios_signature(bios);
    if (p) {
// just use the first address
    apanel_addr = readb(p + 8 + 3) >> 1;
    }
    iounmap(bios);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_onboard_device_info {
    pub name: *const c_char,
    pub type: u8,
    pub i2c_addr: c_ushort,
    pub i2c_type: *const c_char,
}

    static const struct dmi_onboard_device_info dmi_devices[] = {
    { "Syleus", DMI_DEV_TYPE_OTHER, 0x73, "fscsyl" },
    { "Hermes", DMI_DEV_TYPE_OTHER, 0x73, "fscher" },
    { "Hades",  DMI_DEV_TYPE_OTHER, 0x73, "fschds" },
    };
    static void dmi_check_onboard_device(u8 type, const char *name,
    struct i2c_adapter *adap)
    {
    int i;
    struct i2c_board_info info;
    for (i = 0; i < ARRAY_SIZE(dmi_devices); i++) {
// & ~0x80, ignore enabled/disabled bit
    if ((type & ~0x80) != dmi_devices[i].type)
    continue;
    if (strcasecmp(name, dmi_devices[i].name))
    continue;
    memset(&info, 0, sizeof(struct i2c_board_info));
    info.addr = dmi_devices[i].i2c_addr;
    strscpy(info.type, dmi_devices[i].i2c_type, I2C_NAME_SIZE);
    i2c_new_client_device(adap, &info);
    break;
    }
    }
// We use our own function to check for onboard devices instead of
    dmi_find_device() as some buggy BIOS's have the devices we are interested
    in marked as disabled */
#[no_mangle]
unsafe extern "C" fn dmi_check_onboard_devices(dm: *const dmi_header, adap: *mut c_void) {
    static void dmi_check_onboard_devices(const struct dmi_header *dm, void *adap)
    {
    int i, count;
    if (dm.type != DMI_ENTRY_ONBOARD_DEVICE)
    return;
    count = (dm.length - sizeof(struct dmi_header)) / 2;
    for (i = 0; i < count; i++) {
    const u8 *d = (char *)(dm + 1) + (i * 2);
    const char *name = ((char *) dm) + dm.length;
    let mut type: u8 = d[0];
    let mut s: u8 = d[1];
    if (!s)
    continue;
    s--;
    while (s > 0 && name[0]) {
    name += strlen(name) + 1;
    s--;
    }
    if (name[0] == 0) /* Bogus string reference */
    continue;
    dmi_check_onboard_device(type, name, adap);
    }
    }
// Register optional targets
#[no_mangle]
unsafe extern "C" fn i801_probe_optional_targets(priv: *mut i801_priv) {
    static void i801_probe_optional_targets(struct i801_priv *priv)
    {
// Only register targets on main SMBus channel
    if (priv.features & FEATURE_IDF)
    return;
    if (apanel_addr) {
    struct i2c_board_info info = {
    .addr = apanel_addr,
    .type = "fujitsu_apanel",
    };
    i2c_new_client_device(&priv.adapter, &info);
    }
    if (dmi_name_in_vendors("FUJITSU"))
    dmi_walk(dmi_check_onboard_devices, &priv.adapter);
// Instantiate SPD EEPROMs unless the SMBus is multiplexed

    if (!priv.mux_pdev)

    i2c_register_spd_write_enable(&priv.adapter);
    }

    static void __init input_apanel_init(void) {}
    static void i801_probe_optional_targets(struct i801_priv *priv) {}

    static struct i801_mux_config i801_mux_config_asus_z8_d12 = {
    .gpio_chip = "gpio_ich",
    .values = { 0x02, 0x03 },
    .n_values = 2,
    .gpios = { 52, 53 },
    .n_gpios = 2,
    };
    static struct i801_mux_config i801_mux_config_asus_z8_d18 = {
    .gpio_chip = "gpio_ich",
    .values = { 0x02, 0x03, 0x01 },
    .n_values = 3,
    .gpios = { 52, 53 },
    .n_gpios = 2,
    };
    static const struct dmi_system_id mux_dmi_table[] = {
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8NA-D6(C)"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8P(N)E-D12(X)"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8NH-D12"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8PH-D12/IFB"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8NR-D12"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8P(N)H-D12"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8PG-D18"),
    },
    .driver_data = &i801_mux_config_asus_z8_d18,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8PE-D18"),
    },
    .driver_data = &i801_mux_config_asus_z8_d18,
    },
    {
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_BOARD_NAME, "Z8PS-D12"),
    },
    .driver_data = &i801_mux_config_asus_z8_d12,
    },
    { }
    };
    static int i801_notifier_call(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct i801_priv *priv = container_of(nb, struct i801_priv, mux_notifier_block);
    struct device *dev = data;
    if (action != BUS_NOTIFY_ADD_DEVICE ||
    dev.type != &i2c_adapter_type ||
    i2c_root_adapter(dev) != &priv.adapter)
    return NOTIFY_DONE;
// Call i2c_register_spd for muxed child segments
    i2c_register_spd_write_enable(to_i2c_adapter(dev));
    return NOTIFY_OK;
    }
// Setup multiplexing if needed
#[no_mangle]
unsafe extern "C" fn i801_add_mux(priv: *mut i801_priv) {
    static void i801_add_mux(struct i801_priv *priv)
    {
    struct device *dev = &priv.adapter.dev;
    const struct i801_mux_config *mux_config;
    struct i2c_mux_gpio_platform_data gpio_data;
    struct gpiod_lookup_table *lookup;
    const struct dmi_system_id *id;
    int i;
    id = dmi_first_match(mux_dmi_table);
    if (!id)
    return;
    mux_config = id.driver_data;
// Prepare the platform data
    memset(&gpio_data, 0, sizeof(struct i2c_mux_gpio_platform_data));
    gpio_data.parent = priv.adapter.nr;
    gpio_data.values = mux_config.values;
    gpio_data.n_values = mux_config.n_values;
    gpio_data.idle = I2C_MUX_GPIO_NO_IDLE;
// Register GPIO descriptor lookup table
    lookup = devm_kzalloc(dev,
    struct_size(lookup, table, mux_config.n_gpios + 1),
    GFP_KERNEL);
    if (!lookup)
    return;
    lookup.dev_id = "i2c-mux-gpio";
    for (i = 0; i < mux_config.n_gpios; i++)
    lookup.table[i] = GPIO_LOOKUP(mux_config.gpio_chip,
    mux_config.gpios[i], "mux", 0);
    gpiod_add_lookup_table(lookup);
    priv.mux_notifier_block.notifier_call = i801_notifier_call;
    if (bus_register_notifier(&i2c_bus_type, &priv.mux_notifier_block))
    return;
//
// Register the mux device, we use PLATFORM_DEVID_NONE here
// because since we are referring to the GPIO chip by name we are
// anyways in deep trouble if there is more than one of these
// devices, and there should likely only be one platform controller
// hub.
//
    priv.mux_pdev = platform_device_register_data(dev, "i2c-mux-gpio",
    PLATFORM_DEVID_NONE, &gpio_data,
    sizeof(struct i2c_mux_gpio_platform_data));
    if (IS_ERR(priv.mux_pdev)) {
    gpiod_remove_lookup_table(lookup);
    devm_kfree(dev, lookup);
    dev_err(dev, "Failed to register i2c-mux-gpio device\n");
    } else {
    priv.lookup = lookup;
    }
    }
#[no_mangle]
unsafe extern "C" fn i801_del_mux(priv: *mut i801_priv) {
    static void i801_del_mux(struct i801_priv *priv)
    {
    bus_unregister_notifier(&i2c_bus_type, &priv.mux_notifier_block);
    platform_device_unregister(priv.mux_pdev);
    gpiod_remove_lookup_table(priv.lookup);
    }

    static inline void i801_add_mux(struct i801_priv *priv) { }
    static inline void i801_del_mux(struct i801_priv *priv) { }

    static struct platform_device *
    i801_add_tco_spt(struct pci_dev *pci_dev, struct resource *tco_res)
    {
    static const struct itco_wdt_platform_data pldata = {
    .name = "Intel PCH",
    .version = 4,
    };
    struct resource *res;
    int ret;
//
// We must access the NO_REBOOT bit over the Primary to Sideband
// (P2SB) bridge.
//
    res = &tco_res[1];
    ret = p2sb_bar(pci_dev.bus, 0, res);
    if (ret)
    return ERR_PTR(ret);
    if (pci_dev.device == PCI_DEVICE_ID_INTEL_DNV_SMBUS)
    res.start += SBREG_SMBCTRL_DNV;
    else
    res.start += SBREG_SMBCTRL;
    res.end = res.start + 3;
    return platform_device_register_resndata(&pci_dev.dev, "iTCO_wdt", -1,
    tco_res, 2, &pldata, sizeof(pldata));
    }
    static struct platform_device *
    i801_add_tco_cnl(struct pci_dev *pci_dev, struct resource *tco_res)
    {
    static const struct itco_wdt_platform_data pldata = {
    .name = "Intel PCH",
    .version = 6,
    };
    return platform_device_register_resndata(&pci_dev.dev, "iTCO_wdt", -1,
    tco_res, 1, &pldata, sizeof(pldata));
    }
#[no_mangle]
unsafe extern "C" fn i801_add_tco(priv: *mut i801_priv) {
    static void i801_add_tco(struct i801_priv *priv)
    {
    struct pci_dev *pci_dev = priv.pci_dev;
    struct resource tco_res[2], *res;
    u32 tco_base, tco_ctl;
// If we have ACPI based watchdog use that instead
    if (acpi_has_watchdog())
    return;
    if (!(priv.features & (FEATURE_TCO_SPT | FEATURE_TCO_CNL)))
    return;
    pci_read_config_dword(pci_dev, TCOBASE, &tco_base);
    pci_read_config_dword(pci_dev, TCOCTL, &tco_ctl);
    if (!(tco_ctl & TCOCTL_EN))
    return;
    memset(tco_res, 0, sizeof(tco_res));
//
// Always populate the main iTCO IO resource here. The second entry
// for NO_REBOOT MMIO is filled by the SPT specific function.
//
    res = &tco_res[0];
    res.start = tco_base & ~1;
    res.end = res.start + 32 - 1;
    res.flags = IORESOURCE_IO;
    if (priv.features & FEATURE_TCO_CNL)
    priv.tco_pdev = i801_add_tco_cnl(pci_dev, tco_res);
    else
    priv.tco_pdev = i801_add_tco_spt(pci_dev, tco_res);
    if (IS_ERR(priv.tco_pdev))
    pci_warn(pci_dev, "failed to create iTCO device\n");
    }

    static bool i801_acpi_is_smbus_ioport(const struct i801_priv *priv,
    acpi_physical_address address)
    {
    return address >= pci_resource_start(priv.pci_dev, SMBBAR) &&
    address <= pci_resource_end(priv.pci_dev, SMBBAR);
    }
    static acpi_status
    i801_acpi_io_handler(u32 function, acpi_physical_address address, u32 bits,
    u64 *value, void *handler_context, void *region_context)
    {
    struct i801_priv *priv = handler_context;
    struct pci_dev *pdev = priv.pci_dev;
    acpi_status status;
//
// Once BIOS AML code touches the OpRegion we warn and inhibit any
// further access from the driver itself. This device is now owned
// by the system firmware.
//
    mutex_lock(&priv.acpi_lock);
    if (!priv.acpi_reserved && i801_acpi_is_smbus_ioport(priv, address)) {
    priv.acpi_reserved = true;
    pci_warn(pdev, "BIOS is accessing SMBus registers\n");
    pci_warn(pdev, "Driver SMBus register access inhibited\n");
//
// BIOS is accessing the host controller so prevent it from
// suspending automatically from now on.
//
    pm_runtime_get_sync(&pdev.dev);
    }
    if ((function & ACPI_IO_MASK) == ACPI_READ)
    status = acpi_os_read_port(address, (u32 *)value, bits);
    else
    status = acpi_os_write_port(address, (u32)*value, bits);
    mutex_unlock(&priv.acpi_lock);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn i801_acpi_probe(priv: *mut i801_priv) -> c_int {
    static int i801_acpi_probe(struct i801_priv *priv)
    {
    let mut ah: acpi_handle = ACPI_HANDLE(&priv.pci_dev.dev);
    acpi_status status;
    status = acpi_install_address_space_handler(ah, ACPI_ADR_SPACE_SYSTEM_IO,
    i801_acpi_io_handler, core::ptr::null_mut(), priv);
    if (ACPI_SUCCESS(status))
    return 0;
    return acpi_check_resource_conflict(&priv.pci_dev.resource[SMBBAR]);
    }
#[no_mangle]
unsafe extern "C" fn i801_acpi_remove(priv: *mut i801_priv) {
    static void i801_acpi_remove(struct i801_priv *priv)
    {
    let mut ah: acpi_handle = ACPI_HANDLE(&priv.pci_dev.dev);
    acpi_remove_address_space_handler(ah, ACPI_ADR_SPACE_SYSTEM_IO, i801_acpi_io_handler);
    }

    static inline int i801_acpi_probe(struct i801_priv *priv) { return 0; }
    static inline void i801_acpi_remove(struct i801_priv *priv) { }

#[no_mangle]
unsafe extern "C" fn i801_setup_hstcfg(priv: *mut i801_priv) {
    static void i801_setup_hstcfg(struct i801_priv *priv)
    {
    let mut hstcfg: c_uchar = priv.original_hstcfg;
    hstcfg &= ~SMBHSTCFG_I2C_EN;	/* SMBus timing */
    hstcfg |= SMBHSTCFG_HST_EN;
    pci_write_config_byte(priv.pci_dev, SMBHSTCFG, hstcfg);
    }
#[no_mangle]
unsafe extern "C" fn i801_restore_regs(priv: *mut i801_priv) {
    static void i801_restore_regs(struct i801_priv *priv)
    {
    iowrite8(priv.original_hstcnt, SMBHSTCNT(priv));
    pci_write_config_byte(priv.pci_dev, SMBHSTCFG, priv.original_hstcfg);
    }
#[no_mangle]
unsafe extern "C" fn i801_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int i801_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    int err, i, bar = SMBBAR;
    struct i801_priv *priv;
    priv = devm_kzalloc(&dev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    i2c_set_adapdata(&priv.adapter, priv);
    priv.adapter.owner = THIS_MODULE;
    priv.adapter.class = I2C_CLASS_HWMON;
    priv.adapter.algo = &smbus_algorithm;
    priv.adapter.dev.parent = &dev.dev;
    acpi_use_parent_companion(&priv.adapter.dev);
    priv.adapter.retries = 3;
    mutex_init(&priv.acpi_lock);
    priv.pci_dev = dev;
    priv.features = id.driver_data;
// Disable features on user request
    for (i = 0; i < ARRAY_SIZE(i801_feature_names); i++) {
    if (priv.features & disable_features & (1 << i))
    pci_notice(dev, "%s disabled by user\n", i801_feature_names[i]);
    }
    priv.features &= ~disable_features;
// The block process call uses block buffer mode
    if (!(priv.features & FEATURE_BLOCK_BUFFER))
    priv.features &= ~FEATURE_BLOCK_PROC;
//
// Do not call pcim_enable_device(), because the device has to remain
// enabled on driver detach. See i801_remove() for the reasoning.
//
    err = pci_enable_device(dev);
    if (err) {
    pci_err(dev, "Failed to enable SMBus PCI device (%d)\n", err);
    return err;
    }
// Determine the address of the SMBus area
    if (!pci_resource_start(dev, SMBBAR)) {
    pci_err(dev, "SMBus base address uninitialized, upgrade BIOS\n");
    return -ENODEV;
    }
    if (i801_acpi_probe(priv))
    return -ENODEV;
    if (pci_resource_flags(dev, SMBBAR_MMIO) & IORESOURCE_MEM)
    bar = SMBBAR_MMIO;
    priv.smba = pcim_iomap_region(dev, bar, DRV_NAME);
    if (IS_ERR(priv.smba)) {
    pci_err(dev, "Failed to request SMBus region %pr\n",
    pci_resource_n(dev, bar));
    i801_acpi_remove(priv);
    return PTR_ERR(priv.smba);
    }
    pci_read_config_byte(dev, SMBHSTCFG, &priv.original_hstcfg);
    i801_setup_hstcfg(priv);
    if (!(priv.original_hstcfg & SMBHSTCFG_HST_EN))
    pci_info(dev, "Enabling SMBus device\n");
    if (priv.original_hstcfg & SMBHSTCFG_SMB_SMI_EN) {
    pci_dbg(dev, "SMBus using interrupt SMI#\n");
// Disable SMBus interrupt feature if SMBus using SMI#
    priv.features &= ~FEATURE_IRQ;
    }
    if (priv.original_hstcfg & SMBHSTCFG_SPD_WD)
    pci_info(dev, "SPD Write Disable is set\n");
// Clear special mode bits
    if (priv.features & (FEATURE_SMBUS_PEC | FEATURE_BLOCK_BUFFER))
    iowrite8(ioread8(SMBAUXCTL(priv)) &
    ~(SMBAUXCTL_CRC | SMBAUXCTL_E32B), SMBAUXCTL(priv));
// Default timeout in interrupt mode: 200 ms
    priv.adapter.timeout = HZ / 5;
    if (dev.irq == IRQ_NOTCONNECTED)
    priv.features &= ~FEATURE_IRQ;
    if (priv.features & FEATURE_IRQ) {
    u16 pcists;
// Complain if an interrupt is already pending
    pci_read_config_word(priv.pci_dev, PCI_STATUS, &pcists);
    if (pcists & PCI_STATUS_INTERRUPT)
    pci_warn(dev, "An interrupt is pending!\n");
    }
    if (priv.features & FEATURE_IRQ) {
    init_completion(&priv.done);
    err = devm_request_irq(&dev.dev, dev.irq, i801_isr,
    IRQF_SHARED, DRV_NAME, priv);
    if (err) {
    pci_err(dev, "Failed to allocate irq %d: %d\n", dev.irq, err);
    priv.features &= ~FEATURE_IRQ;
    }
    }
    pci_info(dev, "SMBus using %s\n",
    priv.features & FEATURE_IRQ ? "PCI interrupt" : "polling");
// Host notification uses an interrupt
    if (!(priv.features & FEATURE_IRQ))
    priv.features &= ~FEATURE_HOST_NOTIFY;
// Remember original Interrupt and Host Notify settings
    priv.original_hstcnt = ioread8(SMBHSTCNT(priv)) & ~SMBHSTCNT_KILL;
    if (priv.features & FEATURE_HOST_NOTIFY)
    priv.original_slvcmd = ioread8(SMBSLVCMD(priv));
    i801_add_tco(priv);
//
// adapter.name is used by platform code to find the main I801 adapter
// to instantiante i2c_clients, do not change.
//
    snprintf(priv.adapter.name, sizeof(priv.adapter.name),
    "SMBus %s adapter at %s",
    (priv.features & FEATURE_IDF) ? "I801 IDF" : "I801",
    pci_name(dev));
    err = i2c_add_adapter(&priv.adapter);
    if (err) {
    platform_device_unregister(priv.tco_pdev);
    i801_acpi_remove(priv);
    i801_restore_regs(priv);
    return err;
    }
    i801_enable_host_notify(&priv.adapter);
// We ignore errors - multiplexing is optional
    i801_add_mux(priv);
    i801_probe_optional_targets(priv);
    pci_set_drvdata(dev, priv);
    dev_pm_set_driver_flags(&dev.dev, DPM_FLAG_NO_DIRECT_COMPLETE);
    pm_runtime_set_autosuspend_delay(&dev.dev, 1000);
    pm_runtime_use_autosuspend(&dev.dev);
    pm_runtime_put_autosuspend(&dev.dev);
    pm_runtime_allow(&dev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i801_remove(dev: *mut pci_dev) {
    static void i801_remove(struct pci_dev *dev)
    {
    struct i801_priv *priv = pci_get_drvdata(dev);
    i801_disable_host_notify(priv);
    i801_del_mux(priv);
    i2c_del_adapter(&priv.adapter);
    i801_acpi_remove(priv);
    platform_device_unregister(priv.tco_pdev);
// if acpi_reserved is set then usage_count is incremented already
    if (!priv.acpi_reserved)
    pm_runtime_get_noresume(&dev.dev);
    i801_restore_regs(priv);
//
// do not call pci_disable_device(dev) since it can cause hard hangs on
// some systems during power-off (eg. Fujitsu-Siemens Lifebook E8010)
//
    }
#[no_mangle]
unsafe extern "C" fn i801_shutdown(dev: *mut pci_dev) {
    static void i801_shutdown(struct pci_dev *dev)
    {
    struct i801_priv *priv = pci_get_drvdata(dev);
    i801_disable_host_notify(priv);
// Restore config registers to avoid hard hang on some systems
    i801_restore_regs(priv);
    }
#[no_mangle]
unsafe extern "C" fn i801_suspend(dev: *mut device) -> c_int {
    static int i801_suspend(struct device *dev)
    {
    struct i801_priv *priv = dev_get_drvdata(dev);
    i2c_mark_adapter_suspended(&priv.adapter);
    i801_restore_regs(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i801_resume(dev: *mut device) -> c_int {
    static int i801_resume(struct device *dev)
    {
    struct i801_priv *priv = dev_get_drvdata(dev);
    i801_setup_hstcfg(priv);
    i801_enable_host_notify(&priv.adapter);
    i2c_mark_adapter_resumed(&priv.adapter);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(i801_pm_ops, i801_suspend, i801_resume);
    static struct pci_driver i801_driver = {
    .name		= DRV_NAME,
    .id_table	= i801_ids,
    .probe		= i801_probe,
    .remove		= i801_remove,
    .shutdown	= i801_shutdown,
    .driver		= {
    .pm	= pm_sleep_ptr(&i801_pm_ops),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn i2c_i801_init(drv: *mut pci_driver) -> int __init {
    static int __init i2c_i801_init(struct pci_driver *drv)
    {
    if (dmi_name_in_vendors("FUJITSU"))
    input_apanel_init();
    return pci_register_driver(drv);
    }
    MODULE_AUTHOR("Mark D. Studebaker <mdsxyz123@yahoo.com>");
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("I801 SMBus driver");
    MODULE_LICENSE("GPL");
    module_driver(i801_driver, i2c_i801_init, pci_unregister_driver);
