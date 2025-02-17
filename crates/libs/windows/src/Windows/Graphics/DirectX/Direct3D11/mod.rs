#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice(::windows::core::IUnknown);
impl IDirect3DDevice {
    pub fn Trim(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Trim)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IDirect3DDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IDirect3DDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DDevice> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IDirect3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice {}
impl ::core::fmt::Debug for IDirect3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDirect3DDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a37624ab-8d5f-4650-9d3e-9eae3d9bc670}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDirect3DDevice {
    type Vtable = IDirect3DDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3DDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37624ab_8d5f_4650_9d3e_9eae3d9bc670);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
pub struct IDirect3DSurface(::windows::core::IUnknown);
impl IDirect3DSurface {
    pub fn Description(&self) -> ::windows::core::Result<Direct3DSurfaceDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IDirect3DSurface, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IDirect3DSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DSurface> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DSurface) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IDirect3DSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSurface {}
impl ::core::fmt::Debug for IDirect3DSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDirect3DSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0bf4a146-13c1-4694-bee3-7abf15eaf586}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDirect3DSurface {
    type Vtable = IDirect3DSurface_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirect3DSurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bf4a146_13c1_4694_bee3_7abf15eaf586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Self = Self(1u32);
    pub const IndexBuffer: Self = Self(2u32);
    pub const ConstantBuffer: Self = Self(4u32);
    pub const ShaderResource: Self = Self(8u32);
    pub const StreamOutput: Self = Self(16u32);
    pub const RenderTarget: Self = Self(32u32);
    pub const DepthStencil: Self = Self(64u32);
    pub const UnorderedAccess: Self = Self(128u32);
    pub const Decoder: Self = Self(512u32);
    pub const VideoEncoder: Self = Self(1024u32);
}
impl ::core::marker::Copy for Direct3DBindings {}
impl ::core::clone::Clone for Direct3DBindings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Direct3DBindings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Direct3DBindings {
    type Abi = Self;
}
impl ::core::fmt::Debug for Direct3DBindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DBindings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for Direct3DBindings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for Direct3DBindings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for Direct3DBindings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for Direct3DBindings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for Direct3DBindings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for Direct3DBindings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DBindings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Self = Self(0i32);
    pub const Immutable: Self = Self(1i32);
    pub const Dynamic: Self = Self(2i32);
    pub const Staging: Self = Self(3i32);
}
impl ::core::marker::Copy for Direct3DUsage {}
impl ::core::clone::Clone for Direct3DUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Direct3DUsage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Direct3DUsage {
    type Abi = Self;
}
impl ::core::fmt::Debug for Direct3DUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Direct3DUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DUsage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
pub struct Direct3DMultisampleDescription {
    pub Count: i32,
    pub Quality: i32,
}
impl ::core::marker::Copy for Direct3DMultisampleDescription {}
impl ::core::clone::Clone for Direct3DMultisampleDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Direct3DMultisampleDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DMultisampleDescription").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
unsafe impl ::windows::core::Abi for Direct3DMultisampleDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Direct3DMultisampleDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Direct3DMultisampleDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for Direct3DMultisampleDescription {}
impl ::core::default::Default for Direct3DMultisampleDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
pub struct Direct3DSurfaceDescription {
    pub Width: i32,
    pub Height: i32,
    pub Format: super::DirectXPixelFormat,
    pub MultisampleDescription: Direct3DMultisampleDescription,
}
impl ::core::marker::Copy for Direct3DSurfaceDescription {}
impl ::core::clone::Clone for Direct3DSurfaceDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Direct3DSurfaceDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DSurfaceDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("MultisampleDescription", &self.MultisampleDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for Direct3DSurfaceDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Direct3DSurfaceDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription;i4;i4;enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4);struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Direct3DSurfaceDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.MultisampleDescription == other.MultisampleDescription
    }
}
impl ::core::cmp::Eq for Direct3DSurfaceDescription {}
impl ::core::default::Default for Direct3DSurfaceDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
