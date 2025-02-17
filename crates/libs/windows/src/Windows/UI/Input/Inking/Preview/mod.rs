#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
}
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPalmRejectionDelayZonePreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisual: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: *mut ::core::ffi::c_void, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisualWithViewportClip: usize,
}
#[doc = "*Required features: `\"UI_Input_Inking_Preview\"`*"]
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(::windows::core::IUnknown);
impl PalmRejectionDelayZonePreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisual<P0>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::Composition::Visual>>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForVisual)(::windows::core::Vtable::as_raw(this), inputpanelvisual.into().abi(), inputpanelrect, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Composition\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateForVisualWithViewportClip<P0, P1>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: P1, viewportrect: super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::Composition::Visual>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::Composition::Visual>>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForVisualWithViewportClip)(::windows::core::Vtable::as_raw(this), inputpanelvisual.into().abi(), inputpanelrect, viewportvisual.into().abi(), viewportrect, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PalmRejectionDelayZonePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PalmRejectionDelayZonePreview {}
impl ::core::fmt::Debug for PalmRejectionDelayZonePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PalmRejectionDelayZonePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
}
unsafe impl ::windows::core::Interface for PalmRejectionDelayZonePreview {
    const IID: ::windows::core::GUID = <IPalmRejectionDelayZonePreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
::windows::core::interface_hierarchy!(PalmRejectionDelayZonePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PalmRejectionDelayZonePreview) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PalmRejectionDelayZonePreview> for super::super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PalmRejectionDelayZonePreview> for ::windows::core::InParam<super::super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Sync for PalmRejectionDelayZonePreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
