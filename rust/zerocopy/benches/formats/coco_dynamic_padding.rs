
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
#[derive(TryFromBytes, KnownLayout, Immutable)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

#[derive(FromBytes, KnownLayout, Immutable, SplitAt)]
#[repr(C, align(4))]
pub struct Packet<Magic> {
    magic_number: Magic,
    milk: u8,
    mug_size: u8,
    temperature: [u8; 5],
    marshmallows: [[u8; 3]],
}

/// A packet begining with the magic number `0xC0C0`.
pub type CocoPacket = Packet<C0C0>;

/// A packet beginning with any two initialized bytes.
pub type LocoPacket = Packet<[u8; 2]>;
