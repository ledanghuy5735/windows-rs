#[doc(hidden)]
#[repr(transparent)]
pub struct IBasicProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBasicProperties {
    type Vtable = IBasicProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IBasicProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd05d55db_785e_4a66_be02_9beec58aea81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateModified: usize,
    #[cfg(feature = "Foundation")]
    pub ItemDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDocumentProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDocumentProperties {
    type Vtable = IDocumentProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IDocumentProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eab19bc_1821_4923_b4a9_0aea404d0070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocumentProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Author: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeotagHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGeotagHelperStatics {
    type Vtable = IGeotagHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGeotagHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41493244_2524_4655_86a6_ed16f5fc716b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeotagHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetGeotagAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetGeotagAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub SetGeotagFromGeolocatorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, geolocator: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    SetGeotagFromGeolocatorAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub SetGeotagAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, geopoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    SetGeotagAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IImageProperties {
    type Vtable = IImageProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IImageProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x523c9424_fcff_4275_afee_ecdb9ab47973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    #[cfg(feature = "Foundation")]
    pub DateTaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateTaken: usize,
    #[cfg(feature = "Foundation")]
    pub SetDateTaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDateTaken: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Latitude: usize,
    #[cfg(feature = "Foundation")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Longitude: usize,
    pub CameraManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCameraManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CameraModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCameraModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoOrientation) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PeopleNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PeopleNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMusicProperties {
    type Vtable = IMusicProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IMusicProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc8aab62_66ec_419a_bc5d_ca65a4cb46da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlbum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genre: usize,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Composers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Composers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Conductors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Conductors: usize,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Producers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Producers: usize,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Writers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Writers: usize,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemContentProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageItemContentProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05294bad_bc38_48bf_85d7_770e0e2ae0ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemContentProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetMusicPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMusicPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetVideoPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetImagePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetImagePropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDocumentPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDocumentPropertiesAsync: usize,
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct IStorageItemExtraProperties(::windows::core::IUnknown);
impl IStorageItemExtraProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IStorageItemExtraProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStorageItemExtraProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemExtraProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemExtraProperties {}
impl ::core::fmt::Debug for IStorageItemExtraProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemExtraProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemExtraProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c54361b2-54cd-432b-bdbc-4b19c4b470d7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStorageItemExtraProperties {
    type Vtable = IStorageItemExtraProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageItemExtraProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc54361b2_54cd_432b_bdbc_4b19c4b470d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemExtraProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SavePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestosave: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavePropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SavePropertiesAsyncOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SavePropertiesAsyncOverloadDefault: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThumbnailProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IThumbnailProperties {
    type Vtable = IThumbnailProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IThumbnailProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x693dd42f_dbe7_49b5_b3b3_2893ac5d3423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OriginalWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OriginalHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReturnedSmallerCachedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ThumbnailType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVideoProperties {
    type Vtable = IVideoProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IVideoProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x719ae507_68de_4db8_97de_49998c059f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Latitude: usize,
    #[cfg(feature = "Foundation")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Longitude: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Producers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Producers: usize,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Writers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Writers: usize,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Directors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Directors: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoOrientation) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct BasicProperties(::windows::core::IUnknown);
impl BasicProperties {
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateModified(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DateModified)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BasicProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BasicProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BasicProperties {}
impl ::core::fmt::Debug for BasicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BasicProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BasicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.BasicProperties;{d05d55db-785e-4a66-be02-9beec58aea81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BasicProperties {
    type Vtable = IBasicProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for BasicProperties {
    const IID: ::windows::core::GUID = <IBasicProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BasicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.BasicProperties";
}
::windows::core::interface_hierarchy!(BasicProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: BasicProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BasicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &BasicProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BasicProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BasicProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct DocumentProperties(::windows::core::IUnknown);
impl DocumentProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Author(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Author)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Keywords)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DocumentProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DocumentProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DocumentProperties {}
impl ::core::fmt::Debug for DocumentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DocumentProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DocumentProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.DocumentProperties;{7eab19bc-1821-4923-b4a9-0aea404d0070})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DocumentProperties {
    type Vtable = IDocumentProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for DocumentProperties {
    const IID: ::windows::core::GUID = <IDocumentProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DocumentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.DocumentProperties";
}
::windows::core::interface_hierarchy!(DocumentProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: DocumentProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DocumentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &DocumentProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&DocumentProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DocumentProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
pub struct GeotagHelper;
impl GeotagHelper {
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetGeotagAsync<P0, E0>(file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Geolocation::Geopoint>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGeotagAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn SetGeotagFromGeolocatorAsync<P0, E0>(file: P0, geolocator: &super::super::Devices::Geolocation::Geolocator) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetGeotagFromGeolocatorAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(geolocator), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn SetGeotagAsync<P0, E0>(file: P0, geopoint: &super::super::Devices::Geolocation::Geopoint) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGeotagHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetGeotagAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(geopoint), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeotagHelperStatics<R, F: FnOnce(&IGeotagHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GeotagHelper, IGeotagHelperStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GeotagHelper {
    const NAME: &'static str = "Windows.Storage.FileProperties.GeotagHelper";
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct ImageProperties(::windows::core::IUnknown);
impl ImageProperties {
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRating)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Keywords)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateTaken(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DateTaken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDateTaken(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDateTaken)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Latitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Longitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraManufacturer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCameraManufacturer(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCameraManufacturer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CameraModel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCameraModel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCameraModel)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PhotoOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orientation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PeopleNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeopleNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ImageProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageProperties {}
impl ::core::fmt::Debug for ImageProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.ImageProperties;{523c9424-fcff-4275-afee-ecdb9ab47973})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ImageProperties {
    type Vtable = IImageProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for ImageProperties {
    const IID: ::windows::core::GUID = <IImageProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.ImageProperties";
}
::windows::core::interface_hierarchy!(ImageProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ImageProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct MusicProperties(::windows::core::IUnknown);
impl MusicProperties {
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Album)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAlbum(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlbum)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Artist)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetArtist)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genre(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Genre)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrackNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrackNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTrackNumber)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRating)(::windows::core::Vtable::as_raw(this), value).ok() }
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
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bitrate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlbumArtist)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAlbumArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlbumArtist)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Composers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Composers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Conductors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Conductors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subtitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubtitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Producers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Publisher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPublisher(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPublisher)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Writers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Year(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Year)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetYear(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetYear)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MusicProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MusicProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MusicProperties {}
impl ::core::fmt::Debug for MusicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MusicProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MusicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.MusicProperties;{bc8aab62-66ec-419a-bc5d-ca65a4cb46da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MusicProperties {
    type Vtable = IMusicProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for MusicProperties {
    const IID: ::windows::core::GUID = <IMusicProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MusicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.MusicProperties";
}
::windows::core::interface_hierarchy!(MusicProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: MusicProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MusicProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &MusicProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MusicProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MusicProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct StorageItemContentProperties(::windows::core::IUnknown);
impl StorageItemContentProperties {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMusicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MusicProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMusicPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVideoPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VideoProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVideoPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetImagePropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetImagePropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDocumentPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DocumentProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDocumentPropertiesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorageItemContentProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageItemContentProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemContentProperties {}
impl ::core::fmt::Debug for StorageItemContentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemContentProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageItemContentProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemContentProperties;{05294bad-bc38-48bf-85d7-770e0e2ae0ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorageItemContentProperties {
    type Vtable = IStorageItemContentProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for StorageItemContentProperties {
    const IID: ::windows::core::GUID = <IStorageItemContentProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageItemContentProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemContentProperties";
}
::windows::core::interface_hierarchy!(StorageItemContentProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemContentProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemContentProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemContentProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&StorageItemContentProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemContentProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct StorageItemThumbnail(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl StorageItemThumbnail {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0, E0>(&self, buffer: P0, count: u32, options: super::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OriginalWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OriginalHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReturnedSmallerCachedSize(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReturnedSmallerCachedSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<ThumbnailType> {
        let this = &::windows::core::Interface::cast::<IThumbnailProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for StorageItemThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for StorageItemThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for StorageItemThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for StorageItemThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StorageItemThumbnail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.StorageItemThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Vtable for StorageItemThumbnail {
    type Vtable = super::Streams::IRandomAccessStreamWithContentType_Vtbl;
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StorageItemThumbnail {
    const IID: ::windows::core::GUID = <super::Streams::IRandomAccessStreamWithContentType as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for StorageItemThumbnail {
    const NAME: &'static str = "Windows.Storage.FileProperties.StorageItemThumbnail";
}
#[cfg(feature = "Storage_Streams")]
::windows::core::interface_hierarchy!(StorageItemThumbnail, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::Streams::IContentTypeProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::Streams::IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::Streams::IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::Streams::IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for super::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageItemThumbnail> for ::windows::core::InParam<super::Streams::IRandomAccessStreamWithContentType> {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemThumbnail) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
pub struct VideoProperties(::windows::core::IUnknown);
impl VideoProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrievePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavePropertiesAsync<P0, E0>(&self, propertiestosave: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestosave.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavePropertiesAsyncOverloadDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Rating(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rating)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRating(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRating)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Keywords)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Latitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Latitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Longitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Longitude)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subtitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubtitle)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Producers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Producers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Publisher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPublisher(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPublisher)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Writers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Writers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Year(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Year)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetYear(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetYear)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bitrate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Directors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Directors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<VideoOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orientation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VideoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoProperties {}
impl ::core::fmt::Debug for VideoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.FileProperties.VideoProperties;{719ae507-68de-4db8-97de-49998c059f2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VideoProperties {
    type Vtable = IVideoProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for VideoProperties {
    const IID: ::windows::core::GUID = <IVideoProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.VideoProperties";
}
::windows::core::interface_hierarchy!(VideoProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoProperties> for IStorageItemExtraProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&VideoProperties> for ::windows::core::InParam<IStorageItemExtraProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoOrientation(pub i32);
impl PhotoOrientation {
    pub const Unspecified: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const FlipHorizontal: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
    pub const FlipVertical: Self = Self(4i32);
    pub const Transpose: Self = Self(5i32);
    pub const Rotate270: Self = Self(6i32);
    pub const Transverse: Self = Self(7i32);
    pub const Rotate90: Self = Self(8i32);
}
impl ::core::marker::Copy for PhotoOrientation {}
impl ::core::clone::Clone for PhotoOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PhotoOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyPrefetchOptions(pub u32);
impl PropertyPrefetchOptions {
    pub const None: Self = Self(0u32);
    pub const MusicProperties: Self = Self(1u32);
    pub const VideoProperties: Self = Self(2u32);
    pub const ImageProperties: Self = Self(4u32);
    pub const DocumentProperties: Self = Self(8u32);
    pub const BasicProperties: Self = Self(16u32);
}
impl ::core::marker::Copy for PropertyPrefetchOptions {}
impl ::core::clone::Clone for PropertyPrefetchOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyPrefetchOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyPrefetchOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyPrefetchOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyPrefetchOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PropertyPrefetchOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PropertyPrefetchOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PropertyPrefetchOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PropertyPrefetchOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PropertyPrefetchOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyPrefetchOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.PropertyPrefetchOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ThumbnailMode(pub i32);
impl ThumbnailMode {
    pub const PicturesView: Self = Self(0i32);
    pub const VideosView: Self = Self(1i32);
    pub const MusicView: Self = Self(2i32);
    pub const DocumentsView: Self = Self(3i32);
    pub const ListView: Self = Self(4i32);
    pub const SingleItem: Self = Self(5i32);
}
impl ::core::marker::Copy for ThumbnailMode {}
impl ::core::clone::Clone for ThumbnailMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ThumbnailOptions(pub u32);
impl ThumbnailOptions {
    pub const None: Self = Self(0u32);
    pub const ReturnOnlyIfCached: Self = Self(1u32);
    pub const ResizeThumbnail: Self = Self(2u32);
    pub const UseCurrentScale: Self = Self(4u32);
}
impl ::core::marker::Copy for ThumbnailOptions {}
impl ::core::clone::Clone for ThumbnailOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ThumbnailOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ThumbnailOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ThumbnailOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ThumbnailOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ThumbnailOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ThumbnailType(pub i32);
impl ThumbnailType {
    pub const Image: Self = Self(0i32);
    pub const Icon: Self = Self(1i32);
}
impl ::core::marker::Copy for ThumbnailType {}
impl ::core::clone::Clone for ThumbnailType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ThumbnailType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ThumbnailType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ThumbnailType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThumbnailType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThumbnailType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.ThumbnailType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_FileProperties\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoOrientation(pub i32);
impl VideoOrientation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(90i32);
    pub const Rotate180: Self = Self(180i32);
    pub const Rotate270: Self = Self(270i32);
}
impl ::core::marker::Copy for VideoOrientation {}
impl ::core::clone::Clone for VideoOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileProperties.VideoOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
