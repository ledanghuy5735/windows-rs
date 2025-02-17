#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemProtectionStatics {
    type Vtable = ISystemProtectionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemProtectionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemProtectionUnlockStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System\"`*"]
pub struct SystemProtection;
impl SystemProtection {
    pub fn ScreenLocked() -> ::windows::core::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScreenLocked)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RequestScreenUnlock() -> ::windows::core::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RequestScreenUnlock)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemProtectionStatics<R, F: FnOnce(&ISystemProtectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SystemProtection, ISystemProtectionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemProtectionUnlockStatics<R, F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SystemProtection, ISystemProtectionUnlockStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
