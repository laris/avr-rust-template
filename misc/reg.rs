use core::{cmp, convert, marker, ops};

/// A value that a register can store.
///
/// All registers are either `u8` or `u16`.
pub trait RegisterValue : Copy + Clone +
               ops::BitAnd<Output=Self> +
               ops::BitAndAssign +
               ops::BitOr<Output=Self> +
               ops::BitOrAssign +
               ops::BitXor<Output=Self> +
               ops::BitXorAssign +
               ops::Not<Output=Self> +
               cmp::PartialEq + cmp::Eq +
               cmp::PartialOrd + cmp::Ord +
               convert::From<u8> {
}

/// Single bit of register
pub trait RegisterBit {
    type  T: RegisterValue;
    const BITMASK: *mut Self::T;

    /// Sets a bitmask in a register.
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set_mask_raw(mask: Self::T) {
        unsafe {
            *Self::BITMASK |= mask;
        }
    }

    /// Set the bit to high.
    #[inline(always)]
    fn set() {
        Self::T::set_mask_raw(Self::BITMASK);
    }

    /// Clears a bitmask from a register.
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset_mask_raw(mask: Self::T) {
        unsafe {
                core::ptr::write_volatile(
                            Self::BITMASK, 
                            core::ptr::read_volatile(Self::BITMASK) & !mask)
        }
    }

    /// Set the bit to low.
    #[inline(always)]
    fn clear() {
        Self::T::unset_mask_raw(Self::BITMASK);
    }

    /// Toggles a mask in the register.
    /// This is equivalent to `r ^= mask`.
    #[inline(always)]
    fn toggle_raw(mask: Self::T) {
        unsafe {
                core::ptr::write_volatile(
                            Self::ADDRESS, 
                            core::ptr::read_volatile(Self::ADDRESS) ^ mask)
        }
    }

    /// Toggles the bit.
    #[inline(always)]
    fn toggle() {
        // FIXME: We can optimise this on post-2006 AVRs.
        // http://www.avrfreaks.net/forum/toggle-state-output-pin
        // set(Self::T, Self::BITMASK);
        Self::T::toggle_raw(Self::BITMASK);
    }

    /// Check if the bit is currently high.
    #[inline(always)]
    fn is_high() -> bool {
        Self::T::is_mask_set_raw(Self::BITMASK)
    }

    /// Checks if the bit is currently low.
    #[inline(always)]
    fn is_low() -> bool {
        Self::T::is_clear_raw(Self::BITMASK)
    }
}