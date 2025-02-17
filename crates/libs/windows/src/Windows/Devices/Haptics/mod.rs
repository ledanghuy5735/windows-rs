#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownSimpleHapticsControllerWaveformsStatics {
    type Vtable = IKnownSimpleHapticsControllerWaveformsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef7_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Click: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BuzzContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RumbleContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Press: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownSimpleHapticsControllerWaveformsStatics2 {
    type Vtable = IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d24c27_b79d_510a_bf79_ff6d49173e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BrushContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ChiselMarkerContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub EraserContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GalaxyPenContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Hover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub InkContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub MarkerContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub PencilContinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Success: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleHapticsController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISimpleHapticsController {
    type Vtable = ISimpleHapticsController_Vtbl;
}
unsafe impl ::windows::core::Interface for ISimpleHapticsController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef9_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFeedback: usize,
    pub IsIntensitySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPlayCountSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPlayDurationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReplayPauseIntervalSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub StopFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendHapticFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendHapticFeedbackWithIntensity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: *mut ::core::ffi::c_void, intensity: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendHapticFeedbackForDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: *mut ::core::ffi::c_void, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendHapticFeedbackForDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SendHapticFeedbackForPlayCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: *mut ::core::ffi::c_void, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendHapticFeedbackForPlayCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleHapticsControllerFeedback(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISimpleHapticsControllerFeedback {
    type Vtable = ISimpleHapticsControllerFeedback_Vtbl;
}
unsafe impl ::windows::core::Interface for ISimpleHapticsControllerFeedback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef8_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsControllerFeedback_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Waveform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IVibrationDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f21a3e_8844_47ff_b312_06185a3844da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVibrationDeviceStatics {
    type Vtable = IVibrationDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IVibrationDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e2eded_2290_4ac9_8eb3_1a84122eb71c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
pub struct KnownSimpleHapticsControllerWaveforms;
impl KnownSimpleHapticsControllerWaveforms {
    pub fn Click() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Click)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BuzzContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BuzzContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RumbleContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RumbleContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Press() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Press)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Release() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Release)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BrushContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BrushContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ChiselMarkerContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChiselMarkerContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EraserContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EraserContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Error() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GalaxyPenContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GalaxyPenContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Hover() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hover)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn InkContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InkContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MarkerContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkerContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PencilContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PencilContinuous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Success() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Success)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics2<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownSimpleHapticsControllerWaveforms {
    const NAME: &'static str = "Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
}
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
#[repr(transparent)]
pub struct SimpleHapticsController(::windows::core::IUnknown);
impl SimpleHapticsController {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFeedback(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFeedback)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsIntensitySupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIntensitySupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPlayCountSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPlayCountSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPlayDurationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPlayDurationSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReplayPauseIntervalSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReplayPauseIntervalSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StopFeedback(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopFeedback)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SendHapticFeedback(&self, feedback: &SimpleHapticsControllerFeedback) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendHapticFeedback)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feedback)).ok() }
    }
    pub fn SendHapticFeedbackWithIntensity(&self, feedback: &SimpleHapticsControllerFeedback, intensity: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendHapticFeedbackWithIntensity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feedback), intensity).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendHapticFeedbackForDuration(&self, feedback: &SimpleHapticsControllerFeedback, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendHapticFeedbackForDuration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feedback), intensity, playduration).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendHapticFeedbackForPlayCount(&self, feedback: &SimpleHapticsControllerFeedback, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendHapticFeedbackForPlayCount)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(feedback), intensity, playcount, replaypauseinterval).ok() }
    }
}
impl ::core::clone::Clone for SimpleHapticsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleHapticsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleHapticsController {}
impl ::core::fmt::Debug for SimpleHapticsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleHapticsController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleHapticsController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.SimpleHapticsController;{3d577ef9-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SimpleHapticsController {
    type Vtable = ISimpleHapticsController_Vtbl;
}
unsafe impl ::windows::core::Interface for SimpleHapticsController {
    const IID: ::windows::core::GUID = <ISimpleHapticsController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsController";
}
::windows::core::interface_hierarchy!(SimpleHapticsController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SimpleHapticsController {}
unsafe impl ::core::marker::Sync for SimpleHapticsController {}
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
#[repr(transparent)]
pub struct SimpleHapticsControllerFeedback(::windows::core::IUnknown);
impl SimpleHapticsControllerFeedback {
    pub fn Waveform(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Waveform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SimpleHapticsControllerFeedback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleHapticsControllerFeedback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleHapticsControllerFeedback {}
impl ::core::fmt::Debug for SimpleHapticsControllerFeedback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleHapticsControllerFeedback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleHapticsControllerFeedback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.SimpleHapticsControllerFeedback;{3d577ef8-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SimpleHapticsControllerFeedback {
    type Vtable = ISimpleHapticsControllerFeedback_Vtbl;
}
unsafe impl ::windows::core::Interface for SimpleHapticsControllerFeedback {
    const IID: ::windows::core::GUID = <ISimpleHapticsControllerFeedback as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
}
::windows::core::interface_hierarchy!(SimpleHapticsControllerFeedback, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SimpleHapticsControllerFeedback {}
unsafe impl ::core::marker::Sync for SimpleHapticsControllerFeedback {}
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
#[repr(transparent)]
pub struct VibrationDevice(::windows::core::IUnknown);
impl VibrationDevice {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SimpleHapticsController)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VibrationDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VibrationDevice {}
impl ::core::fmt::Debug for VibrationDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VibrationDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.VibrationDevice;{40f21a3e-8844-47ff-b312-06185a3844da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for VibrationDevice {
    const IID: ::windows::core::GUID = <IVibrationDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.VibrationDevice";
}
::windows::core::interface_hierarchy!(VibrationDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VibrationDevice {}
unsafe impl ::core::marker::Sync for VibrationDevice {}
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrationAccessStatus {}
impl ::core::clone::Clone for VibrationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VibrationAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VibrationAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for VibrationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VibrationAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Haptics.VibrationAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
