#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialGraphInteropFrameOfReferencePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8271b23_735f_5729_a98e_e64ed189abc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropFrameOfReferencePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NodeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CoordinateSystemToNodeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CoordinateSystemToNodeTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialGraphInteropPreviewStatics {
    type Vtable = ISpatialGraphInteropPreviewStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialGraphInteropPreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc042644c_20d8_4ed0_aef7_6805b8e53f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateCoordinateSystemForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPositionAndOrientation: usize,
    pub CreateLocatorForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialGraphInteropPreviewStatics2 {
    type Vtable = ISpatialGraphInteropPreviewStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialGraphInteropPreviewStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2490b15f_6cbd_4b1e_b765_31e462a32df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryCreateFrameOfReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPositionAndOrientation: usize,
}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
#[repr(transparent)]
pub struct SpatialGraphInteropFrameOfReferencePreview(::windows::core::IUnknown);
impl SpatialGraphInteropFrameOfReferencePreview {
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CoordinateSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NodeId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NodeId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CoordinateSystemToNodeTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CoordinateSystemToNodeTransform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialGraphInteropFrameOfReferencePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialGraphInteropFrameOfReferencePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialGraphInteropFrameOfReferencePreview {}
impl ::core::fmt::Debug for SpatialGraphInteropFrameOfReferencePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialGraphInteropFrameOfReferencePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview;{a8271b23-735f-5729-a98e-e64ed189abc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialGraphInteropFrameOfReferencePreview {
    const IID: ::windows::core::GUID = <ISpatialGraphInteropFrameOfReferencePreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview";
}
::windows::core::interface_hierarchy!(SpatialGraphInteropFrameOfReferencePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialGraphInteropFrameOfReferencePreview {}
unsafe impl ::core::marker::Sync for SpatialGraphInteropFrameOfReferencePreview {}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
pub struct SpatialGraphInteropPreview;
impl SpatialGraphInteropPreview {
    pub fn CreateCoordinateSystemForNode(nodeid: ::windows::core::GUID) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCoordinateSystemForNode)(::windows::core::Vtable::as_raw(this), nodeid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPosition(nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCoordinateSystemForNodeWithPosition)(::windows::core::Vtable::as_raw(this), nodeid, relativeposition, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPositionAndOrientation(nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCoordinateSystemForNodeWithPositionAndOrientation)(::windows::core::Vtable::as_raw(this), nodeid, relativeposition, relativeorientation, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateLocatorForNode(nodeid: ::windows::core::GUID) -> ::windows::core::Result<super::SpatialLocator> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLocatorForNode)(::windows::core::Vtable::as_raw(this), nodeid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryCreateFrameOfReference(coordinatesystem: &super::SpatialCoordinateSystem) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFrameOfReference)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPosition(coordinatesystem: &super::SpatialCoordinateSystem, relativeposition: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFrameOfReferenceWithPosition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), relativeposition, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPositionAndOrientation(coordinatesystem: &super::SpatialCoordinateSystem, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFrameOfReferenceWithPositionAndOrientation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), relativeposition, relativeorientation, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics2<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SpatialGraphInteropPreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
