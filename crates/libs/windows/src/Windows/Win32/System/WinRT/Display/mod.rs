#[doc = "*Required features: `\"Win32_System_WinRT_Display\"`*"]
#[repr(transparent)]
pub struct IDisplayDeviceInterop(::windows::core::IUnknown);
impl IDisplayDeviceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0>(&self, pobject: P0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), psecurityattributes, access, ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0>(&self, nthandle: P0, riid: ::windows::core::GUID) -> ::windows::core::Result<*mut ::core::ffi::c_void>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), ::core::mem::transmute(riid), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDisplayDeviceInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDisplayDeviceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayDeviceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayDeviceInterop {}
impl ::core::fmt::Debug for IDisplayDeviceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayDeviceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IDisplayDeviceInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64338358_366a_471b_bd56_dd8ef48e439b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: *mut ::core::ffi::c_void, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenSharedHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Display\"`*"]
#[repr(transparent)]
pub struct IDisplayPathInterop(::windows::core::IUnknown);
impl IDisplayPathInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSourcePresentationHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSourceId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDisplayPathInterop, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDisplayPathInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayPathInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayPathInterop {}
impl ::core::fmt::Debug for IDisplayPathInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayPathInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDisplayPathInterop {
    type Vtable = IDisplayPathInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IDisplayPathInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSourcePresentationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSourcePresentationHandle: usize,
    pub GetSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
