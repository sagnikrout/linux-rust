
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

use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [u8; 2],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);
